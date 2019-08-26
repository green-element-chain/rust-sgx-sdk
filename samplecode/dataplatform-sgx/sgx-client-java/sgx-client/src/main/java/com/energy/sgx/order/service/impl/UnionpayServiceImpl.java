package com.energy.sgx.order.service.impl;

import com.energy.sgx.order.dto.SocketMessage;
import com.energy.sgx.order.service.UnionpayService;
import com.energy.sgx.socket.service.LocalSocketClient;
import com.energy.sgx.utils.JsonUtil;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.util.StringUtils;

/**
 * @author Bryan
 * @date 2019-08-16
 */
@Slf4j
@Service
public class UnionpayServiceImpl implements UnionpayService {

    @Autowired
    private LocalSocketClient socketClient;

    @Data
    @AllArgsConstructor
    static class PaymentInfo {

        private Integer bill;
    }

    @Data
    @AllArgsConstructor
    static class NotifyInfo {

        private String param;
    }

    @Override
    public Object paymentToSgxServer(Integer bill) {
        PaymentInfo info = new PaymentInfo(bill);
        SocketMessage message = new SocketMessage("/payment", JsonUtil.toString(info));
        String response = socketClient.sendData(message);
        if (!StringUtils.isEmpty(response)) {
            log.info("payment sgx response : {}", response);
        }
        return response;
    }

    @Override
    public Object paymentByB2BToSgxServer(Integer bill) {
        PaymentInfo info = new PaymentInfo(bill);
        SocketMessage message = new SocketMessage("/payment/b2b", JsonUtil.toString(info));
        String response = socketClient.sendData(message);
        if (!StringUtils.isEmpty(response)) {
            log.info("payment B2B sgx response : {}", response);
        }
        return null;
    }

    @Override
    public Object paymentNotifyToSgxServer(String notifyParams) {
        NotifyInfo info = new NotifyInfo(notifyParams);
        SocketMessage message = new SocketMessage("/notify", JsonUtil.toString(info));
        String response = socketClient.sendData(message);
        if (!StringUtils.isEmpty(response)) {
            log.info("notify sgx response : {}", response);
        }
        return null;
    }

    @Override
    public Object paymentByB2BNotifyToSgxServer(String notifyParams) {
        NotifyInfo info = new NotifyInfo(notifyParams);
        SocketMessage message = new SocketMessage("/notify/b2b", JsonUtil.toString(info));
        String response = socketClient.sendData(message);
        if (!StringUtils.isEmpty(response)) {
            log.info("notify B2B sgx response : {}", response);
        }
        return null;
    }
}
