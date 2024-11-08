<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.116/src/error.rs`."><title>error.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="serde_json" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../serde_json/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! When serializing or deserializing JSON goes wrong.

</span><span class="kw">use </span><span class="kw">crate</span>::io;
<span class="kw">use </span>alloc::boxed::Box;
<span class="kw">use </span>alloc::string::{String, ToString};
<span class="kw">use </span>core::fmt::{<span class="self">self</span>, Debug, Display};
<span class="kw">use </span>core::result;
<span class="kw">use </span>core::str::FromStr;
<span class="kw">use </span>serde::{de, ser};
<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">use </span>std::error;
<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">use </span>std::io::ErrorKind;

<span class="doccomment">/// This type represents all possible errors that can occur when serializing or
/// deserializing JSON data.
</span><span class="kw">pub struct </span>Error {
    <span class="doccomment">/// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result&lt;T, Error&gt;`.
    </span>err: Box&lt;ErrorImpl&gt;,
}

<span class="doccomment">/// Alias for a `Result` with the error type `serde_json::Error`.
</span><span class="kw">pub type </span><span class="prelude-ty">Result</span>&lt;T&gt; = result::Result&lt;T, Error&gt;;

<span class="kw">impl </span>Error {
    <span class="doccomment">/// One-based line number at which the error was detected.
    ///
    /// Characters in the first line of the input (before the first newline
    /// character) are in line 1.
    </span><span class="kw">pub fn </span>line(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.err.line
    }

    <span class="doccomment">/// One-based column number at which the error was detected.
    ///
    /// The first character in the input and any characters immediately
    /// following a newline character are in column 1.
    ///
    /// Note that errors may occur in column 0, for example if a read from an
    /// I/O stream fails immediately following a previously read newline
    /// character.
    </span><span class="kw">pub fn </span>column(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.err.column
    }

    <span class="doccomment">/// Categorizes the cause of this error.
    ///
    /// - `Category::Io` - failure to read or write bytes on an I/O stream
    /// - `Category::Syntax` - input that is not syntactically valid JSON
    /// - `Category::Data` - input data that is semantically incorrect
    /// - `Category::Eof` - unexpected end of the input data
    </span><span class="kw">pub fn </span>classify(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Category {
        <span class="kw">match </span><span class="self">self</span>.err.code {
            ErrorCode::Message(<span class="kw">_</span>) =&gt; Category::Data,
            ErrorCode::Io(<span class="kw">_</span>) =&gt; Category::Io,
            ErrorCode::EofWhileParsingList
            | ErrorCode::EofWhileParsingObject
            | ErrorCode::EofWhileParsingString
            | ErrorCode::EofWhileParsingValue =&gt; Category::Eof,
            ErrorCode::ExpectedColon
            | ErrorCode::ExpectedListCommaOrEnd
            | ErrorCode::ExpectedObjectCommaOrEnd
            | ErrorCode::ExpectedSomeIdent
            | ErrorCode::ExpectedSomeValue
            | ErrorCode::ExpectedDoubleQuote
            | ErrorCode::InvalidEscape
            | ErrorCode::InvalidNumber
            | ErrorCode::NumberOutOfRange
            | ErrorCode::InvalidUnicodeCodePoint
            | ErrorCode::ControlCharacterWhileParsingString
            | ErrorCode::KeyMustBeAString
            | ErrorCode::ExpectedNumericKey
            | ErrorCode::FloatKeyMustBeFinite
            | ErrorCode::LoneLeadingSurrogateInHexEscape
            | ErrorCode::TrailingComma
            | ErrorCode::TrailingCharacters
            | ErrorCode::UnexpectedEndOfHexEscape
            | ErrorCode::RecursionLimitExceeded =&gt; Category::Syntax,
        }
    }

    <span class="doccomment">/// Returns true if this error was caused by a failure to read or write
    /// bytes on an I/O stream.
    </span><span class="kw">pub fn </span>is_io(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.classify() == Category::Io
    }

    <span class="doccomment">/// Returns true if this error was caused by input that was not
    /// syntactically valid JSON.
    </span><span class="kw">pub fn </span>is_syntax(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.classify() == Category::Syntax
    }

    <span class="doccomment">/// Returns true if this error was caused by input data that was
    /// semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect when the
    /// type being deserialized into holds a String.
    </span><span class="kw">pub fn </span>is_data(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.classify() == Category::Data
    }

    <span class="doccomment">/// Returns true if this error was caused by prematurely reaching the end of
    /// the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    </span><span class="kw">pub fn </span>is_eof(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.classify() == Category::Eof
    }

    <span class="doccomment">/// The kind reported by the underlying standard library I/O error, if this
    /// error was caused by a failure to read or write bytes on an I/O stream.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::Value;
    /// use std::io::{self, ErrorKind, Read};
    /// use std::process;
    ///
    /// struct ReaderThatWillTimeOut&lt;'a&gt;(&amp;'a [u8]);
    ///
    /// impl&lt;'a&gt; Read for ReaderThatWillTimeOut&lt;'a&gt; {
    ///     fn read(&amp;mut self, buf: &amp;mut [u8]) -&gt; io::Result&lt;usize&gt; {
    ///         if self.0.is_empty() {
    ///             Err(io::Error::new(ErrorKind::TimedOut, "timed out"))
    ///         } else {
    ///             self.0.read(buf)
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let reader = ReaderThatWillTimeOut(br#" {"k": "#);
    ///
    ///     let _: Value = match serde_json::from_reader(reader) {
    ///         Ok(value) =&gt; value,
    ///         Err(error) =&gt; {
    ///             if error.io_error_kind() == Some(ErrorKind::TimedOut) {
    ///                 // Maybe this application needs to retry certain kinds of errors.
    ///
    ///                 # return;
    ///             } else {
    ///                 eprintln!("error: {}", error);
    ///                 process::exit(1);
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    </span><span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
    </span><span class="kw">pub fn </span>io_error_kind(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;ErrorKind&gt; {
        <span class="kw">if let </span>ErrorCode::Io(io_error) = <span class="kw-2">&amp;</span><span class="self">self</span>.err.code {
            <span class="prelude-val">Some</span>(io_error.kind())
        } <span class="kw">else </span>{
            <span class="prelude-val">None
        </span>}
    }
}

<span class="doccomment">/// Categorizes the cause of a `serde_json::Error`.
</span><span class="attr">#[derive(Copy, Clone, PartialEq, Eq, Debug)]
</span><span class="kw">pub enum </span>Category {
    <span class="doccomment">/// The error was caused by a failure to read or write bytes on an I/O
    /// stream.
    </span>Io,

    <span class="doccomment">/// The error was caused by input that was not syntactically valid JSON.
    </span>Syntax,

    <span class="doccomment">/// The error was caused by input data that was semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect when the
    /// type being deserialized into holds a String.
    </span>Data,

    <span class="doccomment">/// The error was caused by prematurely reaching the end of the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    </span>Eof,
}

<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
#[allow(clippy::fallible_impl_from)]
</span><span class="kw">impl </span>From&lt;Error&gt; <span class="kw">for </span>io::Error {
    <span class="doccomment">/// Convert a `serde_json::Error` into an `io::Error`.
    ///
    /// JSON syntax and data errors are turned into `InvalidData` I/O errors.
    /// EOF errors are turned into `UnexpectedEof` I/O errors.
    ///
    /// ```
    /// use std::io;
    ///
    /// enum MyError {
    ///     Io(io::Error),
    ///     Json(serde_json::Error),
    /// }
    ///
    /// impl From&lt;serde_json::Error&gt; for MyError {
    ///     fn from(err: serde_json::Error) -&gt; MyError {
    ///         use serde_json::error::Category;
    ///         match err.classify() {
    ///             Category::Io =&gt; {
    ///                 MyError::Io(err.into())
    ///             }
    ///             Category::Syntax | Category::Data | Category::Eof =&gt; {
    ///                 MyError::Json(err)
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    </span><span class="kw">fn </span>from(j: Error) -&gt; <span class="self">Self </span>{
        <span class="kw">if let </span>ErrorCode::Io(err) = j.err.code {
            err
        } <span class="kw">else </span>{
            <span class="kw">match </span>j.classify() {
                Category::Io =&gt; <span class="macro">unreachable!</span>(),
                Category::Syntax | Category::Data =&gt; io::Error::new(ErrorKind::InvalidData, j),
                Category::Eof =&gt; io::Error::new(ErrorKind::UnexpectedEof, j),
            }
        }
    }
}

<span class="kw">struct </span>ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
}

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">enum </span>ErrorCode {
    <span class="doccomment">/// Catchall for syntax error messages
    </span>Message(Box&lt;str&gt;),

    <span class="doccomment">/// Some I/O error occurred while serializing or deserializing.
    </span>Io(io::Error),

    <span class="doccomment">/// EOF while parsing a list.
    </span>EofWhileParsingList,

    <span class="doccomment">/// EOF while parsing an object.
    </span>EofWhileParsingObject,

    <span class="doccomment">/// EOF while parsing a string.
    </span>EofWhileParsingString,

    <span class="doccomment">/// EOF while parsing a JSON value.
    </span>EofWhileParsingValue,

    <span class="doccomment">/// Expected this character to be a `':'`.
    </span>ExpectedColon,

    <span class="doccomment">/// Expected this character to be either a `','` or a `']'`.
    </span>ExpectedListCommaOrEnd,

    <span class="doccomment">/// Expected this character to be either a `','` or a `'}'`.
    </span>ExpectedObjectCommaOrEnd,

    <span class="doccomment">/// Expected to parse either a `true`, `false`, or a `null`.
    </span>ExpectedSomeIdent,

    <span class="doccomment">/// Expected this character to start a JSON value.
    </span>ExpectedSomeValue,

    <span class="doccomment">/// Expected this character to be a `"`.
    </span>ExpectedDoubleQuote,

    <span class="doccomment">/// Invalid hex escape code.
    </span>InvalidEscape,

    <span class="doccomment">/// Invalid number.
    </span>InvalidNumber,

    <span class="doccomment">/// Number is bigger than the maximum value of its type.
    </span>NumberOutOfRange,

    <span class="doccomment">/// Invalid unicode code point.
    </span>InvalidUnicodeCodePoint,

    <span class="doccomment">/// Control character found while parsing a string.
    </span>ControlCharacterWhileParsingString,

    <span class="doccomment">/// Object key is not a string.
    </span>KeyMustBeAString,

    <span class="doccomment">/// Contents of key were supposed to be a number.
    </span>ExpectedNumericKey,

    <span class="doccomment">/// Object key is a non-finite float value.
    </span>FloatKeyMustBeFinite,

    <span class="doccomment">/// Lone leading surrogate in hex escape.
    </span>LoneLeadingSurrogateInHexEscape,

    <span class="doccomment">/// JSON has a comma after the last value in an array or map.
    </span>TrailingComma,

    <span class="doccomment">/// JSON has non-whitespace trailing characters after the value.
    </span>TrailingCharacters,

    <span class="doccomment">/// Unexpected end of hex escape.
    </span>UnexpectedEndOfHexEscape,

    <span class="doccomment">/// Encountered nesting of JSON maps and arrays more than 128 layers deep.
    </span>RecursionLimitExceeded,
}

