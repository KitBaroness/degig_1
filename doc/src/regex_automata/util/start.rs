<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.6/src/util/start.rs`."><title>start.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="regex_automata" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../regex_automata/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">/*!
Provides helpers for dealing with start state configurations in DFAs.
*/

</span><span class="kw">use </span><span class="kw">crate</span>::util::{
    look::LookMatcher,
    search::{Anchored, Input},
    wire::{<span class="self">self</span>, DeserializeError, SerializeError},
};

<span class="doccomment">/// The configuration used to determine a DFA's start state for a search.
///
/// A DFA has a single starting state in the typical textbook description. That
/// is, it corresponds to the set of all starting states for the NFA that built
/// it, along with their espsilon closures. In this crate, however, DFAs have
/// many possible start states due to a few factors:
///
/// * DFAs support the ability to run either anchored or unanchored searches.
/// Each type of search needs its own start state. For example, an unanchored
/// search requires starting at a state corresponding to a regex with a
/// `(?s-u:.)*?` prefix, which will match through anything.
/// * DFAs also optionally support starting an anchored search for any one
/// specific pattern. Each such pattern requires its own start state.
/// * If a look-behind assertion like `^` or `\b` is used in the regex, then
/// the DFA will need to inspect a single byte immediately before the start of
/// the search to choose the correct start state.
///
/// Indeed, this configuration precisely encapsulates all of the above factors.
/// The [`Config::anchored`] method sets which kind of anchored search to
/// perform while the [`Config::look_behind`] method provides a way to set
/// the byte that occurs immediately before the start of the search.
///
/// Generally speaking, this type is only useful when you want to run searches
/// without using an [`Input`]. In particular, an `Input` wants a haystack
/// slice, but callers may not have a contiguous sequence of bytes as a
/// haystack in all cases. This type provides a lower level of control such
/// that callers can provide their own anchored configuration and look-behind
/// byte explicitly.
///
/// # Example
///
/// This shows basic usage that permits running a search with a DFA without
/// using the `Input` abstraction.
///
/// ```
/// use regex_automata::{
///     dfa::{Automaton, dense},
///     util::start,
///     Anchored,
/// };
///
/// let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
/// let haystack = "quartz";
///
/// let config = start::Config::new().anchored(Anchored::Yes);
/// let mut state = dfa.start_state(&amp;config)?;
/// for &amp;b in haystack.as_bytes().iter() {
///     state = dfa.next_state(state, b);
/// }
/// state = dfa.next_eoi_state(state);
/// assert!(dfa.is_match_state(state));
///
/// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
/// ```
///
/// This example shows how to correctly run a search that doesn't begin at
/// the start of a haystack. Notice how we set the look-behind byte, and as
/// a result, the `\b` assertion does not match.
///
/// ```
/// use regex_automata::{
///     dfa::{Automaton, dense},
///     util::start,
///     Anchored,
/// };
///
/// let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
/// let haystack = "quartz";
///
/// let config = start::Config::new()
///     .anchored(Anchored::Yes)
///     .look_behind(Some(b'q'));
/// let mut state = dfa.start_state(&amp;config)?;
/// for &amp;b in haystack.as_bytes().iter().skip(1) {
///     state = dfa.next_state(state, b);
/// }
/// state = dfa.next_eoi_state(state);
/// // No match!
/// assert!(!dfa.is_match_state(state));
///
/// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
/// ```
///
/// If we had instead not set a look-behind byte, then the DFA would assume
/// that it was starting at the beginning of the haystack, and thus `\b` should
/// match. This in turn would result in erroneously reporting a match:
///
/// ```
/// use regex_automata::{
///     dfa::{Automaton, dense},
///     util::start,
///     Anchored,
/// };
///
/// let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
/// let haystack = "quartz";
///
/// // Whoops, forgot the look-behind byte...
/// let config = start::Config::new().anchored(Anchored::Yes);
/// let mut state = dfa.start_state(&amp;config)?;
/// for &amp;b in haystack.as_bytes().iter().skip(1) {
///     state = dfa.next_state(state, b);
/// }
/// state = dfa.next_eoi_state(state);
/// // And now we get a match unexpectedly.
/// assert!(dfa.is_match_state(state));
///
/// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
/// ```
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>Config {
    look_behind: <span class="prelude-ty">Option</span>&lt;u8&gt;,
    anchored: Anchored,
}

