use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// title of homework
    #[arg(short, long)]
    title: String,

    /// Course Number
    #[arg(short, long)]
    course: String,

    /// No. of questions
    #[arg(short, long)]
    length: u8,
}

fn main() {
    let args = Args::parse();
    println!("{args:#?}");
}
