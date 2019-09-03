package com.energy.sgx.sgxdata.dto.request;

import lombok.AllArgsConstructor;
import lombok.Data;

/**
 * @author Bryan
 * @date 2019-09-03
 */
@Data
@AllArgsConstructor
public class ProjectReceiptVo {

    /** 项目ID */
    private Integer projectId;

    /** 分账支付模式 */
    private Byte chargeMode;

    /** 银行卡号 */
    private String cardNum;

    /** 银行卡用户名 */
    private String cardUser;

    /** 手机号 */
    private String mobile;
}
