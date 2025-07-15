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
- **Variants**: Genetic variants and their associations

## Features

- 🚀 **High Performance**: Built with Rust for fast HTTP requests and JSON parsing
- 🔍 **Comprehensive Filtering**: Support for p-value thresholds, base pair ranges, and more
- 📊 **Flexible Data Access**: Retrieve harmonized, raw, or both data formats
- 🔗 **Complete API Coverage**: All GWAS Summary Statistics Database endpoints supported
- 📝 **HAL Format Support**: Full support for Hypertext Application Language responses
- 🎯 **Type Safety**: Leverages Rust's type system for reliable data handling

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

#### Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows
Download and run [rustup-init.exe](https://rustup.rs/)

## Quick Start

```r
library(iani)

# Test the connection
gwas_client_new()

# Get associations for a specific variant
variant_data <- gwas_get_variant_associations("rs10875231", size = 5)
cat(variant_data)

# Get all chromosomes
chromosomes <- gwas_get_chromosomes()
cat(chromosomes)

# Get associations for chromosome 1 with p-value filtering
chr1_assoc <- gwas_get_chromosome_associations(
  chromosome = "1",
  p_upper = "1e-5",
  size = 10
)
cat(chr1_assoc)

# Get all traits
traits <- gwas_get_traits(size = 5)
cat(traits)
```

## API Functions

### Core Functions

| Function | Description |
|----------|-------------|
| `gwas_client_new()` | Initialize GWAS API client |
| `gwas_get_associations()` | Get all associations with filtering |
| `gwas_get_variant_associations()` | Get associations for specific variant |

### Chromosome Functions

| Function | Description |
|----------|-------------|
| `gwas_get_chromosomes()` | List all chromosomes |
| `gwas_get_chromosome()` | Get specific chromosome |
| `gwas_get_chromosome_associations()` | Get associations for chromosome |
| `gwas_get_chromosome_variant_associations()` | Get variant associations on chromosome |

### Study Functions

| Function | Description |
|----------|-------------|
| `gwas_get_studies()` | List all studies |
| `gwas_get_study()` | Get specific study |
| `gwas_get_study_associations()` | Get associations for study |

### Trait Functions

| Function | Description |
|----------|-------------|
| `gwas_get_traits()` | List all traits |
| `gwas_get_trait()` | Get specific trait |
| `gwas_get_trait_associations()` | Get associations for trait |
| `gwas_get_trait_studies()` | Get studies for trait |
| `gwas_get_trait_study()` | Get specific trait-study combination |
| `gwas_get_trait_study_associations()` | Get associations for trait-study |

## Usage Examples

### Basic Association Queries

```r
# Get first 10 associations
associations <- gwas_get_associations(size = 10)

# Filter by p-value
significant <- gwas_get_associations(
  p_upper = "5e-8",  # Genome-wide significance
  size = 100
)

# Get raw data instead of harmonized
raw_data <- gwas_get_associations(
  reveal = "raw",
  size = 10
)

# Get both harmonized and raw data
all_data <- gwas_get_associations(
  reveal = "all",
  size = 10
)
```

### Chromosome-Specific Queries

```r
# Get associations for chromosome 22
chr22 <- gwas_get_chromosome_associations(
  chromosome = "22",
  size = 50
)

# Filter by base pair location
region <- gwas_get_chromosome_associations(
  chromosome = "1",
  bp_lower = 1000000,
  bp_upper = 2000000,
  size = 100
)

# Get specific variant on chromosome
variant_chr <- gwas_get_chromosome_variant_associations(
  chromosome = "1",
  variant_id = "rs10875231"
)
```

### Study and Trait Queries

```r
# Get studies for a specific trait
trait_studies <- gwas_get_trait_studies("EFO_0003785")

# Get associations for a specific study
study_assoc <- gwas_get_study_associations(
  study_accession = "GCST005038",
  p_upper = "1e-5"
)

# Get associations for trait-study combination
trait_study_assoc <- gwas_get_trait_study_associations(
  trait_id = "EFO_0003785",
  study_accession = "GCST005038"
)
```

### Advanced Filtering

```r
# Complex filtering example
filtered_data <- gwas_get_chromosome_associations(
  chromosome = "1",
  p_lower = "1e-10",      # Lower p-value bound
  p_upper = "5e-8",       # Upper p-value bound  
  bp_lower = 1000000,     # Base pair range start
  bp_upper = 10000000,    # Base pair range end
  reveal = "all",         # Show both harmonized and raw
  size = 200              # Return up to 200 results
)
```

## Data Format

All functions return JSON strings that can be parsed using `jsonlite::fromJSON()`:

```r
library(jsonlite)

# Get data and parse JSON
raw_json <- gwas_get_associations(size = 5)
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

## Query Parameters

### Common Parameters

- `start`: Offset for pagination (default: 0)
- `size`: Number of items to return (default: 20)
- `reveal`: Data format - "raw", "all", or harmonized (default)

### Association Filtering

- `p_lower`: Lower p-value threshold (e.g., "1e-10")
- `p_upper`: Upper p-value threshold (e.g., "5e-8")
- `study_accession`: Filter by specific study
- `trait`: Filter by specific trait ID

### Chromosome-Specific

- `bp_lower`: Lower base pair location threshold
- `bp_upper`: Upper base pair location threshold

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

## Error Handling

The package returns descriptive error messages for common issues:

```r
# Invalid variant ID
result <- gwas_get_variant_associations("invalid_rsid")
# Returns: "Error fetching variant associations: 404 Not Found"

# Network issues
# Returns: "Error creating client: ..."
```

## Performance Tips

1. **Use appropriate page sizes**: Start with small `size` values for exploration
2. **Filter early**: Use p-value and base pair filters to reduce data transfer
3. **Chromosome-specific queries**: Use chromosome endpoints when possible for faster results
4. **Batch processing**: Process results in chunks for large datasets

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
