use clap::{Parser, Subcommand};
use log::info;
use std::error::Error;
use env_logger;

mod features {
    pub mod local_forward;
    pub mod remote_forward;
    pub mod local_download;
    pub mod remote_download;
}

use features::{
    local_forward::local_forward,
    remote_forward::remote_forward,
    local_download::local_download,
    remote_download::{remote_download_server, remote_download_client},
};

#[derive(Parser)]
#[command(author, version, about="Tunneling demo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lancement d’un port forwarding local
    LocalForward {
        #[arg(long, default_value = "127.0.0.1:9000")]
        listen: String,

        #[arg(long, default_value = "127.0.0.1:8080")]
        dest: String,
    },

    /// Lancement d’un port forwarding “remote” (même code, d’autres IP)
    RemoteForward {
        #[arg(long, default_value = "0.0.0.0:9000")]
        listen: String,

        #[arg(long, default_value = "127.0.0.1:8080")]
        dest: String,
    },

    /// Copie en local d’un fichier
    LocalDownload {
        #[arg(long)]
        src: String,

        #[arg(long)]
        dst: String,
    },

    /// Démarre un server qui envoie un fichier au client
    RemoteDownloadServer {
        #[arg(long, default_value = "0.0.0.0:9001")]
        listen: String,

        #[arg(long)]
        file: String,
    },

    /// Client qui se connecte au remote download server et récupère le fichier
    RemoteDownloadClient {
        #[arg(long)]
        server: String,

        #[arg(long)]
        out: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::LocalForward { listen, dest } => {
            info!("Launching local forward ...");
            local_forward(&listen, &dest).await?;
        }
        Commands::RemoteForward { listen, dest } => {
            info!("Launching remote forward ...");
            remote_forward(&listen, &dest).await?;
        }
        Commands::LocalDownload { src, dst } => {
            info!("Local download (copy)...");
            local_download(&src, &dst)?;
        }
        Commands::RemoteDownloadServer { listen, file } => {
            info!("RemoteDownloadServer ...");
            remote_download_server(&listen, &file).await?;
        }
        Commands::RemoteDownloadClient { server, out } => {
            info!("RemoteDownloadClient ...");
            remote_download_client(&server, &out).await?;
        }
    }

    Ok(())
}
