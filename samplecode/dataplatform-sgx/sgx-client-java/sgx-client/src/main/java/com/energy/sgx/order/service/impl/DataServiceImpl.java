package com.energy.sgx.order.service.impl;

import com.energy.sgx.order.dto.OrderDataVo;
import com.energy.sgx.order.dto.ProjectAssetVo;
import com.energy.sgx.order.dto.ProjectLedgerVo;
import com.energy.sgx.order.dto.SocketMessage;
import com.energy.sgx.order.service.DataService;
import com.energy.sgx.socket.service.LocalSocketClient;
import com.energy.sgx.utils.JsonUtil;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.util.StringUtils;

/**
 * @author Bryan
 * @date 2019-07-24
 */
@Slf4j
@Service
public class DataServiceImpl implements DataService {

    private static AtomicInteger orderId = new AtomicInteger(0);

    @Autowired
    private LocalSocketClient socketClient;

    @Override
    public Object sendOrderToSgx(Integer assetType) {
        List<OrderDataVo> orderList = new ArrayList<>();
        for (int i = 0, j = 0; i < 10; i++) {
            int id = orderId.incrementAndGet();
            OrderDataVo orderData = new OrderDataVo();
            orderData.setOrderId(id);
            orderData.setAssetType(assetType);
            orderData.setAssetId(++j);
            orderData.setRevenue(100 * id);
            orderData.setOrderTime(System.currentTimeMillis() / 1000);
            orderList.add(orderData);
        }
        SocketMessage message = new SocketMessage("/order_data/set", JsonUtil.toString(orderList));
        String response = socketClient.sendData(message);
        if (!StringUtils.isEmpty(response)) {
            log.info("order response : {}", response);
        }
        return response;
    }

    @Override
    public Object getOrderFromSgx(Integer assetType, String date) {
        SocketMessage message = new SocketMessage("/order_data/get", "");
        String orderData = socketClient.sendData(message);
        //TODO-xb Bryan.Xu, 2019-08-12 11:19:55, 处理业务数据
        return orderData;
    }

    @Override
    public Object sendProjectAssetToSgx(Integer projectId) {
        List<ProjectAssetVo> projects = new ArrayList<>();
        for (int i = 0, j = 0; i < 2; i++) {
            ProjectAssetVo assetVo = new ProjectAssetVo();
            assetVo.setProjectId(i + 1);
            assetVo.setAssets(Arrays.asList(++j, ++j, ++j, ++j, ++j, ++j, ++j, ++j, ++j, ++j));
            projects.add(assetVo);
        }

        SocketMessage message = new SocketMessage("/project_asset/set", JsonUtil.toString(projects));
        String response = socketClient.sendData(message);
        if (!StringUtils.isEmpty(response)) {
            log.info("asset response : {}", response);
        }
        return response;
    }

    @Override
    public Object sendProjectLedgerToSgx(Integer projectId) {
        ProjectLedgerVo ledgerVo = new ProjectLedgerVo();
        ledgerVo.setProjectId(1);
        ledgerVo.setBillDate(3);
        ledgerVo.setBillStartDate("2019-08-10");
        ledgerVo.setBillCycle(1);
        ledgerVo.setLedgerDate(10);
        ledgerVo.setLedgerContent("999991905069034^60%;999991905069035^40%");

        SocketMessage message = new SocketMessage("/project_ledger/set", JsonUtil.toString(ledgerVo));
        String response = socketClient.sendData(message);
        if (!StringUtils.isEmpty(response)) {
            log.info("ledger response : {}", response);
        }
        return response;
    }

    @Override
    public Object sendCreateProjectBillToSgx(Integer projectId) {
        SocketMessage message = new SocketMessage("/project_bill/create", projectId.toString());
        String response = socketClient.sendData(message);
        if (!StringUtils.isEmpty(response)) {
            log.info("create bill response : {}", response);
        }
        return response;
    }
}
