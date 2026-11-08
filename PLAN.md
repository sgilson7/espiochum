# Plan

Stage 2. Approved in advance (BRIEF.md, Autonomy). Each step with its reason and the test that shows it holds.

Baseline (before any edit): Gate Crasher at e5e12de, copied without its history, passes `./scripts/check.sh` in Chromium, Firefox and WebKit (1 min 31 s).

| # | Decision | Reason | Test that shows it holds |
| --- | --- | --- | --- |
| 1 | Cut to mission 1, Caged Bird: `data/missions.json` keeps only it; the title, strings and README say Espiochum. | The brief's MVP is the first mission. | The browser check starts mission 1 and the core's mission tests still pass. |
| 2 | Raise fidelity in Style Lab's pipeline where the game shows it: hands close on what they hold (finger poses), locomotion clips loop over one stride in place, matched to Gate Crasher's stride rule (`STRIDE + STRIDE_GROW x speed`) so feet do not slide, and every game clip is authored against the core's own offsets (hold, carry, mule). | The clipping in the Style Lab clips came from hand-placed bodies and floating contacts. | Step 3's check. |
| 3 | A clip check: `scripts/check_clips.py` rebuilds each baked clip's bone capsules from the baked matrices and fails when a body passes through another body (paired clips, at the core's offsets), a limb through its own torso, or anything below the ground, past set depths in `data/clips.json`. A planted defect (`tests/clip_defect.py`) breaks a clip and must fail it. | The brief's "no insane clipping", made checkable. Pure Python, so CI runs it without Blender. | The check passes on the shipped clips and fails on the planted one, both in `check.sh`. |
| 4 | Bake the cast for the browser (`style-lab/tools/mesh/bake_game.py`): Big Chum, the soldier (the veteran), Corporal Grace and the Memphis Mule, each decimated, materials baked to textures, skin weights onto a reduced bone set, every clip sampled to bone matrices. Written to `web/mesh/` and `data/mesh/` with a manifest holding every offset. | The page may not parse formats with numbers in its own code (D3): the manifest carries them. | Manifest and file sizes checked by `check_clips.py`; the browser check draws every model. |
| 5 | The core hands people to the page as actor records (model, clip, frame, position, heading) in place of posed boxes, choosing the clip from the state it already keeps; clip frames come from `walk_phase` for gaits and from state timers otherwise. | Rules stay in the core (R1); the page only draws. | Core tests: each pose maps to the named clip; a full stride walks the clip once. |
| 6 | A WebGL 2 skinned pass: bone matrices from a float texture, the clip and frame as uniforms, the new style's light (low sun, sky fill, rim) and a sun shadow map for people and world. The world keeps Gate Crasher's collision boxes as its geometry, drawn with weathered procedural materials (concrete, corrugated metal, sand). | What is drawn is what collides, so nobody walks through a wall that looks solid. | Browser check: models load, no console error, a still of the yard compared by eye in the journal. |
| 7 | Menu pictures are stills rendered by the pipeline in place of the TikZ art. | The new style. | Files listed in LICENSES.md. |
| 8 | Deploy: a public repository `sgilson7/espiochum`; a push to `main` runs the check and publishes Pages. | Approved in the brief. | The live address loads and the browser check passes against it. |

## Open questions for the person

None blocking: the brief approves the plan. Choices made alone are in `SECOND-ORDER.md`.

## Status

- [x] Approved in advance (BRIEF.md, 2026-11-07).
