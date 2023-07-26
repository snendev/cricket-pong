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

impl From<&Vec3> for math::Vec3 {
    fn from(value: &Vec3) -> Self {
        math::Vec3::new(value.x, value.y, value.z)
    }
}

impl From<&math::Vec3> for Vec3 {
    fn from(value: &math::Vec3) -> Self {
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

impl From<&Quat> for math::Quat {
    fn from(value: &Quat) -> Self {
        math::Quat::from_array(value.inner)
    }
}

impl From<&math::Quat> for Quat {
    fn from(value: &math::Quat) -> Self {
        Quat {
            inner: value.to_array(),
        }
    }
}

#[derive(Component, Replicate)]
pub struct Transform {
    pub translation: Property<Vec3>,
    pub rotation: Property<Quat>,
    pub scale: Property<Vec3>,
}

impl From<&Transform> for BevyTransform {
    fn from(value: &Transform) -> Self {
        BevyTransform {
            translation: (&*value.translation).into(),
            rotation: (&*value.rotation).into(),
            scale: (&*value.scale).into(),
        }
    }
}

impl From<&BevyTransform> for Transform {
    fn from(value: &BevyTransform) -> Self {
        Transform::new_complete(
            (&value.translation).into(),
            (&value.rotation).into(),
            (&value.scale).into(),
        )
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::from(&BevyTransform::from_xyz(0., 0., 1.))
    }
}

impl std::fmt::Debug for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyncTransform")
            .field("translation", &*self.translation)
            .field("rotation", &*self.rotation)
            .field("scale", &*self.scale)
            .finish()
    }
}

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
