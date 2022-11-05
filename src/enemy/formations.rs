use bevy::prelude::Component;

/// Component - Enemy Formation (per enemy)
#[derive(Component)]
pub struct Formation {
    pub start: (f32, f32),
    pub radius: (f32, f32),
    pub pivot: (f32, f32),
    pub speed: f32,
    pub angle: f32,
}

/// Resource - Formation Maker

pub struct FormationMaker {
    current_template: Option<Formation>,
    current_members: u32,
}

/// Formation factory implemention