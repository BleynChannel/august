#[derive(Debug, Clone)]
pub enum RegisterPluginError<'a>
{
	NotFound,
	UnpackError(&'a str),
	DoesNotContainConfig,
}

#[derive(Debug, Clone)]
pub enum LoadPluginError<'a> {
	NotFoundDependencies(&'a [&'a str]),
	LoadDependency((&'a str, &'a LoadPluginError<'a>)),
	UnknownManagerFormat(&'a str),
}