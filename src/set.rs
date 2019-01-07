use crate::map::Iter;
use crate::map::TrieMap;
use std::borrow::Borrow;
use std::hash::Hash;

pub struct TrieSet<T, A = u8> (TrieMap<T, T, A>);

impl<A: Clone + Eq + Hash, T: AsRef<[A]> + Clone> TrieSet<T, A> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn clear(&mut self) {
		let TrieSet (this) = self;
		this.clear();
	}

	pub fn contains<S: AsRef<[A]> + ?Sized>(&self, elem: &S) -> bool
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

	pub fn iter_prefix<S: AsRef<[A]> + ?Sized>(&self, prefix: &S) -> Iter<&T>
	where T: Borrow<S> {
		let TrieSet (this) = self;
		this.iter_prefix(prefix)
	}
}

impl<T, A: Clone + Eq + Hash> Default for TrieSet<T, A> {
	fn default() -> Self {
		TrieSet (TrieMap::default())
	}
}
