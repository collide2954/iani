# iani: GWAS Summary Statistics Database API Client

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![R](https://img.shields.io/badge/R-%3E%3D4.2-blue.svg)](https://www.r-project.org/)
[![Rust](https://img.shields.io/badge/Rust-1.65+-orange.svg)](https://www.rust-lang.org/)

A comprehensive R package providing a high-performance interface to the [GWAS Catalog Summary Statistics Database API](https://www.ebi.ac.uk/gwas/summary-statistics/docs/api). Built with Rust for optimal performance and memory efficiency.

## Overview

`iani` enables R users to programmatically access genome-wide association study (GWAS) summary statistics from the EBI GWAS Catalog. The package provides functions to query:

- **Associations**: Variant-trait associations with statistical measures
- **Chromosomes**: Chromosome-specific data and associations
- **Studies**: GWAS study metadata and results
- **Traits**: Phenotypic traits and their associated studies
- **Files**: Summary statistics file listing and downloading

## Features

- **High Performance**: Built with Rust for fast HTTP requests and JSON parsing
- **Flexible Filtering**: Advanced filtering with `gwas_filter()` objects
- **Comprehensive Data Access**: Retrieve harmonized, raw, or both data formats
- **Parallel Downloads**: Multi-threaded file downloading capabilities
- **Type Safety**: Leverages Rust's type system for reliable data handling

## Installation

### Prerequisites

- R (>= 4.2)
- Rust toolchain (>= 1.65)
- Cargo (Rust package manager)

### Install from GitHub

```r
# Install devtools if you haven't already
install.packages("devtools")

# Install iani
devtools::install_github("collide2954/iani")
```

### System Requirements

The package requires the Rust toolchain to compile the underlying API client:

#### macOS & Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows
Download and run [rustup-init.exe](https://rustup.rs/)

## Quick Start

```r
library(iani)

# Get all chromosomes
chromosomes <- gwas_get("chromosomes")
cat(chromosomes)

# Get first 5 traits
traits <- gwas_get("traits", size = 5)
cat(traits)

# Get associations for a specific variant
variant_data <- gwas_associations("variant", "rs10875231", size = 5)
cat(variant_data)

# Get associations with p-value filtering
significant <- gwas_associations(
  p_value_min = "1e-8",
  p_value_max = "5e-5",
  size = 10
)
cat(significant)
```

## API Functions

The simplified API consists of 6 core functions:

### Core Functions

| Function | Description |
|----------|-------------|
| `gwas_filter()` | Create filter objects for advanced queries |
| `gwas_get()` | Get entities (chromosomes, studies, traits) |
| `gwas_associations()` | Get associations with flexible filtering |
| `gwas_files()` | Unified file operations (list/download) |
| `gwas_list_files()` | Convenient wrapper for listing files |
| `gwas_download_files()` | Convenient wrapper for downloading files |

## Usage Examples

### Entity Retrieval

```r
# Get all chromosomes
all_chromosomes <- gwas_get("chromosomes")

# Get specific chromosome
chr1 <- gwas_get("chromosomes", id = "1")

# Get studies with pagination
studies <- gwas_get("studies", start = 0, size = 10)

# Get specific trait
trait <- gwas_get("traits", id = "EFO_0003785")
```

### Association Queries

```r
# Get all associations (first 20)
all_assoc <- gwas_associations()

# Get associations for a specific variant
variant_assoc <- gwas_associations("variant", "rs10875231")

# Get associations for chromosome 1
chr1_assoc <- gwas_associations("chromosome", "1", size = 50)

# Get associations for a specific study
study_assoc <- gwas_associations("study", "GCST005038")

# Get associations for a trait
trait_assoc <- gwas_associations("trait", "EFO_0003785")
```

### Advanced Filtering with gwas_filter()

```r
# Create a filter object
filter <- gwas_filter(
  p_value = c(1e-8, 5e-5),      # P-value range
  bp_location = c(1000000, 2000000),  # Base pair range
  study = "GCST005038",          # Specific study
  reveal = "all",                # Show all data
  size = 100                     # Return 100 results
)

# Use filter with associations
filtered_assoc <- gwas_associations(filter = filter)

# Use filter with chromosome associations
chr_filtered <- gwas_associations("chromosome", "1", filter = filter)
```

### Direct Parameter Filtering

```r
# Filter associations by p-value
significant <- gwas_associations(
  p_value_min = "5e-8",
  p_value_max = "1e-5",
  size = 50
)

# Filter chromosome associations by location and p-value
region_assoc <- gwas_associations(
  entity_type = "chromosome",
  entity_id = "1",
  p_value_max = "1e-6",
  bp_min = 1000000,
  bp_max = 10000000,
  reveal = "all"
)
```

### File Operations

```r
# List summary statistics files for a study
study_files <- gwas_list_files("study", "GCST005038")
files_data <- jsonlite::fromJSON(study_files)

# List files for a trait
trait_files <- gwas_list_files("trait", "EFO_0003785")

# List files for trait-study combination
trait_study_files <- gwas_list_files("trait", "EFO_0003785", 
                                    secondary_id = "GCST005038")

# Download files
urls <- c(
  "https://www.ebi.ac.uk/gwas/summary-statistics/api/files/GCST005038.tsv.gz"
)
paths <- c("GCST005038.tsv.gz")
gwas_download_files(urls, paths, max_concurrent = 4)
```

### Unified File Operations

```r
# List files using unified interface
files_json <- gwas_files("list", "study", "GCST005038")

# Download files using unified interface
gwas_files(
  operation = "download",
  entity_type = "study",
  entity_id = "GCST005038",
  file_urls = urls,
  output_paths = paths,
  max_concurrent = 4
)
```

## Data Format

All functions return JSON strings that can be parsed using `jsonlite::fromJSON()`:

```r
library(jsonlite)

# Get data and parse JSON
raw_json <- gwas_associations(size = 5)
parsed_data <- fromJSON(raw_json)

# Access embedded associations
associations <- parsed_data$`_embedded`$associations

# Access pagination links
links <- parsed_data$`_links`
```

### Response Structure

Responses follow the HAL (Hypertext Application Language) format:

```json
{
  "_embedded": {
    "associations": {
      "0": {
        "variant_id": "rs10875231",
        "chromosome": 1,
        "base_pair_location": 99534456,
        "p_value": "2.826e-1",
        "effect_allele": "T",
        "other_allele": "G",
        ...
      }
    }
  },
  "_links": {
    "self": {"href": "..."},
    "next": {"href": "..."},
    "first": {"href": "..."}
  }
}
```

## Parameters

### Common Parameters

- `start`: Offset for pagination (default: 0)
- `size`: Number of items to return (default: 20)
- `reveal`: Data format - "raw", "all", or harmonized (default)

### Filter Parameters

- `p_value`: P-value range as `c(min, max)`
- `bp_location`: Base pair location range as `c(min, max)`
- `study`: Study accession filter
- `trait`: Trait ID filter

### Association Function Parameters

- `entity_type`: "variant", "chromosome", "study", "trait"
- `entity_id`: Specific entity identifier
- `p_value_min`/`p_value_max`: P-value thresholds
- `bp_min`/`bp_max`: Base pair location thresholds

## Data Fields

### Association Data

| Field | Type | Description |
|-------|------|-------------|
| `variant_id` | String | rsID of the variant |
| `chromosome` | Number | Chromosome number |
| `base_pair_location` | Number | Base pair position |
| `study_accession` | String | Study accession ID |
| `trait` | String | EFO trait identifier |
| `p_value` | String | Association p-value |
| `effect_allele` | String | Effect allele |
| `other_allele` | String | Non-effect allele |
| `effect_allele_frequency` | Number | Effect allele frequency |
| `odds_ratio` | Number | Odds ratio |
| `beta` | Number | Beta coefficient |
| `ci_lower` | Number | Lower confidence interval |
| `ci_upper` | Number | Upper confidence interval |
| `se` | Number | Standard error |
| `code` | Number | Harmonization status code |

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Citation

If you use this package in your research, please cite:

```
Gilio, B. (2025). iani: GWAS Summary Statistics Database API Client. https://github.com/collide2954/iani
```

## Related Resources

- [GWAS Catalog](https://www.ebi.ac.uk/gwas/)
- [GWAS Summary Statistics Database](https://www.ebi.ac.uk/gwas/summary-statistics)
- [API Documentation](https://www.ebi.ac.uk/gwas/summary-statistics/docs/api)
- [extendr](https://extendr.github.io/) - R and Rust integration