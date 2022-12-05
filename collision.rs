use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use crate::player::PlayerFlag;
use crate::types::*;

pub fn is_free(line: Line, data_query: &Query<&HitBox>) -> bool {
	// draw_line(line.x1, line.y1, line.x2, line.y2, 4.0, GREEN);
	for hitbox in data_query {
		if (line.x1 < (hitbox.x + hitbox.w) && (line.x2) > hitbox.x) &&
			(line.y1 < (hitbox.y + hitbox.h) && (line.y2) > hitbox.y) {
			return false;
		}
	}
	return true;
}