use dynamic_pool::{DynamicPool, DynamicReset};

#[derive(Default)]
struct Person {
    name: String,
    age: u16,
}

impl DynamicReset for Person {
    fn reset(&mut self) {
        self.name.clear();
        self.age = 0;
    }
}

fn main() {
    // Creates a new pool that will hold at most 10 items, starting with 1 item by default.
    let pool = DynamicPool::new(1, 10, Person::default);
    // Assert we have one item in the pool.
    assert_eq!(pool.available(), 1);

    // Take an item from the pool.
    let mut person = pool.take();
    person.name = "jake".into();
    person.age = 99;

    // Assert the pool is empty since we took the person above.
    assert_eq!(pool.available(), 0);
    // Dropping returns the item to the pool.
    drop(person);
    // We now have stuff available in the pool to take.
    assert_eq!(pool.available(), 1);

    // Take person from the pool again, it should be reset.
    let person = pool.take();
    assert_eq!(person.name, "");
    assert_eq!(person.age, 0);

    // Nothing is in the queue.
    assert_eq!(pool.available(), 0);
    // try_take returns an Option. Since the pool is empty, nothing will be created.
    assert!(pool.try_take().is_none());
    // Dropping again returns the person to the pool.
    drop(person);
    // We have stuff in the pool now!
    assert_eq!(pool.available(), 1);

    // try_take would succeed here!
    let person = pool.try_take().unwrap();

    // We can also then detach the `person` from the pool, meaning it won't get
    // recycled.
    let person = person.detach();
    // We can then drop that person, and see that it's not returned to the pool.
    drop(person);
    assert_eq!(pool.available(), 0);
}
