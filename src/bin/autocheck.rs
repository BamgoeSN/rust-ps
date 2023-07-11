use rust_ps::{
	boj_scraper::*,
	compile_run::{compile, run_exec},
};
use std::{
	env, fs,
	io::{stdout, Write},
};

fn main() {
	print!("Compiling the code... ");
	stdout().flush().unwrap();
	let exec = compile("./src/main.rs", "./a.out").expect("Failed to compile");
	println!("Done.");
	stdout().flush().unwrap();

	let mut args = env::args();
	args.next();
	let id: u32 = args.next().expect("No argument provided").parse().expect("Invalid problem ID");
	print!("Getting samples from BOJ problem #{id}... ");
	stdout().flush().unwrap();
	let samples = get_boj_samples(id);
	println!("Done.");
	println!();
	stdout().flush().unwrap();

	for (i, (input, output)) in samples.enumerate() {
		print!("Sample {}... ", i + 1);
		stdout().flush().unwrap();
		let result = run_exec(&exec, &input);
		if result.as_ref().map_or(false, |ret| ret.trim_end().lines().zip(output.trim_end().lines()).all(|(x, y)| x == y)) {
			// Correct
			println!("ok");
		} else {
			// Incorrect
			println!("WRONG!");
			println!("Expected...");
			println!("{}", output.trim_end());
			println!("Found...");
			let ret = match result {
				Ok(ref s) => s,
				Err(ref s) => s,
			};
			println!("{}", ret.trim_end());
			println!();
		}
		stdout().flush().unwrap();
	}

	fs::remove_file("./a.out").expect("Failed to remove the generated executable");
}
