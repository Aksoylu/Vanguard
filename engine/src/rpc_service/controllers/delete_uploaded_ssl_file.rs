use crate::rpc_service::models::{
    delete_uploaded_ssl_file_request::DeleteUploadedSslFileRequest,
    delete_uploaded_ssl_file_response::DeleteUploadedSslFileResponse,
};
use crate::utils::file_utility::{delete_file, get_absolute_ssl_file_path};
use jsonrpc_core::{Error, Value};

pub fn delete_uploaded_ssl_file(params: Value) -> Result<Value, Error> {
    let request = DeleteUploadedSslFileRequest::new(params)?;
    let file_name = request.get_file_name();

    let absolute_path = get_absolute_ssl_file_path(&file_name)?;
    let is_success = delete_file(absolute_path);

    let mut response = DeleteUploadedSslFileResponse::new();
    response.set_success(is_success);

    Ok(response.build())
}
