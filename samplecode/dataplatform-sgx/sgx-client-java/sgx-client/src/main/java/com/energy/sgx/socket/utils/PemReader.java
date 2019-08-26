package com.energy.sgx.socket.utils;

import static java.util.regex.Pattern.CASE_INSENSITIVE;

import java.io.ByteArrayInputStream;
import java.io.DataInputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.Reader;
import java.nio.CharBuffer;
import java.nio.charset.StandardCharsets;
import java.security.GeneralSecurityException;
import java.security.KeyFactory;
import java.security.PrivateKey;
import java.security.cert.CertificateFactory;
import java.security.cert.X509Certificate;
import java.security.spec.PKCS8EncodedKeySpec;
import java.util.ArrayList;
import java.util.Base64;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * @author Bryan
 * @date 2019-07-17
 */
public class PemReader {

    private static final Pattern CERT_PATTERN = Pattern.compile(
        new StringBuilder()
            // Header
            .append("-+BEGIN\\s+.*CERTIFICATE[^-]*-+(?:\\s|\\r|\\n)+")
            // Base64 text
            .append("([a-z0-9+/=\\r\\n]+)")
            // Footer
            .append("-+END\\s+.*CERTIFICATE[^-]*-+")
            .toString(),
        CASE_INSENSITIVE);

    private static final Pattern PEM_PATTERN = Pattern.compile(
        "-+BEGIN PRIVATE KEY-+\n([a-z0-9+/=\\r\\n]+)\n-+END PRIVATE KEY-+");

    public static List<X509Certificate> readCertificate(File certificateFile)
        throws IOException, GeneralSecurityException {
        String contents = readFile(certificateFile);
        Matcher matcher = CERT_PATTERN.matcher(contents);

        CertificateFactory certificateFactory = CertificateFactory.getInstance("X.509");
        List<X509Certificate> certificates = new ArrayList<>();

        int start = 0;
        while (matcher.find(start)) {
            byte[] buffer = base64Decode(matcher.group(1));
            certificates.add((X509Certificate) certificateFactory.generateCertificate(new ByteArrayInputStream(buffer)));
            start = matcher.end();
        }
        return certificates;
    }

    private static String readFile(File file) throws IOException {
        Reader reader = null;
        try {
            reader = new InputStreamReader(new FileInputStream(file), StandardCharsets.US_ASCII);
            StringBuilder stringBuilder = new StringBuilder();
            CharBuffer buffer = CharBuffer.allocate(2048);
            while (reader.read(buffer) != -1) {
                buffer.flip();
                stringBuilder.append(buffer);
                buffer.clear();
            }
            return stringBuilder.toString();

        } finally {
            if (reader != null) {
                reader.close();
            }
        }
    }

    private static byte[] base64Decode(String base64) {
        return Base64.getMimeDecoder().decode(base64.getBytes(StandardCharsets.US_ASCII));
    }

    public static PrivateKey getPemPrivateKey(String filename, String algorithm) throws Exception {
        DataInputStream dataInputStream = null;
        try {
            File file = new File(filename);
            dataInputStream = new DataInputStream(new FileInputStream(file));
            byte[] keyBytes = new byte[(int) file.length()];
            dataInputStream.readFully(keyBytes);

            String privateKeyPem = new String(keyBytes)
                .replaceAll("-+BEGIN PRIVATE KEY-+\n", "")
                .replaceAll("\n-+END PRIVATE KEY-+", "")
                .trim();

            byte[] decoded = base64Decode(privateKeyPem);
            PKCS8EncodedKeySpec keySpec = new PKCS8EncodedKeySpec(decoded);
            KeyFactory keyFactory = KeyFactory.getInstance(algorithm);
            return keyFactory.generatePrivate(keySpec);
        } finally {
            if (dataInputStream != null) {
                dataInputStream.close();
            }
        }
    }
}
