use macroquad::{
    color::{BLACK, WHITE},
    prelude::Color,
};

// Renomeado para ficar mais genérico, mas pode manter ball_colors se preferir
pub mod game_colors {
    use super::*; // Importa Color, BLACK, WHITE do escopo pai (colors.rs)

    pub const CUE: Color = WHITE;
    pub const SOLID_YELLOW: Color = Color::new(0.98, 0.80, 0.08, 1.0); // #facc15
    pub const SOLID_BLUE: Color = Color::new(0.23, 0.51, 0.96, 1.0); // #3b82f6
    pub const SOLID_RED: Color = Color::new(0.94, 0.27, 0.27, 1.0); // #ef4444
    pub const SOLID_PURPLE: Color = Color::new(0.66, 0.33, 0.97, 1.0); // #a855f7
    pub const SOLID_ORANGE: Color = Color::new(0.98, 0.45, 0.09, 1.0); // #f97316
    pub const SOLID_GREEN: Color = Color::new(0.13, 0.77, 0.35, 1.0); // #22c55e
    pub const SOLID_MAROON: Color = Color::new(0.53, 0.07, 0.21, 1.0); // #881337
    pub const EIGHT_BALL: Color = Color::new(0.09, 0.09, 0.09, 1.0); // #171717
    pub const STRIPE_PRIMARY: Color = WHITE;
    pub const POCKET_CENTER: Color = Color::new(0.06, 0.06, 0.06, 1.0);
    pub const POCKET_BORDER: Color = BLACK;
    pub const TABLE_BG: Color = Color::new(0.02, 0.59, 0.41, 1.0); // #059669
    pub const TABLE_BORDER_COLOR: Color = Color::new(0.47, 0.21, 0.06, 1.0); // #78350f
    pub const CUE_TIP: Color = Color::new(0.38, 0.65, 0.98, 1.0); // #60a5fa
    pub const AIM_LINE_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.4);
    pub const REPOSITION_AREA_FILL: Color = Color::new(1.0, 1.0, 1.0, 0.1);
    pub const REPOSITION_AREA_STROKE: Color = Color::new(1.0, 1.0, 1.0, 0.5);
    pub const BUTTON_BG: Color = Color::new(0.23, 0.51, 0.96, 1.0); // blue-500
    pub const BUTTON_TEXT: Color = WHITE;
    pub const BUTTON_HOVER_BG: Color = Color::new(0.14, 0.39, 0.92, 1.0); // blue-600
    pub const UI_BG_COLOR: Color = Color::new(0.067, 0.094, 0.153, 0.95); // Tailwind gray-900 (#111827) com alpha
    pub const HUD_TEXT_COLOR: Color = WHITE;
    pub const HUD_POCKETED_BALL_BG: Color = Color::new(0.2, 0.2, 0.2, 0.5);
}

