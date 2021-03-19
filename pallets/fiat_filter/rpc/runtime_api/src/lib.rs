#![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::Parameter;
use codec::Codec;
use sp_runtime::traits::MaybeDisplay;

// type Call = (); // TODO limit to crate::Call<Runtime>

sp_api::decl_runtime_apis! {
    pub trait FiatFeeRuntimeApi<Balance> where
        Balance: Codec + MaybeDisplay,
    {
        // Block refers to the type argument auto-added to FiatFeeRuntimeApi by the macro
        /// Returns the call fee in microDOCK
        fn get_call_fee_dock(uxt: Block::Extrinsic) -> Balance;
        // fn get_call_fee_dock(call: Call) -> Option<u32>;
        // fn get_call_fee_dock(call: Block::Extrinsic) -> Option<u32>;
    }
}
