use btleplug::api::{Central,bleuuid::uuid_from_u16, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::Manager;
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_service = uuid_from_u16(0x7809); // Target service UUID in 16-bit format, as supported by btleplug
    let scan_interval = Duration::from_secs(1); // Interval between scans

    // Initialize BLE manager
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().next().expect("No BLE adapter found");

    // Start scanning for devices with no filter
    central.start_scan(ScanFilter::default()).await?;
    println!("Scanning for devices...");

    // Wait until a device with the target service starts advertising
    let mut advertising_duration = None;
    loop {
        let peripherals = central.peripherals().await?;

        // Check each peripheral for the target service UUID asynchronously
        let mut target_found = false;
        for peripheral in &peripherals {
            if let Some(properties) = peripheral.properties().await.unwrap() {
                if properties.services.contains(&target_service) {
                    println!("Target device with service 0x7809 started advertising.");
                    let start_time = Instant::now();
                    target_found = true;
                    loop {
                        // clean central cache
                        let manager = Manager::new().await?;
                        let adapters = manager.adapters().await?;
                        let central = adapters.into_iter().next().expect("No BLE adapter found");
                        central.start_scan(ScanFilter::default()).await?;
                        sleep(Duration::from_secs(5)).await;
                        let peripherals = central.peripherals().await?;
                        let mut still_advertising = false;

                        // Asynchronously check each peripheral for the target service UUID
                        for p in &peripherals {
                            
                            if let Some(props) = p.properties().await.unwrap() {
                                if props.services.contains(&target_service) {
                                    still_advertising = true;
                                    break;
                                }
                            }
                        }

                        if !still_advertising {
                            advertising_duration = Some(start_time.elapsed());
                            println!("Target device with service 0x7809 stopped advertising.");
                            break;
                        }
                    }
                    break;
                }
            }
        }

        if target_found {
            break;
        }
        sleep(scan_interval).await;
    }

    if let Some(duration) = advertising_duration {
        println!("Advertising duration: {:?}", duration);
    } else {
        println!("Failed to measure advertising duration.");
    }

    Ok(())
}
