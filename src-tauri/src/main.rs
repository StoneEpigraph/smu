#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
    Manager,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::encode_md5,
            commands::decode_md5,
            commands::encode_sha1,
            commands::encode_sha256,
            commands::encode_sha512,
            commands::encode_base64,
            commands::decode_base64,
            commands::encode_url,
            commands::decode_url,
            commands::encode_hex,
            commands::decode_hex,
            commands::exit_app,
            commands::add_note,
            commands::get_notes,
            commands::delete_note,
            commands::increment_use_count,
            commands::get_use_counts,
            commands::format_json,
            commands::minify_json,
        ])
        .setup(|app| {
            let show_item = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "隐藏", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;
            
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("SMU 工具箱")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "hide" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.hide();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            
            // 确保主窗口显示
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}