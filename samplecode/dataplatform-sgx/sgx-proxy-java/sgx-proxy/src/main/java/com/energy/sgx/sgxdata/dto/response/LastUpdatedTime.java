package com.energy.sgx.sgxdata.dto.response;

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
}
