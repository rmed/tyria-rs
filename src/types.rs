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
    /// Error description provided by the API
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

/// API key details
#[derive(Deserialize, Debug)]
pub struct APIKey {
    /// Requested API key
    id: String,
    /// Name given to the API key by the account owner (not escaped!)
    name: String,
    /// Which permissions the API key has
    permissions: Vec<String>
}

/// User account
#[derive(Deserialize, Debug)]
pub struct Account {
    /// Unique persisten account GUID
    id: String,
    /// Age of the account in seconds
    age: i32,
    /// Unique account name with numerical suffix
    name: String,
    /// ID of the home world the account is assigned to
    world: i32,
    /// List of guilds assigned to the given account
    #[serde(default)]
    guilds: Vec<String>,
    /// List of guilds the account is leader of
    #[serde(default)]
    guild_leader: Vec<String>,
    /// Timestamp of when the account was created
    created: DateTime<Utc>,
    /// Type of game the account has access to (F2P, base game, HoT, etc.)
    access: String,
    /// True if the player has bought a commander tag
    commander: bool,
    /// Account's personal fractal reward level (requires `progression` scope)
    #[serde(default)]
    fractal_level: i32,
    /// Account's daily AP (requires `progression` scope)
    #[serde(default)]
    daily_ap: i32,
    /// Account's monthly AP (requires `progression` scope)
    #[serde(default)]
    monthly_ap: i32,
    /// Account's personal WvW rank (requires `progression` scope)
    #[serde(default)]
    wvw_rank: i32
}

/// Achievements that the account has progress on
#[derive(Deserialize, Debug)]
pub struct AccountAchievement {
    /// Achievement ID
    id: i32,
    /// Player's current progress towards the achievement (if any)
    #[serde(default)]
    current: i32,
    /// Amount needed to complete the achievements (if any).
    /// Most WvW achievements have this set to `-1`
    #[serde(default)]
    max: i32,
    /// Whether or not the achievement is done
    done: bool,
    /// Number of times the achievement has been completed (if repeatable)
    #[serde(default)]
    repeated: i32,
    /// Bits giving more information on the progress for the achievement
    #[serde(default)]
    bits: Vec<i32>
}

/// Currencies in an account's wallet
#[derive(Deserialize, Debug)]
pub struct AccountCurrency {
    /// ID of the currency
    id: i32,
    /// Amount of this currency
    value: i32
}

/// Finishers unlocked for the account
#[derive(Deserialize, Debug)]
pub struct AccountFinisher {
    /// ID of the finisher
    id: i32,
    /// Indicates if the finisher is permanent or temporary
    permanent: bool,
    /// If not permanent, indicates the remaining uses
    #[serde(default)]
    quantity: i32,
}

/// Unlocked masteries for the account
#[derive(Deserialize, Debug)]
pub struct AccountMastery {
    /// ID of the mastery
    id: i32,
    /// Level at which the mastery is on the account
    level: i32
}

/// Materials stored in the account's vault
#[derive(Deserialize, Debug)]
pub struct AccountMaterial {
    /// Item ID of the material
    id: i32,
    /// Material category the item belongs to
    category: i32,
    /// Number of the material that is stored in the account vault
    count: i32
}

/// Player achievements
#[derive(Deserialize, Debug)]
pub struct Achievement {
    /// Achievement ID
    id: i32,
    /// Achievement icon (if any)
    #[serde(default)]
    icon: String,
    /// Achievement name
    name: String,
    /// Achievement description
    description: String,
    /// Achievement requirement as listed in-game
    requirement: String,
    /// Achievement description prior to unlocking it
    locked_text: String,
    /// Achievement type
    #[serde(rename = "type")]
    kind: String,
    /// Achievement categories
    flags: Vec<String>,
    /// Describes the achievement's tiers
    tiers: Vec<AchievementTier>,
    /// Achievement IDs required to progress the given achievement
    #[serde(default)]
    prerequisites: Vec<i32>,
    /// Describes the rewards given for the achievement
    #[serde(default)]
    rewards: Vec<AchievementReward>,
    /// Bitmask value that can give futher information on achievement progress
    #[serde(default)]
    bits: Vec<AchievementBit>,
    /// Maximum number of AP that can be rewarded by a repeatable achievement
    #[serde(default)]
    point_cap: i32
}

