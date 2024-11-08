<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/aes-gcm-siv-0.10.3/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="aes_gcm_siv" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../aes_gcm_siv/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#401" id="401">401</a>
<a href="#402" id="402">402</a>
<a href="#403" id="403">403</a>
<a href="#404" id="404">404</a>
<a href="#405" id="405">405</a>
<a href="#406" id="406">406</a>
<a href="#407" id="407">407</a>
<a href="#408" id="408">408</a>
<a href="#409" id="409">409</a>
<a href="#410" id="410">410</a>
<a href="#411" id="411">411</a>
<a href="#412" id="412">412</a>
<a href="#413" id="413">413</a>
<a href="#414" id="414">414</a>
<a href="#415" id="415">415</a>
<a href="#416" id="416">416</a>
<a href="#417" id="417">417</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! [AES-GCM-SIV][1] ([RFC 8452][2]): high-performance
//! [Authenticated Encryption with Associated Data (AEAD)][3] cipher which also
//! provides [nonce reuse misuse resistance][4].
//!
//! Suitable as a general purpose symmetric encryption cipher, AES-GCM-SIV also
//! removes many of the "sharp edges" of AES-GCM, providing significantly better
//! security bounds while simultaneously eliminating the most catastrophic risks
//! of nonce reuse that exist in AES-GCM.
//!
//! Decryption performance is equivalent to AES-GCM.
//! Encryption is marginally slower.
//!
//! See also:
//!
//! - [Adam Langley: AES-GCM-SIV][5]
//! - [Coda Hale: Towards A Safer Footgun][6]
//!
//! ## Performance Notes
//!
//! By default this crate will use software implementations of both AES and
//! the POLYVAL universal hash function.
//!
//! When targeting modern x86/x86_64 CPUs, use the following `RUSTFLAGS` to
//! take advantage of high performance AES-NI and CLMUL CPU intrinsics:
//!
//! ```text
//! RUSTFLAGS="-Ctarget-cpu=sandybridge -Ctarget-feature=+aes,+sse2,+sse4.1,+ssse3"
//! ```
//!
//! ## Security Warning
//!
//! No security audits of this crate have ever been performed.
//!
//! Some of this crate's dependencies were [audited by by NCC Group][7] as part of
//! an audit of the `aes-gcm` crate, including the AES implementations (both AES-NI
//! and a portable software implementation), as well as the `polyval` crate which
//! is used as an authenticator. There were no significant findings.
//!
//! All implementations contained in the crate are designed to execute in constant
//! time, either by relying on hardware intrinsics (i.e. AES-NI and CLMUL on
//! x86/x86_64), or using a portable implementation which is only constant time
//! on processors which implement constant-time multiplication.
//!
//! It is not suitable for use on processors with a variable-time multiplication
//! operation (e.g. short circuit on multiply-by-zero / multiply-by-one, such as
//! certain 32-bit PowerPC CPUs and some non-ARM microcontrollers).
//!
//! USE AT YOUR OWN RISK!
//!
//! # Usage
//!
//! Simple usage (allocating, no associated data):
//!
//! ```
//! use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // Or `Aes128GcmSiv`
//! use aes_gcm_siv::aead::{Aead, NewAead};
//!
//! let key = Key::from_slice(b"an example very very secret key.");
//! let cipher = Aes256GcmSiv::new(key);
//!
//! let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
//!
//! let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
//!     .expect("encryption failure!");  // NOTE: handle this error to avoid panics!
//!
//!
//! let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
//!     .expect("decryption failure!");  // NOTE: handle this error to avoid panics!
//!
//! assert_eq!(&amp;plaintext, b"plaintext message");
//! ```
//!
//! ## In-place Usage (eliminates `alloc` requirement)
//!
//! This crate has an optional `alloc` feature which can be disabled in e.g.
//! microcontroller environments that don't have a heap.
//!
//! The [`AeadInPlace::encrypt_in_place`] and [`AeadInPlace::decrypt_in_place`]
//! methods accept any type that impls the [`aead::Buffer`] trait which
//! contains the plaintext for encryption or ciphertext for decryption.
//!
//! Note that if you enable the `heapless` feature of this crate,
//! you will receive an impl of [`aead::Buffer`] for `heapless::Vec`
//! (re-exported from the [`aead`] crate as [`aead::heapless::Vec`]),
//! which can then be passed as the `buffer` parameter to the in-place encrypt
//! and decrypt methods:
//!
//! ```
//! # #[cfg(feature = "heapless")]
//! # {
//! use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // Or `Aes128GcmSiv`
//! use aes_gcm_siv::aead::{AeadInPlace, NewAead};
//! use aes_gcm_siv::aead::heapless::Vec;
//!
//! let key = Key::from_slice(b"an example very very secret key.");
//! let cipher = Aes256GcmSiv::new(key);
//!
//! let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
//!
//! let mut buffer: Vec&lt;u8, 128&gt; = Vec::new();
//! buffer.extend_from_slice(b"plaintext message");
//!
//! // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
//! cipher.encrypt_in_place(nonce, b"", &amp;mut buffer).expect("encryption failure!");
//!
//! // `buffer` now contains the message ciphertext
//! assert_ne!(&amp;buffer, b"plaintext message");
//!
//! // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
//! cipher.decrypt_in_place(nonce, b"", &amp;mut buffer).expect("decryption failure!");
//! assert_eq!(&amp;buffer, b"plaintext message");
//! # }
//! ```
//!
//! [1]: https://en.wikipedia.org/wiki/AES-GCM-SIV
//! [2]: https://tools.ietf.org/html/rfc8452
//! [3]: https://en.wikipedia.org/wiki/Authenticated_encryption
//! [4]: https://github.com/miscreant/meta/wiki/Nonce-Reuse-Misuse-Resistance
//! [5]: https://www.imperialviolet.org/2017/05/14/aesgcmsiv.html
//! [6]: https://codahale.com/towards-a-safer-footgun/
//! [7]: https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/

