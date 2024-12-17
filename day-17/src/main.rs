fn parse_register(register: &str) -> u64 {
    let (_, value) = register.split_once(": ").unwrap();
    value.parse().unwrap()
}

fn parse_registers(registers: &str) -> [u64; 3] {
    let mut values = registers.lines().map(parse_register);
    [values.next().unwrap(), values.next().unwrap(), values.next().unwrap()]
}

fn parse_program(program: &str) -> Vec<u8> {
    let (_, value) = program.split_once(": ").unwrap();
    value.split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_input(input: &str) -> ([u64; 3], Vec<u8>) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    (parse_registers(registers), parse_program(program))
}

fn decode_combo_operand(registers: [u64; 3], op: u8) -> u64 {
    match op {
        0..=3 => op as u64,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => 0,
    }
}

fn execute_program(mut registers: [u64; 3], program: &[u8]) -> Vec<u8> {
    let mut ip = 0;
    let mut output = Vec::new();
    while ip as usize + 1 < program.len() {
        let lit = program[ip as usize + 1] as u64;
        let com = decode_combo_operand(registers, program[ip as usize + 1]);
        match program[ip as usize] {
            0 => registers[0] >>= com,
            1 => registers[1] ^= lit,
            2 => registers[1] = com & 0b111,
            3 => ip = if registers[0] != 0 { lit as i32 - 2 } else { ip },
            4 => registers[1] ^= registers[2],
            5 => output.push(com as u8 & 0b111),
            6 => registers[1] = registers[0] >> com,
            7 => registers[2] = registers[0] >> com,
            _ => panic!("invalid opcode"),
        }
        ip += 2;
    }

    output
}

fn solve_for_a(program: &[u8], a: u64, index: usize) -> Option<u64> {
    for chunk in 0..8 {
        let new_a = a | chunk << (index * 3);
        let output = execute_program([new_a, 0, 0], program);
        if output.get(index) == program.get(index) {
            if index == 0 {
                return Some(new_a);
            }
            if let Some(solution) = solve_for_a(program, new_a, index - 1) {
                return Some(solution);
            }
        }
    }
    None
}

fn part_1(input: String) -> String {
    let (registers, program) = parse_input(&input);
    let output = execute_program(registers, &program);
    output.iter().map(|n| format!("{}", n)).collect::<Vec<_>>().join(",")
}

fn part_2(input: String) -> u64 {
    let (_, program) = parse_input(&input);
    solve_for_a(&program, 0, program.len() - 1).unwrap()
}

aoc::main!();
