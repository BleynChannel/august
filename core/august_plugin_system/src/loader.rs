use std::{
    cell::{Ref, RefCell, RefMut},
    path::Path,
    rc::Rc,
};

use crate::{
    context::LoadPluginContext,
    utils::{
        BuilderError, LoadPluginError, RegisterManagerError, RegisterPluginError, StopLoaderError,
        UnloadPluginError, UnregisterManagerError, UnregisterPluginError,
    },
    Link, Requests, Plugin, PluginManager, Registry,
};

pub struct PluginLoader {
    managers: Vec<Link<Box<dyn PluginManager>>>,
    registry: Link<Registry>,
    requests: Link<Requests>,
    plugins: Vec<Link<Plugin>>,
}

impl PluginLoader {
    pub(crate) fn new(
        managers: Vec<Box<dyn PluginManager>>,
        registry: Registry,
        requests: Requests,
    ) -> Result<Self, BuilderError> {
        let mut loader = Self {
            managers: Vec::new(),
            registry: Rc::new(RefCell::new(registry)),
            requests: Rc::new(RefCell::new(requests)),
            plugins: Vec::new(),
        };

        loader.register_managers(managers)?;

        Ok(loader)
    }

    pub fn stop(&mut self) -> Result<(), StopLoaderError> {
        PrivateLoader::stop_plugins(self)?;
        PrivateLoader::stop_managers(self)?;
        Ok(())
    }

    pub fn register_manager(
        &mut self,
        manager: Box<dyn PluginManager>,
    ) -> Result<(), RegisterManagerError> {
        if let Some(_) = self
            .managers
            .iter()
            .find(|m| manager.format() == m.borrow().format())
        {
            return Err(RegisterManagerError::AlreadyOccupiedFormat(
                manager.format().to_string(),
            ));
        }

        let wrapper = self.wrap();

        self.managers.push(Rc::new(RefCell::new(manager)));
        let manager = self.managers.last().unwrap();
        manager.borrow_mut().register_manager(wrapper)?;

        Ok(())
    }

    pub fn register_managers(
        &mut self,
        managers: Vec<Box<dyn PluginManager>>,
    ) -> Result<(), RegisterManagerError> {
        for manager in managers {
            self.register_manager(manager)?;
        }

        Ok(())
    }

    pub fn unregister_manager(&mut self, index: usize) -> Result<(), UnregisterManagerError> {
        if let Some(manager) = self.managers.get(index) {
            manager.borrow_mut().unregister_manager()?;
            self.managers.remove(index);

            Ok(())
        } else {
            return Err(UnregisterManagerError::NotFound);
        }
    }

