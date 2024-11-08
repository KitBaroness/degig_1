<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ed25519-dalek-bip32-0.2.0/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="ed25519_dalek_bip32" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../ed25519_dalek_bip32/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#418" id="418">418</a>
<a href="#419" id="419">419</a>
<a href="#420" id="420">420</a>
<a href="#421" id="421">421</a>
<a href="#422" id="422">422</a>
<a href="#423" id="423">423</a>
<a href="#424" id="424">424</a>
<a href="#425" id="425">425</a>
<a href="#426" id="426">426</a>
<a href="#427" id="427">427</a>
<a href="#428" id="428">428</a>
<a href="#429" id="429">429</a>
<a href="#430" id="430">430</a>
<a href="#431" id="431">431</a>
<a href="#432" id="432">432</a>
<a href="#433" id="433">433</a>
<a href="#434" id="434">434</a>
<a href="#435" id="435">435</a>
<a href="#436" id="436">436</a>
<a href="#437" id="437">437</a>
<a href="#438" id="438">438</a>
<a href="#439" id="439">439</a>
<a href="#440" id="440">440</a>
<a href="#441" id="441">441</a>
<a href="#442" id="442">442</a>
<a href="#443" id="443">443</a>
<a href="#444" id="444">444</a>
<a href="#445" id="445">445</a>
<a href="#446" id="446">446</a>
<a href="#447" id="447">447</a>
<a href="#448" id="448">448</a>
<a href="#449" id="449">449</a>
<a href="#450" id="450">450</a>
<a href="#451" id="451">451</a>
<a href="#452" id="452">452</a>
<a href="#453" id="453">453</a>
<a href="#454" id="454">454</a>
<a href="#455" id="455">455</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! A simple BIP32 implementation for ed25519 public keys. Although there exists [another very good
//! library that does this](https://docs.rs/ed25519-bip32), this library preserves 32 byte secret
//! keys and doesn't allow for extended public keys or "normal" child indexes, so that it can be as
//! close to the BIP32 specifications as possible, allowing for compatibility with libraries like
//! `trezor-crypto`

</span><span class="attr">#![cfg_attr(not(feature = <span class="string">"std"</span>), no_std)]

</span><span class="kw">pub extern crate </span>derivation_path;
<span class="kw">pub extern crate </span>ed25519_dalek;

<span class="kw">pub use </span>derivation_path::{ChildIndex, DerivationPath};
<span class="kw">pub use </span>ed25519_dalek::{PublicKey, SecretKey};

<span class="kw">use </span>core::fmt;
<span class="kw">use </span>hmac::{Hmac, Mac};
<span class="kw">use </span>sha2::Sha512;

<span class="kw">const </span>ED25519_BIP32_NAME: <span class="kw-2">&amp;</span>str = <span class="string">"ed25519 seed"</span>;

<span class="doccomment">/// Errors thrown while deriving secret keys
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub enum </span>Error {
    Ed25519,
    ExpectedHardenedIndex(ChildIndex),
}

<span class="kw">impl </span>fmt::Display <span class="kw">for </span>Error {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        <span class="kw">match </span><span class="self">self </span>{
            <span class="self">Self</span>::Ed25519 =&gt; f.write_str(<span class="string">"ed25519 error"</span>),
            <span class="self">Self</span>::ExpectedHardenedIndex(index) =&gt; {
                f.write_fmt(<span class="macro">format_args!</span>(<span class="string">"expected hardened child index: {}"</span>, index))
            }
        }
    }
}

<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">impl </span>std::error::Error <span class="kw">for </span>Error {}

<span class="doccomment">/// An expanded secret key with chain code and meta data
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>ExtendedSecretKey {
    <span class="doccomment">/// How many derivations this key is from the root (0 for root)
    </span><span class="kw">pub </span>depth: u8,
    <span class="doccomment">/// Child index of the key used to derive from parent (`Normal(0)` for root)
    </span><span class="kw">pub </span>child_index: ChildIndex,
    <span class="doccomment">/// Secret Key
    </span><span class="kw">pub </span>secret_key: SecretKey,
    <span class="doccomment">/// Chain code
    </span><span class="kw">pub </span>chain_code: [u8; <span class="number">32</span>],
}

<span class="kw">type </span>HmacSha512 = Hmac&lt;Sha512&gt;;

<span class="doccomment">/// A convenience wrapper for a [`core::result::Result`] with an [`Error`]
</span><span class="kw">pub type </span><span class="prelude-ty">Result</span>&lt;T, E = Error&gt; = core::result::Result&lt;T, E&gt;;

