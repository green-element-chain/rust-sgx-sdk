package org.rustsgx.mioraclientjava;

import com.google.gson.Gson;
import org.rustsgx.mioraclientjava.bean.ComputeResult;
import org.rustsgx.mioraclientjava.bean.Person;
import org.rustsgx.mioraclientjava.bean.SGXReport;
import org.rustsgx.mioraclientjava.raverify.CommonUtils;
import org.rustsgx.mioraclientjava.raverify.HMAC_SHA1;
import org.rustsgx.mioraclientjava.raverify.SgxCertVerifier;
import org.springframework.boot.autoconfigure.SpringBootApplication;

import javax.net.ssl.*;
import java.io.*;
import java.net.Socket;
import java.security.*;

@SpringBootApplication
public class MioRaClientJavaApplication {

    public static void main(String[] args) {
        verifyQuoteReport();
    }

    public static int sendData(BufferedReader in,OutputStream out){
        try{
            Gson gson = new Gson();
            for (int i=0;i<10;i++){
                Person request = new Person();
                if(i==9){
                    request.setAge(i);
                    request.setCity("City"+Integer.toString(i));
                    request.setStreet("Street"+Integer.toString(i));
                    request.setSendStatus("end");
                    out.write(gson.toJson(request).getBytes());
                }else{
                    request.setAge(i);
                    request.setCity("City"+Integer.toString(i));
                    request.setStreet("Street"+Integer.toString(i));
                    request.setSendStatus("not end");
                    out.write(gson.toJson(request).getBytes());
                }

                String rsp = in.readLine();
                if(rsp.equals("success")){
                }else{
                    return -1;
                }
            }
            return 0;
        }catch (Exception e){
            System.out.println(e.toString());
            return -1;
        }

    }

    public static void verifyQuoteReport(){
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
            BufferedReader in = new BufferedReader(new InputStreamReader(s.getInputStream()));

            int status = sendData(in,out);
            if (status == -1){
                System.exit(0);
            }

            String compute_json = in.readLine();
            System.out.printf("server replied:  %s\n", compute_json);

            String hash = in.readLine();
            System.out.printf("server replied:  %s\n", hash);

            //get the pubkey that we saved to local
            String pubkey = CommonUtils.readFileReturnFirstLine("pubkey.txt");
            String report = CommonUtils.readFileReturnFirstLine("report.txt");
            System.out.println(pubkey);

            //unmarshal the result data that sgx send to us
            ComputeResult result = gson.fromJson(compute_json, ComputeResult.class);

            SGXReport sgxRep = new SGXReport();
            sgxRep.setHmacString(hash);
            sgxRep.setPubkey(pubkey);
            sgxRep.setReport(report);

            String sgxJson = gson.toJson(sgxRep);
            System.out.println(sgxJson);

            //recompute genHMAC and verify it
            String genHMAC = HMAC_SHA1.genHMAC(compute_json, pubkey);
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


