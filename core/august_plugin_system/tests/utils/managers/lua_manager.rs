use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    vec,
};

use august_plugin_system::{
    context::LoadPluginContext,
    function::{Arg, Function},
    utils::{FunctionResult, Ptr},
    variable::Variable,
    Manager, Plugin, PluginInfo, Registry, Requests,
};
use rlua::{Context, Lua, MultiValue, ToLua, Value};

pub struct LuaPluginManager {
    lua_refs: HashMap<String, Arc<Mutex<Lua>>>,
}

impl<'a> Manager<'a> for LuaPluginManager {
    fn format(&self) -> &str {
        "fpl"
    }

    fn register_plugin(&mut self, path: &PathBuf) -> FunctionResult<PluginInfo> {
        let info = PluginInfo::new(
            path.parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );

        println!("FunctionPluginManager::register_plugin - {}", info.id);

        Ok(info)
    }

    fn load_plugin(&mut self, context: LoadPluginContext) -> FunctionResult<()> {
        let id = context.plugin().info().id.clone();

        println!("FunctionPluginManager::load_plugin - {:?}", id.clone());

        let lua = Lua::new();
        let (mut context, requests) = lua.context(|lua_context| -> FunctionResult<_> {
            self.registry_to_lua(lua_context, &*context.registry())?;
            self.load_src(lua_context, context.plugin().path().clone())?;

            let requests = self.register_requests(lua_context, &*context.requests())?;

            Ok((context, requests))
        })?;

        self.lua_refs.insert(id.clone(), Arc::new(Mutex::new(lua)));
        let lua_ref = self.lua_refs.get(&id).unwrap();

        for request in requests {
            let lua = lua_ref.clone();
            context.register_request(request, vec![Box::new(lua)])?;
        }

        Ok(())
    }

    fn unload_plugin(&mut self, plugin: Ptr<'a, Plugin>) -> FunctionResult<()> {
        println!(
            "FunctionPluginManager::unload_plugin - {:?}",
            plugin.as_ref().info().id
        );

        Ok(drop(self.lua_refs.remove(&plugin.as_ref().info().id)))
    }
}

impl LuaPluginManager {
	#[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            lua_refs: HashMap::new(),
        }
    }

    // Добавление функций из реестра
    fn registry_to_lua(&self, lua_context: Context, registry: &Registry) -> FunctionResult<()> {
        let globals = lua_context.globals();

        for function in registry.iter() {
            let function_name = function.name();
            let function = function.clone();
            let f = lua_context.create_function(move |ctx, lua_args: MultiValue| {
                let mut args = vec![];
                for arg in lua_args.iter().map(Self::lua2august) {
                    args.push(arg?);
                }

                let output = function
                    .call(&[], &args)
                    .map_err(|e| rlua::Error::RuntimeError(e.to_string()))?
                    .map(|var| Self::august2lua(&var, ctx.clone()));

                match output {
                    Some(out) => Ok(out?),
                    None => Ok(Value::Nil),
                }
            })?;

            globals.set(function_name, f)?;
        }

        Ok(())
    }

    // Загрузка исходного кода плагина
    fn load_src(&self, lua_context: Context, path: PathBuf) -> FunctionResult<()> {
        let src = std::fs::read_to_string(path.join("main.lua"))?;
        lua_context.load(&src).exec()?;
        Ok(())
    }

    // Регистрация заказываемых функций
    fn register_requests(
        &self,
        lua_context: Context,
        requests: &Requests,
    ) -> FunctionResult<Vec<Function>> {
        let globals = lua_context.globals();

        let mut result = vec![];

        for request in requests {
            match globals.get(request.name().clone())? {
                Value::Function(_) => {
                    let request_name = request.name();
                    let function = Function::new(
                        request_name.clone(),
                        "It's description".to_string(),
                        request
                            .inputs()
                            .iter()
                            .enumerate()
                            .map(|(index, ty)| Arg::new(format!("arg_{}", index), ty.clone()))
                            .collect(),
                        request
                            .output()
                            .map(|output| Arg::new("output".to_string(), output.clone())),
                        move |exts, args| {
                            let request_name = request_name.clone();

                            let arc_lua = exts[0].downcast_ref::<Arc<Mutex<Lua>>>().unwrap();
                            let lua = arc_lua.lock().map_err(|e| e.to_string())?;
                            Ok(lua.context(move |ctx| -> FunctionResult<_> {
                                let mut lua_args = vec![];
                                for arg in args {
                                    lua_args.push(Self::august2lua(arg, ctx)?);
                                }

                                let f: rlua::Function = ctx.globals().get(request_name)?;

                                match f.call::<_, Value>(MultiValue::from_vec(lua_args))? {
                                    Value::Nil => Ok(None),
                                    value => Ok(Some(Self::lua2august(&value)?)),
                                }
                            })?)
                        },
                    );
                    result.push(function);
                }
                Value::Nil => {
                    return Err(format!("Функции `{}` не существует", request.name()).into())
                }
                _ => return Err(format!("`{}` должна быть функцией", request.name()).into()),
            }
        }

        Ok(result)
    }

    fn lua2august(arg: &Value) -> rlua::Result<Variable> {
        match arg {
            Value::Nil => Ok(Variable::Null),
            Value::Boolean(var) => Ok(Variable::Bool(*var)),
            Value::LightUserData(_) => Err(rlua::Error::RuntimeError(
                "Неподдерживаемый тип переменной".to_string(),
            )),
            Value::Integer(var) => Ok(Variable::I32(*var as i32)),
            Value::Number(var) => Ok(Variable::F32(*var as f32)),
            Value::String(var) => Ok(Variable::String(var.to_str()?.to_string())),
            Value::Table(var) => {
                let mut list = vec![];
                for pair in var.clone().pairs::<Value, Value>() {
                    list.push(Self::lua2august(&pair?.1)?);
                }
                Ok(Variable::List(list))
            }
            Value::Function(_) => Err(rlua::Error::RuntimeError(
                "Неподдерживаемый тип переменной".to_string(),
            )),
            Value::Thread(_) => Err(rlua::Error::RuntimeError(
                "Неподдерживаемый тип переменной".to_string(),
            )),
            Value::UserData(_) => Err(rlua::Error::RuntimeError(
                "Неподдерживаемый тип переменной".to_string(),
            )),
            Value::Error(err) => Err(err.clone()),
        }
    }

    fn august2lua<'lua>(var: &Variable, context: Context<'lua>) -> rlua::Result<Value<'lua>> {
        match var {
            Variable::Null => Ok(Value::Nil),
            Variable::I8(var) => var.to_lua(context),
            Variable::I16(var) => var.to_lua(context),
            Variable::I32(var) => var.to_lua(context),
            Variable::I64(var) => var.to_lua(context),
            Variable::U8(var) => var.to_lua(context),
            Variable::U16(var) => var.to_lua(context),
            Variable::U32(var) => var.to_lua(context),
            Variable::U64(var) => var.to_lua(context),
            Variable::F32(var) => var.to_lua(context),
            Variable::F64(var) => var.to_lua(context),
            Variable::Bool(var) => var.to_lua(context),
            Variable::Char(var) => var.to_string().to_lua(context),
            Variable::String(var) => var.clone().to_lua(context),
            Variable::List(var) => {
                let mut list = vec![];
                for v in var {
                    list.push(Self::august2lua(v, context)?);
                }
                list.to_lua(context)
            }
        }
    }
}
