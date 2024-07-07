use {bevy::prelude::*, std::marker::PhantomData};

#[derive(Resource)]
pub struct TextureAtlasOwner<T> {
    pub tex: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    _marker: PhantomData<T>,
}

impl<T: Component> TextureAtlasOwner<T> {
    pub fn new(tex: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            tex,
            layout,
            _marker: PhantomData,
        }
    }
}

#[derive(Resource)]
pub struct FontOwner<T> {
    pub font: Handle<Font>,
    _marker: PhantomData<T>,
}

impl<T: Component> FontOwner<T> {
    pub fn new(font: Handle<Font>) -> Self {
        Self {
            font,
            _marker: PhantomData,
        }
    }
}
