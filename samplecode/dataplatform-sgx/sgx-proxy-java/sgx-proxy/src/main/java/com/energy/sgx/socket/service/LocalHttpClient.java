package com.energy.sgx.socket.service;

import java.util.Map;

/**
 * @author Bryan
 * @date 2019-09-27
 */
public interface LocalHttpClient {

    /**
     * 发送Post请求给银联支付服务器
     *
     * @param reqUrl 请求URL地址
     * @param request 请求的数据包
     * @return 服务端响应的参数对，包含post的结果code
     */
    Map<String, String> post(String reqUrl, Map<String, String> request);
}
