#[derive(Clone)]
pub struct StackStructure {
    pub(crate) logo: Vec<&'static str>,
    pub(crate) title: &'static str,
    pub(crate) description: &'static str,
}