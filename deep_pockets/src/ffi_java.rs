use json::Encoder;

/// Construct a new `Cache` with a given table name.
///
/// # Note
///
/// This cache is an in-memory sqlite data store.
#[no_mangle]
pub unsafe extern "system" fn Java_DeepPockets_cacheNew(
    env: JNIEnv,
    _class: JClass,
    table_namer_ptr: JString,
) -> jlong {
    let table_name: String = env
        .get_string(table_namer_ptr)
        .expect("Couldn't get java string!")
        .into();
    let cache = Cache::new(table_name);
    Box::into_raw(Box::new(cache)) as jlong
}

/// Sets the value for the key in the cache.
#[no_mangle]
pub unsafe extern "system" fn Java_DeepPockets_cacheSetObject(
    env: JNIEnv,
    _class: JClass,
    cache_ptr: jlong,
    key_ptr: JString,
    value_ptr: JString,
) {
    let key: String = env
        .get_string(key_ptr)
        .expect("Couldn't get java string!")
        .into();

    let value: String = env
        .get_string(value_ptr)
        .expect("Couldn't get java string!")
        .into();

    let cache = &mut *(cache_ptr as *mut Cache);
    cache.set_object(&key, &value);
}

/// Sets the value for the key in the cache with a given Cost.
#[no_mangle]
pub unsafe extern "system" fn Java_DeepPockets_cacheSetObjectWithCost(
    env: JNIEnv,
    _class: JClass,
    cache_ptr: jlong,
    key_ptr: JString,
    value_ptr: JString,
    cost: jlong,
) {
    let key: String = env
        .get_string(key_ptr)
        .expect("Couldn't get java string!")
        .into();

    let value: String = env
        .get_string(value_ptr)
        .expect("Couldn't get java string!")
        .into();
    
    if cost < 0 {
        let _ = env.throw(("java/lang/Exception", "Cannot be negative"));
    }

    let cost: usize = cost as usize;
    let cache = &mut *(cache_ptr as *mut Cache);
    cache.set_object_cost(&key, &value, cost);
}

/// Returns the value for the given key or an error if the persistence fails.
#[no_mangle]
pub unsafe extern "system" fn Java_DeepPockets_cacheObjectForKey(
    env: JNIEnv,
    _class: JClass,
    cache_ptr: jlong,
    key_ptr: JString,
) -> jstring {
    let key: String = env
        .get_string(key_ptr)
        .expect("Couldn't get java string!")
        .into();

    let cache = &mut *(cache_ptr as *mut Cache);
    match cache.object_for(key) {
        Ok(Some(value)) => {
            return env.new_string(value).unwrap().into_inner();
        }
        Ok(None) => {
            return JObject::null().into_inner();
        }
        Err(err) => {
            let _ = env.throw(("java/lang/Exception", err.to_string()));

            return JObject::null().into_inner();
        }
    }
}

/// Removes the object from the cache.
///
/// # Note
///
/// Removing an object that is not in the cache has no effect.
#[no_mangle]
pub unsafe extern "system" fn Java_DeepPockets_cacheRemoveObject(
    env: JNIEnv,
    _class: JClass,
    cache_ptr: jlong,
    key_ptr: JString,
) {
    let key: String = env
        .get_string(key_ptr)
        .expect("Couldn't get java string!")
        .into();

    let cache = &mut *(cache_ptr as *mut Cache);
    cache.remove_object(key);
}

/// Clears the cache.
#[no_mangle]
pub unsafe extern "system" fn Java_DeepPockets_cacheRemoveAllObjects(
    env: JNIEnv,
    _class: JClass,
    cache_ptr: jlong,
) {
    let cache = &mut *(cache_ptr as *mut Cache);
    cache.remove_all_objects();
}
