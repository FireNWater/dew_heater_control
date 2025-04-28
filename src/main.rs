use std::error::Error;
use std::{thread, u8};
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
//use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
    
    const DH_PORT_1: u8 = 12;
    //const DH_PORT_2: u8 = 13;

    let mut pin1 = Gpio::new()?.get(DH_PORT_1)?.into_output();
    // let mut pin2 = Gpio::new()?.get(DC_PORT_2)?.into_output();
    
    let mut count = 0;
    loop {
        blink_once(&mut pin1, 60);
        count += 1;
        if count > 5 {
             break }
    }


    Ok(())          //Return value to OS
}

fn delay(millisec: u64) {
    thread::sleep(Duration::from_millis(millisec));
}

fn blink_once (dh_pin: &mut OutputPin, percent: u64){
    let pause = percent * 10;                           //Convert % to millis

    dh_pin.set_high();

        println!("On time in milliseconds: {} ", pause);
    delay(pause);

    dh_pin.set_low();
        println!("Off time in milliseconds: {} ", 1000 - pause);

    delay(1000 - pause);
}