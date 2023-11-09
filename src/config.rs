#[derive(clap::Parser, Debug)]
pub struct Config {
    #[clap(long, env)]
    pub smtp_host: String,
    #[clap(long, env)]
    pub smtp_username: String,
    #[clap(long, env)]
    pub smtp_password: String,
    #[clap(long, env)]
    pub smtp_from: String,
    #[clap(long, env)]
    pub mail_to: String,
    #[clap(long, env)]
    pub subject: String,
    #[clap(long, env)]
    pub body: String,
}
