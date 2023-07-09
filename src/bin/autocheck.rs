use rust_ps::boj_scraper::*;

fn main() {
	let id = 16197;
	for (i, (input, output)) in get_boj_samples(id).into_iter().enumerate() {
		println!("Sample {}", i + 1);
		println!("Input");
		println!("{input}");
		println!("Output");
		println!("{output}");
		println!("Done.");
	}
}
