use std::fs;

fn module_fuel() -> i32 {
    let mut input = fs::read_to_string("resources/day1.input").unwrap();
    input.pop();
    let module_masses = input
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut fuel_required = 0;
    for mass in module_masses {
        let module_fuel = mass / 3 - 2;
        fuel_required = fuel_required + accrue_fuel(module_fuel);
    }
    fuel_required
}

fn accrue_fuel(mut unaccounted_fuel: i32) -> i32 {
    let mut fuel_required = 0;
    while unaccounted_fuel > 0 {
        fuel_required = fuel_required + unaccounted_fuel;
        unaccounted_fuel = unaccounted_fuel / 3 - 2;
    }
    fuel_required
}

fn main() {
    let required_fuel = module_fuel();
    println!("Total fuel required: {}", required_fuel);
}
