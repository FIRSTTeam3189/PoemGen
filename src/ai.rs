use async_openai::config::Config;
use async_openai::error::OpenAIError;
use async_openai::types::{
    CreateCompletionRequest, CreateImageRequest, ImageResponseFormat, ImageSize, ImagesResponse, Prompt
};
use async_openai::Client;
use std::fmt::{Display, Formatter};
// use std::time::UNIX_EPOCH;

pub type AiResult<T> = std::result::Result<T, OpenAIError>;

#[derive(Debug, Copy, Clone)]
pub enum AiType {
    GPT4oMini,
    GPT4o,
    GPT3_5,
    GPT3_5_Instruct,
}

impl Display for AiType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AiType::GPT3_5_Instruct => "gpt-3.5-turbo-instruct",
                AiType::GPT4oMini => "gpt-4o-mini",
                AiType::GPT4o => "gpt-4o",
                AiType::GPT3_5 => "gpt-3.5-turbo-0125",
            }
        )
    }
}

pub struct AiSettings {
    model: AiType,
    max_tokens: u32,
    /// Amount of Generations to complete.
    n: u8,
    temperature: f32,
}

impl AiSettings {
    pub fn new_poem(model: AiType) -> Self {
        Self {
            model,
            max_tokens: 128,
            n: 1,
            temperature: 0.969,
        }
    }

    pub fn new_title(model: AiType) -> Self {
        Self {
            model,
            max_tokens: 32,
            n: 1,
            temperature: 0.769,
        }
    }

    pub fn new_prompt(model: AiType) -> Self {
        Self {
            model,
            max_tokens: 64,
            n: 1,
            temperature: 0.769,
        }
    }
}

pub async fn get_ai_response<C: Config>(
    prompt: &str,
    settings: AiSettings,
    client: &Client<C>,
) -> AiResult<String> {
    let completion_request = CreateCompletionRequest {
        model: format!("{}", settings.model),
        n: Some(settings.n),
        prompt: Prompt::String(prompt.to_owned()),
        max_tokens: Some(settings.max_tokens),
        temperature: Some(settings.temperature),
        stream: Some(false),
        ..Default::default()
    };
    println!("1");
    let thing = client.completions().create(completion_request).await?;
    let mut out = String::new();
    for l in thing.choices {
        out.push_str(&l.text);
    }

    // let mut stream = Completion::create_stream(client, completion_request).await?;
    // let mut full_text = String::new();

    // while let Some(response) = stream.next().await {
    //     match response {
    //         Ok(ccr) => ccr.choices.iter().for_each(|c| {
    //             full_text.push_str(c.text.as_str());
    //         }),
    //         Err(e) => eprintln!("{}", e),
    //     }
    // }
    println!("2");
    Ok(out)
}

pub async fn save_to_file(
    title: &str,
    input_prompt: &str,
    generated_prompt: &str,
    text: &str,
) -> Result<String, std::io::Error> {
    // get the current unix timestamp
    // let time = std::time::SystemTime::now();
    // let time = time.duration_since(UNIX_EPOCH).expect("Unable to get time");
    // let timestamp = time.as_secs();

    // create directory
    std::fs::create_dir_all("./poem")?;

    // Replace bad filename characters
    let filename = title.to_ascii_lowercase().replace(['"', ':', ',', '?', '/', '\'', '\n'], "").replace(" ", "-");

    // replace spaces with -
    let poem_name = filename.split_whitespace().collect::<Vec<&str>>().join("-");

    // create contents and write to file
    let filename = format!("./poem/{filename}.poem.txt");
    let sep = "-".repeat(80);
    let text = format!("{title}\n{sep}\nINPUT PROMPT: {input_prompt}\n{sep}\nGENERATED PROMPT:\n{generated_prompt}\n{sep}\nPOEM:\n{sep}\n{text}\n{sep}");
    std::fs::write(filename.as_str(), text)?;
    Ok(format!("./poem/{poem_name}"))
}

pub async fn get_image<C: Config>(
    client: &Client<C>,
    prompt: &str,
    n: u8,
    size: ImageSize,
) -> AiResult<ImagesResponse> {
    let request = CreateImageRequest {
        n: Some(n),
        size: Some(size),
        response_format: Some(ImageResponseFormat::Url),
        prompt: prompt.to_string(),
        user: Some("poem-gen".to_string()),
        ..Default::default()
    };
    client.images().create(request).await
}
