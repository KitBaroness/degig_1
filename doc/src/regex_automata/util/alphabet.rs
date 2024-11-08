<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.6/src/util/alphabet.rs`."><title>alphabet.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="regex_automata" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../regex_automata/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">/*!
This module provides APIs for dealing with the alphabets of finite state
machines.

There are two principal types in this module, [`ByteClasses`] and [`Unit`].
The former defines the alphabet of a finite state machine while the latter
represents an element of that alphabet.

To a first approximation, the alphabet of all automata in this crate is just
a `u8`. Namely, every distinct byte value. All 256 of them. In practice, this
can be quite wasteful when building a transition table for a DFA, since it
requires storing a state identifier for each element in the alphabet. Instead,
we collapse the alphabet of an automaton down into equivalence classes, where
every byte in the same equivalence class never discriminates between a match or
a non-match from any other byte in the same class. For example, in the regex
`[a-z]+`, then you could consider it having an alphabet consisting of two
equivalence classes: `a-z` and everything else. In terms of the transitions on
an automaton, it doesn't actually require representing every distinct byte.
Just the equivalence classes.

The downside of equivalence classes is that, of course, searching a haystack
deals with individual byte values. Those byte values need to be mapped to
their corresponding equivalence class. This is what `ByteClasses` does. In
practice, doing this for every state transition has negligible impact on modern
CPUs. Moreover, it helps make more efficient use of the CPU cache by (possibly
considerably) shrinking the size of the transition table.

One last hiccup concerns `Unit`. Namely, because of look-around and how the
DFAs in this crate work, we need to add a sentinel value to our alphabet
of equivalence classes that represents the "end" of a search. We call that
sentinel [`Unit::eoi`] or "end of input." Thus, a `Unit` is either an
equivalence class corresponding to a set of bytes, or it is a special "end of
input" sentinel.

In general, you should not expect to need either of these types unless you're
doing lower level shenanigans with DFAs, or even building your own DFAs.
(Although, you don't have to use these types to build your own DFAs of course.)
For example, if you're walking a DFA's state graph, it's probably useful to
make use of [`ByteClasses`] to visit each element in the DFA's alphabet instead
of just visiting every distinct `u8` value. The latter isn't necessarily wrong,
but it could be potentially very wasteful.
*/
</span><span class="kw">use </span><span class="kw">crate</span>::util::{
    escape::DebugByte,
    wire::{<span class="self">self</span>, DeserializeError, SerializeError},
};

<span class="doccomment">/// Unit represents a single unit of haystack for DFA based regex engines.
///
/// It is not expected for consumers of this crate to need to use this type
/// unless they are implementing their own DFA. And even then, it's not
/// required: implementors may use other techniques to handle haystack units.
///
/// Typically, a single unit of haystack for a DFA would be a single byte.
/// However, for the DFAs in this crate, matches are delayed by a single byte
/// in order to handle look-ahead assertions (`\b`, `$` and `\z`). Thus, once
/// we have consumed the haystack, we must run the DFA through one additional
/// transition using a unit that indicates the haystack has ended.
///
/// There is no way to represent a sentinel with a `u8` since all possible
/// values *may* be valid haystack units to a DFA, therefore this type
/// explicitly adds room for a sentinel value.
///
/// The sentinel EOI value is always its own equivalence class and is
/// ultimately represented by adding 1 to the maximum equivalence class value.
/// So for example, the regex `^[a-z]+$` might be split into the following
/// equivalence classes:
///
/// ```text
/// 0 =&gt; [\x00-`]
/// 1 =&gt; [a-z]
/// 2 =&gt; [{-\xFF]
/// 3 =&gt; [EOI]
/// ```
///
/// Where EOI is the special sentinel value that is always in its own
/// singleton equivalence class.
</span><span class="attr">#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
</span><span class="kw">pub struct </span>Unit(UnitKind);

<span class="attr">#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
</span><span class="kw">enum </span>UnitKind {
    <span class="doccomment">/// Represents a byte value, or more typically, an equivalence class
    /// represented as a byte value.
    </span>U8(u8),
    <span class="doccomment">/// Represents the "end of input" sentinel. We regretably use a `u16`
    /// here since the maximum sentinel value is `256`. Thankfully, we don't
    /// actually store a `Unit` anywhere, so this extra space shouldn't be too
    /// bad.
    </span>EOI(u16),
}

