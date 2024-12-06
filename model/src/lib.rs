

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Contributions {
    login: String,
    repo: String,
    contributions: u32,
    language: HashMap<String, i64>,
}

impl Contributions {
    pub fn new(login: String, repo: String, contributions: u32, language: HashMap<String, i64>) -> Self {
        Self {
            login,
            repo,
            contributions,
            language,
        }
    }

    pub fn login(&self) -> &String {
        &self.login
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
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stats{
    contributions: Vec<Contributions>
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
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stats() {
        let mut x: HashMap<String, i64> = HashMap::new();
        x.insert("rust".to_string(), 100);
        let c = Contributions::new("silen".to_string(), "test".to_string(), 100, x);
        let s = Stats::new(vec![c]);
        assert_eq!(
            s.as_toml(),
            "[[contributions]]\nlogin = \"silen\"\ncontributions = 100\nlanguage = [\"rust\"]\n"
        );
    }
}
