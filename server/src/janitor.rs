use crate::sandbox::Sandbox;
use std::{
    borrow::Borrow,
    collections::HashSet,
    hash::{Hash, Hasher},
    sync::{Arc, RwLock},
    time::Instant,
};
use uuid::Uuid;

pub type SessionRef = Arc<Session>;

#[derive(Debug)]
pub struct Session {
    pub id: Uuid,
    pub sandbox: Sandbox,
    pub created_at: Instant,
}
impl Session {
    pub fn get_id_string(&self) -> String {
        self.id.to_simple().to_string()
    }
}

/// Helper type to compare `Session` based on the id.
#[derive(Debug)]
struct SessionById(Arc<Session>);
impl SessionById {
    fn to_session_ref(&self) -> SessionRef {
        Arc::clone(&self.0)
    }
}
impl Borrow<Uuid> for SessionById {
    fn borrow(&self) -> &Uuid {
        &self.0.id
    }
}
impl Eq for SessionById {}
impl PartialEq for SessionById {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}
impl Hash for SessionById {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.id.hash(state)
    }
}

/// Keeps track of active sessions.
#[derive(Debug, Default)]
pub struct Janitor {
    sessions: RwLock<HashSet<SessionById>>,
}
impl Janitor {
    fn new_session_id(sessions: &HashSet<SessionById>) -> Uuid {
        loop {
            let id = Uuid::new_v4();
            if !sessions.contains(&id) {
                return id;
            }
        }
    }

    /// Create a new session for the sandbox.
    pub fn create_session(&self, sandbox: Sandbox) -> SessionRef {
        let mut sessions = self.sessions.write().unwrap();
        let id = Self::new_session_id(&sessions);
        let session = Session {
            id,
            sandbox,
            created_at: Instant::now(),
        };
        log::debug!("created new session {}", session.id);
        sessions
            .get_or_insert(SessionById(Arc::new(session)))
            .to_session_ref()
    }

    /// Get a session by its id.
    pub fn get_session(&self, id: &Uuid) -> Option<SessionRef> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(id).map(SessionById::to_session_ref)
    }
}
