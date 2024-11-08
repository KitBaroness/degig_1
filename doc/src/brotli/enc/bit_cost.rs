<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/brotli-4.0.0/src/enc/bit_cost.rs`."><title>bit_cost.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="brotli" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../brotli/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="attr">#![allow(dead_code)]
</span><span class="kw">use </span><span class="kw">super</span>::<span class="kw">super</span>::alloc::SliceWrapper;
<span class="kw">use </span><span class="kw">super</span>::histogram::CostAccessors;
<span class="kw">use </span>core;
<span class="attr">#[cfg(feature = <span class="string">"simd"</span>)]
</span><span class="kw">use </span>core::simd::prelude::SimdPartialOrd;

<span class="kw">use </span><span class="kw">super</span>::util::{brotli_max_uint32_t, floatX, FastLog2, FastLog2u16};
<span class="kw">use </span><span class="kw">super</span>::vectorization::{cast_f32_to_i32, cast_i32_to_f32, log2i, sum8, v256, v256i, Mem256i};

<span class="kw">static </span>kCopyBase: [u32; <span class="number">24</span>] = [
    <span class="number">2</span>, <span class="number">3</span>, <span class="number">4</span>, <span class="number">5</span>, <span class="number">6</span>, <span class="number">7</span>, <span class="number">8</span>, <span class="number">9</span>, <span class="number">10</span>, <span class="number">12</span>, <span class="number">14</span>, <span class="number">18</span>, <span class="number">22</span>, <span class="number">30</span>, <span class="number">38</span>, <span class="number">54</span>, <span class="number">70</span>, <span class="number">102</span>, <span class="number">134</span>, <span class="number">198</span>, <span class="number">326</span>, <span class="number">582</span>, <span class="number">1094</span>, <span class="number">2118</span>,
];

<span class="kw">static </span>kCopyExtra: [u32; <span class="number">24</span>] = [
    <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">1</span>, <span class="number">1</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">3</span>, <span class="number">3</span>, <span class="number">4</span>, <span class="number">4</span>, <span class="number">5</span>, <span class="number">5</span>, <span class="number">6</span>, <span class="number">7</span>, <span class="number">8</span>, <span class="number">9</span>, <span class="number">10</span>, <span class="number">24</span>,
];

<span class="kw">static </span>kBrotliMinWindowBits: i32 = <span class="number">10i32</span>;

<span class="kw">static </span>kBrotliMaxWindowBits: i32 = <span class="number">24i32</span>;

<span class="kw">pub fn </span>ShannonEntropy(
    <span class="kw-2">mut </span>population: <span class="kw-2">&amp;</span>[u32],
    size: usize,
    total: <span class="kw-2">&amp;mut </span>usize,
) -&gt; <span class="kw">super</span>::util::floatX {
    <span class="kw">let </span><span class="kw-2">mut </span>sum: usize = <span class="number">0usize</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>retval: <span class="kw">super</span>::util::floatX = <span class="number">0i32 </span><span class="kw">as </span><span class="kw">super</span>::util::floatX;
    <span class="kw">let </span><span class="kw-2">mut </span>p: usize;
    <span class="kw">if </span>size &amp; <span class="number">1 </span>!= <span class="number">0 </span>&amp;&amp; !population.is_empty() {
        p = population[<span class="number">0</span>] <span class="kw">as </span>usize;
        population = population.split_at(<span class="number">1</span>).<span class="number">1</span>;
        sum = sum.wrapping_add(p);
        retval -= p <span class="kw">as </span><span class="kw">super</span>::util::floatX * FastLog2u16(p <span class="kw">as </span>u16);
    }
    <span class="kw">for </span>pop_iter <span class="kw">in </span>population.split_at((size &gt;&gt; <span class="number">1</span>) &lt;&lt; <span class="number">1</span>).<span class="number">0 </span>{
        p = <span class="kw-2">*</span>pop_iter <span class="kw">as </span>usize;
        sum = sum.wrapping_add(p);
        retval -= p <span class="kw">as </span><span class="kw">super</span>::util::floatX * FastLog2u16(p <span class="kw">as </span>u16);
    }
    <span class="kw">if </span>sum != <span class="number">0 </span>{
        retval += sum <span class="kw">as </span><span class="kw">super</span>::util::floatX * FastLog2(sum <span class="kw">as </span>u64); <span class="comment">// not sure it's 16 bit
    </span>}
    <span class="kw-2">*</span>total = sum;
    retval
}

<span class="attr">#[inline(always)]
</span><span class="kw">pub fn </span>BitsEntropy(population: <span class="kw-2">&amp;</span>[u32], size: usize) -&gt; <span class="kw">super</span>::util::floatX {
    <span class="kw">let </span><span class="kw-2">mut </span>sum: usize = <span class="number">0</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>retval: <span class="kw">super</span>::util::floatX = ShannonEntropy(population, size, <span class="kw-2">&amp;mut </span>sum);
    <span class="kw">if </span>retval &lt; sum <span class="kw">as </span><span class="kw">super</span>::util::floatX {
        retval = sum <span class="kw">as </span><span class="kw">super</span>::util::floatX;
    }
    retval
}

<span class="kw">const </span>BROTLI_REPEAT_ZERO_CODE_LENGTH: usize = <span class="number">17</span>;
<span class="kw">const </span>BROTLI_CODE_LENGTH_CODES: usize = BROTLI_REPEAT_ZERO_CODE_LENGTH + <span class="number">1</span>;
<span class="comment">/*
use std::io::{self, Error, ErrorKind, Read, Write};

macro_rules! println_stderr(
    ($($val:tt)*) =&gt; { {
        writeln!(&amp;mut ::std::io::stderr(), $($val)*).unwrap();
    } }
);
*/

</span><span class="attr">#[cfg(feature = <span class="string">"vector_scratch_space"</span>)]
</span><span class="kw">const </span>vectorize_population_cost: bool = <span class="bool-val">true</span>;

<span class="attr">#[cfg(not(feature = <span class="string">"vector_scratch_space"</span>))]
</span><span class="kw">const </span>vectorize_population_cost: bool = <span class="bool-val">false</span>;

<span class="attr">#[allow(clippy::excessive_precision)]
</span><span class="kw">fn </span>CostComputation&lt;T: SliceWrapper&lt;Mem256i&gt;&gt;(
    depth_histo: <span class="kw-2">&amp;mut </span>[u32; BROTLI_CODE_LENGTH_CODES],
    nnz_data: <span class="kw-2">&amp;</span>T,
    nnz: usize,
    total_count: <span class="kw">super</span>::util::floatX,
    log2total: <span class="kw">super</span>::util::floatX,
) -&gt; <span class="kw">super</span>::util::floatX {
    <span class="kw">let </span><span class="kw-2">mut </span>bits: <span class="kw">super</span>::util::floatX = <span class="number">0.0 </span><span class="kw">as </span><span class="kw">super</span>::util::floatX;
    <span class="kw">if </span><span class="bool-val">true </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>max_depth: usize = <span class="number">1</span>;
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..nnz {
            <span class="comment">// Compute -log2(P(symbol)) = -log2(count(symbol)/total_count) =
            //                            = log2(total_count) - log2(count(symbol))
            </span><span class="kw">let </span>element = nnz_data.slice()[i &gt;&gt; <span class="number">3</span>][i &amp; <span class="number">7</span>];
            <span class="kw">let </span>log2p = log2total - FastLog2u16(element <span class="kw">as </span>u16);
            <span class="comment">// Approximate the bit depth by round(-log2(P(symbol)))
            </span><span class="kw">let </span>depth = core::cmp::min((log2p + <span class="number">0.5</span>) <span class="kw">as </span>u8, <span class="number">15u8</span>);
            bits += element <span class="kw">as </span><span class="kw">super</span>::util::floatX * log2p;
            <span class="kw">if </span>(depth <span class="kw">as </span>usize &gt; max_depth) {
                max_depth = depth <span class="kw">as </span>usize;
            }
            depth_histo[depth <span class="kw">as </span>usize] += <span class="number">1</span>;
        }

        <span class="comment">// Add the estimated encoding cost of the code length code histogram.
        </span>bits += (<span class="number">18 </span>+ <span class="number">2 </span>* max_depth) <span class="kw">as </span><span class="kw">super</span>::util::floatX;
        <span class="comment">// Add the entropy of the code length code histogram.
        </span>bits += BitsEntropy(depth_histo, BROTLI_CODE_LENGTH_CODES);
        <span class="comment">//println_stderr!("{:?} {:?}", &amp;depth_histo[..], bits);
        </span><span class="kw">return </span>bits;
    }
    <span class="kw">let </span>rem = nnz &amp; <span class="number">7</span>;
    <span class="kw">let </span>nnz_srl_3 = nnz &gt;&gt; <span class="number">3</span>;
    <span class="kw">if </span><span class="bool-val">true </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>vec_max_depth: [i32; <span class="number">8</span>] = [<span class="number">1</span>; <span class="number">8</span>];
        <span class="kw">let </span><span class="kw-2">mut </span>depth_histo_vec = [[<span class="number">0i32</span>; BROTLI_CODE_LENGTH_CODES]; <span class="number">8</span>];
        <span class="kw">for </span>nnz_data_vec <span class="kw">in </span>nnz_data.slice().split_at(nnz_srl_3).<span class="number">0</span>.iter() {
            <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">8 </span>{
                <span class="comment">// Compute -log2(P(symbol)) = -log2(count(symbol)/total_count) =
                //                            = log2(total_count) - log2(count(symbol))
                </span><span class="kw">let </span>ele = nnz_data_vec[i];
                <span class="kw">let </span>log2p = log2total - FastLog2u16(ele <span class="kw">as </span>u16);
                <span class="comment">// Approximate the bit depth by round(-log2(P(symbol)))
                </span><span class="kw">let </span>depth = core::cmp::min((log2p + <span class="number">0.5</span>) <span class="kw">as </span>i32, <span class="number">15</span>) <span class="kw">as </span>i32;
                bits += ele <span class="kw">as </span><span class="kw">super</span>::util::floatX * log2p;
                vec_max_depth[i] = core::cmp::max(vec_max_depth[i], depth);
                depth_histo_vec[i][depth <span class="kw">as </span>usize] += <span class="number">1</span>;
            }
        }
        <span class="kw">let </span><span class="kw-2">mut </span>max_depth = vec_max_depth[<span class="number">7</span>];
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">8 </span>{
            <span class="kw">for </span>j <span class="kw">in </span><span class="number">0</span>..BROTLI_CODE_LENGTH_CODES {
                depth_histo[j] += depth_histo_vec[i][j] <span class="kw">as </span>u32;
            }
            max_depth = core::cmp::max(vec_max_depth[i], max_depth);
        }
        <span class="kw">if </span>rem != <span class="number">0 </span>{
            <span class="kw">let </span>last_vec = nnz_data.slice()[nnz_srl_3];
            <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..rem {
                <span class="comment">// remainder won't have last element for sure
                </span><span class="kw">let </span>element = last_vec[i];
                <span class="kw">let </span>log2p = log2total - FastLog2u16(element <span class="kw">as </span>u16);
                <span class="comment">// Approximate the bit depth by round(-log2(P(symbol)))
                </span><span class="kw">let </span>depth = core::cmp::min((log2p + <span class="number">0.5</span>) <span class="kw">as </span>i32, <span class="number">15</span>);
                bits += element <span class="kw">as </span><span class="kw">super</span>::util::floatX * log2p;
                max_depth = core::cmp::max(depth, max_depth);
                depth_histo[depth <span class="kw">as </span>usize] += <span class="number">1</span>;
            }
        }
        <span class="comment">// Add the estimated encoding cost of the code length code histogram.
        </span>bits += (<span class="number">18 </span>+ <span class="number">2 </span>* max_depth) <span class="kw">as </span><span class="kw">super</span>::util::floatX;
        <span class="comment">// Add the entropy of the code length code histogram.
        </span>bits += BitsEntropy(depth_histo, BROTLI_CODE_LENGTH_CODES);
        <span class="comment">//println_stderr!("{:?} {:?}", &amp;depth_histo[..], bits);
        </span><span class="kw">return </span>bits;
    }
    <span class="kw">let </span>pow2l: v256 = [
        <span class="number">1.0</span><span class="comment">/*0.7071067811865476*/ </span><span class="kw">as </span>floatX,
        <span class="number">0.3535533905932738 </span><span class="kw">as </span>floatX,
        <span class="number">0.1767766952966369 </span><span class="kw">as </span>floatX,
        <span class="number">0.0883883476483184 </span><span class="kw">as </span>floatX,
        <span class="number">0.0441941738241592 </span><span class="kw">as </span>floatX,
        <span class="number">0.0220970869120796 </span><span class="kw">as </span>floatX,
        <span class="number">0.0110485434560398 </span><span class="kw">as </span>floatX,
        <span class="number">0.0055242717280199 </span><span class="kw">as </span>floatX,
    ]
    .into();
    <span class="kw">let </span>pow2h: v256 = [
        <span class="comment">//FIXME: setr
        </span><span class="number">0.0027621358640100 </span><span class="kw">as </span>floatX,
        <span class="number">0.0013810679320050 </span><span class="kw">as </span>floatX,
        <span class="number">0.0006905339660025 </span><span class="kw">as </span>floatX,
        <span class="number">0.0003452669830012 </span><span class="kw">as </span>floatX,
        <span class="number">0.0001726334915006 </span><span class="kw">as </span>floatX,
        <span class="number">0.0000863167457503 </span><span class="kw">as </span>floatX,
        <span class="number">0.0000431583728752 </span><span class="kw">as </span>floatX,
        <span class="comment">/*0.0000215791864376f*/ </span><span class="number">0.0 </span><span class="kw">as </span>floatX,
    ]
    .into();
    <span class="kw">let </span>ymm_tc = v256::splat(total_count <span class="kw">as </span>floatX);
    <span class="kw">let </span>search_depthl = cast_f32_to_i32(pow2l * ymm_tc);
    <span class="kw">let </span>search_depthh = cast_f32_to_i32(pow2h * ymm_tc);
    <span class="kw">let </span><span class="kw-2">mut </span>suml = v256i::splat(<span class="number">0</span>);
    <span class="kw">let </span><span class="kw-2">mut </span>sumh = v256i::splat(<span class="number">0</span>);
    <span class="kw">for </span>nnz_data_vec <span class="kw">in </span>nnz_data.slice().split_at(nnz_srl_3).<span class="number">0</span>.iter() {
        <span class="kw">for </span>sub_data_item_index <span class="kw">in </span><span class="number">0</span>..<span class="number">8 </span>{
            <span class="kw">let </span>count = v256i::splat(nnz_data_vec[sub_data_item_index]);
            <span class="kw">let </span>cmpl: v256i = count.simd_gt(search_depthl).to_int();
            <span class="kw">let </span>cmph: v256i = count.simd_gt(search_depthh).to_int();
            suml = suml + cmpl;
            sumh = sumh + cmph;
        }
    }
    <span class="kw">if </span>rem != <span class="number">0 </span>{
        <span class="kw">let </span>last_element = nnz_data.slice()[nnz &gt;&gt; <span class="number">3</span>];
        <span class="kw">for </span>sub_index <span class="kw">in </span><span class="number">0</span>..rem {
            <span class="kw">let </span>count = v256i::splat(last_element[sub_index &amp; <span class="number">7</span>]);
            <span class="kw">let </span>cmpl: v256i = count.simd_gt(search_depthl).to_int();
            <span class="kw">let </span>cmph: v256i = count.simd_gt(search_depthh).to_int();
            suml = suml + cmpl;
            sumh = sumh + cmph;
        }
    }
    <span class="kw">let </span><span class="kw-2">mut </span>max_depth: usize = <span class="number">1</span>;
    <span class="comment">// Deal with depth_histo and max_depth
    </span>{
        <span class="kw">let </span>cumulative_sum: [Mem256i; <span class="number">2</span>] = [suml, sumh];
        <span class="kw">let </span><span class="kw-2">mut </span>prev = cumulative_sum[<span class="number">0</span>][<span class="number">0</span>];
        <span class="kw">for </span>j <span class="kw">in </span><span class="number">1</span>..<span class="number">16 </span>{
            <span class="kw">let </span>cur = cumulative_sum[(j &amp; <span class="number">8</span>) &gt;&gt; <span class="number">3</span>][j &amp; <span class="number">7</span>];
            <span class="kw">let </span>delta = cur - prev;
            prev = cur;
            <span class="kw">let </span>cur = <span class="kw-2">&amp;mut </span>depth_histo[j];
            <span class="kw-2">*</span>cur = (<span class="kw-2">*</span>cur <span class="kw">as </span>i32 + delta) <span class="kw">as </span>u32; <span class="comment">// depth_histo[j] += delta
            </span><span class="kw">if </span>delta != <span class="number">0 </span>{
                max_depth = j;
            }
        }
    }
    <span class="kw">let </span>ymm_log2total = v256::splat(log2total);
    <span class="kw">let </span><span class="kw-2">mut </span>bits_cumulative = v256::splat(<span class="number">0.0 </span><span class="kw">as </span>floatX);
    <span class="kw">for </span>nnz_data_item <span class="kw">in </span>nnz_data.slice().split_at(nnz_srl_3).<span class="number">0</span>.iter() {
        <span class="kw">let </span>counts = cast_i32_to_f32(<span class="kw-2">*</span>nnz_data_item);
        <span class="kw">let </span>log_counts = log2i(<span class="kw-2">*</span>nnz_data_item);
        <span class="kw">let </span>log2p = ymm_log2total - log_counts;
        <span class="kw">let </span>tmp = counts * log2p;
        bits_cumulative += tmp;
    }
    bits += sum8(bits_cumulative);
    <span class="kw">if </span>rem != <span class="number">0 </span>{
        <span class="kw">let </span>last_vec = nnz_data.slice()[nnz_srl_3];
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..rem {
            <span class="kw">let </span>last_item = last_vec[i];
            <span class="kw">let </span>log2p = log2total - FastLog2u16(last_item <span class="kw">as </span>u16);
            bits += last_item <span class="kw">as </span><span class="kw">super</span>::util::floatX * log2p;
        }
    }

    <span class="comment">// Add the estimated encoding cost of the code length code histogram.
    </span>bits += (<span class="number">18 </span>+ <span class="number">2 </span>* max_depth) <span class="kw">as </span><span class="kw">super</span>::util::floatX;
    <span class="comment">// Add the entropy of the code length code histogram.
    </span>bits += BitsEntropy(depth_histo, BROTLI_CODE_LENGTH_CODES);
    <span class="comment">//println_stderr!("{:?} {:?}", depth_histo, bits);
    </span>bits
}
<span class="kw">use </span>alloc::SliceWrapperMut;

<span class="kw">pub fn </span>BrotliPopulationCost&lt;HistogramType: SliceWrapper&lt;u32&gt; + CostAccessors&gt;(
    histogram: <span class="kw-2">&amp;</span>HistogramType,
    nnz_data: <span class="kw-2">&amp;mut </span>HistogramType::i32vec,
) -&gt; <span class="kw">super</span>::util::floatX {
    <span class="kw">static </span>kOneSymbolHistogramCost: <span class="kw">super</span>::util::floatX = <span class="number">12i32 </span><span class="kw">as </span><span class="kw">super</span>::util::floatX;
    <span class="kw">static </span>kTwoSymbolHistogramCost: <span class="kw">super</span>::util::floatX = <span class="number">20i32 </span><span class="kw">as </span><span class="kw">super</span>::util::floatX;
    <span class="kw">static </span>kThreeSymbolHistogramCost: <span class="kw">super</span>::util::floatX = <span class="number">28i32 </span><span class="kw">as </span><span class="kw">super</span>::util::floatX;
    <span class="kw">static </span>kFourSymbolHistogramCost: <span class="kw">super</span>::util::floatX = <span class="number">37i32 </span><span class="kw">as </span><span class="kw">super</span>::util::floatX;
    <span class="kw">let </span>data_size: usize = histogram.slice().len();
    <span class="kw">let </span><span class="kw-2">mut </span>count: i32 = <span class="number">0i32</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>s: [usize; <span class="number">5</span>] = [<span class="number">0</span>; <span class="number">5</span>];

    <span class="kw">let </span><span class="kw-2">mut </span>bits: <span class="kw">super</span>::util::floatX = <span class="number">0.0 </span><span class="kw">as </span><span class="kw">super</span>::util::floatX;
    <span class="kw">let </span><span class="kw-2">mut </span>i: usize;
    <span class="kw">if </span>histogram.total_count() == <span class="number">0usize </span>{
        <span class="kw">return </span>kOneSymbolHistogramCost;
    }
    i = <span class="number">0usize</span>;
    <span class="lifetime">'break1</span>: <span class="kw">while </span>i &lt; data_size {
        {
            <span class="kw">if </span>histogram.slice()[i] &gt; <span class="number">0u32 </span>{
                s[count <span class="kw">as </span>usize] = i;
                count += <span class="number">1</span>;
                <span class="kw">if </span>count &gt; <span class="number">4i32 </span>{
                    <span class="kw">break </span><span class="lifetime">'break1</span>;
                }
            }
        }
        i = i.wrapping_add(<span class="number">1</span>);
    }
    <span class="kw">if </span>count == <span class="number">1i32 </span>{
        <span class="kw">return </span>kOneSymbolHistogramCost;
    }
    <span class="kw">if </span>count == <span class="number">2i32 </span>{
        <span class="kw">return </span>kTwoSymbolHistogramCost + histogram.total_count() <span class="kw">as </span><span class="kw">super</span>::util::floatX;
    }
    <span class="kw">if </span>count == <span class="number">3i32 </span>{
        <span class="kw">let </span>histo0: u32 = histogram.slice()[s[<span class="number">0</span>]];
        <span class="kw">let </span>histo1: u32 = histogram.slice()[s[<span class="number">1</span>]];
        <span class="kw">let </span>histo2: u32 = histogram.slice()[s[<span class="number">2</span>]];
        <span class="kw">let </span>histomax: u32 = brotli_max_uint32_t(histo0, brotli_max_uint32_t(histo1, histo2));
        <span class="kw">return </span>kThreeSymbolHistogramCost
            + (<span class="number">2u32</span>).wrapping_mul(histo0.wrapping_add(histo1).wrapping_add(histo2))
                <span class="kw">as </span><span class="kw">super</span>::util::floatX
            - histomax <span class="kw">as </span><span class="kw">super</span>::util::floatX;
    }
    <span class="kw">if </span>count == <span class="number">4i32 </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>histo: [u32; <span class="number">4</span>] = [<span class="number">0</span>; <span class="number">4</span>];

        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0usize</span>..<span class="number">4usize </span>{
            histo[i] = histogram.slice()[s[i]];
        }
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0usize</span>..<span class="number">4usize </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>j: usize;
            j = i.wrapping_add(<span class="number">1</span>);
            <span class="kw">while </span>j &lt; <span class="number">4usize </span>{
                {
                    <span class="kw">if </span>histo[j] &gt; histo[i] {
                        histo.swap(j, i);
                    }
                }
                j = j.wrapping_add(<span class="number">1</span>);
            }
        }
        <span class="kw">let </span>h23: u32 = histo[<span class="number">2</span>].wrapping_add(histo[<span class="number">3</span>]);
        <span class="kw">let </span>histomax: u32 = brotli_max_uint32_t(h23, histo[<span class="number">0</span>]);
        <span class="kw">return </span>kFourSymbolHistogramCost
            + (<span class="number">3u32</span>).wrapping_mul(h23) <span class="kw">as </span><span class="kw">super</span>::util::floatX
            + (<span class="number">2u32</span>).wrapping_mul(histo[<span class="number">0</span>].wrapping_add(histo[<span class="number">1</span>])) <span class="kw">as </span><span class="kw">super</span>::util::floatX
            - histomax <span class="kw">as </span><span class="kw">super</span>::util::floatX;
    }
    <span class="kw">if </span>vectorize_population_cost {
        <span class="comment">// vectorization failed: it's faster to do things inline than split into two loops
        </span><span class="kw">let </span><span class="kw-2">mut </span>nnz: usize = <span class="number">0</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>depth_histo = [<span class="number">0u32</span>; <span class="number">18</span>];
        <span class="kw">let </span>total_count = histogram.total_count() <span class="kw">as </span><span class="kw">super</span>::util::floatX;
        <span class="kw">let </span>log2total = FastLog2(histogram.total_count() <span class="kw">as </span>u64);
        i = <span class="number">0usize</span>;
        <span class="kw">while </span>i &lt; data_size {
            <span class="kw">if </span>histogram.slice()[i] &gt; <span class="number">0u32 </span>{
                <span class="kw">let </span>nnz_val = <span class="kw-2">&amp;mut </span>nnz_data.slice_mut()[nnz &gt;&gt; <span class="number">3</span>];
                nnz_val[nnz &amp; <span class="number">7</span>] = histogram.slice()[i] <span class="kw">as </span>i32;
                i += <span class="number">1</span>;
                nnz += <span class="number">1</span>;
            } <span class="kw">else </span>{
                <span class="kw">let </span><span class="kw-2">mut </span>reps: u32 = <span class="number">1</span>;
                <span class="kw">for </span>hd <span class="kw">in </span>histogram.slice()[i + <span class="number">1</span>..data_size].iter() {
                    <span class="kw">if </span><span class="kw-2">*</span>hd != <span class="number">0 </span>{
                        <span class="kw">break</span>;
                    }
                    reps += <span class="number">1
                </span>}
                i += reps <span class="kw">as </span>usize;
                <span class="kw">if </span>i == data_size {
                    <span class="kw">break</span>;
                }
                <span class="kw">if </span>reps &lt; <span class="number">3 </span>{
                    depth_histo[<span class="number">0</span>] += reps
                } <span class="kw">else </span>{
                    reps -= <span class="number">2</span>;
                    <span class="kw">let </span><span class="kw-2">mut </span>depth_histo_adds: u32 = <span class="number">0</span>;
                    <span class="kw">while </span>reps &gt; <span class="number">0u32 </span>{
                        depth_histo_adds += <span class="number">1</span>;
                        bits += <span class="number">3i32 </span><span class="kw">as </span><span class="kw">super</span>::util::floatX;
                        reps &gt;&gt;= <span class="number">3i32</span>;
                    }
                    depth_histo[BROTLI_REPEAT_ZERO_CODE_LENGTH] += depth_histo_adds;
                }
            }
        }
        bits += CostComputation(<span class="kw-2">&amp;mut </span>depth_histo, nnz_data, nnz, total_count, log2total);
    } <span class="kw">else </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>max_depth: usize = <span class="number">1</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>depth_histo = [<span class="number">0u32</span>; <span class="number">18</span>];
        <span class="kw">let </span>log2total: <span class="kw">super</span>::util::floatX = FastLog2(histogram.total_count() <span class="kw">as </span>u64); <span class="comment">// 64 bit here
        </span><span class="kw">let </span><span class="kw-2">mut </span>reps: u32 = <span class="number">0</span>;
        <span class="kw">for </span>histo <span class="kw">in </span>histogram.slice()[..data_size].iter() {
            <span class="kw">if </span><span class="kw-2">*</span>histo != <span class="number">0 </span>{
                <span class="kw">if </span>reps != <span class="number">0 </span>{
                    <span class="kw">if </span>reps &lt; <span class="number">3 </span>{
                        depth_histo[<span class="number">0</span>] += reps;
                    } <span class="kw">else </span>{
                        reps -= <span class="number">2</span>;
                        <span class="kw">while </span>reps &gt; <span class="number">0u32 </span>{
                            depth_histo[<span class="number">17</span>] += <span class="number">1</span>;
                            bits += <span class="number">3 </span><span class="kw">as </span><span class="kw">super</span>::util::floatX;
                            reps &gt;&gt;= <span class="number">3</span>;
                        }
                    }
                    reps = <span class="number">0</span>;
                }
                <span class="kw">let </span>log2p: <span class="kw">super</span>::util::floatX = log2total - FastLog2u16(<span class="kw-2">*</span>histo <span class="kw">as </span>u16);
                <span class="kw">let </span><span class="kw-2">mut </span>depth: usize = (log2p + <span class="number">0.5 </span><span class="kw">as </span><span class="kw">super</span>::util::floatX) <span class="kw">as </span>usize;
                bits += <span class="kw-2">*</span>histo <span class="kw">as </span><span class="kw">super</span>::util::floatX * log2p;
                depth = core::cmp::min(depth, <span class="number">15</span>);
                max_depth = core::cmp::max(depth, max_depth);
                depth_histo[depth] += <span class="number">1</span>;
            } <span class="kw">else </span>{
                reps += <span class="number">1</span>;
            }
        }
        bits += (<span class="number">18usize</span>).wrapping_add((<span class="number">2usize</span>).wrapping_mul(max_depth)) <span class="kw">as </span><span class="kw">super</span>::util::floatX;
        bits += BitsEntropy(<span class="kw-2">&amp;</span>depth_histo[..], <span class="number">18usize</span>);
    }
    bits
}
<span class="comment">/*
fn HistogramDataSizeCommand() -&gt; usize {
    704usize
}*/

/*
fn HistogramDataSizeDistance() -&gt; usize {
    520usize
}
*/
</span></code></pre></div></section></main></body></html>