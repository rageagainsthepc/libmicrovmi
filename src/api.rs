use std::error::Error;

pub enum DriverType {
    Dummy,
    #[cfg(feature="xen")]
    Xen,
    #[cfg(feature="kvm")]
    KVM,
}

pub trait Introspectable {
    // read physical memory
    fn read_physical(&self, paddr: u64, buf: &mut [u8]) -> Result<(),Box<dyn Error>>;

    // get max physical address
    fn get_max_physical_addr(&self) -> Result<u64,Box<dyn Error>>;

    // pause the VM
    fn pause(&mut self) -> Result<(),Box<dyn Error>>;

    // resume the VM
    fn resume(&mut self) -> Result<(),Box<dyn Error>>;
}