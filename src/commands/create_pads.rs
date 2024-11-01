use crate::otp;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(long, default_value_t = 10)]
    nb_pads: usize,
    #[arg(long, default_value_t = 20)]
    nb_keys: usize,
}

impl Cli {
    pub fn create_pads(&self) -> Result<()> {
        let nb_pads = self.nb_pads;
        let nb_keys = self.nb_keys;

        let mut generator = otp::pad::PadGenerator::new();
        let mut pad_collection = otp::pad::PadCollection::new();

        for _ in 0..nb_pads {
            let pad = generator.generate_pad(nb_keys);
            pad_collection.add_pad(pad);
        }

        println!("{:?}", pad_collection);
        Ok(())
    }
}
