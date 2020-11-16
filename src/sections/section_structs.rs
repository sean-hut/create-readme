pub struct Section<'a> {
    pub flag: &'a str,
    pub append_message: &'a str,
    pub exclude_message: &'a str,
    pub content: &'a str,
}
