use std::path::Path;
use std::sync::{Arc, Mutex};
use gst::prelude::*;
use std::fs;
use std::thread;


pub struct AudioHandler {

    player: gst_player::Player,
    main_loop: glib::MainLoop,

}

impl AudioHandler {

    // ================== PUBLIC INTERFACE ==================
    pub fn new() -> Self{
        // Initialise GST stuff
        gst::init().unwrap();
        let main_loop = glib::MainLoop::new(None, false);
        let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
        let player = gst_player::Player::new(
            None,
            Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
        );

        let main_loop_clone = main_loop.clone();
            // Connect to the player's "end-of-stream" signal, which will tell us when the
            // currently played media stream reached its end.
            player.connect_end_of_stream(move |player| {
                let main_loop = &main_loop_clone;
                player.stop();
            });

        let main_loop_clone = main_loop.clone();
        thread::spawn(move || {
            
            main_loop_clone.run();
        });

        Self {
            player: player,
            main_loop: main_loop,
        }
    }

    pub fn play_music<P: AsRef<Path>>(&self, filename: P) {
        let filename = fs::canonicalize(filename).unwrap();
        let filename = filename.to_string_lossy();
        let uri = format!("file://{}", filename);

        // Tell the player what uri to play.
        self.player.set_uri(&uri);
        self.player.play();
    }

    pub fn play_music_with_sec_offset<P: AsRef<Path>>(filename: P, start_offset: u32) {
        unimplemented!();
    }

    pub fn pause_music(&self) {
        self.player.pause();
    }

    pub fn resume_music(&self) {
        self.player.play();
    }

    pub fn seek(seconds: u32) {
        unimplemented!();
    }

    // ======================================================

}