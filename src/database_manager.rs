use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct DatabaseManager {
    connection: Connection,
    table_name: String,
}

impl DatabaseManager {
    pub fn open<T>(table_name: T) -> Result<Self, rusqlite::Error>
    where
        T: AsRef<str>,
    {
        Ok(DatabaseManager {
            connection: Connection::open_in_memory()?,
            table_name: String::from(table_name.as_ref()),
        })
    }

    pub fn create_table(&self) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS ? (
                id      INTEGER PRIMARY KEY,
                key     TEXT NOT NULL
                value   TEXT NOT NULL
                cost    INTEGER NOT NULL
            )",
            params![self.table_name],
        )?;
        Ok(())
    }

    pub fn drop_table(&self) -> Result<(), rusqlite::Error> {
        self.connection
            .execute("DROP TABLE IF EXISTS ?", params![self.table_name])?;
        Ok(())
    }

    pub fn create_or_update<K, V>(&self, key: K, value: V, cost: usize)
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let str_key = key.as_ref();
        match self.exists(str_key) {
            Ok(maybe_exists) => {
                if let Some(exists) = maybe_exists {
                    if exists {
                        self.update(str_key, value, cost);
                    } else {
                        self.create(str_key, value, cost).ok();
                    }
                }
            }
            Err(err) => println!("update failed: {}", err),
        }
    }

    fn create<K, V>(&self, key: K, value: V, cost: usize) -> Result<(), rusqlite::Error>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.connection.execute(
            &format!(
                "INSERT INTO {} (key, value, cost) VALUES (?1, ?2, ?3)",
                self.table_name
            ),
            params![key.as_ref(), value.as_ref(), format!("{}", cost)],
        )?;
        Ok(())
    }

    fn update<K, V>(&self, key: K, value: V, cost: usize)
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        match self
            .connection
            .execute("UPDATE foo SET bar = 'baz' WHERE qux = ?", &[&1i32])
        {
            Ok(updated) => println!("{} rows were updated", updated),
            Err(err) => println!("update failed: {}", err),
        }
    }

    pub fn find<K>(&self, key: K) -> Result<Option<String>, rusqlite::Error>
    where
        K: AsRef<str>,
    {
        let mut stmt = self
            .connection
            .prepare("SELECT id, value FROM ?1 WHERE key = ?2 LIMIT 1")?;
        let mut find_iter =
            stmt.query_map(params![self.table_name, key.as_ref()], |row| row.get(1))?;
        match find_iter.next() {
            None => Ok(None),
            Some(Ok(value)) => Ok(value),
            Some(Err(error)) => Err(error),
        }
    }

    pub fn exists<K>(&self, key: K) -> Result<Option<bool>, rusqlite::Error>
    where
        K: AsRef<str>,
    {
        match self.connection.execute(
            &format!(
                "SELECT id, value FROM {} WHERE key = ? LIMIT 1",
                self.table_name
            ),
            params![key.as_ref()],
        ) {
            Ok(found_count) => Ok(Some(found_count > 0)),
            Err(err) => Err(err),
        }
    }

    pub fn destroy<K>(&self, key: K)
    where
        K: AsRef<str>,
    {
        match self.connection.execute(
            "DELETE FROM ? WHERE key= ?;",
            params![self.table_name, key.as_ref()],
        ) {
            Ok(deleted) => println!("{} rows were deleted", deleted),
            Err(err) => println!("update failed: {}", err),
        }
    }

    pub fn destroy_all(&self) {
        match self
            .connection
            .execute("DELETE * FROM ?;", params![self.table_name])
        {
            Ok(deleted) => println!("{} rows were deleted", deleted),
            Err(err) => println!("update failed: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DatabaseManager;

    #[test]
    fn test_exists_retuns_true_when_key_is_found_and_false_otherwise() {
        let mut db = DatabaseManager::open(String::from("cache")).unwrap();
        db.create_table();

        let key = String::from("key");
        let value = String::from("value");
        db.create_or_update(key, value, 10);

        let key = String::from("key");
        match db.exists(key) {
            Ok(maybe_exists) => {
                if let Some(exists) = maybe_exists {
                    assert!(exists)
                } else {
                    assert!(false)
                }
            }
            Err(err) => println!("Found {}", err),
        }
    }
    #[test]
    fn test_database_thing() {
        let mut db = DatabaseManager::open(String::from("cache")).unwrap();
        db.create_table();
        db.create_or_update(String::from("a"), String::from("1"), 1);
        let is_contained = db.exists(String::from("a"));
        println!("Found {:?}", is_contained.unwrap());

        db.drop_table();

        // println!(db.read(String::from("a")))

        // let mut cache = Cache::<String, String>::new();
        // cache.set_object(&String::from("key"), &String::from("value"))
        // cache.set_object(&vec![String::from("1"), String::from("2")], "output".into());
    }
}
