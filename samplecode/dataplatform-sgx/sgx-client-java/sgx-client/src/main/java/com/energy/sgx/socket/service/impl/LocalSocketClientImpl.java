package com.energy.sgx.socket.service.impl;

import com.energy.sgx.order.dto.SocketMessage;
import com.energy.sgx.socket.service.LocalSocketClient;
import com.energy.sgx.utils.JsonUtil;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.Socket;
import javax.annotation.PostConstruct;
import javax.net.ssl.SSLContext;
import javax.net.ssl.SSLSocketFactory;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;

/**
 * @author Bryan
 * @date 2019-07-24
 */
@Slf4j
@Service
public class LocalSocketClientImpl extends SSLContextBase implements LocalSocketClient {

    private SSLContext sslContextWithVerify = null;

    @PostConstruct
    private void afterConstruct() {
        try {
            this.sslContextWithVerify = createSSLContext();
        } catch (Exception ex) {
            log.error("Socket client construct exception, message: {}", ex.getMessage());
        }
    }

    @Override
    public String sendData(SocketMessage message) {
        Socket socket = null;
        try {
            socket = createSocket();
            if (socket != null) {
                OutputStream requestStream = socket.getOutputStream();
                String request = JsonUtil.toString(message);
                requestStream.write(request.getBytes());
                return getSocketResponse(socket.getInputStream());
            }
        } catch (Exception e) {
            log.error("send socket data exception {}", e.getMessage());
        } finally {
            try {
                if (socket != null) {
                    socket.close();
                }
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
        return null;
    }

    private Socket createSocket() {
        try {
            if (sslContextWithVerify != null) {
                SSLSocketFactory socketFactory = sslContextWithVerify.getSocketFactory();
                Socket socket = socketFactory.createSocket(getProperties().getHost(), getProperties().getPort());
                return socket;
            }
        } catch (Exception e) {
            log.error("create socket exception {}", e.getMessage());
        }
        return null;
    }

    private String getSocketResponse(InputStream in) throws IOException {
        int length;
        byte[] buf = new byte[1024];
        ByteArrayOutputStream bout = new ByteArrayOutputStream();
        while ((length = in.read(buf, 0, buf.length)) > 0) {
            bout.write(buf, 0, length);
        }
        bout.flush();
        return bout.toString();
    }
}
