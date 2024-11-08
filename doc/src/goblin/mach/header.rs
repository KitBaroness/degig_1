<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/goblin-0.5.4/src/mach/header.rs`."><title>header.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="goblin" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../goblin/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! A header contains minimal architecture information, the binary kind, the number of load commands, as well as an endianness hint

</span><span class="kw">use </span>core::fmt;
<span class="kw">use </span>plain::Plain;
<span class="kw">use </span>scroll::ctx;
<span class="kw">use </span>scroll::ctx::SizeWith;
<span class="kw">use </span>scroll::{Pread, Pwrite, SizeWith};

<span class="kw">use </span><span class="kw">crate</span>::container::{<span class="self">self</span>, Container};
<span class="kw">use </span><span class="kw">crate</span>::error;
<span class="kw">use </span><span class="kw">crate</span>::mach::constants::cputype::{CpuSubType, CpuType, CPU_SUBTYPE_MASK};

<span class="comment">// Constants for the flags field of the mach_header
</span><span class="doccomment">/// the object file has no undefined references
</span><span class="kw">pub const </span>MH_NOUNDEFS: u32 = <span class="number">0x1</span>;
<span class="doccomment">/// the object file is the output of an incremental link against a base file and can't be
/// link edited again
</span><span class="kw">pub const </span>MH_INCRLINK: u32 = <span class="number">0x2</span>;
<span class="doccomment">/// the object file is input for the dynamic linker and can't be staticly link edited again
</span><span class="kw">pub const </span>MH_DYLDLINK: u32 = <span class="number">0x4</span>;
<span class="doccomment">/// the object file's undefined references are bound by the dynamic linker when loaded.
</span><span class="kw">pub const </span>MH_BINDATLOAD: u32 = <span class="number">0x8</span>;
<span class="doccomment">/// the file has its dynamic undefined references prebound.
</span><span class="kw">pub const </span>MH_PREBOUND: u32 = <span class="number">0x10</span>;
<span class="doccomment">/// the file has its read-only and read-write segments split
</span><span class="kw">pub const </span>MH_SPLIT_SEGS: u32 = <span class="number">0x20</span>;
<span class="doccomment">/// the shared library init routine is to be run lazily via catching memory faults to its writeable
/// segments (obsolete)
</span><span class="kw">pub const </span>MH_LAZY_INIT: u32 = <span class="number">0x40</span>;
<span class="doccomment">/// the image is using two-level name space bindings
</span><span class="kw">pub const </span>MH_TWOLEVEL: u32 = <span class="number">0x80</span>;
<span class="doccomment">/// the executable is forcing all images to use flat name space bindings
</span><span class="kw">pub const </span>MH_FORCE_FLAT: u32 = <span class="number">0x100</span>;
<span class="doccomment">/// this umbrella guarantees no multiple defintions of symbols in its sub-images so the
/// two-level namespace hints can always be used.
</span><span class="kw">pub const </span>MH_NOMULTIDEFS: u32 = <span class="number">0x200</span>;
<span class="doccomment">/// do not have dyld notify the prebinding agent about this executable
</span><span class="kw">pub const </span>MH_NOFIXPREBINDING: u32 = <span class="number">0x400</span>;
<span class="doccomment">/// the binary is not prebound but can have its prebinding redone. only used when MH_PREBOUND is not set.
</span><span class="kw">pub const </span>MH_PREBINDABLE: u32 = <span class="number">0x800</span>;
<span class="doccomment">/// indicates that this binary binds to all two-level namespace modules of its dependent libraries.
/// Only used when MH_PREBINDABLE and MH_TWOLEVEL are both set.
</span><span class="kw">pub const </span>MH_ALLMODSBOUND: u32 = <span class="number">0x1000</span>;
<span class="doccomment">/// safe to divide up the sections into sub-sections via symbols for dead code stripping
</span><span class="kw">pub const </span>MH_SUBSECTIONS_VIA_SYMBOLS: u32 = <span class="number">0x2000</span>;
<span class="doccomment">/// the binary has been canonicalized via the unprebind operation
</span><span class="kw">pub const </span>MH_CANONICAL: u32 = <span class="number">0x4000</span>;
<span class="doccomment">/// the final linked image contains external weak symbols
</span><span class="kw">pub const </span>MH_WEAK_DEFINES: u32 = <span class="number">0x8000</span>;
<span class="doccomment">/// the final linked image uses weak symbols
</span><span class="kw">pub const </span>MH_BINDS_TO_WEAK: u32 = <span class="number">0x10000</span>;
<span class="doccomment">/// When this bit is set, all stacks in the task will be given stack execution privilege.
/// Only used in MH_EXECUTE filetypes.
</span><span class="kw">pub const </span>MH_ALLOW_STACK_EXECUTION: u32 = <span class="number">0x20000</span>;
<span class="doccomment">/// When this bit is set, the binary declares it is safe for use in processes with uid zero
</span><span class="kw">pub const </span>MH_ROOT_SAFE: u32 = <span class="number">0x40000</span>;
<span class="doccomment">/// When this bit is set, the binary declares it is safe for use in processes when issetugid() is true
</span><span class="kw">pub const </span>MH_SETUID_SAFE: u32 = <span class="number">0x80000</span>;
<span class="doccomment">/// When this bit is set on a dylib,  the static linker does not need to examine dependent dylibs to
/// see if any are re-exported
</span><span class="kw">pub const </span>MH_NO_REEXPORTED_DYLIBS: u32 = <span class="number">0x0010_0000</span>;
<span class="doccomment">/// When this bit is set, the OS will load the main executable at a random address.
/// Only used in MH_EXECUTE filetypes.
</span><span class="kw">pub const </span>MH_PIE: u32 = <span class="number">0x0020_0000</span>;
<span class="doccomment">/// Only for use on dylibs.  When linking against a dylib that has this bit set, the static linker
/// will automatically not create a LC_LOAD_DYLIB load command to the dylib if no symbols are being
/// referenced from the dylib.
</span><span class="kw">pub const </span>MH_DEAD_STRIPPABLE_DYLIB: u32 = <span class="number">0x0040_0000</span>;
<span class="doccomment">/// Contains a section of type S_THREAD_LOCAL_VARIABLES
</span><span class="kw">pub const </span>MH_HAS_TLV_DESCRIPTORS: u32 = <span class="number">0x0080_0000</span>;
<span class="doccomment">/// When this bit is set, the OS will run the main executable with a non-executable heap even on
/// platforms (e.g. i386) that don't require it. Only used in MH_EXECUTE filetypes.
</span><span class="kw">pub const </span>MH_NO_HEAP_EXECUTION: u32 = <span class="number">0x0100_0000</span>;

<span class="comment">// TODO: verify this number is correct, it was previously 0x02000000 which could indicate a typo/data entry error
</span><span class="doccomment">/// The code was linked for use in an application extension.
</span><span class="kw">pub const </span>MH_APP_EXTENSION_SAFE: u32 = <span class="number">0x0200_0000</span>;

<span class="attr">#[inline(always)]
</span><span class="kw">pub fn </span>flag_to_str(flag: u32) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str {
    <span class="kw">match </span>flag {
        MH_NOUNDEFS =&gt; <span class="string">"MH_NOUNDEFS"</span>,
        MH_INCRLINK =&gt; <span class="string">"MH_INCRLINK"</span>,
        MH_DYLDLINK =&gt; <span class="string">"MH_DYLDLINK"</span>,
        MH_BINDATLOAD =&gt; <span class="string">"MH_BINDATLOAD"</span>,
        MH_PREBOUND =&gt; <span class="string">"MH_PREBOUND"</span>,
        MH_SPLIT_SEGS =&gt; <span class="string">"MH_SPLIT_SEGS"</span>,
        MH_LAZY_INIT =&gt; <span class="string">"MH_LAZY_INIT"</span>,
        MH_TWOLEVEL =&gt; <span class="string">"MH_TWOLEVEL"</span>,
        MH_FORCE_FLAT =&gt; <span class="string">"MH_FORCE_FLAT"</span>,
        MH_NOMULTIDEFS =&gt; <span class="string">"MH_NOMULTIDEFS"</span>,
        MH_NOFIXPREBINDING =&gt; <span class="string">"MH_NOFIXPREBINDING"</span>,
        MH_PREBINDABLE =&gt; <span class="string">"MH_PREBINDABLE "</span>,
        MH_ALLMODSBOUND =&gt; <span class="string">"MH_ALLMODSBOUND"</span>,
        MH_SUBSECTIONS_VIA_SYMBOLS =&gt; <span class="string">"MH_SUBSECTIONS_VIA_SYMBOLS"</span>,
        MH_CANONICAL =&gt; <span class="string">"MH_CANONICAL"</span>,
        MH_WEAK_DEFINES =&gt; <span class="string">"MH_WEAK_DEFINES"</span>,
        MH_BINDS_TO_WEAK =&gt; <span class="string">"MH_BINDS_TO_WEAK"</span>,
        MH_ALLOW_STACK_EXECUTION =&gt; <span class="string">"MH_ALLOW_STACK_EXECUTION"</span>,
        MH_ROOT_SAFE =&gt; <span class="string">"MH_ROOT_SAFE"</span>,
        MH_SETUID_SAFE =&gt; <span class="string">"MH_SETUID_SAFE"</span>,
        MH_NO_REEXPORTED_DYLIBS =&gt; <span class="string">"MH_NO_REEXPORTED_DYLIBS"</span>,
        MH_PIE =&gt; <span class="string">"MH_PIE"</span>,
        MH_DEAD_STRIPPABLE_DYLIB =&gt; <span class="string">"MH_DEAD_STRIPPABLE_DYLIB"</span>,
        MH_HAS_TLV_DESCRIPTORS =&gt; <span class="string">"MH_HAS_TLV_DESCRIPTORS"</span>,
        MH_NO_HEAP_EXECUTION =&gt; <span class="string">"MH_NO_HEAP_EXECUTION"</span>,
        MH_APP_EXTENSION_SAFE =&gt; <span class="string">"MH_APP_EXTENSION_SAFE"</span>,
        <span class="kw">_ </span>=&gt; <span class="string">"UNKNOWN FLAG"</span>,
    }
}

<span class="doccomment">/// Mach Header magic constant
</span><span class="kw">pub const </span>MH_MAGIC: u32 = <span class="number">0xfeed_face</span>;
<span class="kw">pub const </span>MH_CIGAM: u32 = <span class="number">0xcefa_edfe</span>;
<span class="doccomment">/// Mach Header magic constant for 64-bit
</span><span class="kw">pub const </span>MH_MAGIC_64: u32 = <span class="number">0xfeed_facf</span>;
<span class="kw">pub const </span>MH_CIGAM_64: u32 = <span class="number">0xcffa_edfe</span>;

<span class="comment">// Constants for the filetype field of the mach_header
</span><span class="doccomment">/// relocatable object file
</span><span class="kw">pub const </span>MH_OBJECT: u32 = <span class="number">0x1</span>;
<span class="doccomment">/// demand paged executable file
</span><span class="kw">pub const </span>MH_EXECUTE: u32 = <span class="number">0x2</span>;
<span class="doccomment">/// fixed VM shared library file
</span><span class="kw">pub const </span>MH_FVMLIB: u32 = <span class="number">0x3</span>;
<span class="doccomment">/// core file
</span><span class="kw">pub const </span>MH_CORE: u32 = <span class="number">0x4</span>;
<span class="doccomment">/// preloaded executable file
</span><span class="kw">pub const </span>MH_PRELOAD: u32 = <span class="number">0x5</span>;
<span class="doccomment">/// dynamically bound shared library
</span><span class="kw">pub const </span>MH_DYLIB: u32 = <span class="number">0x6</span>;
<span class="doccomment">/// dynamic link editor
</span><span class="kw">pub const </span>MH_DYLINKER: u32 = <span class="number">0x7</span>;
<span class="doccomment">/// dynamically bound bundle file
</span><span class="kw">pub const </span>MH_BUNDLE: u32 = <span class="number">0x8</span>;
<span class="doccomment">/// shared library stub for static linking only, no section contents
</span><span class="kw">pub const </span>MH_DYLIB_STUB: u32 = <span class="number">0x9</span>;
<span class="doccomment">/// companion file with only debug sections
</span><span class="kw">pub const </span>MH_DSYM: u32 = <span class="number">0xa</span>;
<span class="doccomment">/// x86_64 kexts
</span><span class="kw">pub const </span>MH_KEXT_BUNDLE: u32 = <span class="number">0xb</span>;

<span class="kw">pub fn </span>filetype_to_str(filetype: u32) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str {
    <span class="kw">match </span>filetype {
        MH_OBJECT =&gt; <span class="string">"OBJECT"</span>,
        MH_EXECUTE =&gt; <span class="string">"EXECUTE"</span>,
        MH_FVMLIB =&gt; <span class="string">"FVMLIB"</span>,
        MH_CORE =&gt; <span class="string">"CORE"</span>,
        MH_PRELOAD =&gt; <span class="string">"PRELOAD"</span>,
        MH_DYLIB =&gt; <span class="string">"DYLIB"</span>,
        MH_DYLINKER =&gt; <span class="string">"DYLINKER"</span>,
        MH_BUNDLE =&gt; <span class="string">"BUNDLE"</span>,
        MH_DYLIB_STUB =&gt; <span class="string">"DYLIB_STUB"</span>,
        MH_DSYM =&gt; <span class="string">"DSYM"</span>,
        MH_KEXT_BUNDLE =&gt; <span class="string">"KEXT_BUNDLE"</span>,
        <span class="kw">_ </span>=&gt; <span class="string">"UNKNOWN FILETYPE"</span>,
    }
}

<span class="attr">#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Pread, Pwrite, SizeWith)]
</span><span class="doccomment">/// A 32-bit Mach-o header
</span><span class="kw">pub struct </span>Header32 {
    <span class="doccomment">/// mach magic number identifier
    </span><span class="kw">pub </span>magic: u32,
    <span class="doccomment">/// cpu specifier
    </span><span class="kw">pub </span>cputype: u32,
    <span class="doccomment">/// machine specifier
    </span><span class="kw">pub </span>cpusubtype: u32,
    <span class="doccomment">/// type of file
    </span><span class="kw">pub </span>filetype: u32,
    <span class="doccomment">/// number of load commands
    </span><span class="kw">pub </span>ncmds: u32,
    <span class="doccomment">/// the size of all the load commands
    </span><span class="kw">pub </span>sizeofcmds: u32,
    <span class="doccomment">/// flags
    </span><span class="kw">pub </span>flags: u32,
}

<span class="kw">pub const </span>SIZEOF_HEADER_32: usize = <span class="number">0x1c</span>;

<span class="kw">unsafe impl </span>Plain <span class="kw">for </span>Header32 {}

<span class="kw">impl </span>Header32 {
    <span class="doccomment">/// Transmutes the given byte array into the corresponding 32-bit Mach-o header
    </span><span class="kw">pub fn </span>from_bytes(bytes: <span class="kw-2">&amp;</span>[u8; SIZEOF_HEADER_32]) -&gt; <span class="kw-2">&amp;</span><span class="self">Self </span>{
        plain::from_bytes(bytes).unwrap()
    }
    <span class="kw">pub fn </span>size(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        SIZEOF_HEADER_32
    }
}

<span class="attr">#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Pread, Pwrite, SizeWith)]
</span><span class="doccomment">/// A 64-bit Mach-o header
</span><span class="kw">pub struct </span>Header64 {
    <span class="doccomment">/// mach magic number identifier
    </span><span class="kw">pub </span>magic: u32,
    <span class="doccomment">/// cpu specifier
    </span><span class="kw">pub </span>cputype: u32,
    <span class="doccomment">/// machine specifier
    </span><span class="kw">pub </span>cpusubtype: u32,
    <span class="doccomment">/// type of file
    </span><span class="kw">pub </span>filetype: u32,
    <span class="doccomment">/// number of load commands
    </span><span class="kw">pub </span>ncmds: u32,
    <span class="doccomment">/// the size of all the load commands
    </span><span class="kw">pub </span>sizeofcmds: u32,
    <span class="doccomment">/// flags
    </span><span class="kw">pub </span>flags: u32,
    <span class="kw">pub </span>reserved: u32,
}

<span class="kw">unsafe impl </span>Plain <span class="kw">for </span>Header64 {}

<span class="kw">pub const </span>SIZEOF_HEADER_64: usize = <span class="number">32</span>;

<span class="kw">impl </span>Header64 {
    <span class="doccomment">/// Transmutes the given byte array into the corresponding 64-bit Mach-o header
    </span><span class="kw">pub fn </span>from_bytes(bytes: <span class="kw-2">&amp;</span>[u8; SIZEOF_HEADER_64]) -&gt; <span class="kw-2">&amp;</span><span class="self">Self </span>{
        plain::from_bytes(bytes).unwrap()
    }
    <span class="kw">pub fn </span>size(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        SIZEOF_HEADER_64
    }
}

<span class="attr">#[repr(C)]
#[derive(Clone, Copy, Default)]
</span><span class="doccomment">/// Generic sized header
</span><span class="kw">pub struct </span>Header {
    <span class="kw">pub </span>magic: u32,
    <span class="kw">pub </span>cputype: u32,
    <span class="kw">pub </span>cpusubtype: u32,
    <span class="doccomment">/// type of file
    </span><span class="kw">pub </span>filetype: u32,
    <span class="doccomment">/// number of load commands
    </span><span class="kw">pub </span>ncmds: usize,
    <span class="doccomment">/// the size of all the load commands
    </span><span class="kw">pub </span>sizeofcmds: u32,
    <span class="doccomment">/// flags
    </span><span class="kw">pub </span>flags: u32,
    <span class="kw">pub </span>reserved: u32,
}

<span class="kw">impl </span>fmt::Debug <span class="kw">for </span>Header {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        f.debug_struct(<span class="string">"Header"</span>)
            .field(<span class="string">"magic"</span>, <span class="kw-2">&amp;</span><span class="macro">format_args!</span>(<span class="string">"0x{:x}"</span>, <span class="self">self</span>.magic))
            .field(<span class="string">"cputype"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.cputype())
            .field(<span class="string">"cpusubtype"</span>, <span class="kw-2">&amp;</span><span class="macro">format_args!</span>(<span class="string">"0x{:x}"</span>, <span class="self">self</span>.cpusubtype()))
            .field(<span class="string">"filetype"</span>, <span class="kw-2">&amp;</span>filetype_to_str(<span class="self">self</span>.filetype))
            .field(<span class="string">"ncmds"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.ncmds)
            .field(<span class="string">"sizeofcmds"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.sizeofcmds)
            .field(<span class="string">"flags"</span>, <span class="kw-2">&amp;</span><span class="macro">format_args!</span>(<span class="string">"0x{:x}"</span>, <span class="self">self</span>.flags))
            .field(<span class="string">"reserved"</span>, <span class="kw-2">&amp;</span><span class="macro">format_args!</span>(<span class="string">"0x{:x}"</span>, <span class="self">self</span>.reserved))
            .finish()
    }
}

<span class="kw">impl </span>From&lt;Header32&gt; <span class="kw">for </span>Header {
    <span class="kw">fn </span>from(header: Header32) -&gt; <span class="self">Self </span>{
        Header {
            magic: header.magic,
            cputype: header.cputype,
            cpusubtype: header.cpusubtype,
            filetype: header.filetype,
            ncmds: header.ncmds <span class="kw">as </span>usize,
            sizeofcmds: header.sizeofcmds,
            flags: header.flags,
            reserved: <span class="number">0</span>,
        }
    }
}

<span class="kw">impl </span>From&lt;Header&gt; <span class="kw">for </span>Header32 {
    <span class="kw">fn </span>from(header: Header) -&gt; <span class="self">Self </span>{
        Header32 {
            magic: header.magic,
            cputype: header.cputype,
            cpusubtype: header.cpusubtype,
            filetype: header.filetype,
            ncmds: header.ncmds <span class="kw">as </span>u32,
            sizeofcmds: header.sizeofcmds,
            flags: header.flags,
        }
    }
}

<span class="kw">impl </span>From&lt;Header64&gt; <span class="kw">for </span>Header {
    <span class="kw">fn </span>from(header: Header64) -&gt; <span class="self">Self </span>{
        Header {
            magic: header.magic,
            cputype: header.cputype,
            cpusubtype: header.cpusubtype,
            filetype: header.filetype,
            ncmds: header.ncmds <span class="kw">as </span>usize,
            sizeofcmds: header.sizeofcmds,
            flags: header.flags,
            reserved: header.reserved,
        }
    }
}

<span class="kw">impl </span>From&lt;Header&gt; <span class="kw">for </span>Header64 {
    <span class="kw">fn </span>from(header: Header) -&gt; <span class="self">Self </span>{
        Header64 {
            magic: header.magic,
            cputype: header.cputype,
            cpusubtype: header.cpusubtype,
            filetype: header.filetype,
            ncmds: header.ncmds <span class="kw">as </span>u32,
            sizeofcmds: header.sizeofcmds,
            flags: header.flags,
            reserved: header.reserved,
        }
    }
}

<span class="kw">impl </span>Header {
    <span class="kw">pub fn </span>new(ctx: container::Ctx) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>header = Header::default();
        header.magic = <span class="kw">if </span>ctx.is_big() { MH_MAGIC_64 } <span class="kw">else </span>{ MH_MAGIC };
        header
    }
    <span class="doccomment">/// Returns the cpu type
    </span><span class="kw">pub fn </span>cputype(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; CpuType {
        <span class="self">self</span>.cputype
    }
    <span class="doccomment">/// Returns the cpu subtype with the capabilities removed
    </span><span class="kw">pub fn </span>cpusubtype(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; CpuSubType {
        <span class="self">self</span>.cpusubtype &amp; !CPU_SUBTYPE_MASK
    }
    <span class="doccomment">/// Returns the capabilities of the CPU
    </span><span class="kw">pub fn </span>cpu_caps(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u32 {
        (<span class="self">self</span>.cpusubtype &amp; CPU_SUBTYPE_MASK) &gt;&gt; <span class="number">24
    </span>}
}

<span class="kw">impl </span>ctx::SizeWith&lt;container::Ctx&gt; <span class="kw">for </span>Header {
    <span class="kw">fn </span>size_with(container: <span class="kw-2">&amp;</span>container::Ctx) -&gt; usize {
        <span class="kw">match </span>container.container {
            Container::Little =&gt; SIZEOF_HEADER_32,
            Container::Big =&gt; SIZEOF_HEADER_64,
        }
    }
}

<span class="kw">impl </span>ctx::SizeWith&lt;Container&gt; <span class="kw">for </span>Header {
    <span class="kw">fn </span>size_with(container: <span class="kw-2">&amp;</span>Container) -&gt; usize {
        <span class="kw">match </span>container {
            Container::Little =&gt; SIZEOF_HEADER_32,
            Container::Big =&gt; SIZEOF_HEADER_64,
        }
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; ctx::TryFromCtx&lt;<span class="lifetime">'a</span>, container::Ctx&gt; <span class="kw">for </span>Header {
    <span class="kw">type </span>Error = <span class="kw">crate</span>::error::Error;
    <span class="kw">fn </span>try_from_ctx(
        bytes: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8],
        container::Ctx { le, container }: container::Ctx,
    ) -&gt; error::Result&lt;(<span class="self">Self</span>, usize)&gt; {
        <span class="kw">let </span>size = bytes.len();
        <span class="kw">if </span>size &lt; SIZEOF_HEADER_32 || size &lt; SIZEOF_HEADER_64 {
            <span class="kw">let </span>error =
                error::Error::Malformed(<span class="string">"bytes size is smaller than a Mach-o header"</span>.into());
            <span class="prelude-val">Err</span>(error)
        } <span class="kw">else </span>{
            <span class="kw">match </span>container {
                Container::Little =&gt; {
                    <span class="kw">let </span>header = bytes.pread_with::&lt;Header32&gt;(<span class="number">0</span>, le)<span class="question-mark">?</span>;
                    <span class="prelude-val">Ok</span>((Header::from(header), SIZEOF_HEADER_32))
                }
                Container::Big =&gt; {
                    <span class="kw">let </span>header = bytes.pread_with::&lt;Header64&gt;(<span class="number">0</span>, le)<span class="question-mark">?</span>;
                    <span class="prelude-val">Ok</span>((Header::from(header), SIZEOF_HEADER_64))
                }
            }
        }
    }
}

<span class="kw">impl </span>ctx::TryIntoCtx&lt;container::Ctx&gt; <span class="kw">for </span>Header {
    <span class="kw">type </span>Error = <span class="kw">crate</span>::error::Error;
    <span class="kw">fn </span>try_into_ctx(<span class="self">self</span>, bytes: <span class="kw-2">&amp;mut </span>[u8], ctx: container::Ctx) -&gt; error::Result&lt;usize&gt; {
        <span class="kw">match </span>ctx.container {
            Container::Little =&gt; {
                bytes.pwrite_with(Header32::from(<span class="self">self</span>), <span class="number">0</span>, ctx.le)<span class="question-mark">?</span>;
            }
            Container::Big =&gt; {
                bytes.pwrite_with(Header64::from(<span class="self">self</span>), <span class="number">0</span>, ctx.le)<span class="question-mark">?</span>;
            }
        };
        <span class="prelude-val">Ok</span>(Header::size_with(<span class="kw-2">&amp;</span>ctx))
    }
}

<span class="kw">impl </span>ctx::IntoCtx&lt;container::Ctx&gt; <span class="kw">for </span>Header {
    <span class="kw">fn </span>into_ctx(<span class="self">self</span>, bytes: <span class="kw-2">&amp;mut </span>[u8], ctx: container::Ctx) {
        bytes.pwrite_with(<span class="self">self</span>, <span class="number">0</span>, ctx).unwrap();
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;
    <span class="kw">use </span>std::mem::size_of;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_parse_armv7_header() {
        <span class="kw">use </span><span class="kw">crate</span>::mach::constants::cputype::CPU_TYPE_ARM;
        <span class="kw">const </span>CPU_SUBTYPE_ARM_V7: u32 = <span class="number">9</span>;
        <span class="kw">use </span><span class="kw">super</span>::Header;
        <span class="kw">use </span><span class="kw">crate</span>::container::{Container, Ctx, Endian};
        <span class="kw">use </span>scroll::Pread;
        <span class="kw">let </span>bytes = <span class="string">b"\xce\xfa\xed\xfe\x0c\x00\x00\x00\t\x00\x00\x00\n\x00\x00\x00\x06\x00\x00\x00\x8c\r\x00\x00\x00\x00\x00\x00\x1b\x00\x00\x00\x18\x00\x00\x00\xe0\xf7B\xbb\x1c\xf50w\xa6\xf7u\xa3\xba("</span>;
        <span class="kw">let </span>header: Header = bytes
            .pread_with(<span class="number">0</span>, Ctx::new(Container::Little, Endian::Little))
            .unwrap();
        <span class="macro">assert_eq!</span>(header.cputype, CPU_TYPE_ARM);
        <span class="macro">assert_eq!</span>(header.cpusubtype, CPU_SUBTYPE_ARM_V7);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>sizeof_header32() {
        <span class="macro">assert_eq!</span>(SIZEOF_HEADER_32, size_of::&lt;Header32&gt;());
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>sizeof_header64() {
        <span class="macro">assert_eq!</span>(SIZEOF_HEADER_64, size_of::&lt;Header64&gt;());
    }
}
</code></pre></div></section></main></body></html>