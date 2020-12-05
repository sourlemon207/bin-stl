mod common;
use common::TestTriangle;
use bin_stl::*;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;

static TEAPOT_PATH: &str = "tests/stls/utah_teapot.stl";

lazy_static!{
    static ref TEAPOT: std::io::Result<Vec<u8>> = {
        let mut f = File::open(TEAPOT_PATH)?;
        let mut buff = Vec::new();
        f.read_to_end(&mut buff)?;
        assert!(!buff.is_empty());
        Ok(buff)
    };
}

#[test]
fn read_teapot(){
    TEAPOT.as_ref().unwrap();
}

#[test]
fn parse(){
    let teapot_data = TEAPOT.as_ref().unwrap();
    let _ = read_stl::<_ , _ , TestTriangle>(&mut std::io::Cursor::new(teapot_data)).unwrap();
}

#[test]
fn encode_decode(){
    let teapot_data = TEAPOT.as_ref().unwrap();
    let stl_data = read_stl::<_ , _ , TestTriangle>(&mut std::io::Cursor::new(teapot_data)).unwrap();
    let mut encoded = Vec::new();
    write_stl(&mut encoded , stl_data.trigs).unwrap();
    assert_eq!(&teapot_data[84..] , &encoded[84..])
}

#[test]
fn encode_decode_with_header(){
    let teapot_data = TEAPOT.as_ref().unwrap();
    let stl_data = read_stl::<_ , _ , TestTriangle>(&mut std::io::Cursor::new(teapot_data)).unwrap();
    let mut encoded = Vec::new();
    write_stl_with_header(&mut encoded , stl_data.trigs , stl_data.header).unwrap();
    assert_eq!(&encoded , teapot_data)
}
