// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Обробляємо Result, який повертає run()
    if let Err(e) = app_lib::run() {
        // Можна додати кращу обробку помилки, наприклад, логування або показ діалогу
        eprintln!("Error running application: {}", e);
        std::process::exit(1);
    }
}
