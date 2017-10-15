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

/// Utility code

use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use types::APIError;

/// Make a parameter out of a number
///
/// # Arguments
///
/// * `param` - Name of the parameter
/// * `value` - Numerical value to convert
///
/// # Example
///
/// ```
/// use tyria::util::number_to_param;
///
/// let result = number_to_param("id", 42);
/// ```
pub fn number_to_param(param: &str, value: i32) -> String {
    let mut result = String::new();

    // Add parameter label
    result.push_str(param);
    result.push_str("=");
    result.push_str(&value.to_string());

    result
}

/// Make a parameter out of a vector of numbers
///
/// # Arguments
///
/// * `param` - Name of the parameter
/// * `values` - Vector of numbers
///
/// # Example
///
/// ```
/// use tyria::util::numbers_to_param;
///
/// let ids = vec![1, 2, 3, 4, 5, 42];
/// let result = numbers_to_param("ids", &ids);
/// ```
pub fn numbers_to_param(param: &str, values: &Vec<i32>) -> String {
    let mut result = String::new();

    // Add parameter label
    result.push_str(param);
    result.push_str("=");

    // Separate with commas
    for val in values {
        result.push_str(&val.to_string());
        result.push_str(",");
    }

    result
}

/// Make a parameter out of a string
///
/// # Arguments
///
/// * `param` - Name of the parameter
/// * `value` - Numerical value to convert
///
/// # Example
///
/// ```
/// use tyria::util::string_to_param;
///
/// let result = string_to_param("id", "my-id");
/// ```
pub fn string_to_param(param: &str, value: &str) -> String {
    let mut result = String::new();

    // Add parameter label
    result.push_str(param);
    result.push_str("=");
    result.push_str(value);

    result
}

/// Make a parameter out of a vectors of string
///
/// # Arguments
///
/// * `param` - Name of the parameter
/// * `value` - Numerical value to convert
///
/// # Example
///
/// ```
/// use tyria::util::strings_to_param;
///
/// let result = strings_to_param("id", vec!["my-id", "my-id-2"]);
/// ```
pub fn strings_to_param(param: &str, values: Vec<&str>) -> String {
    let mut result = String::new();

    // Add parameter label
    result.push_str(param);
    result.push_str("=");

    for val in values {
        result.push_str(val);
        result.push_str(",");
    }

    result
}

/// Parse an API response into the appropriate type
///
/// This expects to know the data type to use when parsing the JSON
///
/// # Arguments
///
/// * `response` - Response from the API
/// * `valid` - Valid HTTP codes that cause the data to be parsed
/// * `invalid` - Invalid HTTP codes that obtain an `APIError` with a message
///         from the API
pub fn parse_response<T>(
    response: &mut Response,
    valid: Vec<StatusCode>,
    invalid: Vec<StatusCode>
) -> Result<T, APIError> where T: DeserializeOwned {
    if valid.contains(response.status()) {
        return Ok(response.json::<T>().unwrap());

    } else if invalid.contains(response.status()) {
        return Err(response.json::<APIError>().unwrap());
    }

    Err(APIError::new(
        format!("unknown status code: {}", response.status()).as_str()
    ))
}
