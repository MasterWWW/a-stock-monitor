mod quote;

use quote::{fetch_quotes, StockQuote};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};

const WATCHLIST_KEY: &str = "watchlist";
const POLL_INTERVAL_KEY: &str = "poll_interval_ms";

/// 应用内共享状态
pub struct AppState {
    watchlist: Mutex<Vec<String>>,
    poll_interval_ms: Mutex<u64>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            watchlist: Mutex::new(vec!["600519".to_string(), "000001".to_string()]),
            poll_interval_ms: Mutex::new(3000),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistConfig {
    pub codes: Vec<String>,
    pub poll_interval_ms: u64,
}

/// 拉取自选股实时行情
#[tauri::command]
async fn get_quotes(codes: Vec<String>) -> Result<Vec<StockQuote>, String> {
    fetch_quotes(&codes).await
}

/// 读取自选股配置（内部 helper）
fn read_watchlist_config(state: &AppState) -> WatchlistConfig {
    WatchlistConfig {
        codes: state.watchlist.lock().unwrap().clone(),
        poll_interval_ms: *state.poll_interval_ms.lock().unwrap(),
    }
}

/// 读取自选股配置
#[tauri::command]
fn get_watchlist(state: State<'_, AppState>) -> WatchlistConfig {
    read_watchlist_config(&state)
}

/// 保存自选股并广播变更
#[tauri::command]
fn set_watchlist(
    app: AppHandle,
    state: State<'_, AppState>,
    codes: Vec<String>,
    poll_interval_ms: Option<u64>,
) -> Result<(), String> {
    let normalized: Vec<String> = codes
        .into_iter()
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty() && c.len() == 6 && c.chars().all(|ch| ch.is_ascii_digit()))
        .collect();

    {
        let mut list = state.watchlist.lock().map_err(|e| e.to_string())?;
        *list = normalized;
    }

    if let Some(ms) = poll_interval_ms {
        let mut interval = state.poll_interval_ms.lock().map_err(|e| e.to_string())?;
        *interval = ms.clamp(2000, 60000);
    }

    let config = read_watchlist_config(&state);
    let _ = app.emit("watchlist-changed", &config);
    let _ = persist_config(&app, &config);
    Ok(())
}

/// 打开或聚焦大窗
#[tauri::command]
async fn show_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    } else {
        tauri::WebviewWindowBuilder::new(&app, "main", tauri::WebviewUrl::App("index.html".into()))
            .title("A股盯盘助手")
            .inner_size(1140.0, 740.0)
            .center()
            .build()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 从磁盘恢复持久化配置
fn load_persisted_config(app: &AppHandle, state: &AppState) {
    use tauri_plugin_store::StoreExt;

    if let Ok(store) = app.store("config.json") {
        if let Some(codes) = store.get(WATCHLIST_KEY).and_then(|v| v.as_array().cloned()) {
            let parsed: Vec<String> = codes
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            if !parsed.is_empty() {
                *state.watchlist.lock().unwrap() = parsed;
            }
        }
        if let Some(ms) = store.get(POLL_INTERVAL_KEY).and_then(|v| v.as_u64()) {
            *state.poll_interval_ms.lock().unwrap() = ms.clamp(2000, 60000);
        }
    }
}

/// 持久化自选股配置
fn persist_config(app: &AppHandle, config: &WatchlistConfig) -> Result<(), String> {
    use tauri_plugin_store::StoreExt;

    let store = app.store("config.json").map_err(|e| e.to_string())?;
    store.set(WATCHLIST_KEY, serde_json::json!(config.codes));
    store.set(POLL_INTERVAL_KEY, serde_json::json!(config.poll_interval_ms));
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(AppState::default())
        .setup(|app| {
            let state = app.state::<AppState>();
            load_persisted_config(app.handle(), &state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_quotes,
            get_watchlist,
            set_watchlist,
            show_main_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
