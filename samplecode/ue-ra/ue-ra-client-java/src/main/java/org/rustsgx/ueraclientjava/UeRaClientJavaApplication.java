package org.rustsgx.ueraclientjava;

import com.google.gson.Gson;
import org.rustsgx.ueraclientjava.bean.ComputeResult;
import org.rustsgx.ueraclientjava.bean.SGXReport;
import org.springframework.boot.autoconfigure.SpringBootApplication;

import javax.net.ssl.*;
import java.io.*;
import java.net.Socket;
import java.security.*;

@SpringBootApplication
public class UeRaClientJavaApplication {

    public static void main(String[] args) {
        Gson gson = new Gson();

        System.out.println("Starting ue-ra-client-java");

        try {
            SSLContext sc = SSLContext.getInstance("SSL");
            SgxCertVerifier sgxCertVerifier = new SgxCertVerifier();
            sc.init(sgxCertVerifier.keyManagerFactory.getKeyManagers(), sgxCertVerifier.trustAllCerts, new SecureRandom());

            SSLSocketFactory sf = sc.getSocketFactory();

            System.out.println("Connecting to  localhost:3443");
            Socket s = sf.createSocket("127.0.0.1", 3443);

            DataOutputStream out = new DataOutputStream(s.getOutputStream());
            String str = "hello ue-ra-java-client";
            ComputeResult request = new ComputeResult();
            request.setAge(18);
            request.setCity("BeiJing");
            request.setStreet("dongmeng");

            out.write(gson.toJson(request).getBytes());

            BufferedReader in = new BufferedReader(new InputStreamReader(s.getInputStream()));
            String x = in.readLine();
            System.out.printf("server replied:  %s\n", x);

            String y = in.readLine();
            System.out.printf("server replied:  %s\n", y);

            //get the pubkey that we saved to local
            String pubkey = CommonUtils.readFileReturnFirstLine("pubkey.txt");
            System.out.println(pubkey);

            //unmarshal the result data that sgx send to us
            ComputeResult result = gson.fromJson(x,ComputeResult.class);
            System.out.println(result.getStreet());

            SGXReport sgxRep = new SGXReport();
            sgxRep.setHmacString(y);
            sgxRep.setPubkey(pubkey);

            String sgxJson = gson.toJson(sgxRep);
            System.out.println(sgxJson);

            //recompute genHMAC and verify it
            String genHMAC = HMAC_SHA1.genHMAC(x, pubkey);
            if (genHMAC.equals(sgxRep.getHmacString())){
                System.out.println("successed to verify hmac");
            }else{
                System.out.println("failed to verify hmac");
            }

            out.close();
            in.close();
        } catch (Exception e) {
            System.out.println(e.toString());
            System.exit(0);
        }
    }

}
