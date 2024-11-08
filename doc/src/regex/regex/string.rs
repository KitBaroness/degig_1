<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-1.10.4/src/regex/string.rs`."><title>string.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="regex" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../regex/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span>alloc::{borrow::Cow, string::String, sync::Arc};

<span class="kw">use </span>regex_automata::{meta, util::captures, Input, PatternID};

<span class="kw">use crate</span>::{error::Error, RegexBuilder};

<span class="doccomment">/// A compiled regular expression for searching Unicode haystacks.
///
/// A `Regex` can be used to search haystacks, split haystacks into substrings
/// or replace substrings in a haystack with a different substring. All
/// searching is done with an implicit `(?s:.)*?` at the beginning and end of
/// an pattern. To force an expression to match the whole string (or a prefix
/// or a suffix), you must use an anchor like `^` or `$` (or `\A` and `\z`).
///
/// While this crate will handle Unicode strings (whether in the regular
/// expression or in the haystack), all positions returned are **byte
/// offsets**. Every byte offset is guaranteed to be at a Unicode code point
/// boundary. That is, all offsets returned by the `Regex` API are guaranteed
/// to be ranges that can slice a `&amp;str` without panicking. If you want to
/// relax this requirement, then you must search `&amp;[u8]` haystacks with a
/// [`bytes::Regex`](crate::bytes::Regex).
///
/// The only methods that allocate new strings are the string replacement
/// methods. All other methods (searching and splitting) return borrowed
/// references into the haystack given.
///
/// # Example
///
/// Find the offsets of a US phone number:
///
/// ```
/// use regex::Regex;
///
/// let re = Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
/// let m = re.find("phone: 111-222-3333").unwrap();
/// assert_eq!(7..19, m.range());
/// ```
///
/// # Example: extracting capture groups
///
/// A common way to use regexes is with capture groups. That is, instead of
/// just looking for matches of an entire regex, parentheses are used to create
/// groups that represent part of the match.
///
/// For example, consider a haystack with multiple lines, and each line has
/// three whitespace delimited fields where the second field is expected to be
/// a number and the third field a boolean. To make this convenient, we use
/// the [`Captures::extract`] API to put the strings that match each group
/// into a fixed size array:
///
/// ```
/// use regex::Regex;
///
/// let hay = "
/// rabbit         54 true
/// groundhog 2 true
/// does not match
/// fox   109    false
/// ";
/// let re = Regex::new(r"(?m)^\s*(\S+)\s+([0-9]+)\s+(true|false)\s*$").unwrap();
/// let mut fields: Vec&lt;(&amp;str, i64, bool)&gt; = vec![];
/// for (_, [f1, f2, f3]) in re.captures_iter(hay).map(|caps| caps.extract()) {
///     fields.push((f1, f2.parse()?, f3.parse()?));
/// }
/// assert_eq!(fields, vec![
///     ("rabbit", 54, true),
///     ("groundhog", 2, true),
///     ("fox", 109, false),
/// ]);
///
/// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
/// ```
///
/// # Example: searching with the `Pattern` trait
///
/// **Note**: This section requires that this crate is compiled with the
/// `pattern` Cargo feature enabled, which **requires nightly Rust**.
///
/// Since `Regex` implements `Pattern` from the standard library, one can
/// use regexes with methods defined on `&amp;str`. For example, `is_match`,
/// `find`, `find_iter` and `split` can, in some cases, be replaced with
/// `str::contains`, `str::find`, `str::match_indices` and `str::split`.
///
/// Here are some examples:
///
/// ```ignore
/// use regex::Regex;
///
/// let re = Regex::new(r"\d+").unwrap();
/// let hay = "a111b222c";
///
/// assert!(hay.contains(&amp;re));
/// assert_eq!(hay.find(&amp;re), Some(1));
/// assert_eq!(hay.match_indices(&amp;re).collect::&lt;Vec&lt;_&gt;&gt;(), vec![
///     (1, "111"),
///     (5, "222"),
/// ]);
/// assert_eq!(hay.split(&amp;re).collect::&lt;Vec&lt;_&gt;&gt;(), vec!["a", "b", "c"]);
/// ```
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub struct </span>Regex {
    <span class="kw">pub</span>(<span class="kw">crate</span>) meta: meta::Regex,
    <span class="kw">pub</span>(<span class="kw">crate</span>) pattern: Arc&lt;str&gt;,
}

