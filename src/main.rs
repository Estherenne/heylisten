use battery::{
    Manager,
    units::ratio::percent
};

use std::io;
use std::thread;
use std::time::Duration;

fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    loop {
        if battery.state_of_charge().get::<percent>() < 50.0 {
            println!("battery has reached low capacity");
        }
        println!("{:?}", battery.state_of_charge().get::<percent>());
        thread::sleep(Duration::from_secs(1));
        manager.refresh(&mut battery)?;
    }
}
