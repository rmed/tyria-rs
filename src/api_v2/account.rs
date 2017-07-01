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

/// Account endpoints
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
    Account,
    AccountAchievement,
    AccountCurrency,
    AccountFinisher,
    AccountMastery,
    AccountMaterial,
    BankSlot,
    Cat,
    InventorySlot,
};

use reqwest::StatusCode;

/// Obtain the requested endpoint
macro_rules! get_endpoint {
    ("account") => {"/v2/account"};
    ("achievements") => {"/v2/account/achievements"};
    ("bank") => {"/v2/account/bank"};
    ("dungeons") => {"/v2/account/dungeons"};
    ("dyes") => {"/v2/account/dyes"};
    ("finishers") => {"/v2/account/finishers"};
    ("cats") => {"/v2/account/home/cats"};
    ("nodes") => {"/v2/account/home/nodes"};
    ("inventory") => {"/v2/account/inventory"};
    ("masteries") => {"/v2/account/masteries"};
    ("materials") => {"/v2/account/materials"};
    ("minis") => {"/v2/account/minis"};
    ("outfits") => {"/v2/account/outfits"};
    ("raids") => {"/v2/account/raids"};
    ("recipes") => {"/v2/account/recipes"};
    ("skins") => {"/v2/account/skins"};
    ("titles") => {"/v2/account/titles"};
    ("wallet") => {"/v2/account/wallet"};
    ("characters") => {"/v2/characters"};
    ("transactions") => {"/v2/commerce/transactions"};
    ("pvp-stats") => {"/v2/pvp/stats"};
    ("pvp-games") => {"/v2/pvp/games"};
    ("pvp-standings") => {"/v2/pvp/standings"};
    ("tokeninfo") => {"/v2/tokeninfo"};
}


/// Obtain details for the user account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_account = get_account(&client);
/// ```
pub fn get_account(client: &APIClient)
    -> Result<Account, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("account"))
        .expect("failed to get account");

    parse_response::<Account>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain achievements the account has progress on
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_achievements;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_achievements = get_account_achievements(&client);
/// ```
pub fn get_account_achievements(client: &APIClient)
    -> Result<Vec<AccountAchievement>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("achievements"))
        .expect("failed to get account achievements");

    parse_response::<Vec<AccountAchievement>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain bank item slots in the vault
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_bank;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_vault = get_account_bank(&client);
/// ```
pub fn get_account_bank(client: &APIClient)
    -> Result<Vec<BankSlot>, APIError> {

    //TODO check behaviour for empty slots
    let mut response = client
        .make_authenticated_request(&get_endpoint!("bank"))
        .expect("failed to get account bank");

    parse_response::<Vec<BankSlot>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain dungeon pathnames completed since daily dungeon reset
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_dungeons;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_dungeons = get_account_dungeons(&client);
/// ```
pub fn get_account_dungeons(client: &APIClient)
    -> Result<Vec<String>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("dungeons"))
        .expect("failed to get account dungeons");

    parse_response::<Vec<String>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked dyes for the account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_dyes;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_dyes = get_account_dyes(&client);
/// ```
pub fn get_account_dyes(client: &APIClient)
    -> Result<Vec<i32>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("dyes"))
        .expect("failed to get account dyes");

    parse_response::<Vec<i32>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked finishers for the account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_finishers;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_finishers = get_account_finishers(&client);
/// ```
pub fn get_account_finishers(client: &APIClient)
    -> Result<Vec<AccountFinisher>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("finishers"))
        .expect("failed to get account finishers");

    parse_response::<Vec<AccountFinisher>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked cats in the home instance of the account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_cats;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_cats = get_account_cats(&client);
/// ```
pub fn get_account_cats(client: &APIClient)
    -> Result<Vec<Cat>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("cats"))
        .expect("failed to get account cats");

    parse_response::<Vec<Cat>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked nodes in the home instance of the account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_nodes;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_nodes = get_account_nodes(&client);
/// ```
pub fn get_account_nodes(client: &APIClient)
    -> Result<Vec<String>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("nodes"))
        .expect("failed to get account nodes");

    parse_response::<Vec<String>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain shared inventory slots in an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_inventory;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_inventory = get_account_inventory(&client);
/// ```
pub fn get_account_inventory(client: &APIClient)
    -> Result<Vec<InventorySlot>, APIError> {

    //TODO check behaviour with empty slots
    let mut response = client
        .make_authenticated_request(&get_endpoint!("inventory"))
        .expect("failed to get shared account inventory slots");

    parse_response::<Vec<InventorySlot>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked masteries for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_masteries;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_masteries = get_account_masteries(&client);
/// ```
pub fn get_account_masteries(client: &APIClient)
    -> Result<Vec<AccountMastery>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("masteries"))
        .expect("failed to get account masteries");

    parse_response::<Vec<AccountMastery>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain materials stored in an account's vault
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_materials;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_materials = get_account_materials(&client);
/// ```
pub fn get_account_materials(client: &APIClient)
    -> Result<Vec<AccountMaterial>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("materials"))
        .expect("failed to get account materials");

    parse_response::<Vec<AccountMaterial>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked minis for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_minis;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_minis = get_account_minis(&client);
/// ```
pub fn get_account_minis(client: &APIClient)
    -> Result<Vec<i32>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("minis"))
        .expect("failed to get account minis");

    parse_response::<Vec<i32>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain outfits unlocked for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_outfits;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_outfits = get_account_outfits(&client);
/// ```
pub fn get_account_outfits(client: &APIClient)
    -> Result<Vec<i32>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("outfits"))
        .expect("failed to get account outfits");

    parse_response::<Vec<i32>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain account raid encounters completed since weekly raid reset
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_raids;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_raids = get_account_raids(&client);
/// ```
pub fn get_account_raids(client: &APIClient)
    -> Result<Vec<String>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("raids"))
        .expect("failed to get account raids");

    parse_response::<Vec<String>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain recipes unlocked for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_recipes;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_recipes = get_account_recipes(&client);
/// ```
pub fn get_account_recipes(client: &APIClient)
    -> Result<Vec<i32>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("recipes"))
        .expect("failed to get account recipes");

    parse_response::<Vec<i32>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain skins unlocked for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_skins;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_skins = get_account_skins(&client);
/// ```
pub fn get_account_skins(client: &APIClient)
    -> Result<Vec<i32>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("skins"))
        .expect("failed to get account skins");

    parse_response::<Vec<i32>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain titles unlocked for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_titles;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_outfits = get_account_titles(&client);
/// ```
pub fn get_account_titles(client: &APIClient)
    -> Result<Vec<i32>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("titles"))
        .expect("failed to get account titles");

    parse_response::<Vec<i32>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain currencies in the wallet of an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::account::get_account_wallet;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_currencies = get_account_wallet(&client);
/// ```
pub fn get_account_wallet(client: &APIClient)
    -> Result<Vec<AccountCurrency>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("wallet"))
        .expect("failed to get account wallet");

    parse_response::<Vec<AccountCurrency>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}
