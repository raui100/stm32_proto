#![no_std]
#![no_main]
use core::fmt::Write;

use defmt::{debug, error};
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_time::Delay;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use mpu6050::Mpu6050;
use sh1106::mode::GraphicsMode;
use sh1106::Builder;

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    // Prepare I2C Bus
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
    let bus = shared_bus::BusManagerSimple::new(i2c);

    // Init IMU
    let mut mpu = Mpu6050::new(bus.acquire_i2c());
    let mut delay = Delay;
    mpu.init(&mut delay).unwrap();

    // Init display
    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(bus.acquire_i2c()).into();
    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let mut text = heapless::String::<64>::new();
    disp.init().unwrap();
    disp.clear();
    disp.flush().unwrap();

    loop {
        // Reading data
        let gyro = mpu.get_acc().unwrap();
        debug!("{}\n{}\n{}", gyro[0], gyro[1], gyro[2]);

        // Writing sum to display
        text.clear();
        disp.clear();
        write!(&mut text, "{}\n{}\n{}", gyro[0], gyro[1], gyro[2]).unwrap();
        Text::new(&text, Point::new(10, 10), style)
            .draw(&mut disp)
            .unwrap();
        if disp.flush().is_err() {
            error!("Failed writing to display");
        }
    }
}