    pub fn get_manager(&self, index: usize) -> Option<Ref<'_, Box<dyn PluginManager>>> {
        self.managers.get(index).map(|m| m.borrow())
    }

    pub fn get_manager_mut(&mut self, index: usize) -> Option<RefMut<'_, Box<dyn PluginManager>>> {
        self.managers.get(index).map(|m| m.borrow_mut())
    }

    pub fn get_managers(&self) -> Vec<Ref<'_, Box<dyn PluginManager>>> {
        self.managers.iter().map(|m| m.borrow()).collect()
    }

    pub fn register_plugin(&mut self, path: &str) -> Result<Link<Plugin>, RegisterPluginError> {
        let path = Path::new(path).to_path_buf();

        if !path.exists() {
            return Err(RegisterPluginError::NotFound);
        }
        if !path.is_dir() {
            return Err(RegisterPluginError::UnpackError(
                "Not a directory".to_string(),
            ));
        }

        // Получаем формат плагина и ищем подходящий менеджер
        if let Some(plugin_format) = path.extension() {
            let plugin_format = plugin_format.to_str().unwrap();
            if let Some(manager) = self
                .managers
                .iter()
                .find(|m| m.borrow().format() == plugin_format)
            {
                //Получаем нужную информацию про плагин
                let info = manager.borrow_mut().register_plugin(&path)?;

                if self
                    .plugins
                    .iter()
                    .find(|p| p.borrow().info.id == info.id)
                    .is_some()
                {
                    manager.borrow_mut().register_plugin_error(&info);
                    return Err(RegisterPluginError::AlreadyExistsID(info.id.clone()));
                }

                // Регистрируем плагин
                self.plugins.push(Rc::new(RefCell::new(Plugin::new(
                    manager.clone(),
                    path.clone(),
                    info,
                ))));

                return Ok(self.plugins.last().unwrap().clone());
            } else {
                return Err(RegisterPluginError::UnknownManagerFormat(
                    plugin_format.to_string(),
                ));
            }
        } else {
            return Err(RegisterPluginError::UnknownManagerFormat("".to_string()));
        }
    }

    pub fn unregister_plugin(
        &mut self,
        plugin: &Link<Plugin>,
    ) -> Result<(), UnregisterPluginError> {
        self.unload_plugin(plugin)?;

        let plugin_ref = plugin.borrow();

        plugin_ref
            .manager
            .as_ref()
            .borrow_mut()
            .unregister_plugin(&*plugin_ref)?;

        self.plugins.retain(|p| p.borrow().info != plugin_ref.info);

        Ok(())
    }

    pub fn load_plugin(&self, plugin: &Link<Plugin>) -> Result<(), LoadPluginError> {
		if plugin.borrow().is_load {
			return Ok(());
		}

        // Загружаем зависимости
		{
			let mut iter = self.plugins.iter();

			let mut not_found_depends = Vec::new();
			for depend in &plugin.borrow().info.depends {
				if let Some(dep) = iter.find(|p| p.borrow().info.id == *depend) {
					if let Err(e) = self.load_plugin(dep) {
						return Err(LoadPluginError::LoadDependency {
							depend: depend.clone(),
							error: Box::new(e),
						});
					}
				} else {
					not_found_depends.push(depend.clone());
				}
			}

			if !not_found_depends.is_empty() {
				return Err(LoadPluginError::NotFoundDependencies(not_found_depends));
			}

			for depend in &plugin.borrow().info.optional_depends {
				if let Some(dep) = iter.find(|p| p.borrow().info.id == *depend) {
					if let Err(e) = self.load_plugin(dep) {
						return Err(LoadPluginError::LoadDependency {
							depend: depend.clone(),
							error: Box::new(e),
						});
					}
				}
			}
		}

        // Загружаем плагин
        let manager = plugin.borrow().manager.clone();
        manager.borrow_mut().load_plugin(LoadPluginContext::new(
            plugin.clone(),
            self.registry.clone(),
            self.requests.clone(),
        ))?;

        // Проверяем наличие запрашиваемых функций
        let mut not_found_requests: Vec<String> = Vec::new();
        for (_, request) in &plugin.borrow().requests {
            if !self.requests.borrow().iter().any(|o| o.name == request.name) {
                not_found_requests.push(request.name.to_string());
            }
        }

        if !not_found_requests.is_empty() {
            return Err(LoadPluginError::RequestsNotFound(not_found_requests));
        }

        plugin.borrow_mut().is_load = true;

        Ok(())
    }

    pub fn unload_plugin(&self, plugin: &Link<Plugin>) -> Result<(), UnloadPluginError> {
        if plugin.borrow().is_load {
            for plug in self.plugins.iter() {
                let plug_info = &plug.borrow().info;

                if let Some(_) = plug_info
                    .depends
                    .iter()
                    .find(|depend| **depend == plug_info.id)
                {
                    return Err(UnloadPluginError::DependentOnAnotherPlugin(
                        plug_info.id.clone(),
                    ));
                }

                if let Some(_) = plug_info
                    .optional_depends
                    .iter()
                    .find(|depend| **depend == plug_info.id)
                {
                    return Err(UnloadPluginError::DependentOnAnotherPlugin(
                        plug_info.id.clone(),
                    ));
                }
            }

            plugin
                .borrow()
                .manager
                .borrow_mut()
                .unload_plugin(&*plugin.borrow())?;
        }

        plugin.borrow_mut().is_load = false;

        Ok(())
    }

    pub fn load_plugin_now(
        &mut self,
        path: &str,
    ) -> Result<Link<Plugin>, (Option<RegisterPluginError>, Option<LoadPluginError>)> {
        match self.register_plugin(path) {
            Ok(plugin) => {
                if let Err(e) = self.load_plugin(&plugin) {
                    return Err((None, Some(e)));
                }

                return Ok(plugin);
            }
            Err(e) => {
                return Err((Some(e), None));
            }
        }
    }

    pub fn load_plugins(
        &mut self,
        plugin_paths: Vec<&str>,
    ) -> Result<Vec<Link<Plugin>>, (Option<RegisterPluginError>, Option<LoadPluginError>)> {
        let mut plugins = Vec::new();

        for path in plugin_paths {
            plugins.push(match self.register_plugin(path) {
                Ok(plugin) => plugin,
                Err(e) => return Err((Some(e), None)),
            });
        }

        // Перебор плагинов, которые не являются зависимостями для других плагинов
        let not_depend_plugin = self.plugins.iter().filter(|plugin| {
            let id = &plugin.borrow().info.id;
            if let Some(_) = self.plugins.iter().find(|p| {
                let p_info = &p.borrow().info;
                let mut depends_iter = p_info.depends.iter().chain(p_info.optional_depends.iter());

                depends_iter.find(|depend| **depend == *id).is_some()
            }) {
                return false;
            }

            true
        });

        // Загрузка плагинов и их зависимостей
        for plugin in not_depend_plugin {
            if let Err(e) = self.load_plugin(&plugin) {
                return Err((None, Some(e)));
            }
        }

        Ok(plugins)
    }

    pub fn get_plugin(&self, index: usize) -> Option<Link<Plugin>> {
        self.plugins.get(index).map(|x| x.clone())
    }

    pub fn get_plugins(&self) -> Vec<Link<Plugin>> {
        self.plugins.iter().map(|x| x.clone()).collect()
    }
}

