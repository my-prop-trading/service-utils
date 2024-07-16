pub trait DedupItem {
    fn get_key_str(&self) -> &str;
    fn get_name_str(&self) -> &str;
}
