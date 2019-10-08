/**
 * Licensed Property to China UnionPay Co., Ltd.
 *
 * (C) Copyright of China UnionPay Co., Ltd. 2010 All Rights Reserved.
 *
 *
 * Modification History: ============================================================================= Author Date Description
 * ------------ ---------- --------------------------------------------------- xshu 2014-05-28 HTTP通信工具类
 * =============================================================================
 */
package com.energy.sgx.socket.service.impl;

import com.chinapay.secss.LogUtil;
import com.chinapay.secss.SecssConstants;
import com.energy.sgx.socket.service.LocalHttpClient;
import com.energy.sgx.socket.service.impl.HttpSSLSocketFactory.TrustAnyHostnameVerifier;
import com.energy.sgx.utils.CollectUtil;
import java.io.BufferedReader;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.PrintStream;
import java.io.UnsupportedEncodingException;
import java.net.HttpURLConnection;
import java.net.MalformedURLException;
import java.net.URL;
import java.net.URLConnection;
import java.net.URLEncoder;
import java.util.HashMap;
import java.util.Map;
import java.util.Map.Entry;
import javax.net.ssl.HttpsURLConnection;
import lombok.Data;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Service;
import org.springframework.util.StringUtils;

/**
 * 发送Http Restful请求到银联系统
 *
 * @author Bryan
 * @date 2019-09-30
 */
@Data
@Slf4j
@Service
public class LocalHttpClientImpl implements LocalHttpClient {

    class HttpConnection {

        public static final int STATUS_SUCCESS = 200;

        /** 目标地址 */
        private URL url;

        /** 通信连接超时时间 */
        private int connectionTimeout;

        /** 通信读超时时间 */
        private int readTimeOut;

        /** 通信结果 */
        private String result;

        /** 获取通信结果 */
        public String getResult() {
            return result;
        }

        /**
         * 设置通信结果
         *
         * @param result
         */
        public void setResult(String result) {
            this.result = result;
        }

        /**
         * 构造函数
         *
         * @param url 目标地址
         * @param connectionTimeout HTTP连接超时时间
         * @param readTimeOut HTTP读写超时时间
         */
        public HttpConnection(String url, int connectionTimeout, int readTimeOut) {
            try {
                this.url = new URL(url);
                this.connectionTimeout = connectionTimeout;
                this.readTimeOut = readTimeOut;
            } catch (MalformedURLException e) {
                LogUtil.writeErrorLog(e.getMessage(), e);
            }
        }

        /**
         * 发送Post请求到服务端
         *
         * @param data
         * @param encoding
         */
        public int sendPost(Map<String, String> data, String encoding) throws Exception {
            try {
                HttpURLConnection httpConnection = createConnectionPost(encoding);
                if (null == httpConnection) {
                    throw new Exception("Create httpURLConnection Failure");
                }
                String sendData = this.getRequestParamString(data, encoding);
                LogUtil.writeLog("请求报文(对每个报文域的值均已做url编码):[" + sendData + "]");
                log.info("{}", httpConnection.getRequestProperties());
                this.requestServer(httpConnection, sendData, encoding);
                this.result = this.response(httpConnection, encoding);
                LogUtil.writeLog("Response message:[" + result + "]");
                return httpConnection.getResponseCode();
            } catch (Exception e) {
                throw e;
            }
        }

        /**
         * 创建Post连接
         */
        private HttpURLConnection createConnectionPost(String encoding) {
            try {
                HttpURLConnection httpURLConnection = (HttpURLConnection) url.openConnection();
                httpURLConnection.setConnectTimeout(this.connectionTimeout);
                httpURLConnection.setReadTimeout(this.readTimeOut);
                httpURLConnection.setDoInput(true);
                httpURLConnection.setDoOutput(true);
                httpURLConnection.setUseCaches(false);
                httpURLConnection.setRequestProperty("Content-type",
                    "application/x-www-form-urlencoded;charset=" + encoding);
                httpURLConnection.setRequestMethod("POST");
                if ("https".equalsIgnoreCase(url.getProtocol())) {
                    HttpsURLConnection husn = (HttpsURLConnection) httpURLConnection;
                    husn.setSSLSocketFactory(new HttpSSLSocketFactory());
                    /** 解决由于服务器证书问题导致HTTPS无法访问的情况 */
                    husn.setHostnameVerifier(new TrustAnyHostnameVerifier());
                    return husn;
                }
                return httpURLConnection;
            } catch (IOException e) {
                LogUtil.writeErrorLog(e.getMessage(), e);
                return null;
            }
        }

