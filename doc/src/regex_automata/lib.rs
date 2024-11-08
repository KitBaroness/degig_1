<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.4.6/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="regex_automata" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../regex_automata/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">/*!
This crate exposes a variety of regex engines used by the `regex` crate.
It provides a vast, sprawling and "expert" level API to each regex engine.
The regex engines provided by this crate focus heavily on finite automata
implementations and specifically guarantee worst case `O(m * n)` time
complexity for all searches. (Where `m ~ len(regex)` and `n ~ len(haystack)`.)

The primary goal of this crate is to serve as an implementation detail for the
`regex` crate. A secondary goal is to make its internals available for use by
others.

# Table of contents

* [Should I be using this crate?](#should-i-be-using-this-crate) gives some
reasons for and against using this crate.
* [Examples](#examples) provides a small selection of things you can do with
this crate.
* [Available regex engines](#available-regex-engines) provides a hyperlinked
list of all regex engines in this crate.
* [API themes](#api-themes) discusses common elements used throughout this
crate.
* [Crate features](#crate-features) documents the extensive list of Cargo
features available.

# Should I be using this crate?

If you find yourself here because you just want to use regexes, then you should
first check out whether the [`regex` crate](https://docs.rs/regex) meets
your needs. It provides a streamlined and difficult-to-misuse API for regex
searching.

If you're here because there is something specific you want to do that can't
be easily done with `regex` crate, then you are perhaps in the right place.
It's most likely that the first stop you'll want to make is to explore the
[`meta` regex APIs](meta). Namely, the `regex` crate is just a light wrapper
over a [`meta::Regex`], so its API will probably be the easiest to transition
to. In contrast to the `regex` crate, the `meta::Regex` API supports more
search parameters and does multi-pattern searches. However, it isn't quite as
ergonomic.

Otherwise, the following is an inexhaustive list of reasons to use this crate:

* You want to analyze or use a [Thompson `NFA`](nfa::thompson::NFA) directly.
* You want more powerful multi-pattern search than what is provided by
`RegexSet` in the `regex` crate. All regex engines in this crate support
multi-pattern searches.
* You want to use one of the `regex` crate's internal engines directly because
of some interesting configuration that isn't possible via the `regex` crate.
For example, a [lazy DFA's configuration](hybrid::dfa::Config) exposes a
dizzying number of options for controlling its execution.
* You want to use the lower level search APIs. For example, both the [lazy
DFA](hybrid::dfa) and [fully compiled DFAs](dfa) support searching by exploring
the automaton one state at a time. This might be useful, for example, for
stream searches or searches of strings stored in non-contiguous in memory.
* You want to build a fully compiled DFA and then [use zero-copy
deserialization](dfa::dense::DFA::from_bytes) to load it into memory and use
it for searching. This use case is supported in core-only no-std/no-alloc
environments.
* You want to run [anchored searches](Input::anchored) without using the `^`
anchor in your regex pattern.
* You need to work-around contention issues with
sharing a regex across multiple threads. The
[`meta::Regex::search_with`](meta::Regex::search_with) API permits bypassing
any kind of synchronization at all by requiring the caller to provide the
mutable scratch spaced needed during a search.
* You want to build your own regex engine on top of the `regex` crate's
infrastructure.

# Examples

This section tries to identify a few interesting things you can do with this
crate and demonstrates them.

### Multi-pattern searches with capture groups

One of the more frustrating limitations of `RegexSet` in the `regex` crate
(at the time of writing) is that it doesn't report match positions. With this
crate, multi-pattern support was intentionally designed in from the beginning,
which means it works in all regex engines and even for capture groups as well.

This example shows how to search for matches of multiple regexes, where each
regex uses the same capture group names to parse different key-value formats.

```
use regex_automata::{meta::Regex, PatternID};

let re = Regex::new_many(&amp;[
    r#"(?m)^(?&lt;key&gt;[[:word:]]+)=(?&lt;val&gt;[[:word:]]+)$"#,
    r#"(?m)^(?&lt;key&gt;[[:word:]]+)="(?&lt;val&gt;[^"]+)"$"#,
    r#"(?m)^(?&lt;key&gt;[[:word:]]+)='(?&lt;val&gt;[^']+)'$"#,
    r#"(?m)^(?&lt;key&gt;[[:word:]]+):\s*(?&lt;val&gt;[[:word:]]+)$"#,
])?;
let hay = r#"
best_album="Blow Your Face Out"
best_quote='"then as it was, then again it will be"'
best_year=1973
best_simpsons_episode: HOMR
"#;
let mut kvs = vec![];
for caps in re.captures_iter(hay) {
    // N.B. One could use capture indices '1' and '2' here
    // as well. Capture indices are local to each pattern.
    // (Just like names are.)
    let key = &amp;hay[caps.get_group_by_name("key").unwrap()];
    let val = &amp;hay[caps.get_group_by_name("val").unwrap()];
    kvs.push((key, val));
}
assert_eq!(kvs, vec![
    ("best_album", "Blow Your Face Out"),
    ("best_quote", "\"then as it was, then again it will be\""),
    ("best_year", "1973"),
    ("best_simpsons_episode", "HOMR"),
]);

# Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
```

### Build a full DFA and walk it manually

One of the regex engines in this crate is a fully compiled DFA. It takes worst
case exponential time to build, but once built, it can be easily explored and
used for searches. Here's a simple example that uses its lower level APIs to
implement a simple anchored search by hand.

```
use regex_automata::{dfa::{Automaton, dense}, Input};

let dfa = dense::DFA::new(r"(?-u)\b[A-Z]\w+z\b")?;
let haystack = "Quartz";

// The start state is determined by inspecting the position and the
// initial bytes of the haystack.
let mut state = dfa.start_state_forward(&amp;Input::new(haystack))?;
// Walk all the bytes in the haystack.
for &amp;b in haystack.as_bytes().iter() {
    state = dfa.next_state(state, b);
}
// DFAs in this crate require an explicit
// end-of-input transition if a search reaches
// the end of a haystack.
state = dfa.next_eoi_state(state);
assert!(dfa.is_match_state(state));

# Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
```

Or do the same with a lazy DFA that avoids exponential worst case compile time,
but requires mutable scratch space to lazily build the DFA during the search.

```
use regex_automata::{hybrid::dfa::DFA, Input};

let dfa = DFA::new(r"(?-u)\b[A-Z]\w+z\b")?;
let mut cache = dfa.create_cache();
let hay = "Quartz";

// The start state is determined by inspecting the position and the
// initial bytes of the haystack.
let mut state = dfa.start_state_forward(&amp;mut cache, &amp;Input::new(hay))?;
// Walk all the bytes in the haystack.
for &amp;b in hay.as_bytes().iter() {
    state = dfa.next_state(&amp;mut cache, state, b)?;
}
// DFAs in this crate require an explicit
// end-of-input transition if a search reaches
// the end of a haystack.
state = dfa.next_eoi_state(&amp;mut cache, state)?;
assert!(state.is_match());

# Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
```

### Find all overlapping matches

This example shows how to build a DFA and use it to find all possible matches,
including overlapping matches. A similar example will work with a lazy DFA as
well. This also works with multiple patterns and will report all matches at the
same position where multiple patterns match.

```
use regex_automata::{
    dfa::{dense, Automaton, OverlappingState},
    Input, MatchKind,
};

let dfa = dense::DFA::builder()
    .configure(dense::DFA::config().match_kind(MatchKind::All))
    .build(r"(?-u)\w{3,}")?;
let input = Input::new("homer marge bart lisa maggie");
let mut state = OverlappingState::start();

let mut matches = vec![];
while let Some(hm) = {
    dfa.try_search_overlapping_fwd(&amp;input, &amp;mut state)?;
    state.get_match()
} {
    matches.push(hm.offset());
}
assert_eq!(matches, vec![
    3, 4, 5,        // hom, home, homer
    9, 10, 11,      // mar, marg, marge
    15, 16,         // bar, bart
    20, 21,         // lis, lisa
    25, 26, 27, 28, // mag, magg, maggi, maggie
]);

# Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
```

# Available regex engines

The following is a complete list of all regex engines provided by this crate,
along with a very brief description of it and why you might want to use it.

* [`dfa::regex::Regex`] is a regex engine that works on top of either
[dense](dfa::dense) or [sparse](dfa::sparse) fully compiled DFAs. You might
use a DFA if you need the fastest possible regex engine in this crate and can
afford the exorbitant memory usage usually required by DFAs. Low level APIs on
fully compiled DFAs are provided by the [`Automaton` trait](dfa::Automaton).
Fully compiled dense DFAs can handle all regexes except for searching a regex
with a Unicode word boundary on non-ASCII haystacks. A fully compiled DFA based
regex can only report the start and end of each match.
* [`hybrid::regex::Regex`] is a regex engine that works on top of a lazily
built DFA. Its performance profile is very similar to that of fully compiled
DFAs, but can be slower in some pathological cases. Fully compiled DFAs are
also amenable to more optimizations, such as state acceleration, that aren't
available in a lazy DFA. You might use this lazy DFA if you can't abide the
worst case exponential compile time of a full DFA, but still want the DFA
search performance in the vast majority of cases. A lazy DFA based regex can
only report the start and end of each match.
* [`dfa::onepass::DFA`] is a regex engine that is implemented as a DFA, but
can report the matches of each capture group in addition to the start and end
of each match. The catch is that it only works on a somewhat small subset of
regexes known as "one-pass." You'll want to use this for cases when you need
capture group matches and the regex is one-pass since it is likely to be faster
than any alternative. A one-pass DFA can handle all types of regexes, but does
have some reasonable limits on the number of capture groups it can handle.
* [`nfa::thompson::backtrack::BoundedBacktracker`] is a regex engine that uses
backtracking, but keeps track of the work it has done to avoid catastrophic
backtracking. Like the one-pass DFA, it provides the matches of each capture
group. It retains the `O(m * n)` worst case time bound. This tends to be slower
than the one-pass DFA regex engine, but faster than the PikeVM. It can handle
all types of regexes, but usually only works well with small haystacks and
small regexes due to the memory required to avoid redoing work.
* [`nfa::thompson::pikevm::PikeVM`] is a regex engine that can handle all
regexes, of all sizes and provides capture group matches. It tends to be a tool
of last resort because it is also usually the slowest regex engine.
* [`meta::Regex`] is the meta regex engine that combines *all* of the above
engines into one. The reason for this is that each of the engines above have
their own caveats such as, "only handles a subset of regexes" or "is generally
slow." The meta regex engine accounts for all of these caveats and composes
the engines in a way that attempts to mitigate each engine's weaknesses while
emphasizing its strengths. For example, it will attempt to run a lazy DFA even
if it might fail. In which case, it will restart the search with a likely
slower but more capable regex engine. The meta regex engine is what you should
default to. Use one of the above engines directly only if you have a specific
reason to.

# API themes

While each regex engine has its own APIs and configuration options, there are
some general themes followed by all of them.

### The `Input` abstraction

Most search routines in this crate accept anything that implements
`Into&lt;Input&gt;`. Both `&amp;str` and `&amp;[u8]` haystacks satisfy this constraint, which
means that things like `engine.search("foo")` will work as you would expect.

By virtue of accepting an `Into&lt;Input&gt;` though, callers can provide more than
just a haystack. Indeed, the [`Input`] type has more details, but briefly,
callers can use it to configure various aspects of the search:

* The span of the haystack to search via [`Input::span`] or [`Input::range`],
which might be a substring of the haystack.
* Whether to run an anchored search or not via [`Input::anchored`]. This
permits one to require matches to start at the same offset that the search
started.
* Whether to ask the regex engine to stop as soon as a match is seen via
[`Input::earliest`]. This can be used to find the offset of a match as soon
as it is known without waiting for the full leftmost-first match to be found.
This can also be used to avoid the worst case `O(m * n^2)` time complexity
of iteration.

Some lower level search routines accept an `&amp;Input` for performance reasons.
In which case, `&amp;Input::new("haystack")` can be used for a simple search.

### Error reporting

Most, but not all, regex engines in this crate can fail to execute a search.
When a search fails, callers cannot determine whether or not a match exists.
That is, the result is indeterminate.

Search failure, in all cases in this crate, is represented by a [`MatchError`].
Routines that can fail start with the `try_` prefix in their name. For example,
[`hybrid::regex::Regex::try_search`] can fail for a number of reasons.
Conversely, routines that either can't fail or can panic on failure lack the
`try_` prefix. For example, [`hybrid::regex::Regex::find`] will panic in
cases where [`hybrid::regex::Regex::try_search`] would return an error, and
[`meta::Regex::find`] will never panic. Therefore, callers need to pay close
attention to the panicking conditions in the documentation.

In most cases, the reasons that a search fails are either predictable or
configurable, albeit at some additional cost.

An example of predictable failure is
[`BoundedBacktracker::try_search`](nfa::thompson::backtrack::BoundedBacktracker::try_search).
Namely, it fails whenever the multiplication of the haystack, the regex and some
constant exceeds the
[configured visited capacity](nfa::thompson::backtrack::Config::visited_capacity).
Callers can predict the failure in terms of haystack length via the
[`BoundedBacktracker::max_haystack_len`](nfa::thompson::backtrack::BoundedBacktracker::max_haystack_len)
method. While this form of failure is technically avoidable by increasing the
visited capacity, it isn't practical to do so for all inputs because the
memory usage required for larger haystacks becomes impractically large. So in
practice, if one is using the bounded backtracker, you really do have to deal
with the failure.

An example of configurable failure happens when one enables heuristic support
for Unicode word boundaries in a DFA. Namely, since the DFAs in this crate
(except for the one-pass DFA) do not support Unicode word boundaries on
non-ASCII haystacks, building a DFA from an NFA that contains a Unicode word
boundary will itself fail. However, one can configure DFAs to still be built in
this case by
[configuring heuristic support for Unicode word boundaries](hybrid::dfa::Config::unicode_word_boundary).
If the NFA the DFA is built from contains a Unicode word boundary, then the
DFA will still be built, but special transitions will be added to every state
that cause the DFA to fail if any non-ASCII byte is seen. This failure happens
at search time and it requires the caller to opt into this.

There are other ways for regex engines to fail in this crate, but the above
two should represent the general theme of failures one can find. Dealing
with these failures is, in part, one the responsibilities of the [meta regex
engine](meta). Notice, for example, that the meta regex engine exposes an API
that never returns an error nor panics. It carefully manages all of the ways
in which the regex engines can fail and either avoids the predictable ones
entirely (e.g., the bounded backtracker) or reacts to configured failures by
falling back to a different engine (e.g., the lazy DFA quitting because it saw
a non-ASCII byte).

### Configuration and Builders

Most of the regex engines in this crate come with two types to facilitate
building the regex engine: a `Config` and a `Builder`. A `Config` is usually
specific to that particular regex engine, but other objects such as parsing and
NFA compilation have `Config` types too. A `Builder` is the thing responsible
for taking inputs (either pattern strings or already-parsed patterns or even
NFAs directly) and turning them into an actual regex engine that can be used
for searching.

The main reason why building a regex engine is a bit complicated is because
of the desire to permit composition with de-coupled components. For example,
you might want to [manually construct a Thompson NFA](nfa::thompson::Builder)
and then build a regex engine from it without ever using a regex parser
at all. On the other hand, you might also want to build a regex engine directly
from the concrete syntax. This demonstrates why regex engine construction is
so flexible: it needs to support not just convenient construction, but also
construction from parts built elsewhere.

This is also in turn why there are many different `Config` structs in this
crate. Let's look more closely at an example: [`hybrid::regex::Builder`]. It
accepts three different `Config` types for configuring construction of a lazy
DFA regex:

* [`hybrid::regex::Builder::syntax`] accepts a
[`util::syntax::Config`] for configuring the options found in the
[`regex-syntax`](regex_syntax) crate. For example, whether to match
case insensitively.
* [`hybrid::regex::Builder::thompson`] accepts a [`nfa::thompson::Config`] for
configuring construction of a [Thompson NFA](nfa::thompson::NFA). For example,
whether to build an NFA that matches the reverse language described by the
regex.
* [`hybrid::regex::Builder::dfa`] accept a [`hybrid::dfa::Config`] for
configuring construction of the pair of underlying lazy DFAs that make up the
lazy DFA regex engine. For example, changing the capacity of the cache used to
store the transition table.

The lazy DFA regex engine uses all three of those configuration objects for
methods like [`hybrid::regex::Builder::build`], which accepts a pattern
string containing the concrete syntax of your regex. It uses the syntax
configuration to parse it into an AST and translate it into an HIR. Then the
NFA configuration when compiling the HIR into an NFA. And then finally the DFA
configuration when lazily determinizing the NFA into a DFA.

Notice though that the builder also has a
[`hybrid::regex::Builder::build_from_dfas`] constructor. This permits callers
to build the underlying pair of lazy DFAs themselves (one for the forward
searching to find the end of a match and one for the reverse searching to find
the start of a match), and then build the regex engine from them. The lazy
DFAs, in turn, have their own builder that permits [construction directly from
a Thompson NFA](hybrid::dfa::Builder::build_from_nfa). Continuing down the
rabbit hole, a Thompson NFA has its own compiler that permits [construction
directly from an HIR](nfa::thompson::Compiler::build_from_hir). The lazy DFA
regex engine builder lets you follow this rabbit hole all the way down, but
also provides convenience routines that do it for you when you don't need
precise control over every component.

The [meta regex engine](meta) is a good example of something that utilizes the
full flexibility of these builders. It often needs not only precise control
over each component, but also shares them across multiple regex engines.
(Most sharing is done by internal reference accounting. For example, an
[`NFA`](nfa::thompson::NFA) is reference counted internally which makes cloning
cheap.)

### Size limits

Unlike the `regex` crate, the `regex-automata` crate specifically does not
enable any size limits by default. That means users of this crate need to
be quite careful when using untrusted patterns. Namely, because bounded
repetitions can grow exponentially by stacking them, it is possible to build a
very large internal regex object from just a small pattern string. For example,
the NFA built from the pattern `a{10}{10}{10}{10}{10}{10}{10}` is over 240MB.

There are multiple size limit options in this crate. If one or more size limits
are relevant for the object you're building, they will be configurable via
methods on a corresponding `Config` type.

# Crate features

This crate has a dizzying number of features. The main idea is to be able to
control how much stuff you pull in for your specific use case, since the full
crate is quite large and can dramatically increase compile times and binary
size.

The most barebones but useful configuration is to disable all default features
and enable only `dfa-search`. This will bring in just the DFA deserialization
and search routines without any dependency on `std` or `alloc`. This does
require generating and serializing a DFA, and then storing it somewhere, but
it permits regex searches in freestanding or embedded environments.

Because there are so many features, they are split into a few groups.

The default set of features is: `std`, `syntax`, `perf`, `unicode`, `meta`,
`nfa`, `dfa` and `hybrid`. Basically, the default is to enable everything
except for development related features like `logging`.

### Ecosystem features

* **std** - Enables use of the standard library. In terms of APIs, this usually
just means that error types implement the `std::error::Error` trait. Otherwise,
`std` sometimes enables the code to be faster, for example, using a `HashMap`
instead of a `BTreeMap`. (The `std` feature matters more for dependencies like
`aho-corasick` and `memchr`, where `std` is required to enable certain classes
of SIMD optimizations.) Enabling `std` automatically enables `alloc`.
* **alloc** - Enables use of the `alloc` library. This is required for most
APIs in this crate. The main exception is deserializing and searching with
fully compiled DFAs.
* **logging** - Adds a dependency on the `log` crate and makes this crate emit
log messages of varying degrees of utility. The log messages are especially
useful in trying to understand what the meta regex engine is doing.

### Performance features

* **perf** - Enables all of the below features.
* **perf-inline** - When enabled, `inline(always)` is used in (many) strategic
locations to help performance at the expense of longer compile times and
increased binary size.
* **perf-literal** - Enables all literal related optimizations.
    * **perf-literal-substring** - Enables all single substring literal
    optimizations. This includes adding a dependency on the `memchr` crate.
    * **perf-literal-multisubstring** - Enables all multiple substring literal
    optimizations. This includes adding a dependency on the `aho-corasick`
    crate.

### Unicode features

* **unicode** -
  Enables all Unicode features. This feature is enabled by default, and will
  always cover all Unicode features, even if more are added in the future.
* **unicode-age** -
  Provide the data for the
  [Unicode `Age` property](https://www.unicode.org/reports/tr44/tr44-24.html#Character_Age).
  This makes it possible to use classes like `\p{Age:6.0}` to refer to all
  codepoints first introduced in Unicode 6.0
* **unicode-bool** -
  Provide the data for numerous Unicode boolean properties. The full list
  is not included here, but contains properties like `Alphabetic`, `Emoji`,
  `Lowercase`, `Math`, `Uppercase` and `White_Space`.
* **unicode-case** -
  Provide the data for case insensitive matching using
  [Unicode's "simple loose matches" specification](https://www.unicode.org/reports/tr18/#Simple_Loose_Matches).
* **unicode-gencat** -
  Provide the data for
  [Unicode general categories](https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values).
  This includes, but is not limited to, `Decimal_Number`, `Letter`,
  `Math_Symbol`, `Number` and `Punctuation`.
* **unicode-perl** -
  Provide the data for supporting the Unicode-aware Perl character classes,
  corresponding to `\w`, `\s` and `\d`. This is also necessary for using
  Unicode-aware word boundary assertions. Note that if this feature is
  disabled, the `\s` and `\d` character classes are still available if the
  `unicode-bool` and `unicode-gencat` features are enabled, respectively.
* **unicode-script** -
  Provide the data for
  [Unicode scripts and script extensions](https://www.unicode.org/reports/tr24/).
  This includes, but is not limited to, `Arabic`, `Cyrillic`, `Hebrew`,
  `Latin` and `Thai`.
* **unicode-segment** -
  Provide the data necessary to provide the properties used to implement the
  [Unicode text segmentation algorithms](https://www.unicode.org/reports/tr29/).
  This enables using classes like `\p{gcb=Extend}`, `\p{wb=Katakana}` and
  `\p{sb=ATerm}`.
* **unicode-word-boundary** -
  Enables support for Unicode word boundaries, i.e., `\b`, in regexes. When
  this and `unicode-perl` are enabled, then data tables from `regex-syntax` are
  used to implement Unicode word boundaries. However, if `regex-syntax` isn't
  enabled as a dependency then one can still enable this feature. It will
  cause `regex-automata` to bundle its own data table that would otherwise be
  redundant with `regex-syntax`'s table.

### Regex engine features

* **syntax** - Enables a dependency on `regex-syntax`. This makes APIs
for building regex engines from pattern strings available. Without the
`regex-syntax` dependency, the only way to build a regex engine is generally
to deserialize a previously built DFA or to hand assemble an NFA using its
[builder API](nfa::thompson::Builder). Once you have an NFA, you can build any
of the regex engines in this crate. The `syntax` feature also enables `alloc`.
* **meta** - Enables the meta regex engine. This also enables the `syntax` and
`nfa-pikevm` features, as both are the minimal requirements needed. The meta
regex engine benefits from enabling any of the other regex engines and will
use them automatically when appropriate.
* **nfa** - Enables all NFA related features below.
    * **nfa-thompson** - Enables the Thompson NFA APIs. This enables `alloc`.
    * **nfa-pikevm** - Enables the PikeVM regex engine. This enables
    `nfa-thompson`.
    * **nfa-backtrack** - Enables the bounded backtracker regex engine. This
    enables `nfa-thompson`.
* **dfa** - Enables all DFA related features below.
    * **dfa-build** - Enables APIs for determinizing DFAs from NFAs. This
    enables `nfa-thompson` and `dfa-search`.
    * **dfa-search** - Enables APIs for searching with DFAs.
    * **dfa-onepass** - Enables the one-pass DFA API. This enables
    `nfa-thompson`.
* **hybrid** - Enables the hybrid NFA/DFA or "lazy DFA" regex engine. This
enables `alloc` and `nfa-thompson`.

*/

</span><span class="comment">// We are no_std.
</span><span class="attr">#![no_std]
</span><span class="comment">// All APIs need docs!
</span><span class="attr">#![deny(missing_docs)]
</span><span class="comment">// Some intra-doc links are broken when certain features are disabled, so we
// only bleat about it when most (all?) features are enabled. But when we do,
// we block the build. Links need to work.
</span><span class="attr">#![cfg_attr(
    all(
        feature = <span class="string">"std"</span>,
        feature = <span class="string">"nfa"</span>,
        feature = <span class="string">"dfa"</span>,
        feature = <span class="string">"hybrid"
    </span>),
    deny(rustdoc::broken_intra_doc_links)
)]
</span><span class="comment">// Broken rustdoc links are very easy to come by when you start disabling
// features. Namely, features tend to change imports, and imports change what's
// available to link to.
//
// Basically, we just don't support rustdoc for anything other than the maximal
// feature configuration. Other configurations will work, they just won't be
// perfect.
//
// So here, we specifically allow them so we don't even get warned about them.
</span><span class="attr">#![cfg_attr(
    not(all(
        feature = <span class="string">"std"</span>,
        feature = <span class="string">"nfa"</span>,
        feature = <span class="string">"dfa"</span>,
        feature = <span class="string">"hybrid"
    </span>)),
    allow(rustdoc::broken_intra_doc_links)
)]
</span><span class="comment">// Kinda similar, but eliminating all of the dead code and unused import
// warnings for every feature combo is a fool's errand. Instead, we just
// suppress those, but still let them through in a common configuration when we
// build most of everything.
//
// This does actually suggest that when features are disabled, we are actually
// compiling more code than we need to be. And this is perhaps not so great
// because disabling features is usually done in order to reduce compile times
// by reducing the amount of code one compiles... However, usually, most of the
// time this dead code is a relatively small amount from the 'util' module.
// But... I confess... There isn't a ton of visibility on this.
//
// I'm happy to try to address this in a different way, but "let's annotate
// every function in 'util' with some non-local combination of features" just
// cannot be the way forward.
</span><span class="attr">#![cfg_attr(
    not(all(
        feature = <span class="string">"std"</span>,
        feature = <span class="string">"nfa"</span>,
        feature = <span class="string">"dfa"</span>,
        feature = <span class="string">"hybrid"</span>,
        feature = <span class="string">"perf-literal-substring"</span>,
        feature = <span class="string">"perf-literal-multisubstring"</span>,
    )),
    allow(dead_code, unused_imports, unused_variables)
)]
</span><span class="comment">// We generally want all types to impl Debug.
</span><span class="attr">#![warn(missing_debug_implementations)]
</span><span class="comment">// No clue why this thing is still unstable because it's pretty amazing. This
// adds Cargo feature annotations to items in the rustdoc output. Which is
// sadly hugely beneficial for this crate due to the number of features.
</span><span class="attr">#![cfg_attr(docsrs, feature(doc_auto_cfg))]

</span><span class="comment">// I have literally never tested this crate on 16-bit, so it is quite
// suspicious to advertise support for it. But... the regex crate, at time
// of writing, at least claims to support it by not doing any conditional
// compilation based on the target pointer width. So I guess I remain
// consistent with that here.
//
// If you are here because you're on a 16-bit system and you were somehow using
// the regex crate previously, please file an issue. Please be prepared to
// provide some kind of reproduction or carve out some path to getting 16-bit
// working in CI. (Via qemu?)
</span><span class="attr">#[cfg(not(any(
    target_pointer_width = <span class="string">"16"</span>,
    target_pointer_width = <span class="string">"32"</span>,
    target_pointer_width = <span class="string">"64"
</span>)))]
</span><span class="macro">compile_error!</span>(<span class="string">"not supported on non-{16,32,64}, please file an issue"</span>);

<span class="attr">#[cfg(any(test, feature = <span class="string">"std"</span>))]
</span><span class="kw">extern crate </span>std;

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">extern crate </span>alloc;

<span class="attr">#[cfg(doctest)]
</span><span class="macro">doc_comment::doctest!</span>(<span class="string">"../README.md"</span>);

<span class="attr">#[doc(inline)]
</span><span class="kw">pub use </span><span class="kw">crate</span>::util::primitives::PatternID;
<span class="kw">pub use </span><span class="kw">crate</span>::util::search::<span class="kw-2">*</span>;

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>macros;

<span class="attr">#[cfg(any(feature = <span class="string">"dfa-search"</span>, feature = <span class="string">"dfa-onepass"</span>))]
</span><span class="kw">pub mod </span>dfa;
<span class="attr">#[cfg(feature = <span class="string">"hybrid"</span>)]
</span><span class="kw">pub mod </span>hybrid;
<span class="attr">#[cfg(feature = <span class="string">"meta"</span>)]
</span><span class="kw">pub mod </span>meta;
<span class="attr">#[cfg(feature = <span class="string">"nfa-thompson"</span>)]
</span><span class="kw">pub mod </span>nfa;
<span class="kw">pub mod </span>util;
</code></pre></div></section></main></body></html>