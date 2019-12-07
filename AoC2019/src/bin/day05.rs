use aoc_helpers::int_code_computer;

fn main() 
{
    let int_codes = std::fs::read_to_string("assets/day05.txt")
        .unwrap()
        .split(",")
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // Part 1
    {
        let mut int_codes = int_codes.clone();
    
        let result = int_code_computer::run(& mut int_codes, false);
    
        println!("Part 1: Result: {}", result);
        //println!("Part 1: Full int codes: {:?}", int_codes);
    }
}
