use tera::{Context, Tera};

use crate::resources::vanguard_logo::VanguardLogo;

pub struct NotFoundPage {
    url_path: String,
    context: Option<Context>,
}

impl NotFoundPage {
    pub const HTML_TEMPLATE: &'static str = r#"
    <html>
        <head>
            <title>404 Not Found: {{ url_path }}</title>
        </head>
        <body>
            <img src="data:image/png;base64,{{ logo }}" width="400px" alt="Vanguard">
            <h1>404 Not Found Error</h1>
            {{ back_button | safe }}

            <p>Vanguard Web Server could not find any content to server on path:<br/> <b>{{ url_path }}</b></p>
        </body>
    </html>
    "#;

    pub fn new(url_path: &str) -> Self {
        let mut context = Context::new();

        context.insert("logo", VanguardLogo::BASE64_CONTENT);
        context.insert("url_path", &url_path);
        context.insert("back_button", &NotFoundPage::return_back_button());

        NotFoundPage {
            context: Some(context),
            url_path: format!("/{}", url_path),
        }
    }

    pub fn render(&self) -> String {
        if self.context.is_none() {
            return format!("Error while rendering path: {:?}", self.url_path);
        }

        let context = self.context.clone().unwrap();
        let render_process = Tera::one_off(NotFoundPage::HTML_TEMPLATE, &context, true);

        if render_process.is_ok() {
            render_process.unwrap()
        } else {
            format!("Error: {:?}", render_process.err())
        }
    }

    pub fn return_back_button() -> String {
        r#"<p 
            style="text-decoration:underline; color:#337be8; margin-top:-1.5rem; cursor:pointer"
            onclick="history.back()"
        >&#8592; Return Back
        </p>"#.to_string()
    }
}
