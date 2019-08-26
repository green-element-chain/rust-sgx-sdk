package com.energy.sgx.socket.dto;

import java.io.File;
import java.net.URL;
import javax.annotation.PostConstruct;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.context.annotation.Configuration;

/**
 * @author Bryan
 * @date 2019-07-17
 */
@Data
@Configuration
@ConfigurationProperties(value = "server.sgx")
public class ServerSgxProperties {

    /** properties in resource file */
    private String host;
    private Integer port;
    private SgxCertInfo cert;

    @PostConstruct
    private void init() {
        //将properties中的classpath替换成绝对路径
        URL url = this.getClass().getClassLoader().getResource("");
        String fullPath = new File(url.getPath()).getParent();
        cert.updatePath(fullPath);
    }

    @Data
    @NoArgsConstructor
    public static class SgxCertInfo {

        private static final String CLASS_RESOURCES = "classpath:";

        /** properties in resource file */
        private String algorithm;
        private String certificate;
        private String privateKey;
        private String caFile;
        private String output;

        public void updatePath(String fullPath) {
            this.certificate = replace(fullPath, certificate);
            this.privateKey = replace(fullPath, privateKey);
            this.caFile = replace(fullPath, caFile);
            this.output = replace(fullPath, output);
        }

        private String replace(String path, String value) {
            if (value.contains(CLASS_RESOURCES)) {
                return value.replace(CLASS_RESOURCES, path);
            }
            return value;
        }
    }
}
