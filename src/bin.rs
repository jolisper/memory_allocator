use libmemory_allocator::RustAllocator;

#[global_allocator]
static GLOBAL: RustAllocator = RustAllocator;

pub fn main() {
    let mut vs = Vec::new();
    vs.push(1);
    vs.push(2);
    vs.push(3);
    vs.push(4);

    let sum: i32 = vs.iter().sum();

    println!("{}", sum);
}
