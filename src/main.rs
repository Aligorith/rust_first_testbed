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




fn main() {
	loop {
		println!("Choose which demo to run:");
		println!(" 1) range_summing_test()");
		println!(" 2) is_prime(x)");
		println!(" 3) fizzbuzz(x)");
		println!(" ---");
		println!(" z/x/exit - To exit");
		
		terminal_utils::show_prompt("\n> ");
		
		let mut command = String::new();
		
		io::stdin().read_line(&mut command)
			.expect("Please enter one of the numbers above...");
		
		match command.trim() {
			"1" => range_summing_test(),
			"2" => is_prime_test(),
			"3" => fizzbuzz(),
			
			// These two should be the same here - they both exit
			"e" => break,
			"z" | "x" | "exit" => process::exit(0),
			
			// Catch all unknown commands
			_   => println!("Unknown command!")
		}
		
		// If still running, add some blank lines to seaprate the output
		println!("\n\n");
	}
}
