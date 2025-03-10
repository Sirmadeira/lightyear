//! Plugin to register and handle user inputs.

use bevy::{
    app::{App, Plugin},
    ecs::entity::MapEntities,
};

use crate::{
    client::config::ClientConfig,
    inputs::native::input_message::InputMessage,
    prelude::{ChannelDirection, UserAction},
    protocol::message::registry::AppMessageInternalExt,
    server::config::ServerConfig,
    shared::input::InputConfig,
};

pub struct InputPlugin<A: UserAction> {
    pub config: InputConfig<A>,
}

impl<A: UserAction> Default for InputPlugin<A> {
    fn default() -> Self {
        Self {
            config: Default::default(),
        }
    }
}

impl<A: UserAction + MapEntities> Plugin for InputPlugin<A> {
    fn build(&self, app: &mut App) {
        // TODO: this adds a receive_message fn that is never used! Because we have custom handling
        //  of native input message in ConnectionManager.receive()
        app.register_message_internal::<InputMessage<A>>(ChannelDirection::Bidirectional)
            // add entity mapping for:
            // - server receiving pre-predicted entities
            // - client receiving other players' inputs
            // - input itself containing entities
            .add_map_entities();
        let is_client = app.world().get_resource::<ClientConfig>().is_some();
        let is_server = app.world().get_resource::<ServerConfig>().is_some();
        assert!(is_client || is_server, "Either ClientConfig or ServerConfig must be present! Make sure that your SharedPlugin is registered after the ClientPlugins/ServerPlugins");
        if is_client {
            app.add_plugins(crate::client::input::native::InputPlugin::<A>::new(
                self.config.clone(),
            ));
        }
        if is_server {
            app.add_plugins(crate::server::input::native::InputPlugin::<A> {
                rebroadcast_inputs: self.config.rebroadcast_inputs,
                marker: std::marker::PhantomData,
            });
        }
    }
}
