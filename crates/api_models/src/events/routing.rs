use common_utils::events::{ApiEventMetric, ApiEventsType};

use crate::routing::{
    LinkedRoutingConfigRetrieveResponse, MerchantRoutingAlgorithm, ProfileDefaultRoutingConfig,
    RoutingAlgorithmId, RoutingConfigRequest, RoutingDictionaryRecord, RoutingKind,
    RoutingLinkWrapper, RoutingPayloadWrapper, RoutingRetrieveLinkQuery,
    RoutingRetrieveLinkQueryWrapper, RoutingRetrieveQuery, SuccessBasedRoutingConfig,
    SuccessBasedRoutingPayloadWrapper, SuccessBasedRoutingUpdateConfigQuery,
    ToggleDynamicRoutingQuery, ToggleDynamicRoutingWrapper,
};

impl ApiEventMetric for RoutingKind {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for MerchantRoutingAlgorithm {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for RoutingAlgorithmId {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for RoutingDictionaryRecord {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for LinkedRoutingConfigRetrieveResponse {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for RoutingPayloadWrapper {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}
impl ApiEventMetric for ProfileDefaultRoutingConfig {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for RoutingRetrieveQuery {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for RoutingConfigRequest {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for RoutingRetrieveLinkQuery {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for RoutingLinkWrapper {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}
impl ApiEventMetric for RoutingRetrieveLinkQueryWrapper {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for ToggleDynamicRoutingQuery {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for SuccessBasedRoutingConfig {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for SuccessBasedRoutingPayloadWrapper {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for ToggleDynamicRoutingWrapper {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}

impl ApiEventMetric for SuccessBasedRoutingUpdateConfigQuery {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Routing)
    }
}
