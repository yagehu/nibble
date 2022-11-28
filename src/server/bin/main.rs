use std::{fs::File, net::TcpListener};

use color_eyre::eyre::Error as E;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), E> {
    color_eyre::install()?;
    logging::init("NIBBLE_LOG")?;

    let config_file = File::open("config.yaml")?;
    let config: Config = serde_yaml::from_reader(config_file)?;

    let listener = TcpListener::bind("0.0.0.0:8000")?;

    Ok(server::spawn(listener, config)?.await?)
}
