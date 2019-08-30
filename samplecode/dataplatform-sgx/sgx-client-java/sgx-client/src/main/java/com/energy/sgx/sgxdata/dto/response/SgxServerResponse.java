package com.energy.sgx.sgxdata.dto.response;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * @author Bryan
 * @date 2019-08-28
 */
@Data
@AllArgsConstructor
@NoArgsConstructor
public class SgxServerResponse {

    private Boolean success;
    private String message;
    private String data;
}
