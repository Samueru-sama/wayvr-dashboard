use tauri::{Emitter, Manager};
use wayvr_ipc::{
	client::{WayVRClient, WayVRClientMutex},
	ipc,
};

use crate::util::steam_bridge::SteamBridge;

pub struct AppState {
	pub steam_bridge: SteamBridge,
	pub wayvr_client: Option<WayVRClientMutex>,
	pub serial_generator: ipc::SerialGenerator,
}

impl AppState {
	pub async fn new() -> anyhow::Result<Self> {
		let serial_generator = ipc::SerialGenerator::new();

		let steam_bridge = SteamBridge::new()?;

		let wayvr_client = match WayVRClient::new("WayVR Dashboard").await {
			Ok(c) => Some(c),
			Err(e) => {
				log::error!("WayVR Client failed to initialize: {}", e);
				None
			}
		};

		Ok(Self {
			steam_bridge,
			wayvr_client,
			serial_generator,
		})
	}

	pub async fn configure_signal_handler(handle: tauri::AppHandle) {
		let app = handle.app_handle().state::<AppState>();
		let Some(wayvr_client) = &app.wayvr_client else {
			return;
		};

		let mut client = wayvr_client.lock().await;

		// configure signal handler
		let handle = handle.clone();
		client.on_signal = Some(Box::new(move |signal| match signal {
			wayvr_ipc::client::Signal::WvrStateChanged(wvr_state_changed) => {
				if let Err(e) = handle.emit("signal_state_changed", wvr_state_changed) {
					log::error!("Failed to send signal: {:?}", e);
				}
			}
		}));
	}

	pub fn configure_async(handle: tauri::AppHandle) {
		tokio::spawn(async move {
			AppState::configure_signal_handler(handle).await;
		});
	}
}
