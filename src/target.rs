


#[derive(Clone, Debug)]
pub enum GMTarget {
    Object(String),
    Group(String),
    ObjectManager,
    Multiple(Vec<GMTarget>),
}

impl GMTarget {
    pub fn chain(self: Self, other: Self) -> Self {
        match self {
            Self::Multiple(mut list1) => {
                match other {
                    Self::Multiple(list2) => {
                        list1.extend(list2);
                        GMTarget::Multiple(list1)
                    }
                    _ => {
                        list1.push(other);
                        GMTarget::Multiple(list1)
                    }
                }
            }
            _ => {
                match other {
                    Self::Multiple(list2) => {
                        let mut list1 = vec![self];
                        list1.extend(list2);
                        GMTarget::Multiple(list1)
                    }
                    _ => {
                        let list = vec![self, other];
                        GMTarget::Multiple(list)
                    }
                }
            }
        }
    }
}

impl From<&str> for GMTarget {
    fn from(value: &str) -> Self {
        Self::Object(value.to_string())
    }
}

impl From<String> for GMTarget {
    fn from(value: String) -> Self {
        Self::Object(value)
    }
}

impl From<&[&str]> for GMTarget {
    fn from(value: &[&str]) -> Self {
        let targets: Vec<GMTarget> = value.iter().map(|s| GMTarget::from(*s)).collect();
        Self::Multiple(targets)
    }
}

impl From<(&str, &str)> for GMTarget {
    fn from((a, b): (&str, &str)) -> Self {
        let targets = vec![GMTarget::from(a), GMTarget::from(b)];
        Self::Multiple(targets)
    }
}
