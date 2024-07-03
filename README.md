# Microservice: Contract Management Service

**Projeto para estudo de Rust, gRPC e Kafka**

O ContractManagementService é um microserviço dedicado à gestão de contratos dentro de um sistema
ERP SaaS, permitindo a administração eficiente de contratos de clientes, incluindo detalhes de
planos, funcionalidades contratadas, termos de uso assinados, e conformidade com regulamentações
como o LGPD. Este serviço integra funcionalidades para a criação, atualização e monitoramento
de contratos, associando-os a clientes.

## Estrutura do Projeto

```bash
src
├── db.rs
├── env.rs
├── gprc
│   ├── customer_grpc.rs
├── internal
│   ├── adapters
│   │   ├── customer_repository_postgres.rs
│   │   └── mod.rs
│   ├── application
│   │   ├── customer_service.rs
│   │   └── mod.rs
│   └── domain
│       ├── event
│       │   ├── customer
│       │   │   └── customer_created_event.rs
│       │   └── mod.rs
│       ├── model
│       │   ├── customer.rs
│       │   └── mod.rs
│       └── use_case
│           ├── create_customer.rs
│           └── mod.rs
└── main.rs
proto
└── customer.proto
```
