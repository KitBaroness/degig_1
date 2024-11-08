<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/chrono-0.4.38/src/format/strftime.rs`."><title>strftime.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="chrono" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../chrono/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

</span><span class="doccomment">/*!
`strftime`/`strptime`-inspired date and time formatting syntax.

## Specifiers

The following specifiers are available both to formatting and parsing.

| Spec. | Example  | Description                                                                |
|-------|----------|----------------------------------------------------------------------------|
|       |          | **DATE SPECIFIERS:**                                                       |
| `%Y`  | `2001`   | The full proleptic Gregorian year, zero-padded to 4 digits. chrono supports years from -262144 to 262143. Note: years before 1 BCE or after 9999 CE, require an initial sign (+/-).|
| `%C`  | `20`     | The proleptic Gregorian year divided by 100, zero-padded to 2 digits. [^1] |
| `%y`  | `01`     | The proleptic Gregorian year modulo 100, zero-padded to 2 digits. [^1]     |
|       |          |                                                                            |
| `%m`  | `07`     | Month number (01--12), zero-padded to 2 digits.                            |
| `%b`  | `Jul`    | Abbreviated month name. Always 3 letters.                                  |
| `%B`  | `July`   | Full month name. Also accepts corresponding abbreviation in parsing.       |
| `%h`  | `Jul`    | Same as `%b`.                                                              |
|       |          |                                                                            |
| `%d`  | `08`     | Day number (01--31), zero-padded to 2 digits.                              |
| `%e`  | ` 8`     | Same as `%d` but space-padded. Same as `%_d`.                              |
|       |          |                                                                            |
| `%a`  | `Sun`    | Abbreviated weekday name. Always 3 letters.                                |
| `%A`  | `Sunday` | Full weekday name. Also accepts corresponding abbreviation in parsing.     |
| `%w`  | `0`      | Sunday = 0, Monday = 1, ..., Saturday = 6.                                 |
| `%u`  | `7`      | Monday = 1, Tuesday = 2, ..., Sunday = 7. (ISO 8601)                       |
|       |          |                                                                            |
| `%U`  | `28`     | Week number starting with Sunday (00--53), zero-padded to 2 digits. [^2]   |
| `%W`  | `27`     | Same as `%U`, but week 1 starts with the first Monday in that year instead.|
|       |          |                                                                            |
| `%G`  | `2001`   | Same as `%Y` but uses the year number in ISO 8601 week date. [^3]          |
| `%g`  | `01`     | Same as `%y` but uses the year number in ISO 8601 week date. [^3]          |
| `%V`  | `27`     | Same as `%U` but uses the week number in ISO 8601 week date (01--53). [^3] |
|       |          |                                                                            |
| `%j`  | `189`    | Day of the year (001--366), zero-padded to 3 digits.                       |
|       |          |                                                                            |
| `%D`  | `07/08/01`    | Month-day-year format. Same as `%m/%d/%y`.                            |
| `%x`  | `07/08/01`    | Locale's date representation (e.g., 12/31/99).                        |
| `%F`  | `2001-07-08`  | Year-month-day format (ISO 8601). Same as `%Y-%m-%d`.                 |
| `%v`  | ` 8-Jul-2001` | Day-month-year format. Same as `%e-%b-%Y`.                            |
|       |          |                                                                            |
|       |          | **TIME SPECIFIERS:**                                                       |
| `%H`  | `00`     | Hour number (00--23), zero-padded to 2 digits.                             |
| `%k`  | ` 0`     | Same as `%H` but space-padded. Same as `%_H`.                              |
| `%I`  | `12`     | Hour number in 12-hour clocks (01--12), zero-padded to 2 digits.           |
| `%l`  | `12`     | Same as `%I` but space-padded. Same as `%_I`.                              |
|       |          |                                                                            |
| `%P`  | `am`     | `am` or `pm` in 12-hour clocks.                                            |
| `%p`  | `AM`     | `AM` or `PM` in 12-hour clocks.                                            |
|       |          |                                                                            |
| `%M`  | `34`     | Minute number (00--59), zero-padded to 2 digits.                           |
| `%S`  | `60`     | Second number (00--60), zero-padded to 2 digits. [^4]                      |
| `%f`  | `26490000`    | Number of nanoseconds since last whole second. [^7]                   |
| `%.f` | `.026490`| Decimal fraction of a second. Consumes the leading dot. [^7]               |
| `%.3f`| `.026`        | Decimal fraction of a second with a fixed length of 3.                |
| `%.6f`| `.026490`     | Decimal fraction of a second with a fixed length of 6.                |
| `%.9f`| `.026490000`  | Decimal fraction of a second with a fixed length of 9.                |
| `%3f` | `026`         | Decimal fraction of a second like `%.3f` but without the leading dot. |
| `%6f` | `026490`      | Decimal fraction of a second like `%.6f` but without the leading dot. |
| `%9f` | `026490000`   | Decimal fraction of a second like `%.9f` but without the leading dot. |
|       |               |                                                                       |
| `%R`  | `00:34`       | Hour-minute format. Same as `%H:%M`.                                  |
| `%T`  | `00:34:60`    | Hour-minute-second format. Same as `%H:%M:%S`.                        |
| `%X`  | `00:34:60`    | Locale's time representation (e.g., 23:13:48).                        |
| `%r`  | `12:34:60 AM` | Locale's 12 hour clock time. (e.g., 11:11:04 PM). Falls back to `%X` if the locale does not have a 12 hour clock format. |
|       |          |                                                                            |
|       |          | **TIME ZONE SPECIFIERS:**                                                  |
| `%Z`  | `ACST`   | Local time zone name. Skips all non-whitespace characters during parsing. Identical to `%:z` when formatting. [^8] |
| `%z`  | `+0930`  | Offset from the local time to UTC (with UTC being `+0000`).                |
| `%:z` | `+09:30` | Same as `%z` but with a colon.                                             |
|`%::z`|`+09:30:00`| Offset from the local time to UTC with seconds.                            |
|`%:::z`| `+09`    | Offset from the local time to UTC without minutes.                         |
| `%#z` | `+09`    | *Parsing only:* Same as `%z` but allows minutes to be missing or present.  |
|       |          |                                                                            |
|       |          | **DATE &amp; TIME SPECIFIERS:**                                                |
|`%c`|`Sun Jul  8 00:34:60 2001`|Locale's date and time (e.g., Thu Mar  3 23:05:25 2005).       |
| `%+`  | `2001-07-08T00:34:60.026490+09:30` | ISO 8601 / RFC 3339 date &amp; time format. [^5]     |
|       |               |                                                                       |
| `%s`  | `994518299`   | UNIX timestamp, the number of seconds since 1970-01-01 00:00 UTC. [^6]|
|       |          |                                                                            |
|       |          | **SPECIAL SPECIFIERS:**                                                    |
| `%t`  |          | Literal tab (`\t`).                                                        |
| `%n`  |          | Literal newline (`\n`).                                                    |
| `%%`  |          | Literal percent sign.                                                      |

It is possible to override the default padding behavior of numeric specifiers `%?`.
This is not allowed for other specifiers and will result in the `BAD_FORMAT` error.

Modifier | Description
-------- | -----------
`%-?`    | Suppresses any padding including spaces and zeroes. (e.g. `%j` = `012`, `%-j` = `12`)
`%_?`    | Uses spaces as a padding. (e.g. `%j` = `012`, `%_j` = ` 12`)
`%0?`    | Uses zeroes as a padding. (e.g. `%e` = ` 9`, `%0e` = `09`)

Notes:

[^1]: `%C`, `%y`:
   This is floor division, so 100 BCE (year number -99) will print `-1` and `99` respectively.

[^2]: `%U`:
   Week 1 starts with the first Sunday in that year.
   It is possible to have week 0 for days before the first Sunday.

[^3]: `%G`, `%g`, `%V`:
   Week 1 is the first week with at least 4 days in that year.
   Week 0 does not exist, so this should be used with `%G` or `%g`.

[^4]: `%S`:
   It accounts for leap seconds, so `60` is possible.

[^5]: `%+`: Same as `%Y-%m-%dT%H:%M:%S%.f%:z`, i.e. 0, 3, 6 or 9 fractional
   digits for seconds and colons in the time zone offset.
   &lt;br&gt;
   &lt;br&gt;
   This format also supports having a `Z` or `UTC` in place of `%:z`. They
   are equivalent to `+00:00`.
   &lt;br&gt;
   &lt;br&gt;
   Note that all `T`, `Z`, and `UTC` are parsed case-insensitively.
   &lt;br&gt;
   &lt;br&gt;
   The typical `strftime` implementations have different (and locale-dependent)
   formats for this specifier. While Chrono's format for `%+` is far more
   stable, it is best to avoid this specifier if you want to control the exact
   output.

[^6]: `%s`:
   This is not padded and can be negative.
   For the purpose of Chrono, it only accounts for non-leap seconds
   so it slightly differs from ISO C `strftime` behavior.

