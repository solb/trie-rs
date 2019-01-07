use trie::TrieSet;

fn set(set: &mut TrieSet<&'static str>) {
	assert!(set.is_empty());
	assert!(! set.contains(""));
	assert!(! set.contains("foobar"));

	assert!(set.insert("foobar"));
	assert!(! set.is_empty());
	assert!(set.len() == 1);
	assert!(set.contains("foobar"));

	assert!(set.insert("bar"));
	assert!(! set.is_empty());
	assert!(set.len() == 2);
	assert!(set.contains("bar"));
	assert!(set.contains("foobar"));

	assert!(set.insert("foo"));
	assert!(! set.is_empty());
	assert!(set.len() == 3);
	assert!(set.contains("bar"));
	assert!(set.contains("foo"));
	assert!(set.contains("foobar"));

	assert!(set.insert("foobarbaz"));
	assert!(! set.is_empty());
	assert!(set.len() == 4);
	assert!(set.contains("bar"));
	assert!(set.contains("foo"));
	assert!(set.contains("foobar"));
	assert!(set.contains("foobarbaz"));

	assert!(set.insert(""));
	assert!(! set.is_empty());
	assert!(set.len() == 5);
	assert!(set.contains(""));
	assert!(set.contains("bar"));
	assert!(set.contains("foo"));
	assert!(set.contains("foobar"));
	assert!(set.contains("foobarbaz"));

	assert!(! set.insert("foobar"));
	assert!(! set.is_empty());
	assert!(set.len() == 5);
	assert!(set.contains(""));
	assert!(set.contains("bar"));
	assert!(set.contains("foo"));
	assert!(set.contains("foobar"));
	assert!(set.contains("foobarbaz"));
}

#[test]
fn set_insert_contains() {
	set(&mut TrieSet::new());
}

#[test]
fn set_clear() {
	let mut forget = TrieSet::new();
	set(&mut forget);
	forget.clear();
	set(&mut forget);
}

#[test]
fn set_iter() {
	let mut trie = TrieSet::new();
	set(&mut trie);

	let mut elems: Vec<_> = trie.iter().collect();
	elems.sort();
	assert_eq!(&[&"", &"bar", &"foo", &"foobar", &"foobarbaz"], &*elems);
}

#[test]
fn set_iter_prefix() {
	let mut trie = TrieSet::new();
	set(&mut trie);

	let mut elems: Vec<_> = trie.iter_prefix("").collect();
	elems.sort();
	assert_eq!(&[&"", &"bar", &"foo", &"foobar", &"foobarbaz"], &*elems);

	let mut elems: Vec<_> = trie.iter_prefix("f").collect();
	elems.sort();
	assert_eq!(&[&"foo", &"foobar", &"foobarbaz"], &*elems);

	let mut elems: Vec<_> = trie.iter_prefix("fo").collect();
	elems.sort();
	assert_eq!(&[&"foo", &"foobar", &"foobarbaz"], &*elems);

	let mut elems: Vec<_> = trie.iter_prefix("foo").collect();
	elems.sort();
	assert_eq!(&[&"foo", &"foobar", &"foobarbaz"], &*elems);

	let mut elems: Vec<_> = trie.iter_prefix("foob").collect();
	elems.sort();
	assert_eq!(&[&"foobar", &"foobarbaz"], &*elems);

	let mut elems: Vec<_> = trie.iter_prefix("fooba").collect();
	elems.sort();
	assert_eq!(&[&"foobar", &"foobarbaz"], &*elems);

	let mut elems: Vec<_> = trie.iter_prefix("foobar").collect();
	elems.sort();
	assert_eq!(&[&"foobar", &"foobarbaz"], &*elems);

	let mut elems: Vec<_> = trie.iter_prefix("foobarb").collect();
	elems.sort();
	assert_eq!(&[&"foobarbaz"], &*elems);

	let mut elems: Vec<_> = trie.iter_prefix("foobarba").collect();
	elems.sort();
	assert_eq!(&[&"foobarbaz"], &*elems);

	let mut elems: Vec<_> = trie.iter_prefix("foobarbaz").collect();
	elems.sort();
	assert_eq!(&[&"foobarbaz"], &*elems);

	let elems: Vec<_> = trie.iter_prefix("foobarbazz").collect();
	assert!(elems.is_empty());
}
