use std::error::Error;
use std::thread;
use std::time::Duration;

use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};

use display_interface_spi::SPIInterface;
use rppal::gpio::Gpio;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use ssd1306::{prelude::*, Builder};

const GPIO_RS: u8 = 4;
const GPIO_DC: u8 = 5;
const GPIO_CS: u8 = 6;

fn oled() -> Result<(), rppal::gpio::Error> {
    let dc = Gpio::new()?.get(GPIO_DC)?.into_output();
    let cs = Gpio::new()?.get(GPIO_CS)?.into_output();
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0).unwrap();

    let interface = SPIInterface::new(spi, dc, cs);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
    disp.init().unwrap();

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64, 64);

    let im = Image::new(&raw, Point::new(32, 0));

    im.draw(&mut disp).unwrap();

    disp.flush().unwrap();

    Ok(())
}

fn main() -> ! {
    oled().unwrap();
    loop {}
}
