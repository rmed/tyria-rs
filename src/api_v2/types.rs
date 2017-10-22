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

use std::collections::HashMap;
use chrono::prelude::*;
use chrono::DateTime;


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
    /// Type of game the account has access to (F2P, base game, HoT, PoF etc.)
    access: Vec<String>,
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
    achievement_type: String,
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
    bit_type: String,
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
    reward_type: String,
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
    skills: CharacterSkillSets,

    /// Describes the specializations and traits equipped in PvE, PvP, and WvW
    specializations: CharacterSpecializationSet,

    /// Skill trees trained
    training: Vec<CharacterSkillTree>,

    /// WvW abilities trained by the character
    wvw_abilities: Vec<CharacterWvWAbility>,
}

/// Character backstory
#[derive(Deserialize, Debug)]
pub struct CharacterBackstory {
    /// Backstory answer IDs pertaining to character creation questions
    backstory: Vec<String>
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

/// Unlocked crafting disciplines
#[derive(Deserialize, Debug)]
pub struct CharacterCrafting {
    /// All crafting disciplines unlocked by the character
    #[serde(default)]
    crafting: Vec<CraftingDiscipline>
}

/// Current character equipment
#[derive(Deserialize, Debug)]
pub struct CharacterEquipment {
    /// Each piece of equipment currently on the character
    #[serde(default)]
    equipment: Vec<Equipment>
}

/// Character inventory
#[derive(Deserialize, Debug)]
pub struct CharacterInventory {
    /// List of bags in the inventory of the character
    #[serde(default)]
    bags: Vec<Bag>
}

/// PVP equipment setup
#[derive(Deserialize, Debug)]
pub struct CharacterPvPEquipment {
    /// ID for the equipped PvP amulet
    amulet: i32,
    /// Id for the equipped PvP rune
    rune: i32,
    /// ID for all equipped PvP sigils
    sigils: Vec<Option<i32>>
}

/// Recipes unlocked by the character
#[derive(Deserialize, Debug)]
pub struct CharacterRecipes {
    #[serde(default)]
    recipes: Vec<i32>
}

/// Current character skills
#[derive(Deserialize, Debug)]
pub struct CharacterSkills {
    skills: CharacterSkillSets
}

/// Slotted character skills per game mode
#[derive(Deserialize, Debug)]
pub struct CharacterSkillSets {
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
    specializations: CharacterSpecializationSet
}

/// Current specializations and traits in a character
#[derive(Deserialize, Debug)]
pub struct CharacterSpecializationSet {
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

/// Skill trees trained by the character
#[derive(Deserialize, Debug)]
pub struct CharacterTraining {
    #[serde(default)]
    training: Vec<CharacterSkillTree>
}

/// Skill tree item
#[derive(Deserialize, Debug)]
pub struct CharacterSkillTree {
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

/// Revenant legend details
#[derive(Deserialize, Debug)]
pub struct Legend {
    /// Legend ID
    id: String,
    /// ID of the profession skill
    swap: i32,
    /// ID of the heal skill
    heal: i32,
    /// ID of the elite skill
    elite: i32,
    /// List of IDs of the utility skills
    utilities: Vec<i32>
}

/// Mastery details
#[derive(Deserialize, Debug)]
pub struct Mastery {
    /// ID of the mastery
    id: i32,
    /// Name of the selected mastery
    name: String,
    /// Written out requirements to unlock the mastery track
    requirement: String,
    /// Order in which the mastery track appears in a list
    order: i32,
    /// Background URI for the mastery track
    background: String,
    /// In-game region in which the mastery track belongs
    region: String,
    /// Information of each mastery level
    levels: Vec<MasteryLevel>
}

/// Information on mastery levels
#[derive(Deserialize, Debug)]
pub struct MasteryLevel {
    /// Name for the given mastery
    name: String,
    /// In-game description for the given mastery
    description: String,
    /// In-game instructions for the given mastery
    instruction: String,
    /// Icon URI for the mastery
    icon: String,
    /// Amount of mastery points required to unlock the mastery
    point_cost: i32,
    /// Total amount of experience needed to train the given mastery level.
    /// This total is non-cumulative between levels
    exp_cost: i32
}

/// Outfit information
#[derive(Deserialize, Debug)]
pub struct Outfit {
    /// ID of the outfit
    id: i32,
    /// Name of the outfit
    name: String,
    /// Icon URI of the selected outfit
    icon: String,
    /// Item IDs which unlock this outfit
    unlock_items: Vec<i32>
}

/// Pet information
#[derive(Deserialize, Debug)]
pub struct Pet {
    /// Pet ID
    id: i32,
    /// Pet name
    name: String,
    /// Pet description
    description: String,
    /// Icon URI for the pet
    icon: String,
    /// Skills of the pet
    skills: Vec<PetSkill>
}

/// Pet skill details
#[derive(Deserialize, Debug)]
pub struct PetSkill {
    /// ID of the skill
    id: i32
}

/// Details on the given profession
#[derive(Deserialize, Debug)]
pub struct Profession {
    /// Profession ID
    id: String,
    /// Name of the profession
    name: String,
    /// Icon URI for the profession
    icon: String,
    /// Large icon URI for the profession
    icon_big: String,
    /// List of specialization IDs
    specializations: Vec<i32>,
    /// List of training details
    training: Vec<ProfessionTraining>,
    /// Specific flags for the profession (NoRacialSkills, NoWeaponSwap)
    #[serde(default)]
    flags: Vec<String>,
    /// Skills available to the profession
    skills: Vec<ProfessionSkill>,
    /// Weapon and weapon skills available to the profession
    weapons: HashMap<String, ProfessionWeapon>
}

/// Class skills available to the profession
#[derive(Deserialize, Debug)]
pub struct ProfessionSkill {
    /// ID of the skill
    id: i32,
    /// Where the skill can be equipped
    slot: String,
    /// Type of skill
    #[serde(rename = "type")]
    skill_type: String
}

/// Details on training for a given profession
#[derive(Deserialize, Debug)]
pub struct ProfessionTraining {
    /// ID of the item type indicated by `category`
    id: i32,
    /// Category of the training object, may be:
    /// Skills, Specializations, EliteSpecializations
    category: String,
    /// Name of the skill or specialization
    name: String,
    /// Track item details
    track: Vec<ProfessionTrainingItem>
}

/// Skills and traits belonging to a specific training track
#[derive(Deserialize, Debug)]
pub struct ProfessionTrainingItem {
    /// Cost to train this item
    cost: i32,
    /// Type of item, either a skill or a trait
    #[serde(rename = "type")]
    item_type: String,
    /// Skill ID (only if type is "Skill")
    #[serde(default)]
    skill_id: i32,
    /// Trait ID (only if type is "Trait")
    #[serde(default)]
    trait_id: i32
}

/// Weapon details for a given profession
#[derive(Deserialize, Debug)]
pub struct ProfessionWeapon {
    /// ID of the required specialization to use this weapon
    #[serde(default)]
    specialization: i32,
    /// List of weapon skills
    skills: Vec<ProfessionWeaponSkill>,
    /// Where the weapon can be equipped
    flags: Vec<String>
}

/// Weapon skills available to a profession
#[derive(Deserialize, Debug)]
pub struct ProfessionWeaponSkill {
    /// ID of the skill
    id: i32,
    /// Skill bar slot that this skill can be used in
    slot: String,
    /// Offhand weapon type this skill requires to be equipped
    #[serde(default)]
    offhand: String,
    /// Elementalist attunement that this skill requires
    #[serde(default)]
    attunement: String,
    /// Name of the class the skill was stolen from (for Thief)
    #[serde(default)]
    source: String
}

/// Playable race details
#[derive(Deserialize, Debug)]
pub struct Race {
    /// ID of the race
    id: String,
    /// Localized name of the race
    name: String,
    /// Racial skill IDs
    skills: Vec<i32>
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

/// Skill usable by players in the game
#[derive(Deserialize, Debug)]
pub struct Skill {
    /// Skill ID
    id: i32,
    name: String,
    #[serde(default)]
    description: String,
    icon: String,
    chat_link: String,
    /// Skill type (Bundle, Elite, Heal, Profession, Utility, Weapon)
    #[serde(rename = "type")]
    skill_type: String,
    /// Weapon the skill is on. Can be "None" if not applicable
    weapon_type: String,
    /// Professions that can use this skill
    professions: Vec<String>,
    /// Slot in which the skill fits into
    /// (Downed_[1-4], Pet, Profession_[1-5], Utility, Weapon_[1-5])
    slot: String,
    /// Skill facts that describe the skill's effect
    #[serde(default)]
    facts: Vec<SkillFact>,
    /// Skill facts that may apply to the skill depending on the trait choices
    #[serde(default)]
    traited_facts: Vec<SkillTraitedFact>,
}

/// Skill fact that describes the skill's effect
#[derive(Deserialize, Debug)]
pub struct SkillFact {
    text: String,
    #[serde(default)]
    icon: String,
    /// Defines additional fields of the object, can be:
    /// AttributeAdjust, Buff, ComboField, ComboFinisher,
    /// Damage, Distance, Duration, Heal, HealingADjust, NoData, Number,
    /// Percent, PrefixedBuff, Radius, Range, Recharge, Time, Unblockable
    #[serde(rename = "type")]
    fact_type: String,

