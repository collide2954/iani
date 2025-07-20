use anyhow::Result;
use extendr_api::prelude::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use url::Url;

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

#[derive(Debug, Default)]
pub struct GwasFilter {
    pub p_value_range: Option<(String, String)>,
    pub bp_location_range: Option<(i64, i64)>,
    pub study: Option<String>,
    pub trait_id: Option<String>,
    pub reveal: Option<String>,
    pub start: Option<i32>,
    pub size: Option<i32>,
}

impl GwasFilter {
    pub fn to_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();

        if let Some((lower, upper)) = &self.p_value_range {
            params.insert("p_lower".to_string(), lower.clone());
            params.insert("p_upper".to_string(), upper.clone());
        }

        if let Some((lower, upper)) = &self.bp_location_range {
            params.insert("bp_lower".to_string(), lower.to_string());
            params.insert("bp_upper".to_string(), upper.to_string());
        }

        if let Some(study) = &self.study {
            params.insert("study_accession".to_string(), study.clone());
        }

        if let Some(trait_id) = &self.trait_id {
            params.insert("trait".to_string(), trait_id.clone());
        }

        if let Some(reveal) = &self.reveal {
            params.insert("reveal".to_string(), reveal.clone());
        }

        if let Some(start) = self.start {
            params.insert("start".to_string(), start.to_string());
        }

        if let Some(size) = self.size {
            params.insert("size".to_string(), size.to_string());
        }

