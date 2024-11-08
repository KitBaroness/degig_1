<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/percent-encoding-2.3.1/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="percent_encoding" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../percent_encoding/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#477" id="477">477</a>
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2013-2016 The rust-url developers.
//
// Licensed under the Apache License, Version 2.0 &lt;LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0&gt; or the MIT license
// &lt;LICENSE-MIT or http://opensource.org/licenses/MIT&gt;, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

</span><span class="doccomment">//! URLs use special characters to indicate the parts of the request.
//! For example, a `?` question mark marks the end of a path and the start of a query string.
//! In order for that character to exist inside a path, it needs to be encoded differently.
//!
//! Percent encoding replaces reserved characters with the `%` escape character
//! followed by a byte value as two hexadecimal digits.
//! For example, an ASCII space is replaced with `%20`.
//!
//! When encoding, the set of characters that can (and should, for readability) be left alone
//! depends on the context.
//! The `?` question mark mentioned above is not a separator when used literally
//! inside of a query string, and therefore does not need to be encoded.
//! The [`AsciiSet`] parameter of [`percent_encode`] and [`utf8_percent_encode`]
//! lets callers configure this.
//!
//! This crate deliberately does not provide many different sets.
//! Users should consider in what context the encoded string will be used,
//! read relevant specifications, and define their own set.
//! This is done by using the `add` method of an existing set.
//!
//! # Examples
//!
//! ```
//! use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
//!
//! /// https://url.spec.whatwg.org/#fragment-percent-encode-set
//! const FRAGMENT: &amp;AsciiSet = &amp;CONTROLS.add(b' ').add(b'"').add(b'&lt;').add(b'&gt;').add(b'`');
//!
//! assert_eq!(utf8_percent_encode("foo &lt;bar&gt;", FRAGMENT).to_string(), "foo%20%3Cbar%3E");
//! ```
</span><span class="attr">#![no_std]

</span><span class="comment">// For forwards compatibility
</span><span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">extern crate </span>std <span class="kw">as _</span>;

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">extern crate </span>alloc;

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">use </span>alloc::{
    borrow::{Cow, ToOwned},
    string::String,
    vec::Vec,
};
<span class="kw">use </span>core::{fmt, mem, slice, str};

<span class="doccomment">/// Represents a set of characters or bytes in the ASCII range.
///
/// This is used in [`percent_encode`] and [`utf8_percent_encode`].
/// This is similar to [percent-encode sets](https://url.spec.whatwg.org/#percent-encoded-bytes).
///
/// Use the `add` method of an existing set to define a new set. For example:
///
/// ```
/// use percent_encoding::{AsciiSet, CONTROLS};
///
/// /// https://url.spec.whatwg.org/#fragment-percent-encode-set
/// const FRAGMENT: &amp;AsciiSet = &amp;CONTROLS.add(b' ').add(b'"').add(b'&lt;').add(b'&gt;').add(b'`');
/// ```
</span><span class="kw">pub struct </span>AsciiSet {
    mask: [Chunk; ASCII_RANGE_LEN / BITS_PER_CHUNK],
}

<span class="kw">type </span>Chunk = u32;

<span class="kw">const </span>ASCII_RANGE_LEN: usize = <span class="number">0x80</span>;

<span class="kw">const </span>BITS_PER_CHUNK: usize = <span class="number">8 </span>* mem::size_of::&lt;Chunk&gt;();

<span class="kw">impl </span>AsciiSet {
    <span class="doccomment">/// Called with UTF-8 bytes rather than code points.
    /// Not used for non-ASCII bytes.
    </span><span class="kw">const fn </span>contains(<span class="kw-2">&amp;</span><span class="self">self</span>, byte: u8) -&gt; bool {
        <span class="kw">let </span>chunk = <span class="self">self</span>.mask[byte <span class="kw">as </span>usize / BITS_PER_CHUNK];
        <span class="kw">let </span>mask = <span class="number">1 </span>&lt;&lt; (byte <span class="kw">as </span>usize % BITS_PER_CHUNK);
        (chunk &amp; mask) != <span class="number">0
    </span>}

    <span class="kw">fn </span>should_percent_encode(<span class="kw-2">&amp;</span><span class="self">self</span>, byte: u8) -&gt; bool {
        !byte.is_ascii() || <span class="self">self</span>.contains(byte)
    }

    <span class="kw">pub const fn </span>add(<span class="kw-2">&amp;</span><span class="self">self</span>, byte: u8) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>mask = <span class="self">self</span>.mask;
        mask[byte <span class="kw">as </span>usize / BITS_PER_CHUNK] |= <span class="number">1 </span>&lt;&lt; (byte <span class="kw">as </span>usize % BITS_PER_CHUNK);
        AsciiSet { mask }
    }

    <span class="kw">pub const fn </span>remove(<span class="kw-2">&amp;</span><span class="self">self</span>, byte: u8) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>mask = <span class="self">self</span>.mask;
        mask[byte <span class="kw">as </span>usize / BITS_PER_CHUNK] &amp;= !(<span class="number">1 </span>&lt;&lt; (byte <span class="kw">as </span>usize % BITS_PER_CHUNK));
        AsciiSet { mask }
    }
}

