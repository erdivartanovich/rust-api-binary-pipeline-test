FROM alpine:latest
WORKDIR /app

COPY ./target/x86_64-unknown-linux-musl/release/list_dir_api /app/list_dir_api
COPY ./target/x86_64-unknown-linux-musl/release/list_dir /app/list_dir
COPY ./target/x86_64-unknown-linux-musl/release/actix_api /app/actix_api

# Expose the port the application will run on
EXPOSE 8080

# Create a script to execute the desired binary based on the BINARY environment variable
RUN echo -e '#!/bin/sh\n\
if [ -n "$BINARY" ]; then\n\
    ./$BINARY\n\
else\n\
    echo "No binary specified. Defaulting to list_dir_api"\n\
    ./list_dir_api\n\
fi' > /app/start.sh && chmod +x /app/start.sh

# Set the script as CMD
CMD ["/app/start.sh"]
