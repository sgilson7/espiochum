# Brief

Stage 1, written by the person. `C1 Frame the problem and set the rules`, `B1 Establish context`.

## The request, word for word (2026-11-07)

> raise the fidelity first, then rebuild the first mission of gate crasher. make sure the animations do not have the insane clipping issues that the current ones do when they occur in game. now work using builder as a constraint until the mvp is deployed, in a github with a github pages, i approve any make deploy, serve me a link to play Espiochum

The request before it, which set the goal (2026-11-07, Style Lab T3):

> ok, now get a sense of the builder made game gate-crasher. I want you to-recreate the assets for gate-crasher in this new realistic style, with animations to match. ... the goal will be to recreate gatecrasher entirely from scratch using the new animation pipeline and style.

## The tool

**Espiochum**: Gate Crasher's first mission, Caged Bird, rebuilt in the new realistic style. A single-player, third-person stealth game in a desktop browser (keyboard and mouse). Big Chum infiltrates the walled outpost with a silenced pistol and the Memphis Mule, finds out by interrogation where Corporal Grace is held, frees him, and escapes to the landing zone. The rules come from Gate Crasher (ported at commit e5e12de, MIT, the same author); the characters, their animation and the world's look come from Style Lab's mesh pipeline (`../style-lab/tools/mesh/`).

## Done means

- The page loads from `https://sgilson7.github.io/espiochum/` with no off-origin request, and the mission can be played from start to escape.
- Big Chum, the soldiers, Corporal Grace and the Memphis Mule are skinned meshes from the new pipeline, animated by baked clips the core chooses: standing, walking, running, crouching, crawling, aiming, the tranquilliser hit and sleep, CQC hold and choke, hands up, carried, following.
- No clip clips: a check measures every game clip, frame by frame, for one body passing through another, limbs through the torso, and anything below the ground, and fails the build past a set depth. The check has a planted defect that it catches.
- The world is Gate Crasher's outpost, with the new style's weathered materials, sun and shadow; what is drawn is what collides.
- Gate Crasher's stealth rules for mission 1: sight and hearing, alert phases, one hit kills, tranquilliser rounds, CQC with interrogation, the mule, the escape.
- `./scripts/check.sh` passes before every deploy.

## Decisions that stay with the person

- The name (given: Espiochum).
- Every number that tunes the feel.
- Whether the remaining two missions and the portals come back in later rounds.

## Autonomy

Sam approved in advance (2026-11-07): the plan, a public GitHub repository `sgilson7/espiochum`, GitHub Pages deploys ("i approve any make deploy"), and asked the agent to keep working until the MVP is deployed. Where the brief is silent, the agent decides, records the decision in `SECOND-ORDER.md`, and continues. The agent may not loosen a Builder requirement or a boundary test, and may not add a server, an account, analytics or any off-origin request.

- Deploys: when the checks pass
- Subagents: none

## Journal

`default`

## References

- **Gate Crasher** (../refs/gatecrasher.md): every rule of mission 1, the outpost, the HUD, the sounds
- **Style Lab** (../style-lab, its journal ../style-lab-journal): the mesh pipeline, Big Chum, the Memphis Mule, the veteran as the soldier, the new style

## Allowed core dependencies

`serde`, `serde_json` (reference default). Nothing else.
