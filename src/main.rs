use ccwc::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("{:?}", args.should_display_bytes);
}