<span class="kw">impl </span>ExtendedSecretKey {
    <span class="doccomment">/// Create a new extended secret key from a seed
    </span><span class="kw">pub fn </span>from_seed(seed: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>mac = HmacSha512::new_from_slice(ED25519_BIP32_NAME.as_ref()).unwrap();
        mac.update(seed);
        <span class="kw">let </span>bytes = mac.finalize().into_bytes();

        <span class="kw">let </span>secret_key = SecretKey::from_bytes(<span class="kw-2">&amp;</span>bytes[..<span class="number">32</span>])<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>chain_code = [<span class="number">0</span>; <span class="number">32</span>];
        chain_code.copy_from_slice(<span class="kw-2">&amp;</span>bytes[<span class="number">32</span>..]);

        <span class="prelude-val">Ok</span>(<span class="self">Self </span>{
            depth: <span class="number">0</span>,
            child_index: ChildIndex::Normal(<span class="number">0</span>),
            secret_key,
            chain_code,
        })
    }

    <span class="doccomment">/// Derive an extended secret key fom the current using a derivation path
    </span><span class="kw">pub fn </span>derive&lt;P: AsRef&lt;[ChildIndex]&gt;&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, path: <span class="kw-2">&amp;</span>P) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>path = path.as_ref().iter();
        <span class="kw">let </span><span class="kw-2">mut </span>next = <span class="kw">match </span>path.next() {
            <span class="prelude-val">Some</span>(index) =&gt; <span class="self">self</span>.derive_child(<span class="kw-2">*</span>index)<span class="question-mark">?</span>,
            <span class="prelude-val">None </span>=&gt; <span class="self">self</span>.clone(),
        };
        <span class="kw">for </span>index <span class="kw">in </span>path {
            next = next.derive_child(<span class="kw-2">*</span>index)<span class="question-mark">?</span>;
        }
        <span class="prelude-val">Ok</span>(next)
    }

    <span class="doccomment">/// Derive a child extended secret key with an index
    </span><span class="kw">pub fn </span>derive_child(<span class="kw-2">&amp;</span><span class="self">self</span>, index: ChildIndex) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>&gt; {
        <span class="kw">if </span>index.is_normal() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(Error::ExpectedHardenedIndex(index));
        }

        <span class="kw">let </span><span class="kw-2">mut </span>mac = HmacSha512::new_from_slice(<span class="kw-2">&amp;</span><span class="self">self</span>.chain_code).unwrap();
        mac.update(<span class="kw-2">&amp;</span>[<span class="number">0u8</span>]);
        mac.update(<span class="self">self</span>.secret_key.to_bytes().as_ref());
        mac.update(index.to_bits().to_be_bytes().as_ref());
        <span class="kw">let </span>bytes = mac.finalize().into_bytes();

        <span class="kw">let </span>secret_key = SecretKey::from_bytes(<span class="kw-2">&amp;</span>bytes[..<span class="number">32</span>])<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>chain_code = [<span class="number">0</span>; <span class="number">32</span>];
        chain_code.copy_from_slice(<span class="kw-2">&amp;</span>bytes[<span class="number">32</span>..]);

        <span class="prelude-val">Ok</span>(<span class="self">Self </span>{
            depth: <span class="self">self</span>.depth + <span class="number">1</span>,
            child_index: index,
            secret_key,
            chain_code,
        })
    }

    <span class="doccomment">/// Get the associated public key
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>public_key(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; PublicKey {
        PublicKey::from(<span class="kw-2">&amp;</span><span class="self">self</span>.secret_key)
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>clone(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            depth: <span class="self">self</span>.depth,
            child_index: <span class="self">self</span>.child_index,
            secret_key: SecretKey::from_bytes(<span class="kw-2">&amp;</span><span class="self">self</span>.secret_key.to_bytes()).unwrap(),
            chain_code: <span class="self">self</span>.chain_code,
        }
    }
}

