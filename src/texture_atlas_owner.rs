use {bevy::prelude::*, std::marker::PhantomData};

#[derive(Resource)]
pub struct TextureAtlasOwner<T> {
    pub tex: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    _marker: PhantomData<T>,
}

impl<T> TextureAtlasOwner<T> {
    pub fn new(tex: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            tex,
            layout,
            _marker: PhantomData,
        }
    }
}
