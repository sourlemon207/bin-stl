pub trait Point: Copy{
    fn new(x: f32 , y: f32 , z: f32) -> Self;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

impl Point for (f32 , f32 , f32){
    fn new(x: f32 , y: f32 , z: f32) -> Self{
        (x , y , z)
    }
    fn x(&self) -> f32{
        self.0
    }
    fn y(&self) -> f32{
        self.1
    }
    fn z(&self) -> f32{
        self.2
    }
}

impl Point for [f32;3]{
    fn new(x: f32 , y: f32 , z: f32) -> Self{
        [x , y , z]
    }
    fn x(&self) -> f32{
        self[0]
    }
    fn y(&self) -> f32{
        self[1]
    }
    fn z(&self) -> f32{
        self[2]
    }
    
}

pub trait Triangle<P: Point>{
    fn new(normal: P , vert1: P , vert2: P , vert3: P , attr: u16) -> Self;
    fn normal(&self) -> P;
    fn vert1(&self) -> P;
    fn vert2(&self) -> P;
    fn vert3(&self) -> P;
    fn attr(&self) -> u16;
}

#[derive(Copy , Clone , Debug)]
pub struct Trig<P: Point>{
    normal: P,
    v1: P,
    v2: P,
    v3: P,
}

impl<P: Point> Triangle<P> for Trig<P>{
    fn new(normal: P , vert1: P , vert2: P , vert3: P , _: u16) -> Self{
        Self{
            normal,
            v1: vert1,
            v2: vert2,
            v3: vert3,
        }
    }
    fn normal(&self) -> P{
        self.normal
    }
    fn vert1(&self) -> P{
        self.v1
    }
    fn vert2(&self) -> P{
        self.v2
    }
    fn vert3(&self) -> P{
        self.v3
    }
    fn attr(&self) -> u16{0}
}
