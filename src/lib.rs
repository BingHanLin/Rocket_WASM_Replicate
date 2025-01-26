use std::sync::Mutex;

use lazy_static::*;
use pcg_rand::seeds::PcgSeeder;
use pcg_rand::Pcg32Basic;
use rand::SeedableRng;
use wasm_bindgen::prelude::*;

mod controllers;
mod geometry;
mod models;

use controllers::{Actions, CollisionsController, GameState, TimeController};
use geometry::Rect;

lazy_static! {
    static ref DATA: Mutex<GameData> = Mutex::new(new_game_data(1024.0, 600.0));
}

struct GameData {
    state: GameState,
    actions: Actions,
    time_controller: TimeController<Pcg32Basic>,
}

fn new_game_data(width: f64, height: f64) -> GameData {
    GameData {
        state: GameState::new(Rect::new(width, height)),
        actions: Actions::default(),
        time_controller: TimeController::new(Pcg32Basic::from_seed(PcgSeeder::seed(42))),
    }
}

#[wasm_bindgen(raw_module = "../index.js")]
extern "C" {
    fn clear_screen();
    fn draw_player(_: f64, _: f64, _: f64);
    fn draw_enemy(_: f64, _: f64);
    fn draw_bullet(_: f64, _: f64);
    fn draw_particle(_: f64, _: f64, _: f64);
    fn draw_score(_: f64);
}

#[wasm_bindgen]
pub fn resize(width: f64, height: f64) {
    *DATA.lock().unwrap() = new_game_data(width, height);
}

#[wasm_bindgen]
pub fn draw() {
    use models::{Movable, Position};
    let data = &mut DATA.lock().unwrap();
    let world = &data.state.world;

    clear_screen();
    for particle in &world.particles {
        draw_particle(particle.x(), particle.y(), 5.0 * particle.time_out);
    }

    for bullet in &world.bullets {
        draw_bullet(bullet.x(), bullet.y());
    }

    for enemy in &world.enemies {
        draw_enemy(enemy.x(), enemy.y());
    }

    draw_player(world.player.x(), world.player.y(), world.player.direction());
    draw_score(data.state.score as f64);
}

#[wasm_bindgen]
pub fn update(time: f64) {
    let data: &mut GameData = &mut DATA.lock().unwrap();
    data.time_controller
        .update_seconds(time, &data.actions, &mut data.state);
    CollisionsController::handle_collisions(&mut data.state);
}

#[wasm_bindgen]
pub fn toggle_shoot(b: bool) {
    let data = &mut DATA.lock().unwrap();
    data.actions.shoot = b;
}

#[wasm_bindgen]
pub fn toggle_boost(b: bool) {
    let data = &mut DATA.lock().unwrap();
    data.actions.boost = b;
}

#[wasm_bindgen]
pub fn toggle_turn_left(b: bool) {
    let data: &mut std::sync::MutexGuard<'_, GameData> = &mut DATA.lock().unwrap();
    data.actions.rotate_left = b;
}

#[wasm_bindgen]
pub fn toggle_turn_right(b: bool) {
    let data = &mut DATA.lock().unwrap();
    data.actions.rotate_right = b;
}
