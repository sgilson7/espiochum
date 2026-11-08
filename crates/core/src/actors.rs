//! People and the mule as actor records for the page's skinned pass.
//!
//! The page draws each character as a skinned mesh animated by baked clips
//! (`data/clips.json`, written by Style Lab's bake). The core chooses the
//! clip and the frame from the state it already keeps and hands the page one
//! record per actor: the model, the two animation rows to blend and the blend,
//! the model matrix, and whether a soldier's rifle is drawn. The page computes
//! nothing: no frame, no angle, no position (Builder D3).
//!
//! Gaits loop over one stride, so a gait's frame comes from the stride phase
//! (`walk_phase`: a full turn is one stride) and feet keep pace with the
//! ground. Everything else runs on the animation clock at the clip's own rate.

use crate::fx::{int, ONE, TURN};
use crate::geom::{Affine, V3};
use serde_json::Value;

pub const CHUM: usize = 0;
pub const SOLDIER: usize = 1;
pub const GRACE: usize = 2;
pub const MULE: usize = 3;
const MODELS: [&str; 4] = ["chum", "soldier", "grace", "mule"];

/// Values per record: model, row now, row next, blend, 12 matrix values, armed.
pub const VALUES: usize = 17;

/// Clips run at 30 frames a second; the core ticks 60 times a second.
pub const TICKS_PER_FRAME: i32 = 2;

#[derive(Debug, Clone)]
pub struct Clip {
    pub name: String,
    pub frames: i32,
    pub looped: bool,
    pub row: i32,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub models: Vec<Vec<Clip>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Actor {
    pub model: usize,
    pub clip: usize,
    /// frames into the clip, Q16 (a fraction blends to the next frame)
    pub frame: i32,
    pub at: V3,
    pub yaw: i32,
    pub armed: bool,
}

impl Table {
    /// The clip table compiled into the core.
    pub fn load() -> Table {
        Table::parse(include_str!("../../../data/clips.json")).expect("data/clips.json")
    }

    pub fn parse(text: &str) -> Result<Table, String> {
        let v: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let mut models = vec![Vec::new(); MODELS.len()];
        for m in v["models"].as_array().ok_or("no models")? {
            let name = m["model"].as_str().ok_or("model without a name")?;
            let k = MODELS
                .iter()
                .position(|x| *x == name)
                .ok_or(format!("unknown model {name}"))?;
            for c in m["clips"].as_array().ok_or("no clips")? {
                models[k].push(Clip {
                    name: c["name"].as_str().unwrap_or("").to_string(),
                    frames: c["frames"].as_i64().unwrap_or(1).max(1) as i32,
                    looped: c["loop"].as_bool().unwrap_or(false),
                    row: c["row"].as_i64().unwrap_or(0) as i32,
                });
            }
        }
        Ok(Table { models })
    }

    /// A clip's number by name; the first clip when the name is unknown.
    pub fn clip(&self, model: usize, name: &str) -> usize {
        self.models[model]
            .iter()
            .position(|c| c.name == name)
            .unwrap_or(0)
    }

    pub fn frames(&self, model: usize, clip: usize) -> i32 {
        self.models[model][clip].frames
    }

    /// An actor on a clip by name, at a frame (Q16).
    pub fn actor(&self, model: usize, name: &str, frame: i32, at: V3, yaw: i32, armed: bool) -> Actor {
        Actor {
            model,
            clip: self.clip(model, name),
            frame,
            at,
            yaw,
            armed,
        }
    }

    /// The frame (Q16) a gait clip shows at this stride phase.
    pub fn by_phase(&self, model: usize, name: &str, phase: i32) -> i32 {
        let n = self.frames(model, self.clip(model, name)) as i64;
        ((phase.rem_euclid(TURN) as i64 * n * ONE as i64) / TURN as i64) as i32
    }

    /// The frame (Q16) a clip on the animation clock shows after `ticks`.
    pub fn by_ticks(ticks: i32) -> i32 {
        ticks.max(0) * (ONE / TICKS_PER_FRAME)
    }

    /// Whether an actor at `at` is drawn from a camera at `eye` looking along
    /// unit `fwd`: within `far`, and not more than `behind` behind the eye.
    pub fn in_view(at: V3, eye: V3, fwd: V3, far: i32, behind: i32) -> bool {
        let d = at.sub(eye);
        d.len() <= far && d.dot(fwd) >= -behind
    }

