use std::path::Path;
use futures_util::StreamExt;
use tauri::AppHandle;
use tokio::select;
use std::sync::{Arc, Mutex};
use tokio::sync::watch;
use tauri::Emitter;

#[derive(Clone)]
struct AppState {
    cancel_tx: Arc<Mutex<Option<watch::Sender<bool>>>>,
}

#[tauri::command]
fn path_check(path: &str) -> bool {
    return Path::new(&path).exists();
}

#[tauri::command]
fn cancel_download(state: tauri::State<AppState>) -> Result<(), String> {
    let guard = state.cancel_tx.lock().unwrap();

    if let Some(tx) = &*guard {
        let _ = tx.send(true);
        Ok(())
    } else {
        Err("当前没有下载任务".into())
    }
}

#[tauri::command]
async fn download_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    url: &str,
    save_path: &str
) -> Result<(), String> {
    let (tx, mut rx) = watch::channel(false);
    {
        let mut guard = state.cancel_tx.lock().unwrap();
        *guard = Some(tx);
    }

    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let total_size = response.content_length().unwrap_or(0);

    let mut file = std::fs::File::create(&save_path)
        .map_err(|e| e.to_string())?;

    let mut stream = response.bytes_stream();
    let mut downloaded = 0;

    while let Some(item) = stream.next().await {
        select! {
            _ = rx.changed() => {
                if *rx.borrow() {
                    println!("下载被取消");

                    // 可选：删除未完成文件
                    let _ = std::fs::remove_file(&save_path);

                    return Err("已取消".into());
                }
            }

            chunk = async { item } => {
                let chunk = chunk.map_err(|e| e.to_string())?;

                use std::io::Write;
                file.write_all(&chunk).map_err(|e| e.to_string())?;

                downloaded += chunk.len() as u64;

                let progress = if total_size > 0 {
                    downloaded as f64 / total_size as f64
                } else {
                    0.0
                };

                app.emit("download-progress", progress).ok();
            }
        }
    }

    // 清空状态
    {
        let mut guard = state.cancel_tx.lock().unwrap();
        *guard = None;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .setup(|_app| {
            #[cfg(target_os = "macos")]
            {
                use tauri::menu::{Menu, PredefinedMenuItem, Submenu};

                let handle = _app.handle();
                let locale = "zh";
                let (app_menu_name, quit_label, edit_label, copy_label, paste_label, select_all_label, undo_label, redo_label, window_label, minimize_label, fullscreen_label, about_label, hide_label) = 
                if locale == "zh" {
                    ("应用", "退出 toWebp", "编辑", "复制", "粘贴", "全选", "撤销", "重做", "窗口", "最小化", "进入全屏幕", "关于 toWebp", "隐藏 toWebp")
                } else {
                    ("App", "Quit toWebp", "Edit", "Copy", "Paste", "Select All", "Undo", "Redo", "Window", "Minimize", "Fullscreen", "About toWebp", "Hide toWebp")
                };
                
                let app_menu = Submenu::with_items(
                    handle, 
                    app_menu_name, 
                    true, 
                    &[
                        &PredefinedMenuItem::about(handle, Some(about_label), None)?,
                        &PredefinedMenuItem::separator(handle)?,
                        &PredefinedMenuItem::hide(handle, Some(hide_label))?,
                        &PredefinedMenuItem::quit(handle, Some(quit_label))?
                    ]
                )?;

                let edit_menu = Submenu::with_items(
                    handle,
                    edit_label,
                    true,
                    &[
                        &PredefinedMenuItem::undo(handle, Some(undo_label))?,
                        &PredefinedMenuItem::redo(handle, Some(redo_label))?,
                        &PredefinedMenuItem::separator(handle)?,
                        &PredefinedMenuItem::copy(handle, Some(copy_label))?,
                        &PredefinedMenuItem::paste(handle, Some(paste_label))?,
                        &PredefinedMenuItem::select_all(handle, Some(select_all_label))?,
                    ],
                )?;

                let window_menu = Submenu::with_items(
                    handle,
                    window_label,
                    true,
                    &[
                        &PredefinedMenuItem::minimize(handle, Some(minimize_label))?,
                        &PredefinedMenuItem::fullscreen(handle, Some(fullscreen_label))?,
                    ],
                )?;

                let menu = Menu::with_items(handle, &[&app_menu, &edit_menu, &window_menu])?;
                _app.set_menu(menu)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            cancel_tx: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![path_check, download_file, cancel_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
