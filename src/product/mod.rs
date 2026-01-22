use bevy::{ecs::event::Event, platform::collections::HashMap};
use downcast_rs::{Downcast, impl_downcast};

/// 产品元数据
pub struct ProductMeta {
    name: String,
}

/// 产品
pub trait Product: Event + Clone {}

pub trait ErasedProduct: Downcast {}

impl<T: Product> ErasedProduct for T {}

impl_downcast!(ErasedProduct);

pub trait ProductProcessor {
    type Output: Product;

    fn process(&self, product_meta: &ProductMeta) -> Option<Self::Output>;
}

pub trait ErasedProductProcessor: Downcast {
    fn process(&self, product_meta: &ProductMeta) -> Option<Box<dyn ErasedProduct>>;
}

impl_downcast!(ErasedProductProcessor);

pub struct ProductProcessorContainer(HashMap<String, Box<dyn ErasedProductProcessor>>);

impl ProductProcessorContainer {
    pub fn create<T: Product>(&self, product_meta: &ProductMeta) -> Option<T> {
        self.0.get(&product_meta.name).and_then(|processor| {
            processor
                .process(product_meta)
                .and_then(|erased_product| erased_product.downcast_ref::<T>().cloned())
        })
    }
}
