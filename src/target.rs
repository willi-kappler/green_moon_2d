


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
            Self::Multiple(mut left_targets) => {
                match other {
                    Self::Multiple(right_targets) => {
                        left_targets.extend(right_targets);
                        left_targets.into()
                    }
                    _ => {
                        left_targets.push(other);
                        left_targets.into()
                    }
                }
            }
            _ => {
                match other {
                    Self::Multiple(right_targets) => {
                        let mut left_targets = vec![self];
                        left_targets.extend(right_targets);
                        left_targets.into()
                    }
                    _ => {
                        vec![self, other].into()
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

impl From<Vec<GMTarget>> for GMTarget {
    fn from(targets: Vec<GMTarget>) -> Self {
        GMTarget::Multiple(targets)
    }
}
