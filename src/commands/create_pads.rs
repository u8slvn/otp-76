use crate::otp;
use crate::parsers::parse_int_arg;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(long, default_value_t = 10, value_parser = parse_int_arg)]
    nb_pads: u8,
    #[arg(long, default_value_t = 20, value_parser = parse_int_arg)]
    nb_keys: u8,
}

impl Cli {
    pub fn create_pads(&self) -> Result<()> {
        let mut generator = otp::pad::PadGenerator::new();
        let mut pad_collection = otp::pad::PadCollection::new();

        for _ in 0..self.nb_pads {
            let pad = generator.generate_pad(self.nb_keys);
            pad_collection.add_pad(pad.unwrap());
        }

        println!("{:?}", pad_collection);
        Ok(())
    }
}
