use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;

use crate::{
    utils::{
        bundle::Bundle, LoadPluginError, PluginCallRequest, Ptr, RegisterManagerError,
        RegisterPluginError, StopLoaderError, UnloadPluginError, UnregisterManagerError,
        UnregisterPluginError,
    },
    variable::Variable,
    LoaderContext, Manager, Plugin, PluginInfo, Registry, Requests,
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

    pub unsafe fn forced_register_manager(
        &mut self,
        manager: Box<dyn Manager<'a, T>>,
    ) -> Result<(), RegisterManagerError> {
        private_loader::forced_register_manager(self, manager)
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

    pub unsafe fn forced_unregister_manager(
        &mut self,
        index: usize,
    ) -> Result<(), UnregisterManagerError> {
        private_loader::forced_unregister_manager(&mut self.managers, index)
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

    pub unsafe fn forced_register_plugin(
        &mut self,
        manager: &mut Box<dyn Manager<'a, T>>,
        plugin_info: PluginInfo,
    ) -> Result<Bundle, RegisterPluginError> {
        private_loader::forced_register_plugin(&mut self.plugins, Ptr::new(manager), plugin_info)
    }

    pub fn unregister_plugin(
        &mut self,
        id: &str,
        version: &Version,
    ) -> Result<(), UnregisterPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == (id, version))
            .ok_or(UnregisterPluginError::NotFound)?;
        private_loader::unregister_plugin(&mut self.plugins, index)
    }

    pub fn unregister_plugin_by_bundle(
        &mut self,
        bundle: &Bundle,
    ) -> Result<(), UnregisterPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == *bundle)
            .ok_or(UnregisterPluginError::NotFound)?;
        private_loader::unregister_plugin(&mut self.plugins, index)
    }

    pub unsafe fn forced_unregister_plugin(
        &mut self,
        index: usize,
    ) -> Result<(), UnregisterPluginError> {
        private_loader::forced_unregister_plugin(&mut self.plugins, index)
    }

    pub fn load_plugin(&mut self, id: &str, version: &Version) -> Result<(), LoadPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == (id, version))
            .ok_or(LoadPluginError::NotFound)?;
        private_loader::load_plugin(self, index)
    }

    pub fn load_plugin_by_bundle(&mut self, bundle: &Bundle) -> Result<(), LoadPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == *bundle)
            .ok_or(LoadPluginError::NotFound)?;
        private_loader::load_plugin(self, index)
    }

    pub unsafe fn forced_load_plugin(&mut self, index: usize) -> Result<(), LoadPluginError> {
        private_loader::forced_load_plugin(self, index)
    }

    pub fn unload_plugin(&mut self, id: &str, version: &Version) -> Result<(), UnloadPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == (id, version))
            .ok_or(UnloadPluginError::NotFound)?;
        private_loader::unload_plugin(&mut self.plugins, index)
    }

    pub fn unload_plugin_by_bundle(&mut self, bundle: &Bundle) -> Result<(), UnloadPluginError> {
        let index = self
            .plugins
            .iter()
            .position(|plugin| *plugin == *bundle)
            .ok_or(UnloadPluginError::NotFound)?;
        private_loader::unload_plugin(&mut self.plugins, index)
    }

    pub unsafe fn forced_unload_plugin(&mut self, index: usize) -> Result<(), UnloadPluginError> {
        private_loader::forced_unload_plugin(&mut self.plugins, index)
    }

    //TODO: Добавить функцию register_plugins для регистрации нескольких плагинов

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
    ) -> Result<Vec<Bundle>, (Option<RegisterPluginError>, Option<LoadPluginError>)>
    where
        P: IntoIterator<Item = &'b str>,
    {
        let mut infos = vec![];

        for path in plugin_paths {
            infos.push(self.register_plugin(path).map_err(|e| (Some(e), None))?);
        }

        // Перебор плагинов, которые не являются зависимостями для других плагинов
        let mut result = vec![];

        for (index, plugin) in self.plugins.iter().enumerate() {
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
                result.push(index);
            }
        }

        result.into_iter().try_for_each(|index| {
            private_loader::load_plugin(self, index).map_err(|e| (None, Some(e)))
        })?;

        Ok(infos)
    }

    //TODO: Добавить функцию load_only_used_plugins для загрузки только используемых плагинов

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
        //TODO: Выполнять только у плагина с большей версией
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

