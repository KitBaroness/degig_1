<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.6/src/hybrid/dfa.rs`."><title>dfa.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="regex_automata" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../regex_automata/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#756" id="756">756</a>
<a href="#757" id="757">757</a>
<a href="#758" id="758">758</a>
<a href="#759" id="759">759</a>
<a href="#760" id="760">760</a>
<a href="#761" id="761">761</a>
<a href="#762" id="762">762</a>
<a href="#763" id="763">763</a>
<a href="#764" id="764">764</a>
<a href="#765" id="765">765</a>
<a href="#766" id="766">766</a>
<a href="#767" id="767">767</a>
<a href="#768" id="768">768</a>
<a href="#769" id="769">769</a>
<a href="#770" id="770">770</a>
<a href="#771" id="771">771</a>
<a href="#772" id="772">772</a>
<a href="#773" id="773">773</a>
<a href="#774" id="774">774</a>
<a href="#775" id="775">775</a>
<a href="#776" id="776">776</a>
<a href="#777" id="777">777</a>
<a href="#778" id="778">778</a>
<a href="#779" id="779">779</a>
<a href="#780" id="780">780</a>
<a href="#781" id="781">781</a>
<a href="#782" id="782">782</a>
<a href="#783" id="783">783</a>
<a href="#784" id="784">784</a>
<a href="#785" id="785">785</a>
<a href="#786" id="786">786</a>
<a href="#787" id="787">787</a>
<a href="#788" id="788">788</a>
<a href="#789" id="789">789</a>
<a href="#790" id="790">790</a>
<a href="#791" id="791">791</a>
<a href="#792" id="792">792</a>
<a href="#793" id="793">793</a>
<a href="#794" id="794">794</a>
<a href="#795" id="795">795</a>
<a href="#796" id="796">796</a>
<a href="#797" id="797">797</a>
<a href="#798" id="798">798</a>
<a href="#799" id="799">799</a>
<a href="#800" id="800">800</a>
<a href="#801" id="801">801</a>
<a href="#802" id="802">802</a>
<a href="#803" id="803">803</a>
<a href="#804" id="804">804</a>
<a href="#805" id="805">805</a>
<a href="#806" id="806">806</a>
<a href="#807" id="807">807</a>
<a href="#808" id="808">808</a>
<a href="#809" id="809">809</a>
<a href="#810" id="810">810</a>
<a href="#811" id="811">811</a>
<a href="#812" id="812">812</a>
<a href="#813" id="813">813</a>
<a href="#814" id="814">814</a>
<a href="#815" id="815">815</a>
<a href="#816" id="816">816</a>
<a href="#817" id="817">817</a>
<a href="#818" id="818">818</a>
<a href="#819" id="819">819</a>
<a href="#820" id="820">820</a>
<a href="#821" id="821">821</a>
<a href="#822" id="822">822</a>
<a href="#823" id="823">823</a>
<a href="#824" id="824">824</a>
<a href="#825" id="825">825</a>
<a href="#826" id="826">826</a>
<a href="#827" id="827">827</a>
<a href="#828" id="828">828</a>
<a href="#829" id="829">829</a>
<a href="#830" id="830">830</a>
<a href="#831" id="831">831</a>
<a href="#832" id="832">832</a>
<a href="#833" id="833">833</a>
<a href="#834" id="834">834</a>
<a href="#835" id="835">835</a>
<a href="#836" id="836">836</a>
<a href="#837" id="837">837</a>
<a href="#838" id="838">838</a>
<a href="#839" id="839">839</a>
<a href="#840" id="840">840</a>
<a href="#841" id="841">841</a>
<a href="#842" id="842">842</a>
<a href="#843" id="843">843</a>
<a href="#844" id="844">844</a>
<a href="#845" id="845">845</a>
<a href="#846" id="846">846</a>
<a href="#847" id="847">847</a>
<a href="#848" id="848">848</a>
<a href="#849" id="849">849</a>
<a href="#850" id="850">850</a>
<a href="#851" id="851">851</a>
<a href="#852" id="852">852</a>
<a href="#853" id="853">853</a>
<a href="#854" id="854">854</a>
<a href="#855" id="855">855</a>
<a href="#856" id="856">856</a>
<a href="#857" id="857">857</a>
<a href="#858" id="858">858</a>
<a href="#859" id="859">859</a>
<a href="#860" id="860">860</a>
<a href="#861" id="861">861</a>
<a href="#862" id="862">862</a>
<a href="#863" id="863">863</a>
<a href="#864" id="864">864</a>
<a href="#865" id="865">865</a>
<a href="#866" id="866">866</a>
<a href="#867" id="867">867</a>
<a href="#868" id="868">868</a>
<a href="#869" id="869">869</a>
<a href="#870" id="870">870</a>
<a href="#871" id="871">871</a>
<a href="#872" id="872">872</a>
<a href="#873" id="873">873</a>
<a href="#874" id="874">874</a>
<a href="#875" id="875">875</a>
<a href="#876" id="876">876</a>
<a href="#877" id="877">877</a>
<a href="#878" id="878">878</a>
<a href="#879" id="879">879</a>
<a href="#880" id="880">880</a>
<a href="#881" id="881">881</a>
<a href="#882" id="882">882</a>
<a href="#883" id="883">883</a>
<a href="#884" id="884">884</a>
<a href="#885" id="885">885</a>
<a href="#886" id="886">886</a>
<a href="#887" id="887">887</a>
<a href="#888" id="888">888</a>
<a href="#889" id="889">889</a>
<a href="#890" id="890">890</a>
<a href="#891" id="891">891</a>
<a href="#892" id="892">892</a>
<a href="#893" id="893">893</a>
<a href="#894" id="894">894</a>
<a href="#895" id="895">895</a>
<a href="#896" id="896">896</a>
<a href="#897" id="897">897</a>
<a href="#898" id="898">898</a>
<a href="#899" id="899">899</a>
<a href="#900" id="900">900</a>
<a href="#901" id="901">901</a>
<a href="#902" id="902">902</a>
<a href="#903" id="903">903</a>
<a href="#904" id="904">904</a>
<a href="#905" id="905">905</a>
<a href="#906" id="906">906</a>
<a href="#907" id="907">907</a>
<a href="#908" id="908">908</a>
<a href="#909" id="909">909</a>
<a href="#910" id="910">910</a>
<a href="#911" id="911">911</a>
<a href="#912" id="912">912</a>
<a href="#913" id="913">913</a>
<a href="#914" id="914">914</a>
<a href="#915" id="915">915</a>
<a href="#916" id="916">916</a>
<a href="#917" id="917">917</a>
<a href="#918" id="918">918</a>
<a href="#919" id="919">919</a>
<a href="#920" id="920">920</a>
<a href="#921" id="921">921</a>
<a href="#922" id="922">922</a>
<a href="#923" id="923">923</a>
<a href="#924" id="924">924</a>
<a href="#925" id="925">925</a>
<a href="#926" id="926">926</a>
<a href="#927" id="927">927</a>
<a href="#928" id="928">928</a>
<a href="#929" id="929">929</a>
<a href="#930" id="930">930</a>
<a href="#931" id="931">931</a>
<a href="#932" id="932">932</a>
<a href="#933" id="933">933</a>
<a href="#934" id="934">934</a>
<a href="#935" id="935">935</a>
<a href="#936" id="936">936</a>
<a href="#937" id="937">937</a>
<a href="#938" id="938">938</a>
<a href="#939" id="939">939</a>
<a href="#940" id="940">940</a>
<a href="#941" id="941">941</a>
<a href="#942" id="942">942</a>
<a href="#943" id="943">943</a>
<a href="#944" id="944">944</a>
<a href="#945" id="945">945</a>
<a href="#946" id="946">946</a>
<a href="#947" id="947">947</a>
<a href="#948" id="948">948</a>
<a href="#949" id="949">949</a>
<a href="#950" id="950">950</a>
<a href="#951" id="951">951</a>
<a href="#952" id="952">952</a>
<a href="#953" id="953">953</a>
<a href="#954" id="954">954</a>
<a href="#955" id="955">955</a>
<a href="#956" id="956">956</a>
<a href="#957" id="957">957</a>
<a href="#958" id="958">958</a>
<a href="#959" id="959">959</a>
<a href="#960" id="960">960</a>
<a href="#961" id="961">961</a>
<a href="#962" id="962">962</a>
<a href="#963" id="963">963</a>
<a href="#964" id="964">964</a>
<a href="#965" id="965">965</a>
<a href="#966" id="966">966</a>
<a href="#967" id="967">967</a>
<a href="#968" id="968">968</a>
<a href="#969" id="969">969</a>
<a href="#970" id="970">970</a>
<a href="#971" id="971">971</a>
<a href="#972" id="972">972</a>
<a href="#973" id="973">973</a>
<a href="#974" id="974">974</a>
<a href="#975" id="975">975</a>
<a href="#976" id="976">976</a>
<a href="#977" id="977">977</a>
<a href="#978" id="978">978</a>
<a href="#979" id="979">979</a>
<a href="#980" id="980">980</a>
<a href="#981" id="981">981</a>
<a href="#982" id="982">982</a>
<a href="#983" id="983">983</a>
<a href="#984" id="984">984</a>
<a href="#985" id="985">985</a>
<a href="#986" id="986">986</a>
<a href="#987" id="987">987</a>
<a href="#988" id="988">988</a>
<a href="#989" id="989">989</a>
<a href="#990" id="990">990</a>
<a href="#991" id="991">991</a>
<a href="#992" id="992">992</a>
<a href="#993" id="993">993</a>
<a href="#994" id="994">994</a>
<a href="#995" id="995">995</a>
<a href="#996" id="996">996</a>
<a href="#997" id="997">997</a>
<a href="#998" id="998">998</a>
<a href="#999" id="999">999</a>
<a href="#1000" id="1000">1000</a>
<a href="#1001" id="1001">1001</a>
<a href="#1002" id="1002">1002</a>
<a href="#1003" id="1003">1003</a>
<a href="#1004" id="1004">1004</a>
<a href="#1005" id="1005">1005</a>
<a href="#1006" id="1006">1006</a>
<a href="#1007" id="1007">1007</a>
<a href="#1008" id="1008">1008</a>
<a href="#1009" id="1009">1009</a>
<a href="#1010" id="1010">1010</a>
<a href="#1011" id="1011">1011</a>
<a href="#1012" id="1012">1012</a>
<a href="#1013" id="1013">1013</a>
<a href="#1014" id="1014">1014</a>
<a href="#1015" id="1015">1015</a>
<a href="#1016" id="1016">1016</a>
<a href="#1017" id="1017">1017</a>
<a href="#1018" id="1018">1018</a>
<a href="#1019" id="1019">1019</a>
<a href="#1020" id="1020">1020</a>
<a href="#1021" id="1021">1021</a>
<a href="#1022" id="1022">1022</a>
<a href="#1023" id="1023">1023</a>
<a href="#1024" id="1024">1024</a>
<a href="#1025" id="1025">1025</a>
<a href="#1026" id="1026">1026</a>
<a href="#1027" id="1027">1027</a>
<a href="#1028" id="1028">1028</a>
<a href="#1029" id="1029">1029</a>
<a href="#1030" id="1030">1030</a>
<a href="#1031" id="1031">1031</a>
<a href="#1032" id="1032">1032</a>
<a href="#1033" id="1033">1033</a>
<a href="#1034" id="1034">1034</a>
<a href="#1035" id="1035">1035</a>
<a href="#1036" id="1036">1036</a>
<a href="#1037" id="1037">1037</a>
<a href="#1038" id="1038">1038</a>
<a href="#1039" id="1039">1039</a>
<a href="#1040" id="1040">1040</a>
<a href="#1041" id="1041">1041</a>
<a href="#1042" id="1042">1042</a>
<a href="#1043" id="1043">1043</a>
<a href="#1044" id="1044">1044</a>
<a href="#1045" id="1045">1045</a>
<a href="#1046" id="1046">1046</a>
<a href="#1047" id="1047">1047</a>
<a href="#1048" id="1048">1048</a>
<a href="#1049" id="1049">1049</a>
<a href="#1050" id="1050">1050</a>
<a href="#1051" id="1051">1051</a>
<a href="#1052" id="1052">1052</a>
<a href="#1053" id="1053">1053</a>
<a href="#1054" id="1054">1054</a>
<a href="#1055" id="1055">1055</a>
<a href="#1056" id="1056">1056</a>
<a href="#1057" id="1057">1057</a>
<a href="#1058" id="1058">1058</a>
<a href="#1059" id="1059">1059</a>
<a href="#1060" id="1060">1060</a>
<a href="#1061" id="1061">1061</a>
<a href="#1062" id="1062">1062</a>
<a href="#1063" id="1063">1063</a>
<a href="#1064" id="1064">1064</a>
<a href="#1065" id="1065">1065</a>
<a href="#1066" id="1066">1066</a>
<a href="#1067" id="1067">1067</a>
<a href="#1068" id="1068">1068</a>
<a href="#1069" id="1069">1069</a>
<a href="#1070" id="1070">1070</a>
<a href="#1071" id="1071">1071</a>
<a href="#1072" id="1072">1072</a>
<a href="#1073" id="1073">1073</a>
<a href="#1074" id="1074">1074</a>
<a href="#1075" id="1075">1075</a>
<a href="#1076" id="1076">1076</a>
<a href="#1077" id="1077">1077</a>
<a href="#1078" id="1078">1078</a>
<a href="#1079" id="1079">1079</a>
<a href="#1080" id="1080">1080</a>
<a href="#1081" id="1081">1081</a>
<a href="#1082" id="1082">1082</a>
<a href="#1083" id="1083">1083</a>
<a href="#1084" id="1084">1084</a>
<a href="#1085" id="1085">1085</a>
<a href="#1086" id="1086">1086</a>
<a href="#1087" id="1087">1087</a>
<a href="#1088" id="1088">1088</a>
<a href="#1089" id="1089">1089</a>
<a href="#1090" id="1090">1090</a>
<a href="#1091" id="1091">1091</a>
<a href="#1092" id="1092">1092</a>
<a href="#1093" id="1093">1093</a>
<a href="#1094" id="1094">1094</a>
<a href="#1095" id="1095">1095</a>
<a href="#1096" id="1096">1096</a>
<a href="#1097" id="1097">1097</a>
<a href="#1098" id="1098">1098</a>
<a href="#1099" id="1099">1099</a>
<a href="#1100" id="1100">1100</a>
<a href="#1101" id="1101">1101</a>
<a href="#1102" id="1102">1102</a>
<a href="#1103" id="1103">1103</a>
<a href="#1104" id="1104">1104</a>
<a href="#1105" id="1105">1105</a>
<a href="#1106" id="1106">1106</a>
<a href="#1107" id="1107">1107</a>
<a href="#1108" id="1108">1108</a>
<a href="#1109" id="1109">1109</a>
<a href="#1110" id="1110">1110</a>
<a href="#1111" id="1111">1111</a>
<a href="#1112" id="1112">1112</a>
<a href="#1113" id="1113">1113</a>
<a href="#1114" id="1114">1114</a>
<a href="#1115" id="1115">1115</a>
<a href="#1116" id="1116">1116</a>
<a href="#1117" id="1117">1117</a>
<a href="#1118" id="1118">1118</a>
<a href="#1119" id="1119">1119</a>
<a href="#1120" id="1120">1120</a>
<a href="#1121" id="1121">1121</a>
<a href="#1122" id="1122">1122</a>
<a href="#1123" id="1123">1123</a>
<a href="#1124" id="1124">1124</a>
<a href="#1125" id="1125">1125</a>
<a href="#1126" id="1126">1126</a>
<a href="#1127" id="1127">1127</a>
<a href="#1128" id="1128">1128</a>
<a href="#1129" id="1129">1129</a>
<a href="#1130" id="1130">1130</a>
<a href="#1131" id="1131">1131</a>
<a href="#1132" id="1132">1132</a>
<a href="#1133" id="1133">1133</a>
<a href="#1134" id="1134">1134</a>
<a href="#1135" id="1135">1135</a>
<a href="#1136" id="1136">1136</a>
<a href="#1137" id="1137">1137</a>
<a href="#1138" id="1138">1138</a>
<a href="#1139" id="1139">1139</a>
<a href="#1140" id="1140">1140</a>
<a href="#1141" id="1141">1141</a>
<a href="#1142" id="1142">1142</a>
<a href="#1143" id="1143">1143</a>
<a href="#1144" id="1144">1144</a>
<a href="#1145" id="1145">1145</a>
<a href="#1146" id="1146">1146</a>
<a href="#1147" id="1147">1147</a>
<a href="#1148" id="1148">1148</a>
<a href="#1149" id="1149">1149</a>
<a href="#1150" id="1150">1150</a>
<a href="#1151" id="1151">1151</a>
<a href="#1152" id="1152">1152</a>
<a href="#1153" id="1153">1153</a>
<a href="#1154" id="1154">1154</a>
<a href="#1155" id="1155">1155</a>
<a href="#1156" id="1156">1156</a>
<a href="#1157" id="1157">1157</a>
<a href="#1158" id="1158">1158</a>
<a href="#1159" id="1159">1159</a>
<a href="#1160" id="1160">1160</a>
<a href="#1161" id="1161">1161</a>
<a href="#1162" id="1162">1162</a>
<a href="#1163" id="1163">1163</a>
<a href="#1164" id="1164">1164</a>
<a href="#1165" id="1165">1165</a>
<a href="#1166" id="1166">1166</a>
<a href="#1167" id="1167">1167</a>
<a href="#1168" id="1168">1168</a>
<a href="#1169" id="1169">1169</a>
<a href="#1170" id="1170">1170</a>
<a href="#1171" id="1171">1171</a>
<a href="#1172" id="1172">1172</a>
<a href="#1173" id="1173">1173</a>
<a href="#1174" id="1174">1174</a>
<a href="#1175" id="1175">1175</a>
<a href="#1176" id="1176">1176</a>
<a href="#1177" id="1177">1177</a>
<a href="#1178" id="1178">1178</a>
<a href="#1179" id="1179">1179</a>
<a href="#1180" id="1180">1180</a>
<a href="#1181" id="1181">1181</a>
<a href="#1182" id="1182">1182</a>
<a href="#1183" id="1183">1183</a>
<a href="#1184" id="1184">1184</a>
<a href="#1185" id="1185">1185</a>
<a href="#1186" id="1186">1186</a>
<a href="#1187" id="1187">1187</a>
<a href="#1188" id="1188">1188</a>
<a href="#1189" id="1189">1189</a>
<a href="#1190" id="1190">1190</a>
<a href="#1191" id="1191">1191</a>
<a href="#1192" id="1192">1192</a>
<a href="#1193" id="1193">1193</a>
<a href="#1194" id="1194">1194</a>
<a href="#1195" id="1195">1195</a>
<a href="#1196" id="1196">1196</a>
<a href="#1197" id="1197">1197</a>
<a href="#1198" id="1198">1198</a>
<a href="#1199" id="1199">1199</a>
<a href="#1200" id="1200">1200</a>
<a href="#1201" id="1201">1201</a>
<a href="#1202" id="1202">1202</a>
<a href="#1203" id="1203">1203</a>
<a href="#1204" id="1204">1204</a>
<a href="#1205" id="1205">1205</a>
<a href="#1206" id="1206">1206</a>
<a href="#1207" id="1207">1207</a>
<a href="#1208" id="1208">1208</a>
<a href="#1209" id="1209">1209</a>
<a href="#1210" id="1210">1210</a>
<a href="#1211" id="1211">1211</a>
<a href="#1212" id="1212">1212</a>
<a href="#1213" id="1213">1213</a>
<a href="#1214" id="1214">1214</a>
<a href="#1215" id="1215">1215</a>
<a href="#1216" id="1216">1216</a>
<a href="#1217" id="1217">1217</a>
<a href="#1218" id="1218">1218</a>
<a href="#1219" id="1219">1219</a>
<a href="#1220" id="1220">1220</a>
<a href="#1221" id="1221">1221</a>
<a href="#1222" id="1222">1222</a>
<a href="#1223" id="1223">1223</a>
<a href="#1224" id="1224">1224</a>
<a href="#1225" id="1225">1225</a>
<a href="#1226" id="1226">1226</a>
<a href="#1227" id="1227">1227</a>
<a href="#1228" id="1228">1228</a>
<a href="#1229" id="1229">1229</a>
<a href="#1230" id="1230">1230</a>
<a href="#1231" id="1231">1231</a>
<a href="#1232" id="1232">1232</a>
<a href="#1233" id="1233">1233</a>
<a href="#1234" id="1234">1234</a>
<a href="#1235" id="1235">1235</a>
<a href="#1236" id="1236">1236</a>
<a href="#1237" id="1237">1237</a>
<a href="#1238" id="1238">1238</a>
<a href="#1239" id="1239">1239</a>
<a href="#1240" id="1240">1240</a>
<a href="#1241" id="1241">1241</a>
<a href="#1242" id="1242">1242</a>
<a href="#1243" id="1243">1243</a>
<a href="#1244" id="1244">1244</a>
<a href="#1245" id="1245">1245</a>
<a href="#1246" id="1246">1246</a>
<a href="#1247" id="1247">1247</a>
<a href="#1248" id="1248">1248</a>
<a href="#1249" id="1249">1249</a>
<a href="#1250" id="1250">1250</a>
<a href="#1251" id="1251">1251</a>
<a href="#1252" id="1252">1252</a>
<a href="#1253" id="1253">1253</a>
<a href="#1254" id="1254">1254</a>
<a href="#1255" id="1255">1255</a>
<a href="#1256" id="1256">1256</a>
<a href="#1257" id="1257">1257</a>
<a href="#1258" id="1258">1258</a>
<a href="#1259" id="1259">1259</a>
<a href="#1260" id="1260">1260</a>
<a href="#1261" id="1261">1261</a>
<a href="#1262" id="1262">1262</a>
<a href="#1263" id="1263">1263</a>
<a href="#1264" id="1264">1264</a>
<a href="#1265" id="1265">1265</a>
<a href="#1266" id="1266">1266</a>
<a href="#1267" id="1267">1267</a>
<a href="#1268" id="1268">1268</a>
<a href="#1269" id="1269">1269</a>
<a href="#1270" id="1270">1270</a>
<a href="#1271" id="1271">1271</a>
<a href="#1272" id="1272">1272</a>
<a href="#1273" id="1273">1273</a>
<a href="#1274" id="1274">1274</a>
<a href="#1275" id="1275">1275</a>
<a href="#1276" id="1276">1276</a>
<a href="#1277" id="1277">1277</a>
<a href="#1278" id="1278">1278</a>
<a href="#1279" id="1279">1279</a>
<a href="#1280" id="1280">1280</a>
<a href="#1281" id="1281">1281</a>
<a href="#1282" id="1282">1282</a>
<a href="#1283" id="1283">1283</a>
<a href="#1284" id="1284">1284</a>
<a href="#1285" id="1285">1285</a>
<a href="#1286" id="1286">1286</a>
<a href="#1287" id="1287">1287</a>
<a href="#1288" id="1288">1288</a>
<a href="#1289" id="1289">1289</a>
<a href="#1290" id="1290">1290</a>
<a href="#1291" id="1291">1291</a>
<a href="#1292" id="1292">1292</a>
<a href="#1293" id="1293">1293</a>
<a href="#1294" id="1294">1294</a>
<a href="#1295" id="1295">1295</a>
<a href="#1296" id="1296">1296</a>
<a href="#1297" id="1297">1297</a>
<a href="#1298" id="1298">1298</a>
<a href="#1299" id="1299">1299</a>
<a href="#1300" id="1300">1300</a>
<a href="#1301" id="1301">1301</a>
<a href="#1302" id="1302">1302</a>
<a href="#1303" id="1303">1303</a>
<a href="#1304" id="1304">1304</a>
<a href="#1305" id="1305">1305</a>
<a href="#1306" id="1306">1306</a>
<a href="#1307" id="1307">1307</a>
<a href="#1308" id="1308">1308</a>
<a href="#1309" id="1309">1309</a>
<a href="#1310" id="1310">1310</a>
<a href="#1311" id="1311">1311</a>
<a href="#1312" id="1312">1312</a>
<a href="#1313" id="1313">1313</a>
<a href="#1314" id="1314">1314</a>
<a href="#1315" id="1315">1315</a>
<a href="#1316" id="1316">1316</a>
<a href="#1317" id="1317">1317</a>
<a href="#1318" id="1318">1318</a>
<a href="#1319" id="1319">1319</a>
<a href="#1320" id="1320">1320</a>
<a href="#1321" id="1321">1321</a>
<a href="#1322" id="1322">1322</a>
<a href="#1323" id="1323">1323</a>
<a href="#1324" id="1324">1324</a>
<a href="#1325" id="1325">1325</a>
<a href="#1326" id="1326">1326</a>
<a href="#1327" id="1327">1327</a>
<a href="#1328" id="1328">1328</a>
<a href="#1329" id="1329">1329</a>
<a href="#1330" id="1330">1330</a>
<a href="#1331" id="1331">1331</a>
<a href="#1332" id="1332">1332</a>
<a href="#1333" id="1333">1333</a>
<a href="#1334" id="1334">1334</a>
<a href="#1335" id="1335">1335</a>
<a href="#1336" id="1336">1336</a>
<a href="#1337" id="1337">1337</a>
<a href="#1338" id="1338">1338</a>
<a href="#1339" id="1339">1339</a>
<a href="#1340" id="1340">1340</a>
<a href="#1341" id="1341">1341</a>
<a href="#1342" id="1342">1342</a>
<a href="#1343" id="1343">1343</a>
<a href="#1344" id="1344">1344</a>
<a href="#1345" id="1345">1345</a>
<a href="#1346" id="1346">1346</a>
<a href="#1347" id="1347">1347</a>
<a href="#1348" id="1348">1348</a>
<a href="#1349" id="1349">1349</a>
<a href="#1350" id="1350">1350</a>
<a href="#1351" id="1351">1351</a>
<a href="#1352" id="1352">1352</a>
<a href="#1353" id="1353">1353</a>
<a href="#1354" id="1354">1354</a>
<a href="#1355" id="1355">1355</a>
<a href="#1356" id="1356">1356</a>
<a href="#1357" id="1357">1357</a>
<a href="#1358" id="1358">1358</a>
<a href="#1359" id="1359">1359</a>
<a href="#1360" id="1360">1360</a>
<a href="#1361" id="1361">1361</a>
<a href="#1362" id="1362">1362</a>
<a href="#1363" id="1363">1363</a>
<a href="#1364" id="1364">1364</a>
<a href="#1365" id="1365">1365</a>
<a href="#1366" id="1366">1366</a>
<a href="#1367" id="1367">1367</a>
<a href="#1368" id="1368">1368</a>
<a href="#1369" id="1369">1369</a>
<a href="#1370" id="1370">1370</a>
<a href="#1371" id="1371">1371</a>
<a href="#1372" id="1372">1372</a>
<a href="#1373" id="1373">1373</a>
<a href="#1374" id="1374">1374</a>
<a href="#1375" id="1375">1375</a>
<a href="#1376" id="1376">1376</a>
<a href="#1377" id="1377">1377</a>
<a href="#1378" id="1378">1378</a>
<a href="#1379" id="1379">1379</a>
<a href="#1380" id="1380">1380</a>
<a href="#1381" id="1381">1381</a>
<a href="#1382" id="1382">1382</a>
<a href="#1383" id="1383">1383</a>
<a href="#1384" id="1384">1384</a>
<a href="#1385" id="1385">1385</a>
<a href="#1386" id="1386">1386</a>
<a href="#1387" id="1387">1387</a>
<a href="#1388" id="1388">1388</a>
<a href="#1389" id="1389">1389</a>
<a href="#1390" id="1390">1390</a>
<a href="#1391" id="1391">1391</a>
<a href="#1392" id="1392">1392</a>
<a href="#1393" id="1393">1393</a>
<a href="#1394" id="1394">1394</a>
<a href="#1395" id="1395">1395</a>
<a href="#1396" id="1396">1396</a>
<a href="#1397" id="1397">1397</a>
<a href="#1398" id="1398">1398</a>
<a href="#1399" id="1399">1399</a>
<a href="#1400" id="1400">1400</a>
<a href="#1401" id="1401">1401</a>
<a href="#1402" id="1402">1402</a>
<a href="#1403" id="1403">1403</a>
<a href="#1404" id="1404">1404</a>
<a href="#1405" id="1405">1405</a>
<a href="#1406" id="1406">1406</a>
<a href="#1407" id="1407">1407</a>
<a href="#1408" id="1408">1408</a>
<a href="#1409" id="1409">1409</a>
<a href="#1410" id="1410">1410</a>
<a href="#1411" id="1411">1411</a>
<a href="#1412" id="1412">1412</a>
<a href="#1413" id="1413">1413</a>
<a href="#1414" id="1414">1414</a>
<a href="#1415" id="1415">1415</a>
<a href="#1416" id="1416">1416</a>
<a href="#1417" id="1417">1417</a>
<a href="#1418" id="1418">1418</a>
<a href="#1419" id="1419">1419</a>
<a href="#1420" id="1420">1420</a>
<a href="#1421" id="1421">1421</a>
<a href="#1422" id="1422">1422</a>
<a href="#1423" id="1423">1423</a>
<a href="#1424" id="1424">1424</a>
<a href="#1425" id="1425">1425</a>
<a href="#1426" id="1426">1426</a>
<a href="#1427" id="1427">1427</a>
<a href="#1428" id="1428">1428</a>
<a href="#1429" id="1429">1429</a>
<a href="#1430" id="1430">1430</a>
<a href="#1431" id="1431">1431</a>
<a href="#1432" id="1432">1432</a>
<a href="#1433" id="1433">1433</a>
<a href="#1434" id="1434">1434</a>
<a href="#1435" id="1435">1435</a>
<a href="#1436" id="1436">1436</a>
<a href="#1437" id="1437">1437</a>
<a href="#1438" id="1438">1438</a>
<a href="#1439" id="1439">1439</a>
<a href="#1440" id="1440">1440</a>
<a href="#1441" id="1441">1441</a>
<a href="#1442" id="1442">1442</a>
<a href="#1443" id="1443">1443</a>
<a href="#1444" id="1444">1444</a>
<a href="#1445" id="1445">1445</a>
<a href="#1446" id="1446">1446</a>
<a href="#1447" id="1447">1447</a>
<a href="#1448" id="1448">1448</a>
<a href="#1449" id="1449">1449</a>
<a href="#1450" id="1450">1450</a>
<a href="#1451" id="1451">1451</a>
<a href="#1452" id="1452">1452</a>
<a href="#1453" id="1453">1453</a>
<a href="#1454" id="1454">1454</a>
<a href="#1455" id="1455">1455</a>
<a href="#1456" id="1456">1456</a>
<a href="#1457" id="1457">1457</a>
<a href="#1458" id="1458">1458</a>
<a href="#1459" id="1459">1459</a>
<a href="#1460" id="1460">1460</a>
<a href="#1461" id="1461">1461</a>
<a href="#1462" id="1462">1462</a>
<a href="#1463" id="1463">1463</a>
<a href="#1464" id="1464">1464</a>
<a href="#1465" id="1465">1465</a>
<a href="#1466" id="1466">1466</a>
<a href="#1467" id="1467">1467</a>
<a href="#1468" id="1468">1468</a>
<a href="#1469" id="1469">1469</a>
<a href="#1470" id="1470">1470</a>
<a href="#1471" id="1471">1471</a>
<a href="#1472" id="1472">1472</a>
<a href="#1473" id="1473">1473</a>
<a href="#1474" id="1474">1474</a>
<a href="#1475" id="1475">1475</a>
<a href="#1476" id="1476">1476</a>
<a href="#1477" id="1477">1477</a>
<a href="#1478" id="1478">1478</a>
<a href="#1479" id="1479">1479</a>
<a href="#1480" id="1480">1480</a>
<a href="#1481" id="1481">1481</a>
<a href="#1482" id="1482">1482</a>
<a href="#1483" id="1483">1483</a>
<a href="#1484" id="1484">1484</a>
<a href="#1485" id="1485">1485</a>
<a href="#1486" id="1486">1486</a>
<a href="#1487" id="1487">1487</a>
<a href="#1488" id="1488">1488</a>
<a href="#1489" id="1489">1489</a>
<a href="#1490" id="1490">1490</a>
<a href="#1491" id="1491">1491</a>
<a href="#1492" id="1492">1492</a>
<a href="#1493" id="1493">1493</a>
<a href="#1494" id="1494">1494</a>
<a href="#1495" id="1495">1495</a>
<a href="#1496" id="1496">1496</a>
<a href="#1497" id="1497">1497</a>
<a href="#1498" id="1498">1498</a>
<a href="#1499" id="1499">1499</a>
<a href="#1500" id="1500">1500</a>
<a href="#1501" id="1501">1501</a>
<a href="#1502" id="1502">1502</a>
<a href="#1503" id="1503">1503</a>
<a href="#1504" id="1504">1504</a>
<a href="#1505" id="1505">1505</a>
<a href="#1506" id="1506">1506</a>
<a href="#1507" id="1507">1507</a>
<a href="#1508" id="1508">1508</a>
<a href="#1509" id="1509">1509</a>
<a href="#1510" id="1510">1510</a>
<a href="#1511" id="1511">1511</a>
<a href="#1512" id="1512">1512</a>
<a href="#1513" id="1513">1513</a>
<a href="#1514" id="1514">1514</a>
<a href="#1515" id="1515">1515</a>
<a href="#1516" id="1516">1516</a>
<a href="#1517" id="1517">1517</a>
<a href="#1518" id="1518">1518</a>
<a href="#1519" id="1519">1519</a>
<a href="#1520" id="1520">1520</a>
<a href="#1521" id="1521">1521</a>
<a href="#1522" id="1522">1522</a>
<a href="#1523" id="1523">1523</a>
<a href="#1524" id="1524">1524</a>
<a href="#1525" id="1525">1525</a>
<a href="#1526" id="1526">1526</a>
<a href="#1527" id="1527">1527</a>
<a href="#1528" id="1528">1528</a>
<a href="#1529" id="1529">1529</a>
<a href="#1530" id="1530">1530</a>
<a href="#1531" id="1531">1531</a>
<a href="#1532" id="1532">1532</a>
<a href="#1533" id="1533">1533</a>
<a href="#1534" id="1534">1534</a>
<a href="#1535" id="1535">1535</a>
<a href="#1536" id="1536">1536</a>
<a href="#1537" id="1537">1537</a>
<a href="#1538" id="1538">1538</a>
<a href="#1539" id="1539">1539</a>
<a href="#1540" id="1540">1540</a>
<a href="#1541" id="1541">1541</a>
<a href="#1542" id="1542">1542</a>
<a href="#1543" id="1543">1543</a>
<a href="#1544" id="1544">1544</a>
<a href="#1545" id="1545">1545</a>
<a href="#1546" id="1546">1546</a>
<a href="#1547" id="1547">1547</a>
<a href="#1548" id="1548">1548</a>
<a href="#1549" id="1549">1549</a>
<a href="#1550" id="1550">1550</a>
<a href="#1551" id="1551">1551</a>
<a href="#1552" id="1552">1552</a>
<a href="#1553" id="1553">1553</a>
<a href="#1554" id="1554">1554</a>
<a href="#1555" id="1555">1555</a>
<a href="#1556" id="1556">1556</a>
<a href="#1557" id="1557">1557</a>
<a href="#1558" id="1558">1558</a>
<a href="#1559" id="1559">1559</a>
<a href="#1560" id="1560">1560</a>
<a href="#1561" id="1561">1561</a>
<a href="#1562" id="1562">1562</a>
<a href="#1563" id="1563">1563</a>
<a href="#1564" id="1564">1564</a>
<a href="#1565" id="1565">1565</a>
<a href="#1566" id="1566">1566</a>
<a href="#1567" id="1567">1567</a>
<a href="#1568" id="1568">1568</a>
<a href="#1569" id="1569">1569</a>
<a href="#1570" id="1570">1570</a>
<a href="#1571" id="1571">1571</a>
<a href="#1572" id="1572">1572</a>
<a href="#1573" id="1573">1573</a>
<a href="#1574" id="1574">1574</a>
<a href="#1575" id="1575">1575</a>
<a href="#1576" id="1576">1576</a>
<a href="#1577" id="1577">1577</a>
<a href="#1578" id="1578">1578</a>
<a href="#1579" id="1579">1579</a>
<a href="#1580" id="1580">1580</a>
<a href="#1581" id="1581">1581</a>
<a href="#1582" id="1582">1582</a>
<a href="#1583" id="1583">1583</a>
<a href="#1584" id="1584">1584</a>
<a href="#1585" id="1585">1585</a>
<a href="#1586" id="1586">1586</a>
<a href="#1587" id="1587">1587</a>
<a href="#1588" id="1588">1588</a>
<a href="#1589" id="1589">1589</a>
<a href="#1590" id="1590">1590</a>
<a href="#1591" id="1591">1591</a>
<a href="#1592" id="1592">1592</a>
<a href="#1593" id="1593">1593</a>
<a href="#1594" id="1594">1594</a>
<a href="#1595" id="1595">1595</a>
<a href="#1596" id="1596">1596</a>
<a href="#1597" id="1597">1597</a>
<a href="#1598" id="1598">1598</a>
<a href="#1599" id="1599">1599</a>
<a href="#1600" id="1600">1600</a>
<a href="#1601" id="1601">1601</a>
<a href="#1602" id="1602">1602</a>
<a href="#1603" id="1603">1603</a>
<a href="#1604" id="1604">1604</a>
<a href="#1605" id="1605">1605</a>
<a href="#1606" id="1606">1606</a>
<a href="#1607" id="1607">1607</a>
<a href="#1608" id="1608">1608</a>
<a href="#1609" id="1609">1609</a>
<a href="#1610" id="1610">1610</a>
<a href="#1611" id="1611">1611</a>
<a href="#1612" id="1612">1612</a>
<a href="#1613" id="1613">1613</a>
<a href="#1614" id="1614">1614</a>
<a href="#1615" id="1615">1615</a>
<a href="#1616" id="1616">1616</a>
<a href="#1617" id="1617">1617</a>
<a href="#1618" id="1618">1618</a>
<a href="#1619" id="1619">1619</a>
<a href="#1620" id="1620">1620</a>
<a href="#1621" id="1621">1621</a>
<a href="#1622" id="1622">1622</a>
<a href="#1623" id="1623">1623</a>
<a href="#1624" id="1624">1624</a>
<a href="#1625" id="1625">1625</a>
<a href="#1626" id="1626">1626</a>
<a href="#1627" id="1627">1627</a>
<a href="#1628" id="1628">1628</a>
<a href="#1629" id="1629">1629</a>
<a href="#1630" id="1630">1630</a>
<a href="#1631" id="1631">1631</a>
<a href="#1632" id="1632">1632</a>
<a href="#1633" id="1633">1633</a>
<a href="#1634" id="1634">1634</a>
<a href="#1635" id="1635">1635</a>
<a href="#1636" id="1636">1636</a>
<a href="#1637" id="1637">1637</a>
<a href="#1638" id="1638">1638</a>
<a href="#1639" id="1639">1639</a>
<a href="#1640" id="1640">1640</a>
<a href="#1641" id="1641">1641</a>
<a href="#1642" id="1642">1642</a>
<a href="#1643" id="1643">1643</a>
<a href="#1644" id="1644">1644</a>
<a href="#1645" id="1645">1645</a>
<a href="#1646" id="1646">1646</a>
<a href="#1647" id="1647">1647</a>
<a href="#1648" id="1648">1648</a>
<a href="#1649" id="1649">1649</a>
<a href="#1650" id="1650">1650</a>
<a href="#1651" id="1651">1651</a>
<a href="#1652" id="1652">1652</a>
<a href="#1653" id="1653">1653</a>
<a href="#1654" id="1654">1654</a>
<a href="#1655" id="1655">1655</a>
<a href="#1656" id="1656">1656</a>
<a href="#1657" id="1657">1657</a>
<a href="#1658" id="1658">1658</a>
<a href="#1659" id="1659">1659</a>
<a href="#1660" id="1660">1660</a>
<a href="#1661" id="1661">1661</a>
<a href="#1662" id="1662">1662</a>
<a href="#1663" id="1663">1663</a>
<a href="#1664" id="1664">1664</a>
<a href="#1665" id="1665">1665</a>
<a href="#1666" id="1666">1666</a>
<a href="#1667" id="1667">1667</a>
<a href="#1668" id="1668">1668</a>
<a href="#1669" id="1669">1669</a>
<a href="#1670" id="1670">1670</a>
<a href="#1671" id="1671">1671</a>
<a href="#1672" id="1672">1672</a>
<a href="#1673" id="1673">1673</a>
<a href="#1674" id="1674">1674</a>
<a href="#1675" id="1675">1675</a>
<a href="#1676" id="1676">1676</a>
<a href="#1677" id="1677">1677</a>
<a href="#1678" id="1678">1678</a>
<a href="#1679" id="1679">1679</a>
<a href="#1680" id="1680">1680</a>
<a href="#1681" id="1681">1681</a>
<a href="#1682" id="1682">1682</a>
<a href="#1683" id="1683">1683</a>
<a href="#1684" id="1684">1684</a>
<a href="#1685" id="1685">1685</a>
<a href="#1686" id="1686">1686</a>
<a href="#1687" id="1687">1687</a>
<a href="#1688" id="1688">1688</a>
<a href="#1689" id="1689">1689</a>
<a href="#1690" id="1690">1690</a>
<a href="#1691" id="1691">1691</a>
<a href="#1692" id="1692">1692</a>
<a href="#1693" id="1693">1693</a>
<a href="#1694" id="1694">1694</a>
<a href="#1695" id="1695">1695</a>
<a href="#1696" id="1696">1696</a>
<a href="#1697" id="1697">1697</a>
<a href="#1698" id="1698">1698</a>
<a href="#1699" id="1699">1699</a>
<a href="#1700" id="1700">1700</a>
<a href="#1701" id="1701">1701</a>
<a href="#1702" id="1702">1702</a>
<a href="#1703" id="1703">1703</a>
<a href="#1704" id="1704">1704</a>
<a href="#1705" id="1705">1705</a>
<a href="#1706" id="1706">1706</a>
<a href="#1707" id="1707">1707</a>
<a href="#1708" id="1708">1708</a>
<a href="#1709" id="1709">1709</a>
<a href="#1710" id="1710">1710</a>
<a href="#1711" id="1711">1711</a>
<a href="#1712" id="1712">1712</a>
<a href="#1713" id="1713">1713</a>
<a href="#1714" id="1714">1714</a>
<a href="#1715" id="1715">1715</a>
<a href="#1716" id="1716">1716</a>
<a href="#1717" id="1717">1717</a>
<a href="#1718" id="1718">1718</a>
<a href="#1719" id="1719">1719</a>
<a href="#1720" id="1720">1720</a>
<a href="#1721" id="1721">1721</a>
<a href="#1722" id="1722">1722</a>
<a href="#1723" id="1723">1723</a>
<a href="#1724" id="1724">1724</a>
<a href="#1725" id="1725">1725</a>
<a href="#1726" id="1726">1726</a>
<a href="#1727" id="1727">1727</a>
<a href="#1728" id="1728">1728</a>
<a href="#1729" id="1729">1729</a>
<a href="#1730" id="1730">1730</a>
<a href="#1731" id="1731">1731</a>
<a href="#1732" id="1732">1732</a>
<a href="#1733" id="1733">1733</a>
<a href="#1734" id="1734">1734</a>
<a href="#1735" id="1735">1735</a>
<a href="#1736" id="1736">1736</a>
<a href="#1737" id="1737">1737</a>
<a href="#1738" id="1738">1738</a>
<a href="#1739" id="1739">1739</a>
<a href="#1740" id="1740">1740</a>
<a href="#1741" id="1741">1741</a>
<a href="#1742" id="1742">1742</a>
<a href="#1743" id="1743">1743</a>
<a href="#1744" id="1744">1744</a>
<a href="#1745" id="1745">1745</a>
<a href="#1746" id="1746">1746</a>
<a href="#1747" id="1747">1747</a>
<a href="#1748" id="1748">1748</a>
<a href="#1749" id="1749">1749</a>
<a href="#1750" id="1750">1750</a>
<a href="#1751" id="1751">1751</a>
<a href="#1752" id="1752">1752</a>
<a href="#1753" id="1753">1753</a>
<a href="#1754" id="1754">1754</a>
<a href="#1755" id="1755">1755</a>
<a href="#1756" id="1756">1756</a>
<a href="#1757" id="1757">1757</a>
<a href="#1758" id="1758">1758</a>
<a href="#1759" id="1759">1759</a>
<a href="#1760" id="1760">1760</a>
<a href="#1761" id="1761">1761</a>
<a href="#1762" id="1762">1762</a>
<a href="#1763" id="1763">1763</a>
<a href="#1764" id="1764">1764</a>
<a href="#1765" id="1765">1765</a>
<a href="#1766" id="1766">1766</a>
<a href="#1767" id="1767">1767</a>
<a href="#1768" id="1768">1768</a>
<a href="#1769" id="1769">1769</a>
<a href="#1770" id="1770">1770</a>
<a href="#1771" id="1771">1771</a>
<a href="#1772" id="1772">1772</a>
<a href="#1773" id="1773">1773</a>
<a href="#1774" id="1774">1774</a>
<a href="#1775" id="1775">1775</a>
<a href="#1776" id="1776">1776</a>
<a href="#1777" id="1777">1777</a>
<a href="#1778" id="1778">1778</a>
<a href="#1779" id="1779">1779</a>
<a href="#1780" id="1780">1780</a>
<a href="#1781" id="1781">1781</a>
<a href="#1782" id="1782">1782</a>
<a href="#1783" id="1783">1783</a>
<a href="#1784" id="1784">1784</a>
<a href="#1785" id="1785">1785</a>
<a href="#1786" id="1786">1786</a>
<a href="#1787" id="1787">1787</a>
<a href="#1788" id="1788">1788</a>
<a href="#1789" id="1789">1789</a>
<a href="#1790" id="1790">1790</a>
<a href="#1791" id="1791">1791</a>
<a href="#1792" id="1792">1792</a>
<a href="#1793" id="1793">1793</a>
<a href="#1794" id="1794">1794</a>
<a href="#1795" id="1795">1795</a>
<a href="#1796" id="1796">1796</a>
<a href="#1797" id="1797">1797</a>
<a href="#1798" id="1798">1798</a>
<a href="#1799" id="1799">1799</a>
<a href="#1800" id="1800">1800</a>
<a href="#1801" id="1801">1801</a>
<a href="#1802" id="1802">1802</a>
<a href="#1803" id="1803">1803</a>
<a href="#1804" id="1804">1804</a>
<a href="#1805" id="1805">1805</a>
<a href="#1806" id="1806">1806</a>
<a href="#1807" id="1807">1807</a>
<a href="#1808" id="1808">1808</a>
<a href="#1809" id="1809">1809</a>
<a href="#1810" id="1810">1810</a>
<a href="#1811" id="1811">1811</a>
<a href="#1812" id="1812">1812</a>
<a href="#1813" id="1813">1813</a>
<a href="#1814" id="1814">1814</a>
<a href="#1815" id="1815">1815</a>
<a href="#1816" id="1816">1816</a>
<a href="#1817" id="1817">1817</a>
<a href="#1818" id="1818">1818</a>
<a href="#1819" id="1819">1819</a>
<a href="#1820" id="1820">1820</a>
<a href="#1821" id="1821">1821</a>
<a href="#1822" id="1822">1822</a>
<a href="#1823" id="1823">1823</a>
<a href="#1824" id="1824">1824</a>
<a href="#1825" id="1825">1825</a>
<a href="#1826" id="1826">1826</a>
<a href="#1827" id="1827">1827</a>
<a href="#1828" id="1828">1828</a>
<a href="#1829" id="1829">1829</a>
<a href="#1830" id="1830">1830</a>
<a href="#1831" id="1831">1831</a>
<a href="#1832" id="1832">1832</a>
<a href="#1833" id="1833">1833</a>
<a href="#1834" id="1834">1834</a>
<a href="#1835" id="1835">1835</a>
<a href="#1836" id="1836">1836</a>
<a href="#1837" id="1837">1837</a>
<a href="#1838" id="1838">1838</a>
<a href="#1839" id="1839">1839</a>
<a href="#1840" id="1840">1840</a>
<a href="#1841" id="1841">1841</a>
<a href="#1842" id="1842">1842</a>
<a href="#1843" id="1843">1843</a>
<a href="#1844" id="1844">1844</a>
<a href="#1845" id="1845">1845</a>
<a href="#1846" id="1846">1846</a>
<a href="#1847" id="1847">1847</a>
<a href="#1848" id="1848">1848</a>
<a href="#1849" id="1849">1849</a>
<a href="#1850" id="1850">1850</a>
<a href="#1851" id="1851">1851</a>
<a href="#1852" id="1852">1852</a>
<a href="#1853" id="1853">1853</a>
<a href="#1854" id="1854">1854</a>
<a href="#1855" id="1855">1855</a>
<a href="#1856" id="1856">1856</a>
<a href="#1857" id="1857">1857</a>
<a href="#1858" id="1858">1858</a>
<a href="#1859" id="1859">1859</a>
<a href="#1860" id="1860">1860</a>
<a href="#1861" id="1861">1861</a>
<a href="#1862" id="1862">1862</a>
<a href="#1863" id="1863">1863</a>
<a href="#1864" id="1864">1864</a>
<a href="#1865" id="1865">1865</a>
<a href="#1866" id="1866">1866</a>
<a href="#1867" id="1867">1867</a>
<a href="#1868" id="1868">1868</a>
<a href="#1869" id="1869">1869</a>
<a href="#1870" id="1870">1870</a>
<a href="#1871" id="1871">1871</a>
<a href="#1872" id="1872">1872</a>
<a href="#1873" id="1873">1873</a>
<a href="#1874" id="1874">1874</a>
<a href="#1875" id="1875">1875</a>
<a href="#1876" id="1876">1876</a>
<a href="#1877" id="1877">1877</a>
<a href="#1878" id="1878">1878</a>
<a href="#1879" id="1879">1879</a>
<a href="#1880" id="1880">1880</a>
<a href="#1881" id="1881">1881</a>
<a href="#1882" id="1882">1882</a>
<a href="#1883" id="1883">1883</a>
<a href="#1884" id="1884">1884</a>
<a href="#1885" id="1885">1885</a>
<a href="#1886" id="1886">1886</a>
<a href="#1887" id="1887">1887</a>
<a href="#1888" id="1888">1888</a>
<a href="#1889" id="1889">1889</a>
<a href="#1890" id="1890">1890</a>
<a href="#1891" id="1891">1891</a>
<a href="#1892" id="1892">1892</a>
<a href="#1893" id="1893">1893</a>
<a href="#1894" id="1894">1894</a>
<a href="#1895" id="1895">1895</a>
<a href="#1896" id="1896">1896</a>
<a href="#1897" id="1897">1897</a>
<a href="#1898" id="1898">1898</a>
<a href="#1899" id="1899">1899</a>
<a href="#1900" id="1900">1900</a>
<a href="#1901" id="1901">1901</a>
<a href="#1902" id="1902">1902</a>
<a href="#1903" id="1903">1903</a>
<a href="#1904" id="1904">1904</a>
<a href="#1905" id="1905">1905</a>
<a href="#1906" id="1906">1906</a>
<a href="#1907" id="1907">1907</a>
<a href="#1908" id="1908">1908</a>
<a href="#1909" id="1909">1909</a>
<a href="#1910" id="1910">1910</a>
<a href="#1911" id="1911">1911</a>
<a href="#1912" id="1912">1912</a>
<a href="#1913" id="1913">1913</a>
<a href="#1914" id="1914">1914</a>
<a href="#1915" id="1915">1915</a>
<a href="#1916" id="1916">1916</a>
<a href="#1917" id="1917">1917</a>
<a href="#1918" id="1918">1918</a>
<a href="#1919" id="1919">1919</a>
<a href="#1920" id="1920">1920</a>
<a href="#1921" id="1921">1921</a>
<a href="#1922" id="1922">1922</a>
<a href="#1923" id="1923">1923</a>
<a href="#1924" id="1924">1924</a>
<a href="#1925" id="1925">1925</a>
<a href="#1926" id="1926">1926</a>
<a href="#1927" id="1927">1927</a>
<a href="#1928" id="1928">1928</a>
<a href="#1929" id="1929">1929</a>
<a href="#1930" id="1930">1930</a>
<a href="#1931" id="1931">1931</a>
<a href="#1932" id="1932">1932</a>
<a href="#1933" id="1933">1933</a>
<a href="#1934" id="1934">1934</a>
<a href="#1935" id="1935">1935</a>
<a href="#1936" id="1936">1936</a>
<a href="#1937" id="1937">1937</a>
<a href="#1938" id="1938">1938</a>
<a href="#1939" id="1939">1939</a>
<a href="#1940" id="1940">1940</a>
<a href="#1941" id="1941">1941</a>
<a href="#1942" id="1942">1942</a>
<a href="#1943" id="1943">1943</a>
<a href="#1944" id="1944">1944</a>
<a href="#1945" id="1945">1945</a>
<a href="#1946" id="1946">1946</a>
<a href="#1947" id="1947">1947</a>
<a href="#1948" id="1948">1948</a>
<a href="#1949" id="1949">1949</a>
<a href="#1950" id="1950">1950</a>
<a href="#1951" id="1951">1951</a>
<a href="#1952" id="1952">1952</a>
<a href="#1953" id="1953">1953</a>
<a href="#1954" id="1954">1954</a>
<a href="#1955" id="1955">1955</a>
<a href="#1956" id="1956">1956</a>
<a href="#1957" id="1957">1957</a>
<a href="#1958" id="1958">1958</a>
<a href="#1959" id="1959">1959</a>
<a href="#1960" id="1960">1960</a>
<a href="#1961" id="1961">1961</a>
<a href="#1962" id="1962">1962</a>
<a href="#1963" id="1963">1963</a>
<a href="#1964" id="1964">1964</a>
<a href="#1965" id="1965">1965</a>
<a href="#1966" id="1966">1966</a>
<a href="#1967" id="1967">1967</a>
<a href="#1968" id="1968">1968</a>
<a href="#1969" id="1969">1969</a>
<a href="#1970" id="1970">1970</a>
<a href="#1971" id="1971">1971</a>
<a href="#1972" id="1972">1972</a>
<a href="#1973" id="1973">1973</a>
<a href="#1974" id="1974">1974</a>
<a href="#1975" id="1975">1975</a>
<a href="#1976" id="1976">1976</a>
<a href="#1977" id="1977">1977</a>
<a href="#1978" id="1978">1978</a>
<a href="#1979" id="1979">1979</a>
<a href="#1980" id="1980">1980</a>
<a href="#1981" id="1981">1981</a>
<a href="#1982" id="1982">1982</a>
<a href="#1983" id="1983">1983</a>
<a href="#1984" id="1984">1984</a>
<a href="#1985" id="1985">1985</a>
<a href="#1986" id="1986">1986</a>
<a href="#1987" id="1987">1987</a>
<a href="#1988" id="1988">1988</a>
<a href="#1989" id="1989">1989</a>
<a href="#1990" id="1990">1990</a>
<a href="#1991" id="1991">1991</a>
<a href="#1992" id="1992">1992</a>
<a href="#1993" id="1993">1993</a>
<a href="#1994" id="1994">1994</a>
<a href="#1995" id="1995">1995</a>
<a href="#1996" id="1996">1996</a>
<a href="#1997" id="1997">1997</a>
<a href="#1998" id="1998">1998</a>
<a href="#1999" id="1999">1999</a>
<a href="#2000" id="2000">2000</a>
<a href="#2001" id="2001">2001</a>
<a href="#2002" id="2002">2002</a>
<a href="#2003" id="2003">2003</a>
<a href="#2004" id="2004">2004</a>
<a href="#2005" id="2005">2005</a>
<a href="#2006" id="2006">2006</a>
<a href="#2007" id="2007">2007</a>
<a href="#2008" id="2008">2008</a>
<a href="#2009" id="2009">2009</a>
<a href="#2010" id="2010">2010</a>
<a href="#2011" id="2011">2011</a>
<a href="#2012" id="2012">2012</a>
<a href="#2013" id="2013">2013</a>
<a href="#2014" id="2014">2014</a>
<a href="#2015" id="2015">2015</a>
<a href="#2016" id="2016">2016</a>
<a href="#2017" id="2017">2017</a>
<a href="#2018" id="2018">2018</a>
<a href="#2019" id="2019">2019</a>
<a href="#2020" id="2020">2020</a>
<a href="#2021" id="2021">2021</a>
<a href="#2022" id="2022">2022</a>
<a href="#2023" id="2023">2023</a>
<a href="#2024" id="2024">2024</a>
<a href="#2025" id="2025">2025</a>
<a href="#2026" id="2026">2026</a>
<a href="#2027" id="2027">2027</a>
<a href="#2028" id="2028">2028</a>
<a href="#2029" id="2029">2029</a>
<a href="#2030" id="2030">2030</a>
<a href="#2031" id="2031">2031</a>
<a href="#2032" id="2032">2032</a>
<a href="#2033" id="2033">2033</a>
<a href="#2034" id="2034">2034</a>
<a href="#2035" id="2035">2035</a>
<a href="#2036" id="2036">2036</a>
<a href="#2037" id="2037">2037</a>
<a href="#2038" id="2038">2038</a>
<a href="#2039" id="2039">2039</a>
<a href="#2040" id="2040">2040</a>
<a href="#2041" id="2041">2041</a>
<a href="#2042" id="2042">2042</a>
<a href="#2043" id="2043">2043</a>
<a href="#2044" id="2044">2044</a>
<a href="#2045" id="2045">2045</a>
<a href="#2046" id="2046">2046</a>
<a href="#2047" id="2047">2047</a>
<a href="#2048" id="2048">2048</a>
<a href="#2049" id="2049">2049</a>
<a href="#2050" id="2050">2050</a>
<a href="#2051" id="2051">2051</a>
<a href="#2052" id="2052">2052</a>
<a href="#2053" id="2053">2053</a>
<a href="#2054" id="2054">2054</a>
<a href="#2055" id="2055">2055</a>
<a href="#2056" id="2056">2056</a>
<a href="#2057" id="2057">2057</a>
<a href="#2058" id="2058">2058</a>
<a href="#2059" id="2059">2059</a>
<a href="#2060" id="2060">2060</a>
<a href="#2061" id="2061">2061</a>
<a href="#2062" id="2062">2062</a>
<a href="#2063" id="2063">2063</a>
<a href="#2064" id="2064">2064</a>
<a href="#2065" id="2065">2065</a>
<a href="#2066" id="2066">2066</a>
<a href="#2067" id="2067">2067</a>
<a href="#2068" id="2068">2068</a>
<a href="#2069" id="2069">2069</a>
<a href="#2070" id="2070">2070</a>
<a href="#2071" id="2071">2071</a>
<a href="#2072" id="2072">2072</a>
<a href="#2073" id="2073">2073</a>
<a href="#2074" id="2074">2074</a>
<a href="#2075" id="2075">2075</a>
<a href="#2076" id="2076">2076</a>
<a href="#2077" id="2077">2077</a>
<a href="#2078" id="2078">2078</a>
<a href="#2079" id="2079">2079</a>
<a href="#2080" id="2080">2080</a>
<a href="#2081" id="2081">2081</a>
<a href="#2082" id="2082">2082</a>
<a href="#2083" id="2083">2083</a>
<a href="#2084" id="2084">2084</a>
<a href="#2085" id="2085">2085</a>
<a href="#2086" id="2086">2086</a>
<a href="#2087" id="2087">2087</a>
<a href="#2088" id="2088">2088</a>
<a href="#2089" id="2089">2089</a>
<a href="#2090" id="2090">2090</a>
<a href="#2091" id="2091">2091</a>
<a href="#2092" id="2092">2092</a>
<a href="#2093" id="2093">2093</a>
<a href="#2094" id="2094">2094</a>
<a href="#2095" id="2095">2095</a>
<a href="#2096" id="2096">2096</a>
<a href="#2097" id="2097">2097</a>
<a href="#2098" id="2098">2098</a>
<a href="#2099" id="2099">2099</a>
<a href="#2100" id="2100">2100</a>
<a href="#2101" id="2101">2101</a>
<a href="#2102" id="2102">2102</a>
<a href="#2103" id="2103">2103</a>
<a href="#2104" id="2104">2104</a>
<a href="#2105" id="2105">2105</a>
<a href="#2106" id="2106">2106</a>
<a href="#2107" id="2107">2107</a>
<a href="#2108" id="2108">2108</a>
<a href="#2109" id="2109">2109</a>
<a href="#2110" id="2110">2110</a>
<a href="#2111" id="2111">2111</a>
<a href="#2112" id="2112">2112</a>
<a href="#2113" id="2113">2113</a>
<a href="#2114" id="2114">2114</a>
<a href="#2115" id="2115">2115</a>
<a href="#2116" id="2116">2116</a>
<a href="#2117" id="2117">2117</a>
<a href="#2118" id="2118">2118</a>
<a href="#2119" id="2119">2119</a>
<a href="#2120" id="2120">2120</a>
<a href="#2121" id="2121">2121</a>
<a href="#2122" id="2122">2122</a>
<a href="#2123" id="2123">2123</a>
<a href="#2124" id="2124">2124</a>
<a href="#2125" id="2125">2125</a>
<a href="#2126" id="2126">2126</a>
<a href="#2127" id="2127">2127</a>
<a href="#2128" id="2128">2128</a>
<a href="#2129" id="2129">2129</a>
<a href="#2130" id="2130">2130</a>
<a href="#2131" id="2131">2131</a>
<a href="#2132" id="2132">2132</a>
<a href="#2133" id="2133">2133</a>
<a href="#2134" id="2134">2134</a>
<a href="#2135" id="2135">2135</a>
<a href="#2136" id="2136">2136</a>
<a href="#2137" id="2137">2137</a>
<a href="#2138" id="2138">2138</a>
<a href="#2139" id="2139">2139</a>
<a href="#2140" id="2140">2140</a>
<a href="#2141" id="2141">2141</a>
<a href="#2142" id="2142">2142</a>
<a href="#2143" id="2143">2143</a>
<a href="#2144" id="2144">2144</a>
<a href="#2145" id="2145">2145</a>
<a href="#2146" id="2146">2146</a>
<a href="#2147" id="2147">2147</a>
<a href="#2148" id="2148">2148</a>
<a href="#2149" id="2149">2149</a>
<a href="#2150" id="2150">2150</a>
<a href="#2151" id="2151">2151</a>
<a href="#2152" id="2152">2152</a>
<a href="#2153" id="2153">2153</a>
<a href="#2154" id="2154">2154</a>
<a href="#2155" id="2155">2155</a>
<a href="#2156" id="2156">2156</a>
<a href="#2157" id="2157">2157</a>
<a href="#2158" id="2158">2158</a>
<a href="#2159" id="2159">2159</a>
<a href="#2160" id="2160">2160</a>
<a href="#2161" id="2161">2161</a>
<a href="#2162" id="2162">2162</a>
<a href="#2163" id="2163">2163</a>
<a href="#2164" id="2164">2164</a>
<a href="#2165" id="2165">2165</a>
<a href="#2166" id="2166">2166</a>
<a href="#2167" id="2167">2167</a>
<a href="#2168" id="2168">2168</a>
<a href="#2169" id="2169">2169</a>
<a href="#2170" id="2170">2170</a>
<a href="#2171" id="2171">2171</a>
<a href="#2172" id="2172">2172</a>
<a href="#2173" id="2173">2173</a>
<a href="#2174" id="2174">2174</a>
<a href="#2175" id="2175">2175</a>
<a href="#2176" id="2176">2176</a>
<a href="#2177" id="2177">2177</a>
<a href="#2178" id="2178">2178</a>
<a href="#2179" id="2179">2179</a>
<a href="#2180" id="2180">2180</a>
<a href="#2181" id="2181">2181</a>
<a href="#2182" id="2182">2182</a>
<a href="#2183" id="2183">2183</a>
<a href="#2184" id="2184">2184</a>
<a href="#2185" id="2185">2185</a>
<a href="#2186" id="2186">2186</a>
<a href="#2187" id="2187">2187</a>
<a href="#2188" id="2188">2188</a>
<a href="#2189" id="2189">2189</a>
<a href="#2190" id="2190">2190</a>
<a href="#2191" id="2191">2191</a>
<a href="#2192" id="2192">2192</a>
<a href="#2193" id="2193">2193</a>
<a href="#2194" id="2194">2194</a>
<a href="#2195" id="2195">2195</a>
<a href="#2196" id="2196">2196</a>
<a href="#2197" id="2197">2197</a>
<a href="#2198" id="2198">2198</a>
<a href="#2199" id="2199">2199</a>
<a href="#2200" id="2200">2200</a>
<a href="#2201" id="2201">2201</a>
<a href="#2202" id="2202">2202</a>
<a href="#2203" id="2203">2203</a>
<a href="#2204" id="2204">2204</a>
<a href="#2205" id="2205">2205</a>
<a href="#2206" id="2206">2206</a>
<a href="#2207" id="2207">2207</a>
<a href="#2208" id="2208">2208</a>
<a href="#2209" id="2209">2209</a>
<a href="#2210" id="2210">2210</a>
<a href="#2211" id="2211">2211</a>
<a href="#2212" id="2212">2212</a>
<a href="#2213" id="2213">2213</a>
<a href="#2214" id="2214">2214</a>
<a href="#2215" id="2215">2215</a>
<a href="#2216" id="2216">2216</a>
<a href="#2217" id="2217">2217</a>
<a href="#2218" id="2218">2218</a>
<a href="#2219" id="2219">2219</a>
<a href="#2220" id="2220">2220</a>
<a href="#2221" id="2221">2221</a>
<a href="#2222" id="2222">2222</a>
<a href="#2223" id="2223">2223</a>
<a href="#2224" id="2224">2224</a>
<a href="#2225" id="2225">2225</a>
<a href="#2226" id="2226">2226</a>
<a href="#2227" id="2227">2227</a>
<a href="#2228" id="2228">2228</a>
<a href="#2229" id="2229">2229</a>
<a href="#2230" id="2230">2230</a>
<a href="#2231" id="2231">2231</a>
<a href="#2232" id="2232">2232</a>
<a href="#2233" id="2233">2233</a>
<a href="#2234" id="2234">2234</a>
<a href="#2235" id="2235">2235</a>
<a href="#2236" id="2236">2236</a>
<a href="#2237" id="2237">2237</a>
<a href="#2238" id="2238">2238</a>
<a href="#2239" id="2239">2239</a>
<a href="#2240" id="2240">2240</a>
<a href="#2241" id="2241">2241</a>
<a href="#2242" id="2242">2242</a>
<a href="#2243" id="2243">2243</a>
<a href="#2244" id="2244">2244</a>
<a href="#2245" id="2245">2245</a>
<a href="#2246" id="2246">2246</a>
<a href="#2247" id="2247">2247</a>
<a href="#2248" id="2248">2248</a>
<a href="#2249" id="2249">2249</a>
<a href="#2250" id="2250">2250</a>
<a href="#2251" id="2251">2251</a>
<a href="#2252" id="2252">2252</a>
<a href="#2253" id="2253">2253</a>
<a href="#2254" id="2254">2254</a>
<a href="#2255" id="2255">2255</a>
<a href="#2256" id="2256">2256</a>
<a href="#2257" id="2257">2257</a>
<a href="#2258" id="2258">2258</a>
<a href="#2259" id="2259">2259</a>
<a href="#2260" id="2260">2260</a>
<a href="#2261" id="2261">2261</a>
<a href="#2262" id="2262">2262</a>
<a href="#2263" id="2263">2263</a>
<a href="#2264" id="2264">2264</a>
<a href="#2265" id="2265">2265</a>
<a href="#2266" id="2266">2266</a>
<a href="#2267" id="2267">2267</a>
<a href="#2268" id="2268">2268</a>
<a href="#2269" id="2269">2269</a>
<a href="#2270" id="2270">2270</a>
<a href="#2271" id="2271">2271</a>
<a href="#2272" id="2272">2272</a>
<a href="#2273" id="2273">2273</a>
<a href="#2274" id="2274">2274</a>
<a href="#2275" id="2275">2275</a>
<a href="#2276" id="2276">2276</a>
<a href="#2277" id="2277">2277</a>
<a href="#2278" id="2278">2278</a>
<a href="#2279" id="2279">2279</a>
<a href="#2280" id="2280">2280</a>
<a href="#2281" id="2281">2281</a>
<a href="#2282" id="2282">2282</a>
<a href="#2283" id="2283">2283</a>
<a href="#2284" id="2284">2284</a>
<a href="#2285" id="2285">2285</a>
<a href="#2286" id="2286">2286</a>
<a href="#2287" id="2287">2287</a>
<a href="#2288" id="2288">2288</a>
<a href="#2289" id="2289">2289</a>
<a href="#2290" id="2290">2290</a>
<a href="#2291" id="2291">2291</a>
<a href="#2292" id="2292">2292</a>
<a href="#2293" id="2293">2293</a>
<a href="#2294" id="2294">2294</a>
<a href="#2295" id="2295">2295</a>
<a href="#2296" id="2296">2296</a>
<a href="#2297" id="2297">2297</a>
<a href="#2298" id="2298">2298</a>
<a href="#2299" id="2299">2299</a>
<a href="#2300" id="2300">2300</a>
<a href="#2301" id="2301">2301</a>
<a href="#2302" id="2302">2302</a>
<a href="#2303" id="2303">2303</a>
<a href="#2304" id="2304">2304</a>
<a href="#2305" id="2305">2305</a>
<a href="#2306" id="2306">2306</a>
<a href="#2307" id="2307">2307</a>
<a href="#2308" id="2308">2308</a>
<a href="#2309" id="2309">2309</a>
<a href="#2310" id="2310">2310</a>
<a href="#2311" id="2311">2311</a>
<a href="#2312" id="2312">2312</a>
<a href="#2313" id="2313">2313</a>
<a href="#2314" id="2314">2314</a>
<a href="#2315" id="2315">2315</a>
<a href="#2316" id="2316">2316</a>
<a href="#2317" id="2317">2317</a>
<a href="#2318" id="2318">2318</a>
<a href="#2319" id="2319">2319</a>
<a href="#2320" id="2320">2320</a>
<a href="#2321" id="2321">2321</a>
<a href="#2322" id="2322">2322</a>
<a href="#2323" id="2323">2323</a>
<a href="#2324" id="2324">2324</a>
<a href="#2325" id="2325">2325</a>
<a href="#2326" id="2326">2326</a>
<a href="#2327" id="2327">2327</a>
<a href="#2328" id="2328">2328</a>
<a href="#2329" id="2329">2329</a>
<a href="#2330" id="2330">2330</a>
<a href="#2331" id="2331">2331</a>
<a href="#2332" id="2332">2332</a>
<a href="#2333" id="2333">2333</a>
<a href="#2334" id="2334">2334</a>
<a href="#2335" id="2335">2335</a>
<a href="#2336" id="2336">2336</a>
<a href="#2337" id="2337">2337</a>
<a href="#2338" id="2338">2338</a>
<a href="#2339" id="2339">2339</a>
<a href="#2340" id="2340">2340</a>
<a href="#2341" id="2341">2341</a>
<a href="#2342" id="2342">2342</a>
<a href="#2343" id="2343">2343</a>
<a href="#2344" id="2344">2344</a>
<a href="#2345" id="2345">2345</a>
<a href="#2346" id="2346">2346</a>
<a href="#2347" id="2347">2347</a>
<a href="#2348" id="2348">2348</a>
<a href="#2349" id="2349">2349</a>
<a href="#2350" id="2350">2350</a>
<a href="#2351" id="2351">2351</a>
<a href="#2352" id="2352">2352</a>
<a href="#2353" id="2353">2353</a>
<a href="#2354" id="2354">2354</a>
<a href="#2355" id="2355">2355</a>
<a href="#2356" id="2356">2356</a>
<a href="#2357" id="2357">2357</a>
<a href="#2358" id="2358">2358</a>
<a href="#2359" id="2359">2359</a>
<a href="#2360" id="2360">2360</a>
<a href="#2361" id="2361">2361</a>
<a href="#2362" id="2362">2362</a>
<a href="#2363" id="2363">2363</a>
<a href="#2364" id="2364">2364</a>
<a href="#2365" id="2365">2365</a>
<a href="#2366" id="2366">2366</a>
<a href="#2367" id="2367">2367</a>
<a href="#2368" id="2368">2368</a>
<a href="#2369" id="2369">2369</a>
<a href="#2370" id="2370">2370</a>
<a href="#2371" id="2371">2371</a>
<a href="#2372" id="2372">2372</a>
<a href="#2373" id="2373">2373</a>
<a href="#2374" id="2374">2374</a>
<a href="#2375" id="2375">2375</a>
<a href="#2376" id="2376">2376</a>
<a href="#2377" id="2377">2377</a>
<a href="#2378" id="2378">2378</a>
<a href="#2379" id="2379">2379</a>
<a href="#2380" id="2380">2380</a>
<a href="#2381" id="2381">2381</a>
<a href="#2382" id="2382">2382</a>
<a href="#2383" id="2383">2383</a>
<a href="#2384" id="2384">2384</a>
<a href="#2385" id="2385">2385</a>
<a href="#2386" id="2386">2386</a>
<a href="#2387" id="2387">2387</a>
<a href="#2388" id="2388">2388</a>
<a href="#2389" id="2389">2389</a>
<a href="#2390" id="2390">2390</a>
<a href="#2391" id="2391">2391</a>
<a href="#2392" id="2392">2392</a>
<a href="#2393" id="2393">2393</a>
<a href="#2394" id="2394">2394</a>
<a href="#2395" id="2395">2395</a>
<a href="#2396" id="2396">2396</a>
<a href="#2397" id="2397">2397</a>
<a href="#2398" id="2398">2398</a>
<a href="#2399" id="2399">2399</a>
<a href="#2400" id="2400">2400</a>
<a href="#2401" id="2401">2401</a>
<a href="#2402" id="2402">2402</a>
<a href="#2403" id="2403">2403</a>
<a href="#2404" id="2404">2404</a>
<a href="#2405" id="2405">2405</a>
<a href="#2406" id="2406">2406</a>
<a href="#2407" id="2407">2407</a>
<a href="#2408" id="2408">2408</a>
<a href="#2409" id="2409">2409</a>
<a href="#2410" id="2410">2410</a>
<a href="#2411" id="2411">2411</a>
<a href="#2412" id="2412">2412</a>
<a href="#2413" id="2413">2413</a>
<a href="#2414" id="2414">2414</a>
<a href="#2415" id="2415">2415</a>
<a href="#2416" id="2416">2416</a>
<a href="#2417" id="2417">2417</a>
<a href="#2418" id="2418">2418</a>
<a href="#2419" id="2419">2419</a>
<a href="#2420" id="2420">2420</a>
<a href="#2421" id="2421">2421</a>
<a href="#2422" id="2422">2422</a>
<a href="#2423" id="2423">2423</a>
<a href="#2424" id="2424">2424</a>
<a href="#2425" id="2425">2425</a>
<a href="#2426" id="2426">2426</a>
<a href="#2427" id="2427">2427</a>
<a href="#2428" id="2428">2428</a>
<a href="#2429" id="2429">2429</a>
<a href="#2430" id="2430">2430</a>
<a href="#2431" id="2431">2431</a>
<a href="#2432" id="2432">2432</a>
<a href="#2433" id="2433">2433</a>
<a href="#2434" id="2434">2434</a>
<a href="#2435" id="2435">2435</a>
<a href="#2436" id="2436">2436</a>
<a href="#2437" id="2437">2437</a>
<a href="#2438" id="2438">2438</a>
<a href="#2439" id="2439">2439</a>
<a href="#2440" id="2440">2440</a>
<a href="#2441" id="2441">2441</a>
<a href="#2442" id="2442">2442</a>
<a href="#2443" id="2443">2443</a>
<a href="#2444" id="2444">2444</a>
<a href="#2445" id="2445">2445</a>
<a href="#2446" id="2446">2446</a>
<a href="#2447" id="2447">2447</a>
<a href="#2448" id="2448">2448</a>
<a href="#2449" id="2449">2449</a>
<a href="#2450" id="2450">2450</a>
<a href="#2451" id="2451">2451</a>
<a href="#2452" id="2452">2452</a>
<a href="#2453" id="2453">2453</a>
<a href="#2454" id="2454">2454</a>
<a href="#2455" id="2455">2455</a>
<a href="#2456" id="2456">2456</a>
<a href="#2457" id="2457">2457</a>
<a href="#2458" id="2458">2458</a>
<a href="#2459" id="2459">2459</a>
<a href="#2460" id="2460">2460</a>
<a href="#2461" id="2461">2461</a>
<a href="#2462" id="2462">2462</a>
<a href="#2463" id="2463">2463</a>
<a href="#2464" id="2464">2464</a>
<a href="#2465" id="2465">2465</a>
<a href="#2466" id="2466">2466</a>
<a href="#2467" id="2467">2467</a>
<a href="#2468" id="2468">2468</a>
<a href="#2469" id="2469">2469</a>
<a href="#2470" id="2470">2470</a>
<a href="#2471" id="2471">2471</a>
<a href="#2472" id="2472">2472</a>
<a href="#2473" id="2473">2473</a>
<a href="#2474" id="2474">2474</a>
<a href="#2475" id="2475">2475</a>
<a href="#2476" id="2476">2476</a>
<a href="#2477" id="2477">2477</a>
<a href="#2478" id="2478">2478</a>
<a href="#2479" id="2479">2479</a>
<a href="#2480" id="2480">2480</a>
<a href="#2481" id="2481">2481</a>
<a href="#2482" id="2482">2482</a>
<a href="#2483" id="2483">2483</a>
<a href="#2484" id="2484">2484</a>
<a href="#2485" id="2485">2485</a>
<a href="#2486" id="2486">2486</a>
<a href="#2487" id="2487">2487</a>
<a href="#2488" id="2488">2488</a>
<a href="#2489" id="2489">2489</a>
<a href="#2490" id="2490">2490</a>
<a href="#2491" id="2491">2491</a>
<a href="#2492" id="2492">2492</a>
<a href="#2493" id="2493">2493</a>
<a href="#2494" id="2494">2494</a>
<a href="#2495" id="2495">2495</a>
<a href="#2496" id="2496">2496</a>
<a href="#2497" id="2497">2497</a>
<a href="#2498" id="2498">2498</a>
<a href="#2499" id="2499">2499</a>
<a href="#2500" id="2500">2500</a>
<a href="#2501" id="2501">2501</a>
<a href="#2502" id="2502">2502</a>
<a href="#2503" id="2503">2503</a>
<a href="#2504" id="2504">2504</a>
<a href="#2505" id="2505">2505</a>
<a href="#2506" id="2506">2506</a>
<a href="#2507" id="2507">2507</a>
<a href="#2508" id="2508">2508</a>
<a href="#2509" id="2509">2509</a>
<a href="#2510" id="2510">2510</a>
<a href="#2511" id="2511">2511</a>
<a href="#2512" id="2512">2512</a>
<a href="#2513" id="2513">2513</a>
<a href="#2514" id="2514">2514</a>
<a href="#2515" id="2515">2515</a>
<a href="#2516" id="2516">2516</a>
<a href="#2517" id="2517">2517</a>
<a href="#2518" id="2518">2518</a>
<a href="#2519" id="2519">2519</a>
<a href="#2520" id="2520">2520</a>
<a href="#2521" id="2521">2521</a>
<a href="#2522" id="2522">2522</a>
<a href="#2523" id="2523">2523</a>
<a href="#2524" id="2524">2524</a>
<a href="#2525" id="2525">2525</a>
<a href="#2526" id="2526">2526</a>
<a href="#2527" id="2527">2527</a>
<a href="#2528" id="2528">2528</a>
<a href="#2529" id="2529">2529</a>
<a href="#2530" id="2530">2530</a>
<a href="#2531" id="2531">2531</a>
<a href="#2532" id="2532">2532</a>
<a href="#2533" id="2533">2533</a>
<a href="#2534" id="2534">2534</a>
<a href="#2535" id="2535">2535</a>
<a href="#2536" id="2536">2536</a>
<a href="#2537" id="2537">2537</a>
<a href="#2538" id="2538">2538</a>
<a href="#2539" id="2539">2539</a>
<a href="#2540" id="2540">2540</a>
<a href="#2541" id="2541">2541</a>
<a href="#2542" id="2542">2542</a>
<a href="#2543" id="2543">2543</a>
<a href="#2544" id="2544">2544</a>
<a href="#2545" id="2545">2545</a>
<a href="#2546" id="2546">2546</a>
<a href="#2547" id="2547">2547</a>
<a href="#2548" id="2548">2548</a>
<a href="#2549" id="2549">2549</a>
<a href="#2550" id="2550">2550</a>
<a href="#2551" id="2551">2551</a>
<a href="#2552" id="2552">2552</a>
<a href="#2553" id="2553">2553</a>
<a href="#2554" id="2554">2554</a>
<a href="#2555" id="2555">2555</a>
<a href="#2556" id="2556">2556</a>
<a href="#2557" id="2557">2557</a>
<a href="#2558" id="2558">2558</a>
<a href="#2559" id="2559">2559</a>
<a href="#2560" id="2560">2560</a>
<a href="#2561" id="2561">2561</a>
<a href="#2562" id="2562">2562</a>
<a href="#2563" id="2563">2563</a>
<a href="#2564" id="2564">2564</a>
<a href="#2565" id="2565">2565</a>
<a href="#2566" id="2566">2566</a>
<a href="#2567" id="2567">2567</a>
<a href="#2568" id="2568">2568</a>
<a href="#2569" id="2569">2569</a>
<a href="#2570" id="2570">2570</a>
<a href="#2571" id="2571">2571</a>
<a href="#2572" id="2572">2572</a>
<a href="#2573" id="2573">2573</a>
<a href="#2574" id="2574">2574</a>
<a href="#2575" id="2575">2575</a>
<a href="#2576" id="2576">2576</a>
<a href="#2577" id="2577">2577</a>
<a href="#2578" id="2578">2578</a>
<a href="#2579" id="2579">2579</a>
<a href="#2580" id="2580">2580</a>
<a href="#2581" id="2581">2581</a>
<a href="#2582" id="2582">2582</a>
<a href="#2583" id="2583">2583</a>
<a href="#2584" id="2584">2584</a>
<a href="#2585" id="2585">2585</a>
<a href="#2586" id="2586">2586</a>
<a href="#2587" id="2587">2587</a>
<a href="#2588" id="2588">2588</a>
<a href="#2589" id="2589">2589</a>
<a href="#2590" id="2590">2590</a>
<a href="#2591" id="2591">2591</a>
<a href="#2592" id="2592">2592</a>
<a href="#2593" id="2593">2593</a>
<a href="#2594" id="2594">2594</a>
<a href="#2595" id="2595">2595</a>
<a href="#2596" id="2596">2596</a>
<a href="#2597" id="2597">2597</a>
<a href="#2598" id="2598">2598</a>
<a href="#2599" id="2599">2599</a>
<a href="#2600" id="2600">2600</a>
<a href="#2601" id="2601">2601</a>
<a href="#2602" id="2602">2602</a>
<a href="#2603" id="2603">2603</a>
<a href="#2604" id="2604">2604</a>
<a href="#2605" id="2605">2605</a>
<a href="#2606" id="2606">2606</a>
<a href="#2607" id="2607">2607</a>
<a href="#2608" id="2608">2608</a>
<a href="#2609" id="2609">2609</a>
<a href="#2610" id="2610">2610</a>
<a href="#2611" id="2611">2611</a>
<a href="#2612" id="2612">2612</a>
<a href="#2613" id="2613">2613</a>
<a href="#2614" id="2614">2614</a>
<a href="#2615" id="2615">2615</a>
<a href="#2616" id="2616">2616</a>
<a href="#2617" id="2617">2617</a>
<a href="#2618" id="2618">2618</a>
<a href="#2619" id="2619">2619</a>
<a href="#2620" id="2620">2620</a>
<a href="#2621" id="2621">2621</a>
<a href="#2622" id="2622">2622</a>
<a href="#2623" id="2623">2623</a>
<a href="#2624" id="2624">2624</a>
<a href="#2625" id="2625">2625</a>
<a href="#2626" id="2626">2626</a>
<a href="#2627" id="2627">2627</a>
<a href="#2628" id="2628">2628</a>
<a href="#2629" id="2629">2629</a>
<a href="#2630" id="2630">2630</a>
<a href="#2631" id="2631">2631</a>
<a href="#2632" id="2632">2632</a>
<a href="#2633" id="2633">2633</a>
<a href="#2634" id="2634">2634</a>
<a href="#2635" id="2635">2635</a>
<a href="#2636" id="2636">2636</a>
<a href="#2637" id="2637">2637</a>
<a href="#2638" id="2638">2638</a>
<a href="#2639" id="2639">2639</a>
<a href="#2640" id="2640">2640</a>
<a href="#2641" id="2641">2641</a>
<a href="#2642" id="2642">2642</a>
<a href="#2643" id="2643">2643</a>
<a href="#2644" id="2644">2644</a>
<a href="#2645" id="2645">2645</a>
<a href="#2646" id="2646">2646</a>
<a href="#2647" id="2647">2647</a>
<a href="#2648" id="2648">2648</a>
<a href="#2649" id="2649">2649</a>
<a href="#2650" id="2650">2650</a>
<a href="#2651" id="2651">2651</a>
<a href="#2652" id="2652">2652</a>
<a href="#2653" id="2653">2653</a>
<a href="#2654" id="2654">2654</a>
<a href="#2655" id="2655">2655</a>
<a href="#2656" id="2656">2656</a>
<a href="#2657" id="2657">2657</a>
<a href="#2658" id="2658">2658</a>
<a href="#2659" id="2659">2659</a>
<a href="#2660" id="2660">2660</a>
<a href="#2661" id="2661">2661</a>
<a href="#2662" id="2662">2662</a>
<a href="#2663" id="2663">2663</a>
<a href="#2664" id="2664">2664</a>
<a href="#2665" id="2665">2665</a>
<a href="#2666" id="2666">2666</a>
<a href="#2667" id="2667">2667</a>
<a href="#2668" id="2668">2668</a>
<a href="#2669" id="2669">2669</a>
<a href="#2670" id="2670">2670</a>
<a href="#2671" id="2671">2671</a>
<a href="#2672" id="2672">2672</a>
<a href="#2673" id="2673">2673</a>
<a href="#2674" id="2674">2674</a>
<a href="#2675" id="2675">2675</a>
<a href="#2676" id="2676">2676</a>
<a href="#2677" id="2677">2677</a>
<a href="#2678" id="2678">2678</a>
<a href="#2679" id="2679">2679</a>
<a href="#2680" id="2680">2680</a>
<a href="#2681" id="2681">2681</a>
<a href="#2682" id="2682">2682</a>
<a href="#2683" id="2683">2683</a>
<a href="#2684" id="2684">2684</a>
<a href="#2685" id="2685">2685</a>
<a href="#2686" id="2686">2686</a>
<a href="#2687" id="2687">2687</a>
<a href="#2688" id="2688">2688</a>
<a href="#2689" id="2689">2689</a>
<a href="#2690" id="2690">2690</a>
<a href="#2691" id="2691">2691</a>
<a href="#2692" id="2692">2692</a>
<a href="#2693" id="2693">2693</a>
<a href="#2694" id="2694">2694</a>
<a href="#2695" id="2695">2695</a>
<a href="#2696" id="2696">2696</a>
<a href="#2697" id="2697">2697</a>
<a href="#2698" id="2698">2698</a>
<a href="#2699" id="2699">2699</a>
<a href="#2700" id="2700">2700</a>
<a href="#2701" id="2701">2701</a>
<a href="#2702" id="2702">2702</a>
<a href="#2703" id="2703">2703</a>
<a href="#2704" id="2704">2704</a>
<a href="#2705" id="2705">2705</a>
<a href="#2706" id="2706">2706</a>
<a href="#2707" id="2707">2707</a>
<a href="#2708" id="2708">2708</a>
<a href="#2709" id="2709">2709</a>
<a href="#2710" id="2710">2710</a>
<a href="#2711" id="2711">2711</a>
<a href="#2712" id="2712">2712</a>
<a href="#2713" id="2713">2713</a>
<a href="#2714" id="2714">2714</a>
<a href="#2715" id="2715">2715</a>
<a href="#2716" id="2716">2716</a>
<a href="#2717" id="2717">2717</a>
<a href="#2718" id="2718">2718</a>
<a href="#2719" id="2719">2719</a>
<a href="#2720" id="2720">2720</a>
<a href="#2721" id="2721">2721</a>
<a href="#2722" id="2722">2722</a>
<a href="#2723" id="2723">2723</a>
<a href="#2724" id="2724">2724</a>
<a href="#2725" id="2725">2725</a>
<a href="#2726" id="2726">2726</a>
<a href="#2727" id="2727">2727</a>
<a href="#2728" id="2728">2728</a>
<a href="#2729" id="2729">2729</a>
<a href="#2730" id="2730">2730</a>
<a href="#2731" id="2731">2731</a>
<a href="#2732" id="2732">2732</a>
<a href="#2733" id="2733">2733</a>
<a href="#2734" id="2734">2734</a>
<a href="#2735" id="2735">2735</a>
<a href="#2736" id="2736">2736</a>
<a href="#2737" id="2737">2737</a>
<a href="#2738" id="2738">2738</a>
<a href="#2739" id="2739">2739</a>
<a href="#2740" id="2740">2740</a>
<a href="#2741" id="2741">2741</a>
<a href="#2742" id="2742">2742</a>
<a href="#2743" id="2743">2743</a>
<a href="#2744" id="2744">2744</a>
<a href="#2745" id="2745">2745</a>
<a href="#2746" id="2746">2746</a>
<a href="#2747" id="2747">2747</a>
<a href="#2748" id="2748">2748</a>
<a href="#2749" id="2749">2749</a>
<a href="#2750" id="2750">2750</a>
<a href="#2751" id="2751">2751</a>
<a href="#2752" id="2752">2752</a>
<a href="#2753" id="2753">2753</a>
<a href="#2754" id="2754">2754</a>
<a href="#2755" id="2755">2755</a>
<a href="#2756" id="2756">2756</a>
<a href="#2757" id="2757">2757</a>
<a href="#2758" id="2758">2758</a>
<a href="#2759" id="2759">2759</a>
<a href="#2760" id="2760">2760</a>
<a href="#2761" id="2761">2761</a>
<a href="#2762" id="2762">2762</a>
<a href="#2763" id="2763">2763</a>
<a href="#2764" id="2764">2764</a>
<a href="#2765" id="2765">2765</a>
<a href="#2766" id="2766">2766</a>
<a href="#2767" id="2767">2767</a>
<a href="#2768" id="2768">2768</a>
<a href="#2769" id="2769">2769</a>
<a href="#2770" id="2770">2770</a>
<a href="#2771" id="2771">2771</a>
<a href="#2772" id="2772">2772</a>
<a href="#2773" id="2773">2773</a>
<a href="#2774" id="2774">2774</a>
<a href="#2775" id="2775">2775</a>
<a href="#2776" id="2776">2776</a>
<a href="#2777" id="2777">2777</a>
<a href="#2778" id="2778">2778</a>
<a href="#2779" id="2779">2779</a>
<a href="#2780" id="2780">2780</a>
<a href="#2781" id="2781">2781</a>
<a href="#2782" id="2782">2782</a>
<a href="#2783" id="2783">2783</a>
<a href="#2784" id="2784">2784</a>
<a href="#2785" id="2785">2785</a>
<a href="#2786" id="2786">2786</a>
<a href="#2787" id="2787">2787</a>
<a href="#2788" id="2788">2788</a>
<a href="#2789" id="2789">2789</a>
<a href="#2790" id="2790">2790</a>
<a href="#2791" id="2791">2791</a>
<a href="#2792" id="2792">2792</a>
<a href="#2793" id="2793">2793</a>
<a href="#2794" id="2794">2794</a>
<a href="#2795" id="2795">2795</a>
<a href="#2796" id="2796">2796</a>
<a href="#2797" id="2797">2797</a>
<a href="#2798" id="2798">2798</a>
<a href="#2799" id="2799">2799</a>
<a href="#2800" id="2800">2800</a>
<a href="#2801" id="2801">2801</a>
<a href="#2802" id="2802">2802</a>
<a href="#2803" id="2803">2803</a>
<a href="#2804" id="2804">2804</a>
<a href="#2805" id="2805">2805</a>
<a href="#2806" id="2806">2806</a>
<a href="#2807" id="2807">2807</a>
<a href="#2808" id="2808">2808</a>
<a href="#2809" id="2809">2809</a>
<a href="#2810" id="2810">2810</a>
<a href="#2811" id="2811">2811</a>
<a href="#2812" id="2812">2812</a>
<a href="#2813" id="2813">2813</a>
<a href="#2814" id="2814">2814</a>
<a href="#2815" id="2815">2815</a>
<a href="#2816" id="2816">2816</a>
<a href="#2817" id="2817">2817</a>
<a href="#2818" id="2818">2818</a>
<a href="#2819" id="2819">2819</a>
<a href="#2820" id="2820">2820</a>
<a href="#2821" id="2821">2821</a>
<a href="#2822" id="2822">2822</a>
<a href="#2823" id="2823">2823</a>
<a href="#2824" id="2824">2824</a>
<a href="#2825" id="2825">2825</a>
<a href="#2826" id="2826">2826</a>
<a href="#2827" id="2827">2827</a>
<a href="#2828" id="2828">2828</a>
<a href="#2829" id="2829">2829</a>
<a href="#2830" id="2830">2830</a>
<a href="#2831" id="2831">2831</a>
<a href="#2832" id="2832">2832</a>
<a href="#2833" id="2833">2833</a>
<a href="#2834" id="2834">2834</a>
<a href="#2835" id="2835">2835</a>
<a href="#2836" id="2836">2836</a>
<a href="#2837" id="2837">2837</a>
<a href="#2838" id="2838">2838</a>
<a href="#2839" id="2839">2839</a>
<a href="#2840" id="2840">2840</a>
<a href="#2841" id="2841">2841</a>
<a href="#2842" id="2842">2842</a>
<a href="#2843" id="2843">2843</a>
<a href="#2844" id="2844">2844</a>
<a href="#2845" id="2845">2845</a>
<a href="#2846" id="2846">2846</a>
<a href="#2847" id="2847">2847</a>
<a href="#2848" id="2848">2848</a>
<a href="#2849" id="2849">2849</a>
<a href="#2850" id="2850">2850</a>
<a href="#2851" id="2851">2851</a>
<a href="#2852" id="2852">2852</a>
<a href="#2853" id="2853">2853</a>
<a href="#2854" id="2854">2854</a>
<a href="#2855" id="2855">2855</a>
<a href="#2856" id="2856">2856</a>
<a href="#2857" id="2857">2857</a>
<a href="#2858" id="2858">2858</a>
<a href="#2859" id="2859">2859</a>
<a href="#2860" id="2860">2860</a>
<a href="#2861" id="2861">2861</a>
<a href="#2862" id="2862">2862</a>
<a href="#2863" id="2863">2863</a>
<a href="#2864" id="2864">2864</a>
<a href="#2865" id="2865">2865</a>
<a href="#2866" id="2866">2866</a>
<a href="#2867" id="2867">2867</a>
<a href="#2868" id="2868">2868</a>
<a href="#2869" id="2869">2869</a>
<a href="#2870" id="2870">2870</a>
<a href="#2871" id="2871">2871</a>
<a href="#2872" id="2872">2872</a>
<a href="#2873" id="2873">2873</a>
<a href="#2874" id="2874">2874</a>
<a href="#2875" id="2875">2875</a>
<a href="#2876" id="2876">2876</a>
<a href="#2877" id="2877">2877</a>
<a href="#2878" id="2878">2878</a>
<a href="#2879" id="2879">2879</a>
<a href="#2880" id="2880">2880</a>
<a href="#2881" id="2881">2881</a>
<a href="#2882" id="2882">2882</a>
<a href="#2883" id="2883">2883</a>
<a href="#2884" id="2884">2884</a>
<a href="#2885" id="2885">2885</a>
<a href="#2886" id="2886">2886</a>
<a href="#2887" id="2887">2887</a>
<a href="#2888" id="2888">2888</a>
<a href="#2889" id="2889">2889</a>
<a href="#2890" id="2890">2890</a>
<a href="#2891" id="2891">2891</a>
<a href="#2892" id="2892">2892</a>
<a href="#2893" id="2893">2893</a>
<a href="#2894" id="2894">2894</a>
<a href="#2895" id="2895">2895</a>
<a href="#2896" id="2896">2896</a>
<a href="#2897" id="2897">2897</a>
<a href="#2898" id="2898">2898</a>
<a href="#2899" id="2899">2899</a>
<a href="#2900" id="2900">2900</a>
<a href="#2901" id="2901">2901</a>
<a href="#2902" id="2902">2902</a>
<a href="#2903" id="2903">2903</a>
<a href="#2904" id="2904">2904</a>
<a href="#2905" id="2905">2905</a>
<a href="#2906" id="2906">2906</a>
<a href="#2907" id="2907">2907</a>
<a href="#2908" id="2908">2908</a>
<a href="#2909" id="2909">2909</a>
<a href="#2910" id="2910">2910</a>
<a href="#2911" id="2911">2911</a>
<a href="#2912" id="2912">2912</a>
<a href="#2913" id="2913">2913</a>
<a href="#2914" id="2914">2914</a>
<a href="#2915" id="2915">2915</a>
<a href="#2916" id="2916">2916</a>
<a href="#2917" id="2917">2917</a>
<a href="#2918" id="2918">2918</a>
<a href="#2919" id="2919">2919</a>
<a href="#2920" id="2920">2920</a>
<a href="#2921" id="2921">2921</a>
<a href="#2922" id="2922">2922</a>
<a href="#2923" id="2923">2923</a>
<a href="#2924" id="2924">2924</a>
<a href="#2925" id="2925">2925</a>
<a href="#2926" id="2926">2926</a>
<a href="#2927" id="2927">2927</a>
<a href="#2928" id="2928">2928</a>
<a href="#2929" id="2929">2929</a>
<a href="#2930" id="2930">2930</a>
<a href="#2931" id="2931">2931</a>
<a href="#2932" id="2932">2932</a>
<a href="#2933" id="2933">2933</a>
<a href="#2934" id="2934">2934</a>
<a href="#2935" id="2935">2935</a>
<a href="#2936" id="2936">2936</a>
<a href="#2937" id="2937">2937</a>
<a href="#2938" id="2938">2938</a>
<a href="#2939" id="2939">2939</a>
<a href="#2940" id="2940">2940</a>
<a href="#2941" id="2941">2941</a>
<a href="#2942" id="2942">2942</a>
<a href="#2943" id="2943">2943</a>
<a href="#2944" id="2944">2944</a>
<a href="#2945" id="2945">2945</a>
<a href="#2946" id="2946">2946</a>
<a href="#2947" id="2947">2947</a>
<a href="#2948" id="2948">2948</a>
<a href="#2949" id="2949">2949</a>
<a href="#2950" id="2950">2950</a>
<a href="#2951" id="2951">2951</a>
<a href="#2952" id="2952">2952</a>
<a href="#2953" id="2953">2953</a>
<a href="#2954" id="2954">2954</a>
<a href="#2955" id="2955">2955</a>
<a href="#2956" id="2956">2956</a>
<a href="#2957" id="2957">2957</a>
<a href="#2958" id="2958">2958</a>
<a href="#2959" id="2959">2959</a>
<a href="#2960" id="2960">2960</a>
<a href="#2961" id="2961">2961</a>
<a href="#2962" id="2962">2962</a>
<a href="#2963" id="2963">2963</a>
<a href="#2964" id="2964">2964</a>
<a href="#2965" id="2965">2965</a>
<a href="#2966" id="2966">2966</a>
<a href="#2967" id="2967">2967</a>
<a href="#2968" id="2968">2968</a>
<a href="#2969" id="2969">2969</a>
<a href="#2970" id="2970">2970</a>
<a href="#2971" id="2971">2971</a>
<a href="#2972" id="2972">2972</a>
<a href="#2973" id="2973">2973</a>
<a href="#2974" id="2974">2974</a>
<a href="#2975" id="2975">2975</a>
<a href="#2976" id="2976">2976</a>
<a href="#2977" id="2977">2977</a>
<a href="#2978" id="2978">2978</a>
<a href="#2979" id="2979">2979</a>
<a href="#2980" id="2980">2980</a>
<a href="#2981" id="2981">2981</a>
<a href="#2982" id="2982">2982</a>
<a href="#2983" id="2983">2983</a>
<a href="#2984" id="2984">2984</a>
<a href="#2985" id="2985">2985</a>
<a href="#2986" id="2986">2986</a>
<a href="#2987" id="2987">2987</a>
<a href="#2988" id="2988">2988</a>
<a href="#2989" id="2989">2989</a>
<a href="#2990" id="2990">2990</a>
<a href="#2991" id="2991">2991</a>
<a href="#2992" id="2992">2992</a>
<a href="#2993" id="2993">2993</a>
<a href="#2994" id="2994">2994</a>
<a href="#2995" id="2995">2995</a>
<a href="#2996" id="2996">2996</a>
<a href="#2997" id="2997">2997</a>
<a href="#2998" id="2998">2998</a>
<a href="#2999" id="2999">2999</a>
<a href="#3000" id="3000">3000</a>
<a href="#3001" id="3001">3001</a>
<a href="#3002" id="3002">3002</a>
<a href="#3003" id="3003">3003</a>
<a href="#3004" id="3004">3004</a>
<a href="#3005" id="3005">3005</a>
<a href="#3006" id="3006">3006</a>
<a href="#3007" id="3007">3007</a>
<a href="#3008" id="3008">3008</a>
<a href="#3009" id="3009">3009</a>
<a href="#3010" id="3010">3010</a>
<a href="#3011" id="3011">3011</a>
<a href="#3012" id="3012">3012</a>
<a href="#3013" id="3013">3013</a>
<a href="#3014" id="3014">3014</a>
<a href="#3015" id="3015">3015</a>
<a href="#3016" id="3016">3016</a>
<a href="#3017" id="3017">3017</a>
<a href="#3018" id="3018">3018</a>
<a href="#3019" id="3019">3019</a>
<a href="#3020" id="3020">3020</a>
<a href="#3021" id="3021">3021</a>
<a href="#3022" id="3022">3022</a>
<a href="#3023" id="3023">3023</a>
<a href="#3024" id="3024">3024</a>
<a href="#3025" id="3025">3025</a>
<a href="#3026" id="3026">3026</a>
<a href="#3027" id="3027">3027</a>
<a href="#3028" id="3028">3028</a>
<a href="#3029" id="3029">3029</a>
<a href="#3030" id="3030">3030</a>
<a href="#3031" id="3031">3031</a>
<a href="#3032" id="3032">3032</a>
<a href="#3033" id="3033">3033</a>
<a href="#3034" id="3034">3034</a>
<a href="#3035" id="3035">3035</a>
<a href="#3036" id="3036">3036</a>
<a href="#3037" id="3037">3037</a>
<a href="#3038" id="3038">3038</a>
<a href="#3039" id="3039">3039</a>
<a href="#3040" id="3040">3040</a>
<a href="#3041" id="3041">3041</a>
<a href="#3042" id="3042">3042</a>
<a href="#3043" id="3043">3043</a>
<a href="#3044" id="3044">3044</a>
<a href="#3045" id="3045">3045</a>
<a href="#3046" id="3046">3046</a>
<a href="#3047" id="3047">3047</a>
<a href="#3048" id="3048">3048</a>
<a href="#3049" id="3049">3049</a>
<a href="#3050" id="3050">3050</a>
<a href="#3051" id="3051">3051</a>
<a href="#3052" id="3052">3052</a>
<a href="#3053" id="3053">3053</a>
<a href="#3054" id="3054">3054</a>
<a href="#3055" id="3055">3055</a>
<a href="#3056" id="3056">3056</a>
<a href="#3057" id="3057">3057</a>
<a href="#3058" id="3058">3058</a>
<a href="#3059" id="3059">3059</a>
<a href="#3060" id="3060">3060</a>
<a href="#3061" id="3061">3061</a>
<a href="#3062" id="3062">3062</a>
<a href="#3063" id="3063">3063</a>
<a href="#3064" id="3064">3064</a>
<a href="#3065" id="3065">3065</a>
<a href="#3066" id="3066">3066</a>
<a href="#3067" id="3067">3067</a>
<a href="#3068" id="3068">3068</a>
<a href="#3069" id="3069">3069</a>
<a href="#3070" id="3070">3070</a>
<a href="#3071" id="3071">3071</a>
<a href="#3072" id="3072">3072</a>
<a href="#3073" id="3073">3073</a>
<a href="#3074" id="3074">3074</a>
<a href="#3075" id="3075">3075</a>
<a href="#3076" id="3076">3076</a>
<a href="#3077" id="3077">3077</a>
<a href="#3078" id="3078">3078</a>
<a href="#3079" id="3079">3079</a>
<a href="#3080" id="3080">3080</a>
<a href="#3081" id="3081">3081</a>
<a href="#3082" id="3082">3082</a>
<a href="#3083" id="3083">3083</a>
<a href="#3084" id="3084">3084</a>
<a href="#3085" id="3085">3085</a>
<a href="#3086" id="3086">3086</a>
<a href="#3087" id="3087">3087</a>
<a href="#3088" id="3088">3088</a>
<a href="#3089" id="3089">3089</a>
<a href="#3090" id="3090">3090</a>
<a href="#3091" id="3091">3091</a>
<a href="#3092" id="3092">3092</a>
<a href="#3093" id="3093">3093</a>
<a href="#3094" id="3094">3094</a>
<a href="#3095" id="3095">3095</a>
<a href="#3096" id="3096">3096</a>
<a href="#3097" id="3097">3097</a>
<a href="#3098" id="3098">3098</a>
<a href="#3099" id="3099">3099</a>
<a href="#3100" id="3100">3100</a>
<a href="#3101" id="3101">3101</a>
<a href="#3102" id="3102">3102</a>
<a href="#3103" id="3103">3103</a>
<a href="#3104" id="3104">3104</a>
<a href="#3105" id="3105">3105</a>
<a href="#3106" id="3106">3106</a>
<a href="#3107" id="3107">3107</a>
<a href="#3108" id="3108">3108</a>
<a href="#3109" id="3109">3109</a>
<a href="#3110" id="3110">3110</a>
<a href="#3111" id="3111">3111</a>
<a href="#3112" id="3112">3112</a>
<a href="#3113" id="3113">3113</a>
<a href="#3114" id="3114">3114</a>
<a href="#3115" id="3115">3115</a>
<a href="#3116" id="3116">3116</a>
<a href="#3117" id="3117">3117</a>
<a href="#3118" id="3118">3118</a>
<a href="#3119" id="3119">3119</a>
<a href="#3120" id="3120">3120</a>
<a href="#3121" id="3121">3121</a>
<a href="#3122" id="3122">3122</a>
<a href="#3123" id="3123">3123</a>
<a href="#3124" id="3124">3124</a>
<a href="#3125" id="3125">3125</a>
<a href="#3126" id="3126">3126</a>
<a href="#3127" id="3127">3127</a>
<a href="#3128" id="3128">3128</a>
<a href="#3129" id="3129">3129</a>
<a href="#3130" id="3130">3130</a>
<a href="#3131" id="3131">3131</a>
<a href="#3132" id="3132">3132</a>
<a href="#3133" id="3133">3133</a>
<a href="#3134" id="3134">3134</a>
<a href="#3135" id="3135">3135</a>
<a href="#3136" id="3136">3136</a>
<a href="#3137" id="3137">3137</a>
<a href="#3138" id="3138">3138</a>
<a href="#3139" id="3139">3139</a>
<a href="#3140" id="3140">3140</a>
<a href="#3141" id="3141">3141</a>
<a href="#3142" id="3142">3142</a>
<a href="#3143" id="3143">3143</a>
<a href="#3144" id="3144">3144</a>
<a href="#3145" id="3145">3145</a>
<a href="#3146" id="3146">3146</a>
<a href="#3147" id="3147">3147</a>
<a href="#3148" id="3148">3148</a>
<a href="#3149" id="3149">3149</a>
<a href="#3150" id="3150">3150</a>
<a href="#3151" id="3151">3151</a>
<a href="#3152" id="3152">3152</a>
<a href="#3153" id="3153">3153</a>
<a href="#3154" id="3154">3154</a>
<a href="#3155" id="3155">3155</a>
<a href="#3156" id="3156">3156</a>
<a href="#3157" id="3157">3157</a>
<a href="#3158" id="3158">3158</a>
<a href="#3159" id="3159">3159</a>
<a href="#3160" id="3160">3160</a>
<a href="#3161" id="3161">3161</a>
<a href="#3162" id="3162">3162</a>
<a href="#3163" id="3163">3163</a>
<a href="#3164" id="3164">3164</a>
<a href="#3165" id="3165">3165</a>
<a href="#3166" id="3166">3166</a>
<a href="#3167" id="3167">3167</a>
<a href="#3168" id="3168">3168</a>
<a href="#3169" id="3169">3169</a>
<a href="#3170" id="3170">3170</a>
<a href="#3171" id="3171">3171</a>
<a href="#3172" id="3172">3172</a>
<a href="#3173" id="3173">3173</a>
<a href="#3174" id="3174">3174</a>
<a href="#3175" id="3175">3175</a>
<a href="#3176" id="3176">3176</a>
<a href="#3177" id="3177">3177</a>
<a href="#3178" id="3178">3178</a>
<a href="#3179" id="3179">3179</a>
<a href="#3180" id="3180">3180</a>
<a href="#3181" id="3181">3181</a>
<a href="#3182" id="3182">3182</a>
<a href="#3183" id="3183">3183</a>
<a href="#3184" id="3184">3184</a>
<a href="#3185" id="3185">3185</a>
<a href="#3186" id="3186">3186</a>
<a href="#3187" id="3187">3187</a>
<a href="#3188" id="3188">3188</a>
<a href="#3189" id="3189">3189</a>
<a href="#3190" id="3190">3190</a>
<a href="#3191" id="3191">3191</a>
<a href="#3192" id="3192">3192</a>
<a href="#3193" id="3193">3193</a>
<a href="#3194" id="3194">3194</a>
<a href="#3195" id="3195">3195</a>
<a href="#3196" id="3196">3196</a>
<a href="#3197" id="3197">3197</a>
<a href="#3198" id="3198">3198</a>
<a href="#3199" id="3199">3199</a>
<a href="#3200" id="3200">3200</a>
<a href="#3201" id="3201">3201</a>
<a href="#3202" id="3202">3202</a>
<a href="#3203" id="3203">3203</a>
<a href="#3204" id="3204">3204</a>
<a href="#3205" id="3205">3205</a>
<a href="#3206" id="3206">3206</a>
<a href="#3207" id="3207">3207</a>
<a href="#3208" id="3208">3208</a>
<a href="#3209" id="3209">3209</a>
<a href="#3210" id="3210">3210</a>
<a href="#3211" id="3211">3211</a>
<a href="#3212" id="3212">3212</a>
<a href="#3213" id="3213">3213</a>
<a href="#3214" id="3214">3214</a>
<a href="#3215" id="3215">3215</a>
<a href="#3216" id="3216">3216</a>
<a href="#3217" id="3217">3217</a>
<a href="#3218" id="3218">3218</a>
<a href="#3219" id="3219">3219</a>
<a href="#3220" id="3220">3220</a>
<a href="#3221" id="3221">3221</a>
<a href="#3222" id="3222">3222</a>
<a href="#3223" id="3223">3223</a>
<a href="#3224" id="3224">3224</a>
<a href="#3225" id="3225">3225</a>
<a href="#3226" id="3226">3226</a>
<a href="#3227" id="3227">3227</a>
<a href="#3228" id="3228">3228</a>
<a href="#3229" id="3229">3229</a>
<a href="#3230" id="3230">3230</a>
<a href="#3231" id="3231">3231</a>
<a href="#3232" id="3232">3232</a>
<a href="#3233" id="3233">3233</a>
<a href="#3234" id="3234">3234</a>
<a href="#3235" id="3235">3235</a>
<a href="#3236" id="3236">3236</a>
<a href="#3237" id="3237">3237</a>
<a href="#3238" id="3238">3238</a>
<a href="#3239" id="3239">3239</a>
<a href="#3240" id="3240">3240</a>
<a href="#3241" id="3241">3241</a>
<a href="#3242" id="3242">3242</a>
<a href="#3243" id="3243">3243</a>
<a href="#3244" id="3244">3244</a>
<a href="#3245" id="3245">3245</a>
<a href="#3246" id="3246">3246</a>
<a href="#3247" id="3247">3247</a>
<a href="#3248" id="3248">3248</a>
<a href="#3249" id="3249">3249</a>
<a href="#3250" id="3250">3250</a>
<a href="#3251" id="3251">3251</a>
<a href="#3252" id="3252">3252</a>
<a href="#3253" id="3253">3253</a>
<a href="#3254" id="3254">3254</a>
<a href="#3255" id="3255">3255</a>
<a href="#3256" id="3256">3256</a>
<a href="#3257" id="3257">3257</a>
<a href="#3258" id="3258">3258</a>
<a href="#3259" id="3259">3259</a>
<a href="#3260" id="3260">3260</a>
<a href="#3261" id="3261">3261</a>
<a href="#3262" id="3262">3262</a>
<a href="#3263" id="3263">3263</a>
<a href="#3264" id="3264">3264</a>
<a href="#3265" id="3265">3265</a>
<a href="#3266" id="3266">3266</a>
<a href="#3267" id="3267">3267</a>
<a href="#3268" id="3268">3268</a>
<a href="#3269" id="3269">3269</a>
<a href="#3270" id="3270">3270</a>
<a href="#3271" id="3271">3271</a>
<a href="#3272" id="3272">3272</a>
<a href="#3273" id="3273">3273</a>
<a href="#3274" id="3274">3274</a>
<a href="#3275" id="3275">3275</a>
<a href="#3276" id="3276">3276</a>
<a href="#3277" id="3277">3277</a>
<a href="#3278" id="3278">3278</a>
<a href="#3279" id="3279">3279</a>
<a href="#3280" id="3280">3280</a>
<a href="#3281" id="3281">3281</a>
<a href="#3282" id="3282">3282</a>
<a href="#3283" id="3283">3283</a>
<a href="#3284" id="3284">3284</a>
<a href="#3285" id="3285">3285</a>
<a href="#3286" id="3286">3286</a>
<a href="#3287" id="3287">3287</a>
<a href="#3288" id="3288">3288</a>
<a href="#3289" id="3289">3289</a>
<a href="#3290" id="3290">3290</a>
<a href="#3291" id="3291">3291</a>
<a href="#3292" id="3292">3292</a>
<a href="#3293" id="3293">3293</a>
<a href="#3294" id="3294">3294</a>
<a href="#3295" id="3295">3295</a>
<a href="#3296" id="3296">3296</a>
<a href="#3297" id="3297">3297</a>
<a href="#3298" id="3298">3298</a>
<a href="#3299" id="3299">3299</a>
<a href="#3300" id="3300">3300</a>
<a href="#3301" id="3301">3301</a>
<a href="#3302" id="3302">3302</a>
<a href="#3303" id="3303">3303</a>
<a href="#3304" id="3304">3304</a>
<a href="#3305" id="3305">3305</a>
<a href="#3306" id="3306">3306</a>
<a href="#3307" id="3307">3307</a>
<a href="#3308" id="3308">3308</a>
<a href="#3309" id="3309">3309</a>
<a href="#3310" id="3310">3310</a>
<a href="#3311" id="3311">3311</a>
<a href="#3312" id="3312">3312</a>
<a href="#3313" id="3313">3313</a>
<a href="#3314" id="3314">3314</a>
<a href="#3315" id="3315">3315</a>
<a href="#3316" id="3316">3316</a>
<a href="#3317" id="3317">3317</a>
<a href="#3318" id="3318">3318</a>
<a href="#3319" id="3319">3319</a>
<a href="#3320" id="3320">3320</a>
<a href="#3321" id="3321">3321</a>
<a href="#3322" id="3322">3322</a>
<a href="#3323" id="3323">3323</a>
<a href="#3324" id="3324">3324</a>
<a href="#3325" id="3325">3325</a>
<a href="#3326" id="3326">3326</a>
<a href="#3327" id="3327">3327</a>
<a href="#3328" id="3328">3328</a>
<a href="#3329" id="3329">3329</a>
<a href="#3330" id="3330">3330</a>
<a href="#3331" id="3331">3331</a>
<a href="#3332" id="3332">3332</a>
<a href="#3333" id="3333">3333</a>
<a href="#3334" id="3334">3334</a>
<a href="#3335" id="3335">3335</a>
<a href="#3336" id="3336">3336</a>
<a href="#3337" id="3337">3337</a>
<a href="#3338" id="3338">3338</a>
<a href="#3339" id="3339">3339</a>
<a href="#3340" id="3340">3340</a>
<a href="#3341" id="3341">3341</a>
<a href="#3342" id="3342">3342</a>
<a href="#3343" id="3343">3343</a>
<a href="#3344" id="3344">3344</a>
<a href="#3345" id="3345">3345</a>
<a href="#3346" id="3346">3346</a>
<a href="#3347" id="3347">3347</a>
<a href="#3348" id="3348">3348</a>
<a href="#3349" id="3349">3349</a>
<a href="#3350" id="3350">3350</a>
<a href="#3351" id="3351">3351</a>
<a href="#3352" id="3352">3352</a>
<a href="#3353" id="3353">3353</a>
<a href="#3354" id="3354">3354</a>
<a href="#3355" id="3355">3355</a>
<a href="#3356" id="3356">3356</a>
<a href="#3357" id="3357">3357</a>
<a href="#3358" id="3358">3358</a>
<a href="#3359" id="3359">3359</a>
<a href="#3360" id="3360">3360</a>
<a href="#3361" id="3361">3361</a>
<a href="#3362" id="3362">3362</a>
<a href="#3363" id="3363">3363</a>
<a href="#3364" id="3364">3364</a>
<a href="#3365" id="3365">3365</a>
<a href="#3366" id="3366">3366</a>
<a href="#3367" id="3367">3367</a>
<a href="#3368" id="3368">3368</a>
<a href="#3369" id="3369">3369</a>
<a href="#3370" id="3370">3370</a>
<a href="#3371" id="3371">3371</a>
<a href="#3372" id="3372">3372</a>
<a href="#3373" id="3373">3373</a>
<a href="#3374" id="3374">3374</a>
<a href="#3375" id="3375">3375</a>
<a href="#3376" id="3376">3376</a>
<a href="#3377" id="3377">3377</a>
<a href="#3378" id="3378">3378</a>
<a href="#3379" id="3379">3379</a>
<a href="#3380" id="3380">3380</a>
<a href="#3381" id="3381">3381</a>
<a href="#3382" id="3382">3382</a>
<a href="#3383" id="3383">3383</a>
<a href="#3384" id="3384">3384</a>
<a href="#3385" id="3385">3385</a>
<a href="#3386" id="3386">3386</a>
<a href="#3387" id="3387">3387</a>
<a href="#3388" id="3388">3388</a>
<a href="#3389" id="3389">3389</a>
<a href="#3390" id="3390">3390</a>
<a href="#3391" id="3391">3391</a>
<a href="#3392" id="3392">3392</a>
<a href="#3393" id="3393">3393</a>
<a href="#3394" id="3394">3394</a>
<a href="#3395" id="3395">3395</a>
<a href="#3396" id="3396">3396</a>
<a href="#3397" id="3397">3397</a>
<a href="#3398" id="3398">3398</a>
<a href="#3399" id="3399">3399</a>
<a href="#3400" id="3400">3400</a>
<a href="#3401" id="3401">3401</a>
<a href="#3402" id="3402">3402</a>
<a href="#3403" id="3403">3403</a>
<a href="#3404" id="3404">3404</a>
<a href="#3405" id="3405">3405</a>
<a href="#3406" id="3406">3406</a>
<a href="#3407" id="3407">3407</a>
<a href="#3408" id="3408">3408</a>
<a href="#3409" id="3409">3409</a>
<a href="#3410" id="3410">3410</a>
<a href="#3411" id="3411">3411</a>
<a href="#3412" id="3412">3412</a>
<a href="#3413" id="3413">3413</a>
<a href="#3414" id="3414">3414</a>
<a href="#3415" id="3415">3415</a>
<a href="#3416" id="3416">3416</a>
<a href="#3417" id="3417">3417</a>
<a href="#3418" id="3418">3418</a>
<a href="#3419" id="3419">3419</a>
<a href="#3420" id="3420">3420</a>
<a href="#3421" id="3421">3421</a>
<a href="#3422" id="3422">3422</a>
<a href="#3423" id="3423">3423</a>
<a href="#3424" id="3424">3424</a>
<a href="#3425" id="3425">3425</a>
<a href="#3426" id="3426">3426</a>
<a href="#3427" id="3427">3427</a>
<a href="#3428" id="3428">3428</a>
<a href="#3429" id="3429">3429</a>
<a href="#3430" id="3430">3430</a>
<a href="#3431" id="3431">3431</a>
<a href="#3432" id="3432">3432</a>
<a href="#3433" id="3433">3433</a>
<a href="#3434" id="3434">3434</a>
<a href="#3435" id="3435">3435</a>
<a href="#3436" id="3436">3436</a>
<a href="#3437" id="3437">3437</a>
<a href="#3438" id="3438">3438</a>
<a href="#3439" id="3439">3439</a>
<a href="#3440" id="3440">3440</a>
<a href="#3441" id="3441">3441</a>
<a href="#3442" id="3442">3442</a>
<a href="#3443" id="3443">3443</a>
<a href="#3444" id="3444">3444</a>
<a href="#3445" id="3445">3445</a>
<a href="#3446" id="3446">3446</a>
<a href="#3447" id="3447">3447</a>
<a href="#3448" id="3448">3448</a>
<a href="#3449" id="3449">3449</a>
<a href="#3450" id="3450">3450</a>
<a href="#3451" id="3451">3451</a>
<a href="#3452" id="3452">3452</a>
<a href="#3453" id="3453">3453</a>
<a href="#3454" id="3454">3454</a>
<a href="#3455" id="3455">3455</a>
<a href="#3456" id="3456">3456</a>
<a href="#3457" id="3457">3457</a>
<a href="#3458" id="3458">3458</a>
<a href="#3459" id="3459">3459</a>
<a href="#3460" id="3460">3460</a>
<a href="#3461" id="3461">3461</a>
<a href="#3462" id="3462">3462</a>
<a href="#3463" id="3463">3463</a>
<a href="#3464" id="3464">3464</a>
<a href="#3465" id="3465">3465</a>
<a href="#3466" id="3466">3466</a>
<a href="#3467" id="3467">3467</a>
<a href="#3468" id="3468">3468</a>
<a href="#3469" id="3469">3469</a>
<a href="#3470" id="3470">3470</a>
<a href="#3471" id="3471">3471</a>
<a href="#3472" id="3472">3472</a>
<a href="#3473" id="3473">3473</a>
<a href="#3474" id="3474">3474</a>
<a href="#3475" id="3475">3475</a>
<a href="#3476" id="3476">3476</a>
<a href="#3477" id="3477">3477</a>
<a href="#3478" id="3478">3478</a>
<a href="#3479" id="3479">3479</a>
<a href="#3480" id="3480">3480</a>
<a href="#3481" id="3481">3481</a>
<a href="#3482" id="3482">3482</a>
<a href="#3483" id="3483">3483</a>
<a href="#3484" id="3484">3484</a>
<a href="#3485" id="3485">3485</a>
<a href="#3486" id="3486">3486</a>
<a href="#3487" id="3487">3487</a>
<a href="#3488" id="3488">3488</a>
<a href="#3489" id="3489">3489</a>
<a href="#3490" id="3490">3490</a>
<a href="#3491" id="3491">3491</a>
<a href="#3492" id="3492">3492</a>
<a href="#3493" id="3493">3493</a>
<a href="#3494" id="3494">3494</a>
<a href="#3495" id="3495">3495</a>
<a href="#3496" id="3496">3496</a>
<a href="#3497" id="3497">3497</a>
<a href="#3498" id="3498">3498</a>
<a href="#3499" id="3499">3499</a>
<a href="#3500" id="3500">3500</a>
<a href="#3501" id="3501">3501</a>
<a href="#3502" id="3502">3502</a>
<a href="#3503" id="3503">3503</a>
<a href="#3504" id="3504">3504</a>
<a href="#3505" id="3505">3505</a>
<a href="#3506" id="3506">3506</a>
<a href="#3507" id="3507">3507</a>
<a href="#3508" id="3508">3508</a>
<a href="#3509" id="3509">3509</a>
<a href="#3510" id="3510">3510</a>
<a href="#3511" id="3511">3511</a>
<a href="#3512" id="3512">3512</a>
<a href="#3513" id="3513">3513</a>
<a href="#3514" id="3514">3514</a>
<a href="#3515" id="3515">3515</a>
<a href="#3516" id="3516">3516</a>
<a href="#3517" id="3517">3517</a>
<a href="#3518" id="3518">3518</a>
<a href="#3519" id="3519">3519</a>
<a href="#3520" id="3520">3520</a>
<a href="#3521" id="3521">3521</a>
<a href="#3522" id="3522">3522</a>
<a href="#3523" id="3523">3523</a>
<a href="#3524" id="3524">3524</a>
<a href="#3525" id="3525">3525</a>
<a href="#3526" id="3526">3526</a>
<a href="#3527" id="3527">3527</a>
<a href="#3528" id="3528">3528</a>
<a href="#3529" id="3529">3529</a>
<a href="#3530" id="3530">3530</a>
<a href="#3531" id="3531">3531</a>
<a href="#3532" id="3532">3532</a>
<a href="#3533" id="3533">3533</a>
<a href="#3534" id="3534">3534</a>
<a href="#3535" id="3535">3535</a>
<a href="#3536" id="3536">3536</a>
<a href="#3537" id="3537">3537</a>
<a href="#3538" id="3538">3538</a>
<a href="#3539" id="3539">3539</a>
<a href="#3540" id="3540">3540</a>
<a href="#3541" id="3541">3541</a>
<a href="#3542" id="3542">3542</a>
<a href="#3543" id="3543">3543</a>
<a href="#3544" id="3544">3544</a>
<a href="#3545" id="3545">3545</a>
<a href="#3546" id="3546">3546</a>
<a href="#3547" id="3547">3547</a>
<a href="#3548" id="3548">3548</a>
<a href="#3549" id="3549">3549</a>
<a href="#3550" id="3550">3550</a>
<a href="#3551" id="3551">3551</a>
<a href="#3552" id="3552">3552</a>
<a href="#3553" id="3553">3553</a>
<a href="#3554" id="3554">3554</a>
<a href="#3555" id="3555">3555</a>
<a href="#3556" id="3556">3556</a>
<a href="#3557" id="3557">3557</a>
<a href="#3558" id="3558">3558</a>
<a href="#3559" id="3559">3559</a>
<a href="#3560" id="3560">3560</a>
<a href="#3561" id="3561">3561</a>
<a href="#3562" id="3562">3562</a>
<a href="#3563" id="3563">3563</a>
<a href="#3564" id="3564">3564</a>
<a href="#3565" id="3565">3565</a>
<a href="#3566" id="3566">3566</a>
<a href="#3567" id="3567">3567</a>
<a href="#3568" id="3568">3568</a>
<a href="#3569" id="3569">3569</a>
<a href="#3570" id="3570">3570</a>
<a href="#3571" id="3571">3571</a>
<a href="#3572" id="3572">3572</a>
<a href="#3573" id="3573">3573</a>
<a href="#3574" id="3574">3574</a>
<a href="#3575" id="3575">3575</a>
<a href="#3576" id="3576">3576</a>
<a href="#3577" id="3577">3577</a>
<a href="#3578" id="3578">3578</a>
<a href="#3579" id="3579">3579</a>
<a href="#3580" id="3580">3580</a>
<a href="#3581" id="3581">3581</a>
<a href="#3582" id="3582">3582</a>
<a href="#3583" id="3583">3583</a>
<a href="#3584" id="3584">3584</a>
<a href="#3585" id="3585">3585</a>
<a href="#3586" id="3586">3586</a>
<a href="#3587" id="3587">3587</a>
<a href="#3588" id="3588">3588</a>
<a href="#3589" id="3589">3589</a>
<a href="#3590" id="3590">3590</a>
<a href="#3591" id="3591">3591</a>
<a href="#3592" id="3592">3592</a>
<a href="#3593" id="3593">3593</a>
<a href="#3594" id="3594">3594</a>
<a href="#3595" id="3595">3595</a>
<a href="#3596" id="3596">3596</a>
<a href="#3597" id="3597">3597</a>
<a href="#3598" id="3598">3598</a>
<a href="#3599" id="3599">3599</a>
<a href="#3600" id="3600">3600</a>
<a href="#3601" id="3601">3601</a>
<a href="#3602" id="3602">3602</a>
<a href="#3603" id="3603">3603</a>
<a href="#3604" id="3604">3604</a>
<a href="#3605" id="3605">3605</a>
<a href="#3606" id="3606">3606</a>
<a href="#3607" id="3607">3607</a>
<a href="#3608" id="3608">3608</a>
<a href="#3609" id="3609">3609</a>
<a href="#3610" id="3610">3610</a>
<a href="#3611" id="3611">3611</a>
<a href="#3612" id="3612">3612</a>
<a href="#3613" id="3613">3613</a>
<a href="#3614" id="3614">3614</a>
<a href="#3615" id="3615">3615</a>
<a href="#3616" id="3616">3616</a>
<a href="#3617" id="3617">3617</a>
<a href="#3618" id="3618">3618</a>
<a href="#3619" id="3619">3619</a>
<a href="#3620" id="3620">3620</a>
<a href="#3621" id="3621">3621</a>
<a href="#3622" id="3622">3622</a>
<a href="#3623" id="3623">3623</a>
<a href="#3624" id="3624">3624</a>
<a href="#3625" id="3625">3625</a>
<a href="#3626" id="3626">3626</a>
<a href="#3627" id="3627">3627</a>
<a href="#3628" id="3628">3628</a>
<a href="#3629" id="3629">3629</a>
<a href="#3630" id="3630">3630</a>
<a href="#3631" id="3631">3631</a>
<a href="#3632" id="3632">3632</a>
<a href="#3633" id="3633">3633</a>
<a href="#3634" id="3634">3634</a>
<a href="#3635" id="3635">3635</a>
<a href="#3636" id="3636">3636</a>
<a href="#3637" id="3637">3637</a>
<a href="#3638" id="3638">3638</a>
<a href="#3639" id="3639">3639</a>
<a href="#3640" id="3640">3640</a>
<a href="#3641" id="3641">3641</a>
<a href="#3642" id="3642">3642</a>
<a href="#3643" id="3643">3643</a>
<a href="#3644" id="3644">3644</a>
<a href="#3645" id="3645">3645</a>
<a href="#3646" id="3646">3646</a>
<a href="#3647" id="3647">3647</a>
<a href="#3648" id="3648">3648</a>
<a href="#3649" id="3649">3649</a>
<a href="#3650" id="3650">3650</a>
<a href="#3651" id="3651">3651</a>
<a href="#3652" id="3652">3652</a>
<a href="#3653" id="3653">3653</a>
<a href="#3654" id="3654">3654</a>
<a href="#3655" id="3655">3655</a>
<a href="#3656" id="3656">3656</a>
<a href="#3657" id="3657">3657</a>
<a href="#3658" id="3658">3658</a>
<a href="#3659" id="3659">3659</a>
<a href="#3660" id="3660">3660</a>
<a href="#3661" id="3661">3661</a>
<a href="#3662" id="3662">3662</a>
<a href="#3663" id="3663">3663</a>
<a href="#3664" id="3664">3664</a>
<a href="#3665" id="3665">3665</a>
<a href="#3666" id="3666">3666</a>
<a href="#3667" id="3667">3667</a>
<a href="#3668" id="3668">3668</a>
<a href="#3669" id="3669">3669</a>
<a href="#3670" id="3670">3670</a>
<a href="#3671" id="3671">3671</a>
<a href="#3672" id="3672">3672</a>
<a href="#3673" id="3673">3673</a>
<a href="#3674" id="3674">3674</a>
<a href="#3675" id="3675">3675</a>
<a href="#3676" id="3676">3676</a>
<a href="#3677" id="3677">3677</a>
<a href="#3678" id="3678">3678</a>
<a href="#3679" id="3679">3679</a>
<a href="#3680" id="3680">3680</a>
<a href="#3681" id="3681">3681</a>
<a href="#3682" id="3682">3682</a>
<a href="#3683" id="3683">3683</a>
<a href="#3684" id="3684">3684</a>
<a href="#3685" id="3685">3685</a>
<a href="#3686" id="3686">3686</a>
<a href="#3687" id="3687">3687</a>
<a href="#3688" id="3688">3688</a>
<a href="#3689" id="3689">3689</a>
<a href="#3690" id="3690">3690</a>
<a href="#3691" id="3691">3691</a>
<a href="#3692" id="3692">3692</a>
<a href="#3693" id="3693">3693</a>
<a href="#3694" id="3694">3694</a>
<a href="#3695" id="3695">3695</a>
<a href="#3696" id="3696">3696</a>
<a href="#3697" id="3697">3697</a>
<a href="#3698" id="3698">3698</a>
<a href="#3699" id="3699">3699</a>
<a href="#3700" id="3700">3700</a>
<a href="#3701" id="3701">3701</a>
<a href="#3702" id="3702">3702</a>
<a href="#3703" id="3703">3703</a>
<a href="#3704" id="3704">3704</a>
<a href="#3705" id="3705">3705</a>
<a href="#3706" id="3706">3706</a>
<a href="#3707" id="3707">3707</a>
<a href="#3708" id="3708">3708</a>
<a href="#3709" id="3709">3709</a>
<a href="#3710" id="3710">3710</a>
<a href="#3711" id="3711">3711</a>
<a href="#3712" id="3712">3712</a>
<a href="#3713" id="3713">3713</a>
<a href="#3714" id="3714">3714</a>
<a href="#3715" id="3715">3715</a>
<a href="#3716" id="3716">3716</a>
<a href="#3717" id="3717">3717</a>
<a href="#3718" id="3718">3718</a>
<a href="#3719" id="3719">3719</a>
<a href="#3720" id="3720">3720</a>
<a href="#3721" id="3721">3721</a>
<a href="#3722" id="3722">3722</a>
<a href="#3723" id="3723">3723</a>
<a href="#3724" id="3724">3724</a>
<a href="#3725" id="3725">3725</a>
<a href="#3726" id="3726">3726</a>
<a href="#3727" id="3727">3727</a>
<a href="#3728" id="3728">3728</a>
<a href="#3729" id="3729">3729</a>
<a href="#3730" id="3730">3730</a>
<a href="#3731" id="3731">3731</a>
<a href="#3732" id="3732">3732</a>
<a href="#3733" id="3733">3733</a>
<a href="#3734" id="3734">3734</a>
<a href="#3735" id="3735">3735</a>
<a href="#3736" id="3736">3736</a>
<a href="#3737" id="3737">3737</a>
<a href="#3738" id="3738">3738</a>
<a href="#3739" id="3739">3739</a>
<a href="#3740" id="3740">3740</a>
<a href="#3741" id="3741">3741</a>
<a href="#3742" id="3742">3742</a>
<a href="#3743" id="3743">3743</a>
<a href="#3744" id="3744">3744</a>
<a href="#3745" id="3745">3745</a>
<a href="#3746" id="3746">3746</a>
<a href="#3747" id="3747">3747</a>
<a href="#3748" id="3748">3748</a>
<a href="#3749" id="3749">3749</a>
<a href="#3750" id="3750">3750</a>
<a href="#3751" id="3751">3751</a>
<a href="#3752" id="3752">3752</a>
<a href="#3753" id="3753">3753</a>
<a href="#3754" id="3754">3754</a>
<a href="#3755" id="3755">3755</a>
<a href="#3756" id="3756">3756</a>
<a href="#3757" id="3757">3757</a>
<a href="#3758" id="3758">3758</a>
<a href="#3759" id="3759">3759</a>
<a href="#3760" id="3760">3760</a>
<a href="#3761" id="3761">3761</a>
<a href="#3762" id="3762">3762</a>
<a href="#3763" id="3763">3763</a>
<a href="#3764" id="3764">3764</a>
<a href="#3765" id="3765">3765</a>
<a href="#3766" id="3766">3766</a>
<a href="#3767" id="3767">3767</a>
<a href="#3768" id="3768">3768</a>
<a href="#3769" id="3769">3769</a>
<a href="#3770" id="3770">3770</a>
<a href="#3771" id="3771">3771</a>
<a href="#3772" id="3772">3772</a>
<a href="#3773" id="3773">3773</a>
<a href="#3774" id="3774">3774</a>
<a href="#3775" id="3775">3775</a>
<a href="#3776" id="3776">3776</a>
<a href="#3777" id="3777">3777</a>
<a href="#3778" id="3778">3778</a>
<a href="#3779" id="3779">3779</a>
<a href="#3780" id="3780">3780</a>
<a href="#3781" id="3781">3781</a>
<a href="#3782" id="3782">3782</a>
<a href="#3783" id="3783">3783</a>
<a href="#3784" id="3784">3784</a>
<a href="#3785" id="3785">3785</a>
<a href="#3786" id="3786">3786</a>
<a href="#3787" id="3787">3787</a>
<a href="#3788" id="3788">3788</a>
<a href="#3789" id="3789">3789</a>
<a href="#3790" id="3790">3790</a>
<a href="#3791" id="3791">3791</a>
<a href="#3792" id="3792">3792</a>
<a href="#3793" id="3793">3793</a>
<a href="#3794" id="3794">3794</a>
<a href="#3795" id="3795">3795</a>
<a href="#3796" id="3796">3796</a>
<a href="#3797" id="3797">3797</a>
<a href="#3798" id="3798">3798</a>
<a href="#3799" id="3799">3799</a>
<a href="#3800" id="3800">3800</a>
<a href="#3801" id="3801">3801</a>
<a href="#3802" id="3802">3802</a>
<a href="#3803" id="3803">3803</a>
<a href="#3804" id="3804">3804</a>
<a href="#3805" id="3805">3805</a>
<a href="#3806" id="3806">3806</a>
<a href="#3807" id="3807">3807</a>
<a href="#3808" id="3808">3808</a>
<a href="#3809" id="3809">3809</a>
<a href="#3810" id="3810">3810</a>
<a href="#3811" id="3811">3811</a>
<a href="#3812" id="3812">3812</a>
<a href="#3813" id="3813">3813</a>
<a href="#3814" id="3814">3814</a>
<a href="#3815" id="3815">3815</a>
<a href="#3816" id="3816">3816</a>
<a href="#3817" id="3817">3817</a>
<a href="#3818" id="3818">3818</a>
<a href="#3819" id="3819">3819</a>
<a href="#3820" id="3820">3820</a>
<a href="#3821" id="3821">3821</a>
<a href="#3822" id="3822">3822</a>
<a href="#3823" id="3823">3823</a>
<a href="#3824" id="3824">3824</a>
<a href="#3825" id="3825">3825</a>
<a href="#3826" id="3826">3826</a>
<a href="#3827" id="3827">3827</a>
<a href="#3828" id="3828">3828</a>
<a href="#3829" id="3829">3829</a>
<a href="#3830" id="3830">3830</a>
<a href="#3831" id="3831">3831</a>
<a href="#3832" id="3832">3832</a>
<a href="#3833" id="3833">3833</a>
<a href="#3834" id="3834">3834</a>
<a href="#3835" id="3835">3835</a>
<a href="#3836" id="3836">3836</a>
<a href="#3837" id="3837">3837</a>
<a href="#3838" id="3838">3838</a>
<a href="#3839" id="3839">3839</a>
<a href="#3840" id="3840">3840</a>
<a href="#3841" id="3841">3841</a>
<a href="#3842" id="3842">3842</a>
<a href="#3843" id="3843">3843</a>
<a href="#3844" id="3844">3844</a>
<a href="#3845" id="3845">3845</a>
<a href="#3846" id="3846">3846</a>
<a href="#3847" id="3847">3847</a>
<a href="#3848" id="3848">3848</a>
<a href="#3849" id="3849">3849</a>
<a href="#3850" id="3850">3850</a>
<a href="#3851" id="3851">3851</a>
<a href="#3852" id="3852">3852</a>
<a href="#3853" id="3853">3853</a>
<a href="#3854" id="3854">3854</a>
<a href="#3855" id="3855">3855</a>
<a href="#3856" id="3856">3856</a>
<a href="#3857" id="3857">3857</a>
<a href="#3858" id="3858">3858</a>
<a href="#3859" id="3859">3859</a>
<a href="#3860" id="3860">3860</a>
<a href="#3861" id="3861">3861</a>
<a href="#3862" id="3862">3862</a>
<a href="#3863" id="3863">3863</a>
<a href="#3864" id="3864">3864</a>
<a href="#3865" id="3865">3865</a>
<a href="#3866" id="3866">3866</a>
<a href="#3867" id="3867">3867</a>
<a href="#3868" id="3868">3868</a>
<a href="#3869" id="3869">3869</a>
<a href="#3870" id="3870">3870</a>
<a href="#3871" id="3871">3871</a>
<a href="#3872" id="3872">3872</a>
<a href="#3873" id="3873">3873</a>
<a href="#3874" id="3874">3874</a>
<a href="#3875" id="3875">3875</a>
<a href="#3876" id="3876">3876</a>
<a href="#3877" id="3877">3877</a>
<a href="#3878" id="3878">3878</a>
<a href="#3879" id="3879">3879</a>
<a href="#3880" id="3880">3880</a>
<a href="#3881" id="3881">3881</a>
<a href="#3882" id="3882">3882</a>
<a href="#3883" id="3883">3883</a>
<a href="#3884" id="3884">3884</a>
<a href="#3885" id="3885">3885</a>
<a href="#3886" id="3886">3886</a>
<a href="#3887" id="3887">3887</a>
<a href="#3888" id="3888">3888</a>
<a href="#3889" id="3889">3889</a>
<a href="#3890" id="3890">3890</a>
<a href="#3891" id="3891">3891</a>
<a href="#3892" id="3892">3892</a>
<a href="#3893" id="3893">3893</a>
<a href="#3894" id="3894">3894</a>
<a href="#3895" id="3895">3895</a>
<a href="#3896" id="3896">3896</a>
<a href="#3897" id="3897">3897</a>
<a href="#3898" id="3898">3898</a>
<a href="#3899" id="3899">3899</a>
<a href="#3900" id="3900">3900</a>
<a href="#3901" id="3901">3901</a>
<a href="#3902" id="3902">3902</a>
<a href="#3903" id="3903">3903</a>
<a href="#3904" id="3904">3904</a>
<a href="#3905" id="3905">3905</a>
<a href="#3906" id="3906">3906</a>
<a href="#3907" id="3907">3907</a>
<a href="#3908" id="3908">3908</a>
<a href="#3909" id="3909">3909</a>
<a href="#3910" id="3910">3910</a>
<a href="#3911" id="3911">3911</a>
<a href="#3912" id="3912">3912</a>
<a href="#3913" id="3913">3913</a>
<a href="#3914" id="3914">3914</a>
<a href="#3915" id="3915">3915</a>
<a href="#3916" id="3916">3916</a>
<a href="#3917" id="3917">3917</a>
<a href="#3918" id="3918">3918</a>
<a href="#3919" id="3919">3919</a>
<a href="#3920" id="3920">3920</a>
<a href="#3921" id="3921">3921</a>
<a href="#3922" id="3922">3922</a>
<a href="#3923" id="3923">3923</a>
<a href="#3924" id="3924">3924</a>
<a href="#3925" id="3925">3925</a>
<a href="#3926" id="3926">3926</a>
<a href="#3927" id="3927">3927</a>
<a href="#3928" id="3928">3928</a>
<a href="#3929" id="3929">3929</a>
<a href="#3930" id="3930">3930</a>
<a href="#3931" id="3931">3931</a>
<a href="#3932" id="3932">3932</a>
<a href="#3933" id="3933">3933</a>
<a href="#3934" id="3934">3934</a>
<a href="#3935" id="3935">3935</a>
<a href="#3936" id="3936">3936</a>
<a href="#3937" id="3937">3937</a>
<a href="#3938" id="3938">3938</a>
<a href="#3939" id="3939">3939</a>
<a href="#3940" id="3940">3940</a>
<a href="#3941" id="3941">3941</a>
<a href="#3942" id="3942">3942</a>
<a href="#3943" id="3943">3943</a>
<a href="#3944" id="3944">3944</a>
<a href="#3945" id="3945">3945</a>
<a href="#3946" id="3946">3946</a>
<a href="#3947" id="3947">3947</a>
<a href="#3948" id="3948">3948</a>
<a href="#3949" id="3949">3949</a>
<a href="#3950" id="3950">3950</a>
<a href="#3951" id="3951">3951</a>
<a href="#3952" id="3952">3952</a>
<a href="#3953" id="3953">3953</a>
<a href="#3954" id="3954">3954</a>
<a href="#3955" id="3955">3955</a>
<a href="#3956" id="3956">3956</a>
<a href="#3957" id="3957">3957</a>
<a href="#3958" id="3958">3958</a>
<a href="#3959" id="3959">3959</a>
<a href="#3960" id="3960">3960</a>
<a href="#3961" id="3961">3961</a>
<a href="#3962" id="3962">3962</a>
<a href="#3963" id="3963">3963</a>
<a href="#3964" id="3964">3964</a>
<a href="#3965" id="3965">3965</a>
<a href="#3966" id="3966">3966</a>
<a href="#3967" id="3967">3967</a>
<a href="#3968" id="3968">3968</a>
<a href="#3969" id="3969">3969</a>
<a href="#3970" id="3970">3970</a>
<a href="#3971" id="3971">3971</a>
<a href="#3972" id="3972">3972</a>
<a href="#3973" id="3973">3973</a>
<a href="#3974" id="3974">3974</a>
<a href="#3975" id="3975">3975</a>
<a href="#3976" id="3976">3976</a>
<a href="#3977" id="3977">3977</a>
<a href="#3978" id="3978">3978</a>
<a href="#3979" id="3979">3979</a>
<a href="#3980" id="3980">3980</a>
<a href="#3981" id="3981">3981</a>
<a href="#3982" id="3982">3982</a>
<a href="#3983" id="3983">3983</a>
<a href="#3984" id="3984">3984</a>
<a href="#3985" id="3985">3985</a>
<a href="#3986" id="3986">3986</a>
<a href="#3987" id="3987">3987</a>
<a href="#3988" id="3988">3988</a>
<a href="#3989" id="3989">3989</a>
<a href="#3990" id="3990">3990</a>
<a href="#3991" id="3991">3991</a>
<a href="#3992" id="3992">3992</a>
<a href="#3993" id="3993">3993</a>
<a href="#3994" id="3994">3994</a>
<a href="#3995" id="3995">3995</a>
<a href="#3996" id="3996">3996</a>
<a href="#3997" id="3997">3997</a>
<a href="#3998" id="3998">3998</a>
<a href="#3999" id="3999">3999</a>
<a href="#4000" id="4000">4000</a>
<a href="#4001" id="4001">4001</a>
<a href="#4002" id="4002">4002</a>
<a href="#4003" id="4003">4003</a>
<a href="#4004" id="4004">4004</a>
<a href="#4005" id="4005">4005</a>
<a href="#4006" id="4006">4006</a>
<a href="#4007" id="4007">4007</a>
<a href="#4008" id="4008">4008</a>
<a href="#4009" id="4009">4009</a>
<a href="#4010" id="4010">4010</a>
<a href="#4011" id="4011">4011</a>
<a href="#4012" id="4012">4012</a>
<a href="#4013" id="4013">4013</a>
<a href="#4014" id="4014">4014</a>
<a href="#4015" id="4015">4015</a>
<a href="#4016" id="4016">4016</a>
<a href="#4017" id="4017">4017</a>
<a href="#4018" id="4018">4018</a>
<a href="#4019" id="4019">4019</a>
<a href="#4020" id="4020">4020</a>
<a href="#4021" id="4021">4021</a>
<a href="#4022" id="4022">4022</a>
<a href="#4023" id="4023">4023</a>
<a href="#4024" id="4024">4024</a>
<a href="#4025" id="4025">4025</a>
<a href="#4026" id="4026">4026</a>
<a href="#4027" id="4027">4027</a>
<a href="#4028" id="4028">4028</a>
<a href="#4029" id="4029">4029</a>
<a href="#4030" id="4030">4030</a>
<a href="#4031" id="4031">4031</a>
<a href="#4032" id="4032">4032</a>
<a href="#4033" id="4033">4033</a>
<a href="#4034" id="4034">4034</a>
<a href="#4035" id="4035">4035</a>
<a href="#4036" id="4036">4036</a>
<a href="#4037" id="4037">4037</a>
<a href="#4038" id="4038">4038</a>
<a href="#4039" id="4039">4039</a>
<a href="#4040" id="4040">4040</a>
<a href="#4041" id="4041">4041</a>
<a href="#4042" id="4042">4042</a>
<a href="#4043" id="4043">4043</a>
<a href="#4044" id="4044">4044</a>
<a href="#4045" id="4045">4045</a>
<a href="#4046" id="4046">4046</a>
<a href="#4047" id="4047">4047</a>
<a href="#4048" id="4048">4048</a>
<a href="#4049" id="4049">4049</a>
<a href="#4050" id="4050">4050</a>
<a href="#4051" id="4051">4051</a>
<a href="#4052" id="4052">4052</a>
<a href="#4053" id="4053">4053</a>
<a href="#4054" id="4054">4054</a>
<a href="#4055" id="4055">4055</a>
<a href="#4056" id="4056">4056</a>
<a href="#4057" id="4057">4057</a>
<a href="#4058" id="4058">4058</a>
<a href="#4059" id="4059">4059</a>
<a href="#4060" id="4060">4060</a>
<a href="#4061" id="4061">4061</a>
<a href="#4062" id="4062">4062</a>
<a href="#4063" id="4063">4063</a>
<a href="#4064" id="4064">4064</a>
<a href="#4065" id="4065">4065</a>
<a href="#4066" id="4066">4066</a>
<a href="#4067" id="4067">4067</a>
<a href="#4068" id="4068">4068</a>
<a href="#4069" id="4069">4069</a>
<a href="#4070" id="4070">4070</a>
<a href="#4071" id="4071">4071</a>
<a href="#4072" id="4072">4072</a>
<a href="#4073" id="4073">4073</a>
<a href="#4074" id="4074">4074</a>
<a href="#4075" id="4075">4075</a>
<a href="#4076" id="4076">4076</a>
<a href="#4077" id="4077">4077</a>
<a href="#4078" id="4078">4078</a>
<a href="#4079" id="4079">4079</a>
<a href="#4080" id="4080">4080</a>
<a href="#4081" id="4081">4081</a>
<a href="#4082" id="4082">4082</a>
<a href="#4083" id="4083">4083</a>
<a href="#4084" id="4084">4084</a>
<a href="#4085" id="4085">4085</a>
<a href="#4086" id="4086">4086</a>
<a href="#4087" id="4087">4087</a>
<a href="#4088" id="4088">4088</a>
<a href="#4089" id="4089">4089</a>
<a href="#4090" id="4090">4090</a>
<a href="#4091" id="4091">4091</a>
<a href="#4092" id="4092">4092</a>
<a href="#4093" id="4093">4093</a>
<a href="#4094" id="4094">4094</a>
<a href="#4095" id="4095">4095</a>
<a href="#4096" id="4096">4096</a>
<a href="#4097" id="4097">4097</a>
<a href="#4098" id="4098">4098</a>
<a href="#4099" id="4099">4099</a>
<a href="#4100" id="4100">4100</a>
<a href="#4101" id="4101">4101</a>
<a href="#4102" id="4102">4102</a>
<a href="#4103" id="4103">4103</a>
<a href="#4104" id="4104">4104</a>
<a href="#4105" id="4105">4105</a>
<a href="#4106" id="4106">4106</a>
<a href="#4107" id="4107">4107</a>
<a href="#4108" id="4108">4108</a>
<a href="#4109" id="4109">4109</a>
<a href="#4110" id="4110">4110</a>
<a href="#4111" id="4111">4111</a>
<a href="#4112" id="4112">4112</a>
<a href="#4113" id="4113">4113</a>
<a href="#4114" id="4114">4114</a>
<a href="#4115" id="4115">4115</a>
<a href="#4116" id="4116">4116</a>
<a href="#4117" id="4117">4117</a>
<a href="#4118" id="4118">4118</a>
<a href="#4119" id="4119">4119</a>
<a href="#4120" id="4120">4120</a>
<a href="#4121" id="4121">4121</a>
<a href="#4122" id="4122">4122</a>
<a href="#4123" id="4123">4123</a>
<a href="#4124" id="4124">4124</a>
<a href="#4125" id="4125">4125</a>
<a href="#4126" id="4126">4126</a>
<a href="#4127" id="4127">4127</a>
<a href="#4128" id="4128">4128</a>
<a href="#4129" id="4129">4129</a>
<a href="#4130" id="4130">4130</a>
<a href="#4131" id="4131">4131</a>
<a href="#4132" id="4132">4132</a>
<a href="#4133" id="4133">4133</a>
<a href="#4134" id="4134">4134</a>
<a href="#4135" id="4135">4135</a>
<a href="#4136" id="4136">4136</a>
<a href="#4137" id="4137">4137</a>
<a href="#4138" id="4138">4138</a>
<a href="#4139" id="4139">4139</a>
<a href="#4140" id="4140">4140</a>
<a href="#4141" id="4141">4141</a>
<a href="#4142" id="4142">4142</a>
<a href="#4143" id="4143">4143</a>
<a href="#4144" id="4144">4144</a>
<a href="#4145" id="4145">4145</a>
<a href="#4146" id="4146">4146</a>
<a href="#4147" id="4147">4147</a>
<a href="#4148" id="4148">4148</a>
<a href="#4149" id="4149">4149</a>
<a href="#4150" id="4150">4150</a>
<a href="#4151" id="4151">4151</a>
<a href="#4152" id="4152">4152</a>
<a href="#4153" id="4153">4153</a>
<a href="#4154" id="4154">4154</a>
<a href="#4155" id="4155">4155</a>
<a href="#4156" id="4156">4156</a>
<a href="#4157" id="4157">4157</a>
<a href="#4158" id="4158">4158</a>
<a href="#4159" id="4159">4159</a>
<a href="#4160" id="4160">4160</a>
<a href="#4161" id="4161">4161</a>
<a href="#4162" id="4162">4162</a>
<a href="#4163" id="4163">4163</a>
<a href="#4164" id="4164">4164</a>
<a href="#4165" id="4165">4165</a>
<a href="#4166" id="4166">4166</a>
<a href="#4167" id="4167">4167</a>
<a href="#4168" id="4168">4168</a>
<a href="#4169" id="4169">4169</a>
<a href="#4170" id="4170">4170</a>
<a href="#4171" id="4171">4171</a>
<a href="#4172" id="4172">4172</a>
<a href="#4173" id="4173">4173</a>
<a href="#4174" id="4174">4174</a>
<a href="#4175" id="4175">4175</a>
<a href="#4176" id="4176">4176</a>
<a href="#4177" id="4177">4177</a>
<a href="#4178" id="4178">4178</a>
<a href="#4179" id="4179">4179</a>
<a href="#4180" id="4180">4180</a>
<a href="#4181" id="4181">4181</a>
<a href="#4182" id="4182">4182</a>
<a href="#4183" id="4183">4183</a>
<a href="#4184" id="4184">4184</a>
<a href="#4185" id="4185">4185</a>
<a href="#4186" id="4186">4186</a>
<a href="#4187" id="4187">4187</a>
<a href="#4188" id="4188">4188</a>
<a href="#4189" id="4189">4189</a>
<a href="#4190" id="4190">4190</a>
<a href="#4191" id="4191">4191</a>
<a href="#4192" id="4192">4192</a>
<a href="#4193" id="4193">4193</a>
<a href="#4194" id="4194">4194</a>
<a href="#4195" id="4195">4195</a>
<a href="#4196" id="4196">4196</a>
<a href="#4197" id="4197">4197</a>
<a href="#4198" id="4198">4198</a>
<a href="#4199" id="4199">4199</a>
<a href="#4200" id="4200">4200</a>
<a href="#4201" id="4201">4201</a>
<a href="#4202" id="4202">4202</a>
<a href="#4203" id="4203">4203</a>
<a href="#4204" id="4204">4204</a>
<a href="#4205" id="4205">4205</a>
<a href="#4206" id="4206">4206</a>
<a href="#4207" id="4207">4207</a>
<a href="#4208" id="4208">4208</a>
<a href="#4209" id="4209">4209</a>
<a href="#4210" id="4210">4210</a>
<a href="#4211" id="4211">4211</a>
<a href="#4212" id="4212">4212</a>
<a href="#4213" id="4213">4213</a>
<a href="#4214" id="4214">4214</a>
<a href="#4215" id="4215">4215</a>
<a href="#4216" id="4216">4216</a>
<a href="#4217" id="4217">4217</a>
<a href="#4218" id="4218">4218</a>
<a href="#4219" id="4219">4219</a>
<a href="#4220" id="4220">4220</a>
<a href="#4221" id="4221">4221</a>
<a href="#4222" id="4222">4222</a>
<a href="#4223" id="4223">4223</a>
<a href="#4224" id="4224">4224</a>
<a href="#4225" id="4225">4225</a>
<a href="#4226" id="4226">4226</a>
<a href="#4227" id="4227">4227</a>
<a href="#4228" id="4228">4228</a>
<a href="#4229" id="4229">4229</a>
<a href="#4230" id="4230">4230</a>
<a href="#4231" id="4231">4231</a>
<a href="#4232" id="4232">4232</a>
<a href="#4233" id="4233">4233</a>
<a href="#4234" id="4234">4234</a>
<a href="#4235" id="4235">4235</a>
<a href="#4236" id="4236">4236</a>
<a href="#4237" id="4237">4237</a>
<a href="#4238" id="4238">4238</a>
<a href="#4239" id="4239">4239</a>
<a href="#4240" id="4240">4240</a>
<a href="#4241" id="4241">4241</a>
<a href="#4242" id="4242">4242</a>
<a href="#4243" id="4243">4243</a>
<a href="#4244" id="4244">4244</a>
<a href="#4245" id="4245">4245</a>
<a href="#4246" id="4246">4246</a>
<a href="#4247" id="4247">4247</a>
<a href="#4248" id="4248">4248</a>
<a href="#4249" id="4249">4249</a>
<a href="#4250" id="4250">4250</a>
<a href="#4251" id="4251">4251</a>
<a href="#4252" id="4252">4252</a>
<a href="#4253" id="4253">4253</a>
<a href="#4254" id="4254">4254</a>
<a href="#4255" id="4255">4255</a>
<a href="#4256" id="4256">4256</a>
<a href="#4257" id="4257">4257</a>
<a href="#4258" id="4258">4258</a>
<a href="#4259" id="4259">4259</a>
<a href="#4260" id="4260">4260</a>
<a href="#4261" id="4261">4261</a>
<a href="#4262" id="4262">4262</a>
<a href="#4263" id="4263">4263</a>
<a href="#4264" id="4264">4264</a>
<a href="#4265" id="4265">4265</a>
<a href="#4266" id="4266">4266</a>
<a href="#4267" id="4267">4267</a>
<a href="#4268" id="4268">4268</a>
<a href="#4269" id="4269">4269</a>
<a href="#4270" id="4270">4270</a>
<a href="#4271" id="4271">4271</a>
<a href="#4272" id="4272">4272</a>
<a href="#4273" id="4273">4273</a>
<a href="#4274" id="4274">4274</a>
<a href="#4275" id="4275">4275</a>
<a href="#4276" id="4276">4276</a>
<a href="#4277" id="4277">4277</a>
<a href="#4278" id="4278">4278</a>
<a href="#4279" id="4279">4279</a>
<a href="#4280" id="4280">4280</a>
<a href="#4281" id="4281">4281</a>
<a href="#4282" id="4282">4282</a>
<a href="#4283" id="4283">4283</a>
<a href="#4284" id="4284">4284</a>
<a href="#4285" id="4285">4285</a>
<a href="#4286" id="4286">4286</a>
<a href="#4287" id="4287">4287</a>
<a href="#4288" id="4288">4288</a>
<a href="#4289" id="4289">4289</a>
<a href="#4290" id="4290">4290</a>
<a href="#4291" id="4291">4291</a>
<a href="#4292" id="4292">4292</a>
<a href="#4293" id="4293">4293</a>
<a href="#4294" id="4294">4294</a>
<a href="#4295" id="4295">4295</a>
<a href="#4296" id="4296">4296</a>
<a href="#4297" id="4297">4297</a>
<a href="#4298" id="4298">4298</a>
<a href="#4299" id="4299">4299</a>
<a href="#4300" id="4300">4300</a>
<a href="#4301" id="4301">4301</a>
<a href="#4302" id="4302">4302</a>
<a href="#4303" id="4303">4303</a>
<a href="#4304" id="4304">4304</a>
<a href="#4305" id="4305">4305</a>
<a href="#4306" id="4306">4306</a>
<a href="#4307" id="4307">4307</a>
<a href="#4308" id="4308">4308</a>
<a href="#4309" id="4309">4309</a>
<a href="#4310" id="4310">4310</a>
<a href="#4311" id="4311">4311</a>
<a href="#4312" id="4312">4312</a>
<a href="#4313" id="4313">4313</a>
<a href="#4314" id="4314">4314</a>
<a href="#4315" id="4315">4315</a>
<a href="#4316" id="4316">4316</a>
<a href="#4317" id="4317">4317</a>
<a href="#4318" id="4318">4318</a>
<a href="#4319" id="4319">4319</a>
<a href="#4320" id="4320">4320</a>
<a href="#4321" id="4321">4321</a>
<a href="#4322" id="4322">4322</a>
<a href="#4323" id="4323">4323</a>
<a href="#4324" id="4324">4324</a>
<a href="#4325" id="4325">4325</a>
<a href="#4326" id="4326">4326</a>
<a href="#4327" id="4327">4327</a>
<a href="#4328" id="4328">4328</a>
<a href="#4329" id="4329">4329</a>
<a href="#4330" id="4330">4330</a>
<a href="#4331" id="4331">4331</a>
<a href="#4332" id="4332">4332</a>
<a href="#4333" id="4333">4333</a>
<a href="#4334" id="4334">4334</a>
<a href="#4335" id="4335">4335</a>
<a href="#4336" id="4336">4336</a>
<a href="#4337" id="4337">4337</a>
<a href="#4338" id="4338">4338</a>
<a href="#4339" id="4339">4339</a>
<a href="#4340" id="4340">4340</a>
<a href="#4341" id="4341">4341</a>
<a href="#4342" id="4342">4342</a>
<a href="#4343" id="4343">4343</a>
<a href="#4344" id="4344">4344</a>
<a href="#4345" id="4345">4345</a>
<a href="#4346" id="4346">4346</a>
<a href="#4347" id="4347">4347</a>
<a href="#4348" id="4348">4348</a>
<a href="#4349" id="4349">4349</a>
<a href="#4350" id="4350">4350</a>
<a href="#4351" id="4351">4351</a>
<a href="#4352" id="4352">4352</a>
<a href="#4353" id="4353">4353</a>
<a href="#4354" id="4354">4354</a>
<a href="#4355" id="4355">4355</a>
<a href="#4356" id="4356">4356</a>
<a href="#4357" id="4357">4357</a>
<a href="#4358" id="4358">4358</a>
<a href="#4359" id="4359">4359</a>
<a href="#4360" id="4360">4360</a>
<a href="#4361" id="4361">4361</a>
<a href="#4362" id="4362">4362</a>
<a href="#4363" id="4363">4363</a>
<a href="#4364" id="4364">4364</a>
<a href="#4365" id="4365">4365</a>
<a href="#4366" id="4366">4366</a>
<a href="#4367" id="4367">4367</a>
<a href="#4368" id="4368">4368</a>
<a href="#4369" id="4369">4369</a>
<a href="#4370" id="4370">4370</a>
<a href="#4371" id="4371">4371</a>
<a href="#4372" id="4372">4372</a>
<a href="#4373" id="4373">4373</a>
<a href="#4374" id="4374">4374</a>
<a href="#4375" id="4375">4375</a>
<a href="#4376" id="4376">4376</a>
<a href="#4377" id="4377">4377</a>
<a href="#4378" id="4378">4378</a>
<a href="#4379" id="4379">4379</a>
<a href="#4380" id="4380">4380</a>
<a href="#4381" id="4381">4381</a>
<a href="#4382" id="4382">4382</a>
<a href="#4383" id="4383">4383</a>
<a href="#4384" id="4384">4384</a>
<a href="#4385" id="4385">4385</a>
<a href="#4386" id="4386">4386</a>
<a href="#4387" id="4387">4387</a>
<a href="#4388" id="4388">4388</a>
<a href="#4389" id="4389">4389</a>
<a href="#4390" id="4390">4390</a>
<a href="#4391" id="4391">4391</a>
<a href="#4392" id="4392">4392</a>
<a href="#4393" id="4393">4393</a>
<a href="#4394" id="4394">4394</a>
<a href="#4395" id="4395">4395</a>
<a href="#4396" id="4396">4396</a>
<a href="#4397" id="4397">4397</a>
<a href="#4398" id="4398">4398</a>
<a href="#4399" id="4399">4399</a>
<a href="#4400" id="4400">4400</a>
<a href="#4401" id="4401">4401</a>
<a href="#4402" id="4402">4402</a>
<a href="#4403" id="4403">4403</a>
<a href="#4404" id="4404">4404</a>
<a href="#4405" id="4405">4405</a>
<a href="#4406" id="4406">4406</a>
<a href="#4407" id="4407">4407</a>
<a href="#4408" id="4408">4408</a>
<a href="#4409" id="4409">4409</a>
<a href="#4410" id="4410">4410</a>
<a href="#4411" id="4411">4411</a>
<a href="#4412" id="4412">4412</a>
<a href="#4413" id="4413">4413</a>
<a href="#4414" id="4414">4414</a>
<a href="#4415" id="4415">4415</a>
<a href="#4416" id="4416">4416</a>
<a href="#4417" id="4417">4417</a>
<a href="#4418" id="4418">4418</a>
</pre></div><pre class="rust"><code><span class="doccomment">/*!
Types and routines specific to lazy DFAs.

This module is the home of [`hybrid::dfa::DFA`](DFA).

This module also contains a [`hybrid::dfa::Builder`](Builder) and a
[`hybrid::dfa::Config`](Config) for configuring and building a lazy DFA.
*/

</span><span class="kw">use </span>core::{iter, mem::size_of};

<span class="kw">use </span>alloc::vec::Vec;

<span class="kw">use crate</span>::{
    hybrid::{
        error::{BuildError, CacheError, StartError},
        id::{LazyStateID, LazyStateIDError},
        search,
    },
    nfa::thompson,
    util::{
        alphabet::{<span class="self">self</span>, ByteClasses, ByteSet},
        determinize::{<span class="self">self</span>, State, StateBuilderEmpty, StateBuilderNFA},
        empty,
        prefilter::Prefilter,
        primitives::{PatternID, StateID <span class="kw">as </span>NFAStateID},
        search::{
            Anchored, HalfMatch, Input, MatchError, MatchKind, PatternSet,
        },
        sparse_set::SparseSets,
        start::{<span class="self">self</span>, Start, StartByteMap},
    },
};

<span class="doccomment">/// The minimum number of states that a lazy DFA's cache size must support.
///
/// This is checked at time of construction to ensure that at least some small
/// number of states can fit in the given capacity allotment. If we can't fit
/// at least this number of states, then the thinking is that it's pretty
/// senseless to use the lazy DFA. More to the point, parts of the code do
/// assume that the cache can fit at least some small number of states.
</span><span class="kw">const </span>MIN_STATES: usize = SENTINEL_STATES + <span class="number">2</span>;

<span class="doccomment">/// The number of "sentinel" states that get added to every lazy DFA.
///
/// These are special states indicating status conditions of a search: unknown,
/// dead and quit. These states in particular also use zero NFA states, so
/// their memory usage is quite small. This is relevant for computing the
/// minimum memory needed for a lazy DFA cache.
</span><span class="kw">const </span>SENTINEL_STATES: usize = <span class="number">3</span>;

<span class="doccomment">/// A hybrid NFA/DFA (also called a "lazy DFA") for regex searching.
///
/// A lazy DFA is a DFA that builds itself at search time. It otherwise has
/// very similar characteristics as a [`dense::DFA`](crate::dfa::dense::DFA).
/// Indeed, both support precisely the same regex features with precisely the
/// same semantics.
///
/// Where as a `dense::DFA` must be completely built to handle any input before
/// it may be used for search, a lazy DFA starts off effectively empty. During
/// a search, a lazy DFA will build itself depending on whether it has already
/// computed the next transition or not. If it has, then it looks a lot like
/// a `dense::DFA` internally: it does a very fast table based access to find
/// the next transition. Otherwise, if the state hasn't been computed, then it
/// does determinization _for that specific transition_ to compute the next DFA
/// state.
///
/// The main selling point of a lazy DFA is that, in practice, it has
/// the performance profile of a `dense::DFA` without the weakness of it
/// taking worst case exponential time to build. Indeed, for each byte of
/// input, the lazy DFA will construct as most one new DFA state. Thus, a
/// lazy DFA achieves worst case `O(mn)` time for regex search (where `m ~
/// pattern.len()` and `n ~ haystack.len()`).
///
/// The main downsides of a lazy DFA are:
///
/// 1. It requires mutable "cache" space during search. This is where the
/// transition table, among other things, is stored.
/// 2. In pathological cases (e.g., if the cache is too small), it will run
/// out of room and either require a bigger cache capacity or will repeatedly
/// clear the cache and thus repeatedly regenerate DFA states. Overall, this
/// will tend to be slower than a typical NFA simulation.
///
/// # Capabilities
///
/// Like a `dense::DFA`, a single lazy DFA fundamentally supports the following
/// operations:
///
/// 1. Detection of a match.
/// 2. Location of the end of a match.
/// 3. In the case of a lazy DFA with multiple patterns, which pattern matched
/// is reported as well.
///
/// A notable absence from the above list of capabilities is the location of
/// the *start* of a match. In order to provide both the start and end of
/// a match, *two* lazy DFAs are required. This functionality is provided by a
/// [`Regex`](crate::hybrid::regex::Regex).
///
/// # Example
///
/// This shows how to build a lazy DFA with the default configuration and
/// execute a search. Notice how, in contrast to a `dense::DFA`, we must create
/// a cache and pass it to our search routine.
///
/// ```
/// use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};
///
/// let dfa = DFA::new("foo[0-9]+")?;
/// let mut cache = dfa.create_cache();
///
/// let expected = Some(HalfMatch::must(0, 8));
/// assert_eq!(expected, dfa.try_search_fwd(
///     &amp;mut cache, &amp;Input::new("foo12345"))?,
/// );
/// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
/// ```
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>DFA {
    config: Config,
    nfa: thompson::NFA,
    stride2: usize,
    start_map: StartByteMap,
    classes: ByteClasses,
    quitset: ByteSet,
    cache_capacity: usize,
}

<span class="kw">impl </span>DFA {
    <span class="doccomment">/// Parse the given regular expression using a default configuration and
    /// return the corresponding lazy DFA.
    ///
    /// If you want a non-default configuration, then use the [`Builder`] to
    /// set your own configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};
    ///
    /// let dfa = DFA::new("foo[0-9]+bar")?;
    /// let mut cache = dfa.create_cache();
    ///
    /// let expected = HalfMatch::must(0, 11);
    /// assert_eq!(
    ///     Some(expected),
    ///     dfa.try_search_fwd(&amp;mut cache, &amp;Input::new("foo12345bar"))?,
    /// );
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[cfg(feature = <span class="string">"syntax"</span>)]
    </span><span class="kw">pub fn </span>new(pattern: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;DFA, BuildError&gt; {
        DFA::builder().build(pattern)
    }

    <span class="doccomment">/// Parse the given regular expressions using a default configuration and
    /// return the corresponding lazy multi-DFA.
    ///
    /// If you want a non-default configuration, then use the [`Builder`] to
    /// set your own configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};
    ///
    /// let dfa = DFA::new_many(&amp;["[0-9]+", "[a-z]+"])?;
    /// let mut cache = dfa.create_cache();
    ///
    /// let expected = HalfMatch::must(1, 3);
    /// assert_eq!(
    ///     Some(expected),
    ///     dfa.try_search_fwd(&amp;mut cache, &amp;Input::new("foo12345bar"))?,
    /// );
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[cfg(feature = <span class="string">"syntax"</span>)]
    </span><span class="kw">pub fn </span>new_many&lt;P: AsRef&lt;str&gt;&gt;(patterns: <span class="kw-2">&amp;</span>[P]) -&gt; <span class="prelude-ty">Result</span>&lt;DFA, BuildError&gt; {
        DFA::builder().build_many(patterns)
    }

    <span class="doccomment">/// Create a new lazy DFA that matches every input.
    ///
    /// # Example
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};
    ///
    /// let dfa = DFA::always_match()?;
    /// let mut cache = dfa.create_cache();
    ///
    /// let expected = HalfMatch::must(0, 0);
    /// assert_eq!(Some(expected), dfa.try_search_fwd(
    ///     &amp;mut cache, &amp;Input::new(""))?,
    /// );
    /// assert_eq!(Some(expected), dfa.try_search_fwd(
    ///     &amp;mut cache, &amp;Input::new("foo"))?,
    /// );
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>always_match() -&gt; <span class="prelude-ty">Result</span>&lt;DFA, BuildError&gt; {
        <span class="kw">let </span>nfa = thompson::NFA::always_match();
        Builder::new().build_from_nfa(nfa)
    }

    <span class="doccomment">/// Create a new lazy DFA that never matches any input.
    ///
    /// # Example
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, Input};
    ///
    /// let dfa = DFA::never_match()?;
    /// let mut cache = dfa.create_cache();
    ///
    /// assert_eq!(None, dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(""))?);
    /// assert_eq!(None, dfa.try_search_fwd(&amp;mut cache, &amp;Input::new("foo"))?);
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>never_match() -&gt; <span class="prelude-ty">Result</span>&lt;DFA, BuildError&gt; {
        <span class="kw">let </span>nfa = thompson::NFA::never_match();
        Builder::new().build_from_nfa(nfa)
    }

    <span class="doccomment">/// Return a default configuration for a `DFA`.
    ///
    /// This is a convenience routine to avoid needing to import the [`Config`]
    /// type when customizing the construction of a lazy DFA.
    ///
    /// # Example
    ///
    /// This example shows how to build a lazy DFA that heuristically supports
    /// Unicode word boundaries.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{hybrid::dfa::DFA, HalfMatch, MatchError, Input};
    ///
    /// let re = DFA::builder()
    ///     .configure(DFA::config().unicode_word_boundary(true))
    ///     .build(r"\b\w+\b")?;
    /// let mut cache = re.create_cache();
    ///
    /// // Since our haystack is all ASCII, the DFA search sees then and knows
    /// // it is legal to interpret Unicode word boundaries as ASCII word
    /// // boundaries.
    /// let input = Input::new("!!foo!!");
    /// let expected = HalfMatch::must(0, 5);
    /// assert_eq!(Some(expected), re.try_search_fwd(&amp;mut cache, &amp;input)?);
    ///
    /// // But if our haystack contains non-ASCII, then the search will fail
    /// // with an error.
    /// let input = Input::new("!!βββ!!");
    /// let expected = MatchError::quit(b'\xCE', 2);
    /// assert_eq!(Err(expected), re.try_search_fwd(&amp;mut cache, &amp;input));
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>config() -&gt; Config {
        Config::new()
    }

    <span class="doccomment">/// Return a builder for configuring the construction of a `Regex`.
    ///
    /// This is a convenience routine to avoid needing to import the
    /// [`Builder`] type in common cases.
    ///
    /// # Example
    ///
    /// This example shows how to use the builder to disable UTF-8 mode
    /// everywhere for lazy DFAs.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{hybrid::dfa::DFA, util::syntax, HalfMatch, Input};
    ///
    /// let re = DFA::builder()
    ///     .syntax(syntax::Config::new().utf8(false))
    ///     .build(r"foo(?-u:[^b])ar.*")?;
    /// let mut cache = re.create_cache();
    ///
    /// let input = Input::new(b"\xFEfoo\xFFarzz\xE2\x98\xFF\n");
    /// let expected = Some(HalfMatch::must(0, 9));
    /// let got = re.try_search_fwd(&amp;mut cache, &amp;input)?;
    /// assert_eq!(expected, got);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>builder() -&gt; Builder {
        Builder::new()
    }

    <span class="doccomment">/// Create a new cache for this lazy DFA.
    ///
    /// The cache returned should only be used for searches for this
    /// lazy DFA. If you want to reuse the cache for another DFA, then
    /// you must call [`Cache::reset`] with that DFA (or, equivalently,
    /// [`DFA::reset_cache`]).
    </span><span class="kw">pub fn </span>create_cache(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Cache {
        Cache::new(<span class="self">self</span>)
    }

    <span class="doccomment">/// Reset the given cache such that it can be used for searching with the
    /// this lazy DFA (and only this DFA).
    ///
    /// A cache reset permits reusing memory already allocated in this cache
    /// with a different lazy DFA.
    ///
    /// Resetting a cache sets its "clear count" to 0. This is relevant if the
    /// lazy DFA has been configured to "give up" after it has cleared the
    /// cache a certain number of times.
    ///
    /// Any lazy state ID generated by the cache prior to resetting it is
    /// invalid after the reset.
    ///
    /// # Example
    ///
    /// This shows how to re-purpose a cache for use with a different DFA.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};
    ///
    /// let dfa1 = DFA::new(r"\w")?;
    /// let dfa2 = DFA::new(r"\W")?;
    ///
    /// let mut cache = dfa1.create_cache();
    /// assert_eq!(
    ///     Some(HalfMatch::must(0, 2)),
    ///     dfa1.try_search_fwd(&amp;mut cache, &amp;Input::new("Δ"))?,
    /// );
    ///
    /// // Using 'cache' with dfa2 is not allowed. It may result in panics or
    /// // incorrect results. In order to re-purpose the cache, we must reset
    /// // it with the DFA we'd like to use it with.
    /// //
    /// // Similarly, after this reset, using the cache with 'dfa1' is also not
    /// // allowed.
    /// dfa2.reset_cache(&amp;mut cache);
    /// assert_eq!(
    ///     Some(HalfMatch::must(0, 3)),
    ///     dfa2.try_search_fwd(&amp;mut cache, &amp;Input::new("☃"))?,
    /// );
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>reset_cache(<span class="kw-2">&amp;</span><span class="self">self</span>, cache: <span class="kw-2">&amp;mut </span>Cache) {
        Lazy::new(<span class="self">self</span>, cache).reset_cache()
    }

    <span class="doccomment">/// Returns the total number of patterns compiled into this lazy DFA.
    ///
    /// In the case of a DFA that contains no patterns, this returns `0`.
    ///
    /// # Example
    ///
    /// This example shows the pattern length for a DFA that never matches:
    ///
    /// ```
    /// use regex_automata::hybrid::dfa::DFA;
    ///
    /// let dfa = DFA::never_match()?;
    /// assert_eq!(dfa.pattern_len(), 0);
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// And another example for a DFA that matches at every position:
    ///
    /// ```
    /// use regex_automata::hybrid::dfa::DFA;
    ///
    /// let dfa = DFA::always_match()?;
    /// assert_eq!(dfa.pattern_len(), 1);
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// And finally, a DFA that was constructed from multiple patterns:
    ///
    /// ```
    /// use regex_automata::hybrid::dfa::DFA;
    ///
    /// let dfa = DFA::new_many(&amp;["[0-9]+", "[a-z]+", "[A-Z]+"])?;
    /// assert_eq!(dfa.pattern_len(), 3);
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>pattern_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.nfa.pattern_len()
    }

    <span class="doccomment">/// Returns the equivalence classes that make up the alphabet for this DFA.
    ///
    /// Unless [`Config::byte_classes`] was disabled, it is possible that
    /// multiple distinct bytes are grouped into the same equivalence class
    /// if it is impossible for them to discriminate between a match and a
    /// non-match. This has the effect of reducing the overall alphabet size
    /// and in turn potentially substantially reducing the size of the DFA's
    /// transition table.
    ///
    /// The downside of using equivalence classes like this is that every state
    /// transition will automatically use this map to convert an arbitrary
    /// byte to its corresponding equivalence class. In practice this has a
    /// negligible impact on performance.
    </span><span class="kw">pub fn </span>byte_classes(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>ByteClasses {
        <span class="kw-2">&amp;</span><span class="self">self</span>.classes
    }

    <span class="doccomment">/// Returns this lazy DFA's configuration.
    </span><span class="kw">pub fn </span>get_config(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>Config {
        <span class="kw-2">&amp;</span><span class="self">self</span>.config
    }

    <span class="doccomment">/// Returns a reference to the underlying NFA.
    </span><span class="kw">pub fn </span>get_nfa(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>thompson::NFA {
        <span class="kw-2">&amp;</span><span class="self">self</span>.nfa
    }

    <span class="doccomment">/// Returns the stride, as a base-2 exponent, required for these
    /// equivalence classes.
    ///
    /// The stride is always the smallest power of 2 that is greater than or
    /// equal to the alphabet length. This is done so that converting between
    /// state IDs and indices can be done with shifts alone, which is much
    /// faster than integer division.
    </span><span class="kw">fn </span>stride2(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.stride2
    }

    <span class="doccomment">/// Returns the total stride for every state in this lazy DFA. This
    /// corresponds to the total number of transitions used by each state in
    /// this DFA's transition table.
    </span><span class="kw">fn </span>stride(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="number">1 </span>&lt;&lt; <span class="self">self</span>.stride2()
    }

    <span class="doccomment">/// Returns the memory usage, in bytes, of this lazy DFA.
    ///
    /// This does **not** include the stack size used up by this lazy DFA. To
    /// compute that, use `std::mem::size_of::&lt;DFA&gt;()`. This also does not
    /// include the size of the `Cache` used.
    ///
    /// This also does not include any heap memory used by the NFA inside of
    /// this hybrid NFA/DFA. This is because the NFA's ownership is shared, and
    /// thus not owned by this hybrid NFA/DFA. More practically, several regex
    /// engines in this crate embed an NFA, and reporting the NFA's memory
    /// usage in all of them would likely result in reporting higher heap
    /// memory than is actually used.
    </span><span class="kw">pub fn </span>memory_usage(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="comment">// The only thing that uses heap memory in a DFA is the NFA. But the
        // NFA has shared ownership, so reporting its memory as part of the
        // hybrid DFA is likely to lead to double-counting the NFA memory
        // somehow. In particular, this DFA does not really own an NFA, so
        // including it in the DFA's memory usage doesn't seem semantically
        // correct.
        </span><span class="number">0
    </span>}
}

<span class="kw">impl </span>DFA {
    <span class="doccomment">/// Executes a forward search and returns the end position of the leftmost
    /// match that is found. If no match exists, then `None` is returned.
    ///
    /// In particular, this method continues searching even after it enters
    /// a match state. The search only terminates once it has reached the
    /// end of the input or when it has entered a dead or quit state. Upon
    /// termination, the position of the last byte seen while still in a match
    /// state is returned.
    ///
    /// # Errors
    ///
    /// This routine errors if the search could not complete. This can occur
    /// in a number of circumstances:
    ///
    /// * The configuration of the lazy DFA may permit it to "quit" the search.
    /// For example, setting quit bytes or enabling heuristic support for
    /// Unicode word boundaries. The default configuration does not enable any
    /// option that could result in the lazy DFA quitting.
    /// * The configuration of the lazy DFA may also permit it to "give up"
    /// on a search if it makes ineffective use of its transition table
    /// cache. The default configuration does not enable this by default,
    /// although it is typically a good idea to.
    /// * When the provided `Input` configuration is not supported. For
    /// example, by providing an unsupported anchor mode.
    ///
    /// When a search returns an error, callers cannot know whether a match
    /// exists or not.
    ///
    /// # Example
    ///
    /// This example shows how to run a basic search.
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};
    ///
    /// let dfa = DFA::new("foo[0-9]+")?;
    /// let mut cache = dfa.create_cache();
    /// let expected = HalfMatch::must(0, 8);
    /// assert_eq!(Some(expected), dfa.try_search_fwd(
    ///     &amp;mut cache, &amp;Input::new("foo12345"))?,
    /// );
    ///
    /// // Even though a match is found after reading the first byte (`a`),
    /// // the leftmost first match semantics demand that we find the earliest
    /// // match that prefers earlier parts of the pattern over later parts.
    /// let dfa = DFA::new("abc|a")?;
    /// let mut cache = dfa.create_cache();
    /// let expected = HalfMatch::must(0, 3);
    /// assert_eq!(Some(expected), dfa.try_search_fwd(
    ///     &amp;mut cache, &amp;Input::new("abc"))?,
    /// );
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// # Example: specific pattern search
    ///
    /// This example shows how to build a lazy multi-DFA that permits searching
    /// for specific patterns.
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::DFA,
    ///     Anchored, HalfMatch, PatternID, Input,
    /// };
    ///
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().starts_for_each_pattern(true))
    ///     .build_many(&amp;["[a-z0-9]{6}", "[a-z][a-z0-9]{5}"])?;
    /// let mut cache = dfa.create_cache();
    /// let haystack = "foo123";
    ///
    /// // Since we are using the default leftmost-first match and both
    /// // patterns match at the same starting position, only the first pattern
    /// // will be returned in this case when doing a search for any of the
    /// // patterns.
    /// let expected = Some(HalfMatch::must(0, 6));
    /// let got = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(haystack))?;
    /// assert_eq!(expected, got);
    ///
    /// // But if we want to check whether some other pattern matches, then we
    /// // can provide its pattern ID.
    /// let expected = Some(HalfMatch::must(1, 6));
    /// let input = Input::new(haystack)
    ///     .anchored(Anchored::Pattern(PatternID::must(1)));
    /// let got = dfa.try_search_fwd(&amp;mut cache, &amp;input)?;
    /// assert_eq!(expected, got);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// # Example: specifying the bounds of a search
    ///
    /// This example shows how providing the bounds of a search can produce
    /// different results than simply sub-slicing the haystack.
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};
    ///
    /// // N.B. We disable Unicode here so that we use a simple ASCII word
    /// // boundary. Alternatively, we could enable heuristic support for
    /// // Unicode word boundaries since our haystack is pure ASCII.
    /// let dfa = DFA::new(r"(?-u)\b[0-9]{3}\b")?;
    /// let mut cache = dfa.create_cache();
    /// let haystack = "foo123bar";
    ///
    /// // Since we sub-slice the haystack, the search doesn't know about the
    /// // larger context and assumes that `123` is surrounded by word
    /// // boundaries. And of course, the match position is reported relative
    /// // to the sub-slice as well, which means we get `3` instead of `6`.
    /// let expected = Some(HalfMatch::must(0, 3));
    /// let got = dfa.try_search_fwd(
    ///     &amp;mut cache,
    ///     &amp;Input::new(&amp;haystack[3..6]),
    /// )?;
    /// assert_eq!(expected, got);
    ///
    /// // But if we provide the bounds of the search within the context of the
    /// // entire haystack, then the search can take the surrounding context
    /// // into account. (And if we did find a match, it would be reported
    /// // as a valid offset into `haystack` instead of its sub-slice.)
    /// let expected = None;
    /// let got = dfa.try_search_fwd(
    ///     &amp;mut cache,
    ///     &amp;Input::new(haystack).range(3..6),
    /// )?;
    /// assert_eq!(expected, got);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>try_search_fwd(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;mut </span>Cache,
        input: <span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="prelude-ty">Option</span>&lt;HalfMatch&gt;, MatchError&gt; {
        <span class="kw">let </span>utf8empty = <span class="self">self</span>.get_nfa().has_empty() &amp;&amp; <span class="self">self</span>.get_nfa().is_utf8();
        <span class="kw">let </span>hm = <span class="kw">match </span>search::find_fwd(<span class="self">self</span>, cache, input)<span class="question-mark">? </span>{
            <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="prelude-val">None</span>),
            <span class="prelude-val">Some</span>(hm) <span class="kw">if </span>!utf8empty =&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="prelude-val">Some</span>(hm)),
            <span class="prelude-val">Some</span>(hm) =&gt; hm,
        };
        <span class="comment">// We get to this point when we know our DFA can match the empty string
        // AND when UTF-8 mode is enabled. In this case, we skip any matches
        // whose offset splits a codepoint. Such a match is necessarily a
        // zero-width match, because UTF-8 mode requires the underlying NFA
        // to be built such that all non-empty matches span valid UTF-8.
        // Therefore, any match that ends in the middle of a codepoint cannot
        // be part of a span of valid UTF-8 and thus must be an empty match.
        // In such cases, we skip it, so as not to report matches that split a
        // codepoint.
        //
        // Note that this is not a checked assumption. Callers *can* provide an
        // NFA with UTF-8 mode enabled but produces non-empty matches that span
        // invalid UTF-8. But doing so is documented to result in unspecified
        // behavior.
        </span>empty::skip_splits_fwd(input, hm, hm.offset(), |input| {
            <span class="kw">let </span>got = search::find_fwd(<span class="self">self</span>, cache, input)<span class="question-mark">?</span>;
            <span class="prelude-val">Ok</span>(got.map(|hm| (hm, hm.offset())))
        })
    }

    <span class="doccomment">/// Executes a reverse search and returns the start of the position of the
    /// leftmost match that is found. If no match exists, then `None` is
    /// returned.
    ///
    /// # Errors
    ///
    /// This routine errors if the search could not complete. This can occur
    /// in a number of circumstances:
    ///
    /// * The configuration of the lazy DFA may permit it to "quit" the search.
    /// For example, setting quit bytes or enabling heuristic support for
    /// Unicode word boundaries. The default configuration does not enable any
    /// option that could result in the lazy DFA quitting.
    /// * The configuration of the lazy DFA may also permit it to "give up"
    /// on a search if it makes ineffective use of its transition table
    /// cache. The default configuration does not enable this by default,
    /// although it is typically a good idea to.
    /// * When the provided `Input` configuration is not supported. For
    /// example, by providing an unsupported anchor mode.
    ///
    /// When a search returns an error, callers cannot know whether a match
    /// exists or not.
    ///
    /// # Example
    ///
    /// This routine is principally useful when used in
    /// conjunction with the
    /// [`nfa::thompson::Config::reverse`](crate::nfa::thompson::Config::reverse)
    /// configuration. In general, it's unlikely to be correct to use both
    /// `try_search_fwd` and `try_search_rev` with the same DFA since any
    /// particular DFA will only support searching in one direction with
    /// respect to the pattern.
    ///
    /// ```
    /// use regex_automata::{
    ///     nfa::thompson,
    ///     hybrid::dfa::DFA,
    ///     HalfMatch, Input,
    /// };
    ///
    /// let dfa = DFA::builder()
    ///     .thompson(thompson::Config::new().reverse(true))
    ///     .build("foo[0-9]+")?;
    /// let mut cache = dfa.create_cache();
    /// let expected = HalfMatch::must(0, 0);
    /// assert_eq!(
    ///     Some(expected),
    ///     dfa.try_search_rev(&amp;mut cache, &amp;Input::new("foo12345"))?,
    /// );
    ///
    /// // Even though a match is found after reading the last byte (`c`),
    /// // the leftmost first match semantics demand that we find the earliest
    /// // match that prefers earlier parts of the pattern over latter parts.
    /// let dfa = DFA::builder()
    ///     .thompson(thompson::Config::new().reverse(true))
    ///     .build("abc|c")?;
    /// let mut cache = dfa.create_cache();
    /// let expected = HalfMatch::must(0, 0);
    /// assert_eq!(Some(expected), dfa.try_search_rev(
    ///     &amp;mut cache, &amp;Input::new("abc"))?,
    /// );
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// # Example: UTF-8 mode
    ///
    /// This examples demonstrates that UTF-8 mode applies to reverse
    /// DFAs. When UTF-8 mode is enabled in the underlying NFA, then all
    /// matches reported must correspond to valid UTF-8 spans. This includes
    /// prohibiting zero-width matches that split a codepoint.
    ///
    /// UTF-8 mode is enabled by default. Notice below how the only zero-width
    /// matches reported are those at UTF-8 boundaries:
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::DFA,
    ///     nfa::thompson,
    ///     HalfMatch, Input, MatchKind,
    /// };
    ///
    /// let dfa = DFA::builder()
    ///     .thompson(thompson::Config::new().reverse(true))
    ///     .build(r"")?;
    /// let mut cache = dfa.create_cache();
    ///
    /// // Run the reverse DFA to collect all matches.
    /// let mut input = Input::new("☃");
    /// let mut matches = vec![];
    /// loop {
    ///     match dfa.try_search_rev(&amp;mut cache, &amp;input)? {
    ///         None =&gt; break,
    ///         Some(hm) =&gt; {
    ///             matches.push(hm);
    ///             if hm.offset() == 0 || input.end() == 0 {
    ///                 break;
    ///             } else if hm.offset() &lt; input.end() {
    ///                 input.set_end(hm.offset());
    ///             } else {
    ///                 // This is only necessary to handle zero-width
    ///                 // matches, which of course occur in this example.
    ///                 // Without this, the search would never advance
    ///                 // backwards beyond the initial match.
    ///                 input.set_end(input.end() - 1);
    ///             }
    ///         }
    ///     }
    /// }
    ///
    /// // No matches split a codepoint.
    /// let expected = vec![
    ///     HalfMatch::must(0, 3),
    ///     HalfMatch::must(0, 0),
    /// ];
    /// assert_eq!(expected, matches);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// Now let's look at the same example, but with UTF-8 mode on the
    /// underlying NFA disabled:
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::DFA,
    ///     nfa::thompson,
    ///     HalfMatch, Input, MatchKind,
    /// };
    ///
    /// let dfa = DFA::builder()
    ///     .thompson(thompson::Config::new().reverse(true).utf8(false))
    ///     .build(r"")?;
    /// let mut cache = dfa.create_cache();
    ///
    /// // Run the reverse DFA to collect all matches.
    /// let mut input = Input::new("☃");
    /// let mut matches = vec![];
    /// loop {
    ///     match dfa.try_search_rev(&amp;mut cache, &amp;input)? {
    ///         None =&gt; break,
    ///         Some(hm) =&gt; {
    ///             matches.push(hm);
    ///             if hm.offset() == 0 || input.end() == 0 {
    ///                 break;
    ///             } else if hm.offset() &lt; input.end() {
    ///                 input.set_end(hm.offset());
    ///             } else {
    ///                 // This is only necessary to handle zero-width
    ///                 // matches, which of course occur in this example.
    ///                 // Without this, the search would never advance
    ///                 // backwards beyond the initial match.
    ///                 input.set_end(input.end() - 1);
    ///             }
    ///         }
    ///     }
    /// }
    ///
    /// // No matches split a codepoint.
    /// let expected = vec![
    ///     HalfMatch::must(0, 3),
    ///     HalfMatch::must(0, 2),
    ///     HalfMatch::must(0, 1),
    ///     HalfMatch::must(0, 0),
    /// ];
    /// assert_eq!(expected, matches);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>try_search_rev(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;mut </span>Cache,
        input: <span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="prelude-ty">Option</span>&lt;HalfMatch&gt;, MatchError&gt; {
        <span class="kw">let </span>utf8empty = <span class="self">self</span>.get_nfa().has_empty() &amp;&amp; <span class="self">self</span>.get_nfa().is_utf8();
        <span class="kw">let </span>hm = <span class="kw">match </span>search::find_rev(<span class="self">self</span>, cache, input)<span class="question-mark">? </span>{
            <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="prelude-val">None</span>),
            <span class="prelude-val">Some</span>(hm) <span class="kw">if </span>!utf8empty =&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="prelude-val">Some</span>(hm)),
            <span class="prelude-val">Some</span>(hm) =&gt; hm,
        };
        empty::skip_splits_rev(input, hm, hm.offset(), |input| {
            <span class="kw">let </span>got = search::find_rev(<span class="self">self</span>, cache, input)<span class="question-mark">?</span>;
            <span class="prelude-val">Ok</span>(got.map(|hm| (hm, hm.offset())))
        })
    }

    <span class="doccomment">/// Executes an overlapping forward search and returns the end position of
    /// matches as they are found. If no match exists, then `None` is returned.
    ///
    /// This routine is principally only useful when searching for multiple
    /// patterns on inputs where multiple patterns may match the same regions
    /// of text. In particular, callers must preserve the automaton's search
    /// state from prior calls so that the implementation knows where the last
    /// match occurred.
    ///
    /// When using this routine to implement an iterator of overlapping
    /// matches, the `start` of the search should remain invariant throughout
    /// iteration. The `OverlappingState` given to the search will keep track
    /// of the current position of the search. (This is because multiple
    /// matches may be reported at the same position, so only the search
    /// implementation itself knows when to advance the position.)
    ///
    /// If for some reason you want the search to forget about its previous
    /// state and restart the search at a particular position, then setting the
    /// state to [`OverlappingState::start`] will accomplish that.
    ///
    /// # Errors
    ///
    /// This routine errors if the search could not complete. This can occur
    /// in a number of circumstances:
    ///
    /// * The configuration of the lazy DFA may permit it to "quit" the search.
    /// For example, setting quit bytes or enabling heuristic support for
    /// Unicode word boundaries. The default configuration does not enable any
    /// option that could result in the lazy DFA quitting.
    /// * The configuration of the lazy DFA may also permit it to "give up"
    /// on a search if it makes ineffective use of its transition table
    /// cache. The default configuration does not enable this by default,
    /// although it is typically a good idea to.
    /// * When the provided `Input` configuration is not supported. For
    /// example, by providing an unsupported anchor mode.
    ///
    /// When a search returns an error, callers cannot know whether a match
    /// exists or not.
    ///
    /// # Example
    ///
    /// This example shows how to run a basic overlapping search. Notice
    /// that we build the automaton with a `MatchKind::All` configuration.
    /// Overlapping searches are unlikely to work as one would expect when
    /// using the default `MatchKind::LeftmostFirst` match semantics, since
    /// leftmost-first matching is fundamentally incompatible with overlapping
    /// searches. Namely, overlapping searches need to report matches as they
    /// are seen, where as leftmost-first searches will continue searching even
    /// after a match has been observed in order to find the conventional end
    /// position of the match. More concretely, leftmost-first searches use
    /// dead states to terminate a search after a specific match can no longer
    /// be extended. Overlapping searches instead do the opposite by continuing
    /// the search to find totally new matches (potentially of other patterns).
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{
    ///     hybrid::dfa::{DFA, OverlappingState},
    ///     HalfMatch, Input, MatchKind,
    /// };
    ///
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().match_kind(MatchKind::All))
    ///     .build_many(&amp;[r"\w+$", r"\S+$"])?;
    /// let mut cache = dfa.create_cache();
    ///
    /// let haystack = "@foo";
    /// let mut state = OverlappingState::start();
    ///
    /// let expected = Some(HalfMatch::must(1, 4));
    /// dfa.try_search_overlapping_fwd(
    ///     &amp;mut cache, &amp;Input::new(haystack), &amp;mut state,
    /// )?;
    /// assert_eq!(expected, state.get_match());
    ///
    /// // The first pattern also matches at the same position, so re-running
    /// // the search will yield another match. Notice also that the first
    /// // pattern is returned after the second. This is because the second
    /// // pattern begins its match before the first, is therefore an earlier
    /// // match and is thus reported first.
    /// let expected = Some(HalfMatch::must(0, 4));
    /// dfa.try_search_overlapping_fwd(
    ///     &amp;mut cache, &amp;Input::new(haystack), &amp;mut state,
    /// )?;
    /// assert_eq!(expected, state.get_match());
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>try_search_overlapping_fwd(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;mut </span>Cache,
        input: <span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;,
        state: <span class="kw-2">&amp;mut </span>OverlappingState,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), MatchError&gt; {
        <span class="kw">let </span>utf8empty = <span class="self">self</span>.get_nfa().has_empty() &amp;&amp; <span class="self">self</span>.get_nfa().is_utf8();
        search::find_overlapping_fwd(<span class="self">self</span>, cache, input, state)<span class="question-mark">?</span>;
        <span class="kw">match </span>state.get_match() {
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Ok</span>(()),
            <span class="prelude-val">Some</span>(<span class="kw">_</span>) <span class="kw">if </span>!utf8empty =&gt; <span class="prelude-val">Ok</span>(()),
            <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; skip_empty_utf8_splits_overlapping(
                input,
                state,
                |input, state| {
                    search::find_overlapping_fwd(<span class="self">self</span>, cache, input, state)
                },
            ),
        }
    }

    <span class="doccomment">/// Executes a reverse overlapping search and returns the start of the
    /// position of the leftmost match that is found. If no match exists, then
    /// `None` is returned.
    ///
    /// When using this routine to implement an iterator of overlapping
    /// matches, the `start` of the search should remain invariant throughout
    /// iteration. The `OverlappingState` given to the search will keep track
    /// of the current position of the search. (This is because multiple
    /// matches may be reported at the same position, so only the search
    /// implementation itself knows when to advance the position.)
    ///
    /// If for some reason you want the search to forget about its previous
    /// state and restart the search at a particular position, then setting the
    /// state to [`OverlappingState::start`] will accomplish that.
    ///
    /// # Errors
    ///
    /// This routine errors if the search could not complete. This can occur
    /// in a number of circumstances:
    ///
    /// * The configuration of the lazy DFA may permit it to "quit" the search.
    /// For example, setting quit bytes or enabling heuristic support for
    /// Unicode word boundaries. The default configuration does not enable any
    /// option that could result in the lazy DFA quitting.
    /// * The configuration of the lazy DFA may also permit it to "give up"
    /// on a search if it makes ineffective use of its transition table
    /// cache. The default configuration does not enable this by default,
    /// although it is typically a good idea to.
    /// * When the provided `Input` configuration is not supported. For
    /// example, by providing an unsupported anchor mode.
    ///
    /// When a search returns an error, callers cannot know whether a match
    /// exists or not.
    ///
    /// # Example: UTF-8 mode
    ///
    /// This examples demonstrates that UTF-8 mode applies to reverse
    /// DFAs. When UTF-8 mode is enabled in the underlying NFA, then all
    /// matches reported must correspond to valid UTF-8 spans. This includes
    /// prohibiting zero-width matches that split a codepoint.
    ///
    /// UTF-8 mode is enabled by default. Notice below how the only zero-width
    /// matches reported are those at UTF-8 boundaries:
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::{DFA, OverlappingState},
    ///     nfa::thompson,
    ///     HalfMatch, Input, MatchKind,
    /// };
    ///
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().match_kind(MatchKind::All))
    ///     .thompson(thompson::Config::new().reverse(true))
    ///     .build_many(&amp;[r"", r"☃"])?;
    /// let mut cache = dfa.create_cache();
    ///
    /// // Run the reverse DFA to collect all matches.
    /// let input = Input::new("☃");
    /// let mut state = OverlappingState::start();
    /// let mut matches = vec![];
    /// loop {
    ///     dfa.try_search_overlapping_rev(&amp;mut cache, &amp;input, &amp;mut state)?;
    ///     match state.get_match() {
    ///         None =&gt; break,
    ///         Some(hm) =&gt; matches.push(hm),
    ///     }
    /// }
    ///
    /// // No matches split a codepoint.
    /// let expected = vec![
    ///     HalfMatch::must(0, 3),
    ///     HalfMatch::must(1, 0),
    ///     HalfMatch::must(0, 0),
    /// ];
    /// assert_eq!(expected, matches);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// Now let's look at the same example, but with UTF-8 mode on the
    /// underlying NFA disabled:
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::{DFA, OverlappingState},
    ///     nfa::thompson,
    ///     HalfMatch, Input, MatchKind,
    /// };
    ///
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().match_kind(MatchKind::All))
    ///     .thompson(thompson::Config::new().reverse(true).utf8(false))
    ///     .build_many(&amp;[r"", r"☃"])?;
    /// let mut cache = dfa.create_cache();
    ///
    /// // Run the reverse DFA to collect all matches.
    /// let input = Input::new("☃");
    /// let mut state = OverlappingState::start();
    /// let mut matches = vec![];
    /// loop {
    ///     dfa.try_search_overlapping_rev(&amp;mut cache, &amp;input, &amp;mut state)?;
    ///     match state.get_match() {
    ///         None =&gt; break,
    ///         Some(hm) =&gt; matches.push(hm),
    ///     }
    /// }
    ///
    /// // Now *all* positions match, even within a codepoint,
    /// // because we lifted the requirement that matches
    /// // correspond to valid UTF-8 spans.
    /// let expected = vec![
    ///     HalfMatch::must(0, 3),
    ///     HalfMatch::must(0, 2),
    ///     HalfMatch::must(0, 1),
    ///     HalfMatch::must(1, 0),
    ///     HalfMatch::must(0, 0),
    /// ];
    /// assert_eq!(expected, matches);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>try_search_overlapping_rev(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;mut </span>Cache,
        input: <span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;,
        state: <span class="kw-2">&amp;mut </span>OverlappingState,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), MatchError&gt; {
        <span class="kw">let </span>utf8empty = <span class="self">self</span>.get_nfa().has_empty() &amp;&amp; <span class="self">self</span>.get_nfa().is_utf8();
        search::find_overlapping_rev(<span class="self">self</span>, cache, input, state)<span class="question-mark">?</span>;
        <span class="kw">match </span>state.get_match() {
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Ok</span>(()),
            <span class="prelude-val">Some</span>(<span class="kw">_</span>) <span class="kw">if </span>!utf8empty =&gt; <span class="prelude-val">Ok</span>(()),
            <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; skip_empty_utf8_splits_overlapping(
                input,
                state,
                |input, state| {
                    search::find_overlapping_rev(<span class="self">self</span>, cache, input, state)
                },
            ),
        }
    }

    <span class="doccomment">/// Writes the set of patterns that match anywhere in the given search
    /// configuration to `patset`. If multiple patterns match at the same
    /// position and the underlying DFA supports overlapping matches, then all
    /// matching patterns are written to the given set.
    ///
    /// Unless all of the patterns in this DFA are anchored, then generally
    /// speaking, this will visit every byte in the haystack.
    ///
    /// This search routine *does not* clear the pattern set. This gives some
    /// flexibility to the caller (e.g., running multiple searches with the
    /// same pattern set), but does make the API bug-prone if you're reusing
    /// the same pattern set for multiple searches but intended them to be
    /// independent.
    ///
    /// If a pattern ID matched but the given `PatternSet` does not have
    /// sufficient capacity to store it, then it is not inserted and silently
    /// dropped.
    ///
    /// # Errors
    ///
    /// This routine errors if the search could not complete. This can occur
    /// in a number of circumstances:
    ///
    /// * The configuration of the lazy DFA may permit it to "quit" the search.
    /// For example, setting quit bytes or enabling heuristic support for
    /// Unicode word boundaries. The default configuration does not enable any
    /// option that could result in the lazy DFA quitting.
    /// * The configuration of the lazy DFA may also permit it to "give up"
    /// on a search if it makes ineffective use of its transition table
    /// cache. The default configuration does not enable this by default,
    /// although it is typically a good idea to.
    /// * When the provided `Input` configuration is not supported. For
    /// example, by providing an unsupported anchor mode.
    ///
    /// When a search returns an error, callers cannot know whether a match
    /// exists or not.
    ///
    /// # Example
    ///
    /// This example shows how to find all matching patterns in a haystack,
    /// even when some patterns match at the same position as other patterns.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{
    ///     hybrid::dfa::DFA,
    ///     Input, MatchKind, PatternSet,
    /// };
    ///
    /// let patterns = &amp;[
    ///     r"\w+", r"\d+", r"\pL+", r"foo", r"bar", r"barfoo", r"foobar",
    /// ];
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().match_kind(MatchKind::All))
    ///     .build_many(patterns)?;
    /// let mut cache = dfa.create_cache();
    ///
    /// let input = Input::new("foobar");
    /// let mut patset = PatternSet::new(dfa.pattern_len());
    /// dfa.try_which_overlapping_matches(&amp;mut cache, &amp;input, &amp;mut patset)?;
    /// let expected = vec![0, 2, 3, 4, 6];
    /// let got: Vec&lt;usize&gt; = patset.iter().map(|p| p.as_usize()).collect();
    /// assert_eq!(expected, got);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>try_which_overlapping_matches(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;mut </span>Cache,
        input: <span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;,
        patset: <span class="kw-2">&amp;mut </span>PatternSet,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), MatchError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>state = OverlappingState::start();
        <span class="kw">while let </span><span class="prelude-val">Some</span>(m) = {
            <span class="self">self</span>.try_search_overlapping_fwd(cache, input, <span class="kw-2">&amp;mut </span>state)<span class="question-mark">?</span>;
            state.get_match()
        } {
            <span class="kw">let _ </span>= patset.try_insert(m.pattern());
            <span class="comment">// There's nothing left to find, so we can stop. Or the caller
            // asked us to.
            </span><span class="kw">if </span>patset.is_full() || input.get_earliest() {
                <span class="kw">break</span>;
            }
        }
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="kw">impl </span>DFA {
    <span class="doccomment">/// Transitions from the current state to the next state, given the next
    /// byte of input.
    ///
    /// The given cache is used to either reuse pre-computed state
    /// transitions, or to store this newly computed transition for future
    /// reuse. Thus, this routine guarantees that it will never return a state
    /// ID that has an "unknown" tag.
    ///
    /// # State identifier validity
    ///
    /// The only valid value for `current` is the lazy state ID returned
    /// by the most recent call to `next_state`, `next_state_untagged`,
    /// `next_state_untagged_unchecked`, `start_state_forward` or
    /// `state_state_reverse` for the given `cache`. Any state ID returned from
    /// prior calls to these routines (with the same `cache`) is considered
    /// invalid (even if it gives an appearance of working). State IDs returned
    /// from _any_ prior call for different `cache` values are also always
    /// invalid.
    ///
    /// The returned ID is always a valid ID when `current` refers to a valid
    /// ID. Moreover, this routine is defined for all possible values of
    /// `input`.
    ///
    /// These validity rules are not checked, even in debug mode. Callers are
    /// required to uphold these rules themselves.
    ///
    /// Violating these state ID validity rules will not sacrifice memory
    /// safety, but _may_ produce an incorrect result or a panic.
    ///
    /// # Panics
    ///
    /// If the given ID does not refer to a valid state, then this routine
    /// may panic but it also may not panic and instead return an invalid or
    /// incorrect ID.
    ///
    /// # Example
    ///
    /// This shows a simplistic example for walking a lazy DFA for a given
    /// haystack by using the `next_state` method.
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, Input};
    ///
    /// let dfa = DFA::new(r"[a-z]+r")?;
    /// let mut cache = dfa.create_cache();
    /// let haystack = "bar".as_bytes();
    ///
    /// // The start state is determined by inspecting the position and the
    /// // initial bytes of the haystack.
    /// let mut sid = dfa.start_state_forward(
    ///     &amp;mut cache, &amp;Input::new(haystack),
    /// )?;
    /// // Walk all the bytes in the haystack.
    /// for &amp;b in haystack {
    ///     sid = dfa.next_state(&amp;mut cache, sid, b)?;
    /// }
    /// // Matches are always delayed by 1 byte, so we must explicitly walk the
    /// // special "EOI" transition at the end of the search.
    /// sid = dfa.next_eoi_state(&amp;mut cache, sid)?;
    /// assert!(sid.is_match());
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>next_state(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;mut </span>Cache,
        current: LazyStateID,
        input: u8,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, CacheError&gt; {
        <span class="kw">let </span>class = usize::from(<span class="self">self</span>.classes.get(input));
        <span class="kw">let </span>offset = current.as_usize_untagged() + class;
        <span class="kw">let </span>sid = cache.trans[offset];
        <span class="kw">if </span>!sid.is_unknown() {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(sid);
        }
        <span class="kw">let </span>unit = alphabet::Unit::u8(input);
        Lazy::new(<span class="self">self</span>, cache).cache_next_state(current, unit)
    }

    <span class="doccomment">/// Transitions from the current state to the next state, given the next
    /// byte of input and a state ID that is not tagged.
    ///
    /// The only reason to use this routine is performance. In particular, the
    /// `next_state` method needs to do some additional checks, among them is
    /// to account for identifiers to states that are not yet computed. In
    /// such a case, the transition is computed on the fly. However, if it is
    /// known that the `current` state ID is untagged, then these checks can be
    /// omitted.
    ///
    /// Since this routine does not compute states on the fly, it does not
    /// modify the cache and thus cannot return an error. Consequently, `cache`
    /// does not need to be mutable and it is possible for this routine to
    /// return a state ID corresponding to the special "unknown" state. In
    /// this case, it is the caller's responsibility to use the prior state
    /// ID and `input` with `next_state` in order to force the computation of
    /// the unknown transition. Otherwise, trying to use the "unknown" state
    /// ID will just result in transitioning back to itself, and thus never
    /// terminating. (This is technically a special exemption to the state ID
    /// validity rules, but is permissible since this routine is guarateed to
    /// never mutate the given `cache`, and thus the identifier is guaranteed
    /// to remain valid.)
    ///
    /// See [`LazyStateID`] for more details on what it means for a state ID
    /// to be tagged. Also, see
    /// [`next_state_untagged_unchecked`](DFA::next_state_untagged_unchecked)
    /// for this same idea, but with bounds checks forcefully elided.
    ///
    /// # State identifier validity
    ///
    /// The only valid value for `current` is an **untagged** lazy
    /// state ID returned by the most recent call to `next_state`,
    /// `next_state_untagged`, `next_state_untagged_unchecked`,
    /// `start_state_forward` or `state_state_reverse` for the given `cache`.
    /// Any state ID returned from prior calls to these routines (with the
    /// same `cache`) is considered invalid (even if it gives an appearance
    /// of working). State IDs returned from _any_ prior call for different
    /// `cache` values are also always invalid.
    ///
    /// The returned ID is always a valid ID when `current` refers to a valid
    /// ID, although it may be tagged. Moreover, this routine is defined for
    /// all possible values of `input`.
    ///
    /// Not all validity rules are checked, even in debug mode. Callers are
    /// required to uphold these rules themselves.
    ///
    /// Violating these state ID validity rules will not sacrifice memory
    /// safety, but _may_ produce an incorrect result or a panic.
    ///
    /// # Panics
    ///
    /// If the given ID does not refer to a valid state, then this routine
    /// may panic but it also may not panic and instead return an invalid or
    /// incorrect ID.
    ///
    /// # Example
    ///
    /// This shows a simplistic example for walking a lazy DFA for a given
    /// haystack by using the `next_state_untagged` method where possible.
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, Input};
    ///
    /// let dfa = DFA::new(r"[a-z]+r")?;
    /// let mut cache = dfa.create_cache();
    /// let haystack = "bar".as_bytes();
    ///
    /// // The start state is determined by inspecting the position and the
    /// // initial bytes of the haystack.
    /// let mut sid = dfa.start_state_forward(
    ///     &amp;mut cache, &amp;Input::new(haystack),
    /// )?;
    /// // Walk all the bytes in the haystack.
    /// let mut at = 0;
    /// while at &lt; haystack.len() {
    ///     if sid.is_tagged() {
    ///         sid = dfa.next_state(&amp;mut cache, sid, haystack[at])?;
    ///     } else {
    ///         let mut prev_sid = sid;
    ///         // We attempt to chew through as much as we can while moving
    ///         // through untagged state IDs. Thus, the transition function
    ///         // does less work on average per byte. (Unrolling this loop
    ///         // may help even more.)
    ///         while at &lt; haystack.len() {
    ///             prev_sid = sid;
    ///             sid = dfa.next_state_untagged(
    ///                 &amp;mut cache, sid, haystack[at],
    ///             );
    ///             at += 1;
    ///             if sid.is_tagged() {
    ///                 break;
    ///             }
    ///         }
    ///         // We must ensure that we never proceed to the next iteration
    ///         // with an unknown state ID. If we don't account for this
    ///         // case, then search isn't guaranteed to terminate since all
    ///         // transitions on unknown states loop back to itself.
    ///         if sid.is_unknown() {
    ///             sid = dfa.next_state(
    ///                 &amp;mut cache, prev_sid, haystack[at - 1],
    ///             )?;
    ///         }
    ///     }
    /// }
    /// // Matches are always delayed by 1 byte, so we must explicitly walk the
    /// // special "EOI" transition at the end of the search.
    /// sid = dfa.next_eoi_state(&amp;mut cache, sid)?;
    /// assert!(sid.is_match());
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>next_state_untagged(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;</span>Cache,
        current: LazyStateID,
        input: u8,
    ) -&gt; LazyStateID {
        <span class="macro">debug_assert!</span>(!current.is_tagged());
        <span class="kw">let </span>class = usize::from(<span class="self">self</span>.classes.get(input));
        <span class="kw">let </span>offset = current.as_usize_unchecked() + class;
        cache.trans[offset]
    }

    <span class="doccomment">/// Transitions from the current state to the next state, eliding bounds
    /// checks, given the next byte of input and a state ID that is not tagged.
    ///
    /// The only reason to use this routine is performance. In particular, the
    /// `next_state` method needs to do some additional checks, among them is
    /// to account for identifiers to states that are not yet computed. In
    /// such a case, the transition is computed on the fly. However, if it is
    /// known that the `current` state ID is untagged, then these checks can be
    /// omitted.
    ///
    /// Since this routine does not compute states on the fly, it does not
    /// modify the cache and thus cannot return an error. Consequently, `cache`
    /// does not need to be mutable and it is possible for this routine to
    /// return a state ID corresponding to the special "unknown" state. In
    /// this case, it is the caller's responsibility to use the prior state
    /// ID and `input` with `next_state` in order to force the computation of
    /// the unknown transition. Otherwise, trying to use the "unknown" state
    /// ID will just result in transitioning back to itself, and thus never
    /// terminating. (This is technically a special exemption to the state ID
    /// validity rules, but is permissible since this routine is guarateed to
    /// never mutate the given `cache`, and thus the identifier is guaranteed
    /// to remain valid.)
    ///
    /// See [`LazyStateID`] for more details on what it means for a state ID
    /// to be tagged. Also, see
    /// [`next_state_untagged`](DFA::next_state_untagged)
    /// for this same idea, but with memory safety guaranteed by retaining
    /// bounds checks.
    ///
    /// # State identifier validity
    ///
    /// The only valid value for `current` is an **untagged** lazy
    /// state ID returned by the most recent call to `next_state`,
    /// `next_state_untagged`, `next_state_untagged_unchecked`,
    /// `start_state_forward` or `state_state_reverse` for the given `cache`.
    /// Any state ID returned from prior calls to these routines (with the
    /// same `cache`) is considered invalid (even if it gives an appearance
    /// of working). State IDs returned from _any_ prior call for different
    /// `cache` values are also always invalid.
    ///
    /// The returned ID is always a valid ID when `current` refers to a valid
    /// ID, although it may be tagged. Moreover, this routine is defined for
    /// all possible values of `input`.
    ///
    /// Not all validity rules are checked, even in debug mode. Callers are
    /// required to uphold these rules themselves.
    ///
    /// Violating these state ID validity rules will not sacrifice memory
    /// safety, but _may_ produce an incorrect result or a panic.
    ///
    /// # Safety
    ///
    /// Callers of this method must guarantee that `current` refers to a valid
    /// state ID according to the rules described above. If `current` is not a
    /// valid state ID for this automaton, then calling this routine may result
    /// in undefined behavior.
    ///
    /// If `current` is valid, then the ID returned is valid for all possible
    /// values of `input`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub unsafe fn </span>next_state_untagged_unchecked(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;</span>Cache,
        current: LazyStateID,
        input: u8,
    ) -&gt; LazyStateID {
        <span class="macro">debug_assert!</span>(!current.is_tagged());
        <span class="kw">let </span>class = usize::from(<span class="self">self</span>.classes.get(input));
        <span class="kw">let </span>offset = current.as_usize_unchecked() + class;
        <span class="kw-2">*</span>cache.trans.get_unchecked(offset)
    }

    <span class="doccomment">/// Transitions from the current state to the next state for the special
    /// EOI symbol.
    ///
    /// The given cache is used to either reuse pre-computed state
    /// transitions, or to store this newly computed transition for future
    /// reuse. Thus, this routine guarantees that it will never return a state
    /// ID that has an "unknown" tag.
    ///
    /// This routine must be called at the end of every search in a correct
    /// implementation of search. Namely, lazy DFAs in this crate delay matches
    /// by one byte in order to support look-around operators. Thus, after
    /// reaching the end of a haystack, a search implementation must follow one
    /// last EOI transition.
    ///
    /// It is best to think of EOI as an additional symbol in the alphabet of a
    /// DFA that is distinct from every other symbol. That is, the alphabet of
    /// lazy DFAs in this crate has a logical size of 257 instead of 256, where
    /// 256 corresponds to every possible inhabitant of `u8`. (In practice, the
    /// physical alphabet size may be smaller because of alphabet compression
    /// via equivalence classes, but EOI is always represented somehow in the
    /// alphabet.)
    ///
    /// # State identifier validity
    ///
    /// The only valid value for `current` is the lazy state ID returned
    /// by the most recent call to `next_state`, `next_state_untagged`,
    /// `next_state_untagged_unchecked`, `start_state_forward` or
    /// `state_state_reverse` for the given `cache`. Any state ID returned from
    /// prior calls to these routines (with the same `cache`) is considered
    /// invalid (even if it gives an appearance of working). State IDs returned
    /// from _any_ prior call for different `cache` values are also always
    /// invalid.
    ///
    /// The returned ID is always a valid ID when `current` refers to a valid
    /// ID.
    ///
    /// These validity rules are not checked, even in debug mode. Callers are
    /// required to uphold these rules themselves.
    ///
    /// Violating these state ID validity rules will not sacrifice memory
    /// safety, but _may_ produce an incorrect result or a panic.
    ///
    /// # Panics
    ///
    /// If the given ID does not refer to a valid state, then this routine
    /// may panic but it also may not panic and instead return an invalid or
    /// incorrect ID.
    ///
    /// # Example
    ///
    /// This shows a simplistic example for walking a DFA for a given haystack,
    /// and then finishing the search with the final EOI transition.
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, Input};
    ///
    /// let dfa = DFA::new(r"[a-z]+r")?;
    /// let mut cache = dfa.create_cache();
    /// let haystack = "bar".as_bytes();
    ///
    /// // The start state is determined by inspecting the position and the
    /// // initial bytes of the haystack.
    /// let mut sid = dfa.start_state_forward(
    ///     &amp;mut cache, &amp;Input::new(haystack),
    /// )?;
    /// // Walk all the bytes in the haystack.
    /// for &amp;b in haystack {
    ///     sid = dfa.next_state(&amp;mut cache, sid, b)?;
    /// }
    /// // Matches are always delayed by 1 byte, so we must explicitly walk
    /// // the special "EOI" transition at the end of the search. Without this
    /// // final transition, the assert below will fail since the DFA will not
    /// // have entered a match state yet!
    /// sid = dfa.next_eoi_state(&amp;mut cache, sid)?;
    /// assert!(sid.is_match());
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>next_eoi_state(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;mut </span>Cache,
        current: LazyStateID,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, CacheError&gt; {
        <span class="kw">let </span>eoi = <span class="self">self</span>.classes.eoi().as_usize();
        <span class="kw">let </span>offset = current.as_usize_untagged() + eoi;
        <span class="kw">let </span>sid = cache.trans[offset];
        <span class="kw">if </span>!sid.is_unknown() {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(sid);
        }
        <span class="kw">let </span>unit = <span class="self">self</span>.classes.eoi();
        Lazy::new(<span class="self">self</span>, cache).cache_next_state(current, unit)
    }

    <span class="doccomment">/// Return the ID of the start state for this lazy DFA for the given
    /// starting configuration.
    ///
    /// Unlike typical DFA implementations, the start state for DFAs in this
    /// crate is dependent on a few different factors:
    ///
    /// * The [`Anchored`] mode of the search. Unanchored, anchored and
    /// anchored searches for a specific [`PatternID`] all use different start
    /// states.
    /// * Whether a "look-behind" byte exists. For example, the `^` anchor
    /// matches if and only if there is no look-behind byte.
    /// * The specific value of that look-behind byte. For example, a `(?m:^)`
    /// assertion only matches when there is either no look-behind byte, or
    /// when the look-behind byte is a line terminator.
    ///
    /// The [starting configuration](start::Config) provides the above
    /// information.
    ///
    /// This routine can be used for either forward or reverse searches.
    /// Although, as a convenience, if you have an [`Input`], then it
    /// may be more succinct to use [`DFA::start_state_forward`] or
    /// [`DFA::start_state_reverse`]. Note, for example, that the convenience
    /// routines return a [`MatchError`] on failure where as this routine
    /// returns a [`StartError`].
    ///
    /// # Errors
    ///
    /// This may return a [`StartError`] if the search needs to give up when
    /// determining the start state (for example, if it sees a "quit" byte
    /// or if the cache has become inefficient). This can also return an
    /// error if the given configuration contains an unsupported [`Anchored`]
    /// configuration.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"perf-inline"</span>, inline(always))]
    </span><span class="kw">pub fn </span>start_state(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;mut </span>Cache,
        config: <span class="kw-2">&amp;</span>start::Config,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, StartError&gt; {
        <span class="kw">let </span>lazy = LazyRef::new(<span class="self">self</span>, cache);
        <span class="kw">let </span>anchored = config.get_anchored();
        <span class="kw">let </span>start = <span class="kw">match </span>config.get_look_behind() {
            <span class="prelude-val">None </span>=&gt; Start::Text,
            <span class="prelude-val">Some</span>(byte) =&gt; {
                <span class="kw">if </span>!<span class="self">self</span>.quitset.is_empty() &amp;&amp; <span class="self">self</span>.quitset.contains(byte) {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(StartError::quit(byte));
                }
                <span class="self">self</span>.start_map.get(byte)
            }
        };
        <span class="kw">let </span>start_id = lazy.get_cached_start_id(anchored, start)<span class="question-mark">?</span>;
        <span class="kw">if </span>!start_id.is_unknown() {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(start_id);
        }
        Lazy::new(<span class="self">self</span>, cache).cache_start_group(anchored, start)
    }

    <span class="doccomment">/// Return the ID of the start state for this lazy DFA when executing a
    /// forward search.
    ///
    /// This is a convenience routine for calling [`DFA::start_state`] that
    /// converts the given [`Input`] to a [start configuration](start::Config).
    /// Additionally, if an error occurs, it is converted from a [`StartError`]
    /// to a [`MatchError`] using the offset information in the given
    /// [`Input`].
    ///
    /// # Errors
    ///
    /// This may return a [`MatchError`] if the search needs to give up when
    /// determining the start state (for example, if it sees a "quit" byte or
    /// if the cache has become inefficient). This can also return an error if
    /// the given `Input` contains an unsupported [`Anchored`] configuration.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"perf-inline"</span>, inline(always))]
    </span><span class="kw">pub fn </span>start_state_forward(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;mut </span>Cache,
        input: <span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, MatchError&gt; {
        <span class="kw">let </span>config = start::Config::from_input_forward(input);
        <span class="self">self</span>.start_state(cache, <span class="kw-2">&amp;</span>config).map_err(|err| <span class="kw">match </span>err {
            StartError::Cache { .. } =&gt; MatchError::gave_up(input.start()),
            StartError::Quit { byte } =&gt; {
                <span class="kw">let </span>offset = input
                    .start()
                    .checked_sub(<span class="number">1</span>)
                    .expect(<span class="string">"no quit in start without look-behind"</span>);
                MatchError::quit(byte, offset)
            }
            StartError::UnsupportedAnchored { mode } =&gt; {
                MatchError::unsupported_anchored(mode)
            }
        })
    }

    <span class="doccomment">/// Return the ID of the start state for this lazy DFA when executing a
    /// reverse search.
    ///
    /// This is a convenience routine for calling [`DFA::start_state`] that
    /// converts the given [`Input`] to a [start configuration](start::Config).
    /// Additionally, if an error occurs, it is converted from a [`StartError`]
    /// to a [`MatchError`] using the offset information in the given
    /// [`Input`].
    ///
    /// # Errors
    ///
    /// This may return a [`MatchError`] if the search needs to give up when
    /// determining the start state (for example, if it sees a "quit" byte or
    /// if the cache has become inefficient). This can also return an error if
    /// the given `Input` contains an unsupported [`Anchored`] configuration.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"perf-inline"</span>, inline(always))]
    </span><span class="kw">pub fn </span>start_state_reverse(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;mut </span>Cache,
        input: <span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, MatchError&gt; {
        <span class="kw">let </span>config = start::Config::from_input_reverse(input);
        <span class="self">self</span>.start_state(cache, <span class="kw-2">&amp;</span>config).map_err(|err| <span class="kw">match </span>err {
            StartError::Cache { .. } =&gt; MatchError::gave_up(input.end()),
            StartError::Quit { byte } =&gt; {
                <span class="kw">let </span>offset = input.end();
                MatchError::quit(byte, offset)
            }
            StartError::UnsupportedAnchored { mode } =&gt; {
                MatchError::unsupported_anchored(mode)
            }
        })
    }

    <span class="doccomment">/// Returns the total number of patterns that match in this state.
    ///
    /// If the lazy DFA was compiled with one pattern, then this must
    /// necessarily always return `1` for all match states.
    ///
    /// A lazy DFA guarantees that [`DFA::match_pattern`] can be called with
    /// indices up to (but not including) the length returned by this routine
    /// without panicking.
    ///
    /// # Panics
    ///
    /// If the given state is not a match state, then this may either panic
    /// or return an incorrect result.
    ///
    /// # Example
    ///
    /// This example shows a simple instance of implementing overlapping
    /// matches. In particular, it shows not only how to determine how many
    /// patterns have matched in a particular state, but also how to access
    /// which specific patterns have matched.
    ///
    /// Notice that we must use [`MatchKind::All`] when building the DFA. If we
    /// used [`MatchKind::LeftmostFirst`] instead, then the DFA would not be
    /// constructed in a way that supports overlapping matches. (It would only
    /// report a single pattern that matches at any particular point in time.)
    ///
    /// Another thing to take note of is the patterns used and the order in
    /// which the pattern IDs are reported. In the example below, pattern `3`
    /// is yielded first. Why? Because it corresponds to the match that
    /// appears first. Namely, the `@` symbol is part of `\S+` but not part
    /// of any of the other patterns. Since the `\S+` pattern has a match that
    /// starts to the left of any other pattern, its ID is returned before any
    /// other.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{hybrid::dfa::DFA, Input, MatchKind};
    ///
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().match_kind(MatchKind::All))
    ///     .build_many(&amp;[
    ///         r"\w+", r"[a-z]+", r"[A-Z]+", r"\S+",
    ///     ])?;
    /// let mut cache = dfa.create_cache();
    /// let haystack = "@bar".as_bytes();
    ///
    /// // The start state is determined by inspecting the position and the
    /// // initial bytes of the haystack.
    /// let mut sid = dfa.start_state_forward(
    ///     &amp;mut cache, &amp;Input::new(haystack),
    /// )?;
    /// // Walk all the bytes in the haystack.
    /// for &amp;b in haystack {
    ///     sid = dfa.next_state(&amp;mut cache, sid, b)?;
    /// }
    /// sid = dfa.next_eoi_state(&amp;mut cache, sid)?;
    ///
    /// assert!(sid.is_match());
    /// assert_eq!(dfa.match_len(&amp;mut cache, sid), 3);
    /// // The following calls are guaranteed to not panic since `match_len`
    /// // returned `3` above.
    /// assert_eq!(dfa.match_pattern(&amp;mut cache, sid, 0).as_usize(), 3);
    /// assert_eq!(dfa.match_pattern(&amp;mut cache, sid, 1).as_usize(), 0);
    /// assert_eq!(dfa.match_pattern(&amp;mut cache, sid, 2).as_usize(), 1);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>match_len(<span class="kw-2">&amp;</span><span class="self">self</span>, cache: <span class="kw-2">&amp;</span>Cache, id: LazyStateID) -&gt; usize {
        <span class="macro">assert!</span>(id.is_match());
        LazyRef::new(<span class="self">self</span>, cache).get_cached_state(id).match_len()
    }

    <span class="doccomment">/// Returns the pattern ID corresponding to the given match index in the
    /// given state.
    ///
    /// See [`DFA::match_len`] for an example of how to use this method
    /// correctly. Note that if you know your lazy DFA is configured with a
    /// single pattern, then this routine is never necessary since it will
    /// always return a pattern ID of `0` for an index of `0` when `id`
    /// corresponds to a match state.
    ///
    /// Typically, this routine is used when implementing an overlapping
    /// search, as the example for `DFA::match_len` does.
    ///
    /// # Panics
    ///
    /// If the state ID is not a match state or if the match index is out
    /// of bounds for the given state, then this routine may either panic
    /// or produce an incorrect result. If the state ID is correct and the
    /// match index is correct, then this routine always produces a valid
    /// `PatternID`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>match_pattern(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cache: <span class="kw-2">&amp;</span>Cache,
        id: LazyStateID,
        match_index: usize,
    ) -&gt; PatternID {
        <span class="comment">// This is an optimization for the very common case of a DFA with a
        // single pattern. This conditional avoids a somewhat more costly path
        // that finds the pattern ID from the corresponding `State`, which
        // requires a bit of slicing/pointer-chasing. This optimization tends
        // to only matter when matches are frequent.
        </span><span class="kw">if </span><span class="self">self</span>.pattern_len() == <span class="number">1 </span>{
            <span class="kw">return </span>PatternID::ZERO;
        }
        LazyRef::new(<span class="self">self</span>, cache)
            .get_cached_state(id)
            .match_pattern(match_index)
    }
}

<span class="doccomment">/// A cache represents a partially computed DFA.
///
/// A cache is the key component that differentiates a classical DFA and a
/// hybrid NFA/DFA (also called a "lazy DFA"). Where a classical DFA builds a
/// complete transition table that can handle all possible inputs, a hybrid
/// NFA/DFA starts with an empty transition table and builds only the parts
/// required during search. The parts that are built are stored in a cache. For
/// this reason, a cache is a required parameter for nearly every operation on
/// a [`DFA`].
///
/// Caches can be created from their corresponding DFA via
/// [`DFA::create_cache`]. A cache can only be used with either the DFA that
/// created it, or the DFA that was most recently used to reset it with
/// [`Cache::reset`]. Using a cache with any other DFA may result in panics
/// or incorrect results.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>Cache {
    <span class="comment">// N.B. If you're looking to understand how determinization works, it
    // is probably simpler to first grok src/dfa/determinize.rs, since that
    // doesn't have the "laziness" component.
    </span><span class="doccomment">/// The transition table.
    ///
    /// Given a `current` LazyStateID and an `input` byte, the next state can
    /// be computed via `trans[untagged(current) + equiv_class(input)]`. Notice
    /// that no multiplication is used. That's because state identifiers are
    /// "premultiplied."
    ///
    /// Note that the next state may be the "unknown" state. In this case, the
    /// next state is not known and determinization for `current` on `input`
    /// must be performed.
    </span>trans: Vec&lt;LazyStateID&gt;,
    <span class="doccomment">/// The starting states for this DFA.
    ///
    /// These are computed lazily. Initially, these are all set to "unknown"
    /// lazy state IDs.
    ///
    /// When 'starts_for_each_pattern' is disabled (the default), then the size
    /// of this is constrained to the possible starting configurations based
    /// on the search parameters. (At time of writing, that's 4.) However,
    /// when starting states for each pattern is enabled, then there are N
    /// additional groups of starting states, where each group reflects the
    /// different possible configurations and N is the number of patterns.
    </span>starts: Vec&lt;LazyStateID&gt;,
    <span class="doccomment">/// A sequence of NFA/DFA powerset states that have been computed for this
    /// lazy DFA. This sequence is indexable by untagged LazyStateIDs. (Every
    /// tagged LazyStateID can be used to index this sequence by converting it
    /// to its untagged form.)
    </span>states: Vec&lt;State&gt;,
    <span class="doccomment">/// A map from states to their corresponding IDs. This map may be accessed
    /// via the raw byte representation of a state, which means that a `State`
    /// does not need to be allocated to determine whether it already exists
    /// in this map. Indeed, the existence of such a state is what determines
    /// whether we allocate a new `State` or not.
    ///
    /// The higher level idea here is that we do just enough determinization
    /// for a state to check whether we've already computed it. If we have,
    /// then we can save a little (albeit not much) work. The real savings is
    /// in memory usage. If we never checked for trivially duplicate states,
    /// then our memory usage would explode to unreasonable levels.
    </span>states_to_id: StateMap,
    <span class="doccomment">/// Sparse sets used to track which NFA states have been visited during
    /// various traversals.
    </span>sparses: SparseSets,
    <span class="doccomment">/// Scratch space for traversing the NFA graph. (We use space on the heap
    /// instead of the call stack.)
    </span>stack: Vec&lt;NFAStateID&gt;,
    <span class="doccomment">/// Scratch space for building a NFA/DFA powerset state. This is used to
    /// help amortize allocation since not every powerset state generated is
    /// added to the cache. In particular, if it already exists in the cache,
    /// then there is no need to allocate a new `State` for it.
    </span>scratch_state_builder: StateBuilderEmpty,
    <span class="doccomment">/// A simple abstraction for handling the saving of at most a single state
    /// across a cache clearing. This is required for correctness. Namely, if
    /// adding a new state after clearing the cache fails, then the caller
    /// must retain the ability to continue using the state ID given. The
    /// state corresponding to the state ID is what we preserve across cache
    /// clearings.
    </span>state_saver: StateSaver,
    <span class="doccomment">/// The memory usage, in bytes, used by 'states' and 'states_to_id'. We
    /// track this as new states are added since states use a variable amount
    /// of heap. Tracking this as we add states makes it possible to compute
    /// the total amount of memory used by the determinizer in constant time.
    </span>memory_usage_state: usize,
    <span class="doccomment">/// The number of times the cache has been cleared. When a minimum cache
    /// clear count is set, then the cache will return an error instead of
    /// clearing the cache if the count has been exceeded.
    </span>clear_count: usize,
    <span class="doccomment">/// The total number of bytes searched since the last time this cache was
    /// cleared, not including the current search.
    ///
    /// This can be added to the length of the current search to get the true
    /// total number of bytes searched.
    ///
    /// This is generally only non-zero when the
    /// `Cache::search_{start,update,finish}` APIs are used to track search
    /// progress.
    </span>bytes_searched: usize,
    <span class="doccomment">/// The progress of the current search.
    ///
    /// This is only non-`None` when callers utlize the `Cache::search_start`,
    /// `Cache::search_update` and `Cache::search_finish` APIs.
    ///
    /// The purpose of recording search progress is to be able to make a
    /// determination about the efficiency of the cache. Namely, by keeping
    /// track of the
    </span>progress: <span class="prelude-ty">Option</span>&lt;SearchProgress&gt;,
}

<span class="kw">impl </span>Cache {
    <span class="doccomment">/// Create a new cache for the given lazy DFA.
    ///
    /// The cache returned should only be used for searches for the given DFA.
    /// If you want to reuse the cache for another DFA, then you must call
    /// [`Cache::reset`] with that DFA.
    </span><span class="kw">pub fn </span>new(dfa: <span class="kw-2">&amp;</span>DFA) -&gt; Cache {
        <span class="kw">let </span><span class="kw-2">mut </span>cache = Cache {
            trans: <span class="macro">alloc::vec!</span>[],
            starts: <span class="macro">alloc::vec!</span>[],
            states: <span class="macro">alloc::vec!</span>[],
            states_to_id: StateMap::new(),
            sparses: SparseSets::new(dfa.get_nfa().states().len()),
            stack: <span class="macro">alloc::vec!</span>[],
            scratch_state_builder: StateBuilderEmpty::new(),
            state_saver: StateSaver::none(),
            memory_usage_state: <span class="number">0</span>,
            clear_count: <span class="number">0</span>,
            bytes_searched: <span class="number">0</span>,
            progress: <span class="prelude-val">None</span>,
        };
        <span class="macro">debug!</span>(<span class="string">"pre-init lazy DFA cache size: {}"</span>, cache.memory_usage());
        Lazy { dfa, cache: <span class="kw-2">&amp;mut </span>cache }.init_cache();
        <span class="macro">debug!</span>(<span class="string">"post-init lazy DFA cache size: {}"</span>, cache.memory_usage());
        cache
    }

    <span class="doccomment">/// Reset this cache such that it can be used for searching with the given
    /// lazy DFA (and only that DFA).
    ///
    /// A cache reset permits reusing memory already allocated in this cache
    /// with a different lazy DFA.
    ///
    /// Resetting a cache sets its "clear count" to 0. This is relevant if the
    /// lazy DFA has been configured to "give up" after it has cleared the
    /// cache a certain number of times.
    ///
    /// Any lazy state ID generated by the cache prior to resetting it is
    /// invalid after the reset.
    ///
    /// # Example
    ///
    /// This shows how to re-purpose a cache for use with a different DFA.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};
    ///
    /// let dfa1 = DFA::new(r"\w")?;
    /// let dfa2 = DFA::new(r"\W")?;
    ///
    /// let mut cache = dfa1.create_cache();
    /// assert_eq!(
    ///     Some(HalfMatch::must(0, 2)),
    ///     dfa1.try_search_fwd(&amp;mut cache, &amp;Input::new("Δ"))?,
    /// );
    ///
    /// // Using 'cache' with dfa2 is not allowed. It may result in panics or
    /// // incorrect results. In order to re-purpose the cache, we must reset
    /// // it with the DFA we'd like to use it with.
    /// //
    /// // Similarly, after this reset, using the cache with 'dfa1' is also not
    /// // allowed.
    /// cache.reset(&amp;dfa2);
    /// assert_eq!(
    ///     Some(HalfMatch::must(0, 3)),
    ///     dfa2.try_search_fwd(&amp;mut cache, &amp;Input::new("☃"))?,
    /// );
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>reset(<span class="kw-2">&amp;mut </span><span class="self">self</span>, dfa: <span class="kw-2">&amp;</span>DFA) {
        Lazy::new(dfa, <span class="self">self</span>).reset_cache()
    }

    <span class="doccomment">/// Initializes a new search starting at the given position.
    ///
    /// If a previous search was unfinished, then it is finished automatically
    /// and a new search is begun.
    ///
    /// Note that keeping track of search progress is _not necessary_
    /// for correct implementations of search using a lazy DFA. Keeping
    /// track of search progress is only necessary if you want the
    /// [`Config::minimum_bytes_per_state`] configuration knob to work.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>search_start(<span class="kw-2">&amp;mut </span><span class="self">self</span>, at: usize) {
        <span class="comment">// If a previous search wasn't marked as finished, then finish it
        // now automatically.
        </span><span class="kw">if let </span><span class="prelude-val">Some</span>(p) = <span class="self">self</span>.progress.take() {
            <span class="self">self</span>.bytes_searched += p.len();
        }
        <span class="self">self</span>.progress = <span class="prelude-val">Some</span>(SearchProgress { start: at, at });
    }

    <span class="doccomment">/// Updates the current search to indicate that it has search to the
    /// current position.
    ///
    /// No special care needs to be taken for reverse searches. Namely, the
    /// position given may be _less than_ the starting position of the search.
    ///
    /// # Panics
    ///
    /// This panics if no search has been started by [`Cache::search_start`].
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>search_update(<span class="kw-2">&amp;mut </span><span class="self">self</span>, at: usize) {
        <span class="kw">let </span>p =
            <span class="self">self</span>.progress.as_mut().expect(<span class="string">"no in-progress search to update"</span>);
        p.at = at;
    }

    <span class="doccomment">/// Indicates that a search has finished at the given position.
    ///
    /// # Panics
    ///
    /// This panics if no search has been started by [`Cache::search_start`].
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>search_finish(<span class="kw-2">&amp;mut </span><span class="self">self</span>, at: usize) {
        <span class="kw">let </span><span class="kw-2">mut </span>p =
            <span class="self">self</span>.progress.take().expect(<span class="string">"no in-progress search to finish"</span>);
        p.at = at;
        <span class="self">self</span>.bytes_searched += p.len();
    }

    <span class="doccomment">/// Returns the total number of bytes that have been searched since this
    /// cache was last cleared.
    ///
    /// This is useful for determining the efficiency of the cache. For
    /// example, the lazy DFA uses this value in conjunction with the
    /// [`Config::minimum_bytes_per_state`] knob to help determine whether it
    /// should quit searching.
    ///
    /// This always returns `0` if search progress isn't being tracked. Note
    /// that the lazy DFA search routines in this crate always track search
    /// progress.
    </span><span class="kw">pub fn </span>search_total_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.bytes_searched + <span class="self">self</span>.progress.as_ref().map_or(<span class="number">0</span>, |p| p.len())
    }

    <span class="doccomment">/// Returns the total number of times this cache has been cleared since it
    /// was either created or last reset.
    ///
    /// This is useful for informational purposes or if you want to change
    /// search strategies based on the number of times the cache has been
    /// cleared.
    </span><span class="kw">pub fn </span>clear_count(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.clear_count
    }

    <span class="doccomment">/// Returns the heap memory usage, in bytes, of this cache.
    ///
    /// This does **not** include the stack size used up by this cache. To
    /// compute that, use `std::mem::size_of::&lt;Cache&gt;()`.
    </span><span class="kw">pub fn </span>memory_usage(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="kw">const </span>ID_SIZE: usize = size_of::&lt;LazyStateID&gt;();
        <span class="kw">const </span>STATE_SIZE: usize = size_of::&lt;State&gt;();

        <span class="comment">// NOTE: If you make changes to the below, then
        // 'minimum_cache_capacity' should be updated correspondingly.

        </span><span class="self">self</span>.trans.len() * ID_SIZE
        + <span class="self">self</span>.starts.len() * ID_SIZE
        + <span class="self">self</span>.states.len() * STATE_SIZE
        <span class="comment">// Maps likely use more memory than this, but it's probably close.
        </span>+ <span class="self">self</span>.states_to_id.len() * (STATE_SIZE + ID_SIZE)
        + <span class="self">self</span>.sparses.memory_usage()
        + <span class="self">self</span>.stack.capacity() * ID_SIZE
        + <span class="self">self</span>.scratch_state_builder.capacity()
        <span class="comment">// Heap memory used by 'State' in both 'states' and 'states_to_id'.
        </span>+ <span class="self">self</span>.memory_usage_state
    }
}

<span class="doccomment">/// Keeps track of the progress of the current search.
///
/// This is updated via the `Cache::search_{start,update,finish}` APIs to
/// record how many bytes have been searched. This permits computing a
/// heuristic that represents the efficiency of a cache, and thus helps inform
/// whether the lazy DFA should give up or not.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">struct </span>SearchProgress {
    start: usize,
    at: usize,
}

<span class="kw">impl </span>SearchProgress {
    <span class="doccomment">/// Returns the length, in bytes, of this search so far.
    ///
    /// This automatically handles the case of a reverse search, where `at`
    /// is likely to be less than `start`.
    </span><span class="kw">fn </span>len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="kw">if </span><span class="self">self</span>.start &lt;= <span class="self">self</span>.at {
            <span class="self">self</span>.at - <span class="self">self</span>.start
        } <span class="kw">else </span>{
            <span class="self">self</span>.start - <span class="self">self</span>.at
        }
    }
}

<span class="doccomment">/// A map from states to state identifiers. When using std, we use a standard
/// hashmap, since it's a bit faster for this use case. (Other maps, like
/// one's based on FNV, have not yet been benchmarked.)
///
/// The main purpose of this map is to reuse states where possible. This won't
/// fully minimize the DFA, but it works well in a lot of cases.
</span><span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">type </span>StateMap = std::collections::HashMap&lt;State, LazyStateID&gt;;
<span class="attr">#[cfg(not(feature = <span class="string">"std"</span>))]
</span><span class="kw">type </span>StateMap = alloc::collections::BTreeMap&lt;State, LazyStateID&gt;;

<span class="doccomment">/// A type that groups methods that require the base NFA/DFA and writable
/// access to the cache.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">struct </span>Lazy&lt;<span class="lifetime">'i</span>, <span class="lifetime">'c</span>&gt; {
    dfa: <span class="kw-2">&amp;</span><span class="lifetime">'i </span>DFA,
    cache: <span class="kw-2">&amp;</span><span class="lifetime">'c </span><span class="kw-2">mut </span>Cache,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'i</span>, <span class="lifetime">'c</span>&gt; Lazy&lt;<span class="lifetime">'i</span>, <span class="lifetime">'c</span>&gt; {
    <span class="doccomment">/// Creates a new 'Lazy' wrapper for a DFA and its corresponding cache.
    </span><span class="kw">fn </span>new(dfa: <span class="kw-2">&amp;</span><span class="lifetime">'i </span>DFA, cache: <span class="kw-2">&amp;</span><span class="lifetime">'c </span><span class="kw-2">mut </span>Cache) -&gt; Lazy&lt;<span class="lifetime">'i</span>, <span class="lifetime">'c</span>&gt; {
        Lazy { dfa, cache }
    }

    <span class="doccomment">/// Return an immutable view by downgrading a writable cache to a read-only
    /// cache.
    </span><span class="kw">fn </span>as_ref&lt;<span class="lifetime">'a</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="self">self</span>) -&gt; LazyRef&lt;<span class="lifetime">'i</span>, <span class="lifetime">'a</span>&gt; {
        LazyRef::new(<span class="self">self</span>.dfa, <span class="self">self</span>.cache)
    }

    <span class="doccomment">/// This is marked as 'inline(never)' to avoid bloating methods on 'DFA'
    /// like 'next_state' and 'next_eoi_state' that are called in critical
    /// areas. The idea is to let the optimizer focus on the other areas of
    /// those methods as the hot path.
    ///
    /// Here's an example that justifies 'inline(never)'
    ///
    /// ```ignore
    /// regex-cli find match hybrid \
    ///   --cache-capacity 100000000 \
    ///   -p '\pL{100}'
    ///   all-codepoints-utf8-100x
    /// ```
    ///
    /// Where 'all-codepoints-utf8-100x' is the UTF-8 encoding of every
    /// codepoint, in sequence, repeated 100 times.
    ///
    /// With 'inline(never)' hyperfine reports 1.1s per run. With
    /// 'inline(always)', hyperfine reports 1.23s. So that's a 10% improvement.
    </span><span class="attr">#[cold]
    #[inline(never)]
    </span><span class="kw">fn </span>cache_next_state(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        <span class="kw-2">mut </span>current: LazyStateID,
        unit: alphabet::Unit,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, CacheError&gt; {
        <span class="kw">let </span>stride2 = <span class="self">self</span>.dfa.stride2();
        <span class="kw">let </span>empty_builder = <span class="self">self</span>.get_state_builder();
        <span class="kw">let </span>builder = determinize::next(
            <span class="self">self</span>.dfa.get_nfa(),
            <span class="self">self</span>.dfa.get_config().get_match_kind(),
            <span class="kw-2">&amp;mut </span><span class="self">self</span>.cache.sparses,
            <span class="kw-2">&amp;mut </span><span class="self">self</span>.cache.stack,
            <span class="kw-2">&amp;</span><span class="self">self</span>.cache.states[current.as_usize_untagged() &gt;&gt; stride2],
            unit,
            empty_builder,
        );
        <span class="kw">let </span>save_state = !<span class="self">self</span>.as_ref().state_builder_fits_in_cache(<span class="kw-2">&amp;</span>builder);
        <span class="kw">if </span>save_state {
            <span class="self">self</span>.save_state(current);
        }
        <span class="kw">let </span>next = <span class="self">self</span>.add_builder_state(builder, |sid| sid)<span class="question-mark">?</span>;
        <span class="kw">if </span>save_state {
            current = <span class="self">self</span>.saved_state_id();
        }
        <span class="comment">// This is the payoff. The next time 'next_state' is called with this
        // state and alphabet unit, it will find this transition and avoid
        // having to re-determinize this transition.
        </span><span class="self">self</span>.set_transition(current, unit, next);
        <span class="prelude-val">Ok</span>(next)
    }

    <span class="doccomment">/// Compute and cache the starting state for the given pattern ID (if
    /// present) and the starting configuration.
    ///
    /// This panics if a pattern ID is given and the DFA isn't configured to
    /// build anchored start states for each pattern.
    ///
    /// This will never return an unknown lazy state ID.
    ///
    /// If caching this state would otherwise result in a cache that has been
    /// cleared too many times, then an error is returned.
    </span><span class="attr">#[cold]
    #[inline(never)]
    </span><span class="kw">fn </span>cache_start_group(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        anchored: Anchored,
        start: Start,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, StartError&gt; {
        <span class="kw">let </span>nfa_start_id = <span class="kw">match </span>anchored {
            Anchored::No =&gt; <span class="self">self</span>.dfa.get_nfa().start_unanchored(),
            Anchored::Yes =&gt; <span class="self">self</span>.dfa.get_nfa().start_anchored(),
            Anchored::Pattern(pid) =&gt; {
                <span class="kw">if </span>!<span class="self">self</span>.dfa.get_config().get_starts_for_each_pattern() {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(StartError::unsupported_anchored(anchored));
                }
                <span class="kw">match </span><span class="self">self</span>.dfa.get_nfa().start_pattern(pid) {
                    <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="self">self</span>.as_ref().dead_id()),
                    <span class="prelude-val">Some</span>(sid) =&gt; sid,
                }
            }
        };

        <span class="kw">let </span>id = <span class="self">self
            </span>.cache_start_one(nfa_start_id, start)
            .map_err(StartError::cache)<span class="question-mark">?</span>;
        <span class="self">self</span>.set_start_state(anchored, start, id);
        <span class="prelude-val">Ok</span>(id)
    }

    <span class="doccomment">/// Compute and cache the starting state for the given NFA state ID and the
    /// starting configuration. The NFA state ID might be one of the following:
    ///
    /// 1) An unanchored start state to match any pattern.
    /// 2) An anchored start state to match any pattern.
    /// 3) An anchored start state for a particular pattern.
    ///
    /// This will never return an unknown lazy state ID.
    ///
    /// If caching this state would otherwise result in a cache that has been
    /// cleared too many times, then an error is returned.
    </span><span class="kw">fn </span>cache_start_one(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        nfa_start_id: NFAStateID,
        start: Start,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, CacheError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>builder_matches = <span class="self">self</span>.get_state_builder().into_matches();
        determinize::set_lookbehind_from_start(
            <span class="self">self</span>.dfa.get_nfa(),
            <span class="kw-2">&amp;</span>start,
            <span class="kw-2">&amp;mut </span>builder_matches,
        );
        <span class="self">self</span>.cache.sparses.set1.clear();
        determinize::epsilon_closure(
            <span class="self">self</span>.dfa.get_nfa(),
            nfa_start_id,
            builder_matches.look_have(),
            <span class="kw-2">&amp;mut </span><span class="self">self</span>.cache.stack,
            <span class="kw-2">&amp;mut </span><span class="self">self</span>.cache.sparses.set1,
        );
        <span class="kw">let </span><span class="kw-2">mut </span>builder = builder_matches.into_nfa();
        determinize::add_nfa_states(
            <span class="kw-2">&amp;</span><span class="self">self</span>.dfa.get_nfa(),
            <span class="kw-2">&amp;</span><span class="self">self</span>.cache.sparses.set1,
            <span class="kw-2">&amp;mut </span>builder,
        );
        <span class="kw">let </span>tag_starts = <span class="self">self</span>.dfa.get_config().get_specialize_start_states();
        <span class="self">self</span>.add_builder_state(builder, |id| {
            <span class="kw">if </span>tag_starts {
                id.to_start()
            } <span class="kw">else </span>{
                id
            }
        })
    }

    <span class="doccomment">/// Either add the given builder state to this cache, or return an ID to an
    /// equivalent state already in this cache.
    ///
    /// In the case where no equivalent state exists, the idmap function given
    /// may be used to transform the identifier allocated. This is useful if
    /// the caller needs to tag the ID with additional information.
    ///
    /// This will never return an unknown lazy state ID.
    ///
    /// If caching this state would otherwise result in a cache that has been
    /// cleared too many times, then an error is returned.
    </span><span class="kw">fn </span>add_builder_state(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        builder: StateBuilderNFA,
        idmap: <span class="kw">impl </span>Fn(LazyStateID) -&gt; LazyStateID,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, CacheError&gt; {
        <span class="kw">if let </span><span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span>cached_id) =
            <span class="self">self</span>.cache.states_to_id.get(builder.as_bytes())
        {
            <span class="comment">// Since we have a cached state, put the constructed state's
            // memory back into our scratch space, so that it can be reused.
            </span><span class="self">self</span>.put_state_builder(builder);
            <span class="kw">return </span><span class="prelude-val">Ok</span>(cached_id);
        }
        <span class="kw">let </span>result = <span class="self">self</span>.add_state(builder.to_state(), idmap);
        <span class="self">self</span>.put_state_builder(builder);
        result
    }

    <span class="doccomment">/// Allocate a new state ID and add the given state to this cache.
    ///
    /// The idmap function given may be used to transform the identifier
    /// allocated. This is useful if the caller needs to tag the ID with
    /// additional information.
    ///
    /// This will never return an unknown lazy state ID.
    ///
    /// If caching this state would otherwise result in a cache that has been
    /// cleared too many times, then an error is returned.
    </span><span class="kw">fn </span>add_state(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        state: State,
        idmap: <span class="kw">impl </span>Fn(LazyStateID) -&gt; LazyStateID,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, CacheError&gt; {
        <span class="kw">if </span>!<span class="self">self</span>.as_ref().state_fits_in_cache(<span class="kw-2">&amp;</span>state) {
            <span class="self">self</span>.try_clear_cache()<span class="question-mark">?</span>;
        }
        <span class="comment">// It's important for this to come second, since the above may clear
        // the cache. If we clear the cache after ID generation, then the ID
        // is likely bunk since it would have been generated based on a larger
        // transition table.
        </span><span class="kw">let </span><span class="kw-2">mut </span>id = idmap(<span class="self">self</span>.next_state_id()<span class="question-mark">?</span>);
        <span class="kw">if </span>state.is_match() {
            id = id.to_match();
        }
        <span class="comment">// Add room in the transition table. Since this is a fresh state, all
        // of its transitions are unknown.
        </span><span class="self">self</span>.cache.trans.extend(
            iter::repeat(<span class="self">self</span>.as_ref().unknown_id()).take(<span class="self">self</span>.dfa.stride()),
        );
        <span class="comment">// When we add a sentinel state, we never want to set any quit
        // transitions. Technically, this is harmless, since sentinel states
        // have all of their transitions set to loop back to themselves. But
        // when creating sentinel states before the quit sentinel state,
        // this will try to call 'set_transition' on a state ID that doesn't
        // actually exist yet, which isn't allowed. So we just skip doing so
        // entirely.
        </span><span class="kw">if </span>!<span class="self">self</span>.dfa.quitset.is_empty() &amp;&amp; !<span class="self">self</span>.as_ref().is_sentinel(id) {
            <span class="kw">let </span>quit_id = <span class="self">self</span>.as_ref().quit_id();
            <span class="kw">for </span>b <span class="kw">in </span><span class="self">self</span>.dfa.quitset.iter() {
                <span class="self">self</span>.set_transition(id, alphabet::Unit::u8(b), quit_id);
            }
        }
        <span class="self">self</span>.cache.memory_usage_state += state.memory_usage();
        <span class="self">self</span>.cache.states.push(state.clone());
        <span class="self">self</span>.cache.states_to_id.insert(state, id);
        <span class="prelude-val">Ok</span>(id)
    }

    <span class="doccomment">/// Allocate a new state ID.
    ///
    /// This will never return an unknown lazy state ID.
    ///
    /// If caching this state would otherwise result in a cache that has been
    /// cleared too many times, then an error is returned.
    </span><span class="kw">fn </span>next_state_id(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, CacheError&gt; {
        <span class="kw">let </span>sid = <span class="kw">match </span>LazyStateID::new(<span class="self">self</span>.cache.trans.len()) {
            <span class="prelude-val">Ok</span>(sid) =&gt; sid,
            <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; {
                <span class="self">self</span>.try_clear_cache()<span class="question-mark">?</span>;
                <span class="comment">// This has to pass since we check that ID capacity at
                // construction time can fit at least MIN_STATES states.
                </span>LazyStateID::new(<span class="self">self</span>.cache.trans.len()).unwrap()
            }
        };
        <span class="prelude-val">Ok</span>(sid)
    }

    <span class="doccomment">/// Attempt to clear the cache used by this lazy DFA.
    ///
    /// If clearing the cache exceeds the minimum number of required cache
    /// clearings, then this will return a cache error. In this case,
    /// callers should bubble this up as the cache can't be used until it is
    /// reset. Implementations of search should convert this error into a
    /// [`MatchError::gave_up`].
    ///
    /// If 'self.state_saver' is set to save a state, then this state is
    /// persisted through cache clearing. Otherwise, the cache is returned to
    /// its state after initialization with two exceptions: its clear count
    /// is incremented and some of its memory likely has additional capacity.
    /// That is, clearing a cache does _not_ release memory.
    ///
    /// Otherwise, any lazy state ID generated by the cache prior to resetting
    /// it is invalid after the reset.
    </span><span class="kw">fn </span>try_clear_cache(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), CacheError&gt; {
        <span class="kw">let </span>c = <span class="self">self</span>.dfa.get_config();
        <span class="kw">if let </span><span class="prelude-val">Some</span>(min_count) = c.get_minimum_cache_clear_count() {
            <span class="kw">if </span><span class="self">self</span>.cache.clear_count &gt;= min_count {
                <span class="kw">if let </span><span class="prelude-val">Some</span>(min_bytes_per) = c.get_minimum_bytes_per_state() {
                    <span class="kw">let </span>len = <span class="self">self</span>.cache.search_total_len();
                    <span class="kw">let </span>min_bytes =
                        min_bytes_per.saturating_mul(<span class="self">self</span>.cache.states.len());
                    <span class="comment">// If we've searched 0 bytes then probably something has
                    // gone wrong and the lazy DFA search implementation isn't
                    // correctly updating the search progress state.
                    </span><span class="kw">if </span>len == <span class="number">0 </span>{
                        <span class="macro">trace!</span>(
                            <span class="string">"number of bytes searched is 0, but \
                             a minimum bytes per state searched ({}) is \
                             enabled, maybe Cache::search_update \
                             is not being used?"</span>,
                            min_bytes_per,
                        );
                    }
                    <span class="kw">if </span>len &lt; min_bytes {
                        <span class="macro">trace!</span>(
                            <span class="string">"lazy DFA cache has been cleared {} times, \
                             which exceeds the limit of {}, \
                             AND its bytes searched per state is less \
                             than the configured minimum of {}, \
                             therefore lazy DFA is giving up \
                             (bytes searched since cache clear = {}, \
                              number of states = {})"</span>,
                            <span class="self">self</span>.cache.clear_count,
                            min_count,
                            min_bytes_per,
                            len,
                            <span class="self">self</span>.cache.states.len(),
                        );
                        <span class="kw">return </span><span class="prelude-val">Err</span>(CacheError::bad_efficiency());
                    } <span class="kw">else </span>{
                        <span class="macro">trace!</span>(
                            <span class="string">"lazy DFA cache has been cleared {} times, \
                             which exceeds the limit of {}, \
                             AND its bytes searched per state is greater \
                             than the configured minimum of {}, \
                             therefore lazy DFA is continuing! \
                             (bytes searched since cache clear = {}, \
                              number of states = {})"</span>,
                            <span class="self">self</span>.cache.clear_count,
                            min_count,
                            min_bytes_per,
                            len,
                            <span class="self">self</span>.cache.states.len(),
                        );
                    }
                } <span class="kw">else </span>{
                    <span class="macro">trace!</span>(
                        <span class="string">"lazy DFA cache has been cleared {} times, \
                         which exceeds the limit of {}, \
                         since there is no configured bytes per state \
                         minimum, lazy DFA is giving up"</span>,
                        <span class="self">self</span>.cache.clear_count,
                        min_count,
                    );
                    <span class="kw">return </span><span class="prelude-val">Err</span>(CacheError::too_many_cache_clears());
                }
            }
        }
        <span class="self">self</span>.clear_cache();
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Clears _and_ resets the cache. Resetting the cache means that no
    /// states are persisted and the clear count is reset to 0. No heap memory
    /// is released.
    ///
    /// Note that the caller may reset a cache with a different DFA than what
    /// it was created from. In which case, the cache can now be used with the
    /// new DFA (and not the old DFA).
    </span><span class="kw">fn </span>reset_cache(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.cache.state_saver = StateSaver::none();
        <span class="self">self</span>.clear_cache();
        <span class="comment">// If a new DFA is used, it might have a different number of NFA
        // states, so we need to make sure our sparse sets have the appropriate
        // size.
        </span><span class="self">self</span>.cache.sparses.resize(<span class="self">self</span>.dfa.get_nfa().states().len());
        <span class="self">self</span>.cache.clear_count = <span class="number">0</span>;
        <span class="self">self</span>.cache.progress = <span class="prelude-val">None</span>;
    }

    <span class="doccomment">/// Clear the cache used by this lazy DFA.
    ///
    /// If 'self.state_saver' is set to save a state, then this state is
    /// persisted through cache clearing. Otherwise, the cache is returned to
    /// its state after initialization with two exceptions: its clear count
    /// is incremented and some of its memory likely has additional capacity.
    /// That is, clearing a cache does _not_ release memory.
    ///
    /// Otherwise, any lazy state ID generated by the cache prior to resetting
    /// it is invalid after the reset.
    </span><span class="kw">fn </span>clear_cache(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.cache.trans.clear();
        <span class="self">self</span>.cache.starts.clear();
        <span class="self">self</span>.cache.states.clear();
        <span class="self">self</span>.cache.states_to_id.clear();
        <span class="self">self</span>.cache.memory_usage_state = <span class="number">0</span>;
        <span class="self">self</span>.cache.clear_count += <span class="number">1</span>;
        <span class="self">self</span>.cache.bytes_searched = <span class="number">0</span>;
        <span class="kw">if let </span><span class="prelude-val">Some</span>(<span class="kw-2">ref mut </span>progress) = <span class="self">self</span>.cache.progress {
            progress.start = progress.at;
        }
        <span class="macro">trace!</span>(
            <span class="string">"lazy DFA cache has been cleared (count: {})"</span>,
            <span class="self">self</span>.cache.clear_count
        );
        <span class="self">self</span>.init_cache();
        <span class="comment">// If the state we want to save is one of the sentinel
        // (unknown/dead/quit) states, then 'init_cache' adds those back, and
        // their identifier values remains invariant. So there's no need to add
        // it again. (And indeed, doing so would be incorrect!)
        </span><span class="kw">if let </span><span class="prelude-val">Some</span>((old_id, state)) = <span class="self">self</span>.cache.state_saver.take_to_save() {
            <span class="comment">// If the state is one of the special sentinel states, then it is
            // automatically added by cache initialization and its ID always
            // remains the same. With that said, this should never occur since
            // the sentinel states are all loop states back to themselves. So
            // we should never be in a position where we're attempting to save
            // a sentinel state since we never compute transitions out of a
            // sentinel state.
            </span><span class="macro">assert!</span>(
                !<span class="self">self</span>.as_ref().is_sentinel(old_id),
                <span class="string">"cannot save sentinel state"
            </span>);
            <span class="kw">let </span>new_id = <span class="self">self
                </span>.add_state(state, |id| {
                    <span class="kw">if </span>old_id.is_start() {
                        <span class="comment">// We don't need to consult the
                        // 'specialize_start_states' config knob here, because
                        // if it's disabled, old_id.is_start() will never
                        // return true.
                        </span>id.to_start()
                    } <span class="kw">else </span>{
                        id
                    }
                })
                <span class="comment">// The unwrap here is OK because lazy DFA creation ensures that
                // we have room in the cache to add MIN_STATES states. Since
                // 'init_cache' above adds 3, this adds a 4th.
                </span>.expect(<span class="string">"adding one state after cache clear must work"</span>);
            <span class="self">self</span>.cache.state_saver = StateSaver::Saved(new_id);
        }
    }

    <span class="doccomment">/// Initialize this cache from emptiness to a place where it can be used
    /// for search.
    ///
    /// This is called both at cache creation time and after the cache has been
    /// cleared.
    ///
    /// Primarily, this adds the three sentinel states and allocates some
    /// initial memory.
    </span><span class="kw">fn </span>init_cache(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="comment">// Why multiply by 2 here? Because we make room for both the unanchored
        // and anchored start states. Unanchored is first and then anchored.
        </span><span class="kw">let </span><span class="kw-2">mut </span>starts_len = Start::len().checked_mul(<span class="number">2</span>).unwrap();
        <span class="comment">// ... but if we also want start states for every pattern, we make room
        // for that too.
        </span><span class="kw">if </span><span class="self">self</span>.dfa.get_config().get_starts_for_each_pattern() {
            starts_len += Start::len() * <span class="self">self</span>.dfa.pattern_len();
        }
        <span class="self">self</span>.cache
            .starts
            .extend(iter::repeat(<span class="self">self</span>.as_ref().unknown_id()).take(starts_len));
        <span class="comment">// This is the set of NFA states that corresponds to each of our three
        // sentinel states: the empty set.
        </span><span class="kw">let </span>dead = State::dead();
        <span class="comment">// This sets up some states that we use as sentinels that are present
        // in every DFA. While it would be technically possible to implement
        // this DFA without explicitly putting these states in the transition
        // table, this is convenient to do to make `next_state` correct for all
        // valid state IDs without needing explicit conditionals to special
        // case these sentinel states.
        //
        // All three of these states are "dead" states. That is, all of
        // them transition only to themselves. So once you enter one of
        // these states, it's impossible to leave them. Thus, any correct
        // search routine must explicitly check for these state types. (Sans
        // `unknown`, since that is only used internally to represent missing
        // states.)
        </span><span class="kw">let </span>unk_id =
            <span class="self">self</span>.add_state(dead.clone(), |id| id.to_unknown()).unwrap();
        <span class="kw">let </span>dead_id = <span class="self">self</span>.add_state(dead.clone(), |id| id.to_dead()).unwrap();
        <span class="kw">let </span>quit_id = <span class="self">self</span>.add_state(dead.clone(), |id| id.to_quit()).unwrap();
        <span class="macro">assert_eq!</span>(unk_id, <span class="self">self</span>.as_ref().unknown_id());
        <span class="macro">assert_eq!</span>(dead_id, <span class="self">self</span>.as_ref().dead_id());
        <span class="macro">assert_eq!</span>(quit_id, <span class="self">self</span>.as_ref().quit_id());
        <span class="comment">// The idea here is that if you start in an unknown/dead/quit state and
        // try to transition on them, then you should end up where you started.
        </span><span class="self">self</span>.set_all_transitions(unk_id, unk_id);
        <span class="self">self</span>.set_all_transitions(dead_id, dead_id);
        <span class="self">self</span>.set_all_transitions(quit_id, quit_id);
        <span class="comment">// All of these states are technically equivalent from the FSM
        // perspective, so putting all three of them in the cache isn't
        // possible. (They are distinct merely because we use their
        // identifiers as sentinels to mean something, as indicated by the
        // names.) Moreover, we wouldn't want to do that. Unknown and quit
        // states are special in that they are artificial constructions
        // this implementation. But dead states are a natural part of
        // determinization. When you reach a point in the NFA where you cannot
        // go anywhere else, a dead state will naturally arise and we MUST
        // reuse the canonical dead state that we've created here. Why? Because
        // it is the state ID that tells the search routine whether a state is
        // dead or not, and thus, whether to stop the search. Having a bunch of
        // distinct dead states would be quite wasteful!
        </span><span class="self">self</span>.cache.states_to_id.insert(dead, dead_id);
    }

    <span class="doccomment">/// Save the state corresponding to the ID given such that the state
    /// persists through a cache clearing.
    ///
    /// While the state may persist, the ID may not. In order to discover the
    /// new state ID, one must call 'saved_state_id' after a cache clearing.
    </span><span class="kw">fn </span>save_state(<span class="kw-2">&amp;mut </span><span class="self">self</span>, id: LazyStateID) {
        <span class="kw">let </span>state = <span class="self">self</span>.as_ref().get_cached_state(id).clone();
        <span class="self">self</span>.cache.state_saver = StateSaver::ToSave { id, state };
    }

    <span class="doccomment">/// Returns the updated lazy state ID for a state that was persisted
    /// through a cache clearing.
    ///
    /// It is only correct to call this routine when both a state has been
    /// saved and the cache has just been cleared. Otherwise, this panics.
    </span><span class="kw">fn </span>saved_state_id(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; LazyStateID {
        <span class="self">self</span>.cache
            .state_saver
            .take_saved()
            .expect(<span class="string">"state saver does not have saved state ID"</span>)
    }

    <span class="doccomment">/// Set all transitions on the state 'from' to 'to'.
    </span><span class="kw">fn </span>set_all_transitions(<span class="kw-2">&amp;mut </span><span class="self">self</span>, from: LazyStateID, to: LazyStateID) {
        <span class="kw">for </span>unit <span class="kw">in </span><span class="self">self</span>.dfa.classes.representatives(..) {
            <span class="self">self</span>.set_transition(from, unit, to);
        }
    }

    <span class="doccomment">/// Set the transition on 'from' for 'unit' to 'to'.
    ///
    /// This panics if either 'from' or 'to' is invalid.
    ///
    /// All unit values are OK.
    </span><span class="kw">fn </span>set_transition(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        from: LazyStateID,
        unit: alphabet::Unit,
        to: LazyStateID,
    ) {
        <span class="macro">assert!</span>(<span class="self">self</span>.as_ref().is_valid(from), <span class="string">"invalid 'from' id: {:?}"</span>, from);
        <span class="macro">assert!</span>(<span class="self">self</span>.as_ref().is_valid(to), <span class="string">"invalid 'to' id: {:?}"</span>, to);
        <span class="kw">let </span>offset =
            from.as_usize_untagged() + <span class="self">self</span>.dfa.classes.get_by_unit(unit);
        <span class="self">self</span>.cache.trans[offset] = to;
    }

    <span class="doccomment">/// Set the start ID for the given pattern ID (if given) and starting
    /// configuration to the ID given.
    ///
    /// This panics if 'id' is not valid or if a pattern ID is given and
    /// 'starts_for_each_pattern' is not enabled.
    </span><span class="kw">fn </span>set_start_state(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        anchored: Anchored,
        start: Start,
        id: LazyStateID,
    ) {
        <span class="macro">assert!</span>(<span class="self">self</span>.as_ref().is_valid(id));
        <span class="kw">let </span>start_index = start.as_usize();
        <span class="kw">let </span>index = <span class="kw">match </span>anchored {
            Anchored::No =&gt; start_index,
            Anchored::Yes =&gt; Start::len() + start_index,
            Anchored::Pattern(pid) =&gt; {
                <span class="macro">assert!</span>(
                    <span class="self">self</span>.dfa.get_config().get_starts_for_each_pattern(),
                    <span class="string">"attempted to search for a specific pattern \
                     without enabling starts_for_each_pattern"</span>,
                );
                <span class="kw">let </span>pid = pid.as_usize();
                (<span class="number">2 </span>* Start::len()) + (Start::len() * pid) + start_index
            }
        };
        <span class="self">self</span>.cache.starts[index] = id;
    }

    <span class="doccomment">/// Returns a state builder from this DFA that might have existing
    /// capacity. This helps avoid allocs in cases where a state is built that
    /// turns out to already be cached.
    ///
    /// Callers must put the state builder back with 'put_state_builder',
    /// otherwise the allocation reuse won't work.
    </span><span class="kw">fn </span>get_state_builder(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; StateBuilderEmpty {
        core::mem::replace(
            <span class="kw-2">&amp;mut </span><span class="self">self</span>.cache.scratch_state_builder,
            StateBuilderEmpty::new(),
        )
    }

    <span class="doccomment">/// Puts the given state builder back into this DFA for reuse.
    ///
    /// Note that building a 'State' from a builder always creates a new alloc,
    /// so callers should always put the builder back.
    </span><span class="kw">fn </span>put_state_builder(<span class="kw-2">&amp;mut </span><span class="self">self</span>, builder: StateBuilderNFA) {
        <span class="kw">let _ </span>= core::mem::replace(
            <span class="kw-2">&amp;mut </span><span class="self">self</span>.cache.scratch_state_builder,
            builder.clear(),
        );
    }
}

<span class="doccomment">/// A type that groups methods that require the base NFA/DFA and read-only
/// access to the cache.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">struct </span>LazyRef&lt;<span class="lifetime">'i</span>, <span class="lifetime">'c</span>&gt; {
    dfa: <span class="kw-2">&amp;</span><span class="lifetime">'i </span>DFA,
    cache: <span class="kw-2">&amp;</span><span class="lifetime">'c </span>Cache,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'i</span>, <span class="lifetime">'c</span>&gt; LazyRef&lt;<span class="lifetime">'i</span>, <span class="lifetime">'c</span>&gt; {
    <span class="doccomment">/// Creates a new 'Lazy' wrapper for a DFA and its corresponding cache.
    </span><span class="kw">fn </span>new(dfa: <span class="kw-2">&amp;</span><span class="lifetime">'i </span>DFA, cache: <span class="kw-2">&amp;</span><span class="lifetime">'c </span>Cache) -&gt; LazyRef&lt;<span class="lifetime">'i</span>, <span class="lifetime">'c</span>&gt; {
        LazyRef { dfa, cache }
    }

    <span class="doccomment">/// Return the ID of the start state for the given configuration.
    ///
    /// If the start state has not yet been computed, then this returns an
    /// unknown lazy state ID.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"perf-inline"</span>, inline(always))]
    </span><span class="kw">fn </span>get_cached_start_id(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        anchored: Anchored,
        start: Start,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, StartError&gt; {
        <span class="kw">let </span>start_index = start.as_usize();
        <span class="kw">let </span>index = <span class="kw">match </span>anchored {
            Anchored::No =&gt; start_index,
            Anchored::Yes =&gt; Start::len() + start_index,
            Anchored::Pattern(pid) =&gt; {
                <span class="kw">if </span>!<span class="self">self</span>.dfa.get_config().get_starts_for_each_pattern() {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(StartError::unsupported_anchored(anchored));
                }
                <span class="kw">if </span>pid.as_usize() &gt;= <span class="self">self</span>.dfa.pattern_len() {
                    <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="self">self</span>.dead_id());
                }
                (<span class="number">2 </span>* Start::len())
                    + (Start::len() * pid.as_usize())
                    + start_index
            }
        };
        <span class="prelude-val">Ok</span>(<span class="self">self</span>.cache.starts[index])
    }

    <span class="doccomment">/// Return the cached NFA/DFA powerset state for the given ID.
    ///
    /// This panics if the given ID does not address a valid state.
    </span><span class="kw">fn </span>get_cached_state(<span class="kw-2">&amp;</span><span class="self">self</span>, sid: LazyStateID) -&gt; <span class="kw-2">&amp;</span>State {
        <span class="kw">let </span>index = sid.as_usize_untagged() &gt;&gt; <span class="self">self</span>.dfa.stride2();
        <span class="kw-2">&amp;</span><span class="self">self</span>.cache.states[index]
    }

    <span class="doccomment">/// Returns true if and only if the given ID corresponds to a "sentinel"
    /// state.
    ///
    /// A sentinel state is a state that signifies a special condition of
    /// search, and where every transition maps back to itself. See LazyStateID
    /// for more details. Note that start and match states are _not_ sentinels
    /// since they may otherwise be real states with non-trivial transitions.
    /// The purposes of sentinel states is purely to indicate something. Their
    /// transitions are not meant to be followed.
    </span><span class="kw">fn </span>is_sentinel(<span class="kw-2">&amp;</span><span class="self">self</span>, id: LazyStateID) -&gt; bool {
        id == <span class="self">self</span>.unknown_id() || id == <span class="self">self</span>.dead_id() || id == <span class="self">self</span>.quit_id()
    }

    <span class="doccomment">/// Returns the ID of the unknown state for this lazy DFA.
    </span><span class="kw">fn </span>unknown_id(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LazyStateID {
        <span class="comment">// This unwrap is OK since 0 is always a valid state ID.
        </span>LazyStateID::new(<span class="number">0</span>).unwrap().to_unknown()
    }

    <span class="doccomment">/// Returns the ID of the dead state for this lazy DFA.
    </span><span class="kw">fn </span>dead_id(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LazyStateID {
        <span class="comment">// This unwrap is OK since the maximum value here is 1 * 512 = 512,
        // which is &lt;= 2047 (the maximum state ID on 16-bit systems). Where
        // 512 is the worst case for our equivalence classes (every byte is a
        // distinct class).
        </span>LazyStateID::new(<span class="number">1 </span>&lt;&lt; <span class="self">self</span>.dfa.stride2()).unwrap().to_dead()
    }

    <span class="doccomment">/// Returns the ID of the quit state for this lazy DFA.
    </span><span class="kw">fn </span>quit_id(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LazyStateID {
        <span class="comment">// This unwrap is OK since the maximum value here is 2 * 512 = 1024,
        // which is &lt;= 2047 (the maximum state ID on 16-bit systems). Where
        // 512 is the worst case for our equivalence classes (every byte is a
        // distinct class).
        </span>LazyStateID::new(<span class="number">2 </span>&lt;&lt; <span class="self">self</span>.dfa.stride2()).unwrap().to_quit()
    }

    <span class="doccomment">/// Returns true if and only if the given ID is valid.
    ///
    /// An ID is valid if it is both a valid index into the transition table
    /// and is a multiple of the DFA's stride.
    </span><span class="kw">fn </span>is_valid(<span class="kw-2">&amp;</span><span class="self">self</span>, id: LazyStateID) -&gt; bool {
        <span class="kw">let </span>id = id.as_usize_untagged();
        id &lt; <span class="self">self</span>.cache.trans.len() &amp;&amp; id % <span class="self">self</span>.dfa.stride() == <span class="number">0
    </span>}

    <span class="doccomment">/// Returns true if adding the state given would fit in this cache.
    </span><span class="kw">fn </span>state_fits_in_cache(<span class="kw-2">&amp;</span><span class="self">self</span>, state: <span class="kw-2">&amp;</span>State) -&gt; bool {
        <span class="kw">let </span>needed = <span class="self">self</span>.cache.memory_usage()
            + <span class="self">self</span>.memory_usage_for_one_more_state(state.memory_usage());
        <span class="macro">trace!</span>(
            <span class="string">"lazy DFA cache capacity check: {:?} ?&lt;=? {:?}"</span>,
            needed,
            <span class="self">self</span>.dfa.cache_capacity
        );
        needed &lt;= <span class="self">self</span>.dfa.cache_capacity
    }

    <span class="doccomment">/// Returns true if adding the state to be built by the given builder would
    /// fit in this cache.
    </span><span class="kw">fn </span>state_builder_fits_in_cache(<span class="kw-2">&amp;</span><span class="self">self</span>, state: <span class="kw-2">&amp;</span>StateBuilderNFA) -&gt; bool {
        <span class="kw">let </span>needed = <span class="self">self</span>.cache.memory_usage()
            + <span class="self">self</span>.memory_usage_for_one_more_state(state.as_bytes().len());
        needed &lt;= <span class="self">self</span>.dfa.cache_capacity
    }

    <span class="doccomment">/// Returns the additional memory usage, in bytes, required to add one more
    /// state to this cache. The given size should be the heap size, in bytes,
    /// that would be used by the new state being added.
    </span><span class="kw">fn </span>memory_usage_for_one_more_state(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        state_heap_size: usize,
    ) -&gt; usize {
        <span class="kw">const </span>ID_SIZE: usize = size_of::&lt;LazyStateID&gt;();
        <span class="kw">const </span>STATE_SIZE: usize = size_of::&lt;State&gt;();

        <span class="self">self</span>.dfa.stride() * ID_SIZE <span class="comment">// additional space needed in trans table
        </span>+ STATE_SIZE <span class="comment">// space in cache.states
        </span>+ (STATE_SIZE + ID_SIZE) <span class="comment">// space in cache.states_to_id
        </span>+ state_heap_size <span class="comment">// heap memory used by state itself
    </span>}
}

<span class="doccomment">/// A simple type that encapsulates the saving of a state ID through a cache
/// clearing.
///
/// A state ID can be marked for saving with ToSave, while a state ID can be
/// saved itself with Saved.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">enum </span>StateSaver {
    <span class="doccomment">/// An empty state saver. In this case, no states (other than the special
    /// sentinel states) are preserved after clearing the cache.
    </span><span class="prelude-val">None</span>,
    <span class="doccomment">/// An ID of a state (and the state itself) that should be preserved after
    /// the lazy DFA's cache has been cleared. After clearing, the updated ID
    /// is stored in 'Saved' since it may have changed.
    </span>ToSave { id: LazyStateID, state: State },
    <span class="doccomment">/// An ID that of a state that has been persisted through a lazy DFA
    /// cache clearing. The ID recorded here corresponds to an ID that was
    /// once marked as ToSave. The IDs are likely not equivalent even though
    /// the states they point to are.
    </span>Saved(LazyStateID),
}

<span class="kw">impl </span>StateSaver {
    <span class="doccomment">/// Create an empty state saver.
    </span><span class="kw">fn </span>none() -&gt; StateSaver {
        StateSaver::None
    }

    <span class="doccomment">/// Replace this state saver with an empty saver, and if this saver is a
    /// request to save a state, return that request.
    </span><span class="kw">fn </span>take_to_save(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;(LazyStateID, State)&gt; {
        <span class="kw">match </span>core::mem::replace(<span class="self">self</span>, StateSaver::None) {
            StateSaver::None | StateSaver::Saved(<span class="kw">_</span>) =&gt; <span class="prelude-val">None</span>,
            StateSaver::ToSave { id, state } =&gt; <span class="prelude-val">Some</span>((id, state)),
        }
    }

    <span class="doccomment">/// Replace this state saver with an empty saver, and if this saver is a
    /// saved state (or a request to save a state), return that state's ID.
    ///
    /// The idea here is that a request to save a state isn't necessarily
    /// honored because it might not be needed. e.g., Some higher level code
    /// might request a state to be saved on the off chance that the cache gets
    /// cleared when a new state is added at a lower level. But if that new
    /// state is never added, then the cache is never cleared and the state and
    /// its ID remain unchanged.
    </span><span class="kw">fn </span>take_saved(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;LazyStateID&gt; {
        <span class="kw">match </span>core::mem::replace(<span class="self">self</span>, StateSaver::None) {
            StateSaver::None =&gt; <span class="prelude-val">None</span>,
            StateSaver::Saved(id) | StateSaver::ToSave { id, .. } =&gt; <span class="prelude-val">Some</span>(id),
        }
    }
}

<span class="doccomment">/// The configuration used for building a lazy DFA.
///
/// As a convenience, [`DFA::config`] is an alias for [`Config::new`]. The
/// advantage of the former is that it often lets you avoid importing the
/// `Config` type directly.
///
/// A lazy DFA configuration is a simple data object that is typically used
/// with [`Builder::configure`].
///
/// The default configuration guarantees that a search will never return a
/// "gave up" or "quit" error, although it is possible for a search to fail
/// if [`Config::starts_for_each_pattern`] wasn't enabled (which it is not by
/// default) and an [`Anchored::Pattern`] mode is requested via [`Input`].
</span><span class="attr">#[derive(Clone, Debug, Default)]
</span><span class="kw">pub struct </span>Config {
    <span class="comment">// As with other configuration types in this crate, we put all our knobs
    // in options so that we can distinguish between "default" and "not set."
    // This makes it possible to easily combine multiple configurations
    // without default values overwriting explicitly specified values. See the
    // 'overwrite' method.
    //
    // For docs on the fields below, see the corresponding method setters.
    </span>match_kind: <span class="prelude-ty">Option</span>&lt;MatchKind&gt;,
    pre: <span class="prelude-ty">Option</span>&lt;<span class="prelude-ty">Option</span>&lt;Prefilter&gt;&gt;,
    starts_for_each_pattern: <span class="prelude-ty">Option</span>&lt;bool&gt;,
    byte_classes: <span class="prelude-ty">Option</span>&lt;bool&gt;,
    unicode_word_boundary: <span class="prelude-ty">Option</span>&lt;bool&gt;,
    quitset: <span class="prelude-ty">Option</span>&lt;ByteSet&gt;,
    specialize_start_states: <span class="prelude-ty">Option</span>&lt;bool&gt;,
    cache_capacity: <span class="prelude-ty">Option</span>&lt;usize&gt;,
    skip_cache_capacity_check: <span class="prelude-ty">Option</span>&lt;bool&gt;,
    minimum_cache_clear_count: <span class="prelude-ty">Option</span>&lt;<span class="prelude-ty">Option</span>&lt;usize&gt;&gt;,
    minimum_bytes_per_state: <span class="prelude-ty">Option</span>&lt;<span class="prelude-ty">Option</span>&lt;usize&gt;&gt;,
}

<span class="kw">impl </span>Config {
    <span class="doccomment">/// Return a new default lazy DFA builder configuration.
    </span><span class="kw">pub fn </span>new() -&gt; Config {
        Config::default()
    }

    <span class="doccomment">/// Set the desired match semantics.
    ///
    /// The default is [`MatchKind::LeftmostFirst`], which corresponds to the
    /// match semantics of Perl-like regex engines. That is, when multiple
    /// patterns would match at the same leftmost position, the pattern that
    /// appears first in the concrete syntax is chosen.
    ///
    /// Currently, the only other kind of match semantics supported is
    /// [`MatchKind::All`]. This corresponds to classical DFA construction
    /// where all possible matches are added to the lazy DFA.
    ///
    /// Typically, `All` is used when one wants to execute an overlapping
    /// search and `LeftmostFirst` otherwise. In particular, it rarely makes
    /// sense to use `All` with the various "leftmost" find routines, since the
    /// leftmost routines depend on the `LeftmostFirst` automata construction
    /// strategy. Specifically, `LeftmostFirst` adds dead states to the
    /// lazy DFA as a way to terminate the search and report a match.
    /// `LeftmostFirst` also supports non-greedy matches using this strategy
    /// where as `All` does not.
    ///
    /// # Example: overlapping search
    ///
    /// This example shows the typical use of `MatchKind::All`, which is to
    /// report overlapping matches.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{
    ///     hybrid::dfa::{DFA, OverlappingState},
    ///     HalfMatch, Input, MatchKind,
    /// };
    ///
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().match_kind(MatchKind::All))
    ///     .build_many(&amp;[r"\w+$", r"\S+$"])?;
    /// let mut cache = dfa.create_cache();
    /// let haystack = "@foo";
    /// let mut state = OverlappingState::start();
    ///
    /// let expected = Some(HalfMatch::must(1, 4));
    /// dfa.try_search_overlapping_fwd(
    ///     &amp;mut cache, &amp;Input::new(haystack), &amp;mut state,
    /// )?;
    /// assert_eq!(expected, state.get_match());
    ///
    /// // The first pattern also matches at the same position, so re-running
    /// // the search will yield another match. Notice also that the first
    /// // pattern is returned after the second. This is because the second
    /// // pattern begins its match before the first, is therefore an earlier
    /// // match and is thus reported first.
    /// let expected = Some(HalfMatch::must(0, 4));
    /// dfa.try_search_overlapping_fwd(
    ///     &amp;mut cache, &amp;Input::new(haystack), &amp;mut state,
    /// )?;
    /// assert_eq!(expected, state.get_match());
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// # Example: reverse automaton to find start of match
    ///
    /// Another example for using `MatchKind::All` is for constructing a
    /// reverse automaton to find the start of a match. `All` semantics are
    /// used for this in order to find the longest possible match, which
    /// corresponds to the leftmost starting position.
    ///
    /// Note that if you need the starting position then
    /// [`hybrid::regex::Regex`](crate::hybrid::regex::Regex) will handle this
    /// for you, so it's usually not necessary to do this yourself.
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::DFA,
    ///     nfa::thompson::NFA,
    ///     Anchored, HalfMatch, Input, MatchKind,
    /// };
    ///
    /// let input = Input::new("123foobar456");
    /// let pattern = r"[a-z]+r";
    ///
    /// let dfa_fwd = DFA::new(pattern)?;
    /// let dfa_rev = DFA::builder()
    ///     .thompson(NFA::config().reverse(true))
    ///     .configure(DFA::config().match_kind(MatchKind::All))
    ///     .build(pattern)?;
    /// let mut cache_fwd = dfa_fwd.create_cache();
    /// let mut cache_rev = dfa_rev.create_cache();
    ///
    /// let expected_fwd = HalfMatch::must(0, 9);
    /// let expected_rev = HalfMatch::must(0, 3);
    /// let got_fwd = dfa_fwd.try_search_fwd(&amp;mut cache_fwd, &amp;input)?.unwrap();
    /// // Here we don't specify the pattern to search for since there's only
    /// // one pattern and we're doing a leftmost search. But if this were an
    /// // overlapping search, you'd need to specify the pattern that matched
    /// // in the forward direction. (Otherwise, you might wind up finding the
    /// // starting position of a match of some other pattern.) That in turn
    /// // requires building the reverse automaton with starts_for_each_pattern
    /// // enabled.
    /// let input = input
    ///     .clone()
    ///     .range(..got_fwd.offset())
    ///     .anchored(Anchored::Yes);
    /// let got_rev = dfa_rev.try_search_rev(&amp;mut cache_rev, &amp;input)?.unwrap();
    /// assert_eq!(expected_fwd, got_fwd);
    /// assert_eq!(expected_rev, got_rev);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>match_kind(<span class="kw-2">mut </span><span class="self">self</span>, kind: MatchKind) -&gt; Config {
        <span class="self">self</span>.match_kind = <span class="prelude-val">Some</span>(kind);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Set a prefilter to be used whenever a start state is entered.
    ///
    /// A [`Prefilter`] in this context is meant to accelerate searches by
    /// looking for literal prefixes that every match for the corresponding
    /// pattern (or patterns) must start with. Once a prefilter produces a
    /// match, the underlying search routine continues on to try and confirm
    /// the match.
    ///
    /// Be warned that setting a prefilter does not guarantee that the search
    /// will be faster. While it's usually a good bet, if the prefilter
    /// produces a lot of false positive candidates (i.e., positions matched
    /// by the prefilter but not by the regex), then the overall result can
    /// be slower than if you had just executed the regex engine without any
    /// prefilters.
    ///
    /// Note that unless [`Config::specialize_start_states`] has been
    /// explicitly set, then setting this will also enable (when `pre` is
    /// `Some`) or disable (when `pre` is `None`) start state specialization.
    /// This occurs because without start state specialization, a prefilter
    /// is likely to be less effective. And without a prefilter, start state
    /// specialization is usually pointless.
    ///
    /// By default no prefilter is set.
    ///
    /// # Example
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::DFA,
    ///     util::prefilter::Prefilter,
    ///     Input, HalfMatch, MatchKind,
    /// };
    ///
    /// let pre = Prefilter::new(MatchKind::LeftmostFirst, &amp;["foo", "bar"]);
    /// let re = DFA::builder()
    ///     .configure(DFA::config().prefilter(pre))
    ///     .build(r"(foo|bar)[a-z]+")?;
    /// let mut cache = re.create_cache();
    /// let input = Input::new("foo1 barfox bar");
    /// assert_eq!(
    ///     Some(HalfMatch::must(0, 11)),
    ///     re.try_search_fwd(&amp;mut cache, &amp;input)?,
    /// );
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// Be warned though that an incorrect prefilter can lead to incorrect
    /// results!
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::DFA,
    ///     util::prefilter::Prefilter,
    ///     Input, HalfMatch, MatchKind,
    /// };
    ///
    /// let pre = Prefilter::new(MatchKind::LeftmostFirst, &amp;["foo", "car"]);
    /// let re = DFA::builder()
    ///     .configure(DFA::config().prefilter(pre))
    ///     .build(r"(foo|bar)[a-z]+")?;
    /// let mut cache = re.create_cache();
    /// let input = Input::new("foo1 barfox bar");
    /// assert_eq!(
    ///     // No match reported even though there clearly is one!
    ///     None,
    ///     re.try_search_fwd(&amp;mut cache, &amp;input)?,
    /// );
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>prefilter(<span class="kw-2">mut </span><span class="self">self</span>, pre: <span class="prelude-ty">Option</span>&lt;Prefilter&gt;) -&gt; Config {
        <span class="self">self</span>.pre = <span class="prelude-val">Some</span>(pre);
        <span class="kw">if </span><span class="self">self</span>.specialize_start_states.is_none() {
            <span class="self">self</span>.specialize_start_states =
                <span class="prelude-val">Some</span>(<span class="self">self</span>.get_prefilter().is_some());
        }
        <span class="self">self
    </span>}

    <span class="doccomment">/// Whether to compile a separate start state for each pattern in the
    /// lazy DFA.
    ///
    /// When enabled, a separate **anchored** start state is added for each
    /// pattern in the lazy DFA. When this start state is used, then the DFA
    /// will only search for matches for the pattern specified, even if there
    /// are other patterns in the DFA.
    ///
    /// The main downside of this option is that it can potentially increase
    /// the size of the DFA and/or increase the time it takes to build the
    /// DFA at search time. However, since this is configuration for a lazy
    /// DFA, these states aren't actually built unless they're used. Enabling
    /// this isn't necessarily free, however, as it may result in higher cache
    /// usage.
    ///
    /// There are a few reasons one might want to enable this (it's disabled
    /// by default):
    ///
    /// 1. When looking for the start of an overlapping match (using a reverse
    /// DFA), doing it correctly requires starting the reverse search using the
    /// starting state of the pattern that matched in the forward direction.
    /// Indeed, when building a [`Regex`](crate::hybrid::regex::Regex), it
    /// will automatically enable this option when building the reverse DFA
    /// internally.
    /// 2. When you want to use a DFA with multiple patterns to both search
    /// for matches of any pattern or to search for anchored matches of one
    /// particular pattern while using the same DFA. (Otherwise, you would need
    /// to compile a new DFA for each pattern.)
    ///
    /// By default this is disabled.
    ///
    /// # Example
    ///
    /// This example shows how to use this option to permit the same lazy DFA
    /// to run both general searches for any pattern and anchored searches for
    /// a specific pattern.
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::DFA,
    ///     Anchored, HalfMatch, Input, PatternID,
    /// };
    ///
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().starts_for_each_pattern(true))
    ///     .build_many(&amp;[r"[a-z0-9]{6}", r"[a-z][a-z0-9]{5}"])?;
    /// let mut cache = dfa.create_cache();
    /// let haystack = "bar foo123";
    ///
    /// // Here's a normal unanchored search that looks for any pattern.
    /// let expected = HalfMatch::must(0, 10);
    /// let input = Input::new(haystack);
    /// assert_eq!(Some(expected), dfa.try_search_fwd(&amp;mut cache, &amp;input)?);
    /// // We can also do a normal anchored search for any pattern. Since it's
    /// // an anchored search, we position the start of the search where we
    /// // know the match will begin.
    /// let expected = HalfMatch::must(0, 10);
    /// let input = Input::new(haystack).range(4..);
    /// assert_eq!(Some(expected), dfa.try_search_fwd(&amp;mut cache, &amp;input)?);
    /// // Since we compiled anchored start states for each pattern, we can
    /// // also look for matches of other patterns explicitly, even if a
    /// // different pattern would have normally matched.
    /// let expected = HalfMatch::must(1, 10);
    /// let input = Input::new(haystack)
    ///     .range(4..)
    ///     .anchored(Anchored::Pattern(PatternID::must(1)));
    /// assert_eq!(Some(expected), dfa.try_search_fwd(&amp;mut cache, &amp;input)?);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>starts_for_each_pattern(<span class="kw-2">mut </span><span class="self">self</span>, yes: bool) -&gt; Config {
        <span class="self">self</span>.starts_for_each_pattern = <span class="prelude-val">Some</span>(yes);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Whether to attempt to shrink the size of the lazy DFA's alphabet or
    /// not.
    ///
    /// This option is enabled by default and should never be disabled unless
    /// one is debugging the lazy DFA.
    ///
    /// When enabled, the lazy DFA will use a map from all possible bytes
    /// to their corresponding equivalence class. Each equivalence class
    /// represents a set of bytes that does not discriminate between a match
    /// and a non-match in the DFA. For example, the pattern `[ab]+` has at
    /// least two equivalence classes: a set containing `a` and `b` and a set
    /// containing every byte except for `a` and `b`. `a` and `b` are in the
    /// same equivalence classes because they never discriminate between a
    /// match and a non-match.
    ///
    /// The advantage of this map is that the size of the transition table
    /// can be reduced drastically from `#states * 256 * sizeof(LazyStateID)`
    /// to `#states * k * sizeof(LazyStateID)` where `k` is the number of
    /// equivalence classes (rounded up to the nearest power of 2). As a
    /// result, total space usage can decrease substantially. Moreover, since a
    /// smaller alphabet is used, DFA compilation during search becomes faster
    /// as well since it will potentially be able to reuse a single transition
    /// for multiple bytes.
    ///
    /// **WARNING:** This is only useful for debugging lazy DFAs. Disabling
    /// this does not yield any speed advantages. Namely, even when this is
    /// disabled, a byte class map is still used while searching. The only
    /// difference is that every byte will be forced into its own distinct
    /// equivalence class. This is useful for debugging the actual generated
    /// transitions because it lets one see the transitions defined on actual
    /// bytes instead of the equivalence classes.
    </span><span class="kw">pub fn </span>byte_classes(<span class="kw-2">mut </span><span class="self">self</span>, yes: bool) -&gt; Config {
        <span class="self">self</span>.byte_classes = <span class="prelude-val">Some</span>(yes);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Heuristically enable Unicode word boundaries.
    ///
    /// When set, this will attempt to implement Unicode word boundaries as if
    /// they were ASCII word boundaries. This only works when the search input
    /// is ASCII only. If a non-ASCII byte is observed while searching, then a
    /// [`MatchError::quit`] error is returned.
    ///
    /// A possible alternative to enabling this option is to simply use an
    /// ASCII word boundary, e.g., via `(?-u:\b)`. The main reason to use this
    /// option is if you absolutely need Unicode support. This option lets one
    /// use a fast search implementation (a DFA) for some potentially very
    /// common cases, while providing the option to fall back to some other
    /// regex engine to handle the general case when an error is returned.
    ///
    /// If the pattern provided has no Unicode word boundary in it, then this
    /// option has no effect. (That is, quitting on a non-ASCII byte only
    /// occurs when this option is enabled _and_ a Unicode word boundary is
    /// present in the pattern.)
    ///
    /// This is almost equivalent to setting all non-ASCII bytes to be quit
    /// bytes. The only difference is that this will cause non-ASCII bytes to
    /// be quit bytes _only_ when a Unicode word boundary is present in the
    /// pattern.
    ///
    /// When enabling this option, callers _must_ be prepared to
    /// handle a [`MatchError`] error during search. When using a
    /// [`Regex`](crate::hybrid::regex::Regex), this corresponds to using the
    /// `try_` suite of methods. Alternatively, if callers can guarantee that
    /// their input is ASCII only, then a [`MatchError::quit`] error will never
    /// be returned while searching.
    ///
    /// This is disabled by default.
    ///
    /// # Example
    ///
    /// This example shows how to heuristically enable Unicode word boundaries
    /// in a pattern. It also shows what happens when a search comes across a
    /// non-ASCII byte.
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::DFA,
    ///     HalfMatch, Input, MatchError,
    /// };
    ///
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().unicode_word_boundary(true))
    ///     .build(r"\b[0-9]+\b")?;
    /// let mut cache = dfa.create_cache();
    ///
    /// // The match occurs before the search ever observes the snowman
    /// // character, so no error occurs.
    /// let haystack = "foo 123  ☃";
    /// let expected = Some(HalfMatch::must(0, 7));
    /// let got = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(haystack))?;
    /// assert_eq!(expected, got);
    ///
    /// // Notice that this search fails, even though the snowman character
    /// // occurs after the ending match offset. This is because search
    /// // routines read one byte past the end of the search to account for
    /// // look-around, and indeed, this is required here to determine whether
    /// // the trailing \b matches.
    /// let haystack = "foo 123 ☃";
    /// let expected = MatchError::quit(0xE2, 8);
    /// let got = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(haystack));
    /// assert_eq!(Err(expected), got);
    ///
    /// // Another example is executing a search where the span of the haystack
    /// // we specify is all ASCII, but there is non-ASCII just before it. This
    /// // correctly also reports an error.
    /// let input = Input::new("β123").range(2..);
    /// let expected = MatchError::quit(0xB2, 1);
    /// let got = dfa.try_search_fwd(&amp;mut cache, &amp;input);
    /// assert_eq!(Err(expected), got);
    ///
    /// // And similarly for the trailing word boundary.
    /// let input = Input::new("123β").range(..3);
    /// let expected = MatchError::quit(0xCE, 3);
    /// let got = dfa.try_search_fwd(&amp;mut cache, &amp;input);
    /// assert_eq!(Err(expected), got);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>unicode_word_boundary(<span class="kw-2">mut </span><span class="self">self</span>, yes: bool) -&gt; Config {
        <span class="comment">// We have a separate option for this instead of just setting the
        // appropriate quit bytes here because we don't want to set quit bytes
        // for every regex. We only want to set them when the regex contains a
        // Unicode word boundary.
        </span><span class="self">self</span>.unicode_word_boundary = <span class="prelude-val">Some</span>(yes);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Add a "quit" byte to the lazy DFA.
    ///
    /// When a quit byte is seen during search time, then search will return a
    /// [`MatchError::quit`] error indicating the offset at which the search
    /// stopped.
    ///
    /// A quit byte will always overrule any other aspects of a regex. For
    /// example, if the `x` byte is added as a quit byte and the regex `\w` is
    /// used, then observing `x` will cause the search to quit immediately
    /// despite the fact that `x` is in the `\w` class.
    ///
    /// This mechanism is primarily useful for heuristically enabling certain
    /// features like Unicode word boundaries in a DFA. Namely, if the input
    /// to search is ASCII, then a Unicode word boundary can be implemented
    /// via an ASCII word boundary with no change in semantics. Thus, a DFA
    /// can attempt to match a Unicode word boundary but give up as soon as it
    /// observes a non-ASCII byte. Indeed, if callers set all non-ASCII bytes
    /// to be quit bytes, then Unicode word boundaries will be permitted when
    /// building lazy DFAs. Of course, callers should enable
    /// [`Config::unicode_word_boundary`] if they want this behavior instead.
    /// (The advantage being that non-ASCII quit bytes will only be added if a
    /// Unicode word boundary is in the pattern.)
    ///
    /// When enabling this option, callers _must_ be prepared to
    /// handle a [`MatchError`] error during search. When using a
    /// [`Regex`](crate::hybrid::regex::Regex), this corresponds to using the
    /// `try_` suite of methods.
    ///
    /// By default, there are no quit bytes set.
    ///
    /// # Panics
    ///
    /// This panics if heuristic Unicode word boundaries are enabled and any
    /// non-ASCII byte is removed from the set of quit bytes. Namely, enabling
    /// Unicode word boundaries requires setting every non-ASCII byte to a quit
    /// byte. So if the caller attempts to undo any of that, then this will
    /// panic.
    ///
    /// # Example
    ///
    /// This example shows how to cause a search to terminate if it sees a
    /// `\n` byte. This could be useful if, for example, you wanted to prevent
    /// a user supplied pattern from matching across a line boundary.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{hybrid::dfa::DFA, MatchError, Input};
    ///
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().quit(b'\n', true))
    ///     .build(r"foo\p{any}+bar")?;
    /// let mut cache = dfa.create_cache();
    ///
    /// let haystack = "foo\nbar";
    /// // Normally this would produce a match, since \p{any} contains '\n'.
    /// // But since we instructed the automaton to enter a quit state if a
    /// // '\n' is observed, this produces a match error instead.
    /// let expected = MatchError::quit(b'\n', 3);
    /// let got = dfa.try_search_fwd(
    ///     &amp;mut cache,
    ///     &amp;Input::new(haystack),
    /// ).unwrap_err();
    /// assert_eq!(expected, got);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>quit(<span class="kw-2">mut </span><span class="self">self</span>, byte: u8, yes: bool) -&gt; Config {
        <span class="kw">if </span><span class="self">self</span>.get_unicode_word_boundary() &amp;&amp; !byte.is_ascii() &amp;&amp; !yes {
            <span class="macro">panic!</span>(
                <span class="string">"cannot set non-ASCII byte to be non-quit when \
                 Unicode word boundaries are enabled"
            </span>);
        }
        <span class="kw">if </span><span class="self">self</span>.quitset.is_none() {
            <span class="self">self</span>.quitset = <span class="prelude-val">Some</span>(ByteSet::empty());
        }
        <span class="kw">if </span>yes {
            <span class="self">self</span>.quitset.as_mut().unwrap().add(byte);
        } <span class="kw">else </span>{
            <span class="self">self</span>.quitset.as_mut().unwrap().remove(byte);
        }
        <span class="self">self
    </span>}

    <span class="doccomment">/// Enable specializing start states in the lazy DFA.
    ///
    /// When start states are specialized, an implementor of a search routine
    /// using a lazy DFA can tell when the search has entered a starting state.
    /// When start states aren't specialized, then it is impossible to know
    /// whether the search has entered a start state.
    ///
    /// Ideally, this option wouldn't need to exist and we could always
    /// specialize start states. The problem is that start states can be quite
    /// active. This in turn means that an efficient search routine is likely
    /// to ping-pong between a heavily optimized hot loop that handles most
    /// states and to a less optimized specialized handling of start states.
    /// This causes branches to get heavily mispredicted and overall can
    /// materially decrease throughput. Therefore, specializing start states
    /// should only be enabled when it is needed.
    ///
    /// Knowing whether a search is in a start state is typically useful when a
    /// prefilter is active for the search. A prefilter is typically only run
    /// when in a start state and a prefilter can greatly accelerate a search.
    /// Therefore, the possible cost of specializing start states is worth it
    /// in this case. Otherwise, if you have no prefilter, there is likely no
    /// reason to specialize start states.
    ///
    /// This is disabled by default, but note that it is automatically
    /// enabled (or disabled) if [`Config::prefilter`] is set. Namely, unless
    /// `specialize_start_states` has already been set, [`Config::prefilter`]
    /// will automatically enable or disable it based on whether a prefilter
    /// is present or not, respectively. This is done because a prefilter's
    /// effectiveness is rooted in being executed whenever the DFA is in a
    /// start state, and that's only possible to do when they are specialized.
    ///
    /// Note that it is plausibly reasonable to _disable_ this option
    /// explicitly while _enabling_ a prefilter. In that case, a prefilter
    /// will still be run at the beginning of a search, but never again. This
    /// in theory could strike a good balance if you're in a situation where a
    /// prefilter is likely to produce many false positive candidates.
    ///
    /// # Example
    ///
    /// This example shows how to enable start state specialization and then
    /// shows how to check whether a state is a start state or not.
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, MatchError, Input};
    ///
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().specialize_start_states(true))
    ///     .build(r"[a-z]+")?;
    /// let mut cache = dfa.create_cache();
    ///
    /// let haystack = "123 foobar 4567".as_bytes();
    /// let sid = dfa.start_state_forward(&amp;mut cache, &amp;Input::new(haystack))?;
    /// // The ID returned by 'start_state_forward' will always be tagged as
    /// // a start state when start state specialization is enabled.
    /// assert!(sid.is_tagged());
    /// assert!(sid.is_start());
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// Compare the above with the default lazy DFA configuration where
    /// start states are _not_ specialized. In this case, the start state
    /// is not tagged and `sid.is_start()` returns false.
    ///
    /// ```
    /// use regex_automata::{hybrid::dfa::DFA, MatchError, Input};
    ///
    /// let dfa = DFA::new(r"[a-z]+")?;
    /// let mut cache = dfa.create_cache();
    ///
    /// let haystack = "123 foobar 4567".as_bytes();
    /// let sid = dfa.start_state_forward(&amp;mut cache, &amp;Input::new(haystack))?;
    /// // Start states are not tagged in the default configuration!
    /// assert!(!sid.is_tagged());
    /// assert!(!sid.is_start());
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>specialize_start_states(<span class="kw-2">mut </span><span class="self">self</span>, yes: bool) -&gt; Config {
        <span class="self">self</span>.specialize_start_states = <span class="prelude-val">Some</span>(yes);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Sets the maximum amount of heap memory, in bytes, to allocate to the
    /// cache for use during a lazy DFA search. If the lazy DFA would otherwise
    /// use more heap memory, then, depending on other configuration knobs,
    /// either stop the search and return an error or clear the cache and
    /// continue the search.
    ///
    /// The default cache capacity is some "reasonable" number that will
    /// accommodate most regular expressions. You may find that if you need
    /// to build a large DFA then it may be necessary to increase the cache
    /// capacity.
    ///
    /// Note that while building a lazy DFA will do a "minimum" check to ensure
    /// the capacity is big enough, this is more or less about correctness.
    /// If the cache is bigger than the minimum but still "too small," then the
    /// lazy DFA could wind up spending a lot of time clearing the cache and
    /// recomputing transitions, thus negating the performance benefits of a
    /// lazy DFA. Thus, setting the cache capacity is mostly an experimental
    /// endeavor. For most common patterns, however, the default should be
    /// sufficient.
    ///
    /// For more details on how the lazy DFA's cache is used, see the
    /// documentation for [`Cache`].
    ///
    /// # Example
    ///
    /// This example shows what happens if the configured cache capacity is
    /// too small. In such cases, one can override the cache capacity to make
    /// it bigger. Alternatively, one might want to use less memory by setting
    /// a smaller cache capacity.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};
    ///
    /// let pattern = r"\p{L}{1000}";
    ///
    /// // The default cache capacity is likely too small to deal with regexes
    /// // that are very large. Large repetitions of large Unicode character
    /// // classes are a common way to make very large regexes.
    /// let _ = DFA::new(pattern).unwrap_err();
    /// // Bump up the capacity to something bigger.
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().cache_capacity(100 * (1&lt;&lt;20))) // 100 MB
    ///     .build(pattern)?;
    /// let mut cache = dfa.create_cache();
    ///
    /// let haystack = "ͰͲͶͿΆΈΉΊΌΎΏΑΒΓΔΕΖΗΘΙ".repeat(50);
    /// let expected = Some(HalfMatch::must(0, 2000));
    /// let got = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(&amp;haystack))?;
    /// assert_eq!(expected, got);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>cache_capacity(<span class="kw-2">mut </span><span class="self">self</span>, bytes: usize) -&gt; Config {
        <span class="self">self</span>.cache_capacity = <span class="prelude-val">Some</span>(bytes);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Configures construction of a lazy DFA to use the minimum cache capacity
    /// if the configured capacity is otherwise too small for the provided NFA.
    ///
    /// This is useful if you never want lazy DFA construction to fail because
    /// of a capacity that is too small.
    ///
    /// In general, this option is typically not a good idea. In particular,
    /// while a minimum cache capacity does permit the lazy DFA to function
    /// where it otherwise couldn't, it's plausible that it may not function
    /// well if it's constantly running out of room. In that case, the speed
    /// advantages of the lazy DFA may be negated. On the other hand, the
    /// "minimum" cache capacity computed may not be completely accurate and
    /// could actually be bigger than what is really necessary. Therefore, it
    /// is plausible that using the minimum cache capacity could still result
    /// in very good performance.
    ///
    /// This is disabled by default.
    ///
    /// # Example
    ///
    /// This example shows what happens if the configured cache capacity is
    /// too small. In such cases, one could override the capacity explicitly.
    /// An alternative, demonstrated here, let's us force construction to use
    /// the minimum cache capacity if the configured capacity is otherwise
    /// too small.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};
    ///
    /// let pattern = r"\p{L}{1000}";
    ///
    /// // The default cache capacity is likely too small to deal with regexes
    /// // that are very large. Large repetitions of large Unicode character
    /// // classes are a common way to make very large regexes.
    /// let _ = DFA::new(pattern).unwrap_err();
    /// // Configure construction such it automatically selects the minimum
    /// // cache capacity if it would otherwise be too small.
    /// let dfa = DFA::builder()
    ///     .configure(DFA::config().skip_cache_capacity_check(true))
    ///     .build(pattern)?;
    /// let mut cache = dfa.create_cache();
    ///
    /// let haystack = "ͰͲͶͿΆΈΉΊΌΎΏΑΒΓΔΕΖΗΘΙ".repeat(50);
    /// let expected = Some(HalfMatch::must(0, 2000));
    /// let got = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(&amp;haystack))?;
    /// assert_eq!(expected, got);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>skip_cache_capacity_check(<span class="kw-2">mut </span><span class="self">self</span>, yes: bool) -&gt; Config {
        <span class="self">self</span>.skip_cache_capacity_check = <span class="prelude-val">Some</span>(yes);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Configure a lazy DFA search to quit after a certain number of cache
    /// clearings.
    ///
    /// When a minimum is set, then a lazy DFA search will *possibly* "give
    /// up" after the minimum number of cache clearings has occurred. This is
    /// typically useful in scenarios where callers want to detect whether the
    /// lazy DFA search is "efficient" or not. If the cache is cleared too many
    /// times, this is a good indicator that it is not efficient, and thus, the
    /// caller may wish to use some other regex engine.
    ///
    /// Note that the number of times a cache is cleared is a property of
    /// the cache itself. Thus, if a cache is used in a subsequent search
    /// with a similarly configured lazy DFA, then it could cause the
    /// search to "give up" if the cache needed to be cleared, depending
    /// on its internal count and configured minimum. The cache clear
    /// count can only be reset to `0` via [`DFA::reset_cache`] (or
    /// [`Regex::reset_cache`](crate::hybrid::regex::Regex::reset_cache) if
    /// you're using the `Regex` API).
    ///
    /// By default, no minimum is configured. Thus, a lazy DFA search will
    /// never give up due to cache clearings. If you do set this option, you
    /// might consider also setting [`Config::minimum_bytes_per_state`] in
    /// order for the lazy DFA to take efficiency into account before giving
    /// up.
    ///
    /// # Example
    ///
    /// This example uses a somewhat pathological configuration to demonstrate
    /// the _possible_ behavior of cache clearing and how it might result
    /// in a search that returns an error.
    ///
    /// It is important to note that the precise mechanics of how and when
    /// a cache gets cleared is an implementation detail.
    ///
    /// ```
    /// # if cfg!(miri) { return Ok(()); } // miri takes too long
    /// use regex_automata::{hybrid::dfa::DFA, Input, MatchError, MatchErrorKind};
    ///
    /// // This is a carefully chosen regex. The idea is to pick one
    /// // that requires some decent number of states (hence the bounded
    /// // repetition). But we specifically choose to create a class with an
    /// // ASCII letter and a non-ASCII letter so that we can check that no new
    /// // states are created once the cache is full. Namely, if we fill up the
    /// // cache on a haystack of 'a's, then in order to match one 'β', a new
    /// // state will need to be created since a 'β' is encoded with multiple
    /// // bytes. Since there's no room for this state, the search should quit
    /// // at the very first position.
    /// let pattern = r"[aβ]{100}";
    /// let dfa = DFA::builder()
    ///     .configure(
    ///         // Configure it so that we have the minimum cache capacity
    ///         // possible. And that if any clearings occur, the search quits.
    ///         DFA::config()
    ///             .skip_cache_capacity_check(true)
    ///             .cache_capacity(0)
    ///             .minimum_cache_clear_count(Some(0)),
    ///     )
    ///     .build(pattern)?;
    /// let mut cache = dfa.create_cache();
    ///
    /// // Our search will give up before reaching the end!
    /// let haystack = "a".repeat(101).into_bytes();
    /// let result = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(&amp;haystack));
    /// assert!(matches!(
    ///     *result.unwrap_err().kind(),
    ///     MatchErrorKind::GaveUp { .. },
    /// ));
    ///
    /// // Now that we know the cache is full, if we search a haystack that we
    /// // know will require creating at least one new state, it should not
    /// // be able to make much progress.
    /// let haystack = "β".repeat(101).into_bytes();
    /// let result = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(&amp;haystack));
    /// assert!(matches!(
    ///     *result.unwrap_err().kind(),
    ///     MatchErrorKind::GaveUp { .. },
    /// ));
    ///
    /// // If we reset the cache, then we should be able to create more states
    /// // and make more progress with searching for betas.
    /// cache.reset(&amp;dfa);
    /// let haystack = "β".repeat(101).into_bytes();
    /// let result = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(&amp;haystack));
    /// assert!(matches!(
    ///     *result.unwrap_err().kind(),
    ///     MatchErrorKind::GaveUp { .. },
    /// ));
    ///
    /// // ... switching back to ASCII still makes progress since it just needs
    /// // to set transitions on existing states!
    /// let haystack = "a".repeat(101).into_bytes();
    /// let result = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(&amp;haystack));
    /// assert!(matches!(
    ///     *result.unwrap_err().kind(),
    ///     MatchErrorKind::GaveUp { .. },
    /// ));
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>minimum_cache_clear_count(<span class="kw-2">mut </span><span class="self">self</span>, min: <span class="prelude-ty">Option</span>&lt;usize&gt;) -&gt; Config {
        <span class="self">self</span>.minimum_cache_clear_count = <span class="prelude-val">Some</span>(min);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Configure a lazy DFA search to quit only when its efficiency drops
    /// below the given minimum.
    ///
    /// The efficiency of the cache is determined by the number of DFA states
    /// compiled per byte of haystack searched. For example, if the efficiency
    /// is 2, then it means the lazy DFA is creating a new DFA state after
    /// searching approximately 2 bytes in a haystack. Generally speaking, 2
    /// is quite bad and it's likely that even a slower regex engine like the
    /// [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) would be faster.
    ///
    /// This has no effect if [`Config::minimum_cache_clear_count`] is not set.
    /// Namely, this option only kicks in when the cache has been cleared more
    /// than the minimum number. If no minimum is set, then the cache is simply
    /// cleared whenever it fills up and it is impossible for the lazy DFA to
    /// quit due to ineffective use of the cache.
    ///
    /// In general, if one is setting [`Config::minimum_cache_clear_count`],
    /// then one should probably also set this knob as well. The reason is
    /// that the absolute number of times the cache is cleared is generally
    /// not a great predictor of efficiency. For example, if a new DFA state
    /// is created for every 1,000 bytes searched, then it wouldn't be hard
    /// for the cache to get cleared more than `N` times and then cause the
    /// lazy DFA to quit. But a new DFA state every 1,000 bytes is likely quite
    /// good from a performance perspective, and it's likely that the lazy
    /// DFA should continue searching, even if it requires clearing the cache
    /// occasionally.
    ///
    /// Finally, note that if you're implementing your own lazy DFA search
    /// routine and also want this efficiency check to work correctly, then
    /// you'll need to use the following routines to record search progress:
    ///
    /// * Call [`Cache::search_start`] at the beginning of every search.
    /// * Call [`Cache::search_update`] whenever [`DFA::next_state`] is
    /// called.
    /// * Call [`Cache::search_finish`] before completing a search. (It is
    /// not strictly necessary to call this when an error is returned, as
    /// `Cache::search_start` will automatically finish the previous search
    /// for you. But calling it where possible before returning helps improve
    /// the accuracy of how many bytes have actually been searched.)
    </span><span class="kw">pub fn </span>minimum_bytes_per_state(<span class="kw-2">mut </span><span class="self">self</span>, min: <span class="prelude-ty">Option</span>&lt;usize&gt;) -&gt; Config {
        <span class="self">self</span>.minimum_bytes_per_state = <span class="prelude-val">Some</span>(min);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Returns the match semantics set in this configuration.
    </span><span class="kw">pub fn </span>get_match_kind(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; MatchKind {
        <span class="self">self</span>.match_kind.unwrap_or(MatchKind::LeftmostFirst)
    }

    <span class="doccomment">/// Returns the prefilter set in this configuration, if one at all.
    </span><span class="kw">pub fn </span>get_prefilter(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>Prefilter&gt; {
        <span class="self">self</span>.pre.as_ref().unwrap_or(<span class="kw-2">&amp;</span><span class="prelude-val">None</span>).as_ref()
    }

    <span class="doccomment">/// Returns whether this configuration has enabled anchored starting states
    /// for every pattern in the DFA.
    </span><span class="kw">pub fn </span>get_starts_for_each_pattern(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.starts_for_each_pattern.unwrap_or(<span class="bool-val">false</span>)
    }

    <span class="doccomment">/// Returns whether this configuration has enabled byte classes or not.
    /// This is typically a debugging oriented option, as disabling it confers
    /// no speed benefit.
    </span><span class="kw">pub fn </span>get_byte_classes(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.byte_classes.unwrap_or(<span class="bool-val">true</span>)
    }

    <span class="doccomment">/// Returns whether this configuration has enabled heuristic Unicode word
    /// boundary support. When enabled, it is possible for a search to return
    /// an error.
    </span><span class="kw">pub fn </span>get_unicode_word_boundary(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.unicode_word_boundary.unwrap_or(<span class="bool-val">false</span>)
    }

    <span class="doccomment">/// Returns whether this configuration will instruct the lazy DFA to enter
    /// a quit state whenever the given byte is seen during a search. When at
    /// least one byte has this enabled, it is possible for a search to return
    /// an error.
    </span><span class="kw">pub fn </span>get_quit(<span class="kw-2">&amp;</span><span class="self">self</span>, byte: u8) -&gt; bool {
        <span class="self">self</span>.quitset.map_or(<span class="bool-val">false</span>, |q| q.contains(byte))
    }

    <span class="doccomment">/// Returns whether this configuration will instruct the lazy DFA to
    /// "specialize" start states. When enabled, the lazy DFA will tag start
    /// states so that search routines using the lazy DFA can detect when
    /// it's in a start state and do some kind of optimization (like run a
    /// prefilter).
    </span><span class="kw">pub fn </span>get_specialize_start_states(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.specialize_start_states.unwrap_or(<span class="bool-val">false</span>)
    }

    <span class="doccomment">/// Returns the cache capacity set on this configuration.
    </span><span class="kw">pub fn </span>get_cache_capacity(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.cache_capacity.unwrap_or(<span class="number">2 </span>* (<span class="number">1 </span>&lt;&lt; <span class="number">20</span>))
    }

    <span class="doccomment">/// Returns whether the cache capacity check should be skipped.
    </span><span class="kw">pub fn </span>get_skip_cache_capacity_check(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.skip_cache_capacity_check.unwrap_or(<span class="bool-val">false</span>)
    }

    <span class="doccomment">/// Returns, if set, the minimum number of times the cache must be cleared
    /// before a lazy DFA search can give up. When no minimum is set, then a
    /// search will never quit and will always clear the cache whenever it
    /// fills up.
    </span><span class="kw">pub fn </span>get_minimum_cache_clear_count(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="self">self</span>.minimum_cache_clear_count.unwrap_or(<span class="prelude-val">None</span>)
    }

    <span class="doccomment">/// Returns, if set, the minimum number of bytes per state that need to be
    /// processed in order for the lazy DFA to keep going. If the minimum falls
    /// below this number (and the cache has been cleared a minimum number of
    /// times), then the lazy DFA will return a "gave up" error.
    </span><span class="kw">pub fn </span>get_minimum_bytes_per_state(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="self">self</span>.minimum_bytes_per_state.unwrap_or(<span class="prelude-val">None</span>)
    }

    <span class="doccomment">/// Returns the minimum lazy DFA cache capacity required for the given NFA.
    ///
    /// The cache capacity required for a particular NFA may change without
    /// notice. Callers should not rely on it being stable.
    ///
    /// This is useful for informational purposes, but can also be useful for
    /// other reasons. For example, if one wants to check the minimum cache
    /// capacity themselves or if one wants to set the capacity based on the
    /// minimum.
    ///
    /// This may return an error if this configuration does not support all of
    /// the instructions used in the given NFA. For example, if the NFA has a
    /// Unicode word boundary but this configuration does not enable heuristic
    /// support for Unicode word boundaries.
    </span><span class="kw">pub fn </span>get_minimum_cache_capacity(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        nfa: <span class="kw-2">&amp;</span>thompson::NFA,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;usize, BuildError&gt; {
        <span class="kw">let </span>quitset = <span class="self">self</span>.quit_set_from_nfa(nfa)<span class="question-mark">?</span>;
        <span class="kw">let </span>classes = <span class="self">self</span>.byte_classes_from_nfa(nfa, <span class="kw-2">&amp;</span>quitset);
        <span class="kw">let </span>starts = <span class="self">self</span>.get_starts_for_each_pattern();
        <span class="prelude-val">Ok</span>(minimum_cache_capacity(nfa, <span class="kw-2">&amp;</span>classes, starts))
    }

    <span class="doccomment">/// Returns the byte class map used during search from the given NFA.
    ///
    /// If byte classes are disabled on this configuration, then a map is
    /// returned that puts each byte in its own equivalent class.
    </span><span class="kw">fn </span>byte_classes_from_nfa(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        nfa: <span class="kw-2">&amp;</span>thompson::NFA,
        quit: <span class="kw-2">&amp;</span>ByteSet,
    ) -&gt; ByteClasses {
        <span class="kw">if </span>!<span class="self">self</span>.get_byte_classes() {
            <span class="comment">// The lazy DFA will always use the equivalence class map, but
            // enabling this option is useful for debugging. Namely, this will
            // cause all transitions to be defined over their actual bytes
            // instead of an opaque equivalence class identifier. The former is
            // much easier to grok as a human.
            </span>ByteClasses::singletons()
        } <span class="kw">else </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>set = nfa.byte_class_set().clone();
            <span class="comment">// It is important to distinguish any "quit" bytes from all other
            // bytes. Otherwise, a non-quit byte may end up in the same class
            // as a quit byte, and thus cause the DFA stop when it shouldn't.
            //
            // Test case:
            //
            //   regex-cli find match hybrid --unicode-word-boundary \
            //     -p '^#' -p '\b10\.55\.182\.100\b' -y @conn.json.1000x.log
            </span><span class="kw">if </span>!quit.is_empty() {
                set.add_set(<span class="kw-2">&amp;</span>quit);
            }
            set.byte_classes()
        }
    }

    <span class="doccomment">/// Return the quit set for this configuration and the given NFA.
    ///
    /// This may return an error if the NFA is incompatible with this
    /// configuration's quit set. For example, if the NFA has a Unicode word
    /// boundary and the quit set doesn't include non-ASCII bytes.
    </span><span class="kw">fn </span>quit_set_from_nfa(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        nfa: <span class="kw-2">&amp;</span>thompson::NFA,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;ByteSet, BuildError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>quit = <span class="self">self</span>.quitset.unwrap_or(ByteSet::empty());
        <span class="kw">if </span>nfa.look_set_any().contains_word_unicode() {
            <span class="kw">if </span><span class="self">self</span>.get_unicode_word_boundary() {
                <span class="kw">for </span>b <span class="kw">in </span><span class="number">0x80</span>..=<span class="number">0xFF </span>{
                    quit.add(b);
                }
            } <span class="kw">else </span>{
                <span class="comment">// If heuristic support for Unicode word boundaries wasn't
                // enabled, then we can still check if our quit set is correct.
                // If the caller set their quit bytes in a way that causes the
                // DFA to quit on at least all non-ASCII bytes, then that's all
                // we need for heuristic support to work.
                </span><span class="kw">if </span>!quit.contains_range(<span class="number">0x80</span>, <span class="number">0xFF</span>) {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(
                        BuildError::unsupported_dfa_word_boundary_unicode(),
                    );
                }
            }
        }
        <span class="prelude-val">Ok</span>(quit)
    }

    <span class="doccomment">/// Overwrite the default configuration such that the options in `o` are
    /// always used. If an option in `o` is not set, then the corresponding
    /// option in `self` is used. If it's not set in `self` either, then it
    /// remains not set.
    </span><span class="kw">fn </span>overwrite(<span class="kw-2">&amp;</span><span class="self">self</span>, o: Config) -&gt; Config {
        Config {
            match_kind: o.match_kind.or(<span class="self">self</span>.match_kind),
            pre: o.pre.or_else(|| <span class="self">self</span>.pre.clone()),
            starts_for_each_pattern: o
                .starts_for_each_pattern
                .or(<span class="self">self</span>.starts_for_each_pattern),
            byte_classes: o.byte_classes.or(<span class="self">self</span>.byte_classes),
            unicode_word_boundary: o
                .unicode_word_boundary
                .or(<span class="self">self</span>.unicode_word_boundary),
            quitset: o.quitset.or(<span class="self">self</span>.quitset),
            specialize_start_states: o
                .specialize_start_states
                .or(<span class="self">self</span>.specialize_start_states),
            cache_capacity: o.cache_capacity.or(<span class="self">self</span>.cache_capacity),
            skip_cache_capacity_check: o
                .skip_cache_capacity_check
                .or(<span class="self">self</span>.skip_cache_capacity_check),
            minimum_cache_clear_count: o
                .minimum_cache_clear_count
                .or(<span class="self">self</span>.minimum_cache_clear_count),
            minimum_bytes_per_state: o
                .minimum_bytes_per_state
                .or(<span class="self">self</span>.minimum_bytes_per_state),
        }
    }
}

<span class="doccomment">/// A builder for constructing a lazy deterministic finite automaton from
/// regular expressions.
///
/// As a convenience, [`DFA::builder`] is an alias for [`Builder::new`]. The
/// advantage of the former is that it often lets you avoid importing the
/// `Builder` type directly.
///
/// This builder provides two main things:
///
/// 1. It provides a few different `build` routines for actually constructing
/// a DFA from different kinds of inputs. The most convenient is
/// [`Builder::build`], which builds a DFA directly from a pattern string. The
/// most flexible is [`Builder::build_from_nfa`], which builds a DFA straight
/// from an NFA.
/// 2. The builder permits configuring a number of things.
/// [`Builder::configure`] is used with [`Config`] to configure aspects of
/// the DFA and the construction process itself. [`Builder::syntax`] and
/// [`Builder::thompson`] permit configuring the regex parser and Thompson NFA
/// construction, respectively. The syntax and thompson configurations only
/// apply when building from a pattern string.
///
/// This builder always constructs a *single* lazy DFA. As such, this builder
/// can only be used to construct regexes that either detect the presence
/// of a match or find the end location of a match. A single DFA cannot
/// produce both the start and end of a match. For that information, use a
/// [`Regex`](crate::hybrid::regex::Regex), which can be similarly configured
/// using [`regex::Builder`](crate::hybrid::regex::Builder). The main reason
/// to use a DFA directly is if the end location of a match is enough for your
/// use case. Namely, a `Regex` will construct two lazy DFAs instead of one,
/// since a second reverse DFA is needed to find the start of a match.
///
/// # Example
///
/// This example shows how to build a lazy DFA that uses a tiny cache capacity
/// and completely disables Unicode. That is:
///
/// * Things such as `\w`, `.` and `\b` are no longer Unicode-aware. `\w`
///   and `\b` are ASCII-only while `.` matches any byte except for `\n`
///   (instead of any UTF-8 encoding of a Unicode scalar value except for
///   `\n`). Things that are Unicode only, such as `\pL`, are not allowed.
/// * The pattern itself is permitted to match invalid UTF-8. For example,
///   things like `[^a]` that match any byte except for `a` are permitted.
///
/// ```
/// use regex_automata::{
///     hybrid::dfa::DFA,
///     nfa::thompson,
///     util::syntax,
///     HalfMatch, Input,
/// };
///
/// let dfa = DFA::builder()
///     .configure(DFA::config().cache_capacity(5_000))
///     .thompson(thompson::Config::new().utf8(false))
///     .syntax(syntax::Config::new().unicode(false).utf8(false))
///     .build(r"foo[^b]ar.*")?;
/// let mut cache = dfa.create_cache();
///
/// let haystack = b"\xFEfoo\xFFar\xE2\x98\xFF\n";
/// let expected = Some(HalfMatch::must(0, 10));
/// let got = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(haystack))?;
/// assert_eq!(expected, got);
///
/// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
/// ```
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>Builder {
    config: Config,
    <span class="attr">#[cfg(feature = <span class="string">"syntax"</span>)]
    </span>thompson: thompson::Compiler,
}

<span class="kw">impl </span>Builder {
    <span class="doccomment">/// Create a new lazy DFA builder with the default configuration.
    </span><span class="kw">pub fn </span>new() -&gt; Builder {
        Builder {
            config: Config::default(),
            <span class="attr">#[cfg(feature = <span class="string">"syntax"</span>)]
            </span>thompson: thompson::Compiler::new(),
        }
    }

    <span class="doccomment">/// Build a lazy DFA from the given pattern.
    ///
    /// If there was a problem parsing or compiling the pattern, then an error
    /// is returned.
    </span><span class="attr">#[cfg(feature = <span class="string">"syntax"</span>)]
    </span><span class="kw">pub fn </span>build(<span class="kw-2">&amp;</span><span class="self">self</span>, pattern: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;DFA, BuildError&gt; {
        <span class="self">self</span>.build_many(<span class="kw-2">&amp;</span>[pattern])
    }

    <span class="doccomment">/// Build a lazy DFA from the given patterns.
    ///
    /// When matches are returned, the pattern ID corresponds to the index of
    /// the pattern in the slice given.
    </span><span class="attr">#[cfg(feature = <span class="string">"syntax"</span>)]
    </span><span class="kw">pub fn </span>build_many&lt;P: AsRef&lt;str&gt;&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        patterns: <span class="kw-2">&amp;</span>[P],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;DFA, BuildError&gt; {
        <span class="kw">let </span>nfa = <span class="self">self
            </span>.thompson
            .clone()
            <span class="comment">// We can always forcefully disable captures because DFAs do not
            // support them.
            </span>.configure(
                thompson::Config::new()
                    .which_captures(thompson::WhichCaptures::None),
            )
            .build_many(patterns)
            .map_err(BuildError::nfa)<span class="question-mark">?</span>;
        <span class="self">self</span>.build_from_nfa(nfa)
    }

    <span class="doccomment">/// Build a DFA from the given NFA.
    ///
    /// Note that this requires owning a `thompson::NFA`. While this may force
    /// you to clone the NFA, such a clone is not a deep clone. Namely, NFAs
    /// are defined internally to support shared ownership such that cloning is
    /// very cheap.
    ///
    /// # Example
    ///
    /// This example shows how to build a lazy DFA if you already have an NFA
    /// in hand.
    ///
    /// ```
    /// use regex_automata::{
    ///     hybrid::dfa::DFA,
    ///     nfa::thompson,
    ///     HalfMatch, Input,
    /// };
    ///
    /// let haystack = "foo123bar";
    ///
    /// // This shows how to set non-default options for building an NFA.
    /// let nfa = thompson::Compiler::new()
    ///     .configure(thompson::Config::new().shrink(true))
    ///     .build(r"[0-9]+")?;
    /// let dfa = DFA::builder().build_from_nfa(nfa)?;
    /// let mut cache = dfa.create_cache();
    /// let expected = Some(HalfMatch::must(0, 6));
    /// let got = dfa.try_search_fwd(&amp;mut cache, &amp;Input::new(haystack))?;
    /// assert_eq!(expected, got);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>build_from_nfa(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        nfa: thompson::NFA,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;DFA, BuildError&gt; {
        <span class="kw">let </span>quitset = <span class="self">self</span>.config.quit_set_from_nfa(<span class="kw-2">&amp;</span>nfa)<span class="question-mark">?</span>;
        <span class="kw">let </span>classes = <span class="self">self</span>.config.byte_classes_from_nfa(<span class="kw-2">&amp;</span>nfa, <span class="kw-2">&amp;</span>quitset);
        <span class="comment">// Check that we can fit at least a few states into our cache,
        // otherwise it's pretty senseless to use the lazy DFA. This does have
        // a possible failure mode though. This assumes the maximum size of a
        // state in powerset space (so, the total number of NFA states), which
        // may never actually materialize, and could be quite a bit larger
        // than the actual biggest state. If this turns out to be a problem,
        // we could expose a knob that disables this check. But if so, we have
        // to be careful not to panic in other areas of the code (the cache
        // clearing and init code) that tend to assume some minimum useful
        // cache capacity.
        </span><span class="kw">let </span>min_cache = minimum_cache_capacity(
            <span class="kw-2">&amp;</span>nfa,
            <span class="kw-2">&amp;</span>classes,
            <span class="self">self</span>.config.get_starts_for_each_pattern(),
        );
        <span class="kw">let </span><span class="kw-2">mut </span>cache_capacity = <span class="self">self</span>.config.get_cache_capacity();
        <span class="kw">if </span>cache_capacity &lt; min_cache {
            <span class="comment">// When the caller has asked us to skip the cache capacity check,
            // then we simply force the cache capacity to its minimum amount
            // and mush on.
            </span><span class="kw">if </span><span class="self">self</span>.config.get_skip_cache_capacity_check() {
                <span class="macro">debug!</span>(
                    <span class="string">"given capacity ({}) is too small, \
                     since skip_cache_capacity_check is enabled, \
                     setting cache capacity to minimum ({})"</span>,
                    cache_capacity, min_cache,
                );
                cache_capacity = min_cache;
            } <span class="kw">else </span>{
                <span class="kw">return </span><span class="prelude-val">Err</span>(BuildError::insufficient_cache_capacity(
                    min_cache,
                    cache_capacity,
                ));
            }
        }
        <span class="comment">// We also need to check that we can fit at least some small number
        // of states in our state ID space. This is unlikely to trigger in
        // &gt;=32-bit systems, but 16-bit systems have a pretty small state ID
        // space since a number of bits are used up as sentinels.
        </span><span class="kw">if let </span><span class="prelude-val">Err</span>(err) = minimum_lazy_state_id(<span class="kw-2">&amp;</span>classes) {
            <span class="kw">return </span><span class="prelude-val">Err</span>(BuildError::insufficient_state_id_capacity(err));
        }
        <span class="kw">let </span>stride2 = classes.stride2();
        <span class="kw">let </span>start_map = StartByteMap::new(nfa.look_matcher());
        <span class="prelude-val">Ok</span>(DFA {
            config: <span class="self">self</span>.config.clone(),
            nfa,
            stride2,
            start_map,
            classes,
            quitset,
            cache_capacity,
        })
    }

    <span class="doccomment">/// Apply the given lazy DFA configuration options to this builder.
    </span><span class="kw">pub fn </span>configure(<span class="kw-2">&amp;mut </span><span class="self">self</span>, config: Config) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.config = <span class="self">self</span>.config.overwrite(config);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Set the syntax configuration for this builder using
    /// [`syntax::Config`](crate::util::syntax::Config).
    ///
    /// This permits setting things like case insensitivity, Unicode and multi
    /// line mode.
    ///
    /// These settings only apply when constructing a lazy DFA directly from a
    /// pattern.
    </span><span class="attr">#[cfg(feature = <span class="string">"syntax"</span>)]
    </span><span class="kw">pub fn </span>syntax(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        config: <span class="kw">crate</span>::util::syntax::Config,
    ) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.thompson.syntax(config);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Set the Thompson NFA configuration for this builder using
    /// [`nfa::thompson::Config`](crate::nfa::thompson::Config).
    ///
    /// This permits setting things like whether the DFA should match the regex
    /// in reverse or if additional time should be spent shrinking the size of
    /// the NFA.
    ///
    /// These settings only apply when constructing a DFA directly from a
    /// pattern.
    </span><span class="attr">#[cfg(feature = <span class="string">"syntax"</span>)]
    </span><span class="kw">pub fn </span>thompson(<span class="kw-2">&amp;mut </span><span class="self">self</span>, config: thompson::Config) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.thompson.configure(config);
        <span class="self">self
    </span>}
}

<span class="doccomment">/// Represents the current state of an overlapping search.
///
/// This is used for overlapping searches since they need to know something
/// about the previous search. For example, when multiple patterns match at the
/// same position, this state tracks the last reported pattern so that the next
/// search knows whether to report another matching pattern or continue with
/// the search at the next position. Additionally, it also tracks which state
/// the last search call terminated in.
///
/// This type provides little introspection capabilities. The only thing a
/// caller can do is construct it and pass it around to permit search routines
/// to use it to track state, and also ask whether a match has been found.
///
/// Callers should always provide a fresh state constructed via
/// [`OverlappingState::start`] when starting a new search. Reusing state from
/// a previous search may result in incorrect results.
</span><span class="attr">#[derive(Clone, Debug, Eq, PartialEq)]
</span><span class="kw">pub struct </span>OverlappingState {
    <span class="doccomment">/// The match reported by the most recent overlapping search to use this
    /// state.
    ///
    /// If a search does not find any matches, then it is expected to clear
    /// this value.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) mat: <span class="prelude-ty">Option</span>&lt;HalfMatch&gt;,
    <span class="doccomment">/// The state ID of the state at which the search was in when the call
    /// terminated. When this is a match state, `last_match` must be set to a
    /// non-None value.
    ///
    /// A `None` value indicates the start state of the corresponding
    /// automaton. We cannot use the actual ID, since any one automaton may
    /// have many start states, and which one is in use depends on several
    /// search-time factors.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) id: <span class="prelude-ty">Option</span>&lt;LazyStateID&gt;,
    <span class="doccomment">/// The position of the search.
    ///
    /// When `id` is None (i.e., we are starting a search), this is set to
    /// the beginning of the search as given by the caller regardless of its
    /// current value. Subsequent calls to an overlapping search pick up at
    /// this offset.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) at: usize,
    <span class="doccomment">/// The index into the matching patterns of the next match to report if the
    /// current state is a match state. Note that this may be 1 greater than
    /// the total number of matches to report for the current match state. (In
    /// which case, no more matches should be reported at the current position
    /// and the search should advance to the next position.)
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) next_match_index: <span class="prelude-ty">Option</span>&lt;usize&gt;,
    <span class="doccomment">/// This is set to true when a reverse overlapping search has entered its
    /// EOI transitions.
    ///
    /// This isn't used in a forward search because it knows to stop once the
    /// position exceeds the end of the search range. In a reverse search,
    /// since we use unsigned offsets, we don't "know" once we've gone past
    /// `0`. So the only way to detect it is with this extra flag. The reverse
    /// overlapping search knows to terminate specifically after it has
    /// reported all matches after following the EOI transition.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) rev_eoi: bool,
}

<span class="kw">impl </span>OverlappingState {
    <span class="doccomment">/// Create a new overlapping state that begins at the start state of any
    /// automaton.
    </span><span class="kw">pub fn </span>start() -&gt; OverlappingState {
        OverlappingState {
            mat: <span class="prelude-val">None</span>,
            id: <span class="prelude-val">None</span>,
            at: <span class="number">0</span>,
            next_match_index: <span class="prelude-val">None</span>,
            rev_eoi: <span class="bool-val">false</span>,
        }
    }

    <span class="doccomment">/// Return the match result of the most recent search to execute with this
    /// state.
    ///
    /// A searches will clear this result automatically, such that if no
    /// match is found, this will correctly report `None`.
    </span><span class="kw">pub fn </span>get_match(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;HalfMatch&gt; {
        <span class="self">self</span>.mat
    }
}

<span class="doccomment">/// Runs the given overlapping `search` function (forwards or backwards) until
/// a match is found whose offset does not split a codepoint.
///
/// This is *not* always correct to call. It should only be called when the
/// underlying NFA has UTF-8 mode enabled *and* it can produce zero-width
/// matches. Calling this when both of those things aren't true might result
/// in legitimate matches getting skipped.
</span><span class="attr">#[cold]
#[inline(never)]
</span><span class="kw">fn </span>skip_empty_utf8_splits_overlapping&lt;F&gt;(
    input: <span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;,
    state: <span class="kw-2">&amp;mut </span>OverlappingState,
    <span class="kw-2">mut </span>search: F,
) -&gt; <span class="prelude-ty">Result</span>&lt;(), MatchError&gt;
<span class="kw">where
    </span>F: FnMut(<span class="kw-2">&amp;</span>Input&lt;<span class="lifetime">'_</span>&gt;, <span class="kw-2">&amp;mut </span>OverlappingState) -&gt; <span class="prelude-ty">Result</span>&lt;(), MatchError&gt;,
{
    <span class="comment">// Note that this routine works for forwards and reverse searches
    // even though there's no code here to handle those cases. That's
    // because overlapping searches drive themselves to completion via
    // `OverlappingState`. So all we have to do is push it until no matches are
    // found.

    </span><span class="kw">let </span><span class="kw-2">mut </span>hm = <span class="kw">match </span>state.get_match() {
        <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(()),
        <span class="prelude-val">Some</span>(hm) =&gt; hm,
    };
    <span class="kw">if </span>input.get_anchored().is_anchored() {
        <span class="kw">if </span>!input.is_char_boundary(hm.offset()) {
            state.mat = <span class="prelude-val">None</span>;
        }
        <span class="kw">return </span><span class="prelude-val">Ok</span>(());
    }
    <span class="kw">while </span>!input.is_char_boundary(hm.offset()) {
        search(input, state)<span class="question-mark">?</span>;
        hm = <span class="kw">match </span>state.get_match() {
            <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(()),
            <span class="prelude-val">Some</span>(hm) =&gt; hm,
        };
    }
    <span class="prelude-val">Ok</span>(())
}

<span class="doccomment">/// Based on the minimum number of states required for a useful lazy DFA cache,
/// this returns the minimum lazy state ID that must be representable.
///
/// It's not likely for this to have any impact 32-bit systems (or higher), but
/// on 16-bit systems, the lazy state ID space is quite constrained and thus
/// may be insufficient if our MIN_STATES value is (for some reason) too high.
</span><span class="kw">fn </span>minimum_lazy_state_id(
    classes: <span class="kw-2">&amp;</span>ByteClasses,
) -&gt; <span class="prelude-ty">Result</span>&lt;LazyStateID, LazyStateIDError&gt; {
    <span class="kw">let </span>stride = <span class="number">1 </span>&lt;&lt; classes.stride2();
    <span class="kw">let </span>min_state_index = MIN_STATES.checked_sub(<span class="number">1</span>).unwrap();
    LazyStateID::new(min_state_index * stride)
}

<span class="doccomment">/// Based on the minimum number of states required for a useful lazy DFA cache,
/// this returns a heuristic minimum number of bytes of heap space required.
///
/// This is a "heuristic" because the minimum it returns is likely bigger than
/// the true minimum. Namely, it assumes that each powerset NFA/DFA state uses
/// the maximum number of NFA states (all of them). This is likely bigger
/// than what is required in practice. Computing the true minimum effectively
/// requires determinization, which is probably too much work to do for a
/// simple check like this.
///
/// One of the issues with this approach IMO is that it requires that this
/// be in sync with the calculation above for computing how much heap memory
/// the DFA cache uses. If we get it wrong, it's possible for example for the
/// minimum to be smaller than the computed heap memory, and thus, it may be
/// the case that we can't add the required minimum number of states. That in
/// turn will make lazy DFA panic because we assume that we can add at least a
/// minimum number of states.
///
/// Another approach would be to always allow the minimum number of states to
/// be added to the lazy DFA cache, even if it exceeds the configured cache
/// limit. This does mean that the limit isn't really a limit in all cases,
/// which is unfortunate. But it does at least guarantee that the lazy DFA can
/// always make progress, even if it is slow. (This approach is very similar to
/// enabling the 'skip_cache_capacity_check' config knob, except it wouldn't
/// rely on cache size calculation. Instead, it would just always permit a
/// minimum number of states to be added.)
</span><span class="kw">fn </span>minimum_cache_capacity(
    nfa: <span class="kw-2">&amp;</span>thompson::NFA,
    classes: <span class="kw-2">&amp;</span>ByteClasses,
    starts_for_each_pattern: bool,
) -&gt; usize {
    <span class="kw">const </span>ID_SIZE: usize = size_of::&lt;LazyStateID&gt;();
    <span class="kw">const </span>STATE_SIZE: usize = size_of::&lt;State&gt;();

    <span class="kw">let </span>stride = <span class="number">1 </span>&lt;&lt; classes.stride2();
    <span class="kw">let </span>states_len = nfa.states().len();
    <span class="kw">let </span>sparses = <span class="number">2 </span>* states_len * NFAStateID::SIZE;
    <span class="kw">let </span>trans = MIN_STATES * stride * ID_SIZE;

    <span class="kw">let </span><span class="kw-2">mut </span>starts = Start::len() * ID_SIZE;
    <span class="kw">if </span>starts_for_each_pattern {
        starts += (Start::len() * nfa.pattern_len()) * ID_SIZE;
    }

    <span class="comment">// The min number of states HAS to be at least 4: we have 3 sentinel states
    // and then we need space for one more when we save a state after clearing
    // the cache. We also need space for one more, otherwise we get stuck in a
    // loop where we try to add a 5th state, which gets rejected, which clears
    // the cache, which adds back a saved state (4th total state) which then
    // tries to add the 5th state again.
    </span><span class="macro">assert!</span>(MIN_STATES &gt;= <span class="number">5</span>, <span class="string">"minimum number of states has to be at least 5"</span>);
    <span class="comment">// The minimum number of non-sentinel states. We consider this separately
    // because sentinel states are much smaller in that they contain no NFA
    // states. Given our aggressive calculation here, it's worth being more
    // precise with the number of states we need.
    </span><span class="kw">let </span>non_sentinel = MIN_STATES.checked_sub(SENTINEL_STATES).unwrap();

    <span class="comment">// Every `State` has 5 bytes for flags, 4 bytes (max) for the number of
    // patterns, followed by 32-bit encodings of patterns and then delta
    // varint encodings of NFA state IDs. We use the worst case (which isn't
    // technically possible) of 5 bytes for each NFA state ID.
    //
    // HOWEVER, three of the states needed by a lazy DFA are just the sentinel
    // unknown, dead and quit states. Those states have a known size and it is
    // small.
    </span><span class="kw">let </span>dead_state_size = State::dead().memory_usage();
    <span class="kw">let </span>max_state_size = <span class="number">5 </span>+ <span class="number">4 </span>+ (nfa.pattern_len() * <span class="number">4</span>) + (states_len * <span class="number">5</span>);
    <span class="kw">let </span>states = (SENTINEL_STATES * (STATE_SIZE + dead_state_size))
        + (non_sentinel * (STATE_SIZE + max_state_size));
    <span class="comment">// NOTE: We don't double count heap memory used by State for this map since
    // we use reference counting to avoid doubling memory usage. (This tends to
    // be where most memory is allocated in the cache.)
    </span><span class="kw">let </span>states_to_sid = (MIN_STATES * STATE_SIZE) + (MIN_STATES * ID_SIZE);
    <span class="kw">let </span>stack = states_len * NFAStateID::SIZE;
    <span class="kw">let </span>scratch_state_builder = max_state_size;

    trans
        + starts
        + states
        + states_to_sid
        + sparses
        + stack
        + scratch_state_builder
}

<span class="attr">#[cfg(all(test, feature = <span class="string">"syntax"</span>))]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="comment">// Tests that we handle heuristic Unicode word boundary support in reverse
    // DFAs in the specific case of contextual searches.
    //
    // I wrote this test when I discovered a bug in how heuristic word
    // boundaries were handled. Namely, that the starting state selection
    // didn't consider the DFA's quit byte set when looking at the byte
    // immediately before the start of the search (or immediately after the
    // end of the search in the case of a reverse search). As a result, it was
    // possible for '\bfoo\b' to match 'β123' because the trailing \xB2 byte
    // in the 'β' codepoint would be treated as a non-word character. But of
    // course, this search should trigger the DFA to quit, since there is a
    // non-ASCII byte in consideration.
    //
    // Thus, I fixed 'start_state_{forward,reverse}' to check the quit byte set
    // if it wasn't empty. The forward case is tested in the doc test for the
    // Config::unicode_word_boundary API. We test the reverse case here, which
    // is sufficiently niche that it doesn't really belong in a doc test.
    </span><span class="attr">#[test]
    </span><span class="kw">fn </span>heuristic_unicode_reverse() {
        <span class="kw">let </span>dfa = DFA::builder()
            .configure(DFA::config().unicode_word_boundary(<span class="bool-val">true</span>))
            .thompson(thompson::Config::new().reverse(<span class="bool-val">true</span>))
            .build(<span class="string">r"\b[0-9]+\b"</span>)
            .unwrap();
        <span class="kw">let </span><span class="kw-2">mut </span>cache = dfa.create_cache();

        <span class="kw">let </span>input = Input::new(<span class="string">"β123"</span>).range(<span class="number">2</span>..);
        <span class="kw">let </span>expected = MatchError::quit(<span class="number">0xB2</span>, <span class="number">1</span>);
        <span class="kw">let </span>got = dfa.try_search_rev(<span class="kw-2">&amp;mut </span>cache, <span class="kw-2">&amp;</span>input);
        <span class="macro">assert_eq!</span>(<span class="prelude-val">Err</span>(expected), got);

        <span class="kw">let </span>input = Input::new(<span class="string">"123β"</span>).range(..<span class="number">3</span>);
        <span class="kw">let </span>expected = MatchError::quit(<span class="number">0xCE</span>, <span class="number">3</span>);
        <span class="kw">let </span>got = dfa.try_search_rev(<span class="kw-2">&amp;mut </span>cache, <span class="kw-2">&amp;</span>input);
        <span class="macro">assert_eq!</span>(<span class="prelude-val">Err</span>(expected), got);
    }
}
</code></pre></div></section></main></body></html>