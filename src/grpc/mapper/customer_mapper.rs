use crate::grpc::pb::customer_pb::Customer as CustomerGrpc;
use crate::internal::domain::model::Customer;
use prost_types::Timestamp;

pub fn map_to_customer_grpc(customer: Customer) -> CustomerGrpc {
    CustomerGrpc {
        transaction_id: customer.transaction_id.to_string(),
        id: customer.id,
        public_id: customer.public_id.to_string(),
        document: customer.document,
        name: customer.name,
        disabled_at: customer.disabled_at.map(|d| Timestamp {
            seconds: d.timestamp(),
            nanos: d.timestamp_subsec_nanos() as i32,
        }),
        created_at: Some(Timestamp {
            seconds: customer.created_at.timestamp(),
            nanos: customer.created_at.timestamp_subsec_nanos() as i32,
        }),
        updated_at: Some(Timestamp {
            seconds: customer.updated_at.timestamp(),
            nanos: customer.updated_at.timestamp_subsec_nanos() as i32,
        }),
    }
}
