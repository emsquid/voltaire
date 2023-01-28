use crate::result::Result;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub struct GrammarError {
    position: usize,
    length: usize,
    correction: String,
    message: String,
}

pub struct Voltaire {
    sentence: String,
    errors: Vec<GrammarError>,
}

impl Voltaire {
    async fn send_post(sentence: &String) -> Result<String> {
        let client = reqwest::Client::new();
        let queries = [
            ("text", sentence.as_str()),
            ("language", "fr"),
            ("enabledOnly", "false"),
        ];
        let response = client
            .post("https://api.languagetoolplus.com/v2/check")
            .query(&queries)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    // My own little touch, don't mind
    fn emanuel(sentence: &String, errors: &mut Vec<GrammarError>) {
        if let Some(position) = sentence.to_lowercase().find("emmanuel") {
            errors.push(GrammarError {
                position,
                length: 8,
                correction: String::from("Emanuel"),
                message: String::from("Qu'est ce que c'est que ce nom."),
            });
        } else {
            errors.retain(|e| sentence.get(e.position..e.position + e.length) != Some("Emanuel"))
        }
    }

    fn prepare_errors(sentence: &String, errors: &mut Vec<GrammarError>) {
        Voltaire::emanuel(&sentence, errors);
        // sort and merge overlapping
        errors.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        for i in (1..errors.len()).rev() {
            if errors[i - 1].position + errors[i - 1].length
                >= errors[i].position + errors[i].length
            {
                let start = errors[i].position - errors[i - 1].position;
                let end = start + errors[i].length;
                let correction = errors[i].correction.clone();

                errors[i - 1]
                    .correction
                    .replace_range(start..end, &correction);

                errors.remove(i);
            }
        }
    }

    pub fn from_json(sentence: String, json: serde_json::Value) -> Result<Self> {
        let mut errors = Vec::new();

        if let Some(array) = json["matches"].as_array() {
            for error in array {
                let message = error["message"].as_str();
                let position = error["offset"].as_i64();
                let length = error["length"].as_i64();
                let correction = error["replacements"][0]["value"].as_str();

                if let (Some(message), Some(position), Some(length), Some(correction)) =
                    (message, position, length, correction)
                {
                    errors.push(GrammarError {
                        position: position as usize,
                        length: length as usize,
                        correction: correction.to_string(),
                        message: message.to_string(),
                    });
                }
            }
        }

        Voltaire::prepare_errors(&sentence, &mut errors);
        Ok(Self { sentence, errors })
    }

    pub async fn from(sentence: String) -> Result<Self> {
        let response = Voltaire::send_post(&sentence).await?;
        let json = serde_json::from_str(&response)?;

        Voltaire::from_json(sentence, json)
    }

    pub fn corrected(&self) -> String {
        let mut corrected = self.sentence.clone();

        for error in self.errors.iter().rev() {
            let start = error.position;
            let end = error.position + error.length;

            corrected.insert_str(end, RESET);
            corrected.replace_range(start..end, &error.correction);
            corrected.insert_str(start, GREEN);
        }

        corrected
    }

    pub fn print(&self, verbose: bool) {
        let mut styled = self.sentence.clone();
        let mut messages = Vec::new();

        for error in self.errors.iter().rev() {
            let start = error.position;
            let end = error.position + error.length;

            if verbose {
                messages.insert(
                    0,
                    format!(
                        "{}: {RED}{}{RESET} -> {GREEN}{}{RESET}: {}",
                        start,
                        self.sentence.get(start..end).unwrap(),
                        error.correction,
                        error.message,
                    ),
                );
            }

            styled.insert_str(end, RESET);
            styled.insert_str(start, RED);
        }

        for message in messages {
            println!("{}", message);
        }
        println!("{} -> {}", styled, self.corrected());
    }
}
