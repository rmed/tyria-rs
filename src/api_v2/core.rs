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

/// Core game mechanics endpoints

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
    Legend,
    Mastery,
    Outfit,
    Pet,
    Profession,
    Race,
    Skill,
    Specialization,
    Trait
};

use reqwest::StatusCode;

/// Obtain the requested endpoint
macro_rules! get_endpoint {
    ("all_masteries") => {"/v2/masteries"};
    ("masteries_id", $id: expr) => {format!("/v2/masteries?{}", $id)};
    ("all_outfits") => {"/v2/outfits"};
    ("outfits_id", $id: expr) => {format!("/v2/outfits?{}", $id)};
    ("all_pets") => {"/v2/pets"};
    ("pets_id", $id: expr) => {format!("/v2/pets?{}", $id)};
    ("all_professions") => {"/v2/professions"};
    ("professions_id", $id: expr) => {format!("/v2/professions?{}", $id)};
    ("all_races") => {"/v2/races"};
    ("races_id", $id: expr) => {format!("/v2/races?{}", $id)};
    ("all_specs") => {"/v2/specializations"};
    ("specs_id", $id: expr) => {format!("/v2/specializations?{}", $id)};
    ("all_skills") => {"/v2/skills"};
    ("skills_id", $id: expr) => {format!("/v2/skills?{}", $id)};
    ("all_traits") => {"/v2/traits"};
    ("traits_id", $id: expr) => {format!("/v2/traits?{}", $id)};
    ("all_legends") => {"/v2/legends"};
    ("legends_id", $id: expr) => {format!("/v2/legends?{}", $id)};
}

/// Obtain a list of all available mastery IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_mastery_ids(client: &APIClient) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_masteries"))
        .expect("failed to get mastery IDs");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified mastery
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_mastery(client: &APIClient, id: i32) -> Result<Mastery, APIError> {
    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("masteries_id", param))
        .expect("failed to get mastery");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified masteries
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_masteries(
    client: &APIClient,
    ids: Vec<i32>
) -> Result<Vec<Mastery>, APIError> {
    let param = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("masteries_id", param))
        .expect("failed to get masteries");

    parse_response(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}

/// Obtain a list of all available outfit IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_outfit_ids(client: &APIClient) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_outfits"))
        .expect("failed to get outfit IDs");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified outfit
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_outfit(client: &APIClient, id: i32) -> Result<Outfit, APIError> {
    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("outfits_id", param))
        .expect("failed to get outfit");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified outfits
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_outfits(
    client: &APIClient,
    ids: Vec<i32>
) -> Result<Vec<Outfit>, APIError> {
    let param = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("outfits_id", param))
        .expect("failed to get outfits");

    parse_response(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}

/// Obtain a list of all available pet IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_pet_ids(client: &APIClient) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_pets"))
        .expect("failed to get pet IDs");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified pet
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_pet(client: &APIClient, id: i32) -> Result<Pet, APIError> {
    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("pets_id", param))
        .expect("failed to get pet");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified pets
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_pets(
    client: &APIClient,
    ids: Vec<i32>
) -> Result<Vec<Pet>, APIError> {
    let param = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("pets_id", param))
        .expect("failed to get pets");

    parse_response(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}

/// Obtain a list of all available profession IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_profession_ids(client: &APIClient) -> Result<Vec<String>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_professions"))
        .expect("failed to get profession IDs");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified profession
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_profession(
    client: &APIClient,
    id: &str
) -> Result<Profession, APIError> {
    let param = string_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("professions_id", param))
        .expect("failed to get profession");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified professions
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_professions(
    client: &APIClient,
    ids: Vec<&str>
) -> Result<Vec<Profession>, APIError> {
    let param = strings_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("professions_id", param))
        .expect("failed to get professions");

    parse_response::<Vec<Profession>>(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}

/// Obtain a list of all available race IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_race_ids(client: &APIClient) -> Result<Vec<String>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_races"))
        .expect("failed to get race IDs");

    parse_response::<Vec<String>>(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified race
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_race(client: &APIClient, id: &str) -> Result<Race, APIError> {
    let param = string_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("races_id", param))
        .expect("failed to get race");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified races
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_races(
    client: &APIClient,
    ids: Vec<&str>
) -> Result<Vec<Race>, APIError> {
    let param = strings_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("races_id", param))
        .expect("failed to get races");

    parse_response(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}

/// Obtain a list of all available specialization IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_specialization_ids(
    client: &APIClient
) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_specs"))
        .expect("failed to get specialization IDs");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified specialization
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_specialization(
    client: &APIClient,
    id: i32
) -> Result<Specialization, APIError> {
    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("specs_id", param))
        .expect("failed to get specialization");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified specializations
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_specializations(
    client: &APIClient,
    ids: Vec<i32>
) -> Result<Vec<Specialization>, APIError> {
    let param = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("specs_id", param))
        .expect("failed to get specializations");

    parse_response(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}

