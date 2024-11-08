<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/chrono-0.4.38/src/format/parse.rs`."><title>parse.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="chrono" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../chrono/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// This is a part of Chrono.
// Portions copyright (c) 2015, John Nagle.
// See README.md and LICENSE.txt for details.

</span><span class="doccomment">//! Date and time parsing routines.

</span><span class="kw">use </span>core::borrow::Borrow;
<span class="kw">use </span>core::str;
<span class="kw">use </span>core::usize;

<span class="kw">use </span><span class="kw">super</span>::scan;
<span class="kw">use super</span>::{Fixed, InternalFixed, InternalInternal, Item, Numeric, Pad, Parsed};
<span class="kw">use super</span>::{ParseError, ParseResult};
<span class="kw">use super</span>::{BAD_FORMAT, INVALID, OUT_OF_RANGE, TOO_LONG, TOO_SHORT};
<span class="kw">use crate</span>::{DateTime, FixedOffset, Weekday};

<span class="kw">fn </span>set_weekday_with_num_days_from_sunday(p: <span class="kw-2">&amp;mut </span>Parsed, v: i64) -&gt; ParseResult&lt;()&gt; {
    p.set_weekday(<span class="kw">match </span>v {
        <span class="number">0 </span>=&gt; Weekday::Sun,
        <span class="number">1 </span>=&gt; Weekday::Mon,
        <span class="number">2 </span>=&gt; Weekday::Tue,
        <span class="number">3 </span>=&gt; Weekday::Wed,
        <span class="number">4 </span>=&gt; Weekday::Thu,
        <span class="number">5 </span>=&gt; Weekday::Fri,
        <span class="number">6 </span>=&gt; Weekday::Sat,
        <span class="kw">_ </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(OUT_OF_RANGE),
    })
}

<span class="kw">fn </span>set_weekday_with_number_from_monday(p: <span class="kw-2">&amp;mut </span>Parsed, v: i64) -&gt; ParseResult&lt;()&gt; {
    p.set_weekday(<span class="kw">match </span>v {
        <span class="number">1 </span>=&gt; Weekday::Mon,
        <span class="number">2 </span>=&gt; Weekday::Tue,
        <span class="number">3 </span>=&gt; Weekday::Wed,
        <span class="number">4 </span>=&gt; Weekday::Thu,
        <span class="number">5 </span>=&gt; Weekday::Fri,
        <span class="number">6 </span>=&gt; Weekday::Sat,
        <span class="number">7 </span>=&gt; Weekday::Sun,
        <span class="kw">_ </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(OUT_OF_RANGE),
    })
}

<span class="kw">fn </span>parse_rfc2822&lt;<span class="lifetime">'a</span>&gt;(parsed: <span class="kw-2">&amp;mut </span>Parsed, <span class="kw-2">mut </span>s: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str, ())&gt; {
    <span class="macro">macro_rules!</span> try_consume {
        (<span class="macro-nonterminal">$e</span>:expr) =&gt; {{
            <span class="kw">let </span>(s_, v) = <span class="macro-nonterminal">$e</span><span class="question-mark">?</span>;
            s = s_;
            v
        }};
    }

    <span class="comment">// an adapted RFC 2822 syntax from Section 3.3 and 4.3:
    //
    // c-char      = &lt;any char except '(', ')' and '\\'&gt;
    // c-escape    = "\" &lt;any char&gt;
    // comment     = "(" *(comment / c-char / c-escape) ")" *S
    // date-time   = [ day-of-week "," ] date 1*S time *S *comment
    // day-of-week = *S day-name *S
    // day-name    = "Mon" / "Tue" / "Wed" / "Thu" / "Fri" / "Sat" / "Sun"
    // date        = day month year
    // day         = *S 1*2DIGIT *S
    // month       = 1*S month-name 1*S
    // month-name  = "Jan" / "Feb" / "Mar" / "Apr" / "May" / "Jun" /
    //               "Jul" / "Aug" / "Sep" / "Oct" / "Nov" / "Dec"
    // year        = *S 2*DIGIT *S
    // time        = time-of-day 1*S zone
    // time-of-day = hour ":" minute [ ":" second ]
    // hour        = *S 2DIGIT *S
    // minute      = *S 2DIGIT *S
    // second      = *S 2DIGIT *S
    // zone        = ( "+" / "-" ) 4DIGIT /
    //               "UT" / "GMT" /                  ; same as +0000
    //               "EST" / "CST" / "MST" / "PST" / ; same as -0500 to -0800
    //               "EDT" / "CDT" / "MDT" / "PDT" / ; same as -0400 to -0700
    //               1*(%d65-90 / %d97-122)          ; same as -0000
    //
    // some notes:
    //
    // - quoted characters can be in any mixture of lower and upper cases.
    //
    // - we do not recognize a folding white space (FWS) or comment (CFWS).
    //   for our purposes, instead, we accept any sequence of Unicode
    //   white space characters (denoted here to `S`). For comments, we accept
    //   any text within parentheses while respecting escaped parentheses.
    //   Any actual RFC 2822 parser is expected to parse FWS and/or CFWS themselves
    //   and replace it with a single SP (`%x20`); this is legitimate.
    //
    // - two-digit year &lt; 50 should be interpreted by adding 2000.
    //   two-digit year &gt;= 50 or three-digit year should be interpreted
    //   by adding 1900. note that four-or-more-digit years less than 1000
    //   are *never* affected by this rule.
    //
    // - mismatching day-of-week is always an error, which is consistent to
    //   Chrono's own rules.
    //
    // - zones can range from `-9959` to `+9959`, but `FixedOffset` does not
    //   support offsets larger than 24 hours. this is not *that* problematic
    //   since we do not directly go to a `DateTime` so one can recover
    //   the offset information from `Parsed` anyway.

    </span>s = s.trim_start();

    <span class="kw">if let </span><span class="prelude-val">Ok</span>((s_, weekday)) = scan::short_weekday(s) {
        <span class="kw">if </span>!s_.starts_with(<span class="string">','</span>) {
            <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID);
        }
        s = <span class="kw-2">&amp;</span>s_[<span class="number">1</span>..];
        parsed.set_weekday(weekday)<span class="question-mark">?</span>;
    }

    s = s.trim_start();
    parsed.set_day(<span class="macro">try_consume!</span>(scan::number(s, <span class="number">1</span>, <span class="number">2</span>)))<span class="question-mark">?</span>;
    s = scan::space(s)<span class="question-mark">?</span>; <span class="comment">// mandatory
    </span>parsed.set_month(<span class="number">1 </span>+ i64::from(<span class="macro">try_consume!</span>(scan::short_month0(s))))<span class="question-mark">?</span>;
    s = scan::space(s)<span class="question-mark">?</span>; <span class="comment">// mandatory

    // distinguish two- and three-digit years from four-digit years
    </span><span class="kw">let </span>prevlen = s.len();
    <span class="kw">let </span><span class="kw-2">mut </span>year = <span class="macro">try_consume!</span>(scan::number(s, <span class="number">2</span>, usize::MAX));
    <span class="kw">let </span>yearlen = prevlen - s.len();
    <span class="kw">match </span>(yearlen, year) {
        (<span class="number">2</span>, <span class="number">0</span>..=<span class="number">49</span>) =&gt; {
            year += <span class="number">2000</span>;
        } <span class="comment">//   47 -&gt; 2047,   05 -&gt; 2005
        </span>(<span class="number">2</span>, <span class="number">50</span>..=<span class="number">99</span>) =&gt; {
            year += <span class="number">1900</span>;
        } <span class="comment">//   79 -&gt; 1979
        </span>(<span class="number">3</span>, <span class="kw">_</span>) =&gt; {
            year += <span class="number">1900</span>;
        } <span class="comment">//  112 -&gt; 2012,  009 -&gt; 1909
        </span>(<span class="kw">_</span>, <span class="kw">_</span>) =&gt; {} <span class="comment">// 1987 -&gt; 1987, 0654 -&gt; 0654
    </span>}
    parsed.set_year(year)<span class="question-mark">?</span>;

    s = scan::space(s)<span class="question-mark">?</span>; <span class="comment">// mandatory
    </span>parsed.set_hour(<span class="macro">try_consume!</span>(scan::number(s, <span class="number">2</span>, <span class="number">2</span>)))<span class="question-mark">?</span>;
    s = scan::char(s.trim_start(), <span class="string">b':'</span>)<span class="question-mark">?</span>.trim_start(); <span class="comment">// *S ":" *S
    </span>parsed.set_minute(<span class="macro">try_consume!</span>(scan::number(s, <span class="number">2</span>, <span class="number">2</span>)))<span class="question-mark">?</span>;
    <span class="kw">if let </span><span class="prelude-val">Ok</span>(s_) = scan::char(s.trim_start(), <span class="string">b':'</span>) {
        <span class="comment">// [ ":" *S 2DIGIT ]
        </span>parsed.set_second(<span class="macro">try_consume!</span>(scan::number(s_, <span class="number">2</span>, <span class="number">2</span>)))<span class="question-mark">?</span>;
    }

    s = scan::space(s)<span class="question-mark">?</span>; <span class="comment">// mandatory
    </span>parsed.set_offset(i64::from(<span class="macro">try_consume!</span>(scan::timezone_offset_2822(s))))<span class="question-mark">?</span>;

    <span class="comment">// optional comments
    </span><span class="kw">while let </span><span class="prelude-val">Ok</span>((s_out, ())) = scan::comment_2822(s) {
        s = s_out;
    }

    <span class="prelude-val">Ok</span>((s, ()))
}

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>parse_rfc3339&lt;<span class="lifetime">'a</span>&gt;(parsed: <span class="kw-2">&amp;mut </span>Parsed, <span class="kw-2">mut </span>s: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str, ())&gt; {
    <span class="macro">macro_rules!</span> try_consume {
        (<span class="macro-nonterminal">$e</span>:expr) =&gt; {{
            <span class="kw">let </span>(s_, v) = <span class="macro-nonterminal">$e</span><span class="question-mark">?</span>;
            s = s_;
            v
        }};
    }

    <span class="comment">// an adapted RFC 3339 syntax from Section 5.6:
    //
    // date-fullyear  = 4DIGIT
    // date-month     = 2DIGIT ; 01-12
    // date-mday      = 2DIGIT ; 01-28, 01-29, 01-30, 01-31 based on month/year
    // time-hour      = 2DIGIT ; 00-23
    // time-minute    = 2DIGIT ; 00-59
    // time-second    = 2DIGIT ; 00-58, 00-59, 00-60 based on leap second rules
    // time-secfrac   = "." 1*DIGIT
    // time-numoffset = ("+" / "-") time-hour ":" time-minute
    // time-offset    = "Z" / time-numoffset
    // partial-time   = time-hour ":" time-minute ":" time-second [time-secfrac]
    // full-date      = date-fullyear "-" date-month "-" date-mday
    // full-time      = partial-time time-offset
    // date-time      = full-date "T" full-time
    //
    // some notes:
    //
    // - quoted characters can be in any mixture of lower and upper cases.
    //
    // - it may accept any number of fractional digits for seconds.
    //   for Chrono, this means that we should skip digits past first 9 digits.
    //
    // - unlike RFC 2822, the valid offset ranges from -23:59 to +23:59.
    //   note that this restriction is unique to RFC 3339 and not ISO 8601.
    //   since this is not a typical Chrono behavior, we check it earlier.
    //
    // - For readability a full-date and a full-time may be separated by a space character.

    </span>parsed.set_year(<span class="macro">try_consume!</span>(scan::number(s, <span class="number">4</span>, <span class="number">4</span>)))<span class="question-mark">?</span>;
    s = scan::char(s, <span class="string">b'-'</span>)<span class="question-mark">?</span>;
    parsed.set_month(<span class="macro">try_consume!</span>(scan::number(s, <span class="number">2</span>, <span class="number">2</span>)))<span class="question-mark">?</span>;
    s = scan::char(s, <span class="string">b'-'</span>)<span class="question-mark">?</span>;
    parsed.set_day(<span class="macro">try_consume!</span>(scan::number(s, <span class="number">2</span>, <span class="number">2</span>)))<span class="question-mark">?</span>;

    s = <span class="kw">match </span>s.as_bytes().first() {
        <span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span><span class="string">b't' </span>| <span class="kw-2">&amp;</span><span class="string">b'T' </span>| <span class="kw-2">&amp;</span><span class="string">b' '</span>) =&gt; <span class="kw-2">&amp;</span>s[<span class="number">1</span>..],
        <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID),
        <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT),
    };

    parsed.set_hour(<span class="macro">try_consume!</span>(scan::number(s, <span class="number">2</span>, <span class="number">2</span>)))<span class="question-mark">?</span>;
    s = scan::char(s, <span class="string">b':'</span>)<span class="question-mark">?</span>;
    parsed.set_minute(<span class="macro">try_consume!</span>(scan::number(s, <span class="number">2</span>, <span class="number">2</span>)))<span class="question-mark">?</span>;
    s = scan::char(s, <span class="string">b':'</span>)<span class="question-mark">?</span>;
    parsed.set_second(<span class="macro">try_consume!</span>(scan::number(s, <span class="number">2</span>, <span class="number">2</span>)))<span class="question-mark">?</span>;
    <span class="kw">if </span>s.starts_with(<span class="string">'.'</span>) {
        <span class="kw">let </span>nanosecond = <span class="macro">try_consume!</span>(scan::nanosecond(<span class="kw-2">&amp;</span>s[<span class="number">1</span>..]));
        parsed.set_nanosecond(nanosecond)<span class="question-mark">?</span>;
    }

    <span class="kw">let </span>offset = <span class="macro">try_consume!</span>(scan::timezone_offset(s, |s| scan::char(s, <span class="string">b':'</span>), <span class="bool-val">true</span>, <span class="bool-val">false</span>, <span class="bool-val">true</span>));
    <span class="comment">// This range check is similar to the one in `FixedOffset::east_opt`, so it would be redundant.
    // But it is possible to read the offset directly from `Parsed`. We want to only successfully
    // populate `Parsed` if the input is fully valid RFC 3339.
    // Max for the hours field is `23`, and for the minutes field `59`.
    </span><span class="kw">const </span>MAX_RFC3339_OFFSET: i32 = (<span class="number">23 </span>* <span class="number">60 </span>+ <span class="number">59</span>) * <span class="number">60</span>;
    <span class="kw">if </span>!(-MAX_RFC3339_OFFSET..=MAX_RFC3339_OFFSET).contains(<span class="kw-2">&amp;</span>offset) {
        <span class="kw">return </span><span class="prelude-val">Err</span>(OUT_OF_RANGE);
    }
    parsed.set_offset(i64::from(offset))<span class="question-mark">?</span>;

    <span class="prelude-val">Ok</span>((s, ()))
}

<span class="doccomment">/// Tries to parse given string into `parsed` with given formatting items.
/// Returns `Ok` when the entire string has been parsed (otherwise `parsed` should not be used).
/// There should be no trailing string after parsing;
/// use a stray [`Item::Space`](./enum.Item.html#variant.Space) to trim whitespaces.
///
/// This particular date and time parser is:
///
/// - Greedy. It will consume the longest possible prefix.
///   For example, `April` is always consumed entirely when the long month name is requested;
///   it equally accepts `Apr`, but prefers the longer prefix in this case.
///
/// - Padding-agnostic (for numeric items).
///   The [`Pad`](./enum.Pad.html) field is completely ignored,
///   so one can prepend any number of whitespace then any number of zeroes before numbers.
///
/// - (Still) obeying the intrinsic parsing width. This allows, for example, parsing `HHMMSS`.
</span><span class="kw">pub fn </span>parse&lt;<span class="lifetime">'a</span>, I, B&gt;(parsed: <span class="kw-2">&amp;mut </span>Parsed, s: <span class="kw-2">&amp;</span>str, items: I) -&gt; ParseResult&lt;()&gt;
<span class="kw">where
    </span>I: Iterator&lt;Item = B&gt;,
    B: Borrow&lt;Item&lt;<span class="lifetime">'a</span>&gt;&gt;,
{
    <span class="kw">match </span>parse_internal(parsed, s, items) {
        <span class="prelude-val">Ok</span>(<span class="string">""</span>) =&gt; <span class="prelude-val">Ok</span>(()),
        <span class="prelude-val">Ok</span>(<span class="kw">_</span>) =&gt; <span class="prelude-val">Err</span>(TOO_LONG), <span class="comment">// if there are trailing chars it is an error
        </span><span class="prelude-val">Err</span>(e) =&gt; <span class="prelude-val">Err</span>(e),
    }
}

<span class="doccomment">/// Tries to parse given string into `parsed` with given formatting items.
/// Returns `Ok` with a slice of the unparsed remainder.
///
/// This particular date and time parser is:
///
/// - Greedy. It will consume the longest possible prefix.
///   For example, `April` is always consumed entirely when the long month name is requested;
///   it equally accepts `Apr`, but prefers the longer prefix in this case.
///
/// - Padding-agnostic (for numeric items).
///   The [`Pad`](./enum.Pad.html) field is completely ignored,
///   so one can prepend any number of zeroes before numbers.
///
/// - (Still) obeying the intrinsic parsing width. This allows, for example, parsing `HHMMSS`.
</span><span class="kw">pub fn </span>parse_and_remainder&lt;<span class="lifetime">'a</span>, <span class="lifetime">'b</span>, I, B&gt;(
    parsed: <span class="kw-2">&amp;mut </span>Parsed,
    s: <span class="kw-2">&amp;</span><span class="lifetime">'b </span>str,
    items: I,
) -&gt; ParseResult&lt;<span class="kw-2">&amp;</span><span class="lifetime">'b </span>str&gt;
<span class="kw">where
    </span>I: Iterator&lt;Item = B&gt;,
    B: Borrow&lt;Item&lt;<span class="lifetime">'a</span>&gt;&gt;,
{
    parse_internal(parsed, s, items)
}

<span class="kw">fn </span>parse_internal&lt;<span class="lifetime">'a</span>, <span class="lifetime">'b</span>, I, B&gt;(
    parsed: <span class="kw-2">&amp;mut </span>Parsed,
    <span class="kw-2">mut </span>s: <span class="kw-2">&amp;</span><span class="lifetime">'b </span>str,
    items: I,
) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'b </span>str, ParseError&gt;
<span class="kw">where
    </span>I: Iterator&lt;Item = B&gt;,
    B: Borrow&lt;Item&lt;<span class="lifetime">'a</span>&gt;&gt;,
{
    <span class="macro">macro_rules!</span> try_consume {
        (<span class="macro-nonterminal">$e</span>:expr) =&gt; {{
            <span class="kw">match </span><span class="macro-nonterminal">$e </span>{
                <span class="prelude-val">Ok</span>((s_, v)) =&gt; {
                    s = s_;
                    v
                }
                <span class="prelude-val">Err</span>(e) =&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(e),
            }
        }};
    }

    <span class="kw">for </span>item <span class="kw">in </span>items {
        <span class="kw">match </span><span class="kw-2">*</span>item.borrow() {
            Item::Literal(prefix) =&gt; {
                <span class="kw">if </span>s.len() &lt; prefix.len() {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT);
                }
                <span class="kw">if </span>!s.starts_with(prefix) {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID);
                }
                s = <span class="kw-2">&amp;</span>s[prefix.len()..];
            }

            <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
            </span>Item::OwnedLiteral(<span class="kw-2">ref </span>prefix) =&gt; {
                <span class="kw">if </span>s.len() &lt; prefix.len() {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT);
                }
                <span class="kw">if </span>!s.starts_with(<span class="kw-2">&amp;</span>prefix[..]) {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID);
                }
                s = <span class="kw-2">&amp;</span>s[prefix.len()..];
            }

            Item::Space(<span class="kw">_</span>) =&gt; {
                s = s.trim_start();
            }

            <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
            </span>Item::OwnedSpace(<span class="kw">_</span>) =&gt; {
                s = s.trim_start();
            }

            Item::Numeric(<span class="kw-2">ref </span>spec, <span class="kw-2">ref </span>_pad) =&gt; {
                <span class="kw">use </span><span class="kw">super</span>::Numeric::<span class="kw-2">*</span>;
                <span class="kw">type </span>Setter = <span class="kw">fn</span>(<span class="kw-2">&amp;mut </span>Parsed, i64) -&gt; ParseResult&lt;()&gt;;

                <span class="kw">let </span>(width, signed, set): (usize, bool, Setter) = <span class="kw">match </span><span class="kw-2">*</span>spec {
                    Year =&gt; (<span class="number">4</span>, <span class="bool-val">true</span>, Parsed::set_year),
                    YearDiv100 =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_year_div_100),
                    YearMod100 =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_year_mod_100),
                    IsoYear =&gt; (<span class="number">4</span>, <span class="bool-val">true</span>, Parsed::set_isoyear),
                    IsoYearDiv100 =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_isoyear_div_100),
                    IsoYearMod100 =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_isoyear_mod_100),
                    Month =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_month),
                    Day =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_day),
                    WeekFromSun =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_week_from_sun),
                    WeekFromMon =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_week_from_mon),
                    IsoWeek =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_isoweek),
                    NumDaysFromSun =&gt; (<span class="number">1</span>, <span class="bool-val">false</span>, set_weekday_with_num_days_from_sunday),
                    WeekdayFromMon =&gt; (<span class="number">1</span>, <span class="bool-val">false</span>, set_weekday_with_number_from_monday),
                    Ordinal =&gt; (<span class="number">3</span>, <span class="bool-val">false</span>, Parsed::set_ordinal),
                    Hour =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_hour),
                    Hour12 =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_hour12),
                    Minute =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_minute),
                    Second =&gt; (<span class="number">2</span>, <span class="bool-val">false</span>, Parsed::set_second),
                    Nanosecond =&gt; (<span class="number">9</span>, <span class="bool-val">false</span>, Parsed::set_nanosecond),
                    Timestamp =&gt; (usize::MAX, <span class="bool-val">false</span>, Parsed::set_timestamp),

                    <span class="comment">// for the future expansion
                    </span>Internal(<span class="kw-2">ref </span>int) =&gt; <span class="kw">match </span>int._dummy {},
                };

                s = s.trim_start();
                <span class="kw">let </span>v = <span class="kw">if </span>signed {
                    <span class="kw">if </span>s.starts_with(<span class="string">'-'</span>) {
                        <span class="kw">let </span>v = <span class="macro">try_consume!</span>(scan::number(<span class="kw-2">&amp;</span>s[<span class="number">1</span>..], <span class="number">1</span>, usize::MAX));
                        <span class="number">0i64</span>.checked_sub(v).ok_or(OUT_OF_RANGE)<span class="question-mark">?
                    </span>} <span class="kw">else if </span>s.starts_with(<span class="string">'+'</span>) {
                        <span class="macro">try_consume!</span>(scan::number(<span class="kw-2">&amp;</span>s[<span class="number">1</span>..], <span class="number">1</span>, usize::MAX))
                    } <span class="kw">else </span>{
                        <span class="comment">// if there is no explicit sign, we respect the original `width`
                        </span><span class="macro">try_consume!</span>(scan::number(s, <span class="number">1</span>, width))
                    }
                } <span class="kw">else </span>{
                    <span class="macro">try_consume!</span>(scan::number(s, <span class="number">1</span>, width))
                };
                set(parsed, v)<span class="question-mark">?</span>;
            }

            Item::Fixed(<span class="kw-2">ref </span>spec) =&gt; {
                <span class="kw">use </span><span class="kw">super</span>::Fixed::<span class="kw-2">*</span>;

                <span class="kw">match </span>spec {
                    <span class="kw-2">&amp;</span>ShortMonthName =&gt; {
                        <span class="kw">let </span>month0 = <span class="macro">try_consume!</span>(scan::short_month0(s));
                        parsed.set_month(i64::from(month0) + <span class="number">1</span>)<span class="question-mark">?</span>;
                    }

                    <span class="kw-2">&amp;</span>LongMonthName =&gt; {
                        <span class="kw">let </span>month0 = <span class="macro">try_consume!</span>(scan::short_or_long_month0(s));
                        parsed.set_month(i64::from(month0) + <span class="number">1</span>)<span class="question-mark">?</span>;
                    }

                    <span class="kw-2">&amp;</span>ShortWeekdayName =&gt; {
                        <span class="kw">let </span>weekday = <span class="macro">try_consume!</span>(scan::short_weekday(s));
                        parsed.set_weekday(weekday)<span class="question-mark">?</span>;
                    }

                    <span class="kw-2">&amp;</span>LongWeekdayName =&gt; {
                        <span class="kw">let </span>weekday = <span class="macro">try_consume!</span>(scan::short_or_long_weekday(s));
                        parsed.set_weekday(weekday)<span class="question-mark">?</span>;
                    }

                    <span class="kw-2">&amp;</span>LowerAmPm | <span class="kw-2">&amp;</span>UpperAmPm =&gt; {
                        <span class="kw">if </span>s.len() &lt; <span class="number">2 </span>{
                            <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT);
                        }
                        <span class="kw">let </span>ampm = <span class="kw">match </span>(s.as_bytes()[<span class="number">0</span>] | <span class="number">32</span>, s.as_bytes()[<span class="number">1</span>] | <span class="number">32</span>) {
                            (<span class="string">b'a'</span>, <span class="string">b'm'</span>) =&gt; <span class="bool-val">false</span>,
                            (<span class="string">b'p'</span>, <span class="string">b'm'</span>) =&gt; <span class="bool-val">true</span>,
                            <span class="kw">_ </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID),
                        };
                        parsed.set_ampm(ampm)<span class="question-mark">?</span>;
                        s = <span class="kw-2">&amp;</span>s[<span class="number">2</span>..];
                    }

                    <span class="kw-2">&amp;</span>Nanosecond | <span class="kw-2">&amp;</span>Nanosecond3 | <span class="kw-2">&amp;</span>Nanosecond6 | <span class="kw-2">&amp;</span>Nanosecond9 =&gt; {
                        <span class="kw">if </span>s.starts_with(<span class="string">'.'</span>) {
                            <span class="kw">let </span>nano = <span class="macro">try_consume!</span>(scan::nanosecond(<span class="kw-2">&amp;</span>s[<span class="number">1</span>..]));
                            parsed.set_nanosecond(nano)<span class="question-mark">?</span>;
                        }
                    }

                    <span class="kw-2">&amp;</span>Internal(InternalFixed { val: InternalInternal::Nanosecond3NoDot }) =&gt; {
                        <span class="kw">if </span>s.len() &lt; <span class="number">3 </span>{
                            <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT);
                        }
                        <span class="kw">let </span>nano = <span class="macro">try_consume!</span>(scan::nanosecond_fixed(s, <span class="number">3</span>));
                        parsed.set_nanosecond(nano)<span class="question-mark">?</span>;
                    }

                    <span class="kw-2">&amp;</span>Internal(InternalFixed { val: InternalInternal::Nanosecond6NoDot }) =&gt; {
                        <span class="kw">if </span>s.len() &lt; <span class="number">6 </span>{
                            <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT);
                        }
                        <span class="kw">let </span>nano = <span class="macro">try_consume!</span>(scan::nanosecond_fixed(s, <span class="number">6</span>));
                        parsed.set_nanosecond(nano)<span class="question-mark">?</span>;
                    }

                    <span class="kw-2">&amp;</span>Internal(InternalFixed { val: InternalInternal::Nanosecond9NoDot }) =&gt; {
                        <span class="kw">if </span>s.len() &lt; <span class="number">9 </span>{
                            <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT);
                        }
                        <span class="kw">let </span>nano = <span class="macro">try_consume!</span>(scan::nanosecond_fixed(s, <span class="number">9</span>));
                        parsed.set_nanosecond(nano)<span class="question-mark">?</span>;
                    }

                    <span class="kw-2">&amp;</span>TimezoneName =&gt; {
                        <span class="macro">try_consume!</span>(<span class="prelude-val">Ok</span>((s.trim_start_matches(|c: char| !c.is_whitespace()), ())));
                    }

                    <span class="kw-2">&amp;</span>TimezoneOffsetColon
                    | <span class="kw-2">&amp;</span>TimezoneOffsetDoubleColon
                    | <span class="kw-2">&amp;</span>TimezoneOffsetTripleColon
                    | <span class="kw-2">&amp;</span>TimezoneOffset =&gt; {
                        <span class="kw">let </span>offset = <span class="macro">try_consume!</span>(scan::timezone_offset(
                            s.trim_start(),
                            scan::colon_or_space,
                            <span class="bool-val">false</span>,
                            <span class="bool-val">false</span>,
                            <span class="bool-val">true</span>,
                        ));
                        parsed.set_offset(i64::from(offset))<span class="question-mark">?</span>;
                    }

                    <span class="kw-2">&amp;</span>TimezoneOffsetColonZ | <span class="kw-2">&amp;</span>TimezoneOffsetZ =&gt; {
                        <span class="kw">let </span>offset = <span class="macro">try_consume!</span>(scan::timezone_offset(
                            s.trim_start(),
                            scan::colon_or_space,
                            <span class="bool-val">true</span>,
                            <span class="bool-val">false</span>,
                            <span class="bool-val">true</span>,
                        ));
                        parsed.set_offset(i64::from(offset))<span class="question-mark">?</span>;
                    }
                    <span class="kw-2">&amp;</span>Internal(InternalFixed {
                        val: InternalInternal::TimezoneOffsetPermissive,
                    }) =&gt; {
                        <span class="kw">let </span>offset = <span class="macro">try_consume!</span>(scan::timezone_offset(
                            s.trim_start(),
                            scan::colon_or_space,
                            <span class="bool-val">true</span>,
                            <span class="bool-val">true</span>,
                            <span class="bool-val">true</span>,
                        ));
                        parsed.set_offset(i64::from(offset))<span class="question-mark">?</span>;
                    }

                    <span class="kw-2">&amp;</span>RFC2822 =&gt; <span class="macro">try_consume!</span>(parse_rfc2822(parsed, s)),
                    <span class="kw-2">&amp;</span>RFC3339 =&gt; {
                        <span class="comment">// Used for the `%+` specifier, which has the description:
                        // "Same as `%Y-%m-%dT%H:%M:%S%.f%:z` (...)
                        // This format also supports having a `Z` or `UTC` in place of `%:z`."
                        // Use the relaxed parser to match this description.
                        </span><span class="macro">try_consume!</span>(parse_rfc3339_relaxed(parsed, s))
                    }
                }
            }

            Item::Error =&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(BAD_FORMAT);
            }
        }
    }
    <span class="prelude-val">Ok</span>(s)
}

<span class="doccomment">/// Accepts a relaxed form of RFC3339.
/// A space or a 'T' are acepted as the separator between the date and time
/// parts. Additional spaces are allowed between each component.
///
/// All of these examples are equivalent:
/// ```
/// # use chrono::{DateTime, offset::FixedOffset};
/// "2012-12-12T12:12:12Z".parse::&lt;DateTime&lt;FixedOffset&gt;&gt;()?;
/// "2012-12-12 12:12:12Z".parse::&lt;DateTime&lt;FixedOffset&gt;&gt;()?;
/// "2012-  12-12T12:  12:12Z".parse::&lt;DateTime&lt;FixedOffset&gt;&gt;()?;
/// # Ok::&lt;(), chrono::ParseError&gt;(())
/// ```
</span><span class="kw">impl </span>str::FromStr <span class="kw">for </span>DateTime&lt;FixedOffset&gt; {
    <span class="kw">type </span><span class="prelude-val">Err </span>= ParseError;

    <span class="kw">fn </span>from_str(s: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;DateTime&lt;FixedOffset&gt;&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>parsed = Parsed::new();
        <span class="kw">let </span>(s, <span class="kw">_</span>) = parse_rfc3339_relaxed(<span class="kw-2">&amp;mut </span>parsed, s)<span class="question-mark">?</span>;
        <span class="kw">if </span>!s.trim_start().is_empty() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_LONG);
        }
        parsed.to_datetime()
    }
}

