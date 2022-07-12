use rust_tuyapi::{Payload, Result, PayloadStruct,tuyadevice::TuyaDevice};
use serde_json::json;
use std::net::IpAddr;
use std::str::FromStr;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn toggle_switch(on: bool) -> Result<()> {
    match on {
        true => println!("Turning on wall socket"),
        false => println!("Turning off wall socket")
    }

    let mut dps = HashMap::new();
    dps.insert("1".to_string(), json!(on));

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32;

    let payload = Payload::Struct(PayloadStruct{
        dev_id: "bf4c7082353ecd63c6gusv".to_string(),
        gw_id: Some("bf4c7082353ecd63c6gusv".to_string()),
        uid: None,
        t: Some(current_time),
        dp_id: None,
        dps: Some(dps),
    });

    let tuya_device = TuyaDevice::create("ver3.3", Some("2a7988f7abfabb68"), IpAddr::from_str("10.0.105.94").unwrap())?;

    tuya_device.set(payload, 0)?;

    Ok(())
}
