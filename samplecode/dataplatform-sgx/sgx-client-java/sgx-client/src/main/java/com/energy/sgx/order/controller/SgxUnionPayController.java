package com.energy.sgx.order.controller;

import com.energy.sgx.order.service.UnionpayService;
import io.swagger.annotations.Api;
import io.swagger.annotations.ApiImplicitParam;
import io.swagger.annotations.ApiImplicitParams;
import io.swagger.annotations.ApiOperation;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.util.ObjectUtils;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import springfox.documentation.annotations.ApiIgnore;

@Slf4j
@Api(description = "Sgx-Unionpay交易相关接口")
@RestController
@RequestMapping("sgx")
public class SgxUnionPayController {

    @Autowired
    private UnionpayService sgxService;

    @ApiOperation(value = "测试用：对账单分账", notes = "对账单发起SGX Server服务器端的分账交易。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "bill", value = "账单ID", required = true, dataType = "int", defaultValue = "1")
    })
    @PostMapping("transaction/payment/{bill}")
    public Object postPaymentToSgxServer(
        final @ApiIgnore HttpServletRequest request,
        @PathVariable Integer bill) {
        log.info("data params bill-id {}", bill);
        Object data = sgxService.paymentToSgxServer(bill);
        if (!ObjectUtils.isEmpty(data)) {
            log.info("sgx response : {}", data.toString());
        }
        return data;
    }

    @ApiOperation(value = "测试用：对账单分账", notes = "对账单发起SGX Server服务器端的B2B支付交易。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "bill", value = "账单ID", required = true, dataType = "int", defaultValue = "1")
    })
    @PostMapping("transaction/payment/b2b/{bill}")
    public Object postPaymentByB2BToSgxServer(
        final @ApiIgnore HttpServletRequest request,
        @PathVariable Integer bill) {
        log.info("data B2B params bill-id {}", bill);
        Object data = sgxService.paymentByB2BToSgxServer(bill);
        if (!ObjectUtils.isEmpty(data)) {
            log.info("sgx B2B response : {}", data.toString());
        }
        return data;
    }

    @ApiOperation(value = "分账交易回调通知", notes = "分账的回调通知发送到SGX Server服务器处理。")
    @PostMapping("transaction/notify/back")
    public Object notifyPaymentResultToSgxServer(
        final @ApiIgnore HttpServletRequest request,
        HttpServletResponse response) {
        String notifyParams = "";
        log.info("notify back data params {}", notifyParams);
        Object data = sgxService.paymentNotifyToSgxServer(notifyParams);
        if (!ObjectUtils.isEmpty(data)) {
            log.info("notify back sgx response : {}", data.toString());
        }
        return data;
    }

    @ApiOperation(value = "B2B交易回调通知", notes = "B2B交易的回调通知发送到SGX Server服务器处理。")
    @PostMapping("transaction/notify/front")
    public Object notifyB2BPaymentResultToSgxServer(
        final @ApiIgnore HttpServletRequest request,
        HttpServletResponse response) {
        String notifyParams = "";
        log.info("notify front data params {}", notifyParams);
        Object data = sgxService.paymentByB2BNotifyToSgxServer(notifyParams);
        if (!ObjectUtils.isEmpty(data)) {
            log.info("notify front sgx response : {}", data.toString());
        }
        return data;
    }
}
