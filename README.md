# ВИДЕОИГРА: АСТЕРОИДЫ

That's right.

![Gameplay](gameplay.gif)

A small arcade space shooter built over weekends to refresh some Rust knowledge with Macroquad.

## Features

- Endless asteroid survival gameplay with score-based progression.
- Player ship with thrust, braking, strafing, boost, and shooting.
- Breakable asteroids that split into smaller pieces.
- Two enemy types:
  - `AlanEnemy`: wanders, scans the arena with raycasts, and fires at targets it detects.
  - `BonBonEnemy`: kamikaze unit that dives toward the player and explodes on impact.
- Player lives system with hit feedback, sparkle VFX, screen shake, and death explosion.
- Enemy explosions, animated projectiles, animated backgrounds, and HUD stats.
- Best score tracking between runs.

## Controls

- `W`: thrust forward
- `S`: brake / slow down
- `A`: rotate left
- `D`: rotate right
- `Q`: strafe left
- `E`: strafe right
- `Left Shift`: boost
- `Space`: shoot, start game, restart after game over
- `Esc`: quit

Note: while the player death explosion is playing, gameplay input is frozen. `Esc` still works.

## Running The Game

```bash
cargo run
```
