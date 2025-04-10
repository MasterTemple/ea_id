#![doc = include_str!("../README.md")]
use clap::Parser;
use cli::Cli;
use env::{load_env, update_env};
use origin::OriginApi;

pub mod cli;
pub mod env;
pub mod origin;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    load_env();

    let cli = Cli::parse();

    let mut api = if let Some(token) = cli.access_token {
        OriginApi::from_cookies_and_token(cli.remid, cli.sid, token)?
    } else {
        OriginApi::from_cookies(cli.remid, cli.sid)?
    };

    update_env(&api);

    if let Some(id) = cli.identifier.id {
        let user = api.get_user_from_id(&id)?;
        user.print_table();
    }

    if let Some(name) = cli.identifier.name {
        let user = api.get_user_from_name(&name)?;
        user.print_table();
    }

    Ok(())
}
