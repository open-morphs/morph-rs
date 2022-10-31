use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use tokio::time;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use uuid::Uuid;

const COMMAND_UUID: &str = "00001101-D102-11E1-9B23-00025B00A5A5";

async fn find_morphs(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("LE-Morph"))
        {
            return Some(p);
        }
    }
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let command = Uuid::from_str(COMMAND_UUID).unwrap();

    dbg!("INIT MANAGER.");
    let manager = Manager::new().await?;

    dbg!("INIT CENTRAL.");
    let central = manager
        .adapters()
        .await
        .expect("Unable to fetch BT adapter list.")
        .into_iter()
        .next()
        .expect("Unable to find ANY adapters.");

    dbg!("SCAN.");
    central.start_scan(ScanFilter::default()).await?;

    dbg!("WAIT.");
    time::sleep(Duration::from_secs(4)).await;

    dbg!("FIND MORPHS.");
    let morphs = find_morphs(&central).await.expect("No morph found.");
    println!("{:?}", morphs);

    dbg!("CONN MORPHS.");
    morphs.connect().await?;
    morphs.discover_services().await?;

    dbg!("CONN DONE. GET CHARA.");
    let characteristics = morphs.characteristics();
    dbg!("GET CMD CHARA.");
    let cmd_char = characteristics
        .iter()
        .find(|cmd| cmd.uuid == command)
        .expect("Unable to find cmd.");

    let payload = vec![0x0a, 0x66, 0x00, 0x0E, 0x03];
    dbg!("PLAY TONE!");
    morphs
        .write(
            &cmd_char,
            &payload,
            btleplug::api::WriteType::WithoutResponse,
        )
        .await?;

    dbg!("FIN.");

    Ok(())
}
