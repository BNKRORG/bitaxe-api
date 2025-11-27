//! Bitaxe API response types

use serde::{Deserialize, Deserializer, de};

/// Share rejected reason
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ShareRejectedReason {
    /// Shares rejected for this reason
    pub count: u64,
    /// Rejection reason from pool
    pub message: String,
}

/// System information
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SystemInfo {
    /// ASIC model identifier
    #[serde(rename = "ASICModel")]
    pub asic_model: String,
    /// Firmware version
    #[serde(rename = "version")]
    pub version: String,
    /// AxeOS version
    #[serde(rename = "axeOSVersion")]
    pub axe_os_version: String,
    /// Hardware board version
    #[serde(rename = "boardVersion")]
    pub board_version: String,
    /// Primary stratum server URL
    #[serde(rename = "stratumURL")]
    pub stratum_url: String,
    /// Primary stratum server port
    #[serde(rename = "stratumPort")]
    pub stratum_port: u16,
    /// Primary stratum username
    #[serde(rename = "stratumUser")]
    pub stratum_user: String,
    /// Whether using fallback stratum
    #[serde(rename = "isUsingFallbackStratum")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub is_using_fallback_stratum: bool,
    /// Fallback stratum server URL
    #[serde(rename = "fallbackStratumURL")]
    pub fallback_stratum_url: String,
    /// Fallback stratum server port
    #[serde(rename = "fallbackStratumPort")]
    pub fallback_stratum_port: u16,
    /// Fallback stratum username
    #[serde(rename = "fallbackStratumUser")]
    pub fallback_stratum_user: String,
    /// Latency to stratum server in ms
    #[serde(rename = "responseTime")]
    pub stratum_latency: Option<f64>,
    /// Current hashrate
    #[serde(rename = "hashRate")]
    pub hashrate: f64,
    /// Expected hashrate
    #[serde(rename = "expectedHashrate")]
    pub expected_hashrate: f64,
    /// Best difficulty achieved
    #[serde(rename = "bestDiff")]
    #[serde(deserialize_with = "deserialize_difficulty")]
    pub best_diff: u64,
    /// Best difficulty achieved in current session
    #[serde(rename = "bestSessionDiff")]
    #[serde(deserialize_with = "deserialize_difficulty")]
    pub best_session_diff: u64,
    /// Current pool difficulty
    #[serde(rename = "poolDifficulty")]
    pub pool_difficulty: u64,
    /// Number of accepted shares
    #[serde(rename = "sharesAccepted")]
    pub shares_accepted: u64,
    /// Number of rejected shares
    #[serde(rename = "sharesRejected")]
    pub shares_rejected: u64,
    /// Reason(s) shares were rejected
    #[serde(rename = "sharesRejectedReasons")]
    pub shares_rejected_reasons: Vec<ShareRejectedReason>,
    /// Whether a block was found
    #[serde(default)]
    #[serde(rename = "blockFound")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub block_found: bool,
    /// Auto fan speed
    ///
    /// `true` if auto, `false` if manual.
    #[serde(rename = "autofanspeed")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub auto_fan_speed: bool,
    /// Current fan speed in RPM
    #[serde(rename = "fanrpm")]
    pub fan_rpm: i64,
    /// Current fan speed percentage
    #[serde(rename = "fanspeed")]
    pub fan_speed: f64,
    /// ASIC frequency in MHz
    #[serde(rename = "frequency")]
    pub frequency: i64,
    /// Device hostname
    #[serde(rename = "hostname")]
    pub hostname: String,
    /// WiFi network SSID
    #[serde(rename = "ssid")]
    pub ssid: String,
    /// WiFi signal strength
    #[serde(rename = "wifiRSSI")]
    pub wifi_rssi: i64,
    /// WiFi connection status
    #[serde(rename = "wifiStatus")]
    pub wifi_status: String,
    /// Device MAC address
    #[serde(rename = "macAddr")]
    pub mac_addr: String,
    /// Whether AP mode is enabled
    #[serde(rename = "apEnabled")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub ap_enabled: bool,
    /// Whether PSRAM is available
    #[serde(rename = "isPSRAMAvailable")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub is_psram_available: bool,
    /// Set custom voltage/frequency in AxeOS
    #[serde(rename = "overclockEnabled")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub overclock_enabled: bool,
    /// Overheat protection mode
    #[serde(rename = "overheat_mode")]
    #[serde(deserialize_with = "deserialize_bool_from_int")]
    pub overheat_protection_mode: bool,
    /// Average chip temperature
    #[serde(rename = "temp")]
    pub temp: f64,
    /// Manual Temperature Target in °C when autofanspeed is enabled
    #[serde(rename = "temptarget")]
    pub temp_target: f64,
    /// System uptime in seconds
    #[serde(rename = "uptimeSeconds")]
    pub uptime_seconds: u64,
}

fn deserialize_bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: u8 = u8::deserialize(deserializer)?;
    Ok(value != 0)
}

