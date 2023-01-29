use crate::{options::Options, result::Result};
use std::{ops::Range, time::Duration};

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const CROSSED: &str = "\x1b[9m";
const RESET: &str = "\x1b[0m";

fn range_utf8(string: &String, mut start: usize, mut end: usize) -> Range<usize> {
    string.char_indices().for_each(|(pos, ch)| {
        if pos < start {
            start = start + (ch.len_utf8() - 1);
            end = end + (ch.len_utf8() - 1);
        } else if pos < end {
            end = end + (ch.len_utf8() - 1);
        }
    });

    start..end
}

fn get_range(string: &String, start: usize, end: usize) -> String {
    string
        .get(range_utf8(string, start, end))
        .unwrap_or_default()
        .to_string()
}

fn replace_range(string: &mut String, start: usize, end: usize, with: String) {
    string.replace_range(range_utf8(string, start, end), &with);
}

pub struct GrammarError {
    sentence: String,
    position: usize,
    length: usize,
    suggestions: Vec<String>,
    explanation: String,
}

impl GrammarError {
    pub fn from_json(error: &serde_json::Value, options: &Options) -> Option<Self> {
        let message = error["message"].as_str()?;
        let position = error["offset"].as_i64()?;
        let length = error["length"].as_i64()?;
        let mut suggestions = error["replacements"]
            .as_array()?
            .iter()
            .flat_map(|j| Some(j["value"].as_str()?.to_string()))
            .collect::<Vec<String>>();
        suggestions.truncate(options.number as usize);

        Some(GrammarError {
            sentence: options.text.clone(),
            position: position as usize,
            length: length as usize,
            suggestions,
            explanation: message.to_string(),
        })
    }

    pub fn get_start(&self) -> usize {
        self.position
    }

    pub fn get_end(&self) -> usize {
        self.position + self.length
    }

    pub fn get_word(&self, color: &str, modifier: &str) -> String {
        let mut word = get_range(&self.sentence, self.position, self.position + self.length);
        word = word.replace(" ", "_");

        format!("{modifier}{color}{word}{RESET}")
    }

    pub fn get_suggestions(&self, color: &str, modifier: &str) -> String {
        let suggestions = self
            .suggestions
            .iter()
            .map(|r| r.replace(" ", "_"))
            .collect::<Vec<String>>()
            .join(&format!("{RESET}, {modifier}{color}"));

        format!("{modifier}{color}{suggestions}{RESET}",)
    }

    pub fn get_explanation(&self, color: &str, modifier: &str) -> String {
        let explanation = self.explanation.clone();
        format!("{modifier}{color}{explanation}{RESET}")
    }
}

pub struct Voltaire {
    pub sentence: String,
    pub errors: Vec<GrammarError>,
    options: Options,
}

impl Voltaire {
    pub async fn get_analysis(sentence: &String) -> Result<serde_json::Value> {
        let client = reqwest::Client::new();
        let queries = [
            ("text", sentence.as_str()),
            ("language", "auto"),
            ("enabledOnly", "false"),
            ("level", "picky"),
        ];
        let response = client
            .post("https://api.languagetoolplus.com/v2/check")
            .timeout(Duration::from_secs(5))
            .query(&queries)
            .send()
            .await?
            .text()
            .await?;
        let json = serde_json::from_str(&response)?;

        Ok(json)
    }

    pub fn from_analysis(analysis: serde_json::Value, options: &Options) -> Result<Self> {
        let mut errors = Vec::new();

        if let Some(array) = analysis["matches"].as_array() {
            for error in array {
                if let Some(grammar_error) = GrammarError::from_json(&error, options) {
                    errors.push(grammar_error);
                }
            }
        }

        errors.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());

        Ok(Self {
            sentence: options.text.clone(),
            errors,
            options: options.clone(),
        })
    }

    pub async fn from(options: &Options) -> Result<Self> {
        let analysis = Voltaire::get_analysis(&options.text).await?;

        Voltaire::from_analysis(analysis, options)
    }

    pub fn print(&self) {
        let mut styled = self.sentence.clone();
        let mut explanations = Vec::new();

        for error in self.errors.iter().rev() {
            let start = error.get_start();
            let end = error.get_end();
            let word = error.get_word(RED, "");
            let suggestions = error.get_suggestions(GREEN, "");
            let explanation = error.get_explanation("", "");

            if self.options.verbose {
                explanations.push(format!("{start}: {word} -> {suggestions}: {explanation}",));
            }

            replace_range(&mut styled, start, end, error.get_word(RED, CROSSED));
        }

        if self.errors.len() == 0 {
            println!("{GREEN}Great{RESET}: {styled}");
        } else {
            println!("{RED}Disappointing...{RESET}: {styled}");
        }

        for message in explanations.iter().rev() {
            println!("{message}");
        }
    }
}
