//! Editor commands for user actions.
//!
//! This module provides the command system for the rich text editor,
//! handling text manipulation, formatting, and block transformations.

use super::ast::*;

/// Represents a position in the document (block index, character offset).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub block_index: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(block_index: usize, offset: usize) -> Self {
        Self {
            block_index,
            offset,
        }
    }

    pub fn start() -> Self {
        Self {
            block_index: 0,
            offset: 0,
        }
    }
}

/// Represents a selection range in the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Position,
    pub focus: Position,
}

impl Selection {
    pub fn new(anchor: Position, focus: Position) -> Self {
        Self { anchor, focus }
    }

    pub fn collapsed(pos: Position) -> Self {
        Self {
            anchor: pos,
            focus: pos,
        }
    }

    pub fn is_collapsed(&self) -> bool {
        self.anchor == self.focus
    }

    /// Returns the selection normalized so start <= end.
    pub fn normalized(&self) -> (Position, Position) {
        if self.anchor.block_index < self.focus.block_index
            || (self.anchor.block_index == self.focus.block_index
                && self.anchor.offset <= self.focus.offset)
        {
            (self.anchor, self.focus)
        } else {
            (self.focus, self.anchor)
        }
    }
}

/// Editor command interface.
pub trait Command {
    /// Executes the command on the document.
    fn execute(&self, doc: &mut Doc, selection: &Selection) -> Result<Selection, CommandError>;

    /// Returns a human-readable description of the command.
    fn description(&self) -> &str;

    /// Downcasting support for command inspection.
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Command execution errors.
#[derive(Debug, Clone, PartialEq)]
pub enum CommandError {
    InvalidSelection,
    InvalidBlockIndex,
    EmptyDocument,
    UnsupportedOperation,
}

/// Inserts text at the current selection.
pub struct InsertText {
    pub text: String,
}

impl Command for InsertText {
    fn execute(&self, doc: &mut Doc, selection: &Selection) -> Result<Selection, CommandError> {
        if doc.blocks.is_empty() {
            return Err(CommandError::EmptyDocument);
        }

        let pos = selection.focus;
        if pos.block_index >= doc.blocks.len() {
            return Err(CommandError::InvalidBlockIndex);
        }

        let block = &mut doc.blocks[pos.block_index];

        // Insert text at the current position
        let mut char_count = 0;
        let mut insert_at = None;

        for (idx, inline) in block.children.iter_mut().enumerate() {
            match inline {
                Inline::Text { text, .. } => {
                    let text_len = text.chars().count();
                    if char_count + text_len >= pos.offset {
                        // Insert within this text node
                        let local_offset = pos.offset - char_count;
                        let chars: Vec<char> = text.chars().collect();
                        let before: String = chars.iter().take(local_offset).collect();
                        let after: String = chars.iter().skip(local_offset).collect();
                        *text = format!("{}{}{}", before, self.text, after);

                        let new_offset = pos.offset + self.text.chars().count();
                        return Ok(Selection::collapsed(Position::new(
                            pos.block_index,
                            new_offset,
                        )));
                    }
                    char_count += text_len;
                }
                Inline::HardBreak => {
                    char_count += 1;
                }
            }
            if insert_at.is_none() && char_count >= pos.offset {
                insert_at = Some(idx);
            }
        }

        // If we're at the end, append a new text node
        block.children.push(Inline::text(&self.text));
        let new_offset = pos.offset + self.text.chars().count();
        Ok(Selection::collapsed(Position::new(
            pos.block_index,
            new_offset,
        )))
    }

