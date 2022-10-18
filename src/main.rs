mod ds18b20;
mod w1_errors;
//use std::env;

fn main() {
    let sensors = ds18b20::DS18B20::new().unwrap();
    for sensor in &sensors.w1_id {
        let s = String::from(sensor);
        let temp = sensors.read_temp(s).unwrap();
        println!("{} : {:.1} C", sensor, temp.to_celsius());
    }

    // Default print friendly, but allow "-r/--raw" to
    // be passed to display the millicelsius directly instead
    /* 
    let args: Vec<String> = env::args().collect();
    if (args.len() > 1) && args[1].contains("-r") {
        println!("{}", temp.as_u32());
    }
    else {
        
    }
    */
}