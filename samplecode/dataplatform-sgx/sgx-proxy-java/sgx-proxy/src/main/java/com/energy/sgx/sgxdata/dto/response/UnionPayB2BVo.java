package com.energy.sgx.sgxdata.dto.response;

import com.chinapay.secss.SecssConstants;
import com.energy.sgx.utils.CollectUtil;
import java.util.Map;
import lombok.Data;

/**
 * @author Bryan
 * @date 2019-09-27
 */
@Data
public class UnionPayB2BVo {

    private String method;
    private String encoding;
    private String requestUrl;
    private Object sendData;

    public UnionPayB2BVo(Map<String, String> contentData, String url) {
        this.method = "post";
        this.encoding = SecssConstants.CHARSET_COMM;
        this.requestUrl = url;
        this.sendData = CollectUtil.coverMap2String(contentData);
    }
}
