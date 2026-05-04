use tauri::Manager;

#[tauri::command]
fn get_api_port(port: u16) -> u16 {
    port
}

pub fn run_tauri(api_port: u16) {
    let port = api_port;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            // 将 API 端口传递给前端
            let window = app.get_webview_window("main").unwrap();
            let js = format!(
                "window.__PORTHANNIS_API_PORT__ = {};",
                port
            );
            window.eval(&js).ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_api_port])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
