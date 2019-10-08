package com.energy.sgx.sgxdata.service;

import java.util.Map;

/**
 * @author Bryan
 * @date 2019-08-16
 */
public interface TransactionService {

    /**
     * 分账交易接口，通过接口触发SGX的分账交易，如果bill和day都填写，优先bill有效
     *
     * @param bill 账单唯一编号，允许为空
     * @param day 分账日，允许为空
     * @return SGX服务器的响应消息
     */
    Object paymentToSgxServer(Integer bill, Integer day);

    /**
     * B2B分账交易接口，通过接口触发SgxServer的B2B分账交易
     *
     * @param bill 账单唯一编号，允许为空
     * @return SgxServer的响应消息
     */
    Object paymentByB2BToSgxServer(Integer bill);

    /**
     * 交易结果通知到SgxServer接口
     *
     * @param notifyParams 消息参数对
     * @return SgxServer的响应消息NotifyResponse
     */
    Object paymentNotifyToSgxServer(Map<String, String> notifyParams);

    /**
     * 刷新交易结果到SgxServer接口
     *
     * @param bill 消息参数对，允许为空
     * @return SgxServer的响应消息
     */
    Object paymentRefreshToSgxServer(Integer bill);
}
