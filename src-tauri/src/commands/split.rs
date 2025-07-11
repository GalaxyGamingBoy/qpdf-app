use std::path::PathBuf;

use libqpdf_rs::qpdf::{
    self, error::QPDFInternalErrorCode, read::QPDFReadParams, write::QPDFWriteParams,
};

use crate::{
    commands::details::internal_get_pdf_details,
    utils::{empty_qpdf, read_pdf, write_pdf},
};

pub fn internal_split_pdf_to_pages(pdf: PathBuf, out: PathBuf, format: String) -> bool {
    let details = match internal_get_pdf_details(pdf.clone()) {
        Some(v) => v,
        None => return false,
    };

    let qpdf = match read_pdf(pdf) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let pages = qpdf.len_pages();
    for i in 0..pages {
        let outfile = match empty_qpdf() {
            Ok(v) => v,
            Err(_) => return false,
        };

        let page = qpdf.get_page(i as usize).unwrap();
        if outfile.add_page(page, false) == QPDFInternalErrorCode::Errors {
            return false;
        }

        let dest = format
            .clone()
            .replace("$title", &details.title)
            .replace("$author", &details.author)
            .replace("$i", &i.to_string())
            .replace("$n", &pages.to_string());
        let dest = out.join(dest);

        if write_pdf(outfile, dest).is_err() {
            return false;
        }
    }

    true
}

#[tauri::command]
pub fn split_pdf_to_pages(pdf: String, out: String, format: String) -> bool {
    let pdf = PathBuf::from(pdf);
    let out = PathBuf::from(out);
    internal_split_pdf_to_pages(pdf, out, format)
}
