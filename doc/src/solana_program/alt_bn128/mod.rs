<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.18.9/src/alt_bn128/mod.rs`."><title>mod.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="solana_program" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../solana_program/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">pub mod </span>compression;
<span class="kw">pub mod </span>prelude {
    <span class="kw">pub use </span><span class="kw">crate</span>::alt_bn128::{consts::<span class="kw-2">*</span>, target_arch::<span class="kw-2">*</span>, AltBn128Error};
}

<span class="kw">use </span>{
    bytemuck::{Pod, Zeroable},
    consts::<span class="kw-2">*</span>,
    thiserror::Error,
};

<span class="kw">mod </span>consts {
    <span class="doccomment">/// Input length for the add operation.
    </span><span class="kw">pub const </span>ALT_BN128_ADDITION_INPUT_LEN: usize = <span class="number">128</span>;

    <span class="doccomment">/// Input length for the multiplication operation.
    </span><span class="kw">pub const </span>ALT_BN128_MULTIPLICATION_INPUT_LEN: usize = <span class="number">128</span>;

    <span class="doccomment">/// Pair element length.
    </span><span class="kw">pub const </span>ALT_BN128_PAIRING_ELEMENT_LEN: usize = <span class="number">192</span>;

    <span class="doccomment">/// Output length for the add operation.
    </span><span class="kw">pub const </span>ALT_BN128_ADDITION_OUTPUT_LEN: usize = <span class="number">64</span>;

    <span class="doccomment">/// Output length for the multiplication operation.
    </span><span class="kw">pub const </span>ALT_BN128_MULTIPLICATION_OUTPUT_LEN: usize = <span class="number">64</span>;

    <span class="doccomment">/// Output length for pairing operation.
    </span><span class="kw">pub const </span>ALT_BN128_PAIRING_OUTPUT_LEN: usize = <span class="number">32</span>;

    <span class="doccomment">/// Size of the EC point field, in bytes.
    </span><span class="kw">pub const </span>ALT_BN128_FIELD_SIZE: usize = <span class="number">32</span>;

    <span class="doccomment">/// Size of the EC point. `alt_bn128` point contains
    /// the consistently united x and y fields as 64 bytes.
    </span><span class="kw">pub const </span>ALT_BN128_POINT_SIZE: usize = <span class="number">64</span>;

    <span class="kw">pub const </span>ALT_BN128_ADD: u64 = <span class="number">0</span>;
    <span class="kw">pub const </span>ALT_BN128_SUB: u64 = <span class="number">1</span>;
    <span class="kw">pub const </span>ALT_BN128_MUL: u64 = <span class="number">2</span>;
    <span class="kw">pub const </span>ALT_BN128_PAIRING: u64 = <span class="number">3</span>;
}

<span class="attr">#[derive(Debug, Error, Clone, PartialEq, Eq)]
</span><span class="kw">pub enum </span>AltBn128Error {
    <span class="attr">#[error(<span class="string">"The input data is invalid"</span>)]
    </span>InvalidInputData,
    <span class="attr">#[error(<span class="string">"Invalid group data"</span>)]
    </span>GroupError,
    <span class="attr">#[error(<span class="string">"Slice data is going out of input data bounds"</span>)]
    </span>SliceOutOfBounds,
    <span class="attr">#[error(<span class="string">"Unexpected error"</span>)]
    </span>UnexpectedError,
    <span class="attr">#[error(<span class="string">"Failed to convert a byte slice into a vector {0:?}"</span>)]
    </span>TryIntoVecError(Vec&lt;u8&gt;),
    <span class="attr">#[error(<span class="string">"Failed to convert projective to affine g1"</span>)]
    </span>ProjectiveToG1Failed,
}

<span class="kw">impl </span>From&lt;u64&gt; <span class="kw">for </span>AltBn128Error {
    <span class="kw">fn </span>from(v: u64) -&gt; AltBn128Error {
        <span class="kw">match </span>v {
            <span class="number">1 </span>=&gt; AltBn128Error::InvalidInputData,
            <span class="number">2 </span>=&gt; AltBn128Error::GroupError,
            <span class="number">3 </span>=&gt; AltBn128Error::SliceOutOfBounds,
            <span class="number">4 </span>=&gt; AltBn128Error::TryIntoVecError(Vec::new()),
            <span class="number">5 </span>=&gt; AltBn128Error::ProjectiveToG1Failed,
            <span class="kw">_ </span>=&gt; AltBn128Error::UnexpectedError,
        }
    }
}

<span class="kw">impl </span>From&lt;AltBn128Error&gt; <span class="kw">for </span>u64 {
    <span class="kw">fn </span>from(v: AltBn128Error) -&gt; u64 {
        <span class="kw">match </span>v {
            AltBn128Error::InvalidInputData =&gt; <span class="number">1</span>,
            AltBn128Error::GroupError =&gt; <span class="number">2</span>,
            AltBn128Error::SliceOutOfBounds =&gt; <span class="number">3</span>,
            AltBn128Error::TryIntoVecError(<span class="kw">_</span>) =&gt; <span class="number">4</span>,
            AltBn128Error::ProjectiveToG1Failed =&gt; <span class="number">5</span>,
            AltBn128Error::UnexpectedError =&gt; <span class="number">0</span>,
        }
    }
}

<span class="attr">#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
#[repr(transparent)]
</span><span class="kw">pub struct </span>PodG1(<span class="kw">pub </span>[u8; <span class="number">64</span>]);

<span class="attr">#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
#[repr(transparent)]
</span><span class="kw">pub struct </span>PodG2(<span class="kw">pub </span>[u8; <span class="number">128</span>]);

<span class="attr">#[cfg(not(target_os = <span class="string">"solana"</span>))]
</span><span class="kw">mod </span>target_arch {
    <span class="kw">use </span>{
        <span class="kw">super</span>::<span class="kw-2">*</span>,
        ark_bn254::{<span class="self">self</span>, Config},
        ark_ec::{<span class="self">self</span>, models::bn::Bn, pairing::Pairing, AffineRepr},
        ark_ff::{BigInteger, BigInteger256, One},
        ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate},
    };

    <span class="kw">type </span>G1 = ark_bn254::g1::G1Affine;
    <span class="kw">type </span>G2 = ark_bn254::g2::G2Affine;

    <span class="kw">impl </span>TryFrom&lt;PodG1&gt; <span class="kw">for </span>G1 {
        <span class="kw">type </span>Error = AltBn128Error;

        <span class="kw">fn </span>try_from(bytes: PodG1) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, <span class="self">Self</span>::Error&gt; {
            <span class="kw">if </span>bytes.<span class="number">0 </span>== [<span class="number">0u8</span>; <span class="number">64</span>] {
                <span class="kw">return </span><span class="prelude-val">Ok</span>(G1::zero());
            }
            <span class="kw">let </span>g1 = <span class="self">Self</span>::deserialize_with_mode(
                <span class="kw-2">&amp;*</span>[<span class="kw-2">&amp;</span>bytes.<span class="number">0</span>[..], <span class="kw-2">&amp;</span>[<span class="number">0u8</span>][..]].concat(),
                Compress::No,
                Validate::Yes,
            );

            <span class="kw">match </span>g1 {
                <span class="prelude-val">Ok</span>(g1) =&gt; {
                    <span class="kw">if </span>!g1.is_on_curve() {
                        <span class="prelude-val">Err</span>(AltBn128Error::GroupError)
                    } <span class="kw">else </span>{
                        <span class="prelude-val">Ok</span>(g1)
                    }
                }
                <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; <span class="prelude-val">Err</span>(AltBn128Error::InvalidInputData),
            }
        }
    }

    <span class="kw">impl </span>TryFrom&lt;PodG2&gt; <span class="kw">for </span>G2 {
        <span class="kw">type </span>Error = AltBn128Error;

        <span class="kw">fn </span>try_from(bytes: PodG2) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, <span class="self">Self</span>::Error&gt; {
            <span class="kw">if </span>bytes.<span class="number">0 </span>== [<span class="number">0u8</span>; <span class="number">128</span>] {
                <span class="kw">return </span><span class="prelude-val">Ok</span>(G2::zero());
            }
            <span class="kw">let </span>g2 = <span class="self">Self</span>::deserialize_with_mode(
                <span class="kw-2">&amp;*</span>[<span class="kw-2">&amp;</span>bytes.<span class="number">0</span>[..], <span class="kw-2">&amp;</span>[<span class="number">0u8</span>][..]].concat(),
                Compress::No,
                Validate::Yes,
            );

            <span class="kw">match </span>g2 {
                <span class="prelude-val">Ok</span>(g2) =&gt; {
                    <span class="kw">if </span>!g2.is_on_curve() {
                        <span class="prelude-val">Err</span>(AltBn128Error::GroupError)
                    } <span class="kw">else </span>{
                        <span class="prelude-val">Ok</span>(g2)
                    }
                }
                <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; <span class="prelude-val">Err</span>(AltBn128Error::InvalidInputData),
            }
        }
    }

    <span class="kw">pub fn </span>alt_bn128_addition(input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;Vec&lt;u8&gt;, AltBn128Error&gt; {
        <span class="kw">if </span>input.len() &gt; ALT_BN128_ADDITION_INPUT_LEN {
            <span class="kw">return </span><span class="prelude-val">Err</span>(AltBn128Error::InvalidInputData);
        }

        <span class="kw">let </span><span class="kw-2">mut </span>input = input.to_vec();
        input.resize(ALT_BN128_ADDITION_INPUT_LEN, <span class="number">0</span>);

        <span class="kw">let </span>p: G1 = PodG1(
            convert_edianness_64(<span class="kw-2">&amp;</span>input[..<span class="number">64</span>])
                .try_into()
                .map_err(AltBn128Error::TryIntoVecError)<span class="question-mark">?</span>,
        )
        .try_into()<span class="question-mark">?</span>;
        <span class="kw">let </span>q: G1 = PodG1(
            convert_edianness_64(<span class="kw-2">&amp;</span>input[<span class="number">64</span>..ALT_BN128_ADDITION_INPUT_LEN])
                .try_into()
                .map_err(AltBn128Error::TryIntoVecError)<span class="question-mark">?</span>,
        )
        .try_into()<span class="question-mark">?</span>;

        <span class="attr">#[allow(clippy::arithmetic_side_effects)]
        </span><span class="kw">let </span>result_point = p + q;

        <span class="kw">let </span><span class="kw-2">mut </span>result_point_data = [<span class="number">0u8</span>; ALT_BN128_ADDITION_OUTPUT_LEN];
        <span class="kw">let </span>result_point_affine: G1 = result_point.into();
        result_point_affine
            .x
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>result_point_data[..<span class="number">32</span>], Compress::No)
            .map_err(|<span class="kw">_</span>| AltBn128Error::InvalidInputData)<span class="question-mark">?</span>;
        result_point_affine
            .y
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>result_point_data[<span class="number">32</span>..], Compress::No)
            .map_err(|<span class="kw">_</span>| AltBn128Error::InvalidInputData)<span class="question-mark">?</span>;

        <span class="prelude-val">Ok</span>(convert_edianness_64(<span class="kw-2">&amp;</span>result_point_data[..]).to_vec())
    }

    <span class="kw">pub fn </span>alt_bn128_multiplication(input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;Vec&lt;u8&gt;, AltBn128Error&gt; {
        <span class="kw">if </span>input.len() &gt; ALT_BN128_MULTIPLICATION_INPUT_LEN {
            <span class="kw">return </span><span class="prelude-val">Err</span>(AltBn128Error::InvalidInputData);
        }

        <span class="kw">let </span><span class="kw-2">mut </span>input = input.to_vec();
        input.resize(ALT_BN128_MULTIPLICATION_INPUT_LEN, <span class="number">0</span>);

        <span class="kw">let </span>p: G1 = PodG1(
            convert_edianness_64(<span class="kw-2">&amp;</span>input[..<span class="number">64</span>])
                .try_into()
                .map_err(AltBn128Error::TryIntoVecError)<span class="question-mark">?</span>,
        )
        .try_into()<span class="question-mark">?</span>;
        <span class="kw">let </span>fr = BigInteger256::deserialize_uncompressed_unchecked(
            <span class="kw-2">&amp;</span>convert_edianness_64(<span class="kw-2">&amp;</span>input[<span class="number">64</span>..<span class="number">96</span>])[..],
        )
        .map_err(|<span class="kw">_</span>| AltBn128Error::InvalidInputData)<span class="question-mark">?</span>;

        <span class="kw">let </span>result_point: G1 = p.mul_bigint(fr).into();

        <span class="kw">let </span><span class="kw-2">mut </span>result_point_data = [<span class="number">0u8</span>; ALT_BN128_MULTIPLICATION_OUTPUT_LEN];

        result_point
            .x
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>result_point_data[..<span class="number">32</span>], Compress::No)
            .map_err(|<span class="kw">_</span>| AltBn128Error::InvalidInputData)<span class="question-mark">?</span>;
        result_point
            .y
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>result_point_data[<span class="number">32</span>..], Compress::No)
            .map_err(|<span class="kw">_</span>| AltBn128Error::InvalidInputData)<span class="question-mark">?</span>;

        <span class="prelude-val">Ok</span>(
            convert_edianness_64(<span class="kw-2">&amp;</span>result_point_data[..ALT_BN128_MULTIPLICATION_OUTPUT_LEN])
                .to_vec(),
        )
    }

    <span class="kw">pub fn </span>alt_bn128_pairing(input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;Vec&lt;u8&gt;, AltBn128Error&gt; {
        <span class="kw">if </span>input
            .len()
            .checked_rem(consts::ALT_BN128_PAIRING_ELEMENT_LEN)
            .is_none()
        {
            <span class="kw">return </span><span class="prelude-val">Err</span>(AltBn128Error::InvalidInputData);
        }

        <span class="kw">let </span>ele_len = input.len().saturating_div(ALT_BN128_PAIRING_ELEMENT_LEN);

        <span class="kw">let </span><span class="kw-2">mut </span>vec_pairs: Vec&lt;(G1, G2)&gt; = Vec::new();
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..ele_len {
            vec_pairs.push((
                PodG1(
                    convert_edianness_64(
                        <span class="kw-2">&amp;</span>input[i.saturating_mul(ALT_BN128_PAIRING_ELEMENT_LEN)
                            ..i.saturating_mul(ALT_BN128_PAIRING_ELEMENT_LEN)
                                .saturating_add(ALT_BN128_POINT_SIZE)],
                    )
                    .try_into()
                    .map_err(AltBn128Error::TryIntoVecError)<span class="question-mark">?</span>,
                )
                .try_into()<span class="question-mark">?</span>,
                PodG2(
                    convert_edianness_128(
                        <span class="kw-2">&amp;</span>input[i
                            .saturating_mul(ALT_BN128_PAIRING_ELEMENT_LEN)
                            .saturating_add(ALT_BN128_POINT_SIZE)
                            ..i.saturating_mul(ALT_BN128_PAIRING_ELEMENT_LEN)
                                .saturating_add(ALT_BN128_PAIRING_ELEMENT_LEN)],
                    )
                    .try_into()
                    .map_err(AltBn128Error::TryIntoVecError)<span class="question-mark">?</span>,
                )
                .try_into()<span class="question-mark">?</span>,
            ));
        }

        <span class="kw">let </span><span class="kw-2">mut </span>result = BigInteger256::from(<span class="number">0u64</span>);
        <span class="kw">let </span>res = &lt;Bn&lt;Config&gt; <span class="kw">as </span>Pairing&gt;::multi_pairing(
            vec_pairs.iter().map(|pair| pair.<span class="number">0</span>),
            vec_pairs.iter().map(|pair| pair.<span class="number">1</span>),
        );

        <span class="kw">if </span>res.<span class="number">0 </span>== ark_bn254::Fq12::one() {
            result = BigInteger256::from(<span class="number">1u64</span>);
        }

        <span class="kw">let </span>output = result.to_bytes_be();
        <span class="prelude-val">Ok</span>(output)
    }

    <span class="kw">fn </span>convert_edianness_64(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; Vec&lt;u8&gt; {
        bytes
            .chunks(<span class="number">32</span>)
            .flat_map(|b| b.iter().copied().rev().collect::&lt;Vec&lt;u8&gt;&gt;())
            .collect::&lt;Vec&lt;u8&gt;&gt;()
    }

    <span class="kw">fn </span>convert_edianness_128(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; Vec&lt;u8&gt; {
        bytes
            .chunks(<span class="number">64</span>)
            .flat_map(|b| b.iter().copied().rev().collect::&lt;Vec&lt;u8&gt;&gt;())
            .collect::&lt;Vec&lt;u8&gt;&gt;()
    }
}

<span class="attr">#[cfg(target_os = <span class="string">"solana"</span>)]
</span><span class="kw">mod </span>target_arch {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;
    <span class="kw">pub fn </span>alt_bn128_addition(input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;Vec&lt;u8&gt;, AltBn128Error&gt; {
        <span class="kw">if </span>input.len() &gt; ALT_BN128_ADDITION_INPUT_LEN {
            <span class="kw">return </span><span class="prelude-val">Err</span>(AltBn128Error::InvalidInputData);
        }
        <span class="kw">let </span><span class="kw-2">mut </span>result_buffer = [<span class="number">0</span>; ALT_BN128_ADDITION_OUTPUT_LEN];
        <span class="kw">let </span>result = <span class="kw">unsafe </span>{
            <span class="kw">crate</span>::syscalls::sol_alt_bn128_group_op(
                ALT_BN128_ADD,
                input <span class="kw">as </span><span class="kw-2">*const </span><span class="kw">_ as </span><span class="kw-2">*const </span>u8,
                input.len() <span class="kw">as </span>u64,
                <span class="kw-2">&amp;mut </span>result_buffer <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_ as </span><span class="kw-2">*mut </span>u8,
            )
        };

        <span class="kw">match </span>result {
            <span class="number">0 </span>=&gt; <span class="prelude-val">Ok</span>(result_buffer.to_vec()),
            error =&gt; <span class="prelude-val">Err</span>(AltBn128Error::from(error)),
        }
    }

    <span class="kw">pub fn </span>alt_bn128_multiplication(input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;Vec&lt;u8&gt;, AltBn128Error&gt; {
        <span class="kw">if </span>input.len() &gt; ALT_BN128_MULTIPLICATION_INPUT_LEN {
            <span class="kw">return </span><span class="prelude-val">Err</span>(AltBn128Error::InvalidInputData);
        }
        <span class="kw">let </span><span class="kw-2">mut </span>result_buffer = [<span class="number">0u8</span>; ALT_BN128_POINT_SIZE];
        <span class="kw">let </span>result = <span class="kw">unsafe </span>{
            <span class="kw">crate</span>::syscalls::sol_alt_bn128_group_op(
                ALT_BN128_MUL,
                input <span class="kw">as </span><span class="kw-2">*const </span><span class="kw">_ as </span><span class="kw-2">*const </span>u8,
                input.len() <span class="kw">as </span>u64,
                <span class="kw-2">&amp;mut </span>result_buffer <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_ as </span><span class="kw-2">*mut </span>u8,
            )
        };

        <span class="kw">match </span>result {
            <span class="number">0 </span>=&gt; <span class="prelude-val">Ok</span>(result_buffer.to_vec()),
            error =&gt; <span class="prelude-val">Err</span>(AltBn128Error::from(error)),
        }
    }

    <span class="kw">pub fn </span>alt_bn128_pairing(input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;Vec&lt;u8&gt;, AltBn128Error&gt; {
        <span class="kw">if </span>input
            .len()
            .checked_rem(consts::ALT_BN128_PAIRING_ELEMENT_LEN)
            .is_none()
        {
            <span class="kw">return </span><span class="prelude-val">Err</span>(AltBn128Error::InvalidInputData);
        }
        <span class="kw">let </span><span class="kw-2">mut </span>result_buffer = [<span class="number">0u8</span>; <span class="number">32</span>];
        <span class="kw">let </span>result = <span class="kw">unsafe </span>{
            <span class="kw">crate</span>::syscalls::sol_alt_bn128_group_op(
                ALT_BN128_PAIRING,
                input <span class="kw">as </span><span class="kw-2">*const </span><span class="kw">_ as </span><span class="kw-2">*const </span>u8,
                input.len() <span class="kw">as </span>u64,
                <span class="kw-2">&amp;mut </span>result_buffer <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_ as </span><span class="kw-2">*mut </span>u8,
            )
        };

        <span class="kw">match </span>result {
            <span class="number">0 </span>=&gt; <span class="prelude-val">Ok</span>(result_buffer.to_vec()),
            error =&gt; <span class="prelude-val">Err</span>(AltBn128Error::from(error)),
        }
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span>{
        <span class="kw">crate</span>::alt_bn128::{prelude::<span class="kw-2">*</span>, PodG1},
        ark_bn254::g1::G1Affine,
        ark_ec::AffineRepr,
        ark_serialize::{CanonicalSerialize, Compress},
    };

    <span class="attr">#[test]
    </span><span class="kw">fn </span>zero_serialization_test() {
        <span class="kw">let </span>zero = G1Affine::zero();
        <span class="kw">let </span><span class="kw-2">mut </span>result_point_data = [<span class="number">0u8</span>; <span class="number">64</span>];
        zero.x
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>result_point_data[..<span class="number">32</span>], Compress::No)
            .map_err(|<span class="kw">_</span>| AltBn128Error::InvalidInputData)
            .unwrap();
        zero.y
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>result_point_data[<span class="number">32</span>..], Compress::No)
            .map_err(|<span class="kw">_</span>| AltBn128Error::InvalidInputData)
            .unwrap();
        <span class="macro">assert_eq!</span>(result_point_data, [<span class="number">0u8</span>; <span class="number">64</span>]);

        <span class="kw">let </span>p: G1Affine = PodG1(result_point_data[..<span class="number">64</span>].try_into().unwrap())
            .try_into()
            .unwrap();
        <span class="macro">assert_eq!</span>(p, zero);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>alt_bn128_addition_test() {
        <span class="kw">use </span>serde::Deserialize;

        <span class="kw">let </span>test_data = <span class="string">r#"[
        {
            "Input": "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f3726607c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7",
            "Expected": "2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c915",
            "Name": "chfast1",
            "Gas": 150,
            "NoBenchmark": false
        },{
            "Input": "2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c91518b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266",
            "Expected": "2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb721611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb204",
            "Name": "chfast2",
            "Gas": 150,
            "NoBenchmark": false
        },{
            "Input": "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "Expected": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "Name": "cdetrio1",
            "Gas": 150,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "Expected": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "Name": "cdetrio2",
            "Gas": 150,
            "NoBenchmark": false
        },{
            "Input": "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "Expected": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "Name": "cdetrio3",
            "Gas": 150,
            "NoBenchmark": false
        },{
            "Input": "",
            "Expected": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "Name": "cdetrio4",
            "Gas": 150,
            "NoBenchmark": false
        },{
            "Input": "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
            "Expected": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
            "Name": "cdetrio5",
            "Gas": 150,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
            "Expected": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
            "Name": "cdetrio6",
            "Gas": 150,
            "NoBenchmark": false
        },{
            "Input": "0000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "Expected": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
            "Gas": 150,
            "Name": "cdetrio7",
            "NoBenchmark": false
        },{
            "Input": "0000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
            "Expected": "030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd315ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4",
            "Name": "cdetrio8",
            "Gas": 150,
            "NoBenchmark": false
        },{
            "Input": "17c139df0efee0f766bc0204762b774362e4ded88953a39ce849a8a7fa163fa901e0559bacb160664764a357af8a9fe70baa9258e0b959273ffc5718c6d4cc7c039730ea8dff1254c0fee9c0ea777d29a9c710b7e616683f194f18c43b43b869073a5ffcc6fc7a28c30723d6e58ce577356982d65b833a5a5c15bf9024b43d98",
            "Expected": "15bf2bb17880144b5d1cd2b1f46eff9d617bffd1ca57c37fb5a49bd84e53cf66049c797f9ce0d17083deb32b5e36f2ea2a212ee036598dd7624c168993d1355f",
            "Name": "cdetrio9",
            "Gas": 150,
            "NoBenchmark": false
        }
        ]"#</span>;

        <span class="attr">#[derive(Deserialize)]
        #[serde(rename_all = <span class="string">"PascalCase"</span>)]
        </span><span class="kw">struct </span>TestCase {
            input: String,
            expected: String,
        }

        <span class="kw">let </span>test_cases: Vec&lt;TestCase&gt; = serde_json::from_str(test_data).unwrap();

        test_cases.iter().for_each(|test| {
            <span class="kw">let </span>input = array_bytes::hex2bytes_unchecked(<span class="kw-2">&amp;</span>test.input);
            <span class="kw">let </span>result = alt_bn128_addition(<span class="kw-2">&amp;</span>input);
            <span class="macro">assert!</span>(result.is_ok());

            <span class="kw">let </span>expected = array_bytes::hex2bytes_unchecked(<span class="kw-2">&amp;</span>test.expected);

            <span class="macro">assert_eq!</span>(result.unwrap(), expected);
        });
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>alt_bn128_multiplication_test() {
        <span class="kw">use </span>serde::Deserialize;

        <span class="kw">let </span>test_data = <span class="string">r#"[
        {
            "Input": "2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb721611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb20400000000000000000000000000000000000000000000000011138ce750fa15c2",
            "Expected": "070a8d6a982153cae4be29d434e8faef8a47b274a053f5a4ee2a6c9c13c31e5c031b8ce914eba3a9ffb989f9cdd5b0f01943074bf4f0f315690ec3cec6981afc",
            "Name": "chfast1",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "070a8d6a982153cae4be29d434e8faef8a47b274a053f5a4ee2a6c9c13c31e5c031b8ce914eba3a9ffb989f9cdd5b0f01943074bf4f0f315690ec3cec6981afc30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46",
            "Expected": "025a6f4181d2b4ea8b724290ffb40156eb0adb514c688556eb79cdea0752c2bb2eff3f31dea215f1eb86023a133a996eb6300b44da664d64251d05381bb8a02e",
            "Name": "chfast2",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "025a6f4181d2b4ea8b724290ffb40156eb0adb514c688556eb79cdea0752c2bb2eff3f31dea215f1eb86023a133a996eb6300b44da664d64251d05381bb8a02e183227397098d014dc2822db40c0ac2ecbc0b548b438e5469e10460b6c3e7ea3",
            "Expected": "14789d0d4a730b354403b5fac948113739e276c23e0258d8596ee72f9cd9d3230af18a63153e0ec25ff9f2951dd3fa90ed0197bfef6e2a1a62b5095b9d2b4a27",
            "Name": "chfast3",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "1a87b0584ce92f4593d161480614f2989035225609f08058ccfa3d0f940febe31a2f3c951f6dadcc7ee9007dff81504b0fcd6d7cf59996efdc33d92bf7f9f8f6ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "Expected": "2cde5879ba6f13c0b5aa4ef627f159a3347df9722efce88a9afbb20b763b4c411aa7e43076f6aee272755a7f9b84832e71559ba0d2e0b17d5f9f01755e5b0d11",
            "Name": "cdetrio1",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "1a87b0584ce92f4593d161480614f2989035225609f08058ccfa3d0f940febe31a2f3c951f6dadcc7ee9007dff81504b0fcd6d7cf59996efdc33d92bf7f9f8f630644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000",
            "Expected": "1a87b0584ce92f4593d161480614f2989035225609f08058ccfa3d0f940febe3163511ddc1c3f25d396745388200081287b3fd1472d8339d5fecb2eae0830451",
            "Name": "cdetrio2",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "1a87b0584ce92f4593d161480614f2989035225609f08058ccfa3d0f940febe31a2f3c951f6dadcc7ee9007dff81504b0fcd6d7cf59996efdc33d92bf7f9f8f60000000000000000000000000000000100000000000000000000000000000000",
            "Expected": "1051acb0700ec6d42a88215852d582efbaef31529b6fcbc3277b5c1b300f5cf0135b2394bb45ab04b8bd7611bd2dfe1de6a4e6e2ccea1ea1955f577cd66af85b",
            "Name": "cdetrio3",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "1a87b0584ce92f4593d161480614f2989035225609f08058ccfa3d0f940febe31a2f3c951f6dadcc7ee9007dff81504b0fcd6d7cf59996efdc33d92bf7f9f8f60000000000000000000000000000000000000000000000000000000000000009",
            "Expected": "1dbad7d39dbc56379f78fac1bca147dc8e66de1b9d183c7b167351bfe0aeab742cd757d51289cd8dbd0acf9e673ad67d0f0a89f912af47ed1be53664f5692575",
            "Name": "cdetrio4",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "1a87b0584ce92f4593d161480614f2989035225609f08058ccfa3d0f940febe31a2f3c951f6dadcc7ee9007dff81504b0fcd6d7cf59996efdc33d92bf7f9f8f60000000000000000000000000000000000000000000000000000000000000001",
            "Expected": "1a87b0584ce92f4593d161480614f2989035225609f08058ccfa3d0f940febe31a2f3c951f6dadcc7ee9007dff81504b0fcd6d7cf59996efdc33d92bf7f9f8f6",
            "Name": "cdetrio5",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "17c139df0efee0f766bc0204762b774362e4ded88953a39ce849a8a7fa163fa901e0559bacb160664764a357af8a9fe70baa9258e0b959273ffc5718c6d4cc7cffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "Expected": "29e587aadd7c06722aabba753017c093f70ba7eb1f1c0104ec0564e7e3e21f6022b1143f6a41008e7755c71c3d00b6b915d386de21783ef590486d8afa8453b1",
            "Name": "cdetrio6",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "17c139df0efee0f766bc0204762b774362e4ded88953a39ce849a8a7fa163fa901e0559bacb160664764a357af8a9fe70baa9258e0b959273ffc5718c6d4cc7c30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000",
            "Expected": "17c139df0efee0f766bc0204762b774362e4ded88953a39ce849a8a7fa163fa92e83f8d734803fc370eba25ed1f6b8768bd6d83887b87165fc2434fe11a830cb",
            "Name": "cdetrio7",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "17c139df0efee0f766bc0204762b774362e4ded88953a39ce849a8a7fa163fa901e0559bacb160664764a357af8a9fe70baa9258e0b959273ffc5718c6d4cc7c0000000000000000000000000000000100000000000000000000000000000000",
            "Expected": "221a3577763877920d0d14a91cd59b9479f83b87a653bb41f82a3f6f120cea7c2752c7f64cdd7f0e494bff7b60419f242210f2026ed2ec70f89f78a4c56a1f15",
            "Name": "cdetrio8",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "17c139df0efee0f766bc0204762b774362e4ded88953a39ce849a8a7fa163fa901e0559bacb160664764a357af8a9fe70baa9258e0b959273ffc5718c6d4cc7c0000000000000000000000000000000000000000000000000000000000000009",
            "Expected": "228e687a379ba154554040f8821f4e41ee2be287c201aa9c3bc02c9dd12f1e691e0fd6ee672d04cfd924ed8fdc7ba5f2d06c53c1edc30f65f2af5a5b97f0a76a",
            "Name": "cdetrio9",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "17c139df0efee0f766bc0204762b774362e4ded88953a39ce849a8a7fa163fa901e0559bacb160664764a357af8a9fe70baa9258e0b959273ffc5718c6d4cc7c0000000000000000000000000000000000000000000000000000000000000001",
            "Expected": "17c139df0efee0f766bc0204762b774362e4ded88953a39ce849a8a7fa163fa901e0559bacb160664764a357af8a9fe70baa9258e0b959273ffc5718c6d4cc7c",
            "Name": "cdetrio10",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "039730ea8dff1254c0fee9c0ea777d29a9c710b7e616683f194f18c43b43b869073a5ffcc6fc7a28c30723d6e58ce577356982d65b833a5a5c15bf9024b43d98ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "Expected": "00a1a234d08efaa2616607e31eca1980128b00b415c845ff25bba3afcb81dc00242077290ed33906aeb8e42fd98c41bcb9057ba03421af3f2d08cfc441186024",
            "Name": "cdetrio11",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "039730ea8dff1254c0fee9c0ea777d29a9c710b7e616683f194f18c43b43b869073a5ffcc6fc7a28c30723d6e58ce577356982d65b833a5a5c15bf9024b43d9830644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000",
            "Expected": "039730ea8dff1254c0fee9c0ea777d29a9c710b7e616683f194f18c43b43b8692929ee761a352600f54921df9bf472e66217e7bb0cee9032e00acc86b3c8bfaf",
            "Name": "cdetrio12",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "039730ea8dff1254c0fee9c0ea777d29a9c710b7e616683f194f18c43b43b869073a5ffcc6fc7a28c30723d6e58ce577356982d65b833a5a5c15bf9024b43d980000000000000000000000000000000100000000000000000000000000000000",
            "Expected": "1071b63011e8c222c5a771dfa03c2e11aac9666dd097f2c620852c3951a4376a2f46fe2f73e1cf310a168d56baa5575a8319389d7bfa6b29ee2d908305791434",
            "Name": "cdetrio13",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "039730ea8dff1254c0fee9c0ea777d29a9c710b7e616683f194f18c43b43b869073a5ffcc6fc7a28c30723d6e58ce577356982d65b833a5a5c15bf9024b43d980000000000000000000000000000000000000000000000000000000000000009",
            "Expected": "19f75b9dd68c080a688774a6213f131e3052bd353a304a189d7a2ee367e3c2582612f545fb9fc89fde80fd81c68fc7dcb27fea5fc124eeda69433cf5c46d2d7f",
            "Name": "cdetrio14",
            "Gas": 6000,
            "NoBenchmark": true
        },{
            "Input": "039730ea8dff1254c0fee9c0ea777d29a9c710b7e616683f194f18c43b43b869073a5ffcc6fc7a28c30723d6e58ce577356982d65b833a5a5c15bf9024b43d980000000000000000000000000000000000000000000000000000000000000001",
            "Expected": "039730ea8dff1254c0fee9c0ea777d29a9c710b7e616683f194f18c43b43b869073a5ffcc6fc7a28c30723d6e58ce577356982d65b833a5a5c15bf9024b43d98",
            "Name": "cdetrio15",
            "Gas": 6000,
            "NoBenchmark": true
        }
        ]"#</span>;

        <span class="attr">#[derive(Deserialize)]
        #[serde(rename_all = <span class="string">"PascalCase"</span>)]
        </span><span class="kw">struct </span>TestCase {
            input: String,
            expected: String,
        }

        <span class="kw">let </span>test_cases: Vec&lt;TestCase&gt; = serde_json::from_str(test_data).unwrap();

        test_cases.iter().for_each(|test| {
            <span class="kw">let </span>input = array_bytes::hex2bytes_unchecked(<span class="kw-2">&amp;</span>test.input);
            <span class="kw">let </span>result = alt_bn128_multiplication(<span class="kw-2">&amp;</span>input);
            <span class="macro">assert!</span>(result.is_ok());
            <span class="kw">let </span>expected = array_bytes::hex2bytes_unchecked(<span class="kw-2">&amp;</span>test.expected);
            <span class="macro">assert_eq!</span>(result.unwrap(), expected);
        });
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>alt_bn128_pairing_test() {
        <span class="kw">use </span>serde::Deserialize;

        <span class="kw">let </span>test_data = <span class="string">r#"[
        {
            "Input": "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "jeff1",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "2eca0c7238bf16e83e7a1e6c5d49540685ff51380f309842a98561558019fc0203d3260361bb8451de5ff5ecd17f010ff22f5c31cdf184e9020b06fa5997db841213d2149b006137fcfb23036606f848d638d576a120ca981b5b1a5f9300b3ee2276cf730cf493cd95d64677bbb75fc42db72513a4c1e387b476d056f80aa75f21ee6226d31426322afcda621464d0611d226783262e21bb3bc86b537e986237096df1f82dff337dd5972e32a8ad43e28a78a96a823ef1cd4debe12b6552ea5f06967a1237ebfeca9aaae0d6d0bab8e28c198c5a339ef8a2407e31cdac516db922160fa257a5fd5b280642ff47b65eca77e626cb685c84fa6d3b6882a283ddd1198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "jeff2",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "0f25929bcb43d5a57391564615c9e70a992b10eafa4db109709649cf48c50dd216da2f5cb6be7a0aa72c440c53c9bbdfec6c36c7d515536431b3a865468acbba2e89718ad33c8bed92e210e81d1853435399a271913a6520736a4729cf0d51eb01a9e2ffa2e92599b68e44de5bcf354fa2642bd4f26b259daa6f7ce3ed57aeb314a9a87b789a58af499b314e13c3d65bede56c07ea2d418d6874857b70763713178fb49a2d6cd347dc58973ff49613a20757d0fcc22079f9abd10c3baee245901b9e027bd5cfc2cb5db82d4dc9677ac795ec500ecd47deee3b5da006d6d049b811d7511c78158de484232fc68daf8a45cf217d1c2fae693ff5871e8752d73b21198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "jeff3",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "2f2ea0b3da1e8ef11914acf8b2e1b32d99df51f5f4f206fc6b947eae860eddb6068134ddb33dc888ef446b648d72338684d678d2eb2371c61a50734d78da4b7225f83c8b6ab9de74e7da488ef02645c5a16a6652c3c71a15dc37fe3a5dcb7cb122acdedd6308e3bb230d226d16a105295f523a8a02bfc5e8bd2da135ac4c245d065bbad92e7c4e31bf3757f1fe7362a63fbfee50e7dc68da116e67d600d9bf6806d302580dc0661002994e7cd3a7f224e7ddc27802777486bf80f40e4ca3cfdb186bac5188a98c45e6016873d107f5cd131f3a3e339d0375e58bd6219347b008122ae2b09e539e152ec5364e7e2204b03d11d3caa038bfc7cd499f8176aacbee1f39e4e4afc4bc74790a4a028aff2c3d2538731fb755edefd8cb48d6ea589b5e283f150794b6736f670d6a1033f9b46c6f5204f50813eb85c8dc4b59db1c5d39140d97ee4d2b36d99bc49974d18ecca3e7ad51011956051b464d9e27d46cc25e0764bb98575bd466d32db7b15f582b2d5c452b36aa394b789366e5e3ca5aabd415794ab061441e51d01e94640b7e3084a07e02c78cf3103c542bc5b298669f211b88da1679b0b64a63b7e0e7bfe52aae524f73a55be7fe70c7e9bfc94b4cf0da1213d2149b006137fcfb23036606f848d638d576a120ca981b5b1a5f9300b3ee2276cf730cf493cd95d64677bbb75fc42db72513a4c1e387b476d056f80aa75f21ee6226d31426322afcda621464d0611d226783262e21bb3bc86b537e986237096df1f82dff337dd5972e32a8ad43e28a78a96a823ef1cd4debe12b6552ea5f",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "jeff4",
            "Gas": 147000,
            "NoBenchmark": false
        },{
            "Input": "20a754d2071d4d53903e3b31a7e98ad6882d58aec240ef981fdf0a9d22c5926a29c853fcea789887315916bbeb89ca37edb355b4f980c9a12a94f30deeed30211213d2149b006137fcfb23036606f848d638d576a120ca981b5b1a5f9300b3ee2276cf730cf493cd95d64677bbb75fc42db72513a4c1e387b476d056f80aa75f21ee6226d31426322afcda621464d0611d226783262e21bb3bc86b537e986237096df1f82dff337dd5972e32a8ad43e28a78a96a823ef1cd4debe12b6552ea5f1abb4a25eb9379ae96c84fff9f0540abcfc0a0d11aeda02d4f37e4baf74cb0c11073b3ff2cdbb38755f8691ea59e9606696b3ff278acfc098fa8226470d03869217cee0a9ad79a4493b5253e2e4e3a39fc2df38419f230d341f60cb064a0ac290a3d76f140db8418ba512272381446eb73958670f00cf46f1d9e64cba057b53c26f64a8ec70387a13e41430ed3ee4a7db2059cc5fc13c067194bcc0cb49a98552fd72bd9edb657346127da132e5b82ab908f5816c826acb499e22f2412d1a2d70f25929bcb43d5a57391564615c9e70a992b10eafa4db109709649cf48c50dd2198a1f162a73261f112401aa2db79c7dab1533c9935c77290a6ce3b191f2318d198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "jeff5",
            "Gas": 147000,
            "NoBenchmark": false
        },{
            "Input": "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c103188585e2364128fe25c70558f1560f4f9350baf3959e603cc91486e110936198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000000",
            "Name": "jeff6",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "empty_data",
            "Gas": 45000,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000000",
            "Name": "one_point",
            "Gas": 79000,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "two_point_match_2",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "two_point_match_3",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "105456a333e6d636854f987ea7bb713dfd0ae8371a72aea313ae0c32c0bf10160cf031d41b41557f3e7e3ba0c51bebe5da8e6ecd855ec50fc87efcdeac168bcc0476be093a6d2b4bbf907172049874af11e1b6267606e00804d3ff0037ec57fd3010c68cb50161b7d1d96bb71edfec9880171954e56871abf3d93cc94d745fa114c059d74e5b6c4ec14ae5864ebe23a71781d86c29fb8fb6cce94f70d3de7a2101b33461f39d9e887dbb100f170a2345dde3c07e256d1dfa2b657ba5cd030427000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000021a2c3013d2ea92e13c800cde68ef56a294b883f6ac35d25f587c09b1b3c635f7290158a80cd3d66530f74dc94c94adb88f5cdb481acca997b6e60071f08a115f2f997f3dbd66a7afe07fe7862ce239edba9e05c5afff7f8a1259c9733b2dfbb929d1691530ca701b4a106054688728c9972c8512e9789e9567aae23e302ccd75",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "two_point_match_4",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "ten_point_match_1",
            "Gas": 385000,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "ten_point_match_2",
            "Gas": 385000,
            "NoBenchmark": false
        },{
            "Input": "105456a333e6d636854f987ea7bb713dfd0ae8371a72aea313ae0c32c0bf10160cf031d41b41557f3e7e3ba0c51bebe5da8e6ecd855ec50fc87efcdeac168bcc0476be093a6d2b4bbf907172049874af11e1b6267606e00804d3ff0037ec57fd3010c68cb50161b7d1d96bb71edfec9880171954e56871abf3d93cc94d745fa114c059d74e5b6c4ec14ae5864ebe23a71781d86c29fb8fb6cce94f70d3de7a2101b33461f39d9e887dbb100f170a2345dde3c07e256d1dfa2b657ba5cd030427000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000021a2c3013d2ea92e13c800cde68ef56a294b883f6ac35d25f587c09b1b3c635f7290158a80cd3d66530f74dc94c94adb88f5cdb481acca997b6e60071f08a115f2f997f3dbd66a7afe07fe7862ce239edba9e05c5afff7f8a1259c9733b2dfbb929d1691530ca701b4a106054688728c9972c8512e9789e9567aae23e302ccd75",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "ten_point_match_3",
            "Gas": 113000,
            "NoBenchmark": false
        }
        ]"#</span>;

        <span class="attr">#[derive(Deserialize)]
        #[serde(rename_all = <span class="string">"PascalCase"</span>)]
        </span><span class="kw">struct </span>TestCase {
            input: String,
            expected: String,
        }

        <span class="kw">let </span>test_cases: Vec&lt;TestCase&gt; = serde_json::from_str(test_data).unwrap();

        test_cases.iter().for_each(|test| {
            <span class="kw">let </span>input = array_bytes::hex2bytes_unchecked(<span class="kw-2">&amp;</span>test.input);
            <span class="kw">let </span>result = alt_bn128_pairing(<span class="kw-2">&amp;</span>input);
            <span class="macro">assert!</span>(result.is_ok());
            <span class="kw">let </span>expected = array_bytes::hex2bytes_unchecked(<span class="kw-2">&amp;</span>test.expected);
            <span class="macro">assert_eq!</span>(result.unwrap(), expected);
        });
    }
}
</code></pre></div></section></main></body></html>