/// Achievement bits
#[derive(Deserialize, Debug)]
pub struct AchievementBit {
    /// Type of bit (`Text`, `Item`, `Minipet`, `Skin`)
    #[serde(rename = "type")]
    kind: String,
    /// ID of the item, mini, or skin, if applicable
    #[serde(default)]
    id: i32,
    /// Text for the bit if type is `Text`
    #[serde(default)]
    text: String
}

/// Achievement categories
#[derive(Deserialize, Debug)]
pub struct AchievementCategory {
    /// Category's ID
    id: i32,
    /// Category name
    name: String,
    /// Category description
    description: String,
    /// Describes where to sort this category among the other categories in
    /// its group. Lowest numbers go first, highest numbers go last
    order: i32,
    /// URL to an image for the icon of the category
    icon: String,
    /// Achievement IDs that this category contains
    achievements: Vec<i32>
}

/// Achievement groups
#[derive(Deserialize, Debug)]
pub struct AchievementGroup {
    /// Group's ID
    id: String,
    /// Group name
    name: String,
    /// Group description
    description: String,
    /// Describes where to sort this group among other groups.
    /// Lowest numbers go first, highest numbers go last
    order: i32,
    /// Category IDs that this group contains
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
    /// Type of reward (`Coins`, `Item`, `Mastery`, `Title`)
    #[serde(rename = "type")]
    kind: String,
    /// ID of reward (when type is `Item`, `Mastery`, or `Title`)
    #[serde(default)]
    id: i32,
    /// Number of items awarded (when type is `Item`)
    #[serde(default)]
    count: i32,
    /// Region in which the Mastery Point applies to (when type is `Mastery`)
    #[serde(default)]
    region: String
}

/// Achievement tiers
///
/// This is used for achievements that can be repeated, showing the item count
/// necessary to unlock the next tier and the points awarded.
#[derive(Deserialize, Debug)]
pub struct AchievementTier {
    /// Number of "things" that must be completed to achieve this tier
    count: i32,
    /// Amount of AP awarded for completing this tier
    points: i32
}

/// Equiped bags in a character
#[derive(Deserialize, Debug)]
pub struct Bag {
    /// Item ID of the bag
    id: i32,
    /// Amount of slogs available in this bag
    size: i32,
    /// Describes item slots. If no item is in the specific slot, its value
    /// will be `None`
    #[serde(default)]
    inventory: Vec<Option<BagSlot>>
}

/// Bag slot
#[derive(Deserialize, Debug)]
pub struct BagSlot {
    /// Item ID
    id: i32,
    /// Amount of item in the stack (min: 1, max: 250)
    count: i32,
    /// List of infusion item IDs (if any)
    #[serde(default)]
    infusions: Vec<i32>,
    /// List of upgrade component item IDs (if any)
    #[serde(default)]
    upgrades: Vec<i32>,
    /// Skin ID for the given equipment piece (if any)
    #[serde(default)]
    skin: i32,
    /// Contains information on the stats chosen if the item offers an option
    /// for stats/prefix
    #[serde(default)]
    stats: Option<EquipmentStats>,
    /// Describes which type of binding the item has
    #[serde(default)]
    binding: String,
    /// If character bound, name of the character the item is bound to
    #[serde(default)]
    bound_to: String
}

/// Item slot in the bank
#[derive(Deserialize, Debug)]
pub struct BankSlot {
    /// Item's ID
    id: i32,
    /// Amount of items in the item stack
    count: i32,
    /// The skin applied to the item, if it is different from its original
    #[serde(default)]
    skin: i32,
    /// Item IDs for each rune or signet applied to the item
    #[serde(default)]
    upgrades: Vec<i32>,
    /// Item IDs for each infusion applied to the item
    #[serde(default)]
    infusions: Vec<i32>,
    /// Current binding of the item
    #[serde(default)]
    binding: String,
    /// Amount of charges remaining on the item
    #[serde(default)]
    charges: i32,
    /// If `binding` is `Character`, which character the item is bound to
    #[serde(default)]
    bound_to: String
}

