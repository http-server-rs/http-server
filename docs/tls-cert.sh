#!bin/sh

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
  -keyout localhost.pem \
  -out localhost.csr \
  -subj '/CN=localhost' -extensions EXT -config <( \
  printf "[dn]\nCN=localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")

echo "Creating a self-signed certificate"
openssl x509 \
  -req \
  -days 365 \
  -in localhost.csr \
  -signkey localhost.key \
  -out localhost.crt
