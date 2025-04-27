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
        blink_once(&mut pin1, 50);
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
    //let mut pin = Gpio::new()?.get(dh_pin)?.into_output();
    println!("pin on, {} percent power.", percent);
    dh_pin.set_high();
    //println!("Blinking LED {:?} on a {}.", dh_pin, DeviceInfo::new()?.model());
    let dellay = 100000 / percent;
    println!("Delay time in milliseconds: {} ", dellay);
    delay(dellay);
    println!("pin off, {} percent power.", percent);
    dh_pin.set_low();
    delay(dellay);
}