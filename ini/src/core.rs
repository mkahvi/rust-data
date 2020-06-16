/// Core INI container.
pub struct Ini
{
	/// Number of changes made since last save/flush.
	changes: u64,
}

impl Ini {
    pub fn  new() -> Self { Self { changes: 0 } }
}

impl mkah_traits::Flush for Ini {
	/*
	This honestly should save the Ini first, but if Ini itself is not Saveable by itself, how would that go about?
	*/
	fn flush(&mut self) { self.changes = 0; }
}
