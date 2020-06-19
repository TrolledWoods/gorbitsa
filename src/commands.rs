/// Defines the commands to use. 
macro_rules! commands {
	($vm:ident, $arg:ident => $($long_name:expr; $desc:expr; $name:expr; ($has_arg:expr) $run:expr),*,) => {
		pub fn name_to_instr(long: &str) -> Option<u8> {
			match long {
				$(
					$long_name => Some($name[0]),
				)*
				_ => None,
			}
		}

		pub fn instr_to_name(instr: u8) -> &'static str {
			match &[instr] {
				$(
					$name => $long_name,
				)*
				_ => panic!("Invalid command {}", instr),
			}
		}

		pub fn instr_to_desc(instr: u8) -> &'static str {
			match &[instr] {
				$(
					$name => $desc,
				)*
				_ => panic!("Invalid command {}", instr),
			}
		}

		pub fn run_command($vm: &mut impl VM, cmd: u8, $arg: u8) {
			match &[cmd] {
				$(
					$name => $run,
				)*
				_ => panic!("Invalid command {}", cmd),
			}
		}

		pub fn command_has_arg(cmd: u8) -> bool {
			match &[cmd] {
				$(
					$name => $has_arg,
				)*
				_ => panic!("Invalid command {}", cmd),
			}
		}
	}
}

commands!(vm, arg =>
	"0"; "just zero bro";
	b"\0"; (false) {},
	"mem"; "x = mem[argument]";
	b"G"; (true)  vm.set_x(vm.mem(arg)),
	"mem_set_x"; "mem[argument] = x";
	b"O"; (true)  vm.set_mem(arg, vm.x()),
	"input"; "x = next value in input";
	b"R"; (false) { 
		let input = vm.read_input();
		vm.set_x(input);
	}, 
	"branch"; "if x == 0 goto argument";
	b"B"; (true)  if vm.x() == 0 { vm.jump(arg); },
	"incr"; "x += argument";
	b"I"; (true)  vm.set_x(vm.x().wrapping_add(arg)),
	"output_x"; "push x to output";
	b"T"; (false) vm.write_output(vm.x()),
	"const"; "x = argument";
	b"S"; (true)  vm.set_x(arg),
	"incr_mem"; "x += mem[argument";
	b"A"; (true)  vm.set_x(vm.x() + vm.mem(arg)),

	"memi"; "x = mem[mem[argument]]";
	b"g"; (true)  vm.set_x(vm.mem(vm.mem(arg))),
	"memi_set_x"; "mem[mem[argument]] = x";
	b"o"; (true)  vm.set_mem(vm.mem(arg), vm.x()),
	"mem_set_input"; "mem[argument] = next value in input";
	b"r"; (true)  { 
		let input = vm.read_input();
		vm.set_mem(arg, input);
	},
	"branch_mem"; "if x == 0 goto mem[argument]";
	b"b"; (true)  if vm.x() == 0 { vm.jump(vm.mem(arg)); },
	"mem_incr_x"; "mem[argument] += x";
	b"i"; (true)  vm.set_mem(arg, vm.mem(arg).wrapping_add(vm.x())),
	"output_mem"; "print mem[argument]";
	b"t"; (true)  vm.write_output(vm.mem(arg)),
	"xor"; "x = xor(mem[argument], x)";
	b"s"; (true)  vm.set_x(vm.mem(arg) ^ vm.x()),
	"incr_memi"; "x += mem[mem[argument]]";
	b"a"; (true)  vm.set_x(vm.mem(vm.mem(arg)).wrapping_add(vm.x())),
);

pub trait VM {
	fn mem(&self, pos: u8) -> u8;
	fn set_mem(&mut self, pos: u8, value: u8);
	fn x(&self) -> u8;
	fn set_x(&mut self, value: u8);

	fn read_input(&mut self) -> u8;
	fn write_output(&mut self, output: u8);

	/// Returns the next instruction and argument, and increments the
	/// instruction pointer.
	fn next(&mut self) -> Option<(u8, u8)>;
	fn jump(&mut self, instr: u8);
}

pub struct RamVm<'a> {
	pub ram: [u8; 256],
	pub instr_pointer: u16,
	pub x_register: u8,
	pub input: &'a [u8],
	pub output: Vec<u8>,
}

impl RamVm<'_> {
	pub fn get_gorbitsa(&self) -> String {
		let mut index = 0;
		let mut string = String::new();
		while self.ram[index] > 0 {
			string.push(self.ram[index] as char);
			if command_has_arg(self.ram[index]) {
				use std::fmt::Write;
				write!(string, "{} ", self.ram[index + 1]).unwrap();
			} else { 
				string.push(' '); 
			}
			index += 2;
		}
		string
	}
}

impl VM for RamVm<'_> {
	fn mem(&self, pos: u8) -> u8 { self.ram[pos as usize] }
	fn set_mem(&mut self, pos: u8, value: u8) { 
		self.ram[pos as usize] = value; 
	}
	fn x(&self) -> u8 { self.x_register }
	fn set_x(&mut self, value: u8) { self.x_register = value; }

	fn read_input(&mut self) -> u8 { 
		if let Some(first) = self.input.first().copied() {
			self.input = &self.input[1..];
			first
		} else {
			// The default input
			0
		}
	}

	fn write_output(&mut self, output: u8) {
		self.output.push(output);
	}

	fn next(&mut self) -> Option<(u8, u8)> {
		let instr = *self.ram.get(self.instr_pointer as usize)?;
		let arg   = *self.ram.get(self.instr_pointer as usize + 1)?;

		self.instr_pointer += 2;

		Some((instr, arg))
	}

	fn jump(&mut self, pos: u8) {
		self.instr_pointer = pos as u16;
	}
}
