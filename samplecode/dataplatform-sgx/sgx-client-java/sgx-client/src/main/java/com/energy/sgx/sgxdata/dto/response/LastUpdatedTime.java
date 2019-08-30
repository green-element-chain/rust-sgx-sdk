package com.energy.sgx.sgxdata.dto.response;

import com.energy.utils.JsonUtil;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.extern.slf4j.Slf4j;

/**
 * @author Bryan
 * @date 2019-08-28
 */
@Slf4j
@Data
@AllArgsConstructor
@NoArgsConstructor
public class LastUpdatedTime {

    private UpdatedTime time;
    private String message;

    public boolean invalidTime() {
        return time == null;
    }

    public Integer getLastTime() {
        return time.getLastTime();
    }

    /** 内部内，转JSON需要，必须设置为public */
    @Getter
    @AllArgsConstructor()
    public static class UpdatedTime {

        private Integer lastTime;
    }

    public static class Utils {

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

}
