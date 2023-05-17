
#[derive(Clone, Debug)]
pub enum GMTarget {
    Single(String),
    Multiple(Vec<String>),
    Group(String),
    MultipleGroups(Vec<String>),
    ObjectManager,
}

impl From<&str> for GMTarget {
    fn from(value: &str) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<String> for GMTarget {
    fn from(value: String) -> Self {
        Self::Single(value)
    }
}

impl From<&[&str]> for GMTarget {
    fn from(value: &[&str]) -> Self {
        let vec = value.to_vec();
        let vec2: Vec<String> = vec.iter().map(|s| s.to_string()).collect();
        Self::Multiple(vec2)
    }
}

impl From<(&str, &str)> for GMTarget {
    fn from((a, b): (&str, &str)) -> Self {
        let vec = vec![a.to_string(), b.to_string()];
        Self::Multiple(vec)
    }
}
