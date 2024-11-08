<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/chrono-0.4.38/src/format/mod.rs`."><title>mod.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="chrono" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../chrono/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

</span><span class="doccomment">//! Formatting (and parsing) utilities for date and time.
//!
//! This module provides the common types and routines to implement,
//! for example, [`DateTime::format`](../struct.DateTime.html#method.format) or
//! [`DateTime::parse_from_str`](../struct.DateTime.html#method.parse_from_str) methods.
//! For most cases you should use these high-level interfaces.
//!
//! Internally the formatting and parsing shares the same abstract **formatting items**,
//! which are just an [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) of
//! the [`Item`](./enum.Item.html) type.
//! They are generated from more readable **format strings**;
//! currently Chrono supports a built-in syntax closely resembling
//! C's `strftime` format. The available options can be found [here](./strftime/index.html).
//!
//! # Example
//! ```
//! # #[cfg(feature = "alloc")] {
//! use chrono::{NaiveDateTime, TimeZone, Utc};
//!
//! let date_time = Utc.with_ymd_and_hms(2020, 11, 10, 0, 1, 32).unwrap();
//!
//! let formatted = format!("{}", date_time.format("%Y-%m-%d %H:%M:%S"));
//! assert_eq!(formatted, "2020-11-10 00:01:32");
//!
//! let parsed = NaiveDateTime::parse_from_str(&amp;formatted, "%Y-%m-%d %H:%M:%S")?.and_utc();
//! assert_eq!(parsed, date_time);
//! # }
//! # Ok::&lt;(), chrono::ParseError&gt;(())
//! ```

</span><span class="attr">#[cfg(all(feature = <span class="string">"alloc"</span>, not(feature = <span class="string">"std"</span>), not(test)))]
</span><span class="kw">use </span>alloc::boxed::Box;
<span class="kw">use </span>core::fmt;
<span class="kw">use </span>core::str::FromStr;
<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">use </span>std::error::Error;

<span class="kw">use crate</span>::{Month, ParseMonthError, ParseWeekdayError, Weekday};

<span class="kw">mod </span>formatting;
<span class="kw">mod </span>parsed;

<span class="comment">// due to the size of parsing routines, they are in separate modules.
</span><span class="kw">mod </span>parse;
<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">mod </span>scan;

<span class="kw">pub mod </span>strftime;

<span class="attr">#[allow(unused)]
</span><span class="comment">// TODO: remove '#[allow(unused)]' once we use this module for parsing or something else that does
// not require `alloc`.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">mod </span>locales;

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span>formatting::write_hundreds;
<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span>formatting::write_rfc2822;
<span class="attr">#[cfg(any(feature = <span class="string">"alloc"</span>, feature = <span class="string">"serde"</span>))]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span>formatting::write_rfc3339;
<span class="kw">pub use </span>formatting::SecondsFormat;
<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
#[allow(deprecated)]
</span><span class="kw">pub use </span>formatting::{format, format_item, DelayedFormat};
<span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
</span><span class="kw">pub use </span>locales::Locale;
<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span>parse::parse_rfc3339;
<span class="kw">pub use </span>parse::{parse, parse_and_remainder};
<span class="kw">pub use </span>parsed::Parsed;
<span class="kw">pub use </span>strftime::StrftimeItems;

<span class="doccomment">/// An uninhabited type used for `InternalNumeric` and `InternalFixed` below.
</span><span class="attr">#[derive(Clone, PartialEq, Eq, Hash)]
</span><span class="kw">enum </span>Void {}

<span class="doccomment">/// Padding characters for numeric items.
</span><span class="attr">#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
</span><span class="kw">pub enum </span>Pad {
    <span class="doccomment">/// No padding.
    </span><span class="prelude-val">None</span>,
    <span class="doccomment">/// Zero (`0`) padding.
    </span>Zero,
    <span class="doccomment">/// Space padding.
    </span>Space,
}

