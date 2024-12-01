use strum::{AsRefStr, EnumIter, EnumString};

#[derive(Debug, Default, PartialEq, Eq, Hash, EnumString, AsRefStr, EnumIter)]
pub enum Role {
    Admin,
    #[default]
    User,
}
