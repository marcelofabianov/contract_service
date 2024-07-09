# Microservice: Contract Service

[Home](../README.md)

## Políticas de desenvolvimento

Este documento descreve as práticas e diretrizes de desenvolvimento para o microserviço ContractService, que utiliza Rust, gRPC, Kafka, Postgres e Kong para comunicação com clientes.

### Padrões de Codificação

**Convencões de nomenclatura**

- **CamelCase**: Utilize CamelCase para nomes de structs, enums, traits.
- **snake_case**: Utilize snake_case para nomes de funções, variáveis, campos de structs, enums, traits, módulos, arquivos e diretórios.
- **SCREAMING_SNAKE_CASE**: Utilize SCREAMING_SNAKE_CASE para constantes.

**Estilo de código**

- **Indentação**: Utilize 4 espaços para indentação.
- **Comprimento da linha**: Limite o comprimento da linha a 100 caracteres.
- **Comentários**: Utilize comentários para explicar o código, não o óbvio, mas o porquê.
- **Documentação**: Documente o código utilizando doc comments.

**Evite o uso**

- **unsafe**: Evite o uso de código inseguro.
- **macros**: Evite o uso de macros, preferindo funções.
- **mut**: Evite o uso de mut, preferindo imutabilidade sempre que possível.
- **clippy**: Evite o uso de código que gere warnings do clippy.
- **panic!**: Evite o uso de panic!, preferindo Result ou Option.
- **Result**: Evite o uso de Result sem tratamento de erro.
- **Option**: Evite o uso de Option sem tratamento de erro.
- **unwrap**: Evite o uso de unwrap_or ou similar, preferindo match ou if let.
- **expect**: Evite o uso de expect ou similar, preferindo match ou if let.

### Controle de Versão

**Versionamento Semântico**

- Adotamos o versionamento semântico para o projeto, seguindo o formato MAJOR.MINOR.PATCH. [Leia mais](https://semver.org/)
- Adotamos o uso de GitFlow para branchs e tags. [Leia mais](https://www.atlassian.com/br/git/tutorials/comparing-workflows/gitflow-workflow#:~:text=O%20que%20%C3%A9%20Gitflow%3F,por%20Vincent%20Driessen%20no%20nvie.)
- Adotamos para commit convencional commits. [Leia mais](https://www.conventionalcommits.org/en/v1.0.0/)
- Adotamos para changelog apresentar a lista de alterações por versão. [Leia mais](https://keepachangelog.com/pt-BR/1.0.0/)

Mais detalhes sobre versionamento podem ser encontrados em [Versionamento](_docs/versionamento.md).

### Code Review

No processo de code review, adotamos as seguintes práticas:

- **Pull Requests**: Crie pull requests para revisão de código.
- **Revisão**: Solicite revisão de código de outro membro da equipe.
- **Comentários**: Adicione comentários e sugestões de melhoria.
- **Aprovação**: Aguarde a aprovação de 2 ou mais membros da equipe.
- **Merge**: Realize o merge apenas após a aprovação.
- **Branchs**: Mantenha branchs atualizadas com a develop antes de criar pull requests.
- **Feedback**: Forneça feedback construtivo e respeitoso.
- **Aprendizado**: Utilize o code review para aprendizado e melhoria contínua.
- **Dados sensíveis**: Sempre observe dados sensíveis durante o code review.
- **Segurança**: Sempre observe vulnerabilidades de segurança durante o code review.

### Gerenciamento de Dependências

Em Rust, utilizamos o Cargo para gerenciamento de dependências. Adotamos as seguintes práticas:

- **Cargo.toml**: Mantenha o arquivo Cargo.toml organizado e com as dependências atualizadas.
- **Cargo.lock**: Mantenha o arquivo Cargo.lock versionado.
- **Crates.io**: Utilize bibliotecas de terceiros apenas quando necessário e de confiança.
- **Crates**: Prefira bibliotecas comunitárias bem mantidas e com documentação.
- **Importante**: Fique atendo a novas versões de dependências, atualize sempre que possível.

### Testes

Criamos testes unitários e de integração para garantir a qualidade do código. Adotamos as seguintes práticas:

- **Testes unitários**: Teste funções, métodos e módulos isoladamente.
- **Testes de integração**: Teste a integração entre módulos, serviços e sistemas.
- **Testes de carga**: Teste a capacidade de carga do sistema.


