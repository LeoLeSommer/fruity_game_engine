use fruity_game_engine::any::FruityAny;
use fruity_game_engine::inject::Inject;
use fruity_game_engine::profile_scope;
use fruity_game_engine::resource::ResourceContainer;
use fruity_game_engine::sync::Arc;
use fruity_game_engine::sync::Mutex;
use fruity_game_engine::FruityResult;
use fruity_game_engine::{export, export_impl, export_struct};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

#[cfg(not(target_arch = "wasm32"))]
use std::thread;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

mod system_pool;
pub use system_pool::*;

/// A callback for a system called every frame
pub type SystemCallback = dyn Fn() -> FruityResult<()> + Send + Sync + 'static;

/// A callback for a startup system dispose callback
pub type StartupDisposeSystemCallback =
    Option<Box<dyn FnOnce() -> FruityResult<()> + Send + Sync + 'static>>;

/// A callback for a startup system
pub type StartupSystemCallback =
    dyn Fn() -> FruityResult<StartupDisposeSystemCallback> + Send + Sync + 'static;

/// Params for a system
#[derive(Debug, Clone, FruityAny, Default)]
#[export_struct(from_raw_js_object = true)]
pub struct SystemParams {
    /// The pool index
    pub pool_index: Option<usize>,

    /// If true, the system is still running while pause
    pub ignore_pause: Option<bool>,

    /// If true, the system will be executed in the main thread
    pub execute_in_main_thread: Option<bool>,
}

#[export_impl]
impl SystemParams {}

/// Params for a system
#[derive(Debug, Clone, FruityAny, Default)]
#[export_struct(from_raw_js_object = true)]
pub struct StartupSystemParams {
    /// If true, the system is still running while pause
    pub ignore_pause: Option<bool>,

    /// If true, the system will be executed in the main thread
    pub execute_in_main_thread: Option<bool>,
}

#[export_impl]
impl StartupSystemParams {}

struct StartupDisposeSystem {
    identifier: String,
    callback: Box<dyn FnOnce() -> FruityResult<()> + Send + Sync + 'static>,
    execute_in_main_thread: bool,
}

/// System service
///
#[derive(FruityAny)]
#[export_struct]
pub struct SystemService {
    pause: Arc<AtomicBool>,
    system_pools: BTreeMap<usize, FrameSystemPool>,
    startup_systems: StartupSystemPool,
    startup_pause_systems: StartupSystemPool,
    startup_dispose_callbacks: Arc<Mutex<StartupDisposeSystemPool>>,
    startup_pause_dispose_callbacks: Arc<Mutex<StartupDisposeSystemPool>>,
    resource_container: ResourceContainer,
}

impl Debug for SystemService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[export_impl]
impl SystemService {
    /// Returns a SystemService
    pub fn new(resource_container: ResourceContainer) -> SystemService {
        let pause = Arc::new(AtomicBool::new(false));
        let startup_dispose_callbacks = Arc::new(Mutex::new(StartupDisposeSystemPool {
            systems: Vec::new(),
        }));
        let startup_pause_dispose_callbacks = Arc::new(Mutex::new(StartupDisposeSystemPool {
            systems: Vec::new(),
        }));

        SystemService {
            pause,
            system_pools: BTreeMap::new(),
            startup_systems: StartupSystemPool {
                systems: Default::default(),
                dispose_callbacks: startup_dispose_callbacks.clone(),
            },
            startup_pause_systems: StartupSystemPool {
                systems: Default::default(),
                dispose_callbacks: startup_pause_dispose_callbacks.clone(),
            },
            startup_dispose_callbacks,
            startup_pause_dispose_callbacks,
            resource_container,
        }
    }

