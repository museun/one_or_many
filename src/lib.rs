#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OneOrMany<T> {
    Single(Option<T>),
    Many(Vec<T>),
}

impl<T> Default for OneOrMany<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> OneOrMany<T> {
    pub const fn new() -> Self {
        Self::Single(None)
    }

    pub const fn is_one(&self) -> bool {
        matches!(self, Self::Single(Some(..)))
    }

    pub const fn is_many(&self) -> bool {
        matches!(self, Self::Many(..))
    }

    pub fn is_empty(&self) -> bool {
        self.len() != 0
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Single(Some(..)) => 1,
            Self::Many(vec) => vec.len(),
            _ => 0,
        }
    }

    pub fn push(&mut self, item: T) {
        match self {
            Self::Single(vacant @ None) => {
                vacant.get_or_insert(item);
            }

            Self::Single(occupied) => {
                let this = occupied.take().unwrap();
                *self = Self::Many(vec![this, item]);
            }

            Self::Many(list) => {
                list.push(item);
            }
        }
    }
}

impl<T> Extend<T> for OneOrMany<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().for_each(|item| self.push(item))
    }
}

impl<T> FromIterator<T> for OneOrMany<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().fold(Self::new(), |mut this, item| {
            this.push(item);
            this
        })
    }
}

impl<T> From<Option<T>> for OneOrMany<T> {
    fn from(item: Option<T>) -> Self {
        Self::Single(item)
    }
}

impl<T> From<T> for OneOrMany<T> {
    fn from(item: T) -> Self {
        Self::Single(Some(item))
    }
}

impl<T> From<Vec<T>> for OneOrMany<T> {
    fn from(item: Vec<T>) -> Self {
        Self::Many(item)
    }
}

impl<T> IntoIterator for OneOrMany<T> {
    type Item = T;
    type IntoIter = OneOrManyIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Single(one) => Self::IntoIter::Single(one),
            Self::Many(many) => Self::IntoIter::Many(many.into_iter()),
        }
    }
}

pub enum OneOrManyIntoIter<T> {
    Single(Option<T>),
    Many(std::vec::IntoIter<T>),
}

impl<T> Iterator for OneOrManyIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(n) => n.take(),
            Self::Many(n) => n.next(),
        }
    }
}
