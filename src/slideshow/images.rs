use gfx::*;

use piston_window::*;
use std::path::Path;

use gfx::{
  Resources,
  Factory,
};

pub struct Manager {

}

impl Manager {

  pub fn load_image<F, R>(factory: &mut F, path: &str) -> Texture<R> 
    where R: Resources, F: Factory<R>
  {

    let mut texture_settings = TextureSettings::new();
    texture_settings.set_filter(Filter::Nearest);

    Texture::from_path(
      factory,
      &Path::new(path),
      Flip::None,
      &texture_settings,
    ).unwrap()
  }

}

pub struct ImageItem<R: Resources> {
    texture: Option<Texture<R>>,
    path: String,
}

impl<R: Resources> ImageItem<R> {
  pub fn new<F: Factory<R>>(factory: &mut F, path: String) -> Self {
    ImageItem {
      path,
      texture: None,
    }
  }
}

use slideshow::Item;
impl<R: Resources> Item for ImageItem<R> {

  fn in_view(&mut self, show:bool) {
    // do nothing here
  }
}

use slideshow::Rendering;
impl<R: Resources> Rendering<R> for ImageItem<R> {

  fn get_texture(&self) -> Option<Texture<R>> {
    if let Some(ref e) = self.texture {
      return Some(e.clone());
    }

    None
  }

  // eventuell in zukünftiger version in get _texture implelmentiereun um so speicher zu sparen....
  fn load<F: Factory<R>>(&mut self, factory: &mut F) {
    self.texture = Some(Manager::load_image(factory,self.path.as_str()));
  }
}