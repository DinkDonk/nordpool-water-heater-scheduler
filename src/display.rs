use image;
use std::process::Command;

mod sprites;
use crate::nordpool::Price;
use crate::nordpool;

const WIDTH:u32 = 176;
const HEIGHT:u32= 264;

pub fn draw(prices: Vec<Price>) {
    let min_price = (nordpool::get_min_price(&prices) as f64 / 1000.0).ceil() as u32;
    let max_price = (nordpool::get_max_price(&prices) as f64 / 1000.0).ceil() as u32;

    println!("Min price: {}, Max price: {}", min_price, max_price);

    let mut data = vec![255; (WIDTH * HEIGHT).try_into().unwrap()];

    prices.iter().enumerate().for_each(|(i, price)| {
        let price_value = (price.value as f64 / 1000.0).ceil() as u32;
        draw_row(i, price.active, price_value, min_price, max_price, price.approved, &mut data);
    });

    image::save_buffer("display.png", &data, 176, 264, image::ColorType::L8).unwrap();

    convert_to_xbm();
    update_display();
}

fn convert_to_xbm() {
    let mut process = Command::new("convert")
        .arg("-rotate")
        .arg("90")
        .arg("display.png")
        .arg("display.xbm")
        .spawn()
        .ok()
        .expect("failed to convert PNG to XBM");

    match process.wait() {
        Ok(status) => println!("Process finished: {}", status),
        Err(e) => println!("Process failed, error: {}", e)
    }
}

fn update_display() {
    let mut process = Command::new("bash")
        .arg("-c")
        .arg("echo C > /tmp/epd/command")
        .spawn()
        .ok()
        .expect("failed to initiate display command");

    match process.wait() {
        Ok(status) => println!("Process finished: {}", status),
        Err(e) => println!("Process failed, error: {}", e)
    }

    let mut process = Command::new("bash")
        .arg("-c")
        .arg("xbm2bin < display.xbm > /tmp/epd/display")
        .spawn()
        .ok()
        .expect("failed to send XBM to display");

    match process.wait() {
        Ok(status) => println!("Process finished: {}", status),
        Err(e) => println!("Process failed, error: {}", e)
    }

    let mut process = Command::new("bash")
        .arg("-c")
        .arg("echo U > /tmp/epd/command")
        .spawn()
        .ok()
        .expect("failed to update display");

    match process.wait() {
        Ok(status) => println!("Process finished: {}", status),
        Err(e) => println!("Process failed, error: {}", e)
    }
}

fn map_range(from_range: (u32, u32), to_range: (u32, u32), s: u32) -> u32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn draw_row(index:usize, active:bool, price:u32, min_price:u32, max_price:u32, approved:bool, image:&mut Vec<u8>) {
    if active {
        sprites::place_sprite(&sprites::MARKER, 3, index as u32 * 11, false, image);
    }

    if approved {
        fill(0, (index as u32 * 11) + 4, 3, 3, 0, image);
    }

    let formatted_hour = format!("{:0>2}", index);

    formatted_hour.chars().enumerate().for_each(|(i, char)| {
        let char_index:usize = char.to_string().parse::<usize>().unwrap();
        sprites::place_sprite(&sprites::CHARS[char_index], (i as u32 * 6) + 4, (index as u32 * 11) + 1, active, image);
    });

	let formatted_price = format!("{:0>3}", price);
	let price_column_width = map_range((min_price, max_price), (0, 134), price);

	fill_stripes(20, (index as u32 * 11) + 1, price_column_width, 9, 0, image);

    formatted_price.chars().enumerate().for_each(|(i, char)| {
        let char_index:usize = char.to_string().parse::<usize>().unwrap();
        sprites::place_sprite(&sprites::CHARS[char_index], (i as u32 * 6) + 158, (index as u32 * 11) + 1, false, image);
    });
}

fn fill_circle(x:u32, y:u32, color:u8, image:&mut Vec<u8>) {
    fill(x + 4, y, 5, 1, color, image);
    fill(x + 2, y + 1, 9, 1, color, image);
    fill(x + 1, y + 2, 11, 1, color, image);
    fill(x + 1, y + 3, 11, 1, color, image);
    fill(x, y + 4, 13, 5, color, image);
    fill(x + 1, y + 9, 11, 1, color, image);
    fill(x + 1, y + 10, 11, 1, color, image);
    fill(x + 2, y + 11, 9, 1, color, image);
    fill(x + 4, y + 12, 5, 1, color, image);
}

fn fill(x:u32, y:u32, width:u32, height:u32, color:u8, image:&mut Vec<u8>) {
    let sprite:(usize, usize, &[u8]) = (
        width as usize,
        height as usize,
        &vec![color; (width * height).try_into().unwrap()]
    );

    sprites::place_sprite(&sprite, x, y, false, image);
}

fn fill_stripes(x:u32, y:u32, width:u32, height:u32, _color:u8, image:&mut Vec<u8>) {
    let full_width = WIDTH - x - 1;
    let mut data = vec![255; (full_width * height).try_into().unwrap()];

    for (i, pixel) in data.iter_mut().enumerate() {
        if i % 3 == 0 {
            *pixel = 0;
        }
    }

    let sprite:(usize, usize, &[u8]) = (
        full_width as usize,
        height as usize,
        &data
    );

    sprites::place_sprite(&sprite, x, y, false, image);

    fill(WIDTH - (full_width - width), y, full_width - width, height, 255, image);
}
