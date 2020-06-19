mod commands;
mod lexer;

fn main() {
	let mut program = [0; 256];
	program[0] = 'R' as u8;
	program[1] = 0;
	program[2] = 'T' as u8;
	program[3] = 0;
	program[4] = 'B' as u8;
	program[5] = 10;
	program[6] = 'S' as u8;
	program[7] = 0;
	program[8] = 'B' as u8;
	program[9] = 0;
	let input = b"Hello world!";
	let mut commands = commands::RamVm {
		ram: program,
		instr_pointer: 0,
		x_register: 0,
		input: input,
		output: Vec::new(),
	};

	use commands::VM;
	while let Some((instr, arg)) = commands.next() {
		println!("instr: {}, arg: {}", instr as char, arg);
		if instr == 0 { break; }
		commands::run_command(&mut commands, instr, arg);
	}

	println!("{}", String::from_utf8(commands.output).unwrap());
}
