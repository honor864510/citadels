#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TownCardId {
    Temple, Church, Monastery, Cathedral,
    Watchtower, Prison, Barracks, Fortress,
    Tavern, Market, TradingPost, Docks, Harbor,
    Manor, Castle, Palace,
    HauntedCity, Keep, Laboratory, Observatory, Smithy, University, DragonGate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterCardId {
    Assassin, Thief, Magician, King, Bishop, Merchant, Architect, Warlord,
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
    pub ability: CharacterAbility,
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
    ExtraGoldPerColor(CardColor),
    DrawExtraCards(usize),
    DestroyBuilding { cost_discount: u8 },
    ImmuneToWarlord,
}

#[derive(Debug)]
pub enum CharacterTarget {
    Any,
    ExcludeAssassin,
    ExcludeThief,
}

