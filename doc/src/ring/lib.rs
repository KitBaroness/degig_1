<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.17.8/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="ring" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../ring/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#97" id="97">97</a>
<a href="#98" id="98">98</a>
<a href="#99" id="99">99</a>
<a href="#100" id="100">100</a>
<a href="#101" id="101">101</a>
<a href="#102" id="102">102</a>
<a href="#103" id="103">103</a>
<a href="#104" id="104">104</a>
<a href="#105" id="105">105</a>
<a href="#106" id="106">106</a>
<a href="#107" id="107">107</a>
<a href="#108" id="108">108</a>
<a href="#109" id="109">109</a>
<a href="#110" id="110">110</a>
<a href="#111" id="111">111</a>
<a href="#112" id="112">112</a>
<a href="#113" id="113">113</a>
<a href="#114" id="114">114</a>
<a href="#115" id="115">115</a>
<a href="#116" id="116">116</a>
<a href="#117" id="117">117</a>
<a href="#118" id="118">118</a>
<a href="#119" id="119">119</a>
<a href="#120" id="120">120</a>
<a href="#121" id="121">121</a>
<a href="#122" id="122">122</a>
<a href="#123" id="123">123</a>
<a href="#124" id="124">124</a>
<a href="#125" id="125">125</a>
<a href="#126" id="126">126</a>
<a href="#127" id="127">127</a>
<a href="#128" id="128">128</a>
<a href="#129" id="129">129</a>
<a href="#130" id="130">130</a>
<a href="#131" id="131">131</a>
<a href="#132" id="132">132</a>
<a href="#133" id="133">133</a>
<a href="#134" id="134">134</a>
<a href="#135" id="135">135</a>
<a href="#136" id="136">136</a>
<a href="#137" id="137">137</a>
<a href="#138" id="138">138</a>
<a href="#139" id="139">139</a>
<a href="#140" id="140">140</a>
<a href="#141" id="141">141</a>
<a href="#142" id="142">142</a>
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2015-2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

</span><span class="doccomment">//! Safe, fast, small crypto using Rust with BoringSSL's cryptography
//! primitives.
//!
//! # Feature Flags
//!
//! &lt;table&gt;
//! &lt;tr&gt;&lt;th&gt;Feature
//!     &lt;th&gt;Description
//! &lt;tr&gt;&lt;td&gt;&lt;code&gt;alloc (default)&lt;/code&gt;
//!     &lt;td&gt;Enable features that require use of the heap, RSA in particular.
//! &lt;tr&gt;&lt;td&gt;&lt;code&gt;less-safe-getrandom-custom-or-rdrand&lt;/code&gt;
//!     &lt;td&gt;Treat user-provided ("custom") and RDRAND-based &lt;code&gt;getrandom&lt;/code&gt;
//!         implementations as secure random number generators (see
//!         &lt;code&gt;SecureRandom&lt;/code&gt;). This feature only works with
//!         &lt;code&gt;os = "none"&lt;/code&gt; targets. See
//!         &lt;a href="https://docs.rs/getrandom/0.2.10/getrandom/macro.register_custom_getrandom.html"&gt;
//!             &lt;code&gt;register_custom_getrandom&lt;/code&gt;
//!         &lt;/a&gt; and &lt;a href="https://docs.rs/getrandom/0.2.10/getrandom/#rdrand-on-x86"&gt;
//!             RDRAND on x86
//!         &lt;/a&gt; for additional details.
//! &lt;tr&gt;&lt;td&gt;&lt;code&gt;std&lt;/code&gt;
//!     &lt;td&gt;Enable features that use libstd, in particular
//!         &lt;code&gt;std::error::Error&lt;/code&gt; integration. Implies `alloc`.
//! &lt;tr&gt;&lt;td&gt;&lt;code&gt;wasm32_unknown_unknown_js&lt;/code&gt;
//!     &lt;td&gt;When this feature is enabled, for the wasm32-unknown-unknown target,
//!         Web APIs will be used to implement features like `ring::rand` that
//!         require an operating environment of some kind. This has no effect
//!         for any other target. This enables the `getrandom` crate's `js`
//!         feature.
//! &lt;/table&gt;

</span><span class="comment">// When running mk/package.sh, don't actually build any code.
</span><span class="attr">#![cfg(not(pregenerate_asm_only))]
#![allow(
    clippy::collapsible_if,
    clippy::identity_op,
    clippy::len_without_is_empty,
    clippy::let_unit_value,
    clippy::new_without_default,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::too_many_arguments,
    clippy::type_complexity,
    non_camel_case_types,
    non_snake_case,
    unsafe_code
)]
#![deny(variant_size_differences)]
#![forbid(
    unused_results,
    clippy::char_lit_as_u8,
    clippy::fn_to_numeric_cast,
    clippy::fn_to_numeric_cast_with_truncation,
    clippy::ptr_as_ptr
)]
#![warn(
    clippy::unnecessary_cast,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
#![no_std]

#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">extern crate </span>alloc;

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>debug;

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>prefixed;

<span class="attr">#[macro_use]
</span><span class="kw">pub mod </span>test;

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>arithmetic;

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>bssl;

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>polyfill;

<span class="kw">pub mod </span>aead;

<span class="kw">pub mod </span>agreement;

<span class="kw">mod </span>bits;

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">mod </span>c;
<span class="kw">pub mod </span>constant_time;

<span class="kw">pub mod </span>io;

<span class="kw">mod </span>cpu;
<span class="kw">pub mod </span>digest;
<span class="kw">mod </span>ec;
<span class="kw">mod </span>endian;
<span class="kw">pub mod </span>error;
<span class="kw">pub mod </span>hkdf;
<span class="kw">pub mod </span>hmac;
<span class="kw">mod </span>limb;
<span class="kw">pub mod </span>pbkdf2;
<span class="kw">pub mod </span>pkcs8;
<span class="kw">pub mod </span>rand;

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">pub mod </span>rsa;

<span class="kw">pub mod </span>signature;

<span class="kw">mod </span>sealed {
    <span class="doccomment">/// Traits that are designed to only be implemented internally in *ring*.
    </span><span class="comment">//
    // Usage:
    // ```
    // use crate::sealed;
    //
    // pub trait MyType: sealed::Sealed {
    //     // [...]
    // }
    //
    // impl sealed::Sealed for MyType {}
    // ```
    </span><span class="kw">pub trait </span>Sealed {}
}
</code></pre></div></section></main></body></html>