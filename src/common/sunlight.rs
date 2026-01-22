use bevy::prelude::*;

use crate::product::{Product, ProductMeta};

#[derive(Debug, Clone, Message)]
pub struct Sunlight(pub u32);

impl Product for Sunlight {}

#[derive(Debug)]
pub struct SunlightProductProcessor;

impl crate::product::ProductProcessor for SunlightProductProcessor {
    type Output = Sunlight;

    fn process(&self, product_meta: &ProductMeta) -> Option<Self::Output> {
        if product_meta.name == "Sunlight" {
            Some(Sunlight(product_meta.value as u32))
        } else {
            None
        }
    }
}
