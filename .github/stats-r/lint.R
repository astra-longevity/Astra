# lintr + styler on every R file in .github/stats-r/ (D45: warnings are errors).
# Usage: Rscript .github/stats-r/lint.R   Exit 1 on any lint or style change.

.libPaths(c(Sys.getenv("R_LIBS_USER"), .libPaths()))
files <- list.files(".github/stats-r", pattern = "[.]R$", full.names = TRUE)
lints <- unlist(lapply(files, lintr::lint), recursive = FALSE)
for (l in lints) print(l)
styled <- styler::style_file(files, dry = "on")
changed <- styled$file[styled$changed]
if (length(changed) > 0) {
  message("styler would change: ", paste(changed, collapse = ", "))
}
if (length(lints) > 0 || length(changed) > 0) quit(status = 1, save = "no")
cat(length(files), "R file(s) clean\n")
