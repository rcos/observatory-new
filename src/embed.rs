use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "client/dist"]
pub struct Embed;
