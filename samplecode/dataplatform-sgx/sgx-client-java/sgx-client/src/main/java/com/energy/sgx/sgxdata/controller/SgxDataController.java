package com.energy.sgx.sgxdata.controller;

import com.energy.sgx.sgxdata.service.DataService;
import io.swagger.annotations.Api;
import io.swagger.annotations.ApiImplicitParam;
import io.swagger.annotations.ApiImplicitParams;
import io.swagger.annotations.ApiOperation;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@Slf4j
@Api(description = "SGX基础数据管理接口")
@RestController
@RequestMapping("sgx")
public class SgxDataController {

    @Autowired
    private DataService dataService;

    @ApiOperation(value = "转移资产订单数据到SGX", notes = "将用于分账的数据写入到SGX Server服务器，在SGX内部自动执行分账。")
    @PostMapping("asset/order/set")
    public Object transferAssetOrderToSgxServer() {
        log.info("transfer asset order data at {}", System.currentTimeMillis());
        return dataService.transferAssetOrderToSgx();
    }

    @ApiOperation(value = "测试用：从SGX查询数据", notes = "从SGX Server服务器查询需要的数据，用于显示使用。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "assetType", value = "资产类型ID", required = true, dataType = "int", defaultValue = "10"),
        @ApiImplicitParam(name = "date", value = "数据日期，例如：'2019-07-16'", required = true, defaultValue = "2019-07-18")
    })
    @GetMapping("asset/order/get/{assetType}/{date}")
    public Object queryAssetOrderFromSgxServer(
        @PathVariable Integer assetType,
        @PathVariable String date) {
        log.info("query data from sgx server, asset type {}, date {}", assetType, date);
        return dataService.queryAssetOrderFromSgx(assetType, date);
    }

    @ApiOperation(value = "转移项目关联资产到SGX", notes = "将用于分账的项目资产写入到SGX Server服务器，让分账协议在SGX内部自动执行。")
    @PostMapping("project/asset/set")
    public Object transferProjectAssetToSgxServer() {
        log.info("transfer project asset data at {}", System.currentTimeMillis());
        return dataService.transferProjectAssetToSgx();
    }

    @ApiOperation(value = "转移项目分账协议到SGX", notes = "将用于分账的分账协议写入到SGX Server服务器，让分账协议在SGX内部自动执行。")
    @PostMapping("project/ledger/set")
    public Object transferProjectLedgerToSgxServer() {
        log.info("transfer project ledger data at {}", System.currentTimeMillis());
        return dataService.transferProjectLedgerToSgx();
    }

    @ApiOperation(value = "从SGX查询项目分账协议", notes = "从SGX Server服务器查询需要的数据，用于显示使用。")
    @GetMapping("project/ledger/get")
    public Object queryProjectLedgerFromSgxServer() {
        log.info("query project ledger from sgx at {}", System.currentTimeMillis());
        return dataService.queryProjectLedgerFromSgx();
    }

    @ApiOperation(value = "转移项目分账卡域信息到SGX", notes = "将用于分账的分账用到的卡域信息写入到SGX Server服务器，让分账协议在SGX内部自动执行。")
    @PostMapping("project/receipt/set")
    public Object transferProjectReceiptToSgxServer() {
        log.info("transfer project receipt data at {}", System.currentTimeMillis());
        return dataService.transferProjectReceiptToSgx();
    }

    @ApiOperation(value = "从SGX查询项目分账卡域信息", notes = "从SGX Server服务器查询需要的数据，用于显示使用。")
    @GetMapping("project/receipt/get")
    public Object queryProjectReceiptFromSgxServer() {
        log.info("query project receipt from sgx at {}", System.currentTimeMillis());
        return dataService.queryProjectReceiptFromSgx();
    }


    @ApiOperation(value = "创建项目账单", notes = "触发SGX Server服务器生成项目的账单数据。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "day", value = "账单日", required = true, dataType = "int", defaultValue = "1"),
        @ApiImplicitParam(name = "projectId", value = "项目ID", dataType = "int", defaultValue = "1")
    })
    @PostMapping("project/bill/create")
    public Object generateProjectBillInSgxServer(
        @RequestParam Integer day,
        @RequestParam(required = false) Integer projectId) {
        log.info("generate project bill data at {}", System.currentTimeMillis());
        if (day < 0 || day >= 30) {
            throw new RuntimeException("Invalid param day");
        }
        return dataService.generateProjectBillToSgx(day, projectId);
    }

    @ApiOperation(value = "从SGX查询项目账单协议", notes = "从SGX Server服务器查询需要的数据，用于显示使用。")
    @GetMapping("project/bill/get")
    public Object queryProjectBillFromSgxServer() {
        log.info("query data from sgx at {}", System.currentTimeMillis());
        return dataService.queryProjectBillFromSgx();
    }
}