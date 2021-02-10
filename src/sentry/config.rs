#[derive(Deserialize, Debug)]
pub struct Captif {
    pub url: String,
    pub expires: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Genesis {
    pub captif: Option<Captif>
}
