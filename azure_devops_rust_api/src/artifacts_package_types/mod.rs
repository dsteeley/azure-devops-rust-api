// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: crate::Credential,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: crate::Credential,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = "https://pkgs.dev.azure.com";
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: crate::Credential) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self
            .scopes
            .unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &crate::Credential {
        &self.credential
    }
    #[allow(dead_code)]
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(
        &self,
        request: &mut azure_core::Request,
    ) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: crate::Credential) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: crate::Credential,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn maven_client(&self) -> maven::Client {
        maven::Client(self.clone())
    }
    pub fn npm_client(&self) -> npm::Client {
        npm::Client(self.clone())
    }
    pub fn nu_get_client(&self) -> nu_get::Client {
        nu_get::Client(self.clone())
    }
    pub fn python_client(&self) -> python::Client {
        python::Client(self.clone())
    }
    pub fn universal_client(&self) -> universal::Client {
        universal::Client(self.clone())
    }
}
pub mod maven {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the upstreaming behavior of a package within the context of a feed"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed`: The name or id of the feed"]
        #[doc = "* `group_id`: The group id of the package"]
        #[doc = "* `artifact_id`: The artifact id of the package"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_upstreaming_behavior(
            &self,
            organization: impl Into<String>,
            feed: impl Into<String>,
            group_id: impl Into<String>,
            artifact_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_upstreaming_behavior::Builder {
            get_upstreaming_behavior::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed: feed.into(),
                group_id: group_id.into(),
                artifact_id: artifact_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Set the upstreaming behavior of a package within the context of a feed\n\nThe package does not need to necessarily exist in the feed prior to setting the behavior.\nThis assists with packages that are not yet ingested from an upstream, yet the feed owner wants\nto apply a specific behavior on the first ingestion."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed`: The name or id of the feed"]
        #[doc = "* `body`: The behavior to apply to the package within the scope of the feed"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn set_upstreaming_behavior(
            &self,
            organization: impl Into<String>,
            feed: impl Into<String>,
            group_id: impl Into<String>,
            artifact_id: impl Into<String>,
            body: impl Into<models::UpstreamingBehavior>,
            project: impl Into<String>,
        ) -> set_upstreaming_behavior::Builder {
            set_upstreaming_behavior::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed: feed.into(),
                group_id: group_id.into(),
                artifact_id: artifact_id.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get information about a package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed`: Name or ID of the feed."]
        #[doc = "* `group_id`: Group ID of the package."]
        #[doc = "* `artifact_id`: Artifact ID of the package."]
        #[doc = "* `version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version(
            &self,
            organization: impl Into<String>,
            feed: impl Into<String>,
            group_id: impl Into<String>,
            artifact_id: impl Into<String>,
            version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version::Builder {
            get_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed: feed.into(),
                group_id: group_id.into(),
                artifact_id: artifact_id.into(),
                version: version.into(),
                project: project.into(),
                show_deleted: None,
            }
        }
        #[doc = "Update state for a package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Details to be updated."]
        #[doc = "* `feed`: Name or ID of the feed."]
        #[doc = "* `group_id`: Group ID of the package."]
        #[doc = "* `artifact_id`: Artifact ID of the package."]
        #[doc = "* `version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_package_version(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PackageVersionDetails>,
            feed: impl Into<String>,
            group_id: impl Into<String>,
            artifact_id: impl Into<String>,
            version: impl Into<String>,
            project: impl Into<String>,
        ) -> update_package_version::Builder {
            update_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed: feed.into(),
                group_id: group_id.into(),
                artifact_id: artifact_id.into(),
                version: version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete a package version from the feed and move it to the feed's recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed`: Name or ID of the feed."]
        #[doc = "* `group_id`: Group ID of the package."]
        #[doc = "* `artifact_id`: Artifact ID of the package."]
        #[doc = "* `version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_package_version(
            &self,
            organization: impl Into<String>,
            feed: impl Into<String>,
            group_id: impl Into<String>,
            artifact_id: impl Into<String>,
            version: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_package_version::Builder {
            delete_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed: feed.into(),
                group_id: group_id.into(),
                artifact_id: artifact_id.into(),
                version: version.into(),
                project: project.into(),
            }
        }
        #[doc = "Get information about a package version in the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed`: Name or ID of the feed."]
        #[doc = "* `group_id`: Group ID of the package."]
        #[doc = "* `artifact_id`: Artifact ID of the package."]
        #[doc = "* `version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed: impl Into<String>,
            group_id: impl Into<String>,
            artifact_id: impl Into<String>,
            version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version_from_recycle_bin::Builder {
            get_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed: feed.into(),
                group_id: group_id.into(),
                artifact_id: artifact_id.into(),
                version: version.into(),
                project: project.into(),
            }
        }
        #[doc = "Restore a package version from the recycle bin to its associated feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Set the 'Deleted' property to false to restore the package."]
        #[doc = "* `feed`: Name or ID of the feed."]
        #[doc = "* `group_id`: Group ID of the package."]
        #[doc = "* `artifact_id`: Artifact ID of the package."]
        #[doc = "* `version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn restore_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::MavenRecycleBinPackageVersionDetails>,
            feed: impl Into<String>,
            group_id: impl Into<String>,
            artifact_id: impl Into<String>,
            version: impl Into<String>,
            project: impl Into<String>,
        ) -> restore_package_version_from_recycle_bin::Builder {
            restore_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed: feed.into(),
                group_id: group_id.into(),
                artifact_id: artifact_id.into(),
                version: version.into(),
                project: project.into(),
            }
        }
        #[doc = "Permanently delete a package from a feed's recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed`: Name or ID of the feed."]
        #[doc = "* `group_id`: Group ID of the package."]
        #[doc = "* `artifact_id`: Artifact ID of the package."]
        #[doc = "* `version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed: impl Into<String>,
            group_id: impl Into<String>,
            artifact_id: impl Into<String>,
            version: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_package_version_from_recycle_bin::Builder {
            delete_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed: feed.into(),
                group_id: group_id.into(),
                artifact_id: artifact_id.into(),
                version: version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete or restore several package versions from the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Information about the packages to update, the operation to perform, and its associated data."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_recycle_bin_packages(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::MavenPackagesBatchRequest>,
            feed: impl Into<String>,
            project: impl Into<String>,
        ) -> update_recycle_bin_packages::Builder {
            update_recycle_bin_packages::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed: feed.into(),
                project: project.into(),
            }
        }
        #[doc = "Fulfills Maven package file download requests by either returning the URL of the requested package file or, in the case of Azure DevOps Server (OnPrem), returning the content as a stream.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `group_id`: GroupId of the maven package"]
        #[doc = "* `artifact_id`: ArtifactId of the maven package"]
        #[doc = "* `version`: Version of the package"]
        #[doc = "* `file_name`: File name to download"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn download_package(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            group_id: impl Into<String>,
            artifact_id: impl Into<String>,
            version: impl Into<String>,
            file_name: impl Into<String>,
            project: impl Into<String>,
        ) -> download_package::Builder {
            download_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                group_id: group_id.into(),
                artifact_id: artifact_id.into(),
                version: version.into(),
                file_name: file_name.into(),
                project: project.into(),
            }
        }
        #[doc = "Update several packages from a single feed in a single request. The updates to the packages do not happen atomically.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Information about the packages to update, the operation to perform, and its associated data."]
        #[doc = "* `feed_id`: Feed which contains the packages to update."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_package_versions(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::MavenPackagesBatchRequest>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_package_versions::Builder {
            update_package_versions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
    }
    pub mod get_upstreaming_behavior {
        use super::models;
        type Response = models::UpstreamingBehavior;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed: String,
            pub(crate) group_id: String,
            pub(crate) artifact_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/maven/groups/{}/artifacts/{}/upstreaming" , this . client . endpoint () , & this . organization , & this . project , & this . feed , & this . group_id , & this . artifact_id)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpstreamingBehavior =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod set_upstreaming_behavior {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed: String,
            pub(crate) group_id: String,
            pub(crate) artifact_id: String,
            pub(crate) body: models::UpstreamingBehavior,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/maven/groups/{}/artifacts/{}/upstreaming" , this . client . endpoint () , & this . organization , & this . project , & this . feed , & this . group_id , & this . artifact_id)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed: String,
            pub(crate) group_id: String,
            pub(crate) artifact_id: String,
            pub(crate) version: String,
            pub(crate) project: String,
            pub(crate) show_deleted: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to show information for deleted packages."]
            pub fn show_deleted(mut self, show_deleted: bool) -> Self {
                self.show_deleted = Some(show_deleted);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/maven/groups/{}/artifacts/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed , & this . group_id , & this . artifact_id , & this . version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(show_deleted) = &this.show_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("showDeleted", &show_deleted.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_package_version {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PackageVersionDetails,
            pub(crate) feed: String,
            pub(crate) group_id: String,
            pub(crate) artifact_id: String,
            pub(crate) version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/maven/groups/{}/artifacts/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed , & this . group_id , & this . artifact_id , & this . version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_package_version {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed: String,
            pub(crate) group_id: String,
            pub(crate) artifact_id: String,
            pub(crate) version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/maven/groups/{}/artifacts/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed , & this . group_id , & this . artifact_id , & this . version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version_from_recycle_bin {
        use super::models;
        type Response = models::MavenPackageVersionDeletionState;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed: String,
            pub(crate) group_id: String,
            pub(crate) artifact_id: String,
            pub(crate) version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/maven/RecycleBin/groups/{}/artifacts/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed , & this . group_id , & this . artifact_id , & this . version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MavenPackageVersionDeletionState =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod restore_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::MavenRecycleBinPackageVersionDetails,
            pub(crate) feed: String,
            pub(crate) group_id: String,
            pub(crate) artifact_id: String,
            pub(crate) version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/maven/RecycleBin/groups/{}/artifacts/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed , & this . group_id , & this . artifact_id , & this . version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed: String,
            pub(crate) group_id: String,
            pub(crate) artifact_id: String,
            pub(crate) version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/maven/RecycleBin/groups/{}/artifacts/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed , & this . group_id , & this . artifact_id , & this . version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_recycle_bin_packages {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::MavenPackagesBatchRequest,
            pub(crate) feed: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/maven/RecycleBin/packagesBatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod download_package {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) group_id: String,
            pub(crate) artifact_id: String,
            pub(crate) version: String,
            pub(crate) file_name: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/maven/{}/{}/{}/{}/content",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.group_id,
                            &this.artifact_id,
                            &this.version,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_package_versions {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::MavenPackagesBatchRequest,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/maven/packagesbatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod npm {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get information about a scoped package version (such as @scope/name).\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_scope`: Scope of the package (the 'scope' part of @scope/name)."]
        #[doc = "* `unscoped_package_name`: Name of the package (the 'name' part of @scope/name)."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_scoped_package_version(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_scope: impl Into<String>,
            unscoped_package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_scoped_package_version::Builder {
            get_scoped_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_scope: package_scope.into(),
                unscoped_package_name: unscoped_package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_scoped_package(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PackageVersionDetails>,
            feed_id: impl Into<String>,
            package_scope: impl Into<String>,
            unscoped_package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> update_scoped_package::Builder {
            update_scoped_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_scope: package_scope.into(),
                unscoped_package_name: unscoped_package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Unpublish a scoped package version (such as @scope/name).\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_scope`: Scope of the package (the 'scope' part of @scope/name)."]
        #[doc = "* `unscoped_package_name`: Name of the package (the 'name' part of @scope/name)."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn unpublish_scoped_package(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_scope: impl Into<String>,
            unscoped_package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> unpublish_scoped_package::Builder {
            unpublish_scoped_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_scope: package_scope.into(),
                unscoped_package_name: unscoped_package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Get information about an unscoped package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version::Builder {
            get_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_package(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PackageVersionDetails>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> update_package::Builder {
            update_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Unpublish an unscoped package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn unpublish_package(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> unpublish_package::Builder {
            unpublish_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Get the upstreaming behavior of the (scoped) package within the context of a feed"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: The name or id of the feed"]
        #[doc = "* `package_scope`: The scope of the package"]
        #[doc = "* `unscoped_package_name`: The name of the scoped package"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_upstreaming_behavior(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_scope: impl Into<String>,
            unscoped_package_name: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_upstreaming_behavior::Builder {
            get_package_upstreaming_behavior::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_scope: package_scope.into(),
                unscoped_package_name: unscoped_package_name.into(),
                project: project.into(),
            }
        }
        #[doc = "Set the upstreaming behavior of a (scoped) package within the context of a feed\n\nThe package does not need to necessarily exist in the feed prior to setting the behavior.\nThis assists with packages that are not yet ingested from an upstream, yet the feed owner wants\nto apply a specific behavior on the first ingestion."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: The name or id of the feed"]
        #[doc = "* `package_scope`: The scope of the package"]
        #[doc = "* `unscoped_package_name`: The name of the scoped package"]
        #[doc = "* `body`: The behavior to apply to the scoped package within the scope of the feed"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn set_scoped_upstreaming_behavior(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_scope: impl Into<String>,
            unscoped_package_name: impl Into<String>,
            body: impl Into<models::UpstreamingBehavior>,
            project: impl Into<String>,
        ) -> set_scoped_upstreaming_behavior::Builder {
            set_scoped_upstreaming_behavior::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_scope: package_scope.into(),
                unscoped_package_name: unscoped_package_name.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn download_scoped_package(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_scope: impl Into<String>,
            unscoped_package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> download_scoped_package::Builder {
            download_scoped_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_scope: package_scope.into(),
                unscoped_package_name: unscoped_package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Get the Readme for a package version with an npm scope.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_scope`: Scope of the package (the 'scope' part of @scope\\name)"]
        #[doc = "* `unscoped_package_name`: Name of the package (the 'name' part of @scope\\name)"]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_scoped_package_readme(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_scope: impl Into<String>,
            unscoped_package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_scoped_package_readme::Builder {
            get_scoped_package_readme::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_scope: package_scope.into(),
                unscoped_package_name: unscoped_package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Get the upstreaming behavior of the (unscoped) package within the context of a feed"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: The name or id of the feed"]
        #[doc = "* `package_name`: The name of the package"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_scoped_package_upstreaming_behavior(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            project: impl Into<String>,
        ) -> get_scoped_package_upstreaming_behavior::Builder {
            get_scoped_package_upstreaming_behavior::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                project: project.into(),
            }
        }
        #[doc = "Set the upstreaming behavior of a (scoped) package within the context of a feed\n\nThe package does not need to necessarily exist in the feed prior to setting the behavior.\nThis assists with packages that are not yet ingested from an upstream, yet the feed owner wants\nto apply a specific behavior on the first ingestion."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: The name or id of the feed"]
        #[doc = "* `package_name`: The name of the package"]
        #[doc = "* `body`: The behavior to apply to the scoped package within the scope of the feed"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn set_upstreaming_behavior(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            body: impl Into<models::UpstreamingBehavior>,
            project: impl Into<String>,
        ) -> set_upstreaming_behavior::Builder {
            set_upstreaming_behavior::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get an unscoped npm package.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn download_package(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> download_package::Builder {
            download_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Get the Readme for a package version that has no npm scope.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_readme(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_readme::Builder {
            get_package_readme::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Update several packages from a single feed in a single request. The updates to the packages do not happen atomically.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Information about the packages to update, the operation to perform, and its associated data."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_packages(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::NpmPackagesBatchRequest>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_packages::Builder {
            update_packages::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Get information about a scoped package version in the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_scope`: Scope of the package (the 'scope' part of @scope/name)"]
        #[doc = "* `unscoped_package_name`: Name of the package (the 'name' part of @scope/name)."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_scoped_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_scope: impl Into<String>,
            unscoped_package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_scoped_package_version_from_recycle_bin::Builder {
            get_scoped_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_scope: package_scope.into(),
                unscoped_package_name: unscoped_package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Restore a package version with an npm scope from the recycle bin to its feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_scope`: Scope of the package (the 'scope' part of @scope/name)."]
        #[doc = "* `unscoped_package_name`: Name of the package (the 'name' part of @scope/name)."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn restore_scoped_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::NpmRecycleBinPackageVersionDetails>,
            feed_id: impl Into<String>,
            package_scope: impl Into<String>,
            unscoped_package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> restore_scoped_package_version_from_recycle_bin::Builder {
            restore_scoped_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_scope: package_scope.into(),
                unscoped_package_name: unscoped_package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete a package version with an npm scope from the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_scope`: Scope of the package (the 'scope' part of @scope/name)."]
        #[doc = "* `unscoped_package_name`: Name of the package (the 'name' part of @scope/name)."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_scoped_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_scope: impl Into<String>,
            unscoped_package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_scoped_package_version_from_recycle_bin::Builder {
            delete_scoped_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_scope: package_scope.into(),
                unscoped_package_name: unscoped_package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Get information about an unscoped package version in the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version_from_recycle_bin::Builder {
            get_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Restore a package version without an npm scope from the recycle bin to its feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn restore_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::NpmRecycleBinPackageVersionDetails>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> restore_package_version_from_recycle_bin::Builder {
            restore_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete a package version without an npm scope from the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_package_version_from_recycle_bin::Builder {
            delete_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete or restore several package versions from the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Information about the packages to update, the operation to perform, and its associated data."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_recycle_bin_packages(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::NpmPackagesBatchRequest>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_recycle_bin_packages::Builder {
            update_recycle_bin_packages::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
    }
    pub mod get_scoped_package_version {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_scope: String,
            pub(crate) unscoped_package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/@{}/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_scope,
                            &this.unscoped_package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_scoped_package {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PackageVersionDetails,
            pub(crate) feed_id: String,
            pub(crate) package_scope: String,
            pub(crate) unscoped_package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/@{}/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_scope,
                            &this.unscoped_package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod unpublish_scoped_package {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_scope: String,
            pub(crate) unscoped_package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/@{}/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_scope,
                            &this.unscoped_package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_package {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PackageVersionDetails,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod unpublish_package {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_upstreaming_behavior {
        use super::models;
        type Response = models::UpstreamingBehavior;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_scope: String,
            pub(crate) unscoped_package_name: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/packages/@{}/{}/upstreaming",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_scope,
                            &this.unscoped_package_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpstreamingBehavior =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod set_scoped_upstreaming_behavior {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_scope: String,
            pub(crate) unscoped_package_name: String,
            pub(crate) body: models::UpstreamingBehavior,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/packages/@{}/{}/upstreaming",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_scope,
                            &this.unscoped_package_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod download_scoped_package {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_scope: String,
            pub(crate) unscoped_package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/npm/packages/@{}/{}/versions/{}/content" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_scope , & this . unscoped_package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_scoped_package_readme {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_scope: String,
            pub(crate) unscoped_package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/npm/packages/@{}/{}/versions/{}/readme" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_scope , & this . unscoped_package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_scoped_package_upstreaming_behavior {
        use super::models;
        type Response = models::UpstreamingBehavior;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/packages/{}/upstreaming",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpstreamingBehavior =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod set_upstreaming_behavior {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) body: models::UpstreamingBehavior,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/packages/{}/upstreaming",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod download_package {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/packages/{}/versions/{}/content",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_readme {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/packages/{}/versions/{}/readme",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_packages {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::NpmPackagesBatchRequest,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/packagesbatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_scoped_package_version_from_recycle_bin {
        use super::models;
        type Response = models::NpmPackageVersionDeletionState;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_scope: String,
            pub(crate) unscoped_package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/npm/RecycleBin/packages/@{}/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_scope , & this . unscoped_package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NpmPackageVersionDeletionState =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod restore_scoped_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::NpmRecycleBinPackageVersionDetails,
            pub(crate) feed_id: String,
            pub(crate) package_scope: String,
            pub(crate) unscoped_package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/npm/RecycleBin/packages/@{}/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_scope , & this . unscoped_package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_scoped_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_scope: String,
            pub(crate) unscoped_package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/npm/RecycleBin/packages/@{}/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_scope , & this . unscoped_package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version_from_recycle_bin {
        use super::models;
        type Response = models::NpmPackageVersionDeletionState;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/npm/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NpmPackageVersionDeletionState =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod restore_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::NpmRecycleBinPackageVersionDetails,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/npm/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/npm/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_recycle_bin_packages {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::NpmPackagesBatchRequest,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/npm/RecycleBin/PackagesBatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod nu_get {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the upstreaming behavior of a package within the context of a feed"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: The name or id of the feed"]
        #[doc = "* `package_name`: The name of the package"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_upstreaming_behavior(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            project: impl Into<String>,
        ) -> get_upstreaming_behavior::Builder {
            get_upstreaming_behavior::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                project: project.into(),
            }
        }
        #[doc = "Set the upstreaming behavior of a package within the context of a feed\n\nThe package does not need to necessarily exist in the feed prior to setting the behavior.\nThis assists with packages that are not yet ingested from an upstream, yet the feed owner wants\nto apply a specific behavior on the first ingestion."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: The name or id of the feed"]
        #[doc = "* `package_name`: The name of the package"]
        #[doc = "* `body`: The behavior to apply to the package within the scope of the feed"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn set_upstreaming_behavior(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            body: impl Into<models::UpstreamingBehavior>,
            project: impl Into<String>,
        ) -> set_upstreaming_behavior::Builder {
            set_upstreaming_behavior::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get information about a package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version::Builder {
            get_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
                show_deleted: None,
            }
        }
        #[doc = "Set mutable state on a package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: New state to apply to the referenced package."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package to update."]
        #[doc = "* `package_version`: Version of the package to update."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_package_version(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PackageVersionDetails>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> update_package_version::Builder {
            update_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Send a package version from the feed to its paired recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package to delete."]
        #[doc = "* `package_version`: Version of the package to delete."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_package_version(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_package_version::Builder {
            delete_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Download a package version directly.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn download_package(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> download_package::Builder {
            download_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
                source_protocol_version: None,
            }
        }
        #[doc = "Update several packages from a single feed in a single request. The updates to the packages do not happen atomically.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Information about the packages to update, the operation to perform, and its associated data."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_package_versions(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::NuGetPackagesBatchRequest>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_package_versions::Builder {
            update_package_versions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "View a package version's deletion/recycled status\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version_from_recycle_bin::Builder {
            get_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Restore a package version from a feed's recycle bin back into the active feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Set the 'Deleted' member to 'false' to apply the restore operation"]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn restore_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::NuGetRecycleBinPackageVersionDetails>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> restore_package_version_from_recycle_bin::Builder {
            restore_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete a package version from a feed's recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_package_version_from_recycle_bin::Builder {
            delete_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete or restore several package versions from the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Information about the packages to update, the operation to perform, and its associated data. <c>Operation</c> must be <c>PermanentDelete</c> or <c>RestoreToFeed</c>"]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_recycle_bin_package_versions(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::NuGetPackagesBatchRequest>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_recycle_bin_package_versions::Builder {
            update_recycle_bin_package_versions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
    }
    pub mod get_upstreaming_behavior {
        use super::models;
        type Response = models::UpstreamingBehavior;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/nuget/packages/{}/upstreaming",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpstreamingBehavior =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod set_upstreaming_behavior {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) body: models::UpstreamingBehavior,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/nuget/packages/{}/upstreaming",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
            pub(crate) show_deleted: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to include deleted packages in the response."]
            pub fn show_deleted(mut self, show_deleted: bool) -> Self {
                self.show_deleted = Some(show_deleted);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/nuget/packages/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(show_deleted) = &this.show_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("showDeleted", &show_deleted.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_package_version {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PackageVersionDetails,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/nuget/packages/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_package_version {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/nuget/packages/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod download_package {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
            pub(crate) source_protocol_version: Option<String>,
        }
        impl Builder {
            #[doc = "Unused"]
            pub fn source_protocol_version(
                mut self,
                source_protocol_version: impl Into<String>,
            ) -> Self {
                self.source_protocol_version = Some(source_protocol_version.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/nuget/packages/{}/versions/{}/content" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(source_protocol_version) = &this.source_protocol_version {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("sourceProtocolVersion", source_protocol_version);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_package_versions {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::NuGetPackagesBatchRequest,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/nuget/packagesbatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version_from_recycle_bin {
        use super::models;
        type Response = models::NuGetPackageVersionDeletionState;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/nuget/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NuGetPackageVersionDeletionState =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod restore_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::NuGetRecycleBinPackageVersionDetails,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/nuget/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/nuget/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_recycle_bin_package_versions {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::NuGetPackagesBatchRequest,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/nuget/RecycleBin/packagesBatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod python {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the upstreaming behavior of a package within the context of a feed"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: The name or id of the feed"]
        #[doc = "* `package_name`: The name of the package"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_upstreaming_behavior(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            project: impl Into<String>,
        ) -> get_upstreaming_behavior::Builder {
            get_upstreaming_behavior::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                project: project.into(),
            }
        }
        #[doc = "Set the upstreaming behavior of a package within the context of a feed\n\nThe package does not need to necessarily exist in the feed prior to setting the behavior.\nThis assists with packages that are not yet ingested from an upstream, yet the feed owner wants\nto apply a specific behavior on the first ingestion."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: The name or id of the feed"]
        #[doc = "* `package_name`: The name of the package"]
        #[doc = "* `body`: The behavior to apply to the package within the scope of the feed"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn set_upstreaming_behavior(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            body: impl Into<models::UpstreamingBehavior>,
            project: impl Into<String>,
        ) -> set_upstreaming_behavior::Builder {
            set_upstreaming_behavior::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get information about a package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version::Builder {
            get_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
                show_deleted: None,
            }
        }
        #[doc = "Update state for a package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Details to be updated."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_package_version(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PackageVersionDetails>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> update_package_version::Builder {
            update_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete a package version, moving it to the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_package_version(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_package_version::Builder {
            delete_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Download a python package file directly. This API is intended for manual UI download options, not for programmatic access and scripting.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `file_name`: Name of the file in the package"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn download_package(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            file_name: impl Into<String>,
            project: impl Into<String>,
        ) -> download_package::Builder {
            download_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                file_name: file_name.into(),
                project: project.into(),
            }
        }
        #[doc = "Update several packages from a single feed in a single request. The updates to the packages do not happen atomically.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Information about the packages to update, the operation to perform, and its associated data."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_package_versions(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PyPiPackagesBatchRequest>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_package_versions::Builder {
            update_package_versions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Get information about a package version in the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version_from_recycle_bin::Builder {
            get_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Restore a package version from the recycle bin to its associated feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Set the 'Deleted' state to 'false' to restore the package to its feed."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn restore_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PyPiRecycleBinPackageVersionDetails>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> restore_package_version_from_recycle_bin::Builder {
            restore_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete a package version from the feed, moving it to the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_package_version_from_recycle_bin::Builder {
            delete_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete or restore several package versions from the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Information about the packages to update, the operation to perform, and its associated data. <c>Operation</c> must be <c>PermanentDelete</c> or <c>RestoreToFeed</c>"]
        #[doc = "* `feed_id`: Feed which contains the packages to update."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_recycle_bin_package_versions(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PyPiPackagesBatchRequest>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_recycle_bin_package_versions::Builder {
            update_recycle_bin_package_versions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
    }
    pub mod get_upstreaming_behavior {
        use super::models;
        type Response = models::UpstreamingBehavior;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/pypi/packages/{}/upstreaming",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UpstreamingBehavior =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod set_upstreaming_behavior {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) body: models::UpstreamingBehavior,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/pypi/packages/{}/upstreaming",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
            pub(crate) show_deleted: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to show information for deleted package versions."]
            pub fn show_deleted(mut self, show_deleted: bool) -> Self {
                self.show_deleted = Some(show_deleted);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/pypi/packages/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(show_deleted) = &this.show_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("showDeleted", &show_deleted.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_package_version {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PackageVersionDetails,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/pypi/packages/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_package_version {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/pypi/packages/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod download_package {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) file_name: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/pypi/packages/{}/versions/{}/{}/content" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version , & this . file_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_package_versions {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PyPiPackagesBatchRequest,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/pypi/packagesbatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version_from_recycle_bin {
        use super::models;
        type Response = models::PyPiPackageVersionDeletionState;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/pypi/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PyPiPackageVersionDeletionState =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod restore_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PyPiRecycleBinPackageVersionDetails,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/pypi/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/pypi/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_recycle_bin_package_versions {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PyPiPackagesBatchRequest,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/pypi/RecycleBin/packagesBatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod universal {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Show information about a package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version::Builder {
            get_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
                show_deleted: None,
            }
        }
        #[doc = "Update information for a package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_package_version(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PackageVersionDetails>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> update_package_version::Builder {
            update_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete a package version from a feed's recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_package_version(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_package_version::Builder {
            delete_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Update several packages from a single feed in a single request. The updates to the packages do not happen atomically.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Information about the packages to update, the operation to perform, and its associated data."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_package_versions(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UPackPackagesBatchRequest>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_package_versions::Builder {
            update_package_versions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Get information about a package version in the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version_from_recycle_bin::Builder {
            get_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Restore a package version from the recycle bin to its associated feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Set the 'Deleted' property to 'false' to restore the package."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn restore_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UPackRecycleBinPackageVersionDetails>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> restore_package_version_from_recycle_bin::Builder {
            restore_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete a package version from the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `package_name`: Name of the package."]
        #[doc = "* `package_version`: Version of the package."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_package_version_from_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_name: impl Into<String>,
            package_version: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_package_version_from_recycle_bin::Builder {
            delete_package_version_from_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_name: package_name.into(),
                package_version: package_version.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete or restore several package versions from the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Information about the packages to update, the operation to perform, and its associated data. <c>Operation</c> must be <c>PermanentDelete</c> or <c>RestoreToFeed</c>"]
        #[doc = "* `feed_id`: Feed which contains the packages to update."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_recycle_bin_package_versions(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UPackPackagesBatchRequest>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_recycle_bin_package_versions::Builder {
            update_recycle_bin_package_versions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
    }
    pub mod get_package_version {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
            pub(crate) show_deleted: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to show information for deleted versions"]
            pub fn show_deleted(mut self, show_deleted: bool) -> Self {
                self.show_deleted = Some(show_deleted);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/upack/packages/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(show_deleted) = &this.show_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("showDeleted", &show_deleted.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_package_version {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PackageVersionDetails,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/upack/packages/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_package_version {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/upack/packages/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_name,
                            &this.package_version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_package_versions {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::UPackPackagesBatchRequest,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/upack/packagesbatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version_from_recycle_bin {
        use super::models;
        type Response = models::UPackPackageVersionDeletionState;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/upack/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::UPackPackageVersionDeletionState =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod restore_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::UPackRecycleBinPackageVersionDetails,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/upack/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_package_version_from_recycle_bin {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_name: String,
            pub(crate) package_version: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/{}/{}/_apis/packaging/feeds/{}/upack/RecycleBin/packages/{}/versions/{}" , this . client . endpoint () , & this . organization , & this . project , & this . feed_id , & this . package_name , & this . package_version)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_recycle_bin_package_versions {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::UPackPackagesBatchRequest,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}/upack/RecycleBin/packagesBatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            &this
                                .client
                                .token_credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
