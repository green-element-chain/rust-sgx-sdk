package com.energy.sgx.sgxdata.dto.request;

import java.math.BigDecimal;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * @author Bryan
 * @date 2019-07-24
 */
@Data
@NoArgsConstructor
public class AssetOrderVo {

    /** 订单主键ID */
    private Integer orderId;

    /** 资产类型ID */
    private Integer assetType;

    /** 资产主键ID */
    private Integer assetId;

    /** 订单收入值，单位：分 */
    private Integer revenue;

    /** 订单时间 */
    private Long orderTime;

    public AssetOrderVo(Integer orderId, Integer assetType, Integer assetId, Integer revenue, BigDecimal orderTime) {
        this.orderId = orderId;
        this.assetType = assetType;
        this.assetId = assetId;
        this.revenue = revenue;
        this.orderTime = orderTime.longValue();
    }
}
