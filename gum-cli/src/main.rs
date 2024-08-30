use clap::{Arg, Command};
use gum_core::bootstrap::Bootstrapper;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("myapp")
        .version("1.0")
        .about("A simple CLI app using clap")
        .subcommand(
            Command::new("watch")
                .about("Watches for changes"),
        )
        .subcommand(
            Command::new("init")
                .about("Initializes something")
                .alias("i") // Alias for init
                .arg(
                    Arg::new("name")
                        .required(true),
                ),
        )
        .get_matches();

    // Handle the matches
    match matches.subcommand() {
        Some(("watch", _)) => {
            println!("Watching for changes...");
        }
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").expect("Name is required");

            Bootstrapper::new_from_named(name)
                .strap()
                .await?;

            println!("Initializing with name: {}", name);
        }
        _ => {
            println!("No valid command was provided. Use 'gum help' for more information.");
        }
    }

    Ok(())
}