/// Home instance cats
#[derive(Deserialize, Debug)]
pub struct Cat {
    /// ID for the cat
    id: i32,
    /// Hint to identify what is needed for each cat
    #[serde(default)]
    hint: String
}

/// Character information
#[derive(Deserialize, Debug)]
pub struct Character {
    /// Backstory answer IDs pertaining to the questions answered during
    /// character creation
    #[serde(default)]
    backstory: Vec<String>,

    /// Character's name
    name: String,
    /// Character's race
    race: String,
    /// Character's gender
    gender: String,
    /// Character's profession
    profession: String,
    /// Character's level
    level: i32,
    /// Guild ID of the character's currently represented guild (if any)
    #[serde(default)]
    guild: String,
    /// Amount of seconds this character was played
    age: i32,
    /// Timestamp of the character's creation time
    created: DateTime<Utc>,
    /// Amount of times this character has been defeated
    deaths: i32,
    /// Currently selected title ID for the character
    #[serde(default)]
    title: i32,

    /// List of crafting disciplines the character has unlocked
    crafting: Vec<CraftingDiscipline>,

    /// List of pieces of equipment currently on the character
    equipment: Vec<Equipment>,
    /// Contains information on character's PvP equipment setup
    equipment_pvp: CharacterPvPEquipment,

    /// Describes bags in the character's inventory
    bags: Vec<Bag>,

    /// List of recipe IDs unlocked by the character
    recipes: Vec<i32>,

    /// Describes the utility skills equipped in PvE, PvP, and WvW
    skills: CharacterSkills,

    /// Describes the specializations and traits equipped in PvE, PvP, and WvW
    specializations: CharacterSpecializations,

    /// Skill trees trained
    training: Vec<CharacterTraining>,

    /// WvW abilities trained by the character
    wvw_abilities: Vec<CharacterWvWAbility>,
}

/// Core information of a character
#[derive(Deserialize, Debug)]
pub struct CharacterCore {
    /// Character's name
    name: String,
    /// Character's race
    race: String,
    /// Character's gender
    gender: String,
    /// Character's profession
    profession: String,
    /// Character's level
    level: i32,
    /// Guild ID of the character's currently represented guild (if any)
    #[serde(default)]
    guild: String,
    /// Amount of seconds this character was played
    age: i32,
    /// Timestamp of the character's creation time
    created: DateTime<Utc>,
    /// Amount of times this character has been defeated
    deaths: i32,
    /// Currently selected title ID for the character
    #[serde(default)]
    title: i32,
}

/// PVP equipment setup
#[derive(Deserialize, Debug)]
pub struct CharacterPvPEquipment {
    /// ID for the equipped PvP amulet
    amulet: i32,
    /// Id for the equipped PvP rune
    rune: i32,
    /// ID for all equipped PvP sigils
    sigils: Vec<i32>
}

/// Slotted character skills per game mode
#[derive(Deserialize, Debug)]
pub struct CharacterSkills {
    /// PvE character skill set
    pve: CharacterSkillSet,
    /// PvP character skill set
    pvp: CharacterSkillSet,
    /// WvW character skill set
    wvw: CharacterSkillSet
}

/// Set of skills slotted
#[derive(Deserialize, Debug)]
pub struct CharacterSkillSet {
    /// Skill ID for the heal skill
    heal: i32,
    /// List of skill IDs for the equipped utilities
    utilities: Vec<i32>,
    /// Skill ID for the elite skill
    elite: i32
}

/// Current specializations and traits in a character
#[derive(Deserialize, Debug)]
pub struct CharacterSpecializations {
    /// PvE character specializations
    pve: Vec<CharacterSpecialization>,
    /// PvP character specializations
    pvp: Vec<CharacterSpecialization>,
    /// WvW character specializations
    wvw: Vec<CharacterSpecialization>
}

/// Current specializations and traits in a character
#[derive(Deserialize, Debug)]
pub struct CharacterSpecialization {
    /// Specialization ID
    id: i32,
    /// List of IDs for each selected trait
    traits: Vec<i32>
}

