// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

// search_package.rs
// Search package example.
use anyhow::Result;
use azure_devops_rust_api::search;
use azure_devops_rust_api::search::models::{
    EntitySearchRequest, EntitySearchRequestBase, PackageSearchRequest,
};
use azure_devops_rust_api::Credential;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get authentication credential either from a PAT ("ADO_TOKEN") or via the az cli
    let credential = match env::var("ADO_TOKEN") {
        Ok(token) => {
            println!("Authenticate using PAT provided via $ADO_TOKEN");
            Credential::from_pat(token)
        }
        Err(_) => {
            println!("Authenticate using Azure CLI");
            Credential::from_token_credential(Arc::new(azure_identity::AzureCliCredential::new()))
        }
    };

    // Get ADO configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let search_package_name =
        env::var("PKG_NAME").expect("Must define a package name <PKG_NAME> to be searched");

    // Create a search client
    println!("Create search client");
    let search_client = search::ClientBuilder::new(credential).build();

    let entity_search_request_base = EntitySearchRequestBase {
        filters: None,
        search_text: Some(search_package_name.to_string()),
    };

    let entity_search_request = EntitySearchRequest {
        entity_search_request_base,
        top: Some(10), // Must specify `top`, otherwise search fails
        ..Default::default()
    };

    let package_search_request = PackageSearchRequest {
        entity_search_request,
    };

    // Do the search
    println!("Search...");
    let search_results = search_client
        .package_search_results_client()
        .fetch_package_search_results(organization, package_search_request)
        .into_future()
        .await?
        .results;

    println!("Found {} results", search_results.len());
    if let Some(result) = search_results.iter().next() {
        println!("Example search result:\n{:#?}", result);
    }

    Ok(())
}