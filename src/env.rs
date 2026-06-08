use std::env;
use validator::Validate;

#[derive(Debug, Validate, Clone)]
pub struct Env {
    #[validate(url)]
    pub cdn_url: String,
}

impl Env {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let cdn_url = env::var("CDN_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());

        let env = Env { cdn_url };

        if let Err(errors) = env.validate() {
            panic!("❌ Invalid environment variables:\n{errors}");
        }

        env
    }
}
