// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{Model, ValidationError};

fn default_schema() -> String {
    "public".to_string()
}

fn default_sslmode() -> String {
    "verify-full".to_string()
}

/// PostgreSQL connection configuration and HikariCP pool settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostgresConnection {
    /// The database URL in JDBC format.
    pub url: String,
    /// The name of the schema to use.
    #[serde(default = "default_schema", rename = "schema")]
    pub provider_schema: String,
    /// The username to use for authentication.
    pub username: String,
    /// The password to use for authentication.
    #[serde(default)]
    pub password: String,
    /// The SSL mode to use, one of "disable", "allow", "prefer", "require", "verify-ca" or
    /// "verify-full".
    #[serde(default = "default_sslmode")]
    pub sslmode: String,
    /// The full path of the certificate file.
    #[serde(default)]
    pub sslcert: Option<String>,
    /// The full path of the key file.
    #[serde(default)]
    pub sslkey: Option<String>,
    /// The full path of the root certificate file.
    #[serde(default)]
    pub sslrootcert: Option<String>,
    /// Maximum milliseconds to wait for connections from the pool.
    #[serde(default)]
    pub connection_timeout: Option<i64>,
    /// Maximum milliseconds a connection may sit idle in the pool.
    #[serde(default)]
    pub idle_timeout: Option<i64>,
    /// Frequency in milliseconds that the pool will keep an idle connection alive.
    #[serde(default)]
    pub keepalive_time: Option<i64>,
    /// Maximum lifetime of a connection in milliseconds.
    #[serde(default)]
    pub max_lifetime: Option<i64>,
    /// Maximum size of the connection pool.
    #[serde(default)]
    pub maximum_pool_size: Option<i64>,
    /// Minimum number of idle connections that the pool tries to maintain.
    #[serde(default)]
    pub minimum_idle: Option<i64>,
}

impl fmt::Display for PostgresConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl Model for PostgresConnection {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.url.is_empty() {
            return Err(ValidationError::MissingField {
                field: "url".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_url_fails_validation() {
        let conn = PostgresConnection {
            url: String::new(),
            provider_schema: default_schema(),
            username: "user".to_string(),
            password: String::new(),
            sslmode: default_sslmode(),
            sslcert: None,
            sslkey: None,
            sslrootcert: None,
            connection_timeout: None,
            idle_timeout: None,
            keepalive_time: None,
            max_lifetime: None,
            maximum_pool_size: None,
            minimum_idle: None,
        };
        assert!(conn.validate().is_err());
    }
}
