use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, RwLock,
};
use std::thread;
use std::time::{Duration, Instant};

use enigo::{Enigo, MouseButton, MouseControllable};
use rand::Rng;
use rdev::{listen, EventType, Key};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClickConfig {
    pub button: String,
    pub interval_ms: u64,
    pub random_minus: u64,
    pub random_plus: u64,
    pub follow_cursor: bool,
    pub fixed_x: Option<i32>,
    pub fixed_y: Option<i32>,
}

impl Default for ClickConfig {
    fn default() -> Self {
        Self {
            button: "left".into(),
            interval_ms: 50,
            random_minus: 0,
            random_plus: 0,
            follow_cursor: true,
            fixed_x: None,
            fixed_y: None,
        }
    }
}

pub struct AppState {
    pub running: Arc<AtomicBool>,
    pub cfg: Arc<RwLock<ClickConfig>>,
    pub cfg_version: Arc<AtomicU64>,
    pub clicks: Arc<AtomicU64>
}

mod cmds {
    use super::*;

    #[tauri::command]
    pub fn update_click_config(new_cfg: ClickConfig, state: State<'_, AppState>) -> Result<(), String> {
        {
            let mut w = state.cfg.write().map_err(|_| "cfg lock poisoned".to_string())?;
            *w = new_cfg;
        }
        state.cfg_version.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    #[tauri::command]
    pub fn start_clicking(state: State<'_, AppState>) -> Result<(), String> {
        if state.running.swap(true, Ordering::SeqCst) {
            return Ok(());
        }

        let running = state.running.clone();
        let cfg_arc = state.cfg.clone();
        let clicks = state.clicks.clone();

        thread::spawn(move || {
            let mut enigo = Enigo::new();
            let mut rng = rand::thread_rng();
            let mut last_ms = 0u64;

            while running.load(Ordering::SeqCst) {
                let c = { cfg_arc.read().unwrap().clone() };

                if c.interval_ms != last_ms {
                    //println!("[worker] interval_ms = {}", c.interval_ms);
                    last_ms = c.interval_ms;
                }

                let btn = match c.button.as_str() {
                    "right" => MouseButton::Right,
                    "middle" => MouseButton::Middle,
                    _ => MouseButton::Left,
                };

                if !c.follow_cursor {
                    if let (Some(x), Some(y)) = (c.fixed_x, c.fixed_y) {
                        enigo.mouse_move_to(x, y);
                    }
                }

                enigo.mouse_click(btn);
                clicks.fetch_add(1, Ordering::SeqCst);
                
                let base = c.interval_ms.max(1);
                let neg = if c.random_minus == 0 { 0 } else { rng.gen_range(0..=c.random_minus) };
                let pos = if c.random_plus == 0 { 0 } else { rng.gen_range(0..=c.random_plus) };
                let sleep_ms = base.saturating_sub(neg).saturating_add(pos).max(1);
                thread::sleep(Duration::from_millis(sleep_ms));
            }
        });

        Ok(())
    }

    #[tauri::command]
    pub fn stop_clicking(state: State<'_, AppState>) -> Result<(), String> {
        state.running.store(false, Ordering::SeqCst);
        Ok(())
    }

    #[tauri::command]
    pub fn is_running(state: State<'_, AppState>) -> Result<bool, String> {
        Ok(state.running.load(Ordering::SeqCst))
    }

    #[tauri::command]
    pub fn get_clicks(state: State<'_, AppState>) -> Result<u64, String> {
        Ok(state.clicks.load(Ordering::SeqCst))
    }

    #[tauri::command]
    pub fn clear_clicks(state: State<'_, AppState>) -> Result<(), String> {
        state.clicks.store(0, Ordering::SeqCst);
        Ok(())
    }
}

fn on_f6_pressed(app_handle: &AppHandle) {
    let state: State<'_, AppState> = app_handle.state::<AppState>();

    if state.running.load(Ordering::SeqCst) {
        //println!("[F6] stop");
        state.running.store(false, Ordering::SeqCst);
        return;
    }

    //println!("[F6] request cfg + start");

    let before = state.cfg_version.load(Ordering::SeqCst);
    let _ = app_handle.emit("request-cfg", ());
    let deadline = Instant::now() + Duration::from_millis(200);
    while Instant::now() < deadline {
        if state.cfg_version.load(Ordering::SeqCst) != before {
            break;
        }
        thread::sleep(std::time::Duration::from_millis(5));
    }

    let _ = cmds::start_clicking(state);
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            running: Arc::new(AtomicBool::new(false)),
            cfg: Arc::new(RwLock::new(ClickConfig::default())),
            cfg_version: Arc::new(AtomicU64::new(0)),
            clicks: Arc::new(AtomicU64::new(0))
        })
        .invoke_handler(tauri::generate_handler![
            cmds::update_click_config,
            cmds::start_clicking,
            cmds::stop_clicking,
            cmds::is_running,
            cmds::get_clicks,
            cmds::clear_clicks
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            std::thread::spawn(move || {
                if let Err(e) = listen(move |event| {
                    if let EventType::KeyPress(key) = event.event_type {
                        if key == Key::F6 {
                            on_f6_pressed(&app_handle);
                        }
                    }
                }) {
                    eprintln!("[hotkey] listen error: {:?}", e);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running app");
}