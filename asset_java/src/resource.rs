use bevy::{
    asset::{Asset, AssetLoader, LoadContext, io::Reader},
    image::Image,
};
use thiserror::Error;

thread_local! {
    static LOAD_CONTEXT_PTR: std::cell::Cell<*mut ()> = std::cell::Cell::new(std::ptr::null_mut());
}

fn with_load_context<R>(f: impl FnOnce(&mut LoadContext<'_>) -> R) -> R {
    LOAD_CONTEXT_PTR.with(|cell| {
        let ptr = cell.get();
        assert!(!ptr.is_null());
        let ctx: &mut LoadContext<'_> = unsafe { &mut *(ptr as *mut LoadContext<'_>) };
        f(ctx)
    })
}

pub struct JsonLoader<A>(std::marker::PhantomData<A>);

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum JsonLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::error::Error),
}

impl<A> Default for JsonLoader<A> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<A: ResourceType> AssetLoader for JsonLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = JsonLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        LOAD_CONTEXT_PTR.with(|cell| cell.set(load_context as *mut _ as *mut ()));
        let asset = serde_json::from_slice::<A>(&bytes);
        LOAD_CONTEXT_PTR.with(|cell| cell.set(std::ptr::null_mut()));
        Ok(asset?)
    }

    fn extensions(&self) -> &[&str] {
        let extension: &'static str = format!("{}.{}", A::prefix(), A::extension()).leak();
        vec![extension].leak()
    }
}

pub trait ResourceType {
    fn prefix() -> &'static str;

    fn extension() -> &'static str;
}

impl ResourceType for Image {
    fn prefix() -> &'static str {
        "textures"
    }

    fn extension() -> &'static str {
        "png"
    }
}

pub mod resource_location {
    use bevy::asset::{Asset, Handle};
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::resource::{ResourceType, with_load_context};

    pub fn serialize<S, A>(handle: &Handle<A>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        A: Asset + ResourceType,
    {
        use serde::ser::Error;

        let path = handle.path().ok_or_else(|| S::Error::custom(""))?;
        let mut path_components = path.path().components();

        let Some(std::path::Component::Normal(namespace)) = path_components.next() else {
            return Err(S::Error::custom(""));
        };
        let mut namespace = namespace.to_string_lossy().into_owned();
        namespace.push(':');
        let _prefix = path_components.next();
        let resource_location = path_components.fold(namespace, |mut path, component| {
            path.push_str(&component.as_os_str().to_string_lossy());
            path
        });
        let (resource_location, _extension) = resource_location
            .split_once('.')
            .ok_or_else(|| S::Error::custom(""))?;

        serializer.serialize_str(resource_location)
    }

    pub fn deserialize<'de, D, A>(deserializer: D) -> Result<Handle<A>, D::Error>
    where
        D: Deserializer<'de>,
        A: Asset + ResourceType,
    {
        let resource_location = String::deserialize(deserializer)?;
        let (namespace, path) = resource_location
            .split_once(':')
            .unwrap_or(("minecraft", &resource_location));
        let handle = with_load_context(|ctx| {
            ctx.load::<A>(format!(
                "{namespace}/{prefix}/{path}.{extension}",
                prefix = A::prefix(),
                extension = A::extension()
            ))
        });
        Ok(handle)
    }
}