<span class="kw">impl </span>Unit {
    <span class="doccomment">/// Create a new haystack unit from a byte value.
    ///
    /// All possible byte values are legal. However, when creating a haystack
    /// unit for a specific DFA, one should be careful to only construct units
    /// that are in that DFA's alphabet. Namely, one way to compact a DFA's
    /// in-memory representation is to collapse its transitions to a set of
    /// equivalence classes into a set of all possible byte values. If a DFA
    /// uses equivalence classes instead of byte values, then the byte given
    /// here should be the equivalence class.
    </span><span class="kw">pub fn </span>u8(byte: u8) -&gt; Unit {
        Unit(UnitKind::U8(byte))
    }

    <span class="doccomment">/// Create a new "end of input" haystack unit.
    ///
    /// The value given is the sentinel value used by this unit to represent
    /// the "end of input." The value should be the total number of equivalence
    /// classes in the corresponding alphabet. Its maximum value is `256`,
    /// which occurs when every byte is its own equivalence class.
    ///
    /// # Panics
    ///
    /// This panics when `num_byte_equiv_classes` is greater than `256`.
    </span><span class="kw">pub fn </span>eoi(num_byte_equiv_classes: usize) -&gt; Unit {
        <span class="macro">assert!</span>(
            num_byte_equiv_classes &lt;= <span class="number">256</span>,
            <span class="string">"max number of byte-based equivalent classes is 256, but got {}"</span>,
            num_byte_equiv_classes,
        );
        Unit(UnitKind::EOI(u16::try_from(num_byte_equiv_classes).unwrap()))
    }

    <span class="doccomment">/// If this unit is not an "end of input" sentinel, then returns its
    /// underlying byte value. Otherwise return `None`.
    </span><span class="kw">pub fn </span>as_u8(<span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;u8&gt; {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0 </span>{
            UnitKind::U8(b) =&gt; <span class="prelude-val">Some</span>(b),
            UnitKind::EOI(<span class="kw">_</span>) =&gt; <span class="prelude-val">None</span>,
        }
    }

    <span class="doccomment">/// If this unit is an "end of input" sentinel, then return the underlying
    /// sentinel value that was given to [`Unit::eoi`]. Otherwise return
    /// `None`.
    </span><span class="kw">pub fn </span>as_eoi(<span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;u16&gt; {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0 </span>{
            UnitKind::U8(<span class="kw">_</span>) =&gt; <span class="prelude-val">None</span>,
            UnitKind::EOI(sentinel) =&gt; <span class="prelude-val">Some</span>(sentinel),
        }
    }

    <span class="doccomment">/// Return this unit as a `usize`, regardless of whether it is a byte value
    /// or an "end of input" sentinel. In the latter case, the underlying
    /// sentinel value given to [`Unit::eoi`] is returned.
    </span><span class="kw">pub fn </span>as_usize(<span class="self">self</span>) -&gt; usize {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0 </span>{
            UnitKind::U8(b) =&gt; usize::from(b),
            UnitKind::EOI(eoi) =&gt; usize::from(eoi),
        }
    }

    <span class="doccomment">/// Returns true if and only of this unit is a byte value equivalent to the
    /// byte given. This always returns false when this is an "end of input"
    /// sentinel.
    </span><span class="kw">pub fn </span>is_byte(<span class="self">self</span>, byte: u8) -&gt; bool {
        <span class="self">self</span>.as_u8().map_or(<span class="bool-val">false</span>, |b| b == byte)
    }

    <span class="doccomment">/// Returns true when this unit represents an "end of input" sentinel.
    </span><span class="kw">pub fn </span>is_eoi(<span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.as_eoi().is_some()
    }

    <span class="doccomment">/// Returns true when this unit corresponds to an ASCII word byte.
    ///
    /// This always returns false when this unit represents an "end of input"
    /// sentinel.
    </span><span class="kw">pub fn </span>is_word_byte(<span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.as_u8().map_or(<span class="bool-val">false</span>, <span class="kw">crate</span>::util::utf8::is_word_byte)
    }
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>Unit {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0 </span>{
            UnitKind::U8(b) =&gt; <span class="macro">write!</span>(f, <span class="string">"{:?}"</span>, DebugByte(b)),
            UnitKind::EOI(<span class="kw">_</span>) =&gt; <span class="macro">write!</span>(f, <span class="string">"EOI"</span>),
        }
    }
}

<span class="doccomment">/// A representation of byte oriented equivalence classes.
///
/// This is used in a DFA to reduce the size of the transition table. This can
/// have a particularly large impact not only on the total size of a dense DFA,
/// but also on compile times.
///
/// The essential idea here is that the alphabet of a DFA is shrunk from the
/// usual 256 distinct byte values down to a set of equivalence classes. The
/// guarantee you get is that any byte belonging to the same equivalence class
/// can be treated as if it were any other byte in the same class, and the
/// result of a search wouldn't change.
///
/// # Example
///
/// This example shows how to get byte classes from an
/// [`NFA`](crate::nfa::thompson::NFA) and ask for the class of various bytes.
///
/// ```
/// use regex_automata::nfa::thompson::NFA;
///
/// let nfa = NFA::new("[a-z]+")?;
/// let classes = nfa.byte_classes();
/// // 'a' and 'z' are in the same class for this regex.
/// assert_eq!(classes.get(b'a'), classes.get(b'z'));
/// // But 'a' and 'A' are not.
/// assert_ne!(classes.get(b'a'), classes.get(b'A'));
///
/// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
/// ```
</span><span class="attr">#[derive(Clone, Copy)]
</span><span class="kw">pub struct </span>ByteClasses([u8; <span class="number">256</span>]);

<span class="kw">impl </span>ByteClasses {
    <span class="doccomment">/// Creates a new set of equivalence classes where all bytes are mapped to
    /// the same class.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>empty() -&gt; ByteClasses {
        ByteClasses([<span class="number">0</span>; <span class="number">256</span>])
    }

    <span class="doccomment">/// Creates a new set of equivalence classes where each byte belongs to
    /// its own equivalence class.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>singletons() -&gt; ByteClasses {
        <span class="kw">let </span><span class="kw-2">mut </span>classes = ByteClasses::empty();
        <span class="kw">for </span>b <span class="kw">in </span><span class="number">0</span>..=<span class="number">255 </span>{
            classes.set(b, b);
        }
        classes
    }

    <span class="doccomment">/// Deserializes a byte class map from the given slice. If the slice is of
    /// insufficient length or otherwise contains an impossible mapping, then
    /// an error is returned. Upon success, the number of bytes read along with
    /// the map are returned. The number of bytes read is always a multiple of
    /// 8.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>from_bytes(
        slice: <span class="kw-2">&amp;</span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(ByteClasses, usize), DeserializeError&gt; {
        wire::check_slice_len(slice, <span class="number">256</span>, <span class="string">"byte class map"</span>)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>classes = ByteClasses::empty();
        <span class="kw">for </span>(b, <span class="kw-2">&amp;</span>class) <span class="kw">in </span>slice[..<span class="number">256</span>].iter().enumerate() {
            classes.set(u8::try_from(b).unwrap(), class);
        }
        <span class="comment">// We specifically don't use 'classes.iter()' here because that
        // iterator depends on 'classes.alphabet_len()' being correct. But that
        // is precisely the thing we're trying to verify below!
        </span><span class="kw">for </span><span class="kw-2">&amp;</span>b <span class="kw">in </span>classes.<span class="number">0</span>.iter() {
            <span class="kw">if </span>usize::from(b) &gt;= classes.alphabet_len() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(DeserializeError::generic(
                    <span class="string">"found equivalence class greater than alphabet len"</span>,
                ));
            }
        }
        <span class="prelude-val">Ok</span>((classes, <span class="number">256</span>))
    }

    <span class="doccomment">/// Writes this byte class map to the given byte buffer. if the given
    /// buffer is too small, then an error is returned. Upon success, the total
    /// number of bytes written is returned. The number of bytes written is
    /// guaranteed to be a multiple of 8.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>write_to(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        <span class="kw-2">mut </span>dst: <span class="kw-2">&amp;mut </span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;usize, SerializeError&gt; {
        <span class="kw">let </span>nwrite = <span class="self">self</span>.write_to_len();
        <span class="kw">if </span>dst.len() &lt; nwrite {
            <span class="kw">return </span><span class="prelude-val">Err</span>(SerializeError::buffer_too_small(<span class="string">"byte class map"</span>));
        }
        <span class="kw">for </span>b <span class="kw">in </span><span class="number">0</span>..=<span class="number">255 </span>{
            dst[<span class="number">0</span>] = <span class="self">self</span>.get(b);
            dst = <span class="kw-2">&amp;mut </span>dst[<span class="number">1</span>..];
        }
        <span class="prelude-val">Ok</span>(nwrite)
    }

    <span class="doccomment">/// Returns the total number of bytes written by `write_to`.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>write_to_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="number">256
    </span>}

    <span class="doccomment">/// Set the equivalence class for the given byte.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>set(<span class="kw-2">&amp;mut </span><span class="self">self</span>, byte: u8, class: u8) {
        <span class="self">self</span>.<span class="number">0</span>[usize::from(byte)] = class;
    }

    <span class="doccomment">/// Get the equivalence class for the given byte.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>get(<span class="kw-2">&amp;</span><span class="self">self</span>, byte: u8) -&gt; u8 {
        <span class="self">self</span>.<span class="number">0</span>[usize::from(byte)]
    }

    <span class="doccomment">/// Get the equivalence class for the given haystack unit and return the
    /// class as a `usize`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>get_by_unit(<span class="kw-2">&amp;</span><span class="self">self</span>, unit: Unit) -&gt; usize {
        <span class="kw">match </span>unit.<span class="number">0 </span>{
            UnitKind::U8(b) =&gt; usize::from(<span class="self">self</span>.get(b)),
            UnitKind::EOI(b) =&gt; usize::from(b),
        }
    }

    <span class="doccomment">/// Create a unit that represents the "end of input" sentinel based on the
    /// number of equivalence classes.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>eoi(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Unit {
        <span class="comment">// The alphabet length already includes the EOI sentinel, hence why
        // we subtract 1.
        </span>Unit::eoi(<span class="self">self</span>.alphabet_len().checked_sub(<span class="number">1</span>).unwrap())
    }

    <span class="doccomment">/// Return the total number of elements in the alphabet represented by
    /// these equivalence classes. Equivalently, this returns the total number
    /// of equivalence classes.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>alphabet_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="comment">// Add one since the number of equivalence classes is one bigger than
        // the last one. But add another to account for the final EOI class
        // that isn't explicitly represented.
        </span>usize::from(<span class="self">self</span>.<span class="number">0</span>[<span class="number">255</span>]) + <span class="number">1 </span>+ <span class="number">1
    </span>}

    <span class="doccomment">/// Returns the stride, as a base-2 exponent, required for these
    /// equivalence classes.
    ///
    /// The stride is always the smallest power of 2 that is greater than or
    /// equal to the alphabet length, and the `stride2` returned here is the
    /// exponent applied to `2` to get the smallest power. This is done so that
    /// converting between premultiplied state IDs and indices can be done with
    /// shifts alone, which is much faster than integer division.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>stride2(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="kw">let </span>zeros = <span class="self">self</span>.alphabet_len().next_power_of_two().trailing_zeros();
        usize::try_from(zeros).unwrap()
    }

    <span class="doccomment">/// Returns true if and only if every byte in this class maps to its own
    /// equivalence class. Equivalently, there are 257 equivalence classes
    /// and each class contains either exactly one byte or corresponds to the
    /// singleton class containing the "end of input" sentinel.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_singleton(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.alphabet_len() == <span class="number">257
    </span>}

    <span class="doccomment">/// Returns an iterator over all equivalence classes in this set.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>iter(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; ByteClassIter&lt;<span class="lifetime">'_</span>&gt; {
        ByteClassIter { classes: <span class="self">self</span>, i: <span class="number">0 </span>}
    }

    <span class="doccomment">/// Returns an iterator over a sequence of representative bytes from each
    /// equivalence class within the range of bytes given.
    ///
    /// When the given range is unbounded on both sides, the iterator yields
    /// exactly N items, where N is equivalent to the number of equivalence
    /// classes. Each item is an arbitrary byte drawn from each equivalence
    /// class.
    ///
    /// This is useful when one is determinizing an NFA and the NFA's alphabet
    /// hasn't been converted to equivalence classes. Picking an arbitrary byte
    /// from each equivalence class then permits a full exploration of the NFA
    /// instead of using every possible byte value and thus potentially saves
    /// quite a lot of redundant work.
    ///
    /// # Example
    ///
    /// This shows an example of what a complete sequence of representatives
    /// might look like from a real example.
    ///
    /// ```
    /// use regex_automata::{nfa::thompson::NFA, util::alphabet::Unit};
    ///
    /// let nfa = NFA::new("[a-z]+")?;
    /// let classes = nfa.byte_classes();
    /// let reps: Vec&lt;Unit&gt; = classes.representatives(..).collect();
    /// // Note that the specific byte values yielded are not guaranteed!
    /// let expected = vec![
    ///     Unit::u8(b'\x00'),
    ///     Unit::u8(b'a'),
    ///     Unit::u8(b'{'),
    ///     Unit::eoi(3),
    /// ];
    /// assert_eq!(expected, reps);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// Note though, that you can ask for an arbitrary range of bytes, and only
    /// representatives for that range will be returned:
    ///
    /// ```
    /// use regex_automata::{nfa::thompson::NFA, util::alphabet::Unit};
    ///
    /// let nfa = NFA::new("[a-z]+")?;
    /// let classes = nfa.byte_classes();
    /// let reps: Vec&lt;Unit&gt; = classes.representatives(b'A'..=b'z').collect();
    /// // Note that the specific byte values yielded are not guaranteed!
    /// let expected = vec![
    ///     Unit::u8(b'A'),
    ///     Unit::u8(b'a'),
    /// ];
    /// assert_eq!(expected, reps);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="kw">pub fn </span>representatives&lt;R: core::ops::RangeBounds&lt;u8&gt;&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        range: R,
    ) -&gt; ByteClassRepresentatives&lt;<span class="lifetime">'_</span>&gt; {
        <span class="kw">use </span>core::ops::Bound;

        <span class="kw">let </span>cur_byte = <span class="kw">match </span>range.start_bound() {
            Bound::Included(<span class="kw-2">&amp;</span>i) =&gt; usize::from(i),
            Bound::Excluded(<span class="kw-2">&amp;</span>i) =&gt; usize::from(i).checked_add(<span class="number">1</span>).unwrap(),
            Bound::Unbounded =&gt; <span class="number">0</span>,
        };
        <span class="kw">let </span>end_byte = <span class="kw">match </span>range.end_bound() {
            Bound::Included(<span class="kw-2">&amp;</span>i) =&gt; {
                <span class="prelude-val">Some</span>(usize::from(i).checked_add(<span class="number">1</span>).unwrap())
            }
            Bound::Excluded(<span class="kw-2">&amp;</span>i) =&gt; <span class="prelude-val">Some</span>(usize::from(i)),
            Bound::Unbounded =&gt; <span class="prelude-val">None</span>,
        };
        <span class="macro">assert_ne!</span>(
            cur_byte,
            usize::MAX,
            <span class="string">"start range must be less than usize::MAX"</span>,
        );
        ByteClassRepresentatives {
            classes: <span class="self">self</span>,
            cur_byte,
            end_byte,
            last_class: <span class="prelude-val">None</span>,
        }
    }

    <span class="doccomment">/// Returns an iterator of the bytes in the given equivalence class.
    ///
    /// This is useful when one needs to know the actual bytes that belong to
    /// an equivalence class. For example, conceptually speaking, accelerating
    /// a DFA state occurs when a state only has a few outgoing transitions.
    /// But in reality, what is required is that there are only a small
    /// number of distinct bytes that can lead to an outgoing transition. The
    /// difference is that any one transition can correspond to an equivalence
    /// class which may contains many bytes. Therefore, DFA state acceleration
    /// considers the actual elements in each equivalence class of each
    /// outgoing transition.
    ///
    /// # Example
    ///
    /// This shows an example of how to get all of the elements in an
    /// equivalence class.
    ///
    /// ```
    /// use regex_automata::{nfa::thompson::NFA, util::alphabet::Unit};
    ///
    /// let nfa = NFA::new("[a-z]+")?;
    /// let classes = nfa.byte_classes();
    /// let elements: Vec&lt;Unit&gt; = classes.elements(Unit::u8(1)).collect();
    /// let expected: Vec&lt;Unit&gt; = (b'a'..=b'z').map(Unit::u8).collect();
    /// assert_eq!(expected, elements);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>elements(<span class="kw-2">&amp;</span><span class="self">self</span>, class: Unit) -&gt; ByteClassElements {
        ByteClassElements { classes: <span class="self">self</span>, class, byte: <span class="number">0 </span>}
    }

    <span class="doccomment">/// Returns an iterator of byte ranges in the given equivalence class.
    ///
    /// That is, a sequence of contiguous ranges are returned. Typically, every
    /// class maps to a single contiguous range.
    </span><span class="kw">fn </span>element_ranges(<span class="kw-2">&amp;</span><span class="self">self</span>, class: Unit) -&gt; ByteClassElementRanges {
        ByteClassElementRanges { elements: <span class="self">self</span>.elements(class), range: <span class="prelude-val">None </span>}
    }
}

