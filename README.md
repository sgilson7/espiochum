# Espiochum

A third-person stealth game that runs in the browser: the first mission of Gate Crasher, Caged Bird, rebuilt in a realistic style. You are Big Chum, a legendary soldier, with a silenced pistol, a portal gun and the Memphis Mule. Somewhere in a walled island outpost the soldiers hold Corporal Grace, and nobody will say where until you make someone talk. Free him and get out to the landing zone. Soldiers see and hear you the way they do in Metal Gear Solid V, and one bullet kills you.

**Play:** https://sgilson7.github.io/espiochum/

## What you can do

- **Move as in MGS V.** Stand, crouch (C), crawl (Z), sprint (Shift). Space vaults onto a ledge up to 1.7 m; on the move it vaults clean over anything low and thin (up to 1.2 m high and 1.5 m deep). A ledge up to 2.6 m high she leaps at and hangs from, Uncharted style: A and D shimmy along it, W or Space climbs up, C lets go. Steps up to 0.45 m are walked up. Space out of a sprint is a dive: a leap through the air that lands you prone, and that bowls over a soldier in the way for four seconds. Footsteps are heard as far as the gait carries them: sprinting about 16 m, crouch-walking about 1.5 m, crawling not at all. Floors indoors carry further, grass and sand less.
- **Hide.** In bushes and tall grass, in lockers (E), and in a cardboard box (B) that soldiers ignore while it keeps still.
- **Camouflage.** A camo index in percent, as in MGS3: the uniform against the ground underfoot (four uniforms, each strong on some grounds and weak on others; change in the field with U), plus stance, movement, cover and shade. The index scales how far a soldier can see you. Active camo (V) runs on a battery.
- **Portals.** Q and R open a blue and an orange portal on a large wall, roof or floor in sight (crates, lockers and railings are too small). Walk into one and you come out of the other, turned and with your speed carried through. With both open, each one shows the view out of the other, and a shot into one comes out of the other. Ordinary soldiers cannot see them. Elite soldiers wear lenses that can, and walk over to shut any portal they spot.
- **The pistol.** You start with 8 tranquilliser rounds (more with R&D staff), so spend them carefully. Tranquilliser rounds are silent; a soldier hit in the body stays on his feet for a moment, a hit in the head drops him at once. He sleeps five minutes, then wakes and radios it in. Soldiers near him who see him fall go and wake him. Lethal rounds (T) are loud and leave a body that does not get up.
- **CQC.** Crouch-walking up behind a soldier is silent. F behind him grabs him; then E interrogates (each soldier answers in his own words, from `data/talk.json`), F chokes him out, X kills, C throws him down. F from the front takes his rifle and throws him; aim at him on the ground and E interrogates him.
- **Hold-ups.** Aim at a soldier close up who has noticed you, or whose back is to you, and he drops his rifle and puts his hands up. E interrogates him and F grabs him. Take the gun off him for long and he picks the rifle up and radios in.
- **Bodies.** Carry them (E) and leave them in a bush or a locker. A soldier who finds a body raises the caution phase; one who finds a sleeper wakes him.
- **The Memphis Mule.** He follows you and soldiers never notice him. H whistles him over. M sends him to ram the soldier you point at. Y ties a soldier to his back (one lying near you, on your shoulder or in your hold), or puts Corporal Grace on; if the mule is far, Y calls him over; E interrogates a soldier tied there, who almost always talks and tells more. J rides him: fast, and seen and heard from far off. Soldiers waking a tranquillised buddy kick him awake. The mule also marks soldiers he smells nearby.
- **Fulton.** G sends a downed soldier up on a balloon, outdoors, to your base, Graceland. The soldiers on this outpost hate serving it; each one you extract radios his thanks on landing and signs on with Big Chum. Recruits are kept between visits, and each unit's level improves your kit: R&D adds tranquilliser rounds, Support adds active camo time, Intel lengthens the mule's nose.
- **Score.** Intel, recruits, finishing and finishing fast earn points; each alert costs 1,000 and each soldier killed costs 2,500 (Colonel Varga, the mission 2 target, excepted). The score shows on the HUD and the results screen.
- **Detection.** A soldier's sight is drawn on the ground. White ticks round the screen centre show who is noticing you. At full awareness the game drops into reflex mode, two and a half seconds of slow motion to put him down; miss it and the base goes to alert, then evasion, then caution. Nobody fires for a second and a half after the alarm. In an alert each soldier hunts where he last knew you to be: the alarm tells everyone where you were spotted, a soldier who sees you follows you, and the others are told again only when someone sights you after you have been out of sight for a second. Break line of sight and move, and they search the wrong place.

Press I for the intel you have, Esc to pause, N for sound.

## How it is built

Espiochum is a [Builder](https://github.com/sgilson7/builder-setting) project. Its rules are Gate Crasher's, ported at commit e5e12de (MIT, the same author): a Rust core in Q16.16 fixed point with no floats, clocks or input and output (`crates/core`), a WebAssembly shim that decides nothing (`crates/shim`), and a static WebGL 2 page that draws what the core returns (`web/`).

What is new in Espiochum:

- **The cast** (Big Chum, the soldiers, Corporal Grace, the Memphis Mule) are skinned meshes built and animated by script in Blender, from CC0 MakeHuman assets, by the Style Lab pipeline (`style-lab/tools/mesh/`: `build.py`, `skin.py`, `gear.py`, `mule.py`, `game_clips.py`, `bake_game.py`). The bake writes each model to `web/mesh/` with a manifest in `data/mesh/`, and every clip as bone matrices the shader reads from a texture.
- **The core chooses the animation** (`crates/core/src/actors.rs`): one record per person with the clip, the frame and the model matrix. Gaits loop over exactly one stride, so the core picks the frame from its stride counter and feet keep pace with the ground.
- **Nothing clips** (`scripts/check_clips.py`): every frame of every clip is rebuilt as bone capsules from the baked files and measured for bodies through bodies (a held soldier, a body on a shoulder or on the mule), limbs through the torso, and anything below the ground. `tests/clip_defect.py` plants two defects and requires the check to fail on both. Both run in `./scripts/check.sh`.
- **The world's look**: Gate Crasher's collision boxes, drawn with weathered materials (stained concrete, rusting corrugated metal, timber, patchy ground) under a low sun, a sky fill and haze (`data/shaders/scene.frag`), so what is drawn is what collides.
- **One mission on offer**: the other three stay in the rules and their tests, off the menu (`offered` in `data/missions.json`).

```bash
./scripts/setup.sh   # once: Playwright and three browser engines
./scripts/check.sh   # fmt, clippy, the core tests, the wasm build and audit, the clip check, the browser check
python3 scripts/serve.py 8000
```

A push to `main` runs the same check in Chromium and deploys `dist/` to GitHub Pages if it passes.
