use crate::ResourceContainer;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::inject::Inject;
use fruity_game_engine::profile_scope;
use fruity_game_engine::script_value::convert::TryFromScriptValue;
use fruity_game_engine::FruityResult;
use fruity_game_engine::Mutex;
use fruity_game_engine::{export, export_impl, export_struct};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use std::thread;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

/// A callback for a system called every frame
pub type SystemCallback = dyn Fn() -> FruityResult<()> + Send + Sync + 'static;

/// A callback for a startup system dispose callback
pub type StartupDisposeSystemCallback =
    Option<Box<dyn FnOnce() -> FruityResult<()> + Send + Sync + 'static>>;

/// A callback for a startup system
pub type StartupSystemCallback =
    dyn Fn() -> FruityResult<StartupDisposeSystemCallback> + Send + Sync + 'static;

/// Params for a system
#[derive(Debug, Clone, TryFromScriptValue, Default)]
pub struct SystemParams {
    /// The pool index
    pub pool_index: Option<usize>,

    /// If true, the system is still running while pause
    pub ignore_pause: Option<bool>,

    /// If true, the system will be executed in the main thread
    pub execute_in_main_thread: Option<bool>,
}

/// Params for a system
#[derive(Debug, Clone, TryFromScriptValue, Default)]
pub struct StartupSystemParams {
    /// If true, the system is still running while pause
    pub ignore_pause: Option<bool>,

    /// If true, the system will be executed in the main thread
    pub execute_in_main_thread: Option<bool>,
}

#[derive(Clone)]
struct StartupSystem {
    identifier: String,
    callback: Arc<StartupSystemCallback>,
    ignore_pause: bool,
    execute_in_main_thread: bool,
}

struct StartupDisposeSystem {
    identifier: String,
    callback: Box<dyn FnOnce() -> FruityResult<()> + Send + Sync + 'static>,
    execute_in_main_thread: bool,
}

#[derive(Clone)]
struct FrameSystem {
    identifier: String,
    callback: Arc<SystemCallback>,
    ignore_pause: bool,
    execute_in_main_thread: bool,
}

/// A system pool, see [‘SystemService‘] for more informations
#[derive(Clone)]
pub struct FrameSystemPool {
    /// Systems of the pool
    systems: Vec<FrameSystem>,

    /// Is the pool enabled
    enabled: bool,
}