<span class="kw">impl </span>Config {
    <span class="doccomment">/// Create a new default start configuration.
    ///
    /// The default is an unanchored search that starts at the beginning of the
    /// haystack.
    </span><span class="kw">pub fn </span>new() -&gt; Config {
        Config { anchored: Anchored::No, look_behind: <span class="prelude-val">None </span>}
    }

    <span class="doccomment">/// A convenience routine for building a start configuration from an
    /// [`Input`] for a forward search.
    ///
    /// This automatically sets the look-behind byte to the byte immediately
    /// preceding the start of the search. If the start of the search is at
    /// offset `0`, then no look-behind byte is set.
    </span><span class="kw">pub fn </span>from_input_forward(input: <span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;) -&gt; Config {
        <span class="kw">let </span>look_behind = input
            .start()
            .checked_sub(<span class="number">1</span>)
            .and_then(|i| input.haystack().get(i).copied());
        Config { look_behind, anchored: input.get_anchored() }
    }

    <span class="doccomment">/// A convenience routine for building a start configuration from an
    /// [`Input`] for a reverse search.
    ///
    /// This automatically sets the look-behind byte to the byte immediately
    /// following the end of the search. If the end of the search is at
    /// offset `haystack.len()`, then no look-behind byte is set.
    </span><span class="kw">pub fn </span>from_input_reverse(input: <span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;) -&gt; Config {
        <span class="kw">let </span>look_behind = input.haystack().get(input.end()).copied();
        Config { look_behind, anchored: input.get_anchored() }
    }

    <span class="doccomment">/// Set the look-behind byte at the start of a search.
    ///
    /// Unless the search is intended to logically start at the beginning of a
    /// haystack, this should _always_ be set to the byte immediately preceding
    /// the start of the search. If no look-behind byte is set, then the start
    /// configuration will assume it is at the beginning of the haystack. For
    /// example, the anchor `^` will match.
    ///
    /// The default is that no look-behind byte is set.
    </span><span class="kw">pub fn </span>look_behind(<span class="kw-2">mut </span><span class="self">self</span>, byte: <span class="prelude-ty">Option</span>&lt;u8&gt;) -&gt; Config {
        <span class="self">self</span>.look_behind = byte;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Set the anchored mode of a search.
    ///
    /// The default is an unanchored search.
    </span><span class="kw">pub fn </span>anchored(<span class="kw-2">mut </span><span class="self">self</span>, mode: Anchored) -&gt; Config {
        <span class="self">self</span>.anchored = mode;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Return the look-behind byte in this configuration, if one exists.
    </span><span class="kw">pub fn </span>get_look_behind(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;u8&gt; {
        <span class="self">self</span>.look_behind
    }

    <span class="doccomment">/// Return the anchored mode in this configuration.
    </span><span class="kw">pub fn </span>get_anchored(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Anchored {
        <span class="self">self</span>.anchored
    }
}

<span class="doccomment">/// A map from every possible byte value to its corresponding starting
/// configuration.
///
/// This map is used in order to lookup the start configuration for a particular
/// position in a haystack. This start configuration is then used in
/// combination with things like the anchored mode and pattern ID to fully
/// determine the start state.
///
/// Generally speaking, this map is only used for fully compiled DFAs and lazy
/// DFAs. For NFAs (including the one-pass DFA), the start state is generally
/// selected by virtue of traversing the NFA state graph. DFAs do the same
/// thing, but at build time and not search time. (Well, technically the lazy
/// DFA does it at search time, but it does enough work to cache the full
/// result of the epsilon closure that the NFA engines tend to need to do.)
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>StartByteMap {
    map: [Start; <span class="number">256</span>],
}

<span class="kw">impl </span>StartByteMap {
    <span class="doccomment">/// Create a new map from byte values to their corresponding starting
    /// configurations. The map is determined, in part, by how look-around
    /// assertions are matched via the matcher given.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>new(lookm: <span class="kw-2">&amp;</span>LookMatcher) -&gt; StartByteMap {
        <span class="kw">let </span><span class="kw-2">mut </span>map = [Start::NonWordByte; <span class="number">256</span>];
        map[usize::from(<span class="string">b'\n'</span>)] = Start::LineLF;
        map[usize::from(<span class="string">b'\r'</span>)] = Start::LineCR;
        map[usize::from(<span class="string">b'_'</span>)] = Start::WordByte;

        <span class="kw">let </span><span class="kw-2">mut </span>byte = <span class="string">b'0'</span>;
        <span class="kw">while </span>byte &lt;= <span class="string">b'9' </span>{
            map[usize::from(byte)] = Start::WordByte;
            byte += <span class="number">1</span>;
        }
        byte = <span class="string">b'A'</span>;
        <span class="kw">while </span>byte &lt;= <span class="string">b'Z' </span>{
            map[usize::from(byte)] = Start::WordByte;
            byte += <span class="number">1</span>;
        }
        byte = <span class="string">b'a'</span>;
        <span class="kw">while </span>byte &lt;= <span class="string">b'z' </span>{
            map[usize::from(byte)] = Start::WordByte;
            byte += <span class="number">1</span>;
        }

        <span class="kw">let </span>lineterm = lookm.get_line_terminator();
        <span class="comment">// If our line terminator is normal, then it is already handled by
        // the LineLF and LineCR configurations. But if it's weird, then we
        // overwrite whatever was there before for that terminator with a
        // special configuration. The trick here is that if the terminator
        // is, say, a word byte like `a`, then callers seeing this start
        // configuration need to account for that and build their DFA state as
        // if it *also* came from a word byte.
        </span><span class="kw">if </span>lineterm != <span class="string">b'\r' </span>&amp;&amp; lineterm != <span class="string">b'\n' </span>{
            map[usize::from(lineterm)] = Start::CustomLineTerminator;
        }
        StartByteMap { map }
    }

    <span class="doccomment">/// Return the starting configuration for the given look-behind byte.
    ///
    /// If no look-behind exists, callers should use `Start::Text`.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"perf-inline"</span>, inline(always))]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>get(<span class="kw-2">&amp;</span><span class="self">self</span>, byte: u8) -&gt; Start {
        <span class="self">self</span>.map[usize::from(byte)]
    }

    <span class="doccomment">/// Deserializes a byte class map from the given slice. If the slice is of
    /// insufficient length or otherwise contains an impossible mapping, then
    /// an error is returned. Upon success, the number of bytes read along with
    /// the map are returned. The number of bytes read is always a multiple of
    /// 8.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>from_bytes(
        slice: <span class="kw-2">&amp;</span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(StartByteMap, usize), DeserializeError&gt; {
        wire::check_slice_len(slice, <span class="number">256</span>, <span class="string">"start byte map"</span>)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>map = [Start::NonWordByte; <span class="number">256</span>];
        <span class="kw">for </span>(i, <span class="kw-2">&amp;</span>repr) <span class="kw">in </span>slice[..<span class="number">256</span>].iter().enumerate() {
            map[i] = <span class="kw">match </span>Start::from_usize(usize::from(repr)) {
                <span class="prelude-val">Some</span>(start) =&gt; start,
                <span class="prelude-val">None </span>=&gt; {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(DeserializeError::generic(
                        <span class="string">"found invalid starting configuration"</span>,
                    ))
                }
            };
        }
        <span class="prelude-val">Ok</span>((StartByteMap { map }, <span class="number">256</span>))
    }

    <span class="doccomment">/// Writes this map to the given byte buffer. if the given buffer is too
    /// small, then an error is returned. Upon success, the total number of
    /// bytes written is returned. The number of bytes written is guaranteed to
    /// be a multiple of 8.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>write_to(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        dst: <span class="kw-2">&amp;mut </span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;usize, SerializeError&gt; {
        <span class="kw">let </span>nwrite = <span class="self">self</span>.write_to_len();
        <span class="kw">if </span>dst.len() &lt; nwrite {
            <span class="kw">return </span><span class="prelude-val">Err</span>(SerializeError::buffer_too_small(<span class="string">"start byte map"</span>));
        }
        <span class="kw">for </span>(i, <span class="kw-2">&amp;</span>start) <span class="kw">in </span><span class="self">self</span>.map.iter().enumerate() {
            dst[i] = start.as_u8();
        }
        <span class="prelude-val">Ok</span>(nwrite)
    }

    <span class="doccomment">/// Returns the total number of bytes written by `write_to`.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>write_to_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="number">256
    </span>}
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>StartByteMap {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        <span class="kw">use </span><span class="kw">crate</span>::util::escape::DebugByte;

        <span class="macro">write!</span>(f, <span class="string">"StartByteMap{{"</span>)<span class="question-mark">?</span>;
        <span class="kw">for </span>byte <span class="kw">in </span><span class="number">0</span>..=<span class="number">255 </span>{
            <span class="kw">if </span>byte &gt; <span class="number">0 </span>{
                <span class="macro">write!</span>(f, <span class="string">", "</span>)<span class="question-mark">?</span>;
            }
            <span class="kw">let </span>start = <span class="self">self</span>.map[usize::from(byte)];
            <span class="macro">write!</span>(f, <span class="string">"{:?} =&gt; {:?}"</span>, DebugByte(byte), start)<span class="question-mark">?</span>;
        }
        <span class="macro">write!</span>(f, <span class="string">"}}"</span>)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="doccomment">/// Represents the six possible starting configurations of a DFA search.
///
/// The starting configuration is determined by inspecting the the beginning
/// of the haystack (up to 1 byte). Ultimately, this along with a pattern ID
/// (if specified) and the type of search (anchored or not) is what selects the
/// start state to use in a DFA.
///
/// As one example, if a DFA only supports unanchored searches and does not
/// support anchored searches for each pattern, then it will have at most 6
/// distinct start states. (Some start states may be reused if determinization
/// can determine that they will be equivalent.) If the DFA supports both
/// anchored and unanchored searches, then it will have a maximum of 12
/// distinct start states. Finally, if the DFA also supports anchored searches
/// for each pattern, then it can have up to `12 + (N * 6)` start states, where
/// `N` is the number of patterns.
///
/// Handling each of these starting configurations in the context of DFA
/// determinization can be *quite* tricky and subtle. But the code is small
/// and can be found at `crate::util::determinize::set_lookbehind_from_start`.
</span><span class="attr">#[derive(Clone, Copy, Debug, Eq, PartialEq)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">enum </span>Start {
    <span class="doccomment">/// This occurs when the starting position is not any of the ones below.
    </span>NonWordByte = <span class="number">0</span>,
    <span class="doccomment">/// This occurs when the byte immediately preceding the start of the search
    /// is an ASCII word byte.
    </span>WordByte = <span class="number">1</span>,
    <span class="doccomment">/// This occurs when the starting position of the search corresponds to the
    /// beginning of the haystack.
    </span>Text = <span class="number">2</span>,
    <span class="doccomment">/// This occurs when the byte immediately preceding the start of the search
    /// is a line terminator. Specifically, `\n`.
    </span>LineLF = <span class="number">3</span>,
    <span class="doccomment">/// This occurs when the byte immediately preceding the start of the search
    /// is a line terminator. Specifically, `\r`.
    </span>LineCR = <span class="number">4</span>,
    <span class="doccomment">/// This occurs when a custom line terminator has been set via a
    /// `LookMatcher`, and when that line terminator is neither a `\r` or a
    /// `\n`.
    ///
    /// If the custom line terminator is a word byte, then this start
    /// configuration is still selected. DFAs that implement word boundary
    /// assertions will likely need to check whether the custom line terminator
    /// is a word byte, in which case, it should behave as if the byte
    /// satisfies `\b` in addition to multi-line anchors.
    </span>CustomLineTerminator = <span class="number">5</span>,
}

<span class="kw">impl </span>Start {
    <span class="doccomment">/// Return the starting state corresponding to the given integer. If no
    /// starting state exists for the given integer, then None is returned.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>from_usize(n: usize) -&gt; <span class="prelude-ty">Option</span>&lt;Start&gt; {
        <span class="kw">match </span>n {
            <span class="number">0 </span>=&gt; <span class="prelude-val">Some</span>(Start::NonWordByte),
            <span class="number">1 </span>=&gt; <span class="prelude-val">Some</span>(Start::WordByte),
            <span class="number">2 </span>=&gt; <span class="prelude-val">Some</span>(Start::Text),
            <span class="number">3 </span>=&gt; <span class="prelude-val">Some</span>(Start::LineLF),
            <span class="number">4 </span>=&gt; <span class="prelude-val">Some</span>(Start::LineCR),
            <span class="number">5 </span>=&gt; <span class="prelude-val">Some</span>(Start::CustomLineTerminator),
            <span class="kw">_ </span>=&gt; <span class="prelude-val">None</span>,
        }
    }

    <span class="doccomment">/// Returns the total number of starting state configurations.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>len() -&gt; usize {
        <span class="number">6
    </span>}

    <span class="doccomment">/// Return this starting configuration as `u8` integer. It is guaranteed to
    /// be less than `Start::len()`.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"perf-inline"</span>, inline(always))]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>as_u8(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u8 {
        <span class="comment">// AFAIK, 'as' is the only way to zero-cost convert an int enum to an
        // actual int.
        </span><span class="kw-2">*</span><span class="self">self </span><span class="kw">as </span>u8
    }

    <span class="doccomment">/// Return this starting configuration as a `usize` integer. It is
    /// guaranteed to be less than `Start::len()`.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"perf-inline"</span>, inline(always))]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>as_usize(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        usize::from(<span class="self">self</span>.as_u8())
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>start_fwd_done_range() {
        <span class="kw">let </span>smap = StartByteMap::new(<span class="kw-2">&amp;</span>LookMatcher::default());
        <span class="kw">let </span>input = Input::new(<span class="string">""</span>).range(<span class="number">1</span>..<span class="number">0</span>);
        <span class="kw">let </span>config = Config::from_input_forward(<span class="kw-2">&amp;</span>input);
        <span class="kw">let </span>start =
            config.get_look_behind().map_or(Start::Text, |b| smap.get(b));
        <span class="macro">assert_eq!</span>(Start::Text, start);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>start_rev_done_range() {
        <span class="kw">let </span>smap = StartByteMap::new(<span class="kw-2">&amp;</span>LookMatcher::default());
        <span class="kw">let </span>input = Input::new(<span class="string">""</span>).range(<span class="number">1</span>..<span class="number">0</span>);
        <span class="kw">let </span>config = Config::from_input_reverse(<span class="kw-2">&amp;</span>input);
        <span class="kw">let </span>start =
            config.get_look_behind().map_or(Start::Text, |b| smap.get(b));
        <span class="macro">assert_eq!</span>(Start::Text, start);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>start_fwd() {
        <span class="kw">let </span>f = |haystack, start, end| {
            <span class="kw">let </span>smap = StartByteMap::new(<span class="kw-2">&amp;</span>LookMatcher::default());
            <span class="kw">let </span>input = Input::new(haystack).range(start..end);
            <span class="kw">let </span>config = Config::from_input_forward(<span class="kw-2">&amp;</span>input);
            <span class="kw">let </span>start =
                config.get_look_behind().map_or(Start::Text, |b| smap.get(b));
            start
        };

        <span class="macro">assert_eq!</span>(Start::Text, f(<span class="string">""</span>, <span class="number">0</span>, <span class="number">0</span>));
        <span class="macro">assert_eq!</span>(Start::Text, f(<span class="string">"abc"</span>, <span class="number">0</span>, <span class="number">3</span>));
        <span class="macro">assert_eq!</span>(Start::Text, f(<span class="string">"\nabc"</span>, <span class="number">0</span>, <span class="number">3</span>));

        <span class="macro">assert_eq!</span>(Start::LineLF, f(<span class="string">"\nabc"</span>, <span class="number">1</span>, <span class="number">3</span>));

        <span class="macro">assert_eq!</span>(Start::LineCR, f(<span class="string">"\rabc"</span>, <span class="number">1</span>, <span class="number">3</span>));

        <span class="macro">assert_eq!</span>(Start::WordByte, f(<span class="string">"abc"</span>, <span class="number">1</span>, <span class="number">3</span>));

        <span class="macro">assert_eq!</span>(Start::NonWordByte, f(<span class="string">" abc"</span>, <span class="number">1</span>, <span class="number">3</span>));
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>start_rev() {
        <span class="kw">let </span>f = |haystack, start, end| {
            <span class="kw">let </span>smap = StartByteMap::new(<span class="kw-2">&amp;</span>LookMatcher::default());
            <span class="kw">let </span>input = Input::new(haystack).range(start..end);
            <span class="kw">let </span>config = Config::from_input_reverse(<span class="kw-2">&amp;</span>input);
            <span class="kw">let </span>start =
                config.get_look_behind().map_or(Start::Text, |b| smap.get(b));
            start
        };

        <span class="macro">assert_eq!</span>(Start::Text, f(<span class="string">""</span>, <span class="number">0</span>, <span class="number">0</span>));
        <span class="macro">assert_eq!</span>(Start::Text, f(<span class="string">"abc"</span>, <span class="number">0</span>, <span class="number">3</span>));
        <span class="macro">assert_eq!</span>(Start::Text, f(<span class="string">"abc\n"</span>, <span class="number">0</span>, <span class="number">4</span>));

        <span class="macro">assert_eq!</span>(Start::LineLF, f(<span class="string">"abc\nz"</span>, <span class="number">0</span>, <span class="number">3</span>));

        <span class="macro">assert_eq!</span>(Start::LineCR, f(<span class="string">"abc\rz"</span>, <span class="number">0</span>, <span class="number">3</span>));

        <span class="macro">assert_eq!</span>(Start::WordByte, f(<span class="string">"abc"</span>, <span class="number">0</span>, <span class="number">2</span>));

        <span class="macro">assert_eq!</span>(Start::NonWordByte, f(<span class="string">"abc "</span>, <span class="number">0</span>, <span class="number">3</span>));
    }
}
</code></pre></div></section></main></body></html>