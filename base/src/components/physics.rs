use bevy_ecs::prelude::Component;
use bevy_math::prelude as math;
use bevy_transform::prelude::Transform as BevyTransform;

use naia_bevy_shared::{Property, Replicate, Serde};

use bevy_rapier2d::prelude::{ExternalImpulse as RapierImpulse, Velocity as RapierVelocity};

// These types can and should only be constructed from and into their
// corresponding Bevy and Rapier types. They are here to help sync Bevy and
// Rapier values over the network.

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl From<&Vec2> for math::Vec2 {
    fn from(value: &Vec2) -> Self {
        math::Vec2::new(value.x, value.y)
    }
}

impl From<&math::Vec2> for Vec2 {
    fn from(value: &math::Vec2) -> Self {
        Vec2 {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
}

impl From<Vec3> for math::Vec3 {
    fn from(value: Vec3) -> Self {
        math::Vec3::new(value.x, value.y, value.z)
    }
}

impl From<math::Vec3> for Vec3 {
    fn from(value: math::Vec3) -> Self {
        Vec3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serde)]
pub struct Quat {
    inner: [f32; 4],
}

impl From<Quat> for math::Quat {
    fn from(value: Quat) -> Self {
        math::Quat::from_array(value.inner)
    }
}

impl From<math::Quat> for Quat {
    fn from(value: math::Quat) -> Self {
        Quat {
            inner: value.to_array(),
        }
    }
}

// velocity

#[derive(Component, Replicate)]
pub struct Velocity {
    pub linear: Property<Vec2>,
    pub angular: Property<f32>,
}

impl From<&Velocity> for RapierVelocity {
    fn from(value: &Velocity) -> Self {
        RapierVelocity {
            linvel: (&*value.linear).into(),
            angvel: *value.angular,
        }
    }
}

impl From<&RapierVelocity> for Velocity {
    fn from(value: &RapierVelocity) -> Self {
        Velocity::new_complete((&value.linvel).into(), value.angvel)
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
    pub linear: Property<Vec2>,
    pub angular: Property<f32>,
}

impl From<&ExternalImpulse> for RapierImpulse {
    fn from(value: &ExternalImpulse) -> Self {
        RapierImpulse {
            impulse: (&*value.linear).into(),
            torque_impulse: *value.angular,
        }
    }
}
impl From<&RapierImpulse> for ExternalImpulse {
    fn from(value: &RapierImpulse) -> Self {
        ExternalImpulse::new_complete((&value.impulse).into(), value.torque_impulse)
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
    inner: Property<Vec3>,
}

impl Translation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Translation::new_complete(Vec3::new(x, y, z))
    }
}

impl From<Vec3> for Translation {
    fn from(value: Vec3) -> Self {
        Translation::new_complete(value)
    }
}

impl From<&Translation> for Vec3 {
    fn from(value: &Translation) -> Self {
        *value.inner
    }
}

impl From<&BevyTransform> for Translation {
    fn from(transform: &BevyTransform) -> Self {
        Translation::new_complete(transform.translation.into())
    }
}

impl Default for Translation {
    fn default() -> Self {
        Translation::new_complete(Vec3::new(0., 0., 1.))
    }
}

impl std::fmt::Debug for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SyncTranslation")
            .field(&*self.inner)
            .finish()
    }
}

#[derive(Component, Replicate)]
pub struct Rotation {
    inner: Property<Quat>,
}

impl From<Quat> for Rotation {
    fn from(value: Quat) -> Self {
        Rotation::new_complete(value)
    }
}

impl From<&Rotation> for Quat {
    fn from(value: &Rotation) -> Self {
        *value.inner
    }
}

impl From<&BevyTransform> for Rotation {
    fn from(transform: &BevyTransform) -> Self {
        Rotation::new_complete(transform.rotation.into())
    }
}

impl std::fmt::Debug for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SyncRotation").field(&*self.inner).finish()
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Quat::from(bevy_math::Quat::default()).into()
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
            translation: (*value.0.inner).into(),
            rotation: (*value.1.inner).into(),
            ..Default::default()
        }
    }
}
