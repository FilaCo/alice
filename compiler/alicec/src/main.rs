use alicec::prelude::*;
use alicec_interface::prelude::Alicec;
use clap::Parser;

fn main() {
    let args = AlicecCli::parse();
    let ci = Alicec::default();

    ci.run(&args.into())
}
