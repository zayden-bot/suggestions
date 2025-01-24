pub mod error;
pub mod guild_manager;
pub mod slash_command;

pub use error::Error;
use error::Result;
pub use guild_manager::{SuggestionsGuildManager, SuggestionsGuildRow};
pub use slash_command::FetchSuggestions;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