<span class="kw">impl </span>core::fmt::Display <span class="kw">for </span>Regex {
    <span class="doccomment">/// Shows the original regular expression.
    </span><span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; core::fmt::Result {
        <span class="macro">write!</span>(f, <span class="string">"{}"</span>, <span class="self">self</span>.as_str())
    }
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>Regex {
    <span class="doccomment">/// Shows the original regular expression.
    </span><span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; core::fmt::Result {
        f.debug_tuple(<span class="string">"Regex"</span>).field(<span class="kw-2">&amp;</span><span class="self">self</span>.as_str()).finish()
    }
}

<span class="kw">impl </span>core::str::FromStr <span class="kw">for </span>Regex {
    <span class="kw">type </span><span class="prelude-val">Err </span>= Error;

    <span class="doccomment">/// Attempts to parse a string into a regular expression
    </span><span class="kw">fn </span>from_str(s: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;Regex, Error&gt; {
        Regex::new(s)
    }
}

<span class="kw">impl </span>TryFrom&lt;<span class="kw-2">&amp;</span>str&gt; <span class="kw">for </span>Regex {
    <span class="kw">type </span>Error = Error;

    <span class="doccomment">/// Attempts to parse a string into a regular expression
    </span><span class="kw">fn </span>try_from(s: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;Regex, Error&gt; {
        Regex::new(s)
    }
}

<span class="kw">impl </span>TryFrom&lt;String&gt; <span class="kw">for </span>Regex {
    <span class="kw">type </span>Error = Error;

    <span class="doccomment">/// Attempts to parse a string into a regular expression
    </span><span class="kw">fn </span>try_from(s: String) -&gt; <span class="prelude-ty">Result</span>&lt;Regex, Error&gt; {
        Regex::new(<span class="kw-2">&amp;</span>s)
    }
}

<span class="doccomment">/// Core regular expression methods.
</span><span class="kw">impl </span>Regex {
    <span class="doccomment">/// Compiles a regular expression. Once compiled, it can be used repeatedly
    /// to search, split or replace substrings in a haystack.
    ///
    /// Note that regex compilation tends to be a somewhat expensive process,
    /// and unlike higher level environments, compilation is not automatically
    /// cached for you. One should endeavor to compile a regex once and then
    /// reuse it. For example, it's a bad idea to compile the same regex
    /// repeatedly in a loop.
    ///
    /// # Errors
    ///
    /// If an invalid pattern is given, then an error is returned.
    /// An error is also returned if the pattern is valid, but would
    /// produce a regex that is bigger than the configured size limit via
    /// [`RegexBuilder::size_limit`]. (A reasonable size limit is enabled by
    /// default.)
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// // An Invalid pattern because of an unclosed parenthesis
    /// assert!(Regex::new(r"foo(bar").is_err());
    /// // An invalid pattern because the regex would be too big
    /// // because Unicode tends to inflate things.
    /// assert!(Regex::new(r"\w{1000}").is_err());
    /// // Disabling Unicode can make the regex much smaller,
    /// // potentially by up to or more than an order of magnitude.
    /// assert!(Regex::new(r"(?-u:\w){1000}").is_ok());
    /// ```
    </span><span class="kw">pub fn </span>new(re: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;Regex, Error&gt; {
        RegexBuilder::new(re).build()
    }

    <span class="doccomment">/// Returns true if and only if there is a match for the regex anywhere
    /// in the haystack given.
    ///
    /// It is recommended to use this method if all you need to do is test
    /// whether a match exists, since the underlying matching engine may be
    /// able to do less work.
    ///
    /// # Example
    ///
    /// Test if some haystack contains at least one word with exactly 13
    /// Unicode word characters:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"\b\w{13}\b").unwrap();
    /// let hay = "I categorically deny having triskaidekaphobia.";
    /// assert!(re.is_match(hay));
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_match(<span class="kw-2">&amp;</span><span class="self">self</span>, haystack: <span class="kw-2">&amp;</span>str) -&gt; bool {
        <span class="self">self</span>.is_match_at(haystack, <span class="number">0</span>)
    }

    <span class="doccomment">/// This routine searches for the first match of this regex in the
    /// haystack given, and if found, returns a [`Match`]. The `Match`
    /// provides access to both the byte offsets of the match and the actual
    /// substring that matched.
    ///
    /// Note that this should only be used if you want to find the entire
    /// match. If instead you just want to test the existence of a match,
    /// it's potentially faster to use `Regex::is_match(hay)` instead of
    /// `Regex::find(hay).is_some()`.
    ///
    /// # Example
    ///
    /// Find the first word with exactly 13 Unicode word characters:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"\b\w{13}\b").unwrap();
    /// let hay = "I categorically deny having triskaidekaphobia.";
    /// let mat = re.find(hay).unwrap();
    /// assert_eq!(2..15, mat.range());
    /// assert_eq!("categorically", mat.as_str());
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>find&lt;<span class="lifetime">'h</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str) -&gt; <span class="prelude-ty">Option</span>&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="self">self</span>.find_at(haystack, <span class="number">0</span>)
    }

    <span class="doccomment">/// Returns an iterator that yields successive non-overlapping matches in
    /// the given haystack. The iterator yields values of type [`Match`].
    ///
    /// # Time complexity
    ///
    /// Note that since `find_iter` runs potentially many searches on the
    /// haystack and since each search has worst case `O(m * n)` time
    /// complexity, the overall worst case time complexity for iteration is
    /// `O(m * n^2)`.
    ///
    /// # Example
    ///
    /// Find every word with exactly 13 Unicode word characters:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"\b\w{13}\b").unwrap();
    /// let hay = "Retroactively relinquishing remunerations is reprehensible.";
    /// let matches: Vec&lt;_&gt; = re.find_iter(hay).map(|m| m.as_str()).collect();
    /// assert_eq!(matches, vec![
    ///     "Retroactively",
    ///     "relinquishing",
    ///     "remunerations",
    ///     "reprehensible",
    /// ]);
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>find_iter&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'r </span><span class="self">self</span>, haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str) -&gt; Matches&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
        Matches { haystack, it: <span class="self">self</span>.meta.find_iter(haystack) }
    }

    <span class="doccomment">/// This routine searches for the first match of this regex in the haystack
    /// given, and if found, returns not only the overall match but also the
    /// matches of each capture group in the regex. If no match is found, then
    /// `None` is returned.
    ///
    /// Capture group `0` always corresponds to an implicit unnamed group that
    /// includes the entire match. If a match is found, this group is always
    /// present. Subsequent groups may be named and are numbered, starting
    /// at 1, by the order in which the opening parenthesis appears in the
    /// pattern. For example, in the pattern `(?&lt;a&gt;.(?&lt;b&gt;.))(?&lt;c&gt;.)`, `a`,
    /// `b` and `c` correspond to capture group indices `1`, `2` and `3`,
    /// respectively.
    ///
    /// You should only use `captures` if you need access to the capture group
    /// matches. Otherwise, [`Regex::find`] is generally faster for discovering
    /// just the overall match.
    ///
    /// # Example
    ///
    /// Say you have some haystack with movie names and their release years,
    /// like "'Citizen Kane' (1941)". It'd be nice if we could search for
    /// substrings looking like that, while also extracting the movie name and
    /// its release year separately. The example below shows how to do that.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    /// let hay = "Not my favorite movie: 'Citizen Kane' (1941).";
    /// let caps = re.captures(hay).unwrap();
    /// assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");
    /// assert_eq!(caps.get(1).unwrap().as_str(), "Citizen Kane");
    /// assert_eq!(caps.get(2).unwrap().as_str(), "1941");
    /// // You can also access the groups by index using the Index notation.
    /// // Note that this will panic on an invalid index. In this case, these
    /// // accesses are always correct because the overall regex will only
    /// // match when these capture groups match.
    /// assert_eq!(&amp;caps[0], "'Citizen Kane' (1941)");
    /// assert_eq!(&amp;caps[1], "Citizen Kane");
    /// assert_eq!(&amp;caps[2], "1941");
    /// ```
    ///
    /// Note that the full match is at capture group `0`. Each subsequent
    /// capture group is indexed by the order of its opening `(`.
    ///
    /// We can make this example a bit clearer by using *named* capture groups:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"'(?&lt;title&gt;[^']+)'\s+\((?&lt;year&gt;\d{4})\)").unwrap();
    /// let hay = "Not my favorite movie: 'Citizen Kane' (1941).";
    /// let caps = re.captures(hay).unwrap();
    /// assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");
    /// assert_eq!(caps.name("title").unwrap().as_str(), "Citizen Kane");
    /// assert_eq!(caps.name("year").unwrap().as_str(), "1941");
    /// // You can also access the groups by name using the Index notation.
    /// // Note that this will panic on an invalid group name. In this case,
    /// // these accesses are always correct because the overall regex will
    /// // only match when these capture groups match.
    /// assert_eq!(&amp;caps[0], "'Citizen Kane' (1941)");
    /// assert_eq!(&amp;caps["title"], "Citizen Kane");
    /// assert_eq!(&amp;caps["year"], "1941");
    /// ```
    ///
    /// Here we name the capture groups, which we can access with the `name`
    /// method or the `Index` notation with a `&amp;str`. Note that the named
    /// capture groups are still accessible with `get` or the `Index` notation
    /// with a `usize`.
    ///
    /// The `0`th capture group is always unnamed, so it must always be
    /// accessed with `get(0)` or `[0]`.
    ///
    /// Finally, one other way to to get the matched substrings is with the
    /// [`Captures::extract`] API:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    /// let hay = "Not my favorite movie: 'Citizen Kane' (1941).";
    /// let (full, [title, year]) = re.captures(hay).unwrap().extract();
    /// assert_eq!(full, "'Citizen Kane' (1941)");
    /// assert_eq!(title, "Citizen Kane");
    /// assert_eq!(year, "1941");
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>captures&lt;<span class="lifetime">'h</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str) -&gt; <span class="prelude-ty">Option</span>&lt;Captures&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="self">self</span>.captures_at(haystack, <span class="number">0</span>)
    }

    <span class="doccomment">/// Returns an iterator that yields successive non-overlapping matches in
    /// the given haystack. The iterator yields values of type [`Captures`].
    ///
    /// This is the same as [`Regex::find_iter`], but instead of only providing
    /// access to the overall match, each value yield includes access to the
    /// matches of all capture groups in the regex. Reporting this extra match
    /// data is potentially costly, so callers should only use `captures_iter`
    /// over `find_iter` when they actually need access to the capture group
    /// matches.
    ///
    /// # Time complexity
    ///
    /// Note that since `captures_iter` runs potentially many searches on the
    /// haystack and since each search has worst case `O(m * n)` time
    /// complexity, the overall worst case time complexity for iteration is
    /// `O(m * n^2)`.
    ///
    /// # Example
    ///
    /// We can use this to find all movie titles and their release years in
    /// some haystack, where the movie is formatted like "'Title' (xxxx)":
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    /// let hay = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    /// let mut movies = vec![];
    /// for (_, [title, year]) in re.captures_iter(hay).map(|c| c.extract()) {
    ///     movies.push((title, year.parse::&lt;i64&gt;()?));
    /// }
    /// assert_eq!(movies, vec![
    ///     ("Citizen Kane", 1941),
    ///     ("The Wizard of Oz", 1939),
    ///     ("M", 1931),
    /// ]);
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    ///
    /// Or with named groups:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"'(?&lt;title&gt;[^']+)'\s+\((?&lt;year&gt;[0-9]{4})\)").unwrap();
    /// let hay = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    /// let mut it = re.captures_iter(hay);
    ///
    /// let caps = it.next().unwrap();
    /// assert_eq!(&amp;caps["title"], "Citizen Kane");
    /// assert_eq!(&amp;caps["year"], "1941");
    ///
    /// let caps = it.next().unwrap();
    /// assert_eq!(&amp;caps["title"], "The Wizard of Oz");
    /// assert_eq!(&amp;caps["year"], "1939");
    ///
    /// let caps = it.next().unwrap();
    /// assert_eq!(&amp;caps["title"], "M");
    /// assert_eq!(&amp;caps["year"], "1931");
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>captures_iter&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt;(
        <span class="kw-2">&amp;</span><span class="lifetime">'r </span><span class="self">self</span>,
        haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
    ) -&gt; CaptureMatches&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
        CaptureMatches { haystack, it: <span class="self">self</span>.meta.captures_iter(haystack) }
    }

    <span class="doccomment">/// Returns an iterator of substrings of the haystack given, delimited by a
    /// match of the regex. Namely, each element of the iterator corresponds to
    /// a part of the haystack that *isn't* matched by the regular expression.
    ///
    /// # Time complexity
    ///
    /// Since iterators over all matches requires running potentially many
    /// searches on the haystack, and since each search has worst case
    /// `O(m * n)` time complexity, the overall worst case time complexity for
    /// this routine is `O(m * n^2)`.
    ///
    /// # Example
    ///
    /// To split a string delimited by arbitrary amounts of spaces or tabs:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"[ \t]+").unwrap();
    /// let hay = "a b \t  c\td    e";
    /// let fields: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(fields, vec!["a", "b", "c", "d", "e"]);
    /// ```
    ///
    /// # Example: more cases
    ///
    /// Basic usage:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r" ").unwrap();
    /// let hay = "Mary had a little lamb";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(got, vec!["Mary", "had", "a", "little", "lamb"]);
    ///
    /// let re = Regex::new(r"X").unwrap();
    /// let hay = "";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(got, vec![""]);
    ///
    /// let re = Regex::new(r"X").unwrap();
    /// let hay = "lionXXtigerXleopard";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(got, vec!["lion", "", "tiger", "leopard"]);
    ///
    /// let re = Regex::new(r"::").unwrap();
    /// let hay = "lion::tiger::leopard";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(got, vec!["lion", "tiger", "leopard"]);
    /// ```
    ///
    /// If a haystack contains multiple contiguous matches, you will end up
    /// with empty spans yielded by the iterator:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"X").unwrap();
    /// let hay = "XXXXaXXbXc";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(got, vec!["", "", "", "", "a", "", "b", "c"]);
    ///
    /// let re = Regex::new(r"/").unwrap();
    /// let hay = "(///)";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(got, vec!["(", "", "", ")"]);
    /// ```
    ///
    /// Separators at the start or end of a haystack are neighbored by empty
    /// substring.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"0").unwrap();
    /// let hay = "010";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(got, vec!["", "1", ""]);
    /// ```
    ///
    /// When the empty string is used as a regex, it splits at every valid
    /// UTF-8 boundary by default (which includes the beginning and end of the
    /// haystack):
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"").unwrap();
    /// let hay = "rust";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(got, vec!["", "r", "u", "s", "t", ""]);
    ///
    /// // Splitting by an empty string is UTF-8 aware by default!
    /// let re = Regex::new(r"").unwrap();
    /// let hay = "☃";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(got, vec!["", "☃", ""]);
    /// ```
    ///
    /// Contiguous separators (commonly shows up with whitespace), can lead to
    /// possibly surprising behavior. For example, this code is correct:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r" ").unwrap();
    /// let hay = "    a  b c";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// assert_eq!(got, vec!["", "", "", "", "a", "", "b", "c"]);
    /// ```
    ///
    /// It does *not* give you `["a", "b", "c"]`. For that behavior, you'd want
    /// to match contiguous space characters:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r" +").unwrap();
    /// let hay = "    a  b c";
    /// let got: Vec&lt;&amp;str&gt; = re.split(hay).collect();
    /// // N.B. This does still include a leading empty span because ' +'
    /// // matches at the beginning of the haystack.
    /// assert_eq!(got, vec!["", "a", "b", "c"]);
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>split&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'r </span><span class="self">self</span>, haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str) -&gt; Split&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
        Split { haystack, it: <span class="self">self</span>.meta.split(haystack) }
    }

    <span class="doccomment">/// Returns an iterator of at most `limit` substrings of the haystack
    /// given, delimited by a match of the regex. (A `limit` of `0` will return
    /// no substrings.) Namely, each element of the iterator corresponds to a
    /// part of the haystack that *isn't* matched by the regular expression.
    /// The remainder of the haystack that is not split will be the last
    /// element in the iterator.
    ///
    /// # Time complexity
    ///
    /// Since iterators over all matches requires running potentially many
    /// searches on the haystack, and since each search has worst case
    /// `O(m * n)` time complexity, the overall worst case time complexity for
    /// this routine is `O(m * n^2)`.
    ///
    /// Although note that the worst case time here has an upper bound given
    /// by the `limit` parameter.
    ///
    /// # Example
    ///
    /// Get the first two words in some haystack:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"\W+").unwrap();
    /// let hay = "Hey! How are you?";
    /// let fields: Vec&lt;&amp;str&gt; = re.splitn(hay, 3).collect();
    /// assert_eq!(fields, vec!["Hey", "How", "are you?"]);
    /// ```
    ///
    /// # Examples: more cases
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r" ").unwrap();
    /// let hay = "Mary had a little lamb";
    /// let got: Vec&lt;&amp;str&gt; = re.splitn(hay, 3).collect();
    /// assert_eq!(got, vec!["Mary", "had", "a little lamb"]);
    ///
    /// let re = Regex::new(r"X").unwrap();
    /// let hay = "";
    /// let got: Vec&lt;&amp;str&gt; = re.splitn(hay, 3).collect();
    /// assert_eq!(got, vec![""]);
    ///
    /// let re = Regex::new(r"X").unwrap();
    /// let hay = "lionXXtigerXleopard";
    /// let got: Vec&lt;&amp;str&gt; = re.splitn(hay, 3).collect();
    /// assert_eq!(got, vec!["lion", "", "tigerXleopard"]);
    ///
    /// let re = Regex::new(r"::").unwrap();
    /// let hay = "lion::tiger::leopard";
    /// let got: Vec&lt;&amp;str&gt; = re.splitn(hay, 2).collect();
    /// assert_eq!(got, vec!["lion", "tiger::leopard"]);
    ///
    /// let re = Regex::new(r"X").unwrap();
    /// let hay = "abcXdef";
    /// let got: Vec&lt;&amp;str&gt; = re.splitn(hay, 1).collect();
    /// assert_eq!(got, vec!["abcXdef"]);
    ///
    /// let re = Regex::new(r"X").unwrap();
    /// let hay = "abcdef";
    /// let got: Vec&lt;&amp;str&gt; = re.splitn(hay, 2).collect();
    /// assert_eq!(got, vec!["abcdef"]);
    ///
    /// let re = Regex::new(r"X").unwrap();
    /// let hay = "abcXdef";
    /// let got: Vec&lt;&amp;str&gt; = re.splitn(hay, 0).collect();
    /// assert!(got.is_empty());
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>splitn&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt;(
        <span class="kw-2">&amp;</span><span class="lifetime">'r </span><span class="self">self</span>,
        haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
        limit: usize,
    ) -&gt; SplitN&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
        SplitN { haystack, it: <span class="self">self</span>.meta.splitn(haystack, limit) }
    }

    <span class="doccomment">/// Replaces the leftmost-first match in the given haystack with the
    /// replacement provided. The replacement can be a regular string (where
    /// `$N` and `$name` are expanded to match capture groups) or a function
    /// that takes a [`Captures`] and returns the replaced string.
    ///
    /// If no match is found, then the haystack is returned unchanged. In that
    /// case, this implementation will likely return a `Cow::Borrowed` value
    /// such that no allocation is performed.
    ///
    /// When a `Cow::Borrowed` is returned, the value returned is guaranteed
    /// to be equivalent to the `haystack` given.
    ///
    /// # Replacement string syntax
    ///
    /// All instances of `$ref` in the replacement string are replaced with
    /// the substring corresponding to the capture group identified by `ref`.
    ///
    /// `ref` may be an integer corresponding to the index of the capture group
    /// (counted by order of opening parenthesis where `0` is the entire match)
    /// or it can be a name (consisting of letters, digits or underscores)
    /// corresponding to a named capture group.
    ///
    /// If `ref` isn't a valid capture group (whether the name doesn't exist or
    /// isn't a valid index), then it is replaced with the empty string.
    ///
    /// The longest possible name is used. For example, `$1a` looks up the
    /// capture group named `1a` and not the capture group at index `1`. To
    /// exert more precise control over the name, use braces, e.g., `${1}a`.
    ///
    /// To write a literal `$` use `$$`.
    ///
    /// # Example
    ///
    /// Note that this function is polymorphic with respect to the replacement.
    /// In typical usage, this can just be a normal string:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"[^01]+").unwrap();
    /// assert_eq!(re.replace("1078910", ""), "1010");
    /// ```
    ///
    /// But anything satisfying the [`Replacer`] trait will work. For example,
    /// a closure of type `|&amp;Captures| -&gt; String` provides direct access to the
    /// captures corresponding to a match. This allows one to access capturing
    /// group matches easily:
    ///
    /// ```
    /// use regex::{Captures, Regex};
    ///
    /// let re = Regex::new(r"([^,\s]+),\s+(\S+)").unwrap();
    /// let result = re.replace("Springsteen, Bruce", |caps: &amp;Captures| {
    ///     format!("{} {}", &amp;caps[2], &amp;caps[1])
    /// });
    /// assert_eq!(result, "Bruce Springsteen");
    /// ```
    ///
    /// But this is a bit cumbersome to use all the time. Instead, a simple
    /// syntax is supported (as described above) that expands `$name` into the
    /// corresponding capture group. Here's the last example, but using this
    /// expansion technique with named capture groups:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(?&lt;last&gt;[^,\s]+),\s+(?&lt;first&gt;\S+)").unwrap();
    /// let result = re.replace("Springsteen, Bruce", "$first $last");
    /// assert_eq!(result, "Bruce Springsteen");
    /// ```
    ///
    /// Note that using `$2` instead of `$first` or `$1` instead of `$last`
    /// would produce the same result. To write a literal `$` use `$$`.
    ///
    /// Sometimes the replacement string requires use of curly braces to
    /// delineate a capture group replacement when it is adjacent to some other
    /// literal text. For example, if we wanted to join two words together with
    /// an underscore:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(?&lt;first&gt;\w+)\s+(?&lt;second&gt;\w+)").unwrap();
    /// let result = re.replace("deep fried", "${first}_$second");
    /// assert_eq!(result, "deep_fried");
    /// ```
    ///
    /// Without the curly braces, the capture group name `first_` would be
    /// used, and since it doesn't exist, it would be replaced with the empty
    /// string.
    ///
    /// Finally, sometimes you just want to replace a literal string with no
    /// regard for capturing group expansion. This can be done by wrapping a
    /// string with [`NoExpand`]:
    ///
    /// ```
    /// use regex::{NoExpand, Regex};
    ///
    /// let re = Regex::new(r"(?&lt;last&gt;[^,\s]+),\s+(\S+)").unwrap();
    /// let result = re.replace("Springsteen, Bruce", NoExpand("$2 $last"));
    /// assert_eq!(result, "$2 $last");
    /// ```
    ///
    /// Using `NoExpand` may also be faster, since the replacement string won't
    /// need to be parsed for the `$` syntax.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>replace&lt;<span class="lifetime">'h</span>, R: Replacer&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
        rep: R,
    ) -&gt; Cow&lt;<span class="lifetime">'h</span>, str&gt; {
        <span class="self">self</span>.replacen(haystack, <span class="number">1</span>, rep)
    }

    <span class="doccomment">/// Replaces all non-overlapping matches in the haystack with the
    /// replacement provided. This is the same as calling `replacen` with
    /// `limit` set to `0`.
    ///
    /// If no match is found, then the haystack is returned unchanged. In that
    /// case, this implementation will likely return a `Cow::Borrowed` value
    /// such that no allocation is performed.
    ///
    /// When a `Cow::Borrowed` is returned, the value returned is guaranteed
    /// to be equivalent to the `haystack` given.
    ///
    /// The documentation for [`Regex::replace`] goes into more detail about
    /// what kinds of replacement strings are supported.
    ///
    /// # Time complexity
    ///
    /// Since iterators over all matches requires running potentially many
    /// searches on the haystack, and since each search has worst case
    /// `O(m * n)` time complexity, the overall worst case time complexity for
    /// this routine is `O(m * n^2)`.
    ///
    /// # Fallibility
    ///
    /// If you need to write a replacement routine where any individual
    /// replacement might "fail," doing so with this API isn't really feasible
    /// because there's no way to stop the search process if a replacement
    /// fails. Instead, if you need this functionality, you should consider
    /// implementing your own replacement routine:
    ///
    /// ```
    /// use regex::{Captures, Regex};
    ///
    /// fn replace_all&lt;E&gt;(
    ///     re: &amp;Regex,
    ///     haystack: &amp;str,
    ///     replacement: impl Fn(&amp;Captures) -&gt; Result&lt;String, E&gt;,
    /// ) -&gt; Result&lt;String, E&gt; {
    ///     let mut new = String::with_capacity(haystack.len());
    ///     let mut last_match = 0;
    ///     for caps in re.captures_iter(haystack) {
    ///         let m = caps.get(0).unwrap();
    ///         new.push_str(&amp;haystack[last_match..m.start()]);
    ///         new.push_str(&amp;replacement(&amp;caps)?);
    ///         last_match = m.end();
    ///     }
    ///     new.push_str(&amp;haystack[last_match..]);
    ///     Ok(new)
    /// }
    ///
    /// // Let's replace each word with the number of bytes in that word.
    /// // But if we see a word that is "too long," we'll give up.
    /// let re = Regex::new(r"\w+").unwrap();
    /// let replacement = |caps: &amp;Captures| -&gt; Result&lt;String, &amp;'static str&gt; {
    ///     if caps[0].len() &gt;= 5 {
    ///         return Err("word too long");
    ///     }
    ///     Ok(caps[0].len().to_string())
    /// };
    /// assert_eq!(
    ///     Ok("2 3 3 3?".to_string()),
    ///     replace_all(&amp;re, "hi how are you?", &amp;replacement),
    /// );
    /// assert!(replace_all(&amp;re, "hi there", &amp;replacement).is_err());
    /// ```
    ///
    /// # Example
    ///
    /// This example shows how to flip the order of whitespace (excluding line
    /// terminators) delimited fields, and normalizes the whitespace that
    /// delimits the fields:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(?m)^(\S+)[\s--\r\n]+(\S+)$").unwrap();
    /// let hay = "
    /// Greetings  1973
    /// Wild\t1973
    /// BornToRun\t\t\t\t1975
    /// Darkness                    1978
    /// TheRiver 1980
    /// ";
    /// let new = re.replace_all(hay, "$2 $1");
    /// assert_eq!(new, "
    /// 1973 Greetings
    /// 1973 Wild
    /// 1975 BornToRun
    /// 1978 Darkness
    /// 1980 TheRiver
    /// ");
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>replace_all&lt;<span class="lifetime">'h</span>, R: Replacer&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
        rep: R,
    ) -&gt; Cow&lt;<span class="lifetime">'h</span>, str&gt; {
        <span class="self">self</span>.replacen(haystack, <span class="number">0</span>, rep)
    }

    <span class="doccomment">/// Replaces at most `limit` non-overlapping matches in the haystack with
    /// the replacement provided. If `limit` is `0`, then all non-overlapping
    /// matches are replaced. That is, `Regex::replace_all(hay, rep)` is
    /// equivalent to `Regex::replacen(hay, 0, rep)`.
    ///
    /// If no match is found, then the haystack is returned unchanged. In that
    /// case, this implementation will likely return a `Cow::Borrowed` value
    /// such that no allocation is performed.
    ///
    /// When a `Cow::Borrowed` is returned, the value returned is guaranteed
    /// to be equivalent to the `haystack` given.
    ///
    /// The documentation for [`Regex::replace`] goes into more detail about
    /// what kinds of replacement strings are supported.
    ///
    /// # Time complexity
    ///
    /// Since iterators over all matches requires running potentially many
    /// searches on the haystack, and since each search has worst case
    /// `O(m * n)` time complexity, the overall worst case time complexity for
    /// this routine is `O(m * n^2)`.
    ///
    /// Although note that the worst case time here has an upper bound given
    /// by the `limit` parameter.
    ///
    /// # Fallibility
    ///
    /// See the corresponding section in the docs for [`Regex::replace_all`]
    /// for tips on how to deal with a replacement routine that can fail.
    ///
    /// # Example
    ///
    /// This example shows how to flip the order of whitespace (excluding line
    /// terminators) delimited fields, and normalizes the whitespace that
    /// delimits the fields. But we only do it for the first two matches.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(?m)^(\S+)[\s--\r\n]+(\S+)$").unwrap();
    /// let hay = "
    /// Greetings  1973
    /// Wild\t1973
    /// BornToRun\t\t\t\t1975
    /// Darkness                    1978
    /// TheRiver 1980
    /// ";
    /// let new = re.replacen(hay, 2, "$2 $1");
    /// assert_eq!(new, "
    /// 1973 Greetings
    /// 1973 Wild
    /// BornToRun\t\t\t\t1975
    /// Darkness                    1978
    /// TheRiver 1980
    /// ");
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>replacen&lt;<span class="lifetime">'h</span>, R: Replacer&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
        limit: usize,
        <span class="kw-2">mut </span>rep: R,
    ) -&gt; Cow&lt;<span class="lifetime">'h</span>, str&gt; {
        <span class="comment">// If we know that the replacement doesn't have any capture expansions,
        // then we can use the fast path. The fast path can make a tremendous
        // difference:
        //
        //   1) We use `find_iter` instead of `captures_iter`. Not asking for
        //      captures generally makes the regex engines faster.
        //   2) We don't need to look up all of the capture groups and do
        //      replacements inside the replacement string. We just push it
        //      at each match and be done with it.
        </span><span class="kw">if let </span><span class="prelude-val">Some</span>(rep) = rep.no_expansion() {
            <span class="kw">let </span><span class="kw-2">mut </span>it = <span class="self">self</span>.find_iter(haystack).enumerate().peekable();
            <span class="kw">if </span>it.peek().is_none() {
                <span class="kw">return </span>Cow::Borrowed(haystack);
            }
            <span class="kw">let </span><span class="kw-2">mut </span>new = String::with_capacity(haystack.len());
            <span class="kw">let </span><span class="kw-2">mut </span>last_match = <span class="number">0</span>;
            <span class="kw">for </span>(i, m) <span class="kw">in </span>it {
                new.push_str(<span class="kw-2">&amp;</span>haystack[last_match..m.start()]);
                new.push_str(<span class="kw-2">&amp;</span>rep);
                last_match = m.end();
                <span class="kw">if </span>limit &gt; <span class="number">0 </span>&amp;&amp; i &gt;= limit - <span class="number">1 </span>{
                    <span class="kw">break</span>;
                }
            }
            new.push_str(<span class="kw-2">&amp;</span>haystack[last_match..]);
            <span class="kw">return </span>Cow::Owned(new);
        }

        <span class="comment">// The slower path, which we use if the replacement may need access to
        // capture groups.
        </span><span class="kw">let </span><span class="kw-2">mut </span>it = <span class="self">self</span>.captures_iter(haystack).enumerate().peekable();
        <span class="kw">if </span>it.peek().is_none() {
            <span class="kw">return </span>Cow::Borrowed(haystack);
        }
        <span class="kw">let </span><span class="kw-2">mut </span>new = String::with_capacity(haystack.len());
        <span class="kw">let </span><span class="kw-2">mut </span>last_match = <span class="number">0</span>;
        <span class="kw">for </span>(i, cap) <span class="kw">in </span>it {
            <span class="comment">// unwrap on 0 is OK because captures only reports matches
            </span><span class="kw">let </span>m = cap.get(<span class="number">0</span>).unwrap();
            new.push_str(<span class="kw-2">&amp;</span>haystack[last_match..m.start()]);
            rep.replace_append(<span class="kw-2">&amp;</span>cap, <span class="kw-2">&amp;mut </span>new);
            last_match = m.end();
            <span class="kw">if </span>limit &gt; <span class="number">0 </span>&amp;&amp; i &gt;= limit - <span class="number">1 </span>{
                <span class="kw">break</span>;
            }
        }
        new.push_str(<span class="kw-2">&amp;</span>haystack[last_match..]);
        Cow::Owned(new)
    }
}

