package com.energy.sgx.socket.dto;

import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * @author Bryan
 * @date 2019-07-17
 */
@Data
public class QuoteReportData {

    private int version;
    private int signType;
    private QuoteReportBody quoteReportBody;

    public void pareReport(byte[] quoteRep, String repHex, QuoteReportData quoteReportData) {
        quoteReportData.quoteReportBody = new QuoteReportBody();
        quoteReportData.version = Byte.toUnsignedInt(quoteRep[0]);
        quoteReportData.signType = Byte.toUnsignedInt(quoteRep[1]);
        quoteReportData.quoteReportBody.mrEnclave = repHex.substring(224, 288);
        quoteReportData.quoteReportBody.mrSigner = repHex.substring(352, 416);
        quoteReportData.quoteReportBody.reportData = repHex.substring(736, 864);
    }

    @Data
    @NoArgsConstructor
    public static class QuoteReportBody {

        private String mrEnclave;
        private String mrSigner;
        private String reportData;
    }
}
