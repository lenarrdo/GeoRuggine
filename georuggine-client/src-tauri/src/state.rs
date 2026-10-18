use crate::auth::UserSession;
use crate::messages::ClientIncomingMessage;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

type WsTx = Arc<Mutex<Option<mpsc::UnboundedSender<ClientIncomingMessage>>>>;
type Handle = Arc<Mutex<Option<AbortHandle>>>;
type Session = Arc<Mutex<Option<UserSession>>>;

/// Stato condiviso thread-safe dell'applicazione client
#[derive(Default, Clone)]
pub struct AppState {
    pub session: Session,
    // Canale MPSC condiviso tra i produttori (UI e Tracking) e il consumatore (Task 1 sul Web)
    pub ws_tx: WsTx,
    // Handle dei task per distruzione immediata alla chiusura
    pub tracking_abort: Handle,
    pub send_abort: Handle,
    pub recv_abort: Handle,
}

impl AppState {
    pub fn save_session(&self, session: UserSession) -> Result<(), String> {
        let mut guard = self.session.lock().map_err(|e| e.to_string())?;
        *guard = Some(session);
        Ok(())
    }

    pub fn clear_session(&self) -> Result<(), String> {
        let mut guard = self.session.lock().map_err(|e| e.to_string())?;
        *guard = None;
        Ok(())
    }

    pub fn get_session(&self) -> Result<Option<UserSession>, String> {
        let guard = self.session.lock().map_err(|e| e.to_string())?;
        Ok(guard.clone())
    }

    /// Inserisce un messaggio nel canale MPSC condiviso
    pub fn send_ws_message(&self, msg: ClientIncomingMessage) -> Result<(), String> {
        let guard = self.ws_tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = guard.as_ref() {
            tx.send(msg)
                .map_err(|e| format!("Errore accodamento messaggio sul canale MPSC: {e}"))
        } else {
            Err("WebSocket non connessa al server".to_string())
        }
    }

    /// Distrugge completamente tutte le risorse alla disconnessione:
    /// - Abortisce il task di tracking (Task 3)
    /// - Abortisce il task di trasmissione (Task 1)
    /// - Abortisce il task di ricezione (Task 2)
    /// - Distrugge il canale MPSC azzerando il sender (ws_tx = None)
    /// - Rilascia il socket TCP sottostante (inviando FIN al server)
    pub fn disconnect_ws(&self) {
        // 1. Ferma e distrugge il task di tracking (Task 3)
        if let Ok(mut guard) = self.tracking_abort.lock() {
            if let Some(handle) = guard.take() {
                handle.abort();
                println!("[Tracking] Task tracking abortito e distrutto");
            }
        }

        // 2. Ferma e distrugge il task di trasmissione (Task 1)
        if let Ok(mut guard) = self.send_abort.lock() {
            if let Some(handle) = guard.take() {
                handle.abort();
                println!("[WS Client] Task 1 (Trasmissione) abortito e distrutto");
            }
        }

        // 3. Ferma e distrugge il task di ricezione (Task 2)
        if let Ok(mut guard) = self.recv_abort.lock() {
            if let Some(handle) = guard.take() {
                handle.abort();
                println!("[WS Client] Task 2 (Ricezione) abortito e distrutto");
            }
        }

        // 4. Distrugge il canale MPSC rilasciando il Sender
        if let Ok(mut tx_guard) = self.ws_tx.lock() {
            *tx_guard = None;
            println!("[WS Client] Canale MPSC azzerato e deallocato");
        }
    }
}
