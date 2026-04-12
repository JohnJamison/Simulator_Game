// =====================================================================
//   BIOME CONFIGURATION
// =====================================================================
pub struct Biome {

    // Water Generation
    pub water_scale: f64,       // Controls sporadic nature (Higher = more scattered puddles)
    pub water_threshold: f64,   // Controls total water volume (Higher = flooded world)
    
    // Cliff Generation
    pub cliff_scale: f64,       // Controls mountain width/spread (Lower = wider mountain ranges)
    pub cliff_threshold: f64,   // Controls mountain frequency (Lower = more mountains)
    pub cliff_layers: u8,
    pub cliff_spacing: f64,
    
    // Ramp Generation
    pub ramp_chance: f32,
    pub min_ramp_width: usize,
    pub max_ramp_width: usize,
}

impl Biome {
    pub fn default() -> Self {
        Self {
            water_scale: 0.03,
            water_threshold: -0.3,
            cliff_scale: 0.05,
            cliff_threshold: 0.1,
            cliff_layers:  3,
            cliff_spacing: 0.3,
            ramp_chance: 0.4,
            min_ramp_width: 2,
            max_ramp_width: 14,
        }
    }

    pub fn flat_mountains() -> Self {
        Self {
            water_scale: 0.03,
            water_threshold: -0.3,
            cliff_scale: 0.05,
            cliff_threshold: -0.9,
            cliff_layers:  3,
            cliff_spacing: 0.3,
            ramp_chance: 0.4,
            min_ramp_width: 2,
            max_ramp_width: 14,
        }
    }
}