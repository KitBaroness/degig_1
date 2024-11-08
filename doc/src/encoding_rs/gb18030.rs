<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/encoding_rs-0.8.34/src/gb18030.rs`."><title>gb18030.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="encoding_rs" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../encoding_rs/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// Copyright Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 &lt;LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0&gt; or the MIT license
// &lt;LICENSE-MIT or https://opensource.org/licenses/MIT&gt;, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

</span><span class="kw">use super</span>::<span class="kw-2">*</span>;
<span class="kw">use </span><span class="kw">crate</span>::data::<span class="kw-2">*</span>;
<span class="kw">use </span><span class="kw">crate</span>::handles::<span class="kw-2">*</span>;
<span class="kw">use </span><span class="kw">crate</span>::variant::<span class="kw-2">*</span>;
<span class="comment">// Rust 1.14.0 requires the following despite the asterisk above.
</span><span class="kw">use </span><span class="kw">super</span>::in_inclusive_range16;
<span class="kw">use </span><span class="kw">super</span>::in_range16;

<span class="kw">enum </span>Gb18030Pending {
    <span class="prelude-val">None</span>,
    One(u8),
    Two(u8, u8),
    Three(u8, u8, u8),
}

<span class="kw">impl </span>Gb18030Pending {
    <span class="kw">fn </span>is_none(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="kw">match </span><span class="kw-2">*</span><span class="self">self </span>{
            Gb18030Pending::None =&gt; <span class="bool-val">true</span>,
            <span class="kw">_ </span>=&gt; <span class="bool-val">false</span>,
        }
    }

    <span class="kw">fn </span>count(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="kw">match </span><span class="kw-2">*</span><span class="self">self </span>{
            Gb18030Pending::None =&gt; <span class="number">0</span>,
            Gb18030Pending::One(<span class="kw">_</span>) =&gt; <span class="number">1</span>,
            Gb18030Pending::Two(<span class="kw">_</span>, <span class="kw">_</span>) =&gt; <span class="number">2</span>,
            Gb18030Pending::Three(<span class="kw">_</span>, <span class="kw">_</span>, <span class="kw">_</span>) =&gt; <span class="number">3</span>,
        }
    }
}

<span class="kw">pub struct </span>Gb18030Decoder {
    first: <span class="prelude-ty">Option</span>&lt;u8&gt;,
    second: <span class="prelude-ty">Option</span>&lt;u8&gt;,
    third: <span class="prelude-ty">Option</span>&lt;u8&gt;,
    pending: Gb18030Pending,
    pending_ascii: <span class="prelude-ty">Option</span>&lt;u8&gt;,
}

<span class="kw">impl </span>Gb18030Decoder {
    <span class="kw">pub fn </span>new() -&gt; VariantDecoder {
        VariantDecoder::Gb18030(Gb18030Decoder {
            first: <span class="prelude-val">None</span>,
            second: <span class="prelude-val">None</span>,
            third: <span class="prelude-val">None</span>,
            pending: Gb18030Pending::None,
            pending_ascii: <span class="prelude-val">None</span>,
        })
    }

    <span class="kw">pub fn </span>in_neutral_state(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.first.is_none()
            &amp;&amp; <span class="self">self</span>.second.is_none()
            &amp;&amp; <span class="self">self</span>.third.is_none()
            &amp;&amp; <span class="self">self</span>.pending.is_none()
            &amp;&amp; <span class="self">self</span>.pending_ascii.is_none()
    }

    <span class="kw">fn </span>extra_from_state(<span class="kw-2">&amp;</span><span class="self">self</span>, byte_length: usize) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        byte_length.checked_add(
            <span class="self">self</span>.pending.count()
                + <span class="kw">match </span><span class="self">self</span>.first {
                    <span class="prelude-val">None </span>=&gt; <span class="number">0</span>,
                    <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="number">1</span>,
                }
                + <span class="kw">match </span><span class="self">self</span>.second {
                    <span class="prelude-val">None </span>=&gt; <span class="number">0</span>,
                    <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="number">1</span>,
                }
                + <span class="kw">match </span><span class="self">self</span>.third {
                    <span class="prelude-val">None </span>=&gt; <span class="number">0</span>,
                    <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="number">1</span>,
                }
                + <span class="kw">match </span><span class="self">self</span>.pending_ascii {
                    <span class="prelude-val">None </span>=&gt; <span class="number">0</span>,
                    <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="number">1</span>,
                },
        )
    }

    <span class="kw">pub fn </span>max_utf16_buffer_length(<span class="kw-2">&amp;</span><span class="self">self</span>, byte_length: usize) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="comment">// ASCII: 1 to 1 (worst case)
        // gbk: 2 to 1
        // ranges: 4 to 1 or 4 to 2
        </span>checked_add(<span class="number">1</span>, <span class="self">self</span>.extra_from_state(byte_length))
    }

    <span class="kw">pub fn </span>max_utf8_buffer_length_without_replacement(<span class="kw-2">&amp;</span><span class="self">self</span>, byte_length: usize) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="comment">// ASCII: 1 to 1
        // gbk: 2 to 2 or 2 to 3
        // ranges: 4 to 2, 4 to 3 or 4 to 4
        // 0x80: 1 to 3 (worst case)
        </span><span class="self">self</span>.max_utf8_buffer_length(byte_length)
    }

    <span class="kw">pub fn </span>max_utf8_buffer_length(<span class="kw-2">&amp;</span><span class="self">self</span>, byte_length: usize) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        checked_add(<span class="number">1</span>, checked_mul(<span class="number">3</span>, <span class="self">self</span>.extra_from_state(byte_length)))
    }

    <span class="macro">gb18030_decoder_functions!</span>(
        {
            <span class="comment">// If first is between 0x81 and 0xFE, inclusive,
            // subtract offset 0x81.
            </span><span class="kw">let </span>non_ascii_minus_offset = non_ascii.wrapping_sub(<span class="number">0x81</span>);
            <span class="kw">if </span>non_ascii_minus_offset &gt; (<span class="number">0xFE </span>- <span class="number">0x81</span>) {
                <span class="kw">if </span>non_ascii == <span class="number">0x80 </span>{
                    handle.write_upper_bmp(<span class="number">0x20ACu16</span>);
                    <span class="kw">continue </span><span class="lifetime">'outermost</span>;
                }
                <span class="kw">return </span>(DecoderResult::Malformed(<span class="number">1</span>, <span class="number">0</span>),
                        source.consumed(),
                        handle.written());
            }
            non_ascii_minus_offset
        },
        {
            <span class="comment">// Two-byte (or error)
            </span><span class="kw">if </span>first_minus_offset &gt;= <span class="number">0x20 </span>{
                <span class="comment">// Not the gbk ideograph range above GB2312
                </span><span class="kw">let </span>trail_minus_offset = second.wrapping_sub(<span class="number">0xA1</span>);
                <span class="kw">if </span>trail_minus_offset &lt;= (<span class="number">0xFE </span>- <span class="number">0xA1</span>) {
                    <span class="comment">// GB2312
                    </span><span class="kw">let </span>hanzi_lead = first_minus_offset.wrapping_sub(<span class="number">0x2F</span>);
                    <span class="kw">if </span>hanzi_lead &lt; (<span class="number">0x77 </span>- <span class="number">0x2F</span>) {
                        <span class="comment">// Level 1 Hanzi, Level 2 Hanzi
                        // or one of the 5 PUA code
                        // points in between.
                        </span><span class="kw">let </span>hanzi_pointer = mul_94(hanzi_lead) + trail_minus_offset <span class="kw">as </span>usize;
                        <span class="kw">let </span>upper_bmp = GB2312_HANZI[hanzi_pointer];
                        handle.write_upper_bmp(upper_bmp)
                    } <span class="kw">else if </span>first_minus_offset == <span class="number">0x20 </span>{
                        <span class="comment">// Symbols (starting with ideographic space)
                        </span><span class="kw">let </span>bmp = GB2312_SYMBOLS[trail_minus_offset <span class="kw">as </span>usize];
                        handle.write_bmp_excl_ascii(bmp)
                    } <span class="kw">else if </span>first_minus_offset == <span class="number">0x25 </span>&amp;&amp; ((trail_minus_offset.wrapping_sub(<span class="number">63</span>) <span class="kw">as </span>usize) &lt; GB2312_SYMBOLS_AFTER_GREEK.len()) {
                        handle.write_bmp_excl_ascii(GB2312_SYMBOLS_AFTER_GREEK[trail_minus_offset.wrapping_sub(<span class="number">63</span>) <span class="kw">as </span>usize])
                    } <span class="kw">else if </span>first_minus_offset == <span class="number">0x27 </span>&amp;&amp; (trail_minus_offset <span class="kw">as </span>usize) &lt; GB2312_PINYIN.len() {
                        handle.write_bmp_excl_ascii(GB2312_PINYIN[trail_minus_offset <span class="kw">as </span>usize])
                    } <span class="kw">else if </span>first_minus_offset &gt; <span class="number">0x76 </span>{
                        <span class="comment">// Bottom PUA
                        </span><span class="kw">let </span>pua = (<span class="number">0xE234 </span>+ mul_94(first_minus_offset - <span class="number">0x77</span>) + trail_minus_offset <span class="kw">as </span>usize) <span class="kw">as </span>u16;
                        handle.write_upper_bmp(pua)
                    } <span class="kw">else </span>{
                        <span class="kw">let </span>bmp = gb2312_other_decode((mul_94(first_minus_offset - <span class="number">0x21</span>) + (trail_minus_offset <span class="kw">as </span>usize)) <span class="kw">as </span>u16);
                        handle.write_bmp_excl_ascii(bmp)
                    }
                } <span class="kw">else </span>{
                    <span class="comment">// gbk range on the left
                    </span><span class="kw">let </span><span class="kw-2">mut </span>trail_minus_offset = second.wrapping_sub(<span class="number">0x40</span>);
                    <span class="kw">if </span>trail_minus_offset &gt; (<span class="number">0x7E </span>- <span class="number">0x40</span>) {
                        <span class="kw">let </span>trail_minus_range_start = second.wrapping_sub(<span class="number">0x80</span>);
                        <span class="kw">if </span>trail_minus_range_start &gt; (<span class="number">0xA0 </span>- <span class="number">0x80</span>) {
                            <span class="kw">if </span>second &lt; <span class="number">0x80 </span>{
                                <span class="kw">return </span>(DecoderResult::Malformed(<span class="number">1</span>, <span class="number">0</span>),
                                        unread_handle_second.unread(),
                                        handle.written());
                            }
                            <span class="kw">return </span>(DecoderResult::Malformed(<span class="number">2</span>, <span class="number">0</span>),
                                    unread_handle_second.consumed(),
                                    handle.written());
                        }
                        trail_minus_offset = second - <span class="number">0x41</span>;
                    }
                    <span class="comment">// Zero-base lead
                    </span><span class="kw">let </span>left_lead = first_minus_offset - <span class="number">0x20</span>;
                    <span class="kw">let </span>left_pointer = left_lead <span class="kw">as </span>usize * (<span class="number">190 </span>- <span class="number">94</span>) +
                                       trail_minus_offset <span class="kw">as </span>usize;
                    <span class="kw">let </span>gbk_left_ideograph_pointer = left_pointer.wrapping_sub((<span class="number">0x29 </span>- <span class="number">0x20</span>) * (<span class="number">190 </span>- <span class="number">94</span>));
                    <span class="kw">if </span>gbk_left_ideograph_pointer &lt; (((<span class="number">0x7D </span>- <span class="number">0x29</span>) * (<span class="number">190 </span>- <span class="number">94</span>)) - <span class="number">5</span>) {
                        <span class="kw">let </span>upper_bmp = gbk_left_ideograph_decode(gbk_left_ideograph_pointer <span class="kw">as </span>u16);
                        handle.write_upper_bmp(upper_bmp)
                    } <span class="kw">else if </span>left_pointer &lt; ((<span class="number">0x29 </span>- <span class="number">0x20</span>) * (<span class="number">190 </span>- <span class="number">94</span>)) {
                        <span class="kw">let </span>bmp = gbk_other_decode(left_pointer <span class="kw">as </span>u16);
                        handle.write_bmp_excl_ascii(bmp)
                    } <span class="kw">else </span>{
                        <span class="kw">let </span>bottom_pointer = left_pointer - (((<span class="number">0x7D </span>- <span class="number">0x20</span>) * (<span class="number">190 </span>- <span class="number">94</span>)) - <span class="number">5</span>);
                        <span class="kw">let </span>upper_bmp = GBK_BOTTOM[bottom_pointer];
                        handle.write_upper_bmp(upper_bmp)
                    }
                }
            } <span class="kw">else </span>{
                <span class="comment">// gbk ideograph range above GB2312
                </span><span class="kw">let </span><span class="kw-2">mut </span>trail_minus_offset = second.wrapping_sub(<span class="number">0x40</span>);
                <span class="kw">if </span>trail_minus_offset &gt; (<span class="number">0x7E </span>- <span class="number">0x40</span>) {
                    <span class="kw">let </span>trail_minus_range_start = second.wrapping_sub(<span class="number">0x80</span>);
                    <span class="kw">if </span>trail_minus_range_start &gt; (<span class="number">0xFE </span>- <span class="number">0x80</span>) {
                        <span class="kw">if </span>second &lt; <span class="number">0x80 </span>{
                            <span class="kw">return </span>(DecoderResult::Malformed(<span class="number">1</span>, <span class="number">0</span>),
                                    unread_handle_second.unread(),
                                    handle.written());
                        }
                        <span class="kw">return </span>(DecoderResult::Malformed(<span class="number">2</span>, <span class="number">0</span>),
                                unread_handle_second.consumed(),
                                handle.written());
                    }
                    trail_minus_offset = second - <span class="number">0x41</span>;
                }
                <span class="kw">let </span>pointer = first_minus_offset <span class="kw">as </span>usize * <span class="number">190usize </span>+
                              trail_minus_offset <span class="kw">as </span>usize;
                <span class="kw">let </span>upper_bmp = gbk_top_ideograph_decode(pointer <span class="kw">as </span>u16);
                handle.write_upper_bmp(upper_bmp)
            }
        },
        {
            <span class="comment">// If third is between 0x81 and 0xFE, inclusive,
            // subtract offset 0x81.
            </span><span class="kw">let </span>third_minus_offset = third.wrapping_sub(<span class="number">0x81</span>);
            <span class="kw">if </span>third_minus_offset &gt; (<span class="number">0xFE </span>- <span class="number">0x81</span>) {
                <span class="comment">// We have an error. Let's inline what's going
                // to happen when `second` is
                // reprocessed. (`third` gets unread.)
                // `second` is guaranteed ASCII, so let's
                // put it in `pending_ascii`. Recompute
                // `second` from `second_minus_offset`.
                </span><span class="self">self</span>.pending_ascii = <span class="prelude-val">Some</span>(second_minus_offset + <span class="number">0x30</span>);
                <span class="comment">// Now unread `third` and designate the previous
                // `first` as being in error.
                </span><span class="kw">return </span>(DecoderResult::Malformed(<span class="number">1</span>, <span class="number">1</span>),
                        unread_handle_third.unread(),
                        handle.written());
            }
            third_minus_offset
        },
        {
            <span class="comment">// If fourth is between 0x30 and 0x39, inclusive,
            // subtract offset 0x30.
            //
            // If we have an error, we'll inline what's going
            // to happen when `second` and `third` are
            // reprocessed. (`fourth` gets unread.)
            // `second` is guaranteed ASCII, so let's
            // put it in `pending_ascii`. Recompute
            // `second` from `second_minus_offset` to
            // make this block reusable when `second`
            // is not in scope.
            //
            // `third` is guaranteed to be in the range
            // that makes it become the new `self.first`.
            //
            // `fourth` gets unread and the previous
            // `first` gets designates as being in error.
            </span><span class="kw">let </span>fourth_minus_offset = fourth.wrapping_sub(<span class="number">0x30</span>);
            <span class="kw">if </span>fourth_minus_offset &gt; (<span class="number">0x39 </span>- <span class="number">0x30</span>) {
                <span class="self">self</span>.pending_ascii = <span class="prelude-val">Some</span>(second_minus_offset + <span class="number">0x30</span>);
                <span class="self">self</span>.pending = Gb18030Pending::One(third_minus_offset);
                <span class="kw">return </span>(DecoderResult::Malformed(<span class="number">1</span>, <span class="number">2</span>),
                        unread_handle_fourth.unread(),
                        handle.written());
            }
            <span class="kw">let </span>pointer = (first_minus_offset <span class="kw">as </span>usize * (<span class="number">10 </span>* <span class="number">126 </span>* <span class="number">10</span>)) +
                          (second_minus_offset <span class="kw">as </span>usize * (<span class="number">10 </span>* <span class="number">126</span>)) +
                          (third_minus_offset <span class="kw">as </span>usize * <span class="number">10</span>) +
                          fourth_minus_offset <span class="kw">as </span>usize;
            <span class="kw">if </span>pointer &lt;= <span class="number">39419 </span>{
                <span class="comment">// BMP
                </span><span class="kw">if </span>pointer == <span class="number">7457 </span>{
                    handle.write_upper_bmp(<span class="number">0xE7C7</span>)
                } <span class="kw">else </span>{
                    handle.write_bmp_excl_ascii(gb18030_range_decode(pointer <span class="kw">as </span>u16))
                }
            } <span class="kw">else if </span>pointer &gt;= <span class="number">189_000 </span>&amp;&amp; pointer &lt;= <span class="number">1_237_575 </span>{
                <span class="comment">// Astral
                </span>handle.write_astral((pointer - (<span class="number">189_000usize </span>- <span class="number">0x1_0000usize</span>)) <span class="kw">as </span>u32)
            } <span class="kw">else </span>{
                <span class="kw">return </span>(DecoderResult::Malformed(<span class="number">4</span>, <span class="number">0</span>),
                        unread_handle_fourth.consumed(),
                        handle.written());
            }
        },
        <span class="self">self</span>,
        non_ascii,
        first_minus_offset,
        second,
        second_minus_offset,
        unread_handle_second,
        third,
        third_minus_offset,
        unread_handle_third,
        fourth,
        fourth_minus_offset,
        unread_handle_fourth,
        source,
        handle,
        <span class="lifetime">'outermost</span>);
}

<span class="comment">// XXX Experiment with inline directives
</span><span class="kw">fn </span>gbk_encode_non_unified(bmp: u16) -&gt; <span class="prelude-ty">Option</span>&lt;(usize, usize)&gt; {
    <span class="comment">// Try ideographic punctuation first as it's the most likely case.
    // Throwing in the check for full-width currencies and tilde is probably
    // more size-efficient here than elsewhere.
    </span><span class="kw">if </span>in_inclusive_range16(bmp, <span class="number">0x2014</span>, <span class="number">0x3017</span>) || in_inclusive_range16(bmp, <span class="number">0xFF04</span>, <span class="number">0xFFE1</span>) {
        <span class="kw">if let </span><span class="prelude-val">Some</span>(pos) = position(<span class="kw-2">&amp;</span>GB2312_SYMBOLS[..], bmp) {
            <span class="kw">return </span><span class="prelude-val">Some</span>((<span class="number">0xA1</span>, pos + <span class="number">0xA1</span>));
        }
    }
    <span class="comment">// Ext A
    </span><span class="kw">if </span>in_range16(bmp, <span class="number">0x3400</span>, <span class="number">0x4E00</span>) {
        <span class="kw">return </span>position(<span class="kw-2">&amp;</span>GBK_BOTTOM[<span class="number">21</span>..<span class="number">100</span>], bmp).map(|pos| {
            (
                <span class="number">0xFE</span>,
                pos + <span class="kw">if </span>pos &lt; (<span class="number">0x3F </span>- <span class="number">16</span>) {
                    <span class="number">0x40 </span>+ <span class="number">16
                </span>} <span class="kw">else </span>{
                    <span class="number">0x41 </span>+ <span class="number">16
                </span>},
            )
        });
    }
    <span class="comment">// Compatibility ideographs
    </span><span class="kw">if </span>in_range16(bmp, <span class="number">0xF900</span>, <span class="number">0xFB00</span>) {
        <span class="kw">return </span>position(<span class="kw-2">&amp;</span>GBK_BOTTOM[<span class="number">0</span>..<span class="number">21</span>], bmp).map(|pos| {
            <span class="kw">if </span>pos &lt; <span class="number">5 </span>{
                <span class="comment">// end of second to last row
                </span>(<span class="number">0xFD</span>, pos + (<span class="number">190 </span>- <span class="number">94 </span>- <span class="number">5 </span>+ <span class="number">0x41</span>))
            } <span class="kw">else </span>{
                <span class="comment">// last row
                </span>(<span class="number">0xFE</span>, pos + (<span class="number">0x40 </span>- <span class="number">5</span>))
            }
        });
    }
    <span class="comment">// Handle everything below U+02CA, which is in GBK_OTHER.
    </span><span class="kw">if </span>bmp &lt; <span class="number">0x02CA </span>{
        <span class="kw">if </span>in_range16(bmp, <span class="number">0x00E0</span>, <span class="number">0x0262</span>) &amp;&amp; bmp != <span class="number">0x00F7 </span>{
            <span class="comment">// Pinyin except U+1E3F
            </span><span class="kw">if let </span><span class="prelude-val">Some</span>(pos) = position(<span class="kw-2">&amp;</span>GB2312_PINYIN[..], bmp) {
                <span class="kw">return </span><span class="prelude-val">Some</span>((<span class="number">0xA8</span>, pos + <span class="number">0xA1</span>));
            }
        } <span class="kw">else if </span>in_inclusive_range16(bmp, <span class="number">0x00A4</span>, <span class="number">0x00F7</span>)
            || in_inclusive_range16(bmp, <span class="number">0x02C7</span>, <span class="number">0x02C9</span>)
        {
            <span class="comment">// Diacritics and Latin 1 symbols
            </span><span class="kw">if let </span><span class="prelude-val">Some</span>(pos) = position(<span class="kw-2">&amp;</span>GB2312_SYMBOLS[<span class="number">3</span>..(<span class="number">0xAC </span>- <span class="number">0x60</span>)], bmp) {
                <span class="kw">return </span><span class="prelude-val">Some</span>((<span class="number">0xA1</span>, pos + <span class="number">0xA1 </span>+ <span class="number">3</span>));
            }
        }
        <span class="kw">return </span><span class="prelude-val">None</span>;
    }
    <span class="kw">if </span>bmp &gt;= <span class="number">0xE794 </span>{
        <span class="comment">// Various brackets, all in PUA or full-width regions
        </span><span class="kw">if let </span><span class="prelude-val">Some</span>(pos) = position(<span class="kw-2">&amp;</span>GB2312_SYMBOLS_AFTER_GREEK[..], bmp) {
            <span class="kw">return </span><span class="prelude-val">Some</span>((<span class="number">0xA6</span>, pos + (<span class="number">0x9F </span>- <span class="number">0x60 </span>+ <span class="number">0xA1</span>)));
        }
    } <span class="kw">else if </span>bmp == <span class="number">0x1E3F </span>{
        <span class="comment">// The one Pinyin placed elsewhere on the BMP
        </span><span class="kw">return </span><span class="prelude-val">Some</span>((<span class="number">0xA8</span>, <span class="number">0x7B </span>- <span class="number">0x60 </span>+ <span class="number">0xA1</span>));
    } <span class="kw">else if </span>in_range16(bmp, <span class="number">0xA000</span>, <span class="number">0xD800</span>) {
        <span class="comment">// Since Korean has usage in China, let's spend a branch to fast-track
        // Hangul.
        </span><span class="kw">return </span><span class="prelude-val">None</span>;
    }
    <span class="comment">// GB2312 other (except bottom PUA and PUA between Hanzi levels).
    </span><span class="kw">if let </span><span class="prelude-val">Some</span>(other_pointer) = gb2312_other_encode(bmp) {
        <span class="kw">let </span>other_lead = other_pointer <span class="kw">as </span>usize / <span class="number">94</span>;
        <span class="kw">let </span>other_trail = other_pointer <span class="kw">as </span>usize % <span class="number">94</span>;
        <span class="kw">return </span><span class="prelude-val">Some</span>((<span class="number">0xA2 </span>+ other_lead, <span class="number">0xA1 </span>+ other_trail));
    }
    <span class="comment">// At this point, we've handled all mappable characters above U+02D9 but
    // below U+2010. Let's check for that range in order to let lower BMP
    // characters used for minority languages in China avoid the subsequent
    // search that deals mainly with various symbols.
    </span><span class="kw">if </span>in_range16(bmp, <span class="number">0x02DA</span>, <span class="number">0x2010</span>) {
        <span class="kw">return </span><span class="prelude-val">None</span>;
    }
    <span class="comment">// GBK other (except radicals and PUA in GBK_BOTTOM).
    </span><span class="kw">if let </span><span class="prelude-val">Some</span>(other_pointer) = gbk_other_encode(bmp) {
        <span class="kw">let </span>other_lead = other_pointer <span class="kw">as </span>usize / (<span class="number">190 </span>- <span class="number">94</span>);
        <span class="kw">let </span>other_trail = other_pointer <span class="kw">as </span>usize % (<span class="number">190 </span>- <span class="number">94</span>);
        <span class="kw">let </span>offset = <span class="kw">if </span>other_trail &lt; <span class="number">0x3F </span>{ <span class="number">0x40 </span>} <span class="kw">else </span>{ <span class="number">0x41 </span>};
        <span class="kw">return </span><span class="prelude-val">Some</span>((other_lead + (<span class="number">0x81 </span>+ <span class="number">0x20</span>), other_trail + offset));
    }
    <span class="comment">// CJK Radicals Supplement or PUA in GBK_BOTTOM
    </span><span class="kw">if </span>in_inclusive_range16(bmp, <span class="number">0x2E81</span>, <span class="number">0x2ECA</span>) || in_inclusive_range16(bmp, <span class="number">0xE816</span>, <span class="number">0xE864</span>) {
        <span class="kw">if let </span><span class="prelude-val">Some</span>(pos) = position(<span class="kw-2">&amp;</span>GBK_BOTTOM[<span class="number">21</span>..], bmp) {
            <span class="kw">let </span>trail = pos + <span class="number">16</span>;
            <span class="kw">let </span>offset = <span class="kw">if </span>trail &lt; <span class="number">0x3F </span>{ <span class="number">0x40 </span>} <span class="kw">else </span>{ <span class="number">0x41 </span>};
            <span class="kw">return </span><span class="prelude-val">Some</span>((<span class="number">0xFE</span>, trail + offset));
        }
    }
    <span class="comment">// GB2312 bottom PUA
    </span><span class="kw">let </span>bmp_minus_gb2312_bottom_pua = bmp.wrapping_sub(<span class="number">0xE234</span>);
    <span class="kw">if </span>bmp_minus_gb2312_bottom_pua &lt;= (<span class="number">0xE4C5 </span>- <span class="number">0xE234</span>) {
        <span class="kw">let </span>pua_lead = bmp_minus_gb2312_bottom_pua <span class="kw">as </span>usize / <span class="number">94</span>;
        <span class="kw">let </span>pua_trail = bmp_minus_gb2312_bottom_pua <span class="kw">as </span>usize % <span class="number">94</span>;
        <span class="kw">return </span><span class="prelude-val">Some</span>((<span class="number">0x81 </span>+ <span class="number">0x77 </span>+ pua_lead, <span class="number">0xA1 </span>+ pua_trail));
    }
    <span class="comment">// PUA between Hanzi Levels
    </span><span class="kw">let </span>bmp_minus_pua_between_hanzi = bmp.wrapping_sub(<span class="number">0xE810</span>);
    <span class="kw">if </span>bmp_minus_pua_between_hanzi &lt; <span class="number">5 </span>{
        <span class="kw">return </span><span class="prelude-val">Some</span>((<span class="number">0x81 </span>+ <span class="number">0x56</span>, <span class="number">0xFF </span>- <span class="number">5 </span>+ bmp_minus_pua_between_hanzi <span class="kw">as </span>usize));
    }
    <span class="prelude-val">None
</span>}

<span class="attr">#[cfg(not(feature = <span class="string">"fast-gb-hanzi-encode"</span>))]
#[inline(always)]
</span><span class="kw">fn </span>encode_hanzi(bmp: u16, <span class="kw">_</span>: u16) -&gt; (u8, u8) {
    <span class="kw">if let </span><span class="prelude-val">Some</span>((lead, trail)) = gb2312_level1_hanzi_encode(bmp) {
        (lead, trail)
    } <span class="kw">else if let </span><span class="prelude-val">Some</span>(hanzi_pointer) = gb2312_level2_hanzi_encode(bmp) {
        <span class="kw">let </span>hanzi_lead = (hanzi_pointer / <span class="number">94</span>) + (<span class="number">0xD8</span>);
        <span class="kw">let </span>hanzi_trail = (hanzi_pointer % <span class="number">94</span>) + <span class="number">0xA1</span>;
        (hanzi_lead <span class="kw">as </span>u8, hanzi_trail <span class="kw">as </span>u8)
    } <span class="kw">else </span>{
        <span class="kw">let </span>(lead, gbk_trail) = <span class="kw">if </span>bmp &lt; <span class="number">0x72DC </span>{
            <span class="comment">// Above GB2312
            </span><span class="kw">let </span>pointer = gbk_top_ideograph_encode(bmp) <span class="kw">as </span>usize;
            <span class="kw">let </span>lead = (pointer / <span class="number">190</span>) + <span class="number">0x81</span>;
            <span class="kw">let </span>gbk_trail = pointer % <span class="number">190</span>;
            (lead, gbk_trail)
        } <span class="kw">else </span>{
            <span class="comment">// To the left of GB2312
            </span><span class="kw">let </span>gbk_left_ideograph_pointer = gbk_left_ideograph_encode(bmp) <span class="kw">as </span>usize;
            <span class="kw">let </span>lead = (gbk_left_ideograph_pointer / (<span class="number">190 </span>- <span class="number">94</span>)) + (<span class="number">0x81 </span>+ <span class="number">0x29</span>);
            <span class="kw">let </span>gbk_trail = gbk_left_ideograph_pointer % (<span class="number">190 </span>- <span class="number">94</span>);
            (lead, gbk_trail)
        };
        <span class="kw">let </span>offset = <span class="kw">if </span>gbk_trail &lt; <span class="number">0x3F </span>{ <span class="number">0x40 </span>} <span class="kw">else </span>{ <span class="number">0x41 </span>};
        (lead <span class="kw">as </span>u8, (gbk_trail + offset) <span class="kw">as </span>u8)
    }
}

<span class="attr">#[cfg(feature = <span class="string">"fast-gb-hanzi-encode"</span>)]
#[inline(always)]
</span><span class="kw">fn </span>encode_hanzi(<span class="kw">_</span>: u16, bmp_minus_unified_start: u16) -&gt; (u8, u8) {
    gbk_hanzi_encode(bmp_minus_unified_start)
}

<span class="kw">pub struct </span>Gb18030Encoder {
    extended: bool,
}

<span class="kw">impl </span>Gb18030Encoder {
    <span class="kw">pub fn </span>new(encoding: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>Encoding, extended_range: bool) -&gt; Encoder {
        Encoder::new(
            encoding,
            VariantEncoder::Gb18030(Gb18030Encoder {
                extended: extended_range,
            }),
        )
    }

    <span class="kw">pub fn </span>max_buffer_length_from_utf16_without_replacement(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        u16_length: usize,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="kw">if </span><span class="self">self</span>.extended {
            u16_length.checked_mul(<span class="number">4</span>)
        } <span class="kw">else </span>{
            <span class="comment">// Need to add, because space check is done with the four-byte
            // assumption.
            </span>checked_add(<span class="number">2</span>, u16_length.checked_mul(<span class="number">2</span>))
        }
    }

    <span class="kw">pub fn </span>max_buffer_length_from_utf8_without_replacement(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        byte_length: usize,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="kw">if </span><span class="self">self</span>.extended {
            <span class="comment">// 1 to 1
            // 2 to 2
            // 3 to 2
            // 2 to 4 (worst)
            // 3 to 4
            // 4 to 4
            </span>checked_add(<span class="number">2</span>, byte_length.checked_mul(<span class="number">2</span>))
        } <span class="kw">else </span>{
            <span class="comment">// 1 to 1
            // 2 to 2
            // 3 to 2
            // Need to add, because space check is done with the four-byte
            // assumption.
            </span>byte_length.checked_add(<span class="number">3</span>)
        }
    }

    <span class="macro">ascii_compatible_encoder_functions!</span>(
        {
            <span class="kw">let </span>bmp_minus_unified_start = bmp.wrapping_sub(<span class="number">0x4E00</span>);
            <span class="kw">if </span>bmp_minus_unified_start &lt; (<span class="number">0x9FA6 </span>- <span class="number">0x4E00</span>) {
                <span class="comment">// CJK Unified Ideographs
                // Can't fail now, since all are
                // mapped.
                </span><span class="kw">let </span>(lead, trail) = encode_hanzi(bmp, bmp_minus_unified_start);
                handle.write_two(lead, trail)
            } <span class="kw">else if </span>bmp == <span class="number">0xE5E5 </span>{
                <span class="comment">// It's not optimal to check for the unmappable
                // and for euro at this stage, but getting
                // the out of the way makes the rest of the
                // code less messy.
                </span><span class="kw">return </span>(
                    EncoderResult::unmappable_from_bmp(bmp),
                    source.consumed(),
                    handle.written(),
                );
            } <span class="kw">else if </span>bmp == <span class="number">0x20AC </span>&amp;&amp; !<span class="self">self</span>.extended {
                handle.write_one(<span class="number">0x80u8</span>)
            } <span class="kw">else </span>{
                <span class="kw">match </span>gbk_encode_non_unified(bmp) {
                    <span class="prelude-val">Some</span>((lead, trail)) =&gt; handle.write_two(lead <span class="kw">as </span>u8, trail <span class="kw">as </span>u8),
                    <span class="prelude-val">None </span>=&gt; {
                        <span class="kw">if </span>!<span class="self">self</span>.extended {
                            <span class="kw">return </span>(
                                EncoderResult::unmappable_from_bmp(bmp),
                                source.consumed(),
                                handle.written(),
                            );
                        }
                        <span class="kw">let </span>range_pointer = gb18030_range_encode(bmp);
                        <span class="kw">let </span>first = range_pointer / (<span class="number">10 </span>* <span class="number">126 </span>* <span class="number">10</span>);
                        <span class="kw">let </span>rem_first = range_pointer % (<span class="number">10 </span>* <span class="number">126 </span>* <span class="number">10</span>);
                        <span class="kw">let </span>second = rem_first / (<span class="number">10 </span>* <span class="number">126</span>);
                        <span class="kw">let </span>rem_second = rem_first % (<span class="number">10 </span>* <span class="number">126</span>);
                        <span class="kw">let </span>third = rem_second / <span class="number">10</span>;
                        <span class="kw">let </span>fourth = rem_second % <span class="number">10</span>;
                        handle.write_four(
                            (first + <span class="number">0x81</span>) <span class="kw">as </span>u8,
                            (second + <span class="number">0x30</span>) <span class="kw">as </span>u8,
                            (third + <span class="number">0x81</span>) <span class="kw">as </span>u8,
                            (fourth + <span class="number">0x30</span>) <span class="kw">as </span>u8,
                        )
                    }
                }
            }
        },
        {
            <span class="kw">if </span>!<span class="self">self</span>.extended {
                <span class="kw">return </span>(
                    EncoderResult::Unmappable(astral),
                    source.consumed(),
                    handle.written(),
                );
            }
            <span class="kw">let </span>range_pointer = astral <span class="kw">as </span>usize + (<span class="number">189_000usize </span>- <span class="number">0x1_0000usize</span>);
            <span class="kw">let </span>first = range_pointer / (<span class="number">10 </span>* <span class="number">126 </span>* <span class="number">10</span>);
            <span class="kw">let </span>rem_first = range_pointer % (<span class="number">10 </span>* <span class="number">126 </span>* <span class="number">10</span>);
            <span class="kw">let </span>second = rem_first / (<span class="number">10 </span>* <span class="number">126</span>);
            <span class="kw">let </span>rem_second = rem_first % (<span class="number">10 </span>* <span class="number">126</span>);
            <span class="kw">let </span>third = rem_second / <span class="number">10</span>;
            <span class="kw">let </span>fourth = rem_second % <span class="number">10</span>;
            handle.write_four(
                (first + <span class="number">0x81</span>) <span class="kw">as </span>u8,
                (second + <span class="number">0x30</span>) <span class="kw">as </span>u8,
                (third + <span class="number">0x81</span>) <span class="kw">as </span>u8,
                (fourth + <span class="number">0x30</span>) <span class="kw">as </span>u8,
            )
        },
        bmp,
        astral,
        <span class="self">self</span>,
        source,
        handle,
        copy_ascii_to_check_space_four,
        check_space_four,
        <span class="bool-val">false
    </span>);
}

<span class="comment">// Any copyright to the test code below this comment is dedicated to the
// Public Domain. http://creativecommons.org/publicdomain/zero/1.0/

</span><span class="attr">#[cfg(all(test, feature = <span class="string">"alloc"</span>))]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span><span class="kw">super</span>::<span class="kw">super</span>::testing::<span class="kw-2">*</span>;
    <span class="kw">use </span><span class="kw">super</span>::<span class="kw">super</span>::<span class="kw-2">*</span>;

    <span class="kw">fn </span>decode_gb18030(bytes: <span class="kw-2">&amp;</span>[u8], expect: <span class="kw-2">&amp;</span>str) {
        decode(GB18030, bytes, expect);
    }

    <span class="kw">fn </span>encode_gb18030(string: <span class="kw-2">&amp;</span>str, expect: <span class="kw-2">&amp;</span>[u8]) {
        encode(GB18030, string, expect);
    }

    <span class="kw">fn </span>encode_gbk(string: <span class="kw-2">&amp;</span>str, expect: <span class="kw-2">&amp;</span>[u8]) {
        encode(GBK, string, expect);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_gb18030_decode() {
        <span class="comment">// Empty
        </span>decode_gb18030(<span class="string">b""</span>, <span class="kw-2">&amp;</span><span class="string">""</span>);

        <span class="comment">// ASCII
        </span>decode_gb18030(<span class="string">b"\x61\x62"</span>, <span class="string">"\u{0061}\u{0062}"</span>);

        <span class="comment">// euro
        </span>decode_gb18030(<span class="string">b"\x80"</span>, <span class="string">"\u{20AC}"</span>);
        decode_gb18030(<span class="string">b"\xA2\xE3"</span>, <span class="string">"\u{20AC}"</span>);

        <span class="comment">// two bytes
        </span>decode_gb18030(<span class="string">b"\x81\x40"</span>, <span class="string">"\u{4E02}"</span>);
        decode_gb18030(<span class="string">b"\x81\x7E"</span>, <span class="string">"\u{4E8A}"</span>);
        decode_gb18030(<span class="string">b"\x81\x7F"</span>, <span class="string">"\u{FFFD}\u{007F}"</span>);
        decode_gb18030(<span class="string">b"\x81\x80"</span>, <span class="string">"\u{4E90}"</span>);
        decode_gb18030(<span class="string">b"\x81\xFE"</span>, <span class="string">"\u{4FA2}"</span>);
        decode_gb18030(<span class="string">b"\xFE\x40"</span>, <span class="string">"\u{FA0C}"</span>);
        decode_gb18030(<span class="string">b"\xFE\x7E"</span>, <span class="string">"\u{E843}"</span>);
        decode_gb18030(<span class="string">b"\xFE\x7F"</span>, <span class="string">"\u{FFFD}\u{007F}"</span>);
        decode_gb18030(<span class="string">b"\xFE\x80"</span>, <span class="string">"\u{4723}"</span>);
        decode_gb18030(<span class="string">b"\xFE\xFE"</span>, <span class="string">"\u{E4C5}"</span>);

        <span class="comment">// The difference from the original GB18030
        </span>decode_gb18030(<span class="string">b"\xA3\xA0"</span>, <span class="string">"\u{3000}"</span>);
        decode_gb18030(<span class="string">b"\xA1\xA1"</span>, <span class="string">"\u{3000}"</span>);

        <span class="comment">// 0xFF
        </span>decode_gb18030(<span class="string">b"\xFF\x40"</span>, <span class="string">"\u{FFFD}\u{0040}"</span>);
        decode_gb18030(<span class="string">b"\xE3\xFF\x9A\x33"</span>, <span class="string">"\u{FFFD}\u{FFFD}"</span>); <span class="comment">// not \u{FFFD}\u{FFFD}\u{0033} !
        </span>decode_gb18030(<span class="string">b"\xFF\x32\x9A\x33"</span>, <span class="string">"\u{FFFD}\u{0032}\u{FFFD}"</span>); <span class="comment">// not \u{FFFD}\u{0032}\u{FFFD}\u{0033} !
        </span>decode_gb18030(<span class="string">b"\xFF\x40\x00"</span>, <span class="string">"\u{FFFD}\u{0040}\u{0000}"</span>);
        decode_gb18030(<span class="string">b"\xE3\xFF\x9A\x33\x00"</span>, <span class="string">"\u{FFFD}\u{FFFD}\u{0033}\u{0000}"</span>);
        decode_gb18030(
            <span class="string">b"\xFF\x32\x9A\x33\x00"</span>,
            <span class="string">"\u{FFFD}\u{0032}\u{FFFD}\u{0033}\u{0000}"</span>,
        );

        <span class="comment">// Four bytes
        </span>decode_gb18030(<span class="string">b"\x81\x30\x81\x30"</span>, <span class="string">"\u{0080}"</span>);
        decode_gb18030(<span class="string">b"\x81\x35\xF4\x37"</span>, <span class="string">"\u{E7C7}"</span>);
        decode_gb18030(<span class="string">b"\x81\x37\xA3\x30"</span>, <span class="string">"\u{2603}"</span>);
        decode_gb18030(<span class="string">b"\x94\x39\xDA\x33"</span>, <span class="string">"\u{1F4A9}"</span>);
        decode_gb18030(<span class="string">b"\xE3\x32\x9A\x35"</span>, <span class="string">"\u{10FFFF}"</span>);
        decode_gb18030(<span class="string">b"\xE3\x32\x9A\x36\x81\x30"</span>, <span class="string">"\u{FFFD}\u{FFFD}"</span>);
        decode_gb18030(<span class="string">b"\xE3\x32\x9A\x36\x81\x40"</span>, <span class="string">"\u{FFFD}\u{4E02}"</span>);
        decode_gb18030(<span class="string">b"\xE3\x32\x9A"</span>, <span class="string">"\u{FFFD}"</span>); <span class="comment">// not \u{FFFD}\u{0032}\u{FFFD} !
        </span>decode_gb18030(<span class="string">b"\xE3\x32\x9A\x00"</span>, <span class="string">"\u{FFFD}\u{0032}\u{FFFD}\u{0000}"</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_gb18030_encode() {
        <span class="comment">// Empty
        </span>encode_gb18030(<span class="string">""</span>, <span class="string">b""</span>);

        <span class="comment">// ASCII
        </span>encode_gb18030(<span class="string">"\u{0061}\u{0062}"</span>, <span class="string">b"\x61\x62"</span>);

        <span class="comment">// euro
        </span>encode_gb18030(<span class="string">"\u{20AC}"</span>, <span class="string">b"\xA2\xE3"</span>);

        <span class="comment">// two bytes
        </span>encode_gb18030(<span class="string">"\u{4E02}"</span>, <span class="string">b"\x81\x40"</span>);
        encode_gb18030(<span class="string">"\u{4E8A}"</span>, <span class="string">b"\x81\x7E"</span>);
        <span class="kw">if </span>!<span class="macro">cfg!</span>(miri) {
            <span class="comment">// Miri is too slow
            </span>encode_gb18030(<span class="string">"\u{4E90}"</span>, <span class="string">b"\x81\x80"</span>);
            encode_gb18030(<span class="string">"\u{4FA2}"</span>, <span class="string">b"\x81\xFE"</span>);
            encode_gb18030(<span class="string">"\u{FA0C}"</span>, <span class="string">b"\xFE\x40"</span>);
            encode_gb18030(<span class="string">"\u{E843}"</span>, <span class="string">b"\xFE\x7E"</span>);
            encode_gb18030(<span class="string">"\u{4723}"</span>, <span class="string">b"\xFE\x80"</span>);
            encode_gb18030(<span class="string">"\u{E4C5}"</span>, <span class="string">b"\xFE\xFE"</span>);
        }

        <span class="comment">// The difference from the original GB18030
        </span>encode_gb18030(<span class="string">"\u{E5E5}"</span>, <span class="string">b"&amp;#58853;"</span>);
        encode_gb18030(<span class="string">"\u{3000}"</span>, <span class="string">b"\xA1\xA1"</span>);

        <span class="comment">// Four bytes
        </span>encode_gb18030(<span class="string">"\u{0080}"</span>, <span class="string">b"\x81\x30\x81\x30"</span>);
        encode_gb18030(<span class="string">"\u{E7C7}"</span>, <span class="string">b"\x81\x35\xF4\x37"</span>);
        <span class="kw">if </span>!<span class="macro">cfg!</span>(miri) {
            <span class="comment">// Miri is too slow
            </span>encode_gb18030(<span class="string">"\u{2603}"</span>, <span class="string">b"\x81\x37\xA3\x30"</span>);
            encode_gb18030(<span class="string">"\u{1F4A9}"</span>, <span class="string">b"\x94\x39\xDA\x33"</span>);
            encode_gb18030(<span class="string">"\u{10FFFF}"</span>, <span class="string">b"\xE3\x32\x9A\x35"</span>);
        }

        <span class="comment">// Edge cases
        </span>encode_gb18030(<span class="string">"\u{00F7}"</span>, <span class="string">b"\xA1\xC2"</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_gbk_encode() {
        <span class="comment">// Empty
        </span>encode_gbk(<span class="string">""</span>, <span class="string">b""</span>);

        <span class="comment">// ASCII
        </span>encode_gbk(<span class="string">"\u{0061}\u{0062}"</span>, <span class="string">b"\x61\x62"</span>);

        <span class="comment">// euro
        </span>encode_gbk(<span class="string">"\u{20AC}"</span>, <span class="string">b"\x80"</span>);

        <span class="comment">// two bytes
        </span>encode_gbk(<span class="string">"\u{4E02}"</span>, <span class="string">b"\x81\x40"</span>);
        encode_gbk(<span class="string">"\u{4E8A}"</span>, <span class="string">b"\x81\x7E"</span>);
        <span class="kw">if </span>!<span class="macro">cfg!</span>(miri) {
            <span class="comment">// Miri is too slow
            </span>encode_gbk(<span class="string">"\u{4E90}"</span>, <span class="string">b"\x81\x80"</span>);
            encode_gbk(<span class="string">"\u{4FA2}"</span>, <span class="string">b"\x81\xFE"</span>);
            encode_gbk(<span class="string">"\u{FA0C}"</span>, <span class="string">b"\xFE\x40"</span>);
            encode_gbk(<span class="string">"\u{E843}"</span>, <span class="string">b"\xFE\x7E"</span>);
            encode_gbk(<span class="string">"\u{4723}"</span>, <span class="string">b"\xFE\x80"</span>);
            encode_gbk(<span class="string">"\u{E4C5}"</span>, <span class="string">b"\xFE\xFE"</span>);
        }

        <span class="comment">// The difference from the original gb18030
        </span>encode_gbk(<span class="string">"\u{E5E5}"</span>, <span class="string">b"&amp;#58853;"</span>);
        encode_gbk(<span class="string">"\u{3000}"</span>, <span class="string">b"\xA1\xA1"</span>);

        <span class="comment">// Four bytes
        </span>encode_gbk(<span class="string">"\u{0080}"</span>, <span class="string">b"&amp;#128;"</span>);
        encode_gbk(<span class="string">"\u{E7C7}"</span>, <span class="string">b"&amp;#59335;"</span>);
        <span class="kw">if </span>!<span class="macro">cfg!</span>(miri) {
            <span class="comment">// Miri is too slow
            </span>encode_gbk(<span class="string">"\u{2603}"</span>, <span class="string">b"&amp;#9731;"</span>);
            encode_gbk(<span class="string">"\u{1F4A9}"</span>, <span class="string">b"&amp;#128169;"</span>);
            encode_gbk(<span class="string">"\u{10FFFF}"</span>, <span class="string">b"&amp;#1114111;"</span>);
        }

        <span class="comment">// Edge cases
        </span>encode_gbk(<span class="string">"\u{00F7}"</span>, <span class="string">b"\xA1\xC2"</span>);
    }

    <span class="attr">#[test]
    #[cfg_attr(miri, ignore)] </span><span class="comment">// Miri is too slow
    </span><span class="kw">fn </span>test_gb18030_decode_all() {
        <span class="kw">let </span>input = <span class="macro">include_bytes!</span>(<span class="string">"test_data/gb18030_in.txt"</span>);
        <span class="kw">let </span>expectation = <span class="macro">include_str!</span>(<span class="string">"test_data/gb18030_in_ref.txt"</span>);
        <span class="kw">let </span>(cow, had_errors) = GB18030.decode_without_bom_handling(input);
        <span class="macro">assert!</span>(!had_errors, <span class="string">"Should not have had errors."</span>);
        <span class="macro">assert_eq!</span>(<span class="kw-2">&amp;</span>cow[..], expectation);
    }

    <span class="attr">#[test]
    #[cfg_attr(miri, ignore)] </span><span class="comment">// Miri is too slow
    </span><span class="kw">fn </span>test_gb18030_encode_all() {
        <span class="kw">let </span>input = <span class="macro">include_str!</span>(<span class="string">"test_data/gb18030_out.txt"</span>);
        <span class="kw">let </span>expectation = <span class="macro">include_bytes!</span>(<span class="string">"test_data/gb18030_out_ref.txt"</span>);
        <span class="kw">let </span>(cow, encoding, had_errors) = GB18030.encode(input);
        <span class="macro">assert!</span>(!had_errors, <span class="string">"Should not have had errors."</span>);
        <span class="macro">assert_eq!</span>(encoding, GB18030);
        <span class="macro">assert_eq!</span>(<span class="kw-2">&amp;</span>cow[..], <span class="kw-2">&amp;</span>expectation[..]);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_gb18030_encode_from_utf16_max_length() {
        <span class="kw">let </span><span class="kw-2">mut </span>output = [<span class="number">0u8</span>; <span class="number">20</span>];
        <span class="kw">let </span><span class="kw-2">mut </span>encoder = GB18030.new_encoder();
        {
            <span class="kw">let </span>needed = encoder
                .max_buffer_length_from_utf16_without_replacement(<span class="number">1</span>)
                .unwrap();
            <span class="kw">let </span>(result, read, written) = encoder.encode_from_utf16_without_replacement(
                <span class="kw-2">&amp;</span>[<span class="number">0x3000</span>],
                <span class="kw-2">&amp;mut </span>output[..needed],
                <span class="bool-val">true</span>,
            );
            <span class="macro">assert_eq!</span>(result, EncoderResult::InputEmpty);
            <span class="macro">assert_eq!</span>(read, <span class="number">1</span>);
            <span class="macro">assert_eq!</span>(written, <span class="number">2</span>);
            <span class="macro">assert_eq!</span>(output[<span class="number">0</span>], <span class="number">0xA1</span>);
            <span class="macro">assert_eq!</span>(output[<span class="number">1</span>], <span class="number">0xA1</span>);
        }
    }
}
</code></pre></div></section></main></body></html>