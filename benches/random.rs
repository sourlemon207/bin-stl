mod common;
use common::{TestTriangle};
use quickcheck::{Gen, StdGen , Arbitrary};
use criterion::*;
use bin_stl::*;

fn gen_random_vec(size: usize) -> Vec<TestTriangle>{
    let mut gen = StdGen::new(rand::thread_rng() , usize::MAX);
    std::iter::repeat_with(||{
        TestTriangle::arbitrary(&mut gen)
    }).take(size).collect()
}

fn encode(c: &mut Criterion){
    let mut group = c.benchmark_group("encode");
    for i in [100 , 1000 , 10000].iter(){
        group.throughput(Throughput::Bytes(50 * *i));
        group.bench_with_input(format!("Encode {}" , i) , i , |b , count|{
            let data = gen_random_vec(*count as usize);
            let mut writepad = Vec::with_capacity(84 + (data.len() * 50));
            b.iter(||{
                black_box(write_stl(black_box(&mut writepad) , &data)).unwrap();
            })
        });
    }
}

fn decode(c: &mut Criterion){
    let mut group = c.benchmark_group("decode");
    for i in [100 , 1000 , 10000].iter(){
        group.throughput(Throughput::Bytes(50 * *i));
        group.bench_with_input(format!("Decode {}" ,  i) , i , |b , count|{
            let trigs = gen_random_vec(*count as usize);
            let mut encoded = Vec::with_capacity(84 + (trigs.len() * 50));
            write_stl(&mut encoded , trigs).unwrap();
            b.iter(||{
                black_box(read_stl::<_ , _  , TestTriangle>(&mut std::io::Cursor::new(&mut encoded))).unwrap();
            })
        });
    }
}

criterion_group!(benches, encode , decode);
criterion_main!(benches);
