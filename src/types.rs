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

/// Type definitions for the deserialization of API results

/// API errors
#[derive(Deserialize, Debug)]
pub struct APIError {
    text: String
}

/// Used when defining miscelaneous errors
impl APIError {
    pub fn new(text: &str) -> APIError {
        APIError {
            text: text.to_string()
        }
    }
}

/// Player achievements
#[derive(Deserialize, Debug)]
pub struct Achievement {
    id: i32,
    #[serde(default)]
    icon: String,
    name: String,
    description: String,
    requirement: String,
    locked_text: String,
    #[serde(rename = "type")]
    kind: String,
    flags: Vec<String>,
    tiers: Vec<AchievementTier>,
    #[serde(default)]
    prerequisites: Vec<i32>,
    #[serde(default)]
    rewards: Vec<AchievementReward>,
    #[serde(default)]
    bits: Vec<AchievementBit>,
    #[serde(default)]
    point_cap: i32
}

/// Achievement bits
#[derive(Deserialize, Debug)]
pub struct AchievementBit {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    id: i32,
    #[serde(default)]
    text: String
}

/// Achievement categories
#[derive(Deserialize, Debug)]
pub struct AchievementCategory {
    id: i32,
    name: String,
    description: String,
    order: i32,
    icon: String,
    achievements: Vec<i32>
}

/// Achievement groups
#[derive(Deserialize, Debug)]
pub struct AchievementGroup {
    id: String,
    name: String,
    description: String,
    order: i32,
    categories: Vec<i32>
}

/// Achievement awards
///
/// Reward types may be:
///
/// - "Coins": uses attribute `count`
/// - "Item": uses attributes `id` and `count`
/// - "Mastery": uses attributes `id` and `region`
/// - "Title": uses attribute `id`
#[derive(Deserialize, Debug)]
pub struct AchievementReward {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    id: i32,
    #[serde(default)]
    count: i32,
    #[serde(default)]
    region: String
}

/// Achievement tiers
///
/// This is used for achievements that can be repeated, showing the item count
/// necessary to unlock the next tier and the points awarded.
#[derive(Deserialize, Debug)]
pub struct AchievementTier {
    count: i32,
    points: i32
}

/// Daily achievements
#[derive(Deserialize, Debug)]
pub struct DailyAchievements {
    pve: Vec<DailyAchievement>,
    pvp: Vec<DailyAchievement>,
    wvw: Vec<DailyAchievement>,
    fractals: Vec<DailyAchievement>,
    special: Vec<DailyAchievement>
}

/// Daily achievement item
#[derive(Deserialize, Debug)]
pub struct DailyAchievement {
    id: i32,
    level: DailyAchievementLevel,
    required_access: Vec<String>
}

/// Level range for the daily achievement
#[derive(Deserialize, Debug)]
pub struct DailyAchievementLevel {
    min: i32,
    max: i32
}