        params
    }
}

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

    fn build_url(&self, endpoint: &str, params: &HashMap<String, String>) -> Result<Url> {
        let mut url = Url::parse(&format!(
            "{}/{}",
            self.base_url,
            endpoint.trim_start_matches('/')
        ))?;
        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }
        Ok(url)
    }

    fn check_json_response(
        &self,
        response: reqwest::blocking::Response,
    ) -> Result<reqwest::blocking::Response> {
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .unwrap_or_else(|_| "Unable to read response body".to_string());
            return Err(anyhow::anyhow!("HTTP {}: {}", status, text));
        }

        if let Some(content_type) = response.headers().get("content-type") {
            if let Ok(ct_str) = content_type.to_str() {
                if !ct_str.contains("application/json") {
                    return Err(anyhow::anyhow!("Expected JSON response, got: {}", ct_str));
                }
            }
        }

        Ok(response)
    }

    pub fn get_associations(
        &self,
        params: HashMap<String, String>,
    ) -> Result<HalResponse<HashMap<String, Association>>> {
        let url = self.build_url("/associations", &params)?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    pub fn get_variant_associations(
        &self,
        variant_id: &str,
        params: HashMap<String, String>,
    ) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/associations/{variant_id}");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    pub fn get_chromosomes(&self) -> Result<HalResponse<Vec<Chromosome>>> {
        let url = self.build_url("/chromosomes", &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<Vec<Chromosome>> = response.json()?;
        Ok(data)
    }

    pub fn get_chromosome(&self, chromosome: &str) -> Result<Chromosome> {
        let endpoint = format!("/chromosomes/{chromosome}");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: Chromosome = response.json()?;
        Ok(data)
    }

    pub fn get_chromosome_associations(
        &self,
        chromosome: &str,
        params: HashMap<String, String>,
    ) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/chromosomes/{chromosome}/associations");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    pub fn get_chromosome_variant_associations(
        &self,
        chromosome: &str,
        variant_id: &str,
        params: HashMap<String, String>,
    ) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/chromosomes/{chromosome}/associations/{variant_id}");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    pub fn get_studies(
        &self,
        params: HashMap<String, String>,
    ) -> Result<HalResponse<Vec<Vec<Study>>>> {
        let url = self.build_url("/studies", &params)?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<Vec<Vec<Study>>> = response.json()?;
        Ok(data)
    }

    pub fn get_study(&self, study_accession: &str) -> Result<Study> {
        let endpoint = format!("/studies/{study_accession}");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: Study = response.json()?;
        Ok(data)
    }

    pub fn get_study_associations(
        &self,
        study_accession: &str,
        params: HashMap<String, String>,
    ) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/studies/{study_accession}/associations");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    pub fn get_traits(&self, params: HashMap<String, String>) -> Result<HalResponse<Vec<Trait>>> {
        let url = self.build_url("/traits", &params)?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<Vec<Trait>> = response.json()?;
        Ok(data)
    }

    pub fn get_trait(&self, trait_id: &str) -> Result<Trait> {
        let endpoint = format!("/traits/{trait_id}");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: Trait = response.json()?;
        Ok(data)
    }

    pub fn get_trait_associations(
        &self,
        trait_id: &str,
        params: HashMap<String, String>,
    ) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/traits/{trait_id}/associations");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    pub fn get_trait_studies(
        &self,
        trait_id: &str,
        params: HashMap<String, String>,
    ) -> Result<HalResponse<Vec<Study>>> {
        let endpoint = format!("/traits/{trait_id}/studies");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<Vec<Study>> = response.json()?;
        Ok(data)
    }

    pub fn get_trait_study(&self, trait_id: &str, study_accession: &str) -> Result<Study> {
        let endpoint = format!("/traits/{trait_id}/studies/{study_accession}");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: Study = response.json()?;
        Ok(data)
    }

    pub fn get_trait_study_associations(
        &self,
        trait_id: &str,
        study_accession: &str,
        params: HashMap<String, String>,
    ) -> Result<HalResponse<HashMap<String, Association>>> {
        let endpoint = format!("/traits/{trait_id}/studies/{study_accession}/associations");
        let url = self.build_url(&endpoint, &params)?;
        let response = self.client.get(url).send()?;
        let response = self.check_json_response(response)?;
        let data: HalResponse<HashMap<String, Association>> = response.json()?;
        Ok(data)
    }

    pub fn get_study_summary_stats_files(
        &self,
        study_accession: &str,
    ) -> Result<HalResponse<Vec<SummaryStatsFile>>> {
        let endpoint = format!("/studies/{study_accession}/summary-statistics");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;

        let response = self.check_json_response(response)?;
        let data: HalResponse<Vec<SummaryStatsFile>> = response.json()?;
        Ok(data)
    }

    pub fn get_trait_summary_stats_files(
        &self,
        trait_id: &str,
    ) -> Result<HalResponse<Vec<SummaryStatsFile>>> {
        let endpoint = format!("/traits/{trait_id}/summary-statistics");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;

        let response = self.check_json_response(response)?;
        let data: HalResponse<Vec<SummaryStatsFile>> = response.json()?;
        Ok(data)
    }

    pub fn get_trait_study_summary_stats_files(
        &self,
        trait_id: &str,
        study_accession: &str,
    ) -> Result<HalResponse<Vec<SummaryStatsFile>>> {
        let endpoint = format!("/traits/{trait_id}/studies/{study_accession}/summary-statistics");
        let url = self.build_url(&endpoint, &HashMap::new())?;
        let response = self.client.get(url).send()?;

        let response = self.check_json_response(response)?;
        let data: HalResponse<Vec<SummaryStatsFile>> = response.json()?;
        Ok(data)
    }

    pub fn download_summary_stats_file(&self, file_url: &str, output_path: &str) -> Result<String> {
        let mut response = self.client.get(file_url).send()?;
        if let Some(parent) = Path::new(output_path).parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(output_path)?;
        std::io::copy(&mut response, &mut file)?;
        Ok(output_path.to_string())
    }

    pub fn get_entity(
        &self,
        entity_type: &str,
        id: Option<&str>,
        filter: &GwasFilter,
    ) -> Result<String> {
        let params = filter.to_params();

        match entity_type {
            "chromosomes" => {
                if let Some(chromosome_id) = id {
                    match self.get_chromosome(chromosome_id) {
                        Ok(data) => Ok(serde_json::to_string_pretty(&data)?),
                        Err(e) => Err(e),
                    }
                } else {
                    match self.get_chromosomes() {
                        Ok(data) => Ok(serde_json::to_string_pretty(&data)?),
                        Err(e) => Err(e),
                    }
                }
            }
            "studies" => {
                if let Some(study_id) = id {
                    match self.get_study(study_id) {
                        Ok(data) => Ok(serde_json::to_string_pretty(&data)?),
                        Err(e) => Err(e),
                    }
                } else {
                    match self.get_studies(params) {
                        Ok(data) => Ok(serde_json::to_string_pretty(&data)?),
                        Err(e) => Err(e),
                    }
                }
            }
            "traits" => {
                if let Some(trait_id) = id {
                    match self.get_trait(trait_id) {
                        Ok(data) => Ok(serde_json::to_string_pretty(&data)?),
                        Err(e) => Err(e),
                    }
                } else {
                    match self.get_traits(params) {
                        Ok(data) => Ok(serde_json::to_string_pretty(&data)?),
                        Err(e) => Err(e),
                    }
                }
            }
            _ => Err(anyhow::anyhow!("Invalid entity type: {}", entity_type)),
        }
    }

    pub fn get_unified_associations(
        &self,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
        filter: &GwasFilter,
    ) -> Result<String> {
        let params = filter.to_params();

        let result = match (entity_type, entity_id) {
            (None, None) => self.get_associations(params),
            (Some("variant"), Some(variant_id)) => {
                self.get_variant_associations(variant_id, params)
            }
            (Some("chromosome"), Some(chromosome_id)) => {
                self.get_chromosome_associations(chromosome_id, params)
            }
            (Some("study"), Some(study_id)) => self.get_study_associations(study_id, params),
            (Some("trait"), Some(trait_id)) => self.get_trait_associations(trait_id, params),
            _ => return Err(anyhow::anyhow!("Invalid entity type or missing ID")),
        };

        match result {
            Ok(data) => Ok(serde_json::to_string_pretty(&data)?),
            Err(e) => Err(e),
        }
    }

    pub fn list_files(
        &self,
        entity_type: &str,
        entity_id: &str,
        secondary_id: Option<&str>,
    ) -> Result<String> {
        let result = match (entity_type, secondary_id) {
            ("study", None) => self.get_study_summary_stats_files(entity_id),
            ("trait", None) => self.get_trait_summary_stats_files(entity_id),
            ("trait", Some(study_id)) => {
                self.get_trait_study_summary_stats_files(entity_id, study_id)
            }
            _ => return Err(anyhow::anyhow!("Invalid file entity type or parameters")),
        };

        match result {
            Ok(data) => Ok(serde_json::to_string_pretty(&data)?),
            Err(e) => Err(e),
        }
    }
}