<span class="doccomment">/// Numeric item types.
/// They have associated formatting width (FW) and parsing width (PW).
///
/// The **formatting width** is the minimal width to be formatted.
/// If the number is too short, and the padding is not [`Pad::None`](./enum.Pad.html#variant.None),
/// then it is left-padded.
/// If the number is too long or (in some cases) negative, it is printed as is.
///
/// The **parsing width** is the maximal width to be scanned.
/// The parser only tries to consume from one to given number of digits (greedily).
/// It also trims the preceding whitespace if any.
/// It cannot parse the negative number, so some date and time cannot be formatted then
/// parsed with the same formatting items.
</span><span class="attr">#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
</span><span class="kw">pub enum </span>Numeric {
    <span class="doccomment">/// Full Gregorian year (FW=4, PW=∞).
    /// May accept years before 1 BCE or after 9999 CE, given an initial sign (+/-).
    </span>Year,
    <span class="doccomment">/// Gregorian year divided by 100 (century number; FW=PW=2). Implies the non-negative year.
    </span>YearDiv100,
    <span class="doccomment">/// Gregorian year modulo 100 (FW=PW=2). Cannot be negative.
    </span>YearMod100,
    <span class="doccomment">/// Year in the ISO week date (FW=4, PW=∞).
    /// May accept years before 1 BCE or after 9999 CE, given an initial sign.
    </span>IsoYear,
    <span class="doccomment">/// Year in the ISO week date, divided by 100 (FW=PW=2). Implies the non-negative year.
    </span>IsoYearDiv100,
    <span class="doccomment">/// Year in the ISO week date, modulo 100 (FW=PW=2). Cannot be negative.
    </span>IsoYearMod100,
    <span class="doccomment">/// Month (FW=PW=2).
    </span>Month,
    <span class="doccomment">/// Day of the month (FW=PW=2).
    </span>Day,
    <span class="doccomment">/// Week number, where the week 1 starts at the first Sunday of January (FW=PW=2).
    </span>WeekFromSun,
    <span class="doccomment">/// Week number, where the week 1 starts at the first Monday of January (FW=PW=2).
    </span>WeekFromMon,
    <span class="doccomment">/// Week number in the ISO week date (FW=PW=2).
    </span>IsoWeek,
    <span class="doccomment">/// Day of the week, where Sunday = 0 and Saturday = 6 (FW=PW=1).
    </span>NumDaysFromSun,
    <span class="doccomment">/// Day of the week, where Monday = 1 and Sunday = 7 (FW=PW=1).
    </span>WeekdayFromMon,
    <span class="doccomment">/// Day of the year (FW=PW=3).
    </span>Ordinal,
    <span class="doccomment">/// Hour number in the 24-hour clocks (FW=PW=2).
    </span>Hour,
    <span class="doccomment">/// Hour number in the 12-hour clocks (FW=PW=2).
    </span>Hour12,
    <span class="doccomment">/// The number of minutes since the last whole hour (FW=PW=2).
    </span>Minute,
    <span class="doccomment">/// The number of seconds since the last whole minute (FW=PW=2).
    </span>Second,
    <span class="doccomment">/// The number of nanoseconds since the last whole second (FW=PW=9).
    /// Note that this is *not* left-aligned;
    /// see also [`Fixed::Nanosecond`](./enum.Fixed.html#variant.Nanosecond).
    </span>Nanosecond,
    <span class="doccomment">/// The number of non-leap seconds since the midnight UTC on January 1, 1970 (FW=1, PW=∞).
    /// For formatting, it assumes UTC upon the absence of time zone offset.
    </span>Timestamp,

    <span class="doccomment">/// Internal uses only.
    ///
    /// This item exists so that one can add additional internal-only formatting
    /// without breaking major compatibility (as enum variants cannot be selectively private).
    </span>Internal(InternalNumeric),
}

<span class="doccomment">/// An opaque type representing numeric item types for internal uses only.
</span><span class="attr">#[derive(Clone, Eq, Hash, PartialEq)]
</span><span class="kw">pub struct </span>InternalNumeric {
    _dummy: Void,
}

