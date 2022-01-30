use scrap::dxgi::*;
use device_query::{DeviceQuery, DeviceState, Keycode};
extern crate scrap;

fn init_capt() -> (Capturer, i32) {
    let dis = scrap::dxgi::Displays::new().unwrap().next().unwrap();

    (Capturer::new(&dis).unwrap(), dis.width())
}

fn get_pixel(state: &device_query::DeviceState, cap: &mut Capturer, width: i32) {
    let (x, y) = state.get_mouse().coords;
    let pos = ((x + y*width)*4) as usize;

    let frame = cap.frame(10).unwrap();

    let b = frame[pos];
    let g = frame[pos+1];
    let r = frame[pos+2];
    let a = frame[pos+3];

    println!();
    println!("x: {x}, y: {y}");
    println!("r: {r}, g: {g}, b: {b}, a: {a}");
    println!("hex: #{:02x}{:02x}{:02x}", r, g, b)
}

fn main() {
    let (mut screen, width) = init_capt();
    let state = DeviceState::new();

    let mut pressed = false;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
        let keys: Vec<Keycode> = state.get_keys();
        if keys.contains(&Keycode::T) {
            if !pressed {
                get_pixel(&state, &mut screen, width);
            }
            pressed = true;
        }
        else {
            pressed = false;
        }
    }
}