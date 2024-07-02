# Microservice: Contract Management Service

O ContractManagementService é um microserviço dedicado à gestão de contratos dentro de um sistema
ERP SaaS, permitindo a administração eficiente de contratos de clientes, incluindo detalhes de
planos, funcionalidades contratadas, termos de uso assinados, e conformidade com regulamentações
como o LGPD. Este serviço integra funcionalidades para a criação, atualização e monitoramento
de contratos, associando-os a clientes.

## Entidades

**Customer**

Entidade que representa o cliente contratante do serviço.

- trace_id: `uuid` `required` `unique` | Identificador único de trace transacional.
- id: `number` `required` `unique` | Identificador único do cliente interno.
- public_id: `uuid` `required` `unique` | Identificador público do cliente uso externo.
- document: `string` `required`| Documento de identificação do cliente CNPJ.
- name: `string` `required`| Nome do cliente.
- disabled_at: `datetime` `optional`| Data de desativação do cliente.
- created_at: `datetime` `required`| Data de criação do cliente.
- updated_at: `datetime` `required`| Data de atualização do cliente.
- deleted_at: `datetime` `optional`| Data de exclusão do cliente.

**Ownership**

Entidade que representa o proprietário legal do dados do cliente perante LGPD.

- trace_id: `uuid` `required` `unique` | Identificador único de trace transacional.
- id: `number` `required` `unique` | Identificador único do proprietário interno.
- public_id: `uuid` `required` `unique` | Identificador público do proprietário uso externo.
- customer_id: `uuid` `required`| Identificador do cliente.
- disabled_at: `datetime` `optional`| Data de desativação do cliente.
- created_at: `datetime` `required`| Data de criação do cliente.
- updated_at: `datetime` `required`| Data de atualização do cliente.
- deleted_at: `datetime` `optional`| Data de exclusão do cliente.

**Contact**

Entidade que representa o contato do cliente.

- ownership_id: `uuid` `required` `unique` | Identificador único do proprietário do dado LGPD.
- trace_id: `uuid` `required` `unique` | Identificador único de trace transacional.
- id: `number` `required` `unique` | Identificador único do contato interno.
- public_id: `uuid` `required` `unique` | Identificador público do contato uso externo.
- customer_id: `uuid` `required`| Identificador do cliente.
- description: `string` `required`| Nome do contato.
- value: `string` `required`| Valor do contato.
- contact_type: `string` `required`| Tipo do contato.
- disabled_at: `datetime` `optional`| Data de desativação do contato.
- created_at: `datetime` `required`| Data de criação do contato.
- updated_at: `datetime` `required`| Data de atualização do contato.
- deleted_at: `datetime` `optional`| Data de exclusão do contato.

**Person**

Entidade que representa uma pessoa física responsável pelo cliente.

- ownership_id: `uuid` `required` `unique` | Identificador único do proprietário do dado LGPD.
- trace_id: `uuid` `required` `unique` | Identificador único de trace transacional.
- id: `number` `required` `unique` | Identificador único da pessoa interno.
- public_id: `uuid` `required` `unique` | Identificador público da pessoa uso externo.
- customer_id: `uuid` `required`| Identificador do cliente.
- full_name: `string` `required`| Nome completo da pessoa.
- document: `string` `required`| Documento de identificação da pessoa CPF.
- email: `string` `required`| E-mail de contato da pessoa.
- phone: `string` `optional`| Telefone de contato da pessoa.
- disabled_at: `datetime` `optional`| Data de desativação da pessoa.
- created_at: `datetime` `required`| Data de criação da pessoa.
- updated_at: `datetime` `required`| Data de atualização da pessoa.
- deleted_at: `datetime` `optional`| Data de exclusão da pessoa.

**Term**

Entidade que representa o termo de uso.

- trace_id: `uuid` `required` `unique` | Identificador único de trace transacional.
- id: `number` `required` `unique` | Identificador único do termo interno.
- public_id: `uuid` `required` `unique` | Identificador público do termo uso externo.
- title: `string` `required`| Título do termo.
- external_link: `string` `required`| Link externo do termo.
- version: `string` `required`| Versão do termo.
- disabled_at: `datetime` `optional`| Data de desativação do termo.
- created_at: `datetime` `required`| Data de criação do termo.
- updated_at: `datetime` `required`| Data de atualização do termo.
- deleted_at: `datetime` `optional`| Data de exclusão do termo.

**Feature**

Entidade que representa a funcionalidade compoem o produto.

