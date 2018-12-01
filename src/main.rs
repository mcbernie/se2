//extern crate piston;
extern crate piston_window;

use gfx::traits::*;

extern crate vecmath;
//extern crate camera_controllers;
extern crate gfx_device_gl;
extern crate gfx_texture;
extern crate image;
extern crate imageproc;

extern crate url;
extern crate reqwest;
extern crate tempdir;

//#[cfg(feature="piston")] #[macro_use] 
//extern crate conrod;

#[macro_use]
extern crate gfx;

extern crate shader_version;
#[macro_use] extern crate log;

#[macro_use]
mod util;

/*use util::pso::{
    WatcherPsoCellBuilder,
    PsoCell,
};*/

//extern crate freetype;
//mod gfx_text;

//use std::path::{Path, PathBuf};
/*use piston_window::{
    //PistonWindow, 
    UpdateEvent, 
    Window, 
    //WindowSettings,
    //GenericEvent,
};*/

//extern crate nalgebra as na;
//use na::{Orthographic3, Point3, Vector3, Isometry3};

//extern crate ffmpeg;

mod extensions;
mod slideshow;
use slideshow::{
    Item,
    element,
    transitions,
    images,
    //videos,
};


//----------------------------------------
// Cube associated data

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


gfx_pipeline!( background_pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    aspect: gfx::Global<f32> = "aspect",
    scale: gfx::Global<[f32; 2]> = "scale",
    offset: gfx::Global<[f32; 2]> = "offset",
    //colored_noise: gfx::Global<Bool> = "coloredNoise",
    smoothing: gfx::Global<[f32; 2]> = "smoothing",
    noise_alpha: gfx::Global<f32> = "noiseAlpha",
    color1: gfx::Global<[f32; 3]> = "color1",
    color2: gfx::Global<[f32; 3]> = "color2",
    out_color: gfx::RenderTarget<::gfx::format::Rgba8> = "o_Color",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> = gfx::preset::depth::LESS_EQUAL_WRITE,
});

//----------------------------------------



