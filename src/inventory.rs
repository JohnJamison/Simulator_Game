#[derive(Clone, Copy, PartialEq)]
pub enum Item { Berry, Twig, FishingRod, Fish }

impl Item {
    pub fn weight(&self) -> u32 { 
        match self { Item::Berry => 1, Item::Twig => 1, Item::FishingRod => 2, Item::Fish => 3 } 
    }
    pub fn name(&self) -> &str { 
        match self { Item::Berry => "Fresh Berry", Item::Twig => "Twig", Item::FishingRod => "Fishing Rod", Item::Fish => "Raw Fish" } 
    }
    pub fn nutrition(&self) -> f32 { 
        match self { Item::Berry => 15.0, Item::Twig => 0.0, Item::FishingRod => 0.0, Item::Fish => 35.0 } 
    }
}

pub struct Inventory {
    pub slots: Vec<(Item, u32)>, 
    pub max_capacity: u32,
    pub equipped: Option<Item>,
}

impl Inventory {
    pub fn new() -> Self { Inventory { slots: Vec::new(), max_capacity: 30, equipped: None } }
    
    pub fn current_weight(&self) -> u32 {
        let mut total = 0;
        for (item, count) in &self.slots { total += item.weight() * count; }
        total
    }

    pub fn try_add_item(&mut self, item: Item) -> bool {
        if self.current_weight() + item.weight() <= self.max_capacity {
            for slot in &mut self.slots {
                if slot.0 == item { slot.1 += 1; return true; }
            }
            self.slots.push((item, 1));
            true
        } else { false }
    }
    
    pub fn consume_item(&mut self, index: usize) -> Option<Item> {
        if index < self.slots.len() {
            let item = self.slots[index].0;
            self.slots[index].1 -= 1;
            if self.slots[index].1 == 0 { self.slots.remove(index); }
            Some(item)
        } else { None }
    }

    pub fn count_item(&self, item_type: Item) -> u32 {
        for slot in &self.slots {
            if slot.0 == item_type { return slot.1; }
        }
        0
    }

    pub fn remove_items(&mut self, item_type: Item, amount: u32) {
        let mut remaining = amount;
        for i in (0..self.slots.len()).rev() {
            if self.slots[i].0 == item_type {
                if self.slots[i].1 > remaining {
                    self.slots[i].1 -= remaining;
                    return;
                } else {
                    remaining -= self.slots[i].1;
                    self.slots.remove(i);
                }
            }
        }
    }
}