<span class="doccomment">/// The set of 0x00 to 0x1F (C0 controls), and 0x7F (DEL).
///
/// Note that this includes the newline and tab characters, but not the space 0x20.
///
/// &lt;https://url.spec.whatwg.org/#c0-control-percent-encode-set&gt;
</span><span class="kw">pub const </span>CONTROLS: <span class="kw-2">&amp;</span>AsciiSet = <span class="kw-2">&amp;</span>AsciiSet {
    mask: [
        !<span class="number">0_u32</span>, <span class="comment">// C0: 0x00 to 0x1F (32 bits set)
        </span><span class="number">0</span>,
        <span class="number">0</span>,
        <span class="number">1 </span>&lt;&lt; (<span class="number">0x7F_u32 </span>% <span class="number">32</span>), <span class="comment">// DEL: 0x7F (one bit set)
    </span>],
};

<span class="macro">macro_rules!</span> static_assert {
    ($( <span class="macro-nonterminal">$bool</span>: expr, )+) =&gt; {
        <span class="kw">fn </span>_static_assert() {
            $(
                <span class="kw">let _ </span>= mem::transmute::&lt;[u8; <span class="macro-nonterminal">$bool </span><span class="kw">as </span>usize], u8&gt;;
            )+
        }
    }
}

<span class="macro">static_assert!</span> {
    CONTROLS.contains(<span class="number">0x00</span>),
    CONTROLS.contains(<span class="number">0x1F</span>),
    !CONTROLS.contains(<span class="number">0x20</span>),
    !CONTROLS.contains(<span class="number">0x7E</span>),
    CONTROLS.contains(<span class="number">0x7F</span>),
}

<span class="doccomment">/// Everything that is not an ASCII letter or digit.
///
/// This is probably more eager than necessary in any context.
</span><span class="kw">pub const </span>NON_ALPHANUMERIC: <span class="kw-2">&amp;</span>AsciiSet = <span class="kw-2">&amp;</span>CONTROLS
    .add(<span class="string">b' '</span>)
    .add(<span class="string">b'!'</span>)
    .add(<span class="string">b'"'</span>)
    .add(<span class="string">b'#'</span>)
    .add(<span class="string">b'$'</span>)
    .add(<span class="string">b'%'</span>)
    .add(<span class="string">b'&amp;'</span>)
    .add(<span class="string">b'\''</span>)
    .add(<span class="string">b'('</span>)
    .add(<span class="string">b')'</span>)
    .add(<span class="string">b'*'</span>)
    .add(<span class="string">b'+'</span>)
    .add(<span class="string">b','</span>)
    .add(<span class="string">b'-'</span>)
    .add(<span class="string">b'.'</span>)
    .add(<span class="string">b'/'</span>)
    .add(<span class="string">b':'</span>)
    .add(<span class="string">b';'</span>)
    .add(<span class="string">b'&lt;'</span>)
    .add(<span class="string">b'='</span>)
    .add(<span class="string">b'&gt;'</span>)
    .add(<span class="string">b'?'</span>)
    .add(<span class="string">b'@'</span>)
    .add(<span class="string">b'['</span>)
    .add(<span class="string">b'\\'</span>)
    .add(<span class="string">b']'</span>)
    .add(<span class="string">b'^'</span>)
    .add(<span class="string">b'_'</span>)
    .add(<span class="string">b'`'</span>)
    .add(<span class="string">b'{'</span>)
    .add(<span class="string">b'|'</span>)
    .add(<span class="string">b'}'</span>)
    .add(<span class="string">b'~'</span>);