<span class="doccomment">/// Accepts a relaxed form of RFC3339.
///
/// Differences with RFC3339:
/// - Values don't require padding to two digits.
/// - Years outside the range 0...=9999 are accepted, but they must include a sign.
/// - `UTC` is accepted as a valid timezone name/offset (for compatibility with the debug format of
///   `DateTime&lt;Utc&gt;`.
/// - There can be spaces between any of the components.
/// - The colon in the offset may be missing.
</span><span class="kw">fn </span>parse_rfc3339_relaxed&lt;<span class="lifetime">'a</span>&gt;(parsed: <span class="kw-2">&amp;mut </span>Parsed, <span class="kw-2">mut </span>s: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str) -&gt; ParseResult&lt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str, ())&gt; {
    <span class="kw">const </span>DATE_ITEMS: <span class="kw-2">&amp;</span>[Item&lt;<span class="lifetime">'static</span>&gt;] = <span class="kw-2">&amp;</span>[
        Item::Numeric(Numeric::Year, Pad::Zero),
        Item::Space(<span class="string">""</span>),
        Item::Literal(<span class="string">"-"</span>),
        Item::Numeric(Numeric::Month, Pad::Zero),
        Item::Space(<span class="string">""</span>),
        Item::Literal(<span class="string">"-"</span>),
        Item::Numeric(Numeric::Day, Pad::Zero),
    ];
    <span class="kw">const </span>TIME_ITEMS: <span class="kw-2">&amp;</span>[Item&lt;<span class="lifetime">'static</span>&gt;] = <span class="kw-2">&amp;</span>[
        Item::Numeric(Numeric::Hour, Pad::Zero),
        Item::Space(<span class="string">""</span>),
        Item::Literal(<span class="string">":"</span>),
        Item::Numeric(Numeric::Minute, Pad::Zero),
        Item::Space(<span class="string">""</span>),
        Item::Literal(<span class="string">":"</span>),
        Item::Numeric(Numeric::Second, Pad::Zero),
        Item::Fixed(Fixed::Nanosecond),
        Item::Space(<span class="string">""</span>),
    ];

    s = parse_internal(parsed, s, DATE_ITEMS.iter())<span class="question-mark">?</span>;

    s = <span class="kw">match </span>s.as_bytes().first() {
        <span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span><span class="string">b't' </span>| <span class="kw-2">&amp;</span><span class="string">b'T' </span>| <span class="kw-2">&amp;</span><span class="string">b' '</span>) =&gt; <span class="kw-2">&amp;</span>s[<span class="number">1</span>..],
        <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(INVALID),
        <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(TOO_SHORT),
    };

    s = parse_internal(parsed, s, TIME_ITEMS.iter())<span class="question-mark">?</span>;
    s = s.trim_start();
    <span class="kw">let </span>(s, offset) = <span class="kw">if </span>s.len() &gt;= <span class="number">3 </span>&amp;&amp; <span class="string">"UTC"</span>.as_bytes().eq_ignore_ascii_case(<span class="kw-2">&amp;</span>s.as_bytes()[..<span class="number">3</span>]) {
        (<span class="kw-2">&amp;</span>s[<span class="number">3</span>..], <span class="number">0</span>)
    } <span class="kw">else </span>{
        scan::timezone_offset(s, scan::colon_or_space, <span class="bool-val">true</span>, <span class="bool-val">false</span>, <span class="bool-val">true</span>)<span class="question-mark">?
    </span>};
    parsed.set_offset(i64::from(offset))<span class="question-mark">?</span>;
    <span class="prelude-val">Ok</span>((s, ()))
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span><span class="kw">crate</span>::format::<span class="kw-2">*</span>;
    <span class="kw">use crate</span>::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Timelike, Utc};

    <span class="macro">macro_rules!</span> parsed {
        ($(<span class="macro-nonterminal">$k</span>:ident: <span class="macro-nonterminal">$v</span>:expr),<span class="kw-2">*</span>) =&gt; (<span class="attr">#[allow(unused_mut)] </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>expected = Parsed::new();
            $(expected.<span class="macro-nonterminal">$k </span>= <span class="prelude-val">Some</span>(<span class="macro-nonterminal">$v</span>);)*
            <span class="prelude-val">Ok</span>(expected)
        });
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_parse_whitespace_and_literal() {
        <span class="kw">use </span><span class="kw">crate</span>::format::Item::{Literal, Space};

        <span class="comment">// empty string
        </span>parses(<span class="string">""</span>, <span class="kw-2">&amp;</span>[]);
        check(<span class="string">" "</span>, <span class="kw-2">&amp;</span>[], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"a"</span>, <span class="kw-2">&amp;</span>[], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"abc"</span>, <span class="kw-2">&amp;</span>[], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"🤠"</span>, <span class="kw-2">&amp;</span>[], <span class="prelude-val">Err</span>(TOO_LONG));

        <span class="comment">// whitespaces
        </span>parses(<span class="string">""</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">""</span>)]);
        parses(<span class="string">" "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>)]);
        parses(<span class="string">"  "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>)]);
        parses(<span class="string">"   "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"   "</span>)]);
        parses(<span class="string">" "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">""</span>)]);
        parses(<span class="string">"  "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>)]);
        parses(<span class="string">"   "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>)]);
        parses(<span class="string">"    "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>)]);
        parses(<span class="string">""</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>)]);
        parses(<span class="string">" "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>)]);
        parses(<span class="string">"  "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"   "</span>)]);
        parses(<span class="string">"  "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>), Space(<span class="string">"  "</span>)]);
        parses(<span class="string">"   "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>), Space(<span class="string">"  "</span>)]);
        parses(<span class="string">"  "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>), Space(<span class="string">" "</span>)]);
        parses(<span class="string">"   "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>), Space(<span class="string">" "</span>)]);
        parses(<span class="string">"   "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>), Space(<span class="string">"  "</span>)]);
        parses(<span class="string">"   "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>), Space(<span class="string">" "</span>), Space(<span class="string">" "</span>)]);
        parses(<span class="string">"\t"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">""</span>)]);
        parses(<span class="string">" \n\r  \n"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">""</span>)]);
        parses(<span class="string">"\t"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"\t"</span>)]);
        parses(<span class="string">"\t"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>)]);
        parses(<span class="string">" "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"\t"</span>)]);
        parses(<span class="string">"\t\r"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"\t\r"</span>)]);
        parses(<span class="string">"\t\r "</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"\t\r "</span>)]);
        parses(<span class="string">"\t \r"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"\t \r"</span>)]);
        parses(<span class="string">" \t\r"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" \t\r"</span>)]);
        parses(<span class="string">" \n\r  \n"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" \n\r  \n"</span>)]);
        parses(<span class="string">" \t\n"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" \t"</span>)]);
        parses(<span class="string">" \n\t"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" \t\n"</span>)]);
        parses(<span class="string">"\u{2002}"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"\u{2002}"</span>)]);
        <span class="comment">// most unicode whitespace characters
        </span>parses(
            <span class="string">"\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{3000}"</span>,
            <span class="kw-2">&amp;</span>[Space(<span class="string">"\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{3000}"</span>)]
        );
        <span class="comment">// most unicode whitespace characters
        </span>parses(
            <span class="string">"\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{3000}"</span>,
            <span class="kw-2">&amp;</span>[
                Space(<span class="string">"\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}"</span>),
                Space(<span class="string">"\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{3000}"</span>)
            ]
        );
        check(<span class="string">"a"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">""</span>)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"a"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>)], <span class="prelude-val">Err</span>(TOO_LONG));
        <span class="comment">// a Space containing a literal does not match a literal
        </span>check(<span class="string">"a"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"a"</span>)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"abc"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">""</span>)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"abc"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">" abc"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">""</span>)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">" abc"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>)], <span class="prelude-val">Err</span>(TOO_LONG));

        <span class="comment">// `\u{0363}` is combining diacritic mark "COMBINING LATIN SMALL LETTER A"

        // literal
        </span>parses(<span class="string">""</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">""</span>)]);
        check(<span class="string">""</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a"</span>)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">" "</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a"</span>)], <span class="prelude-val">Err</span>(INVALID));
        parses(<span class="string">"a"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a"</span>)]);
        parses(<span class="string">"+"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"+"</span>)]);
        parses(<span class="string">"-"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"-"</span>)]);
        parses(<span class="string">"−"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"−"</span>)]); <span class="comment">// MINUS SIGN (U+2212)
        </span>parses(<span class="string">" "</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">" "</span>)]); <span class="comment">// a Literal may contain whitespace and match whitespace
        </span>check(<span class="string">"aa"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a"</span>)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"🤠"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a"</span>)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"A"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a"</span>)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"a"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"z"</span>)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"a"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"🤠"</span>)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"a"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"\u{0363}a"</span>)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"\u{0363}a"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a"</span>)], <span class="prelude-val">Err</span>(INVALID));
        parses(<span class="string">"\u{0363}a"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"\u{0363}a"</span>)]);
        check(<span class="string">"a"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"ab"</span>)], <span class="prelude-val">Err</span>(TOO_SHORT));
        parses(<span class="string">"xy"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"xy"</span>)]);
        parses(<span class="string">"xy"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"x"</span>), Literal(<span class="string">"y"</span>)]);
        parses(<span class="string">"1"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"1"</span>)]);
        parses(<span class="string">"1234"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"1234"</span>)]);
        parses(<span class="string">"+1234"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"+1234"</span>)]);
        parses(<span class="string">"-1234"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"-1234"</span>)]);
        parses(<span class="string">"−1234"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"−1234"</span>)]); <span class="comment">// MINUS SIGN (U+2212)
        </span>parses(<span class="string">"PST"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"PST"</span>)]);
        parses(<span class="string">"🤠"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"🤠"</span>)]);
        parses(<span class="string">"🤠a"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"🤠"</span>), Literal(<span class="string">"a"</span>)]);
        parses(<span class="string">"🤠a🤠"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"🤠"</span>), Literal(<span class="string">"a🤠"</span>)]);
        parses(<span class="string">"a🤠b"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a"</span>), Literal(<span class="string">"🤠"</span>), Literal(<span class="string">"b"</span>)]);
        <span class="comment">// literals can be together
        </span>parses(<span class="string">"xy"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"xy"</span>)]);
        parses(<span class="string">"xyz"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"xyz"</span>)]);
        <span class="comment">// or literals can be apart
        </span>parses(<span class="string">"xy"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"x"</span>), Literal(<span class="string">"y"</span>)]);
        parses(<span class="string">"xyz"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"x"</span>), Literal(<span class="string">"yz"</span>)]);
        parses(<span class="string">"xyz"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"xy"</span>), Literal(<span class="string">"z"</span>)]);
        parses(<span class="string">"xyz"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"x"</span>), Literal(<span class="string">"y"</span>), Literal(<span class="string">"z"</span>)]);
        <span class="comment">//
        </span>check(<span class="string">"x y"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"x"</span>), Literal(<span class="string">"y"</span>)], <span class="prelude-val">Err</span>(INVALID));
        parses(<span class="string">"xy"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"x"</span>), Space(<span class="string">""</span>), Literal(<span class="string">"y"</span>)]);
        parses(<span class="string">"x y"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"x"</span>), Space(<span class="string">""</span>), Literal(<span class="string">"y"</span>)]);
        parses(<span class="string">"x y"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"x"</span>), Space(<span class="string">" "</span>), Literal(<span class="string">"y"</span>)]);

        <span class="comment">// whitespaces + literals
        </span>parses(<span class="string">"a\n"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a"</span>), Space(<span class="string">"\n"</span>)]);
        parses(<span class="string">"\tab\n"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"\t"</span>), Literal(<span class="string">"ab"</span>), Space(<span class="string">"\n"</span>)]);
        parses(
            <span class="string">"ab\tcd\ne"</span>,
            <span class="kw-2">&amp;</span>[Literal(<span class="string">"ab"</span>), Space(<span class="string">"\t"</span>), Literal(<span class="string">"cd"</span>), Space(<span class="string">"\n"</span>), Literal(<span class="string">"e"</span>)],
        );
        parses(
            <span class="string">"+1ab\tcd\r\n+,."</span>,
            <span class="kw-2">&amp;</span>[Literal(<span class="string">"+1ab"</span>), Space(<span class="string">"\t"</span>), Literal(<span class="string">"cd"</span>), Space(<span class="string">"\r\n"</span>), Literal(<span class="string">"+,."</span>)],
        );
        <span class="comment">// whitespace and literals can be intermixed
        </span>parses(<span class="string">"a\tb"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a\tb"</span>)]);
        parses(<span class="string">"a\tb"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"a"</span>), Space(<span class="string">"\t"</span>), Literal(<span class="string">"b"</span>)]);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_parse_numeric() {
        <span class="kw">use </span><span class="kw">crate</span>::format::Item::{Literal, Space};
        <span class="kw">use </span><span class="kw">crate</span>::format::Numeric::<span class="kw-2">*</span>;

        <span class="comment">// numeric
        </span>check(<span class="string">"1987"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">1987</span>));
        check(<span class="string">"1987 "</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"0x12"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(TOO_LONG)); <span class="comment">// `0` is parsed
        </span>check(<span class="string">"x123"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"o123"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"2015"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">2015</span>));
        check(<span class="string">"0000"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">0</span>));
        check(<span class="string">"9999"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">9999</span>));
        check(<span class="string">" \t987"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">987</span>));
        check(<span class="string">" \t987"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" \t"</span>), num(Year)], <span class="macro">parsed!</span>(year: <span class="number">987</span>));
        check(<span class="string">" \t987🤠"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" \t"</span>), num(Year), Literal(<span class="string">"🤠"</span>)], <span class="macro">parsed!</span>(year: <span class="number">987</span>));
        check(<span class="string">"987🤠"</span>, <span class="kw-2">&amp;</span>[num(Year), Literal(<span class="string">"🤠"</span>)], <span class="macro">parsed!</span>(year: <span class="number">987</span>));
        check(<span class="string">"5"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">5</span>));
        check(<span class="string">"5\0"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"\x005"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">""</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"12345"</span>, <span class="kw-2">&amp;</span>[num(Year), Literal(<span class="string">"5"</span>)], <span class="macro">parsed!</span>(year: <span class="number">1234</span>));
        check(<span class="string">"12345"</span>, <span class="kw-2">&amp;</span>[nums(Year), Literal(<span class="string">"5"</span>)], <span class="macro">parsed!</span>(year: <span class="number">1234</span>));
        check(<span class="string">"12345"</span>, <span class="kw-2">&amp;</span>[num0(Year), Literal(<span class="string">"5"</span>)], <span class="macro">parsed!</span>(year: <span class="number">1234</span>));
        check(<span class="string">"12341234"</span>, <span class="kw-2">&amp;</span>[num(Year), num(Year)], <span class="macro">parsed!</span>(year: <span class="number">1234</span>));
        check(<span class="string">"1234 1234"</span>, <span class="kw-2">&amp;</span>[num(Year), num(Year)], <span class="macro">parsed!</span>(year: <span class="number">1234</span>));
        check(<span class="string">"1234 1234"</span>, <span class="kw-2">&amp;</span>[num(Year), Space(<span class="string">" "</span>), num(Year)], <span class="macro">parsed!</span>(year: <span class="number">1234</span>));
        check(<span class="string">"1234 1235"</span>, <span class="kw-2">&amp;</span>[num(Year), num(Year)], <span class="prelude-val">Err</span>(IMPOSSIBLE));
        check(<span class="string">"1234 1234"</span>, <span class="kw-2">&amp;</span>[num(Year), Literal(<span class="string">"x"</span>), num(Year)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234x1234"</span>, <span class="kw-2">&amp;</span>[num(Year), Literal(<span class="string">"x"</span>), num(Year)], <span class="macro">parsed!</span>(year: <span class="number">1234</span>));
        check(<span class="string">"1234 x 1234"</span>, <span class="kw-2">&amp;</span>[num(Year), Literal(<span class="string">"x"</span>), num(Year)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234xx1234"</span>, <span class="kw-2">&amp;</span>[num(Year), Literal(<span class="string">"x"</span>), num(Year)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234xx1234"</span>, <span class="kw-2">&amp;</span>[num(Year), Literal(<span class="string">"xx"</span>), num(Year)], <span class="macro">parsed!</span>(year: <span class="number">1234</span>));
        check(
            <span class="string">"1234 x 1234"</span>,
            <span class="kw-2">&amp;</span>[num(Year), Space(<span class="string">" "</span>), Literal(<span class="string">"x"</span>), Space(<span class="string">" "</span>), num(Year)],
            <span class="macro">parsed!</span>(year: <span class="number">1234</span>),
        );
        check(
            <span class="string">"1234 x 1235"</span>,
            <span class="kw-2">&amp;</span>[num(Year), Space(<span class="string">" "</span>), Literal(<span class="string">"x"</span>), Space(<span class="string">" "</span>), Literal(<span class="string">"1235"</span>)],
            <span class="macro">parsed!</span>(year: <span class="number">1234</span>),
        );

        <span class="comment">// signed numeric
        </span>check(<span class="string">"-42"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: -<span class="number">42</span>));
        check(<span class="string">"+42"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">42</span>));
        check(<span class="string">"-0042"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: -<span class="number">42</span>));
        check(<span class="string">"+0042"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">42</span>));
        check(<span class="string">"-42195"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: -<span class="number">42195</span>));
        check(<span class="string">"−42195"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(INVALID)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+42195"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">42195</span>));
        check(<span class="string">"  -42195"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: -<span class="number">42195</span>));
        check(<span class="string">" +42195"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">42195</span>));
        check(<span class="string">"  -42195"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: -<span class="number">42195</span>));
        check(<span class="string">"  +42195"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="macro">parsed!</span>(year: <span class="number">42195</span>));
        check(<span class="string">"-42195 "</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+42195 "</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"  -   42"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"  +   42"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"  -42195"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>), num(Year)], <span class="macro">parsed!</span>(year: -<span class="number">42195</span>));
        check(<span class="string">"  −42195"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>), num(Year)], <span class="prelude-val">Err</span>(INVALID)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"  +42195"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>), num(Year)], <span class="macro">parsed!</span>(year: <span class="number">42195</span>));
        check(<span class="string">"  -   42"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>), num(Year)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"  +   42"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"  "</span>), num(Year)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"-"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+"</span>, <span class="kw-2">&amp;</span>[num(Year)], <span class="prelude-val">Err</span>(TOO_SHORT));

        <span class="comment">// unsigned numeric
        </span>check(<span class="string">"345"</span>, <span class="kw-2">&amp;</span>[num(Ordinal)], <span class="macro">parsed!</span>(ordinal: <span class="number">345</span>));
        check(<span class="string">"+345"</span>, <span class="kw-2">&amp;</span>[num(Ordinal)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"-345"</span>, <span class="kw-2">&amp;</span>[num(Ordinal)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" 345"</span>, <span class="kw-2">&amp;</span>[num(Ordinal)], <span class="macro">parsed!</span>(ordinal: <span class="number">345</span>));
        check(<span class="string">"−345"</span>, <span class="kw-2">&amp;</span>[num(Ordinal)], <span class="prelude-val">Err</span>(INVALID)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"345 "</span>, <span class="kw-2">&amp;</span>[num(Ordinal)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">" 345"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>), num(Ordinal)], <span class="macro">parsed!</span>(ordinal: <span class="number">345</span>));
        check(<span class="string">"345 "</span>, <span class="kw-2">&amp;</span>[num(Ordinal), Space(<span class="string">" "</span>)], <span class="macro">parsed!</span>(ordinal: <span class="number">345</span>));
        check(<span class="string">"345🤠 "</span>, <span class="kw-2">&amp;</span>[num(Ordinal), Literal(<span class="string">"🤠"</span>), Space(<span class="string">" "</span>)], <span class="macro">parsed!</span>(ordinal: <span class="number">345</span>));
        check(<span class="string">"345🤠"</span>, <span class="kw-2">&amp;</span>[num(Ordinal)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"\u{0363}345"</span>, <span class="kw-2">&amp;</span>[num(Ordinal)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" +345"</span>, <span class="kw-2">&amp;</span>[num(Ordinal)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" -345"</span>, <span class="kw-2">&amp;</span>[num(Ordinal)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"\t345"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">"\t"</span>), num(Ordinal)], <span class="macro">parsed!</span>(ordinal: <span class="number">345</span>));
        check(<span class="string">" +345"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>), num(Ordinal)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" -345"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>), num(Ordinal)], <span class="prelude-val">Err</span>(INVALID));

        <span class="comment">// various numeric fields
        </span>check(<span class="string">"1234 5678"</span>, <span class="kw-2">&amp;</span>[num(Year), num(IsoYear)], <span class="macro">parsed!</span>(year: <span class="number">1234</span>, isoyear: <span class="number">5678</span>));
        check(<span class="string">"1234 5678"</span>, <span class="kw-2">&amp;</span>[num(Year), num(IsoYear)], <span class="macro">parsed!</span>(year: <span class="number">1234</span>, isoyear: <span class="number">5678</span>));
        check(
            <span class="string">"12 34 56 78"</span>,
            <span class="kw-2">&amp;</span>[num(YearDiv100), num(YearMod100), num(IsoYearDiv100), num(IsoYearMod100)],
            <span class="macro">parsed!</span>(year_div_100: <span class="number">12</span>, year_mod_100: <span class="number">34</span>, isoyear_div_100: <span class="number">56</span>, isoyear_mod_100: <span class="number">78</span>),
        );
        check(
            <span class="string">"1 2 3 4 5"</span>,
            <span class="kw-2">&amp;</span>[num(Month), num(Day), num(WeekFromSun), num(NumDaysFromSun), num(IsoWeek)],
            <span class="macro">parsed!</span>(month: <span class="number">1</span>, day: <span class="number">2</span>, week_from_sun: <span class="number">3</span>, weekday: Weekday::Thu, isoweek: <span class="number">5</span>),
        );
        check(
            <span class="string">"6 7 89 01"</span>,
            <span class="kw-2">&amp;</span>[num(WeekFromMon), num(WeekdayFromMon), num(Ordinal), num(Hour12)],
            <span class="macro">parsed!</span>(week_from_mon: <span class="number">6</span>, weekday: Weekday::Sun, ordinal: <span class="number">89</span>, hour_mod_12: <span class="number">1</span>),
        );
        check(
            <span class="string">"23 45 6 78901234 567890123"</span>,
            <span class="kw-2">&amp;</span>[num(Hour), num(Minute), num(Second), num(Nanosecond), num(Timestamp)],
            <span class="macro">parsed!</span>(hour_div_12: <span class="number">1</span>, hour_mod_12: <span class="number">11</span>, minute: <span class="number">45</span>, second: <span class="number">6</span>, nanosecond: <span class="number">78_901_234</span>, timestamp: <span class="number">567_890_123</span>),
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_parse_fixed() {
        <span class="kw">use </span><span class="kw">crate</span>::format::Fixed::<span class="kw-2">*</span>;
        <span class="kw">use </span><span class="kw">crate</span>::format::Item::{Literal, Space};

        <span class="comment">// fixed: month and weekday names
        </span>check(<span class="string">"apr"</span>, <span class="kw-2">&amp;</span>[fixed(ShortMonthName)], <span class="macro">parsed!</span>(month: <span class="number">4</span>));
        check(<span class="string">"Apr"</span>, <span class="kw-2">&amp;</span>[fixed(ShortMonthName)], <span class="macro">parsed!</span>(month: <span class="number">4</span>));
        check(<span class="string">"APR"</span>, <span class="kw-2">&amp;</span>[fixed(ShortMonthName)], <span class="macro">parsed!</span>(month: <span class="number">4</span>));
        check(<span class="string">"ApR"</span>, <span class="kw-2">&amp;</span>[fixed(ShortMonthName)], <span class="macro">parsed!</span>(month: <span class="number">4</span>));
        check(<span class="string">"\u{0363}APR"</span>, <span class="kw-2">&amp;</span>[fixed(ShortMonthName)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"April"</span>, <span class="kw-2">&amp;</span>[fixed(ShortMonthName)], <span class="prelude-val">Err</span>(TOO_LONG)); <span class="comment">// `Apr` is parsed
        </span>check(<span class="string">"A"</span>, <span class="kw-2">&amp;</span>[fixed(ShortMonthName)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"Sol"</span>, <span class="kw-2">&amp;</span>[fixed(ShortMonthName)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"Apr"</span>, <span class="kw-2">&amp;</span>[fixed(LongMonthName)], <span class="macro">parsed!</span>(month: <span class="number">4</span>));
        check(<span class="string">"Apri"</span>, <span class="kw-2">&amp;</span>[fixed(LongMonthName)], <span class="prelude-val">Err</span>(TOO_LONG)); <span class="comment">// `Apr` is parsed
        </span>check(<span class="string">"April"</span>, <span class="kw-2">&amp;</span>[fixed(LongMonthName)], <span class="macro">parsed!</span>(month: <span class="number">4</span>));
        check(<span class="string">"Aprill"</span>, <span class="kw-2">&amp;</span>[fixed(LongMonthName)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"Aprill"</span>, <span class="kw-2">&amp;</span>[fixed(LongMonthName), Literal(<span class="string">"l"</span>)], <span class="macro">parsed!</span>(month: <span class="number">4</span>));
        check(<span class="string">"Aprl"</span>, <span class="kw-2">&amp;</span>[fixed(LongMonthName), Literal(<span class="string">"l"</span>)], <span class="macro">parsed!</span>(month: <span class="number">4</span>));
        check(<span class="string">"April"</span>, <span class="kw-2">&amp;</span>[fixed(LongMonthName), Literal(<span class="string">"il"</span>)], <span class="prelude-val">Err</span>(TOO_SHORT)); <span class="comment">// do not backtrack
        </span>check(<span class="string">"thu"</span>, <span class="kw-2">&amp;</span>[fixed(ShortWeekdayName)], <span class="macro">parsed!</span>(weekday: Weekday::Thu));
        check(<span class="string">"Thu"</span>, <span class="kw-2">&amp;</span>[fixed(ShortWeekdayName)], <span class="macro">parsed!</span>(weekday: Weekday::Thu));
        check(<span class="string">"THU"</span>, <span class="kw-2">&amp;</span>[fixed(ShortWeekdayName)], <span class="macro">parsed!</span>(weekday: Weekday::Thu));
        check(<span class="string">"tHu"</span>, <span class="kw-2">&amp;</span>[fixed(ShortWeekdayName)], <span class="macro">parsed!</span>(weekday: Weekday::Thu));
        check(<span class="string">"Thursday"</span>, <span class="kw-2">&amp;</span>[fixed(ShortWeekdayName)], <span class="prelude-val">Err</span>(TOO_LONG)); <span class="comment">// `Thu` is parsed
        </span>check(<span class="string">"T"</span>, <span class="kw-2">&amp;</span>[fixed(ShortWeekdayName)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"The"</span>, <span class="kw-2">&amp;</span>[fixed(ShortWeekdayName)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"Nop"</span>, <span class="kw-2">&amp;</span>[fixed(ShortWeekdayName)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"Thu"</span>, <span class="kw-2">&amp;</span>[fixed(LongWeekdayName)], <span class="macro">parsed!</span>(weekday: Weekday::Thu));
        check(<span class="string">"Thur"</span>, <span class="kw-2">&amp;</span>[fixed(LongWeekdayName)], <span class="prelude-val">Err</span>(TOO_LONG)); <span class="comment">// `Thu` is parsed
        </span>check(<span class="string">"Thurs"</span>, <span class="kw-2">&amp;</span>[fixed(LongWeekdayName)], <span class="prelude-val">Err</span>(TOO_LONG)); <span class="comment">// `Thu` is parsed
        </span>check(<span class="string">"Thursday"</span>, <span class="kw-2">&amp;</span>[fixed(LongWeekdayName)], <span class="macro">parsed!</span>(weekday: Weekday::Thu));
        check(<span class="string">"Thursdays"</span>, <span class="kw-2">&amp;</span>[fixed(LongWeekdayName)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"Thursdays"</span>, <span class="kw-2">&amp;</span>[fixed(LongWeekdayName), Literal(<span class="string">"s"</span>)], <span class="macro">parsed!</span>(weekday: Weekday::Thu));
        check(<span class="string">"Thus"</span>, <span class="kw-2">&amp;</span>[fixed(LongWeekdayName), Literal(<span class="string">"s"</span>)], <span class="macro">parsed!</span>(weekday: Weekday::Thu));
        check(<span class="string">"Thursday"</span>, <span class="kw-2">&amp;</span>[fixed(LongWeekdayName), Literal(<span class="string">"rsday"</span>)], <span class="prelude-val">Err</span>(TOO_SHORT)); <span class="comment">// do not backtrack

        // fixed: am/pm
        </span>check(<span class="string">"am"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">0</span>));
        check(<span class="string">"pm"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">1</span>));
        check(<span class="string">"AM"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">0</span>));
        check(<span class="string">"PM"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">1</span>));
        check(<span class="string">"am"</span>, <span class="kw-2">&amp;</span>[fixed(UpperAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">0</span>));
        check(<span class="string">"pm"</span>, <span class="kw-2">&amp;</span>[fixed(UpperAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">1</span>));
        check(<span class="string">"AM"</span>, <span class="kw-2">&amp;</span>[fixed(UpperAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">0</span>));
        check(<span class="string">"PM"</span>, <span class="kw-2">&amp;</span>[fixed(UpperAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">1</span>));
        check(<span class="string">"Am"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">0</span>));
        check(<span class="string">" Am"</span>, <span class="kw-2">&amp;</span>[Space(<span class="string">" "</span>), fixed(LowerAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">0</span>));
        check(<span class="string">"Am🤠"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm), Literal(<span class="string">"🤠"</span>)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">0</span>));
        check(<span class="string">"🤠Am"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"🤠"</span>), fixed(LowerAmPm)], <span class="macro">parsed!</span>(hour_div_12: <span class="number">0</span>));
        check(<span class="string">"\u{0363}am"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"\u{0360}am"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" Am"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"Am "</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"a.m."</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"A.M."</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"ame"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(TOO_LONG)); <span class="comment">// `am` is parsed
        </span>check(<span class="string">"a"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"p"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"x"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"xx"</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">""</span>, <span class="kw-2">&amp;</span>[fixed(LowerAmPm)], <span class="prelude-val">Err</span>(TOO_SHORT));
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_parse_fixed_nanosecond() {
        <span class="kw">use </span><span class="kw">crate</span>::format::Fixed::Nanosecond;
        <span class="kw">use </span><span class="kw">crate</span>::format::InternalInternal::<span class="kw-2">*</span>;
        <span class="kw">use </span><span class="kw">crate</span>::format::Item::Literal;
        <span class="kw">use </span><span class="kw">crate</span>::format::Numeric::Second;

        <span class="comment">// fixed: dot plus nanoseconds
        </span>check(<span class="string">""</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>()); <span class="comment">// no field set, but not an error
        </span>check(<span class="string">"."</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"4"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="prelude-val">Err</span>(TOO_LONG)); <span class="comment">// never consumes `4`
        </span>check(<span class="string">"4"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond), num(Second)], <span class="macro">parsed!</span>(second: <span class="number">4</span>));
        check(<span class="string">".0"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">0</span>));
        check(<span class="string">".4"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">400_000_000</span>));
        check(<span class="string">".42"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">420_000_000</span>));
        check(<span class="string">".421"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_000_000</span>));
        check(<span class="string">".42195"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_950_000</span>));
        check(<span class="string">".421951"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_951_000</span>));
        check(<span class="string">".4219512"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_951_200</span>));
        check(<span class="string">".42195123"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_951_230</span>));
        check(<span class="string">".421950803"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_950_803</span>));
        check(<span class="string">".4219508035"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_950_803</span>));
        check(<span class="string">".42195080354"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_950_803</span>));
        check(<span class="string">".421950803547"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_950_803</span>));
        check(<span class="string">".000000003"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">3</span>));
        check(<span class="string">".0000000031"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">3</span>));
        check(<span class="string">".0000000035"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">3</span>));
        check(<span class="string">".000000003547"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">3</span>));
        check(<span class="string">".0000000009"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">0</span>));
        check(<span class="string">".000000000547"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">0</span>));
        check(<span class="string">".0000000009999999999999999999999999"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="macro">parsed!</span>(nanosecond: <span class="number">0</span>));
        check(<span class="string">".4🤠"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond), Literal(<span class="string">"🤠"</span>)], <span class="macro">parsed!</span>(nanosecond: <span class="number">400_000_000</span>));
        check(<span class="string">".4x"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">".  4"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"  .4"</span>, <span class="kw-2">&amp;</span>[fixed(Nanosecond)], <span class="prelude-val">Err</span>(TOO_LONG)); <span class="comment">// no automatic trimming

        // fixed: nanoseconds without the dot
        </span>check(<span class="string">""</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"."</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"0"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"4"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"42"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"421"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_000_000</span>));
        check(<span class="string">"4210"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(
            <span class="string">"42143"</span>,
            <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot), num(Second)],
            <span class="macro">parsed!</span>(nanosecond: <span class="number">421_000_000</span>, second: <span class="number">43</span>),
        );
        check(
            <span class="string">"421🤠"</span>,
            <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot), Literal(<span class="string">"🤠"</span>)],
            <span class="macro">parsed!</span>(nanosecond: <span class="number">421_000_000</span>),
        );
        check(
            <span class="string">"🤠421"</span>,
            <span class="kw-2">&amp;</span>[Literal(<span class="string">"🤠"</span>), internal_fixed(Nanosecond3NoDot)],
            <span class="macro">parsed!</span>(nanosecond: <span class="number">421_000_000</span>),
        );
        check(<span class="string">"42195"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"123456789"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"4x"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"  4"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">".421"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond3NoDot)], <span class="prelude-val">Err</span>(INVALID));

        check(<span class="string">""</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"."</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"0"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"1234"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"12345"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"421950"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_950_000</span>));
        check(<span class="string">"000003"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="macro">parsed!</span>(nanosecond: <span class="number">3000</span>));
        check(<span class="string">"000000"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="macro">parsed!</span>(nanosecond: <span class="number">0</span>));
        check(<span class="string">"1234567"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"123456789"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"4x"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"     4"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">".42100"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond6NoDot)], <span class="prelude-val">Err</span>(INVALID));

        check(<span class="string">""</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"."</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"42195"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"12345678"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"421950803"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="macro">parsed!</span>(nanosecond: <span class="number">421_950_803</span>));
        check(<span class="string">"000000003"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="macro">parsed!</span>(nanosecond: <span class="number">3</span>));
        check(
            <span class="string">"42195080354"</span>,
            <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot), num(Second)],
            <span class="macro">parsed!</span>(nanosecond: <span class="number">421_950_803</span>, second: <span class="number">54</span>),
        ); <span class="comment">// don't skip digits that come after the 9
        </span>check(<span class="string">"1234567890"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"000000000"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="macro">parsed!</span>(nanosecond: <span class="number">0</span>));
        check(<span class="string">"00000000x"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"        4"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">".42100000"</span>, <span class="kw-2">&amp;</span>[internal_fixed(Nanosecond9NoDot)], <span class="prelude-val">Err</span>(INVALID));
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_parse_fixed_timezone_offset() {
        <span class="kw">use </span><span class="kw">crate</span>::format::Fixed::<span class="kw-2">*</span>;
        <span class="kw">use </span><span class="kw">crate</span>::format::InternalInternal::<span class="kw-2">*</span>;
        <span class="kw">use </span><span class="kw">crate</span>::format::Item::Literal;

        <span class="comment">// TimezoneOffset
        </span>check(<span class="string">"1"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"123"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12345"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"123456"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234567"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+1"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+123"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12345"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+123456"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+1234567"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12345678"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12:3"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"-12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"−12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+12:34:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:5"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12  34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34:56"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+12::34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12: :34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12:::34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12::::34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12::34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12:34:56"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:3456"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+1234:56"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+1234:567"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+00:00"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">"-00:00"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">"−00:00"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+00:01"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">60</span>));
        check(<span class="string">"-00:01"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">60</span>));
        check(<span class="string">"+00:30"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">1_800</span>));
        check(<span class="string">"-00:30"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">1_800</span>));
        check(<span class="string">"+24:00"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">86_400</span>));
        check(<span class="string">"-24:00"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">86_400</span>));
        check(<span class="string">"−24:00"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">86_400</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+99:59"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">359_940</span>));
        check(<span class="string">"-99:59"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">359_940</span>));
        check(<span class="string">"+00:60"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(OUT_OF_RANGE));
        check(<span class="string">"+00:99"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(OUT_OF_RANGE));
        check(<span class="string">"#12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+12:34 "</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12 34 "</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">" +12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">" -12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">" −12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"  +12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"  -12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"\t -12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"-12: 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"-12 :34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"-12 : 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"-12 :  34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"-12  : 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"-12:  34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"-12  :34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"-12  :  34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"12:34 "</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" 12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">""</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(
            <span class="string">"+12345"</span>,
            <span class="kw-2">&amp;</span>[fixed(TimezoneOffset), num(Numeric::Day)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>, day: <span class="number">5</span>),
        );
        check(
            <span class="string">"+12:345"</span>,
            <span class="kw-2">&amp;</span>[fixed(TimezoneOffset), num(Numeric::Day)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>, day: <span class="number">5</span>),
        );
        check(<span class="string">"+12:34:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset), Literal(<span class="string">":"</span>)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"Z12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"X12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"Z+12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"X+12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"X−12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"🤠+12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+12:34🤠"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:🤠34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+1234🤠"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset), Literal(<span class="string">"🤠"</span>)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"-1234🤠"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset), Literal(<span class="string">"🤠"</span>)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"−1234🤠"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset), Literal(<span class="string">"🤠"</span>)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+12:34🤠"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset), Literal(<span class="string">"🤠"</span>)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"-12:34🤠"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset), Literal(<span class="string">"🤠"</span>)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"−12:34🤠"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset), Literal(<span class="string">"🤠"</span>)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"🤠+12:34"</span>, <span class="kw-2">&amp;</span>[Literal(<span class="string">"🤠"</span>), fixed(TimezoneOffset)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"A"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"PST"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"#Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">":Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+:Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+Z:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" :Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffset)], <span class="prelude-val">Err</span>(INVALID));

        <span class="comment">// TimezoneOffsetColon
        </span>check(<span class="string">"1"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"123"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12345"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"123456"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234567"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12345678"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+1"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+123"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"-1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"−1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+12345"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+123456"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+1234567"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12345678"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"1:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:3"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34:5"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34:56"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+1:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+12:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12:3"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"-12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"−12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+12:34:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:5"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56:7"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56:78"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:3456"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+1234:56"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"−12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"−12 : 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+12 :34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12: 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12: 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12 :34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12 : 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"-12 : 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"+12  : 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12 :  34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12  :  34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12::34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12: :34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12:::34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12::::34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12::34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"#1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"#12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+12:34 "</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">" +12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"\t+12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"\t\t+12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"12:34 "</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" 12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">""</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">":"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(
            <span class="string">"+12345"</span>,
            <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon), num(Numeric::Day)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>, day: <span class="number">5</span>),
        );
        check(
            <span class="string">"+12:345"</span>,
            <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon), num(Numeric::Day)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>, day: <span class="number">5</span>),
        );
        check(<span class="string">"+12:34:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon), Literal(<span class="string">":"</span>)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"A"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"PST"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"#Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">":Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+:Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+Z:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" :Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetColon)], <span class="prelude-val">Err</span>(INVALID));
        <span class="comment">// testing `TimezoneOffsetColon` also tests same path as `TimezoneOffsetDoubleColon`
        // and `TimezoneOffsetTripleColon` for function `parse_internal`.
        // No need for separate tests for `TimezoneOffsetDoubleColon` and
        // `TimezoneOffsetTripleColon`.

        // TimezoneOffsetZ
        </span>check(<span class="string">"1"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"123"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12345"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"123456"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234567"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12345678"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+1"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+123"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"-1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"−1234"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+12345"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+123456"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+1234567"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12345678"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"1:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:3"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34:5"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34:56"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+1:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+12:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12:3"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"-12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"−12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+12:34:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:5"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56:7"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56:78"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12::34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12:3456"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+1234:56"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12  34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12: 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12 :34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12 : 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12  : 34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12 :  34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12  :  34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"12:34 "</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" 12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+12:34 "</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12 34 "</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">" +12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(
            <span class="string">"+12345"</span>,
            <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ), num(Numeric::Day)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>, day: <span class="number">5</span>),
        );
        check(
            <span class="string">"+12:345"</span>,
            <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ), num(Numeric::Day)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>, day: <span class="number">5</span>),
        );
        check(<span class="string">"+12:34:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ), Literal(<span class="string">":"</span>)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"Z12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"X12:34"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">"z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">" Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">" z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">"\u{0363}Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"Z "</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"A"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"PST"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"#Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">":Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">":z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"-Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+A"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+🙃"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+Z:"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" :Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" +Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">" -Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+:Z"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"Y"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"Zulu"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ), Literal(<span class="string">"ulu"</span>)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">"zulu"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ), Literal(<span class="string">"ulu"</span>)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">"+1234ulu"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ), Literal(<span class="string">"ulu"</span>)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12:34ulu"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneOffsetZ), Literal(<span class="string">"ulu"</span>)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        <span class="comment">// Testing `TimezoneOffsetZ` also tests same path as `TimezoneOffsetColonZ`
        // in function `parse_internal`.
        // No need for separate tests for `TimezoneOffsetColonZ`.

        // TimezoneOffsetPermissive
        </span>check(<span class="string">"1"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"123"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12345"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"123456"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"1234567"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12345678"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+1"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">43_200</span>));
        check(<span class="string">"+123"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+1234"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"-1234"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"−1234"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+12345"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+123456"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+1234567"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12345678"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"1:"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:3"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34:"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34:5"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"12:34:56"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+1:"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+12:"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">43_200</span>));
        check(<span class="string">"+12:3"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+12:34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"-12:34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">"−12:34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(<span class="string">"+12:34:"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:5"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56:"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56:7"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:34:56:78"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12 34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12  34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12 :34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12: 34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12 : 34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12  :34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12:  34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12  :  34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12::34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12 ::34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12: :34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12:: 34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12  ::34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12:  :34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12::  34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12:::34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"+12::::34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">"12:34 "</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" 12:34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+12:34 "</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">" +12:34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>));
        check(<span class="string">" -12:34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>));
        check(<span class="string">" −12:34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: -<span class="number">45_240</span>)); <span class="comment">// MINUS SIGN (U+2212)
        </span>check(
            <span class="string">"+12345"</span>,
            <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive), num(Numeric::Day)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>, day: <span class="number">5</span>),
        );
        check(
            <span class="string">"+12:345"</span>,
            <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive), num(Numeric::Day)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>, day: <span class="number">5</span>),
        );
        check(
            <span class="string">"+12:34:"</span>,
            <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive), Literal(<span class="string">":"</span>)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>),
        );
        check(<span class="string">"🤠+12:34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+12:34🤠"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"+12:🤠34"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(
            <span class="string">"+12:34🤠"</span>,
            <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive), Literal(<span class="string">"🤠"</span>)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>),
        );
        check(
            <span class="string">"🤠+12:34"</span>,
            <span class="kw-2">&amp;</span>[Literal(<span class="string">"🤠"</span>), internal_fixed(TimezoneOffsetPermissive)],
            <span class="macro">parsed!</span>(offset: <span class="number">45_240</span>),
        );
        check(<span class="string">"Z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">"A"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"PST"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">" Z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">" z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="macro">parsed!</span>(offset: <span class="number">0</span>));
        check(<span class="string">"Z "</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"#Z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">":Z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">":z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+Z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"-Z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+A"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+PST"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+🙃"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"+Z:"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" :Z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">" +Z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">" -Z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(TOO_SHORT));
        check(<span class="string">"+:Z"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));
        check(<span class="string">"Y"</span>, <span class="kw-2">&amp;</span>[internal_fixed(TimezoneOffsetPermissive)], <span class="prelude-val">Err</span>(INVALID));

        <span class="comment">// TimezoneName
        </span>check(<span class="string">"CEST"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneName)], <span class="macro">parsed!</span>());
        check(<span class="string">"cest"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneName)], <span class="macro">parsed!</span>()); <span class="comment">// lowercase
        </span>check(<span class="string">"XXXXXXXX"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneName)], <span class="macro">parsed!</span>()); <span class="comment">// not a real timezone name
        </span>check(<span class="string">"!!!!"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneName)], <span class="macro">parsed!</span>()); <span class="comment">// not a real timezone name!
        </span>check(<span class="string">"CEST 5"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneName), Literal(<span class="string">" "</span>), num(Numeric::Day)], <span class="macro">parsed!</span>(day: <span class="number">5</span>));
        check(<span class="string">"CEST "</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneName)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">" CEST"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneName)], <span class="prelude-val">Err</span>(TOO_LONG));
        check(<span class="string">"CE ST"</span>, <span class="kw-2">&amp;</span>[fixed(TimezoneName)], <span class="prelude-val">Err</span>(TOO_LONG));
    }

    <span class="attr">#[test]
    #[rustfmt::skip]
    </span><span class="kw">fn </span>test_parse_practical_examples() {
        <span class="kw">use </span><span class="kw">crate</span>::format::InternalInternal::<span class="kw-2">*</span>;
        <span class="kw">use </span><span class="kw">crate</span>::format::Item::{Literal, Space};
        <span class="kw">use </span><span class="kw">crate</span>::format::Numeric::<span class="kw-2">*</span>;

        <span class="comment">// some practical examples
        </span>check(
            <span class="string">"2015-02-04T14:37:05+09:00"</span>,
            <span class="kw-2">&amp;</span>[
                num(Year), Literal(<span class="string">"-"</span>), num(Month), Literal(<span class="string">"-"</span>), num(Day), Literal(<span class="string">"T"</span>),
                num(Hour), Literal(<span class="string">":"</span>), num(Minute), Literal(<span class="string">":"</span>), num(Second),
                fixed(Fixed::TimezoneOffset),
            ],
            <span class="macro">parsed!</span>(
                year: <span class="number">2015</span>, month: <span class="number">2</span>, day: <span class="number">4</span>, hour_div_12: <span class="number">1</span>, hour_mod_12: <span class="number">2</span>, minute: <span class="number">37</span>,
                second: <span class="number">5</span>, offset: <span class="number">32400
            </span>),
        );
        check(
            <span class="string">"2015-02-04T14:37:05-09:00"</span>,
            <span class="kw-2">&amp;</span>[
                num(Year), Literal(<span class="string">"-"</span>), num(Month), Literal(<span class="string">"-"</span>), num(Day), Literal(<span class="string">"T"</span>),
                num(Hour), Literal(<span class="string">":"</span>), num(Minute), Literal(<span class="string">":"</span>), num(Second),
                fixed(Fixed::TimezoneOffset),
            ],
            <span class="macro">parsed!</span>(
                year: <span class="number">2015</span>, month: <span class="number">2</span>, day: <span class="number">4</span>, hour_div_12: <span class="number">1</span>, hour_mod_12: <span class="number">2</span>, minute: <span class="number">37</span>,
                second: <span class="number">5</span>, offset: -<span class="number">32400
            </span>),
        );
        check(
            <span class="string">"2015-02-04T14:37:05−09:00"</span>, <span class="comment">// timezone offset using MINUS SIGN (U+2212)
            </span><span class="kw-2">&amp;</span>[
                num(Year), Literal(<span class="string">"-"</span>), num(Month), Literal(<span class="string">"-"</span>), num(Day), Literal(<span class="string">"T"</span>),
                num(Hour), Literal(<span class="string">":"</span>), num(Minute), Literal(<span class="string">":"</span>), num(Second),
                fixed(Fixed::TimezoneOffset)
            ],
            <span class="macro">parsed!</span>(
                year: <span class="number">2015</span>, month: <span class="number">2</span>, day: <span class="number">4</span>, hour_div_12: <span class="number">1</span>, hour_mod_12: <span class="number">2</span>, minute: <span class="number">37</span>,
                second: <span class="number">5</span>, offset: -<span class="number">32400
            </span>),
        );
        check(
            <span class="string">"20150204143705567"</span>,
            <span class="kw-2">&amp;</span>[
                num(Year), num(Month), num(Day), num(Hour), num(Minute), num(Second),
                internal_fixed(Nanosecond3NoDot)
            ],
            <span class="macro">parsed!</span>(
                year: <span class="number">2015</span>, month: <span class="number">2</span>, day: <span class="number">4</span>, hour_div_12: <span class="number">1</span>, hour_mod_12: <span class="number">2</span>, minute: <span class="number">37</span>,
                second: <span class="number">5</span>, nanosecond: <span class="number">567000000
            </span>),
        );
        check(
            <span class="string">"Mon, 10 Jun 2013 09:32:37 GMT"</span>,
            <span class="kw-2">&amp;</span>[
                fixed(Fixed::ShortWeekdayName), Literal(<span class="string">","</span>), Space(<span class="string">" "</span>), num(Day), Space(<span class="string">" "</span>),
                fixed(Fixed::ShortMonthName), Space(<span class="string">" "</span>), num(Year), Space(<span class="string">" "</span>), num(Hour),
                Literal(<span class="string">":"</span>), num(Minute), Literal(<span class="string">":"</span>), num(Second), Space(<span class="string">" "</span>), Literal(<span class="string">"GMT"</span>)
            ],
            <span class="macro">parsed!</span>(
                year: <span class="number">2013</span>, month: <span class="number">6</span>, day: <span class="number">10</span>, weekday: Weekday::Mon,
                hour_div_12: <span class="number">0</span>, hour_mod_12: <span class="number">9</span>, minute: <span class="number">32</span>, second: <span class="number">37
            </span>),
        );
        check(
            <span class="string">"🤠Mon, 10 Jun🤠2013 09:32:37  GMT🤠"</span>,
            <span class="kw-2">&amp;</span>[
                Literal(<span class="string">"🤠"</span>), fixed(Fixed::ShortWeekdayName), Literal(<span class="string">","</span>), Space(<span class="string">" "</span>), num(Day),
                Space(<span class="string">" "</span>), fixed(Fixed::ShortMonthName), Literal(<span class="string">"🤠"</span>), num(Year), Space(<span class="string">" "</span>),
                num(Hour), Literal(<span class="string">":"</span>), num(Minute), Literal(<span class="string">":"</span>), num(Second), Space(<span class="string">"  "</span>),
                Literal(<span class="string">"GMT"</span>), Literal(<span class="string">"🤠"</span>)
            ],
            <span class="macro">parsed!</span>(
                year: <span class="number">2013</span>, month: <span class="number">6</span>, day: <span class="number">10</span>, weekday: Weekday::Mon,
                hour_div_12: <span class="number">0</span>, hour_mod_12: <span class="number">9</span>, minute: <span class="number">32</span>, second: <span class="number">37
            </span>),
        );
        check(
            <span class="string">"Sun Aug 02 13:39:15 CEST 2020"</span>,
            <span class="kw-2">&amp;</span>[
                fixed(Fixed::ShortWeekdayName), Space(<span class="string">" "</span>), fixed(Fixed::ShortMonthName),
                Space(<span class="string">" "</span>), num(Day), Space(<span class="string">" "</span>), num(Hour), Literal(<span class="string">":"</span>), num(Minute),
                Literal(<span class="string">":"</span>), num(Second), Space(<span class="string">" "</span>), fixed(Fixed::TimezoneName), Space(<span class="string">" "</span>),
                num(Year)
            ],
            <span class="macro">parsed!</span>(
                year: <span class="number">2020</span>, month: <span class="number">8</span>, day: <span class="number">2</span>, weekday: Weekday::Sun,
                hour_div_12: <span class="number">1</span>, hour_mod_12: <span class="number">1</span>, minute: <span class="number">39</span>, second: <span class="number">15
            </span>),
        );
        check(
            <span class="string">"20060102150405"</span>,
            <span class="kw-2">&amp;</span>[num(Year), num(Month), num(Day), num(Hour), num(Minute), num(Second)],
            <span class="macro">parsed!</span>(
                year: <span class="number">2006</span>, month: <span class="number">1</span>, day: <span class="number">2</span>, hour_div_12: <span class="number">1</span>, hour_mod_12: <span class="number">3</span>, minute: <span class="number">4</span>, second: <span class="number">5
            </span>),
        );
        check(
            <span class="string">"3:14PM"</span>,
            <span class="kw-2">&amp;</span>[num(Hour12), Literal(<span class="string">":"</span>), num(Minute), fixed(Fixed::LowerAmPm)],
            <span class="macro">parsed!</span>(hour_div_12: <span class="number">1</span>, hour_mod_12: <span class="number">3</span>, minute: <span class="number">14</span>),
        );
        check(
            <span class="string">"12345678901234.56789"</span>,
            <span class="kw-2">&amp;</span>[num(Timestamp), Literal(<span class="string">"."</span>), num(Nanosecond)],
            <span class="macro">parsed!</span>(nanosecond: <span class="number">56_789</span>, timestamp: <span class="number">12_345_678_901_234</span>),
        );
        check(
            <span class="string">"12345678901234.56789"</span>,
            <span class="kw-2">&amp;</span>[num(Timestamp), fixed(Fixed::Nanosecond)],
            <span class="macro">parsed!</span>(nanosecond: <span class="number">567_890_000</span>, timestamp: <span class="number">12_345_678_901_234</span>),
        );

        <span class="comment">// docstring examples from `impl str::FromStr`
        </span>check(
            <span class="string">"2000-01-02T03:04:05Z"</span>,
            <span class="kw-2">&amp;</span>[
                num(Year), Literal(<span class="string">"-"</span>), num(Month), Literal(<span class="string">"-"</span>), num(Day), Literal(<span class="string">"T"</span>),
                num(Hour), Literal(<span class="string">":"</span>), num(Minute), Literal(<span class="string">":"</span>), num(Second),
                internal_fixed(TimezoneOffsetPermissive)
            ],
            <span class="macro">parsed!</span>(
                year: <span class="number">2000</span>, month: <span class="number">1</span>, day: <span class="number">2</span>, hour_div_12: <span class="number">0</span>, hour_mod_12: <span class="number">3</span>, minute: <span class="number">4</span>, second: <span class="number">5</span>,
                offset: <span class="number">0
            </span>),
        );
        check(
            <span class="string">"2000-01-02 03:04:05Z"</span>,
            <span class="kw-2">&amp;</span>[
                num(Year), Literal(<span class="string">"-"</span>), num(Month), Literal(<span class="string">"-"</span>), num(Day), Space(<span class="string">" "</span>),
                num(Hour), Literal(<span class="string">":"</span>), num(Minute), Literal(<span class="string">":"</span>), num(Second),
                internal_fixed(TimezoneOffsetPermissive)
            ],
            <span class="macro">parsed!</span>(
                year: <span class="number">2000</span>, month: <span class="number">1</span>, day: <span class="number">2</span>, hour_div_12: <span class="number">0</span>, hour_mod_12: <span class="number">3</span>, minute: <span class="number">4</span>, second: <span class="number">5</span>,
                offset: <span class="number">0
            </span>),
        );
    }

    <span class="attr">#[track_caller]
    </span><span class="kw">fn </span>parses(s: <span class="kw-2">&amp;</span>str, items: <span class="kw-2">&amp;</span>[Item]) {
        <span class="kw">let </span><span class="kw-2">mut </span>parsed = Parsed::new();
        <span class="macro">assert!</span>(parse(<span class="kw-2">&amp;mut </span>parsed, s, items.iter()).is_ok());
    }

    <span class="attr">#[track_caller]
    </span><span class="kw">fn </span>check(s: <span class="kw-2">&amp;</span>str, items: <span class="kw-2">&amp;</span>[Item], expected: ParseResult&lt;Parsed&gt;) {
        <span class="kw">let </span><span class="kw-2">mut </span>parsed = Parsed::new();
        <span class="kw">let </span>result = parse(<span class="kw-2">&amp;mut </span>parsed, s, items.iter());
        <span class="kw">let </span>parsed = result.map(|<span class="kw">_</span>| parsed);
        <span class="macro">assert_eq!</span>(parsed, expected);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_rfc2822() {
        <span class="kw">let </span>ymd_hmsn = |y, m, d, h, n, s, nano, off| {
            FixedOffset::east_opt(off * <span class="number">60 </span>* <span class="number">60</span>)
                .unwrap()
                .with_ymd_and_hms(y, m, d, h, n, s)
                .unwrap()
                .with_nanosecond(nano)
                .unwrap()
        };

        <span class="comment">// Test data - (input, Ok(expected result) or Err(error code))
        </span><span class="kw">let </span>testdates = [
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 -0800"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))), <span class="comment">// normal case
            </span>(<span class="string">"Fri,  2 Jan 2015 17:35:20 -0800"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">2</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))), <span class="comment">// folding whitespace
            </span>(<span class="string">"Fri, 02 Jan 2015 17:35:20 -0800"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">2</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))), <span class="comment">// leading zero
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 -0800 (UTC)"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))), <span class="comment">// trailing comment
            </span>(
                <span class="string">r"Tue, 20 Jan 2015 17:35:20 -0800 ( (UTC ) (\( (a)\(( \t ) ) \\( \) ))"</span>,
                <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>)),
            ), <span class="comment">// complex trailing comment
            </span>(<span class="string">r"Tue, 20 Jan 2015 17:35:20 -0800 (UTC\)"</span>, <span class="prelude-val">Err</span>(TOO_LONG)), <span class="comment">// incorrect comment, not enough closing parentheses
            </span>(
                <span class="string">"Tue, 20 Jan 2015 17:35:20 -0800 (UTC)\t \r\n(Anothercomment)"</span>,
                <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>)),
            ), <span class="comment">// multiple comments
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 -0800 (UTC) "</span>, <span class="prelude-val">Err</span>(TOO_LONG)), <span class="comment">// trailing whitespace after comment
            </span>(<span class="string">"20 Jan 2015 17:35:20 -0800"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))), <span class="comment">// no day of week
            </span>(<span class="string">"20 JAN 2015 17:35:20 -0800"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))), <span class="comment">// upper case month
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35 -0800"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">0</span>, <span class="number">0</span>, -<span class="number">8</span>))), <span class="comment">// no second
            </span>(<span class="string">"11 Sep 2001 09:45:00 +0000"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2001</span>, <span class="number">9</span>, <span class="number">11</span>, <span class="number">9</span>, <span class="number">45</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>))),
            (<span class="string">"11 Sep 2001 09:45:00 EST"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2001</span>, <span class="number">9</span>, <span class="number">11</span>, <span class="number">9</span>, <span class="number">45</span>, <span class="number">0</span>, <span class="number">0</span>, -<span class="number">5</span>))),
            (<span class="string">"11 Sep 2001 09:45:00 GMT"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2001</span>, <span class="number">9</span>, <span class="number">11</span>, <span class="number">9</span>, <span class="number">45</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>))),
            (<span class="string">"30 Feb 2015 17:35:20 -0800"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad day of month
            </span>(<span class="string">"Tue, 20 Jan 2015"</span>, <span class="prelude-val">Err</span>(TOO_SHORT)),              <span class="comment">// omitted fields
            </span>(<span class="string">"Tue, 20 Avr 2015 17:35:20 -0800"</span>, <span class="prelude-val">Err</span>(INVALID)), <span class="comment">// bad month name
            </span>(<span class="string">"Tue, 20 Jan 2015 25:35:20 -0800"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad hour
            </span>(<span class="string">"Tue, 20 Jan 2015 7:35:20 -0800"</span>, <span class="prelude-val">Err</span>(INVALID)),  <span class="comment">// bad # of digits in hour
            </span>(<span class="string">"Tue, 20 Jan 2015 17:65:20 -0800"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad minute
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:90 -0800"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad second
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 -0890"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad offset
            </span>(<span class="string">"6 Jun 1944 04:00:00Z"</span>, <span class="prelude-val">Err</span>(INVALID)),            <span class="comment">// bad offset (zulu not allowed)
            // named timezones that have specific timezone offsets
            // see https://www.rfc-editor.org/rfc/rfc2822#section-4.3
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 GMT"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, <span class="number">0</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 UT"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, <span class="number">0</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 ut"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, <span class="number">0</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 EDT"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">4</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 EST"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">5</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 CDT"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">5</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 CST"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">6</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 MDT"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">6</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 MST"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">7</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 PDT"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">7</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 PST"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 pst"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))),
            <span class="comment">// named single-letter military timezones must fallback to +0000
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 Z"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, <span class="number">0</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 A"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, <span class="number">0</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 a"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, <span class="number">0</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 K"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, <span class="number">0</span>))),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 k"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, <span class="number">0</span>))),
            <span class="comment">// named single-letter timezone "J" is specifically not valid
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 J"</span>, <span class="prelude-val">Err</span>(INVALID)),
            (<span class="string">"Tue, 20 Jan 2015 17:35:20 -0890"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad offset minutes
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20Z"</span>, <span class="prelude-val">Err</span>(INVALID)),           <span class="comment">// bad offset: zulu not allowed
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 Zulu"</span>, <span class="prelude-val">Err</span>(INVALID)),       <span class="comment">// bad offset: zulu not allowed
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 ZULU"</span>, <span class="prelude-val">Err</span>(INVALID)),       <span class="comment">// bad offset: zulu not allowed
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 −0800"</span>, <span class="prelude-val">Err</span>(INVALID)), <span class="comment">// bad offset: timezone offset using MINUS SIGN (U+2212), not specified for RFC 2822
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 0800"</span>, <span class="prelude-val">Err</span>(INVALID)),  <span class="comment">// missing offset sign
            </span>(<span class="string">"Tue, 20 Jan 2015 17:35:20 HAS"</span>, <span class="prelude-val">Err</span>(INVALID)),   <span class="comment">// bad named timezone
            </span>(<span class="string">"Tue, 20 Jan 2015😈17:35:20 -0800"</span>, <span class="prelude-val">Err</span>(INVALID)), <span class="comment">// bad character!
        </span>];

        <span class="kw">fn </span>rfc2822_to_datetime(date: <span class="kw-2">&amp;</span>str) -&gt; ParseResult&lt;DateTime&lt;FixedOffset&gt;&gt; {
            <span class="kw">let </span><span class="kw-2">mut </span>parsed = Parsed::new();
            parse(<span class="kw-2">&amp;mut </span>parsed, date, [Item::Fixed(Fixed::RFC2822)].iter())<span class="question-mark">?</span>;
            parsed.to_datetime()
        }

        <span class="comment">// Test against test data above
        </span><span class="kw">for </span><span class="kw-2">&amp;</span>(date, checkdate) <span class="kw">in </span>testdates.iter() {
            <span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
            </span><span class="macro">eprintln!</span>(<span class="string">"Test input: {:?}\n    Expect: {:?}"</span>, date, checkdate);
            <span class="kw">let </span>dt = rfc2822_to_datetime(date); <span class="comment">// parse a date
            </span><span class="kw">if </span>dt != checkdate {
                <span class="comment">// check for expected result
                </span><span class="macro">panic!</span>(
                    <span class="string">"Date conversion failed for {}\nReceived: {:?}\nExpected: {:?}"</span>,
                    date, dt, checkdate
                );
            }
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_rfc850() {
        <span class="kw">static </span>RFC850_FMT: <span class="kw-2">&amp;</span>str = <span class="string">"%A, %d-%b-%y %T GMT"</span>;

        <span class="kw">let </span>dt = Utc.with_ymd_and_hms(<span class="number">1994</span>, <span class="number">11</span>, <span class="number">6</span>, <span class="number">8</span>, <span class="number">49</span>, <span class="number">37</span>).unwrap();

        <span class="comment">// Check that the format is what we expect
        </span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
        </span><span class="macro">assert_eq!</span>(dt.format(RFC850_FMT).to_string(), <span class="string">"Sunday, 06-Nov-94 08:49:37 GMT"</span>);

        <span class="comment">// Check that it parses correctly
        </span><span class="macro">assert_eq!</span>(
            NaiveDateTime::parse_from_str(<span class="string">"Sunday, 06-Nov-94 08:49:37 GMT"</span>, RFC850_FMT),
            <span class="prelude-val">Ok</span>(dt.naive_utc())
        );

        <span class="comment">// Check that the rest of the weekdays parse correctly (this test originally failed because
        // Sunday parsed incorrectly).
        </span><span class="kw">let </span>testdates = [
            (
                Utc.with_ymd_and_hms(<span class="number">1994</span>, <span class="number">11</span>, <span class="number">7</span>, <span class="number">8</span>, <span class="number">49</span>, <span class="number">37</span>).unwrap(),
                <span class="string">"Monday, 07-Nov-94 08:49:37 GMT"</span>,
            ),
            (
                Utc.with_ymd_and_hms(<span class="number">1994</span>, <span class="number">11</span>, <span class="number">8</span>, <span class="number">8</span>, <span class="number">49</span>, <span class="number">37</span>).unwrap(),
                <span class="string">"Tuesday, 08-Nov-94 08:49:37 GMT"</span>,
            ),
            (
                Utc.with_ymd_and_hms(<span class="number">1994</span>, <span class="number">11</span>, <span class="number">9</span>, <span class="number">8</span>, <span class="number">49</span>, <span class="number">37</span>).unwrap(),
                <span class="string">"Wednesday, 09-Nov-94 08:49:37 GMT"</span>,
            ),
            (
                Utc.with_ymd_and_hms(<span class="number">1994</span>, <span class="number">11</span>, <span class="number">10</span>, <span class="number">8</span>, <span class="number">49</span>, <span class="number">37</span>).unwrap(),
                <span class="string">"Thursday, 10-Nov-94 08:49:37 GMT"</span>,
            ),
            (
                Utc.with_ymd_and_hms(<span class="number">1994</span>, <span class="number">11</span>, <span class="number">11</span>, <span class="number">8</span>, <span class="number">49</span>, <span class="number">37</span>).unwrap(),
                <span class="string">"Friday, 11-Nov-94 08:49:37 GMT"</span>,
            ),
            (
                Utc.with_ymd_and_hms(<span class="number">1994</span>, <span class="number">11</span>, <span class="number">12</span>, <span class="number">8</span>, <span class="number">49</span>, <span class="number">37</span>).unwrap(),
                <span class="string">"Saturday, 12-Nov-94 08:49:37 GMT"</span>,
            ),
        ];

        <span class="kw">for </span>val <span class="kw">in </span><span class="kw-2">&amp;</span>testdates {
            <span class="macro">assert_eq!</span>(NaiveDateTime::parse_from_str(val.<span class="number">1</span>, RFC850_FMT), <span class="prelude-val">Ok</span>(val.<span class="number">0</span>.naive_utc()));
        }

        <span class="kw">let </span>test_dates_fail = [
            <span class="string">"Saturday, 12-Nov-94 08:49:37"</span>,
            <span class="string">"Saturday, 12-Nov-94 08:49:37 Z"</span>,
            <span class="string">"Saturday, 12-Nov-94 08:49:37 GMTTTT"</span>,
            <span class="string">"Saturday, 12-Nov-94 08:49:37 gmt"</span>,
            <span class="string">"Saturday, 12-Nov-94 08:49:37 +08:00"</span>,
            <span class="string">"Caturday, 12-Nov-94 08:49:37 GMT"</span>,
            <span class="string">"Saturday, 99-Nov-94 08:49:37 GMT"</span>,
            <span class="string">"Saturday, 12-Nov-2000 08:49:37 GMT"</span>,
            <span class="string">"Saturday, 12-Mop-94 08:49:37 GMT"</span>,
            <span class="string">"Saturday, 12-Nov-94 28:49:37 GMT"</span>,
            <span class="string">"Saturday, 12-Nov-94 08:99:37 GMT"</span>,
            <span class="string">"Saturday, 12-Nov-94 08:49:99 GMT"</span>,
        ];

        <span class="kw">for </span>val <span class="kw">in </span><span class="kw-2">&amp;</span>test_dates_fail {
            <span class="macro">assert!</span>(NaiveDateTime::parse_from_str(val, RFC850_FMT).is_err());
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_rfc3339() {
        <span class="kw">let </span>ymd_hmsn = |y, m, d, h, n, s, nano, off| {
            FixedOffset::east_opt(off * <span class="number">60 </span>* <span class="number">60</span>)
                .unwrap()
                .with_ymd_and_hms(y, m, d, h, n, s)
                .unwrap()
                .with_nanosecond(nano)
                .unwrap()
        };

        <span class="comment">// Test data - (input, Ok(expected result) or Err(error code))
        </span><span class="kw">let </span>testdates = [
            (<span class="string">"2015-01-20T17:35:20-08:00"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))), <span class="comment">// normal case
            </span>(<span class="string">"2015-01-20T17:35:20−08:00"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))), <span class="comment">// normal case with MINUS SIGN (U+2212)
            </span>(<span class="string">"1944-06-06T04:04:00Z"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">1944</span>, <span class="number">6</span>, <span class="number">6</span>, <span class="number">4</span>, <span class="number">4</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>))),           <span class="comment">// D-day
            </span>(<span class="string">"2001-09-11T09:45:00-08:00"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2001</span>, <span class="number">9</span>, <span class="number">11</span>, <span class="number">9</span>, <span class="number">45</span>, <span class="number">0</span>, <span class="number">0</span>, -<span class="number">8</span>))),
            (<span class="string">"2015-01-20T17:35:20.001-08:00"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">1_000_000</span>, -<span class="number">8</span>))),
            (<span class="string">"2015-01-20T17:35:20.001−08:00"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">1_000_000</span>, -<span class="number">8</span>))), <span class="comment">// with MINUS SIGN (U+2212)
            </span>(<span class="string">"2015-01-20T17:35:20.000031-08:00"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">31_000</span>, -<span class="number">8</span>))),
            (<span class="string">"2015-01-20T17:35:20.000000004-08:00"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">4</span>, -<span class="number">8</span>))),
            (<span class="string">"2015-01-20T17:35:20.000000004−08:00"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">4</span>, -<span class="number">8</span>))), <span class="comment">// with MINUS SIGN (U+2212)
            </span>(
                <span class="string">"2015-01-20T17:35:20.000000000452-08:00"</span>,
                <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>)),
            ), <span class="comment">// too small
            </span>(
                <span class="string">"2015-01-20T17:35:20.000000000452−08:00"</span>,
                <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>)),
            ), <span class="comment">// too small with MINUS SIGN (U+2212)
            </span>(<span class="string">"2015-01-20 17:35:20-08:00"</span>, <span class="prelude-val">Ok</span>(ymd_hmsn(<span class="number">2015</span>, <span class="number">1</span>, <span class="number">20</span>, <span class="number">17</span>, <span class="number">35</span>, <span class="number">20</span>, <span class="number">0</span>, -<span class="number">8</span>))), <span class="comment">// without 'T'
            </span>(<span class="string">"2015/01/20T17:35:20.001-08:00"</span>, <span class="prelude-val">Err</span>(INVALID)), <span class="comment">// wrong separator char YMD
            </span>(<span class="string">"2015-01-20T17-35-20.001-08:00"</span>, <span class="prelude-val">Err</span>(INVALID)), <span class="comment">// wrong separator char HMS
            </span>(<span class="string">"-01-20T17:35:20-08:00"</span>, <span class="prelude-val">Err</span>(INVALID)),         <span class="comment">// missing year
            </span>(<span class="string">"99-01-20T17:35:20-08:00"</span>, <span class="prelude-val">Err</span>(INVALID)),       <span class="comment">// bad year format
            </span>(<span class="string">"99999-01-20T17:35:20-08:00"</span>, <span class="prelude-val">Err</span>(INVALID)),    <span class="comment">// bad year value
            </span>(<span class="string">"-2000-01-20T17:35:20-08:00"</span>, <span class="prelude-val">Err</span>(INVALID)),    <span class="comment">// bad year value
            </span>(<span class="string">"2015-02-30T17:35:20-08:00"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad day of month value
            </span>(<span class="string">"2015-01-20T25:35:20-08:00"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad hour value
            </span>(<span class="string">"2015-01-20T17:65:20-08:00"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad minute value
            </span>(<span class="string">"2015-01-20T17:35:90-08:00"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad second value
            </span>(<span class="string">"2015-01-20T17:35:20-24:00"</span>, <span class="prelude-val">Err</span>(OUT_OF_RANGE)), <span class="comment">// bad offset value
            </span>(<span class="string">"15-01-20T17:35:20-08:00"</span>, <span class="prelude-val">Err</span>(INVALID)),       <span class="comment">// bad year format
            </span>(<span class="string">"15-01-20T17:35:20-08:00:00"</span>, <span class="prelude-val">Err</span>(INVALID)),    <span class="comment">// bad year format, bad offset format
            </span>(<span class="string">"2015-01-20T17:35:2008:00"</span>, <span class="prelude-val">Err</span>(INVALID)),      <span class="comment">// missing offset sign
            </span>(<span class="string">"2015-01-20T17:35:20 08:00"</span>, <span class="prelude-val">Err</span>(INVALID)),     <span class="comment">// missing offset sign
            </span>(<span class="string">"2015-01-20T17:35:20Zulu"</span>, <span class="prelude-val">Err</span>(TOO_LONG)),      <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T17:35:20 Zulu"</span>, <span class="prelude-val">Err</span>(INVALID)),      <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T17:35:20GMT"</span>, <span class="prelude-val">Err</span>(INVALID)),        <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T17:35:20 GMT"</span>, <span class="prelude-val">Err</span>(INVALID)),       <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T17:35:20+GMT"</span>, <span class="prelude-val">Err</span>(INVALID)),       <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T17:35:20++08:00"</span>, <span class="prelude-val">Err</span>(INVALID)),    <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T17:35:20--08:00"</span>, <span class="prelude-val">Err</span>(INVALID)),    <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T17:35:20−−08:00"</span>, <span class="prelude-val">Err</span>(INVALID)), <span class="comment">// bad offset format with MINUS SIGN (U+2212)
            </span>(<span class="string">"2015-01-20T17:35:20±08:00"</span>, <span class="prelude-val">Err</span>(INVALID)),  <span class="comment">// bad offset sign
            </span>(<span class="string">"2015-01-20T17:35:20-08-00"</span>, <span class="prelude-val">Err</span>(INVALID)),  <span class="comment">// bad offset separator
            </span>(<span class="string">"2015-01-20T17:35:20-08;00"</span>, <span class="prelude-val">Err</span>(INVALID)),  <span class="comment">// bad offset separator
            </span>(<span class="string">"2015-01-20T17:35:20-0800"</span>, <span class="prelude-val">Err</span>(INVALID)),   <span class="comment">// bad offset separator
            </span>(<span class="string">"2015-01-20T17:35:20-08:0"</span>, <span class="prelude-val">Err</span>(TOO_SHORT)), <span class="comment">// bad offset minutes
            </span>(<span class="string">"2015-01-20T17:35:20-08:AA"</span>, <span class="prelude-val">Err</span>(INVALID)),  <span class="comment">// bad offset minutes
            </span>(<span class="string">"2015-01-20T17:35:20-08:ZZ"</span>, <span class="prelude-val">Err</span>(INVALID)),  <span class="comment">// bad offset minutes
            </span>(<span class="string">"2015-01-20T17:35:20.001-08 : 00"</span>, <span class="prelude-val">Err</span>(INVALID)), <span class="comment">// bad offset separator
            </span>(<span class="string">"2015-01-20T17:35:20-08:00:00"</span>, <span class="prelude-val">Err</span>(TOO_LONG)), <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T17:35:20+08:"</span>, <span class="prelude-val">Err</span>(TOO_SHORT)),  <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T17:35:20-08:"</span>, <span class="prelude-val">Err</span>(TOO_SHORT)),  <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T17:35:20−08:"</span>, <span class="prelude-val">Err</span>(TOO_SHORT)), <span class="comment">// bad offset format with MINUS SIGN (U+2212)
            </span>(<span class="string">"2015-01-20T17:35:20-08"</span>, <span class="prelude-val">Err</span>(TOO_SHORT)),  <span class="comment">// bad offset format
            </span>(<span class="string">"2015-01-20T"</span>, <span class="prelude-val">Err</span>(TOO_SHORT)),             <span class="comment">// missing HMS
            </span>(<span class="string">"2015-01-20T00:00:1"</span>, <span class="prelude-val">Err</span>(TOO_SHORT)),      <span class="comment">// missing complete S
            </span>(<span class="string">"2015-01-20T00:00:1-08:00"</span>, <span class="prelude-val">Err</span>(INVALID)),  <span class="comment">// missing complete S
        </span>];

        <span class="comment">// Test against test data above
        </span><span class="kw">for </span><span class="kw-2">&amp;</span>(date, checkdate) <span class="kw">in </span>testdates.iter() {
            <span class="kw">let </span>dt = DateTime::&lt;FixedOffset&gt;::parse_from_rfc3339(date);
            <span class="kw">if </span>dt != checkdate {
                <span class="comment">// check for expected result
                </span><span class="macro">panic!</span>(
                    <span class="string">"Date conversion failed for {}\nReceived: {:?}\nExpected: {:?}"</span>,
                    date, dt, checkdate
                );
            }
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_issue_1010() {
        <span class="kw">let </span>dt = <span class="kw">crate</span>::NaiveDateTime::parse_from_str(<span class="string">"\u{c}SUN\u{e}\u{3000}\0m@J\u{3000}\0\u{3000}\0m\u{c}!\u{c}\u{b}\u{c}\u{c}\u{c}\u{c}%A\u{c}\u{b}\0SU\u{c}\u{c}"</span>,
        <span class="string">"\u{c}\u{c}%A\u{c}\u{b}\0SUN\u{c}\u{c}\u{c}SUNN\u{c}\u{c}\u{c}SUN\u{c}\u{c}!\u{c}\u{b}\u{c}\u{c}\u{c}\u{c}%A\u{c}\u{b}%a"</span>);
        <span class="macro">assert_eq!</span>(dt, <span class="prelude-val">Err</span>(ParseError(ParseErrorKind::Invalid)));
    }
}
</code></pre></div></section></main></body></html>