<span class="doccomment">/// A group of advanced or "lower level" search methods. Some methods permit
/// starting the search at a position greater than `0` in the haystack. Other
/// methods permit reusing allocations, for example, when extracting the
/// matches for capture groups.
</span><span class="kw">impl </span>Regex {
    <span class="doccomment">/// Returns the end byte offset of the first match in the haystack given.
    ///
    /// This method may have the same performance characteristics as
    /// `is_match`. Behaviorlly, it doesn't just report whether it match
    /// occurs, but also the end offset for a match. In particular, the offset
    /// returned *may be shorter* than the proper end of the leftmost-first
    /// match that you would find via [`Regex::find`].
    ///
    /// Note that it is not guaranteed that this routine finds the shortest or
    /// "earliest" possible match. Instead, the main idea of this API is that
    /// it returns the offset at the point at which the internal regex engine
    /// has determined that a match has occurred. This may vary depending on
    /// which internal regex engine is used, and thus, the offset itself may
    /// change based on internal heuristics.
    ///
    /// # Example
    ///
    /// Typically, `a+` would match the entire first sequence of `a` in some
    /// haystack, but `shortest_match` *may* give up as soon as it sees the
    /// first `a`.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"a+").unwrap();
    /// let offset = re.shortest_match("aaaaa").unwrap();
    /// assert_eq!(offset, 1);
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>shortest_match(<span class="kw-2">&amp;</span><span class="self">self</span>, haystack: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="self">self</span>.shortest_match_at(haystack, <span class="number">0</span>)
    }

    <span class="doccomment">/// Returns the same as [`Regex::shortest_match`], but starts the search at
    /// the given offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only match
    /// when `start == 0`.
    ///
    /// If a match is found, the offset returned is relative to the beginning
    /// of the haystack, not the beginning of the search.
    ///
    /// # Panics
    ///
    /// This panics when `start &gt;= haystack.len() + 1`.
    ///
    /// # Example
    ///
    /// This example shows the significance of `start` by demonstrating how it
    /// can be used to permit look-around assertions in a regex to take the
    /// surrounding context into account.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"\bchew\b").unwrap();
    /// let hay = "eschew";
    /// // We get a match here, but it's probably not intended.
    /// assert_eq!(re.shortest_match(&amp;hay[2..]), Some(4));
    /// // No match because the  assertions take the context into account.
    /// assert_eq!(re.shortest_match_at(hay, 2), None);
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>shortest_match_at(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        haystack: <span class="kw-2">&amp;</span>str,
        start: usize,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="kw">let </span>input =
            Input::new(haystack).earliest(<span class="bool-val">true</span>).span(start..haystack.len());
        <span class="self">self</span>.meta.search_half(<span class="kw-2">&amp;</span>input).map(|hm| hm.offset())
    }

    <span class="doccomment">/// Returns the same as [`Regex::is_match`], but starts the search at the
    /// given offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    ///
    /// # Panics
    ///
    /// This panics when `start &gt;= haystack.len() + 1`.
    ///
    /// # Example
    ///
    /// This example shows the significance of `start` by demonstrating how it
    /// can be used to permit look-around assertions in a regex to take the
    /// surrounding context into account.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"\bchew\b").unwrap();
    /// let hay = "eschew";
    /// // We get a match here, but it's probably not intended.
    /// assert!(re.is_match(&amp;hay[2..]));
    /// // No match because the  assertions take the context into account.
    /// assert!(!re.is_match_at(hay, 2));
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_match_at(<span class="kw-2">&amp;</span><span class="self">self</span>, haystack: <span class="kw-2">&amp;</span>str, start: usize) -&gt; bool {
        <span class="kw">let </span>input =
            Input::new(haystack).earliest(<span class="bool-val">true</span>).span(start..haystack.len());
        <span class="self">self</span>.meta.search_half(<span class="kw-2">&amp;</span>input).is_some()
    }

    <span class="doccomment">/// Returns the same as [`Regex::find`], but starts the search at the given
    /// offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    ///
    /// # Panics
    ///
    /// This panics when `start &gt;= haystack.len() + 1`.
    ///
    /// # Example
    ///
    /// This example shows the significance of `start` by demonstrating how it
    /// can be used to permit look-around assertions in a regex to take the
    /// surrounding context into account.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"\bchew\b").unwrap();
    /// let hay = "eschew";
    /// // We get a match here, but it's probably not intended.
    /// assert_eq!(re.find(&amp;hay[2..]).map(|m| m.range()), Some(0..4));
    /// // No match because the  assertions take the context into account.
    /// assert_eq!(re.find_at(hay, 2), None);
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>find_at&lt;<span class="lifetime">'h</span>&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
        start: usize,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="kw">let </span>input = Input::new(haystack).span(start..haystack.len());
        <span class="self">self</span>.meta
            .search(<span class="kw-2">&amp;</span>input)
            .map(|m| Match::new(haystack, m.start(), m.end()))
    }

    <span class="doccomment">/// Returns the same as [`Regex::captures`], but starts the search at the
    /// given offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    ///
    /// # Panics
    ///
    /// This panics when `start &gt;= haystack.len() + 1`.
    ///
    /// # Example
    ///
    /// This example shows the significance of `start` by demonstrating how it
    /// can be used to permit look-around assertions in a regex to take the
    /// surrounding context into account.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"\bchew\b").unwrap();
    /// let hay = "eschew";
    /// // We get a match here, but it's probably not intended.
    /// assert_eq!(&amp;re.captures(&amp;hay[2..]).unwrap()[0], "chew");
    /// // No match because the  assertions take the context into account.
    /// assert!(re.captures_at(hay, 2).is_none());
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>captures_at&lt;<span class="lifetime">'h</span>&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
        start: usize,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;Captures&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="kw">let </span>input = Input::new(haystack).span(start..haystack.len());
        <span class="kw">let </span><span class="kw-2">mut </span>caps = <span class="self">self</span>.meta.create_captures();
        <span class="self">self</span>.meta.search_captures(<span class="kw-2">&amp;</span>input, <span class="kw-2">&amp;mut </span>caps);
        <span class="kw">if </span>caps.is_match() {
            <span class="kw">let </span>static_captures_len = <span class="self">self</span>.static_captures_len();
            <span class="prelude-val">Some</span>(Captures { haystack, caps, static_captures_len })
        } <span class="kw">else </span>{
            <span class="prelude-val">None
        </span>}
    }

    <span class="doccomment">/// This is like [`Regex::captures`], but writes the byte offsets of each
    /// capture group match into the locations given.
    ///
    /// A [`CaptureLocations`] stores the same byte offsets as a [`Captures`],
    /// but does *not* store a reference to the haystack. This makes its API
    /// a bit lower level and less convenient. But in exchange, callers
    /// may allocate their own `CaptureLocations` and reuse it for multiple
    /// searches. This may be helpful if allocating a `Captures` shows up in a
    /// profile as too costly.
    ///
    /// To create a `CaptureLocations` value, use the
    /// [`Regex::capture_locations`] method.
    ///
    /// This also returns the overall match if one was found. When a match is
    /// found, its offsets are also always stored in `locs` at index `0`.
    ///
    /// # Panics
    ///
    /// This routine may panic if the given `CaptureLocations` was not created
    /// by this regex.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"^([a-z]+)=(\S*)$").unwrap();
    /// let mut locs = re.capture_locations();
    /// assert!(re.captures_read(&amp;mut locs, "id=foo123").is_some());
    /// assert_eq!(Some((0, 9)), locs.get(0));
    /// assert_eq!(Some((0, 2)), locs.get(1));
    /// assert_eq!(Some((3, 9)), locs.get(2));
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>captures_read&lt;<span class="lifetime">'h</span>&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        locs: <span class="kw-2">&amp;mut </span>CaptureLocations,
        haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="self">self</span>.captures_read_at(locs, haystack, <span class="number">0</span>)
    }

    <span class="doccomment">/// Returns the same as [`Regex::captures_read`], but starts the search at
    /// the given offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    ///
    /// # Panics
    ///
    /// This panics when `start &gt;= haystack.len() + 1`.
    ///
    /// This routine may also panic if the given `CaptureLocations` was not
    /// created by this regex.
    ///
    /// # Example
    ///
    /// This example shows the significance of `start` by demonstrating how it
    /// can be used to permit look-around assertions in a regex to take the
    /// surrounding context into account.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"\bchew\b").unwrap();
    /// let hay = "eschew";
    /// let mut locs = re.capture_locations();
    /// // We get a match here, but it's probably not intended.
    /// assert!(re.captures_read(&amp;mut locs, &amp;hay[2..]).is_some());
    /// // No match because the  assertions take the context into account.
    /// assert!(re.captures_read_at(&amp;mut locs, hay, 2).is_none());
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>captures_read_at&lt;<span class="lifetime">'h</span>&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        locs: <span class="kw-2">&amp;mut </span>CaptureLocations,
        haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
        start: usize,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="kw">let </span>input = Input::new(haystack).span(start..haystack.len());
        <span class="self">self</span>.meta.search_captures(<span class="kw-2">&amp;</span>input, <span class="kw-2">&amp;mut </span>locs.<span class="number">0</span>);
        locs.<span class="number">0</span>.get_match().map(|m| Match::new(haystack, m.start(), m.end()))
    }

    <span class="doccomment">/// An undocumented alias for `captures_read_at`.
    ///
    /// The `regex-capi` crate previously used this routine, so to avoid
    /// breaking that crate, we continue to provide the name as an undocumented
    /// alias.
    </span><span class="attr">#[doc(hidden)]
    #[inline]
    </span><span class="kw">pub fn </span>read_captures_at&lt;<span class="lifetime">'h</span>&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        locs: <span class="kw-2">&amp;mut </span>CaptureLocations,
        haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
        start: usize,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="self">self</span>.captures_read_at(locs, haystack, start)
    }
}

