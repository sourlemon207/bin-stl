use byteorder::{ByteOrder, LittleEndian};
use std::convert::TryInto;
use std::io::{Read, Write};
use thiserror::Error;

pub mod triangle;
use triangle::{Point, Triangle};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(
        "Wrong triangle count, amount in header {count_in_header} amount recived {count_recived}"
    )]
    WrongTrigCount {
        count_in_header: u32,
        count_recived: u32,
    },

    #[error("Triangle data is corrupt")]
    CorruptData,

    #[error("IO error")]
    IoErr(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("To many triangles, max is {max} fount {found}")]
    ToManyTriangles { max: usize, found: usize },
    #[error("IO error")]
    IoErr(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy)]
pub struct STLHeader {
    pub header: [u8; 80],
    pub triangle_count: u32,
}

fn header_from_buff(buff: [u8; 84]) -> STLHeader {
    let trig_count = LittleEndian::read_u32(&buff[80..84]);
    STLHeader {
        header: (&buff[0..80]).try_into().unwrap(),
        triangle_count: trig_count,
    }
}

fn header_into_buff(header: STLHeader) -> [u8; 84] {
    let mut buff = [0u8; 84];
    buff[0..80].copy_from_slice(&header.header);
    LittleEndian::write_u32(&mut buff[80..84], header.triangle_count);
    buff
}

pub struct STLData<T: triangle::Triangle<P>, P: triangle::Point> {
    pub header: STLHeader,
    pub trigs: Vec<T>,
    __phantom__: std::marker::PhantomData<P>,
}

#[inline]
fn point_from_buff<P: Point>(buff: [u8; 12]) -> P {
    let x: f32 = LittleEndian::read_f32(&buff[0..4]);
    let y: f32 = LittleEndian::read_f32(&buff[4..8]);
    let z: f32 = LittleEndian::read_f32(&buff[8..12]);
    P::new(x, y, z)
}

fn triangle_from_buff<P: Point, T: Triangle<P>>(buff: [u8; 50]) -> T {
    let normal = point_from_buff(buff[0..12].try_into().unwrap());
    let vert1 = point_from_buff(buff[12..24].try_into().unwrap());
    let vert2 = point_from_buff(buff[24..36].try_into().unwrap());
    let vert3 = point_from_buff(buff[36..48].try_into().unwrap());
    let attr = LittleEndian::read_u16(&buff[48..50]);
    T::new(normal, vert1, vert2, vert3, attr)
}

#[inline]
fn point_into_buff<P: Point>(point: &P) -> [u8; 12] {
    let x = point.x();
    let y = point.y();
    let z = point.z();
    let mut buff = [0u8; 12];
    LittleEndian::write_f32(&mut buff[0..4], x);
    LittleEndian::write_f32(&mut buff[4..8], y);
    LittleEndian::write_f32(&mut buff[8..12], z);
    buff
}

fn triangle_into_buff<P: Point, T: Triangle<P>>(trig: &T) -> [u8; 50] {
    let mut buff = [0u8; 50];
    buff[0..12].copy_from_slice(&point_into_buff(&trig.normal()));
    buff[12..24].copy_from_slice(&point_into_buff(&trig.vert1()));
    buff[24..36].copy_from_slice(&point_into_buff(&trig.vert2()));
    buff[36..48].copy_from_slice(&point_into_buff(&trig.vert3()));
    LittleEndian::write_u16(&mut buff[48..50], trig.attr());
    buff
}

fn read_triangle<R: Read, P: Point, T: Triangle<P>>(rd: &mut R) -> Result<Option<T>, ParseError> {
    let mut buff = [0u8; 50];
    let bytes_read = rd.read(&mut buff)?;
    if bytes_read == buff.len() {
        Ok(Some(triangle_from_buff(buff)))
    } else if bytes_read == 0 {
        Ok(None)
    } else {
        Err(ParseError::CorruptData)
    }
}
pub fn read_stl<R: Read, P: Point, T: Triangle<P>>(
    rd: &mut R,
) -> Result<STLData<T, P>, ParseError> {
    let mut header_data = [0u8; 84];
    rd.read_exact(&mut header_data)?;
    let header = header_from_buff(header_data);
    let mut trigs: Vec<T> = Vec::with_capacity(header.triangle_count as usize);
    while let Some(triangle) = read_triangle::<_, _, T>(rd)? {
        if trigs.len() + 1 > header.triangle_count as usize {
            return Err(ParseError::WrongTrigCount {
                count_in_header: header.triangle_count,
                count_recived: (trigs.len() + 1) as u32,
            });
        } else {
            trigs.push(triangle);
        }
    }
    Ok(STLData {
        header,
        trigs,
        __phantom__: Default::default(),
    })
}


