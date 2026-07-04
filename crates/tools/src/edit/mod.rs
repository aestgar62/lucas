//


pub struct FileChange {
    pub file_path: String,
    pub changes: Vec<Change>,
}

pub struct Change {
    pub old_content: String,
    pub new_content: String,
}

fn replace_block(content: &str, old_block: &str, new_block: &str) -> String {
    content.replace(old_block, new_block)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_block() {
        let content = "for example, content with a block of text that needs to be replaced.";
        let old_block = "block of text";
        let new_block = "new content";
        let result = replace_block(content, old_block, new_block);
        assert_eq!(result, "for example, content with a new content that needs to be replaced.");
    }
}