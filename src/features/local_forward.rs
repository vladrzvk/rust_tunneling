use tokio::net::{TcpListener, TcpStream};
use tokio::io;
use log::{info, error};

/// Lance un port forwarding LOCAL.
/// Exemple: écoute sur `listen_addr` et forward vers `dest_addr`.
pub async fn local_forward(listen_addr: &str, dest_addr: &str) -> io::Result<()> {
    info!("[local_forward] Listening on {} -> forward to {}", listen_addr, dest_addr);

    let listener = TcpListener::bind(listen_addr).await?;
    loop {
        let (inbound, addr) = listener.accept().await?;
        info!("New local connection from {:?}", addr);

        let dest_clone = dest_addr.to_string();
        tokio::spawn(async move {
            match TcpStream::connect(dest_clone).await {
                Ok(outbound) => {
                    if let Err(e) = forward_connection(inbound, outbound).await {
                        error!("Forward error: {}", e);
                    }
                }
                Err(e) => error!("Failed to connect: {}", e),
            }
        });
    }
}

/// Transfère les données inbound <-> outbound
async fn forward_connection(mut inbound: TcpStream, mut outbound: TcpStream) -> io::Result<()> {
    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = io::copy(&mut ri, &mut wo);
    let server_to_client = io::copy(&mut ro, &mut wi);

    tokio::select! {
        r1 = client_to_server => { r1?; }
        r2 = server_to_client => { r2?; }
    }

    Ok(())
}
