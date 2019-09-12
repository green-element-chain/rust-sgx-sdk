#!/bin/bash

openssl pkcs12 -in nlkf_payment.pfx -out sign.crt -nodes
