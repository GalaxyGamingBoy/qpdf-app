use std::path::PathBuf;

use libqpdf_rs::qpdf::{
    self, error::QPDFInternalErrorCode, read::QPDFReadParams, write::QPDFWriteParams,
};

use crate::commands::details::get_pdf_details;

#[tauri::command]
pub fn split_pdf_to_pages(pdf: String, out: String, format: String) -> bool {
    let details = match get_pdf_details(pdf.clone()) {
        Some(v) => v,
        None => return false,
    };

    let pdf = PathBuf::from(pdf);
    let out = PathBuf::from(out);

    let qpdf = qpdf::QPDF::default();
    let status = match qpdf.process_file(pdf.clone(), QPDFReadParams::default(), None) {
        Ok(e) => e,
        Err(_) => return false,
    };

    if status == QPDFInternalErrorCode::Errors {
        return false;
    }

    let pages = qpdf.len_pages();
    for i in 0..pages {
        let outfile = qpdf::QPDF::default();
        if outfile.empty() == QPDFInternalErrorCode::Errors {
            return false;
        }

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

        let status =
            match outfile.write_init(dest, QPDFWriteParams::default().with_preserve_encryption()) {
                Ok(v) => v,
                Err(_) => return false,
            };

        if status == QPDFInternalErrorCode::Errors {
            return false;
        }

        if outfile.write() == QPDFInternalErrorCode::Errors {
            return false;
        }
    }

    true
}

pub mod details;
pub mod merge;
