use crate::mutations::mutation::Mutation;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::FruityResult;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(FruityAny)]
#[export_struct]
pub struct MutationService {
    previous_mutations: Vec<Box<dyn Mutation>>,
    next_mutations: Vec<Box<dyn Mutation>>,
}

#[export_impl]
impl MutationService {
    pub fn new(_resource_container: ResourceContainer) -> Self {
        Self {
            previous_mutations: Vec::new(),
            next_mutations: Vec::new(),
        }
    }

    pub fn push_action(&mut self, mut mutation: impl Mutation + 'static) -> FruityResult<()> {
        mutation.apply()?;

        self.next_mutations.clear();
        self.previous_mutations.push(Box::new(mutation));

        Ok(())
    }

    #[export]
    pub fn undo(&mut self) -> FruityResult<()> {
        if let Some(mut mutation) = self.previous_mutations.pop() {
            mutation.undo()?;
            self.next_mutations.push(mutation);
        }

        Ok(())
    }

    #[export]
    pub fn redo(&mut self) -> FruityResult<()> {
        if let Some(mut mutation) = self.next_mutations.pop() {
            mutation.apply()?;
            self.previous_mutations.push(mutation);
        }

        Ok(())
    }

    #[export]
    pub fn can_undo(&self) -> bool {
        self.previous_mutations.len() > 0
    }

    #[export]
    pub fn can_redo(&self) -> bool {
        self.next_mutations.len() > 0
    }
}

impl Debug for MutationService {
    fn fmt(&self, _formatter: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}
