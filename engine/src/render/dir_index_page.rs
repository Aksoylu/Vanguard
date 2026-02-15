use std::path::PathBuf;
use tera::{Context, Tera};

use crate::{
    resources::vanguard_logo::VanguardLogo, utils::directory_utility::list_directory_content,
};

pub struct DirIndexPage {
    dir_path: PathBuf,
    context: Option<Context>,
}

impl DirIndexPage {
    pub const HTML_TEMPLATE: &'static str = r#"
    <html>
        <head>
            <title>Index of {{ current_path_name }}</title>
        </head>
        <body>
            <img src="data:image/png;base64,{{ logo }}" width="400px" alt="Vanguard">
            {{ stack_navigator  | safe }}
            <ul style="list-style-type: none;  margin: 0; padding: 0;">
                {{parent_path_button  | safe }}

                {% for dir in directories %}
                    <li>&#128193; <a href="/{{current_path}}/{{ dir }}">{{dir}}</a> </li>
                {% endfor %}

                {% for file in files %}
                    <li>&#128196; <a href="/{{current_path}}/{{file}}">{{ file }}</a></li>
                {% endfor %}
            </ul>
        </body>
    </html>
    "#;

    pub fn new(dir_path: &PathBuf, url_path: &str) -> Self {
        let mut context = Context::new();

        let list_directory_operation: Option<(Vec<String>, Vec<String>)> =
            list_directory_content(dir_path);
        if list_directory_operation.is_none() {
            return DirIndexPage {
                dir_path: dir_path.to_path_buf(),
                context: None,
            };
        }

        let path_parts: &Vec<&str> = &DirIndexPage::split_path_parts(url_path);
        let current_path_name = &DirIndexPage::get_current_path_name(path_parts);

        let (files, directories) = list_directory_operation.unwrap();
        context.insert("logo", VanguardLogo::BASE64_CONTENT);
        context.insert("current_path", &url_path);
        context.insert("current_path_name", &current_path_name);
        context.insert(
            "stack_navigator",
            &DirIndexPage::render_stack_navigator(path_parts),
        );
        context.insert(
            "parent_path_button",
            &DirIndexPage::render_parent_path_button(path_parts),
        );
        context.insert("files", &files);
        context.insert("directories", &directories);

        DirIndexPage {
            dir_path: dir_path.to_path_buf(),
            context: Some(context),
        }
    }

    pub fn render(&self) -> String {
        if self.context.is_none() {
            return format!("Error while rendering path: {:?}", self.dir_path);
        }

        let context = self.context.clone().unwrap();

        let render_process = Tera::one_off(DirIndexPage::HTML_TEMPLATE, &context, true);

        if render_process.is_ok() {
            render_process.unwrap()
        } else {
            format!("Error: {:?}", render_process.err())
        }
    }

    fn render_parent_path_button(path_parts: &Vec<&str>) -> String {
        let mut upper_path_buffer = String::new();

        let part_count: usize = path_parts.len();

        if part_count > 1 {
            let upper_path_parts = &path_parts[0..part_count - 1];

            for part in upper_path_parts {
                upper_path_buffer = format!("{}/{}", upper_path_buffer, part);
            }
        } else {
            upper_path_buffer = "/".to_owned();
        }

        let component_html = format!(
            r#"<li>&#8593; <a href="{}">..</a> </li>"#,
            upper_path_buffer
        );
        component_html
    }

    fn render_stack_navigator(path_parts: &Vec<&str>) -> String {
        let mut breadcrumb_items: Vec<String> = vec![];

        for i in 0..(path_parts.len() + 1) {
            let parent_path_parts = &path_parts[0..(i)].to_vec();

            let part_path = &DirIndexPage::build_path(parent_path_parts);
            let part_name = &DirIndexPage::get_current_path_name(parent_path_parts);

            if part_name.trim().is_empty() {
                continue;
            }

            breadcrumb_items.push(format!(r#"<a href="{}">{}</a>"#, part_path, part_name));
        }

        format!(
            r#"<h3>Index: {} /</h3>"#,
            breadcrumb_items.join(" <b>/</b> ")
        )
    }

    fn split_path_parts(url_path: &str) -> Vec<&str> {
        url_path.split("/").collect()
    }

    fn get_current_path_name(path_parts: &Vec<&str>) -> String {
        if !path_parts.is_empty() {
            path_parts[path_parts.len() - 1].to_string()
        } else {
            String::from("Root")
        }
    }

    fn build_path(path_parts: &Vec<&str>) -> String {
        if !path_parts.is_empty() {
            let mut build_buffer = String::new();
            for part in path_parts {
                build_buffer = format!("{}/{}", build_buffer, part);
            }

            build_buffer
        } else {
            String::from("/")
        }
    }
}
