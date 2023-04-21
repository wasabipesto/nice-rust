//! A module with server connection utilities.

use super::*;

/// Deserialize BigInts from the server that are wrapped in quotes.
pub fn deserialize_stringified_number<'de, D>(deserializer: D) -> Result<Natural, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.trim_matches('"')
        .parse()
        .map_err(|_| serde::de::Error::custom(format!("invalid number: {}", s)))
}

/// Generate a field offline for benchmark testing.
pub fn get_field_benchmark(base: Option<u32>, range: Option<u32>) -> FieldClaim {
    let base = base.unwrap_or(40);
    if base % 5 == 1 {
        panic!("Invalid base {}! Base cannot be 1 mod 5.", base)
    }
    let (search_start, range_end) = get_base_range(base);
    let range = Natural::from(range.unwrap_or(100000));
    let search_end = range_end.min(&search_start + &range);
    let search_range = &search_end - &search_start;
    return FieldClaim {
        id: 0,
        username: "benchmark".to_owned(),
        base,
        search_start,
        search_end,
        search_range,
    };
}

/// Build a field request url.
fn get_claim_url(
    mode: &Mode,
    api_base: &str,
    username: &str,
    base: &Option<u32>,
    max_range: &Option<u32>,
    field: &Option<u32>,
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
    max_range: &Option<u32>,
    field: &Option<u32>,
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
