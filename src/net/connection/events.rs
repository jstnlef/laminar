use std::net::SocketAddr;

use crate::net::NetworkQuality;

/// Events that are generated in response to a change in state of the connected client
pub enum ClientStateChange {
    /// A new client connects. Clients are uniquely identified by the ip:port combination at this layer.
    Connected(SocketAddr),
    /// A client disconnects. This can be generated from the server-side intentionally disconnecting a client,
    /// or it could be from the client disconnecting.
    Disconnected(SocketAddr),
    /// This is generated if the server has not seen traffic from a client for a configurable amount of time.
    TimedOut(SocketAddr),
    /// This is generated when there is a change in the connection quality of a client.
    QualityChange {
        /// Connection whose quality changed
        conn: SocketAddr,
        /// Previous quality
        from: NetworkQuality,
        /// New quality
        to: NetworkQuality,
    },
}

#[cfg(test)]
mod test {
    use super::ClientStateChange;
    use std::net::ToSocketAddrs;

    static TEST_HOST_IP: &'static str = "127.0.0.1";
    static TEST_PORT: &'static str = "20000";

    #[test]
    fn test_create_event() {
        let addr = format!("{}:{}", TEST_HOST_IP, TEST_PORT).to_socket_addrs();
        let mut addr = addr.unwrap();
        let _ = ClientStateChange::Connected(addr.next().unwrap());
    }
}
