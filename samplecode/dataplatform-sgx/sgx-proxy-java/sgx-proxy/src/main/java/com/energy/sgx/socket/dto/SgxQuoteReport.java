package com.energy.sgx.socket.dto;

import lombok.Data;

/**
 * @author Bryan
 * @date 2019-07-17
 */
@Data
public class SgxQuoteReport {

    private String id;
    private String timestamp;
    private int version;
    private String isvEnclaveQuoteStatus;
    private String platformInfoBlob;
    private String isvEnclaveQuoteBody;
}
