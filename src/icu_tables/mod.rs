#[cfg(feature = "emoji")]
mod emoji_data;

#[cfg(feature = "emoji")]
pub(crate) mod emoji {

    #[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
    #[non_exhaustive]
    /// The emoji character properties of a character.
    pub enum EmojiStatus {
        /// `Emoji=NO`, `Emoji_Component=NO`
        NonEmoji,
        /// `Emoji=NO`, `Emoji_Component=YES`
        NonEmojiButEmojiComponent,
        /// `Emoji=YES`, `Emoji_Component=NO`;`Emoji_Presentation=YES`
        EmojiPresentation,
        /// `Emoji=YES`, `Emoji_Component=NO`;`Emoji_Modifier_Base=YES`
        EmojiModifierBase,
        /// `Emoji=YES`, `Emoji_Component=NO`;`Emoji_Presentation=YES`, `Emoji_Modifier_Base=YES`
        EmojiPresentationAndModifierBase,
        /// `Emoji=YES`, `Emoji_Component=NO`
        EmojiOther,
        /// `Emoji=YES`, `Emoji_Component=YES`;`Emoji_Presentation=YES`
        EmojiPresentationAndEmojiComponent,
        /// `Emoji=YES`, `Emoji_Component=YES`;`Emoji_Presentation=YES`, `Emoji_Modifier=YES`
        EmojiPresentationAndModifierAndEmojiComponent,
        /// `Emoji=YES`, `Emoji_Component=YES`
        EmojiOtherAndEmojiComponent,
    }

    use icu_properties::sets::{self, CodePointSetData};
    use once_cell::unsync::Lazy;

    thread_local! {
        static EMOJI_CPSD: Lazy<CodePointSetData> = Lazy::new(|| {
            sets::load_emoji(&super::emoji_data::BakedDataProvider)
                .expect("The data should be valid")
        });

        static ECOMP_CPSD: Lazy<CodePointSetData> = Lazy::new(|| {
            sets::load_emoji_component(&super::emoji_data::BakedDataProvider)
                .expect("The data should be valid")
        });
    }

    #[inline]
    pub(crate) fn is_emoji_char(c: char) -> bool {
        EMOJI_CPSD.with(|emoji_cpsd| {
            let emoji_cpsd = (*emoji_cpsd).as_borrowed();
            emoji_cpsd.contains(c)
        })
    }
    #[inline]
    pub(crate) fn is_emoji_component(c: char) -> bool {
        ECOMP_CPSD.with(|ecomp_cpsd| {
            let ecomp_cpsd = (*ecomp_cpsd).as_borrowed();
            ecomp_cpsd.contains(c)
        })
    }

    #[inline]
    pub(crate) fn is_emoji_char_or_emoji_component(c: char) -> bool {
        let is_emoji = EMOJI_CPSD.with(|emoji_cpsd| {
            let emoji_cpsd = (*emoji_cpsd).as_borrowed();
            emoji_cpsd.contains(c)
        });
        if is_emoji {
            return true;
        }
        let is_component = ECOMP_CPSD.with(|ecomp_cpsd| {
            let ecomp_cpsd = (*ecomp_cpsd).as_borrowed();
            ecomp_cpsd.contains(c)
        });
        is_component
    }

    thread_local! {
        static EPRES_CPSD: Lazy<CodePointSetData> = Lazy::new(|| {
            sets::load_emoji_presentation(&super::emoji_data::BakedDataProvider)
                .expect("The data should be valid")
        });

        static EBASE_CPSD: Lazy<CodePointSetData> = Lazy::new(|| {
            sets::load_emoji_modifier_base(&super::emoji_data::BakedDataProvider)
                .expect("The data should be valid")
        });

        static EMODIFIER_CPSD: Lazy<CodePointSetData> = Lazy::new(|| {
            sets::load_emoji_modifier(&super::emoji_data::BakedDataProvider)
                .expect("The data should be valid")
        });
    }

