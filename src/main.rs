use clap::{App };
mod config;

type AppResult<T> = Result<T, std::boxed::Box<dyn std::error::Error>>;

fn main() -> AppResult<()> {
    let clap_matches = App::new("rust-server-stats-gatherer")
        .version("0.1")
        .author("Alessandro Menezes <alessandroasm@gmail.com>")
        .about("This application gathers stats from several hosts using SSH")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
            -v                   'Verbose mode'",
        )
        .get_matches();

    let config_file = clap_matches.value_of("config").unwrap_or("config.yaml");
    config::load_config_file(config_file)?;

    Ok(())
}
