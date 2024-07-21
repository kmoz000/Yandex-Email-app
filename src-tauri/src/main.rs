// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        /* .menu(|handle| {
            tauri::menu::Menu::with_items(
                handle,
                &[&tauri::menu::Submenu::with_items(
                    handle,
                    "Tools",
                    true,
                    &[
                        &tauri::menu::MenuItem::new(handle, "Send", true, None::<&str>)?,
                    ],
                )?],
            )
        }) */
       .on_page_load(|app, view|{
        let lang = tauri_plugin_os::locale().unwrap();
        let allowed = ["fr-FR", "fr-ma", "en-US", "en-UK", "ru-RU", "ar-AR"].iter().map(|s|s.to_string()).collect::<Vec<String>>();
        let choosed = match allowed.contains(&lang) {
            true => &lang.split("-").next().unwrap(),
            false =>"fr"
        };

        println!("language:{:?}", choosed);
/*         app.eval(
             r#"
         (function() {
                const yatranslate = {
                    lang: 'ru',
                    langFirstVisit: 'fr'
                };

                document.addEventListener('DOMContentLoaded', function () {
                    yaTranslateInit();
                });

                function yaTranslateInit() {
                    if (yatranslate.langFirstVisit && !localStorage.getItem('yt-widget')) {
                        yaTranslateSetLang(yatranslate.langFirstVisit);
                    }

                    let script = document.createElement('script');
                    script.src = `https://translate.yandex.net/website-widget/v1/widget.js?widgetId=ytWidget&pageLang=${yatranslate.lang}&widgetTheme=light&autoMode=false`;
                    script.onload = () => {
                        const widget = new window.YandexTranslateWidget({
                            widgetId: 'ytWidget',
                            pageLang: yatranslate.lang,
                            widgetTheme: 'light',
                            autoMode: false
                        });

                        widget.setLang(yatranslate.langFirstVisit);
                    };
                    document.head.appendChild(script);
                }

                function yaTranslateSetLang(lang) {
                    localStorage.setItem('yt-widget', JSON.stringify({
                        "lang": lang,
                        "active": true
                    }));
                }
            })();
             "#
        ).unwrap(); */
       })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
