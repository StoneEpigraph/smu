#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
    Manager,
};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
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
            // commands::sm2_encrypt,  // 暂时禁用，sm2 0.13.3 不支持 PKE
            // commands::sm2_decrypt,  // 暂时禁用，sm2 0.13.3 不支持 PKE
            commands::sm2_sign,
            commands::sm2_verify,
        ])
        .setup(|app| {
            let show_item = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "隐藏", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;
            
            // 显式设置窗口属性，确保配置生效
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_always_on_top(true);
                let _ = window.center();
            }
            
            // 注册全局快捷键 - 使用简单的功能键
            let _app_handle = app.handle().clone();
            let _ = app.global_shortcut().on_shortcut("F1", move |app: &tauri::AppHandle, shortcut: &tauri_plugin_global_shortcut::Shortcut, _event: tauri_plugin_global_shortcut::ShortcutEvent| {
                println!("Global shortcut triggered: {:?}", shortcut);
                println!("Attempting to toggle window visibility");
                if let Some(window) = app.get_webview_window("main") {
                    match window.is_visible() {
                        Ok(is_visible) => {
                            println!("Window visible: {}", is_visible);
                            if is_visible {
                                println!("Hiding window");
                                let _ = window.hide();
                            } else {
                                println!("Showing window");
                                let _ = window.set_always_on_top(true);
                                let _ = window.show();
                                let _ = window.set_always_on_top(true);
                                let _ = window.set_focus();
                            }
                        }
                        Err(e) => {
                            println!("Error checking window visibility: {:?}", e);
                        }
                    }
                } else {
                    println!("Could not get main window");
                }
            });
            
            // 尝试简单的功能键和组合
            let shortcuts = vec!["F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"];
            for shortcut in shortcuts {
                println!("Trying to register shortcut: {}", shortcut);
                let _ = app.global_shortcut().unregister(shortcut);
                match app.global_shortcut().register(shortcut) {
                    Ok(_) => println!("Global shortcut {} registered successfully", shortcut),
                    Err(e) => println!("Failed to register shortcut {}: {:?}", shortcut, e),
                }
            }
            
            // 处理窗口关闭事件，隐藏窗口而不是退出应用
            let main_window = app.get_webview_window("main").unwrap();
            let window_clone = main_window.clone();
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window_clone.hide();
                }
            });
            
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("SMU 工具箱")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
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
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.set_always_on_top(true);
                            let _ = window.show();
                            let _ = window.set_always_on_top(true);
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            
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