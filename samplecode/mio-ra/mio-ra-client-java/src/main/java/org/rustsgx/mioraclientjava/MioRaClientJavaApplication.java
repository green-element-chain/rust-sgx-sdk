package org.rustsgx.mioraclientjava;

import com.google.gson.FieldNamingPolicy;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import org.rustsgx.mioraclientjava.bean.Teacher;
import org.rustsgx.mioraclientjava.raverify.SgxCertVerifier;
import org.springframework.boot.autoconfigure.SpringBootApplication;

import javax.net.ssl.*;
import java.io.*;
import java.net.Socket;
import java.security.*;
import java.util.Random;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

@SpringBootApplication
public class MioRaClientJavaApplication {

    public static void main(String[] args) throws Exception{
        int threadCount = 50;
        ExecutorService service = Executors.newFixedThreadPool(threadCount);
        for (int i = 0; i < threadCount; i++) {
            service.execute(() -> {
                try {
                    verifyMioServer();
                } catch (Exception ex) {
                    ex.printStackTrace();
                }
            });
        }
        service.shutdown();
        service.awaitTermination(10000000, TimeUnit.SECONDS);
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

            int max=100,min=1;
            int ran2 = (int) (Math.random()*(max-min)+min);
            int clientID = (int)(ran2*1);
            System.out.printf("clientId is %d\n",clientID);

            int status = sendData(in,out,clientID);
            if (status == -1){
            }

            System.out.println("end senddata");

        }catch (Exception e){
            System.out.println(e.fillInStackTrace());
        }
    }

    public static int sendData(BufferedReader in,OutputStream out, int clientID){
        try{
            GsonBuilder gsonBuilder = new GsonBuilder();
            gsonBuilder.setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES);
            Gson gson = gsonBuilder.create();
            for (int i=0;i<10;i++){
                Teacher request = new Teacher();
                if(i==9){
                    request.setAge(i);
                    request.setCity("City"+Integer.toString(i));
                    request.setStreet("Street"+Integer.toString(i));
                    request.setSendStatus("end");
                    request.setClientId(clientID);
                    out.write(gson.toJson(request).getBytes());
                }else{
                    request.setAge(i);
                    request.setCity("City"+Integer.toString(i));
                    request.setStreet("Street"+Integer.toString(i));
                    request.setSendStatus("not end");
                    request.setClientId(clientID);
                    out.write(gson.toJson(request).getBytes());
                }
                System.out.println(gson.toJson(request));
                //every write need wait data, if not it will make parsing error of json in sgx
                int getStatus = getReturnData(in,i,clientID);
                System.out.println(getStatus);

            }
            return 0;
        }catch (Exception e){
            System.out.println(e.toString());
            return -1;
        }

    }

    public static int getReturnData(BufferedReader in,int i, int clientID){
        try{
            String rsp = in.readLine();
            System.out.println(rsp);
            if(rsp.equals("success")){
                System.out.printf("the %d: %d data success\n",clientID,i);
            }else{
                return -1;
            }
        }catch (Exception e){
            System.out.println(e.fillInStackTrace());
            return -1;
        }
        return 0;
    }
}
