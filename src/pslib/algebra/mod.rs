use core::ops::*;

/// Returns gcd(x, y).
pub(crate) fn gcd<T>(x: T, y: T) -> T
where
	T: Copy + PartialEq + PartialOrd + Rem<Output = T> + From<u8>,
{
	if y == 0.into() {
		x
	} else {
		let v = x % y;
		gcd(y, v)
	}
}

/// Returns lcm(x, y).
pub(crate) fn lcm<T>(x: T, y: T) -> T
where
	T: Copy + PartialEq + PartialOrd + Rem<Output = T> + Div<Output = T> + Mul<Output = T> + From<u8>,
{
	x / gcd(x, y) * y
}

/// `ext_gcd(a, b)` returns `(g, s, t)` such that `g == gcd(a, b)` and `a*s + b*t == g`.
pub(crate) fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
	let (mut s, mut old_s) = (0, 1);
	let (mut g, mut old_g) = (b, a);
	while g != 0 {
		let q = old_g / g;
		let (new_r, new_s) = (old_g - q * g, old_s - q * s);
		old_g = g; // Not using destructuring to support low version
		g = new_r; // AtCoder is using 1.42.0
		old_s = s;
		s = new_s;
	}

	(old_g, old_s, if b != 0 { (old_g - old_s * a) / b } else { 0 })
}

/// `crt(r, m)` returns `Some(x)` such that `x = r[i] mod m[i]` for all `i`. If such `x` doesn't
/// exist, then it returns `None`.
pub(crate) fn crt(r: &[i64], m: &[i64]) -> Option<i64> {
	let (mut x, mut m_prod) = (0, 1);
	for (bi, mi) in r.iter().zip(m.iter()) {
		let (g, s, _) = ext_gcd(m_prod, *mi);
		if ((bi - x) % mi).rem_euclid(g) != 0 {
			return None;
		}
		x += m_prod * ((s * ((bi - x).rem_euclid(*mi))).div_euclid(g));
		m_prod = (m_prod * mi).div_euclid(gcd(m_prod, *mi));
	}
	Some(x.rem_euclid(m_prod))
}

pub(crate) mod ntt;
pub(crate) mod prime;