<span class="doccomment">/// Return the percent-encoding of the given byte.
///
/// This is unconditional, unlike `percent_encode()` which has an `AsciiSet` parameter.
///
/// # Examples
///
/// ```
/// use percent_encoding::percent_encode_byte;
///
/// assert_eq!("foo bar".bytes().map(percent_encode_byte).collect::&lt;String&gt;(),
///            "%66%6F%6F%20%62%61%72");
/// ```
</span><span class="attr">#[inline]
</span><span class="kw">pub fn </span>percent_encode_byte(byte: u8) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str {
    <span class="kw">static </span>ENC_TABLE: <span class="kw-2">&amp;</span>[u8; <span class="number">768</span>] = <span class="string">b"\
      %00%01%02%03%04%05%06%07%08%09%0A%0B%0C%0D%0E%0F\
      %10%11%12%13%14%15%16%17%18%19%1A%1B%1C%1D%1E%1F\
      %20%21%22%23%24%25%26%27%28%29%2A%2B%2C%2D%2E%2F\
      %30%31%32%33%34%35%36%37%38%39%3A%3B%3C%3D%3E%3F\
      %40%41%42%43%44%45%46%47%48%49%4A%4B%4C%4D%4E%4F\
      %50%51%52%53%54%55%56%57%58%59%5A%5B%5C%5D%5E%5F\
      %60%61%62%63%64%65%66%67%68%69%6A%6B%6C%6D%6E%6F\
      %70%71%72%73%74%75%76%77%78%79%7A%7B%7C%7D%7E%7F\
      %80%81%82%83%84%85%86%87%88%89%8A%8B%8C%8D%8E%8F\
      %90%91%92%93%94%95%96%97%98%99%9A%9B%9C%9D%9E%9F\
      %A0%A1%A2%A3%A4%A5%A6%A7%A8%A9%AA%AB%AC%AD%AE%AF\
      %B0%B1%B2%B3%B4%B5%B6%B7%B8%B9%BA%BB%BC%BD%BE%BF\
      %C0%C1%C2%C3%C4%C5%C6%C7%C8%C9%CA%CB%CC%CD%CE%CF\
      %D0%D1%D2%D3%D4%D5%D6%D7%D8%D9%DA%DB%DC%DD%DE%DF\
      %E0%E1%E2%E3%E4%E5%E6%E7%E8%E9%EA%EB%EC%ED%EE%EF\
      %F0%F1%F2%F3%F4%F5%F6%F7%F8%F9%FA%FB%FC%FD%FE%FF\
      "</span>;

    <span class="kw">let </span>index = usize::from(byte) * <span class="number">3</span>;
    <span class="comment">// SAFETY: ENC_TABLE is ascii-only, so any subset if it should be
    // ascii-only too, which is valid utf8.
    </span><span class="kw">unsafe </span>{ str::from_utf8_unchecked(<span class="kw-2">&amp;</span>ENC_TABLE[index..index + <span class="number">3</span>]) }
}

<span class="doccomment">/// Percent-encode the given bytes with the given set.
///
/// Non-ASCII bytes and bytes in `ascii_set` are encoded.
///
/// The return type:
///
/// * Implements `Iterator&lt;Item = &amp;str&gt;` and therefore has a `.collect::&lt;String&gt;()` method,
/// * Implements `Display` and therefore has a `.to_string()` method,
/// * Implements `Into&lt;Cow&lt;str&gt;&gt;` borrowing `input` when none of its bytes are encoded.
///
/// # Examples
///
/// ```
/// use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
///
/// assert_eq!(percent_encode(b"foo bar?", NON_ALPHANUMERIC).to_string(), "foo%20bar%3F");
/// ```
</span><span class="attr">#[inline]
</span><span class="kw">pub fn </span>percent_encode&lt;<span class="lifetime">'a</span>&gt;(input: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8], ascii_set: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>AsciiSet) -&gt; PercentEncode&lt;<span class="lifetime">'a</span>&gt; {
    PercentEncode {
        bytes: input,
        ascii_set,
    }
}