<span class="doccomment">/// Auxiliary methods.
</span><span class="kw">impl </span>Regex {
    <span class="doccomment">/// Returns the original string of this regex.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"foo\w+bar").unwrap();
    /// assert_eq!(re.as_str(), r"foo\w+bar");
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>as_str(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>str {
        <span class="kw-2">&amp;</span><span class="self">self</span>.pattern
    }

    <span class="doccomment">/// Returns an iterator over the capture names in this regex.
    ///
    /// The iterator returned yields elements of type `Option&lt;&amp;str&gt;`. That is,
    /// the iterator yields values for all capture groups, even ones that are
    /// unnamed. The order of the groups corresponds to the order of the group's
    /// corresponding opening parenthesis.
    ///
    /// The first element of the iterator always yields the group corresponding
    /// to the overall match, and this group is always unnamed. Therefore, the
    /// iterator always yields at least one group.
    ///
    /// # Example
    ///
    /// This shows basic usage with a mix of named and unnamed capture groups:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(?&lt;a&gt;.(?&lt;b&gt;.))(.)(?:.)(?&lt;c&gt;.)").unwrap();
    /// let mut names = re.capture_names();
    /// assert_eq!(names.next(), Some(None));
    /// assert_eq!(names.next(), Some(Some("a")));
    /// assert_eq!(names.next(), Some(Some("b")));
    /// assert_eq!(names.next(), Some(None));
    /// // the '(?:.)' group is non-capturing and so doesn't appear here!
    /// assert_eq!(names.next(), Some(Some("c")));
    /// assert_eq!(names.next(), None);
    /// ```
    ///
    /// The iterator always yields at least one element, even for regexes with
    /// no capture groups and even for regexes that can never match:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"").unwrap();
    /// let mut names = re.capture_names();
    /// assert_eq!(names.next(), Some(None));
    /// assert_eq!(names.next(), None);
    ///
    /// let re = Regex::new(r"[a&amp;&amp;b]").unwrap();
    /// let mut names = re.capture_names();
    /// assert_eq!(names.next(), Some(None));
    /// assert_eq!(names.next(), None);
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>capture_names(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; CaptureNames&lt;<span class="lifetime">'_</span>&gt; {
        CaptureNames(<span class="self">self</span>.meta.group_info().pattern_names(PatternID::ZERO))
    }

    <span class="doccomment">/// Returns the number of captures groups in this regex.
    ///
    /// This includes all named and unnamed groups, including the implicit
    /// unnamed group that is always present and corresponds to the entire
    /// match.
    ///
    /// Since the implicit unnamed group is always included in this length, the
    /// length returned is guaranteed to be greater than zero.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"foo").unwrap();
    /// assert_eq!(1, re.captures_len());
    ///
    /// let re = Regex::new(r"(foo)").unwrap();
    /// assert_eq!(2, re.captures_len());
    ///
    /// let re = Regex::new(r"(?&lt;a&gt;.(?&lt;b&gt;.))(.)(?:.)(?&lt;c&gt;.)").unwrap();
    /// assert_eq!(5, re.captures_len());
    ///
    /// let re = Regex::new(r"[a&amp;&amp;b]").unwrap();
    /// assert_eq!(1, re.captures_len());
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>captures_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.meta.group_info().group_len(PatternID::ZERO)
    }

    <span class="doccomment">/// Returns the total number of capturing groups that appear in every
    /// possible match.
    ///
    /// If the number of capture groups can vary depending on the match, then
    /// this returns `None`. That is, a value is only returned when the number
    /// of matching groups is invariant or "static."
    ///
    /// Note that like [`Regex::captures_len`], this **does** include the
    /// implicit capturing group corresponding to the entire match. Therefore,
    /// when a non-None value is returned, it is guaranteed to be at least `1`.
    /// Stated differently, a return value of `Some(0)` is impossible.
    ///
    /// # Example
    ///
    /// This shows a few cases where a static number of capture groups is
    /// available and a few cases where it is not.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let len = |pattern| {
    ///     Regex::new(pattern).map(|re| re.static_captures_len())
    /// };
    ///
    /// assert_eq!(Some(1), len("a")?);
    /// assert_eq!(Some(2), len("(a)")?);
    /// assert_eq!(Some(2), len("(a)|(b)")?);
    /// assert_eq!(Some(3), len("(a)(b)|(c)(d)")?);
    /// assert_eq!(None, len("(a)|b")?);
    /// assert_eq!(None, len("a|(b)")?);
    /// assert_eq!(None, len("(b)*")?);
    /// assert_eq!(Some(2), len("(b)+")?);
    ///
    /// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>static_captures_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="self">self</span>.meta.static_captures_len()
    }

    <span class="doccomment">/// Returns a fresh allocated set of capture locations that can
    /// be reused in multiple calls to [`Regex::captures_read`] or
    /// [`Regex::captures_read_at`].
    ///
    /// The returned locations can be used for any subsequent search for this
    /// particular regex. There is no guarantee that it is correct to use for
    /// other regexes, even if they have the same number of capture groups.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(.)(.)(\w+)").unwrap();
    /// let mut locs = re.capture_locations();
    /// assert!(re.captures_read(&amp;mut locs, "Padron").is_some());
    /// assert_eq!(locs.get(0), Some((0, 6)));
    /// assert_eq!(locs.get(1), Some((0, 1)));
    /// assert_eq!(locs.get(2), Some((1, 2)));
    /// assert_eq!(locs.get(3), Some((2, 6)));
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>capture_locations(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; CaptureLocations {
        CaptureLocations(<span class="self">self</span>.meta.create_captures())
    }

    <span class="doccomment">/// An alias for `capture_locations` to preserve backward compatibility.
    ///
    /// The `regex-capi` crate used this method, so to avoid breaking that
    /// crate, we continue to export it as an undocumented API.
    </span><span class="attr">#[doc(hidden)]
    #[inline]
    </span><span class="kw">pub fn </span>locations(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; CaptureLocations {
        <span class="self">self</span>.capture_locations()
    }
}

<span class="doccomment">/// Represents a single match of a regex in a haystack.
///
/// A `Match` contains both the start and end byte offsets of the match and the
/// actual substring corresponding to the range of those byte offsets. It is
/// guaranteed that `start &lt;= end`. When `start == end`, the match is empty.
///
/// Since this `Match` can only be produced by the top-level `Regex` APIs
/// that only support searching UTF-8 encoded strings, the byte offsets for a
/// `Match` are guaranteed to fall on valid UTF-8 codepoint boundaries. That
/// is, slicing a `&amp;str` with [`Match::range`] is guaranteed to never panic.
///
/// Values with this type are created by [`Regex::find`] or
/// [`Regex::find_iter`]. Other APIs can create `Match` values too. For
/// example, [`Captures::get`].
///
/// The lifetime parameter `'h` refers to the lifetime of the matched of the
/// haystack that this match was produced from.
///
/// # Numbering
///
/// The byte offsets in a `Match` form a half-open interval. That is, the
/// start of the range is inclusive and the end of the range is exclusive.
/// For example, given a haystack `abcFOOxyz` and a match of `FOO`, its byte
/// offset range starts at `3` and ends at `6`. `3` corresponds to `F` and
/// `6` corresponds to `x`, which is one past the end of the match. This
/// corresponds to the same kind of slicing that Rust uses.
///
/// For more on why this was chosen over other schemes (aside from being
/// consistent with how Rust the language works), see [this discussion] and
/// [Dijkstra's note on a related topic][note].
///
/// [this discussion]: https://github.com/rust-lang/regex/discussions/866
/// [note]: https://www.cs.utexas.edu/users/EWD/transcriptions/EWD08xx/EWD831.html
///
/// # Example
///
/// This example shows the value of each of the methods on `Match` for a
/// particular search.
///
/// ```
/// use regex::Regex;
///
/// let re = Regex::new(r"\p{Greek}+").unwrap();
/// let hay = "Greek: αβγδ";
/// let m = re.find(hay).unwrap();
/// assert_eq!(7, m.start());
/// assert_eq!(15, m.end());
/// assert!(!m.is_empty());
/// assert_eq!(8, m.len());
/// assert_eq!(7..15, m.range());
/// assert_eq!("αβγδ", m.as_str());
/// ```
</span><span class="attr">#[derive(Copy, Clone, Eq, PartialEq)]
</span><span class="kw">pub struct </span>Match&lt;<span class="lifetime">'h</span>&gt; {
    haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
    start: usize,
    end: usize,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'h</span>&gt; Match&lt;<span class="lifetime">'h</span>&gt; {
    <span class="doccomment">/// Returns the byte offset of the start of the match in the haystack. The
    /// start of the match corresponds to the position where the match begins
    /// and includes the first byte in the match.
    ///
    /// It is guaranteed that `Match::start() &lt;= Match::end()`.
    ///
    /// This is guaranteed to fall on a valid UTF-8 codepoint boundary. That
    /// is, it will never be an offset that appears between the UTF-8 code
    /// units of a UTF-8 encoded Unicode scalar value. Consequently, it is
    /// always safe to slice the corresponding haystack using this offset.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>start(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.start
    }

    <span class="doccomment">/// Returns the byte offset of the end of the match in the haystack. The
    /// end of the match corresponds to the byte immediately following the last
    /// byte in the match. This means that `&amp;slice[start..end]` works as one
    /// would expect.
    ///
    /// It is guaranteed that `Match::start() &lt;= Match::end()`.
    ///
    /// This is guaranteed to fall on a valid UTF-8 codepoint boundary. That
    /// is, it will never be an offset that appears between the UTF-8 code
    /// units of a UTF-8 encoded Unicode scalar value. Consequently, it is
    /// always safe to slice the corresponding haystack using this offset.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>end(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.end
    }

    <span class="doccomment">/// Returns true if and only if this match has a length of zero.
    ///
    /// Note that an empty match can only occur when the regex itself can
    /// match the empty string. Here are some examples of regexes that can
    /// all match the empty string: `^`, `^$`, `\b`, `a?`, `a*`, `a{0}`,
    /// `(foo|\d+|quux)?`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_empty(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.start == <span class="self">self</span>.end
    }

    <span class="doccomment">/// Returns the length, in bytes, of this match.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.end - <span class="self">self</span>.start
    }

    <span class="doccomment">/// Returns the range over the starting and ending byte offsets of the
    /// match in the haystack.
    ///
    /// It is always correct to slice the original haystack searched with this
    /// range. That is, because the offsets are guaranteed to fall on valid
    /// UTF-8 boundaries, the range returned is always valid.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>range(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; core::ops::Range&lt;usize&gt; {
        <span class="self">self</span>.start..<span class="self">self</span>.end
    }

    <span class="doccomment">/// Returns the substring of the haystack that matched.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>as_str(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str {
        <span class="kw-2">&amp;</span><span class="self">self</span>.haystack[<span class="self">self</span>.range()]
    }

    <span class="doccomment">/// Creates a new match from the given haystack and byte offsets.
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>new(haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str, start: usize, end: usize) -&gt; Match&lt;<span class="lifetime">'h</span>&gt; {
        Match { haystack, start, end }
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'h</span>&gt; core::fmt::Debug <span class="kw">for </span>Match&lt;<span class="lifetime">'h</span>&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        f.debug_struct(<span class="string">"Match"</span>)
            .field(<span class="string">"start"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.start)
            .field(<span class="string">"end"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.end)
            .field(<span class="string">"string"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.as_str())
            .finish()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'h</span>&gt; From&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt; <span class="kw">for </span><span class="kw-2">&amp;</span><span class="lifetime">'h </span>str {
    <span class="kw">fn </span>from(m: Match&lt;<span class="lifetime">'h</span>&gt;) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str {
        m.as_str()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'h</span>&gt; From&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt; <span class="kw">for </span>core::ops::Range&lt;usize&gt; {
    <span class="kw">fn </span>from(m: Match&lt;<span class="lifetime">'h</span>&gt;) -&gt; core::ops::Range&lt;usize&gt; {
        m.range()
    }
}

<span class="doccomment">/// Represents the capture groups for a single match.
///
/// Capture groups refer to parts of a regex enclosed in parentheses. They
/// can be optionally named. The purpose of capture groups is to be able to
/// reference different parts of a match based on the original pattern. In
/// essence, a `Captures` is a container of [`Match`] values for each group
/// that participated in a regex match. Each `Match` can be looked up by either
/// its capture group index or name (if it has one).
///
/// For example, say you want to match the individual letters in a 5-letter
/// word:
///
/// ```text
/// (?&lt;first&gt;\w)(\w)(?:\w)\w(?&lt;last&gt;\w)
/// ```
///
/// This regex has 4 capture groups:
///
/// * The group at index `0` corresponds to the overall match. It is always
/// present in every match and never has a name.
/// * The group at index `1` with name `first` corresponding to the first
/// letter.
/// * The group at index `2` with no name corresponding to the second letter.
/// * The group at index `3` with name `last` corresponding to the fifth and
/// last letter.
///
/// Notice that `(?:\w)` was not listed above as a capture group despite it
/// being enclosed in parentheses. That's because `(?:pattern)` is a special
/// syntax that permits grouping but *without* capturing. The reason for not
/// treating it as a capture is that tracking and reporting capture groups
/// requires additional state that may lead to slower searches. So using as few
/// capture groups as possible can help performance. (Although the difference
/// in performance of a couple of capture groups is likely immaterial.)
///
/// Values with this type are created by [`Regex::captures`] or
/// [`Regex::captures_iter`].
///
/// `'h` is the lifetime of the haystack that these captures were matched from.
///
/// # Example
///
/// ```
/// use regex::Regex;
///
/// let re = Regex::new(r"(?&lt;first&gt;\w)(\w)(?:\w)\w(?&lt;last&gt;\w)").unwrap();
/// let caps = re.captures("toady").unwrap();
/// assert_eq!("toady", &amp;caps[0]);
/// assert_eq!("t", &amp;caps["first"]);
/// assert_eq!("o", &amp;caps[2]);
/// assert_eq!("y", &amp;caps["last"]);
/// ```
</span><span class="kw">pub struct </span>Captures&lt;<span class="lifetime">'h</span>&gt; {
    haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
    caps: captures::Captures,
    static_captures_len: <span class="prelude-ty">Option</span>&lt;usize&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'h</span>&gt; Captures&lt;<span class="lifetime">'h</span>&gt; {
    <span class="doccomment">/// Returns the `Match` associated with the capture group at index `i`. If
    /// `i` does not correspond to a capture group, or if the capture group did
    /// not participate in the match, then `None` is returned.
    ///
    /// When `i == 0`, this is guaranteed to return a non-`None` value.
    ///
    /// # Examples
    ///
    /// Get the substring that matched with a default of an empty string if the
    /// group didn't participate in the match:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    /// let caps = re.captures("abc123").unwrap();
    ///
    /// let substr1 = caps.get(1).map_or("", |m| m.as_str());
    /// let substr2 = caps.get(2).map_or("", |m| m.as_str());
    /// assert_eq!(substr1, "123");
    /// assert_eq!(substr2, "");
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>get(<span class="kw-2">&amp;</span><span class="self">self</span>, i: usize) -&gt; <span class="prelude-ty">Option</span>&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="self">self</span>.caps
            .get_group(i)
            .map(|sp| Match::new(<span class="self">self</span>.haystack, sp.start, sp.end))
    }

    <span class="doccomment">/// Returns the `Match` associated with the capture group named `name`. If
    /// `name` isn't a valid capture group or it refers to a group that didn't
    /// match, then `None` is returned.
    ///
    /// Note that unlike `caps["name"]`, this returns a `Match` whose lifetime
    /// matches the lifetime of the haystack in this `Captures` value.
    /// Conversely, the substring returned by `caps["name"]` has a lifetime
    /// of the `Captures` value, which is likely shorter than the lifetime of
    /// the haystack. In some cases, it may be necessary to use this method to
    /// access the matching substring instead of the `caps["name"]` notation.
    ///
    /// # Examples
    ///
    /// Get the substring that matched with a default of an empty string if the
    /// group didn't participate in the match:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(
    ///     r"[a-z]+(?:(?&lt;numbers&gt;[0-9]+)|(?&lt;letters&gt;[A-Z]+))",
    /// ).unwrap();
    /// let caps = re.captures("abc123").unwrap();
    ///
    /// let numbers = caps.name("numbers").map_or("", |m| m.as_str());
    /// let letters = caps.name("letters").map_or("", |m| m.as_str());
    /// assert_eq!(numbers, "123");
    /// assert_eq!(letters, "");
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>name(<span class="kw-2">&amp;</span><span class="self">self</span>, name: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Option</span>&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="self">self</span>.caps
            .get_group_by_name(name)
            .map(|sp| Match::new(<span class="self">self</span>.haystack, sp.start, sp.end))
    }

    <span class="doccomment">/// This is a convenience routine for extracting the substrings
    /// corresponding to matching capture groups.
    ///
    /// This returns a tuple where the first element corresponds to the full
    /// substring of the haystack that matched the regex. The second element is
    /// an array of substrings, with each corresponding to the to the substring
    /// that matched for a particular capture group.
    ///
    /// # Panics
    ///
    /// This panics if the number of possible matching groups in this
    /// `Captures` value is not fixed to `N` in all circumstances.
    /// More precisely, this routine only works when `N` is equivalent to
    /// [`Regex::static_captures_len`].
    ///
    /// Stated more plainly, if the number of matching capture groups in a
    /// regex can vary from match to match, then this function always panics.
    ///
    /// For example, `(a)(b)|(c)` could produce two matching capture groups
    /// or one matching capture group for any given match. Therefore, one
    /// cannot use `extract` with such a pattern.
    ///
    /// But a pattern like `(a)(b)|(c)(d)` can be used with `extract` because
    /// the number of capture groups in every match is always equivalent,
    /// even if the capture _indices_ in each match are not.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    /// let hay = "On 2010-03-14, I became a Tenneessee lamb.";
    /// let Some((full, [year, month, day])) =
    ///     re.captures(hay).map(|caps| caps.extract()) else { return };
    /// assert_eq!("2010-03-14", full);
    /// assert_eq!("2010", year);
    /// assert_eq!("03", month);
    /// assert_eq!("14", day);
    /// ```
    ///
    /// # Example: iteration
    ///
    /// This example shows how to use this method when iterating over all
    /// `Captures` matches in a haystack.
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    /// let hay = "1973-01-05, 1975-08-25 and 1980-10-18";
    ///
    /// let mut dates: Vec&lt;(&amp;str, &amp;str, &amp;str)&gt; = vec![];
    /// for (_, [y, m, d]) in re.captures_iter(hay).map(|c| c.extract()) {
    ///     dates.push((y, m, d));
    /// }
    /// assert_eq!(dates, vec![
    ///     ("1973", "01", "05"),
    ///     ("1975", "08", "25"),
    ///     ("1980", "10", "18"),
    /// ]);
    /// ```
    ///
    /// # Example: parsing different formats
    ///
    /// This API is particularly useful when you need to extract a particular
    /// value that might occur in a different format. Consider, for example,
    /// an identifier that might be in double quotes or single quotes:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r#"id:(?:"([^"]+)"|'([^']+)')"#).unwrap();
    /// let hay = r#"The first is id:"foo" and the second is id:'bar'."#;
    /// let mut ids = vec![];
    /// for (_, [id]) in re.captures_iter(hay).map(|c| c.extract()) {
    ///     ids.push(id);
    /// }
    /// assert_eq!(ids, vec!["foo", "bar"]);
    /// ```
    </span><span class="kw">pub fn </span>extract&lt;<span class="kw">const </span>N: usize&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (<span class="kw-2">&amp;</span><span class="lifetime">'h </span>str, [<span class="kw-2">&amp;</span><span class="lifetime">'h </span>str; N]) {
        <span class="kw">let </span>len = <span class="self">self
            </span>.static_captures_len
            .expect(<span class="string">"number of capture groups can vary in a match"</span>)
            .checked_sub(<span class="number">1</span>)
            .expect(<span class="string">"number of groups is always greater than zero"</span>);
        <span class="macro">assert_eq!</span>(N, len, <span class="string">"asked for {} groups, but must ask for {}"</span>, N, len);
        <span class="comment">// The regex-automata variant of extract is a bit more permissive.
        // It doesn't require the number of matching capturing groups to be
        // static, and you can even request fewer groups than what's there. So
        // this is guaranteed to never panic because we've asserted above that
        // the user has requested precisely the number of groups that must be
        // present in any match for this regex.
        </span><span class="self">self</span>.caps.extract(<span class="self">self</span>.haystack)
    }

    <span class="doccomment">/// Expands all instances of `$ref` in `replacement` to the corresponding
    /// capture group, and writes them to the `dst` buffer given. A `ref` can
    /// be a capture group index or a name. If `ref` doesn't refer to a capture
    /// group that participated in the match, then it is replaced with the
    /// empty string.
    ///
    /// # Format
    ///
    /// The format of the replacement string supports two different kinds of
    /// capture references: unbraced and braced.
    ///
    /// For the unbraced format, the format supported is `$ref` where `name`
    /// can be any character in the class `[0-9A-Za-z_]`. `ref` is always
    /// the longest possible parse. So for example, `$1a` corresponds to the
    /// capture group named `1a` and not the capture group at index `1`. If
    /// `ref` matches `^[0-9]+$`, then it is treated as a capture group index
    /// itself and not a name.
    ///
    /// For the braced format, the format supported is `${ref}` where `ref` can
    /// be any sequence of bytes except for `}`. If no closing brace occurs,
    /// then it is not considered a capture reference. As with the unbraced
    /// format, if `ref` matches `^[0-9]+$`, then it is treated as a capture
    /// group index and not a name.
    ///
    /// The braced format is useful for exerting precise control over the name
    /// of the capture reference. For example, `${1}a` corresponds to the
    /// capture group reference `1` followed by the letter `a`, where as `$1a`
    /// (as mentioned above) corresponds to the capture group reference `1a`.
    /// The braced format is also useful for expressing capture group names
    /// that use characters not supported by the unbraced format. For example,
    /// `${foo[bar].baz}` refers to the capture group named `foo[bar].baz`.
    ///
    /// If a capture group reference is found and it does not refer to a valid
    /// capture group, then it will be replaced with the empty string.
    ///
    /// To write a literal `$`, use `$$`.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(
    ///     r"(?&lt;day&gt;[0-9]{2})-(?&lt;month&gt;[0-9]{2})-(?&lt;year&gt;[0-9]{4})",
    /// ).unwrap();
    /// let hay = "On 14-03-2010, I became a Tenneessee lamb.";
    /// let caps = re.captures(hay).unwrap();
    ///
    /// let mut dst = String::new();
    /// caps.expand("year=$year, month=$month, day=$day", &amp;mut dst);
    /// assert_eq!(dst, "year=2010, month=03, day=14");
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>expand(<span class="kw-2">&amp;</span><span class="self">self</span>, replacement: <span class="kw-2">&amp;</span>str, dst: <span class="kw-2">&amp;mut </span>String) {
        <span class="self">self</span>.caps.interpolate_string_into(<span class="self">self</span>.haystack, replacement, dst);
    }

    <span class="doccomment">/// Returns an iterator over all capture groups. This includes both
    /// matching and non-matching groups.
    ///
    /// The iterator always yields at least one matching group: the first group
    /// (at index `0`) with no name. Subsequent groups are returned in the order
    /// of their opening parenthesis in the regex.
    ///
    /// The elements yielded have type `Option&lt;Match&lt;'h&gt;&gt;`, where a non-`None`
    /// value is present if the capture group matches.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(\w)(\d)?(\w)").unwrap();
    /// let caps = re.captures("AZ").unwrap();
    ///
    /// let mut it = caps.iter();
    /// assert_eq!(it.next().unwrap().map(|m| m.as_str()), Some("AZ"));
    /// assert_eq!(it.next().unwrap().map(|m| m.as_str()), Some("A"));
    /// assert_eq!(it.next().unwrap().map(|m| m.as_str()), None);
    /// assert_eq!(it.next().unwrap().map(|m| m.as_str()), Some("Z"));
    /// assert_eq!(it.next(), None);
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>iter&lt;<span class="lifetime">'c</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'c </span><span class="self">self</span>) -&gt; SubCaptureMatches&lt;<span class="lifetime">'c</span>, <span class="lifetime">'h</span>&gt; {
        SubCaptureMatches { haystack: <span class="self">self</span>.haystack, it: <span class="self">self</span>.caps.iter() }
    }

    <span class="doccomment">/// Returns the total number of capture groups. This includes both
    /// matching and non-matching groups.
    ///
    /// The length returned is always equivalent to the number of elements
    /// yielded by [`Captures::iter`]. Consequently, the length is always
    /// greater than zero since every `Captures` value always includes the
    /// match for the entire regex.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(\w)(\d)?(\w)").unwrap();
    /// let caps = re.captures("AZ").unwrap();
    /// assert_eq!(caps.len(), 4);
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.caps.group_len()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'h</span>&gt; core::fmt::Debug <span class="kw">for </span>Captures&lt;<span class="lifetime">'h</span>&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; core::fmt::Result {
        <span class="doccomment">/// A little helper type to provide a nice map-like debug
        /// representation for our capturing group spans.
        ///
        /// regex-automata has something similar, but it includes the pattern
        /// ID in its debug output, which is confusing. It also doesn't include
        /// that strings that match because a regex-automata `Captures` doesn't
        /// borrow the haystack.
        </span><span class="kw">struct </span>CapturesDebugMap&lt;<span class="lifetime">'a</span>&gt; {
            caps: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>Captures&lt;<span class="lifetime">'a</span>&gt;,
        }

        <span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; core::fmt::Debug <span class="kw">for </span>CapturesDebugMap&lt;<span class="lifetime">'a</span>&gt; {
            <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
                <span class="kw">let </span><span class="kw-2">mut </span>map = f.debug_map();
                <span class="kw">let </span>names =
                    <span class="self">self</span>.caps.caps.group_info().pattern_names(PatternID::ZERO);
                <span class="kw">for </span>(group_index, maybe_name) <span class="kw">in </span>names.enumerate() {
                    <span class="kw">let </span>key = Key(group_index, maybe_name);
                    <span class="kw">match </span><span class="self">self</span>.caps.get(group_index) {
                        <span class="prelude-val">None </span>=&gt; map.entry(<span class="kw-2">&amp;</span>key, <span class="kw-2">&amp;</span><span class="prelude-val">None</span>::&lt;()&gt;),
                        <span class="prelude-val">Some</span>(mat) =&gt; map.entry(<span class="kw-2">&amp;</span>key, <span class="kw-2">&amp;</span>Value(mat)),
                    };
                }
                map.finish()
            }
        }

        <span class="kw">struct </span>Key&lt;<span class="lifetime">'a</span>&gt;(usize, <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str&gt;);

        <span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; core::fmt::Debug <span class="kw">for </span>Key&lt;<span class="lifetime">'a</span>&gt; {
            <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
                <span class="macro">write!</span>(f, <span class="string">"{}"</span>, <span class="self">self</span>.<span class="number">0</span>)<span class="question-mark">?</span>;
                <span class="kw">if let </span><span class="prelude-val">Some</span>(name) = <span class="self">self</span>.<span class="number">1 </span>{
                    <span class="macro">write!</span>(f, <span class="string">"/{:?}"</span>, name)<span class="question-mark">?</span>;
                }
                <span class="prelude-val">Ok</span>(())
            }
        }

        <span class="kw">struct </span>Value&lt;<span class="lifetime">'a</span>&gt;(Match&lt;<span class="lifetime">'a</span>&gt;);

        <span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; core::fmt::Debug <span class="kw">for </span>Value&lt;<span class="lifetime">'a</span>&gt; {
            <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
                <span class="macro">write!</span>(
                    f,
                    <span class="string">"{}..{}/{:?}"</span>,
                    <span class="self">self</span>.<span class="number">0</span>.start(),
                    <span class="self">self</span>.<span class="number">0</span>.end(),
                    <span class="self">self</span>.<span class="number">0</span>.as_str()
                )
            }
        }

        f.debug_tuple(<span class="string">"Captures"</span>)
            .field(<span class="kw-2">&amp;</span>CapturesDebugMap { caps: <span class="self">self </span>})
            .finish()
    }
}

