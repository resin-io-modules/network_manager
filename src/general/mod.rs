/// Gets the Network Manager status.
///
/// # Examples
///
/// ```
/// let status = network_manager::general::status().unwrap();
/// println!("{:?}", status);
/// ```
pub fn status() -> Result<Status, String> {
    // Get network manager status

    let status = Status {
        state: NetworkManagerState::ConnectedGlobal,
        connectivity: Connectivity::Full,
        wifi_enabled: true,
        eth_enabled: false,
    };

    Ok(status)
}

#[derive(Debug)]
pub struct Status {
    state: NetworkManagerState,
    connectivity: Connectivity,
    wifi_enabled: bool,
    eth_enabled: bool,
}

#[derive(Debug)]
pub enum NetworkManagerState {
    Unknown,
    Asleep,
    Disconnected,
    Disconnecting,
    Connecting,
    ConnectedLocal,
    ConnectedSite,
    ConnectedGlobal,
}

#[derive(Debug)]
pub enum ServiceState {
    Active,
    Reloading,
    Inactive,
    Failed,
    Activating,
    Deactivating,
}

#[derive(Debug)]
pub enum ConnectionState {
    Unknown,
    Activating,
    Activated,
    Deactivating,
    Deactivated,
}

#[derive(Debug)]
pub enum DeviceState {
    Unknown,
    Unmanaged,
    Unavailable,
    Disconnected,
    Activated,
    Deactivating,
    Failed,
}

#[derive(Debug)]
pub enum Connectivity {
    Unknown,
    None,
    Portal,
    Limited,
    Full,
}

#[derive(Debug)]
pub enum Security {
    None,
    WEP,
    WPA1,
    WPA2,
}

#[derive(Debug)]
pub enum Interface {
    Unknown,
    Generic,
    Ethernet,
    WiFi,
    Bridge,
}
