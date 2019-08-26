package com.energy.sgx.utils;

import com.alibaba.fastjson.JSON;
import java.util.List;
import org.springframework.util.ObjectUtils;

/**
 * @author Bryan
 * @date 2019-07-17
 */
public class JsonUtil {

    public static <T> String toString(T object) {
        return JSON.toJSONString(object);
    }

    public static <T> String toString(List<T> data) {
        if (!ObjectUtils.isEmpty(data)) {
            return JSON.toJSONString(data);
        }
        return null;
    }

    public static <T> T parseJson(String jsonStr, Class<T> clazz) {
        return JSON.parseObject(jsonStr, clazz);
    }

    public static <T> List<T> parseJsonList(String jsonStr, Class<T> clazz) {
        return JSON.parseArray(jsonStr, clazz);
    }
}
