use gtk::{Builder};
use std::path::PathBuf;
#[derive(Clone)]
pub struct LoadUI{
    pub build: Builder,
    ui_path: String,
    dir_file: PathBuf,
}

impl LoadUI{

    pub fn get_ui(path_ui: &str, dir: &PathBuf) -> Self{
        let _build: Builder = Builder::from_file(
                dir.parent()
                    .unwrap()
                    .join(path_ui)
                    .to_string_lossy()
                    .to_string(),
            );

        LoadUI{ 
            build : _build, 
            ui_path: path_ui.to_string(), 
            dir_file: dir.clone()
        }
    }

    pub fn get_new_ui(&self) -> LoadUI{
        let _build: Builder = Builder::from_file(
                self.dir_file.parent()
                    .unwrap()
                    .join(self.ui_path.as_str())
                    .to_string_lossy()
                    .to_string(),
            );
        LoadUI { 
            build: _build, 
            ui_path: self.ui_path.to_string(),
            dir_file: self.dir_file.clone()
        }
    }
}