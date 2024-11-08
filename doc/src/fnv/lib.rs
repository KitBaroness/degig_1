<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fnv-1.0.7/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="fnv" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../fnv/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#342" id="342">342</a>
<a href="#343" id="343">343</a>
<a href="#344" id="344">344</a>
<a href="#345" id="345">345</a>
<a href="#346" id="346">346</a>
<a href="#347" id="347">347</a>
<a href="#348" id="348">348</a>
<a href="#349" id="349">349</a>
<a href="#350" id="350">350</a>
<a href="#351" id="351">351</a>
<a href="#352" id="352">352</a>
<a href="#353" id="353">353</a>
<a href="#354" id="354">354</a>
<a href="#355" id="355">355</a>
<a href="#356" id="356">356</a>
<a href="#357" id="357">357</a>
<a href="#358" id="358">358</a>
<a href="#359" id="359">359</a>
<a href="#360" id="360">360</a>
<a href="#361" id="361">361</a>
<a href="#362" id="362">362</a>
<a href="#363" id="363">363</a>
<a href="#364" id="364">364</a>
<a href="#365" id="365">365</a>
<a href="#366" id="366">366</a>
<a href="#367" id="367">367</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! An implementation of the [Fowler–Noll–Vo hash function][chongo].
//!
//! ## About
//!
//! The FNV hash function is a custom `Hasher` implementation that is more
//! efficient for smaller hash keys.
//!
//! [The Rust FAQ states that][faq] while the default `Hasher` implementation,
//! SipHash, is good in many cases, it is notably slower than other algorithms
//! with short keys, such as when you have a map of integers to other values.
//! In cases like these, [FNV is demonstrably faster][graphs].
//!
//! Its disadvantages are that it performs badly on larger inputs, and
//! provides no protection against collision attacks, where a malicious user
//! can craft specific keys designed to slow a hasher down. Thus, it is
//! important to profile your program to ensure that you are using small hash
//! keys, and be certain that your program could not be exposed to malicious
//! inputs (including being a networked server).
//!
//! The Rust compiler itself uses FNV, as it is not worried about
//! denial-of-service attacks, and can assume that its inputs are going to be
//! small—a perfect use case for FNV.
//!
</span><span class="attr">#![cfg_attr(feature = <span class="string">"std"</span>, doc = <span class="string">r#"

## Using FNV in a `HashMap`

The `FnvHashMap` type alias is the easiest way to use the standard library’s
`HashMap` with FNV.

```rust
use fnv::FnvHashMap;

let mut map = FnvHashMap::default();
map.insert(1, "one");
map.insert(2, "two");

map = FnvHashMap::with_capacity_and_hasher(10, Default::default());
map.insert(1, "one");
map.insert(2, "two");
```

Note, the standard library’s `HashMap::new` and `HashMap::with_capacity`
are only implemented for the `RandomState` hasher, so using `Default` to
get the hasher is the next best option.

## Using FNV in a `HashSet`

Similarly, `FnvHashSet` is a type alias for the standard library’s `HashSet`
with FNV.

```rust
use fnv::FnvHashSet;

let mut set = FnvHashSet::default();
set.insert(1);
set.insert(2);

set = FnvHashSet::with_capacity_and_hasher(10, Default::default());
set.insert(1);
set.insert(2);
```
"#</span>)]
</span><span class="doccomment">//!
//! [chongo]: http://www.isthe.com/chongo/tech/comp/fnv/index.html
//! [faq]: https://www.rust-lang.org/en-US/faq.html#why-are-rusts-hashmaps-slow
//! [graphs]: https://cglab.ca/~abeinges/blah/hash-rs/

</span><span class="attr">#![cfg_attr(not(feature = <span class="string">"std"</span>), no_std)]

#[cfg(all(not(feature = <span class="string">"std"</span>), test))]
</span><span class="kw">extern crate </span>alloc;

<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">use </span>std::default::Default;
<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">use </span>std::hash::{Hasher, BuildHasherDefault};
<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">use </span>std::collections::{HashMap, HashSet};
<span class="attr">#[cfg(not(feature = <span class="string">"std"</span>))]
</span><span class="kw">use </span>core::default::Default;
<span class="attr">#[cfg(not(feature = <span class="string">"std"</span>))]
</span><span class="kw">use </span>core::hash::{Hasher, BuildHasherDefault};

