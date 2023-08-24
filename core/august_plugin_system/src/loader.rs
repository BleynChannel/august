use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;

use crate::{
    utils::{
        bundle::Bundle, LoadPluginError, PluginCallRequest, RegisterManagerError,
        RegisterPluginError, StopLoaderError, UnloadPluginError, UnregisterManagerError,
        UnregisterPluginError,
    },
    variable::Variable,
    LoaderContext, Manager, Plugin, Registry, Requests,
};

pub struct Loader<'a, T: Send + Sync> {
    managers: Vec<Box<dyn Manager<'a, T>>>,
    pub(crate) registry: Registry<T>,
    pub(crate) requests: Requests,
    plugins: Vec<Plugin<'a, T>>,
}

impl<'a, T: Send + Sync> Loader<'a, T> {
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
        FO: FnOnce(LoaderContext<'a, '_, T>) -> R,
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
        M: Manager<'a, T> + 'static,
    {
        private_loader::register_manager(self, Box::new(manager))
    }

    pub fn register_managers<M>(&mut self, managers: M) -> Result<(), RegisterManagerError>
    where
        M: IntoIterator<Item = Box<dyn Manager<'a, T>>>,
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

    pub fn get_manager_ref(&self, format: &str) -> Option<&Box<dyn Manager<'a, T>>> {
        self.managers.iter().find(|m| m.format() == format)
    }

    pub fn get_manager_mut(&mut self, format: &str) -> Option<&mut Box<dyn Manager<'a, T>>> {
        self.managers.iter_mut().find(|m| m.format() == format)
    }

    pub fn register_plugin(&mut self, path: &str) -> Result<Bundle, RegisterPluginError> {
        private_loader::register_plugin(self, path)
    }

    pub fn unregister_plugin(
        &mut self,
        id: &str,
        version: &Version,
    ) -> Result<(), UnregisterPluginError> {
        let plugin = self
            .get_plugin_mut(id, version)
            .ok_or(UnregisterPluginError::NotFound)? as *mut Plugin<'a, T>;
        private_loader::unregister_plugin(self, plugin)
    }

    pub fn unregister_plugin_by_bundle(
        &mut self,
        bundle: &Bundle,
    ) -> Result<(), UnregisterPluginError> {
        let plugin = self
            .get_plugin_mut_by_bundle(bundle)
            .ok_or(UnregisterPluginError::NotFound)? as *mut Plugin<'a, T>;
        private_loader::unregister_plugin(self, plugin)
    }

    pub fn load_plugin(&mut self, id: &str, version: &Version) -> Result<(), LoadPluginError> {
        let plugin = self
            .get_plugin_mut(id, version)
            .ok_or(LoadPluginError::NotFound)? as *mut Plugin<'a, T>;
        private_loader::load_plugin(self, plugin)
    }

    pub fn load_plugin_by_bundle(&mut self, bundle: &Bundle) -> Result<(), LoadPluginError> {
        let plugin = self
            .get_plugin_mut_by_bundle(bundle)
            .ok_or(LoadPluginError::NotFound)? as *mut Plugin<'a, T>;
        private_loader::load_plugin(self, plugin)
    }

    pub fn unload_plugin(&mut self, id: &str, version: &Version) -> Result<(), UnloadPluginError> {
        let plugin = self
            .get_plugin_mut(id, version)
            .ok_or(UnloadPluginError::NotFound)? as *mut Plugin<'a, T>;
        private_loader::unload_plugin(self, plugin)
    }

    pub fn unload_plugin_by_bundle(&mut self, bundle: &Bundle) -> Result<(), UnloadPluginError> {
        let plugin = self
            .get_plugin_mut_by_bundle(bundle)
            .ok_or(UnloadPluginError::NotFound)? as *mut Plugin<'a, T>;
        private_loader::unload_plugin(self, plugin)
    }

    pub fn load_plugin_now(
        &mut self,
        path: &str,
    ) -> Result<Bundle, (Option<RegisterPluginError>, Option<LoadPluginError>)> {
        match self.register_plugin(path) {
            Ok(bundle) => {
                if let Err(e) = self.load_plugin_by_bundle(&bundle) {
                    return Err((None, Some(e)));
                }

                return Ok(bundle);
            }
            Err(e) => {
                return Err((Some(e), None));
            }
        }
    }

    //TODO: Добавить параллельную версию метода
    pub fn load_plugins<'b, P>(
        &mut self,
        plugin_paths: P,
    ) -> Result<
        Vec<Bundle>,
        (
            Option<RegisterPluginError>,
            Option<UnregisterPluginError>,
            Option<LoadPluginError>,
        ),
    >
    where
        P: IntoIterator<Item = &'b str>,
    {
        let mut infos = vec![];

        for path in plugin_paths {
            infos.push(
                self.register_plugin(path)
                    .map_err(|e| (Some(e), None, None))?,
            );
        }

        // Перебор плагинов, которые не являются зависимостями для других плагинов
        let mut result = vec![];
        let mut unregistered_plugins = vec![];

        'outer: for (index, plugin) in self.plugins.iter().enumerate() {
            let bundle = &plugin.info.bundle;

            let find_plugin = self
                .plugins
                .iter()
                .find(|pl| {
                    pl.info
                        .info
                        .depends
                        .iter()
                        .chain(pl.info.info.optional_depends.iter())
                        .any(|d| *d == *bundle)
                })
                .is_some();
            if !find_plugin {
                // Убираем неиспользуемые версии плагинов
                //TODO: Протестировать данный участок
                let id = &bundle.id;
                let version = &bundle.version;
                for pl in self.plugins.iter() {
                    if pl.info.bundle.id == *id && pl.info.bundle.version > *version {
                        unregistered_plugins.push(plugin.info.bundle.clone());
                        continue 'outer;
                    }
                }

                result.push(index);
            }
        }

        // Выгружаем неиспользуемые версии плагинов
        unregistered_plugins.into_iter().try_for_each(|bundle| {
            self.unregister_plugin_by_bundle(&bundle)
                .map_err(|e| (None, Some(e), None))
        })?;

        result.into_iter().try_for_each(|index| {
            let plugin = &mut self.plugins[index] as *mut Plugin<'a, T>;
            private_loader::load_plugin(self, plugin).map_err(|e| (None, None, Some(e)))
        })?;

        Ok(infos)
    }

    //TODO: Добавить параллельную версию метода
    pub fn get_plugin(&self, id: &str, version: &Version) -> Option<&Plugin<'a, T>> {
        self.plugins.iter().find(|plugin| **plugin == (id, version))
    }

    //TODO: Добавить параллельную версию метода
    pub fn get_plugin_by_bundle(&self, bundle: &Bundle) -> Option<&Plugin<'a, T>> {
        self.plugins.iter().find(|plugin| *plugin == bundle)
    }

    //TODO: Добавить параллельную версию метода
    pub fn get_plugin_mut(&mut self, id: &str, version: &Version) -> Option<&mut Plugin<'a, T>> {
        self.plugins
            .iter_mut()
            .find(|plugin| **plugin == (id, version))
    }

    //TODO: Добавить параллельную версию метода
    pub fn get_plugin_mut_by_bundle(&mut self, bundle: &Bundle) -> Option<&mut Plugin<'a, T>> {
        self.plugins.iter_mut().find(|plugin| *plugin == bundle)
    }

    //TODO: Добавить параллельную версию метода
    pub fn get_plugins_by_id(&self, id: &str) -> Vec<&Plugin<'a, T>> {
        self.plugins
            .iter()
            .filter(|plugin| plugin.info.bundle.id == id)
            .collect()
    }

    //TODO: Добавить параллельную версию метода
    pub fn get_plugins_by_id_mut(&mut self, id: &str) -> Vec<&mut Plugin<'a, T>> {
        self.plugins
            .iter_mut()
            .filter(|plugin| plugin.info.bundle.id == id)
            .collect()
    }

    pub const fn get_plugins(&self) -> &Vec<Plugin<'a, T>> {
        &self.plugins
    }

    pub const fn get_registry(&self) -> &Registry<T> {
        &self.registry
    }

    pub const fn get_requests(&self) -> &Requests {
        &self.requests
    }

    pub fn call_request(&self, name: &str, args: &[Variable]) -> Result<Vec<T>, PluginCallRequest> {
        self.plugins
            .iter()
            .map(|plugin| plugin.call_request(name, args))
            .collect()
    }

    pub fn par_call_request(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<Vec<T>, PluginCallRequest> {
        self.plugins
            .par_iter()
            .map(|plugin| plugin.call_request(name, args))
            .collect()
    }
}

