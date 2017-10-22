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
use common::{
    APIError,
    parse_response
};
use api_v2::types::{
    Character,
    CharacterBackstory,
    CharacterCore,
    CharacterCrafting,
    CharacterEquipment,
    CharacterInventory,
    CharacterRecipes,
    CharacterSkills,
    CharacterSpecializations,
    CharacterTraining,
    SABProgress,
};

use reqwest::StatusCode;

/// Obtain the requested endpoint
macro_rules! get_endpoint {
    ("names") => {"/v2/characters"};
    ("character", $id: expr) => {format!("/v2/characters/{}", $id)};
    ("backstory", $id: expr) => {format!("/v2/characters/{}/backstory", $id)};
    ("core", $id: expr) => {format!("/v2/characters/{}/core", $id)};
    ("crafting", $id: expr) => {format!("/v2/characters/{}/crafting", $id)};
    ("equip", $id: expr) => {format!("/v2/characters/{}/equipment", $id)};
    ("hp", $id: expr) => {format!("/v2/characters/{}/heropoints", $id)};
    ("inv", $id: expr) => {format!("/v2/characters/{}/inventory", $id)};
    ("recipes", $id: expr) => {format!("/v2/characters/{}/recipes", $id)};
    ("sab", $id: expr) => {format!("/v2/characters/{}/sab", $id)};
    ("skills", $id: expr) => {format!("/v2/characters/{}/skills", $id)};
    ("specs", $id: expr) => {format!("/v2/characters/{}/specializations", $id)};
    ("training", $id: expr) => {format!("/v2/characters/{}/training", $id)};
}

//TODO percent-encode character names


/// Obtain summary of details for the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character name to fetch
pub fn get_character(
    client: &APIClient,
    name: &str
) -> Result<Character, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("character", name))
        .expect("failed to get character");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain backstory answers for a character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character to obtain backstory for
pub fn get_character_backstory(
    client: &APIClient,
    name: &str
) -> Result<CharacterBackstory, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("backstory", name))
        .expect("failed to get character backstory");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![
            StatusCode::NotFound,
            StatusCode::Forbidden,
            StatusCode::BadRequest
        ]
    )
}


/// Obtain core information for the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character to fetch
pub fn get_character_core(
    client: &APIClient,
    name: &str
) -> Result<CharacterCore, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("core", name))
        .expect("failed to get character information");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![
            StatusCode::NotFound,
            StatusCode::Forbidden,
            StatusCode::BadRequest
        ]
    )
}

/// Obtain crafting disciplines for the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character to fetch
pub fn get_character_crafting(
    client: &APIClient,
    name: &str
) -> Result<CharacterCrafting, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("crafting", name))
        .expect("failed to get crafting disciplines");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![
            StatusCode::NotFound,
            StatusCode::Forbidden,
            StatusCode::BadRequest
        ]
    )
}

/// Obtain equipment on the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character to fetch
pub fn get_character_equipment(
    client: &APIClient,
    name: &str
) -> Result<CharacterEquipment, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("equip", name))
        .expect("failed to get character equipment");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![
            StatusCode::NotFound,
            StatusCode::Forbidden,
            StatusCode::BadRequest
        ]
    )
}

/// Obtain hero points unlocked for the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character name to fetch
pub fn get_character_heropoints(
    client: &APIClient,
    name: &str
) -> Result<Vec<String>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("hp", name))
        .expect("failed to get hero points");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![
            StatusCode::NotFound,
            StatusCode::Forbidden,
            StatusCode::BadRequest
        ]
    )
}

/// Obtain inventory of the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character to fetch
pub fn get_character_inventory(
    client: &APIClient,
    name: &str
) -> Result<CharacterInventory, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("inv", name))
        .expect("failed to get character inventory");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![
            StatusCode::NotFound,
            StatusCode::Forbidden,
            StatusCode::BadRequest
        ]
    )
}

/// Obtain character names for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_character_names(
    client: &APIClient
) -> Result<Vec<String>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("names"))
        .expect("failed to get character names");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked recipes for the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character name to fetch
pub fn get_character_recipes(
    client: &APIClient,
    name: &str
) -> Result<CharacterRecipes, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("recipes", name))
        .expect("failed to get unlocked recipes");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![
            StatusCode::NotFound,
            StatusCode::Forbidden,
            StatusCode::BadRequest
        ]
    )
}

/// Obtain SAB progress for the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character name to fetch
pub fn get_character_sab(
    client: &APIClient,
    name: &str
) -> Result<SABProgress, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("sab", name))
        .expect("failed to get SAB progress");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain skills (PVE, PVP, WvW) of the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character to fetch
pub fn get_character_skills(
    client: &APIClient,
    name: &str
) -> Result<CharacterSkills, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("skills", name))
        .expect("failed to get character skills");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![
            StatusCode::NotFound,
            StatusCode::Forbidden,
            StatusCode::BadRequest
        ]
    )
}

/// Obtain specializations of the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character to fetch
pub fn get_character_specializations(
    client: &APIClient,
    name: &str
) -> Result<CharacterSpecializations, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("specs", name))
        .expect("failed to get character specializations");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![
            StatusCode::NotFound,
            StatusCode::Forbidden,
            StatusCode::BadRequest
        ]
    )
}

/// Obtain skill trees of the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
/// * `name` - Character to fetch
pub fn get_character_training(
    client: &APIClient,
    name: &str
) -> Result<CharacterTraining, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("training", name))
        .expect("failed to get character training");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![
            StatusCode::NotFound,
            StatusCode::Forbidden,
            StatusCode::BadRequest
        ]
    )
}

#[cfg(test)]
mod tests {
    use std::env;
    use client::APIClient;
    use api_v2::characters::*;

    macro_rules! parse_test {
        ($result:expr) => {
            match $result {
                Ok(_) => assert!(true),
                Err(e) => panic!(e.description().to_string()),
            };
        }
    }

    fn setup_client() -> APIClient {
        match env::var("TOKEN") {
            Ok(token) => APIClient::new("en", Some(token.to_string())),
            Err(_) => panic!("Need a token to test endpoint"),
        }
    }

    fn set_name() -> String {
        match env::var("CHAR_NAME") {
            Ok(name) => name,
            Err(_) => panic!("Need a character name to test endpoint"),
        }
    }

    #[test]
    fn character() {
        let client = setup_client();
        let name = set_name();
        let result = get_character(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_backstory() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_backstory(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_core() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_core(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_crafting() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_crafting(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_equipment() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_equipment(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_heropoints() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_heropoints(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_inventory() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_inventory(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_names() {
        let client = setup_client();
        let result = get_character_names(&client);
        parse_test!(result);
    }

    #[test]
    fn character_recipes() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_recipes(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_sab() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_sab(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_skills() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_skills(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_specializations() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_specializations(&client, &name.as_str());
        parse_test!(result);
    }

    #[test]
    fn character_training() {
        let client = setup_client();
        let name = set_name();
        let result = get_character_training(&client, &name.as_str());
        parse_test!(result);
    }
}