    /// The record the page reads for one actor.
    pub fn record(&self, a: &Actor, out: &mut Vec<i32>) {
        let c = &self.models[a.model][a.clip];
        let whole = a.frame.max(0) / ONE;
        let blend = a.frame.max(0) % ONE;
        let (now, next) = if c.looped {
            (whole % c.frames, (whole + 1) % c.frames)
        } else {
            ((whole).min(c.frames - 1), (whole + 1).min(c.frames - 1))
        };
        out.push(int(a.model as i32));
        out.push(int(c.row + now));
        out.push(int(c.row + next));
        out.push(blend);
        let m = Affine::translate(a.at).then(&Affine::rot_y(a.yaw));
        for v in [m.c0, m.c1, m.c2, m.t] {
            out.extend_from_slice(&[v.x, v.y, v.z]);
        }
        out.push(if a.armed { ONE } else { 0 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::TURN;
    use crate::geom::v3;

    fn table() -> Table {
        Table::parse(
            r#"{"models":[{"model":"chum","clips":[{"name":"idle","frames":60,"loop":true,"row":0},
                {"name":"run","frames":22,"loop":true,"row":60},{"name":"dive","frames":20,"loop":false,"row":82}]}]}"#,
        )
        .unwrap()
    }

    #[test]
    fn a_full_stride_plays_the_gait_clip_once() {
        let t = table();
        assert_eq!(t.by_phase(CHUM, "run", 0), 0);
        // half a stride is half the clip: 11 of 22 frames
        assert_eq!(t.by_phase(CHUM, "run", TURN / 2), int(11));
        // a whole stride wraps to the start
        assert_eq!(t.by_phase(CHUM, "run", TURN), 0);
    }

    #[test]
    fn the_record_names_absolute_rows_and_wraps_a_loop() {
        let t = table();
        let mut out = Vec::new();
        // frame 21.5 of the 22-frame run: rows 60 + 21 and 60 + 0, half way
        t.record(
            &t.actor(CHUM, "run", int(21) + ONE / 2, v3(0, 0, 0), 0, false),
            &mut out,
        );
        assert_eq!(&out[..4], &[int(0), int(81), int(60), ONE / 2]);
        assert_eq!(out.len(), VALUES);
    }

    #[test]
    fn a_clip_that_does_not_loop_holds_its_last_frame() {
        let t = table();
        let mut out = Vec::new();
        t.record(&t.actor(CHUM, "dive", int(40), v3(0, 0, 0), 0, false), &mut out);
        // 20 frames from row 82: the last is 101, and it stays there
        assert_eq!(&out[1..3], &[int(101), int(101)]);
    }

    #[test]
    fn the_matrix_places_and_turns_the_actor() {
        let t = table();
        let mut out = Vec::new();
        t.record(
            &t.actor(CHUM, "idle", 0, v3(int(3), int(1), int(-2)), 0, true),
            &mut out,
        );
        // no turn: the identity's columns, then the position
        assert_eq!(
            &out[4..16],
            &[ONE, 0, 0, 0, ONE, 0, 0, 0, ONE, int(3), int(1), int(-2)]
        );
        assert_eq!(out[16], ONE, "armed");
    }

    #[test]
    fn only_actors_near_and_ahead_of_the_camera_are_drawn() {
        let (eye, fwd) = (v3(0, 0, 0), v3(0, 0, -ONE));
        assert!(
            Table::in_view(v3(0, 0, int(-30)), eye, fwd, int(70), int(3)),
            "30 m ahead"
        );
        assert!(
            !Table::in_view(v3(0, 0, int(-80)), eye, fwd, int(70), int(3)),
            "80 m ahead is too far"
        );
        assert!(
            Table::in_view(v3(0, 0, int(2)), eye, fwd, int(70), int(3)),
            "2 m behind the eye still shows"
        );
        assert!(
            !Table::in_view(v3(0, 0, int(10)), eye, fwd, int(70), int(3)),
            "10 m behind is not drawn"
        );
    }

    #[test]
    fn the_shipped_table_has_every_clip_the_game_asks_for() {
        let t = Table::load();
        for (m, names) in [
            (
                CHUM,
                &[
                    "idle",
                    "run",
                    "sprint",
                    "crouch_walk",
                    "crouch_idle",
                    "prone_idle",
                    "crawl",
                    "carry_walk",
                    "carry_idle",
                    "hold",
                    "hang",
                    "ride",
                    "dive",
                    "aim",
                    "crouch_aim",
                ][..],
            ),
            (
                SOLDIER,
                &[
                    "idle",
                    "walk",
                    "run",
                    "aim",
                    "held",
                    "handsup",
                    "crouch_idle",
                    "lying",
                    "bound",
                    "draped",
                    "on_mule",
                    "kick",
                ][..],
            ),
            (GRACE, &["caged", "lying", "draped", "on_mule"][..]),
            (MULE, &["idle", "walk", "trot", "charge"][..]),
        ] {
            for n in names {
                assert!(
                    t.models[m].iter().any(|c| c.name == *n),
                    "{} has no clip {n}",
                    MODELS[m]
                );
            }
        }
    }
}
