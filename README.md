# Seek

## About
The `seek` binary is a tool used to quickly find directories, files, or symbolic links via the command line; hence "seek".

## Getting Started
The seek binary implements various flags, the only argument that you only need to worry about is the `<OBJECT>` argument.
The searching process is case insensitive and doesn't look for the exact object given, rather it separates the stem from
the extension and compares each file path's base name by checking if the path's base name contains the specified object's stem and extension by default.

### Usage
```console
seek example.exe
```
Searches for an object with "example" in its stem name and "exe" in its extension, if its a file.

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
|--name| -n | Used to specify the cache file name, by default, the name is "info.json"|

### Aftermath Flags

| Flag | Alias | Description |
|------|-|-------------|
| --copy |  | Use to copy a specific path onto your clipboard via its index|

