// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::cmp;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if (self.health > 0)
        {
            return None;
        } else {
            let mana = if self.level >= 10 { Some(100 as u32) } else { None };

            return Some(Player{
                health: 100,
                mana: mana,
                level: self.level
             });
        }        
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if (self.level < 10)
        {
            self.health = std::cmp::max(0, self.health as i32 - mana_cost as i32) as u32;
            return 0 as u32;
        } else
        {
            return match self.mana {
                None => {0 as u32},
                Some(mana) => {
                    if mana>=mana_cost
                    {
                        self.mana = Some(mana-mana_cost);
                        return mana_cost * 2;
                    } else {
                        return 0 as u32;
                    }
                }
            };
        }
    }
}