    fn description(&self) -> &str {
        "Insert text"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Toggles a mark (bold, italic, etc.) on the current selection.
pub struct ToggleMark {
    pub mark_type: MarkType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkType {
    Bold,
    Italic,
    Underline,
    Strike,
    Code,
}

impl Command for ToggleMark {
    fn execute(&self, doc: &mut Doc, selection: &Selection) -> Result<Selection, CommandError> {
        if doc.blocks.is_empty() {
            return Err(CommandError::EmptyDocument);
        }

        let (start, end) = selection.normalized();

        // For now, handle single-block selections
        if start.block_index != end.block_index {
            return Err(CommandError::UnsupportedOperation);
        }

        if start.block_index >= doc.blocks.len() {
            return Err(CommandError::InvalidBlockIndex);
        }

        let block = &mut doc.blocks[start.block_index];

        // Apply mark to all text nodes in range
        let mut char_count = 0;
        for inline in &mut block.children {
            if let Inline::Text { text, marks, .. } = inline {
                let text_len = text.chars().count();
                let node_range = char_count..(char_count + text_len);

                // Check if this node overlaps with selection
                if node_range.start < end.offset && node_range.end > start.offset {
                    match self.mark_type {
                        MarkType::Bold => marks.bold = !marks.bold,
                        MarkType::Italic => marks.italic = !marks.italic,
                        MarkType::Underline => marks.underline = !marks.underline,
                        MarkType::Strike => marks.strike = !marks.strike,
                        MarkType::Code => marks.code = !marks.code,
                    }
                }

                char_count += text_len;
            } else if let Inline::HardBreak = inline {
                char_count += 1;
            }
        }

        Ok(selection.clone())
    }

    fn description(&self) -> &str {
        match self.mark_type {
            MarkType::Bold => "Toggle bold",
            MarkType::Italic => "Toggle italic",
            MarkType::Underline => "Toggle underline",
            MarkType::Strike => "Toggle strikethrough",
            MarkType::Code => "Toggle code",
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Converts the current block to a different type.
pub struct SetBlockType {
    pub block_type: BlockKind,
}

impl Command for SetBlockType {
    fn execute(&self, doc: &mut Doc, selection: &Selection) -> Result<Selection, CommandError> {
        if doc.blocks.is_empty() {
            return Err(CommandError::EmptyDocument);
        }

        let pos = selection.focus;
        if pos.block_index >= doc.blocks.len() {
            return Err(CommandError::InvalidBlockIndex);
        }

        let block = &mut doc.blocks[pos.block_index];
        block.kind = self.block_type.clone();

        Ok(selection.clone())
    }

    fn description(&self) -> &str {
        "Change block type"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Inserts a new block at the current position.
pub struct InsertBlock {
    pub block_type: BlockKind,
}

impl Command for InsertBlock {
    fn execute(&self, doc: &mut Doc, selection: &Selection) -> Result<Selection, CommandError> {
        let pos = selection.focus;
        if pos.block_index >= doc.blocks.len() {
            return Err(CommandError::InvalidBlockIndex);
        }

        let new_block = Block::new(self.block_type.clone());
        doc.blocks.insert(pos.block_index + 1, new_block);

        Ok(Selection::collapsed(Position::new(pos.block_index + 1, 0)))
    }

    fn description(&self) -> &str {
        "Insert block"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Splits the current block at the cursor position (Enter key).
pub struct SplitBlock;

impl Command for SplitBlock {
    fn execute(&self, doc: &mut Doc, selection: &Selection) -> Result<Selection, CommandError> {
        if doc.blocks.is_empty() {
            return Err(CommandError::EmptyDocument);
        }

        let pos = selection.focus;
        if pos.block_index >= doc.blocks.len() {
            return Err(CommandError::InvalidBlockIndex);
        }

        let current_block = &doc.blocks[pos.block_index];

        // Create new block of the same type
        let mut new_block = Block::new(current_block.kind.clone());
        new_block.align = current_block.align.clone();

        // Split content at cursor position
        let block = &mut doc.blocks[pos.block_index];
        let mut char_count = 0;
        let mut split_index = None;
        let mut split_offset = 0;

        for (idx, inline) in block.children.iter().enumerate() {
            match inline {
                Inline::Text { text, .. } => {
                    let text_len = text.chars().count();
                    if char_count + text_len >= pos.offset {
                        split_index = Some(idx);
                        split_offset = pos.offset - char_count;
                        break;
                    }
                    char_count += text_len;
                }
                Inline::HardBreak => {
                    char_count += 1;
                }
            }
        }

        if let Some(idx) = split_index {
            // Split the text node
            if let Inline::Text { text, marks, link } = &block.children[idx].clone() {
                let chars: Vec<char> = text.chars().collect();
                let before: String = chars.iter().take(split_offset).collect();
                let after: String = chars.iter().skip(split_offset).collect();

                block.children[idx] = Inline::Text {
                    text: before,
                    marks: marks.clone(),
                    link: link.clone(),
                };

                if !after.is_empty() {
                    new_block.children.push(Inline::Text {
                        text: after,
                        marks: marks.clone(),
                        link: link.clone(),
                    });
                }

                // Move remaining inlines to new block
                new_block.children.extend(block.children.drain((idx + 1)..));
            }
        }

        doc.blocks.insert(pos.block_index + 1, new_block);

        Ok(Selection::collapsed(Position::new(pos.block_index + 1, 0)))
    }

    fn description(&self) -> &str {
        "Split block (Enter)"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Deletes content in the current selection.
pub struct DeleteSelection;

impl Command for DeleteSelection {
    fn execute(&self, doc: &mut Doc, selection: &Selection) -> Result<Selection, CommandError> {
        if doc.blocks.is_empty() {
            return Err(CommandError::EmptyDocument);
        }

        let (start, end) = selection.normalized();

        // Single block deletion
        if start.block_index == end.block_index {
            if start.block_index >= doc.blocks.len() {
                return Err(CommandError::InvalidBlockIndex);
            }

            let block = &mut doc.blocks[start.block_index];
            let mut char_count = 0;

            for inline in &mut block.children {
                if let Inline::Text { text, .. } = inline {
                    let text_len = text.chars().count();
                    let node_range = char_count..(char_count + text_len);

                    if node_range.start < end.offset && node_range.end > start.offset {
                        let chars: Vec<char> = text.chars().collect();
                        let delete_start = start.offset.saturating_sub(char_count);
                        let delete_end = (end.offset - char_count).min(text_len);

                        let before: String = chars.iter().take(delete_start).collect();
                        let after: String = chars.iter().skip(delete_end).collect();
                        *text = format!("{}{}", before, after);
                    }

                    char_count += text_len;
                }
            }

            // Remove empty text nodes
            block.children.retain(|inline| {
                if let Inline::Text { text, .. } = inline {
                    !text.is_empty()
                } else {
                    true
                }
            });

            return Ok(Selection::collapsed(start));
        }

        // Multi-block deletion
        Err(CommandError::UnsupportedOperation)
    }

    fn description(&self) -> &str {
        "Delete selection"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Inserts a link at the current selection.
pub struct InsertLink {
    pub href: String,
    pub title: Option<String>,
    pub target_blank: bool,
}

impl Command for InsertLink {
    fn execute(&self, doc: &mut Doc, selection: &Selection) -> Result<Selection, CommandError> {
        if doc.blocks.is_empty() {
            return Err(CommandError::EmptyDocument);
        }

        let (start, end) = selection.normalized();

        // Single block only
        if start.block_index != end.block_index {
            return Err(CommandError::UnsupportedOperation);
        }

        if start.block_index >= doc.blocks.len() {
            return Err(CommandError::InvalidBlockIndex);
        }

        let block = &mut doc.blocks[start.block_index];
        let link = Link {
            href: self.href.clone(),
            title: self.title.clone(),
            target_blank: self.target_blank,
        };

        let mut char_count = 0;
        for inline in &mut block.children {
            if let Inline::Text {
                text,
                link: ref mut inline_link,
                ..
            } = inline
            {
                let text_len = text.chars().count();
                let node_range = char_count..(char_count + text_len);

                if node_range.start < end.offset && node_range.end > start.offset {
                    *inline_link = Some(link.clone());
                }

                char_count += text_len;
            }
        }

        Ok(selection.clone())
    }

    fn description(&self) -> &str {
        "Insert link"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_inserts_text() {
        let mut doc = Doc::default();
        let selection = Selection::collapsed(Position::new(0, 0));
        let cmd = InsertText {
            text: "Hello".to_string(),
        };

        let result = cmd.execute(&mut doc, &selection);
        assert!(result.is_ok());
        assert!(doc.blocks[0].children.len() > 0);
    }

    #[test]
    fn it_toggles_bold() {
        let mut doc = Doc::default();
        doc.blocks[0] = Block::new_paragraph().with_text("Test");
        let selection = Selection::new(Position::new(0, 0), Position::new(0, 4));
        let cmd = ToggleMark {
            mark_type: MarkType::Bold,
        };

        let result = cmd.execute(&mut doc, &selection);
        assert!(result.is_ok());

        if let Inline::Text { marks, .. } = &doc.blocks[0].children[0] {
            assert!(marks.bold);
        }
    }

    #[test]
    fn it_splits_block() {
        let mut doc = Doc::default();
        doc.blocks[0] = Block::new_paragraph().with_text("Hello world");
        let selection = Selection::collapsed(Position::new(0, 6));
        let cmd = SplitBlock;

        let result = cmd.execute(&mut doc, &selection);
        assert!(result.is_ok());
        assert_eq!(doc.blocks.len(), 2);
    }

    #[test]
    fn it_changes_block_type() {
        let mut doc = Doc::default();
        let selection = Selection::collapsed(Position::new(0, 0));
        let cmd = SetBlockType {
            block_type: BlockKind::Heading { level: 1 },
        };

        let result = cmd.execute(&mut doc, &selection);
        assert!(result.is_ok());
        assert!(matches!(
            doc.blocks[0].kind,
            BlockKind::Heading { level: 1 }
        ));
    }

    #[test]
    fn selection_normalized_works() {
        let sel = Selection::new(Position::new(1, 5), Position::new(0, 2));
        let (start, end) = sel.normalized();
        assert_eq!(start, Position::new(0, 2));
        assert_eq!(end, Position::new(1, 5));
    }
}
