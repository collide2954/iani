use extendr_api::prelude::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use url::Url;
use std::fs;
use std::path::Path;

// Data structures for GWAS API responses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Association {
    pub variant_id: Option<String>,
    pub chromosome: Option<i32>,
    pub base_pair_location: Option<i64>,
    pub study_accession: Option<String>,
    #[serde(rename = "trait")]
    pub trait_ids: Option<Vec<String>>,
    pub p_value: Option<f64>,
    pub code: Option<i32>,
    pub effect_allele: Option<String>,
    pub other_allele: Option<String>,
    pub effect_allele_frequency: Option<f64>,
    pub odds_ratio: Option<f64>,
    pub ci_lower: Option<f64>,
    pub ci_upper: Option<f64>,
    pub beta: Option<f64>,
    pub se: Option<f64>,
    #[serde(rename = "_links")]
    pub links: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    pub href: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HalResponse<T> {
    #[serde(rename = "_embedded")]
    pub embedded: Option<HashMap<String, T>>,
    #[serde(rename = "_links")]
    pub links: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chromosome {
    pub chromosome: String,
    #[serde(rename = "_links")]
    pub links: Option<HashMap<String, Link>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Study {
    pub study_accession: String,
    #[serde(rename = "_links")]
    pub links: Option<HashMap<String, Link>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trait {
    #[serde(rename = "trait")]
    pub trait_name: String,
    #[serde(rename = "_links")]
    pub links: Option<HashMap<String, Link>>,
}

// Data structure for summary statistics files
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SummaryStatsFile {
    pub study_accession: String,
    pub trait_id: Option<String>,
    pub file_path: String,
    pub file_size: Option<i64>,
    pub file_type: Option<String>,
    pub download_url: Option<String>,
    #[serde(rename = "_links")]
    pub links: Option<HashMap<String, Link>>,
}

// GWAS API Client
#[derive(Debug, Clone)]
pub struct GwasClient {
    client: Client,
    base_url: String,
}

impl GwasClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            base_url: "https://www.ebi.ac.uk/gwas/summary-statistics/api".to_string(),
        })
    }

    pub fn with_base_url(base_url: String) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            base_url,
        })
    }

    // Build URL with query parameters
    fn build_url(&self, endpoint: &str, params: &HashMap<String, String>) -> Result<Url> {
        let mut url = Url::parse(&format!("{}/{}", self.base_url, endpoint.trim_start_matches('/')))?;
        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }
        Ok(url)
    }

    // Get all associations
    pub fn get_associations(&self, params: HashMap<String, String>) -> Result<HalResponse<HashMap<String, Association>>> {
        let url = self.build_url("/associations", &params)?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    // Get associations for a specific variant
    pub fn get_variant_associations(&self, variant_id: &str, params: HashMap<String, String>) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/associations/{variant_id}");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    // Get all chromosomes
    pub fn get_chromosomes(&self) -> Result<HalResponse<Vec<Chromosome>>> {
        let url = self.build_url("/chromosomes", &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<Vec<Chromosome>> = response.json()?;
        Ok(data)
    }

    // Get specific chromosome
    pub fn get_chromosome(&self, chromosome: &str) -> Result<Chromosome> {
        let endpoint = format!("/chromosomes/{chromosome}");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let data: Chromosome = response.json()?;
        Ok(data)
    }

    // Get associations for a chromosome
    pub fn get_chromosome_associations(&self, chromosome: &str, params: HashMap<String, String>) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/chromosomes/{chromosome}/associations");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    // Get variant associations on specific chromosome
    pub fn get_chromosome_variant_associations(&self, chromosome: &str, variant_id: &str, params: HashMap<String, String>) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/chromosomes/{chromosome}/associations/{variant_id}");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    // Get all studies
    pub fn get_studies(&self, params: HashMap<String, String>) -> Result<HalResponse<Vec<Vec<Study>>>> {
        let url = self.build_url("/studies", &params)?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<Vec<Vec<Study>>> = response.json()?;
        Ok(data)
    }

    // Get specific study
    pub fn get_study(&self, study_accession: &str) -> Result<Study> {
        let endpoint = format!("/studies/{study_accession}");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let data: Study = response.json()?;
        Ok(data)
    }

    // Get study associations
    pub fn get_study_associations(&self, study_accession: &str, params: HashMap<String, String>) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/studies/{study_accession}/associations");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    // Get all traits
    pub fn get_traits(&self, params: HashMap<String, String>) -> Result<HalResponse<Vec<Trait>>> {
        let url = self.build_url("/traits", &params)?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<Vec<Trait>> = response.json()?;
        Ok(data)
    }

    // Get specific trait
    pub fn get_trait(&self, trait_id: &str) -> Result<Trait> {
        let endpoint = format!("/traits/{trait_id}");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let data: Trait = response.json()?;
        Ok(data)
    }

    // Get trait associations
    pub fn get_trait_associations(&self, trait_id: &str, params: HashMap<String, String>) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/traits/{trait_id}/associations");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    // Get trait studies
    pub fn get_trait_studies(&self, trait_id: &str, params: HashMap<String, String>) -> Result<HalResponse<Vec<Study>>> {
        let endpoint = format!("/traits/{trait_id}/studies");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<Vec<Study>> = response.json()?;
        Ok(data)
    }

    // Get trait study
    pub fn get_trait_study(&self, trait_id: &str, study_accession: &str) -> Result<Study> {
        let endpoint = format!("/traits/{trait_id}/studies/{study_accession}");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let data: Study = response.json()?;
        Ok(data)
    }

    // Get trait study associations
    pub fn get_trait_study_associations(&self, trait_id: &str, study_accession: &str, params: HashMap<String, String>) -> Result<HalResponse<HashMap<String, Association>>> {
            let endpoint = format!("/traits/{trait_id}/studies/{study_accession}/associations");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    // Get summary statistics files for a study
    pub fn get_study_summary_stats_files(&self, study_accession: &str) -> Result<HalResponse<Vec<SummaryStatsFile>>> {
        let endpoint = format!("/studies/{study_accession}/summary-statistics");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<Vec<SummaryStatsFile>> = response.json()?;
        Ok(data)
    }

    // Get summary statistics files for a trait
    pub fn get_trait_summary_stats_files(&self, trait_id: &str) -> Result<HalResponse<Vec<SummaryStatsFile>>> {
        let endpoint = format!("/traits/{trait_id}/summary-statistics");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<Vec<SummaryStatsFile>> = response.json()?;
        Ok(data)
    }

    // Get summary statistics files for a trait-study combination
    pub fn get_trait_study_summary_stats_files(&self, trait_id: &str, study_accession: &str) -> Result<HalResponse<Vec<SummaryStatsFile>>> {
        let endpoint = format!("/traits/{trait_id}/studies/{study_accession}/summary-statistics");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let data: HalResponse<Vec<SummaryStatsFile>> = response.json()?;
        Ok(data)
    }

    // Download a summary statistics file
    pub fn download_summary_stats_file(&self, file_url: &str, output_path: &str) -> Result<String> {
        let mut response = self.client.get(file_url).send()?;
        // Create directory if it doesn't exist
        if let Some(parent) = Path::new(output_path).parent() {
            fs::create_dir_all(parent)?;
        }
        // Write file
        let mut file = fs::File::create(output_path)?;
        std::io::copy(&mut response, &mut file)?;
        Ok(output_path.to_string())
    }
}

// R interface functions
/// Create a new GWAS API client
/// @export
#[extendr]
fn gwas_client_new() -> String {
    match GwasClient::new() {
        Ok(_) => "GWAS client created successfully".to_string(),
        Err(e) => format!("Error creating GWAS client: {e}"),
    }
}

/// Get all associations with optional parameters
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @param reveal Show raw/all data ("raw" or "all")
/// @param p_lower Lower p-value threshold
/// @param p_upper Upper p-value threshold
/// @export
#[extendr]
fn gwas_get_associations(
    start: Option<i32>,
    size: Option<i32>,
    reveal: Option<String>,
    p_lower: Option<String>,
    p_upper: Option<String>
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let mut params = HashMap::new();
    if let Some(s) = start { params.insert("start".to_string(), s.to_string()); }
    if let Some(s) = size { params.insert("size".to_string(), s.to_string()); }
    if let Some(r) = reveal { params.insert("reveal".to_string(), r); }
    if let Some(p) = p_lower { params.insert("p_lower".to_string(), p); }
    if let Some(p) = p_upper { params.insert("p_upper".to_string(), p); }

    match client.get_associations(params) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching associations: {e}"),
    }
}

/// Get associations for a specific variant
/// @param variant_id The rsid of the variant
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @param reveal Show raw/all data ("raw" or "all")
/// @param p_lower Lower p-value threshold
/// @param p_upper Upper p-value threshold
/// @param study_accession Specific study accession
/// @export
#[extendr]
fn gwas_get_variant_associations(
    variant_id: String,
    start: Option<i32>,
    size: Option<i32>,
    reveal: Option<String>,
    p_lower: Option<String>,
    p_upper: Option<String>,
    study_accession: Option<String>
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let mut params = HashMap::new();
    if let Some(s) = start { params.insert("start".to_string(), s.to_string()); }
    if let Some(s) = size { params.insert("size".to_string(), s.to_string()); }
    if let Some(r) = reveal { params.insert("reveal".to_string(), r); }
    if let Some(p) = p_lower { params.insert("p_lower".to_string(), p); }
    if let Some(p) = p_upper { params.insert("p_upper".to_string(), p); }
    if let Some(sa) = study_accession { params.insert("study_accession".to_string(), sa); }

    match client.get_variant_associations(&variant_id, params) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching variant associations: {e}"),
    }
}