<span class="doccomment">/// Get a matching capture group's haystack substring by index.
///
/// The haystack substring returned can't outlive the `Captures` object if this
/// method is used, because of how `Index` is defined (normally `a[i]` is part
/// of `a` and can't outlive it). To work around this limitation, do that, use
/// [`Captures::get`] instead.
///
/// `'h` is the lifetime of the matched haystack, but the lifetime of the
/// `&amp;str` returned by this implementation is the lifetime of the `Captures`
/// value itself.
///
/// # Panics
///
/// If there is no matching group at the given index.
</span><span class="kw">impl</span>&lt;<span class="lifetime">'h</span>&gt; core::ops::Index&lt;usize&gt; <span class="kw">for </span>Captures&lt;<span class="lifetime">'h</span>&gt; {
    <span class="kw">type </span>Output = str;

    <span class="comment">// The lifetime is written out to make it clear that the &amp;str returned
    // does NOT have a lifetime equivalent to 'h.
    </span><span class="kw">fn </span>index&lt;<span class="lifetime">'a</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="self">self</span>, i: usize) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str {
        <span class="self">self</span>.get(i)
            .map(|m| m.as_str())
            .unwrap_or_else(|| <span class="macro">panic!</span>(<span class="string">"no group at index '{}'"</span>, i))
    }
}

