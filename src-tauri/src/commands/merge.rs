use std::path::PathBuf;

use libqpdf_rs::qpdf::error::QPDFInternalErrorCode;

use crate::utils::{empty_qpdf, read_pdf, write_pdf};

pub fn internal_merge_all_pdfs(pdfs: Vec<PathBuf>, out: PathBuf) -> bool {
    let merged_pdf = match empty_qpdf() {
        Ok(v) => v,
        Err(_) => return false,
    };

    for pdf in pdfs {
        let qpdf = match read_pdf(pdf) {
            Ok(v) => v,
            Err(_) => return false,
        };

        let page_num = qpdf.len_pages();

        for i in 0..page_num {
            let page = qpdf.get_page(i as usize).unwrap();

            let status = merged_pdf.add_page(page, false);
            if status == QPDFInternalErrorCode::Errors {
                return false;
            }
        }
    }

    write_pdf(merged_pdf, out).is_ok()
}

#[tauri::command]
pub fn merge_all_pdfs(pdfs: Vec<String>, out: String) -> bool {
    let pdfs: Vec<PathBuf> = pdfs.iter().map(|p| PathBuf::from(p)).collect();
    let out = PathBuf::from(out);

    internal_merge_all_pdfs(pdfs, out)
}
