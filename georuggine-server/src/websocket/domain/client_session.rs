use std::net::SocketAddr;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ClientSession {
    pub addr: SocketAddr,
    pub user_id: Option<i64>,
    pub user_agent: String,
}
