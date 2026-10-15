use pyo3::{prelude::*, types::PyModuleMethods};
use trinity_meta::TrinityRegistry;
use trinity_rs::TrinityRuntime;
use trinityal::{
    ALResult, ALResultCategory, E_INVALIDARG, PixelFormat, TextureType, Tr2BitmapDimensions,
};

#[pyclass(name = "ALResult", module = "_trinity_rs")]
#[derive(Clone)]
pub struct PyALResult {
    inner: ALResult,
}

#[pymethods]
impl PyALResult {
    #[new]
    #[pyo3(signature = (result = 0))]
    fn new(result: i32) -> Self {
        Self {
            inner: ALResult::new(result),
        }
    }

    #[pyo3(name = "GetResult")]
    fn get_result(&self) -> i32 {
        self.inner.get_result()
    }

    #[pyo3(name = "GetCategory")]
    fn get_category(&self) -> &'static str {
        match self.inner.get_category() {
            ALResultCategory::Success => "SUCCESS",
            ALResultCategory::InvalidCall => "INVALID_CALL",
            ALResultCategory::OutOfMemory => "OUT_OF_MEMORY",
            ALResultCategory::DeviceLost => "DEVICE_LOST",
            ALResultCategory::Other => "OTHER",
        }
    }

    #[pyo3(name = "IsSuccess")]
    fn is_success(&self) -> bool {
        self.inner.is_success()
    }

    fn __int__(&self) -> i32 {
        self.inner.get_result()
    }
}

#[pyclass(name = "Tr2BitmapDimensions", module = "_trinity_rs")]
#[derive(Clone, Default)]
pub struct PyTr2BitmapDimensions {
    inner: Tr2BitmapDimensions,
}

#[pymethods]
impl PyTr2BitmapDimensions {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[getter]
    fn width(&self) -> u32 {
        self.inner.width
    }
    #[setter]
    fn set_width(&mut self, value: u32) {
        self.inner.width = value;
    }

    #[getter]
    fn height(&self) -> u32 {
        self.inner.height
    }
    #[setter]
    fn set_height(&mut self, value: u32) {
        self.inner.height = value;
    }

    #[getter]
    fn depth(&self) -> u32 {
        self.inner.volume_depth
    }
    #[setter]
    fn set_depth(&mut self, value: u32) {
        self.inner.volume_depth = value;
    }

    #[getter]
    fn mip_count(&self) -> u32 {
        self.inner.mip_count
    }
    #[setter]
    fn set_mip_count(&mut self, value: u32) {
        self.inner.mip_count = value;
    }

    #[pyo3(name = "GetTrueMipCount")]
    fn get_true_mip_count(&self) -> u32 {
        self.inner.get_true_mip_count()
    }

    #[pyo3(name = "GetMipWidth")]
    fn get_mip_width(&self, mip: u32) -> u32 {
        self.inner.get_mip_width(mip)
    }

    #[pyo3(name = "GetMipHeight")]
    fn get_mip_height(&self, mip: u32) -> u32 {
        self.inner.get_mip_height(mip)
    }

    #[pyo3(name = "Set2D")]
    fn set_2d(&mut self, width: u32, height: u32, mip_count: u32) {
        self.inner.texture_type = TextureType::Texture2D;
        self.inner.width = width;
        self.inner.height = height;
        self.inner.volume_depth = 0;
        self.inner.mip_count = mip_count;
    }

    #[pyo3(name = "SetFormatB8G8R8A8")]
    fn set_format_bgra8(&mut self) {
        self.inner.format = PixelFormat::B8G8R8A8Unorm;
    }
}

#[pyclass(name = "Runtime", module = "_trinity_rs", unsendable)]
pub struct PyTrinityRuntime {
    runtime: TrinityRuntime,
}

#[pymethods]
impl PyTrinityRuntime {
    #[new]
    fn new() -> Self {
        Self {
            runtime: TrinityRuntime::new(),
        }
    }

    #[pyo3(name = "Update")]
    fn update(&mut self) {
        self.runtime.update();
    }

    #[pyo3(name = "RegisteredClasses")]
    fn registered_classes(&self) -> Vec<String> {
        self.runtime
            .world()
            .get_resource::<TrinityRegistry>()
            .map_or_else(Vec::new, |registry| {
                registry
                    .classes()
                    .map(|info| info.qualified_name())
                    .collect()
            })
    }
}

#[pyfunction(name = "InvalidArgument")]
fn invalid_argument() -> PyALResult {
    PyALResult {
        inner: ALResult::new(E_INVALIDARG),
    }
}

#[pymodule]
fn _trinity_rs(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.gil_used(true)?;
    module.add_class::<PyALResult>()?;
    module.add_class::<PyTr2BitmapDimensions>()?;
    module.add_class::<PyTrinityRuntime>()?;
    module.add_function(wrap_pyfunction!(invalid_argument, module)?)?;
    Ok(())
}