<span class="kw">impl </span>Error {
    <span class="attr">#[cold]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>syntax(code: ErrorCode, line: usize, column: usize) -&gt; <span class="self">Self </span>{
        Error {
            err: Box::new(ErrorImpl { code, line, column }),
        }
    }

    <span class="comment">// Not public API. Should be pub(crate).
    //
    // Update `eager_json` crate when this function changes.
    </span><span class="attr">#[doc(hidden)]
    #[cold]
    </span><span class="kw">pub fn </span>io(error: io::Error) -&gt; <span class="self">Self </span>{
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(error),
                line: <span class="number">0</span>,
                column: <span class="number">0</span>,
            }),
        }
    }

    <span class="attr">#[cold]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>fix_position&lt;F&gt;(<span class="self">self</span>, f: F) -&gt; <span class="self">Self
    </span><span class="kw">where
        </span>F: FnOnce(ErrorCode) -&gt; Error,
    {
        <span class="kw">if </span><span class="self">self</span>.err.line == <span class="number">0 </span>{
            f(<span class="self">self</span>.err.code)
        } <span class="kw">else </span>{
            <span class="self">self
        </span>}
    }
}

<span class="kw">impl </span>Display <span class="kw">for </span>ErrorCode {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">match </span><span class="self">self </span>{
            ErrorCode::Message(msg) =&gt; f.write_str(msg),
            ErrorCode::Io(err) =&gt; Display::fmt(err, f),
            ErrorCode::EofWhileParsingList =&gt; f.write_str(<span class="string">"EOF while parsing a list"</span>),
            ErrorCode::EofWhileParsingObject =&gt; f.write_str(<span class="string">"EOF while parsing an object"</span>),
            ErrorCode::EofWhileParsingString =&gt; f.write_str(<span class="string">"EOF while parsing a string"</span>),
            ErrorCode::EofWhileParsingValue =&gt; f.write_str(<span class="string">"EOF while parsing a value"</span>),
            ErrorCode::ExpectedColon =&gt; f.write_str(<span class="string">"expected `:`"</span>),
            ErrorCode::ExpectedListCommaOrEnd =&gt; f.write_str(<span class="string">"expected `,` or `]`"</span>),
            ErrorCode::ExpectedObjectCommaOrEnd =&gt; f.write_str(<span class="string">"expected `,` or `}`"</span>),
            ErrorCode::ExpectedSomeIdent =&gt; f.write_str(<span class="string">"expected ident"</span>),
            ErrorCode::ExpectedSomeValue =&gt; f.write_str(<span class="string">"expected value"</span>),
            ErrorCode::ExpectedDoubleQuote =&gt; f.write_str(<span class="string">"expected `\"`"</span>),
            ErrorCode::InvalidEscape =&gt; f.write_str(<span class="string">"invalid escape"</span>),
            ErrorCode::InvalidNumber =&gt; f.write_str(<span class="string">"invalid number"</span>),
            ErrorCode::NumberOutOfRange =&gt; f.write_str(<span class="string">"number out of range"</span>),
            ErrorCode::InvalidUnicodeCodePoint =&gt; f.write_str(<span class="string">"invalid unicode code point"</span>),
            ErrorCode::ControlCharacterWhileParsingString =&gt; {
                f.write_str(<span class="string">"control character (\\u0000-\\u001F) found while parsing a string"</span>)
            }
            ErrorCode::KeyMustBeAString =&gt; f.write_str(<span class="string">"key must be a string"</span>),
            ErrorCode::ExpectedNumericKey =&gt; {
                f.write_str(<span class="string">"invalid value: expected key to be a number in quotes"</span>)
            }
            ErrorCode::FloatKeyMustBeFinite =&gt; {
                f.write_str(<span class="string">"float key must be finite (got NaN or +/-inf)"</span>)
            }
            ErrorCode::LoneLeadingSurrogateInHexEscape =&gt; {
                f.write_str(<span class="string">"lone leading surrogate in hex escape"</span>)
            }
            ErrorCode::TrailingComma =&gt; f.write_str(<span class="string">"trailing comma"</span>),
            ErrorCode::TrailingCharacters =&gt; f.write_str(<span class="string">"trailing characters"</span>),
            ErrorCode::UnexpectedEndOfHexEscape =&gt; f.write_str(<span class="string">"unexpected end of hex escape"</span>),
            ErrorCode::RecursionLimitExceeded =&gt; f.write_str(<span class="string">"recursion limit exceeded"</span>),
        }
    }
}