<span class="kw">impl </span>Default <span class="kw">for </span>ByteClasses {
    <span class="kw">fn </span>default() -&gt; ByteClasses {
        ByteClasses::singletons()
    }
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>ByteClasses {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; core::fmt::Result {
        <span class="kw">if </span><span class="self">self</span>.is_singleton() {
            <span class="macro">write!</span>(f, <span class="string">"ByteClasses({{singletons}})"</span>)
        } <span class="kw">else </span>{
            <span class="macro">write!</span>(f, <span class="string">"ByteClasses("</span>)<span class="question-mark">?</span>;
            <span class="kw">for </span>(i, class) <span class="kw">in </span><span class="self">self</span>.iter().enumerate() {
                <span class="kw">if </span>i &gt; <span class="number">0 </span>{
                    <span class="macro">write!</span>(f, <span class="string">", "</span>)<span class="question-mark">?</span>;
                }
                <span class="macro">write!</span>(f, <span class="string">"{:?} =&gt; ["</span>, class.as_usize())<span class="question-mark">?</span>;
                <span class="kw">for </span>(start, end) <span class="kw">in </span><span class="self">self</span>.element_ranges(class) {
                    <span class="kw">if </span>start == end {
                        <span class="macro">write!</span>(f, <span class="string">"{:?}"</span>, start)<span class="question-mark">?</span>;
                    } <span class="kw">else </span>{
                        <span class="macro">write!</span>(f, <span class="string">"{:?}-{:?}"</span>, start, end)<span class="question-mark">?</span>;
                    }
                }
                <span class="macro">write!</span>(f, <span class="string">"]"</span>)<span class="question-mark">?</span>;
            }
            <span class="macro">write!</span>(f, <span class="string">")"</span>)
        }
    }
}

<span class="doccomment">/// An iterator over each equivalence class.
///
/// The last element in this iterator always corresponds to [`Unit::eoi`].
///
/// This is created by the [`ByteClasses::iter`] method.
///
/// The lifetime `'a` refers to the lifetime of the byte classes that this
/// iterator was created from.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>ByteClassIter&lt;<span class="lifetime">'a</span>&gt; {
    classes: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>ByteClasses,
    i: usize,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>ByteClassIter&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = Unit;

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Unit&gt; {
        <span class="kw">if </span><span class="self">self</span>.i + <span class="number">1 </span>== <span class="self">self</span>.classes.alphabet_len() {
            <span class="self">self</span>.i += <span class="number">1</span>;
            <span class="prelude-val">Some</span>(<span class="self">self</span>.classes.eoi())
        } <span class="kw">else if </span><span class="self">self</span>.i &lt; <span class="self">self</span>.classes.alphabet_len() {
            <span class="kw">let </span>class = u8::try_from(<span class="self">self</span>.i).unwrap();
            <span class="self">self</span>.i += <span class="number">1</span>;
            <span class="prelude-val">Some</span>(Unit::u8(class))
        } <span class="kw">else </span>{
            <span class="prelude-val">None
        </span>}
    }
}

