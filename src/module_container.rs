use crate::modules::{Module, ModuleParts};
use glib::IsA;
use gtk::Widget;
use std::fmt::Debug;

struct ModuleContainer<'a> {
    container: &'a gtk::Box,
}

impl<'a> ModuleContainer<'a> {
    fn new(container: &'a gtk::Box) -> Self {
        Self { container }
    }
}

// trait Foo {
//     fn create();
// }
//
// impl<W> Foo for Module<W>
// where
//     W: IsA<Widget>,
// {
//     fn create() {
//         todo!()
//     }
// }

trait ModuleFactory {
    fn create<TModule, TWidget, TSend, TRev>(
        module: TModule,
    ) -> color_eyre::Result<ModuleParts<TWidget>>
    where
        TModule: Module<TWidget, SendMessage = TSend, ReceiveMessage = TRev>,
        TWidget: IsA<gtk::Widget>,
        TSend: Debug + Clone + Send + 'static;
}

struct BarModuleFactory {}

impl ModuleFactory for BarModuleFactory {
    fn create<TModule, TWidget, TSend, TRev>(
        module: TModule,
    ) -> color_eyre::Result<ModuleParts<TWidget>>
    where
        TModule: Module<TWidget, SendMessage = TSend, ReceiveMessage = TRev>,
        TWidget: IsA<Widget>,
        TSend: Debug + Clone + Send + 'static,
    {

        todo!()
    }
}