<span class="kw">impl </span>fmt::Debug <span class="kw">for </span>InternalNumeric {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="macro">write!</span>(f, <span class="string">"&lt;InternalNumeric&gt;"</span>)
    }
}

<span class="doccomment">/// Fixed-format item types.
///
/// They have their own rules of formatting and parsing.
/// Otherwise noted, they print in the specified cases but parse case-insensitively.
</span><span class="attr">#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
</span><span class="kw">pub enum </span>Fixed {
    <span class="doccomment">/// Abbreviated month names.
    ///
    /// Prints a three-letter-long name in the title case, reads the same name in any case.
    </span>ShortMonthName,
    <span class="doccomment">/// Full month names.
    ///
    /// Prints a full name in the title case, reads either a short or full name in any case.
    </span>LongMonthName,
    <span class="doccomment">/// Abbreviated day of the week names.
    ///
    /// Prints a three-letter-long name in the title case, reads the same name in any case.
    </span>ShortWeekdayName,
    <span class="doccomment">/// Full day of the week names.
    ///
    /// Prints a full name in the title case, reads either a short or full name in any case.
    </span>LongWeekdayName,
    <span class="doccomment">/// AM/PM.
    ///
    /// Prints in lower case, reads in any case.
    </span>LowerAmPm,
    <span class="doccomment">/// AM/PM.
    ///
    /// Prints in upper case, reads in any case.
    </span>UpperAmPm,
    <span class="doccomment">/// An optional dot plus one or more digits for left-aligned nanoseconds.
    /// May print nothing, 3, 6 or 9 digits according to the available accuracy.
    /// See also [`Numeric::Nanosecond`](./enum.Numeric.html#variant.Nanosecond).
    </span>Nanosecond,
    <span class="doccomment">/// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 3.
    </span>Nanosecond3,
    <span class="doccomment">/// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 6.
    </span>Nanosecond6,
    <span class="doccomment">/// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 9.
    </span>Nanosecond9,
    <span class="doccomment">/// Timezone name.
    ///
    /// It does not support parsing, its use in the parser is an immediate failure.
    </span>TimezoneName,
    <span class="doccomment">/// Offset from the local time to UTC (`+09:00` or `-04:00` or `+00:00`).
    ///
    /// In the parser, the colon can be omitted and/or surrounded with any amount of whitespace.
    /// The offset is limited from `-24:00` to `+24:00`,
    /// which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.
    </span>TimezoneOffsetColon,
    <span class="doccomment">/// Offset from the local time to UTC with seconds (`+09:00:00` or `-04:00:00` or `+00:00:00`).
    ///
    /// In the parser, the colon can be omitted and/or surrounded with any amount of whitespace.
    /// The offset is limited from `-24:00:00` to `+24:00:00`,
    /// which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.
    </span>TimezoneOffsetDoubleColon,
    <span class="doccomment">/// Offset from the local time to UTC without minutes (`+09` or `-04` or `+00`).
    ///
    /// In the parser, the colon can be omitted and/or surrounded with any amount of whitespace.
    /// The offset is limited from `-24` to `+24`,
    /// which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.
    </span>TimezoneOffsetTripleColon,
    <span class="doccomment">/// Offset from the local time to UTC (`+09:00` or `-04:00` or `Z`).
    ///
    /// In the parser, the colon can be omitted and/or surrounded with any amount of whitespace,
    /// and `Z` can be either in upper case or in lower case.
    /// The offset is limited from `-24:00` to `+24:00`,
    /// which is the same as [`FixedOffset`](../offset/struct.FixedOffset.html)'s range.
    </span>TimezoneOffsetColonZ,
    <span class="doccomment">/// Same as [`TimezoneOffsetColon`](#variant.TimezoneOffsetColon) but prints no colon.
    /// Parsing allows an optional colon.
    </span>TimezoneOffset,
    <span class="doccomment">/// Same as [`TimezoneOffsetColonZ`](#variant.TimezoneOffsetColonZ) but prints no colon.
    /// Parsing allows an optional colon.
    </span>TimezoneOffsetZ,
    <span class="doccomment">/// RFC 2822 date and time syntax. Commonly used for email and MIME date and time.
    </span>RFC2822,
    <span class="doccomment">/// RFC 3339 &amp; ISO 8601 date and time syntax.
    </span>RFC3339,

    <span class="doccomment">/// Internal uses only.
    ///
    /// This item exists so that one can add additional internal-only formatting
    /// without breaking major compatibility (as enum variants cannot be selectively private).
    </span>Internal(InternalFixed),
}

