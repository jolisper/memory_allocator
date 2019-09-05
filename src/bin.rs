use libmemory_allocator::FirstFit;

#[global_allocator]
static GLOBAL: FirstFit = FirstFit;

pub fn main() {
    // Alloc simple i32
    let _n = Box::new(3);

    {
        Box::new([1, 2, 3]);
    };

    // Usign String to force mem alloc
    let mut vs = Vec::<String>::new();
    vs.push(String::from("Hello"));
    vs.push(String::from("world,"));
    vs.push(String::from("global"));
    vs.push(String::from("allocator"));

    println!("{}", concatenate(vs));
}

fn concatenate(vs: Vec<String>) -> String {
    vs.join(" ")
}
