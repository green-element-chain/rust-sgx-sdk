package org.rustsgx.mioraclientjava;

import com.google.gson.Gson;
import org.rustsgx.mioraclientjava.bean.Person;
import org.rustsgx.mioraclientjava.raverify.SgxCertVerifier;
import org.springframework.boot.autoconfigure.SpringBootApplication;

import javax.net.ssl.*;
import java.io.*;
import java.net.Socket;
import java.security.*;

@SpringBootApplication
public class MioRaClientJavaApplication {

    public static void main(String[] args) {
        verifyMioServer();
    }

    public static void verifyMioServer(){
        Gson gson = new Gson();
        System.out.println("Starting mio-client-java");

        try {
            SSLContext sc = SSLContext.getInstance("TLS");
            SgxCertVerifier sgxCertVerifier = new SgxCertVerifier();
            sc.init(sgxCertVerifier.keyManagerFactory.getKeyManagers(), sgxCertVerifier.trustAllCerts, new SecureRandom());

            SSLSocketFactory sf = sc.getSocketFactory();

            Socket s = sf.createSocket("127.0.0.1", 8443);

            DataOutputStream out = new DataOutputStream(s.getOutputStream());
            BufferedReader in = new BufferedReader(new InputStreamReader(s.getInputStream()));

            System.out.println("before senddata");

            int status = sendData(in,out);
            if (status == -1){
                System.exit(0);
            }

            System.out.println("end senddata");

            String senddata = in.readLine();
            System.out.printf("server replied:  %s\n", senddata);

        }catch (Exception e){
            System.out.println(e.fillInStackTrace());
        }
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
                    System.out.printf("the %d data success\n",i);
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
}