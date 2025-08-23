pub mod error;
pub mod fs;
pub mod util;

use std::sync::Arc;

use chrono::{DateTime, Utc};
use error::RunicError;
use serde::Serialize;
use tauri::{ipc::Channel, AppHandle, Listener};
use tauri_plugin_dialog::DialogExt;
use tokio::sync::Notify;
use util::cancellable;
use wormhole::{
    transit::{ConnectionType, RelayHint},
    MailboxConnection, Wormhole,
};

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
#[tauri::command(async)]
async fn send_file(app: AppHandle, channel: Channel<WormholeEvent>, code_length: usize) -> Result<(), RunicError> {
    let mut last_progress = DateTime::<Utc>::default();

    let notify_cancel_write = Arc::new(Notify::new());
    let notify_cancel_read = notify_cancel_write.clone();

    app.once("cancel", move |_| {
        notify_cancel_write.notify_one();
    });

    channel.send(WormholeEvent::PickingFile).unwrap();

    let Some(filepath) = app.dialog().file().blocking_pick_file() else {
        return Err(RunicError::Cancelled);
    };

    let fs::FileData {
        mut file,
        size,
        name,
    } = fs::open_file(app, filepath).await?;

    println!("file: {}", name);

    channel.send(WormholeEvent::MailboxConnecting).unwrap();

    let app_config = wormhole::AppConfig {
        id: wormhole::AppID::new("lothar.com/wormhole/text-or-file-xfer"),
        rendezvous_url: "ws://relay.magic-wormhole.io:4000/v1".to_string().into(),
        app_version: wormhole::transfer::AppVersion::default(),
    };

    let connection = MailboxConnection::create(app_config, code_length).await?;

    println!("CODE: {}", connection.code());
    channel
        .send(WormholeEvent::MailboxConnected {
            code: connection.code().to_string(),
        })
        .unwrap();

    let wormhole =
        cancellable(Wormhole::connect(connection), notify_cancel_read.notified()).await??;

    let relay_hints = vec![RelayHint::from_urls(
        None,
        ["tcp://transit.magic-wormhole.io:4001".parse().unwrap()],
    )
    .unwrap()];

    cancellable(
        wormhole::transfer::send_file(
            wormhole,
            relay_hints,
            &mut file,
            name,
            size,
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
                let now = Utc::now();
                let diff = now - last_progress;

                if diff.num_milliseconds() < 50 {
                    return;
                }

                last_progress = now;

                channel
                    .send(WormholeEvent::Progress { sent, total })
                    .unwrap();
            },
            std::future::pending(),
        ),
        notify_cancel_read.notified(),
    )
    .await??;

    println!("DONE");
    Ok(())
}

#[tauri::command(async)]
async fn receive_file(
    app: AppHandle,
    channel: Channel<WormholeEvent>,
    code: String,
) -> Result<(), RunicError> {
    let mut last_progress = DateTime::<Utc>::default();

    let notify_cancel_write = Arc::new(Notify::new());
    let notify_cancel_read = notify_cancel_write.clone();

    app.once("cancel", move |_| {
        notify_cancel_write.notify_one();
    });

    channel.send(WormholeEvent::MailboxConnecting).unwrap();

    let app_config = wormhole::AppConfig {
        id: wormhole::AppID::new("lothar.com/wormhole/text-or-file-xfer"),
        rendezvous_url: "ws://relay.magic-wormhole.io:4000/v1".to_string().into(),
        app_version: wormhole::transfer::AppVersion::default(),
    };

    let connection = MailboxConnection::connect(app_config, wormhole::Code(code), false).await?;

    println!("CODE: {}", connection.code());
    channel
        .send(WormholeEvent::MailboxConnected {
            code: connection.code().to_string(),
        })
        .unwrap();

    let wormhole =
        cancellable(Wormhole::connect(connection), notify_cancel_read.notified()).await??;

    let relay_hints = vec![RelayHint::from_urls(
        None,
        ["tcp://transit.magic-wormhole.io:4001".parse().unwrap()],
    )
    .unwrap()];

    let request = cancellable(
        wormhole::transfer::request_file(
            wormhole,
            relay_hints,
            wormhole::transit::Abilities::ALL,
            std::future::pending(),
        ),
        notify_cancel_read.notified(),
    )
    .await??
    .ok_or(RunicError::Cancelled)?;

    let filename = request.file_name();
    let filepath = app
        .dialog()
        .file()
        .set_file_name(filename)
        .blocking_save_file()
        .ok_or(RunicError::Cancelled)?;

    let mut file = fs::save_file(app, filepath).await?;

    cancellable(
        request.accept(
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
                let now = Utc::now();
                let diff = now - last_progress;

                if diff.num_milliseconds() < 50 {
                    return;
                }

                last_progress = now;

                channel
                    .send(WormholeEvent::Progress { sent, total })
                    .unwrap();
            },
            &mut file,
            std::future::pending(),
        ),
        notify_cancel_read.notified(),
    )
    .await??;

    println!("DONE");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![send_file, receive_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
