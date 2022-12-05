use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use crate::*;
use crate::types::*;

#[derive(Component, Clone)]
pub enum TileType {
	Default
}

#[derive(Component, Clone)]
pub struct TileFlag;

#[derive(Bundle, Clone)]
pub struct Tile {
	pub flag: TileFlag,
	pub tile_type: TileType,
	pub position: Position,
	pub size: Size,
	pub visibility: Visibility,
	pub hitbox: HitBox
}

impl Default for Tile {
	fn default() -> Self {
		Tile {
			flag: TileFlag,
			tile_type: TileType::Default,
			position: Default::default(),
			size: Default::default(),
			visibility: Default::default(),
			hitbox: Default::default()
		}
	}
}

pub fn draw(data_query: Query<(&Visibility, &TileType, &Position, &Size), With<TileFlag>>) {
	for (visibility, tile_type, position, size) in data_query.iter() {
		draw_rectangle(position.x, position.y, size.w, size.h, BLUE);
	}
}