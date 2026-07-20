#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub arguments: Vec<String>,
}