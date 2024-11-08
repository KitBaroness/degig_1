<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ark-bn254-0.4.0/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="ark_bn254" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../ark_bn254/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="attr">#![cfg_attr(not(feature = <span class="string">"std"</span>), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

</span><span class="doccomment">//! This library implements the BN254 curve that was sampled as part of the [\[BCTV14\]](https://eprint.iacr.org/2013/879.pdf) paper .
//! The name denotes that it is a Barreto--Naehrig curve of embedding degree 12,
//! defined over a 254-bit (prime) field. The scalar field is highly 2-adic.
//!
//! This curve is also implemented in [libff](https://github.com/scipr-lab/libff/tree/master/libff/algebra/curves/alt_bn128) under the name `bn128`.
//! It is the same as the `bn256` curve used in Ethereum (eg: [go-ethereum](https://github.com/ethereum/go-ethereum/tree/master/crypto/bn254/cloudflare)).
//!
//! #CAUTION
//! **This curve does not satisfy the 128-bit security level anymore.**
//!
//!
//! Curve information:
//! * Base field: q =
//!   21888242871839275222246405745257275088696311157297823662689037894645226208583
//! * Scalar field: r =
//!   21888242871839275222246405745257275088548364400416034343698204186575808495617
//! * valuation(q - 1, 2) = 1
//! * valuation(r - 1, 2) = 28
//! * G1 curve equation: y^2 = x^3 + 3
//! * G2 curve equation: y^2 = x^3 + B, where
//!    * B = 3/(u+9) where Fq2 is represented as Fq\[u\]/(u^2+1) =
//!      Fq2(19485874751759354771024239261021720505790618469301721065564631296452457478373,
//!      266929791119991161246907387137283842545076965332900288569378510910307636690)

</span><span class="attr">#[cfg(feature = <span class="string">"curve"</span>)]
</span><span class="kw">mod </span>curves;

<span class="kw">mod </span>fields;

<span class="attr">#[cfg(feature = <span class="string">"curve"</span>)]
</span><span class="kw">pub use </span>curves::<span class="kw-2">*</span>;

<span class="kw">pub use </span>fields::<span class="kw-2">*</span>;
</code></pre></div></section></main></body></html>