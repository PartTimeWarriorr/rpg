
use slotmap::*;
use crate::characters::CharacterId;
use crate::ability::Ability;
use crate::action::ActionType;

new_key_type! { struct MenuNodeKey; }

struct MenuNode {
    value: MenuNodeValue,
    label: String,
    children: Vec<MenuNodeKey>,
    parent: Option<MenuNodeKey>,
}

enum MenuNodeValue {
    CharacterNode(CharacterId),
    ActionTypeNode(ActionType),
    AbilityNode(Ability),
}

struct Menu {
    current: MenuNodeKey,
    nodes: SlotMap<MenuNodeKey, MenuNodeValue>,
}