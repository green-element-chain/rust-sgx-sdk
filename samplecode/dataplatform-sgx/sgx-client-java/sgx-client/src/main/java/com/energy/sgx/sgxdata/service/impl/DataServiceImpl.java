package com.energy.sgx.sgxdata.service.impl;

import com.energy.sgx.sgxdata.dto.request.AssetOrderVo;
import com.energy.sgx.sgxdata.dto.request.ProjectAssetVo;
import com.energy.sgx.sgxdata.dto.request.ProjectBillGenVo;
import com.energy.sgx.sgxdata.dto.request.ProjectLedgerVo;
import com.energy.sgx.sgxdata.dto.request.ProjectReceiptVo;
import com.energy.sgx.sgxdata.dto.request.SocketMessage;
import com.energy.sgx.sgxdata.dto.response.LastUpdatedTime;
import com.energy.sgx.sgxdata.dto.response.LastUpdatedTime.Utils;
import com.energy.sgx.sgxdata.dto.response.SgxServerResponse;
import com.energy.sgx.sgxdata.service.DataService;
import com.energy.sgx.sgxdata.service.impl.order.AssetOrderService;
import com.energy.sgx.sgxdata.service.impl.project.ProjectAssetService;
import com.energy.sgx.sgxdata.service.impl.project.ProjectLedgerService;
import com.energy.sgx.sgxdata.service.impl.project.ProjectReceiptService;
import com.energy.utils.JsonUtil;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.util.ObjectUtils;

/**
 * @author Bryan
 * @date 2019-07-24
 */
@Slf4j
@Service
public class DataServiceImpl extends ServiceBase implements DataService {

    @Autowired
    private ProjectAssetService projectAssetService;
    @Autowired
    private ProjectLedgerService projectLedgerService;
    @Autowired
    private ProjectReceiptService projectReceiptService;
    @Autowired
    private AssetOrderService assetOrderService;

    private LastUpdatedTime getLastUpdatedTime(String url) {
        SocketMessage message = new SocketMessage(url, "");
        String updateTimeResp = socketClient.sendData(message);
        log.info("{}", updateTimeResp);
        return Utils.fromJson(updateTimeResp);
    }

    private <T> Object batchSendDataListToSgx(String url, List<List<T>> dataList) {
        AtomicInteger total = new AtomicInteger();
        dataList.forEach(list -> {
            Object object = sendDataToSgx(url, JsonUtil.toString(list));
            log.info("{}, size {}", object, list.size());
            total.set(total.get() + list.size());
        });

        String result = "transfer data to sgx success. total: " + total.intValue();
        log.info("{}", result);
        return result;
    }

    private Object queryDataListFromSgx(String url, String param) {
        SocketMessage message = new SocketMessage(url, param);
        String responseJson = socketClient.sendData(message);
        SgxServerResponse response = JsonUtil.fromJson(responseJson, SgxServerResponse.class);
        return response.getData();
    }

    @Override
    public Object transferAssetOrderToSgx() {
        LastUpdatedTime updatedTime = getLastUpdatedTime("/order_data/lastUpdateTime");
        if (updatedTime.invalidTime()) {
            return updatedTime.getMessage();
        }

        List<List<AssetOrderVo>> assetOrders = assetOrderService.get(updatedTime.getLastTime());
        if (ObjectUtils.isEmpty(assetOrders)) {
            return "查询不到资产定单，请确认是否存在资产的定单数据。";
        }

        return batchSendDataListToSgx("/order_data/set", assetOrders);
    }

    @Override
    public Object queryAssetOrderFromSgx(Integer assetType, String date) {
        SocketMessage message = new SocketMessage("/order_data/get", "");
        String responseJson = socketClient.sendData(message);
        SgxServerResponse response = JsonUtil.fromJson(responseJson, SgxServerResponse.class);
        return response.getData();
    }

    @Override
    public Object transferProjectAssetToSgx() {
        LastUpdatedTime updatedTime = getLastUpdatedTime("/project_asset/lastUpdateTime");
        if (updatedTime.invalidTime()) {
            return updatedTime.getMessage();
        }

        List<List<ProjectAssetVo>> projectAssets = projectAssetService.get(updatedTime.getLastTime());
        if (ObjectUtils.isEmpty(projectAssets)) {
            return "查询不到项目资产，请确认项目是否绑定了资产。";
        }

        return batchSendDataListToSgx("/project_asset/set", projectAssets);
    }

    @Override
    public Object transferProjectLedgerToSgx() {
        LastUpdatedTime updatedTime = getLastUpdatedTime("/project_ledger/lastUpdateTime");
        if (updatedTime.invalidTime()) {
            return updatedTime.getMessage();
        }

        List<List<ProjectLedgerVo>> projectLedgers = projectLedgerService.get(updatedTime.getLastTime());
        if (ObjectUtils.isEmpty(projectLedgers)) {
            return "查询不到项目分账协议，请确认项目分账协议是否已经设置上链。";
        }

        return batchSendDataListToSgx("/project_ledger/set", projectLedgers);
    }

    @Override
    public Object queryProjectLedgerFromSgx() {
        return queryDataListFromSgx("/project_ledger/get", "");
    }

    @Override
    public Object transferProjectReceiptToSgx() {
        LastUpdatedTime updatedTime = getLastUpdatedTime("/project_receipt/lastUpdateTime");
        if (updatedTime.invalidTime()) {
            return updatedTime.getMessage();
        }

        List<List<ProjectReceiptVo>> projectReceipts = projectReceiptService.get(updatedTime.getLastTime());
        if (ObjectUtils.isEmpty(projectReceipts)) {
            return "查询不到项目分账协议，请确认项目分账协议是否已经设置上链。";
        }

        return batchSendDataListToSgx("/project_receipt/set", projectReceipts);
    }

    @Override
    public Object queryProjectReceiptFromSgx() {
        return queryDataListFromSgx("/project_receipt/get", "");
    }

    @Override
    public Object generateProjectBillToSgx(Integer day, Integer projectId) {
        ProjectBillGenVo billGenVo = new ProjectBillGenVo(day);

        if (projectId != null) {
            billGenVo.add(projectId);
        }
        String dataJsonStr = JsonUtil.toString(billGenVo);
        return sendDataToSgx("/project_bill/create", dataJsonStr);
    }

    @Override
    public Object queryProjectBillFromSgx() {
        return queryDataListFromSgx("/project_bill/get", "");
    }
}