- trace_id: `uuid` `required` `unique` | Identificador único de trace transacional.
- id: `number` `required` `unique` | Identificador único da funcionalidade interno.
- public_id: `uuid` `required` `unique` | Identificador público da funcionalidade uso externo.
- label: `string` `required` `unique` | Rótulo da funcionalidade.
- name: `string` `required`| Nome da funcionalidade.
- description: `string` `required`| Descrição da funcionalidade.
- disabled_at: `datetime` `optional`| Data de desativação da funcionalidade.
- created_at: `datetime` `required`| Data de criação da funcionalidade.
- updated_at: `datetime` `required`| Data de atualização da funcionalidade.
- deleted_at: `datetime` `optional`| Data de exclusão da funcionalidade.

**Plan**

Entidade que representa o plano com os serviços contratados pelo cliente.

- trace_id: `uuid` `required` `unique` | Identificador único de trace transacional.
- id: `number` `required` `unique` | Identificador único do plano interno.
- public_id: `uuid` `required` `unique` | Identificador público do plano uso externo.
- label: `string` `required` `unique` | Rótulo do plano.
- name: `string` `required`| Nome do plano.
- description: `string` `required`| Descrição do plano.
- price: `float` `required`| Preço do plano.
- features: `json` `required`| Lista de funcionalidades do plano.
- disabled_at: `datetime` `optional`| Data de desativação do plano.
- created_at: `datetime` `required`| Data de criação do plano.
- updated_at: `datetime` `required`| Data de atualização do plano.
- deleted_at: `datetime` `optional`| Data de exclusão do plano.

**Contract**

Entidade que representa o contrato de serviço do cliente com a empresa.

- ownership_id: `uuid` `required` `unique` | Identificador único do proprietário do dado LGPD.
- trace_id: `uuid` `required` `unique` | Identificador único de trace transacional.
- id: `number` `required` `unique` | Identificador único do contrato interno.
- public_id: `uuid` `required` `unique` | Identificador público do contrato uso externo.
- customer_id: `uuid` `required`| Identificador do cliente.
- disabled_at: `datetime` `optional`| Data de desativação do contrato.
- created_at: `datetime` `required`| Data de criação do contrato.
- updated_at: `datetime` `required`| Data de atualização do contrato.
- deleted_at: `datetime` `optional`| Data de exclusão do contrato.

**ContractStatus**

Entidade que representa o status do contrato de serviço do cliente com a empresa.

- ownership_id: `uuid` `required` `unique` | Identificador único do proprietário do dado LGPD.
- trace_id: `uuid` `required` `unique` | Identificador único de trace transacional.
- id: `number` `required` `unique` | Identificador único do status interno.
- public_id: `uuid` `required` `unique` | Identificador público do status uso externo.
- contract_id: `uuid` `required`| Identificador do contrato.
- status: `string` `required`| Status do contrato.
- enabled: `boolean` `required`| Indicador de status ativo.
- disabled_at: `datetime` `optional`| Data de desativação do status.
- created_at: `datetime` `required`| Data de criação do status.
- updated_at: `datetime` `required`| Data de atualização do status.
- deleted_at: `datetime` `optional`| Data de exclusão do status.

**ContractSignature**

Entidade que representa a assinatura do contrato de serviço do cliente com a empresa.

- ownership_id: `uuid` `required` `unique` | Identificador único do proprietário do dado LGPD.
- trace_id: `uuid` `required` `unique` | Identificador único de trace transacional.
- id: `number` `required` `unique` | Identificador único da assinatura interno.
- public_id: `uuid` `required` `unique` | Ident
- contract_id: `uuid` `required`| Identificador do contrato.
- customer_id: `uuid` `required`| Identificador do cliente.
- person_id: `uuid` `required`| Identificador da pessoa.
- term_id: `uuid` `required`| Identificador do termo.
- plan_id: `uuid` `required`| Identificador do plano.
- renewal_date: `datetime` `optional`| Data de renovação do contrato.
- disabled_at: `datetime` `optional`| Data de desativação da assinatura.
- created_at: `datetime` `required`| Data de criação da assinatura.
- updated_at: `datetime` `required`| Data de atualização da assinatura.
- deleted_at: `datetime` `optional`| Data de exclusão da assinatura.

**ContractPeriod**

Entidade que representa o período de vigência do contrato de serviço do cliente com a empresa.

