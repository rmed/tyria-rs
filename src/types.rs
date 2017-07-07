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

use chrono::prelude::*;
use chrono::DateTime;


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

/// User account
#[derive(Deserialize, Debug)]
pub struct Account {
    id: String,
    age: i32,
    name: String,
    world: i32,
    #[serde(default)]
    guilds: Vec<String>,
    #[serde(default)]
    guild_leader: Vec<String>,
    created: DateTime<Utc>,
    access: String,
    commander: bool,
    #[serde(default)]
    fractal_level: i32,
    #[serde(default)]
    daily_ap: i32,
    #[serde(default)]
    monthly_ap: i32,
    #[serde(default)]
    wvw_rank: i32
}

/// Achievements that the account has progress on
#[derive(Deserialize, Debug)]
pub struct AccountAchievement {
    id: i32,
    #[serde(default)]
    current: i32,
    #[serde(default)]
    max: i32,
    done: bool,
    #[serde(default)]
    repeated: i32,
    #[serde(default)]
    bits: Vec<i32>
}

/// Currencies in an account's wallet
#[derive(Deserialize, Debug)]
pub struct AccountCurrency {
    id: i32,
    value: i32
}

/// Finishers unlocked for the account
#[derive(Deserialize, Debug)]
pub struct AccountFinisher {
    id: i32,
    permanent: bool,
    #[serde(default)]
    quantity: i32,
}

/// Unlocked masteries for the account
#[derive(Deserialize, Debug)]
pub struct AccountMastery {
    id: i32,
    level: i32
}

/// Materials stored in the account's vault
#[derive(Deserialize, Debug)]
pub struct AccountMaterial {
    id: i32,
    category: i32,
    count: i32
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

/// Equiped bags in a character
#[derive(Deserialize, Debug)]
pub struct Bag {
    id: i32,
    size: i32,
    inventory: Vec<BagSlot>
}

/// Bag slot
#[derive(Deserialize, Debug)]
pub struct BagSlot {
    id: i32,
    count: i32,
    #[serde(default)]
    infusions: Vec<i32>,
    #[serde(default)]
    upgrades: Vec<i32>,
    #[serde(default)]
    skin: i32,
    #[serde(default)]
    stats: Option<EquipmentStats>,
    #[serde(default)]
    binding: String,
    #[serde(default)]
    bound_to: String
}

/// Item slot in the bank
#[derive(Deserialize, Debug)]
pub struct BankSlot {
    id: i32,
    count: i32,
    #[serde(default)]
    skin: i32,
    #[serde(default)]
    upgrades: Vec<i32>,
    #[serde(default)]
    infusions: Vec<i32>,
    #[serde(default)]
    binding: String,
    #[serde(default)]
    charges: i32,
    #[serde(default)]
    bound_to: String
}

/// Home instance cats
#[derive(Deserialize, Debug)]
pub struct Cat {
    id: i32,
    #[serde(default)]
    hint: String
}

/// Character information
#[derive(Deserialize, Debug)]
pub struct Character {
    // Backstory
    #[serde(default)]
    backstory: Vec<String>,

    // Core
    name: String,
    race: String,
    gender: String,
    profession: String,
    level: i32,
    #[serde(default)]
    guild: String,
    age: i32,
    created: DateTime<Utc>,
    deaths: i32,
    #[serde(default)]
    title: i32,

    // Crafting
    crafting: Vec<CraftingDiscipline>,

    // Equipment
    equipment: Vec<Equipment>,
    equipment_pvp: CharacterPvPEquipment,

    // Inventory
    bags: Vec<Bag>,

    // Recipes
    recipes: Vec<i32>,

    // Skills
    skills: CharacterSkills,

    // Specializations
    specializations: CharacterSpecializations,

    // Training
    training: Vec<CharacterTraining>,