    #[inline]
    pub(crate) fn emoji_status(c: char) -> EmojiStatus {
        let is_emoji = EMOJI_CPSD.with(|emoji_cpsd| {
            let emoji_cpsd = (*emoji_cpsd).as_borrowed();
            emoji_cpsd.contains(c)
        });
        let is_component = ECOMP_CPSD.with(|ecomp_cpsd| {
            let ecomp_cpsd = (*ecomp_cpsd).as_borrowed();
            ecomp_cpsd.contains(c)
        });
        if !is_emoji {
            if !is_component {
                return EmojiStatus::NonEmoji;
            } else {
                return EmojiStatus::NonEmojiButEmojiComponent;
            }
        }
        let is_presentation = EPRES_CPSD.with(|epres_cpsd| {
            let epres_cpsd = (*epres_cpsd).as_borrowed();
            epres_cpsd.contains(c)
        });
        let is_modifier_base = EBASE_CPSD.with(|ebase_cpsd| {
            let ebase_cpsd = (*ebase_cpsd).as_borrowed();
            ebase_cpsd.contains(c)
        });
        let is_modifier = EMODIFIER_CPSD.with(|emodifier_cpsd| {
            let emodifier_cpsd = (*emodifier_cpsd).as_borrowed();
            emodifier_cpsd.contains(c)
        });
        if !is_component {
            assert_eq!(is_modifier, false);
            match (is_presentation, is_modifier_base) {
                (true, true) => EmojiStatus::EmojiPresentationAndModifierBase,
                (true, false) => EmojiStatus::EmojiPresentation,
                (false, true) => EmojiStatus::EmojiModifierBase,
                (false, false) => EmojiStatus::EmojiOther,
            }
        } else {
            assert_eq!(is_modifier_base, false);
            match (is_presentation, is_modifier) {
                (true, true) => EmojiStatus::EmojiPresentationAndModifierAndEmojiComponent,
                (true, false) => EmojiStatus::EmojiPresentationAndEmojiComponent,
                (false, true) => unreachable!(),
                (false, false) => EmojiStatus::EmojiOtherAndEmojiComponent,
            }
        }
    }
    #[inline]
    pub(crate) fn is_emoji_status_for_emoji_char_or_emoji_component(s: EmojiStatus) -> bool {
        !matches!(s, EmojiStatus::NonEmoji)
    }
    #[inline]
    pub(crate) fn is_emoji_status_for_emoji_char(s: EmojiStatus) -> bool {
        !matches!(
            s,
            EmojiStatus::NonEmoji | EmojiStatus::NonEmojiButEmojiComponent
        )
    }
    #[inline]
    pub(crate) fn is_emoji_status_for_emoji_component(s: EmojiStatus) -> bool {
        matches!(
            s,
            EmojiStatus::EmojiPresentationAndEmojiComponent
                | EmojiStatus::EmojiPresentationAndModifierAndEmojiComponent
                | EmojiStatus::EmojiOtherAndEmojiComponent
        )
    }
}

#[cfg(feature = "general-category")]
mod general_category_data;

#[cfg(feature = "general-category")]
pub(crate) mod general_category {
    #[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
    /// The most general classification of a character.
    pub enum GeneralCategory {
        /// `Lu`, an uppercase letter
        LetterUppercase,
        /// `Ll`, a lowercase letter
        LetterLowercase,
        /// `Lt`, a digraphic character, with first part uppercase
        LetterTitlecase,
        /// `Lm`, a modifier letter
        LetterModifier,
        /// `Lo`, other letters, including syllables and ideographs
        LetterOther,
        /// `Mn`, a nonspacing combining mark (zero advance width)
        MarkNonspacing,
        /// `Mc`, a spacing combining mark (positive advance width)
        MarkSpacing,
        /// `Me`, an enclosing combining mark
        MarkEnclosing,
        /// `Nd`, a decimal digit
        NumberDecimal,
        /// `Nl`, a letterlike numeric character
        NumberLetter,
        /// `No`, a numeric character of other type
        NumberOther,
        /// `Pc`, a connecting punctuation mark, like a tie
        PunctuationConnector,
        /// `Pd`, a dash or hyphen punctuation mark
        PunctuationDash,
        /// `Ps`, an opening punctuation mark (of a pair)
        PunctuationOpen,
        /// `Pe`, a closing punctuation mark (of a pair)
        PunctuationClose,
        /// `Pi`, an initial quotation mark
        PunctuationInitial,
        /// `Pf`, a final quotation mark
        PunctuationFinal,
        /// `Po`, a punctuation mark of other type
        PunctuationOther,
        /// `Sm`, a symbol of mathematical use
        SymbolMath,
        /// `Sc`, a currency sign
        SymbolCurrency,
        /// `Sk`, a non-letterlike modifier symbol
        SymbolModifier,
        /// `So`, a symbol of other type
        SymbolOther,
        /// `Zs`, a space character (of various non-zero widths)
        SeparatorSpace,
        /// `Zl`, U+2028 LINE SEPARATOR only
        SeparatorLine,
        /// `Zp`, U+2029 PARAGRAPH SEPARATOR only
        SeparatorParagraph,
        /// `Cc`, a C0 or C1 control code
        OtherControl,
        /// `Cf`, a format control character
        OtherFormat,
        /// `Cs`, a surrogate code point
        OtherSurrogate,
        /// `Co`, a private-use character
        OtherPrivateUse,
        /// `Cn`, a reserved unassigned code point or a noncharacter
        OtherUnassigned,
    }

