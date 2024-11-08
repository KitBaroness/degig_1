<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ring-0.17.8/src/limb.rs`."><title>limb.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="ring" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../ring/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2016 David Judd.
// Copyright 2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

</span><span class="doccomment">//! Unsigned multi-precision integer arithmetic.
//!
//! Limbs ordered least-significant-limb to most-significant-limb. The bits
//! limbs use the native endianness.

</span><span class="kw">use crate</span>::{c, error, polyfill::ArrayFlatMap};

<span class="attr">#[cfg(any(test, feature = <span class="string">"alloc"</span>))]
</span><span class="kw">use </span><span class="kw">crate</span>::bits;

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">use </span>core::num::Wrapping;

<span class="comment">// XXX: Not correct for x32 ABIs.
</span><span class="attr">#[cfg(target_pointer_width = <span class="string">"64"</span>)]
</span><span class="kw">pub type </span>Limb = u64;
<span class="attr">#[cfg(target_pointer_width = <span class="string">"32"</span>)]
</span><span class="kw">pub type </span>Limb = u32;
<span class="attr">#[cfg(target_pointer_width = <span class="string">"64"</span>)]
</span><span class="kw">pub const </span>LIMB_BITS: usize = <span class="number">64</span>;
<span class="attr">#[cfg(target_pointer_width = <span class="string">"32"</span>)]
</span><span class="kw">pub const </span>LIMB_BITS: usize = <span class="number">32</span>;

<span class="attr">#[cfg(target_pointer_width = <span class="string">"64"</span>)]
#[derive(Debug, PartialEq)]
#[repr(u64)]
</span><span class="kw">pub enum </span>LimbMask {
    True = <span class="number">0xffff_ffff_ffff_ffff</span>,
    False = <span class="number">0</span>,
}

<span class="attr">#[cfg(target_pointer_width = <span class="string">"32"</span>)]
#[derive(Debug, PartialEq)]
#[repr(u32)]
</span><span class="kw">pub enum </span>LimbMask {
    True = <span class="number">0xffff_ffff</span>,
    False = <span class="number">0</span>,
}

<span class="kw">pub const </span>LIMB_BYTES: usize = (LIMB_BITS + <span class="number">7</span>) / <span class="number">8</span>;

<span class="attr">#[inline]
</span><span class="kw">pub fn </span>limbs_equal_limbs_consttime(a: <span class="kw-2">&amp;</span>[Limb], b: <span class="kw-2">&amp;</span>[Limb]) -&gt; LimbMask {
    <span class="macro">prefixed_extern!</span> {
        <span class="kw">fn </span>LIMBS_equal(a: <span class="kw-2">*const </span>Limb, b: <span class="kw-2">*const </span>Limb, num_limbs: c::size_t) -&gt; LimbMask;
    }

    <span class="macro">assert_eq!</span>(a.len(), b.len());
    <span class="kw">unsafe </span>{ LIMBS_equal(a.as_ptr(), b.as_ptr(), a.len()) }
}

<span class="attr">#[inline]
</span><span class="kw">pub fn </span>limbs_less_than_limbs_consttime(a: <span class="kw-2">&amp;</span>[Limb], b: <span class="kw-2">&amp;</span>[Limb]) -&gt; LimbMask {
    <span class="macro">assert_eq!</span>(a.len(), b.len());
    <span class="kw">unsafe </span>{ LIMBS_less_than(a.as_ptr(), b.as_ptr(), b.len()) }
}

<span class="attr">#[inline]
</span><span class="kw">pub fn </span>limbs_less_than_limbs_vartime(a: <span class="kw-2">&amp;</span>[Limb], b: <span class="kw-2">&amp;</span>[Limb]) -&gt; bool {
    limbs_less_than_limbs_consttime(a, b) == LimbMask::True
}

<span class="attr">#[inline]
#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">pub fn </span>limbs_less_than_limb_constant_time(a: <span class="kw-2">&amp;</span>[Limb], b: Limb) -&gt; LimbMask {
    <span class="kw">unsafe </span>{ LIMBS_less_than_limb(a.as_ptr(), b, a.len()) }
}

<span class="attr">#[inline]
</span><span class="kw">pub fn </span>limbs_are_zero_constant_time(limbs: <span class="kw-2">&amp;</span>[Limb]) -&gt; LimbMask {
    <span class="kw">unsafe </span>{ LIMBS_are_zero(limbs.as_ptr(), limbs.len()) }
}

<span class="attr">#[cfg(any(test, feature = <span class="string">"alloc"</span>))]
#[inline]
</span><span class="kw">pub fn </span>limbs_are_even_constant_time(limbs: <span class="kw-2">&amp;</span>[Limb]) -&gt; LimbMask {
    <span class="kw">unsafe </span>{ LIMBS_are_even(limbs.as_ptr(), limbs.len()) }
}

<span class="attr">#[cfg(any(test, feature = <span class="string">"alloc"</span>))]
#[inline]
</span><span class="kw">pub fn </span>limbs_equal_limb_constant_time(a: <span class="kw-2">&amp;</span>[Limb], b: Limb) -&gt; LimbMask {
    <span class="kw">unsafe </span>{ LIMBS_equal_limb(a.as_ptr(), b, a.len()) }
}

<span class="doccomment">/// Returns the number of bits in `a`.
</span><span class="comment">//
// This strives to be constant-time with respect to the values of all bits
// except the most significant bit. This does not attempt to be constant-time
// with respect to `a.len()` or the value of the result or the value of the
// most significant bit (It's 1, unless the input is zero, in which case it's
// zero.)
</span><span class="attr">#[cfg(any(test, feature = <span class="string">"alloc"</span>))]
</span><span class="kw">pub fn </span>limbs_minimal_bits(a: <span class="kw-2">&amp;</span>[Limb]) -&gt; bits::BitLength {
    <span class="kw">for </span>num_limbs <span class="kw">in </span>(<span class="number">1</span>..=a.len()).rev() {
        <span class="kw">let </span>high_limb = a[num_limbs - <span class="number">1</span>];

        <span class="comment">// Find the number of set bits in |high_limb| by a linear scan from the
        // most significant bit to the least significant bit. This works great
        // for the most common inputs because usually the most significant bit
        // it set.
        </span><span class="kw">for </span>high_limb_num_bits <span class="kw">in </span>(<span class="number">1</span>..=LIMB_BITS).rev() {
            <span class="kw">let </span>shifted = <span class="kw">unsafe </span>{ LIMB_shr(high_limb, high_limb_num_bits - <span class="number">1</span>) };
            <span class="kw">if </span>shifted != <span class="number">0 </span>{
                <span class="kw">return </span>bits::BitLength::from_usize_bits(
                    ((num_limbs - <span class="number">1</span>) * LIMB_BITS) + high_limb_num_bits,
                );
            }
        }
    }

    <span class="comment">// No bits were set.
    </span>bits::BitLength::from_usize_bits(<span class="number">0</span>)
}

<span class="doccomment">/// Equivalent to `if (r &gt;= m) { r -= m; }`
</span><span class="attr">#[inline]
</span><span class="kw">pub fn </span>limbs_reduce_once_constant_time(r: <span class="kw-2">&amp;mut </span>[Limb], m: <span class="kw-2">&amp;</span>[Limb]) {
    <span class="macro">assert_eq!</span>(r.len(), m.len());
    <span class="kw">unsafe </span>{ LIMBS_reduce_once(r.as_mut_ptr(), m.as_ptr(), m.len()) };
}

<span class="attr">#[derive(Clone, Copy, PartialEq)]
</span><span class="kw">pub enum </span>AllowZero {
    No,
    Yes,
}

<span class="doccomment">/// Parses `input` into `result`, verifies that the value is less than
/// `max_exclusive`, and pads `result` with zeros to its length. If `allow_zero`
/// is not `AllowZero::Yes`, zero values are rejected.
///
/// This attempts to be constant-time with respect to the actual value *only if*
/// the value is actually in range. In other words, this won't leak anything
/// about a valid value, but it might leak small amounts of information about an
/// invalid value (which constraint it failed).
</span><span class="kw">pub fn </span>parse_big_endian_in_range_and_pad_consttime(
    input: untrusted::Input,
    allow_zero: AllowZero,
    max_exclusive: <span class="kw-2">&amp;</span>[Limb],
    result: <span class="kw-2">&amp;mut </span>[Limb],
) -&gt; <span class="prelude-ty">Result</span>&lt;(), error::Unspecified&gt; {
    parse_big_endian_and_pad_consttime(input, result)<span class="question-mark">?</span>;
    <span class="kw">if </span>limbs_less_than_limbs_consttime(result, max_exclusive) != LimbMask::True {
        <span class="kw">return </span><span class="prelude-val">Err</span>(error::Unspecified);
    }
    <span class="kw">if </span>allow_zero != AllowZero::Yes {
        <span class="kw">if </span>limbs_are_zero_constant_time(result) != LimbMask::False {
            <span class="kw">return </span><span class="prelude-val">Err</span>(error::Unspecified);
        }
    }
    <span class="prelude-val">Ok</span>(())
}

<span class="doccomment">/// Parses `input` into `result`, padding `result` with zeros to its length.
/// This attempts to be constant-time with respect to the value but not with
/// respect to the length; it is assumed that the length is public knowledge.
</span><span class="kw">pub fn </span>parse_big_endian_and_pad_consttime(
    input: untrusted::Input,
    result: <span class="kw-2">&amp;mut </span>[Limb],
) -&gt; <span class="prelude-ty">Result</span>&lt;(), error::Unspecified&gt; {
    <span class="kw">if </span>input.is_empty() {
        <span class="kw">return </span><span class="prelude-val">Err</span>(error::Unspecified);
    }

    <span class="comment">// `bytes_in_current_limb` is the number of bytes in the current limb.
    // It will be `LIMB_BYTES` for all limbs except maybe the highest-order
    // limb.
    </span><span class="kw">let </span><span class="kw-2">mut </span>bytes_in_current_limb = input.len() % LIMB_BYTES;
    <span class="kw">if </span>bytes_in_current_limb == <span class="number">0 </span>{
        bytes_in_current_limb = LIMB_BYTES;
    }

    <span class="kw">let </span>num_encoded_limbs = (input.len() / LIMB_BYTES)
        + (<span class="kw">if </span>bytes_in_current_limb == LIMB_BYTES {
            <span class="number">0
        </span>} <span class="kw">else </span>{
            <span class="number">1
        </span>});
    <span class="kw">if </span>num_encoded_limbs &gt; result.len() {
        <span class="kw">return </span><span class="prelude-val">Err</span>(error::Unspecified);
    }

    result.fill(<span class="number">0</span>);

    <span class="comment">// XXX: Questionable as far as constant-timedness is concerned.
    // TODO: Improve this.
    </span>input.read_all(error::Unspecified, |input| {
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..num_encoded_limbs {
            <span class="kw">let </span><span class="kw-2">mut </span>limb: Limb = <span class="number">0</span>;
            <span class="kw">for _ in </span><span class="number">0</span>..bytes_in_current_limb {
                <span class="kw">let </span>b: Limb = input.read_byte()<span class="question-mark">?</span>.into();
                limb = (limb &lt;&lt; <span class="number">8</span>) | b;
            }
            result[num_encoded_limbs - i - <span class="number">1</span>] = limb;
            bytes_in_current_limb = LIMB_BYTES;
        }
        <span class="prelude-val">Ok</span>(())
    })
}

<span class="kw">pub fn </span>big_endian_from_limbs(limbs: <span class="kw-2">&amp;</span>[Limb], out: <span class="kw-2">&amp;mut </span>[u8]) {
    <span class="kw">let </span>be_bytes = unstripped_be_bytes(limbs);
    <span class="macro">assert_eq!</span>(out.len(), be_bytes.len());
    out.iter_mut().zip(be_bytes).for_each(|(o, i)| {
        <span class="kw-2">*</span>o = i;
    });
}

<span class="doccomment">/// Returns an iterator of the big-endian encoding of `limbs`.
///
/// The number of bytes returned will be a multiple of `LIMB_BYTES`
/// and thus may be padded with leading zeros.
</span><span class="kw">pub fn </span>unstripped_be_bytes(limbs: <span class="kw-2">&amp;</span>[Limb]) -&gt; <span class="kw">impl </span>ExactSizeIterator&lt;Item = u8&gt; + Clone + <span class="lifetime">'_ </span>{
    <span class="comment">// The unwrap is safe because a slice can never be larger than `usize` bytes.
    </span>ArrayFlatMap::new(limbs.iter().rev().copied(), Limb::to_be_bytes).unwrap()
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">pub type </span>Window = Limb;

<span class="doccomment">/// Processes `limbs` as a sequence of 5-bit windows, folding the windows from
/// most significant to least significant and returning the accumulated result.
/// The first window will be mapped by `init` to produce the initial value for
/// the accumulator. Then `f` will be called to fold the accumulator and the
/// next window until all windows are processed. When the input's bit length
/// isn't divisible by 5, the window passed to `init` will be partial; all
/// windows passed to `fold` will be full.
///
/// This is designed to avoid leaking the contents of `limbs` through side
/// channels as long as `init` and `fold` are side-channel free.
///
/// Panics if `limbs` is empty.
</span><span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">pub fn </span>fold_5_bit_windows&lt;R, I: FnOnce(Window) -&gt; R, F: Fn(R, Window) -&gt; R&gt;(
    limbs: <span class="kw-2">&amp;</span>[Limb],
    init: I,
    fold: F,
) -&gt; R {
    <span class="attr">#[derive(Clone, Copy)]
    #[repr(transparent)]
    </span><span class="kw">struct </span>BitIndex(Wrapping&lt;c::size_t&gt;);

    <span class="kw">const </span>WINDOW_BITS: Wrapping&lt;c::size_t&gt; = Wrapping(<span class="number">5</span>);

    <span class="macro">prefixed_extern!</span> {
        <span class="kw">fn </span>LIMBS_window5_split_window(
            lower_limb: Limb,
            higher_limb: Limb,
            index_within_word: BitIndex,
        ) -&gt; Window;
        <span class="kw">fn </span>LIMBS_window5_unsplit_window(limb: Limb, index_within_word: BitIndex) -&gt; Window;
    }

    <span class="kw">let </span>num_limbs = limbs.len();
    <span class="kw">let </span><span class="kw-2">mut </span>window_low_bit = {
        <span class="kw">let </span>num_whole_windows = (num_limbs * LIMB_BITS) / <span class="number">5</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>leading_bits = (num_limbs * LIMB_BITS) - (num_whole_windows * <span class="number">5</span>);
        <span class="kw">if </span>leading_bits == <span class="number">0 </span>{
            leading_bits = WINDOW_BITS.<span class="number">0</span>;
        }
        BitIndex(Wrapping(LIMB_BITS - leading_bits))
    };

    <span class="kw">let </span>initial_value = {
        <span class="kw">let </span>leading_partial_window =
            <span class="kw">unsafe </span>{ LIMBS_window5_split_window(<span class="kw-2">*</span>limbs.last().unwrap(), <span class="number">0</span>, window_low_bit) };
        window_low_bit.<span class="number">0 </span>-= WINDOW_BITS;
        init(leading_partial_window)
    };

    <span class="kw">let </span><span class="kw-2">mut </span>low_limb = <span class="number">0</span>;
    limbs
        .iter()
        .rev()
        .fold(initial_value, |<span class="kw-2">mut </span>acc, current_limb| {
            <span class="kw">let </span>higher_limb = low_limb;
            low_limb = <span class="kw-2">*</span>current_limb;

            <span class="kw">if </span>window_low_bit.<span class="number">0 </span>&gt; Wrapping(LIMB_BITS) - WINDOW_BITS {
                <span class="kw">let </span>window =
                    <span class="kw">unsafe </span>{ LIMBS_window5_split_window(low_limb, higher_limb, window_low_bit) };
                window_low_bit.<span class="number">0 </span>-= WINDOW_BITS;
                acc = fold(acc, window);
            };
            <span class="kw">while </span>window_low_bit.<span class="number">0 </span>&lt; Wrapping(LIMB_BITS) {
                <span class="kw">let </span>window = <span class="kw">unsafe </span>{ LIMBS_window5_unsplit_window(low_limb, window_low_bit) };
                <span class="comment">// The loop exits when this subtraction underflows, causing `window_low_bit` to
                // wrap around to a very large value.
                </span>window_low_bit.<span class="number">0 </span>-= WINDOW_BITS;
                acc = fold(acc, window);
            }
            window_low_bit.<span class="number">0 </span>+= Wrapping(LIMB_BITS); <span class="comment">// "Fix" the underflow.

            </span>acc
        })
}

<span class="attr">#[inline]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>limbs_add_assign_mod(a: <span class="kw-2">&amp;mut </span>[Limb], b: <span class="kw-2">&amp;</span>[Limb], m: <span class="kw-2">&amp;</span>[Limb]) {
    <span class="macro">debug_assert_eq!</span>(a.len(), m.len());
    <span class="macro">debug_assert_eq!</span>(b.len(), m.len());
    <span class="macro">prefixed_extern!</span> {
        <span class="comment">// `r` and `a` may alias.
        </span><span class="kw">fn </span>LIMBS_add_mod(
            r: <span class="kw-2">*mut </span>Limb,
            a: <span class="kw-2">*const </span>Limb,
            b: <span class="kw-2">*const </span>Limb,
            m: <span class="kw-2">*const </span>Limb,
            num_limbs: c::size_t,
        );
    }
    <span class="kw">unsafe </span>{ LIMBS_add_mod(a.as_mut_ptr(), a.as_ptr(), b.as_ptr(), m.as_ptr(), m.len()) }
}

<span class="comment">// r *= 2 (mod m).
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>limbs_double_mod(r: <span class="kw-2">&amp;mut </span>[Limb], m: <span class="kw-2">&amp;</span>[Limb]) {
    <span class="macro">assert_eq!</span>(r.len(), m.len());
    <span class="macro">prefixed_extern!</span> {
        <span class="kw">fn </span>LIMBS_shl_mod(r: <span class="kw-2">*mut </span>Limb, a: <span class="kw-2">*const </span>Limb, m: <span class="kw-2">*const </span>Limb, num_limbs: c::size_t);
    }
    <span class="kw">unsafe </span>{
        LIMBS_shl_mod(r.as_mut_ptr(), r.as_ptr(), m.as_ptr(), m.len());
    }
}

<span class="comment">// *r = -a, assuming a is odd.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>limbs_negative_odd(r: <span class="kw-2">&amp;mut </span>[Limb], a: <span class="kw-2">&amp;</span>[Limb]) {
    <span class="macro">debug_assert_eq!</span>(r.len(), a.len());
    <span class="comment">// Two's complement step 1: flip all the bits.
    // The compiler should optimize this to vectorized (a ^ !0).
    </span>r.iter_mut().zip(a.iter()).for_each(|(r, <span class="kw-2">&amp;</span>a)| {
        <span class="kw-2">*</span>r = !a;
    });
    <span class="comment">// Two's complement step 2: Add one. Since `a` is odd, `r` is even. Thus we
    // can use a bitwise or for addition.
    </span>r[<span class="number">0</span>] |= <span class="number">1</span>;
}

<span class="macro">prefixed_extern!</span> {
    <span class="kw">fn </span>LIMBS_are_zero(a: <span class="kw-2">*const </span>Limb, num_limbs: c::size_t) -&gt; LimbMask;
    <span class="kw">fn </span>LIMBS_less_than(a: <span class="kw-2">*const </span>Limb, b: <span class="kw-2">*const </span>Limb, num_limbs: c::size_t) -&gt; LimbMask;
    <span class="kw">fn </span>LIMBS_reduce_once(r: <span class="kw-2">*mut </span>Limb, m: <span class="kw-2">*const </span>Limb, num_limbs: c::size_t);
}

<span class="attr">#[cfg(any(test, feature = <span class="string">"alloc"</span>))]
</span><span class="macro">prefixed_extern!</span> {
    <span class="kw">fn </span>LIMB_shr(a: Limb, shift: c::size_t) -&gt; Limb;
    <span class="kw">fn </span>LIMBS_are_even(a: <span class="kw-2">*const </span>Limb, num_limbs: c::size_t) -&gt; LimbMask;
    <span class="kw">fn </span>LIMBS_equal_limb(a: <span class="kw-2">*const </span>Limb, b: Limb, num_limbs: c::size_t) -&gt; LimbMask;
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="macro">prefixed_extern!</span> {
    <span class="kw">fn </span>LIMBS_less_than_limb(a: <span class="kw-2">*const </span>Limb, b: Limb, num_limbs: c::size_t) -&gt; LimbMask;
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="kw">const </span>MAX: Limb = LimbMask::True <span class="kw">as </span>Limb;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_limbs_are_even() {
        <span class="kw">static </span>EVENS: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>[Limb]] = <span class="kw-2">&amp;</span>[
            <span class="kw-2">&amp;</span>[],
            <span class="kw-2">&amp;</span>[<span class="number">0</span>],
            <span class="kw-2">&amp;</span>[<span class="number">2</span>],
            <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>],
            <span class="kw-2">&amp;</span>[<span class="number">2</span>, <span class="number">0</span>],
            <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">1</span>],
            <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">2</span>],
            <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">3</span>],
            <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, MAX],
        ];
        <span class="kw">for </span>even <span class="kw">in </span>EVENS {
            <span class="macro">assert_eq!</span>(limbs_are_even_constant_time(even), LimbMask::True);
        }
        <span class="kw">static </span>ODDS: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>[Limb]] = <span class="kw-2">&amp;</span>[
            <span class="kw-2">&amp;</span>[<span class="number">1</span>],
            <span class="kw-2">&amp;</span>[<span class="number">3</span>],
            <span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">0</span>],
            <span class="kw-2">&amp;</span>[<span class="number">3</span>, <span class="number">0</span>],
            <span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">1</span>],
            <span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">2</span>],
            <span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">3</span>],
            <span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, MAX],
        ];
        <span class="kw">for </span>odd <span class="kw">in </span>ODDS {
            <span class="macro">assert_eq!</span>(limbs_are_even_constant_time(odd), LimbMask::False);
        }
    }

    <span class="kw">static </span>ZEROES: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>[Limb]] = <span class="kw-2">&amp;</span>[
        <span class="kw-2">&amp;</span>[],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>],
    ];

    <span class="kw">static </span>NONZEROES: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>[Limb]] = <span class="kw-2">&amp;</span>[
        <span class="kw-2">&amp;</span>[<span class="number">1</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">1</span>],
        <span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">1</span>],
        <span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">1</span>, <span class="number">0</span>, <span class="number">0</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">1</span>, <span class="number">0</span>],
        <span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">1</span>],
    ];

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_limbs_are_zero() {
        <span class="kw">for </span>zero <span class="kw">in </span>ZEROES {
            <span class="macro">assert_eq!</span>(limbs_are_zero_constant_time(zero), LimbMask::True);
        }
        <span class="kw">for </span>nonzero <span class="kw">in </span>NONZEROES {
            <span class="macro">assert_eq!</span>(limbs_are_zero_constant_time(nonzero), LimbMask::False);
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_limbs_equal_limb() {
        <span class="kw">for </span>zero <span class="kw">in </span>ZEROES {
            <span class="macro">assert_eq!</span>(limbs_equal_limb_constant_time(zero, <span class="number">0</span>), LimbMask::True);
        }
        <span class="kw">for </span>nonzero <span class="kw">in </span>NONZEROES {
            <span class="macro">assert_eq!</span>(limbs_equal_limb_constant_time(nonzero, <span class="number">0</span>), LimbMask::False);
        }
        <span class="kw">static </span>EQUAL: <span class="kw-2">&amp;</span>[(<span class="kw-2">&amp;</span>[Limb], Limb)] = <span class="kw-2">&amp;</span>[
            (<span class="kw-2">&amp;</span>[<span class="number">1</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[MAX], MAX),
            (<span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[MAX, <span class="number">0</span>, <span class="number">0</span>], MAX),
            (<span class="kw-2">&amp;</span>[<span class="number">0b100</span>], <span class="number">0b100</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0b100</span>, <span class="number">0</span>], <span class="number">0b100</span>),
        ];
        <span class="kw">for </span><span class="kw-2">&amp;</span>(a, b) <span class="kw">in </span>EQUAL {
            <span class="macro">assert_eq!</span>(limbs_equal_limb_constant_time(a, b), LimbMask::True);
        }
        <span class="kw">static </span>UNEQUAL: <span class="kw-2">&amp;</span>[(<span class="kw-2">&amp;</span>[Limb], Limb)] = <span class="kw-2">&amp;</span>[
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">2</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">3</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">1</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0b100</span>, <span class="number">0b100</span>], <span class="number">0b100</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">0</span>, <span class="number">0b100</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0b100</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[MAX, MAX], MAX),
            (<span class="kw-2">&amp;</span>[MAX, <span class="number">1</span>], MAX),
        ];
        <span class="kw">for </span><span class="kw-2">&amp;</span>(a, b) <span class="kw">in </span>UNEQUAL {
            <span class="macro">assert_eq!</span>(limbs_equal_limb_constant_time(a, b), LimbMask::False);
        }
    }

    <span class="attr">#[test]
    #[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>test_limbs_less_than_limb_constant_time() {
        <span class="kw">static </span>LESSER: <span class="kw-2">&amp;</span>[(<span class="kw-2">&amp;</span>[Limb], Limb)] = <span class="kw-2">&amp;</span>[
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">0</span>], <span class="number">2</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">2</span>, <span class="number">0</span>], <span class="number">3</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">2</span>, <span class="number">0</span>], <span class="number">3</span>),
            (<span class="kw-2">&amp;</span>[MAX - <span class="number">1</span>], MAX),
            (<span class="kw-2">&amp;</span>[MAX - <span class="number">1</span>, <span class="number">0</span>], MAX),
        ];
        <span class="kw">for </span><span class="kw-2">&amp;</span>(a, b) <span class="kw">in </span>LESSER {
            <span class="macro">assert_eq!</span>(limbs_less_than_limb_constant_time(a, b), LimbMask::True);
        }
        <span class="kw">static </span>EQUAL: <span class="kw-2">&amp;</span>[(<span class="kw-2">&amp;</span>[Limb], Limb)] = <span class="kw-2">&amp;</span>[
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>], <span class="number">0</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>], <span class="number">0</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">1</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[MAX], MAX),
        ];
        <span class="kw">static </span>GREATER: <span class="kw-2">&amp;</span>[(<span class="kw-2">&amp;</span>[Limb], Limb)] = <span class="kw-2">&amp;</span>[
            (<span class="kw-2">&amp;</span>[<span class="number">1</span>], <span class="number">0</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">2</span>, <span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">3</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">1</span>, <span class="number">0</span>, <span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">1</span>, <span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>, <span class="number">1</span>, <span class="number">1</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[MAX], MAX - <span class="number">1</span>),
        ];
        <span class="kw">for </span><span class="kw-2">&amp;</span>(a, b) <span class="kw">in </span>EQUAL.iter().chain(GREATER.iter()) {
            <span class="macro">assert_eq!</span>(limbs_less_than_limb_constant_time(a, b), LimbMask::False);
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_parse_big_endian_and_pad_consttime() {
        <span class="kw">const </span>LIMBS: usize = <span class="number">4</span>;

        {
            <span class="comment">// Empty input.
            </span><span class="kw">let </span>inp = untrusted::Input::from(<span class="kw-2">&amp;</span>[]);
            <span class="kw">let </span><span class="kw-2">mut </span>result = [<span class="number">0</span>; LIMBS];
            <span class="macro">assert!</span>(parse_big_endian_and_pad_consttime(inp, <span class="kw-2">&amp;mut </span>result).is_err());
        }

        <span class="comment">// The input is longer than will fit in the given number of limbs.
        </span>{
            <span class="kw">let </span>inp = [<span class="number">1</span>, <span class="number">2</span>, <span class="number">3</span>, <span class="number">4</span>, <span class="number">5</span>, <span class="number">6</span>, <span class="number">7</span>, <span class="number">8</span>, <span class="number">9</span>];
            <span class="kw">let </span>inp = untrusted::Input::from(<span class="kw-2">&amp;</span>inp);
            <span class="kw">let </span><span class="kw-2">mut </span>result = [<span class="number">0</span>; <span class="number">8 </span>/ LIMB_BYTES];
            <span class="macro">assert!</span>(parse_big_endian_and_pad_consttime(inp, <span class="kw-2">&amp;mut </span>result[..]).is_err());
        }

        <span class="comment">// Less than a full limb.
        </span>{
            <span class="kw">let </span>inp = [<span class="number">0xfe</span>];
            <span class="kw">let </span>inp = untrusted::Input::from(<span class="kw-2">&amp;</span>inp);
            <span class="kw">let </span><span class="kw-2">mut </span>result = [<span class="number">0</span>; LIMBS];
            <span class="macro">assert_eq!</span>(
                <span class="prelude-val">Ok</span>(()),
                parse_big_endian_and_pad_consttime(inp, <span class="kw-2">&amp;mut </span>result[..])
            );
            <span class="macro">assert_eq!</span>(<span class="kw-2">&amp;</span>[<span class="number">0xfe</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>], <span class="kw-2">&amp;</span>result);
        }

        <span class="comment">// A whole limb for 32-bit, half a limb for 64-bit.
        </span>{
            <span class="kw">let </span>inp = [<span class="number">0xbe</span>, <span class="number">0xef</span>, <span class="number">0xf0</span>, <span class="number">0x0d</span>];
            <span class="kw">let </span>inp = untrusted::Input::from(<span class="kw-2">&amp;</span>inp);
            <span class="kw">let </span><span class="kw-2">mut </span>result = [<span class="number">0</span>; LIMBS];
            <span class="macro">assert_eq!</span>(<span class="prelude-val">Ok</span>(()), parse_big_endian_and_pad_consttime(inp, <span class="kw-2">&amp;mut </span>result));
            <span class="macro">assert_eq!</span>(<span class="kw-2">&amp;</span>[<span class="number">0xbeeff00d</span>, <span class="number">0</span>, <span class="number">0</span>, <span class="number">0</span>], <span class="kw-2">&amp;</span>result);
        }

        <span class="comment">// XXX: This is a weak set of tests. TODO: expand it.
    </span>}

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_big_endian_from_limbs_same_length() {
        <span class="attr">#[cfg(target_pointer_width = <span class="string">"32"</span>)]
        </span><span class="kw">let </span>limbs = [
            <span class="number">0xbccddeef</span>, <span class="number">0x89900aab</span>, <span class="number">0x45566778</span>, <span class="number">0x01122334</span>, <span class="number">0xddeeff00</span>, <span class="number">0x99aabbcc</span>, <span class="number">0x55667788</span>,
            <span class="number">0x11223344</span>,
        ];

        <span class="attr">#[cfg(target_pointer_width = <span class="string">"64"</span>)]
        </span><span class="kw">let </span>limbs = [
            <span class="number">0x8990_0aab_bccd_deef</span>,
            <span class="number">0x0112_2334_4556_6778</span>,
            <span class="number">0x99aa_bbcc_ddee_ff00</span>,
            <span class="number">0x1122_3344_5566_7788</span>,
        ];

        <span class="kw">let </span>expected = [
            <span class="number">0x11</span>, <span class="number">0x22</span>, <span class="number">0x33</span>, <span class="number">0x44</span>, <span class="number">0x55</span>, <span class="number">0x66</span>, <span class="number">0x77</span>, <span class="number">0x88</span>, <span class="number">0x99</span>, <span class="number">0xaa</span>, <span class="number">0xbb</span>, <span class="number">0xcc</span>, <span class="number">0xdd</span>, <span class="number">0xee</span>,
            <span class="number">0xff</span>, <span class="number">0x00</span>, <span class="number">0x01</span>, <span class="number">0x12</span>, <span class="number">0x23</span>, <span class="number">0x34</span>, <span class="number">0x45</span>, <span class="number">0x56</span>, <span class="number">0x67</span>, <span class="number">0x78</span>, <span class="number">0x89</span>, <span class="number">0x90</span>, <span class="number">0x0a</span>, <span class="number">0xab</span>,
            <span class="number">0xbc</span>, <span class="number">0xcd</span>, <span class="number">0xde</span>, <span class="number">0xef</span>,
        ];

        <span class="kw">let </span><span class="kw-2">mut </span>out = [<span class="number">0xabu8</span>; <span class="number">32</span>];
        big_endian_from_limbs(<span class="kw-2">&amp;</span>limbs[..], <span class="kw-2">&amp;mut </span>out);
        <span class="macro">assert_eq!</span>(<span class="kw-2">&amp;</span>out[..], <span class="kw-2">&amp;</span>expected[..]);
    }

    <span class="attr">#[should_panic]
    #[test]
    </span><span class="kw">fn </span>test_big_endian_from_limbs_fewer_limbs() {
        <span class="attr">#[cfg(target_pointer_width = <span class="string">"32"</span>)]
        </span><span class="comment">// Two fewer limbs.
        </span><span class="kw">let </span>limbs = [
            <span class="number">0xbccddeef</span>, <span class="number">0x89900aab</span>, <span class="number">0x45566778</span>, <span class="number">0x01122334</span>, <span class="number">0xddeeff00</span>, <span class="number">0x99aabbcc</span>,
        ];

        <span class="comment">// One fewer limb.
        </span><span class="attr">#[cfg(target_pointer_width = <span class="string">"64"</span>)]
        </span><span class="kw">let </span>limbs = [
            <span class="number">0x8990_0aab_bccd_deef</span>,
            <span class="number">0x0112_2334_4556_6778</span>,
            <span class="number">0x99aa_bbcc_ddee_ff00</span>,
        ];

        <span class="kw">let </span><span class="kw-2">mut </span>out = [<span class="number">0xabu8</span>; <span class="number">32</span>];

        big_endian_from_limbs(<span class="kw-2">&amp;</span>limbs[..], <span class="kw-2">&amp;mut </span>out);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_limbs_minimal_bits() {
        <span class="kw">const </span>ALL_ONES: Limb = LimbMask::True <span class="kw">as </span>Limb;
        <span class="kw">static </span>CASES: <span class="kw-2">&amp;</span>[(<span class="kw-2">&amp;</span>[Limb], usize)] = <span class="kw-2">&amp;</span>[
            (<span class="kw-2">&amp;</span>[], <span class="number">0</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>], <span class="number">0</span>),
            (<span class="kw-2">&amp;</span>[ALL_ONES], LIMB_BITS),
            (<span class="kw-2">&amp;</span>[ALL_ONES, <span class="number">0</span>], LIMB_BITS),
            (<span class="kw-2">&amp;</span>[ALL_ONES, <span class="number">1</span>], LIMB_BITS + <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">0</span>], <span class="number">0</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">1</span>, <span class="number">0</span>], <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>, <span class="number">1</span>], LIMB_BITS + <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[<span class="number">0</span>, ALL_ONES], <span class="number">2 </span>* LIMB_BITS),
            (<span class="kw-2">&amp;</span>[ALL_ONES, ALL_ONES], <span class="number">2 </span>* LIMB_BITS),
            (<span class="kw-2">&amp;</span>[ALL_ONES, ALL_ONES &gt;&gt; <span class="number">1</span>], <span class="number">2 </span>* LIMB_BITS - <span class="number">1</span>),
            (<span class="kw-2">&amp;</span>[ALL_ONES, <span class="number">0b100_0000</span>], LIMB_BITS + <span class="number">7</span>),
            (<span class="kw-2">&amp;</span>[ALL_ONES, <span class="number">0b101_0000</span>], LIMB_BITS + <span class="number">7</span>),
            (<span class="kw-2">&amp;</span>[ALL_ONES, ALL_ONES &gt;&gt; <span class="number">1</span>], LIMB_BITS + (LIMB_BITS) - <span class="number">1</span>),
        ];
        <span class="kw">for </span>(limbs, bits) <span class="kw">in </span>CASES {
            <span class="macro">assert_eq!</span>(limbs_minimal_bits(limbs).as_bits(), <span class="kw-2">*</span>bits);
        }
    }
}
</code></pre></div></section></main></body></html>