use tauri::Manager;

pub fn run_tauri(api_port: u16) {
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();
            let url = format!("http://127.0.0.1:{}", api_port);
            window
                .eval(&format!("window.location.href = '{}';", url))
                .ok();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