    pub use icu_properties::GeneralCategory as ICUGC;

    impl From<ICUGC> for GeneralCategory {
        fn from(v: ICUGC) -> Self {
            match v {
                ICUGC::UppercaseLetter => GeneralCategory::LetterUppercase,
                ICUGC::LowercaseLetter => GeneralCategory::LetterLowercase,
                ICUGC::TitlecaseLetter => GeneralCategory::LetterTitlecase,
                ICUGC::ModifierLetter => GeneralCategory::LetterModifier,
                ICUGC::OtherLetter => GeneralCategory::LetterOther,
                ICUGC::NonspacingMark => GeneralCategory::MarkNonspacing,
                ICUGC::SpacingMark => GeneralCategory::MarkSpacing,
                ICUGC::EnclosingMark => GeneralCategory::MarkEnclosing,
                ICUGC::DecimalNumber => GeneralCategory::NumberDecimal,
                ICUGC::LetterNumber => GeneralCategory::NumberLetter,
                ICUGC::OtherNumber => GeneralCategory::NumberOther,
                ICUGC::ConnectorPunctuation => GeneralCategory::PunctuationConnector,
                ICUGC::DashPunctuation => GeneralCategory::PunctuationDash,
                ICUGC::OpenPunctuation => GeneralCategory::PunctuationOpen,
                ICUGC::ClosePunctuation => GeneralCategory::PunctuationClose,
                ICUGC::InitialPunctuation => GeneralCategory::PunctuationInitial,
                ICUGC::FinalPunctuation => GeneralCategory::PunctuationFinal,
                ICUGC::OtherPunctuation => GeneralCategory::PunctuationOther,
                ICUGC::MathSymbol => GeneralCategory::SymbolMath,
                ICUGC::CurrencySymbol => GeneralCategory::SymbolCurrency,
                ICUGC::ModifierSymbol => GeneralCategory::SymbolModifier,
                ICUGC::OtherSymbol => GeneralCategory::SymbolOther,
                ICUGC::SpaceSeparator => GeneralCategory::SeparatorSpace,
                ICUGC::LineSeparator => GeneralCategory::SeparatorLine,
                ICUGC::ParagraphSeparator => GeneralCategory::SeparatorParagraph,
                ICUGC::Control => GeneralCategory::OtherControl,
                ICUGC::Format => GeneralCategory::OtherFormat,
                ICUGC::Surrogate => GeneralCategory::OtherSurrogate,
                ICUGC::PrivateUse => GeneralCategory::OtherPrivateUse,
                ICUGC::Unassigned => GeneralCategory::OtherUnassigned,
            }
        }
    }

    use icu_properties::maps::{self, CodePointMapData};
    use once_cell::unsync::Lazy;

    thread_local! {
        static GC_CPMD: Lazy<CodePointMapData<ICUGC>> = Lazy::new(|| {
            maps::load_general_category(&super::general_category_data::BakedDataProvider)
                .expect("The data should be valid")
        });
    }

    #[inline]
    pub(crate) fn general_category_of_char(c: char) -> GeneralCategory {
        GC_CPMD
            .with(|gc_cpmd| {
                let gc_cpmd = (*gc_cpmd).as_borrowed();
                gc_cpmd.get(c)
            })
            .into()
    }

    #[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
    /// Groupings of the most general classification of a character.
    pub enum GeneralCategoryGroup {
        /// Lu | Ll | Lt | Lm | Lo
        Letter,
        /// Mn | Mc | Me
        Mark,
        /// Nd | Nl | No
        Number,
        /// Pc | Pd | Ps | Pe | Pi | Pf | Po
        Punctuation,
        /// Sm | Sc | Sk | So
        Symbol,
        /// Zs | Zl | Zp
        Separator,
        /// Cc | Cf | Cs | Co | Cn
        Other,
    }

