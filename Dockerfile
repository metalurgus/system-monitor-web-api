FROM ubuntu:24.04

# Install required system libraries
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY server-monitor-api /app/server-monitor-api

# Copy static files
COPY static ./static

EXPOSE 9999

CMD ["./server-monitor-api"]