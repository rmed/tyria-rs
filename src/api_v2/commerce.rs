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
    ("current_buy") => {"/v2/commerce/transactions/current/buy"};
    ("current_sell") => {"/v2/commerce/transactions/current/sell"};
    ("history_buy") => {"/v2/commerce/transactions/history/buy"};
    ("history_sell") => {"/v2/commerce/transactions/history/sell"};
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
        StatusCode::Ok,
        vec![StatusCode::NotFound]
    )
}

/// Obtain current coins to gems exchange rate
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `amount` - The amount of coins to exchange for gems
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::commerce::get_coin_exchange;
///
/// let client = APIClient::new("en", None);
///
/// let exchange = get_coin_exchange(&client, 42);
/// ```
pub fn get_coin_exchange(
    client: &APIClient,
    amount: i32
) -> Result<ExchangeRate, APIError> {

    let param = number_to_param("quantity", amount);
    let mut response = client
        .make_request(&get_endpoint!("exchange_coins", param))
        .expect("failed to get coin exchange rate");

    parse_response::<ExchangeRate>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::BadRequest]
    )
}

/// Obtain current gem to coins exchange rate
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
/// * `amount` - The amount of gems to exchange for coins
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::commerce::get_gem_exchange;
///
/// let client = APIClient::new("en", None);
///
/// let rate = get_gem_exchange(&client, 42);
/// ```
pub fn get_gem_exchange(
    client: &APIClient,
    amount: i32
) -> Result<ExchangeRate, APIError> {

    let param = number_to_param("quantity", amount);
    let mut response = client
        .make_request(&get_endpoint!("exchange_gems", param))
        .expect("failed to get gem exchange rate");

    parse_response::<ExchangeRate>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound, StatusCode::BadRequest]
    )
}

/// Obtain a list of all trading post listings IDs
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::commerce::get_listing_ids;
///
/// let client = APIClient::new("en", None);
///
/// let listing_ids = get_listing_ids(&client);
/// ```
pub fn get_listing_ids(client: &APIClient) -> Result<Vec<i32>, APIError> {
    let mut response = client.make_request(get_endpoint!("all_listings"))
        .expect("failed to get listings IDs");

    parse_response::<Vec<i32>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified item listing
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
/// use tyria::api_v2::commerce::get_listing;
///
/// let client = APIClient::new("en", None);
///
/// let listing = get_listing(&client, 19684);
/// ```
pub fn get_listing(
    client: &APIClient,
    id: i32
) -> Result<TPItem, APIError> {

    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("listings_id", param))
        .expect("failed to get item listing");

    parse_response::<TPItem>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified item listings
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
/// use tyria::api_v2::commerce::get_listings;
///
/// let client = APIClient::new("en", None);
///
/// let listings = get_listings(&client, vec![19684, 19685]);
/// ```
pub fn get_listings(
    client: &APIClient,
    ids: Vec<i32>
) -> Result<Vec<TPItem>, APIError> {

    let params = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("listings_id", params))
        .expect("failed to get item listings");

    parse_response::<Vec<TPItem>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound]
    )
}

/// Obtain a list of item IDs present in the trading post
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::commerce::get_pricing_ids;
///
/// let client = APIClient::new("en", None);
///
/// let pricing_ids = get_pricing_ids(&client);
/// ```
pub fn get_pricing_ids(client: &APIClient) -> Result<Vec<i32>, APIError> {
    let mut response = client.make_request(get_endpoint!("all_prices"))
        .expect("failed to get item IDs");

    parse_response::<Vec<i32>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified item in the trading post
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
/// use tyria::api_v2::commerce::get_pricing;
///
/// let client = APIClient::new("en", None);
///
/// let pricing = get_pricing(&client, 19684);
/// ```
pub fn get_pricing(
    client: &APIClient,
    id: i32
) -> Result<TPItemInfo, APIError> {

    let param = number_to_param("id", id);
    let mut response = client
        .make_request(&get_endpoint!("prices_id", param))
        .expect("failed to get item information");

    parse_response::<TPItemInfo>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound]
    )
}

/// Obtain details for the specified item listings
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
/// use tyria::api_v2::commerce::get_pricings;
///
/// let client = APIClient::new("en", None);
///
/// let pricings = get_pricings(&client, vec![19684, 19685]);
/// ```
pub fn get_pricings(
    client: &APIClient,
    ids: Vec<i32>
) -> Result<Vec<TPItemInfo>, APIError> {

    let params = numbers_to_param("ids", &ids);
    let mut response = client
        .make_request(&get_endpoint!("prices_id", params))
        .expect("failed to get item information");

    parse_response::<Vec<TPItemInfo>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::NotFound]
    )
}

/// Obtain currently unfulfilled buy transactions for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::commerce::get_current_buy_transactions;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_transactions = get_current_buy_transactions(&client);
/// ```
pub fn get_current_buy_transactions(
    client: &APIClient
) -> Result<Vec<TPTransaction>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("current_buy"))
        .expect("failed to get transactions");

    parse_response::<Vec<TPTransaction>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain currently unfulfilled sell transactions for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::commerce::get_current_sell_transactions;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_transactions = get_current_sell_transactions(&client);
/// ```
pub fn get_current_sell_transactions(
    client: &APIClient
) -> Result<Vec<TPTransaction>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("current_sell"))
        .expect("failed to get transactions");

    parse_response::<Vec<TPTransaction>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain fulfilled buy transactions in the past 90 days for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::commerce::get_history_buy_transactions;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_transactions = get_history_buy_transactions(&client);
/// ```
pub fn get_history_buy_transactions(
    client: &APIClient
) -> Result<Vec<TPTransaction>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("history_buy"))
        .expect("failed to get transactions");

    parse_response::<Vec<TPTransaction>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}

/// Obtain fulfilled sell transactions in the past 90 days for an account
///
/// # Arguments
///
/// * `client` - The client to use when performing API requests
///
/// # Example
///
/// ```
/// use tyria::client::APIClient;
/// use tyria::api_v2::commerce::get_history_sell_transactions;
///
/// let client = APIClient::new("en", Some("mykey".to_string()));
///
/// let my_transactions = get_history_sell_transactions(&client);
/// ```
pub fn get_history_sell_transactions(
    client: &APIClient
) -> Result<Vec<TPTransaction>, APIError> {

    let mut response = client
        .make_authenticated_request(&get_endpoint!("history_sell"))
        .expect("failed to get transactions");

    parse_response::<Vec<TPTransaction>>(
        &mut response,
        StatusCode::Ok,
        vec![StatusCode::Forbidden]
    )
}