<span class="doccomment">/// An implementation of the Fowler–Noll–Vo hash function.
///
/// See the [crate documentation](index.html) for more details.
</span><span class="attr">#[allow(missing_copy_implementations)]
</span><span class="kw">pub struct </span>FnvHasher(u64);

<span class="kw">impl </span>Default <span class="kw">for </span>FnvHasher {

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>default() -&gt; FnvHasher {
        FnvHasher(<span class="number">0xcbf29ce484222325</span>)
    }
}

<span class="kw">impl </span>FnvHasher {
    <span class="doccomment">/// Create an FNV hasher starting with a state corresponding
    /// to the hash `key`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>with_key(key: u64) -&gt; FnvHasher {
        FnvHasher(key)
    }
}

<span class="kw">impl </span>Hasher <span class="kw">for </span>FnvHasher {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>finish(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u64 {
        <span class="self">self</span>.<span class="number">0
    </span>}

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>write(<span class="kw-2">&amp;mut </span><span class="self">self</span>, bytes: <span class="kw-2">&amp;</span>[u8]) {
        <span class="kw">let </span>FnvHasher(<span class="kw-2">mut </span>hash) = <span class="kw-2">*</span><span class="self">self</span>;

        <span class="kw">for </span>byte <span class="kw">in </span>bytes.iter() {
            hash = hash ^ (<span class="kw-2">*</span>byte <span class="kw">as </span>u64);
            hash = hash.wrapping_mul(<span class="number">0x100000001b3</span>);
        }

        <span class="kw-2">*</span><span class="self">self </span>= FnvHasher(hash);
    }
}

<span class="doccomment">/// A builder for default FNV hashers.
</span><span class="kw">pub type </span>FnvBuildHasher = BuildHasherDefault&lt;FnvHasher&gt;;

<span class="doccomment">/// A `HashMap` using a default FNV hasher.
</span><span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">pub type </span>FnvHashMap&lt;K, V&gt; = HashMap&lt;K, V, FnvBuildHasher&gt;;

<span class="doccomment">/// A `HashSet` using a default FNV hasher.
</span><span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">pub type </span>FnvHashSet&lt;T&gt; = HashSet&lt;T, FnvBuildHasher&gt;;


<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>test {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;
    <span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
    </span><span class="kw">use </span>std::hash::Hasher;
    <span class="attr">#[cfg(not(feature = <span class="string">"std"</span>))]
    </span><span class="kw">use </span>alloc::vec::Vec;

    <span class="kw">fn </span>fnv1a(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; u64 {
        <span class="kw">let </span><span class="kw-2">mut </span>hasher = FnvHasher::default();
        hasher.write(bytes);
        hasher.finish()
    }

    <span class="kw">fn </span>repeat_10(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; Vec&lt;u8&gt; {
        (<span class="number">0</span>..<span class="number">10</span>).flat_map(|<span class="kw">_</span>| bytes.iter().cloned()).collect()
    }

    <span class="kw">fn </span>repeat_500(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; Vec&lt;u8&gt; {
        (<span class="number">0</span>..<span class="number">500</span>).flat_map(|<span class="kw">_</span>| bytes.iter().cloned()).collect()
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>basic_tests() {
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b""</span>), <span class="number">0xcbf29ce484222325</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"a"</span>), <span class="number">0xaf63dc4c8601ec8c</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"b"</span>), <span class="number">0xaf63df4c8601f1a5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"c"</span>), <span class="number">0xaf63de4c8601eff2</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"d"</span>), <span class="number">0xaf63d94c8601e773</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"e"</span>), <span class="number">0xaf63d84c8601e5c0</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"f"</span>), <span class="number">0xaf63db4c8601ead9</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"fo"</span>), <span class="number">0x08985907b541d342</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"foo"</span>), <span class="number">0xdcb27518fed9d577</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"foob"</span>), <span class="number">0xdd120e790c2512af</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"fooba"</span>), <span class="number">0xcac165afa2fef40a</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"foobar"</span>), <span class="number">0x85944171f73967e8</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\0"</span>), <span class="number">0xaf63bd4c8601b7df</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"a\0"</span>), <span class="number">0x089be207b544f1e4</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"b\0"</span>), <span class="number">0x08a61407b54d9b5f</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"c\0"</span>), <span class="number">0x08a2ae07b54ab836</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"d\0"</span>), <span class="number">0x0891b007b53c4869</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"e\0"</span>), <span class="number">0x088e4a07b5396540</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"f\0"</span>), <span class="number">0x08987c07b5420ebb</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"fo\0"</span>), <span class="number">0xdcb28a18fed9f926</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"foo\0"</span>), <span class="number">0xdd1270790c25b935</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"foob\0"</span>), <span class="number">0xcac146afa2febf5d</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"fooba\0"</span>), <span class="number">0x8593d371f738acfe</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"foobar\0"</span>), <span class="number">0x34531ca7168b8f38</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"ch"</span>), <span class="number">0x08a25607b54a22ae</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"cho"</span>), <span class="number">0xf5faf0190cf90df3</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chon"</span>), <span class="number">0xf27397910b3221c7</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chong"</span>), <span class="number">0x2c8c2b76062f22e0</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo"</span>), <span class="number">0xe150688c8217b8fd</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo "</span>), <span class="number">0xf35a83c10e4f1f87</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo w"</span>), <span class="number">0xd1edd10b507344d0</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo wa"</span>), <span class="number">0x2a5ee739b3ddb8c3</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was"</span>), <span class="number">0xdcfb970ca1c0d310</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was "</span>), <span class="number">0x4054da76daa6da90</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was h"</span>), <span class="number">0xf70a2ff589861368</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was he"</span>), <span class="number">0x4c628b38aed25f17</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was her"</span>), <span class="number">0x9dd1f6510f78189f</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was here"</span>), <span class="number">0xa3de85bd491270ce</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was here!"</span>), <span class="number">0x858e2fa32a55e61d</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was here!\n"</span>), <span class="number">0x46810940eff5f915</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"ch\0"</span>), <span class="number">0xf5fadd190cf8edaa</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"cho\0"</span>), <span class="number">0xf273ed910b32b3e9</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chon\0"</span>), <span class="number">0x2c8c5276062f6525</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chong\0"</span>), <span class="number">0xe150b98c821842a0</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo\0"</span>), <span class="number">0xf35aa3c10e4f55e7</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo \0"</span>), <span class="number">0xd1ed680b50729265</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo w\0"</span>), <span class="number">0x2a5f0639b3dded70</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo wa\0"</span>), <span class="number">0xdcfbaa0ca1c0f359</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was\0"</span>), <span class="number">0x4054ba76daa6a430</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was \0"</span>), <span class="number">0xf709c7f5898562b0</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was h\0"</span>), <span class="number">0x4c62e638aed2f9b8</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was he\0"</span>), <span class="number">0x9dd1a8510f779415</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was her\0"</span>), <span class="number">0xa3de2abd4911d62d</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was here\0"</span>), <span class="number">0x858e0ea32a55ae0a</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was here!\0"</span>), <span class="number">0x46810f40eff60347</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo was here!\n\0"</span>), <span class="number">0xc33bce57bef63eaf</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"cu"</span>), <span class="number">0x08a24307b54a0265</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"cur"</span>), <span class="number">0xf5b9fd190cc18d15</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curd"</span>), <span class="number">0x4c968290ace35703</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds"</span>), <span class="number">0x07174bd5c64d9350</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds "</span>), <span class="number">0x5a294c3ff5d18750</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds a"</span>), <span class="number">0x05b3c1aeb308b843</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds an"</span>), <span class="number">0xb92a48da37d0f477</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and"</span>), <span class="number">0x73cdddccd80ebc49</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and "</span>), <span class="number">0xd58c4c13210a266b</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and w"</span>), <span class="number">0xe78b6081243ec194</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and wh"</span>), <span class="number">0xb096f77096a39f34</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and whe"</span>), <span class="number">0xb425c54ff807b6a3</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and whey"</span>), <span class="number">0x23e520e2751bb46e</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and whey\n"</span>), <span class="number">0x1a0b44ccfe1385ec</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"cu\0"</span>), <span class="number">0xf5ba4b190cc2119f</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"cur\0"</span>), <span class="number">0x4c962690ace2baaf</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curd\0"</span>), <span class="number">0x0716ded5c64cda19</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds\0"</span>), <span class="number">0x5a292c3ff5d150f0</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds \0"</span>), <span class="number">0x05b3e0aeb308ecf0</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds a\0"</span>), <span class="number">0xb92a5eda37d119d9</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds an\0"</span>), <span class="number">0x73ce41ccd80f6635</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and\0"</span>), <span class="number">0xd58c2c132109f00b</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and \0"</span>), <span class="number">0xe78baf81243f47d1</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and w\0"</span>), <span class="number">0xb0968f7096a2ee7c</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and wh\0"</span>), <span class="number">0xb425a84ff807855c</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and whe\0"</span>), <span class="number">0x23e4e9e2751b56f9</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and whey\0"</span>), <span class="number">0x1a0b4eccfe1396ea</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"curds and whey\n\0"</span>), <span class="number">0x54abd453bb2c9004</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"hi"</span>), <span class="number">0x08ba5f07b55ec3da</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"hi\0"</span>), <span class="number">0x337354193006cb6e</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"hello"</span>), <span class="number">0xa430d84680aabd0b</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"hello\0"</span>), <span class="number">0xa9bc8acca21f39b1</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\xff\x00\x00\x01"</span>), <span class="number">0x6961196491cc682d</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\x01\x00\x00\xff"</span>), <span class="number">0xad2bb1774799dfe9</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\xff\x00\x00\x02"</span>), <span class="number">0x6961166491cc6314</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\x02\x00\x00\xff"</span>), <span class="number">0x8d1bb3904a3b1236</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\xff\x00\x00\x03"</span>), <span class="number">0x6961176491cc64c7</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\x03\x00\x00\xff"</span>), <span class="number">0xed205d87f40434c7</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\xff\x00\x00\x04"</span>), <span class="number">0x6961146491cc5fae</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\x04\x00\x00\xff"</span>), <span class="number">0xcd3baf5e44f8ad9c</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\x40\x51\x4e\x44"</span>), <span class="number">0xe3b36596127cd6d8</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\x44\x4e\x51\x40"</span>), <span class="number">0xf77f1072c8e8a646</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\x40\x51\x4e\x4a"</span>), <span class="number">0xe3b36396127cd372</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\x4a\x4e\x51\x40"</span>), <span class="number">0x6067dce9932ad458</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\x40\x51\x4e\x54"</span>), <span class="number">0xe3b37596127cf208</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"\x54\x4e\x51\x40"</span>), <span class="number">0x4b7b10fa9fe83936</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"127.0.0.1"</span>), <span class="number">0xaabafe7104d914be</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"127.0.0.1\0"</span>), <span class="number">0xf4d3180b3cde3eda</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"127.0.0.2"</span>), <span class="number">0xaabafd7104d9130b</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"127.0.0.2\0"</span>), <span class="number">0xf4cfb20b3cdb5bb1</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"127.0.0.3"</span>), <span class="number">0xaabafc7104d91158</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"127.0.0.3\0"</span>), <span class="number">0xf4cc4c0b3cd87888</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"64.81.78.68"</span>), <span class="number">0xe729bac5d2a8d3a7</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"64.81.78.68\0"</span>), <span class="number">0x74bc0524f4dfa4c5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"64.81.78.74"</span>), <span class="number">0xe72630c5d2a5b352</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"64.81.78.74\0"</span>), <span class="number">0x6b983224ef8fb456</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"64.81.78.84"</span>), <span class="number">0xe73042c5d2ae266d</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"64.81.78.84\0"</span>), <span class="number">0x8527e324fdeb4b37</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"feedface"</span>), <span class="number">0x0a83c86fee952abc</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"feedface\0"</span>), <span class="number">0x7318523267779d74</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"feedfacedaffdeed"</span>), <span class="number">0x3e66d3d56b8caca1</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"feedfacedaffdeed\0"</span>), <span class="number">0x956694a5c0095593</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"feedfacedeadbeef"</span>), <span class="number">0xcac54572bb1a6fc8</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"feedfacedeadbeef\0"</span>), <span class="number">0xa7a4c9f3edebf0d8</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"line 1\nline 2\nline 3"</span>), <span class="number">0x7829851fac17b143</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo &lt;Landon Curt Noll&gt; /\\../\\"</span>), <span class="number">0x2c8f4c9af81bcf06</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo &lt;Landon Curt Noll&gt; /\\../\\\0"</span>), <span class="number">0xd34e31539740c732</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo (Landon Curt Noll) /\\../\\"</span>), <span class="number">0x3605a2ac253d2db1</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"chongo (Landon Curt Noll) /\\../\\\0"</span>), <span class="number">0x08c11b8346f4a3c3</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://antwrp.gsfc.nasa.gov/apod/astropix.html"</span>), <span class="number">0x6be396289ce8a6da</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://en.wikipedia.org/wiki/Fowler_Noll_Vo_hash"</span>), <span class="number">0xd9b957fb7fe794c5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://epod.usra.edu/"</span>), <span class="number">0x05be33da04560a93</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://exoplanet.eu/"</span>), <span class="number">0x0957f1577ba9747c</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://hvo.wr.usgs.gov/cam3/"</span>), <span class="number">0xda2cc3acc24fba57</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://hvo.wr.usgs.gov/cams/HMcam/"</span>), <span class="number">0x74136f185b29e7f0</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://hvo.wr.usgs.gov/kilauea/update/deformation.html"</span>), <span class="number">0xb2f2b4590edb93b2</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://hvo.wr.usgs.gov/kilauea/update/images.html"</span>), <span class="number">0xb3608fce8b86ae04</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://hvo.wr.usgs.gov/kilauea/update/maps.html"</span>), <span class="number">0x4a3a865079359063</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://hvo.wr.usgs.gov/volcanowatch/current_issue.html"</span>), <span class="number">0x5b3a7ef496880a50</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://neo.jpl.nasa.gov/risk/"</span>), <span class="number">0x48fae3163854c23b</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://norvig.com/21-days.html"</span>), <span class="number">0x07aaa640476e0b9a</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://primes.utm.edu/curios/home.php"</span>), <span class="number">0x2f653656383a687d</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://slashdot.org/"</span>), <span class="number">0xa1031f8e7599d79c</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://tux.wr.usgs.gov/Maps/155.25-19.5.html"</span>), <span class="number">0xa31908178ff92477</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://volcano.wr.usgs.gov/kilaueastatus.php"</span>), <span class="number">0x097edf3c14c3fb83</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.avo.alaska.edu/activity/Redoubt.php"</span>), <span class="number">0xb51ca83feaa0971b</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.dilbert.com/fast/"</span>), <span class="number">0xdd3c0d96d784f2e9</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.fourmilab.ch/gravitation/orbits/"</span>), <span class="number">0x86cd26a9ea767d78</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.fpoa.net/"</span>), <span class="number">0xe6b215ff54a30c18</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.ioccc.org/index.html"</span>), <span class="number">0xec5b06a1c5531093</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/cgi-bin/number.cgi"</span>), <span class="number">0x45665a929f9ec5e5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/bio.html"</span>), <span class="number">0x8c7609b4a9f10907</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/index.html"</span>), <span class="number">0x89aac3a491f0d729</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/src/calc/lucas-calc"</span>), <span class="number">0x32ce6b26e0f4a403</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/tech/astro/venus2004.html"</span>), <span class="number">0x614ab44e02b53e01</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/tech/astro/vita.html"</span>), <span class="number">0xfa6472eb6eef3290</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/tech/comp/c/expert.html"</span>), <span class="number">0x9e5d75eb1948eb6a</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/tech/comp/calc/index.html"</span>), <span class="number">0xb6d12ad4a8671852</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/tech/comp/fnv/index.html"</span>), <span class="number">0x88826f56eba07af1</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/tech/math/number/howhigh.html"</span>), <span class="number">0x44535bf2645bc0fd</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/tech/math/number/number.html"</span>), <span class="number">0x169388ffc21e3728</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/tech/math/prime/mersenne.html"</span>), <span class="number">0xf68aac9e396d8224</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.isthe.com/chongo/tech/math/prime/mersenne.html#largest"</span>), <span class="number">0x8e87d7e7472b3883</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.lavarnd.org/cgi-bin/corpspeak.cgi"</span>), <span class="number">0x295c26caa8b423de</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.lavarnd.org/cgi-bin/haiku.cgi"</span>), <span class="number">0x322c814292e72176</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.lavarnd.org/cgi-bin/rand-none.cgi"</span>), <span class="number">0x8a06550eb8af7268</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.lavarnd.org/cgi-bin/randdist.cgi"</span>), <span class="number">0xef86d60e661bcf71</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.lavarnd.org/index.html"</span>), <span class="number">0x9e5426c87f30ee54</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.lavarnd.org/what/nist-test.html"</span>), <span class="number">0xf1ea8aa826fd047e</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.macosxhints.com/"</span>), <span class="number">0x0babaf9a642cb769</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.mellis.com/"</span>), <span class="number">0x4b3341d4068d012e</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.nature.nps.gov/air/webcams/parks/havoso2alert/havoalert.cfm"</span>), <span class="number">0xd15605cbc30a335c</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.nature.nps.gov/air/webcams/parks/havoso2alert/timelines_24.cfm"</span>), <span class="number">0x5b21060aed8412e5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.paulnoll.com/"</span>), <span class="number">0x45e2cda1ce6f4227</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.pepysdiary.com/"</span>), <span class="number">0x50ae3745033ad7d4</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.sciencenews.org/index/home/activity/view"</span>), <span class="number">0xaa4588ced46bf414</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.skyandtelescope.com/"</span>), <span class="number">0xc1b0056c4a95467e</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.sput.nl/~rob/sirius.html"</span>), <span class="number">0x56576a71de8b4089</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.systemexperts.com/"</span>), <span class="number">0xbf20965fa6dc927e</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.tq-international.com/phpBB3/index.php"</span>), <span class="number">0x569f8383c2040882</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.travelquesttours.com/index.htm"</span>), <span class="number">0xe1e772fba08feca0</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="string">b"http://www.wunderground.com/global/stations/89606.html"</span>), <span class="number">0x4ced94af97138ac4</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"21701"</span>)), <span class="number">0xc4112ffb337a82fb</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"M21701"</span>)), <span class="number">0xd64a4fd41de38b7d</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"2^21701-1"</span>)), <span class="number">0x4cfc32329edebcbb</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"\x54\xc5"</span>)), <span class="number">0x0803564445050395</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"\xc5\x54"</span>)), <span class="number">0xaa1574ecf4642ffd</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"23209"</span>)), <span class="number">0x694bc4e54cc315f9</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"M23209"</span>)), <span class="number">0xa3d7cb273b011721</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"2^23209-1"</span>)), <span class="number">0x577c2f8b6115bfa5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"\x5a\xa9"</span>)), <span class="number">0xb7ec8c1a769fb4c1</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"\xa9\x5a"</span>)), <span class="number">0x5d5cfce63359ab19</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"391581216093"</span>)), <span class="number">0x33b96c3cd65b5f71</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"391581*2^216093-1"</span>)), <span class="number">0xd845097780602bb9</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"\x05\xf9\x9d\x03\x4c\x81"</span>)), <span class="number">0x84d47645d02da3d5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"FEDCBA9876543210"</span>)), <span class="number">0x83544f33b58773a5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"\xfe\xdc\xba\x98\x76\x54\x32\x10"</span>)), <span class="number">0x9175cbb2160836c5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"EFCDAB8967452301"</span>)), <span class="number">0xc71b3bc175e72bc5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"\xef\xcd\xab\x89\x67\x45\x23\x01"</span>)), <span class="number">0x636806ac222ec985</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"0123456789ABCDEF"</span>)), <span class="number">0xb6ef0e6950f52ed5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"\x01\x23\x45\x67\x89\xab\xcd\xef"</span>)), <span class="number">0xead3d8a0f3dfdaa5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"1032547698BADCFE"</span>)), <span class="number">0x922908fe9a861ba5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_10(<span class="string">b"\x10\x32\x54\x76\x98\xba\xdc\xfe"</span>)), <span class="number">0x6d4821de275fd5c5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_500(<span class="string">b"\x00"</span>)), <span class="number">0x1fe3fce62bd816b5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_500(<span class="string">b"\x07"</span>)), <span class="number">0xc23e9fccd6f70591</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_500(<span class="string">b"~"</span>)), <span class="number">0xc1af12bdfe16b5b5</span>);
        <span class="macro">assert_eq!</span>(fnv1a(<span class="kw-2">&amp;</span>repeat_500(<span class="string">b"\x7f"</span>)), <span class="number">0x39e9f18f2f85e221</span>);
    }
}
</code></pre></div></section></main></body></html>