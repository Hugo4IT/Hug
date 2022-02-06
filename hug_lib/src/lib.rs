#[macro_export]
macro_rules! hug_module {
    ($init:path, $deinit:path) => {
        use hug_lib::HugModule;

        #[no_mangle]
        unsafe extern "C" fn __HUG_MODULE_INIT(module: &mut HugModule) {
            $init(module);
        }

        #[no_mangle]
        unsafe extern "C" fn __HUG_MODULE_DEINIT(module: &mut HugModule) {
            $deinit(module);
        }
    };
}

pub struct HugFunction {

}

pub struct HugModule {
    functions: Vec<HugFunction>,
}

impl HugModule {
    pub fn new() -> HugModule {
        HugModule {
            functions: Vec::new(),
        }
    }
}