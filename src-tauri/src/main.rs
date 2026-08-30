#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod error;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            println!("Received single instance callback");
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![
            commands::encode_md5,
            commands::decode_md5,
            commands::encode_sha1,
            commands::encode_sha256,
            commands::encode_sha512,
            commands::encode_sm3,
            commands::encode_sm3_hash,
            commands::encode_base64,
            commands::decode_base64,
            commands::encode_url,
            commands::decode_url,
            commands::encode_hex,
            commands::decode_hex,
            commands::exit_app,
            commands::get_app_version,
            commands::save_settings,
            commands::load_settings,
            commands::add_note,
            commands::get_notes,
            commands::delete_note,
            commands::increment_use_count,
            commands::get_use_counts,
            commands::format_json,
            commands::minify_json,
            commands::generate_sm2_keypair,
            commands::sm2_encrypt,
            commands::sm2_decrypt,
            commands::sm2_sign,
            commands::sm2_verify,
            commands::sm2_encrypt_base64,
            commands::sm2_decrypt_base64,
            commands::add_calendar_todo,
            commands::get_calendar_todos,
            commands::update_calendar_todo,
            commands::delete_calendar_todo,
            commands::check_due_reminders,
        ])
        .setup(|app| {
            let db = db::init(app.handle())?;
            app.manage(db);

            let show_item = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "隐藏", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;

            // 显式设置窗口属性，确保配置生效；处理窗口关闭事件，隐藏窗口而不是退出应用
            if let Some(main_window) = app.get_webview_window("main") {
                let _ = main_window.set_always_on_top(true);
                let _ = main_window.center();

                let window_clone = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = window_clone.hide();
                    }
                });
            }

            let mut tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("SMU 工具箱")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.set_always_on_top(true);
                            let _ = window.show();
                            let _ = window.set_always_on_top(true);
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
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.set_always_on_top(true);
                            let _ = window.show();
                            let _ = window.set_always_on_top(true);
                            let _ = window.set_focus();
                        }
                    }
                });
            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }
            let _tray = tray_builder.build(app)?;

            // 确保主窗口显示并始终在最前面
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_always_on_top(true);
                let _ = window.show();
                let _ = window.set_focus();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
