package com.energy.sgx.sgxdata.service.impl;

import com.chinapay.secss.SecssUtil;
import com.energy.sgx.sgxdata.dto.response.NotifyResponse;
import com.energy.sgx.sgxdata.dto.response.SgxServerResponse;
import com.energy.sgx.sgxdata.dto.response.UnionPayB2BVo;
import com.energy.sgx.sgxdata.service.TransactionService;
import com.energy.sgx.socket.service.LocalHttpClient;
import com.energy.sgx.utils.CollectUtil;
import com.energy.sgx.utils.SDKConstants;
import com.energy.sgx.utils.SDKSecssUtil;
import com.energy.utils.JsonUtil;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.util.ObjectUtils;
import org.springframework.util.StringUtils;

/**
 * @author Bryan
 * @date 2019-08-16
 */
@Slf4j
@Service
public class TransactionServiceImpl extends ServiceBase implements TransactionService {

    @Autowired
    private SDKSecssUtil sdkSecssUtil;
    @Autowired
    private LocalHttpClient httpClient;

    @Data
    @AllArgsConstructor
    @NoArgsConstructor
    static class PaymentBill {

        private Integer bill;
        private Boolean b2b;
    }

    @Override
    public Object paymentToSgxServer(Integer bill, Integer day) {
        if (bill == null && day == null) {
            return "invalid input parameters, bill and day is empty.";
        }

        List<Integer> bills;
        if (bill == null) {
            String responseJson = sendDataToSgx("/project_bill/pay", day.toString()).toString();
            SgxServerResponse sgxResponse = JsonUtil.fromJson(responseJson, SgxServerResponse.class);
            bills = JsonUtil.fromJsonArray(sgxResponse.getData(), Integer.class);
        } else {
            bills = Collections.singletonList(bill);
        }

        List<Integer> succBills = new ArrayList<>();
        SecssUtil secssUtil = sdkSecssUtil.getSecssUtil();
        for (Integer id : bills) {
            PaymentBill pb = new PaymentBill(id, false);
            String unionPayParamJson = sendDataToSgx("/unionpay/trans/param", JsonUtil.toString(pb)).toString();
            SgxServerResponse sgxResponse = JsonUtil.fromJson(unionPayParamJson, SgxServerResponse.class);
            Map<String, String> postParams = CollectUtil.convertJsonStringToMap(sgxResponse.getData());
            if (!sgxResponse.getSuccess() || ObjectUtils.isEmpty(postParams)) {
                log.error("process payment bill {}, {}", pb.bill, sgxResponse.getMessage());
                continue;
            }
            String reqUrl = postParams.remove(SDKConstants.param_postUrl);

            //收款卡域信息加密、交易数据签名
            String customerInfo = postParams.remove(SDKConstants.param_customerInfo);
            customerInfo = sdkSecssUtil.encodeCustomerInfo(customerInfo, secssUtil);
            postParams.put(SDKConstants.param_customerInfo, customerInfo);
            sdkSecssUtil.sign(postParams, secssUtil);

            Map<String, String> transResult = httpClient.post(reqUrl, postParams);
            if (ObjectUtils.isEmpty(transResult)) {
                //根据状态填写失败信息
            }
            transResult.remove(SDKConstants.param_signature);
            transResult.put(SDKConstants.param_orderId, postParams.get(SDKConstants.param_orderId));
            CollectUtil.mapValueURLDecode(transResult);

            String updateResponseJson = sendDataToSgx("/payment/record/update", transResult).toString();
            log.info("payment record update response: {}", updateResponseJson);
            sgxResponse = JsonUtil.fromJson(updateResponseJson, SgxServerResponse.class);
            if (sgxResponse.getSuccess()) {
                succBills.add(id);

            }
        }

        return String.format("success split bills %s", succBills.toString());
    }

