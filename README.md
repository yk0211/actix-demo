# stellar

[Windows]
1. Add in .cargo/config
[target.x86_64-pc-windows-msvc]
rustflags = ["-Ctarget-feature=+crt-static"]

2. Install MySQL Connector/C
(1) download and install: https://downloads.mysql.com/archives/c-c/
(2) Set variables in environment variables
MYSQLCLIENT_LIB_DIR
C:\Program Files\MySQL\MySQL Connector C 6.1\lib\vs14

3. Install OpenSSL
(1) download and install: http://slproweb.com/products/Win32OpenSSL.html
(2) Set variables in environment variables
OPENSSL_DIR
C:\Program Files (x86)\OpenSSL-Win64