/// A systems collection
///
/// There is three type of systems:
/// - begin_systems are called just before the rendering but after the resources allocations, it's perfect for initiliazing your entities
/// - end systems is called before closing the software
/// - systems are called every frame
///
/// There is a pool system, when you add a system, you can provide a pool, every systems of the same pool will be executed in parallel
/// Try to use it realy rarely, cause parallel execution is realy usefull
/// Pools from 0 to 10 and from 90 to 100 are reservec by the engine, you should avoid to create pool outside this range
/// Pool 98 is for drawing
/// Pool 99 is for camera
///
#[derive(FruityAny)]
#[export_struct]
pub struct SystemService {
    pause: AtomicBool,
    system_pools: BTreeMap<usize, FrameSystemPool>,
    startup_systems: Vec<StartupSystem>,
    startup_dispose_callbacks: Arc<Mutex<Vec<StartupDisposeSystem>>>,
    startup_pause_dispose_callbacks: Arc<Mutex<Vec<StartupDisposeSystem>>>,
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
        SystemService {
            pause: AtomicBool::new(false),
            system_pools: BTreeMap::new(),
            startup_systems: Vec::new(),
            startup_dispose_callbacks: Arc::new(Mutex::new(Vec::new())),
            startup_pause_dispose_callbacks: Arc::new(Mutex::new(Vec::new())),
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
        self.add_arc_system(
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
        callback: Arc<dyn Send + Sync + Fn() -> FruityResult<()>>,
        params: Option<SystemParams>,
    ) {
        self.add_arc_system(
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
    pub fn add_arc_system(
        &mut self,
        identifier: &str,
        callback: Arc<dyn Send + Sync + Fn() -> FruityResult<()>>,
        params: Option<SystemParams>,
    ) {
        let params = params.unwrap_or_default();
        let system = FrameSystem {
            identifier: identifier.to_string(),
            callback,
            ignore_pause: params.ignore_pause.unwrap_or(false),
            execute_in_main_thread: params.execute_in_main_thread.unwrap_or(false),
        };

        if let Some(pool) = self.system_pools.get_mut(&params.pool_index.unwrap_or(50)) {
            pool.systems.push(system)
        } else {
            // If the pool not exists, we create it
            let systems = vec![system];
            self.system_pools.insert(
                params.pool_index.unwrap_or(50),
                FrameSystemPool {
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
        callback: Arc<
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
        callback: Arc<
            dyn Send
                + Sync
                + Fn() -> FruityResult<
                    Option<Box<dyn FnOnce() -> FruityResult<()> + Send + Sync + 'static>>,
                >,
        >,
        params: Option<StartupSystemParams>,
    ) {
        let params = params.unwrap_or_default();
        let system = StartupSystem {
            identifier: identifier.to_string(),
            callback,
            ignore_pause: params.ignore_pause.unwrap_or(false),
            execute_in_main_thread: params.execute_in_main_thread.unwrap_or(false),
        };

        self.startup_systems.push(system);
    }

    /// Iter over all the systems pools
    fn iter_system_pools(&self) -> impl Iterator<Item = &FrameSystemPool> {
        self.system_pools.iter().map(|pool| pool.1)
    }

    /// Run all the stored systems
    pub fn run_frame(&self) -> FruityResult<()> {
        profile_scope!("frame_systems");

        let is_paused = self.is_paused();

        self.iter_system_pools()
            .map(|pool| pool.clone())
            .try_for_each(|pool| {
                if pool.enabled {
                    // Run the threaded systems
                    Self::run_systems_collection(
                        pool.systems.clone(),
                        |system| system.execute_in_main_thread,
                        move |system| {
                            if !is_paused || system.ignore_pause {
                                profile_scope!(&system.identifier);
                                (system.callback)()
                            } else {
                                Ok(())
                            }
                        },
                    )?;
                }

                FruityResult::Ok(())
            })
    }

    /// Run all the startup systems
    pub(crate) fn run_start(&self) -> FruityResult<()> {
        profile_scope!("start_systems");

        // Run the threaded systems
        let startup_dispose_callbacks = self.startup_dispose_callbacks.clone();

        Self::run_systems_collection(
            self.startup_systems
                .clone()
                .into_iter()
                .filter(|system| system.ignore_pause)
                .collect::<Vec<_>>(),
            |system| system.execute_in_main_thread,
            move |system| {
                profile_scope!(&system.identifier);

                let dispose_callback = (system.callback)()?;

                if let Some(dispose_callback) = dispose_callback {
                    let mut startup_dispose_callbacks = startup_dispose_callbacks.lock();
                    startup_dispose_callbacks.push(StartupDisposeSystem {
                        identifier: system.identifier.clone(),
                        callback: dispose_callback,
                        execute_in_main_thread: system.execute_in_main_thread,
                    });
                }

                FruityResult::Ok(())
            },
        )?;

        if !self.is_paused() {
            self.run_unpause_start()?;
        }

        Result::Ok(())
    }

    /// Run all startup dispose callbacks
    pub(crate) fn run_end(&self) -> FruityResult<()> {
        profile_scope!("end_systems");

        if !self.is_paused() {
            self.run_unpause_end()?;
        }

        let mut startup_dispose_callbacks = self.startup_dispose_callbacks.lock();
        Self::run_systems_collection(
            startup_dispose_callbacks.drain(..).collect::<Vec<_>>(),
            |system| system.execute_in_main_thread,
            move |system| {
                profile_scope!(&system.identifier);
                (system.callback)()
            },
        )?;

        Ok(())
    }

    /// Run all the startup systems that start when pause is stopped
    fn run_unpause_start(&self) -> FruityResult<()> {
        let startup_pause_dispose_callbacks = self.startup_pause_dispose_callbacks.clone();

        Self::run_systems_collection(
            self.startup_systems
                .clone()
                .into_iter()
                .filter(|system| !system.ignore_pause)
                .collect::<Vec<_>>(),
            |system| system.execute_in_main_thread,
            move |system| {
                profile_scope!(&system.identifier);

                let dispose_callback = (system.callback)()?;

                if let Some(dispose_callback) = dispose_callback {
                    let mut startup_dispose_callbacks = startup_pause_dispose_callbacks.lock();
                    startup_dispose_callbacks.push(StartupDisposeSystem {
                        identifier: system.identifier.clone(),
                        callback: dispose_callback,
                        execute_in_main_thread: system.execute_in_main_thread,
                    });
                }

                FruityResult::Ok(())
            },
        )
    }

    /// Run all the startup dispose callbacks of systems that start when pause is stopped
    fn run_unpause_end(&self) -> FruityResult<()> {
        let mut startup_dispose_callbacks = self.startup_pause_dispose_callbacks.lock();
        Self::run_systems_collection(
            startup_dispose_callbacks.drain(..).collect::<Vec<_>>(),
            |system| system.execute_in_main_thread,
            move |system| {
                profile_scope!(&system.identifier);
                (system.callback)()
            },
        )?;

        Ok(())
    }

    fn run_systems_collection<'a, T: Sync + Send + 'static>(
        mut systems: Vec<T>,
        is_execute_in_main_thread_closure: impl Fn(&T) -> bool,
        execute_systems_closure: impl Fn(T) -> FruityResult<()> + Clone + Send + Sync + 'static,
    ) -> FruityResult<()> {
        // Separate main thread and parallel systems
        let main_thread_systems = systems
            .drain_filter(|system| is_execute_in_main_thread_closure(system))
            .collect::<Vec<_>>();

        let parallel_systems = systems;

        #[cfg(not(target_arch = "wasm32"))]
        let handler = {
            profile_scope!("parallel_systems");
            let execute_systems_closure = execute_systems_closure.clone();
            thread::spawn(move || {
                parallel_systems
                    .into_iter()
                    .par_bridge()
                    .try_for_each(execute_systems_closure)
            })
        };

        #[cfg(target_arch = "wasm32")]
        parallel_systems
            .into_iter()
            .try_for_each(execute_systems_closure.clone())?;

        // Run the main thread systems
        {
            profile_scope!("main_thread_systems");
            main_thread_systems
                .into_iter()
                .try_for_each(execute_systems_closure)?;
        }

        // Wait all the threaded systems
        #[cfg(not(target_arch = "wasm32"))]
        {
            profile_scope!("join_parallel_systems");
            handler.join().unwrap()?;
        }

        Ok(())
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
        if !paused && self.is_paused() {
            self.run_unpause_start()?;
        }

        if paused && !self.is_paused() {
            self.run_unpause_end()?;
        }

        self.pause.store(paused, Ordering::Relaxed);
        Ok(())
    }
}