    @Override
    public Object paymentByB2BToSgxServer(Integer bill) {
        PaymentBill pb = new PaymentBill(bill, true);
        String unionPayParamJson = sendDataToSgx("/unionpay/param", JsonUtil.toString(pb)).toString();
        SgxServerResponse sgxResponse = JsonUtil.fromJson(unionPayParamJson, SgxServerResponse.class);

        //交易数据签名
        SecssUtil secssUtil = sdkSecssUtil.getSecssUtil();
        Map<String, String> paramMap = CollectUtil.convertResultStringToMap(sgxResponse.getData());
        if (!ObjectUtils.isEmpty(paramMap)) {
            String url = paramMap.remove(SDKConstants.param_postUrl);
            sdkSecssUtil.sign(paramMap, secssUtil);

            UnionPayB2BVo vo = new UnionPayB2BVo(paramMap, url);
            return JsonUtil.toString(vo);
        }
        log.error("failed to b2b payment for bill {}, {}", bill, sgxResponse.getMessage());
        return sgxResponse.getMessage();
    }

    @Override
    public Object paymentNotifyToSgxServer(Map<String, String> notifyParams) {
        SecssUtil secssUtil = sdkSecssUtil.getSecssUtil();
        secssUtil.verify(notifyParams);

        NotifyResponse response = new NotifyResponse("unknown error");
        //if (!SecssConstants.SUCCESS.equals(secssUtil.getErrCode())) {
        //    response.setMessage("notify data verify failed.");
        //} else {
        String params = CollectUtil.coverMap2String(notifyParams);
        String sgxResponseJson = sendDataToSgx("/notify", params).toString();
        log.info("notify sgx response : {}", sgxResponseJson);
        if (!StringUtils.isEmpty(response)) {
            SgxServerResponse sgxResponse = JsonUtil.fromJson(sgxResponseJson, SgxServerResponse.class);
            response.setMessage(sgxResponse.getMessage());
            response.setUrl(sgxResponse.getData());
        }
        //}
        return response;
    }

    @Override
    public Object paymentRefreshToSgxServer(Integer bill) {
        List<Integer> bills;
        if (bill == null) {
            String responseJson = sendDataToSgx("/project_bill/refresh", "").toString();
            SgxServerResponse sgxResponse = JsonUtil.fromJson(responseJson, SgxServerResponse.class);
            bills = JsonUtil.fromJsonArray(sgxResponse.getData(), Integer.class);
        } else {
            bills = Collections.singletonList(bill);
        }

        List<Integer> succBills = new ArrayList<>();
        SecssUtil secssUtil = sdkSecssUtil.getSecssUtil();
        for (Integer id : bills) {
            PaymentBill pb = new PaymentBill(id, false);
            String unionPayParamJson = sendDataToSgx("/unionpay/query/param", JsonUtil.toString(pb)).toString();
            SgxServerResponse sgxResponse = JsonUtil.fromJson(unionPayParamJson, SgxServerResponse.class);
            Map<String, String> postParams = CollectUtil.convertJsonStringToMap(sgxResponse.getData());
            if (!sgxResponse.getSuccess() || ObjectUtils.isEmpty(postParams)) {
                log.error("process refresh bill {}, {}", pb.bill, sgxResponse.getMessage());
                continue;
            }
            String reqUrl = postParams.remove(SDKConstants.param_postUrl);

            //交易数据签名
            sdkSecssUtil.sign(postParams, secssUtil);

            Map<String, String> requestResult = httpClient.post(reqUrl, postParams);
            if (ObjectUtils.isEmpty(requestResult)) {
                //根据状态填写失败信息
            }
            requestResult.remove(SDKConstants.param_signature);
            requestResult.put(SDKConstants.param_orderId, postParams.get(SDKConstants.param_orderId));
            CollectUtil.mapValueURLDecode(requestResult);

            String updateResponseJson = sendDataToSgx("/notify", requestResult).toString();
            log.info("payment record status refresh response: {}", updateResponseJson);
            sgxResponse = JsonUtil.fromJson(updateResponseJson, SgxServerResponse.class);
            if (sgxResponse.getSuccess()) {
                succBills.add(id);
            }
        }

        return String.format("success refresh bills %s", succBills.toString());
    }
}
