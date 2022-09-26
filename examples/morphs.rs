use futures::stream::StreamExt;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use tokio::time;

use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use uuid::Uuid;

async fn find_morphs(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("Morph"))
        {
            return Some(p);
        }
    }
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let morph_command: Uuid = Uuid::from_str("00001101-D102-11E1-9B23-00025B00A5A5").unwrap();
    let morph_response: Uuid = Uuid::from_str("00001102-D102-11E1-9B23-00025B00A5A5").unwrap();

    let manager = Manager::new().await?;

    let central = manager
        .adapters()
        .await
        .expect("Unable to fetch BT adapter list.")
        .into_iter()
        .next()
        .expect("Unable to find ANY adapters.");

    central.start_scan(ScanFilter::default()).await?;

    time::sleep(Duration::from_secs(2)).await;

    let morphs = find_morphs(&central).await.expect("No morph found.");

    morphs.connect().await?;
    morphs.discover_services().await?;

    let characteristics = morphs.characteristics();
    let response_char = characteristics
        .iter()
        .find(|cmd| cmd.uuid == morph_response && cmd.properties.contains(CharPropFlags::NOTIFY))
        .expect("Unable to find cmd.");

    println!("{:?}", response_char);

    println!("pre sub.");
    morphs.subscribe(response_char).await?;
    println!("subbed.");

    let mut stream = morphs.notifications().await?;

    while let Some(data) = stream.next().await {
        println!("{:?}\n{:?}\n", data.uuid, data.value);
    }

    Ok(())
}
