use std::sync::Mutex;
use tauri::State;
use haven_core::engine::{VaultEngine, VaultSession};
use haven_core::models::{VaultPayload, Item, Field};

pub struct AppState {
    pub session: Mutex<Option<VaultSession>>,
    pub active_path: Mutex<Option<String>>,
}

#[tauri::command]
fn unlock_vault(state: State<'_, AppState>, path: String, password: String) -> Result<VaultPayload, String> {
    let session = VaultEngine::unlock_vault(&path, &password)?;
    let payload = session.payload.clone();
    *state.session.lock().unwrap() = Some(session);
    *state.active_path.lock().unwrap() = Some(path);
    Ok(payload)
}

#[tauri::command]
fn create_vault(state: State<'_, AppState>, path: String, password: String, vault_name: String) -> Result<VaultPayload, String> {
    let session = VaultEngine::create_vault(&path, &password, vault_name)?;
    let payload = session.payload.clone();
    *state.session.lock().unwrap() = Some(session);
    *state.active_path.lock().unwrap() = Some(path);
    Ok(payload)
}

#[tauri::command]
fn add_item(state: State<'_, AppState>, title: String) -> Result<Item, String> {
    let mut session_guard = state.session.lock().unwrap();
    let session = session_guard.as_mut().ok_or("Vault is not unlocked")?;
    let path_guard = state.active_path.lock().unwrap();
    let path = path_guard.as_ref().ok_or("No active vault path")?;
    let new_item = Item::new(title, None);
    session.payload.items.push(new_item.clone());
    VaultEngine::save_session(session, path)?;
    Ok(new_item)
}

fn main() {
    tauri::Builder::default().manage(AppState {
        session: Mutex::new(None),
        active_path: Mutex::new(None),
    }).invoke_handler(tauri::generate_handler![
        unlock_vault,
        create_vault,
        add_item,
    ]).run(tauri::generate_context!()).expect("error while runnning tauri")
}