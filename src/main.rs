use std::error::Error;
use std::{thread, time};

// use std::time::Duration;

use embedded_graphics::{
    fonts::{Font6x12, Font6x8, Text},
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
};

use display_interface_spi::SPIInterface;
use rppal::gpio::{Gpio, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use ssd1306::{prelude::*, Builder};

const GPIO_RS: u8 = 4;
const GPIO_DC: u8 = 5;
const GPIO_CS: u8 = 6;
// let mut disp: GraphicsMode<_>;

// fn get_display() -> Result<
//     GraphicsMode<display_interface_spi::SPIInterface<rppal::spi::Spi, OutputPin, OutputPin>>,
//     rppal::gpio::Error,
// > {
//     let dc = Gpio::new()?.get(GPIO_DC)?.into_output();
//     let cs = Gpio::new()?.get(GPIO_CS)?.into_output();
//     let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0).unwrap();

//     let interface = SPIInterface::new(spi, dc, cs);
//     let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
//     disp.init().unwrap();
//     Ok(disp)
// }

fn draw_rust(
    disp: &mut GraphicsMode<
        display_interface_spi::SPIInterface<rppal::spi::Spi, OutputPin, OutputPin>,
    >,
    pos_x: i32,
) -> Result<(), rppal::gpio::Error> {
    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64, 64);

    let im = Image::new(&raw, Point::new(pos_x, 0));

    im.draw(disp).unwrap();

    Ok(())
}

fn draw_text(
    disp: &mut GraphicsMode<
        display_interface_spi::SPIInterface<rppal::spi::Spi, OutputPin, OutputPin>,
    >,
) {
    let text_style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .build();

    Text::new("Eyes Everywhere - Tochal", Point::zero())
        .into_styled(text_style)
        .draw(disp)
        .unwrap();
    Text::new("(Original mix)", Point::new(0, 10))
        .into_styled(text_style)
        .draw(disp)
        .unwrap();
}

fn oled_init() -> Result<(), rppal::gpio::Error> {
    let mut pos_x: i32 = 32;
    let mut dir: i32 = -1;
    let dc = Gpio::new()?.get(GPIO_DC)?.into_output();
    let cs = Gpio::new()?.get(GPIO_CS)?.into_output();
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode3).unwrap();

    let interface = SPIInterface::new(spi, dc, cs);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
    disp.init().unwrap();
    loop {
        // let mut disp: GraphicsMode<_> = get_display().unwrap();
        if pos_x > 63 {
            dir = -1;
        } else if pos_x < 1 {
            dir = 1;
        }
        pos_x = pos_x + dir;
        disp.clear();
        // draw_text(&mut disp);
        disp.flush().unwrap();
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }
}

fn main() {
    oled_init().unwrap();
}