pub fn write_stl<W: Write, P: Point, T: Triangle<P>, S: AsRef<[T]>>(
    writer: &mut W,
    trigs: S,
) -> Result<(), WriteError> {
    let trigs = trigs.as_ref();
    let header = STLHeader {
        triangle_count: trigs.len().try_into().unwrap(),
        header: [0u8; 80],
    };
    write_stl_with_header(writer, trigs, header)
}

pub fn write_stl_with_header<W: Write, P: Point, T: Triangle<P>, S: AsRef<[T]>>(
    writer: &mut W,
    trigs: S,
    header: STLHeader,
) -> Result<(), WriteError> {
    let trigs = trigs.as_ref();
    writer.write_all(&header_into_buff(header))?;
    for trig in trigs.iter() {
        let buff = triangle_into_buff(trig);
        writer.write_all(&buff)?;
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use triangle::Trig;

    #[derive(Debug, Copy, Clone)]
    struct TestHeader(STLHeader);

    impl PartialEq for TestHeader {
        fn eq(&self, other: &Self) -> bool {
            self.0.header == other.0.header && self.0.triangle_count == other.0.triangle_count
        }
    }

    fn near_tuple(t1: (f32, f32, f32), t2: (f32, f32, f32), within: f32) -> bool {
        (t1.0 - t2.0).abs() <= within
            && (t1.1 - t2.1).abs() <= within
            && (t1.2 - t2.2).abs() <= within
    }
    fn point_into_tuple<P: Point>(p: P) -> (f32, f32, f32) {
        (p.x(), p.y(), p.z())
    }
    fn triangle_near<P: Point, T: Triangle<P>>(lhs: &T, rhs: &T, within: f32) -> bool {
        let n1 = point_into_tuple(lhs.normal());
        let v1_1 = point_into_tuple(lhs.vert1());
        let v2_1 = point_into_tuple(lhs.vert2());
        let v3_1 = point_into_tuple(lhs.vert3());

        let n2 = point_into_tuple(rhs.normal());
        let v1_2 = point_into_tuple(rhs.vert1());
        let v2_2 = point_into_tuple(rhs.vert2());
        let v3_2 = point_into_tuple(rhs.vert3());

        near_tuple(n1, n2, within)
            && near_tuple(v1_1, v1_2, within)
            && near_tuple(v2_1, v2_2, within)
            && near_tuple(v3_1, v3_2, within)
    }
    fn gen_triangle<P: Point>() -> Trig<P> {
        let p = P::new(1.0, 2.0, 3.0);
        Trig::new(p, p, p, p, 0)
    }
    #[test]
    fn near() {
        let p = (1.0, 2.0, 3.0);
        let t = Trig::new(p, p, p, p, 0);
        assert!(triangle_near(&t, &t, 0.01))
    }
    #[test]
    fn header() {
        let header = TestHeader(STLHeader {
            header: [0u8; 80],
            triangle_count: 32,
        });
        let header_data = header_into_buff(header.0);
        let header2 = TestHeader(header_from_buff(header_data));
        assert_eq!(header2, header)
    }
    #[test]
    fn encode_decode() {
        use std::io::Cursor;
        let v: Vec<_> = std::iter::repeat(gen_triangle::<(f32, f32, f32)>())
            .take(10)
            .collect();
        let mut encoded = Vec::new();
        write_stl(&mut encoded, &v).unwrap();
        let data = read_stl(&mut Cursor::new(encoded)).unwrap();
        data.trigs
            .iter()
            .zip(v.iter())
            .for_each(|(x, y)| assert!(triangle_near(x, y, 0.1)));
        assert_eq!(v.len(), data.trigs.len());
    }
}
