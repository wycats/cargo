use core::{VersionReq,SourceId};
use util::CargoResult;

#[deriving(PartialEq,Clone,Show)]
pub struct Dependency {
    name: String,
    namespace: SourceId,
    req: VersionReq,
    envs: Option<Vec<String>>
}

impl Dependency {
    pub fn parse(name: &str, version: Option<&str>,
                 namespace: &SourceId, envs: Option<Vec<String>>) -> CargoResult<Dependency>
    {
        let version = match version {
            Some(v) => try!(VersionReq::parse(v)),
            None => VersionReq::any()
        };

        Ok(Dependency {
            name: name.to_str(),
            namespace: namespace.clone(),
            req: version,
            envs: envs
        })
    }

    pub fn get_version_req<'a>(&'a self) -> &'a VersionReq {
        &self.req
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        self.name.as_slice()
    }

    pub fn get_namespace<'a>(&'a self) -> &'a SourceId {
        &self.namespace
    }
}

#[deriving(PartialEq,Clone,Encodable)]
pub struct SerializedDependency {
    name: String,
    req: String
}

impl SerializedDependency {
    pub fn from_dependency(dep: &Dependency) -> SerializedDependency {
        SerializedDependency {
            name: dep.get_name().to_str(),
            req: dep.get_version_req().to_str()
        }
    }
}
