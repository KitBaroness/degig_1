<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sha2-0.10.8/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="sha2" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../sha2/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
<a href="#2" id="2">2</a>
<a href="#3" id="3">3</a>
<a href="#4" id="4">4</a>
<a href="#5" id="5">5</a>
<a href="#6" id="6">6</a>
<a href="#7" id="7">7</a>
<a href="#8" id="8">8</a>
<a href="#9" id="9">9</a>
<a href="#10" id="10">10</a>
<a href="#11" id="11">11</a>
<a href="#12" id="12">12</a>
<a href="#13" id="13">13</a>
<a href="#14" id="14">14</a>
<a href="#15" id="15">15</a>
<a href="#16" id="16">16</a>
<a href="#17" id="17">17</a>
<a href="#18" id="18">18</a>
<a href="#19" id="19">19</a>
<a href="#20" id="20">20</a>
<a href="#21" id="21">21</a>
<a href="#22" id="22">22</a>
<a href="#23" id="23">23</a>
<a href="#24" id="24">24</a>
<a href="#25" id="25">25</a>
<a href="#26" id="26">26</a>
<a href="#27" id="27">27</a>
<a href="#28" id="28">28</a>
<a href="#29" id="29">29</a>
<a href="#30" id="30">30</a>
<a href="#31" id="31">31</a>
<a href="#32" id="32">32</a>
<a href="#33" id="33">33</a>
<a href="#34" id="34">34</a>
<a href="#35" id="35">35</a>
<a href="#36" id="36">36</a>
<a href="#37" id="37">37</a>
<a href="#38" id="38">38</a>
<a href="#39" id="39">39</a>
<a href="#40" id="40">40</a>
<a href="#41" id="41">41</a>
<a href="#42" id="42">42</a>
<a href="#43" id="43">43</a>
<a href="#44" id="44">44</a>
<a href="#45" id="45">45</a>
<a href="#46" id="46">46</a>
<a href="#47" id="47">47</a>
<a href="#48" id="48">48</a>
<a href="#49" id="49">49</a>
<a href="#50" id="50">50</a>
<a href="#51" id="51">51</a>
<a href="#52" id="52">52</a>
<a href="#53" id="53">53</a>
<a href="#54" id="54">54</a>
<a href="#55" id="55">55</a>
<a href="#56" id="56">56</a>
<a href="#57" id="57">57</a>
<a href="#58" id="58">58</a>
<a href="#59" id="59">59</a>
<a href="#60" id="60">60</a>
<a href="#61" id="61">61</a>
<a href="#62" id="62">62</a>
<a href="#63" id="63">63</a>
<a href="#64" id="64">64</a>
<a href="#65" id="65">65</a>
<a href="#66" id="66">66</a>
<a href="#67" id="67">67</a>
<a href="#68" id="68">68</a>
<a href="#69" id="69">69</a>
<a href="#70" id="70">70</a>
<a href="#71" id="71">71</a>
<a href="#72" id="72">72</a>
<a href="#73" id="73">73</a>
<a href="#74" id="74">74</a>
<a href="#75" id="75">75</a>
<a href="#76" id="76">76</a>
<a href="#77" id="77">77</a>
<a href="#78" id="78">78</a>
<a href="#79" id="79">79</a>
<a href="#80" id="80">80</a>
<a href="#81" id="81">81</a>
<a href="#82" id="82">82</a>
<a href="#83" id="83">83</a>
<a href="#84" id="84">84</a>
<a href="#85" id="85">85</a>
<a href="#86" id="86">86</a>
<a href="#87" id="87">87</a>
<a href="#88" id="88">88</a>
<a href="#89" id="89">89</a>
<a href="#90" id="90">90</a>
<a href="#91" id="91">91</a>
<a href="#92" id="92">92</a>
<a href="#93" id="93">93</a>
<a href="#94" id="94">94</a>
<a href="#95" id="95">95</a>
<a href="#96" id="96">96</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! An implementation of the [SHA-2][1] cryptographic hash algorithms.
//!
//! There are 6 standard algorithms specified in the SHA-2 standard: [`Sha224`],
//! [`Sha256`], [`Sha512_224`], [`Sha512_256`], [`Sha384`], and [`Sha512`].
//!
//! Algorithmically, there are only 2 core algorithms: SHA-256 and SHA-512.
//! All other algorithms are just applications of these with different initial
//! hash values, and truncated to different digest bit lengths. The first two
//! algorithms in the list are based on SHA-256, while the last four are based
//! on SHA-512.
//!
//! # Usage
//!
//! ```rust
//! use hex_literal::hex;
//! use sha2::{Sha256, Sha512, Digest};
//!
//! // create a Sha256 object
//! let mut hasher = Sha256::new();
//!
//! // write input message
//! hasher.update(b"hello world");
//!
//! // read hash digest and consume hasher
//! let result = hasher.finalize();
//!
//! assert_eq!(result[..], hex!("
//!     b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
//! ")[..]);
//!
//! // same for Sha512
//! let mut hasher = Sha512::new();
//! hasher.update(b"hello world");
//! let result = hasher.finalize();
//!
//! assert_eq!(result[..], hex!("
//!     309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f
//!     989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f
//! ")[..]);
//! ```
//!
//! Also see [RustCrypto/hashes][2] readme.
//!
//! [1]: https://en.wikipedia.org/wiki/SHA-2
//! [2]: https://github.com/RustCrypto/hashes

