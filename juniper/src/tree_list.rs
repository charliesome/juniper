pub enum TreeList<T> {
    Empty,
    Item(T),
    Append(Box<TreeList<T>>, Box<TreeList<T>>),
}

impl<T> TreeList<T> {
    pub fn empty() -> Self {
        TreeList::Empty
    }

    pub fn item(val: T) -> Self {
        TreeList::Item(val)
    }

    pub fn from_iter<I: Iterator<Item=T>>(other: I) -> Self {
        other.map(TreeList::Item).fold(TreeList::empty(), TreeList::append)
    }

    pub fn append(self, other: TreeList<T>) -> Self {
        match (self, other) {
            (TreeList::Empty, TreeList::Empty) => TreeList::Empty,
            (a, TreeList::Empty) | (TreeList::Empty, a) => a,
            (a, b) => TreeList::Append(Box::new(a), Box::new(b)),
        }
    }

    #[allow(unused)]
    pub fn iter<'a>(&'a self) -> TreeListIterator<'a, T> {
        TreeListIterator { stack: vec![self] }
    }
}

impl<T> IntoIterator for TreeList<T> {
    type Item = T;
    type IntoIter = TreeListIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeListIntoIterator { stack: vec![self] }
    }
}

pub struct TreeListIntoIterator<T> {
    stack: Vec<TreeList<T>>,
}

impl<T> Iterator for TreeListIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.stack.pop() {
            match item {
                TreeList::Empty => {
                    continue;
                }
                TreeList::Item(val) => {
                    return Some(val);
                }
                TreeList::Append(left, right) => {
                    self.stack.push(*right);
                    self.stack.push(*left);
                }
            }
        }

        None
    }
}

#[allow(unused)]
pub struct TreeListIterator<'a, T: 'a> {
    stack: Vec<&'a TreeList<T>>,
}

#[allow(unused)]
impl<'a, T> Iterator for TreeListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.stack.pop() {
            match *item {
                TreeList::Empty => {
                    continue;
                }
                TreeList::Item(ref val) => {
                    return Some(val);
                }
                TreeList::Append(ref left, ref right) => {
                    self.stack.push(right);
                    self.stack.push(left);
                }
            }
        }

        None
    }
}
