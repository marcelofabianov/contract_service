# Microservice: Contract Service

[Home](../README.md)

## Arquitetura

Neste documento será descrito a arquitetura do microservice de contrato.

### Tecnologias

- **Linguagem**: Rust
- **Banco de Dados**: Postgres
- **Mensageria**: Kafka
- **Protocolo de Comunicação**: gRPC
- **CI/CD**: Github Actions
- **Monitoramento**: Open Telemetry, Prometheus, Grafana, Elastic Stack, Kibana
- **Testes**: Unitários, Integração, AB
- **Documentação**: Swagger, Open API

### Estrutura de Diretórios

A estrutura de diretórios, arquivos e módulos do projeto segue o seguinte padrão:

```bash
src
|-- container.rs
|-- db.rs
|-- env.rs
|-- error.rs
|-- errors
|   |-- env.rs
|   |-- kafka.rs
|   |-- mod.rs
|   `-- postgres.rs
|-- grpc
|   |-- controller
|   |   |-- customer_controller.rs
|   |   `-- mod.rs
|   |-- mapper
|   |   |-- customer_mapper.rs
|   |   `-- mod.rs
|   |-- mod.rs
|   `-- pb
|       |-- customer_pb.rs
|       |-- descriptor.bin
|       `-- mod.rs
|-- internal
|   |-- adapters
|   |   |-- customer_repository_postgres.rs
|   |   `-- mod.rs
|   |-- application
|   |   |-- customer_service.rs
|   |   `-- mod.rs
|   |-- domain
|   |   |-- event
|   |   |   |-- customer
|   |   |   `-- mod.rs
|   |   |-- model
|   |   |   |-- customer.rs
|   |   |   `-- mod.rs
|   |   `-- use_case
|   |       |-- create_customer.rs
|   |       `-- mod.rs
|   `-- mod.rs
|-- kafka
|   |-- mod.rs
|   `-- producer.rs
|-- main.rs
`-- proto
    `-- customer.proto
```

### Arquitetura de Software

A arquitetura de software do microservice de contrato é baseada no padrão de arquitetura hexagonal, onde a aplicação é dividida em camadas de acordo com suas responsabilidades.

#### Camada de Aplicação

A camada de aplicação é responsável por orquestrar os recursos da aplicação, como a comunicação entre as camadas de domínio e infraestrutura. Nesta camada, são utilizado os casos de uso para implementar as regras de negócio da aplicação.

#### Camada de Domínio

A camada de domínio é responsável por representar o domínio da aplicação, ou seja, as entidades e objetos de valor que representam o negócio da aplicação. Nesta camada, são definidos os modelos de dados, eventos de domínio e casos de uso.

#### Camada de Infraestrutura

A camada de infraestrutura é responsável por prover os recursos necessários para a aplicação, como o acesso a banco de dados, mensageria, protocolos de comunicação, entre outros. Nesta camada, são implementados os adaptadores para integração com os recursos externos.

### Arquitetura de Dados

A arquitetura de dados do microservice é baseada em um banco de dados relacional, onde são armazenados os dados dos contratos. O banco de dados utilizado é o Postgres, que é um banco de dados relacional de código aberto.

### Comunicação

A comunicação do microservice é baseada no protocolo de comunicação gRPC, que é um protocolo de comunicação de alto desempenho e eficiente para comunicação entre serviços. Neste microservice, são utilizados os serviços gRPC para comunicação entre os diferentes componentes da aplicação utilizando protocol buffers.

### Eventos

Os eventos do microservice é baseada em eventos de domínio, que são eventos que representam mudanças no estado do domínio da aplicação. Neste microservice, são utilizados eventos de domínio para representar a criação, atualização e exclusão. Os eventos de domínio são publicados em um tópico do Kafka.

### Monitoramento

O monitoramento do microservice é baseado em ferramentas de monitoramento de código aberto, como o Open Telemetry, Prometheus, Grafana, Elastic Stack e Kibana. Estas ferramentas são utilizadas para monitorar o desempenho, disponibilidade e confiabilidade.

### Testes

Os testes do microservice são baseados em testes unitários, testes de integração e testes AB. Os testes unitários são utilizados para testar as unidades de código da aplicação, os testes de integração são utilizados para testar a integração entre os diferentes componentes da aplicação e os testes AB são utilizados para testar o desempenho da aplicação.

### Documentação

A documentação do microservice é baseada em Swagger e Open API, que são ferramentas de documentação de código aberto para documentar a API da aplicação. A documentação é utilizada para descrever os endpoints da API, os parâmetros de entrada e saída, os códigos de status, entre outros.

### CI/CD

O CI/CD do microservice é baseado no Github Actions, que é uma ferramenta de integração contínua e entrega contínua de código aberto. O Github Actions é utilizado para automatizar o processo de build, teste e deploy da aplicação.
