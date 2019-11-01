package com.energy.sgx.socket.utils;

import com.energy.sgx.socket.dto.PlatformInfoBlob;
import com.energy.sgx.socket.dto.QuoteReportData;
import com.energy.sgx.socket.dto.ServerCertInfo;
import com.energy.sgx.socket.dto.SgxQuoteReport;
import com.energy.utils.JsonUtil;
import com.sun.org.apache.xerces.internal.impl.dv.util.HexBin;
import java.io.BufferedReader;
import java.io.ByteArrayInputStream;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.security.Signature;
import java.security.cert.CertificateFactory;
import java.security.cert.X509Certificate;
import java.util.Arrays;
import java.util.Base64;
import java.util.List;
import lombok.extern.slf4j.Slf4j;
import org.bouncycastle.cert.X509CertificateHolder;
import org.bouncycastle.cert.jcajce.JcaX509CertificateConverter;
import org.bouncycastle.openssl.PEMParser;

/**
 * @author Bryan
 * @date 2019-07-17
 */
@Slf4j
public class VerifyMarshalCert {

    public ServerCertInfo unMarshalByte(List<Byte> certList) {
        // Search for Public Key prime256v1 OID
        String[] prime256v1_oid_string = new String[]{"0x06",
            "0x08", "0x2a", "0x86", "0x48", "0xce", "0x3d", "0x03", "0x01", "0x07"};

        List<Byte> prime256v1_oid = SocketUtil.string2BytesList(prime256v1_oid_string);
        int offset = SocketUtil.getIndexOf(certList, prime256v1_oid);
        // 10 + TAG (0x03)
        offset += 11;

        // Obtain Public Key length
        int length = Byte.toUnsignedInt(certList.get(offset));
        if (length > Byte.toUnsignedInt(SocketUtil.hexToByte("80"))) {
            length = Byte.toUnsignedInt(certList.get(offset + 1)) * 256 + Byte.toUnsignedInt(certList.get(offset + 2));
            offset += 2;
        }

        // Obtain Public Key
        offset += 1;
        // skip "00 04"
        byte[] pub_k = SocketUtil.list2array(certList.subList(offset + 2, offset + length));

        String[] ns_cmt_oid_string = new String[]{"0x06",
            "0x09", "0x60", "0x86", "0x48", "0x01", "0x86", "0xf8", "0x42", "0x01", "0x0d"};
        List<Byte> ns_cmt_oid = SocketUtil.string2BytesList(ns_cmt_oid_string);
        offset = SocketUtil.getIndexOf(certList, ns_cmt_oid);
        // 10 + TAG (0x03)
        offset += 12;

        // Obtain Netscape Comment length
        length = Byte.toUnsignedInt(certList.get(offset));
        if (length > Byte.toUnsignedInt(SocketUtil.hexToByte("80"))) {
            length = Byte.toUnsignedInt(certList.get(offset + 1)) * 256 + Byte.toUnsignedInt(certList.get(offset + 2));
            offset += 2;
        }

        offset += 1;
        List<Byte> payload = certList.subList(offset, offset + length);

        return new ServerCertInfo(payload, pub_k);
    }

