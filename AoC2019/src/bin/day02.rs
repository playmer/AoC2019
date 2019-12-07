use aoc_helpers::int_code_computer;

fn main() 
{
    let int_codes = std::fs::read_to_string("assets/day02.txt")
        .unwrap()
        .split(",")
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // Part 1
    {
        let mut int_codes = int_codes.clone();
        // Gotta restore the gravity assist program to the "1202 program alarm" state it had just before the last computer caught fire.
        int_codes[1] = 12;
        int_codes[2] = 2;
    
        let result = int_code_computer::run(& mut int_codes, true);
    
        println!("Part 1: Result: {}", result);
        println!("Part 1: Full int codes: {:?}", int_codes);   
    }

    // Part 2
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
            
                let result = int_code_computer::run(& mut int_codes_attempt, false);

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
