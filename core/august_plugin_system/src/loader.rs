use std::path::Path;

use crate::{
    utils::{
        LoadPluginError, Ptr, RegisterManagerError, RegisterPluginError, StopLoaderError,
        UnloadPluginError, UnregisterManagerError, UnregisterPluginError,
    },
    LoadPluginContext, LoaderContext, Manager, Plugin, Registry, Requests,
};

pub struct Loader<'a> {
    managers: Vec<Box<dyn Manager<'a>>>,
    pub(crate) registry: Registry,
    pub(crate) requests: Requests,
    plugins: Vec<Plugin<'a>>,
}

impl<'a> Loader<'a> {
    pub const fn new() -> Self {
        Self {
            managers: vec![],
            registry: vec![],
            requests: vec![],
            plugins: vec![],
        }
    }

    pub fn context<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(LoaderContext<'a, '_>) -> R,
    {
        f(LoaderContext::new(self))
    }

    pub fn stop(&mut self) -> Result<(), StopLoaderError> {
        private_loader::stop_plugins(self)?;
        private_loader::stop_managers(self)?;
        Ok(())
    }

    pub fn register_manager<M>(&mut self, manager: M) -> Result<(), RegisterManagerError>
    where
        M: Manager<'a> + 'static,
    {
        private_loader::register_manager(self, Box::new(manager))
    }

    pub fn register_managers<M>(&mut self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'a>>>,
    {
        for manager in managers {
            private_loader::register_manager(self, manager)?;
        }

        Ok(())
    }

    pub fn unregister_manager(&mut self, format: &str) -> Result<(), UnregisterManagerError> {
        let index = self.managers.iter().enumerate().find_map(|(i, manager)| {
            match manager.format() == format {
                true => Some(i),
                false => None,
            }
        });

        match index {
            Some(index) => self._unregister_manager(index),
            None => return Err(UnregisterManagerError::NotFound),
        }
    }

    fn _unregister_manager(&mut self, index: usize) -> Result<(), UnregisterManagerError> {
        let mut manager = self.managers.remove(index);
        //TODO: Необходимо найти все плагины, использующие данный менеджер и выгрузить

        match manager.unregister_manager() {
            Ok(_) => Ok(()),
            Err(e) => Err(UnregisterManagerError::UnregisterManagerByManager(e)),
        }
    }

    pub fn get_manager_ref(&self, format: &str) -> Option<&Box<dyn Manager<'a>>> {
        self.managers.iter().find(|m| m.format() == format)
    }

