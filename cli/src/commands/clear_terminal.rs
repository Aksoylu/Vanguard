
use crate::{log_error, utils::console::clear_screen};

pub async fn clear_terminal(){
   let execution_result = clear_screen();
   
   if execution_result.is_err(){
    let error_content = execution_result.err();
    log_error!("{}", error_content.unwrap().to_string())
   }
}
