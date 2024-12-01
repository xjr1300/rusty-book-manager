pub mod event;

use crate::model::id::UserId;
use crate::model::role::Role;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub role: Role,
}

#[derive(Debug)]
pub struct BookOwner {
    pub id: UserId,
    pub name: String,
}

#[derive(Debug)]
pub struct CheckoutUser {
    pub id: UserId,
    pub name: String,
}
