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


// Main entrypoint
fn main() {
	let table = vec![
		DemoProgramEntry { name: "range_summing_test()".to_string(),  cb: range_summing_test },
		DemoProgramEntry { name: "is_prime(x)".to_string(),           cb: is_prime_test },
		DemoProgramEntry { name: "fizzbuzz(x)".to_string(),           cb: fizzbuzz },
	];
	
	loop {
		// Print listing of available commands
		println!("Choose which demo to run:");
		for (i, item) in table.iter().enumerate() {
			println!(" {index}) {description}",
			         index = i + 1,
			         description = item.name);
		}
		println!(" ---");
		println!(" z/x/exit - To exit");
		
		
		// Get command input, and react to it...
		terminal_utils::show_prompt("\n> ");
		
		let mut raw_input = String::new();
		io::stdin().read_line(&mut raw_input).expect("Please enter one of the numbers above...");
		
		let command = raw_input.trim();
		
		match command.parse::<usize>() {
			Ok(value) => {
				// We got a number, so treat it as a demo program index
				let index = value - 1;
				if (0 <= index) && (index < table.len()) {
					(table[index as usize].cb)();
				}
				else {
					println!("Unknown command number!");
				}
			},
			Err(e) => {
				// We got a string, so maybe it was the exit button?
				match command {
					// These two should be the same here - they both exit
					"e" => break,
					"z" | "x" | "exit" => process::exit(0),
					
					// Catch all unknown commands
					_   => println!("Unknown command!")
				}
			}
		}
		
		// If still running, add some blank lines to seaprate the output
		println!("\n\n");
	}
}
