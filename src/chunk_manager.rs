// chunk_manager.rs
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::chunk::Chunk;
use crate::biomes::Biome;
use crate::world_gen::generate_chunk;

pub struct ChunkManager {
    // Store loaded chunks by their (x, y) offset
    pub loaded_chunks: HashMap<(i32, i32), Chunk>,
    
    // Channel to receive chunks from background threads
    receiver: Receiver<Chunk>,
    sender: Sender<Chunk>,
    
    // Track what is currently generating so we don't request it twice
    generating: HashMap<(i32, i32), bool>,
    
    world_seed: u32,
    current_biome: Biome,
}

impl ChunkManager {
    pub fn new(seed: u32) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            loaded_chunks: HashMap::new(),
            receiver,
            sender,
            generating: HashMap::new(),
            world_seed: seed,
            current_biome: Biome::default(),
        }
    }

    // Call this when the player moves or when an NPC needs a distant chunk
    pub fn request_chunk(&mut self, chunk_x: i32, chunk_y: i32) {
        let key = (chunk_x, chunk_y);
        
        // If we already have it or are already making it, do nothing
        if self.loaded_chunks.contains_key(&key) || self.generating.contains_key(&key) {
            return;
        }

        self.generating.insert(key, true);

        // Clone data needed for the thread
        let tx = self.sender.clone();
        let biome = self.current_biome;
        let seed = self.world_seed;

        // Spawn a background thread to do the heavy math
        thread::spawn(move || {
            let chunk = generate_chunk(chunk_x, chunk_y, &biome, seed);
            // Send the completed data array back to the main thread
            let _ = tx.send(chunk); 
        });
    }

    // Call this once per frame in your main loop to catch incoming chunks
    pub fn process_incoming_chunks(&mut self) {
        // try_recv() won't block the frame if no chunk is ready
        while let Ok(chunk) = self.receiver.try_recv() {
            let key = (chunk.chunk_offset_x, chunk.chunk_offset_y);
            self.generating.remove(&key);
            self.loaded_chunks.insert(key, chunk);
        }
    }
    
    // Unload chunks that are too far from the player
    pub fn cleanup_distant_chunks(&mut self, player_chunk_x: i32, player_chunk_y: i32, radius: i32) {
        self.loaded_chunks.retain(|&(cx, cy), _| {
            (cx - player_chunk_x).abs() <= radius && (cy - player_chunk_y).abs() <= radius
        });
    }
}