# Azure DevOps Rust API

> This is an unofficial Azure DevOps Rust API crate.
> It is in early development and only a small subset of function has been tested, so there will be issues and breaking changes.
> If you find any issues then please raise them via [Github](https://github.com/microsoft/azure-devops-rust-api/issues).

## Overview

`azure_devops_rust_api` implements a Rust interface to the Azure DevOps REST API (version 7.1).

The crate is autogenerated from the [Azure DevOps OpenAPI spec](https://github.com/MicrosoftDocs/vsts-rest-api-specs).

## Usage

### Usage overview

The crate has many features/modules, but the general approach is similar for all:

- Obtain authentication credentials
- Create a client for the feature/module that you want to use
- Use the client to make requests

### Code example

Example usage (from [examples/git_repo_list.rs](examples/git_repo_list.rs)):

```rust
    // Get authentication credentials via the az cli
    let credential = Arc::new(azure_identity::AzureCliCredential {});

    // Get ADO server configuration via environment variables
    let service_endpoint =
        env::var("ADO_SERVICE_ENDPOINT").expect("Must define ADO_SERVICE_ENDPOINT");
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a "git" client
    let client = git::operations::Client::new(service_endpoint, credential, vec![]);

    // Use the client to list all repositories in the specified organization/project
    let repos = client
        .repositories()
        .list(organization, project)
        .into_future()
        .await
        .unwrap()
        .value;

    // Output repo names
    for repo in repos.iter() {
        println!("{}", repo.name);
    }
    println!("{} repos found", repos.len());
```

Individual components in the API are enabled via Rust [`features`](https://doc.rust-lang.org/cargo/reference/features.html).

See the `features` section of [Cargo.toml](Cargo.toml) for the full list of features.

Example application `Cargo.toml` dependency spec showing how to specify desired features:

```toml
[dependencies]
...
azure_devops_rust_api = { version = "0.1.0", features = ["git", "pipelines"] }
```

## Examples

See [examples](examples/) directory.

Define environment variables:

```sh
export ADO_SERVICE_ENDPOINT=https://dev.azure.com
export ADO_ORGANIZATION=<organization-name>
export ADO_PROJECT=<project-name>
```

Note that you need to authenticate via `az login` before running the examples.

```sh
az login
cargo run --example git_repo_get --features="git" <repo-name>
```

## Useful links

- [Azure DevOps REST API Reference](https://docs.microsoft.com/en-us/rest/api/azure/devops/)
- [vsts-rest-api-specs](https://github.com/MicrosoftDocs/vsts-rest-api-specs)