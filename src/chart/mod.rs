mod world_map;

use crate::chart::world_map::WorldMap;
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

struct Chart<M: Map = WorldMap> {
    map: M,
    ship_position: [Option<Cell>; 6],
    log: Vec<Event>,
}

impl Chart {
    pub fn generate(seed: Seed, params: &Params) -> Result<Chart, GenError> {
        let radius = params.archipelago_radius;
        let islands_count = params.island_count;
        let cliff_count = params.cliff_count;
        let rock_count = params.rock_count;
        let round_cap = params.round_cap;

        let grid = WorldMap::new(radius);

        let grid = match grid {
            Ok(x) => x,
            _ => todo!(), // TODO: parse the MapError into a GenError
        };

        let ship_position = grid.corners().map(|x| Some(x));

        let chart = Chart {
            map: grid,
            ship_position,
            log: todo!(), // Add the log
        };
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