    // AttributeAdjust, Number, Range, Recharge, Unblockable
    //TODO check Unblockable, it is boolean
    /// Amount that `target` gets adjusted, based on a level 80 character
    /// stats, or the number value as referenced by `text`, or the range of
    /// the trait/skill, or the recharge time in seconds, or true if type
    /// is "Unblockable"
    #[serde(default)]
    value: Option<i32>,

    // AttributeAdjust
    /// Attribute this fact adjusts. A value of "Healing" indicates the fact
    /// is a heal, and Ferocity is encoded as "CritDamage"
    #[serde(default)]
    target: Option<String>,

    // Buff, PrefixedBuff
    /// Boon, condition, or effect referred to by the fact
    #[serde(default)]
    status: Option<String>,
    /// Description of status effect if any
    #[serde(default)]
    description: Option<String>,
    /// Number of stacks applied
    #[serde(default)]
    apply_count: Option<i32>,

    // Buff, Duration, PrefixedBuff, Time
    /// Duration of the effect in seconds, or the time value in seconds
    #[serde(default)]
    duration: Option<i32>,

    // ComboField
    /// Type of field (Air, Dark, Fire, Ice, Light, Lightning, Posion, Smoke,
    /// Ethereal, Water)
    #[serde(default)]
    field_type: Option<String>,

