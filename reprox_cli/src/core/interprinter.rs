use crate::utils::{console::clear_screen, version::print_version};

pub struct CommandInterprinter{

}

impl CommandInterprinter{
    pub async fn execute(input: String){
        println!("You entered: {}", input);

        if input.eq("clean"){
            clear_screen();
        }
        else if input.eq("version"){
            print_version();
        }
    }
}