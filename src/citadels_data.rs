#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TownCardId {
    TownHall,
    Castle,
    Market,
    Cathedral,
    Laboratory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterCardId {
    Assassin,
    Thief,
    Magician,
    King,
    Bishop,
}

#[derive(Debug)]
pub struct TownCard {
    pub id: TownCardId,
    pub name: &'static str,
    pub cost: u8,
    pub color: CardColor,
}

#[derive(Debug)]
pub struct CharacterCard {
    pub id: CharacterCardId,
    pub name: &'static str,
    pub ability: Ability,
}

#[derive(Debug)]
pub enum CardColor {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
}

#[derive(Debug)]
pub enum CharacterAbility {
    KillCharacter { target: CharacterTarget },
    StealGold { target: CharacterTarget },
    ExchangeCards,
    GainCrown,
    GainGoldPerColor(CardColor),
    ImmuneToWarlord,
}

#[derive(Debug)]
pub enum CharacterTarget {
    Any,
    ExcludeAssassin,
    ExcludeThief,
}

// ===== TOWN CARDS =====
pub static TOWN_CARDS: &[TownCard] = &[
    TownCard { id: TownCardId::TownHall, name: "Town Hall", cost: 5, color: CardColor::Yellow },
    TownCard { id: TownCardId::Castle,    name: "Castle",    cost: 4, color: CardColor::Red    },
    TownCard { id: TownCardId::Market,    name: "Market",    cost: 2, color: CardColor::Green  },
    TownCard { id: TownCardId::Cathedral, name: "Cathedral", cost: 5, color: CardColor::Blue   },
    TownCard { id: TownCardId::Laboratory,name: "Laboratory",cost: 5, color: CardColor::Purple },
];

// ===== CHARACTER CARDS =====
pub static CHARACTER_CARDS: &[CharacterCard] = &[
    CharacterCard {
        id: CharacterCardId::Assassin,
        name: "Assassin",
        ability: Ability::KillCharacter { target: CharacterTarget::Any }
    },
    CharacterCard {
        id: CharacterCardId::Thief,
        name: "Thief",
        ability: Ability::StealGold { target: CharacterTarget::ExcludeAssassin }
    },
    CharacterCard {
        id: CharacterCardId::Magician,
        name: "Magician",
        ability: Ability::ExchangeCards
    },
    CharacterCard {
        id: CharacterCardId::King,
        name: "King",
        ability: Ability::GainCrown
    },
    CharacterCard {
        id: CharacterCardId::Bishop,
        name: "Bishop",
        ability: Ability::GainGoldPerColor(CardColor::Blue)
    },
];

// ===== Lookup Helpers =====
pub fn get_town_card(id: TownCardId) -> &'static TownCard {
    &TOWN_CARDS[id as usize]
}

pub fn get_character_card(id: CharacterCardId) -> &'static CharacterCard {
    &CHARACTER_CARDS[id as usize]
}


