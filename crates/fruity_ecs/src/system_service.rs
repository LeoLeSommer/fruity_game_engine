use crate::ResourceContainer;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export;
use fruity_game_engine::fruity_export;
use fruity_game_engine::inject::Inject;
use fruity_game_engine::profile::profile_scope;
use fruity_game_engine::resource::Resource;
use fruity_game_engine::script_value::convert::TryFromScriptValue;
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::script_value::ScriptCallback;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::FruityResult;
use fruity_game_engine::Mutex;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

/// A callback for a system called every frame
pub type SystemCallback = dyn Fn(ResourceContainer) + Sync + Send + 'static;

/// A callback for a startup system dispose callback
pub type StartupDisposeSystemCallback = Option<Box<dyn FnOnce() + Sync + Send + 'static>>;

/// A callback for a startup system
pub type StartupSystemCallback =
    dyn Fn(ResourceContainer) -> StartupDisposeSystemCallback + Sync + Send + 'static;

/// Params for a system
#[derive(Debug, Clone, TryFromScriptValue)]
pub struct SystemParams {
    /// The pool index
    pub pool_index: usize,

    /// If true, the system is still running while pause
    pub ignore_pause: bool,
}

impl Default for SystemParams {
    fn default() -> Self {
        Self {
            pool_index: 50,
            ignore_pause: false,
        }
    }
}

/// Params for a system
#[derive(Debug, Clone, TryFromScriptValue)]
pub struct StartupSystemParams {
    /// If true, the system is still running while pause
    pub ignore_pause: bool,
}

impl Default for StartupSystemParams {
    fn default() -> Self {
        Self {
            ignore_pause: false,
        }
    }
}

#[derive(Clone)]
struct StartupSystem {
    identifier: String,
    callback: Arc<StartupSystemCallback>,
    ignore_pause: bool,
}

struct StartupDisposeSystem {
    identifier: String,
    callback: Box<dyn FnOnce() + Sync + Send + 'static>,
}

#[derive(Clone)]
struct ScriptFrameSystem {
    identifier: String,
    callback: Rc<dyn ScriptCallback>,
    ignore_pause: bool,
}

// All the functions that use ScriptFrameSystem should be wrapped into
// an unsafe call
//
// If we want to keep SystemService as a resource, we need it to store it as Send + Sync
// The only calls to the run functions are made in the middleware
// The only calls to add a ScriptFrameSystem are unsafe or called by the scripting language
// which is mono-threaded
//
// TODO: Find a safer way to do that, for example by splitting system service into a multi threads
// service for native systems and a single threaded service for scripting systems
//
unsafe impl Sync for ScriptFrameSystem {}
unsafe impl Send for ScriptFrameSystem {}

#[derive(Clone)]
struct ScriptStartupSystem {
    identifier: String,
    callback: Rc<dyn ScriptCallback>,
    ignore_pause: bool,
}

// All the functions that use ScriptStartupSystem should be wrapped into
// an unsafe call
//
// If we want to keep SystemService as a resource, we need it to store it as Send + Sync
// The only calls to the run functions are made in the middleware
// The only calls to add a ScriptStartupSystem are unsafe or called by the scripting language
// which is mono-threaded
//
// TODO: Find a safer way to do that, for example by splitting system service into a multi threads
// service for native systems and a single threaded service for scripting systems
//
unsafe impl Sync for ScriptStartupSystem {}
unsafe impl Send for ScriptStartupSystem {}

pub(crate) struct ScriptStartupDisposeSystem {
    identifier: String,
    callback: Rc<dyn ScriptCallback>,
}

// All the functions that use ScriptStartupDisposeSystem should be wrapped into
// an unsafe call
//
// If we want to keep SystemService as a resource, we need it to store it as Send + Sync
// The only calls to the run functions are made in the middleware
// The only calls to add a ScriptStartupDisposeSystem are unsafe or called by the scripting language
// which is mono-threaded
//
// TODO: Find a safer way to do that, for example by splitting system service into a multi threads
// service for native systems and a single threaded service for scripting systems
//
unsafe impl Sync for ScriptStartupDisposeSystem {}
unsafe impl Send for ScriptStartupDisposeSystem {}

#[derive(Clone)]
struct FrameSystem {
    identifier: String,
    callback: Arc<SystemCallback>,
    ignore_pause: bool,
}

/// A system pool, see [‘SystemService‘] for more informations
#[derive(Clone)]
pub struct FrameSystemPool {
    /// Systems of the pool
    systems: Vec<FrameSystem>,

