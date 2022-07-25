<p align="center"><img src="./assets/logo.svg" width="200"></p>

Nordpool Water Heater Scheduler
===============================

Keeps your water heater on for the 8 cheapest hours every day.  
Based on realtime data from [Nordpool](https://www.nordpoolgroup.com/en/Market-data1/#/nordic/table).

## Requirements

- [Nedis Smart Power Plug WIFIPO120xxx](https://nedis.no/no-no/product/sikkerhet-personvern/smart-home/strom/550710067/smartlife-smart-plug-wi-fi-ip44-efektmaler-3680-w-jordet-kontakt-type-f-cee-77-30-40-0c-android-ios-gra-hvit) (Thanks for choosing the worst IoT power plug ever made Aslak!)
- [Gratis](https://github.com/repaper/gratis/tree/master/PlatformWithOS) (EPD fuse needs to be running, and xbm2bin need to be in PATH)

## Setup

1. Give the Nedis smart power plug a static IP in your DHCP server
2. To get the Nedis smart power plug device `IP`, `ID` and `local key` [Follow this guide](https://github.com/jasonacox/tinytuya)
3. Fill in `device_ip`, `device_id` and `device_local_key` in `Settings.toml`

## Build

Build for Raspberry PI 64 bit:

```bash
cargo build --target aarch64-unknown-linux-musl --release
```

## Troubleshooting

### Power plug is linked to `Nedis SmartLife` app, and not `Tuya Smart` app

1. Reset the power plug by holding the power button for 6 seconds until the LED starts blinking.
2. Add the power plug in the `Tuya Smart` app.

## Links

- [Nordpool API](https://developers.nordpoolgroup.com/)
- [TuyAPI](https://github.com/EmilSodergren/rust-tuyapi)
