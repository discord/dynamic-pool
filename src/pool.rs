use crossbeam_queue::ArrayQueue;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Weak};

use crate::DynamicReset;

/// a lock-free, thread-safe, dynamically-sized object pool.
///
/// this pool begins with an initial capacity and will continue creating new objects on request when none are available.
/// pooled objects are returned to the pool on destruction (with an extra provision to optionally "reset" the state of
/// an object for re-use).
///
/// if, during an attempted return, a pool already has `maximum_capacity` objects in the pool, the pool will throw away
/// that object.
#[derive(Debug)]
pub struct DynamicPool<T: DynamicReset> {
    data: Arc<PoolData<T>>,
}

impl<T: DynamicReset> DynamicPool<T> {
    /// creates a new `DynamicPool<T>`. this pool will create `initial_capacity` objects, and retain up to
    /// `maximum_capacity` objects.
    ///
    /// # panics.
    ///
    /// panics if `initial_capacity > maximum_capacity`.
    pub fn new<F: Fn() -> T + Sync + 'static>(
        initial_capacity: usize,
        maximum_capacity: usize,
        create: F,
    ) -> DynamicPool<T> {
        assert![initial_capacity <= maximum_capacity];

        let items = ArrayQueue::new(maximum_capacity);

        for x in (0..initial_capacity).map(|_| create()) {
            items
                .push(x)
                .expect("invariant: items.len() always less than initial_capacity.");
        }

        let data = PoolData {
            items,
            create: Box::new(create),
        };
        let data = Arc::new(data);

        DynamicPool { data }
    }

    /// takes an item from the pool, creating one if none are available.
    pub fn take(&self) -> DynamicPoolItem<T> {
        let object = self
            .data
            .items
            .pop()
            .unwrap_or_else(|_| (self.data.create)());

        DynamicPoolItem {
            data: Arc::downgrade(&self.data),
            object: Some(object),
        }
    }

    /// attempts to take an item from the pool, returning `none` if none is available. will never allocate.
    pub fn try_take(&self) -> Option<DynamicPoolItem<T>> {
        let object = self.data.items.pop().ok()?;
        let data = Arc::downgrade(&self.data);

        Some(DynamicPoolItem {
            data,
            object: Some(object),
        })
    }

    /// returns the number of free objects in the pool.
    #[inline]
    pub fn available(&self) -> usize {
        self.data.items.len()
    }

    /// returns the number of objects currently in use. does not include objects that have been detached.
    #[inline]
    pub fn used(&self) -> usize {
        Arc::weak_count(&self.data)
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.data.items.capacity()
    }
}

impl<T: DynamicReset> Clone for DynamicPool<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

// data shared by a `DynamicPool`.
struct PoolData<T> {
    items: ArrayQueue<T>,
    create: Box<dyn Fn() -> T + Sync + 'static>,
}

impl<T: DynamicReset + Debug> Debug for PoolData<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        formatter
            .debug_struct("PoolData")
            .field("items", &self.items)
            .field("create", &"Box<dyn Fn() -> T>")
            .finish()
    }
}

/// an object, checked out from a dynamic pool object.
#[derive(Debug)]
pub struct DynamicPoolItem<T: DynamicReset> {
    data: Weak<PoolData<T>>,
    object: Option<T>,
}

impl<T: DynamicReset> DynamicPoolItem<T> {
    /// detaches this instance from the pool, returns T.
    pub fn detach(mut self) -> T {
        self.object
            .take()
            .expect("invariant: object is always `some`.")
    }
}

impl<T: DynamicReset> AsRef<T> for DynamicPoolItem<T> {
    fn as_ref(&self) -> &T {
        self.object
            .as_ref()
            .expect("invariant: object is always `some`.")
    }
}

impl<T: DynamicReset> Deref for DynamicPoolItem<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.object
            .as_ref()
            .expect("invariant: object is always `some`.")
    }
}

impl<T: DynamicReset> DerefMut for DynamicPoolItem<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.object
            .as_mut()
            .expect("invariant: object is always `some`.")
    }
}

impl<T: DynamicReset> Drop for DynamicPoolItem<T> {
    fn drop(&mut self) {
        if let Some(mut object) = self.object.take() {
            object.reset();
            if let Some(pool) = self.data.upgrade() {
                pool.items.push(object).ok();
            }
        }
    }
}