[^7]: `%f`, `%.f`:
   &lt;br&gt;
   `%f` and `%.f` are notably different formatting specifiers.&lt;br&gt;
   `%f` counts the number of nanoseconds since the last whole second, while `%.f` is a fraction of a
   second.&lt;br&gt;
   Example: 7μs is formatted as `7000` with `%f`, and formatted as `.000007` with `%.f`.

[^8]: `%Z`:
   Since `chrono` is not aware of timezones beyond their offsets, this specifier
   **only prints the offset** when used for formatting. The timezone abbreviation
   will NOT be printed. See [this issue](https://github.com/chronotope/chrono/issues/960)
   for more information.
   &lt;br&gt;
   &lt;br&gt;
   Offset will not be populated from the parsed data, nor will it be validated.
   Timezone is completely ignored. Similar to the glibc `strptime` treatment of
   this format code.
   &lt;br&gt;
   &lt;br&gt;
   It is not possible to reliably convert from an abbreviation to an offset,
   for example CDT can mean either Central Daylight Time (North America) or
   China Daylight Time.
*/

</span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">extern crate </span>alloc;

<span class="kw">use super</span>::{fixed, internal_fixed, num, num0, nums};
<span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
</span><span class="kw">use super</span>::{locales, Locale};
<span class="kw">use super</span>::{Fixed, InternalInternal, Item, Numeric, Pad};
<span class="attr">#[cfg(any(feature = <span class="string">"alloc"</span>, feature = <span class="string">"std"</span>))]
</span><span class="kw">use super</span>::{ParseError, BAD_FORMAT};
<span class="attr">#[cfg(all(feature = <span class="string">"alloc"</span>, not(feature = <span class="string">"std"</span>), not(test)))]
</span><span class="kw">use </span>alloc::vec::Vec;

<span class="doccomment">/// Parsing iterator for `strftime`-like format strings.
///
/// See the [`format::strftime` module](crate::format::strftime) for supported formatting
/// specifiers.
///
/// `StrftimeItems` is used in combination with more low-level methods such as [`format::parse()`]
/// or [`format_with_items`].
///
/// If formatting or parsing date and time values is not performance-critical, the methods
/// [`parse_from_str`] and [`format`] on types such as [`DateTime`](crate::DateTime) are easier to
/// use.
///
/// [`format`]: crate::DateTime::format
/// [`format_with_items`]: crate::DateTime::format
/// [`parse_from_str`]: crate::DateTime::parse_from_str
/// [`DateTime`]: crate::DateTime
/// [`format::parse()`]: crate::format::parse()
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>StrftimeItems&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// Remaining portion of the string.
    </span>remainder: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str,
    <span class="doccomment">/// If the current specifier is composed of multiple formatting items (e.g. `%+`),
    /// `queue` stores a slice of `Item`s that have to be returned one by one.
    </span>queue: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[Item&lt;<span class="lifetime">'static</span>&gt;],
    <span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
    </span>locale_str: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str,
    <span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
    </span>locale: <span class="prelude-ty">Option</span>&lt;Locale&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; StrftimeItems&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// Creates a new parsing iterator from a `strftime`-like format string.
    ///
    /// # Errors
    ///
    /// While iterating [`Item::Error`] will be returned if the format string contains an invalid
    /// or unrecognized formatting specifier.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::format::*;
    ///
    /// let strftime_parser = StrftimeItems::new("%F"); // %F: year-month-day (ISO 8601)
    ///
    /// const ISO8601_YMD_ITEMS: &amp;[Item&lt;'static&gt;] = &amp;[
    ///     Item::Numeric(Numeric::Year, Pad::Zero),
    ///     Item::Literal("-"),
    ///     Item::Numeric(Numeric::Month, Pad::Zero),
    ///     Item::Literal("-"),
    ///     Item::Numeric(Numeric::Day, Pad::Zero),
    /// ];
    /// assert!(strftime_parser.eq(ISO8601_YMD_ITEMS.iter().cloned()));
    /// ```
    </span><span class="attr">#[must_use]
    </span><span class="kw">pub const fn </span>new(s: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str) -&gt; StrftimeItems&lt;<span class="lifetime">'a</span>&gt; {
        <span class="attr">#[cfg(not(feature = <span class="string">"unstable-locales"</span>))]
        </span>{
            StrftimeItems { remainder: s, queue: <span class="kw-2">&amp;</span>[] }
        }
        <span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
        </span>{
            StrftimeItems { remainder: s, queue: <span class="kw-2">&amp;</span>[], locale_str: <span class="string">""</span>, locale: <span class="prelude-val">None </span>}
        }
    }

    <span class="doccomment">/// Creates a new parsing iterator from a `strftime`-like format string, with some formatting
    /// specifiers adjusted to match [`Locale`].
    ///
    /// Note: `StrftimeItems::new_with_locale` only localizes the *format*. You usually want to
    /// combine it with other locale-aware methods such as
    /// [`DateTime::format_localized_with_items`] to get things like localized month or day names.
    ///
    /// The `%x` formatting specifier will use the local date format, `%X` the local time format,
    ///  and `%c` the local format for date and time.
    /// `%r` will use the local 12-hour clock format (e.g., 11:11:04 PM). Not all locales have such
    /// a format, in which case we fall back to a 24-hour clock (`%X`).
    ///
    /// See the [`format::strftime` module](crate::format::strftime) for all supported formatting
    /// specifiers.
    ///
    ///  [`DateTime::format_localized_with_items`]: crate::DateTime::format_localized_with_items
    ///
    /// # Errors
    ///
    /// While iterating [`Item::Error`] will be returned if the format string contains an invalid
    /// or unrecognized formatting specifier.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "alloc")] {
    /// use chrono::format::{Locale, StrftimeItems};
    /// use chrono::{FixedOffset, TimeZone};
    ///
    /// let dt = FixedOffset::east_opt(9 * 60 * 60)
    ///     .unwrap()
    ///     .with_ymd_and_hms(2023, 7, 11, 0, 34, 59)
    ///     .unwrap();
    ///
    /// // Note: you usually want to combine `StrftimeItems::new_with_locale` with other
    /// // locale-aware methods such as `DateTime::format_localized_with_items`.
    /// // We use the regular `format_with_items` to show only how the formatting changes.
    ///
    /// let fmtr = dt.format_with_items(StrftimeItems::new_with_locale("%x", Locale::en_US));
    /// assert_eq!(fmtr.to_string(), "07/11/2023");
    /// let fmtr = dt.format_with_items(StrftimeItems::new_with_locale("%x", Locale::ko_KR));
    /// assert_eq!(fmtr.to_string(), "2023년 07월 11일");
    /// let fmtr = dt.format_with_items(StrftimeItems::new_with_locale("%x", Locale::ja_JP));
    /// assert_eq!(fmtr.to_string(), "2023年07月11日");
    /// # }
    /// ```
    </span><span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
    #[must_use]
    </span><span class="kw">pub const fn </span>new_with_locale(s: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str, locale: Locale) -&gt; StrftimeItems&lt;<span class="lifetime">'a</span>&gt; {
        StrftimeItems { remainder: s, queue: <span class="kw-2">&amp;</span>[], locale_str: <span class="string">""</span>, locale: <span class="prelude-val">Some</span>(locale) }
    }

    <span class="doccomment">/// Parse format string into a `Vec` of formatting [`Item`]'s.
    ///
    /// If you need to format or parse multiple values with the same format string, it is more
    /// efficient to convert it to a `Vec` of formatting [`Item`]'s than to re-parse the format
    /// string on every use.
    ///
    /// The `format_with_items` methods on [`DateTime`], [`NaiveDateTime`], [`NaiveDate`] and
    /// [`NaiveTime`] accept the result for formatting. [`format::parse()`] can make use of it for
    /// parsing.
    ///
    /// [`DateTime`]: crate::DateTime::format_with_items
    /// [`NaiveDateTime`]: crate::NaiveDateTime::format_with_items
    /// [`NaiveDate`]: crate::NaiveDate::format_with_items
    /// [`NaiveTime`]: crate::NaiveTime::format_with_items
    /// [`format::parse()`]: crate::format::parse()
    ///
    /// # Errors
    ///
    /// Returns an error if the format string contains an invalid or unrecognized formatting
    /// specifier.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::format::{parse, Parsed, StrftimeItems};
    /// use chrono::NaiveDate;
    ///
    /// let fmt_items = StrftimeItems::new("%e %b %Y %k.%M").parse()?;
    /// let datetime = NaiveDate::from_ymd_opt(2023, 7, 11).unwrap().and_hms_opt(9, 0, 0).unwrap();
    ///
    /// // Formatting
    /// assert_eq!(
    ///     datetime.format_with_items(fmt_items.as_slice().iter()).to_string(),
    ///     "11 Jul 2023  9.00"
    /// );
    ///
    /// // Parsing
    /// let mut parsed = Parsed::new();
    /// parse(&amp;mut parsed, "11 Jul 2023  9.00", fmt_items.as_slice().iter())?;
    /// let parsed_dt = parsed.to_naive_datetime_with_offset(0)?;
    /// assert_eq!(parsed_dt, datetime);
    /// # Ok::&lt;(), chrono::ParseError&gt;(())
    /// ```
    </span><span class="attr">#[cfg(any(feature = <span class="string">"alloc"</span>, feature = <span class="string">"std"</span>))]
    </span><span class="kw">pub fn </span>parse(<span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Vec&lt;Item&lt;<span class="lifetime">'a</span>&gt;&gt;, ParseError&gt; {
        <span class="self">self</span>.into_iter()
            .map(|item| <span class="kw">match </span>item == Item::Error {
                <span class="bool-val">false </span>=&gt; <span class="prelude-val">Ok</span>(item),
                <span class="bool-val">true </span>=&gt; <span class="prelude-val">Err</span>(BAD_FORMAT),
            })
            .collect()
    }

    <span class="doccomment">/// Parse format string into a `Vec` of [`Item`]'s that contain no references to slices of the
    /// format string.
    ///
    /// A `Vec` created with [`StrftimeItems::parse`] contains references to the format string,
    /// binding the lifetime of the `Vec` to that string. [`StrftimeItems::parse_to_owned`] will
    /// convert the references to owned types.
    ///
    /// # Errors
    ///
    /// Returns an error if the format string contains an invalid or unrecognized formatting
    /// specifier.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::format::{Item, ParseError, StrftimeItems};
    /// use chrono::NaiveDate;
    ///
    /// fn format_items(date_fmt: &amp;str, time_fmt: &amp;str) -&gt; Result&lt;Vec&lt;Item&lt;'static&gt;&gt;, ParseError&gt; {
    ///     // `fmt_string` is dropped at the end of this function.
    ///     let fmt_string = format!("{} {}", date_fmt, time_fmt);
    ///     StrftimeItems::new(&amp;fmt_string).parse_to_owned()
    /// }
    ///
    /// let fmt_items = format_items("%e %b %Y", "%k.%M")?;
    /// let datetime = NaiveDate::from_ymd_opt(2023, 7, 11).unwrap().and_hms_opt(9, 0, 0).unwrap();
    ///
    /// assert_eq!(
    ///     datetime.format_with_items(fmt_items.as_slice().iter()).to_string(),
    ///     "11 Jul 2023  9.00"
    /// );
    /// # Ok::&lt;(), ParseError&gt;(())
    /// ```
    </span><span class="attr">#[cfg(any(feature = <span class="string">"alloc"</span>, feature = <span class="string">"std"</span>))]
    </span><span class="kw">pub fn </span>parse_to_owned(<span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Vec&lt;Item&lt;<span class="lifetime">'static</span>&gt;&gt;, ParseError&gt; {
        <span class="self">self</span>.into_iter()
            .map(|item| <span class="kw">match </span>item == Item::Error {
                <span class="bool-val">false </span>=&gt; <span class="prelude-val">Ok</span>(item.to_owned()),
                <span class="bool-val">true </span>=&gt; <span class="prelude-val">Err</span>(BAD_FORMAT),
            })
            .collect()
    }
}

