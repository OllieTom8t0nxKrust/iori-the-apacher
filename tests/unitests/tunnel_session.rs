#[cfg(test)]
mod tests {
    use iori_the_apacher::domain::tunnel::TunnelSession;

    #[test]
    fn test_tunnel_session_creation() {
        let subdomain = "test-sub".to_string();
        let port = 8080;
        let protocol = "http".to_string();
        
        let session = TunnelSession::new(subdomain.clone(), port, protocol.clone());
        
        assert_eq!(session.subdomain, subdomain);
        assert_eq!(session.target_port, port);
        assert_eq!(session.protocol, protocol);
        assert!(session.active);
        assert!(!session.id.is_empty());
    }
}
