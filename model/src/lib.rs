use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Contributions {
    login: String,
    owner: String,
    repo: String,
    contributions: u32,
    language: HashMap<String, i64>,
}

impl Contributions {
    pub fn new(
        login: String,
        owner: String,
        repo: String,
        contributions: u32,
        language: HashMap<String, i64>,
    ) -> Self {
        Self {
            login,
            owner,
            repo,
            contributions,
            language,
        }
    }

    pub fn anonymize(&self) -> Self {
        Contributions::new(
            self.login.clone(),
            "private".to_string(),
            "private".to_string(),
            self.contributions,
            self.language.clone(),
        )
    }

    pub fn login(&self) -> &String {
        &self.login
    }

    pub fn owner(&self) -> &String {
        &self.owner
    }

    pub fn contributions(&self) -> &u32 {
        &self.contributions
    }

    pub fn language(&self) -> &HashMap<String, i64> {
        &self.language
    }

    pub fn as_toml(&self) -> String {
        toml::to_string_pretty(&self).unwrap()
    }

    pub fn from_toml(toml: &str) -> Self {
        toml::from_str(toml).unwrap()
    }
}

pub fn by_language(contributions: Vec<Contributions>) -> HashMap<String, i64> {
    let mut lang: HashMap<String, i64> = HashMap::new();
    for c in contributions.iter() {
        for (k, v) in c.language.iter() {
            let count = lang.entry(k.clone()).or_insert(0);
            *count += v;
        }
    }
    lang
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stats {
    contributions: Vec<Contributions>,
}
impl Stats {
    pub fn new(contributions: Vec<Contributions>) -> Self {
        Self { contributions }
    }

    pub fn get_contributions(&self) -> &Vec<Contributions> {
        &self.contributions
    }

    pub fn as_toml(&self) -> String {
        toml::to_string_pretty(&self).unwrap()
    }

    pub fn from_toml(toml: &str) -> Self {
        toml::from_str(toml).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stats() {
        let mut x: HashMap<String, i64> = HashMap::new();
        x.insert("rust".to_string(), 100);
        let c = Contributions::new(
            "silen".to_string(),
            "silen".to_string(),
            "test".to_string(),
            100,
            x,
        );
        let s = Stats::new(vec![c]);
        assert_eq!(
            s.as_toml(),
            "[[contributions]]\nlogin = \"silen\"\ncontributions = 100\nlanguage = [\"rust\"]\n"
        );
    }
}