<span class="doccomment">/// Get a matching capture group's haystack substring by name.
///
/// The haystack substring returned can't outlive the `Captures` object if this
/// method is used, because of how `Index` is defined (normally `a[i]` is part
/// of `a` and can't outlive it). To work around this limitation, do that, use
/// [`Captures::name`] instead.
///
/// `'h` is the lifetime of the matched haystack, but the lifetime of the
/// `&amp;str` returned by this implementation is the lifetime of the `Captures`
/// value itself.
///
/// `'n` is the lifetime of the group name used to index the `Captures` value.
///
/// # Panics
///
/// If there is no matching group at the given name.
</span><span class="kw">impl</span>&lt;<span class="lifetime">'h</span>, <span class="lifetime">'n</span>&gt; core::ops::Index&lt;<span class="kw-2">&amp;</span><span class="lifetime">'n </span>str&gt; <span class="kw">for </span>Captures&lt;<span class="lifetime">'h</span>&gt; {
    <span class="kw">type </span>Output = str;

    <span class="kw">fn </span>index&lt;<span class="lifetime">'a</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="self">self</span>, name: <span class="kw-2">&amp;</span><span class="lifetime">'n </span>str) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str {
        <span class="self">self</span>.name(name)
            .map(|m| m.as_str())
            .unwrap_or_else(|| <span class="macro">panic!</span>(<span class="string">"no group named '{}'"</span>, name))
    }
}

