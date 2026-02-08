#[derive(Clone, Copy)]
pub struct Recommandation {
    pub src: &'static str,
    pub author: &'static str,
    pub texte: Option<&'static str>,
}