use geometry::{Cell, DefaultMap, Direction};
use shared::{IslandId, Params, Round, Seed, ShipId};
use std::collections::HashMap;
use wind::wind_at;

// TODO: Types to implement
struct GenError {}

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Action {
    Move { to: Cell },
    Moor { island: IslandId },
    AskClue,
    Bombard { target: ShipId },
    ProposeAlliance { to: ShipId },
    AcceptAlliance { proposer: ShipId },
    BreakAlliance,
    Declare,
    Hold,
}

struct ValidatedAction {
    action: Action,
    ship_id: ShipId,
}

struct Rejection {}

struct Event {
    message: String,
}

struct ShipView {}
struct SpectatorView {}
struct Outcome {}

struct Chart {
    map: DefaultMap,
    // Discuss with the team: using a fixed-size array doesn't make much sense here
    ship_position: [Option<Cell>; 6],
    ship_position_provvisorio: HashMap<ShipId, Cell>,
    // Same applies to the islands
    moored_ships_status: HashMap<ShipId, Option<(IslandId, Round)>>,
    log: Vec<Event>,
    // Added by Francesco
    treasure_cell: Cell,
    round: Round,
    round_cap: Round,
    is_eneded: bool,
    // Added for wind mechanics
    wind_seed: Seed,
    direction: Direction,
}

impl Chart {
    pub fn generate(seed: Seed, params: &Params) -> Result<Chart, GenError> {
        // TODO(@TizioGrigio0)
        todo!()
    }

    pub fn validate(&self, ship: ShipId, action: &Action) -> Result<ValidatedAction, Rejection> {
        // TODO(@SpidyAml)
        todo!()
    }

    pub fn apply(&mut self, action: ValidatedAction) -> Vec<Event> {
        // TODO(@Fr4nk40016)
        let mut events_vec: Vec<Event> = Vec::new();

        match action.action {
            Action::Move { to } => {
                let ship_pos = self.ship_position_provvisorio.get_mut(&action.ship_id);
                if let Some(ship_pos_unwrapped) = ship_pos {
                    *ship_pos_unwrapped = to;
                }

                events_vec.push(Event {
                    message: format!("The ship {:?} moved to {:?}", &action.ship_id, &to),
                })
            }
            Action::Moor { island } => {
                // Clarify whether only one ship can be moored at a given island at a time
                self.moored_ships_status.insert(action.ship_id, Some((island, 0)));

                events_vec.push(Event {
                    message: format!("The ship {:?} is moored in the island {:?}", &action.ship_id, &island),
                })
            }
            Action::AskClue => {
                todo!("Wait for Island implementation")
            }
            Action::Bombard { target } => {
                todo!("Wait for Ship implementation")
            }
            Action::ProposeAlliance { to } => {
                todo!("Wait for Alliance implementation")
            }
            Action::AcceptAlliance { proposer } => {
                todo!("Wait for Ship trait implementation")
            }
            Action::BreakAlliance => {
                todo!("Wait for Ship implementation")
            }
            Action::Declare => {
                if let Some(ship_position) = self.ship_position_provvisorio.get(&action.ship_id) {
                    if *ship_position == self.treasure_cell {
                        self.is_eneded = true;
                        events_vec.push(Event {
                            message: format!("The ship {:?} found the treasure", action.ship_id),
                        });
                        todo!("Determine what happens once the treasure is found");
                    } else {
                        events_vec.push(Event {
                            message: format!("The ship {:?} did not find the treasure", action.ship_id),
                        });
                        todo!("Determine what happens if the treasure is not found")
                    }
                }
            }
            Action::Hold => events_vec.push(Event {
                message: format!("The ship {:?} holds", action.ship_id),
            }),
        }

        events_vec
    }

    /// Ends the current round: advances the round number and lets the islands take their turn.
    ///
    /// This function performs 3 steps in a fixed order:
    /// - Increments the round number so the wind updates to its new value.
    /// - Gives every island its turn (in descending order of identifier).
    /// - Determines whether the game has ended.
    ///
    /// # Arguments
    /// * `self` - The game chart.
    ///
    /// # Returns
    /// A `Vec<Event>` containing the log of events that occurred.
    pub fn advance_round(&mut self) -> Vec<Event> {
        self.round += 1;

        // Update wind direction
        self.direction = wind_at(self.wind_seed, self.round);

        // Give every island its turn
        for (_island, round) in self.moored_ships_status.values_mut().flatten() {
            *round += 1;
        }

        if self.is_eneded || self.round >= self.round_cap {
            todo!("Handle game over state");
        }

        // Verify if the game has ended
        todo!("Return the vec event after deciding the correct messages")
    }

    pub fn view_for(&self, ship: ShipId) -> ShipView {
        // TODO(@Davide)
        todo!()
    }

    pub fn spectator_view(&self) -> SpectatorView {
        // TODO(@SpidyAml)
        todo!()
    }

    pub fn outcome(&self) -> Option<Outcome> {
        // TODO(@Davide)
        todo!()
    }
}
