// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let level = self.level;
        match self.health {
            
            0 => {
                let health = 100;
                match self.level {
                    l if l < 10 => Some(Self {
                        health,
                        level,
                        mana: None
                    }),
                    _ => Some(Self {
                        health,
                        level,
                        mana: Some(100)
                    })
                }
                
            },
            _ => None
                     
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(m) => { 
                let rem = if m <= mana_cost {
                    0
                } else {
                    m - mana_cost * 2
                };
                self.mana = Some(rem);
                rem
            },
            None => {
                self.health = if self.health <= mana_cost {
                    0
                } else {
                    self.health - mana_cost
                };
                0
            }
        }
    }
}


pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = magazine.clone().to_vec();
    // let map = std::collections::HashMap::new();

    for w in note {
        if let Some(ind) = words.iter().position(|i| i == w) {
            words.remove(ind);
        } else {
            return false
        }
    }
    true
}