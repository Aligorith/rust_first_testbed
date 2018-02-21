extern crate time;
extern crate rand;

use std::thread;
use std::time as std_time;
use rand::distributions::{IndependentSample, Range};

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

/* Test of move semantics */
fn move_semantics_test() {
	let x = 5;
	let mut y = x;
	
	println!("X = {0}, y = {1}", x, y);
	while y < 10 {
		println!("X = {0}, y = {1}", x, y);
		//y++;
		y += 1;
	}
}

/* NOTE: The following code doesn't compile because "x" gets copied, 
 * since vec's don't implement the Copy trait
 */
// fn move_semantics_test2() {
// 	let x = vec![1, 2, 3];
// 	let mut y = x;
	
// 	y.push(4);
// 	y.push(5);
// 	y.pop();
	
// 	println!("x = {:?}", x);
// 	println!("y = {:?}", y);
// }

/* *********************************************** */

/* Playing with vecs */
fn vec_test_1(x : i64) {
	//let mut v1 = Vec::new(); // XXX: This is not as efficient when we know the size needed
	//let mut v1 = Vec::with_capacity(x as usize); // XXX: This risks overflow issues...
	
	let mut v1 = if x > 1 { 
					Vec::with_capacity(x as usize) 
				 }
				 else { 
					Vec:: new() 
				 };
				 
	
	/* 1) Populate the array */
	for i in 1..x {
		v1.push(i);
	}
	println!("Length of v1 = {0}", v1.len());
	assert_eq!(v1.len(), (x-1) as usize);
	
	/* 2) Try to pop an item off */
	match v1.pop() {
		None => println!("Too small to pop"),
		y    => println!("Length of v1 after {1:?} popped = {0}", v1.len(), y)
	}
}

/* Wrapper for vec_test_1 */
fn vec_test_1_w() {
	let value = terminal_utils::get_int("Please enter a postive integer:");
	vec_test_1(value);
}

/* *********************************************** */

/* Matching Test Function */
fn match_test_logic(val : i64) {
	match val {
		// ((val % 3) == 0) => println!("Multiple of 3"),
		// ((val % 5) == 0) => println!("Multiple of 5"),
		// ((val % 15) == 0) => println!("Multiple of both 3 and 5"),
		
		/* "Match Guards" */
		val if (val % 15 == 0) => println!("Multiple of both 3 and 5"),  /* Note: This must come first, as they're evaluated in order of appearance */
		val if (val % 3 == 0) => println!("Multiple of 3"),
		val if (val % 5 == 0) => println!("Multiple of 5"),
		//val if (val % 15 == 0) => println!("Multiple of both 3 and 5"),
		
		_ => println!("{}", val)
	}
}

/* Test function for match_test */
fn match_test() {
	for i in 1..30 {
		match_test_logic(i);
	}
}

/* *********************************************** */

/* Populate a list/vector with random floats */
// fn make_random_float_array(n: usize, min: f64, max: f64) -> Vec<f64> {
// 	let mut result = Vec::with_capacity(n as usize);
//	
// 	let between = Range::new(min, max); // from rand::distributions
// 	let mut rng = rand::thread_rng();
//	
// 	for _ in 0..n {
// 		let val = between.ind_sample(&mut rng);
// 		result.push(val);
// 	}
//	
// 	return result;
// }

/* Simpler implmentation of `make_random_float_array()` */
fn make_random_float_array_compact(n: usize, min: f64, max: f64) -> Vec<f64> {
	let between = Range::new(min, max);
	let mut rng = rand::thread_rng();
	
	(0..n).map(|_| between.ind_sample(&mut rng)).collect()
}


/* Compute average of list the standard way */
fn calc_average_standard(values: &Vec<f64>) -> f64 {
	let mut total = 0.0_f64;
	for val in values.iter() {
		total += *val;
	}
	return total / (values.len() as f64);
}

/* Compute average of list using the "online" way (to avoid overflows)
 * https://stackoverflow.com/questions/28820904/how-to-efficiently-compute-average-on-the-fly-moving-average
 */
fn calc_average_online(values: &Vec<f64>) -> f64 {
	let mut n = 1;
	let mut avg = 0.0_f64;
	
	for val in values.iter() {
		avg += (val - avg) / (n as f64);
		n += 1;
	}
	
	return avg;
}


/* Test two ways of averaging numbers, by running a whole bunch of these tests */
fn online_avg_test() {
	let mut n = terminal_utils::get_int_optional("Number of items in list:", 5) as usize;
	let mut repeats = terminal_utils::get_int_optional("Number of repeats:", 10) as usize;
	
	/* Sanity checks */
	// XXX: Make these part of the constraints on get_int()
	if n < 2 {
		/* This is pointless otherwise */
		n = 2;
	}
	if repeats < 2 {
		/* This is pointless otherwise */
		repeats = 2;
	}
	
	/* Run this experiment multiple times */
	let mut success_count = 0;
	
	// TODO: Align output columns
	println!("Testing Averaging Methods for List with n={0} items, {1} times...",
	         n, repeats);
	println!("==================================================================");
	
	
	for i in 0..repeats {
		let min = 1.0_f64;
		let max = 10.0_f64;
		
		// let values = make_random_float_array(n, min, max);
		let values = make_random_float_array_compact(n, min, max);
		
		
		let standard = calc_average_standard(&values);
		let online = calc_average_online(&values);
		
		let values_match = standard == online;
		let calc_error   = standard - online;
		if values_match {
			success_count += 1;
		}
		
		// TODO: compact representation?
		println!("\n  >> {0} | vs = {1:?}", i, &values);
		
		println!("    Standard = {0},  Online = {1}  | Match = {2} | Err = {3} \n",
		         standard, online, values_match, calc_error);
		
		if i < repeats - 1 {
			println!("---------------------------------------------------------------");
		}
	}
	
	println!("==================================================================");
	println!("\nSuccessful matches = {0} / {1}", success_count, repeats);
	
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
		demo_runner::DemoProgramEntry { name: "move_semantics_test()".to_string(), cb: move_semantics_test },
		demo_runner::DemoProgramEntry { name: "vec_test_1(x)".to_string(),         cb: vec_test_1_w },
		demo_runner::DemoProgramEntry { name: "match_test()".to_string(),          cb: match_test },
		demo_runner::DemoProgramEntry { name: "online_avg_test()".to_string(),     cb: online_avg_test },
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
