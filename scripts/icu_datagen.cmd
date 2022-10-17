icu4x-datagen -W --pretty --fingerprint --insert-feature-gates --use-separate-crates --cldr-tag latest --icuexport-tag release-71-1 --format mod -l und -k props/Emoji@1 props/EComp@1 props/EPres@1 props/EBase@1 props/EMod@1 -o icu_tables\emoji_data
icu4x-datagen -W --pretty --fingerprint --insert-feature-gates --use-separate-crates --cldr-tag latest --icuexport-tag release-71-1 --format mod -l und -k props/gc@1 -o icu_tables\general_category_data
echo /// The version of [Unicode](http://www.unicode.org/) > icu_tables\version.rs
echo /// that this version of unicode-properties is based on. >> icu_tables\version.rs
echo pub const UNICODE_VERSION: (u64, u64, u64) = (15, 0, 0); >> icu_tables\version.rs
