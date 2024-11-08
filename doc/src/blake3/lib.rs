<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/blake3-1.5.1/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="blake3" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../blake3/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! The official Rust implementation of the [BLAKE3] cryptographic hash
//! function.
//!
//! # Examples
//!
//! ```
//! # fn main() -&gt; Result&lt;(), Box&lt;dyn std::error::Error&gt;&gt; {
//! // Hash an input all at once.
//! let hash1 = blake3::hash(b"foobarbaz");
//!
//! // Hash an input incrementally.
//! let mut hasher = blake3::Hasher::new();
//! hasher.update(b"foo");
//! hasher.update(b"bar");
//! hasher.update(b"baz");
//! let hash2 = hasher.finalize();
//! assert_eq!(hash1, hash2);
//!
//! // Extended output. OutputReader also implements Read and Seek.
//! # #[cfg(feature = "std")] {
//! let mut output = [0; 1000];
//! let mut output_reader = hasher.finalize_xof();
//! output_reader.fill(&amp;mut output);
//! assert_eq!(hash1, output[..32]);
//! # }
//!
//! // Print a hash as hex.
//! println!("{}", hash1);
//! # Ok(())
//! # }
//! ```
//!
//! # Cargo Features
//!
//! The `std` feature (the only feature enabled by default) is required for
//! implementations of the [`Write`] and [`Seek`] traits, the
//! [`update_reader`](Hasher::update_reader) helper method, and runtime CPU
//! feature detection on x86. If this feature is disabled, the only way to use
//! the x86 SIMD implementations is to enable the corresponding instruction sets
//! globally, with e.g. `RUSTFLAGS="-C target-cpu=native"`. The resulting binary
//! will not be portable to other machines.
//!
//! The `rayon` feature (disabled by default, but enabled for [docs.rs]) adds
//! the [`update_rayon`](Hasher::update_rayon) and (in combination with `mmap`
//! below) [`update_mmap_rayon`](Hasher::update_mmap_rayon) methods, for
//! multithreaded hashing. However, even if this feature is enabled, all other
//! APIs remain single-threaded.
//!
//! The `mmap` feature (disabled by default, but enabled for [docs.rs]) adds the
//! [`update_mmap`](Hasher::update_mmap) and (in combination with `rayon` above)
//! [`update_mmap_rayon`](Hasher::update_mmap_rayon) helper methods for
//! memory-mapped IO.
//!
//! The `zeroize` feature (disabled by default, but enabled for [docs.rs])
//! implements
//! [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) for
//! this crate's types.
//!
//! The `serde` feature (disabled by default, but enabled for [docs.rs]) implements
//! [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and
//! [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html)
//! for [`Hash`](struct@Hash).
//!
//! The NEON implementation is enabled by default for AArch64 but requires the
//! `neon` feature for other ARM targets. Not all ARMv7 CPUs support NEON, and
//! enabling this feature will produce a binary that's not portable to CPUs
//! without NEON support.
//!
//! The `traits-preview` feature enables implementations of traits from the
//! RustCrypto [`digest`] crate, and re-exports that crate as `traits::digest`.
//! However, the traits aren't stable, and they're expected to change in
//! incompatible ways before that crate reaches 1.0. For that reason, this crate
//! makes no SemVer guarantees for this feature, and callers who use it should
//! expect breaking changes between patch versions. (The "-preview" feature name
//! follows the conventions of the RustCrypto [`signature`] crate.)
//!
//! [`Hasher::update_rayon`]: struct.Hasher.html#method.update_rayon
//! [BLAKE3]: https://blake3.io
//! [Rayon]: https://github.com/rayon-rs/rayon
//! [docs.rs]: https://docs.rs/
//! [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
//! [`Seek`]: https://doc.rust-lang.org/std/io/trait.Seek.html
//! [`digest`]: https://crates.io/crates/digest
//! [`signature`]: https://crates.io/crates/signature

</span><span class="attr">#![cfg_attr(not(feature = <span class="string">"std"</span>), no_std)]

#[cfg(test)]
</span><span class="kw">mod </span>test;

<span class="comment">// The guts module is for incremental use cases like the `bao` crate that need
// to explicitly compute chunk and parent chaining values. It is semi-stable
// and likely to keep working, but largely undocumented and not intended for
// widespread use.
</span><span class="attr">#[doc(hidden)]
</span><span class="kw">pub mod </span>guts;

<span class="doccomment">/// Undocumented and unstable, for benchmarks only.
</span><span class="attr">#[doc(hidden)]
</span><span class="kw">pub mod </span>platform;

<span class="comment">// Platform-specific implementations of the compression function. These
// BLAKE3-specific cfg flags are set in build.rs.
</span><span class="attr">#[cfg(blake3_avx2_rust)]
#[path = <span class="string">"rust_avx2.rs"</span>]
</span><span class="kw">mod </span>avx2;
<span class="attr">#[cfg(blake3_avx2_ffi)]
#[path = <span class="string">"ffi_avx2.rs"</span>]
</span><span class="kw">mod </span>avx2;
<span class="attr">#[cfg(blake3_avx512_ffi)]
#[path = <span class="string">"ffi_avx512.rs"</span>]
</span><span class="kw">mod </span>avx512;
<span class="attr">#[cfg(blake3_neon)]
#[path = <span class="string">"ffi_neon.rs"</span>]
</span><span class="kw">mod </span>neon;
<span class="kw">mod </span>portable;
<span class="attr">#[cfg(blake3_sse2_rust)]
#[path = <span class="string">"rust_sse2.rs"</span>]
</span><span class="kw">mod </span>sse2;
<span class="attr">#[cfg(blake3_sse2_ffi)]
#[path = <span class="string">"ffi_sse2.rs"</span>]
</span><span class="kw">mod </span>sse2;
<span class="attr">#[cfg(blake3_sse41_rust)]
#[path = <span class="string">"rust_sse41.rs"</span>]
</span><span class="kw">mod </span>sse41;
<span class="attr">#[cfg(blake3_sse41_ffi)]
#[path = <span class="string">"ffi_sse41.rs"</span>]
</span><span class="kw">mod </span>sse41;

<span class="attr">#[cfg(feature = <span class="string">"traits-preview"</span>)]
</span><span class="kw">pub mod </span>traits;

<span class="kw">mod </span>io;
<span class="kw">mod </span>join;

<span class="kw">use </span>arrayref::{array_mut_ref, array_ref};
<span class="kw">use </span>arrayvec::{ArrayString, ArrayVec};
<span class="kw">use </span>core::cmp;
<span class="kw">use </span>core::fmt;
<span class="kw">use </span>platform::{Platform, MAX_SIMD_DEGREE, MAX_SIMD_DEGREE_OR_2};

<span class="doccomment">/// The number of bytes in a [`Hash`](struct.Hash.html), 32.
</span><span class="kw">pub const </span>OUT_LEN: usize = <span class="number">32</span>;

<span class="doccomment">/// The number of bytes in a key, 32.
</span><span class="kw">pub const </span>KEY_LEN: usize = <span class="number">32</span>;

<span class="kw">const </span>MAX_DEPTH: usize = <span class="number">54</span>; <span class="comment">// 2^54 * CHUNK_LEN = 2^64
</span><span class="kw">use </span>guts::{BLOCK_LEN, CHUNK_LEN};

<span class="comment">// While iterating the compression function within a chunk, the CV is
// represented as words, to avoid doing two extra endianness conversions for
// each compression in the portable implementation. But the hash_many interface
// needs to hash both input bytes and parent nodes, so its better for its
// output CVs to be represented as bytes.
</span><span class="kw">type </span>CVWords = [u32; <span class="number">8</span>];
<span class="kw">type </span>CVBytes = [u8; <span class="number">32</span>]; <span class="comment">// little-endian

</span><span class="kw">const </span>IV: <span class="kw-2">&amp;</span>CVWords = <span class="kw-2">&amp;</span>[
    <span class="number">0x6A09E667</span>, <span class="number">0xBB67AE85</span>, <span class="number">0x3C6EF372</span>, <span class="number">0xA54FF53A</span>, <span class="number">0x510E527F</span>, <span class="number">0x9B05688C</span>, <span class="number">0x1F83D9AB</span>, <span class="number">0x5BE0CD19</span>,
];

<span class="kw">const </span>MSG_SCHEDULE: [[usize; <span class="number">16</span>]; <span class="number">7</span>] = [
    [<span class="number">0</span>, <span class="number">1</span>, <span class="number">2</span>, <span class="number">3</span>, <span class="number">4</span>, <span class="number">5</span>, <span class="number">6</span>, <span class="number">7</span>, <span class="number">8</span>, <span class="number">9</span>, <span class="number">10</span>, <span class="number">11</span>, <span class="number">12</span>, <span class="number">13</span>, <span class="number">14</span>, <span class="number">15</span>],
    [<span class="number">2</span>, <span class="number">6</span>, <span class="number">3</span>, <span class="number">10</span>, <span class="number">7</span>, <span class="number">0</span>, <span class="number">4</span>, <span class="number">13</span>, <span class="number">1</span>, <span class="number">11</span>, <span class="number">12</span>, <span class="number">5</span>, <span class="number">9</span>, <span class="number">14</span>, <span class="number">15</span>, <span class="number">8</span>],
    [<span class="number">3</span>, <span class="number">4</span>, <span class="number">10</span>, <span class="number">12</span>, <span class="number">13</span>, <span class="number">2</span>, <span class="number">7</span>, <span class="number">14</span>, <span class="number">6</span>, <span class="number">5</span>, <span class="number">9</span>, <span class="number">0</span>, <span class="number">11</span>, <span class="number">15</span>, <span class="number">8</span>, <span class="number">1</span>],
    [<span class="number">10</span>, <span class="number">7</span>, <span class="number">12</span>, <span class="number">9</span>, <span class="number">14</span>, <span class="number">3</span>, <span class="number">13</span>, <span class="number">15</span>, <span class="number">4</span>, <span class="number">0</span>, <span class="number">11</span>, <span class="number">2</span>, <span class="number">5</span>, <span class="number">8</span>, <span class="number">1</span>, <span class="number">6</span>],
    [<span class="number">12</span>, <span class="number">13</span>, <span class="number">9</span>, <span class="number">11</span>, <span class="number">15</span>, <span class="number">10</span>, <span class="number">14</span>, <span class="number">8</span>, <span class="number">7</span>, <span class="number">2</span>, <span class="number">5</span>, <span class="number">3</span>, <span class="number">0</span>, <span class="number">1</span>, <span class="number">6</span>, <span class="number">4</span>],
    [<span class="number">9</span>, <span class="number">14</span>, <span class="number">11</span>, <span class="number">5</span>, <span class="number">8</span>, <span class="number">12</span>, <span class="number">15</span>, <span class="number">1</span>, <span class="number">13</span>, <span class="number">3</span>, <span class="number">0</span>, <span class="number">10</span>, <span class="number">2</span>, <span class="number">6</span>, <span class="number">4</span>, <span class="number">7</span>],
    [<span class="number">11</span>, <span class="number">15</span>, <span class="number">5</span>, <span class="number">0</span>, <span class="number">1</span>, <span class="number">9</span>, <span class="number">8</span>, <span class="number">6</span>, <span class="number">14</span>, <span class="number">10</span>, <span class="number">2</span>, <span class="number">12</span>, <span class="number">3</span>, <span class="number">4</span>, <span class="number">7</span>, <span class="number">13</span>],
];