<span class="kw">impl </span>serde::de::StdError <span class="kw">for </span>Error {
    <span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
    </span><span class="kw">fn </span>source(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>(<span class="kw">dyn </span>error::Error + <span class="lifetime">'static</span>)&gt; {
        <span class="kw">match </span><span class="kw-2">&amp;</span><span class="self">self</span>.err.code {
            ErrorCode::Io(err) =&gt; err.source(),
            <span class="kw">_ </span>=&gt; <span class="prelude-val">None</span>,
        }
    }
}

<span class="kw">impl </span>Display <span class="kw">for </span>Error {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        Display::fmt(<span class="kw-2">&amp;*</span><span class="self">self</span>.err, f)
    }
}

<span class="kw">impl </span>Display <span class="kw">for </span>ErrorImpl {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">if </span><span class="self">self</span>.line == <span class="number">0 </span>{
            Display::fmt(<span class="kw-2">&amp;</span><span class="self">self</span>.code, f)
        } <span class="kw">else </span>{
            <span class="macro">write!</span>(
                f,
                <span class="string">"{} at line {} column {}"</span>,
                <span class="self">self</span>.code, <span class="self">self</span>.line, <span class="self">self</span>.column
            )
        }
    }
}

<span class="comment">// Remove two layers of verbosity from the debug representation. Humans often
// end up seeing this representation because it is what unwrap() shows.
</span><span class="kw">impl </span>Debug <span class="kw">for </span>Error {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="macro">write!</span>(
            f,
            <span class="string">"Error({:?}, line: {}, column: {})"</span>,
            <span class="self">self</span>.err.code.to_string(),
            <span class="self">self</span>.err.line,
            <span class="self">self</span>.err.column
        )
    }
}

