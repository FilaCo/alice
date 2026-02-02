use ac_interface::compiler::Config;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
#[command(version)]
pub struct AcDriver {
    /// Input source file
    pub input: PathBuf,
}

impl Default for AcDriver {
    fn default() -> Self {
        AcDriver::parse()
    }
}

impl From<AcDriver> for Config {
    fn from(driver: AcDriver) -> Self {
        Self {
            input: driver.input,
        }
    }
}
