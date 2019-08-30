package com.energy.sgx.sgxdata.dto.request;

import lombok.AllArgsConstructor;
import lombok.Data;

/**
 * @author Bryan
 * @date 2019-08-19
 */
@Data
@AllArgsConstructor
public class ProjectLedgerVo {

    /** 项目ID */
    private Integer projectId;

    /** 账单日(1~28) */
    private Integer billDate;

    /** 账单开始日(格式：2019-08-19) */
    private String billStartDate;

    /** 账单周期,单位:月 */
    private Integer billCycle;

    /** 分账日(1~28) */
    private Integer ledgerDate;

    /** 分账协议的内容 */
    private String ledgerContent;
}