    /// Add a system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_system<T: Inject<FruityResult<()>>>(
        &mut self,
        identifier: &str,
        callback: T,
        params: Option<SystemParams>,
    ) {
        self.add_boxed_system(
            identifier,
            callback.inject(&self.resource_container).into(),
            params,
        )
    }

    /// Add a system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    #[export(name = "add_system")]
    pub fn add_script_system(
        &mut self,
        identifier: String,
        callback: Box<dyn Send + Sync + Fn() -> FruityResult<()>>,
        params: Option<SystemParams>,
    ) {
        self.add_boxed_system(
            identifier.as_str(),
            callback,
            Some(
                params
                    .map(|params| SystemParams {
                        execute_in_main_thread: Some(true),
                        ..params
                    })
                    .unwrap_or(SystemParams {
                        execute_in_main_thread: Some(true),
                        ..Default::default()
                    }),
            ),
        )
    }

    /// Add a system to the collection
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_boxed_system(
        &mut self,
        identifier: &str,
        system: Box<dyn Send + Sync + Fn() -> FruityResult<()>>,
        params: Option<SystemParams>,
    ) {
        let params = params.unwrap_or_default();
        let system = FrameSystem {
            identifier: identifier.to_string(),
            system,
            ignore_pause: params.ignore_pause.unwrap_or(false),
            execute_in_main_thread: params.execute_in_main_thread.unwrap_or(false),
        };

        if let Some(pool) = self.system_pools.get_mut(&params.pool_index.unwrap_or(50)) {
            pool.add_system(system)
        } else {
            // If the pool not exists, we create it
            let systems = vec![system];
            self.system_pools.insert(
                params.pool_index.unwrap_or(50),
                FrameSystemPool {
                    pause: self.pause.clone(),
                    systems,
                    enabled: true,
                },
            );
        };
    }

    /// Add a startup system
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_startup_system<T: Inject<FruityResult<StartupDisposeSystemCallback>>>(
        &mut self,
        identifier: &str,
        callback: T,
        params: Option<StartupSystemParams>,
    ) {
        self.add_arc_startup_system(
            identifier,
            callback.inject(&self.resource_container).into(),
            params,
        );
    }

    /// Add a startup system
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    #[export(name = "add_startup_system")]
    pub fn add_script_startup_system(
        &mut self,
        identifier: String,
        callback: Box<
            dyn Send
                + Sync
                + Fn() -> FruityResult<
                    Option<Box<dyn FnOnce() -> FruityResult<()> + Send + Sync + 'static>>,
                >,
        >,
        params: Option<StartupSystemParams>,
    ) {
        self.add_arc_startup_system(
            identifier.as_str(),
            callback,
            Some(
                params
                    .map(|params| StartupSystemParams {
                        execute_in_main_thread: Some(true),
                        ..params
                    })
                    .unwrap_or(StartupSystemParams {
                        execute_in_main_thread: Some(true),
                        ..Default::default()
                    }),
            ),
        )
    }

    /// Add a startup system
    ///
    /// # Arguments
    /// * `system` - A function that will compute the world
    /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
    ///
    pub fn add_arc_startup_system(
        &mut self,
        identifier: &str,
        callback: Box<
            dyn Send
                + Sync
                + Fn() -> FruityResult<
                    Option<Box<dyn FnOnce() -> FruityResult<()> + Send + Sync + 'static>>,
                >,
        >,
        params: Option<StartupSystemParams>,
    ) {
        let params = params.unwrap_or_default();

        if params.ignore_pause.unwrap_or(false) {
            self.startup_systems.add_system(StartupSystem {
                identifier: identifier.to_string(),
                system: callback,
                execute_in_main_thread: params.execute_in_main_thread.unwrap_or(false),
            });
        } else {
            self.startup_pause_systems.add_system(StartupSystem {
                identifier: identifier.to_string(),
                system: callback,
                execute_in_main_thread: params.execute_in_main_thread.unwrap_or(false),
            });
        }
    }

    /// Run all the stored systems
    pub fn run_frame(&self) -> FruityResult<()> {
        profile_scope!("frame_systems");

        let was_paused = self.is_paused();

        self.system_pools.iter().try_for_each(|(_, pool)| {
            if pool.enabled {
                (pool as &dyn SystemPool<FrameSystem>).run_systems()?;
            }

            FruityResult::Ok(())
        })?;

        // Run pause/unpause systems if needed
        if !was_paused && self.is_paused() {
            self.run_unpause_start()?;
        }

        if was_paused && !self.is_paused() {
            self.run_unpause_end()?;
        }

        Ok(())
    }

    /// Run all the startup systems
    pub fn run_start(&self) -> FruityResult<()> {
        profile_scope!("start_systems");

        (&self.startup_systems as &dyn SystemPool<StartupSystem>).run_systems()?;

        if !self.is_paused() {
            (&self.startup_pause_systems as &dyn SystemPool<StartupSystem>).run_systems()?;
        }

        Result::Ok(())
    }

    /// Run all startup dispose callbacks
    pub fn run_end(&self) -> FruityResult<()> {
        profile_scope!("end_systems");

        if !self.is_paused() {
            self.run_unpause_end()?;
        }

        let mut startup_dispose_callbacks = self.startup_dispose_callbacks.lock();
        startup_dispose_callbacks.run_systems()
    }

    /// Run all the startup systems that start when pause is stopped
    fn run_unpause_start(&self) -> FruityResult<()> {
        (&self.startup_pause_systems as &dyn SystemPool<StartupSystem>).run_systems()
    }

    /// Run all the startup dispose callbacks of systems that start when pause is stopped
    fn run_unpause_end(&self) -> FruityResult<()> {
        let mut startup_dispose_callbacks = self.startup_pause_dispose_callbacks.lock();
        startup_dispose_callbacks.run_systems()
    }

    /// Enable a pool
    pub fn enable_pool(&mut self, index: usize) {
        if let Some(pool) = self.system_pools.get_mut(&index) {
            pool.enabled = true;
        }
    }

    /// Disable a pool
    pub fn disable_pool(&mut self, index: usize) {
        if let Some(pool) = self.system_pools.get_mut(&index) {
            pool.enabled = false;
        }
    }

    /// Is systems paused
    #[export]
    pub fn is_paused(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }

    /// Set if systems are paused, only systems that ignore pause will be executed
    ///
    /// # Arguments
    /// * `paused` - The paused value
    ///
    #[export]
    pub fn set_paused(&self, paused: bool) -> FruityResult<()> {
        self.pause.store(paused, Ordering::Relaxed);
        Ok(())
    }
}

