csx/                                # Projektrot
├─ Cargo.toml                       # Workspace-definition
├─ crates/                          # Alla återanvändbara "subsystem"
│  ├─ core/                         # Spelregler (rounds, buy, hitreg)
│  │  ├─ src/lib.rs
│  │  ├─ src/player.rs
│  │  ├─ src/weapon.rs
│  │  ├─ src/round.rs
│  │  └─ src/combat.rs
│  ├─ net/                          # Nätverkslager (snapshot/command)
│  │  ├─ src/lib.rs
│  │  ├─ src/protocol.rs
│  │  └─ src/lag_comp.rs
│  ├─ physics/                      # Fysik & rörelse
│  │  ├─ src/lib.rs
│  │  └─ src/character.rs
│  ├─ map/                          # Kartformat, spawnpoints, bombsites
│  │  ├─ src/lib.rs
│  │  ├─ src/loader.rs
│  │  └─ src/navmesh.rs
│  ├─ ui/                           # HUD, scoreboard, buy-meny
│  │  ├─ src/lib.rs
│  │  └─ src/hud.rs
│  ├─ audio/                        # Steg, vapenljud, röstkommandon
│  │  └─ src/lib.rs
│  └─ shared/                       # Gemensamma typer & konstanter
│     ├─ src/lib.rs
│     ├─ src/messages.rs
│     ├─ src/constants.rs
│     └─ src/types.rs
├─ apps/                            # Körbara program
│  ├─ client/
│  │  ├─ src/main.rs
│  │  ├─ src/input.rs
│  │  └─ src/render.rs
│  └─ server/
│     ├─ src/main.rs
│     └─ src/game_loop.rs
├─ tools/                           # Utvecklarverktyg
│  ├─ mapc/                         # Kartkompilator (light + navmesh)
│  │  ├─ src/main.rs
│  │  └─ Cargo.toml
│  └─ xtask/                        # `cargo xtask build-assets` etc.
│     └─ src/main.rs
├─ assets_raw/                      # Råmaterial (GLTF, WAV, PNG…)
│  ├─ maps/
│  ├─ fonts/
│  ├─ models/
│  ├─ textures/
│  └─ sounds/
├─ assets_packed/                   # Färdigpaketerade assets
└─ docs/                            # Design & teknikdokument
