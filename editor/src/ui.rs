use crate::colors::ColorScheme;
use abstutil;
//use cpuprofiler;
use crate::objects::{Ctx, RenderingHints, ID, ROOT_MENU};
use crate::render::RenderOptions;
use crate::state::UIState;
use ezgui::{Canvas, Color, EventLoopMode, GfxCtx, Text, UserInput, BOTTOM_LEFT, GUI};
use kml;
use map_model::{BuildingID, LaneID};
use piston::input::Key;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::process;

const MIN_ZOOM_FOR_MOUSEOVER: f64 = 4.0;

pub struct UI<S: UIState> {
    state: S,
    canvas: Canvas,
    cs: ColorScheme,
}

impl<S: UIState> GUI<RenderingHints> for UI<S> {
    fn event(&mut self, mut input: UserInput) -> (EventLoopMode, RenderingHints) {
        let mut hints = RenderingHints {
            mode: EventLoopMode::InputOnly,
            osd: Text::new(),
            suppress_intersection_icon: None,
            color_crosswalks: HashMap::new(),
            hide_crosswalks: HashSet::new(),
            hide_turn_icons: HashSet::new(),
        };

        // First update the camera and handle zoom
        let old_zoom = self.canvas.cam_zoom;
        self.canvas.handle_event(&mut input);
        let new_zoom = self.canvas.cam_zoom;
        self.state.handle_zoom(old_zoom, new_zoom);

        // Always handle mouseover
        if old_zoom >= MIN_ZOOM_FOR_MOUSEOVER && new_zoom < MIN_ZOOM_FOR_MOUSEOVER {
            self.state.set_current_selection(None);
        }
        if !self.canvas.is_dragging()
            && input.get_moved_mouse().is_some()
            && new_zoom >= MIN_ZOOM_FOR_MOUSEOVER
        {
            self.state.set_current_selection(self.mouseover_something());
        }

        let mut recalculate_current_selection = false;
        self.state.event(
            &mut input,
            &mut hints,
            &mut recalculate_current_selection,
            &mut self.cs,
            &mut self.canvas,
        );
        if recalculate_current_selection {
            self.state.set_current_selection(self.mouseover_something());
        }

        // Can do this at any time.
        if input.unimportant_key_pressed(Key::Escape, ROOT_MENU, "quit") {
            self.save_editor_state();
            self.cs.save();
            info!("Saved color_scheme");
            //cpuprofiler::PROFILER.lock().unwrap().stop().unwrap();
            process::exit(0);
        }

        input.populate_osd(&mut hints.osd);

        (hints.mode, hints)
    }

    fn get_mut_canvas(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    fn draw(&self, g: &mut GfxCtx, hints: RenderingHints) {
        g.clear(self.cs.get_def("map background", Color::rgb(242, 239, 233)));

        let ctx = Ctx {
            cs: &self.cs,
            map: &self.state.primary().map,
            draw_map: &self.state.primary().draw_map,
            canvas: &self.canvas,
            sim: &self.state.primary().sim,
            hints: &hints,
        };

        let (statics, dynamics) = self.state.get_objects_onscreen(&self.canvas);
        for obj in statics
            .into_iter()
            .chain(dynamics.iter().map(|obj| Box::new(obj.borrow())))
        {
            let opts = RenderOptions {
                color: self.color_obj(obj.get_id(), &ctx),
                cam_zoom: self.canvas.cam_zoom,
                debug_mode: self.state.is_debug_mode_enabled(),
            };
            obj.draw(g, opts, &ctx);
        }

        self.state.draw(g, &ctx);

        self.canvas.draw_text(g, hints.osd, BOTTOM_LEFT);
    }

    fn dump_before_abort(&self) {
        self.state.dump_before_abort();
        self.save_editor_state();
    }
}

impl<S: UIState> UI<S> {
    pub fn new(state: S, canvas: Canvas) -> UI<S> {
        let mut ui = UI {
            state,
            canvas,
            cs: ColorScheme::load().unwrap(),
        };

        match abstutil::read_json::<EditorState>("editor_state") {
            Ok(ref state) if ui.state.primary().map.get_name() == &state.map_name => {
                info!("Loaded previous editor_state");
                ui.canvas.cam_x = state.cam_x;
                ui.canvas.cam_y = state.cam_y;
                ui.canvas.cam_zoom = state.cam_zoom;
            }
            _ => {
                warn!("Couldn't load editor_state or it's for a different map, so just focusing on an arbitrary building");
                // TODO window_size isn't set yet, so this actually kinda breaks
                let focus_pt = ID::Building(BuildingID(0))
                    .canonical_point(
                        &ui.state.primary().map,
                        &ui.state.primary().sim,
                        &ui.state.primary().draw_map,
                    )
                    .or_else(|| {
                        ID::Lane(LaneID(0)).canonical_point(
                            &ui.state.primary().map,
                            &ui.state.primary().sim,
                            &ui.state.primary().draw_map,
                        )
                    })
                    .expect("Can't get canonical_point of BuildingID(0) or Road(0)");
                ui.canvas.center_on_map_pt(focus_pt);
            }
        }

        ui
    }

    fn mouseover_something(&self) -> Option<ID> {
        let pt = self.canvas.get_cursor_in_map_space();

        let (statics, dynamics) = self.state.get_objects_onscreen(&self.canvas);
        // Check front-to-back
        for obj in dynamics
            .iter()
            .map(|obj| Box::new(obj.borrow()))
            .chain(statics.into_iter().rev())
        {
            if obj.contains_pt(pt) {
                return Some(obj.get_id());
            }
        }

        None
    }

    fn color_obj(&self, id: ID, ctx: &Ctx) -> Option<Color> {
        self.state.color_obj(id, ctx)
    }

    fn save_editor_state(&self) {
        let state = EditorState {
            map_name: self.state.primary().map.get_name().clone(),
            cam_x: self.canvas.cam_x,
            cam_y: self.canvas.cam_y,
            cam_zoom: self.canvas.cam_zoom,
        };
        // TODO maybe make state line up with the map, so loading from a new map doesn't break
        abstutil::write_json("editor_state", &state).expect("Saving editor_state failed");
        info!("Saved editor_state");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditorState {
    pub map_name: String,
    pub cam_x: f64,
    pub cam_y: f64,
    pub cam_zoom: f64,
}
