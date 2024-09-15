use std::{collections::HashMap, net::SocketAddr, path::PathBuf, sync::Arc};
use tera::{Context, Tera};

use crate::{resources::vanguard_logo::VanguardLogo, utils::directory_utility::list_directory_content};

pub struct DirIndexPage {
    dir_path: PathBuf,
    context: Option<Context>,
}

impl DirIndexPage {
    pub const HTML_TEMPLATE: &'static str = r#"
    <html>
        <head>
            <title>Directory: {{ path }}!</title>
        </head>
        <body>
            <img src="data:image/png;base64,{{ logo }}" width="300px" alt="Vanguard">
            <h3>Index: {{ path }} /</h3>

            <ul>
                
                {% for dir in directories %}
                    <li>&#128193; <a href="/{{path}}/{{ dir }}">{{dir}}</a> </li>
                {% endfor %}

                {% for file in files %}
                    <li>&#128196; <a href="/{{path}}/{{file}}">{{ file }}</a></li>
                {% endfor %}
            </ul>



        </body>
    </html>
    "#;

    pub fn new(dir_path: &PathBuf, url_path: &str) -> Self {
        let mut context = Context::new();

        let list_directory_operation = list_directory_content(dir_path);
        if list_directory_operation.is_none() {
            return DirIndexPage {dir_path: dir_path.to_path_buf(), context: None };
        }

        let (files, directories) = list_directory_operation.unwrap();
        context.insert("logo", VanguardLogo::BASE64_CONTENT);
        context.insert("path", url_path);
        context.insert("files", &files);
        context.insert("directories", &directories);

        DirIndexPage { dir_path: dir_path.to_path_buf(), context: Some(context) }
    }

    pub fn render(&self) -> String {

        if self.context.is_none(){
            return format!("Error while rendering path: {:?}", self.dir_path);
        }

        let context = self.context.clone().unwrap();

        let render_process = Tera::one_off(DirIndexPage::HTML_TEMPLATE, &context, true);

        if render_process.is_ok() {
            return render_process.unwrap();
        } else {
            return format!("Error: {:?}", render_process.err());
        }
    }
}
