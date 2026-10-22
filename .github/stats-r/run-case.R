# Runs one second-implementation spec (verification/second-impl/<case>.yaml)
# and compares the package output with its reference (D6, D38, D106).
#
# Usage: Rscript .github/stats-r/run-case.R SPEC.yaml OUT_DIR
#
# Exit 0 = every compared value within tolerance, or case skipped
# (`status: blocked`); 1 = a value out of tolerance, a missing spec field,
# a package version that differs from the spec, or an R error; 2 = usage.
# Writes OUT_DIR/<case>.json and OUT_DIR/<case>.md. Never writes to the repo.

required <- c(
  "case", "reference", "reference_kind", "inputs", "call",
  "outputs", "expected", "tolerance"
)

fail <- function(...) {
  message("FAIL: ", ...)
  quit(status = 1, save = "no")
}

check_fields <- function(spec, stem) {
  missing <- setdiff(required, names(spec))
  if (!("package" %in% names(spec) || "packages" %in% names(spec))) {
    missing <- c(missing, "package")
  }
  if (length(missing) > 0) {
    fail(stem, ": missing spec field(s): ", paste(missing, collapse = ", "))
  }
  if (!identical(spec$case, stem)) {
    fail(stem, ": case '", spec$case, "' must equal the file name")
  }
}

check_version <- function(pkg) {
  name <- pkg$name
  if (!requireNamespace(name, quietly = TRUE)) {
    fail(name, " is not installed (add it to .github/stats-r/renv.lock)")
  }
  if (identical(pkg$source, "GitHub")) {
    sha <- utils::packageDescription(name)$RemoteSha
    if (!identical(sha, pkg$commit)) {
      fail(name, ": installed commit ", sha, ", spec wants ", pkg$commit)
    }
    return(paste0(name, "@", substr(sha, 1, 12)))
  }
  found <- as.character(utils::packageVersion(name))
  if (!identical(found, pkg$version)) {
    fail(name, ": installed ", found, ", spec wants ", pkg$version)
  }
  paste0(name, " ", found)
}

tolerance_for <- function(tol, quantity) {
  if (is.null(tol)) {
    return(NULL)
  }
  if (!is.null(tol[[quantity]])) {
    return(as.numeric(tol[[quantity]]))
  }
  if (grepl("^(SNP\\.)?PP", quantity) && !is.null(tol$PP)) {
    return(as.numeric(tol$PP))
  }
  if (grepl("^lABF", quantity) && !is.null(tol$lABF)) {
    return(as.numeric(tol$lABF))
  }
  NULL
}

value_of <- function(expr, env) {
  out <- eval(parse(text = expr), envir = env)
  out <- unname(as.numeric(out))
  if (length(out) != 1 || is.na(out)) {
    fail("output '", expr, "' is not one number")
  }
  out
}

expected_values <- function(expected) {
  if (!is.null(expected$file)) {
    table <- utils::read.csv(expected$file, stringsAsFactors = FALSE)
    rows <- table[eval(parse(text = expected$filter), envir = table), ]
    return(stats::setNames(as.list(rows$value), rows$quantity))
  }
  meta <- c("file", "rule", "smoke_test", "for_context")
  expected[!(names(expected) %in% meta)]
}

row <- function(quantity, value, target, tol, kind) {
  diff <- if (is.null(target)) NA_real_ else abs(value - as.numeric(target))
  verdict <- if (is.null(target)) {
    "reported"
  } else if (is.null(tol)) {
    "deviation only"
  } else if (diff <= tol) {
    "pass"
  } else {
    "FAIL"
  }
  data.frame(
    quantity = quantity, kind = kind, value = value,
    target = if (is.null(target)) NA_real_ else as.numeric(target),
    abs_diff = diff, tolerance = if (is.null(tol)) NA_real_ else tol,
    verdict = verdict, stringsAsFactors = FALSE
  )
}

