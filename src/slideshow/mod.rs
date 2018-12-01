
pub mod element;
pub mod transitions;

pub mod images;
//pub mod videos;

pub mod renderer;

use std::marker::PhantomData;
use gfx::{CommandBuffer, Encoder, Factory, Resources, Slice, CombinedError, PipelineStateError };


/*use videos::{
    VideoItem,
};*/

use images::{
    ImageItem,
};

use gfx::traits::FactoryExt;
use piston_window::Texture;

enum State {
    Pause,
    FromPauseToTransition,
    Transition,
    FromTransitionToPause,
}

pub enum SlideShowType {
    ImageSlide,
    //VideoSlide,
    ProgramSlide,
}

pub enum DrawableTypes<R: Resources> {
    Image(ImageItem<R>),
    //Video(VideoItem<R>),
}

pub struct System<R: Resources, F: Factory<R>  + Clone> {
    factory: F,
    items: Vec<Box<DrawableTypes<R>>>,
    current_position: usize,
    pub progress: f32,
    current_duration: f32,
    state: State,
    _r: PhantomData<R>,
}

impl<R: Resources, F: Factory<R> + Clone> System<R,F> {

    pub fn new(factory :  F) -> Self {
        System {
            factory: factory,
            items: Vec::new(),
            current_position: 0,
            progress: 0.0,
            current_duration: 0.0,
            state: State::Pause,
            _r: PhantomData,
        }
    }

    pub fn get_factory(&mut self) -> &mut F {
        &mut self.factory
    }

    pub fn load(&mut self, path: String, slide_type: SlideShowType) -> Result<(), &'static str> {
        let item: Box<DrawableTypes<R>> = match slide_type {
            SlideShowType::ImageSlide => {
                println!("load image: {:}", path);
                let mut i = images::ImageItem::new(&mut self.factory, path.to_string());
                i.load(&mut self.factory);
                Box::new(DrawableTypes::Image(i))
            },
            /*SlideShowType::VideoSlide => {
                println!("load video: {:}", path);
                let mut i = videos::VideoItem::new(&mut self.factory, path.to_string());
                i.load(&mut self.factory);
                Box::new(DrawableTypes::Video(i))
            },*/
            SlideShowType::ProgramSlide => {
                /*let i = videos::VideoItem::new(&mut self.factory,  path.to_string());
                Box::new(DrawableTypes::Video(i))*/
                let mut i = images::ImageItem::new(&mut self.factory, path.to_string());
                i.load(&mut self.factory);
                Box::new(DrawableTypes::Image(i))
            },
        };

        self.items.push(item);

