use ac_interface::compiler::{Config, run_compiler};
use ac_query::compile;

use crate::driver::AcDriver;

impl AcDriver {
    pub fn compile(self) {
        run_compiler(Config::from(self), |db| {
            let input = db.input().expect("unable to get input file");
            compile::compile(db, input);
        });
    }
}
