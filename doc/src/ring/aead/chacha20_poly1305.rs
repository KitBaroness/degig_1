<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.17.8/src/aead/chacha20_poly1305.rs`."><title>chacha20_poly1305.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="ring" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../ring/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#206" id="206">206</a>
<a href="#207" id="207">207</a>
<a href="#208" id="208">208</a>
<a href="#209" id="209">209</a>
<a href="#210" id="210">210</a>
<a href="#211" id="211">211</a>
<a href="#212" id="212">212</a>
<a href="#213" id="213">213</a>
<a href="#214" id="214">214</a>
<a href="#215" id="215">215</a>
<a href="#216" id="216">216</a>
<a href="#217" id="217">217</a>
<a href="#218" id="218">218</a>
<a href="#219" id="219">219</a>
<a href="#220" id="220">220</a>
<a href="#221" id="221">221</a>
<a href="#222" id="222">222</a>
<a href="#223" id="223">223</a>
<a href="#224" id="224">224</a>
<a href="#225" id="225">225</a>
<a href="#226" id="226">226</a>
<a href="#227" id="227">227</a>
<a href="#228" id="228">228</a>
<a href="#229" id="229">229</a>
<a href="#230" id="230">230</a>
<a href="#231" id="231">231</a>
<a href="#232" id="232">232</a>
<a href="#233" id="233">233</a>
<a href="#234" id="234">234</a>
<a href="#235" id="235">235</a>
<a href="#236" id="236">236</a>
<a href="#237" id="237">237</a>
<a href="#238" id="238">238</a>
<a href="#239" id="239">239</a>
<a href="#240" id="240">240</a>
<a href="#241" id="241">241</a>
<a href="#242" id="242">242</a>
<a href="#243" id="243">243</a>
<a href="#244" id="244">244</a>
<a href="#245" id="245">245</a>
<a href="#246" id="246">246</a>
<a href="#247" id="247">247</a>
<a href="#248" id="248">248</a>
<a href="#249" id="249">249</a>
<a href="#250" id="250">250</a>
<a href="#251" id="251">251</a>
<a href="#252" id="252">252</a>
<a href="#253" id="253">253</a>
<a href="#254" id="254">254</a>
<a href="#255" id="255">255</a>
<a href="#256" id="256">256</a>
<a href="#257" id="257">257</a>
<a href="#258" id="258">258</a>
<a href="#259" id="259">259</a>
<a href="#260" id="260">260</a>
<a href="#261" id="261">261</a>
<a href="#262" id="262">262</a>
<a href="#263" id="263">263</a>
<a href="#264" id="264">264</a>
<a href="#265" id="265">265</a>
<a href="#266" id="266">266</a>
<a href="#267" id="267">267</a>
<a href="#268" id="268">268</a>
<a href="#269" id="269">269</a>
<a href="#270" id="270">270</a>
<a href="#271" id="271">271</a>
<a href="#272" id="272">272</a>
<a href="#273" id="273">273</a>
<a href="#274" id="274">274</a>
<a href="#275" id="275">275</a>
<a href="#276" id="276">276</a>
<a href="#277" id="277">277</a>
<a href="#278" id="278">278</a>
<a href="#279" id="279">279</a>
<a href="#280" id="280">280</a>
<a href="#281" id="281">281</a>
<a href="#282" id="282">282</a>
<a href="#283" id="283">283</a>
<a href="#284" id="284">284</a>
<a href="#285" id="285">285</a>
<a href="#286" id="286">286</a>
<a href="#287" id="287">287</a>
<a href="#288" id="288">288</a>
<a href="#289" id="289">289</a>
<a href="#290" id="290">290</a>
<a href="#291" id="291">291</a>
<a href="#292" id="292">292</a>
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

</span><span class="kw">use super</span>::{
    chacha::{<span class="self">self</span>, Counter, Iv},
    poly1305, Aad, Nonce, Tag,
};
<span class="kw">use crate</span>::{
    aead, cpu, error,
    polyfill::{u64_from_usize, usize_from_u64_saturated, ArrayFlatten},
};
<span class="kw">use </span>core::ops::RangeFrom;

<span class="doccomment">/// ChaCha20-Poly1305 as described in [RFC 8439].
///
/// The keys are 256 bits long and the nonces are 96 bits long.
///
/// [RFC 8439]: https://tools.ietf.org/html/rfc8439
</span><span class="kw">pub static </span>CHACHA20_POLY1305: aead::Algorithm = aead::Algorithm {
    key_len: chacha::KEY_LEN,
    init: chacha20_poly1305_init,
    seal: chacha20_poly1305_seal,
    open: chacha20_poly1305_open,
    id: aead::AlgorithmID::CHACHA20_POLY1305,
};

<span class="kw">const </span>MAX_IN_OUT_LEN: usize = <span class="kw">super</span>::max_input_len(<span class="number">64</span>, <span class="number">1</span>);
<span class="comment">// https://tools.ietf.org/html/rfc8439#section-2.8
</span><span class="kw">const </span>_MAX_IN_OUT_LEN_BOUNDED_BY_RFC: () =
    <span class="macro">assert!</span>(MAX_IN_OUT_LEN == usize_from_u64_saturated(<span class="number">274_877_906_880u64</span>));

<span class="doccomment">/// Copies |key| into |ctx_buf|.
</span><span class="kw">fn </span>chacha20_poly1305_init(
    key: <span class="kw-2">&amp;</span>[u8],
    _cpu_features: cpu::Features,
) -&gt; <span class="prelude-ty">Result</span>&lt;aead::KeyInner, error::Unspecified&gt; {
    <span class="kw">let </span>key: [u8; chacha::KEY_LEN] = key.try_into()<span class="question-mark">?</span>;
    <span class="prelude-val">Ok</span>(aead::KeyInner::ChaCha20Poly1305(chacha::Key::new(key)))
}

<span class="kw">fn </span>chacha20_poly1305_seal(
    key: <span class="kw-2">&amp;</span>aead::KeyInner,
    nonce: Nonce,
    aad: Aad&lt;<span class="kw-2">&amp;</span>[u8]&gt;,
    in_out: <span class="kw-2">&amp;mut </span>[u8],
    cpu_features: cpu::Features,
) -&gt; <span class="prelude-ty">Result</span>&lt;Tag, error::Unspecified&gt; {
    <span class="kw">let </span>chacha20_key = <span class="kw">match </span>key {
        aead::KeyInner::ChaCha20Poly1305(key) =&gt; key,
        <span class="kw">_ </span>=&gt; <span class="macro">unreachable!</span>(),
    };

    <span class="kw">if </span>in_out.len() &gt; MAX_IN_OUT_LEN {
        <span class="kw">return </span><span class="prelude-val">Err</span>(error::Unspecified);
    }
    <span class="doccomment">/// RFC 8439 Section 2.8 says the maximum AAD length is 2**64 - 1, which is
    /// never larger than usize::MAX, so we don't need an explicit length
    /// check.
    </span><span class="kw">const </span>_USIZE_BOUNDED_BY_U64: u64 = u64_from_usize(usize::MAX);

    <span class="attr">#[cfg(any(target_arch = <span class="string">"aarch64"</span>, target_arch = <span class="string">"x86_64"</span>))]
    </span><span class="kw">if </span>has_integrated(cpu_features) {
        <span class="comment">// XXX: BoringSSL uses `alignas(16)` on `key` instead of on the
        // structure, but Rust can't do that yet; see
        // https://github.com/rust-lang/rust/issues/73557.
        //
        // Keep in sync with the anonymous struct of BoringSSL's
        // `chacha20_poly1305_seal_data`.
        </span><span class="attr">#[repr(align(<span class="number">16</span>), C)]
        #[derive(Clone, Copy)]
        </span><span class="kw">struct </span>seal_data_in {
            key: [u32; chacha::KEY_LEN / <span class="number">4</span>],
            counter: u32,
            nonce: [u8; <span class="kw">super</span>::NONCE_LEN],
            extra_ciphertext: <span class="kw-2">*const </span>u8,
            extra_ciphertext_len: usize,
        }

        <span class="kw">let </span><span class="kw-2">mut </span>data = InOut {
            input: seal_data_in {
                key: <span class="kw-2">*</span>chacha20_key.words_less_safe(),
                counter: <span class="number">0</span>,
                nonce: <span class="kw-2">*</span>nonce.as_ref(),
                extra_ciphertext: core::ptr::null(),
                extra_ciphertext_len: <span class="number">0</span>,
            },
        };

        <span class="comment">// Encrypts `plaintext_len` bytes from `plaintext` and writes them to `out_ciphertext`.
        </span><span class="macro">prefixed_extern!</span> {
            <span class="kw">fn </span>chacha20_poly1305_seal(
                out_ciphertext: <span class="kw-2">*mut </span>u8,
                plaintext: <span class="kw-2">*const </span>u8,
                plaintext_len: usize,
                ad: <span class="kw-2">*const </span>u8,
                ad_len: usize,
                data: <span class="kw-2">&amp;mut </span>InOut&lt;seal_data_in&gt;,
            );
        }

        <span class="kw">let </span>out = <span class="kw">unsafe </span>{
            chacha20_poly1305_seal(
                in_out.as_mut_ptr(),
                in_out.as_ptr(),
                in_out.len(),
                aad.as_ref().as_ptr(),
                aad.as_ref().len(),
                <span class="kw-2">&amp;mut </span>data,
            );
            <span class="kw-2">&amp;</span>data.out
        };

        <span class="kw">return </span><span class="prelude-val">Ok</span>(Tag(out.tag));
    }

    <span class="kw">let </span><span class="kw-2">mut </span>counter = Counter::zero(nonce);
    <span class="kw">let </span><span class="kw-2">mut </span>auth = {
        <span class="kw">let </span>key = derive_poly1305_key(chacha20_key, counter.increment());
        poly1305::Context::from_key(key, cpu_features)
    };

    poly1305_update_padded_16(<span class="kw-2">&amp;mut </span>auth, aad.as_ref());
    chacha20_key.encrypt_in_place(counter, in_out);
    poly1305_update_padded_16(<span class="kw-2">&amp;mut </span>auth, in_out);
    <span class="prelude-val">Ok</span>(finish(auth, aad.as_ref().len(), in_out.len()))
}

<span class="kw">fn </span>chacha20_poly1305_open(
    key: <span class="kw-2">&amp;</span>aead::KeyInner,
    nonce: Nonce,
    aad: Aad&lt;<span class="kw-2">&amp;</span>[u8]&gt;,
    in_out: <span class="kw-2">&amp;mut </span>[u8],
    src: RangeFrom&lt;usize&gt;,
    cpu_features: cpu::Features,
) -&gt; <span class="prelude-ty">Result</span>&lt;Tag, error::Unspecified&gt; {
    <span class="kw">let </span>chacha20_key = <span class="kw">match </span>key {
        aead::KeyInner::ChaCha20Poly1305(key) =&gt; key,
        <span class="kw">_ </span>=&gt; <span class="macro">unreachable!</span>(),
    };

    <span class="kw">let </span>unprefixed_len = in_out
        .len()
        .checked_sub(src.start)
        .ok_or(error::Unspecified)<span class="question-mark">?</span>;
    <span class="kw">if </span>unprefixed_len &gt; MAX_IN_OUT_LEN {
        <span class="kw">return </span><span class="prelude-val">Err</span>(error::Unspecified);
    }
    <span class="comment">// RFC 8439 Section 2.8 says the maximum AAD length is 2**64 - 1, which is
    // never larger than usize::MAX, so we don't need an explicit length
    // check.
    </span><span class="kw">const </span>_USIZE_BOUNDED_BY_U64: u64 = u64_from_usize(usize::MAX);

    <span class="attr">#[cfg(any(target_arch = <span class="string">"aarch64"</span>, target_arch = <span class="string">"x86_64"</span>))]
    </span><span class="kw">if </span>has_integrated(cpu_features) {
        <span class="comment">// XXX: BoringSSL uses `alignas(16)` on `key` instead of on the
        // structure, but Rust can't do that yet; see
        // https://github.com/rust-lang/rust/issues/73557.
        //
        // Keep in sync with the anonymous struct of BoringSSL's
        // `chacha20_poly1305_open_data`.
        </span><span class="attr">#[derive(Copy, Clone)]
        #[repr(align(<span class="number">16</span>), C)]
        </span><span class="kw">struct </span>open_data_in {
            key: [u32; chacha::KEY_LEN / <span class="number">4</span>],
            counter: u32,
            nonce: [u8; <span class="kw">super</span>::NONCE_LEN],
        }

        <span class="kw">let </span><span class="kw-2">mut </span>data = InOut {
            input: open_data_in {
                key: <span class="kw-2">*</span>chacha20_key.words_less_safe(),
                counter: <span class="number">0</span>,
                nonce: <span class="kw-2">*</span>nonce.as_ref(),
            },
        };

        <span class="comment">// Decrypts `plaintext_len` bytes from `ciphertext` and writes them to `out_plaintext`.
        </span><span class="macro">prefixed_extern!</span> {
            <span class="kw">fn </span>chacha20_poly1305_open(
                out_plaintext: <span class="kw-2">*mut </span>u8,
                ciphertext: <span class="kw-2">*const </span>u8,
                plaintext_len: usize,
                ad: <span class="kw-2">*const </span>u8,
                ad_len: usize,
                data: <span class="kw-2">&amp;mut </span>InOut&lt;open_data_in&gt;,
            );
        }

        <span class="kw">let </span>out = <span class="kw">unsafe </span>{
            chacha20_poly1305_open(
                in_out.as_mut_ptr(),
                in_out.as_ptr().add(src.start),
                unprefixed_len,
                aad.as_ref().as_ptr(),
                aad.as_ref().len(),
                <span class="kw-2">&amp;mut </span>data,
            );
            <span class="kw-2">&amp;</span>data.out
        };

        <span class="kw">return </span><span class="prelude-val">Ok</span>(Tag(out.tag));
    }

    <span class="kw">let </span><span class="kw-2">mut </span>counter = Counter::zero(nonce);
    <span class="kw">let </span><span class="kw-2">mut </span>auth = {
        <span class="kw">let </span>key = derive_poly1305_key(chacha20_key, counter.increment());
        poly1305::Context::from_key(key, cpu_features)
    };

    poly1305_update_padded_16(<span class="kw-2">&amp;mut </span>auth, aad.as_ref());
    poly1305_update_padded_16(<span class="kw-2">&amp;mut </span>auth, <span class="kw-2">&amp;</span>in_out[src.clone()]);
    chacha20_key.encrypt_within(counter, in_out, src.clone());
    <span class="prelude-val">Ok</span>(finish(auth, aad.as_ref().len(), unprefixed_len))
}

<span class="attr">#[cfg(any(target_arch = <span class="string">"aarch64"</span>, target_arch = <span class="string">"x86_64"</span>))]
#[allow(clippy::needless_return)]
#[inline(always)]
</span><span class="kw">fn </span>has_integrated(cpu_features: cpu::Features) -&gt; bool {
    <span class="attr">#[cfg(target_arch = <span class="string">"aarch64"</span>)]
    </span>{
        <span class="kw">return </span>cpu::arm::NEON.available(cpu_features);
    }

    <span class="attr">#[cfg(target_arch = <span class="string">"x86_64"</span>)]
    </span>{
        <span class="kw">return </span>cpu::intel::SSE41.available(cpu_features);
    }
}

<span class="kw">fn </span>finish(<span class="kw-2">mut </span>auth: poly1305::Context, aad_len: usize, in_out_len: usize) -&gt; Tag {
    <span class="kw">let </span>block: [[u8; <span class="number">8</span>]; <span class="number">2</span>] = [aad_len, in_out_len]
        .map(u64_from_usize)
        .map(u64::to_le_bytes);
    auth.update(<span class="kw-2">&amp;</span>block.array_flatten());
    auth.finish()
}

<span class="kw">pub type </span>Key = chacha::Key;

<span class="comment">// Keep in sync with BoringSSL's `chacha20_poly1305_open_data` and
// `chacha20_poly1305_seal_data`.
</span><span class="attr">#[repr(C)]
#[cfg(any(target_arch = <span class="string">"aarch64"</span>, target_arch = <span class="string">"x86_64"</span>))]
</span><span class="kw">union </span>InOut&lt;T&gt;
<span class="kw">where
    </span>T: Copy,
{
    input: T,
    out: Out,
}

<span class="comment">// It isn't obvious whether the assembly code works for tags that aren't
// 16-byte aligned. In practice it will always be 16-byte aligned because it
// is embedded in a union where the other member of the union is 16-byte
// aligned.
</span><span class="attr">#[cfg(any(target_arch = <span class="string">"aarch64"</span>, target_arch = <span class="string">"x86_64"</span>))]
#[derive(Clone, Copy)]
#[repr(align(<span class="number">16</span>), C)]
</span><span class="kw">struct </span>Out {
    tag: [u8; <span class="kw">super</span>::TAG_LEN],
}

<span class="attr">#[inline]
</span><span class="kw">fn </span>poly1305_update_padded_16(ctx: <span class="kw-2">&amp;mut </span>poly1305::Context, input: <span class="kw-2">&amp;</span>[u8]) {
    <span class="kw">if </span>!input.is_empty() {
        ctx.update(input);
        <span class="kw">let </span>remainder_len = input.len() % poly1305::BLOCK_LEN;
        <span class="kw">if </span>remainder_len != <span class="number">0 </span>{
            <span class="kw">const </span>ZEROES: [u8; poly1305::BLOCK_LEN] = [<span class="number">0</span>; poly1305::BLOCK_LEN];
            ctx.update(<span class="kw-2">&amp;</span>ZEROES[..(poly1305::BLOCK_LEN - remainder_len)])
        }
    }
}

<span class="comment">// Also used by chacha20_poly1305_openssh.
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>derive_poly1305_key(chacha_key: <span class="kw-2">&amp;</span>chacha::Key, iv: Iv) -&gt; poly1305::Key {
    <span class="kw">let </span><span class="kw-2">mut </span>key_bytes = [<span class="number">0u8</span>; poly1305::KEY_LEN];
    chacha_key.encrypt_iv_xor_in_place(iv, <span class="kw-2">&amp;mut </span>key_bytes);
    poly1305::Key::new(key_bytes)
}
</code></pre></div></section></main></body></html>