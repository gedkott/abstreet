use piston::input::Key;
use plugins::{Plugin, PluginCtx};
use sim::TripID;

// TODO woops, all the plugins that work off of trips now don't work for buses. :(
#[derive(PartialEq)]
pub enum FollowState {
    Empty,
    Active(TripID),
}

impl FollowState {
    pub fn new() -> FollowState {
        FollowState::Empty
    }
}

impl Plugin for FollowState {
    fn event(&mut self, ctx: PluginCtx) -> bool {
        if *self == FollowState::Empty {
            if let Some(agent) = ctx.primary.current_selection.and_then(|id| id.agent_id()) {
                if let Some(trip) = ctx.primary.sim.agent_to_trip(agent) {
                    if ctx.input.key_pressed(Key::F, &format!("follow {}", agent)) {
                        *self = FollowState::Active(trip);
                        return true;
                    }
                }
            }
        }

        let mut quit = false;
        if let FollowState::Active(trip) = self {
            if let Some(pt) = ctx.primary.sim.get_stats().canonical_pt_per_trip.get(&trip) {
                ctx.canvas.center_on_map_pt(*pt);
            } else {
                // TODO ideally they wouldnt vanish for so long according to
                // get_canonical_point_for_trip
                warn!("{} is gone... temporarily or not?", trip);
            }
            quit = ctx.input.key_pressed(Key::Return, "stop following");
        };
        if quit {
            *self = FollowState::Empty;
        }
        match self {
            FollowState::Empty => false,
            _ => true,
        }
    }
}
