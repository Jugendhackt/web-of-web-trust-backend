use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RuegenInformation {
    pub medium: String,
    pub identified: String,
    pub title: String,
    pub ziffer: String,
    pub year: u16,
}