</span><span class="attr">#![no_std]
#![doc(
    html_logo_url = <span class="string">"https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"</span>,
    html_favicon_url = <span class="string">"https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"
</span>)]
#![warn(missing_docs, rust_2018_idioms)]

</span><span class="kw">pub use </span>aead;

<span class="kw">use </span>aead::{AeadCore, AeadInPlace, Error, NewAead};
<span class="kw">use </span>cipher::{
    consts::{U0, U12, U16},
    generic_array::{typenum::Unsigned, ArrayLength, GenericArray},
    Block, BlockCipher, BlockEncrypt, FromBlockCipher, NewBlockCipher, StreamCipher,
};
<span class="kw">use </span>ctr::Ctr32LE;
<span class="kw">use </span>polyval::{
    universal_hash::{NewUniversalHash, UniversalHash},
    Polyval,
};
<span class="kw">use </span>zeroize::Zeroize;

<span class="doccomment">/// AES is optional to allow swapping in hardware-specific backends
</span><span class="attr">#[cfg(feature = <span class="string">"aes"</span>)]
</span><span class="kw">use </span>aes::{Aes128, Aes256};

<span class="doccomment">/// Maximum length of associated data (from RFC 8452 Section 6)
</span><span class="kw">pub const </span>A_MAX: u64 = <span class="number">1 </span>&lt;&lt; <span class="number">36</span>;

<span class="doccomment">/// Maximum length of plaintext (from RFC 8452 Section 6)
</span><span class="kw">pub const </span>P_MAX: u64 = <span class="number">1 </span>&lt;&lt; <span class="number">36</span>;

<span class="doccomment">/// Maximum length of ciphertext (from RFC 8452 Section 6)
</span><span class="kw">pub const </span>C_MAX: u64 = (<span class="number">1 </span>&lt;&lt; <span class="number">36</span>) + <span class="number">16</span>;

