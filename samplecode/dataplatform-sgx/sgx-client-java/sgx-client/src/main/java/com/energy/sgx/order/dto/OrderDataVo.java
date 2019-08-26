package com.energy.sgx.order.dto;

import lombok.Data;

/**
 * @author Bryan
 * @date 2019-07-24
 */
@Data
public class OrderDataVo {

    /** 订单主键ID */
    private Integer orderId;

    /** 资产类型ID */
    private Integer assetType;

    /** 资产主键ID */
    private int assetId;

    /** 订单收入值，单位：分 */
    private Integer revenue;

    /** 订单时间 */
    private long orderTime;
}
