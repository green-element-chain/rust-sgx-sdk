package com.energy.sgx.socket.service.impl;

import com.energy.sgx.socket.dto.ServerCertInfo;
import com.energy.sgx.socket.dto.ServerSgxProperties;
import com.energy.sgx.socket.utils.SocketUtil;
import com.energy.sgx.socket.utils.PemReader;
import com.energy.sgx.socket.utils.VerifyMarshalCert;
import java.io.InputStream;
import java.security.KeyStore;
import java.security.PrivateKey;
import java.security.cert.Certificate;
import java.security.cert.CertificateException;
import java.security.cert.X509Certificate;
import java.util.ArrayList;
import java.util.List;
import javax.net.ssl.KeyManagerFactory;
import javax.net.ssl.SSLContext;
import javax.net.ssl.TrustManager;
import javax.net.ssl.X509TrustManager;
import lombok.Getter;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.util.ObjectUtils;
import org.springframework.util.StringUtils;

/**
 * @author Bryan
 * @date 2019-08-09
 */
@Slf4j
@Getter
public class SSLContextBase {

    private PemReader pemReader = new PemReader();
    private VerifyMarshalCert marshalCert = new VerifyMarshalCert();

    @Autowired
    private ServerSgxProperties properties;

    SSLContext createSSLContext() throws Exception {
        TrustManager trustManager = new X509TrustManager() {
            @Override
            public void checkClientTrusted(X509Certificate[] certs, String authType) throws CertificateException {
            }

            @Override
            public void checkServerTrusted(X509Certificate[] certs, String authType) throws CertificateException {
                if (properties.getCert().getServerTrusted()) {
                    String message = null;
                    try {
                        // CommonUtil.printCert(certs[0].getEncoded());
                        List<Byte> byteArray = new ArrayList<>();
                        for (int i = 0; i < certs[0].getEncoded().length; i++) {
                            byteArray.add(certs[0].getEncoded()[i]);
                        }

                        // get the public key and payload from raw data
                        ServerCertInfo certData = marshalCert.unMarshalByte(byteArray);

                        // load Intel CA, then verify cert and signature
                        ServerSgxProperties.SgxCertInfo sgxCertInfo = properties.getCert();
                        InputStream is = sgxCertInfo.getInputStream(sgxCertInfo.getCaFile());
                        byte[] attnReportRaw = marshalCert.verifyCert(is, certData.payload);
                        if (!ObjectUtils.isEmpty(attnReportRaw)) {
                            marshalCert.verifyAttnReport(attnReportRaw, certData.pubKey);

                            String outputPath = properties.getCert().getOutput();
                            SocketUtil.writeInFileByfb(SocketUtil.bytesToHex(certData.pubKey), outputPath + "/pubkey.txt");
                            SocketUtil.writeInFileByfb(SocketUtil.bytesToHex(attnReportRaw), outputPath + "/report.txt");
                        } else {
                            message = "attn report raw is empty.";
                        }
                    } catch (Exception e) {
                        message = e.toString();
                    }

                    if (!StringUtils.isEmpty(message)) {
                        log.error(message);
                        throw new CertificateException(message);
                    }
                }
            }

            @Override
            public X509Certificate[] getAcceptedIssuers() {
                return new X509Certificate[0];
            }
        };

        SSLContext sc = SSLContext.getInstance("SSL");
        KeyManagerFactory keyManagers = getKeyManagerFactory();
        sc.init(keyManagers.getKeyManagers(), new TrustManager[]{trustManager}, null);
        return sc;
    }

    private KeyManagerFactory getKeyManagerFactory() throws Exception {
        ServerSgxProperties.SgxCertInfo sgxCertInfo = properties.getCert();
        InputStream certInfoInputStream = sgxCertInfo.getInputStream(sgxCertInfo.getCertificate());
        List<X509Certificate> certificates = pemReader.readCertificate(certInfoInputStream);

        InputStream privateKeyInputStream = sgxCertInfo.getInputStream(sgxCertInfo.getPrivateKey());
        PrivateKey key = pemReader.getPemPrivateKey(privateKeyInputStream, sgxCertInfo.getAlgorithm());

        KeyStore keyStore = KeyStore.getInstance("JKS");
        keyStore.load(null, null);
        keyStore.setKeyEntry("key", key, "".toCharArray(), certificates.stream().toArray(Certificate[]::new));

        KeyManagerFactory factory = KeyManagerFactory.getInstance("SunX509");
        factory.init(keyStore, "".toCharArray());

        return factory;
    }
}
