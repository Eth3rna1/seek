# Seek

## About
The `seek` binary is a tool used to quickly find directories, files, or symbolic links via the command line; hence "seek".

## Getting Started
The seek binary implements various flags, the only argument that you only need to worry about is the `<OBJECT>` argument.
The searching process is case insensitive and doesn't look for the exact object match by default, rather it separates the stem from the extension and compares each file path's base name by checking if the path's base name contains the specified object's stem and extension.

## Examples
Searches for an object with "example" in its stem name and "exe" in its extension.
```console
seek example.exe
```
Based on the matches, the example sought objects can be:

An exact match
```text
1.) example.exe
```
Or

A match where the object includes your target object in the name and extension
```text
1.) example__but_with_a_longer_name.exe
```
Or

A mixture of both instances regarding the extension and stem
```text
1.) example__but_with_a_longer_name.executable
```

**The only way to enforce an exact match is by using the (--exact) flag**
```console
seek example.exe --exact
```

To look for objects with only regarding to the extension, you can replace the stem name with a `*`.
```console
seek *.exe
```
returns
```text
1.) bin1.exe
2.) thisisabin.exe
3.) example.exe
```

## Flags
### Configuration Flags

| Flag | Alias | Description |
|------|-|-------------|
|--extension| -e | The extension of the file you are searching for if applicable; generally not needed since its handled automatically|
|--exact|  | Searches for an exact match on regarding the basename and extension, if any|
| --path | -p | Indicates the path on where to start searching; the default path is the current working directory |
| --root | -r | Indicates to start searching from root |
| --log | -l | Prints out the state of the program throughout the runtime |
| --depth | -d | The depth in subdirectories you'd like to search |
| --no-extension |  | Indicates that the object does not contain an extension; helpful when the object contains multiple periods within the name |
|--help| -h | Use to display the help message|

### Cache Flags

| Flag | Alias | Description |
|------|-|-------------|
|--cache| -c | Caches the entire sought directories and saves them into a JSON file, then exits |
|--use-cache| -u | Indicates to not search, rather to read from the cache file |
|--update-cache| | Forces an update on the cache |
|--ignore-update| -i | Ignores the invalidity of the cache and uses the cache anyway; must be used along with the --use-cache flag |
|--cache-location| | Used to specify the cache file location along with its JSON file name. [default: ./info.json]|

### Aftermath Flags

| Flag | Alias | Description |
|------|-|-------------|
| --copy |  | Use to copy a specific path onto your clipboard via its index|

