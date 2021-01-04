class DeepPockets {
    private static native long cacheNew(String table_namer_ptr);
    private static native void cacheSetObject(long cache_ptr, String key_ptr, String value_ptr);
    private static native void cacheSetObjectWithCost(long cache_ptr, String key_ptr, String value_ptr, long cost) throws java.lang.Exception;
    private static native String cacheObjectForKey(long cache_ptr, String key_ptr) throws java.lang.Exception;
    private static native void cacheRemoveObject(long cache_ptr, String key_ptr);
    private static native void cacheRemoveAllObjects();

    static {
        System.loadLibrary("deeppockets");
    }

    public static void main(String[] args) {
    }
}