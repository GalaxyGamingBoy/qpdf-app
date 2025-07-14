use std::fs;

use base64::Engine;

#[tauri::command]
pub fn read_pdf_file(file: String) -> Result<String, ()> {
    let file = match fs::read(file) {
        Ok(v) => v,
        Err(_) => return Err(()),
    };

    let b64 = base64::engine::general_purpose::STANDARD;
    Ok(b64.encode(&file))
}
