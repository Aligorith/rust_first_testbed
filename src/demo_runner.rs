/// "demo_runner"
/// Stuff to launch the demos
use std::io;
use std::process;

/* Note: Cannot do "mod terminal_utils" here, because that's seen as
 * telling Rust to "redeclare" the terminal_utils module. This is because
 * terminal_utils.rs is beside this file (and probably because we later
 * import (?) both of these in the main file)
 */
use terminal_utils;


// Function pointer/callback type for demo programs used here
pub type DemoProgramCallback = fn();

// Wrapper for defining a demo program
pub struct DemoProgramEntry {
	pub name : String,
	pub cb   : DemoProgramCallback, 
}


// Helper function: Print demo-command table
pub fn print_demo_listing(table: &Vec<DemoProgramEntry>) {
	println!("Choose which demo to run:");
	for (i, item) in table.iter().enumerate() {
		println!(" {index}) {description}",
		         index = i + 1,
		         description = item.name);
	}
	println!(" ---");
	println!(" z/x/exit - To exit");
}

// Helper function: Handle command input (single round)
pub fn handle_command_input(table: &Vec<DemoProgramEntry>) {
	// Show prompt
	terminal_utils::show_prompt("\n> ");
	
	// Read a line of text into the buffer
	let mut raw_input = String::new();
	io::stdin().read_line(&mut raw_input).expect("Please enter one of the numbers above...");
	
	// Strip off extra whitespace (including the newline)
	let command = raw_input.trim();
	
	// Try to treat the input as a number (common case)
	match command.parse::<usize>() {
		Ok(value) if (value > 0) && (value <= table.len()) => {
			// We got a number, so treat it as a demo program index
			let index = value - 1;
			(table[index].cb)();
		},
		Ok(value) => {
			// We got a number, but it was out of bounds
			println!("Unknown command number - {0}", value);
		}
		Err(_) => {
			// We got a string, so maybe it was the exit button?
			match command {
				// These two should be the same here - they both exit
				//"e" => break,
				"z" | "x" | "exit" => process::exit(0),
				
				// Catch all unknown commands
				_   => println!("Unknown command!")
			}
		}
	}	
}