<span class="comment">// These are the internal flags that we use to domain separate root/non-root,
// chunk/parent, and chunk beginning/middle/end. These get set at the high end
// of the block flags word in the compression function, so their values start
// high and go down.
</span><span class="kw">const </span>CHUNK_START: u8 = <span class="number">1 </span>&lt;&lt; <span class="number">0</span>;
<span class="kw">const </span>CHUNK_END: u8 = <span class="number">1 </span>&lt;&lt; <span class="number">1</span>;
<span class="kw">const </span>PARENT: u8 = <span class="number">1 </span>&lt;&lt; <span class="number">2</span>;
<span class="kw">const </span>ROOT: u8 = <span class="number">1 </span>&lt;&lt; <span class="number">3</span>;
<span class="kw">const </span>KEYED_HASH: u8 = <span class="number">1 </span>&lt;&lt; <span class="number">4</span>;
<span class="kw">const </span>DERIVE_KEY_CONTEXT: u8 = <span class="number">1 </span>&lt;&lt; <span class="number">5</span>;
<span class="kw">const </span>DERIVE_KEY_MATERIAL: u8 = <span class="number">1 </span>&lt;&lt; <span class="number">6</span>;

<span class="attr">#[inline]
</span><span class="kw">fn </span>counter_low(counter: u64) -&gt; u32 {
    counter <span class="kw">as </span>u32
}

<span class="attr">#[inline]
</span><span class="kw">fn </span>counter_high(counter: u64) -&gt; u32 {
    (counter &gt;&gt; <span class="number">32</span>) <span class="kw">as </span>u32
}

<span class="doccomment">/// An output of the default size, 32 bytes, which provides constant-time
/// equality checking.
///
/// `Hash` implements [`From`] and [`Into`] for `[u8; 32]`, and it provides
/// [`from_bytes`] and [`as_bytes`] for explicit conversions between itself and
/// `[u8; 32]`. However, byte arrays and slices don't provide constant-time
/// equality checking, which is often a security requirement in software that
/// handles private data. `Hash` doesn't implement [`Deref`] or [`AsRef`], to
/// avoid situations where a type conversion happens implicitly and the
/// constant-time property is accidentally lost.
///
/// `Hash` provides the [`to_hex`] and [`from_hex`] methods for converting to
/// and from hexadecimal. It also implements [`Display`] and [`FromStr`].
///
/// [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
/// [`as_bytes`]: #method.as_bytes
/// [`from_bytes`]: #method.from_bytes
/// [`Deref`]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
/// [`AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
/// [`to_hex`]: #method.to_hex
/// [`from_hex`]: #method.from_hex
/// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
/// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
</span><span class="attr">#[cfg_attr(feature = <span class="string">"zeroize"</span>, derive(zeroize::Zeroize))]
#[cfg_attr(feature = <span class="string">"serde"</span>, derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Hash)]
</span><span class="kw">pub struct </span>Hash([u8; OUT_LEN]);

<span class="kw">impl </span>Hash {
    <span class="doccomment">/// The raw bytes of the `Hash`. Note that byte arrays don't provide
    /// constant-time equality checking, so if  you need to compare hashes,
    /// prefer the `Hash` type.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub const fn </span>as_bytes(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>[u8; OUT_LEN] {
        <span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0
    </span>}

    <span class="doccomment">/// Create a `Hash` from its raw bytes representation.
    </span><span class="kw">pub const fn </span>from_bytes(bytes: [u8; OUT_LEN]) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>(bytes)
    }

    <span class="doccomment">/// Encode a `Hash` in lowercase hexadecimal.
    ///
    /// The returned [`ArrayString`] is a fixed size and doesn't allocate memory
    /// on the heap. Note that [`ArrayString`] doesn't provide constant-time
    /// equality checking, so if you need to compare hashes, prefer the `Hash`
    /// type.
    ///
    /// [`ArrayString`]: https://docs.rs/arrayvec/0.5.1/arrayvec/struct.ArrayString.html
    </span><span class="kw">pub fn </span>to_hex(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; ArrayString&lt;{ <span class="number">2 </span>* OUT_LEN }&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>s = ArrayString::new();
        <span class="kw">let </span>table = <span class="string">b"0123456789abcdef"</span>;
        <span class="kw">for </span><span class="kw-2">&amp;</span>b <span class="kw">in </span><span class="self">self</span>.<span class="number">0</span>.iter() {
            s.push(table[(b &gt;&gt; <span class="number">4</span>) <span class="kw">as </span>usize] <span class="kw">as </span>char);
            s.push(table[(b &amp; <span class="number">0xf</span>) <span class="kw">as </span>usize] <span class="kw">as </span>char);
        }
        s
    }

    <span class="doccomment">/// Decode a `Hash` from hexadecimal. Both uppercase and lowercase ASCII
    /// bytes are supported.
    ///
    /// Any byte outside the ranges `'0'...'9'`, `'a'...'f'`, and `'A'...'F'`
    /// results in an error. An input length other than 64 also results in an
    /// error.
    ///
    /// Note that `Hash` also implements `FromStr`, so `Hash::from_hex("...")`
    /// is equivalent to `"...".parse()`.
    </span><span class="kw">pub fn </span>from_hex(hex: <span class="kw">impl </span>AsRef&lt;[u8]&gt;) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, HexError&gt; {
        <span class="kw">fn </span>hex_val(byte: u8) -&gt; <span class="prelude-ty">Result</span>&lt;u8, HexError&gt; {
            <span class="kw">match </span>byte {
                <span class="string">b'A'</span>..=<span class="string">b'F' </span>=&gt; <span class="prelude-val">Ok</span>(byte - <span class="string">b'A' </span>+ <span class="number">10</span>),
                <span class="string">b'a'</span>..=<span class="string">b'f' </span>=&gt; <span class="prelude-val">Ok</span>(byte - <span class="string">b'a' </span>+ <span class="number">10</span>),
                <span class="string">b'0'</span>..=<span class="string">b'9' </span>=&gt; <span class="prelude-val">Ok</span>(byte - <span class="string">b'0'</span>),
                <span class="kw">_ </span>=&gt; <span class="prelude-val">Err</span>(HexError(HexErrorInner::InvalidByte(byte))),
            }
        }
        <span class="kw">let </span>hex_bytes: <span class="kw-2">&amp;</span>[u8] = hex.as_ref();
        <span class="kw">if </span>hex_bytes.len() != OUT_LEN * <span class="number">2 </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(HexError(HexErrorInner::InvalidLen(hex_bytes.len())));
        }
        <span class="kw">let </span><span class="kw-2">mut </span>hash_bytes: [u8; OUT_LEN] = [<span class="number">0</span>; OUT_LEN];
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..OUT_LEN {
            hash_bytes[i] = <span class="number">16 </span>* hex_val(hex_bytes[<span class="number">2 </span>* i])<span class="question-mark">? </span>+ hex_val(hex_bytes[<span class="number">2 </span>* i + <span class="number">1</span>])<span class="question-mark">?</span>;
        }
        <span class="prelude-val">Ok</span>(Hash::from(hash_bytes))
    }
}

<span class="kw">impl </span>From&lt;[u8; OUT_LEN]&gt; <span class="kw">for </span>Hash {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from(bytes: [u8; OUT_LEN]) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::from_bytes(bytes)
    }
}

<span class="kw">impl </span>From&lt;Hash&gt; <span class="kw">for </span>[u8; OUT_LEN] {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from(hash: Hash) -&gt; <span class="self">Self </span>{
        hash.<span class="number">0
    </span>}
}

<span class="kw">impl </span>core::str::FromStr <span class="kw">for </span>Hash {
    <span class="kw">type </span><span class="prelude-val">Err </span>= HexError;

    <span class="kw">fn </span>from_str(s: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, <span class="self">Self</span>::Err&gt; {
        Hash::from_hex(s)
    }
}

<span class="comment">// A proper implementation of constant time equality is tricky, and we get it from the
// constant_time_eq crate instead of rolling our own. However, that crate isn't compatible with
// Miri, so we roll our own just for that.
</span><span class="attr">#[cfg(miri)]
</span><span class="kw">fn </span>constant_time_eq_miri(a: <span class="kw-2">&amp;</span>[u8], b: <span class="kw-2">&amp;</span>[u8]) -&gt; bool {
    <span class="kw">if </span>a.len() != b.len() {
        <span class="kw">return </span><span class="bool-val">false</span>;
    }
    <span class="kw">let </span><span class="kw-2">mut </span>x = <span class="number">0</span>;
    <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..a.len() {
        x |= a[i] ^ b[i];
    }
    x == <span class="number">0
</span>}

<span class="doccomment">/// This implementation is constant-time.
</span><span class="kw">impl </span>PartialEq <span class="kw">for </span>Hash {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>Hash) -&gt; bool {
        <span class="attr">#[cfg(miri)]
        </span><span class="kw">return </span>constant_time_eq_miri(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>, <span class="kw-2">&amp;</span>other.<span class="number">0</span>);
        <span class="attr">#[cfg(not(miri))]
        </span>constant_time_eq::constant_time_eq_32(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>, <span class="kw-2">&amp;</span>other.<span class="number">0</span>)
    }
}

<span class="doccomment">/// This implementation is constant-time.
</span><span class="kw">impl </span>PartialEq&lt;[u8; OUT_LEN]&gt; <span class="kw">for </span>Hash {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>[u8; OUT_LEN]) -&gt; bool {
        <span class="attr">#[cfg(miri)]
        </span><span class="kw">return </span>constant_time_eq_miri(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>, other);
        <span class="attr">#[cfg(not(miri))]
        </span>constant_time_eq::constant_time_eq_32(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>, other)
    }
}

<span class="doccomment">/// This implementation is constant-time if the target is 32 bytes long.
</span><span class="kw">impl </span>PartialEq&lt;[u8]&gt; <span class="kw">for </span>Hash {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>[u8]) -&gt; bool {
        <span class="attr">#[cfg(miri)]
        </span><span class="kw">return </span>constant_time_eq_miri(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>, other);
        <span class="attr">#[cfg(not(miri))]
        </span>constant_time_eq::constant_time_eq(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>, other)
    }
}

<span class="kw">impl </span>Eq <span class="kw">for </span>Hash {}

<span class="kw">impl </span>fmt::Display <span class="kw">for </span>Hash {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="comment">// Formatting field as `&amp;str` to reduce code size since the `Debug`
        // dynamic dispatch table for `&amp;str` is likely needed elsewhere already,
        // but that for `ArrayString&lt;[u8; 64]&gt;` is not.
        </span><span class="kw">let </span>hex = <span class="self">self</span>.to_hex();
        <span class="kw">let </span>hex: <span class="kw-2">&amp;</span>str = hex.as_str();

        f.write_str(hex)
    }
}

<span class="kw">impl </span>fmt::Debug <span class="kw">for </span>Hash {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="comment">// Formatting field as `&amp;str` to reduce code size since the `Debug`
        // dynamic dispatch table for `&amp;str` is likely needed elsewhere already,
        // but that for `ArrayString&lt;[u8; 64]&gt;` is not.
        </span><span class="kw">let </span>hex = <span class="self">self</span>.to_hex();
        <span class="kw">let </span>hex: <span class="kw-2">&amp;</span>str = hex.as_str();

        f.debug_tuple(<span class="string">"Hash"</span>).field(<span class="kw-2">&amp;</span>hex).finish()
    }
}

<span class="doccomment">/// The error type for [`Hash::from_hex`].
///
/// The `.to_string()` representation of this error currently distinguishes between bad length
/// errors and bad character errors. This is to help with logging and debugging, but it isn't a
/// stable API detail, and it may change at any time.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>HexError(HexErrorInner);

