<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ed25519-dalek-1.0.1/src/signature.rs`."><title>signature.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="ed25519_dalek" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../ed25519_dalek/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// -*- mode: rust; -*-
//
// This file is part of ed25519-dalek.
// Copyright (c) 2017-2019 isis lovecruft
// See LICENSE for licensing information.
//
// Authors:
// - isis agora lovecruft &lt;isis@patternsinthevoid.net&gt;

</span><span class="doccomment">//! An ed25519 signature.

</span><span class="kw">use </span>core::convert::TryFrom;
<span class="kw">use </span>core::fmt::Debug;

<span class="kw">use </span>curve25519_dalek::edwards::CompressedEdwardsY;
<span class="kw">use </span>curve25519_dalek::scalar::Scalar;
<span class="kw">use </span>ed25519::signature::Signature <span class="kw">as _</span>;

<span class="kw">use </span><span class="kw">crate</span>::constants::<span class="kw-2">*</span>;
<span class="kw">use </span><span class="kw">crate</span>::errors::<span class="kw-2">*</span>;

<span class="doccomment">/// An ed25519 signature.
///
/// # Note
///
/// These signatures, unlike the ed25519 signature reference implementation, are
/// "detached"—that is, they do **not** include a copy of the message which has
/// been signed.
</span><span class="attr">#[allow(non_snake_case)]
#[derive(Copy, Eq, PartialEq)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>InternalSignature {
    <span class="doccomment">/// `R` is an `EdwardsPoint`, formed by using an hash function with
    /// 512-bits output to produce the digest of:
    ///
    /// - the nonce half of the `ExpandedSecretKey`, and
    /// - the message to be signed.
    ///
    /// This digest is then interpreted as a `Scalar` and reduced into an
    /// element in ℤ/lℤ.  The scalar is then multiplied by the distinguished
    /// basepoint to produce `R`, and `EdwardsPoint`.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) R: CompressedEdwardsY,

    <span class="doccomment">/// `s` is a `Scalar`, formed by using an hash function with 512-bits output
    /// to produce the digest of:
    ///
    /// - the `r` portion of this `Signature`,
    /// - the `PublicKey` which should be used to verify this `Signature`, and
    /// - the message to be signed.
    ///
    /// This digest is then interpreted as a `Scalar` and reduced into an
    /// element in ℤ/lℤ.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) s: Scalar,
}

<span class="kw">impl </span>Clone <span class="kw">for </span>InternalSignature {
    <span class="kw">fn </span>clone(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="kw-2">*</span><span class="self">self
    </span>}
}

<span class="kw">impl </span>Debug <span class="kw">for </span>InternalSignature {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>::core::fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; ::core::fmt::Result {
        <span class="macro">write!</span>(f, <span class="string">"Signature( R: {:?}, s: {:?} )"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.R, <span class="kw-2">&amp;</span><span class="self">self</span>.s)
    }
}

<span class="attr">#[cfg(feature = <span class="string">"legacy_compatibility"</span>)]
#[inline(always)]
</span><span class="kw">fn </span>check_scalar(bytes: [u8; <span class="number">32</span>]) -&gt; <span class="prelude-ty">Result</span>&lt;Scalar, SignatureError&gt; {
    <span class="comment">// The highest 3 bits must not be set.  No other checking for the
    // remaining 2^253 - 2^252 + 27742317777372353535851937790883648493
    // potential non-reduced scalars is performed.
    //
    // This is compatible with ed25519-donna and libsodium when
    // -DED25519_COMPAT is NOT specified.
    </span><span class="kw">if </span>bytes[<span class="number">31</span>] &amp; <span class="number">224 </span>!= <span class="number">0 </span>{
        <span class="kw">return </span><span class="prelude-val">Err</span>(InternalError::ScalarFormatError.into());
    }

    <span class="prelude-val">Ok</span>(Scalar::from_bits(bytes))
}

<span class="attr">#[cfg(not(feature = <span class="string">"legacy_compatibility"</span>))]
#[inline(always)]
</span><span class="kw">fn </span>check_scalar(bytes: [u8; <span class="number">32</span>]) -&gt; <span class="prelude-ty">Result</span>&lt;Scalar, SignatureError&gt; {
    <span class="comment">// Since this is only used in signature deserialisation (i.e. upon
    // verification), we can do a "succeed fast" trick by checking that the most
    // significant 4 bits are unset.  If they are unset, we can succeed fast
    // because we are guaranteed that the scalar is fully reduced.  However, if
    // the 4th most significant bit is set, we must do the full reduction check,
    // as the order of the basepoint is roughly a 2^(252.5) bit number.
    //
    // This succeed-fast trick should succeed for roughly half of all scalars.
    </span><span class="kw">if </span>bytes[<span class="number">31</span>] &amp; <span class="number">240 </span>== <span class="number">0 </span>{
        <span class="kw">return </span><span class="prelude-val">Ok</span>(Scalar::from_bits(bytes))
    }

    <span class="kw">match </span>Scalar::from_canonical_bytes(bytes) {
        <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(InternalError::ScalarFormatError.into()),
        <span class="prelude-val">Some</span>(x) =&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(x),
    };
}

<span class="kw">impl </span>InternalSignature {
    <span class="doccomment">/// Convert this `Signature` to a byte array.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>to_bytes(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; [u8; SIGNATURE_LENGTH] {
        <span class="kw">let </span><span class="kw-2">mut </span>signature_bytes: [u8; SIGNATURE_LENGTH] = [<span class="number">0u8</span>; SIGNATURE_LENGTH];

        signature_bytes[..<span class="number">32</span>].copy_from_slice(<span class="kw-2">&amp;</span><span class="self">self</span>.R.as_bytes()[..]);
        signature_bytes[<span class="number">32</span>..].copy_from_slice(<span class="kw-2">&amp;</span><span class="self">self</span>.s.as_bytes()[..]);
        signature_bytes
    }

    <span class="doccomment">/// Construct a `Signature` from a slice of bytes.
    ///
    /// # Scalar Malleability Checking
    ///
    /// As originally specified in the ed25519 paper (cf. the "Malleability"
    /// section of the README in this repo), no checks whatsoever were performed
    /// for signature malleability.
    ///
    /// Later, a semi-functional, hacky check was added to most libraries to
    /// "ensure" that the scalar portion, `s`, of the signature was reduced `mod
    /// \ell`, the order of the basepoint:
    ///
    /// ```ignore
    /// if signature.s[31] &amp; 224 != 0 {
    ///     return Err();
    /// }
    /// ```
    ///
    /// This bit-twiddling ensures that the most significant three bits of the
    /// scalar are not set:
    ///
    /// ```python,ignore
    /// &gt;&gt;&gt; 0b00010000 &amp; 224
    /// 0
    /// &gt;&gt;&gt; 0b00100000 &amp; 224
    /// 32
    /// &gt;&gt;&gt; 0b01000000 &amp; 224
    /// 64
    /// &gt;&gt;&gt; 0b10000000 &amp; 224
    /// 128
    /// ```
    ///
    /// However, this check is hacky and insufficient to check that the scalar is
    /// fully reduced `mod \ell = 2^252 + 27742317777372353535851937790883648493` as
    /// it leaves us with a guanteed bound of 253 bits.  This means that there are
    /// `2^253 - 2^252 + 2774231777737235353585193779088364849311` remaining scalars
    /// which could cause malleabilllity.
    ///
    /// RFC8032 [states](https://tools.ietf.org/html/rfc8032#section-5.1.7):
    ///
    /// &gt; To verify a signature on a message M using public key A, [...]
    /// &gt; first split the signature into two 32-octet halves.  Decode the first
    /// &gt; half as a point R, and the second half as an integer S, in the range
    /// &gt; 0 &lt;= s &lt; L.  Decode the public key A as point A'.  If any of the
    /// &gt; decodings fail (including S being out of range), the signature is
    /// &gt; invalid.
    ///
    /// However, by the time this was standardised, most libraries in use were
    /// only checking the most significant three bits.  (See also the
    /// documentation for `PublicKey.verify_strict`.)
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>from_bytes(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;InternalSignature, SignatureError&gt; {
        <span class="kw">if </span>bytes.len() != SIGNATURE_LENGTH {
            <span class="kw">return </span><span class="prelude-val">Err</span>(InternalError::BytesLengthError {
                name: <span class="string">"Signature"</span>,
                length: SIGNATURE_LENGTH,
            }.into());
        }
        <span class="kw">let </span><span class="kw-2">mut </span>lower: [u8; <span class="number">32</span>] = [<span class="number">0u8</span>; <span class="number">32</span>];
        <span class="kw">let </span><span class="kw-2">mut </span>upper: [u8; <span class="number">32</span>] = [<span class="number">0u8</span>; <span class="number">32</span>];

        lower.copy_from_slice(<span class="kw-2">&amp;</span>bytes[..<span class="number">32</span>]);
        upper.copy_from_slice(<span class="kw-2">&amp;</span>bytes[<span class="number">32</span>..]);

        <span class="kw">let </span>s: Scalar;

        <span class="kw">match </span>check_scalar(upper) {
            <span class="prelude-val">Ok</span>(x)  =&gt; s = x,
            <span class="prelude-val">Err</span>(x) =&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(x),
        }

        <span class="prelude-val">Ok</span>(InternalSignature {
            R: CompressedEdwardsY(lower),
            s: s,
        })
    }
}

<span class="kw">impl </span>TryFrom&lt;<span class="kw-2">&amp;</span>ed25519::Signature&gt; <span class="kw">for </span>InternalSignature {
    <span class="kw">type </span>Error = SignatureError;

    <span class="kw">fn </span>try_from(sig: <span class="kw-2">&amp;</span>ed25519::Signature) -&gt; <span class="prelude-ty">Result</span>&lt;InternalSignature, SignatureError&gt; {
        InternalSignature::from_bytes(sig.as_bytes())
    }
}

<span class="kw">impl </span>From&lt;InternalSignature&gt; <span class="kw">for </span>ed25519::Signature {
    <span class="kw">fn </span>from(sig: InternalSignature) -&gt; ed25519::Signature {
        ed25519::Signature::from_bytes(<span class="kw-2">&amp;</span>sig.to_bytes()).unwrap()
    }
}
</code></pre></div></section></main></body></html>