use bluest::{Adapter, Device};

use anyhow::Result;
use chrono::{Duration, Utc};
use flutter_rust_bridge::StreamSink;
use futures_util::{pin_mut, StreamExt};
use local_ip_address::local_ip;
use os_info::*;
use rxing;
use std::error::Error;
use std::pin::Pin;
use std::str::FromStr;
use tokio::sync::OnceCell;
use tracing::info;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub name: Option<String>,
    pub address: Option<String>,
    pub status: bool,
    pub service_uuid: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LocalIP {
    pub address: String,
    pub is_ipv4: bool,
    pub is_ipv6: bool,
}

#[derive(Debug)]

pub struct CameraScanner {
    pub decode_text: Option<String>,
    pub status: bool,
}

impl CameraScanner {
    fn new(decode_text: Option<String>, status: bool) -> Self {
        CameraScanner {
            decode_text,

            status,
        }
    }
}

impl LocalIP {
    fn new(address: String, is_ipv4: bool, is_ipv6: bool) -> Self {
        LocalIP {
            address,
            is_ipv4,
            is_ipv6,
        }
    }
}

impl BluetoothDevice {
    fn new(
        name: Option<String>,
        address: Option<String>,
        status: bool,
        service_uuid: Vec<String>,
    ) -> Self {
        BluetoothDevice {
            name,
            address,
            status,
            service_uuid,
        }
    }
}

// impl support::IntoDart for BluetoothDevice {
//     fn into_dart(self) -> support::ffi::DartCObject {
//         self.into_dart()
//     }
// }

#[derive(Debug, Clone)]
pub struct MethodChannel {
    pub command: String,
    pub device: BluetoothDevice,
    pub bytes: Option<Vec<u8>>,
}

impl MethodChannel {
    fn new(command: String, device: BluetoothDevice, bytes: Option<Vec<u8>>) -> Self {
        MethodChannel {
            command: command,
            device: device,
            bytes: bytes,
        }
    }
}

#[tokio::main]
pub async fn invoke_method(channel: MethodChannel) -> Result<String, Box<dyn Error>> {
    let mut result = String::from("");
    let backup_channel = MethodChannel::new(channel.command, channel.device, channel.bytes).clone();

    println!("{}", result);

    let adapter = Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found");

    let pinned_device: Option<bluest::Device>;

    let uuid: Uuid = Uuid::from_str(&backup_channel.device.service_uuid.first().unwrap()).unwrap();

    if let Ok(_adapter_exist) = adapter {
        let device = _adapter_exist
            .discover_devices(&[uuid])
            .await?
            .next()
            .await
            .ok_or("Failed to discover device")??;

        pinned_device = Some(device);

        pin_mut!(pinned_device);

        let borrowed_device_pin: Pin<&mut Option<bluest::Device>> = pinned_device;

        match backup_channel.command.as_str() {
            "connect" => {
                if let Some(device) = borrowed_device_pin.clone() {
                    let connect = _adapter_exist
                        .open_device(&borrowed_device_pin.clone().unwrap().id())
                        .await;
                    if let Ok(_connect_result) = connect {
                        result = format!(
                            "success-connected to {}",
                            device.name().as_deref().unwrap_or("(unknown)"),
                        );
                        println!("{}", result);
                    } else {
                        result = format!(
                            "error-connection to {}",
                            device.name().as_deref().unwrap_or("(unknown)"),
                        );
                        println!("{}", result);
                    }
                }
            }
            "write_bytes" => {
                if let Some(_) = &borrowed_device_pin.clone() {
                    if find_device(
                        &borrowed_device_pin.clone().unwrap(),
                        &backup_channel.device.address.unwrap(),
                    ) {
                        let service = match borrowed_device_pin
                            .clone()
                            .unwrap()
                            .discover_services_with_uuid(uuid)
                            .await?
                            .get(0)
                        {
                            Some(service) => service.clone(),
                            None => return Err("service not found".into()),
                        };

                        info!("found printer service");

                        let characteristics = service.discover_characteristics().await?;
                        info!("discovered characteristics");

                        info!("start print");
                        let res = characteristics[1]
                            .write(&backup_channel.bytes.unwrap())
                            .await
                            .is_ok();

                        if res {
                            result = "Print Success".to_string();
                        }
                        println!("{}", result);
                    }
                }
            }

            _ => {
                result = "Command not found".to_string();
                println!("{}", result);
            }
        }
    }

    Ok(result)
}

/// FIND LOCAL IP
#[tokio::main]
pub async fn find_my_local_ip() -> Result<LocalIP, Box<dyn Error>> {
    let my_local_ip = LocalIP::new(
        local_ip().unwrap().to_string(),
        local_ip().unwrap().is_ipv4(),
        local_ip().unwrap().is_ipv6(),
    );
    println!("This is my local IP address: {:?}", my_local_ip);

    Ok(my_local_ip)
}

fn find_device(central: &Device, address: &String) -> bool {
    if central.id().to_string().contains(address) {
        return true;
    }
    false
}

/// INIT LOGGER
#[tokio::main]
pub async fn init_() {
    let _registry = tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
}

