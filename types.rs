use bevy_ecs::prelude::*;
use macroquad::prelude::*;

#[derive(Component, Clone)]
pub struct Position {
	pub x: f32,
	pub y: f32
}
impl Default for Position {
	fn default() -> Self {
		Position {
			x: 0.0,
			y: 0.0
		}
	}
}


#[derive(Component, Clone)]
pub struct Velocity {
	pub x: f32,
	pub y: f32
}
impl Default for Velocity {
	fn default() -> Self {
		Velocity {
			x: 0.0,
			y: 0.0
		}
	}
}


#[derive(Component, Clone)]
pub struct Rotation {
	value: f32
}
impl Default for Rotation {
	fn default() -> Rotation {
		Rotation {
			value: 0.0
		}
	}
}

#[derive(Component, Clone)]
pub struct Size {
	pub w: f32,
	pub h: f32
}
impl Default for Size {
	fn default() -> Self {
		Size {
			w: 0.0,
			h: 0.0
		}
	}
}

#[derive(Component, Clone)]
pub struct Visibility {
	pub value: bool
}
impl Default for Visibility {
	fn default() -> Self {
		Visibility{
			value: true
		}
	}
}


#[derive(Component, Clone)]
pub struct Line {
	pub x1: f32,
	pub y1: f32,
	pub x2: f32,
	pub y2: f32
}
impl Default for Line {
	fn default() -> Self {
		Line {
			x1: 0.0,
			y1: 0.0,
			x2: 0.0,
			y2: 0.0
		}
	}
}


#[derive(Component, Clone)]
pub struct HitBox {
	pub x: f32,
	pub y: f32,
	pub w: f32,
	pub h: f32
}
impl Default for HitBox {
	fn default() -> Self {
		HitBox {
			x: 0.0,
			y: 0.0,
			w: 0.0,
			h: 0.0
		}
	}
}