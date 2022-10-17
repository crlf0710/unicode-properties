// @generated
impl AnyProvider for BakedDataProvider {
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        #[cfg(feature = "icu_properties")]
        const GENERALCATEGORYV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::GeneralCategoryV1Marker::KEY.hashed();
        #[allow(clippy::match_single_binding)]
        match key.hashed() {
            #[cfg(feature = "icu_properties")]
            GENERALCATEGORYV1MARKER => props::gc_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            _ => Err(DataErrorKind::MissingDataKey),
        }
        .map_err(|e| e.with_req(key, req))
        .map(|payload| AnyResponse {
            payload: Some(payload),
            metadata: Default::default(),
        })
    }
}
