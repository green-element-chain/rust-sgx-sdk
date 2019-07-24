使用方法：
1. 设置RUST_SGX_SDK环境变量，指定SDK的绝对路径
```asm
    $ export RUST_SGX_SDK="../../../rust-sgx-sdk"
```
2. 执行dev_script/sgx_docker.sh进入容器，在docker容器中进行编译
3. 退出容器，在sgx-server/bin目录下面执行app，启动sgx应用
```asm
    $ cd sgx-server
    $ ./app
```