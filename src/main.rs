use battery::{
    Manager,
    units::ratio::percent
};

use notify_rust::{
    Notification,
    Hint,
    Urgency::Critical
};

use std::{
    io,
    thread,
    time::Duration
};

fn main() -> battery::Result<()> {
    let manager = Manager::new()?;
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
        
    let battery_model_text = format!("Battery {}:", battery.model().unwrap());
    let mut sent_notification_low = false;
    let mut sent_notification_critical = false;
    let mut sent_notification_full = false;

    loop {
        if !sent_notification_low && battery.state_of_charge().get::<percent>() <= 20.0 {
            let _ = Notification::new()
                .summary(&battery_model_text)
                .body("Power level is low")
                .show();
            sent_notification_low = true;
        } else if sent_notification_low && battery.state_of_charge().get::<percent>() >= 21.0 {
            sent_notification_low = false;
        }

        if !sent_notification_critical && battery.state_of_charge().get::<percent>() <= 10.0 {
            let _ = Notification::new()
                .summary(&battery_model_text) 
                .body("Power level is critical")
                .hint(Hint::Urgency(Critical))
                .show(); 
            sent_notification_critical = true;
        } else if sent_notification_critical && battery.state_of_charge().get::<percent>() >= 11.0 {
            sent_notification_critical = false;
        }

        if !sent_notification_full && battery.state_of_charge().get::<percent>() == 100.0 {
            let _ = Notification::new()
                .summary(&battery_model_text)
                .body("Battery is full")
                .show();
            sent_notification_full = true;
        } else if sent_notification_full && battery.state_of_charge().get::<percent>() <= 99.0 {
            sent_notification_full = false;
        }

        thread::sleep(Duration::from_secs(1));
        manager.refresh(&mut battery)?;
    }
}
