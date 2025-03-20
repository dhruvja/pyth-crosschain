// /// Crate a default [`anchor_lang::IdlBuild`] implementation for the given type.
// ///
// /// This is used in order to make wrapper accounts of `anchor-spl` work with `idl-build` feature.
// macro_rules! impl_idl_build {
//     ($ty: ty) => {
//         impl anchor_lang::IdlBuild for $ty {}

//         // This is not used for the IDL generation since default `IdlBuild` impl doesn't include
//         // the type in the IDL but it stil needs to be added in order to make compilation work.
//         //
//         // TODO: Find a better way to handle discriminators of wrapped external accounts.
//         impl anchor_lang::Discriminator for $ty {
//             const DISCRIMINATOR: &'static [u8] = &[];
//         }
//     };
// }

// // // impl_idl_build!(crate::price_update::PriceUpdateV2);
// impl_idl_build!(crate::price_update::LocalFeedId);
