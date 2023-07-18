pub(crate) mod rng {
	pub struct RNG {
		val: u64,
	}

	impl RNG {
		pub fn new(seed: u64) -> Self { Self { val: seed } }
		pub fn next_u64(&mut self) -> u64 {
			let mut x = self.val;
			x ^= x << 13;
			x ^= x >> 7;
			x ^= x << 17;
			self.val = x;
			x
		}
	}
}
