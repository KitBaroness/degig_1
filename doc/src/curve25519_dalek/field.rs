<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/curve25519-dalek-3.2.1/src/field.rs`."><title>field.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="curve25519_dalek" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../curve25519_dalek/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#456" id="456">456</a>
<a href="#457" id="457">457</a>
<a href="#458" id="458">458</a>
<a href="#459" id="459">459</a>
<a href="#460" id="460">460</a>
<a href="#461" id="461">461</a>
<a href="#462" id="462">462</a>
<a href="#463" id="463">463</a>
<a href="#464" id="464">464</a>
<a href="#465" id="465">465</a>
<a href="#466" id="466">466</a>
<a href="#467" id="467">467</a>
<a href="#468" id="468">468</a>
<a href="#469" id="469">469</a>
<a href="#470" id="470">470</a>
<a href="#471" id="471">471</a>
<a href="#472" id="472">472</a>
<a href="#473" id="473">473</a>
<a href="#474" id="474">474</a>
<a href="#475" id="475">475</a>
<a href="#476" id="476">476</a>
</pre></div><pre class="rust"><code><span class="comment">// -*- mode: rust; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2021 isis agora lovecruft
// Copyright (c) 2016-2019 Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft &lt;isis@patternsinthevoid.net&gt;
// - Henry de Valence &lt;hdevalence@hdevalence.ca&gt;

</span><span class="doccomment">//! Field arithmetic modulo \\(p = 2\^{255} - 19\\).
//!
//! The `curve25519_dalek::field` module provides a type alias
//! `curve25519_dalek::field::FieldElement` to a field element type
//! defined in the `backend` module; either `FieldElement51` or
//! `FieldElement2625`.
//!
//! Field operations defined in terms of machine
//! operations, such as field multiplication or squaring, are defined in
//! the backend implementation.
//!
//! Field operations defined in terms of other field operations, such as
//! field inversion or square roots, are defined here.

</span><span class="kw">use </span>core::cmp::{Eq, PartialEq};

<span class="kw">use </span>subtle::ConditionallySelectable;
<span class="kw">use </span>subtle::ConditionallyNegatable;
<span class="kw">use </span>subtle::Choice;
<span class="kw">use </span>subtle::ConstantTimeEq;

<span class="kw">use </span>constants;
<span class="kw">use </span>backend;

<span class="attr">#[cfg(feature = <span class="string">"fiat_u32_backend"</span>)]
</span><span class="kw">pub use </span>backend::serial::fiat_u32::field::<span class="kw-2">*</span>;
<span class="attr">#[cfg(feature = <span class="string">"fiat_u64_backend"</span>)]
</span><span class="kw">pub use </span>backend::serial::fiat_u64::field::<span class="kw-2">*</span>;
<span class="doccomment">/// A `FieldElement` represents an element of the field
/// \\( \mathbb Z / (2\^{255} - 19)\\).
///
/// The `FieldElement` type is an alias for one of the platform-specific
/// implementations.
/// Using formally-verified field arithmetic from fiat-crypto
</span><span class="attr">#[cfg(feature = <span class="string">"fiat_u32_backend"</span>)]
</span><span class="kw">pub type </span>FieldElement = backend::serial::fiat_u32::field::FieldElement2625;
<span class="attr">#[cfg(feature = <span class="string">"fiat_u64_backend"</span>)]
</span><span class="kw">pub type </span>FieldElement = backend::serial::fiat_u64::field::FieldElement51;

<span class="attr">#[cfg(feature = <span class="string">"u64_backend"</span>)]
</span><span class="kw">pub use </span>backend::serial::u64::field::<span class="kw-2">*</span>;
<span class="doccomment">/// A `FieldElement` represents an element of the field
/// \\( \mathbb Z / (2\^{255} - 19)\\).
///
/// The `FieldElement` type is an alias for one of the platform-specific
/// implementations.
</span><span class="attr">#[cfg(feature = <span class="string">"u64_backend"</span>)]
</span><span class="kw">pub type </span>FieldElement = backend::serial::u64::field::FieldElement51;

<span class="attr">#[cfg(feature = <span class="string">"u32_backend"</span>)]
</span><span class="kw">pub use </span>backend::serial::u32::field::<span class="kw-2">*</span>;
<span class="doccomment">/// A `FieldElement` represents an element of the field
/// \\( \mathbb Z / (2\^{255} - 19)\\).
///
/// The `FieldElement` type is an alias for one of the platform-specific
/// implementations.
</span><span class="attr">#[cfg(feature = <span class="string">"u32_backend"</span>)]
</span><span class="kw">pub type </span>FieldElement = backend::serial::u32::field::FieldElement2625;

<span class="kw">impl </span>Eq <span class="kw">for </span>FieldElement {}

<span class="kw">impl </span>PartialEq <span class="kw">for </span>FieldElement {
    <span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>FieldElement) -&gt; bool {
        <span class="self">self</span>.ct_eq(other).unwrap_u8() == <span class="number">1u8
    </span>}
}

<span class="kw">impl </span>ConstantTimeEq <span class="kw">for </span>FieldElement {
    <span class="doccomment">/// Test equality between two `FieldElement`s.  Since the
    /// internal representation is not canonical, the field elements
    /// are normalized to wire format before comparison.
    </span><span class="kw">fn </span>ct_eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>FieldElement) -&gt; Choice {
        <span class="self">self</span>.to_bytes().ct_eq(<span class="kw-2">&amp;</span>other.to_bytes())
    }
}

<span class="kw">impl </span>FieldElement {
    <span class="doccomment">/// Determine if this `FieldElement` is negative, in the sense
    /// used in the ed25519 paper: `x` is negative if the low bit is
    /// set.
    ///
    /// # Return
    ///
    /// If negative, return `Choice(1)`.  Otherwise, return `Choice(0)`.
    </span><span class="kw">pub fn </span>is_negative(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Choice {
        <span class="kw">let </span>bytes = <span class="self">self</span>.to_bytes();
        (bytes[<span class="number">0</span>] &amp; <span class="number">1</span>).into()
    }

    <span class="doccomment">/// Determine if this `FieldElement` is zero.
    ///
    /// # Return
    ///
    /// If zero, return `Choice(1)`.  Otherwise, return `Choice(0)`.
    </span><span class="kw">pub fn </span>is_zero(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Choice {
        <span class="kw">let </span>zero = [<span class="number">0u8</span>; <span class="number">32</span>];
        <span class="kw">let </span>bytes = <span class="self">self</span>.to_bytes();

        bytes.ct_eq(<span class="kw-2">&amp;</span>zero)
    }

    <span class="doccomment">/// Compute (self^(2^250-1), self^11), used as a helper function
    /// within invert() and pow22523().
    </span><span class="kw">fn </span>pow22501(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (FieldElement, FieldElement) {
        <span class="comment">// Instead of managing which temporary variables are used
        // for what, we define as many as we need and leave stack
        // allocation to the compiler
        //
        // Each temporary variable t_i is of the form (self)^e_i.
        // Squaring t_i corresponds to multiplying e_i by 2,
        // so the pow2k function shifts e_i left by k places.
        // Multiplying t_i and t_j corresponds to adding e_i + e_j.
        //
        // Temporary t_i                      Nonzero bits of e_i
        //
        </span><span class="kw">let </span>t0  = <span class="self">self</span>.square();           <span class="comment">// 1         e_0 = 2^1
        </span><span class="kw">let </span>t1  = t0.square().square();    <span class="comment">// 3         e_1 = 2^3
        </span><span class="kw">let </span>t2  = <span class="self">self </span>* <span class="kw-2">&amp;</span>t1;              <span class="comment">// 3,0       e_2 = 2^3 + 2^0
        </span><span class="kw">let </span>t3  = <span class="kw-2">&amp;</span>t0 * <span class="kw-2">&amp;</span>t2;               <span class="comment">// 3,1,0
        </span><span class="kw">let </span>t4  = t3.square();             <span class="comment">// 4,2,1
        </span><span class="kw">let </span>t5  = <span class="kw-2">&amp;</span>t2 * <span class="kw-2">&amp;</span>t4;               <span class="comment">// 4,3,2,1,0
        </span><span class="kw">let </span>t6  = t5.pow2k(<span class="number">5</span>);             <span class="comment">// 9,8,7,6,5
        </span><span class="kw">let </span>t7  = <span class="kw-2">&amp;</span>t6 * <span class="kw-2">&amp;</span>t5;               <span class="comment">// 9,8,7,6,5,4,3,2,1,0
        </span><span class="kw">let </span>t8  = t7.pow2k(<span class="number">10</span>);            <span class="comment">// 19..10
        </span><span class="kw">let </span>t9  = <span class="kw-2">&amp;</span>t8 * <span class="kw-2">&amp;</span>t7;               <span class="comment">// 19..0
        </span><span class="kw">let </span>t10 = t9.pow2k(<span class="number">20</span>);            <span class="comment">// 39..20
        </span><span class="kw">let </span>t11 = <span class="kw-2">&amp;</span>t10 * <span class="kw-2">&amp;</span>t9;              <span class="comment">// 39..0
        </span><span class="kw">let </span>t12 = t11.pow2k(<span class="number">10</span>);           <span class="comment">// 49..10
        </span><span class="kw">let </span>t13 = <span class="kw-2">&amp;</span>t12 * <span class="kw-2">&amp;</span>t7;              <span class="comment">// 49..0
        </span><span class="kw">let </span>t14 = t13.pow2k(<span class="number">50</span>);           <span class="comment">// 99..50
        </span><span class="kw">let </span>t15 = <span class="kw-2">&amp;</span>t14 * <span class="kw-2">&amp;</span>t13;             <span class="comment">// 99..0
        </span><span class="kw">let </span>t16 = t15.pow2k(<span class="number">100</span>);          <span class="comment">// 199..100
        </span><span class="kw">let </span>t17 = <span class="kw-2">&amp;</span>t16 * <span class="kw-2">&amp;</span>t15;             <span class="comment">// 199..0
        </span><span class="kw">let </span>t18 = t17.pow2k(<span class="number">50</span>);           <span class="comment">// 249..50
        </span><span class="kw">let </span>t19 = <span class="kw-2">&amp;</span>t18 * <span class="kw-2">&amp;</span>t13;             <span class="comment">// 249..0

        </span>(t19, t3)
    }

    <span class="doccomment">/// Given a slice of public `FieldElements`, replace each with its inverse.
    ///
    /// All input `FieldElements` **MUST** be nonzero.
    </span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">pub fn </span>batch_invert(inputs: <span class="kw-2">&amp;mut </span>[FieldElement]) {
        <span class="comment">// Montgomery’s Trick and Fast Implementation of Masked AES
        // Genelle, Prouff and Quisquater
        // Section 3.2

        </span><span class="kw">let </span>n = inputs.len();
        <span class="kw">let </span><span class="kw-2">mut </span>scratch = <span class="macro">vec!</span>[FieldElement::one(); n];

        <span class="comment">// Keep an accumulator of all of the previous products
        </span><span class="kw">let </span><span class="kw-2">mut </span>acc = FieldElement::one();

        <span class="comment">// Pass through the input vector, recording the previous
        // products in the scratch space
        </span><span class="kw">for </span>(input, scratch) <span class="kw">in </span>inputs.iter().zip(scratch.iter_mut()) {
            <span class="kw-2">*</span>scratch = acc;
            acc = <span class="kw-2">&amp;</span>acc * input;
        }

	<span class="comment">// acc is nonzero iff all inputs are nonzero
        </span><span class="macro">assert_eq!</span>(acc.is_zero().unwrap_u8(), <span class="number">0</span>);

        <span class="comment">// Compute the inverse of all products
        </span>acc = acc.invert();

        <span class="comment">// Pass through the vector backwards to compute the inverses
        // in place
        </span><span class="kw">for </span>(input, scratch) <span class="kw">in </span>inputs.iter_mut().rev().zip(scratch.into_iter().rev()) {
            <span class="kw">let </span>tmp = <span class="kw-2">&amp;</span>acc * input;
            <span class="kw-2">*</span>input = <span class="kw-2">&amp;</span>acc * <span class="kw-2">&amp;</span>scratch;
            acc = tmp;
        }
    }

    <span class="doccomment">/// Given a nonzero field element, compute its inverse.
    ///
    /// The inverse is computed as self^(p-2), since
    /// x^(p-2)x = x^(p-1) = 1 (mod p).
    ///
    /// This function returns zero on input zero.
    </span><span class="kw">pub fn </span>invert(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; FieldElement {
        <span class="comment">// The bits of p-2 = 2^255 -19 -2 are 11010111111...11.
        //
        //                                 nonzero bits of exponent
        </span><span class="kw">let </span>(t19, t3) = <span class="self">self</span>.pow22501();   <span class="comment">// t19: 249..0 ; t3: 3,1,0
        </span><span class="kw">let </span>t20 = t19.pow2k(<span class="number">5</span>);            <span class="comment">// 254..5
        </span><span class="kw">let </span>t21 = <span class="kw-2">&amp;</span>t20 * <span class="kw-2">&amp;</span>t3;              <span class="comment">// 254..5,3,1,0

        </span>t21
    }

    <span class="doccomment">/// Raise this field element to the power (p-5)/8 = 2^252 -3.
    </span><span class="kw">fn </span>pow_p58(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; FieldElement {
        <span class="comment">// The bits of (p-5)/8 are 101111.....11.
        //
        //                                 nonzero bits of exponent
        </span><span class="kw">let </span>(t19, <span class="kw">_</span>) = <span class="self">self</span>.pow22501();    <span class="comment">// 249..0
        </span><span class="kw">let </span>t20 = t19.pow2k(<span class="number">2</span>);            <span class="comment">// 251..2
        </span><span class="kw">let </span>t21 = <span class="self">self </span>* <span class="kw-2">&amp;</span>t20;             <span class="comment">// 251..2,0

        </span>t21
    }

    <span class="doccomment">/// Given `FieldElements` `u` and `v`, compute either `sqrt(u/v)`
    /// or `sqrt(i*u/v)` in constant time.
    ///
    /// This function always returns the nonnegative square root.
    ///
    /// # Return
    ///
    /// - `(Choice(1), +sqrt(u/v))  ` if `v` is nonzero and `u/v` is square;
    /// - `(Choice(1), zero)        ` if `u` is zero;
    /// - `(Choice(0), zero)        ` if `v` is zero and `u` is nonzero;
    /// - `(Choice(0), +sqrt(i*u/v))` if `u/v` is nonsquare (so `i*u/v` is square).
    ///
    </span><span class="kw">pub fn </span>sqrt_ratio_i(u: <span class="kw-2">&amp;</span>FieldElement, v: <span class="kw-2">&amp;</span>FieldElement) -&gt; (Choice, FieldElement) {
        <span class="comment">// Using the same trick as in ed25519 decoding, we merge the
        // inversion, the square root, and the square test as follows.
        //
        // To compute sqrt(α), we can compute β = α^((p+3)/8).
        // Then β^2 = ±α, so multiplying β by sqrt(-1) if necessary
        // gives sqrt(α).
        //
        // To compute 1/sqrt(α), we observe that
        //    1/β = α^(p-1 - (p+3)/8) = α^((7p-11)/8)
        //                            = α^3 * (α^7)^((p-5)/8).
        //
        // We can therefore compute sqrt(u/v) = sqrt(u)/sqrt(v)
        // by first computing
        //    r = u^((p+3)/8) v^(p-1-(p+3)/8)
        //      = u u^((p-5)/8) v^3 (v^7)^((p-5)/8)
        //      = (uv^3) (uv^7)^((p-5)/8).
        //
        // If v is nonzero and u/v is square, then r^2 = ±u/v,
        //                                     so vr^2 = ±u.
        // If vr^2 =  u, then sqrt(u/v) = r.
        // If vr^2 = -u, then sqrt(u/v) = r*sqrt(-1).
        //
        // If v is zero, r is also zero.

        </span><span class="kw">let </span>v3 = <span class="kw-2">&amp;</span>v.square()  * v;
        <span class="kw">let </span>v7 = <span class="kw-2">&amp;</span>v3.square() * v;
        <span class="kw">let </span><span class="kw-2">mut </span>r = <span class="kw-2">&amp;</span>(u * <span class="kw-2">&amp;</span>v3) * <span class="kw-2">&amp;</span>(u * <span class="kw-2">&amp;</span>v7).pow_p58();
        <span class="kw">let </span>check = v * <span class="kw-2">&amp;</span>r.square();

        <span class="kw">let </span>i = <span class="kw-2">&amp;</span>constants::SQRT_M1;

        <span class="kw">let </span>correct_sign_sqrt   = check.ct_eq(        u);
        <span class="kw">let </span>flipped_sign_sqrt   = check.ct_eq(     <span class="kw-2">&amp;</span>(-u));
        <span class="kw">let </span>flipped_sign_sqrt_i = check.ct_eq(<span class="kw-2">&amp;</span>(<span class="kw-2">&amp;</span>(-u)<span class="kw-2">*</span>i));

        <span class="kw">let </span>r_prime = <span class="kw-2">&amp;</span>constants::SQRT_M1 * <span class="kw-2">&amp;</span>r;
        r.conditional_assign(<span class="kw-2">&amp;</span>r_prime, flipped_sign_sqrt | flipped_sign_sqrt_i);

        <span class="comment">// Choose the nonnegative square root.
        </span><span class="kw">let </span>r_is_negative = r.is_negative();
        r.conditional_negate(r_is_negative);

        <span class="kw">let </span>was_nonzero_square = correct_sign_sqrt | flipped_sign_sqrt;

        (was_nonzero_square, r)
    }

    <span class="doccomment">/// Attempt to compute `sqrt(1/self)` in constant time.
    ///
    /// Convenience wrapper around `sqrt_ratio_i`.
    ///
    /// This function always returns the nonnegative square root.
    ///
    /// # Return
    ///
    /// - `(Choice(1), +sqrt(1/self))  ` if `self` is a nonzero square;
    /// - `(Choice(0), zero)           ` if `self` is zero;
    /// - `(Choice(0), +sqrt(i/self))  ` if `self` is a nonzero nonsquare;
    ///
    </span><span class="kw">pub fn </span>invsqrt(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (Choice, FieldElement) {
        FieldElement::sqrt_ratio_i(<span class="kw-2">&amp;</span>FieldElement::one(), <span class="self">self</span>)
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>test {
    <span class="kw">use </span>field::<span class="kw-2">*</span>;
    <span class="kw">use </span>subtle::ConditionallyNegatable;

    <span class="doccomment">/// Random element a of GF(2^255-19), from Sage
    /// a = 1070314506888354081329385823235218444233221\
    ///     2228051251926706380353716438957572
    </span><span class="kw">static </span>A_BYTES: [u8; <span class="number">32</span>] =
        [ <span class="number">0x04</span>, <span class="number">0xfe</span>, <span class="number">0xdf</span>, <span class="number">0x98</span>, <span class="number">0xa7</span>, <span class="number">0xfa</span>, <span class="number">0x0a</span>, <span class="number">0x68</span>,
          <span class="number">0x84</span>, <span class="number">0x92</span>, <span class="number">0xbd</span>, <span class="number">0x59</span>, <span class="number">0x08</span>, <span class="number">0x07</span>, <span class="number">0xa7</span>, <span class="number">0x03</span>,
          <span class="number">0x9e</span>, <span class="number">0xd1</span>, <span class="number">0xf6</span>, <span class="number">0xf2</span>, <span class="number">0xe1</span>, <span class="number">0xd9</span>, <span class="number">0xe2</span>, <span class="number">0xa4</span>,
          <span class="number">0xa4</span>, <span class="number">0x51</span>, <span class="number">0x47</span>, <span class="number">0x36</span>, <span class="number">0xf3</span>, <span class="number">0xc3</span>, <span class="number">0xa9</span>, <span class="number">0x17</span>];

    <span class="doccomment">/// Byte representation of a**2
    </span><span class="kw">static </span>ASQ_BYTES: [u8; <span class="number">32</span>] =
        [ <span class="number">0x75</span>, <span class="number">0x97</span>, <span class="number">0x24</span>, <span class="number">0x9e</span>, <span class="number">0xe6</span>, <span class="number">0x06</span>, <span class="number">0xfe</span>, <span class="number">0xab</span>,
          <span class="number">0x24</span>, <span class="number">0x04</span>, <span class="number">0x56</span>, <span class="number">0x68</span>, <span class="number">0x07</span>, <span class="number">0x91</span>, <span class="number">0x2d</span>, <span class="number">0x5d</span>,
          <span class="number">0x0b</span>, <span class="number">0x0f</span>, <span class="number">0x3f</span>, <span class="number">0x1c</span>, <span class="number">0xb2</span>, <span class="number">0x6e</span>, <span class="number">0xf2</span>, <span class="number">0xe2</span>,
          <span class="number">0x63</span>, <span class="number">0x9c</span>, <span class="number">0x12</span>, <span class="number">0xba</span>, <span class="number">0x73</span>, <span class="number">0x0b</span>, <span class="number">0xe3</span>, <span class="number">0x62</span>];

    <span class="doccomment">/// Byte representation of 1/a
    </span><span class="kw">static </span>AINV_BYTES: [u8; <span class="number">32</span>] =
        [<span class="number">0x96</span>, <span class="number">0x1b</span>, <span class="number">0xcd</span>, <span class="number">0x8d</span>, <span class="number">0x4d</span>, <span class="number">0x5e</span>, <span class="number">0xa2</span>, <span class="number">0x3a</span>,
         <span class="number">0xe9</span>, <span class="number">0x36</span>, <span class="number">0x37</span>, <span class="number">0x93</span>, <span class="number">0xdb</span>, <span class="number">0x7b</span>, <span class="number">0x4d</span>, <span class="number">0x70</span>,
         <span class="number">0xb8</span>, <span class="number">0x0d</span>, <span class="number">0xc0</span>, <span class="number">0x55</span>, <span class="number">0xd0</span>, <span class="number">0x4c</span>, <span class="number">0x1d</span>, <span class="number">0x7b</span>,
         <span class="number">0x90</span>, <span class="number">0x71</span>, <span class="number">0xd8</span>, <span class="number">0xe9</span>, <span class="number">0xb6</span>, <span class="number">0x18</span>, <span class="number">0xe6</span>, <span class="number">0x30</span>];

    <span class="doccomment">/// Byte representation of a^((p-5)/8)
    </span><span class="kw">static </span>AP58_BYTES: [u8; <span class="number">32</span>] =
        [<span class="number">0x6a</span>, <span class="number">0x4f</span>, <span class="number">0x24</span>, <span class="number">0x89</span>, <span class="number">0x1f</span>, <span class="number">0x57</span>, <span class="number">0x60</span>, <span class="number">0x36</span>,
         <span class="number">0xd0</span>, <span class="number">0xbe</span>, <span class="number">0x12</span>, <span class="number">0x3c</span>, <span class="number">0x8f</span>, <span class="number">0xf5</span>, <span class="number">0xb1</span>, <span class="number">0x59</span>,
         <span class="number">0xe0</span>, <span class="number">0xf0</span>, <span class="number">0xb8</span>, <span class="number">0x1b</span>, <span class="number">0x20</span>, <span class="number">0xd2</span>, <span class="number">0xb5</span>, <span class="number">0x1f</span>,
         <span class="number">0x15</span>, <span class="number">0x21</span>, <span class="number">0xf9</span>, <span class="number">0xe3</span>, <span class="number">0xe1</span>, <span class="number">0x61</span>, <span class="number">0x21</span>, <span class="number">0x55</span>];

    <span class="attr">#[test]
    </span><span class="kw">fn </span>a_mul_a_vs_a_squared_constant() {
        <span class="kw">let </span>a = FieldElement::from_bytes(<span class="kw-2">&amp;</span>A_BYTES);
        <span class="kw">let </span>asq = FieldElement::from_bytes(<span class="kw-2">&amp;</span>ASQ_BYTES);
        <span class="macro">assert_eq!</span>(asq, <span class="kw-2">&amp;</span>a * <span class="kw-2">&amp;</span>a);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>a_square_vs_a_squared_constant() {
        <span class="kw">let </span>a = FieldElement::from_bytes(<span class="kw-2">&amp;</span>A_BYTES);
        <span class="kw">let </span>asq = FieldElement::from_bytes(<span class="kw-2">&amp;</span>ASQ_BYTES);
        <span class="macro">assert_eq!</span>(asq, a.square());
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>a_square2_vs_a_squared_constant() {
        <span class="kw">let </span>a = FieldElement::from_bytes(<span class="kw-2">&amp;</span>A_BYTES);
        <span class="kw">let </span>asq = FieldElement::from_bytes(<span class="kw-2">&amp;</span>ASQ_BYTES);
        <span class="macro">assert_eq!</span>(a.square2(), <span class="kw-2">&amp;</span>asq+<span class="kw-2">&amp;</span>asq);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>a_invert_vs_inverse_of_a_constant() {
        <span class="kw">let </span>a    = FieldElement::from_bytes(<span class="kw-2">&amp;</span>A_BYTES);
        <span class="kw">let </span>ainv = FieldElement::from_bytes(<span class="kw-2">&amp;</span>AINV_BYTES);
        <span class="kw">let </span>should_be_inverse = a.invert();
        <span class="macro">assert_eq!</span>(ainv, should_be_inverse);
        <span class="macro">assert_eq!</span>(FieldElement::one(), <span class="kw-2">&amp;</span>a * <span class="kw-2">&amp;</span>should_be_inverse);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>batch_invert_a_matches_nonbatched() {
        <span class="kw">let </span>a    = FieldElement::from_bytes(<span class="kw-2">&amp;</span>A_BYTES);
        <span class="kw">let </span>ap58 = FieldElement::from_bytes(<span class="kw-2">&amp;</span>AP58_BYTES);
        <span class="kw">let </span>asq  = FieldElement::from_bytes(<span class="kw-2">&amp;</span>ASQ_BYTES);
        <span class="kw">let </span>ainv = FieldElement::from_bytes(<span class="kw-2">&amp;</span>AINV_BYTES);
        <span class="kw">let </span>a2   = <span class="kw-2">&amp;</span>a + <span class="kw-2">&amp;</span>a;
        <span class="kw">let </span>a_list = <span class="macro">vec!</span>[a, ap58, asq, ainv, a2];
        <span class="kw">let </span><span class="kw-2">mut </span>ainv_list = a_list.clone();
        FieldElement::batch_invert(<span class="kw-2">&amp;mut </span>ainv_list[..]);
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">5 </span>{
            <span class="macro">assert_eq!</span>(a_list[i].invert(), ainv_list[i]);
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>sqrt_ratio_behavior() {
        <span class="kw">let </span>zero = FieldElement::zero();
        <span class="kw">let </span>one = FieldElement::one();
        <span class="kw">let </span>i = constants::SQRT_M1;
        <span class="kw">let </span>two = <span class="kw-2">&amp;</span>one + <span class="kw-2">&amp;</span>one; <span class="comment">// 2 is nonsquare mod p.
        </span><span class="kw">let </span>four = <span class="kw-2">&amp;</span>two + <span class="kw-2">&amp;</span>two; <span class="comment">// 4 is square mod p.

        // 0/0 should return (1, 0) since u is 0
        </span><span class="kw">let </span>(choice, sqrt) = FieldElement::sqrt_ratio_i(<span class="kw-2">&amp;</span>zero, <span class="kw-2">&amp;</span>zero);
        <span class="macro">assert_eq!</span>(choice.unwrap_u8(), <span class="number">1</span>);
        <span class="macro">assert_eq!</span>(sqrt, zero);
        <span class="macro">assert_eq!</span>(sqrt.is_negative().unwrap_u8(), <span class="number">0</span>);

        <span class="comment">// 1/0 should return (0, 0) since v is 0, u is nonzero
        </span><span class="kw">let </span>(choice, sqrt) = FieldElement::sqrt_ratio_i(<span class="kw-2">&amp;</span>one, <span class="kw-2">&amp;</span>zero);
        <span class="macro">assert_eq!</span>(choice.unwrap_u8(), <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(sqrt, zero);
        <span class="macro">assert_eq!</span>(sqrt.is_negative().unwrap_u8(), <span class="number">0</span>);

        <span class="comment">// 2/1 is nonsquare, so we expect (0, sqrt(i*2))
        </span><span class="kw">let </span>(choice, sqrt) = FieldElement::sqrt_ratio_i(<span class="kw-2">&amp;</span>two, <span class="kw-2">&amp;</span>one);
        <span class="macro">assert_eq!</span>(choice.unwrap_u8(), <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(sqrt.square(), <span class="kw-2">&amp;</span>two * <span class="kw-2">&amp;</span>i);
        <span class="macro">assert_eq!</span>(sqrt.is_negative().unwrap_u8(), <span class="number">0</span>);

        <span class="comment">// 4/1 is square, so we expect (1, sqrt(4))
        </span><span class="kw">let </span>(choice, sqrt) = FieldElement::sqrt_ratio_i(<span class="kw-2">&amp;</span>four, <span class="kw-2">&amp;</span>one);
        <span class="macro">assert_eq!</span>(choice.unwrap_u8(), <span class="number">1</span>);
        <span class="macro">assert_eq!</span>(sqrt.square(), four);
        <span class="macro">assert_eq!</span>(sqrt.is_negative().unwrap_u8(), <span class="number">0</span>);

        <span class="comment">// 1/4 is square, so we expect (1, 1/sqrt(4))
        </span><span class="kw">let </span>(choice, sqrt) = FieldElement::sqrt_ratio_i(<span class="kw-2">&amp;</span>one, <span class="kw-2">&amp;</span>four);
        <span class="macro">assert_eq!</span>(choice.unwrap_u8(), <span class="number">1</span>);
        <span class="macro">assert_eq!</span>(<span class="kw-2">&amp;</span>sqrt.square() * <span class="kw-2">&amp;</span>four, one);
        <span class="macro">assert_eq!</span>(sqrt.is_negative().unwrap_u8(), <span class="number">0</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>a_p58_vs_ap58_constant() {
        <span class="kw">let </span>a    = FieldElement::from_bytes(<span class="kw-2">&amp;</span>A_BYTES);
        <span class="kw">let </span>ap58 = FieldElement::from_bytes(<span class="kw-2">&amp;</span>AP58_BYTES);
        <span class="macro">assert_eq!</span>(ap58, a.pow_p58());
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>equality() {
        <span class="kw">let </span>a    = FieldElement::from_bytes(<span class="kw-2">&amp;</span>A_BYTES);
        <span class="kw">let </span>ainv = FieldElement::from_bytes(<span class="kw-2">&amp;</span>AINV_BYTES);
        <span class="macro">assert!</span>(a == a);
        <span class="macro">assert!</span>(a != ainv);
    }

    <span class="doccomment">/// Notice that the last element has the high bit set, which
    /// should be ignored
    </span><span class="kw">static </span>B_BYTES: [u8;<span class="number">32</span>] =
        [<span class="number">113</span>, <span class="number">191</span>, <span class="number">169</span>, <span class="number">143</span>,  <span class="number">91</span>, <span class="number">234</span>, <span class="number">121</span>,  <span class="number">15</span>,
         <span class="number">241</span>, <span class="number">131</span>, <span class="number">217</span>,  <span class="number">36</span>, <span class="number">230</span>, <span class="number">101</span>,  <span class="number">92</span>, <span class="number">234</span>,
           <span class="number">8</span>, <span class="number">208</span>, <span class="number">170</span>, <span class="number">251</span>,  <span class="number">97</span>, <span class="number">127</span>,  <span class="number">70</span>, <span class="number">210</span>,
          <span class="number">58</span>,  <span class="number">23</span>, <span class="number">166</span>,  <span class="number">87</span>, <span class="number">240</span>, <span class="number">169</span>, <span class="number">184</span>, <span class="number">178</span>];

    <span class="attr">#[test]
    </span><span class="kw">fn </span>from_bytes_highbit_is_ignored() {
        <span class="kw">let </span><span class="kw-2">mut </span>cleared_bytes = B_BYTES;
        cleared_bytes[<span class="number">31</span>] &amp;= <span class="number">127u8</span>;
        <span class="kw">let </span>with_highbit_set    = FieldElement::from_bytes(<span class="kw-2">&amp;</span>B_BYTES);
        <span class="kw">let </span>without_highbit_set = FieldElement::from_bytes(<span class="kw-2">&amp;</span>cleared_bytes);
        <span class="macro">assert_eq!</span>(without_highbit_set, with_highbit_set);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>conditional_negate() {
        <span class="kw">let       </span>one = FieldElement::one();
        <span class="kw">let </span>minus_one = FieldElement::minus_one();
        <span class="kw">let </span><span class="kw-2">mut </span>x = one;
        x.conditional_negate(Choice::from(<span class="number">1</span>));
        <span class="macro">assert_eq!</span>(x, minus_one);
        x.conditional_negate(Choice::from(<span class="number">0</span>));
        <span class="macro">assert_eq!</span>(x, minus_one);
        x.conditional_negate(Choice::from(<span class="number">1</span>));
        <span class="macro">assert_eq!</span>(x, one);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>encoding_is_canonical() {
        <span class="comment">// Encode 1 wrongly as 1 + (2^255 - 19) = 2^255 - 18
        </span><span class="kw">let </span>one_encoded_wrongly_bytes: [u8;<span class="number">32</span>] = [<span class="number">0xee</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0xff</span>, <span class="number">0x7f</span>];
        <span class="comment">// Decode to a field element
        </span><span class="kw">let </span>one = FieldElement::from_bytes(<span class="kw-2">&amp;</span>one_encoded_wrongly_bytes);
        <span class="comment">// .. then check that the encoding is correct
        </span><span class="kw">let </span>one_bytes = one.to_bytes();
        <span class="macro">assert_eq!</span>(one_bytes[<span class="number">0</span>], <span class="number">1</span>);
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">1</span>..<span class="number">32 </span>{
            <span class="macro">assert_eq!</span>(one_bytes[i], <span class="number">0</span>);
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>batch_invert_empty() {
        FieldElement::batch_invert(<span class="kw-2">&amp;mut </span>[]);
    }
}
</code></pre></div></section></main></body></html>