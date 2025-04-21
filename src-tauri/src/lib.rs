use log::{error, info};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Result, RunEvent, Wry,
};
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};

// Функція для створення меню трея
fn create_tray_menu(app: &AppHandle<Wry>) -> Result<Menu<Wry>> {
    let quit = MenuItem::with_id(app, "quit", "Quit PomoPomo", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let play_pause = MenuItem::with_id(app, "play_pause", "Play/Pause", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;

    // Передаємо посилання безпосередньо
    Menu::with_items(
        app,
        &[&show, &hide, &settings, &play_pause, &separator, &quit],
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
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
                // RotationStrategy може мати інші варіанти в Tauri 2.x
                // закоментуємо це, поки не перевіримо документацію
                //.rotation_strategy(RotationStrategy::Daily)
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

            // Створюємо tray та зберігаємо посилання на нього в стані додатку
            let tray = tray_builder
                .on_menu_event(|app, event| {
                    info!("Tray menu item clicked: {:?}", event.id);
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "hide" | "show" => { // Об'єднуємо логіку для hide/show
                            if let Some(window) = app.get_webview_window("main") {
                                info!("Handling {}", event.id.as_ref());
                                // Використовуємо нативні методи замість JavaScript
                                if event.id.as_ref() == "hide" {
                                    if let Err(e) = window.hide() {
                                        error!("Failed to hide window: {}", e);
                                    }
                                } else {
                                    if let Err(e) = window.show() {
                                        error!("Failed to show window: {}", e);
                                    }
                                    if let Err(e) = window.set_focus() {
                                        error!("Failed to focus window: {}", e);
                                    }
                                }
                            } else {
                                error!("Could not get main window for hide/show");
                            }
                        }
                        "settings" => {
                            // Викликаємо функцію відкриття налаштувань
                            if let Some(window) = app.get_webview_window("main") {
                                info!("Opening settings from tray menu");
                                // Показуємо вікно, якщо воно приховане
                                if let Err(e) = window.show() {
                                    error!("Failed to show window: {}", e);
                                }
                                if let Err(e) = window.set_focus() {
                                    error!("Failed to focus window: {}", e);
                                }    
                                // Відправляємо подію через JavaScript
                                if let Err(e) = window.eval("window.dispatchEvent(new CustomEvent('pomopomo://open-settings'))") {
                                    error!("Failed to execute settings script: {}", e);
                                }
                            } else {
                                error!("Could not get main window for settings");
                            }
                        }
                        "play_pause" => {
                            // Викликаємо функцію паузи/продовження
                            if let Some(window) = app.get_webview_window("main") {
                                info!("Toggling play/pause from tray menu");
                                // Відправляємо подію через JavaScript
                                if let Err(e) = window.eval("window.dispatchEvent(new CustomEvent('pomopomo://toggle-play-pause'))") {
                                    error!("Failed to execute play/pause script: {}", e);
                                }
                            } else {
                                error!("Could not get main window for play/pause");
                            }
                        }
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
                                    if let Err(e) = window.hide() {
                                        error!("Failed to hide window: {}", e);
                                    }
                                } else {
                                    if let Err(e) = window.show() {
                                        error!("Failed to show window: {}", e);
                                    }
                                    if let Err(e) = window.set_focus() {
                                        error!("Failed to focus window: {}", e);
                                    }
                                }
                            } else {
                                error!("Could not get main window for left click toggle");
                            }
                        }
                        _ => {}
                    }
                })
                .build(app.handle())?;
                
            // Зберігаємо посилання на tray icon у стані додатку
            app.manage(tray);
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
            _ => {}
        });

    Ok(())
}
