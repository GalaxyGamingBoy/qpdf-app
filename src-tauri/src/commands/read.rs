use std::{fs::File, io::BufReader};

use base64::Engine;

#[tauri::command]
pub fn read_pdf_file(file: String) -> Result<String, ()> {
    let file = match File::open(file) {
        Ok(v) => v,
        Err(_) => return Err(()),
    };
    let file = BufReader::new(file);

    let z = match zstd::stream::encode_all(file, 3) {
        Ok(v) => v,
        Err(_) => return Err(()),
    };
    let b64 = base64::engine::general_purpose::STANDARD;

    Ok(b64.encode(z))
}
