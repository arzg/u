use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() -> anyhow::Result<()> {
    let mut files: Vec<_> = get_files().collect();
    let mut stdout = io::stdout();

    for line in io::stdin().lock().lines() {
        let line = line?;

        for file in &mut files {
            writeln!(file, "{}", line)?;
        }
        writeln!(stdout, "{}", line)?;
    }

    Ok(())
}

fn get_files() -> impl Iterator<Item = File> {
    env::args().skip(1).filter_map(|path| {
        File::create(&path).map_or_else(
            |e| {
                eprintln!(
                    "Error: {:?}",
                    anyhow::Error::new(e)
                        .context(format!("failed to create file at path ‘{}’", path))
                );
                None
            },
            Some,
        )
    })
}
