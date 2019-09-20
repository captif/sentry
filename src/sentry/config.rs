#[derive(Deserialize, Debug)]
pub struct Captif {
    pub url: String
}

#[derive(Deserialize, Debug)]
pub struct Genesis {
    pub captif: Option<Captif>
}
