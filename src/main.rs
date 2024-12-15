pub mod ai;
pub mod poem;

#[cfg(feature = "img_gen")]
use async_openai::types::ImageSize;
use async_openai::{self as openai, config::OpenAIConfig};

use openai::Client;
use poem::PoemGenBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = OpenAIConfig::new()
        .with_api_key(std::fs::read_to_string("./token").expect("Unable to load token"));
    let mut client = Client::with_config(config);

    // let poem = PoemGenBuilder::new().with_prompt("Poem about a mean robot".to_string()).generate(&mut client).await?;
    let poem = PoemGenBuilder::new().generate(&mut client).await?;
    // Do image generation
    #[cfg(feature = "img_gen")]
    {
        let rand_type = &poem.poem_type;
        let title = &poem.title;
        let full_text = &poem.poem;
        println!("Generating image using prompt:");
        let image_prompt =
            format!("Picture of the {rand_type} named {title}\n\nCONTENT:\n\n{full_text}\n\n");
        let trimmed_prompt = image_prompt.chars().take(1000).collect::<String>();
        println!("{}", "+".repeat(80));
        println!("{trimmed_prompt}");
        println!("{}", "+".repeat(80));
        let image_data = ai::get_image(&client, &trimmed_prompt, 1, ImageSize::S512x512).await?;

        println!("Saving poem and representative image.");

        let filename = ai::save_to_file(title, &poem.first_stage, &poem.second_stage, &full_text)
            .await
            .expect("Failed to save file");

        // Download/save image data
        match image_data.save(filename.as_str()).await {
            Ok(_) => println!("Saved image(s) to ./poem/{filename}"),
            Err(err) => println!("Failed to save images: {err:?}"),
        };
    }
    #[cfg(not(feature = "img_gen"))]
    {
        println!("Saving poem...");

        let filename = ai::save_to_file(
            &poem.title,
            &poem.first_stage,
            &poem.second_stage,
            &poem.poem,
        )
        .await
        .expect("Failed to save file");

        println!("Poem Saved @ {filename}");
    }

    Ok(())
}
