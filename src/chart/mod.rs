use geometry::{Cell, Map};
use shared::{Params, Seed, ShipId};

// TODO Types to implement
struct GenError {}
struct Action {}
struct ValidatedAction {}
struct Rejection {}
struct Event {}
struct ShipView {}
struct SpectatorView {}
struct Outcome {}

struct Chart {
    map: Map,
    ship_position: [Option<Cell>; 6],
    log: Vec<Event>,
}

impl Chart {
    pub fn generate(seed: Seed, params: &Params) -> Result<Chart, GenError> {
        //TODO(@TizioGrigio0)
        todo!()
    }

    pub fn validate(&self, ship: ShipId, action: &Action) -> Result<ValidatedAction, Rejection> {
        //TODO(@SpidyAml)
        todo!()
    }

    pub fn apply(&mut self, action: ValidatedAction) -> Vec<Event> {
        //TODO(@Fr4nk40016)
        todo!()
    }

    pub fn advance_round(&mut self) -> Vec<Event> {
        //TODO(@Fr4nk40016)
        todo!()
    }

    pub fn view_for(&self, ship: ShipId) -> ShipView {
        //TODO(@Davide)
        todo!()
    }

    pub fn spectator_view(&self) -> SpectatorView {
        //TODO(@SpidyAml)
        todo!()
    }

    pub fn outcome(&self) -> Option<Outcome> {
        //TODO(@Davide)
        todo!()
    }
}
