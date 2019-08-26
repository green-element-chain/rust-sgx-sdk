package com.energy.sgx;

import com.energy.framework.swagger.EnableSpringBootSwagger;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cache.annotation.EnableCaching;
import org.springframework.scheduling.annotation.EnableScheduling;

/**
 * @author Bryan
 * @date 2019-07-24
 */
@EnableCaching
@EnableScheduling
@EnableSpringBootSwagger
@SpringBootApplication
public class SgxClientApplication {

    public static void main(String[] args) {
        SpringApplication.run(SgxClientApplication.class, args);
    }
}
