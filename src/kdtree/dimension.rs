
#[derive(Clone, PartialEq)]
pub enum Dimension {
    X,
    Y,
    Z,
    Null,
}
impl Dimension {
    pub fn as_string(&self) -> &str {
        match self {
            &Dimension::X => return "X",
            &Dimension::Y => return "Y",
            &Dimension::Z => return "Z",
            _ => return "Null",
        }
    }
}
