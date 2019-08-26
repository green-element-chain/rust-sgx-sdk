package com.energy.sgx.order.controller;

import com.energy.sgx.order.service.DataService;
import io.swagger.annotations.Api;
import io.swagger.annotations.ApiImplicitParam;
import io.swagger.annotations.ApiImplicitParams;
import io.swagger.annotations.ApiOperation;
import javax.servlet.http.HttpServletRequest;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import springfox.documentation.annotations.ApiIgnore;

@Slf4j
@Api(description = "基础资产管理接口")
@RestController
@RequestMapping("sgx")
public class SgxDataController {

    @Autowired
    private DataService dataService;

    @ApiOperation(value = "写入订单数据到SGX", notes = "将用于分账的数据写入到SGX Server服务器，在SGX内部自动执行分账。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "assetType", value = "资产类型ID", required = true, dataType = "int", defaultValue = "10")
    })
    @PostMapping("order/{assetType}/set")
    public Object postDataToSgxServer(
        final @ApiIgnore HttpServletRequest request,
        @PathVariable Integer assetType) {
        log.info("set data params asset type {}", assetType);
        return dataService.sendOrderToSgx(assetType);
    }

    @ApiOperation(value = "测试用：从SGX查询数据", notes = "从SGX Server服务器查询需要的数据，用于显示使用。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "assetType", value = "资产类型ID", required = true, dataType = "int", defaultValue = "10"),
        @ApiImplicitParam(name = "date", value = "数据日期，例如：'2019-07-16'", required = true, defaultValue = "2019-07-18")
    })
    @GetMapping("order/{assetType}/get/{date}")
    public Object getDataFromSgxServer(
        final @ApiIgnore HttpServletRequest request,
        @PathVariable Integer assetType,
        @PathVariable String date) {
        log.info("data params asset type {}, date {}", assetType, date);
        return dataService.getOrderFromSgx(assetType, date);
    }

    @ApiOperation(value = "写入项目关联资产到SGX", notes = "将用于分账的项目资产写入到SGX Server服务器，让分账协议在SGX内部自动执行。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "projectId", value = "项目ID", required = true, dataType = "int", defaultValue = "1")
    })
    @PostMapping("asset/{projectId}/set")
    public Object postProjectAssetToSgxServer(
        final @ApiIgnore HttpServletRequest request,
        @PathVariable Integer projectId) {
        log.info("set asset params project {}", projectId);
        return dataService.sendProjectAssetToSgx(projectId);
    }

    @ApiOperation(value = "写入项目分账协议到SGX", notes = "将用于分账的分账协议写入到SGX Server服务器，让分账协议在SGX内部自动执行。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "projectId", value = "项目ID", required = true, dataType = "int", defaultValue = "1")
    })
    @PostMapping("ledger/{projectId}/set")
    public Object postProjectLedgerToSgxServer(
        final @ApiIgnore HttpServletRequest request,
        @PathVariable Integer projectId) {
        log.info("set ledger params project {}", projectId);
        return dataService.sendProjectLedgerToSgx(projectId);
    }

    @ApiOperation(value = "测试用：生成项目账单", notes = "触发SGX Server服务器生成项目的账单数据。")
    @ApiImplicitParams({
        @ApiImplicitParam(name = "projectId", value = "项目ID", required = true, dataType = "int", defaultValue = "1")
    })
    @PostMapping("bill/{projectId}/create")
    public Object postProjectBillInSgxServer(
        final @ApiIgnore HttpServletRequest request,
        @PathVariable Integer projectId) {
        log.info("set ledger params project {}", projectId);
        return dataService.sendCreateProjectBillToSgx(projectId);
    }
}