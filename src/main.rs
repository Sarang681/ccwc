use ccwc::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    args.execute();
}
