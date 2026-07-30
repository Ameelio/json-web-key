#!/bin/sh
#
#

set -e

SCRATCHD=$(mktemp -d 2>/dev/null || mktemp -d -t '/tmp/scratchd')

openssl genrsa -out "${SCRATCHD}/rsa.pem"
openssl rsa -in "${SCRATCHD}/rsa.pem" -noout -modulus | sed 's/^Modulus=//' | xxd -pr | base64 > "${SCRATCHD}/mod.bin"
openssl rsa -in "${SCRATCHD}/rsa.pem" -noout -text | grep 'publicExponent' | awk '{print $2}' | xxd -pr | base64 > "${SCRATCHD}/exp.bin"

SCRATCHD=$SCRATCHD yq e -n -o json '
.alg = "RSA256" |
  .kid = "test" |
  .kty = "RSA" |
  .e = load_str(strenv(SCRATCHD) + "/exp.bin") |
  .e |= sub("\\n"; "") |
  .n = load_str(strenv(SCRATCHD) + "/mod.bin") |
  .n |= sub("\\n"; "") |
  .use = "sign"
'