<span class="doccomment">/// Percent-encode the UTF-8 encoding of the given string.
///
/// See [`percent_encode`] regarding the return type.
///
/// # Examples
///
/// ```
/// use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
///
/// assert_eq!(utf8_percent_encode("foo bar?", NON_ALPHANUMERIC).to_string(), "foo%20bar%3F");
/// ```
</span><span class="attr">#[inline]
</span><span class="kw">pub fn </span>utf8_percent_encode&lt;<span class="lifetime">'a</span>&gt;(input: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str, ascii_set: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>AsciiSet) -&gt; PercentEncode&lt;<span class="lifetime">'a</span>&gt; {
    percent_encode(input.as_bytes(), ascii_set)
}

<span class="doccomment">/// The return type of [`percent_encode`] and [`utf8_percent_encode`].
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub struct </span>PercentEncode&lt;<span class="lifetime">'a</span>&gt; {
    bytes: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8],
    ascii_set: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>AsciiSet,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>PercentEncode&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str;

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str&gt; {
        <span class="kw">if let </span><span class="prelude-val">Some</span>((<span class="kw-2">&amp;</span>first_byte, remaining)) = <span class="self">self</span>.bytes.split_first() {
            <span class="kw">if </span><span class="self">self</span>.ascii_set.should_percent_encode(first_byte) {
                <span class="self">self</span>.bytes = remaining;
                <span class="prelude-val">Some</span>(percent_encode_byte(first_byte))
            } <span class="kw">else </span>{
                <span class="comment">// The unsafe blocks here are appropriate because the bytes are
                // confirmed as a subset of UTF-8 in should_percent_encode.
                </span><span class="kw">for </span>(i, <span class="kw-2">&amp;</span>byte) <span class="kw">in </span>remaining.iter().enumerate() {
                    <span class="kw">if </span><span class="self">self</span>.ascii_set.should_percent_encode(byte) {
                        <span class="comment">// 1 for first_byte + i for previous iterations of this loop
                        </span><span class="kw">let </span>(unchanged_slice, remaining) = <span class="self">self</span>.bytes.split_at(<span class="number">1 </span>+ i);
                        <span class="self">self</span>.bytes = remaining;
                        <span class="kw">return </span><span class="prelude-val">Some</span>(<span class="kw">unsafe </span>{ str::from_utf8_unchecked(unchanged_slice) });
                    }
                }
                <span class="kw">let </span>unchanged_slice = <span class="self">self</span>.bytes;
                <span class="self">self</span>.bytes = <span class="kw-2">&amp;</span>[][..];
                <span class="prelude-val">Some</span>(<span class="kw">unsafe </span>{ str::from_utf8_unchecked(unchanged_slice) })
            }
        } <span class="kw">else </span>{
            <span class="prelude-val">None
        </span>}
    }

    <span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        <span class="kw">if </span><span class="self">self</span>.bytes.is_empty() {
            (<span class="number">0</span>, <span class="prelude-val">Some</span>(<span class="number">0</span>))
        } <span class="kw">else </span>{
            (<span class="number">1</span>, <span class="prelude-val">Some</span>(<span class="self">self</span>.bytes.len()))
        }
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; fmt::Display <span class="kw">for </span>PercentEncode&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, formatter: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        <span class="kw">for </span>c <span class="kw">in </span>(<span class="kw-2">*</span><span class="self">self</span>).clone() {
            formatter.write_str(c)<span class="question-mark">?
        </span>}
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; From&lt;PercentEncode&lt;<span class="lifetime">'a</span>&gt;&gt; <span class="kw">for </span>Cow&lt;<span class="lifetime">'a</span>, str&gt; {
    <span class="kw">fn </span>from(<span class="kw-2">mut </span>iter: PercentEncode&lt;<span class="lifetime">'a</span>&gt;) -&gt; <span class="self">Self </span>{
        <span class="kw">match </span>iter.next() {
            <span class="prelude-val">None </span>=&gt; <span class="string">""</span>.into(),
            <span class="prelude-val">Some</span>(first) =&gt; <span class="kw">match </span>iter.next() {
                <span class="prelude-val">None </span>=&gt; first.into(),
                <span class="prelude-val">Some</span>(second) =&gt; {
                    <span class="kw">let </span><span class="kw-2">mut </span>string = first.to_owned();
                    string.push_str(second);
                    string.extend(iter);
                    string.into()
                }
            },
        }
    }
}

