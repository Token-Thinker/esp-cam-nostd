#![no_std]
#![no_main]

mod utils;

use anyhow::Result;

use esp_hal::peripheral::Peripheral;
use esp_println::println;
use esp_backtrace as _;
use utils::controllers::cam::{Camera, camera};



fn main() -> Result<()> {
    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let camera = Camera::new(
        peripherals.GPIO32,
        peripherals.GPIO0,
        peripherals.GPIO5,
        peripherals.GPIO18,
        peripherals.GPIO19,
        peripherals.GPIO21,
        peripherals.GPIO36,
        peripherals.GPIO39,
        peripherals.GPIO34,
        peripherals.GPIO35,
        peripherals.GPIO25,
        peripherals.GPIO23,
        peripherals.GPIO22,
        peripherals.GPIO26,
        peripherals.GPIO27,
        camera::pixformat_t_PIXFORMAT_JPEG,
        camera::framesize_t_FRAMESIZE_UXGA,
    )
        .unwrap();

    loop {
        let framebuffer = camera.get_framebuffer();

        if let Some(framebuffer) = framebuffer {
            println!("Got framebuffer!");
            println!("width: {}", framebuffer.width());
            println!("height: {}", framebuffer.height());
            println!("len: {}", framebuffer.data().len());
            println!("format: {}", framebuffer.format());

            //std::thread::sleep(std::time::Duration::from_millis(1000));
        } else {
            panic!("no framebuffer");
        }
    }
}
