# Project File Format

### .hugproject

```rs
struct HugProject {
    edition: u32, // The edition of hug this project was made for/with
    version: Version, // User-specified semver triple
    dependencies: Vec<Dependency>,
}
```

### Version

```rs
struct Version { // Semver: major.minor.patch-suffix
    major: u32,
    minor: u32,
    patch: u32,
    suffix: Option<String>,
}
```

### Dependency

```rs
enum Dependency {
    External(ExternalLocation), // Path to .hugproject file
    Internal(String),
}

enum ExternalLocation {
    Url(String), // 
    GitHub {
        url: String,
        commit: String,
    },
    Path(String),
}
```