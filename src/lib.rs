pub mod api;
pub mod capi;
mod driver;

#[macro_use]
extern crate log;

use api::DriverType;
use api::Introspectable;
use driver::dummy::Dummy;
#[cfg(feature = "hyper-v")]
use driver::hyperv::HyperV;
#[cfg(feature = "kvm")]
use driver::kvm::Kvm;
#[cfg(feature = "virtualbox")]
use driver::virtualbox::VBox;
#[cfg(feature = "xen")]
use driver::xen::Xen;
#[cfg(feature = "kvm")]
use kvmi::create_kvmi;

#[allow(unreachable_code)]
pub fn init(domain_name: &str, driver_type: Option<DriverType>) -> Box<dyn Introspectable> {
    debug!("Microvmi init");
    match driver_type {
        Some(drv_type) => match drv_type {
            DriverType::Dummy => Box::new(Dummy::new(domain_name)) as Box<dyn Introspectable>,
            #[cfg(feature = "hyper-v")]
            DriverType::HyperV => Box::new(HyperV::new(domain_name)) as Box<dyn Introspectable>,
            #[cfg(feature = "kvm")]
            DriverType::KVM => create_kvm(domain_name),
            #[cfg(feature = "virtualbox")]
            DriverType::VirtualBox => Box::new(VBox::new(domain_name)) as Box<dyn Introspectable>,
            #[cfg(feature = "xen")]
            DriverType::Xen => create_xen(domain_name),
        },
        None => {
            // test Hyper-V
            #[cfg(feature = "hyper-v")]
            {
                return Box::new(HyperV::new(domain_name)) as Box<dyn Introspectable>;
            }

            // test KVM
            #[cfg(feature = "kvm")]
            {
                return create_kvm(domain_name);
            }

            // test VirtualBox
            #[cfg(feature = "virtualbox")]
            {
                return Box::new(VBox::new(domain_name)) as Box<dyn Introspectable>;
            }

            // test Xen
            #[cfg(feature = "xen")]
            {
                return create_xen(domain_name);
            }
            // return Dummy if no other driver has been compiled
            Box::new(Dummy::new(domain_name)) as Box<dyn Introspectable>
        }
    }
}

#[cfg(feature = "kvm")]
fn create_kvm(domain_name: &str) -> Box<dyn Introspectable> {
    Box::new(Kvm::new(domain_name, create_kvmi()).unwrap()) as Box<dyn Introspectable>
}

#[cfg(feature = "xen")]
fn create_xen(domain_name: &str) -> Box<dyn Introspectable> {
    Box::new(
        Xen::new(domain_name).unwrap(),
        XenControl::new(None, None, 0).unwrap(),
    ) as Box<dyn Introspectable>
}
