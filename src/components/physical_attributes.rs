use bevy::math::Vec3;

pub struct MassRadius {
    pub mass: f32,
    pub radius: f32
}

pub enum Gravity {
    Movable(MassRadius),
    Immovable(MassRadius)
}

pub struct Thrust {
    pub thrust: f32,
    pub facing: f32
}

pub struct Velocity {
    pub velocity: Vec3
}
