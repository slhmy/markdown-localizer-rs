use std::fs::{self, read_to_string};
use std::path::PathBuf;

pub mod image;

pub struct MarkdownContent {
    pub lines: Vec<String>,
}

impl MarkdownContent {
    pub fn load_from_file(source: PathBuf) -> anyhow::Result<Self> {
        if source.is_file() {
            if let Some(ext) = source.extension() {
                if ext.to_str() == Some("md") || ext.to_str() == Some("markdown") {
                    let mut lines = Vec::new();

                    for line in read_to_string(source.clone()).unwrap().lines() {
                        lines.push(line.to_string())
                    }
                    println!(
                        "🔎\tLoaded {} lines from '{}'",
                        lines.len(),
                        source.to_string_lossy()
                    );
                    return Ok(Self { lines });
                } else {
                    anyhow::bail!("Unsupported extension '{}'.", ext.to_str().unwrap())
                }
            }
        }
        anyhow::bail!("Source '{}' is not a file", source.to_string_lossy())
    }

    pub fn save_to_local(&self, target: PathBuf) -> anyhow::Result<()> {
        fs::File::create(&target)?;
        let mut content = "".to_string();
        for (idx, line) in self.lines.clone().into_iter().enumerate() {
            if idx != 0 {
                content += "\n";
            }
            content += &line;
        }
        fs::write(&target, content)?;
        println!("💾\tSaved '{}' in in content", target.to_string_lossy());
        
        Ok(())
    }
}