<span class="kw">impl </span>de::Error <span class="kw">for </span>Error {
    <span class="attr">#[cold]
    </span><span class="kw">fn </span>custom&lt;T: Display&gt;(msg: T) -&gt; Error {
        make_error(msg.to_string())
    }

    <span class="attr">#[cold]
    </span><span class="kw">fn </span>invalid_type(unexp: de::Unexpected, exp: <span class="kw-2">&amp;</span><span class="kw">dyn </span>de::Expected) -&gt; <span class="self">Self </span>{
        Error::custom(<span class="macro">format_args!</span>(
            <span class="string">"invalid type: {}, expected {}"</span>,
            JsonUnexpected(unexp),
            exp,
        ))
    }

    <span class="attr">#[cold]
    </span><span class="kw">fn </span>invalid_value(unexp: de::Unexpected, exp: <span class="kw-2">&amp;</span><span class="kw">dyn </span>de::Expected) -&gt; <span class="self">Self </span>{
        Error::custom(<span class="macro">format_args!</span>(
            <span class="string">"invalid value: {}, expected {}"</span>,
            JsonUnexpected(unexp),
            exp,
        ))
    }
}

<span class="kw">impl </span>ser::Error <span class="kw">for </span>Error {
    <span class="attr">#[cold]
    </span><span class="kw">fn </span>custom&lt;T: Display&gt;(msg: T) -&gt; Error {
        make_error(msg.to_string())
    }
}

