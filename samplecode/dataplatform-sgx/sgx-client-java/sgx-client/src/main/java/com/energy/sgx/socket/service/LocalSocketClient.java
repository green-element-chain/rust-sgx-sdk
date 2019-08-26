package com.energy.sgx.socket.service;

import com.energy.sgx.order.dto.SocketMessage;
import javax.validation.constraints.NotNull;

/**
 * @author Bryan
 * @date 2019-07-24
 */
public interface LocalSocketClient {

    /**
     * 通过TCPSSL发送数据给SGX服务端，并获取返回消息
     *
     * @param message 请求的数据包
     * @return 服务端的响应消息
     */
    String sendData(@NotNull SocketMessage message);
}
