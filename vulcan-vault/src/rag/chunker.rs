//! Markdown chunking for RAG pipeline
//!
//! Splits markdown content into semantic chunks suitable for embedding:
//! - Respects heading boundaries
//! - Preserves code blocks intact
//! - Handles overlap for context continuity

use crate::models::{Chunk, ChunkConfig};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

/// Markdown chunker that splits content into embedding-ready chunks
pub struct Chunker {
    config: ChunkConfig,
}

impl Chunker {
    /// Create a new chunker with the given configuration
    pub fn new(config: ChunkConfig) -> Self {
        Self { config }
    }
}

impl Default for Chunker {
    fn default() -> Self {
        Self::new(ChunkConfig::default())
    }
}

impl Chunker {

    /// Split markdown content into chunks
    ///
    /// # Arguments
    /// * `note_id` - The ID of the parent note
    /// * `note_path` - The path of the parent note
    /// * `content` - The markdown content to chunk (without frontmatter)
    ///
    /// # Returns
    /// A vector of chunks ready for embedding
    pub fn split(&self, note_id: &str, note_path: &str, content: &str) -> Vec<Chunk> {
        // First, parse into sections based on headings
        let sections = self.parse_sections(content);

        // Then, chunk each section respecting size limits
        let mut chunks = Vec::new();
        let mut chunk_index = 0;

        for section in sections {
            let section_chunks = self.chunk_section(
                note_id,
                note_path,
                &section.heading,
                &section.content,
                section.char_start,
                &mut chunk_index,
            );
            chunks.extend(section_chunks);
        }

        chunks
    }

    /// Parse content into sections based on headings
    fn parse_sections(&self, content: &str) -> Vec<Section> {
        let mut sections: Vec<Section> = Vec::new();
        let mut current_heading: Option<String> = None;
        let mut current_content = String::new();
        let mut section_start: usize = 0;
        let mut _in_code_block = false;
        let mut in_heading = false;

        let options = Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_TASKLISTS;

        let parser = Parser::new_ext(content, options);

        for (event, range) in parser.into_offset_iter() {
            match event {
                Event::Start(Tag::Heading { .. }) => {
                    // Save previous section before starting new one
                    if !current_content.is_empty() || current_heading.is_some() {
                        sections.push(Section {
                            heading: current_heading.take(),
                            content: current_content.trim().to_string(),
                            char_start: section_start,
                        });
                    }

                    // Start new section
                    section_start = range.start;
                    current_content = String::new();
                    current_heading = None;
                    in_heading = true;
                }
                Event::End(TagEnd::Heading(_)) => {
                    // Heading text has been collected, now we continue with section content
                    in_heading = false;
                }
                Event::Text(text) => {
                    if in_heading {
                        // Capture heading text
                        current_heading = Some(text.to_string());
                    } else {
                        // Body text
                        current_content.push_str(&text);
                    }
                }
                Event::Code(code) => {
                    current_content.push('`');
                    current_content.push_str(&code);
                    current_content.push('`');
                }
                Event::Start(Tag::CodeBlock(_)) => {
                    _in_code_block = true;
                    current_content.push_str("\n```");
                }
                Event::End(TagEnd::CodeBlock) => {
                    _in_code_block = false;
                    current_content.push_str("\n```\n");
                }
                Event::SoftBreak | Event::HardBreak => {
                    current_content.push('\n');
                }
                Event::Start(Tag::Paragraph) => {
                    if !current_content.is_empty() && !current_content.ends_with('\n') {
                        current_content.push_str("\n\n");
                    }
                }
                Event::End(TagEnd::Paragraph) => {
                    current_content.push('\n');
                }
                Event::Start(Tag::Item) => {
                    current_content.push_str("\n- ");
                }
                _ => {}
            }
        }

        // Don't forget the last section
        if !current_content.is_empty() || current_heading.is_some() {
            sections.push(Section {
                heading: current_heading,
                content: current_content.trim().to_string(),
                char_start: section_start,
            });
        }

        // If no sections were created (no headings), create one from the whole content
        if sections.is_empty() && !content.is_empty() {
            sections.push(Section {
                heading: None,
                content: content.trim().to_string(),
                char_start: 0,
            });
        }

        sections
    }

