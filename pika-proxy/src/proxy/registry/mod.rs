use registry_adapter::RegistryAdapter;

mod etcd_adapter;
mod redis_adapter;
mod registry_adapter;

pub struct Registry {
    adapter: Box<dyn RegistryAdapter>,
}
