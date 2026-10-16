pub mod auth;
pub mod messages;
pub mod state;
pub mod tracking;
pub mod websocket;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::default();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        // ─────────────────────────────────────────────────────────────
        // MAIN THREAD: All'atto della chiusura della finestra del client
        // distrugge immediatamente tutti i task, il canale MPSC e il WebSocket.
        // ─────────────────────────────────────────────────────────────
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { .. } | tauri::WindowEvent::Destroyed => {
                    println!("[Main Thread] Chiusura finestra rilevata: distruzione immediata canali, socket e task...");
                    window.state::<AppState>().disconnect_ws();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Autenticazione (REST con reqwest)
            auth::register,
            auth::login,
            auth::logout,
            auth::get_current_user,
            // Produttore 1 (UI): invio messaggi di chat
            messages::send_chat_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
