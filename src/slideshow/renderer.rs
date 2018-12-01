
use gfx;
use std::collections::hash_map::{Entry, HashMap};
use std::marker::PhantomData;
use gfx::{CommandBuffer, Encoder, Factory, Resources, Slice, CombinedError, PipelineStateError };
use gfx::shade::ProgramError;
use gfx::handle::{
    Buffer,
    RenderTargetView,
    ShaderResourceView,
    Sampler,
};

use gfx::texture::{
    SamplerInfo,
    FilterMethod,
    WrapMode,
};

use gfx::pso::PipelineState;
use gfx::traits::FactoryExt;
use gfx::memory::Typed;

#[derive(Debug)]
pub enum Error {
    PipelineError(PipelineStateError<String>),
    ProgramError(ProgramError),
    CombinedError(CombinedError),
    NoTextureSet(String),
}

impl From<PipelineStateError<String>> for Error {
    fn from(e: PipelineStateError<String>) -> Error { Error::PipelineError(e) }
}

impl From<ProgramError> for Error {
    fn from(e: ProgramError) -> Error { Error::ProgramError(e) }
}

pub struct Renderer<R: Resources, F: Factory<R>  + Clone> {
    factory: F,
    pso_map: HashMap<gfx::format::Format, PipelineState<R, pipe::Meta>>,
    shaders: gfx::ShaderSet<R>,
    vertex_buffer: Buffer<R, Vertex>,
    slice: Slice<R>,
    ratio: f32,
    sampler: Sampler<R>,
    from_shader_view: Option<ShaderResourceView<R, [f32; 4]>>,
    to_shader_view: Option<ShaderResourceView<R, [f32; 4]>>,
}

pub struct RendererBuilder<R: Resources, F: Factory<R>> {
    factory: F,
    _r: PhantomData<R>,
    default_texture: Option<ShaderResourceView<R, [f32; 4]>>,
    ratio: f32,
}

pub fn new<R: Resources, F: Factory<R> + Clone>(factory: F) -> RendererBuilder<R,F> {
    RendererBuilder::new(factory)
}

impl<R: Resources, F: Factory<R> + Clone> RendererBuilder<R,F> {
    pub fn new(factory: F) -> Self {
        RendererBuilder {
            factory: factory,
            ratio: 1.0,
            _r: PhantomData,
            default_texture: None,
        }
    }

    pub fn with_ratio(mut self, ratio: f32) -> Self {
        self.ratio = ratio;
        self
    }

    pub fn with_default_texture(mut self, default_texture: piston_window::Texture<R>) -> Self {
        //self.from  default_texture.view
        self.default_texture = Some(default_texture.view);
        self
    }

    pub fn build(mut self) -> Result<Renderer<R, F>, Error> {

         let vertex_data = vec![
            //top (0, 0, 1)
            Vertex::new([-1.0, -1.0,  1.0], [0, 1]),
            Vertex::new([ 1.0, -1.0,  1.0], [1, 1]),
            Vertex::new([ 1.0,  1.0,  1.0], [1, 0]),
            Vertex::new([-1.0,  1.0,  1.0], [0, 0]),
        ];

        let index_data: &[u16] = &[
            0,  1,  2,  2,  3,  0, // top
        ];

        let (vbuf, slice) = self.factory.create_vertex_buffer_with_slice
            (&vertex_data, index_data);

        // create default shader
        let shaders = try!(self.factory.create_shader_set(VERTEX_SRC, DEFAULT_EMPTY_TRANSITION));

        // create an default sampler
        let sinfo = SamplerInfo::new(
            FilterMethod::Trilinear,
            WrapMode::Clamp);

        let sampler = self.factory.create_sampler(sinfo);

        Ok(
            Renderer {
                factory: self.factory,
                pso_map: HashMap::new(),
                shaders: shaders,
                vertex_buffer: vbuf,
                slice: slice,
                ratio: self.ratio,
                sampler: sampler,
                from_shader_view: self.default_texture.clone(),
                to_shader_view: self.default_texture.clone(),
            }
        )
    }
}

impl<R: Resources, F:Factory<R> + Clone> Renderer<R,F> {

    fn prepare_pso(&mut self, format: gfx::format::Format) -> Result<(), Error> {
        Ok(if let Entry::Vacant(e) = self.pso_map.entry(format) {
            let init = pipe::Init {
                vbuf: (),
                u_model_view_proj: "u_model_view_proj",
                from_texture: "from",
                to_texture: "to",
                ratio: "ratio",
                progress: "progress",
                from_r: "_fromR",
                to_r: "_toR",
                out_color: ("o_Color", format, gfx::state::ColorMask::all(), Some(gfx::preset::blend::ALPHA)),
            };
            let pso = try!(self.factory.create_pipeline_state(
                &self.shaders,
                gfx::Primitive::TriangleList,
                gfx::state::Rasterizer::new_fill(), //.with_cull_back(),
                init,              
            ));
            e.insert(pso);
        })
    }

    pub fn change_fragment_shader( &mut self, shader:&[u8] ) -> Result<(), Error> {
        let shaders = try!(self.factory.create_shader_set(VERTEX_SRC, shader));
        self.shaders = shaders;

        Ok(())
    }



    pub fn set_textures(&mut self, from: piston_window::Texture<R>, to: piston_window::Texture<R>) -> Result<(), Error> {
        self.from_shader_view = Some(from.clone().view);
        self.to_shader_view = Some(to.clone().view);

        Ok(())
    }

