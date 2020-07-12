use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyBytes;

extern crate image;
use image::imageops::FilterType::Lanczos3;
use image::DynamicImage;
use image::RgbImage;


#[pyfunction]
fn save_image(file: &str, dest: &str) {
    let img = image::open(file).unwrap();
    img.save(dest).unwrap()
}

#[pyfunction]
fn image_size(file: &str) -> (u32, u32) {
    image::image_dimensions(file).unwrap()
}

fn resize(file: &DynamicImage, width: u32, height: u32) -> Vec<u8> {
    file.resize_exact(width, height, Lanczos3).to_bytes()
}


fn bytes_to_py(py: Python, bytes: Vec<u8>) -> &PyBytes {
    PyBytes::new(py, &bytes)
}

#[pyfunction]
fn py_resize<'a>(py: Python<'a>, file: &'a str, width: u32, height: u32) -> &'a PyBytes {
    let dynimage = image::open(file).unwrap();
    let bytes = resize(&dynimage, width, height);
    bytes_to_py(py, bytes)
}

#[pyfunction]
fn save_bytes(bytes: Vec<u8>, width: u32, height: u32, dest: &str) {
    // https://github.com/image-rs/image/issues/1108
    let image = RgbImage::from_raw(width, height, bytes).unwrap();
    image.save(dest).unwrap()
}


#[pymodule]
fn pyimagers(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(save_image))?;
    m.add_wrapped(wrap_pyfunction!(image_size))?;
    m.add_wrapped(wrap_pyfunction!(py_resize))?;
    m.add_wrapped(wrap_pyfunction!(save_bytes))?;

    Ok(())
}
