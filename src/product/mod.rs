use std::fmt::Debug;

use bevy::{app::App, ecs::{message::Message, resource::Resource}, platform::collections::HashMap};
use downcast_rs::{Downcast, impl_downcast};

/// 产品元数据
#[derive(Debug, Clone, Message)]
pub struct ProductMeta {
    pub name: String,
    pub value: f32,
}

/// 产品
pub trait Product: Message + Clone {}

pub trait ErasedProduct: Downcast {}

impl<T: Product> ErasedProduct for T {}

impl_downcast!(ErasedProduct);

pub trait ProductProcessor: 'static + Send + Sync + Debug {
    type Output: Product;

    fn process(&self, product_meta: &ProductMeta) -> Option<Self::Output>;
}

pub trait ErasedProductProcessor: Downcast + 'static + Send + Sync + Debug {
    fn process(&self, product_meta: &ProductMeta) -> Option<Box<dyn ErasedProduct>>;
}

impl<T: ProductProcessor> ErasedProductProcessor for T {
    fn process(&self, product_meta: &ProductMeta) -> Option<Box<dyn ErasedProduct>> {
        self.process(product_meta)
            .map(|product| Box::new(product) as Box<dyn ErasedProduct>)
    }
}

impl_downcast!(ErasedProductProcessor);

#[derive(Debug, Resource)]
pub struct ProductSystem(HashMap<String, Box<dyn ErasedProductProcessor>>);

impl ProductSystem {
    pub fn empty() -> Self {
        ProductSystem(HashMap::new())
    }

    pub fn register_processor<P>(&mut self, name: &str, processor: P)
    where
        P: ProductProcessor,
    {
        let erased_processor = Box::new(processor);
        self.0.insert(name.to_string(), erased_processor);
    }

    pub fn create<T: Product>(&self, product_meta: &ProductMeta) -> Option<T> {
        self.0.get(&product_meta.name).and_then(|processor| {
            processor
                .process(product_meta)
                .and_then(|erased_product| erased_product.downcast_ref::<T>().cloned())
        })
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_message::<ProductMeta>();
}
