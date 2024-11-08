<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.3/src/raw/mod.rs`."><title>mod.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="hashbrown" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../hashbrown/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#2605" id="2605">2605</a>
<a href="#2606" id="2606">2606</a>
<a href="#2607" id="2607">2607</a>
<a href="#2608" id="2608">2608</a>
<a href="#2609" id="2609">2609</a>
<a href="#2610" id="2610">2610</a>
<a href="#2611" id="2611">2611</a>
<a href="#2612" id="2612">2612</a>
<a href="#2613" id="2613">2613</a>
<a href="#2614" id="2614">2614</a>
<a href="#2615" id="2615">2615</a>
<a href="#2616" id="2616">2616</a>
<a href="#2617" id="2617">2617</a>
<a href="#2618" id="2618">2618</a>
<a href="#2619" id="2619">2619</a>
<a href="#2620" id="2620">2620</a>
<a href="#2621" id="2621">2621</a>
<a href="#2622" id="2622">2622</a>
<a href="#2623" id="2623">2623</a>
<a href="#2624" id="2624">2624</a>
<a href="#2625" id="2625">2625</a>
<a href="#2626" id="2626">2626</a>
<a href="#2627" id="2627">2627</a>
<a href="#2628" id="2628">2628</a>
<a href="#2629" id="2629">2629</a>
<a href="#2630" id="2630">2630</a>
<a href="#2631" id="2631">2631</a>
<a href="#2632" id="2632">2632</a>
<a href="#2633" id="2633">2633</a>
<a href="#2634" id="2634">2634</a>
<a href="#2635" id="2635">2635</a>
<a href="#2636" id="2636">2636</a>
<a href="#2637" id="2637">2637</a>
<a href="#2638" id="2638">2638</a>
<a href="#2639" id="2639">2639</a>
<a href="#2640" id="2640">2640</a>
<a href="#2641" id="2641">2641</a>
<a href="#2642" id="2642">2642</a>
<a href="#2643" id="2643">2643</a>
<a href="#2644" id="2644">2644</a>
<a href="#2645" id="2645">2645</a>
<a href="#2646" id="2646">2646</a>
<a href="#2647" id="2647">2647</a>
<a href="#2648" id="2648">2648</a>
<a href="#2649" id="2649">2649</a>
<a href="#2650" id="2650">2650</a>
<a href="#2651" id="2651">2651</a>
<a href="#2652" id="2652">2652</a>
<a href="#2653" id="2653">2653</a>
<a href="#2654" id="2654">2654</a>
<a href="#2655" id="2655">2655</a>
<a href="#2656" id="2656">2656</a>
<a href="#2657" id="2657">2657</a>
<a href="#2658" id="2658">2658</a>
<a href="#2659" id="2659">2659</a>
<a href="#2660" id="2660">2660</a>
<a href="#2661" id="2661">2661</a>
<a href="#2662" id="2662">2662</a>
<a href="#2663" id="2663">2663</a>
<a href="#2664" id="2664">2664</a>
<a href="#2665" id="2665">2665</a>
<a href="#2666" id="2666">2666</a>
<a href="#2667" id="2667">2667</a>
<a href="#2668" id="2668">2668</a>
<a href="#2669" id="2669">2669</a>
<a href="#2670" id="2670">2670</a>
<a href="#2671" id="2671">2671</a>
<a href="#2672" id="2672">2672</a>
<a href="#2673" id="2673">2673</a>
<a href="#2674" id="2674">2674</a>
<a href="#2675" id="2675">2675</a>
<a href="#2676" id="2676">2676</a>
<a href="#2677" id="2677">2677</a>
<a href="#2678" id="2678">2678</a>
<a href="#2679" id="2679">2679</a>
<a href="#2680" id="2680">2680</a>
<a href="#2681" id="2681">2681</a>
<a href="#2682" id="2682">2682</a>
<a href="#2683" id="2683">2683</a>
<a href="#2684" id="2684">2684</a>
<a href="#2685" id="2685">2685</a>
<a href="#2686" id="2686">2686</a>
<a href="#2687" id="2687">2687</a>
<a href="#2688" id="2688">2688</a>
<a href="#2689" id="2689">2689</a>
<a href="#2690" id="2690">2690</a>
<a href="#2691" id="2691">2691</a>
<a href="#2692" id="2692">2692</a>
<a href="#2693" id="2693">2693</a>
<a href="#2694" id="2694">2694</a>
<a href="#2695" id="2695">2695</a>
<a href="#2696" id="2696">2696</a>
<a href="#2697" id="2697">2697</a>
<a href="#2698" id="2698">2698</a>
<a href="#2699" id="2699">2699</a>
<a href="#2700" id="2700">2700</a>
<a href="#2701" id="2701">2701</a>
<a href="#2702" id="2702">2702</a>
<a href="#2703" id="2703">2703</a>
<a href="#2704" id="2704">2704</a>
<a href="#2705" id="2705">2705</a>
<a href="#2706" id="2706">2706</a>
<a href="#2707" id="2707">2707</a>
<a href="#2708" id="2708">2708</a>
<a href="#2709" id="2709">2709</a>
<a href="#2710" id="2710">2710</a>
<a href="#2711" id="2711">2711</a>
<a href="#2712" id="2712">2712</a>
<a href="#2713" id="2713">2713</a>
<a href="#2714" id="2714">2714</a>
<a href="#2715" id="2715">2715</a>
<a href="#2716" id="2716">2716</a>
<a href="#2717" id="2717">2717</a>
<a href="#2718" id="2718">2718</a>
<a href="#2719" id="2719">2719</a>
<a href="#2720" id="2720">2720</a>
<a href="#2721" id="2721">2721</a>
<a href="#2722" id="2722">2722</a>
<a href="#2723" id="2723">2723</a>
<a href="#2724" id="2724">2724</a>
<a href="#2725" id="2725">2725</a>
<a href="#2726" id="2726">2726</a>
<a href="#2727" id="2727">2727</a>
<a href="#2728" id="2728">2728</a>
<a href="#2729" id="2729">2729</a>
<a href="#2730" id="2730">2730</a>
<a href="#2731" id="2731">2731</a>
<a href="#2732" id="2732">2732</a>
<a href="#2733" id="2733">2733</a>
<a href="#2734" id="2734">2734</a>
<a href="#2735" id="2735">2735</a>
<a href="#2736" id="2736">2736</a>
<a href="#2737" id="2737">2737</a>
<a href="#2738" id="2738">2738</a>
<a href="#2739" id="2739">2739</a>
<a href="#2740" id="2740">2740</a>
<a href="#2741" id="2741">2741</a>
<a href="#2742" id="2742">2742</a>
<a href="#2743" id="2743">2743</a>
<a href="#2744" id="2744">2744</a>
<a href="#2745" id="2745">2745</a>
<a href="#2746" id="2746">2746</a>
<a href="#2747" id="2747">2747</a>
<a href="#2748" id="2748">2748</a>
<a href="#2749" id="2749">2749</a>
<a href="#2750" id="2750">2750</a>
<a href="#2751" id="2751">2751</a>
<a href="#2752" id="2752">2752</a>
<a href="#2753" id="2753">2753</a>
<a href="#2754" id="2754">2754</a>
<a href="#2755" id="2755">2755</a>
<a href="#2756" id="2756">2756</a>
<a href="#2757" id="2757">2757</a>
<a href="#2758" id="2758">2758</a>
<a href="#2759" id="2759">2759</a>
<a href="#2760" id="2760">2760</a>
<a href="#2761" id="2761">2761</a>
<a href="#2762" id="2762">2762</a>
<a href="#2763" id="2763">2763</a>
<a href="#2764" id="2764">2764</a>
<a href="#2765" id="2765">2765</a>
<a href="#2766" id="2766">2766</a>
<a href="#2767" id="2767">2767</a>
<a href="#2768" id="2768">2768</a>
<a href="#2769" id="2769">2769</a>
<a href="#2770" id="2770">2770</a>
<a href="#2771" id="2771">2771</a>
<a href="#2772" id="2772">2772</a>
<a href="#2773" id="2773">2773</a>
<a href="#2774" id="2774">2774</a>
<a href="#2775" id="2775">2775</a>
<a href="#2776" id="2776">2776</a>
<a href="#2777" id="2777">2777</a>
<a href="#2778" id="2778">2778</a>
<a href="#2779" id="2779">2779</a>
<a href="#2780" id="2780">2780</a>
<a href="#2781" id="2781">2781</a>
<a href="#2782" id="2782">2782</a>
<a href="#2783" id="2783">2783</a>
<a href="#2784" id="2784">2784</a>
<a href="#2785" id="2785">2785</a>
<a href="#2786" id="2786">2786</a>
<a href="#2787" id="2787">2787</a>
<a href="#2788" id="2788">2788</a>
<a href="#2789" id="2789">2789</a>
<a href="#2790" id="2790">2790</a>
<a href="#2791" id="2791">2791</a>
<a href="#2792" id="2792">2792</a>
<a href="#2793" id="2793">2793</a>
<a href="#2794" id="2794">2794</a>
<a href="#2795" id="2795">2795</a>
<a href="#2796" id="2796">2796</a>
<a href="#2797" id="2797">2797</a>
<a href="#2798" id="2798">2798</a>
<a href="#2799" id="2799">2799</a>
<a href="#2800" id="2800">2800</a>
<a href="#2801" id="2801">2801</a>
<a href="#2802" id="2802">2802</a>
<a href="#2803" id="2803">2803</a>
<a href="#2804" id="2804">2804</a>
<a href="#2805" id="2805">2805</a>
<a href="#2806" id="2806">2806</a>
<a href="#2807" id="2807">2807</a>
<a href="#2808" id="2808">2808</a>
<a href="#2809" id="2809">2809</a>
<a href="#2810" id="2810">2810</a>
<a href="#2811" id="2811">2811</a>
<a href="#2812" id="2812">2812</a>
<a href="#2813" id="2813">2813</a>
<a href="#2814" id="2814">2814</a>
<a href="#2815" id="2815">2815</a>
<a href="#2816" id="2816">2816</a>
<a href="#2817" id="2817">2817</a>
<a href="#2818" id="2818">2818</a>
<a href="#2819" id="2819">2819</a>
<a href="#2820" id="2820">2820</a>
<a href="#2821" id="2821">2821</a>
<a href="#2822" id="2822">2822</a>
<a href="#2823" id="2823">2823</a>
<a href="#2824" id="2824">2824</a>
<a href="#2825" id="2825">2825</a>
<a href="#2826" id="2826">2826</a>
<a href="#2827" id="2827">2827</a>
<a href="#2828" id="2828">2828</a>
<a href="#2829" id="2829">2829</a>
<a href="#2830" id="2830">2830</a>
<a href="#2831" id="2831">2831</a>
<a href="#2832" id="2832">2832</a>
<a href="#2833" id="2833">2833</a>
<a href="#2834" id="2834">2834</a>
<a href="#2835" id="2835">2835</a>
<a href="#2836" id="2836">2836</a>
<a href="#2837" id="2837">2837</a>
<a href="#2838" id="2838">2838</a>
<a href="#2839" id="2839">2839</a>
<a href="#2840" id="2840">2840</a>
<a href="#2841" id="2841">2841</a>
<a href="#2842" id="2842">2842</a>
<a href="#2843" id="2843">2843</a>
<a href="#2844" id="2844">2844</a>
<a href="#2845" id="2845">2845</a>
<a href="#2846" id="2846">2846</a>
<a href="#2847" id="2847">2847</a>
<a href="#2848" id="2848">2848</a>
<a href="#2849" id="2849">2849</a>
<a href="#2850" id="2850">2850</a>
<a href="#2851" id="2851">2851</a>
<a href="#2852" id="2852">2852</a>
<a href="#2853" id="2853">2853</a>
<a href="#2854" id="2854">2854</a>
<a href="#2855" id="2855">2855</a>
<a href="#2856" id="2856">2856</a>
<a href="#2857" id="2857">2857</a>
<a href="#2858" id="2858">2858</a>
<a href="#2859" id="2859">2859</a>
<a href="#2860" id="2860">2860</a>
<a href="#2861" id="2861">2861</a>
<a href="#2862" id="2862">2862</a>
<a href="#2863" id="2863">2863</a>
<a href="#2864" id="2864">2864</a>
<a href="#2865" id="2865">2865</a>
<a href="#2866" id="2866">2866</a>
<a href="#2867" id="2867">2867</a>
<a href="#2868" id="2868">2868</a>
<a href="#2869" id="2869">2869</a>
<a href="#2870" id="2870">2870</a>
<a href="#2871" id="2871">2871</a>
<a href="#2872" id="2872">2872</a>
<a href="#2873" id="2873">2873</a>
<a href="#2874" id="2874">2874</a>
<a href="#2875" id="2875">2875</a>
<a href="#2876" id="2876">2876</a>
<a href="#2877" id="2877">2877</a>
<a href="#2878" id="2878">2878</a>
<a href="#2879" id="2879">2879</a>
<a href="#2880" id="2880">2880</a>
<a href="#2881" id="2881">2881</a>
<a href="#2882" id="2882">2882</a>
<a href="#2883" id="2883">2883</a>
<a href="#2884" id="2884">2884</a>
<a href="#2885" id="2885">2885</a>
<a href="#2886" id="2886">2886</a>
<a href="#2887" id="2887">2887</a>
<a href="#2888" id="2888">2888</a>
<a href="#2889" id="2889">2889</a>
<a href="#2890" id="2890">2890</a>
<a href="#2891" id="2891">2891</a>
<a href="#2892" id="2892">2892</a>
<a href="#2893" id="2893">2893</a>
<a href="#2894" id="2894">2894</a>
<a href="#2895" id="2895">2895</a>
<a href="#2896" id="2896">2896</a>
<a href="#2897" id="2897">2897</a>
<a href="#2898" id="2898">2898</a>
<a href="#2899" id="2899">2899</a>
<a href="#2900" id="2900">2900</a>
<a href="#2901" id="2901">2901</a>
<a href="#2902" id="2902">2902</a>
<a href="#2903" id="2903">2903</a>
<a href="#2904" id="2904">2904</a>
<a href="#2905" id="2905">2905</a>
<a href="#2906" id="2906">2906</a>
<a href="#2907" id="2907">2907</a>
<a href="#2908" id="2908">2908</a>
<a href="#2909" id="2909">2909</a>
<a href="#2910" id="2910">2910</a>
<a href="#2911" id="2911">2911</a>
<a href="#2912" id="2912">2912</a>
<a href="#2913" id="2913">2913</a>
<a href="#2914" id="2914">2914</a>
<a href="#2915" id="2915">2915</a>
<a href="#2916" id="2916">2916</a>
<a href="#2917" id="2917">2917</a>
<a href="#2918" id="2918">2918</a>
<a href="#2919" id="2919">2919</a>
<a href="#2920" id="2920">2920</a>
<a href="#2921" id="2921">2921</a>
<a href="#2922" id="2922">2922</a>
<a href="#2923" id="2923">2923</a>
<a href="#2924" id="2924">2924</a>
<a href="#2925" id="2925">2925</a>
<a href="#2926" id="2926">2926</a>
<a href="#2927" id="2927">2927</a>
<a href="#2928" id="2928">2928</a>
<a href="#2929" id="2929">2929</a>
<a href="#2930" id="2930">2930</a>
<a href="#2931" id="2931">2931</a>
<a href="#2932" id="2932">2932</a>
<a href="#2933" id="2933">2933</a>
<a href="#2934" id="2934">2934</a>
<a href="#2935" id="2935">2935</a>
<a href="#2936" id="2936">2936</a>
<a href="#2937" id="2937">2937</a>
<a href="#2938" id="2938">2938</a>
<a href="#2939" id="2939">2939</a>
<a href="#2940" id="2940">2940</a>
<a href="#2941" id="2941">2941</a>
<a href="#2942" id="2942">2942</a>
<a href="#2943" id="2943">2943</a>
<a href="#2944" id="2944">2944</a>
<a href="#2945" id="2945">2945</a>
<a href="#2946" id="2946">2946</a>
<a href="#2947" id="2947">2947</a>
<a href="#2948" id="2948">2948</a>
<a href="#2949" id="2949">2949</a>
<a href="#2950" id="2950">2950</a>
<a href="#2951" id="2951">2951</a>
<a href="#2952" id="2952">2952</a>
<a href="#2953" id="2953">2953</a>
<a href="#2954" id="2954">2954</a>
<a href="#2955" id="2955">2955</a>
<a href="#2956" id="2956">2956</a>
<a href="#2957" id="2957">2957</a>
<a href="#2958" id="2958">2958</a>
<a href="#2959" id="2959">2959</a>
<a href="#2960" id="2960">2960</a>
<a href="#2961" id="2961">2961</a>
<a href="#2962" id="2962">2962</a>
<a href="#2963" id="2963">2963</a>
<a href="#2964" id="2964">2964</a>
<a href="#2965" id="2965">2965</a>
<a href="#2966" id="2966">2966</a>
<a href="#2967" id="2967">2967</a>
<a href="#2968" id="2968">2968</a>
<a href="#2969" id="2969">2969</a>
<a href="#2970" id="2970">2970</a>
<a href="#2971" id="2971">2971</a>
<a href="#2972" id="2972">2972</a>
<a href="#2973" id="2973">2973</a>
<a href="#2974" id="2974">2974</a>
<a href="#2975" id="2975">2975</a>
<a href="#2976" id="2976">2976</a>
<a href="#2977" id="2977">2977</a>
<a href="#2978" id="2978">2978</a>
<a href="#2979" id="2979">2979</a>
<a href="#2980" id="2980">2980</a>
<a href="#2981" id="2981">2981</a>
<a href="#2982" id="2982">2982</a>
<a href="#2983" id="2983">2983</a>
<a href="#2984" id="2984">2984</a>
<a href="#2985" id="2985">2985</a>
<a href="#2986" id="2986">2986</a>
<a href="#2987" id="2987">2987</a>
<a href="#2988" id="2988">2988</a>
<a href="#2989" id="2989">2989</a>
<a href="#2990" id="2990">2990</a>
<a href="#2991" id="2991">2991</a>
<a href="#2992" id="2992">2992</a>
<a href="#2993" id="2993">2993</a>
<a href="#2994" id="2994">2994</a>
<a href="#2995" id="2995">2995</a>
<a href="#2996" id="2996">2996</a>
<a href="#2997" id="2997">2997</a>
<a href="#2998" id="2998">2998</a>
<a href="#2999" id="2999">2999</a>
<a href="#3000" id="3000">3000</a>
<a href="#3001" id="3001">3001</a>
<a href="#3002" id="3002">3002</a>
<a href="#3003" id="3003">3003</a>
<a href="#3004" id="3004">3004</a>
<a href="#3005" id="3005">3005</a>
<a href="#3006" id="3006">3006</a>
<a href="#3007" id="3007">3007</a>
<a href="#3008" id="3008">3008</a>
<a href="#3009" id="3009">3009</a>
<a href="#3010" id="3010">3010</a>
<a href="#3011" id="3011">3011</a>
<a href="#3012" id="3012">3012</a>
<a href="#3013" id="3013">3013</a>
<a href="#3014" id="3014">3014</a>
<a href="#3015" id="3015">3015</a>
<a href="#3016" id="3016">3016</a>
<a href="#3017" id="3017">3017</a>
<a href="#3018" id="3018">3018</a>
<a href="#3019" id="3019">3019</a>
<a href="#3020" id="3020">3020</a>
<a href="#3021" id="3021">3021</a>
<a href="#3022" id="3022">3022</a>
<a href="#3023" id="3023">3023</a>
<a href="#3024" id="3024">3024</a>
<a href="#3025" id="3025">3025</a>
<a href="#3026" id="3026">3026</a>
<a href="#3027" id="3027">3027</a>
<a href="#3028" id="3028">3028</a>
<a href="#3029" id="3029">3029</a>
<a href="#3030" id="3030">3030</a>
<a href="#3031" id="3031">3031</a>
<a href="#3032" id="3032">3032</a>
<a href="#3033" id="3033">3033</a>
<a href="#3034" id="3034">3034</a>
<a href="#3035" id="3035">3035</a>
<a href="#3036" id="3036">3036</a>
<a href="#3037" id="3037">3037</a>
<a href="#3038" id="3038">3038</a>
<a href="#3039" id="3039">3039</a>
<a href="#3040" id="3040">3040</a>
<a href="#3041" id="3041">3041</a>
<a href="#3042" id="3042">3042</a>
<a href="#3043" id="3043">3043</a>
<a href="#3044" id="3044">3044</a>
<a href="#3045" id="3045">3045</a>
<a href="#3046" id="3046">3046</a>
<a href="#3047" id="3047">3047</a>
<a href="#3048" id="3048">3048</a>
<a href="#3049" id="3049">3049</a>
<a href="#3050" id="3050">3050</a>
<a href="#3051" id="3051">3051</a>
<a href="#3052" id="3052">3052</a>
<a href="#3053" id="3053">3053</a>
<a href="#3054" id="3054">3054</a>
<a href="#3055" id="3055">3055</a>
<a href="#3056" id="3056">3056</a>
<a href="#3057" id="3057">3057</a>
<a href="#3058" id="3058">3058</a>
<a href="#3059" id="3059">3059</a>
<a href="#3060" id="3060">3060</a>
<a href="#3061" id="3061">3061</a>
<a href="#3062" id="3062">3062</a>
<a href="#3063" id="3063">3063</a>
<a href="#3064" id="3064">3064</a>
<a href="#3065" id="3065">3065</a>
<a href="#3066" id="3066">3066</a>
<a href="#3067" id="3067">3067</a>
<a href="#3068" id="3068">3068</a>
<a href="#3069" id="3069">3069</a>
<a href="#3070" id="3070">3070</a>
<a href="#3071" id="3071">3071</a>
<a href="#3072" id="3072">3072</a>
<a href="#3073" id="3073">3073</a>
<a href="#3074" id="3074">3074</a>
<a href="#3075" id="3075">3075</a>
<a href="#3076" id="3076">3076</a>
<a href="#3077" id="3077">3077</a>
<a href="#3078" id="3078">3078</a>
<a href="#3079" id="3079">3079</a>
<a href="#3080" id="3080">3080</a>
<a href="#3081" id="3081">3081</a>
<a href="#3082" id="3082">3082</a>
<a href="#3083" id="3083">3083</a>
<a href="#3084" id="3084">3084</a>
<a href="#3085" id="3085">3085</a>
<a href="#3086" id="3086">3086</a>
<a href="#3087" id="3087">3087</a>
<a href="#3088" id="3088">3088</a>
<a href="#3089" id="3089">3089</a>
<a href="#3090" id="3090">3090</a>
<a href="#3091" id="3091">3091</a>
<a href="#3092" id="3092">3092</a>
<a href="#3093" id="3093">3093</a>
<a href="#3094" id="3094">3094</a>
<a href="#3095" id="3095">3095</a>
<a href="#3096" id="3096">3096</a>
<a href="#3097" id="3097">3097</a>
<a href="#3098" id="3098">3098</a>
<a href="#3099" id="3099">3099</a>
<a href="#3100" id="3100">3100</a>
<a href="#3101" id="3101">3101</a>
<a href="#3102" id="3102">3102</a>
<a href="#3103" id="3103">3103</a>
<a href="#3104" id="3104">3104</a>
<a href="#3105" id="3105">3105</a>
<a href="#3106" id="3106">3106</a>
<a href="#3107" id="3107">3107</a>
<a href="#3108" id="3108">3108</a>
<a href="#3109" id="3109">3109</a>
<a href="#3110" id="3110">3110</a>
<a href="#3111" id="3111">3111</a>
<a href="#3112" id="3112">3112</a>
<a href="#3113" id="3113">3113</a>
<a href="#3114" id="3114">3114</a>
<a href="#3115" id="3115">3115</a>
<a href="#3116" id="3116">3116</a>
<a href="#3117" id="3117">3117</a>
<a href="#3118" id="3118">3118</a>
<a href="#3119" id="3119">3119</a>
<a href="#3120" id="3120">3120</a>
<a href="#3121" id="3121">3121</a>
<a href="#3122" id="3122">3122</a>
<a href="#3123" id="3123">3123</a>
<a href="#3124" id="3124">3124</a>
<a href="#3125" id="3125">3125</a>
<a href="#3126" id="3126">3126</a>
<a href="#3127" id="3127">3127</a>
<a href="#3128" id="3128">3128</a>
<a href="#3129" id="3129">3129</a>
<a href="#3130" id="3130">3130</a>
<a href="#3131" id="3131">3131</a>
<a href="#3132" id="3132">3132</a>
<a href="#3133" id="3133">3133</a>
<a href="#3134" id="3134">3134</a>
<a href="#3135" id="3135">3135</a>
<a href="#3136" id="3136">3136</a>
<a href="#3137" id="3137">3137</a>
<a href="#3138" id="3138">3138</a>
<a href="#3139" id="3139">3139</a>
<a href="#3140" id="3140">3140</a>
<a href="#3141" id="3141">3141</a>
<a href="#3142" id="3142">3142</a>
<a href="#3143" id="3143">3143</a>
<a href="#3144" id="3144">3144</a>
<a href="#3145" id="3145">3145</a>
<a href="#3146" id="3146">3146</a>
<a href="#3147" id="3147">3147</a>
<a href="#3148" id="3148">3148</a>
<a href="#3149" id="3149">3149</a>
<a href="#3150" id="3150">3150</a>
<a href="#3151" id="3151">3151</a>
<a href="#3152" id="3152">3152</a>
<a href="#3153" id="3153">3153</a>
<a href="#3154" id="3154">3154</a>
<a href="#3155" id="3155">3155</a>
<a href="#3156" id="3156">3156</a>
<a href="#3157" id="3157">3157</a>
<a href="#3158" id="3158">3158</a>
<a href="#3159" id="3159">3159</a>
<a href="#3160" id="3160">3160</a>
<a href="#3161" id="3161">3161</a>
<a href="#3162" id="3162">3162</a>
<a href="#3163" id="3163">3163</a>
<a href="#3164" id="3164">3164</a>
<a href="#3165" id="3165">3165</a>
<a href="#3166" id="3166">3166</a>
<a href="#3167" id="3167">3167</a>
<a href="#3168" id="3168">3168</a>
<a href="#3169" id="3169">3169</a>
<a href="#3170" id="3170">3170</a>
<a href="#3171" id="3171">3171</a>
<a href="#3172" id="3172">3172</a>
<a href="#3173" id="3173">3173</a>
<a href="#3174" id="3174">3174</a>
<a href="#3175" id="3175">3175</a>
<a href="#3176" id="3176">3176</a>
<a href="#3177" id="3177">3177</a>
<a href="#3178" id="3178">3178</a>
<a href="#3179" id="3179">3179</a>
<a href="#3180" id="3180">3180</a>
<a href="#3181" id="3181">3181</a>
<a href="#3182" id="3182">3182</a>
<a href="#3183" id="3183">3183</a>
<a href="#3184" id="3184">3184</a>
<a href="#3185" id="3185">3185</a>
<a href="#3186" id="3186">3186</a>
<a href="#3187" id="3187">3187</a>
<a href="#3188" id="3188">3188</a>
<a href="#3189" id="3189">3189</a>
<a href="#3190" id="3190">3190</a>
<a href="#3191" id="3191">3191</a>
<a href="#3192" id="3192">3192</a>
<a href="#3193" id="3193">3193</a>
<a href="#3194" id="3194">3194</a>
<a href="#3195" id="3195">3195</a>
<a href="#3196" id="3196">3196</a>
<a href="#3197" id="3197">3197</a>
<a href="#3198" id="3198">3198</a>
<a href="#3199" id="3199">3199</a>
<a href="#3200" id="3200">3200</a>
<a href="#3201" id="3201">3201</a>
<a href="#3202" id="3202">3202</a>
<a href="#3203" id="3203">3203</a>
<a href="#3204" id="3204">3204</a>
<a href="#3205" id="3205">3205</a>
<a href="#3206" id="3206">3206</a>
<a href="#3207" id="3207">3207</a>
<a href="#3208" id="3208">3208</a>
<a href="#3209" id="3209">3209</a>
<a href="#3210" id="3210">3210</a>
<a href="#3211" id="3211">3211</a>
<a href="#3212" id="3212">3212</a>
<a href="#3213" id="3213">3213</a>
<a href="#3214" id="3214">3214</a>
<a href="#3215" id="3215">3215</a>
<a href="#3216" id="3216">3216</a>
<a href="#3217" id="3217">3217</a>
<a href="#3218" id="3218">3218</a>
<a href="#3219" id="3219">3219</a>
<a href="#3220" id="3220">3220</a>
<a href="#3221" id="3221">3221</a>
<a href="#3222" id="3222">3222</a>
<a href="#3223" id="3223">3223</a>
<a href="#3224" id="3224">3224</a>
<a href="#3225" id="3225">3225</a>
<a href="#3226" id="3226">3226</a>
<a href="#3227" id="3227">3227</a>
<a href="#3228" id="3228">3228</a>
<a href="#3229" id="3229">3229</a>
<a href="#3230" id="3230">3230</a>
<a href="#3231" id="3231">3231</a>
<a href="#3232" id="3232">3232</a>
<a href="#3233" id="3233">3233</a>
<a href="#3234" id="3234">3234</a>
<a href="#3235" id="3235">3235</a>
<a href="#3236" id="3236">3236</a>
<a href="#3237" id="3237">3237</a>
<a href="#3238" id="3238">3238</a>
<a href="#3239" id="3239">3239</a>
<a href="#3240" id="3240">3240</a>
<a href="#3241" id="3241">3241</a>
<a href="#3242" id="3242">3242</a>
<a href="#3243" id="3243">3243</a>
<a href="#3244" id="3244">3244</a>
<a href="#3245" id="3245">3245</a>
<a href="#3246" id="3246">3246</a>
<a href="#3247" id="3247">3247</a>
<a href="#3248" id="3248">3248</a>
<a href="#3249" id="3249">3249</a>
<a href="#3250" id="3250">3250</a>
<a href="#3251" id="3251">3251</a>
<a href="#3252" id="3252">3252</a>
<a href="#3253" id="3253">3253</a>
<a href="#3254" id="3254">3254</a>
<a href="#3255" id="3255">3255</a>
<a href="#3256" id="3256">3256</a>
<a href="#3257" id="3257">3257</a>
<a href="#3258" id="3258">3258</a>
<a href="#3259" id="3259">3259</a>
<a href="#3260" id="3260">3260</a>
<a href="#3261" id="3261">3261</a>
<a href="#3262" id="3262">3262</a>
<a href="#3263" id="3263">3263</a>
<a href="#3264" id="3264">3264</a>
<a href="#3265" id="3265">3265</a>
<a href="#3266" id="3266">3266</a>
<a href="#3267" id="3267">3267</a>
<a href="#3268" id="3268">3268</a>
<a href="#3269" id="3269">3269</a>
<a href="#3270" id="3270">3270</a>
<a href="#3271" id="3271">3271</a>
<a href="#3272" id="3272">3272</a>
<a href="#3273" id="3273">3273</a>
<a href="#3274" id="3274">3274</a>
<a href="#3275" id="3275">3275</a>
<a href="#3276" id="3276">3276</a>
<a href="#3277" id="3277">3277</a>
<a href="#3278" id="3278">3278</a>
<a href="#3279" id="3279">3279</a>
<a href="#3280" id="3280">3280</a>
<a href="#3281" id="3281">3281</a>
<a href="#3282" id="3282">3282</a>
<a href="#3283" id="3283">3283</a>
<a href="#3284" id="3284">3284</a>
<a href="#3285" id="3285">3285</a>
<a href="#3286" id="3286">3286</a>
<a href="#3287" id="3287">3287</a>
<a href="#3288" id="3288">3288</a>
<a href="#3289" id="3289">3289</a>
<a href="#3290" id="3290">3290</a>
<a href="#3291" id="3291">3291</a>
<a href="#3292" id="3292">3292</a>
<a href="#3293" id="3293">3293</a>
<a href="#3294" id="3294">3294</a>
<a href="#3295" id="3295">3295</a>
<a href="#3296" id="3296">3296</a>
<a href="#3297" id="3297">3297</a>
<a href="#3298" id="3298">3298</a>
<a href="#3299" id="3299">3299</a>
<a href="#3300" id="3300">3300</a>
<a href="#3301" id="3301">3301</a>
<a href="#3302" id="3302">3302</a>
<a href="#3303" id="3303">3303</a>
<a href="#3304" id="3304">3304</a>
<a href="#3305" id="3305">3305</a>
<a href="#3306" id="3306">3306</a>
<a href="#3307" id="3307">3307</a>
<a href="#3308" id="3308">3308</a>
<a href="#3309" id="3309">3309</a>
<a href="#3310" id="3310">3310</a>
<a href="#3311" id="3311">3311</a>
<a href="#3312" id="3312">3312</a>
<a href="#3313" id="3313">3313</a>
<a href="#3314" id="3314">3314</a>
<a href="#3315" id="3315">3315</a>
<a href="#3316" id="3316">3316</a>
<a href="#3317" id="3317">3317</a>
<a href="#3318" id="3318">3318</a>
<a href="#3319" id="3319">3319</a>
<a href="#3320" id="3320">3320</a>
<a href="#3321" id="3321">3321</a>
<a href="#3322" id="3322">3322</a>
<a href="#3323" id="3323">3323</a>
<a href="#3324" id="3324">3324</a>
<a href="#3325" id="3325">3325</a>
<a href="#3326" id="3326">3326</a>
<a href="#3327" id="3327">3327</a>
<a href="#3328" id="3328">3328</a>
<a href="#3329" id="3329">3329</a>
<a href="#3330" id="3330">3330</a>
<a href="#3331" id="3331">3331</a>
<a href="#3332" id="3332">3332</a>
<a href="#3333" id="3333">3333</a>
<a href="#3334" id="3334">3334</a>
<a href="#3335" id="3335">3335</a>
<a href="#3336" id="3336">3336</a>
<a href="#3337" id="3337">3337</a>
<a href="#3338" id="3338">3338</a>
<a href="#3339" id="3339">3339</a>
<a href="#3340" id="3340">3340</a>
<a href="#3341" id="3341">3341</a>
<a href="#3342" id="3342">3342</a>
<a href="#3343" id="3343">3343</a>
<a href="#3344" id="3344">3344</a>
<a href="#3345" id="3345">3345</a>
<a href="#3346" id="3346">3346</a>
<a href="#3347" id="3347">3347</a>
<a href="#3348" id="3348">3348</a>
<a href="#3349" id="3349">3349</a>
<a href="#3350" id="3350">3350</a>
<a href="#3351" id="3351">3351</a>
<a href="#3352" id="3352">3352</a>
<a href="#3353" id="3353">3353</a>
<a href="#3354" id="3354">3354</a>
<a href="#3355" id="3355">3355</a>
<a href="#3356" id="3356">3356</a>
<a href="#3357" id="3357">3357</a>
<a href="#3358" id="3358">3358</a>
<a href="#3359" id="3359">3359</a>
<a href="#3360" id="3360">3360</a>
<a href="#3361" id="3361">3361</a>
<a href="#3362" id="3362">3362</a>
<a href="#3363" id="3363">3363</a>
<a href="#3364" id="3364">3364</a>
<a href="#3365" id="3365">3365</a>
<a href="#3366" id="3366">3366</a>
<a href="#3367" id="3367">3367</a>
<a href="#3368" id="3368">3368</a>
<a href="#3369" id="3369">3369</a>
<a href="#3370" id="3370">3370</a>
<a href="#3371" id="3371">3371</a>
<a href="#3372" id="3372">3372</a>
<a href="#3373" id="3373">3373</a>
<a href="#3374" id="3374">3374</a>
<a href="#3375" id="3375">3375</a>
<a href="#3376" id="3376">3376</a>
<a href="#3377" id="3377">3377</a>
<a href="#3378" id="3378">3378</a>
<a href="#3379" id="3379">3379</a>
<a href="#3380" id="3380">3380</a>
<a href="#3381" id="3381">3381</a>
<a href="#3382" id="3382">3382</a>
<a href="#3383" id="3383">3383</a>
<a href="#3384" id="3384">3384</a>
<a href="#3385" id="3385">3385</a>
<a href="#3386" id="3386">3386</a>
<a href="#3387" id="3387">3387</a>
<a href="#3388" id="3388">3388</a>
<a href="#3389" id="3389">3389</a>
<a href="#3390" id="3390">3390</a>
<a href="#3391" id="3391">3391</a>
<a href="#3392" id="3392">3392</a>
<a href="#3393" id="3393">3393</a>
<a href="#3394" id="3394">3394</a>
<a href="#3395" id="3395">3395</a>
<a href="#3396" id="3396">3396</a>
<a href="#3397" id="3397">3397</a>
<a href="#3398" id="3398">3398</a>
<a href="#3399" id="3399">3399</a>
<a href="#3400" id="3400">3400</a>
<a href="#3401" id="3401">3401</a>
<a href="#3402" id="3402">3402</a>
<a href="#3403" id="3403">3403</a>
<a href="#3404" id="3404">3404</a>
<a href="#3405" id="3405">3405</a>
<a href="#3406" id="3406">3406</a>
<a href="#3407" id="3407">3407</a>
<a href="#3408" id="3408">3408</a>
<a href="#3409" id="3409">3409</a>
<a href="#3410" id="3410">3410</a>
<a href="#3411" id="3411">3411</a>
<a href="#3412" id="3412">3412</a>
<a href="#3413" id="3413">3413</a>
<a href="#3414" id="3414">3414</a>
<a href="#3415" id="3415">3415</a>
<a href="#3416" id="3416">3416</a>
<a href="#3417" id="3417">3417</a>
<a href="#3418" id="3418">3418</a>
<a href="#3419" id="3419">3419</a>
<a href="#3420" id="3420">3420</a>
<a href="#3421" id="3421">3421</a>
<a href="#3422" id="3422">3422</a>
<a href="#3423" id="3423">3423</a>
<a href="#3424" id="3424">3424</a>
<a href="#3425" id="3425">3425</a>
<a href="#3426" id="3426">3426</a>
<a href="#3427" id="3427">3427</a>
<a href="#3428" id="3428">3428</a>
<a href="#3429" id="3429">3429</a>
<a href="#3430" id="3430">3430</a>
<a href="#3431" id="3431">3431</a>
<a href="#3432" id="3432">3432</a>
<a href="#3433" id="3433">3433</a>
<a href="#3434" id="3434">3434</a>
<a href="#3435" id="3435">3435</a>
<a href="#3436" id="3436">3436</a>
<a href="#3437" id="3437">3437</a>
<a href="#3438" id="3438">3438</a>
<a href="#3439" id="3439">3439</a>
<a href="#3440" id="3440">3440</a>
<a href="#3441" id="3441">3441</a>
<a href="#3442" id="3442">3442</a>
<a href="#3443" id="3443">3443</a>
<a href="#3444" id="3444">3444</a>
<a href="#3445" id="3445">3445</a>
<a href="#3446" id="3446">3446</a>
<a href="#3447" id="3447">3447</a>
<a href="#3448" id="3448">3448</a>
<a href="#3449" id="3449">3449</a>
<a href="#3450" id="3450">3450</a>
<a href="#3451" id="3451">3451</a>
<a href="#3452" id="3452">3452</a>
<a href="#3453" id="3453">3453</a>
<a href="#3454" id="3454">3454</a>
<a href="#3455" id="3455">3455</a>
<a href="#3456" id="3456">3456</a>
<a href="#3457" id="3457">3457</a>
<a href="#3458" id="3458">3458</a>
<a href="#3459" id="3459">3459</a>
<a href="#3460" id="3460">3460</a>
<a href="#3461" id="3461">3461</a>
<a href="#3462" id="3462">3462</a>
<a href="#3463" id="3463">3463</a>
<a href="#3464" id="3464">3464</a>
<a href="#3465" id="3465">3465</a>
<a href="#3466" id="3466">3466</a>
<a href="#3467" id="3467">3467</a>
<a href="#3468" id="3468">3468</a>
<a href="#3469" id="3469">3469</a>
<a href="#3470" id="3470">3470</a>
<a href="#3471" id="3471">3471</a>
<a href="#3472" id="3472">3472</a>
<a href="#3473" id="3473">3473</a>
<a href="#3474" id="3474">3474</a>
<a href="#3475" id="3475">3475</a>
<a href="#3476" id="3476">3476</a>
<a href="#3477" id="3477">3477</a>
<a href="#3478" id="3478">3478</a>
<a href="#3479" id="3479">3479</a>
<a href="#3480" id="3480">3480</a>
<a href="#3481" id="3481">3481</a>
<a href="#3482" id="3482">3482</a>
<a href="#3483" id="3483">3483</a>
<a href="#3484" id="3484">3484</a>
<a href="#3485" id="3485">3485</a>
<a href="#3486" id="3486">3486</a>
<a href="#3487" id="3487">3487</a>
<a href="#3488" id="3488">3488</a>
<a href="#3489" id="3489">3489</a>
<a href="#3490" id="3490">3490</a>
<a href="#3491" id="3491">3491</a>
<a href="#3492" id="3492">3492</a>
<a href="#3493" id="3493">3493</a>
<a href="#3494" id="3494">3494</a>
<a href="#3495" id="3495">3495</a>
<a href="#3496" id="3496">3496</a>
<a href="#3497" id="3497">3497</a>
<a href="#3498" id="3498">3498</a>
<a href="#3499" id="3499">3499</a>
<a href="#3500" id="3500">3500</a>
<a href="#3501" id="3501">3501</a>
<a href="#3502" id="3502">3502</a>
<a href="#3503" id="3503">3503</a>
<a href="#3504" id="3504">3504</a>
<a href="#3505" id="3505">3505</a>
<a href="#3506" id="3506">3506</a>
<a href="#3507" id="3507">3507</a>
<a href="#3508" id="3508">3508</a>
<a href="#3509" id="3509">3509</a>
<a href="#3510" id="3510">3510</a>
<a href="#3511" id="3511">3511</a>
<a href="#3512" id="3512">3512</a>
<a href="#3513" id="3513">3513</a>
<a href="#3514" id="3514">3514</a>
<a href="#3515" id="3515">3515</a>
<a href="#3516" id="3516">3516</a>
<a href="#3517" id="3517">3517</a>
<a href="#3518" id="3518">3518</a>
<a href="#3519" id="3519">3519</a>
<a href="#3520" id="3520">3520</a>
<a href="#3521" id="3521">3521</a>
<a href="#3522" id="3522">3522</a>
<a href="#3523" id="3523">3523</a>
<a href="#3524" id="3524">3524</a>
<a href="#3525" id="3525">3525</a>
<a href="#3526" id="3526">3526</a>
<a href="#3527" id="3527">3527</a>
<a href="#3528" id="3528">3528</a>
<a href="#3529" id="3529">3529</a>
<a href="#3530" id="3530">3530</a>
<a href="#3531" id="3531">3531</a>
<a href="#3532" id="3532">3532</a>
<a href="#3533" id="3533">3533</a>
<a href="#3534" id="3534">3534</a>
<a href="#3535" id="3535">3535</a>
<a href="#3536" id="3536">3536</a>
<a href="#3537" id="3537">3537</a>
<a href="#3538" id="3538">3538</a>
<a href="#3539" id="3539">3539</a>
<a href="#3540" id="3540">3540</a>
<a href="#3541" id="3541">3541</a>
<a href="#3542" id="3542">3542</a>
<a href="#3543" id="3543">3543</a>
<a href="#3544" id="3544">3544</a>
<a href="#3545" id="3545">3545</a>
<a href="#3546" id="3546">3546</a>
<a href="#3547" id="3547">3547</a>
<a href="#3548" id="3548">3548</a>
<a href="#3549" id="3549">3549</a>
<a href="#3550" id="3550">3550</a>
<a href="#3551" id="3551">3551</a>
<a href="#3552" id="3552">3552</a>
<a href="#3553" id="3553">3553</a>
<a href="#3554" id="3554">3554</a>
<a href="#3555" id="3555">3555</a>
<a href="#3556" id="3556">3556</a>
<a href="#3557" id="3557">3557</a>
<a href="#3558" id="3558">3558</a>
<a href="#3559" id="3559">3559</a>
<a href="#3560" id="3560">3560</a>
<a href="#3561" id="3561">3561</a>
<a href="#3562" id="3562">3562</a>
<a href="#3563" id="3563">3563</a>
<a href="#3564" id="3564">3564</a>
<a href="#3565" id="3565">3565</a>
<a href="#3566" id="3566">3566</a>
<a href="#3567" id="3567">3567</a>
<a href="#3568" id="3568">3568</a>
<a href="#3569" id="3569">3569</a>
<a href="#3570" id="3570">3570</a>
<a href="#3571" id="3571">3571</a>
<a href="#3572" id="3572">3572</a>
<a href="#3573" id="3573">3573</a>
<a href="#3574" id="3574">3574</a>
<a href="#3575" id="3575">3575</a>
<a href="#3576" id="3576">3576</a>
<a href="#3577" id="3577">3577</a>
<a href="#3578" id="3578">3578</a>
<a href="#3579" id="3579">3579</a>
<a href="#3580" id="3580">3580</a>
<a href="#3581" id="3581">3581</a>
<a href="#3582" id="3582">3582</a>
<a href="#3583" id="3583">3583</a>
<a href="#3584" id="3584">3584</a>
<a href="#3585" id="3585">3585</a>
<a href="#3586" id="3586">3586</a>
<a href="#3587" id="3587">3587</a>
<a href="#3588" id="3588">3588</a>
<a href="#3589" id="3589">3589</a>
<a href="#3590" id="3590">3590</a>
<a href="#3591" id="3591">3591</a>
<a href="#3592" id="3592">3592</a>
<a href="#3593" id="3593">3593</a>
<a href="#3594" id="3594">3594</a>
<a href="#3595" id="3595">3595</a>
<a href="#3596" id="3596">3596</a>
<a href="#3597" id="3597">3597</a>
<a href="#3598" id="3598">3598</a>
<a href="#3599" id="3599">3599</a>
<a href="#3600" id="3600">3600</a>
<a href="#3601" id="3601">3601</a>
<a href="#3602" id="3602">3602</a>
<a href="#3603" id="3603">3603</a>
<a href="#3604" id="3604">3604</a>
<a href="#3605" id="3605">3605</a>
<a href="#3606" id="3606">3606</a>
<a href="#3607" id="3607">3607</a>
<a href="#3608" id="3608">3608</a>
<a href="#3609" id="3609">3609</a>
<a href="#3610" id="3610">3610</a>
<a href="#3611" id="3611">3611</a>
<a href="#3612" id="3612">3612</a>
<a href="#3613" id="3613">3613</a>
<a href="#3614" id="3614">3614</a>
<a href="#3615" id="3615">3615</a>
<a href="#3616" id="3616">3616</a>
<a href="#3617" id="3617">3617</a>
<a href="#3618" id="3618">3618</a>
<a href="#3619" id="3619">3619</a>
<a href="#3620" id="3620">3620</a>
<a href="#3621" id="3621">3621</a>
<a href="#3622" id="3622">3622</a>
<a href="#3623" id="3623">3623</a>
<a href="#3624" id="3624">3624</a>
<a href="#3625" id="3625">3625</a>
<a href="#3626" id="3626">3626</a>
<a href="#3627" id="3627">3627</a>
<a href="#3628" id="3628">3628</a>
<a href="#3629" id="3629">3629</a>
<a href="#3630" id="3630">3630</a>
<a href="#3631" id="3631">3631</a>
<a href="#3632" id="3632">3632</a>
<a href="#3633" id="3633">3633</a>
<a href="#3634" id="3634">3634</a>
<a href="#3635" id="3635">3635</a>
<a href="#3636" id="3636">3636</a>
<a href="#3637" id="3637">3637</a>
<a href="#3638" id="3638">3638</a>
<a href="#3639" id="3639">3639</a>
<a href="#3640" id="3640">3640</a>
<a href="#3641" id="3641">3641</a>
<a href="#3642" id="3642">3642</a>
<a href="#3643" id="3643">3643</a>
<a href="#3644" id="3644">3644</a>
<a href="#3645" id="3645">3645</a>
<a href="#3646" id="3646">3646</a>
<a href="#3647" id="3647">3647</a>
<a href="#3648" id="3648">3648</a>
<a href="#3649" id="3649">3649</a>
<a href="#3650" id="3650">3650</a>
<a href="#3651" id="3651">3651</a>
<a href="#3652" id="3652">3652</a>
<a href="#3653" id="3653">3653</a>
<a href="#3654" id="3654">3654</a>
<a href="#3655" id="3655">3655</a>
<a href="#3656" id="3656">3656</a>
<a href="#3657" id="3657">3657</a>
<a href="#3658" id="3658">3658</a>
<a href="#3659" id="3659">3659</a>
<a href="#3660" id="3660">3660</a>
<a href="#3661" id="3661">3661</a>
<a href="#3662" id="3662">3662</a>
<a href="#3663" id="3663">3663</a>
<a href="#3664" id="3664">3664</a>
<a href="#3665" id="3665">3665</a>
<a href="#3666" id="3666">3666</a>
<a href="#3667" id="3667">3667</a>
<a href="#3668" id="3668">3668</a>
<a href="#3669" id="3669">3669</a>
<a href="#3670" id="3670">3670</a>
<a href="#3671" id="3671">3671</a>
<a href="#3672" id="3672">3672</a>
<a href="#3673" id="3673">3673</a>
<a href="#3674" id="3674">3674</a>
<a href="#3675" id="3675">3675</a>
<a href="#3676" id="3676">3676</a>
<a href="#3677" id="3677">3677</a>
<a href="#3678" id="3678">3678</a>
<a href="#3679" id="3679">3679</a>
<a href="#3680" id="3680">3680</a>
<a href="#3681" id="3681">3681</a>
<a href="#3682" id="3682">3682</a>
<a href="#3683" id="3683">3683</a>
<a href="#3684" id="3684">3684</a>
<a href="#3685" id="3685">3685</a>
<a href="#3686" id="3686">3686</a>
<a href="#3687" id="3687">3687</a>
<a href="#3688" id="3688">3688</a>
<a href="#3689" id="3689">3689</a>
<a href="#3690" id="3690">3690</a>
<a href="#3691" id="3691">3691</a>
<a href="#3692" id="3692">3692</a>
<a href="#3693" id="3693">3693</a>
<a href="#3694" id="3694">3694</a>
<a href="#3695" id="3695">3695</a>
<a href="#3696" id="3696">3696</a>
<a href="#3697" id="3697">3697</a>
<a href="#3698" id="3698">3698</a>
<a href="#3699" id="3699">3699</a>
<a href="#3700" id="3700">3700</a>
<a href="#3701" id="3701">3701</a>
<a href="#3702" id="3702">3702</a>
<a href="#3703" id="3703">3703</a>
<a href="#3704" id="3704">3704</a>
<a href="#3705" id="3705">3705</a>
<a href="#3706" id="3706">3706</a>
<a href="#3707" id="3707">3707</a>
<a href="#3708" id="3708">3708</a>
<a href="#3709" id="3709">3709</a>
<a href="#3710" id="3710">3710</a>
<a href="#3711" id="3711">3711</a>
<a href="#3712" id="3712">3712</a>
<a href="#3713" id="3713">3713</a>
<a href="#3714" id="3714">3714</a>
<a href="#3715" id="3715">3715</a>
<a href="#3716" id="3716">3716</a>
<a href="#3717" id="3717">3717</a>
<a href="#3718" id="3718">3718</a>
<a href="#3719" id="3719">3719</a>
<a href="#3720" id="3720">3720</a>
<a href="#3721" id="3721">3721</a>
<a href="#3722" id="3722">3722</a>
<a href="#3723" id="3723">3723</a>
<a href="#3724" id="3724">3724</a>
<a href="#3725" id="3725">3725</a>
<a href="#3726" id="3726">3726</a>
<a href="#3727" id="3727">3727</a>
<a href="#3728" id="3728">3728</a>
<a href="#3729" id="3729">3729</a>
<a href="#3730" id="3730">3730</a>
<a href="#3731" id="3731">3731</a>
<a href="#3732" id="3732">3732</a>
<a href="#3733" id="3733">3733</a>
<a href="#3734" id="3734">3734</a>
<a href="#3735" id="3735">3735</a>
<a href="#3736" id="3736">3736</a>
<a href="#3737" id="3737">3737</a>
<a href="#3738" id="3738">3738</a>
<a href="#3739" id="3739">3739</a>
<a href="#3740" id="3740">3740</a>
<a href="#3741" id="3741">3741</a>
<a href="#3742" id="3742">3742</a>
<a href="#3743" id="3743">3743</a>
<a href="#3744" id="3744">3744</a>
<a href="#3745" id="3745">3745</a>
<a href="#3746" id="3746">3746</a>
<a href="#3747" id="3747">3747</a>
<a href="#3748" id="3748">3748</a>
<a href="#3749" id="3749">3749</a>
<a href="#3750" id="3750">3750</a>
<a href="#3751" id="3751">3751</a>
<a href="#3752" id="3752">3752</a>
<a href="#3753" id="3753">3753</a>
<a href="#3754" id="3754">3754</a>
<a href="#3755" id="3755">3755</a>
<a href="#3756" id="3756">3756</a>
<a href="#3757" id="3757">3757</a>
<a href="#3758" id="3758">3758</a>
<a href="#3759" id="3759">3759</a>
<a href="#3760" id="3760">3760</a>
<a href="#3761" id="3761">3761</a>
<a href="#3762" id="3762">3762</a>
<a href="#3763" id="3763">3763</a>
<a href="#3764" id="3764">3764</a>
<a href="#3765" id="3765">3765</a>
<a href="#3766" id="3766">3766</a>
<a href="#3767" id="3767">3767</a>
<a href="#3768" id="3768">3768</a>
<a href="#3769" id="3769">3769</a>
<a href="#3770" id="3770">3770</a>
<a href="#3771" id="3771">3771</a>
<a href="#3772" id="3772">3772</a>
<a href="#3773" id="3773">3773</a>
<a href="#3774" id="3774">3774</a>
<a href="#3775" id="3775">3775</a>
<a href="#3776" id="3776">3776</a>
<a href="#3777" id="3777">3777</a>
<a href="#3778" id="3778">3778</a>
<a href="#3779" id="3779">3779</a>
<a href="#3780" id="3780">3780</a>
<a href="#3781" id="3781">3781</a>
<a href="#3782" id="3782">3782</a>
<a href="#3783" id="3783">3783</a>
<a href="#3784" id="3784">3784</a>
<a href="#3785" id="3785">3785</a>
<a href="#3786" id="3786">3786</a>
<a href="#3787" id="3787">3787</a>
<a href="#3788" id="3788">3788</a>
<a href="#3789" id="3789">3789</a>
<a href="#3790" id="3790">3790</a>
<a href="#3791" id="3791">3791</a>
<a href="#3792" id="3792">3792</a>
<a href="#3793" id="3793">3793</a>
<a href="#3794" id="3794">3794</a>
<a href="#3795" id="3795">3795</a>
<a href="#3796" id="3796">3796</a>
<a href="#3797" id="3797">3797</a>
<a href="#3798" id="3798">3798</a>
<a href="#3799" id="3799">3799</a>
<a href="#3800" id="3800">3800</a>
<a href="#3801" id="3801">3801</a>
<a href="#3802" id="3802">3802</a>
<a href="#3803" id="3803">3803</a>
<a href="#3804" id="3804">3804</a>
<a href="#3805" id="3805">3805</a>
<a href="#3806" id="3806">3806</a>
<a href="#3807" id="3807">3807</a>
<a href="#3808" id="3808">3808</a>
<a href="#3809" id="3809">3809</a>
<a href="#3810" id="3810">3810</a>
<a href="#3811" id="3811">3811</a>
<a href="#3812" id="3812">3812</a>
<a href="#3813" id="3813">3813</a>
<a href="#3814" id="3814">3814</a>
<a href="#3815" id="3815">3815</a>
<a href="#3816" id="3816">3816</a>
<a href="#3817" id="3817">3817</a>
<a href="#3818" id="3818">3818</a>
<a href="#3819" id="3819">3819</a>
<a href="#3820" id="3820">3820</a>
<a href="#3821" id="3821">3821</a>
<a href="#3822" id="3822">3822</a>
<a href="#3823" id="3823">3823</a>
<a href="#3824" id="3824">3824</a>
<a href="#3825" id="3825">3825</a>
<a href="#3826" id="3826">3826</a>
<a href="#3827" id="3827">3827</a>
<a href="#3828" id="3828">3828</a>
<a href="#3829" id="3829">3829</a>
<a href="#3830" id="3830">3830</a>
<a href="#3831" id="3831">3831</a>
<a href="#3832" id="3832">3832</a>
<a href="#3833" id="3833">3833</a>
<a href="#3834" id="3834">3834</a>
<a href="#3835" id="3835">3835</a>
<a href="#3836" id="3836">3836</a>
<a href="#3837" id="3837">3837</a>
<a href="#3838" id="3838">3838</a>
<a href="#3839" id="3839">3839</a>
<a href="#3840" id="3840">3840</a>
<a href="#3841" id="3841">3841</a>
<a href="#3842" id="3842">3842</a>
<a href="#3843" id="3843">3843</a>
<a href="#3844" id="3844">3844</a>
<a href="#3845" id="3845">3845</a>
<a href="#3846" id="3846">3846</a>
<a href="#3847" id="3847">3847</a>
<a href="#3848" id="3848">3848</a>
<a href="#3849" id="3849">3849</a>
<a href="#3850" id="3850">3850</a>
<a href="#3851" id="3851">3851</a>
<a href="#3852" id="3852">3852</a>
<a href="#3853" id="3853">3853</a>
<a href="#3854" id="3854">3854</a>
<a href="#3855" id="3855">3855</a>
<a href="#3856" id="3856">3856</a>
<a href="#3857" id="3857">3857</a>
<a href="#3858" id="3858">3858</a>
<a href="#3859" id="3859">3859</a>
<a href="#3860" id="3860">3860</a>
<a href="#3861" id="3861">3861</a>
<a href="#3862" id="3862">3862</a>
<a href="#3863" id="3863">3863</a>
<a href="#3864" id="3864">3864</a>
<a href="#3865" id="3865">3865</a>
<a href="#3866" id="3866">3866</a>
<a href="#3867" id="3867">3867</a>
<a href="#3868" id="3868">3868</a>
<a href="#3869" id="3869">3869</a>
<a href="#3870" id="3870">3870</a>
<a href="#3871" id="3871">3871</a>
<a href="#3872" id="3872">3872</a>
<a href="#3873" id="3873">3873</a>
<a href="#3874" id="3874">3874</a>
<a href="#3875" id="3875">3875</a>
<a href="#3876" id="3876">3876</a>
<a href="#3877" id="3877">3877</a>
<a href="#3878" id="3878">3878</a>
<a href="#3879" id="3879">3879</a>
<a href="#3880" id="3880">3880</a>
<a href="#3881" id="3881">3881</a>
<a href="#3882" id="3882">3882</a>
<a href="#3883" id="3883">3883</a>
<a href="#3884" id="3884">3884</a>
<a href="#3885" id="3885">3885</a>
<a href="#3886" id="3886">3886</a>
<a href="#3887" id="3887">3887</a>
<a href="#3888" id="3888">3888</a>
<a href="#3889" id="3889">3889</a>
<a href="#3890" id="3890">3890</a>
<a href="#3891" id="3891">3891</a>
<a href="#3892" id="3892">3892</a>
<a href="#3893" id="3893">3893</a>
<a href="#3894" id="3894">3894</a>
<a href="#3895" id="3895">3895</a>
<a href="#3896" id="3896">3896</a>
<a href="#3897" id="3897">3897</a>
<a href="#3898" id="3898">3898</a>
<a href="#3899" id="3899">3899</a>
<a href="#3900" id="3900">3900</a>
<a href="#3901" id="3901">3901</a>
<a href="#3902" id="3902">3902</a>
<a href="#3903" id="3903">3903</a>
<a href="#3904" id="3904">3904</a>
<a href="#3905" id="3905">3905</a>
<a href="#3906" id="3906">3906</a>
<a href="#3907" id="3907">3907</a>
<a href="#3908" id="3908">3908</a>
<a href="#3909" id="3909">3909</a>
<a href="#3910" id="3910">3910</a>
<a href="#3911" id="3911">3911</a>
<a href="#3912" id="3912">3912</a>
<a href="#3913" id="3913">3913</a>
<a href="#3914" id="3914">3914</a>
<a href="#3915" id="3915">3915</a>
<a href="#3916" id="3916">3916</a>
<a href="#3917" id="3917">3917</a>
<a href="#3918" id="3918">3918</a>
<a href="#3919" id="3919">3919</a>
<a href="#3920" id="3920">3920</a>
<a href="#3921" id="3921">3921</a>
<a href="#3922" id="3922">3922</a>
<a href="#3923" id="3923">3923</a>
<a href="#3924" id="3924">3924</a>
<a href="#3925" id="3925">3925</a>
<a href="#3926" id="3926">3926</a>
<a href="#3927" id="3927">3927</a>
<a href="#3928" id="3928">3928</a>
<a href="#3929" id="3929">3929</a>
<a href="#3930" id="3930">3930</a>
<a href="#3931" id="3931">3931</a>
<a href="#3932" id="3932">3932</a>
<a href="#3933" id="3933">3933</a>
<a href="#3934" id="3934">3934</a>
<a href="#3935" id="3935">3935</a>
<a href="#3936" id="3936">3936</a>
<a href="#3937" id="3937">3937</a>
<a href="#3938" id="3938">3938</a>
<a href="#3939" id="3939">3939</a>
<a href="#3940" id="3940">3940</a>
<a href="#3941" id="3941">3941</a>
<a href="#3942" id="3942">3942</a>
<a href="#3943" id="3943">3943</a>
<a href="#3944" id="3944">3944</a>
<a href="#3945" id="3945">3945</a>
<a href="#3946" id="3946">3946</a>
<a href="#3947" id="3947">3947</a>
<a href="#3948" id="3948">3948</a>
<a href="#3949" id="3949">3949</a>
<a href="#3950" id="3950">3950</a>
<a href="#3951" id="3951">3951</a>
<a href="#3952" id="3952">3952</a>
<a href="#3953" id="3953">3953</a>
<a href="#3954" id="3954">3954</a>
<a href="#3955" id="3955">3955</a>
<a href="#3956" id="3956">3956</a>
<a href="#3957" id="3957">3957</a>
<a href="#3958" id="3958">3958</a>
<a href="#3959" id="3959">3959</a>
<a href="#3960" id="3960">3960</a>
<a href="#3961" id="3961">3961</a>
<a href="#3962" id="3962">3962</a>
<a href="#3963" id="3963">3963</a>
<a href="#3964" id="3964">3964</a>
<a href="#3965" id="3965">3965</a>
<a href="#3966" id="3966">3966</a>
<a href="#3967" id="3967">3967</a>
<a href="#3968" id="3968">3968</a>
<a href="#3969" id="3969">3969</a>
<a href="#3970" id="3970">3970</a>
<a href="#3971" id="3971">3971</a>
<a href="#3972" id="3972">3972</a>
<a href="#3973" id="3973">3973</a>
<a href="#3974" id="3974">3974</a>
<a href="#3975" id="3975">3975</a>
<a href="#3976" id="3976">3976</a>
<a href="#3977" id="3977">3977</a>
<a href="#3978" id="3978">3978</a>
<a href="#3979" id="3979">3979</a>
<a href="#3980" id="3980">3980</a>
<a href="#3981" id="3981">3981</a>
<a href="#3982" id="3982">3982</a>
<a href="#3983" id="3983">3983</a>
<a href="#3984" id="3984">3984</a>
<a href="#3985" id="3985">3985</a>
<a href="#3986" id="3986">3986</a>
<a href="#3987" id="3987">3987</a>
<a href="#3988" id="3988">3988</a>
<a href="#3989" id="3989">3989</a>
<a href="#3990" id="3990">3990</a>
<a href="#3991" id="3991">3991</a>
<a href="#3992" id="3992">3992</a>
<a href="#3993" id="3993">3993</a>
<a href="#3994" id="3994">3994</a>
<a href="#3995" id="3995">3995</a>
<a href="#3996" id="3996">3996</a>
<a href="#3997" id="3997">3997</a>
<a href="#3998" id="3998">3998</a>
<a href="#3999" id="3999">3999</a>
<a href="#4000" id="4000">4000</a>
<a href="#4001" id="4001">4001</a>
<a href="#4002" id="4002">4002</a>
<a href="#4003" id="4003">4003</a>
<a href="#4004" id="4004">4004</a>
<a href="#4005" id="4005">4005</a>
<a href="#4006" id="4006">4006</a>
<a href="#4007" id="4007">4007</a>
<a href="#4008" id="4008">4008</a>
<a href="#4009" id="4009">4009</a>
<a href="#4010" id="4010">4010</a>
<a href="#4011" id="4011">4011</a>
<a href="#4012" id="4012">4012</a>
<a href="#4013" id="4013">4013</a>
<a href="#4014" id="4014">4014</a>
<a href="#4015" id="4015">4015</a>
<a href="#4016" id="4016">4016</a>
<a href="#4017" id="4017">4017</a>
<a href="#4018" id="4018">4018</a>
<a href="#4019" id="4019">4019</a>
<a href="#4020" id="4020">4020</a>
<a href="#4021" id="4021">4021</a>
<a href="#4022" id="4022">4022</a>
<a href="#4023" id="4023">4023</a>
<a href="#4024" id="4024">4024</a>
<a href="#4025" id="4025">4025</a>
<a href="#4026" id="4026">4026</a>
<a href="#4027" id="4027">4027</a>
<a href="#4028" id="4028">4028</a>
<a href="#4029" id="4029">4029</a>
<a href="#4030" id="4030">4030</a>
<a href="#4031" id="4031">4031</a>
<a href="#4032" id="4032">4032</a>
<a href="#4033" id="4033">4033</a>
<a href="#4034" id="4034">4034</a>
<a href="#4035" id="4035">4035</a>
<a href="#4036" id="4036">4036</a>
<a href="#4037" id="4037">4037</a>
<a href="#4038" id="4038">4038</a>
<a href="#4039" id="4039">4039</a>
<a href="#4040" id="4040">4040</a>
<a href="#4041" id="4041">4041</a>
<a href="#4042" id="4042">4042</a>
<a href="#4043" id="4043">4043</a>
<a href="#4044" id="4044">4044</a>
<a href="#4045" id="4045">4045</a>
<a href="#4046" id="4046">4046</a>
<a href="#4047" id="4047">4047</a>
<a href="#4048" id="4048">4048</a>
<a href="#4049" id="4049">4049</a>
<a href="#4050" id="4050">4050</a>
<a href="#4051" id="4051">4051</a>
<a href="#4052" id="4052">4052</a>
<a href="#4053" id="4053">4053</a>
<a href="#4054" id="4054">4054</a>
<a href="#4055" id="4055">4055</a>
<a href="#4056" id="4056">4056</a>
<a href="#4057" id="4057">4057</a>
<a href="#4058" id="4058">4058</a>
<a href="#4059" id="4059">4059</a>
<a href="#4060" id="4060">4060</a>
<a href="#4061" id="4061">4061</a>
<a href="#4062" id="4062">4062</a>
<a href="#4063" id="4063">4063</a>
<a href="#4064" id="4064">4064</a>
<a href="#4065" id="4065">4065</a>
<a href="#4066" id="4066">4066</a>
<a href="#4067" id="4067">4067</a>
<a href="#4068" id="4068">4068</a>
<a href="#4069" id="4069">4069</a>
<a href="#4070" id="4070">4070</a>
<a href="#4071" id="4071">4071</a>
<a href="#4072" id="4072">4072</a>
<a href="#4073" id="4073">4073</a>
<a href="#4074" id="4074">4074</a>
<a href="#4075" id="4075">4075</a>
<a href="#4076" id="4076">4076</a>
<a href="#4077" id="4077">4077</a>
<a href="#4078" id="4078">4078</a>
<a href="#4079" id="4079">4079</a>
<a href="#4080" id="4080">4080</a>
<a href="#4081" id="4081">4081</a>
<a href="#4082" id="4082">4082</a>
<a href="#4083" id="4083">4083</a>
<a href="#4084" id="4084">4084</a>
<a href="#4085" id="4085">4085</a>
<a href="#4086" id="4086">4086</a>
<a href="#4087" id="4087">4087</a>
<a href="#4088" id="4088">4088</a>
<a href="#4089" id="4089">4089</a>
<a href="#4090" id="4090">4090</a>
<a href="#4091" id="4091">4091</a>
<a href="#4092" id="4092">4092</a>
<a href="#4093" id="4093">4093</a>
<a href="#4094" id="4094">4094</a>
<a href="#4095" id="4095">4095</a>
<a href="#4096" id="4096">4096</a>
<a href="#4097" id="4097">4097</a>
<a href="#4098" id="4098">4098</a>
<a href="#4099" id="4099">4099</a>
<a href="#4100" id="4100">4100</a>
<a href="#4101" id="4101">4101</a>
<a href="#4102" id="4102">4102</a>
<a href="#4103" id="4103">4103</a>
<a href="#4104" id="4104">4104</a>
<a href="#4105" id="4105">4105</a>
<a href="#4106" id="4106">4106</a>
<a href="#4107" id="4107">4107</a>
<a href="#4108" id="4108">4108</a>
<a href="#4109" id="4109">4109</a>
<a href="#4110" id="4110">4110</a>
<a href="#4111" id="4111">4111</a>
<a href="#4112" id="4112">4112</a>
<a href="#4113" id="4113">4113</a>
<a href="#4114" id="4114">4114</a>
<a href="#4115" id="4115">4115</a>
<a href="#4116" id="4116">4116</a>
<a href="#4117" id="4117">4117</a>
<a href="#4118" id="4118">4118</a>
<a href="#4119" id="4119">4119</a>
<a href="#4120" id="4120">4120</a>
<a href="#4121" id="4121">4121</a>
<a href="#4122" id="4122">4122</a>
<a href="#4123" id="4123">4123</a>
<a href="#4124" id="4124">4124</a>
<a href="#4125" id="4125">4125</a>
<a href="#4126" id="4126">4126</a>
<a href="#4127" id="4127">4127</a>
<a href="#4128" id="4128">4128</a>
<a href="#4129" id="4129">4129</a>
<a href="#4130" id="4130">4130</a>
<a href="#4131" id="4131">4131</a>
<a href="#4132" id="4132">4132</a>
<a href="#4133" id="4133">4133</a>
<a href="#4134" id="4134">4134</a>
<a href="#4135" id="4135">4135</a>
<a href="#4136" id="4136">4136</a>
<a href="#4137" id="4137">4137</a>
<a href="#4138" id="4138">4138</a>
<a href="#4139" id="4139">4139</a>
<a href="#4140" id="4140">4140</a>
<a href="#4141" id="4141">4141</a>
<a href="#4142" id="4142">4142</a>
<a href="#4143" id="4143">4143</a>
<a href="#4144" id="4144">4144</a>
<a href="#4145" id="4145">4145</a>
<a href="#4146" id="4146">4146</a>
<a href="#4147" id="4147">4147</a>
<a href="#4148" id="4148">4148</a>
<a href="#4149" id="4149">4149</a>
<a href="#4150" id="4150">4150</a>
<a href="#4151" id="4151">4151</a>
<a href="#4152" id="4152">4152</a>
<a href="#4153" id="4153">4153</a>
<a href="#4154" id="4154">4154</a>
<a href="#4155" id="4155">4155</a>
<a href="#4156" id="4156">4156</a>
<a href="#4157" id="4157">4157</a>
<a href="#4158" id="4158">4158</a>
<a href="#4159" id="4159">4159</a>
<a href="#4160" id="4160">4160</a>
<a href="#4161" id="4161">4161</a>
<a href="#4162" id="4162">4162</a>
<a href="#4163" id="4163">4163</a>
<a href="#4164" id="4164">4164</a>
<a href="#4165" id="4165">4165</a>
<a href="#4166" id="4166">4166</a>
<a href="#4167" id="4167">4167</a>
<a href="#4168" id="4168">4168</a>
<a href="#4169" id="4169">4169</a>
<a href="#4170" id="4170">4170</a>
<a href="#4171" id="4171">4171</a>
<a href="#4172" id="4172">4172</a>
<a href="#4173" id="4173">4173</a>
<a href="#4174" id="4174">4174</a>
<a href="#4175" id="4175">4175</a>
<a href="#4176" id="4176">4176</a>
<a href="#4177" id="4177">4177</a>
<a href="#4178" id="4178">4178</a>
<a href="#4179" id="4179">4179</a>
<a href="#4180" id="4180">4180</a>
<a href="#4181" id="4181">4181</a>
<a href="#4182" id="4182">4182</a>
<a href="#4183" id="4183">4183</a>
<a href="#4184" id="4184">4184</a>
<a href="#4185" id="4185">4185</a>
<a href="#4186" id="4186">4186</a>
<a href="#4187" id="4187">4187</a>
<a href="#4188" id="4188">4188</a>
<a href="#4189" id="4189">4189</a>
<a href="#4190" id="4190">4190</a>
<a href="#4191" id="4191">4191</a>
<a href="#4192" id="4192">4192</a>
<a href="#4193" id="4193">4193</a>
<a href="#4194" id="4194">4194</a>
<a href="#4195" id="4195">4195</a>
<a href="#4196" id="4196">4196</a>
<a href="#4197" id="4197">4197</a>
<a href="#4198" id="4198">4198</a>
<a href="#4199" id="4199">4199</a>
<a href="#4200" id="4200">4200</a>
<a href="#4201" id="4201">4201</a>
<a href="#4202" id="4202">4202</a>
<a href="#4203" id="4203">4203</a>
<a href="#4204" id="4204">4204</a>
<a href="#4205" id="4205">4205</a>
<a href="#4206" id="4206">4206</a>
<a href="#4207" id="4207">4207</a>
<a href="#4208" id="4208">4208</a>
<a href="#4209" id="4209">4209</a>
<a href="#4210" id="4210">4210</a>
<a href="#4211" id="4211">4211</a>
<a href="#4212" id="4212">4212</a>
<a href="#4213" id="4213">4213</a>
<a href="#4214" id="4214">4214</a>
<a href="#4215" id="4215">4215</a>
<a href="#4216" id="4216">4216</a>
<a href="#4217" id="4217">4217</a>
<a href="#4218" id="4218">4218</a>
<a href="#4219" id="4219">4219</a>
<a href="#4220" id="4220">4220</a>
<a href="#4221" id="4221">4221</a>
<a href="#4222" id="4222">4222</a>
<a href="#4223" id="4223">4223</a>
<a href="#4224" id="4224">4224</a>
<a href="#4225" id="4225">4225</a>
<a href="#4226" id="4226">4226</a>
<a href="#4227" id="4227">4227</a>
<a href="#4228" id="4228">4228</a>
<a href="#4229" id="4229">4229</a>
<a href="#4230" id="4230">4230</a>
<a href="#4231" id="4231">4231</a>
<a href="#4232" id="4232">4232</a>
<a href="#4233" id="4233">4233</a>
<a href="#4234" id="4234">4234</a>
<a href="#4235" id="4235">4235</a>
<a href="#4236" id="4236">4236</a>
<a href="#4237" id="4237">4237</a>
<a href="#4238" id="4238">4238</a>
<a href="#4239" id="4239">4239</a>
<a href="#4240" id="4240">4240</a>
<a href="#4241" id="4241">4241</a>
<a href="#4242" id="4242">4242</a>
<a href="#4243" id="4243">4243</a>
<a href="#4244" id="4244">4244</a>
<a href="#4245" id="4245">4245</a>
<a href="#4246" id="4246">4246</a>
<a href="#4247" id="4247">4247</a>
<a href="#4248" id="4248">4248</a>
<a href="#4249" id="4249">4249</a>
<a href="#4250" id="4250">4250</a>
<a href="#4251" id="4251">4251</a>
<a href="#4252" id="4252">4252</a>
<a href="#4253" id="4253">4253</a>
<a href="#4254" id="4254">4254</a>
<a href="#4255" id="4255">4255</a>
<a href="#4256" id="4256">4256</a>
<a href="#4257" id="4257">4257</a>
<a href="#4258" id="4258">4258</a>
<a href="#4259" id="4259">4259</a>
<a href="#4260" id="4260">4260</a>
<a href="#4261" id="4261">4261</a>
<a href="#4262" id="4262">4262</a>
<a href="#4263" id="4263">4263</a>
<a href="#4264" id="4264">4264</a>
<a href="#4265" id="4265">4265</a>
<a href="#4266" id="4266">4266</a>
<a href="#4267" id="4267">4267</a>
<a href="#4268" id="4268">4268</a>
<a href="#4269" id="4269">4269</a>
<a href="#4270" id="4270">4270</a>
<a href="#4271" id="4271">4271</a>
<a href="#4272" id="4272">4272</a>
<a href="#4273" id="4273">4273</a>
<a href="#4274" id="4274">4274</a>
<a href="#4275" id="4275">4275</a>
<a href="#4276" id="4276">4276</a>
<a href="#4277" id="4277">4277</a>
<a href="#4278" id="4278">4278</a>
<a href="#4279" id="4279">4279</a>
<a href="#4280" id="4280">4280</a>
<a href="#4281" id="4281">4281</a>
<a href="#4282" id="4282">4282</a>
<a href="#4283" id="4283">4283</a>
<a href="#4284" id="4284">4284</a>
<a href="#4285" id="4285">4285</a>
<a href="#4286" id="4286">4286</a>
<a href="#4287" id="4287">4287</a>
<a href="#4288" id="4288">4288</a>
<a href="#4289" id="4289">4289</a>
<a href="#4290" id="4290">4290</a>
<a href="#4291" id="4291">4291</a>
<a href="#4292" id="4292">4292</a>
<a href="#4293" id="4293">4293</a>
<a href="#4294" id="4294">4294</a>
<a href="#4295" id="4295">4295</a>
<a href="#4296" id="4296">4296</a>
<a href="#4297" id="4297">4297</a>
<a href="#4298" id="4298">4298</a>
<a href="#4299" id="4299">4299</a>
<a href="#4300" id="4300">4300</a>
<a href="#4301" id="4301">4301</a>
<a href="#4302" id="4302">4302</a>
<a href="#4303" id="4303">4303</a>
<a href="#4304" id="4304">4304</a>
<a href="#4305" id="4305">4305</a>
<a href="#4306" id="4306">4306</a>
<a href="#4307" id="4307">4307</a>
<a href="#4308" id="4308">4308</a>
<a href="#4309" id="4309">4309</a>
<a href="#4310" id="4310">4310</a>
<a href="#4311" id="4311">4311</a>
<a href="#4312" id="4312">4312</a>
<a href="#4313" id="4313">4313</a>
<a href="#4314" id="4314">4314</a>
<a href="#4315" id="4315">4315</a>
<a href="#4316" id="4316">4316</a>
<a href="#4317" id="4317">4317</a>
<a href="#4318" id="4318">4318</a>
<a href="#4319" id="4319">4319</a>
<a href="#4320" id="4320">4320</a>
<a href="#4321" id="4321">4321</a>
<a href="#4322" id="4322">4322</a>
<a href="#4323" id="4323">4323</a>
<a href="#4324" id="4324">4324</a>
<a href="#4325" id="4325">4325</a>
<a href="#4326" id="4326">4326</a>
<a href="#4327" id="4327">4327</a>
<a href="#4328" id="4328">4328</a>
<a href="#4329" id="4329">4329</a>
<a href="#4330" id="4330">4330</a>
<a href="#4331" id="4331">4331</a>
<a href="#4332" id="4332">4332</a>
<a href="#4333" id="4333">4333</a>
<a href="#4334" id="4334">4334</a>
<a href="#4335" id="4335">4335</a>
<a href="#4336" id="4336">4336</a>
<a href="#4337" id="4337">4337</a>
<a href="#4338" id="4338">4338</a>
<a href="#4339" id="4339">4339</a>
<a href="#4340" id="4340">4340</a>
<a href="#4341" id="4341">4341</a>
<a href="#4342" id="4342">4342</a>
<a href="#4343" id="4343">4343</a>
<a href="#4344" id="4344">4344</a>
<a href="#4345" id="4345">4345</a>
<a href="#4346" id="4346">4346</a>
<a href="#4347" id="4347">4347</a>
<a href="#4348" id="4348">4348</a>
<a href="#4349" id="4349">4349</a>
<a href="#4350" id="4350">4350</a>
<a href="#4351" id="4351">4351</a>
<a href="#4352" id="4352">4352</a>
<a href="#4353" id="4353">4353</a>
<a href="#4354" id="4354">4354</a>
<a href="#4355" id="4355">4355</a>
<a href="#4356" id="4356">4356</a>
<a href="#4357" id="4357">4357</a>
<a href="#4358" id="4358">4358</a>
<a href="#4359" id="4359">4359</a>
<a href="#4360" id="4360">4360</a>
<a href="#4361" id="4361">4361</a>
<a href="#4362" id="4362">4362</a>
<a href="#4363" id="4363">4363</a>
<a href="#4364" id="4364">4364</a>
<a href="#4365" id="4365">4365</a>
<a href="#4366" id="4366">4366</a>
<a href="#4367" id="4367">4367</a>
<a href="#4368" id="4368">4368</a>
<a href="#4369" id="4369">4369</a>
<a href="#4370" id="4370">4370</a>
<a href="#4371" id="4371">4371</a>
<a href="#4372" id="4372">4372</a>
<a href="#4373" id="4373">4373</a>
<a href="#4374" id="4374">4374</a>
<a href="#4375" id="4375">4375</a>
<a href="#4376" id="4376">4376</a>
<a href="#4377" id="4377">4377</a>
<a href="#4378" id="4378">4378</a>
<a href="#4379" id="4379">4379</a>
<a href="#4380" id="4380">4380</a>
<a href="#4381" id="4381">4381</a>
<a href="#4382" id="4382">4382</a>
<a href="#4383" id="4383">4383</a>
<a href="#4384" id="4384">4384</a>
<a href="#4385" id="4385">4385</a>
<a href="#4386" id="4386">4386</a>
<a href="#4387" id="4387">4387</a>
<a href="#4388" id="4388">4388</a>
<a href="#4389" id="4389">4389</a>
<a href="#4390" id="4390">4390</a>
<a href="#4391" id="4391">4391</a>
<a href="#4392" id="4392">4392</a>
<a href="#4393" id="4393">4393</a>
<a href="#4394" id="4394">4394</a>
<a href="#4395" id="4395">4395</a>
<a href="#4396" id="4396">4396</a>
<a href="#4397" id="4397">4397</a>
<a href="#4398" id="4398">4398</a>
<a href="#4399" id="4399">4399</a>
<a href="#4400" id="4400">4400</a>
<a href="#4401" id="4401">4401</a>
<a href="#4402" id="4402">4402</a>
<a href="#4403" id="4403">4403</a>
<a href="#4404" id="4404">4404</a>
<a href="#4405" id="4405">4405</a>
<a href="#4406" id="4406">4406</a>
<a href="#4407" id="4407">4407</a>
<a href="#4408" id="4408">4408</a>
<a href="#4409" id="4409">4409</a>
<a href="#4410" id="4410">4410</a>
<a href="#4411" id="4411">4411</a>
<a href="#4412" id="4412">4412</a>
<a href="#4413" id="4413">4413</a>
<a href="#4414" id="4414">4414</a>
<a href="#4415" id="4415">4415</a>
<a href="#4416" id="4416">4416</a>
<a href="#4417" id="4417">4417</a>
<a href="#4418" id="4418">4418</a>
<a href="#4419" id="4419">4419</a>
<a href="#4420" id="4420">4420</a>
<a href="#4421" id="4421">4421</a>
<a href="#4422" id="4422">4422</a>
<a href="#4423" id="4423">4423</a>
<a href="#4424" id="4424">4424</a>
<a href="#4425" id="4425">4425</a>
<a href="#4426" id="4426">4426</a>
<a href="#4427" id="4427">4427</a>
<a href="#4428" id="4428">4428</a>
<a href="#4429" id="4429">4429</a>
<a href="#4430" id="4430">4430</a>
<a href="#4431" id="4431">4431</a>
<a href="#4432" id="4432">4432</a>
<a href="#4433" id="4433">4433</a>
<a href="#4434" id="4434">4434</a>
<a href="#4435" id="4435">4435</a>
<a href="#4436" id="4436">4436</a>
<a href="#4437" id="4437">4437</a>
<a href="#4438" id="4438">4438</a>
<a href="#4439" id="4439">4439</a>
<a href="#4440" id="4440">4440</a>
<a href="#4441" id="4441">4441</a>
<a href="#4442" id="4442">4442</a>
<a href="#4443" id="4443">4443</a>
<a href="#4444" id="4444">4444</a>
<a href="#4445" id="4445">4445</a>
<a href="#4446" id="4446">4446</a>
<a href="#4447" id="4447">4447</a>
<a href="#4448" id="4448">4448</a>
<a href="#4449" id="4449">4449</a>
<a href="#4450" id="4450">4450</a>
<a href="#4451" id="4451">4451</a>
<a href="#4452" id="4452">4452</a>
<a href="#4453" id="4453">4453</a>
<a href="#4454" id="4454">4454</a>
<a href="#4455" id="4455">4455</a>
<a href="#4456" id="4456">4456</a>
<a href="#4457" id="4457">4457</a>
<a href="#4458" id="4458">4458</a>
<a href="#4459" id="4459">4459</a>
<a href="#4460" id="4460">4460</a>
<a href="#4461" id="4461">4461</a>
<a href="#4462" id="4462">4462</a>
<a href="#4463" id="4463">4463</a>
<a href="#4464" id="4464">4464</a>
<a href="#4465" id="4465">4465</a>
<a href="#4466" id="4466">4466</a>
<a href="#4467" id="4467">4467</a>
<a href="#4468" id="4468">4468</a>
<a href="#4469" id="4469">4469</a>
<a href="#4470" id="4470">4470</a>
<a href="#4471" id="4471">4471</a>
<a href="#4472" id="4472">4472</a>
<a href="#4473" id="4473">4473</a>
<a href="#4474" id="4474">4474</a>
<a href="#4475" id="4475">4475</a>
<a href="#4476" id="4476">4476</a>
<a href="#4477" id="4477">4477</a>
<a href="#4478" id="4478">4478</a>
<a href="#4479" id="4479">4479</a>
<a href="#4480" id="4480">4480</a>
<a href="#4481" id="4481">4481</a>
<a href="#4482" id="4482">4482</a>
<a href="#4483" id="4483">4483</a>
<a href="#4484" id="4484">4484</a>
<a href="#4485" id="4485">4485</a>
<a href="#4486" id="4486">4486</a>
<a href="#4487" id="4487">4487</a>
<a href="#4488" id="4488">4488</a>
<a href="#4489" id="4489">4489</a>
<a href="#4490" id="4490">4490</a>
<a href="#4491" id="4491">4491</a>
<a href="#4492" id="4492">4492</a>
<a href="#4493" id="4493">4493</a>
<a href="#4494" id="4494">4494</a>
<a href="#4495" id="4495">4495</a>
<a href="#4496" id="4496">4496</a>
<a href="#4497" id="4497">4497</a>
<a href="#4498" id="4498">4498</a>
<a href="#4499" id="4499">4499</a>
<a href="#4500" id="4500">4500</a>
<a href="#4501" id="4501">4501</a>
<a href="#4502" id="4502">4502</a>
<a href="#4503" id="4503">4503</a>
<a href="#4504" id="4504">4504</a>
<a href="#4505" id="4505">4505</a>
<a href="#4506" id="4506">4506</a>
<a href="#4507" id="4507">4507</a>
<a href="#4508" id="4508">4508</a>
<a href="#4509" id="4509">4509</a>
<a href="#4510" id="4510">4510</a>
<a href="#4511" id="4511">4511</a>
<a href="#4512" id="4512">4512</a>
<a href="#4513" id="4513">4513</a>
<a href="#4514" id="4514">4514</a>
<a href="#4515" id="4515">4515</a>
<a href="#4516" id="4516">4516</a>
<a href="#4517" id="4517">4517</a>
<a href="#4518" id="4518">4518</a>
<a href="#4519" id="4519">4519</a>
<a href="#4520" id="4520">4520</a>
<a href="#4521" id="4521">4521</a>
<a href="#4522" id="4522">4522</a>
<a href="#4523" id="4523">4523</a>
<a href="#4524" id="4524">4524</a>
<a href="#4525" id="4525">4525</a>
<a href="#4526" id="4526">4526</a>
<a href="#4527" id="4527">4527</a>
<a href="#4528" id="4528">4528</a>
<a href="#4529" id="4529">4529</a>
<a href="#4530" id="4530">4530</a>
<a href="#4531" id="4531">4531</a>
<a href="#4532" id="4532">4532</a>
<a href="#4533" id="4533">4533</a>
<a href="#4534" id="4534">4534</a>
<a href="#4535" id="4535">4535</a>
<a href="#4536" id="4536">4536</a>
<a href="#4537" id="4537">4537</a>
<a href="#4538" id="4538">4538</a>
<a href="#4539" id="4539">4539</a>
<a href="#4540" id="4540">4540</a>
<a href="#4541" id="4541">4541</a>
<a href="#4542" id="4542">4542</a>
<a href="#4543" id="4543">4543</a>
<a href="#4544" id="4544">4544</a>
<a href="#4545" id="4545">4545</a>
<a href="#4546" id="4546">4546</a>
<a href="#4547" id="4547">4547</a>
<a href="#4548" id="4548">4548</a>
<a href="#4549" id="4549">4549</a>
<a href="#4550" id="4550">4550</a>
<a href="#4551" id="4551">4551</a>
<a href="#4552" id="4552">4552</a>
<a href="#4553" id="4553">4553</a>
<a href="#4554" id="4554">4554</a>
<a href="#4555" id="4555">4555</a>
<a href="#4556" id="4556">4556</a>
<a href="#4557" id="4557">4557</a>
<a href="#4558" id="4558">4558</a>
<a href="#4559" id="4559">4559</a>
<a href="#4560" id="4560">4560</a>
<a href="#4561" id="4561">4561</a>
<a href="#4562" id="4562">4562</a>
<a href="#4563" id="4563">4563</a>
<a href="#4564" id="4564">4564</a>
<a href="#4565" id="4565">4565</a>
<a href="#4566" id="4566">4566</a>
<a href="#4567" id="4567">4567</a>
<a href="#4568" id="4568">4568</a>
<a href="#4569" id="4569">4569</a>
<a href="#4570" id="4570">4570</a>
<a href="#4571" id="4571">4571</a>
<a href="#4572" id="4572">4572</a>
<a href="#4573" id="4573">4573</a>
<a href="#4574" id="4574">4574</a>
<a href="#4575" id="4575">4575</a>
<a href="#4576" id="4576">4576</a>
<a href="#4577" id="4577">4577</a>
<a href="#4578" id="4578">4578</a>
<a href="#4579" id="4579">4579</a>
<a href="#4580" id="4580">4580</a>
<a href="#4581" id="4581">4581</a>
<a href="#4582" id="4582">4582</a>
<a href="#4583" id="4583">4583</a>
<a href="#4584" id="4584">4584</a>
<a href="#4585" id="4585">4585</a>
<a href="#4586" id="4586">4586</a>
<a href="#4587" id="4587">4587</a>
<a href="#4588" id="4588">4588</a>
<a href="#4589" id="4589">4589</a>
<a href="#4590" id="4590">4590</a>
<a href="#4591" id="4591">4591</a>
<a href="#4592" id="4592">4592</a>
<a href="#4593" id="4593">4593</a>
<a href="#4594" id="4594">4594</a>
<a href="#4595" id="4595">4595</a>
<a href="#4596" id="4596">4596</a>
<a href="#4597" id="4597">4597</a>
<a href="#4598" id="4598">4598</a>
<a href="#4599" id="4599">4599</a>
<a href="#4600" id="4600">4600</a>
<a href="#4601" id="4601">4601</a>
<a href="#4602" id="4602">4602</a>
<a href="#4603" id="4603">4603</a>
<a href="#4604" id="4604">4604</a>
<a href="#4605" id="4605">4605</a>
<a href="#4606" id="4606">4606</a>
<a href="#4607" id="4607">4607</a>
<a href="#4608" id="4608">4608</a>
<a href="#4609" id="4609">4609</a>
<a href="#4610" id="4610">4610</a>
<a href="#4611" id="4611">4611</a>
<a href="#4612" id="4612">4612</a>
<a href="#4613" id="4613">4613</a>
<a href="#4614" id="4614">4614</a>
<a href="#4615" id="4615">4615</a>
<a href="#4616" id="4616">4616</a>
<a href="#4617" id="4617">4617</a>
<a href="#4618" id="4618">4618</a>
<a href="#4619" id="4619">4619</a>
<a href="#4620" id="4620">4620</a>
<a href="#4621" id="4621">4621</a>
<a href="#4622" id="4622">4622</a>
<a href="#4623" id="4623">4623</a>
<a href="#4624" id="4624">4624</a>
<a href="#4625" id="4625">4625</a>
<a href="#4626" id="4626">4626</a>
<a href="#4627" id="4627">4627</a>
<a href="#4628" id="4628">4628</a>
<a href="#4629" id="4629">4629</a>
<a href="#4630" id="4630">4630</a>
<a href="#4631" id="4631">4631</a>
<a href="#4632" id="4632">4632</a>
<a href="#4633" id="4633">4633</a>
<a href="#4634" id="4634">4634</a>
<a href="#4635" id="4635">4635</a>
<a href="#4636" id="4636">4636</a>
<a href="#4637" id="4637">4637</a>
<a href="#4638" id="4638">4638</a>
<a href="#4639" id="4639">4639</a>
<a href="#4640" id="4640">4640</a>
<a href="#4641" id="4641">4641</a>
<a href="#4642" id="4642">4642</a>
<a href="#4643" id="4643">4643</a>
<a href="#4644" id="4644">4644</a>
<a href="#4645" id="4645">4645</a>
<a href="#4646" id="4646">4646</a>
<a href="#4647" id="4647">4647</a>
<a href="#4648" id="4648">4648</a>
<a href="#4649" id="4649">4649</a>
<a href="#4650" id="4650">4650</a>
<a href="#4651" id="4651">4651</a>
<a href="#4652" id="4652">4652</a>
<a href="#4653" id="4653">4653</a>
<a href="#4654" id="4654">4654</a>
<a href="#4655" id="4655">4655</a>
<a href="#4656" id="4656">4656</a>
<a href="#4657" id="4657">4657</a>
<a href="#4658" id="4658">4658</a>
<a href="#4659" id="4659">4659</a>
<a href="#4660" id="4660">4660</a>
<a href="#4661" id="4661">4661</a>
<a href="#4662" id="4662">4662</a>
<a href="#4663" id="4663">4663</a>
<a href="#4664" id="4664">4664</a>
<a href="#4665" id="4665">4665</a>
<a href="#4666" id="4666">4666</a>
<a href="#4667" id="4667">4667</a>
<a href="#4668" id="4668">4668</a>
<a href="#4669" id="4669">4669</a>
<a href="#4670" id="4670">4670</a>
<a href="#4671" id="4671">4671</a>
<a href="#4672" id="4672">4672</a>
<a href="#4673" id="4673">4673</a>
<a href="#4674" id="4674">4674</a>
<a href="#4675" id="4675">4675</a>
<a href="#4676" id="4676">4676</a>
<a href="#4677" id="4677">4677</a>
<a href="#4678" id="4678">4678</a>
<a href="#4679" id="4679">4679</a>
<a href="#4680" id="4680">4680</a>
<a href="#4681" id="4681">4681</a>
<a href="#4682" id="4682">4682</a>
<a href="#4683" id="4683">4683</a>
<a href="#4684" id="4684">4684</a>
<a href="#4685" id="4685">4685</a>
<a href="#4686" id="4686">4686</a>
<a href="#4687" id="4687">4687</a>
<a href="#4688" id="4688">4688</a>
<a href="#4689" id="4689">4689</a>
<a href="#4690" id="4690">4690</a>
<a href="#4691" id="4691">4691</a>
<a href="#4692" id="4692">4692</a>
<a href="#4693" id="4693">4693</a>
<a href="#4694" id="4694">4694</a>
<a href="#4695" id="4695">4695</a>
<a href="#4696" id="4696">4696</a>
<a href="#4697" id="4697">4697</a>
<a href="#4698" id="4698">4698</a>
<a href="#4699" id="4699">4699</a>
<a href="#4700" id="4700">4700</a>
<a href="#4701" id="4701">4701</a>
<a href="#4702" id="4702">4702</a>
<a href="#4703" id="4703">4703</a>
<a href="#4704" id="4704">4704</a>
<a href="#4705" id="4705">4705</a>
<a href="#4706" id="4706">4706</a>
<a href="#4707" id="4707">4707</a>
<a href="#4708" id="4708">4708</a>
<a href="#4709" id="4709">4709</a>
<a href="#4710" id="4710">4710</a>
<a href="#4711" id="4711">4711</a>
<a href="#4712" id="4712">4712</a>
<a href="#4713" id="4713">4713</a>
<a href="#4714" id="4714">4714</a>
<a href="#4715" id="4715">4715</a>
<a href="#4716" id="4716">4716</a>
<a href="#4717" id="4717">4717</a>
<a href="#4718" id="4718">4718</a>
<a href="#4719" id="4719">4719</a>
<a href="#4720" id="4720">4720</a>
<a href="#4721" id="4721">4721</a>
<a href="#4722" id="4722">4722</a>
<a href="#4723" id="4723">4723</a>
<a href="#4724" id="4724">4724</a>
<a href="#4725" id="4725">4725</a>
<a href="#4726" id="4726">4726</a>
<a href="#4727" id="4727">4727</a>
<a href="#4728" id="4728">4728</a>
<a href="#4729" id="4729">4729</a>
<a href="#4730" id="4730">4730</a>
<a href="#4731" id="4731">4731</a>
<a href="#4732" id="4732">4732</a>
<a href="#4733" id="4733">4733</a>
<a href="#4734" id="4734">4734</a>
<a href="#4735" id="4735">4735</a>
<a href="#4736" id="4736">4736</a>
<a href="#4737" id="4737">4737</a>
<a href="#4738" id="4738">4738</a>
<a href="#4739" id="4739">4739</a>
<a href="#4740" id="4740">4740</a>
<a href="#4741" id="4741">4741</a>
<a href="#4742" id="4742">4742</a>
<a href="#4743" id="4743">4743</a>
<a href="#4744" id="4744">4744</a>
<a href="#4745" id="4745">4745</a>
<a href="#4746" id="4746">4746</a>
<a href="#4747" id="4747">4747</a>
<a href="#4748" id="4748">4748</a>
<a href="#4749" id="4749">4749</a>
<a href="#4750" id="4750">4750</a>
<a href="#4751" id="4751">4751</a>
<a href="#4752" id="4752">4752</a>
<a href="#4753" id="4753">4753</a>
<a href="#4754" id="4754">4754</a>
<a href="#4755" id="4755">4755</a>
<a href="#4756" id="4756">4756</a>
<a href="#4757" id="4757">4757</a>
<a href="#4758" id="4758">4758</a>
<a href="#4759" id="4759">4759</a>
<a href="#4760" id="4760">4760</a>
<a href="#4761" id="4761">4761</a>
<a href="#4762" id="4762">4762</a>
<a href="#4763" id="4763">4763</a>
<a href="#4764" id="4764">4764</a>
<a href="#4765" id="4765">4765</a>
<a href="#4766" id="4766">4766</a>
<a href="#4767" id="4767">4767</a>
<a href="#4768" id="4768">4768</a>
<a href="#4769" id="4769">4769</a>
<a href="#4770" id="4770">4770</a>
<a href="#4771" id="4771">4771</a>
<a href="#4772" id="4772">4772</a>
<a href="#4773" id="4773">4773</a>
<a href="#4774" id="4774">4774</a>
<a href="#4775" id="4775">4775</a>
<a href="#4776" id="4776">4776</a>
<a href="#4777" id="4777">4777</a>
<a href="#4778" id="4778">4778</a>
<a href="#4779" id="4779">4779</a>
<a href="#4780" id="4780">4780</a>
<a href="#4781" id="4781">4781</a>
<a href="#4782" id="4782">4782</a>
<a href="#4783" id="4783">4783</a>
<a href="#4784" id="4784">4784</a>
<a href="#4785" id="4785">4785</a>
<a href="#4786" id="4786">4786</a>
<a href="#4787" id="4787">4787</a>
<a href="#4788" id="4788">4788</a>
<a href="#4789" id="4789">4789</a>
<a href="#4790" id="4790">4790</a>
<a href="#4791" id="4791">4791</a>
<a href="#4792" id="4792">4792</a>
<a href="#4793" id="4793">4793</a>
<a href="#4794" id="4794">4794</a>
<a href="#4795" id="4795">4795</a>
<a href="#4796" id="4796">4796</a>
<a href="#4797" id="4797">4797</a>
<a href="#4798" id="4798">4798</a>
<a href="#4799" id="4799">4799</a>
<a href="#4800" id="4800">4800</a>
<a href="#4801" id="4801">4801</a>
<a href="#4802" id="4802">4802</a>
<a href="#4803" id="4803">4803</a>
<a href="#4804" id="4804">4804</a>
<a href="#4805" id="4805">4805</a>
<a href="#4806" id="4806">4806</a>
<a href="#4807" id="4807">4807</a>
<a href="#4808" id="4808">4808</a>
<a href="#4809" id="4809">4809</a>
<a href="#4810" id="4810">4810</a>
<a href="#4811" id="4811">4811</a>
<a href="#4812" id="4812">4812</a>
<a href="#4813" id="4813">4813</a>
<a href="#4814" id="4814">4814</a>
<a href="#4815" id="4815">4815</a>
<a href="#4816" id="4816">4816</a>
<a href="#4817" id="4817">4817</a>
<a href="#4818" id="4818">4818</a>
<a href="#4819" id="4819">4819</a>
<a href="#4820" id="4820">4820</a>
<a href="#4821" id="4821">4821</a>
<a href="#4822" id="4822">4822</a>
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">crate</span>::alloc::alloc::{handle_alloc_error, Layout};
<span class="kw">use </span><span class="kw">crate</span>::scopeguard::{guard, ScopeGuard};
<span class="kw">use </span><span class="kw">crate</span>::TryReserveError;
<span class="kw">use </span>core::iter::FusedIterator;
<span class="kw">use </span>core::marker::PhantomData;
<span class="kw">use </span>core::mem;
<span class="kw">use </span>core::mem::MaybeUninit;
<span class="kw">use </span>core::ptr::NonNull;
<span class="kw">use </span>core::{hint, ptr};

<span class="macro">cfg_if!</span> {
    <span class="comment">// Use the SSE2 implementation if possible: it allows us to scan 16 buckets
    // at once instead of 8. We don't bother with AVX since it would require
    // runtime dispatch and wouldn't gain us much anyways: the probability of
    // finding a match drops off drastically after the first few buckets.
    //
    // I attempted an implementation on ARM using NEON instructions, but it
    // turns out that most NEON instructions have multi-cycle latency, which in
    // the end outweighs any gains over the generic implementation.
    </span><span class="kw">if </span><span class="attr">#[cfg(all(
        target_feature = <span class="string">"sse2"</span>,
        any(target_arch = <span class="string">"x86"</span>, target_arch = <span class="string">"x86_64"</span>),
        not(miri),
    ))] </span>{
        <span class="kw">mod </span>sse2;
        <span class="kw">use </span>sse2 <span class="kw">as </span>imp;
    } <span class="kw">else if </span><span class="attr">#[cfg(all(
        target_arch = <span class="string">"aarch64"</span>,
        target_feature = <span class="string">"neon"</span>,
        <span class="comment">// NEON intrinsics are currently broken on big-endian targets.
        // See https://github.com/rust-lang/stdarch/issues/1484.
        </span>target_endian = <span class="string">"little"</span>,
        not(miri),
    ))] </span>{
        <span class="kw">mod </span>neon;
        <span class="kw">use </span>neon <span class="kw">as </span>imp;
    } <span class="kw">else </span>{
        <span class="kw">mod </span>generic;
        <span class="kw">use </span>generic <span class="kw">as </span>imp;
    }
}

<span class="kw">mod </span>alloc;
<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">use </span><span class="self">self</span>::alloc::{do_alloc, Allocator, Global};

<span class="kw">mod </span>bitmask;

<span class="kw">use </span><span class="self">self</span>::bitmask::BitMaskIter;
<span class="kw">use </span><span class="self">self</span>::imp::Group;

<span class="comment">// Branch prediction hint. This is currently only available on nightly but it
// consistently improves performance by 10-15%.
</span><span class="attr">#[cfg(not(feature = <span class="string">"nightly"</span>))]
</span><span class="kw">use </span>core::convert::identity <span class="kw">as </span>likely;
<span class="attr">#[cfg(not(feature = <span class="string">"nightly"</span>))]
</span><span class="kw">use </span>core::convert::identity <span class="kw">as </span>unlikely;
<span class="attr">#[cfg(feature = <span class="string">"nightly"</span>)]
</span><span class="kw">use </span>core::intrinsics::{likely, unlikely};

<span class="comment">// FIXME: use strict provenance functions once they are stable.
// Implement it with a transmute for now.
</span><span class="attr">#[inline(always)]
#[allow(clippy::useless_transmute)] </span><span class="comment">// clippy is wrong, cast and transmute are different here
</span><span class="kw">fn </span>invalid_mut&lt;T&gt;(addr: usize) -&gt; <span class="kw-2">*mut </span>T {
    <span class="kw">unsafe </span>{ core::mem::transmute(addr) }
}

<span class="attr">#[inline]
</span><span class="kw">unsafe fn </span>offset_from&lt;T&gt;(to: <span class="kw-2">*const </span>T, from: <span class="kw-2">*const </span>T) -&gt; usize {
    to.offset_from(from) <span class="kw">as </span>usize
}

<span class="doccomment">/// Whether memory allocation errors should return an error or abort.
</span><span class="attr">#[derive(Copy, Clone)]
</span><span class="kw">enum </span>Fallibility {
    Fallible,
    Infallible,
}

<span class="kw">impl </span>Fallibility {
    <span class="doccomment">/// Error to return on capacity overflow.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>capacity_overflow(<span class="self">self</span>) -&gt; TryReserveError {
        <span class="kw">match </span><span class="self">self </span>{
            Fallibility::Fallible =&gt; TryReserveError::CapacityOverflow,
            Fallibility::Infallible =&gt; <span class="macro">panic!</span>(<span class="string">"Hash table capacity overflow"</span>),
        }
    }

    <span class="doccomment">/// Error to return on allocation error.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>alloc_err(<span class="self">self</span>, layout: Layout) -&gt; TryReserveError {
        <span class="kw">match </span><span class="self">self </span>{
            Fallibility::Fallible =&gt; TryReserveError::AllocError { layout },
            Fallibility::Infallible =&gt; handle_alloc_error(layout),
        }
    }
}

<span class="kw">trait </span>SizedTypeProperties: Sized {
    <span class="kw">const </span>IS_ZERO_SIZED: bool = mem::size_of::&lt;<span class="self">Self</span>&gt;() == <span class="number">0</span>;
    <span class="kw">const </span>NEEDS_DROP: bool = mem::needs_drop::&lt;<span class="self">Self</span>&gt;();
}

<span class="kw">impl</span>&lt;T&gt; SizedTypeProperties <span class="kw">for </span>T {}

<span class="doccomment">/// Control byte value for an empty bucket.
</span><span class="kw">const </span>EMPTY: u8 = <span class="number">0b1111_1111</span>;

<span class="doccomment">/// Control byte value for a deleted bucket.
</span><span class="kw">const </span>DELETED: u8 = <span class="number">0b1000_0000</span>;

<span class="doccomment">/// Checks whether a control byte represents a full bucket (top bit is clear).
</span><span class="attr">#[inline]
</span><span class="kw">fn </span>is_full(ctrl: u8) -&gt; bool {
    ctrl &amp; <span class="number">0x80 </span>== <span class="number">0
</span>}

<span class="doccomment">/// Checks whether a control byte represents a special value (top bit is set).
</span><span class="attr">#[inline]
</span><span class="kw">fn </span>is_special(ctrl: u8) -&gt; bool {
    ctrl &amp; <span class="number">0x80 </span>!= <span class="number">0
</span>}

<span class="doccomment">/// Checks whether a special control value is EMPTY (just check 1 bit).
</span><span class="attr">#[inline]
</span><span class="kw">fn </span>special_is_empty(ctrl: u8) -&gt; bool {
    <span class="macro">debug_assert!</span>(is_special(ctrl));
    ctrl &amp; <span class="number">0x01 </span>!= <span class="number">0
</span>}

<span class="doccomment">/// Primary hash function, used to select the initial bucket to probe from.
</span><span class="attr">#[inline]
#[allow(clippy::cast_possible_truncation)]
</span><span class="kw">fn </span>h1(hash: u64) -&gt; usize {
    <span class="comment">// On 32-bit platforms we simply ignore the higher hash bits.
    </span>hash <span class="kw">as </span>usize
}

<span class="comment">// Constant for h2 function that grabing the top 7 bits of the hash.
</span><span class="kw">const </span>MIN_HASH_LEN: usize = <span class="kw">if </span>mem::size_of::&lt;usize&gt;() &lt; mem::size_of::&lt;u64&gt;() {
    mem::size_of::&lt;usize&gt;()
} <span class="kw">else </span>{
    mem::size_of::&lt;u64&gt;()
};

<span class="doccomment">/// Secondary hash function, saved in the low 7 bits of the control byte.
</span><span class="attr">#[inline]
#[allow(clippy::cast_possible_truncation)]
</span><span class="kw">fn </span>h2(hash: u64) -&gt; u8 {
    <span class="comment">// Grab the top 7 bits of the hash. While the hash is normally a full 64-bit
    // value, some hash functions (such as FxHash) produce a usize result
    // instead, which means that the top 32 bits are 0 on 32-bit platforms.
    // So we use MIN_HASH_LEN constant to handle this.
    </span><span class="kw">let </span>top7 = hash &gt;&gt; (MIN_HASH_LEN * <span class="number">8 </span>- <span class="number">7</span>);
    (top7 &amp; <span class="number">0x7f</span>) <span class="kw">as </span>u8 <span class="comment">// truncation
</span>}

<span class="doccomment">/// Probe sequence based on triangular numbers, which is guaranteed (since our
/// table size is a power of two) to visit every group of elements exactly once.
///
/// A triangular probe has us jump by 1 more group every time. So first we
/// jump by 1 group (meaning we just continue our linear scan), then 2 groups
/// (skipping over 1 group), then 3 groups (skipping over 2 groups), and so on.
///
/// Proof that the probe will visit every group in the table:
/// &lt;https://fgiesen.wordpress.com/2015/02/22/triangular-numbers-mod-2n/&gt;
</span><span class="kw">struct </span>ProbeSeq {
    pos: usize,
    stride: usize,
}

<span class="kw">impl </span>ProbeSeq {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>move_next(<span class="kw-2">&amp;mut </span><span class="self">self</span>, bucket_mask: usize) {
        <span class="comment">// We should have found an empty bucket by now and ended the probe.
        </span><span class="macro">debug_assert!</span>(
            <span class="self">self</span>.stride &lt;= bucket_mask,
            <span class="string">"Went past end of probe sequence"
        </span>);

        <span class="self">self</span>.stride += Group::WIDTH;
        <span class="self">self</span>.pos += <span class="self">self</span>.stride;
        <span class="self">self</span>.pos &amp;= bucket_mask;
    }
}

<span class="doccomment">/// Returns the number of buckets needed to hold the given number of items,
/// taking the maximum load factor into account.
///
/// Returns `None` if an overflow occurs.
</span><span class="comment">// Workaround for emscripten bug emscripten-core/emscripten-fastcomp#258
</span><span class="attr">#[cfg_attr(target_os = <span class="string">"emscripten"</span>, inline(never))]
#[cfg_attr(not(target_os = <span class="string">"emscripten"</span>), inline)]
</span><span class="kw">fn </span>capacity_to_buckets(cap: usize) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
    <span class="macro">debug_assert_ne!</span>(cap, <span class="number">0</span>);

    <span class="comment">// For small tables we require at least 1 empty bucket so that lookups are
    // guaranteed to terminate if an element doesn't exist in the table.
    </span><span class="kw">if </span>cap &lt; <span class="number">8 </span>{
        <span class="comment">// We don't bother with a table size of 2 buckets since that can only
        // hold a single element. Instead we skip directly to a 4 bucket table
        // which can hold 3 elements.
        </span><span class="kw">return </span><span class="prelude-val">Some</span>(<span class="kw">if </span>cap &lt; <span class="number">4 </span>{ <span class="number">4 </span>} <span class="kw">else </span>{ <span class="number">8 </span>});
    }

    <span class="comment">// Otherwise require 1/8 buckets to be empty (87.5% load)
    //
    // Be careful when modifying this, calculate_layout relies on the
    // overflow check here.
    </span><span class="kw">let </span>adjusted_cap = cap.checked_mul(<span class="number">8</span>)<span class="question-mark">? </span>/ <span class="number">7</span>;

    <span class="comment">// Any overflows will have been caught by the checked_mul. Also, any
    // rounding errors from the division above will be cleaned up by
    // next_power_of_two (which can't overflow because of the previous division).
    </span><span class="prelude-val">Some</span>(adjusted_cap.next_power_of_two())
}

<span class="doccomment">/// Returns the maximum effective capacity for the given bucket mask, taking
/// the maximum load factor into account.
</span><span class="attr">#[inline]
</span><span class="kw">fn </span>bucket_mask_to_capacity(bucket_mask: usize) -&gt; usize {
    <span class="kw">if </span>bucket_mask &lt; <span class="number">8 </span>{
        <span class="comment">// For tables with 1/2/4/8 buckets, we always reserve one empty slot.
        // Keep in mind that the bucket mask is one less than the bucket count.
        </span>bucket_mask
    } <span class="kw">else </span>{
        <span class="comment">// For larger tables we reserve 12.5% of the slots as empty.
        </span>((bucket_mask + <span class="number">1</span>) / <span class="number">8</span>) * <span class="number">7
    </span>}
}

<span class="doccomment">/// Helper which allows the max calculation for ctrl_align to be statically computed for each T
/// while keeping the rest of `calculate_layout_for` independent of `T`
</span><span class="attr">#[derive(Copy, Clone)]
</span><span class="kw">struct </span>TableLayout {
    size: usize,
    ctrl_align: usize,
}

<span class="kw">impl </span>TableLayout {
    <span class="attr">#[inline]
    </span><span class="kw">const fn </span>new&lt;T&gt;() -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>layout = Layout::new::&lt;T&gt;();
        <span class="self">Self </span>{
            size: layout.size(),
            ctrl_align: <span class="kw">if </span>layout.align() &gt; Group::WIDTH {
                layout.align()
            } <span class="kw">else </span>{
                Group::WIDTH
            },
        }
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>calculate_layout_for(<span class="self">self</span>, buckets: usize) -&gt; <span class="prelude-ty">Option</span>&lt;(Layout, usize)&gt; {
        <span class="macro">debug_assert!</span>(buckets.is_power_of_two());

        <span class="kw">let </span>TableLayout { size, ctrl_align } = <span class="self">self</span>;
        <span class="comment">// Manual layout calculation since Layout methods are not yet stable.
        </span><span class="kw">let </span>ctrl_offset =
            size.checked_mul(buckets)<span class="question-mark">?</span>.checked_add(ctrl_align - <span class="number">1</span>)<span class="question-mark">? </span>&amp; !(ctrl_align - <span class="number">1</span>);
        <span class="kw">let </span>len = ctrl_offset.checked_add(buckets + Group::WIDTH)<span class="question-mark">?</span>;

        <span class="comment">// We need an additional check to ensure that the allocation doesn't
        // exceed `isize::MAX` (https://github.com/rust-lang/rust/pull/95295).
        </span><span class="kw">if </span>len &gt; isize::MAX <span class="kw">as </span>usize - (ctrl_align - <span class="number">1</span>) {
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }

        <span class="prelude-val">Some</span>((
            <span class="kw">unsafe </span>{ Layout::from_size_align_unchecked(len, ctrl_align) },
            ctrl_offset,
        ))
    }
}

<span class="doccomment">/// A reference to an empty bucket into which an can be inserted.
</span><span class="kw">pub struct </span>InsertSlot {
    index: usize,
}

<span class="doccomment">/// A reference to a hash table bucket containing a `T`.
///
/// This is usually just a pointer to the element itself. However if the element
/// is a ZST, then we instead track the index of the element in the table so
/// that `erase` works properly.
</span><span class="kw">pub struct </span>Bucket&lt;T&gt; {
    <span class="comment">// Actually it is pointer to next element than element itself
    // this is needed to maintain pointer arithmetic invariants
    // keeping direct pointer to element introduces difficulty.
    // Using `NonNull` for variance and niche layout
    </span>ptr: NonNull&lt;T&gt;,
}

<span class="comment">// This Send impl is needed for rayon support. This is safe since Bucket is
// never exposed in a public API.
</span><span class="kw">unsafe impl</span>&lt;T&gt; Send <span class="kw">for </span>Bucket&lt;T&gt; {}

<span class="kw">impl</span>&lt;T&gt; Clone <span class="kw">for </span>Bucket&lt;T&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>clone(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{ ptr: <span class="self">self</span>.ptr }
    }
}

<span class="kw">impl</span>&lt;T&gt; Bucket&lt;T&gt; {
    <span class="doccomment">/// Creates a [`Bucket`] that contain pointer to the data.
    /// The pointer calculation is performed by calculating the
    /// offset from given `base` pointer (convenience for
    /// `base.as_ptr().sub(index)`).
    ///
    /// `index` is in units of `T`; e.g., an `index` of 3 represents a pointer
    /// offset of `3 * size_of::&lt;T&gt;()` bytes.
    ///
    /// If the `T` is a ZST, then we instead track the index of the element
    /// in the table so that `erase` works properly (return
    /// `NonNull::new_unchecked((index + 1) as *mut T)`)
    ///
    /// # Safety
    ///
    /// If `mem::size_of::&lt;T&gt;() != 0`, then the safety rules are directly derived
    /// from the safety rules for [`&lt;*mut T&gt;::sub`] method of `*mut T` and the safety
    /// rules of [`NonNull::new_unchecked`] function.
    ///
    /// Thus, in order to uphold the safety contracts for the [`&lt;*mut T&gt;::sub`] method
    /// and [`NonNull::new_unchecked`] function, as well as for the correct
    /// logic of the work of this crate, the following rules are necessary and
    /// sufficient:
    ///
    /// * the `base` pointer must not be `dangling` and must points to the
    ///   end of the first `value element` from the `data part` of the table, i.e.
    ///   must be the pointer that returned by [`RawTable::data_end`] or by
    ///   [`RawTableInner::data_end&lt;T&gt;`];
    ///
    /// * `index` must not be greater than `RawTableInner.bucket_mask`, i.e.
    ///   `index &lt;= RawTableInner.bucket_mask` or, in other words, `(index + 1)`
    ///   must be no greater than the number returned by the function
    ///   [`RawTable::buckets`] or [`RawTableInner::buckets`].
    ///
    /// If `mem::size_of::&lt;T&gt;() == 0`, then the only requirement is that the
    /// `index` must not be greater than `RawTableInner.bucket_mask`, i.e.
    /// `index &lt;= RawTableInner.bucket_mask` or, in other words, `(index + 1)`
    /// must be no greater than the number returned by the function
    /// [`RawTable::buckets`] or [`RawTableInner::buckets`].
    ///
    /// [`Bucket`]: crate::raw::Bucket
    /// [`&lt;*mut T&gt;::sub`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.sub-1
    /// [`NonNull::new_unchecked`]: https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.new_unchecked
    /// [`RawTable::data_end`]: crate::raw::RawTable::data_end
    /// [`RawTableInner::data_end&lt;T&gt;`]: RawTableInner::data_end&lt;T&gt;
    /// [`RawTable::buckets`]: crate::raw::RawTable::buckets
    /// [`RawTableInner::buckets`]: RawTableInner::buckets
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>from_base_index(base: NonNull&lt;T&gt;, index: usize) -&gt; <span class="self">Self </span>{
        <span class="comment">// If mem::size_of::&lt;T&gt;() != 0 then return a pointer to an `element` in
        // the data part of the table (we start counting from "0", so that
        // in the expression T[last], the "last" index actually one less than the
        // "buckets" number in the table, i.e. "last = RawTableInner.bucket_mask"):
        //
        //                   `from_base_index(base, 1).as_ptr()` returns a pointer that
        //                   points here in the data part of the table
        //                   (to the start of T1)
        //                        |
        //                        |        `base: NonNull&lt;T&gt;` must point here
        //                        |         (to the end of T0 or to the start of C0)
        //                        v         v
        // [Padding], Tlast, ..., |T1|, T0, |C0, C1, ..., Clast
        //                           ^
        //                           `from_base_index(base, 1)` returns a pointer
        //                           that points here in the data part of the table
        //                           (to the end of T1)
        //
        // where: T0...Tlast - our stored data; C0...Clast - control bytes
        // or metadata for data.
        </span><span class="kw">let </span>ptr = <span class="kw">if </span>T::IS_ZERO_SIZED {
            <span class="comment">// won't overflow because index must be less than length (bucket_mask)
            // and bucket_mask is guaranteed to be less than `isize::MAX`
            // (see TableLayout::calculate_layout_for method)
            </span>invalid_mut(index + <span class="number">1</span>)
        } <span class="kw">else </span>{
            base.as_ptr().sub(index)
        };
        <span class="self">Self </span>{
            ptr: NonNull::new_unchecked(ptr),
        }
    }

    <span class="doccomment">/// Calculates the index of a [`Bucket`] as distance between two pointers
    /// (convenience for `base.as_ptr().offset_from(self.ptr.as_ptr()) as usize`).
    /// The returned value is in units of T: the distance in bytes divided by
    /// [`core::mem::size_of::&lt;T&gt;()`].
    ///
    /// If the `T` is a ZST, then we return the index of the element in
    /// the table so that `erase` works properly (return `self.ptr.as_ptr() as usize - 1`).
    ///
    /// This function is the inverse of [`from_base_index`].
    ///
    /// # Safety
    ///
    /// If `mem::size_of::&lt;T&gt;() != 0`, then the safety rules are directly derived
    /// from the safety rules for [`&lt;*const T&gt;::offset_from`] method of `*const T`.
    ///
    /// Thus, in order to uphold the safety contracts for [`&lt;*const T&gt;::offset_from`]
    /// method, as well as for the correct logic of the work of this crate, the
    /// following rules are necessary and sufficient:
    ///
    /// * `base` contained pointer must not be `dangling` and must point to the
    ///   end of the first `element` from the `data part` of the table, i.e.
    ///   must be a pointer that returns by [`RawTable::data_end`] or by
    ///   [`RawTableInner::data_end&lt;T&gt;`];
    ///
    /// * `self` also must not contain dangling pointer;
    ///
    /// * both `self` and `base` must be created from the same [`RawTable`]
    ///   (or [`RawTableInner`]).
    ///
    /// If `mem::size_of::&lt;T&gt;() == 0`, this function is always safe.
    ///
    /// [`Bucket`]: crate::raw::Bucket
    /// [`from_base_index`]: crate::raw::Bucket::from_base_index
    /// [`RawTable::data_end`]: crate::raw::RawTable::data_end
    /// [`RawTableInner::data_end&lt;T&gt;`]: RawTableInner::data_end&lt;T&gt;
    /// [`RawTable`]: crate::raw::RawTable
    /// [`RawTableInner`]: RawTableInner
    /// [`&lt;*const T&gt;::offset_from`]: https://doc.rust-lang.org/nightly/core/primitive.pointer.html#method.offset_from
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>to_base_index(<span class="kw-2">&amp;</span><span class="self">self</span>, base: NonNull&lt;T&gt;) -&gt; usize {
        <span class="comment">// If mem::size_of::&lt;T&gt;() != 0 then return an index under which we used to store the
        // `element` in the data part of the table (we start counting from "0", so
        // that in the expression T[last], the "last" index actually is one less than the
        // "buckets" number in the table, i.e. "last = RawTableInner.bucket_mask").
        // For example for 5th element in table calculation is performed like this:
        //
        //                        mem::size_of::&lt;T&gt;()
        //                          |
        //                          |         `self = from_base_index(base, 5)` that returns pointer
        //                          |         that points here in tha data part of the table
        //                          |         (to the end of T5)
        //                          |           |                    `base: NonNull&lt;T&gt;` must point here
        //                          v           |                    (to the end of T0 or to the start of C0)
        //                        /???\         v                      v
        // [Padding], Tlast, ..., |T10|, ..., T5|, T4, T3, T2, T1, T0, |C0, C1, C2, C3, C4, C5, ..., C10, ..., Clast
        //                                      \__________  __________/
        //                                                 \/
        //                                     `bucket.to_base_index(base)` = 5
        //                                     (base.as_ptr() as usize - self.ptr.as_ptr() as usize) / mem::size_of::&lt;T&gt;()
        //
        // where: T0...Tlast - our stored data; C0...Clast - control bytes or metadata for data.
        </span><span class="kw">if </span>T::IS_ZERO_SIZED {
            <span class="comment">// this can not be UB
            </span><span class="self">self</span>.ptr.as_ptr() <span class="kw">as </span>usize - <span class="number">1
        </span>} <span class="kw">else </span>{
            offset_from(base.as_ptr(), <span class="self">self</span>.ptr.as_ptr())
        }
    }

    <span class="doccomment">/// Acquires the underlying raw pointer `*mut T` to `data`.
    ///
    /// # Note
    ///
    /// If `T` is not [`Copy`], do not use `*mut T` methods that can cause calling the
    /// destructor of `T` (for example the [`&lt;*mut T&gt;::drop_in_place`] method), because
    /// for properly dropping the data we also need to clear `data` control bytes. If we
    /// drop data, but do not clear `data control byte` it leads to double drop when
    /// [`RawTable`] goes out of scope.
    ///
    /// If you modify an already initialized `value`, so [`Hash`] and [`Eq`] on the new
    /// `T` value and its borrowed form *must* match those for the old `T` value, as the map
    /// will not re-evaluate where the new value should go, meaning the value may become
    /// "lost" if their location does not reflect their state.
    ///
    /// [`RawTable`]: crate::raw::RawTable
    /// [`&lt;*mut T&gt;::drop_in_place`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.drop_in_place
    /// [`Hash`]: https://doc.rust-lang.org/core/hash/trait.Hash.html
    /// [`Eq`]: https://doc.rust-lang.org/core/cmp/trait.Eq.html
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "raw")]
    /// # fn test() {
    /// use core::hash::{BuildHasher, Hash};
    /// use hashbrown::raw::{Bucket, RawTable};
    ///
    /// type NewHashBuilder = core::hash::BuildHasherDefault&lt;ahash::AHasher&gt;;
    ///
    /// fn make_hash&lt;K: Hash + ?Sized, S: BuildHasher&gt;(hash_builder: &amp;S, key: &amp;K) -&gt; u64 {
    ///     use core::hash::Hasher;
    ///     let mut state = hash_builder.build_hasher();
    ///     key.hash(&amp;mut state);
    ///     state.finish()
    /// }
    ///
    /// let hash_builder = NewHashBuilder::default();
    /// let mut table = RawTable::new();
    ///
    /// let value = ("a", 100);
    /// let hash = make_hash(&amp;hash_builder, &amp;value.0);
    ///
    /// table.insert(hash, value.clone(), |val| make_hash(&amp;hash_builder, &amp;val.0));
    ///
    /// let bucket: Bucket&lt;(&amp;str, i32)&gt; = table.find(hash, |(k1, _)| k1 == &amp;value.0).unwrap();
    ///
    /// assert_eq!(unsafe { &amp;*bucket.as_ptr() }, &amp;("a", 100));
    /// # }
    /// # fn main() {
    /// #     #[cfg(feature = "raw")]
    /// #     test()
    /// # }
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>as_ptr(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">*mut </span>T {
        <span class="kw">if </span>T::IS_ZERO_SIZED {
            <span class="comment">// Just return an arbitrary ZST pointer which is properly aligned
            // invalid pointer is good enough for ZST
            </span>invalid_mut(mem::align_of::&lt;T&gt;())
        } <span class="kw">else </span>{
            <span class="kw">unsafe </span>{ <span class="self">self</span>.ptr.as_ptr().sub(<span class="number">1</span>) }
        }
    }

    <span class="doccomment">/// Create a new [`Bucket`] that is offset from the `self` by the given
    /// `offset`. The pointer calculation is performed by calculating the
    /// offset from `self` pointer (convenience for `self.ptr.as_ptr().sub(offset)`).
    /// This function is used for iterators.
    ///
    /// `offset` is in units of `T`; e.g., a `offset` of 3 represents a pointer
    /// offset of `3 * size_of::&lt;T&gt;()` bytes.
    ///
    /// # Safety
    ///
    /// If `mem::size_of::&lt;T&gt;() != 0`, then the safety rules are directly derived
    /// from the safety rules for [`&lt;*mut T&gt;::sub`] method of `*mut T` and safety
    /// rules of [`NonNull::new_unchecked`] function.
    ///
    /// Thus, in order to uphold the safety contracts for [`&lt;*mut T&gt;::sub`] method
    /// and [`NonNull::new_unchecked`] function, as well as for the correct
    /// logic of the work of this crate, the following rules are necessary and
    /// sufficient:
    ///
    /// * `self` contained pointer must not be `dangling`;
    ///
    /// * `self.to_base_index() + ofset` must not be greater than `RawTableInner.bucket_mask`,
    ///   i.e. `(self.to_base_index() + ofset) &lt;= RawTableInner.bucket_mask` or, in other
    ///   words, `self.to_base_index() + ofset + 1` must be no greater than the number returned
    ///   by the function [`RawTable::buckets`] or [`RawTableInner::buckets`].
    ///
    /// If `mem::size_of::&lt;T&gt;() == 0`, then the only requirement is that the
    /// `self.to_base_index() + ofset` must not be greater than `RawTableInner.bucket_mask`,
    /// i.e. `(self.to_base_index() + ofset) &lt;= RawTableInner.bucket_mask` or, in other words,
    /// `self.to_base_index() + ofset + 1` must be no greater than the number returned by the
    /// function [`RawTable::buckets`] or [`RawTableInner::buckets`].
    ///
    /// [`Bucket`]: crate::raw::Bucket
    /// [`&lt;*mut T&gt;::sub`]: https://doc.rust-lang.org/core/primitive.pointer.html#method.sub-1
    /// [`NonNull::new_unchecked`]: https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.new_unchecked
    /// [`RawTable::buckets`]: crate::raw::RawTable::buckets
    /// [`RawTableInner::buckets`]: RawTableInner::buckets
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>next_n(<span class="kw-2">&amp;</span><span class="self">self</span>, offset: usize) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>ptr = <span class="kw">if </span>T::IS_ZERO_SIZED {
            <span class="comment">// invalid pointer is good enough for ZST
            </span>invalid_mut(<span class="self">self</span>.ptr.as_ptr() <span class="kw">as </span>usize + offset)
        } <span class="kw">else </span>{
            <span class="self">self</span>.ptr.as_ptr().sub(offset)
        };
        <span class="self">Self </span>{
            ptr: NonNull::new_unchecked(ptr),
        }
    }

    <span class="doccomment">/// Executes the destructor (if any) of the pointed-to `data`.
    ///
    /// # Safety
    ///
    /// See [`ptr::drop_in_place`] for safety concerns.
    ///
    /// You should use [`RawTable::erase`] instead of this function,
    /// or be careful with calling this function directly, because for
    /// properly dropping the data we need also clear `data` control bytes.
    /// If we drop data, but do not erase `data control byte` it leads to
    /// double drop when [`RawTable`] goes out of scope.
    ///
    /// [`ptr::drop_in_place`]: https://doc.rust-lang.org/core/ptr/fn.drop_in_place.html
    /// [`RawTable`]: crate::raw::RawTable
    /// [`RawTable::erase`]: crate::raw::RawTable::erase
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">unsafe fn </span>drop(<span class="kw-2">&amp;</span><span class="self">self</span>) {
        <span class="self">self</span>.as_ptr().drop_in_place();
    }

    <span class="doccomment">/// Reads the `value` from `self` without moving it. This leaves the
    /// memory in `self` unchanged.
    ///
    /// # Safety
    ///
    /// See [`ptr::read`] for safety concerns.
    ///
    /// You should use [`RawTable::remove`] instead of this function,
    /// or be careful with calling this function directly, because compiler
    /// calls its destructor when readed `value` goes out of scope. It
    /// can cause double dropping when [`RawTable`] goes out of scope,
    /// because of not erased `data control byte`.
    ///
    /// [`ptr::read`]: https://doc.rust-lang.org/core/ptr/fn.read.html
    /// [`RawTable`]: crate::raw::RawTable
    /// [`RawTable::remove`]: crate::raw::RawTable::remove
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">unsafe fn </span>read(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; T {
        <span class="self">self</span>.as_ptr().read()
    }

    <span class="doccomment">/// Overwrites a memory location with the given `value` without reading
    /// or dropping the old value (like [`ptr::write`] function).
    ///
    /// # Safety
    ///
    /// See [`ptr::write`] for safety concerns.
    ///
    /// # Note
    ///
    /// [`Hash`] and [`Eq`] on the new `T` value and its borrowed form *must* match
    /// those for the old `T` value, as the map will not re-evaluate where the new
    /// value should go, meaning the value may become "lost" if their location
    /// does not reflect their state.
    ///
    /// [`ptr::write`]: https://doc.rust-lang.org/core/ptr/fn.write.html
    /// [`Hash`]: https://doc.rust-lang.org/core/hash/trait.Hash.html
    /// [`Eq`]: https://doc.rust-lang.org/core/cmp/trait.Eq.html
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">unsafe fn </span>write(<span class="kw-2">&amp;</span><span class="self">self</span>, val: T) {
        <span class="self">self</span>.as_ptr().write(val);
    }

    <span class="doccomment">/// Returns a shared immutable reference to the `value`.
    ///
    /// # Safety
    ///
    /// See [`NonNull::as_ref`] for safety concerns.
    ///
    /// [`NonNull::as_ref`]: https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.as_ref
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "raw")]
    /// # fn test() {
    /// use core::hash::{BuildHasher, Hash};
    /// use hashbrown::raw::{Bucket, RawTable};
    ///
    /// type NewHashBuilder = core::hash::BuildHasherDefault&lt;ahash::AHasher&gt;;
    ///
    /// fn make_hash&lt;K: Hash + ?Sized, S: BuildHasher&gt;(hash_builder: &amp;S, key: &amp;K) -&gt; u64 {
    ///     use core::hash::Hasher;
    ///     let mut state = hash_builder.build_hasher();
    ///     key.hash(&amp;mut state);
    ///     state.finish()
    /// }
    ///
    /// let hash_builder = NewHashBuilder::default();
    /// let mut table = RawTable::new();
    ///
    /// let value: (&amp;str, String) = ("A pony", "is a small horse".to_owned());
    /// let hash = make_hash(&amp;hash_builder, &amp;value.0);
    ///
    /// table.insert(hash, value.clone(), |val| make_hash(&amp;hash_builder, &amp;val.0));
    ///
    /// let bucket: Bucket&lt;(&amp;str, String)&gt; = table.find(hash, |(k, _)| k == &amp;value.0).unwrap();
    ///
    /// assert_eq!(
    ///     unsafe { bucket.as_ref() },
    ///     &amp;("A pony", "is a small horse".to_owned())
    /// );
    /// # }
    /// # fn main() {
    /// #     #[cfg(feature = "raw")]
    /// #     test()
    /// # }
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub unsafe fn </span>as_ref&lt;<span class="lifetime">'a</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'a </span>T {
        <span class="kw-2">&amp;*</span><span class="self">self</span>.as_ptr()
    }

    <span class="doccomment">/// Returns a unique mutable reference to the `value`.
    ///
    /// # Safety
    ///
    /// See [`NonNull::as_mut`] for safety concerns.
    ///
    /// # Note
    ///
    /// [`Hash`] and [`Eq`] on the new `T` value and its borrowed form *must* match
    /// those for the old `T` value, as the map will not re-evaluate where the new
    /// value should go, meaning the value may become "lost" if their location
    /// does not reflect their state.
    ///
    /// [`NonNull::as_mut`]: https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.as_mut
    /// [`Hash`]: https://doc.rust-lang.org/core/hash/trait.Hash.html
    /// [`Eq`]: https://doc.rust-lang.org/core/cmp/trait.Eq.html
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "raw")]
    /// # fn test() {
    /// use core::hash::{BuildHasher, Hash};
    /// use hashbrown::raw::{Bucket, RawTable};
    ///
    /// type NewHashBuilder = core::hash::BuildHasherDefault&lt;ahash::AHasher&gt;;
    ///
    /// fn make_hash&lt;K: Hash + ?Sized, S: BuildHasher&gt;(hash_builder: &amp;S, key: &amp;K) -&gt; u64 {
    ///     use core::hash::Hasher;
    ///     let mut state = hash_builder.build_hasher();
    ///     key.hash(&amp;mut state);
    ///     state.finish()
    /// }
    ///
    /// let hash_builder = NewHashBuilder::default();
    /// let mut table = RawTable::new();
    ///
    /// let value: (&amp;str, String) = ("A pony", "is a small horse".to_owned());
    /// let hash = make_hash(&amp;hash_builder, &amp;value.0);
    ///
    /// table.insert(hash, value.clone(), |val| make_hash(&amp;hash_builder, &amp;val.0));
    ///
    /// let bucket: Bucket&lt;(&amp;str, String)&gt; = table.find(hash, |(k, _)| k == &amp;value.0).unwrap();
    ///
    /// unsafe {
    ///     bucket
    ///         .as_mut()
    ///         .1
    ///         .push_str(" less than 147 cm at the withers")
    /// };
    /// assert_eq!(
    ///     unsafe { bucket.as_ref() },
    ///     &amp;(
    ///         "A pony",
    ///         "is a small horse less than 147 cm at the withers".to_owned()
    ///     )
    /// );
    /// # }
    /// # fn main() {
    /// #     #[cfg(feature = "raw")]
    /// #     test()
    /// # }
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub unsafe fn </span>as_mut&lt;<span class="lifetime">'a</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>T {
        <span class="kw-2">&amp;mut *</span><span class="self">self</span>.as_ptr()
    }

    <span class="doccomment">/// Copies `size_of&lt;T&gt;` bytes from `other` to `self`. The source
    /// and destination may *not* overlap.
    ///
    /// # Safety
    ///
    /// See [`ptr::copy_nonoverlapping`] for safety concerns.
    ///
    /// Like [`read`], `copy_nonoverlapping` creates a bitwise copy of `T`, regardless of
    /// whether `T` is [`Copy`]. If `T` is not [`Copy`], using *both* the values
    /// in the region beginning at `*self` and the region beginning at `*other` can
    /// [violate memory safety].
    ///
    /// # Note
    ///
    /// [`Hash`] and [`Eq`] on the new `T` value and its borrowed form *must* match
    /// those for the old `T` value, as the map will not re-evaluate where the new
    /// value should go, meaning the value may become "lost" if their location
    /// does not reflect their state.
    ///
    /// [`ptr::copy_nonoverlapping`]: https://doc.rust-lang.org/core/ptr/fn.copy_nonoverlapping.html
    /// [`read`]: https://doc.rust-lang.org/core/ptr/fn.read.html
    /// [violate memory safety]: https://doc.rust-lang.org/std/ptr/fn.read.html#ownership-of-the-returned-value
    /// [`Hash`]: https://doc.rust-lang.org/core/hash/trait.Hash.html
    /// [`Eq`]: https://doc.rust-lang.org/core/cmp/trait.Eq.html
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    #[inline]
    </span><span class="kw">pub unsafe fn </span>copy_from_nonoverlapping(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) {
        <span class="self">self</span>.as_ptr().copy_from_nonoverlapping(other.as_ptr(), <span class="number">1</span>);
    }
}

<span class="doccomment">/// A raw hash table with an unsafe API.
</span><span class="kw">pub struct </span>RawTable&lt;T, A: Allocator = Global&gt; {
    table: RawTableInner,
    alloc: A,
    <span class="comment">// Tell dropck that we own instances of T.
    </span>marker: PhantomData&lt;T&gt;,
}

<span class="doccomment">/// Non-generic part of `RawTable` which allows functions to be instantiated only once regardless
/// of how many different key-value types are used.
</span><span class="kw">struct </span>RawTableInner {
    <span class="comment">// Mask to get an index from a hash value. The value is one less than the
    // number of buckets in the table.
    </span>bucket_mask: usize,

    <span class="comment">// [Padding], T1, T2, ..., Tlast, C1, C2, ...
    //                                ^ points here
    </span>ctrl: NonNull&lt;u8&gt;,

    <span class="comment">// Number of elements that can be inserted before we need to grow the table
    </span>growth_left: usize,

    <span class="comment">// Number of elements in the table, only really used by len()
    </span>items: usize,
}

<span class="kw">impl</span>&lt;T&gt; RawTable&lt;T, Global&gt; {
    <span class="doccomment">/// Creates a new empty hash table without allocating any memory.
    ///
    /// In effect this returns a table with exactly 1 bucket. However we can
    /// leave the data pointer dangling since that bucket is never written to
    /// due to our load factor forcing us to always have at least 1 free bucket.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub const fn </span>new() -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            table: RawTableInner::NEW,
            alloc: Global,
            marker: PhantomData,
        }
    }

    <span class="doccomment">/// Attempts to allocate a new hash table with at least enough capacity
    /// for inserting the given number of elements without reallocating.
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">pub fn </span>try_with_capacity(capacity: usize) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, TryReserveError&gt; {
        <span class="self">Self</span>::try_with_capacity_in(capacity, Global)
    }

    <span class="doccomment">/// Allocates a new hash table with at least enough capacity for inserting
    /// the given number of elements without reallocating.
    </span><span class="kw">pub fn </span>with_capacity(capacity: usize) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::with_capacity_in(capacity, Global)
    }
}

<span class="kw">impl</span>&lt;T, A: Allocator&gt; RawTable&lt;T, A&gt; {
    <span class="kw">const </span>TABLE_LAYOUT: TableLayout = TableLayout::new::&lt;T&gt;();

    <span class="doccomment">/// Creates a new empty hash table without allocating any memory, using the
    /// given allocator.
    ///
    /// In effect this returns a table with exactly 1 bucket. However we can
    /// leave the data pointer dangling since that bucket is never written to
    /// due to our load factor forcing us to always have at least 1 free bucket.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub const fn </span>new_in(alloc: A) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            table: RawTableInner::NEW,
            alloc,
            marker: PhantomData,
        }
    }

    <span class="doccomment">/// Allocates a new hash table with the given number of buckets.
    ///
    /// The control bytes are left uninitialized.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">unsafe fn </span>new_uninitialized(
        alloc: A,
        buckets: usize,
        fallibility: Fallibility,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, TryReserveError&gt; {
        <span class="macro">debug_assert!</span>(buckets.is_power_of_two());

        <span class="prelude-val">Ok</span>(<span class="self">Self </span>{
            table: RawTableInner::new_uninitialized(
                <span class="kw-2">&amp;</span>alloc,
                <span class="self">Self</span>::TABLE_LAYOUT,
                buckets,
                fallibility,
            )<span class="question-mark">?</span>,
            alloc,
            marker: PhantomData,
        })
    }

    <span class="doccomment">/// Attempts to allocate a new hash table using the given allocator, with at least enough
    /// capacity for inserting the given number of elements without reallocating.
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">pub fn </span>try_with_capacity_in(capacity: usize, alloc: A) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, TryReserveError&gt; {
        <span class="prelude-val">Ok</span>(<span class="self">Self </span>{
            table: RawTableInner::fallible_with_capacity(
                <span class="kw-2">&amp;</span>alloc,
                <span class="self">Self</span>::TABLE_LAYOUT,
                capacity,
                Fallibility::Fallible,
            )<span class="question-mark">?</span>,
            alloc,
            marker: PhantomData,
        })
    }

    <span class="doccomment">/// Allocates a new hash table using the given allocator, with at least enough capacity for
    /// inserting the given number of elements without reallocating.
    </span><span class="kw">pub fn </span>with_capacity_in(capacity: usize, alloc: A) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            table: RawTableInner::with_capacity(<span class="kw-2">&amp;</span>alloc, <span class="self">Self</span>::TABLE_LAYOUT, capacity),
            alloc,
            marker: PhantomData,
        }
    }

    <span class="doccomment">/// Returns a reference to the underlying allocator.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>allocator(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>A {
        <span class="kw-2">&amp;</span><span class="self">self</span>.alloc
    }

    <span class="doccomment">/// Returns pointer to one past last `data` element in the the table as viewed from
    /// the start point of the allocation.
    ///
    /// The caller must ensure that the `RawTable` outlives the returned [`NonNull&lt;T&gt;`],
    /// otherwise using it may result in [`undefined behavior`].
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>data_end(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; NonNull&lt;T&gt; {
        <span class="comment">// SAFETY: `self.table.ctrl` is `NonNull`, so casting it is safe
        //
        //                        `self.table.ctrl.as_ptr().cast()` returns pointer that
        //                        points here (to the end of `T0`)
        //                          ∨
        // [Pad], T_n, ..., T1, T0, |CT0, CT1, ..., CT_n|, CTa_0, CTa_1, ..., CTa_m
        //                           \________  ________/
        //                                    \/
        //       `n = buckets - 1`, i.e. `RawTable::buckets() - 1`
        //
        // where: T0...T_n  - our stored data;
        //        CT0...CT_n - control bytes or metadata for `data`.
        //        CTa_0...CTa_m - additional control bytes, where `m = Group::WIDTH - 1` (so that the search
        //                        with loading `Group` bytes from the heap works properly, even if the result
        //                        of `h1(hash) &amp; self.bucket_mask` is equal to `self.bucket_mask`). See also
        //                        `RawTableInner::set_ctrl` function.
        //
        // P.S. `h1(hash) &amp; self.bucket_mask` is the same as `hash as usize % self.buckets()` because the number
        // of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
        </span><span class="kw">unsafe </span>{ NonNull::new_unchecked(<span class="self">self</span>.table.ctrl.as_ptr().cast()) }
    }

    <span class="doccomment">/// Returns pointer to start of data table.
    </span><span class="attr">#[inline]
    #[cfg(any(feature = <span class="string">"raw"</span>, feature = <span class="string">"nightly"</span>))]
    </span><span class="kw">pub unsafe fn </span>data_start(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; NonNull&lt;T&gt; {
        NonNull::new_unchecked(<span class="self">self</span>.data_end().as_ptr().wrapping_sub(<span class="self">self</span>.buckets()))
    }

    <span class="doccomment">/// Return the information about memory allocated by the table.
    ///
    /// `RawTable` allocates single memory block to store both data and metadata.
    /// This function returns allocation size and alignment and the beginning of the area.
    /// These are the arguments which will be passed to `dealloc` when the table is dropped.
    ///
    /// This function might be useful for memory profiling.
    </span><span class="attr">#[inline]
    #[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">pub fn </span>allocation_info(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (NonNull&lt;u8&gt;, Layout) {
        <span class="comment">// SAFETY: We use the same `table_layout` that was used to allocate
        // this table.
        </span><span class="kw">unsafe </span>{ <span class="self">self</span>.table.allocation_info_or_zero(<span class="self">Self</span>::TABLE_LAYOUT) }
    }

    <span class="doccomment">/// Returns the index of a bucket from a `Bucket`.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub unsafe fn </span>bucket_index(<span class="kw-2">&amp;</span><span class="self">self</span>, bucket: <span class="kw-2">&amp;</span>Bucket&lt;T&gt;) -&gt; usize {
        bucket.to_base_index(<span class="self">self</span>.data_end())
    }

    <span class="doccomment">/// Returns a pointer to an element in the table.
    ///
    /// The caller must ensure that the `RawTable` outlives the returned [`Bucket&lt;T&gt;`],
    /// otherwise using it may result in [`undefined behavior`].
    ///
    /// # Safety
    ///
    /// If `mem::size_of::&lt;T&gt;() != 0`, then the caller of this function must observe the
    /// following safety rules:
    ///
    /// * The table must already be allocated;
    ///
    /// * The `index` must not be greater than the number returned by the [`RawTable::buckets`]
    ///   function, i.e. `(index + 1) &lt;= self.buckets()`.
    ///
    /// It is safe to call this function with index of zero (`index == 0`) on a table that has
    /// not been allocated, but using the returned [`Bucket`] results in [`undefined behavior`].
    ///
    /// If `mem::size_of::&lt;T&gt;() == 0`, then the only requirement is that the `index` must
    /// not be greater than the number returned by the [`RawTable::buckets`] function, i.e.
    /// `(index + 1) &lt;= self.buckets()`.
    ///
    /// [`RawTable::buckets`]: RawTable::buckets
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">pub unsafe fn </span>bucket(<span class="kw-2">&amp;</span><span class="self">self</span>, index: usize) -&gt; Bucket&lt;T&gt; {
        <span class="comment">// If mem::size_of::&lt;T&gt;() != 0 then return a pointer to the `element` in the `data part` of the table
        // (we start counting from "0", so that in the expression T[n], the "n" index actually one less than
        // the "buckets" number of our `RawTable`, i.e. "n = RawTable::buckets() - 1"):
        //
        //           `table.bucket(3).as_ptr()` returns a pointer that points here in the `data`
        //           part of the `RawTable`, i.e. to the start of T3 (see `Bucket::as_ptr`)
        //                  |
        //                  |               `base = self.data_end()` points here
        //                  |               (to the start of CT0 or to the end of T0)
        //                  v                 v
        // [Pad], T_n, ..., |T3|, T2, T1, T0, |CT0, CT1, CT2, CT3, ..., CT_n, CTa_0, CTa_1, ..., CTa_m
        //                     ^                                              \__________  __________/
        //        `table.bucket(3)` returns a pointer that points                        \/
        //         here in the `data` part of the `RawTable` (to              additional control bytes
        //         the end of T3)                                              `m = Group::WIDTH - 1`
        //
        // where: T0...T_n  - our stored data;
        //        CT0...CT_n - control bytes or metadata for `data`;
        //        CTa_0...CTa_m - additional control bytes (so that the search with loading `Group` bytes from
        //                        the heap works properly, even if the result of `h1(hash) &amp; self.table.bucket_mask`
        //                        is equal to `self.table.bucket_mask`). See also `RawTableInner::set_ctrl` function.
        //
        // P.S. `h1(hash) &amp; self.table.bucket_mask` is the same as `hash as usize % self.buckets()` because the number
        // of buckets is a power of two, and `self.table.bucket_mask = self.buckets() - 1`.
        </span><span class="macro">debug_assert_ne!</span>(<span class="self">self</span>.table.bucket_mask, <span class="number">0</span>);
        <span class="macro">debug_assert!</span>(index &lt; <span class="self">self</span>.buckets());
        Bucket::from_base_index(<span class="self">self</span>.data_end(), index)
    }

    <span class="doccomment">/// Erases an element from the table without dropping it.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">unsafe fn </span>erase_no_drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>, item: <span class="kw-2">&amp;</span>Bucket&lt;T&gt;) {
        <span class="kw">let </span>index = <span class="self">self</span>.bucket_index(item);
        <span class="self">self</span>.table.erase(index);
    }

    <span class="doccomment">/// Erases an element from the table, dropping it in place.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    #[allow(clippy::needless_pass_by_value)]
    </span><span class="kw">pub unsafe fn </span>erase(<span class="kw-2">&amp;mut </span><span class="self">self</span>, item: Bucket&lt;T&gt;) {
        <span class="comment">// Erase the element from the table first since drop might panic.
        </span><span class="self">self</span>.erase_no_drop(<span class="kw-2">&amp;</span>item);
        item.drop();
    }

    <span class="doccomment">/// Finds and erases an element from the table, dropping it in place.
    /// Returns true if an element was found.
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    #[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>erase_entry(<span class="kw-2">&amp;mut </span><span class="self">self</span>, hash: u64, eq: <span class="kw">impl </span>FnMut(<span class="kw-2">&amp;</span>T) -&gt; bool) -&gt; bool {
        <span class="comment">// Avoid `Option::map` because it bloats LLVM IR.
        </span><span class="kw">if let </span><span class="prelude-val">Some</span>(bucket) = <span class="self">self</span>.find(hash, eq) {
            <span class="kw">unsafe </span>{
                <span class="self">self</span>.erase(bucket);
            }
            <span class="bool-val">true
        </span>} <span class="kw">else </span>{
            <span class="bool-val">false
        </span>}
    }

    <span class="doccomment">/// Removes an element from the table, returning it.
    ///
    /// This also returns an `InsertSlot` pointing to the newly free bucket.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    #[allow(clippy::needless_pass_by_value)]
    </span><span class="kw">pub unsafe fn </span>remove(<span class="kw-2">&amp;mut </span><span class="self">self</span>, item: Bucket&lt;T&gt;) -&gt; (T, InsertSlot) {
        <span class="self">self</span>.erase_no_drop(<span class="kw-2">&amp;</span>item);
        (
            item.read(),
            InsertSlot {
                index: <span class="self">self</span>.bucket_index(<span class="kw-2">&amp;</span>item),
            },
        )
    }

    <span class="doccomment">/// Finds and removes an element from the table, returning it.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>remove_entry(<span class="kw-2">&amp;mut </span><span class="self">self</span>, hash: u64, eq: <span class="kw">impl </span>FnMut(<span class="kw-2">&amp;</span>T) -&gt; bool) -&gt; <span class="prelude-ty">Option</span>&lt;T&gt; {
        <span class="comment">// Avoid `Option::map` because it bloats LLVM IR.
        </span><span class="kw">match </span><span class="self">self</span>.find(hash, eq) {
            <span class="prelude-val">Some</span>(bucket) =&gt; <span class="prelude-val">Some</span>(<span class="kw">unsafe </span>{ <span class="self">self</span>.remove(bucket).<span class="number">0 </span>}),
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">None</span>,
        }
    }

    <span class="doccomment">/// Marks all table buckets as empty without dropping their contents.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>clear_no_drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.table.clear_no_drop();
    }

    <span class="doccomment">/// Removes all elements from the table without freeing the backing memory.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>clear(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">if </span><span class="self">self</span>.is_empty() {
            <span class="comment">// Special case empty table to avoid surprising O(capacity) time.
            </span><span class="kw">return</span>;
        }
        <span class="comment">// Ensure that the table is reset even if one of the drops panic
        </span><span class="kw">let </span><span class="kw-2">mut </span>self_ = guard(<span class="self">self</span>, |self_| self_.clear_no_drop());
        <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY: ScopeGuard sets to zero the `items` field of the table
            // even in case of panic during the dropping of the elements so
            // that there will be no double drop of the elements.
            </span>self_.table.drop_elements::&lt;T&gt;();
        }
    }

    <span class="doccomment">/// Shrinks the table to fit `max(self.len(), min_size)` elements.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>shrink_to(<span class="kw-2">&amp;mut </span><span class="self">self</span>, min_size: usize, hasher: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; u64) {
        <span class="comment">// Calculate the minimal number of elements that we need to reserve
        // space for.
        </span><span class="kw">let </span>min_size = usize::max(<span class="self">self</span>.table.items, min_size);
        <span class="kw">if </span>min_size == <span class="number">0 </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>old_inner = mem::replace(<span class="kw-2">&amp;mut </span><span class="self">self</span>.table, RawTableInner::NEW);
            <span class="kw">unsafe </span>{
                <span class="comment">// SAFETY:
                // 1. We call the function only once;
                // 2. We know for sure that `alloc` and `table_layout` matches the [`Allocator`]
                //    and [`TableLayout`] that were used to allocate this table.
                // 3. If any elements' drop function panics, then there will only be a memory leak,
                //    because we have replaced the inner table with a new one.
                </span>old_inner.drop_inner_table::&lt;T, <span class="kw">_</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>.alloc, <span class="self">Self</span>::TABLE_LAYOUT);
            }
            <span class="kw">return</span>;
        }

        <span class="comment">// Calculate the number of buckets that we need for this number of
        // elements. If the calculation overflows then the requested bucket
        // count must be larger than what we have right and nothing needs to be
        // done.
        </span><span class="kw">let </span>min_buckets = <span class="kw">match </span>capacity_to_buckets(min_size) {
            <span class="prelude-val">Some</span>(buckets) =&gt; buckets,
            <span class="prelude-val">None </span>=&gt; <span class="kw">return</span>,
        };

        <span class="comment">// If we have more buckets than we need, shrink the table.
        </span><span class="kw">if </span>min_buckets &lt; <span class="self">self</span>.buckets() {
            <span class="comment">// Fast path if the table is empty
            </span><span class="kw">if </span><span class="self">self</span>.table.items == <span class="number">0 </span>{
                <span class="kw">let </span>new_inner =
                    RawTableInner::with_capacity(<span class="kw-2">&amp;</span><span class="self">self</span>.alloc, <span class="self">Self</span>::TABLE_LAYOUT, min_size);
                <span class="kw">let </span><span class="kw-2">mut </span>old_inner = mem::replace(<span class="kw-2">&amp;mut </span><span class="self">self</span>.table, new_inner);
                <span class="kw">unsafe </span>{
                    <span class="comment">// SAFETY:
                    // 1. We call the function only once;
                    // 2. We know for sure that `alloc` and `table_layout` matches the [`Allocator`]
                    //    and [`TableLayout`] that were used to allocate this table.
                    // 3. If any elements' drop function panics, then there will only be a memory leak,
                    //    because we have replaced the inner table with a new one.
                    </span>old_inner.drop_inner_table::&lt;T, <span class="kw">_</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>.alloc, <span class="self">Self</span>::TABLE_LAYOUT);
                }
            } <span class="kw">else </span>{
                <span class="comment">// Avoid `Result::unwrap_or_else` because it bloats LLVM IR.
                </span><span class="kw">unsafe </span>{
                    <span class="comment">// SAFETY:
                    // 1. We know for sure that `min_size &gt;= self.table.items`.
                    // 2. The [`RawTableInner`] must already have properly initialized control bytes since
                    //    we will never expose RawTable::new_uninitialized in a public API.
                    </span><span class="kw">if </span><span class="self">self
                        </span>.resize(min_size, hasher, Fallibility::Infallible)
                        .is_err()
                    {
                        <span class="comment">// SAFETY: The result of calling the `resize` function cannot be an error
                        // because `fallibility == Fallibility::Infallible.
                        </span>hint::unreachable_unchecked()
                    }
                }
            }
        }
    }

    <span class="doccomment">/// Ensures that at least `additional` items can be inserted into the table
    /// without reallocation.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>reserve(<span class="kw-2">&amp;mut </span><span class="self">self</span>, additional: usize, hasher: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; u64) {
        <span class="kw">if </span>unlikely(additional &gt; <span class="self">self</span>.table.growth_left) {
            <span class="comment">// Avoid `Result::unwrap_or_else` because it bloats LLVM IR.
            </span><span class="kw">unsafe </span>{
                <span class="comment">// SAFETY: The [`RawTableInner`] must already have properly initialized control
                // bytes since we will never expose RawTable::new_uninitialized in a public API.
                </span><span class="kw">if </span><span class="self">self
                    </span>.reserve_rehash(additional, hasher, Fallibility::Infallible)
                    .is_err()
                {
                    <span class="comment">// SAFETY: All allocation errors will be caught inside `RawTableInner::reserve_rehash`.
                    </span>hint::unreachable_unchecked()
                }
            }
        }
    }

    <span class="doccomment">/// Tries to ensure that at least `additional` items can be inserted into
    /// the table without reallocation.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>try_reserve(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        additional: usize,
        hasher: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; u64,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), TryReserveError&gt; {
        <span class="kw">if </span>additional &gt; <span class="self">self</span>.table.growth_left {
            <span class="comment">// SAFETY: The [`RawTableInner`] must already have properly initialized control
            // bytes since we will never expose RawTable::new_uninitialized in a public API.
            </span><span class="kw">unsafe </span>{ <span class="self">self</span>.reserve_rehash(additional, hasher, Fallibility::Fallible) }
        } <span class="kw">else </span>{
            <span class="prelude-val">Ok</span>(())
        }
    }

    <span class="doccomment">/// Out-of-line slow path for `reserve` and `try_reserve`.
    ///
    /// # Safety
    ///
    /// The [`RawTableInner`] must have properly initialized control bytes,
    /// otherwise calling this function results in [`undefined behavior`]
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[cold]
    #[inline(never)]
    </span><span class="kw">unsafe fn </span>reserve_rehash(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        additional: usize,
        hasher: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; u64,
        fallibility: Fallibility,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), TryReserveError&gt; {
        <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY:
            // 1. We know for sure that `alloc` and `layout` matches the [`Allocator`] and
            //    [`TableLayout`] that were used to allocate this table.
            // 2. The `drop` function is the actual drop function of the elements stored in
            //    the table.
            // 3. The caller ensures that the control bytes of the `RawTableInner`
            //    are already initialized.
            </span><span class="self">self</span>.table.reserve_rehash_inner(
                <span class="kw-2">&amp;</span><span class="self">self</span>.alloc,
                additional,
                <span class="kw-2">&amp;</span>|table, index| hasher(table.bucket::&lt;T&gt;(index).as_ref()),
                fallibility,
                <span class="self">Self</span>::TABLE_LAYOUT,
                <span class="kw">if </span>T::NEEDS_DROP {
                    <span class="prelude-val">Some</span>(mem::transmute(ptr::drop_in_place::&lt;T&gt; <span class="kw">as unsafe fn</span>(<span class="kw-2">*mut </span>T)))
                } <span class="kw">else </span>{
                    <span class="prelude-val">None
                </span>},
            )
        }
    }

    <span class="doccomment">/// Allocates a new table of a different size and moves the contents of the
    /// current table into it.
    ///
    /// # Safety
    ///
    /// The [`RawTableInner`] must have properly initialized control bytes,
    /// otherwise calling this function results in [`undefined behavior`]
    ///
    /// The caller of this function must ensure that `capacity &gt;= self.table.items`
    /// otherwise:
    ///
    /// * If `self.table.items != 0`, calling of this function with `capacity`
    ///   equal to 0 (`capacity == 0`) results in [`undefined behavior`].
    ///
    /// * If `capacity_to_buckets(capacity) &lt; Group::WIDTH` and
    ///   `self.table.items &gt; capacity_to_buckets(capacity)`
    ///   calling this function results in [`undefined behavior`].
    ///
    /// * If `capacity_to_buckets(capacity) &gt;= Group::WIDTH` and
    ///   `self.table.items &gt; capacity_to_buckets(capacity)`
    ///   calling this function are never return (will go into an
    ///   infinite loop).
    ///
    /// See [`RawTableInner::find_insert_slot`] for more information.
    ///
    /// [`RawTableInner::find_insert_slot`]: RawTableInner::find_insert_slot
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="kw">unsafe fn </span>resize(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        capacity: usize,
        hasher: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; u64,
        fallibility: Fallibility,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), TryReserveError&gt; {
        <span class="comment">// SAFETY:
        // 1. The caller of this function guarantees that `capacity &gt;= self.table.items`.
        // 2. We know for sure that `alloc` and `layout` matches the [`Allocator`] and
        //    [`TableLayout`] that were used to allocate this table.
        // 3. The caller ensures that the control bytes of the `RawTableInner`
        //    are already initialized.
        </span><span class="self">self</span>.table.resize_inner(
            <span class="kw-2">&amp;</span><span class="self">self</span>.alloc,
            capacity,
            <span class="kw-2">&amp;</span>|table, index| hasher(table.bucket::&lt;T&gt;(index).as_ref()),
            fallibility,
            <span class="self">Self</span>::TABLE_LAYOUT,
        )
    }

    <span class="doccomment">/// Inserts a new element into the table, and returns its raw bucket.
    ///
    /// This does not check if the given element already exists in the table.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>insert(<span class="kw-2">&amp;mut </span><span class="self">self</span>, hash: u64, value: T, hasher: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; u64) -&gt; Bucket&lt;T&gt; {
        <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY:
            // 1. The [`RawTableInner`] must already have properly initialized control bytes since
            //    we will never expose `RawTable::new_uninitialized` in a public API.
            //
            // 2. We reserve additional space (if necessary) right after calling this function.
            </span><span class="kw">let </span><span class="kw-2">mut </span>slot = <span class="self">self</span>.table.find_insert_slot(hash);

            <span class="comment">// We can avoid growing the table once we have reached our load factor if we are replacing
            // a tombstone. This works since the number of EMPTY slots does not change in this case.
            //
            // SAFETY: The function is guaranteed to return [`InsertSlot`] that contains an index
            // in the range `0..=self.buckets()`.
            </span><span class="kw">let </span>old_ctrl = <span class="kw-2">*</span><span class="self">self</span>.table.ctrl(slot.index);
            <span class="kw">if </span>unlikely(<span class="self">self</span>.table.growth_left == <span class="number">0 </span>&amp;&amp; special_is_empty(old_ctrl)) {
                <span class="self">self</span>.reserve(<span class="number">1</span>, hasher);
                <span class="comment">// SAFETY: We know for sure that `RawTableInner` has control bytes
                // initialized and that there is extra space in the table.
                </span>slot = <span class="self">self</span>.table.find_insert_slot(hash);
            }

            <span class="self">self</span>.insert_in_slot(hash, slot, value)
        }
    }

    <span class="doccomment">/// Attempts to insert a new element without growing the table and return its raw bucket.
    ///
    /// Returns an `Err` containing the given element if inserting it would require growing the
    /// table.
    ///
    /// This does not check if the given element already exists in the table.
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    #[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>try_insert_no_grow(<span class="kw-2">&amp;mut </span><span class="self">self</span>, hash: u64, value: T) -&gt; <span class="prelude-ty">Result</span>&lt;Bucket&lt;T&gt;, T&gt; {
        <span class="kw">unsafe </span>{
            <span class="kw">match </span><span class="self">self</span>.table.prepare_insert_no_grow(hash) {
                <span class="prelude-val">Ok</span>(index) =&gt; {
                    <span class="kw">let </span>bucket = <span class="self">self</span>.bucket(index);
                    bucket.write(value);
                    <span class="prelude-val">Ok</span>(bucket)
                }
                <span class="prelude-val">Err</span>(()) =&gt; <span class="prelude-val">Err</span>(value),
            }
        }
    }

    <span class="doccomment">/// Inserts a new element into the table, and returns a mutable reference to it.
    ///
    /// This does not check if the given element already exists in the table.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>insert_entry(<span class="kw-2">&amp;mut </span><span class="self">self</span>, hash: u64, value: T, hasher: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; u64) -&gt; <span class="kw-2">&amp;mut </span>T {
        <span class="kw">unsafe </span>{ <span class="self">self</span>.insert(hash, value, hasher).as_mut() }
    }

    <span class="doccomment">/// Inserts a new element into the table, without growing the table.
    ///
    /// There must be enough space in the table to insert the new element.
    ///
    /// This does not check if the given element already exists in the table.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    #[cfg(any(feature = <span class="string">"raw"</span>, feature = <span class="string">"rustc-internal-api"</span>))]
    </span><span class="kw">pub unsafe fn </span>insert_no_grow(<span class="kw-2">&amp;mut </span><span class="self">self</span>, hash: u64, value: T) -&gt; Bucket&lt;T&gt; {
        <span class="kw">let </span>(index, old_ctrl) = <span class="self">self</span>.table.prepare_insert_slot(hash);
        <span class="kw">let </span>bucket = <span class="self">self</span>.table.bucket(index);

        <span class="comment">// If we are replacing a DELETED entry then we don't need to update
        // the load counter.
        </span><span class="self">self</span>.table.growth_left -= special_is_empty(old_ctrl) <span class="kw">as </span>usize;

        bucket.write(value);
        <span class="self">self</span>.table.items += <span class="number">1</span>;
        bucket
    }

    <span class="doccomment">/// Temporary removes a bucket, applying the given function to the removed
    /// element and optionally put back the returned value in the same bucket.
    ///
    /// Returns `true` if the bucket still contains an element
    ///
    /// This does not check if the given bucket is actually occupied.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub unsafe fn </span>replace_bucket_with&lt;F&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, bucket: Bucket&lt;T&gt;, f: F) -&gt; bool
    <span class="kw">where
        </span>F: FnOnce(T) -&gt; <span class="prelude-ty">Option</span>&lt;T&gt;,
    {
        <span class="kw">let </span>index = <span class="self">self</span>.bucket_index(<span class="kw-2">&amp;</span>bucket);
        <span class="kw">let </span>old_ctrl = <span class="kw-2">*</span><span class="self">self</span>.table.ctrl(index);
        <span class="macro">debug_assert!</span>(<span class="self">self</span>.is_bucket_full(index));
        <span class="kw">let </span>old_growth_left = <span class="self">self</span>.table.growth_left;
        <span class="kw">let </span>item = <span class="self">self</span>.remove(bucket).<span class="number">0</span>;
        <span class="kw">if let </span><span class="prelude-val">Some</span>(new_item) = f(item) {
            <span class="self">self</span>.table.growth_left = old_growth_left;
            <span class="self">self</span>.table.set_ctrl(index, old_ctrl);
            <span class="self">self</span>.table.items += <span class="number">1</span>;
            <span class="self">self</span>.bucket(index).write(new_item);
            <span class="bool-val">true
        </span>} <span class="kw">else </span>{
            <span class="bool-val">false
        </span>}
    }

    <span class="doccomment">/// Searches for an element in the table. If the element is not found,
    /// returns `Err` with the position of a slot where an element with the
    /// same hash could be inserted.
    ///
    /// This function may resize the table if additional space is required for
    /// inserting an element.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>find_or_find_insert_slot(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        hash: u64,
        <span class="kw-2">mut </span>eq: <span class="kw">impl </span>FnMut(<span class="kw-2">&amp;</span>T) -&gt; bool,
        hasher: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; u64,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;Bucket&lt;T&gt;, InsertSlot&gt; {
        <span class="self">self</span>.reserve(<span class="number">1</span>, hasher);

        <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY:
            // 1. We know for sure that there is at least one empty `bucket` in the table.
            // 2. The [`RawTableInner`] must already have properly initialized control bytes since we will
            //    never expose `RawTable::new_uninitialized` in a public API.
            // 3. The `find_or_find_insert_slot_inner` function returns the `index` of only the full bucket,
            //    which is in the range `0..self.buckets()` (since there is at least one empty `bucket` in
            //    the table), so calling `self.bucket(index)` and `Bucket::as_ref` is safe.
            </span><span class="kw">match </span><span class="self">self
                </span>.table
                .find_or_find_insert_slot_inner(hash, <span class="kw-2">&amp;mut </span>|index| eq(<span class="self">self</span>.bucket(index).as_ref()))
            {
                <span class="comment">// SAFETY: See explanation above.
                </span><span class="prelude-val">Ok</span>(index) =&gt; <span class="prelude-val">Ok</span>(<span class="self">self</span>.bucket(index)),
                <span class="prelude-val">Err</span>(slot) =&gt; <span class="prelude-val">Err</span>(slot),
            }
        }
    }

    <span class="doccomment">/// Inserts a new element into the table in the given slot, and returns its
    /// raw bucket.
    ///
    /// # Safety
    ///
    /// `slot` must point to a slot previously returned by
    /// `find_or_find_insert_slot`, and no mutation of the table must have
    /// occurred since that call.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub unsafe fn </span>insert_in_slot(<span class="kw-2">&amp;mut </span><span class="self">self</span>, hash: u64, slot: InsertSlot, value: T) -&gt; Bucket&lt;T&gt; {
        <span class="kw">let </span>old_ctrl = <span class="kw-2">*</span><span class="self">self</span>.table.ctrl(slot.index);
        <span class="self">self</span>.table.record_item_insert_at(slot.index, old_ctrl, hash);

        <span class="kw">let </span>bucket = <span class="self">self</span>.bucket(slot.index);
        bucket.write(value);
        bucket
    }

    <span class="doccomment">/// Searches for an element in the table.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>find(<span class="kw-2">&amp;</span><span class="self">self</span>, hash: u64, <span class="kw-2">mut </span>eq: <span class="kw">impl </span>FnMut(<span class="kw-2">&amp;</span>T) -&gt; bool) -&gt; <span class="prelude-ty">Option</span>&lt;Bucket&lt;T&gt;&gt; {
        <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY:
            // 1. The [`RawTableInner`] must already have properly initialized control bytes since we
            //    will never expose `RawTable::new_uninitialized` in a public API.
            // 1. The `find_inner` function returns the `index` of only the full bucket, which is in
            //    the range `0..self.buckets()`, so calling `self.bucket(index)` and `Bucket::as_ref`
            //    is safe.
            </span><span class="kw">let </span>result = <span class="self">self
                </span>.table
                .find_inner(hash, <span class="kw-2">&amp;mut </span>|index| eq(<span class="self">self</span>.bucket(index).as_ref()));

            <span class="comment">// Avoid `Option::map` because it bloats LLVM IR.
            </span><span class="kw">match </span>result {
                <span class="comment">// SAFETY: See explanation above.
                </span><span class="prelude-val">Some</span>(index) =&gt; <span class="prelude-val">Some</span>(<span class="self">self</span>.bucket(index)),
                <span class="prelude-val">None </span>=&gt; <span class="prelude-val">None</span>,
            }
        }
    }

    <span class="doccomment">/// Gets a reference to an element in the table.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>get(<span class="kw-2">&amp;</span><span class="self">self</span>, hash: u64, eq: <span class="kw">impl </span>FnMut(<span class="kw-2">&amp;</span>T) -&gt; bool) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>T&gt; {
        <span class="comment">// Avoid `Option::map` because it bloats LLVM IR.
        </span><span class="kw">match </span><span class="self">self</span>.find(hash, eq) {
            <span class="prelude-val">Some</span>(bucket) =&gt; <span class="prelude-val">Some</span>(<span class="kw">unsafe </span>{ bucket.as_ref() }),
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">None</span>,
        }
    }

    <span class="doccomment">/// Gets a mutable reference to an element in the table.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>get_mut(<span class="kw-2">&amp;mut </span><span class="self">self</span>, hash: u64, eq: <span class="kw">impl </span>FnMut(<span class="kw-2">&amp;</span>T) -&gt; bool) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;mut </span>T&gt; {
        <span class="comment">// Avoid `Option::map` because it bloats LLVM IR.
        </span><span class="kw">match </span><span class="self">self</span>.find(hash, eq) {
            <span class="prelude-val">Some</span>(bucket) =&gt; <span class="prelude-val">Some</span>(<span class="kw">unsafe </span>{ bucket.as_mut() }),
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">None</span>,
        }
    }

    <span class="doccomment">/// Attempts to get mutable references to `N` entries in the table at once.
    ///
    /// Returns an array of length `N` with the results of each query.
    ///
    /// At most one mutable reference will be returned to any entry. `None` will be returned if any
    /// of the hashes are duplicates. `None` will be returned if the hash is not found.
    ///
    /// The `eq` argument should be a closure such that `eq(i, k)` returns true if `k` is equal to
    /// the `i`th key to be looked up.
    </span><span class="kw">pub fn </span>get_many_mut&lt;<span class="kw">const </span>N: usize&gt;(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        hashes: [u64; N],
        eq: <span class="kw">impl </span>FnMut(usize, <span class="kw-2">&amp;</span>T) -&gt; bool,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;[<span class="kw-2">&amp;</span><span class="lifetime">'_ </span><span class="kw-2">mut </span>T; N]&gt; {
        <span class="kw">unsafe </span>{
            <span class="kw">let </span>ptrs = <span class="self">self</span>.get_many_mut_pointers(hashes, eq)<span class="question-mark">?</span>;

            <span class="kw">for </span>(i, <span class="kw-2">&amp;</span>cur) <span class="kw">in </span>ptrs.iter().enumerate() {
                <span class="kw">if </span>ptrs[..i].iter().any(|<span class="kw-2">&amp;</span>prev| ptr::eq::&lt;T&gt;(prev, cur)) {
                    <span class="kw">return </span><span class="prelude-val">None</span>;
                }
            }
            <span class="comment">// All bucket are distinct from all previous buckets so we're clear to return the result
            // of the lookup.

            // TODO use `MaybeUninit::array_assume_init` here instead once that's stable.
            </span><span class="prelude-val">Some</span>(mem::transmute_copy(<span class="kw-2">&amp;</span>ptrs))
        }
    }

    <span class="kw">pub unsafe fn </span>get_many_unchecked_mut&lt;<span class="kw">const </span>N: usize&gt;(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        hashes: [u64; N],
        eq: <span class="kw">impl </span>FnMut(usize, <span class="kw-2">&amp;</span>T) -&gt; bool,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;[<span class="kw-2">&amp;</span><span class="lifetime">'_ </span><span class="kw-2">mut </span>T; N]&gt; {
        <span class="kw">let </span>ptrs = <span class="self">self</span>.get_many_mut_pointers(hashes, eq)<span class="question-mark">?</span>;
        <span class="prelude-val">Some</span>(mem::transmute_copy(<span class="kw-2">&amp;</span>ptrs))
    }

    <span class="kw">unsafe fn </span>get_many_mut_pointers&lt;<span class="kw">const </span>N: usize&gt;(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        hashes: [u64; N],
        <span class="kw-2">mut </span>eq: <span class="kw">impl </span>FnMut(usize, <span class="kw-2">&amp;</span>T) -&gt; bool,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;[<span class="kw-2">*mut </span>T; N]&gt; {
        <span class="comment">// TODO use `MaybeUninit::uninit_array` here instead once that's stable.
        </span><span class="kw">let </span><span class="kw-2">mut </span>outs: MaybeUninit&lt;[<span class="kw-2">*mut </span>T; N]&gt; = MaybeUninit::uninit();
        <span class="kw">let </span>outs_ptr = outs.as_mut_ptr();

        <span class="kw">for </span>(i, <span class="kw-2">&amp;</span>hash) <span class="kw">in </span>hashes.iter().enumerate() {
            <span class="kw">let </span>cur = <span class="self">self</span>.find(hash, |k| eq(i, k))<span class="question-mark">?</span>;
            <span class="kw-2">*</span>(<span class="kw-2">*</span>outs_ptr).get_unchecked_mut(i) = cur.as_mut();
        }

        <span class="comment">// TODO use `MaybeUninit::array_assume_init` here instead once that's stable.
        </span><span class="prelude-val">Some</span>(outs.assume_init())
    }

    <span class="doccomment">/// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the table might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>capacity(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.table.items + <span class="self">self</span>.table.growth_left
    }

    <span class="doccomment">/// Returns the number of elements in the table.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.table.items
    }

    <span class="doccomment">/// Returns `true` if the table contains no elements.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_empty(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.len() == <span class="number">0
    </span>}

    <span class="doccomment">/// Returns the number of buckets in the table.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>buckets(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.table.bucket_mask + <span class="number">1
    </span>}

    <span class="doccomment">/// Checks whether the bucket at `index` is full.
    ///
    /// # Safety
    ///
    /// The caller must ensure `index` is less than the number of buckets.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub unsafe fn </span>is_bucket_full(<span class="kw-2">&amp;</span><span class="self">self</span>, index: usize) -&gt; bool {
        <span class="self">self</span>.table.is_bucket_full(index)
    }

    <span class="doccomment">/// Returns an iterator over every element in the table. It is up to
    /// the caller to ensure that the `RawTable` outlives the `RawIter`.
    /// Because we cannot make the `next` method unsafe on the `RawIter`
    /// struct, we have to make the `iter` method unsafe.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub unsafe fn </span>iter(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; RawIter&lt;T&gt; {
        <span class="comment">// SAFETY:
        // 1. The caller must uphold the safety contract for `iter` method.
        // 2. The [`RawTableInner`] must already have properly initialized control bytes since
        //    we will never expose RawTable::new_uninitialized in a public API.
        </span><span class="self">self</span>.table.iter()
    }

    <span class="doccomment">/// Returns an iterator over occupied buckets that could match a given hash.
    ///
    /// `RawTable` only stores 7 bits of the hash value, so this iterator may
    /// return items that have a hash value different than the one provided. You
    /// should always validate the returned values before using them.
    ///
    /// It is up to the caller to ensure that the `RawTable` outlives the
    /// `RawIterHash`. Because we cannot make the `next` method unsafe on the
    /// `RawIterHash` struct, we have to make the `iter_hash` method unsafe.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    #[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">pub unsafe fn </span>iter_hash(<span class="kw-2">&amp;</span><span class="self">self</span>, hash: u64) -&gt; RawIterHash&lt;T&gt; {
        RawIterHash::new(<span class="self">self</span>, hash)
    }

    <span class="doccomment">/// Returns an iterator which removes all elements from the table without
    /// freeing the memory.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>drain(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; RawDrain&lt;<span class="lifetime">'_</span>, T, A&gt; {
        <span class="kw">unsafe </span>{
            <span class="kw">let </span>iter = <span class="self">self</span>.iter();
            <span class="self">self</span>.drain_iter_from(iter)
        }
    }

    <span class="doccomment">/// Returns an iterator which removes all elements from the table without
    /// freeing the memory.
    ///
    /// Iteration starts at the provided iterator's current location.
    ///
    /// It is up to the caller to ensure that the iterator is valid for this
    /// `RawTable` and covers all items that remain in the table.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub unsafe fn </span>drain_iter_from(<span class="kw-2">&amp;mut </span><span class="self">self</span>, iter: RawIter&lt;T&gt;) -&gt; RawDrain&lt;<span class="lifetime">'_</span>, T, A&gt; {
        <span class="macro">debug_assert_eq!</span>(iter.len(), <span class="self">self</span>.len());
        RawDrain {
            iter,
            table: mem::replace(<span class="kw-2">&amp;mut </span><span class="self">self</span>.table, RawTableInner::NEW),
            orig_table: NonNull::from(<span class="kw-2">&amp;mut </span><span class="self">self</span>.table),
            marker: PhantomData,
        }
    }

    <span class="doccomment">/// Returns an iterator which consumes all elements from the table.
    ///
    /// Iteration starts at the provided iterator's current location.
    ///
    /// It is up to the caller to ensure that the iterator is valid for this
    /// `RawTable` and covers all items that remain in the table.
    </span><span class="kw">pub unsafe fn </span>into_iter_from(<span class="self">self</span>, iter: RawIter&lt;T&gt;) -&gt; RawIntoIter&lt;T, A&gt; {
        <span class="macro">debug_assert_eq!</span>(iter.len(), <span class="self">self</span>.len());

        <span class="kw">let </span>allocation = <span class="self">self</span>.into_allocation();
        RawIntoIter {
            iter,
            allocation,
            marker: PhantomData,
        }
    }

    <span class="doccomment">/// Converts the table into a raw allocation. The contents of the table
    /// should be dropped using a `RawIter` before freeing the allocation.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>into_allocation(<span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;(NonNull&lt;u8&gt;, Layout, A)&gt; {
        <span class="kw">let </span>alloc = <span class="kw">if </span><span class="self">self</span>.table.is_empty_singleton() {
            <span class="prelude-val">None
        </span>} <span class="kw">else </span>{
            <span class="comment">// Avoid `Option::unwrap_or_else` because it bloats LLVM IR.
            </span><span class="kw">let </span>(layout, ctrl_offset) =
                <span class="kw">match </span><span class="self">Self</span>::TABLE_LAYOUT.calculate_layout_for(<span class="self">self</span>.table.buckets()) {
                    <span class="prelude-val">Some</span>(lco) =&gt; lco,
                    <span class="prelude-val">None </span>=&gt; <span class="kw">unsafe </span>{ hint::unreachable_unchecked() },
                };
            <span class="prelude-val">Some</span>((
                <span class="kw">unsafe </span>{ NonNull::new_unchecked(<span class="self">self</span>.table.ctrl.as_ptr().sub(ctrl_offset)) },
                layout,
                <span class="kw">unsafe </span>{ ptr::read(<span class="kw-2">&amp;</span><span class="self">self</span>.alloc) },
            ))
        };
        mem::forget(<span class="self">self</span>);
        alloc
    }
}

<span class="kw">unsafe impl</span>&lt;T, A: Allocator&gt; Send <span class="kw">for </span>RawTable&lt;T, A&gt;
<span class="kw">where
    </span>T: Send,
    A: Send,
{
}
<span class="kw">unsafe impl</span>&lt;T, A: Allocator&gt; Sync <span class="kw">for </span>RawTable&lt;T, A&gt;
<span class="kw">where
    </span>T: Sync,
    A: Sync,
{
}

<span class="kw">impl </span>RawTableInner {
    <span class="kw">const </span>NEW: <span class="self">Self </span>= RawTableInner::new();

    <span class="doccomment">/// Creates a new empty hash table without allocating any memory.
    ///
    /// In effect this returns a table with exactly 1 bucket. However we can
    /// leave the data pointer dangling since that bucket is never accessed
    /// due to our load factor forcing us to always have at least 1 free bucket.
    </span><span class="attr">#[inline]
    </span><span class="kw">const fn </span>new() -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            <span class="comment">// Be careful to cast the entire slice to a raw pointer.
            </span>ctrl: <span class="kw">unsafe </span>{ NonNull::new_unchecked(Group::static_empty() <span class="kw">as </span><span class="kw-2">*const </span><span class="kw">_ as </span><span class="kw-2">*mut </span>u8) },
            bucket_mask: <span class="number">0</span>,
            items: <span class="number">0</span>,
            growth_left: <span class="number">0</span>,
        }
    }
}

<span class="kw">impl </span>RawTableInner {
    <span class="doccomment">/// Allocates a new [`RawTableInner`] with the given number of buckets.
    /// The control bytes and buckets are left uninitialized.
    ///
    /// # Safety
    ///
    /// The caller of this function must ensure that the `buckets` is power of two
    /// and also initialize all control bytes of the length `self.bucket_mask + 1 +
    /// Group::WIDTH` with the [`EMPTY`] bytes.
    ///
    /// See also [`Allocator`] API for other safety concerns.
    ///
    /// [`Allocator`]: https://doc.rust-lang.org/alloc/alloc/trait.Allocator.html
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">unsafe fn </span>new_uninitialized&lt;A&gt;(
        alloc: <span class="kw-2">&amp;</span>A,
        table_layout: TableLayout,
        buckets: usize,
        fallibility: Fallibility,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, TryReserveError&gt;
    <span class="kw">where
        </span>A: Allocator,
    {
        <span class="macro">debug_assert!</span>(buckets.is_power_of_two());

        <span class="comment">// Avoid `Option::ok_or_else` because it bloats LLVM IR.
        </span><span class="kw">let </span>(layout, ctrl_offset) = <span class="kw">match </span>table_layout.calculate_layout_for(buckets) {
            <span class="prelude-val">Some</span>(lco) =&gt; lco,
            <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(fallibility.capacity_overflow()),
        };

        <span class="kw">let </span>ptr: NonNull&lt;u8&gt; = <span class="kw">match </span>do_alloc(alloc, layout) {
            <span class="prelude-val">Ok</span>(block) =&gt; block.cast(),
            <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(fallibility.alloc_err(layout)),
        };

        <span class="comment">// SAFETY: null pointer will be caught in above check
        </span><span class="kw">let </span>ctrl = NonNull::new_unchecked(ptr.as_ptr().add(ctrl_offset));
        <span class="prelude-val">Ok</span>(<span class="self">Self </span>{
            ctrl,
            bucket_mask: buckets - <span class="number">1</span>,
            items: <span class="number">0</span>,
            growth_left: bucket_mask_to_capacity(buckets - <span class="number">1</span>),
        })
    }

    <span class="doccomment">/// Attempts to allocate a new [`RawTableInner`] with at least enough
    /// capacity for inserting the given number of elements without reallocating.
    ///
    /// All the control bytes are initialized with the [`EMPTY`] bytes.
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>fallible_with_capacity&lt;A&gt;(
        alloc: <span class="kw-2">&amp;</span>A,
        table_layout: TableLayout,
        capacity: usize,
        fallibility: Fallibility,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, TryReserveError&gt;
    <span class="kw">where
        </span>A: Allocator,
    {
        <span class="kw">if </span>capacity == <span class="number">0 </span>{
            <span class="prelude-val">Ok</span>(<span class="self">Self</span>::NEW)
        } <span class="kw">else </span>{
            <span class="comment">// SAFETY: We checked that we could successfully allocate the new table, and then
            // initialized all control bytes with the constant `EMPTY` byte.
            </span><span class="kw">unsafe </span>{
                <span class="kw">let </span>buckets =
                    capacity_to_buckets(capacity).ok_or_else(|| fallibility.capacity_overflow())<span class="question-mark">?</span>;

                <span class="kw">let </span>result = <span class="self">Self</span>::new_uninitialized(alloc, table_layout, buckets, fallibility)<span class="question-mark">?</span>;
                <span class="comment">// SAFETY: We checked that the table is allocated and therefore the table already has
                // `self.bucket_mask + 1 + Group::WIDTH` number of control bytes (see TableLayout::calculate_layout_for)
                // so writing `self.num_ctrl_bytes() == bucket_mask + 1 + Group::WIDTH` bytes is safe.
                </span>result.ctrl(<span class="number">0</span>).write_bytes(EMPTY, result.num_ctrl_bytes());

                <span class="prelude-val">Ok</span>(result)
            }
        }
    }

    <span class="doccomment">/// Allocates a new [`RawTableInner`] with at least enough capacity for inserting
    /// the given number of elements without reallocating.
    ///
    /// Panics if the new capacity exceeds [`isize::MAX`] bytes and [`abort`] the program
    /// in case of allocation error. Use [`fallible_with_capacity`] instead if you want to
    /// handle memory allocation failure.
    ///
    /// All the control bytes are initialized with the [`EMPTY`] bytes.
    ///
    /// [`fallible_with_capacity`]: RawTableInner::fallible_with_capacity
    /// [`abort`]: https://doc.rust-lang.org/alloc/alloc/fn.handle_alloc_error.html
    </span><span class="kw">fn </span>with_capacity&lt;A&gt;(alloc: <span class="kw-2">&amp;</span>A, table_layout: TableLayout, capacity: usize) -&gt; <span class="self">Self
    </span><span class="kw">where
        </span>A: Allocator,
    {
        <span class="comment">// Avoid `Result::unwrap_or_else` because it bloats LLVM IR.
        </span><span class="kw">match </span><span class="self">Self</span>::fallible_with_capacity(alloc, table_layout, capacity, Fallibility::Infallible) {
            <span class="prelude-val">Ok</span>(table_inner) =&gt; table_inner,
            <span class="comment">// SAFETY: All allocation errors will be caught inside `RawTableInner::new_uninitialized`.
            </span><span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; <span class="kw">unsafe </span>{ hint::unreachable_unchecked() },
        }
    }

    <span class="doccomment">/// Fixes up an insertion slot returned by the [`RawTableInner::find_insert_slot_in_group`] method.
    ///
    /// In tables smaller than the group width (`self.buckets() &lt; Group::WIDTH`), trailing control
    /// bytes outside the range of the table are filled with [`EMPTY`] entries. These will unfortunately
    /// trigger a match of [`RawTableInner::find_insert_slot_in_group`] function. This is because
    /// the `Some(bit)` returned by `group.match_empty_or_deleted().lowest_set_bit()` after masking
    /// (`(probe_seq.pos + bit) &amp; self.bucket_mask`) may point to a full bucket that is already occupied.
    /// We detect this situation here and perform a second scan starting at the beginning of the table.
    /// This second scan is guaranteed to find an empty slot (due to the load factor) before hitting the
    /// trailing control bytes (containing [`EMPTY`] bytes).
    ///
    /// If this function is called correctly, it is guaranteed to return [`InsertSlot`] with an
    /// index of an empty or deleted bucket in the range `0..self.buckets()` (see `Warning` and
    /// `Safety`).
    ///
    /// # Warning
    ///
    /// The table must have at least 1 empty or deleted `bucket`, otherwise if the table is less than
    /// the group width (`self.buckets() &lt; Group::WIDTH`) this function returns an index outside of the
    /// table indices range `0..self.buckets()` (`0..=self.bucket_mask`). Attempt to write data at that
    /// index will cause immediate [`undefined behavior`].
    ///
    /// # Safety
    ///
    /// The safety rules are directly derived from the safety rules for [`RawTableInner::ctrl`] method.
    /// Thus, in order to uphold those safety contracts, as well as for the correct logic of the work
    /// of this crate, the following rules are necessary and sufficient:
    ///
    /// * The [`RawTableInner`] must have properly initialized control bytes otherwise calling this
    ///   function results in [`undefined behavior`].
    ///
    /// * This function must only be used on insertion slots found by [`RawTableInner::find_insert_slot_in_group`]
    ///   (after the `find_insert_slot_in_group` function, but before insertion into the table).
    ///
    /// * The `index` must not be greater than the `self.bucket_mask`, i.e. `(index + 1) &lt;= self.buckets()`
    ///   (this one is provided by the [`RawTableInner::find_insert_slot_in_group`] function).
    ///
    /// Calling this function with an index not provided by [`RawTableInner::find_insert_slot_in_group`]
    /// may result in [`undefined behavior`] even if the index satisfies the safety rules of the
    /// [`RawTableInner::ctrl`] function (`index &lt; self.bucket_mask + 1 + Group::WIDTH`).
    ///
    /// [`RawTableInner::ctrl`]: RawTableInner::ctrl
    /// [`RawTableInner::find_insert_slot_in_group`]: RawTableInner::find_insert_slot_in_group
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>fix_insert_slot(<span class="kw-2">&amp;</span><span class="self">self</span>, <span class="kw-2">mut </span>index: usize) -&gt; InsertSlot {
        <span class="comment">// SAFETY: The caller of this function ensures that `index` is in the range `0..=self.bucket_mask`.
        </span><span class="kw">if </span>unlikely(<span class="self">self</span>.is_bucket_full(index)) {
            <span class="macro">debug_assert!</span>(<span class="self">self</span>.bucket_mask &lt; Group::WIDTH);
            <span class="comment">// SAFETY:
            //
            // * Since the caller of this function ensures that the control bytes are properly
            //   initialized and `ptr = self.ctrl(0)` points to the start of the array of control
            //   bytes, therefore: `ctrl` is valid for reads, properly aligned to `Group::WIDTH`
            //   and points to the properly initialized control bytes (see also
            //   `TableLayout::calculate_layout_for` and `ptr::read`);
            //
            // * Because the caller of this function ensures that the index was provided by the
            //   `self.find_insert_slot_in_group()` function, so for for tables larger than the
            //   group width (self.buckets() &gt;= Group::WIDTH), we will never end up in the given
            //   branch, since `(probe_seq.pos + bit) &amp; self.bucket_mask` in `find_insert_slot_in_group`
            //   cannot return a full bucket index. For tables smaller than the group width, calling
            //   the `unwrap_unchecked` function is also safe, as the trailing control bytes outside
            //   the range of the table are filled with EMPTY bytes (and we know for sure that there
            //   is at least one FULL bucket), so this second scan either finds an empty slot (due to
            //   the load factor) or hits the trailing control bytes (containing EMPTY).
            </span>index = Group::load_aligned(<span class="self">self</span>.ctrl(<span class="number">0</span>))
                .match_empty_or_deleted()
                .lowest_set_bit()
                .unwrap_unchecked();
        }
        InsertSlot { index }
    }

    <span class="doccomment">/// Finds the position to insert something in a group.
    ///
    /// **This may have false positives and must be fixed up with `fix_insert_slot`
    /// before it's used.**
    ///
    /// The function is guaranteed to return the index of an empty or deleted [`Bucket`]
    /// in the range `0..self.buckets()` (`0..=self.bucket_mask`).
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>find_insert_slot_in_group(<span class="kw-2">&amp;</span><span class="self">self</span>, group: <span class="kw-2">&amp;</span>Group, probe_seq: <span class="kw-2">&amp;</span>ProbeSeq) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="kw">let </span>bit = group.match_empty_or_deleted().lowest_set_bit();

        <span class="kw">if </span>likely(bit.is_some()) {
            <span class="comment">// This is the same as `(probe_seq.pos + bit) % self.buckets()` because the number
            // of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
            </span><span class="prelude-val">Some</span>((probe_seq.pos + bit.unwrap()) &amp; <span class="self">self</span>.bucket_mask)
        } <span class="kw">else </span>{
            <span class="prelude-val">None
        </span>}
    }

    <span class="doccomment">/// Searches for an element in the table, or a potential slot where that element could
    /// be inserted (an empty or deleted [`Bucket`] index).
    ///
    /// This uses dynamic dispatch to reduce the amount of code generated, but that is
    /// eliminated by LLVM optimizations.
    ///
    /// This function does not make any changes to the `data` part of the table, or any
    /// changes to the `items` or `growth_left` field of the table.
    ///
    /// The table must have at least 1 empty or deleted `bucket`, otherwise, if the
    /// `eq: &amp;mut dyn FnMut(usize) -&gt; bool` function does not return `true`, this function
    /// will never return (will go into an infinite loop) for tables larger than the group
    /// width, or return an index outside of the table indices range if the table is less
    /// than the group width.
    ///
    /// This function is guaranteed to provide the `eq: &amp;mut dyn FnMut(usize) -&gt; bool`
    /// function with only `FULL` buckets' indices and return the `index` of the found
    /// element (as `Ok(index)`). If the element is not found and there is at least 1
    /// empty or deleted [`Bucket`] in the table, the function is guaranteed to return
    /// [InsertSlot] with an index in the range `0..self.buckets()`, but in any case,
    /// if this function returns [`InsertSlot`], it will contain an index in the range
    /// `0..=self.buckets()`.
    ///
    /// # Safety
    ///
    /// The [`RawTableInner`] must have properly initialized control bytes otherwise calling
    /// this function results in [`undefined behavior`].
    ///
    /// Attempt to write data at the [`InsertSlot`] returned by this function when the table is
    /// less than the group width and if there was not at least one empty or deleted bucket in
    /// the table will cause immediate [`undefined behavior`]. This is because in this case the
    /// function will return `self.bucket_mask + 1` as an index due to the trailing [`EMPTY]
    /// control bytes outside the table range.
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>find_or_find_insert_slot_inner(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        hash: u64,
        eq: <span class="kw-2">&amp;mut </span><span class="kw">dyn </span>FnMut(usize) -&gt; bool,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;usize, InsertSlot&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>insert_slot = <span class="prelude-val">None</span>;

        <span class="kw">let </span>h2_hash = h2(hash);
        <span class="kw">let </span><span class="kw-2">mut </span>probe_seq = <span class="self">self</span>.probe_seq(hash);

        <span class="kw">loop </span>{
            <span class="comment">// SAFETY:
            // * Caller of this function ensures that the control bytes are properly initialized.
            //
            // * `ProbeSeq.pos` cannot be greater than `self.bucket_mask = self.buckets() - 1`
            //   of the table due to masking with `self.bucket_mask` and also because mumber of
            //   buckets is a power of two (see `self.probe_seq` function).
            //
            // * Even if `ProbeSeq.pos` returns `position == self.bucket_mask`, it is safe to
            //   call `Group::load` due to the extended control bytes range, which is
            //  `self.bucket_mask + 1 + Group::WIDTH` (in fact, this means that the last control
            //   byte will never be read for the allocated table);
            //
            // * Also, even if `RawTableInner` is not already allocated, `ProbeSeq.pos` will
            //   always return "0" (zero), so Group::load will read unaligned `Group::static_empty()`
            //   bytes, which is safe (see RawTableInner::new).
            </span><span class="kw">let </span>group = <span class="kw">unsafe </span>{ Group::load(<span class="self">self</span>.ctrl(probe_seq.pos)) };

            <span class="kw">for </span>bit <span class="kw">in </span>group.match_byte(h2_hash) {
                <span class="kw">let </span>index = (probe_seq.pos + bit) &amp; <span class="self">self</span>.bucket_mask;

                <span class="kw">if </span>likely(eq(index)) {
                    <span class="kw">return </span><span class="prelude-val">Ok</span>(index);
                }
            }

            <span class="comment">// We didn't find the element we were looking for in the group, try to get an
            // insertion slot from the group if we don't have one yet.
            </span><span class="kw">if </span>likely(insert_slot.is_none()) {
                insert_slot = <span class="self">self</span>.find_insert_slot_in_group(<span class="kw-2">&amp;</span>group, <span class="kw-2">&amp;</span>probe_seq);
            }

            <span class="comment">// Only stop the search if the group contains at least one empty element.
            // Otherwise, the element that we are looking for might be in a following group.
            </span><span class="kw">if </span>likely(group.match_empty().any_bit_set()) {
                <span class="comment">// We must have found a insert slot by now, since the current group contains at
                // least one. For tables smaller than the group width, there will still be an
                // empty element in the current (and only) group due to the load factor.
                </span><span class="kw">unsafe </span>{
                    <span class="comment">// SAFETY:
                    // * Caller of this function ensures that the control bytes are properly initialized.
                    //
                    // * We use this function with the slot / index found by `self.find_insert_slot_in_group`
                    </span><span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.fix_insert_slot(insert_slot.unwrap_unchecked()));
                }
            }

            probe_seq.move_next(<span class="self">self</span>.bucket_mask);
        }
    }

    <span class="doccomment">/// Searches for an empty or deleted bucket which is suitable for inserting a new
    /// element and sets the hash for that slot. Returns an index of that slot and the
    /// old control byte stored in the found index.
    ///
    /// This function does not check if the given element exists in the table. Also,
    /// this function does not check if there is enough space in the table to insert
    /// a new element. Caller of the funtion must make ensure that the table has at
    /// least 1 empty or deleted `bucket`, otherwise this function will never return
    /// (will go into an infinite loop) for tables larger than the group width, or
    /// return an index outside of the table indices range if the table is less than
    /// the group width.
    ///
    /// If there is at least 1 empty or deleted `bucket` in the table, the function is
    /// guaranteed to return an `index` in the range `0..self.buckets()`, but in any case,
    /// if this function returns an `index` it will be in the range `0..=self.buckets()`.
    ///
    /// This function does not make any changes to the `data` parts of the table,
    /// or any changes to the the `items` or `growth_left` field of the table.
    ///
    /// # Safety
    ///
    /// The safety rules are directly derived from the safety rules for the
    /// [`RawTableInner::set_ctrl_h2`] and [`RawTableInner::find_insert_slot`] methods.
    /// Thus, in order to uphold the safety contracts for that methods, as well as for
    /// the correct logic of the work of this crate, you must observe the following rules
    /// when calling this function:
    ///
    /// * The [`RawTableInner`] has already been allocated and has properly initialized
    ///   control bytes otherwise calling this function results in [`undefined behavior`].
    ///
    /// * The caller of this function must ensure that the "data" parts of the table
    ///   will have an entry in the returned index (matching the given hash) right
    ///   after calling this function.
    ///
    /// Attempt to write data at the `index` returned by this function when the table is
    /// less than the group width and if there was not at least one empty or deleted bucket in
    /// the table will cause immediate [`undefined behavior`]. This is because in this case the
    /// function will return `self.bucket_mask + 1` as an index due to the trailing [`EMPTY]
    /// control bytes outside the table range.
    ///
    /// The caller must independently increase the `items` field of the table, and also,
    /// if the old control byte was [`EMPTY`], then decrease the table's `growth_left`
    /// field, and do not change it if the old control byte was [`DELETED`].
    ///
    /// See also [`Bucket::as_ptr`] method, for more information about of properly removing
    /// or saving `element` from / into the [`RawTable`] / [`RawTableInner`].
    ///
    /// [`Bucket::as_ptr`]: Bucket::as_ptr
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    /// [`RawTableInner::ctrl`]: RawTableInner::ctrl
    /// [`RawTableInner::set_ctrl_h2`]: RawTableInner::set_ctrl_h2
    /// [`RawTableInner::find_insert_slot`]: RawTableInner::find_insert_slot
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>prepare_insert_slot(<span class="kw-2">&amp;mut </span><span class="self">self</span>, hash: u64) -&gt; (usize, u8) {
        <span class="comment">// SAFETY: Caller of this function ensures that the control bytes are properly initialized.
        </span><span class="kw">let </span>index: usize = <span class="self">self</span>.find_insert_slot(hash).index;
        <span class="comment">// SAFETY:
        // 1. The `find_insert_slot` function either returns an `index` less than or
        //    equal to `self.buckets() = self.bucket_mask + 1` of the table, or never
        //    returns if it cannot find an empty or deleted slot.
        // 2. The caller of this function guarantees that the table has already been
        //    allocated
        </span><span class="kw">let </span>old_ctrl = <span class="kw-2">*</span><span class="self">self</span>.ctrl(index);
        <span class="self">self</span>.set_ctrl_h2(index, hash);
        (index, old_ctrl)
    }

    <span class="doccomment">/// Searches for an empty or deleted bucket which is suitable for inserting
    /// a new element, returning the `index` for the new [`Bucket`].
    ///
    /// This function does not make any changes to the `data` part of the table, or any
    /// changes to the `items` or `growth_left` field of the table.
    ///
    /// The table must have at least 1 empty or deleted `bucket`, otherwise this function
    /// will never return (will go into an infinite loop) for tables larger than the group
    /// width, or return an index outside of the table indices range if the table is less
    /// than the group width.
    ///
    /// If there is at least 1 empty or deleted `bucket` in the table, the function is
    /// guaranteed to return [`InsertSlot`] with an index in the range `0..self.buckets()`,
    /// but in any case, if this function returns [`InsertSlot`], it will contain an index
    /// in the range `0..=self.buckets()`.
    ///
    /// # Safety
    ///
    /// The [`RawTableInner`] must have properly initialized control bytes otherwise calling
    /// this function results in [`undefined behavior`].
    ///
    /// Attempt to write data at the [`InsertSlot`] returned by this function when the table is
    /// less than the group width and if there was not at least one empty or deleted bucket in
    /// the table will cause immediate [`undefined behavior`]. This is because in this case the
    /// function will return `self.bucket_mask + 1` as an index due to the trailing [`EMPTY]
    /// control bytes outside the table range.
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>find_insert_slot(<span class="kw-2">&amp;</span><span class="self">self</span>, hash: u64) -&gt; InsertSlot {
        <span class="kw">let </span><span class="kw-2">mut </span>probe_seq = <span class="self">self</span>.probe_seq(hash);
        <span class="kw">loop </span>{
            <span class="comment">// SAFETY:
            // * Caller of this function ensures that the control bytes are properly initialized.
            //
            // * `ProbeSeq.pos` cannot be greater than `self.bucket_mask = self.buckets() - 1`
            //   of the table due to masking with `self.bucket_mask` and also because mumber of
            //   buckets is a power of two (see `self.probe_seq` function).
            //
            // * Even if `ProbeSeq.pos` returns `position == self.bucket_mask`, it is safe to
            //   call `Group::load` due to the extended control bytes range, which is
            //  `self.bucket_mask + 1 + Group::WIDTH` (in fact, this means that the last control
            //   byte will never be read for the allocated table);
            //
            // * Also, even if `RawTableInner` is not already allocated, `ProbeSeq.pos` will
            //   always return "0" (zero), so Group::load will read unaligned `Group::static_empty()`
            //   bytes, which is safe (see RawTableInner::new).
            </span><span class="kw">let </span>group = <span class="kw">unsafe </span>{ Group::load(<span class="self">self</span>.ctrl(probe_seq.pos)) };

            <span class="kw">let </span>index = <span class="self">self</span>.find_insert_slot_in_group(<span class="kw-2">&amp;</span>group, <span class="kw-2">&amp;</span>probe_seq);
            <span class="kw">if </span>likely(index.is_some()) {
                <span class="comment">// SAFETY:
                // * Caller of this function ensures that the control bytes are properly initialized.
                //
                // * We use this function with the slot / index found by `self.find_insert_slot_in_group`
                </span><span class="kw">unsafe </span>{
                    <span class="kw">return </span><span class="self">self</span>.fix_insert_slot(index.unwrap_unchecked());
                }
            }
            probe_seq.move_next(<span class="self">self</span>.bucket_mask);
        }
    }

    <span class="doccomment">/// Searches for an element in a table, returning the `index` of the found element.
    /// This uses dynamic dispatch to reduce the amount of code generated, but it is
    /// eliminated by LLVM optimizations.
    ///
    /// This function does not make any changes to the `data` part of the table, or any
    /// changes to the `items` or `growth_left` field of the table.
    ///
    /// The table must have at least 1 empty `bucket`, otherwise, if the
    /// `eq: &amp;mut dyn FnMut(usize) -&gt; bool` function does not return `true`,
    /// this function will also never return (will go into an infinite loop).
    ///
    /// This function is guaranteed to provide the `eq: &amp;mut dyn FnMut(usize) -&gt; bool`
    /// function with only `FULL` buckets' indices and return the `index` of the found
    /// element as `Some(index)`, so the index will always be in the range
    /// `0..self.buckets()`.
    ///
    /// # Safety
    ///
    /// The [`RawTableInner`] must have properly initialized control bytes otherwise calling
    /// this function results in [`undefined behavior`].
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">unsafe fn </span>find_inner(<span class="kw-2">&amp;</span><span class="self">self</span>, hash: u64, eq: <span class="kw-2">&amp;mut </span><span class="kw">dyn </span>FnMut(usize) -&gt; bool) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="kw">let </span>h2_hash = h2(hash);
        <span class="kw">let </span><span class="kw-2">mut </span>probe_seq = <span class="self">self</span>.probe_seq(hash);

        <span class="kw">loop </span>{
            <span class="comment">// SAFETY:
            // * Caller of this function ensures that the control bytes are properly initialized.
            //
            // * `ProbeSeq.pos` cannot be greater than `self.bucket_mask = self.buckets() - 1`
            //   of the table due to masking with `self.bucket_mask`.
            //
            // * Even if `ProbeSeq.pos` returns `position == self.bucket_mask`, it is safe to
            //   call `Group::load` due to the extended control bytes range, which is
            //  `self.bucket_mask + 1 + Group::WIDTH` (in fact, this means that the last control
            //   byte will never be read for the allocated table);
            //
            // * Also, even if `RawTableInner` is not already allocated, `ProbeSeq.pos` will
            //   always return "0" (zero), so Group::load will read unaligned `Group::static_empty()`
            //   bytes, which is safe (see RawTableInner::new_in).
            </span><span class="kw">let </span>group = <span class="kw">unsafe </span>{ Group::load(<span class="self">self</span>.ctrl(probe_seq.pos)) };

            <span class="kw">for </span>bit <span class="kw">in </span>group.match_byte(h2_hash) {
                <span class="comment">// This is the same as `(probe_seq.pos + bit) % self.buckets()` because the number
                // of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
                </span><span class="kw">let </span>index = (probe_seq.pos + bit) &amp; <span class="self">self</span>.bucket_mask;

                <span class="kw">if </span>likely(eq(index)) {
                    <span class="kw">return </span><span class="prelude-val">Some</span>(index);
                }
            }

            <span class="kw">if </span>likely(group.match_empty().any_bit_set()) {
                <span class="kw">return </span><span class="prelude-val">None</span>;
            }

            probe_seq.move_next(<span class="self">self</span>.bucket_mask);
        }
    }

    <span class="doccomment">/// Prepares for rehashing data in place (that is, without allocating new memory).
    /// Converts all full index `control bytes` to `DELETED` and all `DELETED` control
    /// bytes to `EMPTY`, i.e. performs the following conversion:
    ///
    /// - `EMPTY` control bytes   -&gt; `EMPTY`;
    /// - `DELETED` control bytes -&gt; `EMPTY`;
    /// - `FULL` control bytes    -&gt; `DELETED`.
    ///
    /// This function does not make any changes to the `data` parts of the table,
    /// or any changes to the the `items` or `growth_left` field of the table.
    ///
    /// # Safety
    ///
    /// You must observe the following safety rules when calling this function:
    ///
    /// * The [`RawTableInner`] has already been allocated;
    ///
    /// * The caller of this function must convert the `DELETED` bytes back to `FULL`
    ///   bytes when re-inserting them into their ideal position (which was impossible
    ///   to do during the first insert due to tombstones). If the caller does not do
    ///   this, then calling this function may result in a memory leak.
    ///
    /// * The [`RawTableInner`] must have properly initialized control bytes otherwise
    ///   calling this function results in [`undefined behavior`].
    ///
    /// Calling this function on a table that has not been allocated results in
    /// [`undefined behavior`].
    ///
    /// See also [`Bucket::as_ptr`] method, for more information about of properly removing
    /// or saving `data element` from / into the [`RawTable`] / [`RawTableInner`].
    ///
    /// [`Bucket::as_ptr`]: Bucket::as_ptr
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[allow(clippy::mut_mut)]
    #[inline]
    </span><span class="kw">unsafe fn </span>prepare_rehash_in_place(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="comment">// Bulk convert all full control bytes to DELETED, and all DELETED control bytes to EMPTY.
        // This effectively frees up all buckets containing a DELETED entry.
        //
        // SAFETY:
        // 1. `i` is guaranteed to be within bounds since we are iterating from zero to `buckets - 1`;
        // 2. Even if `i` will be `i == self.bucket_mask`, it is safe to call `Group::load_aligned`
        //    due to the extended control bytes range, which is `self.bucket_mask + 1 + Group::WIDTH`;
        // 3. The caller of this function guarantees that [`RawTableInner`] has already been allocated;
        // 4. We can use `Group::load_aligned` and `Group::store_aligned` here since we start from 0
        //    and go to the end with a step equal to `Group::WIDTH` (see TableLayout::calculate_layout_for).
        </span><span class="kw">for </span>i <span class="kw">in </span>(<span class="number">0</span>..<span class="self">self</span>.buckets()).step_by(Group::WIDTH) {
            <span class="kw">let </span>group = Group::load_aligned(<span class="self">self</span>.ctrl(i));
            <span class="kw">let </span>group = group.convert_special_to_empty_and_full_to_deleted();
            group.store_aligned(<span class="self">self</span>.ctrl(i));
        }

        <span class="comment">// Fix up the trailing control bytes. See the comments in set_ctrl
        // for the handling of tables smaller than the group width.
        //
        // SAFETY: The caller of this function guarantees that [`RawTableInner`]
        // has already been allocated
        </span><span class="kw">if </span>unlikely(<span class="self">self</span>.buckets() &lt; Group::WIDTH) {
            <span class="comment">// SAFETY: We have `self.bucket_mask + 1 + Group::WIDTH` number of control bytes,
            // so copying `self.buckets() == self.bucket_mask + 1` bytes with offset equal to
            // `Group::WIDTH` is safe
            </span><span class="self">self</span>.ctrl(<span class="number">0</span>)
                .copy_to(<span class="self">self</span>.ctrl(Group::WIDTH), <span class="self">self</span>.buckets());
        } <span class="kw">else </span>{
            <span class="comment">// SAFETY: We have `self.bucket_mask + 1 + Group::WIDTH` number of
            // control bytes,so copying `Group::WIDTH` bytes with offset equal
            // to `self.buckets() == self.bucket_mask + 1` is safe
            </span><span class="self">self</span>.ctrl(<span class="number">0</span>)
                .copy_to(<span class="self">self</span>.ctrl(<span class="self">self</span>.buckets()), Group::WIDTH);
        }
    }

    <span class="doccomment">/// Returns an iterator over every element in the table.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result
    /// is [`undefined behavior`]:
    ///
    /// * The caller has to ensure that the `RawTableInner` outlives the
    ///   `RawIter`. Because we cannot make the `next` method unsafe on
    ///   the `RawIter` struct, we have to make the `iter` method unsafe.
    ///
    /// * The [`RawTableInner`] must have properly initialized control bytes.
    ///
    /// The type `T` must be the actual type of the elements stored in the table,
    /// otherwise using the returned [`RawIter`] results in [`undefined behavior`].
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>iter&lt;T&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; RawIter&lt;T&gt; {
        <span class="comment">// SAFETY:
        // 1. Since the caller of this function ensures that the control bytes
        //    are properly initialized and `self.data_end()` points to the start
        //    of the array of control bytes, therefore: `ctrl` is valid for reads,
        //    properly aligned to `Group::WIDTH` and points to the properly initialized
        //    control bytes.
        // 2. `data` bucket index in the table is equal to the `ctrl` index (i.e.
        //    equal to zero).
        // 3. We pass the exact value of buckets of the table to the function.
        //
        //                         `ctrl` points here (to the start
        //                         of the first control byte `CT0`)
        //                          ∨
        // [Pad], T_n, ..., T1, T0, |CT0, CT1, ..., CT_n|, CTa_0, CTa_1, ..., CTa_m
        //                           \________  ________/
        //                                    \/
        //       `n = buckets - 1`, i.e. `RawTableInner::buckets() - 1`
        //
        // where: T0...T_n  - our stored data;
        //        CT0...CT_n - control bytes or metadata for `data`.
        //        CTa_0...CTa_m - additional control bytes, where `m = Group::WIDTH - 1` (so that the search
        //                        with loading `Group` bytes from the heap works properly, even if the result
        //                        of `h1(hash) &amp; self.bucket_mask` is equal to `self.bucket_mask`). See also
        //                        `RawTableInner::set_ctrl` function.
        //
        // P.S. `h1(hash) &amp; self.bucket_mask` is the same as `hash as usize % self.buckets()` because the number
        // of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
        </span><span class="kw">let </span>data = Bucket::from_base_index(<span class="self">self</span>.data_end(), <span class="number">0</span>);
        RawIter {
            <span class="comment">// SAFETY: See explanation above
            </span>iter: RawIterRange::new(<span class="self">self</span>.ctrl.as_ptr(), data, <span class="self">self</span>.buckets()),
            items: <span class="self">self</span>.items,
        }
    }

    <span class="doccomment">/// Executes the destructors (if any) of the values stored in the table.
    ///
    /// # Note
    ///
    /// This function does not erase the control bytes of the table and does
    /// not make any changes to the `items` or `growth_left` fields of the
    /// table. If necessary, the caller of this function must manually set
    /// up these table fields, for example using the [`clear_no_drop`] function.
    ///
    /// Be careful during calling this function, because drop function of
    /// the elements can panic, and this can leave table in an inconsistent
    /// state.
    ///
    /// # Safety
    ///
    /// The type `T` must be the actual type of the elements stored in the table,
    /// otherwise calling this function may result in [`undefined behavior`].
    ///
    /// If `T` is a type that should be dropped and **the table is not empty**,
    /// calling this function more than once results in [`undefined behavior`].
    ///
    /// If `T` is not [`Copy`], attempting to use values stored in the table after
    /// calling this function may result in [`undefined behavior`].
    ///
    /// It is safe to call this function on a table that has not been allocated,
    /// on a table with uninitialized control bytes, and on a table with no actual
    /// data but with `Full` control bytes if `self.items == 0`.
    ///
    /// See also [`Bucket::drop`] / [`Bucket::as_ptr`] methods, for more information
    /// about of properly removing or saving `element` from / into the [`RawTable`] /
    /// [`RawTableInner`].
    ///
    /// [`Bucket::drop`]: Bucket::drop
    /// [`Bucket::as_ptr`]: Bucket::as_ptr
    /// [`clear_no_drop`]: RawTableInner::clear_no_drop
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="kw">unsafe fn </span>drop_elements&lt;T&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="comment">// Check that `self.items != 0`. Protects against the possibility
        // of creating an iterator on an table with uninitialized control bytes.
        </span><span class="kw">if </span>T::NEEDS_DROP &amp;&amp; <span class="self">self</span>.items != <span class="number">0 </span>{
            <span class="comment">// SAFETY: We know for sure that RawTableInner will outlive the
            // returned `RawIter` iterator, and the caller of this function
            // must uphold the safety contract for `drop_elements` method.
            </span><span class="kw">for </span>item <span class="kw">in </span><span class="self">self</span>.iter::&lt;T&gt;() {
                <span class="comment">// SAFETY: The caller must uphold the safety contract for
                // `drop_elements` method.
                </span>item.drop();
            }
        }
    }

    <span class="doccomment">/// Executes the destructors (if any) of the values stored in the table and than
    /// deallocates the table.
    ///
    /// # Note
    ///
    /// Calling this function automatically makes invalid (dangling) all instances of
    /// buckets ([`Bucket`]) and makes invalid (dangling) the `ctrl` field of the table.
    ///
    /// This function does not make any changes to the `bucket_mask`, `items` or `growth_left`
    /// fields of the table. If necessary, the caller of this function must manually set
    /// up these table fields.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is [`undefined behavior`]:
    ///
    /// * Calling this function more than once;
    ///
    /// * The type `T` must be the actual type of the elements stored in the table.
    ///
    /// * The `alloc` must be the same [`Allocator`] as the `Allocator` that was used
    ///   to allocate this table.
    ///
    /// * The `table_layout` must be the same [`TableLayout`] as the `TableLayout` that
    ///   was used to allocate this table.
    ///
    /// The caller of this function should pay attention to the possibility of the
    /// elements' drop function panicking, because this:
    ///
    ///    * May leave the table in an inconsistent state;
    ///
    ///    * Memory is never deallocated, so a memory leak may occur.
    ///
    /// Attempt to use the `ctrl` field of the table (dereference) after calling this
    /// function results in [`undefined behavior`].
    ///
    /// It is safe to call this function on a table that has not been allocated,
    /// on a table with uninitialized control bytes, and on a table with no actual
    /// data but with `Full` control bytes if `self.items == 0`.
    ///
    /// See also [`RawTableInner::drop_elements`] or [`RawTableInner::free_buckets`]
    /// for more  information.
    ///
    /// [`RawTableInner::drop_elements`]: RawTableInner::drop_elements
    /// [`RawTableInner::free_buckets`]: RawTableInner::free_buckets
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="kw">unsafe fn </span>drop_inner_table&lt;T, A: Allocator&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, alloc: <span class="kw-2">&amp;</span>A, table_layout: TableLayout) {
        <span class="kw">if </span>!<span class="self">self</span>.is_empty_singleton() {
            <span class="kw">unsafe </span>{
                <span class="comment">// SAFETY: The caller must uphold the safety contract for `drop_inner_table` method.
                </span><span class="self">self</span>.drop_elements::&lt;T&gt;();
                <span class="comment">// SAFETY:
                // 1. We have checked that our table is allocated.
                // 2. The caller must uphold the safety contract for `drop_inner_table` method.
                </span><span class="self">self</span>.free_buckets(alloc, table_layout);
            }
        }
    }

    <span class="doccomment">/// Returns a pointer to an element in the table (convenience for
    /// `Bucket::from_base_index(self.data_end::&lt;T&gt;(), index)`).
    ///
    /// The caller must ensure that the `RawTableInner` outlives the returned [`Bucket&lt;T&gt;`],
    /// otherwise using it may result in [`undefined behavior`].
    ///
    /// # Safety
    ///
    /// If `mem::size_of::&lt;T&gt;() != 0`, then the safety rules are directly derived from the
    /// safety rules of the [`Bucket::from_base_index`] function. Therefore, when calling
    /// this function, the following safety rules must be observed:
    ///
    /// * The table must already be allocated;
    ///
    /// * The `index` must not be greater than the number returned by the [`RawTableInner::buckets`]
    ///   function, i.e. `(index + 1) &lt;= self.buckets()`.
    ///
    /// * The type `T` must be the actual type of the elements stored in the table, otherwise
    ///   using the returned [`Bucket`] may result in [`undefined behavior`].
    ///
    /// It is safe to call this function with index of zero (`index == 0`) on a table that has
    /// not been allocated, but using the returned [`Bucket`] results in [`undefined behavior`].
    ///
    /// If `mem::size_of::&lt;T&gt;() == 0`, then the only requirement is that the `index` must
    /// not be greater than the number returned by the [`RawTable::buckets`] function, i.e.
    /// `(index + 1) &lt;= self.buckets()`.
    ///
    /// ```none
    /// If mem::size_of::&lt;T&gt;() != 0 then return a pointer to the `element` in the `data part` of the table
    /// (we start counting from "0", so that in the expression T[n], the "n" index actually one less than
    /// the "buckets" number of our `RawTableInner`, i.e. "n = RawTableInner::buckets() - 1"):
    ///
    ///           `table.bucket(3).as_ptr()` returns a pointer that points here in the `data`
    ///           part of the `RawTableInner`, i.e. to the start of T3 (see [`Bucket::as_ptr`])
    ///                  |
    ///                  |               `base = table.data_end::&lt;T&gt;()` points here
    ///                  |               (to the start of CT0 or to the end of T0)
    ///                  v                 v
    /// [Pad], T_n, ..., |T3|, T2, T1, T0, |CT0, CT1, CT2, CT3, ..., CT_n, CTa_0, CTa_1, ..., CTa_m
    ///                     ^                                              \__________  __________/
    ///        `table.bucket(3)` returns a pointer that points                        \/
    ///         here in the `data` part of the `RawTableInner`             additional control bytes
    ///         (to the end of T3)                                          `m = Group::WIDTH - 1`
    ///
    /// where: T0...T_n  - our stored data;
    ///        CT0...CT_n - control bytes or metadata for `data`;
    ///        CTa_0...CTa_m - additional control bytes (so that the search with loading `Group` bytes from
    ///                        the heap works properly, even if the result of `h1(hash) &amp; self.bucket_mask`
    ///                        is equal to `self.bucket_mask`). See also `RawTableInner::set_ctrl` function.
    ///
    /// P.S. `h1(hash) &amp; self.bucket_mask` is the same as `hash as usize % self.buckets()` because the number
    /// of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
    /// ```
    ///
    /// [`Bucket::from_base_index`]: Bucket::from_base_index
    /// [`RawTableInner::buckets`]: RawTableInner::buckets
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>bucket&lt;T&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, index: usize) -&gt; Bucket&lt;T&gt; {
        <span class="macro">debug_assert_ne!</span>(<span class="self">self</span>.bucket_mask, <span class="number">0</span>);
        <span class="macro">debug_assert!</span>(index &lt; <span class="self">self</span>.buckets());
        Bucket::from_base_index(<span class="self">self</span>.data_end(), index)
    }

    <span class="doccomment">/// Returns a raw `*mut u8` pointer to the start of the `data` element in the table
    /// (convenience for `self.data_end::&lt;u8&gt;().as_ptr().sub((index + 1) * size_of)`).
    ///
    /// The caller must ensure that the `RawTableInner` outlives the returned `*mut u8`,
    /// otherwise using it may result in [`undefined behavior`].
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is [`undefined behavior`]:
    ///
    /// * The table must already be allocated;
    ///
    /// * The `index` must not be greater than the number returned by the [`RawTableInner::buckets`]
    ///   function, i.e. `(index + 1) &lt;= self.buckets()`;
    ///
    /// * The `size_of` must be equal to the size of the elements stored in the table;
    ///
    /// ```none
    /// If mem::size_of::&lt;T&gt;() != 0 then return a pointer to the `element` in the `data part` of the table
    /// (we start counting from "0", so that in the expression T[n], the "n" index actually one less than
    /// the "buckets" number of our `RawTableInner`, i.e. "n = RawTableInner::buckets() - 1"):
    ///
    ///           `table.bucket_ptr(3, mem::size_of::&lt;T&gt;())` returns a pointer that points here in the
    ///           `data` part of the `RawTableInner`, i.e. to the start of T3
    ///                  |
    ///                  |               `base = table.data_end::&lt;u8&gt;()` points here
    ///                  |               (to the start of CT0 or to the end of T0)
    ///                  v                 v
    /// [Pad], T_n, ..., |T3|, T2, T1, T0, |CT0, CT1, CT2, CT3, ..., CT_n, CTa_0, CTa_1, ..., CTa_m
    ///                                                                    \__________  __________/
    ///                                                                               \/
    ///                                                                    additional control bytes
    ///                                                                     `m = Group::WIDTH - 1`
    ///
    /// where: T0...T_n  - our stored data;
    ///        CT0...CT_n - control bytes or metadata for `data`;
    ///        CTa_0...CTa_m - additional control bytes (so that the search with loading `Group` bytes from
    ///                        the heap works properly, even if the result of `h1(hash) &amp; self.bucket_mask`
    ///                        is equal to `self.bucket_mask`). See also `RawTableInner::set_ctrl` function.
    ///
    /// P.S. `h1(hash) &amp; self.bucket_mask` is the same as `hash as usize % self.buckets()` because the number
    /// of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
    /// ```
    ///
    /// [`RawTableInner::buckets`]: RawTableInner::buckets
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>bucket_ptr(<span class="kw-2">&amp;</span><span class="self">self</span>, index: usize, size_of: usize) -&gt; <span class="kw-2">*mut </span>u8 {
        <span class="macro">debug_assert_ne!</span>(<span class="self">self</span>.bucket_mask, <span class="number">0</span>);
        <span class="macro">debug_assert!</span>(index &lt; <span class="self">self</span>.buckets());
        <span class="kw">let </span>base: <span class="kw-2">*mut </span>u8 = <span class="self">self</span>.data_end().as_ptr();
        base.sub((index + <span class="number">1</span>) * size_of)
    }

    <span class="doccomment">/// Returns pointer to one past last `data` element in the the table as viewed from
    /// the start point of the allocation (convenience for `self.ctrl.cast()`).
    ///
    /// This function actually returns a pointer to the end of the `data element` at
    /// index "0" (zero).
    ///
    /// The caller must ensure that the `RawTableInner` outlives the returned [`NonNull&lt;T&gt;`],
    /// otherwise using it may result in [`undefined behavior`].
    ///
    /// # Note
    ///
    /// The type `T` must be the actual type of the elements stored in the table, otherwise
    /// using the returned [`NonNull&lt;T&gt;`] may result in [`undefined behavior`].
    ///
    /// ```none
    ///                        `table.data_end::&lt;T&gt;()` returns pointer that points here
    ///                        (to the end of `T0`)
    ///                          ∨
    /// [Pad], T_n, ..., T1, T0, |CT0, CT1, ..., CT_n|, CTa_0, CTa_1, ..., CTa_m
    ///                           \________  ________/
    ///                                    \/
    ///       `n = buckets - 1`, i.e. `RawTableInner::buckets() - 1`
    ///
    /// where: T0...T_n  - our stored data;
    ///        CT0...CT_n - control bytes or metadata for `data`.
    ///        CTa_0...CTa_m - additional control bytes, where `m = Group::WIDTH - 1` (so that the search
    ///                        with loading `Group` bytes from the heap works properly, even if the result
    ///                        of `h1(hash) &amp; self.bucket_mask` is equal to `self.bucket_mask`). See also
    ///                        `RawTableInner::set_ctrl` function.
    ///
    /// P.S. `h1(hash) &amp; self.bucket_mask` is the same as `hash as usize % self.buckets()` because the number
    /// of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
    /// ```
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>data_end&lt;T&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; NonNull&lt;T&gt; {
        <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY: `self.ctrl` is `NonNull`, so casting it is safe
            </span>NonNull::new_unchecked(<span class="self">self</span>.ctrl.as_ptr().cast())
        }
    }

    <span class="doccomment">/// Returns an iterator-like object for a probe sequence on the table.
    ///
    /// This iterator never terminates, but is guaranteed to visit each bucket
    /// group exactly once. The loop using `probe_seq` must terminate upon
    /// reaching a group containing an empty bucket.
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>probe_seq(<span class="kw-2">&amp;</span><span class="self">self</span>, hash: u64) -&gt; ProbeSeq {
        ProbeSeq {
            <span class="comment">// This is the same as `hash as usize % self.buckets()` because the number
            // of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
            </span>pos: h1(hash) &amp; <span class="self">self</span>.bucket_mask,
            stride: <span class="number">0</span>,
        }
    }

    <span class="doccomment">/// Returns the index of a bucket for which a value must be inserted if there is enough rooom
    /// in the table, otherwise returns error
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    #[inline]
    </span><span class="kw">unsafe fn </span>prepare_insert_no_grow(<span class="kw-2">&amp;mut </span><span class="self">self</span>, hash: u64) -&gt; <span class="prelude-ty">Result</span>&lt;usize, ()&gt; {
        <span class="kw">let </span>index = <span class="self">self</span>.find_insert_slot(hash).index;
        <span class="kw">let </span>old_ctrl = <span class="kw-2">*</span><span class="self">self</span>.ctrl(index);
        <span class="kw">if </span>unlikely(<span class="self">self</span>.growth_left == <span class="number">0 </span>&amp;&amp; special_is_empty(old_ctrl)) {
            <span class="prelude-val">Err</span>(())
        } <span class="kw">else </span>{
            <span class="self">self</span>.record_item_insert_at(index, old_ctrl, hash);
            <span class="prelude-val">Ok</span>(index)
        }
    }

    <span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>record_item_insert_at(<span class="kw-2">&amp;mut </span><span class="self">self</span>, index: usize, old_ctrl: u8, hash: u64) {
        <span class="self">self</span>.growth_left -= usize::from(special_is_empty(old_ctrl));
        <span class="self">self</span>.set_ctrl_h2(index, hash);
        <span class="self">self</span>.items += <span class="number">1</span>;
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>is_in_same_group(<span class="kw-2">&amp;</span><span class="self">self</span>, i: usize, new_i: usize, hash: u64) -&gt; bool {
        <span class="kw">let </span>probe_seq_pos = <span class="self">self</span>.probe_seq(hash).pos;
        <span class="kw">let </span>probe_index =
            |pos: usize| (pos.wrapping_sub(probe_seq_pos) &amp; <span class="self">self</span>.bucket_mask) / Group::WIDTH;
        probe_index(i) == probe_index(new_i)
    }

    <span class="doccomment">/// Sets a control byte to the hash, and possibly also the replicated control byte at
    /// the end of the array.
    ///
    /// This function does not make any changes to the `data` parts of the table,
    /// or any changes to the the `items` or `growth_left` field of the table.
    ///
    /// # Safety
    ///
    /// The safety rules are directly derived from the safety rules for [`RawTableInner::set_ctrl`]
    /// method. Thus, in order to uphold the safety contracts for the method, you must observe the
    /// following rules when calling this function:
    ///
    /// * The [`RawTableInner`] has already been allocated;
    ///
    /// * The `index` must not be greater than the `RawTableInner.bucket_mask`, i.e.
    ///   `index &lt;= RawTableInner.bucket_mask` or, in other words, `(index + 1)` must
    ///   be no greater than the number returned by the function [`RawTableInner::buckets`].
    ///
    /// Calling this function on a table that has not been allocated results in [`undefined behavior`].
    ///
    /// See also [`Bucket::as_ptr`] method, for more information about of properly removing
    /// or saving `data element` from / into the [`RawTable`] / [`RawTableInner`].
    ///
    /// [`RawTableInner::set_ctrl`]: RawTableInner::set_ctrl
    /// [`RawTableInner::buckets`]: RawTableInner::buckets
    /// [`Bucket::as_ptr`]: Bucket::as_ptr
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>set_ctrl_h2(<span class="kw-2">&amp;mut </span><span class="self">self</span>, index: usize, hash: u64) {
        <span class="comment">// SAFETY: The caller must uphold the safety rules for the [`RawTableInner::set_ctrl_h2`]
        </span><span class="self">self</span>.set_ctrl(index, h2(hash));
    }

    <span class="doccomment">/// Replaces the hash in the control byte at the given index with the provided one,
    /// and possibly also replicates the new control byte at the end of the array of control
    /// bytes, returning the old control byte.
    ///
    /// This function does not make any changes to the `data` parts of the table,
    /// or any changes to the the `items` or `growth_left` field of the table.
    ///
    /// # Safety
    ///
    /// The safety rules are directly derived from the safety rules for [`RawTableInner::set_ctrl_h2`]
    /// and [`RawTableInner::ctrl`] methods. Thus, in order to uphold the safety contracts for both
    /// methods, you must observe the following rules when calling this function:
    ///
    /// * The [`RawTableInner`] has already been allocated;
    ///
    /// * The `index` must not be greater than the `RawTableInner.bucket_mask`, i.e.
    ///   `index &lt;= RawTableInner.bucket_mask` or, in other words, `(index + 1)` must
    ///   be no greater than the number returned by the function [`RawTableInner::buckets`].
    ///
    /// Calling this function on a table that has not been allocated results in [`undefined behavior`].
    ///
    /// See also [`Bucket::as_ptr`] method, for more information about of properly removing
    /// or saving `data element` from / into the [`RawTable`] / [`RawTableInner`].
    ///
    /// [`RawTableInner::set_ctrl_h2`]: RawTableInner::set_ctrl_h2
    /// [`RawTableInner::buckets`]: RawTableInner::buckets
    /// [`Bucket::as_ptr`]: Bucket::as_ptr
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>replace_ctrl_h2(<span class="kw-2">&amp;mut </span><span class="self">self</span>, index: usize, hash: u64) -&gt; u8 {
        <span class="comment">// SAFETY: The caller must uphold the safety rules for the [`RawTableInner::replace_ctrl_h2`]
        </span><span class="kw">let </span>prev_ctrl = <span class="kw-2">*</span><span class="self">self</span>.ctrl(index);
        <span class="self">self</span>.set_ctrl_h2(index, hash);
        prev_ctrl
    }

    <span class="doccomment">/// Sets a control byte, and possibly also the replicated control byte at
    /// the end of the array.
    ///
    /// This function does not make any changes to the `data` parts of the table,
    /// or any changes to the the `items` or `growth_left` field of the table.
    ///
    /// # Safety
    ///
    /// You must observe the following safety rules when calling this function:
    ///
    /// * The [`RawTableInner`] has already been allocated;
    ///
    /// * The `index` must not be greater than the `RawTableInner.bucket_mask`, i.e.
    ///   `index &lt;= RawTableInner.bucket_mask` or, in other words, `(index + 1)` must
    ///   be no greater than the number returned by the function [`RawTableInner::buckets`].
    ///
    /// Calling this function on a table that has not been allocated results in [`undefined behavior`].
    ///
    /// See also [`Bucket::as_ptr`] method, for more information about of properly removing
    /// or saving `data element` from / into the [`RawTable`] / [`RawTableInner`].
    ///
    /// [`RawTableInner::buckets`]: RawTableInner::buckets
    /// [`Bucket::as_ptr`]: Bucket::as_ptr
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>set_ctrl(<span class="kw-2">&amp;mut </span><span class="self">self</span>, index: usize, ctrl: u8) {
        <span class="comment">// Replicate the first Group::WIDTH control bytes at the end of
        // the array without using a branch. If the tables smaller than
        // the group width (self.buckets() &lt; Group::WIDTH),
        // `index2 = Group::WIDTH + index`, otherwise `index2` is:
        //
        // - If index &gt;= Group::WIDTH then index == index2.
        // - Otherwise index2 == self.bucket_mask + 1 + index.
        //
        // The very last replicated control byte is never actually read because
        // we mask the initial index for unaligned loads, but we write it
        // anyways because it makes the set_ctrl implementation simpler.
        //
        // If there are fewer buckets than Group::WIDTH then this code will
        // replicate the buckets at the end of the trailing group. For example
        // with 2 buckets and a group size of 4, the control bytes will look
        // like this:
        //
        //     Real    |             Replicated
        // ---------------------------------------------
        // | [A] | [B] | [EMPTY] | [EMPTY] | [A] | [B] |
        // ---------------------------------------------

        // This is the same as `(index.wrapping_sub(Group::WIDTH)) % self.buckets() + Group::WIDTH`
        // because the number of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
        </span><span class="kw">let </span>index2 = ((index.wrapping_sub(Group::WIDTH)) &amp; <span class="self">self</span>.bucket_mask) + Group::WIDTH;

        <span class="comment">// SAFETY: The caller must uphold the safety rules for the [`RawTableInner::set_ctrl`]
        </span><span class="kw-2">*</span><span class="self">self</span>.ctrl(index) = ctrl;
        <span class="kw-2">*</span><span class="self">self</span>.ctrl(index2) = ctrl;
    }

    <span class="doccomment">/// Returns a pointer to a control byte.
    ///
    /// # Safety
    ///
    /// For the allocated [`RawTableInner`], the result is [`Undefined Behavior`],
    /// if the `index` is greater than the `self.bucket_mask + 1 + Group::WIDTH`.
    /// In that case, calling this function with `index == self.bucket_mask + 1 + Group::WIDTH`
    /// will return a pointer to the end of the allocated table and it is useless on its own.
    ///
    /// Calling this function with `index &gt;= self.bucket_mask + 1 + Group::WIDTH` on a
    /// table that has not been allocated results in [`Undefined Behavior`].
    ///
    /// So to satisfy both requirements you should always follow the rule that
    /// `index &lt; self.bucket_mask + 1 + Group::WIDTH`
    ///
    /// Calling this function on [`RawTableInner`] that are not already allocated is safe
    /// for read-only purpose.
    ///
    /// See also [`Bucket::as_ptr()`] method, for more information about of properly removing
    /// or saving `data element` from / into the [`RawTable`] / [`RawTableInner`].
    ///
    /// [`Bucket::as_ptr()`]: Bucket::as_ptr()
    /// [`Undefined Behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>ctrl(<span class="kw-2">&amp;</span><span class="self">self</span>, index: usize) -&gt; <span class="kw-2">*mut </span>u8 {
        <span class="macro">debug_assert!</span>(index &lt; <span class="self">self</span>.num_ctrl_bytes());
        <span class="comment">// SAFETY: The caller must uphold the safety rules for the [`RawTableInner::ctrl`]
        </span><span class="self">self</span>.ctrl.as_ptr().add(index)
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>buckets(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.bucket_mask + <span class="number">1
    </span>}

    <span class="doccomment">/// Checks whether the bucket at `index` is full.
    ///
    /// # Safety
    ///
    /// The caller must ensure `index` is less than the number of buckets.
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>is_bucket_full(<span class="kw-2">&amp;</span><span class="self">self</span>, index: usize) -&gt; bool {
        <span class="macro">debug_assert!</span>(index &lt; <span class="self">self</span>.buckets());
        is_full(<span class="kw-2">*</span><span class="self">self</span>.ctrl(index))
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>num_ctrl_bytes(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.bucket_mask + <span class="number">1 </span>+ Group::WIDTH
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>is_empty_singleton(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.bucket_mask == <span class="number">0
    </span>}

    <span class="doccomment">/// Attempts to allocate a new hash table with at least enough capacity
    /// for inserting the given number of elements without reallocating,
    /// and return it inside ScopeGuard to protect against panic in the hash
    /// function.
    ///
    /// # Note
    ///
    /// It is recommended (but not required):
    ///
    /// * That the new table's `capacity` be greater than or equal to `self.items`.
    ///
    /// * The `alloc` is the same [`Allocator`] as the `Allocator` used
    ///   to allocate this table.
    ///
    /// * The `table_layout` is the same [`TableLayout`] as the `TableLayout` used
    ///   to allocate this table.
    ///
    /// If `table_layout` does not match the `TableLayout` that was used to allocate
    /// this table, then using `mem::swap` with the `self` and the new table returned
    /// by this function results in [`undefined behavior`].
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[allow(clippy::mut_mut)]
    #[inline]
    </span><span class="kw">fn </span>prepare_resize&lt;<span class="lifetime">'a</span>, A&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        alloc: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>A,
        table_layout: TableLayout,
        capacity: usize,
        fallibility: Fallibility,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="kw">crate</span>::scopeguard::ScopeGuard&lt;<span class="self">Self</span>, <span class="kw">impl </span>FnMut(<span class="kw-2">&amp;mut </span><span class="self">Self</span>) + <span class="lifetime">'a</span>&gt;, TryReserveError&gt;
    <span class="kw">where
        </span>A: Allocator,
    {
        <span class="macro">debug_assert!</span>(<span class="self">self</span>.items &lt;= capacity);

        <span class="comment">// Allocate and initialize the new table.
        </span><span class="kw">let </span>new_table =
            RawTableInner::fallible_with_capacity(alloc, table_layout, capacity, fallibility)<span class="question-mark">?</span>;

        <span class="comment">// The hash function may panic, in which case we simply free the new
        // table without dropping any elements that may have been copied into
        // it.
        //
        // This guard is also used to free the old table on success, see
        // the comment at the bottom of this function.
        </span><span class="prelude-val">Ok</span>(guard(new_table, <span class="kw">move </span>|self_| {
            <span class="kw">if </span>!self_.is_empty_singleton() {
                <span class="comment">// SAFETY:
                // 1. We have checked that our table is allocated.
                // 2. We know for sure that the `alloc` and `table_layout` matches the
                //    [`Allocator`] and [`TableLayout`] used to allocate this table.
                </span><span class="kw">unsafe </span>{ self_.free_buckets(alloc, table_layout) };
            }
        }))
    }

    <span class="doccomment">/// Reserves or rehashes to make room for `additional` more elements.
    ///
    /// This uses dynamic dispatch to reduce the amount of
    /// code generated, but it is eliminated by LLVM optimizations when inlined.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is
    /// [`undefined behavior`]:
    ///
    /// * The `alloc` must be the same [`Allocator`] as the `Allocator` used
    ///   to allocate this table.
    ///
    /// * The `layout` must be the same [`TableLayout`] as the `TableLayout`
    ///   used to allocate this table.
    ///
    /// * The `drop` function (`fn(*mut u8)`) must be the actual drop function of
    ///   the elements stored in the table.
    ///
    /// * The [`RawTableInner`] must have properly initialized control bytes.
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[allow(clippy::inline_always)]
    #[inline(always)]
    </span><span class="kw">unsafe fn </span>reserve_rehash_inner&lt;A&gt;(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        alloc: <span class="kw-2">&amp;</span>A,
        additional: usize,
        hasher: <span class="kw-2">&amp;</span><span class="kw">dyn </span>Fn(<span class="kw-2">&amp;mut </span><span class="self">Self</span>, usize) -&gt; u64,
        fallibility: Fallibility,
        layout: TableLayout,
        drop: <span class="prelude-ty">Option</span>&lt;<span class="kw">fn</span>(<span class="kw-2">*mut </span>u8)&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), TryReserveError&gt;
    <span class="kw">where
        </span>A: Allocator,
    {
        <span class="comment">// Avoid `Option::ok_or_else` because it bloats LLVM IR.
        </span><span class="kw">let </span>new_items = <span class="kw">match </span><span class="self">self</span>.items.checked_add(additional) {
            <span class="prelude-val">Some</span>(new_items) =&gt; new_items,
            <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(fallibility.capacity_overflow()),
        };
        <span class="kw">let </span>full_capacity = bucket_mask_to_capacity(<span class="self">self</span>.bucket_mask);
        <span class="kw">if </span>new_items &lt;= full_capacity / <span class="number">2 </span>{
            <span class="comment">// Rehash in-place without re-allocating if we have plenty of spare
            // capacity that is locked up due to DELETED entries.

            // SAFETY:
            // 1. We know for sure that `[`RawTableInner`]` has already been allocated
            //    (since new_items &lt;= full_capacity / 2);
            // 2. The caller ensures that `drop` function is the actual drop function of
            //    the elements stored in the table.
            // 3. The caller ensures that `layout` matches the [`TableLayout`] that was
            //    used to allocate this table.
            // 4. The caller ensures that the control bytes of the `RawTableInner`
            //    are already initialized.
            </span><span class="self">self</span>.rehash_in_place(hasher, layout.size, drop);
            <span class="prelude-val">Ok</span>(())
        } <span class="kw">else </span>{
            <span class="comment">// Otherwise, conservatively resize to at least the next size up
            // to avoid churning deletes into frequent rehashes.
            //
            // SAFETY:
            // 1. We know for sure that `capacity &gt;= self.items`.
            // 2. The caller ensures that `alloc` and `layout` matches the [`Allocator`] and
            //    [`TableLayout`] that were used to allocate this table.
            // 3. The caller ensures that the control bytes of the `RawTableInner`
            //    are already initialized.
            </span><span class="self">self</span>.resize_inner(
                alloc,
                usize::max(new_items, full_capacity + <span class="number">1</span>),
                hasher,
                fallibility,
                layout,
            )
        }
    }

    <span class="doccomment">/// Returns an iterator over full buckets indices in the table.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// * The caller has to ensure that the `RawTableInner` outlives the
    ///   `FullBucketsIndices`. Because we cannot make the `next` method
    ///   unsafe on the `FullBucketsIndices` struct, we have to make the
    ///   `full_buckets_indices` method unsafe.
    ///
    /// * The [`RawTableInner`] must have properly initialized control bytes.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">unsafe fn </span>full_buckets_indices(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; FullBucketsIndices {
        <span class="comment">// SAFETY:
        // 1. Since the caller of this function ensures that the control bytes
        //    are properly initialized and `self.ctrl(0)` points to the start
        //    of the array of control bytes, therefore: `ctrl` is valid for reads,
        //    properly aligned to `Group::WIDTH` and points to the properly initialized
        //    control bytes.
        // 2. The value of `items` is equal to the amount of data (values) added
        //    to the table.
        //
        //                         `ctrl` points here (to the start
        //                         of the first control byte `CT0`)
        //                          ∨
        // [Pad], T_n, ..., T1, T0, |CT0, CT1, ..., CT_n|, Group::WIDTH
        //                           \________  ________/
        //                                    \/
        //       `n = buckets - 1`, i.e. `RawTableInner::buckets() - 1`
        //
        // where: T0...T_n  - our stored data;
        //        CT0...CT_n - control bytes or metadata for `data`.
        </span><span class="kw">let </span>ctrl = NonNull::new_unchecked(<span class="self">self</span>.ctrl(<span class="number">0</span>));

        FullBucketsIndices {
            <span class="comment">// Load the first group
            // SAFETY: See explanation above.
            </span>current_group: Group::load_aligned(ctrl.as_ptr()).match_full().into_iter(),
            group_first_index: <span class="number">0</span>,
            ctrl,
            items: <span class="self">self</span>.items,
        }
    }

    <span class="doccomment">/// Allocates a new table of a different size and moves the contents of the
    /// current table into it.
    ///
    /// This uses dynamic dispatch to reduce the amount of
    /// code generated, but it is eliminated by LLVM optimizations when inlined.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is
    /// [`undefined behavior`]:
    ///
    /// * The `alloc` must be the same [`Allocator`] as the `Allocator` used
    ///   to allocate this table;
    ///
    /// * The `layout` must be the same [`TableLayout`] as the `TableLayout`
    ///   used to allocate this table;
    ///
    /// * The [`RawTableInner`] must have properly initialized control bytes.
    ///
    /// The caller of this function must ensure that `capacity &gt;= self.items`
    /// otherwise:
    ///
    /// * If `self.items != 0`, calling of this function with `capacity == 0`
    ///   results in [`undefined behavior`].
    ///
    /// * If `capacity_to_buckets(capacity) &lt; Group::WIDTH` and
    ///   `self.items &gt; capacity_to_buckets(capacity)` calling this function
    ///   results in [`undefined behavior`].
    ///
    /// * If `capacity_to_buckets(capacity) &gt;= Group::WIDTH` and
    ///   `self.items &gt; capacity_to_buckets(capacity)` calling this function
    ///   are never return (will go into an infinite loop).
    ///
    /// Note: It is recommended (but not required) that the new table's `capacity`
    /// be greater than or equal to `self.items`. In case if `capacity &lt;= self.items`
    /// this function can never return. See [`RawTableInner::find_insert_slot`] for
    /// more information.
    ///
    /// [`RawTableInner::find_insert_slot`]: RawTableInner::find_insert_slot
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[allow(clippy::inline_always)]
    #[inline(always)]
    </span><span class="kw">unsafe fn </span>resize_inner&lt;A&gt;(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        alloc: <span class="kw-2">&amp;</span>A,
        capacity: usize,
        hasher: <span class="kw-2">&amp;</span><span class="kw">dyn </span>Fn(<span class="kw-2">&amp;mut </span><span class="self">Self</span>, usize) -&gt; u64,
        fallibility: Fallibility,
        layout: TableLayout,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), TryReserveError&gt;
    <span class="kw">where
        </span>A: Allocator,
    {
        <span class="comment">// SAFETY: We know for sure that `alloc` and `layout` matches the [`Allocator`] and [`TableLayout`]
        // that were used to allocate this table.
        </span><span class="kw">let </span><span class="kw-2">mut </span>new_table = <span class="self">self</span>.prepare_resize(alloc, layout, capacity, fallibility)<span class="question-mark">?</span>;

        <span class="comment">// SAFETY: We know for sure that RawTableInner will outlive the
        // returned `FullBucketsIndices` iterator, and the caller of this
        // function ensures that the control bytes are properly initialized.
        </span><span class="kw">for </span>full_byte_index <span class="kw">in </span><span class="self">self</span>.full_buckets_indices() {
            <span class="comment">// This may panic.
            </span><span class="kw">let </span>hash = hasher(<span class="self">self</span>, full_byte_index);

            <span class="comment">// SAFETY:
            // We can use a simpler version of insert() here since:
            // 1. There are no DELETED entries.
            // 2. We know there is enough space in the table.
            // 3. All elements are unique.
            // 4. The caller of this function guarantees that `capacity &gt; 0`
            //    so `new_table` must already have some allocated memory.
            // 5. We set `growth_left` and `items` fields of the new table
            //    after the loop.
            // 6. We insert into the table, at the returned index, the data
            //    matching the given hash immediately after calling this function.
            </span><span class="kw">let </span>(new_index, <span class="kw">_</span>) = new_table.prepare_insert_slot(hash);

            <span class="comment">// SAFETY:
            //
            // * `src` is valid for reads of `layout.size` bytes, since the
            //   table is alive and the `full_byte_index` is guaranteed to be
            //   within bounds (see `FullBucketsIndices::next_impl`);
            //
            // * `dst` is valid for writes of `layout.size` bytes, since the
            //   caller ensures that `table_layout` matches the [`TableLayout`]
            //   that was used to allocate old table and we have the `new_index`
            //   returned by `prepare_insert_slot`.
            //
            // * Both `src` and `dst` are properly aligned.
            //
            // * Both `src` and `dst` point to different region of memory.
            </span>ptr::copy_nonoverlapping(
                <span class="self">self</span>.bucket_ptr(full_byte_index, layout.size),
                new_table.bucket_ptr(new_index, layout.size),
                layout.size,
            );
        }

        <span class="comment">// The hash function didn't panic, so we can safely set the
        // `growth_left` and `items` fields of the new table.
        </span>new_table.growth_left -= <span class="self">self</span>.items;
        new_table.items = <span class="self">self</span>.items;

        <span class="comment">// We successfully copied all elements without panicking. Now replace
        // self with the new table. The old table will have its memory freed but
        // the items will not be dropped (since they have been moved into the
        // new table).
        // SAFETY: The caller ensures that `table_layout` matches the [`TableLayout`]
        // that was used to allocate this table.
        </span>mem::swap(<span class="self">self</span>, <span class="kw-2">&amp;mut </span>new_table);

        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Rehashes the contents of the table in place (i.e. without changing the
    /// allocation).
    ///
    /// If `hasher` panics then some the table's contents may be lost.
    ///
    /// This uses dynamic dispatch to reduce the amount of
    /// code generated, but it is eliminated by LLVM optimizations when inlined.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is [`undefined behavior`]:
    ///
    /// * The `size_of` must be equal to the size of the elements stored in the table;
    ///
    /// * The `drop` function (`fn(*mut u8)`) must be the actual drop function of
    ///   the elements stored in the table.
    ///
    /// * The [`RawTableInner`] has already been allocated;
    ///
    /// * The [`RawTableInner`] must have properly initialized control bytes.
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[allow(clippy::inline_always)]
    #[cfg_attr(feature = <span class="string">"inline-more"</span>, inline(always))]
    #[cfg_attr(not(feature = <span class="string">"inline-more"</span>), inline)]
    </span><span class="kw">unsafe fn </span>rehash_in_place(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        hasher: <span class="kw-2">&amp;</span><span class="kw">dyn </span>Fn(<span class="kw-2">&amp;mut </span><span class="self">Self</span>, usize) -&gt; u64,
        size_of: usize,
        drop: <span class="prelude-ty">Option</span>&lt;<span class="kw">fn</span>(<span class="kw-2">*mut </span>u8)&gt;,
    ) {
        <span class="comment">// If the hash function panics then properly clean up any elements
        // that we haven't rehashed yet. We unfortunately can't preserve the
        // element since we lost their hash and have no way of recovering it
        // without risking another panic.
        </span><span class="self">self</span>.prepare_rehash_in_place();

        <span class="kw">let </span><span class="kw-2">mut </span>guard = guard(<span class="self">self</span>, <span class="kw">move </span>|self_| {
            <span class="kw">if let </span><span class="prelude-val">Some</span>(drop) = drop {
                <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..self_.buckets() {
                    <span class="kw">if </span><span class="kw-2">*</span>self_.ctrl(i) == DELETED {
                        self_.set_ctrl(i, EMPTY);
                        drop(self_.bucket_ptr(i, size_of));
                        self_.items -= <span class="number">1</span>;
                    }
                }
            }
            self_.growth_left = bucket_mask_to_capacity(self_.bucket_mask) - self_.items;
        });

        <span class="comment">// At this point, DELETED elements are elements that we haven't
        // rehashed yet. Find them and re-insert them at their ideal
        // position.
        </span><span class="lifetime">'outer</span>: <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..guard.buckets() {
            <span class="kw">if </span><span class="kw-2">*</span>guard.ctrl(i) != DELETED {
                <span class="kw">continue</span>;
            }

            <span class="kw">let </span>i_p = guard.bucket_ptr(i, size_of);

            <span class="lifetime">'inner</span>: <span class="kw">loop </span>{
                <span class="comment">// Hash the current item
                </span><span class="kw">let </span>hash = hasher(<span class="kw-2">*</span>guard, i);

                <span class="comment">// Search for a suitable place to put it
                //
                // SAFETY: Caller of this function ensures that the control bytes
                // are properly initialized.
                </span><span class="kw">let </span>new_i = guard.find_insert_slot(hash).index;

                <span class="comment">// Probing works by scanning through all of the control
                // bytes in groups, which may not be aligned to the group
                // size. If both the new and old position fall within the
                // same unaligned group, then there is no benefit in moving
                // it and we can just continue to the next item.
                </span><span class="kw">if </span>likely(guard.is_in_same_group(i, new_i, hash)) {
                    guard.set_ctrl_h2(i, hash);
                    <span class="kw">continue </span><span class="lifetime">'outer</span>;
                }

                <span class="kw">let </span>new_i_p = guard.bucket_ptr(new_i, size_of);

                <span class="comment">// We are moving the current item to a new position. Write
                // our H2 to the control byte of the new position.
                </span><span class="kw">let </span>prev_ctrl = guard.replace_ctrl_h2(new_i, hash);
                <span class="kw">if </span>prev_ctrl == EMPTY {
                    guard.set_ctrl(i, EMPTY);
                    <span class="comment">// If the target slot is empty, simply move the current
                    // element into the new slot and clear the old control
                    // byte.
                    </span>ptr::copy_nonoverlapping(i_p, new_i_p, size_of);
                    <span class="kw">continue </span><span class="lifetime">'outer</span>;
                } <span class="kw">else </span>{
                    <span class="comment">// If the target slot is occupied, swap the two elements
                    // and then continue processing the element that we just
                    // swapped into the old slot.
                    </span><span class="macro">debug_assert_eq!</span>(prev_ctrl, DELETED);
                    ptr::swap_nonoverlapping(i_p, new_i_p, size_of);
                    <span class="kw">continue </span><span class="lifetime">'inner</span>;
                }
            }
        }

        guard.growth_left = bucket_mask_to_capacity(guard.bucket_mask) - guard.items;

        mem::forget(guard);
    }

    <span class="doccomment">/// Deallocates the table without dropping any entries.
    ///
    /// # Note
    ///
    /// This function must be called only after [`drop_elements`](RawTableInner::drop_elements),
    /// else it can lead to leaking of memory. Also calling this function automatically
    /// makes invalid (dangling) all instances of buckets ([`Bucket`]) and makes invalid
    /// (dangling) the `ctrl` field of the table.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is [`Undefined Behavior`]:
    ///
    /// * The [`RawTableInner`] has already been allocated;
    ///
    /// * The `alloc` must be the same [`Allocator`] as the `Allocator` that was used
    ///   to allocate this table.
    ///
    /// * The `table_layout` must be the same [`TableLayout`] as the `TableLayout` that was used
    ///   to allocate this table.
    ///
    /// See also [`GlobalAlloc::dealloc`] or [`Allocator::deallocate`] for more  information.
    ///
    /// [`Undefined Behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    /// [`GlobalAlloc::dealloc`]: https://doc.rust-lang.org/alloc/alloc/trait.GlobalAlloc.html#tymethod.dealloc
    /// [`Allocator::deallocate`]: https://doc.rust-lang.org/alloc/alloc/trait.Allocator.html#tymethod.deallocate
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>free_buckets&lt;A&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, alloc: <span class="kw-2">&amp;</span>A, table_layout: TableLayout)
    <span class="kw">where
        </span>A: Allocator,
    {
        <span class="comment">// SAFETY: The caller must uphold the safety contract for `free_buckets`
        // method.
        </span><span class="kw">let </span>(ptr, layout) = <span class="self">self</span>.allocation_info(table_layout);
        alloc.deallocate(ptr, layout);
    }

    <span class="doccomment">/// Returns a pointer to the allocated memory and the layout that was used to
    /// allocate the table.
    ///
    /// # Safety
    ///
    /// Caller of this function must observe the following safety rules:
    ///
    /// * The [`RawTableInner`] has already been allocated, otherwise
    ///   calling this function results in [`undefined behavior`]
    ///
    /// * The `table_layout` must be the same [`TableLayout`] as the `TableLayout`
    ///   that was used to allocate this table. Failure to comply with this condition
    ///   may result in [`undefined behavior`].
    ///
    /// See also [`GlobalAlloc::dealloc`] or [`Allocator::deallocate`] for more  information.
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    /// [`GlobalAlloc::dealloc`]: https://doc.rust-lang.org/alloc/alloc/trait.GlobalAlloc.html#tymethod.dealloc
    /// [`Allocator::deallocate`]: https://doc.rust-lang.org/alloc/alloc/trait.Allocator.html#tymethod.deallocate
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>allocation_info(<span class="kw-2">&amp;</span><span class="self">self</span>, table_layout: TableLayout) -&gt; (NonNull&lt;u8&gt;, Layout) {
        <span class="macro">debug_assert!</span>(
            !<span class="self">self</span>.is_empty_singleton(),
            <span class="string">"this function can only be called on non-empty tables"
        </span>);

        <span class="comment">// Avoid `Option::unwrap_or_else` because it bloats LLVM IR.
        </span><span class="kw">let </span>(layout, ctrl_offset) = <span class="kw">match </span>table_layout.calculate_layout_for(<span class="self">self</span>.buckets()) {
            <span class="prelude-val">Some</span>(lco) =&gt; lco,
            <span class="prelude-val">None </span>=&gt; <span class="kw">unsafe </span>{ hint::unreachable_unchecked() },
        };
        (
            <span class="comment">// SAFETY: The caller must uphold the safety contract for `allocation_info` method.
            </span><span class="kw">unsafe </span>{ NonNull::new_unchecked(<span class="self">self</span>.ctrl.as_ptr().sub(ctrl_offset)) },
            layout,
        )
    }

    <span class="doccomment">/// Returns a pointer to the allocated memory and the layout that was used to
    /// allocate the table. If [`RawTableInner`] has not been allocated, this
    /// function return `dangling` pointer and `()` (unit) layout.
    ///
    /// # Safety
    ///
    /// The `table_layout` must be the same [`TableLayout`] as the `TableLayout`
    /// that was used to allocate this table. Failure to comply with this condition
    /// may result in [`undefined behavior`].
    ///
    /// See also [`GlobalAlloc::dealloc`] or [`Allocator::deallocate`] for more  information.
    ///
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    /// [`GlobalAlloc::dealloc`]: https://doc.rust-lang.org/alloc/alloc/trait.GlobalAlloc.html#tymethod.dealloc
    /// [`Allocator::deallocate`]: https://doc.rust-lang.org/alloc/alloc/trait.Allocator.html#tymethod.deallocate
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">unsafe fn </span>allocation_info_or_zero(<span class="kw-2">&amp;</span><span class="self">self</span>, table_layout: TableLayout) -&gt; (NonNull&lt;u8&gt;, Layout) {
        <span class="kw">if </span><span class="self">self</span>.is_empty_singleton() {
            (NonNull::dangling(), Layout::new::&lt;()&gt;())
        } <span class="kw">else </span>{
            <span class="comment">// SAFETY:
            // 1. We have checked that our table is allocated.
            // 2. The caller ensures that `table_layout` matches the [`TableLayout`]
            // that was used to allocate this table.
            </span><span class="kw">unsafe </span>{ <span class="self">self</span>.allocation_info(table_layout) }
        }
    }

    <span class="doccomment">/// Marks all table buckets as empty without dropping their contents.
    </span><span class="attr">#[inline]
    </span><span class="kw">fn </span>clear_no_drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">if </span>!<span class="self">self</span>.is_empty_singleton() {
            <span class="kw">unsafe </span>{
                <span class="self">self</span>.ctrl(<span class="number">0</span>).write_bytes(EMPTY, <span class="self">self</span>.num_ctrl_bytes());
            }
        }
        <span class="self">self</span>.items = <span class="number">0</span>;
        <span class="self">self</span>.growth_left = bucket_mask_to_capacity(<span class="self">self</span>.bucket_mask);
    }

    <span class="doccomment">/// Erases the [`Bucket`]'s control byte at the given index so that it does not
    /// triggered as full, decreases the `items` of the table and, if it can be done,
    /// increases `self.growth_left`.
    ///
    /// This function does not actually erase / drop the [`Bucket`] itself, i.e. it
    /// does not make any changes to the `data` parts of the table. The caller of this
    /// function must take care to properly drop the `data`, otherwise calling this
    /// function may result in a memory leak.
    ///
    /// # Safety
    ///
    /// You must observe the following safety rules when calling this function:
    ///
    /// * The [`RawTableInner`] has already been allocated;
    ///
    /// * It must be the full control byte at the given position;
    ///
    /// * The `index` must not be greater than the `RawTableInner.bucket_mask`, i.e.
    ///   `index &lt;= RawTableInner.bucket_mask` or, in other words, `(index + 1)` must
    ///   be no greater than the number returned by the function [`RawTableInner::buckets`].
    ///
    /// Calling this function on a table that has not been allocated results in [`undefined behavior`].
    ///
    /// Calling this function on a table with no elements is unspecified, but calling subsequent
    /// functions is likely to result in [`undefined behavior`] due to overflow subtraction
    /// (`self.items -= 1 cause overflow when self.items == 0`).
    ///
    /// See also [`Bucket::as_ptr`] method, for more information about of properly removing
    /// or saving `data element` from / into the [`RawTable`] / [`RawTableInner`].
    ///
    /// [`RawTableInner::buckets`]: RawTableInner::buckets
    /// [`Bucket::as_ptr`]: Bucket::as_ptr
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline]
    </span><span class="kw">unsafe fn </span>erase(<span class="kw-2">&amp;mut </span><span class="self">self</span>, index: usize) {
        <span class="macro">debug_assert!</span>(<span class="self">self</span>.is_bucket_full(index));

        <span class="comment">// This is the same as `index.wrapping_sub(Group::WIDTH) % self.buckets()` because
        // the number of buckets is a power of two, and `self.bucket_mask = self.buckets() - 1`.
        </span><span class="kw">let </span>index_before = index.wrapping_sub(Group::WIDTH) &amp; <span class="self">self</span>.bucket_mask;
        <span class="comment">// SAFETY:
        // - The caller must uphold the safety contract for `erase` method;
        // - `index_before` is guaranteed to be in range due to masking with `self.bucket_mask`
        </span><span class="kw">let </span>empty_before = Group::load(<span class="self">self</span>.ctrl(index_before)).match_empty();
        <span class="kw">let </span>empty_after = Group::load(<span class="self">self</span>.ctrl(index)).match_empty();

        <span class="comment">// Inserting and searching in the map is performed by two key functions:
        //
        // - The `find_insert_slot` function that looks up the index of any `EMPTY` or `DELETED`
        //   slot in a group to be able to insert. If it doesn't find an `EMPTY` or `DELETED`
        //   slot immediately in the first group, it jumps to the next `Group` looking for it,
        //   and so on until it has gone through all the groups in the control bytes.
        //
        // - The `find_inner` function that looks for the index of the desired element by looking
        //   at all the `FULL` bytes in the group. If it did not find the element right away, and
        //   there is no `EMPTY` byte in the group, then this means that the `find_insert_slot`
        //   function may have found a suitable slot in the next group. Therefore, `find_inner`
        //   jumps further, and if it does not find the desired element and again there is no `EMPTY`
        //   byte, then it jumps further, and so on. The search stops only if `find_inner` function
        //   finds the desired element or hits an `EMPTY` slot/byte.
        //
        // Accordingly, this leads to two consequences:
        //
        // - The map must have `EMPTY` slots (bytes);
        //
        // - You can't just mark the byte to be erased as `EMPTY`, because otherwise the `find_inner`
        //   function may stumble upon an `EMPTY` byte before finding the desired element and stop
        //   searching.
        //
        // Thus it is necessary to check all bytes after and before the erased element. If we are in
        // a contiguous `Group` of `FULL` or `DELETED` bytes (the number of `FULL` or `DELETED` bytes
        // before and after is greater than or equal to `Group::WIDTH`), then we must mark our byte as
        // `DELETED` in order for the `find_inner` function to go further. On the other hand, if there
        // is at least one `EMPTY` slot in the `Group`, then the `find_inner` function will still stumble
        // upon an `EMPTY` byte, so we can safely mark our erased byte as `EMPTY` as well.
        //
        // Finally, since `index_before == (index.wrapping_sub(Group::WIDTH) &amp; self.bucket_mask) == index`
        // and given all of the above, tables smaller than the group width (self.buckets() &lt; Group::WIDTH)
        // cannot have `DELETED` bytes.
        //
        // Note that in this context `leading_zeros` refers to the bytes at the end of a group, while
        // `trailing_zeros` refers to the bytes at the beginning of a group.
        </span><span class="kw">let </span>ctrl = <span class="kw">if </span>empty_before.leading_zeros() + empty_after.trailing_zeros() &gt;= Group::WIDTH {
            DELETED
        } <span class="kw">else </span>{
            <span class="self">self</span>.growth_left += <span class="number">1</span>;
            EMPTY
        };
        <span class="comment">// SAFETY: the caller must uphold the safety contract for `erase` method.
        </span><span class="self">self</span>.set_ctrl(index, ctrl);
        <span class="self">self</span>.items -= <span class="number">1</span>;
    }
}

<span class="kw">impl</span>&lt;T: Clone, A: Allocator + Clone&gt; Clone <span class="kw">for </span>RawTable&lt;T, A&gt; {
    <span class="kw">fn </span>clone(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="kw">if </span><span class="self">self</span>.table.is_empty_singleton() {
            <span class="self">Self</span>::new_in(<span class="self">self</span>.alloc.clone())
        } <span class="kw">else </span>{
            <span class="kw">unsafe </span>{
                <span class="comment">// Avoid `Result::ok_or_else` because it bloats LLVM IR.
                //
                // SAFETY: This is safe as we are taking the size of an already allocated table
                // and therefore сapacity overflow cannot occur, `self.table.buckets()` is power
                // of two and all allocator errors will be caught inside `RawTableInner::new_uninitialized`.
                </span><span class="kw">let </span><span class="kw-2">mut </span>new_table = <span class="kw">match </span><span class="self">Self</span>::new_uninitialized(
                    <span class="self">self</span>.alloc.clone(),
                    <span class="self">self</span>.table.buckets(),
                    Fallibility::Infallible,
                ) {
                    <span class="prelude-val">Ok</span>(table) =&gt; table,
                    <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; hint::unreachable_unchecked(),
                };

                <span class="comment">// Cloning elements may fail (the clone function may panic). But we don't
                // need to worry about uninitialized control bits, since:
                // 1. The number of items (elements) in the table is zero, which means that
                //    the control bits will not be readed by Drop function.
                // 2. The `clone_from_spec` method will first copy all control bits from
                //    `self` (thus initializing them). But this will not affect the `Drop`
                //    function, since the `clone_from_spec` function sets `items` only after
                //    successfully clonning all elements.
                </span>new_table.clone_from_spec(<span class="self">self</span>);
                new_table
            }
        }
    }

    <span class="kw">fn </span>clone_from(<span class="kw-2">&amp;mut </span><span class="self">self</span>, source: <span class="kw-2">&amp;</span><span class="self">Self</span>) {
        <span class="kw">if </span>source.table.is_empty_singleton() {
            <span class="kw">let </span><span class="kw-2">mut </span>old_inner = mem::replace(<span class="kw-2">&amp;mut </span><span class="self">self</span>.table, RawTableInner::NEW);
            <span class="kw">unsafe </span>{
                <span class="comment">// SAFETY:
                // 1. We call the function only once;
                // 2. We know for sure that `alloc` and `table_layout` matches the [`Allocator`]
                //    and [`TableLayout`] that were used to allocate this table.
                // 3. If any elements' drop function panics, then there will only be a memory leak,
                //    because we have replaced the inner table with a new one.
                </span>old_inner.drop_inner_table::&lt;T, <span class="kw">_</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>.alloc, <span class="self">Self</span>::TABLE_LAYOUT);
            }
        } <span class="kw">else </span>{
            <span class="kw">unsafe </span>{
                <span class="comment">// Make sure that if any panics occurs, we clear the table and
                // leave it in an empty state.
                </span><span class="kw">let </span><span class="kw-2">mut </span>self_ = guard(<span class="self">self</span>, |self_| {
                    self_.clear_no_drop();
                });

                <span class="comment">// First, drop all our elements without clearing the control
                // bytes. If this panics then the scope guard will clear the
                // table, leaking any elements that were not dropped yet.
                //
                // This leak is unavoidable: we can't try dropping more elements
                // since this could lead to another panic and abort the process.
                //
                // SAFETY: If something gets wrong we clear our table right after
                // dropping the elements, so there is no double drop, since `items`
                // will be equal to zero.
                </span>self_.table.drop_elements::&lt;T&gt;();

                <span class="comment">// If necessary, resize our table to match the source.
                </span><span class="kw">if </span>self_.buckets() != source.buckets() {
                    <span class="kw">let </span>new_inner = <span class="kw">match </span>RawTableInner::new_uninitialized(
                        <span class="kw-2">&amp;</span>self_.alloc,
                        <span class="self">Self</span>::TABLE_LAYOUT,
                        source.buckets(),
                        Fallibility::Infallible,
                    ) {
                        <span class="prelude-val">Ok</span>(table) =&gt; table,
                        <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; hint::unreachable_unchecked(),
                    };
                    <span class="comment">// Replace the old inner with new uninitialized one. It's ok, since if something gets
                    // wrong `ScopeGuard` will initialize all control bytes and leave empty table.
                    </span><span class="kw">let </span><span class="kw-2">mut </span>old_inner = mem::replace(<span class="kw-2">&amp;mut </span>self_.table, new_inner);
                    <span class="kw">if </span>!old_inner.is_empty_singleton() {
                        <span class="comment">// SAFETY:
                        // 1. We have checked that our table is allocated.
                        // 2. We know for sure that `alloc` and `table_layout` matches
                        // the [`Allocator`] and [`TableLayout`] that were used to allocate this table.
                        </span>old_inner.free_buckets(<span class="kw-2">&amp;</span>self_.alloc, <span class="self">Self</span>::TABLE_LAYOUT);
                    }
                }

                <span class="comment">// Cloning elements may fail (the clone function may panic), but the `ScopeGuard`
                // inside the `clone_from_impl` function will take care of that, dropping all
                // cloned elements if necessary. Our `ScopeGuard` will clear the table.
                </span>self_.clone_from_spec(source);

                <span class="comment">// Disarm the scope guard if cloning was successful.
                </span>ScopeGuard::into_inner(self_);
            }
        }
    }
}

<span class="doccomment">/// Specialization of `clone_from` for `Copy` types
</span><span class="kw">trait </span>RawTableClone {
    <span class="kw">unsafe fn </span>clone_from_spec(<span class="kw-2">&amp;mut </span><span class="self">self</span>, source: <span class="kw-2">&amp;</span><span class="self">Self</span>);
}
<span class="kw">impl</span>&lt;T: Clone, A: Allocator + Clone&gt; RawTableClone <span class="kw">for </span>RawTable&lt;T, A&gt; {
    <span class="macro">default_fn!</span> {
        <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
        </span><span class="kw">unsafe fn </span>clone_from_spec(<span class="kw-2">&amp;mut </span><span class="self">self</span>, source: <span class="kw-2">&amp;</span><span class="self">Self</span>) {
            <span class="self">self</span>.clone_from_impl(source);
        }
    }
}
<span class="attr">#[cfg(feature = <span class="string">"nightly"</span>)]
</span><span class="kw">impl</span>&lt;T: Copy, A: Allocator + Clone&gt; RawTableClone <span class="kw">for </span>RawTable&lt;T, A&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">unsafe fn </span>clone_from_spec(<span class="kw-2">&amp;mut </span><span class="self">self</span>, source: <span class="kw-2">&amp;</span><span class="self">Self</span>) {
        source
            .table
            .ctrl(<span class="number">0</span>)
            .copy_to_nonoverlapping(<span class="self">self</span>.table.ctrl(<span class="number">0</span>), <span class="self">self</span>.table.num_ctrl_bytes());
        source
            .data_start()
            .as_ptr()
            .copy_to_nonoverlapping(<span class="self">self</span>.data_start().as_ptr(), <span class="self">self</span>.table.buckets());

        <span class="self">self</span>.table.items = source.table.items;
        <span class="self">self</span>.table.growth_left = source.table.growth_left;
    }
}

<span class="kw">impl</span>&lt;T: Clone, A: Allocator + Clone&gt; RawTable&lt;T, A&gt; {
    <span class="doccomment">/// Common code for clone and clone_from. Assumes:
    /// - `self.buckets() == source.buckets()`.
    /// - Any existing elements have been dropped.
    /// - The control bytes are not initialized yet.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">unsafe fn </span>clone_from_impl(<span class="kw-2">&amp;mut </span><span class="self">self</span>, source: <span class="kw-2">&amp;</span><span class="self">Self</span>) {
        <span class="comment">// Copy the control bytes unchanged. We do this in a single pass
        </span>source
            .table
            .ctrl(<span class="number">0</span>)
            .copy_to_nonoverlapping(<span class="self">self</span>.table.ctrl(<span class="number">0</span>), <span class="self">self</span>.table.num_ctrl_bytes());

        <span class="comment">// The cloning of elements may panic, in which case we need
        // to make sure we drop only the elements that have been
        // cloned so far.
        </span><span class="kw">let </span><span class="kw-2">mut </span>guard = guard((<span class="number">0</span>, <span class="kw-2">&amp;mut *</span><span class="self">self</span>), |(index, self_)| {
            <span class="kw">if </span>T::NEEDS_DROP {
                <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..=<span class="kw-2">*</span>index {
                    <span class="kw">if </span>self_.is_bucket_full(i) {
                        self_.bucket(i).drop();
                    }
                }
            }
        });

        <span class="kw">for </span>from <span class="kw">in </span>source.iter() {
            <span class="kw">let </span>index = source.bucket_index(<span class="kw-2">&amp;</span>from);
            <span class="kw">let </span>to = guard.<span class="number">1</span>.bucket(index);
            to.write(from.as_ref().clone());

            <span class="comment">// Update the index in case we need to unwind.
            </span>guard.<span class="number">0 </span>= index;
        }

        <span class="comment">// Successfully cloned all items, no need to clean up.
        </span>mem::forget(guard);

        <span class="self">self</span>.table.items = source.table.items;
        <span class="self">self</span>.table.growth_left = source.table.growth_left;
    }

    <span class="doccomment">/// Variant of `clone_from` to use when a hasher is available.
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">pub fn </span>clone_from_with_hasher(<span class="kw-2">&amp;mut </span><span class="self">self</span>, source: <span class="kw-2">&amp;</span><span class="self">Self</span>, hasher: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; u64) {
        <span class="comment">// If we have enough capacity in the table, just clear it and insert
        // elements one by one. We don't do this if we have the same number of
        // buckets as the source since we can just copy the contents directly
        // in that case.
        </span><span class="kw">if </span><span class="self">self</span>.table.buckets() != source.table.buckets()
            &amp;&amp; bucket_mask_to_capacity(<span class="self">self</span>.table.bucket_mask) &gt;= source.len()
        {
            <span class="self">self</span>.clear();

            <span class="kw">let </span><span class="kw-2">mut </span>guard_self = guard(<span class="kw-2">&amp;mut *</span><span class="self">self</span>, |self_| {
                <span class="comment">// Clear the partially copied table if a panic occurs, otherwise
                // items and growth_left will be out of sync with the contents
                // of the table.
                </span>self_.clear();
            });

            <span class="kw">unsafe </span>{
                <span class="kw">for </span>item <span class="kw">in </span>source.iter() {
                    <span class="comment">// This may panic.
                    </span><span class="kw">let </span>item = item.as_ref().clone();
                    <span class="kw">let </span>hash = hasher(<span class="kw-2">&amp;</span>item);

                    <span class="comment">// We can use a simpler version of insert() here since:
                    // - there are no DELETED entries.
                    // - we know there is enough space in the table.
                    // - all elements are unique.
                    </span><span class="kw">let </span>(index, <span class="kw">_</span>) = guard_self.table.prepare_insert_slot(hash);
                    guard_self.bucket(index).write(item);
                }
            }

            <span class="comment">// Successfully cloned all items, no need to clean up.
            </span>mem::forget(guard_self);

            <span class="self">self</span>.table.items = source.table.items;
            <span class="self">self</span>.table.growth_left -= source.table.items;
        } <span class="kw">else </span>{
            <span class="self">self</span>.clone_from(source);
        }
    }
}

<span class="kw">impl</span>&lt;T, A: Allocator + Default&gt; Default <span class="kw">for </span>RawTable&lt;T, A&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::new_in(Default::default())
    }
}

<span class="attr">#[cfg(feature = <span class="string">"nightly"</span>)]
</span><span class="kw">unsafe impl</span>&lt;<span class="attr">#[may_dangle] </span>T, A: Allocator&gt; Drop <span class="kw">for </span>RawTable&lt;T, A&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY:
            // 1. We call the function only once;
            // 2. We know for sure that `alloc` and `table_layout` matches the [`Allocator`]
            //    and [`TableLayout`] that were used to allocate this table.
            // 3. If the drop function of any elements fails, then only a memory leak will occur,
            //    and we don't care because we are inside the `Drop` function of the `RawTable`,
            //    so there won't be any table left in an inconsistent state.
            </span><span class="self">self</span>.table
                .drop_inner_table::&lt;T, <span class="kw">_</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>.alloc, <span class="self">Self</span>::TABLE_LAYOUT);
        }
    }
}
<span class="attr">#[cfg(not(feature = <span class="string">"nightly"</span>))]
</span><span class="kw">impl</span>&lt;T, A: Allocator&gt; Drop <span class="kw">for </span>RawTable&lt;T, A&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY:
            // 1. We call the function only once;
            // 2. We know for sure that `alloc` and `table_layout` matches the [`Allocator`]
            //    and [`TableLayout`] that were used to allocate this table.
            // 3. If the drop function of any elements fails, then only a memory leak will occur,
            //    and we don't care because we are inside the `Drop` function of the `RawTable`,
            //    so there won't be any table left in an inconsistent state.
            </span><span class="self">self</span>.table
                .drop_inner_table::&lt;T, <span class="kw">_</span>&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>.alloc, <span class="self">Self</span>::TABLE_LAYOUT);
        }
    }
}

<span class="kw">impl</span>&lt;T, A: Allocator&gt; IntoIterator <span class="kw">for </span>RawTable&lt;T, A&gt; {
    <span class="kw">type </span>Item = T;
    <span class="kw">type </span>IntoIter = RawIntoIter&lt;T, A&gt;;

    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>into_iter(<span class="self">self</span>) -&gt; RawIntoIter&lt;T, A&gt; {
        <span class="kw">unsafe </span>{
            <span class="kw">let </span>iter = <span class="self">self</span>.iter();
            <span class="self">self</span>.into_iter_from(iter)
        }
    }
}

<span class="doccomment">/// Iterator over a sub-range of a table. Unlike `RawIter` this iterator does
/// not track an item count.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>RawIterRange&lt;T&gt; {
    <span class="comment">// Mask of full buckets in the current group. Bits are cleared from this
    // mask as each element is processed.
    </span>current_group: BitMaskIter,

    <span class="comment">// Pointer to the buckets for the current group.
    </span>data: Bucket&lt;T&gt;,

    <span class="comment">// Pointer to the next group of control bytes,
    // Must be aligned to the group size.
    </span>next_ctrl: <span class="kw-2">*const </span>u8,

    <span class="comment">// Pointer one past the last control byte of this range.
    </span>end: <span class="kw-2">*const </span>u8,
}

<span class="kw">impl</span>&lt;T&gt; RawIterRange&lt;T&gt; {
    <span class="doccomment">/// Returns a `RawIterRange` covering a subset of a table.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is
    /// [`undefined behavior`]:
    ///
    /// * `ctrl` must be [valid] for reads, i.e. table outlives the `RawIterRange`;
    ///
    /// * `ctrl` must be properly aligned to the group size (Group::WIDTH);
    ///
    /// * `ctrl` must point to the array of properly initialized control bytes;
    ///
    /// * `data` must be the [`Bucket`] at the `ctrl` index in the table;
    ///
    /// * the value of `len` must be less than or equal to the number of table buckets,
    ///   and the returned value of `ctrl.as_ptr().add(len).offset_from(ctrl.as_ptr())`
    ///   must be positive.
    ///
    /// * The `ctrl.add(len)` pointer must be either in bounds or one
    ///   byte past the end of the same [allocated table].
    ///
    /// * The `len` must be a power of two.
    ///
    /// [valid]: https://doc.rust-lang.org/std/ptr/index.html#safety
    /// [`undefined behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">unsafe fn </span>new(ctrl: <span class="kw-2">*const </span>u8, data: Bucket&lt;T&gt;, len: usize) -&gt; <span class="self">Self </span>{
        <span class="macro">debug_assert_ne!</span>(len, <span class="number">0</span>);
        <span class="macro">debug_assert_eq!</span>(ctrl <span class="kw">as </span>usize % Group::WIDTH, <span class="number">0</span>);
        <span class="comment">// SAFETY: The caller must uphold the safety rules for the [`RawIterRange::new`]
        </span><span class="kw">let </span>end = ctrl.add(len);

        <span class="comment">// Load the first group and advance ctrl to point to the next group
        // SAFETY: The caller must uphold the safety rules for the [`RawIterRange::new`]
        </span><span class="kw">let </span>current_group = Group::load_aligned(ctrl).match_full();
        <span class="kw">let </span>next_ctrl = ctrl.add(Group::WIDTH);

        <span class="self">Self </span>{
            current_group: current_group.into_iter(),
            data,
            next_ctrl,
            end,
        }
    }

    <span class="doccomment">/// Splits a `RawIterRange` into two halves.
    ///
    /// Returns `None` if the remaining range is smaller than or equal to the
    /// group width.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    #[cfg(feature = <span class="string">"rayon"</span>)]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>split(<span class="kw-2">mut </span><span class="self">self</span>) -&gt; (<span class="self">Self</span>, <span class="prelude-ty">Option</span>&lt;RawIterRange&lt;T&gt;&gt;) {
        <span class="kw">unsafe </span>{
            <span class="kw">if </span><span class="self">self</span>.end &lt;= <span class="self">self</span>.next_ctrl {
                <span class="comment">// Nothing to split if the group that we are current processing
                // is the last one.
                </span>(<span class="self">self</span>, <span class="prelude-val">None</span>)
            } <span class="kw">else </span>{
                <span class="comment">// len is the remaining number of elements after the group that
                // we are currently processing. It must be a multiple of the
                // group size (small tables are caught by the check above).
                </span><span class="kw">let </span>len = offset_from(<span class="self">self</span>.end, <span class="self">self</span>.next_ctrl);
                <span class="macro">debug_assert_eq!</span>(len % Group::WIDTH, <span class="number">0</span>);

                <span class="comment">// Split the remaining elements into two halves, but round the
                // midpoint down in case there is an odd number of groups
                // remaining. This ensures that:
                // - The tail is at least 1 group long.
                // - The split is roughly even considering we still have the
                //   current group to process.
                </span><span class="kw">let </span>mid = (len / <span class="number">2</span>) &amp; !(Group::WIDTH - <span class="number">1</span>);

                <span class="kw">let </span>tail = <span class="self">Self</span>::new(
                    <span class="self">self</span>.next_ctrl.add(mid),
                    <span class="self">self</span>.data.next_n(Group::WIDTH).next_n(mid),
                    len - mid,
                );
                <span class="macro">debug_assert_eq!</span>(
                    <span class="self">self</span>.data.next_n(Group::WIDTH).next_n(mid).ptr,
                    tail.data.ptr
                );
                <span class="macro">debug_assert_eq!</span>(<span class="self">self</span>.end, tail.end);
                <span class="self">self</span>.end = <span class="self">self</span>.next_ctrl.add(mid);
                <span class="macro">debug_assert_eq!</span>(<span class="self">self</span>.end.add(Group::WIDTH), tail.next_ctrl);
                (<span class="self">self</span>, <span class="prelude-val">Some</span>(tail))
            }
        }
    }

    <span class="doccomment">/// # Safety
    /// If DO_CHECK_PTR_RANGE is false, caller must ensure that we never try to iterate
    /// after yielding all elements.
    </span><span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">unsafe fn </span>next_impl&lt;<span class="kw">const </span>DO_CHECK_PTR_RANGE: bool&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Bucket&lt;T&gt;&gt; {
        <span class="kw">loop </span>{
            <span class="kw">if let </span><span class="prelude-val">Some</span>(index) = <span class="self">self</span>.current_group.next() {
                <span class="kw">return </span><span class="prelude-val">Some</span>(<span class="self">self</span>.data.next_n(index));
            }

            <span class="kw">if </span>DO_CHECK_PTR_RANGE &amp;&amp; <span class="self">self</span>.next_ctrl &gt;= <span class="self">self</span>.end {
                <span class="kw">return </span><span class="prelude-val">None</span>;
            }

            <span class="comment">// We might read past self.end up to the next group boundary,
            // but this is fine because it only occurs on tables smaller
            // than the group size where the trailing control bytes are all
            // EMPTY. On larger tables self.end is guaranteed to be aligned
            // to the group size (since tables are power-of-two sized).
            </span><span class="self">self</span>.current_group = Group::load_aligned(<span class="self">self</span>.next_ctrl).match_full().into_iter();
            <span class="self">self</span>.data = <span class="self">self</span>.data.next_n(Group::WIDTH);
            <span class="self">self</span>.next_ctrl = <span class="self">self</span>.next_ctrl.add(Group::WIDTH);
        }
    }

    <span class="doccomment">/// Folds every element into an accumulator by applying an operation,
    /// returning the final result.
    ///
    /// `fold_impl()` takes three arguments: the number of items remaining in
    /// the iterator, an initial value, and a closure with two arguments: an
    /// 'accumulator', and an element. The closure returns the value that the
    /// accumulator should have for the next iteration.
    ///
    /// The initial value is the value the accumulator will have on the first call.
    ///
    /// After applying this closure to every element of the iterator, `fold_impl()`
    /// returns the accumulator.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is
    /// [`Undefined Behavior`]:
    ///
    /// * The [`RawTableInner`] / [`RawTable`] must be alive and not moved,
    ///   i.e. table outlives the `RawIterRange`;
    ///
    /// * The provided `n` value must match the actual number of items
    ///   in the table.
    ///
    /// [`Undefined Behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[allow(clippy::while_let_on_iterator)]
    #[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">unsafe fn </span>fold_impl&lt;F, B&gt;(<span class="kw-2">mut </span><span class="self">self</span>, <span class="kw-2">mut </span>n: usize, <span class="kw-2">mut </span>acc: B, <span class="kw-2">mut </span>f: F) -&gt; B
    <span class="kw">where
        </span>F: FnMut(B, Bucket&lt;T&gt;) -&gt; B,
    {
        <span class="kw">loop </span>{
            <span class="kw">while let </span><span class="prelude-val">Some</span>(index) = <span class="self">self</span>.current_group.next() {
                <span class="comment">// The returned `index` will always be in the range `0..Group::WIDTH`,
                // so that calling `self.data.next_n(index)` is safe (see detailed explanation below).
                </span><span class="macro">debug_assert!</span>(n != <span class="number">0</span>);
                <span class="kw">let </span>bucket = <span class="self">self</span>.data.next_n(index);
                acc = f(acc, bucket);
                n -= <span class="number">1</span>;
            }

            <span class="kw">if </span>n == <span class="number">0 </span>{
                <span class="kw">return </span>acc;
            }

            <span class="comment">// SAFETY: The caller of this function ensures that:
            //
            // 1. The provided `n` value matches the actual number of items in the table;
            // 2. The table is alive and did not moved.
            //
            // Taking the above into account, we always stay within the bounds, because:
            //
            // 1. For tables smaller than the group width (self.buckets() &lt;= Group::WIDTH),
            //    we will never end up in the given branch, since we should have already
            //    yielded all the elements of the table.
            //
            // 2. For tables larger than the group width. The the number of buckets is a
            //    power of two (2 ^ n), Group::WIDTH is also power of two (2 ^ k). Sinse
            //    `(2 ^ n) &gt; (2 ^ k)`, than `(2 ^ n) % (2 ^ k) = 0`. As we start from the
            //    the start of the array of control bytes, and never try to iterate after
            //    getting all the elements, the last `self.current_group` will read bytes
            //    from the `self.buckets() - Group::WIDTH` index.  We know also that
            //    `self.current_group.next()` will always retun indices within the range
            //    `0..Group::WIDTH`.
            //
            //    Knowing all of the above and taking into account that we are synchronizing
            //    the `self.data` index with the index we used to read the `self.current_group`,
            //    the subsequent `self.data.next_n(index)` will always return a bucket with
            //    an index number less than `self.buckets()`.
            //
            //    The last `self.next_ctrl`, whose index would be `self.buckets()`, will never
            //    actually be read, since we should have already yielded all the elements of
            //    the table.
            </span><span class="self">self</span>.current_group = Group::load_aligned(<span class="self">self</span>.next_ctrl).match_full().into_iter();
            <span class="self">self</span>.data = <span class="self">self</span>.data.next_n(Group::WIDTH);
            <span class="self">self</span>.next_ctrl = <span class="self">self</span>.next_ctrl.add(Group::WIDTH);
        }
    }
}

<span class="comment">// We make raw iterators unconditionally Send and Sync, and let the PhantomData
// in the actual iterator implementations determine the real Send/Sync bounds.
</span><span class="kw">unsafe impl</span>&lt;T&gt; Send <span class="kw">for </span>RawIterRange&lt;T&gt; {}
<span class="kw">unsafe impl</span>&lt;T&gt; Sync <span class="kw">for </span>RawIterRange&lt;T&gt; {}

<span class="kw">impl</span>&lt;T&gt; Clone <span class="kw">for </span>RawIterRange&lt;T&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>clone(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            data: <span class="self">self</span>.data.clone(),
            next_ctrl: <span class="self">self</span>.next_ctrl,
            current_group: <span class="self">self</span>.current_group,
            end: <span class="self">self</span>.end,
        }
    }
}

<span class="kw">impl</span>&lt;T&gt; Iterator <span class="kw">for </span>RawIterRange&lt;T&gt; {
    <span class="kw">type </span>Item = Bucket&lt;T&gt;;

    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Bucket&lt;T&gt;&gt; {
        <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY: We set checker flag to true.
            </span><span class="self">self</span>.next_impl::&lt;<span class="bool-val">true</span>&gt;()
        }
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        <span class="comment">// We don't have an item count, so just guess based on the range size.
        </span><span class="kw">let </span>remaining_buckets = <span class="kw">if </span><span class="self">self</span>.end &gt; <span class="self">self</span>.next_ctrl {
            <span class="kw">unsafe </span>{ offset_from(<span class="self">self</span>.end, <span class="self">self</span>.next_ctrl) }
        } <span class="kw">else </span>{
            <span class="number">0
        </span>};

        <span class="comment">// Add a group width to include the group we are currently processing.
        </span>(<span class="number">0</span>, <span class="prelude-val">Some</span>(Group::WIDTH + remaining_buckets))
    }
}

<span class="kw">impl</span>&lt;T&gt; FusedIterator <span class="kw">for </span>RawIterRange&lt;T&gt; {}

<span class="doccomment">/// Iterator which returns a raw pointer to every full bucket in the table.
///
/// For maximum flexibility this iterator is not bound by a lifetime, but you
/// must observe several rules when using it:
/// - You must not free the hash table while iterating (including via growing/shrinking).
/// - It is fine to erase a bucket that has been yielded by the iterator.
/// - Erasing a bucket that has not yet been yielded by the iterator may still
///   result in the iterator yielding that bucket (unless `reflect_remove` is called).
/// - It is unspecified whether an element inserted after the iterator was
///   created will be yielded by that iterator (unless `reflect_insert` is called).
/// - The order in which the iterator yields bucket is unspecified and may
///   change in the future.
</span><span class="kw">pub struct </span>RawIter&lt;T&gt; {
    <span class="kw">pub</span>(<span class="kw">crate</span>) iter: RawIterRange&lt;T&gt;,
    items: usize,
}

<span class="kw">impl</span>&lt;T&gt; RawIter&lt;T&gt; {
    <span class="doccomment">/// Refresh the iterator so that it reflects a removal from the given bucket.
    ///
    /// For the iterator to remain valid, this method must be called once
    /// for each removed bucket before `next` is called again.
    ///
    /// This method should be called _before_ the removal is made. It is not necessary to call this
    /// method if you are removing an item that this iterator yielded in the past.
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">pub unsafe fn </span>reflect_remove(<span class="kw-2">&amp;mut </span><span class="self">self</span>, b: <span class="kw-2">&amp;</span>Bucket&lt;T&gt;) {
        <span class="self">self</span>.reflect_toggle_full(b, <span class="bool-val">false</span>);
    }

    <span class="doccomment">/// Refresh the iterator so that it reflects an insertion into the given bucket.
    ///
    /// For the iterator to remain valid, this method must be called once
    /// for each insert before `next` is called again.
    ///
    /// This method does not guarantee that an insertion of a bucket with a greater
    /// index than the last one yielded will be reflected in the iterator.
    ///
    /// This method should be called _after_ the given insert is made.
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">pub unsafe fn </span>reflect_insert(<span class="kw-2">&amp;mut </span><span class="self">self</span>, b: <span class="kw-2">&amp;</span>Bucket&lt;T&gt;) {
        <span class="self">self</span>.reflect_toggle_full(b, <span class="bool-val">true</span>);
    }

    <span class="doccomment">/// Refresh the iterator so that it reflects a change to the state of the given bucket.
    </span><span class="attr">#[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">unsafe fn </span>reflect_toggle_full(<span class="kw-2">&amp;mut </span><span class="self">self</span>, b: <span class="kw-2">&amp;</span>Bucket&lt;T&gt;, is_insert: bool) {
        <span class="kw">if </span>b.as_ptr() &gt; <span class="self">self</span>.iter.data.as_ptr() {
            <span class="comment">// The iterator has already passed the bucket's group.
            // So the toggle isn't relevant to this iterator.
            </span><span class="kw">return</span>;
        }

        <span class="kw">if </span><span class="self">self</span>.iter.next_ctrl &lt; <span class="self">self</span>.iter.end
            &amp;&amp; b.as_ptr() &lt;= <span class="self">self</span>.iter.data.next_n(Group::WIDTH).as_ptr()
        {
            <span class="comment">// The iterator has not yet reached the bucket's group.
            // We don't need to reload anything, but we do need to adjust the item count.

            </span><span class="kw">if </span><span class="macro">cfg!</span>(debug_assertions) {
                <span class="comment">// Double-check that the user isn't lying to us by checking the bucket state.
                // To do that, we need to find its control byte. We know that self.iter.data is
                // at self.iter.next_ctrl - Group::WIDTH, so we work from there:
                </span><span class="kw">let </span>offset = offset_from(<span class="self">self</span>.iter.data.as_ptr(), b.as_ptr());
                <span class="kw">let </span>ctrl = <span class="self">self</span>.iter.next_ctrl.sub(Group::WIDTH).add(offset);
                <span class="comment">// This method should be called _before_ a removal, or _after_ an insert,
                // so in both cases the ctrl byte should indicate that the bucket is full.
                </span><span class="macro">assert!</span>(is_full(<span class="kw-2">*</span>ctrl));
            }

            <span class="kw">if </span>is_insert {
                <span class="self">self</span>.items += <span class="number">1</span>;
            } <span class="kw">else </span>{
                <span class="self">self</span>.items -= <span class="number">1</span>;
            }

            <span class="kw">return</span>;
        }

        <span class="comment">// The iterator is at the bucket group that the toggled bucket is in.
        // We need to do two things:
        //
        //  - Determine if the iterator already yielded the toggled bucket.
        //    If it did, we're done.
        //  - Otherwise, update the iterator cached group so that it won't
        //    yield a to-be-removed bucket, or _will_ yield a to-be-added bucket.
        //    We'll also need to update the item count accordingly.
        </span><span class="kw">if let </span><span class="prelude-val">Some</span>(index) = <span class="self">self</span>.iter.current_group.<span class="number">0</span>.lowest_set_bit() {
            <span class="kw">let </span>next_bucket = <span class="self">self</span>.iter.data.next_n(index);
            <span class="kw">if </span>b.as_ptr() &gt; next_bucket.as_ptr() {
                <span class="comment">// The toggled bucket is "before" the bucket the iterator would yield next. We
                // therefore don't need to do anything --- the iterator has already passed the
                // bucket in question.
                //
                // The item count must already be correct, since a removal or insert "prior" to
                // the iterator's position wouldn't affect the item count.
            </span>} <span class="kw">else </span>{
                <span class="comment">// The removed bucket is an upcoming bucket. We need to make sure it does _not_
                // get yielded, and also that it's no longer included in the item count.
                //
                // NOTE: We can't just reload the group here, both since that might reflect
                // inserts we've already passed, and because that might inadvertently unset the
                // bits for _other_ removals. If we do that, we'd have to also decrement the
                // item count for those other bits that we unset. But the presumably subsequent
                // call to reflect for those buckets might _also_ decrement the item count.
                // Instead, we _just_ flip the bit for the particular bucket the caller asked
                // us to reflect.
                </span><span class="kw">let </span>our_bit = offset_from(<span class="self">self</span>.iter.data.as_ptr(), b.as_ptr());
                <span class="kw">let </span>was_full = <span class="self">self</span>.iter.current_group.flip(our_bit);
                <span class="macro">debug_assert_ne!</span>(was_full, is_insert);

                <span class="kw">if </span>is_insert {
                    <span class="self">self</span>.items += <span class="number">1</span>;
                } <span class="kw">else </span>{
                    <span class="self">self</span>.items -= <span class="number">1</span>;
                }

                <span class="kw">if </span><span class="macro">cfg!</span>(debug_assertions) {
                    <span class="kw">if </span>b.as_ptr() == next_bucket.as_ptr() {
                        <span class="comment">// The removed bucket should no longer be next
                        </span><span class="macro">debug_assert_ne!</span>(<span class="self">self</span>.iter.current_group.<span class="number">0</span>.lowest_set_bit(), <span class="prelude-val">Some</span>(index));
                    } <span class="kw">else </span>{
                        <span class="comment">// We should not have changed what bucket comes next.
                        </span><span class="macro">debug_assert_eq!</span>(<span class="self">self</span>.iter.current_group.<span class="number">0</span>.lowest_set_bit(), <span class="prelude-val">Some</span>(index));
                    }
                }
            }
        } <span class="kw">else </span>{
            <span class="comment">// We must have already iterated past the removed item.
        </span>}
    }

    <span class="kw">unsafe fn </span>drop_elements(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">if </span>T::NEEDS_DROP &amp;&amp; <span class="self">self</span>.items != <span class="number">0 </span>{
            <span class="kw">for </span>item <span class="kw">in </span><span class="self">self </span>{
                item.drop();
            }
        }
    }
}

<span class="kw">impl</span>&lt;T&gt; Clone <span class="kw">for </span>RawIter&lt;T&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>clone(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            iter: <span class="self">self</span>.iter.clone(),
            items: <span class="self">self</span>.items,
        }
    }
}

<span class="kw">impl</span>&lt;T&gt; Iterator <span class="kw">for </span>RawIter&lt;T&gt; {
    <span class="kw">type </span>Item = Bucket&lt;T&gt;;

    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Bucket&lt;T&gt;&gt; {
        <span class="comment">// Inner iterator iterates over buckets
        // so it can do unnecessary work if we already yielded all items.
        </span><span class="kw">if </span><span class="self">self</span>.items == <span class="number">0 </span>{
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }

        <span class="kw">let </span>nxt = <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY: We check number of items to yield using `items` field.
            </span><span class="self">self</span>.iter.next_impl::&lt;<span class="bool-val">false</span>&gt;()
        };

        <span class="macro">debug_assert!</span>(nxt.is_some());
        <span class="self">self</span>.items -= <span class="number">1</span>;

        nxt
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        (<span class="self">self</span>.items, <span class="prelude-val">Some</span>(<span class="self">self</span>.items))
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>fold&lt;B, F&gt;(<span class="self">self</span>, init: B, f: F) -&gt; B
    <span class="kw">where
        </span><span class="self">Self</span>: Sized,
        F: FnMut(B, <span class="self">Self</span>::Item) -&gt; B,
    {
        <span class="kw">unsafe </span>{ <span class="self">self</span>.iter.fold_impl(<span class="self">self</span>.items, init, f) }
    }
}

<span class="kw">impl</span>&lt;T&gt; ExactSizeIterator <span class="kw">for </span>RawIter&lt;T&gt; {}
<span class="kw">impl</span>&lt;T&gt; FusedIterator <span class="kw">for </span>RawIter&lt;T&gt; {}

<span class="doccomment">/// Iterator which returns an index of every full bucket in the table.
///
/// For maximum flexibility this iterator is not bound by a lifetime, but you
/// must observe several rules when using it:
/// - You must not free the hash table while iterating (including via growing/shrinking).
/// - It is fine to erase a bucket that has been yielded by the iterator.
/// - Erasing a bucket that has not yet been yielded by the iterator may still
///   result in the iterator yielding index of that bucket.
/// - It is unspecified whether an element inserted after the iterator was
///   created will be yielded by that iterator.
/// - The order in which the iterator yields indices of the buckets is unspecified
///   and may change in the future.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>FullBucketsIndices {
    <span class="comment">// Mask of full buckets in the current group. Bits are cleared from this
    // mask as each element is processed.
    </span>current_group: BitMaskIter,

    <span class="comment">// Initial value of the bytes' indices of the current group (relative
    // to the start of the control bytes).
    </span>group_first_index: usize,

    <span class="comment">// Pointer to the current group of control bytes,
    // Must be aligned to the group size (Group::WIDTH).
    </span>ctrl: NonNull&lt;u8&gt;,

    <span class="comment">// Number of elements in the table.
    </span>items: usize,
}

<span class="kw">impl </span>FullBucketsIndices {
    <span class="doccomment">/// Advances the iterator and returns the next value.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is
    /// [`Undefined Behavior`]:
    ///
    /// * The [`RawTableInner`] / [`RawTable`] must be alive and not moved,
    ///   i.e. table outlives the `FullBucketsIndices`;
    ///
    /// * It never tries to iterate after getting all elements.
    ///
    /// [`Undefined Behavior`]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">unsafe fn </span>next_impl(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="kw">loop </span>{
            <span class="kw">if let </span><span class="prelude-val">Some</span>(index) = <span class="self">self</span>.current_group.next() {
                <span class="comment">// The returned `self.group_first_index + index` will always
                // be in the range `0..self.buckets()`. See explanation below.
                </span><span class="kw">return </span><span class="prelude-val">Some</span>(<span class="self">self</span>.group_first_index + index);
            }

            <span class="comment">// SAFETY: The caller of this function ensures that:
            //
            // 1. It never tries to iterate after getting all the elements;
            // 2. The table is alive and did not moved;
            // 3. The first `self.ctrl` pointed to the start of the array of control bytes.
            //
            // Taking the above into account, we always stay within the bounds, because:
            //
            // 1. For tables smaller than the group width (self.buckets() &lt;= Group::WIDTH),
            //    we will never end up in the given branch, since we should have already
            //    yielded all the elements of the table.
            //
            // 2. For tables larger than the group width. The the number of buckets is a
            //    power of two (2 ^ n), Group::WIDTH is also power of two (2 ^ k). Sinse
            //    `(2 ^ n) &gt; (2 ^ k)`, than `(2 ^ n) % (2 ^ k) = 0`. As we start from the
            //    the start of the array of control bytes, and never try to iterate after
            //    getting all the elements, the last `self.ctrl` will be equal to
            //    the `self.buckets() - Group::WIDTH`, so `self.current_group.next()`
            //    will always contains indices within the range `0..Group::WIDTH`,
            //    and subsequent `self.group_first_index + index` will always return a
            //    number less than `self.buckets()`.
            </span><span class="self">self</span>.ctrl = NonNull::new_unchecked(<span class="self">self</span>.ctrl.as_ptr().add(Group::WIDTH));

            <span class="comment">// SAFETY: See explanation above.
            </span><span class="self">self</span>.current_group = Group::load_aligned(<span class="self">self</span>.ctrl.as_ptr())
                .match_full()
                .into_iter();
            <span class="self">self</span>.group_first_index += Group::WIDTH;
        }
    }
}

<span class="kw">impl </span>Iterator <span class="kw">for </span>FullBucketsIndices {
    <span class="kw">type </span>Item = usize;

    <span class="doccomment">/// Advances the iterator and returns the next value. It is up to
    /// the caller to ensure that the `RawTable` outlives the `FullBucketsIndices`,
    /// because we cannot make the `next` method unsafe.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="comment">// Return if we already yielded all items.
        </span><span class="kw">if </span><span class="self">self</span>.items == <span class="number">0 </span>{
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }

        <span class="kw">let </span>nxt = <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY:
            // 1. We check number of items to yield using `items` field.
            // 2. The caller ensures that the table is alive and has not moved.
            </span><span class="self">self</span>.next_impl()
        };

        <span class="macro">debug_assert!</span>(nxt.is_some());
        <span class="self">self</span>.items -= <span class="number">1</span>;

        nxt
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        (<span class="self">self</span>.items, <span class="prelude-val">Some</span>(<span class="self">self</span>.items))
    }
}

<span class="kw">impl </span>ExactSizeIterator <span class="kw">for </span>FullBucketsIndices {}
<span class="kw">impl </span>FusedIterator <span class="kw">for </span>FullBucketsIndices {}

<span class="doccomment">/// Iterator which consumes a table and returns elements.
</span><span class="kw">pub struct </span>RawIntoIter&lt;T, A: Allocator = Global&gt; {
    iter: RawIter&lt;T&gt;,
    allocation: <span class="prelude-ty">Option</span>&lt;(NonNull&lt;u8&gt;, Layout, A)&gt;,
    marker: PhantomData&lt;T&gt;,
}

<span class="kw">impl</span>&lt;T, A: Allocator&gt; RawIntoIter&lt;T, A&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>iter(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; RawIter&lt;T&gt; {
        <span class="self">self</span>.iter.clone()
    }
}

<span class="kw">unsafe impl</span>&lt;T, A: Allocator&gt; Send <span class="kw">for </span>RawIntoIter&lt;T, A&gt;
<span class="kw">where
    </span>T: Send,
    A: Send,
{
}
<span class="kw">unsafe impl</span>&lt;T, A: Allocator&gt; Sync <span class="kw">for </span>RawIntoIter&lt;T, A&gt;
<span class="kw">where
    </span>T: Sync,
    A: Sync,
{
}

<span class="attr">#[cfg(feature = <span class="string">"nightly"</span>)]
</span><span class="kw">unsafe impl</span>&lt;<span class="attr">#[may_dangle] </span>T, A: Allocator&gt; Drop <span class="kw">for </span>RawIntoIter&lt;T, A&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">unsafe </span>{
            <span class="comment">// Drop all remaining elements
            </span><span class="self">self</span>.iter.drop_elements();

            <span class="comment">// Free the table
            </span><span class="kw">if let </span><span class="prelude-val">Some</span>((ptr, layout, <span class="kw-2">ref </span>alloc)) = <span class="self">self</span>.allocation {
                alloc.deallocate(ptr, layout);
            }
        }
    }
}
<span class="attr">#[cfg(not(feature = <span class="string">"nightly"</span>))]
</span><span class="kw">impl</span>&lt;T, A: Allocator&gt; Drop <span class="kw">for </span>RawIntoIter&lt;T, A&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">unsafe </span>{
            <span class="comment">// Drop all remaining elements
            </span><span class="self">self</span>.iter.drop_elements();

            <span class="comment">// Free the table
            </span><span class="kw">if let </span><span class="prelude-val">Some</span>((ptr, layout, <span class="kw-2">ref </span>alloc)) = <span class="self">self</span>.allocation {
                alloc.deallocate(ptr, layout);
            }
        }
    }
}

<span class="kw">impl</span>&lt;T, A: Allocator&gt; Iterator <span class="kw">for </span>RawIntoIter&lt;T, A&gt; {
    <span class="kw">type </span>Item = T;

    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;T&gt; {
        <span class="kw">unsafe </span>{ <span class="prelude-val">Some</span>(<span class="self">self</span>.iter.next()<span class="question-mark">?</span>.read()) }
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        <span class="self">self</span>.iter.size_hint()
    }
}

<span class="kw">impl</span>&lt;T, A: Allocator&gt; ExactSizeIterator <span class="kw">for </span>RawIntoIter&lt;T, A&gt; {}
<span class="kw">impl</span>&lt;T, A: Allocator&gt; FusedIterator <span class="kw">for </span>RawIntoIter&lt;T, A&gt; {}

<span class="doccomment">/// Iterator which consumes elements without freeing the table storage.
</span><span class="kw">pub struct </span>RawDrain&lt;<span class="lifetime">'a</span>, T, A: Allocator = Global&gt; {
    iter: RawIter&lt;T&gt;,

    <span class="comment">// The table is moved into the iterator for the duration of the drain. This
    // ensures that an empty table is left if the drain iterator is leaked
    // without dropping.
    </span>table: RawTableInner,
    orig_table: NonNull&lt;RawTableInner&gt;,

    <span class="comment">// We don't use a &amp;'a mut RawTable&lt;T&gt; because we want RawDrain to be
    // covariant over T.
    </span>marker: PhantomData&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>RawTable&lt;T, A&gt;&gt;,
}

<span class="kw">impl</span>&lt;T, A: Allocator&gt; RawDrain&lt;<span class="lifetime">'_</span>, T, A&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub fn </span>iter(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; RawIter&lt;T&gt; {
        <span class="self">self</span>.iter.clone()
    }
}

<span class="kw">unsafe impl</span>&lt;T, A: Allocator&gt; Send <span class="kw">for </span>RawDrain&lt;<span class="lifetime">'_</span>, T, A&gt;
<span class="kw">where
    </span>T: Send,
    A: Send,
{
}
<span class="kw">unsafe impl</span>&lt;T, A: Allocator&gt; Sync <span class="kw">for </span>RawDrain&lt;<span class="lifetime">'_</span>, T, A&gt;
<span class="kw">where
    </span>T: Sync,
    A: Sync,
{
}

<span class="kw">impl</span>&lt;T, A: Allocator&gt; Drop <span class="kw">for </span>RawDrain&lt;<span class="lifetime">'_</span>, T, A&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">unsafe </span>{
            <span class="comment">// Drop all remaining elements. Note that this may panic.
            </span><span class="self">self</span>.iter.drop_elements();

            <span class="comment">// Reset the contents of the table now that all elements have been
            // dropped.
            </span><span class="self">self</span>.table.clear_no_drop();

            <span class="comment">// Move the now empty table back to its original location.
            </span><span class="self">self</span>.orig_table
                .as_ptr()
                .copy_from_nonoverlapping(<span class="kw-2">&amp;</span><span class="self">self</span>.table, <span class="number">1</span>);
        }
    }
}

<span class="kw">impl</span>&lt;T, A: Allocator&gt; Iterator <span class="kw">for </span>RawDrain&lt;<span class="lifetime">'_</span>, T, A&gt; {
    <span class="kw">type </span>Item = T;

    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;T&gt; {
        <span class="kw">unsafe </span>{
            <span class="kw">let </span>item = <span class="self">self</span>.iter.next()<span class="question-mark">?</span>;
            <span class="prelude-val">Some</span>(item.read())
        }
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        <span class="self">self</span>.iter.size_hint()
    }
}

<span class="kw">impl</span>&lt;T, A: Allocator&gt; ExactSizeIterator <span class="kw">for </span>RawDrain&lt;<span class="lifetime">'_</span>, T, A&gt; {}
<span class="kw">impl</span>&lt;T, A: Allocator&gt; FusedIterator <span class="kw">for </span>RawDrain&lt;<span class="lifetime">'_</span>, T, A&gt; {}

<span class="doccomment">/// Iterator over occupied buckets that could match a given hash.
///
/// `RawTable` only stores 7 bits of the hash value, so this iterator may return
/// items that have a hash value different than the one provided. You should
/// always validate the returned values before using them.
///
/// For maximum flexibility this iterator is not bound by a lifetime, but you
/// must observe several rules when using it:
/// - You must not free the hash table while iterating (including via growing/shrinking).
/// - It is fine to erase a bucket that has been yielded by the iterator.
/// - Erasing a bucket that has not yet been yielded by the iterator may still
///   result in the iterator yielding that bucket.
/// - It is unspecified whether an element inserted after the iterator was
///   created will be yielded by that iterator.
/// - The order in which the iterator yields buckets is unspecified and may
///   change in the future.
</span><span class="kw">pub struct </span>RawIterHash&lt;T&gt; {
    inner: RawIterHashInner,
    _marker: PhantomData&lt;T&gt;,
}

<span class="kw">struct </span>RawIterHashInner {
    <span class="comment">// See `RawTableInner`'s corresponding fields for details.
    // We can't store a `*const RawTableInner` as it would get
    // invalidated by the user calling `&amp;mut` methods on `RawTable`.
    </span>bucket_mask: usize,
    ctrl: NonNull&lt;u8&gt;,

    <span class="comment">// The top 7 bits of the hash.
    </span>h2_hash: u8,

    <span class="comment">// The sequence of groups to probe in the search.
    </span>probe_seq: ProbeSeq,

    group: Group,

    <span class="comment">// The elements within the group with a matching h2-hash.
    </span>bitmask: BitMaskIter,
}

<span class="kw">impl</span>&lt;T&gt; RawIterHash&lt;T&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    #[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">unsafe fn </span>new&lt;A: Allocator&gt;(table: <span class="kw-2">&amp;</span>RawTable&lt;T, A&gt;, hash: u64) -&gt; <span class="self">Self </span>{
        RawIterHash {
            inner: RawIterHashInner::new(<span class="kw-2">&amp;</span>table.table, hash),
            _marker: PhantomData,
        }
    }
}
<span class="kw">impl </span>RawIterHashInner {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    #[cfg(feature = <span class="string">"raw"</span>)]
    </span><span class="kw">unsafe fn </span>new(table: <span class="kw-2">&amp;</span>RawTableInner, hash: u64) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>h2_hash = h2(hash);
        <span class="kw">let </span>probe_seq = table.probe_seq(hash);
        <span class="kw">let </span>group = Group::load(table.ctrl(probe_seq.pos));
        <span class="kw">let </span>bitmask = group.match_byte(h2_hash).into_iter();

        RawIterHashInner {
            bucket_mask: table.bucket_mask,
            ctrl: table.ctrl,
            h2_hash,
            probe_seq,
            group,
            bitmask,
        }
    }
}

<span class="kw">impl</span>&lt;T&gt; Iterator <span class="kw">for </span>RawIterHash&lt;T&gt; {
    <span class="kw">type </span>Item = Bucket&lt;T&gt;;

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Bucket&lt;T&gt;&gt; {
        <span class="kw">unsafe </span>{
            <span class="kw">match </span><span class="self">self</span>.inner.next() {
                <span class="prelude-val">Some</span>(index) =&gt; {
                    <span class="comment">// Can't use `RawTable::bucket` here as we don't have
                    // an actual `RawTable` reference to use.
                    </span><span class="macro">debug_assert!</span>(index &lt;= <span class="self">self</span>.inner.bucket_mask);
                    <span class="kw">let </span>bucket = Bucket::from_base_index(<span class="self">self</span>.inner.ctrl.cast(), index);
                    <span class="prelude-val">Some</span>(bucket)
                }
                <span class="prelude-val">None </span>=&gt; <span class="prelude-val">None</span>,
            }
        }
    }
}

<span class="kw">impl </span>Iterator <span class="kw">for </span>RawIterHashInner {
    <span class="kw">type </span>Item = usize;

    <span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt; {
        <span class="kw">unsafe </span>{
            <span class="kw">loop </span>{
                <span class="kw">if let </span><span class="prelude-val">Some</span>(bit) = <span class="self">self</span>.bitmask.next() {
                    <span class="kw">let </span>index = (<span class="self">self</span>.probe_seq.pos + bit) &amp; <span class="self">self</span>.bucket_mask;
                    <span class="kw">return </span><span class="prelude-val">Some</span>(index);
                }
                <span class="kw">if </span>likely(<span class="self">self</span>.group.match_empty().any_bit_set()) {
                    <span class="kw">return </span><span class="prelude-val">None</span>;
                }
                <span class="self">self</span>.probe_seq.move_next(<span class="self">self</span>.bucket_mask);

                <span class="comment">// Can't use `RawTableInner::ctrl` here as we don't have
                // an actual `RawTableInner` reference to use.
                </span><span class="kw">let </span>index = <span class="self">self</span>.probe_seq.pos;
                <span class="macro">debug_assert!</span>(index &lt; <span class="self">self</span>.bucket_mask + <span class="number">1 </span>+ Group::WIDTH);
                <span class="kw">let </span>group_ctrl = <span class="self">self</span>.ctrl.as_ptr().add(index);

                <span class="self">self</span>.group = Group::load(group_ctrl);
                <span class="self">self</span>.bitmask = <span class="self">self</span>.group.match_byte(<span class="self">self</span>.h2_hash).into_iter();
            }
        }
    }
}

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>RawExtractIf&lt;<span class="lifetime">'a</span>, T, A: Allocator&gt; {
    <span class="kw">pub </span>iter: RawIter&lt;T&gt;,
    <span class="kw">pub </span>table: <span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>RawTable&lt;T, A&gt;,
}

<span class="kw">impl</span>&lt;T, A: Allocator&gt; RawExtractIf&lt;<span class="lifetime">'_</span>, T, A&gt; {
    <span class="attr">#[cfg_attr(feature = <span class="string">"inline-more"</span>, inline)]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>next&lt;F&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw-2">mut </span>f: F) -&gt; <span class="prelude-ty">Option</span>&lt;T&gt;
    <span class="kw">where
        </span>F: FnMut(<span class="kw-2">&amp;mut </span>T) -&gt; bool,
    {
        <span class="kw">unsafe </span>{
            <span class="kw">for </span>item <span class="kw">in </span><span class="kw-2">&amp;mut </span><span class="self">self</span>.iter {
                <span class="kw">if </span>f(item.as_mut()) {
                    <span class="kw">return </span><span class="prelude-val">Some</span>(<span class="self">self</span>.table.remove(item).<span class="number">0</span>);
                }
            }
        }
        <span class="prelude-val">None
    </span>}
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>test_map {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="kw">fn </span>rehash_in_place&lt;T&gt;(table: <span class="kw-2">&amp;mut </span>RawTable&lt;T&gt;, hasher: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; u64) {
        <span class="kw">unsafe </span>{
            table.table.rehash_in_place(
                <span class="kw-2">&amp;</span>|table, index| hasher(table.bucket::&lt;T&gt;(index).as_ref()),
                mem::size_of::&lt;T&gt;(),
                <span class="kw">if </span>mem::needs_drop::&lt;T&gt;() {
                    <span class="prelude-val">Some</span>(mem::transmute(ptr::drop_in_place::&lt;T&gt; <span class="kw">as unsafe fn</span>(<span class="kw-2">*mut </span>T)))
                } <span class="kw">else </span>{
                    <span class="prelude-val">None
                </span>},
            );
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>rehash() {
        <span class="kw">let </span><span class="kw-2">mut </span>table = RawTable::new();
        <span class="kw">let </span>hasher = |i: <span class="kw-2">&amp;</span>u64| <span class="kw-2">*</span>i;
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">100 </span>{
            table.insert(i, i, hasher);
        }

        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">100 </span>{
            <span class="kw">unsafe </span>{
                <span class="macro">assert_eq!</span>(table.find(i, |x| <span class="kw-2">*</span>x == i).map(|b| b.read()), <span class="prelude-val">Some</span>(i));
            }
            <span class="macro">assert!</span>(table.find(i + <span class="number">100</span>, |x| <span class="kw-2">*</span>x == i + <span class="number">100</span>).is_none());
        }

        rehash_in_place(<span class="kw-2">&amp;mut </span>table, hasher);

        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">100 </span>{
            <span class="kw">unsafe </span>{
                <span class="macro">assert_eq!</span>(table.find(i, |x| <span class="kw-2">*</span>x == i).map(|b| b.read()), <span class="prelude-val">Some</span>(i));
            }
            <span class="macro">assert!</span>(table.find(i + <span class="number">100</span>, |x| <span class="kw-2">*</span>x == i + <span class="number">100</span>).is_none());
        }
    }

    <span class="doccomment">/// CHECKING THAT WE ARE NOT TRYING TO READ THE MEMORY OF
    /// AN UNINITIALIZED TABLE DURING THE DROP
    </span><span class="attr">#[test]
    </span><span class="kw">fn </span>test_drop_uninitialized() {
        <span class="kw">use </span>::alloc::vec::Vec;

        <span class="kw">let </span>table = <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY: The `buckets` is power of two and we're not
            // trying to actually use the returned RawTable.
            </span>RawTable::&lt;(u64, Vec&lt;i32&gt;)&gt;::new_uninitialized(Global, <span class="number">8</span>, Fallibility::Infallible)
                .unwrap()
        };
        drop(table);
    }

    <span class="doccomment">/// CHECKING THAT WE DON'T TRY TO DROP DATA IF THE `ITEMS`
    /// ARE ZERO, EVEN IF WE HAVE `FULL` CONTROL BYTES.
    </span><span class="attr">#[test]
    </span><span class="kw">fn </span>test_drop_zero_items() {
        <span class="kw">use </span>::alloc::vec::Vec;
        <span class="kw">unsafe </span>{
            <span class="comment">// SAFETY: The `buckets` is power of two and we're not
            // trying to actually use the returned RawTable.
            </span><span class="kw">let </span>table =
                RawTable::&lt;(u64, Vec&lt;i32&gt;)&gt;::new_uninitialized(Global, <span class="number">8</span>, Fallibility::Infallible)
                    .unwrap();

            <span class="comment">// WE SIMULATE, AS IT WERE, A FULL TABLE.

            // SAFETY: We checked that the table is allocated and therefore the table already has
            // `self.bucket_mask + 1 + Group::WIDTH` number of control bytes (see TableLayout::calculate_layout_for)
            // so writing `table.table.num_ctrl_bytes() == bucket_mask + 1 + Group::WIDTH` bytes is safe.
            </span>table
                .table
                .ctrl(<span class="number">0</span>)
                .write_bytes(EMPTY, table.table.num_ctrl_bytes());

            <span class="comment">// SAFETY: table.capacity() is guaranteed to be smaller than table.buckets()
            </span>table.table.ctrl(<span class="number">0</span>).write_bytes(<span class="number">0</span>, table.capacity());

            <span class="comment">// Fix up the trailing control bytes. See the comments in set_ctrl
            // for the handling of tables smaller than the group width.
            </span><span class="kw">if </span>table.buckets() &lt; Group::WIDTH {
                <span class="comment">// SAFETY: We have `self.bucket_mask + 1 + Group::WIDTH` number of control bytes,
                // so copying `self.buckets() == self.bucket_mask + 1` bytes with offset equal to
                // `Group::WIDTH` is safe
                </span>table
                    .table
                    .ctrl(<span class="number">0</span>)
                    .copy_to(table.table.ctrl(Group::WIDTH), table.table.buckets());
            } <span class="kw">else </span>{
                <span class="comment">// SAFETY: We have `self.bucket_mask + 1 + Group::WIDTH` number of
                // control bytes,so copying `Group::WIDTH` bytes with offset equal
                // to `self.buckets() == self.bucket_mask + 1` is safe
                </span>table
                    .table
                    .ctrl(<span class="number">0</span>)
                    .copy_to(table.table.ctrl(table.table.buckets()), Group::WIDTH);
            }
            drop(table);
        }
    }

    <span class="doccomment">/// CHECKING THAT WE DON'T TRY TO DROP DATA IF THE `ITEMS`
    /// ARE ZERO, EVEN IF WE HAVE `FULL` CONTROL BYTES.
    </span><span class="attr">#[test]
    </span><span class="kw">fn </span>test_catch_panic_clone_from() {
        <span class="kw">use </span>::alloc::sync::Arc;
        <span class="kw">use </span>::alloc::vec::Vec;
        <span class="kw">use </span>allocator_api2::alloc::{AllocError, Allocator, Global};
        <span class="kw">use </span>core::sync::atomic::{AtomicI8, Ordering};
        <span class="kw">use </span>std::thread;

        <span class="kw">struct </span>MyAllocInner {
            drop_count: Arc&lt;AtomicI8&gt;,
        }

        <span class="attr">#[derive(Clone)]
        </span><span class="kw">struct </span>MyAlloc {
            _inner: Arc&lt;MyAllocInner&gt;,
        }

        <span class="kw">impl </span>Drop <span class="kw">for </span>MyAllocInner {
            <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
                <span class="macro">println!</span>(<span class="string">"MyAlloc freed."</span>);
                <span class="self">self</span>.drop_count.fetch_sub(<span class="number">1</span>, Ordering::SeqCst);
            }
        }

        <span class="kw">unsafe impl </span>Allocator <span class="kw">for </span>MyAlloc {
            <span class="kw">fn </span>allocate(<span class="kw-2">&amp;</span><span class="self">self</span>, layout: Layout) -&gt; std::result::Result&lt;NonNull&lt;[u8]&gt;, AllocError&gt; {
                <span class="kw">let </span>g = Global;
                g.allocate(layout)
            }

            <span class="kw">unsafe fn </span>deallocate(<span class="kw-2">&amp;</span><span class="self">self</span>, ptr: NonNull&lt;u8&gt;, layout: Layout) {
                <span class="kw">let </span>g = Global;
                g.deallocate(ptr, layout)
            }
        }

        <span class="kw">const </span>DISARMED: bool = <span class="bool-val">false</span>;
        <span class="kw">const </span>ARMED: bool = <span class="bool-val">true</span>;

        <span class="kw">struct </span>CheckedCloneDrop {
            panic_in_clone: bool,
            dropped: bool,
            need_drop: Vec&lt;u64&gt;,
        }

        <span class="kw">impl </span>Clone <span class="kw">for </span>CheckedCloneDrop {
            <span class="kw">fn </span>clone(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
                <span class="kw">if </span><span class="self">self</span>.panic_in_clone {
                    <span class="macro">panic!</span>(<span class="string">"panic in clone"</span>)
                }
                <span class="self">Self </span>{
                    panic_in_clone: <span class="self">self</span>.panic_in_clone,
                    dropped: <span class="self">self</span>.dropped,
                    need_drop: <span class="self">self</span>.need_drop.clone(),
                }
            }
        }

        <span class="kw">impl </span>Drop <span class="kw">for </span>CheckedCloneDrop {
            <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
                <span class="kw">if </span><span class="self">self</span>.dropped {
                    <span class="macro">panic!</span>(<span class="string">"double drop"</span>);
                }
                <span class="self">self</span>.dropped = <span class="bool-val">true</span>;
            }
        }

        <span class="kw">let </span>dropped: Arc&lt;AtomicI8&gt; = Arc::new(AtomicI8::new(<span class="number">2</span>));

        <span class="kw">let </span><span class="kw-2">mut </span>table = RawTable::new_in(MyAlloc {
            _inner: Arc::new(MyAllocInner {
                drop_count: dropped.clone(),
            }),
        });

        <span class="kw">for </span>(idx, panic_in_clone) <span class="kw">in </span>core::iter::repeat(DISARMED).take(<span class="number">7</span>).enumerate() {
            <span class="kw">let </span>idx = idx <span class="kw">as </span>u64;
            table.insert(
                idx,
                (
                    idx,
                    CheckedCloneDrop {
                        panic_in_clone,
                        dropped: <span class="bool-val">false</span>,
                        need_drop: <span class="macro">vec!</span>[idx],
                    },
                ),
                |(k, <span class="kw">_</span>)| <span class="kw-2">*</span>k,
            );
        }

        <span class="macro">assert_eq!</span>(table.len(), <span class="number">7</span>);

        thread::scope(|s| {
            <span class="kw">let </span>result = s.spawn(|| {
                <span class="kw">let </span>armed_flags = [
                    DISARMED, DISARMED, ARMED, DISARMED, DISARMED, DISARMED, DISARMED,
                ];
                <span class="kw">let </span><span class="kw-2">mut </span>scope_table = RawTable::new_in(MyAlloc {
                    _inner: Arc::new(MyAllocInner {
                        drop_count: dropped.clone(),
                    }),
                });
                <span class="kw">for </span>(idx, <span class="kw-2">&amp;</span>panic_in_clone) <span class="kw">in </span>armed_flags.iter().enumerate() {
                    <span class="kw">let </span>idx = idx <span class="kw">as </span>u64;
                    scope_table.insert(
                        idx,
                        (
                            idx,
                            CheckedCloneDrop {
                                panic_in_clone,
                                dropped: <span class="bool-val">false</span>,
                                need_drop: <span class="macro">vec!</span>[idx + <span class="number">100</span>],
                            },
                        ),
                        |(k, <span class="kw">_</span>)| <span class="kw-2">*</span>k,
                    );
                }
                table.clone_from(<span class="kw-2">&amp;</span>scope_table);
            });
            <span class="macro">assert!</span>(result.join().is_err());
        });

        <span class="comment">// Let's check that all iterators work fine and do not return elements
        // (especially `RawIterRange`, which does not depend on the number of
        // elements in the table, but looks directly at the control bytes)
        //
        // SAFETY: We know for sure that `RawTable` will outlive
        // the returned `RawIter / RawIterRange` iterator.
        </span><span class="macro">assert_eq!</span>(table.len(), <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(<span class="kw">unsafe </span>{ table.iter().count() }, <span class="number">0</span>);
        <span class="macro">assert_eq!</span>(<span class="kw">unsafe </span>{ table.iter().iter.count() }, <span class="number">0</span>);

        <span class="kw">for </span>idx <span class="kw">in </span><span class="number">0</span>..table.buckets() {
            <span class="kw">let </span>idx = idx <span class="kw">as </span>u64;
            <span class="macro">assert!</span>(
                table.find(idx, |(k, <span class="kw">_</span>)| <span class="kw-2">*</span>k == idx).is_none(),
                <span class="string">"Index: {idx}"
            </span>);
        }

        <span class="comment">// All allocator clones should already be dropped.
        </span><span class="macro">assert_eq!</span>(dropped.load(Ordering::SeqCst), <span class="number">1</span>);
    }
}
</code></pre></div></section></main></body></html>