<span class="doccomment">/// AES-GCM-SIV keys
</span><span class="kw">pub type </span>Key&lt;KeySize&gt; = GenericArray&lt;u8, KeySize&gt;;

<span class="doccomment">/// AES-GCM-SIV nonces
</span><span class="kw">pub type </span>Nonce = GenericArray&lt;u8, U12&gt;;

<span class="doccomment">/// AES-GCM-SIV tags
</span><span class="kw">pub type </span>Tag = GenericArray&lt;u8, U16&gt;;

<span class="doccomment">/// AES-GCM-SIV with a 128-bit key
</span><span class="attr">#[cfg(feature = <span class="string">"aes"</span>)]
</span><span class="kw">pub type </span>Aes128GcmSiv = AesGcmSiv&lt;Aes128&gt;;

<span class="doccomment">/// AES-GCM-SIV with a 256-bit key
</span><span class="attr">#[cfg(feature = <span class="string">"aes"</span>)]
</span><span class="kw">pub type </span>Aes256GcmSiv = AesGcmSiv&lt;Aes256&gt;;

<span class="doccomment">/// AES-GCM-SIV: Misuse-Resistant Authenticated Encryption Cipher (RFC 8452)
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub struct </span>AesGcmSiv&lt;Aes&gt;
<span class="kw">where
    </span>Aes: BlockCipher&lt;BlockSize = U16&gt; + BlockEncrypt,
    Aes::ParBlocks: ArrayLength&lt;Block&lt;Aes&gt;&gt;,
{
    <span class="doccomment">/// Key generating key used to derive AES-GCM-SIV subkeys
    </span>key_generating_key: Aes,
}

<span class="kw">impl</span>&lt;Aes&gt; NewAead <span class="kw">for </span>AesGcmSiv&lt;Aes&gt;
<span class="kw">where
    </span>Aes: NewBlockCipher + BlockCipher&lt;BlockSize = U16&gt; + BlockEncrypt,
    Aes::ParBlocks: ArrayLength&lt;Block&lt;Aes&gt;&gt;,
{
    <span class="kw">type </span>KeySize = Aes::KeySize;

    <span class="kw">fn </span>new(key_bytes: <span class="kw-2">&amp;</span>Key&lt;Aes::KeySize&gt;) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            key_generating_key: Aes::new(key_bytes),
        }
    }
}

<span class="kw">impl</span>&lt;Aes&gt; From&lt;Aes&gt; <span class="kw">for </span>AesGcmSiv&lt;Aes&gt;
<span class="kw">where
    </span>Aes: BlockCipher&lt;BlockSize = U16&gt; + BlockEncrypt,
    Aes::ParBlocks: ArrayLength&lt;Block&lt;Aes&gt;&gt;,
{
    <span class="kw">fn </span>from(key_generating_key: Aes) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{ key_generating_key }
    }
}

<span class="kw">impl</span>&lt;Aes&gt; AeadCore <span class="kw">for </span>AesGcmSiv&lt;Aes&gt;
<span class="kw">where
    </span>Aes: NewBlockCipher + BlockCipher&lt;BlockSize = U16&gt; + BlockEncrypt,
    Aes::ParBlocks: ArrayLength&lt;Block&lt;Aes&gt;&gt;,
{
    <span class="kw">type </span>NonceSize = U12;
    <span class="kw">type </span>TagSize = U16;
    <span class="kw">type </span>CiphertextOverhead = U0;
}

