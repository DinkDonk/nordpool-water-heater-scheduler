use rust_tuyapi::{Payload, Result, PayloadStruct,tuyadevice::TuyaDevice};
use serde_json::json;
use std::net::IpAddr;
use std::str::FromStr;
use std::collections::HashMap;
use std::time::SystemTime;
use config::Config;

pub fn toggle_switch(on: bool) -> Result<()> {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("Settings"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let device_ip: String = settings.get::<String>("device_ip").expect("`device_ip` needs to be set in Settings.toml");
    let device_id: String = settings.get::<String>("device_id").expect("`device_id` needs to be set in Settings.toml");
    let device_local_key: String = settings.get::<String>("device_local_key").expect("`device_local_key` needs to be set in Settings.toml");

    match on {
        true => println!("Turning on wall socket"),
        false => println!("Turning off wall socket")
    }

    let mut dps = HashMap::new();
    dps.insert("1".to_string(), json!(on));

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32;

    let payload = Payload::Struct(PayloadStruct{
        dev_id: device_id.clone(),
        gw_id: Some(device_id),
        uid: None,
        t: Some(current_time),
        dp_id: None,
        dps: Some(dps),
    });

    let tuya_device = TuyaDevice::create("ver3.3", Some(&device_local_key), IpAddr::from_str(&device_ip).unwrap())?;

    tuya_device.set(payload, 0)?;

    Ok(())
}
