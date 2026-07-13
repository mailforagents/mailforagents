use std::{env, net::SocketAddr, path::PathBuf};

const DEFAULT_ADDRESS: &str = "127.0.0.1:3200";
const DEFAULT_DATABASE_PATH: &str = "data/app.sqlite";

#[derive(Clone, Debug)]
pub struct Config {
    pub address: SocketAddr,
    pub database_path: PathBuf,
    pub deployment_mode: DeploymentMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeploymentMode {
    Cloud,
    SelfHosted,
}

impl DeploymentMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Cloud => "Mail for Agents Cloud",
            Self::SelfHosted => "Self-hosted",
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let address = env::var("MFA_APP_ADDR")
            .unwrap_or_else(|_| DEFAULT_ADDRESS.into())
            .parse()
            .map_err(|error| format!("MFA_APP_ADDR must be a valid socket address: {error}"))?;
        let database_path = env::var("MFA_DATABASE_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(DEFAULT_DATABASE_PATH));
        let deployment_mode = match env::var("MFA_DEPLOYMENT_MODE")
            .unwrap_or_else(|_| "self-hosted".into())
            .to_ascii_lowercase()
            .as_str()
        {
            "cloud" => DeploymentMode::Cloud,
            "self-hosted" | "self_hosted" => DeploymentMode::SelfHosted,
            value => {
                return Err(format!(
                    "MFA_DEPLOYMENT_MODE must be cloud or self-hosted, got {value}"
                ));
            }
        };

        Ok(Self {
            address,
            database_path,
            deployment_mode,
        })
    }
}