fn deserialize_difficulty<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    let parts: Vec<&str> = s.split_whitespace().collect();

    if parts.is_empty() {
        return Err(de::Error::custom(format!(
            "Invalid difficulty format: {}",
            s
        )));
    }

    let number: f64 = parts[0]
        .parse()
        .map_err(|_| de::Error::custom(format!("Invalid number: {}", parts[0])))?;

    // If there's no unit, return the number as-is
    if parts.len() == 1 {
        return Ok(number as u64);
    }

    let multiplier: f64 = match parts[1] {
        "K" => 1_000.0,
        "M" => 1_000_000.0,
        "G" => 1_000_000_000.0,
        "T" => 1_000_000_000_000.0,
        "P" => 1_000_000_000_000_000.0,
        "E" => 1_000_000_000_000_000_000.0,
        unit => return Err(de::Error::custom(format!("Unknown unit: {}", unit))),
    };

    Ok((number * multiplier).round() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_system_info() {
        let json: &str = r#"{
	"power":	22.060667037963867,
	"voltage":	5046.875,
	"current":	14937.5,
	"temp":	64.125,
	"temp2":	0,
	"vrTemp":	86,
	"maxPower":	40,
	"nominalVoltage":	5,
	"hashRate":	1184.8093224631666,
	"expectedHashrate":	1071,
	"bestDiff":	"2.03 G",
	"bestSessionDiff":	"138.17 M",
	"poolDifficulty":	1000,
	"isUsingFallbackStratum":	0,
	"isPSRAMAvailable":	1,
	"freeHeap":	8372724,
	"coreVoltage":	1150,
	"coreVoltageActual":	1131,
	"frequency":	525,
	"ssid":	"My Home Wi-Fi",
	"macAddr":	"66:F3:55:23:1A:BD",
	"hostname":	"BM1370",
	"wifiStatus":	"Connected!",
	"wifiRSSI":	-35,
	"apEnabled":	0,
	"sharesAccepted":	20205,
	"sharesRejected":	24,
	"sharesRejectedReasons":	[{
			"message":	"Above target",
			"count":	20
		}, {
			"message":	"Stale",
			"count":	4
		}],
	"uptimeSeconds":	258691,
	"smallCoreCount":	2040,
	"ASICModel":	"BM1370",
	"stratumURL":	"192.168.1.11",
	"stratumPort":	3333,
	"stratumUser":	"1PKN98VN2z5gwSGZvGKS2bj8aADZBkyhkZ",
	"stratumSuggestedDifficulty":	1000,
	"stratumExtranonceSubscribe":	0,
	"fallbackStratumURL":	"solo.ckpool.org",
	"fallbackStratumPort":	3333,
	"fallbackStratumUser":	"1PKN98VN2z5gwSGZvGKS2bj8aADZBkyhkZ",
	"fallbackStratumSuggestedDifficulty":	1000,
	"fallbackStratumExtranonceSubscribe":	0,
	"responseTime":	22.331,
	"version":	"v2.10.1",
	"axeOSVersion":	"v2.10.1",
	"idfVersion":	"v5.5",
	"boardVersion":	"601",
	"runningPartition":	"ota_1",
	"overheat_mode":	0,
	"overclockEnabled":	0,
	"display":	"SSD1306 (128x32)",
	"rotation":	0,
	"invertscreen":	0,
	"displayTimeout":	-1,
	"autofanspeed":	1,
	"fanspeed":	100,
	"minFanSpeed":	25,
	"temptarget":	60,
	"fanrpm":	5471,
	"statsFrequency":	0
}"#;

        let info: SystemInfo = serde_json::from_str(json).unwrap();
        assert_eq!(
            info,
            SystemInfo {
                asic_model: String::from("BM1370"),
                version: String::from("v2.10.1"),
                axe_os_version: String::from("v2.10.1"),
                board_version: String::from("601"),
                stratum_url: String::from("192.168.1.11"),
                stratum_port: 3333,
                stratum_user: String::from("1PKN98VN2z5gwSGZvGKS2bj8aADZBkyhkZ"),
                is_using_fallback_stratum: false,
                fallback_stratum_url: String::from("solo.ckpool.org"),
                fallback_stratum_port: 3333,
                fallback_stratum_user: String::from("1PKN98VN2z5gwSGZvGKS2bj8aADZBkyhkZ"),
                stratum_latency: Some(22.331),
                hashrate: 1184.8093224631666,
                expected_hashrate: 1071.0,
                best_diff: 2.03e9 as u64,
                best_session_diff: 138.17e6 as u64,
                pool_difficulty: 1000,
                shares_accepted: 20205,
                shares_rejected: 24,
                shares_rejected_reasons: vec![
                    ShareRejectedReason {
                        message: String::from("Above target"),
                        count: 20
                    },
                    ShareRejectedReason {
                        message: String::from("Stale"),
                        count: 4
                    }
                ],
                block_found: false,
                auto_fan_speed: true,
                fan_rpm: 5471,
                fan_speed: 100.0,
                frequency: 525,
                hostname: String::from("BM1370"),
                ssid: String::from("My Home Wi-Fi"),
                wifi_rssi: -35,
                wifi_status: String::from("Connected!"),
                mac_addr: String::from("66:F3:55:23:1A:BD"),
                ap_enabled: false,
                is_psram_available: true,
                overclock_enabled: false,
                overheat_protection_mode: false,
                temp: 64.125,
                temp_target: 60.0,
                uptime_seconds: 258691,
            }
        )
    }
}
