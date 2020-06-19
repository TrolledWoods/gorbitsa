use std::collections::HashMap;
use crate::commands;

/// Just a line number.
pub type Loc = usize;
pub type Error = (Loc, String);

pub fn parse_assembly(input: &str) -> Result<Vec<u8>, Error> {
	let mut commands = Vec::new();
	let mut labels = HashMap::new();
	let mut label_uses = Vec::new();

	let mut variables = HashMap::new();
	let mut variable_uses = Vec::new();

	let mut empty_argument_slots = Vec::new();

	for (n_line, line) in input.lines().map(|v| v.trim()).enumerate() {
		if line.len() == 0 { continue; }

		if line.starts_with("#") {
			// It's a comment.
			continue;
		}

		let mut sections = line.split_whitespace();

		// We know that the length is not zero after trimming. so there
		// has to be something here.
		let name = sections.next().unwrap();

		if name.starts_with("$") {
			// It's a variable. We cannot figure out the position of it in
			// memory yet.
			let name = name[1..].to_string();

			let size = if let Some(size_str) = sections.next() {
				match size_str.parse::<u8>() {
					Ok(size) => size,
					Err(_) => return Err((
						n_line, 
						format!("Invalid argument offset")
					)),
				}
			} else {
				return Err((
					n_line,
					format!("A variable expects a size"),
				));
			};

			variables.insert(name, (size, 0));
			continue;
		}

		if name.starts_with(":") {
			// It's a label!
			labels.insert(line[1..].to_string(), commands.len());
			continue;
		}

		let instr = match commands::name_to_instr(name) {
			Some(instr) => instr,
			None => 
				return Err((n_line, format!("'{}' is not a command", name))),
		};

		let arg = if commands::command_has_arg(instr) {
			let arg_str = match sections.next() {
				Some(val) => val,
				None => return Err((
					n_line,
					format!("Expected an argument"),
				)),
			};

			let mut arg = parse_argument(
				arg_str,
				&mut label_uses,
				&mut variable_uses,
				commands.len() + 1,
				n_line,
			)?;

			if let Some(offset_str) = sections.next() {
				arg += match offset_str.parse::<u8>() {
					Ok(offset) => offset,
					Err(_) => return Err((
						n_line, 
						format!("Invalid argument offset")
					)),
				};
			}

			arg
		} else {
			// This byte isn't used, so we could use it to store a
			// variable.
			empty_argument_slots.push(commands.len() + 1);

			0
		};

		if sections.next().is_some() {
			return Err((
					n_line,
					format!("'{}'[{}] does not want this many arguments!",
						name, commands::instr_to_desc(instr)),
			));
		}

		commands.reserve(2);
		commands.push(instr);
		commands.push(arg);

		if commands.len() > 255 {
			return Err((
					n_line,
					format!("Program cannot exceed 255 bytes in size"),
			));
		}
	}

	for (name, loc, line) in label_uses {
		let label_loc = match labels.get(&name) {
			Some(value) => *value,
			None => return Err((
					line,
					format!("Label '{}' does not exist", &name),
			)),
		};

		commands[loc] = commands[loc].wrapping_add(label_loc as u8);
	}

	let mut variable_loc = commands.len() + 1;
	for (name, (size, pos)) in variables.iter_mut() {
		// Try to see if there is an empty argument slot to use.
		if *size == 1 {
			if let Some(slot) = empty_argument_slots.pop() {
				*pos = slot;
				continue;
			}
		}

		*pos = variable_loc;
		variable_loc += *size as usize;
	}

	for (name, loc, line) in variable_uses {
		let variable_loc = match variables.get(&name) {
			Some((_, value)) => *value,
			None => return Err((
					line,
					format!("Variable '{}' does not exist", &name),
			)),
		};

		commands[loc] = commands[loc].wrapping_add(variable_loc as u8);
	}

	println!("MEMORY USAGE(in bytes): {}", variable_loc);

	Ok(commands)
}

fn parse_argument(
	arg: &str, 
	label_uses: &mut Vec<(String, usize, usize)>, 
	variable_uses: &mut Vec<(String, usize, usize)>,
	arg_loc: usize,
	n_line: usize,
) -> Result<u8, (usize, String)> {
	if arg.starts_with("-") {
		Ok((!parse_argument(&arg[1..], label_uses, variable_uses, arg_loc, n_line)?).wrapping_add(1))
	}else if arg.starts_with(":") {
		label_uses.push((
				arg[1..].to_string(), 
				arg_loc,
				n_line,
		));

		Ok(0)
	} else if arg.starts_with("$") {
		variable_uses.push((
			arg[1..].to_string(),
			arg_loc,
			n_line,
		));

		Ok(0)
	} else if arg.starts_with("'") {
		let mut bytes = arg.bytes();
		bytes.next();
		let byte = match bytes.next() {
			Some(val) => val,
			None => return Err((n_line, format!("Expected character after '"))),
		};
		match bytes.next() {
			Some(_) => 
				return Err((n_line, format!("Too many characters after '"))),
			None => (),
		}

		Ok(byte)
	} else {
		match arg.parse() {
			Ok(val) => Ok(val),
			Err(_) => return Err((n_line, format!("Invalid argument"))),
		}
	}

}
