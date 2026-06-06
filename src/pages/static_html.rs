#[macro_export]
macro_rules! static_page {
    ($name:ident, $title:literal, $path:literal) => {
        ::pastey::paste! {
            #[derive(::askama::Template)]
            #[template(path = $path)]
            struct [< $name:camel >] {
                title: &'static str
            }

            async fn [< $name:snake >]() -> ::axum::response::Html<String> {
                ::axum::response::Html(
                    <[< $name:camel >] as ::askama::Template>::render(
                        &[< $name:camel >] {
                            title: $title,
                        }
                    ).unwrap()
                )
            }
        }
    };
}