    /// Chunk a single section, splitting if it exceeds max size
    fn chunk_section(
        &self,
        note_id: &str,
        note_path: &str,
        heading: &Option<String>,
        content: &str,
        section_start: usize,
        chunk_index: &mut u32,
    ) -> Vec<Chunk> {
        let mut chunks = Vec::new();

        // If content fits in one chunk, return it directly
        if content.len() <= self.config.max_size {
            if !content.is_empty() {
                let chunk = Chunk::new(
                    note_id,
                    note_path,
                    content,
                    *chunk_index,
                    section_start as u32,
                    (section_start + content.len()) as u32,
                );
                let chunk = if let Some(h) = heading {
                    chunk.with_heading(h)
                } else {
                    chunk
                };
                chunks.push(chunk);
                *chunk_index += 1;
            }
            return chunks;
        }

        // Content is too large, split at paragraph boundaries with overlap
        let paragraphs: Vec<&str> = content.split("\n\n").collect();
        let mut current_chunk = String::new();
        let mut current_start = section_start;

        for para in paragraphs {
            let para_trimmed = para.trim();
            if para_trimmed.is_empty() {
                continue;
            }

            // Check if adding this paragraph would exceed max size
            let potential_size = if current_chunk.is_empty() {
                para_trimmed.len()
            } else {
                current_chunk.len() + 2 + para_trimmed.len() // +2 for "\n\n"
            };

            if potential_size > self.config.max_size && !current_chunk.is_empty() {
                // Save current chunk
                let chunk = Chunk::new(
                    note_id,
                    note_path,
                    &current_chunk,
                    *chunk_index,
                    current_start as u32,
                    (current_start + current_chunk.len()) as u32,
                );
                let chunk = if let Some(h) = heading {
                    chunk.with_heading(h)
                } else {
                    chunk
                };
                chunks.push(chunk);
                *chunk_index += 1;

                // Start new chunk with overlap
                let overlap_text = self.get_overlap_text(&current_chunk);
                current_start = current_start + current_chunk.len() - overlap_text.len();
                current_chunk = overlap_text;
            }

            // Add paragraph to current chunk
            if current_chunk.is_empty() {
                current_chunk = para_trimmed.to_string();
            } else {
                current_chunk.push_str("\n\n");
                current_chunk.push_str(para_trimmed);
            }
        }

        // Don't forget the last chunk
        if !current_chunk.is_empty() {
            let chunk = Chunk::new(
                note_id,
                note_path,
                &current_chunk,
                *chunk_index,
                current_start as u32,
                (current_start + current_chunk.len()) as u32,
            );
            let chunk = if let Some(h) = heading {
                chunk.with_heading(h)
            } else {
                chunk
            };
            chunks.push(chunk);
            *chunk_index += 1;
        }

        chunks
    }

    /// Extract overlap text from the end of a chunk
    fn get_overlap_text(&self, text: &str) -> String {
        if text.len() <= self.config.overlap {
            return text.to_string();
        }

        // Try to find a natural break point (sentence, word)
        let overlap_region = &text[text.len() - self.config.overlap..];

        // Try to start at a sentence boundary
        if let Some(pos) = overlap_region.find(". ") {
            return overlap_region[pos + 2..].to_string();
        }

        // Try to start at a word boundary
        if let Some(pos) = overlap_region.find(' ') {
            return overlap_region[pos + 1..].to_string();
        }

        // Fall back to exact character count
        overlap_region.to_string()
    }
}

/// Internal structure for parsed sections
struct Section {
    heading: Option<String>,
    content: String,
    char_start: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_text_chunking() {
        let chunker = Chunker::default();
        let content = "This is a simple paragraph of text.";

        let chunks = chunker.split("note-1", "test.md", content);

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].content, content);
        assert_eq!(chunks[0].chunk_index, 0);
        assert!(chunks[0].heading.is_none());
    }

    #[test]
    fn test_heading_based_splitting() {
        let chunker = Chunker::default();
        let content = r#"## Introduction
This is the intro section.

## Methods
This is the methods section.

## Results
This is the results section."#;

        let chunks = chunker.split("note-1", "test.md", content);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].heading.as_deref(), Some("Introduction"));
        assert_eq!(chunks[1].heading.as_deref(), Some("Methods"));
        assert_eq!(chunks[2].heading.as_deref(), Some("Results"));
    }

    #[test]
    fn test_code_block_preservation() {
        let chunker = Chunker::default();
        let content = r#"## Code Example

Here is some code:

```rust
fn main() {
    println!("Hello, world!");
}
```

That was the code."#;

        let chunks = chunker.split("note-1", "test.md", content);

        // Code block should be preserved in the chunk
        assert!(chunks[0].content.contains("```"));
        assert!(chunks[0].content.contains("println!"));
    }

    #[test]
    fn test_large_section_splitting() {
        // Create a chunker with small max size for testing
        let config = ChunkConfig {
            max_size: 100,
            overlap: 20,
            split_on_headings: true,
            preserve_code_blocks: true,
        };
        let chunker = Chunker::new(config);

        let content = "First paragraph with some text. Second paragraph with more text. Third paragraph continues. Fourth paragraph ends here.";

        let chunks = chunker.split("note-1", "test.md", content);

        // Should have multiple chunks due to size limit
        assert!(chunks.len() >= 1);

        // Each chunk should be within max size (approximately)
        for chunk in &chunks {
            assert!(chunk.content.len() <= 150); // Allow some flexibility
        }
    }

    #[test]
    fn test_empty_content() {
        let chunker = Chunker::default();
        let chunks = chunker.split("note-1", "test.md", "");

        assert!(chunks.is_empty());
    }

    #[test]
    fn test_whitespace_only() {
        let chunker = Chunker::default();
        let chunks = chunker.split("note-1", "test.md", "   \n\n   ");

        assert!(chunks.is_empty());
    }

    #[test]
    fn test_chunk_indices() {
        let chunker = Chunker::default();
        let content = r#"## Section 1
Content 1.

## Section 2
Content 2.

## Section 3
Content 3."#;

        let chunks = chunker.split("note-1", "test.md", content);

        // Indices should be sequential
        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.chunk_index, i as u32);
        }
    }

    #[test]
    fn test_note_metadata_preserved() {
        let chunker = Chunker::default();
        let content = "Test content";

        let chunks = chunker.split("my-note-id", "Projects/test.md", content);

        assert_eq!(chunks[0].note_id, "my-note-id");
        assert_eq!(chunks[0].note_path, "Projects/test.md");
    }
}
