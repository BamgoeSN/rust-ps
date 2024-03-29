#![no_main]

#[allow(unused)]
use std::{cmp::*, collections::*, iter, mem::*, num::*, ops::*};

fn solve<'t, It: Iterator<Item = &'t str>>(sc: &mut fastio::Tokenizer<It>) {
	while let Ok(v) = sc.next_ok::<i32>() {
		println!("{v}");
	}
}

#[allow(unused)]
mod fastio {
	use super::ioutil::*;

	pub struct Tokenizer<It> {
		it: It,
	}

	impl<'i, 's: 'i, It> Tokenizer<It> {
		pub fn new(text: &'s str, split: impl FnOnce(&'i str) -> It) -> Self {
			Self { it: split(text) }
		}
	}

	impl<'t, It: Iterator<Item = &'t str>> Tokenizer<It> {
		pub fn next_ok<T: IterParse<'t>>(&mut self) -> PRes<'t, T> {
			T::parse_from_iter(&mut self.it)
		}

		pub fn next<T: IterParse<'t>>(&mut self) -> T {
			self.next_ok().unwrap()
		}

		pub fn next_map<T: IterParse<'t>, U, const N: usize>(&mut self, f: impl FnMut(T) -> U) -> [U; N] {
			let x: [T; N] = self.next();
			x.map(f)
		}

		pub fn next_it<T: IterParse<'t>>(&mut self) -> impl Iterator<Item = T> + '_ {
			std::iter::repeat_with(move || self.next_ok().ok()).map_while(|x| x)
		}

		pub fn next_collect<T: IterParse<'t>, V: std::iter::FromIterator<T>>(&mut self, size: usize) -> V {
			self.next_it().take(size).collect()
		}
	}
}

mod ioutil {
	use std::{fmt::*, num::*};

	pub enum InputError<'t> {
		InputExhaust,
		ParseError(&'t str),
	}
	use self::InputError::*;

	pub type PRes<'t, T> = std::result::Result<T, InputError<'t>>;

	impl<'t> Debug for InputError<'t> {
		fn fmt(&self, f: &mut Formatter<'_>) -> Result {
			match self {
				InputExhaust => f.debug_struct("InputExhaust").finish(),
				ParseError(s) => f.debug_struct("ParseError").field("str", s).finish(),
			}
		}
	}

	pub trait Atom<'t>: Sized {
		fn parse(text: &'t str) -> PRes<'t, Self>;
	}

	impl<'t> Atom<'t> for &'t str {
		fn parse(text: &'t str) -> PRes<'t, Self> {
			Ok(text)
		}
	}

	impl<'t> Atom<'t> for &'t [u8] {
		fn parse(text: &'t str) -> PRes<'t, Self> {
			Ok(text.as_bytes())
		}
	}

	macro_rules! impl_atom {
		($($t:ty) *) => { $(impl Atom<'_> for $t { fn parse(text: &str) -> PRes<Self> { text.parse().map_err(|_| ParseError(text)) } })* };
	}
	impl_atom!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool char String NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize);

	pub trait IterParse<'t>: Sized {
		fn parse_from_iter<'s, It: Iterator<Item = &'t str>>(it: &'s mut It) -> PRes<'t, Self>
		where
			't: 's;
	}

	impl<'t, A: Atom<'t>> IterParse<'t> for A {
		fn parse_from_iter<'s, It: Iterator<Item = &'t str>>(it: &'s mut It) -> PRes<'t, Self>
		where
			't: 's,
		{
			it.next().map_or(Err(InputExhaust), <Self as Atom>::parse)
		}
	}

	impl<'t, A: IterParse<'t>, const N: usize> IterParse<'t> for [A; N] {
		fn parse_from_iter<'s, It: Iterator<Item = &'t str>>(it: &'s mut It) -> PRes<'t, Self>
		where
			't: 's,
		{
			use std::mem::*;
			let mut x: [MaybeUninit<A>; N] = unsafe { MaybeUninit::uninit().assume_init() };
			for p in x.iter_mut() {
				*p = MaybeUninit::new(A::parse_from_iter(it)?);
			}
			Ok(unsafe { transmute_copy(&x) })
		}
	}

	macro_rules! impl_tuple {
		($u:ident) => {};
		($u:ident $($t:ident)+) => { impl<'t, $u: IterParse<'t>, $($t: IterParse<'t>),+> IterParse<'t> for ($u, $($t),+) { fn parse_from_iter<'s, It: Iterator<Item = &'t str>>(_it: &'s mut It) -> PRes<'t, Self> where 't: 's { Ok(($u::parse_from_iter(_it)?, $($t::parse_from_iter(_it)?),+)) } } impl_tuple!($($t) +); };
	}

	impl_tuple!(Q W E R T Y U I O P A S D F G H J K L Z X C V B N M);
}

#[link(name = "c")]
extern "C" {}

fn get_input() -> &'static str {
	use std::io::*;
	let input = read_to_string(stdin()).unwrap();
	Box::leak(input.into_boxed_str())
}

#[no_mangle]
unsafe fn main() -> i32 {
	use std::io::*;
	let mut sc = fastio::Tokenizer::new(get_input(), |s| s.split_ascii_whitespace());
	let stdout = stdout();
	WRITER = Some(BufWriter::new(stdout.lock()));
	solve(&mut sc);
	WRITER.as_mut().unwrap_unchecked().flush().ok();
	0
}

use std::io::{BufWriter, StdoutLock};
static mut WRITER: Option<BufWriter<StdoutLock>> = None;
#[macro_export]
macro_rules! print {
	($($t:tt)*) => {{ use std::io::*; write!(unsafe{ WRITER.as_mut().unwrap_unchecked() }, $($t)*).unwrap(); }};
}
#[macro_export]
macro_rules! println {
	($($t:tt)*) => {{ use std::io::*; writeln!(unsafe{ WRITER.as_mut().unwrap_unchecked() }, $($t)*).unwrap(); }};
}