/// Get all chromosomes
/// @export
#[extendr]
fn gwas_get_chromosomes() -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    match client.get_chromosomes() {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching chromosomes: {e}"),
    }
}

/// Get a specific chromosome
/// @param chromosome Chromosome identifier (1-22, X, Y, MT mapped to 23, 24, 25)
/// @export
#[extendr]
fn gwas_get_chromosome(chromosome: String) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    match client.get_chromosome(&chromosome) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching chromosome: {e}"),
    }
}

/// Get associations for a specific chromosome
/// @param chromosome Chromosome identifier
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @param reveal Show raw/all data ("raw" or "all")
/// @param p_lower Lower p-value threshold
/// @param p_upper Upper p-value threshold
/// @param bp_lower Lower base pair location threshold
/// @param bp_upper Upper base pair location threshold
/// @param study_accession Specific study accession
/// @param trait_name Specific trait ID
/// @export
#[allow(clippy::too_many_arguments)] 
#[extendr]
fn gwas_get_chromosome_associations(
    chromosome: String,
    start: Option<i32>,
    size: Option<i32>,
    reveal: Option<String>,
    p_lower: Option<String>,
    p_upper: Option<String>,
    bp_lower: Option<i64>,
    bp_upper: Option<i64>,
    study_accession: Option<String>,
    trait_name: Option<String>
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let mut params = HashMap::new();
    if let Some(s) = start { params.insert("start".to_string(), s.to_string()); }
    if let Some(s) = size { params.insert("size".to_string(), s.to_string()); }
    if let Some(r) = reveal { params.insert("reveal".to_string(), r); }
    if let Some(p) = p_lower { params.insert("p_lower".to_string(), p); }
    if let Some(p) = p_upper { params.insert("p_upper".to_string(), p); }
    if let Some(bp) = bp_lower { params.insert("bp_lower".to_string(), bp.to_string()); }
    if let Some(bp) = bp_upper { params.insert("bp_upper".to_string(), bp.to_string()); }
    if let Some(sa) = study_accession { params.insert("study_accession".to_string(), sa); }
    if let Some(t) = trait_name { params.insert("trait".to_string(), t); }

    match client.get_chromosome_associations(&chromosome, params) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching chromosome associations: {e}"),
    }
}

