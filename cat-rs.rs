// tiny cat rs

use std::{
    fs,
    io::{self, stdout, Read, Write},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser)]
struct App {
    /// ...
    #[arg(long = "belnstuv", short = 'b')]
    belnstuv: bool,

    /// ...
    file: PathBuf,
}

fn main() -> io::Result<()> {
    let app = App::parse();

    let mut buf = String::new();
    fs::File::open(app.file)?.read_to_string(&mut buf)?;

    let lines = buf.split('\n').collect::<Vec<&str>>();
    let lines_iter = lines.iter().enumerate();

    for line in lines_iter {
        let string = if app.belnstuv {
            format!("{} {}\n", line.0, line.1)
        } else {
            format!("{}\n", line.1)
        };
        stdout().write(string.as_bytes())?;
    }

    Ok(())
}