<span class="doccomment">/// An opaque type representing fixed-format item types for internal uses only.
</span><span class="attr">#[derive(Debug, Clone, PartialEq, Eq, Hash)]
</span><span class="kw">pub struct </span>InternalFixed {
    val: InternalInternal,
}

<span class="attr">#[derive(Debug, Clone, PartialEq, Eq, Hash)]
</span><span class="kw">enum </span>InternalInternal {
    <span class="doccomment">/// Same as [`TimezoneOffsetColonZ`](#variant.TimezoneOffsetColonZ), but
    /// allows missing minutes (per [ISO 8601][iso8601]).
    ///
    /// # Panics
    ///
    /// If you try to use this for printing.
    ///
    /// [iso8601]: https://en.wikipedia.org/wiki/ISO_8601#Time_offsets_from_UTC
    </span>TimezoneOffsetPermissive,
    <span class="doccomment">/// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 3 and there is no leading dot.
    </span>Nanosecond3NoDot,
    <span class="doccomment">/// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 6 and there is no leading dot.
    </span>Nanosecond6NoDot,
    <span class="doccomment">/// Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 9 and there is no leading dot.
    </span>Nanosecond9NoDot,
}

<span class="doccomment">/// Type for specifying the format of UTC offsets.
</span><span class="attr">#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
</span><span class="kw">pub struct </span>OffsetFormat {
    <span class="doccomment">/// See `OffsetPrecision`.
    </span><span class="kw">pub </span>precision: OffsetPrecision,
    <span class="doccomment">/// Separator between hours, minutes and seconds.
    </span><span class="kw">pub </span>colons: Colons,
    <span class="doccomment">/// Represent `+00:00` as `Z`.
    </span><span class="kw">pub </span>allow_zulu: bool,
    <span class="doccomment">/// Pad the hour value to two digits.
    </span><span class="kw">pub </span>padding: Pad,
}

<span class="doccomment">/// The precision of an offset from UTC formatting item.
</span><span class="attr">#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
</span><span class="kw">pub enum </span>OffsetPrecision {
    <span class="doccomment">/// Format offset from UTC as only hours. Not recommended, it is not uncommon for timezones to
    /// have an offset of 30 minutes, 15 minutes, etc.
    /// Any minutes and seconds get truncated.
    </span>Hours,
    <span class="doccomment">/// Format offset from UTC as hours and minutes.
    /// Any seconds will be rounded to the nearest minute.
    </span>Minutes,
    <span class="doccomment">/// Format offset from UTC as hours, minutes and seconds.
    </span>Seconds,
    <span class="doccomment">/// Format offset from UTC as hours, and optionally with minutes.
    /// Any seconds will be rounded to the nearest minute.
    </span>OptionalMinutes,
    <span class="doccomment">/// Format offset from UTC as hours and minutes, and optionally seconds.
    </span>OptionalSeconds,
    <span class="doccomment">/// Format offset from UTC as hours and optionally minutes and seconds.
    </span>OptionalMinutesAndSeconds,
}

<span class="doccomment">/// The separator between hours and minutes in an offset.
</span><span class="attr">#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
</span><span class="kw">pub enum </span>Colons {
    <span class="doccomment">/// No separator
    </span><span class="prelude-val">None</span>,
    <span class="doccomment">/// Colon (`:`) as separator
    </span>Colon,
    <span class="doccomment">/// No separator when formatting, colon allowed when parsing.
    </span>Maybe,
}

