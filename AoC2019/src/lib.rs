pub mod int_code_computer 
{
    use num_derive::FromPrimitive;    
    use num_traits::FromPrimitive;

    
    #[derive(Clone, Copy, FromPrimitive)]
    enum Opcode {
        Addition = 1,
        Multiplication = 2,
        Input = 3,
        Output = 4,
        End = 99
    }

    #[derive(Clone, Copy, PartialEq)]
    enum ParameterMode {
        Position = 0,
        Immediate = 1
    }
    
    fn addition(instruction_pointer : usize, int_codes :  & mut Vec<i32>, parameter_modes : Vec<ParameterMode>, asm_dump :  & mut Vec<String>) -> usize
    {
        assert!(parameter_modes[2] == ParameterMode::Position, "Write arguement should never be immediate");
        let arg_1 = if parameter_modes[0] == ParameterMode::Position 
        {
            int_codes[int_codes[instruction_pointer + 1] as usize]
        }
        else
        {
            int_codes[instruction_pointer + 1]
        };

        let arg_2 = if parameter_modes[1] == ParameterMode::Position 
        {
            int_codes[int_codes[instruction_pointer + 2] as usize]
        }
        else
        {
            int_codes[instruction_pointer + 2]
        };

        let result_address = int_codes[instruction_pointer + 3] as usize;
        
        int_codes[result_address] = arg_1 + arg_2;
    
        asm_dump.push(format!("add: {}, {}, {}", arg_1, arg_2, result_address));
        return instruction_pointer + 4;
    }
    
    
    fn multiplication(instruction_pointer : usize, int_codes :  & mut Vec<i32>, parameter_modes : Vec<ParameterMode>, asm_dump :  & mut Vec<String>) -> usize
    {
        assert!(parameter_modes[2] == ParameterMode::Position, "Write arguement should never be immediate");
        let arg_1 = if parameter_modes[0] == ParameterMode::Position 
        {
            int_codes[int_codes[instruction_pointer + 1] as usize]
        }
        else
        {
            int_codes[instruction_pointer + 1]
        };

        let arg_2 = if parameter_modes[1] == ParameterMode::Position 
        {
            int_codes[int_codes[instruction_pointer + 2] as usize]
        }
        else
        {
            int_codes[instruction_pointer + 2]
        };

        let result_address = int_codes[instruction_pointer + 3] as usize;
        
        int_codes[result_address] = arg_1 * arg_2;
    
        asm_dump.push(format!("multiplication: {}, {}, {}", arg_1, arg_2, result_address));
        return instruction_pointer + 4;
    }
    
    fn input(instruction_pointer : usize, int_codes :  & mut Vec<i32>, parameter_modes : Vec<ParameterMode>, asm_dump :  & mut Vec<String>) -> usize
    {
        use std::io::{Read};

        assert!(parameter_modes[0] == ParameterMode::Position, "Write arguement should never be immediate");
        let result_address = int_codes[instruction_pointer + 1] as usize;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Could not read i32 from stdin!");

        let input = buffer.trim().parse::<i32>().unwrap();
    
        int_codes[result_address] = input;
    
        asm_dump.push(format!("input: {}", result_address));
        return instruction_pointer + 2;
    }
    
    fn output(instruction_pointer : usize, int_codes :  & mut Vec<i32>, parameter_modes : Vec<ParameterMode>, asm_dump :  & mut Vec<String>) -> usize
    {
        let arg_1 = if parameter_modes[0] == ParameterMode::Position 
        {
            int_codes[int_codes[instruction_pointer + 1] as usize]
        }
        else
        {
            int_codes[instruction_pointer + 1]
        };

        println!("{}", arg_1);
    
        asm_dump.push(format!("output: {}", arg_1));
        return instruction_pointer + 2;
    }

    fn get_default_parameter_modes(opcode : Opcode) -> Vec<ParameterMode>
    {
        match opcode
        {
            Opcode::Addition => 
            {
                return vec![ParameterMode::Position; 3];
            }
            Opcode::Multiplication => 
            {
                return vec![ParameterMode::Position; 3];
            }
            Opcode::Input => 
            {
                return vec![ParameterMode::Position; 1];
            }
            Opcode::Output => 
            {
                return vec![ParameterMode::Position; 1];
            }
            Opcode::End => 
            {
                return Vec::new();
            }
        }
    }

    fn get_opcode_and_parameter_modes(unparsed_opcode_given : i32) -> (Opcode, Vec<ParameterMode>)
    {
        let unparsed_opcode: String = unparsed_opcode_given.to_string();

        if unparsed_opcode.len() > 2
        {
            let (parameter_modes, opcode) = unparsed_opcode.split_at(unparsed_opcode.len() - 2);

            let opcode : Opcode = FromPrimitive::from_i32(opcode.parse::<i32>().unwrap()).unwrap();

            let mut default_parameter_modes = get_default_parameter_modes(opcode);

            for (i, parameter_mode) in parameter_modes.chars().rev().enumerate()
            {
                match parameter_mode
                {
                    '0' => default_parameter_modes[i] = ParameterMode::Position,
                    '1' => default_parameter_modes[i] = ParameterMode::Immediate,
                    _ => panic!("Got a bad parameter mode! It was '{}'", parameter_mode)
                }

            }

            return (opcode, default_parameter_modes);
        }

        let opcode = FromPrimitive::from_i32(unparsed_opcode_given).unwrap();

        return (opcode, get_default_parameter_modes(opcode));
    }

    pub fn run(int_codes :  & mut Vec<i32>, should_print_asm : bool) -> i32
    {
        let mut instruction_pointer = 0;

        let mut asm_dump = Vec::<String>::new();

        loop
        {
            let (opcode, parameter_modes) = get_opcode_and_parameter_modes(int_codes[instruction_pointer]);

            match opcode
            {
                Opcode::Addition => 
                {
                    instruction_pointer = addition(instruction_pointer, int_codes, parameter_modes, & mut asm_dump);
                }
                Opcode::Multiplication => 
                {
                    instruction_pointer = multiplication(instruction_pointer, int_codes, parameter_modes, & mut asm_dump);
                }
                Opcode::Input => 
                {
                    instruction_pointer = input(instruction_pointer, int_codes, parameter_modes, & mut asm_dump);
                }
                Opcode::Output => 
                {
                    instruction_pointer = output(instruction_pointer, int_codes, parameter_modes, & mut asm_dump);
                }
                Opcode::End => 
                {
                    asm_dump.push("return".to_string());
                    break;
                }
            }
        }

        if should_print_asm
        {
            println!("asm_dump:\n {}", asm_dump.join("\n"));
        }

        return int_codes[0];
    }
}