#!/bin/bash

HOST="localhost"
PORT="50051"
SERVICE="customer_pb.CustomerService"
METHOD="CreateCustomer"
DOCUMENT="123456789"
NAME="John Doe"

DATA=$(cat <<EOF
{
  "document": "$DOCUMENT",
  "name": "$NAME"
}
EOF
)

grpcurl -plaintext -d "$DATA" $HOST:$PORT $SERVICE/$METHOD
