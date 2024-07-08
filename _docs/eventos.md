# Microservice: Contract Service

## Eventos

Neste documento estão descritos os eventos que são produzidos e consumidos pelo microserviço de contrato.

### Eventos Produzidos

Os eventos produzidos pelo microserviço de contrato são:

#### Evento: `customer.created`

- **Nome do Evento**: `customer.created`
- **Nome do Tópico**: `{microservice}.customer.created.{version}.{environment}`
- **Versão atual**: `1.0.0`
- **Consumidores**: `audit`

**Descrição**

Evento que representa a criação de um cliente.

**Template do Evento**:
```json
{
  "topic": "{microservice}.customer.created.{version}.{environment}",
  "producer_id": "{producer_id = uuid}",
  "producer_name": "{microservice = string}",
  "transaction_id": "uuid",
  "timestamp": "timestamp",
  "event_type": "customer.created",
  "payload": {
    "transaction_id": "uuid",
    "id": "int",
    "public_id": "uuid",
    "document": "string",
    "name": "string",
    "disabled_at": "timestamp",
    "created_at": "timestamp",
    "updated_at": "timestamp",
    "deleted_at": "timestamp"
  },
  "metadata": {
    "event_id": "uuid",
    "event_version": "{version}",
    "event_date": "timestamp",
    "environment": "{environment}"
  }
}
```

**Exemplo do Evento**:
```json
{
  "topic": "contract_service.customer.created.v1.dev",
  "producer_id": "af006830-d886-49b5-a1f9-45495795cfe8",
  "producer_name": "contract_service",
  "transaction_id": "b15ef0b0-470c-4d63-949f-d32b2dc0789d",
  "timestamp": "1720474158",
  "event_type": "customer.created",
  "payload": {
    "transaction_id": "b15ef0b0-470c-4d63-949f-d32b2dc0789d",
    "id": 4,
    "public_id": "742f4c13-f20e-429e-9db4-092b9babef5a",
    "document": "123456789",
    "name": "Rust Foundation",
    "disabled_at": null,
    "created_at": "2024-07-08T21:29:18.275050Z",
    "updated_at": "2024-07-08T21:29:18.275052Z",
    "deleted_at": null
  },
  "metadata": {
    "event_id": "3d2a9672-3e53-41ae-a8ea-a53f7ece805d",
    "event_version": "v1",
    "event_date": "2024-07-08T21:29:18.278721555Z",
    "environment": "dev"
  }
}
```

### Eventos Consumidos

Atualmente o microserviço de contrato não consome eventos de outros microserviços.