<span class="doccomment">/// An iterator over representative bytes from each equivalence class.
///
/// This is created by the [`ByteClasses::representatives`] method.
///
/// The lifetime `'a` refers to the lifetime of the byte classes that this
/// iterator was created from.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>ByteClassRepresentatives&lt;<span class="lifetime">'a</span>&gt; {
    classes: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>ByteClasses,
    cur_byte: usize,
    end_byte: <span class="prelude-ty">Option</span>&lt;usize&gt;,
    last_class: <span class="prelude-ty">Option</span>&lt;u8&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>ByteClassRepresentatives&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = Unit;

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Unit&gt; {
        <span class="kw">while </span><span class="self">self</span>.cur_byte &lt; <span class="self">self</span>.end_byte.unwrap_or(<span class="number">256</span>) {
            <span class="kw">let </span>byte = u8::try_from(<span class="self">self</span>.cur_byte).unwrap();
            <span class="kw">let </span>class = <span class="self">self</span>.classes.get(byte);
            <span class="self">self</span>.cur_byte += <span class="number">1</span>;

            <span class="kw">if </span><span class="self">self</span>.last_class != <span class="prelude-val">Some</span>(class) {
                <span class="self">self</span>.last_class = <span class="prelude-val">Some</span>(class);
                <span class="kw">return </span><span class="prelude-val">Some</span>(Unit::u8(byte));
            }
        }
        <span class="kw">if </span><span class="self">self</span>.cur_byte != usize::MAX &amp;&amp; <span class="self">self</span>.end_byte.is_none() {
            <span class="comment">// Using usize::MAX as a sentinel is OK because we ban usize::MAX
            // from appearing as a start bound in iterator construction. But
            // why do it this way? Well, we want to return the EOI class
            // whenever the end of the given range is unbounded because EOI
            // isn't really a "byte" per se, so the only way it should be
            // excluded is if there is a bounded end to the range. Therefore,
            // when the end is unbounded, we just need to know whether we've
            // reported EOI or not. When we do, we set cur_byte to a value it
            // can never otherwise be.
            </span><span class="self">self</span>.cur_byte = usize::MAX;
            <span class="kw">return </span><span class="prelude-val">Some</span>(<span class="self">self</span>.classes.eoi());
        }
        <span class="prelude-val">None
    </span>}
}