    // ComboFinisher
    /// Type of finisher (Blast, Leap, Projectile, Whirl)
    #[serde(default)]
    finisher_type: Option<String>,

    // ComboFinisher, Percent
    /// Percent chance that the finisher will trigger or the percentage value
    /// as referenced by `text`
    #[serde(default)]
    percent: Option<i32>,

    // Damage, Heal, HealingAdjust
    /// Amount of times the damage hits or number of times the heal is applied
    #[serde(default)]
    hit_count: Option<i32>,

    /// Damage multiplier value of the skill
    #[serde(default)]
    dmg_multiplier: Option<f32>,

    // Distance, Radius
    /// Distance value or radius value
    #[serde(default)]
    distance: Option<i32>,

    // PrefixedBuff
    /// Icon to show before the fact
    #[serde(default)]
    prefix: Option<SkillFactPrefix>,
}

/// Icon to show before skill fact
#[derive(Deserialize, Debug)]
pub struct SkillFactPrefix {
    text: String,
    icon: String,
    status: String,
    description: String
}

/// Skill fact that describes the skill's effect, based on selected traits
#[derive(Deserialize, Debug)]
pub struct SkillTraitedFact {
    text: String,
    #[serde(default)]
    icon: String,
    /// Defines additional fields of the object, can be:
    /// AttributeAdjust, Buff, ComboField, ComboFinisher, Damage, Distance,
    /// Duration, Heal, HealingADjust, NoData, Number, Percent, PrefixedBuff,
    /// Radius, Range, Recharge, Time, Unblockable
    #[serde(rename = "type")]
    fact_type: String,

    /// Which trait has to be selected in order for this fact to take effect
    requires_trait: i32,
    /// Array index of the facts object this fact overrides, if the trait
    /// specified in `requires_trait` is selected. If this field is omitted,
    /// then the fact contained within this object is to be appended to the
    /// existing `facts` array
    #[serde(default)]
    overrides: Option<i32>,

    // AttributeAdjust, Number, Range, Recharge, Unblockable
    //TODO check Unblockable, it is boolean
    /// Amount that `target` gets adjusted, based on a level 80 character
    /// stats, or the number value as referenced by `text`, or the range of
    /// the trait/skill, or the recharge time in seconds, or true if type
    /// is "Unblockable"
    #[serde(default)]
    value: Option<i32>,

    // AttributeAdjust
    /// Attribute this fact adjusts. A value of "Healing" indicates the fact
    /// is a heal, and Ferocity is encoded as "CritDamage"
    #[serde(default)]
    target: Option<String>,

