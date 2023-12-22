use crate::core::{
	boj_scraper::*,
	compile_run::{compile, run_exec},
};
use std::{
	env, fs,
	io::{stdout, Write},
};

fn main() {
	const SRC: &str = "./rust-ps/src/main.rs";
	const OUT: &str = "./boj-test/tmp/a.out";

	print!("Compiling the code... ");
	stdout().flush().unwrap();
	let exec = compile(SRC, OUT).expect("Failed to compile");
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
		if result.as_ref().map_or(false, |ret| check_if_correct(ret, &output)) {
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

	fs::remove_file(OUT).expect("Failed to remove the generated executable");
}

fn check_if_correct(ret: &str, out: &str) -> bool {
	let xarr: Vec<_> = ret.trim_end().lines().collect();
	let yarr: Vec<_> = out.trim_end().lines().collect();
	xarr == yarr
}

mod core {
	pub mod boj_scraper {
		use scraper::Html;
		use std::rc::Rc;

		/// Returns a vector of pairs of sample inputs and outputs.
		pub fn get_boj_samples(id: u32) -> impl Iterator<Item = (String, String)> {
			let doc = Rc::new(get_boj_problem(id));
			let h1 = doc.clone();
			boj_sample_inputs(h1).zip(boj_sample_outputs(doc))
		}

		fn get_boj_problem(id: u32) -> Html {
			let url = format!("https://www.acmicpc.net/problem/{id}");
			get_html(&url)
		}

		fn boj_sample_inputs(doc: Rc<Html>) -> impl Iterator<Item = String> {
			(1..).map_while(move |id| {
				let query = scraper::Selector::parse(&format!("#sample-input-{id}")).unwrap();
				doc.select(&query).map(|x| x.inner_html()).next()
			})
		}

		fn boj_sample_outputs(doc: Rc<Html>) -> impl Iterator<Item = String> {
			(1..).map_while(move |id| {
				let query = scraper::Selector::parse(&format!("#sample-output-{id}")).unwrap();
				doc.select(&query).map(|x| x.inner_html()).next()
			})
		}

		fn get_html(url: &str) -> Html {
			let request = ureq::get(url).call().unwrap().into_string().unwrap();
			Html::parse_document(&request)
		}
	}

	pub mod compile_run {
		use rand::{distributions::Alphanumeric, thread_rng, Rng};
		use std::{
			env,
			ffi::OsStr,
			fs::{self, File},
			io::{self, Write},
			path::{Path, PathBuf},
			process::{Command, Stdio},
		};

		pub fn compile(code_path: impl AsRef<Path>, exec_path: impl AsRef<Path>) -> io::Result<PathBuf> {
			let code_path = absolute_path(code_path)?;
			let exec_path = absolute_path(exec_path)?;

			Command::new("rustc")
				.args(["--edition", "2021", "-O", "-o"])
				.arg(&exec_path)
				.arg(&code_path)
				.spawn()
				.and_then(|mut x| x.wait())
				.expect("Failed to execute compilation");

			if exec_path.exists() {
				Ok(exec_path)
			} else {
				Err(io::Error::new(io::ErrorKind::NotFound, "Requested executable couldn't be generated"))
			}
		}

		pub fn run_exec(exec_path: impl AsRef<OsStr>, input: &str) -> Result<String, String> {
			let curr_path = absolute_path("./boj-test/tmp/").map_err(|e| e.to_string())?;

			let input_loc = generate_file(curr_path, input).map_err(|e| format!("Error while generating the file: {e}"))?;
			let input_file = File::open(&input_loc).map_err(|e| format!("Error while opening the input file: {e}"))?;

			let proc = Command::new(&exec_path)
				.stdin(Stdio::from(input_file))
				.stdout(Stdio::piped())
				.stdout(Stdio::piped())
				.spawn()
				.map_err(|e| format!("Error while spawning process: {e}"))?;

			let ret = proc.wait_with_output().map_err(|e| format!("Error while waiting for the process: {e}"))?;
			while fs::remove_file(&input_loc).is_ok() {}
			if ret.stderr.is_empty() {
				Ok(std::str::from_utf8(&ret.stdout).map_err(|e| format!("stdout is not utf8: {e}"))?.to_owned())
			} else {
				Err(std::str::from_utf8(&ret.stderr).map_err(|e| format!("stderr is not utf8: {e}"))?.to_owned())
			}
		}

		fn generate_file(dir: impl AsRef<Path>, content: &str) -> io::Result<PathBuf> {
			let mut file_path = dir.as_ref().to_owned();
			file_path.push(random_name());
			file_path.set_extension("txt");
			let mut file = File::create(file_path.clone())?;
			file.write_all(content.as_bytes())?;
			Ok(file_path)
		}

		fn random_name() -> String { thread_rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect() }

		fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
			let path = path.as_ref();

			let absolute_path = if path.is_absolute() { path.to_path_buf() } else { env::current_dir()?.join(path) };

			Ok(absolute_path)
		}

		#[cfg(test)]
		mod test {
			use super::*;
			use std::io;

			#[test]
			fn compile_test() -> io::Result<()> {
				let code_path = "./boj-test/src/main.rs";
				let exec_path = "./boj-test/tmp/a.out";

				let _exec = compile(code_path, exec_path);
				Ok(())
			}

			#[test]
			fn run_test() -> io::Result<()> {
				let code_path = "./boj-test/src/main.rs";
				let exec_path = "./boj-test/tmp/a.out";

				let exec = compile(code_path, exec_path)?;
				let output = "xxx";
				let ret = run_exec(exec, "xxx").unwrap();
				assert_eq!(output, ret);
				Ok(())
			}
		}
	}
}
