// Note:
//  This does not work on a desktop.  This might only report computer battery status vs status of
//  all battery related devices.
//
//  This "sorta" works on the H700 devices for Knulli.  It finds the battery but is unable to gather
//  battery data stats other than Charging / Discharging?
fn main() -> Result<(), battery::Error> {
    println!("Rust-Battery-Example");

    let manager = battery::Manager::new()?;

    for (idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        println!("Battery #{}:", idx);
        println!("Vendor: {:?}", battery.vendor());
        println!("Model: {:?}", battery.model());
        println!("State: {:?}", battery.state());
        println!("Time to full charge: {:?}", battery.time_to_full());
        println!("");
    }

    Ok(())
}
