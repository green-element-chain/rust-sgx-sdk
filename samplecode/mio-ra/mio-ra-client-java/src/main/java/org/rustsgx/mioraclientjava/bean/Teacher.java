package org.rustsgx.mioraclientjava.bean;

public class Teacher {
    private String street;
    private String city;
    private int age;
    private String sendStatus;
    private int clientId;

    public String getStreet() {
        return street;
    }

    public void setStreet(String street) {
        this.street = street;
    }

    public String getCity() {
        return city;
    }

    public int getClientId() {
        return clientId;
    }

    public void setClientId(int clientId) {
        this.clientId = clientId;
    }

    public void setCity(String city) {
        this.city = city;
    }

    public int getAge() {
        return age;
    }

    public void setAge(int age) {
        this.age = age;
    }

    public String getSendStatus() {
        return sendStatus;
    }

    public void setSendStatus(String sendStatus) {
        this.sendStatus = sendStatus;
    }

    public void constructTeacher(int i,String sendStatus,int clientId){
        this.setAge(i);
        this.setCity("City"+Integer.toString(i));
        this.setStreet("Street"+Integer.toString(i));
        this.setSendStatus(sendStatus);
        this.setClientId(clientId);
    }

}