/// Get associations for a variant on a specific chromosome
/// @param chromosome Chromosome identifier
/// @param variant_id The rsid of the variant
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @param reveal Show raw/all data ("raw" or "all")
/// @param p_lower Lower p-value threshold
/// @param p_upper Upper p-value threshold
/// @param study_accession Specific study accession
/// @param trait_name Specific trait ID
/// @export
#[allow(clippy::too_many_arguments)] 
#[extendr]
fn gwas_get_chromosome_variant_associations(
    chromosome: String,
    variant_id: String,
    start: Option<i32>,
    size: Option<i32>,
    reveal: Option<String>,
    p_lower: Option<String>,
    p_upper: Option<String>,
    study_accession: Option<String>,
    trait_name: Option<String>
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let mut params = HashMap::new();
    if let Some(s) = start { params.insert("start".to_string(), s.to_string()); }
    if let Some(s) = size { params.insert("size".to_string(), s.to_string()); }
    if let Some(r) = reveal { params.insert("reveal".to_string(), r); }
    if let Some(p) = p_lower { params.insert("p_lower".to_string(), p); }
    if let Some(p) = p_upper { params.insert("p_upper".to_string(), p); }
    if let Some(sa) = study_accession { params.insert("study_accession".to_string(), sa); }
    if let Some(t) = trait_name { params.insert("trait".to_string(), t); }

    match client.get_chromosome_variant_associations(&chromosome, &variant_id, params) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching chromosome variant associations: {e}"),
    }
}