<span class="doccomment">/// An iterator over all elements in an equivalence class.
///
/// This is created by the [`ByteClasses::elements`] method.
///
/// The lifetime `'a` refers to the lifetime of the byte classes that this
/// iterator was created from.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>ByteClassElements&lt;<span class="lifetime">'a</span>&gt; {
    classes: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>ByteClasses,
    class: Unit,
    byte: usize,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>ByteClassElements&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = Unit;

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Unit&gt; {
        <span class="kw">while </span><span class="self">self</span>.byte &lt; <span class="number">256 </span>{
            <span class="kw">let </span>byte = u8::try_from(<span class="self">self</span>.byte).unwrap();
            <span class="self">self</span>.byte += <span class="number">1</span>;
            <span class="kw">if </span><span class="self">self</span>.class.is_byte(<span class="self">self</span>.classes.get(byte)) {
                <span class="kw">return </span><span class="prelude-val">Some</span>(Unit::u8(byte));
            }
        }
        <span class="kw">if </span><span class="self">self</span>.byte &lt; <span class="number">257 </span>{
            <span class="self">self</span>.byte += <span class="number">1</span>;
            <span class="kw">if </span><span class="self">self</span>.class.is_eoi() {
                <span class="kw">return </span><span class="prelude-val">Some</span>(Unit::eoi(<span class="number">256</span>));
            }
        }
        <span class="prelude-val">None
    </span>}
}

<span class="doccomment">/// An iterator over all elements in an equivalence class expressed as a
/// sequence of contiguous ranges.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">struct </span>ByteClassElementRanges&lt;<span class="lifetime">'a</span>&gt; {
    elements: ByteClassElements&lt;<span class="lifetime">'a</span>&gt;,
    range: <span class="prelude-ty">Option</span>&lt;(Unit, Unit)&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>ByteClassElementRanges&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = (Unit, Unit);

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;(Unit, Unit)&gt; {
        <span class="kw">loop </span>{
            <span class="kw">let </span>element = <span class="kw">match </span><span class="self">self</span>.elements.next() {
                <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="self">self</span>.range.take(),
                <span class="prelude-val">Some</span>(element) =&gt; element,
            };
            <span class="kw">match </span><span class="self">self</span>.range.take() {
                <span class="prelude-val">None </span>=&gt; {
                    <span class="self">self</span>.range = <span class="prelude-val">Some</span>((element, element));
                }
                <span class="prelude-val">Some</span>((start, end)) =&gt; {
                    <span class="kw">if </span>end.as_usize() + <span class="number">1 </span>!= element.as_usize()
                        || element.is_eoi()
                    {
                        <span class="self">self</span>.range = <span class="prelude-val">Some</span>((element, element));
                        <span class="kw">return </span><span class="prelude-val">Some</span>((start, end));
                    }
                    <span class="self">self</span>.range = <span class="prelude-val">Some</span>((start, element));
                }
            }
        }
    }
}

<span class="doccomment">/// A partitioning of bytes into equivalence classes.
///
/// A byte class set keeps track of an *approximation* of equivalence classes
/// of bytes during NFA construction. That is, every byte in an equivalence
/// class cannot discriminate between a match and a non-match.
///
/// For example, in the regex `[ab]+`, the bytes `a` and `b` would be in the
/// same equivalence class because it never matters whether an `a` or a `b` is
/// seen, and no combination of `a`s and `b`s in the text can discriminate a
/// match.
///
/// Note though that this does not compute the minimal set of equivalence
/// classes. For example, in the regex `[ac]+`, both `a` and `c` are in the
/// same equivalence class for the same reason that `a` and `b` are in the
/// same equivalence class in the aforementioned regex. However, in this
/// implementation, `a` and `c` are put into distinct equivalence classes. The
/// reason for this is implementation complexity. In the future, we should
/// endeavor to compute the minimal equivalence classes since they can have a
/// rather large impact on the size of the DFA. (Doing this will likely require
/// rethinking how equivalence classes are computed, including changing the
/// representation here, which is only able to group contiguous bytes into the
/// same equivalence class.)
</span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
#[derive(Clone, Debug)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>ByteClassSet(ByteSet);

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">impl </span>Default <span class="kw">for </span>ByteClassSet {
    <span class="kw">fn </span>default() -&gt; ByteClassSet {
        ByteClassSet::empty()
    }
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">impl </span>ByteClassSet {
    <span class="doccomment">/// Create a new set of byte classes where all bytes are part of the same
    /// equivalence class.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>empty() -&gt; <span class="self">Self </span>{
        ByteClassSet(ByteSet::empty())
    }

    <span class="doccomment">/// Indicate the the range of byte given (inclusive) can discriminate a
    /// match between it and all other bytes outside of the range.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>set_range(<span class="kw-2">&amp;mut </span><span class="self">self</span>, start: u8, end: u8) {
        <span class="macro">debug_assert!</span>(start &lt;= end);
        <span class="kw">if </span>start &gt; <span class="number">0 </span>{
            <span class="self">self</span>.<span class="number">0</span>.add(start - <span class="number">1</span>);
        }
        <span class="self">self</span>.<span class="number">0</span>.add(end);
    }

    <span class="doccomment">/// Add the contiguous ranges in the set given to this byte class set.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>add_set(<span class="kw-2">&amp;mut </span><span class="self">self</span>, set: <span class="kw-2">&amp;</span>ByteSet) {
        <span class="kw">for </span>(start, end) <span class="kw">in </span>set.iter_ranges() {
            <span class="self">self</span>.set_range(start, end);
        }
    }

    <span class="doccomment">/// Convert this boolean set to a map that maps all byte values to their
    /// corresponding equivalence class. The last mapping indicates the largest
    /// equivalence class identifier (which is never bigger than 255).
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>byte_classes(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; ByteClasses {
        <span class="kw">let </span><span class="kw-2">mut </span>classes = ByteClasses::empty();
        <span class="kw">let </span><span class="kw-2">mut </span>class = <span class="number">0u8</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>b = <span class="number">0u8</span>;
        <span class="kw">loop </span>{
            classes.set(b, class);
            <span class="kw">if </span>b == <span class="number">255 </span>{
                <span class="kw">break</span>;
            }
            <span class="kw">if </span><span class="self">self</span>.<span class="number">0</span>.contains(b) {
                class = class.checked_add(<span class="number">1</span>).unwrap();
            }
            b = b.checked_add(<span class="number">1</span>).unwrap();
        }
        classes
    }
}

<span class="doccomment">/// A simple set of bytes that is reasonably cheap to copy and allocation free.
</span><span class="attr">#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>ByteSet {
    bits: BitSet,
}

<span class="doccomment">/// The representation of a byte set. Split out so that we can define a
/// convenient Debug impl for it while keeping "ByteSet" in the output.
</span><span class="attr">#[derive(Clone, Copy, Default, Eq, PartialEq)]
</span><span class="kw">struct </span>BitSet([u128; <span class="number">2</span>]);

