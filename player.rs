use bevy_ecs::prelude::*;
use collision::*;
use macroquad::prelude::*;
use crate::*;
use crate::types::*;

#[derive(Component, Clone)]
pub struct PlayerFlag;

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
	pub flag: PlayerFlag,
	pub position: Position,
	pub velocity: Velocity,
	pub size: Size,
	pub visibility: Visibility,
	pub hitbox: HitBox
}

impl Default for PlayerBundle {
	fn default() -> Self {
		PlayerBundle {
			flag: PlayerFlag,
			position: Default::default(),
			velocity: Default::default(),
			size: Default::default(),
			visibility: Default::default(),
			hitbox: Default::default()
		}
	}
}

fn test() -> bool{
	println!("a");
	true
}

pub fn movement(mut data_query: Query<(&mut Position, &mut Velocity, &Size), With<PlayerFlag>>,
				mut hitbox_query: Query<&HitBox>) {
	for (mut pos, mut vel, size) in data_query.iter_mut() {
		let dt = get_frame_time() * 100.0;
		vel.x = 2.0;
		vel.y = 2.0;

		if is_key_down(KeyCode::LeftShift) {
			vel.x = 4.0;
			vel.y = 4.0;
		}
		vel.x *= dt;
		vel.y *= dt;
		let x = pos.x;
		let y = pos.y;
		if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)) && is_free(Line{x1: pos.x+size.w+vel.x, y1: pos.y, x2: pos.x+size.w+vel.x, y2: pos.y+size.h}, &hitbox_query) {
			pos.x += vel.x;
		}
		if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)) && is_free(Line{x1: pos.x-vel.x, y1: pos.y, x2:pos.x-vel.x, y2: pos.y+size.h}, &hitbox_query) {
			pos.x -= vel.x;
		}
		if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) && is_free(Line{x1: pos.x, y1: pos.y-vel.y, x2:pos.x+size.w, y2: pos.y-vel.y}, &hitbox_query) {
			pos.y -= vel.y;
		}
		if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)) && is_free(Line{x1: pos.x, y1: pos.y+size.h+vel.y, x2:pos.x+size.w, y2: pos.y+size.h+vel.y}, &hitbox_query) {
			pos.y += vel.y;
		}
	}
}

pub fn update(mut data_query: Query<(Entity, &mut Position, &mut HitBox, &Size), With<PlayerFlag>>,
			  mut commands: Commands) {
	for (chara, mut position, mut hitbox, size) in data_query.iter_mut() {
		if is_key_down(KeyCode::Kp0) {
			commands.entity(chara).despawn();
			break;
		}
		hitbox.x = position.x;
		hitbox.y = position.y;
		hitbox.w = size.w;
		hitbox.h = size.h;
	}
}

pub fn draw(data_query: Query<(&Visibility, &Position, &Size, &HitBox, &Velocity), With<PlayerFlag>>) {
	for (visibility, pos, size, hitbox, velocity) in data_query.iter() {
		if visibility.value {
			let x = pos.x;
			let y = pos.y;
			// draw_text(&*format!("X: {x:.0}"), 0.0, 20.0, 32.0, WHITE);
			// draw_text(&*format!("Y: {y:.0}"), 0.0, 50.0, 32.0, WHITE);
			// draw_text(&*x.to_string(), 0.0, 20.0, 32.0, WHITE);
			// draw_text(&*y.to_string(), 0.0, 50.0, 32.0, WHITE);
			// draw_text(&*format!("Vel: {vel_x} ; {vel_y}"), 0.0, 80.0, 32.0, WHITE);
			draw_rectangle(pos.x as i32 as f32, pos.y as i32 as f32, size.w, size.h, WHITE);
			// draw_rectangle_lines(hitbox.x, hitbox.y, size.w, size.h, 4.0, RED);
		}
	}
}
