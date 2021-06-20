use bevy::prelude::*;
use crate::components::entities;
use crate::components::physical_attributes as phys;

pub fn gravity_system(
    time: Res<Time>,
    mut set: QuerySet<(
        Query<(&phys::Gravity, &Transform, &entities::Planet)>,
        Query<(&phys::Gravity, &Transform, &mut phys::Velocity)>
    )>
) {
    let mut planet_mass_radius = phys::MassRadius { mass: 0.0, radius: 0.0 };
    let mut planet_translation = Vec3::new(0.0, 0.0, 0.0);
    if let Ok((planet_gravity, planet_transform, _planet)) = set.q0().single() {
        //Some cheating is going on here:
        // - Only one planet
        if let phys::Gravity::Immovable(pmr) = planet_gravity {
            planet_mass_radius.mass = pmr.mass;
            planet_mass_radius.radius = pmr.radius;
            planet_translation.x = planet_transform.translation.x;
            planet_translation.y = planet_transform.translation.y;
        }
    }
    for (object_gravity, object_transform, mut velocity) in set.q1_mut().iter_mut() {
        if let phys::Gravity::Movable(object_mass_radius) = object_gravity {
            let distance_between_objects = distance_between_two_vec(planet_translation, object_transform.translation);
            if distance_between_objects <= (planet_mass_radius.radius + object_mass_radius.radius) {
                // object has crashed - it needs to stop moving
                velocity.velocity.x = 0.0;
                velocity.velocity.y = 0.0;
                velocity.velocity.z = 0.0;
                continue;
            }
            //We should definitely not be calcing this constantly.
            //https://en.wikipedia.org/wiki/Gravitational_constant
            let gravitational_constant: f32 = 6.674f32 * (10f32).powi(-11);
            let force = gravitational_constant * ((object_mass_radius.mass * planet_mass_radius.mass) / distance_between_objects.powi(2));
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