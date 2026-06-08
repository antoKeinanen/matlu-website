#[macro_export]
macro_rules! static_page {
    ($name:ident, $title:literal, $path:literal) => {
        use axum::extract::State;
        use crate::AppState;
        use crate::env::Env;

        ::pastey::paste! {
            #[derive(::askama::Template)]
            #[template(path = $path)]
            struct [< $name:camel >] {
                title: &'static str,
                env: Env,
            }

            async fn [< $name:snake >](State(state): State<AppState>) -> ::axum::response::Html<String> {
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
