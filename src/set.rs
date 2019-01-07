use crate::map::Iter;
use crate::map::Key;
use crate::map::TrieMap;
use std::borrow::Borrow;

pub struct TrieSet<T> (TrieMap<T, T>);

impl<T: AsRef<[Atom]> + Clone> TrieSet<T> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn clear(&mut self) {
		let TrieSet (this) = self;
		this.clear();
	}

	pub fn contains<S: AsRef<[Atom]> + ?Sized>(&self, elem: &S) -> bool
	where T: Borrow<S> {
		let TrieSet (this) = self;
		this.contains(elem)
	}

	pub fn insert(&mut self, elem: T) -> bool {
		let TrieSet (this) = self;
		this.insert(&elem.clone(), elem).is_none()
	}

	pub fn iter(&self) -> Iter<&T> {
		let TrieSet (this) = self;
		this.iter()
	}

	pub fn iter_prefix<S: AsRef<[Atom]> + ?Sized>(&self, prefix: &S) -> Iter<&T>
	where T: Borrow<S> {
		let TrieSet (this) = self;
		this.iter_prefix(prefix)
	}
}

impl<T> Default for TrieSet<T> {
	fn default() -> Self {
		TrieSet (TrieMap::default())
	}
}

pub type Atom = Key;
