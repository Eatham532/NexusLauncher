use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct ProfileOptions {
    pub MaxMemorey: u64,
    pub MinMemory: u64,
    pub JavaArgs: String,
}

impl ProfileOptions {
    pub fn new(min: u64, max: u64) -> Self {
        Self {
            JavaArgs: String::new(),
            MinMemory: min,
            MaxMemorey: max,
        }
    }
}