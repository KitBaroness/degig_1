<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.17.8/src/ec/suite_b/ecdsa/verification.rs`."><title>verification.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../../" data-static-root-path="../../../../../static.files/" data-current-crate="ring" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../../src-files.js"></script><script defer src="../../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../../ring/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#316" id="316">316</a>
<a href="#317" id="317">317</a>
<a href="#318" id="318">318</a>
<a href="#319" id="319">319</a>
<a href="#320" id="320">320</a>
<a href="#321" id="321">321</a>
<a href="#322" id="322">322</a>
<a href="#323" id="323">323</a>
<a href="#324" id="324">324</a>
<a href="#325" id="325">325</a>
<a href="#326" id="326">326</a>
<a href="#327" id="327">327</a>
<a href="#328" id="328">328</a>
<a href="#329" id="329">329</a>
<a href="#330" id="330">330</a>
<a href="#331" id="331">331</a>
<a href="#332" id="332">332</a>
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

</span><span class="doccomment">//! ECDSA Signatures using the P-256 and P-384 curves.

</span><span class="kw">use </span><span class="kw">super</span>::digest_scalar::digest_scalar;
<span class="kw">use crate</span>::{
    arithmetic::montgomery::<span class="kw-2">*</span>,
    digest,
    ec::suite_b::{ops::<span class="kw-2">*</span>, public_key::<span class="kw-2">*</span>, verify_jacobian_point_is_on_the_curve},
    error,
    io::der,
    limb, sealed, signature,
};

<span class="doccomment">/// An ECDSA verification algorithm.
</span><span class="kw">pub struct </span>EcdsaVerificationAlgorithm {
    ops: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>PublicScalarOps,
    digest_alg: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>digest::Algorithm,
    split_rs:
        <span class="kw">for</span>&lt;<span class="lifetime">'a</span>&gt; <span class="kw">fn</span>(
            ops: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>ScalarOps,
            input: <span class="kw-2">&amp;mut </span>untrusted::Reader&lt;<span class="lifetime">'a</span>&gt;,
        )
            -&gt; <span class="prelude-ty">Result</span>&lt;(untrusted::Input&lt;<span class="lifetime">'a</span>&gt;, untrusted::Input&lt;<span class="lifetime">'a</span>&gt;), error::Unspecified&gt;,
    id: AlgorithmID,
}

<span class="attr">#[derive(Debug)]
</span><span class="kw">enum </span>AlgorithmID {
    ECDSA_P256_SHA256_ASN1,
    ECDSA_P256_SHA256_FIXED,
    ECDSA_P256_SHA384_ASN1,
    ECDSA_P384_SHA256_ASN1,
    ECDSA_P384_SHA384_ASN1,
    ECDSA_P384_SHA384_FIXED,
}

<span class="macro">derive_debug_via_id!</span>(EcdsaVerificationAlgorithm);

<span class="kw">impl </span>signature::VerificationAlgorithm <span class="kw">for </span>EcdsaVerificationAlgorithm {
    <span class="kw">fn </span>verify(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        public_key: untrusted::Input,
        msg: untrusted::Input,
        signature: untrusted::Input,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), error::Unspecified&gt; {
        <span class="kw">let </span>e = {
            <span class="comment">// NSA Guide Step 2: "Use the selected hash function to compute H =
            // Hash(M)."
            </span><span class="kw">let </span>h = digest::digest(<span class="self">self</span>.digest_alg, msg.as_slice_less_safe());

            <span class="comment">// NSA Guide Step 3: "Convert the bit string H to an integer e as
            // described in Appendix B.2."
            </span>digest_scalar(<span class="self">self</span>.ops.scalar_ops, h)
        };

        <span class="self">self</span>.verify_digest(public_key, e, signature)
    }
}

<span class="kw">impl </span>EcdsaVerificationAlgorithm {
    <span class="doccomment">/// This is intentionally not public.
    </span><span class="kw">fn </span>verify_digest(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        public_key: untrusted::Input,
        e: Scalar,
        signature: untrusted::Input,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), error::Unspecified&gt; {
        <span class="comment">// NSA Suite B Implementer's Guide to ECDSA Section 3.4.2.

        </span><span class="kw">let </span>public_key_ops = <span class="self">self</span>.ops.public_key_ops;
        <span class="kw">let </span>scalar_ops = <span class="self">self</span>.ops.scalar_ops;

        <span class="comment">// NSA Guide Prerequisites:
        //
        //    Prior to accepting a verified digital signature as valid the
        //    verifier shall have:
        //
        //    1. assurance of the signatory’s claimed identity,
        //    2. an authentic copy of the domain parameters, (q, FR, a, b, SEED,
        //       G, n, h),
        //    3. assurance of the validity of the public key, and
        //    4. assurance that the claimed signatory actually possessed the
        //       private key that was used to generate the digital signature at
        //       the time that the signature was generated.
        //
        // Prerequisites #1 and #4 are outside the scope of what this function
        // can do. Prerequisite #2 is handled implicitly as the domain
        // parameters are hard-coded into the source. Prerequisite #3 is
        // handled by `parse_uncompressed_point`.
        </span><span class="kw">let </span>peer_pub_key = parse_uncompressed_point(public_key_ops, public_key)<span class="question-mark">?</span>;

        <span class="kw">let </span>(r, s) = signature.read_all(error::Unspecified, |input| {
            (<span class="self">self</span>.split_rs)(scalar_ops, input)
        })<span class="question-mark">?</span>;

        <span class="comment">// NSA Guide Step 1: "If r and s are not both integers in the interval
        // [1, n − 1], output INVALID."
        </span><span class="kw">let </span>r = scalar_parse_big_endian_variable(public_key_ops.common, limb::AllowZero::No, r)<span class="question-mark">?</span>;
        <span class="kw">let </span>s = scalar_parse_big_endian_variable(public_key_ops.common, limb::AllowZero::No, s)<span class="question-mark">?</span>;

        <span class="comment">// NSA Guide Step 4: "Compute w = s**−1 mod n, using the routine in
        // Appendix B.1."
        </span><span class="kw">let </span>w = <span class="self">self</span>.ops.scalar_inv_to_mont_vartime(<span class="kw-2">&amp;</span>s);

        <span class="comment">// NSA Guide Step 5: "Compute u1 = (e * w) mod n, and compute
        // u2 = (r * w) mod n."
        </span><span class="kw">let </span>u1 = scalar_ops.scalar_product(<span class="kw-2">&amp;</span>e, <span class="kw-2">&amp;</span>w);
        <span class="kw">let </span>u2 = scalar_ops.scalar_product(<span class="kw-2">&amp;</span>r, <span class="kw-2">&amp;</span>w);

        <span class="comment">// NSA Guide Step 6: "Compute the elliptic curve point
        // R = (xR, yR) = u1*G + u2*Q, using EC scalar multiplication and EC
        // addition. If R is equal to the point at infinity, output INVALID."
        </span><span class="kw">let </span>product = (<span class="self">self</span>.ops.twin_mul)(<span class="kw-2">&amp;</span>u1, <span class="kw-2">&amp;</span>u2, <span class="kw-2">&amp;</span>peer_pub_key);

        <span class="comment">// Verify that the point we computed is on the curve; see
        // `verify_affine_point_is_on_the_curve_scaled` for details on why. It
        // would be more secure to do the check on the affine coordinates if we
        // were going to convert to affine form (again, see
        // `verify_affine_point_is_on_the_curve_scaled` for details on why).
        // But, we're going to avoid converting to affine for performance
        // reasons, so we do the verification using the Jacobian coordinates.
        </span><span class="kw">let </span>z2 = verify_jacobian_point_is_on_the_curve(public_key_ops.common, <span class="kw-2">&amp;</span>product)<span class="question-mark">?</span>;

        <span class="comment">// NSA Guide Step 7: "Compute v = xR mod n."
        // NSA Guide Step 8: "Compare v and r0. If v = r0, output VALID;
        // otherwise, output INVALID."
        //
        // Instead, we use Greg Maxwell's trick to avoid the inversion mod `q`
        // that would be necessary to compute the affine X coordinate.
        </span><span class="kw">let </span>x = public_key_ops.common.point_x(<span class="kw-2">&amp;</span>product);
        <span class="kw">fn </span>sig_r_equals_x(
            ops: <span class="kw-2">&amp;</span>PublicScalarOps,
            r: <span class="kw-2">&amp;</span>Elem&lt;Unencoded&gt;,
            x: <span class="kw-2">&amp;</span>Elem&lt;R&gt;,
            z2: <span class="kw-2">&amp;</span>Elem&lt;R&gt;,
        ) -&gt; bool {
            <span class="kw">let </span>cops = ops.public_key_ops.common;
            <span class="kw">let </span>r_jacobian = cops.elem_product(z2, r);
            <span class="kw">let </span>x = cops.elem_unencoded(x);
            ops.elem_equals_vartime(<span class="kw-2">&amp;</span>r_jacobian, <span class="kw-2">&amp;</span>x)
        }
        <span class="kw">let </span><span class="kw-2">mut </span>r = <span class="self">self</span>.ops.scalar_as_elem(<span class="kw-2">&amp;</span>r);
        <span class="kw">if </span>sig_r_equals_x(<span class="self">self</span>.ops, <span class="kw-2">&amp;</span>r, <span class="kw-2">&amp;</span>x, <span class="kw-2">&amp;</span>z2) {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(());
        }
        <span class="kw">if </span><span class="self">self</span>.ops.elem_less_than(<span class="kw-2">&amp;</span>r, <span class="kw-2">&amp;</span><span class="self">self</span>.ops.q_minus_n) {
            <span class="self">self</span>.ops.scalar_ops.common.elem_add(<span class="kw-2">&amp;mut </span>r, <span class="self">self</span>.ops.n());
            <span class="kw">if </span>sig_r_equals_x(<span class="self">self</span>.ops, <span class="kw-2">&amp;</span>r, <span class="kw-2">&amp;</span>x, <span class="kw-2">&amp;</span>z2) {
                <span class="kw">return </span><span class="prelude-val">Ok</span>(());
            }
        }

        <span class="prelude-val">Err</span>(error::Unspecified)
    }
}

<span class="kw">impl </span>sealed::Sealed <span class="kw">for </span>EcdsaVerificationAlgorithm {}

<span class="kw">fn </span>split_rs_fixed&lt;<span class="lifetime">'a</span>&gt;(
    ops: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>ScalarOps,
    input: <span class="kw-2">&amp;mut </span>untrusted::Reader&lt;<span class="lifetime">'a</span>&gt;,
) -&gt; <span class="prelude-ty">Result</span>&lt;(untrusted::Input&lt;<span class="lifetime">'a</span>&gt;, untrusted::Input&lt;<span class="lifetime">'a</span>&gt;), error::Unspecified&gt; {
    <span class="kw">let </span>scalar_len = ops.scalar_bytes_len();
    <span class="kw">let </span>r = input.read_bytes(scalar_len)<span class="question-mark">?</span>;
    <span class="kw">let </span>s = input.read_bytes(scalar_len)<span class="question-mark">?</span>;
    <span class="prelude-val">Ok</span>((r, s))
}

<span class="kw">fn </span>split_rs_asn1&lt;<span class="lifetime">'a</span>&gt;(
    _ops: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>ScalarOps,
    input: <span class="kw-2">&amp;mut </span>untrusted::Reader&lt;<span class="lifetime">'a</span>&gt;,
) -&gt; <span class="prelude-ty">Result</span>&lt;(untrusted::Input&lt;<span class="lifetime">'a</span>&gt;, untrusted::Input&lt;<span class="lifetime">'a</span>&gt;), error::Unspecified&gt; {
    der::nested(input, der::Tag::Sequence, error::Unspecified, |input| {
        <span class="kw">let </span>r = der::positive_integer(input)<span class="question-mark">?</span>.big_endian_without_leading_zero_as_input();
        <span class="kw">let </span>s = der::positive_integer(input)<span class="question-mark">?</span>.big_endian_without_leading_zero_as_input();
        <span class="prelude-val">Ok</span>((r, s))
    })
}

<span class="doccomment">/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the
/// P-256 curve and SHA-256.
///
/// See "`ECDSA_*_FIXED` Details" in `ring::signature`'s module-level
/// documentation for more details.
</span><span class="kw">pub static </span>ECDSA_P256_SHA256_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    ops: <span class="kw-2">&amp;</span>p256::PUBLIC_SCALAR_OPS,
    digest_alg: <span class="kw-2">&amp;</span>digest::SHA256,
    split_rs: split_rs_fixed,
    id: AlgorithmID::ECDSA_P256_SHA256_FIXED,
};

<span class="doccomment">/// Verification of fixed-length (PKCS#11 style) ECDSA signatures using the
/// P-384 curve and SHA-384.
///
/// See "`ECDSA_*_FIXED` Details" in `ring::signature`'s module-level
/// documentation for more details.
</span><span class="kw">pub static </span>ECDSA_P384_SHA384_FIXED: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    ops: <span class="kw-2">&amp;</span>p384::PUBLIC_SCALAR_OPS,
    digest_alg: <span class="kw-2">&amp;</span>digest::SHA384,
    split_rs: split_rs_fixed,
    id: AlgorithmID::ECDSA_P384_SHA384_FIXED,
};

<span class="doccomment">/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-256 curve
/// and SHA-256.
///
/// See "`ECDSA_*_ASN1` Details" in `ring::signature`'s module-level
/// documentation for more details.
</span><span class="kw">pub static </span>ECDSA_P256_SHA256_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    ops: <span class="kw-2">&amp;</span>p256::PUBLIC_SCALAR_OPS,
    digest_alg: <span class="kw-2">&amp;</span>digest::SHA256,
    split_rs: split_rs_asn1,
    id: AlgorithmID::ECDSA_P256_SHA256_ASN1,
};

<span class="doccomment">/// *Not recommended*. Verification of ASN.1 DER-encoded ECDSA signatures using
/// the P-256 curve and SHA-384.
///
/// In most situations, P-256 should be used only with SHA-256 and P-384
/// should be used only with SHA-384. However, in some cases, particularly TLS
/// on the web, it is necessary to support P-256 with SHA-384 for compatibility
/// with widely-deployed implementations that do not follow these guidelines.
///
/// See "`ECDSA_*_ASN1` Details" in `ring::signature`'s module-level
/// documentation for more details.
</span><span class="kw">pub static </span>ECDSA_P256_SHA384_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    ops: <span class="kw-2">&amp;</span>p256::PUBLIC_SCALAR_OPS,
    digest_alg: <span class="kw-2">&amp;</span>digest::SHA384,
    split_rs: split_rs_asn1,
    id: AlgorithmID::ECDSA_P256_SHA384_ASN1,
};

<span class="doccomment">/// *Not recommended*. Verification of ASN.1 DER-encoded ECDSA signatures using
/// the P-384 curve and SHA-256.
///
/// In most situations, P-256 should be used only with SHA-256 and P-384
/// should be used only with SHA-384. However, in some cases, particularly TLS
/// on the web, it is necessary to support P-256 with SHA-384 for compatibility
/// with widely-deployed implementations that do not follow these guidelines.
///
/// See "`ECDSA_*_ASN1` Details" in `ring::signature`'s module-level
/// documentation for more details.
</span><span class="kw">pub static </span>ECDSA_P384_SHA256_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    ops: <span class="kw-2">&amp;</span>p384::PUBLIC_SCALAR_OPS,
    digest_alg: <span class="kw-2">&amp;</span>digest::SHA256,
    split_rs: split_rs_asn1,
    id: AlgorithmID::ECDSA_P384_SHA256_ASN1,
};

<span class="doccomment">/// Verification of ASN.1 DER-encoded ECDSA signatures using the P-384 curve
/// and SHA-384.
///
/// See "`ECDSA_*_ASN1` Details" in `ring::signature`'s module-level
/// documentation for more details.
</span><span class="kw">pub static </span>ECDSA_P384_SHA384_ASN1: EcdsaVerificationAlgorithm = EcdsaVerificationAlgorithm {
    ops: <span class="kw-2">&amp;</span>p384::PUBLIC_SCALAR_OPS,
    digest_alg: <span class="kw-2">&amp;</span>digest::SHA384,
    split_rs: split_rs_asn1,
    id: AlgorithmID::ECDSA_P384_SHA384_ASN1,
};

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">extern crate </span>alloc;
    <span class="kw">use super</span>::<span class="kw-2">*</span>;
    <span class="kw">use </span><span class="kw">crate</span>::test;
    <span class="kw">use </span>alloc::{vec, vec::Vec};

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_digest_based_test_vectors() {
        test::run(
            <span class="macro">test_file!</span>(<span class="string">"../../../../crypto/fipsmodule/ecdsa/ecdsa_verify_tests.txt"</span>),
            |section, test_case| {
                <span class="macro">assert_eq!</span>(section, <span class="string">""</span>);

                <span class="kw">let </span>curve_name = test_case.consume_string(<span class="string">"Curve"</span>);

                <span class="kw">let </span>public_key = {
                    <span class="kw">let </span><span class="kw-2">mut </span>public_key = <span class="macro">vec!</span>[<span class="number">0x04</span>];
                    public_key.extend(<span class="kw-2">&amp;</span>test_case.consume_bytes(<span class="string">"X"</span>));
                    public_key.extend(<span class="kw-2">&amp;</span>test_case.consume_bytes(<span class="string">"Y"</span>));
                    public_key
                };

                <span class="kw">let </span>digest = test_case.consume_bytes(<span class="string">"Digest"</span>);

                <span class="kw">let </span>sig = {
                    <span class="kw">let </span><span class="kw-2">mut </span>sig = Vec::new();
                    sig.extend(<span class="kw-2">&amp;</span>test_case.consume_bytes(<span class="string">"R"</span>));
                    sig.extend(<span class="kw-2">&amp;</span>test_case.consume_bytes(<span class="string">"S"</span>));
                    sig
                };

                <span class="kw">let </span>invalid = test_case.consume_optional_string(<span class="string">"Invalid"</span>);

                <span class="kw">let </span>alg = <span class="kw">match </span>curve_name.as_str() {
                    <span class="string">"P-256" </span>=&gt; <span class="kw-2">&amp;</span>ECDSA_P256_SHA256_FIXED,
                    <span class="string">"P-384" </span>=&gt; <span class="kw-2">&amp;</span>ECDSA_P384_SHA384_FIXED,
                    <span class="kw">_ </span>=&gt; {
                        <span class="macro">panic!</span>(<span class="string">"Unsupported curve: {}"</span>, curve_name);
                    }
                };

                <span class="kw">let </span>digest = <span class="kw">super</span>::<span class="kw">super</span>::digest_scalar::digest_bytes_scalar(
                    alg.ops.scalar_ops,
                    <span class="kw-2">&amp;</span>digest[..],
                );
                <span class="kw">let </span>actual_result = alg.verify_digest(
                    untrusted::Input::from(<span class="kw-2">&amp;</span>public_key[..]),
                    digest,
                    untrusted::Input::from(<span class="kw-2">&amp;</span>sig[..]),
                );
                <span class="macro">assert_eq!</span>(actual_result.is_ok(), invalid.is_none());

                <span class="prelude-val">Ok</span>(())
            },
        );
    }
}
</code></pre></div></section></main></body></html>