<span class="doccomment">/// A single formatting item. This is used for both formatting and parsing.
</span><span class="attr">#[derive(Clone, PartialEq, Eq, Debug, Hash)]
</span><span class="kw">pub enum </span>Item&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// A literally printed and parsed text.
    </span>Literal(<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str),
    <span class="doccomment">/// Same as `Literal` but with the string owned by the item.
    </span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span>OwnedLiteral(Box&lt;str&gt;),
    <span class="doccomment">/// Whitespace. Prints literally but reads zero or more whitespace.
    </span>Space(<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str),
    <span class="doccomment">/// Same as `Space` but with the string owned by the item.
    </span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span>OwnedSpace(Box&lt;str&gt;),
    <span class="doccomment">/// Numeric item. Can be optionally padded to the maximal length (if any) when formatting;
    /// the parser simply ignores any padded whitespace and zeroes.
    </span>Numeric(Numeric, Pad),
    <span class="doccomment">/// Fixed-format item.
    </span>Fixed(Fixed),
    <span class="doccomment">/// Issues a formatting error. Used to signal an invalid format string.
    </span>Error,
}

<span class="kw">const fn </span>num(numeric: Numeric) -&gt; Item&lt;<span class="lifetime">'static</span>&gt; {
    Item::Numeric(numeric, Pad::None)
}

<span class="kw">const fn </span>num0(numeric: Numeric) -&gt; Item&lt;<span class="lifetime">'static</span>&gt; {
    Item::Numeric(numeric, Pad::Zero)
}

<span class="kw">const fn </span>nums(numeric: Numeric) -&gt; Item&lt;<span class="lifetime">'static</span>&gt; {
    Item::Numeric(numeric, Pad::Space)
}

<span class="kw">const fn </span>fixed(fixed: Fixed) -&gt; Item&lt;<span class="lifetime">'static</span>&gt; {
    Item::Fixed(fixed)
}

<span class="kw">const fn </span>internal_fixed(val: InternalInternal) -&gt; Item&lt;<span class="lifetime">'static</span>&gt; {
    Item::Fixed(Fixed::Internal(InternalFixed { val }))
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Item&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// Convert items that contain a reference to the format string into an owned variant.
    </span><span class="attr">#[cfg(any(feature = <span class="string">"alloc"</span>, feature = <span class="string">"std"</span>))]
    </span><span class="kw">pub fn </span>to_owned(<span class="self">self</span>) -&gt; Item&lt;<span class="lifetime">'static</span>&gt; {
        <span class="kw">match </span><span class="self">self </span>{
            Item::Literal(s) =&gt; Item::OwnedLiteral(Box::from(s)),
            Item::Space(s) =&gt; Item::OwnedSpace(Box::from(s)),
            Item::Numeric(n, p) =&gt; Item::Numeric(n, p),
            Item::Fixed(f) =&gt; Item::Fixed(f),
            Item::OwnedLiteral(l) =&gt; Item::OwnedLiteral(l),
            Item::OwnedSpace(s) =&gt; Item::OwnedSpace(s),
            Item::Error =&gt; Item::Error,
        }
    }
}

<span class="doccomment">/// An error from the `parse` function.
</span><span class="attr">#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
</span><span class="kw">pub struct </span>ParseError(ParseErrorKind);

