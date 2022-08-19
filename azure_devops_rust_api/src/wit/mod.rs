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
pub const DEFAULT_ENDPOINT: &str = "https://dev.azure.com";
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
    #[doc = "Set per-call policies."]
    #[must_use]
    pub fn per_call_policies(
        mut self,
        policies: impl Into<Vec<std::sync::Arc<dyn azure_core::Policy>>>,
    ) -> Self {
        self.options = self.options.per_call_policies(policies);
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
    pub fn account_my_work_recent_activity_client(
        &self,
    ) -> account_my_work_recent_activity::Client {
        account_my_work_recent_activity::Client(self.clone())
    }
    pub fn artifact_link_types_client(&self) -> artifact_link_types::Client {
        artifact_link_types::Client(self.clone())
    }
    pub fn artifact_uri_query_client(&self) -> artifact_uri_query::Client {
        artifact_uri_query::Client(self.clone())
    }
    pub fn attachments_client(&self) -> attachments::Client {
        attachments::Client(self.clone())
    }
    pub fn classification_nodes_client(&self) -> classification_nodes::Client {
        classification_nodes::Client(self.clone())
    }
    pub fn comment_reactions_engaged_users_client(
        &self,
    ) -> comment_reactions_engaged_users::Client {
        comment_reactions_engaged_users::Client(self.clone())
    }
    pub fn comments_client(&self) -> comments::Client {
        comments::Client(self.clone())
    }
    pub fn comments_reactions_client(&self) -> comments_reactions::Client {
        comments_reactions::Client(self.clone())
    }
    pub fn comments_versions_client(&self) -> comments_versions::Client {
        comments_versions::Client(self.clone())
    }
    pub fn fields_client(&self) -> fields::Client {
        fields::Client(self.clone())
    }
    pub fn project_process_migration_client(&self) -> project_process_migration::Client {
        project_process_migration::Client(self.clone())
    }
    pub fn queries_client(&self) -> queries::Client {
        queries::Client(self.clone())
    }
    pub fn recyclebin_client(&self) -> recyclebin::Client {
        recyclebin::Client(self.clone())
    }
    pub fn reporting_work_item_links_client(&self) -> reporting_work_item_links::Client {
        reporting_work_item_links::Client(self.clone())
    }
    pub fn reporting_work_item_revisions_client(&self) -> reporting_work_item_revisions::Client {
        reporting_work_item_revisions::Client(self.clone())
    }
    pub fn revisions_client(&self) -> revisions::Client {
        revisions::Client(self.clone())
    }
    pub fn send_mail_client(&self) -> send_mail::Client {
        send_mail::Client(self.clone())
    }
    pub fn tags_client(&self) -> tags::Client {
        tags::Client(self.clone())
    }
    pub fn templates_client(&self) -> templates::Client {
        templates::Client(self.clone())
    }
    pub fn updates_client(&self) -> updates::Client {
        updates::Client(self.clone())
    }
    pub fn wiql_client(&self) -> wiql::Client {
        wiql::Client(self.clone())
    }
    pub fn work_item_icons_client(&self) -> work_item_icons::Client {
        work_item_icons::Client(self.clone())
    }
    pub fn work_item_relation_types_client(&self) -> work_item_relation_types::Client {
        work_item_relation_types::Client(self.clone())
    }
    pub fn work_item_revisions_discussions_client(
        &self,
    ) -> work_item_revisions_discussions::Client {
        work_item_revisions_discussions::Client(self.clone())
    }
    pub fn work_item_transitions_client(&self) -> work_item_transitions::Client {
        work_item_transitions::Client(self.clone())
    }
    pub fn work_item_type_categories_client(&self) -> work_item_type_categories::Client {
        work_item_type_categories::Client(self.clone())
    }
    pub fn work_item_type_states_client(&self) -> work_item_type_states::Client {
        work_item_type_states::Client(self.clone())
    }
    pub fn work_item_types_client(&self) -> work_item_types::Client {
        work_item_types::Client(self.clone())
    }
    pub fn work_item_types_field_client(&self) -> work_item_types_field::Client {
        work_item_types_field::Client(self.clone())
    }
    pub fn work_items_client(&self) -> work_items::Client {
        work_items::Client(self.clone())
    }
}
pub mod classification_nodes {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets root classification nodes or list of classification nodes for a given list of nodes ids, for a given project. In case ids parameter is supplied you will  get list of classification nodes for those ids. Otherwise you will get root classification nodes for this project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `ids`: Comma separated integer classification nodes ids. It's not required, if you want root nodes."]
        pub fn get_classification_nodes(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            ids: impl Into<String>,
        ) -> get_classification_nodes::Builder {
            get_classification_nodes::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                ids: ids.into(),
                depth: None,
                error_policy: None,
            }
        }
        #[doc = "Gets root classification nodes under the project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_root_nodes(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get_root_nodes::Builder {
            get_root_nodes::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                depth: None,
            }
        }
        #[doc = "Gets the classification node for a given node path."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `structure_group`: Structure group of the classification node, area or iteration."]
        #[doc = "* `path`: Path of the classification node."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            structure_group: impl Into<String>,
            path: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                structure_group: structure_group.into(),
                path: path.into(),
                depth: None,
            }
        }
        #[doc = "Create new or update an existing classification node."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Node to create or update."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `structure_group`: Structure group of the classification node, area or iteration."]
        #[doc = "* `path`: Path of the classification node."]
        pub fn create_or_update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemClassificationNode>,
            project: impl Into<String>,
            structure_group: impl Into<String>,
            path: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                structure_group: structure_group.into(),
                path: path.into(),
            }
        }
        #[doc = "Update an existing classification node."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Node to create or update."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `structure_group`: Structure group of the classification node, area or iteration."]
        #[doc = "* `path`: Path of the classification node."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemClassificationNode>,
            project: impl Into<String>,
            structure_group: impl Into<String>,
            path: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                structure_group: structure_group.into(),
                path: path.into(),
            }
        }
        #[doc = "Delete an existing classification node."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `structure_group`: Structure group of the classification node, area or iteration."]
        #[doc = "* `path`: Path of the classification node."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            structure_group: impl Into<String>,
            path: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                structure_group: structure_group.into(),
                path: path.into(),
                reclassify_id: None,
            }
        }
    }
    pub mod get_classification_nodes {
        use super::models;
        type Response = models::WorkItemClassificationNodeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) ids: String,
            pub(crate) depth: Option<i32>,
            pub(crate) error_policy: Option<String>,
        }
        impl Builder {
            #[doc = "Depth of children to fetch."]
            pub fn depth(mut self, depth: i32) -> Self {
                self.depth = Some(depth);
                self
            }
            #[doc = "Flag to handle errors in getting some nodes. Possible options are Fail and Omit."]
            pub fn error_policy(mut self, error_policy: impl Into<String>) -> Self {
                self.error_policy = Some(error_policy.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/classificationnodes?ids={}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.ids
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let ids = &this.ids;
                        req.url_mut().query_pairs_mut().append_pair("ids", ids);
                        if let Some(depth) = &this.depth {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$depth", &depth.to_string());
                        }
                        if let Some(error_policy) = &this.error_policy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("errorPolicy", error_policy);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemClassificationNodeList =
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
    pub mod get_root_nodes {
        use super::models;
        type Response = models::WorkItemClassificationNodeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) depth: Option<i32>,
        }
        impl Builder {
            #[doc = "Depth of children to fetch."]
            pub fn depth(mut self, depth: i32) -> Self {
                self.depth = Some(depth);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/classificationnodes",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(depth) = &this.depth {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$depth", &depth.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemClassificationNodeList =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemClassificationNode;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) structure_group: String,
            pub(crate) path: String,
            pub(crate) depth: Option<i32>,
        }
        impl Builder {
            #[doc = "Depth of children to fetch."]
            pub fn depth(mut self, depth: i32) -> Self {
                self.depth = Some(depth);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/classificationnodes/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.structure_group,
                            &this.path
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(depth) = &this.depth {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$depth", &depth.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemClassificationNode =
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
    pub mod create_or_update {
        use super::models;
        type Response = models::WorkItemClassificationNode;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WorkItemClassificationNode,
            pub(crate) project: String,
            pub(crate) structure_group: String,
            pub(crate) path: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/classificationnodes/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.structure_group,
                            &this.path
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemClassificationNode =
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
    pub mod update {
        use super::models;
        type Response = models::WorkItemClassificationNode;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WorkItemClassificationNode,
            pub(crate) project: String,
            pub(crate) structure_group: String,
            pub(crate) path: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/classificationnodes/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.structure_group,
                            &this.path
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemClassificationNode =
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) structure_group: String,
            pub(crate) path: String,
            pub(crate) reclassify_id: Option<i32>,
        }
        impl Builder {
            #[doc = "Id of the target classification node for reclassification."]
            pub fn reclassify_id(mut self, reclassify_id: i32) -> Self {
                self.reclassify_id = Some(reclassify_id);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/classificationnodes/{}/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.structure_group,
                            &this.path
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(reclassify_id) = &this.reclassify_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$reclassifyId", &reclassify_id.to_string());
                        }
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
}
pub mod queries {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Searches all queries the user has access to in the current project"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `filter`: The text to filter the queries with."]
        pub fn search_queries(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            filter: impl Into<String>,
        ) -> search_queries::Builder {
            search_queries::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                filter: filter.into(),
                top: None,
                expand: None,
                include_deleted: None,
            }
        }
        #[doc = "Gets the root queries and their children"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                expand: None,
                depth: None,
                include_deleted: None,
            }
        }
        #[doc = "Retrieves an individual query and its children"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `query`: ID or path of the query."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            query: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                query: query.into(),
                expand: None,
                depth: None,
                include_deleted: None,
                use_iso_date_format: None,
            }
        }
        #[doc = "Creates a query, or moves a query.\n\nLearn more about Work Item Query Language (WIQL) syntax [here](https://docs.microsoft.com/en-us/vsts/collaborate/wiql-syntax?toc=/vsts/work/track/toc.json&bc=/vsts/work/track/breadcrumb/toc.json&view=vsts)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The query to create."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `query`: The parent id or path under which the query is to be created."]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::QueryHierarchyItem>,
            project: impl Into<String>,
            query: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                query: query.into(),
                validate_wiql_only: None,
            }
        }
        #[doc = "Update a query or a folder. This allows you to update, rename and move queries and folders."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The query to update."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `query`: The ID or path for the query to update."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::QueryHierarchyItem>,
            project: impl Into<String>,
            query: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                query: query.into(),
                undelete_descendants: None,
            }
        }
        #[doc = "Delete a query or a folder. This deletes any permission change on the deleted query or folder and any of its descendants if it is a folder. It is important to note that the deleted permission changes cannot be recovered upon undeleting the query or folder."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `query`: ID or path of the query or folder to delete."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            query: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                query: query.into(),
            }
        }
        #[doc = "Gets a list of queries by ids (Maximum 1000)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_queries_batch(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::QueryBatchGetRequest>,
            project: impl Into<String>,
        ) -> get_queries_batch::Builder {
            get_queries_batch::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod search_queries {
        use super::models;
        type Response = models::QueryHierarchyItemsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) filter: String,
            pub(crate) top: Option<i32>,
            pub(crate) expand: Option<String>,
            pub(crate) include_deleted: Option<bool>,
        }
        impl Builder {
            #[doc = "The number of queries to return (Default is 50 and maximum is 200)."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Include deleted queries and folders"]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/queries?$filter={}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.filter
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let filter = &this.filter;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("$filter", filter);
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$includeDeleted", &include_deleted.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryHierarchyItemsResult =
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
    pub mod list {
        use super::models;
        type Response = models::QueryHierarchyItemList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) expand: Option<String>,
            pub(crate) depth: Option<i32>,
            pub(crate) include_deleted: Option<bool>,
        }
        impl Builder {
            #[doc = "Include the query string (wiql), clauses, query result columns, and sort options in the results."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "In the folder of queries, return child queries and folders to this depth."]
            pub fn depth(mut self, depth: i32) -> Self {
                self.depth = Some(depth);
                self
            }
            #[doc = "Include deleted queries and folders"]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/queries",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        if let Some(depth) = &this.depth {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$depth", &depth.to_string());
                        }
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$includeDeleted", &include_deleted.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryHierarchyItemList =
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
    pub mod get {
        use super::models;
        type Response = models::QueryHierarchyItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) query: String,
            pub(crate) expand: Option<String>,
            pub(crate) depth: Option<i32>,
            pub(crate) include_deleted: Option<bool>,
            pub(crate) use_iso_date_format: Option<bool>,
        }
        impl Builder {
            #[doc = "Include the query string (wiql), clauses, query result columns, and sort options in the results."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "In the folder of queries, return child queries and folders to this depth."]
            pub fn depth(mut self, depth: i32) -> Self {
                self.depth = Some(depth);
                self
            }
            #[doc = "Include deleted queries and folders"]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            #[doc = "DateTime query clauses will be formatted using a ISO 8601 compliant format"]
            pub fn use_iso_date_format(mut self, use_iso_date_format: bool) -> Self {
                self.use_iso_date_format = Some(use_iso_date_format);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/queries/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.query
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        if let Some(depth) = &this.depth {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$depth", &depth.to_string());
                        }
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$includeDeleted", &include_deleted.to_string());
                        }
                        if let Some(use_iso_date_format) = &this.use_iso_date_format {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$useIsoDateFormat", &use_iso_date_format.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryHierarchyItem =
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
    pub mod create {
        use super::models;
        type Response = models::QueryHierarchyItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::QueryHierarchyItem,
            pub(crate) project: String,
            pub(crate) query: String,
            pub(crate) validate_wiql_only: Option<bool>,
        }
        impl Builder {
            #[doc = "If you only want to validate your WIQL query without actually creating one, set it to true. Default is false."]
            pub fn validate_wiql_only(mut self, validate_wiql_only: bool) -> Self {
                self.validate_wiql_only = Some(validate_wiql_only);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/queries/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.query
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(validate_wiql_only) = &this.validate_wiql_only {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("validateWiqlOnly", &validate_wiql_only.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryHierarchyItem =
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
    pub mod update {
        use super::models;
        type Response = models::QueryHierarchyItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::QueryHierarchyItem,
            pub(crate) project: String,
            pub(crate) query: String,
            pub(crate) undelete_descendants: Option<bool>,
        }
        impl Builder {
            #[doc = "Undelete the children of this folder. It is important to note that this will not bring back the permission changes that were previously applied to the descendants."]
            pub fn undelete_descendants(mut self, undelete_descendants: bool) -> Self {
                self.undelete_descendants = Some(undelete_descendants);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/queries/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.query
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(undelete_descendants) = &this.undelete_descendants {
                            req.url_mut().query_pairs_mut().append_pair(
                                "$undeleteDescendants",
                                &undelete_descendants.to_string(),
                            );
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryHierarchyItem =
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) query: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/queries/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.query
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
    pub mod get_queries_batch {
        use super::models;
        type Response = models::QueryHierarchyItemList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::QueryBatchGetRequest,
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
                            "{}/{}/{}/_apis/wit/queriesbatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::QueryHierarchyItemList =
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
}
pub mod recyclebin {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the work items from the recycle bin, whose IDs have been specified in the parameters"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `ids`: Comma separated list of IDs of the deleted work items to be returned"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_deleted_work_items(
            &self,
            organization: impl Into<String>,
            ids: impl Into<String>,
            project: impl Into<String>,
        ) -> get_deleted_work_items::Builder {
            get_deleted_work_items::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                ids: ids.into(),
                project: project.into(),
            }
        }
        #[doc = "Gets a list of the IDs and the URLs of the deleted the work items in the Recycle Bin."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_deleted_work_item_shallow_references(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get_deleted_work_item_shallow_references::Builder {
            get_deleted_work_item_shallow_references::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
            }
        }
        #[doc = "Gets a deleted work item from Recycle Bin."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: ID of the work item to be returned"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            id: i32,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
                project: project.into(),
            }
        }
        #[doc = "Restores the deleted work item from Recycle Bin."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Paylod with instructions to update the IsDeleted flag to false"]
        #[doc = "* `id`: ID of the work item to be restored"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn restore_work_item(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemDeleteUpdate>,
            id: i32,
            project: impl Into<String>,
        ) -> restore_work_item::Builder {
            restore_work_item::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                id,
                project: project.into(),
            }
        }
        #[doc = "Destroys the specified work item permanently from the Recycle Bin. This action can not be undone."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: ID of the work item to be destroyed permanently"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn destroy_work_item(
            &self,
            organization: impl Into<String>,
            id: i32,
            project: impl Into<String>,
        ) -> destroy_work_item::Builder {
            destroy_work_item::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
                project: project.into(),
            }
        }
    }
    pub mod get_deleted_work_items {
        use super::models;
        type Response = models::WorkItemDeleteReferenceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) ids: String,
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
                            "{}/{}/{}/_apis/wit/recyclebin?ids={}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.ids
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let ids = &this.ids;
                        req.url_mut().query_pairs_mut().append_pair("ids", ids);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemDeleteReferenceList =
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
    pub mod get_deleted_work_item_shallow_references {
        use super::models;
        type Response = models::WorkItemDeleteShallowReferenceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
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
                            "{}/{}/{}/_apis/wit/recyclebin",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemDeleteShallowReferenceList =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemDelete;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
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
                            "{}/{}/{}/_apis/wit/recyclebin/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemDelete =
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
    pub mod restore_work_item {
        use super::models;
        type Response = models::WorkItemDelete;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WorkItemDeleteUpdate,
            pub(crate) id: i32,
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
                            "{}/{}/{}/_apis/wit/recyclebin/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemDelete =
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
    pub mod destroy_work_item {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
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
                            "{}/{}/{}/_apis/wit/recyclebin/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
}
pub mod comments {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns a list of work item comments, pageable."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `work_item_id`: Id of a work item to get comments for."]
        pub fn get_comments(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            work_item_id: i32,
        ) -> get_comments::Builder {
            get_comments::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                work_item_id,
                top: None,
                continuation_token: None,
                include_deleted: None,
                expand: None,
                order: None,
            }
        }
        #[doc = "Returns a list of work item comments by ids."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `work_item_id`: Id of a work item to get comments for."]
        #[doc = "* `ids`: Comma-separated list of comment ids to return."]
        pub fn get_comments_batch(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            work_item_id: i32,
            ids: impl Into<String>,
        ) -> get_comments_batch::Builder {
            get_comments_batch::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                work_item_id,
                ids: ids.into(),
                include_deleted: None,
                expand: None,
            }
        }
        #[doc = "Add a comment on a work item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Comment create request."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `work_item_id`: Id of a work item."]
        pub fn add(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CommentCreate>,
            project: impl Into<String>,
            work_item_id: i32,
        ) -> add::Builder {
            add::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                work_item_id,
            }
        }
        #[doc = "Returns a work item comment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `work_item_id`: Id of a work item to get the comment."]
        #[doc = "* `comment_id`: Id of the comment to return."]
        pub fn get_comment(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            work_item_id: i32,
            comment_id: i32,
        ) -> get_comment::Builder {
            get_comment::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                work_item_id,
                comment_id,
                include_deleted: None,
                expand: None,
            }
        }
        #[doc = "Update a comment on a work item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Comment update request."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `work_item_id`: Id of a work item."]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CommentUpdate>,
            project: impl Into<String>,
            work_item_id: i32,
            comment_id: i32,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                work_item_id,
                comment_id,
            }
        }
        #[doc = "Delete a comment on a work item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `work_item_id`: Id of a work item."]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            work_item_id: i32,
            comment_id: i32,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                work_item_id,
                comment_id,
            }
        }
    }
    pub mod get_comments {
        use super::models;
        type Response = models::CommentList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) top: Option<i32>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) include_deleted: Option<bool>,
            pub(crate) expand: Option<String>,
            pub(crate) order: Option<String>,
        }
        impl Builder {
            #[doc = "Max number of comments to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Used to query for the next page of comments."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Specify if the deleted comments should be retrieved."]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            #[doc = "Specifies the additional data retrieval options for work item comments."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Order in which the comments should be returned."]
            pub fn order(mut self, order: impl Into<String>) -> Self {
                self.order = Some(order.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments?",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDeleted", &include_deleted.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        if let Some(order) = &this.order {
                            req.url_mut().query_pairs_mut().append_pair("order", order);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CommentList =
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
    pub mod get_comments_batch {
        use super::models;
        type Response = models::CommentList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) ids: String,
            pub(crate) include_deleted: Option<bool>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Specify if the deleted comments should be retrieved."]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            #[doc = "Specifies the additional data retrieval options for work item comments."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let ids = &this.ids;
                        req.url_mut().query_pairs_mut().append_pair("ids", ids);
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDeleted", &include_deleted.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CommentList =
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
    pub mod add {
        use super::models;
        type Response = models::Comment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::CommentCreate,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::Comment = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_comment {
        use super::models;
        type Response = models::Comment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) comment_id: i32,
            pub(crate) include_deleted: Option<bool>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Specify if the deleted comment should be retrieved."]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            #[doc = "Specifies the additional data retrieval options for work item comments."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id,
                            &this.comment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDeleted", &include_deleted.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Comment = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        type Response = models::Comment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::CommentUpdate,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) comment_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id,
                            &this.comment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::Comment = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) comment_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id,
                            &this.comment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
}
pub mod artifact_link_types {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the list of work item tracking outbound artifact link types."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkArtifactLinkList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/wit/artifactlinktypes",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkArtifactLinkList =
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
}
pub mod work_item_icons {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of all work item icons."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Get a work item icon given the friendly name and icon color."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `icon`: The name of the icon"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get(
            &self,
            icon: impl Into<String>,
            organization: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                icon: icon.into(),
                organization: organization.into(),
                color: None,
                v: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemIconList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/wit/workitemicons",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemIconList =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemIcon;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) icon: String,
            pub(crate) organization: String,
            pub(crate) color: Option<String>,
            pub(crate) v: Option<i32>,
        }
        impl Builder {
            #[doc = "The 6-digit hex color for the icon"]
            pub fn color(mut self, color: impl Into<String>) -> Self {
                self.color = Some(color.into());
                self
            }
            #[doc = "The version of the icon (used only for cache invalidation)"]
            pub fn v(mut self, v: i32) -> Self {
                self.v = Some(v);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/wit/workitemicons/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.icon
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(color) = &this.color {
                            req.url_mut().query_pairs_mut().append_pair("color", color);
                        }
                        if let Some(v) = &this.v {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("v", &v.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemIcon =
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
}
pub mod work_item_relation_types {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the work item relation types."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
        #[doc = "Gets the work item relation type definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `relation`: The relation name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            relation: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                relation: relation.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemRelationTypeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/wit/workitemrelationtypes",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemRelationTypeList =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemRelationType;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) relation: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/wit/workitemrelationtypes/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.relation
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemRelationType =
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
}
pub mod work_item_transitions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the next state on the given work item IDs."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `ids`: list of work item ids"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            ids: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                ids: ids.into(),
                action: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemNextStateOnTransitionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) ids: String,
            pub(crate) action: Option<String>,
        }
        impl Builder {
            #[doc = "possible actions. Currently only supports checkin"]
            pub fn action(mut self, action: impl Into<String>) -> Self {
                self.action = Some(action.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/wit/workitemtransitions",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let ids = &this.ids;
                        req.url_mut().query_pairs_mut().append_pair("ids", ids);
                        if let Some(action) = &this.action {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("action", action);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemNextStateOnTransitionList =
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
}
pub mod account_my_work_recent_activity {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets recent work item activities"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn list(&self, organization: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::AccountRecentActivityWorkItemModel2List;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/work/accountmyworkrecentactivity",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::AccountRecentActivityWorkItemModel2List =
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
}
pub mod artifact_uri_query {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Queries work items linked to a given list of artifact URI."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Defines a list of artifact URI for querying work items."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn query(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ArtifactUriQuery>,
            project: impl Into<String>,
        ) -> query::Builder {
            query::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod query {
        use super::models;
        type Response = models::ArtifactUriQueryResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ArtifactUriQuery,
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
                            "{}/{}/{}/_apis/wit/artifacturiquery",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::ArtifactUriQueryResult =
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
}
pub mod attachments {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Uploads an attachment.\n\nOn accounts with higher attachment upload limits (>130MB), you will need to use chunked upload.\nTo upload an attachment in multiple chunks, you first need to [**Start a Chunked Upload**](#start_a_chunked_upload) and then follow the example from the **Upload Chunk** section."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Stream to upload"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<String>,
            project: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                file_name: None,
                upload_type: None,
                area_path: None,
            }
        }
        #[doc = "Downloads an attachment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: Attachment ID"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            id: impl Into<String>,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id: id.into(),
                project: project.into(),
                file_name: None,
                download: None,
            }
        }
        #[doc = "Uploads an attachment chunk.\n\nBefore performing [**Upload a Chunk**](#upload_a_chunk), make sure to have an attachment id returned in **Start a Chunked Upload** example on **Create** section. Specify the byte range of the chunk using Content-Length. For example: \"Content - Length\": \"bytes 0 - 39999 / 50000\" for the first 40000 bytes of a 50000 byte file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Stream to upload"]
        #[doc = "* `id`: The id of the attachment"]
        #[doc = "* `content_range_header`: starting and ending byte positions for chunked file upload, format is \"Content-Range\": \"bytes 0-10000/50000\""]
        #[doc = "* `project`: Project ID or project name"]
        pub fn upload_chunk(
            &self,
            organization: impl Into<String>,
            body: impl Into<String>,
            id: impl Into<String>,
            content_range_header: impl Into<String>,
            project: impl Into<String>,
        ) -> upload_chunk::Builder {
            upload_chunk::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                id: id.into(),
                content_range_header: content_range_header.into(),
                project: project.into(),
                file_name: None,
            }
        }
    }
    pub mod create {
        use super::models;
        type Response = models::AttachmentReference;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: String,
            pub(crate) project: String,
            pub(crate) file_name: Option<String>,
            pub(crate) upload_type: Option<String>,
            pub(crate) area_path: Option<String>,
        }
        impl Builder {
            #[doc = "The name of the file"]
            pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
                self.file_name = Some(file_name.into());
                self
            }
            #[doc = "Attachment upload type: Simple or Chunked"]
            pub fn upload_type(mut self, upload_type: impl Into<String>) -> Self {
                self.upload_type = Some(upload_type.into());
                self
            }
            #[doc = "Target project Area Path"]
            pub fn area_path(mut self, area_path: impl Into<String>) -> Self {
                self.area_path = Some(area_path.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/attachments",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(file_name) = &this.file_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fileName", file_name);
                        }
                        if let Some(upload_type) = &this.upload_type {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("uploadType", upload_type);
                        }
                        if let Some(area_path) = &this.area_path {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("areaPath", area_path);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AttachmentReference =
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
    pub mod get {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: String,
            pub(crate) project: String,
            pub(crate) file_name: Option<String>,
            pub(crate) download: Option<bool>,
        }
        impl Builder {
            #[doc = "Name of the file"]
            pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
                self.file_name = Some(file_name.into());
                self
            }
            #[doc = "If set to <c>true</c> always download attachment"]
            pub fn download(mut self, download: bool) -> Self {
                self.download = Some(download);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/attachments/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(file_name) = &this.file_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fileName", file_name);
                        }
                        if let Some(download) = &this.download {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("download", &download.to_string());
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
    pub mod upload_chunk {
        use super::models;
        type Response = models::AttachmentReference;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: String,
            pub(crate) id: String,
            pub(crate) content_range_header: String,
            pub(crate) project: String,
            pub(crate) file_name: Option<String>,
        }
        impl Builder {
            pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
                self.file_name = Some(file_name.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/attachments/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.insert_header("contentrangeheader", &this.content_range_header);
                        if let Some(file_name) = &this.file_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fileName", file_name);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AttachmentReference =
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
}
pub mod fields {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns information for all fields. The project ID/name parameter is optional."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                expand: None,
            }
        }
        #[doc = "Create a new field."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: New field definition"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemField>,
            project: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Gets information on a specific field."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `field_name_or_ref_name`: Field simple name or reference name"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            field_name_or_ref_name: impl Into<String>,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                field_name_or_ref_name: field_name_or_ref_name.into(),
                project: project.into(),
            }
        }
        #[doc = "Update a field."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Payload contains desired value of the field's properties"]
        #[doc = "* `field_name_or_ref_name`: Name/reference name of the field to be updated"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::UpdateWorkItemField>,
            field_name_or_ref_name: impl Into<String>,
            project: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                field_name_or_ref_name: field_name_or_ref_name.into(),
                project: project.into(),
            }
        }
        #[doc = "Deletes the field. To undelete a filed, see \"Update Field\" API."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `field_name_or_ref_name`: Field simple name or reference name"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            field_name_or_ref_name: impl Into<String>,
            project: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                field_name_or_ref_name: field_name_or_ref_name.into(),
                project: project.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemFieldList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Use ExtensionFields to include extension fields, otherwise exclude them. Unless the feature flag for this parameter is enabled, extension fields are always included."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/fields",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemFieldList =
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
    pub mod create {
        use super::models;
        type Response = models::WorkItemField;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WorkItemField,
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
                            "{}/{}/{}/_apis/wit/fields",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemField =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemField;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) field_name_or_ref_name: String,
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
                            "{}/{}/{}/_apis/wit/fields/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.field_name_or_ref_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemField =
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
    pub mod update {
        use super::models;
        type Response = models::WorkItemField;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::UpdateWorkItemField,
            pub(crate) field_name_or_ref_name: String,
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
                            "{}/{}/{}/_apis/wit/fields/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.field_name_or_ref_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemField =
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) field_name_or_ref_name: String,
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
                            "{}/{}/{}/_apis/wit/fields/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.field_name_or_ref_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
}
pub mod project_process_migration {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Migrates a project to a different process within the same OOB type. For example, you can only migrate a project from agile/custom-agile to agile/custom-agile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn migrate_projects_process(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ProcessIdModel>,
            project: impl Into<String>,
        ) -> migrate_projects_process::Builder {
            migrate_projects_process::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod migrate_projects_process {
        use super::models;
        type Response = models::ProcessMigrationResultModel;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ProcessIdModel,
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
                            "{}/{}/{}/_apis/wit/projectprocessmigration",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::ProcessMigrationResultModel =
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
}
pub mod reporting_work_item_links {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a batch of work item links"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                link_types: None,
                types: None,
                continuation_token: None,
                start_date_time: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ReportingWorkItemLinksBatch;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) link_types: Option<String>,
            pub(crate) types: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) start_date_time: Option<time::OffsetDateTime>,
        }
        impl Builder {
            #[doc = "A list of types to filter the results to specific link types. Omit this parameter to get work item links of all link types."]
            pub fn link_types(mut self, link_types: impl Into<String>) -> Self {
                self.link_types = Some(link_types.into());
                self
            }
            #[doc = "A list of types to filter the results to specific work item types. Omit this parameter to get work item links of all work item types."]
            pub fn types(mut self, types: impl Into<String>) -> Self {
                self.types = Some(types.into());
                self
            }
            #[doc = "Specifies the continuation token to start the batch from. Omit this parameter to get the first batch of links."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Date/time to use as a starting point for link changes. Only link changes that occurred after that date/time will be returned. Cannot be used in conjunction with 'watermark' parameter."]
            pub fn start_date_time(
                mut self,
                start_date_time: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.start_date_time = Some(start_date_time.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/reporting/workitemlinks",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(link_types) = &this.link_types {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("linkTypes", link_types);
                        }
                        if let Some(types) = &this.types {
                            req.url_mut().query_pairs_mut().append_pair("types", types);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(start_date_time) = &this.start_date_time {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("startDateTime", &start_date_time.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReportingWorkItemLinksBatch =
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
}
pub mod reporting_work_item_revisions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a batch of work item revisions with the option of including deleted items"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn read_reporting_revisions_get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> read_reporting_revisions_get::Builder {
            read_reporting_revisions_get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                fields: None,
                types: None,
                continuation_token: None,
                start_date_time: None,
                include_identity_ref: None,
                include_deleted: None,
                include_tag_ref: None,
                include_latest_only: None,
                expand: None,
                include_discussion_changes_only: None,
                max_page_size: None,
            }
        }
        #[doc = "Get a batch of work item revisions. This request may be used if your list of fields is large enough that it may run the URL over the length limit."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: An object that contains request settings: field filter, type filter, identity format"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn read_reporting_revisions_post(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::ReportingWorkItemRevisionsFilter>,
            project: impl Into<String>,
        ) -> read_reporting_revisions_post::Builder {
            read_reporting_revisions_post::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                continuation_token: None,
                start_date_time: None,
                expand: None,
            }
        }
    }
    pub mod read_reporting_revisions_get {
        use super::models;
        type Response = models::ReportingWorkItemRevisionsBatch;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) fields: Option<String>,
            pub(crate) types: Option<String>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) start_date_time: Option<time::OffsetDateTime>,
            pub(crate) include_identity_ref: Option<bool>,
            pub(crate) include_deleted: Option<bool>,
            pub(crate) include_tag_ref: Option<bool>,
            pub(crate) include_latest_only: Option<bool>,
            pub(crate) expand: Option<String>,
            pub(crate) include_discussion_changes_only: Option<bool>,
            pub(crate) max_page_size: Option<i32>,
        }
        impl Builder {
            #[doc = "A list of fields to return in work item revisions. Omit this parameter to get all reportable fields."]
            pub fn fields(mut self, fields: impl Into<String>) -> Self {
                self.fields = Some(fields.into());
                self
            }
            #[doc = "A list of types to filter the results to specific work item types. Omit this parameter to get work item revisions of all work item types."]
            pub fn types(mut self, types: impl Into<String>) -> Self {
                self.types = Some(types.into());
                self
            }
            #[doc = "Specifies the watermark to start the batch from. Omit this parameter to get the first batch of revisions."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Date/time to use as a starting point for revisions, all revisions will occur after this date/time. Cannot be used in conjunction with 'watermark' parameter."]
            pub fn start_date_time(
                mut self,
                start_date_time: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.start_date_time = Some(start_date_time.into());
                self
            }
            #[doc = "Return an identity reference instead of a string value for identity fields."]
            pub fn include_identity_ref(mut self, include_identity_ref: bool) -> Self {
                self.include_identity_ref = Some(include_identity_ref);
                self
            }
            #[doc = "Specify if the deleted item should be returned."]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            #[doc = "Specify if the tag objects should be returned for System.Tags field."]
            pub fn include_tag_ref(mut self, include_tag_ref: bool) -> Self {
                self.include_tag_ref = Some(include_tag_ref);
                self
            }
            #[doc = "Return only the latest revisions of work items, skipping all historical revisions"]
            pub fn include_latest_only(mut self, include_latest_only: bool) -> Self {
                self.include_latest_only = Some(include_latest_only);
                self
            }
            #[doc = "Return all the fields in work item revisions, including long text fields which are not returned by default"]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "Return only the those revisions of work items, where only history field was changed"]
            pub fn include_discussion_changes_only(
                mut self,
                include_discussion_changes_only: bool,
            ) -> Self {
                self.include_discussion_changes_only = Some(include_discussion_changes_only);
                self
            }
            #[doc = "The maximum number of results to return in this batch"]
            pub fn max_page_size(mut self, max_page_size: i32) -> Self {
                self.max_page_size = Some(max_page_size);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/reporting/workitemrevisions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(fields) = &this.fields {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fields", fields);
                        }
                        if let Some(types) = &this.types {
                            req.url_mut().query_pairs_mut().append_pair("types", types);
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(start_date_time) = &this.start_date_time {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("startDateTime", &start_date_time.to_string());
                        }
                        if let Some(include_identity_ref) = &this.include_identity_ref {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeIdentityRef",
                                &include_identity_ref.to_string(),
                            );
                        }
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDeleted", &include_deleted.to_string());
                        }
                        if let Some(include_tag_ref) = &this.include_tag_ref {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeTagRef", &include_tag_ref.to_string());
                        }
                        if let Some(include_latest_only) = &this.include_latest_only {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeLatestOnly", &include_latest_only.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        if let Some(include_discussion_changes_only) =
                            &this.include_discussion_changes_only
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeDiscussionChangesOnly",
                                &include_discussion_changes_only.to_string(),
                            );
                        }
                        if let Some(max_page_size) = &this.max_page_size {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$maxPageSize", &max_page_size.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReportingWorkItemRevisionsBatch =
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
    pub mod read_reporting_revisions_post {
        use super::models;
        type Response = models::ReportingWorkItemRevisionsBatch;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::ReportingWorkItemRevisionsFilter,
            pub(crate) project: String,
            pub(crate) continuation_token: Option<String>,
            pub(crate) start_date_time: Option<time::OffsetDateTime>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Specifies the watermark to start the batch from. Omit this parameter to get the first batch of revisions."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "Date/time to use as a starting point for revisions, all revisions will occur after this date/time. Cannot be used in conjunction with 'watermark' parameter."]
            pub fn start_date_time(
                mut self,
                start_date_time: impl Into<time::OffsetDateTime>,
            ) -> Self {
                self.start_date_time = Some(start_date_time.into());
                self
            }
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/reporting/workitemrevisions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(start_date_time) = &this.start_date_time {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("startDateTime", &start_date_time.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReportingWorkItemRevisionsBatch =
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
}
pub mod work_item_revisions_discussions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn read_reporting_discussions(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> read_reporting_discussions::Builder {
            read_reporting_discussions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                continuation_token: None,
                max_page_size: None,
            }
        }
    }
    pub mod read_reporting_discussions {
        use super::models;
        type Response = models::ReportingWorkItemRevisionsBatch;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) continuation_token: Option<String>,
            pub(crate) max_page_size: Option<i32>,
        }
        impl Builder {
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn max_page_size(mut self, max_page_size: i32) -> Self {
                self.max_page_size = Some(max_page_size);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/reporting/workItemRevisions/discussions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(max_page_size) = &this.max_page_size {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$maxPageSize", &max_page_size.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReportingWorkItemRevisionsBatch =
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
}
pub mod send_mail {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "RESTful method to send mail for selected/queried work items."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn send_mail(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::SendMailBody>,
            project: impl Into<String>,
        ) -> send_mail::Builder {
            send_mail::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod send_mail {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::SendMailBody,
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
                            "{}/{}/{}/_apis/wit/sendmail",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
pub mod tags {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            tag_id_or_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                tag_id_or_name: tag_id_or_name.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemTagDefinition>,
            project: impl Into<String>,
            tag_id_or_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                tag_id_or_name: tag_id_or_name.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            tag_id_or_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                tag_id_or_name: tag_id_or_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemTagDefinitionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
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
                            "{}/{}/{}/_apis/wit/tags",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemTagDefinitionList =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemTagDefinition;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) tag_id_or_name: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/tags/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.tag_id_or_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemTagDefinition =
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
    pub mod update {
        use super::models;
        type Response = models::WorkItemTagDefinition;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WorkItemTagDefinition,
            pub(crate) project: String,
            pub(crate) tag_id_or_name: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/tags/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.tag_id_or_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemTagDefinition =
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) tag_id_or_name: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/tags/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.tag_id_or_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
}
pub mod work_items {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns a list of work items (Maximum 200)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `ids`: The comma-separated list of requested work item ids. (Maximum 200 ids allowed)."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            ids: impl Into<String>,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                ids: ids.into(),
                project: project.into(),
                fields: None,
                as_of: None,
                expand: None,
                error_policy: None,
            }
        }
        #[doc = "Returns a single work item from a template."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `type_`: The work item type name"]
        pub fn get_work_item_template(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            type_: impl Into<String>,
        ) -> get_work_item_template::Builder {
            get_work_item_template::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                type_: type_.into(),
                fields: None,
                as_of: None,
                expand: None,
            }
        }
        #[doc = "Creates a single work item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The JSON Patch document representing the work item"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `type_`: The work item type of the work item to create"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::JsonPatchDocument>,
            project: impl Into<String>,
            type_: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                type_: type_.into(),
                validate_only: None,
                bypass_rules: None,
                suppress_notifications: None,
                expand: None,
            }
        }
        #[doc = "Returns a single work item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: The work item id"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_work_item(
            &self,
            organization: impl Into<String>,
            id: i32,
            project: impl Into<String>,
        ) -> get_work_item::Builder {
            get_work_item::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
                project: project.into(),
                fields: None,
                as_of: None,
                expand: None,
            }
        }
        #[doc = "Updates a single work item."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The JSON Patch document representing the update"]
        #[doc = "* `id`: The id of the work item to update"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::JsonPatchDocument>,
            id: i32,
            project: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                id,
                project: project.into(),
                validate_only: None,
                bypass_rules: None,
                suppress_notifications: None,
                expand: None,
            }
        }
        #[doc = "Deletes the specified work item and sends it to the Recycle Bin, so that it can be restored back, if required. Optionally, if the destroy parameter has been set to true, it destroys the work item permanently. WARNING: If the destroy parameter is set to true, work items deleted by this command will NOT go to recycle-bin and there is no way to restore/recover them after deletion. It is recommended NOT to use this parameter. If you do, please use this parameter with extreme caution."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: ID of the work item to be deleted"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            id: i32,
            project: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
                project: project.into(),
                destroy: None,
            }
        }
        #[doc = "Gets work items for a list of work item ids (Maximum 200)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_work_items_batch(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemBatchGetRequest>,
            project: impl Into<String>,
        ) -> get_work_items_batch::Builder {
            get_work_items_batch::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) ids: String,
            pub(crate) project: String,
            pub(crate) fields: Option<String>,
            pub(crate) as_of: Option<time::OffsetDateTime>,
            pub(crate) expand: Option<String>,
            pub(crate) error_policy: Option<String>,
        }
        impl Builder {
            #[doc = "Comma-separated list of requested fields"]
            pub fn fields(mut self, fields: impl Into<String>) -> Self {
                self.fields = Some(fields.into());
                self
            }
            #[doc = "AsOf UTC date time string"]
            pub fn as_of(mut self, as_of: impl Into<time::OffsetDateTime>) -> Self {
                self.as_of = Some(as_of.into());
                self
            }
            #[doc = "The expand parameters for work item attributes. Possible options are { None, Relations, Fields, Links, All }."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            #[doc = "The flag to control error policy in a bulk get work items request. Possible options are {Fail, Omit}."]
            pub fn error_policy(mut self, error_policy: impl Into<String>) -> Self {
                self.error_policy = Some(error_policy.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitems",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let ids = &this.ids;
                        req.url_mut().query_pairs_mut().append_pair("ids", ids);
                        if let Some(fields) = &this.fields {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fields", fields);
                        }
                        if let Some(as_of) = &this.as_of {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("asOf", &as_of.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        if let Some(error_policy) = &this.error_policy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("errorPolicy", error_policy);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemList =
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
    pub mod get_work_item_template {
        use super::models;
        type Response = models::WorkItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) type_: String,
            pub(crate) fields: Option<String>,
            pub(crate) as_of: Option<time::OffsetDateTime>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Comma-separated list of requested fields"]
            pub fn fields(mut self, fields: impl Into<String>) -> Self {
                self.fields = Some(fields.into());
                self
            }
            #[doc = "AsOf UTC date time string"]
            pub fn as_of(mut self, as_of: impl Into<time::OffsetDateTime>) -> Self {
                self.as_of = Some(as_of.into());
                self
            }
            #[doc = "The expand parameters for work item attributes. Possible options are { None, Relations, Fields, Links, All }."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitems/${}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.type_
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(fields) = &this.fields {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fields", fields);
                        }
                        if let Some(as_of) = &this.as_of {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("asOf", &as_of.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItem =
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
    pub mod create {
        use super::models;
        type Response = models::WorkItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::JsonPatchDocument,
            pub(crate) project: String,
            pub(crate) type_: String,
            pub(crate) validate_only: Option<bool>,
            pub(crate) bypass_rules: Option<bool>,
            pub(crate) suppress_notifications: Option<bool>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Indicate if you only want to validate the changes without saving the work item"]
            pub fn validate_only(mut self, validate_only: bool) -> Self {
                self.validate_only = Some(validate_only);
                self
            }
            #[doc = "Do not enforce the work item type rules on this update"]
            pub fn bypass_rules(mut self, bypass_rules: bool) -> Self {
                self.bypass_rules = Some(bypass_rules);
                self
            }
            #[doc = "Do not fire any notifications for this change"]
            pub fn suppress_notifications(mut self, suppress_notifications: bool) -> Self {
                self.suppress_notifications = Some(suppress_notifications);
                self
            }
            #[doc = "The expand parameters for work item attributes. Possible options are { None, Relations, Fields, Links, All }."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitems/${}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.type_
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(validate_only) = &this.validate_only {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("validateOnly", &validate_only.to_string());
                        }
                        if let Some(bypass_rules) = &this.bypass_rules {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("bypassRules", &bypass_rules.to_string());
                        }
                        if let Some(suppress_notifications) = &this.suppress_notifications {
                            req.url_mut().query_pairs_mut().append_pair(
                                "suppressNotifications",
                                &suppress_notifications.to_string(),
                            );
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItem =
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
    pub mod get_work_item {
        use super::models;
        type Response = models::WorkItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
            pub(crate) project: String,
            pub(crate) fields: Option<String>,
            pub(crate) as_of: Option<time::OffsetDateTime>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Comma-separated list of requested fields"]
            pub fn fields(mut self, fields: impl Into<String>) -> Self {
                self.fields = Some(fields.into());
                self
            }
            #[doc = "AsOf UTC date time string"]
            pub fn as_of(mut self, as_of: impl Into<time::OffsetDateTime>) -> Self {
                self.as_of = Some(as_of.into());
                self
            }
            #[doc = "The expand parameters for work item attributes. Possible options are { None, Relations, Fields, Links, All }."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitems/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(fields) = &this.fields {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("fields", fields);
                        }
                        if let Some(as_of) = &this.as_of {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("asOf", &as_of.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItem =
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
    pub mod update {
        use super::models;
        type Response = models::WorkItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::JsonPatchDocument,
            pub(crate) id: i32,
            pub(crate) project: String,
            pub(crate) validate_only: Option<bool>,
            pub(crate) bypass_rules: Option<bool>,
            pub(crate) suppress_notifications: Option<bool>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Indicate if you only want to validate the changes without saving the work item"]
            pub fn validate_only(mut self, validate_only: bool) -> Self {
                self.validate_only = Some(validate_only);
                self
            }
            #[doc = "Do not enforce the work item type rules on this update"]
            pub fn bypass_rules(mut self, bypass_rules: bool) -> Self {
                self.bypass_rules = Some(bypass_rules);
                self
            }
            #[doc = "Do not fire any notifications for this change"]
            pub fn suppress_notifications(mut self, suppress_notifications: bool) -> Self {
                self.suppress_notifications = Some(suppress_notifications);
                self
            }
            #[doc = "The expand parameters for work item attributes. Possible options are { None, Relations, Fields, Links, All }."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitems/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(validate_only) = &this.validate_only {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("validateOnly", &validate_only.to_string());
                        }
                        if let Some(bypass_rules) = &this.bypass_rules {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("bypassRules", &bypass_rules.to_string());
                        }
                        if let Some(suppress_notifications) = &this.suppress_notifications {
                            req.url_mut().query_pairs_mut().append_pair(
                                "suppressNotifications",
                                &suppress_notifications.to_string(),
                            );
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItem =
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
    pub mod delete {
        use super::models;
        type Response = models::WorkItemDelete;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
            pub(crate) project: String,
            pub(crate) destroy: Option<bool>,
        }
        impl Builder {
            #[doc = "Optional parameter, if set to true, the work item is deleted permanently. Please note: the destroy action is PERMANENT and cannot be undone."]
            pub fn destroy(mut self, destroy: bool) -> Self {
                self.destroy = Some(destroy);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitems/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(destroy) = &this.destroy {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("destroy", &destroy.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemDelete =
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
    pub mod get_work_items_batch {
        use super::models;
        type Response = models::WorkItemList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WorkItemBatchGetRequest,
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
                            "{}/{}/{}/_apis/wit/workitemsbatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemList =
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
}
pub mod revisions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the list of fully hydrated work item revisions, paged."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            id: i32,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
                project: project.into(),
                top: None,
                skip: None,
                expand: None,
            }
        }
        #[doc = "Returns a fully hydrated work item for the requested revision"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            id: i32,
            revision_number: i32,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
                revision_number,
                project: project.into(),
                expand: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
            pub(crate) project: String,
            pub(crate) top: Option<i32>,
            pub(crate) skip: Option<i32>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/revisions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemList =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
            pub(crate) revision_number: i32,
            pub(crate) project: String,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/revisions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id,
                            &this.revision_number
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItem =
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
}
pub mod updates {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns a the deltas between work item revisions"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            id: i32,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
                project: project.into(),
                top: None,
                skip: None,
            }
        }
        #[doc = "Returns a single update for a work item"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            id: i32,
            update_number: i32,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id,
                update_number,
                project: project.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemUpdateList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
            pub(crate) project: String,
            pub(crate) top: Option<i32>,
            pub(crate) skip: Option<i32>,
        }
        impl Builder {
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/updates",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemUpdateList =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemUpdate;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: i32,
            pub(crate) update_number: i32,
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
                            "{}/{}/{}/_apis/wit/workItems/{}/updates/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.id,
                            &this.update_number
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemUpdate =
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
}
pub mod comments_reactions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets reactions of a comment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `work_item_id`: WorkItem ID"]
        #[doc = "* `comment_id`: Comment ID"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            work_item_id: i32,
            comment_id: i32,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                work_item_id,
                comment_id,
            }
        }
        #[doc = "Adds a new reaction to a comment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `work_item_id`: WorkItem ID"]
        #[doc = "* `comment_id`: Comment ID"]
        #[doc = "* `reaction_type`: Type of the reaction"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            work_item_id: i32,
            comment_id: i32,
            reaction_type: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                work_item_id,
                comment_id,
                reaction_type: reaction_type.into(),
            }
        }
        #[doc = "Deletes an existing reaction on a comment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `work_item_id`: WorkItem ID"]
        #[doc = "* `comment_id`: Comment ID"]
        #[doc = "* `reaction_type`: Type of the reaction"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            work_item_id: i32,
            comment_id: i32,
            reaction_type: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                work_item_id,
                comment_id,
                reaction_type: reaction_type.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::CommentReactionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) comment_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments/{}/reactions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id,
                            &this.comment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::CommentReactionList =
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
    pub mod create {
        use super::models;
        type Response = models::CommentReaction;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) comment_id: i32,
            pub(crate) reaction_type: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments/{}/reactions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id,
                            &this.comment_id,
                            &this.reaction_type
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::CommentReaction =
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
    pub mod delete {
        use super::models;
        type Response = models::CommentReaction;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) comment_id: i32,
            pub(crate) reaction_type: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments/{}/reactions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id,
                            &this.comment_id,
                            &this.reaction_type
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::CommentReaction =
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
}
pub mod comment_reactions_engaged_users {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get users who reacted on the comment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `work_item_id`: WorkItem ID."]
        #[doc = "* `comment_id`: Comment ID."]
        #[doc = "* `reaction_type`: Type of the reaction."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            work_item_id: i32,
            comment_id: i32,
            reaction_type: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                work_item_id,
                comment_id,
                reaction_type: reaction_type.into(),
                top: None,
                skip: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::IdentityRefList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) comment_id: i32,
            pub(crate) reaction_type: String,
            pub(crate) top: Option<i32>,
            pub(crate) skip: Option<i32>,
        }
        impl Builder {
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments/{}/reactions/{}/users",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id,
                            &this.comment_id,
                            &this.reaction_type
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IdentityRefList =
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
}
pub mod comments_versions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            work_item_id: i32,
            comment_id: i32,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                work_item_id,
                comment_id,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            work_item_id: i32,
            comment_id: i32,
            version: i32,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                work_item_id,
                comment_id,
                version,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::CommentVersionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) comment_id: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments/{}/versions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id,
                            &this.comment_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::CommentVersionList =
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
    pub mod get {
        use super::models;
        type Response = models::CommentVersion;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) work_item_id: i32,
            pub(crate) comment_id: i32,
            pub(crate) version: i32,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workItems/{}/comments/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.work_item_id,
                            &this.comment_id,
                            &this.version
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::CommentVersion =
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
}
pub mod work_item_type_categories {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all work item type categories."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
            }
        }
        #[doc = "Get specific work item type category by name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `category`: The category name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            category: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                category: category.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemTypeCategoryList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
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
                            "{}/{}/{}/_apis/wit/workitemtypecategories",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemTypeCategoryList =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemTypeCategory;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) category: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitemtypecategories/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.category
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemTypeCategory =
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
}
pub mod work_item_types {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the list of work item types"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
            }
        }
        #[doc = "Returns a work item type definition."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `type_`: Work item type name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            type_: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                type_: type_.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemTypeList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
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
                            "{}/{}/{}/_apis/wit/workitemtypes",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemTypeList =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemType;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) type_: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitemtypes/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.type_
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemType =
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
}
pub mod work_item_types_field {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of fields for a work item type with detailed references."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `type_`: Work item type."]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            type_: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                type_: type_.into(),
                expand: None,
            }
        }
        #[doc = "Get a field for a work item type with detailed references."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `type_`: Work item type."]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            type_: impl Into<String>,
            field: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                type_: type_.into(),
                field: field.into(),
                expand: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemTypeFieldWithReferencesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) type_: String,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Expand level for the API response. Properties: to include allowedvalues, default value, isRequired etc. as a part of response; None: to skip these properties."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitemtypes/{}/fields",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.type_
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemTypeFieldWithReferencesList =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemTypeFieldWithReferences;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) type_: String,
            pub(crate) field: String,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Expand level for the API response. Properties: to include allowedvalues, default value, isRequired etc. as a part of response; None: to skip these properties."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitemtypes/{}/fields/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.type_,
                            &this.field
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(expand) = &this.expand {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemTypeFieldWithReferences =
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
}
pub mod work_item_type_states {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns the state names and colors for a work item type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `type_`: The state name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            type_: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                type_: type_.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemStateColorList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) type_: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/wit/workitemtypes/{}/states",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.type_
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemStateColorList =
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
}
pub mod templates {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets template"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
                workitemtypename: None,
            }
        }
        #[doc = "Creates a template"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Template contents"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemTemplate>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
            }
        }
        #[doc = "Gets the template with specified id"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        #[doc = "* `template_id`: Template Id"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
            template_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
                template_id: template_id.into(),
            }
        }
        #[doc = "Replace template contents"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Template contents to replace with"]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        #[doc = "* `template_id`: Template id"]
        pub fn replace_template(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemTemplate>,
            project: impl Into<String>,
            team: impl Into<String>,
            template_id: impl Into<String>,
        ) -> replace_template::Builder {
            replace_template::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
                template_id: template_id.into(),
            }
        }
        #[doc = "Deletes the template with given id"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        #[doc = "* `template_id`: Template id"]
        pub fn delete(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
            template_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                team: team.into(),
                template_id: template_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::WorkItemTemplateReferenceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) workitemtypename: Option<String>,
        }
        impl Builder {
            #[doc = "Optional, When specified returns templates for given Work item type."]
            pub fn workitemtypename(mut self, workitemtypename: impl Into<String>) -> Self {
                self.workitemtypename = Some(workitemtypename.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/wit/templates",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(workitemtypename) = &this.workitemtypename {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("workitemtypename", workitemtypename);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemTemplateReferenceList =
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
    pub mod create {
        use super::models;
        type Response = models::WorkItemTemplate;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WorkItemTemplate,
            pub(crate) project: String,
            pub(crate) team: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/wit/templates",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemTemplate =
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
    pub mod get {
        use super::models;
        type Response = models::WorkItemTemplate;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) template_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/wit/templates/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.template_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemTemplate =
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
    pub mod replace_template {
        use super::models;
        type Response = models::WorkItemTemplate;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WorkItemTemplate,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) template_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/wit/templates/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.template_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
                                let rsp_value: models::WorkItemTemplate =
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
    pub mod delete {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) template_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/wit/templates/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.template_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
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
}
pub mod wiql {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the results of the query given its WIQL."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The query containing the WIQL."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn query_by_wiql(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Wiql>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> query_by_wiql::Builder {
            query_by_wiql::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
                team: team.into(),
                time_precision: None,
                top: None,
            }
        }
        #[doc = "Gets the results of the query given the query ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: The query ID."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn query_by_id(
            &self,
            organization: impl Into<String>,
            id: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> query_by_id::Builder {
            query_by_id::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id: id.into(),
                project: project.into(),
                team: team.into(),
                time_precision: None,
                top: None,
            }
        }
        #[doc = "Gets the results of the query given the query ID."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `id`: The query ID."]
        #[doc = "* `project`: Project ID or project name"]
        #[doc = "* `team`: Team ID or team name"]
        pub fn get(
            &self,
            organization: impl Into<String>,
            id: impl Into<String>,
            project: impl Into<String>,
            team: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                id: id.into(),
                project: project.into(),
                team: team.into(),
                time_precision: None,
                top: None,
            }
        }
    }
    pub mod query_by_wiql {
        use super::models;
        type Response = models::WorkItemQueryResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::Wiql,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) time_precision: Option<bool>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Whether or not to use time precision."]
            pub fn time_precision(mut self, time_precision: bool) -> Self {
                self.time_precision = Some(time_precision);
                self
            }
            #[doc = "The max number of results to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/wit/wiql",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(time_precision) = &this.time_precision {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("timePrecision", &time_precision.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemQueryResult =
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
    pub mod query_by_id {
        use super::models;
        type Response = models::WorkItemQueryResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) time_precision: Option<bool>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Whether or not to use time precision."]
            pub fn time_precision(mut self, time_precision: bool) -> Self {
                self.time_precision = Some(time_precision);
                self
            }
            #[doc = "The max number of results to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/wit/wiql/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(time_precision) = &this.time_precision {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("timePrecision", &time_precision.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WorkItemQueryResult =
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
    pub mod get {
        use super::models;
        type Response = i32;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) id: String,
            pub(crate) project: String,
            pub(crate) team: String,
            pub(crate) time_precision: Option<bool>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "Whether or not to use time precision."]
            pub fn time_precision(mut self, time_precision: bool) -> Self {
                self.time_precision = Some(time_precision);
                self
            }
            #[doc = "The max number of results to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/{}/_apis/wit/wiql/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.team,
                            &this.id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(time_precision) = &this.time_precision {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("timePrecision", &time_precision.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: i32 = serde_json::from_slice(&rsp_body)?;
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
}
