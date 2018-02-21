/// "terminal_utils" module
/// Contains utilities for interacting with the user
/// (e.g. getting/handling input, showing prompts, etc.)

use std::io::{self, Write};
use std::error::Error;


pub fn show_prompt(prompt_str: &str) {
	print!("{0}", prompt_str);
	io::stdout().flush().unwrap(); // stdout is buffered
}

// TODO: Allow extra restrictions about positive/negative
pub fn get_int(prompt_str: &str) -> i64 {
	println!("{0}", prompt_str);
	show_prompt("# ");
	
	/* NOTE: Use "loop" for infinite loops; Using "while" for this will complain about needing an int? */
	loop {
		let mut raw_input = String::new();
		
		io::stdin().read_line(&mut raw_input)
			.expect("Please enter a number...");
		
		/* NOTE: raw_input() is a String, while raw_input.trim() 
		 * is a &str (i.e. a slice/reference to the original string buffer) 
		 */
		let input = raw_input.trim();
		
		match input.parse::<i64>() {
			Ok(value) => {
				return value;
			},
			Err(e) => {
				println!("Invalid input '{0}' - You must enter a number", input);
				println!("e = {0}", e);
				
				show_prompt("\n# ");
			}
		}
	}
}

/* Like get_int, except if the user just presses enter, we just use the default value */
pub fn get_int_optional(prompt_str: &str, default_value: i64) -> i64 {
	println!("{0} [default = {1}]", prompt_str, default_value);
	show_prompt("# ");
	
	loop {
		let mut raw_input = String::new();
		
		io::stdin().read_line(&mut raw_input)
			.expect("Please enter a number...");
		
		/* NOTE: raw_input is a String, while raw_input.trim() 
		 * is a &str (i.e. a slice/reference to the original string buffer) 
		 */
		let input = raw_input.trim();
		
		match input.parse::<i64>() {
			Ok(value) => {
				return value;
			},
			Err(e) => {
				match e.description() {
					"cannot parse integer from empty string" => {
						/* Special Case: Return default_value (since user was too lazy to enter one,
						 * and just pressed enter) 
						 */
						return default_value;
					},
					"invalid digit found in string" => {
						/* Check for text vs dots */
						if input.find('.').is_some() {
							println!("Invalid input '{0}' - You must enter an integer", input);
						}
						else {
							println!("Invalid input '{0}' - You must enter an integer (digits 0-9) only", input);
						}
						
						show_prompt("\n# ");
					},
					_ => {
						/* Default Case: Unknown error */
						println!("Unrecognised input '{0}' - You must enter an integer (digits 0-9) only", input);
						show_prompt("\n# ");
					}
				}
			}
		}
	}
}
