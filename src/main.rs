use std::fs::{read_to_string, write};

const PATH: &str = "/sys/class/backlight/intel_backlight/brightness";

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let action = match args.as_slice() {
        [] => panic!("No args!"),
        [_, direction] if direction.as_str() == "up" => increment,
        [_, direction] if direction.as_str() == "down" => decrement,
        _ => panic!("Usage: {} up/down", args[0]),
    };

    let current_value = read_to_string(PATH)
        .expect("Error reading current value")
        .trim()
        .parse::<u16>()
        .expect("Current value was not an number");

    let new_value = action(current_value);

    write(PATH, format!("{}\n", new_value)).expect("Error writing new value");
}

fn increment(i: u16) -> u16 {
    if i >= 6500 {
        7500
    } else {
        i + 1000
    }
}

fn decrement(i: u16) -> u16 {
    if i <= 1000 {
        0
    } else {
        i - 1000
    }
}
