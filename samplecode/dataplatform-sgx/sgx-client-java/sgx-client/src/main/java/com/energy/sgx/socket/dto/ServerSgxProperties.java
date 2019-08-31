package com.energy.sgx.socket.dto;

import java.io.FileInputStream;
import java.io.InputStream;
import javax.annotation.PostConstruct;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.context.annotation.Configuration;

/**
 * @author Bryan
 * @date 2019-07-17
 */
@Slf4j
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
        cert.updatePath();
    }

    @Data
    @NoArgsConstructor
    public static class SgxCertInfo {

        private static final String CLASS_RESOURCES = "classpath:";
        private boolean isResourceFile = false;

        /** properties in resource file */
        private Boolean serverTrusted;
        private String algorithm;
        private String certificate;
        private String privateKey;
        private String caFile;
        private String output;

        void updatePath() {
            this.certificate = replace(certificate);
            this.privateKey = replace(privateKey);
            this.caFile = replace(caFile);
            this.output = replace(output);
        }

        String replace(String value) {
            if (value.contains(CLASS_RESOURCES)) {
                this.isResourceFile = true;
                return value.replace(CLASS_RESOURCES, "");
            }
            return value;
        }

        public InputStream getInputStream(String fileName) throws Exception {
            InputStream is = (isResourceFile)
                ? this.getClass().getResourceAsStream(fileName)
                : new FileInputStream(fileName);
            log.info("InputStream size: {} {}", is.available(), fileName);
            return is;
        }
    }
}
