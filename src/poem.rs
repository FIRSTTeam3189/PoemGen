use std::fmt::{Display, Formatter};

use async_openai::{config::Config, Client};

use crate::ai::{get_ai_response, AiResult, AiSettings, AiType};

pub struct PoemGenBuilder {
    prompt: String,
    poem_type: Option<PoemType>,
    first_stage_ai: AiType,
    poem_ai: AiType,
    title_ai: AiType,
}

impl PoemGenBuilder {
    pub fn new() -> Self {
        Self {
            prompt: String::new(),
            poem_type: None,
            first_stage_ai: AiType::GPT3_5_instruct,
            poem_ai: AiType::GPT3_5_instruct,
            title_ai: AiType::GPT3_5_instruct,
        }
    }
    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }

    pub fn with_prompt(mut self, prompt: String) -> Self {
        self.prompt = prompt;
        self
    }
    pub async fn generate<C: Config>(self, client: &mut Client<C>) -> AiResult<Poem> {
        let prompt = if !self.prompt.trim().is_empty() {
            self.prompt
        } else {
            get_poem_prompt()
        };
        let ty = if let Some(pt) = self.poem_type {
            pt.clone()
        } else {
            PoemType::random()
        };
        let first_stage = format!("Write a {ty} prompt {prompt}");
        // Generate a poem prompt for extra randomness in poem gen.
        let full_prompt = get_ai_response(
            &first_stage,
            AiSettings::new_prompt(self.first_stage_ai),
            &client,
        )
        .await?
        .trim()
        .to_string();
        // finish prompts for title and poem.
        let second_stage = format!("Write a {ty} about:\n{full_prompt}");
        let title_prompt = format!("Write a title for a {ty} about:\n{full_prompt}");
        // generate title
        let title = get_ai_response(&title_prompt, AiSettings::new_title(self.title_ai), &client)
            .await?.trim().to_owned();
        // generate poem.
        let poem = get_ai_response(
            &second_stage,
            AiSettings::new_poem(self.poem_ai),
            &client,
        )
        .await?
        .trim()
        .to_string();
        Ok(Poem {
            poem_type: ty,
            first_stage,
            second_stage,
            title,
            poem,
        })
    }
}

pub fn get_poem_prompt() -> String {
    let selector: u8 = rand::random();
    match selector % 4 {
     0 => format!("for a poem about robots"),
     1 => format!("for poem about students engineering building a robot for team-based competitive game"),
     2 => format!("about robots playing competitive games together"),
     _ => format!("pertaining to the themes education, science, robotics and friendly competition"),
    }
}

pub struct Poem {
    pub poem_type: PoemType,
    pub first_stage: String,
    pub second_stage: String,
    pub title: String,
    pub poem: String,
}

impl Display for Poem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}FIRST-STAGE{}", "-".repeat(35), "-".repeat(34))?;
        writeln!(f, "{}", self.first_stage)?;
        writeln!(f, "{}", "-".repeat(80))?;
        writeln!(f, "{}SECOND-STAGE{}", "-".repeat(34), "-".repeat(34))?;
        writeln!(f, "{}", self.second_stage)?;
        writeln!(f, "{}", "^".repeat(80))?;
        writeln!(f, "{}", self.title)?;
        writeln!(f, "{}", "-".repeat(80))?;
        writeln!(f, "{}", self.poem)?;
        writeln!(f, "{}", "=".repeat(80))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PoemType {
    Sonnet,
    Villanelle,
    Haiku,
    Ekphrastic,
    Concrete,
    Elegy,
    Epigram,
    Limerick,
    Ballad,
    Epitaph,
    DrSeuss,
    Ode,
    FreeVerse,
}

impl PoemType {
    pub fn random() -> Self {
        let r: u8 = rand::random();
        match r % 10 {
            0 => PoemType::Sonnet,
            1 => PoemType::Villanelle,
            2 => PoemType::Ekphrastic,
            3 => PoemType::Concrete,
            4 => PoemType::Elegy,
            5 => PoemType::Epigram,
            // 6 => PoemType::Ballad,
            6 => PoemType::DrSeuss,
            7 => PoemType::Ode,
            _ => PoemType::FreeVerse,
            // 0 => PoemType::Haiku,
            // 0 => PoemType::Limerick,
            // 0 => PoemType::Epitaph,
        }
    }
}

impl Display for PoemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PoemType::Sonnet => "Sonnet",
                PoemType::Villanelle => "Villanelle Poem",
                PoemType::Haiku => "Haiku",
                PoemType::Ekphrastic => "Ekphrastic Poem",
                PoemType::Concrete => "Concrete Poem",
                PoemType::Elegy => "Elegy",
                PoemType::Epigram => "Epigram",
                PoemType::Limerick => "Limerick",
                PoemType::Ballad => "Ballad",
                PoemType::Epitaph => "Epitaph",
                PoemType::DrSeuss => "Dr. Seuss Poem",
                PoemType::Ode => "Ode",
                PoemType::FreeVerse => "Free Verse Poem",
            }
        )
    }
}
