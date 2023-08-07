use bevy_ecs::prelude::Component;
use bevy_math::{Quat, Vec2, Vec3};
use bevy_transform::prelude::Transform as BevyTransform;

use naia_bevy_shared::{Property, Replicate};

use bevy_rapier2d::prelude::{ExternalImpulse as RapierImpulse, Velocity as RapierVelocity};

// These types can and should only be constructed from and into their
// corresponding Bevy and Rapier types. They are here to help sync Bevy and
// Rapier values over the network.

// velocity

#[derive(Component, Replicate)]
pub struct Velocity {
    pub linear: Property<(f32, f32)>,
    pub angular: Property<f32>,
}

impl From<&Velocity> for RapierVelocity {
    fn from(value: &Velocity) -> Self {
        RapierVelocity {
            linvel: Vec2::new(value.linear.0, value.linear.1),
            angvel: *value.angular,
        }
    }
}

impl From<&RapierVelocity> for Velocity {
    fn from(value: &RapierVelocity) -> Self {
        Velocity::new_complete((value.linvel.x, value.linvel.y), value.angvel)
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Velocity::from(&RapierVelocity::default())
    }
}

impl std::fmt::Debug for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyncVelocity")
            .field("linear", &*self.linear)
            .field("angular", &*self.angular)
            .finish()
    }
}

// impulse

#[derive(Component, Replicate)]
pub struct ExternalImpulse {
    pub linear: Property<(f32, f32)>,
    pub angular: Property<f32>,
}

impl From<&ExternalImpulse> for RapierImpulse {
    fn from(value: &ExternalImpulse) -> Self {
        RapierImpulse {
            impulse: Vec2::new(value.linear.0, value.linear.1),
            torque_impulse: *value.angular,
        }
    }
}
impl From<&RapierImpulse> for ExternalImpulse {
    fn from(value: &RapierImpulse) -> Self {
        ExternalImpulse::new_complete((value.impulse.x, value.impulse.y), value.torque_impulse)
    }
}

impl Default for ExternalImpulse {
    fn default() -> Self {
        ExternalImpulse::from(&RapierImpulse::default())
    }
}

impl std::fmt::Debug for ExternalImpulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyncExternalImpulse")
            .field("linear", &*self.linear)
            .field("angular", &*self.angular)
            .finish()
    }
}

// transform

#[derive(Component, Replicate)]
pub struct Translation {
    x: Property<f32>,
    y: Property<f32>,
    z: Property<f32>,
}

impl Translation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Translation::new_complete(x, y, z)
    }
}

impl From<Vec3> for Translation {
    fn from(value: Vec3) -> Self {
        Translation::new(value.x, value.y, value.z)
    }
}

impl From<&Translation> for Vec3 {
    fn from(value: &Translation) -> Self {
        Vec3::new(*value.x, *value.y, *value.z)
    }
}

impl From<&BevyTransform> for Translation {
    fn from(transform: &BevyTransform) -> Self {
        Translation::from(transform.translation)
    }
}

impl Default for Translation {
    fn default() -> Self {
        Translation::new(0., 0., 1.)
    }
}

impl std::fmt::Debug for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SyncTranslation")
            .field(&*self.x)
            .field(&*self.y)
            .field(&*self.z)
            .finish()
    }
}

#[derive(Component, Replicate)]
pub struct Rotation {
    x: Property<f32>,
    y: Property<f32>,
    z: Property<f32>,
    w: Property<f32>,
}

impl Rotation {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Rotation::new_complete(x, y, z, w)
    }
}

impl From<Quat> for Rotation {
    fn from(value: Quat) -> Self {
        Rotation::new(value.x, value.y, value.z, value.w)
    }
}

impl From<&Rotation> for Quat {
    fn from(value: &Rotation) -> Self {
        Quat::from_xyzw(*value.x, *value.y, *value.z, *value.w)
    }
}

impl From<&BevyTransform> for Rotation {
    fn from(transform: &BevyTransform) -> Self {
        Rotation::from(transform.rotation)
    }
}

impl std::fmt::Debug for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SyncRotation")
            .field(&*self.x)
            .field(&*self.y)
            .field(&*self.z)
            .field(&*self.w)
            .finish()
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Quat::default().into()
    }
}

pub struct Transform<'a>(&'a Translation, &'a Rotation);

impl<'a> Transform<'a> {
    pub fn new(translation: &'a Translation, rotation: &'a Rotation) -> Self {
        Transform(translation, rotation)
    }
}

impl<'a> From<Transform<'a>> for BevyTransform {
    fn from(value: Transform<'a>) -> Self {
        BevyTransform {
            translation: value.0.into(),
            rotation: value.1.into(),
            ..Default::default()
        }
    }
}
