mod world_map;

use crate::chart::world_map::WorldMap;
use clues::Clue;
use geometry::{Cell, Map};
use island::island::Island;
use shared::{IslandId, Params, Round, Seed, ShipId};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

// TODO Types to implement
struct GenError {}
struct Action {}
struct ValidatedAction {}
struct Rejection {}
struct Event {}
struct ShipView {}
struct SpectatorView {}
struct Outcome {}

pub struct Report {
    pub to: BTreeSet<ShipId>,
    pub event: Event,
}
pub trait Chart: Sized {
    type ValidatedAction: Debug;
    fn generate(seed: Seed, params: &Params) -> Result<Self, GenError>;
    /// Decide whether an action is legal. The only way to obtain a ValidatedAction.
    fn validate(&self, ship: ShipId, action: &Action) -> Result<Self::ValidatedAction, Rejection>;
    /// Apply an already validated action. Cannot fail.
    fn apply(&mut self, action: Self::ValidatedAction) -> Vec<Report>;
    /// End of round: the round number advances and the Islands take their turn.
    fn advance_round(&mut self) -> Vec<Report>;
    /// What a Ship is permitted to see.
    fn view_for(&self, ship: ShipId) -> Option<ShipView>;
    /// What a spectator sees: everything.
    fn spectator_view(&self) -> SpectatorView;
    fn outcome(&self) -> Option<Outcome>;
    fn params(&self) -> Params;
    fn map(&self) -> &impl Map;
    fn weather_seed(&self) -> Seed;
    fn round(&self) -> Round;
}

pub struct MooringInfo {
    pub island: IslandId,
    pub rounds_waited: u32,
}

pub enum ShipStatus {
    Afloat {
        position: Cell,
        hull: u32,
        moored: Option<MooringInfo>,
        ally: Option<ShipId>,
        outgoing_proposal: Option<ShipId>,
        owned_clues: BTreeSet<Clue>,
    },
    Sunk {
        sunk_round: Round,
        full_strength_clues_at_sinking: usize,
    },
}

pub struct WorldChart<M: Map = WorldMap> {
    // Map and params
    map: M,
    params: Params,

    ships: BTreeMap<ShipId, ShipStatus>,
    islands: BTreeMap<IslandId, Box<dyn Island>>,

    // Secrets and original clues
    treasure: Cell,
    original_clues: BTreeSet<Clue>, // Max strenght clues generated at the start

    // Status of turn and weather
    round: Round,
    weather_seed: Seed, // A different seed from the one used to generate the map

    game_end: Option<GameEndReason>,

    // History
    log: Vec<Report>,
}

pub enum GameEndReason {
    Declared { winner: ShipId, round: Round },
    RoundCapReached,
    AllShipsSunk { round: Round },
}

impl Chart for WorldChart {
    type ValidatedAction = (); //TODO

    fn generate(seed: Seed, params: &Params) -> Result<WorldChart, GenError> {
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

        let chart = WorldChart {
            map: grid,
            params: *params,
            ships: todo!(),
            islands: todo!(),
            treasure: todo!(),
            original_clues: todo!(),
            round: 0,
            weather_seed: todo!(),
            log: todo!(), // Add the log
            game_end: None,
        };
        todo!();
    }

    fn validate(&self, ship: ShipId, action: &Action) -> Result<Self::ValidatedAction, Rejection> {
        //TODO(@SpidyAml)
        todo!()
    }

    fn apply(&mut self, action: Self::ValidatedAction) -> Vec<Report> {
        //TODO(@Fr4nk40016)
        todo!()
    }

    fn advance_round(&mut self) -> Vec<Report> {
        //TODO(@Fr4nk40016)
        todo!()
    }

    fn view_for(&self, ship: ShipId) -> Option<ShipView> {
        //TODO(@Davide)
        todo!()
    }

    fn spectator_view(&self) -> SpectatorView {
        //TODO(@SpidyAml)
        todo!()
    }

    fn outcome(&self) -> Option<Outcome> {
        //TODO(@Davide)
        todo!()
    }

    fn params(&self) -> Params {
        todo!()
    }

    fn map(&self) -> &impl Map {
        &self.map
    }

    fn weather_seed(&self) -> Seed {
        todo!()
    }

    fn round(&self) -> Round {
        todo!()
    }
}
