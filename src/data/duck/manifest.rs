use bevy::{prelude::*, utils::HashMap};
use leafwing_manifest::{
    identifier::Id,
    manifest::{Manifest, ManifestFormat},
};
use serde::{Deserialize, Serialize};

use crate::game::assets::{HandleMap, ImageKey};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RawDuckData {
    name: String,
    position: Vec2,
    scale: Vec3,
    sprite: ImageKey,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)] // illustration only
pub struct DuckData {
    pub name: String,
    pub position: Vec2,
    pub scale: Vec3,
    pub sprite: Handle<Image>,
}

#[derive(Debug, Resource, PartialEq)]
pub struct DuckManifest {
    pub(crate) ducks: HashMap<Id<DuckData>, DuckData>,
}

#[derive(Debug, Asset, TypePath, Serialize, Deserialize, PartialEq)]
pub struct RawDuckManifest {
    items: Vec<RawDuckData>,
}

impl Manifest for DuckManifest {
    type Item = DuckData;
    type RawItem = RawDuckData;
    type RawManifest = RawDuckManifest;
    type ConversionError = std::convert::Infallible;

    const FORMAT: ManifestFormat = ManifestFormat::Ron;

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.ducks.get(&id)
    }

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        world: &mut World,
    ) -> Result<Self, Self::ConversionError> {
        info!("Loading RawDuckManifest");

        let sprite_handles = world.resource::<HandleMap<ImageKey>>();

        let ducks: HashMap<_, _> = raw_manifest
            .items
            .into_iter()
            .map(|raw_item| {
                let item = DuckData {
                    name: raw_item.name.clone(),
                    position: raw_item.position,
                    scale: raw_item.scale,
                    sprite: sprite_handles[&raw_item.sprite].clone_weak(),
                };

                let id = Id::from_name(&raw_item.name);

                (id, item)
            })
            .collect();

        Ok(DuckManifest { ducks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_duck_manifest() {
        let mut items = Vec::default();

        items.push(RawDuckData {
            name: "duck1".into(),
            position: Vec2::new(150., 250.),
            scale: Vec3::new(0.3, 0.3, 1.0),
            sprite: ImageKey::Ducky,
        });

        items.push(RawDuckData {
            name: "duck2".into(),
            position: Vec2::new(250., 150.),
            scale: Vec3::new(0.2, 0.2, 1.0),
            sprite: ImageKey::Ducky,
        });

        let raw_manifest: RawDuckManifest = RawDuckManifest { items };
        let serialized = ron::ser::to_string_pretty(&raw_manifest, Default::default()).unwrap();
        std::fs::write("assets/data/ducks.ron", &serialized).unwrap();

        assert_eq!(
            raw_manifest,
            ron::de::from_str::<RawDuckManifest>(&serialized).unwrap()
        );
    }
}
