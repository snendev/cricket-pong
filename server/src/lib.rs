use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::{
    Entity, IntoSystemConfigs, IntoSystemSetConfig, Resource, States, SystemSet,
};
use bevy_utils::HashMap;

use cricket_pong_game::GameplayPlugin;
use naia_bevy_server::UserKey;
use naia_bevy_server::{Plugin as NaiaServerPlugin, ReceiveEvents, ServerConfig};

use cricket_pong_game::base::protocol::protocol;

pub mod connection;
pub mod init;
pub mod tick;

#[derive(Resource, Default)]
pub struct UserEntities {
    user_to_entity_map: HashMap<UserKey, Entity>,
    entity_to_user_map: HashMap<Entity, UserKey>,
}

impl UserEntities {
    #[allow(dead_code)]
    fn get_entity(&self, user: &UserKey) -> Option<&Entity> {
        self.user_to_entity_map.get(user)
    }

    #[allow(dead_code)]
    fn get_user(&self, entity: &Entity) -> Option<&UserKey> {
        self.entity_to_user_map.get(entity)
    }

    fn insert(&mut self, user_key: UserKey, entity: Entity) {
        self.user_to_entity_map.insert(user_key, entity);
        self.entity_to_user_map.insert(entity, user_key);
    }

    fn remove(&mut self, user: &UserKey) -> Option<Entity> {
        self.user_to_entity_map.remove(user).and_then(|entity| {
            self.entity_to_user_map.remove(&entity);
            Some(entity)
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
struct TickSet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
enum ServerState {
    #[default]
    Active,
}

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NaiaServerPlugin::new(
            ServerConfig {
                require_auth: false,
                ..Default::default()
            },
            protocol(),
        ))
        .configure_set(Update, TickSet.in_set(ReceiveEvents))
        .init_resource::<UserEntities>()
        .add_systems(Startup, init::init)
        .add_systems(
            Update,
            (
                (
                    connection::connect_events,
                    connection::disconnect_events,
                    connection::error_events,
                )
                    .before(TickSet),
                tick::update_entity_scopes.after(TickSet),
            )
                .in_set(ReceiveEvents),
        )
        .add_plugins(GameplayPlugin::new(
            TickSet,
            ServerState::Active,
            tick::tick_events,
        ));
    }
}
