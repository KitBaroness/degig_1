<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/unicode-bidi-0.3.15/src/level.rs`."><title>level.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="unicode_bidi" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../unicode_bidi/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#368" id="368">368</a>
<a href="#369" id="369">369</a>
<a href="#370" id="370">370</a>
<a href="#371" id="371">371</a>
<a href="#372" id="372">372</a>
<a href="#373" id="373">373</a>
<a href="#374" id="374">374</a>
<a href="#375" id="375">375</a>
<a href="#376" id="376">376</a>
<a href="#377" id="377">377</a>
<a href="#378" id="378">378</a>
<a href="#379" id="379">379</a>
<a href="#380" id="380">380</a>
<a href="#381" id="381">381</a>
<a href="#382" id="382">382</a>
<a href="#383" id="383">383</a>
<a href="#384" id="384">384</a>
<a href="#385" id="385">385</a>
<a href="#386" id="386">386</a>
<a href="#387" id="387">387</a>
<a href="#388" id="388">388</a>
<a href="#389" id="389">389</a>
<a href="#390" id="390">390</a>
<a href="#391" id="391">391</a>
<a href="#392" id="392">392</a>
<a href="#393" id="393">393</a>
<a href="#394" id="394">394</a>
<a href="#395" id="395">395</a>
<a href="#396" id="396">396</a>
<a href="#397" id="397">397</a>
<a href="#398" id="398">398</a>
<a href="#399" id="399">399</a>
<a href="#400" id="400">400</a>
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2017 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 &lt;LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0&gt; or the MIT license
// &lt;LICENSE-MIT or http://opensource.org/licenses/MIT&gt;, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

</span><span class="doccomment">//! Bidi Embedding Level
//!
//! See [`Level`](struct.Level.html) for more details.
//!
//! &lt;http://www.unicode.org/reports/tr9/#BD2&gt;

</span><span class="kw">use </span>alloc::string::{String, ToString};
<span class="kw">use </span>alloc::vec::Vec;
<span class="kw">use </span>core::convert::{From, Into};
<span class="kw">use </span>core::slice;

<span class="kw">use </span><span class="kw">super</span>::char_data::BidiClass;

<span class="doccomment">/// Embedding Level
///
/// Embedding Levels are numbers between 0 and 126 (inclusive), where even values denote a
/// left-to-right (LTR) direction and odd values a right-to-left (RTL) direction.
///
/// This struct maintains a *valid* status for level numbers, meaning that creating a new level, or
/// mutating an existing level, with the value smaller than `0` (before conversion to `u8`) or
/// larger than 125 results in an `Error`.
///
/// &lt;http://www.unicode.org/reports/tr9/#BD2&gt;
</span><span class="attr">#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = <span class="string">"serde"</span>, derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
</span><span class="kw">pub struct </span>Level(u8);

<span class="kw">pub const </span>LTR_LEVEL: Level = Level(<span class="number">0</span>);
<span class="kw">pub const </span>RTL_LEVEL: Level = Level(<span class="number">1</span>);

<span class="kw">const </span>MAX_DEPTH: u8 = <span class="number">125</span>;
<span class="doccomment">/// During explicit level resolution, embedding level can go as high as `max_depth`.
</span><span class="kw">pub const </span>MAX_EXPLICIT_DEPTH: u8 = MAX_DEPTH;
<span class="doccomment">/// During implicit level resolution, embedding level can go as high as `max_depth + 1`.
</span><span class="kw">pub const </span>MAX_IMPLICIT_DEPTH: u8 = MAX_DEPTH + <span class="number">1</span>;

<span class="doccomment">/// Errors that can occur on Level creation or mutation
</span><span class="attr">#[derive(Debug, PartialEq)]
</span><span class="kw">pub enum </span>Error {
    <span class="doccomment">/// Out-of-range (invalid) embedding level number.
    </span>OutOfRangeNumber,
}

<span class="kw">impl </span>Level {
    <span class="doccomment">/// New LTR level with smallest number value (0).
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>ltr() -&gt; Level {
        LTR_LEVEL
    }

    <span class="doccomment">/// New RTL level with smallest number value (1).
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>rtl() -&gt; Level {
        RTL_LEVEL
    }

    <span class="doccomment">/// Maximum depth of the directional status stack during implicit resolutions.
    </span><span class="kw">pub fn </span>max_implicit_depth() -&gt; u8 {
        MAX_IMPLICIT_DEPTH
    }

    <span class="doccomment">/// Maximum depth of the directional status stack during explicit resolutions.
    </span><span class="kw">pub fn </span>max_explicit_depth() -&gt; u8 {
        MAX_EXPLICIT_DEPTH
    }

    <span class="comment">// == Inquiries ==

    </span><span class="doccomment">/// Create new level, fail if number is larger than `max_depth + 1`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>new(number: u8) -&gt; <span class="prelude-ty">Result</span>&lt;Level, Error&gt; {
        <span class="kw">if </span>number &lt;= MAX_IMPLICIT_DEPTH {
            <span class="prelude-val">Ok</span>(Level(number))
        } <span class="kw">else </span>{
            <span class="prelude-val">Err</span>(Error::OutOfRangeNumber)
        }
    }

    <span class="doccomment">/// Create new level, fail if number is larger than `max_depth`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>new_explicit(number: u8) -&gt; <span class="prelude-ty">Result</span>&lt;Level, Error&gt; {
        <span class="kw">if </span>number &lt;= MAX_EXPLICIT_DEPTH {
            <span class="prelude-val">Ok</span>(Level(number))
        } <span class="kw">else </span>{
            <span class="prelude-val">Err</span>(Error::OutOfRangeNumber)
        }
    }

    <span class="comment">// == Inquiries ==

    </span><span class="doccomment">/// The level number.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>number(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u8 {
        <span class="self">self</span>.<span class="number">0
    </span>}

    <span class="doccomment">/// If this level is left-to-right.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_ltr(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.<span class="number">0 </span>% <span class="number">2 </span>== <span class="number">0
    </span>}

    <span class="doccomment">/// If this level is right-to-left.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_rtl(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.<span class="number">0 </span>% <span class="number">2 </span>== <span class="number">1
    </span>}

    <span class="comment">// == Mutators ==

    </span><span class="doccomment">/// Raise level by `amount`, fail if number is larger than `max_depth + 1`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>raise(<span class="kw-2">&amp;mut </span><span class="self">self</span>, amount: u8) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0</span>.checked_add(amount) {
            <span class="prelude-val">Some</span>(number) =&gt; {
                <span class="kw">if </span>number &lt;= MAX_IMPLICIT_DEPTH {
                    <span class="self">self</span>.<span class="number">0 </span>= number;
                    <span class="prelude-val">Ok</span>(())
                } <span class="kw">else </span>{
                    <span class="prelude-val">Err</span>(Error::OutOfRangeNumber)
                }
            }
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Err</span>(Error::OutOfRangeNumber),
        }
    }

    <span class="doccomment">/// Raise level by `amount`, fail if number is larger than `max_depth`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>raise_explicit(<span class="kw-2">&amp;mut </span><span class="self">self</span>, amount: u8) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0</span>.checked_add(amount) {
            <span class="prelude-val">Some</span>(number) =&gt; {
                <span class="kw">if </span>number &lt;= MAX_EXPLICIT_DEPTH {
                    <span class="self">self</span>.<span class="number">0 </span>= number;
                    <span class="prelude-val">Ok</span>(())
                } <span class="kw">else </span>{
                    <span class="prelude-val">Err</span>(Error::OutOfRangeNumber)
                }
            }
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Err</span>(Error::OutOfRangeNumber),
        }
    }

    <span class="doccomment">/// Lower level by `amount`, fail if number goes below zero.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>lower(<span class="kw-2">&amp;mut </span><span class="self">self</span>, amount: u8) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0</span>.checked_sub(amount) {
            <span class="prelude-val">Some</span>(number) =&gt; {
                <span class="self">self</span>.<span class="number">0 </span>= number;
                <span class="prelude-val">Ok</span>(())
            }
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Err</span>(Error::OutOfRangeNumber),
        }
    }

    <span class="comment">// == Helpers ==

    </span><span class="doccomment">/// The next LTR (even) level greater than this, or fail if number is larger than `max_depth`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>new_explicit_next_ltr(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Level, Error&gt; {
        Level::new_explicit((<span class="self">self</span>.<span class="number">0 </span>+ <span class="number">2</span>) &amp; !<span class="number">1</span>)
    }

    <span class="doccomment">/// The next RTL (odd) level greater than this, or fail if number is larger than `max_depth`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>new_explicit_next_rtl(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Level, Error&gt; {
        Level::new_explicit((<span class="self">self</span>.<span class="number">0 </span>+ <span class="number">1</span>) | <span class="number">1</span>)
    }

    <span class="doccomment">/// The lowest RTL (odd) level greater than or equal to this, or fail if number is larger than
    /// `max_depth + 1`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>new_lowest_ge_rtl(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Level, Error&gt; {
        Level::new(<span class="self">self</span>.<span class="number">0 </span>| <span class="number">1</span>)
    }

    <span class="doccomment">/// Generate a character type based on a level (as specified in steps X10 and N2).
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>bidi_class(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; BidiClass {
        <span class="kw">if </span><span class="self">self</span>.is_rtl() {
            BidiClass::R
        } <span class="kw">else </span>{
            BidiClass::L
        }
    }

    <span class="kw">pub fn </span>vec(v: <span class="kw-2">&amp;</span>[u8]) -&gt; Vec&lt;Level&gt; {
        v.iter().map(|<span class="kw-2">&amp;</span>x| x.into()).collect()
    }

    <span class="doccomment">/// Converts a byte slice to a slice of Levels
    ///
    /// Does _not_ check if each level is within bounds (`&lt;=` [`MAX_IMPLICIT_DEPTH`]),
    /// which is not a requirement for safety but is a requirement for correctness of the algorithm.
    </span><span class="kw">pub fn </span>from_slice_unchecked(v: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="kw-2">&amp;</span>[Level] {
        <span class="macro">debug_assert_eq!</span>(core::mem::size_of::&lt;u8&gt;(), core::mem::size_of::&lt;Level&gt;());
        <span class="kw">unsafe </span>{
            <span class="comment">// Safety: The two arrays are the same size and layout-compatible since
            // Level is `repr(transparent)` over `u8`
            </span>slice::from_raw_parts(v <span class="kw">as </span><span class="kw-2">*const </span>[u8] <span class="kw">as </span><span class="kw-2">*const </span>u8 <span class="kw">as </span><span class="kw-2">*const </span>Level, v.len())
        }
    }
}

<span class="doccomment">/// If levels has any RTL (odd) level
///
/// This information is usually used to skip re-ordering of text when no RTL level is present
</span><span class="attr">#[inline]
</span><span class="kw">pub fn </span>has_rtl(levels: <span class="kw-2">&amp;</span>[Level]) -&gt; bool {
    levels.iter().any(|<span class="kw-2">&amp;</span>lvl| lvl.is_rtl())
}

<span class="kw">impl </span>Into&lt;u8&gt; <span class="kw">for </span>Level {
    <span class="doccomment">/// Convert to the level number
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>into(<span class="self">self</span>) -&gt; u8 {
        <span class="self">self</span>.number()
    }
}

<span class="kw">impl </span>From&lt;u8&gt; <span class="kw">for </span>Level {
    <span class="doccomment">/// Create level by number
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>from(number: u8) -&gt; Level {
        Level::new(number).expect(<span class="string">"Level number error"</span>)
    }
}

<span class="doccomment">/// Used for matching levels in conformance tests
</span><span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; PartialEq&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str&gt; <span class="kw">for </span>Level {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, s: &amp;&amp;<span class="lifetime">'a </span>str) -&gt; bool {
        <span class="kw-2">*</span>s == <span class="string">"x" </span>|| <span class="kw-2">*</span>s == <span class="self">self</span>.<span class="number">0</span>.to_string()
    }
}

<span class="doccomment">/// Used for matching levels in conformance tests
</span><span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; PartialEq&lt;String&gt; <span class="kw">for </span>Level {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, s: <span class="kw-2">&amp;</span>String) -&gt; bool {
        <span class="self">self </span>== <span class="kw-2">&amp;</span>s.as_str()
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_new() {
        <span class="macro">assert_eq!</span>(Level::new(<span class="number">0</span>), <span class="prelude-val">Ok</span>(Level(<span class="number">0</span>)));
        <span class="macro">assert_eq!</span>(Level::new(<span class="number">1</span>), <span class="prelude-val">Ok</span>(Level(<span class="number">1</span>)));
        <span class="macro">assert_eq!</span>(Level::new(<span class="number">10</span>), <span class="prelude-val">Ok</span>(Level(<span class="number">10</span>)));
        <span class="macro">assert_eq!</span>(Level::new(<span class="number">125</span>), <span class="prelude-val">Ok</span>(Level(<span class="number">125</span>)));
        <span class="macro">assert_eq!</span>(Level::new(<span class="number">126</span>), <span class="prelude-val">Ok</span>(Level(<span class="number">126</span>)));
        <span class="macro">assert_eq!</span>(Level::new(<span class="number">127</span>), <span class="prelude-val">Err</span>(Error::OutOfRangeNumber));
        <span class="macro">assert_eq!</span>(Level::new(<span class="number">255</span>), <span class="prelude-val">Err</span>(Error::OutOfRangeNumber));
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_new_explicit() {
        <span class="macro">assert_eq!</span>(Level::new_explicit(<span class="number">0</span>), <span class="prelude-val">Ok</span>(Level(<span class="number">0</span>)));
        <span class="macro">assert_eq!</span>(Level::new_explicit(<span class="number">1</span>), <span class="prelude-val">Ok</span>(Level(<span class="number">1</span>)));
        <span class="macro">assert_eq!</span>(Level::new_explicit(<span class="number">10</span>), <span class="prelude-val">Ok</span>(Level(<span class="number">10</span>)));
        <span class="macro">assert_eq!</span>(Level::new_explicit(<span class="number">125</span>), <span class="prelude-val">Ok</span>(Level(<span class="number">125</span>)));
        <span class="macro">assert_eq!</span>(Level::new_explicit(<span class="number">126</span>), <span class="prelude-val">Err</span>(Error::OutOfRangeNumber));
        <span class="macro">assert_eq!</span>(Level::new_explicit(<span class="number">255</span>), <span class="prelude-val">Err</span>(Error::OutOfRangeNumber));
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_is_ltr() {
        <span class="macro">assert_eq!</span>(Level(<span class="number">0</span>).is_ltr(), <span class="bool-val">true</span>);
        <span class="macro">assert_eq!</span>(Level(<span class="number">1</span>).is_ltr(), <span class="bool-val">false</span>);
        <span class="macro">assert_eq!</span>(Level(<span class="number">10</span>).is_ltr(), <span class="bool-val">true</span>);
        <span class="macro">assert_eq!</span>(Level(<span class="number">11</span>).is_ltr(), <span class="bool-val">false</span>);
        <span class="macro">assert_eq!</span>(Level(<span class="number">124</span>).is_ltr(), <span class="bool-val">true</span>);
        <span class="macro">assert_eq!</span>(Level(<span class="number">125</span>).is_ltr(), <span class="bool-val">false</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_is_rtl() {
        <span class="macro">assert_eq!</span>(Level(<span class="number">0</span>).is_rtl(), <span class="bool-val">false</span>);
        <span class="macro">assert_eq!</span>(Level(<span class="number">1</span>).is_rtl(), <span class="bool-val">true</span>);
        <span class="macro">assert_eq!</span>(Level(<span class="number">10</span>).is_rtl(), <span class="bool-val">false</span>);
        <span class="macro">assert_eq!</span>(Level(<span class="number">11</span>).is_rtl(), <span class="bool-val">true</span>);
        <span class="macro">assert_eq!</span>(Level(<span class="number">124</span>).is_rtl(), <span class="bool-val">false</span>);
        <span class="macro">assert_eq!</span>(Level(<span class="number">125</span>).is_rtl(), <span class="bool-val">true</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_raise() {
        <span class="kw">let </span><span class="kw-2">mut </span>level = Level::ltr();
        <span class="macro">assert_eq!</span>(level.number(), <span class="number">0</span>);
        <span class="macro">assert!</span>(level.raise(<span class="number">100</span>).is_ok());
        <span class="macro">assert_eq!</span>(level.number(), <span class="number">100</span>);
        <span class="macro">assert!</span>(level.raise(<span class="number">26</span>).is_ok());
        <span class="macro">assert_eq!</span>(level.number(), <span class="number">126</span>);
        <span class="macro">assert!</span>(level.raise(<span class="number">1</span>).is_err()); <span class="comment">// invalid!
        </span><span class="macro">assert!</span>(level.raise(<span class="number">250</span>).is_err()); <span class="comment">// overflow!
        </span><span class="macro">assert_eq!</span>(level.number(), <span class="number">126</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_raise_explicit() {
        <span class="kw">let </span><span class="kw-2">mut </span>level = Level::ltr();
        <span class="macro">assert_eq!</span>(level.number(), <span class="number">0</span>);
        <span class="macro">assert!</span>(level.raise_explicit(<span class="number">100</span>).is_ok());
        <span class="macro">assert_eq!</span>(level.number(), <span class="number">100</span>);
        <span class="macro">assert!</span>(level.raise_explicit(<span class="number">25</span>).is_ok());
        <span class="macro">assert_eq!</span>(level.number(), <span class="number">125</span>);
        <span class="macro">assert!</span>(level.raise_explicit(<span class="number">1</span>).is_err()); <span class="comment">// invalid!
        </span><span class="macro">assert!</span>(level.raise_explicit(<span class="number">250</span>).is_err()); <span class="comment">// overflow!
        </span><span class="macro">assert_eq!</span>(level.number(), <span class="number">125</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_lower() {
        <span class="kw">let </span><span class="kw-2">mut </span>level = Level::rtl();
        <span class="macro">assert_eq!</span>(level.number(), <span class="number">1</span>);
        <span class="macro">assert!</span>(level.lower(<span class="number">1</span>).is_ok());
        <span class="macro">assert_eq!</span>(level.number(), <span class="number">0</span>);
        <span class="macro">assert!</span>(level.lower(<span class="number">1</span>).is_err()); <span class="comment">// underflow!
        </span><span class="macro">assert!</span>(level.lower(<span class="number">250</span>).is_err()); <span class="comment">// underflow!
        </span><span class="macro">assert_eq!</span>(level.number(), <span class="number">0</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_has_rtl() {
        <span class="macro">assert_eq!</span>(has_rtl(<span class="kw-2">&amp;</span>Level::vec(<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>])), <span class="bool-val">false</span>);
        <span class="macro">assert_eq!</span>(has_rtl(<span class="kw-2">&amp;</span>Level::vec(<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">1</span>, <span class="number">0</span>])), <span class="bool-val">true</span>);
        <span class="macro">assert_eq!</span>(has_rtl(<span class="kw-2">&amp;</span>Level::vec(<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">2</span>, <span class="number">0</span>])), <span class="bool-val">false</span>);
        <span class="macro">assert_eq!</span>(has_rtl(<span class="kw-2">&amp;</span>Level::vec(<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">125</span>, <span class="number">0</span>])), <span class="bool-val">true</span>);
        <span class="macro">assert_eq!</span>(has_rtl(<span class="kw-2">&amp;</span>Level::vec(<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">126</span>, <span class="number">0</span>])), <span class="bool-val">false</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_into() {
        <span class="kw">let </span>level = Level::rtl();
        <span class="kw">let </span>number: u8 = level.into();
        <span class="macro">assert_eq!</span>(<span class="number">1u8</span>, number);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_vec() {
        <span class="macro">assert_eq!</span>(
            Level::vec(<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">1</span>, <span class="number">125</span>]),
            <span class="macro">vec!</span>[Level(<span class="number">0</span>), Level(<span class="number">1</span>), Level(<span class="number">125</span>)]
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_str_eq() {
        <span class="macro">assert_eq!</span>(Level::vec(<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">1</span>, <span class="number">4</span>, <span class="number">125</span>]), <span class="macro">vec!</span>[<span class="string">"0"</span>, <span class="string">"1"</span>, <span class="string">"x"</span>, <span class="string">"125"</span>]);
        <span class="macro">assert_ne!</span>(Level::vec(<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">1</span>, <span class="number">4</span>, <span class="number">125</span>]), <span class="macro">vec!</span>[<span class="string">"0"</span>, <span class="string">"1"</span>, <span class="string">"5"</span>, <span class="string">"125"</span>]);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_string_eq() {
        <span class="macro">assert_eq!</span>(
            Level::vec(<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">1</span>, <span class="number">4</span>, <span class="number">125</span>]),
            <span class="macro">vec!</span>[<span class="string">"0"</span>.to_string(), <span class="string">"1"</span>.to_string(), <span class="string">"x"</span>.to_string(), <span class="string">"125"</span>.to_string()]
        );
    }
}

<span class="attr">#[cfg(all(feature = <span class="string">"serde"</span>, test))]
</span><span class="kw">mod </span>serde_tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;
    <span class="kw">use </span>serde_test::{assert_tokens, Token};

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_statics() {
        assert_tokens(
            <span class="kw-2">&amp;</span>Level::ltr(),
            <span class="kw-2">&amp;</span>[Token::NewtypeStruct { name: <span class="string">"Level" </span>}, Token::U8(<span class="number">0</span>)],
        );
        assert_tokens(
            <span class="kw-2">&amp;</span>Level::rtl(),
            <span class="kw-2">&amp;</span>[Token::NewtypeStruct { name: <span class="string">"Level" </span>}, Token::U8(<span class="number">1</span>)],
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_new() {
        <span class="kw">let </span>level = Level::new(<span class="number">42</span>).unwrap();
        assert_tokens(
            <span class="kw-2">&amp;</span>level,
            <span class="kw-2">&amp;</span>[Token::NewtypeStruct { name: <span class="string">"Level" </span>}, Token::U8(<span class="number">42</span>)],
        );
    }
}
</code></pre></div></section></main></body></html>