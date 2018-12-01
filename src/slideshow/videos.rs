use ffmpeg::{time, Rational, Error};
use ffmpeg;

use gfx::{
    Encoder,
    Resources,
    Factory,
    CommandBuffer,
};

use image::{RgbaImage, Rgba, GenericImage};
use piston_window::{
    Texture,
    TextureSettings,
    Filter,
};

//use imageproc::noise;

use extensions::video;

pub struct VideoItem<R: Resources> {
    texture: Option<Texture<R>>,
    video: Option<video::Video>,
    video_width: u32,
    video_height: u32,
    path: String,
    play: bool,
    stopped_at: f64,
}

impl<R: Resources> VideoItem<R> {
    
    // load an item
    // later the new function check if the item is an image or is an video
    // check if there is an url so we need to download the thing.. and so on...
    pub fn new<F: Factory<R>>(factory : &mut F, path: String) -> Self {
        VideoItem {
            path,
            texture: None,
            video_width: 800,
            video_height: 600,
            video: None,
            play: false,
            stopped_at: 0.0,
        }
    }

    pub fn load_video<F: Factory<R>>(&mut self, factory: &mut F) -> Result<(), Error> {
        // Do some video testing
        println!("load the video...");
        let v = video::spawn_video(self.path.as_str());
        self.video = match v {
            Err(error) => {
                println!("error: ffmpeg: {}", error);
                return Err(error);
            },

            Ok(v) =>
                v
        };

        let (video_width, video_height) = {
            if let Some(video) = self.video.as_ref() {
                let w = video.width() as u32;
                let h = video.height() as u32;

                println!("Video w:{:}, h:{:}", w, h);
                
                ( w, h )
            } else {
                ( 800, 600)
            }
        };

        self.video_width = video_width;
        self.video_height = video_height;


        // create texture for video and set size
        let mut texture_settings = TextureSettings::new();
        texture_settings.set_filter(Filter::Nearest);

        let mut i = RgbaImage::new(self.video_width, self.video_height);
        
        //noise::gaussian_noise_mut(&mut i, 32.0,12.0, 20);

        self.texture = Some(Texture::from_image(
            factory,
            &i,
            &texture_settings,
        ).unwrap());

        return Ok(());
    }


    pub fn start(&mut self) {
        //Synchronize start times?
        // Synchronize start times.
        let start = time::relative() as f64 / 1_000_000.0;

        if let Some(video) = self.video.as_mut() {
            println!("begin with the video...");
            video.start(start);
        }
    }

    /// pause the video add the time where pause function is called
    pub fn pause(&mut self) {
        self.stopped_at = time::relative() as f64;
    }

    /// resume the video so add the time left by now to the last pause
    pub fn resume(&mut self) {
        
        if self.stopped_at > 0.0 {
            if let Some(video) = self.video.as_mut() {
                println!("resume with the video...");
                video.resume((time::relative() as f64 - self.stopped_at) / 1_000_000.0);
                self.stopped_at = 0.0;
            }

        }
        
    }

}


use slideshow::Item;

impl<R: Resources> Item for VideoItem<R> {

    fn in_view(&mut self, show: bool) {
        self.play = show;
        if show == true {
            //println!("resume video");
            self.resume();
        } else {
            //println!("add pause to video");
            self.pause();
        }
    }
}

use slideshow::Rendering;
impl<R: Resources> Rendering<R> for VideoItem<R> {

  fn tick<C: CommandBuffer<R>>(&mut self, encoder: &mut Encoder<R, C>) {
        // Do Video synchronizing
        if let Some(video) = self.video.as_mut() {
            if self.play == true {
                video.sync();
            }
            
        };

        let frame = self.video.as_ref().and_then( |v| 
            if v.is_done() {
                None
            } else {
                Some(v.frame())
            }
        );

        if let Some(f) = frame {

            let p = f.data(0).to_vec();

            if let Some(img) = RgbaImage::from_raw(self.video_width, self.video_height, p) {
                //let flipped_img = flip_vertical(&img);
                //img.save("HALLO.png");
                if let Some(ref mut t) = self.texture {
                    if let Err(e) = t.update(
                        encoder,
                        &img,
                    ) {
                        println!("Error on update texture! err: {:?}", e);
                    }
                } else {
                    println!("no texture to update...");
                }
                
            }

        }
    }

    fn get_texture(&self) -> Option<Texture<R>> {
        if let Some(ref e) = self.texture {
            return Some(e.clone());
        }

        None
    }

    fn load<F: Factory<R>>(&mut self,  factory: &mut F) {
        self.load_video(factory);
        self.start();
    }

}