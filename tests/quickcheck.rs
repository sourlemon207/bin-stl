#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod common;
use common::*;
use bin_stl::*;

#[quickcheck]
fn encode_decode(v: Vec<TestTriangle>) -> bool{
    let mut encoded = Vec::new();
    write_stl(&mut encoded , &v).unwrap();
    let stldata = read_stl(&mut std::io::Cursor::new(encoded)).unwrap();

    v.near(stldata.trigs.as_slice() , 0.01)
}
