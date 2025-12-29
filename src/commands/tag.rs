use crate::api::RepsonaClient;
use crate::cli::TagCommands;
use crate::output::{print, OutputFormat};
use anyhow::Result;

/// Parse a comma-separated string of tag IDs into a vector of u64
pub fn parse_tags(tags: &str) -> Vec<u64> {
    tags.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

pub async fn handle(client: &RepsonaClient, command: TagCommands, json: bool) -> Result<()> {
    let format = if json {
        OutputFormat::Json
    } else {
        OutputFormat::Human
    };

    match command {
        TagCommands::List => {
            let response = client.list_tags().await?;
            print(&response.data.tags, format)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_parse_tags_valid() {
        let result = parse_tags("1,2,3");
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_tags_with_spaces() {
        let result = parse_tags("1, 2, 3");
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_tags_empty() {
        let result = parse_tags("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_tags_with_invalid() {
        let result = parse_tags("1,abc,3");
        assert_eq!(result, vec![1, 3]);
    }

    // =========================================================================
    // Property-Based Tests
    // =========================================================================

    proptest! {
        #[test]
        fn prop_parse_tags_valid_numbers(tag_string in "[0-9]+(,[0-9]+)*") {
            let result = parse_tags(&tag_string);
            let expected: Vec<u64> = tag_string.split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            prop_assert_eq!(result, expected);
        }

        #[test]
        fn prop_parse_tags_with_spaces(tag_string in "[0-9]+(\\s*,\\s*[0-9]+)*") {
            let result = parse_tags(&tag_string);
            let expected: Vec<u64> = tag_string.split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            prop_assert_eq!(result, expected);
        }

        #[test]
        fn prop_parse_tags_with_invalid(mixed in "[a-z0-9]+([,][a-z0-9]+)*") {
            let result = parse_tags(&mixed);
            let expected: Vec<u64> = mixed.split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            prop_assert_eq!(result, expected);
        }

        #[test]
        fn prop_parse_tags_empty_string(input in "") {
            let result = parse_tags(&input);
            prop_assert!(result.is_empty());
        }
    }
}
