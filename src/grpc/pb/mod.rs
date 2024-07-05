pub mod customer_pb {
    include!("customer_pb.rs");
}

pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("descriptor");
