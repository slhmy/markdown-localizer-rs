use reqwest::blocking::Client;
use std::fs::{create_dir_all, File};

use regex::Regex;

use super::MarkdownContent;

static MD_IMAGE_REGEX: &str = r"!\[(?P<name>[^\]]+)\]\((?P<link>https?://[^\)]+)\)";

impl MarkdownContent {
    pub fn localize_image(&mut self) -> anyhow::Result<()> {
        create_dir_all("images")?;
        println!("🗃️\tFinding image with remote link");

        let regex = Regex::new(MD_IMAGE_REGEX).unwrap();
        let mut new_lines: Vec<String> = vec![];
        let reqwest_client = Client::new();
        for mut line in self.lines.clone().into_iter() {
            let line_clone = line.clone();
            let captures = regex.captures_iter(&line_clone);
            for capture in captures {
                let image_name = &capture["name"];
                let remote_link = &capture["link"];
                println!("🖼️\tFound '{}' with link '{}'", image_name, remote_link);

                let local_link = format!("images/{}", image_name);
                
                let mut response = reqwest_client
                    .get(remote_link)
                    .header("User-Agent", "PostmanRuntime/7.32.3")
                    .send()?;

                if response.status().is_success() {
                    let mut file = File::create(&local_link)?;
                    response.copy_to(&mut file)?;
                    println!("⏬\tImage '{}' downloaded", image_name);
                } else {
                    anyhow::bail!(
                        "Failed download image '{}', {}",
                        image_name,
                        response.status()
                    );
                }

                let raw_image_content = format!("![{}]({})", image_name, remote_link);
                let new_image_content = format!("![{}]({})", image_name, local_link);
                line = line.replace(&raw_image_content, &new_image_content);
                println!("📝\tApply changes for image '{}' in line", image_name);
            }
            new_lines.push(line);
        }

        self.lines = new_lines;
        Ok(())
    }
}
