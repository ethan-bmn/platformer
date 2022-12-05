use macroquad::prelude::*;

pub fn window_config() -> Conf {
    Conf {
        window_title: "platformer.rs".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: false,
        fullscreen: false,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default()
    }
}
