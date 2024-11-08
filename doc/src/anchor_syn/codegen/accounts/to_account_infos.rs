<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anchor-syn-0.30.0/src/codegen/accounts/to_account_infos.rs`."><title>to_account_infos.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../" data-static-root-path="../../../../static.files/" data-current-crate="anchor_syn" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../src-files.js"></script><script defer src="../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../anchor_syn/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">crate</span>::codegen::accounts::{generics, ParsedGenerics};
<span class="kw">use crate</span>::{AccountField, AccountsStruct};
<span class="kw">use </span>quote::quote;

<span class="comment">// Generates the `ToAccountInfos` trait implementation.
</span><span class="kw">pub fn </span>generate(accs: <span class="kw-2">&amp;</span>AccountsStruct) -&gt; proc_macro2::TokenStream {
    <span class="kw">let </span>name = <span class="kw-2">&amp;</span>accs.ident;
    <span class="kw">let </span>ParsedGenerics {
        combined_generics,
        trait_generics,
        struct_generics,
        where_clause,
    } = generics(accs);

    <span class="kw">let </span>to_acc_infos: Vec&lt;proc_macro2::TokenStream&gt; = accs
        .fields
        .iter()
        .map(|f: <span class="kw-2">&amp;</span>AccountField| {
            <span class="kw">let </span>name = <span class="kw-2">&amp;</span>f.ident();
            <span class="macro">quote!</span> { account_infos.extend(<span class="self">self</span>.#name.to_account_infos()); }
        })
        .collect();
    <span class="macro">quote!</span> {
        <span class="attr">#[automatically_derived]
        </span><span class="kw">impl</span>&lt;#combined_generics&gt; anchor_lang::ToAccountInfos&lt;#trait_generics&gt; <span class="kw">for </span>#name &lt;#struct_generics&gt; #where_clause{
            <span class="kw">fn </span>to_account_infos(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Vec&lt;anchor_lang::solana_program::account_info::AccountInfo&lt;#trait_generics&gt;&gt; {
                <span class="kw">let </span><span class="kw-2">mut </span>account_infos = <span class="macro">vec!</span>[];

                #(#to_acc_infos)*

                account_infos
            }
        }
    }
}
</code></pre></div></section></main></body></html>