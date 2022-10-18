mod ds18b20;
mod w1_errors;
use std::{thread, time};

fn main() {
    let sensors = ds18b20::DS18B20::new().unwrap();

    let sleep_delay = time::Duration::from_secs(30);
    let mut now = time::Instant::now();
    loop {
        
        for sensor in &sensors.w1_id {
            let s = String::from(sensor);
            let temp = sensors.read_temp(s).unwrap();
            println!("{} : {:.1} C", sensor, temp.to_celsius());
        }
        let read_done = time::Instant::now();
        now = now.checked_add(sleep_delay).unwrap();
        let dur = now - read_done;
        println!("Sleep {:#?} s",  dur );
        thread::sleep(dur);
    }
}