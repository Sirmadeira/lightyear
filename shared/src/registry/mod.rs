use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::Hash;

pub mod channel;
pub mod message;

/// Id used to serialize IDs over the network efficiently
pub(crate) type NetId = u16;

// TODO: read https://willcrichton.net/rust-api-type-patterns/registries.html more in detail

pub trait TypeKind: From<TypeId> + Copy + PartialEq + Eq + Hash {}

/// Struct to map a type to an id that can be serialized over the network
#[derive(Clone)]
pub struct TypeMapper<K: TypeKind> {
    pub(in crate::registry) next_net_id: NetId,
    pub(in crate::registry) kind_map: HashMap<K, NetId>,
    pub(in crate::registry) id_map: HashMap<NetId, K>,
}

impl<K: TypeKind> Default for TypeMapper<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: TypeKind> TypeMapper<K> {
    pub fn new() -> Self {
        Self {
            next_net_id: 0,
            kind_map: HashMap::new(),
            id_map: HashMap::new(),
        }
    }

    /// Register a new type
    pub fn add<T: 'static>(&mut self) -> K {
        let kind = K::from(TypeId::of::<T>());
        if self.kind_map.contains_key(&kind) {
            panic!("Type already registered");
        }
        let net_id = self.next_net_id;
        self.kind_map.insert(kind, net_id);
        self.id_map.insert(net_id, kind);
        self.next_net_id += 1;
        kind
    }

    pub fn kind(&self, net_id: NetId) -> Option<&K> {
        self.id_map.get(&net_id)
    }

    pub fn net_id(&self, kind: &K) -> Option<&NetId> {
        self.kind_map.get(&kind)
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.kind_map.len()
    }
}