use compact_str::CompactString;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct DedupItemKey(pub CompactString);

impl Deref for DedupItemKey {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0.as_str()
    }
}

impl From<&str> for DedupItemKey {
    fn from(value: &str) -> Self {
        DedupItemKey(value.into())
    }
}

impl From<String> for DedupItemKey {
    fn from(value: String) -> Self {
        DedupItemKey(value.into())
    }
}

impl From<CompactString> for DedupItemKey {
    fn from(value: CompactString) -> Self {
        DedupItemKey(value)
    }
}

impl From<&CompactString> for DedupItemKey {
    fn from(value: &CompactString) -> Self {
        DedupItemKey(value.to_owned())
    }
}

impl From<&String> for DedupItemKey {
    fn from(value: &String) -> Self {
        DedupItemKey(value.into())
    }
}

impl Display for DedupItemKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
