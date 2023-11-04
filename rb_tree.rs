enum Color {
  Red,
  Black,
}

pub struct RBNode<K: Ord, V> {
  key: K,
  value: V,
  color: Color,
  parent: *mut RBNode<K, V>,
  left: *mut RBNode<K, V>,
  right: *mut RBNode<K, V>,
}

impl<K: Ord, V> RBNode<K, V> {
  fn new(key: K, value: V) -> RBNode<K, V> {
    RBNode {
      key,
      value,
      color: Color::Red,
      parent: null_mut(),
      left: null_mut(),
      right: null_mut(),
    }
  }
}

pub struct RBTree<K: Ord, V> {
  root: *mut RBNode<K, V>,
}


impl<K: Ord, V> RBTree<K, V> {
  pub fn new() -> RBTree<K, V> {
    RBTree::<K, V> { root: null_mut() }
  }

  
