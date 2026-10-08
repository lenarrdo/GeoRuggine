use crate::auth::domain::session::UserSession;
use crate::state::AppState;

pub struct SessionRepository;

impl SessionRepository {
    pub fn save(state: &AppState, session: UserSession) -> Result<(), String> {
        let mut guard = state.session.lock().map_err(|e| e.to_string())?;
        *guard = Some(session);
        Ok(())
    }

    pub fn clear(state: &AppState) -> Result<(), String> {
        let mut guard = state.session.lock().map_err(|e| e.to_string())?;
        *guard = None;
        Ok(())
    }

    pub fn get(state: &AppState) -> Result<Option<UserSession>, String> {
        let guard = state.session.lock().map_err(|e| e.to_string())?;
        Ok(guard.clone())
    }
}
