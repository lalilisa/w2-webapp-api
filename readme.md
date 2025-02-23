
# Project Name

<span style="font-size: 1.5em;"> 
     English Test Rest API
</span>


# Platform and tool
<span style="font-size: 1.5em;"> 
     Axum Rust, Postgresql, Redis, Kafka, sqlx-template
</span>

# Pre
```aiignore
    Install Kafka, Redis, Postgresql before start Project
    
    You can using my docker-compose.yml file
```

# Environment Variable
<span style="font-size: 1.0em;">
Change value in .env to connect DB, customize port server
</span>

```
DATABASE_USER=
DATABASE_HOST=127.0.0.1
DATABASE_PASS=
DATABASE_PORT=5432
DATABASE_NAME=

SERVER_PORT=8080
RUST_LOG=info,tower_http=debug,kafka=info,sqlx=debug,sql-template=debug

DATABASE_URL=postgres://${DATABASE_USER}:${DATABASE_PASS}@${DATABASE_HOST}:${DATABASE_PORT}/${DATABASE_NAME}

REDIS_URL=redis://127.0.0.1:6378

KAFKA_URLS=kafka:9092
    
```
# Install

```
    git clone 
    
    cd app
    
    crate topic notification before start. I don't know why crate kafka can't create consumer when topic is not exit
    
    if you using docker run cmd (note for image confluentinc/cp-kafka)
    
    docker exec -it kafka kafka-topics --bootstrap-server localhost:9092 --create --topic notification --partitions 1 --replication-factor 1
     
    cargo run
    
````


# Case using  crate Kafka in Windows
````
git clone https://github.com/microsoft/vcpkg.git C:\vcpkg
cd C:\vcpkg
.\bootstrap-vcpkg.bat

.\vcpkg.exe install openssl:x64-windows

$env:OPENSSL_DIR = "C:\path\to\vcpkg\installed\x64-windows"
$env:OPENSSL_LIB_DIR = "$env:OPENSSL_DIR\lib"
$env:OPENSSL_INCLUDE_DIR = "$env:OPENSSL_DIR\include"

.\vcpkg.exe integrate install

After set path
$env:Path += ";$env:OPENSSL_DIR"
$env:Path += ";$env:OPENSSL_LIB_DIR"
$env:Path += ";$env:OPENSSL_INCLUDE_DIR"

Certainly, restart your machine after set

````

# Usage

```aiignore
    Import file Englisg.postman_collection.json to postman 
```
