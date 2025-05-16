use std::time::Duration;

use async_std::{fs::File, task::sleep};
use wormhole::{
    transfer::TransferError,
    transit::{ConnectionType, RelayHint, TransitInfo},
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

fn transit_handler(info: TransitInfo) {
    let conn_type = format!("{:#?}", info.conn_type);
    let peer_addr = info.peer_addr.to_string();

    if info.conn_type == ConnectionType::Direct {
        println!("Connecting {} to {}", conn_type, peer_addr);
    } else {
        println!("Connecting {}", conn_type);
    };
}

fn progress_handler(sent: u64, total: u64) {
    println!("Progress: {}/{}", sent, total);
}

async fn cancel() {
    // TODO: Wait until file is fully transferred
    sleep(Duration::from_secs(20)).await;
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command()]
async fn send_file(path: String) -> Result<String, RunicError> {
    let app_config = wormhole::AppConfig {
        id: wormhole::AppID::new("lothar.com/wormhole/text-or-file-xfer"),
        rendezvous_url: "ws://relay.magic-wormhole.io:4000/v1".to_string().into(),
        app_version: wormhole::transfer::AppVersion::default(),
    };

    let connection = MailboxConnection::create(app_config, 4).await?;

    println!("CODE: {}", connection.code());

    let wormhole = Wormhole::connect(connection).await?;

    let relay_hints = vec![RelayHint::from_urls(
        None,
        ["tcp://transit.magic-wormhole.io:4001".parse().unwrap()],
    )
    .unwrap()];

    let mut file = File::open(path).await?;

    let metadata = file.metadata().await?;

    wormhole::transfer::send_file(
        wormhole,
        relay_hints,
        &mut file,
        "filename.png".to_string(),
        metadata.len(),
        wormhole::transit::Abilities::ALL,
        transit_handler,
        progress_handler,
        cancel(),
    )
    .await?;

    Ok("DONE".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![send_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
