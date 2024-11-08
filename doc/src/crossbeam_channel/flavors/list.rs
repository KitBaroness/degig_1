<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/crossbeam-channel-0.5.12/src/flavors/list.rs`."><title>list.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="crossbeam_channel" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../crossbeam_channel/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#478" id="478">478</a>
<a href="#479" id="479">479</a>
<a href="#480" id="480">480</a>
<a href="#481" id="481">481</a>
<a href="#482" id="482">482</a>
<a href="#483" id="483">483</a>
<a href="#484" id="484">484</a>
<a href="#485" id="485">485</a>
<a href="#486" id="486">486</a>
<a href="#487" id="487">487</a>
<a href="#488" id="488">488</a>
<a href="#489" id="489">489</a>
<a href="#490" id="490">490</a>
<a href="#491" id="491">491</a>
<a href="#492" id="492">492</a>
<a href="#493" id="493">493</a>
<a href="#494" id="494">494</a>
<a href="#495" id="495">495</a>
<a href="#496" id="496">496</a>
<a href="#497" id="497">497</a>
<a href="#498" id="498">498</a>
<a href="#499" id="499">499</a>
<a href="#500" id="500">500</a>
<a href="#501" id="501">501</a>
<a href="#502" id="502">502</a>
<a href="#503" id="503">503</a>
<a href="#504" id="504">504</a>
<a href="#505" id="505">505</a>
<a href="#506" id="506">506</a>
<a href="#507" id="507">507</a>
<a href="#508" id="508">508</a>
<a href="#509" id="509">509</a>
<a href="#510" id="510">510</a>
<a href="#511" id="511">511</a>
<a href="#512" id="512">512</a>
<a href="#513" id="513">513</a>
<a href="#514" id="514">514</a>
<a href="#515" id="515">515</a>
<a href="#516" id="516">516</a>
<a href="#517" id="517">517</a>
<a href="#518" id="518">518</a>
<a href="#519" id="519">519</a>
<a href="#520" id="520">520</a>
<a href="#521" id="521">521</a>
<a href="#522" id="522">522</a>
<a href="#523" id="523">523</a>
<a href="#524" id="524">524</a>
<a href="#525" id="525">525</a>
<a href="#526" id="526">526</a>
<a href="#527" id="527">527</a>
<a href="#528" id="528">528</a>
<a href="#529" id="529">529</a>
<a href="#530" id="530">530</a>
<a href="#531" id="531">531</a>
<a href="#532" id="532">532</a>
<a href="#533" id="533">533</a>
<a href="#534" id="534">534</a>
<a href="#535" id="535">535</a>
<a href="#536" id="536">536</a>
<a href="#537" id="537">537</a>
<a href="#538" id="538">538</a>
<a href="#539" id="539">539</a>
<a href="#540" id="540">540</a>
<a href="#541" id="541">541</a>
<a href="#542" id="542">542</a>
<a href="#543" id="543">543</a>
<a href="#544" id="544">544</a>
<a href="#545" id="545">545</a>
<a href="#546" id="546">546</a>
<a href="#547" id="547">547</a>
<a href="#548" id="548">548</a>
<a href="#549" id="549">549</a>
<a href="#550" id="550">550</a>
<a href="#551" id="551">551</a>
<a href="#552" id="552">552</a>
<a href="#553" id="553">553</a>
<a href="#554" id="554">554</a>
<a href="#555" id="555">555</a>
<a href="#556" id="556">556</a>
<a href="#557" id="557">557</a>
<a href="#558" id="558">558</a>
<a href="#559" id="559">559</a>
<a href="#560" id="560">560</a>
<a href="#561" id="561">561</a>
<a href="#562" id="562">562</a>
<a href="#563" id="563">563</a>
<a href="#564" id="564">564</a>
<a href="#565" id="565">565</a>
<a href="#566" id="566">566</a>
<a href="#567" id="567">567</a>
<a href="#568" id="568">568</a>
<a href="#569" id="569">569</a>
<a href="#570" id="570">570</a>
<a href="#571" id="571">571</a>
<a href="#572" id="572">572</a>
<a href="#573" id="573">573</a>
<a href="#574" id="574">574</a>
<a href="#575" id="575">575</a>
<a href="#576" id="576">576</a>
<a href="#577" id="577">577</a>
<a href="#578" id="578">578</a>
<a href="#579" id="579">579</a>
<a href="#580" id="580">580</a>
<a href="#581" id="581">581</a>
<a href="#582" id="582">582</a>
<a href="#583" id="583">583</a>
<a href="#584" id="584">584</a>
<a href="#585" id="585">585</a>
<a href="#586" id="586">586</a>
<a href="#587" id="587">587</a>
<a href="#588" id="588">588</a>
<a href="#589" id="589">589</a>
<a href="#590" id="590">590</a>
<a href="#591" id="591">591</a>
<a href="#592" id="592">592</a>
<a href="#593" id="593">593</a>
<a href="#594" id="594">594</a>
<a href="#595" id="595">595</a>
<a href="#596" id="596">596</a>
<a href="#597" id="597">597</a>
<a href="#598" id="598">598</a>
<a href="#599" id="599">599</a>
<a href="#600" id="600">600</a>
<a href="#601" id="601">601</a>
<a href="#602" id="602">602</a>
<a href="#603" id="603">603</a>
<a href="#604" id="604">604</a>
<a href="#605" id="605">605</a>
<a href="#606" id="606">606</a>
<a href="#607" id="607">607</a>
<a href="#608" id="608">608</a>
<a href="#609" id="609">609</a>
<a href="#610" id="610">610</a>
<a href="#611" id="611">611</a>
<a href="#612" id="612">612</a>
<a href="#613" id="613">613</a>
<a href="#614" id="614">614</a>
<a href="#615" id="615">615</a>
<a href="#616" id="616">616</a>
<a href="#617" id="617">617</a>
<a href="#618" id="618">618</a>
<a href="#619" id="619">619</a>
<a href="#620" id="620">620</a>
<a href="#621" id="621">621</a>
<a href="#622" id="622">622</a>
<a href="#623" id="623">623</a>
<a href="#624" id="624">624</a>
<a href="#625" id="625">625</a>
<a href="#626" id="626">626</a>
<a href="#627" id="627">627</a>
<a href="#628" id="628">628</a>
<a href="#629" id="629">629</a>
<a href="#630" id="630">630</a>
<a href="#631" id="631">631</a>
<a href="#632" id="632">632</a>
<a href="#633" id="633">633</a>
<a href="#634" id="634">634</a>
<a href="#635" id="635">635</a>
<a href="#636" id="636">636</a>
<a href="#637" id="637">637</a>
<a href="#638" id="638">638</a>
<a href="#639" id="639">639</a>
<a href="#640" id="640">640</a>
<a href="#641" id="641">641</a>
<a href="#642" id="642">642</a>
<a href="#643" id="643">643</a>
<a href="#644" id="644">644</a>
<a href="#645" id="645">645</a>
<a href="#646" id="646">646</a>
<a href="#647" id="647">647</a>
<a href="#648" id="648">648</a>
<a href="#649" id="649">649</a>
<a href="#650" id="650">650</a>
<a href="#651" id="651">651</a>
<a href="#652" id="652">652</a>
<a href="#653" id="653">653</a>
<a href="#654" id="654">654</a>
<a href="#655" id="655">655</a>
<a href="#656" id="656">656</a>
<a href="#657" id="657">657</a>
<a href="#658" id="658">658</a>
<a href="#659" id="659">659</a>
<a href="#660" id="660">660</a>
<a href="#661" id="661">661</a>
<a href="#662" id="662">662</a>
<a href="#663" id="663">663</a>
<a href="#664" id="664">664</a>
<a href="#665" id="665">665</a>
<a href="#666" id="666">666</a>
<a href="#667" id="667">667</a>
<a href="#668" id="668">668</a>
<a href="#669" id="669">669</a>
<a href="#670" id="670">670</a>
<a href="#671" id="671">671</a>
<a href="#672" id="672">672</a>
<a href="#673" id="673">673</a>
<a href="#674" id="674">674</a>
<a href="#675" id="675">675</a>
<a href="#676" id="676">676</a>
<a href="#677" id="677">677</a>
<a href="#678" id="678">678</a>
<a href="#679" id="679">679</a>
<a href="#680" id="680">680</a>
<a href="#681" id="681">681</a>
<a href="#682" id="682">682</a>
<a href="#683" id="683">683</a>
<a href="#684" id="684">684</a>
<a href="#685" id="685">685</a>
<a href="#686" id="686">686</a>
<a href="#687" id="687">687</a>
<a href="#688" id="688">688</a>
<a href="#689" id="689">689</a>
<a href="#690" id="690">690</a>
<a href="#691" id="691">691</a>
<a href="#692" id="692">692</a>
<a href="#693" id="693">693</a>
<a href="#694" id="694">694</a>
<a href="#695" id="695">695</a>
<a href="#696" id="696">696</a>
<a href="#697" id="697">697</a>
<a href="#698" id="698">698</a>
<a href="#699" id="699">699</a>
<a href="#700" id="700">700</a>
<a href="#701" id="701">701</a>
<a href="#702" id="702">702</a>
<a href="#703" id="703">703</a>
<a href="#704" id="704">704</a>
<a href="#705" id="705">705</a>
<a href="#706" id="706">706</a>
<a href="#707" id="707">707</a>
<a href="#708" id="708">708</a>
<a href="#709" id="709">709</a>
<a href="#710" id="710">710</a>
<a href="#711" id="711">711</a>
<a href="#712" id="712">712</a>
<a href="#713" id="713">713</a>
<a href="#714" id="714">714</a>
<a href="#715" id="715">715</a>
<a href="#716" id="716">716</a>
<a href="#717" id="717">717</a>
<a href="#718" id="718">718</a>
<a href="#719" id="719">719</a>
<a href="#720" id="720">720</a>
<a href="#721" id="721">721</a>
<a href="#722" id="722">722</a>
<a href="#723" id="723">723</a>
<a href="#724" id="724">724</a>
<a href="#725" id="725">725</a>
<a href="#726" id="726">726</a>
<a href="#727" id="727">727</a>
<a href="#728" id="728">728</a>
<a href="#729" id="729">729</a>
<a href="#730" id="730">730</a>
<a href="#731" id="731">731</a>
<a href="#732" id="732">732</a>
<a href="#733" id="733">733</a>
<a href="#734" id="734">734</a>
<a href="#735" id="735">735</a>
<a href="#736" id="736">736</a>
<a href="#737" id="737">737</a>
<a href="#738" id="738">738</a>
<a href="#739" id="739">739</a>
<a href="#740" id="740">740</a>
<a href="#741" id="741">741</a>
<a href="#742" id="742">742</a>
<a href="#743" id="743">743</a>
<a href="#744" id="744">744</a>
<a href="#745" id="745">745</a>
<a href="#746" id="746">746</a>
<a href="#747" id="747">747</a>
<a href="#748" id="748">748</a>
<a href="#749" id="749">749</a>
<a href="#750" id="750">750</a>
<a href="#751" id="751">751</a>
<a href="#752" id="752">752</a>
<a href="#753" id="753">753</a>
<a href="#754" id="754">754</a>
<a href="#755" id="755">755</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! Unbounded channel implemented as a linked list.

</span><span class="kw">use </span>std::boxed::Box;
<span class="kw">use </span>std::cell::UnsafeCell;
<span class="kw">use </span>std::marker::PhantomData;
<span class="kw">use </span>std::mem::MaybeUninit;
<span class="kw">use </span>std::ptr;
<span class="kw">use </span>std::sync::atomic::{<span class="self">self</span>, AtomicPtr, AtomicUsize, Ordering};
<span class="kw">use </span>std::time::Instant;

<span class="kw">use </span>crossbeam_utils::{Backoff, CachePadded};

<span class="kw">use </span><span class="kw">crate</span>::context::Context;
<span class="kw">use </span><span class="kw">crate</span>::err::{RecvTimeoutError, SendTimeoutError, TryRecvError, TrySendError};
<span class="kw">use </span><span class="kw">crate</span>::select::{Operation, SelectHandle, Selected, Token};
<span class="kw">use </span><span class="kw">crate</span>::waker::SyncWaker;

<span class="comment">// TODO(stjepang): Once we bump the minimum required Rust version to 1.28 or newer, re-apply the
// following changes by @kleimkuhler:
//
// 1. https://github.com/crossbeam-rs/crossbeam-channel/pull/100
// 2. https://github.com/crossbeam-rs/crossbeam-channel/pull/101

// Bits indicating the state of a slot:
// * If a message has been written into the slot, `WRITE` is set.
// * If a message has been read from the slot, `READ` is set.
// * If the block is being destroyed, `DESTROY` is set.
</span><span class="kw">const </span>WRITE: usize = <span class="number">1</span>;
<span class="kw">const </span>READ: usize = <span class="number">2</span>;
<span class="kw">const </span>DESTROY: usize = <span class="number">4</span>;

<span class="comment">// Each block covers one "lap" of indices.
</span><span class="kw">const </span>LAP: usize = <span class="number">32</span>;
<span class="comment">// The maximum number of messages a block can hold.
</span><span class="kw">const </span>BLOCK_CAP: usize = LAP - <span class="number">1</span>;
<span class="comment">// How many lower bits are reserved for metadata.
</span><span class="kw">const </span>SHIFT: usize = <span class="number">1</span>;
<span class="comment">// Has two different purposes:
// * If set in head, indicates that the block is not the last one.
// * If set in tail, indicates that the channel is disconnected.
</span><span class="kw">const </span>MARK_BIT: usize = <span class="number">1</span>;

<span class="doccomment">/// A slot in a block.
</span><span class="kw">struct </span>Slot&lt;T&gt; {
    <span class="doccomment">/// The message.
    </span>msg: UnsafeCell&lt;MaybeUninit&lt;T&gt;&gt;,

    <span class="doccomment">/// The state of the slot.
    </span>state: AtomicUsize,
}

<span class="kw">impl</span>&lt;T&gt; Slot&lt;T&gt; {
    <span class="kw">const </span>UNINIT: <span class="self">Self </span>= <span class="self">Self </span>{
        msg: UnsafeCell::new(MaybeUninit::uninit()),
        state: AtomicUsize::new(<span class="number">0</span>),
    };

    <span class="doccomment">/// Waits until a message is written into the slot.
    </span><span class="kw">fn </span>wait_write(<span class="kw-2">&amp;</span><span class="self">self</span>) {
        <span class="kw">let </span>backoff = Backoff::new();
        <span class="kw">while </span><span class="self">self</span>.state.load(Ordering::Acquire) &amp; WRITE == <span class="number">0 </span>{
            backoff.snooze();
        }
    }
}

<span class="doccomment">/// A block in a linked list.
///
/// Each block in the list can hold up to `BLOCK_CAP` messages.
</span><span class="kw">struct </span>Block&lt;T&gt; {
    <span class="doccomment">/// The next block in the linked list.
    </span>next: AtomicPtr&lt;Block&lt;T&gt;&gt;,

    <span class="doccomment">/// Slots for messages.
    </span>slots: [Slot&lt;T&gt;; BLOCK_CAP],
}

<span class="kw">impl</span>&lt;T&gt; Block&lt;T&gt; {
    <span class="doccomment">/// Creates an empty block.
    </span><span class="kw">fn </span>new() -&gt; Block&lt;T&gt; {
        <span class="self">Self </span>{
            next: AtomicPtr::new(ptr::null_mut()),
            slots: [Slot::UNINIT; BLOCK_CAP],
        }
    }

    <span class="doccomment">/// Waits until the next pointer is set.
    </span><span class="kw">fn </span>wait_next(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">*mut </span>Block&lt;T&gt; {
        <span class="kw">let </span>backoff = Backoff::new();
        <span class="kw">loop </span>{
            <span class="kw">let </span>next = <span class="self">self</span>.next.load(Ordering::Acquire);
            <span class="kw">if </span>!next.is_null() {
                <span class="kw">return </span>next;
            }
            backoff.snooze();
        }
    }

    <span class="doccomment">/// Sets the `DESTROY` bit in slots starting from `start` and destroys the block.
    </span><span class="kw">unsafe fn </span>destroy(this: <span class="kw-2">*mut </span>Block&lt;T&gt;, start: usize) {
        <span class="comment">// It is not necessary to set the `DESTROY` bit in the last slot because that slot has
        // begun destruction of the block.
        </span><span class="kw">for </span>i <span class="kw">in </span>start..BLOCK_CAP - <span class="number">1 </span>{
            <span class="kw">let </span>slot = (<span class="kw-2">*</span>this).slots.get_unchecked(i);

            <span class="comment">// Mark the `DESTROY` bit if a thread is still using the slot.
            </span><span class="kw">if </span>slot.state.load(Ordering::Acquire) &amp; READ == <span class="number">0
                </span>&amp;&amp; slot.state.fetch_or(DESTROY, Ordering::AcqRel) &amp; READ == <span class="number">0
            </span>{
                <span class="comment">// If a thread is still using the slot, it will continue destruction of the block.
                </span><span class="kw">return</span>;
            }
        }

        <span class="comment">// No thread is using the block, now it is safe to destroy it.
        </span>drop(Box::from_raw(this));
    }
}

<span class="doccomment">/// A position in a channel.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">struct </span>Position&lt;T&gt; {
    <span class="doccomment">/// The index in the channel.
    </span>index: AtomicUsize,

    <span class="doccomment">/// The block in the linked list.
    </span>block: AtomicPtr&lt;Block&lt;T&gt;&gt;,
}

<span class="doccomment">/// The token type for the list flavor.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>ListToken {
    <span class="doccomment">/// The block of slots.
    </span>block: <span class="kw-2">*const </span>u8,

    <span class="doccomment">/// The offset into the block.
    </span>offset: usize,
}

<span class="kw">impl </span>Default <span class="kw">for </span>ListToken {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        ListToken {
            block: ptr::null(),
            offset: <span class="number">0</span>,
        }
    }
}

<span class="doccomment">/// Unbounded channel implemented as a linked list.
///
/// Each message sent into the channel is assigned a sequence number, i.e. an index. Indices are
/// represented as numbers of type `usize` and wrap on overflow.
///
/// Consecutive messages are grouped into blocks in order to put less pressure on the allocator and
/// improve cache efficiency.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Channel&lt;T&gt; {
    <span class="doccomment">/// The head of the channel.
    </span>head: CachePadded&lt;Position&lt;T&gt;&gt;,

    <span class="doccomment">/// The tail of the channel.
    </span>tail: CachePadded&lt;Position&lt;T&gt;&gt;,

    <span class="doccomment">/// Receivers waiting while the channel is empty and not disconnected.
    </span>receivers: SyncWaker,

    <span class="doccomment">/// Indicates that dropping a `Channel&lt;T&gt;` may drop messages of type `T`.
    </span>_marker: PhantomData&lt;T&gt;,
}

<span class="kw">impl</span>&lt;T&gt; Channel&lt;T&gt; {
    <span class="doccomment">/// Creates a new unbounded channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>new() -&gt; <span class="self">Self </span>{
        Channel {
            head: CachePadded::new(Position {
                block: AtomicPtr::new(ptr::null_mut()),
                index: AtomicUsize::new(<span class="number">0</span>),
            }),
            tail: CachePadded::new(Position {
                block: AtomicPtr::new(ptr::null_mut()),
                index: AtomicUsize::new(<span class="number">0</span>),
            }),
            receivers: SyncWaker::new(),
            _marker: PhantomData,
        }
    }

    <span class="doccomment">/// Returns a receiver handle to the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>receiver(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Receiver&lt;<span class="lifetime">'_</span>, T&gt; {
        Receiver(<span class="self">self</span>)
    }

    <span class="doccomment">/// Returns a sender handle to the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>sender(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Sender&lt;<span class="lifetime">'_</span>, T&gt; {
        Sender(<span class="self">self</span>)
    }

    <span class="doccomment">/// Attempts to reserve a slot for sending a message.
    </span><span class="kw">fn </span>start_send(<span class="kw-2">&amp;</span><span class="self">self</span>, token: <span class="kw-2">&amp;mut </span>Token) -&gt; bool {
        <span class="kw">let </span>backoff = Backoff::new();
        <span class="kw">let </span><span class="kw-2">mut </span>tail = <span class="self">self</span>.tail.index.load(Ordering::Acquire);
        <span class="kw">let </span><span class="kw-2">mut </span>block = <span class="self">self</span>.tail.block.load(Ordering::Acquire);
        <span class="kw">let </span><span class="kw-2">mut </span>next_block = <span class="prelude-val">None</span>;

        <span class="kw">loop </span>{
            <span class="comment">// Check if the channel is disconnected.
            </span><span class="kw">if </span>tail &amp; MARK_BIT != <span class="number">0 </span>{
                token.list.block = ptr::null();
                <span class="kw">return </span><span class="bool-val">true</span>;
            }

            <span class="comment">// Calculate the offset of the index into the block.
            </span><span class="kw">let </span>offset = (tail &gt;&gt; SHIFT) % LAP;

            <span class="comment">// If we reached the end of the block, wait until the next one is installed.
            </span><span class="kw">if </span>offset == BLOCK_CAP {
                backoff.snooze();
                tail = <span class="self">self</span>.tail.index.load(Ordering::Acquire);
                block = <span class="self">self</span>.tail.block.load(Ordering::Acquire);
                <span class="kw">continue</span>;
            }

            <span class="comment">// If we're going to have to install the next block, allocate it in advance in order to
            // make the wait for other threads as short as possible.
            </span><span class="kw">if </span>offset + <span class="number">1 </span>== BLOCK_CAP &amp;&amp; next_block.is_none() {
                next_block = <span class="prelude-val">Some</span>(Box::new(Block::&lt;T&gt;::new()));
            }

            <span class="comment">// If this is the first message to be sent into the channel, we need to allocate the
            // first block and install it.
            </span><span class="kw">if </span>block.is_null() {
                <span class="kw">let </span>new = Box::into_raw(Box::new(Block::&lt;T&gt;::new()));

                <span class="kw">if </span><span class="self">self
                    </span>.tail
                    .block
                    .compare_exchange(block, new, Ordering::Release, Ordering::Relaxed)
                    .is_ok()
                {
                    <span class="self">self</span>.head.block.store(new, Ordering::Release);
                    block = new;
                } <span class="kw">else </span>{
                    next_block = <span class="kw">unsafe </span>{ <span class="prelude-val">Some</span>(Box::from_raw(new)) };
                    tail = <span class="self">self</span>.tail.index.load(Ordering::Acquire);
                    block = <span class="self">self</span>.tail.block.load(Ordering::Acquire);
                    <span class="kw">continue</span>;
                }
            }

            <span class="kw">let </span>new_tail = tail + (<span class="number">1 </span>&lt;&lt; SHIFT);

            <span class="comment">// Try advancing the tail forward.
            </span><span class="kw">match </span><span class="self">self</span>.tail.index.compare_exchange_weak(
                tail,
                new_tail,
                Ordering::SeqCst,
                Ordering::Acquire,
            ) {
                <span class="prelude-val">Ok</span>(<span class="kw">_</span>) =&gt; <span class="kw">unsafe </span>{
                    <span class="comment">// If we've reached the end of the block, install the next one.
                    </span><span class="kw">if </span>offset + <span class="number">1 </span>== BLOCK_CAP {
                        <span class="kw">let </span>next_block = Box::into_raw(next_block.unwrap());
                        <span class="self">self</span>.tail.block.store(next_block, Ordering::Release);
                        <span class="self">self</span>.tail.index.fetch_add(<span class="number">1 </span>&lt;&lt; SHIFT, Ordering::Release);
                        (<span class="kw-2">*</span>block).next.store(next_block, Ordering::Release);
                    }

                    token.list.block = block <span class="kw">as </span><span class="kw-2">*const </span>u8;
                    token.list.offset = offset;
                    <span class="kw">return </span><span class="bool-val">true</span>;
                },
                <span class="prelude-val">Err</span>(t) =&gt; {
                    tail = t;
                    block = <span class="self">self</span>.tail.block.load(Ordering::Acquire);
                    backoff.spin();
                }
            }
        }
    }

    <span class="doccomment">/// Writes a message into the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">unsafe fn </span>write(<span class="kw-2">&amp;</span><span class="self">self</span>, token: <span class="kw-2">&amp;mut </span>Token, msg: T) -&gt; <span class="prelude-ty">Result</span>&lt;(), T&gt; {
        <span class="comment">// If there is no slot, the channel is disconnected.
        </span><span class="kw">if </span>token.list.block.is_null() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(msg);
        }

        <span class="comment">// Write the message into the slot.
        </span><span class="kw">let </span>block = token.list.block.cast::&lt;Block&lt;T&gt;&gt;();
        <span class="kw">let </span>offset = token.list.offset;
        <span class="kw">let </span>slot = (<span class="kw-2">*</span>block).slots.get_unchecked(offset);
        slot.msg.get().write(MaybeUninit::new(msg));
        slot.state.fetch_or(WRITE, Ordering::Release);

        <span class="comment">// Wake a sleeping receiver.
        </span><span class="self">self</span>.receivers.notify();
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Attempts to reserve a slot for receiving a message.
    </span><span class="kw">fn </span>start_recv(<span class="kw-2">&amp;</span><span class="self">self</span>, token: <span class="kw-2">&amp;mut </span>Token) -&gt; bool {
        <span class="kw">let </span>backoff = Backoff::new();
        <span class="kw">let </span><span class="kw-2">mut </span>head = <span class="self">self</span>.head.index.load(Ordering::Acquire);
        <span class="kw">let </span><span class="kw-2">mut </span>block = <span class="self">self</span>.head.block.load(Ordering::Acquire);

        <span class="kw">loop </span>{
            <span class="comment">// Calculate the offset of the index into the block.
            </span><span class="kw">let </span>offset = (head &gt;&gt; SHIFT) % LAP;

            <span class="comment">// If we reached the end of the block, wait until the next one is installed.
            </span><span class="kw">if </span>offset == BLOCK_CAP {
                backoff.snooze();
                head = <span class="self">self</span>.head.index.load(Ordering::Acquire);
                block = <span class="self">self</span>.head.block.load(Ordering::Acquire);
                <span class="kw">continue</span>;
            }

            <span class="kw">let </span><span class="kw-2">mut </span>new_head = head + (<span class="number">1 </span>&lt;&lt; SHIFT);

            <span class="kw">if </span>new_head &amp; MARK_BIT == <span class="number">0 </span>{
                atomic::fence(Ordering::SeqCst);
                <span class="kw">let </span>tail = <span class="self">self</span>.tail.index.load(Ordering::Relaxed);

                <span class="comment">// If the tail equals the head, that means the channel is empty.
                </span><span class="kw">if </span>head &gt;&gt; SHIFT == tail &gt;&gt; SHIFT {
                    <span class="comment">// If the channel is disconnected...
                    </span><span class="kw">if </span>tail &amp; MARK_BIT != <span class="number">0 </span>{
                        <span class="comment">// ...then receive an error.
                        </span>token.list.block = ptr::null();
                        <span class="kw">return </span><span class="bool-val">true</span>;
                    } <span class="kw">else </span>{
                        <span class="comment">// Otherwise, the receive operation is not ready.
                        </span><span class="kw">return </span><span class="bool-val">false</span>;
                    }
                }

                <span class="comment">// If head and tail are not in the same block, set `MARK_BIT` in head.
                </span><span class="kw">if </span>(head &gt;&gt; SHIFT) / LAP != (tail &gt;&gt; SHIFT) / LAP {
                    new_head |= MARK_BIT;
                }
            }

            <span class="comment">// The block can be null here only if the first message is being sent into the channel.
            // In that case, just wait until it gets initialized.
            </span><span class="kw">if </span>block.is_null() {
                backoff.snooze();
                head = <span class="self">self</span>.head.index.load(Ordering::Acquire);
                block = <span class="self">self</span>.head.block.load(Ordering::Acquire);
                <span class="kw">continue</span>;
            }

            <span class="comment">// Try moving the head index forward.
            </span><span class="kw">match </span><span class="self">self</span>.head.index.compare_exchange_weak(
                head,
                new_head,
                Ordering::SeqCst,
                Ordering::Acquire,
            ) {
                <span class="prelude-val">Ok</span>(<span class="kw">_</span>) =&gt; <span class="kw">unsafe </span>{
                    <span class="comment">// If we've reached the end of the block, move to the next one.
                    </span><span class="kw">if </span>offset + <span class="number">1 </span>== BLOCK_CAP {
                        <span class="kw">let </span>next = (<span class="kw-2">*</span>block).wait_next();
                        <span class="kw">let </span><span class="kw-2">mut </span>next_index = (new_head &amp; !MARK_BIT).wrapping_add(<span class="number">1 </span>&lt;&lt; SHIFT);
                        <span class="kw">if </span>!(<span class="kw-2">*</span>next).next.load(Ordering::Relaxed).is_null() {
                            next_index |= MARK_BIT;
                        }

                        <span class="self">self</span>.head.block.store(next, Ordering::Release);
                        <span class="self">self</span>.head.index.store(next_index, Ordering::Release);
                    }

                    token.list.block = block <span class="kw">as </span><span class="kw-2">*const </span>u8;
                    token.list.offset = offset;
                    <span class="kw">return </span><span class="bool-val">true</span>;
                },
                <span class="prelude-val">Err</span>(h) =&gt; {
                    head = h;
                    block = <span class="self">self</span>.head.block.load(Ordering::Acquire);
                    backoff.spin();
                }
            }
        }
    }

    <span class="doccomment">/// Reads a message from the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">unsafe fn </span>read(<span class="kw-2">&amp;</span><span class="self">self</span>, token: <span class="kw-2">&amp;mut </span>Token) -&gt; <span class="prelude-ty">Result</span>&lt;T, ()&gt; {
        <span class="kw">if </span>token.list.block.is_null() {
            <span class="comment">// The channel is disconnected.
            </span><span class="kw">return </span><span class="prelude-val">Err</span>(());
        }

        <span class="comment">// Read the message.
        </span><span class="kw">let </span>block = token.list.block <span class="kw">as </span><span class="kw-2">*mut </span>Block&lt;T&gt;;
        <span class="kw">let </span>offset = token.list.offset;
        <span class="kw">let </span>slot = (<span class="kw-2">*</span>block).slots.get_unchecked(offset);
        slot.wait_write();
        <span class="kw">let </span>msg = slot.msg.get().read().assume_init();

        <span class="comment">// Destroy the block if we've reached the end, or if another thread wanted to destroy but
        // couldn't because we were busy reading from the slot.
        </span><span class="kw">if </span>offset + <span class="number">1 </span>== BLOCK_CAP {
            Block::destroy(block, <span class="number">0</span>);
        } <span class="kw">else if </span>slot.state.fetch_or(READ, Ordering::AcqRel) &amp; DESTROY != <span class="number">0 </span>{
            Block::destroy(block, offset + <span class="number">1</span>);
        }

        <span class="prelude-val">Ok</span>(msg)
    }

    <span class="doccomment">/// Attempts to send a message into the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>try_send(<span class="kw-2">&amp;</span><span class="self">self</span>, msg: T) -&gt; <span class="prelude-ty">Result</span>&lt;(), TrySendError&lt;T&gt;&gt; {
        <span class="self">self</span>.send(msg, <span class="prelude-val">None</span>).map_err(|err| <span class="kw">match </span>err {
            SendTimeoutError::Disconnected(msg) =&gt; TrySendError::Disconnected(msg),
            SendTimeoutError::Timeout(<span class="kw">_</span>) =&gt; <span class="macro">unreachable!</span>(),
        })
    }

    <span class="doccomment">/// Sends a message into the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>send(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        msg: T,
        _deadline: <span class="prelude-ty">Option</span>&lt;Instant&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), SendTimeoutError&lt;T&gt;&gt; {
        <span class="kw">let </span>token = <span class="kw-2">&amp;mut </span>Token::default();
        <span class="macro">assert!</span>(<span class="self">self</span>.start_send(token));
        <span class="kw">unsafe </span>{
            <span class="self">self</span>.write(token, msg)
                .map_err(SendTimeoutError::Disconnected)
        }
    }

    <span class="doccomment">/// Attempts to receive a message without blocking.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>try_recv(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;T, TryRecvError&gt; {
        <span class="kw">let </span>token = <span class="kw-2">&amp;mut </span>Token::default();

        <span class="kw">if </span><span class="self">self</span>.start_recv(token) {
            <span class="kw">unsafe </span>{ <span class="self">self</span>.read(token).map_err(|<span class="kw">_</span>| TryRecvError::Disconnected) }
        } <span class="kw">else </span>{
            <span class="prelude-val">Err</span>(TryRecvError::Empty)
        }
    }

    <span class="doccomment">/// Receives a message from the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>recv(<span class="kw-2">&amp;</span><span class="self">self</span>, deadline: <span class="prelude-ty">Option</span>&lt;Instant&gt;) -&gt; <span class="prelude-ty">Result</span>&lt;T, RecvTimeoutError&gt; {
        <span class="kw">let </span>token = <span class="kw-2">&amp;mut </span>Token::default();
        <span class="kw">loop </span>{
            <span class="comment">// Try receiving a message several times.
            </span><span class="kw">let </span>backoff = Backoff::new();
            <span class="kw">loop </span>{
                <span class="kw">if </span><span class="self">self</span>.start_recv(token) {
                    <span class="kw">unsafe </span>{
                        <span class="kw">return </span><span class="self">self</span>.read(token).map_err(|<span class="kw">_</span>| RecvTimeoutError::Disconnected);
                    }
                }

                <span class="kw">if </span>backoff.is_completed() {
                    <span class="kw">break</span>;
                } <span class="kw">else </span>{
                    backoff.snooze();
                }
            }

            <span class="kw">if let </span><span class="prelude-val">Some</span>(d) = deadline {
                <span class="kw">if </span>Instant::now() &gt;= d {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(RecvTimeoutError::Timeout);
                }
            }

            <span class="comment">// Prepare for blocking until a sender wakes us up.
            </span>Context::with(|cx| {
                <span class="kw">let </span>oper = Operation::hook(token);
                <span class="self">self</span>.receivers.register(oper, cx);

                <span class="comment">// Has the channel become ready just now?
                </span><span class="kw">if </span>!<span class="self">self</span>.is_empty() || <span class="self">self</span>.is_disconnected() {
                    <span class="kw">let _ </span>= cx.try_select(Selected::Aborted);
                }

                <span class="comment">// Block the current thread.
                </span><span class="kw">let </span>sel = cx.wait_until(deadline);

                <span class="kw">match </span>sel {
                    Selected::Waiting =&gt; <span class="macro">unreachable!</span>(),
                    Selected::Aborted | Selected::Disconnected =&gt; {
                        <span class="self">self</span>.receivers.unregister(oper).unwrap();
                        <span class="comment">// If the channel was disconnected, we still have to check for remaining
                        // messages.
                    </span>}
                    Selected::Operation(<span class="kw">_</span>) =&gt; {}
                }
            });
        }
    }

    <span class="doccomment">/// Returns the current number of messages inside the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="kw">loop </span>{
            <span class="comment">// Load the tail index, then load the head index.
            </span><span class="kw">let </span><span class="kw-2">mut </span>tail = <span class="self">self</span>.tail.index.load(Ordering::SeqCst);
            <span class="kw">let </span><span class="kw-2">mut </span>head = <span class="self">self</span>.head.index.load(Ordering::SeqCst);

            <span class="comment">// If the tail index didn't change, we've got consistent indices to work with.
            </span><span class="kw">if </span><span class="self">self</span>.tail.index.load(Ordering::SeqCst) == tail {
                <span class="comment">// Erase the lower bits.
                </span>tail &amp;= !((<span class="number">1 </span>&lt;&lt; SHIFT) - <span class="number">1</span>);
                head &amp;= !((<span class="number">1 </span>&lt;&lt; SHIFT) - <span class="number">1</span>);

                <span class="comment">// Fix up indices if they fall onto block ends.
                </span><span class="kw">if </span>(tail &gt;&gt; SHIFT) &amp; (LAP - <span class="number">1</span>) == LAP - <span class="number">1 </span>{
                    tail = tail.wrapping_add(<span class="number">1 </span>&lt;&lt; SHIFT);
                }
                <span class="kw">if </span>(head &gt;&gt; SHIFT) &amp; (LAP - <span class="number">1</span>) == LAP - <span class="number">1 </span>{
                    head = head.wrapping_add(<span class="number">1 </span>&lt;&lt; SHIFT);
                }

                <span class="comment">// Rotate indices so that head falls into the first block.
                </span><span class="kw">let </span>lap = (head &gt;&gt; SHIFT) / LAP;
                tail = tail.wrapping_sub((lap * LAP) &lt;&lt; SHIFT);
                head = head.wrapping_sub((lap * LAP) &lt;&lt; SHIFT);

                <span class="comment">// Remove the lower bits.
                </span>tail &gt;&gt;= SHIFT;
                head &gt;&gt;= SHIFT;

                <span class="comment">// Return the difference minus the number of blocks between tail and head.
                </span><span class="kw">return </span>tail - head - tail / LAP;
            }
        }
    }

    <span class="doccomment">/// Returns the capacity of the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>capacity(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="prelude-val">None
    </span>}

    <span class="doccomment">/// Disconnects senders and wakes up all blocked receivers.
    ///
    /// Returns `true` if this call disconnected the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>disconnect_senders(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="kw">let </span>tail = <span class="self">self</span>.tail.index.fetch_or(MARK_BIT, Ordering::SeqCst);

        <span class="kw">if </span>tail &amp; MARK_BIT == <span class="number">0 </span>{
            <span class="self">self</span>.receivers.disconnect();
            <span class="bool-val">true
        </span>} <span class="kw">else </span>{
            <span class="bool-val">false
        </span>}
    }

    <span class="doccomment">/// Disconnects receivers.
    ///
    /// Returns `true` if this call disconnected the channel.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>disconnect_receivers(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="kw">let </span>tail = <span class="self">self</span>.tail.index.fetch_or(MARK_BIT, Ordering::SeqCst);

        <span class="kw">if </span>tail &amp; MARK_BIT == <span class="number">0 </span>{
            <span class="comment">// If receivers are dropped first, discard all messages to free
            // memory eagerly.
            </span><span class="self">self</span>.discard_all_messages();
            <span class="bool-val">true
        </span>} <span class="kw">else </span>{
            <span class="bool-val">false
        </span>}
    }

    <span class="doccomment">/// Discards all messages.
    ///
    /// This method should only be called when all receivers are dropped.
    </span><span class="kw">fn </span>discard_all_messages(<span class="kw-2">&amp;</span><span class="self">self</span>) {
        <span class="kw">let </span>backoff = Backoff::new();
        <span class="kw">let </span><span class="kw-2">mut </span>tail = <span class="self">self</span>.tail.index.load(Ordering::Acquire);
        <span class="kw">loop </span>{
            <span class="kw">let </span>offset = (tail &gt;&gt; SHIFT) % LAP;
            <span class="kw">if </span>offset != BLOCK_CAP {
                <span class="kw">break</span>;
            }

            <span class="comment">// New updates to tail will be rejected by MARK_BIT and aborted unless it's
            // at boundary. We need to wait for the updates take affect otherwise there
            // can be memory leaks.
            </span>backoff.snooze();
            tail = <span class="self">self</span>.tail.index.load(Ordering::Acquire);
        }

        <span class="kw">let </span><span class="kw-2">mut </span>head = <span class="self">self</span>.head.index.load(Ordering::Acquire);
        <span class="kw">let </span><span class="kw-2">mut </span>block = <span class="self">self</span>.head.block.swap(ptr::null_mut(), Ordering::AcqRel);

        <span class="comment">// If we're going to be dropping messages we need to synchronize with initialization
        </span><span class="kw">if </span>head &gt;&gt; SHIFT != tail &gt;&gt; SHIFT {
            <span class="comment">// The block can be null here only if a sender is in the process of initializing the
            // channel while another sender managed to send a message by inserting it into the
            // semi-initialized channel and advanced the tail.
            // In that case, just wait until it gets initialized.
            </span><span class="kw">while </span>block.is_null() {
                backoff.snooze();
                block = <span class="self">self</span>.head.block.load(Ordering::Acquire);
            }
        }

        <span class="kw">unsafe </span>{
            <span class="comment">// Drop all messages between head and tail and deallocate the heap-allocated blocks.
            </span><span class="kw">while </span>head &gt;&gt; SHIFT != tail &gt;&gt; SHIFT {
                <span class="kw">let </span>offset = (head &gt;&gt; SHIFT) % LAP;

                <span class="kw">if </span>offset &lt; BLOCK_CAP {
                    <span class="comment">// Drop the message in the slot.
                    </span><span class="kw">let </span>slot = (<span class="kw-2">*</span>block).slots.get_unchecked(offset);
                    slot.wait_write();
                    (<span class="kw-2">*</span>slot.msg.get()).assume_init_drop();
                } <span class="kw">else </span>{
                    (<span class="kw-2">*</span>block).wait_next();
                    <span class="comment">// Deallocate the block and move to the next one.
                    </span><span class="kw">let </span>next = (<span class="kw-2">*</span>block).next.load(Ordering::Acquire);
                    drop(Box::from_raw(block));
                    block = next;
                }

                head = head.wrapping_add(<span class="number">1 </span>&lt;&lt; SHIFT);
            }

            <span class="comment">// Deallocate the last remaining block.
            </span><span class="kw">if </span>!block.is_null() {
                drop(Box::from_raw(block));
            }
        }
        head &amp;= !MARK_BIT;
        <span class="self">self</span>.head.index.store(head, Ordering::Release);
    }

    <span class="doccomment">/// Returns `true` if the channel is disconnected.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_disconnected(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.tail.index.load(Ordering::SeqCst) &amp; MARK_BIT != <span class="number">0
    </span>}

    <span class="doccomment">/// Returns `true` if the channel is empty.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_empty(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="kw">let </span>head = <span class="self">self</span>.head.index.load(Ordering::SeqCst);
        <span class="kw">let </span>tail = <span class="self">self</span>.tail.index.load(Ordering::SeqCst);
        head &gt;&gt; SHIFT == tail &gt;&gt; SHIFT
    }

    <span class="doccomment">/// Returns `true` if the channel is full.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_full(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="bool-val">false
    </span>}
}

<span class="kw">impl</span>&lt;T&gt; Drop <span class="kw">for </span>Channel&lt;T&gt; {
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">let </span><span class="kw-2">mut </span>head = <span class="kw-2">*</span><span class="self">self</span>.head.index.get_mut();
        <span class="kw">let </span><span class="kw-2">mut </span>tail = <span class="kw-2">*</span><span class="self">self</span>.tail.index.get_mut();
        <span class="kw">let </span><span class="kw-2">mut </span>block = <span class="kw-2">*</span><span class="self">self</span>.head.block.get_mut();

        <span class="comment">// Erase the lower bits.
        </span>head &amp;= !((<span class="number">1 </span>&lt;&lt; SHIFT) - <span class="number">1</span>);
        tail &amp;= !((<span class="number">1 </span>&lt;&lt; SHIFT) - <span class="number">1</span>);

        <span class="kw">unsafe </span>{
            <span class="comment">// Drop all messages between head and tail and deallocate the heap-allocated blocks.
            </span><span class="kw">while </span>head != tail {
                <span class="kw">let </span>offset = (head &gt;&gt; SHIFT) % LAP;

                <span class="kw">if </span>offset &lt; BLOCK_CAP {
                    <span class="comment">// Drop the message in the slot.
                    </span><span class="kw">let </span>slot = (<span class="kw-2">*</span>block).slots.get_unchecked(offset);
                    (<span class="kw-2">*</span>slot.msg.get()).assume_init_drop();
                } <span class="kw">else </span>{
                    <span class="comment">// Deallocate the block and move to the next one.
                    </span><span class="kw">let </span>next = <span class="kw-2">*</span>(<span class="kw-2">*</span>block).next.get_mut();
                    drop(Box::from_raw(block));
                    block = next;
                }

                head = head.wrapping_add(<span class="number">1 </span>&lt;&lt; SHIFT);
            }

            <span class="comment">// Deallocate the last remaining block.
            </span><span class="kw">if </span>!block.is_null() {
                drop(Box::from_raw(block));
            }
        }
    }
}

<span class="doccomment">/// Receiver handle to a channel.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Receiver&lt;<span class="lifetime">'a</span>, T&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span>Channel&lt;T&gt;);

<span class="doccomment">/// Sender handle to a channel.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Sender&lt;<span class="lifetime">'a</span>, T&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span>Channel&lt;T&gt;);

<span class="kw">impl</span>&lt;T&gt; SelectHandle <span class="kw">for </span>Receiver&lt;<span class="lifetime">'_</span>, T&gt; {
    <span class="kw">fn </span>try_select(<span class="kw-2">&amp;</span><span class="self">self</span>, token: <span class="kw-2">&amp;mut </span>Token) -&gt; bool {
        <span class="self">self</span>.<span class="number">0</span>.start_recv(token)
    }

    <span class="kw">fn </span>deadline(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Instant&gt; {
        <span class="prelude-val">None
    </span>}

    <span class="kw">fn </span>register(<span class="kw-2">&amp;</span><span class="self">self</span>, oper: Operation, cx: <span class="kw-2">&amp;</span>Context) -&gt; bool {
        <span class="self">self</span>.<span class="number">0</span>.receivers.register(oper, cx);
        <span class="self">self</span>.is_ready()
    }

    <span class="kw">fn </span>unregister(<span class="kw-2">&amp;</span><span class="self">self</span>, oper: Operation) {
        <span class="self">self</span>.<span class="number">0</span>.receivers.unregister(oper);
    }

    <span class="kw">fn </span>accept(<span class="kw-2">&amp;</span><span class="self">self</span>, token: <span class="kw-2">&amp;mut </span>Token, _cx: <span class="kw-2">&amp;</span>Context) -&gt; bool {
        <span class="self">self</span>.try_select(token)
    }

    <span class="kw">fn </span>is_ready(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        !<span class="self">self</span>.<span class="number">0</span>.is_empty() || <span class="self">self</span>.<span class="number">0</span>.is_disconnected()
    }

    <span class="kw">fn </span>watch(<span class="kw-2">&amp;</span><span class="self">self</span>, oper: Operation, cx: <span class="kw-2">&amp;</span>Context) -&gt; bool {
        <span class="self">self</span>.<span class="number">0</span>.receivers.watch(oper, cx);
        <span class="self">self</span>.is_ready()
    }

    <span class="kw">fn </span>unwatch(<span class="kw-2">&amp;</span><span class="self">self</span>, oper: Operation) {
        <span class="self">self</span>.<span class="number">0</span>.receivers.unwatch(oper);
    }
}

<span class="kw">impl</span>&lt;T&gt; SelectHandle <span class="kw">for </span>Sender&lt;<span class="lifetime">'_</span>, T&gt; {
    <span class="kw">fn </span>try_select(<span class="kw-2">&amp;</span><span class="self">self</span>, token: <span class="kw-2">&amp;mut </span>Token) -&gt; bool {
        <span class="self">self</span>.<span class="number">0</span>.start_send(token)
    }

    <span class="kw">fn </span>deadline(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Instant&gt; {
        <span class="prelude-val">None
    </span>}

    <span class="kw">fn </span>register(<span class="kw-2">&amp;</span><span class="self">self</span>, _oper: Operation, _cx: <span class="kw-2">&amp;</span>Context) -&gt; bool {
        <span class="self">self</span>.is_ready()
    }

    <span class="kw">fn </span>unregister(<span class="kw-2">&amp;</span><span class="self">self</span>, _oper: Operation) {}

    <span class="kw">fn </span>accept(<span class="kw-2">&amp;</span><span class="self">self</span>, token: <span class="kw-2">&amp;mut </span>Token, _cx: <span class="kw-2">&amp;</span>Context) -&gt; bool {
        <span class="self">self</span>.try_select(token)
    }

    <span class="kw">fn </span>is_ready(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="bool-val">true
    </span>}

    <span class="kw">fn </span>watch(<span class="kw-2">&amp;</span><span class="self">self</span>, _oper: Operation, _cx: <span class="kw-2">&amp;</span>Context) -&gt; bool {
        <span class="self">self</span>.is_ready()
    }

    <span class="kw">fn </span>unwatch(<span class="kw-2">&amp;</span><span class="self">self</span>, _oper: Operation) {}
}
</code></pre></div></section></main></body></html>