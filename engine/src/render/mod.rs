use std::path::PathBuf;

mod dir_index_page;
mod internal_error_page;
mod not_found_page;

use crate::render::{
    dir_index_page::DirIndexPage, internal_error_page::InternalErrorPage,
    not_found_page::NotFoundPage,
};

pub struct Render {}

impl Render {
    pub fn directory_explorer_page(dir_path: &PathBuf, url_path: &str) -> String {
        let content = DirIndexPage::new(dir_path, url_path);

        content.render()
    }

    pub fn internal_server_error(url_path: &str, reason: &str) -> String {
        let content = InternalErrorPage::new(url_path, reason);

        content.render()
    }

    pub fn not_found_error(url_path: &str) -> String {
        let content = NotFoundPage::new(url_path);

        content.render()
    }
}
