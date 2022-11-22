use crate::export;
use crate::module::Module;
use crate::FruityResult;
use crate::ResourceContainer;

/// A service for frame management
#[derive(Clone)]
pub struct ModulesService {
    modules: Vec<Module>,
}

impl ModulesService {
    /// Returns an ModulesService
    pub fn new(_resource_container: ResourceContainer) -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    /// Register a module
    #[export]
    pub fn register_module(&mut self, module: Module) {
        self.modules.push(module);
    }

    /// Traverse the stored modules, order taking care of dependencies
    pub fn traverse_modules_by_dependencies(
        &self,
        callback: &dyn Fn(Module) -> FruityResult<()>,
    ) -> FruityResult<()> {
        let mut processed_module_identifiers = Vec::<String>::new();
        let mut remaining_modules = self
            .modules
            .iter()
            .map(|module| module.clone())
            .collect::<Vec<_>>();

        while remaining_modules.len() > 0 {
            let (with_all_dependencies_loaded, others): (Vec<_>, Vec<_>) =
                remaining_modules.into_iter().partition(|loader| {
                    loader
                        .dependencies
                        .iter()
                        .all(|dependency| processed_module_identifiers.contains(&dependency))
                });

            if with_all_dependencies_loaded.len() == 0 {
                panic!("A problem happened, couldn't load the dependencies cause there is a missing dependency, the modules that are still waiting to be initialized are:\n{:#?}", &others.iter().map(|module| module.name.clone()).collect::<Vec<_>>());
            }

            processed_module_identifiers.append(
                &mut with_all_dependencies_loaded
                    .iter()
                    .map(|module| module.name.clone())
                    .collect::<Vec<_>>(),
            );
            with_all_dependencies_loaded
                .into_iter()
                .try_for_each(|module| callback(module))?;

            remaining_modules = others;
        }

        Ok(())
    }
}
