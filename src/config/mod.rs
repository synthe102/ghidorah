use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "Ghidora",
    version = "0.1.0",
    author = "Léonard Suslian (@synthe102)"
)]
pub struct Args {
    #[arg(short, long, default_value = "./auth/cert.pem")]
    pub cert: String,
    #[arg(short, long, default_value = "./auth/key.pem")]
    pub key: String,
    #[arg(long)]
    pub control_plane_url: String,
}
