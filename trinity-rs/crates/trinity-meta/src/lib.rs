use bevy::ecs::reflect::ReflectResource;
use std::collections::BTreeMap;

use bevy::{prelude::*, reflect::GetTypeRegistration};
use blueexposure::{BlueClassInfo, BlueClassKind, BlueFieldInfo, BlueRegistry};
use delegate::delegate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Reflect, Serialize, Deserialize)]
pub enum TrinityClassKind {
    #[default]
    Component,
    Resource,
    Asset,
    Value,
}

impl TrinityClassKind {
    #[must_use]
    pub const fn blue_kind(self) -> BlueClassKind {
        match self {
            Self::Resource => BlueClassKind::Resource,
            Self::Component | Self::Asset | Self::Value => BlueClassKind::Component,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Reflect, Serialize, Deserialize)]
pub struct TrinityFieldInfo {
    pub name: String,
    pub readable: bool,
    pub writable: bool,
    pub persisted: bool,
}

impl TrinityFieldInfo {
    #[must_use]
    pub fn new(name: impl Into<String>, readable: bool, writable: bool, persisted: bool) -> Self {
        Self { name: name.into(), readable, writable, persisted }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Reflect, Serialize, Deserialize)]
pub struct TrinityClassInfo {
    pub module: String,
    pub carbon_name: String,
    pub rust_name: String,
    pub legacy_header: String,
    pub doc: String,
    pub kind: TrinityClassKind,
    pub fields: Vec<TrinityFieldInfo>,
}

impl TrinityClassInfo {
    #[must_use]
    pub fn qualified_name(&self) -> String { format!("{}.{}", self.module, self.carbon_name) }

    #[must_use]
    pub fn as_blue_class(&self) -> BlueClassInfo {
        BlueClassInfo::new(
            self.module.clone(), self.carbon_name.clone(), self.doc.clone(), self.kind.blue_kind(),
            self.fields.iter().map(|field| BlueFieldInfo::new(field.name.clone(), field.readable, field.writable, field.persisted)).collect(),
        )
    }
}

pub trait TrinityClass: GetTypeRegistration + Send + Sync + 'static {
    const CARBON_MODULE: &'static str;
    const CARBON_NAME: &'static str;
    fn trinity_class_info() -> TrinityClassInfo;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Reflect, Serialize, Deserialize)]
pub enum VerificationClass { Kani, Native, RenderDevice, Screenshot, Integration, Unmapped }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrinityTestDescriptor {
    pub legacy: &'static str,
    pub rust_test: &'static str,
    pub verification: VerificationClass,
}

impl TrinityTestDescriptor {
    #[must_use]
    pub const fn new(legacy: &'static str, rust_test: &'static str, verification: VerificationClass) -> Self {
        Self { legacy, rust_test, verification }
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct TrinityRegistry { classes: BTreeMap<String, TrinityClassInfo> }

impl TrinityRegistry {
    pub fn register_class(&mut self, info: TrinityClassInfo) { self.classes.insert(info.qualified_name(), info); }

    #[must_use]
    pub fn class(&self, qualified_name: &str) -> Option<&TrinityClassInfo> { self.classes.get(qualified_name) }

    delegate! {
        to self.classes {
            #[call(len)]
            #[must_use]
            pub fn class_count(&self) -> usize;

            #[call(is_empty)]
            #[must_use]
            pub fn is_empty(&self) -> bool;
        }
    }

    pub fn classes(&self) -> impl Iterator<Item = &TrinityClassInfo> { self.classes.values() }
}

pub trait TrinityAppExt {
    fn register_trinity_class<T>(&mut self) -> &mut Self where T: TrinityClass;
}

impl TrinityAppExt for App {
    fn register_trinity_class<T>(&mut self) -> &mut Self where T: TrinityClass {
        self.init_resource::<TrinityRegistry>()
            .register_type::<TrinityRegistry>()
            .register_type::<TrinityClassKind>()
            .register_type::<TrinityFieldInfo>()
            .register_type::<TrinityClassInfo>()
            .register_type::<VerificationClass>()
            .register_type::<T>();
        let info = T::trinity_class_info();
        self.world_mut().resource_mut::<TrinityRegistry>().register_class(info.clone());
        self.init_resource::<BlueRegistry>();
        self.world_mut().resource_mut::<BlueRegistry>().register_class(info.as_blue_class());
        self
    }
}

pub mod prelude {
    pub use crate::{TrinityAppExt, TrinityClass, TrinityClassInfo, TrinityClassKind, TrinityFieldInfo, TrinityRegistry, TrinityTestDescriptor, VerificationClass};
}
