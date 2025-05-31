use async_std::fs::File;
use tauri::AppHandle;
use tauri_plugin_fs::FilePath;

use crate::error::RunicError;

pub struct FileData {
    pub file: File,
    pub size: u64,
    pub name: String,
}

pub async fn open_file(_app: AppHandle, filepath: FilePath) -> Result<FileData, RunicError> {
    match filepath {
        FilePath::Path(pathbuf) => {
            // Access with pathbuf on desktop operating systems

            let file = File::open(&pathbuf).await?;
            let size = file.metadata().await?.len();
            let name = pathbuf
                .file_name()
                .ok_or(RunicError::ParseFileNameError)?
                .to_str()
                .ok_or(RunicError::ParseFileNameError)?
                .to_owned();

            Ok(FileData { file, size, name })
        }

        FilePath::Url(_) => {
            // Only runs if OS isn't Android
            #[cfg(not(target_os = "android"))]
            {
                return Err(RunicError::StringError("URL is not supported".to_string()));
            }

            // Access with URL on Android (why would they treat files like URLs???)
            #[cfg(target_os = "android")]
            {
                use std::os::fd::{FromRawFd, IntoRawFd};
                use tauri_plugin_fs::{FsExt, OpenOptions};

                let options = OpenOptions::new().read(true).to_owned();
                let fd = _app.fs().open(filepath.clone(), options)?.into_raw_fd();

                // // I really don't like this, but it's the only way I found to do it
                // https://www.reddit.com/r/rust/comments/1duc594/comment/lbfubam
                let file = unsafe { File::from_raw_fd(fd) };
                let name = "test.png".to_string(); // TODO
                let size = file.metadata().await?.len();

                return Ok(FileData { file, size, name });
            }
        }
    }
}

pub async fn save_file(_app: AppHandle, filepath: FilePath) -> Result<File, RunicError> {
    match filepath {
        FilePath::Path(pathbuf) => {
            // Access with pathbuf on desktop operating systems

            let file = File::create(&pathbuf).await?;
            Ok(file)
        }

        FilePath::Url(_) => {
            // Only runs if OS isn't Android
            #[cfg(not(target_os = "android"))]
            {
                return Err(RunicError::StringError("URL is not supported".to_string()));
            }

            // Access with URL on Android (why would they treat files like URLs???)
            #[cfg(target_os = "android")]
            {
                use std::os::fd::{FromRawFd, IntoRawFd};
                use tauri_plugin_fs::{FsExt, OpenOptions};

                let options = OpenOptions::new().create(true).write(true).to_owned();
                let fd = _app.fs().open(filepath.clone(), options)?.into_raw_fd();

                // // I really don't like this, but it's the only way I found to do it
                // https://www.reddit.com/r/rust/comments/1duc594/comment/lbfubam
                let file = unsafe { File::from_raw_fd(fd) };

                return Ok(file);
            }
        }
    }
}