    /*
    #[inline]
    pub(crate) fn general_category_is_letter_cased(gc: GeneralCategory) -> bool {
        matches!(
            gc,
            GeneralCategory::UppercaseLetter
                | GeneralCategory::LowercaseLetter
                | GeneralCategory::TitlecaseLetter
        )
    }

    #[inline]
    pub(crate) fn general_category_group(gc: GeneralCategory) -> GeneralCategoryGroup {
        match gc {
            GeneralCategory::UppercaseLetter
            | GeneralCategory::LowercaseLetter
            | GeneralCategory::TitlecaseLetter
            | GeneralCategory::ModifierLetter
            | GeneralCategory::OtherLetter => GeneralCategoryGroup::Letter,
            GeneralCategory::NonspacingMark
            | GeneralCategory::SpacingMark
            | GeneralCategory::EnclosingMark => GeneralCategoryGroup::Mark,
            GeneralCategory::DecimalNumber
            | GeneralCategory::LetterNumber
            | GeneralCategory::OtherNumber => GeneralCategoryGroup::Number,
            GeneralCategory::ConnectorPunctuation
            | GeneralCategory::DashPunctuation
            | GeneralCategory::OpenPunctuation
            | GeneralCategory::ClosePunctuation
            | GeneralCategory::InitialPunctuation
            | GeneralCategory::FinalPunctuation
            | GeneralCategory::OtherPunctuation => GeneralCategoryGroup::Punctuation,
            GeneralCategory::MathSymbol
            | GeneralCategory::CurrencySymbol
            | GeneralCategory::ModifierSymbol
            | GeneralCategory::OtherSymbol => GeneralCategoryGroup::Symbol,
            GeneralCategory::SpaceSeparator
            | GeneralCategory::LineSeparator
            | GeneralCategory::ParagraphSeparator => GeneralCategoryGroup::Separator,
            GeneralCategory::Control
            | GeneralCategory::Format
            | GeneralCategory::Surrogate
            | GeneralCategory::PrivateUse
            | GeneralCategory::Unassigned => GeneralCategoryGroup::Other,
        }
    }
    */

    #[inline]
    pub(crate) fn general_category_is_letter_cased(gc: GeneralCategory) -> bool {
        matches!(
            gc,
            GeneralCategory::LetterUppercase
                | GeneralCategory::LetterLowercase
                | GeneralCategory::LetterTitlecase
        )
    }

    #[inline]
    pub(crate) fn general_category_group(gc: GeneralCategory) -> GeneralCategoryGroup {
        match gc {
            GeneralCategory::LetterUppercase
            | GeneralCategory::LetterLowercase
            | GeneralCategory::LetterTitlecase
            | GeneralCategory::LetterModifier
            | GeneralCategory::LetterOther => GeneralCategoryGroup::Letter,
            GeneralCategory::MarkNonspacing
            | GeneralCategory::MarkSpacing
            | GeneralCategory::MarkEnclosing => GeneralCategoryGroup::Mark,
            GeneralCategory::NumberDecimal
            | GeneralCategory::NumberLetter
            | GeneralCategory::NumberOther => GeneralCategoryGroup::Number,
            GeneralCategory::PunctuationConnector
            | GeneralCategory::PunctuationDash
            | GeneralCategory::PunctuationOpen
            | GeneralCategory::PunctuationClose
            | GeneralCategory::PunctuationInitial
            | GeneralCategory::PunctuationFinal
            | GeneralCategory::PunctuationOther => GeneralCategoryGroup::Punctuation,
            GeneralCategory::SymbolMath
            | GeneralCategory::SymbolCurrency
            | GeneralCategory::SymbolModifier
            | GeneralCategory::SymbolOther => GeneralCategoryGroup::Symbol,
            GeneralCategory::SeparatorSpace
            | GeneralCategory::SeparatorLine
            | GeneralCategory::SeparatorParagraph => GeneralCategoryGroup::Separator,
            GeneralCategory::OtherControl
            | GeneralCategory::OtherFormat
            | GeneralCategory::OtherSurrogate
            | GeneralCategory::OtherPrivateUse
            | GeneralCategory::OtherUnassigned => GeneralCategoryGroup::Other,
        }
    }
}

mod version;

pub use version::UNICODE_VERSION;
