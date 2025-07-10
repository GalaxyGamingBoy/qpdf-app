use std::path::PathBuf;

use libqpdf_rs::qpdf::{
    self, error::QPDFInternalErrorCode, read::QPDFReadParams, write::QPDFWriteParams,
};

#[tauri::command]
pub fn merge_all_pdfs(pdfs: Vec<String>, out: String) -> bool {
    let merged_pdf = qpdf::QPDF::default();
    merged_pdf.empty();

    for file in pdfs {
        let file = PathBuf::from(file);

        let qpdf = qpdf::QPDF::default();
        let status = qpdf
            .process_file(file, QPDFReadParams::default(), None)
            .unwrap();

        if status == QPDFInternalErrorCode::Errors {
            return false;
        }

        let page_num = qpdf.len_pages();

        for i in 0..page_num {
            let page = qpdf.get_page(i as usize).unwrap();
            merged_pdf.add_page(page, false);
        }
    }

    let write_params = QPDFWriteParams::default().with_preserve_encryption();
    let out = PathBuf::from(out);

    let status = merged_pdf.write_init(out, write_params).unwrap();

    if status == QPDFInternalErrorCode::Errors {
        return false;
    }

    merged_pdf.write();

    true
}
