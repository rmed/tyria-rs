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

use hyper::header::LanguageTag;
use reqwest;
use reqwest::header::{Headers, AcceptLanguage, Authorization, qitem};

macro_rules! get_request_url {
    ($endpoint: expr) => {format!("https://api.guildwars2.com{}", $endpoint)}
}

/// Client in charge of performing requests to the API
pub struct APIClient {
    /// Locale to use for requests
    lang: String,
    /// API token to use in certain endpoints that require authentication
    token: Option<String>,
    /// HTTP client
    client: reqwest::Client
}

impl APIClient {
    /// Create a new API client
    ///
    /// # Arguments
    ///
    /// * `lang` - Language to use in the API calls
    /// * `token` - Optional token to use in authenticated endpoints
    pub fn new(lang: &str, token: Option<String>) -> APIClient {
        APIClient {
            lang: lang.to_string(),
            token: token,
            client: reqwest::Client::new().unwrap()
        }
    }

    /// Make an authenticated request to the API
    ///
    /// This expects the token to have been previously configured when
    /// initialising the client
    ///
    /// # Arguments
    ///
    /// * `url` - URL to make the request to
    pub fn make_authenticated_request(&self, url: &str)
        -> reqwest::Result<reqwest::Response> {

        let full_url = get_request_url!(url);
        let mut headers = Headers::new();

        // Set authentication
        let token = self.token.to_owned();
        headers.set(
            Authorization(
                format!("Bearer {}", token.expect("token is not configured"))
            )
        );

        // Set language
        let mut langtag: LanguageTag = Default::default();
        langtag.language = Some(self.lang.to_owned());
        headers.set(
            AcceptLanguage(vec![
                qitem(langtag),
            ])
        );

        self.client.get(&full_url).headers(headers).send()
    }

    /// Make a request to the API
    ///
    /// # Arguments
    ///
    /// * `url` - URL to make the request to
    pub fn make_request(&self, url: &str)
        -> reqwest::Result<reqwest::Response> {

        let full_url = get_request_url!(url);

        // Set language
        let mut headers = Headers::new();
        let mut langtag: LanguageTag = Default::default();
        langtag.language = Some(self.lang.to_owned());
        headers.set(
            AcceptLanguage(vec![
                qitem(langtag),
            ])
        );

        self.client.get(&full_url).headers(headers).send()
    }
}