<span class="kw">const </span>HAVE_ALTERNATES: <span class="kw-2">&amp;</span>str = <span class="string">"z"</span>;

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>StrftimeItems&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = Item&lt;<span class="lifetime">'a</span>&gt;;

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Item&lt;<span class="lifetime">'a</span>&gt;&gt; {
        <span class="comment">// We have items queued to return from a specifier composed of multiple formatting items.
        </span><span class="kw">if let </span><span class="prelude-val">Some</span>((item, remainder)) = <span class="self">self</span>.queue.split_first() {
            <span class="self">self</span>.queue = remainder;
            <span class="kw">return </span><span class="prelude-val">Some</span>(item.clone());
        }

        <span class="comment">// We are in the middle of parsing the localized formatting string of a specifier.
        </span><span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
        </span><span class="kw">if </span>!<span class="self">self</span>.locale_str.is_empty() {
            <span class="kw">let </span>(remainder, item) = <span class="self">self</span>.parse_next_item(<span class="self">self</span>.locale_str)<span class="question-mark">?</span>;
            <span class="self">self</span>.locale_str = remainder;
            <span class="kw">return </span><span class="prelude-val">Some</span>(item);
        }

        <span class="comment">// Normal: we are parsing the formatting string.
        </span><span class="kw">let </span>(remainder, item) = <span class="self">self</span>.parse_next_item(<span class="self">self</span>.remainder)<span class="question-mark">?</span>;
        <span class="self">self</span>.remainder = remainder;
        <span class="prelude-val">Some</span>(item)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; StrftimeItems&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">fn </span>parse_next_item(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw-2">mut </span>remainder: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str) -&gt; <span class="prelude-ty">Option</span>&lt;(<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str, Item&lt;<span class="lifetime">'a</span>&gt;)&gt; {
        <span class="kw">use </span>InternalInternal::<span class="kw-2">*</span>;
        <span class="kw">use </span>Item::{Literal, Space};
        <span class="kw">use </span>Numeric::<span class="kw-2">*</span>;

        <span class="kw">static </span>D_FMT: <span class="kw-2">&amp;</span>[Item&lt;<span class="lifetime">'static</span>&gt;] =
            <span class="kw-2">&amp;</span>[num0(Month), Literal(<span class="string">"/"</span>), num0(Day), Literal(<span class="string">"/"</span>), num0(YearMod100)];
        <span class="kw">static </span>D_T_FMT: <span class="kw-2">&amp;</span>[Item&lt;<span class="lifetime">'static</span>&gt;] = <span class="kw-2">&amp;</span>[
            fixed(Fixed::ShortWeekdayName),
            Space(<span class="string">" "</span>),
            fixed(Fixed::ShortMonthName),
            Space(<span class="string">" "</span>),
            nums(Day),
            Space(<span class="string">" "</span>),
            num0(Hour),
            Literal(<span class="string">":"</span>),
            num0(Minute),
            Literal(<span class="string">":"</span>),
            num0(Second),
            Space(<span class="string">" "</span>),
            num0(Year),
        ];
        <span class="kw">static </span>T_FMT: <span class="kw-2">&amp;</span>[Item&lt;<span class="lifetime">'static</span>&gt;] =
            <span class="kw-2">&amp;</span>[num0(Hour), Literal(<span class="string">":"</span>), num0(Minute), Literal(<span class="string">":"</span>), num0(Second)];
        <span class="kw">static </span>T_FMT_AMPM: <span class="kw-2">&amp;</span>[Item&lt;<span class="lifetime">'static</span>&gt;] = <span class="kw-2">&amp;</span>[
            num0(Hour12),
            Literal(<span class="string">":"</span>),
            num0(Minute),
            Literal(<span class="string">":"</span>),
            num0(Second),
            Space(<span class="string">" "</span>),
            fixed(Fixed::UpperAmPm),
        ];

        <span class="kw">match </span>remainder.chars().next() {
            <span class="comment">// we are done
            </span><span class="prelude-val">None </span>=&gt; <span class="prelude-val">None</span>,

            <span class="comment">// the next item is a specifier
            </span><span class="prelude-val">Some</span>(<span class="string">'%'</span>) =&gt; {
                remainder = <span class="kw-2">&amp;</span>remainder[<span class="number">1</span>..];

                <span class="macro">macro_rules!</span> next {
                    () =&gt; {
                        <span class="kw">match </span>remainder.chars().next() {
                            <span class="prelude-val">Some</span>(x) =&gt; {
                                remainder = <span class="kw-2">&amp;</span>remainder[x.len_utf8()..];
                                x
                            }
                            <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Some</span>((remainder, Item::Error)), <span class="comment">// premature end of string
                        </span>}
                    };
                }

                <span class="kw">let </span>spec = <span class="macro">next!</span>();
                <span class="kw">let </span>pad_override = <span class="kw">match </span>spec {
                    <span class="string">'-' </span>=&gt; <span class="prelude-val">Some</span>(Pad::None),
                    <span class="string">'0' </span>=&gt; <span class="prelude-val">Some</span>(Pad::Zero),
                    <span class="string">'_' </span>=&gt; <span class="prelude-val">Some</span>(Pad::Space),
                    <span class="kw">_ </span>=&gt; <span class="prelude-val">None</span>,
                };
                <span class="kw">let </span>is_alternate = spec == <span class="string">'#'</span>;
                <span class="kw">let </span>spec = <span class="kw">if </span>pad_override.is_some() || is_alternate { <span class="macro">next!</span>() } <span class="kw">else </span>{ spec };
                <span class="kw">if </span>is_alternate &amp;&amp; !HAVE_ALTERNATES.contains(spec) {
                    <span class="kw">return </span><span class="prelude-val">Some</span>((remainder, Item::Error));
                }

                <span class="macro">macro_rules!</span> queue {
                    [<span class="macro-nonterminal">$head</span>:expr, $(<span class="macro-nonterminal">$tail</span>:expr),+ $(,)<span class="kw-2">*</span>] =&gt; ({
                        <span class="kw">const </span>QUEUE: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[Item&lt;<span class="lifetime">'static</span>&gt;] = <span class="kw-2">&amp;</span>[$(<span class="macro-nonterminal">$tail</span>),+];
                        <span class="self">self</span>.queue = QUEUE;
                        <span class="macro-nonterminal">$head
                    </span>})
                }
                <span class="attr">#[cfg(not(feature = <span class="string">"unstable-locales"</span>))]
                </span><span class="macro">macro_rules!</span> queue_from_slice {
                    (<span class="macro-nonterminal">$slice</span>:expr) =&gt; {{
                        <span class="self">self</span>.queue = <span class="kw-2">&amp;</span><span class="macro-nonterminal">$slice</span>[<span class="number">1</span>..];
                        <span class="macro-nonterminal">$slice</span>[<span class="number">0</span>].clone()
                    }};
                }

                <span class="kw">let </span>item = <span class="kw">match </span>spec {
                    <span class="string">'A' </span>=&gt; fixed(Fixed::LongWeekdayName),
                    <span class="string">'B' </span>=&gt; fixed(Fixed::LongMonthName),
                    <span class="string">'C' </span>=&gt; num0(YearDiv100),
                    <span class="string">'D' </span>=&gt; {
                        <span class="macro">queue!</span>[num0(Month), Literal(<span class="string">"/"</span>), num0(Day), Literal(<span class="string">"/"</span>), num0(YearMod100)]
                    }
                    <span class="string">'F' </span>=&gt; <span class="macro">queue!</span>[num0(Year), Literal(<span class="string">"-"</span>), num0(Month), Literal(<span class="string">"-"</span>), num0(Day)],
                    <span class="string">'G' </span>=&gt; num0(IsoYear),
                    <span class="string">'H' </span>=&gt; num0(Hour),
                    <span class="string">'I' </span>=&gt; num0(Hour12),
                    <span class="string">'M' </span>=&gt; num0(Minute),
                    <span class="string">'P' </span>=&gt; fixed(Fixed::LowerAmPm),
                    <span class="string">'R' </span>=&gt; <span class="macro">queue!</span>[num0(Hour), Literal(<span class="string">":"</span>), num0(Minute)],
                    <span class="string">'S' </span>=&gt; num0(Second),
                    <span class="string">'T' </span>=&gt; {
                        <span class="macro">queue!</span>[num0(Hour), Literal(<span class="string">":"</span>), num0(Minute), Literal(<span class="string">":"</span>), num0(Second)]
                    }
                    <span class="string">'U' </span>=&gt; num0(WeekFromSun),
                    <span class="string">'V' </span>=&gt; num0(IsoWeek),
                    <span class="string">'W' </span>=&gt; num0(WeekFromMon),
                    <span class="attr">#[cfg(not(feature = <span class="string">"unstable-locales"</span>))]
                    </span><span class="string">'X' </span>=&gt; <span class="macro">queue_from_slice!</span>(T_FMT),
                    <span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
                    </span><span class="string">'X' </span>=&gt; <span class="self">self</span>.switch_to_locale_str(locales::t_fmt, T_FMT),
                    <span class="string">'Y' </span>=&gt; num0(Year),
                    <span class="string">'Z' </span>=&gt; fixed(Fixed::TimezoneName),
                    <span class="string">'a' </span>=&gt; fixed(Fixed::ShortWeekdayName),
                    <span class="string">'b' </span>| <span class="string">'h' </span>=&gt; fixed(Fixed::ShortMonthName),
                    <span class="attr">#[cfg(not(feature = <span class="string">"unstable-locales"</span>))]
                    </span><span class="string">'c' </span>=&gt; <span class="macro">queue_from_slice!</span>(D_T_FMT),
                    <span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
                    </span><span class="string">'c' </span>=&gt; <span class="self">self</span>.switch_to_locale_str(locales::d_t_fmt, D_T_FMT),
                    <span class="string">'d' </span>=&gt; num0(Day),
                    <span class="string">'e' </span>=&gt; nums(Day),
                    <span class="string">'f' </span>=&gt; num0(Nanosecond),
                    <span class="string">'g' </span>=&gt; num0(IsoYearMod100),
                    <span class="string">'j' </span>=&gt; num0(Ordinal),
                    <span class="string">'k' </span>=&gt; nums(Hour),
                    <span class="string">'l' </span>=&gt; nums(Hour12),
                    <span class="string">'m' </span>=&gt; num0(Month),
                    <span class="string">'n' </span>=&gt; Space(<span class="string">"\n"</span>),
                    <span class="string">'p' </span>=&gt; fixed(Fixed::UpperAmPm),
                    <span class="attr">#[cfg(not(feature = <span class="string">"unstable-locales"</span>))]
                    </span><span class="string">'r' </span>=&gt; <span class="macro">queue_from_slice!</span>(T_FMT_AMPM),
                    <span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
                    </span><span class="string">'r' </span>=&gt; {
                        <span class="kw">if </span><span class="self">self</span>.locale.is_some()
                            &amp;&amp; locales::t_fmt_ampm(<span class="self">self</span>.locale.unwrap()).is_empty()
                        {
                            <span class="comment">// 12-hour clock not supported by this locale. Switch to 24-hour format.
                            </span><span class="self">self</span>.switch_to_locale_str(locales::t_fmt, T_FMT)
                        } <span class="kw">else </span>{
                            <span class="self">self</span>.switch_to_locale_str(locales::t_fmt_ampm, T_FMT_AMPM)
                        }
                    }
                    <span class="string">'s' </span>=&gt; num(Timestamp),
                    <span class="string">'t' </span>=&gt; Space(<span class="string">"\t"</span>),
                    <span class="string">'u' </span>=&gt; num(WeekdayFromMon),
                    <span class="string">'v' </span>=&gt; {
                        <span class="macro">queue!</span>[
                            nums(Day),
                            Literal(<span class="string">"-"</span>),
                            fixed(Fixed::ShortMonthName),
                            Literal(<span class="string">"-"</span>),
                            num0(Year)
                        ]
                    }
                    <span class="string">'w' </span>=&gt; num(NumDaysFromSun),
                    <span class="attr">#[cfg(not(feature = <span class="string">"unstable-locales"</span>))]
                    </span><span class="string">'x' </span>=&gt; <span class="macro">queue_from_slice!</span>(D_FMT),
                    <span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
                    </span><span class="string">'x' </span>=&gt; <span class="self">self</span>.switch_to_locale_str(locales::d_fmt, D_FMT),
                    <span class="string">'y' </span>=&gt; num0(YearMod100),
                    <span class="string">'z' </span>=&gt; {
                        <span class="kw">if </span>is_alternate {
                            internal_fixed(TimezoneOffsetPermissive)
                        } <span class="kw">else </span>{
                            fixed(Fixed::TimezoneOffset)
                        }
                    }
                    <span class="string">'+' </span>=&gt; fixed(Fixed::RFC3339),
                    <span class="string">':' </span>=&gt; {
                        <span class="kw">if </span>remainder.starts_with(<span class="string">"::z"</span>) {
                            remainder = <span class="kw-2">&amp;</span>remainder[<span class="number">3</span>..];
                            fixed(Fixed::TimezoneOffsetTripleColon)
                        } <span class="kw">else if </span>remainder.starts_with(<span class="string">":z"</span>) {
                            remainder = <span class="kw-2">&amp;</span>remainder[<span class="number">2</span>..];
                            fixed(Fixed::TimezoneOffsetDoubleColon)
                        } <span class="kw">else if </span>remainder.starts_with(<span class="string">'z'</span>) {
                            remainder = <span class="kw-2">&amp;</span>remainder[<span class="number">1</span>..];
                            fixed(Fixed::TimezoneOffsetColon)
                        } <span class="kw">else </span>{
                            Item::Error
                        }
                    }
                    <span class="string">'.' </span>=&gt; <span class="kw">match </span><span class="macro">next!</span>() {
                        <span class="string">'3' </span>=&gt; <span class="kw">match </span><span class="macro">next!</span>() {
                            <span class="string">'f' </span>=&gt; fixed(Fixed::Nanosecond3),
                            <span class="kw">_ </span>=&gt; Item::Error,
                        },
                        <span class="string">'6' </span>=&gt; <span class="kw">match </span><span class="macro">next!</span>() {
                            <span class="string">'f' </span>=&gt; fixed(Fixed::Nanosecond6),
                            <span class="kw">_ </span>=&gt; Item::Error,
                        },
                        <span class="string">'9' </span>=&gt; <span class="kw">match </span><span class="macro">next!</span>() {
                            <span class="string">'f' </span>=&gt; fixed(Fixed::Nanosecond9),
                            <span class="kw">_ </span>=&gt; Item::Error,
                        },
                        <span class="string">'f' </span>=&gt; fixed(Fixed::Nanosecond),
                        <span class="kw">_ </span>=&gt; Item::Error,
                    },
                    <span class="string">'3' </span>=&gt; <span class="kw">match </span><span class="macro">next!</span>() {
                        <span class="string">'f' </span>=&gt; internal_fixed(Nanosecond3NoDot),
                        <span class="kw">_ </span>=&gt; Item::Error,
                    },
                    <span class="string">'6' </span>=&gt; <span class="kw">match </span><span class="macro">next!</span>() {
                        <span class="string">'f' </span>=&gt; internal_fixed(Nanosecond6NoDot),
                        <span class="kw">_ </span>=&gt; Item::Error,
                    },
                    <span class="string">'9' </span>=&gt; <span class="kw">match </span><span class="macro">next!</span>() {
                        <span class="string">'f' </span>=&gt; internal_fixed(Nanosecond9NoDot),
                        <span class="kw">_ </span>=&gt; Item::Error,
                    },
                    <span class="string">'%' </span>=&gt; Literal(<span class="string">"%"</span>),
                    <span class="kw">_ </span>=&gt; Item::Error, <span class="comment">// no such specifier
                </span>};

                <span class="comment">// Adjust `item` if we have any padding modifier.
                // Not allowed on non-numeric items or on specifiers composed out of multiple
                // formatting items.
                </span><span class="kw">if let </span><span class="prelude-val">Some</span>(new_pad) = pad_override {
                    <span class="kw">match </span>item {
                        Item::Numeric(<span class="kw-2">ref </span>kind, _pad) <span class="kw">if </span><span class="self">self</span>.queue.is_empty() =&gt; {
                            <span class="prelude-val">Some</span>((remainder, Item::Numeric(kind.clone(), new_pad)))
                        }
                        <span class="kw">_ </span>=&gt; <span class="prelude-val">Some</span>((remainder, Item::Error)),
                    }
                } <span class="kw">else </span>{
                    <span class="prelude-val">Some</span>((remainder, item))
                }
            }

            <span class="comment">// the next item is space
            </span><span class="prelude-val">Some</span>(c) <span class="kw">if </span>c.is_whitespace() =&gt; {
                <span class="comment">// `%` is not a whitespace, so `c != '%'` is redundant
                </span><span class="kw">let </span>nextspec =
                    remainder.find(|c: char| !c.is_whitespace()).unwrap_or(remainder.len());
                <span class="macro">assert!</span>(nextspec &gt; <span class="number">0</span>);
                <span class="kw">let </span>item = Space(<span class="kw-2">&amp;</span>remainder[..nextspec]);
                remainder = <span class="kw-2">&amp;</span>remainder[nextspec..];
                <span class="prelude-val">Some</span>((remainder, item))
            }

            <span class="comment">// the next item is literal
            </span><span class="kw">_ </span>=&gt; {
                <span class="kw">let </span>nextspec = remainder
                    .find(|c: char| c.is_whitespace() || c == <span class="string">'%'</span>)
                    .unwrap_or(remainder.len());
                <span class="macro">assert!</span>(nextspec &gt; <span class="number">0</span>);
                <span class="kw">let </span>item = Literal(<span class="kw-2">&amp;</span>remainder[..nextspec]);
                remainder = <span class="kw-2">&amp;</span>remainder[nextspec..];
                <span class="prelude-val">Some</span>((remainder, item))
            }
        }
    }

    <span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
    </span><span class="kw">fn </span>switch_to_locale_str(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        localized_fmt_str: <span class="kw">impl </span>Fn(Locale) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str,
        fallback: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[Item&lt;<span class="lifetime">'static</span>&gt;],
    ) -&gt; Item&lt;<span class="lifetime">'a</span>&gt; {
        <span class="kw">if let </span><span class="prelude-val">Some</span>(locale) = <span class="self">self</span>.locale {
            <span class="macro">assert!</span>(<span class="self">self</span>.locale_str.is_empty());
            <span class="kw">let </span>(fmt_str, item) = <span class="self">self</span>.parse_next_item(localized_fmt_str(locale)).unwrap();
            <span class="self">self</span>.locale_str = fmt_str;
            item
        } <span class="kw">else </span>{
            <span class="self">self</span>.queue = <span class="kw-2">&amp;</span>fallback[<span class="number">1</span>..];
            fallback[<span class="number">0</span>].clone()
        }
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span><span class="kw">super</span>::StrftimeItems;
    <span class="kw">use </span><span class="kw">crate</span>::format::Item::{<span class="self">self</span>, Literal, Space};
    <span class="attr">#[cfg(feature = <span class="string">"unstable-locales"</span>)]
    </span><span class="kw">use </span><span class="kw">crate</span>::format::Locale;
    <span class="kw">use </span><span class="kw">crate</span>::format::{fixed, internal_fixed, num, num0, nums};
    <span class="kw">use </span><span class="kw">crate</span>::format::{Fixed, InternalInternal, Numeric::<span class="kw-2">*</span>};
    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">use crate</span>::{DateTime, FixedOffset, NaiveDate, TimeZone, Timelike, Utc};

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_strftime_items() {
        <span class="kw">fn </span>parse_and_collect(s: <span class="kw-2">&amp;</span>str) -&gt; Vec&lt;Item&lt;<span class="lifetime">'_</span>&gt;&gt; {
            <span class="comment">// map any error into `[Item::Error]`. useful for easy testing.
            </span><span class="macro">eprintln!</span>(<span class="string">"test_strftime_items: parse_and_collect({:?})"</span>, s);
            <span class="kw">let </span>items = StrftimeItems::new(s);
            <span class="kw">let </span>items = items.map(|spec| <span class="kw">if </span>spec == Item::Error { <span class="prelude-val">None </span>} <span class="kw">else </span>{ <span class="prelude-val">Some</span>(spec) });
            items.collect::&lt;<span class="prelude-ty">Option</span>&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;&gt;().unwrap_or_else(|| <span class="macro">vec!</span>[Item::Error])
        }

        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">""</span>), []);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">" "</span>), [Space(<span class="string">" "</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"  "</span>), [Space(<span class="string">"  "</span>)]);
        <span class="comment">// ne!
        </span><span class="macro">assert_ne!</span>(parse_and_collect(<span class="string">"  "</span>), [Space(<span class="string">" "</span>), Space(<span class="string">" "</span>)]);
        <span class="comment">// eq!
        </span><span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"  "</span>), [Space(<span class="string">"  "</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"a"</span>), [Literal(<span class="string">"a"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"ab"</span>), [Literal(<span class="string">"ab"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽"</span>), [Literal(<span class="string">"😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"a😽"</span>), [Literal(<span class="string">"a😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽a"</span>), [Literal(<span class="string">"😽a"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">" 😽"</span>), [Space(<span class="string">" "</span>), Literal(<span class="string">"😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽 "</span>), [Literal(<span class="string">"😽"</span>), Space(<span class="string">" "</span>)]);
        <span class="comment">// ne!
        </span><span class="macro">assert_ne!</span>(parse_and_collect(<span class="string">"😽😽"</span>), [Literal(<span class="string">"😽"</span>)]);
        <span class="macro">assert_ne!</span>(parse_and_collect(<span class="string">"😽"</span>), [Literal(<span class="string">"😽😽"</span>)]);
        <span class="macro">assert_ne!</span>(parse_and_collect(<span class="string">"😽😽"</span>), [Literal(<span class="string">"😽😽"</span>), Literal(<span class="string">"😽"</span>)]);
        <span class="comment">// eq!
        </span><span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽😽"</span>), [Literal(<span class="string">"😽😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">" \t\n\r "</span>), [Space(<span class="string">" \t\n\r "</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"hello?"</span>), [Literal(<span class="string">"hello?"</span>)]);
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"a  b\t\nc"</span>),
            [Literal(<span class="string">"a"</span>), Space(<span class="string">"  "</span>), Literal(<span class="string">"b"</span>), Space(<span class="string">"\t\n"</span>), Literal(<span class="string">"c"</span>)]
        );
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"100%%"</span>), [Literal(<span class="string">"100"</span>), Literal(<span class="string">"%"</span>)]);
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"100%% ok"</span>),
            [Literal(<span class="string">"100"</span>), Literal(<span class="string">"%"</span>), Space(<span class="string">" "</span>), Literal(<span class="string">"ok"</span>)]
        );
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%%PDF-1.0"</span>), [Literal(<span class="string">"%"</span>), Literal(<span class="string">"PDF-1.0"</span>)]);
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"%Y-%m-%d"</span>),
            [num0(Year), Literal(<span class="string">"-"</span>), num0(Month), Literal(<span class="string">"-"</span>), num0(Day)]
        );
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽   "</span>), [Literal(<span class="string">"😽"</span>), Space(<span class="string">"   "</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽😽"</span>), [Literal(<span class="string">"😽😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽😽😽"</span>), [Literal(<span class="string">"😽😽😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽😽 😽"</span>), [Literal(<span class="string">"😽😽"</span>), Space(<span class="string">" "</span>), Literal(<span class="string">"😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽😽a 😽"</span>), [Literal(<span class="string">"😽😽a"</span>), Space(<span class="string">" "</span>), Literal(<span class="string">"😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽😽a b😽"</span>), [Literal(<span class="string">"😽😽a"</span>), Space(<span class="string">" "</span>), Literal(<span class="string">"b😽"</span>)]);
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"😽😽a b😽c"</span>),
            [Literal(<span class="string">"😽😽a"</span>), Space(<span class="string">" "</span>), Literal(<span class="string">"b😽c"</span>)]
        );
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽😽   "</span>), [Literal(<span class="string">"😽😽"</span>), Space(<span class="string">"   "</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽😽   😽"</span>), [Literal(<span class="string">"😽😽"</span>), Space(<span class="string">"   "</span>), Literal(<span class="string">"😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"   😽"</span>), [Space(<span class="string">"   "</span>), Literal(<span class="string">"😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"   😽 "</span>), [Space(<span class="string">"   "</span>), Literal(<span class="string">"😽"</span>), Space(<span class="string">" "</span>)]);
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"   😽 😽"</span>),
            [Space(<span class="string">"   "</span>), Literal(<span class="string">"😽"</span>), Space(<span class="string">" "</span>), Literal(<span class="string">"😽"</span>)]
        );
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"   😽 😽 "</span>),
            [Space(<span class="string">"   "</span>), Literal(<span class="string">"😽"</span>), Space(<span class="string">" "</span>), Literal(<span class="string">"😽"</span>), Space(<span class="string">" "</span>)]
        );
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"   😽  😽 "</span>),
            [Space(<span class="string">"   "</span>), Literal(<span class="string">"😽"</span>), Space(<span class="string">"  "</span>), Literal(<span class="string">"😽"</span>), Space(<span class="string">" "</span>)]
        );
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"   😽  😽😽 "</span>),
            [Space(<span class="string">"   "</span>), Literal(<span class="string">"😽"</span>), Space(<span class="string">"  "</span>), Literal(<span class="string">"😽😽"</span>), Space(<span class="string">" "</span>)]
        );
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"   😽😽"</span>), [Space(<span class="string">"   "</span>), Literal(<span class="string">"😽😽"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"   😽😽 "</span>), [Space(<span class="string">"   "</span>), Literal(<span class="string">"😽😽"</span>), Space(<span class="string">" "</span>)]);
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"   😽😽    "</span>),
            [Space(<span class="string">"   "</span>), Literal(<span class="string">"😽😽"</span>), Space(<span class="string">"    "</span>)]
        );
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"   😽😽    "</span>),
            [Space(<span class="string">"   "</span>), Literal(<span class="string">"😽😽"</span>), Space(<span class="string">"    "</span>)]
        );
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">" 😽😽    "</span>), [Space(<span class="string">" "</span>), Literal(<span class="string">"😽😽"</span>), Space(<span class="string">"    "</span>)]);
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">" 😽 😽😽    "</span>),
            [Space(<span class="string">" "</span>), Literal(<span class="string">"😽"</span>), Space(<span class="string">" "</span>), Literal(<span class="string">"😽😽"</span>), Space(<span class="string">"    "</span>)]
        );
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">" 😽 😽はい😽    ハンバーガー"</span>),
            [
                Space(<span class="string">" "</span>),
                Literal(<span class="string">"😽"</span>),
                Space(<span class="string">" "</span>),
                Literal(<span class="string">"😽はい😽"</span>),
                Space(<span class="string">"    "</span>),
                Literal(<span class="string">"ハンバーガー"</span>)
            ]
        );
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"%%😽%%😽"</span>),
            [Literal(<span class="string">"%"</span>), Literal(<span class="string">"😽"</span>), Literal(<span class="string">"%"</span>), Literal(<span class="string">"😽"</span>)]
        );
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%Y--%m"</span>), [num0(Year), Literal(<span class="string">"--"</span>), num0(Month)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"[%F]"</span>), parse_and_collect(<span class="string">"[%Y-%m-%d]"</span>));
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"100%%😽"</span>), [Literal(<span class="string">"100"</span>), Literal(<span class="string">"%"</span>), Literal(<span class="string">"😽"</span>)]);
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"100%%😽%%a"</span>),
            [Literal(<span class="string">"100"</span>), Literal(<span class="string">"%"</span>), Literal(<span class="string">"😽"</span>), Literal(<span class="string">"%"</span>), Literal(<span class="string">"a"</span>)]
        );
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"😽100%%"</span>), [Literal(<span class="string">"😽100"</span>), Literal(<span class="string">"%"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%m %d"</span>), [num0(Month), Space(<span class="string">" "</span>), num0(Day)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%%"</span>), [Literal(<span class="string">"%"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%%%"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%a"</span>), [fixed(Fixed::ShortWeekdayName)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%aa"</span>), [fixed(Fixed::ShortWeekdayName), Literal(<span class="string">"a"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%%a%"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%😽"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%😽😽"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%%%%"</span>), [Literal(<span class="string">"%"</span>), Literal(<span class="string">"%"</span>)]);
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"%%%%ハンバーガー"</span>),
            [Literal(<span class="string">"%"</span>), Literal(<span class="string">"%"</span>), Literal(<span class="string">"ハンバーガー"</span>)]
        );
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"foo%?"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"bar%42"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"quux% +"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%.Z"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%:Z"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%-Z"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%0Z"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%_Z"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%.j"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%:j"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%-j"</span>), [num(Ordinal)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%0j"</span>), [num0(Ordinal)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%_j"</span>), [nums(Ordinal)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%.e"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%:e"</span>), [Item::Error]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%-e"</span>), [num(Day)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%0e"</span>), [num0(Day)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%_e"</span>), [nums(Day)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%z"</span>), [fixed(Fixed::TimezoneOffset)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%:z"</span>), [fixed(Fixed::TimezoneOffsetColon)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%Z"</span>), [fixed(Fixed::TimezoneName)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%ZZZZ"</span>), [fixed(Fixed::TimezoneName), Literal(<span class="string">"ZZZ"</span>)]);
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%Z😽"</span>), [fixed(Fixed::TimezoneName), Literal(<span class="string">"😽"</span>)]);
        <span class="macro">assert_eq!</span>(
            parse_and_collect(<span class="string">"%#z"</span>),
            [internal_fixed(InternalInternal::TimezoneOffsetPermissive)]
        );
        <span class="macro">assert_eq!</span>(parse_and_collect(<span class="string">"%#m"</span>), [Item::Error]);
    }

    <span class="attr">#[test]
    #[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>test_strftime_docs() {
        <span class="kw">let </span>dt = FixedOffset::east_opt(<span class="number">34200</span>)
            .unwrap()
            .from_local_datetime(
                <span class="kw-2">&amp;</span>NaiveDate::from_ymd_opt(<span class="number">2001</span>, <span class="number">7</span>, <span class="number">8</span>)
                    .unwrap()
                    .and_hms_nano_opt(<span class="number">0</span>, <span class="number">34</span>, <span class="number">59</span>, <span class="number">1_026_490_708</span>)
                    .unwrap(),
            )
            .unwrap();

        <span class="comment">// date specifiers
        </span><span class="macro">assert_eq!</span>(dt.format(<span class="string">"%Y"</span>).to_string(), <span class="string">"2001"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%C"</span>).to_string(), <span class="string">"20"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%y"</span>).to_string(), <span class="string">"01"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%m"</span>).to_string(), <span class="string">"07"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%b"</span>).to_string(), <span class="string">"Jul"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%B"</span>).to_string(), <span class="string">"July"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%h"</span>).to_string(), <span class="string">"Jul"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%d"</span>).to_string(), <span class="string">"08"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%e"</span>).to_string(), <span class="string">" 8"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%e"</span>).to_string(), dt.format(<span class="string">"%_d"</span>).to_string());
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%a"</span>).to_string(), <span class="string">"Sun"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%A"</span>).to_string(), <span class="string">"Sunday"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%w"</span>).to_string(), <span class="string">"0"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%u"</span>).to_string(), <span class="string">"7"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%U"</span>).to_string(), <span class="string">"27"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%W"</span>).to_string(), <span class="string">"27"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%G"</span>).to_string(), <span class="string">"2001"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%g"</span>).to_string(), <span class="string">"01"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%V"</span>).to_string(), <span class="string">"27"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%j"</span>).to_string(), <span class="string">"189"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%D"</span>).to_string(), <span class="string">"07/08/01"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%x"</span>).to_string(), <span class="string">"07/08/01"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%F"</span>).to_string(), <span class="string">"2001-07-08"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%v"</span>).to_string(), <span class="string">" 8-Jul-2001"</span>);

        <span class="comment">// time specifiers
        </span><span class="macro">assert_eq!</span>(dt.format(<span class="string">"%H"</span>).to_string(), <span class="string">"00"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%k"</span>).to_string(), <span class="string">" 0"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%k"</span>).to_string(), dt.format(<span class="string">"%_H"</span>).to_string());
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%I"</span>).to_string(), <span class="string">"12"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%l"</span>).to_string(), <span class="string">"12"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%l"</span>).to_string(), dt.format(<span class="string">"%_I"</span>).to_string());
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%P"</span>).to_string(), <span class="string">"am"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%p"</span>).to_string(), <span class="string">"AM"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%M"</span>).to_string(), <span class="string">"34"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%S"</span>).to_string(), <span class="string">"60"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%f"</span>).to_string(), <span class="string">"026490708"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%.f"</span>).to_string(), <span class="string">".026490708"</span>);
        <span class="macro">assert_eq!</span>(dt.with_nanosecond(<span class="number">1_026_490_000</span>).unwrap().format(<span class="string">"%.f"</span>).to_string(), <span class="string">".026490"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%.3f"</span>).to_string(), <span class="string">".026"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%.6f"</span>).to_string(), <span class="string">".026490"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%.9f"</span>).to_string(), <span class="string">".026490708"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%3f"</span>).to_string(), <span class="string">"026"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%6f"</span>).to_string(), <span class="string">"026490"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%9f"</span>).to_string(), <span class="string">"026490708"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%R"</span>).to_string(), <span class="string">"00:34"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%T"</span>).to_string(), <span class="string">"00:34:60"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%X"</span>).to_string(), <span class="string">"00:34:60"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%r"</span>).to_string(), <span class="string">"12:34:60 AM"</span>);

        <span class="comment">// time zone specifiers
        //assert_eq!(dt.format("%Z").to_string(), "ACST");
        </span><span class="macro">assert_eq!</span>(dt.format(<span class="string">"%z"</span>).to_string(), <span class="string">"+0930"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%:z"</span>).to_string(), <span class="string">"+09:30"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%::z"</span>).to_string(), <span class="string">"+09:30:00"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%:::z"</span>).to_string(), <span class="string">"+09"</span>);

        <span class="comment">// date &amp; time specifiers
        </span><span class="macro">assert_eq!</span>(dt.format(<span class="string">"%c"</span>).to_string(), <span class="string">"Sun Jul  8 00:34:60 2001"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%+"</span>).to_string(), <span class="string">"2001-07-08T00:34:60.026490708+09:30"</span>);

        <span class="macro">assert_eq!</span>(
            dt.with_timezone(<span class="kw-2">&amp;</span>Utc).format(<span class="string">"%+"</span>).to_string(),
            <span class="string">"2001-07-07T15:04:60.026490708+00:00"
        </span>);
        <span class="macro">assert_eq!</span>(
            dt.with_timezone(<span class="kw-2">&amp;</span>Utc),
            DateTime::parse_from_str(<span class="string">"2001-07-07T15:04:60.026490708Z"</span>, <span class="string">"%+"</span>).unwrap()
        );
        <span class="macro">assert_eq!</span>(
            dt.with_timezone(<span class="kw-2">&amp;</span>Utc),
            DateTime::parse_from_str(<span class="string">"2001-07-07T15:04:60.026490708UTC"</span>, <span class="string">"%+"</span>).unwrap()
        );
        <span class="macro">assert_eq!</span>(
            dt.with_timezone(<span class="kw-2">&amp;</span>Utc),
            DateTime::parse_from_str(<span class="string">"2001-07-07t15:04:60.026490708utc"</span>, <span class="string">"%+"</span>).unwrap()
        );

        <span class="macro">assert_eq!</span>(
            dt.with_nanosecond(<span class="number">1_026_490_000</span>).unwrap().format(<span class="string">"%+"</span>).to_string(),
            <span class="string">"2001-07-08T00:34:60.026490+09:30"
        </span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%s"</span>).to_string(), <span class="string">"994518299"</span>);

        <span class="comment">// special specifiers
        </span><span class="macro">assert_eq!</span>(dt.format(<span class="string">"%t"</span>).to_string(), <span class="string">"\t"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%n"</span>).to_string(), <span class="string">"\n"</span>);
        <span class="macro">assert_eq!</span>(dt.format(<span class="string">"%%"</span>).to_string(), <span class="string">"%"</span>);

        <span class="comment">// complex format specifiers
        </span><span class="macro">assert_eq!</span>(dt.format(<span class="string">"  %Y%d%m%%%%%t%H%M%S\t"</span>).to_string(), <span class="string">"  20010807%%\t003460\t"</span>);
        <span class="macro">assert_eq!</span>(
            dt.format(<span class="string">"  %Y%d%m%%%%%t%H:%P:%M%S%:::z\t"</span>).to_string(),
            <span class="string">"  20010807%%\t00:am:3460+09\t"
        </span>);
    }

    <span class="attr">#[test]
    #[cfg(all(feature = <span class="string">"unstable-locales"</span>, feature = <span class="string">"alloc"</span>))]
    </span><span class="kw">fn </span>test_strftime_docs_localized() {
        <span class="kw">let </span>dt = FixedOffset::east_opt(<span class="number">34200</span>)
            .unwrap()
            .with_ymd_and_hms(<span class="number">2001</span>, <span class="number">7</span>, <span class="number">8</span>, <span class="number">0</span>, <span class="number">34</span>, <span class="number">59</span>)
            .unwrap()
            .with_nanosecond(<span class="number">1_026_490_708</span>)
            .unwrap();

        <span class="comment">// date specifiers
        </span><span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%b"</span>, Locale::fr_BE).to_string(), <span class="string">"jui"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%B"</span>, Locale::fr_BE).to_string(), <span class="string">"juillet"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%h"</span>, Locale::fr_BE).to_string(), <span class="string">"jui"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%a"</span>, Locale::fr_BE).to_string(), <span class="string">"dim"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%A"</span>, Locale::fr_BE).to_string(), <span class="string">"dimanche"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%D"</span>, Locale::fr_BE).to_string(), <span class="string">"07/08/01"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%x"</span>, Locale::fr_BE).to_string(), <span class="string">"08/07/01"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%F"</span>, Locale::fr_BE).to_string(), <span class="string">"2001-07-08"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%v"</span>, Locale::fr_BE).to_string(), <span class="string">" 8-jui-2001"</span>);

        <span class="comment">// time specifiers
        </span><span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%P"</span>, Locale::fr_BE).to_string(), <span class="string">""</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%p"</span>, Locale::fr_BE).to_string(), <span class="string">""</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%R"</span>, Locale::fr_BE).to_string(), <span class="string">"00:34"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%T"</span>, Locale::fr_BE).to_string(), <span class="string">"00:34:60"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%X"</span>, Locale::fr_BE).to_string(), <span class="string">"00:34:60"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%r"</span>, Locale::fr_BE).to_string(), <span class="string">"00:34:60"</span>);

        <span class="comment">// date &amp; time specifiers
        </span><span class="macro">assert_eq!</span>(
            dt.format_localized(<span class="string">"%c"</span>, Locale::fr_BE).to_string(),
            <span class="string">"dim 08 jui 2001 00:34:60 +09:30"
        </span>);

        <span class="kw">let </span>nd = NaiveDate::from_ymd_opt(<span class="number">2001</span>, <span class="number">7</span>, <span class="number">8</span>).unwrap();

        <span class="comment">// date specifiers
        </span><span class="macro">assert_eq!</span>(nd.format_localized(<span class="string">"%b"</span>, Locale::de_DE).to_string(), <span class="string">"Jul"</span>);
        <span class="macro">assert_eq!</span>(nd.format_localized(<span class="string">"%B"</span>, Locale::de_DE).to_string(), <span class="string">"Juli"</span>);
        <span class="macro">assert_eq!</span>(nd.format_localized(<span class="string">"%h"</span>, Locale::de_DE).to_string(), <span class="string">"Jul"</span>);
        <span class="macro">assert_eq!</span>(nd.format_localized(<span class="string">"%a"</span>, Locale::de_DE).to_string(), <span class="string">"So"</span>);
        <span class="macro">assert_eq!</span>(nd.format_localized(<span class="string">"%A"</span>, Locale::de_DE).to_string(), <span class="string">"Sonntag"</span>);
        <span class="macro">assert_eq!</span>(nd.format_localized(<span class="string">"%D"</span>, Locale::de_DE).to_string(), <span class="string">"07/08/01"</span>);
        <span class="macro">assert_eq!</span>(nd.format_localized(<span class="string">"%x"</span>, Locale::de_DE).to_string(), <span class="string">"08.07.2001"</span>);
        <span class="macro">assert_eq!</span>(nd.format_localized(<span class="string">"%F"</span>, Locale::de_DE).to_string(), <span class="string">"2001-07-08"</span>);
        <span class="macro">assert_eq!</span>(nd.format_localized(<span class="string">"%v"</span>, Locale::de_DE).to_string(), <span class="string">" 8-Jul-2001"</span>);
    }

    <span class="doccomment">/// Ensure parsing a timestamp with the parse-only stftime formatter "%#z" does
    /// not cause a panic.
    ///
    /// See &lt;https://github.com/chronotope/chrono/issues/1139&gt;.
    </span><span class="attr">#[test]
    #[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>test_parse_only_timezone_offset_permissive_no_panic() {
        <span class="kw">use </span><span class="kw">crate</span>::NaiveDate;
        <span class="kw">use crate</span>::{FixedOffset, TimeZone};
        <span class="kw">use </span>std::fmt::Write;

        <span class="kw">let </span>dt = FixedOffset::east_opt(<span class="number">34200</span>)
            .unwrap()
            .from_local_datetime(
                <span class="kw-2">&amp;</span>NaiveDate::from_ymd_opt(<span class="number">2001</span>, <span class="number">7</span>, <span class="number">8</span>)
                    .unwrap()
                    .and_hms_nano_opt(<span class="number">0</span>, <span class="number">34</span>, <span class="number">59</span>, <span class="number">1_026_490_708</span>)
                    .unwrap(),
            )
            .unwrap();

        <span class="kw">let </span><span class="kw-2">mut </span>buf = String::new();
        <span class="kw">let _ </span>= <span class="macro">write!</span>(buf, <span class="string">"{}"</span>, dt.format(<span class="string">"%#z"</span>)).expect_err(<span class="string">"parse-only formatter should fail"</span>);
    }

    <span class="attr">#[test]
    #[cfg(all(feature = <span class="string">"unstable-locales"</span>, feature = <span class="string">"alloc"</span>))]
    </span><span class="kw">fn </span>test_strftime_localized_korean() {
        <span class="kw">let </span>dt = FixedOffset::east_opt(<span class="number">34200</span>)
            .unwrap()
            .with_ymd_and_hms(<span class="number">2001</span>, <span class="number">7</span>, <span class="number">8</span>, <span class="number">0</span>, <span class="number">34</span>, <span class="number">59</span>)
            .unwrap()
            .with_nanosecond(<span class="number">1_026_490_708</span>)
            .unwrap();

        <span class="comment">// date specifiers
        </span><span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%b"</span>, Locale::ko_KR).to_string(), <span class="string">" 7월"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%B"</span>, Locale::ko_KR).to_string(), <span class="string">"7월"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%h"</span>, Locale::ko_KR).to_string(), <span class="string">" 7월"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%a"</span>, Locale::ko_KR).to_string(), <span class="string">"일"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%A"</span>, Locale::ko_KR).to_string(), <span class="string">"일요일"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%D"</span>, Locale::ko_KR).to_string(), <span class="string">"07/08/01"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%x"</span>, Locale::ko_KR).to_string(), <span class="string">"2001년 07월 08일"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%F"</span>, Locale::ko_KR).to_string(), <span class="string">"2001-07-08"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%v"</span>, Locale::ko_KR).to_string(), <span class="string">" 8- 7월-2001"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%r"</span>, Locale::ko_KR).to_string(), <span class="string">"오전 12시 34분 60초"</span>);

        <span class="comment">// date &amp; time specifiers
        </span><span class="macro">assert_eq!</span>(
            dt.format_localized(<span class="string">"%c"</span>, Locale::ko_KR).to_string(),
            <span class="string">"2001년 07월 08일 (일) 오전 12시 34분 60초"
        </span>);
    }

    <span class="attr">#[test]
    #[cfg(all(feature = <span class="string">"unstable-locales"</span>, feature = <span class="string">"alloc"</span>))]
    </span><span class="kw">fn </span>test_strftime_localized_japanese() {
        <span class="kw">let </span>dt = FixedOffset::east_opt(<span class="number">34200</span>)
            .unwrap()
            .with_ymd_and_hms(<span class="number">2001</span>, <span class="number">7</span>, <span class="number">8</span>, <span class="number">0</span>, <span class="number">34</span>, <span class="number">59</span>)
            .unwrap()
            .with_nanosecond(<span class="number">1_026_490_708</span>)
            .unwrap();

        <span class="comment">// date specifiers
        </span><span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%b"</span>, Locale::ja_JP).to_string(), <span class="string">" 7月"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%B"</span>, Locale::ja_JP).to_string(), <span class="string">"7月"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%h"</span>, Locale::ja_JP).to_string(), <span class="string">" 7月"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%a"</span>, Locale::ja_JP).to_string(), <span class="string">"日"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%A"</span>, Locale::ja_JP).to_string(), <span class="string">"日曜日"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%D"</span>, Locale::ja_JP).to_string(), <span class="string">"07/08/01"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%x"</span>, Locale::ja_JP).to_string(), <span class="string">"2001年07月08日"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%F"</span>, Locale::ja_JP).to_string(), <span class="string">"2001-07-08"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%v"</span>, Locale::ja_JP).to_string(), <span class="string">" 8- 7月-2001"</span>);
        <span class="macro">assert_eq!</span>(dt.format_localized(<span class="string">"%r"</span>, Locale::ja_JP).to_string(), <span class="string">"午前12時34分60秒"</span>);

        <span class="comment">// date &amp; time specifiers
        </span><span class="macro">assert_eq!</span>(
            dt.format_localized(<span class="string">"%c"</span>, Locale::ja_JP).to_string(),
            <span class="string">"2001年07月08日 00時34分60秒"
        </span>);
    }

    <span class="attr">#[test]
    #[cfg(all(feature = <span class="string">"unstable-locales"</span>, feature = <span class="string">"alloc"</span>))]
    </span><span class="kw">fn </span>test_strftime_localized_time() {
        <span class="kw">let </span>dt1 = Utc.with_ymd_and_hms(<span class="number">2024</span>, <span class="number">2</span>, <span class="number">9</span>, <span class="number">6</span>, <span class="number">54</span>, <span class="number">32</span>).unwrap();
        <span class="kw">let </span>dt2 = Utc.with_ymd_and_hms(<span class="number">2024</span>, <span class="number">2</span>, <span class="number">9</span>, <span class="number">18</span>, <span class="number">54</span>, <span class="number">32</span>).unwrap();
        <span class="comment">// Some of these locales gave issues before pure-rust-locales 0.8.0 with chrono 0.4.27+
        </span><span class="macro">assert_eq!</span>(dt1.format_localized(<span class="string">"%X"</span>, Locale::nl_NL).to_string(), <span class="string">"06:54:32"</span>);
        <span class="macro">assert_eq!</span>(dt2.format_localized(<span class="string">"%X"</span>, Locale::nl_NL).to_string(), <span class="string">"18:54:32"</span>);
        <span class="macro">assert_eq!</span>(dt1.format_localized(<span class="string">"%X"</span>, Locale::en_US).to_string(), <span class="string">"06:54:32 AM"</span>);
        <span class="macro">assert_eq!</span>(dt2.format_localized(<span class="string">"%X"</span>, Locale::en_US).to_string(), <span class="string">"06:54:32 PM"</span>);
        <span class="macro">assert_eq!</span>(dt1.format_localized(<span class="string">"%X"</span>, Locale::hy_AM).to_string(), <span class="string">"06:54:32"</span>);
        <span class="macro">assert_eq!</span>(dt2.format_localized(<span class="string">"%X"</span>, Locale::hy_AM).to_string(), <span class="string">"18:54:32"</span>);
        <span class="macro">assert_eq!</span>(dt1.format_localized(<span class="string">"%X"</span>, Locale::chr_US).to_string(), <span class="string">"06:54:32 ᏌᎾᎴ"</span>);
        <span class="macro">assert_eq!</span>(dt2.format_localized(<span class="string">"%X"</span>, Locale::chr_US).to_string(), <span class="string">"06:54:32 ᏒᎯᏱᎢᏗᏢ"</span>);
    }

    <span class="attr">#[test]
    #[cfg(all(feature = <span class="string">"unstable-locales"</span>, target_pointer_width = <span class="string">"64"</span>))]
    </span><span class="kw">fn </span>test_type_sizes() {
        <span class="kw">use </span>core::mem::size_of;
        <span class="macro">assert_eq!</span>(size_of::&lt;Item&gt;(), <span class="number">24</span>);
        <span class="macro">assert_eq!</span>(size_of::&lt;StrftimeItems&gt;(), <span class="number">56</span>);
        <span class="macro">assert_eq!</span>(size_of::&lt;Locale&gt;(), <span class="number">2</span>);
    }

    <span class="attr">#[test]
    #[cfg(all(feature = <span class="string">"unstable-locales"</span>, target_pointer_width = <span class="string">"32"</span>))]
    </span><span class="kw">fn </span>test_type_sizes() {
        <span class="kw">use </span>core::mem::size_of;
        <span class="macro">assert_eq!</span>(size_of::&lt;Item&gt;(), <span class="number">12</span>);
        <span class="macro">assert_eq!</span>(size_of::&lt;StrftimeItems&gt;(), <span class="number">28</span>);
        <span class="macro">assert_eq!</span>(size_of::&lt;Locale&gt;(), <span class="number">2</span>);
    }

    <span class="attr">#[test]
    #[cfg(any(feature = <span class="string">"alloc"</span>, feature = <span class="string">"std"</span>))]
    </span><span class="kw">fn </span>test_strftime_parse() {
        <span class="kw">let </span>fmt_str = StrftimeItems::new(<span class="string">"%Y-%m-%dT%H:%M:%S%z"</span>);
        <span class="kw">let </span>fmt_items = fmt_str.parse().unwrap();
        <span class="kw">let </span>dt = Utc.with_ymd_and_hms(<span class="number">2014</span>, <span class="number">5</span>, <span class="number">7</span>, <span class="number">12</span>, <span class="number">34</span>, <span class="number">56</span>).unwrap();
        <span class="macro">assert_eq!</span>(<span class="kw-2">&amp;</span>dt.format_with_items(fmt_items.iter()).to_string(), <span class="string">"2014-05-07T12:34:56+0000"</span>);
    }
}
</code></pre></div></section></main></body></html>