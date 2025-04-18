use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserDto {
    #[validate(length(min =1, max = 50))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeleteNameIndexDto {
    #[validate(range(min = 0))]
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeleteNameDto {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
}
