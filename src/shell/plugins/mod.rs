pub mod methods;
pub mod namespaces;
mod library_iter;
mod string;

pub(crate) use self::library_iter::*;
pub(crate) use self::string::StringError;

use app_dirs::{app_root, AppDataType, AppInfo};
use std::path::PathBuf;

pub(crate) fn config_dir() -> Option<PathBuf> {
    match app_root(
        AppDataType::UserConfig,
        &AppInfo { name:   "ion", author: "Redox OS Developers" },
    ) {
        Ok(mut path) => {
            path.push("plugins");
            Some(path)
        }
        Err(why) => {
            eprintln!("ion: unable to get config directory: {:?}", why);
            None
        }
    }
}