<span class="kw">impl </span>ByteSet {
    <span class="doccomment">/// Create an empty set of bytes.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>empty() -&gt; ByteSet {
        ByteSet { bits: BitSet([<span class="number">0</span>; <span class="number">2</span>]) }
    }

    <span class="doccomment">/// Add a byte to this set.
    ///
    /// If the given byte already belongs to this set, then this is a no-op.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>add(<span class="kw-2">&amp;mut </span><span class="self">self</span>, byte: u8) {
        <span class="kw">let </span>bucket = byte / <span class="number">128</span>;
        <span class="kw">let </span>bit = byte % <span class="number">128</span>;
        <span class="self">self</span>.bits.<span class="number">0</span>[usize::from(bucket)] |= <span class="number">1 </span>&lt;&lt; bit;
    }

    <span class="doccomment">/// Remove a byte from this set.
    ///
    /// If the given byte is not in this set, then this is a no-op.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>remove(<span class="kw-2">&amp;mut </span><span class="self">self</span>, byte: u8) {
        <span class="kw">let </span>bucket = byte / <span class="number">128</span>;
        <span class="kw">let </span>bit = byte % <span class="number">128</span>;
        <span class="self">self</span>.bits.<span class="number">0</span>[usize::from(bucket)] &amp;= !(<span class="number">1 </span>&lt;&lt; bit);
    }

    <span class="doccomment">/// Return true if and only if the given byte is in this set.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>contains(<span class="kw-2">&amp;</span><span class="self">self</span>, byte: u8) -&gt; bool {
        <span class="kw">let </span>bucket = byte / <span class="number">128</span>;
        <span class="kw">let </span>bit = byte % <span class="number">128</span>;
        <span class="self">self</span>.bits.<span class="number">0</span>[usize::from(bucket)] &amp; (<span class="number">1 </span>&lt;&lt; bit) &gt; <span class="number">0
    </span>}

    <span class="doccomment">/// Return true if and only if the given inclusive range of bytes is in
    /// this set.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>contains_range(<span class="kw-2">&amp;</span><span class="self">self</span>, start: u8, end: u8) -&gt; bool {
        (start..=end).all(|b| <span class="self">self</span>.contains(b))
    }

    <span class="doccomment">/// Returns an iterator over all bytes in this set.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>iter(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; ByteSetIter {
        ByteSetIter { set: <span class="self">self</span>, b: <span class="number">0 </span>}
    }

    <span class="doccomment">/// Returns an iterator over all contiguous ranges of bytes in this set.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>iter_ranges(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; ByteSetRangeIter {
        ByteSetRangeIter { set: <span class="self">self</span>, b: <span class="number">0 </span>}
    }

    <span class="doccomment">/// Return true if and only if this set is empty.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"perf-inline"</span>, inline(always))]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_empty(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.bits.<span class="number">0 </span>== [<span class="number">0</span>, <span class="number">0</span>]
    }

    <span class="doccomment">/// Deserializes a byte set from the given slice. If the slice is of
    /// incorrect length or is otherwise malformed, then an error is returned.
    /// Upon success, the number of bytes read along with the set are returned.
    /// The number of bytes read is always a multiple of 8.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>from_bytes(
        slice: <span class="kw-2">&amp;</span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(ByteSet, usize), DeserializeError&gt; {
        <span class="kw">use </span>core::mem::size_of;

        wire::check_slice_len(slice, <span class="number">2 </span>* size_of::&lt;u128&gt;(), <span class="string">"byte set"</span>)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>nread = <span class="number">0</span>;
        <span class="kw">let </span>(low, nr) = wire::try_read_u128(slice, <span class="string">"byte set low bucket"</span>)<span class="question-mark">?</span>;
        nread += nr;
        <span class="kw">let </span>(high, nr) = wire::try_read_u128(slice, <span class="string">"byte set high bucket"</span>)<span class="question-mark">?</span>;
        nread += nr;
        <span class="prelude-val">Ok</span>((ByteSet { bits: BitSet([low, high]) }, nread))
    }

    <span class="doccomment">/// Writes this byte set to the given byte buffer. If the given buffer is
    /// too small, then an error is returned. Upon success, the total number of
    /// bytes written is returned. The number of bytes written is guaranteed to
    /// be a multiple of 8.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>write_to&lt;E: <span class="kw">crate</span>::util::wire::Endian&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        dst: <span class="kw-2">&amp;mut </span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;usize, SerializeError&gt; {
        <span class="kw">use </span>core::mem::size_of;

        <span class="kw">let </span>nwrite = <span class="self">self</span>.write_to_len();
        <span class="kw">if </span>dst.len() &lt; nwrite {
            <span class="kw">return </span><span class="prelude-val">Err</span>(SerializeError::buffer_too_small(<span class="string">"byte set"</span>));
        }
        <span class="kw">let </span><span class="kw-2">mut </span>nw = <span class="number">0</span>;
        E::write_u128(<span class="self">self</span>.bits.<span class="number">0</span>[<span class="number">0</span>], <span class="kw-2">&amp;mut </span>dst[nw..]);
        nw += size_of::&lt;u128&gt;();
        E::write_u128(<span class="self">self</span>.bits.<span class="number">0</span>[<span class="number">1</span>], <span class="kw-2">&amp;mut </span>dst[nw..]);
        nw += size_of::&lt;u128&gt;();
        <span class="macro">assert_eq!</span>(nwrite, nw, <span class="string">"expected to write certain number of bytes"</span>,);
        <span class="macro">assert_eq!</span>(
            nw % <span class="number">8</span>,
            <span class="number">0</span>,
            <span class="string">"expected to write multiple of 8 bytes for byte set"</span>,
        );
        <span class="prelude-val">Ok</span>(nw)
    }

    <span class="doccomment">/// Returns the total number of bytes written by `write_to`.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>write_to_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="number">2 </span>* core::mem::size_of::&lt;u128&gt;()
    }
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>BitSet {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        <span class="kw">let </span><span class="kw-2">mut </span>fmtd = f.debug_set();
        <span class="kw">for </span>b <span class="kw">in </span><span class="number">0u8</span>..=<span class="number">255 </span>{
            <span class="kw">if </span>(ByteSet { bits: <span class="kw-2">*</span><span class="self">self </span>}).contains(b) {
                fmtd.entry(<span class="kw-2">&amp;</span>b);
            }
        }
        fmtd.finish()
    }
}

<span class="attr">#[derive(Debug)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>ByteSetIter&lt;<span class="lifetime">'a</span>&gt; {
    set: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>ByteSet,
    b: usize,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>ByteSetIter&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = u8;

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;u8&gt; {
        <span class="kw">while </span><span class="self">self</span>.b &lt;= <span class="number">255 </span>{
            <span class="kw">let </span>b = u8::try_from(<span class="self">self</span>.b).unwrap();
            <span class="self">self</span>.b += <span class="number">1</span>;
            <span class="kw">if </span><span class="self">self</span>.set.contains(b) {
                <span class="kw">return </span><span class="prelude-val">Some</span>(b);
            }
        }
        <span class="prelude-val">None
    </span>}
}

