# Buy/Sale Stocks
This project provide an API GraphQL service to buy/sell stocks and hold stocks and track portafolio performance. We use Kafka to queue message to execute the buy/sell orders independently of the API service.
## Stack of technologies
That are some of main technologies used in the project:
- [Rust](https://www.rust-lang.org/)
- [Async-graphql](https://async-graphql.github.io/async-graphql/en/index.html)
- [Apache Kafka](https://kafka.apache.org/)
- [Docker](https://www.docker.com/)
- [PostgreSQL](https://www.postgresql.org/)
## Prerequisites
To run the project locally you only need Docker. Without Docker, you might need to install the following:
- [Rust](https://www.rust-lang.org/tools/install)
- [Diesel CLI](https://diesel.rs/guides/getting-started.html)
- [CMake](https://cmake.org/install/) 
- [PostgreSQL](https://www.postgresql.org/download/)
- [Apache Kafka](https://kafka.apache.org/quickstart)
- [npm](https://docs.npmjs.com/getting-started)
## Clone the Repositories
In your workspace clone this repositories:
- Consumer
```bash
$ git clone git@github.com:ppzzmm/stocks-consumer.git
```
- Service to buy/sale stocks
```bash
$ git clone git@github.com:ppzzmm/stocks-endpoints.git
```
- GraphQL service to see your portafolio performance
```bash
$ git@github.com:ppzzmm/stocks-services-graphql.git
```
## Run project
### With Docker
In this case we are going to use docker to run the projects, first of all we have to create our zookeeper, kafka and database images, first we need zookeeper because it wil be necessary to raise kafka:

- We are going to create a network to put our containers, step inside on your workspace and run this commands:
```bash
$ docker network create stocks-app
```
- Command to create a **zookeeper** container:
```bash
$ docker run --name=zookeeper -d \
 --network stocks-app \
 --network-alias zookeeper \
 -e ZOOKEEPER_CLIENT_PORT=2181 \
 -e ZOOKEEPER_TICK_TIME=2000 \
 -p 2181:2181 \
 wurstmeister/zookeeper 
```
- Command to create a **kafka** container:
```bash
$ docker run --name=kafka -d \
 --network stocks-app \
 --network-alias kafka \
 -e KAFKA_CREATE_TOPICS="topic-stocks:1:1" \
 -e KAFKA_BROKER_ID=1 \
 -e KAFKA_ZOOKEEPER_CONNECT=zookeeper:2181 \
 -e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://kafka:9092 \
 -e KAFKA_LISTENERS=PLAINTEXT://kafka:9092 \
 -p 9092:9092 \
 wurstmeister/kafka
```
- Command to create our **Postgres DataBase** container:
```bash
$ docker run -d \
 --name stock-db \
 --network stocks-app \
 --network-alias stock-db \
 -p 5432:5432 \
 -e POSTGRES_PASSWORD=password \
 postgres
```
- Step inside the **stocks-consumer** project folder and run this commands, the fisrt command build the project and the next command creates the container:
```bash
$ docker build -t stocks-consumer .
```
```bash
$ docker run \
 --name=stocks-consumer \
 --network stocks-app \
 --network-alias stocks-consumer \
 -dp 8002:8080 \
 -e DATABASE_URL=postgres://postgres:password@stock-db:5432/postgres \
 -e KAFKA_BROKER=kafka:9092 \
 stocks-consumer
```
- Step inside the **stocks-endpoints** project folder and run this commands, the fisrt command build the project and the next command creates the container:
```bash
$ docker build -t stocks-endpoints .
```
```bash
$ docker run \
 --name=stocks-endpoints \
 --network stocks-app \
 --network-alias stocks-endpoints \
 -dp 8080:8080 \
 -e DATABASE_URL=postgres://postgres:password@stock-db:5432/postgres \
 -e KAFKA_BROKER=kafka:9092 \
 stocks-endpoints
```
- Step inside the **stocks-services-graphql** project folder and run this commands, the fisrt command build the project and the next command creates the container:
```bash
$ docker build -t stocks-services-graphql .
```
```bash
$ docker run \
 --name=stocks-services-graphql \
 --network stocks-app \
 --network-alias stocks-services-graphql \
 -dp 8001:8080 \
 -e DATABASE_URL=postgres://postgres:password@stock-db:5432/postgres \
 -e KAFKA_BROKER=kafka:9092 \
 -e SERVER_PORT=8001 \
 stocks-services-graphql
```
### Without Docker
#### Setting Kafka and Zookeeper
- First, download the latest Kafka release [here](https://www.apache.org/dyn/closer.cgi?path=/kafka/3.2.1/kafka_2.13-3.2.1.tgz)
- Extract the compressed file and open it, after that, we have to start the ZooKeeper server with this command:
```bash
$ bin/zookeeper-server-start.sh config/zookeeper.properties
```
- Then, open another terminal session in the decompressed folder and start the Kafka broker:
```bash
$ bin/kafka-server-start.sh config/server.properties
```
- Next we have to create a kafka topic and setting producers and consumers to publish our events.
  - On another terminal exec this command to create a topic:
```bash
$ bin/kafka-topics.sh --create --topic topic-stocks --bootstrap-server localhost:9092
```
- To open the consumer and producer that kafka provided, run this commands:
```bash
$ bin/kafka-console-consumer.sh --topic topic-stocks --from-beginning --bootstrap-server localhost:9092
$ bin/kafka-console-producer.sh --topic topic-stocks --bootstrap-server localhost:9092
```
### Create the Postgres DataBase
- Install [postgres](https://www.postgresql.org/) in your machine, and run in a terminal this:
```bash
$ createdb postgres
```
#### Run the applications and services
- Placed inside the **stocks-consumer** project in a terminal, first we have to run this project because that contain the migrations to create the database tables, here we have the kafka consumer to process the stocks that the user bought or sold:
```bash
$ cargo run
```
- Placed inside the **stocks-services-graphql** project in another terminal, in this project we have the **GrapHQL** endpoints to get the information about the stocks, run the following command:
```bash
$ cargo run
```
- Placed inside the **stocks-endpointsr** project in another terminal, to finish the project launch we have this **Rest API** in rust to buy or sale stocks, this endpoints send an event in the kafka topic to the consumer (**stocks-consumer**) process the information:
```bash
$ cargo run
```
### Run tests
- To run the tests, step inside the **stocks-consumer** project folder and run this command:
```bash
$ cargo test
```
### Testing
- [Here](https://documenter.getpostman.com/view/2220937/2s9YJW55ye#4a2e2bf0-07ee-4066-84a8-db120f3dfb96) you can see how to run the services in postman:
  <img width="1657" alt="Screenshot 2023-09-23 at 23 47 41" src="https://github.com/ppzzmm/rust-pzm-project/assets/29339482/a9b7ab8e-031e-4c8f-9fe3-9c27e7c0b78f">
- If you already used the endpoints to buy or sale stocks, page this command Curl in a terminal to see the information:
```bash
$curl 'http://localhost:8001/stocks' -H 'Accept-Encoding: gzip, deflate, br' -H 'Content-Type: application/json' -H 'Accept: application/json' -H 'Connection: keep-alive' -H 'DNT: 1' -H 'Origin: http://localhost:8001' --data-binary '{"query":"{\n  stocksSummary {\n    symbol\n    profitLoss\n    shares\n    totalValue\n    lowestPrice\n    highestPrice\n    averagePrice\n    priceByHours\n  }\n}"}' --compressed
```
- Or open your browser in this URL [http://localhost:8001/stocks](http://localhost:8001/stocks) and page this query to see the information about your stocks:
```bash
{
  stocksSummary {
    symbol
    profitLoss
    shares
    totalValue
    lowestPrice
    highestPrice
    averagePrice
    priceByHours
  }
}
```

![Screen Recording 2023-09-23 at 23 31 26](https://github.com/ppzzmm/rust-pzm-project/assets/29339482/5fa898b7-4e43-44be-a68f-44c9c0c7a754)

