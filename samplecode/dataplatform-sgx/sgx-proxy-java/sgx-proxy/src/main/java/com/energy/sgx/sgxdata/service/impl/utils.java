package com.energy.sgx.sgxdata.service.impl;

import com.energy.sgx.sgxdata.dto.response.LastUpdatedTime;
import com.energy.sgx.sgxdata.dto.response.LastUpdatedTime.UpdatedTime;
import com.energy.sgx.sgxdata.dto.response.SgxServerResponse;
import com.energy.utils.JsonUtil;
import lombok.extern.slf4j.Slf4j;

/**
 * @author Bryan
 * @date 2019-09-25
 */
@Slf4j
public class utils {

    /**
     * 查询上次更新数据的最大时间。查询不到时间，则lastTime为null
     */
    public static LastUpdatedTime fromJson(String jsonStr) {
        LastUpdatedTime updatedTime = new LastUpdatedTime();
        SgxServerResponse resp = JsonUtil.fromJson(jsonStr, SgxServerResponse.class);
        if (resp.getSuccess()) {
            updatedTime.setTime(JsonUtil.fromJson(resp.getData(), UpdatedTime.class));
        } else {
            log.error("message: {}", resp.getMessage());
            updatedTime.setMessage(resp.getMessage());
        }
        return updatedTime;
    }
}
