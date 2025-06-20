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