- ownership_id: `uuid` `required` `unique` | Identificador único do proprietário do dado LGPD.
- trace_id: `uuid` `required`| Identificador único de trace transacional.
- id: `number` `required`| Identificador único do período interno.
- public_id: `uuid` `required`| Identificador público do período uso externo.
- contract_id: `uuid` `required`| Identificador do contrato.
- signed_by: `uuid` `required`| Identificador da pessoa que assinou o contrato.
- start_date: `datetime` `required`| Data de início do contrato.
- end_date: `datetime` `required`| Data de término do contrato.
- renewal: `boolean` `required`| Foi gerado apartir de uma renovação ou não.
- open: `boolean` `required`| Indicador de contrato em aberto.
- disabled_at: `datetime` `optional`| Data de desativação do período.
- created_at: `datetime` `required`| Data de criação do período.
- updated_at: `datetime` `required`| Data de atualização do período.
- deleted_at: `datetime` `optional`| Data de exclusão do período.

**FeatureFlagStrategy**

Entidade que representa feature flag para controle de disponibilidade de funcionalidades.

- trace_id: `uuid` `required`| Identificador único de trace transacional.
- id: `number` `required`| Identificador único da flag interno.
- public_id: `uuid` `required`| Identificador público da flag uso externo.
- label: `string` `required` `unique` | Rótulo da flag.
- name: `string` `required`| Nome da flag.
- description: `string` `required`| Descrição da flag.
- feature_type: `string` `required` `FeatureFlagType`| Tipo da funcionalidade.
- strategy_type: `string` `required` `FeatureFlagStrategyType` | Tipo de Estratégia de lançamento da funcionalidade.
- scope: `string` `required` `FeatureFlagScope` | Escopo da funcionalidade.
- enabled: `boolean` `required`| Indicador de flag ativa.
- disabled_at: `datetime` `optional`| Data de desativação da flag.
- created_at: `datetime` `required`| Data de criação da flag.
- updated_at: `datetime` `required`| Data de atualização da flag.
- deleted_at: `datetime` `optional`| Data de exclusão da flag.

## Enumeradores

**FeatureFlagType**

- `security`| Tipo de funcionalidade de segurança.
- `operation`| Tipo de funcionalidade de operação.
- `business`| Tipo de funcionalidade de negócio.
- `marketing`| Tipo de funcionalidade de marketing.
- `support`| Tipo de funcionalidade de suporte.
- `infrastructure`| Tipo de funcionalidade de infraestrutura.

**FeatureFlagScope**

- `Global`| FF que afetam todo o serviço
- `Product`| FF que afetam o ambiente de produção
- `Staging`| FF que afetam o ambiente de homologação
- `Quality`| FF que afetam o ambiente de qualidade
- `SandBox`| FF que afetam o ambiente de sandbox
- `Development`| FF que afetam o ambiente de desenvolvimento
- `Microservice`| FF que afetam o ambiente de microserviço produção
- `Mobile`| FF que afetam para ambiente de aplicativo mobile
- `Web`| FF que afetam o ambiente de aplicativo web
- `Cli`| FF que afetam o ambiente de aplicativo cli
- `Region`| FF que afetam o ambiente de clientes de uma região
- `Country`| FF que afetam o ambiente de clientes de um país
- `State`| FF que afetam o ambiente de clientes de um estado
- `City`| FF que afetam o ambiente de clientes de uma cidade
- `Segment`| FF que afetam o ambiente de clientes de um segmento
- `Customer`| FF que afetam o ambiente de usuários de um cliente
- `Group`| FF que afetam o ambiente de grupo de clientes ou de usuários
- `Role`| FF que afetam o ambiente de clientes de um perfil
- `User`| FF que afetam o ambiente de um usuário

**FeatureFlagStrategyType**

- `gradual`| Estratégia de lançamento gradual.
- `all`| Estratégia de lançamento para todos.
- `test`| Estratégia de lançamento para testes.
- `temporary`| Estratégia de lançamento temporário.
- `permanent`| Estratégia de lançamento permanente.
- `manual`| Estratégia de lançamento manual.
- `experiment`| Estratégia de lançamento para experimentos.
- `internal`| Estratégia de lançamento interno.
- `percentage`| Estratégia de lançamento por porcentagem.

**ContactType**

- `email`| Tipo de contato e-mail.
- `phone`| Tipo de contato telefone.
- `mobile`| Tipo de contato celular.
- `fax`| Tipo de contato fax.
- `website`| Tipo de contato website.
- `social`| Tipo de contato rede social.
- `other`| Tipo de contato outro.

**ContractSignatureType**

- `digital`| Tipo de assinatura digital.
- `physical`| Tipo de assinatura física.