/// Unified get function for entities (chromosomes, studies, traits)
/// @param entity_type Type of entity: "chromosomes", "studies", or "traits"
/// @param id Optional entity ID for specific entity
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @export
#[extendr]
fn gwas_get(
    entity_type: String,
    id: Option<String>,
    start: Option<i32>,
    size: Option<i32>,
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let filter = GwasFilter {
        start,
        size,
        ..Default::default()
    };

    match client.get_entity(&entity_type, id.as_deref(), &filter) {
        Ok(data) => data,
        Err(e) => format!("Error fetching {entity_type}: {e}"),
    }
}

/// Unified associations function with filtering
/// @param entity_type Optional entity type: "variant", "chromosome", "study", "trait"
/// @param entity_id Optional entity ID
/// @param p_value_min Optional minimum p-value threshold
/// @param p_value_max Optional maximum p-value threshold
/// @param bp_min Optional minimum base pair location
/// @param bp_max Optional maximum base pair location
/// @param study Optional study accession filter
/// @param trait_id Optional trait ID filter
/// @param reveal Optional reveal mode ("raw" or "all")
/// @param start Offset number (default: 0)
/// @param size Number of items returned (default: 20)
/// @export
#[allow(clippy::too_many_arguments)]
#[extendr]
fn gwas_associations(
    entity_type: Option<String>,
    entity_id: Option<String>,
    p_value_min: Option<String>,
    p_value_max: Option<String>,
    bp_min: Option<i64>,
    bp_max: Option<i64>,
    study: Option<String>,
    trait_id: Option<String>,
    reveal: Option<String>,
    start: Option<i32>,
    size: Option<i32>,
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    let p_value_range = match (p_value_min, p_value_max) {
        (Some(min), Some(max)) => Some((min, max)),
        (Some(min), None) => Some((min, "1.0".to_string())),
        (None, Some(max)) => Some(("0.0".to_string(), max)),
        (None, None) => None,
    };

    let bp_location_range = match (bp_min, bp_max) {
        (Some(min), Some(max)) => Some((min, max)),
        _ => None,
    };

    let filter = GwasFilter {
        p_value_range,
        bp_location_range,
        study,
        trait_id,
        reveal,
        start,
        size,
    };

    match client.get_unified_associations(entity_type.as_deref(), entity_id.as_deref(), &filter) {
        Ok(data) => data,
        Err(e) => format!("Error fetching associations: {e}"),
    }
}

