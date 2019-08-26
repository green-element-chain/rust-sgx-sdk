package com.energy.sgx.order.service;

/**
 * @author Bryan
 * @date 2019-08-16
 */
public interface UnionpayService {

    /**
     * 分账交易接口，通过接口触发SgxServer的分账交易
     *
     * @param bill 账单唯一编号
     * @return SgxServer的响应消息
     */
    Object paymentToSgxServer(Integer bill);

    /**
     * B2B分账交易接口，通过接口触发SgxServer的B2B分账交易
     *
     * @param bill 账单唯一编号
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
