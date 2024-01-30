#![no_std]
#![no_main]

use defmt::error;
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use sh1106::mode::GraphicsMode;
use sh1106::Builder;

// use stm32f4xx_hal as _;

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    let i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        NoDma,
        NoDma,
        Hertz(100_000),
        Default::default(),
    );

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    disp.init().unwrap();
    disp.flush().unwrap();

    let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let mut forward = true;
    let mut x = 0;
    loop {
        disp.clear();
        Text::new("Hello world!", Point::new(x, 30), style)
            .draw(&mut disp)
            .unwrap();
        if disp.flush().is_err() {
            error!("Failed writing to display");
        }

        if forward {
            x += 1;
            if x > 10 {
                forward = false;
            }
        } else {
            x -= 1;
            if x == 0 {
                forward = true;
            }
        }
    }
}
