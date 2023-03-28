use fruity_game_engine::{profile_scope, FruityResult};

#[cfg(not(target_arch = "wasm32"))]
use std::thread;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

/// A system pool is a collection of systems that can be executed in parallel
pub trait SystemPool<System: Sync + Send + 'static>: Send + Sync + 'static {
    /// Add a system to the system pool
    fn add_system(&mut self, system: System);

    /// Get an iterator over the systems in the system pool
    fn iter(&self) -> Box<dyn Iterator<Item = &System> + '_>;

    /// Check if a system should be executed in the main thread
    fn is_main_thread_system(&self, system: &System) -> bool;

    /// Execute a system
    fn execute_system(&self, system: &System) -> FruityResult<()>;
}

impl<System: Sync + Send + 'static> dyn SystemPool<System> {
    /// Run all the systems in the system pool
    pub fn run_systems(&self) -> FruityResult<()> {
        let (main_thread_systems, parallel_systems): (Vec<_>, Vec<_>) = self
            .iter()
            .partition(|system| self.is_main_thread_system(system));

        #[cfg(not(target_arch = "wasm32"))]
        {
            profile_scope!("parallel_systems");

            thread::scope(|s| {
                let handler = s.spawn(move || {
                    parallel_systems
                        .into_iter()
                        // .par_bridge()
                        .try_for_each(|system| self.execute_system(system))
                });

                #[cfg(target_arch = "wasm32")]
                parallel_systems
                    .into_iter()
                    .try_for_each(execute_system.clone())?;

                // Run the main thread systems
                {
                    profile_scope!("main_thread_systems");
                    main_thread_systems
                        .into_iter()
                        .try_for_each(|system| self.execute_system(system))?;
                }

                // Wait all the threaded systems
                #[cfg(not(target_arch = "wasm32"))]
                {
                    profile_scope!("join_parallel_systems");
                    handler.join().unwrap()?;
                }

                Ok(())
            })
        }
    }
}
