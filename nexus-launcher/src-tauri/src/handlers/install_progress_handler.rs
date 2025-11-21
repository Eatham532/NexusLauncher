use serde::Serialize;
use tauri::Window;
use piston_lib::processes::launcher::installation::HandleProgress;

#[derive(Clone, Serialize)]
struct InstallPayload {
    progress: i32,
    id: String,
    message: String,
}

pub struct InstallProgressHandler {
    window: Window
}

impl HandleProgress for InstallProgressHandler {
    fn update_progress(&self, progress: i32, id: &str, message: &str) {
        println!("{}", message);
        self.window.emit("game-install-progress",

                         InstallPayload {
                                    progress,
                                    id: id.to_string(),
                                    message: message.to_string(),

                         }).unwrap();
    }
}

impl InstallProgressHandler {
    pub fn new(window: Window) -> Self {
        InstallProgressHandler {
            window
        }
    }
}