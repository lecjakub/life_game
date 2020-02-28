use crate::life_game::*;

#[derive(Clone)]
pub struct Cell {
    alive: bool,
    next_life_state: LifeState,
}

impl Cell {
    pub fn new(alive: bool) -> Cell {
        Cell {
            alive,
            next_life_state: LifeState::None,
        }
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn set_life(&mut self, alive: bool) {
        self.alive = alive;
    }

    pub fn set_next_state(&mut self, state:LifeState){
        self.next_life_state = state;
    }
    pub fn update_state(&mut self){
        match self.next_life_state{
            LifeState::Revive | LifeState::Survive => self.alive = true,
            LifeState::Die => self.alive = false,
            _ => {}
        }
    }
}