<span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">enum </span>HexErrorInner {
    InvalidByte(u8),
    InvalidLen(usize),
}

<span class="kw">impl </span>fmt::Display <span class="kw">for </span>HexError {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0 </span>{
            HexErrorInner::InvalidByte(byte) =&gt; {
                <span class="kw">if </span>byte &lt; <span class="number">128 </span>{
                    <span class="macro">write!</span>(f, <span class="string">"invalid hex character: {:?}"</span>, byte <span class="kw">as </span>char)
                } <span class="kw">else </span>{
                    <span class="macro">write!</span>(f, <span class="string">"invalid hex character: 0x{:x}"</span>, byte)
                }
            }
            HexErrorInner::InvalidLen(len) =&gt; {
                <span class="macro">write!</span>(f, <span class="string">"expected 64 hex bytes, received {}"</span>, len)
            }
        }
    }
}

<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">impl </span>std::error::Error <span class="kw">for </span>HexError {}

<span class="comment">// Each chunk or parent node can produce either a 32-byte chaining value or, by
// setting the ROOT flag, any number of final output bytes. The Output struct
// captures the state just prior to choosing between those two possibilities.
</span><span class="attr">#[cfg_attr(feature = <span class="string">"zeroize"</span>, derive(zeroize::Zeroize))]
#[derive(Clone)]
</span><span class="kw">struct </span>Output {
    input_chaining_value: CVWords,
    block: [u8; <span class="number">64</span>],
    block_len: u8,
    counter: u64,
    flags: u8,
    <span class="attr">#[cfg_attr(feature = <span class="string">"zeroize"</span>, zeroize(skip))]
    </span>platform: Platform,
}

<span class="kw">impl </span>Output {
    <span class="kw">fn </span>chaining_value(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; CVBytes {
        <span class="kw">let </span><span class="kw-2">mut </span>cv = <span class="self">self</span>.input_chaining_value;
        <span class="self">self</span>.platform.compress_in_place(
            <span class="kw-2">&amp;mut </span>cv,
            <span class="kw-2">&amp;</span><span class="self">self</span>.block,
            <span class="self">self</span>.block_len,
            <span class="self">self</span>.counter,
            <span class="self">self</span>.flags,
        );
        platform::le_bytes_from_words_32(<span class="kw-2">&amp;</span>cv)
    }

    <span class="kw">fn </span>root_hash(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Hash {
        <span class="macro">debug_assert_eq!</span>(<span class="self">self</span>.counter, <span class="number">0</span>);
        <span class="kw">let </span><span class="kw-2">mut </span>cv = <span class="self">self</span>.input_chaining_value;
        <span class="self">self</span>.platform
            .compress_in_place(<span class="kw-2">&amp;mut </span>cv, <span class="kw-2">&amp;</span><span class="self">self</span>.block, <span class="self">self</span>.block_len, <span class="number">0</span>, <span class="self">self</span>.flags | ROOT);
        Hash(platform::le_bytes_from_words_32(<span class="kw-2">&amp;</span>cv))
    }

    <span class="kw">fn </span>root_output_block(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; [u8; <span class="number">2 </span>* OUT_LEN] {
        <span class="self">self</span>.platform.compress_xof(
            <span class="kw-2">&amp;</span><span class="self">self</span>.input_chaining_value,
            <span class="kw-2">&amp;</span><span class="self">self</span>.block,
            <span class="self">self</span>.block_len,
            <span class="self">self</span>.counter,
            <span class="self">self</span>.flags | ROOT,
        )
    }
}

<span class="attr">#[derive(Clone)]
#[cfg_attr(feature = <span class="string">"zeroize"</span>, derive(zeroize::Zeroize))]
</span><span class="kw">struct </span>ChunkState {
    cv: CVWords,
    chunk_counter: u64,
    buf: [u8; BLOCK_LEN],
    buf_len: u8,
    blocks_compressed: u8,
    flags: u8,
    <span class="attr">#[cfg_attr(feature = <span class="string">"zeroize"</span>, zeroize(skip))]
    </span>platform: Platform,
}

<span class="kw">impl </span>ChunkState {
    <span class="kw">fn </span>new(key: <span class="kw-2">&amp;</span>CVWords, chunk_counter: u64, flags: u8, platform: Platform) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            cv: <span class="kw-2">*</span>key,
            chunk_counter,
            buf: [<span class="number">0</span>; BLOCK_LEN],
            buf_len: <span class="number">0</span>,
            blocks_compressed: <span class="number">0</span>,
            flags,
            platform,
        }
    }

    <span class="kw">fn </span>len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        BLOCK_LEN * <span class="self">self</span>.blocks_compressed <span class="kw">as </span>usize + <span class="self">self</span>.buf_len <span class="kw">as </span>usize
    }

    <span class="kw">fn </span>fill_buf(<span class="kw-2">&amp;mut </span><span class="self">self</span>, input: <span class="kw-2">&amp;mut &amp;</span>[u8]) {
        <span class="kw">let </span>want = BLOCK_LEN - <span class="self">self</span>.buf_len <span class="kw">as </span>usize;
        <span class="kw">let </span>take = cmp::min(want, input.len());
        <span class="self">self</span>.buf[<span class="self">self</span>.buf_len <span class="kw">as </span>usize..][..take].copy_from_slice(<span class="kw-2">&amp;</span>input[..take]);
        <span class="self">self</span>.buf_len += take <span class="kw">as </span>u8;
        <span class="kw-2">*</span>input = <span class="kw-2">&amp;</span>input[take..];
    }

    <span class="kw">fn </span>start_flag(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u8 {
        <span class="kw">if </span><span class="self">self</span>.blocks_compressed == <span class="number">0 </span>{
            CHUNK_START
        } <span class="kw">else </span>{
            <span class="number">0
        </span>}
    }

    <span class="comment">// Try to avoid buffering as much as possible, by compressing directly from
    // the input slice when full blocks are available.
    </span><span class="kw">fn </span>update(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw-2">mut </span>input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="kw">if </span><span class="self">self</span>.buf_len &gt; <span class="number">0 </span>{
            <span class="self">self</span>.fill_buf(<span class="kw-2">&amp;mut </span>input);
            <span class="kw">if </span>!input.is_empty() {
                <span class="macro">debug_assert_eq!</span>(<span class="self">self</span>.buf_len <span class="kw">as </span>usize, BLOCK_LEN);
                <span class="kw">let </span>block_flags = <span class="self">self</span>.flags | <span class="self">self</span>.start_flag(); <span class="comment">// borrowck
                </span><span class="self">self</span>.platform.compress_in_place(
                    <span class="kw-2">&amp;mut </span><span class="self">self</span>.cv,
                    <span class="kw-2">&amp;</span><span class="self">self</span>.buf,
                    BLOCK_LEN <span class="kw">as </span>u8,
                    <span class="self">self</span>.chunk_counter,
                    block_flags,
                );
                <span class="self">self</span>.buf_len = <span class="number">0</span>;
                <span class="self">self</span>.buf = [<span class="number">0</span>; BLOCK_LEN];
                <span class="self">self</span>.blocks_compressed += <span class="number">1</span>;
            }
        }

        <span class="kw">while </span>input.len() &gt; BLOCK_LEN {
            <span class="macro">debug_assert_eq!</span>(<span class="self">self</span>.buf_len, <span class="number">0</span>);
            <span class="kw">let </span>block_flags = <span class="self">self</span>.flags | <span class="self">self</span>.start_flag(); <span class="comment">// borrowck
            </span><span class="self">self</span>.platform.compress_in_place(
                <span class="kw-2">&amp;mut </span><span class="self">self</span>.cv,
                <span class="macro">array_ref!</span>(input, <span class="number">0</span>, BLOCK_LEN),
                BLOCK_LEN <span class="kw">as </span>u8,
                <span class="self">self</span>.chunk_counter,
                block_flags,
            );
            <span class="self">self</span>.blocks_compressed += <span class="number">1</span>;
            input = <span class="kw-2">&amp;</span>input[BLOCK_LEN..];
        }

        <span class="self">self</span>.fill_buf(<span class="kw-2">&amp;mut </span>input);
        <span class="macro">debug_assert!</span>(input.is_empty());
        <span class="macro">debug_assert!</span>(<span class="self">self</span>.len() &lt;= CHUNK_LEN);
        <span class="self">self
    </span>}

    <span class="kw">fn </span>output(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Output {
        <span class="kw">let </span>block_flags = <span class="self">self</span>.flags | <span class="self">self</span>.start_flag() | CHUNK_END;
        Output {
            input_chaining_value: <span class="self">self</span>.cv,
            block: <span class="self">self</span>.buf,
            block_len: <span class="self">self</span>.buf_len,
            counter: <span class="self">self</span>.chunk_counter,
            flags: block_flags,
            platform: <span class="self">self</span>.platform,
        }
    }
}

<span class="comment">// Don't derive(Debug), because the state may be secret.
</span><span class="kw">impl </span>fmt::Debug <span class="kw">for </span>ChunkState {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        f.debug_struct(<span class="string">"ChunkState"</span>)
            .field(<span class="string">"len"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.len())
            .field(<span class="string">"chunk_counter"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.chunk_counter)
            .field(<span class="string">"flags"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.flags)
            .field(<span class="string">"platform"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.platform)
            .finish()
    }
}

<span class="comment">// IMPLEMENTATION NOTE
// ===================
// The recursive function compress_subtree_wide(), implemented below, is the
// basis of high-performance BLAKE3. We use it both for all-at-once hashing,
// and for the incremental input with Hasher (though we have to be careful with
// subtree boundaries in the incremental case). compress_subtree_wide() applies
// several optimizations at the same time:
// - Multithreading with Rayon.
// - Parallel chunk hashing with SIMD.
// - Parallel parent hashing with SIMD. Note that while SIMD chunk hashing
//   maxes out at MAX_SIMD_DEGREE*CHUNK_LEN, parallel parent hashing continues
//   to benefit from larger inputs, because more levels of the tree benefit can
//   use full-width SIMD vectors for parent hashing. Without parallel parent
//   hashing, we lose about 10% of overall throughput on AVX2 and AVX-512.

</span><span class="doccomment">/// Undocumented and unstable, for benchmarks only.
</span><span class="attr">#[doc(hidden)]
#[derive(Clone, Copy)]
</span><span class="kw">pub enum </span>IncrementCounter {
    Yes,
    No,
}

<span class="kw">impl </span>IncrementCounter {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>yes(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="kw">match </span><span class="self">self </span>{
            IncrementCounter::Yes =&gt; <span class="bool-val">true</span>,
            IncrementCounter::No =&gt; <span class="bool-val">false</span>,
        }
    }
}

<span class="comment">// The largest power of two less than or equal to `n`, used for left_len()
// immediately below, and also directly in Hasher::update().
</span><span class="kw">fn </span>largest_power_of_two_leq(n: usize) -&gt; usize {
    ((n / <span class="number">2</span>) + <span class="number">1</span>).next_power_of_two()
}

<span class="comment">// Given some input larger than one chunk, return the number of bytes that
// should go in the left subtree. This is the largest power-of-2 number of
// chunks that leaves at least 1 byte for the right subtree.
</span><span class="kw">fn </span>left_len(content_len: usize) -&gt; usize {
    <span class="macro">debug_assert!</span>(content_len &gt; CHUNK_LEN);
    <span class="comment">// Subtract 1 to reserve at least one byte for the right side.
    </span><span class="kw">let </span>full_chunks = (content_len - <span class="number">1</span>) / CHUNK_LEN;
    largest_power_of_two_leq(full_chunks) * CHUNK_LEN
}

