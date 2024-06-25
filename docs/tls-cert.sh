#!/bin/bash

# Check if the user have OpenSSL in their system
if ! [ -x "$(command -v openssl)" ]; then
  echo 'Error: openssl is not available in your system.' >&2
  exit 1
fi

echo "Generating a Key and CSR"
openssl req \
  -new \
  -newkey rsa:3072 \
  -nodes \
  -keyout localhost.key \
  -out localhost.csr \
  -subj '/CN=127.0.0.1' -extensions EXT -config <( \
  printf "[dn]\nCN=localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")

echo "Creating a self-signed certificate"
openssl x509 \
  -req \
  -days 365 \
  -in localhost.csr \
  -signkey localhost.key \
  -out localhost.crt

echo "Certificates are available on:"
echo $PWA

echo "Next steps are:"
echo "Provide your certificate and key to the HTTP Server as follows"
echo "http-server --tls --tls-cert $PWD/localhost.crt --tls-key $PWD/localhost.key"
echo "Note: Keep in mind that Certificate installation may differ depending on OS"
