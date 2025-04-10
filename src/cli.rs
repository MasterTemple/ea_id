use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// REMID, required (env or CLI)
    #[arg(long, env = "REMID")]
    pub remid: String,

    /// SID, required (env or CLI)
    #[arg(long, env = "SID")]
    pub sid: String,

    /// ACCESS_TOKEN, optional (env or CLI)
    #[arg(long, env = "ACCESS_TOKEN")]
    pub access_token: Option<String>,

    /// One of `name` or `id` must be provided
    #[command(flatten)]
    pub identifier: Identifier,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Identifier {
    /// Provide the user's name
    #[arg(long)]
    pub name: Option<String>,

    /// Provide the user's ID
    #[arg(long)]
    pub id: Option<String>,
}