<span class="comment">// Use SIMD parallelism to hash up to MAX_SIMD_DEGREE chunks at the same time
// on a single thread. Write out the chunk chaining values and return the
// number of chunks hashed. These chunks are never the root and never empty;
// those cases use a different codepath.
</span><span class="kw">fn </span>compress_chunks_parallel(
    input: <span class="kw-2">&amp;</span>[u8],
    key: <span class="kw-2">&amp;</span>CVWords,
    chunk_counter: u64,
    flags: u8,
    platform: Platform,
    out: <span class="kw-2">&amp;mut </span>[u8],
) -&gt; usize {
    <span class="macro">debug_assert!</span>(!input.is_empty(), <span class="string">"empty chunks below the root"</span>);
    <span class="macro">debug_assert!</span>(input.len() &lt;= MAX_SIMD_DEGREE * CHUNK_LEN);

    <span class="kw">let </span><span class="kw-2">mut </span>chunks_exact = input.chunks_exact(CHUNK_LEN);
    <span class="kw">let </span><span class="kw-2">mut </span>chunks_array = ArrayVec::&lt;<span class="kw-2">&amp;</span>[u8; CHUNK_LEN], MAX_SIMD_DEGREE&gt;::new();
    <span class="kw">for </span>chunk <span class="kw">in </span><span class="kw-2">&amp;mut </span>chunks_exact {
        chunks_array.push(<span class="macro">array_ref!</span>(chunk, <span class="number">0</span>, CHUNK_LEN));
    }
    platform.hash_many(
        <span class="kw-2">&amp;</span>chunks_array,
        key,
        chunk_counter,
        IncrementCounter::Yes,
        flags,
        CHUNK_START,
        CHUNK_END,
        out,
    );

    <span class="comment">// Hash the remaining partial chunk, if there is one. Note that the empty
    // chunk (meaning the empty message) is a different codepath.
    </span><span class="kw">let </span>chunks_so_far = chunks_array.len();
    <span class="kw">if </span>!chunks_exact.remainder().is_empty() {
        <span class="kw">let </span>counter = chunk_counter + chunks_so_far <span class="kw">as </span>u64;
        <span class="kw">let </span><span class="kw-2">mut </span>chunk_state = ChunkState::new(key, counter, flags, platform);
        chunk_state.update(chunks_exact.remainder());
        <span class="kw-2">*</span><span class="macro">array_mut_ref!</span>(out, chunks_so_far * OUT_LEN, OUT_LEN) =
            chunk_state.output().chaining_value();
        chunks_so_far + <span class="number">1
    </span>} <span class="kw">else </span>{
        chunks_so_far
    }
}

<span class="comment">// Use SIMD parallelism to hash up to MAX_SIMD_DEGREE parents at the same time
// on a single thread. Write out the parent chaining values and return the
// number of parents hashed. (If there's an odd input chaining value left over,
// return it as an additional output.) These parents are never the root and
// never empty; those cases use a different codepath.
</span><span class="kw">fn </span>compress_parents_parallel(
    child_chaining_values: <span class="kw-2">&amp;</span>[u8],
    key: <span class="kw-2">&amp;</span>CVWords,
    flags: u8,
    platform: Platform,
    out: <span class="kw-2">&amp;mut </span>[u8],
) -&gt; usize {
    <span class="macro">debug_assert_eq!</span>(child_chaining_values.len() % OUT_LEN, <span class="number">0</span>, <span class="string">"wacky hash bytes"</span>);
    <span class="kw">let </span>num_children = child_chaining_values.len() / OUT_LEN;
    <span class="macro">debug_assert!</span>(num_children &gt;= <span class="number">2</span>, <span class="string">"not enough children"</span>);
    <span class="macro">debug_assert!</span>(num_children &lt;= <span class="number">2 </span>* MAX_SIMD_DEGREE_OR_2, <span class="string">"too many"</span>);

    <span class="kw">let </span><span class="kw-2">mut </span>parents_exact = child_chaining_values.chunks_exact(BLOCK_LEN);
    <span class="comment">// Use MAX_SIMD_DEGREE_OR_2 rather than MAX_SIMD_DEGREE here, because of
    // the requirements of compress_subtree_wide().
    </span><span class="kw">let </span><span class="kw-2">mut </span>parents_array = ArrayVec::&lt;<span class="kw-2">&amp;</span>[u8; BLOCK_LEN], MAX_SIMD_DEGREE_OR_2&gt;::new();
    <span class="kw">for </span>parent <span class="kw">in </span><span class="kw-2">&amp;mut </span>parents_exact {
        parents_array.push(<span class="macro">array_ref!</span>(parent, <span class="number">0</span>, BLOCK_LEN));
    }
    platform.hash_many(
        <span class="kw-2">&amp;</span>parents_array,
        key,
        <span class="number">0</span>, <span class="comment">// Parents always use counter 0.
        </span>IncrementCounter::No,
        flags | PARENT,
        <span class="number">0</span>, <span class="comment">// Parents have no start flags.
        </span><span class="number">0</span>, <span class="comment">// Parents have no end flags.
        </span>out,
    );

    <span class="comment">// If there's an odd child left over, it becomes an output.
    </span><span class="kw">let </span>parents_so_far = parents_array.len();
    <span class="kw">if </span>!parents_exact.remainder().is_empty() {
        out[parents_so_far * OUT_LEN..][..OUT_LEN].copy_from_slice(parents_exact.remainder());
        parents_so_far + <span class="number">1
    </span>} <span class="kw">else </span>{
        parents_so_far
    }
}

<span class="comment">// The wide helper function returns (writes out) an array of chaining values
// and returns the length of that array. The number of chaining values returned
// is the dynamically detected SIMD degree, at most MAX_SIMD_DEGREE. Or fewer,
// if the input is shorter than that many chunks. The reason for maintaining a
// wide array of chaining values going back up the tree, is to allow the
// implementation to hash as many parents in parallel as possible.
//
// As a special case when the SIMD degree is 1, this function will still return
// at least 2 outputs. This guarantees that this function doesn't perform the
// root compression. (If it did, it would use the wrong flags, and also we
// wouldn't be able to implement extendable output.) Note that this function is
// not used when the whole input is only 1 chunk long; that's a different
// codepath.
//
// Why not just have the caller split the input on the first update(), instead
// of implementing this special rule? Because we don't want to limit SIMD or
// multithreading parallelism for that update().
</span><span class="kw">fn </span>compress_subtree_wide&lt;J: join::Join&gt;(
    input: <span class="kw-2">&amp;</span>[u8],
    key: <span class="kw-2">&amp;</span>CVWords,
    chunk_counter: u64,
    flags: u8,
    platform: Platform,
    out: <span class="kw-2">&amp;mut </span>[u8],
) -&gt; usize {
    <span class="comment">// Note that the single chunk case does *not* bump the SIMD degree up to 2
    // when it is 1. This allows Rayon the option of multithreading even the
    // 2-chunk case, which can help performance on smaller platforms.
    </span><span class="kw">if </span>input.len() &lt;= platform.simd_degree() * CHUNK_LEN {
        <span class="kw">return </span>compress_chunks_parallel(input, key, chunk_counter, flags, platform, out);
    }

    <span class="comment">// With more than simd_degree chunks, we need to recurse. Start by dividing
    // the input into left and right subtrees. (Note that this is only optimal
    // as long as the SIMD degree is a power of 2. If we ever get a SIMD degree
    // of 3 or something, we'll need a more complicated strategy.)
    </span><span class="macro">debug_assert_eq!</span>(platform.simd_degree().count_ones(), <span class="number">1</span>, <span class="string">"power of 2"</span>);
    <span class="kw">let </span>(left, right) = input.split_at(left_len(input.len()));
    <span class="kw">let </span>right_chunk_counter = chunk_counter + (left.len() / CHUNK_LEN) <span class="kw">as </span>u64;

    <span class="comment">// Make space for the child outputs. Here we use MAX_SIMD_DEGREE_OR_2 to
    // account for the special case of returning 2 outputs when the SIMD degree
    // is 1.
    </span><span class="kw">let </span><span class="kw-2">mut </span>cv_array = [<span class="number">0</span>; <span class="number">2 </span>* MAX_SIMD_DEGREE_OR_2 * OUT_LEN];
    <span class="kw">let </span>degree = <span class="kw">if </span>left.len() == CHUNK_LEN {
        <span class="comment">// The "simd_degree=1 and we're at the leaf nodes" case.
        </span><span class="macro">debug_assert_eq!</span>(platform.simd_degree(), <span class="number">1</span>);
        <span class="number">1
    </span>} <span class="kw">else </span>{
        cmp::max(platform.simd_degree(), <span class="number">2</span>)
    };
    <span class="kw">let </span>(left_out, right_out) = cv_array.split_at_mut(degree * OUT_LEN);

    <span class="comment">// Recurse! For update_rayon(), this is where we take advantage of RayonJoin and use multiple
    // threads.
    </span><span class="kw">let </span>(left_n, right_n) = J::join(
        || compress_subtree_wide::&lt;J&gt;(left, key, chunk_counter, flags, platform, left_out),
        || compress_subtree_wide::&lt;J&gt;(right, key, right_chunk_counter, flags, platform, right_out),
    );

    <span class="comment">// The special case again. If simd_degree=1, then we'll have left_n=1 and
    // right_n=1. Rather than compressing them into a single output, return
    // them directly, to make sure we always have at least two outputs.
    </span><span class="macro">debug_assert_eq!</span>(left_n, degree);
    <span class="macro">debug_assert!</span>(right_n &gt;= <span class="number">1 </span>&amp;&amp; right_n &lt;= left_n);
    <span class="kw">if </span>left_n == <span class="number">1 </span>{
        out[..<span class="number">2 </span>* OUT_LEN].copy_from_slice(<span class="kw-2">&amp;</span>cv_array[..<span class="number">2 </span>* OUT_LEN]);
        <span class="kw">return </span><span class="number">2</span>;
    }

    <span class="comment">// Otherwise, do one layer of parent node compression.
    </span><span class="kw">let </span>num_children = left_n + right_n;
    compress_parents_parallel(
        <span class="kw-2">&amp;</span>cv_array[..num_children * OUT_LEN],
        key,
        flags,
        platform,
        out,
    )
}

