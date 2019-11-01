package com.energy.sgx.sgxdata.service;

import java.util.Date;

/**
 * @author Bryan
 * @date 2019-07-24
 */
public interface DataService {

    /**
     * 转移新的订单数据到SGX服务器
     *
     * @return SGX服务器响应消息
     */
    Object transferAssetOrderToSgx();

    /**
     * 从SGX服务器获取指定资产类型、日期的数据
     *
     * @param assetType 资产类型
     * @param inputDate 日期，格式：2019-07-24
     * @param pageNo 分页号
     * @return SGX服务器响应消息：详细数据列表JSON字符串
     */
    Object queryAssetOrderFromSgx(Integer assetType, Date inputDate, Integer pageNo);

    /**
     * 转移新的项目关联的资产到SGX服务器
     *
     * @return SGX服务器响应消息
     */
    Object transferProjectAssetToSgx();

    /**
     * 转移项目分账协议到SGX服务器
     *
     * @return SGX服务器响应消息
     */
    Object transferProjectLedgerToSgx();

    /**
     * 从SGX服务器查询项目分账信息
     *
     * @param pageNo 分页号
     * @return SGX服务器响应消息
     */
    Object queryProjectLedgerFromSgx(Integer pageNo);

    /**
     * 转移项目分账卡域信息到SGX服务器
     *
     * @return SGX服务器响应消息
     */
    Object transferProjectReceiptToSgx();

    /**
     * 从SGX服务器查询项目分账卡域信息
     *
     * @param pageNo 分页号
     * @return SGX服务器响应消息
     */
    Object queryProjectReceiptFromSgx(Integer pageNo);

    /**
     * 生成项目账单指令发送到SGX服务器
     *
     * @param day 生成账单的日期，每月的几日
     * @param projectId 项目ID，允许为空
     * @return SGX服务器响应消息
     */
    Object generateProjectBillToSgx(Integer day, Integer projectId);

    /**
     * 从SGX服务器查询项目账单信息
     *
     * @param pageNo 分页号
     * @return SGX服务器响应消息
     */
    Object queryProjectBillFromSgx(Integer pageNo);

    /**
     * 从SGX服务器查询项目账单信息
     *
     * @param pageNo 分页号
     * @return SGX服务器响应消息
     */
    Object queryProjectTransactionFromSgx(Integer pageNo);
}
