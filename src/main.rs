#![no_main]

#[allow(unused)]
use std::{cmp::*, collections::*, fmt::*, io::*, iter, mem::*, num::*, ops::*};

fn solve<R: Read>(mut rd: IStream<R>) {}

#[no_mangle]
fn main() -> i32 {
	solve(IStream::default());
	flush!();
	0
}

#[allow(unused)]
mod fastio {
	pub use super::ioutil::*;
	use std::{
		io::{read_to_string, stdin, Read, StdinLock},
		str::from_utf8,
	};

	pub const BUFFER_SIZE: usize = 1 << 17;

	#[derive(Debug)]
	pub struct IStream<R> {
		buf: Box<[u8; BUFFER_SIZE]>,
		off: usize,
		len: usize,
		inp: R,
	}

	impl Default for IStream<StdinLock<'_>> {
		fn default() -> Self {
			Self {
				buf: Box::new([0u8; BUFFER_SIZE]),
				off: BUFFER_SIZE,
				len: BUFFER_SIZE,
				inp: stdin().lock(),
			}
		}
	}

	impl<R: Read> IStream<R> {
		fn skip_whitespace(&mut self) {
			while self.len > 0 {
				if let Some(i) = self.buf[self.off..self.len].iter().position(|&b| b > b' ') {
					self.off += i;
					break;
				}
				self.fill();
			}
		}

		fn remain(&self) -> &[u8] {
			&self.buf[self.off..self.len]
		}

		fn fill(&mut self) {
			let len = self.inp.read(self.buf.as_mut_slice()).unwrap();
			(self.off, self.len) = (0, len);
		}

		fn next_token(&mut self) -> Option<String> {
			self.skip_whitespace();
			let mut s = String::new();
			loop {
				if self.len == 0 {
					return None;
				}
				let remain = self.remain();
				if let Some(i) = remain.iter().position(|&b| b <= b' ') {
					s.push_str(from_utf8(&remain[..i]).unwrap());
					self.off += i;
					break;
				} else {
					s.push_str(from_utf8(remain).unwrap());
					self.fill();
				}
			}
			Some(s)
		}
	}

	impl<R: Read> Iterator for IStream<R> {
		type Item = String;
		fn next(&mut self) -> Option<Self::Item> {
			self.next_token()
		}
	}

	impl<R: Read> IStream<R> {
		pub fn read_checked<A: IterParse<<Self as Iterator>::Item>>(&mut self) -> Option<A> {
			A::parse_from_iter(self)
		}
		pub fn read<A: IterParse<<Self as Iterator>::Item>>(&mut self) -> A {
			self.read_checked().unwrap()
		}
		pub fn read_map<A: IterParse<<Self as Iterator>::Item>, B, const N: usize>(&mut self, f: impl FnMut(A) -> B) -> [B; N] {
			self.read::<[A; N]>().map(f)
		}
		pub fn read_iter<A: IterParse<<Self as Iterator>::Item>>(&mut self) -> impl Iterator<Item = A> + '_ {
			std::iter::repeat_with(move || self.read_checked()).map_while(|x| x)
		}
		pub fn read_collect<A: IterParse<<Self as Iterator>::Item>, V: FromIterator<A>>(&mut self, size: usize) -> V {
			self.read_iter().take(size).collect()
		}
	}
}
use fastio::*;

mod ioutil {
	use std::{num::*, str::FromStr};

	pub trait Atom: FromStr {}

	macro_rules! impl_atom {
		($($t:ty)+) => {$( impl Atom for $t {} )+};
	}
	impl_atom!(f32 f64 bool);
	impl_atom!(i8 i16 i32 i64 i128 isize);
	impl_atom!(u8 u16 u32 u64 u128 usize);
	impl_atom!(NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize);
	impl_atom!(NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize);
	impl_atom!(String);

	pub trait IterParse<T: AsRef<str>>: Sized {
		fn parse_from_iter<It: Iterator<Item = T>>(it: &mut It) -> Option<Self>;
	}

	impl<A: Atom, T: AsRef<str>> IterParse<T> for A {
		fn parse_from_iter<It: Iterator<Item = T>>(it: &mut It) -> Option<Self> {
			it.next().and_then(|s| A::from_str(s.as_ref()).ok())
		}
	}

	impl<A: IterParse<T>, const N: usize, T: AsRef<str>> IterParse<T> for [A; N] {
		fn parse_from_iter<It: Iterator<Item = T>>(it: &mut It) -> Option<Self> {
			use std::mem::*;
			let mut x: [MaybeUninit<A>; N] = unsafe { MaybeUninit::uninit().assume_init() };
			for p in x.iter_mut() {
				*p = MaybeUninit::new(A::parse_from_iter(it)?);
			}
			Some(unsafe { transmute_copy(&x) })
		}
	}

	macro_rules! impl_tuple {
		($u:ident) => {};
		($u:ident $($t:ident)+) => {
			impl<$u: IterParse<T>, $($t: IterParse<T>),+, T: AsRef<str>> IterParse<T> for ($u, $($t),+) {
				fn parse_from_iter<It: Iterator<Item = T>>(it: &mut It) -> Option<Self> {
					Some(($u::parse_from_iter(it)?, $($t::parse_from_iter(it)?),+))
				}
			}
			impl_tuple!($($t)+);
		};
	}

	impl_tuple!(Q W E R TT Y U I O P A S D F G H J K L Z X C V B N M);
}

#[link(name = "c")]
extern "C" {}

use std::cell::RefCell;
thread_local! {
	static STDOUT: RefCell<BufWriter<StdoutLock<'static>>>
		= RefCell::new(BufWriter::with_capacity(crate::fastio::BUFFER_SIZE, stdout().lock()));
}

use std::io::Write;
#[macro_export]
macro_rules! println {
	($($t:tt)*) => { STDOUT.with(|cell| writeln!(cell.borrow_mut(), $($t)*).unwrap()); };
}
#[macro_export]
macro_rules! print {
	($($t:tt)*) => { STDOUT.with(|cell| write!(cell.borrow_mut(), $($t)*).unwrap()); };
}
#[macro_export]
macro_rules! flush {
	() => {
		STDOUT.with(|cell| cell.borrow_mut().flush().unwrap());
	};
}
