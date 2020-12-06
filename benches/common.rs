use bin_stl::triangle::{Point, Triangle, Trig};
use quickcheck::{Arbitrary, Gen};

#[derive(Copy, Clone, Debug)]
pub struct TestPoint(f32, f32, f32);

impl Point for TestPoint {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }
    fn x(&self) -> f32 {
        self.0
    }
    fn y(&self) -> f32 {
        self.1
    }
    fn z(&self) -> f32 {
        self.2
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TestTriangle(Trig<TestPoint>);

impl Triangle<TestPoint> for TestTriangle {
    fn new(normal: TestPoint, v1: TestPoint, v2: TestPoint, v3: TestPoint, _: u16) -> Self {
        Self(Trig::new(normal, v1, v2, v3, 0))
    }
    fn normal(&self) -> TestPoint {
        self.0.normal()
    }
    fn vert1(&self) -> TestPoint {
        self.0.vert1()
    }
    fn vert2(&self) -> TestPoint {
        self.0.vert2()
    }
    fn vert3(&self) -> TestPoint {
        self.0.vert3()
    }
    fn attr(&self) -> u16 {
        0
    }
}

impl Arbitrary for TestPoint {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Self(f32::arbitrary(g), f32::arbitrary(g), f32::arbitrary(g))
    }
}

impl Arbitrary for TestTriangle {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Self(Trig::new(
            TestPoint::arbitrary(g),
            TestPoint::arbitrary(g),
            TestPoint::arbitrary(g),
            TestPoint::arbitrary(g),
            u16::arbitrary(g),
        ))
    }
}
pub trait Near {
    fn near(&self, other: &Self, within: f32) -> bool;
}
impl Near for TestPoint {
    fn near(&self, other: &Self, within: f32) -> bool {
        (self.0 - other.0).abs() <= within
            && (self.1 - other.1).abs() <= within
            && (self.2 - other.2).abs() <= within
    }
}

impl Near for TestTriangle{
    fn near(&self, other: &Self , within: f32) -> bool{
        self.normal().near(&other.normal() , within)
            && self.vert1().near(&other.vert1() , within)
            && self.vert2().near(&other.vert2() , within)
            && self.vert3().near(&other.vert3() , within)
    }
}

impl<N: Near> Near for [N]{
    fn near(&self, other: &Self , within: f32) -> bool{
        if self.len() != other.len(){
            false
        }else{
            self.iter().zip(other.iter()).map(|(x , y)|{
                x.near(y , within)
            }).fold(true , |_ , i|{
                i
            })
        }
    }
} 
