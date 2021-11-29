use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ActionValue{
    OneAction,
    TwoActions,
    ThreeActions,
    FreeAction,
    Reaction,
    OneToThreeAction,
    OneOrTwoActions,
    Passive,
    Other(String)
}