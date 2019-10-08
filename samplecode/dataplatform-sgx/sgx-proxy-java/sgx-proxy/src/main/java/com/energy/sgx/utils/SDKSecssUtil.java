package com.energy.sgx.utils;

import com.chinapay.secss.CertUtil;
import com.chinapay.secss.LogUtil;
import com.chinapay.secss.SecssConstants;
import com.chinapay.secss.SecssUtil;
import com.chinapay.secss.SecurityException;
import java.io.File;
import java.io.IOException;
import java.io.InputStreamReader;
import java.lang.reflect.Field;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.util.Map;
import java.util.Properties;
import java.util.Set;
import javax.annotation.PostConstruct;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;

/**
 * @author Bryan
 * @date 2019-09-27
 */
@Slf4j
@Component
public class SDKSecssUtil {

    private static String PROPERTIES_FILE = "security.properties";
    private static String CLASS_RESOURCES = "classpath:resources";

    @PostConstruct
    void init() {
        log.info("开始从默认配置文件初始化安全控件.");
        try {
            Properties pros = loadProperties();
            CertUtil.init(pros);
        } catch (Exception var) {
            throw new RuntimeException(var);
        } finally {
            log.info("从默认配置文件初始化安全控件结束.");
        }
    }

    private Properties loadProperties() throws SecurityException {
        Properties properties = null;
        InputStreamReader in = null;
        try {
            URL url = this.getClass().getClassLoader().getResource(PROPERTIES_FILE);
            assert url != null;
            in = new InputStreamReader(url.openStream(), StandardCharsets.UTF_8);
            Properties tempProperties = new Properties();
            tempProperties.load(in);

            try {
                in.close();
            } catch (IOException var1) {
                var1.printStackTrace();
            }
            //将properties中的classpath替换成绝对路径
            String fullPath = new File(url.getPath()).getParent();
            Set<String> propKeys = tempProperties.stringPropertyNames();
            propKeys.forEach(key -> {
                String value = tempProperties.getProperty(key);
                if (value.contains(CLASS_RESOURCES)) {
                    String realValue = value.replace(CLASS_RESOURCES, fullPath);
                    tempProperties.setProperty(key, realValue);
                }
            });
            properties = tempProperties;
        } catch (IOException var2) {
            var2.printStackTrace();
        } finally {
            if (in != null) {
                try {
                    in.close();
                } catch (IOException var3) {
                    var3.printStackTrace();
                }
            }
        }
        if (properties == null) {
            throw new SecurityException(SecssConstants.LOAD_CONFIG_ERROR);
        }
        return properties;
    }

    public SecssUtil getSecssUtil() {
        SecssUtil util = new SecssUtil();
        try {
            Class<?> cls = SecssUtil.class;
            Field f = cls.getDeclaredField("initFlag1");
            f.setAccessible(true);
            f.set(util, true);
        } catch (Exception e) {
            e.printStackTrace();
        }
        return util;
    }

    /**
     * 功能：持卡人信息域customerInfo构造
     *
     * @param customerInfo 已经经过base64加密后的卡域JSON串信息
     * @param secssUtil 银联的加密算法类
     * @return 加密后的卡域信息
     */
    public String encodeCustomerInfo(String customerInfo, SecssUtil secssUtil) {
        LogUtil.writeLog("卡域customerInfo信息：" + customerInfo);
        try {
            secssUtil.encryptData(customerInfo);
        } catch (Exception e) {
            LogUtil.writeErrorLog(e.getMessage(), e);
            throw new RuntimeException(e);
        }
        if (!SecssConstants.SUCCESS.equals(secssUtil.getErrCode())) {
            log.error("failed to encrypt data {}", customerInfo);
            throw new RuntimeException(secssUtil.getErrMsg());
        }
        return secssUtil.getEncValue();
    }

    public void sign(Map<String, String> data, SecssUtil secssUtil) {
        secssUtil.sign(data);
        if (!SecssConstants.SUCCESS.equals(secssUtil.getErrCode())) {
            log.error("failed to sign data {}", data);
            throw new RuntimeException(secssUtil.getErrMsg());
        }
        data.put(SDKConstants.param_signature, secssUtil.getSign());
    }

}
