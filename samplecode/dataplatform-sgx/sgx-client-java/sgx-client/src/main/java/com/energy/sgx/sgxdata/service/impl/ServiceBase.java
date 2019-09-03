package com.energy.sgx.sgxdata.service.impl;

import com.energy.sgx.sgxdata.dto.request.SocketMessage;
import com.energy.sgx.socket.service.LocalSocketClient;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;
import org.springframework.util.StringUtils;

/**
 * @author Bryan
 * @date 2019-09-02
 */
@Slf4j
@Component
class ServiceBase {

    @Autowired
    LocalSocketClient socketClient;

    Object sendDataToSgx(String url, String dataJsonStr) {
        log.info("send request to sgx: {}", url);
        SocketMessage message = new SocketMessage(url, dataJsonStr);
        String response = socketClient.sendData(message);
        if (!StringUtils.isEmpty(response)) {
            log.debug("response from sgx: {}", response);
        }
        return response;
    }
}
