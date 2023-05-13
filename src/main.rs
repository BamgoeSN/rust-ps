#![no_main]

#[no_mangle]
fn main() -> i32 {
    // FastIO
    use fastio::*;
    let input_str = get_input();
    let mut sc = Tokenizer::new(input_str, |s| s.split_ascii_whitespace());
    use std::io::{stdout, BufWriter, Write};
    let stdout = stdout();
    let wr = &mut BufWriter::new(stdout.lock());

    // FastIO Macros
    macro_rules! out { ($($arg:tt)*) => { write!(wr, $($arg)*).ok(); }; }
    macro_rules! outln { ($($arg:tt)*) => { writeln!(wr, $($arg)*).ok(); }; }

    // Main

    wr.flush().unwrap();
    0
}

#[allow(unused)]
mod fastio {
    use super::ioutil::*;
    use std::io;

    #[link(name = "c")]
    extern "C" {}

    pub fn get_input() -> &'static str {
        let buf = io::read_to_string(io::stdin()).unwrap();
        Box::leak(buf.into_boxed_str())
    }

    pub struct Tokenizer<It> {
        it: It,
    }

    impl<'i, 's: 'i, It> Tokenizer<It> {
        pub fn new(text: &'s str, split: impl FnOnce(&'i str) -> It) -> Self {
            Self { it: split(text) }
        }
    }

    impl<'t, It> Tokenizer<It>
    where
        It: Iterator<Item = &'t str>,
    {
        pub fn next_ok<T: IterParse<'t>>(&mut self) -> Result<'t, T> {
            T::parse_from_iter(&mut self.it)
        }

        pub fn next<T: IterParse<'t>>(&mut self) -> T {
            self.next_ok().unwrap()
        }

        pub fn next_map<T, U, const N: usize>(&mut self, f: impl FnMut(T) -> U) -> [U; N]
        where
            T: IterParse<'t>,
        {
            let x: [T; N] = self.next();
            x.map(f)
        }

        pub fn next_it<T: IterParse<'t>>(&mut self) -> impl Iterator<Item = T> + '_ {
            std::iter::repeat_with(move || self.next_ok().ok()).map_while(|x| x)
        }

        pub fn next_collect<T, V>(&mut self, size: usize) -> V
        where
            T: IterParse<'t>,
            V: FromIterator<T>,
        {
            self.next_it().take(size).collect()
        }
    }
}

mod ioutil {
    use std::{
        fmt::{Result as FRes, *},
        num::*,
    };

    pub enum InputError<'t> {
        InputExhaust,
        ParseError(&'t str),
    }
    use InputError::*;

    pub type Result<'t, T> = std::result::Result<T, InputError<'t>>;

    impl<'t> Debug for InputError<'t> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FRes {
            match self {
                InputExhaust => f.debug_struct("InputExhaust").finish(),
                ParseError(s) => f.debug_struct("ParseError").field("str", s).finish(),
            }
        }
    }

    pub trait Atom<'t>: Sized {
        fn parse(text: &'t str) -> Result<'t, Self>;
    }

    impl<'t> Atom<'t> for &'t str {
        fn parse(text: &'t str) -> Result<'t, Self> {
            Ok(text)
        }
    }

    macro_rules! impl_atom_from_fromstr {
        ($($t:ty) *) => { $(
            impl Atom<'_> for $t {
                fn parse(text: &'_ str) -> Result<'_, Self> {
                    text.parse().map_err(|_| ParseError(text))
                }
            }
        )* };
    }

    impl_atom_from_fromstr!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool char String);
    impl_atom_from_fromstr!(NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize);

    pub trait IterParse<'t>: Sized {
        fn parse_from_iter<'s, It>(it: &'s mut It) -> Result<'t, Self>
        where
            't: 's,
            It: Iterator<Item = &'t str>;
    }

    impl<'t, A> IterParse<'t> for A
    where
        A: Atom<'t>,
    {
        fn parse_from_iter<'s, It>(it: &'s mut It) -> Result<'t, Self>
        where
            't: 's,
            It: Iterator<Item = &'t str>,
        {
            it.next().map_or(Err(InputExhaust), <Self as Atom>::parse)
        }
    }

    impl<'t, A, const N: usize> IterParse<'t> for [A; N]
    where
        A: IterParse<'t>,
    {
        fn parse_from_iter<'s, It>(it: &'s mut It) -> Result<'t, Self>
        where
            't: 's,
            It: Iterator<Item = &'t str>,
        {
            use std::mem::*;
            let mut x: [MaybeUninit<A>; N] = unsafe { MaybeUninit::uninit().assume_init() };
            for p in x.iter_mut() {
                *p = MaybeUninit::new(A::parse_from_iter(it)?);
            }
            Ok(unsafe { transmute_copy(&x) })
        }
    }

    macro_rules! impl_iterparse_for_tuple {
        ($($t:ident) *) => {
            impl<'t, $($t),*> IterParse<'t> for ($($t),*) where $($t: IterParse<'t>),* {
                fn parse_from_iter<'s, It>(_it: &'s mut It) -> Result<'t, Self>
                where 't: 's, It: Iterator<Item = &'t str> {
                    Ok(($($t::parse_from_iter(_it)?),*))
                }
            }
        };
    }

    impl_iterparse_for_tuple!();
    impl_iterparse_for_tuple!(A B);
    impl_iterparse_for_tuple!(A B C);
    impl_iterparse_for_tuple!(A B C D);
    impl_iterparse_for_tuple!(A B C D E);
    impl_iterparse_for_tuple!(A B C D E F);
    impl_iterparse_for_tuple!(A B C D E F G);
    impl_iterparse_for_tuple!(A B C D E F G H);
    impl_iterparse_for_tuple!(A B C D E F G H I);
    impl_iterparse_for_tuple!(A B C D E F G H I J);
    impl_iterparse_for_tuple!(A B C D E F G H I J K);
}
