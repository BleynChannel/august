use std::path::Path;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    function::Function,
    utils::{
        LoadPluginError, PluginCallRequest, Ptr, RegisterManagerError, RegisterPluginError,
        StopLoaderError, UnloadPluginError, UnregisterManagerError, UnregisterPluginError,
    },
    variable::Variable,
    LoaderContext, Manager, Plugin, Registry, Requests,
};

pub struct Loader<'a, F: Function> {
    managers: Vec<Box<dyn Manager<'a, F>>>,
    pub(crate) registry: Registry<F>,
    pub(crate) requests: Requests,
    plugins: Vec<Plugin<'a, F>>,
}

impl<'a, F: Function> Loader<'a, F> {
    pub const fn new() -> Self {
        Self {
            managers: vec![],
            registry: vec![],
            requests: vec![],
            plugins: vec![],
        }
    }

    pub fn context<FO, R>(&mut self, f: FO) -> R
    where
        FO: FnOnce(LoaderContext<'a, '_, F>) -> R,
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
        M: Manager<'a, F> + 'static,
    {
        private_loader::register_manager(self, Box::new(manager))
    }

    pub fn register_managers<M>(&mut self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'a, F>>>,
    {
        managers
            .into_iter()
            .try_for_each(|manager| private_loader::register_manager(self, manager))?;

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
            Some(index) => private_loader::unregister_manager(self, index),
            None => return Err(UnregisterManagerError::NotFound),
        }
    }