/// Unified file operations (list and download)
/// @param operation Operation type: "list" or "download"
/// @param entity_type Entity type: "study" or "trait"
/// @param entity_id Primary entity ID
/// @param secondary_id Optional secondary ID (for trait-study combinations)
/// @param file_urls Optional vector of file URLs (for download)
/// @param output_paths Optional vector of output paths (for download)
/// @param max_concurrent Optional max concurrent downloads (default: 4)
/// @export
#[allow(clippy::too_many_arguments)]
#[extendr]
fn gwas_files(
    operation: String,
    entity_type: String,
    entity_id: String,
    secondary_id: Option<String>,
    file_urls: Option<Vec<String>>,
    output_paths: Option<Vec<String>>,
    max_concurrent: Option<usize>,
) -> String {
    let client = match GwasClient::new() {
        Ok(c) => c,
        Err(e) => return format!("Error creating client: {e}"),
    };

    match operation.as_str() {
        "list" => match client.list_files(&entity_type, &entity_id, secondary_id.as_deref()) {
            Ok(data) => data,
            Err(e) => format!("Error listing files: {e}"),
        },
        "download" => {
            match (file_urls, output_paths) {
                (Some(urls), Some(paths)) => {
                    if urls.len() != paths.len() {
                        return "Error: file_urls and output_paths must have the same length."
                            .to_string();
                    }

                    let max_concurrent = max_concurrent.unwrap_or(4);

                    use rayon::prelude::*;
                    use rayon::ThreadPoolBuilder;

                    // Build a custom thread pool with the desired number of threads
                    let pool = match ThreadPoolBuilder::new().num_threads(max_concurrent).build() {
                        Ok(p) => p,
                        Err(e) => return format!("Error creating thread pool: {e}"),
                    };

                    let results = pool.install(|| {
                        urls.par_iter()
                            .zip(paths.par_iter())
                            .map(|(url, path)| {
                                match client.download_summary_stats_file(url, path) {
                                    Ok(p) => Ok(format!("Downloaded: {p}")),
                                    Err(e) => Err(format!("Failed to download {url}: {e}")),
                                }
                            })
                            .collect::<Vec<_>>()
                    });

                    // Format results
                    let mut success_count = 0;
                    let mut error_messages = Vec::new();

                    for result in results {
                        match result {
                            Ok(_) => success_count += 1,
                            Err(err) => error_messages.push(err),
                        }
                    }

                    format!(
                        "Downloaded {} of {} files successfully.\n{}",
                        success_count,
                        urls.len(),
                        error_messages.join("\n")
                    )
                }
                _ => {
                    "Error: file_urls and output_paths required for download operation".to_string()
                }
            }
        }
        _ => format!("Invalid operation: {operation}. Use 'list' or 'download'"),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod iani;
    fn gwas_get;
    fn gwas_associations;
    fn gwas_files;
}
