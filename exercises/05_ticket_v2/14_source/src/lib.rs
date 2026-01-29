use crate::status::Status;
use crate::status::ParseStatusError;  // This is the error type that Status::try_from() returns when it fails
use std::error::Error;

mod status;

// This enum lists all the ways that Ticket::new() can fail.
// thiserror::Error auto-generates boilerplate code for us.
#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    // These four are simple errors with no "cause" - they just happen.
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,

    // This one is different - it WRAPS another error (ParseStatusError).
    //
    // #[from] does TWO things:
    //   1. Lets you use ? to auto-convert ParseStatusError -> TicketNewError
    //      (generates: impl From<ParseStatusError> for TicketNewError)
    //   2. Makes source() return the inner ParseStatusError
    //      (so you can ask "what caused this error?" and get an answer)
    //
    // #[error("{source}")] means: when you print this error, print the inner error's message.
    // So if ParseStatusError says "`banana` is not valid", that's what you'll see.
    #[error("{source}")]
    InvalidStatus {
        #[from]
        source: ParseStatusError
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    // This function returns Result<Ticket, TicketNewError>
    // That means: either a Ticket, or one of the errors defined above.
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // HERE'S WHERE THE MAGIC HAPPENS:
        //
        // Status::try_from(status) returns Result<Status, ParseStatusError>
        //
        // If status is "todo" -> Ok(Status::ToDo)
        // If status is "banana" -> Err(ParseStatusError { invalid_status: "banana" })
        //
        // The ? operator does this:
        //   - If Ok(value) -> unwrap it, continue with value
        //   - If Err(e) -> convert e to our error type and return early
        //
        // But wait - we need to return TicketNewError, not ParseStatusError!
        //
        // This is where #[from] saves us. Because we wrote:
        //   InvalidStatus { #[from] source: ParseStatusError }
        //
        // Rust generated this code automatically:
        //   impl From<ParseStatusError> for TicketNewError {
        //       fn from(e: ParseStatusError) -> TicketNewError {
        //           TicketNewError::InvalidStatus { source: e }
        //       }
        //   }
        //
        // So when ? sees a ParseStatusError, it calls .into() which converts it
        // to TicketNewError::InvalidStatus, wrapping the original error inside.
        //
        // WITHOUT #[from], you'd have to write this manually:
        //   let status = match Status::try_from(status) {
        //       Ok(s) => s,
        //       Err(e) => return Err(TicketNewError::InvalidStatus { source: e }),
        //   };
        let status = Status::try_from(status)?;

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        // Create a ticket with invalid status "invalid"
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();

        // err is TicketNewError::InvalidStatus { source: ParseStatusError }
        //
        // err.to_string() prints the error message.
        // Because we wrote #[error("{source}")], it prints the INNER error's message.
        // ParseStatusError's message is: "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );

        // err.source() returns the inner error that caused this one.
        // Because we used #[from], it returns Some(&ParseStatusError).
        // If we hadn't wrapped another error, this would return None.
        assert!(err.source().is_some());
    }
}
