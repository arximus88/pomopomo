use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager, AppHandle, RunEvent, Result, Wry,
};
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};
use log::{info, error};

// Функція для створення меню трея
fn create_tray_menu(app: &AppHandle<Wry>) -> Result<Menu<Wry>> {
    let quit = MenuItem::with_id(app, "quit", "Quit PomoPomo", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    
    // Передаємо посилання безпосередньо
    Menu::with_items(app, &[&show, &hide, &separator, &quit])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    // Використовуємо Target::new(TargetKind::...)
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                    Target::new(TargetKind::Folder { 
                        path: std::path::PathBuf::from("."), // Можливо, краще вказати директорію логів
                        file_name: Some("pomopomo".into()) // Додаємо ім'я файлу
                    })
                ])
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| { // Створюємо трей всередині .setup()
            info!("Setting up tray...");
            let menu = create_tray_menu(app.handle())?;
            
            // Створюємо TrayIconBuilder один раз
            let mut tray_builder = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("PomoPomo Timer");

            // Спробуємо встановити іконку
            if let Some(icon) = app.default_window_icon().cloned() {
                tray_builder = tray_builder.icon(icon);
            } else {
                error!("Could not get default window icon for tray");
                // Можна встановити резервну іконку тут, якщо потрібно
            }

            let _tray = tray_builder
                .on_menu_event(|app, event| {
                    info!("Tray menu item clicked: {:?}", event.id);
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "hide" | "show" => { // Об'єднуємо логіку для hide/show
                            if let Some(window) = app.get_webview_window("main") {
                                if event.id.as_ref() == "hide" {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            } else {
                                error!("Could not get main window for hide/show");
                            }
                        }
                        // TODO: Обробка Play/Pause, Reset
                        _ => { info!("Unhandled menu item: {:?}", event.id); }
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    info!("Tray icon event: {:?}", event);
                    match event {
                        TrayIconEvent::Click {
                            button: tauri::tray::MouseButton::Left,
                            button_state: tauri::tray::MouseButtonState::Up,
                            ..
                        } => {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                info!("Tray left click: Toggling window visibility");
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            } else {
                                error!("Could not get main window for left click toggle");
                            }
                        }
                        _ => {}
                    }
                })
                .build(app.handle())?;
            info!("Tray setup complete.");
            Ok(())
        })
        .build(tauri::generate_context!())?
        .run(|_app_handle, event| match event { // Використовуємо .build() та .run() з обробником подій
            RunEvent::WindowEvent { event: tauri::WindowEvent::CloseRequested { api: _, .. }, .. } => {
                // Перехоплюємо закриття вікна тут, якщо setup не спрацював
                // _app_handle.get_webview_window("main").unwrap().hide().unwrap();
                // api.prevent_close();
                // Краще залишити цей обробник у .setup(), якщо можливо
                // Або використовувати плагін window-state
            }
            RunEvent::ExitRequested { api, .. } => {
                 // Дозволяємо додатку закритися, якщо це ініційовано з меню трея або іншим чином
                // api.prevent_exit(); // Прибираємо блокування виходу
            }
            _ => {}
        });
    
    Ok(())
}
