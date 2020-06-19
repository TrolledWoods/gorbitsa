/// Defines the commands to use. 
macro_rules! commands {
	($vm:ident, $arg:ident => $($name:expr; ($has_arg:expr) $run:expr),*,) => {
		pub fn run_command($vm: &mut impl VM, cmd: char, $arg: u8) {
			match cmd {
				$(
					$name => $run,
				)*
				_ => panic!("Invalid command {}", cmd),
			}
		}

		pub fn command_has_arg(cmd: char) -> bool {
			match cmd {
				$(
					$name => $has_arg,
				)*
				_ => panic!("Invalid command {}", cmd),
			}
		}
	}
}

commands!(vm, arg =>
	'G'; (true)  vm.set_x(vm.mem(arg)),
	'O'; (true)  vm.set_mem(arg, vm.x()),
	'R'; (false) { 
		let input = vm.read_input();
		vm.set_x(input);
	}, 
	'B'; (true)  if vm.x() == 0 { vm.jump(arg); },
	'I'; (true)  vm.set_x(vm.x().wrapping_add(arg)),
	'T'; (false) vm.write_output(vm.x()),
	'S'; (true)  vm.set_x(arg),
	'A'; (true)  vm.set_x(vm.x() + vm.mem(arg)),

	'g'; (true)  vm.set_x(vm.mem(vm.mem(arg))),
	'o'; (true)  vm.set_mem(vm.mem(arg), vm.x()),
	'r'; (true)  { 
		let input = vm.read_input();
		vm.set_mem(vm.x(), input);
	},
	'b'; (true)  if vm.x() == 0 { vm.jump(vm.mem(arg)); },
	'i'; (true)  vm.set_mem(arg, vm.mem(arg).wrapping_add(vm.x())),
	't'; (true)  vm.write_output(vm.mem(arg)),
	's'; (true)  vm.set_x(vm.mem(arg) ^ vm.x()),
	'a'; (true)  vm.set_x(vm.mem(vm.mem(arg)).wrapping_add(vm.x())),
);

pub trait VM {
	fn mem(&self, pos: u8) -> u8;
	fn set_mem(&self, pos: u8, value: u8);
	fn x(&self) -> u8;
	fn set_x(&mut self, value: u8);

	fn read_input(&mut self) -> u8;
	fn write_output(&mut self, output: u8);

	/// Returns the next instruction and argument, and increments the
	/// instruction pointer.
	fn next(&mut self) -> Option<(char, u8)>;
	fn jump(&mut self, instr: u8);
}

pub struct RamVm {
	ram: [256; u8],
	instr_pointer: u8,
	x_register: u8,
	input: Vec<u8>,
	output: Vec<u8>,
}

impl VM {
	fn mem(&self, pos: u8) -> u8 { self.ram[pos as usize] }
	fn set_mem(&mut self, pos: u8, value) { self.ram[pos as usize] = value; }
	fn x(&self) -> u8 { self.x_register }
	fn set_x(&mut self, value: u8) -> { self.x_register = value; }
}