<span class="comment">// Hash a subtree with compress_subtree_wide(), and then condense the resulting
// list of chaining values down to a single parent node. Don't compress that
// last parent node, however. Instead, return its message bytes (the
// concatenated chaining values of its children). This is necessary when the
// first call to update() supplies a complete subtree, because the topmost
// parent node of that subtree could end up being the root. It's also necessary
// for extended output in the general case.
//
// As with compress_subtree_wide(), this function is not used on inputs of 1
// chunk or less. That's a different codepath.
</span><span class="kw">fn </span>compress_subtree_to_parent_node&lt;J: join::Join&gt;(
    input: <span class="kw-2">&amp;</span>[u8],
    key: <span class="kw-2">&amp;</span>CVWords,
    chunk_counter: u64,
    flags: u8,
    platform: Platform,
) -&gt; [u8; BLOCK_LEN] {
    <span class="macro">debug_assert!</span>(input.len() &gt; CHUNK_LEN);
    <span class="kw">let </span><span class="kw-2">mut </span>cv_array = [<span class="number">0</span>; MAX_SIMD_DEGREE_OR_2 * OUT_LEN];
    <span class="kw">let </span><span class="kw-2">mut </span>num_cvs =
        compress_subtree_wide::&lt;J&gt;(input, <span class="kw-2">&amp;</span>key, chunk_counter, flags, platform, <span class="kw-2">&amp;mut </span>cv_array);
    <span class="macro">debug_assert!</span>(num_cvs &gt;= <span class="number">2</span>);

    <span class="comment">// If MAX_SIMD_DEGREE is greater than 2 and there's enough input,
    // compress_subtree_wide() returns more than 2 chaining values. Condense
    // them into 2 by forming parent nodes repeatedly.
    </span><span class="kw">let </span><span class="kw-2">mut </span>out_array = [<span class="number">0</span>; MAX_SIMD_DEGREE_OR_2 * OUT_LEN / <span class="number">2</span>];
    <span class="kw">while </span>num_cvs &gt; <span class="number">2 </span>{
        <span class="kw">let </span>cv_slice = <span class="kw-2">&amp;</span>cv_array[..num_cvs * OUT_LEN];
        num_cvs = compress_parents_parallel(cv_slice, key, flags, platform, <span class="kw-2">&amp;mut </span>out_array);
        cv_array[..num_cvs * OUT_LEN].copy_from_slice(<span class="kw-2">&amp;</span>out_array[..num_cvs * OUT_LEN]);
    }
    <span class="kw-2">*</span><span class="macro">array_ref!</span>(cv_array, <span class="number">0</span>, <span class="number">2 </span>* OUT_LEN)
}

<span class="comment">// Hash a complete input all at once. Unlike compress_subtree_wide() and
// compress_subtree_to_parent_node(), this function handles the 1 chunk case.
</span><span class="kw">fn </span>hash_all_at_once&lt;J: join::Join&gt;(input: <span class="kw-2">&amp;</span>[u8], key: <span class="kw-2">&amp;</span>CVWords, flags: u8) -&gt; Output {
    <span class="kw">let </span>platform = Platform::detect();

    <span class="comment">// If the whole subtree is one chunk, hash it directly with a ChunkState.
    </span><span class="kw">if </span>input.len() &lt;= CHUNK_LEN {
        <span class="kw">return </span>ChunkState::new(key, <span class="number">0</span>, flags, platform)
            .update(input)
            .output();
    }

    <span class="comment">// Otherwise construct an Output object from the parent node returned by
    // compress_subtree_to_parent_node().
    </span>Output {
        input_chaining_value: <span class="kw-2">*</span>key,
        block: compress_subtree_to_parent_node::&lt;J&gt;(input, key, <span class="number">0</span>, flags, platform),
        block_len: BLOCK_LEN <span class="kw">as </span>u8,
        counter: <span class="number">0</span>,
        flags: flags | PARENT,
        platform,
    }
}

<span class="doccomment">/// The default hash function.
///
/// For an incremental version that accepts multiple writes, see
/// [`Hasher::update`].
///
/// For output sizes other than 32 bytes, see [`Hasher::finalize_xof`] and
/// [`OutputReader`].
///
/// This function is always single-threaded. For multithreading support, see
/// [`Hasher::update_rayon`](struct.Hasher.html#method.update_rayon).
</span><span class="kw">pub fn </span>hash(input: <span class="kw-2">&amp;</span>[u8]) -&gt; Hash {
    hash_all_at_once::&lt;join::SerialJoin&gt;(input, IV, <span class="number">0</span>).root_hash()
}

<span class="doccomment">/// The keyed hash function.
///
/// This is suitable for use as a message authentication code, for example to
/// replace an HMAC instance. In that use case, the constant-time equality
/// checking provided by [`Hash`](struct.Hash.html) is almost always a security
/// requirement, and callers need to be careful not to compare MACs as raw
/// bytes.
///
/// For output sizes other than 32 bytes, see [`Hasher::new_keyed`],
/// [`Hasher::finalize_xof`], and [`OutputReader`].
///
/// This function is always single-threaded. For multithreading support, see
/// [`Hasher::new_keyed`] and
/// [`Hasher::update_rayon`](struct.Hasher.html#method.update_rayon).
</span><span class="kw">pub fn </span>keyed_hash(key: <span class="kw-2">&amp;</span>[u8; KEY_LEN], input: <span class="kw-2">&amp;</span>[u8]) -&gt; Hash {
    <span class="kw">let </span>key_words = platform::words_from_le_bytes_32(key);
    hash_all_at_once::&lt;join::SerialJoin&gt;(input, <span class="kw-2">&amp;</span>key_words, KEYED_HASH).root_hash()
}

<span class="doccomment">/// The key derivation function.
///
/// Given cryptographic key material of any length and a context string of any
/// length, this function outputs a 32-byte derived subkey. **The context string
/// should be hardcoded, globally unique, and application-specific.** A good
/// default format for such strings is `"[application] [commit timestamp]
/// [purpose]"`, e.g., `"example.com 2019-12-25 16:18:03 session tokens v1"`.
///
/// Key derivation is important when you want to use the same key in multiple
/// algorithms or use cases. Using the same key with different cryptographic
/// algorithms is generally forbidden, and deriving a separate subkey for each
/// use case protects you from bad interactions. Derived keys also mitigate the
/// damage from one part of your application accidentally leaking its key.
///
/// As a rare exception to that general rule, however, it is possible to use
/// `derive_key` itself with key material that you are already using with
/// another algorithm. You might need to do this if you're adding features to
/// an existing application, which does not yet use key derivation internally.
/// However, you still must not share key material with algorithms that forbid
/// key reuse entirely, like a one-time pad. For more on this, see sections 6.2
/// and 7.8 of the [BLAKE3 paper](https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf).
///
/// Note that BLAKE3 is not a password hash, and **`derive_key` should never be
/// used with passwords.** Instead, use a dedicated password hash like
/// [Argon2]. Password hashes are entirely different from generic hash
/// functions, with opposite design requirements.
///
/// For output sizes other than 32 bytes, see [`Hasher::new_derive_key`],
/// [`Hasher::finalize_xof`], and [`OutputReader`].
///
/// This function is always single-threaded. For multithreading support, see
/// [`Hasher::new_derive_key`] and
/// [`Hasher::update_rayon`](struct.Hasher.html#method.update_rayon).
///
/// [Argon2]: https://en.wikipedia.org/wiki/Argon2
</span><span class="kw">pub fn </span>derive_key(context: <span class="kw-2">&amp;</span>str, key_material: <span class="kw-2">&amp;</span>[u8]) -&gt; [u8; OUT_LEN] {
    <span class="kw">let </span>context_key =
        hash_all_at_once::&lt;join::SerialJoin&gt;(context.as_bytes(), IV, DERIVE_KEY_CONTEXT)
            .root_hash();
    <span class="kw">let </span>context_key_words = platform::words_from_le_bytes_32(context_key.as_bytes());
    hash_all_at_once::&lt;join::SerialJoin&gt;(key_material, <span class="kw-2">&amp;</span>context_key_words, DERIVE_KEY_MATERIAL)
        .root_hash()
        .<span class="number">0
</span>}

<span class="kw">fn </span>parent_node_output(
    left_child: <span class="kw-2">&amp;</span>CVBytes,
    right_child: <span class="kw-2">&amp;</span>CVBytes,
    key: <span class="kw-2">&amp;</span>CVWords,
    flags: u8,
    platform: Platform,
) -&gt; Output {
    <span class="kw">let </span><span class="kw-2">mut </span>block = [<span class="number">0</span>; BLOCK_LEN];
    block[..<span class="number">32</span>].copy_from_slice(left_child);
    block[<span class="number">32</span>..].copy_from_slice(right_child);
    Output {
        input_chaining_value: <span class="kw-2">*</span>key,
        block,
        block_len: BLOCK_LEN <span class="kw">as </span>u8,
        counter: <span class="number">0</span>,
        flags: flags | PARENT,
        platform,
    }
}

<span class="doccomment">/// An incremental hash state that can accept any number of writes.
///
/// The `rayon` and `mmap` Cargo features enable additional methods on this
/// type related to multithreading and memory-mapped IO.
///
/// When the `traits-preview` Cargo feature is enabled, this type implements
/// several commonly used traits from the
/// [`digest`](https://crates.io/crates/digest) crate. However, those
/// traits aren't stable, and they're expected to change in incompatible ways
/// before that crate reaches 1.0. For that reason, this crate makes no SemVer
/// guarantees for this feature, and callers who use it should expect breaking
/// changes between patch versions.
///
/// # Examples
///
/// ```
/// # fn main() -&gt; Result&lt;(), Box&lt;dyn std::error::Error&gt;&gt; {
/// // Hash an input incrementally.
/// let mut hasher = blake3::Hasher::new();
/// hasher.update(b"foo");
/// hasher.update(b"bar");
/// hasher.update(b"baz");
/// assert_eq!(hasher.finalize(), blake3::hash(b"foobarbaz"));
///
/// // Extended output. OutputReader also implements Read and Seek.
/// # #[cfg(feature = "std")] {
/// let mut output = [0; 1000];
/// let mut output_reader = hasher.finalize_xof();
/// output_reader.fill(&amp;mut output);
/// assert_eq!(&amp;output[..32], blake3::hash(b"foobarbaz").as_bytes());
/// # }
/// # Ok(())
/// # }
/// ```
</span><span class="attr">#[derive(Clone)]
#[cfg_attr(feature = <span class="string">"zeroize"</span>, derive(zeroize::Zeroize))]
</span><span class="kw">pub struct </span>Hasher {
    key: CVWords,
    chunk_state: ChunkState,
    <span class="comment">// The stack size is MAX_DEPTH + 1 because we do lazy merging. For example,
    // with 7 chunks, we have 3 entries in the stack. Adding an 8th chunk
    // requires a 4th entry, rather than merging everything down to 1, because
    // we don't know whether more input is coming. This is different from how
    // the reference implementation does things.
    </span>cv_stack: ArrayVec&lt;CVBytes, { MAX_DEPTH + <span class="number">1 </span>}&gt;,
}

