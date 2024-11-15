use crate::book::{Book, BookItem};
use crate::renderer::{RenderContext, Renderer};
use anyhow::Result;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

/// foo
#[derive(Default)]
pub struct SingleFileRenderer;

impl SingleFileRenderer {
    /// bar
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for SingleFileRenderer {
    fn name(&self) -> &'static str {
        "single-file"
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        let book = &ctx.book;
        let destination = &ctx.destination;

        // Create output directory if it doesn't exist
        fs::create_dir_all(destination)?;

        // Generate Markdown version
        self.render_markdown(book, destination)?;

        Ok(())
    }
}

impl SingleFileRenderer {
    fn render_markdown(&self, book: &Book, destination: &PathBuf) -> io::Result<()> {
        let mut output = String::new();

        // Add default title if none exists
        output.push_str("# Documentation\n\n");

        // Recursively process all chapters
        for item in &book.sections {
            self.process_item_markdown(item, 0, &mut output);
        }

        // Write to file
        let output_path = destination.join("book.md");
        let mut file = File::create(output_path)?;
        file.write_all(output.as_bytes())?;

        Ok(())
    }

    fn process_item_markdown(&self, item: &BookItem, level: usize, output: &mut String) {
        match item {
            BookItem::Chapter(ref chapter) => {
                // Add chapter title with appropriate heading level
                output.push_str(&format!("{} {}\n\n", "#".repeat(level + 1), chapter.name));

                // Add chapter content
                output.push_str(&chapter.content);
                output.push_str("\n\n");

                // Process sub-chapters
                for sub_item in &chapter.sub_items {
                    self.process_item_markdown(sub_item, level + 1, output);
                }
            }
            BookItem::Separator => {
                output.push_str("---\n\n");
            }
            BookItem::PartTitle(title) => {
                // Add part title with appropriate heading level
                output.push_str(&format!("{} {}\n\n", "#".repeat(level + 1), title));
            }
        }
    }
}
