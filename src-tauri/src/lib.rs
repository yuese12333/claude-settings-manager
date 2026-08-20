mod commands;
mod models;
mod utils;

use commands::{
    detect_settings_path, load_profiles, load_settings, load_settings_raw, pick_settings_file,
    save_profiles, save_settings, save_settings_raw, sibling_settings_path, validate_settings_json,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init());

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let builder = builder.plugin(tauri_plugin_updater::Builder::new().build());

    builder
        .invoke_handler(tauri::generate_handler![
            detect_settings_path,
            load_settings,
            load_settings_raw,
            save_settings,
            save_settings_raw,
            validate_settings_json,
            pick_settings_file,
            sibling_settings_path,
            load_profiles,
            save_profiles,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