<span class="doccomment">/// A low level representation of the byte offsets of each capture group.
///
/// You can think of this as a lower level [`Captures`], where this type does
/// not support named capturing groups directly and it does not borrow the
/// haystack that these offsets were matched on.
///
/// Primarily, this type is useful when using the lower level `Regex` APIs such
/// as [`Regex::captures_read`], which permits amortizing the allocation in
/// which capture match offsets are stored.
///
/// In order to build a value of this type, you'll need to call the
/// [`Regex::capture_locations`] method. The value returned can then be reused
/// in subsequent searches for that regex. Using it for other regexes may
/// result in a panic or otherwise incorrect results.
///
/// # Example
///
/// This example shows how to create and use `CaptureLocations` in a search.
///
/// ```
/// use regex::Regex;
///
/// let re = Regex::new(r"(?&lt;first&gt;\w+)\s+(?&lt;last&gt;\w+)").unwrap();
/// let mut locs = re.capture_locations();
/// let m = re.captures_read(&amp;mut locs, "Bruce Springsteen").unwrap();
/// assert_eq!(0..17, m.range());
/// assert_eq!(Some((0, 17)), locs.get(0));
/// assert_eq!(Some((0, 5)), locs.get(1));
/// assert_eq!(Some((6, 17)), locs.get(2));
///
/// // Asking for an invalid capture group always returns None.
/// assert_eq!(None, locs.get(3));
/// # // literals are too big for 32-bit usize: #1041
/// # #[cfg(target_pointer_width = "64")]
/// assert_eq!(None, locs.get(34973498648));
/// # #[cfg(target_pointer_width = "64")]
/// assert_eq!(None, locs.get(9944060567225171988));
/// ```
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>CaptureLocations(captures::Captures);

<span class="doccomment">/// A type alias for `CaptureLocations` for backwards compatibility.
///
/// Previously, we exported `CaptureLocations` as `Locations` in an
/// undocumented API. To prevent breaking that code (e.g., in `regex-capi`),
/// we continue re-exporting the same undocumented API.
</span><span class="attr">#[doc(hidden)]
</span><span class="kw">pub type </span>Locations = CaptureLocations;

<span class="kw">impl </span>CaptureLocations {
    <span class="doccomment">/// Returns the start and end byte offsets of the capture group at index
    /// `i`. This returns `None` if `i` is not a valid capture group or if the
    /// capture group did not match.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(?&lt;first&gt;\w+)\s+(?&lt;last&gt;\w+)").unwrap();
    /// let mut locs = re.capture_locations();
    /// re.captures_read(&amp;mut locs, "Bruce Springsteen").unwrap();
    /// assert_eq!(Some((0, 17)), locs.get(0));
    /// assert_eq!(Some((0, 5)), locs.get(1));
    /// assert_eq!(Some((6, 17)), locs.get(2));
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>get(<span class="kw-2">&amp;</span><span class="self">self</span>, i: usize) -&gt; <span class="prelude-ty">Option</span>&lt;(usize, usize)&gt; {
        <span class="self">self</span>.<span class="number">0</span>.get_group(i).map(|sp| (sp.start, sp.end))
    }

    <span class="doccomment">/// Returns the total number of capture groups (even if they didn't match).
    /// That is, the length returned is unaffected by the result of a search.
    ///
    /// This is always at least `1` since every regex has at least `1`
    /// capturing group that corresponds to the entire match.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"(?&lt;first&gt;\w+)\s+(?&lt;last&gt;\w+)").unwrap();
    /// let mut locs = re.capture_locations();
    /// assert_eq!(3, locs.len());
    /// re.captures_read(&amp;mut locs, "Bruce Springsteen").unwrap();
    /// assert_eq!(3, locs.len());
    /// ```
    ///
    /// Notice that the length is always at least `1`, regardless of the regex:
    ///
    /// ```
    /// use regex::Regex;
    ///
    /// let re = Regex::new(r"").unwrap();
    /// let locs = re.capture_locations();
    /// assert_eq!(1, locs.len());
    ///
    /// // [a&amp;&amp;b] is a regex that never matches anything.
    /// let re = Regex::new(r"[a&amp;&amp;b]").unwrap();
    /// let locs = re.capture_locations();
    /// assert_eq!(1, locs.len());
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="comment">// self.0.group_len() returns 0 if the underlying captures doesn't
        // represent a match, but the behavior guaranteed for this method is
        // that the length doesn't change based on a match or not.
        </span><span class="self">self</span>.<span class="number">0</span>.group_info().group_len(PatternID::ZERO)
    }

    <span class="doccomment">/// An alias for the `get` method for backwards compatibility.
    ///
    /// Previously, we exported `get` as `pos` in an undocumented API. To
    /// prevent breaking that code (e.g., in `regex-capi`), we continue
    /// re-exporting the same undocumented API.
    </span><span class="attr">#[doc(hidden)]
    #[inline]
    </span><span class="kw">pub fn </span>pos(<span class="kw-2">&amp;</span><span class="self">self</span>, i: usize) -&gt; <span class="prelude-ty">Option</span>&lt;(usize, usize)&gt; {
        <span class="self">self</span>.get(i)
    }
}

