use std::error::Error;
use std::fs;
use log::{info, error};

/// Copie un fichier en local (ex: `C:\some\file.txt` -> `C:\some\backup\file.txt`).
/// Pour la démo, on fait un "simple" fs::copy.
pub fn local_download(src_path: &str, dst_path: &str) -> Result<(), Box<dyn Error>> {
    info!("[local_download] Copy from {} to {}", src_path, dst_path);

    // fs::copy renvoie le nombre d'octets copiés, ou une erreur
    match fs::copy(src_path, dst_path) {
        Ok(bytes) => {
            info!("Successfully copied {} bytes", bytes);
            Ok(())
        },
        Err(e) => {
            error!("Error copying file: {}", e);
            Err(Box::new(e))
        }
    }
}
