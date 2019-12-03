//use std::io;
use std::io::BufRead;
//use std::io::{BufReader};
//
fn lines_from_file(filename: impl AsRef<std::path::Path>) -> Vec<String> {
    let file = std::fs::File::open(filename).expect("no such file");
    let buf = std::io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn calculate_fuel(module : &i32) -> i32
{
    let module = *module as f32;
    let fuel = module / 3.0;
    (fuel.floor() as i32) - 2
}



fn calculate_full_fuel(mass : &i32) -> i32
{
    let mass = *mass as f32;
    let fuel_for_mass = mass / 3.0;
    let fuel_for_mass = (fuel_for_mass.floor() as i32) - 2;

    if fuel_for_mass < 1
    {
        return 0;
    }

    fuel_for_mass + calculate_full_fuel(&fuel_for_mass)
}

fn main() 
{
    //let stdin = std::io::stdin();
    //let inputs = std::io::stdin().lock().lines()
    //    .filter_map(|l| l.ok())
    //    .collect::<Vec<String>>();

    let inputs = lines_from_file("adventInput.txt");

    let modules = inputs.iter().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    // let mut fuel_sum = 0;

    // for module in &modules
    // {
    //     let fuel_mass = calculate_fuel(module);
        
    //     fuel_sum += fuel_mass;
    // }

    // dbg!(fuel_sum);

    println!("Part 1: {}", modules.iter().map(calculate_fuel).sum::<i32>());
    println!("Part 2: {}", modules.iter().map(calculate_full_fuel).sum::<i32>());
}
