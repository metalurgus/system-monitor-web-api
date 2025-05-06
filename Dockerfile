FROM debian:bullseye-slim

WORKDIR /app
COPY server-monitor-api /app/server-monitor-api

EXPOSE 9999

CMD ["./server-monitor-api"]