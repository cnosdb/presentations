pub struct RadixNode<T> {
    key: Vec<u8>,
    childs: Vec<Box<RadixNode<T>>>,
    val: Option<T>,
}

impl<T> Default for RadixNode<T> {
    fn default() -> Self {
        Self {
            key: vec![],
            childs: vec![],
            val: None,
        }
    }
}

impl<T> RadixNode<T> {
    pub fn new() -> Self {
        Self {
            key: vec![],
            childs: vec![],
            val: None,
        }
    }

    pub fn insert(&mut self, key: &[u8], val: T) {
        for child in self.childs.iter_mut() {
            let prefix_len = longgest_prefix_len(&child.key, key);
            if prefix_len == 0 {
                continue;
            }
            let prefix = &key[..prefix_len];
            let new_child_key = &child.key[prefix_len..];
            if new_child_key.is_empty() {
                let new_key = &key[prefix_len..];
                if new_key.is_empty() {
                    child.val = Some(val);
                } else {
                    child.insert(new_key, val);
                };
                return;
            } else {
                child.key = new_child_key.to_owned();
                let mut new_node = RadixNode {
                    key: prefix.to_owned(),
                    childs: vec![std::mem::take(child)],
                    val: None,
                };
                let new_key = &key[prefix_len..];
                if new_key.is_empty() {
                    new_node.val = Some(val);
                } else {
                    new_node.insert(new_key, val);
                }
                *child = Box::new(new_node);
                return;
            }
        }
        let new_node = RadixNode {
            key: key.to_owned(),
            childs: vec![],
            val: Some(val),
        };
        self.childs.push(Box::new(new_node));
    }

    pub fn get(&self, key: &[u8]) -> Option<&T> {
        for child in &self.childs {
            if key.starts_with(&child.key) {
                let remain = &key[child.key.len()..];
                if remain.is_empty() {
                    return child.val.as_ref();
                }
                return child.get(remain);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &[u8]) {
        let mut remove_idx = None;
        for (idx, child) in self.childs.iter_mut().enumerate() {
            if key.starts_with(&child.key) {
                let remain = &key[child.key.len()..];
                if remain.is_empty() {
                    if child.val.is_some() {
                        child.val = None;
                        if child.childs.is_empty() {
                            remove_idx = Some(idx);
                        } else if child.childs.len() == 1 {
                            child.merge_with_child();
                        }
                    }
                } else {
                    child.remove(remain);
                }
                break;
            }
        }
        if let Some(idx) = remove_idx {
            self.childs.remove(idx);
            if self.childs.len() == 1 && self.val.is_none() && !self.key.is_empty() {
                self.merge_with_child();
            }
        }
    }

    fn merge_with_child(&mut self) {
        self.key.append(&mut self.childs[0].key);
        self.val = self.childs[0].val.take();
        self.childs = std::mem::take(&mut self.childs[0].childs);
    }
}

fn longgest_prefix_len(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter().zip(s2.iter()).take_while(|c| c.0 == c.1).count()
}
