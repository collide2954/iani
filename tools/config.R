# Note: Any variables prefixed with `.` are used for text
# replacement in the Makevars.in and Makevars.win.in

# check the packages MSRV first
source("tools/msrv.R")

# check DEBUG and NOT_CRAN environment variables
env_debug <- Sys.getenv("DEBUG")
env_not_cran <- Sys.getenv("NOT_CRAN")

# check if the vendored zip file exists
vendor_exists <- file.exists("src/rust/vendor.tar.xz")

is_not_cran <- env_not_cran != ""
is_debug <- env_debug != ""

if (is_debug) {
  # if we have DEBUG then we set not cran to true
  # CRAN is always release build
  is_not_cran <- TRUE
  message("Creating DEBUG build.")
}

if (!is_not_cran) {
  message("Building for CRAN.")
}

# we set cran flags only if NOT_CRAN is empty and if
# the vendored crates are present.
.cran_flags <- ifelse(
  !is_not_cran && vendor_exists,
  "-j 2 --offline",
  ""
)

# when DEBUG env var is present we use `--debug` build
.profile <- ifelse(is_debug, "dev", "release")
.clean_targets <- ifelse(is_debug, "", "$(TARGET_DIR)")

# We specify this target when building for webR
webr_target <- "wasm32-unknown-emscripten"

# here we check if the platform we are building for is webr
is_wasm <- identical(R.version$platform, webr_target)

# print to terminal to inform we are building for webr
if (is_wasm) {
  message("Building for WebR")
}

# Detect OpenSSL and other required libraries
detect_openssl <- function() {
  # Try to find OpenSSL using pkg-config
  pkg_config_available <- system("which pkg-config", ignore.stdout = TRUE, ignore.stderr = TRUE) == 0

  if (pkg_config_available) {
    # Try to get OpenSSL flags from pkg-config
    ssl_libs_result <- system("pkg-config --libs openssl", intern = TRUE, ignore.stderr = TRUE)
    ssl_cflags_result <- system("pkg-config --cflags openssl", intern = TRUE, ignore.stderr = TRUE)

    if (length(ssl_libs_result) > 0 && !is.na(ssl_libs_result)) {
      message("Found OpenSSL via pkg-config")
      return(paste(ssl_libs_result, "-ldl -lm"))
    }
  }

  # Fallback: try common system locations
  common_ssl_paths <- c(
    "/usr/lib/x86_64-linux-gnu",
    "/usr/lib64",
    "/usr/local/lib",
    "/opt/local/lib",
    "/usr/lib"
  )

  for (path in common_ssl_paths) {
    if (file.exists(file.path(path, "libssl.so")) || file.exists(file.path(path, "libssl.a"))) {
      message(paste("Found OpenSSL in", path))
      return(paste("-L", path, "-lssl -lcrypto -ldl -lm", sep = ""))
    }
  }

  # Default fallback
  message("Using default OpenSSL linking flags")
  return("-lssl -lcrypto -ldl -lm")
}

# Set library directory based on profile
.libdir <- ifelse(is_debug, "debug", "release")

# Detect required system libraries
if (is_wasm) {
  # For WebR, we don't need OpenSSL
  .pkg_libs <- ""
} else {
  # For regular builds, detect and include OpenSSL
  .pkg_libs <- detect_openssl()
}

# Write the configuration
writeLines(c(
  paste("LIBDIR =", .libdir),
  paste("PKG_LIBS =", .pkg_libs),
  paste("PROFILE =", .profile)
), "src/Makevars.tmp")

# Read Makevars.in template and substitute variables
makevars_in <- readLines("src/Makevars.in")
makevars_out <- gsub("@LIBDIR@", .libdir, makevars_in)
makevars_out <- gsub("@PKG_LIBS@", .pkg_libs, makevars_out)
makevars_out <- gsub("@PROFILE@", .profile, makevars_out)

# Write the final Makevars
writeLines(makevars_out, "src/Makevars")

message("Configuration complete.")
