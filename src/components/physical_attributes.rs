use bevy::math::Vec3;

pub struct MassRadius {
    pub mass: f32,
    pub radius: f32
}

impl MassRadius {
    pub fn from_density(density: f32, radius: f32) -> MassRadius {
        let volume: f32 = (4.0/3.0) * 3.14 * (radius * radius * radius);
        MassRadius {
            radius,
            mass: density * volume
        }
    }
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
