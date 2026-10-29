use crate::auth::UserSession;
use crate::messages::ClientIncomingMessage;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task::AbortHandle;

type WsTx = Arc<Mutex<Option<mpsc::UnboundedSender<ClientIncomingMessage>>>>;
type Handle = Arc<Mutex<Option<AbortHandle>>>;
type Session = Arc<Mutex<Option<UserSession>>>;

/// Thread-safe shared state of the client application
#[derive(Default, Clone)]
pub struct AppState {
    pub session: Session,
    // MPSC channel shared between the producers (UI and Tracking) and the consumer (Task 1 on the Web)
    pub ws_tx: WsTx,
    // Task handles for immediate destruction on shutdown
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

    /// Checks whether the WebSocket connection is currently active
    pub fn is_ws_connected(&self) -> bool {
        self.ws_tx
            .lock()
            .map(|guard| guard.is_some())
            .unwrap_or(false)
    }

    /// Enqueues a message on the shared MPSC channel
    pub fn send_ws_message(&self, msg: ClientIncomingMessage) -> Result<(), String> {
        let guard = self.ws_tx.lock().map_err(|e| e.to_string())?;
        if let Some(tx) = guard.as_ref() {
            tx.send(msg)
                .map_err(|e| format!("Failed to enqueue message on the MPSC channel: {e}"))
        } else {
            Err("WebSocket is not connected to the server".to_string())
        }
    }

    /// Fully destroys every resource on disconnect:
    /// - Aborts the tracking task (Task 3)
    /// - Aborts the transmission task (Task 1)
    /// - Aborts the reception task (Task 2)
    /// - Destroys the MPSC channel by clearing the sender (ws_tx = None)
    /// - Releases the underlying TCP socket (sending FIN to the server)
    pub fn disconnect_ws(&self) {
        // 1. Stop and destroy the tracking task (Task 3)
        if let Ok(mut guard) = self.tracking_abort.lock() {
            if let Some(handle) = guard.take() {
                handle.abort();
            }
        }

        // 2. Stop and destroy the transmission task (Task 1)
        if let Ok(mut guard) = self.send_abort.lock() {
            if let Some(handle) = guard.take() {
                handle.abort();
            }
        }

        // 3. Stop and destroy the reception task (Task 2)
        if let Ok(mut guard) = self.recv_abort.lock() {
            if let Some(handle) = guard.take() {
                handle.abort();
            }
        }

        // 4. Destroy the MPSC channel by dropping the Sender
        if let Ok(mut tx_guard) = self.ws_tx.lock() {
            *tx_guard = None;
        }
    }
}
