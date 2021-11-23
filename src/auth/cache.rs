use std::time;

// Generic cache struct.
pub struct Cache<T: Clone> {
    value_and_expire: Option<(T, time::Instant)>,
}

impl<T: Clone> Cache<T> {
    pub fn new() -> Self {
        Self {
            value_and_expire: None,
        }
    }

    pub fn get(&self, current_time: &time::Instant) -> Option<&T> {
        let (value, expire) = match &self.value_and_expire {
            None => return None,
            Some(x) => x,
        };
        match current_time < expire {
            true => Some(value),
            false => None,
        }
    }

    pub fn set(&mut self, value: T, expire: time::Instant) {
        self.value_and_expire = Some((value, expire))
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::Cache;
    #[test]
    fn valid_cache_should_return_some() {
        let mut cache = Cache::<i32>::new();

        let current = Instant::now();
        cache.set(42, current + Duration::new(1, 0));
        assert_eq!(cache.get(&current), Some(&42));

        cache.set(43, current + Duration::new(1, 0));
        assert_eq!(cache.get(&current), Some(&43));
    }

    #[test]
    fn expired_cache_should_return_none() {
        let mut cache = Cache::<i32>::new();

        let current = Instant::now();
        cache.set(42, current);
        assert_eq!(cache.get(&(current + Duration::new(1, 0))), None);
    }
}