        /**
         * 将Map存储的对象，转换为key=value&key=value的字符
         *
         * @param requestParam
         * @param coder
         */
        private String getRequestParamString(Map<String, String> requestParam, String coder) {
            if (null == coder || "".equals(coder)) {
                coder = "UTF-8";
            }
            StringBuffer sf = new StringBuffer();
            String reqstr = "";
            if (null != requestParam && 0 != requestParam.size()) {
                for (Entry<String, String> en : requestParam.entrySet()) {
                    try {
                        sf.append(en.getKey()
                            + "="
                            + (null == en.getValue() || "".equals(en.getValue()) ? ""
                            : URLEncoder.encode(en.getValue(), coder)) + "&");
                    } catch (UnsupportedEncodingException e) {
                        LogUtil.writeErrorLog(e.getMessage(), e);
                        return "";
                    }
                }
                reqstr = sf.substring(0, sf.length() - 1);
            }
            LogUtil.writeLog("Request Message:[" + reqstr + "]");
            return reqstr;
        }

        /**
         * HTTP Post发送消息
         *
         * @param connection
         * @param message
         */
        private void requestServer(final URLConnection connection, String message, String encoder) throws Exception {
            PrintStream out = null;
            try {
                connection.connect();
                out = new PrintStream(connection.getOutputStream(), false, encoder);
                out.print(message);
                out.flush();
            } catch (Exception e) {
                throw e;
            } finally {
                if (null != out) {
                    out.close();
                }
            }
        }

        /**
         * 显示Response消息
         *
         * @param connection
         */
        private String response(final HttpURLConnection connection, String encoding) throws Exception {
            InputStream in = null;
            StringBuilder sb = new StringBuilder(1024);
            BufferedReader br = null;
            try {
                if (STATUS_SUCCESS == connection.getResponseCode()) {
                    in = connection.getInputStream();
                    sb.append(new String(read(in), encoding));
                } else {
                    in = connection.getErrorStream();
                    sb.append(new String(read(in), encoding));
                }
                LogUtil.writeLog("HTTP Return Status-Code:[" + connection.getResponseCode() + "]");
                return sb.toString();
            } catch (Exception e) {
                throw e;
            } finally {
                if (null != br) {
                    br.close();
                }
                if (null != in) {
                    in.close();
                }
                if (null != connection) {
                    connection.disconnect();
                }
            }
        }

        private byte[] read(InputStream in) throws IOException {
            byte[] buf = new byte[1024];
            int length = 0;
            ByteArrayOutputStream bout = new ByteArrayOutputStream();
            while ((length = in.read(buf, 0, buf.length)) > 0) {
                bout.write(buf, 0, length);
            }
            bout.flush();
            return bout.toByteArray();
        }
    }

    /**
     * 发送Post请求给服务器
     *
     * @param reqUrl 请求URL地址
     * @param reqData 请求的数据包
     * @return 服务器相应数据，包含Http请求响应码
     */
    @Override
    public Map<String, String> post(String reqUrl, Map<String, String> reqData) {
        String encoding = SecssConstants.CHARSET_COMM;
        log.info("请求银联地址:" + reqUrl);
        Map<String, String> rspData = new HashMap<>();
        HttpConnection hc = new HttpConnection(reqUrl, 30000, 30000);
        try {
            int status = hc.sendPost(reqData, encoding);
            if (HttpConnection.STATUS_SUCCESS == status) {
                String resultString = hc.getResult();
                if (!StringUtils.isEmpty(resultString)) {
                    Map<String, String> tmpRspData = CollectUtil.convertJsonStringToMap(resultString);
                    rspData.putAll(tmpRspData);
                }
            } else {
                log.error("返回http状态码[" + status + "]，请检查请求报文或者请求地址是否正确");
            }
        } catch (Exception e) {
            log.error(e.getMessage(), e);
        }
        return rspData;
    }
}
