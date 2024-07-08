# Microservice: Contract Service

## Requisitos Funcionais

### RF01 - Cadastrar Cliente

O primeiro caso de uso seria o start da arquitetura do microservice. Onde várias
decisões de design e arquitetura serão tomadas.

**Fase 1**
- [x] Criar migrations para a tabela de customers
- [x] Criar model de customer
- [x] Criar adapter repository postgres para customer
- [x] Criar caso de uso para cadastrar customer
- [x] Criar estrutura do evento de cadastro de customer.created
- [x] Criar service para publicar o evento e persistir o customer

**Fase 2**
- [x] Criar arquivo de proto para customer
- [x] Criar service gRPC com reflection service
- [x] Criar controller para cadastro de customer
- [ ] Criar validacao de dados no request do controller

**Fase 3**
- [ ] Criar docker-compose para rodar o kafka
- [ ] Criar producer para publicar eventos no kafka
- [ ] Criar consumer para ouvir eventos no kafka

**Fase 4**
- [ ] Criar testes unitários
- [ ] Criar testes de integração
