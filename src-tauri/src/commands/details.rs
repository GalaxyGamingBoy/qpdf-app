use std::path::PathBuf;

use crate::{types::PDFDetails, utils::read_pdf};

pub fn internal_get_pdf_details(pdf: PathBuf) -> Option<PDFDetails> {
    let qpdf = match read_pdf(pdf.clone()) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let title = qpdf.pdf_get_info_key("/Title".to_string());
    let title = match title {
        Ok(v) => v,
        Err(_) => {
            let mut pdf = pdf.clone();
            pdf.set_extension("");
            pdf.file_name().unwrap().to_str().unwrap().to_string()
        }
    };

    let author = qpdf.pdf_get_info_key("/Author".to_string());
    let author = match author {
        Ok(v) => v,
        Err(_) => "John Doe".to_string(),
    };

    Some(PDFDetails { title, author })
}

#[tauri::command]
pub fn get_pdf_details(pdf: String) -> Option<PDFDetails> {
    let pdf = PathBuf::from(pdf);
    internal_get_pdf_details(pdf)
}
