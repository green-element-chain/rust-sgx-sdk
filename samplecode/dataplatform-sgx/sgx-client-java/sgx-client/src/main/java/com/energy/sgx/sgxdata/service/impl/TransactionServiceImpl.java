package com.energy.sgx.sgxdata.service.impl;

import com.energy.sgx.sgxdata.dto.request.SocketMessage;
import com.energy.sgx.sgxdata.service.TransactionService;
import com.energy.utils.JsonUtil;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.util.StringUtils;

/**
 * @author Bryan
 * @date 2019-08-16
 */
@Slf4j
@Service
public class TransactionServiceImpl extends ServiceBase implements TransactionService {

    @Data
    @AllArgsConstructor
    @NoArgsConstructor
    static class PaymentBill {

        private Integer bill;
        private Integer day;
    }

    @Data
    @AllArgsConstructor
    static class NotifyInfo {

        private String param;
    }

    private String getPaymentBillParam(Integer bill, Integer day) {
        PaymentBill paymentBill = new PaymentBill();
        paymentBill.setBill((bill != null) ? bill : 0);
        paymentBill.setDay((day != null) ? day : 0);

        return JsonUtil.toString(paymentBill);
    }

    @Override
    public Object paymentToSgxServer(Integer bill, Integer day) {
        String dataJsonStr = getPaymentBillParam(bill, day);
        return sendDataToSgx("/payment", dataJsonStr);
    }

    @Override
    public Object paymentByB2BToSgxServer(Integer bill) {
        String dataJsonStr = getPaymentBillParam(bill, null);
        return sendDataToSgx("/payment/b2b", dataJsonStr);
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