/// Skill tree item
#[derive(Deserialize, Debug)]
pub struct CharacterTraining {
    /// Skill tree ID
    id: i32,
    /// Shows how many hero points have been spent in this tree
    spent: i32,
    /// States whether or not the tree is fully trained
    done: bool
}

/// Character WvW abilities
#[derive(Deserialize, Debug)]
pub struct CharacterWvWAbility {
    /// AbilityID
    id: i32,
    /// Current rank for the given ability
    rank: i32
}

/// A character's crafting discipline
#[derive(Deserialize, Debug)]
pub struct CraftingDiscipline {
    /// Name of the discipline
    discipline: String,
    /// Current crafting level for the given discipline and character
    rating: i32,
    /// Describes if the given discipline is currently active on the character
    active: bool
}

/// Daily achievement item
#[derive(Deserialize, Debug)]
pub struct DailyAchievement {
    /// Achievement ID
    id: i32,
    /// Level requirement for the daily to appear
    level: DailyAchievementLevel,
    /// Which Guild Wars 2 campaigns are required to see this daily achievement
    required_access: Vec<String>
}

/// Level range for the daily achievement
#[derive(Deserialize, Debug)]
pub struct DailyAchievementLevel {
    /// Minimum level. Any character below this level will not see the
    /// daily achievemtn
    min: i32,
    /// Maximum level. Any character above this level will not see the
    /// daily achievemtn
    max: i32
}

/// Daily achievements
#[derive(Deserialize, Debug)]
pub struct DailyAchievements {
    /// PvE daily achievements
    pve: Vec<DailyAchievement>,
    /// PvP daily achievements
    pvp: Vec<DailyAchievement>,
    /// WvW daily achievements
    wvw: Vec<DailyAchievement>,
    /// Fractals daily achievements
    fractals: Vec<DailyAchievement>,
    /// Special daily achievements
    special: Vec<DailyAchievement>
}

/// Piece of equipment on a character
#[derive(Deserialize, Debug)]
pub struct Equipment {
    /// Item ID
    id: i32,
    /// Equipment slot in which the item is slotted
    slot: String,
    /// List of infusion item IDs on the piece of equipment
    #[serde(default)]
    infusions: Vec<i32>,
    /// List of upgrade component item IDs on the piece of equipment
    #[serde(default)]
    upgrades: Vec<i32>,
    /// Skin ID for the given equipment piece
    #[serde(default)]
    skin: i32,
    /// Information on the stats chosen if the item offers an option for
    /// stats/prefix
    #[serde(default)]
    stats: Option<EquipmentStats>,
    /// Describes which kind of binding the item has
    #[serde(default)]
    binding: String,
    /// The amount of charges remaining on the item
    #[serde(default)]
    charges: i32,
    /// If bound, name of the character the item is bound to
    #[serde(default)]
    bound_to: String,
    /// List of selected dyes for the piece. Values default to `None` if no
    /// dye is selected
    #[serde(default)]
    dyes: Vec<Option<i32>>
}

/// Summary of the stats on an item
#[derive(Deserialize, Debug)]
pub struct EquipmentAttributes {
    /// Amount of Power given
    #[serde(default)]
    #[serde(rename = "Power")]
    power: i32,
    /// Amount of Precision given
    #[serde(default)]
    #[serde(rename = "Precision")]
    precision: i32,
    /// Amount of Toughness given
    #[serde(default)]
    #[serde(rename = "Toughness")]
    toughness: i32,
    /// Amount of Vitality given
    #[serde(default)]
    #[serde(rename = "Vitality")]
    vitality: i32,
    /// Amount of Condition Damage given
    #[serde(default)]
    #[serde(rename = "ConditionDamage")]
    condition_damage: i32,
    /// Amount of Condition Duration given
    #[serde(default)]
    #[serde(rename = "ConditionDuration")]
    condition_duration: i32,
    /// Amount of Critical Damage given
    #[serde(default)]
    #[serde(rename = "CritDamage")]
    critical_damage: i32,
    /// Amount of Healing Power given
    #[serde(default)]
    #[serde(rename = "Healing")]
    healing: i32,
    /// Amount of Boon duration given
    #[serde(default)]
    #[serde(rename = "BoonDuration")]
    boon_duration: i32
}

