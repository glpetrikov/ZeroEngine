use local_ip_address::list_afinet_netifas;

#[derive(Debug, Clone)]
pub struct LocalIpInfo {
	interface: String,
	address: String,
}

impl LocalIpInfo {
	pub fn interface(&self) -> &str { &self.interface }
	pub fn address(&self) -> &str { &self.address }
}

pub fn refresh_local_ips() -> Vec<LocalIpInfo> {
	let Ok(addresses) = list_afinet_netifas() else {
		return Vec::new();
	};

	addresses
		.into_iter()
		.map(|(interface, address)| LocalIpInfo {
			interface,
			address: address.to_string(),
		})
		.collect()
}
