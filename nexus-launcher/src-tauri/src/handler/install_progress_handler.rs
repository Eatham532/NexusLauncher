use serde::Serialize;
use tauri::Window;
use piston_lib::processes::installation::HandleProgress;

#[derive(Clone, Serialize)]
pub struct Payload {
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

                         Payload {
                                    progress,
                                    id: id.into(),
                                    message: message.into(),

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