mod private_loader {

    use std::path::Path;

    use crate::{
        utils::{
            bundle::Bundle, LoadPluginError, Ptr, RegisterManagerError, RegisterPluginError,
            StopLoaderError, UnloadPluginError, UnregisterManagerError, UnregisterPluginError,
        },
        LoadPluginContext, Manager, Plugin, PluginInfo, RegisterPluginContext, Registry, Requests,
    };

    pub fn stop_plugins<T: Send + Sync>(
        loader: &mut super::Loader<'_, T>,
    ) -> Result<(), StopLoaderError> {
        // Сортируем плагины в порядке их зависимостей
        let sort_plugins = sort_plugins(&loader.plugins.iter().collect());

        let mut errors = vec![];

        // Выгружаем плагины
        for plugin in sort_plugins.into_iter() {
            //TODO: Внедрить принудительную версию метода
            if let Err(e) = unregister_plugin(loader, plugin) {
                //TODO: Добавить debug вывод
                errors.push((unsafe { &*plugin }.info.bundle.id.clone(), e));
            }
        }

        match !errors.is_empty() {
            true => Err(StopLoaderError::UnregisterPluginFailed(errors)),
            false => Ok(()),
        }
    }

    pub fn stop_managers<'a, T: Send + Sync>(
        loader: &'a mut super::Loader<'_, T>,
    ) -> Result<(), StopLoaderError> {
        // Открепляем менеджеры плагинов от загрузчика
        let mut errors = vec![];
        while !loader.managers.is_empty() {
            //TODO: Внедрить принудительную версию метода
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
    pub fn sort_plugins<'a, T: Send + Sync>(
        plugins: &Vec<&Plugin<'a, T>>,
    ) -> Vec<*mut Plugin<'a, T>> {
        let mut result = vec![];

        'outer: for plugin in plugins.iter() {
            {
                let find_plugin = plugins
                    .iter()
                    .find(|pl| {
                        pl.info
                            .info
                            .depends
                            .iter()
                            .chain(pl.info.info.optional_depends.iter())
                            .any(|d| *d == **plugin)
                    })
                    .is_some();

                if find_plugin {
                    continue 'outer;
                }
            }

            sort_pick(
                plugins,
                *plugin as *const Plugin<'a, T> as *mut Plugin<'a, T>,
                &mut result,
            );
        }

        result
    }

    pub fn sort_pick<'a, T: Send + Sync>(
        plugins: &Vec<&Plugin<'a, T>>,
        plugin: *mut Plugin<'a, T>,
        result: &mut Vec<*mut Plugin<'a, T>>,
    ) {
        result.push(plugin);

        let plugin_info = &unsafe { &*plugin }.info;
        let depends = plugin_info
            .info
            .depends
            .iter()
            .chain(plugin_info.info.optional_depends.iter());
        'outer: for depend in depends {
            if !result.iter().any(|pl| *unsafe { &**pl } == *depend) {
                let mut p = None;

                for plug in plugins.iter() {
                    let plug_info = &plug.info;
                    if plug_info.bundle == *depend {
                        p = Some(*plug as *const Plugin<'a, T> as *mut Plugin<'a, T>);
                        continue;
                    }

                    if !result
                        .iter()
                        .any(|pl| unsafe { &**pl }.info.bundle == plug_info.bundle)
                        && (plug_info.info.depends.contains(depend)
                            || plug_info.info.optional_depends.contains(depend))
                    {
                        continue 'outer;
                    }
                }

                sort_pick(plugins, p.unwrap(), result);
            }
        }
    }

    pub fn register_manager<'a, T: Send + Sync>(
        loader: &mut super::Loader<'a, T>,
        mut manager: Box<dyn Manager<'a, T>>,
    ) -> Result<(), RegisterManagerError> {
        if let Some(_) = loader.managers.iter().find(|m| manager == **m) {
            return Err(RegisterManagerError::AlreadyOccupiedFormat(
                manager.format().to_string(),
            ));
        }

        manager.as_mut().register_manager(Ptr::<'a>::new(loader))?;
        loader.managers.push(manager);

        Ok(())
    }

    //TODO: Добавить принудительную версию метода
    pub fn unregister_manager<T: Send + Sync>(
        loader: &mut super::Loader<'_, T>,
        index: usize,
    ) -> Result<(), UnregisterManagerError> {
        let manager = &loader.managers[index];

        // Выгружаем все плагины, относящиеся к менеджеру
        let plugins_from_manager = loader
            .plugins
            .iter()
            .filter(|plugin| *plugin.manager.as_ref() == *manager)
            .collect();

        // Сортируем плагины менеджера в порядке их зависимостей
        let sort_plugins = sort_plugins(&plugins_from_manager);

        // Выгружаем плагины
        for plugin in sort_plugins.into_iter() {
            unregister_plugin(loader, plugin)?;
        }

        // Выгружаем менеджер
        let mut manager = loader.managers.remove(index);
        match manager.unregister_manager() {
            Ok(_) => Ok(()),
            Err(e) => Err(UnregisterManagerError::UnregisterManagerByManager(e)),
        }
    }

    //TODO: Добавить принудительную версию метода
    pub fn register_plugin<'a, T: Send + Sync>(
        loader: &mut super::Loader<'a, T>,
        path: &str,
    ) -> Result<Bundle, RegisterPluginError> {
        let path = Path::new(path).to_path_buf();

        if !path.is_dir() {
            return Err(RegisterPluginError::NotFound);
        }

        if let Some(_) = path.extension() {
            let bundle = Bundle::from_filename(path.file_name().unwrap())?;

            // Проверяем, есть ли уже такой плагин
            if loader.get_plugin_by_bundle(&bundle).is_some() {
                return Err(RegisterPluginError::AlreadyExistsIDAndVersion(
                    bundle.id.clone(),
                    bundle.version.clone(),
                ));
            }

            // Ищем подходящий менеджер
            let plugin_format = bundle.format.clone();
            let manager = loader
                .get_manager_mut(plugin_format.as_str())
                .ok_or(RegisterPluginError::UnknownManagerFormat(plugin_format))?;

            // Менеджер регистрирует плагин
            let info = manager.register_plugin(RegisterPluginContext {
                path: &path,
                bundle: &bundle,
            })?;
            let plugin_info = PluginInfo { path, bundle, info };

            // Регистрируем плагин
            let plugin = Plugin::<'a>::new(Ptr::<'a>::new(manager), plugin_info);
            let bundle = plugin.info.bundle.clone();
            loader.plugins.push(plugin);

            return Ok(bundle);
        } else {
            return Err(RegisterPluginError::UnknownManagerFormat("".to_string()));
        }
    }

    //TODO: Добавить принудительную версию метода
    pub fn unregister_plugin<'a, T: Send + Sync>(
        loader: &mut super::Loader<'a, T>,
        plugin: *mut Plugin<'a, T>,
    ) -> Result<(), UnregisterPluginError> {
        unload_plugin(loader, plugin.clone())?;

        unsafe { &*plugin }
            .manager
            .as_mut()
            .unregister_plugin(Ptr::new(plugin))?;

        loader.plugins.retain(|p| *p != *unsafe { &*plugin });

        Ok(())
    }

    pub fn load_plugin<T: Send + Sync>(
        loader: &mut super::Loader<'_, T>,
        plugin: *mut Plugin<'_, T>,
    ) -> Result<(), LoadPluginError> {
        if unsafe { &*plugin }.is_load {
            return Ok(());
        }

        // Загружаем зависимости
        let info = &unsafe { &*plugin }.info;
        let depends_iter = info
            .info
            .depends
            .iter()
            .map(|d| (true, d))
            .chain(info.info.optional_depends.iter().map(|d| (false, d)));

        let mut not_found_depends = vec![];
        for (is_depend, depend) in depends_iter {
            if let Some(_) = loader.plugins.iter().find(|p| p.info.bundle == *depend) {
                if let Err(e) = loader.load_plugin(&depend.id, &depend.version) {
                    return Err(LoadPluginError::LoadDependency {
                        depend: depend.clone(),
                        error: Box::new(e),
                    });
                }
            } else if is_depend {
                not_found_depends.push(depend.clone());
            }
        }

        if !not_found_depends.is_empty() {
            return Err(LoadPluginError::NotFoundDependencies(not_found_depends));
        }

        // Загружаем плагин
        unsafe { &*plugin }
            .manager
            .as_mut()
            .load_plugin(LoadPluginContext::new(
                Ptr::new(plugin),
                Ptr::new(&mut loader.registry as *mut Registry<T>),
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
                    .any(|req| *req.name == request.name())
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

    //TODO: Добавить принудительную версию метода
    pub fn unload_plugin<T: Send + Sync>(
        loader: &super::Loader<'_, T>,
        plugin: *mut Plugin<'_, T>,
    ) -> Result<(), UnloadPluginError> {
        if unsafe { &*plugin }.is_load {
            let bundle = &unsafe { &*plugin }.info.bundle;

            // Проверяем, что плагин не используется как зависимость загруженными плагинами
            loader.plugins.iter().try_for_each(|plug| {
                let plug_info = &plug.info;

                let find_depend = plug_info
                    .info
                    .depends
                    .iter()
                    .chain(plug_info.info.optional_depends.iter())
                    .find(|depend| **depend == *bundle)
                    .is_some();
                match plug.is_load && find_depend {
                    true => Err(UnloadPluginError::CurrentlyUsesDepend {
                        plugin: plug_info.bundle.clone(),
                        depend: bundle.clone(),
                    }),
                    false => Ok(()),
                }
            })?;

            unsafe { &*plugin }
                .manager
                .as_mut()
                .unload_plugin(Ptr::new(plugin))?;
        }

        unsafe { &mut *plugin }.is_load = false;

        Ok(())
    }
}