    // Buff, PrefixedBuff
    /// Boon, condition, or effect referred to by the fact
    #[serde(default)]
    status: Option<String>,
    /// Description of status effect if any
    #[serde(default)]
    description: Option<String>,
    /// Number of stacks applied
    #[serde(default)]
    apply_count: Option<i32>,

    // Buff, Duration, PrefixedBuff, Time
    /// Duration of the effect in seconds, or the time value in seconds
    #[serde(default)]
    duration: Option<i32>,

    // ComboField
    /// Type of field (Air, Dark, Fire, Ice, Light, Lightning, Posion, Smoke,
    /// Ethereal, Water)
    #[serde(default)]
    field_type: Option<String>,

    // ComboFinisher
    /// Type of finisher (Blast, Leap, Projectile, Whirl)
    #[serde(default)]
    finisher_type: Option<String>,

    // ComboFinisher, Percent
    /// Percent chance that the finisher will trigger or the percentage value
    /// as referenced by `text`
    #[serde(default)]
    percent: Option<i32>,

    // Damage, Heal, HealingAdjust
    /// Amount of times the damage hits or number of times the heal is applied
    #[serde(default)]
    hit_count: Option<i32>,

    /// Damage multiplier value of the skill
    #[serde(default)]
    dmg_multiplier: Option<f32>,

    // Distance, Radius
    /// Distance value or radius value
    #[serde(default)]
    distance: Option<i32>,

    // PrefixedBuff
    /// Icon to show before the fact
    #[serde(default)]
    prefix: Option<SkillFactPrefix>,
}

/// Specialization details
#[derive(Deserialize, Debug)]
pub struct Specialization {
    /// Specialization ID
    id: i32,
    /// Name of the specialization
    name: String,
    /// Profession that this specialization belongs to
    profession: String,
    /// Whether this is an elite specialization
    elite: bool,
    /// URI to the icon of the specialization
    icon: String,
    /// URI to the background of the specialization
    background: String,
    /// IDs of minor traits in the specialization
    minor_traits: Vec<i32>,
    /// IDs of major traits in the specialization
    major_traits: Vec<i32>
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
    #[serde(default)]
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
    id: i64,
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

/// Trait details
#[derive(Deserialize, Debug)]
pub struct Trait {
    /// ID of the trait
    id: i32,
    /// Name of the trait
    name: String,
    /// Icon URL of the trait
    icon: String,
    /// Description of the trait
    description: String,
    /// ID of the specialization this trait belongs to
    specialization: i32,
    /// Trait's tier (Adept, Master, Grandmaster) in a scale 0-3
    tier: i32,
    /// Either "Major" or "Minor" depending on the trait's slot
    slot: String,
    #[serde(default)]
    facts: Vec<TraitFact>,
    #[serde(default)]
    traited_facts: Vec<TraitTraitedFact>,
    #[serde(default)]
    skills: Vec<Skill>
}

/// Trait fact that describes the trait's effect
#[derive(Deserialize, Debug)]
pub struct TraitFact {
    text: String,
    #[serde(default)]
    icon: String,
    /// Defines additional fields of the object, can be:
    /// AttributeAdjust, Buff, BuffConversion ComboField, ComboFinisher,
    /// Damage, Distance, Duration, Heal, HealingADjust, NoData, Number,
    /// Percent, PrefixedBuff, Radius, Range, Recharge, Time, Unblockable
    #[serde(rename = "type")]
    fact_type: String,

    // AttributeAdjust, Number, Range, Recharge, Unblockable
    //TODO check Unblockable, it is boolean
    /// Amount that `target` gets adjusted, based on a level 80 character
    /// stats, or the number value as referenced by `text`, or the range of
    /// the trait/skill, or the recharge time in seconds, or true if type
    /// is "Unblockable"
    #[serde(default)]
    value: Option<i32>,

    // AttributeAdjust, BuffConversion
    /// Attribute this fact adjusts. A value of "Healing" indicates the fact
    /// is a heal, and Ferocity is encoded as "CritDamage"
    #[serde(default)]
    target: Option<String>,

    // Buff, PrefixedBuff
    /// Boon, condition, or effect referred to by the fact
    #[serde(default)]
    status: Option<String>,
    /// Description of status effect if any
    #[serde(default)]
    description: Option<String>,
    /// Number of stacks applied
    #[serde(default)]
    apply_count: Option<i32>,