/// CHECK BLUETOOTH IS EXIST OR NOT
#[tokio::main]
pub async fn get_bluetooth_adapter_state() -> Result<bool, Box<dyn Error>> {
    let adapter_manager = Adapter::default()
        .await
        .ok_or("Bluetooth Adapter Not Found");

    if let Ok(_adapter) = adapter_manager {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// INFO : CONNECT TO DEVICE
#[tokio::main]
pub async fn connect_to_bluetooth_device(service_uuid: String) -> Result<bool, Box<dyn Error>> {
    let adapter = Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found");

    if let Ok(_adapter_exist) = adapter {
        let uuid: Uuid = Uuid::from_str(&service_uuid).unwrap();

        let device = _adapter_exist
            .discover_devices(&[uuid])
            .await?
            .next()
            .await
            .ok_or("Failed to discover device")??;

        let connect = _adapter_exist.connect_device(&device).await;
        if let Ok(_connect_result) = connect {
            info!(
                "connected to {:?}",
                device.name().as_deref().unwrap_or("(unknown)")
            );
        } else {
            info!("error, skipping connection");
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tokio::main]
pub async fn disconnect_bluetooth_device(service_uuid: String) -> Result<bool, Box<dyn Error>> {
    let adapter = Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found")?;
    adapter.wait_available().await?;

    let uuid: Uuid = Uuid::from_str(&service_uuid).unwrap();

    let device = adapter
        .discover_devices(&[uuid])
        .await?
        .next()
        .await
        .ok_or("Failed to discover device")??;

    let connect = adapter.disconnect_device(&device).await;
    if let Ok(_connect_result) = connect {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// INFO : WRITE BYTES TO PRINTER
#[tokio::main]
pub async fn bluetooth_write_bytes(
    service_uuid: String,
    address: String,
    data: Vec<u8>,
) -> Result<bool, Box<dyn Error>> {
    let adapter = Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found")?;

    let mut loop_devices = adapter.scan(&[]).await?;

    let advertising_device = loop_devices
        .next()
        .await
        .into_iter()
        .find(|d| d.device.id().to_string().contains(&address))
        .unwrap();

    let uuid: Uuid = Uuid::from_str(&service_uuid).unwrap();
    let reconnect_device = adapter.open_device(&advertising_device.device.id()).await?;

    let service = match reconnect_device
        .discover_services_with_uuid(uuid)
        .await?
        .get(0)
    {
        Some(service) => service.clone(),
        None => return Err("service not found".into()),
    };
    info!("found printer service");

    let characteristics = service.discover_characteristics().await?;
    info!("discovered characteristics");

    info!("start print");
    let res = characteristics[1].write(&data).await.is_ok();

    Ok(res)
}

static THREAD_RUNTIME: OnceCell<StreamSink<BluetoothDevice>> = OnceCell::const_new();
/// INFO : STREAM DISCOVER BLUETOOTH DEVICE
#[tokio::main]
pub async fn bluetooth_start_scan(
    s: StreamSink<BluetoothDevice>,
    timeout_sec: Duration,
) -> Result<(), Box<dyn Error>> {
    let _coba = THREAD_RUNTIME.set(s.to_owned());

    let adapter = Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found")?;
    adapter.wait_available().await?;

    let mut scan = adapter.scan(&[]).await?;

    let time_out_time = Utc::now() + timeout_sec;
    pin_mut!(time_out_time);

    while let Some(value) = scan.next().await {
        let mut service_uuids: Vec<String> = Vec::new();
        if !value.adv_data.services.is_empty() && value.adv_data.is_connectable != false {
            for sid in value.adv_data.services.iter() {
                service_uuids.push(sid.to_string());
            }

            let bluetooth_device = BluetoothDevice::new(
                Some(value.device.name().unwrap_or(String::from("Unknown"))),
                Some(value.device.id().to_string()),
                value.adv_data.is_connectable,
                service_uuids,
            );

            info!("{:?}", bluetooth_device);
            s.add(bluetooth_device);
            if Utc::now().timestamp_micros() > time_out_time.timestamp_micros() {
                s.close();
                println!("close stream after {} second", timeout_sec.num_seconds());
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn bluetooth_stop_scan() -> Result<()> {
    let _rt = THREAD_RUNTIME.get().expect("Error");
    _rt.close();
    tokio::spawn(async move {});
    Ok(())
}

#[tokio::main]
pub async fn scanner_process_image(image_path: String) -> Result<CameraScanner, Box<dyn Error>> {
    let mut cam_result = CameraScanner::new(Some("".into()), false);

    let scanner = rxing::helpers::detect_multiple_in_file(&image_path).unwrap();

    for scan_result in scanner {
        println!(
            "Barcode Format {} -> Result {}",
            scan_result.getBarcodeFormat(),
            scan_result.getText()
        );
        cam_result.decode_text = Some(scan_result.getText().to_string());
        cam_result.status = scan_result.getText().to_string() == "";

        if scan_result.getText().to_string() != "" {
            break;
        }
    }

    Ok(cam_result)
}

#[derive(Debug, Clone)]
pub struct WindowsOSInfo {
    pub os_type: String,
    pub version: String,
    pub edition: String,
    pub code_name: String,
    pub bitness: String,
    pub architecture: String,
}

impl WindowsOSInfo {
    fn new(
        os_type: String,
        version: String,
        edition: String,
        code_name: String,
        bitness: String,
        architecture: String,
    ) -> Self {
        WindowsOSInfo {
            os_type,
            version,
            edition,
            code_name,
            bitness,
            architecture,
        }
    }
}

#[tokio::main]
pub async fn get_windows_info() -> Result<WindowsOSInfo, Box<dyn std::error::Error>> {
    let info = get();

    println!("{:?}", info);

    let windows_info = WindowsOSInfo::new(
        info.os_type().to_string(),
        info.version().to_string(),
        info.edition().unwrap_or("unknown").to_string(),
        info.codename().unwrap_or("unknown").to_string(),
        info.bitness().to_string(),
        info.architecture().unwrap_or("unknown").to_string(),
    );

    Ok(windows_info)
}