<span class="attr">#[derive(Debug)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>ByteSetRangeIter&lt;<span class="lifetime">'a</span>&gt; {
    set: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>ByteSet,
    b: usize,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>ByteSetRangeIter&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = (u8, u8);

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;(u8, u8)&gt; {
        <span class="kw">let </span>asu8 = |n: usize| u8::try_from(n).unwrap();
        <span class="kw">while </span><span class="self">self</span>.b &lt;= <span class="number">255 </span>{
            <span class="kw">let </span>start = asu8(<span class="self">self</span>.b);
            <span class="self">self</span>.b += <span class="number">1</span>;
            <span class="kw">if </span>!<span class="self">self</span>.set.contains(start) {
                <span class="kw">continue</span>;
            }

            <span class="kw">let </span><span class="kw-2">mut </span>end = start;
            <span class="kw">while </span><span class="self">self</span>.b &lt;= <span class="number">255 </span>&amp;&amp; <span class="self">self</span>.set.contains(asu8(<span class="self">self</span>.b)) {
                end = asu8(<span class="self">self</span>.b);
                <span class="self">self</span>.b += <span class="number">1</span>;
            }
            <span class="kw">return </span><span class="prelude-val">Some</span>((start, end));
        }
        <span class="prelude-val">None
    </span>}
}