<span class="doccomment">/// An iterator over all non-overlapping matches in a haystack.
///
/// This iterator yields [`Match`] values. The iterator stops when no more
/// matches can be found.
///
/// `'r` is the lifetime of the compiled regular expression and `'h` is the
/// lifetime of the haystack.
///
/// This iterator is created by [`Regex::find_iter`].
///
/// # Time complexity
///
/// Note that since an iterator runs potentially many searches on the haystack
/// and since each search has worst case `O(m * n)` time complexity, the
/// overall worst case time complexity for iteration is `O(m * n^2)`.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>Matches&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
    haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
    it: meta::FindMatches&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; Iterator <span class="kw">for </span>Matches&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
    <span class="kw">type </span>Item = Match&lt;<span class="lifetime">'h</span>&gt;;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="self">self</span>.it
            .next()
            .map(|sp| Match::new(<span class="self">self</span>.haystack, sp.start(), sp.end()))
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>count(<span class="self">self</span>) -&gt; usize {
        <span class="comment">// This can actually be up to 2x faster than calling `next()` until
        // completion, because counting matches when using a DFA only requires
        // finding the end of each match. But returning a `Match` via `next()`
        // requires the start of each match which, with a DFA, requires a
        // reverse forward scan to find it.
        </span><span class="self">self</span>.it.count()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; core::iter::FusedIterator <span class="kw">for </span>Matches&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {}

<span class="doccomment">/// An iterator over all non-overlapping capture matches in a haystack.
///
/// This iterator yields [`Captures`] values. The iterator stops when no more
/// matches can be found.
///
/// `'r` is the lifetime of the compiled regular expression and `'h` is the
/// lifetime of the matched string.
///
/// This iterator is created by [`Regex::captures_iter`].
///
/// # Time complexity
///
/// Note that since an iterator runs potentially many searches on the haystack
/// and since each search has worst case `O(m * n)` time complexity, the
/// overall worst case time complexity for iteration is `O(m * n^2)`.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>CaptureMatches&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
    haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
    it: meta::CapturesMatches&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; Iterator <span class="kw">for </span>CaptureMatches&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
    <span class="kw">type </span>Item = Captures&lt;<span class="lifetime">'h</span>&gt;;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Captures&lt;<span class="lifetime">'h</span>&gt;&gt; {
        <span class="kw">let </span>static_captures_len = <span class="self">self</span>.it.regex().static_captures_len();
        <span class="self">self</span>.it.next().map(|caps| Captures {
            haystack: <span class="self">self</span>.haystack,
            caps,
            static_captures_len,
        })
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>count(<span class="self">self</span>) -&gt; usize {
        <span class="comment">// This can actually be up to 2x faster than calling `next()` until
        // completion, because counting matches when using a DFA only requires
        // finding the end of each match. But returning a `Match` via `next()`
        // requires the start of each match which, with a DFA, requires a
        // reverse forward scan to find it.
        </span><span class="self">self</span>.it.count()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; core::iter::FusedIterator <span class="kw">for </span>CaptureMatches&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {}

<span class="doccomment">/// An iterator over all substrings delimited by a regex match.
///
/// `'r` is the lifetime of the compiled regular expression and `'h` is the
/// lifetime of the byte string being split.
///
/// This iterator is created by [`Regex::split`].
///
/// # Time complexity
///
/// Note that since an iterator runs potentially many searches on the haystack
/// and since each search has worst case `O(m * n)` time complexity, the
/// overall worst case time complexity for iteration is `O(m * n^2)`.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>Split&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
    haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
    it: meta::Split&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; Iterator <span class="kw">for </span>Split&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
    <span class="kw">type </span>Item = <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'h </span>str&gt; {
        <span class="self">self</span>.it.next().map(|span| <span class="kw-2">&amp;</span><span class="self">self</span>.haystack[span])
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; core::iter::FusedIterator <span class="kw">for </span>Split&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {}

<span class="doccomment">/// An iterator over at most `N` substrings delimited by a regex match.
///
/// The last substring yielded by this iterator will be whatever remains after
/// `N-1` splits.
///
/// `'r` is the lifetime of the compiled regular expression and `'h` is the
/// lifetime of the byte string being split.
///
/// This iterator is created by [`Regex::splitn`].
///
/// # Time complexity
///
/// Note that since an iterator runs potentially many searches on the haystack
/// and since each search has worst case `O(m * n)` time complexity, the
/// overall worst case time complexity for iteration is `O(m * n^2)`.
///
/// Although note that the worst case time here has an upper bound given
/// by the `limit` parameter to [`Regex::splitn`].
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>SplitN&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
    haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
    it: meta::SplitN&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; Iterator <span class="kw">for </span>SplitN&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {
    <span class="kw">type </span>Item = <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'h </span>str&gt; {
        <span class="self">self</span>.it.next().map(|span| <span class="kw-2">&amp;</span><span class="self">self</span>.haystack[span])
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        <span class="self">self</span>.it.size_hint()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; core::iter::FusedIterator <span class="kw">for </span>SplitN&lt;<span class="lifetime">'r</span>, <span class="lifetime">'h</span>&gt; {}

<span class="doccomment">/// An iterator over the names of all capture groups in a regex.
///
/// This iterator yields values of type `Option&lt;&amp;str&gt;` in order of the opening
/// capture group parenthesis in the regex pattern. `None` is yielded for
/// groups with no name. The first element always corresponds to the implicit
/// and unnamed group for the overall match.
///
/// `'r` is the lifetime of the compiled regular expression.
///
/// This iterator is created by [`Regex::capture_names`].
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>CaptureNames&lt;<span class="lifetime">'r</span>&gt;(captures::GroupInfoPatternNames&lt;<span class="lifetime">'r</span>&gt;);

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>&gt; Iterator <span class="kw">for </span>CaptureNames&lt;<span class="lifetime">'r</span>&gt; {
    <span class="kw">type </span>Item = <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'r </span>str&gt;;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'r </span>str&gt;&gt; {
        <span class="self">self</span>.<span class="number">0</span>.next()
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        <span class="self">self</span>.<span class="number">0</span>.size_hint()
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>count(<span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.<span class="number">0</span>.count()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>&gt; ExactSizeIterator <span class="kw">for </span>CaptureNames&lt;<span class="lifetime">'r</span>&gt; {}

<span class="kw">impl</span>&lt;<span class="lifetime">'r</span>&gt; core::iter::FusedIterator <span class="kw">for </span>CaptureNames&lt;<span class="lifetime">'r</span>&gt; {}

<span class="doccomment">/// An iterator over all group matches in a [`Captures`] value.
///
/// This iterator yields values of type `Option&lt;Match&lt;'h&gt;&gt;`, where `'h` is the
/// lifetime of the haystack that the matches are for. The order of elements
/// yielded corresponds to the order of the opening parenthesis for the group
/// in the regex pattern. `None` is yielded for groups that did not participate
/// in the match.
///
/// The first element always corresponds to the implicit group for the overall
/// match. Since this iterator is created by a [`Captures`] value, and a
/// `Captures` value is only created when a match occurs, it follows that the
/// first element yielded by this iterator is guaranteed to be non-`None`.
///
/// The lifetime `'c` corresponds to the lifetime of the `Captures` value that
/// created this iterator, and the lifetime `'h` corresponds to the originally
/// matched haystack.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>SubCaptureMatches&lt;<span class="lifetime">'c</span>, <span class="lifetime">'h</span>&gt; {
    haystack: <span class="kw-2">&amp;</span><span class="lifetime">'h </span>str,
    it: captures::CapturesPatternIter&lt;<span class="lifetime">'c</span>&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'c</span>, <span class="lifetime">'h</span>&gt; Iterator <span class="kw">for </span>SubCaptureMatches&lt;<span class="lifetime">'c</span>, <span class="lifetime">'h</span>&gt; {
    <span class="kw">type </span>Item = <span class="prelude-ty">Option</span>&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt;;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="prelude-ty">Option</span>&lt;Match&lt;<span class="lifetime">'h</span>&gt;&gt;&gt; {
        <span class="self">self</span>.it.next().map(|group| {
            group.map(|sp| Match::new(<span class="self">self</span>.haystack, sp.start, sp.end))
        })
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        <span class="self">self</span>.it.size_hint()
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>count(<span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.it.count()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'c</span>, <span class="lifetime">'h</span>&gt; ExactSizeIterator <span class="kw">for </span>SubCaptureMatches&lt;<span class="lifetime">'c</span>, <span class="lifetime">'h</span>&gt; {}

<span class="kw">impl</span>&lt;<span class="lifetime">'c</span>, <span class="lifetime">'h</span>&gt; core::iter::FusedIterator <span class="kw">for </span>SubCaptureMatches&lt;<span class="lifetime">'c</span>, <span class="lifetime">'h</span>&gt; {}

<span class="doccomment">/// A trait for types that can be used to replace matches in a haystack.
///
/// In general, users of this crate shouldn't need to implement this trait,
/// since implementations are already provided for `&amp;str` along with other
/// variants of string types, as well as `FnMut(&amp;Captures) -&gt; String` (or any
/// `FnMut(&amp;Captures) -&gt; T` where `T: AsRef&lt;str&gt;`). Those cover most use cases,
/// but callers can implement this trait directly if necessary.
///
/// # Example
///
/// This example shows a basic implementation of  the `Replacer` trait. This
/// can be done much more simply using the replacement string interpolation
/// support (e.g., `$first $last`), but this approach avoids needing to parse
/// the replacement string at all.
///
/// ```
/// use regex::{Captures, Regex, Replacer};
///
/// struct NameSwapper;
///
/// impl Replacer for NameSwapper {
///     fn replace_append(&amp;mut self, caps: &amp;Captures&lt;'_&gt;, dst: &amp;mut String) {
///         dst.push_str(&amp;caps["first"]);
///         dst.push_str(" ");
///         dst.push_str(&amp;caps["last"]);
///     }
/// }
///
/// let re = Regex::new(r"(?&lt;last&gt;[^,\s]+),\s+(?&lt;first&gt;\S+)").unwrap();
/// let result = re.replace("Springsteen, Bruce", NameSwapper);
/// assert_eq!(result, "Bruce Springsteen");
/// ```
</span><span class="kw">pub trait </span>Replacer {
    <span class="doccomment">/// Appends possibly empty data to `dst` to replace the current match.
    ///
    /// The current match is represented by `caps`, which is guaranteed to
    /// have a match at capture group `0`.
    ///
    /// For example, a no-op replacement would be `dst.push_str(&amp;caps[0])`.
    </span><span class="kw">fn </span>replace_append(<span class="kw-2">&amp;mut </span><span class="self">self</span>, caps: <span class="kw-2">&amp;</span>Captures&lt;<span class="lifetime">'_</span>&gt;, dst: <span class="kw-2">&amp;mut </span>String);

    <span class="doccomment">/// Return a fixed unchanging replacement string.
    ///
    /// When doing replacements, if access to [`Captures`] is not needed (e.g.,
    /// the replacement string does not need `$` expansion), then it can be
    /// beneficial to avoid finding sub-captures.
    ///
    /// In general, this is called once for every call to a replacement routine
    /// such as [`Regex::replace_all`].
    </span><span class="kw">fn </span>no_expansion&lt;<span class="lifetime">'r</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'r </span><span class="kw-2">mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Cow&lt;<span class="lifetime">'r</span>, str&gt;&gt; {
        <span class="prelude-val">None
    </span>}

    <span class="doccomment">/// Returns a type that implements `Replacer`, but that borrows and wraps
    /// this `Replacer`.
    ///
    /// This is useful when you want to take a generic `Replacer` (which might
    /// not be cloneable) and use it without consuming it, so it can be used
    /// more than once.
    ///
    /// # Example
    ///
    /// ```
    /// use regex::{Regex, Replacer};
    ///
    /// fn replace_all_twice&lt;R: Replacer&gt;(
    ///     re: Regex,
    ///     src: &amp;str,
    ///     mut rep: R,
    /// ) -&gt; String {
    ///     let dst = re.replace_all(src, rep.by_ref());
    ///     let dst = re.replace_all(&amp;dst, rep.by_ref());
    ///     dst.into_owned()
    /// }
    /// ```
    </span><span class="kw">fn </span>by_ref&lt;<span class="lifetime">'r</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'r </span><span class="kw-2">mut </span><span class="self">self</span>) -&gt; ReplacerRef&lt;<span class="lifetime">'r</span>, <span class="self">Self</span>&gt; {
        ReplacerRef(<span class="self">self</span>)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Replacer <span class="kw">for </span><span class="kw-2">&amp;</span><span class="lifetime">'a </span>str {
    <span class="kw">fn </span>replace_append(<span class="kw-2">&amp;mut </span><span class="self">self</span>, caps: <span class="kw-2">&amp;</span>Captures&lt;<span class="lifetime">'_</span>&gt;, dst: <span class="kw-2">&amp;mut </span>String) {
        caps.expand(<span class="kw-2">*</span><span class="self">self</span>, dst);
    }

    <span class="kw">fn </span>no_expansion(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Cow&lt;<span class="lifetime">'_</span>, str&gt;&gt; {
        no_expansion(<span class="self">self</span>)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Replacer <span class="kw">for </span><span class="kw-2">&amp;</span><span class="lifetime">'a </span>String {
    <span class="kw">fn </span>replace_append(<span class="kw-2">&amp;mut </span><span class="self">self</span>, caps: <span class="kw-2">&amp;</span>Captures&lt;<span class="lifetime">'_</span>&gt;, dst: <span class="kw-2">&amp;mut </span>String) {
        <span class="self">self</span>.as_str().replace_append(caps, dst)
    }

    <span class="kw">fn </span>no_expansion(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Cow&lt;<span class="lifetime">'_</span>, str&gt;&gt; {
        no_expansion(<span class="self">self</span>)
    }
}

<span class="kw">impl </span>Replacer <span class="kw">for </span>String {
    <span class="kw">fn </span>replace_append(<span class="kw-2">&amp;mut </span><span class="self">self</span>, caps: <span class="kw-2">&amp;</span>Captures&lt;<span class="lifetime">'_</span>&gt;, dst: <span class="kw-2">&amp;mut </span>String) {
        <span class="self">self</span>.as_str().replace_append(caps, dst)
    }

    <span class="kw">fn </span>no_expansion(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Cow&lt;<span class="lifetime">'_</span>, str&gt;&gt; {
        no_expansion(<span class="self">self</span>)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Replacer <span class="kw">for </span>Cow&lt;<span class="lifetime">'a</span>, str&gt; {
    <span class="kw">fn </span>replace_append(<span class="kw-2">&amp;mut </span><span class="self">self</span>, caps: <span class="kw-2">&amp;</span>Captures&lt;<span class="lifetime">'_</span>&gt;, dst: <span class="kw-2">&amp;mut </span>String) {
        <span class="self">self</span>.as_ref().replace_append(caps, dst)
    }

    <span class="kw">fn </span>no_expansion(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Cow&lt;<span class="lifetime">'_</span>, str&gt;&gt; {
        no_expansion(<span class="self">self</span>)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Replacer <span class="kw">for </span><span class="kw-2">&amp;</span><span class="lifetime">'a </span>Cow&lt;<span class="lifetime">'a</span>, str&gt; {
    <span class="kw">fn </span>replace_append(<span class="kw-2">&amp;mut </span><span class="self">self</span>, caps: <span class="kw-2">&amp;</span>Captures&lt;<span class="lifetime">'_</span>&gt;, dst: <span class="kw-2">&amp;mut </span>String) {
        <span class="self">self</span>.as_ref().replace_append(caps, dst)
    }

    <span class="kw">fn </span>no_expansion(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Cow&lt;<span class="lifetime">'_</span>, str&gt;&gt; {
        no_expansion(<span class="self">self</span>)
    }
}

<span class="kw">impl</span>&lt;F, T&gt; Replacer <span class="kw">for </span>F
<span class="kw">where
    </span>F: FnMut(<span class="kw-2">&amp;</span>Captures&lt;<span class="lifetime">'_</span>&gt;) -&gt; T,
    T: AsRef&lt;str&gt;,
{
    <span class="kw">fn </span>replace_append(<span class="kw-2">&amp;mut </span><span class="self">self</span>, caps: <span class="kw-2">&amp;</span>Captures&lt;<span class="lifetime">'_</span>&gt;, dst: <span class="kw-2">&amp;mut </span>String) {
        dst.push_str((<span class="kw-2">*</span><span class="self">self</span>)(caps).as_ref());
    }
}

<span class="doccomment">/// A by-reference adaptor for a [`Replacer`].
///
/// This permits reusing the same `Replacer` value in multiple calls to a
/// replacement routine like [`Regex::replace_all`].
///
/// This type is created by [`Replacer::by_ref`].
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>ReplacerRef&lt;<span class="lifetime">'a</span>, R: <span class="question-mark">?</span>Sized&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>R);

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, R: Replacer + <span class="question-mark">?</span>Sized + <span class="lifetime">'a</span>&gt; Replacer <span class="kw">for </span>ReplacerRef&lt;<span class="lifetime">'a</span>, R&gt; {
    <span class="kw">fn </span>replace_append(<span class="kw-2">&amp;mut </span><span class="self">self</span>, caps: <span class="kw-2">&amp;</span>Captures&lt;<span class="lifetime">'_</span>&gt;, dst: <span class="kw-2">&amp;mut </span>String) {
        <span class="self">self</span>.<span class="number">0</span>.replace_append(caps, dst)
    }

    <span class="kw">fn </span>no_expansion(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Cow&lt;<span class="lifetime">'_</span>, str&gt;&gt; {
        <span class="self">self</span>.<span class="number">0</span>.no_expansion()
    }
}

<span class="doccomment">/// A helper type for forcing literal string replacement.
///
/// It can be used with routines like [`Regex::replace`] and
/// [`Regex::replace_all`] to do a literal string replacement without expanding
/// `$name` to their corresponding capture groups. This can be both convenient
/// (to avoid escaping `$`, for example) and faster (since capture groups
/// don't need to be found).
///
/// `'s` is the lifetime of the literal string to use.
///
/// # Example
///
/// ```
/// use regex::{NoExpand, Regex};
///
/// let re = Regex::new(r"(?&lt;last&gt;[^,\s]+),\s+(\S+)").unwrap();
/// let result = re.replace("Springsteen, Bruce", NoExpand("$2 $last"));
/// assert_eq!(result, "$2 $last");
/// ```
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>NoExpand&lt;<span class="lifetime">'s</span>&gt;(<span class="kw">pub </span><span class="kw-2">&amp;</span><span class="lifetime">'s </span>str);

<span class="kw">impl</span>&lt;<span class="lifetime">'s</span>&gt; Replacer <span class="kw">for </span>NoExpand&lt;<span class="lifetime">'s</span>&gt; {
    <span class="kw">fn </span>replace_append(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw">_</span>: <span class="kw-2">&amp;</span>Captures&lt;<span class="lifetime">'_</span>&gt;, dst: <span class="kw-2">&amp;mut </span>String) {
        dst.push_str(<span class="self">self</span>.<span class="number">0</span>);
    }

    <span class="kw">fn </span>no_expansion(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Cow&lt;<span class="lifetime">'_</span>, str&gt;&gt; {
        <span class="prelude-val">Some</span>(Cow::Borrowed(<span class="self">self</span>.<span class="number">0</span>))
    }
}

<span class="doccomment">/// Quickly checks the given replacement string for whether interpolation
/// should be done on it. It returns `None` if a `$` was found anywhere in the
/// given string, which suggests interpolation needs to be done. But if there's
/// no `$` anywhere, then interpolation definitely does not need to be done. In
/// that case, the given string is returned as a borrowed `Cow`.
///
/// This is meant to be used to implement the `Replacer::no_expandsion` method
/// in its various trait impls.
</span><span class="kw">fn </span>no_expansion&lt;T: AsRef&lt;str&gt;&gt;(replacement: <span class="kw-2">&amp;</span>T) -&gt; <span class="prelude-ty">Option</span>&lt;Cow&lt;<span class="lifetime">'_</span>, str&gt;&gt; {
    <span class="kw">let </span>replacement = replacement.as_ref();
    <span class="kw">match </span><span class="kw">crate</span>::find_byte::find_byte(<span class="string">b'$'</span>, replacement.as_bytes()) {
        <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="prelude-val">None</span>,
        <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Some</span>(Cow::Borrowed(replacement)),
    }
}
</code></pre></div></section></main></body></html>