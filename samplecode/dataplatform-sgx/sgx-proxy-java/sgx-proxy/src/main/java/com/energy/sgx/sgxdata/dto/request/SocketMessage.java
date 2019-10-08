package com.energy.sgx.sgxdata.dto.request;

import lombok.AllArgsConstructor;
import lombok.Data;

/**
 * @author Bryan
 * @date 2019-08-12
 */
@Data
@AllArgsConstructor
public class SocketMessage {

    /** 业务区分处理，类似Restful请求的URL路径，不带http(s)://IP:port部分 */
    private String url;

    /** 请求的参数数据，可以是自定义的字符串，也可以是JSON格式的字符串 */
    private String param;
}
