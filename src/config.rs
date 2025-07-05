use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Job {
    pub name: String,
    pub command: String,
    pub schedule: String,
    pub timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub jobs: Vec<Job>,
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
