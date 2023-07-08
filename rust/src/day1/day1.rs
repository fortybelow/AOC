// Title: The Tyranny of the Rocket Equation
// Description:
//   Santa has become stranded at the edge of the Solar System while delivering
//   presents to other planets! To accurately calculate his position in space,
//   safely align his warp drive, and return to Earth in time to save Christmas,
//   he needs you to bring him measurements from fifty stars.
//
//   Collect stars by solving puzzles. Two puzzles will be made available on each
//   day in the Advent calendar; the second puzzle is unlocked when you complete
//   the first. Each puzzle grants one star. Good luck!
//
//   The Elves quickly load you into a spacecraft and prepare to launch.
//
//   At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper.
//   They haven't determined the amount of fuel required yet.
//
//   Fuel required to launch a given module is based on its mass. Specifically,
//   to find the fuel required for a module, take its mass, divide by three,
//   round down, and subtract 2.
//

struct Module {
    mass: i64,
}

impl Module {
    fn fuel_required(&self) -> i64 {
        self.mass / 3 - 2
    }

    fn fuel_required_recursive(&self) -> i64 {
        let mut fuel = self.fuel_required();
        let mut total_fuel = fuel;
        while fuel > 0 {
            fuel = fuel / 3 - 2;
            if fuel > 0 {
                total_fuel += fuel;
            }
        }
        total_fuel
    }
}

fn solve(file_path: &str, recursive: bool) -> i64 {
    std::fs::read_to_string(file_path).unwrap().lines().map(|line| {
        let module = Module { mass: line.parse().unwrap() };
        if recursive {
            module.fuel_required_recursive()
        } else {
            module.fuel_required()
        }
    }).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).expect("Please provide a file path");
    println!("Normal: {}", solve(file_path, false));
    println!("Extensive: {}", solve(file_path, true));

    Ok(())
}