struct PrivateLoader;

impl PrivateLoader {
    fn stop_plugins(loader: &mut PluginLoader) -> Result<(), StopLoaderError> {
        // Сортируем плагины в порядке их зависимостей
        let sort_plugins = PrivateLoader::stop_sort_plugins(loader);

        let mut errors = Vec::new();

        // Выгружаем плагины
        for plugin in sort_plugins.iter() {
            if let Err(e) = loader.unregister_plugin(plugin) {
                //TODO: Добавить debug вывод
                errors.push((plugin.borrow().info.id.clone(), e));
            }
        }

        if !errors.is_empty() {
            return Err(StopLoaderError::UnregisterPluginFailed(errors));
        }
        Ok(())
    }

    fn stop_managers(loader: &mut PluginLoader) -> Result<(), StopLoaderError> {
        // Открепляем менеджеры плагинов от загрузчика
        let mut errors = Vec::new();
        for (index, manager) in loader.managers.clone().iter().enumerate() {
            if let Err(e) = loader.unregister_manager(index) {
                errors.push((manager.borrow().format().to_string(), e));
            }
        }

        if !errors.is_empty() {
            return Err(StopLoaderError::UnregisterManagerFailed(errors));
        }
        Ok(())
    }

    fn stop_sort_pick(
        loader: &PluginLoader,
        plugin: &Link<Plugin>,
        result: &mut Vec<Link<Plugin>>,
    ) {
        result.push(plugin.clone());

        let plugin_info = &plugin.borrow().info;
        'outer: for depend in plugin_info
            .depends
            .iter()
            .chain(plugin_info.optional_depends.iter())
        {
            if !result.iter().any(|pl| pl.borrow().info.id == *depend) {
                let mut p: Option<&Link<Plugin>> = None;

                for plug in loader.plugins.iter() {
                    let plug_info = &plug.borrow().info;
                    if plug_info.id == *depend {
                        p = Some(plug);
                        continue;
                    }

                    if !result.iter().any(|pl| pl.borrow().info.id == plug_info.id)
                        && (plug_info.depends.contains(depend)
                            || plug_info.optional_depends.contains(depend))
                    {
                        continue 'outer;
                    }
                }

                PrivateLoader::stop_sort_pick(loader, p.unwrap(), result);
            }
        }
    }

    // Продвинутая древовидная сортировка
    fn stop_sort_plugins(loader: &PluginLoader) -> Vec<Link<Plugin>> {
        let mut result = vec![];

        'outer: for plugin in loader.plugins.iter() {
            {
                let plugin_info = &plugin.borrow().info;
                for pl in loader.plugins.iter() {
                    let pl_info = &pl.borrow().info;
                    if pl_info.depends.contains(&plugin_info.id)
                        || pl_info.optional_depends.contains(&plugin_info.id)
                    {
                        continue 'outer;
                    }
                }
            }

            PrivateLoader::stop_sort_pick(loader, plugin, &mut result);
        }

        result
    }
}
