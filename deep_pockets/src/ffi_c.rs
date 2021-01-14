use libc::{boolean_t, c_char, size_t};
use std::ffi;
use std::ptr;

use crate::cache::Cache;

/// Construct a new `Cache` with a given table name.
///
/// # Note
///
/// This cache is an in-memory sqlite data store.
#[no_mangle]
pub unsafe extern "C" fn CacheCreateWithCacheName(cache_name: *const c_char) -> *mut Cache {
    if cache_name.is_null() {
        return ptr::null_mut();
    }

    let input_str = {
        if let Ok(value) = ffi::CStr::from_ptr(cache_name).to_str() {
            value
        } else {
            return ptr::null_mut();
        }
    };

    let cache = Cache::new(&input_str);
    Box::into_raw(Box::new(cache))
}

/// Destroy a `Cache` once you are done with it.
#[no_mangle]
pub unsafe extern "C" fn CacheDestroy(cache_ptr: *mut Cache) {
    if !cache_ptr.is_null() {
        drop(Box::from_raw(cache_ptr));
    }
}

/// Destroy a string copied out of a `Cache` instance once copied.
#[no_mangle]
pub unsafe extern "C" fn CacheDestroyString(string: *mut c_char) {
    if !string.is_null() {
        drop(ffi::CString::from_raw(string));
    }
}

/// Sets the value for the key in the cache.
#[no_mangle]
pub unsafe extern "C" fn CacheSetObject(
    cache_ptr: *mut Cache,
    key: *const c_char,
    value: *const c_char,
) {
    if cache_ptr.is_null() || key.is_null() || value.is_null() {
        return;
    }

    let key_str = {
        if let Ok(value) = ffi::CStr::from_ptr(key).to_str() {
            value
        } else {
            return;
        }
    };

    let value_str = {
        if let Ok(value) = ffi::CStr::from_ptr(value).to_str() {
            value
        } else {
            return;
        }
    };

    (&mut *cache_ptr).set_object(&key_str, &value_str);
}

/// Sets the value for the key in the cache with a given Cost.
#[no_mangle]
pub unsafe extern "C" fn CacheSetObjectWithCost(
    cache_ptr: *mut Cache,
    key: *const c_char,
    value: *const c_char,
    cost: size_t,
) {
    if cache_ptr.is_null() || key.is_null() || value.is_null() {
        return;
    }

    let key_str = {
        if let Ok(value) = ffi::CStr::from_ptr(key).to_str() {
            value
        } else {
            return;
        }
    };

    let value_str = {
        if let Ok(value) = ffi::CStr::from_ptr(value).to_str() {
            value
        } else {
            return;
        }
    };

    (&mut *cache_ptr).set_object_cost(&key_str, &value_str, cost);
}

/// Returns the value for the given key or an error if the persistence fails.
#[no_mangle]
pub unsafe extern "C" fn CacheObjectFor(
    cache_ptr: *mut Cache,
    key: *const c_char,
    out_value: *mut *const c_char,
) -> boolean_t {
    if cache_ptr.is_null() || key.is_null() || out_value.is_null() {
        return 0;
    }

    let key_str = {
        if let Ok(value) = ffi::CStr::from_ptr(key).to_str() {
            value
        } else {
            return 0;
        }
    };

    match (&mut *cache_ptr).object_for(key_str) {
        Ok(Some(value)) => {
            let out_str = ffi::CString::new(value).ok();
            (*out_value) = out_str
                .map(ffi::CString::into_raw)
                .unwrap_or(ptr::null_mut());
            1
        }
        Ok(None) => {
            (*out_value) = ptr::null_mut();
            1
        }
        Err(_) => 0,
    }
}

/// Removes the object from the cache.
///
/// # Note
///
/// Removing an object that is not in the cache has no effect.
#[no_mangle]
pub unsafe extern "C" fn CacheRemoveObject(cache_ptr: *mut Cache, key: *const c_char) {
    if cache_ptr.is_null() || key.is_null() {
        return;
    }

    let key_str = {
        if let Ok(value) = ffi::CStr::from_ptr(key).to_str() {
            value
        } else {
            return;
        }
    };

    (&mut *cache_ptr).remove_object(key_str);
}

/// Clears the cache.
#[no_mangle]
pub unsafe extern "C" fn CacheRemoveAllObjects(cache_ptr: *mut Cache) {
    (&mut *cache_ptr).remove_all_objects();
}

#[cfg(test)]
mod tests {
    use super::Cache;

    #[test]
    fn test() {
        let mut cache = Cache::new(String::from("cache"));
    }
}
