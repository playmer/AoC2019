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
        JumpIfTrue = 5,
        JumpIfFalse = 6,
        LessThan = 7,
        Equals = 8,
        End = 99
    }

    #[derive(Clone, Copy, PartialEq)]
    enum ParameterMode {
        Position = 0,
        Immediate = 1
    }
    
    fn get_args(instruction_pointer : usize, int_codes : & Vec<i32>, parameter_modes : & Vec<ParameterMode>, arguments : & mut Vec<i32>)
    {
        arguments.clear();

        for (i, parameter_mode) in parameter_modes.iter().enumerate()
        {
            match parameter_mode
            {
                ParameterMode::Position => arguments.push(int_codes[int_codes[instruction_pointer + i + 1] as usize]),
                ParameterMode::Immediate => arguments.push(int_codes[instruction_pointer + i + 1])
            }
        }
    }
    
    fn addition(instruction_pointer : usize, int_codes :  & mut Vec<i32>, arguments : & Vec<i32>) -> usize
    {
        int_codes[arguments[2] as usize] = arguments[0] + arguments[1];
    
        return instruction_pointer + 4;
    }
    
    
    fn multiplication(instruction_pointer : usize, int_codes :  & mut Vec<i32>, arguments : & Vec<i32>) -> usize
    {
        int_codes[arguments[2] as usize] = arguments[0] * arguments[1];
    
        return instruction_pointer + 4;
    }
    
    fn input(instruction_pointer : usize, int_codes :  & mut Vec<i32>, arguments : & Vec<i32>) -> usize
    {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Could not read i32 from stdin!");

        let input = buffer.trim().parse::<i32>().unwrap();
    
        int_codes[arguments[0] as usize] = input;
    
        return instruction_pointer + 2;
    }
    
    fn output(instruction_pointer : usize, arguments : & Vec<i32>) -> usize
    {
        println!("{}", arguments[0]);
    
        return instruction_pointer + 2;
    }

    fn jump_if_true(instruction_pointer : usize, arguments : & Vec<i32>) -> usize
    {
        if arguments[0] != 0
        {
            return arguments[1] as usize;
        }
    
        return instruction_pointer + 2;
    }

    fn jump_if_false(instruction_pointer : usize, arguments : & Vec<i32>) -> usize
    {
        if arguments[0] == 0
        {
            return arguments[1] as usize;
        }
    
        return instruction_pointer + 2;
    }
    
    fn less_than(instruction_pointer : usize, int_codes :  & mut Vec<i32>, arguments : & Vec<i32>) -> usize
    {
        int_codes[arguments[2] as usize] = if arguments[0] < arguments[1] { 1 } else { 0 };
    
        return instruction_pointer + 4;
    }
    
    fn equals(instruction_pointer : usize, int_codes :  & mut Vec<i32>, arguments : & Vec<i32>) -> usize
    {
        let result_address = int_codes[instruction_pointer + 3] as usize;
        
        int_codes[result_address] = if arguments[0] == arguments[1] { 1 } else { 0 };
    
        return instruction_pointer + 4;
    }

    fn fill(number : usize, parameter_modes : & mut Vec<ParameterMode>)
    {
        for _x in 0..number
        {
            parameter_modes.push(ParameterMode::Position);
        }
    }

    fn get_default_parameter_modes(opcode : Opcode, parameter_modes : & mut Vec<ParameterMode>)
    {
        parameter_modes.clear();

        match opcode
        {
            Opcode::Addition => fill(3, parameter_modes),
            Opcode::Multiplication => fill(3, parameter_modes),
            Opcode::Input => fill(1, parameter_modes),
            Opcode::Output => fill(1, parameter_modes),
            Opcode::JumpIfTrue => fill(2, parameter_modes),
            Opcode::JumpIfFalse => fill(2, parameter_modes),
            Opcode::LessThan => fill(3, parameter_modes),
            Opcode::Equals => fill(3, parameter_modes),
            Opcode::End => return
        }
    }
    
    
    fn opcode_parameter_mode_check(opcode : &Opcode, parameter_modes : & mut Vec<ParameterMode>)
    {
        match opcode
        {
            Opcode::Addition |  Opcode::Multiplication | Opcode::LessThan |  Opcode::Equals =>
            {
                assert!(parameter_modes[2] == ParameterMode::Position, "Write argument should never be immediate");
                parameter_modes[2] = ParameterMode::Immediate;
                return;
            },
            Opcode::Input =>
            {
                assert!(parameter_modes[0] == ParameterMode::Position, "Write argument should never be immediate");
                parameter_modes[0] = ParameterMode::Immediate;
                return;
            },
            Opcode::Output | Opcode::JumpIfTrue  | Opcode::JumpIfFalse | Opcode::End =>
            {
                return;
            }
        }
    }

    fn get_opcode_and_parameter_modes(unparsed_opcode_given : i32, parameter_modes : & mut Vec<ParameterMode>) -> Opcode
    {
        let unparsed_opcode: String = unparsed_opcode_given.to_string();

        if unparsed_opcode.len() > 2
        {
            let (parameter_modes_str, opcode) = unparsed_opcode.split_at(unparsed_opcode.len() - 2);

            let opcode : Opcode = FromPrimitive::from_i32(opcode.parse::<i32>().unwrap()).unwrap();

            get_default_parameter_modes(opcode, parameter_modes);

            for (i, parameter_mode) in parameter_modes_str.chars().rev().enumerate()
            {
                match parameter_mode
                {
                    '0' => parameter_modes[i] = ParameterMode::Position,
                    '1' => parameter_modes[i] = ParameterMode::Immediate,
                    _ => panic!("Got a bad parameter mode! It was '{}'", parameter_mode)
                }
            }

            opcode_parameter_mode_check(&opcode, parameter_modes);
            return opcode;
        }

        let opcode = FromPrimitive::from_i32(unparsed_opcode_given).unwrap();
        get_default_parameter_modes(opcode, parameter_modes);
        opcode_parameter_mode_check(&opcode, parameter_modes);

        return opcode;
    }

    fn opcode_to_string(opcode : &Opcode) -> &'static str
    {
        match opcode
        {
            Opcode::Addition => return "Opcode::Addition",
            Opcode::Multiplication => return "Opcode::Multiplication",
            Opcode::Input => return "Opcode::Input",
            Opcode::Output => return "Opcode::Output",
            Opcode::JumpIfTrue => return "Opcode::JumpIfTrue",
            Opcode::JumpIfFalse => return "Opcode::JumpIfFalse",
            Opcode::LessThan => return "Opcode::LessThan",
            Opcode::Equals => return "Opcode::Equals",
            Opcode::End => return "Opcode::End"
        }
    }

    pub fn run(int_codes :  & mut Vec<i32>, should_print_asm : bool) -> i32
    {
        let mut instruction_pointer = 0;

        let mut asm_dump = Vec::<String>::new();
        let mut arguments = Vec::<i32>::new();
        let mut parameter_modes = Vec::<ParameterMode>::new();

        loop
        {
            let opcode = get_opcode_and_parameter_modes(int_codes[instruction_pointer], & mut parameter_modes);

            get_args(instruction_pointer, int_codes, &parameter_modes, & mut arguments);
            
            if should_print_asm
            {
                asm_dump.push(format!("{}: {:?}", opcode_to_string(&opcode), arguments));
            }

            match opcode
            {
                Opcode::Addition => 
                {
                    instruction_pointer = addition(instruction_pointer, int_codes, &arguments);
                }
                Opcode::Multiplication => 
                {
                    instruction_pointer = multiplication(instruction_pointer, int_codes, &arguments);
                }
                Opcode::Input => 
                {
                    instruction_pointer = input(instruction_pointer, int_codes, &arguments);
                }
                Opcode::Output => 
                {
                    instruction_pointer = output(instruction_pointer, &arguments);
                }
                Opcode::JumpIfTrue => 
                {
                    instruction_pointer = jump_if_true(instruction_pointer, &arguments);
                }
                Opcode::JumpIfFalse => 
                {
                    instruction_pointer = jump_if_false(instruction_pointer, &arguments);
                }
                Opcode::LessThan => 
                {
                    instruction_pointer = less_than(instruction_pointer, int_codes, &arguments);
                }
                Opcode::Equals => 
                {
                    instruction_pointer = equals(instruction_pointer, int_codes, &arguments);
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