<span class="kw">impl </span>From&lt;ed25519_dalek::SignatureError&gt; <span class="kw">for </span>Error {
    <span class="kw">fn </span>from(<span class="kw">_</span>: ed25519_dalek::SignatureError) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::Ed25519
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="kw">extern crate </span>alloc;
    <span class="kw">use </span>alloc::vec::Vec;

    <span class="kw">fn </span>hex_str(string: <span class="kw-2">&amp;</span>str) -&gt; Vec&lt;u8&gt; {
        hex::decode(string).unwrap()
    }

    <span class="kw">fn </span>root(seed: <span class="kw-2">&amp;</span>str) -&gt; ExtendedSecretKey {
        ExtendedSecretKey::from_seed(<span class="kw-2">&amp;</span>hex_str(seed)).unwrap()
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>derivation_path() {
        <span class="kw">let </span>vector1_path: DerivationPath = <span class="string">"m/0'/1'/2'/2'/1000000000'"</span>.parse().unwrap();
        <span class="kw">let </span>vector2_path: DerivationPath = <span class="string">"m/0'/2147483647'/1'/2147483646'/2'"</span>.parse().unwrap();

        <span class="kw">let </span>node = root(<span class="string">"000102030405060708090a0b0c0d0e0f"</span>)
            .derive(<span class="kw-2">&amp;</span>vector1_path)
            .unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">5</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">1000000000</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"68789923a0cac2cd5a29172a475fe9e0fb14cd6adb5ad98a3fa70333e7afa230"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"8f94d394a8e8fd6b1bc2f3f49f5c47e385281d5c17e65324b0f62483e37e8793"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"3c24da049451555d51a7014a37337aa4e12d41e485abccfa46b47dfb2af54b7a"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="kw">let </span>node = root(<span class="string">"fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542"</span>).derive(<span class="kw-2">&amp;</span>vector2_path).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">5</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">2</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"5d70af781f3a37b829f0d060924d5e960bdc02e85423494afc0b1a41bbe196d4"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"551d333177df541ad876a60ea71f00447931c0a9da16f227c11ea080d7391b8d"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"47150c75db263559a70d5778bf36abbab30fb061ad69f69ece61a72b0cfa4fc0"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>normal_errs() {
        <span class="kw">let </span>node = root(<span class="string">"000102030405060708090a0b0c0d0e0f"</span>);

        <span class="kw">let </span>res = node.derive_child(ChildIndex::Normal(<span class="number">0</span>));
        <span class="macro">assert!</span>(<span class="macro">matches!</span>(
            res,
            <span class="prelude-val">Err</span>(Error::ExpectedHardenedIndex(ChildIndex::Normal(<span class="number">0</span>)))
        ));

        <span class="kw">let </span>res = node.derive_child(ChildIndex::Normal(<span class="number">100000</span>));
        <span class="macro">assert!</span>(<span class="macro">matches!</span>(
            res,
            <span class="prelude-val">Err</span>(Error::ExpectedHardenedIndex(ChildIndex::Normal(<span class="number">100000</span>)))
        ));

        <span class="kw">let </span>soft_path: DerivationPath = <span class="string">"m/0'/1'/2'/3/4'"</span>.parse().unwrap();
        <span class="kw">let </span>res = node.derive(<span class="kw-2">&amp;</span>soft_path);
        <span class="macro">assert!</span>(<span class="macro">matches!</span>(
            res,
            <span class="prelude-val">Err</span>(Error::ExpectedHardenedIndex(ChildIndex::Normal(<span class="number">3</span>)))
        ));
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>vector1() {
        <span class="comment">// Chain m
        </span><span class="kw">let </span>node = root(<span class="string">"000102030405060708090a0b0c0d0e0f"</span>);
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Normal(<span class="number">0</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"90046a93de5380a72b5e45010748567d5ea02bbf6522f979e05c0d8d8ca9fffb"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"2b4be7f19ee27bbf30c667b642d5f4aa69fd169872f8fc3059c08ebae2eb19e7"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"a4b2856bfec510abab89753fac1ac0e1112364e7d250545963f135f2a33188ed"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="comment">// Chain m/0'
        </span><span class="kw">let </span>node = node.derive_child(ChildIndex::Hardened(<span class="number">0</span>)).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">1</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">0</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"8b59aa11380b624e81507a27fedda59fea6d0b779a778918a2fd3590e16e9c69"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"68e0fe46dfb67e368c75379acec591dad19df3cde26e63b93a8e704f1dade7a3"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"8c8a13df77a28f3445213a0f432fde644acaa215fc72dcdf300d5efaa85d350c"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="comment">// Chain m/0'/1'
        </span><span class="kw">let </span>node = node.derive_child(ChildIndex::Hardened(<span class="number">1</span>)).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">2</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">1</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"a320425f77d1b5c2505a6b1b27382b37368ee640e3557c315416801243552f14"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"b1d0bad404bf35da785a64ca1ac54b2617211d2777696fbffaf208f746ae84f2"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"1932a5270f335bed617d5b935c80aedb1a35bd9fc1e31acafd5372c30f5c1187"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="comment">// Chain m/0'/1'/2'
        </span><span class="kw">let </span>node = node.derive_child(ChildIndex::Hardened(<span class="number">2</span>)).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">3</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">2</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"2e69929e00b5ab250f49c3fb1c12f252de4fed2c1db88387094a0f8c4c9ccd6c"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"92a5b23c0b8a99e37d07df3fb9966917f5d06e02ddbd909c7e184371463e9fc9"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"ae98736566d30ed0e9d2f4486a64bc95740d89c7db33f52121f8ea8f76ff0fc1"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="comment">// Chain m/0'/1'/2'/2'
        </span><span class="kw">let </span>node = node.derive_child(ChildIndex::Hardened(<span class="number">2</span>)).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">4</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">2</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"8f6d87f93d750e0efccda017d662a1b31a266e4a6f5993b15f5c1f07f74dd5cc"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"30d1dc7e5fc04c31219ab25a27ae00b50f6fd66622f6e9c913253d6511d1e662"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"8abae2d66361c879b900d204ad2cc4984fa2aa344dd7ddc46007329ac76c429c"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="comment">// Chain m/0'/1'/2'/2'/1000000000'
        </span><span class="kw">let </span>node = node.derive_child(ChildIndex::Hardened(<span class="number">1000000000</span>)).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">5</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">1000000000</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"68789923a0cac2cd5a29172a475fe9e0fb14cd6adb5ad98a3fa70333e7afa230"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"8f94d394a8e8fd6b1bc2f3f49f5c47e385281d5c17e65324b0f62483e37e8793"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"3c24da049451555d51a7014a37337aa4e12d41e485abccfa46b47dfb2af54b7a"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>vector2() {
        <span class="comment">// Chain m
        </span><span class="kw">let </span>node = root(<span class="string">"fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542"</span>);
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Normal(<span class="number">0</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"ef70a74db9c3a5af931b5fe73ed8e1a53464133654fd55e7a66f8570b8e33c3b"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"171cb88b1b3c1db25add599712e36245d75bc65a1a5c9e18d76f9f2b1eab4012"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"8fe9693f8fa62a4305a140b9764c5ee01e455963744fe18204b4fb948249308a"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="comment">// Chain m/0'
        </span><span class="kw">let </span>node = node.derive_child(ChildIndex::Hardened(<span class="number">0</span>)).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">1</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">0</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"0b78a3226f915c082bf118f83618a618ab6dec793752624cbeb622acb562862d"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"1559eb2bbec5790b0c65d8693e4d0875b1747f4970ae8b650486ed7470845635"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"86fab68dcb57aa196c77c5f264f215a112c22a912c10d123b0d03c3c28ef1037"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="comment">// Chain m/0'/2147483647'
        </span><span class="kw">let </span>node = node.derive_child(ChildIndex::Hardened(<span class="number">2147483647</span>)).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">2</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">2147483647</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"138f0b2551bcafeca6ff2aa88ba8ed0ed8de070841f0c4ef0165df8181eaad7f"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"ea4f5bfe8694d8bb74b7b59404632fd5968b774ed545e810de9c32a4fb4192f4"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"5ba3b9ac6e90e83effcd25ac4e58a1365a9e35a3d3ae5eb07b9e4d90bcf7506d"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="comment">// Chain m/0'/2147483647'/1'
        </span><span class="kw">let </span>node = node.derive_child(ChildIndex::Hardened(<span class="number">1</span>)).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">3</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">1</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"73bd9fff1cfbde33a1b846c27085f711c0fe2d66fd32e139d3ebc28e5a4a6b90"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"3757c7577170179c7868353ada796c839135b3d30554bbb74a4b1e4a5a58505c"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"2e66aa57069c86cc18249aecf5cb5a9cebbfd6fadeab056254763874a9352b45"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="comment">// Chain m/0'/2147483647'/1'/2147483646'
        </span><span class="kw">let </span>node = node.derive_child(ChildIndex::Hardened(<span class="number">2147483646</span>)).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">4</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">2147483646</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"0902fe8a29f9140480a00ef244bd183e8a13288e4412d8389d140aac1794825a"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"5837736c89570de861ebc173b1086da4f505d4adb387c6a1b1342d5e4ac9ec72"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"e33c0f7d81d843c572275f287498e8d408654fdf0d1e065b84e2e6f157aab09b"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());

        <span class="comment">// Chain m/0'/2147483647'/1'/2147483646'/2'
        </span><span class="kw">let </span>node = node.derive_child(ChildIndex::Hardened(<span class="number">2</span>)).unwrap();
        <span class="macro">assert_eq!</span>(node.depth, <span class="number">5</span>);
        <span class="macro">assert_eq!</span>(node.child_index, ChildIndex::Hardened(<span class="number">2</span>));
        <span class="macro">assert_eq!</span>(
            node.chain_code.as_ref(),
            hex_str(<span class="string">"5d70af781f3a37b829f0d060924d5e960bdc02e85423494afc0b1a41bbe196d4"</span>)
        );
        <span class="kw">let </span>secret = SecretKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"551d333177df541ad876a60ea71f00447931c0a9da16f227c11ea080d7391b8d"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.secret_key.to_bytes(), secret.to_bytes());
        <span class="kw">let </span>public = PublicKey::from_bytes(<span class="kw-2">&amp;</span>hex_str(
            <span class="string">"47150c75db263559a70d5778bf36abbab30fb061ad69f69ece61a72b0cfa4fc0"</span>,
        ))
        .unwrap();
        <span class="macro">assert_eq!</span>(node.public_key().to_bytes(), public.to_bytes());
    }
}
</code></pre></div></section></main></body></html>