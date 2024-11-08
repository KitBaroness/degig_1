<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/polyval-0.5.3/src/backend/soft64.rs`."><title>soft64.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="polyval" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/RustCrypto/media/8f1a9894/logo.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../polyval/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#143" id="143">143</a>
<a href="#144" id="144">144</a>
<a href="#145" id="145">145</a>
<a href="#146" id="146">146</a>
<a href="#147" id="147">147</a>
<a href="#148" id="148">148</a>
<a href="#149" id="149">149</a>
<a href="#150" id="150">150</a>
<a href="#151" id="151">151</a>
<a href="#152" id="152">152</a>
<a href="#153" id="153">153</a>
<a href="#154" id="154">154</a>
<a href="#155" id="155">155</a>
<a href="#156" id="156">156</a>
<a href="#157" id="157">157</a>
<a href="#158" id="158">158</a>
<a href="#159" id="159">159</a>
<a href="#160" id="160">160</a>
<a href="#161" id="161">161</a>
<a href="#162" id="162">162</a>
<a href="#163" id="163">163</a>
<a href="#164" id="164">164</a>
<a href="#165" id="165">165</a>
<a href="#166" id="166">166</a>
<a href="#167" id="167">167</a>
<a href="#168" id="168">168</a>
<a href="#169" id="169">169</a>
<a href="#170" id="170">170</a>
<a href="#171" id="171">171</a>
<a href="#172" id="172">172</a>
<a href="#173" id="173">173</a>
<a href="#174" id="174">174</a>
<a href="#175" id="175">175</a>
<a href="#176" id="176">176</a>
<a href="#177" id="177">177</a>
<a href="#178" id="178">178</a>
<a href="#179" id="179">179</a>
<a href="#180" id="180">180</a>
<a href="#181" id="181">181</a>
<a href="#182" id="182">182</a>
<a href="#183" id="183">183</a>
<a href="#184" id="184">184</a>
<a href="#185" id="185">185</a>
<a href="#186" id="186">186</a>
<a href="#187" id="187">187</a>
<a href="#188" id="188">188</a>
<a href="#189" id="189">189</a>
<a href="#190" id="190">190</a>
<a href="#191" id="191">191</a>
<a href="#192" id="192">192</a>
<a href="#193" id="193">193</a>
<a href="#194" id="194">194</a>
<a href="#195" id="195">195</a>
<a href="#196" id="196">196</a>
<a href="#197" id="197">197</a>
<a href="#198" id="198">198</a>
<a href="#199" id="199">199</a>
<a href="#200" id="200">200</a>
<a href="#201" id="201">201</a>
<a href="#202" id="202">202</a>
<a href="#203" id="203">203</a>
<a href="#204" id="204">204</a>
<a href="#205" id="205">205</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! Constant-time software implementation of POLYVAL for 64-bit architectures.
//! Adapted from BearSSL's `ghash_ctmul64.c`:
//!
//! &lt;https://bearssl.org/gitweb/?p=BearSSL;a=blob;f=src/hash/ghash_ctmul64.c;hb=4b6046412&gt;
//!
//! Copyright (c) 2016 Thomas Pornin &lt;pornin@bolet.org&gt;

</span><span class="kw">use crate</span>::{Block, Key};
<span class="kw">use </span>core::{
    convert::TryInto,
    num::Wrapping,
    ops::{Add, Mul},
};
<span class="kw">use </span>universal_hash::{consts::U16, NewUniversalHash, Output, UniversalHash};

<span class="attr">#[cfg(feature = <span class="string">"zeroize"</span>)]
</span><span class="kw">use </span>zeroize::Zeroize;

<span class="doccomment">/// **POLYVAL**: GHASH-like universal hash over GF(2^128).
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub struct </span>Polyval {
    <span class="doccomment">/// GF(2^128) field element input blocks are multiplied by
    </span>h: U64x2,

    <span class="doccomment">/// Field element representing the computed universal hash
    </span>s: U64x2,
}

<span class="kw">impl </span>NewUniversalHash <span class="kw">for </span>Polyval {
    <span class="kw">type </span>KeySize = U16;

    <span class="doccomment">/// Initialize POLYVAL with the given `H` field element
    </span><span class="kw">fn </span>new(h: <span class="kw-2">&amp;</span>Key) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            h: h.into(),
            s: U64x2::default(),
        }
    }
}

<span class="kw">impl </span>UniversalHash <span class="kw">for </span>Polyval {
    <span class="kw">type </span>BlockSize = U16;

    <span class="doccomment">/// Input a field element `X` to be authenticated
    </span><span class="kw">fn </span>update(<span class="kw-2">&amp;mut </span><span class="self">self</span>, x: <span class="kw-2">&amp;</span>Block) {
        <span class="kw">let </span>x = U64x2::from(x);
        <span class="self">self</span>.s = (<span class="self">self</span>.s + x) * <span class="self">self</span>.h;
    }

    <span class="doccomment">/// Reset internal state
    </span><span class="kw">fn </span>reset(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.s = U64x2::default();
    }

    <span class="doccomment">/// Get POLYVAL result (i.e. computed `S` field element)
    </span><span class="kw">fn </span>finalize(<span class="self">self</span>) -&gt; Output&lt;<span class="self">Self</span>&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>block = Block::default();

        <span class="kw">for </span>(chunk, i) <span class="kw">in </span>block.chunks_mut(<span class="number">8</span>).zip(<span class="kw-2">&amp;</span>[<span class="self">self</span>.s.<span class="number">0</span>, <span class="self">self</span>.s.<span class="number">1</span>]) {
            chunk.copy_from_slice(<span class="kw-2">&amp;</span>i.to_le_bytes());
        }

        Output::new(block)
    }
}

<span class="attr">#[cfg(feature = <span class="string">"zeroize"</span>)]
</span><span class="kw">impl </span>Drop <span class="kw">for </span>Polyval {
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.h.zeroize();
        <span class="self">self</span>.s.zeroize();
    }
}

<span class="doccomment">/// 2 x `u64` values
</span><span class="attr">#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
</span><span class="kw">struct </span>U64x2(u64, u64);

<span class="kw">impl </span>From&lt;<span class="kw-2">&amp;</span>Block&gt; <span class="kw">for </span>U64x2 {
    <span class="kw">fn </span>from(bytes: <span class="kw-2">&amp;</span>Block) -&gt; U64x2 {
        U64x2(
            u64::from_le_bytes(bytes[..<span class="number">8</span>].try_into().unwrap()),
            u64::from_le_bytes(bytes[<span class="number">8</span>..].try_into().unwrap()),
        )
    }
}

<span class="attr">#[allow(clippy::suspicious_arithmetic_impl)]
</span><span class="kw">impl </span>Add <span class="kw">for </span>U64x2 {
    <span class="kw">type </span>Output = <span class="self">Self</span>;

    <span class="doccomment">/// Adds two POLYVAL field elements.
    </span><span class="kw">fn </span>add(<span class="self">self</span>, rhs: <span class="self">Self</span>) -&gt; <span class="self">Self</span>::Output {
        U64x2(<span class="self">self</span>.<span class="number">0 </span>^ rhs.<span class="number">0</span>, <span class="self">self</span>.<span class="number">1 </span>^ rhs.<span class="number">1</span>)
    }
}

<span class="attr">#[allow(clippy::suspicious_arithmetic_impl)]
</span><span class="kw">impl </span>Mul <span class="kw">for </span>U64x2 {
    <span class="kw">type </span>Output = <span class="self">Self</span>;

    <span class="doccomment">/// Computes carryless POLYVAL multiplication over GF(2^128) in constant time.
    ///
    /// Method described at:
    /// &lt;https://www.bearssl.org/constanttime.html#ghash-for-gcm&gt;
    ///
    /// POLYVAL multiplication is effectively the little endian equivalent of
    /// GHASH multiplication, aside from one small detail described here:
    ///
    /// &lt;https://crypto.stackexchange.com/questions/66448/how-does-bearssls-gcm-modular-reduction-work/66462#66462&gt;
    ///
    /// &gt; The product of two bit-reversed 128-bit polynomials yields the
    /// &gt; bit-reversed result over 255 bits, not 256. The BearSSL code ends up
    /// &gt; with a 256-bit result in zw[], and that value is shifted by one bit,
    /// &gt; because of that reversed convention issue. Thus, the code must
    /// &gt; include a shifting step to put it back where it should
    ///
    /// This shift is unnecessary for POLYVAL and has been removed.
    </span><span class="kw">fn </span>mul(<span class="self">self</span>, rhs: <span class="self">Self</span>) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>h0 = <span class="self">self</span>.<span class="number">0</span>;
        <span class="kw">let </span>h1 = <span class="self">self</span>.<span class="number">1</span>;
        <span class="kw">let </span>h0r = rev64(h0);
        <span class="kw">let </span>h1r = rev64(h1);
        <span class="kw">let </span>h2 = h0 ^ h1;
        <span class="kw">let </span>h2r = h0r ^ h1r;

        <span class="kw">let </span>y0 = rhs.<span class="number">0</span>;
        <span class="kw">let </span>y1 = rhs.<span class="number">1</span>;
        <span class="kw">let </span>y0r = rev64(y0);
        <span class="kw">let </span>y1r = rev64(y1);
        <span class="kw">let </span>y2 = y0 ^ y1;
        <span class="kw">let </span>y2r = y0r ^ y1r;
        <span class="kw">let </span>z0 = bmul64(y0, h0);
        <span class="kw">let </span>z1 = bmul64(y1, h1);

        <span class="kw">let </span><span class="kw-2">mut </span>z2 = bmul64(y2, h2);
        <span class="kw">let </span><span class="kw-2">mut </span>z0h = bmul64(y0r, h0r);
        <span class="kw">let </span><span class="kw-2">mut </span>z1h = bmul64(y1r, h1r);
        <span class="kw">let </span><span class="kw-2">mut </span>z2h = bmul64(y2r, h2r);

        z2 ^= z0 ^ z1;
        z2h ^= z0h ^ z1h;
        z0h = rev64(z0h) &gt;&gt; <span class="number">1</span>;
        z1h = rev64(z1h) &gt;&gt; <span class="number">1</span>;
        z2h = rev64(z2h) &gt;&gt; <span class="number">1</span>;

        <span class="kw">let </span>v0 = z0;
        <span class="kw">let </span><span class="kw-2">mut </span>v1 = z0h ^ z2;
        <span class="kw">let </span><span class="kw-2">mut </span>v2 = z1 ^ z2h;
        <span class="kw">let </span><span class="kw-2">mut </span>v3 = z1h;

        v2 ^= v0 ^ (v0 &gt;&gt; <span class="number">1</span>) ^ (v0 &gt;&gt; <span class="number">2</span>) ^ (v0 &gt;&gt; <span class="number">7</span>);
        v1 ^= (v0 &lt;&lt; <span class="number">63</span>) ^ (v0 &lt;&lt; <span class="number">62</span>) ^ (v0 &lt;&lt; <span class="number">57</span>);
        v3 ^= v1 ^ (v1 &gt;&gt; <span class="number">1</span>) ^ (v1 &gt;&gt; <span class="number">2</span>) ^ (v1 &gt;&gt; <span class="number">7</span>);
        v2 ^= (v1 &lt;&lt; <span class="number">63</span>) ^ (v1 &lt;&lt; <span class="number">62</span>) ^ (v1 &lt;&lt; <span class="number">57</span>);

        U64x2(v2, v3)
    }
}

<span class="attr">#[cfg(feature = <span class="string">"zeroize"</span>)]
</span><span class="kw">impl </span>Zeroize <span class="kw">for </span>U64x2 {
    <span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.<span class="number">0</span>.zeroize();
        <span class="self">self</span>.<span class="number">1</span>.zeroize();
    }
}

<span class="doccomment">/// Multiplication in GF(2)[X], truncated to the low 64-bits, with “holes”
/// (sequences of zeroes) to avoid carry spilling.
///
/// When carries do occur, they wind up in a "hole" and are subsequently masked
/// out of the result.
</span><span class="kw">fn </span>bmul64(x: u64, y: u64) -&gt; u64 {
    <span class="kw">let </span>x0 = Wrapping(x &amp; <span class="number">0x1111_1111_1111_1111</span>);
    <span class="kw">let </span>x1 = Wrapping(x &amp; <span class="number">0x2222_2222_2222_2222</span>);
    <span class="kw">let </span>x2 = Wrapping(x &amp; <span class="number">0x4444_4444_4444_4444</span>);
    <span class="kw">let </span>x3 = Wrapping(x &amp; <span class="number">0x8888_8888_8888_8888</span>);
    <span class="kw">let </span>y0 = Wrapping(y &amp; <span class="number">0x1111_1111_1111_1111</span>);
    <span class="kw">let </span>y1 = Wrapping(y &amp; <span class="number">0x2222_2222_2222_2222</span>);
    <span class="kw">let </span>y2 = Wrapping(y &amp; <span class="number">0x4444_4444_4444_4444</span>);
    <span class="kw">let </span>y3 = Wrapping(y &amp; <span class="number">0x8888_8888_8888_8888</span>);

    <span class="kw">let </span><span class="kw-2">mut </span>z0 = ((x0 * y0) ^ (x1 * y3) ^ (x2 * y2) ^ (x3 * y1)).<span class="number">0</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>z1 = ((x0 * y1) ^ (x1 * y0) ^ (x2 * y3) ^ (x3 * y2)).<span class="number">0</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>z2 = ((x0 * y2) ^ (x1 * y1) ^ (x2 * y0) ^ (x3 * y3)).<span class="number">0</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>z3 = ((x0 * y3) ^ (x1 * y2) ^ (x2 * y1) ^ (x3 * y0)).<span class="number">0</span>;

    z0 &amp;= <span class="number">0x1111_1111_1111_1111</span>;
    z1 &amp;= <span class="number">0x2222_2222_2222_2222</span>;
    z2 &amp;= <span class="number">0x4444_4444_4444_4444</span>;
    z3 &amp;= <span class="number">0x8888_8888_8888_8888</span>;

    z0 | z1 | z2 | z3
}

<span class="doccomment">/// Bit-reverse a `u64` in constant time
</span><span class="kw">fn </span>rev64(<span class="kw-2">mut </span>x: u64) -&gt; u64 {
    x = ((x &amp; <span class="number">0x5555_5555_5555_5555</span>) &lt;&lt; <span class="number">1</span>) | ((x &gt;&gt; <span class="number">1</span>) &amp; <span class="number">0x5555_5555_5555_5555</span>);
    x = ((x &amp; <span class="number">0x3333_3333_3333_3333</span>) &lt;&lt; <span class="number">2</span>) | ((x &gt;&gt; <span class="number">2</span>) &amp; <span class="number">0x3333_3333_3333_3333</span>);
    x = ((x &amp; <span class="number">0x0f0f_0f0f_0f0f_0f0f</span>) &lt;&lt; <span class="number">4</span>) | ((x &gt;&gt; <span class="number">4</span>) &amp; <span class="number">0x0f0f_0f0f_0f0f_0f0f</span>);
    x = ((x &amp; <span class="number">0x00ff_00ff_00ff_00ff</span>) &lt;&lt; <span class="number">8</span>) | ((x &gt;&gt; <span class="number">8</span>) &amp; <span class="number">0x00ff_00ff_00ff_00ff</span>);
    x = ((x &amp; <span class="number">0xffff_0000_ffff</span>) &lt;&lt; <span class="number">16</span>) | ((x &gt;&gt; <span class="number">16</span>) &amp; <span class="number">0xffff_0000_ffff</span>);
    (x &lt;&lt; <span class="number">32</span>) | (x &gt;&gt; <span class="number">32</span>)
}
</code></pre></div></section></main></body></html>