extern crate clap;
// Allowing this for now as we will need SubCommand shortly.
#[allow(unused_imports)]
use clap::{Arg, App, SubCommand};

fn main() {
    // This parses the command-line arguments. It is here we can
    // add subcommands and flags along with help messages and so forth.
    let matches = App::new("Slack it to me")
        .version("0.0.1")
        .author("Alex Afshar, Brant Goddard")
        .about("Connects to Slack from CLI")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    // Here we pull out the value passed in by the user for the config, or
    // use a default `sitm.conf`.
    let config = matches.value_of("config").unwrap_or("sitm.conf");
    println!("Config file to be used: {}", config);

    println!("All done, for now!");
}
