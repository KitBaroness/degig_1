<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ark-poly-0.4.2/src/domain/mod.rs`."><title>mod.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="ark_poly" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../ark_poly/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! This module contains an `EvaluationDomain` abstraction for
//! performing various kinds of polynomial arithmetic on top of
//! fields that are friendly to fast-fourier-transforms (FFTs).
//!
//! A field is FFT-friendly if it contains enough
//! roots of unity to perform the FFT in O(n log n) time.
//! These roots of unity comprise the domain over which
//! polynomial arithmetic is performed.

</span><span class="kw">use </span>ark_ff::FftField;
<span class="kw">use </span>ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
<span class="kw">use </span>ark_std::{fmt, hash, rand::Rng, vec::Vec};

<span class="attr">#[cfg(feature = <span class="string">"parallel"</span>)]
</span><span class="kw">use </span>rayon::prelude::<span class="kw-2">*</span>;

<span class="kw">pub mod </span>general;
<span class="kw">pub mod </span>mixed_radix;
<span class="kw">pub mod </span>radix2;
<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">mod </span>utils;

<span class="kw">pub use </span>general::GeneralEvaluationDomain;
<span class="kw">pub use </span>mixed_radix::MixedRadixEvaluationDomain;
<span class="kw">pub use </span>radix2::Radix2EvaluationDomain;

<span class="doccomment">/// Defines a domain over which finite field (I)FFTs can be performed. The
/// size of the supported FFT depends on the size of the multiplicative
/// subgroup. For efficiency, we recommend that the field has at least one large
/// subgroup generated by a root of unity.
</span><span class="kw">pub trait </span>EvaluationDomain&lt;F: FftField&gt;:
    Copy + Clone + hash::Hash + Eq + PartialEq + fmt::Debug + CanonicalSerialize + CanonicalDeserialize
{
    <span class="doccomment">/// The type of the elements iterator.
    </span><span class="kw">type </span>Elements: Iterator&lt;Item = F&gt; + Sized;

    <span class="doccomment">/// Sample an element that is *not* in the domain.
    </span><span class="kw">fn </span>sample_element_outside_domain&lt;R: Rng&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, rng: <span class="kw-2">&amp;mut </span>R) -&gt; F {
        <span class="kw">let </span><span class="kw-2">mut </span>t = F::rand(rng);
        <span class="kw">while </span><span class="self">self</span>.evaluate_vanishing_polynomial(t).is_zero() {
            t = F::rand(rng);
        }
        t
    }

    <span class="doccomment">/// Construct a domain that is large enough for evaluations of a polynomial
    /// having `num_coeffs` coefficients.
    </span><span class="kw">fn </span>new(num_coeffs: usize) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt;;

    <span class="doccomment">/// Construct a coset domain that is large enough for evaluations of a polynomial
    /// having `num_coeffs` coefficients.
    </span><span class="kw">fn </span>new_coset(num_coeffs: usize, offset: F) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
        <span class="self">Self</span>::new(num_coeffs)<span class="question-mark">?</span>.get_coset(offset)
    }

    <span class="doccomment">/// Construct a coset domain from a subgroup domain
    </span><span class="kw">fn </span>get_coset(<span class="kw-2">&amp;</span><span class="self">self</span>, offset: F) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt;;

    <span class="doccomment">/// Return the size of a domain that is large enough for evaluations of a
    /// polynomial having `num_coeffs` coefficients.
    </span><span class="kw">fn </span>compute_size_of_domain(num_coeffs: usize) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt;;

    <span class="doccomment">/// Return the size of `self`.
    </span><span class="kw">fn </span>size(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize;

    <span class="doccomment">/// Return the size of `self` as a field element.
    </span><span class="kw">fn </span>size_as_field_element(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; F {
        F::from(<span class="self">self</span>.size() <span class="kw">as </span>u64)
    }

    <span class="doccomment">/// Return log_2(size) of `self`.
    </span><span class="kw">fn </span>log_size_of_group(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u64;

    <span class="doccomment">/// Return the inverse of `self.size_as_field_element()`.
    </span><span class="kw">fn </span>size_inv(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; F;

    <span class="doccomment">/// Return the generator for the multiplicative subgroup that defines this domain.
    </span><span class="kw">fn </span>group_gen(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; F;

    <span class="doccomment">/// Return the group inverse of `self.group_gen()`.
    </span><span class="kw">fn </span>group_gen_inv(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; F;

    <span class="doccomment">/// Return the group offset that defines this domain.
    </span><span class="kw">fn </span>coset_offset(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; F;

    <span class="doccomment">/// Return the inverse of `self.offset()`.
    </span><span class="kw">fn </span>coset_offset_inv(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; F;

    <span class="doccomment">/// Return `offset^size`.
    </span><span class="kw">fn </span>coset_offset_pow_size(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; F;

    <span class="doccomment">/// Compute a FFT.
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>fft&lt;T: DomainCoeff&lt;F&gt;&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, coeffs: <span class="kw-2">&amp;</span>[T]) -&gt; Vec&lt;T&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>coeffs = coeffs.to_vec();
        <span class="self">self</span>.fft_in_place(<span class="kw-2">&amp;mut </span>coeffs);
        coeffs
    }

    <span class="doccomment">/// Compute a FFT, modifying the vector in place.
    </span><span class="kw">fn </span>fft_in_place&lt;T: DomainCoeff&lt;F&gt;&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, coeffs: <span class="kw-2">&amp;mut </span>Vec&lt;T&gt;);

    <span class="doccomment">/// Compute a IFFT.
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>ifft&lt;T: DomainCoeff&lt;F&gt;&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, evals: <span class="kw-2">&amp;</span>[T]) -&gt; Vec&lt;T&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>evals = evals.to_vec();
        <span class="self">self</span>.ifft_in_place(<span class="kw-2">&amp;mut </span>evals);
        evals
    }

    <span class="doccomment">/// Compute a IFFT, modifying the vector in place.
    </span><span class="kw">fn </span>ifft_in_place&lt;T: DomainCoeff&lt;F&gt;&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, evals: <span class="kw-2">&amp;mut </span>Vec&lt;T&gt;);

    <span class="doccomment">/// Multiply the `i`-th element of `coeffs` with `g^i`.
    </span><span class="kw">fn </span>distribute_powers&lt;T: DomainCoeff&lt;F&gt;&gt;(coeffs: <span class="kw-2">&amp;mut </span>[T], g: F) {
        <span class="self">Self</span>::distribute_powers_and_mul_by_const(coeffs, g, F::one());
    }

    <span class="doccomment">/// Multiply the `i`-th element of `coeffs` with `c*g^i`.
    </span><span class="attr">#[cfg(not(feature = <span class="string">"parallel"</span>))]
    </span><span class="kw">fn </span>distribute_powers_and_mul_by_const&lt;T: DomainCoeff&lt;F&gt;&gt;(coeffs: <span class="kw-2">&amp;mut </span>[T], g: F, c: F) {
        <span class="comment">// invariant: pow = c*g^i at the ith iteration of the loop
        </span><span class="kw">let </span><span class="kw-2">mut </span>pow = c;
        coeffs.iter_mut().for_each(|coeff| {
            <span class="kw-2">*</span>coeff <span class="kw-2">*</span>= pow;
            pow <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>g
        })
    }

    <span class="doccomment">/// Multiply the `i`-th element of `coeffs` with `c*g^i`.
    </span><span class="attr">#[cfg(feature = <span class="string">"parallel"</span>)]
    </span><span class="kw">fn </span>distribute_powers_and_mul_by_const&lt;T: DomainCoeff&lt;F&gt;&gt;(coeffs: <span class="kw-2">&amp;mut </span>[T], g: F, c: F) {
        <span class="kw">use </span>ark_std::cmp::max;
        <span class="kw">let </span>min_parallel_chunk_size = <span class="number">1024</span>;
        <span class="kw">let </span>num_cpus_available = rayon::current_num_threads();
        <span class="kw">let </span>num_elem_per_thread = max(coeffs.len() / num_cpus_available, min_parallel_chunk_size);

        <span class="macro">ark_std::cfg_chunks_mut!</span>(coeffs, num_elem_per_thread)
            .enumerate()
            .for_each(|(i, chunk)| {
                <span class="kw">let </span>offset = c * g.pow([(i * num_elem_per_thread) <span class="kw">as </span>u64]);
                <span class="kw">let </span><span class="kw-2">mut </span>pow = offset;
                chunk.iter_mut().for_each(|coeff| {
                    <span class="kw-2">*</span>coeff <span class="kw-2">*</span>= pow;
                    pow <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>g
                })
            });
    }

    <span class="doccomment">/// Evaluate all the lagrange polynomials defined by this domain at the
    /// point `tau`. This is computed in time O(|domain|).
    /// Then given the evaluations of a degree d polynomial P over this domain,
    /// where d &lt; |domain|, `P(tau)` can be computed as
    /// `P(tau) = sum_{i in [|Domain|]} L_{i, Domain}(tau) * P(g^i)`.
    /// `L_{i, Domain}` is the value of the i-th lagrange coefficient
    /// in the returned vector.
    </span><span class="kw">fn </span>evaluate_all_lagrange_coefficients(<span class="kw-2">&amp;</span><span class="self">self</span>, tau: F) -&gt; Vec&lt;F&gt; {
        <span class="comment">// Evaluate all Lagrange polynomials at tau to get the lagrange coefficients.
        // Define the following as
        // - H: The coset we are in, with generator g and offset h
        // - m: The size of the coset H
        // - Z_H: The vanishing polynomial for H. Z_H(x) = prod_{i in m} (x - hg^i) = x^m - h^m
        // - v_i: A sequence of values, where v_0 = 1/(m * h^(m-1)), and v_{i + 1} = g * v_i
        //
        // We then compute L_{i,H}(tau) as `L_{i,H}(tau) = Z_H(tau) * v_i / (tau - h * g^i)`
        //
        // However, if tau in H, both the numerator and denominator equal 0
        // when i corresponds to the value tau equals, and the coefficient is 0
        // everywhere else. We handle this case separately, and we can easily
        // detect by checking if the vanishing poly is 0.
        </span><span class="kw">let </span>size = <span class="self">self</span>.size();
        <span class="kw">let </span>z_h_at_tau = <span class="self">self</span>.evaluate_vanishing_polynomial(tau);
        <span class="kw">let </span>offset = <span class="self">self</span>.coset_offset();
        <span class="kw">let </span>group_gen = <span class="self">self</span>.group_gen();
        <span class="kw">if </span>z_h_at_tau.is_zero() {
            <span class="comment">// In this case, we know that tau = hg^i, for some value i.
            // Then i-th lagrange coefficient in this case is then simply 1,
            // and all other lagrange coefficients are 0.
            // Thus we find i by brute force.
            </span><span class="kw">let </span><span class="kw-2">mut </span>u = <span class="macro">vec!</span>[F::zero(); size];
            <span class="kw">let </span><span class="kw-2">mut </span>omega_i = offset;
            <span class="kw">for </span>u_i <span class="kw">in </span>u.iter_mut().take(size) {
                <span class="kw">if </span>omega_i == tau {
                    <span class="kw-2">*</span>u_i = F::one();
                    <span class="kw">break</span>;
                }
                omega_i <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>group_gen;
            }
            u
        } <span class="kw">else </span>{
            <span class="comment">// In this case we have to compute `Z_H(tau) * v_i / (tau - h g^i)`
            // for i in 0..size
            // We actually compute this by computing (Z_H(tau) * v_i)^{-1} * (tau - h g^i)
            // and then batch inverting to get the correct lagrange coefficients.
            // We let `l_i = (Z_H(tau) * v_i)^-1` and `r_i = tau - h g^i`
            // Notice that since Z_H(tau) is i-independent,
            // and v_i = g * v_{i-1}, it follows that
            // l_i = g^-1 * l_{i-1}
            // TODO: consider caching the computation of l_i to save N multiplications
            </span><span class="kw">use </span>ark_ff::fields::batch_inversion;

            <span class="kw">let </span>group_gen_inv = <span class="self">self</span>.group_gen_inv();

            <span class="comment">// v_0_inv = m * h^(m-1)
            </span><span class="kw">let </span>v_0_inv = <span class="self">self</span>.size_as_field_element() * offset.pow([size <span class="kw">as </span>u64 - <span class="number">1</span>]);
            <span class="kw">let </span><span class="kw-2">mut </span>l_i = z_h_at_tau.inverse().unwrap() * v_0_inv;
            <span class="kw">let </span><span class="kw-2">mut </span>negative_cur_elem = -offset;
            <span class="kw">let </span><span class="kw-2">mut </span>lagrange_coefficients_inverse = <span class="macro">vec!</span>[F::zero(); size];
            <span class="kw">for </span>coeff <span class="kw">in </span><span class="kw-2">&amp;mut </span>lagrange_coefficients_inverse {
                <span class="kw">let </span>r_i = tau + negative_cur_elem;
                <span class="kw-2">*</span>coeff = l_i * r_i;
                <span class="comment">// Increment l_i and negative_cur_elem
                </span>l_i <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>group_gen_inv;
                negative_cur_elem <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>group_gen;
            }

            <span class="comment">// Invert the lagrange coefficients inverse, to get the actual coefficients,
            // and return these
            </span>batch_inversion(lagrange_coefficients_inverse.as_mut_slice());
            lagrange_coefficients_inverse
        }
    }

    <span class="doccomment">/// Return the sparse vanishing polynomial.
    </span><span class="kw">fn </span>vanishing_polynomial(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw">crate</span>::univariate::SparsePolynomial&lt;F&gt; {
        <span class="kw">let </span>constant_coeff = <span class="self">self</span>.coset_offset_pow_size();
        <span class="kw">let </span>coeffs = <span class="macro">vec!</span>[(<span class="number">0</span>, -constant_coeff), (<span class="self">self</span>.size(), F::one())];
        <span class="kw">crate</span>::univariate::SparsePolynomial::from_coefficients_vec(coeffs)
    }

    <span class="doccomment">/// This evaluates the vanishing polynomial for this domain at tau.
    </span><span class="kw">fn </span>evaluate_vanishing_polynomial(<span class="kw-2">&amp;</span><span class="self">self</span>, tau: F) -&gt; F {
        <span class="comment">// TODO: Consider precomputed exponentiation tables if we need this to be
        // faster.
        </span>tau.pow([<span class="self">self</span>.size() <span class="kw">as </span>u64]) - <span class="self">self</span>.coset_offset_pow_size()
    }

    <span class="doccomment">/// Returns the `i`-th element of the domain.
    </span><span class="kw">fn </span>element(<span class="kw-2">&amp;</span><span class="self">self</span>, i: usize) -&gt; F {
        <span class="kw">let </span><span class="kw-2">mut </span>result = <span class="self">self</span>.group_gen().pow([i <span class="kw">as </span>u64]);
        <span class="kw">if </span>!<span class="self">self</span>.coset_offset().is_one() {
            result <span class="kw-2">*</span>= <span class="self">self</span>.coset_offset()
        }
        result
    }

    <span class="doccomment">/// Return an iterator over the elements of the domain.
    </span><span class="kw">fn </span>elements(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self</span>::Elements;

    <span class="doccomment">/// Given an index which assumes the first elements of this domain are the
    /// elements of another (sub)domain,
    /// this returns the actual index into this domain.
    </span><span class="kw">fn </span>reindex_by_subdomain(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="self">Self</span>, index: usize) -&gt; usize {
        <span class="macro">assert!</span>(<span class="self">self</span>.size() &gt;= other.size());
        <span class="comment">// Let this subgroup be G, and the subgroup we're re-indexing by be S.
        // Since its a subgroup, the 0th element of S is at index 0 in G, the first
        // element of S is at index |G|/|S|, the second at 2*|G|/|S|, etc.
        // Thus for an index i that corresponds to S, the index in G is i*|G|/|S|
        </span><span class="kw">let </span>period = <span class="self">self</span>.size() / other.size();
        <span class="kw">if </span>index &lt; other.size() {
            index * period
        } <span class="kw">else </span>{
            <span class="comment">// Let i now be the index of this element in G \ S
            // Let x be the number of elements in G \ S, for every element in S. Then x =
            // (|G|/|S| - 1). At index i in G \ S, the number of elements in S
            // that appear before the index in G to which i corresponds to, is
            // floor(i / x) + 1. The +1 is because index 0 of G is S_0, so the
            // position is offset by at least one. The floor(i / x) term is
            // because after x elements in G \ S, there is one more element from S
            // that will have appeared in G.
            </span><span class="kw">let </span>i = index - other.size();
            <span class="kw">let </span>x = period - <span class="number">1</span>;
            i + (i / x) + <span class="number">1
        </span>}
    }

    <span class="doccomment">/// Perform O(n) multiplication of two polynomials that are presented by
    /// their evaluations in the domain.
    /// Returns the evaluations of the product over the domain.
    ///
    /// Assumes that the domain is large enough to allow for successful
    /// interpolation after multiplication.
    </span><span class="attr">#[must_use]
    </span><span class="kw">fn </span>mul_polynomials_in_evaluation_domain(<span class="kw-2">&amp;</span><span class="self">self</span>, self_evals: <span class="kw-2">&amp;</span>[F], other_evals: <span class="kw-2">&amp;</span>[F]) -&gt; Vec&lt;F&gt; {
        <span class="macro">assert_eq!</span>(self_evals.len(), other_evals.len());
        <span class="kw">let </span><span class="kw-2">mut </span>result = self_evals.to_vec();

        <span class="macro">ark_std::cfg_iter_mut!</span>(result)
            .zip(other_evals)
            .for_each(|(a, b)| <span class="kw-2">*</span>a <span class="kw-2">*</span>= b);

        result
    }
}

<span class="doccomment">/// Types that can be FFT-ed must implement this trait.
</span><span class="kw">pub trait </span>DomainCoeff&lt;F: FftField&gt;:
    Copy
    + Send
    + Sync
    + core::ops::Add&lt;Output = <span class="self">Self</span>&gt;
    + core::ops::Sub&lt;Output = <span class="self">Self</span>&gt;
    + core::ops::AddAssign
    + core::ops::SubAssign
    + ark_ff::Zero
    + core::ops::MulAssign&lt;F&gt;
    + core::fmt::Debug
    + PartialEq
{
}

<span class="kw">impl</span>&lt;T, F&gt; DomainCoeff&lt;F&gt; <span class="kw">for </span>T
<span class="kw">where
    </span>F: FftField,
    T: Copy
        + Send
        + Sync
        + core::ops::Add&lt;Output = <span class="self">Self</span>&gt;
        + core::ops::Sub&lt;Output = <span class="self">Self</span>&gt;
        + core::ops::AddAssign
        + core::ops::SubAssign
        + ark_ff::Zero
        + core::ops::MulAssign&lt;F&gt;
        + core::fmt::Debug
        + PartialEq,
{
}
</code></pre></div></section></main></body></html>