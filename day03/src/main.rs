use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum Direction
{
    Up = 'U' as isize,
    Down = 'D' as isize,
    Left = 'L' as isize,
    Right = 'R' as isize
}

struct Move
{
    direction : Direction,
    distance : i32,
}

impl std::str::FromStr for Move {
    type Err = std::num::ParseIntError;

    fn from_str(move_string: &str) -> Result<Self, Self::Err> 
    {
        let (direction, distance) = move_string.split_at(1);

        // Handle Direction
        let direction = direction.parse::<i32>().unwrap();
        let direction = FromPrimitive::from_i32(direction).unwrap();

        // Handle Distance
        let distance = distance.parse::<i32>().unwrap();

        Ok(Move { distance : distance, direction : direction })
    }
}



fn main() 
{
    let wires = std::fs::read_to_string("adventInput.txt")
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>();

    // wires.iter()
    //     .split(",")
    //     .map(|l| l.parse::<i32>().unwrap())
    //     .collect::<Vec<i32>>();

    
}
