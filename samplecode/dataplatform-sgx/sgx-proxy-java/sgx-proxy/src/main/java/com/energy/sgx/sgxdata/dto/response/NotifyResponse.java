package com.energy.sgx.sgxdata.dto.response;

import lombok.Data;

/**
 * @author Bryan
 * @date 2019-09-27
 */
@Data
public class NotifyResponse {

    /** 数据处理的消息结果 **/
    private String message;

    /** 返回给前端的跳转页面地址 **/
    private String url;

    public NotifyResponse(String message) {
        this.message = message;
    }
}
