use std::borrow::Borrow;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;

pub struct TrieMap<K, V> (HashMap<Key, Val<K, V>>);

impl<K, V> TrieMap<K, V> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn clear(&mut self) {
		let TrieMap (this) = self;
		this.clear();
	}

	pub fn contains<T: AsRef<[Key]> + ?Sized>(&self, key: &T) -> bool
	where K: Borrow<T> {
		self.get(key).is_some()
	}

	pub fn insert<T: AsRef<[Key]> + ?Sized>(&mut self, key: &T, val: V) -> Option<V>
	where K: Borrow<T> {
		self.insert_helper(key.as_ref(), val)
	}

	pub fn get<T: AsRef<[Key]> + ?Sized>(&self, key: &T) -> Option<&V>
	where K: Borrow<T> {
		self.get_helper(key.as_ref())
	}

	pub fn get_mut<T: AsRef<[Key]> + ?Sized>(&mut self, key: &T) -> Option<&mut V>
	where K: Borrow<T> {
		self.get_mut_helper(key.as_ref())
	}

	fn insert_helper(&mut self, key: &[Key], val: V) -> Option<V> {
		use std::collections::hash_map::Entry;
		use std::mem::replace;

		let TrieMap (this) = self;
		assert!(key.len() != 0, "key must be nonempty");
		match this.entry(key[0]) {
			Entry::Occupied(entry) => match entry.into_mut() {
				Val::Leaf(entry) => Some(replace(entry, val)),
				Val::Branch(entry) => entry.insert_helper(&key[1..], val),
			},
			Entry::Vacant(entry) => {
				if key.len() == 1 {
					entry.insert(Val::Leaf(Leaf::from(val)));
				} else {
					let mut subtrie = Branch::default();
					assert!(subtrie.insert_helper(&key[1..], val).is_none());
					entry.insert(Val::Branch(subtrie));
				}
				None
			},
		}
	}

	fn get_helper(&self, key: &[Key]) -> Option<&V> {
		let TrieMap (this) = self;
		match this.get(key.get(0)?)? {
			Val::Leaf(val) => Some(val),
			Val::Branch(subtrie) => subtrie.get_helper(&key[1..]),
		}
	}

	fn get_mut_helper(&mut self, key: &[Key]) -> Option<&mut V> {
		let TrieMap (this) = self;
		match this.get_mut(key.get(0)?)? {
			Val::Leaf(val) => Some(val),
			Val::Branch(subtrie) => subtrie.get_mut_helper(&key[1..]),
		}
	}
}

impl<K, V> Default for TrieMap<K, V> {
	fn default() -> Self {
		TrieMap (HashMap::default())
	}
}

impl<T: AsRef<[Key]> + ?Sized, K: Borrow<T>, V> Index<&T> for TrieMap<K, V> {
	type Output = V;

	fn index(&self, key: &T) -> &Self::Output {
		self.get(key).unwrap()
	}
}

impl<T: AsRef<[Key]> + ?Sized, K: Borrow<T>, V> IndexMut<&T> for TrieMap<K, V> {
	fn index_mut(&mut self, key: &T) -> &mut Self::Output {
		self.get_mut(key).unwrap()
	}
}

type Key = u8;

enum Val<K, V> {
	Leaf(Leaf<K, V>),
	Branch(Branch<K, V>),
}

struct Leaf<K, V> (PhantomData<K>, V);

impl<K, V> Deref for Leaf<K, V> {
	type Target = V;

	fn deref(&self) -> &Self::Target {
		let Leaf (_, val) = self;
		val
	}
}

impl<K, V> DerefMut for Leaf<K, V> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		let Leaf (_, val) = self;
		val
	}
}

impl<K, V> From<V> for Leaf<K, V> {
	fn from(val: V) -> Self {
		Leaf (PhantomData::default(), val)
	}
}

type Branch<K, V> = TrieMap<K, V>;
