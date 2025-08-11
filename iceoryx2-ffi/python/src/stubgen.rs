use _iceoryx2;
use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = _iceoryx2::stub_info()?;
    stub.generate()?;

    Ok(())
}
