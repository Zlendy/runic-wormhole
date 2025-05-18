use std::os::fd::{FromRawFd, IntoRawFd};

use async_std::fs::File;
use serde::Serialize;
use tauri::{ipc::Channel, AppHandle};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::{FsExt, OpenOptions};
use wormhole::{
    transfer::TransferError,
    transit::{ConnectionType, RelayHint},
    MailboxConnection, Wormhole, WormholeError,
};

#[derive(Debug, thiserror::Error)]
enum RunicError {
    #[error(transparent)]
    WormholeError(#[from] WormholeError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    TransferError(#[from] TransferError),

    #[error("{0}")]
    StringError(String),

    #[error("Could not parse filename")]
    ParseFileNameError,
}

// we must manually implement serde::Serialize
impl serde::Serialize for RunicError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
enum WormholeEvent {
    PickingFile,
    MailboxConnecting,
    MailboxConnected { code: String },
    Progress { sent: u64, total: u64 },
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn send_file(app: AppHandle, channel: Channel<WormholeEvent>) -> Result<(), RunicError> {
    channel.send(WormholeEvent::PickingFile).unwrap();

    let Some(path) = app.dialog().file().blocking_pick_file() else {
        return Err(RunicError::StringError("Cancelled".to_string()));
    };

    let options = OpenOptions::new().read(true).to_owned();
    let fd = app.fs().open(path.clone(), options)?.into_raw_fd();

    let filename = "test.png".to_string(); // TODO

    // // I really don't like this, but it's the only way I found to do it
    // https://www.reddit.com/r/rust/comments/1duc594/comment/lbfubam
    let mut file = unsafe { File::from_raw_fd(fd) };

    let metadata = file.metadata().await?;

    println!("file: {}", filename);

    channel.send(WormholeEvent::MailboxConnecting).unwrap();

    let app_config = wormhole::AppConfig {
        id: wormhole::AppID::new("lothar.com/wormhole/text-or-file-xfer"),
        rendezvous_url: "ws://relay.magic-wormhole.io:4000/v1".to_string().into(),
        app_version: wormhole::transfer::AppVersion::default(),
    };

    let connection = MailboxConnection::create(app_config, 4).await?;

    println!("CODE: {}", connection.code());
    channel
        .send(WormholeEvent::MailboxConnected {
            code: connection.code().to_string(),
        })
        .unwrap();

    let wormhole = Wormhole::connect(connection).await?;

    let relay_hints = vec![RelayHint::from_urls(
        None,
        ["tcp://transit.magic-wormhole.io:4001".parse().unwrap()],
    )
    .unwrap()];

    wormhole::transfer::send_file(
        wormhole,
        relay_hints,
        &mut file,
        filename,
        metadata.len(),
        wormhole::transit::Abilities::ALL,
        |info| {
            let conn_type = format!("{:#?}", info.conn_type);
            let peer_addr = info.peer_addr.to_string();

            if info.conn_type == ConnectionType::Direct {
                println!("Connecting {} to {}", conn_type, peer_addr);
            } else {
                println!("Connecting {}", conn_type);
            };
        },
        move |sent, total| {
            channel
                .send(WormholeEvent::Progress { sent, total })
                .unwrap();
        },
        std::future::pending(), // TODO: Implement cancel
    )
    .await?;

    println!("DONE");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![send_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
