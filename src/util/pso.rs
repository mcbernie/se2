// should not used anymore!!

use std::sync::mpsc;
use std::path::PathBuf;
use gfx::traits::FactoryExt;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use gfx::*;

use transitions;

/// Container for pipeline state object and a factory
pub trait PsoCell<R: Resources, F: Factory<R>, I: pso::PipelineInit> {
  fn pso(&mut self) -> &mut PipelineState<R, I::Meta>;
  fn factory(&mut self) -> &mut F;
}


pub fn shader_bytes(path: &String) -> Result<Vec<u8>, Box<Error>> {
  let mut shader = Vec::new();

  File::open(path)?.read_to_end(&mut shader)?;
  Ok(shader)
}


/*fn combine_shader_bytes(default: &String, shader: &'static [u8]) -> Result<Vec<u8>, Box<Error>> {
  let mut default_shader_content = String::new();
  File::open(default)?.read_to_string(&mut default_shader_content)?;

  let mut shader_content = String::new();
  File::open(shader)?.read_to_string(&mut shader_content)?;

  let new_shader = default_shader_content.replace("// REPLACE", shader_content.as_str());

  //println!("new_shader: {:}", new_shader);

  let shader:Vec<u8> = new_shader.into_bytes();

  Ok(shader)
}*/

/// Container that watches shader files and reloads pipeline state object after modification
pub struct WatcherPsoCell<'r, R: Resources, F: Factory<R>, I: pso::PipelineInit> {
  vertex_shader:  Option<&'r [u8]>,
  fragment_shader: Option<&'r [u8]>,
  init: I,
  primitive: Primitive,
  rasterizer: state::Rasterizer,
  shader_mods: mpsc::Receiver<transitions::NextTransition>,
  factory: F,
  pso: PipelineState<R, I::Meta>,
}

impl<'r, R: Resources, F: Factory<R>, I: pso::PipelineInit + Clone> WatcherPsoCell<'r, R, F, I> {

  fn recv_modified_pso(&mut self) -> Option<PipelineState<R, I::Meta>>
  where
    R: Resources,
    F: Factory<R>,
  {

    if let Ok(event) = self.shader_mods.try_recv() {
      match event {
        transitions::NextTransition(shader) => {
          //self.fragment_shader = Some(shader.as_bytes());
          match self.build_pso() {
            Ok(pso) => {
              return Some(pso);
            }
            Err(err) => println!("Error in Fragment Shader: {:?}", err)
          }
        }
        _ => {
          println!("other called...")
        }
      }
    }
    None
  }

  fn build_pso(&mut self) -> Result<PipelineState<R, I::Meta>, Box<Error>>
  where
    R: Resources,
    F: Factory<R>,
  {

    //let fragment_shader = combine_shader_bytes(&String::from("./assets/default.glslf"), &self.fragment_shader)?;
    //let fragment_shader = shader_bytes(&self.fragment_shader)?;
    //let vertex_shader = shader_bytes(&self.vertex_shader)?;

    let set = self
      .factory
      .create_shader_set(self.vertex_shader.unwrap(), self.fragment_shader.unwrap())?;

    Ok(self.factory.create_pipeline_state(
      &set,
      self.primitive,
      self.rasterizer,
      self.init.clone(),
    )?)

  }
}

impl<'r, R: Resources, F: Factory<R>, I: pso::PipelineInit + Clone> PsoCell<R, F, I> for WatcherPsoCell<'r, R, F, I>
{

  fn pso(&mut self) -> &mut PipelineState<R, I::Meta> {
    if let Some(updated) = self.recv_modified_pso() {
      self.pso = updated;
    }

    &mut self.pso
  }

  fn factory(&mut self) -> &mut F {
    &mut self.factory
  }

}

/// Builds `WatcherPsoCell`
#[derive(Debug)]
pub struct WatcherPsoCellBuilder<'r, I: pso::PipelineInit> {
  vertex_shader: Option<&'r[u8]>,
  fragment_shader: Option<&'r[u8]>,
  primitive: Primitive,
  rasterizer: state::Rasterizer,
  init: I,
}

impl<'r, I: pso::PipelineInit + Clone> WatcherPsoCellBuilder<'r, I> {
  pub fn using(init_struct: I) -> WatcherPsoCellBuilder<'r, I> {
    WatcherPsoCellBuilder {
      vertex_shader: None,
      fragment_shader: None,
      init: init_struct,
      primitive: Primitive::TriangleList,
      rasterizer: state::Rasterizer::new_fill(),
    }
  }

  pub fn vertex_shader(mut self, shader_byte: &'r [u8]) -> Self {
    self.vertex_shader = Some(shader_byte);
    self
  }

  pub fn fragment_shader(mut self, shader_byte: &'r [u8]) -> Self {
    self.fragment_shader = Some(shader_byte);
    self
  }

  /*pub fn primitive(mut self, p: Primitive) -> WatcherPsoCellBuilder<I> {
    self.primitive = p;
    self
  }

  pub fn rasterizer(mut self, r: state::Rasterizer) -> WatcherPsoCellBuilder<I> {
    self.rasterizer = r;
    self
  }*/


  pub fn build<R, F>(self, mut factory: F, shader_mods: mpsc::Receiver<transitions::NextTransition>) -> Result<WatcherPsoCell<'r, R, F, I>, Box<Error>>
    where
    R: Resources,
    F: Factory<R>,
  {

    let pso = {
      let set = factory.create_shader_set(self.vertex_shader.unwrap(), self.fragment_shader.unwrap())?;
      factory.create_pipeline_state(&set, self.primitive, self.rasterizer, self.init.clone())?
    };

    Ok(WatcherPsoCell {
      vertex_shader: Some(self.vertex_shader.ok_or("missing vertex shader")?),
      fragment_shader: Some(self.fragment_shader.ok_or("missing fragment shader")?),
      init: self.init,
      primitive: self.primitive,
      rasterizer: self.rasterizer,
      shader_mods,

      factory,
      pso,
    })

  }

}