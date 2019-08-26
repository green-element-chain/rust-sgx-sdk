package com.energy.sgx.order.service;

/**
 * @author Bryan
 * @date 2019-07-24
 */
public interface DataService {

    /**
     * 发送指定的订单数据到SGX服务器
     *
     * @param assetType 资产类型
     * @return SgxServer的响应消息
     */
    Object sendOrderToSgx(Integer assetType);

    /**
     * 从SGX服务器获取指定资产类型、日期的数据
     *
     * @param assetType 资产类型
     * @param date 日期，例如：2019-07-24
     * @return SgxServer的响应消息：详细数据列表JSON字符串
     */
    Object getOrderFromSgx(Integer assetType, String date);

    /**
     * 发送项目关联的资产到SGX服务器
     *
     * @param projectId 项目ID
     * @return SgxServer的响应消息
     */
    Object sendProjectAssetToSgx(Integer projectId);

    /**
     * 发送项目的分账协议到SGX服务器
     *
     * @param projectId 项目ID
     * @return SgxServer的响应消息
     */
    Object sendProjectLedgerToSgx(Integer projectId);

    /**
     * 发送生成项目账单的指令到SGX服务器
     *
     * @param projectId 项目ID
     * @return SgxServer的响应消息
     */
    Object sendCreateProjectBillToSgx(Integer projectId);
}
