use crate::database_manager::DatabaseManager;

#[derive(Debug)]
pub struct Cache {
    name: String,
    count_limit: isize,
    total_cost_limit: isize,
    storage: DatabaseManager,
}

impl Cache {
    pub fn new<I>(cache_name: I) -> Self
    where
        I: AsRef<str>,
    {
        Cache {
            name: String::from(cache_name.as_ref()),
            storage: DatabaseManager::open(cache_name).unwrap(),
            count_limit: 100,
            total_cost_limit: 1000,
        }
    }

    pub fn set_object<K, V>(&mut self, key: K, object: V)
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let cost = object.as_ref().len();
        self.set_serialized_object(key.as_ref(), object.as_ref(), cost)
    }

    pub fn set_object_cost<K, V>(&mut self, key: K, object: V, cost: usize)
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.set_serialized_object(key.as_ref(), object.as_ref(), cost)
    }

    pub fn object_for<K>(&mut self, key: K) -> Result<Option<String>, rusqlite::Error>
    where
        K: AsRef<str>,
    {
        self.storage.find(key)
    }

    pub fn remove_object<K>(&mut self, key: K)
    where
        K: AsRef<str>,
    {
        self.storage.destroy(key);
    }

    pub fn remove_all_objects(&mut self) {
        self.storage.destroy_all()
    }

    fn set_serialized_object(
        &mut self,
        serialized_key: &str,
        serialized_object: &str,
        cost: usize,
    ) {
        self.storage
            .create_or_update(serialized_key, serialized_object, cost)
    }
}

#[cfg(test)]
mod tests {
    use super::Cache;

    #[test]
    fn test_thing() {
        let mut cache = Cache::new(String::from("akash"));
        cache.set_object(&String::from("key"), &String::from("value"))
        // cache.set_object(&vec![String::from("1"), String::from("2")], "output".into());
    }
}
