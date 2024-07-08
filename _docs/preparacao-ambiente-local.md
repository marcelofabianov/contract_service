# Microservice: Contract Service

## Preparação do ambiente local

Neste documento, você encontrará informações sobre como preparar o ambiente local para executar o microservice `Contract Service`.

### Ferramentas necessárias

Relacionamos abaixo as ferramentas devem estar instaladas em sua máquina para executar o microservice `Contract Service`:

- [Git](https://git-scm.com/downloads)
- [Docker](https://docs.docker.com/get-docker/)

**Executar requisições gRPC e testes de carga**

- [httpd](https://httpd.apache.org/download.cgi)
- [ghz](https://ghz.sh/docs/install)
- [grpcurl](https://github.com/fullstorydev/grpcurl)

### Clonar o repositório

Para clonar o repositório do microservice `Contract Service`, execute o comando abaixo:

```bash
git clone ...
```

Configure o environment do projeto.

```bash
cp .env.example .env
```

### Executar o microservice

Para executar o microservice `Contract Service`, execute o comando abaixo:

```bash
docker compose up -d
```

Aguarde alguns segundos até que o microservice e as dependências sejam inicializadas.

### Verificar se o microservice está em execução

Para verificar se o microservice `Contract Service` está em execução, execute o comando abaixo:

```bash
docker ps
```

Instale as dependências do projeto e realize o build do projeto.

```bash
docker exec -it contract_service bash
```

```bash
cargo build
```

### Executando migrações

Para executar as migrações, execute o comando abaixo:

```bash
cargo sqlx migrate run
```

### Executando o microservice

Para executar o microservice, execute o comando abaixo:

```bash
cargo run
```

### Executando testes

Para executar os testes, execute o comando abaixo:

```bash
cargo test
```

### Executando requisições gRPC

Para executar requisições gRPC, execute o comando abaixo:

```bash
grpcurl -plaintext localhost:50051 list
```

Mais informações sobre como executar requisições gRPC podem ser encontradas na [documentação gRPC](./grpc.md), ou no diretorio da raiz **"_req"**.
