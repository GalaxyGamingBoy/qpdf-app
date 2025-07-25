#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::merge::merge_all_pdfs,
            commands::details::get_pdf_details,
            commands::split::split_pdf_to_pages,
            commands::read::read_pdf_file,
            commands::reorder::reorder_pdf
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod commands;
pub mod types;
pub mod utils;
