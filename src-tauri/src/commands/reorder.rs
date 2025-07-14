use std::path::PathBuf;

use libqpdf_rs::qpdf::error::QPDFInternalErrorCode;

use crate::utils::{empty_qpdf, read_pdf, write_pdf};

pub fn internal_reorder_pdf(file: PathBuf, dest: PathBuf, pages: Vec<i32>) -> bool {
    let pages: Vec<i32> = pages.iter().map(|p| p - 1).collect();

    let pdf = match read_pdf(file) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let out = match empty_qpdf() {
        Ok(v) => v,
        Err(_) => return false,
    };

    for num in pages {
        let page = match pdf.get_page(num as usize) {
            Some(v) => v,
            None => return false,
        };

        if out.add_page(page, false) == QPDFInternalErrorCode::Errors {
            return false;
        }
    }

    write_pdf(out, dest).is_ok()
}

#[tauri::command]
pub fn reorder_pdf(file: String, dest: String, pages: Vec<i32>) -> bool {
    let file = PathBuf::from(file);
    let dest = PathBuf::from(dest);

    internal_reorder_pdf(file, dest, pages)
}