<span class="kw">impl </span>Hasher {
    <span class="kw">fn </span>new_internal(key: <span class="kw-2">&amp;</span>CVWords, flags: u8) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            key: <span class="kw-2">*</span>key,
            chunk_state: ChunkState::new(key, <span class="number">0</span>, flags, Platform::detect()),
            cv_stack: ArrayVec::new(),
        }
    }

    <span class="doccomment">/// Construct a new `Hasher` for the regular hash function.
    </span><span class="kw">pub fn </span>new() -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::new_internal(IV, <span class="number">0</span>)
    }

    <span class="doccomment">/// Construct a new `Hasher` for the keyed hash function. See
    /// [`keyed_hash`].
    ///
    /// [`keyed_hash`]: fn.keyed_hash.html
    </span><span class="kw">pub fn </span>new_keyed(key: <span class="kw-2">&amp;</span>[u8; KEY_LEN]) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>key_words = platform::words_from_le_bytes_32(key);
        <span class="self">Self</span>::new_internal(<span class="kw-2">&amp;</span>key_words, KEYED_HASH)
    }

    <span class="doccomment">/// Construct a new `Hasher` for the key derivation function. See
    /// [`derive_key`]. The context string should be hardcoded, globally
    /// unique, and application-specific.
    ///
    /// [`derive_key`]: fn.derive_key.html
    </span><span class="kw">pub fn </span>new_derive_key(context: <span class="kw-2">&amp;</span>str) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>context_key =
            hash_all_at_once::&lt;join::SerialJoin&gt;(context.as_bytes(), IV, DERIVE_KEY_CONTEXT)
                .root_hash();
        <span class="kw">let </span>context_key_words = platform::words_from_le_bytes_32(context_key.as_bytes());
        <span class="self">Self</span>::new_internal(<span class="kw-2">&amp;</span>context_key_words, DERIVE_KEY_MATERIAL)
    }

    <span class="doccomment">/// Reset the `Hasher` to its initial state.
    ///
    /// This is functionally the same as overwriting the `Hasher` with a new
    /// one, using the same key or context string if any.
    </span><span class="kw">pub fn </span>reset(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.chunk_state = ChunkState::new(
            <span class="kw-2">&amp;</span><span class="self">self</span>.key,
            <span class="number">0</span>,
            <span class="self">self</span>.chunk_state.flags,
            <span class="self">self</span>.chunk_state.platform,
        );
        <span class="self">self</span>.cv_stack.clear();
        <span class="self">self
    </span>}

    <span class="comment">// As described in push_cv() below, we do "lazy merging", delaying merges
    // until right before the next CV is about to be added. This is different
    // from the reference implementation. Another difference is that we aren't
    // always merging 1 chunk at a time. Instead, each CV might represent any
    // power-of-two number of chunks, as long as the smaller-above-larger stack
    // order is maintained. Instead of the "count the trailing 0-bits"
    // algorithm described in the spec, we use a "count the total number of
    // 1-bits" variant that doesn't require us to retain the subtree size of
    // the CV on top of the stack. The principle is the same: each CV that
    // should remain in the stack is represented by a 1-bit in the total number
    // of chunks (or bytes) so far.
    </span><span class="kw">fn </span>merge_cv_stack(<span class="kw-2">&amp;mut </span><span class="self">self</span>, total_len: u64) {
        <span class="kw">let </span>post_merge_stack_len = total_len.count_ones() <span class="kw">as </span>usize;
        <span class="kw">while </span><span class="self">self</span>.cv_stack.len() &gt; post_merge_stack_len {
            <span class="kw">let </span>right_child = <span class="self">self</span>.cv_stack.pop().unwrap();
            <span class="kw">let </span>left_child = <span class="self">self</span>.cv_stack.pop().unwrap();
            <span class="kw">let </span>parent_output = parent_node_output(
                <span class="kw-2">&amp;</span>left_child,
                <span class="kw-2">&amp;</span>right_child,
                <span class="kw-2">&amp;</span><span class="self">self</span>.key,
                <span class="self">self</span>.chunk_state.flags,
                <span class="self">self</span>.chunk_state.platform,
            );
            <span class="self">self</span>.cv_stack.push(parent_output.chaining_value());
        }
    }

    <span class="comment">// In reference_impl.rs, we merge the new CV with existing CVs from the
    // stack before pushing it. We can do that because we know more input is
    // coming, so we know none of the merges are root.
    //
    // This setting is different. We want to feed as much input as possible to
    // compress_subtree_wide(), without setting aside anything for the
    // chunk_state. If the user gives us 64 KiB, we want to parallelize over
    // all 64 KiB at once as a single subtree, if at all possible.
    //
    // This leads to two problems:
    // 1) This 64 KiB input might be the only call that ever gets made to
    //    update. In this case, the root node of the 64 KiB subtree would be
    //    the root node of the whole tree, and it would need to be ROOT
    //    finalized. We can't compress it until we know.
    // 2) This 64 KiB input might complete a larger tree, whose root node is
    //    similarly going to be the the root of the whole tree. For example,
    //    maybe we have 196 KiB (that is, 128 + 64) hashed so far. We can't
    //    compress the node at the root of the 256 KiB subtree until we know
    //    how to finalize it.
    //
    // The second problem is solved with "lazy merging". That is, when we're
    // about to add a CV to the stack, we don't merge it with anything first,
    // as the reference impl does. Instead we do merges using the *previous* CV
    // that was added, which is sitting on top of the stack, and we put the new
    // CV (unmerged) on top of the stack afterwards. This guarantees that we
    // never merge the root node until finalize().
    //
    // Solving the first problem requires an additional tool,
    // compress_subtree_to_parent_node(). That function always returns the top
    // *two* chaining values of the subtree it's compressing. We then do lazy
    // merging with each of them separately, so that the second CV will always
    // remain unmerged. (That also helps us support extendable output when
    // we're hashing an input all-at-once.)
    </span><span class="kw">fn </span>push_cv(<span class="kw-2">&amp;mut </span><span class="self">self</span>, new_cv: <span class="kw-2">&amp;</span>CVBytes, chunk_counter: u64) {
        <span class="self">self</span>.merge_cv_stack(chunk_counter);
        <span class="self">self</span>.cv_stack.push(<span class="kw-2">*</span>new_cv);
    }

    <span class="doccomment">/// Add input bytes to the hash state. You can call this any number of times.
    ///
    /// This method is always single-threaded. For multithreading support, see
    /// [`update_rayon`](#method.update_rayon) (enabled with the `rayon` Cargo feature).
    ///
    /// Note that the degree of SIMD parallelism that `update` can use is limited by the size of
    /// this input buffer. See [`update_reader`](#method.update_reader).
    </span><span class="kw">pub fn </span>update(<span class="kw-2">&amp;mut </span><span class="self">self</span>, input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.update_with_join::&lt;join::SerialJoin&gt;(input)
    }

    <span class="kw">fn </span>update_with_join&lt;J: join::Join&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw-2">mut </span>input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="comment">// If we have some partial chunk bytes in the internal chunk_state, we
        // need to finish that chunk first.
        </span><span class="kw">if </span><span class="self">self</span>.chunk_state.len() &gt; <span class="number">0 </span>{
            <span class="kw">let </span>want = CHUNK_LEN - <span class="self">self</span>.chunk_state.len();
            <span class="kw">let </span>take = cmp::min(want, input.len());
            <span class="self">self</span>.chunk_state.update(<span class="kw-2">&amp;</span>input[..take]);
            input = <span class="kw-2">&amp;</span>input[take..];
            <span class="kw">if </span>!input.is_empty() {
                <span class="comment">// We've filled the current chunk, and there's more input
                // coming, so we know it's not the root and we can finalize it.
                // Then we'll proceed to hashing whole chunks below.
                </span><span class="macro">debug_assert_eq!</span>(<span class="self">self</span>.chunk_state.len(), CHUNK_LEN);
                <span class="kw">let </span>chunk_cv = <span class="self">self</span>.chunk_state.output().chaining_value();
                <span class="self">self</span>.push_cv(<span class="kw-2">&amp;</span>chunk_cv, <span class="self">self</span>.chunk_state.chunk_counter);
                <span class="self">self</span>.chunk_state = ChunkState::new(
                    <span class="kw-2">&amp;</span><span class="self">self</span>.key,
                    <span class="self">self</span>.chunk_state.chunk_counter + <span class="number">1</span>,
                    <span class="self">self</span>.chunk_state.flags,
                    <span class="self">self</span>.chunk_state.platform,
                );
            } <span class="kw">else </span>{
                <span class="kw">return </span><span class="self">self</span>;
            }
        }

        <span class="comment">// Now the chunk_state is clear, and we have more input. If there's
        // more than a single chunk (so, definitely not the root chunk), hash
        // the largest whole subtree we can, with the full benefits of SIMD and
        // multithreading parallelism. Two restrictions:
        // - The subtree has to be a power-of-2 number of chunks. Only subtrees
        //   along the right edge can be incomplete, and we don't know where
        //   the right edge is going to be until we get to finalize().
        // - The subtree must evenly divide the total number of chunks up until
        //   this point (if total is not 0). If the current incomplete subtree
        //   is only waiting for 1 more chunk, we can't hash a subtree of 4
        //   chunks. We have to complete the current subtree first.
        // Because we might need to break up the input to form powers of 2, or
        // to evenly divide what we already have, this part runs in a loop.
        </span><span class="kw">while </span>input.len() &gt; CHUNK_LEN {
            <span class="macro">debug_assert_eq!</span>(<span class="self">self</span>.chunk_state.len(), <span class="number">0</span>, <span class="string">"no partial chunk data"</span>);
            <span class="macro">debug_assert_eq!</span>(CHUNK_LEN.count_ones(), <span class="number">1</span>, <span class="string">"power of 2 chunk len"</span>);
            <span class="kw">let </span><span class="kw-2">mut </span>subtree_len = largest_power_of_two_leq(input.len());
            <span class="kw">let </span>count_so_far = <span class="self">self</span>.chunk_state.chunk_counter * CHUNK_LEN <span class="kw">as </span>u64;
            <span class="comment">// Shrink the subtree_len until it evenly divides the count so far.
            // We know that subtree_len itself is a power of 2, so we can use a
            // bitmasking trick instead of an actual remainder operation. (Note
            // that if the caller consistently passes power-of-2 inputs of the
            // same size, as is hopefully typical, this loop condition will
            // always fail, and subtree_len will always be the full length of
            // the input.)
            //
            // An aside: We don't have to shrink subtree_len quite this much.
            // For example, if count_so_far is 1, we could pass 2 chunks to
            // compress_subtree_to_parent_node. Since we'll get 2 CVs back,
            // we'll still get the right answer in the end, and we might get to
            // use 2-way SIMD parallelism. The problem with this optimization,
            // is that it gets us stuck always hashing 2 chunks. The total
            // number of chunks will remain odd, and we'll never graduate to
            // higher degrees of parallelism. See
            // https://github.com/BLAKE3-team/BLAKE3/issues/69.
            </span><span class="kw">while </span>(subtree_len - <span class="number">1</span>) <span class="kw">as </span>u64 &amp; count_so_far != <span class="number">0 </span>{
                subtree_len /= <span class="number">2</span>;
            }
            <span class="comment">// The shrunken subtree_len might now be 1 chunk long. If so, hash
            // that one chunk by itself. Otherwise, compress the subtree into a
            // pair of CVs.
            </span><span class="kw">let </span>subtree_chunks = (subtree_len / CHUNK_LEN) <span class="kw">as </span>u64;
            <span class="kw">if </span>subtree_len &lt;= CHUNK_LEN {
                <span class="macro">debug_assert_eq!</span>(subtree_len, CHUNK_LEN);
                <span class="self">self</span>.push_cv(
                    <span class="kw-2">&amp;</span>ChunkState::new(
                        <span class="kw-2">&amp;</span><span class="self">self</span>.key,
                        <span class="self">self</span>.chunk_state.chunk_counter,
                        <span class="self">self</span>.chunk_state.flags,
                        <span class="self">self</span>.chunk_state.platform,
                    )
                    .update(<span class="kw-2">&amp;</span>input[..subtree_len])
                    .output()
                    .chaining_value(),
                    <span class="self">self</span>.chunk_state.chunk_counter,
                );
            } <span class="kw">else </span>{
                <span class="comment">// This is the high-performance happy path, though getting here
                // depends on the caller giving us a long enough input.
                </span><span class="kw">let </span>cv_pair = compress_subtree_to_parent_node::&lt;J&gt;(
                    <span class="kw-2">&amp;</span>input[..subtree_len],
                    <span class="kw-2">&amp;</span><span class="self">self</span>.key,
                    <span class="self">self</span>.chunk_state.chunk_counter,
                    <span class="self">self</span>.chunk_state.flags,
                    <span class="self">self</span>.chunk_state.platform,
                );
                <span class="kw">let </span>left_cv = <span class="macro">array_ref!</span>(cv_pair, <span class="number">0</span>, <span class="number">32</span>);
                <span class="kw">let </span>right_cv = <span class="macro">array_ref!</span>(cv_pair, <span class="number">32</span>, <span class="number">32</span>);
                <span class="comment">// Push the two CVs we received into the CV stack in order. Because
                // the stack merges lazily, this guarantees we aren't merging the
                // root.
                </span><span class="self">self</span>.push_cv(left_cv, <span class="self">self</span>.chunk_state.chunk_counter);
                <span class="self">self</span>.push_cv(
                    right_cv,
                    <span class="self">self</span>.chunk_state.chunk_counter + (subtree_chunks / <span class="number">2</span>),
                );
            }
            <span class="self">self</span>.chunk_state.chunk_counter += subtree_chunks;
            input = <span class="kw-2">&amp;</span>input[subtree_len..];
        }

        <span class="comment">// What remains is 1 chunk or less. Add it to the chunk state.
        </span><span class="macro">debug_assert!</span>(input.len() &lt;= CHUNK_LEN);
        <span class="kw">if </span>!input.is_empty() {
            <span class="self">self</span>.chunk_state.update(input);
            <span class="comment">// Having added some input to the chunk_state, we know what's in
            // the CV stack won't become the root node, and we can do an extra
            // merge. This simplifies finalize().
            </span><span class="self">self</span>.merge_cv_stack(<span class="self">self</span>.chunk_state.chunk_counter);
        }

        <span class="self">self
    </span>}

    <span class="kw">fn </span>final_output(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Output {
        <span class="comment">// If the current chunk is the only chunk, that makes it the root node
        // also. Convert it directly into an Output. Otherwise, we need to
        // merge subtrees below.
        </span><span class="kw">if </span><span class="self">self</span>.cv_stack.is_empty() {
            <span class="macro">debug_assert_eq!</span>(<span class="self">self</span>.chunk_state.chunk_counter, <span class="number">0</span>);
            <span class="kw">return </span><span class="self">self</span>.chunk_state.output();
        }

        <span class="comment">// If there are any bytes in the ChunkState, finalize that chunk and
        // merge its CV with everything in the CV stack. In that case, the work
        // we did at the end of update() above guarantees that the stack
        // doesn't contain any unmerged subtrees that need to be merged first.
        // (This is important, because if there were two chunk hashes sitting
        // on top of the stack, they would need to merge with each other, and
        // merging a new chunk hash into them would be incorrect.)
        //
        // If there are no bytes in the ChunkState, we'll merge what's already
        // in the stack. In this case it's fine if there are unmerged chunks on
        // top, because we'll merge them with each other. Note that the case of
        // the empty chunk is taken care of above.
        </span><span class="kw">let </span><span class="kw-2">mut </span>output: Output;
        <span class="kw">let </span><span class="kw-2">mut </span>num_cvs_remaining = <span class="self">self</span>.cv_stack.len();
        <span class="kw">if </span><span class="self">self</span>.chunk_state.len() &gt; <span class="number">0 </span>{
            <span class="macro">debug_assert_eq!</span>(
                <span class="self">self</span>.cv_stack.len(),
                <span class="self">self</span>.chunk_state.chunk_counter.count_ones() <span class="kw">as </span>usize,
                <span class="string">"cv stack does not need a merge"
            </span>);
            output = <span class="self">self</span>.chunk_state.output();
        } <span class="kw">else </span>{
            <span class="macro">debug_assert!</span>(<span class="self">self</span>.cv_stack.len() &gt;= <span class="number">2</span>);
            output = parent_node_output(
                <span class="kw-2">&amp;</span><span class="self">self</span>.cv_stack[num_cvs_remaining - <span class="number">2</span>],
                <span class="kw-2">&amp;</span><span class="self">self</span>.cv_stack[num_cvs_remaining - <span class="number">1</span>],
                <span class="kw-2">&amp;</span><span class="self">self</span>.key,
                <span class="self">self</span>.chunk_state.flags,
                <span class="self">self</span>.chunk_state.platform,
            );
            num_cvs_remaining -= <span class="number">2</span>;
        }
        <span class="kw">while </span>num_cvs_remaining &gt; <span class="number">0 </span>{
            output = parent_node_output(
                <span class="kw-2">&amp;</span><span class="self">self</span>.cv_stack[num_cvs_remaining - <span class="number">1</span>],
                <span class="kw-2">&amp;</span>output.chaining_value(),
                <span class="kw-2">&amp;</span><span class="self">self</span>.key,
                <span class="self">self</span>.chunk_state.flags,
                <span class="self">self</span>.chunk_state.platform,
            );
            num_cvs_remaining -= <span class="number">1</span>;
        }
        output
    }

    <span class="doccomment">/// Finalize the hash state and return the [`Hash`](struct.Hash.html) of
    /// the input.
    ///
    /// This method is idempotent. Calling it twice will give the same result.
    /// You can also add more input and finalize again.
    </span><span class="kw">pub fn </span>finalize(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Hash {
        <span class="self">self</span>.final_output().root_hash()
    }

    <span class="doccomment">/// Finalize the hash state and return an [`OutputReader`], which can
    /// supply any number of output bytes.
    ///
    /// This method is idempotent. Calling it twice will give the same result.
    /// You can also add more input and finalize again.
    ///
    /// [`OutputReader`]: struct.OutputReader.html
    </span><span class="kw">pub fn </span>finalize_xof(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; OutputReader {
        OutputReader::new(<span class="self">self</span>.final_output())
    }

    <span class="doccomment">/// Return the total number of bytes hashed so far.
    </span><span class="kw">pub fn </span>count(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u64 {
        <span class="self">self</span>.chunk_state.chunk_counter * CHUNK_LEN <span class="kw">as </span>u64 + <span class="self">self</span>.chunk_state.len() <span class="kw">as </span>u64
    }

    <span class="doccomment">/// As [`update`](Hasher::update), but reading from a
    /// [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) implementation.
    ///
    /// [`Hasher`] implements
    /// [`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html), so it's possible to
    /// use [`std::io::copy`](https://doc.rust-lang.org/std/io/fn.copy.html) to update a [`Hasher`]
    /// from any reader. Unfortunately, this standard approach can limit performance, because
    /// `copy` currently uses an internal 8 KiB buffer that isn't big enough to take advantage of
    /// all SIMD instruction sets. (In particular, [AVX-512](https://en.wikipedia.org/wiki/AVX-512)
    /// needs a 16 KiB buffer.) `update_reader` avoids this performance problem and is slightly
    /// more convenient.
    ///
    /// The internal buffer size this method uses may change at any time, and it may be different
    /// for different targets. The only guarantee is that it will be large enough for all of this
    /// crate's SIMD implementations on the current platform.
    ///
    /// The most common implementer of
    /// [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) might be
    /// [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html), but note that memory
    /// mapping can be faster than this method for hashing large files. See
    /// [`update_mmap`](Hasher::update_mmap) and [`update_mmap_rayon`](Hasher::update_mmap_rayon),
    /// which require the `mmap` and (for the latter) `rayon` Cargo features.
    ///
    /// This method requires the `std` Cargo feature, which is enabled by default.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// # use std::io;
    /// # fn main() -&gt; io::Result&lt;()&gt; {
    /// // Hash standard input.
    /// let mut hasher = blake3::Hasher::new();
    /// hasher.update_reader(std::io::stdin().lock())?;
    /// println!("{}", hasher.finalize());
    /// # Ok(())
    /// # }
    /// ```
    </span><span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
    </span><span class="kw">pub fn </span>update_reader(<span class="kw-2">&amp;mut </span><span class="self">self</span>, reader: <span class="kw">impl </span>std::io::Read) -&gt; std::io::Result&lt;<span class="kw-2">&amp;mut </span><span class="self">Self</span>&gt; {
        io::copy_wide(reader, <span class="self">self</span>)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(<span class="self">self</span>)
    }

    <span class="doccomment">/// As [`update`](Hasher::update), but using Rayon-based multithreading
    /// internally.
    ///
    /// This method is gated by the `rayon` Cargo feature, which is disabled by
    /// default but enabled on [docs.rs](https://docs.rs).
    ///
    /// To get any performance benefit from multithreading, the input buffer
    /// needs to be large. As a rule of thumb on x86_64, `update_rayon` is
    /// _slower_ than `update` for inputs under 128 KiB. That threshold varies
    /// quite a lot across different processors, and it's important to benchmark
    /// your specific use case. See also the performance warning associated with
    /// [`update_mmap_rayon`](Hasher::update_mmap_rayon).
    ///
    /// If you already have a large buffer in memory, and you want to hash it
    /// with multiple threads, this method is a good option. However, reading a
    /// file into memory just to call this method can be a performance mistake,
    /// both because it requires lots of memory and because single-threaded
    /// reads can be slow. For hashing whole files, see
    /// [`update_mmap_rayon`](Hasher::update_mmap_rayon), which is gated by both
    /// the `rayon` and `mmap` Cargo features.
    </span><span class="attr">#[cfg(feature = <span class="string">"rayon"</span>)]
    </span><span class="kw">pub fn </span>update_rayon(<span class="kw-2">&amp;mut </span><span class="self">self</span>, input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.update_with_join::&lt;join::RayonJoin&gt;(input)
    }

    <span class="doccomment">/// As [`update`](Hasher::update), but reading the contents of a file using memory mapping.
    ///
    /// Not all files can be memory mapped, and memory mapping small files can be slower than
    /// reading them the usual way. In those cases, this method will fall back to standard file IO.
    /// The heuristic for whether to use memory mapping is currently very simple (file size &gt;=
    /// 16 KiB), and it might change at any time.
    ///
    /// Like [`update`](Hasher::update), this method is single-threaded. In this author's
    /// experience, memory mapping improves single-threaded performance by ~10% for large files
    /// that are already in cache. This probably varies between platforms, and as always it's a
    /// good idea to benchmark your own use case. In comparison, the multithreaded
    /// [`update_mmap_rayon`](Hasher::update_mmap_rayon) method can have a much larger impact on
    /// performance.
    ///
    /// There's a correctness reason that this method takes
    /// [`Path`](https://doc.rust-lang.org/stable/std/path/struct.Path.html) instead of
    /// [`File`](https://doc.rust-lang.org/std/fs/struct.File.html): reading from a memory-mapped
    /// file ignores the seek position of the original file handle (it neither respects the current
    /// position nor updates the position). This difference in behavior would've caused
    /// `update_mmap` and [`update_reader`](Hasher::update_reader) to give different answers and
    /// have different side effects in some cases. Taking a
    /// [`Path`](https://doc.rust-lang.org/stable/std/path/struct.Path.html) avoids this problem by
    /// making it clear that a new [`File`](https://doc.rust-lang.org/std/fs/struct.File.html) is
    /// opened internally.
    ///
    /// This method requires the `mmap` Cargo feature, which is disabled by default but enabled on
    /// [docs.rs](https://docs.rs).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::io;
    /// # use std::path::Path;
    /// # fn main() -&gt; io::Result&lt;()&gt; {
    /// let path = Path::new("file.dat");
    /// let mut hasher = blake3::Hasher::new();
    /// hasher.update_mmap(path)?;
    /// println!("{}", hasher.finalize());
    /// # Ok(())
    /// # }
    /// ```
    </span><span class="attr">#[cfg(feature = <span class="string">"mmap"</span>)]
    </span><span class="kw">pub fn </span>update_mmap(<span class="kw-2">&amp;mut </span><span class="self">self</span>, path: <span class="kw">impl </span>AsRef&lt;std::path::Path&gt;) -&gt; std::io::Result&lt;<span class="kw-2">&amp;mut </span><span class="self">Self</span>&gt; {
        <span class="kw">let </span>file = std::fs::File::open(path.as_ref())<span class="question-mark">?</span>;
        <span class="kw">if let </span><span class="prelude-val">Some</span>(mmap) = io::maybe_mmap_file(<span class="kw-2">&amp;</span>file)<span class="question-mark">? </span>{
            <span class="self">self</span>.update(<span class="kw-2">&amp;</span>mmap);
        } <span class="kw">else </span>{
            io::copy_wide(<span class="kw-2">&amp;</span>file, <span class="self">self</span>)<span class="question-mark">?</span>;
        }
        <span class="prelude-val">Ok</span>(<span class="self">self</span>)
    }

    <span class="doccomment">/// As [`update_rayon`](Hasher::update_rayon), but reading the contents of a file using
    /// memory mapping. This is the default behavior of `b3sum`.
    ///
    /// For large files that are likely to be in cache, this can be much faster than
    /// single-threaded hashing. When benchmarks report that BLAKE3 is 10x or 20x faster than other
    /// cryptographic hashes, this is usually what they're measuring. However...
    ///
    /// **Performance Warning:** There are cases where multithreading hurts performance. The worst
    /// case is [a large file on a spinning disk](https://github.com/BLAKE3-team/BLAKE3/issues/31),
    /// where simultaneous reads from multiple threads can cause "thrashing" (i.e. the disk spends
    /// more time seeking around than reading data). Windows tends to be somewhat worse about this,
    /// in part because it's less likely than Linux to keep very large files in cache. More
    /// generally, if your CPU cores are already busy, then multithreading will add overhead
    /// without improving performance. If your code runs in different environments that you don't
    /// control and can't measure, then unfortunately there's no one-size-fits-all answer for
    /// whether multithreading is a good idea.
    ///
    /// The memory mapping behavior of this function is the same as
    /// [`update_mmap`](Hasher::update_mmap), and the heuristic for when to fall back to standard
    /// file IO might change at any time.
    ///
    /// This method requires both the `mmap` and `rayon` Cargo features, which are disabled by
    /// default but enabled on [docs.rs](https://docs.rs).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::io;
    /// # use std::path::Path;
    /// # fn main() -&gt; io::Result&lt;()&gt; {
    /// # #[cfg(feature = "rayon")]
    /// # {
    /// let path = Path::new("big_file.dat");
    /// let mut hasher = blake3::Hasher::new();
    /// hasher.update_mmap_rayon(path)?;
    /// println!("{}", hasher.finalize());
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    </span><span class="attr">#[cfg(feature = <span class="string">"mmap"</span>)]
    #[cfg(feature = <span class="string">"rayon"</span>)]
    </span><span class="kw">pub fn </span>update_mmap_rayon(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        path: <span class="kw">impl </span>AsRef&lt;std::path::Path&gt;,
    ) -&gt; std::io::Result&lt;<span class="kw-2">&amp;mut </span><span class="self">Self</span>&gt; {
        <span class="kw">let </span>file = std::fs::File::open(path.as_ref())<span class="question-mark">?</span>;
        <span class="kw">if let </span><span class="prelude-val">Some</span>(mmap) = io::maybe_mmap_file(<span class="kw-2">&amp;</span>file)<span class="question-mark">? </span>{
            <span class="self">self</span>.update_rayon(<span class="kw-2">&amp;</span>mmap);
        } <span class="kw">else </span>{
            io::copy_wide(<span class="kw-2">&amp;</span>file, <span class="self">self</span>)<span class="question-mark">?</span>;
        }
        <span class="prelude-val">Ok</span>(<span class="self">self</span>)
    }
}

<span class="comment">// Don't derive(Debug), because the state may be secret.
</span><span class="kw">impl </span>fmt::Debug <span class="kw">for </span>Hasher {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        f.debug_struct(<span class="string">"Hasher"</span>)
            .field(<span class="string">"flags"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.chunk_state.flags)
            .field(<span class="string">"platform"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.chunk_state.platform)
            .finish()
    }
}

<span class="kw">impl </span>Default <span class="kw">for </span>Hasher {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::new()
    }
}

<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">impl </span>std::io::Write <span class="kw">for </span>Hasher {
    <span class="doccomment">/// This is equivalent to [`update`](#method.update).
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>write(<span class="kw-2">&amp;mut </span><span class="self">self</span>, input: <span class="kw-2">&amp;</span>[u8]) -&gt; std::io::Result&lt;usize&gt; {
        <span class="self">self</span>.update(input);
        <span class="prelude-val">Ok</span>(input.len())
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>flush(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; std::io::Result&lt;()&gt; {
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="doccomment">/// An incremental reader for extended output, returned by
/// [`Hasher::finalize_xof`](struct.Hasher.html#method.finalize_xof).
///
/// Shorter BLAKE3 outputs are prefixes of longer ones, and explicitly requesting a short output is
/// equivalent to truncating the default-length output. Note that this is a difference between
/// BLAKE2 and BLAKE3.
///
/// # Security notes
///
/// Outputs shorter than the default length of 32 bytes (256 bits) provide less security. An N-bit
/// BLAKE3 output is intended to provide N bits of first and second preimage resistance and N/2
/// bits of collision resistance, for any N up to 256. Longer outputs don't provide any additional
/// security.
///
/// Avoid relying on the secrecy of the output offset, that is, the number of output bytes read or
/// the arguments to [`seek`](struct.OutputReader.html#method.seek) or
/// [`set_position`](struct.OutputReader.html#method.set_position). [_Block-Cipher-Based Tree
/// Hashing_ by Aldo Gunsing](https://eprint.iacr.org/2022/283) shows that an attacker who knows
/// both the message and the key (if any) can easily determine the offset of an extended output.
/// For comparison, AES-CTR has a similar property: if you know the key, you can decrypt a block
/// from an unknown position in the output stream to recover its block index. Callers with strong
/// secret keys aren't affected in practice, but secret offsets are a [design
/// smell](https://en.wikipedia.org/wiki/Design_smell) in any case.
</span><span class="attr">#[cfg_attr(feature = <span class="string">"zeroize"</span>, derive(zeroize::Zeroize))]
#[derive(Clone)]
</span><span class="kw">pub struct </span>OutputReader {
    inner: Output,
    position_within_block: u8,
}

<span class="kw">impl </span>OutputReader {
    <span class="kw">fn </span>new(inner: Output) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            inner,
            position_within_block: <span class="number">0</span>,
        }
    }

    <span class="doccomment">/// Fill a buffer with output bytes and advance the position of the
    /// `OutputReader`. This is equivalent to [`Read::read`], except that it
    /// doesn't return a `Result`. Both methods always fill the entire buffer.
    ///
    /// Note that `OutputReader` doesn't buffer output bytes internally, so
    /// calling `fill` repeatedly with a short-length or odd-length slice will
    /// end up performing the same compression multiple times. If you're
    /// reading output in a loop, prefer a slice length that's a multiple of
    /// 64.
    ///
    /// The maximum output size of BLAKE3 is 2&lt;sup&gt;64&lt;/sup&gt;-1 bytes. If you try
    /// to extract more than that, for example by seeking near the end and
    /// reading further, the behavior is unspecified.
    ///
    /// [`Read::read`]: #method.read
    </span><span class="kw">pub fn </span>fill(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw-2">mut </span>buf: <span class="kw-2">&amp;mut </span>[u8]) {
        <span class="kw">while </span>!buf.is_empty() {
            <span class="kw">let </span>block: [u8; BLOCK_LEN] = <span class="self">self</span>.inner.root_output_block();
            <span class="kw">let </span>output_bytes = <span class="kw-2">&amp;</span>block[<span class="self">self</span>.position_within_block <span class="kw">as </span>usize..];
            <span class="kw">let </span>take = cmp::min(buf.len(), output_bytes.len());
            buf[..take].copy_from_slice(<span class="kw-2">&amp;</span>output_bytes[..take]);
            buf = <span class="kw-2">&amp;mut </span>buf[take..];
            <span class="self">self</span>.position_within_block += take <span class="kw">as </span>u8;
            <span class="kw">if </span><span class="self">self</span>.position_within_block == BLOCK_LEN <span class="kw">as </span>u8 {
                <span class="self">self</span>.inner.counter += <span class="number">1</span>;
                <span class="self">self</span>.position_within_block = <span class="number">0</span>;
            }
        }
    }

    <span class="doccomment">/// Return the current read position in the output stream. This is
    /// equivalent to [`Seek::stream_position`], except that it doesn't return
    /// a `Result`. The position of a new `OutputReader` starts at 0, and each
    /// call to [`fill`] or [`Read::read`] moves the position forward by the
    /// number of bytes read.
    ///
    /// [`Seek::stream_position`]: #method.stream_position
    /// [`fill`]: #method.fill
    /// [`Read::read`]: #method.read
    </span><span class="kw">pub fn </span>position(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u64 {
        <span class="self">self</span>.inner.counter * BLOCK_LEN <span class="kw">as </span>u64 + <span class="self">self</span>.position_within_block <span class="kw">as </span>u64
    }

    <span class="doccomment">/// Seek to a new read position in the output stream. This is equivalent to
    /// calling [`Seek::seek`] with [`SeekFrom::Start`], except that it doesn't
    /// return a `Result`.
    ///
    /// [`Seek::seek`]: #method.seek
    /// [`SeekFrom::Start`]: https://doc.rust-lang.org/std/io/enum.SeekFrom.html
    </span><span class="kw">pub fn </span>set_position(<span class="kw-2">&amp;mut </span><span class="self">self</span>, position: u64) {
        <span class="self">self</span>.position_within_block = (position % BLOCK_LEN <span class="kw">as </span>u64) <span class="kw">as </span>u8;
        <span class="self">self</span>.inner.counter = position / BLOCK_LEN <span class="kw">as </span>u64;
    }
}

<span class="comment">// Don't derive(Debug), because the state may be secret.
</span><span class="kw">impl </span>fmt::Debug <span class="kw">for </span>OutputReader {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        f.debug_struct(<span class="string">"OutputReader"</span>)
            .field(<span class="string">"position"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.position())
            .finish()
    }
}

<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">impl </span>std::io::Read <span class="kw">for </span>OutputReader {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>read(<span class="kw-2">&amp;mut </span><span class="self">self</span>, buf: <span class="kw-2">&amp;mut </span>[u8]) -&gt; std::io::Result&lt;usize&gt; {
        <span class="self">self</span>.fill(buf);
        <span class="prelude-val">Ok</span>(buf.len())
    }
}

<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">impl </span>std::io::Seek <span class="kw">for </span>OutputReader {
    <span class="kw">fn </span>seek(<span class="kw-2">&amp;mut </span><span class="self">self</span>, pos: std::io::SeekFrom) -&gt; std::io::Result&lt;u64&gt; {
        <span class="kw">let </span>max_position = u64::max_value() <span class="kw">as </span>i128;
        <span class="kw">let </span>target_position: i128 = <span class="kw">match </span>pos {
            std::io::SeekFrom::Start(x) =&gt; x <span class="kw">as </span>i128,
            std::io::SeekFrom::Current(x) =&gt; <span class="self">self</span>.position() <span class="kw">as </span>i128 + x <span class="kw">as </span>i128,
            std::io::SeekFrom::End(<span class="kw">_</span>) =&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    <span class="string">"seek from end not supported"</span>,
                ));
            }
        };
        <span class="kw">if </span>target_position &lt; <span class="number">0 </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                <span class="string">"seek before start"</span>,
            ));
        }
        <span class="self">self</span>.set_position(cmp::min(target_position, max_position) <span class="kw">as </span>u64);
        <span class="prelude-val">Ok</span>(<span class="self">self</span>.position())
    }
}
</code></pre></div></section></main></body></html>