    /// Script systems of the pool
    script_systems: Vec<ScriptFrameSystem>,

    /// Is the pool enabled
    enabled: bool,
}

fruity_export! {
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
    #[derive(FruityAny, Resource)]
    pub struct SystemService {
        pause: AtomicBool,
        system_pools: BTreeMap<usize, FrameSystemPool>,
        startup_systems: Vec<StartupSystem>,
        startup_dispose_callbacks: Mutex<Vec<StartupDisposeSystem>>,
        startup_pause_dispose_callbacks: Mutex<Vec<StartupDisposeSystem>>,
        script_startup_systems: Vec<ScriptStartupSystem>,
        script_startup_dispose_callbacks: Vec<ScriptStartupDisposeSystem>,
        resource_container: ResourceContainer,
    }

    impl Debug for SystemService {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
            Ok(())
        }
    }

    impl SystemService {
        /// Returns a SystemService
        pub fn new(resource_container: ResourceContainer) -> SystemService {
            SystemService {
                pause: AtomicBool::new(false),
                system_pools: BTreeMap::new(),
                startup_systems: Vec::new(),
                startup_dispose_callbacks: Mutex::new(Vec::new()),
                startup_pause_dispose_callbacks: Mutex::new(Vec::new()),
                script_startup_systems: Vec::new(),
                script_startup_dispose_callbacks: Vec::new(),
                resource_container,
            }
        }

        /// Add a system to the collection
        ///
        /// # Arguments
        /// * `system` - A function that will compute the world
        /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
        ///
        pub fn add_system<T: Inject<()>>(
            &mut self,
            identifier: &str,
            callback: T,
            params: Option<SystemParams>,
        ) {
            let params = params.unwrap_or_default();
            let system = FrameSystem {
                identifier: identifier.to_string(),
                callback: callback.inject().into(),
                ignore_pause: params.ignore_pause,
            };

            if let Some(pool) = self.system_pools.get_mut(&params.pool_index) {
                pool.systems.push(system)
            } else {
                // If the pool not exists, we create it
                let systems = vec![system];
                self.system_pools.insert(
                    params.pool_index,
                    FrameSystemPool {
                        systems,
                        script_systems: vec![],
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
        pub fn add_startup_system<T: Inject<StartupDisposeSystemCallback>>(
            &mut self,
            identifier: &str,
            callback: T,
            params: Option<StartupSystemParams>,
        ) {
            let params = params.unwrap_or_default();
            let system = StartupSystem {
                identifier: identifier.to_string(),
                callback: callback.inject().into(),
                ignore_pause: params.ignore_pause,
            };

            self.startup_systems.push(system);
        }

        /// Add a system to the collection
        ///
        /// # Arguments
        /// * `system` - A function that will compute the world
        /// * `pool_index` - A pool identifier, all the systems of the same pool will be processed together in parallel
        ///
        #[export(name = "add_system")]
        /*unsafe */pub fn add_script_system(
            &mut self,
            identifier: String,
            callback: Rc<dyn ScriptCallback>,
            params: Option<SystemParams>,
        ) {
            let params = params.unwrap_or_default();
            let system = ScriptFrameSystem {
                identifier: identifier.to_string(),
                callback: callback,
                ignore_pause: params.ignore_pause,
            };

            if let Some(pool) = self.system_pools.get_mut(&params.pool_index) {
                pool.script_systems.push(system)
            } else {
                // If the pool not exists, we create it
                let script_systems = vec![system];
                self.system_pools.insert(
                    params.pool_index,
                    FrameSystemPool {
                        systems: vec![],
                        script_systems,
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
        #[export(name = "add_startup_system")]
        /*unsafe */pub fn add_script_startup_system(
            &mut self,
            identifier: String,
            callback: Rc<dyn ScriptCallback>,
            params: Option<StartupSystemParams>,
        ) {
            let params = params.unwrap_or_default();
            let system = ScriptStartupSystem {
                identifier: identifier.to_string(),
                callback: callback,
                ignore_pause: params.ignore_pause,
            };

            self.script_startup_systems.push(system);
        }

        /// Iter over all the systems pools
        fn iter_system_pools(&self) -> impl Iterator<Item = &FrameSystemPool> {
            self.system_pools.iter().map(|pool| pool.1)
        }

        /// Run all the stored systems
        pub(crate) fn run_frame(&self) -> FruityResult<()> {
            let is_paused = self.is_paused();

            self.iter_system_pools().map(|pool| pool.clone()).try_for_each(|pool| {
                if pool.enabled {
                    // Run the threaded systems
                    let resource_container = self.resource_container.clone();
                    let handler = thread::spawn(move || {
                        pool.systems.iter().par_bridge().for_each(|system| {
                            if !is_paused || system.ignore_pause {
                                let _profiler_scope = profile_scope(&system.identifier);
                                (system.callback)(resource_container.clone())
                            }
                        });
                    });

                    // Run the script systems
                    let resource_container = self.resource_container.clone();
                    pool.script_systems.iter().try_for_each(|system| {
                        if !is_paused || system.ignore_pause {
                            let _profiler_scope = profile_scope(&system.identifier);
                            system.callback.call(vec![resource_container.clone().into_script_value()?])?;
                        }

                        FruityResult::Ok(())
                    })?;

                    // Wait all the threaded systems
                    handler.join().unwrap();
                }

                FruityResult::Ok(())
            })
        }

        /// Run all the startup systems
        pub(crate) fn run_start(&mut self) -> FruityResult<()> {
            // Run the threaded systems
            self.startup_systems
                .par_iter()
                .filter(|system| system.ignore_pause)
                .for_each(|system| {
                    let _profiler_scope = profile_scope(&system.identifier);

                    let dispose_callback = (system.callback)(self.resource_container.clone());

                    if let Some(dispose_callback) = dispose_callback {
                        let mut startup_dispose_callbacks = self.startup_dispose_callbacks.lock();
                        startup_dispose_callbacks.push(StartupDisposeSystem {
                            identifier: system.identifier.clone(),
                            callback: dispose_callback,
                        });
                    }
                });

            // Run the script systems
            self.script_startup_systems
                .iter()
                .filter(|system| system.ignore_pause)
                .try_for_each(|system| {
                    let _profiler_scope = profile_scope(&system.identifier);

                    let dispose_callback = system.callback.call(vec![self.resource_container.clone().into_script_value()?])?;

                    if let ScriptValue::Callback(dispose_callback) = dispose_callback {
                        self.script_startup_dispose_callbacks.push(ScriptStartupDisposeSystem {
                            identifier: system.identifier.clone(),
                            callback: dispose_callback,
                        });
                    }

                    FruityResult::Ok(())
                })?;

            if !self.is_paused() {
                self.run_unpause_start();
            }

            Result::Ok(())
        }

        /// Run all startup dispose callbacks
        pub(crate) fn run_end(&mut self) -> FruityResult<()> {
            if !self.is_paused() {
                self.run_unpause_end();
            }

            // Run the threaded systems
            let mut startup_dispose_callbacks = self.startup_dispose_callbacks.lock();
            startup_dispose_callbacks.drain(..).par_bridge().for_each(|system| {
                let _profiler_scope = profile_scope(&system.identifier);
                (system.callback)()
            });

            // Run the script systems
            self.script_startup_dispose_callbacks.drain(..).try_for_each(|system| {
                let _profiler_scope = profile_scope(&system.identifier);
                system.callback.call(vec![]).map(|_| ())
            })?;

            FruityResult::Ok(())
        }

        /// Run all the startup systems that start when pause is stopped
        fn run_unpause_start(&self) {
            self.startup_systems
                .iter()
                .filter(|system| !system.ignore_pause)
                .for_each(|system| {
                    let _profiler_scope = profile_scope(&system.identifier);

                    let dispose_callback = (system.callback)(self.resource_container.clone());

                    if let Some(dispose_callback) = dispose_callback {
                        let mut startup_dispose_callbacks = self.startup_pause_dispose_callbacks.lock();
                        startup_dispose_callbacks.push(StartupDisposeSystem {
                            identifier: system.identifier.clone(),
                            callback: dispose_callback,
                        });
                    }
                });
        }

        /// Run all the startup dispose callbacks of systems that start when pause is stopped
        fn run_unpause_end(&self) {
            let mut startup_dispose_callbacks = self.startup_pause_dispose_callbacks.lock();
            startup_dispose_callbacks.drain(..).for_each(|system| {
                let _profiler_scope = profile_scope(&system.identifier);
                (system.callback)()
            });
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
        pub fn set_paused(&self, paused: bool) {
            if !paused && self.is_paused() {
                self.run_unpause_start();
            }

            if paused && !self.is_paused() {
                self.run_unpause_end();
            }

            self.pause.store(paused, Ordering::Relaxed);
        }
    }
}
