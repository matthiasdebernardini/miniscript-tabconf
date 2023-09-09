use pyo3::prelude::*;
use std::str::FromStr;



#[pyfunction]
fn get_address_from_miniscript(s: &str) -> PyResult<String> {
    let desc = miniscript::Descriptor::<bitcoin::PublicKey>::from_str(s).unwrap();
    // "sh(wsh(or_d(c:pk_k(020e0338c96a8870479f2396c373cc7696ba124e8635d41b0ea581112b67817261),c:pk_k(0250863ad64a87ae8a2fe83c1af1a8403cb53f53e486d8511dad8a04887e5b2352))))

    // "3CJxbQBfWAe1ZkKiGQNEYrioV73ZwvBWns"
    let a = desc.address(bitcoin::Network::Bitcoin).unwrap().to_string();

    Ok(a)


}

/// A Python module implemented in Rust.
#[pymodule]
fn miniscript_tabconf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_address_from_miniscript, m)?)?;
    Ok(())
}