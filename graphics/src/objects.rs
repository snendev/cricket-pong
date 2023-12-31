use bevy::{
    prelude::{
        shape, Added, App, Assets, Color, Commands, Entity, IntoSystemConfigs, Mesh, Plugin,
        PostUpdate, Query, ResMut, SystemSet, Transform, Vec2,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use bevy_prototype_lyon::prelude::{
    shapes, Fill, GeometryBuilder, ShapeBundle, ShapePlugin, Stroke,
};

use cricket_pong_base::{
    ball::Ball,
    batter::{Batter, Wicket},
    fielder::{Boundary, Fielder, FielderRing},
};

fn setup_ball_shape(
    mut commands: Commands,
    added_ball_query: Query<(Entity, &Transform), Added<Ball>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform) in added_ball_query.iter() {
        commands.entity(entity).insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(Ball::RADIUS).into()).into(),
            material: materials.add(Color::MIDNIGHT_BLUE.into()),
            transform: *transform,
            ..Default::default()
        });
    }
}

fn setup_field_shape(
    mut commands: Commands,
    added_fielder_ring_query: Query<(Entity, &FielderRing), Added<FielderRing>>,
) {
    for (entity, fielder_ring) in added_fielder_ring_query.iter() {
        let shape = shapes::Circle {
            radius: fielder_ring.radius(),
            ..Default::default()
        };
        commands.entity(entity).insert((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..Default::default()
            },
            Stroke::new(Color::BLACK, 4.),
        ));
    }
}

fn setup_fielder_shape(
    mut commands: Commands,
    added_fielder_query: Query<(Entity, &Transform, &Fielder), Added<Fielder>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform, fielder) in added_fielder_query.iter() {
        commands.entity(entity).insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Quad::new(Vec2::new(fielder.hwidth() * 2., Fielder::HDEPTH * 2.)).into(),
                )
                .into(),
            material: materials.add(Color::AQUAMARINE.into()),
            transform: *transform,
            ..Default::default()
        });
    }
}

fn setup_boundary_shape(
    mut commands: Commands,
    added_boundary_query: Query<Entity, Added<Boundary>>,
) {
    for entity in added_boundary_query.iter() {
        let shape = shapes::Circle {
            radius: Boundary::RADIUS,
            ..Default::default()
        };
        commands.entity(entity).insert((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..Default::default()
            },
            Stroke::new(Color::DARK_GREEN, 8.),
        ));
    }
}

fn setup_batter_shape(
    mut commands: Commands,
    added_batter_query: Query<(Entity, &Transform), Added<Batter>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform) in added_batter_query.iter() {
        let shape = shapes::Circle {
            radius: Batter::RADIUS,
            ..Default::default()
        };
        // batter ring
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..Default::default()
            },
            Stroke::new(Color::BLACK, 4.),
        ));
        // bat itself
        commands.entity(entity).insert((MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(Batter::HWIDTH * 2., Batter::HDEPTH * 2.)).into())
                .into(),
            material: materials.add(Color::rgb(0.59, 0.29, 0.).into()),
            transform: *transform,
            ..Default::default()
        },));
    }
}

fn setup_wicket_shape(mut commands: Commands, added_wicket_query: Query<Entity, Added<Wicket>>) {
    for entity in added_wicket_query.iter() {
        let shape = shapes::Circle {
            radius: Wicket::RADIUS,
            ..Default::default()
        };
        commands.entity(entity).insert((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..Default::default()
            },
            Stroke::new(Color::BLACK, 2.),
            Fill::color(Color::DARK_GREEN),
        ));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, SystemSet)]
pub struct ObjectGraphicsSet;

pub struct ObjectGraphicsPlugin;

impl Plugin for ObjectGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin).add_systems(
            PostUpdate,
            (
                setup_ball_shape,
                setup_field_shape,
                setup_fielder_shape,
                setup_boundary_shape,
                setup_batter_shape,
                setup_wicket_shape,
            )
                .in_set(ObjectGraphicsSet),
        );
    }
}
