use std::sync::LazyLock;

/// Global application configuration loaded from environment variables.
/// This is initialized once on first access and cached for the lifetime of the application.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Config::from_env().unwrap_or_else(|e| {
        panic!("Failed to load configuration: {}", e);
    })
});

/// Application configuration struct containing all environment-based settings.
#[derive(Debug, Clone)]
pub struct Config {
    /// Server address to bind to (from SIMPLE_HEALTH_ADDR)
    pub addr: String,

    /// Server port to bind to (from SIMPLE_HEALTH_PORT)
    pub port: u16,

    /// PostgreSQL database connection URL (from SIMPLE_HEALTH_PG_URL)
    pub database_url: String,

    /// JWT signing secret key (from SIMPLE_HEALTH_JWT_SECRET)
    pub jwt_secret: String,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `SIMPLE_HEALTH_ADDR`: Server address (default: "localhost")
    /// - `SIMPLE_HEALTH_PORT`: Server port (default: "3000")
    /// - `SIMPLE_HEALTH_PG_URL`: PostgreSQL connection URL (required)
    /// - `SIMPLE_HEALTH_JWT_SECRET`: JWT signing secret key (required)
    ///
    /// # Errors
    /// Returns an error if:
    /// - Required environment variables are missing
    /// - Port cannot be parsed as a valid u16
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let addr = std::env::var("SIMPLE_HEALTH_ADDR").unwrap_or_else(|_| "localhost".to_string());

        let port_str = std::env::var("SIMPLE_HEALTH_PORT").unwrap_or_else(|_| "3000".to_string());
        let port = port_str
            .parse::<u16>()
            .map_err(|e| format!("Invalid port '{}': {}", port_str, e))?;

        let database_url = std::env::var("SIMPLE_HEALTH_PG_URL")
            .map_err(|_| "Missing required environment variable: SIMPLE_HEALTH_PG_URL")?;

        let jwt_secret = std::env::var("SIMPLE_HEALTH_JWT_SECRET")
            .map_err(|_| "Missing required environment variable: SIMPLE_HEALTH_JWT_SECRET")?;

        Ok(Config {
            addr,
            port,
            database_url,
            jwt_secret,
        })
    }

    /// Get the full socket address (host:port) for binding the server.
    pub fn socket_addr(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}

/// Convenience function to access the global configuration.
/// This will initialize the config on first call and return a reference to it.
pub fn get_config() -> &'static Config {
    &CONFIG
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_addr() {
        let config = Config {
            addr: "127.0.0.1".to_string(),
            port: 8080,
            database_url: "postgres://localhost/test".to_string(),
            jwt_secret: "test_secret".to_string(),
        };

        assert_eq!(config.socket_addr(), "127.0.0.1:8080");
    }
}
