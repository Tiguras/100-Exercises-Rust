// The three possible statuses a ticket can have.
#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

// TryFrom is a trait for conversions that might fail.
// This says: "You can try to convert a String into a Status, but it might fail."
//
// Compare to From (no "Try"):
//   From<A> for B     = A can ALWAYS become B (infallible)
//   TryFrom<A> for B  = A MIGHT become B, or it might fail (fallible)
//
// When you implement TryFrom, you get .try_from() and .try_into() methods.
impl TryFrom<String> for Status {
    // If the conversion fails, what error type do we return?
    // We return ParseStatusError (defined below).
    type Error = ParseStatusError;

    // The actual conversion logic.
    // Takes a String, returns either Ok(Status) or Err(ParseStatusError).
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            // Valid inputs -> return Ok with the corresponding Status
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),

            // Anything else -> return Err with details about what went wrong
            // We store the invalid string so we can show it in the error message.
            _ => Err(ParseStatusError {
                invalid_status: value,
            }),
        }
    }
}

// This is the error type for when Status::try_from() fails.
//
// #[derive(thiserror::Error)] generates the std::error::Error implementation.
//
// #[error("...")] defines what .to_string() returns.
// {invalid_status} gets replaced with the value of the invalid_status field.
//
// So if someone passes "banana":
//   ParseStatusError { invalid_status: "banana".to_string() }
//   .to_string() -> "`banana` is not a valid status. Use one of: ToDo, InProgress, Done"
#[derive(Debug, thiserror::Error)]
#[error("`{invalid_status}` is not a valid status. Use one of: ToDo, InProgress, Done")]
pub struct ParseStatusError {
    // We store the bad input so we can include it in the error message.
    // This is private (no `pub`), so only this module can create ParseStatusError.
    invalid_status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
        // If you tried Status::try_from("banana".to_string()), you'd get:
        // Err(ParseStatusError { invalid_status: "banana" })
    }

    #[test]
    fn test_try_from_string_failure() {
        assert!(Status::try_from("somesh".to_string()).is_err());
    }

    #[test]
    fn test_tryin_from_string_failure_message() {
        let err = Status::try_from("somesh".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "`somesh` is not a valid status. Use one of: ToDo, InProgress, Done");
    }
}
