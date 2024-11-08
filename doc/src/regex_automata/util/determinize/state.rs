<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.6/src/util/determinize/state.rs`."><title>state.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../" data-static-root-path="../../../../static.files/" data-current-crate="regex_automata" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../src-files.js"></script><script defer src="../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../regex_automata/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">/*!
This module defines a DFA state representation and builders for constructing
DFA states.

This representation is specifically for use in implementations of NFA-to-DFA
conversion via powerset construction. (Also called "determinization" in this
crate.)

The term "DFA state" is somewhat overloaded in this crate. In some cases, it
refers to the set of transitions over an alphabet for a particular state. In
other cases, it refers to a set of NFA states. The former is really about the
final representation of a state in a DFA's transition table, where as the
latter---what this module is focused on---is closer to an intermediate form
that is used to help eventually build the transition table.

This module exports four types. All four types represent the same idea: an
ordered set of NFA states. This ordered set represents the epsilon closure of a
particular NFA state, where the "epsilon closure" is the set of NFA states that
can be transitioned to without consuming any input. i.e., Follow all of the NFA
state's epsilon transitions. In addition, this implementation of DFA states
cares about two other things: the ordered set of pattern IDs corresponding
to the patterns that match if the state is a match state, and the set of
look-behind assertions that were true when the state was created.

The first, `State`, is a frozen representation of a state that cannot be
modified. It may be cheaply cloned without copying the state itself and can be
accessed safely from multiple threads simultaneously. This type is useful for
when one knows that the DFA state being constructed is distinct from any other
previously constructed states. Namely, powerset construction, in practice,
requires one to keep a cache of previously created DFA states. Otherwise,
the number of DFA states created in memory balloons to an impractically
large number. For this reason, equivalent states should endeavor to have an
equivalent byte-level representation. (In general, "equivalency" here means,
"equivalent assertions, pattern IDs and NFA state IDs." We do not require that
full DFA minimization be implemented here. This form of equivalency is only
surface deep and is more-or-less a practical necessity.)

The other three types represent different phases in the construction of a
DFA state. Internally, these three types (and `State`) all use the same
byte-oriented representation. That means one can use any of the builder types
to check whether the state it represents already exists or not. If it does,
then there is no need to freeze it into a `State` (which requires an alloc and
a copy). Here are the three types described succinctly:

* `StateBuilderEmpty` represents a state with no pattern IDs, no assertions
and no NFA states. Creating a `StateBuilderEmpty` performs no allocs. A
`StateBuilderEmpty` can only be used to query its underlying memory capacity,
or to convert into a builder for recording pattern IDs and/or assertions.

* `StateBuilderMatches` represents a state with zero or more pattern IDs, zero
or more satisfied assertions and zero NFA state IDs. A `StateBuilderMatches`
can only be used for adding pattern IDs and recording assertions.

* `StateBuilderNFA` represents a state with zero or more pattern IDs, zero or
more satisfied assertions and zero or more NFA state IDs. A `StateBuilderNFA`
can only be used for adding NFA state IDs and recording some assertions.

The expected flow here is to use the above builders to construct a candidate
DFA state to check if it already exists. If it does, then there's no need to
freeze it into a `State`. It it doesn't exist, then `StateBuilderNFA::to_state`
can be called to freeze the builder into an immutable `State`. In either
case, `clear` should be called on the builder to turn it back into a
`StateBuilderEmpty` that reuses the underlying memory.

The main purpose for splitting the builder into these distinct types is to
make it impossible to do things like adding a pattern ID after adding an NFA
state ID. Namely, this makes it simpler to use a space-and-time efficient
binary representation for the state. (The format is documented on the `Repr`
type below.) If we just used one type for everything, it would be possible for
callers to use an incorrect interleaving of calls and thus result in a corrupt
representation. I chose to use more type machinery to make this impossible to
do because 1) determinization is itself pretty complex and it wouldn't be too
hard to foul this up and 2) there isn't too much machinery involved and it's
well contained.

As an optimization, sometimes states won't have certain things set. For
example, if the underlying NFA has no word boundary assertions, then there is
no reason to set a state's look-behind assertion as to whether it was generated
from a word byte or not. Similarly, if a state has no NFA states corresponding
to look-around assertions, then there is no reason to set `look_have` to a
non-empty set. Finally, callers usually omit unconditional epsilon transitions
when adding NFA state IDs since they aren't discriminatory.

Finally, the binary representation used by these states is, thankfully, not
serialized anywhere. So any kind of change can be made with reckless abandon,
as long as everything in this module agrees.
*/

</span><span class="kw">use </span>core::mem;

<span class="kw">use </span>alloc::{sync::Arc, vec::Vec};

<span class="kw">use </span><span class="kw">crate</span>::util::{
    int::{I32, U32},
    look::LookSet,
    primitives::{PatternID, StateID},
    wire::{<span class="self">self</span>, Endian},
};

<span class="doccomment">/// A DFA state that, at its core, is represented by an ordered set of NFA
/// states.
///
/// This type is intended to be used only in NFA-to-DFA conversion via powerset
/// construction.
///
/// It may be cheaply cloned and accessed safely from multiple threads
/// simultaneously.
</span><span class="attr">#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>State(Arc&lt;[u8]&gt;);

<span class="doccomment">/// This Borrow impl permits us to lookup any state in a map by its byte
/// representation. This is particularly convenient when one has a StateBuilder
/// and we want to see if a correspondingly equivalent state already exists. If
/// one does exist, then we can reuse the allocation required by StateBuilder
/// without having to convert it into a State first.
</span><span class="kw">impl </span>core::borrow::Borrow&lt;[u8]&gt; <span class="kw">for </span>State {
    <span class="kw">fn </span>borrow(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>[u8] {
        <span class="kw-2">&amp;*</span><span class="self">self</span>.<span class="number">0
    </span>}
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>State {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        f.debug_tuple(<span class="string">"State"</span>).field(<span class="kw-2">&amp;</span><span class="self">self</span>.repr()).finish()
    }
}

<span class="doccomment">/// For docs on these routines, see the internal Repr and ReprVec types below.
</span><span class="kw">impl </span>State {
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>dead() -&gt; State {
        StateBuilderEmpty::new().into_matches().into_nfa().to_state()
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_match(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.repr().is_match()
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_from_word(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.repr().is_from_word()
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_half_crlf(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.repr().is_half_crlf()
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>look_have(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LookSet {
        <span class="self">self</span>.repr().look_have()
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>look_need(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LookSet {
        <span class="self">self</span>.repr().look_need()
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>match_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.repr().match_len()
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>match_pattern(<span class="kw-2">&amp;</span><span class="self">self</span>, index: usize) -&gt; PatternID {
        <span class="self">self</span>.repr().match_pattern(index)
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>match_pattern_ids(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Vec&lt;PatternID&gt;&gt; {
        <span class="self">self</span>.repr().match_pattern_ids()
    }

    <span class="attr">#[cfg(all(test, not(miri)))]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>iter_match_pattern_ids&lt;F: FnMut(PatternID)&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, f: F) {
        <span class="self">self</span>.repr().iter_match_pattern_ids(f)
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>iter_nfa_state_ids&lt;F: FnMut(StateID)&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, f: F) {
        <span class="self">self</span>.repr().iter_nfa_state_ids(f)
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>memory_usage(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.<span class="number">0</span>.len()
    }

    <span class="kw">fn </span>repr(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Repr&lt;<span class="lifetime">'_</span>&gt; {
        Repr(<span class="kw-2">&amp;*</span><span class="self">self</span>.<span class="number">0</span>)
    }
}

<span class="doccomment">/// A state builder that represents an empty state.
///
/// This is a useful "initial condition" for state construction. It has no
/// NFA state IDs, no assertions set and no pattern IDs. No allocations are
/// made when new() is called. Its main use is for being converted into a
/// builder that can capture assertions and pattern IDs.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>StateBuilderEmpty(Vec&lt;u8&gt;);

<span class="doccomment">/// For docs on these routines, see the internal Repr and ReprVec types below.
</span><span class="kw">impl </span>StateBuilderEmpty {
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>new() -&gt; StateBuilderEmpty {
        StateBuilderEmpty(<span class="macro">alloc::vec!</span>[])
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>into_matches(<span class="kw-2">mut </span><span class="self">self</span>) -&gt; StateBuilderMatches {
        <span class="self">self</span>.<span class="number">0</span>.extend_from_slice(<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>]);
        StateBuilderMatches(<span class="self">self</span>.<span class="number">0</span>)
    }

    <span class="kw">fn </span>clear(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.<span class="number">0</span>.clear();
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>capacity(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.<span class="number">0</span>.capacity()
    }
}

<span class="doccomment">/// A state builder that collects assertions and pattern IDs.
///
/// When collecting pattern IDs is finished, this can be converted into a
/// builder that collects NFA state IDs.
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>StateBuilderMatches(Vec&lt;u8&gt;);

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>StateBuilderMatches {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        f.debug_tuple(<span class="string">"StateBuilderMatches"</span>).field(<span class="kw-2">&amp;</span><span class="self">self</span>.repr()).finish()
    }
}

<span class="doccomment">/// For docs on these routines, see the internal Repr and ReprVec types below.
</span><span class="kw">impl </span>StateBuilderMatches {
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>into_nfa(<span class="kw-2">mut </span><span class="self">self</span>) -&gt; StateBuilderNFA {
        <span class="self">self</span>.repr_vec().close_match_pattern_ids();
        StateBuilderNFA { repr: <span class="self">self</span>.<span class="number">0</span>, prev_nfa_state_id: StateID::ZERO }
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>set_is_from_word(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.repr_vec().set_is_from_word()
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>set_is_half_crlf(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.repr_vec().set_is_half_crlf()
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>look_have(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LookSet {
        LookSet::read_repr(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>[<span class="number">1</span>..])
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>set_look_have(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        set: <span class="kw">impl </span>FnMut(LookSet) -&gt; LookSet,
    ) {
        <span class="self">self</span>.repr_vec().set_look_have(set)
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>add_match_pattern_id(<span class="kw-2">&amp;mut </span><span class="self">self</span>, pid: PatternID) {
        <span class="self">self</span>.repr_vec().add_match_pattern_id(pid)
    }

    <span class="kw">fn </span>repr(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Repr&lt;<span class="lifetime">'_</span>&gt; {
        Repr(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>)
    }

    <span class="kw">fn </span>repr_vec(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; ReprVec&lt;<span class="lifetime">'_</span>&gt; {
        ReprVec(<span class="kw-2">&amp;mut </span><span class="self">self</span>.<span class="number">0</span>)
    }
}

<span class="doccomment">/// A state builder that collects some assertions and NFA state IDs.
///
/// When collecting NFA state IDs is finished, this can be used to build a
/// `State` if necessary.
///
/// When dont with building a state (regardless of whether it got kept or not),
/// it's usually a good idea to call `clear` to get an empty builder back so
/// that it can be reused to build the next state.
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>StateBuilderNFA {
    repr: Vec&lt;u8&gt;,
    prev_nfa_state_id: StateID,
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>StateBuilderNFA {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        f.debug_tuple(<span class="string">"StateBuilderNFA"</span>).field(<span class="kw-2">&amp;</span><span class="self">self</span>.repr()).finish()
    }
}

<span class="doccomment">/// For docs on these routines, see the internal Repr and ReprVec types below.
</span><span class="kw">impl </span>StateBuilderNFA {
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>to_state(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; State {
        State(Arc::from(<span class="kw-2">&amp;*</span><span class="self">self</span>.repr))
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>clear(<span class="self">self</span>) -&gt; StateBuilderEmpty {
        <span class="kw">let </span><span class="kw-2">mut </span>builder = StateBuilderEmpty(<span class="self">self</span>.repr);
        builder.clear();
        builder
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>look_need(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LookSet {
        <span class="self">self</span>.repr().look_need()
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>set_look_have(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        set: <span class="kw">impl </span>FnMut(LookSet) -&gt; LookSet,
    ) {
        <span class="self">self</span>.repr_vec().set_look_have(set)
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>set_look_need(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        set: <span class="kw">impl </span>FnMut(LookSet) -&gt; LookSet,
    ) {
        <span class="self">self</span>.repr_vec().set_look_need(set)
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>add_nfa_state_id(<span class="kw-2">&amp;mut </span><span class="self">self</span>, sid: StateID) {
        ReprVec(<span class="kw-2">&amp;mut </span><span class="self">self</span>.repr)
            .add_nfa_state_id(<span class="kw-2">&amp;mut </span><span class="self">self</span>.prev_nfa_state_id, sid)
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>as_bytes(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>[u8] {
        <span class="kw-2">&amp;</span><span class="self">self</span>.repr
    }

    <span class="kw">fn </span>repr(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Repr&lt;<span class="lifetime">'_</span>&gt; {
        Repr(<span class="kw-2">&amp;</span><span class="self">self</span>.repr)
    }

    <span class="kw">fn </span>repr_vec(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; ReprVec&lt;<span class="lifetime">'_</span>&gt; {
        ReprVec(<span class="kw-2">&amp;mut </span><span class="self">self</span>.repr)
    }
}

<span class="doccomment">/// Repr is a read-only view into the representation of a DFA state.
///
/// Primarily, a Repr is how we achieve DRY: we implement decoding the format
/// in one place, and then use a Repr to implement the various methods on the
/// public state types.
///
/// The format is as follows:
///
/// The first three bytes correspond to bitsets.
///
/// Byte 0 is a bitset corresponding to miscellaneous flags associated with the
/// state. Bit 0 is set to 1 if the state is a match state. Bit 1 is set to 1
/// if the state has pattern IDs explicitly written to it. (This is a flag that
/// is not meant to be set by determinization, but rather, is used as part of
/// an internal space-saving optimization.) Bit 2 is set to 1 if the state was
/// generated by a transition over a "word" byte. (Callers may not always set
/// this. For example, if the NFA has no word boundary assertion, then needing
/// to track whether a state came from a word byte or not is superfluous and
/// wasteful.) Bit 3 is set to 1 if the state was generated by a transition
/// from a `\r` (forward search) or a `\n` (reverse search) when CRLF mode is
/// enabled.
///
/// Bytes 1..5 correspond to the look-behind assertions that were satisfied
/// by the transition that created this state. (Look-ahead assertions are not
/// tracked as part of states. Instead, these are applied by re-computing the
/// epsilon closure of a state when computing the transition function. See
/// `next` in the parent module.)
///
/// Bytes 5..9 correspond to the set of look-around assertions (including both
/// look-behind and look-ahead) that appear somewhere in this state's set of
/// NFA state IDs. This is used to determine whether this state's epsilon
/// closure should be re-computed when computing the transition function.
/// Namely, look-around assertions are "just" conditional epsilon transitions,
/// so if there are new assertions available when computing the transition
/// function, we should only re-compute the epsilon closure if those new
/// assertions are relevant to this particular state.
///
/// Bytes 9..13 correspond to a 32-bit native-endian encoded integer
/// corresponding to the number of patterns encoded in this state. If the state
/// is not a match state (byte 0 bit 0 is 0) or if it's only pattern ID is
/// PatternID::ZERO, then no integer is encoded at this position. Instead, byte
/// offset 3 is the position at which the first NFA state ID is encoded.
///
/// For a match state with at least one non-ZERO pattern ID, the next bytes
/// correspond to a sequence of 32-bit native endian encoded integers that
/// represent each pattern ID, in order, that this match state represents.
///
/// After the pattern IDs (if any), NFA state IDs are delta encoded as
/// varints.[1] The first NFA state ID is encoded as itself, and each
/// subsequent NFA state ID is encoded as the difference between itself and the
/// previous NFA state ID.
///
/// [1] - https://developers.google.com/protocol-buffers/docs/encoding#varints
</span><span class="kw">struct </span>Repr&lt;<span class="lifetime">'a</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8]);

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Repr&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// Returns true if and only if this is a match state.
    ///
    /// If callers have added pattern IDs to this state, then callers MUST set
    /// this state as a match state explicitly. However, as a special case,
    /// states that are marked as match states but with no pattern IDs, then
    /// the state is treated as if it had a single pattern ID equivalent to
    /// PatternID::ZERO.
    </span><span class="kw">fn </span>is_match(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.<span class="number">0</span>[<span class="number">0</span>] &amp; (<span class="number">1 </span>&lt;&lt; <span class="number">0</span>) &gt; <span class="number">0
    </span>}

    <span class="doccomment">/// Returns true if and only if this state has had at least one pattern
    /// ID added to it.
    ///
    /// This is an internal-only flag that permits the representation to save
    /// space in the common case of an NFA with one pattern in it. In that
    /// case, a match state can only ever have exactly one pattern ID:
    /// PatternID::ZERO. So there's no need to represent it.
    </span><span class="kw">fn </span>has_pattern_ids(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.<span class="number">0</span>[<span class="number">0</span>] &amp; (<span class="number">1 </span>&lt;&lt; <span class="number">1</span>) &gt; <span class="number">0
    </span>}

    <span class="doccomment">/// Returns true if and only if this state is marked as having been created
    /// from a transition over a word byte. This is useful for checking whether
    /// a word boundary assertion is true or not, which requires look-behind
    /// (whether the current state came from a word byte or not) and look-ahead
    /// (whether the transition byte is a word byte or not).
    ///
    /// Since states with this set are distinct from states that don't have
    /// this set (even if they are otherwise equivalent), callers should not
    /// set this assertion unless the underlying NFA has at least one word
    /// boundary assertion somewhere. Otherwise, a superfluous number of states
    /// may be created.
    </span><span class="kw">fn </span>is_from_word(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.<span class="number">0</span>[<span class="number">0</span>] &amp; (<span class="number">1 </span>&lt;&lt; <span class="number">2</span>) &gt; <span class="number">0
    </span>}

    <span class="doccomment">/// Returns true if and only if this state is marked as being inside of a
    /// CRLF terminator. In the forward direction, this means the state was
    /// created after seeing a `\r`. In the reverse direction, this means the
    /// state was created after seeing a `\n`.
    </span><span class="kw">fn </span>is_half_crlf(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.<span class="number">0</span>[<span class="number">0</span>] &amp; (<span class="number">1 </span>&lt;&lt; <span class="number">3</span>) &gt; <span class="number">0
    </span>}

    <span class="doccomment">/// The set of look-behind assertions that were true in the transition that
    /// created this state.
    ///
    /// Generally, this should be empty if 'look_need' is empty, since there is
    /// no reason to track which look-behind assertions are true if the state
    /// has no conditional epsilon transitions.
    ///
    /// Satisfied look-ahead assertions are not tracked in states. Instead,
    /// these are re-computed on demand via epsilon closure when computing the
    /// transition function.
    </span><span class="kw">fn </span>look_have(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LookSet {
        LookSet::read_repr(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>[<span class="number">1</span>..])
    }

    <span class="doccomment">/// The set of look-around (both behind and ahead) assertions that appear
    /// at least once in this state's set of NFA states.
    ///
    /// This is used to determine whether the epsilon closure needs to be
    /// re-computed when computing the transition function. Namely, if the
    /// state has no conditional epsilon transitions, then there is no need
    /// to re-compute the epsilon closure.
    </span><span class="kw">fn </span>look_need(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LookSet {
        LookSet::read_repr(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>[<span class="number">5</span>..])
    }

    <span class="doccomment">/// Returns the total number of match pattern IDs in this state.
    ///
    /// If this state is not a match state, then this always returns 0.
    </span><span class="kw">fn </span>match_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="kw">if </span>!<span class="self">self</span>.is_match() {
            <span class="kw">return </span><span class="number">0</span>;
        } <span class="kw">else if </span>!<span class="self">self</span>.has_pattern_ids() {
            <span class="number">1
        </span>} <span class="kw">else </span>{
            <span class="self">self</span>.encoded_pattern_len()
        }
    }

    <span class="doccomment">/// Returns the pattern ID for this match state at the given index.
    ///
    /// If the given index is greater than or equal to `match_len()` for this
    /// state, then this could panic or return incorrect results.
    </span><span class="kw">fn </span>match_pattern(<span class="kw-2">&amp;</span><span class="self">self</span>, index: usize) -&gt; PatternID {
        <span class="kw">if </span>!<span class="self">self</span>.has_pattern_ids() {
            PatternID::ZERO
        } <span class="kw">else </span>{
            <span class="kw">let </span>offset = <span class="number">13 </span>+ index * PatternID::SIZE;
            <span class="comment">// This is OK since we only ever serialize valid PatternIDs to
            // states.
            </span>wire::read_pattern_id_unchecked(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>[offset..]).<span class="number">0
        </span>}
    }

    <span class="doccomment">/// Returns a copy of all match pattern IDs in this state. If this state
    /// is not a match state, then this returns None.
    </span><span class="kw">fn </span>match_pattern_ids(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Vec&lt;PatternID&gt;&gt; {
        <span class="kw">if </span>!<span class="self">self</span>.is_match() {
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }
        <span class="kw">let </span><span class="kw-2">mut </span>pids = <span class="macro">alloc::vec!</span>[];
        <span class="self">self</span>.iter_match_pattern_ids(|pid| pids.push(pid));
        <span class="prelude-val">Some</span>(pids)
    }

    <span class="doccomment">/// Calls the given function on every pattern ID in this state.
    </span><span class="kw">fn </span>iter_match_pattern_ids&lt;F: FnMut(PatternID)&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, <span class="kw-2">mut </span>f: F) {
        <span class="kw">if </span>!<span class="self">self</span>.is_match() {
            <span class="kw">return</span>;
        }
        <span class="comment">// As an optimization for a very common case, when this is a match
        // state for an NFA with only one pattern, we don't actually write the
        // pattern ID to the state representation. Instead, we know it must
        // be there since it is the only possible choice.
        </span><span class="kw">if </span>!<span class="self">self</span>.has_pattern_ids() {
            f(PatternID::ZERO);
            <span class="kw">return</span>;
        }
        <span class="kw">let </span><span class="kw-2">mut </span>pids = <span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>[<span class="number">13</span>..<span class="self">self</span>.pattern_offset_end()];
        <span class="kw">while </span>!pids.is_empty() {
            <span class="kw">let </span>pid = wire::read_u32(pids);
            pids = <span class="kw-2">&amp;</span>pids[PatternID::SIZE..];
            <span class="comment">// This is OK since we only ever serialize valid PatternIDs to
            // states. And since pattern IDs can never exceed a usize, the
            // unwrap is OK.
            </span>f(PatternID::new_unchecked(usize::try_from(pid).unwrap()));
        }
    }

    <span class="doccomment">/// Calls the given function on every NFA state ID in this state.
    </span><span class="kw">fn </span>iter_nfa_state_ids&lt;F: FnMut(StateID)&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, <span class="kw-2">mut </span>f: F) {
        <span class="kw">let </span><span class="kw-2">mut </span>sids = <span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>[<span class="self">self</span>.pattern_offset_end()..];
        <span class="kw">let </span><span class="kw-2">mut </span>prev = <span class="number">0i32</span>;
        <span class="kw">while </span>!sids.is_empty() {
            <span class="kw">let </span>(delta, nr) = read_vari32(sids);
            sids = <span class="kw-2">&amp;</span>sids[nr..];
            <span class="kw">let </span>sid = prev + delta;
            prev = sid;
            <span class="comment">// This is OK since we only ever serialize valid StateIDs to
            // states. And since state IDs can never exceed an isize, they must
            // always be able to fit into a usize, and thus cast is OK.
            </span>f(StateID::new_unchecked(sid.as_usize()))
        }
    }

    <span class="doccomment">/// Returns the offset into this state's representation where the pattern
    /// IDs end and the NFA state IDs begin.
    </span><span class="kw">fn </span>pattern_offset_end(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="kw">let </span>encoded = <span class="self">self</span>.encoded_pattern_len();
        <span class="kw">if </span>encoded == <span class="number">0 </span>{
            <span class="kw">return </span><span class="number">9</span>;
        }
        <span class="comment">// This arithmetic is OK since we were able to address this many bytes
        // when writing to the state, thus, it must fit into a usize.
        </span>encoded.checked_mul(<span class="number">4</span>).unwrap().checked_add(<span class="number">13</span>).unwrap()
    }

    <span class="doccomment">/// Returns the total number of *encoded* pattern IDs in this state.
    ///
    /// This may return 0 even when this is a match state, since the pattern
    /// ID `PatternID::ZERO` is not encoded when it's the only pattern ID in
    /// the match state (the overwhelming common case).
    </span><span class="kw">fn </span>encoded_pattern_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="kw">if </span>!<span class="self">self</span>.has_pattern_ids() {
            <span class="kw">return </span><span class="number">0</span>;
        }
        <span class="comment">// This unwrap is OK since the total number of patterns is always
        // guaranteed to fit into a usize.
        </span>usize::try_from(wire::read_u32(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>[<span class="number">9</span>..<span class="number">13</span>])).unwrap()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; core::fmt::Debug <span class="kw">for </span>Repr&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        <span class="kw">let </span><span class="kw-2">mut </span>nfa_ids = <span class="macro">alloc::vec!</span>[];
        <span class="self">self</span>.iter_nfa_state_ids(|sid| nfa_ids.push(sid));
        f.debug_struct(<span class="string">"Repr"</span>)
            .field(<span class="string">"is_match"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.is_match())
            .field(<span class="string">"is_from_word"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.is_from_word())
            .field(<span class="string">"is_half_crlf"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.is_half_crlf())
            .field(<span class="string">"look_have"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.look_have())
            .field(<span class="string">"look_need"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.look_need())
            .field(<span class="string">"match_pattern_ids"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.match_pattern_ids())
            .field(<span class="string">"nfa_state_ids"</span>, <span class="kw-2">&amp;</span>nfa_ids)
            .finish()
    }
}

<span class="doccomment">/// ReprVec is a write-only view into the representation of a DFA state.
///
/// See Repr for more details on the purpose of this type and also the format.
///
/// Note that not all possible combinations of methods may be called. This is
/// precisely what the various StateBuilder types encapsulate: they only
/// permit valid combinations via Rust's linear typing.
</span><span class="kw">struct </span>ReprVec&lt;<span class="lifetime">'a</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>Vec&lt;u8&gt;);

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; ReprVec&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// Set this state as a match state.
    ///
    /// This should not be exposed explicitly outside of this module. It is
    /// set automatically when a pattern ID is added.
    </span><span class="kw">fn </span>set_is_match(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.<span class="number">0</span>[<span class="number">0</span>] |= <span class="number">1 </span>&lt;&lt; <span class="number">0</span>;
    }

    <span class="doccomment">/// Set that this state has pattern IDs explicitly written to it.
    ///
    /// This should not be exposed explicitly outside of this module. This is
    /// used internally as a space saving optimization. Namely, if the state
    /// is a match state but does not have any pattern IDs written to it,
    /// then it is automatically inferred to have a pattern ID of ZERO.
    </span><span class="kw">fn </span>set_has_pattern_ids(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.<span class="number">0</span>[<span class="number">0</span>] |= <span class="number">1 </span>&lt;&lt; <span class="number">1</span>;
    }

    <span class="doccomment">/// Set this state as being built from a transition over a word byte.
    ///
    /// Setting this is only necessary when one needs to deal with word
    /// boundary assertions. Therefore, if the underlying NFA has no word
    /// boundary assertions, callers should not set this.
    </span><span class="kw">fn </span>set_is_from_word(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.<span class="number">0</span>[<span class="number">0</span>] |= <span class="number">1 </span>&lt;&lt; <span class="number">2</span>;
    }

    <span class="doccomment">/// Set this state as having seen half of a CRLF terminator.
    ///
    /// In the forward direction, this should be set when a `\r` has been seen.
    /// In the reverse direction, this should be set when a `\n` has been seen.
    </span><span class="kw">fn </span>set_is_half_crlf(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.<span class="number">0</span>[<span class="number">0</span>] |= <span class="number">1 </span>&lt;&lt; <span class="number">3</span>;
    }

    <span class="doccomment">/// The set of look-behind assertions that were true in the transition that
    /// created this state.
    </span><span class="kw">fn </span>look_have(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LookSet {
        <span class="self">self</span>.repr().look_have()
    }

    <span class="doccomment">/// The set of look-around (both behind and ahead) assertions that appear
    /// at least once in this state's set of NFA states.
    </span><span class="kw">fn </span>look_need(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LookSet {
        <span class="self">self</span>.repr().look_need()
    }

    <span class="doccomment">/// Mutate the set of look-behind assertions that were true in the
    /// transition that created this state.
    </span><span class="kw">fn </span>set_look_have(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw-2">mut </span>set: <span class="kw">impl </span>FnMut(LookSet) -&gt; LookSet) {
        set(<span class="self">self</span>.look_have()).write_repr(<span class="kw-2">&amp;mut </span><span class="self">self</span>.<span class="number">0</span>[<span class="number">1</span>..]);
    }

    <span class="doccomment">/// Mutate the set of look-around (both behind and ahead) assertions that
    /// appear at least once in this state's set of NFA states.
    </span><span class="kw">fn </span>set_look_need(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw-2">mut </span>set: <span class="kw">impl </span>FnMut(LookSet) -&gt; LookSet) {
        set(<span class="self">self</span>.look_need()).write_repr(<span class="kw-2">&amp;mut </span><span class="self">self</span>.<span class="number">0</span>[<span class="number">5</span>..]);
    }

    <span class="doccomment">/// Add a pattern ID to this state. All match states must have at least
    /// one pattern ID associated with it.
    ///
    /// Callers must never add duplicative pattern IDs.
    ///
    /// The order in which patterns are added must correspond to the order
    /// in which patterns are reported as matches.
    </span><span class="kw">fn </span>add_match_pattern_id(<span class="kw-2">&amp;mut </span><span class="self">self</span>, pid: PatternID) {
        <span class="comment">// As a (somewhat small) space saving optimization, in the case where
        // a matching state has exactly one pattern ID, PatternID::ZERO, we do
        // not write either the pattern ID or the number of patterns encoded.
        // Instead, all we do is set the 'is_match' bit on this state. Overall,
        // this saves 8 bytes per match state for the overwhelming majority of
        // match states.
        //
        // In order to know whether pattern IDs need to be explicitly read or
        // not, we use another internal-only bit, 'has_pattern_ids', to
        // indicate whether they have been explicitly written or not.
        </span><span class="kw">if </span>!<span class="self">self</span>.repr().has_pattern_ids() {
            <span class="kw">if </span>pid == PatternID::ZERO {
                <span class="self">self</span>.set_is_match();
                <span class="kw">return</span>;
            }
            <span class="comment">// Make room for 'close_match_pattern_ids' to write the total
            // number of pattern IDs written.
            </span><span class="self">self</span>.<span class="number">0</span>.extend(core::iter::repeat(<span class="number">0</span>).take(PatternID::SIZE));
            <span class="self">self</span>.set_has_pattern_ids();
            <span class="comment">// If this was already a match state, then the only way that's
            // possible when the state doesn't have pattern IDs is if
            // PatternID::ZERO was added by the caller previously. In this
            // case, we are now adding a non-ZERO pattern ID after it, in
            // which case, we want to make sure to represent ZERO explicitly
            // now.
            </span><span class="kw">if </span><span class="self">self</span>.repr().is_match() {
                write_u32(<span class="self">self</span>.<span class="number">0</span>, <span class="number">0</span>)
            } <span class="kw">else </span>{
                <span class="comment">// Otherwise, just make sure the 'is_match' bit is set.
                </span><span class="self">self</span>.set_is_match();
            }
        }
        write_u32(<span class="self">self</span>.<span class="number">0</span>, pid.as_u32());
    }

    <span class="doccomment">/// Indicate that no more pattern IDs will be added to this state.
    ///
    /// Once this is called, callers must not call it or 'add_match_pattern_id'
    /// again.
    ///
    /// This should not be exposed explicitly outside of this module. It
    /// should be called only when converting a StateBuilderMatches into a
    /// StateBuilderNFA.
    </span><span class="kw">fn </span>close_match_pattern_ids(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="comment">// If we never wrote any pattern IDs, then there's nothing to do here.
        </span><span class="kw">if </span>!<span class="self">self</span>.repr().has_pattern_ids() {
            <span class="kw">return</span>;
        }
        <span class="kw">let </span>patsize = PatternID::SIZE;
        <span class="kw">let </span>pattern_bytes = <span class="self">self</span>.<span class="number">0</span>.len() - <span class="number">13</span>;
        <span class="comment">// Every pattern ID uses 4 bytes, so number of bytes should be
        // divisible by 4.
        </span><span class="macro">assert_eq!</span>(pattern_bytes % patsize, <span class="number">0</span>);
        <span class="comment">// This unwrap is OK since we are guaranteed that the maximum number
        // of possible patterns fits into a u32.
        </span><span class="kw">let </span>count32 = u32::try_from(pattern_bytes / patsize).unwrap();
        wire::NE::write_u32(count32, <span class="kw-2">&amp;mut </span><span class="self">self</span>.<span class="number">0</span>[<span class="number">9</span>..<span class="number">13</span>]);
    }

    <span class="doccomment">/// Add an NFA state ID to this state. The order in which NFA states are
    /// added matters. It is the caller's responsibility to ensure that
    /// duplicate NFA state IDs are not added.
    </span><span class="kw">fn </span>add_nfa_state_id(<span class="kw-2">&amp;mut </span><span class="self">self</span>, prev: <span class="kw-2">&amp;mut </span>StateID, sid: StateID) {
        <span class="kw">let </span>delta = sid.as_i32() - prev.as_i32();
        write_vari32(<span class="self">self</span>.<span class="number">0</span>, delta);
        <span class="kw-2">*</span>prev = sid;
    }

    <span class="doccomment">/// Return a read-only view of this state's representation.
    </span><span class="kw">fn </span>repr(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Repr&lt;<span class="lifetime">'_</span>&gt; {
        Repr(<span class="self">self</span>.<span class="number">0</span>.as_slice())
    }
}

<span class="doccomment">/// Write a signed 32-bit integer using zig-zag encoding.
///
/// https://developers.google.com/protocol-buffers/docs/encoding#varints
</span><span class="kw">fn </span>write_vari32(data: <span class="kw-2">&amp;mut </span>Vec&lt;u8&gt;, n: i32) {
    <span class="kw">let </span><span class="kw-2">mut </span>un = n.to_bits() &lt;&lt; <span class="number">1</span>;
    <span class="kw">if </span>n &lt; <span class="number">0 </span>{
        un = !un;
    }
    write_varu32(data, un)
}

<span class="doccomment">/// Read a signed 32-bit integer using zig-zag encoding. Also, return the
/// number of bytes read.
///
/// https://developers.google.com/protocol-buffers/docs/encoding#varints
</span><span class="kw">fn </span>read_vari32(data: <span class="kw-2">&amp;</span>[u8]) -&gt; (i32, usize) {
    <span class="kw">let </span>(un, i) = read_varu32(data);
    <span class="kw">let </span><span class="kw-2">mut </span>n = i32::from_bits(un &gt;&gt; <span class="number">1</span>);
    <span class="kw">if </span>un &amp; <span class="number">1 </span>!= <span class="number">0 </span>{
        n = !n;
    }
    (n, i)
}

<span class="doccomment">/// Write an unsigned 32-bit integer as a varint. In essence, `n` is written
/// as a sequence of bytes where all bytes except for the last one have the
/// most significant bit set. The least significant 7 bits correspond to the
/// actual bits of `n`. So in the worst case, a varint uses 5 bytes, but in
/// very common cases, it uses fewer than 4.
///
/// https://developers.google.com/protocol-buffers/docs/encoding#varints
</span><span class="kw">fn </span>write_varu32(data: <span class="kw-2">&amp;mut </span>Vec&lt;u8&gt;, <span class="kw-2">mut </span>n: u32) {
    <span class="kw">while </span>n &gt;= <span class="number">0b1000_0000 </span>{
        data.push(n.low_u8() | <span class="number">0b1000_0000</span>);
        n &gt;&gt;= <span class="number">7</span>;
    }
    data.push(n.low_u8());
}

<span class="doccomment">/// Read an unsigned 32-bit varint. Also, return the number of bytes read.
///
/// https://developers.google.com/protocol-buffers/docs/encoding#varints
</span><span class="kw">fn </span>read_varu32(data: <span class="kw-2">&amp;</span>[u8]) -&gt; (u32, usize) {
    <span class="comment">// N.B. We can assume correctness here since we know that all varuints are
    // written with write_varu32. Hence, the 'as' uses and unchecked arithmetic
    // is all okay.
    </span><span class="kw">let </span><span class="kw-2">mut </span>n: u32 = <span class="number">0</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>shift: u32 = <span class="number">0</span>;
    <span class="kw">for </span>(i, <span class="kw-2">&amp;</span>b) <span class="kw">in </span>data.iter().enumerate() {
        <span class="kw">if </span>b &lt; <span class="number">0b1000_0000 </span>{
            <span class="kw">return </span>(n | (u32::from(b) &lt;&lt; shift), i + <span class="number">1</span>);
        }
        n |= (u32::from(b) &amp; <span class="number">0b0111_1111</span>) &lt;&lt; shift;
        shift += <span class="number">7</span>;
    }
    (<span class="number">0</span>, <span class="number">0</span>)
}

<span class="doccomment">/// Push a native-endian encoded `n` on to `dst`.
</span><span class="kw">fn </span>write_u32(dst: <span class="kw-2">&amp;mut </span>Vec&lt;u8&gt;, n: u32) {
    <span class="kw">use </span><span class="kw">crate</span>::util::wire::NE;

    <span class="kw">let </span>start = dst.len();
    dst.extend(core::iter::repeat(<span class="number">0</span>).take(mem::size_of::&lt;u32&gt;()));
    NE::write_u32(n, <span class="kw-2">&amp;mut </span>dst[start..]);
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span>alloc::vec;

    <span class="kw">use </span>quickcheck::quickcheck;

    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="attr">#[cfg(not(miri))]
    </span><span class="macro">quickcheck!</span> {
        <span class="kw">fn </span>prop_state_read_write_nfa_state_ids(sids: Vec&lt;StateID&gt;) -&gt; bool {
            <span class="comment">// Builders states do not permit duplicate IDs.
            </span><span class="kw">let </span>sids = dedup_state_ids(sids);

            <span class="kw">let </span><span class="kw-2">mut </span>b = StateBuilderEmpty::new().into_matches().into_nfa();
            <span class="kw">for </span><span class="kw-2">&amp;</span>sid <span class="kw">in </span><span class="kw-2">&amp;</span>sids {
                b.add_nfa_state_id(sid);
            }
            <span class="kw">let </span>s = b.to_state();
            <span class="kw">let </span><span class="kw-2">mut </span>got = <span class="macro">vec!</span>[];
            s.iter_nfa_state_ids(|sid| got.push(sid));
            got == sids
        }

        <span class="kw">fn </span>prop_state_read_write_pattern_ids(pids: Vec&lt;PatternID&gt;) -&gt; bool {
            <span class="comment">// Builders states do not permit duplicate IDs.
            </span><span class="kw">let </span>pids = dedup_pattern_ids(pids);

            <span class="kw">let </span><span class="kw-2">mut </span>b = StateBuilderEmpty::new().into_matches();
            <span class="kw">for </span><span class="kw-2">&amp;</span>pid <span class="kw">in </span><span class="kw-2">&amp;</span>pids {
                b.add_match_pattern_id(pid);
            }
            <span class="kw">let </span>s = b.into_nfa().to_state();
            <span class="kw">let </span><span class="kw-2">mut </span>got = <span class="macro">vec!</span>[];
            s.iter_match_pattern_ids(|pid| got.push(pid));
            got == pids
        }

        <span class="kw">fn </span>prop_state_read_write_nfa_state_and_pattern_ids(
            sids: Vec&lt;StateID&gt;,
            pids: Vec&lt;PatternID&gt;
        ) -&gt; bool {
            <span class="comment">// Builders states do not permit duplicate IDs.
            </span><span class="kw">let </span>sids = dedup_state_ids(sids);
            <span class="kw">let </span>pids = dedup_pattern_ids(pids);

            <span class="kw">let </span><span class="kw-2">mut </span>b = StateBuilderEmpty::new().into_matches();
            <span class="kw">for </span><span class="kw-2">&amp;</span>pid <span class="kw">in </span><span class="kw-2">&amp;</span>pids {
                b.add_match_pattern_id(pid);
            }

            <span class="kw">let </span><span class="kw-2">mut </span>b = b.into_nfa();
            <span class="kw">for </span><span class="kw-2">&amp;</span>sid <span class="kw">in </span><span class="kw-2">&amp;</span>sids {
                b.add_nfa_state_id(sid);
            }

            <span class="kw">let </span>s = b.to_state();
            <span class="kw">let </span><span class="kw-2">mut </span>got_pids = <span class="macro">vec!</span>[];
            s.iter_match_pattern_ids(|pid| got_pids.push(pid));
            <span class="kw">let </span><span class="kw-2">mut </span>got_sids = <span class="macro">vec!</span>[];
            s.iter_nfa_state_ids(|sid| got_sids.push(sid));
            got_pids == pids &amp;&amp; got_sids == sids
        }
    }

    <span class="macro">quickcheck!</span> {
        <span class="kw">fn </span>prop_read_write_varu32(n: u32) -&gt; bool {
            <span class="kw">let </span><span class="kw-2">mut </span>buf = <span class="macro">vec!</span>[];
            write_varu32(<span class="kw-2">&amp;mut </span>buf, n);
            <span class="kw">let </span>(got, nread) = read_varu32(<span class="kw-2">&amp;</span>buf);
            nread == buf.len() &amp;&amp; got == n
        }

        <span class="kw">fn </span>prop_read_write_vari32(n: i32) -&gt; bool {
            <span class="kw">let </span><span class="kw-2">mut </span>buf = <span class="macro">vec!</span>[];
            write_vari32(<span class="kw-2">&amp;mut </span>buf, n);
            <span class="kw">let </span>(got, nread) = read_vari32(<span class="kw-2">&amp;</span>buf);
            nread == buf.len() &amp;&amp; got == n
        }
    }

    <span class="attr">#[cfg(not(miri))]
    </span><span class="kw">fn </span>dedup_state_ids(sids: Vec&lt;StateID&gt;) -&gt; Vec&lt;StateID&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>set = alloc::collections::BTreeSet::new();
        <span class="kw">let </span><span class="kw-2">mut </span>deduped = <span class="macro">vec!</span>[];
        <span class="kw">for </span>sid <span class="kw">in </span>sids {
            <span class="kw">if </span>set.contains(<span class="kw-2">&amp;</span>sid) {
                <span class="kw">continue</span>;
            }
            set.insert(sid);
            deduped.push(sid);
        }
        deduped
    }

    <span class="attr">#[cfg(not(miri))]
    </span><span class="kw">fn </span>dedup_pattern_ids(pids: Vec&lt;PatternID&gt;) -&gt; Vec&lt;PatternID&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>set = alloc::collections::BTreeSet::new();
        <span class="kw">let </span><span class="kw-2">mut </span>deduped = <span class="macro">vec!</span>[];
        <span class="kw">for </span>pid <span class="kw">in </span>pids {
            <span class="kw">if </span>set.contains(<span class="kw-2">&amp;</span>pid) {
                <span class="kw">continue</span>;
            }
            set.insert(pid);
            deduped.push(pid);
        }
        deduped
    }
}
</code></pre></div></section></main></body></html>