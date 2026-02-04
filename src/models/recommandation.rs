#[derive(Clone, Copy)]
pub struct Recommandation {
    pub id: i32,
    pub src: &'static str,
    pub author: &'static str,
    pub recommendation: Option<&'static str>,
}