<span class="kw">impl</span>&lt;Aes&gt; AeadInPlace <span class="kw">for </span>AesGcmSiv&lt;Aes&gt;
<span class="kw">where
    </span>Aes: NewBlockCipher + BlockCipher&lt;BlockSize = U16&gt; + BlockEncrypt,
    Aes::ParBlocks: ArrayLength&lt;Block&lt;Aes&gt;&gt;,
{
    <span class="kw">fn </span>encrypt_in_place_detached(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        nonce: <span class="kw-2">&amp;</span>Nonce,
        associated_data: <span class="kw-2">&amp;</span>[u8],
        buffer: <span class="kw-2">&amp;mut </span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;Tag, Error&gt; {
        Cipher::&lt;Aes&gt;::new(<span class="kw-2">&amp;</span><span class="self">self</span>.key_generating_key, nonce)
            .encrypt_in_place_detached(associated_data, buffer)
    }

    <span class="kw">fn </span>decrypt_in_place_detached(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        nonce: <span class="kw-2">&amp;</span>Nonce,
        associated_data: <span class="kw-2">&amp;</span>[u8],
        buffer: <span class="kw-2">&amp;mut </span>[u8],
        tag: <span class="kw-2">&amp;</span>Tag,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        Cipher::&lt;Aes&gt;::new(<span class="kw-2">&amp;</span><span class="self">self</span>.key_generating_key, nonce).decrypt_in_place_detached(
            associated_data,
            buffer,
            tag,
        )
    }
}

<span class="doccomment">/// AES-GCM-SIV: Misuse-Resistant Authenticated Encryption Cipher (RFC 8452)
</span><span class="kw">struct </span>Cipher&lt;Aes&gt;
<span class="kw">where
    </span>Aes: BlockCipher&lt;BlockSize = U16&gt; + BlockEncrypt,
    Aes::ParBlocks: ArrayLength&lt;Block&lt;Aes&gt;&gt;,
{
    <span class="doccomment">/// Encryption cipher
    </span>enc_cipher: Aes,

    <span class="doccomment">/// POLYVAL universal hash
    </span>polyval: Polyval,

    <span class="doccomment">/// Nonce
    </span>nonce: Nonce,
}

<span class="kw">impl</span>&lt;Aes&gt; Cipher&lt;Aes&gt;
<span class="kw">where
    </span>Aes: NewBlockCipher + BlockCipher&lt;BlockSize = U16&gt; + BlockEncrypt,
    Aes::ParBlocks: ArrayLength&lt;Block&lt;Aes&gt;&gt;,
{
    <span class="doccomment">/// Initialize AES-GCM-SIV, deriving per-nonce message-authentication and
    /// message-encryption keys.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>new(key_generating_key: <span class="kw-2">&amp;</span>Aes, nonce: <span class="kw-2">&amp;</span>Nonce) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>mac_key = polyval::Key::default();
        <span class="kw">let </span><span class="kw-2">mut </span>enc_key = Key::default();
        <span class="kw">let </span><span class="kw-2">mut </span>block = cipher::Block::&lt;Aes&gt;::default();
        <span class="kw">let </span><span class="kw-2">mut </span>counter = <span class="number">0u32</span>;

        <span class="comment">// Derive subkeys from the master key-generating-key in counter mode.
        //
        // From RFC 8452 Section 4:
        // &lt;https://tools.ietf.org/html/rfc8452#section-4&gt;
        //
        // &gt; The message-authentication key is 128 bit, and the message-encryption
        // &gt; key is either 128 (for AES-128) or 256 bit (for AES-256).
        // &gt;
        // &gt; These keys are generated by encrypting a series of plaintext blocks
        // &gt; that contain a 32-bit, little-endian counter followed by the nonce,
        // &gt; and then discarding the second half of the resulting ciphertext.  In
        // &gt; the AES-128 case, 128 + 128 = 256 bits of key material need to be
        // &gt; generated, and, since encrypting each block yields 64 bits after
        // &gt; discarding half, four blocks need to be encrypted.  The counter
        // &gt; values for these blocks are 0, 1, 2, and 3.  For AES-256, six blocks
        // &gt; are needed in total, with counter values 0 through 5 (inclusive).
        </span><span class="kw">for </span>derived_key <span class="kw">in </span><span class="kw-2">&amp;mut </span>[mac_key.as_mut_slice(), enc_key.as_mut_slice()] {
            <span class="kw">for </span>chunk <span class="kw">in </span>derived_key.chunks_mut(<span class="number">8</span>) {
                block[..<span class="number">4</span>].copy_from_slice(<span class="kw-2">&amp;</span>counter.to_le_bytes());
                block[<span class="number">4</span>..].copy_from_slice(nonce.as_slice());

                key_generating_key.encrypt_block(<span class="kw-2">&amp;mut </span>block);
                chunk.copy_from_slice(<span class="kw-2">&amp;</span>block.as_slice()[..<span class="number">8</span>]);

                counter += <span class="number">1</span>;
            }
        }

        <span class="kw">let </span>result = <span class="self">Self </span>{
            enc_cipher: Aes::new(<span class="kw-2">&amp;</span>enc_key),
            polyval: Polyval::new(<span class="kw-2">&amp;</span>mac_key),
            nonce: <span class="kw-2">*</span>nonce,
        };

        <span class="comment">// Zeroize all intermediate buffers
        // TODO(tarcieri): use `Zeroizing` when const generics land
        </span>mac_key.as_mut_slice().zeroize();
        enc_key.as_mut_slice().zeroize();
        block.as_mut_slice().zeroize();

        result
    }

    <span class="doccomment">/// Encrypt the given message in-place, returning the authentication tag
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>encrypt_in_place_detached(
        <span class="kw-2">mut </span><span class="self">self</span>,
        associated_data: <span class="kw-2">&amp;</span>[u8],
        buffer: <span class="kw-2">&amp;mut </span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;Tag, Error&gt; {
        <span class="kw">if </span>buffer.len() <span class="kw">as </span>u64 &gt; P_MAX || associated_data.len() <span class="kw">as </span>u64 &gt; A_MAX {
            <span class="kw">return </span><span class="prelude-val">Err</span>(Error);
        }

        <span class="kw">let </span>tag = <span class="self">self</span>.compute_tag(associated_data, buffer);
        init_ctr(<span class="kw-2">&amp;</span><span class="self">self</span>.enc_cipher, <span class="kw-2">&amp;</span>tag).apply_keystream(buffer);
        <span class="prelude-val">Ok</span>(tag)
    }

    <span class="doccomment">/// Decrypt the given message, first authenticating ciphertext integrity
    /// and returning an error if it's been tampered with.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>decrypt_in_place_detached(
        <span class="kw-2">mut </span><span class="self">self</span>,
        associated_data: <span class="kw-2">&amp;</span>[u8],
        buffer: <span class="kw-2">&amp;mut </span>[u8],
        tag: <span class="kw-2">&amp;</span>Tag,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="kw">if </span>buffer.len() <span class="kw">as </span>u64 &gt; C_MAX || associated_data.len() <span class="kw">as </span>u64 &gt; A_MAX {
            <span class="kw">return </span><span class="prelude-val">Err</span>(Error);
        }

        <span class="self">self</span>.polyval.update_padded(associated_data);
        <span class="kw">let </span><span class="kw-2">mut </span>ctr = init_ctr(<span class="kw-2">&amp;</span><span class="self">self</span>.enc_cipher, tag);

        <span class="kw">for </span>chunk <span class="kw">in </span>buffer.chunks_mut(Aes::BlockSize::to_usize() * Aes::ParBlocks::to_usize()) {
            ctr.apply_keystream(chunk);
            <span class="self">self</span>.polyval.update_padded(chunk);
        }

        <span class="kw">let </span>expected_tag = <span class="self">self</span>.finish_tag(associated_data.len(), buffer.len());

        <span class="kw">use </span>subtle::ConstantTimeEq;
        <span class="kw">if </span>expected_tag.ct_eq(<span class="kw-2">&amp;</span>tag).unwrap_u8() == <span class="number">1 </span>{
            <span class="prelude-val">Ok</span>(())
        } <span class="kw">else </span>{
            <span class="comment">// On MAC verify failure, re-encrypt the plaintext buffer to
            // prevent accidental exposure.
            </span>init_ctr(<span class="kw-2">&amp;</span><span class="self">self</span>.enc_cipher, tag).apply_keystream(buffer);
            <span class="prelude-val">Err</span>(Error)
        }
    }

    <span class="doccomment">/// Authenticate the given plaintext and associated data using POLYVAL
    </span><span class="kw">fn </span>compute_tag(<span class="kw-2">&amp;mut </span><span class="self">self</span>, associated_data: <span class="kw-2">&amp;</span>[u8], buffer: <span class="kw-2">&amp;mut </span>[u8]) -&gt; Tag {
        <span class="self">self</span>.polyval.update_padded(associated_data);
        <span class="self">self</span>.polyval.update_padded(buffer);
        <span class="self">self</span>.finish_tag(associated_data.len(), buffer.len())
    }

    <span class="doccomment">/// Finish computing POLYVAL tag for AAD and buffer of the given length
    </span><span class="kw">fn </span>finish_tag(<span class="kw-2">&amp;mut </span><span class="self">self</span>, associated_data_len: usize, buffer_len: usize) -&gt; Tag {
        <span class="kw">let </span>associated_data_bits = (associated_data_len <span class="kw">as </span>u64) * <span class="number">8</span>;
        <span class="kw">let </span>buffer_bits = (buffer_len <span class="kw">as </span>u64) * <span class="number">8</span>;

        <span class="kw">let </span><span class="kw-2">mut </span>block = polyval::Block::default();
        block[..<span class="number">8</span>].copy_from_slice(<span class="kw-2">&amp;</span>associated_data_bits.to_le_bytes());
        block[<span class="number">8</span>..].copy_from_slice(<span class="kw-2">&amp;</span>buffer_bits.to_le_bytes());
        <span class="self">self</span>.polyval.update(<span class="kw-2">&amp;</span>block);

        <span class="kw">let </span><span class="kw-2">mut </span>tag = <span class="self">self</span>.polyval.finalize_reset().into_bytes();

        <span class="comment">// XOR the nonce into the resulting tag
        </span><span class="kw">for </span>(i, byte) <span class="kw">in </span>tag[..<span class="number">12</span>].iter_mut().enumerate() {
            <span class="kw-2">*</span>byte ^= <span class="self">self</span>.nonce[i];
        }

        <span class="comment">// Clear the highest bit
        </span>tag[<span class="number">15</span>] &amp;= <span class="number">0x7f</span>;

        <span class="self">self</span>.enc_cipher.encrypt_block(<span class="kw-2">&amp;mut </span>tag);
        tag
    }
}

<span class="doccomment">/// Initialize counter mode.
///
/// From RFC 8452 Section 4:
/// &lt;https://tools.ietf.org/html/rfc8452#section-4&gt;
///
/// &gt; The initial counter block is the tag with the most significant bit
/// &gt; of the last byte set to one.
</span><span class="kw">fn </span>init_ctr&lt;Aes&gt;(cipher: Aes, nonce: <span class="kw-2">&amp;</span>cipher::Block&lt;Aes&gt;) -&gt; Ctr32LE&lt;Aes&gt;
<span class="kw">where
    </span>Aes: BlockCipher&lt;BlockSize = U16&gt; + BlockEncrypt,
    Aes::ParBlocks: ArrayLength&lt;Block&lt;Aes&gt;&gt;,
{
    <span class="kw">let </span><span class="kw-2">mut </span>counter_block = <span class="kw-2">*</span>nonce;
    counter_block[<span class="number">15</span>] |= <span class="number">0x80</span>;
    Ctr32LE::from_block_cipher(cipher, <span class="kw-2">&amp;</span>counter_block)
}
</code></pre></div></section></main></body></html>