<span class="doccomment">/// Percent-decode the given string.
///
/// &lt;https://url.spec.whatwg.org/#string-percent-decode&gt;
///
/// See [`percent_decode`] regarding the return type.
</span><span class="attr">#[inline]
</span><span class="kw">pub fn </span>percent_decode_str(input: <span class="kw-2">&amp;</span>str) -&gt; PercentDecode&lt;<span class="lifetime">'_</span>&gt; {
    percent_decode(input.as_bytes())
}

<span class="doccomment">/// Percent-decode the given bytes.
///
/// &lt;https://url.spec.whatwg.org/#percent-decode&gt;
///
/// Any sequence of `%` followed by two hexadecimal digits is decoded.
/// The return type:
///
/// * Implements `Into&lt;Cow&lt;u8&gt;&gt;` borrowing `input` when it contains no percent-encoded sequence,
/// * Implements `Iterator&lt;Item = u8&gt;` and therefore has a `.collect::&lt;Vec&lt;u8&gt;&gt;()` method,
/// * Has `decode_utf8()` and `decode_utf8_lossy()` methods.
///
/// # Examples
///
/// ```
/// use percent_encoding::percent_decode;
///
/// assert_eq!(percent_decode(b"foo%20bar%3f").decode_utf8().unwrap(), "foo bar?");
/// ```
</span><span class="attr">#[inline]
</span><span class="kw">pub fn </span>percent_decode(input: <span class="kw-2">&amp;</span>[u8]) -&gt; PercentDecode&lt;<span class="lifetime">'_</span>&gt; {
    PercentDecode {
        bytes: input.iter(),
    }
}

<span class="doccomment">/// The return type of [`percent_decode`].
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>PercentDecode&lt;<span class="lifetime">'a</span>&gt; {
    bytes: slice::Iter&lt;<span class="lifetime">'a</span>, u8&gt;,
}

