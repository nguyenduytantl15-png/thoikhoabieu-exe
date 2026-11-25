// Ẩn cửa sổ dòng lệnh khi chạy bản phát hành trên Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_dialog::DialogExt;

/// Hiện hộp "Lưu thành..." của Windows rồi ghi dữ liệu (Excel, .json, .html...) ra file.
/// Trả về true nếu đã lưu, false nếu người dùng bấm Hủy.
#[tauri::command]
async fn save_file(app: tauri::AppHandle, name: String, data: Vec<u8>) -> Result<bool, String> {
    let ext = std::path::Path::new(&name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    let mut dlg = app.dialog().file().set_file_name(&name);
    if !ext.is_empty() {
        dlg = dlg.add_filter(ext.to_uppercase(), &[ext.as_str()]);
    }
    match dlg.blocking_save_file() {
        Some(fp) => {
            let path = fp.into_path().map_err(|e| e.to_string())?;
            std::fs::write(&path, &data).map_err(|e| e.to_string())?;
            Ok(true)
        }
        None => Ok(false),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![save_file])
        .run(tauri::generate_context!())
        .expect("Không khởi động được phần mềm");
}
