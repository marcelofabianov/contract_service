# Microservice: Contract Service

## Políticas de versionamento

Este documento descreve as políticas de versionamento adotadas para o projeto Contract Service.

### Versionamento Semântico

O projeto Contract Service segue o padrão de versionamento semântico, conforme descrito em [semver.org](https://semver.org/).

## Commits

Os commits devem seguir o padrão de mensagem definido pelo [Conventional Commits](https://www.conventionalcommits.org/).

### Tipos

- **feat**: Nova funcionalidade
- **fix**: Correção de bug
- **docs**: Documentação
- **style**: Formatação
- **refactor**: Refatoração
- **test**: Testes
- **chore**: Tarefas de build

### Mensagem

- Os commits devem estar no imperativo, com a primeira letra maiúscula e sem ponto final.
- A mensagem deve ser clara e objetiva, descrevendo o que foi feito.
- A mensagem deve ter no máximo 100 caracteres.
- A mensagem deve ser escrita em português após o tipo.
- Ao final de cada mensagem de commit deve ser adicionado o número da issue relacionada ao commit e o nome do microserviço. No "Assunto do commit".

**Importante**:

- Caso o commit não se encaixe em nenhum dos tipos acima, utilize o tipo `chore`.
- Caso o commit seja de algo complexo crie mensagem mais detalhada com `git commit -a`
- Caso o commit seja de algo simples, crie mensagem curta com `git commit -m`
- Tenha em mente criar commits atômicos, ou seja, que representem uma única mudança.

### Exemplo

```bash
git commit -m "feat: Adiciona nova funcionalidade"
```

### Branches

- **main**: Branch principal, contém a versão de produção.
- **develop**: Branch de desenvolvimento, contém a versão de homologação.
- **feature**: Branch de desenvolvimento de novas funcionalidades.
- **hotfix**: Branch de correção de bugs em produção.

**Importante**:

- As branches de feature e hotfix devem ser criadas a partir da branch develop.
- As branches de feature e hotfix devem ser mescladas na branch develop.
- A branch develop deve ser mesclada na branch main.
- O nome das branches deve seguir o padrão `tipo/nome`, onde `tipo` é o tipo da branch e `nome` é o nome da branch.
- O nome das branches deve ser escrito em minúsculo e separado por hífen.
- O nome das branches deve ser curto e objetivo.
- As branches a seguir devem ser protegidas permitindo apenas merge via pull request: main, develop
- Cada membro do time deve fazer seu próprio fork do repositório e trabalhar em branches separadas.
- Cada membro do time deve fazer um pull request para a branch raiz do repositório para o branch da feature que está trabalhando.

### Merge

- O merge deve ser feito via pull request.
- O pull request deve ser revisado por outro membro do time.
- O pull request deve passar por testes automatizados.

### Tags

- As tags devem ser criadas a partir da branch main.
- As tags devem seguir o padrão `vX.Y.Z`, onde `X` é a versão principal, `Y` é a versão secundária e `Z` é a versão de correção.