<span class="kw">struct </span>JsonUnexpected&lt;<span class="lifetime">'a</span>&gt;(de::Unexpected&lt;<span class="lifetime">'a</span>&gt;);

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Display <span class="kw">for </span>JsonUnexpected&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, formatter: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0 </span>{
            de::Unexpected::Unit =&gt; formatter.write_str(<span class="string">"null"</span>),
            de::Unexpected::Float(value) =&gt; <span class="macro">write!</span>(
                formatter,
                <span class="string">"floating point `{}`"</span>,
                ryu::Buffer::new().format(value),
            ),
            unexp =&gt; Display::fmt(<span class="kw-2">&amp;</span>unexp, formatter),
        }
    }
}

<span class="comment">// Parse our own error message that looks like "{} at line {} column {}" to work
// around erased-serde round-tripping the error through de::Error::custom.
</span><span class="kw">fn </span>make_error(<span class="kw-2">mut </span>msg: String) -&gt; Error {
    <span class="kw">let </span>(line, column) = parse_line_col(<span class="kw-2">&amp;mut </span>msg).unwrap_or((<span class="number">0</span>, <span class="number">0</span>));
    Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message(msg.into_boxed_str()),
            line,
            column,
        }),
    }
}

<span class="kw">fn </span>parse_line_col(msg: <span class="kw-2">&amp;mut </span>String) -&gt; <span class="prelude-ty">Option</span>&lt;(usize, usize)&gt; {
    <span class="kw">let </span>start_of_suffix = <span class="kw">match </span>msg.rfind(<span class="string">" at line "</span>) {
        <span class="prelude-val">Some</span>(index) =&gt; index,
        <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">None</span>,
    };

    <span class="comment">// Find start and end of line number.
    </span><span class="kw">let </span>start_of_line = start_of_suffix + <span class="string">" at line "</span>.len();
    <span class="kw">let </span><span class="kw-2">mut </span>end_of_line = start_of_line;
    <span class="kw">while </span>starts_with_digit(<span class="kw-2">&amp;</span>msg[end_of_line..]) {
        end_of_line += <span class="number">1</span>;
    }

    <span class="kw">if </span>!msg[end_of_line..].starts_with(<span class="string">" column "</span>) {
        <span class="kw">return </span><span class="prelude-val">None</span>;
    }

    <span class="comment">// Find start and end of column number.
    </span><span class="kw">let </span>start_of_column = end_of_line + <span class="string">" column "</span>.len();
    <span class="kw">let </span><span class="kw-2">mut </span>end_of_column = start_of_column;
    <span class="kw">while </span>starts_with_digit(<span class="kw-2">&amp;</span>msg[end_of_column..]) {
        end_of_column += <span class="number">1</span>;
    }

    <span class="kw">if </span>end_of_column &lt; msg.len() {
        <span class="kw">return </span><span class="prelude-val">None</span>;
    }

    <span class="comment">// Parse numbers.
    </span><span class="kw">let </span>line = <span class="kw">match </span>usize::from_str(<span class="kw-2">&amp;</span>msg[start_of_line..end_of_line]) {
        <span class="prelude-val">Ok</span>(line) =&gt; line,
        <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; <span class="kw">return </span><span class="prelude-val">None</span>,
    };
    <span class="kw">let </span>column = <span class="kw">match </span>usize::from_str(<span class="kw-2">&amp;</span>msg[start_of_column..end_of_column]) {
        <span class="prelude-val">Ok</span>(column) =&gt; column,
        <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; <span class="kw">return </span><span class="prelude-val">None</span>,
    };

    msg.truncate(start_of_suffix);
    <span class="prelude-val">Some</span>((line, column))
}

<span class="kw">fn </span>starts_with_digit(slice: <span class="kw-2">&amp;</span>str) -&gt; bool {
    <span class="kw">match </span>slice.as_bytes().first() {
        <span class="prelude-val">None </span>=&gt; <span class="bool-val">false</span>,
        <span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span>byte) =&gt; byte &gt;= <span class="string">b'0' </span>&amp;&amp; byte &lt;= <span class="string">b'9'</span>,
    }
}
</code></pre></div></section></main></body></html>