/// Get all studies
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @export
#[extendr]
fn gwas_get_studies(start: Option<i32>, size: Option<i32>) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let mut params = HashMap::new();
    if let Some(s) = start { params.insert("start".to_string(), s.to_string()); }
    if let Some(s) = size { params.insert("size".to_string(), s.to_string()); }

    match client.get_studies(params) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching studies: {e}"),
    }
}

/// Get a specific study
/// @param study_accession Study accession ID
/// @export
#[extendr]
fn gwas_get_study(study_accession: String) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    match client.get_study(&study_accession) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching study: {e}"),
    }
}

/// Get associations for a specific study
/// @param study_accession Study accession ID
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @param reveal Show raw/all data ("raw" or "all")
/// @param p_lower Lower p-value threshold
/// @param p_upper Upper p-value threshold
/// @export
#[extendr]
fn gwas_get_study_associations(
    study_accession: String,
    start: Option<i32>,
    size: Option<i32>,
    reveal: Option<String>,
    p_lower: Option<String>,
    p_upper: Option<String>
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let mut params = HashMap::new();
    if let Some(s) = start { params.insert("start".to_string(), s.to_string()); }
    if let Some(s) = size { params.insert("size".to_string(), s.to_string()); }
    if let Some(r) = reveal { params.insert("reveal".to_string(), r); }
    if let Some(p) = p_lower { params.insert("p_lower".to_string(), p); }
    if let Some(p) = p_upper { params.insert("p_upper".to_string(), p); }

    match client.get_study_associations(&study_accession, params) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching study associations: {e}"),
    }
}

/// Get all traits
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @export
#[extendr]
fn gwas_get_traits(start: Option<i32>, size: Option<i32>) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let mut params = HashMap::new();
    if let Some(s) = start { params.insert("start".to_string(), s.to_string()); }
    if let Some(s) = size { params.insert("size".to_string(), s.to_string()); }

    match client.get_traits(params) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching traits: {e}"),
    }
}

/// Get a specific trait
/// @param trait_id Trait identifier
/// @export
#[extendr]
fn gwas_get_trait(trait_id: String) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    match client.get_trait(&trait_id) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching trait: {e}"),
    }
}

/// Get associations for a specific trait
/// @param trait_id Trait identifier
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @param reveal Show raw/all data ("raw" or "all")
/// @param p_lower Lower p-value threshold
/// @param p_upper Upper p-value threshold
/// @export
#[extendr]
fn gwas_get_trait_associations(
    trait_id: String,
    start: Option<i32>,
    size: Option<i32>,
    reveal: Option<String>,
    p_lower: Option<String>,
    p_upper: Option<String>
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let mut params = HashMap::new();
    if let Some(s) = start { params.insert("start".to_string(), s.to_string()); }
    if let Some(s) = size { params.insert("size".to_string(), s.to_string()); }
    if let Some(r) = reveal { params.insert("reveal".to_string(), r); }
    if let Some(p) = p_lower { params.insert("p_lower".to_string(), p); }
    if let Some(p) = p_upper { params.insert("p_upper".to_string(), p); }

    match client.get_trait_associations(&trait_id, params) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching trait associations: {e}"),
    }
}

/// Get studies for a specific trait
/// @param trait_id Trait identifier
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @export
#[extendr]
fn gwas_get_trait_studies(
    trait_id: String,
    start: Option<i32>,
    size: Option<i32>
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let mut params = HashMap::new();
    if let Some(s) = start { params.insert("start".to_string(), s.to_string()); }
    if let Some(s) = size { params.insert("size".to_string(), s.to_string()); }

    match client.get_trait_studies(&trait_id, params) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching trait studies: {e}"),
    }
}

/// Get a specific study for a trait
/// @param trait_id Trait identifier
/// @param study_accession Study accession ID
/// @export
#[extendr]
fn gwas_get_trait_study(trait_id: String, study_accession: String) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    match client.get_trait_study(&trait_id, &study_accession) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching trait study: {e}"),
    }
}

