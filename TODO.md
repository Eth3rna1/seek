# To-Do list

## In progress
* [ ] Fix the `--cmd` flag file file path interpolation

* [ ] Fix the `--include` flag - It's not actually including.
    * [ ] Decide on renaming the `--include` flag to `--parent` or `--ancestor`, which takes a list

* [ ] Add the functionality for the `--exclude` (`-x`) flag to exclude result files as well
    * [ ] Add a `--keep-result` flag to explicitly state to keep the result paths without filtering

* [ ] Add an `--all` flag to apply logic to all result file paths

* [ ] Add flags for extended functionality
    * [ ] `--move`
    * [ ] `--rename`
    * [ ] `--delete`
        * Require explicit confirmation, or provide a `--force` option for non-empty directories

* [ ] Discrepancy: Searching for `*.zip` will result in a regex error, make it possible to use that syntax

* [ ] Add a `--name` flag to display only the basenames

* [ ] Fix error handling, and propagate errors gracefully

* [ ] Implement tests

* [ ] Add a flag that disables enumeration when displaying the result file paths


## Completed
