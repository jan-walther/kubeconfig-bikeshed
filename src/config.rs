use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

const ACTIVE_FILE_NAME: &str = "active";

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    // attempt to respect XDG_CONFIG_HOME, fall back to $HOME/.config if it's not set.
    let path = match env::var("XDG_CONFIG_HOME") {
        Ok(s) => Path::new(&s).to_path_buf(),
        Err(_) => {
            let path = home::home_dir().ok_or("could not determine home directory")?;
            path.join(".config")
        }
    };

    Ok(path.join("kbs"))
}

pub fn get_last_active(config_path: &Path) -> io::Result<String> {
    fs::read_to_string(config_path.join(ACTIVE_FILE_NAME))
}

pub fn save_last_active(config_path: &Path, name: &String) -> io::Result<()> {
    fs::write(config_path.join(ACTIVE_FILE_NAME), name)
}
