use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;

/// Soubor s daty aplikace: %APPDATA%\cz.haulingpyro.app\hauling-pyro.json
fn state_file(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("hauling-pyro.json"))
}

#[tauri::command]
fn load_state(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let p = state_file(&app)?;
    if p.exists() {
        fs::read_to_string(p).map(Some).map_err(|e| e.to_string())
    } else {
        Ok(None)
    }
}

/// Zápis přes dočasný soubor, aby se data při pádu nepoškodila.
#[tauri::command]
fn save_state(app: tauri::AppHandle, json: String) -> Result<(), String> {
    let p = state_file(&app)?;
    let tmp = p.with_extension("json.tmp");
    fs::write(&tmp, json).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &p).map_err(|e| e.to_string())
}

#[tauri::command]
fn data_path(app: tauri::AppHandle) -> Result<String, String> {
    Ok(state_file(&app)?.display().to_string())
}

/// Záloha do složky Dokumenty, vrací cestu k souboru.
#[tauri::command]
fn export_backup(app: tauri::AppHandle, json: String) -> Result<String, String> {
    let dir = app
        .path()
        .document_dir()
        .or_else(|_| app.path().home_dir())
        .map_err(|e| e.to_string())?;
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let p = dir.join(format!("hauling-pyro-zaloha-{}.json", secs));
    fs::write(&p, json).map_err(|e| e.to_string())?;
    Ok(p.display().to_string())
}

/// Načte konec Game.log (max. 8 MB) – jen čtení, soubor hry se nemění.
#[tauri::command]
fn read_game_log(path: String) -> Result<String, String> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = fs::File::open(&path).map_err(|e| format!("{} ({})", e, path))?;
    let len = f.metadata().map_err(|e| e.to_string())?.len();
    let max: u64 = 8 * 1024 * 1024;
    if len > max {
        f.seek(SeekFrom::Start(len - max)).map_err(|e| e.to_string())?;
    }
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

#[tauri::command]
fn set_always_on_top(window: tauri::WebviewWindow, on: bool) -> Result<(), String> {
    window.set_always_on_top(on).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_state,
            save_state,
            data_path,
            export_backup,
            read_game_log,
            set_always_on_top
        ])
        .run(tauri::generate_context!())
        .expect("chyba při spuštění aplikace Hauling Pyro");
}
