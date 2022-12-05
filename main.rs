use std::process::exit;
use bevy_ecs::prelude::*;
use bevy_ecs::world::EntityMut;
use macroquad::prelude::*;
use crate::types::*;
use crate::window_config::*;
mod collision;
mod tile;
mod types;
mod player;
mod window_config;

#[derive(StageLabel)]
pub struct UpdateLabel;

#[macroquad::main(window_config)]
async fn main() {
    let mut world = World::new();
    let mut schedule = 	Schedule::default();
    schedule.add_stage(UpdateLabel, SystemStage::parallel()
        .with_system_set(
            SystemSet::new()
                .label("update")
                .with_system(player::movement)
                .with_system(player::update)
        )
        .with_system_set(
            SystemSet::new()
                .label("draw")
                .with_system(player::draw)
                .with_system(tile::draw)
        ));

    let chara_template = player::PlayerBundle {
        flag: player::PlayerFlag,
        position: Position {x: get_screen_data().width as f32 /2.0-16.0, y: get_screen_data().height as f32/2.0-16.0},
        size: Size {w: 32.0, h: 32.0},
        ..Default::default()
    };

    world.spawn(chara_template.clone());
    world.spawn(tile::Tile{
        flag: tile::TileFlag,
        tile_type: tile::TileType::Default,
        position: Position{x: 300.0, y: 150.0},
        size: Size{w: 32.0, h: 32.0},
        visibility: Default::default(),
        hitbox: HitBox{
            x: 300.0,
            y: 150.0,
            w: 32.0,
            h: 32.0
        }
    });

    world.spawn(tile::Tile{
        flag: tile::TileFlag,
        tile_type: tile::TileType::Default,
        position: Position{x: 600.0, y: 200.0},
        size: Size{w: 32.0, h: 32.0},
        visibility: Default::default(),
        hitbox: HitBox{
            x: 600.0,
            y: 200.0,
            w: 32.0,
            h: 32.0
        }
    });

    loop {
        //clear_background(BLACK);
        schedule.run(&mut world);
        let fps = get_fps();
        let entity_count = world.entities().len();
        if is_key_down(KeyCode::Escape) {
            exit(0);
        }
        if is_key_pressed(KeyCode::Kp1) {
            world.spawn(chara_template.clone());
        }
        // draw_text(&*format!("Entities: {entity_count}"), 0.0, 80.0, 32.0, WHITE);
        // draw_text(&*format!("FPS: {fps}"), 0.0, 110.0, 32.0, WHITE);
        next_frame().await;
    }
}