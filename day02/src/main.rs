use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
enum Opcode {
    Addition = 1,
    Multiplication = 2,
    End = 99
}

fn addition(index : usize, int_codes :  & mut Vec<i32>)
{
    let left_address = int_codes[index + 1] as usize;
    let right_address = int_codes[index + 2] as usize;
    let result_address = int_codes[index + 3] as usize;
    
    int_codes[result_address] = int_codes[left_address] + int_codes[right_address];
}


fn multiplication(index : usize, int_codes :  & mut Vec<i32>)
{
    let left_address = int_codes[index + 1] as usize;
    let right_address = int_codes[index + 2] as usize;
    let result_address = int_codes[index + 3] as usize;

    int_codes[result_address] = int_codes[left_address] * int_codes[right_address];
}

fn run_int_code_computer(int_codes :  & mut Vec<i32>) -> i32
{
    let mut index = 0;

    loop
    {
        let opcode = FromPrimitive::from_i32(int_codes[index]);
        match opcode
        {
            Some(Opcode::Addition) => 
            {
                addition(index, int_codes);
                index += 4;
            }
            Some(Opcode::Multiplication) => 
            {
                multiplication(index, int_codes);
                index += 4;
            }
            Some(Opcode::End) => break,
            None => panic!("Unknown error!")
        }
    }

    return int_codes[0];
}

fn main() 
{
    let int_codes = std::fs::read_to_string("adventInput.txt")
        .unwrap()
        .split(",")
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    {
        let mut int_codes = int_codes.clone();
        // Gotta restore the gravity assist program to the "1202 program alarm" state it had just before the last computer caught fire.
        int_codes[1] = 12;
        int_codes[2] = 2;
    
        let result = run_int_code_computer(& mut int_codes);
    
        println!("Part 1: Result: {}", result);
        println!("Part 1: Full int codes: {:?}", int_codes);   
    }

    
    {
        let mut int_codes_final = int_codes.clone();
        let mut final_noun = 0;
        let mut final_verb = 0;

        'outer: for noun in 0 .. 100
        {
            for verb in 0 .. 100
            {
                let mut int_codes_attempt = int_codes.clone();

                int_codes_attempt[1] = noun;
                int_codes_attempt[2] = verb;
            
                let result = run_int_code_computer(& mut int_codes_attempt);

                if result == 19690720
                {
                    int_codes_final = int_codes_attempt.clone();
                    final_noun = noun;
                    final_verb = verb;
                    break 'outer;
                }
            }
        }

        let result = int_codes_final[0];
        

        if result != 19690720
        {
            panic!("Got the wrong result!");
        }

        println!("Part 2: Result: {}", 100 * final_noun + final_verb);
        println!("Part 2: Noun: {}", final_noun);
        println!("Part 2: Verb: {}", final_verb);
        println!("Part 2: Full int codes: {:?}", int_codes_final);   
    }
}