struct FrameSystem {
    identifier: String,
    system: Box<SystemCallback>,
    ignore_pause: bool,
    execute_in_main_thread: bool,
}

/// A system pool, see [‘SystemService‘] for more informations
struct FrameSystemPool {
    pause: Arc<AtomicBool>,

    /// Systems of the pool
    systems: Vec<FrameSystem>,

    /// Is the pool enabled
    enabled: bool,
}

impl SystemPool<FrameSystem> for FrameSystemPool {
    fn add_system(&mut self, system: FrameSystem) {
        self.systems.push(system)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &FrameSystem> + '_> {
        if self.pause.load(Ordering::Relaxed) {
            Box::new(self.systems.iter().filter(|system| system.ignore_pause))
        } else {
            Box::new(self.systems.iter())
        }
    }

    fn is_main_thread_system(&self, system: &FrameSystem) -> bool {
        system.execute_in_main_thread
    }

    fn execute_system(&self, system: &FrameSystem) -> FruityResult<()> {
        profile_scope!(&system.identifier);
        (system.system)()
    }
}

struct StartupSystem {
    identifier: String,
    system: Box<StartupSystemCallback>,
    execute_in_main_thread: bool,
}

struct StartupSystemPool {
    systems: Vec<StartupSystem>,
    dispose_callbacks: Arc<Mutex<StartupDisposeSystemPool>>,
}

impl SystemPool<StartupSystem> for StartupSystemPool {
    fn add_system(&mut self, system: StartupSystem) {
        self.systems.push(system)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &StartupSystem> + '_> {
        Box::new(self.systems.iter())
    }

    fn is_main_thread_system(&self, system: &StartupSystem) -> bool {
        system.execute_in_main_thread
    }

    fn execute_system(&self, system: &StartupSystem) -> FruityResult<()> {
        profile_scope!(&system.identifier);
        let dispose = (system.system)()?;

        if let Some(dispose) = dispose {
            let mut dispose_callbacks = self.dispose_callbacks.lock();
            dispose_callbacks.add_system(StartupDisposeSystem {
                identifier: system.identifier.clone(),
                callback: dispose,
                execute_in_main_thread: system.execute_in_main_thread,
            });
        }

        Ok(())
    }
}

struct StartupDisposeSystemPool {
    systems: Vec<StartupDisposeSystem>,
}

impl StartupDisposeSystemPool {
    fn add_system(&mut self, system: StartupDisposeSystem) {
        self.systems.push(system)
    }

    fn execute_system(system: StartupDisposeSystem) -> FruityResult<()> {
        profile_scope!(&system.identifier);
        (system.callback)()
    }

    /// Run all the systems in the system pool
    pub fn run_systems(&mut self) -> FruityResult<()> {
        #[cfg(target_arch = "wasm32")]
        {
            profile_scope!("main_thread_systems");

            self.systems
                .drain(..)
                .into_iter()
                .try_for_each(|system| Self::execute_system(system))?;

            Ok(())
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let (main_thread_systems, parallel_systems): (Vec<_>, Vec<_>) = self
                .systems
                .drain(..)
                .partition(|system| system.execute_in_main_thread);

            profile_scope!("parallel_systems");

            thread::scope(|s| {
                let handler = s.spawn(move || {
                    parallel_systems
                        .into_iter()
                        .par_bridge()
                        .try_for_each(|system| Self::execute_system(system))
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
                        .try_for_each(|system| Self::execute_system(system))?;
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
