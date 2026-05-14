use std::env;

#[derive(Clone, Debug)]
pub struct Settings {
    pub service_name: String,
    pub environment: String,
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub cors_allowed_origin: String,
    pub external_api_base_url: String,
    pub request_timeout_seconds: u64,
}

impl Settings {
    pub fn from_env() -> Result<Self, String> {
        let _ = dotenvy::dotenv();

        let service_name = env_or("SERVICE_NAME", "actix-web-microservice");
        let environment = env_or("ENVIRONMENT", "local");
        let host = env_or("HOST", "127.0.0.1");
        let port = parse_u16("PORT", 8080)?;
        let log_level = env_or("RUST_LOG", "info,actix_web=info");
        let cors_allowed_origin = env_or("CORS_ALLOWED_ORIGIN", "*");
        let external_api_base_url = env_or("EXTERNAL_API_BASE_URL", "http://localhost:8081");
        let request_timeout_seconds = parse_u64("REQUEST_TIMEOUT_SECONDS", 10)?;

        if service_name.trim().is_empty() {
            return Err("SERVICE_NAME cannot be empty".to_owned());
        }
        if request_timeout_seconds == 0 {
            return Err("REQUEST_TIMEOUT_SECONDS must be greater than zero".to_owned());
        }

        Ok(Self {
            service_name,
            environment,
            host,
            port,
            log_level,
            cors_allowed_origin,
            external_api_base_url,
            request_timeout_seconds,
        })
    }

    pub fn for_tests() -> Self {
        Self {
            service_name: "actix-web-microservice-test".to_owned(),
            environment: "test".to_owned(),
            host: "127.0.0.1".to_owned(),
            port: 0,
            log_level: "debug".to_owned(),
            cors_allowed_origin: "*".to_owned(),
            external_api_base_url: "http://localhost:8081".to_owned(),
            request_timeout_seconds: 5,
        }
    }
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| default.to_owned())
}

fn parse_u16(key: &str, default: u16) -> Result<u16, String> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => value
            .parse::<u16>()
            .map_err(|_| format!("{key} must be a valid u16")),
        _ => Ok(default),
    }
}

fn parse_u64(key: &str, default: u64) -> Result<u64, String> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => value
            .parse::<u64>()
            .map_err(|_| format!("{key} must be a valid u64")),
        _ => Ok(default),
    }
}
