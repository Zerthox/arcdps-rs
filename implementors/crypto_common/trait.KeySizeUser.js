(function() {var implementors = {
"aes":[["impl <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"aes/struct.Aes192Dec.html\" title=\"struct aes::Aes192Dec\">Aes192Dec</a>"],["impl <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"aes/struct.Aes128Dec.html\" title=\"struct aes::Aes128Dec\">Aes128Dec</a>"],["impl <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"aes/struct.Aes192Enc.html\" title=\"struct aes::Aes192Enc\">Aes192Enc</a>"],["impl <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"aes/struct.Aes256Enc.html\" title=\"struct aes::Aes256Enc\">Aes256Enc</a>"],["impl <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"aes/struct.Aes128Enc.html\" title=\"struct aes::Aes128Enc\">Aes128Enc</a>"],["impl <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"aes/struct.Aes128.html\" title=\"struct aes::Aes128\">Aes128</a>"],["impl <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"aes/struct.Aes256Dec.html\" title=\"struct aes::Aes256Dec\">Aes256Dec</a>"],["impl <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"aes/struct.Aes192.html\" title=\"struct aes::Aes192\">Aes192</a>"],["impl <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"aes/struct.Aes256.html\" title=\"struct aes::Aes256\">Aes256</a>"]],
"cipher":[["impl&lt;T: <a class=\"trait\" href=\"cipher/trait.KeySizeUser.html\" title=\"trait cipher::KeySizeUser\">KeySizeUser</a> + <a class=\"trait\" href=\"cipher/trait.BlockSizeUser.html\" title=\"trait cipher::BlockSizeUser\">BlockSizeUser</a>&gt; <a class=\"trait\" href=\"cipher/trait.KeySizeUser.html\" title=\"trait cipher::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"cipher/struct.StreamCipherCoreWrapper.html\" title=\"struct cipher::StreamCipherCoreWrapper\">StreamCipherCoreWrapper</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"cipher/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type cipher::BlockSizeUser::BlockSize\">BlockSize</a>: <a class=\"trait\" href=\"typenum/type_operators/trait.IsLess.html\" title=\"trait typenum::type_operators::IsLess\">IsLess</a>&lt;<a class=\"type\" href=\"cipher/consts/type.U256.html\" title=\"type cipher::consts::U256\">U256</a>&gt;,\n    <a class=\"type\" href=\"typenum/operator_aliases/type.Le.html\" title=\"type typenum::operator_aliases::Le\">Le</a>&lt;T::<a class=\"associatedtype\" href=\"cipher/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type cipher::BlockSizeUser::BlockSize\">BlockSize</a>, <a class=\"type\" href=\"cipher/consts/type.U256.html\" title=\"type cipher::consts::U256\">U256</a>&gt;: <a class=\"trait\" href=\"typenum/marker_traits/trait.NonZero.html\" title=\"trait typenum::marker_traits::NonZero\">NonZero</a>,</span>"]],
"crypto_common":[],
"digest":[["impl&lt;T&gt; <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"digest/core_api/struct.CoreWrapper.html\" title=\"struct digest::core_api::CoreWrapper\">CoreWrapper</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"digest/core_api/trait.BufferKindUser.html\" title=\"trait digest::core_api::BufferKindUser\">BufferKindUser</a> + <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a>,\n    T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>: <a class=\"trait\" href=\"typenum/type_operators/trait.IsLess.html\" title=\"trait typenum::type_operators::IsLess\">IsLess</a>&lt;<a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;,\n    <a class=\"type\" href=\"typenum/operator_aliases/type.Le.html\" title=\"type typenum::operator_aliases::Le\">Le</a>&lt;T::<a class=\"associatedtype\" href=\"digest/core_api/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type digest::core_api::BlockSizeUser::BlockSize\">BlockSize</a>, <a class=\"type\" href=\"digest/consts/type.U256.html\" title=\"type digest::consts::U256\">U256</a>&gt;: <a class=\"trait\" href=\"typenum/marker_traits/trait.NonZero.html\" title=\"trait typenum::marker_traits::NonZero\">NonZero</a>,</span>"]],
"hmac":[["impl&lt;D&gt; <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"hmac/struct.HmacCore.html\" title=\"struct hmac::HmacCore\">HmacCore</a>&lt;D&gt;<span class=\"where fmt-newline\">where\n    D: <a class=\"trait\" href=\"digest/core_api/wrapper/trait.CoreProxy.html\" title=\"trait digest::core_api::wrapper::CoreProxy\">CoreProxy</a>,\n    D::<a class=\"associatedtype\" href=\"digest/core_api/wrapper/trait.CoreProxy.html#associatedtype.Core\" title=\"type digest::core_api::wrapper::CoreProxy::Core\">Core</a>: <a class=\"trait\" href=\"digest/digest/trait.HashMarker.html\" title=\"trait digest::digest::HashMarker\">HashMarker</a> + <a class=\"trait\" href=\"digest/core_api/trait.UpdateCore.html\" title=\"trait digest::core_api::UpdateCore\">UpdateCore</a> + <a class=\"trait\" href=\"digest/core_api/trait.FixedOutputCore.html\" title=\"trait digest::core_api::FixedOutputCore\">FixedOutputCore</a> + <a class=\"trait\" href=\"digest/core_api/trait.BufferKindUser.html\" title=\"trait digest::core_api::BufferKindUser\">BufferKindUser</a>&lt;BufferKind = <a class=\"struct\" href=\"block_buffer/struct.Eager.html\" title=\"struct block_buffer::Eager\">Eager</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    &lt;D::<a class=\"associatedtype\" href=\"digest/core_api/wrapper/trait.CoreProxy.html#associatedtype.Core\" title=\"type digest::core_api::wrapper::CoreProxy::Core\">Core</a> as <a class=\"trait\" href=\"crypto_common/trait.BlockSizeUser.html\" title=\"trait crypto_common::BlockSizeUser\">BlockSizeUser</a>&gt;::<a class=\"associatedtype\" href=\"crypto_common/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type crypto_common::BlockSizeUser::BlockSize\">BlockSize</a>: <a class=\"trait\" href=\"typenum/type_operators/trait.IsLess.html\" title=\"trait typenum::type_operators::IsLess\">IsLess</a>&lt;<a class=\"type\" href=\"typenum/generated/consts/type.U256.html\" title=\"type typenum::generated::consts::U256\">U256</a>&gt;,\n    <a class=\"type\" href=\"typenum/operator_aliases/type.Le.html\" title=\"type typenum::operator_aliases::Le\">Le</a>&lt;&lt;D::<a class=\"associatedtype\" href=\"digest/core_api/wrapper/trait.CoreProxy.html#associatedtype.Core\" title=\"type digest::core_api::wrapper::CoreProxy::Core\">Core</a> as <a class=\"trait\" href=\"crypto_common/trait.BlockSizeUser.html\" title=\"trait crypto_common::BlockSizeUser\">BlockSizeUser</a>&gt;::<a class=\"associatedtype\" href=\"crypto_common/trait.BlockSizeUser.html#associatedtype.BlockSize\" title=\"type crypto_common::BlockSizeUser::BlockSize\">BlockSize</a>, <a class=\"type\" href=\"typenum/generated/consts/type.U256.html\" title=\"type typenum::generated::consts::U256\">U256</a>&gt;: <a class=\"trait\" href=\"typenum/marker_traits/trait.NonZero.html\" title=\"trait typenum::marker_traits::NonZero\">NonZero</a>,</span>"],["impl&lt;D: <a class=\"trait\" href=\"digest/digest/trait.Digest.html\" title=\"trait digest::digest::Digest\">Digest</a> + <a class=\"trait\" href=\"crypto_common/trait.BlockSizeUser.html\" title=\"trait crypto_common::BlockSizeUser\">BlockSizeUser</a>&gt; <a class=\"trait\" href=\"crypto_common/trait.KeySizeUser.html\" title=\"trait crypto_common::KeySizeUser\">KeySizeUser</a> for <a class=\"struct\" href=\"hmac/struct.SimpleHmac.html\" title=\"struct hmac::SimpleHmac\">SimpleHmac</a>&lt;D&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()