    pub fn draw<C: CommandBuffer<R>, T: gfx::format::RenderFormat>(
        &mut self,
        encoder: &mut Encoder<R,C>,
        target: &RenderTargetView<R,T>,
        progress: f32,
    ) -> Result<(), Error> {

        if self.from_shader_view == None {
            return Err(Error::NoTextureSet(String::from("No from Texture is set")));
        }

        if self.to_shader_view == None {
            return Err(Error::NoTextureSet(String::from("No to Texture is set")));
        }

        let from_view = self.from_shader_view.take().unwrap();
        let to_view = self.to_shader_view.take().unwrap();

        let texture_from = (from_view, self.sampler.clone());
        let texture_to = (to_view, self.sampler.clone());

        let data = pipe::Data {
            vbuf: self.vertex_buffer.clone(),
            u_model_view_proj: [[0.0; 4]; 4],
            from_texture: texture_from,
            to_texture: texture_to,
            from_r: 1.0,
            to_r: 1.0,
            ratio: 1.0,
            progress: progress,
            out_color: target.raw().clone(),
        };

        try!(self.prepare_pso(T::get_format()));
        let pso = &self.pso_map[&T::get_format()];

        encoder.draw(&self.slice, pso, &data);

        Ok(())

    }
}

gfx_vertex_struct!( Vertex {
    a_pos: [f32; 4] = "position",
    a_tex_coord: [f32; 2] = "a_uv",
});

impl Vertex {
    fn new(pos: [f32; 3], tc: [i8; 2]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1.0],
            a_tex_coord: [tc[0] as f32, tc[1] as f32],
        }
    }
}

gfx_pipeline_base!( pipe {
    vbuf: gfx::VertexBuffer<Vertex>,
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]>,
    from_texture: gfx::TextureSampler<[f32; 4]>,
    to_texture: gfx::TextureSampler<[f32; 4]>,
    ratio : gfx::Global<f32>,
    progress : gfx::Global<f32>,
    from_r : gfx::Global<f32>,
    to_r : gfx::Global<f32>,
    out_color: gfx::RawRenderTarget,
    //out_color: gfx::BlendTarget<::gfx::format::Srgba8> = ("o_Color", gfx::state::ColorMask::all(), Some(gfx::preset::blend::ALPHA)),
    
    //out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
});

/*gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    from_texture: gfx::TextureSampler<[f32; 4]> = "from",
    to_texture: gfx::TextureSampler<[f32; 4]> = "to",
    ratio : gfx::Global<f32> = "ratio",
    progress : gfx::Global<f32> = "progress",
    from_r : gfx::Global<f32> = "_fromR",
    to_r : gfx::Global<f32> = "_toR",
    out_color: gfx::RawRenderTarget = ("o_Color", gfx::format::Srgba8::new(), gfx::state::ColorMask::all(), Some(gfx::preset::blend::ALPHA)),
    //out_color: gfx::BlendTarget<::gfx::format::Srgba8> = ("o_Color", gfx::state::ColorMask::all(), Some(gfx::preset::blend::ALPHA)),
    
    //out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
});*/

const VERTEX_SRC: &'static [u8] = b"
    #version 120
    attribute vec3 position;
    attribute vec2 a_uv;
    varying vec2 _uv;
    uniform mat4 u_model_view_proj;

    //gl_Position = project * view * world * vec4(position, 1.0);

    void main() {
        gl_Position = vec4(position,1.0);
        //vec2 uv = position * 0.5 + 0.5;
        _uv = a_uv;
    }
";

const FRAGMENT_START_SRC: &'static [u8] = b"
    #version 120
    varying vec2 _uv;
    uniform sampler2D from;
    uniform sampler2D to;
    uniform float progress;
    uniform float ratio;
    uniform float _fromR;
    uniform float _toR;

    vec4 getFromColor(vec2 uv) {
        return texture2D(from, uv);
    }

    vec4 getToColor(vec2 uv) {
        return texture2D(to, uv);
    }
";

const FRAGMENT_END_SRC: &'static [u8] = b"
    void main() {
        gl_FragColor = transition(_uv);
    }
";

const DEFAULT_EMPTY_TRANSITION: &'static [u8] = b"
   #version 120
    varying vec2 _uv;
    uniform sampler2D from;
    uniform sampler2D to;
    uniform float progress;
    uniform float ratio;
    uniform float _fromR;
    uniform float _toR;

    vec4 getFromColor(vec2 uv) {
        return texture2D(from, uv);
    }

    vec4 getToColor(vec2 uv) {
        return texture2D(to, uv);
    }

    ivec2 squaresMin = ivec2(20) ; // minimum number of squares (when the effect is at its higher level)
    int steps = 50; // zero disable the stepping

    float d = min(progress, 1.0 - progress);
    float dist = steps>0 ? ceil(d * float(steps)) / float(steps) : d;
    vec2 squareSize = 2.0 * dist / vec2(squaresMin);

    vec4 transition(vec2 uv) {
        vec2 p = dist>0.0 ? (floor(uv / squareSize) + 0.5) * squareSize : uv;
        return mix(getFromColor(p), getToColor(p), progress);
    }

    void main() {
        gl_FragColor = transition(_uv);
    }
";