    public byte[] verifyCert(InputStream is, List<Byte> payload) throws Exception {
        Base64.Decoder decoder = Base64.getDecoder();

        int startIndex = payload.indexOf(SocketUtil.hexToByte("7c"));
        int endIndex = payload.lastIndexOf(SocketUtil.hexToByte("7c"));
        byte[] attnReportRaw = SocketUtil.list2array(payload.subList(0, startIndex));

        PEMParser pemParser = null;
        try {
            pemParser = new PEMParser(new BufferedReader(new InputStreamReader(is)));
            X509CertificateHolder x509CertificateHolder = (X509CertificateHolder) pemParser.readObject();
            X509Certificate provider = new JcaX509CertificateConverter().getCertificate(x509CertificateHolder);

            CertificateFactory cf = CertificateFactory.getInstance("X509");
            byte[] sigCertRaw = SocketUtil.list2array(payload.subList(endIndex + 1, payload.size()));
            byte[] sigCert = decoder.decode(sigCertRaw);
            X509Certificate server = (X509Certificate) cf.generateCertificate(new ByteArrayInputStream(sigCert));
            server.verify(provider.getPublicKey());

            log.info("Cert is good");
            Signature signature = Signature.getInstance(server.getSigAlgName());
            signature.initVerify(server);
            signature.update(attnReportRaw);

            byte[] sigRaw = SocketUtil.list2array(payload.subList(startIndex + 1, endIndex));
            byte[] sig = decoder.decode(sigRaw);
            if (!signature.verify(sig)) {
                throw new Exception("failed to parse root certificate");
            }
        } finally {
            try {
                if (pemParser != null) {
                    pemParser.close();
                }
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
        log.info("Signature good");
        return attnReportRaw;
    }

    public void verifyAttnReport(byte[] attnReportRaw, byte[] pubK) throws Exception {
        //extract data from attReportJson
        StringBuilder attReportJson = new StringBuilder();
        for (int i = 0; i < attnReportRaw.length; i++) {
            attReportJson.append((char) attnReportRaw[i]);
        }
        SgxQuoteReport sgxQr = JsonUtil.fromJson(attReportJson.toString(), SgxQuoteReport.class);

        //1 Check timestamp is within 24H
        if (sgxQr.getTimestamp().length() != 0) {
            String timeFixed = sgxQr.getTimestamp() + 'Z';
            log.info("Time diff = {}", timeFixed);
            /*DateTime dateTime = new DateTime(timeFixed);
            DateTime now = new DateTime();
            Interval interval = new Interval(dateTime.getMillis(), now.getMillis());
            log.info("Time diff =  %d\n", Seconds.secondsIn(interval).getSeconds());*/
        } else {
            throw new Exception("Failed to fetch timestamp from attestation report");
        }

        //2 Verify quote status (mandatory field)
        if (sgxQr.getIsvEnclaveQuoteStatus().length() != 0) {
            log.info("isvEnclaveQuoteStatus = {}", sgxQr.getIsvEnclaveQuoteStatus());
            switch (sgxQr.getIsvEnclaveQuoteStatus()) {
                case "OK":
                    break;
                case "GROUP_OUT_OF_DATE":
                case "GROUP_REVOKED":
                case "CONFIGURATION_NEEDED":
                    if (sgxQr.getPlatformInfoBlob().length() != 0) {
                        byte[] pfBlob = HexBin.decode(sgxQr.getPlatformInfoBlob());
                        PlatformInfoBlob platformInfoBlob = new PlatformInfoBlob();
                        platformInfoBlob.parsePlatInfo(Arrays.copyOfRange(pfBlob, 4, pfBlob.length), platformInfoBlob);
                        log.info("Platform info is: {}", JsonUtil.toString(platformInfoBlob));
                    } else {
                        throw new Exception("Failed to fetch platformInfoBlob from attestation report");
                    }
                    break;
                default:
                    throw new Exception("SGX_ERROR_UNEXPECTED");
            }
        } else {
            throw new Exception("Failed to fetch isvEnclaveQuoteStatus from attestation report");
        }

        // 3 Verify quote body
        if (sgxQr.getIsvEnclaveQuoteBody().length() != 0) {
            Base64.Decoder decoder = Base64.getDecoder();
            byte[] qb = decoder.decode(sgxQr.getIsvEnclaveQuoteBody());
            String qbString = new String();
            String qbBytes = new String();
            String pubKeyString = new String();
            for (int i = 0; i < qb.length; i++) {
                qbBytes += String.format("%d, ", Byte.toUnsignedInt(qb[i]));
                qbString += String.format("%02x", qb[i]);
            }
            for (int i = 0; i < pubK.length; i++) {
                pubKeyString += String.format("%02x", pubK[i]);
            }

            QuoteReportData quoteReportData = new QuoteReportData();
            quoteReportData.pareReport(qb, qbString, quoteReportData);
            log.info("Quote = [" + qbBytes.substring(0, qbBytes.length() - 2) + "]");
            log.info("sgx quote version = {}", quoteReportData.getVersion());
            log.info("sgx quote signature type = {}", quoteReportData.getSignType());
            log.info("sgx quote report_data = {}", quoteReportData.getQuoteReportBody().getReportData());
            log.info("sgx quote mr_enclave = {}", quoteReportData.getQuoteReportBody().getMrEnclave());
            log.info("sgx quote mr_signer = {}", quoteReportData.getQuoteReportBody().getMrSigner());
            log.info("Anticipated public key = {}", pubKeyString);

            if (pubKeyString.equals(quoteReportData.getQuoteReportBody().getReportData())) {
                log.info("tls connection success!");
            }
        } else {
            throw new Exception("Failed to fetch isvEnclaveQuoteBody from attestation report");
        }
    }
}
