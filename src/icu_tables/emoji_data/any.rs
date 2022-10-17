// @generated
impl AnyProvider for BakedDataProvider {
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        #[cfg(feature = "icu_properties")]
        const EMOJICOMPONENTV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiComponentV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EMOJIMODIFIERBASEV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiModifierBaseV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EMOJIMODIFIERV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiModifierV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EMOJIPRESENTATIONV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiPresentationV1Marker::KEY.hashed();
        #[cfg(feature = "icu_properties")]
        const EMOJIV1MARKER: ::icu_provider::DataKeyHash =
            ::icu_properties::provider::EmojiV1Marker::KEY.hashed();
        #[allow(clippy::match_single_binding)]
        match key.hashed() {
            #[cfg(feature = "icu_properties")]
            EMOJICOMPONENTV1MARKER => props::ecomp_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EMOJIMODIFIERBASEV1MARKER => props::ebase_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EMOJIMODIFIERV1MARKER => props::emod_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EMOJIPRESENTATIONV1MARKER => props::epres_v1::DATA
                .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                .copied()
                .map(AnyPayload::from_static_ref)
                .ok_or(DataErrorKind::MissingLocale),
            #[cfg(feature = "icu_properties")]
            EMOJIV1MARKER => props::emoji_v1::DATA
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
