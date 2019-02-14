/// Used to choose the offline mode behavior, `Manual` or `Auto`.
#[derive(Debug, PartialEq)]
pub enum OfflineMode {
    Manual,
    Auto,
}

use std::time;

/// Options are used to configure Kuzzle SDK behavior.
/// Use them when instanciate `Kuzzle` structure to pass it a set of options.
#[derive(Debug)]
pub struct KuzzleOptions {
    _auto_queue: bool,
    _auto_reconnect: bool,
    _auto_replay: bool,
    _auto_resubscribe: bool,
    _host: String,
    _port: u32,
    _offline_mode: OfflineMode,
    _queue_max_size: u32,
    _queue_ttl: time::Duration,
    _reconnection_delay: time::Duration,
    _replay_interval: time::Duration,
    _ssl_connection: bool,
}

impl Default for KuzzleOptions {
    fn default() -> KuzzleOptions {
        KuzzleOptions {
            _auto_queue: false,
            _auto_reconnect: true,
            _auto_replay: false,
            _auto_resubscribe: true,
            _host: String::from("localhost"),
            _port: 7512,
            _offline_mode: OfflineMode::Manual,
            _queue_max_size: 500,
            _queue_ttl: time::Duration::from_millis(120000),
            _reconnection_delay: time::Duration::from_millis(1000),
            _replay_interval: time::Duration::from_millis(10),
            _ssl_connection: false,
        }
    }
}

impl KuzzleOptions {
    /// Returns a KuzzleOption struct with the given host and port.
    /// KuzzleOptions is used to pass options to Kuzzle struct.
    /// When created this struct follow the builder pattern and permits
    /// on the fly updates.
    ///
    /// # Arguments
    ///
    /// * `host` - A `&str` representing the Kuzzle server host.
    /// * `port` - A `u32` representing the Kuzzle server port.
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::types::KuzzleOptions;
    /// let standard_use = KuzzleOptions::new("localhost", 7512);
    /// // or
    /// let builder_use = KuzzleOptions::new("localhost", 7512)
    ///     .set_ssl_connection(true)
    ///     .set_auto_queue(false)
    ///     .set_queue_ttl(1000);
    /// ```
    pub fn new(host: &str, port: u32) -> KuzzleOptions {
        KuzzleOptions {
            _host: String::from(host),
            _port: port,
            ..Self::default()
        }
    }

    pub fn auto_queue(&self) -> &bool {
        &self._auto_queue
    }

    pub fn auto_reconnect(&self) -> &bool {
        &self._auto_reconnect
    }

    pub fn auto_replay(&self) -> &bool {
        &self._auto_replay
    }

    pub fn auto_resubscribe(&self) -> &bool {
        &self._auto_resubscribe
    }

    pub fn offline_mode(&self) -> &OfflineMode {
        &self._offline_mode
    }

    pub fn host(&self) -> &String {
        &self._host
    }

    pub fn port(&self) -> &u32 {
        &self._port
    }

    pub fn queue_max_size(&self) -> &u32 {
        &self._queue_max_size
    }

    pub fn queue_ttl(&self) -> &time::Duration {
        &self._queue_ttl
    }

    pub fn reconnection_delay(&self) -> &time::Duration {
        &self._reconnection_delay
    }

    pub fn replay_interval(&self) -> &time::Duration {
        &self._replay_interval
    }

    pub fn ssl_connection(&self) -> &bool {
        &self._ssl_connection
    }

    pub fn set_auto_queue(mut self, auto_queue: bool) -> Self {
        self._auto_queue = auto_queue;
        self
    }

    pub fn set_auto_reconnect(mut self, auto_reconnect: bool) -> Self {
        self._auto_reconnect = auto_reconnect;
        self
    }

    pub fn set_auto_replay(mut self, auto_replay: bool) -> Self {
        self._auto_replay = auto_replay;
        self
    }

    pub fn set_auto_resubscribe(mut self, auto_resubscribe: bool) -> Self {
        self._auto_resubscribe = auto_resubscribe;
        self
    }

    pub fn set_offline_mode(mut self, mode: OfflineMode) -> Self {
        self._offline_mode = mode;
        self
    }

    pub fn set_host(mut self, host: &str) -> Self {
        self._host = String::from(host);
        self
    }

    pub fn set_port(mut self, port: u32) -> Self {
        self._port = port;
        self
    }

    pub fn set_queue_max_size(mut self, max_size: u32) -> Self {
        self._queue_max_size = max_size;
        self
    }

    pub fn set_queue_ttl(mut self, ttl: u64) -> Self {
        self._queue_ttl = time::Duration::from_millis(ttl);
        self
    }

    pub fn set_reconnection_delay(mut self, delay: u64) -> Self {
        self._reconnection_delay = time::Duration::from_millis(delay);
        self
    }

    pub fn set_replay_interval(mut self, interval: u64) -> Self {
        self._replay_interval = time::Duration::from_millis(interval);
        self
    }

    pub fn set_ssl_connection(mut self, ssl: bool) -> Self {
        self._ssl_connection = ssl;
        self
    }
}

pub struct QueryOptions {
    queuable: bool,
}

impl QueryOptions {
    pub fn new() -> QueryOptions {
        QueryOptions { queuable: true }
    }

    pub fn queuable(&self) -> bool {
        self.queuable
    }
}
