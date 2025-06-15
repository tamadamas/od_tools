use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod generator;
use generator::GameLogGenerator;

#[derive(Parser, Debug)]
#[command(name = "od_tools", author, version, about = "The Open Dominion Game tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates a game log from a sim file
    GenerateLog {
        /// Path to the sim file (e.g., sim.xlsm)
        #[arg(short, long, value_name = "FILE")]
        sim: PathBuf,

        /// Path to the result file. Prints to stdout if not provided.
        #[arg(short, long, value_name = "FILE")]
        result: Option<PathBuf>,

        /// Set a specific hour to process
        #[arg(long)]
        hour: Option<usize>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateLog { sim, result, hour } => {
            println!("Generating log for sim file: {:?}", sim);

            let mut generator = GameLogGenerator::new(&sim)?;
            let output = generator.execute(hour)?;

            if let Some(result_path) = result {
                std::fs::write(&result_path, output)?;
                println!("Successfully wrote result to {:?}", result_path);
            } else {
                println!("{}", output);
            }
        }
    }

    Ok(())
}
