mod commands;
mod parser;

// const -11
// mem_set_x :ctr 1
// :loop
// mem :ctr 1
// incr 1
// mem_set_x :ctr 1
// branch :end
// incr '9 1
// :ctr
// output_x
// const 0
// branch :loop
// :end

fn main() {
	let mut program = [0; 256];
	let parsed_program = parser::parse_assembly(include_str!("src.gbt_asm")).unwrap();
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

	println!("{}", commands.get_gorbitsa());

	use commands::VM;
	while let Some((instr, arg)) = commands.next() {
		if instr == 0 { break; }
		commands::run_command(&mut commands, instr, arg);
		// for (i, mem) in commands.ram.iter().enumerate() { 
		// 	if commands.instr_pointer as usize == i { print!("->"); }
		// 	print!("{} ", mem); 
		// }
		// println!();
		// println!("instr: {} ({}), x: {} ({}), arg: {}", commands.instr_pointer, instr as char, commands.x_register, commands.mem(commands.x()), arg);
		// println!("{}", commands::instr_to_desc(instr));
	}

	println!("{}", String::from_utf8(commands.output).unwrap());
}
