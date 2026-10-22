# Installs the packages pinned in .github/stats-r/renv.lock into R_LIBS_USER.
# Usage: Rscript .github/stats-r/restore.R (env: R_LIBS_USER, LOCKFILE)

lib <- Sys.getenv("R_LIBS_USER")
.libPaths(c(lib, .libPaths()))
rspm <- "https://packagemanager.posit.co/cran/__linux__/noble/latest"
options(repos = c(CRAN = rspm))
if (!requireNamespace("renv", quietly = TRUE)) {
  utils::install.packages("renv", lib = lib)
}
renv::restore(
  lockfile = Sys.getenv("LOCKFILE"), library = lib,
  repos = getOption("repos"), prompt = FALSE
)
