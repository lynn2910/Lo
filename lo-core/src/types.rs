#[derive(Debug, Default, Eq, PartialEq)]
pub enum Type {
    #[default]
    Any,
    None,
    Int,
    Float,
    String
}