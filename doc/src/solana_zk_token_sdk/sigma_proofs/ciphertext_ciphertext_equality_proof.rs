<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-zk-token-sdk-1.18.9/src/sigma_proofs/ciphertext_ciphertext_equality_proof.rs`."><title>ciphertext_ciphertext_equality_proof.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="solana_zk_token_sdk" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../solana_zk_token_sdk/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#333" id="333">333</a>
<a href="#334" id="334">334</a>
<a href="#335" id="335">335</a>
<a href="#336" id="336">336</a>
<a href="#337" id="337">337</a>
<a href="#338" id="338">338</a>
<a href="#339" id="339">339</a>
<a href="#340" id="340">340</a>
<a href="#341" id="341">341</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! The ciphertext-ciphertext equality sigma proof system.
//!
//! The protocol guarantees computational soundness (by the hardness of discrete log) and perfect
//! zero-knowledge in the random oracle model.

</span><span class="attr">#[cfg(not(target_os = <span class="string">"solana"</span>))]
</span><span class="kw">use </span>{
    <span class="kw">crate</span>::{
        encryption::{
            elgamal::{ElGamalCiphertext, ElGamalKeypair, ElGamalPubkey},
            pedersen::{PedersenOpening, G, H},
        },
        sigma_proofs::{canonical_scalar_from_optional_slice, ristretto_point_from_optional_slice},
        UNIT_LEN,
    },
    curve25519_dalek::traits::MultiscalarMul,
    rand::rngs::OsRng,
    zeroize::Zeroize,
};
<span class="kw">use </span>{
    <span class="kw">crate</span>::{
        sigma_proofs::errors::{EqualityProofVerificationError, SigmaProofVerificationError},
        transcript::TranscriptProtocol,
    },
    curve25519_dalek::{
        ristretto::{CompressedRistretto, RistrettoPoint},
        scalar::Scalar,
        traits::{IsIdentity, VartimeMultiscalarMul},
    },
    merlin::Transcript,
};

<span class="doccomment">/// Byte length of a ciphertext-ciphertext equality proof.
</span><span class="kw">const </span>CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_LEN: usize = UNIT_LEN * <span class="number">7</span>;

<span class="doccomment">/// The ciphertext-ciphertext equality proof.
///
/// Contains all the elliptic curve and scalar components that make up the sigma protocol.
</span><span class="attr">#[allow(non_snake_case)]
#[derive(Clone)]
</span><span class="kw">pub struct </span>CiphertextCiphertextEqualityProof {
    Y_0: CompressedRistretto,
    Y_1: CompressedRistretto,
    Y_2: CompressedRistretto,
    Y_3: CompressedRistretto,
    z_s: Scalar,
    z_x: Scalar,
    z_r: Scalar,
}

<span class="attr">#[allow(non_snake_case)]
#[cfg(not(target_os = <span class="string">"solana"</span>))]
</span><span class="kw">impl </span>CiphertextCiphertextEqualityProof {
    <span class="doccomment">/// Creates a ciphertext-ciphertext equality proof.
    ///
    /// The function does *not* hash the public key, ciphertext, or commitment into the transcript.
    /// For security, the caller (the main protocol) should hash these public components prior to
    /// invoking this constructor.
    ///
    /// This function is randomized. It uses `OsRng` internally to generate random scalars.
    ///
    /// * `source_keypair` - The ElGamal keypair associated with the first ciphertext to be proved
    /// * `destination_pubkey` - The ElGamal pubkey associated with the second ElGamal ciphertext
    /// * `source_ciphertext` - The first ElGamal ciphertext for which the prover knows a
    /// decryption key for
    /// * `destination_opening` - The opening (randomness) associated with the second ElGamal ciphertext
    /// * `amount` - The message associated with the ElGamal ciphertext and Pedersen commitment
    /// * `transcript` - The transcript that does the bookkeeping for the Fiat-Shamir heuristic
    </span><span class="kw">pub fn </span>new(
        source_keypair: <span class="kw-2">&amp;</span>ElGamalKeypair,
        destination_pubkey: <span class="kw-2">&amp;</span>ElGamalPubkey,
        source_ciphertext: <span class="kw-2">&amp;</span>ElGamalCiphertext,
        destination_opening: <span class="kw-2">&amp;</span>PedersenOpening,
        amount: u64,
        transcript: <span class="kw-2">&amp;mut </span>Transcript,
    ) -&gt; <span class="self">Self </span>{
        transcript.equality_proof_domain_separator();

        <span class="comment">// extract the relevant scalar and Ristretto points from the inputs
        </span><span class="kw">let </span>P_source = source_keypair.pubkey().get_point();
        <span class="kw">let </span>D_source = source_ciphertext.handle.get_point();
        <span class="kw">let </span>P_destination = destination_pubkey.get_point();

        <span class="kw">let </span>s = source_keypair.secret().get_scalar();
        <span class="kw">let </span>x = Scalar::from(amount);
        <span class="kw">let </span>r = destination_opening.get_scalar();

        <span class="comment">// generate random masking factors that also serves as nonces
        </span><span class="kw">let </span><span class="kw-2">mut </span>y_s = Scalar::random(<span class="kw-2">&amp;mut </span>OsRng);
        <span class="kw">let </span><span class="kw-2">mut </span>y_x = Scalar::random(<span class="kw-2">&amp;mut </span>OsRng);
        <span class="kw">let </span><span class="kw-2">mut </span>y_r = Scalar::random(<span class="kw-2">&amp;mut </span>OsRng);

        <span class="kw">let </span>Y_0 = (<span class="kw-2">&amp;</span>y_s * P_source).compress();
        <span class="kw">let </span>Y_1 =
            RistrettoPoint::multiscalar_mul(<span class="macro">vec!</span>[<span class="kw-2">&amp;</span>y_x, <span class="kw-2">&amp;</span>y_s], <span class="macro">vec!</span>[<span class="kw-2">&amp;</span>(<span class="kw-2">*</span>G), D_source]).compress();
        <span class="kw">let </span>Y_2 = RistrettoPoint::multiscalar_mul(<span class="macro">vec!</span>[<span class="kw-2">&amp;</span>y_x, <span class="kw-2">&amp;</span>y_r], <span class="macro">vec!</span>[<span class="kw-2">&amp;</span>(<span class="kw-2">*</span>G), <span class="kw-2">&amp;</span>(<span class="kw-2">*</span>H)]).compress();
        <span class="kw">let </span>Y_3 = (<span class="kw-2">&amp;</span>y_r * P_destination).compress();

        <span class="comment">// record masking factors in the transcript
        </span>transcript.append_point(<span class="string">b"Y_0"</span>, <span class="kw-2">&amp;</span>Y_0);
        transcript.append_point(<span class="string">b"Y_1"</span>, <span class="kw-2">&amp;</span>Y_1);
        transcript.append_point(<span class="string">b"Y_2"</span>, <span class="kw-2">&amp;</span>Y_2);
        transcript.append_point(<span class="string">b"Y_3"</span>, <span class="kw-2">&amp;</span>Y_3);

        <span class="kw">let </span>c = transcript.challenge_scalar(<span class="string">b"c"</span>);
        transcript.challenge_scalar(<span class="string">b"w"</span>);

        <span class="comment">// compute the masked values
        </span><span class="kw">let </span>z_s = <span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>c * s) + <span class="kw-2">&amp;</span>y_s;
        <span class="kw">let </span>z_x = <span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>c * <span class="kw-2">&amp;</span>x) + <span class="kw-2">&amp;</span>y_x;
        <span class="kw">let </span>z_r = <span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>c * r) + <span class="kw-2">&amp;</span>y_r;

        <span class="comment">// zeroize random scalars
        </span>y_s.zeroize();
        y_x.zeroize();
        y_r.zeroize();

        CiphertextCiphertextEqualityProof {
            Y_0,
            Y_1,
            Y_2,
            Y_3,
            z_s,
            z_x,
            z_r,
        }
    }

    <span class="doccomment">/// Verifies a ciphertext-ciphertext equality proof.
    ///
    /// * `source_pubkey` - The ElGamal pubkey associated with the first ciphertext to be proved
    /// * `destination_pubkey` - The ElGamal pubkey associated with the second ciphertext to be proved
    /// * `source_ciphertext` - The first ElGamal ciphertext to be proved
    /// * `destination_ciphertext` - The second ElGamal ciphertext to be proved
    /// * `transcript` - The transcript that does the bookkeeping for the Fiat-Shamir heuristic
    </span><span class="kw">pub fn </span>verify(
        <span class="self">self</span>,
        source_pubkey: <span class="kw-2">&amp;</span>ElGamalPubkey,
        destination_pubkey: <span class="kw-2">&amp;</span>ElGamalPubkey,
        source_ciphertext: <span class="kw-2">&amp;</span>ElGamalCiphertext,
        destination_ciphertext: <span class="kw-2">&amp;</span>ElGamalCiphertext,
        transcript: <span class="kw-2">&amp;mut </span>Transcript,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), EqualityProofVerificationError&gt; {
        transcript.equality_proof_domain_separator();

        <span class="comment">// extract the relevant scalar and Ristretto points from the inputs
        </span><span class="kw">let </span>P_source = source_pubkey.get_point();
        <span class="kw">let </span>C_source = source_ciphertext.commitment.get_point();
        <span class="kw">let </span>D_source = source_ciphertext.handle.get_point();

        <span class="kw">let </span>P_destination = destination_pubkey.get_point();
        <span class="kw">let </span>C_destination = destination_ciphertext.commitment.get_point();
        <span class="kw">let </span>D_destination = destination_ciphertext.handle.get_point();

        <span class="comment">// include Y_0, Y_1, Y_2 to transcript and extract challenges
        </span>transcript.validate_and_append_point(<span class="string">b"Y_0"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.Y_0)<span class="question-mark">?</span>;
        transcript.validate_and_append_point(<span class="string">b"Y_1"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.Y_1)<span class="question-mark">?</span>;
        transcript.validate_and_append_point(<span class="string">b"Y_2"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.Y_2)<span class="question-mark">?</span>;
        transcript.validate_and_append_point(<span class="string">b"Y_3"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.Y_3)<span class="question-mark">?</span>;

        <span class="kw">let </span>c = transcript.challenge_scalar(<span class="string">b"c"</span>);
        <span class="kw">let </span>w = transcript.challenge_scalar(<span class="string">b"w"</span>); <span class="comment">// w used for batch verification
        </span><span class="kw">let </span>ww = <span class="kw-2">&amp;</span>w * <span class="kw-2">&amp;</span>w;
        <span class="kw">let </span>www = <span class="kw-2">&amp;</span>w * <span class="kw-2">&amp;</span>ww;

        <span class="kw">let </span>w_negated = -<span class="kw-2">&amp;</span>w;
        <span class="kw">let </span>ww_negated = -<span class="kw-2">&amp;</span>ww;
        <span class="kw">let </span>www_negated = -<span class="kw-2">&amp;</span>www;

        <span class="comment">// check that the required algebraic condition holds
        </span><span class="kw">let </span>Y_0 = <span class="self">self
            </span>.Y_0
            .decompress()
            .ok_or(SigmaProofVerificationError::Deserialization)<span class="question-mark">?</span>;
        <span class="kw">let </span>Y_1 = <span class="self">self
            </span>.Y_1
            .decompress()
            .ok_or(SigmaProofVerificationError::Deserialization)<span class="question-mark">?</span>;
        <span class="kw">let </span>Y_2 = <span class="self">self
            </span>.Y_2
            .decompress()
            .ok_or(SigmaProofVerificationError::Deserialization)<span class="question-mark">?</span>;
        <span class="kw">let </span>Y_3 = <span class="self">self
            </span>.Y_3
            .decompress()
            .ok_or(SigmaProofVerificationError::Deserialization)<span class="question-mark">?</span>;

        <span class="kw">let </span>check = RistrettoPoint::vartime_multiscalar_mul(
            <span class="macro">vec!</span>[
                <span class="kw-2">&amp;</span><span class="self">self</span>.z_s,            <span class="comment">// z_s
                </span><span class="kw-2">&amp;</span>(-<span class="kw-2">&amp;</span>c),               <span class="comment">// -c
                </span><span class="kw-2">&amp;</span>(-<span class="kw-2">&amp;</span>Scalar::one()),   <span class="comment">// -identity
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>w * <span class="kw-2">&amp;</span><span class="self">self</span>.z_x),    <span class="comment">// w * z_x
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>w * <span class="kw-2">&amp;</span><span class="self">self</span>.z_s),    <span class="comment">// w * z_s
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>w_negated * <span class="kw-2">&amp;</span>c),   <span class="comment">// -w * c
                </span><span class="kw-2">&amp;</span>w_negated,           <span class="comment">// -w
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>ww * <span class="kw-2">&amp;</span><span class="self">self</span>.z_x),   <span class="comment">// ww * z_x
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>ww * <span class="kw-2">&amp;</span><span class="self">self</span>.z_r),   <span class="comment">// ww * z_r
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>ww_negated * <span class="kw-2">&amp;</span>c),  <span class="comment">// -ww * c
                </span><span class="kw-2">&amp;</span>ww_negated,          <span class="comment">// -ww
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>www * <span class="kw-2">&amp;</span><span class="self">self</span>.z_r),  <span class="comment">// z_r
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>www_negated * <span class="kw-2">&amp;</span>c), <span class="comment">// -www * c
                </span><span class="kw-2">&amp;</span>www_negated,
            ],
            <span class="macro">vec!</span>[
                P_source,      <span class="comment">// P_source
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">*</span>H),         <span class="comment">// H
                </span><span class="kw-2">&amp;</span>Y_0,          <span class="comment">// Y_0
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">*</span>G),         <span class="comment">// G
                </span>D_source,      <span class="comment">// D_source
                </span>C_source,      <span class="comment">// C_source
                </span><span class="kw-2">&amp;</span>Y_1,          <span class="comment">// Y_1
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">*</span>G),         <span class="comment">// G
                </span><span class="kw-2">&amp;</span>(<span class="kw-2">*</span>H),         <span class="comment">// H
                </span>C_destination, <span class="comment">// C_destination
                </span><span class="kw-2">&amp;</span>Y_2,          <span class="comment">// Y_2
                </span>P_destination, <span class="comment">// P_destination
                </span>D_destination, <span class="comment">// D_destination
                </span><span class="kw-2">&amp;</span>Y_3,          <span class="comment">// Y_3
            </span>],
        );

        <span class="kw">if </span>check.is_identity() {
            <span class="prelude-val">Ok</span>(())
        } <span class="kw">else </span>{
            <span class="prelude-val">Err</span>(SigmaProofVerificationError::AlgebraicRelation.into())
        }
    }

    <span class="kw">pub fn </span>to_bytes(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; [u8; CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_LEN] {
        <span class="kw">let </span><span class="kw-2">mut </span>buf = [<span class="number">0_u8</span>; CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_LEN];
        <span class="kw">let </span><span class="kw-2">mut </span>chunks = buf.chunks_mut(UNIT_LEN);

        chunks.next().unwrap().copy_from_slice(<span class="self">self</span>.Y_0.as_bytes());
        chunks.next().unwrap().copy_from_slice(<span class="self">self</span>.Y_1.as_bytes());
        chunks.next().unwrap().copy_from_slice(<span class="self">self</span>.Y_2.as_bytes());
        chunks.next().unwrap().copy_from_slice(<span class="self">self</span>.Y_3.as_bytes());
        chunks.next().unwrap().copy_from_slice(<span class="self">self</span>.z_s.as_bytes());
        chunks.next().unwrap().copy_from_slice(<span class="self">self</span>.z_x.as_bytes());
        chunks.next().unwrap().copy_from_slice(<span class="self">self</span>.z_r.as_bytes());

        buf
    }

    <span class="kw">pub fn </span>from_bytes(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, EqualityProofVerificationError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>chunks = bytes.chunks(UNIT_LEN);

        <span class="kw">let </span>Y_0 = ristretto_point_from_optional_slice(chunks.next())<span class="question-mark">?</span>;
        <span class="kw">let </span>Y_1 = ristretto_point_from_optional_slice(chunks.next())<span class="question-mark">?</span>;
        <span class="kw">let </span>Y_2 = ristretto_point_from_optional_slice(chunks.next())<span class="question-mark">?</span>;
        <span class="kw">let </span>Y_3 = ristretto_point_from_optional_slice(chunks.next())<span class="question-mark">?</span>;
        <span class="kw">let </span>z_s = canonical_scalar_from_optional_slice(chunks.next())<span class="question-mark">?</span>;
        <span class="kw">let </span>z_x = canonical_scalar_from_optional_slice(chunks.next())<span class="question-mark">?</span>;
        <span class="kw">let </span>z_r = canonical_scalar_from_optional_slice(chunks.next())<span class="question-mark">?</span>;

        <span class="prelude-val">Ok</span>(CiphertextCiphertextEqualityProof {
            Y_0,
            Y_1,
            Y_2,
            Y_3,
            z_s,
            z_x,
            z_r,
        })
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>test {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_ciphertext_ciphertext_equality_proof_correctness() {
        <span class="comment">// success case
        </span><span class="kw">let </span>source_keypair = ElGamalKeypair::new_rand();
        <span class="kw">let </span>destination_keypair = ElGamalKeypair::new_rand();
        <span class="kw">let </span>message: u64 = <span class="number">55</span>;

        <span class="kw">let </span>source_ciphertext = source_keypair.pubkey().encrypt(message);

        <span class="kw">let </span>destination_opening = PedersenOpening::new_rand();
        <span class="kw">let </span>destination_ciphertext = destination_keypair
            .pubkey()
            .encrypt_with(message, <span class="kw-2">&amp;</span>destination_opening);

        <span class="kw">let </span><span class="kw-2">mut </span>prover_transcript = Transcript::new(<span class="string">b"Test"</span>);
        <span class="kw">let </span><span class="kw-2">mut </span>verifier_transcript = Transcript::new(<span class="string">b"Test"</span>);

        <span class="kw">let </span>proof = CiphertextCiphertextEqualityProof::new(
            <span class="kw-2">&amp;</span>source_keypair,
            destination_keypair.pubkey(),
            <span class="kw-2">&amp;</span>source_ciphertext,
            <span class="kw-2">&amp;</span>destination_opening,
            message,
            <span class="kw-2">&amp;mut </span>prover_transcript,
        );

        <span class="macro">assert!</span>(proof
            .verify(
                source_keypair.pubkey(),
                destination_keypair.pubkey(),
                <span class="kw-2">&amp;</span>source_ciphertext,
                <span class="kw-2">&amp;</span>destination_ciphertext,
                <span class="kw-2">&amp;mut </span>verifier_transcript
            )
            .is_ok());

        <span class="comment">// fail case: encrypted and committed messages are different
        </span><span class="kw">let </span>source_message: u64 = <span class="number">55</span>;
        <span class="kw">let </span>destination_message: u64 = <span class="number">77</span>;

        <span class="kw">let </span>source_ciphertext = source_keypair.pubkey().encrypt(source_message);

        <span class="kw">let </span>destination_opening = PedersenOpening::new_rand();
        <span class="kw">let </span>destination_ciphertext = destination_keypair
            .pubkey()
            .encrypt_with(destination_message, <span class="kw-2">&amp;</span>destination_opening);

        <span class="kw">let </span><span class="kw-2">mut </span>prover_transcript = Transcript::new(<span class="string">b"Test"</span>);
        <span class="kw">let </span><span class="kw-2">mut </span>verifier_transcript = Transcript::new(<span class="string">b"Test"</span>);

        <span class="kw">let </span>proof = CiphertextCiphertextEqualityProof::new(
            <span class="kw-2">&amp;</span>source_keypair,
            destination_keypair.pubkey(),
            <span class="kw-2">&amp;</span>source_ciphertext,
            <span class="kw-2">&amp;</span>destination_opening,
            message,
            <span class="kw-2">&amp;mut </span>prover_transcript,
        );

        <span class="macro">assert!</span>(proof
            .verify(
                source_keypair.pubkey(),
                destination_keypair.pubkey(),
                <span class="kw-2">&amp;</span>source_ciphertext,
                <span class="kw-2">&amp;</span>destination_ciphertext,
                <span class="kw-2">&amp;mut </span>verifier_transcript
            )
            .is_err());
    }
}
</code></pre></div></section></main></body></html>