use clap::Parser;

#[derive(Parser)]
#[command(name = "Cyclon")]
#[command(version = "0.0.1")]
#[command(author = "Kazuki Kijima <kazukiyoshida96@gmail.com>")]
#[command(about = "Check cyclomatic complexity")]
struct Args {
    #[arg(index(1))]
    path: String,
}

fn main() {
    let args = Args::parse();

    println!("input path: {}", args.path);
}
