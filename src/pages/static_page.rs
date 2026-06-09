use askama::Template;

#[macro_export]
macro_rules! static_html {
    ($name:ident, $title:literal, $path:literal) => {
        ::pastey::paste! {
            #[derive(::askama::Template)]
            #[template(path = $path)]
            struct [< $name:camel >] {
                title: &'static str,
                env: $crate::env::Env,
            }

            async fn [< $name:snake >](
                ::axum::extract::State(state): ::axum::extract::State<$crate::AppState>,
            ) -> ::axum::response::Html<String> {
                ::axum::response::Html(
                    <[< $name:camel >] as ::askama::Template>::render(
                        &[< $name:camel >] {
                            title: $title,
                            env: state.env,
                        }
                    ).unwrap()
                )
            }
        }
    };
}

pub fn markdown_to_html(md: &str) -> String {
    use pulldown_cmark::{Options, Parser, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

#[derive(Template)]
#[template(path = "static_markdown.html")]
pub struct MarkdownTemplate {
    pub title: &'static str,
    pub content: String,
    pub env: crate::env::Env,
}

#[macro_export]
macro_rules! static_markdown {
    ($name:ident, $title:literal, $path:literal) => {
        ::pastey::paste! {

            async fn [< $name:snake >](
                ::axum::extract::State(state): ::axum::extract::State<$crate::AppState>,
            ) -> ::axum::response::Html<String> {

                let md: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/" ,$path));
                let content = $crate::pages::static_page::markdown_to_html(md);
                ::axum::response::Html(
                    <$crate::pages::static_page::MarkdownTemplate as ::askama::Template>::render(
                        &$crate::pages::static_page::MarkdownTemplate{
                            title: $title,
                            content,
                            env: state.env,
                        }
                    ).unwrap()
                )
            }
        }
    };
}