    // WvW abilities
    wvw_abilities: Vec<CharacterWvWAbility>,
}

/// Core information of a character
#[derive(Deserialize, Debug)]
pub struct CharacterCore {
    name: String,
    race: String,
    gender: String,
    profession: String,
    level: i32,
    #[serde(default)]
    guild: String,
    age: i32,
    created: DateTime<Utc>,
    deaths: i32,
    #[serde(default)]
    title: i32,
}

/// PVP equipment setup
#[derive(Deserialize, Debug)]
pub struct CharacterPvPEquipment {
    amulet: i32,
    rune: i32,
    sigils: Vec<i32>
}

/// Slotted character skills per game mode
#[derive(Deserialize, Debug)]
pub struct CharacterSkills {
    pve: CharacterSkillSet,
    pvp: CharacterSkillSet,
    wvw: CharacterSkillSet
}

/// Set of skills slotted
#[derive(Deserialize, Debug)]
pub struct CharacterSkillSet {
    heal: i32,
    utilities: Vec<i32>,
    elite: i32
}

/// Current specializations and traits in a character
#[derive(Deserialize, Debug)]
pub struct CharacterSpecializations {
    pve: Vec<CharacterSpecialization>,
    pvp: Vec<CharacterSpecialization>,
    wvw: Vec<CharacterSpecialization>
}

/// Current specializations and traits in a character
#[derive(Deserialize, Debug)]
pub struct CharacterSpecialization {
    id: i32,
    traits: Vec<i32>
}

/// Skill tree item
#[derive(Deserialize, Debug)]
pub struct CharacterTraining {
    id: i32,
    spent: i32,
    done: bool
}

/// Character WvW abilities
#[derive(Deserialize, Debug)]
pub struct CharacterWvWAbility {
    id: i32,
    rank: i32
}

/// A character's crafting discipline
#[derive(Deserialize, Debug)]
pub struct CraftingDiscipline {
    discipline: String,
    rating: i32,
    active: bool
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

/// Piece of equipment on a character
#[derive(Deserialize, Debug)]
pub struct Equipment {
    id: i32,
    slot: String,
    #[serde(default)]
    infusions: Vec<i32>,
    #[serde(default)]
    upgrades: Vec<i32>,
    #[serde(default)]
    skin: i32,
    #[serde(default)]
    stats: Option<EquipmentStats>,
    #[serde(default)]
    binding: String,
    #[serde(default)]
    charges: i32,
    #[serde(default)]
    bound_to: String,
    #[serde(default)]
    dyes: Vec<i32>
}

/// Chosen stats of an equiped item
#[derive(Deserialize, Debug)]
pub struct EquipmentStats {
    id: i32,
    #[serde(default)]
    attributes: Option<EquipmentAttributes>,
}

/// Summary of the stats on an item
#[derive(Deserialize, Debug)]
pub struct EquipmentAttributes {
    #[serde(default)]
    #[serde(rename = "Power")]
    power: i32,
    #[serde(default)]
    #[serde(rename = "Precision")]
    precision: i32,
    #[serde(default)]
    #[serde(rename = "Toughness")]
    toughness: i32,
    #[serde(default)]
    #[serde(rename = "Vitality")]
    vitality: i32,
    #[serde(default)]
    #[serde(rename = "ConditionDamage")]
    condition_damage: i32,
    #[serde(default)]
    #[serde(rename = "ConditionDuration")]
    condition_duration: i32,
    #[serde(default)]
    #[serde(rename = "CritDamage")]
    critical_damage: i32,
    #[serde(default)]
    #[serde(rename = "Healing")]
    healing: i32,
    #[serde(default)]
    #[serde(rename = "BoonDuration")]
    boon_duration: i32
}

/// Shared inventory slot
#[derive(Deserialize, Debug)]
pub struct InventorySlot {
    id: i32,
    count: i32,
    #[serde(default)]
    binding: String
}

/// Character progress in Super Adventure Box
#[derive(Deserialize, Debug)]
pub struct SABProgress {
    #[serde(default)]
    zones: Vec<SABZone>,
    #[serde(default)]
    unlocks: Vec<SABUnlock>,
    #[serde(default)]
    songs: Vec<SABSong>
}

/// Specifies unlocked songs on the character
#[derive(Deserialize, Debug)]
pub struct SABSong {
    id: i32,
    name: String
}

/// Specifies unlocks on a character
#[derive(Deserialize, Debug)]
pub struct SABUnlock {
    id: i32,
    name: String
}

/// Specifies which worlds, and in which difficulty, a character has cleared
#[derive(Deserialize, Debug)]
pub struct SABZone {
    id: i32,
    mode: String,
    world: i32,
    zone: i32
}
