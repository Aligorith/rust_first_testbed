extern crate time;
use std::thread;
use std::time as std_time;

mod demo_runner;
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

/* *********************************************** */


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

/* *********************************************** */


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


/* *********************************************** */


fn timestamp_to_str(seconds: u64, nanos: u32) -> String {
	let ms = nanos / 1000000;
	format!("{0}.{1}", seconds, ms)
}


fn time_formatting() {
	let ts = time::now();
	
	println!("The time now is:");
	println!(" .to_timespec() -> {:?}", ts.to_timespec()); // seconds from epoch
	
	let timespec = ts.to_timespec();
	println!(" timespec {0}.{1:2}", timespec.sec, timespec.nsec); // seconds from epoch, manually extracted
	println!(" timespec str = {}",
	         timestamp_to_str(timespec.sec as u64, 
	                          timespec.nsec as u32));
	
	println!(" .strftime(local) -> {}", ts.strftime("%X").unwrap()); // HH:MM:SS formatted in local system time
}


fn time_elapsed() {
	let ts = std_time::Instant::now();
	
	println!("The starting timestamp is: {:?}", ts);
	
	let half_sec = std_time::Duration::from_millis(500);
	thread::sleep(half_sec);
	
	println!("Result after waiting 500ms is: {:?}", ts.elapsed());
	//println!("  => {:.3}", ts.elapsed().subsec_nanos() / 1000000);
	println!("  => {} sec", 
	         timestamp_to_str(ts.elapsed().as_secs(), 
	                          ts.elapsed().subsec_nanos()));
}



/* *********************************************** */


// Main entrypoint
fn main() {
	let table = vec![
		demo_runner::DemoProgramEntry { name: "range_summing_test()".to_string(),  cb: range_summing_test },
		demo_runner::DemoProgramEntry { name: "is_prime(x)".to_string(),           cb: is_prime_test },
		demo_runner::DemoProgramEntry { name: "fizzbuzz(x)".to_string(),           cb: fizzbuzz },
		demo_runner::DemoProgramEntry { name: "time_formatting()".to_string(),     cb: time_formatting },
		demo_runner::DemoProgramEntry { name: "time_elapsed()".to_string(),        cb: time_elapsed },
	];
	
	loop {
		// Print listing of available commands
		demo_runner::print_demo_listing(&table);
		
		// Get command input, and react to it...
		// NOTE: We may exit the program from in here
		demo_runner::handle_command_input(&table);
		
		// If still running, add some blank lines to seaprate the output
		println!("\n\n");
	}
}