        return Ok(())
    }

    pub fn multi_load(&mut self, value: &'static str) {
        use url::Url;
        use tempdir::TempDir;
        use std::io::copy;
        use std::fs::File;
        use std::collections::HashMap;

        match Url::parse(value) {
            Ok(httpurl) => {
                println!("Load as HTTP URL: {:}", value);
                // Load from Web
                let tmp_dir = TempDir::new("tmp_folder").unwrap();
                let mut response = reqwest::get(value).unwrap();
                let (mut dest, mut fname)= {
                    let url_name = response.url();
                    let hash_query: HashMap<_, _> = url_name.query_pairs().into_owned().collect();
                   
                    let fname = hash_query.get("name").unwrap();

                    println!("file to download: '{}'", fname);

                    let fname = tmp_dir.path().join(fname);

                    let raw_fname = format!("{}",fname.to_owned().display());
                    println!("will be located under: '{:?}'", fname);
                    (File::create(fname).unwrap(), raw_fname.to_owned())
                };

                copy(&mut response, &mut dest);
                self.load(fname, SlideShowType::ImageSlide);
            },
            Err(_) => {
                println!("Load as file {:}", value);
                // No web
                self.load(String::from(value), SlideShowType::ImageSlide);
            },
        };

    }

    pub fn tick<C: CommandBuffer<R>>(&mut self, encoder: &mut Encoder<R, C>) {
        for i in self.items.iter_mut() {
            i.tick(encoder);
        }
    }


    fn to_item(&mut self) -> Option<&Box<DrawableTypes<R>>> {
        if self.items.len() < 1 {
            return None;
        }
        
        let position = match self.current_position + 1 {
            p if p > self.items.len() => 0,
            p => p,
        };

        Some(&self.items[position])
        
    }

    fn from_item(&self) -> Option<&Box<DrawableTypes<R>>> {
        if self.items.len() < 1 {
            return None;
        }
        
        Some(&self.items[self.current_position])
    }

    fn next(&mut self) {
        self.current_position = match self.current_position + 1 {
            c if c > self.items.len() => 0,
            c => c,
        };
    }


    /// Update gets called in update
    pub fn update(&mut self, dt:f32, c: &mut renderer::Renderer<R,F>) {
        self.current_duration += dt;

        // get transition duration
        let transition_duration  = match self.to_item() {
            Some(i) => i.transition_duration(),
            None => 2.0,
        };
        
        let complete_duration = match self.from_item() {
            Some(i) => i.pause_duration() + i.transition_duration(),
            None => 2.0 + 5.0,
        };

        match self.state {
            State::Transition => {
                if self.current_duration > transition_duration {
                    self.state = State::FromTransitionToPause;
                } else {
                    self.progress = transition_duration / self.current_duration;
                }
                
            },
            State::FromTransitionToPause => {
                
                // go next frame
                self.next();

                self.progress = 0.0;
                
                //c.set_textures(, to: piston_window::Texture<R>)
                // switch texture,
                // set progress to 0,
                // put "from" slide to nothing,
                // put "to" slide to "from",
                // get next and put it to "from",
                self.state = State::Pause
            },
            State::Pause => {
                if self.current_duration > complete_duration {
                    self.state = State::FromPauseToTransition;
                }
            },
            State::FromPauseToTransition => {
                // something to do?
                // o here i think i can change the shader transition to a random one?
                self.state = State::Transition;
            },
        };

    }
}

pub trait Item
{
    // ein item muss folgende sachen immer haben:
    /**
     * a tick function to update frames if for ex a video
     * a get_texture function to retrive the texture
     * a 
     * */

    // simple refresh texture or does the frame playback thing on videos
    // should be called on windows.draw_3d or windows.draw_2d function call

    //
    fn in_view(&mut self, show: bool);

    /// Returns the duration in seconds of an transition from -> to
    fn transition_duration(&self) -> f32 {
        2.0
    }

    /// Return the duration of the pause
    fn pause_duration(&self) -> f32 {
        5.0
    }
}

pub trait Rendering<R: Resources> {
     fn tick<C: CommandBuffer<R>>(&mut self, encoder: &mut Encoder<R, C>) {
        // empty do nothing
        // does not need to be implemented
    }

    fn get_texture(&self) -> Option<Texture<R>>;

    // load item
    // for videos it should load all required things and should start the thread for playing the video
    fn load<F: Factory<R>>(&mut self, factory: &mut F);
}

impl<R: Resources> Item for DrawableTypes<R> {
    fn in_view(&mut self, show: bool) {
        match *self {
            DrawableTypes::Image(ref mut x) => x.in_view(show),
            //DrawableTypes::Video(ref mut x) => x.in_view(show),
        }
    }
}

impl<R: Resources> Rendering<R> for DrawableTypes<R> {
    fn tick<C: CommandBuffer<R>>(&mut self, encoder: &mut Encoder<R,C>)  {
        match *self {
            DrawableTypes::Image(ref mut x) => x.tick(encoder),
            //DrawableTypes::Video(ref mut x) => x.tick(encoder),
        }
    }

    fn get_texture(&self) -> Option<Texture<R>> {
        match *self {
            DrawableTypes::Image(ref x) => x.get_texture(),
            //DrawableTypes::Video(ref x) => x.get_texture(),
        }
    }

    fn load<F: Factory<R>>(&mut self, factory: &mut F) {
        match *self {
            DrawableTypes::Image(ref mut x) => x.load(factory),
            //DrawableTypes::Video(ref mut x) => x.load(factory),
        }
    }
}

