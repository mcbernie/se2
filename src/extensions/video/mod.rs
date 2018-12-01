
use std::sync::mpsc::{SyncSender, sync_channel};
use std::thread;

use ffmpeg::{format, media, Error, Packet};

pub mod decoder;
pub use self::decoder::Decoder;

pub mod video;
pub use self::video::Video;

pub const FRAMES: usize = 8;
pub const PACKETS: usize = 64;

pub enum Reader {
    /// A new incoming Packet
    Packet(Packet),

	/// The EOF packet.
	///
	/// Note that the sender is sent along so the receiver will be able to
	/// receive all still-standing incoming packets.
    End(SyncSender<Reader>),
}

pub fn spawn_video(path: &str) -> Result<Option<Video>, Error> {
    let path = path.to_owned();
    
    let (video_sender, video_receiver) = sync_channel(FRAMES);

    thread::spawn(move || {
        
        let mut context = match format::input(&path) {
            Ok(context) => 
                context,

            Err(error) => {
                Video::error(&video_sender, error);

                return;
            }
        };

        // Spawn the video decoder

        let  video = match context.streams().find(|s| s.codec().medium() == media::Type::Video) {
            Some(ref stream) => {
                let codec = match stream.codec().decoder().video() {
                    Ok(codec) => 
                        codec,
                    Err(error) => {
                        Video::error(&video_sender, error);
                        return;
                    }
                };

                Some((Video::spawn(codec, &stream, video_sender), stream.index()))
            },

            _ => {
                Video::none(&video_sender);

                None
            }
        };

        for (stream, packet) in context.packets() {
            if let Some((ref channel, index)) = video {
                if stream.index() == index {
                    ret!(channel.send(Reader::Packet(packet.clone())));
                }
            }
        }

        if let Some((ref channel, _)) = video {
            ret!(channel.send(Reader::End(channel.clone())));
        }

    });

    let video = match video_receiver.recv().unwrap() {
        Decoder::Start(None) => 
            Ok(None),

        Decoder::Start(Some(details)) => 
            Ok(Some(Video::new(video_receiver, details))),

        Decoder::Error(error) => 
            Err(error),

        _ => 
            Err(Error::Bug),
    };

    video

}