<span class="kw">fn </span>after_percent_sign(iter: <span class="kw-2">&amp;mut </span>slice::Iter&lt;<span class="lifetime">'_</span>, u8&gt;) -&gt; <span class="prelude-ty">Option</span>&lt;u8&gt; {
    <span class="kw">let </span><span class="kw-2">mut </span>cloned_iter = iter.clone();
    <span class="kw">let </span>h = char::from(<span class="kw-2">*</span>cloned_iter.next()<span class="question-mark">?</span>).to_digit(<span class="number">16</span>)<span class="question-mark">?</span>;
    <span class="kw">let </span>l = char::from(<span class="kw-2">*</span>cloned_iter.next()<span class="question-mark">?</span>).to_digit(<span class="number">16</span>)<span class="question-mark">?</span>;
    <span class="kw-2">*</span>iter = cloned_iter;
    <span class="prelude-val">Some</span>(h <span class="kw">as </span>u8 * <span class="number">0x10 </span>+ l <span class="kw">as </span>u8)
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>PercentDecode&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = u8;

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;u8&gt; {
        <span class="self">self</span>.bytes.next().map(|<span class="kw-2">&amp;</span>byte| {
            <span class="kw">if </span>byte == <span class="string">b'%' </span>{
                after_percent_sign(<span class="kw-2">&amp;mut </span><span class="self">self</span>.bytes).unwrap_or(byte)
            } <span class="kw">else </span>{
                byte
            }
        })
    }

    <span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        <span class="kw">let </span>bytes = <span class="self">self</span>.bytes.len();
        ((bytes + <span class="number">2</span>) / <span class="number">3</span>, <span class="prelude-val">Some</span>(bytes))
    }
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; From&lt;PercentDecode&lt;<span class="lifetime">'a</span>&gt;&gt; <span class="kw">for </span>Cow&lt;<span class="lifetime">'a</span>, [u8]&gt; {
    <span class="kw">fn </span>from(iter: PercentDecode&lt;<span class="lifetime">'a</span>&gt;) -&gt; <span class="self">Self </span>{
        <span class="kw">match </span>iter.if_any() {
            <span class="prelude-val">Some</span>(vec) =&gt; Cow::Owned(vec),
            <span class="prelude-val">None </span>=&gt; Cow::Borrowed(iter.bytes.as_slice()),
        }
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; PercentDecode&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// If the percent-decoding is different from the input, return it as a new bytes vector.
    </span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>if_any(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Vec&lt;u8&gt;&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>bytes_iter = <span class="self">self</span>.bytes.clone();
        <span class="kw">while </span>bytes_iter.any(|<span class="kw-2">&amp;</span>b| b == <span class="string">b'%'</span>) {
            <span class="kw">if let </span><span class="prelude-val">Some</span>(decoded_byte) = after_percent_sign(<span class="kw-2">&amp;mut </span>bytes_iter) {
                <span class="kw">let </span>initial_bytes = <span class="self">self</span>.bytes.as_slice();
                <span class="kw">let </span>unchanged_bytes_len = initial_bytes.len() - bytes_iter.len() - <span class="number">3</span>;
                <span class="kw">let </span><span class="kw-2">mut </span>decoded = initial_bytes[..unchanged_bytes_len].to_owned();
                decoded.push(decoded_byte);
                decoded.extend(PercentDecode { bytes: bytes_iter });
                <span class="kw">return </span><span class="prelude-val">Some</span>(decoded);
            }
        }
        <span class="comment">// Nothing to decode
        </span><span class="prelude-val">None
    </span>}

    <span class="doccomment">/// Decode the result of percent-decoding as UTF-8.
    ///
    /// This is return `Err` when the percent-decoded bytes are not well-formed in UTF-8.
    </span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">pub fn </span>decode_utf8(<span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Cow&lt;<span class="lifetime">'a</span>, str&gt;, str::Utf8Error&gt; {
        <span class="kw">match </span><span class="self">self</span>.clone().into() {
            Cow::Borrowed(bytes) =&gt; <span class="kw">match </span>str::from_utf8(bytes) {
                <span class="prelude-val">Ok</span>(s) =&gt; <span class="prelude-val">Ok</span>(s.into()),
                <span class="prelude-val">Err</span>(e) =&gt; <span class="prelude-val">Err</span>(e),
            },
            Cow::Owned(bytes) =&gt; <span class="kw">match </span>String::from_utf8(bytes) {
                <span class="prelude-val">Ok</span>(s) =&gt; <span class="prelude-val">Ok</span>(s.into()),
                <span class="prelude-val">Err</span>(e) =&gt; <span class="prelude-val">Err</span>(e.utf8_error()),
            },
        }
    }

    <span class="doccomment">/// Decode the result of percent-decoding as UTF-8, lossily.
    ///
    /// Invalid UTF-8 percent-encoded byte sequences will be replaced � U+FFFD,
    /// the replacement character.
    </span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">pub fn </span>decode_utf8_lossy(<span class="self">self</span>) -&gt; Cow&lt;<span class="lifetime">'a</span>, str&gt; {
        decode_utf8_lossy(<span class="self">self</span>.clone().into())
    }
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">fn </span>decode_utf8_lossy(input: Cow&lt;<span class="lifetime">'_</span>, [u8]&gt;) -&gt; Cow&lt;<span class="lifetime">'_</span>, str&gt; {
    <span class="comment">// Note: This function is duplicated in `form_urlencoded/src/query_encoding.rs`.
    </span><span class="kw">match </span>input {
        Cow::Borrowed(bytes) =&gt; String::from_utf8_lossy(bytes),
        Cow::Owned(bytes) =&gt; {
            <span class="kw">match </span>String::from_utf8_lossy(<span class="kw-2">&amp;</span>bytes) {
                Cow::Borrowed(utf8) =&gt; {
                    <span class="comment">// If from_utf8_lossy returns a Cow::Borrowed, then we can
                    // be sure our original bytes were valid UTF-8. This is because
                    // if the bytes were invalid UTF-8 from_utf8_lossy would have
                    // to allocate a new owned string to back the Cow so it could
                    // replace invalid bytes with a placeholder.

                    // First we do a debug_assert to confirm our description above.
                    </span><span class="kw">let </span>raw_utf8: <span class="kw-2">*const </span>[u8] = utf8.as_bytes();
                    <span class="macro">debug_assert!</span>(raw_utf8 == <span class="kw-2">&amp;*</span>bytes <span class="kw">as </span><span class="kw-2">*const </span>[u8]);

                    <span class="comment">// Given we know the original input bytes are valid UTF-8,
                    // and we have ownership of those bytes, we re-use them and
                    // return a Cow::Owned here.
                    </span>Cow::Owned(<span class="kw">unsafe </span>{ String::from_utf8_unchecked(bytes) })
                }
                Cow::Owned(s) =&gt; Cow::Owned(s),
            }
        }
    }
}
</code></pre></div></section></main></body></html>