/// Chosen stats of an equiped item
#[derive(Deserialize, Debug)]
pub struct EquipmentStats {
    /// Itemstat ID
    id: i32,
    /// Summary of the stats on the item
    #[serde(default)]
    attributes: Option<EquipmentAttributes>,
}

/// Details on currency exchange rate
#[derive(Deserialize, Debug)]
pub struct ExchangeRate {
    /// Number of coins required for a single gem, or the number of coins
    /// obtained for a single gem
    coins_per_gem: i32,
    /// Number of gems obtained for the specified quantity of coins, or the
    /// number of coins obtained for the specified quantity of gems
    quantity: i32
}

/// Shared inventory slot
#[derive(Deserialize, Debug)]
pub struct InventorySlot {
    /// Item ID
    id: i32,
    /// Number of this item in the stack
    count: i32,
    /// Scope of the inventory slot
    #[serde(default)]
    binding: String
}

/// Character progress in Super Adventure Box
#[derive(Deserialize, Debug)]
pub struct SABProgress {
    /// Describes which worlds, and in which difficulty, have been cleared
    #[serde(default)]
    zones: Vec<SABZone>,
    /// Describes the unlocks on the given character
    #[serde(default)]
    unlocks: Vec<SABUnlock>,
    /// Unlocked songs on the character
    #[serde(default)]
    songs: Vec<SABSong>
}

/// Specifies unlocked songs on the character
#[derive(Deserialize, Debug)]
pub struct SABSong {
    /// ID of the song
    id: i32,
    /// Name of the song
    name: String
}

/// Specifies unlocks on a character
#[derive(Deserialize, Debug)]
pub struct SABUnlock {
    /// ID of the unlock
    id: i32,
    /// Name of the upgrade
    name: String
}

/// Specifies which worlds, and in which difficulty, a character has cleared
#[derive(Deserialize, Debug)]
pub struct SABZone {
    /// World ID
    id: i32,
    /// Difficulty mode cleared
    mode: String,
    /// World number
    world: i32,
    /// Zone number
    zone: i32
}

/// Item listed in the trading post
#[derive(Deserialize, Debug)]
pub struct TPItem {
    /// Item ID
    id: i32,
    /// A list of all buy listings, ascending from lowest buy order
    #[serde(default)]
    buys: Vec<TPItemListing>,
    /// A list of all sell listings, ascending from lowest sell offer
    #[serde(default)]
    sells: Vec<TPItemListing>
}

/// Information about an item in the trading post
#[derive(Deserialize, Debug)]
pub struct TPItemInfo {
    /// Number ID
    id: i32,
    /// Whether a free to play account can purchase or sell the item in the
    /// trading post
    whitelisted: bool,
    /// Buy information
    buys: TPItemInfoPrice,
    /// Sell information
    sells: TPItemInfoPrice
}

/// Price information on an item
#[derive(Deserialize, Debug)]
pub struct TPItemInfoPrice {
    /// Highest buy order or lowest sell offer price in coins
    unit_price: i32,
    /// Amount of items being sold/bought
    quantity: i32
}

/// Trading post item listing details
#[derive(Deserialize, Debug)]
pub struct TPItemListing {
    /// Number of individual listings this object refers to (e.g. two players
    /// selling at the same price will end up in the same listing)
    listings: i32,
    /// Sell offer or buy order price in coins
    unit_price: i32,
    /// Amount of items being sold/bought in this listing
    quantity: i32
}

/// Trading post transactions for an account
#[derive(Deserialize, Debug)]
pub struct TPTransaction {
    /// ID of the transaction
    id: i32,
    /// Item ID
    item_id: i32,
    /// Price of the item in coins
    price: i32,
    /// Quantity of the item
    quantity: i32,
    /// Date of creation of the transaction
    created: DateTime<Utc>,
    /// Date of purchase (only for past transactions)
    purchased: Option<DateTime<Utc>>
}
