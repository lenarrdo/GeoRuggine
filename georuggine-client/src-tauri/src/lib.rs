pub mod auth;
pub mod messages;
pub mod state;
pub mod websocket;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::default();

    tauri::Builder::default()
        .manage(app_state)
        // ─────────────────────────────────────────────────────────────
        // MAIN THREAD: when the client window closes, immediately
        // destroys every task, the MPSC channel and the WebSocket.
        // ─────────────────────────────────────────────────────────────
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { .. } | tauri::WindowEvent::Destroyed => {
                    window.state::<AppState>().disconnect_ws();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Authentication (REST via reqwest)
            auth::register,
            auth::login,
            auth::logout,
            auth::get_current_user,
            // Producer 1 (UI): chat message sending
            messages::send_chat_message,
            // Producer 2: coordinate file upload
            messages::upload_coordinates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
