# Microservice: Contract Service

[Home](../README.md)

## Eventos

Neste documento estão descritos os eventos que são produzidos e consumidos pelo microserviço de contrato.

### Eventos Produzidos

Os eventos produzidos pelo microserviço de contrato são:

#### Evento: `customer.created`

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

### Eventos Consumidos

Atualmente o microserviço de contrato não consome eventos de outros microserviços.
