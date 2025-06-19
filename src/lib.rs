// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Check if hex string is valid and has at least 8 characters (4 bytes for version)
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    // Extract first 4 bytes (8 hex characters) for version
    let version_hex = &raw_tx_hex[0..8];

    // Decode hex to bytes
    let version_bytes = hex::decode(version_hex).map_err(|_| "Hex decode error".to_string())?;

    // Convert bytes to u32 (little-endian)
    let version = u32::from_le_bytes([
        version_bytes[0],
        version_bytes[1],
        version_bytes[2],
        version_bytes[3],
    ]);

    Ok(version)
}

fn main() {
    let extract = extract_tx_version("02000000000101706dc474338179f4ab8b7f0a4d07a2050113d7a0a9d21162e98b7319b102d3050100000000fdffffff02c9e10100000000001600148744bf9d300850a598b1a891f9a8d66524a4773065fc000000000000160014d1fae9a4de635c9c2e576238251d71be28a34dff0247304402201bf91432bbb345dcaa883a14fb7f18df7c821b160cc693f242112ba1a0acbdeb0220541b082c5fd4174f8eae782e213c1ebfc87b0598740ee0ef8463474debe83817012102062aea304064469ed250f46622e411de7eff4f07703e4273df6c80d58954ac2f00000000"); // Example hex
    match extract {
        Ok(version) => println!("Transaction version: {}", version),
        Err(e) => println!("Error: {}", e),
    }
}
