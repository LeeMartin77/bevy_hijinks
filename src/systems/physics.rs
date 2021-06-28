use bevy::prelude::*;
use crate::components::entities;
use crate::components::physical_attributes as phys;

//https://en.wikipedia.org/wiki/Gravitational_constant
pub const GRAVITATIONAL_CONSTANT: f32 = 0.00000000006674f32; //* (10f32).powi(-11);

const STARTUP_DELAY: f64 = 2.5;

pub fn gravity_system(
    time: Res<Time>,
    mut set: QuerySet<(
        Query<(&phys::Gravity, &Transform, &entities::Planet)>,
        Query<(&phys::Gravity, &Transform, &mut phys::Velocity)>
    )>
) {

    if time.seconds_since_startup() < STARTUP_DELAY {
        return;
    }


    //I'm sure there is a tider/rustier way of doing this, but we're gaining some pre-munging of data anyway here.
    let mut planets = Vec::new();
    for (planet_gravity, planet_transform, _planet) in set.q0().iter() {
        if let phys::Gravity::Immovable(pmr) = planet_gravity {
            planets.push(
                (phys::MassRadius{ mass: pmr.mass, radius: pmr.radius } , 
                    Vec3::new(planet_transform.translation.x, planet_transform.translation.y, 0.0) ));
        }
    }

    for (object_gravity, object_transform, mut velocity) in set.q1_mut().iter_mut() {
        let mut planet_mass_radius = phys::MassRadius { mass: 0.0, radius: 0.0 };
        let mut planet_translation = Vec3::new(0.0, 0.0, 0.0);
        let mut greatest_force_on_player_point = 0.0;
        for (pmr, planet_position) in &planets {
            let planet_force_on_player_point = planet_force_on_point(object_transform.translation, *planet_position, pmr.mass);
            if planet_force_on_player_point > greatest_force_on_player_point
            {
                greatest_force_on_player_point = planet_force_on_player_point;
                planet_mass_radius.mass = pmr.mass;
                planet_mass_radius.radius = pmr.radius;
                planet_translation.x = planet_position.x;
                planet_translation.y = planet_position.y;
            }
        }

        if let phys::Gravity::Movable(object_mass_radius) = object_gravity {
            let distance_between_objects = distance_between_two_vec(planet_translation, object_transform.translation);
            if distance_between_objects <= (planet_mass_radius.radius + object_mass_radius.radius) {
                // object has crashed - it needs to stop moving
                velocity.velocity = Vec3::new(0.0, 0.0, 0.0);
                velocity.crashed = true;
                continue;
            }
            let force = GRAVITATIONAL_CONSTANT * ((object_mass_radius.mass * planet_mass_radius.mass) / distance_between_objects.powi(2));
            //F=MA
            //A = F/M
            let acceleration = force / object_mass_radius.mass;
            //A = distance/time
            //distance = A*time
            let distance = acceleration * time.delta_seconds();
            let normalised_vector = vec_from_angle(angle_between_two_vec(planet_translation, object_transform.translation));
            velocity.velocity += normalised_vector * distance;
        }
    }
}

pub fn thrust_system(
    time: Res<Time>,
    mut query: Query<(&phys::Thrust, &mut phys::Velocity)>
) {

    if time.seconds_since_startup() < STARTUP_DELAY {
        return;
    }
    for (thrust, mut velocity) in query.iter_mut() {
        if thrust.thrust > 0.0 {
            velocity.velocity += time.delta_seconds() * (thrust.thrust * vec_from_angle(thrust.facing));
        }
    }
}

pub fn velocity_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &phys::Velocity)>
) {

    if time.seconds_since_startup() < STARTUP_DELAY {
        return;
    }
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += time.delta_seconds() * velocity.velocity;
    }
}

//Note: this all uses Vec3s, we only want to care about x and y though

fn vec_from_angle(angle: f32) -> Vec3 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vec3::new(vx, vy, 0.0)
}


fn angle_between_two_vec(sourcevec: Vec3, targetvec: Vec3) -> f32 {
    let vec = sourcevec - targetvec;
    vec.x.atan2(vec.y)
}

fn distance_between_two_vec(vecone: Vec3, vectwo: Vec3) -> f32{
    let vec = vecone - vectwo;
    ((vec.x * vec.x) + (vec.y * vec.y)).abs().sqrt()
}

// Not a true force-on-point, it's quickmath.
fn planet_force_on_point(forcepoint: Vec3, planetlocation: Vec3, planet_mass: f32) -> f32 {
    planet_mass / distance_between_two_vec(forcepoint, planetlocation).powi(2)
}