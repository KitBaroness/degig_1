<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-sdk-1.18.9/src/feature_set.rs`."><title>feature_set.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="solana_sdk" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../solana_sdk/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Collection of all runtime features.
//!
//! Steps to add a new feature are outlined below. Note that these steps only cover
//! the process of getting a feature into the core Solana code.
//! - For features that are unambiguously good (ie bug fixes), these steps are sufficient.
//! - For features that should go up for community vote (ie fee structure changes), more
//!   information on the additional steps to follow can be found at:
//!   &lt;https://spl.solana.com/feature-proposal#feature-proposal-life-cycle&gt;
//!
//! 1. Generate a new keypair with `solana-keygen new --outfile feature.json --no-passphrase`
//!    - Keypairs should be held by core contributors only. If you're a non-core contributor going
//!      through these steps, the PR process will facilitate a keypair holder being picked. That
//!      person will generate the keypair, provide pubkey for PR, and ultimately enable the feature.
//! 2. Add a public module for the feature, specifying keypair pubkey as the id with
//!    `solana_sdk::declare_id!()` within the module.
//!    Additionally, add an entry to `FEATURE_NAMES` map.
//! 3. Add desired logic to check for and switch on feature availability.
//!
//! For more information on how features are picked up, see comments for `Feature`.

</span><span class="kw">use </span>{
    lazy_static::lazy_static,
    solana_program::{epoch_schedule::EpochSchedule, stake_history::Epoch},
    solana_sdk::{
        clock::Slot,
        hash::{Hash, Hasher},
        pubkey::Pubkey,
    },
    std::collections::{HashMap, HashSet},
};

<span class="kw">pub mod </span>deprecate_rewards_sysvar {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"GaBtBJvmS4Arjj5W1NmFcyvPjsHN38UGYDq2MDwbs9Qu"</span>);
}

<span class="kw">pub mod </span>pico_inflation {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"4RWNif6C2WCNiKVW7otP4G7dkmkHGyKQWRpuZ1pxKU5m"</span>);
}

<span class="kw">pub mod </span>full_inflation {
    <span class="kw">pub mod </span>devnet_and_testnet {
        <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"DT4n6ABDqs6w4bnfwrXT9rsprcPf6cdDga1egctaPkLC"</span>);
    }

    <span class="kw">pub mod </span>mainnet {
        <span class="kw">pub mod </span>certusone {
            <span class="kw">pub mod </span>vote {
                <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"BzBBveUDymEYoYzcMWNQCx3cd4jQs7puaVFHLtsbB6fm"</span>);
            }
            <span class="kw">pub mod </span>enable {
                <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"7XRJcS5Ud5vxGB54JbK9N2vBZVwnwdBNeJW1ibRgD9gx"</span>);
            }
        }
    }
}

<span class="kw">pub mod </span>secp256k1_program_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"E3PHP7w8kB7np3CTQ1qQ2tW3KCtjRSXBQgW9vM2mWv2Y"</span>);
}

<span class="kw">pub mod </span>spl_token_v2_multisig_fix {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"E5JiFDQCwyC6QfT9REFyMpfK2mHcmv1GUDySU1Ue7TYv"</span>);
}

<span class="kw">pub mod </span>no_overflow_rent_distribution {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"4kpdyrcj5jS47CZb2oJGfVxjYbsMm2Kx97gFyZrxxwXz"</span>);
}

<span class="kw">pub mod </span>filter_stake_delegation_accounts {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"GE7fRxmW46K6EmCD9AMZSbnaJ2e3LfqCZzdHi9hmYAgi"</span>);
}

<span class="kw">pub mod </span>require_custodian_for_locked_stake_authorize {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"D4jsDcXaqdW8tDAWn8H4R25Cdns2YwLneujSL1zvjW6R"</span>);
}

<span class="kw">pub mod </span>spl_token_v2_self_transfer_fix {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"BL99GYhdjjcv6ys22C9wPgn2aTVERDbPHHo4NbS3hgp7"</span>);
}

<span class="kw">pub mod </span>warp_timestamp_again {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"GvDsGDkH5gyzwpDhxNixx8vtx1kwYHH13RiNAPw27zXb"</span>);
}

<span class="kw">pub mod </span>check_init_vote_data {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3ccR6QpxGYsAbWyfevEtBNGfWV4xBffxRj2tD6A9i39F"</span>);
}

<span class="kw">pub mod </span>secp256k1_recover_syscall_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"6RvdSWHh8oh72Dp7wMTS2DBkf3fRPtChfNrAo3cZZoXJ"</span>);
}

<span class="kw">pub mod </span>system_transfer_zero_check {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"BrTR9hzw4WBGFP65AJMbpAo64DcA3U6jdPSga9fMV5cS"</span>);
}

<span class="kw">pub mod </span>blake3_syscall_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"HTW2pSyErTj4BV6KBM9NZ9VBUJVxt7sacNWcf76wtzb3"</span>);
}

<span class="kw">pub mod </span>dedupe_config_program_signers {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"8kEuAshXLsgkUEdcFVLqrjCGGHVWFW99ZZpxvAzzMtBp"</span>);
}

<span class="kw">pub mod </span>verify_tx_signatures_len {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"EVW9B5xD9FFK7vw1SBARwMA4s5eRo5eKJdKpsBikzKBz"</span>);
}

<span class="kw">pub mod </span>vote_stake_checked_instructions {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"BcWknVcgvonN8sL4HE4XFuEVgfcee5MwxWPAgP6ZV89X"</span>);
}

<span class="kw">pub mod </span>rent_for_sysvars {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"BKCPBQQBZqggVnFso5nQ8rQ4RwwogYwjuUt9biBjxwNF"</span>);
}

<span class="kw">pub mod </span>libsecp256k1_0_5_upgrade_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"DhsYfRjxfnh2g7HKJYSzT79r74Afa1wbHkAgHndrA1oy"</span>);
}

<span class="kw">pub mod </span>tx_wide_compute_cap {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"5ekBxc8itEnPv4NzGJtr8BVVQLNMQuLMNQQj7pHoLNZ9"</span>);
}

<span class="kw">pub mod </span>spl_token_v2_set_authority_fix {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"FToKNBYyiF4ky9s8WsmLBXHCht17Ek7RXaLZGHzzQhJ1"</span>);
}

<span class="kw">pub mod </span>merge_nonce_error_into_system_error {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"21AWDosvp3pBamFW91KB35pNoaoZVTM7ess8nr2nt53B"</span>);
}

<span class="kw">pub mod </span>disable_fees_sysvar {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"JAN1trEUEtZjgXYzNBYHU9DYd7GnThhXfFP7SzPXkPsG"</span>);
}

<span class="kw">pub mod </span>stake_merge_with_unmatched_credits_observed {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"meRgp4ArRPhD3KtCY9c5yAf2med7mBLsjKTPeVUHqBL"</span>);
}

<span class="kw">pub mod </span>zk_token_sdk_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"zk1snxsc6Fh3wsGNbbHAJNHiJoYgF29mMnTSusGx5EJ"</span>);
}

<span class="kw">pub mod </span>curve25519_syscall_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"7rcw5UtqgDTBBv2EcynNfYckgdAaH1MAsCjKgXMkN7Ri"</span>);
}

<span class="kw">pub mod </span>curve25519_restrict_msm_length {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"eca6zf6JJRjQsYYPkBHF3N32MTzur4n2WL4QiiacPCL"</span>);
}

<span class="kw">pub mod </span>versioned_tx_message_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3KZZ6Ks1885aGBQ45fwRcPXVBCtzUvxhUTkwKMR41Tca"</span>);
}

<span class="kw">pub mod </span>libsecp256k1_fail_on_bad_count {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"8aXvSuopd1PUj7UhehfXJRg6619RHp8ZvwTyyJHdUYsj"</span>);
}

<span class="kw">pub mod </span>libsecp256k1_fail_on_bad_count2 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"54KAoNiUERNoWWUhTWWwXgym94gzoXFVnHyQwPA18V9A"</span>);
}

<span class="kw">pub mod </span>instructions_sysvar_owned_by_sysvar {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"H3kBSaKdeiUsyHmeHqjJYNc27jesXZ6zWj3zWkowQbkV"</span>);
}

<span class="kw">pub mod </span>stake_program_advance_activating_credits_observed {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"SAdVFw3RZvzbo6DvySbSdBnHN4gkzSTH9dSxesyKKPj"</span>);
}

<span class="kw">pub mod </span>credits_auto_rewind {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"BUS12ciZ5gCoFafUHWW8qaFMMtwFQGVxjsDheWLdqBE2"</span>);
}

<span class="kw">pub mod </span>demote_program_write_locks {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3E3jV7v9VcdJL8iYZUMax9DiDno8j7EWUVbhm9RtShj2"</span>);
}

<span class="kw">pub mod </span>ed25519_program_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"6ppMXNYLhVd7GcsZ5uV11wQEW7spppiMVfqQv5SXhDpX"</span>);
}

<span class="kw">pub mod </span>return_data_syscall_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"DwScAzPUjuv65TMbDnFY7AgwmotzWy3xpEJMXM3hZFaB"</span>);
}

<span class="kw">pub mod </span>reduce_required_deploy_balance {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"EBeznQDjcPG8491sFsKZYBi5S5jTVXMpAKNDJMQPS2kq"</span>);
}

<span class="kw">pub mod </span>sol_log_data_syscall_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"6uaHcKPGUy4J7emLBgUTeufhJdiwhngW6a1R9B7c2ob9"</span>);
}

<span class="kw">pub mod </span>stakes_remove_delegation_if_inactive {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"HFpdDDNQjvcXnXKec697HDDsyk6tFoWS2o8fkxuhQZpL"</span>);
}

<span class="kw">pub mod </span>do_support_realloc {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"75m6ysz33AfLA5DDEzWM1obBrnPQRSsdVQ2nRmc8Vuu1"</span>);
}

<span class="kw">pub mod </span>prevent_calling_precompiles_as_programs {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"4ApgRX3ud6p7LNMJmsuaAcZY5HWctGPr5obAsjB3A54d"</span>);
}

<span class="kw">pub mod </span>optimize_epoch_boundary_updates {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"265hPS8k8xJ37ot82KEgjRunsUp5w4n4Q4VwwiN9i9ps"</span>);
}

<span class="kw">pub mod </span>remove_native_loader {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"HTTgmruMYRZEntyL3EdCDdnS6e4D5wRq1FA7kQsb66qq"</span>);
}

<span class="kw">pub mod </span>send_to_tpu_vote_port {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"C5fh68nJ7uyKAuYZg2x9sEQ5YrVf3dkW6oojNBSc3Jvo"</span>);
}

<span class="kw">pub mod </span>requestable_heap_size {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"CCu4boMmfLuqcmfTLPHQiUo22ZdUsXjgzPAURYaWt1Bw"</span>);
}

<span class="kw">pub mod </span>disable_fee_calculator {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"2jXx2yDmGysmBKfKYNgLj2DQyAQv6mMk2BPh4eSbyB4H"</span>);
}

<span class="kw">pub mod </span>add_compute_budget_program {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"4d5AKtxoh93Dwm1vHXUU3iRATuMndx1c431KgT2td52r"</span>);
}

<span class="kw">pub mod </span>nonce_must_be_writable {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"BiCU7M5w8ZCMykVSyhZ7Q3m2SWoR2qrEQ86ERcDX77ME"</span>);
}

<span class="kw">pub mod </span>spl_token_v3_3_0_release {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Ftok2jhqAqxUWEiCVRrfRs9DPppWP8cgTB7NQNKL88mS"</span>);
}

<span class="kw">pub mod </span>leave_nonce_on_success {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"E8MkiWZNNPGU6n55jkGzyj8ghUmjCHRmDFdYYFYHxWhQ"</span>);
}

<span class="kw">pub mod </span>reject_empty_instruction_without_program {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"9kdtFSrXHQg3hKkbXkQ6trJ3Ja1xpJ22CTFSNAciEwmL"</span>);
}

<span class="kw">pub mod </span>fixed_memcpy_nonoverlapping_check {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"36PRUK2Dz6HWYdG9SpjeAsF5F3KxnFCakA2BZMbtMhSb"</span>);
}

<span class="kw">pub mod </span>reject_non_rent_exempt_vote_withdraws {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"7txXZZD6Um59YoLMF7XUNimbMjsqsWhc7g2EniiTrmp1"</span>);
}

<span class="kw">pub mod </span>evict_invalid_stakes_cache_entries {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"EMX9Q7TVFAmQ9V1CggAkhMzhXSg8ECp7fHrWQX2G1chf"</span>);
}

<span class="kw">pub mod </span>allow_votes_to_directly_update_vote_state {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Ff8b1fBeB86q8cjq47ZhsQLgv5EkHu3G1C99zjUfAzrq"</span>);
}

<span class="kw">pub mod </span>cap_accounts_data_len {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"capRxUrBjNkkCpjrJxPGfPaWijB7q3JoDfsWXAnt46r"</span>);
}

<span class="kw">pub mod </span>max_tx_account_locks {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"CBkDroRDqm8HwHe6ak9cguPjUomrASEkfmxEaZ5CNNxz"</span>);
}

<span class="kw">pub mod </span>require_rent_exempt_accounts {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"BkFDxiJQWZXGTZaJQxH7wVEHkAmwCgSEVkrvswFfRJPD"</span>);
}

<span class="kw">pub mod </span>filter_votes_outside_slot_hashes {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3gtZPqvPpsbXZVCx6hceMfWxtsmrjMzmg8C7PLKSxS2d"</span>);
}

<span class="kw">pub mod </span>update_syscall_base_costs {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"2h63t332mGCCsWK2nqqqHhN4U9ayyqhLVFvczznHDoTZ"</span>);
}

<span class="kw">pub mod </span>stake_deactivate_delinquent_instruction {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"437r62HoAdUb63amq3D7ENnBLDhHT2xY8eFkLJYVKK4x"</span>);
}

<span class="kw">pub mod </span>stake_redelegate_instruction {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"2KKG3C6RBnxQo9jVVrbzsoSh41TDXLK7gBc9gduyxSzW"</span>);
}

<span class="kw">pub mod </span>vote_withdraw_authority_may_change_authorized_voter {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"AVZS3ZsN4gi6Rkx2QUibYuSJG3S6QHib7xCYhG6vGJxU"</span>);
}

<span class="kw">pub mod </span>spl_associated_token_account_v1_0_4 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"FaTa4SpiaSNH44PGC4z8bnGVTkSRYaWvrBs3KTu8XQQq"</span>);
}

<span class="kw">pub mod </span>reject_vote_account_close_unless_zero_credit_epoch {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"ALBk3EWdeAg2WAGf6GPDUf1nynyNqCdEVmgouG7rpuCj"</span>);
}

<span class="kw">pub mod </span>add_get_processed_sibling_instruction_syscall {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"CFK1hRCNy8JJuAAY8Pb2GjLFNdCThS2qwZNe3izzBMgn"</span>);
}

<span class="kw">pub mod </span>bank_transaction_count_fix {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Vo5siZ442SaZBKPXNocthiXysNviW4UYPwRFggmbgAp"</span>);
}

<span class="kw">pub mod </span>disable_bpf_deprecated_load_instructions {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3XgNukcZWf9o3HdA3fpJbm94XFc4qpvTXc8h1wxYwiPi"</span>);
}

<span class="kw">pub mod </span>disable_bpf_unresolved_symbols_at_runtime {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"4yuaYAj2jGMGTh1sSmi4G2eFscsDq8qjugJXZoBN6YEa"</span>);
}

<span class="kw">pub mod </span>record_instruction_in_transaction_context_push {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3aJdcZqxoLpSBxgeYGjPwaYS1zzcByxUDqJkbzWAH1Zb"</span>);
}

<span class="kw">pub mod </span>syscall_saturated_math {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"HyrbKftCdJ5CrUfEti6x26Cj7rZLNe32weugk7tLcWb8"</span>);
}

<span class="kw">pub mod </span>check_physical_overlapping {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"nWBqjr3gpETbiaVj3CBJ3HFC5TMdnJDGt21hnvSTvVZ"</span>);
}

<span class="kw">pub mod </span>limit_secp256k1_recovery_id {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"7g9EUwj4j7CS21Yx1wvgWLjSZeh5aPq8x9kpoPwXM8n8"</span>);
}

<span class="kw">pub mod </span>disable_deprecated_loader {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"GTUMCZ8LTNxVfxdrw7ZsDFTxXb7TutYkzJnFwinpE6dg"</span>);
}

<span class="kw">pub mod </span>check_slice_translation_size {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"GmC19j9qLn2RFk5NduX6QXaDhVpGncVVBzyM8e9WMz2F"</span>);
}

<span class="kw">pub mod </span>stake_split_uses_rent_sysvar {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"FQnc7U4koHqWgRvFaBJjZnV8VPg6L6wWK33yJeDp4yvV"</span>);
}

<span class="kw">pub mod </span>add_get_minimum_delegation_instruction_to_stake_program {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"St8k9dVXP97xT6faW24YmRSYConLbhsMJA4TJTBLmMT"</span>);
}

<span class="kw">pub mod </span>error_on_syscall_bpf_function_hash_collisions {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"8199Q2gMD2kwgfopK5qqVWuDbegLgpuFUFHCcUJQDN8b"</span>);
}

<span class="kw">pub mod </span>reject_callx_r10 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3NKRSwpySNwD3TvP5pHnRmkAQRsdkXWRr1WaQh8p4PWX"</span>);
}

<span class="kw">pub mod </span>drop_redundant_turbine_path {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"4Di3y24QFLt5QEUPZtbnjyfQKfm6ZMTfa6Dw1psfoMKU"</span>);
}

<span class="kw">pub mod </span>executables_incur_cpi_data_cost {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"7GUcYgq4tVtaqNCKT3dho9r4665Qp5TxCZ27Qgjx3829"</span>);
}

<span class="kw">pub mod </span>fix_recent_blockhashes {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"6iyggb5MTcsvdcugX7bEKbHV8c6jdLbpHwkncrgLMhfo"</span>);
}

<span class="kw">pub mod </span>update_rewards_from_cached_accounts {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"28s7i3htzhahXQKqmS2ExzbEoUypg9krwvtK2M9UWXh9"</span>);
}
<span class="kw">pub mod </span>enable_partitioned_epoch_reward {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"41tVp5qR1XwWRt5WifvtSQyuxtqQWJgEK8w91AtBqSwP"</span>);
}

<span class="kw">pub mod </span>spl_token_v3_4_0 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Ftok4njE8b7tDffYkC5bAbCaQv5sL6jispYrprzatUwN"</span>);
}

<span class="kw">pub mod </span>spl_associated_token_account_v1_1_0 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"FaTa17gVKoqbh38HcfiQonPsAaQViyDCCSg71AubYZw8"</span>);
}

<span class="kw">pub mod </span>default_units_per_instruction {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"J2QdYx8crLbTVK8nur1jeLsmc3krDbfjoxoea2V1Uy5Q"</span>);
}

<span class="kw">pub mod </span>stake_allow_zero_undelegated_amount {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"sTKz343FM8mqtyGvYWvbLpTThw3ixRM4Xk8QvZ985mw"</span>);
}

<span class="kw">pub mod </span>require_static_program_ids_in_transaction {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"8FdwgyHFEjhAdjWfV2vfqk7wA1g9X3fQpKH7SBpEv3kC"</span>);
}

<span class="kw">pub mod </span>stake_raise_minimum_delegation_to_1_sol {
    <span class="comment">// This is a feature-proposal *feature id*.  The feature keypair address is `GQXzC7YiSNkje6FFUk6sc2p53XRvKoaZ9VMktYzUMnpL`.
    </span><span class="macro">solana_sdk::declare_id!</span>(<span class="string">"9onWzzvCzNC2jfhxxeqRgs5q7nFAAKpCUvkj6T6GJK9i"</span>);
}

<span class="kw">pub mod </span>stake_minimum_delegation_for_rewards {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"G6ANXD6ptCSyNd9znZm7j4dEczAJCfx7Cy43oBx3rKHJ"</span>);
}

<span class="kw">pub mod </span>add_set_compute_unit_price_ix {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"98std1NSHqXi9WYvFShfVepRdCoq1qvsp8fsR2XZtG8g"</span>);
}

<span class="kw">pub mod </span>disable_deploy_of_alloc_free_syscall {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"79HWsX9rpnnJBPcdNURVqygpMAfxdrAirzAGAVmf92im"</span>);
}

<span class="kw">pub mod </span>include_account_index_in_rent_error {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"2R72wpcQ7qV7aTJWUumdn8u5wmmTyXbK7qzEy7YSAgyY"</span>);
}

<span class="kw">pub mod </span>add_shred_type_to_shred_seed {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Ds87KVeqhbv7Jw8W6avsS1mqz3Mw5J3pRTpPoDQ2QdiJ"</span>);
}

<span class="kw">pub mod </span>warp_timestamp_with_a_vengeance {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3BX6SBeEBibHaVQXywdkcgyUk6evfYZkHdztXiDtEpFS"</span>);
}

<span class="kw">pub mod </span>separate_nonce_from_blockhash {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Gea3ZkK2N4pHuVZVxWcnAtS6UEDdyumdYt4pFcKjA3ar"</span>);
}

<span class="kw">pub mod </span>enable_durable_nonce {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"4EJQtF2pkRyawwcTVfQutzq4Sa5hRhibF6QAK1QXhtEX"</span>);
}

<span class="kw">pub mod </span>vote_state_update_credit_per_dequeue {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"CveezY6FDLVBToHDcvJRmtMouqzsmj4UXYh5ths5G5Uv"</span>);
}

<span class="kw">pub mod </span>quick_bail_on_panic {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"DpJREPyuMZ5nDfU6H3WTqSqUFSXAfw8u7xqmWtEwJDcP"</span>);
}

<span class="kw">pub mod </span>nonce_must_be_authorized {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"HxrEu1gXuH7iD3Puua1ohd5n4iUKJyFNtNxk9DVJkvgr"</span>);
}

<span class="kw">pub mod </span>nonce_must_be_advanceable {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3u3Er5Vc2jVcwz4xr2GJeSAXT3fAj6ADHZ4BJMZiScFd"</span>);
}

<span class="kw">pub mod </span>vote_authorize_with_seed {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"6tRxEYKuy2L5nnv5bgn7iT28MxUbYxp5h7F3Ncf1exrT"</span>);
}

<span class="kw">pub mod </span>cap_accounts_data_size_per_block {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"qywiJyZmqTKspFg2LeuUHqcA5nNvBgobqb9UprywS9N"</span>);
}

<span class="kw">pub mod </span>preserve_rent_epoch_for_rent_exempt_accounts {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"HH3MUYReL2BvqqA3oEcAa7txju5GY6G4nxJ51zvsEjEZ"</span>);
}

<span class="kw">pub mod </span>enable_bpf_loader_extend_program_ix {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"8Zs9W7D9MpSEtUWSQdGniZk2cNmV22y6FLJwCx53asme"</span>);
}

<span class="kw">pub mod </span>enable_early_verification_of_account_modifications {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"7Vced912WrRnfjaiKRiNBcbuFw7RrnLv3E3z95Y4GTNc"</span>);
}

<span class="kw">pub mod </span>skip_rent_rewrites {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"CGB2jM8pwZkeeiXQ66kBMyBR6Np61mggL7XUsmLjVcrw"</span>);
}

<span class="kw">pub mod </span>prevent_crediting_accounts_that_end_rent_paying {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"812kqX67odAp5NFwM8D2N24cku7WTm9CHUTFUXaDkWPn"</span>);
}

<span class="kw">pub mod </span>cap_bpf_program_instruction_accounts {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"9k5ijzTbYPtjzu8wj2ErH9v45xecHzQ1x4PMYMMxFgdM"</span>);
}

<span class="kw">pub mod </span>loosen_cpi_size_restriction {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"GDH5TVdbTPUpRnXaRyQqiKUa7uZAbZ28Q2N9bhbKoMLm"</span>);
}

<span class="kw">pub mod </span>use_default_units_in_fee_calculation {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"8sKQrMQoUHtQSUP83SPG4ta2JDjSAiWs7t5aJ9uEd6To"</span>);
}

<span class="kw">pub mod </span>compact_vote_state_updates {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"86HpNqzutEZwLcPxS6EHDcMNYWk6ikhteg9un7Y2PBKE"</span>);
}

<span class="kw">pub mod </span>incremental_snapshot_only_incremental_hash_calculation {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"25vqsfjk7Nv1prsQJmA4Xu1bN61s8LXCBGUPp8Rfy1UF"</span>);
}

<span class="kw">pub mod </span>disable_cpi_setting_executable_and_rent_epoch {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"B9cdB55u4jQsDNsdTK525yE9dmSc5Ga7YBaBrDFvEhM9"</span>);
}

<span class="kw">pub mod </span>on_load_preserve_rent_epoch_for_rent_exempt_accounts {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"CpkdQmspsaZZ8FVAouQTtTWZkc8eeQ7V3uj7dWz543rZ"</span>);
}

<span class="kw">pub mod </span>account_hash_ignore_slot {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"SVn36yVApPLYsa8koK3qUcy14zXDnqkNYWyUh1f4oK1"</span>);
}

<span class="kw">pub mod </span>set_exempt_rent_epoch_max {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"5wAGiy15X1Jb2hkHnPDCM8oB9V42VNA9ftNVFK84dEgv"</span>);
}

<span class="kw">pub mod </span>relax_authority_signer_check_for_lookup_table_creation {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"FKAcEvNgSY79RpqsPNUV5gDyumopH4cEHqUxyfm8b8Ap"</span>);
}

<span class="kw">pub mod </span>stop_sibling_instruction_search_at_parent {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"EYVpEP7uzH1CoXzbD6PubGhYmnxRXPeq3PPsm1ba3gpo"</span>);
}

<span class="kw">pub mod </span>vote_state_update_root_fix {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"G74BkWBzmsByZ1kxHy44H3wjwp5hp7JbrGRuDpco22tY"</span>);
}

<span class="kw">pub mod </span>cap_accounts_data_allocations_per_transaction {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"9gxu85LYRAcZL38We8MYJ4A9AwgBBPtVBAqebMcT1241"</span>);
}

<span class="kw">pub mod </span>epoch_accounts_hash {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"5GpmAKxaGsWWbPp4bNXFLJxZVvG92ctxf7jQnzTQjF3n"</span>);
}

<span class="kw">pub mod </span>remove_deprecated_request_unit_ix {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"EfhYd3SafzGT472tYQDUc4dPd2xdEfKs5fwkowUgVt4W"</span>);
}

<span class="kw">pub mod </span>disable_rehash_for_rent_epoch {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"DTVTkmw3JSofd8CJVJte8PXEbxNQ2yZijvVr3pe2APPj"</span>);
}

<span class="kw">pub mod </span>increase_tx_account_lock_limit {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"9LZdXeKGeBV6hRLdxS1rHbHoEUsKqesCC2ZAPTPKJAbK"</span>);
}

<span class="kw">pub mod </span>limit_max_instruction_trace_length {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"GQALDaC48fEhZGWRj9iL5Q889emJKcj3aCvHF7VCbbF4"</span>);
}

<span class="kw">pub mod </span>check_syscall_outputs_do_not_overlap {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3uRVPBpyEJRo1emLCrq38eLRFGcu6uKSpUXqGvU8T7SZ"</span>);
}

<span class="kw">pub mod </span>enable_bpf_loader_set_authority_checked_ix {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"5x3825XS7M2A3Ekbn5VGGkvFoAg5qrRWkTrY4bARP1GL"</span>);
}

<span class="kw">pub mod </span>enable_alt_bn128_syscall {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"A16q37opZdQMCbe5qJ6xpBB9usykfv8jZaMkxvZQi4GJ"</span>);
}
<span class="kw">pub mod </span>enable_alt_bn128_compression_syscall {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"EJJewYSddEEtSZHiqugnvhQHiWyZKjkFDQASd7oKSagn"</span>);
}

<span class="kw">pub mod </span>enable_program_redeployment_cooldown {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"J4HFT8usBxpcF63y46t1upYobJgChmKyZPm5uTBRg25Z"</span>);
}

<span class="kw">pub mod </span>commission_updates_only_allowed_in_first_half_of_epoch {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"noRuG2kzACwgaY7TVmLRnUNPLKNVQE1fb7X55YWBehp"</span>);
}

<span class="kw">pub mod </span>enable_turbine_fanout_experiments {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"D31EFnLgdiysi84Woo3of4JMu7VmasUS3Z7j9HYXCeLY"</span>);
}

<span class="kw">pub mod </span>disable_turbine_fanout_experiments {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Gz1aLrbeQ4Q6PTSafCZcGWZXz91yVRi7ASFzFEr1U4sa"</span>);
}

<span class="kw">pub mod </span>move_serialized_len_ptr_in_cpi {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"74CoWuBmt3rUVUrCb2JiSTvh6nXyBWUsK4SaMj3CtE3T"</span>);
}

<span class="kw">pub mod </span>update_hashes_per_tick {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"3uFHb9oKdGfgZGJK9EHaAXN4USvnQtAFC13Fh5gGFS5B"</span>);
}

<span class="kw">pub mod </span>enable_big_mod_exp_syscall {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"EBq48m8irRKuE7ZnMTLvLg2UuGSqhe8s8oMqnmja1fJw"</span>);
}

<span class="kw">pub mod </span>disable_builtin_loader_ownership_chains {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"4UDcAfQ6EcA6bdcadkeHpkarkhZGJ7Bpq7wTAiRMjkoi"</span>);
}

<span class="kw">pub mod </span>cap_transaction_accounts_data_size {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"DdLwVYuvDz26JohmgSbA7mjpJFgX5zP2dkp8qsF2C33V"</span>);
}

<span class="kw">pub mod </span>remove_congestion_multiplier_from_fee_calculation {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"A8xyMHZovGXFkorFqEmVH2PKGLiBip5JD7jt4zsUWo4H"</span>);
}

<span class="kw">pub mod </span>enable_request_heap_frame_ix {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Hr1nUA9b7NJ6eChS26o7Vi8gYYDDwWD3YeBfzJkTbU86"</span>);
}

<span class="kw">pub mod </span>prevent_rent_paying_rent_recipients {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Fab5oP3DmsLYCiQZXdjyqT3ukFFPrsmqhXU4WU1AWVVF"</span>);
}

<span class="kw">pub mod </span>delay_visibility_of_program_deployment {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"GmuBvtFb2aHfSfMXpuFeWZGHyDeCLPS79s48fmCWCfM5"</span>);
}

<span class="kw">pub mod </span>apply_cost_tracker_during_replay {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"2ry7ygxiYURULZCrypHhveanvP5tzZ4toRwVp89oCNSj"</span>);
}
<span class="kw">pub mod </span>bpf_account_data_direct_mapping {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"EenyoWx9UMXYKpR8mW5Jmfmy2fRjzUtM7NduYMY8bx33"</span>);
}

<span class="kw">pub mod </span>add_set_tx_loaded_accounts_data_size_instruction {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"G6vbf1UBok8MWb8m25ex86aoQHeKTzDKzuZADHkShqm6"</span>);
}

<span class="kw">pub mod </span>switch_to_new_elf_parser {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Cdkc8PPTeTNUPoZEfCY5AyetUrEdkZtNPMgz58nqyaHD"</span>);
}

<span class="kw">pub mod </span>round_up_heap_size {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"CE2et8pqgyQMP2mQRg3CgvX8nJBKUArMu3wfiQiQKY1y"</span>);
}

<span class="kw">pub mod </span>remove_bpf_loader_incorrect_program_id {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"2HmTkCj9tXuPE4ueHzdD7jPeMf9JGCoZh5AsyoATiWEe"</span>);
}

<span class="kw">pub mod </span>include_loaded_accounts_data_size_in_fee_calculation {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"EaQpmC6GtRssaZ3PCUM5YksGqUdMLeZ46BQXYtHYakDS"</span>);
}

<span class="kw">pub mod </span>native_programs_consume_cu {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"8pgXCMNXC8qyEFypuwpXyRxLXZdpM4Qo72gJ6k87A6wL"</span>);
}

<span class="kw">pub mod </span>simplify_writable_program_account_check {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"5ZCcFAzJ1zsFKe1KSZa9K92jhx7gkcKj97ci2DBo1vwj"</span>);
}

<span class="kw">pub mod </span>stop_truncating_strings_in_syscalls {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"16FMCmgLzCNNz6eTwGanbyN2ZxvTBSLuQ6DZhgeMshg"</span>);
}

<span class="kw">pub mod </span>clean_up_delegation_errors {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Bj2jmUsM2iRhfdLLDSTkhM5UQRQvQHm57HSmPibPtEyu"</span>);
}

<span class="kw">pub mod </span>vote_state_add_vote_latency {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"7axKe5BTYBDD87ftzWbk5DfzWMGyRvqmWTduuo22Yaqy"</span>);
}

<span class="kw">pub mod </span>checked_arithmetic_in_fee_validation {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"5Pecy6ie6XGm22pc9d4P9W5c31BugcFBuy6hsP2zkETv"</span>);
}

<span class="kw">pub mod </span>last_restart_slot_sysvar {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"HooKD5NC9QNxk25QuzCssB8ecrEzGt6eXEPBUxWp1LaR"</span>);
}

<span class="kw">pub mod </span>reduce_stake_warmup_cooldown {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"GwtDQBghCTBgmX2cpEGNPxTEBUTQRaDMGTr5qychdGMj"</span>);
}

<span class="kw">pub mod </span>revise_turbine_epoch_stakes {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"BTWmtJC8U5ZLMbBUUA1k6As62sYjPEjAiNAT55xYGdJU"</span>);
}

<span class="kw">pub mod </span>enable_poseidon_syscall {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"FL9RsQA6TVUoh5xJQ9d936RHSebA1NLQqe3Zv9sXZRpr"</span>);
}

<span class="kw">pub mod </span>timely_vote_credits {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"tvcF6b1TRz353zKuhBjinZkKzjmihXmBAHJdjNYw1sQ"</span>);
}

<span class="kw">pub mod </span>remaining_compute_units_syscall_enabled {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"5TuppMutoyzhUSfuYdhgzD47F92GL1g89KpCZQKqedxP"</span>);
}

<span class="kw">pub mod </span>enable_program_runtime_v2_and_loader_v4 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"8oBxsYqnCvUTGzgEpxPcnVf7MLbWWPYddE33PftFeBBd"</span>);
}

<span class="kw">pub mod </span>require_rent_exempt_split_destination {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"D2aip4BBr8NPWtU9vLrwrBvbuaQ8w1zV38zFLxx4pfBV"</span>);
}

<span class="kw">pub mod </span>better_error_codes_for_tx_lamport_check {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"Ffswd3egL3tccB6Rv3XY6oqfdzn913vUcjCSnpvCKpfx"</span>);
}

<span class="kw">pub mod </span>update_hashes_per_tick2 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"EWme9uFqfy1ikK1jhJs8fM5hxWnK336QJpbscNtizkTU"</span>);
}

<span class="kw">pub mod </span>update_hashes_per_tick3 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"8C8MCtsab5SsfammbzvYz65HHauuUYdbY2DZ4sznH6h5"</span>);
}

<span class="kw">pub mod </span>update_hashes_per_tick4 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"8We4E7DPwF2WfAN8tRTtWQNhi98B99Qpuj7JoZ3Aikgg"</span>);
}

<span class="kw">pub mod </span>update_hashes_per_tick5 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"BsKLKAn1WM4HVhPRDsjosmqSg2J8Tq5xP2s2daDS6Ni4"</span>);
}

<span class="kw">pub mod </span>update_hashes_per_tick6 {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"FKu1qYwLQSiehz644H6Si65U5ZQ2cp9GxsyFUfYcuADv"</span>);
}

<span class="kw">pub mod </span>validate_fee_collector_account {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"prpFrMtgNmzaNzkPJg9o753fVvbHKqNrNTm76foJ2wm"</span>);
}

<span class="kw">pub mod </span>disable_rent_fees_collection {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"CJzY83ggJHqPGDq8VisV3U91jDJLuEaALZooBrXtnnLU"</span>);
}

<span class="kw">pub mod </span>enable_zk_transfer_with_fee {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"zkNLP7EQALfC1TYeB3biDU7akDckj8iPkvh9y2Mt2K3"</span>);
}

<span class="kw">pub mod </span>drop_legacy_shreds {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"GV49KKQdBNaiv2pgqhS2Dy3GWYJGXMTVYbYkdk91orRy"</span>);
}

<span class="kw">pub mod </span>allow_commission_decrease_at_any_time {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"decoMktMcnmiq6t3u7g5BfgcQu91nKZr6RvMYf9z1Jb"</span>);
}

<span class="kw">pub mod </span>consume_blockstore_duplicate_proofs {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"6YsBCejwK96GZCkJ6mkZ4b68oP63z2PLoQmWjC7ggTqZ"</span>);
}

<span class="kw">pub mod </span>index_erasure_conflict_duplicate_proofs {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"dupPajaLy2SSn8ko42aZz4mHANDNrLe8Nw8VQgFecLa"</span>);
}

<span class="kw">pub mod </span>merkle_conflict_duplicate_proofs {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"mrkPjRg79B2oK2ZLgd7S3AfEJaX9B6gAF3H9aEykRUS"</span>);
}

<span class="kw">pub mod </span>disable_bpf_loader_instructions {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"7WeS1vfPRgeeoXArLh7879YcB9mgE9ktjPDtajXeWfXn"</span>);
}

<span class="kw">pub mod </span>enable_zk_proof_from_account {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"zkiTNuzBKxrCLMKehzuQeKZyLtX2yvFcEKMML8nExU8"</span>);
}

<span class="kw">pub mod </span>cost_model_requested_write_lock_cost {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"wLckV1a64ngtcKPRGU4S4grVTestXjmNjxBjaKZrAcn"</span>);
}

<span class="kw">pub mod </span>enable_gossip_duplicate_proof_ingestion {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"FNKCMBzYUdjhHyPdsKG2LSmdzH8TCHXn3ytj8RNBS4nG"</span>);
}

<span class="kw">pub mod </span>enable_chained_merkle_shreds {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"7uZBkJXJ1HkuP6R3MJfZs7mLwymBcDbKdqbF51ZWLier"</span>);
}

<span class="kw">pub mod </span>deprecate_unused_legacy_vote_plumbing {
    <span class="macro">solana_sdk::declare_id!</span>(<span class="string">"6Uf8S75PVh91MYgPQSHnjRAPQq6an5BDv9vomrCwDqLe"</span>);
}

<span class="macro">lazy_static!</span> {
    <span class="doccomment">/// Map of feature identifiers to user-visible description
    </span><span class="kw">pub static </span><span class="kw-2">ref </span>FEATURE_NAMES: HashMap&lt;Pubkey, <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str&gt; = [
        (secp256k1_program_enabled::id(), <span class="string">"secp256k1 program"</span>),
        (deprecate_rewards_sysvar::id(), <span class="string">"deprecate unused rewards sysvar"</span>),
        (pico_inflation::id(), <span class="string">"pico inflation"</span>),
        (full_inflation::devnet_and_testnet::id(), <span class="string">"full inflation on devnet and testnet"</span>),
        (spl_token_v2_multisig_fix::id(), <span class="string">"spl-token multisig fix"</span>),
        (no_overflow_rent_distribution::id(), <span class="string">"no overflow rent distribution"</span>),
        (filter_stake_delegation_accounts::id(), <span class="string">"filter stake_delegation_accounts #14062"</span>),
        (require_custodian_for_locked_stake_authorize::id(), <span class="string">"require custodian to authorize withdrawer change for locked stake"</span>),
        (spl_token_v2_self_transfer_fix::id(), <span class="string">"spl-token self-transfer fix"</span>),
        (full_inflation::mainnet::certusone::enable::id(), <span class="string">"full inflation enabled by Certus One"</span>),
        (full_inflation::mainnet::certusone::vote::id(), <span class="string">"community vote allowing Certus One to enable full inflation"</span>),
        (warp_timestamp_again::id(), <span class="string">"warp timestamp again, adjust bounding to 25% fast 80% slow #15204"</span>),
        (check_init_vote_data::id(), <span class="string">"check initialized Vote data"</span>),
        (secp256k1_recover_syscall_enabled::id(), <span class="string">"secp256k1_recover syscall"</span>),
        (system_transfer_zero_check::id(), <span class="string">"perform all checks for transfers of 0 lamports"</span>),
        (blake3_syscall_enabled::id(), <span class="string">"blake3 syscall"</span>),
        (dedupe_config_program_signers::id(), <span class="string">"dedupe config program signers"</span>),
        (verify_tx_signatures_len::id(), <span class="string">"prohibit extra transaction signatures"</span>),
        (vote_stake_checked_instructions::id(), <span class="string">"vote/state program checked instructions #18345"</span>),
        (rent_for_sysvars::id(), <span class="string">"collect rent from accounts owned by sysvars"</span>),
        (libsecp256k1_0_5_upgrade_enabled::id(), <span class="string">"upgrade libsecp256k1 to v0.5.0"</span>),
        (tx_wide_compute_cap::id(), <span class="string">"transaction wide compute cap"</span>),
        (spl_token_v2_set_authority_fix::id(), <span class="string">"spl-token set_authority fix"</span>),
        (merge_nonce_error_into_system_error::id(), <span class="string">"merge NonceError into SystemError"</span>),
        (disable_fees_sysvar::id(), <span class="string">"disable fees sysvar"</span>),
        (stake_merge_with_unmatched_credits_observed::id(), <span class="string">"allow merging active stakes with unmatched credits_observed #18985"</span>),
        (zk_token_sdk_enabled::id(), <span class="string">"enable Zk Token proof program and syscalls"</span>),
        (curve25519_syscall_enabled::id(), <span class="string">"enable curve25519 syscalls"</span>),
        (versioned_tx_message_enabled::id(), <span class="string">"enable versioned transaction message processing"</span>),
        (libsecp256k1_fail_on_bad_count::id(), <span class="string">"fail libsecp256k1_verify if count appears wrong"</span>),
        (libsecp256k1_fail_on_bad_count2::id(), <span class="string">"fail libsecp256k1_verify if count appears wrong"</span>),
        (instructions_sysvar_owned_by_sysvar::id(), <span class="string">"fix owner for instructions sysvar"</span>),
        (stake_program_advance_activating_credits_observed::id(), <span class="string">"Enable advancing credits observed for activation epoch #19309"</span>),
        (credits_auto_rewind::id(), <span class="string">"Auto rewind stake's credits_observed if (accidental) vote recreation is detected #22546"</span>),
        (demote_program_write_locks::id(), <span class="string">"demote program write locks to readonly, except when upgradeable loader present #19593 #20265"</span>),
        (ed25519_program_enabled::id(), <span class="string">"enable builtin ed25519 signature verify program"</span>),
        (return_data_syscall_enabled::id(), <span class="string">"enable sol_{set,get}_return_data syscall"</span>),
        (reduce_required_deploy_balance::id(), <span class="string">"reduce required payer balance for program deploys"</span>),
        (sol_log_data_syscall_enabled::id(), <span class="string">"enable sol_log_data syscall"</span>),
        (stakes_remove_delegation_if_inactive::id(), <span class="string">"remove delegations from stakes cache when inactive"</span>),
        (do_support_realloc::id(), <span class="string">"support account data reallocation"</span>),
        (prevent_calling_precompiles_as_programs::id(), <span class="string">"prevent calling precompiles as programs"</span>),
        (optimize_epoch_boundary_updates::id(), <span class="string">"optimize epoch boundary updates"</span>),
        (remove_native_loader::id(), <span class="string">"remove support for the native loader"</span>),
        (send_to_tpu_vote_port::id(), <span class="string">"send votes to the tpu vote port"</span>),
        (requestable_heap_size::id(), <span class="string">"Requestable heap frame size"</span>),
        (disable_fee_calculator::id(), <span class="string">"deprecate fee calculator"</span>),
        (add_compute_budget_program::id(), <span class="string">"Add compute_budget_program"</span>),
        (nonce_must_be_writable::id(), <span class="string">"nonce must be writable"</span>),
        (spl_token_v3_3_0_release::id(), <span class="string">"spl-token v3.3.0 release"</span>),
        (leave_nonce_on_success::id(), <span class="string">"leave nonce as is on success"</span>),
        (reject_empty_instruction_without_program::id(), <span class="string">"fail instructions which have native_loader as program_id directly"</span>),
        (fixed_memcpy_nonoverlapping_check::id(), <span class="string">"use correct check for nonoverlapping regions in memcpy syscall"</span>),
        (reject_non_rent_exempt_vote_withdraws::id(), <span class="string">"fail vote withdraw instructions which leave the account non-rent-exempt"</span>),
        (evict_invalid_stakes_cache_entries::id(), <span class="string">"evict invalid stakes cache entries on epoch boundaries"</span>),
        (allow_votes_to_directly_update_vote_state::id(), <span class="string">"enable direct vote state update"</span>),
        (cap_accounts_data_len::id(), <span class="string">"cap the accounts data len"</span>),
        (max_tx_account_locks::id(), <span class="string">"enforce max number of locked accounts per transaction"</span>),
        (require_rent_exempt_accounts::id(), <span class="string">"require all new transaction accounts with data to be rent-exempt"</span>),
        (filter_votes_outside_slot_hashes::id(), <span class="string">"filter vote slots older than the slot hashes history"</span>),
        (update_syscall_base_costs::id(), <span class="string">"update syscall base costs"</span>),
        (stake_deactivate_delinquent_instruction::id(), <span class="string">"enable the deactivate delinquent stake instruction #23932"</span>),
        (vote_withdraw_authority_may_change_authorized_voter::id(), <span class="string">"vote account withdraw authority may change the authorized voter #22521"</span>),
        (spl_associated_token_account_v1_0_4::id(), <span class="string">"SPL Associated Token Account Program release version 1.0.4, tied to token 3.3.0 #22648"</span>),
        (reject_vote_account_close_unless_zero_credit_epoch::id(), <span class="string">"fail vote account withdraw to 0 unless account earned 0 credits in last completed epoch"</span>),
        (add_get_processed_sibling_instruction_syscall::id(), <span class="string">"add add_get_processed_sibling_instruction_syscall"</span>),
        (bank_transaction_count_fix::id(), <span class="string">"fixes Bank::transaction_count to include all committed transactions, not just successful ones"</span>),
        (disable_bpf_deprecated_load_instructions::id(), <span class="string">"disable ldabs* and ldind* SBF instructions"</span>),
        (disable_bpf_unresolved_symbols_at_runtime::id(), <span class="string">"disable reporting of unresolved SBF symbols at runtime"</span>),
        (record_instruction_in_transaction_context_push::id(), <span class="string">"move the CPI stack overflow check to the end of push"</span>),
        (syscall_saturated_math::id(), <span class="string">"syscalls use saturated math"</span>),
        (check_physical_overlapping::id(), <span class="string">"check physical overlapping regions"</span>),
        (limit_secp256k1_recovery_id::id(), <span class="string">"limit secp256k1 recovery id"</span>),
        (disable_deprecated_loader::id(), <span class="string">"disable the deprecated BPF loader"</span>),
        (check_slice_translation_size::id(), <span class="string">"check size when translating slices"</span>),
        (stake_split_uses_rent_sysvar::id(), <span class="string">"stake split instruction uses rent sysvar"</span>),
        (add_get_minimum_delegation_instruction_to_stake_program::id(), <span class="string">"add GetMinimumDelegation instruction to stake program"</span>),
        (error_on_syscall_bpf_function_hash_collisions::id(), <span class="string">"error on bpf function hash collisions"</span>),
        (reject_callx_r10::id(), <span class="string">"Reject bpf callx r10 instructions"</span>),
        (drop_redundant_turbine_path::id(), <span class="string">"drop redundant turbine path"</span>),
        (executables_incur_cpi_data_cost::id(), <span class="string">"Executables incur CPI data costs"</span>),
        (fix_recent_blockhashes::id(), <span class="string">"stop adding hashes for skipped slots to recent blockhashes"</span>),
        (update_rewards_from_cached_accounts::id(), <span class="string">"update rewards from cached accounts"</span>),
        (enable_partitioned_epoch_reward::id(), <span class="string">"enable partitioned rewards at epoch boundary #32166"</span>),
        (spl_token_v3_4_0::id(), <span class="string">"SPL Token Program version 3.4.0 release #24740"</span>),
        (spl_associated_token_account_v1_1_0::id(), <span class="string">"SPL Associated Token Account Program version 1.1.0 release #24741"</span>),
        (default_units_per_instruction::id(), <span class="string">"Default max tx-wide compute units calculated per instruction"</span>),
        (stake_allow_zero_undelegated_amount::id(), <span class="string">"Allow zero-lamport undelegated amount for initialized stakes #24670"</span>),
        (require_static_program_ids_in_transaction::id(), <span class="string">"require static program ids in versioned transactions"</span>),
        (stake_raise_minimum_delegation_to_1_sol::id(), <span class="string">"Raise minimum stake delegation to 1.0 SOL #24357"</span>),
        (stake_minimum_delegation_for_rewards::id(), <span class="string">"stakes must be at least the minimum delegation to earn rewards"</span>),
        (add_set_compute_unit_price_ix::id(), <span class="string">"add compute budget ix for setting a compute unit price"</span>),
        (disable_deploy_of_alloc_free_syscall::id(), <span class="string">"disable new deployments of deprecated sol_alloc_free_ syscall"</span>),
        (include_account_index_in_rent_error::id(), <span class="string">"include account index in rent tx error #25190"</span>),
        (add_shred_type_to_shred_seed::id(), <span class="string">"add shred-type to shred seed #25556"</span>),
        (warp_timestamp_with_a_vengeance::id(), <span class="string">"warp timestamp again, adjust bounding to 150% slow #25666"</span>),
        (separate_nonce_from_blockhash::id(), <span class="string">"separate durable nonce and blockhash domains #25744"</span>),
        (enable_durable_nonce::id(), <span class="string">"enable durable nonce #25744"</span>),
        (vote_state_update_credit_per_dequeue::id(), <span class="string">"Calculate vote credits for VoteStateUpdate per vote dequeue to match credit awards for Vote instruction"</span>),
        (quick_bail_on_panic::id(), <span class="string">"quick bail on panic"</span>),
        (nonce_must_be_authorized::id(), <span class="string">"nonce must be authorized"</span>),
        (nonce_must_be_advanceable::id(), <span class="string">"durable nonces must be advanceable"</span>),
        (vote_authorize_with_seed::id(), <span class="string">"An instruction you can use to change a vote accounts authority when the current authority is a derived key #25860"</span>),
        (cap_accounts_data_size_per_block::id(), <span class="string">"cap the accounts data size per block #25517"</span>),
        (stake_redelegate_instruction::id(), <span class="string">"enable the redelegate stake instruction #26294"</span>),
        (preserve_rent_epoch_for_rent_exempt_accounts::id(), <span class="string">"preserve rent epoch for rent exempt accounts #26479"</span>),
        (enable_bpf_loader_extend_program_ix::id(), <span class="string">"enable bpf upgradeable loader ExtendProgram instruction #25234"</span>),
        (skip_rent_rewrites::id(), <span class="string">"skip rewriting rent exempt accounts during rent collection #26491"</span>),
        (enable_early_verification_of_account_modifications::id(), <span class="string">"enable early verification of account modifications #25899"</span>),
        (disable_rehash_for_rent_epoch::id(), <span class="string">"on accounts hash calculation, do not try to rehash accounts #28934"</span>),
        (account_hash_ignore_slot::id(), <span class="string">"ignore slot when calculating an account hash #28420"</span>),
        (set_exempt_rent_epoch_max::id(), <span class="string">"set rent epoch to Epoch::MAX for rent-exempt accounts #28683"</span>),
        (on_load_preserve_rent_epoch_for_rent_exempt_accounts::id(), <span class="string">"on bank load account, do not try to fix up rent_epoch #28541"</span>),
        (prevent_crediting_accounts_that_end_rent_paying::id(), <span class="string">"prevent crediting rent paying accounts #26606"</span>),
        (cap_bpf_program_instruction_accounts::id(), <span class="string">"enforce max number of accounts per bpf program instruction #26628"</span>),
        (loosen_cpi_size_restriction::id(), <span class="string">"loosen cpi size restrictions #26641"</span>),
        (use_default_units_in_fee_calculation::id(), <span class="string">"use default units per instruction in fee calculation #26785"</span>),
        (compact_vote_state_updates::id(), <span class="string">"Compact vote state updates to lower block size"</span>),
        (incremental_snapshot_only_incremental_hash_calculation::id(), <span class="string">"only hash accounts in incremental snapshot during incremental snapshot creation #26799"</span>),
        (disable_cpi_setting_executable_and_rent_epoch::id(), <span class="string">"disable setting is_executable and_rent_epoch in CPI #26987"</span>),
        (relax_authority_signer_check_for_lookup_table_creation::id(), <span class="string">"relax authority signer check for lookup table creation #27205"</span>),
        (stop_sibling_instruction_search_at_parent::id(), <span class="string">"stop the search in get_processed_sibling_instruction when the parent instruction is reached #27289"</span>),
        (vote_state_update_root_fix::id(), <span class="string">"fix root in vote state updates #27361"</span>),
        (cap_accounts_data_allocations_per_transaction::id(), <span class="string">"cap accounts data allocations per transaction #27375"</span>),
        (epoch_accounts_hash::id(), <span class="string">"enable epoch accounts hash calculation #27539"</span>),
        (remove_deprecated_request_unit_ix::id(), <span class="string">"remove support for RequestUnitsDeprecated instruction #27500"</span>),
        (increase_tx_account_lock_limit::id(), <span class="string">"increase tx account lock limit to 128 #27241"</span>),
        (limit_max_instruction_trace_length::id(), <span class="string">"limit max instruction trace length #27939"</span>),
        (check_syscall_outputs_do_not_overlap::id(), <span class="string">"check syscall outputs do_not overlap #28600"</span>),
        (enable_bpf_loader_set_authority_checked_ix::id(), <span class="string">"enable bpf upgradeable loader SetAuthorityChecked instruction #28424"</span>),
        (enable_alt_bn128_syscall::id(), <span class="string">"add alt_bn128 syscalls #27961"</span>),
        (enable_program_redeployment_cooldown::id(), <span class="string">"enable program redeployment cooldown #29135"</span>),
        (commission_updates_only_allowed_in_first_half_of_epoch::id(), <span class="string">"validator commission updates are only allowed in the first half of an epoch #29362"</span>),
        (enable_turbine_fanout_experiments::id(), <span class="string">"enable turbine fanout experiments #29393"</span>),
        (disable_turbine_fanout_experiments::id(), <span class="string">"disable turbine fanout experiments #29393"</span>),
        (move_serialized_len_ptr_in_cpi::id(), <span class="string">"cpi ignore serialized_len_ptr #29592"</span>),
        (update_hashes_per_tick::id(), <span class="string">"Update desired hashes per tick on epoch boundary"</span>),
        (enable_big_mod_exp_syscall::id(), <span class="string">"add big_mod_exp syscall #28503"</span>),
        (disable_builtin_loader_ownership_chains::id(), <span class="string">"disable builtin loader ownership chains #29956"</span>),
        (cap_transaction_accounts_data_size::id(), <span class="string">"cap transaction accounts data size up to a limit #27839"</span>),
        (remove_congestion_multiplier_from_fee_calculation::id(), <span class="string">"Remove congestion multiplier from transaction fee calculation #29881"</span>),
        (enable_request_heap_frame_ix::id(), <span class="string">"Enable transaction to request heap frame using compute budget instruction #30076"</span>),
        (prevent_rent_paying_rent_recipients::id(), <span class="string">"prevent recipients of rent rewards from ending in rent-paying state #30151"</span>),
        (delay_visibility_of_program_deployment::id(), <span class="string">"delay visibility of program upgrades #30085"</span>),
        (apply_cost_tracker_during_replay::id(), <span class="string">"apply cost tracker to blocks during replay #29595"</span>),
        (add_set_tx_loaded_accounts_data_size_instruction::id(), <span class="string">"add compute budget instruction for setting account data size per transaction #30366"</span>),
        (switch_to_new_elf_parser::id(), <span class="string">"switch to new ELF parser #30497"</span>),
        (round_up_heap_size::id(), <span class="string">"round up heap size when calculating heap cost #30679"</span>),
        (remove_bpf_loader_incorrect_program_id::id(), <span class="string">"stop incorrectly throwing IncorrectProgramId in bpf_loader #30747"</span>),
        (include_loaded_accounts_data_size_in_fee_calculation::id(), <span class="string">"include transaction loaded accounts data size in base fee calculation #30657"</span>),
        (native_programs_consume_cu::id(), <span class="string">"Native program should consume compute units #30620"</span>),
        (simplify_writable_program_account_check::id(), <span class="string">"Simplify checks performed for writable upgradeable program accounts #30559"</span>),
        (stop_truncating_strings_in_syscalls::id(), <span class="string">"Stop truncating strings in syscalls #31029"</span>),
        (clean_up_delegation_errors::id(), <span class="string">"Return InsufficientDelegation instead of InsufficientFunds or InsufficientStake where applicable #31206"</span>),
        (vote_state_add_vote_latency::id(), <span class="string">"replace Lockout with LandedVote (including vote latency) in vote state #31264"</span>),
        (checked_arithmetic_in_fee_validation::id(), <span class="string">"checked arithmetic in fee validation #31273"</span>),
        (bpf_account_data_direct_mapping::id(), <span class="string">"use memory regions to map account data into the rbpf vm instead of copying the data"</span>),
        (last_restart_slot_sysvar::id(), <span class="string">"enable new sysvar last_restart_slot"</span>),
        (reduce_stake_warmup_cooldown::id(), <span class="string">"reduce stake warmup cooldown from 25% to 9%"</span>),
        (revise_turbine_epoch_stakes::id(), <span class="string">"revise turbine epoch stakes"</span>),
        (enable_poseidon_syscall::id(), <span class="string">"Enable Poseidon syscall"</span>),
        (timely_vote_credits::id(), <span class="string">"use timeliness of votes in determining credits to award"</span>),
        (remaining_compute_units_syscall_enabled::id(), <span class="string">"enable the remaining_compute_units syscall"</span>),
        (enable_program_runtime_v2_and_loader_v4::id(), <span class="string">"Enable Program-Runtime-v2 and Loader-v4 #33293"</span>),
        (require_rent_exempt_split_destination::id(), <span class="string">"Require stake split destination account to be rent exempt"</span>),
        (better_error_codes_for_tx_lamport_check::id(), <span class="string">"better error codes for tx lamport check #33353"</span>),
        (enable_alt_bn128_compression_syscall::id(), <span class="string">"add alt_bn128 compression syscalls"</span>),
        (update_hashes_per_tick2::id(), <span class="string">"Update desired hashes per tick to 2.8M"</span>),
        (update_hashes_per_tick3::id(), <span class="string">"Update desired hashes per tick to 4.4M"</span>),
        (update_hashes_per_tick4::id(), <span class="string">"Update desired hashes per tick to 7.6M"</span>),
        (update_hashes_per_tick5::id(), <span class="string">"Update desired hashes per tick to 9.2M"</span>),
        (update_hashes_per_tick6::id(), <span class="string">"Update desired hashes per tick to 10M"</span>),
        (validate_fee_collector_account::id(), <span class="string">"validate fee collector account #33888"</span>),
        (disable_rent_fees_collection::id(), <span class="string">"Disable rent fees collection #33945"</span>),
        (enable_zk_transfer_with_fee::id(), <span class="string">"enable Zk Token proof program transfer with fee"</span>),
        (drop_legacy_shreds::id(), <span class="string">"drops legacy shreds #34328"</span>),
        (allow_commission_decrease_at_any_time::id(), <span class="string">"Allow commission decrease at any time in epoch #33843"</span>),
        (consume_blockstore_duplicate_proofs::id(), <span class="string">"consume duplicate proofs from blockstore in consensus #34372"</span>),
        (index_erasure_conflict_duplicate_proofs::id(), <span class="string">"generate duplicate proofs for index and erasure conflicts #34360"</span>),
        (merkle_conflict_duplicate_proofs::id(), <span class="string">"generate duplicate proofs for merkle root conflicts #34270"</span>),
        (disable_bpf_loader_instructions::id(), <span class="string">"disable bpf loader management instructions #34194"</span>),
        (enable_zk_proof_from_account::id(), <span class="string">"Enable zk token proof program to read proof from accounts instead of instruction data #34750"</span>),
        (curve25519_restrict_msm_length::id(), <span class="string">"restrict curve25519 multiscalar multiplication vector lengths #34763"</span>),
        (cost_model_requested_write_lock_cost::id(), <span class="string">"cost model uses number of requested write locks #34819"</span>),
        (enable_gossip_duplicate_proof_ingestion::id(), <span class="string">"enable gossip duplicate proof ingestion #32963"</span>),
        (enable_chained_merkle_shreds::id(), <span class="string">"Enable chained Merkle shreds #34916"</span>),
        (deprecate_unused_legacy_vote_plumbing::id(), <span class="string">"Deprecate unused legacy vote tx plumbing"</span>),
        <span class="comment">/*************** ADD NEW FEATURES HERE ***************/
    </span>]
    .iter()
    .cloned()
    .collect();

    <span class="doccomment">/// Unique identifier of the current software's feature set
    </span><span class="kw">pub static </span><span class="kw-2">ref </span>ID: Hash = {
        <span class="kw">let </span><span class="kw-2">mut </span>hasher = Hasher::default();
        <span class="kw">let </span><span class="kw-2">mut </span>feature_ids = FEATURE_NAMES.keys().collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        feature_ids.sort();
        <span class="kw">for </span>feature <span class="kw">in </span>feature_ids {
            hasher.hash(feature.as_ref());
        }
        hasher.result()
    };
}

<span class="attr">#[derive(Clone, PartialEq, Eq, Hash)]
</span><span class="kw">pub struct </span>FullInflationFeaturePair {
    <span class="kw">pub </span>vote_id: Pubkey, <span class="comment">// Feature that grants the candidate the ability to enable full inflation
    </span><span class="kw">pub </span>enable_id: Pubkey, <span class="comment">// Feature to enable full inflation by the candidate
</span>}

<span class="macro">lazy_static!</span> {
    <span class="doccomment">/// Set of feature pairs that once enabled will trigger full inflation
    </span><span class="kw">pub static </span><span class="kw-2">ref </span>FULL_INFLATION_FEATURE_PAIRS: HashSet&lt;FullInflationFeaturePair&gt; = [
        FullInflationFeaturePair {
            vote_id: full_inflation::mainnet::certusone::vote::id(),
            enable_id: full_inflation::mainnet::certusone::enable::id(),
        },
    ]
    .iter()
    .cloned()
    .collect();
}

<span class="doccomment">/// `FeatureSet` holds the set of currently active/inactive runtime features
</span><span class="attr">#[derive(AbiExample, Debug, Clone, Eq, PartialEq)]
</span><span class="kw">pub struct </span>FeatureSet {
    <span class="kw">pub </span>active: HashMap&lt;Pubkey, Slot&gt;,
    <span class="kw">pub </span>inactive: HashSet&lt;Pubkey&gt;,
}
<span class="kw">impl </span>Default <span class="kw">for </span>FeatureSet {
    <span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        <span class="comment">// All features disabled
        </span><span class="self">Self </span>{
            active: HashMap::new(),
            inactive: FEATURE_NAMES.keys().cloned().collect(),
        }
    }
}
<span class="kw">impl </span>FeatureSet {
    <span class="kw">pub fn </span>is_active(<span class="kw-2">&amp;</span><span class="self">self</span>, feature_id: <span class="kw-2">&amp;</span>Pubkey) -&gt; bool {
        <span class="self">self</span>.active.contains_key(feature_id)
    }

    <span class="kw">pub fn </span>activated_slot(<span class="kw-2">&amp;</span><span class="self">self</span>, feature_id: <span class="kw-2">&amp;</span>Pubkey) -&gt; <span class="prelude-ty">Option</span>&lt;Slot&gt; {
        <span class="self">self</span>.active.get(feature_id).copied()
    }

    <span class="doccomment">/// List of enabled features that trigger full inflation
    </span><span class="kw">pub fn </span>full_inflation_features_enabled(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; HashSet&lt;Pubkey&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>hash_set = FULL_INFLATION_FEATURE_PAIRS
            .iter()
            .filter_map(|pair| {
                <span class="kw">if </span><span class="self">self</span>.is_active(<span class="kw-2">&amp;</span>pair.vote_id) &amp;&amp; <span class="self">self</span>.is_active(<span class="kw-2">&amp;</span>pair.enable_id) {
                    <span class="prelude-val">Some</span>(pair.enable_id)
                } <span class="kw">else </span>{
                    <span class="prelude-val">None
                </span>}
            })
            .collect::&lt;HashSet&lt;<span class="kw">_</span>&gt;&gt;();

        <span class="kw">if </span><span class="self">self</span>.is_active(<span class="kw-2">&amp;</span>full_inflation::devnet_and_testnet::id()) {
            hash_set.insert(full_inflation::devnet_and_testnet::id());
        }
        hash_set
    }

    <span class="doccomment">/// All features enabled, useful for testing
    </span><span class="kw">pub fn </span>all_enabled() -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            active: FEATURE_NAMES.keys().cloned().map(|key| (key, <span class="number">0</span>)).collect(),
            inactive: HashSet::new(),
        }
    }

    <span class="doccomment">/// Activate a feature
    </span><span class="kw">pub fn </span>activate(<span class="kw-2">&amp;mut </span><span class="self">self</span>, feature_id: <span class="kw-2">&amp;</span>Pubkey, slot: u64) {
        <span class="self">self</span>.inactive.remove(feature_id);
        <span class="self">self</span>.active.insert(<span class="kw-2">*</span>feature_id, slot);
    }

    <span class="doccomment">/// Deactivate a feature
    </span><span class="kw">pub fn </span>deactivate(<span class="kw-2">&amp;mut </span><span class="self">self</span>, feature_id: <span class="kw-2">&amp;</span>Pubkey) {
        <span class="self">self</span>.active.remove(feature_id);
        <span class="self">self</span>.inactive.insert(<span class="kw-2">*</span>feature_id);
    }

    <span class="kw">pub fn </span>new_warmup_cooldown_rate_epoch(<span class="kw-2">&amp;</span><span class="self">self</span>, epoch_schedule: <span class="kw-2">&amp;</span>EpochSchedule) -&gt; <span class="prelude-ty">Option</span>&lt;Epoch&gt; {
        <span class="self">self</span>.activated_slot(<span class="kw-2">&amp;</span>reduce_stake_warmup_cooldown::id())
            .map(|slot| epoch_schedule.get_epoch(slot))
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>test {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_full_inflation_features_enabled_devnet_and_testnet() {
        <span class="kw">let </span><span class="kw-2">mut </span>feature_set = FeatureSet::default();
        <span class="macro">assert!</span>(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::devnet_and_testnet::id(), <span class="number">42</span>);
        <span class="macro">assert_eq!</span>(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::devnet_and_testnet::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_full_inflation_features_enabled() {
        <span class="comment">// Normal sequence: vote_id then enable_id
        </span><span class="kw">let </span><span class="kw-2">mut </span>feature_set = FeatureSet::default();
        <span class="macro">assert!</span>(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), <span class="number">42</span>);
        <span class="macro">assert!</span>(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), <span class="number">42</span>);
        <span class="macro">assert_eq!</span>(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );

        <span class="comment">// Backwards sequence: enable_id and then vote_id
        </span><span class="kw">let </span><span class="kw-2">mut </span>feature_set = FeatureSet::default();
        <span class="macro">assert!</span>(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), <span class="number">42</span>);
        <span class="macro">assert!</span>(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), <span class="number">42</span>);
        <span class="macro">assert_eq!</span>(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_feature_set_activate_deactivate() {
        <span class="kw">let </span><span class="kw-2">mut </span>feature_set = FeatureSet::default();

        <span class="kw">let </span>feature = Pubkey::new_unique();
        <span class="macro">assert!</span>(!feature_set.is_active(<span class="kw-2">&amp;</span>feature));
        feature_set.activate(<span class="kw-2">&amp;</span>feature, <span class="number">0</span>);
        <span class="macro">assert!</span>(feature_set.is_active(<span class="kw-2">&amp;</span>feature));
        feature_set.deactivate(<span class="kw-2">&amp;</span>feature);
        <span class="macro">assert!</span>(!feature_set.is_active(<span class="kw-2">&amp;</span>feature));
    }
}
</code></pre></div></section></main></body></html>