    pub fn get_manager_ref(&self, format: &str) -> Option<&Box<dyn Manager<'a, F>>> {
        self.managers.iter().find(|m| m.format() == format)
    }

    pub fn get_manager_mut(&mut self, format: &str) -> Option<&mut Box<dyn Manager<'a, F>>> {
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
            .ok_or(UnregisterPluginError::NotFound)? as *mut Plugin<'a, F>;
        private_loader::unregister_plugin(self, plugin)
    }

    pub fn load_plugin(&mut self, id: &String) -> Result<(), LoadPluginError> {
        let plugin =
            self.get_plugin_mut(id).ok_or(LoadPluginError::NotFound)? as *mut Plugin<'a, F>;
        private_loader::load_plugin(self, plugin)
    }

    pub fn unload_plugin(&mut self, id: &String) -> Result<(), UnloadPluginError> {
        let plugin =
            self.get_plugin_mut(id).ok_or(UnloadPluginError::NotFound)? as *mut Plugin<'a, F>;
        private_loader::unload_plugin(self, plugin)
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
            plugins.push(self.register_plugin(path).map_err(|e| (Some(e), None))?);
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

        let not_depend_plugin: Vec<*mut Plugin<'a, F>> = self
            .plugins
            .iter_mut()
            .filter_map(move |plugin| {
                let id = plugin.info.id.clone();
                match plugins_depends
                    .iter()
                    .find(|depends| depends.iter().find(|depend| **depend == id).is_some())
                {
                    Some(_) => None,
                    None => Some(plugin as *mut Plugin<'a, F>),
                }
            })
            .collect();

        // Загрузка плагинов и их зависимостей
        not_depend_plugin.into_iter().try_for_each(|plugin| {
            private_loader::load_plugin(self, plugin).map_err(|e| (None, Some(e)))
        })?;

        Ok(plugins)
    }

    pub fn get_plugin(&self, id: &String) -> Option<&Plugin<'a, F>> {
        self.plugins.iter().find(|plugin| plugin.info.id == *id)
    }

    pub fn get_plugin_mut(&mut self, id: &String) -> Option<&mut Plugin<'a, F>> {
        self.plugins.iter_mut().find(|plugin| plugin.info.id == *id)
    }

    pub const fn get_plugins(&self) -> &Vec<Plugin<'a, F>> {
        &self.plugins
    }

    pub const fn get_registry(&self) -> &Registry<F> {
        &self.registry
    }

    pub const fn get_requests(&self) -> &Requests {
        &self.requests
    }

    pub fn call_request(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<Vec<F::CallResult>, PluginCallRequest> {
        self.plugins
            .iter()
            .map(|plugin| plugin.call_request(name, args))
            .collect::<Result<Vec<F::CallResult>, PluginCallRequest>>()
    }

    pub fn par_call_request(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<Vec<F::CallResult>, PluginCallRequest> {
        self.plugins
            .par_iter()
            .map(|plugin| plugin.call_request(name, args))
            .collect::<Result<Vec<F::CallResult>, PluginCallRequest>>()
    }
}

mod private_loader {
    use crate::{
        function::Function,
        utils::{
            LoadPluginError, Ptr, RegisterManagerError, StopLoaderError, UnloadPluginError,
            UnregisterManagerError, UnregisterPluginError,
        },
        LoadPluginContext, Manager, Plugin, Registry, Requests,
    };

    pub fn stop_plugins<F: Function>(
        loader: &mut super::Loader<'_, F>,
    ) -> Result<(), StopLoaderError> {
        // Сортируем плагины в порядке их зависимостей
        let sort_plugins = stop_sort_plugins(loader);

        let mut errors = vec![];

        // Выгружаем плагины
        for plugin in sort_plugins.iter() {
            if let Err(e) = unregister_plugin(loader, *plugin) {
                //TODO: Добавить debug вывод
                errors.push((unsafe { &**plugin }.info.id.clone(), e));
            }
        }

        match !errors.is_empty() {
            true => Err(StopLoaderError::UnregisterPluginFailed(errors)),
            false => Ok(()),
        }
    }

    pub fn stop_managers<F: Function>(
        loader: &mut super::Loader<'_, F>,
    ) -> Result<(), StopLoaderError> {
        // Открепляем менеджеры плагинов от загрузчика
        let mut errors = vec![];
        while !loader.managers.is_empty() {
            if let Err(e) = unregister_manager(loader, 0_usize) {
                errors.push(e);
            }
        }

        match !errors.is_empty() {
            true => Err(StopLoaderError::UnregisterManagerFailed(errors)),
            false => Ok(()),
        }
    }

    // Продвинутая древовидная сортировка
    pub fn stop_sort_plugins<'a, F: Function>(
        loader: *mut super::Loader<'a, F>,
    ) -> Vec<*mut Plugin<'a, F>> {
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

    pub fn stop_sort_pick<'a, F: Function>(
        loader: &mut super::Loader<'a, F>,
        plugin: *mut Plugin<'a, F>,
        mut result: Vec<*mut Plugin<'a, F>>,
    ) -> Vec<*mut Plugin<'a, F>> {
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
                        p = Some(plug as *mut Plugin<'a, F>);
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

    pub fn register_manager<'a, F: Function>(
        loader: &mut super::Loader<'a, F>,
        mut manager: Box<dyn Manager<'a, F>>,
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

    pub fn unregister_manager<F: Function>(
        loader: &mut super::Loader<'_, F>,
        index: usize,
    ) -> Result<(), UnregisterManagerError> {
        let mut manager = loader.managers.remove(index);
        loader
            .plugins
            .retain(|p| p.manager.as_ref().format() != manager.format());

        match manager.unregister_manager() {
            Ok(_) => Ok(()),
            Err(e) => Err(UnregisterManagerError::UnregisterManagerByManager(e)),
        }
    }

    pub fn unregister_plugin<'a, F: Function>(
        loader: &mut super::Loader<'a, F>,
        plugin: *mut Plugin<'a, F>,
    ) -> Result<(), UnregisterPluginError> {
        unload_plugin(loader, plugin.clone())?;

        unsafe { &mut *plugin }
            .manager
            .as_mut()
            .unregister_plugin(Ptr::<'a>::new(unsafe { &mut *plugin }))?;

        loader
            .plugins
            .retain(|p| p.info.id != unsafe { &*plugin }.info.id);

        Ok(())
    }

    pub fn load_plugin<'a, F: Function>(
        loader: &mut super::Loader<'a, F>,
        plugin: *mut Plugin<'a, F>,
    ) -> Result<(), LoadPluginError> {
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
            if let Some(_) = loader.plugins.iter().find(|p| p.info.id == *id_depend) {
                if let Err(e) = loader.load_plugin(id_depend) {
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
                Ptr::new(&mut loader.registry as *mut Registry<F>),
                Ptr::new(&mut loader.requests as *mut Requests),
            ))?;

        // Проверяем наличие запрашиваемых функций
        let not_found_requests: Vec<String> = unsafe { &*plugin }
            .requests
            .iter()
            .filter_map(|request| {
                match !loader
                    .requests
                    .iter()
                    .any(|req| req.name() == request.name())
                {
                    true => Some(request.name().to_string()),
                    false => None,
                }
            })
            .collect();

        if !not_found_requests.is_empty() {
            return Err(LoadPluginError::RequestsNotFound(not_found_requests));
        }

        unsafe { &mut *plugin }.is_load = true;

        Ok(())
    }

    pub fn unload_plugin<'a, F: Function>(
        loader: &mut super::Loader<'a, F>,
        plugin: *mut Plugin<'a, F>,
    ) -> Result<(), UnloadPluginError> {
        if unsafe { &*plugin }.is_load {
            loader.plugins.iter().try_for_each(|plug| {
                let plug_info = &plug.info;

                let mut depends_iter = plug_info
                    .depends
                    .iter()
                    .chain(plug_info.optional_depends.iter());
                match depends_iter.find(|depend| **depend == plug_info.id) {
                    Some(_) => Err(UnloadPluginError::DependentOnAnotherPlugin(
                        plug_info.id.clone(),
                    )),
                    None => Ok(()),
                }
            })?;

            unsafe { &mut *plugin }
                .manager
                .as_mut()
                .unload_plugin(Ptr::<'a>::new(unsafe { &mut *plugin }))?;
        }

        unsafe { &mut *plugin }.is_load = false;

        Ok(())
    }
}
