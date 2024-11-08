<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/goblin-0.5.4/src/elf/constants_header.rs`."><title>constants_header.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="goblin" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../goblin/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// sweet emacs regexp
// pub const \([[:word:]|_]*\)[[:space:]]*\([[:digit:]]+\)[[:space:]]*/\*\(.*\) \*/
// \\\\3 C-q C-j pub const \1: u32 = \2;

</span><span class="doccomment">/// TODO: use Enum with explicit discriminant and get debug printer for free?

/// No machine
</span><span class="kw">pub const </span>EM_NONE: u16 = <span class="number">0</span>;
<span class="doccomment">/// AT&amp;T WE 32100
</span><span class="kw">pub const </span>EM_M32: u16 = <span class="number">1</span>;
<span class="doccomment">/// SUN SPARC
</span><span class="kw">pub const </span>EM_SPARC: u16 = <span class="number">2</span>;
<span class="doccomment">/// Intel 80386
</span><span class="kw">pub const </span>EM_386: u16 = <span class="number">3</span>;
<span class="doccomment">/// Motorola m68k family
</span><span class="kw">pub const </span>EM_68K: u16 = <span class="number">4</span>;
<span class="doccomment">/// Motorola m88k family
</span><span class="kw">pub const </span>EM_88K: u16 = <span class="number">5</span>;
<span class="doccomment">/// Intel MCU
</span><span class="kw">pub const </span>EM_IAMCU: u16 = <span class="number">6</span>;
<span class="doccomment">/// Intel 80860
</span><span class="kw">pub const </span>EM_860: u16 = <span class="number">7</span>;
<span class="doccomment">/// MIPS R3000 big-endian
</span><span class="kw">pub const </span>EM_MIPS: u16 = <span class="number">8</span>;
<span class="doccomment">/// IBM System/370
</span><span class="kw">pub const </span>EM_S370: u16 = <span class="number">9</span>;
<span class="doccomment">/// MIPS R3000 little-endian
</span><span class="kw">pub const </span>EM_MIPS_RS3_LE: u16 = <span class="number">10</span>;
<span class="comment">// reserved 11-14
</span><span class="doccomment">/// HPPA
</span><span class="kw">pub const </span>EM_PARISC: u16 = <span class="number">15</span>;
<span class="comment">// reserved 16
</span><span class="doccomment">/// Fujitsu VPP500
</span><span class="kw">pub const </span>EM_VPP500: u16 = <span class="number">17</span>;
<span class="doccomment">/// Sun's "v8plus"
</span><span class="kw">pub const </span>EM_SPARC32PLUS: u16 = <span class="number">18</span>;
<span class="doccomment">/// Intel 80960
</span><span class="kw">pub const </span>EM_960: u16 = <span class="number">19</span>;
<span class="doccomment">/// PowerPC
</span><span class="kw">pub const </span>EM_PPC: u16 = <span class="number">20</span>;
<span class="doccomment">/// PowerPC 64-bit
</span><span class="kw">pub const </span>EM_PPC64: u16 = <span class="number">21</span>;
<span class="doccomment">/// IBM S390
</span><span class="kw">pub const </span>EM_S390: u16 = <span class="number">22</span>;
<span class="doccomment">/// IBM SPU/SPC
</span><span class="kw">pub const </span>EM_SPU: u16 = <span class="number">23</span>;
<span class="comment">// reserved 24-35
</span><span class="doccomment">/// NEC V800 series
</span><span class="kw">pub const </span>EM_V800: u16 = <span class="number">36</span>;
<span class="doccomment">/// Fujitsu FR20
</span><span class="kw">pub const </span>EM_FR20: u16 = <span class="number">37</span>;
<span class="doccomment">/// TRW RH-32
</span><span class="kw">pub const </span>EM_RH32: u16 = <span class="number">38</span>;
<span class="doccomment">/// Motorola RCE
</span><span class="kw">pub const </span>EM_RCE: u16 = <span class="number">39</span>;
<span class="doccomment">/// ARM
</span><span class="kw">pub const </span>EM_ARM: u16 = <span class="number">40</span>;
<span class="doccomment">/// Digital Alpha
</span><span class="kw">pub const </span>EM_FAKE_ALPHA: u16 = <span class="number">41</span>;
<span class="doccomment">/// Hitachi SH
</span><span class="kw">pub const </span>EM_SH: u16 = <span class="number">42</span>;
<span class="doccomment">/// SPARC v9 64-bit
</span><span class="kw">pub const </span>EM_SPARCV9: u16 = <span class="number">43</span>;
<span class="doccomment">/// Siemens Tricore
</span><span class="kw">pub const </span>EM_TRICORE: u16 = <span class="number">44</span>;
<span class="doccomment">/// Argonaut RISC Core
</span><span class="kw">pub const </span>EM_ARC: u16 = <span class="number">45</span>;
<span class="doccomment">/// Hitachi H8/300
</span><span class="kw">pub const </span>EM_H8_300: u16 = <span class="number">46</span>;
<span class="doccomment">/// Hitachi H8/300H
</span><span class="kw">pub const </span>EM_H8_300H: u16 = <span class="number">47</span>;
<span class="doccomment">/// Hitachi H8S
</span><span class="kw">pub const </span>EM_H8S: u16 = <span class="number">48</span>;
<span class="doccomment">/// Hitachi H8/500
</span><span class="kw">pub const </span>EM_H8_500: u16 = <span class="number">49</span>;
<span class="doccomment">/// Intel Merced
</span><span class="kw">pub const </span>EM_IA_64: u16 = <span class="number">50</span>;
<span class="doccomment">/// Stanford MIPS-X
</span><span class="kw">pub const </span>EM_MIPS_X: u16 = <span class="number">51</span>;
<span class="doccomment">/// Motorola Coldfire
</span><span class="kw">pub const </span>EM_COLDFIRE: u16 = <span class="number">52</span>;
<span class="doccomment">/// Motorola M68HC12
</span><span class="kw">pub const </span>EM_68HC12: u16 = <span class="number">53</span>;
<span class="doccomment">/// Fujitsu MMA Multimedia Accelerator
</span><span class="kw">pub const </span>EM_MMA: u16 = <span class="number">54</span>;
<span class="doccomment">/// Siemens PCP
</span><span class="kw">pub const </span>EM_PCP: u16 = <span class="number">55</span>;
<span class="doccomment">/// Sony nCPU embeeded RISC
</span><span class="kw">pub const </span>EM_NCPU: u16 = <span class="number">56</span>;
<span class="doccomment">/// Denso NDR1 microprocessor
</span><span class="kw">pub const </span>EM_NDR1: u16 = <span class="number">57</span>;
<span class="doccomment">/// Motorola Start*Core processor
</span><span class="kw">pub const </span>EM_STARCORE: u16 = <span class="number">58</span>;
<span class="doccomment">/// Toyota ME16 processor
</span><span class="kw">pub const </span>EM_ME16: u16 = <span class="number">59</span>;
<span class="doccomment">/// STMicroelectronic ST100 processor
</span><span class="kw">pub const </span>EM_ST100: u16 = <span class="number">60</span>;
<span class="doccomment">/// Advanced Logic Corp. Tinyj emb.fam
</span><span class="kw">pub const </span>EM_TINYJ: u16 = <span class="number">61</span>;
<span class="doccomment">/// AMD x86-64 architecture
</span><span class="kw">pub const </span>EM_X86_64: u16 = <span class="number">62</span>;
<span class="doccomment">/// Sony DSP Processor
</span><span class="kw">pub const </span>EM_PDSP: u16 = <span class="number">63</span>;
<span class="doccomment">/// Digital PDP-10
</span><span class="kw">pub const </span>EM_PDP10: u16 = <span class="number">64</span>;
<span class="doccomment">/// Digital PDP-11
</span><span class="kw">pub const </span>EM_PDP11: u16 = <span class="number">65</span>;
<span class="doccomment">/// Siemens FX66 microcontroller
</span><span class="kw">pub const </span>EM_FX66: u16 = <span class="number">66</span>;
<span class="doccomment">/// STMicroelectronics ST9+ 8/16 mc
</span><span class="kw">pub const </span>EM_ST9PLUS: u16 = <span class="number">67</span>;
<span class="doccomment">/// STmicroelectronics ST7 8 bit mc
</span><span class="kw">pub const </span>EM_ST7: u16 = <span class="number">68</span>;
<span class="doccomment">/// Motorola MC68HC16 microcontroller
</span><span class="kw">pub const </span>EM_68HC16: u16 = <span class="number">69</span>;
<span class="doccomment">/// Motorola MC68HC11 microcontroller
</span><span class="kw">pub const </span>EM_68HC11: u16 = <span class="number">70</span>;
<span class="doccomment">/// Motorola MC68HC08 microcontroller
</span><span class="kw">pub const </span>EM_68HC08: u16 = <span class="number">71</span>;
<span class="doccomment">/// Motorola MC68HC05 microcontroller
</span><span class="kw">pub const </span>EM_68HC05: u16 = <span class="number">72</span>;
<span class="doccomment">/// Silicon Graphics SVx
</span><span class="kw">pub const </span>EM_SVX: u16 = <span class="number">73</span>;
<span class="doccomment">/// STMicroelectronics ST19 8 bit mc
</span><span class="kw">pub const </span>EM_ST19: u16 = <span class="number">74</span>;
<span class="doccomment">/// Digital VAX
</span><span class="kw">pub const </span>EM_VAX: u16 = <span class="number">75</span>;
<span class="doccomment">/// Axis Communications 32-bit emb.proc
</span><span class="kw">pub const </span>EM_CRIS: u16 = <span class="number">76</span>;
<span class="doccomment">/// Infineon Technologies 32-bit emb.proc
</span><span class="kw">pub const </span>EM_JAVELIN: u16 = <span class="number">77</span>;
<span class="doccomment">/// Element 14 64-bit DSP Processor
</span><span class="kw">pub const </span>EM_FIREPATH: u16 = <span class="number">78</span>;
<span class="doccomment">/// LSI Logic 16-bit DSP Processor
</span><span class="kw">pub const </span>EM_ZSP: u16 = <span class="number">79</span>;
<span class="doccomment">/// Donald Knuth's educational 64-bit proc
</span><span class="kw">pub const </span>EM_MMIX: u16 = <span class="number">80</span>;
<span class="doccomment">/// Harvard University machine-independent object files
</span><span class="kw">pub const </span>EM_HUANY: u16 = <span class="number">81</span>;
<span class="doccomment">/// SiTera Prism
</span><span class="kw">pub const </span>EM_PRISM: u16 = <span class="number">82</span>;
<span class="doccomment">/// Atmel AVR 8-bit microcontroller
</span><span class="kw">pub const </span>EM_AVR: u16 = <span class="number">83</span>;
<span class="doccomment">/// Fujitsu FR30
</span><span class="kw">pub const </span>EM_FR30: u16 = <span class="number">84</span>;
<span class="doccomment">/// Mitsubishi D10V
</span><span class="kw">pub const </span>EM_D10V: u16 = <span class="number">85</span>;
<span class="doccomment">/// Mitsubishi D30V
</span><span class="kw">pub const </span>EM_D30V: u16 = <span class="number">86</span>;
<span class="doccomment">/// NEC v850
</span><span class="kw">pub const </span>EM_V850: u16 = <span class="number">87</span>;
<span class="doccomment">/// Mitsubishi M32R
</span><span class="kw">pub const </span>EM_M32R: u16 = <span class="number">88</span>;
<span class="doccomment">/// Matsushita MN10300
</span><span class="kw">pub const </span>EM_MN10300: u16 = <span class="number">89</span>;
<span class="doccomment">/// Matsushita MN10200
</span><span class="kw">pub const </span>EM_MN10200: u16 = <span class="number">90</span>;
<span class="doccomment">/// picoJava
</span><span class="kw">pub const </span>EM_PJ: u16 = <span class="number">91</span>;
<span class="doccomment">/// OpenRISC 32-bit embedded processor
</span><span class="kw">pub const </span>EM_OPENRISC: u16 = <span class="number">92</span>;
<span class="doccomment">/// ARC International ARCompact
</span><span class="kw">pub const </span>EM_ARC_COMPACT: u16 = <span class="number">93</span>;
<span class="doccomment">/// Tensilica Xtensa Architecture
</span><span class="kw">pub const </span>EM_XTENSA: u16 = <span class="number">94</span>;
<span class="doccomment">/// Alphamosaic VideoCore
</span><span class="kw">pub const </span>EM_VIDEOCORE: u16 = <span class="number">95</span>;
<span class="doccomment">/// Thompson Multimedia General Purpose Proc
</span><span class="kw">pub const </span>EM_TMM_GPP: u16 = <span class="number">96</span>;
<span class="doccomment">/// National Semi. 32000
</span><span class="kw">pub const </span>EM_NS32K: u16 = <span class="number">97</span>;
<span class="doccomment">/// Tenor Network TPC
</span><span class="kw">pub const </span>EM_TPC: u16 = <span class="number">98</span>;
<span class="doccomment">/// Trebia SNP 1000
</span><span class="kw">pub const </span>EM_SNP1K: u16 = <span class="number">99</span>;
<span class="doccomment">/// STMicroelectronics ST200
</span><span class="kw">pub const </span>EM_ST200: u16 = <span class="number">100</span>;
<span class="doccomment">/// Ubicom IP2xxx
</span><span class="kw">pub const </span>EM_IP2K: u16 = <span class="number">101</span>;
<span class="doccomment">/// MAX processor
</span><span class="kw">pub const </span>EM_MAX: u16 = <span class="number">102</span>;
<span class="doccomment">/// National Semi. CompactRISC
</span><span class="kw">pub const </span>EM_CR: u16 = <span class="number">103</span>;
<span class="doccomment">/// Fujitsu F2MC16
</span><span class="kw">pub const </span>EM_F2MC16: u16 = <span class="number">104</span>;
<span class="doccomment">/// Texas Instruments msp430
</span><span class="kw">pub const </span>EM_MSP430: u16 = <span class="number">105</span>;
<span class="doccomment">/// Analog Devices Blackfin DSP
</span><span class="kw">pub const </span>EM_BLACKFIN: u16 = <span class="number">106</span>;
<span class="doccomment">/// Seiko Epson S1C33 family
</span><span class="kw">pub const </span>EM_SE_C33: u16 = <span class="number">107</span>;
<span class="doccomment">/// Sharp embedded microprocessor
</span><span class="kw">pub const </span>EM_SEP: u16 = <span class="number">108</span>;
<span class="doccomment">/// Arca RISC
</span><span class="kw">pub const </span>EM_ARCA: u16 = <span class="number">109</span>;
<span class="doccomment">/// PKU-Unity &amp; MPRC Peking Uni. mc series
</span><span class="kw">pub const </span>EM_UNICORE: u16 = <span class="number">110</span>;
<span class="doccomment">/// eXcess configurable cpu
</span><span class="kw">pub const </span>EM_EXCESS: u16 = <span class="number">111</span>;
<span class="doccomment">/// Icera Semi. Deep Execution Processor
</span><span class="kw">pub const </span>EM_DXP: u16 = <span class="number">112</span>;
<span class="doccomment">/// Altera Nios II
</span><span class="kw">pub const </span>EM_ALTERA_NIOS2: u16 = <span class="number">113</span>;
<span class="doccomment">/// National Semi. CompactRISC CRX
</span><span class="kw">pub const </span>EM_CRX: u16 = <span class="number">114</span>;
<span class="doccomment">/// Motorola XGATE
</span><span class="kw">pub const </span>EM_XGATE: u16 = <span class="number">115</span>;
<span class="doccomment">/// Infineon C16x/XC16x
</span><span class="kw">pub const </span>EM_C166: u16 = <span class="number">116</span>;
<span class="doccomment">/// Renesas M16C
</span><span class="kw">pub const </span>EM_M16C: u16 = <span class="number">117</span>;
<span class="doccomment">/// Microchip Technology dsPIC30F
</span><span class="kw">pub const </span>EM_DSPIC30F: u16 = <span class="number">118</span>;
<span class="doccomment">/// Freescale Communication Engine RISC
</span><span class="kw">pub const </span>EM_CE: u16 = <span class="number">119</span>;
<span class="doccomment">/// Renesas M32C
</span><span class="kw">pub const </span>EM_M32C: u16 = <span class="number">120</span>;
<span class="comment">// reserved 121-130
</span><span class="doccomment">/// Altium TSK3000
</span><span class="kw">pub const </span>EM_TSK3000: u16 = <span class="number">131</span>;
<span class="doccomment">/// Freescale RS08
</span><span class="kw">pub const </span>EM_RS08: u16 = <span class="number">132</span>;
<span class="doccomment">/// Analog Devices SHARC family
</span><span class="kw">pub const </span>EM_SHARC: u16 = <span class="number">133</span>;
<span class="doccomment">/// Cyan Technology eCOG2
</span><span class="kw">pub const </span>EM_ECOG2: u16 = <span class="number">134</span>;
<span class="doccomment">/// Sunplus S+core7 RISC
</span><span class="kw">pub const </span>EM_SCORE7: u16 = <span class="number">135</span>;
<span class="doccomment">/// New Japan Radio (NJR) 24-bit DSP
</span><span class="kw">pub const </span>EM_DSP24: u16 = <span class="number">136</span>;
<span class="doccomment">/// Broadcom VideoCore III
</span><span class="kw">pub const </span>EM_VIDEOCORE3: u16 = <span class="number">137</span>;
<span class="doccomment">/// RISC for Lattice FPGA
</span><span class="kw">pub const </span>EM_LATTICEMICO32: u16 = <span class="number">138</span>;
<span class="doccomment">/// Seiko Epson C17
</span><span class="kw">pub const </span>EM_SE_C17: u16 = <span class="number">139</span>;
<span class="doccomment">/// Texas Instruments TMS320C6000 DSP
</span><span class="kw">pub const </span>EM_TI_C6000: u16 = <span class="number">140</span>;
<span class="doccomment">/// Texas Instruments TMS320C2000 DSP
</span><span class="kw">pub const </span>EM_TI_C2000: u16 = <span class="number">141</span>;
<span class="doccomment">/// Texas Instruments TMS320C55x DSP
</span><span class="kw">pub const </span>EM_TI_C5500: u16 = <span class="number">142</span>;
<span class="doccomment">/// Texas Instruments App. Specific RISC
</span><span class="kw">pub const </span>EM_TI_ARP32: u16 = <span class="number">143</span>;
<span class="doccomment">/// Texas Instruments Prog. Realtime Unit
</span><span class="kw">pub const </span>EM_TI_PRU: u16 = <span class="number">144</span>;
<span class="comment">// reserved 145-159
</span><span class="doccomment">/// STMicroelectronics 64bit VLIW DSP
</span><span class="kw">pub const </span>EM_MMDSP_PLUS: u16 = <span class="number">160</span>;
<span class="doccomment">/// Cypress M8C
</span><span class="kw">pub const </span>EM_CYPRESS_M8C: u16 = <span class="number">161</span>;
<span class="doccomment">/// Renesas R32C
</span><span class="kw">pub const </span>EM_R32C: u16 = <span class="number">162</span>;
<span class="doccomment">/// NXP Semi. TriMedia
</span><span class="kw">pub const </span>EM_TRIMEDIA: u16 = <span class="number">163</span>;
<span class="doccomment">/// QUALCOMM DSP6
</span><span class="kw">pub const </span>EM_QDSP6: u16 = <span class="number">164</span>;
<span class="doccomment">/// Intel 8051 and variants
</span><span class="kw">pub const </span>EM_8051: u16 = <span class="number">165</span>;
<span class="doccomment">/// STMicroelectronics STxP7x
</span><span class="kw">pub const </span>EM_STXP7X: u16 = <span class="number">166</span>;
<span class="doccomment">/// Andes Tech. compact code emb. RISC
</span><span class="kw">pub const </span>EM_NDS32: u16 = <span class="number">167</span>;
<span class="doccomment">/// Cyan Technology eCOG1X
</span><span class="kw">pub const </span>EM_ECOG1X: u16 = <span class="number">168</span>;
<span class="doccomment">/// Dallas Semi. MAXQ30 mc
</span><span class="kw">pub const </span>EM_MAXQ30: u16 = <span class="number">169</span>;
<span class="doccomment">/// New Japan Radio (NJR) 16-bit DSP
</span><span class="kw">pub const </span>EM_XIMO16: u16 = <span class="number">170</span>;
<span class="doccomment">/// M2000 Reconfigurable RISC
</span><span class="kw">pub const </span>EM_MANIK: u16 = <span class="number">171</span>;
<span class="doccomment">/// Cray NV2 vector architecture
</span><span class="kw">pub const </span>EM_CRAYNV2: u16 = <span class="number">172</span>;
<span class="doccomment">/// Renesas RX
</span><span class="kw">pub const </span>EM_RX: u16 = <span class="number">173</span>;
<span class="doccomment">/// Imagination Tech. META
</span><span class="kw">pub const </span>EM_METAG: u16 = <span class="number">174</span>;
<span class="doccomment">/// MCST Elbrus
</span><span class="kw">pub const </span>EM_MCST_ELBRUS: u16 = <span class="number">175</span>;
<span class="doccomment">/// Cyan Technology eCOG16
</span><span class="kw">pub const </span>EM_ECOG16: u16 = <span class="number">176</span>;
<span class="doccomment">/// National Semi. CompactRISC CR16
</span><span class="kw">pub const </span>EM_CR16: u16 = <span class="number">177</span>;
<span class="doccomment">/// Freescale Extended Time Processing Unit
</span><span class="kw">pub const </span>EM_ETPU: u16 = <span class="number">178</span>;
<span class="doccomment">/// Infineon Tech. SLE9X
</span><span class="kw">pub const </span>EM_SLE9X: u16 = <span class="number">179</span>;
<span class="doccomment">/// Intel L10M
</span><span class="kw">pub const </span>EM_L10M: u16 = <span class="number">180</span>;
<span class="doccomment">/// Intel K10M
</span><span class="kw">pub const </span>EM_K10M: u16 = <span class="number">181</span>;
<span class="comment">// reserved 182
</span><span class="doccomment">/// ARM AARCH64
</span><span class="kw">pub const </span>EM_AARCH64: u16 = <span class="number">183</span>;
<span class="comment">// reserved 184
</span><span class="doccomment">/// Amtel 32-bit microprocessor
</span><span class="kw">pub const </span>EM_AVR32: u16 = <span class="number">185</span>;
<span class="doccomment">/// STMicroelectronics STM8
</span><span class="kw">pub const </span>EM_STM8: u16 = <span class="number">186</span>;
<span class="doccomment">/// Tileta TILE64
</span><span class="kw">pub const </span>EM_TILE64: u16 = <span class="number">187</span>;
<span class="doccomment">/// Tilera TILEPro
</span><span class="kw">pub const </span>EM_TILEPRO: u16 = <span class="number">188</span>;
<span class="doccomment">/// Xilinx MicroBlaze
</span><span class="kw">pub const </span>EM_MICROBLAZE: u16 = <span class="number">189</span>;
<span class="doccomment">/// NVIDIA CUDA
</span><span class="kw">pub const </span>EM_CUDA: u16 = <span class="number">190</span>;
<span class="doccomment">/// Tilera TILE-Gx
</span><span class="kw">pub const </span>EM_TILEGX: u16 = <span class="number">191</span>;
<span class="doccomment">/// CloudShield
</span><span class="kw">pub const </span>EM_CLOUDSHIELD: u16 = <span class="number">192</span>;
<span class="doccomment">/// KIPO-KAIST Core-A 1st gen.
</span><span class="kw">pub const </span>EM_COREA_1ST: u16 = <span class="number">193</span>;
<span class="doccomment">/// KIPO-KAIST Core-A 2nd gen.
</span><span class="kw">pub const </span>EM_COREA_2ND: u16 = <span class="number">194</span>;
<span class="doccomment">/// Synopsys ARCompact V2
</span><span class="kw">pub const </span>EM_ARC_COMPACT2: u16 = <span class="number">195</span>;
<span class="doccomment">/// Open8 RISC
</span><span class="kw">pub const </span>EM_OPEN8: u16 = <span class="number">196</span>;
<span class="doccomment">/// Renesas RL78
</span><span class="kw">pub const </span>EM_RL78: u16 = <span class="number">197</span>;
<span class="doccomment">/// Broadcom VideoCore V
</span><span class="kw">pub const </span>EM_VIDEOCORE5: u16 = <span class="number">198</span>;
<span class="doccomment">/// Renesas 78KOR
</span><span class="kw">pub const </span>EM_78KOR: u16 = <span class="number">199</span>;
<span class="doccomment">/// Freescale 56800EX DSC
</span><span class="kw">pub const </span>EM_56800EX: u16 = <span class="number">200</span>;
<span class="doccomment">/// Beyond BA1
</span><span class="kw">pub const </span>EM_BA1: u16 = <span class="number">201</span>;
<span class="doccomment">/// Beyond BA2
</span><span class="kw">pub const </span>EM_BA2: u16 = <span class="number">202</span>;
<span class="doccomment">/// XMOS xCORE
</span><span class="kw">pub const </span>EM_XCORE: u16 = <span class="number">203</span>;
<span class="doccomment">/// Microchip 8-bit PIC(r)
</span><span class="kw">pub const </span>EM_MCHP_PIC: u16 = <span class="number">204</span>;
<span class="doccomment">/// Intel Graphics Technology
</span><span class="kw">pub const </span>EM_INTELGT: u16 = <span class="number">205</span>;
<span class="comment">// reserved 206-209
</span><span class="doccomment">/// KM211 KM32
</span><span class="kw">pub const </span>EM_KM32: u16 = <span class="number">210</span>;
<span class="doccomment">/// KM211 KMX32
</span><span class="kw">pub const </span>EM_KMX32: u16 = <span class="number">211</span>;
<span class="doccomment">/// KM211 KMX16
</span><span class="kw">pub const </span>EM_EMX16: u16 = <span class="number">212</span>;
<span class="doccomment">/// KM211 KMX8
</span><span class="kw">pub const </span>EM_EMX8: u16 = <span class="number">213</span>;
<span class="doccomment">/// KM211 KVARC
</span><span class="kw">pub const </span>EM_KVARC: u16 = <span class="number">214</span>;
<span class="doccomment">/// Paneve CDP
</span><span class="kw">pub const </span>EM_CDP: u16 = <span class="number">215</span>;
<span class="doccomment">/// Cognitive Smart Memory Processor
</span><span class="kw">pub const </span>EM_COGE: u16 = <span class="number">216</span>;
<span class="doccomment">/// Bluechip CoolEngine
</span><span class="kw">pub const </span>EM_COOL: u16 = <span class="number">217</span>;
<span class="doccomment">/// Nanoradio Optimized RISC
</span><span class="kw">pub const </span>EM_NORC: u16 = <span class="number">218</span>;
<span class="doccomment">/// CSR Kalimba
</span><span class="kw">pub const </span>EM_CSR_KALIMBA: u16 = <span class="number">219</span>;
<span class="doccomment">/// Zilog Z80
</span><span class="kw">pub const </span>EM_Z80: u16 = <span class="number">220</span>;
<span class="doccomment">/// Controls and Data Services VISIUMcore
</span><span class="kw">pub const </span>EM_VISIUM: u16 = <span class="number">221</span>;
<span class="doccomment">/// FTDI Chip FT32
</span><span class="kw">pub const </span>EM_FT32: u16 = <span class="number">222</span>;
<span class="doccomment">/// Moxie processor
</span><span class="kw">pub const </span>EM_MOXIE: u16 = <span class="number">223</span>;
<span class="doccomment">/// AMD GPU
</span><span class="kw">pub const </span>EM_AMDGPU: u16 = <span class="number">224</span>;
<span class="comment">// reserved 225-242
</span><span class="doccomment">/// RISC-V
</span><span class="kw">pub const </span>EM_RISCV: u16 = <span class="number">243</span>;

<span class="doccomment">/// Linux BPF -- in-kernel virtual machine
</span><span class="kw">pub const </span>EM_BPF: u16 = <span class="number">247</span>;

<span class="doccomment">/// C-SKY
</span><span class="kw">pub const </span>EM_CSKY: u16 = <span class="number">252</span>;

<span class="kw">pub const </span>EM_NUM: u16 = <span class="number">248</span>;

<span class="doccomment">/// Convert machine to str representation
</span><span class="kw">pub fn </span>machine_to_str (machine: u16) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str {
    <span class="kw">match </span>machine {
        EM_M32 =&gt; <span class="string">"M32"</span>,
        EM_SPARC =&gt; <span class="string">"SPARC"</span>,
        EM_386 =&gt; <span class="string">"386"</span>,
        EM_68K =&gt; <span class="string">"68K"</span>,
        EM_88K =&gt; <span class="string">"88K"</span>,
        EM_IAMCU =&gt; <span class="string">"IAMCU"</span>,
        EM_860 =&gt; <span class="string">"860"</span>,
        EM_MIPS =&gt; <span class="string">"MIPS"</span>,
        EM_S370 =&gt; <span class="string">"S370"</span>,
        EM_MIPS_RS3_LE =&gt; <span class="string">"MIPS_RS3_LE"</span>,
        EM_PARISC =&gt; <span class="string">"PARISC"</span>,
        EM_VPP500 =&gt; <span class="string">"VPP500"</span>,
        EM_SPARC32PLUS =&gt; <span class="string">"SPARC32PLUS"</span>,
        EM_960 =&gt; <span class="string">"960"</span>,
        EM_PPC =&gt; <span class="string">"PPC"</span>,
        EM_PPC64 =&gt; <span class="string">"PPC64"</span>,
        EM_S390 =&gt; <span class="string">"S390"</span>,
        EM_SPU =&gt; <span class="string">"SPU"</span>,
        EM_V800 =&gt; <span class="string">"V800"</span>,
        EM_FR20 =&gt; <span class="string">"FR20"</span>,
        EM_RH32 =&gt; <span class="string">"RH32"</span>,
        EM_RCE =&gt; <span class="string">"RCE"</span>,
        EM_ARM =&gt; <span class="string">"ARM"</span>,
        EM_FAKE_ALPHA =&gt; <span class="string">"FAKE_ALPHA"</span>,
        EM_SH =&gt; <span class="string">"SH"</span>,
        EM_SPARCV9 =&gt; <span class="string">"SPARCV9"</span>,
        EM_TRICORE =&gt; <span class="string">"TRICORE"</span>,
        EM_ARC =&gt; <span class="string">"ARC"</span>,
        EM_H8_300 =&gt; <span class="string">"H8_300"</span>,
        EM_H8_300H =&gt; <span class="string">"H8_300H"</span>,
        EM_H8S =&gt; <span class="string">"H8S"</span>,
        EM_H8_500 =&gt; <span class="string">"H8_500"</span>,
        EM_IA_64 =&gt; <span class="string">"IA_64"</span>,
        EM_MIPS_X =&gt; <span class="string">"MIPS_X"</span>,
        EM_COLDFIRE =&gt; <span class="string">"COLDFIRE"</span>,
        EM_68HC12 =&gt; <span class="string">"68HC12"</span>,
        EM_MMA =&gt; <span class="string">"MMA"</span>,
        EM_PCP =&gt; <span class="string">"PCP"</span>,
        EM_NCPU =&gt; <span class="string">"NCPU"</span>,
        EM_NDR1 =&gt; <span class="string">"NDR1"</span>,
        EM_STARCORE =&gt; <span class="string">"STARCORE"</span>,
        EM_ME16 =&gt; <span class="string">"ME16"</span>,
        EM_ST100 =&gt; <span class="string">"ST100"</span>,
        EM_TINYJ =&gt; <span class="string">"TINYJ"</span>,
        EM_X86_64 =&gt; <span class="string">"X86_64"</span>,
        EM_PDSP =&gt; <span class="string">"PDSP"</span>,
        EM_PDP10 =&gt; <span class="string">"PDP10"</span>,
        EM_PDP11 =&gt; <span class="string">"PDP11"</span>,
        EM_FX66 =&gt; <span class="string">"FX66"</span>,
        EM_ST9PLUS =&gt; <span class="string">"ST9PLUS"</span>,
        EM_ST7 =&gt; <span class="string">"ST7"</span>,
        EM_68HC16 =&gt; <span class="string">"68HC16"</span>,
        EM_68HC11 =&gt; <span class="string">"68HC11"</span>,
        EM_68HC08 =&gt; <span class="string">"68HC08"</span>,
        EM_68HC05 =&gt; <span class="string">"68HC05"</span>,
        EM_SVX =&gt; <span class="string">"SVX"</span>,
        EM_ST19 =&gt; <span class="string">"ST19"</span>,
        EM_VAX =&gt; <span class="string">"VAX"</span>,
        EM_CRIS =&gt; <span class="string">"CRIS"</span>,
        EM_JAVELIN =&gt; <span class="string">"JAVELIN"</span>,
        EM_FIREPATH =&gt; <span class="string">"FIREPATH"</span>,
        EM_ZSP =&gt; <span class="string">"ZSP"</span>,
        EM_MMIX =&gt; <span class="string">"MMIX"</span>,
        EM_HUANY =&gt; <span class="string">"HUANY"</span>,
        EM_PRISM =&gt; <span class="string">"PRISM"</span>,
        EM_AVR =&gt; <span class="string">"AVR"</span>,
        EM_FR30 =&gt; <span class="string">"FR30"</span>,
        EM_D10V =&gt; <span class="string">"D10V"</span>,
        EM_D30V =&gt; <span class="string">"D30V"</span>,
        EM_V850 =&gt; <span class="string">"V850"</span>,
        EM_M32R =&gt; <span class="string">"M32R"</span>,
        EM_MN10300 =&gt; <span class="string">"MN10300"</span>,
        EM_MN10200 =&gt; <span class="string">"MN10200"</span>,
        EM_PJ =&gt; <span class="string">"PJ"</span>,
        EM_OPENRISC =&gt; <span class="string">"OPENRISC"</span>,
        EM_ARC_COMPACT =&gt; <span class="string">"ARC_COMPACT"</span>,
        EM_XTENSA =&gt; <span class="string">"XTENSA"</span>,
        EM_VIDEOCORE =&gt; <span class="string">"VIDEOCORE"</span>,
        EM_TMM_GPP =&gt; <span class="string">"TMM_GPP"</span>,
        EM_NS32K =&gt; <span class="string">"NS32K"</span>,
        EM_TPC =&gt; <span class="string">"TPC"</span>,
        EM_SNP1K =&gt; <span class="string">"SNP1K"</span>,
        EM_ST200 =&gt; <span class="string">"ST200"</span>,
        EM_IP2K =&gt; <span class="string">"IP2K"</span>,
        EM_MAX =&gt; <span class="string">"MAX"</span>,
        EM_CR =&gt; <span class="string">"CR"</span>,
        EM_F2MC16 =&gt; <span class="string">"F2MC16"</span>,
        EM_MSP430 =&gt; <span class="string">"MSP430"</span>,
        EM_BLACKFIN =&gt; <span class="string">"BLACKFIN"</span>,
        EM_SE_C33 =&gt; <span class="string">"SE_C33"</span>,
        EM_SEP =&gt; <span class="string">"SEP"</span>,
        EM_ARCA =&gt; <span class="string">"ARCA"</span>,
        EM_UNICORE =&gt; <span class="string">"UNICORE"</span>,
        EM_EXCESS =&gt; <span class="string">"EXCESS"</span>,
        EM_DXP =&gt; <span class="string">"DXP"</span>,
        EM_ALTERA_NIOS2 =&gt; <span class="string">"ALTERA_NIOS2"</span>,
        EM_CRX =&gt; <span class="string">"CRX"</span>,
        EM_XGATE =&gt; <span class="string">"XGATE"</span>,
        EM_C166 =&gt; <span class="string">"C166"</span>,
        EM_M16C =&gt; <span class="string">"M16C"</span>,
        EM_DSPIC30F =&gt; <span class="string">"DSPIC30F"</span>,
        EM_CE =&gt; <span class="string">"CE"</span>,
        EM_M32C =&gt; <span class="string">"M32C"</span>,
        EM_TSK3000 =&gt; <span class="string">"TSK3000"</span>,
        EM_RS08 =&gt; <span class="string">"RS08"</span>,
        EM_SHARC =&gt; <span class="string">"SHARC"</span>,
        EM_ECOG2 =&gt; <span class="string">"ECOG2"</span>,
        EM_SCORE7 =&gt; <span class="string">"SCORE7"</span>,
        EM_DSP24 =&gt; <span class="string">"DSP24"</span>,
        EM_VIDEOCORE3 =&gt; <span class="string">"VIDEOCORE3"</span>,
        EM_LATTICEMICO32 =&gt; <span class="string">"LATTICEMICO32"</span>,
        EM_SE_C17 =&gt; <span class="string">"SE_C17"</span>,
        EM_TI_C6000 =&gt; <span class="string">"TI_C6000"</span>,
        EM_TI_C2000 =&gt; <span class="string">"TI_C2000"</span>,
        EM_TI_C5500 =&gt; <span class="string">"TI_C5500"</span>,
        EM_TI_ARP32 =&gt; <span class="string">"TI_ARP32"</span>,
        EM_TI_PRU =&gt; <span class="string">"TI_PRU"</span>,
        EM_MMDSP_PLUS =&gt; <span class="string">"MMDSP_PLUS"</span>,
        EM_CYPRESS_M8C =&gt; <span class="string">"CYPRESS_M8C"</span>,
        EM_R32C =&gt; <span class="string">"R32C"</span>,
        EM_TRIMEDIA =&gt; <span class="string">"TRIMEDIA"</span>,
        EM_QDSP6 =&gt; <span class="string">"QDSP6"</span>,
        EM_8051 =&gt; <span class="string">"8051"</span>,
        EM_STXP7X =&gt; <span class="string">"STXP7X"</span>,
        EM_NDS32 =&gt; <span class="string">"NDS32"</span>,
        EM_ECOG1X =&gt; <span class="string">"ECOG1X"</span>,
        EM_MAXQ30 =&gt; <span class="string">"MAXQ30"</span>,
        EM_XIMO16 =&gt; <span class="string">"XIMO16"</span>,
        EM_MANIK =&gt; <span class="string">"MANIK"</span>,
        EM_CRAYNV2 =&gt; <span class="string">"CRAYNV2"</span>,
        EM_RX =&gt; <span class="string">"RX"</span>,
        EM_METAG =&gt; <span class="string">"METAG"</span>,
        EM_MCST_ELBRUS =&gt; <span class="string">"MCST_ELBRUS"</span>,
        EM_ECOG16 =&gt; <span class="string">"ECOG16"</span>,
        EM_CR16 =&gt; <span class="string">"CR16"</span>,
        EM_ETPU =&gt; <span class="string">"ETPU"</span>,
        EM_SLE9X =&gt; <span class="string">"SLE9X"</span>,
        EM_L10M =&gt; <span class="string">"L10M"</span>,
        EM_K10M =&gt; <span class="string">"K10M"</span>,
        EM_AARCH64 =&gt; <span class="string">"AARCH64"</span>,
        EM_AVR32 =&gt; <span class="string">"AVR32"</span>,
        EM_STM8 =&gt; <span class="string">"STM8"</span>,
        EM_TILE64 =&gt; <span class="string">"TILE64"</span>,
        EM_TILEPRO =&gt; <span class="string">"TILEPRO"</span>,
        EM_MICROBLAZE =&gt; <span class="string">"MICROBLAZE"</span>,
        EM_CUDA =&gt; <span class="string">"CUDA"</span>,
        EM_TILEGX =&gt; <span class="string">"TILEGX"</span>,
        EM_CLOUDSHIELD =&gt; <span class="string">"CLOUDSHIELD"</span>,
        EM_COREA_1ST =&gt; <span class="string">"COREA_1ST"</span>,
        EM_COREA_2ND =&gt; <span class="string">"COREA_2ND"</span>,
        EM_ARC_COMPACT2 =&gt; <span class="string">"ARC_COMPACT2"</span>,
        EM_OPEN8 =&gt; <span class="string">"OPEN8"</span>,
        EM_RL78 =&gt; <span class="string">"RL78"</span>,
        EM_VIDEOCORE5 =&gt; <span class="string">"VIDEOCORE5"</span>,
        EM_78KOR =&gt; <span class="string">"78KOR"</span>,
        EM_56800EX =&gt; <span class="string">"56800EX"</span>,
        EM_BA1 =&gt; <span class="string">"BA1"</span>,
        EM_BA2 =&gt; <span class="string">"BA2"</span>,
        EM_XCORE =&gt; <span class="string">"XCORE"</span>,
        EM_MCHP_PIC =&gt; <span class="string">"MCHP_PIC"</span>,
        EM_KM32 =&gt; <span class="string">"KM32"</span>,
        EM_KMX32 =&gt; <span class="string">"KMX32"</span>,
        EM_EMX16 =&gt; <span class="string">"EMX16"</span>,
        EM_EMX8 =&gt; <span class="string">"EMX8"</span>,
        EM_KVARC =&gt; <span class="string">"KVARC"</span>,
        EM_CDP =&gt; <span class="string">"CDP"</span>,
        EM_COGE =&gt; <span class="string">"COGE"</span>,
        EM_COOL =&gt; <span class="string">"COOL"</span>,
        EM_NORC =&gt; <span class="string">"NORC"</span>,
        EM_CSR_KALIMBA =&gt; <span class="string">"CSR_KALIMBA"</span>,
        EM_Z80 =&gt; <span class="string">"Z80"</span>,
        EM_VISIUM =&gt; <span class="string">"VISIUM"</span>,
        EM_FT32 =&gt; <span class="string">"FT32"</span>,
        EM_MOXIE =&gt; <span class="string">"MOXIE"</span>,
        EM_AMDGPU =&gt; <span class="string">"AMDGPU"</span>,
        EM_RISCV =&gt; <span class="string">"RISCV"</span>,
        EM_BPF =&gt; <span class="string">"BPF"</span>,
        _val =&gt; <span class="string">"EM_UNKNOWN"</span>,
    }
}
</code></pre></div></section></main></body></html>