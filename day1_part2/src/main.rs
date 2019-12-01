use std::io::prelude::*;
use std::io::{self};
// fuel req = floor(mass / 3) - 2

type Mass = u32;
type Fuel = u32;

/// Calculate the amount of fuel needed for the mass of the module
fn calculate_fuel_rec(module_mass: Mass) -> Fuel {
    if module_mass == 0 {
        return 0;
    }
    let subtotal = ((module_mass as f64 / 3f64).floor() as u32)
        .checked_sub(2)
        .unwrap_or(0);
    subtotal + calculate_fuel_rec(subtotal)
}

fn calculate_fuel(module_mass: Mass) -> Fuel {
    calculate_fuel_rec(module_mass)
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut sum = 0;
    loop {
        if handle.read_line(&mut buffer)? > 0 {
            let module_mass = buffer.trim().parse::<Mass>().unwrap();
            sum += calculate_fuel(module_mass);
        } else {
            break;
        }
        buffer.clear();
    }
    println!("Total fuel needed: {}", sum);

    Ok(())
}