impl<T: Send + Sync> Drop for Loader<'_, T> {
    fn drop(&mut self) {
        self.stop().unwrap();
    }
}

mod private_loader {
    use std::path::Path;

    use crate::{
        utils::{
            bundle::Bundle, LoadPluginError, Ptr, RegisterManagerError, RegisterPluginError,
            StopLoaderError, UnloadPluginError, UnregisterManagerError, UnregisterPluginError,
        },
        Depend, LoadPluginContext, Manager, Plugin, PluginInfo, RegisterPluginContext,
    };

    pub fn stop_plugins<T: Send + Sync>(
        loader: &mut super::Loader<'_, T>,
    ) -> Result<(), StopLoaderError> {
        // Сортируем плагины в порядке их зависимостей
        let sort_plugins = sort_plugins(
            &loader.plugins,
            loader
                .plugins
                .iter()
                .enumerate()
                .map(|(index, _)| index)
                .collect(),
        );

        let mut errors = vec![];

        // Выгружаем плагины
        for index in sort_plugins.iter() {
            if let Err(e) = forced_unload_plugin(&mut loader.plugins, index.clone()) {
                errors.push(UnregisterPluginError::UnloadError(e));
            }
        }

        for _ in 0..loader.plugins.len() {
            if let Err(e) = forced_unregister_plugin(&mut loader.plugins, 0_usize) {
                //TODO: Добавить debug вывод
                errors.push(e);
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
            if let Err(e) = forced_unregister_manager(&mut loader.managers, 0_usize) {
                errors.push(e);
            }
        }

        match !errors.is_empty() {
            true => Err(StopLoaderError::UnregisterManagerFailed(errors)),
            false => Ok(()),
        }
    }

    /*
        TODO: Изменить сортировку плагинов.
        В аргументы функции необходимо передавать список всех плагинов
        и необязательный набор индексов плагинов для точечной сортировки.
        На выходе должен выдаваться индекс начала сортированных плагинов.

        Механика сортировки заключается в смещение в конец списка плагинов
        попутно сортируя их.
    */
    // pub fn sort_plugins<'a, T: Send + Sync>(
    //     plugins: &mut Vec<Plugin<'a, T>>,
    //     plugins_set: Option<Vec<usize>>,
    // ) -> usize

    // Продвинутая древовидная сортировка
    pub fn sort_plugins<'a, T: Send + Sync>(
        plugins: &Vec<Plugin<'a, T>>,
        plugins_set: Vec<usize>,
    ) -> Vec<usize> {
        let mut result = vec![];

        'outer: for index in plugins_set.iter() {
            let find_plugin = plugins.iter().enumerate().find_map(|(i, pl)| {
                pl.info
                    .info
                    .depends
                    .iter()
                    .chain(pl.info.info.optional_depends.iter())
                    .any(|d| *d == plugins[*index])
                    .then_some(i)
            });

            if find_plugin.is_some()
                && plugins_set
                    .iter()
                    .find(|i| **i == find_plugin.unwrap())
                    .is_some()
            {
                continue 'outer;
            }

            sort_pick(plugins, &plugins_set, index, &mut result);
        }

