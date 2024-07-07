use {bevy::prelude::*, std::marker::PhantomData};

#[derive(Resource)]
pub struct TextureAtlasOwner<T> {
    tex: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
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

    pub fn texture(&self) -> Handle<Image> {
        self.tex.clone_weak()
    }

    pub fn layout(&self) -> Handle<TextureAtlasLayout> {
        self.layout.clone_weak()
    }
}

#[derive(Resource)]
pub struct FontOwner<T> {
    font: Handle<Font>,
    _marker: PhantomData<T>,
}

impl<T: Component> FontOwner<T> {
    pub fn new(font: Handle<Font>) -> Self {
        Self {
            font,
            _marker: PhantomData,
        }
    }

    pub fn font(&self) -> Handle<Font> {
        self.font.clone_weak()
    }
}
