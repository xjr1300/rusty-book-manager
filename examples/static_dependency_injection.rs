#![allow(unused)]

struct Configuration {
    retry: u32,
    timeout: u32,
}

trait RequestClient {
    fn send(&self);
}

struct GrpcRequestClient {
    config: Configuration,
}

impl RequestClient for GrpcRequestClient {
    fn send(&self) {
        println!("Sent request by gRPC");
    }
}

struct HttpRequestClient {
    config: Configuration,
}

impl RequestClient for HttpRequestClient {
    fn send(&self) {
        println!("Sent request by HTTP");
    }
}

trait Logger {
    fn log(&self);
}

struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self) {
        println!("Log to stdout");
    }
}

struct RemoteLogger;

impl Logger for RemoteLogger {
    fn log(&self) {
        println!("Sent logs remote")
    }
}

struct Service<T: RequestClient, L: Logger> {
    client: T,
    logger: L,
}

impl<T: RequestClient, L: Logger> Service<T, L> {
    fn call(&self) {
        self.client.send();
        self.logger.log();
    }
}

fn main() {
    let config = Configuration {
        retry: 3,
        timeout: 30,
    };
    let grpc_client = GrpcRequestClient { config };
    let stdout_logger = StdoutLogger;
    let grpc_service = Service {
        client: grpc_client,
        logger: stdout_logger,
    };
    grpc_service.call();

    let config = Configuration {
        retry: 3,
        timeout: 60,
    };
    let http_client = HttpRequestClient { config };
    let remote_logger = RemoteLogger;
    let http_service = Service {
        client: http_client,
        logger: remote_logger,
    };
    http_service.call();
}
