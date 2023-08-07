use bevy::{
    prelude::{
        debug, shape, Added, App, Assets, BuildChildren, Color, Commands, Entity,
        IntoSystemConfigs, Mesh, Or, Plugin, PostUpdate, Query, ResMut, SystemSet, Transform, Vec2,
        With,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use bevy_prototype_lyon::prelude::{
    shapes, Fill, GeometryBuilder, ShapeBundle, ShapePlugin, Stroke,
};

use cricket_pong_base::components::{
    ball::Ball,
    batter::Batter,
    boundary::Boundary,
    fielder::{Fielder, FielderTrack},
    wicket::Wicket,
};

use crate::ShouldRender;

type WithRenderedBall = (
    With<Ball>,
    With<ShouldRender>,
    Or<(Added<Ball>, Added<Transform>, Added<ShouldRender>)>,
);

fn setup_ball_shape(
    mut commands: Commands,
    added_ball_query: Query<(Entity, &Transform), WithRenderedBall>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform) in added_ball_query.iter() {
        debug!("Attaching Ball graphics for entity ({:?})", entity);
        commands.entity(entity).insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(Ball::RADIUS).into()).into(),
            material: materials.add(Color::MIDNIGHT_BLUE.into()),
            transform: *transform,
            ..Default::default()
        });
    }
}

type WithAddedFielderTrack = (
    With<ShouldRender>,
    Or<(Added<FielderTrack>, Added<ShouldRender>)>,
);

fn setup_field_shape(
    mut commands: Commands,
    added_fielder_ring_query: Query<(Entity, &FielderTrack), WithAddedFielderTrack>,
) {
    for (entity, track) in added_fielder_ring_query.iter() {
        debug!("Attaching FieldTrack graphics for entity ({:?})", entity);
        let shape = shapes::Circle {
            radius: track.ring.radius(),
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

type WithAddedFielder = (
    With<ShouldRender>,
    Or<(Added<Fielder>, Added<Transform>, Added<ShouldRender>)>,
);

fn setup_fielder_shape(
    mut commands: Commands,
    added_fielder_query: Query<(Entity, &Transform, &Fielder), WithAddedFielder>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform, fielder) in added_fielder_query.iter() {
        debug!("Attaching Fielder graphics for entity ({:?})", entity);
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

type WithAddedBoundary = (
    With<Boundary>,
    With<ShouldRender>,
    Or<(Added<Boundary>, Added<ShouldRender>)>,
);

fn setup_boundary_shape(
    mut commands: Commands,
    added_boundary_query: Query<Entity, WithAddedBoundary>,
) {
    for entity in added_boundary_query.iter() {
        debug!("Attaching Boundary graphics for entity ({:?})", entity);
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

type WithAddedBatter = (
    With<Batter>,
    With<ShouldRender>,
    Or<(Added<Batter>, Added<Transform>, Added<ShouldRender>)>,
);

fn setup_batter_shape(
    mut commands: Commands,
    added_batter_query: Query<(Entity, &Transform), WithAddedBatter>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform) in added_batter_query.iter() {
        debug!("Attaching Batter graphics for entity ({:?})", entity);
        let shape = shapes::Circle {
            radius: Batter::RADIUS,
            ..Default::default()
        };
        // bat itself
        commands
            .entity(entity)
            .insert(MaterialMesh2dBundle {
                mesh: meshes
                    .add(
                        shape::Quad::new(Vec2::new(Batter::HWIDTH * 2., Batter::HDEPTH * 2.))
                            .into(),
                    )
                    .into(),
                material: materials.add(Color::rgb(0.59, 0.29, 0.).into()),
                transform: *transform,
                ..Default::default()
            })
            .with_children(|parent| {
                // batter ring
                parent.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shape),
                        transform: Transform::from_xyz(-Batter::RADIUS - Batter::HWIDTH, 0., 0.),
                        ..Default::default()
                    },
                    Stroke::new(Color::BLACK, 4.),
                ));
            });
    }
}

type WithAddedWicket = (
    With<Wicket>,
    With<ShouldRender>,
    Or<(Added<Wicket>, Added<ShouldRender>)>,
);

fn setup_wicket_shape(mut commands: Commands, added_wicket_query: Query<Entity, WithAddedWicket>) {
    for entity in added_wicket_query.iter() {
        debug!("Attaching Wicket graphics for entity ({:?})", entity);
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
