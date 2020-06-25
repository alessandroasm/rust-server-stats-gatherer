use clap::{App, Arg};

fn main() {
    let clap_matches = App::new("rust-server-stats-gatherer")
        .version("0.1")
        .author("Alessandro Menezes <alessandroasm@gmail.com>")
        .about("This application gathers stats from several hosts using SSH")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
            -v                   'Verbose mode'",
        )
        .get_matches();

    println!("Matches: {:?}", &clap_matches);
}
