<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rustls-webpki-0.101.7/src/end_entity.rs`."><title>end_entity.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="webpki" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../webpki/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#293" id="293">293</a>
<a href="#294" id="294">294</a>
<a href="#295" id="295">295</a>
<a href="#296" id="296">296</a>
<a href="#297" id="297">297</a>
<a href="#298" id="298">298</a>
<a href="#299" id="299">299</a>
<a href="#300" id="300">300</a>
<a href="#301" id="301">301</a>
<a href="#302" id="302">302</a>
<a href="#303" id="303">303</a>
<a href="#304" id="304">304</a>
<a href="#305" id="305">305</a>
<a href="#306" id="306">306</a>
<a href="#307" id="307">307</a>
<a href="#308" id="308">308</a>
<a href="#309" id="309">309</a>
<a href="#310" id="310">310</a>
<a href="#311" id="311">311</a>
<a href="#312" id="312">312</a>
<a href="#313" id="313">313</a>
<a href="#314" id="314">314</a>
<a href="#315" id="315">315</a>
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2015-2021 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

</span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">use </span><span class="kw">crate</span>::subject_name::GeneralDnsNameRef;
<span class="kw">use crate</span>::{
    cert, signed_data, subject_name, verify_cert, CertRevocationList, Error, KeyUsage,
    SignatureAlgorithm, SubjectNameRef, Time, TrustAnchor,
};
<span class="attr">#[allow(deprecated)]
</span><span class="kw">use crate</span>::{TlsClientTrustAnchors, TlsServerTrustAnchors};

<span class="doccomment">/// An end-entity certificate.
///
/// Server certificate processing in a TLS connection consists of several
/// steps. All of these steps are necessary:
///
/// * `EndEntityCert.verify_is_valid_tls_server_cert`: Verify that the server's
///   certificate is currently valid *for use by a TLS server*.
/// * `EndEntityCert.verify_is_valid_for_subject_name`: Verify that the server's
///   certificate is valid for the host or IP address that is being connected to.
///
/// * `EndEntityCert.verify_signature`: Verify that the signature of server's
///   `ServerKeyExchange` message is valid for the server's certificate.
///
/// Client certificate processing in a TLS connection consists of analogous
/// steps. All of these steps are necessary:
///
/// * `EndEntityCert.verify_is_valid_tls_client_cert`: Verify that the client's
///   certificate is currently valid *for use by a TLS client*.
/// * `EndEntityCert.verify_signature`: Verify that the client's signature in
///   its `CertificateVerify` message is valid using the public key from the
///   client's certificate.
///
/// Although it would be less error-prone to combine all these steps into a
/// single function call, some significant optimizations are possible if the
/// three steps are processed separately (in parallel). It does not matter much
/// which order the steps are done in, but **all of these steps must completed
/// before application data is sent and before received application data is
/// processed**. `EndEntityCert::from` is an inexpensive operation and is
/// deterministic, so if these tasks are done in multiple threads, it is
/// probably best to just call `EndEntityCert::from` multiple times (before each
/// operation) for the same DER-encoded ASN.1 certificate bytes.
</span><span class="kw">pub struct </span>EndEntityCert&lt;<span class="lifetime">'a</span>&gt; {
    inner: cert::Cert&lt;<span class="lifetime">'a</span>&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; TryFrom&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8]&gt; <span class="kw">for </span>EndEntityCert&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Error = Error;

    <span class="doccomment">/// Parse the ASN.1 DER-encoded X.509 encoding of the certificate
    /// `cert_der`.
    </span><span class="kw">fn </span>try_from(cert_der: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, <span class="self">Self</span>::Error&gt; {
        <span class="prelude-val">Ok</span>(<span class="self">Self </span>{
            inner: cert::Cert::from_der(
                untrusted::Input::from(cert_der),
                cert::EndEntityOrCa::EndEntity,
            )<span class="question-mark">?</span>,
        })
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; EndEntityCert&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>inner(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>cert::Cert {
        <span class="kw-2">&amp;</span><span class="self">self</span>.inner
    }

    <span class="kw">fn </span>verify_is_valid_cert(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        supported_sig_algs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>SignatureAlgorithm],
        trust_anchors: <span class="kw-2">&amp;</span>[TrustAnchor],
        intermediate_certs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>[u8]],
        time: Time,
        eku: KeyUsage,
        crls: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span><span class="kw">dyn </span>CertRevocationList],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        verify_cert::build_chain(
            <span class="kw-2">&amp;</span>verify_cert::ChainOptions {
                eku,
                supported_sig_algs,
                trust_anchors,
                intermediate_certs,
                crls,
            },
            <span class="kw-2">&amp;</span><span class="self">self</span>.inner,
            time,
        )
    }

    <span class="doccomment">/// Verifies that the end-entity certificate is valid for use against the
    /// specified Extended Key Usage (EKU).
    ///
    /// * `supported_sig_algs` is the list of signature algorithms that are
    ///   trusted for use in certificate signatures; the end-entity certificate's
    ///   public key is not validated against this list.
    /// * `trust_anchors` is the list of root CAs to trust
    /// * `intermediate_certs` is the sequence of intermediate certificates that
    ///   the server sent in the TLS handshake.
    /// * `time` is the time for which the validation is effective (usually the
    ///   current time).
    /// * `usage` is the intended usage of the certificate, indicating what kind
    ///   of usage we're verifying the certificate for.
    /// * `crls` is the list of certificate revocation lists to check
    ///   the certificate against.
    </span><span class="kw">pub fn </span>verify_for_usage(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        supported_sig_algs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>SignatureAlgorithm],
        trust_anchors: <span class="kw-2">&amp;</span>[TrustAnchor],
        intermediate_certs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>[u8]],
        time: Time,
        usage: KeyUsage,
        crls: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span><span class="kw">dyn </span>CertRevocationList],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="self">self</span>.verify_is_valid_cert(
            supported_sig_algs,
            trust_anchors,
            intermediate_certs,
            time,
            usage,
            crls,
        )
    }

    <span class="doccomment">/// Verifies that the end-entity certificate is valid for use by a TLS
    /// server.
    ///
    /// `supported_sig_algs` is the list of signature algorithms that are
    /// trusted for use in certificate signatures; the end-entity certificate's
    /// public key is not validated against this list. `trust_anchors` is the
    /// list of root CAs to trust. `intermediate_certs` is the sequence of
    /// intermediate certificates that the server sent in the TLS handshake.
    /// `time` is the time for which the validation is effective (usually the
    /// current time).
    </span><span class="attr">#[allow(deprecated)]
    #[deprecated(
        since = <span class="string">"0.101.2"</span>,
        note = <span class="string">"The per-usage trust anchor representations and verification functions are deprecated in \
        favor of the general-purpose `TrustAnchor` type and `EndEntity::verify_for_usage` function. \
        The new `verify_for_usage` function expresses trust anchor and end entity purpose with the \
        key usage argument."
    </span>)]
    </span><span class="kw">pub fn </span>verify_is_valid_tls_server_cert(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        supported_sig_algs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>SignatureAlgorithm],
        <span class="kw-2">&amp;</span>TlsServerTrustAnchors(trust_anchors): <span class="kw-2">&amp;</span>TlsServerTrustAnchors,
        intermediate_certs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>[u8]],
        time: Time,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="self">self</span>.verify_is_valid_cert(
            supported_sig_algs,
            trust_anchors,
            intermediate_certs,
            time,
            KeyUsage::server_auth(),
            <span class="kw-2">&amp;</span>[],
        )
    }

    <span class="doccomment">/// Verifies that the end-entity certificate is valid for use by a TLS
    /// client.
    ///
    /// `supported_sig_algs` is the list of signature algorithms that are
    /// trusted for use in certificate signatures; the end-entity certificate's
    /// public key is not validated against this list. `trust_anchors` is the
    /// list of root CAs to trust. `intermediate_certs` is the sequence of
    /// intermediate certificates that the client sent in the TLS handshake.
    /// `cert` is the purported end-entity certificate of the client. `time` is
    /// the time for which the validation is effective (usually the current
    /// time).
    </span><span class="attr">#[allow(deprecated)]
    #[deprecated(
        since = <span class="string">"0.101.2"</span>,
        note = <span class="string">"The per-usage trust anchor representations and verification functions are deprecated in \
        favor of the general-purpose `TrustAnchor` type and `EndEntity::verify_for_usage` function. \
        The new `verify_for_usage` function expresses trust anchor and end entity purpose with the \
        key usage argument."
    </span>)]
    </span><span class="kw">pub fn </span>verify_is_valid_tls_client_cert(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        supported_sig_algs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>SignatureAlgorithm],
        <span class="kw-2">&amp;</span>TlsClientTrustAnchors(trust_anchors): <span class="kw-2">&amp;</span>TlsClientTrustAnchors,
        intermediate_certs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>[u8]],
        time: Time,
        crls: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span><span class="kw">dyn </span>CertRevocationList],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="self">self</span>.verify_is_valid_cert(
            supported_sig_algs,
            trust_anchors,
            intermediate_certs,
            time,
            KeyUsage::client_auth(),
            crls,
        )
    }

    <span class="doccomment">/// Verifies that the certificate is valid for the given Subject Name.
    </span><span class="kw">pub fn </span>verify_is_valid_for_subject_name(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        subject_name: SubjectNameRef,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        subject_name::verify_cert_subject_name(<span class="self">self</span>, subject_name)
    }

    <span class="doccomment">/// Verifies the signature `signature` of message `msg` using the
    /// certificate's public key.
    ///
    /// `signature_alg` is the algorithm to use to
    /// verify the signature; the certificate's public key is verified to be
    /// compatible with this algorithm.
    ///
    /// For TLS 1.2, `signature` corresponds to TLS's
    /// `DigitallySigned.signature` and `signature_alg` corresponds to TLS's
    /// `DigitallySigned.algorithm` of TLS type `SignatureAndHashAlgorithm`. In
    /// TLS 1.2 a single `SignatureAndHashAlgorithm` may map to multiple
    /// `SignatureAlgorithm`s. For example, a TLS 1.2
    /// `SignatureAndHashAlgorithm` of (ECDSA, SHA-256) may map to any or all
    /// of {`ECDSA_P256_SHA256`, `ECDSA_P384_SHA256`}, depending on how the TLS
    /// implementation is configured.
    ///
    /// For current TLS 1.3 drafts, `signature_alg` corresponds to TLS's
    /// `algorithm` fields of type `SignatureScheme`. There is (currently) a
    /// one-to-one correspondence between TLS 1.3's `SignatureScheme` and
    /// `SignatureAlgorithm`.
    </span><span class="kw">pub fn </span>verify_signature(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        signature_alg: <span class="kw-2">&amp;</span>SignatureAlgorithm,
        msg: <span class="kw-2">&amp;</span>[u8],
        signature: <span class="kw-2">&amp;</span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        signed_data::verify_signature(
            signature_alg,
            <span class="self">self</span>.inner.spki.value(),
            untrusted::Input::from(msg),
            untrusted::Input::from(signature),
        )
    }

    <span class="doccomment">/// Returns a list of the DNS names provided in the subject alternative names extension
    ///
    /// This function must not be used to implement custom DNS name verification.
    /// Verification functions are already provided as `verify_is_valid_for_dns_name`
    /// and `verify_is_valid_for_at_least_one_dns_name`.
    </span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    #[cfg_attr(docsrs, doc(cfg(feature = <span class="string">"alloc"</span>)))]
    </span><span class="kw">pub fn </span>dns_names(<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="kw">impl </span>Iterator&lt;Item = GeneralDnsNameRef&lt;<span class="lifetime">'a</span>&gt;&gt;, Error&gt; {
        subject_name::list_cert_dns_names(<span class="self">self</span>)
    }
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;
    <span class="kw">use </span><span class="kw">crate</span>::test_utils;

    <span class="comment">// This test reproduces https://github.com/rustls/webpki/issues/167 --- an
    // end-entity cert where the common name is a `PrintableString` rather than
    // a `UTF8String` cannot iterate over its subject alternative names.
    </span><span class="attr">#[test]
    </span><span class="kw">fn </span>printable_string_common_name() {
        <span class="kw">const </span>DNS_NAME: <span class="kw-2">&amp;</span>str = <span class="string">"test.example.com"</span>;

        <span class="kw">let </span>issuer = test_utils::make_issuer(<span class="string">"Test"</span>, <span class="prelude-val">None</span>);

        <span class="kw">let </span>ee_cert_der = {
            <span class="kw">let </span><span class="kw-2">mut </span>params = rcgen::CertificateParams::new(<span class="macro">vec!</span>[DNS_NAME.to_string()]);
            <span class="comment">// construct a certificate that uses `PrintableString` as the
            // common name value, rather than `UTF8String`.
            </span>params.distinguished_name.push(
                rcgen::DnType::CommonName,
                rcgen::DnValue::PrintableString(<span class="string">"example.com"</span>.to_string()),
            );
            params.is_ca = rcgen::IsCa::ExplicitNoCa;
            params.alg = test_utils::RCGEN_SIGNATURE_ALG;
            <span class="kw">let </span>cert = rcgen::Certificate::from_params(params)
                .expect(<span class="string">"failed to make ee cert (this is a test bug)"</span>);
            cert.serialize_der_with_signer(<span class="kw-2">&amp;</span>issuer)
                .expect(<span class="string">"failed to serialize signed ee cert (this is a test bug)"</span>)
        };

        expect_dns_name(<span class="kw-2">&amp;</span>ee_cert_der, DNS_NAME);
    }

    <span class="comment">// This test reproduces https://github.com/rustls/webpki/issues/167 --- an
    // end-entity cert where the common name is an empty SEQUENCE.
    </span><span class="attr">#[test]
    </span><span class="kw">fn </span>empty_sequence_common_name() {
        <span class="comment">// handcrafted cert DER produced using `ascii2der`, since `rcgen` is
        // unwilling to generate this particular weird cert.
        </span><span class="kw">let </span>ee_cert_der = <span class="macro">include_bytes!</span>(<span class="string">"../tests/misc/empty_sequence_common_name.der"</span>).as_slice();
        expect_dns_name(ee_cert_der, <span class="string">"example.com"</span>);
    }

    <span class="kw">fn </span>expect_dns_name(der: <span class="kw-2">&amp;</span>[u8], name: <span class="kw-2">&amp;</span>str) {
        <span class="kw">let </span>cert =
            EndEntityCert::try_from(der).expect(<span class="string">"should parse end entity certificate correctly"</span>);

        <span class="kw">let </span><span class="kw-2">mut </span>names = cert
            .dns_names()
            .expect(<span class="string">"should get all DNS names correctly for end entity cert"</span>);
        <span class="macro">assert_eq!</span>(names.next().map(&lt;<span class="kw-2">&amp;</span>str&gt;::from), <span class="prelude-val">Some</span>(name));
        <span class="macro">assert_eq!</span>(names.next().map(&lt;<span class="kw-2">&amp;</span>str&gt;::from), <span class="prelude-val">None</span>);
    }
}
</code></pre></div></section></main></body></html>