compare_single <- function(spec, env) {
  expected <- expected_values(spec$expected)
  rows <- lapply(names(spec$outputs), function(expr) {
    quantity <- spec$outputs[[expr]]
    if (!(quantity %in% names(expected))) {
      expr <- quantity
      quantity <- names(spec$outputs)[match(expr, spec$outputs)]
    }
    row(
      quantity, value_of(expr, env), expected[[quantity]],
      tolerance_for(spec$tolerance, quantity), "vs reference"
    )
  })
  do.call(rbind, rows)
}

compare_pair <- function(spec, env) {
  smoke <- spec$expected$smoke_test
  smoke_tol <- tolerance_for(spec$tolerance, "smoke_test")
  rows <- lapply(names(spec$outputs), function(quantity) {
    exprs <- spec$outputs[[quantity]]
    first <- value_of(exprs[[1]], env)
    second <- value_of(exprs[[2]], env)
    tol <- tolerance_for(spec$tolerance, quantity)
    # No tolerance (e.g. a bootstrap SE): reported, not tested, everywhere.
    own_smoke_tol <- if (is.null(tol)) NULL else smoke_tol
    rbind(
      row(quantity, second, first, tol, "package agreement"),
      row(quantity, first, smoke[[quantity]], own_smoke_tol, "smoke test")
    )
  })
  do.call(rbind, rows)
}

main <- function(args) {
  if (length(args) != 2) {
    message("usage: run-case.R SPEC.yaml OUT_DIR")
    quit(status = 2, save = "no")
  }
  stem <- sub("\\.yaml$", "", basename(args[[1]]))
  spec <- yaml::read_yaml(args[[1]])
  out_dir <- normalizePath(args[[2]], mustWork = FALSE)
  dir.create(out_dir, showWarnings = FALSE, recursive = TRUE)
  md <- file.path(out_dir, paste0(stem, ".md"))
  if (identical(spec$status, "blocked")) {
    writeLines(paste0("- `", stem, "`: skipped (status: blocked)"), md)
    jsonlite::write_json(
      list(case = stem, status = "skipped", reason = "status: blocked"),
      file.path(out_dir, paste0(stem, ".json")),
      auto_unbox = TRUE, pretty = TRUE
    )
    cat("skipped", stem, "(status: blocked)\n")
    return(invisible(0))
  }
  check_fields(spec, stem)
  pkgs <- if (is.null(spec$packages)) list(spec$package) else spec$packages
  versions <- vapply(pkgs, check_version, character(1))
  env <- new.env(parent = globalenv())
  work <- getwd()
  eval(parse(text = spec$call), envir = env)
  setwd(work)
  paired <- all(vapply(spec$outputs, length, integer(1)) == 2)
  table <- if (paired) compare_pair(spec, env) else compare_single(spec, env)
  failed <- any(table$verdict == "FAIL")
  jsonlite::write_json(
    list(
      case = stem, status = if (failed) "fail" else "pass",
      packages = versions, r = R.version.string, results = table
    ),
    file.path(out_dir, paste0(stem, ".json")),
    auto_unbox = TRUE, pretty = TRUE, digits = NA
  )
  fmt <- function(x) ifelse(is.na(x), "", format(x, digits = 15))
  lines <- c(
    paste0("### `", stem, "` (", paste(versions, collapse = ", "), ")"),
    "",
    "| quantity | kind | value | target | abs diff | tolerance | verdict |",
    "|---|---|---|---|---|---|---|",
    sprintf(
      "| %s | %s | %s | %s | %s | %s | %s |", table$quantity, table$kind,
      fmt(table$value), fmt(table$target), fmt(table$abs_diff),
      fmt(table$tolerance), table$verdict
    ),
    ""
  )
  writeLines(lines, md)
  cat(lines, sep = "\n")
  if (failed) fail(stem, ": value(s) out of tolerance")
  invisible(0)
}

main(commandArgs(trailingOnly = TRUE))
