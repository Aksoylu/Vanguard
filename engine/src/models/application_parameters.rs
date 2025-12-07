use clap::Parser;

use crate::utils::console_utility::convert_input_to_boolean;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ApplicationParameters {
    #[arg(long, default_value = "n")]
    overwrite_config: String,

    #[arg(long, default_value = "n")]
    overwrite_router: String,
}

impl ApplicationParameters {
    pub fn get_overwrite_config(&self) -> bool {
        let result: Option<bool> = convert_input_to_boolean(&self.overwrite_config);
        if result.is_some() {
            return result.unwrap();
        }

        false
    }
    pub fn get_overwrite_router(&self) -> bool {
        let result: Option<bool> = convert_input_to_boolean(&self.overwrite_router);

        if result.is_some() {
            return result.unwrap();
        }

        false
    }
}
