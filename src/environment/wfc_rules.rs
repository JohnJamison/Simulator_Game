use std::collections::HashMap;
use crate::environment::{Tile, Direction};
use crate::environment::water::WaterVariant;

type JsonRules = HashMap<String, HashMap<String, Vec<String>>>;

pub struct RuleRegistry {
    pub matrix: HashMap<(Tile, Direction), Vec<Tile>>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        // 1. Load the JSON
        let json_data = include_str!("rules.json"); 
        let parsed: JsonRules = serde_json::from_str(json_data).expect("Failed to parse rules.json");
        let mut matrix: HashMap<(Tile, Direction), Vec<Tile>> = HashMap::new();
        
        // HELPER: Inserts a rule only if it doesn't already exist
        let mut insert_rule = |from: Tile, dir: Direction, to: Tile| {
            let entry = matrix.entry((from, dir)).or_default();
            if !entry.contains(&to) {
                entry.push(to);
            }
        };

        // 2. Parse JSON and Auto-Symmetrize!
        for (tile_str, direction_map) in parsed {
            let current_tile = Self::str_to_tile(&tile_str);
            for (dir_str, allowed_list) in direction_map {
                let dir = Self::str_to_dir(&dir_str);
                
                for allowed_str in allowed_list {
                    let allowed_tile = Self::str_to_tile(&allowed_str);
                    
                    // FORWARD RULE (From the JSON)
                    insert_rule(current_tile, dir, allowed_tile);
                    
                    // AUTO-REVERSE RULE (Prevents Deadlocks!)
                    // If Water allows Grass to the North, Grass MUST allow Water to the South.
                    insert_rule(allowed_tile, dir.opposite(), current_tile);
                }
            }
        }

        // 3. AUTO-LAND RULES
        // Land tiles can always freely touch each other. No need to put this in JSON!
        let land_tiles = [Tile::Grass, Tile::Bush, Tile::BerryBush, Tile::TreeTrunk];
        let dirs = [Direction::North, Direction::South, Direction::East, Direction::West];
        
        for land in &land_tiles {
            for dir in &dirs {
                for other_land in &land_tiles {
                    insert_rule(*land, *dir, *other_land);
                }
            }
        }

        RuleRegistry { matrix }
    }

    fn str_to_tile(s: &str) -> Tile {
        match s {
            "Grass" | "Empty" => Tile::Grass, // Accept both just in case!
            "Bush" => Tile::Bush,
            "BerryBush" => Tile::BerryBush,
            "TreeTrunk" => Tile::TreeTrunk,
            "Center" => Tile::Water(WaterVariant::Center),
            "Top" => Tile::Water(WaterVariant::Top),
            "TopLeft" => Tile::Water(WaterVariant::TopLeft),
            "TopRight" => Tile::Water(WaterVariant::TopRight),
            "Right" => Tile::Water(WaterVariant::Right),
            "BottomRight" => Tile::Water(WaterVariant::BottomRight),
            "Bottom" => Tile::Water(WaterVariant::Bottom),
            "BottomLeft" => Tile::Water(WaterVariant::BottomLeft),
            "Left" => Tile::Water(WaterVariant::Left),
            "InverseCornerTR" => Tile::Water(WaterVariant::InverseCornerTR),
            "InverseCornerTL" => Tile::Water(WaterVariant::InverseCornerTL),
            "InverseCornerBL" => Tile::Water(WaterVariant::InverseCornerBL),
            "InverseCornerBR" => Tile::Water(WaterVariant::InverseCornerBR),
            _ => panic!("Unknown tile requested in JSON: {}", s),
        }
    }

    fn str_to_dir(s: &str) -> Direction {
        match s {
            "North" => Direction::North,
            "East" => Direction::East,
            "South" => Direction::South,
            "West" => Direction::West,
            _ => panic!("Unknown direction requested in JSON: {}", s),
        }
    }
}