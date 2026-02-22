mod bun_env;
mod commands;
mod config;
mod engine;
mod fs;
mod opencode_router;
mod antonic-agent_server;
mod opkg;
mod orchestrator;
mod paths;
mod platform;
mod types;
mod updater;
mod utils;
mod workspace;

pub use types::*;

use commands::command_files::{
    opencode_command_delete, opencode_command_list, opencode_command_write,
};
use commands::config::{read_opencode_config, write_opencode_config};
use commands::engine::{engine_doctor, engine_info, engine_install, engine_start, engine_stop};
use commands::misc::{
    app_build_info, opencode_db_migrate, opencode_mcp_auth, reset_opencode_cache,
    reset_antonic-agent_state,
};
use commands::opencode_router::{
    opencodeRouter_config_set, opencodeRouter_info, opencodeRouter_start, opencodeRouter_status,
    opencodeRouter_stop,
};
use commands::antonic-agent_server::antonic-agent_server_info;
use commands::opkg::{import_skill, opkg_install};
use commands::orchestrator::{
    orchestrator_instance_dispose, orchestrator_start_detached, orchestrator_status,
    orchestrator_workspace_activate, sandbox_cleanup_antonic-agent_containers, sandbox_doctor,
    sandbox_stop,
};
use commands::scheduler::{scheduler_delete_job, scheduler_list_jobs};
use commands::skills::{
    install_skill_template, list_local_skills, read_local_skill, uninstall_skill, write_local_skill,
};
use commands::updater::updater_environment;
use commands::window::set_window_decorations;
use commands::workspace::{
    workspace_add_authorized_root, workspace_bootstrap, workspace_create, workspace_create_remote,
    workspace_export_config, workspace_forget, workspace_import_config, workspace_antonic-agent_read,
    workspace_antonic-agent_write, workspace_set_active, workspace_update_display_name,
    workspace_update_remote,
};
use engine::manager::EngineManager;
use opencode_router::manager::OpenCodeRouterManager;
use antonic-agent_server::manager::OpenworkServerManager;
use orchestrator::manager::OrchestratorManager;
use tauri::Manager;
use workspace::watch::WorkspaceWatchState;

pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init());

    #[cfg(desktop)]
    let builder = builder
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build());

    let app = builder
        .manage(EngineManager::default())
        .manage(OrchestratorManager::default())
        .manage(OpenworkServerManager::default())
        .manage(OpenCodeRouterManager::default())
        .manage(WorkspaceWatchState::default())
        .invoke_handler(tauri::generate_handler![
            engine_start,
            engine_stop,
            engine_info,
            engine_doctor,
            engine_install,
            orchestrator_status,
            orchestrator_workspace_activate,
            orchestrator_instance_dispose,
            orchestrator_start_detached,
            sandbox_doctor,
            sandbox_stop,
            sandbox_cleanup_antonic-agent_containers,
            antonic-agent_server_info,
            opencodeRouter_info,
            opencodeRouter_start,
            opencodeRouter_stop,
            opencodeRouter_status,
            opencodeRouter_config_set,
            workspace_bootstrap,
            workspace_set_active,
            workspace_create,
            workspace_create_remote,
            workspace_update_display_name,
            workspace_update_remote,
            workspace_forget,
            workspace_add_authorized_root,
            workspace_export_config,
            workspace_import_config,
            opencode_command_list,
            opencode_command_write,
            opencode_command_delete,
            workspace_antonic-agent_read,
            workspace_antonic-agent_write,
            opkg_install,
            import_skill,
            install_skill_template,
            list_local_skills,
            read_local_skill,
            uninstall_skill,
            write_local_skill,
            read_opencode_config,
            write_opencode_config,
            updater_environment,
            app_build_info,
            reset_antonic-agent_state,
            reset_opencode_cache,
            opencode_db_migrate,
            opencode_mcp_auth,
            scheduler_list_jobs,
            scheduler_delete_job,
            set_window_decorations
        ])
        .build(tauri::generate_context!())
        .expect("error while building Antonic Agent");

    // Best-effort cleanup on app exit. Without this, background sidecars can keep
    // running after the UI quits (especially during dev), leading to multiple
    // orchestrator/opencode/antonic-agent-server processes and stale ports.
    app.run(|app_handle, event| {
        if matches!(
            event,
            tauri::RunEvent::ExitRequested { .. } | tauri::RunEvent::Exit
        ) {
            if let Ok(mut engine) = app_handle.state::<EngineManager>().inner.lock() {
                EngineManager::stop_locked(&mut engine);
            }
            if let Ok(mut orchestrator) = app_handle.state::<OrchestratorManager>().inner.lock() {
                OrchestratorManager::stop_locked(&mut orchestrator);
            }
            if let Ok(mut antonic-agent_server) =
                app_handle.state::<OpenworkServerManager>().inner.lock()
            {
                OpenworkServerManager::stop_locked(&mut antonic-agent_server);
            }
            if let Ok(mut opencode_router) =
                app_handle.state::<OpenCodeRouterManager>().inner.lock()
            {
                OpenCodeRouterManager::stop_locked(&mut opencode_router);
            }
        }
    });
}