<span class="kw">impl </span>ParseError {
    <span class="doccomment">/// The category of parse error
    </span><span class="kw">pub const fn </span>kind(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; ParseErrorKind {
        <span class="self">self</span>.<span class="number">0
    </span>}
}

<span class="doccomment">/// The category of parse error
</span><span class="attr">#[allow(clippy::manual_non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
</span><span class="kw">pub enum </span>ParseErrorKind {
    <span class="doccomment">/// Given field is out of permitted range.
    </span>OutOfRange,

    <span class="doccomment">/// There is no possible date and time value with given set of fields.
    ///
    /// This does not include the out-of-range conditions, which are trivially invalid.
    /// It includes the case that there are one or more fields that are inconsistent to each other.
    </span>Impossible,

    <span class="doccomment">/// Given set of fields is not enough to make a requested date and time value.
    ///
    /// Note that there *may* be a case that given fields constrain the possible values so much
    /// that there is a unique possible value. Chrono only tries to be correct for
    /// most useful sets of fields however, as such constraint solving can be expensive.
    </span>NotEnough,

    <span class="doccomment">/// The input string has some invalid character sequence for given formatting items.
    </span>Invalid,

    <span class="doccomment">/// The input string has been prematurely ended.
    </span>TooShort,

    <span class="doccomment">/// All formatting items have been read but there is a remaining input.
    </span>TooLong,

    <span class="doccomment">/// There was an error on the formatting string, or there were non-supported formating items.
    </span>BadFormat,

    <span class="comment">// TODO: Change this to `#[non_exhaustive]` (on the enum) with the next breaking release.
    </span><span class="attr">#[doc(hidden)]
    </span>__Nonexhaustive,
}

<span class="doccomment">/// Same as `Result&lt;T, ParseError&gt;`.
</span><span class="kw">pub type </span>ParseResult&lt;T&gt; = <span class="prelude-ty">Result</span>&lt;T, ParseError&gt;;

<span class="kw">impl </span>fmt::Display <span class="kw">for </span>ParseError {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0 </span>{
            ParseErrorKind::OutOfRange =&gt; <span class="macro">write!</span>(f, <span class="string">"input is out of range"</span>),
            ParseErrorKind::Impossible =&gt; <span class="macro">write!</span>(f, <span class="string">"no possible date and time matching input"</span>),
            ParseErrorKind::NotEnough =&gt; <span class="macro">write!</span>(f, <span class="string">"input is not enough for unique date and time"</span>),
            ParseErrorKind::Invalid =&gt; <span class="macro">write!</span>(f, <span class="string">"input contains invalid characters"</span>),
            ParseErrorKind::TooShort =&gt; <span class="macro">write!</span>(f, <span class="string">"premature end of input"</span>),
            ParseErrorKind::TooLong =&gt; <span class="macro">write!</span>(f, <span class="string">"trailing input"</span>),
            ParseErrorKind::BadFormat =&gt; <span class="macro">write!</span>(f, <span class="string">"bad or unsupported format string"</span>),
            <span class="kw">_ </span>=&gt; <span class="macro">unreachable!</span>(),
        }
    }
}

<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">impl </span>Error <span class="kw">for </span>ParseError {
    <span class="attr">#[allow(deprecated)]
    </span><span class="kw">fn </span>description(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>str {
        <span class="string">"parser error, see to_string() for details"
    </span>}
}

<span class="comment">// to be used in this module and submodules
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">const </span>OUT_OF_RANGE: ParseError = ParseError(ParseErrorKind::OutOfRange);
<span class="kw">const </span>IMPOSSIBLE: ParseError = ParseError(ParseErrorKind::Impossible);
<span class="kw">const </span>NOT_ENOUGH: ParseError = ParseError(ParseErrorKind::NotEnough);
<span class="kw">const </span>INVALID: ParseError = ParseError(ParseErrorKind::Invalid);
<span class="kw">const </span>TOO_SHORT: ParseError = ParseError(ParseErrorKind::TooShort);
<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">const </span>TOO_LONG: ParseError = ParseError(ParseErrorKind::TooLong);
<span class="kw">const </span>BAD_FORMAT: ParseError = ParseError(ParseErrorKind::BadFormat);

<span class="comment">// this implementation is here only because we need some private code from `scan`

