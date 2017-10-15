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

/// Trading post endpoints

use client::APIClient;
use util::{
    number_to_param,
    numbers_to_param,
    parse_response
};
use types::{
    APIError,
    ExchangeRate,
    TPItem,
    TPItemInfo,
    TPTransaction
};

use reqwest::StatusCode;

/// Obtain the requested endpoint
macro_rules! get_endpoint {
    ("exchange") => {"/v2/commerce/exchange"};
    ("exchange_coins", $amount: expr) => {
        format!("/v2/commerce/exchange/coins?{}", $amount)
    };
    ("exchange_gems", $amount: expr) => {
        format!("/v2/commerce/exchange/gems?{}", $amount)
    };
    ("all_listings") => {"/v2/commerce/listings"};
    ("listings_id", $id: expr) => {format!("/v2/commerce/listings?{}", $id)};
    ("all_prices") => {"/v2/commerce/prices"};
    ("prices_id", $id: expr) => {format!("/v2/commerce/prices?{}", $id)};
    ("current_buy") => {"/v2/commerce/transactions/current/buys"};
    ("current_sell") => {"/v2/commerce/transactions/current/sells"};
    ("history_buy") => {"/v2/commerce/transactions/history/buys"};
    ("history_sell") => {"/v2/commerce/transactions/history/sells"};
}

/// Obtain a list of accepted resources for the gem exchange
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::commerce::get_exchange;
///
/// let client = APIClient::new("en", None);
///
/// let exchange_resources = get_exchange(&client);
/// ```
pub fn get_exchange(client: &APIClient) -> Result<Vec<String>, APIError> {
    let mut response = client.make_request(get_endpoint!("exchange"))
        .expect("failed to get gem exchange resources");

    parse_response::<Vec<String>>(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain current coins to gems exchange rate
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `amount` - The amount of coins to exchange for gems
pub fn get_coin_exchange(
    client: &APIClient,
    amount: i32
) -> Result<ExchangeRate, APIError> {
    let param = number_to_param("quantity", amount);
    let mut response = client
        .make_request(&get_endpoint!("exchange_coins", param))
        .expect("failed to get coin exchange rate");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound, StatusCode::BadRequest]
    )
}

/// Obtain current gem to coins exchange rate
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `amount` - The amount of gems to exchange for coins
pub fn get_gem_exchange(
    client: &APIClient,
    amount: i32
) -> Result<ExchangeRate, APIError> {
    let param = number_to_param("quantity", amount);
    let mut response = client
        .make_request(&get_endpoint!("exchange_gems", param))
        .expect("failed to get gem exchange rate");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound, StatusCode::BadRequest]
    )
}

/// Obtain a list of all trading post listings IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_listing_ids(client: &APIClient) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_listings"))
        .expect("failed to get listings IDs");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified item listing
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_listing(
    client: &APIClient,
    id: i32
) -> Result<TPItem, APIError> {
    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("listings_id", param))
        .expect("failed to get item listing");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified item listings
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_listings(
    client: &APIClient,
    ids: Vec<i32>
) -> Result<Vec<TPItem>, APIError> {
    let params = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("listings_id", params))
        .expect("failed to get item listings");

    parse_response(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}

/// Obtain a list of item IDs present in the trading post
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
pub fn get_pricing_ids(client: &APIClient) -> Result<Vec<i32>, APIError> {
    let mut response = client
        .make_request(get_endpoint!("all_prices"))
        .expect("failed to get item IDs");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified item in the trading post
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `id` - ID to fetch from the server
pub fn get_pricing(
    client: &APIClient,
    id: i32
) -> Result<TPItemInfo, APIError> {
    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("prices_id", param))
        .expect("failed to get item information");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified item listings
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `ids` - IDs to fetch from the server
pub fn get_pricings(
    client: &APIClient,
    ids: Vec<i32>
) -> Result<Vec<TPItemInfo>, APIError> {
    let params = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("prices_id", params))
        .expect("failed to get item information");

    parse_response(
        &mut response,
        vec![StatusCode::Ok, StatusCode::PartialContent],
        vec![StatusCode::NotFound]
    )
}

/// Obtain currently unfulfilled buy transactions for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_current_buy_transactions(
    client: &APIClient
) -> Result<Vec<TPTransaction>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("current_buy"))
        .expect("failed to get transactions");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain currently unfulfilled sell transactions for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_current_sell_transactions(
    client: &APIClient
) -> Result<Vec<TPTransaction>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("current_sell"))
        .expect("failed to get transactions");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain fulfilled buy transactions in the past 90 days for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_history_buy_transactions(
    client: &APIClient
) -> Result<Vec<TPTransaction>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("history_buy"))
        .expect("failed to get transactions");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

/// Obtain fulfilled sell transactions in the past 90 days for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests. Requires
///     authentication token
pub fn get_history_sell_transactions(
    client: &APIClient
) -> Result<Vec<TPTransaction>, APIError> {
    let mut response = client
        .make_authenticated_request(&get_endpoint!("history_sell"))
        .expect("failed to get transactions");

    parse_response(
        &mut response,
        vec![StatusCode::Ok],
        vec![StatusCode::Forbidden]
    )
}

#[cfg(test)]
mod tests {
    use std::env;
    use client::APIClient;
    use api_v2::commerce::*;

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
    fn exchange() {
        let client = setup_client();
        let result = get_exchange(&client);
        parse_test!(result);
    }

    #[test]
    fn coin_exchange() {
        let client = setup_client();
        let result = get_coin_exchange(&client, 9000);
        parse_test!(result);
    }

    #[test]
    fn gem_exchange() {
        let client = setup_client();
        let result = get_gem_exchange(&client, 100);
        parse_test!(result);
    }

    #[test]
    fn listing_ids() {
        let client = setup_client();
        let result = get_listing_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn listing() {
        let client = setup_client();
        let result = get_listing(&client, 19684);
        parse_test!(result);
    }

    #[test]
    fn listings() {
        let client = setup_client();
        let result = get_listings(&client, vec![19684, 19709]);
        parse_test!(result);
    }

    #[test]
    fn pricing_ids() {
        let client = setup_client();
        let result = get_pricing_ids(&client);
        parse_test!(result);
    }

    #[test]
    fn pricing() {
        let client = setup_client();
        let result = get_pricing(&client, 19684);
        parse_test!(result);
    }

    #[test]
    fn pricings() {
        let client = setup_client();
        let result = get_pricings(&client, vec![19684, 19709]);
        parse_test!(result);
    }

    #[test]
    fn current_buy_transactions() {
        let client = setup_client();
        let result = get_current_buy_transactions(&client);
        parse_test!(result);
    }

    #[test]
    fn current_sell_transactions() {
        let client = setup_client();
        let result = get_current_sell_transactions(&client);
        parse_test!(result);
    }

    #[test]
    fn history_buy_transactions() {
        let client = setup_client();
        let result = get_history_buy_transactions(&client);
        parse_test!(result);
    }

    #[test]
    fn history_sell_transactions() {
        let client = setup_client();
        let result = get_history_sell_transactions(&client);
        parse_test!(result);
    }
}
