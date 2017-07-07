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
    string_to_param,
    parse_response
};
use types::{
    APIError,
    Bag,
    Character,
    CharacterCore,
    CharacterSkills,
    CharacterSpecializations,
    CharacterTraining,
    CraftingDiscipline,
    Equipment,
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
/// * `client` - The client to use when performing API requests
/// * `name` - Character name to fetch
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::characters::get_character;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_character = get_character(&client, "My Char");
/// ```
pub fn get_character(
    client: &APIClient,
    name: &str
) -> Result<Character, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("character", param))
        .expect("failed to get character");

    parse_response::<Character>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain backstory answers for a character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `name` - Character to obtain backstory for
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::characters::get_character_backstory;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_backstory = get_character_backstory(&client, "My Char");
/// ```
pub fn get_character_backstory(
    client: &APIClient,
    name: &str
) -> Result<Vec<String>, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("backstory", param))
        .expect("failed to get character backstory");

    parse_response::<Vec<String>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}


/// Obtain core information for the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `name` - Character to fetch
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::characters::get_character_core;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_character = get_character_core(&client, "My Char");
/// ```
pub fn get_character_core(
    client: &APIClient,
    name: &str
) -> Result<CharacterCore, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("core", param))
        .expect("failed to get character information");

    parse_response::<CharacterCore>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain crafting disciplines for the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `name` - Character to fetch
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::characters::get_character_crafting;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_crafting = get_character_crafting(&client, "My Char");
/// ```
pub fn get_character_crafting(
    client: &APIClient,
    name: &str
) -> Result<Vec<CraftingDiscipline>, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("crafting", param))
        .expect("failed to get crafting disciplines");

    parse_response::<Vec<CraftingDiscipline>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain equipment on the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `name` - Character to fetch
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::characters::get_character_equipment;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_equipment = get_character_equipment(&client, "My Char");
/// ```
pub fn get_character_equipment(
    client: &APIClient,
    name: &str
) -> Result<Vec<Equipment>, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("equip", param))
        .expect("failed to get character equipment");

    parse_response::<Vec<Equipment>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain hero points unlocked for the specified character
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
/// use tyria::api_v2::characters::get_character_heropoints;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_hp = get_character_heropoints(&client, "My Char");
/// ```
pub fn get_character_heropoints(
    client: &APIClient,
    name: &str
)
    -> Result<Vec<String>, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("hp", param))
        .expect("failed to get hero points");

    parse_response::<Vec<String>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain inventory of the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `name` - Character to fetch
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::characters::get_character_inventory;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_skills = get_character_inventory(&client, "My Char");
/// ```
pub fn get_character_inventory(
    client: &APIClient,
    name: &str
) -> Result<Vec<Bag>, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("inv", param))
        .expect("failed to get character inventory");

    parse_response::<Vec<Bag>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
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
/// use tyria::api_v2::characters::get_character_names;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_characters = get_character_names(&client);
/// ```
pub fn get_character_names(
    client: &APIClient
) -> Result<Vec<String>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("names"))
        .expect("failed to get character names");

    parse_response::<Vec<String>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked recipes for the specified character
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
/// use tyria::api_v2::characters::get_character_recipes;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_recipes = get_character_recipes(&client, "My Char");
/// ```
pub fn get_character_recipes(
    client: &APIClient,
    name: &str
) -> Result<Vec<i32>, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("recipes", param))
        .expect("failed to get unlocked recipes");

    parse_response::<Vec<i32>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain SAB progress for the specified character
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
/// use tyria::api_v2::characters::get_character_sab;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_character = get_character_sab(&client, "My Char");
/// ```
pub fn get_character_sab(
    client: &APIClient,
    name: &str
) -> Result<Vec<SABProgress>, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("sab", param))
        .expect("failed to get SAB progress");

    parse_response::<Vec<SABProgress>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain skills (PVE, PVP, WvW) of the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `name` - Character to fetch
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::characters::get_character_skills;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_skills = get_character_skills(&client, "My Char");
/// ```
pub fn get_character_skills(
    client: &APIClient,
    name: &str
) -> Result<CharacterSkills, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("skills", param))
        .expect("failed to get character skills");

    parse_response::<CharacterSkills>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain specializations of the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `name` - Character to fetch
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::characters::get_character_specializations;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_specs = get_character_specializations(&client, "My Char");
/// ```
pub fn get_character_specializations(
    client: &APIClient,
    name: &str
) -> Result<CharacterSpecializations, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("specs", param))
        .expect("failed to get character specializations");

    parse_response::<CharacterSpecializations>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

/// Obtain skill trees of the specified character
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `name` - Character to fetch
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::characters::get_character_training;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_training = get_character_training(&client, "My Char");
/// ```
pub fn get_character_training(
    client: &APIClient,
    name: &str
) -> Result<CharacterTraining, APIError> {

    let param = string_to_param("id", name);
    let mut response = client
        .make_request(&get_endpoint!("training", param))
        .expect("failed to get character training");

    parse_response::<CharacterTraining>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}
