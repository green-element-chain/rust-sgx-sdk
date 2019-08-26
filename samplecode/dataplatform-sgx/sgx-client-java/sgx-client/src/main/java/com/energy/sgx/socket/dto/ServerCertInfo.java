package com.energy.sgx.socket.dto;

import java.util.List;
import lombok.AllArgsConstructor;
import lombok.Data;

/**
 * @author Bryan
 * @date 2019-07-17
 */
@Data
@AllArgsConstructor
public class ServerCertInfo {

    public List<Byte> payload;
    public byte[] pubKey;
}
