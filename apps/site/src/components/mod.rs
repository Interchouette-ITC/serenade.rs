use rangular_host::{Host, HostError, Value};

pub mod architecture;
pub mod components;
pub mod getting_started;
pub mod home;
pub mod site_footer;
pub mod site_header;

pub(crate) struct StaticHost;

impl Host for StaticHost {
    fn get(&self, name: &str) -> Option<Value> {
        let value = match name {
            "space" => " ",
            "architectureTree" => {
                "Your application\n├── Kernel        the conductor\n├── Bundles       your arrangements\n├── Components    instruments, ready to play\n└── Adapters      your choice of venue"
            }
            "setupCommands" => {
                "git clone https://github.com/Interchouette-ITC/Serenade.git\ncd Serenade\nrustup target add wasm32-unknown-unknown\ncargo install trunk"
            }
            "coreCommands" => "make test\nmake demo",
            "spaCommands" => {
                "make spa\npython3 -m http.server 4173 --bind 127.0.0.1 --directory examples/demo-monorepo/apps/demo-spa/dist"
            }
            "ssgCommands" => {
                "make ssg\npython3 -m http.server 4173 --bind 127.0.0.1 --directory examples/demo-monorepo/apps/demo-ssg/dist"
            }
            _ => return None,
        };
        Some(Value::Str(value.to_owned()))
    }

    fn call(&mut self, name: &str, _: &[Value]) -> Result<Value, HostError> {
        Err(HostError::UnknownMember(name.to_owned()))
    }
}
