use std::{fs, path::PathBuf};

use base64::Engine;

#[tauri::command]
pub fn read_pdf_file(file: String) -> Result<String, ()> {
    let path = PathBuf::from(file.clone());

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false);
    if !ext {
        return Err(());
    }

    let file = match fs::read(file) {
        Ok(v) => v,
        Err(_) => return Err(()),
    };

    let b64 = base64::engine::general_purpose::STANDARD;
    Ok(b64.encode(&file))
}
