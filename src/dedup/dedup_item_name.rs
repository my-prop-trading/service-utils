use compact_str::CompactString;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct DedupItemName(pub CompactString);

impl Deref for DedupItemName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0.as_str()
    }
}

impl From<&str> for DedupItemName {
    fn from(value: &str) -> Self {
        DedupItemName(value.into())
    }
}

impl From<String> for DedupItemName {
    fn from(value: String) -> Self {
        DedupItemName(value.into())
    }
}

impl From<CompactString> for DedupItemName {
    fn from(value: CompactString) -> Self {
        DedupItemName(value)
    }
}

impl From<&CompactString> for DedupItemName {
    fn from(value: &CompactString) -> Self {
        DedupItemName(value.to_owned())
    }
}

impl From<&String> for DedupItemName {
    fn from(value: &String) -> Self {
        DedupItemName(value.into())
    }
}

impl Display for DedupItemName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
