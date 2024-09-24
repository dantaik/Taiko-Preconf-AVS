FROM rust:1.80 as builder

# Set the working directory inside the container
WORKDIR /usr/src/node
# Copy the entire project into the container
COPY . .

# Build the project in release mode
RUN cargo build --release

# Use ubuntu as the base image
FROM ubuntu:latest

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/node/target/release/taiko_preconf_avs_node /usr/local/bin/testmevboost

# Run the binary
# CMD in docker-compose
# CMD ["test-mev-boost"]
