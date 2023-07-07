use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ForeingUser {
    pub address: Address,
    pub company: Company,
    pub email: String,
    pub id: usize,
    pub name: String,
    pub phone: String,
    pub username: String,
    pub website: String,
}

#[derive(Deserialize, Debug)]
pub struct Address {
    pub city: String,
    pub geo: LatLong,
    pub street: String,
    pub suite: String,
    pub zipcode: String,
}

#[derive(Deserialize, Debug)]
pub struct LatLong {
    pub lat: String,
    pub lng: String,
}

#[derive(Deserialize, Debug)]
pub struct Company {
    pub bs: String,
    // This field is omitted on purpose, because I want to see what happens when additional stuff
    // get included in the response that our application does not implement yet
    // catchPhrase: String,
    pub name: String,
}