</span><span class="attr">#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = <span class="string">"https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"</span>,
    html_favicon_url = <span class="string">"https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
</span>)]
#![warn(missing_docs, rust_2018_idioms)]

</span><span class="kw">pub use </span>digest::{<span class="self">self</span>, Digest};

<span class="attr">#[cfg(feature = <span class="string">"oid"</span>)]
</span><span class="kw">use </span>digest::const_oid::{AssociatedOid, ObjectIdentifier};
<span class="kw">use </span>digest::{
    consts::{U28, U32, U48, U64},
    core_api::{CoreWrapper, CtVariableCoreWrapper},
    impl_oid_carrier,
};

<span class="attr">#[rustfmt::skip]
</span><span class="kw">mod </span>consts;
<span class="kw">mod </span>core_api;
<span class="kw">mod </span>sha256;
<span class="kw">mod </span>sha512;

<span class="attr">#[cfg(feature = <span class="string">"compress"</span>)]
</span><span class="kw">pub use </span>sha256::compress256;
<span class="attr">#[cfg(feature = <span class="string">"compress"</span>)]
</span><span class="kw">pub use </span>sha512::compress512;

<span class="kw">pub use </span>core_api::{Sha256VarCore, Sha512VarCore};

<span class="macro">impl_oid_carrier!</span>(OidSha256, <span class="string">"2.16.840.1.101.3.4.2.1"</span>);
<span class="macro">impl_oid_carrier!</span>(OidSha384, <span class="string">"2.16.840.1.101.3.4.2.2"</span>);
<span class="macro">impl_oid_carrier!</span>(OidSha512, <span class="string">"2.16.840.1.101.3.4.2.3"</span>);
<span class="macro">impl_oid_carrier!</span>(OidSha224, <span class="string">"2.16.840.1.101.3.4.2.4"</span>);
<span class="macro">impl_oid_carrier!</span>(OidSha512_224, <span class="string">"2.16.840.1.101.3.4.2.5"</span>);
<span class="macro">impl_oid_carrier!</span>(OidSha512_256, <span class="string">"2.16.840.1.101.3.4.2.6"</span>);

<span class="doccomment">/// SHA-224 hasher.
</span><span class="kw">pub type </span>Sha224 = CoreWrapper&lt;CtVariableCoreWrapper&lt;Sha256VarCore, U28, OidSha224&gt;&gt;;
<span class="doccomment">/// SHA-256 hasher.
</span><span class="kw">pub type </span>Sha256 = CoreWrapper&lt;CtVariableCoreWrapper&lt;Sha256VarCore, U32, OidSha256&gt;&gt;;
<span class="doccomment">/// SHA-512/224 hasher.
</span><span class="kw">pub type </span>Sha512_224 = CoreWrapper&lt;CtVariableCoreWrapper&lt;Sha512VarCore, U28, OidSha512_224&gt;&gt;;
<span class="doccomment">/// SHA-512/256 hasher.
</span><span class="kw">pub type </span>Sha512_256 = CoreWrapper&lt;CtVariableCoreWrapper&lt;Sha512VarCore, U32, OidSha512_256&gt;&gt;;
<span class="doccomment">/// SHA-384 hasher.
</span><span class="kw">pub type </span>Sha384 = CoreWrapper&lt;CtVariableCoreWrapper&lt;Sha512VarCore, U48, OidSha384&gt;&gt;;
<span class="doccomment">/// SHA-512 hasher.
</span><span class="kw">pub type </span>Sha512 = CoreWrapper&lt;CtVariableCoreWrapper&lt;Sha512VarCore, U64, OidSha512&gt;&gt;;
</code></pre></div></section></main></body></html>