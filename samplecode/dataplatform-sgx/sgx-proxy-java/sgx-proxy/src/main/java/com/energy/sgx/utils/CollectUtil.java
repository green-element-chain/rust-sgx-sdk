package com.energy.sgx.utils;

import java.io.UnsupportedEncodingException;
import java.net.URLDecoder;
import java.net.URLEncoder;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.TreeMap;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import lombok.extern.slf4j.Slf4j;

/**
 * @author Bryan
 * @date 2019-08-30
 */
@Slf4j
public class CollectUtil {

    private static final Integer MAX_NUMBER = 150;

    public static <T> List<List<T>> splitGroup(List<T> list) {
        int limit = countStep(list.size());
        return Stream.iterate(0, n -> n + 1)
            .limit(limit)
            .parallel()
            .map(a -> list.stream()
                .skip(a * MAX_NUMBER)
                .limit(MAX_NUMBER)
                .parallel()
                .collect(Collectors.toList()))
            .collect(Collectors.toList());
    }

    private static Integer countStep(Integer size) {
        return (size + MAX_NUMBER - 1) / MAX_NUMBER;
    }

    /**
     * 将形如{"xx"="xxx","yy"="yyy"}的字符串转换为相应的Map对象
     *
     * @param result 参数连接字符串
     * @return HashMap容器数据
     */
    public static Map<String, String> convertJsonStringToMap(String result) {
        Map<String, String> map = null;

        if (result != null && !"".equals(result.trim())) {
            if (result.startsWith("{") && result.endsWith("}")) {
                result = result.substring(1, result.length() - 1);
            }
            map = parseQString(result);
        }

        return map;
    }

    /**
     * 将形如key=value&key=value的字符串转换为相应的Map对象
     *
     * @param result 参数连接字符串
     * @return HashMap容器数据
     */
    public static Map<String, String> convertResultStringToMap(String result) {
        Map<String, String> map = null;

        if (result != null && !"".equals(result.trim())) {
            if (result.startsWith("{") && result.endsWith("}")) {
                result = result.substring(1, result.length() - 1);
            }
            map = parseQString(result);
        }

        return map;
    }

    /**
     * 解析应答字符串，生成应答要素
     *
     * @param str 需要解析的字符串
     * @return 解析的结果map
     */
    static Map<String, String> parseQString(String str) {
        Map<String, String> map = new HashMap<>();
        int len = str.length();
        StringBuilder temp = new StringBuilder();
        char curChar;
        String key = null;
        boolean isKey = true;
        /** 值里有嵌套 */
        boolean isOpen = false;
        char openName = 0;
        if (len > 0) {
            // 遍历整个带解析的字符串
            for (int i = 0; i < len; i++) {
                curChar = str.charAt(i);
                // 如果当前生成的是key
                if (isKey) {
                    // 如果读取到=分隔符
                    if (curChar == '=') {
                        key = temp.toString();
                        temp.setLength(0);
                        isKey = false;
                    } else {
                        temp.append(curChar);
                    }
                } else {
                    // 如果当前生成的是value
                    if (isOpen) {
                        if (curChar == openName) {
                            isOpen = false;
                        }
                    } else {//如果没开启嵌套
                        //如果碰到，就开启嵌套
                        if (curChar == '{') {
                            isOpen = true;
                            openName = '}';
                        }
                        if (curChar == '[') {
                            isOpen = true;
                            openName = ']';
                        }
                    }

                    // 如果读取到&分割符,同时这个分割符不是值域，这时将map里添加
                    if (curChar == '&' && !isOpen) {
                        putKeyValueToMap(temp, isKey, key, map);
                        temp.setLength(0);
                        isKey = true;
                    } else {
                        temp.append(curChar);
                    }
                }

            }
            putKeyValueToMap(temp, isKey, key, map);
        }
        return map;
    }

    static void putKeyValueToMap(StringBuilder temp, boolean isKey,
        String key, Map<String, String> map) {
        if (isKey) {
            key = temp.toString();
            if (key.length() == 0) {
                throw new RuntimeException("QString format illegal");
            }
            map.put(key, "");
        } else {
            if (key.length() == 0) {
                throw new RuntimeException("QString format illegal");
            }
            map.put(key, temp.toString());
        }
    }


    /**
     * 将Map中的数据转换成key1=value1&key2=value2的形式，包含签名信息
     *
     * @param data 待拼接的Map数据
     * @return 拼接好后的字符串
     */
    public static String coverMap2String(Map<String, String> data) {
        TreeMap<String, String> tree = new TreeMap<>();
        Iterator<Entry<String, String>> it = data.entrySet().iterator();
        while (it.hasNext()) {
            Entry<String, String> en = it.next();
            /*if (SDKConstants.param_signature.equals(en.getKey().trim())) {
                continue;
            }*/
            tree.put(en.getKey(), en.getValue());
        }
        it = tree.entrySet().iterator();
        StringBuffer sf = new StringBuffer();
        while (it.hasNext()) {
            Entry<String, String> en = it.next();
            sf.append(en.getKey() + SDKConstants.EQUAL + en.getValue()
                + SDKConstants.AMPERSAND);
        }
        return sf.substring(0, sf.length() - 1);
    }

    /**
     * 将Map中的数据转换成key1=value1&key2=value2的形式，参数值需要进行URLEncode加密
     *
     * @param data 交易参数名值对
     * @return 拼接好的字符串
     */
    public static String convertMap2URLEncodeString(Map<String, String> data) {
        StringBuilder sb = new StringBuilder();
        for (Entry<String, String> en : data.entrySet()) {
            try {
                sb.append(en.getKey()
                    + "="
                    + (null == en.getValue() || "".equals(en.getValue()) ? ""
                    : URLEncoder.encode(en.getValue(), "UTF-8")) + "&");
            } catch (UnsupportedEncodingException e) {
                log.error(e.getMessage(), e);
                return "";
            }
        }
        return sb.substring(0, sb.length() - 1);
    }

    /**
     * 将Map中的数据转换成key1=value1&key2=value2的形式，参数值需要进行URLDecode解密
     *
     * @param data 交易参数名值对
     */
    public static void mapValueURLDecode(Map<String, String> data) {
        for (Entry<String, String> en : data.entrySet()) {
            try {
                String value = (null == en.getValue() || "".equals(en.getValue()) ? ""
                    : URLDecoder.decode(en.getValue(), "UTF-8"));
                data.replace(en.getKey(), value);
            } catch (UnsupportedEncodingException e) {
                log.error(e.getMessage(), e);
            }
        }
    }
}
