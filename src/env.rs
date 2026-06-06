use std::env;
use validator::Validate;

#[derive(Debug, Validate)]
pub struct Env {
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
}

impl Env {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let port_str = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

        let env = Env {
            port: port_str
                .parse::<u16>()
                .expect("PORT must be valid number between 1-65535"),
        };

        if let Err(errors) = env.validate() {
            panic!("❌ Invalid environment variables:\n{errors}");
        }

        env
    }
}