fn main() {
    //ffmpeg::init().unwrap();
    use piston_window::*;
    use gfx::traits::*;

    
    // setup up the window and opengl
    // Sdl2Window
    let opengl = OpenGL::V2_1;

    let mut window: PistonWindow = WindowSettings::new("SE_2", [1280, 768])
        .exit_on_esc(true)
        //.samples(4)
        .opengl(opengl)
        .build()
        .unwrap();
    window.set_capture_cursor(false);


    let ref mut factory = window.factory.clone();
    
    
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

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice
        (&vertex_data, index_data);


    let mut texture_settings = TextureSettings::new();
    texture_settings.set_filter(Filter::Nearest);

    // Setup Slideshow renderer...
    let renderer_factory = window.factory.clone();
    let  render_builder = slideshow::renderer::new(renderer_factory);

    let mut renderer = render_builder.with_ratio(1.0).build().expect("error on building slideshow renderer!");



    //let ref mut live_long_factory = window.factory.clone();
    
    let ref mut system = slideshow::System::new(window.factory.clone());
    //system.load(String::from("assets/images/index.php-3.jpeg"), slideshow::SlideShowType::ImageSlide);
    //system.load(String::from("assets/video/tr5_event_bally.mp4"), slideshow::SlideShowType::VideoSlide);
    //system.load(String::from("assets/images/index.php-4.jpeg"), slideshow::SlideShowType::ImageSlide);
    system.multi_load("http://wacogmbh.de:3999/index.php?m=fb&o=image&name=med_1542113613_75980200.jpg");
    system.multi_load("http://wacogmbh.de:3999/index.php?m=fb&o=image&name=med_1540822504_183929.jpg");
    /*system.multi_load("http://wacogmbh.de:3999/index.php?m=fb&o=image&name=med_1540822519_175702.jpg");
    system.multi_load("http://wacogmbh.de:3999/index.php?m=fb&o=image&name=med_1540822560_393374.jpg");
    system.multi_load("http://wacogmbh.de:3999/index.php?m=fb&o=image&name=med_1540822582_358426.jpg");
    system.multi_load("http://wacogmbh.de:3999/index.php?m=fb&o=image&name=med_1540822599_986628.jpg");
    system.multi_load("http://wacogmbh.de:3999/index.php?m=fb&o=image&name=med_1540822614_260791.jpg");*/

    //let default_texture = system.get_from_texture();

    //let from_r:f32 = default_texture.get_width() as f32 / default_texture.get_height() as f32;
   // let to_r:f32 = default_texture.get_width() as f32 / default_texture.get_height() as f32;

    let draw_size = window.draw_size();
    //let ratio = draw_size.width as f32 / draw_size.height as f32;

    let radius = draw_size.width as f32 * 1.05;

    println!("HERE5");
    let mut background = background_pipe::Data {
        vbuf: vbuf.clone(),
        u_model_view_proj: [[0.0; 4]; 4],
        out_color: window.output_color.clone(),
        out_depth: window.output_stencil.clone(),
        scale: [1.0 / draw_size.width as f32 * radius, 1.0 / draw_size.height as f32 * radius],
        aspect: 1.0,

        color1: [0.95, 0.95, 0.95 ],
        color2: [60.0 / 255.0 , 76.0 / 255.0, 88.0 / 255.0 ],
        smoothing: [ -0.5, 1.0 ],
        noise_alpha: 0.12,
        //colored_noise: true,
        offset: [-0.05, -0.15],
    };
    
    println!("HERE2");
    let vertex_shader = util::pso::shader_bytes(&String::from("./assets/default.glslv")).unwrap();
    let fragment_shader = util::pso::shader_bytes(&String::from("./assets/background.glslf")).unwrap();

    println!("HERE3");
    let background_pso = factory.create_pipeline_simple( // defined in `gfx::traits::FactoryExt`
        &vertex_shader, &fragment_shader,
        background_pipe::new()
    ).unwrap();



    // set perspective
    /*let eye        = Point3::new(0.0, 0.0, 1.0);
    let target     = Point3::new(1.0, 0.0, 0.0);
    let view       = Isometry3::look_at_rh(&eye, &target, &Vector3::y());
    let model      = Isometry3::new(Vector3::x(), na::zero());
    let projection = Orthographic3::new(-1., 1., -1., 1., -1., 1.);
    let model_view_projection = projection.unwrap() * (view * model).to_homogeneous();*/


    //font stuff for testing
    /*let assets = Path::new("./assets/fonts");
    let font = assets.join("leaguegothic-regular-webfont.ttf");*/
    /*let font_factory = window.factory.clone();
    let mut text = gfx_text::new(font_factory)
        .with_size(37)
        .with_font("./assets/fonts/leaguegothic-regular-webfont.ttf")
        .build()
        .expect("Error on creating text");*/

    //text.add("Star Entertainer System 2018", [100,100], [0.0, 1.0, 0.0, 1.0]);
    // all the things for the transition animations
    let mut current = 0;
    let mut pause:bool = true;
    let mut current_duration:f32 = 0.0;
    let mut progress:f32 = 1.0;
    let mut start_up_time = 0.0;
    let mut show_slideshow = false;

    println!("HERE1");

    while let Some(e) =window.next() {

        let size = window.size();

        // do some Transition for slideshow
        if let Some(u) = e.update_args() {
            
            system.update(u.dt as f32, &mut renderer);

            if show_slideshow == false {
                 start_up_time += u.dt as f32;
            }

            if start_up_time > 3.0 {
                show_slideshow = true;
            }

        }

        window.draw_3d(&e, |window| {

            window.encoder.clear(&window.output_color, [1.0, 1.0, 1.0, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);
            println!("HERE10");
            window.encoder.draw(&slice,&background_pso, &background); // Background image..
            println!("HERE11");
            //data.u_model_view_proj = *model_view_projection.as_ref();
            if show_slideshow == true {
                system.tick(&mut window.encoder);


                if let Err(e) = renderer.draw(
                    &mut window.encoder, 
                    &window.output_color,
                    system.progress,
                ) {
                    println!("Error on Draw Slideshow elements: {:?}", e);
                }

                //window.encoder.draw(&slice, watcher.pso(), &data);
            } else {
                /*text.add_anchored("Star Entertainer System 2018", [(size.width / 2) as i32,(size.height / 2) as i32], gfx_text::HorizontalAnchor::Center, gfx_text::VerticalAnchor::Center, [0.0, 0.0, 0.8, 1.0]);
                if start_up_time < 2.0 {
                    text.add_anchored("System wird gestartet...", [(size.width / 2) as i32,((size.height / 2) + 50) as i32], gfx_text::HorizontalAnchor::Center, gfx_text::VerticalAnchor::Center, [0.0, 0.0, 0.8, 1.0]);
                } else {
                    if start_up_time >= 2.0 && start_up_time < 2.5 {
                         text.add_anchored("System wird gestartet...", [(size.width / 2) as i32,((size.height / 2) + 50) as i32], gfx_text::HorizontalAnchor::Center, gfx_text::VerticalAnchor::Center, [0.0, 0.0, 0.8, 1.0 - ((start_up_time - 2.0) * 2.0)]);
                    } else {
                        text.add_anchored("System ist bereit", [(size.width / 2) as i32,((size.height / 2) + 50) as i32], gfx_text::HorizontalAnchor::Center, gfx_text::VerticalAnchor::Center, [0.0, 0.0, 0.8, 1.0]);
                    }
                }
                text.draw(&mut window.encoder, &window.output_color).unwrap();*/
            }


           

        });

        /*window.draw_2d(&e, |context, gfx| {

        });*/

        if let Some(_) = e.resize_args() {
            //data.out_color = window.output_color.clone();
            //data.out_depth = window.output_stencil.clone();
            background.out_color = window.output_color.clone();
            background.out_depth = window.output_stencil.clone();
            
            let radius = size.width as f32 * 1.05;
            background.scale =  [1.0 / size.width as f32 * radius, 1.0 / size.height as f32 * radius];

        }
    }
}
