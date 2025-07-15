# Seek

## About
The `seek` binary is a tool used to quickly find directories, files, and symbolic links via the command line; hence **seek**.

## Installation

### Download the Windows Binary via Command Line

You can download the latest Windows binary directly using one of the following commands:

#### Using curl (Windows 10+):

```cmd
curl -L -o seek.exe https://github.com/Eth3rna1/seek/releases/download/v2.1.4/seek.exe
```

#### Using PowerShell
```cmd
Invoke-WebRequest -Uri "https://github.com/Eth3rna1/seek/releases/download/v2.1.4/seek.exe" -OutFile "seek.exe"
```

#### Manual Download
Alternatively, you can download the binary manually from the [Releases page](https://github.com/Eth3rna1/seek/releases/tag/v2.1.4)

## Getting Started
The `seek` binary only cares about one parameter, the `query` parameter, which is meant to be a regular expression string.
Keep in mind that the query is going to be matching the basename of each path. Take the following as an example:

> Path `c:\\this\\is\\a\\file.txt` will take `file.txt` and compare it to the regular expression query.

The binary also doesn't automatically cache.
To start making use of the cache, you'd need to include the `--use-cache` (`-u`) flag, which will then
start applying the cache logic.

The binary searches with a case insensitive query by default.
If you wish to make your query case sensitive, raise the `--cs` flag.

When using the flags `--files` (`-f`), `--dirs` (`-d`), and `--symlinks` (`-s`), which indicate what objects to consider.
By default, the program considers all object types.
If you wish to consider one or more object types, simply raise the flag you wish the program to consider.

Example:

> seek .+ -s -d

The previous example will indicate to the program to only consider symbolic links and directories.


## Things to consider
There are reserved characters in the windows terminals such as the `|` and the `^` characters.

If you need to bypass such reserved characters in your terminal, you can prefix them with the following characters:

|Operating System Terminal|Character| Example|
|-------------------------|---------|--------|
|Windows CMD|`^`|`seek main\.(cpp^\|rs)`|
|Windows PowerShell|`` ` ``|``seek main\.\`(cpp\`\|rs\`)``|
|Linux and MacOS|`\`|`seek main\.\(cpp\|rs\)`|


## Examples
Searches for an object with "example" in its stem name and "exe" in its extension.
```console
seek example.exe
```
Based on the regex, the sought example could be:

An exact match
```text
1.) example.exe
```
Or

A match where the object includes your target object in the name and extension
```text
1.) example__but_with_a_longer_name_example.exe
```
Or

A mixture of both instances regarding the extension and stem
```text
1.) example__but_with_a_longer_name_example.executable
```

**The only way to enforce an exact match is by using the `--exact` (`-e`) flag, which will escape special characters**
```console
seek example.exe -e
```

To look for objects with only regarding the extension, you can end the query with `$`.
To better the regex, you can prefix the query with `\.`, which specifies a literal period.
```console
seek \.exe$
```
could return
```text
1.) bin1.exe
2.) thisisabin.exe
3.) example.exe
```

## Flags
### Configuration Flags

| Flag | Alias | Description |
|------|-|-------------|
|--exact|-e| Searches for an exact match on regarding the regular expression query|
| --path | -p | Indicates the path on where to start searching; the default path is the current working directory |
|--files| -f | Indicates to exclusively consider files |
|--dirs| -d | Indicates to exclusively consider directories|
|--symlinks|-s| Indicates to exclusively consider symbolic links|
| --root | -r | Indicates to start searching from root |
| --log | -l | Prints out the state of the program throughout execution |
| --depth | | The depth in subdirectories to search |
|--help| -h | Used to display the help message|

### Cache Flags

| Flag | Alias | Description |
|------|-|-------------|
|--cache| -c | Caches the entire sought directories and saves them into a JSON file, then exits |
|--use-cache| -u | Indicates to not search, rather to read from the cache file |
|--update-cache| | Forces an update on the cache |
|--ignore-update| -i | Ignores the invalidity of the cache and uses the cache anyway; must be used along with the --use-cache flag |
|--cache-location| | Used to specify the cache file location along with its JSON file name. [default: ./info.json]|
