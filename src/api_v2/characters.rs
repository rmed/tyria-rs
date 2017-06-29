// MIT License
//
// Copyright (c) 2017 Rafael Medina García <rafamedgar@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Character endpoints
/// These require an API key to view

use client::APIClient;
use util::{
    number_to_param,
    numbers_to_param,
    string_to_param,
    strings_to_param,
    parse_response
};
use types::{
    APIError,
    Character,
};

use reqwest::StatusCode;

/// Obtain the requested endpoint
macro_rules! get_endpoint {
    ("names") => {"/v2/characters"};
    ("character", $id: expr) => {format!("/v2/characters?{}", $id)};
}


/// Obtain character names for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::character::get_character_names;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_characters = get_character_names(&client);
/// ```
pub fn get_character_names(client: &APIClient)
    -> Result<Vec<String>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("names"))
        .expect("failed to get character names");

    parse_response::<Vec<String>>(
        &mut response,
        StatusCode::Ok,
        StatusCode::Forbidden
    )
}

/// Obtain details for the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `name` - Character name to fetch
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::character::get_character;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_character = get_character(&client, "My Char");
/// ```
pub fn get_achievement(client: &APIClient, name: &str)
    -> Result<Character, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("character", param))
        .expect("failed to get character");

    //TODO this endpoint has Ok, NotFound and NotAuthorized, change utility?
    parse_response::<Character>(
        &mut response,
        StatusCode::Ok,
        StatusCode::NotFound
    )
}
