mod business_days;
mod expressions;
mod format_localized;
mod holiday;
mod is_workday;
mod sub;
mod timezone;
mod to_julian;

#[cfg(target_os = "linux")]
use jemallocator::Jemalloc;
use pyo3::types::PyModule;
use pyo3::{pyfunction, pymodule, PyResult, Python};
use pyo3_polars::error::PyPolarsErr;
use pyo3_polars::PySeries;

#[global_allocator]
#[cfg(target_os = "linux")]
static ALLOC: Jemalloc = Jemalloc;

#[pyfunction]
fn holiday_range(start: i32, end: i32, countries: Vec<std::string::String>) -> PyResult<PySeries> {
    let ser = holiday::to_holiday_range(start, end, countries)
        .map_err(PyPolarsErr::from)?
        .sort(false);
    Ok(PySeries(ser))
}

#[pymodule]
fn _internal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(pyo3::wrap_pyfunction!(holiday_range, m)?)?;
    Ok(())
}
