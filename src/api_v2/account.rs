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
    parse_response
};
use types::{
    APIError,
    APIKey,
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
    ("tokeninfo") => {"/v2/tokeninfo"};
}


/// Obtain details for the user account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account(
    client: &APIClient
) -> Result<Account, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("account"))
        .expect("failed to get account");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain achievements the account has progress on
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_achievements(
    client: &APIClient
) -> Result<Vec<AccountAchievement>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("achievements"))
        .expect("failed to get account achievements");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain bank item slots in the vault
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_bank(
    client: &APIClient
) -> Result<Vec<Option<BankSlot>>, APIError> {
    //TODO check behaviour for empty slots
    let mut response = client
        .make_authenticated_request(&get_endpoint!("bank"))
        .expect("failed to get account bank");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain dungeon pathnames completed since daily dungeon reset
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_dungeons(
    client: &APIClient
) -> Result<Vec<String>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("dungeons"))
        .expect("failed to get account dungeons");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked dyes for the account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_dyes(
    client: &APIClient
) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("dyes"))
        .expect("failed to get account dyes");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked finishers for the account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_finishers(
    client: &APIClient
) -> Result<Vec<AccountFinisher>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("finishers"))
        .expect("failed to get account finishers");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked cats in the home instance of the account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_cats(
    client: &APIClient
) -> Result<Vec<Cat>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("cats"))
        .expect("failed to get account cats");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked nodes in the home instance of the account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_nodes(
    client: &APIClient
) -> Result<Vec<String>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("nodes"))
        .expect("failed to get account nodes");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain shared inventory slots in an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_inventory(
    client: &APIClient
) -> Result<Vec<Option<InventorySlot>>, APIError> {
    //TODO check behaviour with empty slots
    let mut response = client
        .make_authenticated_request(&get_endpoint!("inventory"))
        .expect("failed to get shared account inventory slots");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked masteries for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_masteries(
    client: &APIClient
) -> Result<Vec<AccountMastery>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("masteries"))
        .expect("failed to get account masteries");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain materials stored in an account's vault
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_materials(
    client: &APIClient
) -> Result<Vec<AccountMaterial>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("materials"))
        .expect("failed to get account materials");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain unlocked minis for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_minis(
    client: &APIClient
) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("minis"))
        .expect("failed to get account minis");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain outfits unlocked for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_outfits(
    client: &APIClient
) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("outfits"))
        .expect("failed to get account outfits");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain account raid encounters completed since weekly raid reset
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_raids(
    client: &APIClient
) -> Result<Vec<String>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("raids"))
        .expect("failed to get account raids");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain recipes unlocked for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_recipes(
    client: &APIClient
) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("recipes"))
        .expect("failed to get account recipes");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain skins unlocked for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_skins(
    client: &APIClient
) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("skins"))
        .expect("failed to get account skins");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain titles unlocked for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_titles(
    client: &APIClient
) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("titles"))
        .expect("failed to get account titles");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain currencies in the wallet of an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_account_wallet(
    client: &APIClient
) -> Result<Vec<AccountCurrency>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("wallet"))
        .expect("failed to get account wallet");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain information on the given token
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_token_info(
    client: &APIClient
) -> Result<APIKey, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("tokeninfo"))
        .expect("failed to get API key details");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound, StatusCode::Forbidden]
    )
}

#[cfg(test)]
mod tests {
    use std::env;
    use client::APIClient;
    use api_v2::account::*;

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

    #[test]
    fn account() {
        let client = setup_client();
        let result = get_account(&client);
        parse_test!(result);
    }

    #[test]
    fn account_achievements() {
        let client = setup_client();
        let result = get_account_achievements(&client);
        parse_test!(result);
    }

    #[test]
    fn account_bank() {
        let client = setup_client();
        let result = get_account_bank(&client);
        parse_test!(result);
    }

    #[test]
    fn account_dungeons() {
        let client = setup_client();
        let result = get_account_dungeons(&client);
        parse_test!(result);
    }

    #[test]
    fn account_dyes() {
        let client = setup_client();
        let result = get_account_dyes(&client);
        parse_test!(result);
    }

    #[test]
    fn account_finishers() {
        let client = setup_client();
        let result = get_account_finishers(&client);
        parse_test!(result);
    }

    #[test]
    fn account_cats() {
        let client = setup_client();
        let result = get_account_cats(&client);
        parse_test!(result);
    }

    #[test]
    fn account_nodes() {
        let client = setup_client();
        let result = get_account_nodes(&client);
        parse_test!(result);
    }

    #[test]
    fn account_inventory() {
        let client = setup_client();
        let result = get_account_inventory(&client);
        parse_test!(result);
    }

    #[test]
    fn account_masteries() {
        let client = setup_client();
        let result = get_account_masteries(&client);
        parse_test!(result);
    }

    #[test]
    fn account_materials() {
        let client = setup_client();
        let result = get_account_materials(&client);
        parse_test!(result);
    }

    #[test]
    fn account_minis() {
        let client = setup_client();
        let result = get_account_minis(&client);
        parse_test!(result);
    }

    #[test]
    fn account_outfits() {
        let client = setup_client();
        let result = get_account_outfits(&client);
        parse_test!(result);
    }

    #[test]
    fn account_raids() {
        let client = setup_client();
        let result = get_account_raids(&client);
        parse_test!(result);
    }

    #[test]
    fn account_recipes() {
        let client = setup_client();
        let result = get_account_recipes(&client);
        parse_test!(result);
    }

    #[test]
    fn account_skins() {
        let client = setup_client();
        let result = get_account_skins(&client);
        parse_test!(result);
    }

    #[test]
    fn account_titles() {
        let client = setup_client();
        let result = get_account_titles(&client);
        parse_test!(result);
    }

    #[test]
    fn account_wallet() {
        let client = setup_client();
        let result = get_account_wallet(&client);
        parse_test!(result);
    }
}
