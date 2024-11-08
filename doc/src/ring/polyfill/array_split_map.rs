<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.17.8/src/polyfill/array_split_map.rs`."><title>array_split_map.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="ring" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../ring/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2023 Brian Smith.
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

</span><span class="kw">pub trait </span>ArraySplitMap&lt;I, O, <span class="kw">const </span>CN: usize, <span class="kw">const </span>ON: usize&gt; {
    <span class="kw">fn </span>array_split_map(<span class="self">self</span>, f: <span class="kw">impl </span>Fn([I; CN]) -&gt; O) -&gt; [O; ON];
}

<span class="kw">impl</span>&lt;I, O&gt; ArraySplitMap&lt;I, O, <span class="number">4</span>, <span class="number">3</span>&gt; <span class="kw">for </span>[I; <span class="number">12</span>] {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>array_split_map(<span class="self">self</span>, f: <span class="kw">impl </span>Fn([I; <span class="number">4</span>]) -&gt; O) -&gt; [O; <span class="number">3</span>] {
        <span class="kw">let </span>[a0, a1, a2, a3, b0, b1, b2, b3, c0, c1, c2, c3] = <span class="self">self</span>;
        [
            f([a0, a1, a2, a3]),
            f([b0, b1, b2, b3]),
            f([c0, c1, c2, c3]),
        ]
    }
}

<span class="kw">impl</span>&lt;I, O&gt; ArraySplitMap&lt;I, O, <span class="number">4</span>, <span class="number">4</span>&gt; <span class="kw">for </span>[I; <span class="number">16</span>] {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>array_split_map(<span class="self">self</span>, f: <span class="kw">impl </span>Fn([I; <span class="number">4</span>]) -&gt; O) -&gt; [O; <span class="number">4</span>] {
        <span class="kw">let </span>[a0, a1, a2, a3, b0, b1, b2, b3, c0, c1, c2, c3, d0, d1, d2, d3] = <span class="self">self</span>;
        [
            f([a0, a1, a2, a3]),
            f([b0, b1, b2, b3]),
            f([c0, c1, c2, c3]),
            f([d0, d1, d2, d3]),
        ]
    }
}

<span class="kw">impl</span>&lt;I, O&gt; ArraySplitMap&lt;I, O, <span class="number">4</span>, <span class="number">8</span>&gt; <span class="kw">for </span>[I; <span class="number">32</span>] {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>array_split_map(<span class="self">self</span>, f: <span class="kw">impl </span>Fn([I; <span class="number">4</span>]) -&gt; O) -&gt; [O; <span class="number">8</span>] {
        <span class="kw">let </span>[a0, a1, a2, a3, b0, b1, b2, b3, c0, c1, c2, c3, d0, d1, d2, d3, e0, e1, e2, e3, f0, f1, f2, f3, g0, g1, g2, g3, h0, h1, h2, h3] =
            <span class="self">self</span>;
        [
            f([a0, a1, a2, a3]),
            f([b0, b1, b2, b3]),
            f([c0, c1, c2, c3]),
            f([d0, d1, d2, d3]),
            f([e0, e1, e2, e3]),
            f([f0, f1, f2, f3]),
            f([g0, g1, g2, g3]),
            f([h0, h1, h2, h3]),
        ]
    }
}

<span class="kw">impl</span>&lt;I, O&gt; ArraySplitMap&lt;I, O, <span class="number">8</span>, <span class="number">2</span>&gt; <span class="kw">for </span>[I; <span class="number">16</span>] {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>array_split_map(<span class="self">self</span>, f: <span class="kw">impl </span>Fn([I; <span class="number">8</span>]) -&gt; O) -&gt; [O; <span class="number">2</span>] {
        <span class="kw">let </span>[a0, a1, a2, a3, a4, a5, a6, a7, b0, b1, b2, b3, b4, b5, b6, b7] = <span class="self">self</span>;
        [
            f([a0, a1, a2, a3, a4, a5, a6, a7]),
            f([b0, b1, b2, b3, b4, b5, b6, b7]),
        ]
    }
}
</code></pre></div></section></main></body></html>