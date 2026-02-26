use simln_lib::ldk::{LdkServer, LdkServerConnection};

#[test]
async fn connect_successfully_to_ldkserver() {
    let conn = LdkServerConnection {
        base_url: "127.0.0.1:3002".to_string(),
        api_key: todo!(),
        server_cert_pem: todo!(),
    };
    let client = LdkServer::new(conn)
        .await
        .expect("Failed to connect to LDK server.");
    todo!()
}