<span class="attr">#[cfg(all(test, feature = <span class="string">"alloc"</span>))]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span>alloc::{vec, vec::Vec};

    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>byte_classes() {
        <span class="kw">let </span><span class="kw-2">mut </span>set = ByteClassSet::empty();
        set.set_range(<span class="string">b'a'</span>, <span class="string">b'z'</span>);

        <span class="kw">let </span>classes = set.byte_classes();
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">0</span>), <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">1</span>), <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">2</span>), <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="string">b'a' </span>- <span class="number">1</span>), <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="string">b'a'</span>), <span class="number">1</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="string">b'm'</span>), <span class="number">1</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="string">b'z'</span>), <span class="number">1</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="string">b'z' </span>+ <span class="number">1</span>), <span class="number">2</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">254</span>), <span class="number">2</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">255</span>), <span class="number">2</span>);

        <span class="kw">let </span><span class="kw-2">mut </span>set = ByteClassSet::empty();
        set.set_range(<span class="number">0</span>, <span class="number">2</span>);
        set.set_range(<span class="number">4</span>, <span class="number">6</span>);
        <span class="kw">let </span>classes = set.byte_classes();
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">0</span>), <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">1</span>), <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">2</span>), <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">3</span>), <span class="number">1</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">4</span>), <span class="number">2</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">5</span>), <span class="number">2</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">6</span>), <span class="number">2</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">7</span>), <span class="number">3</span>);
        <span class="macro">assert_eq!</span>(classes.get(<span class="number">255</span>), <span class="number">3</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>full_byte_classes() {
        <span class="kw">let </span><span class="kw-2">mut </span>set = ByteClassSet::empty();
        <span class="kw">for </span>b <span class="kw">in </span><span class="number">0u8</span>..=<span class="number">255 </span>{
            set.set_range(b, b);
        }
        <span class="macro">assert_eq!</span>(set.byte_classes().alphabet_len(), <span class="number">257</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>elements_typical() {
        <span class="kw">let </span><span class="kw-2">mut </span>set = ByteClassSet::empty();
        set.set_range(<span class="string">b'b'</span>, <span class="string">b'd'</span>);
        set.set_range(<span class="string">b'g'</span>, <span class="string">b'm'</span>);
        set.set_range(<span class="string">b'z'</span>, <span class="string">b'z'</span>);
        <span class="kw">let </span>classes = set.byte_classes();
        <span class="comment">// class 0: \x00-a
        // class 1: b-d
        // class 2: e-f
        // class 3: g-m
        // class 4: n-y
        // class 5: z-z
        // class 6: \x7B-\xFF
        // class 7: EOI
        </span><span class="macro">assert_eq!</span>(classes.alphabet_len(), <span class="number">8</span>);

        <span class="kw">let </span>elements = classes.elements(Unit::u8(<span class="number">0</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(elements.len(), <span class="number">98</span>);
        <span class="macro">assert_eq!</span>(elements[<span class="number">0</span>], Unit::u8(<span class="string">b'\x00'</span>));
        <span class="macro">assert_eq!</span>(elements[<span class="number">97</span>], Unit::u8(<span class="string">b'a'</span>));

        <span class="kw">let </span>elements = classes.elements(Unit::u8(<span class="number">1</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(
            elements,
            <span class="macro">vec!</span>[Unit::u8(<span class="string">b'b'</span>), Unit::u8(<span class="string">b'c'</span>), Unit::u8(<span class="string">b'd'</span>)],
        );

        <span class="kw">let </span>elements = classes.elements(Unit::u8(<span class="number">2</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(elements, <span class="macro">vec!</span>[Unit::u8(<span class="string">b'e'</span>), Unit::u8(<span class="string">b'f'</span>)],);

        <span class="kw">let </span>elements = classes.elements(Unit::u8(<span class="number">3</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(
            elements,
            <span class="macro">vec!</span>[
                Unit::u8(<span class="string">b'g'</span>),
                Unit::u8(<span class="string">b'h'</span>),
                Unit::u8(<span class="string">b'i'</span>),
                Unit::u8(<span class="string">b'j'</span>),
                Unit::u8(<span class="string">b'k'</span>),
                Unit::u8(<span class="string">b'l'</span>),
                Unit::u8(<span class="string">b'm'</span>),
            ],
        );

        <span class="kw">let </span>elements = classes.elements(Unit::u8(<span class="number">4</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(elements.len(), <span class="number">12</span>);
        <span class="macro">assert_eq!</span>(elements[<span class="number">0</span>], Unit::u8(<span class="string">b'n'</span>));
        <span class="macro">assert_eq!</span>(elements[<span class="number">11</span>], Unit::u8(<span class="string">b'y'</span>));

        <span class="kw">let </span>elements = classes.elements(Unit::u8(<span class="number">5</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(elements, <span class="macro">vec!</span>[Unit::u8(<span class="string">b'z'</span>)]);

        <span class="kw">let </span>elements = classes.elements(Unit::u8(<span class="number">6</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(elements.len(), <span class="number">133</span>);
        <span class="macro">assert_eq!</span>(elements[<span class="number">0</span>], Unit::u8(<span class="string">b'\x7B'</span>));
        <span class="macro">assert_eq!</span>(elements[<span class="number">132</span>], Unit::u8(<span class="string">b'\xFF'</span>));

        <span class="kw">let </span>elements = classes.elements(Unit::eoi(<span class="number">7</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(elements, <span class="macro">vec!</span>[Unit::eoi(<span class="number">256</span>)]);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>elements_singletons() {
        <span class="kw">let </span>classes = ByteClasses::singletons();
        <span class="macro">assert_eq!</span>(classes.alphabet_len(), <span class="number">257</span>);

        <span class="kw">let </span>elements = classes.elements(Unit::u8(<span class="string">b'a'</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(elements, <span class="macro">vec!</span>[Unit::u8(<span class="string">b'a'</span>)]);

        <span class="kw">let </span>elements = classes.elements(Unit::eoi(<span class="number">5</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(elements, <span class="macro">vec!</span>[Unit::eoi(<span class="number">256</span>)]);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>elements_empty() {
        <span class="kw">let </span>classes = ByteClasses::empty();
        <span class="macro">assert_eq!</span>(classes.alphabet_len(), <span class="number">2</span>);

        <span class="kw">let </span>elements = classes.elements(Unit::u8(<span class="number">0</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(elements.len(), <span class="number">256</span>);
        <span class="macro">assert_eq!</span>(elements[<span class="number">0</span>], Unit::u8(<span class="string">b'\x00'</span>));
        <span class="macro">assert_eq!</span>(elements[<span class="number">255</span>], Unit::u8(<span class="string">b'\xFF'</span>));

        <span class="kw">let </span>elements = classes.elements(Unit::eoi(<span class="number">1</span>)).collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        <span class="macro">assert_eq!</span>(elements, <span class="macro">vec!</span>[Unit::eoi(<span class="number">256</span>)]);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>representatives() {
        <span class="kw">let </span><span class="kw-2">mut </span>set = ByteClassSet::empty();
        set.set_range(<span class="string">b'b'</span>, <span class="string">b'd'</span>);
        set.set_range(<span class="string">b'g'</span>, <span class="string">b'm'</span>);
        set.set_range(<span class="string">b'z'</span>, <span class="string">b'z'</span>);
        <span class="kw">let </span>classes = set.byte_classes();

        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(..).collect();
        <span class="kw">let </span>expected = <span class="macro">vec!</span>[
            Unit::u8(<span class="string">b'\x00'</span>),
            Unit::u8(<span class="string">b'b'</span>),
            Unit::u8(<span class="string">b'e'</span>),
            Unit::u8(<span class="string">b'g'</span>),
            Unit::u8(<span class="string">b'n'</span>),
            Unit::u8(<span class="string">b'z'</span>),
            Unit::u8(<span class="string">b'\x7B'</span>),
            Unit::eoi(<span class="number">7</span>),
        ];
        <span class="macro">assert_eq!</span>(expected, got);

        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(..<span class="number">0</span>).collect();
        <span class="macro">assert!</span>(got.is_empty());
        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(<span class="number">1</span>..<span class="number">1</span>).collect();
        <span class="macro">assert!</span>(got.is_empty());
        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(<span class="number">255</span>..<span class="number">255</span>).collect();
        <span class="macro">assert!</span>(got.is_empty());

        <span class="comment">// A weird case that is the only guaranteed to way to get an iterator
        // of just the EOI class by excluding all possible byte values.
        </span><span class="kw">let </span>got: Vec&lt;Unit&gt; = classes
            .representatives((
                core::ops::Bound::Excluded(<span class="number">255</span>),
                core::ops::Bound::Unbounded,
            ))
            .collect();
        <span class="kw">let </span>expected = <span class="macro">vec!</span>[Unit::eoi(<span class="number">7</span>)];
        <span class="macro">assert_eq!</span>(expected, got);

        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(..=<span class="number">255</span>).collect();
        <span class="kw">let </span>expected = <span class="macro">vec!</span>[
            Unit::u8(<span class="string">b'\x00'</span>),
            Unit::u8(<span class="string">b'b'</span>),
            Unit::u8(<span class="string">b'e'</span>),
            Unit::u8(<span class="string">b'g'</span>),
            Unit::u8(<span class="string">b'n'</span>),
            Unit::u8(<span class="string">b'z'</span>),
            Unit::u8(<span class="string">b'\x7B'</span>),
        ];
        <span class="macro">assert_eq!</span>(expected, got);

        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(<span class="string">b'b'</span>..=<span class="string">b'd'</span>).collect();
        <span class="kw">let </span>expected = <span class="macro">vec!</span>[Unit::u8(<span class="string">b'b'</span>)];
        <span class="macro">assert_eq!</span>(expected, got);

        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(<span class="string">b'a'</span>..=<span class="string">b'd'</span>).collect();
        <span class="kw">let </span>expected = <span class="macro">vec!</span>[Unit::u8(<span class="string">b'a'</span>), Unit::u8(<span class="string">b'b'</span>)];
        <span class="macro">assert_eq!</span>(expected, got);

        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(<span class="string">b'b'</span>..=<span class="string">b'e'</span>).collect();
        <span class="kw">let </span>expected = <span class="macro">vec!</span>[Unit::u8(<span class="string">b'b'</span>), Unit::u8(<span class="string">b'e'</span>)];
        <span class="macro">assert_eq!</span>(expected, got);

        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(<span class="string">b'A'</span>..=<span class="string">b'Z'</span>).collect();
        <span class="kw">let </span>expected = <span class="macro">vec!</span>[Unit::u8(<span class="string">b'A'</span>)];
        <span class="macro">assert_eq!</span>(expected, got);

        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(<span class="string">b'A'</span>..=<span class="string">b'z'</span>).collect();
        <span class="kw">let </span>expected = <span class="macro">vec!</span>[
            Unit::u8(<span class="string">b'A'</span>),
            Unit::u8(<span class="string">b'b'</span>),
            Unit::u8(<span class="string">b'e'</span>),
            Unit::u8(<span class="string">b'g'</span>),
            Unit::u8(<span class="string">b'n'</span>),
            Unit::u8(<span class="string">b'z'</span>),
        ];
        <span class="macro">assert_eq!</span>(expected, got);

        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(<span class="string">b'z'</span>..).collect();
        <span class="kw">let </span>expected = <span class="macro">vec!</span>[Unit::u8(<span class="string">b'z'</span>), Unit::u8(<span class="string">b'\x7B'</span>), Unit::eoi(<span class="number">7</span>)];
        <span class="macro">assert_eq!</span>(expected, got);

        <span class="kw">let </span>got: Vec&lt;Unit&gt; = classes.representatives(<span class="string">b'z'</span>..=<span class="number">0xFF</span>).collect();
        <span class="kw">let </span>expected = <span class="macro">vec!</span>[Unit::u8(<span class="string">b'z'</span>), Unit::u8(<span class="string">b'\x7B'</span>)];
        <span class="macro">assert_eq!</span>(expected, got);
    }
}
</code></pre></div></section></main></body></html>