/// Obtain a list of all available skill IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_skill_ids(client: &APIClient) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_skills"))
        .expect("failed to get skill IDs");

    parse_response::<Vec<i32>>(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified skill
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_skill(client: &APIClient, id: i32) -> Result<Skill, APIError> {
    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("skills_id", param))
        .expect("failed to get skill");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified skills
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_skills(
    client: &APIClient,
    ids: Vec<i32>
) -> Result<Vec<Skill>, APIError> {
    let param = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("skills_id", param))
        .expect("failed to get skill");

    parse_response(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}

/// Obtain a list of all available  IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_trait_ids(client: &APIClient) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_traits"))
        .expect("failed to get trait IDs");

    parse_response::<Vec<i32>>(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified trait
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_trait(client: &APIClient, id: i32) -> Result<Trait, APIError> {
    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("traits_id", param))
        .expect("failed to get trait");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified traits
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_traits(
    client: &APIClient,
    ids: Vec<i32>
) -> Result<Vec<Trait>, APIError> {
    let param = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("traits_id", param))
        .expect("failed to gettraits");

    parse_response(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}

/// Obtain a list of all available Revenant legend IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_legend_ids(client: &APIClient) -> Result<Vec<String>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_legends"))
        .expect("failed to get legend IDs");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified Revenant legend
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_legend(client: &APIClient, id: &str) -> Result<Legend, APIError> {
    let param = string_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("legends_id", param))
        .expect("failed to get legend");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified Revenant legend
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_legends(
    client: &APIClient,
    ids: Vec<&str>
) -> Result<Vec<Legend>, APIError> {
    let param = strings_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("legends_id", param))
        .expect("failed to get legends");

    parse_response(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}


#[cfg(test)]
mod tests {
    use client::APIClient;
    use api_v2::core::*;

    macro_rules! parse_test {
        ($result:expr) => {
            match $result {
                Ok(_) => assert!(true),
                Err(e) => panic!(e.description().to_string()),
            };
        }
    }

    #[test]
    fn mastery_ids() {
        let client = APIClient::new("en", None);
        let result = get_mastery_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn mastery() {
        let client = APIClient::new("en", None);
        let result = get_mastery(&client, 1);
        parse_test!(result);
    }

    #[test]
    fn masteries() {
        let client = APIClient::new("en", None);
        let result = get_masteries(&client, vec![1, 2]);
        parse_test!(result);
    }

    #[test]
    fn outfit_ids() {
        let client = APIClient::new("en", None);
        let result = get_outfit_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn outfit() {
        let client = APIClient::new("en", None);
        let result = get_outfit(&client, 30);
        parse_test!(result);
    }

    #[test]
    fn outfits() {
        let client = APIClient::new("en", None);
        let result = get_outfits(&client, vec![30, 45]);
        parse_test!(result);
    }

    #[test]
    fn pet_ids() {
        let client = APIClient::new("en", None);
        let result = get_pet_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn pet() {
        let client = APIClient::new("en", None);
        let result = get_pet(&client, 33);
        parse_test!(result);
    }

    #[test]
    fn pets() {
        let client = APIClient::new("en", None);
        let result = get_pets(&client, vec![33, 42]);
        parse_test!(result);
    }

    #[test]
    fn profession_ids() {
        let client = APIClient::new("en", None);
        let result = get_profession_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn profession() {
        let client = APIClient::new("en", None);
        let result = get_profession(&client, "Thief");
        parse_test!(result);
    }

    #[test]
    fn professions() {
        let client = APIClient::new("en", None);
        let result = get_professions(&client, vec!["Thief", "Necromancer"]);
        parse_test!(result);
    }

    #[test]
    fn race_ids() {
        let client = APIClient::new("en", None);
        let result = get_race_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn race() {
        let client = APIClient::new("en", None);
        let result = get_race(&client, "Human");
        parse_test!(result);
    }

    #[test]
    fn races() {
        let client = APIClient::new("en", None);
        let result = get_races(&client, vec!["Human", "Sylvari", "Norn"]);
        parse_test!(result);
    }

    #[test]
    fn specialization_ids() {
        let client = APIClient::new("en", None);
        let result = get_specialization_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn specialization() {
        let client = APIClient::new("en", None);
        let result = get_specialization(&client, 1);
        parse_test!(result);
    }

    #[test]
    fn specializations() {
        let client = APIClient::new("en", None);
        let result = get_specializations(&client, vec![2, 3]);
        parse_test!(result);
    }

    #[test]
    fn skill_ids() {
        let client = APIClient::new("en", None);
        let result = get_skill_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn skill() {
        let client = APIClient::new("en", None);
        let result = get_skill(&client, 14375);
        parse_test!(result);
    }

    #[test]
    fn skills() {
        let client = APIClient::new("en", None);
        let result = get_skills(&client, vec![5516, 5517]);
        parse_test!(result);
    }

    #[test]
    fn trait_ids() {
        let client = APIClient::new("en", None);
        let result = get_trait_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn single_trait() {
        let client = APIClient::new("en", None);
        let result = get_trait(&client, 214);
        parse_test!(result);
    }

    #[test]
    fn traits() {
        let client = APIClient::new("en", None);
        let result = get_traits(&client, vec![277, 334]);
        parse_test!(result);
    }

    #[test]
    fn legend_ids() {
        let client = APIClient::new("en", None);
        let result = get_legend_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn legend() {
        let client = APIClient::new("en", None);
        let result = get_legend(&client, "Legend1");
        parse_test!(result);
    }

    #[test]
    fn legends() {
        let client = APIClient::new("en", None);
        let result = get_legends(&client, vec!["Legend2", "Legend5"]);
        parse_test!(result);
    }
}
