use tauri::State;

use crate::antonic-agent_server::manager::OpenworkServerManager;
use crate::types::OpenworkServerInfo;

#[tauri::command]
pub fn antonic-agent_server_info(manager: State<OpenworkServerManager>) -> OpenworkServerInfo {
    let mut state = manager
        .inner
        .lock()
        .expect("antonic-agent server mutex poisoned");
    OpenworkServerManager::snapshot_locked(&mut state)
}

// start/stop are handled by engine lifecycle