    // Buff, Duration, PrefixedBuff, Time
    /// Duration of the effect in seconds, or the time value in seconds
    #[serde(default)]
    duration: Option<i32>,

    // BuffConversion
    /// Attribute that is used to calculate the attribute gain
    #[serde(default)]
    source: Option<String>,

    // ComboField
    /// Type of field (Air, Dark, Fire, Ice, Light, Lightning, Posion, Smoke,
    /// Ethereal, Water)
    #[serde(default)]
    field_type: Option<String>,

    // ComboFinisher
    /// Type of finisher (Blast, Leap, Projectile, Whirl)
    #[serde(default)]
    finisher_type: Option<String>,

    // ComboFinisher, Percent
    /// Percent chance that the finisher will trigger or the percentage value
    /// as referenced by `text`
    #[serde(default)]
    percent: Option<i32>,

    // Damage, Heal, HealingAdjust
    /// Amount of times the damage hits or number of times the heal is applied
    #[serde(default)]
    hit_count: Option<i32>,

    // Distance, Radius
    /// Distance value or radius value
    #[serde(default)]
    distance: Option<i32>,

    // PrefixedBuff
    /// Icon to show before the fact
    #[serde(default)]
    prefix: Option<SkillFactPrefix>,
}

/// Trait fact that describes the trait's effect, based on selected traits
#[derive(Deserialize, Debug)]
pub struct TraitTraitedFact {
    text: String,
    #[serde(default)]
    icon: String,
    /// Defines additional fields of the object, can be:
    /// AttributeAdjust, Buff, BuffConversion ComboField, ComboFinisher,
    /// Damage, Distance, Duration, Heal, HealingADjust, NoData, Number,
    /// Percent, PrefixedBuff, Radius, Range, Recharge, Time, Unblockable
    #[serde(rename = "type")]
    fact_type: String,

    /// Which trait has to be selected in order for this fact to take effect
    requires_trait: i32,
    /// Array index of the facts object this fact overrides, if the trait
    /// specified in `requires_trait` is selected. If this field is omitted,
    /// then the fact contained within this object is to be appended to the
    /// existing `facts` array
    #[serde(default)]
    overrides: Option<i32>,

    // AttributeAdjust, Number, Range, Recharge, Unblockable
    //TODO check Unblockable, it is boolean
    /// Amount that `target` gets adjusted, based on a level 80 character
    /// stats, or the number value as referenced by `text`, or the range of
    /// the trait/skill, or the recharge time in seconds, or true if type
    /// is "Unblockable"
    #[serde(default)]
    value: Option<i32>,

    // AttributeAdjust, BuffConversion
    /// Attribute this fact adjusts. A value of "Healing" indicates the fact
    /// is a heal, and Ferocity is encoded as "CritDamage"
    #[serde(default)]
    target: Option<String>,

    // Buff, PrefixedBuff
    /// Boon, condition, or effect referred to by the fact
    #[serde(default)]
    status: Option<String>,
    /// Description of status effect if any
    #[serde(default)]
    description: Option<String>,
    /// Number of stacks applied
    #[serde(default)]
    apply_count: Option<i32>,

    // Buff, Duration, PrefixedBuff, Time
    /// Duration of the effect in seconds, or the time value in seconds
    #[serde(default)]
    duration: Option<i32>,

    // BuffConversion
    /// Attribute that is used to calculate the attribute gain
    #[serde(default)]
    source: Option<String>,

    // ComboField
    /// Type of field (Air, Dark, Fire, Ice, Light, Lightning, Posion, Smoke,
    /// Ethereal, Water)
    #[serde(default)]
    field_type: Option<String>,

    // ComboFinisher
    /// Type of finisher (Blast, Leap, Projectile, Whirl)
    #[serde(default)]
    finisher_type: Option<String>,

    // ComboFinisher, Percent
    /// Percent chance that the finisher will trigger or the percentage value
    /// as referenced by `text`
    #[serde(default)]
    percent: Option<i32>,

    // Damage, Heal, HealingAdjust
    /// Amount of times the damage hits or number of times the heal is applied
    #[serde(default)]
    hit_count: Option<i32>,

    // Distance, Radius
    /// Distance value or radius value
    #[serde(default)]
    distance: Option<i32>,

    // PrefixedBuff
    /// Icon to show before the fact
    #[serde(default)]
    prefix: Option<SkillFactPrefix>,
}
