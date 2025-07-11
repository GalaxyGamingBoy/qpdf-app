use std::path::PathBuf;

use libqpdf_rs::qpdf::{
    self, error::QPDFInternalErrorCode, read::QPDFReadParams, write::QPDFWriteParams,
};

pub fn empty_qpdf() -> Result<qpdf::QPDF, ()> {
    let pdf = qpdf::QPDF::default();
    if pdf.empty() == QPDFInternalErrorCode::Errors {
        return Err(());
    }

    Ok(pdf)
}

pub fn read_pdf(pdf: PathBuf) -> Result<qpdf::QPDF, ()> {
    let qpdf = qpdf::QPDF::default();

    let status = qpdf
        .process_file(pdf, QPDFReadParams::default(), None)
        .unwrap();

    if status == QPDFInternalErrorCode::Errors {
        return Err(());
    }

    Ok(qpdf)
}

pub fn write_pdf(qpdf: qpdf::QPDF, file: PathBuf) -> Result<qpdf::QPDF, ()> {
    let status = qpdf
        .write_init(file, QPDFWriteParams::default().with_preserve_encryption())
        .unwrap();
    if status == QPDFInternalErrorCode::Errors {
        return Err(());
    }

    let status = qpdf.write();
    if status == QPDFInternalErrorCode::Errors {
        return Err(());
    }

    Ok(qpdf)
}
