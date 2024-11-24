use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

struct Configuration {
    retry: u32,
    endpoint: IpAddr,
    timeout: Duration,
}

impl Configuration {
    fn new(retry: u32, endpoint: IpAddr, timeout: Duration) -> Self {
        Self {
            retry,
            endpoint,
            timeout,
        }
    }
}

struct RequestClient {
    config: Configuration,
}

impl RequestClient {
    /// 設定（依存関係）を注入する。
    /// 設定を変更することで、RequestClientの機能を変更できる。
    fn new(config: Configuration) -> Self {
        Self { config }
    }

    fn send(&self) {
        println!("Send request to {:?}", self.config.endpoint);
        println!(
            "With timeout {:?}, retry count {:?}",
            self.config.timeout, self.config.retry
        );
    }
}

fn main() {
    let config = Configuration::new(3, Ipv4Addr::LOCALHOST.into(), Duration::from_secs(30));
    let client = RequestClient::new(config);
    client.send()
}
