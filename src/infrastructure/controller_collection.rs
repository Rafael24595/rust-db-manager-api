use axum::{
    extract::{DefaultBodyLimit, Path},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use rust_db_manager_core::{
    commons::configuration::configuration::Configuration,
    domain::{
        collection::generate_collection_query::GenerateCollectionQuery,
        filter::{collection_query::CollectionQuery, data_base_query::DataBaseQuery},
    },
};

use crate::commons::exception::api_exception::ApiException;

use super::{
    dto::{
        action::definition::dto_action_definition::DTOActionDefinition,
        collection::{
            dto_generate_collection_query::DTOGenerateCollectionQuery,
            dto_rename_collection_query::DTORenameCollectionQuery,
        },
        document::{dto_document_data::DTODocumentData, dto_document_schema::DTODocumentSchema},
        table::{
            definition::dto_table_definition::DTOTableDefinition,
            group::dto_table_data_group::DTOTableDataGroup,
        },
    },
    handler, utils,
};

pub struct ControllerCollection {
}

impl ControllerCollection {
    
    pub fn route(router: Router) -> Router {
        router
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/import", post(Self::import))
            .layer(DefaultBodyLimit::max(52428800 ))
            .route("/api/v1/service/:service/data-base/:data_base/collection", get(Self::find_all))
            .route("/api/v1/service/:service/data-base/:data_base/collection", post(Self::insert))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection", delete(Self::delete))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/metadata", get(Self::metadata))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/information", get(Self::information))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/actions", get(Self::actions))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/schema", get(Self::schema))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/rename", post(Self::rename))
            .route("/api/v1/service/:service/data-base/:data_base/collection/:collection/export", get(Self::export))
            .route_layer(middleware::from_fn(handler::autentication_handler))
    }

    async fn find_all(Path((service, data_base)): Path<(String, String)>) -> Result<Json<Vec<String>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = DataBaseQuery::from(data_base);

        let collections = result.unwrap().collection_find_all(&query).await;
        if let Err(error) = collections {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(collections.unwrap()))
    }

    async fn insert(Path((service, _)): Path<(String, String)>, Json(dto): Json<DTOGenerateCollectionQuery>) -> Result<StatusCode, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = dto.from_dto();
        if let Err(exception) = query {
            return Err(exception.into_response());
        }

        let collection = result.unwrap().collection_create(&query.unwrap()).await;
        if let Err(error) = collection {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(StatusCode::ACCEPTED)
    }

    async fn delete(Path((service, data_base, collection)): Path<(String, String, String)>) -> Result<StatusCode, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = GenerateCollectionQuery::from_collection(data_base, collection);

        let collection = result.unwrap().collection_drop(&query).await;
        if let Err(error) = collection {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(StatusCode::ACCEPTED)
    }

    async fn metadata(Path((service, data_base, collection)): Path<(String, String, String)>) -> Result<Json<Vec<DTOTableDataGroup>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }

        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = CollectionQuery::from(data_base, collection);

        let metadata = result.unwrap().collection_metadata(&query).await;
        if let Err(error) = metadata {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        let dto = metadata.unwrap().iter()
            .map(|g| DTOTableDataGroup::from(g))
            .collect();

        Ok(Json(dto))
    }

    async fn information(Path((service, data_base, collection)): Path<(String, String, String)>) -> Result<Json<Vec<DTOTableDefinition>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }

        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = CollectionQuery::from(data_base, collection);

        let metadata = result.unwrap().collection_information(&query).await;
        if let Err(error) = metadata {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        let dto = metadata.unwrap().iter()
            .map(|d| DTOTableDefinition::from(d))
            .collect();

        Ok(Json(dto))
    }

    async fn actions(Path((service, data_base, collection)): Path<(String, String, String)>) -> Result<Json<Vec<DTOActionDefinition>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }

        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = CollectionQuery::from(data_base, collection);

        let actions = result.unwrap().collection_actions(&query).await;
        if let Err(error) = actions {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        let dto = actions.unwrap().iter()
            .map(|a| DTOActionDefinition::from(a))
            .collect();

        Ok(Json(dto))
    }

    async fn schema(Path((service, data_base, collection)): Path<(String, String, String)>) -> Result<Json<DTODocumentSchema>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = CollectionQuery::from(data_base, collection);

        let schema = result.unwrap().schema(&query).await;
        if let Err(error) = schema {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(DTODocumentSchema::from(&schema.unwrap())))
    }

    async fn rename(Path((service, data_base, collection)): Path<(String, String, String)>, Json(dto): Json<DTORenameCollectionQuery>) -> Result<StatusCode, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = CollectionQuery::from(data_base, collection);

        let documents = result.unwrap().collection_rename(&query, &dto.collection).await;
        if let Err(error) = documents {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(StatusCode::OK)
    }

    async fn export(Path((service, data_base, collection)): Path<(String, String, String)>) -> Result<Json<Vec<DTODocumentData>>, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = CollectionQuery::from(data_base, collection);

        let documents = result.unwrap().collection_export(&query).await;
        if let Err(error) = documents {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(Json(documents.unwrap().iter()
            .map(|d| DTODocumentData::from(d))
            .collect())
        )
    }

    async fn import(Path((service, data_base, collection)): Path<(String, String, String)>, documents: Json<Vec<String>>) -> Result<StatusCode, impl IntoResponse> {
        let o_db_service = Configuration::find_service(&service);
        if o_db_service.is_none() {
            return Err(utils::not_found());
        }
        
        let result = o_db_service.unwrap().instance().await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }

        let query = CollectionQuery::from(data_base, collection);

        let result = result.unwrap().collection_import(&query, documents.to_vec()).await;
        if let Err(error) = result {
            let exception = ApiException::from(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), error);
            return Err(exception.into_response());
        }
    
        Ok(StatusCode::OK)
    }

}