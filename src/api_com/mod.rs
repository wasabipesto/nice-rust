//! A module with server connection utilities.

use super::*;

/// A field returned from the server. Used as input for processing.
#[derive(Debug, Deserialize)]
pub struct FieldClaim {
    pub id: u32,
    pub base: u32,
    #[serde(deserialize_with = "deserialize_stringified_number")]
    pub search_start: Natural,
    #[serde(deserialize_with = "deserialize_stringified_number")]
    pub search_end: Natural,
    #[serde(deserialize_with = "deserialize_stringified_number")]
    pub search_range: Natural,
}

/// The compiled results sent to the server after processing. Options for both modes.
#[derive(Debug, Serialize)]
pub struct FieldSubmit<'me> {
    pub id: u32,
    pub username: &'me str,
    pub client_version: &'static str,
    pub unique_count: Option<HashMap<u32, u32>>,
    pub near_misses: Option<HashMap<Natural, u32>>,
    pub nice_list: Option<Vec<Natural>>,
}

/// Deserialize BigInts from the server that are wrapped in quotes.
fn deserialize_stringified_number<'de, D>(deserializer: D) -> Result<Natural, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.trim_matches('"')
        .parse()
        .map_err(|_| serde::de::Error::custom(format!("invalid number: {}", s)))
}

/// Generate a field offline for benchmark testing.
pub fn get_field_benchmark(max_range: Option<u128>) -> FieldClaim {
    let range: u128 = max_range.unwrap_or(100000);
    return FieldClaim {
        id: 0,
        base: 40,
        search_start: Natural::from(916284264916 as u128),
        search_end: Natural::from(6553600000000 as u128)
            .min(Natural::from(916284264916 + range as u128)),
        search_range: Natural::from(range),
    };
}

/// Build a field request url.
fn get_claim_url(
    mode: &Mode,
    api_base: &str,
    username: &str,
    base: &Option<u32>,
    max_range: &Option<u128>,
    field: &Option<u128>,
) -> String {
    let mut query_url = api_base.to_owned();
    query_url += &match mode {
        Mode::Detailed => "/claim/detailed",
        Mode::Niceonly => "/claim/niceonly",
    };
    query_url += &("?username=".to_owned() + &username.to_string());
    if let Some(base_val) = base {
        query_url += &("&base=".to_owned() + &base_val.to_string());
    }
    if let Some(max_range_val) = max_range {
        query_url += &("&max_range=".to_owned() + &max_range_val.to_string());
    }
    if let Some(field_id_val) = field {
        query_url += &("&field=".to_owned() + &field_id_val.to_string());
    }
    query_url += &("&max_base=".to_owned() + &MAX_SUPPORTED_BASE.to_string());
    query_url
}

/// Request a field from the server. Supplies CLI options as query strings.
pub fn get_field(
    mode: &Mode,
    api_base: &str,
    username: &str,
    base: &Option<u32>,
    max_range: &Option<u128>,
    field: &Option<u128>,
) -> FieldClaim {
    let response = reqwest::blocking::get(&get_claim_url(
        mode, api_base, username, base, max_range, field,
    ))
    .unwrap_or_else(|e| panic!("Error: {}", e));
    match response.json::<FieldClaim>() {
        Ok(claim_data) => claim_data,
        Err(e) => panic!("Error: {}", e),
    }
}

/// Submit field results to the server. Panic if there is an error.
pub fn submit_field(mode: &Mode, api_base: &str, submit_data: FieldSubmit) {
    let url = match mode {
        Mode::Detailed => format!("{}/submit/detailed", api_base),
        Mode::Niceonly => format!("{}/submit/niceonly", api_base),
    };

    let response = reqwest::blocking::Client::new()
        .post(&url)
        .json(&submit_data)
        .send();
    match response {
        Ok(response) => {
            if response.status().is_success() {
                return; // 👍
            }
            match response.text() {
                Ok(msg) => panic!("Server returned an error: {}", msg),
                Err(_) => panic!("Server returned an error."),
            }
        }
        Err(e) => panic!("Network error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_claim_url() {
        assert_eq!(
            get_claim_url(
                &Mode::Detailed,
                "https://nicenumbers.net/api",
                "anonymous",
                &None,
                &None,
                &None
            ),
            "https://nicenumbers.net/api/claim/detailed?username=anonymous&max_base=".to_string()
                + &MAX_SUPPORTED_BASE.to_string()
        );
        assert_eq!(
            get_claim_url(
                &Mode::Niceonly,
                "https://nicenumbers.net/api",
                "anonymous",
                &None,
                &None,
                &None
            ),
            "https://nicenumbers.net/api/claim/niceonly?username=anonymous&max_base=".to_string()
                + &MAX_SUPPORTED_BASE.to_string()
        );
        assert_eq!(
            get_claim_url(
                &Mode::Detailed,
                "https://nicenumbers.net/api",
                "anonymous",
                &Some(120),
                &None,
                &None
            ),
            "https://nicenumbers.net/api/claim/detailed?username=anonymous&base=120&max_base="
                .to_string()
                + &MAX_SUPPORTED_BASE.to_string()
        );
        assert_eq!(
            get_claim_url(
                &Mode::Detailed,
                "https://nicenumbers.net/api",
                "anonymous",
                &None,
                &Some(1000000),
                &None
            ),
            "https://nicenumbers.net/api/claim/detailed?username=anonymous&max_range=1000000&max_base=".to_string()
                + &MAX_SUPPORTED_BASE.to_string()
        );
        assert_eq!(
            get_claim_url(
                &Mode::Detailed,
                "https://nicenumbers.net/api",
                "anonymous",
                &None,
                &None,
                &Some(123456)
            ),
            "https://nicenumbers.net/api/claim/detailed?username=anonymous&field=123456&max_base="
                .to_string()
                + &MAX_SUPPORTED_BASE.to_string()
        );
        assert_eq!(
            get_claim_url(
                &Mode::Niceonly,
                "https://nicenumbers.net/api",
                "anonymous",
                &Some(120),
                &Some(1000000),
                &Some(123456)
            ),
            "https://nicenumbers.net/api/claim/niceonly?username=anonymous".to_string()
                + &"&base=120&max_range=1000000&field=123456&max_base=".to_string()
                + &MAX_SUPPORTED_BASE.to_string()
        );
    }
}
