use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "default.conf")]
    config: String,
}