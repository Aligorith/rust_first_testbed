use std::io;

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
	let mut raw_input = String::new();
	
	println!("Please enter an integer:");
	io::stdin().read_line(&mut raw_input)
		.expect("Please enter a number...");
		
	/* NOTE: raw_input() is a String, while raw_input.trim() 
	 * is a &str (i.e. a slice/reference to the original string buffer) 
	 */
	let input = raw_input.trim();
	
	// let value = input.parse::<i64>().unwrap();
	// match is_prime(value) {
	// 	true  => println!("{0} is Prime", value),
	// 	false => println!("{0} is Not Prime", value)
	// }
	
	match input.parse::<i64>() {
		Ok(value) => {
			match is_prime(value) {
				true  => println!("{0} is Prime", value),
				false => println!("{0} is Not Prime", value)
			}
		},
		Err(e) => {
			println!("Invalid input '{0}' - You must enter a number", input);
			println!("e = {0}", e);
		}
	}
	
	
}


fn main() {
	println!("Choose which demo to run:");
	println!(" 1) range_summing_test()");
	println!(" 2) is_prime(x)");
	
	
	// XXX: This only runs once for now, soon it will loop...
	println!("\n> ");
	
	let mut command = String::new();
	
	io::stdin().read_line(&mut command)
		.expect("Please enter one of the numbers above...");
	
	match command.trim() {
		"1" => range_summing_test(),
		"2" => is_prime_test(),
		_   => println!("Unknown command!")
	}
}
