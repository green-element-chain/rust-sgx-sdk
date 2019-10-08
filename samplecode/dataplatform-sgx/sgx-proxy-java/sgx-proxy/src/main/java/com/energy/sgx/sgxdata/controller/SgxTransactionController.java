package com.energy.sgx.sgxdata.controller;

import com.chinapay.secss.SecssConstants;
import com.energy.sgx.sgxdata.dto.response.NotifyResponse;
import com.energy.sgx.sgxdata.service.TransactionService;
import com.energy.sgx.utils.SDKConstants;
import io.swagger.annotations.Api;
import io.swagger.annotations.ApiImplicitParam;
import io.swagger.annotations.ApiImplicitParams;
import io.swagger.annotations.ApiOperation;
import java.net.URLDecoder;
import java.util.Enumeration;
import java.util.HashMap;
import java.util.Map;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import lombok.extern.slf4j.Slf4j;
import org.apache.tomcat.util.codec.binary.Base64;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.util.StringUtils;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import springfox.documentation.annotations.ApiIgnore;

@Slf4j
@Api(description = "SGX分账支付交易相关接口")
@RestController
@RequestMapping("sgx")
public class SgxTransactionController {

    @Autowired
    private TransactionService sgxService;

    @ApiOperation(value = "触发账单分账", notes = "对账单发起SGX Server服务器端的分账交易。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "bill", value = "账单ID", dataType = "int", defaultValue = "1"),
        @ApiImplicitParam(name = "day", value = "分帐日", dataType = "int", defaultValue = "10")
    })
    @PostMapping("transaction/payment")
    public Object postPaymentToSgxServer(
        @RequestParam(required = false) Integer bill,
        @RequestParam(required = false) Integer day) {
        log.info("transaction payment at {}, param: {}, {}", System.currentTimeMillis(), bill, day);
        return sgxService.paymentToSgxServer(bill, day);
    }

    @ApiOperation(value = "刷新分账状态", notes = "对账单发起SGX Server服务器端的分账状态刷新交易。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "bill", value = "账单ID", dataType = "int", defaultValue = "1")
    })
    @PostMapping("transaction/refresh")
    public Object postRefreshToSgxServer(
        @RequestParam(required = false) Integer bill) {
        log.info("transaction status refresh at {}, param: {}", System.currentTimeMillis(), bill);
        return sgxService.paymentRefreshToSgxServer(bill);
    }

    @ApiOperation(value = "对账单发起B2B支付", notes = "对账单发起SGX Server服务器端的B2B支付交易。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "bill", value = "账单ID", required = true, dataType = "int", defaultValue = "1")
    })
    @PostMapping("transaction/payment/b2b/{bill}")
    public Object postPaymentByB2BToSgxServer(
        @PathVariable Integer bill) {
        log.info("data B2B params bill id {}", bill);
        return sgxService.paymentByB2BToSgxServer(bill);
    }

    @ApiIgnore
    @ApiOperation(value = "交易回调后端通知", notes = "分账的回调通知发送到SGX Server服务器处理。")
    @PostMapping("transaction/notify/back")
    public Object notifyPaymentResultToSgxServer(
        final @ApiIgnore HttpServletRequest request) {
        Map<String, String> rspData = getUrlRequestParams(request, false);
        log.info("notify back data params {}", rspData);
        NotifyResponse notifyResponse = (NotifyResponse) sgxService.paymentNotifyToSgxServer(rspData);
        log.info("notify back sgx response : {}", notifyResponse);
        return notifyResponse.getMessage();
    }

    @ApiIgnore
    @ApiOperation(value = "B2B交易回调前端通知", notes = "B2B交易的回调通知发送到SGX Server服务器处理。")
    @PostMapping("transaction/notify/front")
    public void notifyB2BPaymentResultToSgxServer(
        final @ApiIgnore HttpServletRequest request,
        HttpServletResponse response) {
        Map<String, String> rspData = getUrlRequestParams(request, true);
        log.info("notify front data params {}", rspData);
        NotifyResponse notifyResponse = (NotifyResponse) sgxService.paymentNotifyToSgxServer(rspData);
        log.info("notify front sgx response : {}", notifyResponse);
        if (!StringUtils.isEmpty(notifyResponse.getUrl())) {
            try {
                response.sendRedirect(notifyResponse.getUrl());
            } catch (Exception e) {
                log.error("send redirect exception", e);
            }
        }
    }

    @ApiOperation(value = "测试用：加解密测试方法", notes = "")
    @PostMapping("transaction/sign")
    public Object transactionSignTest(
        @RequestParam String data) {
        try {
            String charset = "UTF-8";
            log.info("input data: {}", data);

            byte[] dataBytes = data.getBytes(charset);
            String result = new String(Base64.encodeBase64(dataBytes), charset);

            return result;
        } catch (Exception e) {
            e.printStackTrace();
            return "";
        }
    }

    /**
     * 获取请求参数中所有的信息
     *
     * @param request 银联返回的参数
     */
    private Map<String, String> getUrlRequestParams(HttpServletRequest request, Boolean isFrontNotice) {
        Map<String, String> res = new HashMap<>();
        Enumeration<?> temp = request.getParameterNames();
        if (null != temp) {
            res.put(SDKConstants.param_Notice, isFrontNotice.toString());
            while (temp.hasMoreElements()) {
                String en = (String) temp.nextElement();
                String value = request.getParameter(en);
                if (value != null) {
                    if (!isFrontNotice) {
                        try {
                            value = URLDecoder.decode(value, SecssConstants.CHARSET_COMM);
                        } catch (Exception e) {
                            continue;
                        }
                    }
                    //过滤掉值为空的参数
                    if ("".equals(value)) {
                        continue;
                    }
                    res.put(en, value);
                }
            }
        }
        return res;
    }
}