        result
    }

    pub fn sort_pick<'a, T: Send + Sync>(
        plugins: &Vec<Plugin<'a, T>>,
        plugins_set: &Vec<usize>,
        index: &usize,
        result: &mut Vec<usize>,
    ) {
        result.push(index.clone());

        let plugin_info = &plugins[*index].info;
        let depends = plugin_info
            .info
            .depends
            .iter()
            .chain(plugin_info.info.optional_depends.iter());
        'outer: for depend in depends {
            if !result.iter().any(|inx| plugins[*inx] == *depend) {
                let mut plugin = None;

                for index in plugins_set.iter() {
                    let plug_info = &plugins[*index].info;
                    if plug_info.bundle == *depend {
                        plugin = Some(index);
                        continue;
                    }

                    if !result
                        .iter()
                        .any(|inx| plugins[*inx].info.bundle == plug_info.bundle)
                        && (plug_info.info.depends.contains(depend)
                            || plug_info.info.optional_depends.contains(depend))
                    {
                        continue 'outer;
                    }
                }

                if let Some(index) = plugin {
                    sort_pick(plugins, plugins_set, index, result);
                }
            }
        }
    }

    pub fn forced_register_manager<'a, T: Send + Sync>(
        loader: &mut super::Loader<'a, T>,
        mut manager: Box<dyn Manager<'a, T>>,
    ) -> Result<(), RegisterManagerError> {
        manager.as_mut().register_manager(Ptr::<'a>::new(loader))?;
        loader.managers.push(manager);
        Ok(())
    }

    pub fn register_manager<'a, T: Send + Sync>(
        loader: &mut super::Loader<'a, T>,
        manager: Box<dyn Manager<'a, T>>,
    ) -> Result<(), RegisterManagerError> {
        if let Some(_) = loader.managers.iter().find(|m| manager == **m) {
            return Err(RegisterManagerError::AlreadyOccupiedFormat(
                manager.format().to_string(),
            ));
        }

        forced_register_manager(loader, manager)
    }

    pub fn forced_unregister_manager<T: Send + Sync>(
        managers: &mut Vec<Box<dyn Manager<'_, T>>>,
        index: usize,
    ) -> Result<(), UnregisterManagerError> {
        match managers.remove(index).unregister_manager() {
            Ok(_) => Ok(()),
            Err(e) => Err(UnregisterManagerError::UnregisterManagerByManager(e)),
        }
    }

    pub fn unregister_manager<T: Send + Sync>(
        loader: &mut super::Loader<'_, T>,
        index: usize,
    ) -> Result<(), UnregisterManagerError> {
        let manager = &loader.managers[index];

        // Получаем все плагины, относящиеся к менеджеру
        let plugins_from_manager = loader
            .plugins
            .iter()
            .enumerate()
            .filter_map(
                |(index, plugin)| match *plugin.manager.as_ref() == *manager {
                    true => Some(index),
                    false => None,
                },
            )
            .collect();

        // Сортируем плагины менеджера в порядке их зависимостей
        let sort_plugins = sort_plugins(&loader.plugins, plugins_from_manager);

        // Выгружаем плагины
        for index in sort_plugins.iter() {
            unload_plugin(&mut loader.plugins, index.clone()).map_err(|e| {
                UnregisterManagerError::UnregisterPlugin(UnregisterPluginError::UnloadError(e))
            })?;
        }

        let mut old_indexs = vec![];
        let mut sort_plugins = sort_plugins.into_iter();

        while let Some(index) = sort_plugins.next() {
            let swap = old_indexs
                .iter()
                .fold(0, |acc, i| if index > *i { acc + 1 } else { acc });

            if let Err(e) = forced_unregister_plugin(&mut loader.plugins, index - swap) {
                return Err(UnregisterManagerError::UnregisterPlugin(e));
            }

            old_indexs.push(index);
        }

        // Выгружаем менеджер
        forced_unregister_manager(&mut loader.managers, index)
    }

    pub fn forced_register_plugin<'a, T: Send + Sync>(
        plugins: &mut Vec<Plugin<'a, T>>,
        manager: Ptr<'a, Box<dyn Manager<'a, T>>>,
        plugin_info: PluginInfo,
    ) -> Result<Bundle, RegisterPluginError> {
        let bundle = plugin_info.bundle.clone();
        plugins.push(Plugin::<'a>::new(manager, plugin_info));
        Ok(bundle)
    }

    pub fn register_plugin<'a, T: Send + Sync>(
        loader: &mut super::Loader<'a, T>,
        path: &str,
    ) -> Result<Bundle, RegisterPluginError> {
        let path = Path::new(path).to_path_buf();

        if !path.is_dir() {
            return Err(RegisterPluginError::NotFound);
        }

        if let None = path.extension() {
            return Err(RegisterPluginError::UnknownManagerFormat("".to_string()));
        }

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
        let manager = Ptr::<'a>::new(manager);
        forced_register_plugin(&mut loader.plugins, manager, plugin_info)
    }

    pub fn forced_unregister_plugin<T: Send + Sync>(
        plugins: &mut Vec<Plugin<'_, T>>,
        index: usize,
    ) -> Result<(), UnregisterPluginError> {
        let plugin = plugins.remove(index);
        plugin.manager.as_mut().unregister_plugin(&plugin)?;
        Ok(())
    }

    pub fn unregister_plugin<'a, T: Send + Sync>(
        plugins: &mut Vec<Plugin<'_, T>>,
        index: usize,
    ) -> Result<(), UnregisterPluginError> {
        unload_plugin(plugins, index)?;
        forced_unregister_plugin(plugins, index)
    }

    pub fn forced_load_plugin<T: Send + Sync>(
        loader: &mut super::Loader<'_, T>,
        index: usize,
    ) -> Result<(), LoadPluginError> {
        let manager = Ptr::new(loader.plugins[index].manager.as_ptr());

        manager.as_mut().load_plugin(LoadPluginContext::new(
            &mut loader.plugins[index],
            &loader.requests,
            &loader.registry,
        ))?;

        loader.plugins[index].is_load = true;

        Ok(())
    }

    fn load_depends<'a, T, I>(
        loader: &'a mut super::Loader<'_, T>,
        depends_iter: I,
    ) -> Result<Vec<Depend>, LoadPluginError>
    where
        T: Send + Sync,
        I: IntoIterator<Item = (bool, Depend)>,
    {
        let mut not_found_depends = vec![];

        for (is_depend, depend) in depends_iter.into_iter() {
            if let Some(_) = loader.plugins.iter().find(|p| p.info.bundle == depend) {
                if let Err(e) = loader.load_plugin(&depend.id, &depend.version) {
                    return Err(LoadPluginError::LoadDependency {
                        depend: depend,
                        error: Box::new(e),
                    });
                }
            } else if is_depend {
                not_found_depends.push(depend);
            }
        }
        Ok(not_found_depends)
    }

    fn check_requests<T: Send + Sync>(
        loader: &mut super::Loader<'_, T>,
        index: usize,
    ) -> Vec<String> {
        let mut plugin_requests = loader.plugins[index].requests.iter();
        loader
            .requests
            .iter()
            .filter_map(|req| match plugin_requests.any(|r| r.name() == req.name) {
                true => None,
                false => Some(req.name.clone()),
            })
            .collect()
    }

    pub fn load_plugin<T: Send + Sync>(
        loader: &mut super::Loader<'_, T>,
        index: usize,
    ) -> Result<(), LoadPluginError> {
        if loader.plugins[index].is_load {
            return Ok(());
        }

        // Загружаем зависимости
        let info = &loader.plugins[index].info;
        let depends_iter = info
            .info
            .depends
            .clone()
            .into_iter()
            .map(|d| (true, d))
            .chain(
                info.info
                    .optional_depends
                    .clone()
                    .into_iter()
                    .map(|d| (false, d)),
            );
        let not_found_depends = load_depends(loader, depends_iter)?;

        if !not_found_depends.is_empty() {
            return Err(LoadPluginError::NotFoundDependencies(not_found_depends));
        }

        // Загружаем плагин
        forced_load_plugin(loader, index)?;

        // Проверяем наличие запрашиваемых функций
        let not_found_requests = check_requests(loader, index);

        if !not_found_requests.is_empty() {
            loader.plugins[index].is_load = false;
            return Err(LoadPluginError::RequestsNotFound(not_found_requests));
        }

        Ok(())
    }

    pub fn forced_unload_plugin<T: Send + Sync>(
        plugins: &mut Vec<Plugin<'_, T>>,
        index: usize,
    ) -> Result<(), UnloadPluginError> {
        if plugins[index].is_load {
            plugins[index]
                .manager
                .as_mut()
                .unload_plugin(&plugins[index])?;
        }

        plugins[index].is_load = false;

        Ok(())
    }

    pub fn unload_plugin<'a, T: Send + Sync>(
        plugins: &mut Vec<Plugin<'_, T>>,
        index: usize,
    ) -> Result<(), UnloadPluginError> {
        if plugins[index].is_load {
            let bundle = &plugins[index].info.bundle;

            // Проверяем, что плагин не используется как зависимость загруженными плагинами
            plugins.iter().try_for_each(|plug| {
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
        }

        forced_unload_plugin(plugins, index)
    }
}
