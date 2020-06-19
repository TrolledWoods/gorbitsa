use std::collections::HashMap;
use crate::commands;

/// Just a line number.
pub type Loc = usize;
pub type Error = (Loc, String);

pub fn parse_assembly(input: &str) -> Result<Vec<u8>, Error> {
	let mut commands = Vec::new();
	let mut labels = HashMap::new();
	let mut label_uses = Vec::new();
	for (n_line, line) in input.lines().map(|v| v.trim()).enumerate() {
		if line.len() == 0 { continue; }

		if line.starts_with("#") {
			// It's a comment.
			continue;
		}

		if line.starts_with(":") {
			// It's a label!
			labels.insert(line[1..].to_string(), commands.len());
			continue;
		}

		let mut sections = line.split_whitespace();

		// We know that the length is not zero after trimming. so there
		// has to be something here.
		let name = sections.next().unwrap();

		let instr = match commands::name_to_instr(name) {
			Some(instr) => instr,
			None => 
				return Err((n_line, format!("'{}' is not a command", name))),
		};

		let arg = if commands::command_has_arg(instr) {
			match sections.next() {
				Some(arg) => {
					if arg.starts_with(":") {
						label_uses.push((
								arg[1..].to_string(), 
								commands.len() + 1,
								n_line,
						));
						0
					} else {
						parse_argument(arg).map_err(|v| (n_line, v))?
					}
				}
				None => return Err((
						n_line, 
						format!("'{}'[{}] expects an argument", 
							name, commands::instr_to_desc(instr)),
				)),
			}
		} else {
			// Default argument
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

		commands[loc] = label_loc as u8;
	}

	Ok(commands)
}

fn parse_argument(arg: &str) -> Result<u8, String> {
	match arg.parse() {
		Ok(val) => Ok(val),
		Err(_) => return Err(format!("Argument is not a valid byte")),
	}
}