    pub fn get_manager_mut(&mut self, format: &str) -> Option<&mut Box<dyn Manager<'a>>> {
        self.managers.iter_mut().find(|m| m.format() == format)
    }

    pub fn register_plugin(&mut self, path: &str) -> Result<String, RegisterPluginError> {
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

            let manager = self.get_manager_mut(plugin_format).ok_or(
                RegisterPluginError::UnknownManagerFormat(plugin_format.to_string()),
            )?;
            let manager_ptr = Ptr::<'a>::new(manager);

            let info = manager_ptr.as_mut().register_plugin(&path)?;
            let id_clone = info.id.clone();

            if self.get_plugin(&id_clone).is_some() {
                manager_ptr.as_mut().register_plugin_error(info);
                return Err(RegisterPluginError::AlreadyExistsID(id_clone));
            }

            // Регистрируем плагин
            self.plugins
                .push(Plugin::<'a>::new(manager_ptr, path, info));

            return Ok(id_clone);
        } else {
            return Err(RegisterPluginError::UnknownManagerFormat("".to_string()));
        }
    }

    pub fn unregister_plugin(&mut self, id: &String) -> Result<(), UnregisterPluginError> {
        let plugin = self
            .get_plugin_mut(id)
            .ok_or(UnregisterPluginError::NotFound)? as *mut Plugin<'a>;
        self._unregister_plugin(plugin)
    }

    fn _unregister_plugin(&mut self, plugin: *mut Plugin<'a>) -> Result<(), UnregisterPluginError> {
        self._unload_plugin(plugin.clone())?;

        unsafe { &mut *plugin }
            .manager
            .as_mut()
            .unregister_plugin(Ptr::<'a>::new(unsafe { &mut *plugin }))?;

        self.plugins
            .retain(|p| p.info.id != unsafe { &*plugin }.info.id);

        Ok(())
    }

    pub fn load_plugin(&mut self, id: &String) -> Result<(), LoadPluginError> {
        let plugin = self.get_plugin_mut(id).ok_or(LoadPluginError::NotFound)? as *mut Plugin<'a>;
        self._load_plugin(plugin)
    }

    fn _load_plugin(&mut self, plugin: *mut Plugin<'a>) -> Result<(), LoadPluginError> {
        if unsafe { &*plugin }.is_load {
            return Ok(());
        }

        // Загружаем зависимости
        let info = &unsafe { &*plugin }.info;
        let depends_iter = info
            .depends
            .iter()
            .map(|d| (true, d))
            .chain(info.optional_depends.iter().map(|d| (false, d)));

        let mut not_found_depends = vec![];
        for (is_depend, id_depend) in depends_iter {
            if let Some(_) = self.plugins.iter().find(|p| p.info.id == *id_depend) {
                if let Err(e) = self.load_plugin(id_depend) {
                    return Err(LoadPluginError::LoadDependency {
                        depend: id_depend.clone(),
                        error: Box::new(e),
                    });
                }
            } else if is_depend {
                not_found_depends.push(id_depend.clone());
            }
        }

        if !not_found_depends.is_empty() {
            return Err(LoadPluginError::NotFoundDependencies(not_found_depends));
        }

        // Загружаем плагин
        unsafe { &mut *plugin }
            .manager
            .as_mut()
            .load_plugin(LoadPluginContext::new(
                Ptr::new(plugin),
                Ptr::new(&mut self.registry as *mut Registry),
                Ptr::new(&mut self.requests as *mut Requests),
            ))?;

        // Проверяем наличие запрашиваемых функций
        let mut not_found_requests: Vec<String> = vec![];
        for (_, request) in unsafe { &*plugin }.requests.iter() {
            if !self.requests.iter().any(|req| req.name == request.name) {
                not_found_requests.push(request.name.clone());
            }
        }

        if !not_found_requests.is_empty() {
            return Err(LoadPluginError::RequestsNotFound(not_found_requests));
        }

        unsafe { &mut *plugin }.is_load = true;

        Ok(())
    }

    pub fn unload_plugin(&mut self, id: &String) -> Result<(), UnloadPluginError> {
        let plugin = self.get_plugin_mut(id).ok_or(UnloadPluginError::NotFound)? as *mut Plugin<'a>;
        self._unload_plugin(plugin)
    }

    fn _unload_plugin(&mut self, plugin: *mut Plugin<'a>) -> Result<(), UnloadPluginError> {
        if unsafe { &*plugin }.is_load {
            for plug in self.plugins.iter() {
                let plug_info = &plug.info;

                let mut depends_iter = plug_info
                    .depends
                    .iter()
                    .chain(plug_info.optional_depends.iter());
                if let Some(_) = depends_iter.find(|depend| **depend == plug_info.id) {
                    return Err(UnloadPluginError::DependentOnAnotherPlugin(
                        plug_info.id.clone(),
                    ));
                }
            }

            unsafe { &mut *plugin }
                .manager
                .as_mut()
                .unload_plugin(Ptr::<'a>::new(unsafe { &mut *plugin }))?;
        }

        unsafe { &mut *plugin }.is_load = false;

        Ok(())
    }

    pub fn load_plugin_now(
        &mut self,
        path: &str,
    ) -> Result<String, (Option<RegisterPluginError>, Option<LoadPluginError>)> {
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

    pub fn load_plugins<'b, P>(
        &mut self,
        plugin_paths: P,
    ) -> Result<Vec<String>, (Option<RegisterPluginError>, Option<LoadPluginError>)>
    where
        P: IntoIterator<Item = &'b str>,
    {
        let mut plugins = vec![];

        for path in plugin_paths {
            plugins.push(match self.register_plugin(path) {
                Ok(plugin) => plugin,
                Err(e) => return Err((Some(e), None)),
            });
        }

        // Перебор плагинов, которые не являются зависимостями для других плагинов
        let plugins_depends = self
            .plugins
            .iter()
            .map(|p| {
                p.info
                    .depends
                    .iter()
                    .chain(p.info.optional_depends.iter())
                    .map(|d| d.clone())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        let not_depend_plugin: Vec<*mut Plugin<'a>> = self
            .plugins
            .iter_mut()
            .filter_map(move |plugin| {
                let id = plugin.info.id.clone();
                match plugins_depends
                    .iter()
                    .find(|depends| depends.iter().find(|depend| **depend == id).is_some())
                {
                    Some(_) => None,
                    None => Some(plugin as *mut Plugin<'a>),
                }
            })
            .collect();

        // Загрузка плагинов и их зависимостей
        for plugin in not_depend_plugin {
            if let Err(e) = self._load_plugin(plugin) {
                return Err((None, Some(e)));
            }
        }

        Ok(plugins)
    }

    pub fn get_plugin(&self, id: &String) -> Option<&Plugin<'a>> {
        self.plugins.iter().find(|plugin| plugin.info.id == *id)
    }

    pub fn get_plugin_mut(&mut self, id: &String) -> Option<&mut Plugin<'a>> {
        self.plugins.iter_mut().find(|plugin| plugin.info.id == *id)
    }

    pub const fn get_plugins(&self) -> &Vec<Plugin<'a>> {
        &self.plugins
    }
}

mod private_loader {
    use crate::{
        utils::{Ptr, RegisterManagerError, StopLoaderError},
        Manager, Plugin,
    };

    pub fn stop_plugins(loader: &mut super::Loader) -> Result<(), StopLoaderError> {
        // Сортируем плагины в порядке их зависимостей
        let sort_plugins = stop_sort_plugins(loader);

        let mut errors = vec![];

        // Выгружаем плагины
        for plugin in sort_plugins.iter() {
            if let Err(e) = loader._unregister_plugin(*plugin) {
                //TODO: Добавить debug вывод
                errors.push((unsafe { &**plugin }.info.id.clone(), e));
            }
        }

        match !errors.is_empty() {
            true => Err(StopLoaderError::UnregisterPluginFailed(errors)),
            false => Ok(()),
        }
    }

    pub fn stop_managers(loader: &mut super::Loader) -> Result<(), StopLoaderError> {
        // Открепляем менеджеры плагинов от загрузчика
        let mut errors = vec![];
        while !loader.managers.is_empty() {
            if let Err(e) = loader._unregister_manager(0_usize) {
                errors.push(e);
            }
        }

        match !errors.is_empty() {
            true => Err(StopLoaderError::UnregisterManagerFailed(errors)),
            false => Ok(()),
        }
    }

    // Продвинутая древовидная сортировка
    pub fn stop_sort_plugins<'a>(loader: *mut super::Loader<'a>) -> Vec<*mut Plugin<'a>> {
        let mut result = vec![];

        let plugins_depends = unsafe { &*loader }
            .plugins
            .iter()
            .map(|p| {
                p.info
                    .depends
                    .iter()
                    .chain(p.info.optional_depends.iter())
                    .map(|d| d.clone())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        'outer: for plugin in unsafe { &mut *loader }.plugins.iter_mut() {
            {
                let plugin_id = &plugin.info.id;
                for depends in plugins_depends.iter() {
                    if depends.contains(plugin_id) {
                        continue 'outer;
                    }
                }
            }

            result = stop_sort_pick(unsafe { &mut *loader }, plugin, result);
        }

        result
    }

    pub fn stop_sort_pick<'a>(
        loader: &mut super::Loader<'a>,
        plugin: *mut Plugin<'a>,
        mut result: Vec<*mut Plugin<'a>>,
    ) -> Vec<*mut Plugin<'a>> {
        result.push(plugin);

        let plugin_info = &unsafe { &*plugin }.info;
        'outer: for depend in plugin_info
            .depends
            .iter()
            .chain(plugin_info.optional_depends.iter())
        {
            if !result.iter().any(|pl| unsafe { &**pl }.info.id == *depend) {
                let mut p = None;

                for plug in loader.plugins.iter_mut() {
                    let plug_info = &plug.info;
                    if plug_info.id == *depend {
                        p = Some(plug as *mut Plugin<'a>);
                        continue;
                    }

                    if !result
                        .iter()
                        .any(|pl| unsafe { &**pl }.info.id == plug_info.id)
                        && (plug_info.depends.contains(depend)
                            || plug_info.optional_depends.contains(depend))
                    {
                        continue 'outer;
                    }
                }

                result = stop_sort_pick(loader, p.unwrap(), result);
            }
        }

        result
    }

    pub fn register_manager<'a>(
        loader: &mut super::Loader<'a>,
        mut manager: Box<dyn Manager<'a>>,
    ) -> Result<(), RegisterManagerError> {
        if let Some(_) = loader
            .managers
            .iter()
            .find(|m| manager.format() == m.format())
        {
            return Err(RegisterManagerError::AlreadyOccupiedFormat(
                manager.format().to_string(),
            ));
        }

        manager.as_mut().register_manager(Ptr::<'a>::new(loader))?;
        loader.managers.push(manager);

        Ok(())
    }
}
