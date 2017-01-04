use std::io;
use std::process;

mod terminal_utils;


fn range_summing_test() {
	let fac = 0.5;      // immutable constant
	let mut total = 0.0; // running total sum - can be mutable
	
	// Sum things up
	for i in 0..10 {
		let x = i as f64; // XXX: This seems clunky, but we can't multiply a float by an int, hence the cast here
		let val = (fac * x) + total;
		
		println!("Combining {0} and {1} => {2}", i, total, val);
		// total += x;
		total = val;
	}
	
	println!("Total Sum is {}", total);
}


fn is_prime(x: i64) -> bool {
	for i in 2..(x - 1) {
		if x % i == 0 {
			return false;
		}
	}
	return true;
}

fn is_prime_test() {
	let value = terminal_utils::get_int("Please enter a postive integer:");
	
	match is_prime(value) {
		true  => println!("{0} is Prime", value),
		false => println!("{0} is Not Prime", value)
	}
}




fn fizzbuzz() {
	let stop = terminal_utils::get_int("Please enter a positive integer:");
	let mut n = 0;  // counter
	
	while n < stop {
		if n % 15 == 0 {
			println!("fizzbuzz");
		}
		else if (n % 3) == 0 {
			println!("fizz");
		}
		else if (n % 5) == 0 {
			println!("buzz");
		}
		else {
			println!("{}", n);
		}
		n += 1;
	}
}



// Function pointer/callback type for demo programs used here
type DemoProgramCallback = fn();

// Wrapper for defining a demo program
struct DemoProgramEntry {
	name : String,
	cb   : DemoProgramCallback, 
}


// Helper function: Print demo-command table
fn print_demo_listing(table: &Vec<DemoProgramEntry>) {
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
fn handle_command_input(table: &Vec<DemoProgramEntry>) {
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


// Main entrypoint
fn main() {
	let table = vec![
		DemoProgramEntry { name: "range_summing_test()".to_string(),  cb: range_summing_test },
		DemoProgramEntry { name: "is_prime(x)".to_string(),           cb: is_prime_test },
		DemoProgramEntry { name: "fizzbuzz(x)".to_string(),           cb: fizzbuzz },
	];
	
	loop {
		// Print listing of available commands
		print_demo_listing(&table);
		
		
		// Get command input, and react to it...
		// NOTE: We may exit the program from in here
		handle_command_input(&table);
		
		// If still running, add some blank lines to seaprate the output
		println!("\n\n");
	}
}