</span><span class="doccomment">/// Parsing a `str` into a `Weekday` uses the format [`%A`](./format/strftime/index.html).
///
/// # Example
///
/// ```
/// use chrono::Weekday;
///
/// assert_eq!("Sunday".parse::&lt;Weekday&gt;(), Ok(Weekday::Sun));
/// assert!("any day".parse::&lt;Weekday&gt;().is_err());
/// ```
///
/// The parsing is case-insensitive.
///
/// ```
/// # use chrono::Weekday;
/// assert_eq!("mON".parse::&lt;Weekday&gt;(), Ok(Weekday::Mon));
/// ```
///
/// Only the shortest form (e.g. `sun`) and the longest form (e.g. `sunday`) is accepted.
///
/// ```
/// # use chrono::Weekday;
/// assert!("thurs".parse::&lt;Weekday&gt;().is_err());
/// ```
</span><span class="kw">impl </span>FromStr <span class="kw">for </span>Weekday {
    <span class="kw">type </span><span class="prelude-val">Err </span>= ParseWeekdayError;

    <span class="kw">fn </span>from_str(s: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, <span class="self">Self</span>::Err&gt; {
        <span class="kw">if let </span><span class="prelude-val">Ok</span>((<span class="string">""</span>, w)) = scan::short_or_long_weekday(s) {
            <span class="prelude-val">Ok</span>(w)
        } <span class="kw">else </span>{
            <span class="prelude-val">Err</span>(ParseWeekdayError { _dummy: () })
        }
    }
}

<span class="doccomment">/// Parsing a `str` into a `Month` uses the format [`%B`](./format/strftime/index.html).
///
/// # Example
///
/// ```
/// use chrono::Month;
///
/// assert_eq!("January".parse::&lt;Month&gt;(), Ok(Month::January));
/// assert!("any day".parse::&lt;Month&gt;().is_err());
/// ```
///
/// The parsing is case-insensitive.
///
/// ```
/// # use chrono::Month;
/// assert_eq!("fEbruARy".parse::&lt;Month&gt;(), Ok(Month::February));
/// ```
///
/// Only the shortest form (e.g. `jan`) and the longest form (e.g. `january`) is accepted.
///
/// ```
/// # use chrono::Month;
/// assert!("septem".parse::&lt;Month&gt;().is_err());
/// assert!("Augustin".parse::&lt;Month&gt;().is_err());
/// ```
</span><span class="kw">impl </span>FromStr <span class="kw">for </span>Month {
    <span class="kw">type </span><span class="prelude-val">Err </span>= ParseMonthError;

    <span class="kw">fn </span>from_str(s: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, <span class="self">Self</span>::Err&gt; {
        <span class="kw">if let </span><span class="prelude-val">Ok</span>((<span class="string">""</span>, w)) = scan::short_or_long_month0(s) {
            <span class="kw">match </span>w {
                <span class="number">0 </span>=&gt; <span class="prelude-val">Ok</span>(Month::January),
                <span class="number">1 </span>=&gt; <span class="prelude-val">Ok</span>(Month::February),
                <span class="number">2 </span>=&gt; <span class="prelude-val">Ok</span>(Month::March),
                <span class="number">3 </span>=&gt; <span class="prelude-val">Ok</span>(Month::April),
                <span class="number">4 </span>=&gt; <span class="prelude-val">Ok</span>(Month::May),
                <span class="number">5 </span>=&gt; <span class="prelude-val">Ok</span>(Month::June),
                <span class="number">6 </span>=&gt; <span class="prelude-val">Ok</span>(Month::July),
                <span class="number">7 </span>=&gt; <span class="prelude-val">Ok</span>(Month::August),
                <span class="number">8 </span>=&gt; <span class="prelude-val">Ok</span>(Month::September),
                <span class="number">9 </span>=&gt; <span class="prelude-val">Ok</span>(Month::October),
                <span class="number">10 </span>=&gt; <span class="prelude-val">Ok</span>(Month::November),
                <span class="number">11 </span>=&gt; <span class="prelude-val">Ok</span>(Month::December),
                <span class="kw">_ </span>=&gt; <span class="prelude-val">Err</span>(ParseMonthError { _dummy: () }),
            }
        } <span class="kw">else </span>{
            <span class="prelude-val">Err</span>(ParseMonthError { _dummy: () })
        }
    }
}
</code></pre></div></section></main></body></html>