use crate::{ParseError , header , triangle::{Triangle , Point}};
use std::io::Read;
use byteorder::{LittleEndian , ByteOrder};
use std::convert::TryInto;

pub struct STLData<T: Triangle<P> , P: Point>{
    pub header: header::STLHeader,
    pub trigs: Vec<T>,
    __phantom__: std::marker::PhantomData<P>,
}



#[cfg(test)]
mod tests{
    use super::*;
    use byteorder::{LittleEndian , ByteOrder};

    fn gen_header(trig_count: u32) -> [u8;84]{
        let mut buff = [0u8;84];
        LittleEndian::write_u32(&mut buff[80..84] , trig_count);
        buff
    }

    #[test]
    fn header(){
        use crate::header::STLHeader;
        const X: u32 = 12;
        let header = STLHeader::from(gen_header(X));
        assert_eq!(X , header.triangle_count);
    }

    
}
