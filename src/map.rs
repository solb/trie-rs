use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::Index;
use std::ops::IndexMut;

pub struct TrieMap<K, V, A = u8> {
	_key: PhantomData<K>,
	val: Option<V>,
	subtrie: HashMap<A, TrieMap<K, V, A>>,
}

impl<K, V, A: Clone + Eq + Hash> TrieMap<K, V, A> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn clear(&mut self) {
		self.val.take();
		self.subtrie.clear();
	}

	pub fn contains<T: AsRef<[A]> + ?Sized>(&self, key: &T) -> bool
	where K: Borrow<T> {
		self.get(key).is_some()
	}

	pub fn get<T: AsRef<[A]> + ?Sized>(&self, key: &T) -> Option<&V>
	where K: Borrow<T> {
		self.get_helper(key.as_ref())
	}

	pub fn get_mut<T: AsRef<[A]> + ?Sized>(&mut self, key: &T) -> Option<&mut V>
	where K: Borrow<T> {
		self.get_mut_helper(key.as_ref())
	}

	pub fn insert<T: AsRef<[A]> + ?Sized>(&mut self, key: &T, val: V) -> Option<V>
	where K: Borrow<T> {
		self.insert_helper(key.as_ref(), val)
	}

	pub fn iter(&self) -> Iter<&V> {
		self.iter_helper(&[])
	}

	pub fn iter_mut(&mut self) -> Iter<&mut V> {
		self.iter_mut_helper(&[])
	}

	pub fn iter_prefix<T: AsRef<[A]> + ?Sized>(&self, prefix: &T) -> Iter<&V>
	where K: Borrow<T> {
		self.iter_helper(prefix.as_ref())
	}

	pub fn iter_prefix_mut<T: AsRef<[A]> + ?Sized>(&mut self, prefix: &T) -> Iter<&mut V>
	where K: Borrow<T> {
		self.iter_mut_helper(prefix.as_ref())
	}

	fn get_helper(&self, key: &[A]) -> Option<&V> {
		if let Some(atom) = key.first() {
			self.subtrie.get(atom)?.get_helper(&key[1..])
		} else {
			self.val.as_ref()
		}
	}

	fn get_mut_helper(&mut self, key: &[A]) -> Option<&mut V> {
		if let Some(atom) = key.first() {
			self.subtrie.get_mut(atom)?.get_mut_helper(&key[1..])
		} else {
			self.val.as_mut()
		}
	}

	fn insert_helper(&mut self, key: &[A], val: V) -> Option<V> {
		if let Some(atom) = key.first().cloned() {
			self.subtrie.entry(atom).or_default().insert_helper(&key[1..], val)
		} else {
			self.val.replace(val)
		}
	}

	fn iter_helper(&self, key: &[A]) -> Iter<&V> {
		use std::iter;

		if let Some(atom) = key.first() {
			if let Some(subtrie) = self.subtrie.get(atom) {
				subtrie.iter_helper(&key[1..])
			} else {
				Box::new(iter::empty())
			}
		} else {
			Box::new(self.val.iter().chain(self.subtrie.iter().flat_map(|(_, subtrie)|
				subtrie.iter_helper(&[]))
			))
		}
	}

	fn iter_mut_helper(&mut self, key: &[A]) -> Iter<&mut V> {
		use std::iter;

		if let Some(atom) = key.first() {
			if let Some(subtrie) = self.subtrie.get_mut(atom) {
				subtrie.iter_mut_helper(&key[1..])
			} else {
				Box::new(iter::empty())
			}
		} else {
			Box::new(self.val.iter_mut().chain(self.subtrie.iter_mut().flat_map(|(_, subtrie)|
				subtrie.iter_mut_helper(&[]))
			))
		}
	}
}

impl<K, V, A: Clone + Eq + Hash> Default for TrieMap<K, V, A> {
	fn default() -> Self {
		Self {
			_key: PhantomData::default(),
			val: None,
			subtrie: HashMap::default(),
		}
	}
}

impl<A: Clone + Eq + Hash, T: AsRef<[A]> + ?Sized, K: Borrow<T>, V> Index<&T> for TrieMap<K, V, A> {
	type Output = V;

	fn index(&self, key: &T) -> &Self::Output {
		self.get(key).unwrap()
	}
}

impl<A: Clone + Eq + Hash, T: AsRef<[A]> + ?Sized, K: Borrow<T>, V> IndexMut<&T> for TrieMap<K, V, A> {
	fn index_mut(&mut self, key: &T) -> &mut Self::Output {
		self.get_mut(key).unwrap()
	}
}

pub type Iter<'a, T> = Box<dyn FusedIterator<Item = T> + 'a>;
