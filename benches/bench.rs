use std::default::Default;
use dynamic_pool::{DynamicPool, DynamicReset};
use criterion::{criterion_group, criterion_main, Criterion, black_box};


#[derive(Default)]
struct Person{
    name: String,
    age: u8,
}


impl DynamicReset for Person{
    fn reset(&mut self) {
        self.name.clear();
        self.age = 0;
    }
}


fn allocate(pool: &mut DynamicPool<Person>){
    let mut item = pool.take();
    item.age = 10;
    item.name.push_str("ban");
    item.reset();
}

fn std_allocate(){
    let mut person = Person::default();
    person.age = 10;
    person.name.push_str("ban");
    drop(person);

}

fn criterion_benchmark(c: &mut Criterion) {
    let mut pool = DynamicPool::new(1000, 10000, Person::default);
    c.bench_function("reuse", |b| b.iter(|| allocate(black_box(&mut pool))));
}


fn std_benchmark(c: &mut Criterion){
    c.bench_function("std", |b| b.iter(|| std_allocate()));
}

criterion_group!(benches, criterion_benchmark, std_benchmark);
criterion_main!(benches);