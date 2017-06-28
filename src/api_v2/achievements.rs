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

/// Achievement endpoints

use client::APIClient;
use util::{
    number_to_param,
    numbers_to_param,
    string_to_param,
    strings_to_param
};
use types::{
    APIError,
    Achievement,
    AchievementCategory,
    AchievementGroup,
    DailyAchievements
};

use reqwest::StatusCode;

/// Obtain the requested endpoint
macro_rules! get_endpoint {
    ("all_achievements") => {"/v2/achievements"};
    ("achievements_id", $id: expr) => {format!("/v2/achievements?{}", $id)};
    ("daily_achievements") => {"/v2/achievements/daily"};
    ("daily_achievements_tomorrow") => {"/v2/achievements/daily/tomorrow"};
    ("all_achievement_groups") => {"/v2/achievements/groups"};
    ("achievement_groups_id", $id: expr) => {
        format!("/v2/achievements/groups?{}", $id)
    };
    ("all_achievement_categories") => {"/v2/achievements/categories"};
    ("achievement_categories_id", $id: expr) => {
        format!("/v2/achievements/categories?{}", $id)
    };
}

/// Obtain a list of all the achievement IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_achievement_ids;
///
/// let client = APIClient::new("en", None);
///
/// let achievement_ids = get_achievement_ids(&client);
/// ```
pub fn get_achievement_ids(client: &APIClient) -> Result<Vec<i32>, APIError> {
    let mut response = client.make_request(get_endpoint!("all_achievements"))
        .expect("Failed to get achievement IDs");

    // Check HTTP codes
    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<Vec<i32>>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}

/// Obtain details for the specified achievement
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_achievement;
///
/// let client = APIClient::new("en", None);
///
/// let achievement_ids = get_achievement(&client, 42);
/// ```
pub fn get_achievement(client: &APIClient, id: i32)
    -> Result<Achievement, APIError> {

    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("achievements_id", param))
        .expect("Failed to get achievement");

    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<Achievement>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}

/// Obtain details for the specified achievements
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_achievements;
///
/// let client = APIClient::new("en", None);
///
/// let achievement_ids = get_achievements(&client, vec![1, 2, 42]);
/// ```
pub fn get_achievements(client: &APIClient, ids: Vec<i32>)
    -> Result<Vec<Achievement>, APIError> {

    let params = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("achievements_id", params))
        .expect("Failed to get achievements");

    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<Vec<Achievement>>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}

/// Obtain daily current achievements
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_daily_achievements;
///
/// let client = APIClient::new("en", None);
///
/// let achievements = get_daily_achievements(&client);
/// ```
pub fn get_daily_achievements(client: &APIClient)
    -> Result<DailyAchievements, APIError> {

    let mut response = client
        .make_request(&get_endpoint!("daily_achievements"))
        .expect("Failed to get achievements");

    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<DailyAchievements>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}

/// Obtain daily achievements for tomorrow
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_daily_achievements_tomorrow;
///
/// let client = APIClient::new("en", None);
///
/// let achievements = get_daily_achievements_tomorrow(&client);
/// ```
pub fn get_daily_achievements_tomorrow(client: &APIClient)
    -> Result<DailyAchievements, APIError> {

    let mut response = client
        .make_request(&get_endpoint!("daily_achievements_tomorrow"))
        .expect("Failed to get achievements");

    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<DailyAchievements>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}

/// Obtain a list of all the achievement group IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_achievement_group_ids;
///
/// let client = APIClient::new("en", None);
///
/// let group_ids = get_achievement_group_ids(&client);
/// ```
pub fn get_achievement_group_ids(client: &APIClient)
    -> Result<Vec<String>, APIError> {

    let mut response = client
        .make_request(get_endpoint!("all_achievement_groups"))
        .expect("Failed to get group IDs");

    // Check HTTP codes
    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<Vec<String>>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}

/// Obtain details for the specified achievement group
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_achievement_group;
///
/// let client = APIClient::new("en", None);
///
/// let group = get_achievement_group(
///     &client,
///     "65B4B678-607E-4D97-B458-076C3E96A810"
/// );
/// ```
pub fn get_achievement_group(client: &APIClient, id: &str)
    -> Result<AchievementGroup, APIError> {

    let param = string_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("achievement_groups_id", param))
        .expect("Failed to get group");

    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<AchievementGroup>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}

/// Obtain details for the specified achievement groups
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_achievement_groups;
///
/// let client = APIClient::new("en", None);
///
/// let group = get_achievement_groups(
///     &client,
///     vec![
///         "65B4B678-607E-4D97-B458-076C3E96A810",
///         "1CAFA333-0C2B-4782-BC4C-7DA30E9CE289"
///     ]
/// );
/// ```
pub fn get_achievement_groups(client: &APIClient, ids: Vec<&str>)
    -> Result<Vec<AchievementGroup>, APIError> {

    let param = strings_to_param("ids", ids);
    let mut response = client
        .make_request(&get_endpoint!("achievement_groups_id", param))
        .expect("Failed to get groups");

    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<Vec<AchievementGroup>>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}

/// Obtain a list of all the achievement category IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_achievement_category_ids;
///
/// let client = APIClient::new("en", None);
///
/// let group_ids = get_achievement_category_ids(&client);
/// ```
pub fn get_achievement_category_ids(client: &APIClient)
    -> Result<Vec<i32>, APIError> {

    let mut response = client
        .make_request(get_endpoint!("all_achievement_categories"))
        .expect("Failed to get category IDs");

    // Check HTTP codes
    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<Vec<i32>>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}

/// Obtain details for the specified achievement category
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_achievement_category;
///
/// let client = APIClient::new("en", None);
///
/// let group = get_achievement_category(&client, 42);
/// ```
pub fn get_achievement_category(client: &APIClient, id: i32)
    -> Result<AchievementCategory, APIError> {

    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("achievement_categories_id", param))
        .expect("Failed to get category");

    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<AchievementCategory>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}

/// Obtain details for the specified achievement categories
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::achievements::get_achievement_categories;
///
/// let client = APIClient::new("en", None);
///
/// let group = get_achievement_categories(&client, vec![1, 2, 3, 42]);
/// ```
pub fn get_achievement_categories(client: &APIClient, ids: Vec<i32>)
    -> Result<Vec<AchievementCategory>, APIError> {

    let param = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("achievement_categories_id", param))
        .expect("Failed to get categories");

    if response.status().eq(&StatusCode::Ok) {
        return Ok(response.json::<Vec<AchievementCategory>>().unwrap());

    } else if response.status().eq(&StatusCode::NotFound) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new("unknown status code"))
}
