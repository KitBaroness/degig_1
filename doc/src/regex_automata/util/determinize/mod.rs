<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.6/src/util/determinize/mod.rs`."><title>mod.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../" data-static-root-path="../../../../static.files/" data-current-crate="regex_automata" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../src-files.js"></script><script defer src="../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../regex_automata/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">/*!
This module contains types and routines for implementing determinization.

In this crate, there are at least two places where we implement
determinization: fully ahead-of-time compiled DFAs in the `dfa` module and
lazily compiled DFAs in the `hybrid` module. The stuff in this module
corresponds to the things that are in common between these implementations.

There are three broad things that our implementations of determinization have
in common, as defined by this module:

* The classification of start states. That is, whether we're dealing with
word boundaries, line boundaries, etc., is all the same. This also includes
the look-behind assertions that are satisfied by each starting state
classification.
* The representation of DFA states as sets of NFA states, including
convenience types for building these DFA states that are amenable to reusing
allocations.
* Routines for the "classical" parts of determinization: computing the
epsilon closure, tracking match states (with corresponding pattern IDs, since
we support multi-pattern finite automata) and, of course, computing the
transition function between states for units of input.

I did consider a couple of alternatives to this particular form of code reuse:

1. Don't do any code reuse. The problem here is that we *really* want both
forms of determinization to do exactly identical things when it comes to
their handling of NFA states. While our tests generally ensure this, the code
is tricky and large enough where not reusing code is a pretty big bummer.

2. Implement all of determinization once and make it generic over fully
compiled DFAs and lazily compiled DFAs. While I didn't actually try this
approach, my instinct is that it would be more complex than is needed here.
And the interface required would be pretty hairy. Instead, I think splitting
it into logical sub-components works better.
*/

</span><span class="kw">use </span>alloc::vec::Vec;

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span><span class="self">self</span>::state::{
    State, StateBuilderEmpty, StateBuilderMatches, StateBuilderNFA,
};

<span class="kw">use crate</span>::{
    nfa::thompson,
    util::{
        alphabet,
        look::{Look, LookSet},
        primitives::StateID,
        search::MatchKind,
        sparse_set::{SparseSet, SparseSets},
        start::Start,
        utf8,
    },
};

<span class="kw">mod </span>state;

<span class="doccomment">/// Compute the set of all reachable NFA states, including the full epsilon
/// closure, from a DFA state for a single unit of input. The set of reachable
/// states is returned as a `StateBuilderNFA`. The `StateBuilderNFA` returned
/// also includes any look-behind assertions satisfied by `unit`, in addition
/// to whether it is a match state. For multi-pattern DFAs, the builder will
/// also include the pattern IDs that match (in the order seen).
///
/// `nfa` must be able to resolve any NFA state in `state` and any NFA state
/// reachable via the epsilon closure of any NFA state in `state`. `sparses`
/// must have capacity equivalent to `nfa.len()`.
///
/// `match_kind` should correspond to the match semantics implemented by the
/// DFA being built. Generally speaking, for leftmost-first match semantics,
/// states that appear after the first NFA match state will not be included in
/// the `StateBuilderNFA` returned since they are impossible to visit.
///
/// `sparses` is used as scratch space for NFA traversal. Other than their
/// capacity requirements (detailed above), there are no requirements on what's
/// contained within them (if anything). Similarly, what's inside of them once
/// this routine returns is unspecified.
///
/// `stack` must have length 0. It is used as scratch space for depth first
/// traversal. After returning, it is guaranteed that `stack` will have length
/// 0.
///
/// `state` corresponds to the current DFA state on which one wants to compute
/// the transition for the input `unit`.
///
/// `empty_builder` corresponds to the builder allocation to use to produce a
/// complete `StateBuilderNFA` state. If the state is not needed (or is already
/// cached), then it can be cleared and reused without needing to create a new
/// `State`. The `StateBuilderNFA` state returned is final and ready to be
/// turned into a `State` if necessary.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>next(
    nfa: <span class="kw-2">&amp;</span>thompson::NFA,
    match_kind: MatchKind,
    sparses: <span class="kw-2">&amp;mut </span>SparseSets,
    stack: <span class="kw-2">&amp;mut </span>Vec&lt;StateID&gt;,
    state: <span class="kw-2">&amp;</span>State,
    unit: alphabet::Unit,
    empty_builder: StateBuilderEmpty,
) -&gt; StateBuilderNFA {
    sparses.clear();

    <span class="comment">// Whether the NFA is matched in reverse or not. We use this in some
    // conditional logic for dealing with the exceptionally annoying CRLF-aware
    // line anchors.
    </span><span class="kw">let </span>rev = nfa.is_reverse();
    <span class="comment">// The look-around matcher that our NFA is configured with. We don't
    // actually use it to match look-around assertions, but we do need its
    // configuration for constructing states consistent with how it matches.
    </span><span class="kw">let </span>lookm = nfa.look_matcher();

    <span class="comment">// Put the NFA state IDs into a sparse set in case we need to
    // re-compute their epsilon closure.
    //
    // Doing this state shuffling is technically not necessary unless some
    // kind of look-around is used in the DFA. Some ad hoc experiments
    // suggested that avoiding this didn't lead to much of an improvement,
    // but perhaps more rigorous experimentation should be done. And in
    // particular, avoiding this check requires some light refactoring of
    // the code below.
    </span>state.iter_nfa_state_ids(|nfa_id| {
        sparses.set1.insert(nfa_id);
    });

    <span class="comment">// Compute look-ahead assertions originating from the current state. Based
    // on the input unit we're transitioning over, some additional set of
    // assertions may be true. Thus, we re-compute this state's epsilon closure
    // (but only if necessary). Notably, when we build a DFA state initially,
    // we don't enable any look-ahead assertions because we don't know whether
    // they're true or not at that point.
    </span><span class="kw">if </span>!state.look_need().is_empty() {
        <span class="comment">// Add look-ahead assertions that are now true based on the current
        // input unit.
        </span><span class="kw">let </span><span class="kw-2">mut </span>look_have = state.look_have().clone();
        <span class="kw">match </span>unit.as_u8() {
            <span class="prelude-val">Some</span>(<span class="string">b'\r'</span>) =&gt; {
                <span class="kw">if </span>!rev || !state.is_half_crlf() {
                    look_have = look_have.insert(Look::EndCRLF);
                }
            }
            <span class="prelude-val">Some</span>(<span class="string">b'\n'</span>) =&gt; {
                <span class="kw">if </span>rev || !state.is_half_crlf() {
                    look_have = look_have.insert(Look::EndCRLF);
                }
            }
            <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; {}
            <span class="prelude-val">None </span>=&gt; {
                look_have = look_have
                    .insert(Look::End)
                    .insert(Look::EndLF)
                    .insert(Look::EndCRLF);
            }
        }
        <span class="kw">if </span>unit.is_byte(lookm.get_line_terminator()) {
            look_have = look_have.insert(Look::EndLF);
        }
        <span class="kw">if </span>state.is_half_crlf()
            &amp;&amp; ((rev &amp;&amp; !unit.is_byte(<span class="string">b'\r'</span>))
                || (!rev &amp;&amp; !unit.is_byte(<span class="string">b'\n'</span>)))
        {
            look_have = look_have.insert(Look::StartCRLF);
        }
        <span class="kw">if </span>state.is_from_word() == unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordAsciiNegate)
                .insert(Look::WordUnicodeNegate);
        } <span class="kw">else </span>{
            look_have =
                look_have.insert(Look::WordAscii).insert(Look::WordUnicode);
        }
        <span class="kw">if </span>!unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordEndHalfAscii)
                .insert(Look::WordEndHalfUnicode);
        }
        <span class="kw">if </span>state.is_from_word() &amp;&amp; !unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordEndAscii)
                .insert(Look::WordEndUnicode);
        } <span class="kw">else if </span>!state.is_from_word() &amp;&amp; unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordStartAscii)
                .insert(Look::WordStartUnicode);
        }
        <span class="comment">// If we have new assertions satisfied that are among the set of
        // assertions that exist in this state (that is, just because we added
        // an EndLF assertion above doesn't mean there is an EndLF conditional
        // epsilon transition in this state), then we re-compute this state's
        // epsilon closure using the updated set of assertions.
        //
        // Note that since our DFA states omit unconditional epsilon
        // transitions, this check is necessary for correctness. If we re-did
        // the epsilon closure below needlessly, it could change based on the
        // fact that we omitted epsilon states originally.
        </span><span class="kw">if </span>!look_have
            .subtract(state.look_have())
            .intersect(state.look_need())
            .is_empty()
        {
            <span class="kw">for </span>nfa_id <span class="kw">in </span>sparses.set1.iter() {
                epsilon_closure(
                    nfa,
                    nfa_id,
                    look_have,
                    stack,
                    <span class="kw-2">&amp;mut </span>sparses.set2,
                );
            }
            sparses.swap();
            sparses.set2.clear();
        }
    }

    <span class="comment">// Convert our empty builder into one that can record assertions and match
    // pattern IDs.
    </span><span class="kw">let </span><span class="kw-2">mut </span>builder = empty_builder.into_matches();
    <span class="comment">// Set whether the StartLF look-behind assertion is true for this
    // transition or not. The look-behind assertion for ASCII word boundaries
    // is handled below.
    </span><span class="kw">if </span>nfa.look_set_any().contains_anchor_line()
        &amp;&amp; unit.is_byte(lookm.get_line_terminator())
    {
        <span class="comment">// Why only handle StartLF here and not Start? That's because Start
        // can only impact the starting state, which is special cased in
        // start state handling.
        </span>builder.set_look_have(|have| have.insert(Look::StartLF));
    }
    <span class="comment">// We also need to add StartCRLF to our assertions too, if we can. This
    // is unfortunately a bit more complicated, because it depends on the
    // direction of the search. In the forward direction, ^ matches after a
    // \n, but in the reverse direction, ^ only matches after a \r. (This is
    // further complicated by the fact that reverse a regex means changing a ^
    // to a $ and vice versa.)
    </span><span class="kw">if </span>nfa.look_set_any().contains_anchor_crlf()
        &amp;&amp; ((rev &amp;&amp; unit.is_byte(<span class="string">b'\r'</span>)) || (!rev &amp;&amp; unit.is_byte(<span class="string">b'\n'</span>)))
    {
        builder.set_look_have(|have| have.insert(Look::StartCRLF));
    }
    <span class="comment">// And also for the start-half word boundary assertions. As long as the
    // look-behind byte is not a word char, then the assertions are satisfied.
    </span><span class="kw">if </span>nfa.look_set_any().contains_word() &amp;&amp; !unit.is_word_byte() {
        builder.set_look_have(|have| {
            have.insert(Look::WordStartHalfAscii)
                .insert(Look::WordStartHalfUnicode)
        });
    }
    <span class="kw">for </span>nfa_id <span class="kw">in </span>sparses.set1.iter() {
        <span class="kw">match </span><span class="kw-2">*</span>nfa.state(nfa_id) {
            thompson::State::Union { .. }
            | thompson::State::BinaryUnion { .. }
            | thompson::State::Fail
            | thompson::State::Look { .. }
            | thompson::State::Capture { .. } =&gt; {}
            thompson::State::Match { pattern_id } =&gt; {
                <span class="comment">// Notice here that we are calling the NEW state a match
                // state if the OLD state we are transitioning from
                // contains an NFA match state. This is precisely how we
                // delay all matches by one byte and also what therefore
                // guarantees that starting states cannot be match states.
                //
                // If we didn't delay matches by one byte, then whether
                // a DFA is a matching state or not would be determined
                // by whether one of its own constituent NFA states
                // was a match state. (And that would be done in
                // 'add_nfa_states'.)
                //
                // Also, 'add_match_pattern_id' requires that callers never
                // pass duplicative pattern IDs. We do in fact uphold that
                // guarantee here, but it's subtle. In particular, a Thompson
                // NFA guarantees that each pattern has exactly one match
                // state. Moreover, since we're iterating over the NFA state
                // IDs in a set, we are guarateed not to have any duplicative
                // match states. Thus, it is impossible to add the same pattern
                // ID more than once.
                //
                // N.B. We delay matches by 1 byte as a way to hack 1-byte
                // look-around into DFA searches. This lets us support ^, $
                // and ASCII-only \b. The delay is also why we need a special
                // "end-of-input" (EOI) sentinel and why we need to follow the
                // EOI sentinel at the end of every search. This final EOI
                // transition is necessary to report matches found at the end
                // of a haystack.
                </span>builder.add_match_pattern_id(pattern_id);
                <span class="kw">if </span>!match_kind.continue_past_first_match() {
                    <span class="kw">break</span>;
                }
            }
            thompson::State::ByteRange { <span class="kw-2">ref </span>trans } =&gt; {
                <span class="kw">if </span>trans.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        trans.next,
                        builder.look_have(),
                        stack,
                        <span class="kw-2">&amp;mut </span>sparses.set2,
                    );
                }
            }
            thompson::State::Sparse(<span class="kw-2">ref </span>sparse) =&gt; {
                <span class="kw">if let </span><span class="prelude-val">Some</span>(next) = sparse.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        next,
                        builder.look_have(),
                        stack,
                        <span class="kw-2">&amp;mut </span>sparses.set2,
                    );
                }
            }
            thompson::State::Dense(<span class="kw-2">ref </span>dense) =&gt; {
                <span class="kw">if let </span><span class="prelude-val">Some</span>(next) = dense.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        next,
                        builder.look_have(),
                        stack,
                        <span class="kw-2">&amp;mut </span>sparses.set2,
                    );
                }
            }
        }
    }
    <span class="comment">// We only set the word byte if there's a word boundary look-around
    // anywhere in this regex. Otherwise, there's no point in bloating the
    // number of states if we don't have one.
    //
    // We also only set it when the state has a non-zero number of NFA states.
    // Otherwise, we could wind up with states that *should* be DEAD states
    // but are otherwise distinct from DEAD states because of this look-behind
    // assertion being set. While this can't technically impact correctness *in
    // theory*, it can create pathological DFAs that consume input until EOI or
    // a quit byte is seen. Consuming until EOI isn't a correctness problem,
    // but a (serious) perf problem. Hitting a quit byte, however, could be a
    // correctness problem since it could cause search routines to report an
    // error instead of a detected match once the quit state is entered. (The
    // search routine could be made to be a bit smarter by reporting a match
    // if one was detected once it enters a quit state (and indeed, the search
    // routines in this crate do just that), but it seems better to prevent
    // these things by construction if possible.)
    </span><span class="kw">if </span>!sparses.set2.is_empty() {
        <span class="kw">if </span>nfa.look_set_any().contains_word() &amp;&amp; unit.is_word_byte() {
            builder.set_is_from_word();
        }
        <span class="kw">if </span>nfa.look_set_any().contains_anchor_crlf()
            &amp;&amp; ((rev &amp;&amp; unit.is_byte(<span class="string">b'\n'</span>)) || (!rev &amp;&amp; unit.is_byte(<span class="string">b'\r'</span>)))
        {
            builder.set_is_half_crlf();
        }
    }
    <span class="kw">let </span><span class="kw-2">mut </span>builder_nfa = builder.into_nfa();
    add_nfa_states(nfa, <span class="kw-2">&amp;</span>sparses.set2, <span class="kw-2">&amp;mut </span>builder_nfa);
    builder_nfa
}

<span class="doccomment">/// Compute the epsilon closure for the given NFA state. The epsilon closure
/// consists of all NFA state IDs, including `start_nfa_id`, that can be
/// reached from `start_nfa_id` without consuming any input. These state IDs
/// are written to `set` in the order they are visited, but only if they are
/// not already in `set`. `start_nfa_id` must be a valid state ID for the NFA
/// given.
///
/// `look_have` consists of the satisfied assertions at the current
/// position. For conditional look-around epsilon transitions, these are
/// only followed if they are satisfied by `look_have`.
///
/// `stack` must have length 0. It is used as scratch space for depth first
/// traversal. After returning, it is guaranteed that `stack` will have length
/// 0.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>epsilon_closure(
    nfa: <span class="kw-2">&amp;</span>thompson::NFA,
    start_nfa_id: StateID,
    look_have: LookSet,
    stack: <span class="kw-2">&amp;mut </span>Vec&lt;StateID&gt;,
    set: <span class="kw-2">&amp;mut </span>SparseSet,
) {
    <span class="macro">assert!</span>(stack.is_empty());
    <span class="comment">// If this isn't an epsilon state, then the epsilon closure is always just
    // itself, so there's no need to spin up the machinery below to handle it.
    </span><span class="kw">if </span>!nfa.state(start_nfa_id).is_epsilon() {
        set.insert(start_nfa_id);
        <span class="kw">return</span>;
    }

    stack.push(start_nfa_id);
    <span class="kw">while let </span><span class="prelude-val">Some</span>(<span class="kw-2">mut </span>id) = stack.pop() {
        <span class="comment">// In many cases, we can avoid stack operations when an NFA state only
        // adds one new state to visit. In that case, we just set our ID to
        // that state and mush on. We only use the stack when an NFA state
        // introduces multiple new states to visit.
        </span><span class="kw">loop </span>{
            <span class="comment">// Insert this NFA state, and if it's already in the set and thus
            // already visited, then we can move on to the next one.
            </span><span class="kw">if </span>!set.insert(id) {
                <span class="kw">break</span>;
            }
            <span class="kw">match </span><span class="kw-2">*</span>nfa.state(id) {
                thompson::State::ByteRange { .. }
                | thompson::State::Sparse { .. }
                | thompson::State::Dense { .. }
                | thompson::State::Fail
                | thompson::State::Match { .. } =&gt; <span class="kw">break</span>,
                thompson::State::Look { look, next } =&gt; {
                    <span class="kw">if </span>!look_have.contains(look) {
                        <span class="kw">break</span>;
                    }
                    id = next;
                }
                thompson::State::Union { <span class="kw-2">ref </span>alternates } =&gt; {
                    id = <span class="kw">match </span>alternates.get(<span class="number">0</span>) {
                        <span class="prelude-val">None </span>=&gt; <span class="kw">break</span>,
                        <span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span>id) =&gt; id,
                    };
                    <span class="comment">// We need to process our alternates in order to preserve
                    // match preferences, so put the earliest alternates closer
                    // to the top of the stack.
                    </span>stack.extend(alternates[<span class="number">1</span>..].iter().rev());
                }
                thompson::State::BinaryUnion { alt1, alt2 } =&gt; {
                    id = alt1;
                    stack.push(alt2);
                }
                thompson::State::Capture { next, .. } =&gt; {
                    id = next;
                }
            }
        }
    }
}

<span class="doccomment">/// Add the NFA state IDs in the given `set` to the given DFA builder state.
/// The order in which states are added corresponds to the order in which they
/// were added to `set`.
///
/// The DFA builder state given should already have its complete set of match
/// pattern IDs added (if any) and any look-behind assertions (StartLF, Start
/// and whether this state is being generated for a transition over a word byte
/// when applicable) that are true immediately prior to transitioning into this
/// state (via `builder.look_have()`). The match pattern IDs should correspond
/// to matches that occurred on the previous transition, since all matches are
/// delayed by one byte. The things that should _not_ be set are look-ahead
/// assertions (EndLF, End and whether the next byte is a word byte or not).
/// The builder state should also not have anything in `look_need` set, as this
/// routine will compute that for you.
///
/// The given NFA should be able to resolve all identifiers in `set` to a
/// particular NFA state. Additionally, `set` must have capacity equivalent
/// to `nfa.len()`.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>add_nfa_states(
    nfa: <span class="kw-2">&amp;</span>thompson::NFA,
    set: <span class="kw-2">&amp;</span>SparseSet,
    builder: <span class="kw-2">&amp;mut </span>StateBuilderNFA,
) {
    <span class="kw">for </span>nfa_id <span class="kw">in </span>set.iter() {
        <span class="kw">match </span><span class="kw-2">*</span>nfa.state(nfa_id) {
            thompson::State::ByteRange { .. } =&gt; {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Sparse { .. } =&gt; {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Dense { .. } =&gt; {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Look { look, .. } =&gt; {
                builder.add_nfa_state_id(nfa_id);
                builder.set_look_need(|need| need.insert(look));
            }
            thompson::State::Union { .. }
            | thompson::State::BinaryUnion { .. } =&gt; {
                <span class="comment">// Pure epsilon transitions don't need to be tracked as part
                // of the DFA state. Tracking them is actually superfluous;
                // they won't cause any harm other than making determinization
                // slower.
                //
                // Why aren't these needed? Well, in an NFA, epsilon
                // transitions are really just jumping points to other states.
                // So once you hit an epsilon transition, the same set of
                // resulting states always appears. Therefore, putting them in
                // a DFA's set of ordered NFA states is strictly redundant.
                //
                // Look-around states are also epsilon transitions, but
                // they are *conditional*. So their presence could be
                // discriminatory, and thus, they are tracked above.
                //
                // But wait... why are epsilon states in our `set` in the first
                // place? Why not just leave them out? They're in our `set`
                // because it was generated by computing an epsilon closure,
                // and we want to keep track of all states we visited to avoid
                // re-visiting them. In exchange, we have to do this second
                // iteration over our collected states to finalize our DFA
                // state. In theory, we could avoid this second iteration if
                // we maintained two sets during epsilon closure: the set of
                // visited states (to avoid cycles) and the set of states that
                // will actually be used to construct the next DFA state.
                //
                // Note that this optimization requires that we re-compute the
                // epsilon closure to account for look-ahead in 'next' *only
                // when necessary*. Namely, only when the set of look-around
                // assertions changes and only when those changes are within
                // the set of assertions that are needed in order to step
                // through the closure correctly. Otherwise, if we re-do the
                // epsilon closure needlessly, it could change based on the
                // fact that we are omitting epsilon states here.
                //
                // -----
                //
                // Welp, scratch the above. It turns out that recording these
                // is in fact necessary to seemingly handle one particularly
                // annoying case: when a conditional epsilon transition is
                // put inside of a repetition operator. One specific case I
                // ran into was the regex `(?:\b|%)+` on the haystack `z%`.
                // The correct leftmost first matches are: [0, 0] and [1, 1].
                // But the DFA was reporting [0, 0] and [1, 2]. To understand
                // why this happens, consider the NFA for the aforementioned
                // regex:
                //
                //     &gt;000000: binary-union(4, 1)
                //      000001: \x00-\xFF =&gt; 0
                //      000002: WordAscii =&gt; 5
                //      000003: % =&gt; 5
                //     ^000004: binary-union(2, 3)
                //      000005: binary-union(4, 6)
                //      000006: MATCH(0)
                //
                // The problem here is that one of the DFA start states is
                // going to consist of the NFA states [2, 3] by computing the
                // epsilon closure of state 4. State 4 isn't included because
                // we previously were not keeping track of union states. But
                // only a subset of transitions out of this state will be able
                // to follow WordAscii, and in those cases, the epsilon closure
                // is redone. The only problem is that computing the epsilon
                // closure from [2, 3] is different than computing the epsilon
                // closure from [4]. In the former case, assuming the WordAscii
                // assertion is satisfied, you get: [2, 3, 6]. In the latter
                // case, you get: [2, 6, 3]. Notice that '6' is the match state
                // and appears AFTER '3' in the former case. This leads to a
                // preferential but incorrect match of '%' before returning
                // a match. In the latter case, the match is preferred over
                // continuing to accept the '%'.
                //
                // It almost feels like we might be able to fix the NFA states
                // to avoid this, or to at least only keep track of union
                // states where this actually matters, since in the vast
                // majority of cases, this doesn't matter.
                //
                // Another alternative would be to define a new HIR property
                // called "assertion is repeated anywhere" and compute it
                // inductively over the entire pattern. If it happens anywhere,
                // which is probably pretty rare, then we record union states.
                // Otherwise we don't.
                </span>builder.add_nfa_state_id(nfa_id);
            }
            <span class="comment">// Capture states we definitely do not need to record, since they
            // are unconditional epsilon transitions with no branching.
            </span>thompson::State::Capture { .. } =&gt; {}
            <span class="comment">// It's not totally clear whether we need to record fail states or
            // not, but we do so out of an abundance of caution. Since they are
            // quite rare in practice, there isn't much cost to recording them.
            </span>thompson::State::Fail =&gt; {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Match { .. } =&gt; {
                <span class="comment">// Normally, the NFA match state doesn't actually need to
                // be inside the DFA state. But since we delay matches by
                // one byte, the matching DFA state corresponds to states
                // that transition from the one we're building here. And
                // the way we detect those cases is by looking for an NFA
                // match state. See 'next' for how this is handled.
                </span>builder.add_nfa_state_id(nfa_id);
            }
        }
    }
    <span class="comment">// If we know this state contains no look-around assertions, then
    // there's no reason to track which look-around assertions were
    // satisfied when this state was created.
    </span><span class="kw">if </span>builder.look_need().is_empty() {
        builder.set_look_have(|<span class="kw">_</span>| LookSet::empty());
    }
}

<span class="doccomment">/// Sets the appropriate look-behind assertions on the given state based on
/// this starting configuration.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>set_lookbehind_from_start(
    nfa: <span class="kw-2">&amp;</span>thompson::NFA,
    start: <span class="kw-2">&amp;</span>Start,
    builder: <span class="kw-2">&amp;mut </span>StateBuilderMatches,
) {
    <span class="kw">let </span>rev = nfa.is_reverse();
    <span class="kw">let </span>lineterm = nfa.look_matcher().get_line_terminator();
    <span class="kw">let </span>lookset = nfa.look_set_any();
    <span class="kw">match </span><span class="kw-2">*</span>start {
        Start::NonWordByte =&gt; {
            <span class="kw">if </span>lookset.contains_word() {
                builder.set_look_have(|have| {
                    have.insert(Look::WordStartHalfAscii)
                        .insert(Look::WordStartHalfUnicode)
                });
            }
        }
        Start::WordByte =&gt; {
            <span class="kw">if </span>lookset.contains_word() {
                builder.set_is_from_word();
            }
        }
        Start::Text =&gt; {
            <span class="kw">if </span>lookset.contains_anchor_haystack() {
                builder.set_look_have(|have| have.insert(Look::Start));
            }
            <span class="kw">if </span>lookset.contains_anchor_line() {
                builder.set_look_have(|have| {
                    have.insert(Look::StartLF).insert(Look::StartCRLF)
                });
            }
            <span class="kw">if </span>lookset.contains_word() {
                builder.set_look_have(|have| {
                    have.insert(Look::WordStartHalfAscii)
                        .insert(Look::WordStartHalfUnicode)
                });
            }
        }
        Start::LineLF =&gt; {
            <span class="kw">if </span>rev {
                <span class="kw">if </span>lookset.contains_anchor_crlf() {
                    builder.set_is_half_crlf();
                }
                <span class="kw">if </span>lookset.contains_anchor_line() {
                    builder.set_look_have(|have| have.insert(Look::StartLF));
                }
            } <span class="kw">else </span>{
                <span class="kw">if </span>lookset.contains_anchor_line() {
                    builder.set_look_have(|have| have.insert(Look::StartCRLF));
                }
            }
            <span class="kw">if </span>lookset.contains_anchor_line() &amp;&amp; lineterm == <span class="string">b'\n' </span>{
                builder.set_look_have(|have| have.insert(Look::StartLF));
            }
            <span class="kw">if </span>lookset.contains_word() {
                builder.set_look_have(|have| {
                    have.insert(Look::WordStartHalfAscii)
                        .insert(Look::WordStartHalfUnicode)
                });
            }
        }
        Start::LineCR =&gt; {
            <span class="kw">if </span>lookset.contains_anchor_crlf() {
                <span class="kw">if </span>rev {
                    builder.set_look_have(|have| have.insert(Look::StartCRLF));
                } <span class="kw">else </span>{
                    builder.set_is_half_crlf();
                }
            }
            <span class="kw">if </span>lookset.contains_anchor_line() &amp;&amp; lineterm == <span class="string">b'\r' </span>{
                builder.set_look_have(|have| have.insert(Look::StartLF));
            }
            <span class="kw">if </span>lookset.contains_word() {
                builder.set_look_have(|have| {
                    have.insert(Look::WordStartHalfAscii)
                        .insert(Look::WordStartHalfUnicode)
                });
            }
        }
        Start::CustomLineTerminator =&gt; {
            <span class="kw">if </span>lookset.contains_anchor_line() {
                builder.set_look_have(|have| have.insert(Look::StartLF));
            }
            <span class="comment">// This is a bit of a tricky case, but if the line terminator was
            // set to a word byte, then we also need to behave as if the start
            // configuration is Start::WordByte. That is, we need to mark our
            // state as having come from a word byte.
            </span><span class="kw">if </span>lookset.contains_word() {
                <span class="kw">if </span>utf8::is_word_byte(lineterm) {
                    builder.set_is_from_word();
                } <span class="kw">else </span>{
                    builder.set_look_have(|have| {
                        have.insert(Look::WordStartHalfAscii)
                            .insert(Look::WordStartHalfUnicode)
                    });
                }
            }
        }
    }
}
</code></pre></div></section></main></body></html>