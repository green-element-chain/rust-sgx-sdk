package com.energy.sgx.sgxdata.service;

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
     * @param notifyParams 消息参数
     * @return SgxServer的响应消息
     */
    Object paymentNotifyToSgxServer(String notifyParams);

    /**
     * B2B交易结果通知到SgxServer接口
     *
     * @param notifyParams 消息参数
     * @return SgxServer的响应消息
     */
    Object paymentByB2BNotifyToSgxServer(String notifyParams);
}
