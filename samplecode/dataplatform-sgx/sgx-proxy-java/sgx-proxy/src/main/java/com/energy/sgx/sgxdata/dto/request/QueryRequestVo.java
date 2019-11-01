package com.energy.sgx.sgxdata.dto.request;

import lombok.Data;

/**
 * @author Bryan
 * @date 2019-11-01
 */
@Data
public class QueryRequestVo {

    @Data
    public static class PageInfo {

        private Integer pageNo;
        private Integer pageSize;

        PageInfo(Integer pageNo) {
            this.pageNo = (pageNo == null || pageNo <= 0) ? 0 : (pageNo - 1);
            this.pageSize = 20;
        }
    }

    private PageInfo pageInfo;
    /**
     * 附加条件
     */
    private String extend;

    public QueryRequestVo(Integer pageNo, String params) {
        this.pageInfo = new PageInfo(pageNo);
        this.extend = params;
    }
}
