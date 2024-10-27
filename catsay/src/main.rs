use std::io::{self, Read};
use colored::Colorize;
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Options{
    #[clap(default_value = "Meow!")]
    /// What does the cat say ?
    message: String,
    #[clap(short = 'd', long = "dead" )]
    /// Make the cat appear dead
    dead: bool,
    #[clap(short = 'i', long = "stdin")]
    /// Read the message from the stdin instead of the argument
    stdin: bool,
    #[clap(short = 'f', long = "file" )]
    /// Upload the cat picture from a file
    catfile: Option<std::path::PathBuf>
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();
    if options.stdin{
        io::stdin().read_to_string(&mut message)?;
    }
    else{
        message = options.message;
    };
    let eye = if options.dead {"x"} else {"o"};
    let eye = format!("{}", eye.red().bold());
    if message.to_lowercase() == "woof"{
        eprintln!("A cat can't bark like a dog.")
    }

    match &options.catfile{
        Some(path) => {
            let cat_template = std::fs::read_to_string(path).with_context(
                || format!("Could not read file {:?}", path)
            )?;
            let cat_template = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", cat_template);
        },
        None => {      
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!(" \\");
            println!(" /\\_/\\");
            println!(" ( {eye} {eye} )");
            println!(" =( I )=");  
        }
    }
    Ok(())
}