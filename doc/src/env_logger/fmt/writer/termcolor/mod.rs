<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/env_logger-0.9.3/src/fmt/writer/termcolor/mod.rs`."><title>mod.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../../" data-static-root-path="../../../../../static.files/" data-current-crate="env_logger" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../../src-files.js"></script><script defer src="../../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://www.rust-lang.org/static/images/favicon.ico"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../../env_logger/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">/*
This internal module contains the style and terminal writing implementation.

Its public API is available when the `termcolor` crate is available.
The terminal printing is shimmed when the `termcolor` crate is not available.
*/

</span><span class="attr">#[cfg_attr(feature = <span class="string">"termcolor"</span>, path = <span class="string">"extern_impl.rs"</span>)]
#[cfg_attr(not(feature = <span class="string">"termcolor"</span>), path = <span class="string">"shim_impl.rs"</span>)]
</span><span class="kw">mod </span>imp;

<span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::fmt) <span class="kw">use </span><span class="self">self</span>::imp::<span class="kw-2">*</span>;
</code></pre></div></section></main></body></html>