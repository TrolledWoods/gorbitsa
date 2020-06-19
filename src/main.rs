mod commands;
mod parser;

fn main() {
	let mut program = [0; 256];
	let parsed_program = parser::parse_assembly("
:start
input
output_x
branch :end
const 0
branch :start
:end
").unwrap();
	for (i, thing) in parsed_program.into_iter().enumerate() {
		program[i] = thing;
	}

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
