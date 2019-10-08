package com.energy.sgx.utils;

import com.alibaba.fastjson.JSON;

public class SgxUtils {

    public static <T> String toString(T object) {
        return JSON.toJSONString(object);
    }

    public static <T> T fromJson(String jsonStr, Class<T> clazz) {
        return JSON.parseObject(jsonStr, clazz);
    }
}
