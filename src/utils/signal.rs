pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to hook Ctrl + C signal handler");
    println!("Received Ctrl + C Signal");
}
