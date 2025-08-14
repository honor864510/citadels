use crate::citadels_types::*;

// Macro to reduce repetition for town cards
macro_rules! town {
    ($id:ident, $name:expr, $cost:expr, $color:ident) => {
        TownCard { id: TownCardId::$id, name: $name, cost: $cost, color: CardColor::$color }
    };
}

pub static TOWN_CARDS: &[TownCard] = &[
    // Blue
    town!(Temple, "Temple", 1, Blue), town!(Temple, "Temple", 1, Blue), town!(Temple, "Temple", 1, Blue),
    town!(Church, "Church", 2, Blue), town!(Church, "Church", 2, Blue),
    town!(Monastery, "Monastery", 3, Blue), town!(Monastery, "Monastery", 3, Blue),
    town!(Cathedral, "Cathedral", 5, Blue),

    // Red
    town!(Watchtower, "Watchtower", 1, Red), town!(Watchtower, "Watchtower", 1, Red),
    town!(Prison, "Prison", 2, Red), town!(Prison, "Prison", 2, Red),
    town!(Barracks, "Barracks", 3, Red),
    town!(Fortress, "Fortress", 5, Red),

    // Green
    town!(Tavern, "Tavern", 1, Green), town!(Tavern, "Tavern", 1, Green), town!(Tavern, "Tavern", 1, Green),
    town!(Market, "Market", 2, Green), town!(Market, "Market", 2, Green),
    town!(TradingPost, "Trading Post", 2, Green), town!(TradingPost, "Trading Post", 2, Green),
    town!(Docks, "Docks", 3, Green), town!(Docks, "Docks", 3, Green),
    town!(Harbor, "Harbor", 4, Green), town!(Harbor, "Harbor", 4, Green),

    // Yellow
    town!(Manor, "Manor", 3, Yellow), town!(Manor, "Manor", 3, Yellow),
    town!(Castle, "Castle", 4, Yellow), town!(Castle, "Castle", 4, Yellow),
    town!(Palace, "Palace", 5, Yellow),

    // Purple (unique)
    town!(HauntedCity, "Haunted City", 2, Purple),
    town!(Keep, "Keep", 3, Purple),
    town!(Laboratory, "Laboratory", 5, Purple),
    town!(Observatory, "Observatory", 5, Purple),
    town!(Smithy, "Smithy", 5, Purple),
    town!(University, "University", 6, Purple),
    town!(DragonGate, "Dragon Gate", 6, Purple),
];

pub static CHARACTER_CARDS: &[CharacterCard] = &[
    CharacterCard {
        id: CharacterCardId::Assassin,
        name: "Assassin",
        ability: CharacterAbility::KillCharacter { target: CharacterTarget::Any }
    },
    CharacterCard {
        id: CharacterCardId::Thief,
        name: "Thief",
        ability: CharacterAbility::StealGold { target: CharacterTarget::ExcludeAssassin }
    },
    CharacterCard {
        id: CharacterCardId::Magician,
        name: "Magician",
        ability: CharacterAbility::ExchangeCards
    },
    CharacterCard {
        id: CharacterCardId::King,
        name: "King",
        ability: CharacterAbility::GainCrown
    },
    CharacterCard {
        id: CharacterCardId::Bishop,
        name: "Bishop",
        ability: CharacterAbility::ImmuneToWarlord
    },
    CharacterCard {
        id: CharacterCardId::Merchant,
        name: "Merchant",
        ability: CharacterAbility::ExtraGoldPerColor(CardColor::Green)
    },
    CharacterCard {
        id: CharacterCardId::Architect,
        name: "Architect",
        ability: CharacterAbility::DrawExtraCards(2)
    },
    CharacterCard {
        id: CharacterCardId::Warlord,
        name: "Warlord",
        ability: CharacterAbility::DestroyBuilding { cost_discount: 1 }
    },
];

