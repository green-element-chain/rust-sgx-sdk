package com.energy.sgx.socket.dto;

import com.energy.sgx.socket.utils.CommonUtil;
import java.util.Arrays;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * @author Bryan
 * @date 2019-07-17
 */
@Data
public class PlatformInfoBlob {

    private int sgxEPidGroupFlags;
    private long sgxTcbEvaluationFlags;
    private long pseEvaluationFlags;
    private String latestEquivalentTcbPSvn;
    private String latestPseISVSvn;
    private String latestPSdaSvn;
    private long xEid;
    private long gid;
    private SGXEC256Signature sgxEc256Signature;

    public void parsePlatInfo(byte[] piBlobByte, PlatformInfoBlob pfInfo) {
        pfInfo.sgxEc256Signature = new SGXEC256Signature();
        pfInfo.sgxEPidGroupFlags = Byte.toUnsignedInt(piBlobByte[0]);
        pfInfo.sgxTcbEvaluationFlags = computeDec(Arrays.copyOfRange(piBlobByte, 1, 3));
        pfInfo.pseEvaluationFlags = computeDec(Arrays.copyOfRange(piBlobByte, 3, 5));
        pfInfo.latestEquivalentTcbPSvn = byte2Str(Arrays.copyOfRange(piBlobByte, 5, 23));
        pfInfo.latestPseISVSvn = byte2Str(Arrays.copyOfRange(piBlobByte, 23, 25));
        pfInfo.latestPSdaSvn = byte2Str(Arrays.copyOfRange(piBlobByte, 25, 29));
        pfInfo.xEid = computeDec(Arrays.copyOfRange(piBlobByte, 29, 33));
        pfInfo.gid = computeDec(Arrays.copyOfRange(piBlobByte, 33, 37));
        pfInfo.sgxEc256Signature.gx = byte2Str(Arrays.copyOfRange(piBlobByte, 37, 69));
        pfInfo.sgxEc256Signature.gy = byte2Str(Arrays.copyOf(piBlobByte, 69));

    }

    private long computeDec(byte[] piBlobSlice) {
        StringBuilder hexString = new StringBuilder();
        for (int i = piBlobSlice.length - 1; i >= 0; i--) {
            hexString.append(CommonUtil.byteToHex(piBlobSlice[i]));
        }
        return Long.parseLong(hexString.toString(), 16);
    }

    private String byte2Str(byte[] piBlobSlice) {
        StringBuilder piBlobStr = new StringBuilder();
        for (int i = 0; i < piBlobSlice.length; i++) {
            piBlobStr.append(Byte.toUnsignedInt(piBlobSlice[i])).append(", ");
        }
        return "[" + piBlobStr.substring(0, piBlobStr.length() - 2) + "]";
    }

    @Data
    @NoArgsConstructor
    public static class SGXEC256Signature {

        private String gx;
        private String gy;
    }
}
