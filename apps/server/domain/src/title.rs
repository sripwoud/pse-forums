use crate::{
    validations::{too_long, too_short},
    ValidationError,
};
use nutype::nutype;

const TITLE_MIN_LEN: usize = 10;
const TITLE_MAX_LEN: usize = 50;

#[nutype(
    derive(Deserialize, Serialize, Into, TryFrom),
    sanitize(trim),
    validate(len_char_min = TITLE_MIN_LEN, len_char_max = TITLE_MAX_LEN, not_empty)
)]
pub struct Title(String);

impl From<TitleError> for ValidationError {
    fn from(error: TitleError) -> Self {
        match error {
            TitleError::LenCharMinViolated => ValidationError::Content(too_short(TITLE_MIN_LEN)),
            TitleError::LenCharMaxViolated => ValidationError::Content(too_long(TITLE_MAX_LEN)),
            TitleError::NotEmptyViolated => ValidationError::Content("Empty".to_string()),
        }
    }
}

pub type Name = Title;