/// Get associations for a specific trait and study
/// @param trait_id Trait identifier
/// @param study_accession Study accession ID
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @param reveal Show raw/all data ("raw" or "all")
/// @param p_lower Lower p-value threshold
/// @param p_upper Upper p-value threshold
/// @export
#[extendr]
fn gwas_get_trait_study_associations(
    trait_id: String,
    study_accession: String,
    start: Option<i32>,
    size: Option<i32>,
    reveal: Option<String>,
    p_lower: Option<String>,
    p_upper: Option<String>
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let mut params = HashMap::new();
    if let Some(s) = start { params.insert("start".to_string(), s.to_string()); }
    if let Some(s) = size { params.insert("size".to_string(), s.to_string()); }
    if let Some(r) = reveal { params.insert("reveal".to_string(), r); }
    if let Some(p) = p_lower { params.insert("p_lower".to_string(), p); }
    if let Some(p) = p_upper { params.insert("p_upper".to_string(), p); }

    match client.get_trait_study_associations(&trait_id, &study_accession, params) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching trait study associations: {e}"),
    }
}

/// List summary statistics files for a study
/// @param study_accession Study accession ID
/// @export
#[extendr]
fn gwas_list_summary_stats_files(study_accession: String) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };
    match client.get_study_summary_stats_files(&study_accession) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching summary statistics files: {e}"),
    }
}

/// Download multiple summary statistics files in parallel
/// @param file_urls Vector of file URLs to download
/// @param output_paths Vector of output paths (must match length of file_urls)
/// @param max_concurrent Maximum number of concurrent downloads (default:4
/// @export
#[extendr]
fn gwas_download_summary_stats_files(file_urls: Vec<String>, output_paths: Vec<String>, max_concurrent: Option<usize>) -> String {
    if file_urls.len() != output_paths.len() {
        return "Error: file_urls and output_paths must have the same length.".to_string();
    }
    
    let max_concurrent = max_concurrent.unwrap_or(4);
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };
    
    // Use rayon for parallel processing
    use rayon::prelude::*;
    
    let results: Vec<Result<String, String>> = file_urls
        .par_iter()
        .zip(output_paths.par_iter())
        .map(|(url, path)| {
            match client.download_summary_stats_file(url, path) {
                Ok(p) => Ok(format!("Downloaded: {p}")),
                Err(e) => Err(format!("Failed to download {url}: {e}"))
            }
        })
        .collect();
    
    // Format results
    let mut success_count = 0;
    let mut error_messages = Vec::new();
    
    for result in results {
        match result {
            Ok(msg) => success_count += 1,
            Err(err) => error_messages.push(err)
        }
    }
    
    format!("Downloaded {} of {} files successfully.\n{}", 
            success_count, 
            file_urls.len(), 
            error_messages.join("\n"))
}

/// List summary statistics files for a trait
/// @param trait_id Trait identifier
/// @export
#[extendr]
fn gwas_list_trait_summary_stats_files(trait_id: String) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };
    match client.get_trait_summary_stats_files(&trait_id) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching trait summary statistics files: {e}"),
    }
}

/// List summary statistics files for a trait-study combination
/// @param trait_id Trait identifier
/// @param study_accession Study accession ID
/// @export
#[extendr]
fn gwas_list_trait_study_summary_stats_files(trait_id: String, study_accession: String) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };
    match client.get_trait_study_summary_stats_files(&trait_id, &study_accession) {
        Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|e| format!("Serialization error: {e}")),
        Err(e) => format!("Error fetching trait-study summary statistics files: {e}"),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod iani;
    fn gwas_client_new;
    fn gwas_get_associations;
    fn gwas_get_variant_associations;
    fn gwas_get_chromosomes;
    fn gwas_get_chromosome;
    fn gwas_get_chromosome_associations;
    fn gwas_get_chromosome_variant_associations;
    fn gwas_get_studies;
    fn gwas_get_study;
    fn gwas_get_study_associations;
    fn gwas_get_traits;
    fn gwas_get_trait;
    fn gwas_get_trait_associations;
    fn gwas_get_trait_studies;
    fn gwas_get_trait_study;
    fn gwas_get_trait_study_associations;
    fn gwas_list_summary_stats_files;
    fn gwas_list_trait_summary_stats_files;
    fn gwas_list_trait_study_summary_stats_files;
    fn gwas_download_summary_stats_files;
}
