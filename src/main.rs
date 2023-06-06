use clap::Parser;
use llm_chain::{executor, parameters, prompt};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    prompt: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

// Declare an async main function
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let exec = executor!()?;

    // Define the command-line interface using Clap
    let args = Args::parse();

    // Create our prompt...
    let res = prompt!(
        "You are chatGPT",
        &args.prompt
    )
    .run(&parameters!(), &exec) // ...and run it
    .await?;
    println!("{}", res);

    Ok(())
}

