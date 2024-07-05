# Microservice: Contract Service

**Projeto para estudo de Rust, gRPC e Kafka**

O ContractService é um microserviço dedicado à gestão de contratos dentro de um sistema
ERP SaaS, permitindo a administração eficiente de contratos de clientes, incluindo detalhes de
planos, funcionalidades contratadas, termos de uso assinados.

## Documentação

A seguir temos tópicos que descrevem o projeto e seus requisitos, politicas de versionamento, politicas de desenvolvimento, arquitetura, etc.

- [Políticas de desenvolvimento](_docs/politicas-de-desenvolvimento.md)
- [Versionamento](_docs/versionamento.md)
- [ChangeLog](_docs/changelog.md)
- [Requisitos](_docs/requisitos.md)
- [Requisitos não funcionais](_docs/requisitos-nao-funcionais.md)
- [Arquitetura](_docs/arquitetura.md)
- [gRPC](_docs/grpc.md)
- [Eventos](_docs/eventos.md)
- [Entidades](_docs/entidades.md)

### Preparação do ambiente local

Para rodar o projeto localmente, siga os passos abaixo:

[Leia no guia](_docs/preparacao-ambiente-local.md)

### Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

### Autor

Marcelo Fabiano: [LinkedIn](https://www.linkedin.com/in/marcelofabianov/) e [GitHub](https://github.com/marcelofabianov)

### Referências

**Livros**

- [Fundamentos da Arquitetura de Software](https://www.amazon.com.br/Fundamentos-Arquitetura-Software-Abordagem-Engenharia/dp/8550819859/ref=sr_1_1_sspa?__mk_pt_BR=%C3%85M%C3%85%C5%BD%C3%95%C3%91&crid=2SDBX4TLPGDS3&dib=eyJ2IjoiMSJ9.MvcBgbyiVODTr2aeAdC1-RNvXHvj0c4Gex24KTnRoklFL8c_WAkXHUpb-U-hp7HYQ40G9zCg3v4CorOXaHAtB_SCe8k1Zb4UXoOqxM8JP-EwWgznhMhaIbL8HgAyXDEraeZdV2bioDHDndqpB1cNXH0lAbe3OdXInHR2ZQFovH8eehQF2jQzhqaUd5GV3t-FIWae4EkOfeNPWnFqS_G_1tc7Wb1dfBrVMIgpK05b7dSVfoaDxrtk3crG2QfG5qGkNliV7VayDTn-9TBcgA7mo78EYY0-aw3c1F-Ei_TqBgQ.W1JhhPCx4OCo6t4avmaYRjNu4_e7cGzNGya_dIRWhYs&dib_tag=se&keywords=livros+da+arquitetura+de+software&qid=1720138986&sprefix=livros+da+arquitetura+de+softwa%2Caps%2C235&sr=8-1-spons&sp_csd=d2lkZ2V0TmFtZT1zcF9hdGY&psc=1)
- [Implementando Domain-Driven Design](https://www.amazon.com.br/Implementando-Domain-Driven-design-Vernon/dp/8576089521/ref=sr_1_20?crid=1AA93LQQKX5YX&dib=eyJ2IjoiMSJ9.yMpjr4T1bfjEiKF84ONtZ3aKxMvnGykGZwRnbPEkmx9eAOq2dgadjOYDcI3tBHanZIx5dbQPYlRWkzd0nfaqOeNnTZRRbgRX3KoCoxnsuJIGpLBSfZq3-60Mtizhdb5TN8JuOvVrrzVpGcQkh7PjbK8MKNq70iHq7cIqL8kIWOeScBXFQJGoWFTBWpie6vceFPhCZONooX_hXTOI32G3GuSdaO6IbE178ZbUWygtMa0gce6R7iQ3A58eLmtkiYAfO3jG9stYNkWV1mwTiCLLdFN4L3qPntImfJ61QCyHxrM.zl0_W-TtUaHhXJY1O3r4XbRHjCVhM1w9kyqUFaDcJAY&dib_tag=se&keywords=livros+de+programa%C3%A7%C3%A3o&qid=1720138877&sprefix=livros+de+%2Caps%2C266&sr=8-20&ufe=app_do%3Aamzn1.fos.6d798eae-cadf-45de-946a-f477d47705b9)
- [Domain-Driven Design](https://www.amazon.com.br/Domain-Driven-Design-Atacando-Complexidades-Software/dp/8550800651/ref=pd_sim_d_sccl_2_1/136-3161343-7496541?pd_rd_w=qU630&content-id=amzn1.sym.8555f615-361b-42f7-96c4-206bb8a5174e&pf_rd_p=8555f615-361b-42f7-96c4-206bb8a5174e&pf_rd_r=VG7WJ0H91F215VY8XNNX&pd_rd_wg=wbjIS&pd_rd_r=a4beafb9-2e73-4d5b-af0f-59d9d12e883e&pd_rd_i=8550800651&psc=1)
- [Microservices Patterns](https://www.amazon.com.br/Microservice-Patterns-examples-Chris-Richardson/dp/1617294543/ref=srd_d_ssims_T2_d_sccl_3_18/136-3161343-7496541?pd_rd_w=q7nmD&content-id=amzn1.sym.8986ba87-05f0-4264-97b0-11bb7015de48&pf_rd_p=8986ba87-05f0-4264-97b0-11bb7015de48&pf_rd_r=VG7WJ0H91F215VY8XNNX&pd_rd_wg=wbjIS&pd_rd_r=a4beafb9-2e73-4d5b-af0f-59d9d12e883e&pd_rd_i=1617294543&psc=1)
- [Refatoração: Aperfeiçoando o Design de Códigos Existentes](https://www.amazon.com.br/Refatora%C3%A7%C3%A3o-Aperfei%C3%A7oando-Design-C%C3%B3digos-Existentes/dp/8575227246/ref=sr_1_9?crid=1AA93LQQKX5YX&dib=eyJ2IjoiMSJ9.yMpjr4T1bfjEiKF84ONtZ3aKxMvnGykGZwRnbPEkmx9eAOq2dgadjOYDcI3tBHanZIx5dbQPYlRWkzd0nfaqOeNnTZRRbgRX3KoCoxnsuJIGpLBSfZq3-60Mtizhdb5TN8JuOvVrrzVpGcQkh7PjbK8MKNq70iHq7cIqL8kIWOeScBXFQJGoWFTBWpie6vceFPhCZONooX_hXTOI32G3GuSdaO6IbE178ZbUWygtMa0gce6R7iQ3A58eLmtkiYAfO3jG9stYNkWV1mwTiCLLdFN4L3qPntImfJ61QCyHxrM.zl0_W-TtUaHhXJY1O3r4XbRHjCVhM1w9kyqUFaDcJAY&dib_tag=se&keywords=livros+de+programa%C3%A7%C3%A3o&qid=1720138877&sprefix=livros+de+%2Caps%2C266&sr=8-9&ufe=app_do%3Aamzn1.fos.6d798eae-cadf-45de-946a-f477d47705b9)
- [Padrões de Projetos](https://www.amazon.com.br/Padr%C3%B5es-Projetos-Solu%C3%A7%C3%B5es-Reutiliz%C3%A1veis-Orientados/dp/8573076100/ref=sr_1_4?crid=1AA93LQQKX5YX&dib=eyJ2IjoiMSJ9.yMpjr4T1bfjEiKF84ONtZ3aKxMvnGykGZwRnbPEkmx9eAOq2dgadjOYDcI3tBHanZIx5dbQPYlRWkzd0nfaqOeNnTZRRbgRX3KoCoxnsuJIGpLBSfZq3-60Mtizhdb5TN8JuOvVrrzVpGcQkh7PjbK8MKNq70iHq7cIqL8kIWOeScBXFQJGoWFTBWpie6vceFPhCZONooX_hXTOI32G3GuSdaO6IbE178ZbUWygtMa0gce6R7iQ3A58eLmtkiYAfO3jG9stYNkWV1mwTiCLLdFN4L3qPntImfJ61QCyHxrM.zl0_W-TtUaHhXJY1O3r4XbRHjCVhM1w9kyqUFaDcJAY&dib_tag=se&keywords=livros+de+programa%C3%A7%C3%A3o&qid=1720138877&sprefix=livros+de+%2Caps%2C266&sr=8-4&ufe=app_do%3Aamzn1.fos.6121c6c4-c969-43ae-92f7-cc248fc6181d)
- [Arquitetura de software distribuído](https://www.amazon.com.br/Arquitetura-software-distribu%C3%ADdo-pr%C3%A1ticas-microsservi%C3%A7os-ebook/dp/B09K7WZKSB)
- [Arquitetura Orientada a Eventos](https://www.amazon.com.br/Arquitetura-Orientada-Eventos-Solu%C3%A7%C3%B5es-escal%C3%A1veis-ebook/dp/B0CKWW42ML/ref=sr_1_43?__mk_pt_BR=%C3%85M%C3%85%C5%BD%C3%95%C3%91&crid=2SDBX4TLPGDS3&dib=eyJ2IjoiMSJ9.MvcBgbyiVODTr2aeAdC1-RNvXHvj0c4Gex24KTnRoklFL8c_WAkXHUpb-U-hp7HYQ40G9zCg3v4CorOXaHAtB_SCe8k1Zb4UXoOqxM8JP-EwWgznhMhaIbL8HgAyXDEraeZdV2bioDHDndqpB1cNXH0lAbe3OdXInHR2ZQFovH8eehQF2jQzhqaUd5GV3t-FIWae4EkOfeNPWnFqS_G_1tc7Wb1dfBrVMIgpK05b7dSVfoaDxrtk3crG2QfG5qGkNliV7VayDTn-9TBcgA7mo78EYY0-aw3c1F-Ei_TqBgQ.W1JhhPCx4OCo6t4avmaYRjNu4_e7cGzNGya_dIRWhYs&dib_tag=se&keywords=livros+da+arquitetura+de+software&qid=1720138986&sprefix=livros+da+arquitetura+de+softwa%2Caps%2C235&sr=8-43)

**Sites**

- [Rust](https://www.rust-lang.org/)
- [gRPC](https://grpc.io/)
- [Kafka](https://kafka.apache.org/)
- [OpenTelemetry](https://opentelemetry.io/)
- [Elastic Stack](https://www.elastic.co/pt/)
- [Kong](https://konghq.com/)

### Agradecimentos

- [Full Cycle](https://fullcycle.com.br/)
- [Branas.io](https://branas.io/)
