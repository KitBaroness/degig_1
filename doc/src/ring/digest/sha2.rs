<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.17.8/src/digest/sha2.rs`."><title>sha2.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="ring" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../ring/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2019 Brian Smith.
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

</span><span class="kw">use </span><span class="kw">crate</span>::c;
<span class="kw">use </span>core::{
    num::Wrapping,
    ops::{Add, AddAssign, BitAnd, BitOr, BitXor, Not, Shr},
};

<span class="attr">#[cfg(not(any(target_arch = <span class="string">"aarch64"</span>, target_arch = <span class="string">"arm"</span>, target_arch = <span class="string">"x86_64"</span>)))]
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">extern </span><span class="string">"C" </span><span class="kw">fn </span>sha256_block_data_order(
    state: <span class="kw-2">&amp;mut </span><span class="kw">super</span>::State,
    data: <span class="kw-2">*const </span>u8,
    num: c::size_t,
) {
    <span class="kw">let </span>state = <span class="kw">unsafe </span>{ <span class="kw-2">&amp;mut </span>state.as32 };
    <span class="kw-2">*</span>state = block_data_order(<span class="kw-2">*</span>state, data, num)
}

<span class="attr">#[cfg(not(any(target_arch = <span class="string">"aarch64"</span>, target_arch = <span class="string">"arm"</span>, target_arch = <span class="string">"x86_64"</span>)))]
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">extern </span><span class="string">"C" </span><span class="kw">fn </span>sha512_block_data_order(
    state: <span class="kw-2">&amp;mut </span><span class="kw">super</span>::State,
    data: <span class="kw-2">*const </span>u8,
    num: c::size_t,
) {
    <span class="kw">let </span>state = <span class="kw">unsafe </span>{ <span class="kw-2">&amp;mut </span>state.as64 };
    <span class="kw-2">*</span>state = block_data_order(<span class="kw-2">*</span>state, data, num)
}

<span class="attr">#[cfg_attr(
    any(target_arch = <span class="string">"aarch64"</span>, target_arch = <span class="string">"arm"</span>, target_arch = <span class="string">"x86_64"</span>),
    allow(dead_code)
)]
#[inline]
</span><span class="kw">fn </span>block_data_order&lt;S: Sha2&gt;(
    <span class="kw-2">mut </span>H: [S; CHAINING_WORDS],
    M: <span class="kw-2">*const </span>u8,
    num: c::size_t,
) -&gt; [S; CHAINING_WORDS] {
    <span class="kw">let </span>M = M.cast::&lt;[S::InputBytes; <span class="number">16</span>]&gt;();
    <span class="kw">let </span>M: <span class="kw-2">&amp;</span>[[S::InputBytes; <span class="number">16</span>]] = <span class="kw">unsafe </span>{ core::slice::from_raw_parts(M, num) };

    <span class="kw">for </span>M <span class="kw">in </span>M {
        <span class="comment">// FIPS 180-4 {6.2.2, 6.4.2} Step 1
        //
        // TODO: Use `let W: [S::ZERO; S::ROUNDS]` instead of allocating
        // `MAX_ROUNDS` items and then slicing to `K.len()`; depends on
        // https://github.com/rust-lang/rust/issues/43408.
        </span><span class="kw">let </span><span class="kw-2">mut </span>W = [S::ZERO; MAX_ROUNDS];
        <span class="kw">let </span>W: <span class="kw-2">&amp;</span>[S] = {
            <span class="kw">let </span>W = <span class="kw-2">&amp;mut </span>W[..S::K.len()];
            <span class="kw">for </span>(W, M) <span class="kw">in </span>W.iter_mut().zip(M) {
                <span class="kw-2">*</span>W = S::from_be_bytes(<span class="kw-2">*</span>M);
            }
            <span class="kw">for </span>t <span class="kw">in </span>M.len()..S::K.len() {
                W[t] = sigma_1(W[t - <span class="number">2</span>]) + W[t - <span class="number">7</span>] + sigma_0(W[t - <span class="number">15</span>]) + W[t - <span class="number">16</span>]
            }

            W
        };

        <span class="comment">// FIPS 180-4 {6.2.2, 6.4.2} Step 2
        </span><span class="kw">let </span>[<span class="kw-2">mut </span>a, <span class="kw-2">mut </span>b, <span class="kw-2">mut </span>c, <span class="kw-2">mut </span>d, <span class="kw-2">mut </span>e, <span class="kw-2">mut </span>f, <span class="kw-2">mut </span>g, <span class="kw-2">mut </span>h] = H;

        <span class="comment">// FIPS 180-4 {6.2.2, 6.4.2} Step 3
        </span><span class="kw">for </span>(Kt, Wt) <span class="kw">in </span>S::K.iter().zip(W.iter()) {
            <span class="kw">let </span>T1 = h + SIGMA_1(e) + ch(e, f, g) + <span class="kw-2">*</span>Kt + <span class="kw-2">*</span>Wt;
            <span class="kw">let </span>T2 = SIGMA_0(a) + maj(a, b, c);
            h = g;
            g = f;
            f = e;
            e = d + T1;
            d = c;
            c = b;
            b = a;
            a = T1 + T2;
        }

        <span class="comment">// FIPS 180-4 {6.2.2, 6.4.2} Step 4
        </span>H[<span class="number">0</span>] += a;
        H[<span class="number">1</span>] += b;
        H[<span class="number">2</span>] += c;
        H[<span class="number">3</span>] += d;
        H[<span class="number">4</span>] += e;
        H[<span class="number">5</span>] += f;
        H[<span class="number">6</span>] += g;
        H[<span class="number">7</span>] += h;
    }

    H
}

<span class="comment">// FIPS 180-4 {4.1.1, 4.1.2, 4.1.3}
</span><span class="attr">#[inline(always)]
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>ch&lt;W: Word&gt;(x: W, y: W, z: W) -&gt; W {
    (x &amp; y) | (!x &amp; z)
}

<span class="comment">// FIPS 180-4 {4.1.1, 4.1.2, 4.1.3}
</span><span class="attr">#[inline(always)]
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>maj&lt;W: Word&gt;(x: W, y: W, z: W) -&gt; W {
    (x &amp; y) | (x &amp; z) | (y &amp; z)
}

<span class="comment">// FIPS 180-4 {4.1.2, 4.1.3}
</span><span class="attr">#[inline(always)]
</span><span class="kw">fn </span>SIGMA_0&lt;S: Sha2&gt;(x: S) -&gt; S {
    x.rotr(S::BIG_SIGMA_0.<span class="number">0</span>) ^ x.rotr(S::BIG_SIGMA_0.<span class="number">1</span>) ^ x.rotr(S::BIG_SIGMA_0.<span class="number">2</span>)
}

<span class="comment">// FIPS 180-4 {4.1.2, 4.1.3}
</span><span class="attr">#[inline(always)]
</span><span class="kw">fn </span>SIGMA_1&lt;S: Sha2&gt;(x: S) -&gt; S {
    x.rotr(S::BIG_SIGMA_1.<span class="number">0</span>) ^ x.rotr(S::BIG_SIGMA_1.<span class="number">1</span>) ^ x.rotr(S::BIG_SIGMA_1.<span class="number">2</span>)
}

<span class="comment">// FIPS 180-4 {4.1.2, 4.1.3}
</span><span class="attr">#[inline(always)]
</span><span class="kw">fn </span>sigma_0&lt;S: Sha2&gt;(x: S) -&gt; S {
    x.rotr(S::SMALL_SIGMA_0.<span class="number">0</span>) ^ x.rotr(S::SMALL_SIGMA_0.<span class="number">1</span>) ^ (x &gt;&gt; S::SMALL_SIGMA_0.<span class="number">2</span>)
}

<span class="comment">// FIPS 180-4 {4.1.2, 4.1.3}
</span><span class="attr">#[inline(always)]
</span><span class="kw">fn </span>sigma_1&lt;S: Sha2&gt;(x: S) -&gt; S {
    x.rotr(S::SMALL_SIGMA_1.<span class="number">0</span>) ^ x.rotr(S::SMALL_SIGMA_1.<span class="number">1</span>) ^ (x &gt;&gt; S::SMALL_SIGMA_1.<span class="number">2</span>)
}

<span class="comment">// Commonality between SHA-1 and SHA-2 words.
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">trait </span>Word:
    <span class="lifetime">'static
    </span>+ Sized
    + Copy
    + Add&lt;Output = <span class="self">Self</span>&gt;
    + AddAssign
    + BitAnd&lt;Output = <span class="self">Self</span>&gt;
    + BitOr&lt;Output = <span class="self">Self</span>&gt;
    + Not&lt;Output = <span class="self">Self</span>&gt;
{
    <span class="kw">const </span>ZERO: <span class="self">Self</span>;

    <span class="kw">type </span>InputBytes: Copy;

    <span class="kw">fn </span>from_be_bytes(input: <span class="self">Self</span>::InputBytes) -&gt; <span class="self">Self</span>;

    <span class="kw">fn </span>rotr(<span class="self">self</span>, count: u32) -&gt; <span class="self">Self</span>;
}

<span class="doccomment">/// A SHA-2 input word.
</span><span class="kw">trait </span>Sha2: Word + BitXor&lt;Output = <span class="self">Self</span>&gt; + Shr&lt;usize, Output = <span class="self">Self</span>&gt; {
    <span class="kw">const </span>BIG_SIGMA_0: (u32, u32, u32);
    <span class="kw">const </span>BIG_SIGMA_1: (u32, u32, u32);
    <span class="kw">const </span>SMALL_SIGMA_0: (u32, u32, usize);
    <span class="kw">const </span>SMALL_SIGMA_1: (u32, u32, usize);

    <span class="kw">const </span>K: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[<span class="self">Self</span>];
}

<span class="kw">const </span>MAX_ROUNDS: usize = <span class="number">80</span>;
<span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">const </span>CHAINING_WORDS: usize = <span class="number">8</span>;

<span class="kw">impl </span>Word <span class="kw">for </span>Wrapping&lt;u32&gt; {
    <span class="kw">const </span>ZERO: <span class="self">Self </span>= <span class="self">Self</span>(<span class="number">0</span>);
    <span class="kw">type </span>InputBytes = [u8; <span class="number">4</span>];

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>from_be_bytes(input: <span class="self">Self</span>::InputBytes) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>(u32::from_be_bytes(input))
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>rotr(<span class="self">self</span>, count: u32) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>(<span class="self">self</span>.<span class="number">0</span>.rotate_right(count))
    }
}

<span class="comment">// SHA-256
</span><span class="kw">impl </span>Sha2 <span class="kw">for </span>Wrapping&lt;u32&gt; {
    <span class="comment">// FIPS 180-4 4.1.2
    </span><span class="kw">const </span>BIG_SIGMA_0: (u32, u32, u32) = (<span class="number">2</span>, <span class="number">13</span>, <span class="number">22</span>);
    <span class="kw">const </span>BIG_SIGMA_1: (u32, u32, u32) = (<span class="number">6</span>, <span class="number">11</span>, <span class="number">25</span>);
    <span class="kw">const </span>SMALL_SIGMA_0: (u32, u32, usize) = (<span class="number">7</span>, <span class="number">18</span>, <span class="number">3</span>);
    <span class="kw">const </span>SMALL_SIGMA_1: (u32, u32, usize) = (<span class="number">17</span>, <span class="number">19</span>, <span class="number">10</span>);

    <span class="comment">// FIPS 180-4 4.2.2
    </span><span class="kw">const </span>K: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[<span class="self">Self</span>] = <span class="kw-2">&amp;</span>[
        <span class="self">Self</span>(<span class="number">0x428a2f98</span>),
        <span class="self">Self</span>(<span class="number">0x71374491</span>),
        <span class="self">Self</span>(<span class="number">0xb5c0fbcf</span>),
        <span class="self">Self</span>(<span class="number">0xe9b5dba5</span>),
        <span class="self">Self</span>(<span class="number">0x3956c25b</span>),
        <span class="self">Self</span>(<span class="number">0x59f111f1</span>),
        <span class="self">Self</span>(<span class="number">0x923f82a4</span>),
        <span class="self">Self</span>(<span class="number">0xab1c5ed5</span>),
        <span class="self">Self</span>(<span class="number">0xd807aa98</span>),
        <span class="self">Self</span>(<span class="number">0x12835b01</span>),
        <span class="self">Self</span>(<span class="number">0x243185be</span>),
        <span class="self">Self</span>(<span class="number">0x550c7dc3</span>),
        <span class="self">Self</span>(<span class="number">0x72be5d74</span>),
        <span class="self">Self</span>(<span class="number">0x80deb1fe</span>),
        <span class="self">Self</span>(<span class="number">0x9bdc06a7</span>),
        <span class="self">Self</span>(<span class="number">0xc19bf174</span>),
        <span class="self">Self</span>(<span class="number">0xe49b69c1</span>),
        <span class="self">Self</span>(<span class="number">0xefbe4786</span>),
        <span class="self">Self</span>(<span class="number">0x0fc19dc6</span>),
        <span class="self">Self</span>(<span class="number">0x240ca1cc</span>),
        <span class="self">Self</span>(<span class="number">0x2de92c6f</span>),
        <span class="self">Self</span>(<span class="number">0x4a7484aa</span>),
        <span class="self">Self</span>(<span class="number">0x5cb0a9dc</span>),
        <span class="self">Self</span>(<span class="number">0x76f988da</span>),
        <span class="self">Self</span>(<span class="number">0x983e5152</span>),
        <span class="self">Self</span>(<span class="number">0xa831c66d</span>),
        <span class="self">Self</span>(<span class="number">0xb00327c8</span>),
        <span class="self">Self</span>(<span class="number">0xbf597fc7</span>),
        <span class="self">Self</span>(<span class="number">0xc6e00bf3</span>),
        <span class="self">Self</span>(<span class="number">0xd5a79147</span>),
        <span class="self">Self</span>(<span class="number">0x06ca6351</span>),
        <span class="self">Self</span>(<span class="number">0x14292967</span>),
        <span class="self">Self</span>(<span class="number">0x27b70a85</span>),
        <span class="self">Self</span>(<span class="number">0x2e1b2138</span>),
        <span class="self">Self</span>(<span class="number">0x4d2c6dfc</span>),
        <span class="self">Self</span>(<span class="number">0x53380d13</span>),
        <span class="self">Self</span>(<span class="number">0x650a7354</span>),
        <span class="self">Self</span>(<span class="number">0x766a0abb</span>),
        <span class="self">Self</span>(<span class="number">0x81c2c92e</span>),
        <span class="self">Self</span>(<span class="number">0x92722c85</span>),
        <span class="self">Self</span>(<span class="number">0xa2bfe8a1</span>),
        <span class="self">Self</span>(<span class="number">0xa81a664b</span>),
        <span class="self">Self</span>(<span class="number">0xc24b8b70</span>),
        <span class="self">Self</span>(<span class="number">0xc76c51a3</span>),
        <span class="self">Self</span>(<span class="number">0xd192e819</span>),
        <span class="self">Self</span>(<span class="number">0xd6990624</span>),
        <span class="self">Self</span>(<span class="number">0xf40e3585</span>),
        <span class="self">Self</span>(<span class="number">0x106aa070</span>),
        <span class="self">Self</span>(<span class="number">0x19a4c116</span>),
        <span class="self">Self</span>(<span class="number">0x1e376c08</span>),
        <span class="self">Self</span>(<span class="number">0x2748774c</span>),
        <span class="self">Self</span>(<span class="number">0x34b0bcb5</span>),
        <span class="self">Self</span>(<span class="number">0x391c0cb3</span>),
        <span class="self">Self</span>(<span class="number">0x4ed8aa4a</span>),
        <span class="self">Self</span>(<span class="number">0x5b9cca4f</span>),
        <span class="self">Self</span>(<span class="number">0x682e6ff3</span>),
        <span class="self">Self</span>(<span class="number">0x748f82ee</span>),
        <span class="self">Self</span>(<span class="number">0x78a5636f</span>),
        <span class="self">Self</span>(<span class="number">0x84c87814</span>),
        <span class="self">Self</span>(<span class="number">0x8cc70208</span>),
        <span class="self">Self</span>(<span class="number">0x90befffa</span>),
        <span class="self">Self</span>(<span class="number">0xa4506ceb</span>),
        <span class="self">Self</span>(<span class="number">0xbef9a3f7</span>),
        <span class="self">Self</span>(<span class="number">0xc67178f2</span>),
    ];
}

<span class="kw">impl </span>Word <span class="kw">for </span>Wrapping&lt;u64&gt; {
    <span class="kw">const </span>ZERO: <span class="self">Self </span>= <span class="self">Self</span>(<span class="number">0</span>);
    <span class="kw">type </span>InputBytes = [u8; <span class="number">8</span>];

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>from_be_bytes(input: <span class="self">Self</span>::InputBytes) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>(u64::from_be_bytes(input))
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>rotr(<span class="self">self</span>, count: u32) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>(<span class="self">self</span>.<span class="number">0</span>.rotate_right(count))
    }
}

<span class="comment">// SHA-384 and SHA-512
</span><span class="kw">impl </span>Sha2 <span class="kw">for </span>Wrapping&lt;u64&gt; {
    <span class="comment">// FIPS 180-4 4.1.3
    </span><span class="kw">const </span>BIG_SIGMA_0: (u32, u32, u32) = (<span class="number">28</span>, <span class="number">34</span>, <span class="number">39</span>);
    <span class="kw">const </span>BIG_SIGMA_1: (u32, u32, u32) = (<span class="number">14</span>, <span class="number">18</span>, <span class="number">41</span>);
    <span class="kw">const </span>SMALL_SIGMA_0: (u32, u32, usize) = (<span class="number">1</span>, <span class="number">8</span>, <span class="number">7</span>);
    <span class="kw">const </span>SMALL_SIGMA_1: (u32, u32, usize) = (<span class="number">19</span>, <span class="number">61</span>, <span class="number">6</span>);

    <span class="comment">// FIPS 180-4 4.2.3
    </span><span class="kw">const </span>K: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[<span class="self">Self</span>] = <span class="kw-2">&amp;</span>[
        <span class="self">Self</span>(<span class="number">0x428a2f98d728ae22</span>),
        <span class="self">Self</span>(<span class="number">0x7137449123ef65cd</span>),
        <span class="self">Self</span>(<span class="number">0xb5c0fbcfec4d3b2f</span>),
        <span class="self">Self</span>(<span class="number">0xe9b5dba58189dbbc</span>),
        <span class="self">Self</span>(<span class="number">0x3956c25bf348b538</span>),
        <span class="self">Self</span>(<span class="number">0x59f111f1b605d019</span>),
        <span class="self">Self</span>(<span class="number">0x923f82a4af194f9b</span>),
        <span class="self">Self</span>(<span class="number">0xab1c5ed5da6d8118</span>),
        <span class="self">Self</span>(<span class="number">0xd807aa98a3030242</span>),
        <span class="self">Self</span>(<span class="number">0x12835b0145706fbe</span>),
        <span class="self">Self</span>(<span class="number">0x243185be4ee4b28c</span>),
        <span class="self">Self</span>(<span class="number">0x550c7dc3d5ffb4e2</span>),
        <span class="self">Self</span>(<span class="number">0x72be5d74f27b896f</span>),
        <span class="self">Self</span>(<span class="number">0x80deb1fe3b1696b1</span>),
        <span class="self">Self</span>(<span class="number">0x9bdc06a725c71235</span>),
        <span class="self">Self</span>(<span class="number">0xc19bf174cf692694</span>),
        <span class="self">Self</span>(<span class="number">0xe49b69c19ef14ad2</span>),
        <span class="self">Self</span>(<span class="number">0xefbe4786384f25e3</span>),
        <span class="self">Self</span>(<span class="number">0x0fc19dc68b8cd5b5</span>),
        <span class="self">Self</span>(<span class="number">0x240ca1cc77ac9c65</span>),
        <span class="self">Self</span>(<span class="number">0x2de92c6f592b0275</span>),
        <span class="self">Self</span>(<span class="number">0x4a7484aa6ea6e483</span>),
        <span class="self">Self</span>(<span class="number">0x5cb0a9dcbd41fbd4</span>),
        <span class="self">Self</span>(<span class="number">0x76f988da831153b5</span>),
        <span class="self">Self</span>(<span class="number">0x983e5152ee66dfab</span>),
        <span class="self">Self</span>(<span class="number">0xa831c66d2db43210</span>),
        <span class="self">Self</span>(<span class="number">0xb00327c898fb213f</span>),
        <span class="self">Self</span>(<span class="number">0xbf597fc7beef0ee4</span>),
        <span class="self">Self</span>(<span class="number">0xc6e00bf33da88fc2</span>),
        <span class="self">Self</span>(<span class="number">0xd5a79147930aa725</span>),
        <span class="self">Self</span>(<span class="number">0x06ca6351e003826f</span>),
        <span class="self">Self</span>(<span class="number">0x142929670a0e6e70</span>),
        <span class="self">Self</span>(<span class="number">0x27b70a8546d22ffc</span>),
        <span class="self">Self</span>(<span class="number">0x2e1b21385c26c926</span>),
        <span class="self">Self</span>(<span class="number">0x4d2c6dfc5ac42aed</span>),
        <span class="self">Self</span>(<span class="number">0x53380d139d95b3df</span>),
        <span class="self">Self</span>(<span class="number">0x650a73548baf63de</span>),
        <span class="self">Self</span>(<span class="number">0x766a0abb3c77b2a8</span>),
        <span class="self">Self</span>(<span class="number">0x81c2c92e47edaee6</span>),
        <span class="self">Self</span>(<span class="number">0x92722c851482353b</span>),
        <span class="self">Self</span>(<span class="number">0xa2bfe8a14cf10364</span>),
        <span class="self">Self</span>(<span class="number">0xa81a664bbc423001</span>),
        <span class="self">Self</span>(<span class="number">0xc24b8b70d0f89791</span>),
        <span class="self">Self</span>(<span class="number">0xc76c51a30654be30</span>),
        <span class="self">Self</span>(<span class="number">0xd192e819d6ef5218</span>),
        <span class="self">Self</span>(<span class="number">0xd69906245565a910</span>),
        <span class="self">Self</span>(<span class="number">0xf40e35855771202a</span>),
        <span class="self">Self</span>(<span class="number">0x106aa07032bbd1b8</span>),
        <span class="self">Self</span>(<span class="number">0x19a4c116b8d2d0c8</span>),
        <span class="self">Self</span>(<span class="number">0x1e376c085141ab53</span>),
        <span class="self">Self</span>(<span class="number">0x2748774cdf8eeb99</span>),
        <span class="self">Self</span>(<span class="number">0x34b0bcb5e19b48a8</span>),
        <span class="self">Self</span>(<span class="number">0x391c0cb3c5c95a63</span>),
        <span class="self">Self</span>(<span class="number">0x4ed8aa4ae3418acb</span>),
        <span class="self">Self</span>(<span class="number">0x5b9cca4f7763e373</span>),
        <span class="self">Self</span>(<span class="number">0x682e6ff3d6b2b8a3</span>),
        <span class="self">Self</span>(<span class="number">0x748f82ee5defb2fc</span>),
        <span class="self">Self</span>(<span class="number">0x78a5636f43172f60</span>),
        <span class="self">Self</span>(<span class="number">0x84c87814a1f0ab72</span>),
        <span class="self">Self</span>(<span class="number">0x8cc702081a6439ec</span>),
        <span class="self">Self</span>(<span class="number">0x90befffa23631e28</span>),
        <span class="self">Self</span>(<span class="number">0xa4506cebde82bde9</span>),
        <span class="self">Self</span>(<span class="number">0xbef9a3f7b2c67915</span>),
        <span class="self">Self</span>(<span class="number">0xc67178f2e372532b</span>),
        <span class="self">Self</span>(<span class="number">0xca273eceea26619c</span>),
        <span class="self">Self</span>(<span class="number">0xd186b8c721c0c207</span>),
        <span class="self">Self</span>(<span class="number">0xeada7dd6cde0eb1e</span>),
        <span class="self">Self</span>(<span class="number">0xf57d4f7fee6ed178</span>),
        <span class="self">Self</span>(<span class="number">0x06f067aa72176fba</span>),
        <span class="self">Self</span>(<span class="number">0x0a637dc5a2c898a6</span>),
        <span class="self">Self</span>(<span class="number">0x113f9804bef90dae</span>),
        <span class="self">Self</span>(<span class="number">0x1b710b35131c471b</span>),
        <span class="self">Self</span>(<span class="number">0x28db77f523047d84</span>),
        <span class="self">Self</span>(<span class="number">0x32caab7b40c72493</span>),
        <span class="self">Self</span>(<span class="number">0x3c9ebe0a15c9bebc</span>),
        <span class="self">Self</span>(<span class="number">0x431d67c49c100d4c</span>),
        <span class="self">Self</span>(<span class="number">0x4cc5d4becb3e42b6</span>),
        <span class="self">Self</span>(<span class="number">0x597f299cfc657e2a</span>),
        <span class="self">Self</span>(<span class="number">0x5fcb6fab3ad6faec</span>),
        <span class="self">Self</span>(<span class="number">0x6c44198c4a475817</span>),
    ];
}

<span class="attr">#[cfg(any(target_arch = <span class="string">"aarch64"</span>, target_arch = <span class="string">"arm"</span>, target_arch = <span class="string">"x86_64"</span>))]
</span><span class="macro">prefixed_extern!</span> {
    <span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>sha256_block_data_order(
        state: <span class="kw-2">&amp;mut </span><span class="kw">super</span>::State,
        data: <span class="kw-2">*const </span>u8,
        num: c::size_t,
    );
    <span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>sha512_block_data_order(
        state: <span class="kw-2">&amp;mut </span><span class="kw">super</span>::State,
        data: <span class="kw-2">*const </span>u8,
        num: c::size_t,
    );
}
</code></pre></div></section></main></body></html>