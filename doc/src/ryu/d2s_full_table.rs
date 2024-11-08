<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ryu-1.0.17/src/d2s_full_table.rs`."><title>d2s_full_table.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="ryu" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../ryu/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// Translated from C to Rust. The original C code can be found at
// https://github.com/ulfjack/ryu and carries the following license:
//
// Copyright 2018 Ulf Adams
//
// The contents of this file may be used under the terms of the Apache License,
// Version 2.0.
//
//    (See accompanying file LICENSE-Apache or copy at
//     http://www.apache.org/licenses/LICENSE-2.0)
//
// Alternatively, the contents of this file may be used under the terms of
// the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE-Boost or copy at
//     https://www.boost.org/LICENSE_1_0.txt)
//
// Unless required by applicable law or agreed to in writing, this software
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.

</span><span class="kw">const </span>DOUBLE_POW5_INV_TABLE_SIZE: usize = <span class="number">342</span>;
<span class="kw">const </span>DOUBLE_POW5_TABLE_SIZE: usize = <span class="number">326</span>;

<span class="kw">pub static </span>DOUBLE_POW5_INV_SPLIT: [(u64, u64); DOUBLE_POW5_INV_TABLE_SIZE] = [
    (<span class="number">1</span>, <span class="number">2305843009213693952</span>),
    (<span class="number">11068046444225730970</span>, <span class="number">1844674407370955161</span>),
    (<span class="number">5165088340638674453</span>, <span class="number">1475739525896764129</span>),
    (<span class="number">7821419487252849886</span>, <span class="number">1180591620717411303</span>),
    (<span class="number">8824922364862649494</span>, <span class="number">1888946593147858085</span>),
    (<span class="number">7059937891890119595</span>, <span class="number">1511157274518286468</span>),
    (<span class="number">13026647942995916322</span>, <span class="number">1208925819614629174</span>),
    (<span class="number">9774590264567735146</span>, <span class="number">1934281311383406679</span>),
    (<span class="number">11509021026396098440</span>, <span class="number">1547425049106725343</span>),
    (<span class="number">16585914450600699399</span>, <span class="number">1237940039285380274</span>),
    (<span class="number">15469416676735388068</span>, <span class="number">1980704062856608439</span>),
    (<span class="number">16064882156130220778</span>, <span class="number">1584563250285286751</span>),
    (<span class="number">9162556910162266299</span>, <span class="number">1267650600228229401</span>),
    (<span class="number">7281393426775805432</span>, <span class="number">2028240960365167042</span>),
    (<span class="number">16893161185646375315</span>, <span class="number">1622592768292133633</span>),
    (<span class="number">2446482504291369283</span>, <span class="number">1298074214633706907</span>),
    (<span class="number">7603720821608101175</span>, <span class="number">2076918743413931051</span>),
    (<span class="number">2393627842544570617</span>, <span class="number">1661534994731144841</span>),
    (<span class="number">16672297533003297786</span>, <span class="number">1329227995784915872</span>),
    (<span class="number">11918280793837635165</span>, <span class="number">2126764793255865396</span>),
    (<span class="number">5845275820328197809</span>, <span class="number">1701411834604692317</span>),
    (<span class="number">15744267100488289217</span>, <span class="number">1361129467683753853</span>),
    (<span class="number">3054734472329800808</span>, <span class="number">2177807148294006166</span>),
    (<span class="number">17201182836831481939</span>, <span class="number">1742245718635204932</span>),
    (<span class="number">6382248639981364905</span>, <span class="number">1393796574908163946</span>),
    (<span class="number">2832900194486363201</span>, <span class="number">2230074519853062314</span>),
    (<span class="number">5955668970331000884</span>, <span class="number">1784059615882449851</span>),
    (<span class="number">1075186361522890384</span>, <span class="number">1427247692705959881</span>),
    (<span class="number">12788344622662355584</span>, <span class="number">2283596308329535809</span>),
    (<span class="number">13920024512871794791</span>, <span class="number">1826877046663628647</span>),
    (<span class="number">3757321980813615186</span>, <span class="number">1461501637330902918</span>),
    (<span class="number">10384555214134712795</span>, <span class="number">1169201309864722334</span>),
    (<span class="number">5547241898389809503</span>, <span class="number">1870722095783555735</span>),
    (<span class="number">4437793518711847602</span>, <span class="number">1496577676626844588</span>),
    (<span class="number">10928932444453298728</span>, <span class="number">1197262141301475670</span>),
    (<span class="number">17486291911125277965</span>, <span class="number">1915619426082361072</span>),
    (<span class="number">6610335899416401726</span>, <span class="number">1532495540865888858</span>),
    (<span class="number">12666966349016942027</span>, <span class="number">1225996432692711086</span>),
    (<span class="number">12888448528943286597</span>, <span class="number">1961594292308337738</span>),
    (<span class="number">17689456452638449924</span>, <span class="number">1569275433846670190</span>),
    (<span class="number">14151565162110759939</span>, <span class="number">1255420347077336152</span>),
    (<span class="number">7885109000409574610</span>, <span class="number">2008672555323737844</span>),
    (<span class="number">9997436015069570011</span>, <span class="number">1606938044258990275</span>),
    (<span class="number">7997948812055656009</span>, <span class="number">1285550435407192220</span>),
    (<span class="number">12796718099289049614</span>, <span class="number">2056880696651507552</span>),
    (<span class="number">2858676849947419045</span>, <span class="number">1645504557321206042</span>),
    (<span class="number">13354987924183666206</span>, <span class="number">1316403645856964833</span>),
    (<span class="number">17678631863951955605</span>, <span class="number">2106245833371143733</span>),
    (<span class="number">3074859046935833515</span>, <span class="number">1684996666696914987</span>),
    (<span class="number">13527933681774397782</span>, <span class="number">1347997333357531989</span>),
    (<span class="number">10576647446613305481</span>, <span class="number">2156795733372051183</span>),
    (<span class="number">15840015586774465031</span>, <span class="number">1725436586697640946</span>),
    (<span class="number">8982663654677661702</span>, <span class="number">1380349269358112757</span>),
    (<span class="number">18061610662226169046</span>, <span class="number">2208558830972980411</span>),
    (<span class="number">10759939715039024913</span>, <span class="number">1766847064778384329</span>),
    (<span class="number">12297300586773130254</span>, <span class="number">1413477651822707463</span>),
    (<span class="number">15986332124095098083</span>, <span class="number">2261564242916331941</span>),
    (<span class="number">9099716884534168143</span>, <span class="number">1809251394333065553</span>),
    (<span class="number">14658471137111155161</span>, <span class="number">1447401115466452442</span>),
    (<span class="number">4348079280205103483</span>, <span class="number">1157920892373161954</span>),
    (<span class="number">14335624477811986218</span>, <span class="number">1852673427797059126</span>),
    (<span class="number">7779150767507678651</span>, <span class="number">1482138742237647301</span>),
    (<span class="number">2533971799264232598</span>, <span class="number">1185710993790117841</span>),
    (<span class="number">15122401323048503126</span>, <span class="number">1897137590064188545</span>),
    (<span class="number">12097921058438802501</span>, <span class="number">1517710072051350836</span>),
    (<span class="number">5988988032009131678</span>, <span class="number">1214168057641080669</span>),
    (<span class="number">16961078480698431330</span>, <span class="number">1942668892225729070</span>),
    (<span class="number">13568862784558745064</span>, <span class="number">1554135113780583256</span>),
    (<span class="number">7165741412905085728</span>, <span class="number">1243308091024466605</span>),
    (<span class="number">11465186260648137165</span>, <span class="number">1989292945639146568</span>),
    (<span class="number">16550846638002330379</span>, <span class="number">1591434356511317254</span>),
    (<span class="number">16930026125143774626</span>, <span class="number">1273147485209053803</span>),
    (<span class="number">4951948911778577463</span>, <span class="number">2037035976334486086</span>),
    (<span class="number">272210314680951647</span>, <span class="number">1629628781067588869</span>),
    (<span class="number">3907117066486671641</span>, <span class="number">1303703024854071095</span>),
    (<span class="number">6251387306378674625</span>, <span class="number">2085924839766513752</span>),
    (<span class="number">16069156289328670670</span>, <span class="number">1668739871813211001</span>),
    (<span class="number">9165976216721026213</span>, <span class="number">1334991897450568801</span>),
    (<span class="number">7286864317269821294</span>, <span class="number">2135987035920910082</span>),
    (<span class="number">16897537898041588005</span>, <span class="number">1708789628736728065</span>),
    (<span class="number">13518030318433270404</span>, <span class="number">1367031702989382452</span>),
    (<span class="number">6871453250525591353</span>, <span class="number">2187250724783011924</span>),
    (<span class="number">9186511415162383406</span>, <span class="number">1749800579826409539</span>),
    (<span class="number">11038557946871817048</span>, <span class="number">1399840463861127631</span>),
    (<span class="number">10282995085511086630</span>, <span class="number">2239744742177804210</span>),
    (<span class="number">8226396068408869304</span>, <span class="number">1791795793742243368</span>),
    (<span class="number">13959814484210916090</span>, <span class="number">1433436634993794694</span>),
    (<span class="number">11267656730511734774</span>, <span class="number">2293498615990071511</span>),
    (<span class="number">5324776569667477496</span>, <span class="number">1834798892792057209</span>),
    (<span class="number">7949170070475892320</span>, <span class="number">1467839114233645767</span>),
    (<span class="number">17427382500606444826</span>, <span class="number">1174271291386916613</span>),
    (<span class="number">5747719112518849781</span>, <span class="number">1878834066219066582</span>),
    (<span class="number">15666221734240810795</span>, <span class="number">1503067252975253265</span>),
    (<span class="number">12532977387392648636</span>, <span class="number">1202453802380202612</span>),
    (<span class="number">5295368560860596524</span>, <span class="number">1923926083808324180</span>),
    (<span class="number">4236294848688477220</span>, <span class="number">1539140867046659344</span>),
    (<span class="number">7078384693692692099</span>, <span class="number">1231312693637327475</span>),
    (<span class="number">11325415509908307358</span>, <span class="number">1970100309819723960</span>),
    (<span class="number">9060332407926645887</span>, <span class="number">1576080247855779168</span>),
    (<span class="number">14626963555825137356</span>, <span class="number">1260864198284623334</span>),
    (<span class="number">12335095245094488799</span>, <span class="number">2017382717255397335</span>),
    (<span class="number">9868076196075591040</span>, <span class="number">1613906173804317868</span>),
    (<span class="number">15273158586344293478</span>, <span class="number">1291124939043454294</span>),
    (<span class="number">13369007293925138595</span>, <span class="number">2065799902469526871</span>),
    (<span class="number">7005857020398200553</span>, <span class="number">1652639921975621497</span>),
    (<span class="number">16672732060544291412</span>, <span class="number">1322111937580497197</span>),
    (<span class="number">11918976037903224966</span>, <span class="number">2115379100128795516</span>),
    (<span class="number">5845832015580669650</span>, <span class="number">1692303280103036413</span>),
    (<span class="number">12055363241948356366</span>, <span class="number">1353842624082429130</span>),
    (<span class="number">841837113407818570</span>, <span class="number">2166148198531886609</span>),
    (<span class="number">4362818505468165179</span>, <span class="number">1732918558825509287</span>),
    (<span class="number">14558301248600263113</span>, <span class="number">1386334847060407429</span>),
    (<span class="number">12225235553534690011</span>, <span class="number">2218135755296651887</span>),
    (<span class="number">2401490813343931363</span>, <span class="number">1774508604237321510</span>),
    (<span class="number">1921192650675145090</span>, <span class="number">1419606883389857208</span>),
    (<span class="number">17831303500047873437</span>, <span class="number">2271371013423771532</span>),
    (<span class="number">6886345170554478103</span>, <span class="number">1817096810739017226</span>),
    (<span class="number">1819727321701672159</span>, <span class="number">1453677448591213781</span>),
    (<span class="number">16213177116328979020</span>, <span class="number">1162941958872971024</span>),
    (<span class="number">14873036941900635463</span>, <span class="number">1860707134196753639</span>),
    (<span class="number">15587778368262418694</span>, <span class="number">1488565707357402911</span>),
    (<span class="number">8780873879868024632</span>, <span class="number">1190852565885922329</span>),
    (<span class="number">2981351763563108441</span>, <span class="number">1905364105417475727</span>),
    (<span class="number">13453127855076217722</span>, <span class="number">1524291284333980581</span>),
    (<span class="number">7073153469319063855</span>, <span class="number">1219433027467184465</span>),
    (<span class="number">11317045550910502167</span>, <span class="number">1951092843947495144</span>),
    (<span class="number">12742985255470312057</span>, <span class="number">1560874275157996115</span>),
    (<span class="number">10194388204376249646</span>, <span class="number">1248699420126396892</span>),
    (<span class="number">1553625868034358140</span>, <span class="number">1997919072202235028</span>),
    (<span class="number">8621598323911307159</span>, <span class="number">1598335257761788022</span>),
    (<span class="number">17965325103354776697</span>, <span class="number">1278668206209430417</span>),
    (<span class="number">13987124906400001422</span>, <span class="number">2045869129935088668</span>),
    (<span class="number">121653480894270168</span>, <span class="number">1636695303948070935</span>),
    (<span class="number">97322784715416134</span>, <span class="number">1309356243158456748</span>),
    (<span class="number">14913111714512307107</span>, <span class="number">2094969989053530796</span>),
    (<span class="number">8241140556867935363</span>, <span class="number">1675975991242824637</span>),
    (<span class="number">17660958889720079260</span>, <span class="number">1340780792994259709</span>),
    (<span class="number">17189487779326395846</span>, <span class="number">2145249268790815535</span>),
    (<span class="number">13751590223461116677</span>, <span class="number">1716199415032652428</span>),
    (<span class="number">18379969808252713988</span>, <span class="number">1372959532026121942</span>),
    (<span class="number">14650556434236701088</span>, <span class="number">2196735251241795108</span>),
    (<span class="number">652398703163629901</span>, <span class="number">1757388200993436087</span>),
    (<span class="number">11589965406756634890</span>, <span class="number">1405910560794748869</span>),
    (<span class="number">7475898206584884855</span>, <span class="number">2249456897271598191</span>),
    (<span class="number">2291369750525997561</span>, <span class="number">1799565517817278553</span>),
    (<span class="number">9211793429904618695</span>, <span class="number">1439652414253822842</span>),
    (<span class="number">18428218302589300235</span>, <span class="number">2303443862806116547</span>),
    (<span class="number">7363877012587619542</span>, <span class="number">1842755090244893238</span>),
    (<span class="number">13269799239553916280</span>, <span class="number">1474204072195914590</span>),
    (<span class="number">10615839391643133024</span>, <span class="number">1179363257756731672</span>),
    (<span class="number">2227947767661371545</span>, <span class="number">1886981212410770676</span>),
    (<span class="number">16539753473096738529</span>, <span class="number">1509584969928616540</span>),
    (<span class="number">13231802778477390823</span>, <span class="number">1207667975942893232</span>),
    (<span class="number">6413489186596184024</span>, <span class="number">1932268761508629172</span>),
    (<span class="number">16198837793502678189</span>, <span class="number">1545815009206903337</span>),
    (<span class="number">5580372605318321905</span>, <span class="number">1236652007365522670</span>),
    (<span class="number">8928596168509315048</span>, <span class="number">1978643211784836272</span>),
    (<span class="number">18210923379033183008</span>, <span class="number">1582914569427869017</span>),
    (<span class="number">7190041073742725760</span>, <span class="number">1266331655542295214</span>),
    (<span class="number">436019273762630246</span>, <span class="number">2026130648867672343</span>),
    (<span class="number">7727513048493924843</span>, <span class="number">1620904519094137874</span>),
    (<span class="number">9871359253537050198</span>, <span class="number">1296723615275310299</span>),
    (<span class="number">4726128361433549347</span>, <span class="number">2074757784440496479</span>),
    (<span class="number">7470251503888749801</span>, <span class="number">1659806227552397183</span>),
    (<span class="number">13354898832594820487</span>, <span class="number">1327844982041917746</span>),
    (<span class="number">13989140502667892133</span>, <span class="number">2124551971267068394</span>),
    (<span class="number">14880661216876224029</span>, <span class="number">1699641577013654715</span>),
    (<span class="number">11904528973500979224</span>, <span class="number">1359713261610923772</span>),
    (<span class="number">4289851098633925465</span>, <span class="number">2175541218577478036</span>),
    (<span class="number">18189276137874781665</span>, <span class="number">1740432974861982428</span>),
    (<span class="number">3483374466074094362</span>, <span class="number">1392346379889585943</span>),
    (<span class="number">1884050330976640656</span>, <span class="number">2227754207823337509</span>),
    (<span class="number">5196589079523222848</span>, <span class="number">1782203366258670007</span>),
    (<span class="number">15225317707844309248</span>, <span class="number">1425762693006936005</span>),
    (<span class="number">5913764258841343181</span>, <span class="number">2281220308811097609</span>),
    (<span class="number">8420360221814984868</span>, <span class="number">1824976247048878087</span>),
    (<span class="number">17804334621677718864</span>, <span class="number">1459980997639102469</span>),
    (<span class="number">17932816512084085415</span>, <span class="number">1167984798111281975</span>),
    (<span class="number">10245762345624985047</span>, <span class="number">1868775676978051161</span>),
    (<span class="number">4507261061758077715</span>, <span class="number">1495020541582440929</span>),
    (<span class="number">7295157664148372495</span>, <span class="number">1196016433265952743</span>),
    (<span class="number">7982903447895485668</span>, <span class="number">1913626293225524389</span>),
    (<span class="number">10075671573058298858</span>, <span class="number">1530901034580419511</span>),
    (<span class="number">4371188443704728763</span>, <span class="number">1224720827664335609</span>),
    (<span class="number">14372599139411386667</span>, <span class="number">1959553324262936974</span>),
    (<span class="number">15187428126271019657</span>, <span class="number">1567642659410349579</span>),
    (<span class="number">15839291315758726049</span>, <span class="number">1254114127528279663</span>),
    (<span class="number">3206773216762499739</span>, <span class="number">2006582604045247462</span>),
    (<span class="number">13633465017635730761</span>, <span class="number">1605266083236197969</span>),
    (<span class="number">14596120828850494932</span>, <span class="number">1284212866588958375</span>),
    (<span class="number">4907049252451240275</span>, <span class="number">2054740586542333401</span>),
    (<span class="number">236290587219081897</span>, <span class="number">1643792469233866721</span>),
    (<span class="number">14946427728742906810</span>, <span class="number">1315033975387093376</span>),
    (<span class="number">16535586736504830250</span>, <span class="number">2104054360619349402</span>),
    (<span class="number">5849771759720043554</span>, <span class="number">1683243488495479522</span>),
    (<span class="number">15747863852001765813</span>, <span class="number">1346594790796383617</span>),
    (<span class="number">10439186904235184007</span>, <span class="number">2154551665274213788</span>),
    (<span class="number">15730047152871967852</span>, <span class="number">1723641332219371030</span>),
    (<span class="number">12584037722297574282</span>, <span class="number">1378913065775496824</span>),
    (<span class="number">9066413911450387881</span>, <span class="number">2206260905240794919</span>),
    (<span class="number">10942479943902220628</span>, <span class="number">1765008724192635935</span>),
    (<span class="number">8753983955121776503</span>, <span class="number">1412006979354108748</span>),
    (<span class="number">10317025513452932081</span>, <span class="number">2259211166966573997</span>),
    (<span class="number">874922781278525018</span>, <span class="number">1807368933573259198</span>),
    (<span class="number">8078635854506640661</span>, <span class="number">1445895146858607358</span>),
    (<span class="number">13841606313089133175</span>, <span class="number">1156716117486885886</span>),
    (<span class="number">14767872471458792434</span>, <span class="number">1850745787979017418</span>),
    (<span class="number">746251532941302978</span>, <span class="number">1480596630383213935</span>),
    (<span class="number">597001226353042382</span>, <span class="number">1184477304306571148</span>),
    (<span class="number">15712597221132509104</span>, <span class="number">1895163686890513836</span>),
    (<span class="number">8880728962164096960</span>, <span class="number">1516130949512411069</span>),
    (<span class="number">10793931984473187891</span>, <span class="number">1212904759609928855</span>),
    (<span class="number">17270291175157100626</span>, <span class="number">1940647615375886168</span>),
    (<span class="number">2748186495899949531</span>, <span class="number">1552518092300708935</span>),
    (<span class="number">2198549196719959625</span>, <span class="number">1242014473840567148</span>),
    (<span class="number">18275073973719576693</span>, <span class="number">1987223158144907436</span>),
    (<span class="number">10930710364233751031</span>, <span class="number">1589778526515925949</span>),
    (<span class="number">12433917106128911148</span>, <span class="number">1271822821212740759</span>),
    (<span class="number">8826220925580526867</span>, <span class="number">2034916513940385215</span>),
    (<span class="number">7060976740464421494</span>, <span class="number">1627933211152308172</span>),
    (<span class="number">16716827836597268165</span>, <span class="number">1302346568921846537</span>),
    (<span class="number">11989529279587987770</span>, <span class="number">2083754510274954460</span>),
    (<span class="number">9591623423670390216</span>, <span class="number">1667003608219963568</span>),
    (<span class="number">15051996368420132820</span>, <span class="number">1333602886575970854</span>),
    (<span class="number">13015147745246481542</span>, <span class="number">2133764618521553367</span>),
    (<span class="number">3033420566713364587</span>, <span class="number">1707011694817242694</span>),
    (<span class="number">6116085268112601993</span>, <span class="number">1365609355853794155</span>),
    (<span class="number">9785736428980163188</span>, <span class="number">2184974969366070648</span>),
    (<span class="number">15207286772667951197</span>, <span class="number">1747979975492856518</span>),
    (<span class="number">1097782973908629988</span>, <span class="number">1398383980394285215</span>),
    (<span class="number">1756452758253807981</span>, <span class="number">2237414368630856344</span>),
    (<span class="number">5094511021344956708</span>, <span class="number">1789931494904685075</span>),
    (<span class="number">4075608817075965366</span>, <span class="number">1431945195923748060</span>),
    (<span class="number">6520974107321544586</span>, <span class="number">2291112313477996896</span>),
    (<span class="number">1527430471115325346</span>, <span class="number">1832889850782397517</span>),
    (<span class="number">12289990821117991246</span>, <span class="number">1466311880625918013</span>),
    (<span class="number">17210690286378213644</span>, <span class="number">1173049504500734410</span>),
    (<span class="number">9090360384495590213</span>, <span class="number">1876879207201175057</span>),
    (<span class="number">18340334751822203140</span>, <span class="number">1501503365760940045</span>),
    (<span class="number">14672267801457762512</span>, <span class="number">1201202692608752036</span>),
    (<span class="number">16096930852848599373</span>, <span class="number">1921924308174003258</span>),
    (<span class="number">1809498238053148529</span>, <span class="number">1537539446539202607</span>),
    (<span class="number">12515645034668249793</span>, <span class="number">1230031557231362085</span>),
    (<span class="number">1578287981759648052</span>, <span class="number">1968050491570179337</span>),
    (<span class="number">12330676829633449412</span>, <span class="number">1574440393256143469</span>),
    (<span class="number">13553890278448669853</span>, <span class="number">1259552314604914775</span>),
    (<span class="number">3239480371808320148</span>, <span class="number">2015283703367863641</span>),
    (<span class="number">17348979556414297411</span>, <span class="number">1612226962694290912</span>),
    (<span class="number">6500486015647617283</span>, <span class="number">1289781570155432730</span>),
    (<span class="number">10400777625036187652</span>, <span class="number">2063650512248692368</span>),
    (<span class="number">15699319729512770768</span>, <span class="number">1650920409798953894</span>),
    (<span class="number">16248804598352126938</span>, <span class="number">1320736327839163115</span>),
    (<span class="number">7551343283653851484</span>, <span class="number">2113178124542660985</span>),
    (<span class="number">6041074626923081187</span>, <span class="number">1690542499634128788</span>),
    (<span class="number">12211557331022285596</span>, <span class="number">1352433999707303030</span>),
    (<span class="number">1091747655926105338</span>, <span class="number">2163894399531684849</span>),
    (<span class="number">4562746939482794594</span>, <span class="number">1731115519625347879</span>),
    (<span class="number">7339546366328145998</span>, <span class="number">1384892415700278303</span>),
    (<span class="number">8053925371383123274</span>, <span class="number">2215827865120445285</span>),
    (<span class="number">6443140297106498619</span>, <span class="number">1772662292096356228</span>),
    (<span class="number">12533209867169019542</span>, <span class="number">1418129833677084982</span>),
    (<span class="number">5295740528502789974</span>, <span class="number">2269007733883335972</span>),
    (<span class="number">15304638867027962949</span>, <span class="number">1815206187106668777</span>),
    (<span class="number">4865013464138549713</span>, <span class="number">1452164949685335022</span>),
    (<span class="number">14960057215536570740</span>, <span class="number">1161731959748268017</span>),
    (<span class="number">9178696285890871890</span>, <span class="number">1858771135597228828</span>),
    (<span class="number">14721654658196518159</span>, <span class="number">1487016908477783062</span>),
    (<span class="number">4398626097073393881</span>, <span class="number">1189613526782226450</span>),
    (<span class="number">7037801755317430209</span>, <span class="number">1903381642851562320</span>),
    (<span class="number">5630241404253944167</span>, <span class="number">1522705314281249856</span>),
    (<span class="number">814844308661245011</span>, <span class="number">1218164251424999885</span>),
    (<span class="number">1303750893857992017</span>, <span class="number">1949062802279999816</span>),
    (<span class="number">15800395974054034906</span>, <span class="number">1559250241823999852</span>),
    (<span class="number">5261619149759407279</span>, <span class="number">1247400193459199882</span>),
    (<span class="number">12107939454356961969</span>, <span class="number">1995840309534719811</span>),
    (<span class="number">5997002748743659252</span>, <span class="number">1596672247627775849</span>),
    (<span class="number">8486951013736837725</span>, <span class="number">1277337798102220679</span>),
    (<span class="number">2511075177753209390</span>, <span class="number">2043740476963553087</span>),
    (<span class="number">13076906586428298482</span>, <span class="number">1634992381570842469</span>),
    (<span class="number">14150874083884549109</span>, <span class="number">1307993905256673975</span>),
    (<span class="number">4194654460505726958</span>, <span class="number">2092790248410678361</span>),
    (<span class="number">18113118827372222859</span>, <span class="number">1674232198728542688</span>),
    (<span class="number">3422448617672047318</span>, <span class="number">1339385758982834151</span>),
    (<span class="number">16543964232501006678</span>, <span class="number">2143017214372534641</span>),
    (<span class="number">9545822571258895019</span>, <span class="number">1714413771498027713</span>),
    (<span class="number">15015355686490936662</span>, <span class="number">1371531017198422170</span>),
    (<span class="number">5577825024675947042</span>, <span class="number">2194449627517475473</span>),
    (<span class="number">11840957649224578280</span>, <span class="number">1755559702013980378</span>),
    (<span class="number">16851463748863483271</span>, <span class="number">1404447761611184302</span>),
    (<span class="number">12204946739213931940</span>, <span class="number">2247116418577894884</span>),
    (<span class="number">13453306206113055875</span>, <span class="number">1797693134862315907</span>),
    (<span class="number">3383947335406624054</span>, <span class="number">1438154507889852726</span>),
    (<span class="number">16482362180876329456</span>, <span class="number">2301047212623764361</span>),
    (<span class="number">9496540929959153242</span>, <span class="number">1840837770099011489</span>),
    (<span class="number">11286581558709232917</span>, <span class="number">1472670216079209191</span>),
    (<span class="number">5339916432225476010</span>, <span class="number">1178136172863367353</span>),
    (<span class="number">4854517476818851293</span>, <span class="number">1885017876581387765</span>),
    (<span class="number">3883613981455081034</span>, <span class="number">1508014301265110212</span>),
    (<span class="number">14174937629389795797</span>, <span class="number">1206411441012088169</span>),
    (<span class="number">11611853762797942306</span>, <span class="number">1930258305619341071</span>),
    (<span class="number">5600134195496443521</span>, <span class="number">1544206644495472857</span>),
    (<span class="number">15548153800622885787</span>, <span class="number">1235365315596378285</span>),
    (<span class="number">6430302007287065643</span>, <span class="number">1976584504954205257</span>),
    (<span class="number">16212288050055383484</span>, <span class="number">1581267603963364205</span>),
    (<span class="number">12969830440044306787</span>, <span class="number">1265014083170691364</span>),
    (<span class="number">9683682259845159889</span>, <span class="number">2024022533073106183</span>),
    (<span class="number">15125643437359948558</span>, <span class="number">1619218026458484946</span>),
    (<span class="number">8411165935146048523</span>, <span class="number">1295374421166787957</span>),
    (<span class="number">17147214310975587960</span>, <span class="number">2072599073866860731</span>),
    (<span class="number">10028422634038560045</span>, <span class="number">1658079259093488585</span>),
    (<span class="number">8022738107230848036</span>, <span class="number">1326463407274790868</span>),
    (<span class="number">9147032156827446534</span>, <span class="number">2122341451639665389</span>),
    (<span class="number">11006974540203867551</span>, <span class="number">1697873161311732311</span>),
    (<span class="number">5116230817421183718</span>, <span class="number">1358298529049385849</span>),
    (<span class="number">15564666937357714594</span>, <span class="number">2173277646479017358</span>),
    (<span class="number">1383687105660440706</span>, <span class="number">1738622117183213887</span>),
    (<span class="number">12174996128754083534</span>, <span class="number">1390897693746571109</span>),
    (<span class="number">8411947361780802685</span>, <span class="number">2225436309994513775</span>),
    (<span class="number">6729557889424642148</span>, <span class="number">1780349047995611020</span>),
    (<span class="number">5383646311539713719</span>, <span class="number">1424279238396488816</span>),
    (<span class="number">1235136468979721303</span>, <span class="number">2278846781434382106</span>),
    (<span class="number">15745504434151418335</span>, <span class="number">1823077425147505684</span>),
    (<span class="number">16285752362063044992</span>, <span class="number">1458461940118004547</span>),
    (<span class="number">5649904260166615347</span>, <span class="number">1166769552094403638</span>),
    (<span class="number">5350498001524674232</span>, <span class="number">1866831283351045821</span>),
    (<span class="number">591049586477829062</span>, <span class="number">1493465026680836657</span>),
    (<span class="number">11540886113407994219</span>, <span class="number">1194772021344669325</span>),
    (<span class="number">18673707743239135</span>, <span class="number">1911635234151470921</span>),
    (<span class="number">14772334225162232601</span>, <span class="number">1529308187321176736</span>),
    (<span class="number">8128518565387875758</span>, <span class="number">1223446549856941389</span>),
    (<span class="number">1937583260394870242</span>, <span class="number">1957514479771106223</span>),
    (<span class="number">8928764237799716840</span>, <span class="number">1566011583816884978</span>),
    (<span class="number">14521709019723594119</span>, <span class="number">1252809267053507982</span>),
    (<span class="number">8477339172590109297</span>, <span class="number">2004494827285612772</span>),
    (<span class="number">17849917782297818407</span>, <span class="number">1603595861828490217</span>),
    (<span class="number">6901236596354434079</span>, <span class="number">1282876689462792174</span>),
    (<span class="number">18420676183650915173</span>, <span class="number">2052602703140467478</span>),
    (<span class="number">3668494502695001169</span>, <span class="number">1642082162512373983</span>),
    (<span class="number">10313493231639821582</span>, <span class="number">1313665730009899186</span>),
    (<span class="number">9122891541139893884</span>, <span class="number">2101865168015838698</span>),
    (<span class="number">14677010862395735754</span>, <span class="number">1681492134412670958</span>),
    (<span class="number">673562245690857633</span>, <span class="number">1345193707530136767</span>),
];

<span class="kw">pub static </span>DOUBLE_POW5_SPLIT: [(u64, u64); DOUBLE_POW5_TABLE_SIZE] = [
    (<span class="number">0</span>, <span class="number">1152921504606846976</span>),
    (<span class="number">0</span>, <span class="number">1441151880758558720</span>),
    (<span class="number">0</span>, <span class="number">1801439850948198400</span>),
    (<span class="number">0</span>, <span class="number">2251799813685248000</span>),
    (<span class="number">0</span>, <span class="number">1407374883553280000</span>),
    (<span class="number">0</span>, <span class="number">1759218604441600000</span>),
    (<span class="number">0</span>, <span class="number">2199023255552000000</span>),
    (<span class="number">0</span>, <span class="number">1374389534720000000</span>),
    (<span class="number">0</span>, <span class="number">1717986918400000000</span>),
    (<span class="number">0</span>, <span class="number">2147483648000000000</span>),
    (<span class="number">0</span>, <span class="number">1342177280000000000</span>),
    (<span class="number">0</span>, <span class="number">1677721600000000000</span>),
    (<span class="number">0</span>, <span class="number">2097152000000000000</span>),
    (<span class="number">0</span>, <span class="number">1310720000000000000</span>),
    (<span class="number">0</span>, <span class="number">1638400000000000000</span>),
    (<span class="number">0</span>, <span class="number">2048000000000000000</span>),
    (<span class="number">0</span>, <span class="number">1280000000000000000</span>),
    (<span class="number">0</span>, <span class="number">1600000000000000000</span>),
    (<span class="number">0</span>, <span class="number">2000000000000000000</span>),
    (<span class="number">0</span>, <span class="number">1250000000000000000</span>),
    (<span class="number">0</span>, <span class="number">1562500000000000000</span>),
    (<span class="number">0</span>, <span class="number">1953125000000000000</span>),
    (<span class="number">0</span>, <span class="number">1220703125000000000</span>),
    (<span class="number">0</span>, <span class="number">1525878906250000000</span>),
    (<span class="number">0</span>, <span class="number">1907348632812500000</span>),
    (<span class="number">0</span>, <span class="number">1192092895507812500</span>),
    (<span class="number">0</span>, <span class="number">1490116119384765625</span>),
    (<span class="number">4611686018427387904</span>, <span class="number">1862645149230957031</span>),
    (<span class="number">9799832789158199296</span>, <span class="number">1164153218269348144</span>),
    (<span class="number">12249790986447749120</span>, <span class="number">1455191522836685180</span>),
    (<span class="number">15312238733059686400</span>, <span class="number">1818989403545856475</span>),
    (<span class="number">14528612397897220096</span>, <span class="number">2273736754432320594</span>),
    (<span class="number">13692068767113150464</span>, <span class="number">1421085471520200371</span>),
    (<span class="number">12503399940464050176</span>, <span class="number">1776356839400250464</span>),
    (<span class="number">15629249925580062720</span>, <span class="number">2220446049250313080</span>),
    (<span class="number">9768281203487539200</span>, <span class="number">1387778780781445675</span>),
    (<span class="number">7598665485932036096</span>, <span class="number">1734723475976807094</span>),
    (<span class="number">274959820560269312</span>, <span class="number">2168404344971008868</span>),
    (<span class="number">9395221924704944128</span>, <span class="number">1355252715606880542</span>),
    (<span class="number">2520655369026404352</span>, <span class="number">1694065894508600678</span>),
    (<span class="number">12374191248137781248</span>, <span class="number">2117582368135750847</span>),
    (<span class="number">14651398557727195136</span>, <span class="number">1323488980084844279</span>),
    (<span class="number">13702562178731606016</span>, <span class="number">1654361225106055349</span>),
    (<span class="number">3293144668132343808</span>, <span class="number">2067951531382569187</span>),
    (<span class="number">18199116482078572544</span>, <span class="number">1292469707114105741</span>),
    (<span class="number">8913837547316051968</span>, <span class="number">1615587133892632177</span>),
    (<span class="number">15753982952572452864</span>, <span class="number">2019483917365790221</span>),
    (<span class="number">12152082354571476992</span>, <span class="number">1262177448353618888</span>),
    (<span class="number">15190102943214346240</span>, <span class="number">1577721810442023610</span>),
    (<span class="number">9764256642163156992</span>, <span class="number">1972152263052529513</span>),
    (<span class="number">17631875447420442880</span>, <span class="number">1232595164407830945</span>),
    (<span class="number">8204786253993389888</span>, <span class="number">1540743955509788682</span>),
    (<span class="number">1032610780636961552</span>, <span class="number">1925929944387235853</span>),
    (<span class="number">2951224747111794922</span>, <span class="number">1203706215242022408</span>),
    (<span class="number">3689030933889743652</span>, <span class="number">1504632769052528010</span>),
    (<span class="number">13834660704216955373</span>, <span class="number">1880790961315660012</span>),
    (<span class="number">17870034976990372916</span>, <span class="number">1175494350822287507</span>),
    (<span class="number">17725857702810578241</span>, <span class="number">1469367938527859384</span>),
    (<span class="number">3710578054803671186</span>, <span class="number">1836709923159824231</span>),
    (<span class="number">26536550077201078</span>, <span class="number">2295887403949780289</span>),
    (<span class="number">11545800389866720434</span>, <span class="number">1434929627468612680</span>),
    (<span class="number">14432250487333400542</span>, <span class="number">1793662034335765850</span>),
    (<span class="number">8816941072311974870</span>, <span class="number">2242077542919707313</span>),
    (<span class="number">17039803216263454053</span>, <span class="number">1401298464324817070</span>),
    (<span class="number">12076381983474541759</span>, <span class="number">1751623080406021338</span>),
    (<span class="number">5872105442488401391</span>, <span class="number">2189528850507526673</span>),
    (<span class="number">15199280947623720629</span>, <span class="number">1368455531567204170</span>),
    (<span class="number">9775729147674874978</span>, <span class="number">1710569414459005213</span>),
    (<span class="number">16831347453020981627</span>, <span class="number">2138211768073756516</span>),
    (<span class="number">1296220121283337709</span>, <span class="number">1336382355046097823</span>),
    (<span class="number">15455333206886335848</span>, <span class="number">1670477943807622278</span>),
    (<span class="number">10095794471753144002</span>, <span class="number">2088097429759527848</span>),
    (<span class="number">6309871544845715001</span>, <span class="number">1305060893599704905</span>),
    (<span class="number">12499025449484531656</span>, <span class="number">1631326116999631131</span>),
    (<span class="number">11012095793428276666</span>, <span class="number">2039157646249538914</span>),
    (<span class="number">11494245889320060820</span>, <span class="number">1274473528905961821</span>),
    (<span class="number">532749306367912313</span>, <span class="number">1593091911132452277</span>),
    (<span class="number">5277622651387278295</span>, <span class="number">1991364888915565346</span>),
    (<span class="number">7910200175544436838</span>, <span class="number">1244603055572228341</span>),
    (<span class="number">14499436237857933952</span>, <span class="number">1555753819465285426</span>),
    (<span class="number">8900923260467641632</span>, <span class="number">1944692274331606783</span>),
    (<span class="number">12480606065433357876</span>, <span class="number">1215432671457254239</span>),
    (<span class="number">10989071563364309441</span>, <span class="number">1519290839321567799</span>),
    (<span class="number">9124653435777998898</span>, <span class="number">1899113549151959749</span>),
    (<span class="number">8008751406574943263</span>, <span class="number">1186945968219974843</span>),
    (<span class="number">5399253239791291175</span>, <span class="number">1483682460274968554</span>),
    (<span class="number">15972438586593889776</span>, <span class="number">1854603075343710692</span>),
    (<span class="number">759402079766405302</span>, <span class="number">1159126922089819183</span>),
    (<span class="number">14784310654990170340</span>, <span class="number">1448908652612273978</span>),
    (<span class="number">9257016281882937117</span>, <span class="number">1811135815765342473</span>),
    (<span class="number">16182956370781059300</span>, <span class="number">2263919769706678091</span>),
    (<span class="number">7808504722524468110</span>, <span class="number">1414949856066673807</span>),
    (<span class="number">5148944884728197234</span>, <span class="number">1768687320083342259</span>),
    (<span class="number">1824495087482858639</span>, <span class="number">2210859150104177824</span>),
    (<span class="number">1140309429676786649</span>, <span class="number">1381786968815111140</span>),
    (<span class="number">1425386787095983311</span>, <span class="number">1727233711018888925</span>),
    (<span class="number">6393419502297367043</span>, <span class="number">2159042138773611156</span>),
    (<span class="number">13219259225790630210</span>, <span class="number">1349401336733506972</span>),
    (<span class="number">16524074032238287762</span>, <span class="number">1686751670916883715</span>),
    (<span class="number">16043406521870471799</span>, <span class="number">2108439588646104644</span>),
    (<span class="number">803757039314269066</span>, <span class="number">1317774742903815403</span>),
    (<span class="number">14839754354425000045</span>, <span class="number">1647218428629769253</span>),
    (<span class="number">4714634887749086344</span>, <span class="number">2059023035787211567</span>),
    (<span class="number">9864175832484260821</span>, <span class="number">1286889397367007229</span>),
    (<span class="number">16941905809032713930</span>, <span class="number">1608611746708759036</span>),
    (<span class="number">2730638187581340797</span>, <span class="number">2010764683385948796</span>),
    (<span class="number">10930020904093113806</span>, <span class="number">1256727927116217997</span>),
    (<span class="number">18274212148543780162</span>, <span class="number">1570909908895272496</span>),
    (<span class="number">4396021111970173586</span>, <span class="number">1963637386119090621</span>),
    (<span class="number">5053356204195052443</span>, <span class="number">1227273366324431638</span>),
    (<span class="number">15540067292098591362</span>, <span class="number">1534091707905539547</span>),
    (<span class="number">14813398096695851299</span>, <span class="number">1917614634881924434</span>),
    (<span class="number">13870059828862294966</span>, <span class="number">1198509146801202771</span>),
    (<span class="number">12725888767650480803</span>, <span class="number">1498136433501503464</span>),
    (<span class="number">15907360959563101004</span>, <span class="number">1872670541876879330</span>),
    (<span class="number">14553786618154326031</span>, <span class="number">1170419088673049581</span>),
    (<span class="number">4357175217410743827</span>, <span class="number">1463023860841311977</span>),
    (<span class="number">10058155040190817688</span>, <span class="number">1828779826051639971</span>),
    (<span class="number">7961007781811134206</span>, <span class="number">2285974782564549964</span>),
    (<span class="number">14199001900486734687</span>, <span class="number">1428734239102843727</span>),
    (<span class="number">13137066357181030455</span>, <span class="number">1785917798878554659</span>),
    (<span class="number">11809646928048900164</span>, <span class="number">2232397248598193324</span>),
    (<span class="number">16604401366885338411</span>, <span class="number">1395248280373870827</span>),
    (<span class="number">16143815690179285109</span>, <span class="number">1744060350467338534</span>),
    (<span class="number">10956397575869330579</span>, <span class="number">2180075438084173168</span>),
    (<span class="number">6847748484918331612</span>, <span class="number">1362547148802608230</span>),
    (<span class="number">17783057643002690323</span>, <span class="number">1703183936003260287</span>),
    (<span class="number">17617136035325974999</span>, <span class="number">2128979920004075359</span>),
    (<span class="number">17928239049719816230</span>, <span class="number">1330612450002547099</span>),
    (<span class="number">17798612793722382384</span>, <span class="number">1663265562503183874</span>),
    (<span class="number">13024893955298202172</span>, <span class="number">2079081953128979843</span>),
    (<span class="number">5834715712847682405</span>, <span class="number">1299426220705612402</span>),
    (<span class="number">16516766677914378815</span>, <span class="number">1624282775882015502</span>),
    (<span class="number">11422586310538197711</span>, <span class="number">2030353469852519378</span>),
    (<span class="number">11750802462513761473</span>, <span class="number">1268970918657824611</span>),
    (<span class="number">10076817059714813937</span>, <span class="number">1586213648322280764</span>),
    (<span class="number">12596021324643517422</span>, <span class="number">1982767060402850955</span>),
    (<span class="number">5566670318688504437</span>, <span class="number">1239229412751781847</span>),
    (<span class="number">2346651879933242642</span>, <span class="number">1549036765939727309</span>),
    (<span class="number">7545000868343941206</span>, <span class="number">1936295957424659136</span>),
    (<span class="number">4715625542714963254</span>, <span class="number">1210184973390411960</span>),
    (<span class="number">5894531928393704067</span>, <span class="number">1512731216738014950</span>),
    (<span class="number">16591536947346905892</span>, <span class="number">1890914020922518687</span>),
    (<span class="number">17287239619732898039</span>, <span class="number">1181821263076574179</span>),
    (<span class="number">16997363506238734644</span>, <span class="number">1477276578845717724</span>),
    (<span class="number">2799960309088866689</span>, <span class="number">1846595723557147156</span>),
    (<span class="number">10973347230035317489</span>, <span class="number">1154122327223216972</span>),
    (<span class="number">13716684037544146861</span>, <span class="number">1442652909029021215</span>),
    (<span class="number">12534169028502795672</span>, <span class="number">1803316136286276519</span>),
    (<span class="number">11056025267201106687</span>, <span class="number">2254145170357845649</span>),
    (<span class="number">18439230838069161439</span>, <span class="number">1408840731473653530</span>),
    (<span class="number">13825666510731675991</span>, <span class="number">1761050914342066913</span>),
    (<span class="number">3447025083132431277</span>, <span class="number">2201313642927583642</span>),
    (<span class="number">6766076695385157452</span>, <span class="number">1375821026829739776</span>),
    (<span class="number">8457595869231446815</span>, <span class="number">1719776283537174720</span>),
    (<span class="number">10571994836539308519</span>, <span class="number">2149720354421468400</span>),
    (<span class="number">6607496772837067824</span>, <span class="number">1343575221513417750</span>),
    (<span class="number">17482743002901110588</span>, <span class="number">1679469026891772187</span>),
    (<span class="number">17241742735199000331</span>, <span class="number">2099336283614715234</span>),
    (<span class="number">15387775227926763111</span>, <span class="number">1312085177259197021</span>),
    (<span class="number">5399660979626290177</span>, <span class="number">1640106471573996277</span>),
    (<span class="number">11361262242960250625</span>, <span class="number">2050133089467495346</span>),
    (<span class="number">11712474920277544544</span>, <span class="number">1281333180917184591</span>),
    (<span class="number">10028907631919542777</span>, <span class="number">1601666476146480739</span>),
    (<span class="number">7924448521472040567</span>, <span class="number">2002083095183100924</span>),
    (<span class="number">14176152362774801162</span>, <span class="number">1251301934489438077</span>),
    (<span class="number">3885132398186337741</span>, <span class="number">1564127418111797597</span>),
    (<span class="number">9468101516160310080</span>, <span class="number">1955159272639746996</span>),
    (<span class="number">15140935484454969608</span>, <span class="number">1221974545399841872</span>),
    (<span class="number">479425281859160394</span>, <span class="number">1527468181749802341</span>),
    (<span class="number">5210967620751338397</span>, <span class="number">1909335227187252926</span>),
    (<span class="number">17091912818251750210</span>, <span class="number">1193334516992033078</span>),
    (<span class="number">12141518985959911954</span>, <span class="number">1491668146240041348</span>),
    (<span class="number">15176898732449889943</span>, <span class="number">1864585182800051685</span>),
    (<span class="number">11791404716994875166</span>, <span class="number">1165365739250032303</span>),
    (<span class="number">10127569877816206054</span>, <span class="number">1456707174062540379</span>),
    (<span class="number">8047776328842869663</span>, <span class="number">1820883967578175474</span>),
    (<span class="number">836348374198811271</span>, <span class="number">2276104959472719343</span>),
    (<span class="number">7440246761515338900</span>, <span class="number">1422565599670449589</span>),
    (<span class="number">13911994470321561530</span>, <span class="number">1778206999588061986</span>),
    (<span class="number">8166621051047176104</span>, <span class="number">2222758749485077483</span>),
    (<span class="number">2798295147690791113</span>, <span class="number">1389224218428173427</span>),
    (<span class="number">17332926989895652603</span>, <span class="number">1736530273035216783</span>),
    (<span class="number">17054472718942177850</span>, <span class="number">2170662841294020979</span>),
    (<span class="number">8353202440125167204</span>, <span class="number">1356664275808763112</span>),
    (<span class="number">10441503050156459005</span>, <span class="number">1695830344760953890</span>),
    (<span class="number">3828506775840797949</span>, <span class="number">2119787930951192363</span>),
    (<span class="number">86973725686804766</span>, <span class="number">1324867456844495227</span>),
    (<span class="number">13943775212390669669</span>, <span class="number">1656084321055619033</span>),
    (<span class="number">3594660960206173375</span>, <span class="number">2070105401319523792</span>),
    (<span class="number">2246663100128858359</span>, <span class="number">1293815875824702370</span>),
    (<span class="number">12031700912015848757</span>, <span class="number">1617269844780877962</span>),
    (<span class="number">5816254103165035138</span>, <span class="number">2021587305976097453</span>),
    (<span class="number">5941001823691840913</span>, <span class="number">1263492066235060908</span>),
    (<span class="number">7426252279614801142</span>, <span class="number">1579365082793826135</span>),
    (<span class="number">4671129331091113523</span>, <span class="number">1974206353492282669</span>),
    (<span class="number">5225298841145639904</span>, <span class="number">1233878970932676668</span>),
    (<span class="number">6531623551432049880</span>, <span class="number">1542348713665845835</span>),
    (<span class="number">3552843420862674446</span>, <span class="number">1927935892082307294</span>),
    (<span class="number">16055585193321335241</span>, <span class="number">1204959932551442058</span>),
    (<span class="number">10846109454796893243</span>, <span class="number">1506199915689302573</span>),
    (<span class="number">18169322836923504458</span>, <span class="number">1882749894611628216</span>),
    (<span class="number">11355826773077190286</span>, <span class="number">1176718684132267635</span>),
    (<span class="number">9583097447919099954</span>, <span class="number">1470898355165334544</span>),
    (<span class="number">11978871809898874942</span>, <span class="number">1838622943956668180</span>),
    (<span class="number">14973589762373593678</span>, <span class="number">2298278679945835225</span>),
    (<span class="number">2440964573842414192</span>, <span class="number">1436424174966147016</span>),
    (<span class="number">3051205717303017741</span>, <span class="number">1795530218707683770</span>),
    (<span class="number">13037379183483547984</span>, <span class="number">2244412773384604712</span>),
    (<span class="number">8148361989677217490</span>, <span class="number">1402757983365377945</span>),
    (<span class="number">14797138505523909766</span>, <span class="number">1753447479206722431</span>),
    (<span class="number">13884737113477499304</span>, <span class="number">2191809349008403039</span>),
    (<span class="number">15595489723564518921</span>, <span class="number">1369880843130251899</span>),
    (<span class="number">14882676136028260747</span>, <span class="number">1712351053912814874</span>),
    (<span class="number">9379973133180550126</span>, <span class="number">2140438817391018593</span>),
    (<span class="number">17391698254306313589</span>, <span class="number">1337774260869386620</span>),
    (<span class="number">3292878744173340370</span>, <span class="number">1672217826086733276</span>),
    (<span class="number">4116098430216675462</span>, <span class="number">2090272282608416595</span>),
    (<span class="number">266718509671728212</span>, <span class="number">1306420176630260372</span>),
    (<span class="number">333398137089660265</span>, <span class="number">1633025220787825465</span>),
    (<span class="number">5028433689789463235</span>, <span class="number">2041281525984781831</span>),
    (<span class="number">10060300083759496378</span>, <span class="number">1275800953740488644</span>),
    (<span class="number">12575375104699370472</span>, <span class="number">1594751192175610805</span>),
    (<span class="number">1884160825592049379</span>, <span class="number">1993438990219513507</span>),
    (<span class="number">17318501580490888525</span>, <span class="number">1245899368887195941</span>),
    (<span class="number">7813068920331446945</span>, <span class="number">1557374211108994927</span>),
    (<span class="number">5154650131986920777</span>, <span class="number">1946717763886243659</span>),
    (<span class="number">915813323278131534</span>, <span class="number">1216698602428902287</span>),
    (<span class="number">14979824709379828129</span>, <span class="number">1520873253036127858</span>),
    (<span class="number">9501408849870009354</span>, <span class="number">1901091566295159823</span>),
    (<span class="number">12855909558809837702</span>, <span class="number">1188182228934474889</span>),
    (<span class="number">2234828893230133415</span>, <span class="number">1485227786168093612</span>),
    (<span class="number">2793536116537666769</span>, <span class="number">1856534732710117015</span>),
    (<span class="number">8663489100477123587</span>, <span class="number">1160334207943823134</span>),
    (<span class="number">1605989338741628675</span>, <span class="number">1450417759929778918</span>),
    (<span class="number">11230858710281811652</span>, <span class="number">1813022199912223647</span>),
    (<span class="number">9426887369424876662</span>, <span class="number">2266277749890279559</span>),
    (<span class="number">12809333633531629769</span>, <span class="number">1416423593681424724</span>),
    (<span class="number">16011667041914537212</span>, <span class="number">1770529492101780905</span>),
    (<span class="number">6179525747111007803</span>, <span class="number">2213161865127226132</span>),
    (<span class="number">13085575628799155685</span>, <span class="number">1383226165704516332</span>),
    (<span class="number">16356969535998944606</span>, <span class="number">1729032707130645415</span>),
    (<span class="number">15834525901571292854</span>, <span class="number">2161290883913306769</span>),
    (<span class="number">2979049660840976177</span>, <span class="number">1350806802445816731</span>),
    (<span class="number">17558870131333383934</span>, <span class="number">1688508503057270913</span>),
    (<span class="number">8113529608884566205</span>, <span class="number">2110635628821588642</span>),
    (<span class="number">9682642023980241782</span>, <span class="number">1319147268013492901</span>),
    (<span class="number">16714988548402690132</span>, <span class="number">1648934085016866126</span>),
    (<span class="number">11670363648648586857</span>, <span class="number">2061167606271082658</span>),
    (<span class="number">11905663298832754689</span>, <span class="number">1288229753919426661</span>),
    (<span class="number">1047021068258779650</span>, <span class="number">1610287192399283327</span>),
    (<span class="number">15143834390605638274</span>, <span class="number">2012858990499104158</span>),
    (<span class="number">4853210475701136017</span>, <span class="number">1258036869061940099</span>),
    (<span class="number">1454827076199032118</span>, <span class="number">1572546086327425124</span>),
    (<span class="number">1818533845248790147</span>, <span class="number">1965682607909281405</span>),
    (<span class="number">3442426662494187794</span>, <span class="number">1228551629943300878</span>),
    (<span class="number">13526405364972510550</span>, <span class="number">1535689537429126097</span>),
    (<span class="number">3072948650933474476</span>, <span class="number">1919611921786407622</span>),
    (<span class="number">15755650962115585259</span>, <span class="number">1199757451116504763</span>),
    (<span class="number">15082877684217093670</span>, <span class="number">1499696813895630954</span>),
    (<span class="number">9630225068416591280</span>, <span class="number">1874621017369538693</span>),
    (<span class="number">8324733676974063502</span>, <span class="number">1171638135855961683</span>),
    (<span class="number">5794231077790191473</span>, <span class="number">1464547669819952104</span>),
    (<span class="number">7242788847237739342</span>, <span class="number">1830684587274940130</span>),
    (<span class="number">18276858095901949986</span>, <span class="number">2288355734093675162</span>),
    (<span class="number">16034722328366106645</span>, <span class="number">1430222333808546976</span>),
    (<span class="number">1596658836748081690</span>, <span class="number">1787777917260683721</span>),
    (<span class="number">6607509564362490017</span>, <span class="number">2234722396575854651</span>),
    (<span class="number">1823850468512862308</span>, <span class="number">1396701497859909157</span>),
    (<span class="number">6891499104068465790</span>, <span class="number">1745876872324886446</span>),
    (<span class="number">17837745916940358045</span>, <span class="number">2182346090406108057</span>),
    (<span class="number">4231062170446641922</span>, <span class="number">1363966306503817536</span>),
    (<span class="number">5288827713058302403</span>, <span class="number">1704957883129771920</span>),
    (<span class="number">6611034641322878003</span>, <span class="number">2131197353912214900</span>),
    (<span class="number">13355268687681574560</span>, <span class="number">1331998346195134312</span>),
    (<span class="number">16694085859601968200</span>, <span class="number">1664997932743917890</span>),
    (<span class="number">11644235287647684442</span>, <span class="number">2081247415929897363</span>),
    (<span class="number">4971804045566108824</span>, <span class="number">1300779634956185852</span>),
    (<span class="number">6214755056957636030</span>, <span class="number">1625974543695232315</span>),
    (<span class="number">3156757802769657134</span>, <span class="number">2032468179619040394</span>),
    (<span class="number">6584659645158423613</span>, <span class="number">1270292612261900246</span>),
    (<span class="number">17454196593302805324</span>, <span class="number">1587865765327375307</span>),
    (<span class="number">17206059723201118751</span>, <span class="number">1984832206659219134</span>),
    (<span class="number">6142101308573311315</span>, <span class="number">1240520129162011959</span>),
    (<span class="number">3065940617289251240</span>, <span class="number">1550650161452514949</span>),
    (<span class="number">8444111790038951954</span>, <span class="number">1938312701815643686</span>),
    (<span class="number">665883850346957067</span>, <span class="number">1211445438634777304</span>),
    (<span class="number">832354812933696334</span>, <span class="number">1514306798293471630</span>),
    (<span class="number">10263815553021896226</span>, <span class="number">1892883497866839537</span>),
    (<span class="number">17944099766707154901</span>, <span class="number">1183052186166774710</span>),
    (<span class="number">13206752671529167818</span>, <span class="number">1478815232708468388</span>),
    (<span class="number">16508440839411459773</span>, <span class="number">1848519040885585485</span>),
    (<span class="number">12623618533845856310</span>, <span class="number">1155324400553490928</span>),
    (<span class="number">15779523167307320387</span>, <span class="number">1444155500691863660</span>),
    (<span class="number">1277659885424598868</span>, <span class="number">1805194375864829576</span>),
    (<span class="number">1597074856780748586</span>, <span class="number">2256492969831036970</span>),
    (<span class="number">5609857803915355770</span>, <span class="number">1410308106144398106</span>),
    (<span class="number">16235694291748970521</span>, <span class="number">1762885132680497632</span>),
    (<span class="number">1847873790976661535</span>, <span class="number">2203606415850622041</span>),
    (<span class="number">12684136165428883219</span>, <span class="number">1377254009906638775</span>),
    (<span class="number">11243484188358716120</span>, <span class="number">1721567512383298469</span>),
    (<span class="number">219297180166231438</span>, <span class="number">2151959390479123087</span>),
    (<span class="number">7054589765244976505</span>, <span class="number">1344974619049451929</span>),
    (<span class="number">13429923224983608535</span>, <span class="number">1681218273811814911</span>),
    (<span class="number">12175718012802122765</span>, <span class="number">2101522842264768639</span>),
    (<span class="number">14527352785642408584</span>, <span class="number">1313451776415480399</span>),
    (<span class="number">13547504963625622826</span>, <span class="number">1641814720519350499</span>),
    (<span class="number">12322695186104640628</span>, <span class="number">2052268400649188124</span>),
    (<span class="number">16925056528170176201</span>, <span class="number">1282667750405742577</span>),
    (<span class="number">7321262604930556539</span>, <span class="number">1603334688007178222</span>),
    (<span class="number">18374950293017971482</span>, <span class="number">2004168360008972777</span>),
    (<span class="number">4566814905495150320</span>, <span class="number">1252605225005607986</span>),
    (<span class="number">14931890668723713708</span>, <span class="number">1565756531257009982</span>),
    (<span class="number">9441491299049866327</span>, <span class="number">1957195664071262478</span>),
    (<span class="number">1289246043478778550</span>, <span class="number">1223247290044539049</span>),
    (<span class="number">6223243572775861092</span>, <span class="number">1529059112555673811</span>),
    (<span class="number">3167368447542438461</span>, <span class="number">1911323890694592264</span>),
    (<span class="number">1979605279714024038</span>, <span class="number">1194577431684120165</span>),
    (<span class="number">7086192618069917952</span>, <span class="number">1493221789605150206</span>),
    (<span class="number">18081112809442173248</span>, <span class="number">1866527237006437757</span>),
    (<span class="number">13606538515115052232</span>, <span class="number">1166579523129023598</span>),
    (<span class="number">7784801107039039482</span>, <span class="number">1458224403911279498</span>),
    (<span class="number">507629346944023544</span>, <span class="number">1822780504889099373</span>),
    (<span class="number">5246222702107417334</span>, <span class="number">2278475631111374216</span>),
    (<span class="number">3278889188817135834</span>, <span class="number">1424047269444608885</span>),
    (<span class="number">8710297504448807696</span>, <span class="number">1780059086805761106</span>),
];
</code></pre></div></section></main></body></html>