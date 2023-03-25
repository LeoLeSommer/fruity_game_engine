use fruity_game_engine::any::FruityAny;
use fruity_game_engine::{export_impl, export_struct};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, FruityAny)]
#[export_struct]
pub struct FileExplorerState {
    current_dir: String,
}

impl Default for FileExplorerState {
    fn default() -> Self {
        FileExplorerState {
            current_dir: "./examples/test".to_string(),
        }
    }
}

#[export_impl]
impl FileExplorerState {
    pub fn get_current_dir(&self) -> String {
        self.current_dir.clone()
    }

    pub fn open_dir(&mut self, path: &str) {
        self.current_dir = path.to_string();
    }

    pub fn open_dir_script(&mut self, path: &str) {
        self.current_dir = path.to_string();
    }

    pub fn get_files(&self) -> Vec<PathBuf> {
        match fs::read_dir(&self.current_dir) {
            Ok(dir) => dir
                .filter_map(|file| file.ok())
                .map(|file| file.path())
                .collect::<Vec<_>>(),
            Err(_) => Vec::new(),
        }
    }
}
