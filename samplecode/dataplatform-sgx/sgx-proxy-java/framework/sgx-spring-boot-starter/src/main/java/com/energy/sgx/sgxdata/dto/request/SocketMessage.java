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

    /** 详细的请求数据，JSON格式字符串 */
    private String param;
}
