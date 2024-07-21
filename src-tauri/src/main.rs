// This hides the console for Windows release builds
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            // Load the page with the JavaScript that checks the cookie and redirects
            window.eval(
                r#"
                (function() {
                    function checkCookie(cookieName) {
                        const value = `; ${document.cookie}`;
                        const parts = value.split(`; ${cookieName}=`);
                        if (parts.length === 2) return parts.pop().split(';').shift();
                    }

                    const yandexLoginCookie = checkCookie('yandex_login');

                    if (yandexLoginCookie) {
                        window.location.href = 'https://mail.yandex.ru';
                    } else {
                        window.location.href = 'https://passport.yandex.ru/auth?retpath=https%3A%2F%2Fmail.yandex.ru';
                    }
                })();
                "#
            ).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
