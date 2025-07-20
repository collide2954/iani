#' Create a filter object for GWAS queries
#' @param p_value Optional p-value range as c(min, max)
#' @param bp_location Optional base pair location range as c(min, max) 
#' @param study Optional study accession
#' @param trait Optional trait ID
#' @param reveal Optional reveal mode ("raw" or "all")
#' @param start Optional offset number (default: 0)
#' @param size Optional number of items returned (default: 20)
#' @return A list representing filter parameters
#' @export
gwas_filter <- function(p_value = NULL, bp_location = NULL, study = NULL, trait = NULL, 
                       reveal = NULL, start = NULL, size = NULL) {
  filter <- list()
  
  if (!is.null(p_value) && length(p_value) == 2) {
    filter$p_value_min <- as.character(p_value[1])
    filter$p_value_max <- as.character(p_value[2])
  }
  
  if (!is.null(bp_location) && length(bp_location) == 2) {
    filter$bp_min <- as.integer(bp_location[1])
    filter$bp_max <- as.integer(bp_location[2])
  }
  
  if (!is.null(study)) filter$study <- as.character(study)
  if (!is.null(trait)) filter$trait_id <- as.character(trait)
  if (!is.null(reveal)) filter$reveal <- as.character(reveal)
  if (!is.null(start)) filter$start <- as.integer(start)
  if (!is.null(size)) filter$size <- as.integer(size)
  
  class(filter) <- "gwas_filter"
  filter
}

#' Unified function to get entities (chromosomes, studies, traits)
#' @param entity_type Type of entity: "chromosomes", "studies", or "traits"
#' @param id Optional specific entity ID
#' @param ... Additional filter parameters (start, size)
#' @return JSON response from GWAS API
#' @export
#' @examples
#' \dontrun{
#' # Get all chromosomes
#' gwas_get("chromosomes")
#' 
#' # Get specific chromosome
#' gwas_get("chromosomes", id = "1")
#' 
#' # Get studies with pagination
#' gwas_get("studies", start = 0, size = 10)
#' }
gwas_get <- function(entity_type, id = NULL, start = NULL, size = NULL) {
  .Call(wrap__gwas_get, entity_type, id, start, size)
}

#' Unified function to get associations with flexible filtering
#' @param entity_type Optional entity type: "variant", "chromosome", "study", "trait"
#' @param entity_id Optional entity ID
#' @param filter Optional gwas_filter object or named list
#' @param ... Additional filter parameters
#' @return JSON response from GWAS API
#' @export
#' @examples
#' \dontrun{
#' # Get all associations
#' gwas_associations()
#' 
#' # Get associations for a variant
#' gwas_associations("variant", "rs123456")
#' 
#' # Get associations with p-value filter
#' filter <- gwas_filter(p_value = c(1e-8, 1e-5))
#' gwas_associations(filter = filter)
#' 
#' # Get chromosome associations with multiple filters
#' gwas_associations("chromosome", "1", 
#'                  p_value_min = "1e-8", bp_min = 1000000, bp_max = 2000000)
#' }
gwas_associations <- function(entity_type = NULL, entity_id = NULL, filter = NULL, ...) {
  # Handle filter object or direct parameters
  params <- list(...)
  
  if (!is.null(filter) && inherits(filter, "gwas_filter")) {
    # Merge filter object with additional parameters
    params <- modifyList(filter, params)
  } else if (!is.null(filter) && is.list(filter)) {
    # Merge filter list with additional parameters  
    params <- modifyList(filter, params)
  }
  
  .Call(wrap__gwas_associations,
        entity_type,
        entity_id, 
        params$p_value_min,
        params$p_value_max,
        params$bp_min,
        params$bp_max,
        params$study,
        params$trait_id,
        params$reveal,
        params$start,
        params$size)
}

#' Unified function for file operations (list and download)
#' @param operation Operation type: "list" or "download"
#' @param entity_type Entity type: "study" or "trait"
#' @param entity_id Primary entity ID
#' @param secondary_id Optional secondary ID (for trait-study combinations)
#' @param file_urls Optional vector of file URLs (for download)
#' @param output_paths Optional vector of output paths (for download)
#' @param max_concurrent Optional max concurrent downloads (default: 4)
#' @return JSON response for list operations, status message for downloads
#' @export
#' @examples
#' \dontrun{
#' # List files for a study
#' gwas_files("list", "study", "GCST001")
#' 
#' # List files for a trait
#' gwas_files("list", "trait", "EFO_0000305")
#' 
#' # Download files
#' urls <- c("https://example.com/file1.tsv", "https://example.com/file2.tsv")
#' paths <- c("file1.tsv", "file2.tsv")
#' gwas_files("download", file_urls = urls, output_paths = paths)
#' }
gwas_files <- function(operation, entity_type = NULL, entity_id = NULL, 
                      secondary_id = NULL, file_urls = NULL, 
                      output_paths = NULL, max_concurrent = 4) {
  .Call(wrap__gwas_files, operation, entity_type, entity_id, secondary_id,
        file_urls, output_paths, max_concurrent)
}

#' Convenient wrapper for listing summary statistics files
#' @param entity_type Entity type: "study" or "trait"
#' @param entity_id Primary entity ID
#' @param secondary_id Optional secondary ID (for trait-study combinations)
#' @return JSON response with file information
#' @export
gwas_list_files <- function(entity_type, entity_id, secondary_id = NULL) {
  gwas_files("list", entity_type, entity_id, secondary_id)
}

#' Convenient wrapper for downloading summary statistics files
#' @param file_urls Vector of file URLs to download
#' @param output_paths Vector of output paths (must match length of file_urls)
#' @param max_concurrent Maximum number of concurrent downloads (default: 4)
#' @return Status message with download results
#' @export
gwas_download_files <- function(file_urls, output_paths, max_concurrent = 4) {
  gwas_files("download", file_urls = file_urls, output_paths = output_paths, 
            max_concurrent = max_concurrent)
}