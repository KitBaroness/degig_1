<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/h2-0.3.26/src/server.rs`."><title>server.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="h2" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../h2/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Server implementation of the HTTP/2 protocol.
//!
//! # Getting started
//!
//! Running an HTTP/2 server requires the caller to manage accepting the
//! connections as well as getting the connections to a state that is ready to
//! begin the HTTP/2 handshake. See [here](../index.html#handshake) for more
//! details.
//!
//! This could be as basic as using Tokio's [`TcpListener`] to accept
//! connections, but usually it means using either ALPN or HTTP/1.1 protocol
//! upgrades.
//!
//! Once a connection is obtained, it is passed to [`handshake`],
//! which will begin the [HTTP/2 handshake]. This returns a future that
//! completes once the handshake process is performed and HTTP/2 streams may
//! be received.
//!
//! [`handshake`] uses default configuration values. There are a number of
//! settings that can be changed by using [`Builder`] instead.
//!
//! # Inbound streams
//!
//! The [`Connection`] instance is used to accept inbound HTTP/2 streams. It
//! does this by implementing [`futures::Stream`]. When a new stream is
//! received, a call to [`Connection::accept`] will return `(request, response)`.
//! The `request` handle (of type [`http::Request&lt;RecvStream&gt;`]) contains the
//! HTTP request head as well as provides a way to receive the inbound data
//! stream and the trailers. The `response` handle (of type [`SendResponse`])
//! allows responding to the request, stream the response payload, send
//! trailers, and send push promises.
//!
//! The send ([`SendStream`]) and receive ([`RecvStream`]) halves of the stream
//! can be operated independently.
//!
//! # Managing the connection
//!
//! The [`Connection`] instance is used to manage connection state. The caller
//! is required to call either [`Connection::accept`] or
//! [`Connection::poll_close`] in order to advance the connection state. Simply
//! operating on [`SendStream`] or [`RecvStream`] will have no effect unless the
//! connection state is advanced.
//!
//! It is not required to call **both** [`Connection::accept`] and
//! [`Connection::poll_close`]. If the caller is ready to accept a new stream,
//! then only [`Connection::accept`] should be called. When the caller **does
//! not** want to accept a new stream, [`Connection::poll_close`] should be
//! called.
//!
//! The [`Connection`] instance should only be dropped once
//! [`Connection::poll_close`] returns `Ready`. Once [`Connection::accept`]
//! returns `Ready(None)`, there will no longer be any more inbound streams. At
//! this point, only [`Connection::poll_close`] should be called.
//!
//! # Shutting down the server
//!
//! Graceful shutdown of the server is [not yet
//! implemented](https://github.com/hyperium/h2/issues/69).
//!
//! # Example
//!
//! A basic HTTP/2 server example that runs over TCP and assumes [prior
//! knowledge], i.e. both the client and the server assume that the TCP socket
//! will use the HTTP/2 protocol without prior negotiation.
//!
//! ```no_run
//! use h2::server;
//! use http::{Response, StatusCode};
//! use tokio::net::TcpListener;
//!
//! #[tokio::main]
//! pub async fn main() {
//!     let mut listener = TcpListener::bind("127.0.0.1:5928").await.unwrap();
//!
//!     // Accept all incoming TCP connections.
//!     loop {
//!         if let Ok((socket, _peer_addr)) = listener.accept().await {
//!             // Spawn a new task to process each connection.
//!             tokio::spawn(async {
//!                 // Start the HTTP/2 connection handshake
//!                 let mut h2 = server::handshake(socket).await.unwrap();
//!                 // Accept all inbound HTTP/2 streams sent over the
//!                 // connection.
//!                 while let Some(request) = h2.accept().await {
//!                     let (request, mut respond) = request.unwrap();
//!                     println!("Received request: {:?}", request);
//!
//!                     // Build a response with no body
//!                     let response = Response::builder()
//!                         .status(StatusCode::OK)
//!                         .body(())
//!                         .unwrap();
//!
//!                     // Send the response back to the client
//!                     respond.send_response(response, true)
//!                         .unwrap();
//!                 }
//!
//!             });
//!         }
//!     }
//! }
//! ```
//!
//! [prior knowledge]: http://httpwg.org/specs/rfc7540.html#known-http
//! [`handshake`]: fn.handshake.html
//! [HTTP/2 handshake]: http://httpwg.org/specs/rfc7540.html#ConnectionHeader
//! [`Builder`]: struct.Builder.html
//! [`Connection`]: struct.Connection.html
//! [`Connection::poll`]: struct.Connection.html#method.poll
//! [`Connection::poll_close`]: struct.Connection.html#method.poll_close
//! [`futures::Stream`]: https://docs.rs/futures/0.1/futures/stream/trait.Stream.html
//! [`http::Request&lt;RecvStream&gt;`]: ../struct.RecvStream.html
//! [`RecvStream`]: ../struct.RecvStream.html
//! [`SendStream`]: ../struct.SendStream.html
//! [`TcpListener`]: https://docs.rs/tokio-core/0.1/tokio_core/net/struct.TcpListener.html

</span><span class="kw">use </span><span class="kw">crate</span>::codec::{Codec, UserError};
<span class="kw">use </span><span class="kw">crate</span>::frame::{<span class="self">self</span>, Pseudo, PushPromiseHeaderError, Reason, Settings, StreamId};
<span class="kw">use </span><span class="kw">crate</span>::proto::{<span class="self">self</span>, Config, Error, Prioritized};
<span class="kw">use crate</span>::{FlowControl, PingPong, RecvStream, SendStream};

<span class="kw">use </span>bytes::{Buf, Bytes};
<span class="kw">use </span>http::{HeaderMap, Method, Request, Response};
<span class="kw">use </span>std::future::Future;
<span class="kw">use </span>std::pin::Pin;
<span class="kw">use </span>std::task::{Context, Poll};
<span class="kw">use </span>std::time::Duration;
<span class="kw">use </span>std::{fmt, io};
<span class="kw">use </span>tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
<span class="kw">use </span>tracing::instrument::{Instrument, Instrumented};

<span class="doccomment">/// In progress HTTP/2 connection handshake future.
///
/// This type implements `Future`, yielding a `Connection` instance once the
/// handshake has completed.
///
/// The handshake is completed once the connection preface is fully received
/// from the client **and** the initial settings frame is sent to the client.
///
/// The handshake future does not wait for the initial settings frame from the
/// client.
///
/// See [module] level docs for more details.
///
/// [module]: index.html
</span><span class="attr">#[must_use = <span class="string">"futures do nothing unless polled"</span>]
</span><span class="kw">pub struct </span>Handshake&lt;T, B: Buf = Bytes&gt; {
    <span class="doccomment">/// The config to pass to Connection::new after handshake succeeds.
    </span>builder: Builder,
    <span class="doccomment">/// The current state of the handshake.
    </span>state: Handshaking&lt;T, B&gt;,
    <span class="doccomment">/// Span tracking the handshake
    </span>span: tracing::Span,
}

<span class="doccomment">/// Accepts inbound HTTP/2 streams on a connection.
///
/// A `Connection` is backed by an I/O resource (usually a TCP socket) and
/// implements the HTTP/2 server logic for that connection. It is responsible
/// for receiving inbound streams initiated by the client as well as driving the
/// internal state forward.
///
/// `Connection` values are created by calling [`handshake`]. Once a
/// `Connection` value is obtained, the caller must call [`poll`] or
/// [`poll_close`] in order to drive the internal connection state forward.
///
/// See [module level] documentation for more details
///
/// [module level]: index.html
/// [`handshake`]: struct.Connection.html#method.handshake
/// [`poll`]: struct.Connection.html#method.poll
/// [`poll_close`]: struct.Connection.html#method.poll_close
///
/// # Examples
///
/// ```
/// # use tokio::io::{AsyncRead, AsyncWrite};
/// # use h2::server;
/// # use h2::server::*;
/// #
/// # async fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T) {
/// let mut server = server::handshake(my_io).await.unwrap();
/// while let Some(request) = server.accept().await {
///     tokio::spawn(async move {
///         let (request, respond) = request.unwrap();
///         // Process the request and send the response back to the client
///         // using `respond`.
///     });
/// }
/// # }
/// #
/// # pub fn main() {}
/// ```
</span><span class="attr">#[must_use = <span class="string">"streams do nothing unless polled"</span>]
</span><span class="kw">pub struct </span>Connection&lt;T, B: Buf&gt; {
    connection: proto::Connection&lt;T, Peer, B&gt;,
}

<span class="doccomment">/// Builds server connections with custom configuration values.
///
/// Methods can be chained in order to set the configuration values.
///
/// The server is constructed by calling [`handshake`] and passing the I/O
/// handle that will back the HTTP/2 server.
///
/// New instances of `Builder` are obtained via [`Builder::new`].
///
/// See function level documentation for details on the various server
/// configuration settings.
///
/// [`Builder::new`]: struct.Builder.html#method.new
/// [`handshake`]: struct.Builder.html#method.handshake
///
/// # Examples
///
/// ```
/// # use tokio::io::{AsyncRead, AsyncWrite};
/// # use h2::server::*;
/// #
/// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
/// # -&gt; Handshake&lt;T&gt;
/// # {
/// // `server_fut` is a future representing the completion of the HTTP/2
/// // handshake.
/// let server_fut = Builder::new()
///     .initial_window_size(1_000_000)
///     .max_concurrent_streams(1000)
///     .handshake(my_io);
/// # server_fut
/// # }
/// #
/// # pub fn main() {}
/// ```
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>Builder {
    <span class="doccomment">/// Time to keep locally reset streams around before reaping.
    </span>reset_stream_duration: Duration,

    <span class="doccomment">/// Maximum number of locally reset streams to keep at a time.
    </span>reset_stream_max: usize,

    <span class="doccomment">/// Maximum number of remotely reset streams to allow in the pending
    /// accept queue.
    </span>pending_accept_reset_stream_max: usize,

    <span class="doccomment">/// Initial `Settings` frame to send as part of the handshake.
    </span>settings: Settings,

    <span class="doccomment">/// Initial target window size for new connections.
    </span>initial_target_connection_window_size: <span class="prelude-ty">Option</span>&lt;u32&gt;,

    <span class="doccomment">/// Maximum amount of bytes to "buffer" for writing per stream.
    </span>max_send_buffer_size: usize,

    <span class="doccomment">/// Maximum number of locally reset streams due to protocol error across
    /// the lifetime of the connection.
    ///
    /// When this gets exceeded, we issue GOAWAYs.
    </span>local_max_error_reset_streams: <span class="prelude-ty">Option</span>&lt;usize&gt;,
}

<span class="doccomment">/// Send a response back to the client
///
/// A `SendResponse` instance is provided when receiving a request and is used
/// to send the associated response back to the client. It is also used to
/// explicitly reset the stream with a custom reason.
///
/// It will also be used to initiate push promises linked with the associated
/// stream.
///
/// If the `SendResponse` instance is dropped without sending a response, then
/// the HTTP/2 stream will be reset.
///
/// See [module] level docs for more details.
///
/// [module]: index.html
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>SendResponse&lt;B: Buf&gt; {
    inner: proto::StreamRef&lt;B&gt;,
}

<span class="doccomment">/// Send a response to a promised request
///
/// A `SendPushedResponse` instance is provided when promising a request and is used
/// to send the associated response to the client. It is also used to
/// explicitly reset the stream with a custom reason.
///
/// It can not be used to initiate push promises.
///
/// If the `SendPushedResponse` instance is dropped without sending a response, then
/// the HTTP/2 stream will be reset.
///
/// See [module] level docs for more details.
///
/// [module]: index.html
</span><span class="kw">pub struct </span>SendPushedResponse&lt;B: Buf&gt; {
    inner: SendResponse&lt;B&gt;,
}

<span class="comment">// Manual implementation necessary because of rust-lang/rust#26925
</span><span class="kw">impl</span>&lt;B: Buf + fmt::Debug&gt; fmt::Debug <span class="kw">for </span>SendPushedResponse&lt;B&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="macro">write!</span>(f, <span class="string">"SendPushedResponse {{ {:?} }}"</span>, <span class="self">self</span>.inner)
    }
}

<span class="doccomment">/// Stages of an in-progress handshake.
</span><span class="kw">enum </span>Handshaking&lt;T, B: Buf&gt; {
    <span class="doccomment">/// State 1. Connection is flushing pending SETTINGS frame.
    </span>Flushing(Instrumented&lt;Flush&lt;T, Prioritized&lt;B&gt;&gt;&gt;),
    <span class="doccomment">/// State 2. Connection is waiting for the client preface.
    </span>ReadingPreface(Instrumented&lt;ReadPreface&lt;T, Prioritized&lt;B&gt;&gt;&gt;),
    <span class="doccomment">/// State 3. Handshake is done, polling again would panic.
    </span>Done,
}

<span class="doccomment">/// Flush a Sink
</span><span class="kw">struct </span>Flush&lt;T, B&gt; {
    codec: <span class="prelude-ty">Option</span>&lt;Codec&lt;T, B&gt;&gt;,
}

<span class="doccomment">/// Read the client connection preface
</span><span class="kw">struct </span>ReadPreface&lt;T, B&gt; {
    codec: <span class="prelude-ty">Option</span>&lt;Codec&lt;T, B&gt;&gt;,
    pos: usize,
}

<span class="attr">#[derive(Debug)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Peer;

<span class="kw">const </span>PREFACE: [u8; <span class="number">24</span>] = <span class="kw-2">*</span><span class="string">b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"</span>;

<span class="doccomment">/// Creates a new configured HTTP/2 server with default configuration
/// values backed by `io`.
///
/// It is expected that `io` already be in an appropriate state to commence
/// the [HTTP/2 handshake]. See [Handshake] for more details.
///
/// Returns a future which resolves to the [`Connection`] instance once the
/// HTTP/2 handshake has been completed. The returned [`Connection`]
/// instance will be using default configuration values. Use [`Builder`] to
/// customize the configuration values used by a [`Connection`] instance.
///
/// [HTTP/2 handshake]: http://httpwg.org/specs/rfc7540.html#ConnectionHeader
/// [Handshake]: ../index.html#handshake
/// [`Connection`]: struct.Connection.html
///
/// # Examples
///
/// ```
/// # use tokio::io::{AsyncRead, AsyncWrite};
/// # use h2::server;
/// # use h2::server::*;
/// #
/// # async fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
/// # {
/// let connection = server::handshake(my_io).await.unwrap();
/// // The HTTP/2 handshake has completed, now use `connection` to
/// // accept inbound HTTP/2 streams.
/// # }
/// #
/// # pub fn main() {}
/// ```
</span><span class="kw">pub fn </span>handshake&lt;T&gt;(io: T) -&gt; Handshake&lt;T, Bytes&gt;
<span class="kw">where
    </span>T: AsyncRead + AsyncWrite + Unpin,
{
    Builder::new().handshake(io)
}

<span class="comment">// ===== impl Connection =====

</span><span class="kw">impl</span>&lt;T, B&gt; Connection&lt;T, B&gt;
<span class="kw">where
    </span>T: AsyncRead + AsyncWrite + Unpin,
    B: Buf,
{
    <span class="kw">fn </span>handshake2(io: T, builder: Builder) -&gt; Handshake&lt;T, B&gt; {
        <span class="kw">let </span>span = <span class="macro">tracing::trace_span!</span>(<span class="string">"server_handshake"</span>);
        <span class="kw">let </span>entered = span.enter();

        <span class="comment">// Create the codec.
        </span><span class="kw">let </span><span class="kw-2">mut </span>codec = Codec::new(io);

        <span class="kw">if let </span><span class="prelude-val">Some</span>(max) = builder.settings.max_frame_size() {
            codec.set_max_recv_frame_size(max <span class="kw">as </span>usize);
        }

        <span class="kw">if let </span><span class="prelude-val">Some</span>(max) = builder.settings.max_header_list_size() {
            codec.set_max_recv_header_list_size(max <span class="kw">as </span>usize);
        }

        <span class="comment">// Send initial settings frame.
        </span>codec
            .buffer(builder.settings.clone().into())
            .expect(<span class="string">"invalid SETTINGS frame"</span>);

        <span class="comment">// Create the handshake future.
        </span><span class="kw">let </span>state =
            Handshaking::Flushing(Flush::new(codec).instrument(<span class="macro">tracing::trace_span!</span>(<span class="string">"flush"</span>)));

        drop(entered);

        Handshake {
            builder,
            state,
            span,
        }
    }

    <span class="doccomment">/// Accept the next incoming request on this connection.
    </span><span class="kw">pub async fn </span>accept(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="prelude-ty">Result</span>&lt;(Request&lt;RecvStream&gt;, SendResponse&lt;B&gt;), <span class="kw">crate</span>::Error&gt;&gt; {
        futures_util::future::poll_fn(<span class="kw">move </span>|cx| <span class="self">self</span>.poll_accept(cx)).<span class="kw">await
    </span>}

    <span class="attr">#[doc(hidden)]
    </span><span class="kw">pub fn </span>poll_accept(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        cx: <span class="kw-2">&amp;mut </span>Context&lt;<span class="lifetime">'_</span>&gt;,
    ) -&gt; Poll&lt;<span class="prelude-ty">Option</span>&lt;<span class="prelude-ty">Result</span>&lt;(Request&lt;RecvStream&gt;, SendResponse&lt;B&gt;), <span class="kw">crate</span>::Error&gt;&gt;&gt; {
        <span class="comment">// Always try to advance the internal state. Getting Pending also is
        // needed to allow this function to return Pending.
        </span><span class="kw">if </span><span class="self">self</span>.poll_closed(cx)<span class="question-mark">?</span>.is_ready() {
            <span class="comment">// If the socket is closed, don't return anything
            // TODO: drop any pending streams
            </span><span class="kw">return </span>Poll::Ready(<span class="prelude-val">None</span>);
        }

        <span class="kw">if let </span><span class="prelude-val">Some</span>(inner) = <span class="self">self</span>.connection.next_incoming() {
            <span class="macro">tracing::trace!</span>(<span class="string">"received incoming"</span>);
            <span class="kw">let </span>(head, <span class="kw">_</span>) = inner.take_request().into_parts();
            <span class="kw">let </span>body = RecvStream::new(FlowControl::new(inner.clone_to_opaque()));

            <span class="kw">let </span>request = Request::from_parts(head, body);
            <span class="kw">let </span>respond = SendResponse { inner };

            <span class="kw">return </span>Poll::Ready(<span class="prelude-val">Some</span>(<span class="prelude-val">Ok</span>((request, respond))));
        }

        Poll::Pending
    }

    <span class="doccomment">/// Sets the target window size for the whole connection.
    ///
    /// If `size` is greater than the current value, then a `WINDOW_UPDATE`
    /// frame will be immediately sent to the remote, increasing the connection
    /// level window by `size - current_value`.
    ///
    /// If `size` is less than the current value, nothing will happen
    /// immediately. However, as window capacity is released by
    /// [`FlowControl`] instances, no `WINDOW_UPDATE` frames will be sent
    /// out until the number of "in flight" bytes drops below `size`.
    ///
    /// The default value is 65,535.
    ///
    /// See [`FlowControl`] documentation for more details.
    ///
    /// [`FlowControl`]: ../struct.FlowControl.html
    /// [library level]: ../index.html#flow-control
    </span><span class="kw">pub fn </span>set_target_window_size(<span class="kw-2">&amp;mut </span><span class="self">self</span>, size: u32) {
        <span class="macro">assert!</span>(size &lt;= proto::MAX_WINDOW_SIZE);
        <span class="self">self</span>.connection.set_target_window_size(size);
    }

    <span class="doccomment">/// Set a new `INITIAL_WINDOW_SIZE` setting (in octets) for stream-level
    /// flow control for received data.
    ///
    /// The `SETTINGS` will be sent to the remote, and only applied once the
    /// remote acknowledges the change.
    ///
    /// This can be used to increase or decrease the window size for existing
    /// streams.
    ///
    /// # Errors
    ///
    /// Returns an error if a previous call is still pending acknowledgement
    /// from the remote endpoint.
    </span><span class="kw">pub fn </span>set_initial_window_size(<span class="kw-2">&amp;mut </span><span class="self">self</span>, size: u32) -&gt; <span class="prelude-ty">Result</span>&lt;(), <span class="kw">crate</span>::Error&gt; {
        <span class="macro">assert!</span>(size &lt;= proto::MAX_WINDOW_SIZE);
        <span class="self">self</span>.connection.set_initial_window_size(size)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Enables the [extended CONNECT protocol].
    ///
    /// [extended CONNECT protocol]: https://datatracker.ietf.org/doc/html/rfc8441#section-4
    ///
    /// # Errors
    ///
    /// Returns an error if a previous call is still pending acknowledgement
    /// from the remote endpoint.
    </span><span class="kw">pub fn </span>enable_connect_protocol(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), <span class="kw">crate</span>::Error&gt; {
        <span class="self">self</span>.connection.set_enable_connect_protocol()<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Returns `Ready` when the underlying connection has closed.
    ///
    /// If any new inbound streams are received during a call to `poll_closed`,
    /// they will be queued and returned on the next call to [`poll_accept`].
    ///
    /// This function will advance the internal connection state, driving
    /// progress on all the other handles (e.g. [`RecvStream`] and [`SendStream`]).
    ///
    /// See [here](index.html#managing-the-connection) for more details.
    ///
    /// [`poll_accept`]: struct.Connection.html#method.poll_accept
    /// [`RecvStream`]: ../struct.RecvStream.html
    /// [`SendStream`]: ../struct.SendStream.html
    </span><span class="kw">pub fn </span>poll_closed(<span class="kw-2">&amp;mut </span><span class="self">self</span>, cx: <span class="kw-2">&amp;mut </span>Context) -&gt; Poll&lt;<span class="prelude-ty">Result</span>&lt;(), <span class="kw">crate</span>::Error&gt;&gt; {
        <span class="self">self</span>.connection.poll(cx).map_err(Into::into)
    }

    <span class="attr">#[doc(hidden)]
    #[deprecated(note = <span class="string">"renamed to poll_closed"</span>)]
    </span><span class="kw">pub fn </span>poll_close(<span class="kw-2">&amp;mut </span><span class="self">self</span>, cx: <span class="kw-2">&amp;mut </span>Context) -&gt; Poll&lt;<span class="prelude-ty">Result</span>&lt;(), <span class="kw">crate</span>::Error&gt;&gt; {
        <span class="self">self</span>.poll_closed(cx)
    }

    <span class="doccomment">/// Sets the connection to a GOAWAY state.
    ///
    /// Does not terminate the connection. Must continue being polled to close
    /// connection.
    ///
    /// After flushing the GOAWAY frame, the connection is closed. Any
    /// outstanding streams do not prevent the connection from closing. This
    /// should usually be reserved for shutting down when something bad
    /// external to `h2` has happened, and open streams cannot be properly
    /// handled.
    ///
    /// For graceful shutdowns, see [`graceful_shutdown`](Connection::graceful_shutdown).
    </span><span class="kw">pub fn </span>abrupt_shutdown(<span class="kw-2">&amp;mut </span><span class="self">self</span>, reason: Reason) {
        <span class="self">self</span>.connection.go_away_from_user(reason);
    }

    <span class="doccomment">/// Starts a [graceful shutdown][1] process.
    ///
    /// Must continue being polled to close connection.
    ///
    /// It's possible to receive more requests after calling this method, since
    /// they might have been in-flight from the client already. After about
    /// 1 RTT, no new requests should be accepted. Once all active streams
    /// have completed, the connection is closed.
    ///
    /// [1]: http://httpwg.org/specs/rfc7540.html#GOAWAY
    </span><span class="kw">pub fn </span>graceful_shutdown(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.connection.go_away_gracefully();
    }

    <span class="doccomment">/// Takes a `PingPong` instance from the connection.
    ///
    /// # Note
    ///
    /// This may only be called once. Calling multiple times will return `None`.
    </span><span class="kw">pub fn </span>ping_pong(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;PingPong&gt; {
        <span class="self">self</span>.connection.take_user_pings().map(PingPong::new)
    }

    <span class="doccomment">/// Returns the maximum number of concurrent streams that may be initiated
    /// by the server on this connection.
    ///
    /// This limit is configured by the client peer by sending the
    /// [`SETTINGS_MAX_CONCURRENT_STREAMS` parameter][1] in a `SETTINGS` frame.
    /// This method returns the currently acknowledged value received from the
    /// remote.
    ///
    /// [1]: https://tools.ietf.org/html/rfc7540#section-5.1.2
    </span><span class="kw">pub fn </span>max_concurrent_send_streams(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.connection.max_send_streams()
    }

    <span class="doccomment">/// Returns the maximum number of concurrent streams that may be initiated
    /// by the client on this connection.
    ///
    /// This returns the value of the [`SETTINGS_MAX_CONCURRENT_STREAMS`
    /// parameter][1] sent in a `SETTINGS` frame that has been
    /// acknowledged by the remote peer. The value to be sent is configured by
    /// the [`Builder::max_concurrent_streams`][2] method before handshaking
    /// with the remote peer.
    ///
    /// [1]: https://tools.ietf.org/html/rfc7540#section-5.1.2
    /// [2]: ../struct.Builder.html#method.max_concurrent_streams
    </span><span class="kw">pub fn </span>max_concurrent_recv_streams(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.connection.max_recv_streams()
    }

    <span class="comment">// Could disappear at anytime.
    </span><span class="attr">#[doc(hidden)]
    #[cfg(feature = <span class="string">"unstable"</span>)]
    </span><span class="kw">pub fn </span>num_wired_streams(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.connection.num_wired_streams()
    }
}

<span class="attr">#[cfg(feature = <span class="string">"stream"</span>)]
</span><span class="kw">impl</span>&lt;T, B&gt; futures_core::Stream <span class="kw">for </span>Connection&lt;T, B&gt;
<span class="kw">where
    </span>T: AsyncRead + AsyncWrite + Unpin,
    B: Buf,
{
    <span class="kw">type </span>Item = <span class="prelude-ty">Result</span>&lt;(Request&lt;RecvStream&gt;, SendResponse&lt;B&gt;), <span class="kw">crate</span>::Error&gt;;

    <span class="kw">fn </span>poll_next(<span class="kw-2">mut </span><span class="self">self</span>: Pin&lt;<span class="kw-2">&amp;mut </span><span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>Context&lt;<span class="lifetime">'_</span>&gt;) -&gt; Poll&lt;<span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;&gt; {
        <span class="self">self</span>.poll_accept(cx)
    }
}

<span class="kw">impl</span>&lt;T, B&gt; fmt::Debug <span class="kw">for </span>Connection&lt;T, B&gt;
<span class="kw">where
    </span>T: fmt::Debug,
    B: fmt::Debug + Buf,
{
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, fmt: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        fmt.debug_struct(<span class="string">"Connection"</span>)
            .field(<span class="string">"connection"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.connection)
            .finish()
    }
}

<span class="comment">// ===== impl Builder =====

</span><span class="kw">impl </span>Builder {
    <span class="doccomment">/// Returns a new server builder instance initialized with default
    /// configuration values.
    ///
    /// Configuration methods can be chained on the return value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut = Builder::new()
    ///     .initial_window_size(1_000_000)
    ///     .max_concurrent_streams(1000)
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    </span><span class="kw">pub fn </span>new() -&gt; Builder {
        Builder {
            reset_stream_duration: Duration::from_secs(proto::DEFAULT_RESET_STREAM_SECS),
            reset_stream_max: proto::DEFAULT_RESET_STREAM_MAX,
            pending_accept_reset_stream_max: proto::DEFAULT_REMOTE_RESET_STREAM_MAX,
            settings: Settings::default(),
            initial_target_connection_window_size: <span class="prelude-val">None</span>,
            max_send_buffer_size: proto::DEFAULT_MAX_SEND_BUFFER_SIZE,

            local_max_error_reset_streams: <span class="prelude-val">Some</span>(proto::DEFAULT_LOCAL_RESET_COUNT_MAX),
        }
    }

    <span class="doccomment">/// Indicates the initial window size (in octets) for stream-level
    /// flow control for received data.
    ///
    /// The initial window of a stream is used as part of flow control. For more
    /// details, see [`FlowControl`].
    ///
    /// The default value is 65,535.
    ///
    /// [`FlowControl`]: ../struct.FlowControl.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut = Builder::new()
    ///     .initial_window_size(1_000_000)
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    </span><span class="kw">pub fn </span>initial_window_size(<span class="kw-2">&amp;mut </span><span class="self">self</span>, size: u32) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.settings.set_initial_window_size(<span class="prelude-val">Some</span>(size));
        <span class="self">self
    </span>}

    <span class="doccomment">/// Indicates the initial window size (in octets) for connection-level flow control
    /// for received data.
    ///
    /// The initial window of a connection is used as part of flow control. For more details,
    /// see [`FlowControl`].
    ///
    /// The default value is 65,535.
    ///
    /// [`FlowControl`]: ../struct.FlowControl.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut = Builder::new()
    ///     .initial_connection_window_size(1_000_000)
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    </span><span class="kw">pub fn </span>initial_connection_window_size(<span class="kw-2">&amp;mut </span><span class="self">self</span>, size: u32) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.initial_target_connection_window_size = <span class="prelude-val">Some</span>(size);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Indicates the size (in octets) of the largest HTTP/2 frame payload that the
    /// configured server is able to accept.
    ///
    /// The sender may send data frames that are **smaller** than this value,
    /// but any data larger than `max` will be broken up into multiple `DATA`
    /// frames.
    ///
    /// The value **must** be between 16,384 and 16,777,215. The default value is 16,384.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut = Builder::new()
    ///     .max_frame_size(1_000_000)
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if `max` is not within the legal range specified
    /// above.
    </span><span class="kw">pub fn </span>max_frame_size(<span class="kw-2">&amp;mut </span><span class="self">self</span>, max: u32) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.settings.set_max_frame_size(<span class="prelude-val">Some</span>(max));
        <span class="self">self
    </span>}

    <span class="doccomment">/// Sets the max size of received header frames.
    ///
    /// This advisory setting informs a peer of the maximum size of header list
    /// that the sender is prepared to accept, in octets. The value is based on
    /// the uncompressed size of header fields, including the length of the name
    /// and value in octets plus an overhead of 32 octets for each header field.
    ///
    /// This setting is also used to limit the maximum amount of data that is
    /// buffered to decode HEADERS frames.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut = Builder::new()
    ///     .max_header_list_size(16 * 1024)
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    </span><span class="kw">pub fn </span>max_header_list_size(<span class="kw-2">&amp;mut </span><span class="self">self</span>, max: u32) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.settings.set_max_header_list_size(<span class="prelude-val">Some</span>(max));
        <span class="self">self
    </span>}

    <span class="doccomment">/// Sets the maximum number of concurrent streams.
    ///
    /// The maximum concurrent streams setting only controls the maximum number
    /// of streams that can be initiated by the remote peer. In other words,
    /// when this setting is set to 100, this does not limit the number of
    /// concurrent streams that can be created by the caller.
    ///
    /// It is recommended that this value be no smaller than 100, so as to not
    /// unnecessarily limit parallelism. However, any value is legal, including
    /// 0. If `max` is set to 0, then the remote will not be permitted to
    /// initiate streams.
    ///
    /// Note that streams in the reserved state, i.e., push promises that have
    /// been reserved but the stream has not started, do not count against this
    /// setting.
    ///
    /// Also note that if the remote *does* exceed the value set here, it is not
    /// a protocol level error. Instead, the `h2` library will immediately reset
    /// the stream.
    ///
    /// See [Section 5.1.2] in the HTTP/2 spec for more details.
    ///
    /// [Section 5.1.2]: https://http2.github.io/http2-spec/#rfc.section.5.1.2
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut = Builder::new()
    ///     .max_concurrent_streams(1000)
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    </span><span class="kw">pub fn </span>max_concurrent_streams(<span class="kw-2">&amp;mut </span><span class="self">self</span>, max: u32) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.settings.set_max_concurrent_streams(<span class="prelude-val">Some</span>(max));
        <span class="self">self
    </span>}

    <span class="doccomment">/// Sets the maximum number of concurrent locally reset streams.
    ///
    /// When a stream is explicitly reset by either calling
    /// [`SendResponse::send_reset`] or by dropping a [`SendResponse`] instance
    /// before completing the stream, the HTTP/2 specification requires that
    /// any further frames received for that stream must be ignored for "some
    /// time".
    ///
    /// In order to satisfy the specification, internal state must be maintained
    /// to implement the behavior. This state grows linearly with the number of
    /// streams that are locally reset.
    ///
    /// The `max_concurrent_reset_streams` setting configures sets an upper
    /// bound on the amount of state that is maintained. When this max value is
    /// reached, the oldest reset stream is purged from memory.
    ///
    /// Once the stream has been fully purged from memory, any additional frames
    /// received for that stream will result in a connection level protocol
    /// error, forcing the connection to terminate.
    ///
    /// The default value is 10.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut = Builder::new()
    ///     .max_concurrent_reset_streams(1000)
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    </span><span class="kw">pub fn </span>max_concurrent_reset_streams(<span class="kw-2">&amp;mut </span><span class="self">self</span>, max: usize) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.reset_stream_max = max;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Sets the maximum number of local resets due to protocol errors made by the remote end.
    ///
    /// Invalid frames and many other protocol errors will lead to resets being generated for those streams.
    /// Too many of these often indicate a malicious client, and there are attacks which can abuse this to DOS servers.
    /// This limit protects against these DOS attacks by limiting the amount of resets we can be forced to generate.
    ///
    /// When the number of local resets exceeds this threshold, the server will issue GOAWAYs with an error code of
    /// `ENHANCE_YOUR_CALM` to the client.
    ///
    /// If you really want to disable this, supply [`Option::None`] here.
    /// Disabling this is not recommended and may expose you to DOS attacks.
    ///
    /// The default value is currently 1024, but could change.
    </span><span class="kw">pub fn </span>max_local_error_reset_streams(<span class="kw-2">&amp;mut </span><span class="self">self</span>, max: <span class="prelude-ty">Option</span>&lt;usize&gt;) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.local_max_error_reset_streams = max;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Sets the maximum number of pending-accept remotely-reset streams.
    ///
    /// Streams that have been received by the peer, but not accepted by the
    /// user, can also receive a RST_STREAM. This is a legitimate pattern: one
    /// could send a request and then shortly after, realize it is not needed,
    /// sending a CANCEL.
    ///
    /// However, since those streams are now "closed", they don't count towards
    /// the max concurrent streams. So, they will sit in the accept queue,
    /// using memory.
    ///
    /// When the number of remotely-reset streams sitting in the pending-accept
    /// queue reaches this maximum value, a connection error with the code of
    /// `ENHANCE_YOUR_CALM` will be sent to the peer, and returned by the
    /// `Future`.
    ///
    /// The default value is currently 20, but could change.
    ///
    /// # Examples
    ///
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut = Builder::new()
    ///     .max_pending_accept_reset_streams(100)
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    </span><span class="kw">pub fn </span>max_pending_accept_reset_streams(<span class="kw-2">&amp;mut </span><span class="self">self</span>, max: usize) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.pending_accept_reset_stream_max = max;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Sets the maximum send buffer size per stream.
    ///
    /// Once a stream has buffered up to (or over) the maximum, the stream's
    /// flow control will not "poll" additional capacity. Once bytes for the
    /// stream have been written to the connection, the send buffer capacity
    /// will be freed up again.
    ///
    /// The default is currently ~400KB, but may change.
    ///
    /// # Panics
    ///
    /// This function panics if `max` is larger than `u32::MAX`.
    </span><span class="kw">pub fn </span>max_send_buffer_size(<span class="kw-2">&amp;mut </span><span class="self">self</span>, max: usize) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="macro">assert!</span>(max &lt;= std::u32::MAX <span class="kw">as </span>usize);
        <span class="self">self</span>.max_send_buffer_size = max;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Sets the maximum number of concurrent locally reset streams.
    ///
    /// When a stream is explicitly reset by either calling
    /// [`SendResponse::send_reset`] or by dropping a [`SendResponse`] instance
    /// before completing the stream, the HTTP/2 specification requires that
    /// any further frames received for that stream must be ignored for "some
    /// time".
    ///
    /// In order to satisfy the specification, internal state must be maintained
    /// to implement the behavior. This state grows linearly with the number of
    /// streams that are locally reset.
    ///
    /// The `reset_stream_duration` setting configures the max amount of time
    /// this state will be maintained in memory. Once the duration elapses, the
    /// stream state is purged from memory.
    ///
    /// Once the stream has been fully purged from memory, any additional frames
    /// received for that stream will result in a connection level protocol
    /// error, forcing the connection to terminate.
    ///
    /// The default value is 30 seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// # use std::time::Duration;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut = Builder::new()
    ///     .reset_stream_duration(Duration::from_secs(10))
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    </span><span class="kw">pub fn </span>reset_stream_duration(<span class="kw-2">&amp;mut </span><span class="self">self</span>, dur: Duration) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.reset_stream_duration = dur;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Enables the [extended CONNECT protocol].
    ///
    /// [extended CONNECT protocol]: https://datatracker.ietf.org/doc/html/rfc8441#section-4
    </span><span class="kw">pub fn </span>enable_connect_protocol(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.settings.set_enable_connect_protocol(<span class="prelude-val">Some</span>(<span class="number">1</span>));
        <span class="self">self
    </span>}

    <span class="doccomment">/// Creates a new configured HTTP/2 server backed by `io`.
    ///
    /// It is expected that `io` already be in an appropriate state to commence
    /// the [HTTP/2 handshake]. See [Handshake] for more details.
    ///
    /// Returns a future which resolves to the [`Connection`] instance once the
    /// HTTP/2 handshake has been completed.
    ///
    /// This function also allows the caller to configure the send payload data
    /// type. See [Outbound data type] for more details.
    ///
    /// [HTTP/2 handshake]: http://httpwg.org/specs/rfc7540.html#ConnectionHeader
    /// [Handshake]: ../index.html#handshake
    /// [`Connection`]: struct.Connection.html
    /// [Outbound data type]: ../index.html#outbound-data-type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut = Builder::new()
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    ///
    /// Configures the send-payload data type. In this case, the outbound data
    /// type will be `&amp;'static [u8]`.
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use h2::server::*;
    /// #
    /// # fn doc&lt;T: AsyncRead + AsyncWrite + Unpin&gt;(my_io: T)
    /// # -&gt; Handshake&lt;T, &amp;'static [u8]&gt;
    /// # {
    /// // `server_fut` is a future representing the completion of the HTTP/2
    /// // handshake.
    /// let server_fut: Handshake&lt;_, &amp;'static [u8]&gt; = Builder::new()
    ///     .handshake(my_io);
    /// # server_fut
    /// # }
    /// #
    /// # pub fn main() {}
    /// ```
    </span><span class="kw">pub fn </span>handshake&lt;T, B&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, io: T) -&gt; Handshake&lt;T, B&gt;
    <span class="kw">where
        </span>T: AsyncRead + AsyncWrite + Unpin,
        B: Buf,
    {
        Connection::handshake2(io, <span class="self">self</span>.clone())
    }
}

<span class="kw">impl </span>Default <span class="kw">for </span>Builder {
    <span class="kw">fn </span>default() -&gt; Builder {
        Builder::new()
    }
}

<span class="comment">// ===== impl SendResponse =====

</span><span class="kw">impl</span>&lt;B: Buf&gt; SendResponse&lt;B&gt; {
    <span class="doccomment">/// Send a response to a client request.
    ///
    /// On success, a [`SendStream`] instance is returned. This instance can be
    /// used to stream the response body and send trailers.
    ///
    /// If a body or trailers will be sent on the returned [`SendStream`]
    /// instance, then `end_of_stream` must be set to `false` when calling this
    /// function.
    ///
    /// The [`SendResponse`] instance is already associated with a received
    /// request.  This function may only be called once per instance and only if
    /// [`send_reset`] has not been previously called.
    ///
    /// [`SendResponse`]: #
    /// [`SendStream`]: ../struct.SendStream.html
    /// [`send_reset`]: #method.send_reset
    </span><span class="kw">pub fn </span>send_response(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        response: Response&lt;()&gt;,
        end_of_stream: bool,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;SendStream&lt;B&gt;, <span class="kw">crate</span>::Error&gt; {
        <span class="self">self</span>.inner
            .send_response(response, end_of_stream)
            .map(|<span class="kw">_</span>| SendStream::new(<span class="self">self</span>.inner.clone()))
            .map_err(Into::into)
    }

    <span class="doccomment">/// Push a request and response to the client
    ///
    /// On success, a [`SendResponse`] instance is returned.
    ///
    /// [`SendResponse`]: #
    </span><span class="kw">pub fn </span>push_request(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        request: Request&lt;()&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;SendPushedResponse&lt;B&gt;, <span class="kw">crate</span>::Error&gt; {
        <span class="self">self</span>.inner
            .send_push_promise(request)
            .map(|inner| SendPushedResponse {
                inner: SendResponse { inner },
            })
            .map_err(Into::into)
    }

    <span class="doccomment">/// Send a stream reset to the peer.
    ///
    /// This essentially cancels the stream, including any inbound or outbound
    /// data streams.
    ///
    /// If this function is called before [`send_response`], a call to
    /// [`send_response`] will result in an error.
    ///
    /// If this function is called while a [`SendStream`] instance is active,
    /// any further use of the instance will result in an error.
    ///
    /// This function should only be called once.
    ///
    /// [`send_response`]: #method.send_response
    /// [`SendStream`]: ../struct.SendStream.html
    </span><span class="kw">pub fn </span>send_reset(<span class="kw-2">&amp;mut </span><span class="self">self</span>, reason: Reason) {
        <span class="self">self</span>.inner.send_reset(reason)
    }

    <span class="doccomment">/// Polls to be notified when the client resets this stream.
    ///
    /// If stream is still open, this returns `Poll::Pending`, and
    /// registers the task to be notified if a `RST_STREAM` is received.
    ///
    /// If a `RST_STREAM` frame is received for this stream, calling this
    /// method will yield the `Reason` for the reset.
    ///
    /// # Error
    ///
    /// Calling this method after having called `send_response` will return
    /// a user error.
    </span><span class="kw">pub fn </span>poll_reset(<span class="kw-2">&amp;mut </span><span class="self">self</span>, cx: <span class="kw-2">&amp;mut </span>Context) -&gt; Poll&lt;<span class="prelude-ty">Result</span>&lt;Reason, <span class="kw">crate</span>::Error&gt;&gt; {
        <span class="self">self</span>.inner.poll_reset(cx, proto::PollReset::AwaitingHeaders)
    }

    <span class="doccomment">/// Returns the stream ID of the response stream.
    ///
    /// # Panics
    ///
    /// If the lock on the stream store has been poisoned.
    </span><span class="kw">pub fn </span>stream_id(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw">crate</span>::StreamId {
        <span class="kw">crate</span>::StreamId::from_internal(<span class="self">self</span>.inner.stream_id())
    }
}

<span class="comment">// ===== impl SendPushedResponse =====

</span><span class="kw">impl</span>&lt;B: Buf&gt; SendPushedResponse&lt;B&gt; {
    <span class="doccomment">/// Send a response to a promised request.
    ///
    /// On success, a [`SendStream`] instance is returned. This instance can be
    /// used to stream the response body and send trailers.
    ///
    /// If a body or trailers will be sent on the returned [`SendStream`]
    /// instance, then `end_of_stream` must be set to `false` when calling this
    /// function.
    ///
    /// The [`SendPushedResponse`] instance is associated with a promised
    /// request.  This function may only be called once per instance and only if
    /// [`send_reset`] has not been previously called.
    ///
    /// [`SendPushedResponse`]: #
    /// [`SendStream`]: ../struct.SendStream.html
    /// [`send_reset`]: #method.send_reset
    </span><span class="kw">pub fn </span>send_response(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        response: Response&lt;()&gt;,
        end_of_stream: bool,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;SendStream&lt;B&gt;, <span class="kw">crate</span>::Error&gt; {
        <span class="self">self</span>.inner.send_response(response, end_of_stream)
    }

    <span class="doccomment">/// Send a stream reset to the peer.
    ///
    /// This essentially cancels the stream, including any inbound or outbound
    /// data streams.
    ///
    /// If this function is called before [`send_response`], a call to
    /// [`send_response`] will result in an error.
    ///
    /// If this function is called while a [`SendStream`] instance is active,
    /// any further use of the instance will result in an error.
    ///
    /// This function should only be called once.
    ///
    /// [`send_response`]: #method.send_response
    /// [`SendStream`]: ../struct.SendStream.html
    </span><span class="kw">pub fn </span>send_reset(<span class="kw-2">&amp;mut </span><span class="self">self</span>, reason: Reason) {
        <span class="self">self</span>.inner.send_reset(reason)
    }

    <span class="doccomment">/// Polls to be notified when the client resets this stream.
    ///
    /// If stream is still open, this returns `Poll::Pending`, and
    /// registers the task to be notified if a `RST_STREAM` is received.
    ///
    /// If a `RST_STREAM` frame is received for this stream, calling this
    /// method will yield the `Reason` for the reset.
    ///
    /// # Error
    ///
    /// Calling this method after having called `send_response` will return
    /// a user error.
    </span><span class="kw">pub fn </span>poll_reset(<span class="kw-2">&amp;mut </span><span class="self">self</span>, cx: <span class="kw-2">&amp;mut </span>Context) -&gt; Poll&lt;<span class="prelude-ty">Result</span>&lt;Reason, <span class="kw">crate</span>::Error&gt;&gt; {
        <span class="self">self</span>.inner.poll_reset(cx)
    }

    <span class="doccomment">/// Returns the stream ID of the response stream.
    ///
    /// # Panics
    ///
    /// If the lock on the stream store has been poisoned.
    </span><span class="kw">pub fn </span>stream_id(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw">crate</span>::StreamId {
        <span class="self">self</span>.inner.stream_id()
    }
}

<span class="comment">// ===== impl Flush =====

</span><span class="kw">impl</span>&lt;T, B: Buf&gt; Flush&lt;T, B&gt; {
    <span class="kw">fn </span>new(codec: Codec&lt;T, B&gt;) -&gt; <span class="self">Self </span>{
        Flush { codec: <span class="prelude-val">Some</span>(codec) }
    }
}

<span class="kw">impl</span>&lt;T, B&gt; Future <span class="kw">for </span>Flush&lt;T, B&gt;
<span class="kw">where
    </span>T: AsyncWrite + Unpin,
    B: Buf,
{
    <span class="kw">type </span>Output = <span class="prelude-ty">Result</span>&lt;Codec&lt;T, B&gt;, <span class="kw">crate</span>::Error&gt;;

    <span class="kw">fn </span>poll(<span class="kw-2">mut </span><span class="self">self</span>: Pin&lt;<span class="kw-2">&amp;mut </span><span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>Context&lt;<span class="lifetime">'_</span>&gt;) -&gt; Poll&lt;<span class="self">Self</span>::Output&gt; {
        <span class="comment">// Flush the codec
        </span><span class="macro">ready!</span>(<span class="self">self</span>.codec.as_mut().unwrap().flush(cx)).map_err(<span class="kw">crate</span>::Error::from_io)<span class="question-mark">?</span>;

        <span class="comment">// Return the codec
        </span>Poll::Ready(<span class="prelude-val">Ok</span>(<span class="self">self</span>.codec.take().unwrap()))
    }
}

<span class="kw">impl</span>&lt;T, B: Buf&gt; ReadPreface&lt;T, B&gt; {
    <span class="kw">fn </span>new(codec: Codec&lt;T, B&gt;) -&gt; <span class="self">Self </span>{
        ReadPreface {
            codec: <span class="prelude-val">Some</span>(codec),
            pos: <span class="number">0</span>,
        }
    }

    <span class="kw">fn </span>inner_mut(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span>T {
        <span class="self">self</span>.codec.as_mut().unwrap().get_mut()
    }
}

<span class="kw">impl</span>&lt;T, B&gt; Future <span class="kw">for </span>ReadPreface&lt;T, B&gt;
<span class="kw">where
    </span>T: AsyncRead + Unpin,
    B: Buf,
{
    <span class="kw">type </span>Output = <span class="prelude-ty">Result</span>&lt;Codec&lt;T, B&gt;, <span class="kw">crate</span>::Error&gt;;

    <span class="kw">fn </span>poll(<span class="kw-2">mut </span><span class="self">self</span>: Pin&lt;<span class="kw-2">&amp;mut </span><span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>Context&lt;<span class="lifetime">'_</span>&gt;) -&gt; Poll&lt;<span class="self">Self</span>::Output&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>buf = [<span class="number">0</span>; <span class="number">24</span>];
        <span class="kw">let </span><span class="kw-2">mut </span>rem = PREFACE.len() - <span class="self">self</span>.pos;

        <span class="kw">while </span>rem &gt; <span class="number">0 </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>buf = ReadBuf::new(<span class="kw-2">&amp;mut </span>buf[..rem]);
            <span class="macro">ready!</span>(Pin::new(<span class="self">self</span>.inner_mut()).poll_read(cx, <span class="kw-2">&amp;mut </span>buf))
                .map_err(<span class="kw">crate</span>::Error::from_io)<span class="question-mark">?</span>;
            <span class="kw">let </span>n = buf.filled().len();
            <span class="kw">if </span>n == <span class="number">0 </span>{
                <span class="kw">return </span>Poll::Ready(<span class="prelude-val">Err</span>(<span class="kw">crate</span>::Error::from_io(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    <span class="string">"connection closed before reading preface"</span>,
                ))));
            }

            <span class="kw">if </span><span class="kw-2">&amp;</span>PREFACE[<span class="self">self</span>.pos..<span class="self">self</span>.pos + n] != buf.filled() {
                <span class="macro">proto_err!</span>(conn: <span class="string">"read_preface: invalid preface"</span>);
                <span class="comment">// TODO: Should this just write the GO_AWAY frame directly?
                </span><span class="kw">return </span>Poll::Ready(<span class="prelude-val">Err</span>(Error::library_go_away(Reason::PROTOCOL_ERROR).into()));
            }

            <span class="self">self</span>.pos += n;
            rem -= n; <span class="comment">// TODO test
        </span>}

        Poll::Ready(<span class="prelude-val">Ok</span>(<span class="self">self</span>.codec.take().unwrap()))
    }
}

<span class="comment">// ===== impl Handshake =====

</span><span class="kw">impl</span>&lt;T, B: Buf&gt; Future <span class="kw">for </span>Handshake&lt;T, B&gt;
<span class="kw">where
    </span>T: AsyncRead + AsyncWrite + Unpin,
    B: Buf,
{
    <span class="kw">type </span>Output = <span class="prelude-ty">Result</span>&lt;Connection&lt;T, B&gt;, <span class="kw">crate</span>::Error&gt;;

    <span class="kw">fn </span>poll(<span class="kw-2">mut </span><span class="self">self</span>: Pin&lt;<span class="kw-2">&amp;mut </span><span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>Context&lt;<span class="lifetime">'_</span>&gt;) -&gt; Poll&lt;<span class="self">Self</span>::Output&gt; {
        <span class="kw">let </span>span = <span class="self">self</span>.span.clone(); <span class="comment">// XXX(eliza): T_T
        </span><span class="kw">let </span>_e = span.enter();
        <span class="macro">tracing::trace!</span>(state = <span class="question-mark">?</span><span class="self">self</span>.state);

        <span class="kw">loop </span>{
            <span class="kw">match </span><span class="kw-2">&amp;mut </span><span class="self">self</span>.state {
                Handshaking::Flushing(flush) =&gt; {
                    <span class="comment">// We're currently flushing a pending SETTINGS frame. Poll the
                    // flush future, and, if it's completed, advance our state to wait
                    // for the client preface.
                    </span><span class="kw">let </span>codec = <span class="kw">match </span>Pin::new(flush).poll(cx)<span class="question-mark">? </span>{
                        Poll::Pending =&gt; {
                            <span class="macro">tracing::trace!</span>(flush.poll = %<span class="string">"Pending"</span>);
                            <span class="kw">return </span>Poll::Pending;
                        }
                        Poll::Ready(flushed) =&gt; {
                            <span class="macro">tracing::trace!</span>(flush.poll = %<span class="string">"Ready"</span>);
                            flushed
                        }
                    };
                    <span class="self">self</span>.state = Handshaking::ReadingPreface(
                        ReadPreface::new(codec).instrument(<span class="macro">tracing::trace_span!</span>(<span class="string">"read_preface"</span>)),
                    );
                }
                Handshaking::ReadingPreface(read) =&gt; {
                    <span class="kw">let </span>codec = <span class="macro">ready!</span>(Pin::new(read).poll(cx)<span class="question-mark">?</span>);

                    <span class="self">self</span>.state = Handshaking::Done;

                    <span class="kw">let </span>connection = proto::Connection::new(
                        codec,
                        Config {
                            next_stream_id: <span class="number">2</span>.into(),
                            <span class="comment">// Server does not need to locally initiate any streams
                            </span>initial_max_send_streams: <span class="number">0</span>,
                            max_send_buffer_size: <span class="self">self</span>.builder.max_send_buffer_size,
                            reset_stream_duration: <span class="self">self</span>.builder.reset_stream_duration,
                            reset_stream_max: <span class="self">self</span>.builder.reset_stream_max,
                            remote_reset_stream_max: <span class="self">self</span>.builder.pending_accept_reset_stream_max,
                            local_error_reset_streams_max: <span class="self">self
                                </span>.builder
                                .local_max_error_reset_streams,
                            settings: <span class="self">self</span>.builder.settings.clone(),
                        },
                    );

                    <span class="macro">tracing::trace!</span>(<span class="string">"connection established!"</span>);
                    <span class="kw">let </span><span class="kw-2">mut </span>c = Connection { connection };
                    <span class="kw">if let </span><span class="prelude-val">Some</span>(sz) = <span class="self">self</span>.builder.initial_target_connection_window_size {
                        c.set_target_window_size(sz);
                    }

                    <span class="kw">return </span>Poll::Ready(<span class="prelude-val">Ok</span>(c));
                }
                Handshaking::Done =&gt; {
                    <span class="macro">panic!</span>(<span class="string">"Handshaking::poll() called again after handshaking was complete"</span>)
                }
            }
        }
    }
}

<span class="kw">impl</span>&lt;T, B&gt; fmt::Debug <span class="kw">for </span>Handshake&lt;T, B&gt;
<span class="kw">where
    </span>T: AsyncRead + AsyncWrite + fmt::Debug,
    B: fmt::Debug + Buf,
{
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, fmt: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="macro">write!</span>(fmt, <span class="string">"server::Handshake"</span>)
    }
}

<span class="kw">impl </span>Peer {
    <span class="kw">pub fn </span>convert_send_message(
        id: StreamId,
        response: Response&lt;()&gt;,
        end_of_stream: bool,
    ) -&gt; frame::Headers {
        <span class="kw">use </span>http::response::Parts;

        <span class="comment">// Extract the components of the HTTP request
        </span><span class="kw">let </span>(
            Parts {
                status, headers, ..
            },
            <span class="kw">_</span>,
        ) = response.into_parts();

        <span class="comment">// Build the set pseudo header set. All requests will include `method`
        // and `path`.
        </span><span class="kw">let </span>pseudo = Pseudo::response(status);

        <span class="comment">// Create the HEADERS frame
        </span><span class="kw">let </span><span class="kw-2">mut </span>frame = frame::Headers::new(id, pseudo, headers);

        <span class="kw">if </span>end_of_stream {
            frame.set_end_stream()
        }

        frame
    }

    <span class="kw">pub fn </span>convert_push_message(
        stream_id: StreamId,
        promised_id: StreamId,
        request: Request&lt;()&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;frame::PushPromise, UserError&gt; {
        <span class="kw">use </span>http::request::Parts;

        <span class="kw">if let </span><span class="prelude-val">Err</span>(e) = frame::PushPromise::validate_request(<span class="kw-2">&amp;</span>request) {
            <span class="kw">use </span>PushPromiseHeaderError::<span class="kw-2">*</span>;
            <span class="kw">match </span>e {
                NotSafeAndCacheable =&gt; <span class="macro">tracing::debug!</span>(
                    <span class="question-mark">?</span>promised_id,
                    <span class="string">"convert_push_message: method {} is not safe and cacheable"</span>,
                    request.method(),
                ),
                InvalidContentLength(e) =&gt; <span class="macro">tracing::debug!</span>(
                    <span class="question-mark">?</span>promised_id,
                    <span class="string">"convert_push_message; promised request has invalid content-length {:?}"</span>,
                    e,
                ),
            }
            <span class="kw">return </span><span class="prelude-val">Err</span>(UserError::MalformedHeaders);
        }

        <span class="comment">// Extract the components of the HTTP request
        </span><span class="kw">let </span>(
            Parts {
                method,
                uri,
                headers,
                ..
            },
            <span class="kw">_</span>,
        ) = request.into_parts();

        <span class="kw">let </span>pseudo = Pseudo::request(method, uri, <span class="prelude-val">None</span>);

        <span class="prelude-val">Ok</span>(frame::PushPromise::new(
            stream_id,
            promised_id,
            pseudo,
            headers,
        ))
    }
}

<span class="kw">impl </span>proto::Peer <span class="kw">for </span>Peer {
    <span class="kw">type </span>Poll = Request&lt;()&gt;;

    <span class="kw">const </span>NAME: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str = <span class="string">"Server"</span>;

    <span class="comment">/*
    fn is_server() -&gt; bool {
        true
    }
    */

    </span><span class="kw">fn </span>r#dyn() -&gt; proto::DynPeer {
        proto::DynPeer::Server
    }

    <span class="kw">fn </span>convert_poll_message(
        pseudo: Pseudo,
        fields: HeaderMap,
        stream_id: StreamId,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>::Poll, Error&gt; {
        <span class="kw">use </span>http::{uri, Version};

        <span class="kw">let </span><span class="kw-2">mut </span>b = Request::builder();

        <span class="macro">macro_rules!</span> malformed {
            ($(<span class="macro-nonterminal">$arg</span>:tt)<span class="kw-2">*</span>) =&gt; {{
                <span class="macro">tracing::debug!</span>($(<span class="macro-nonterminal">$arg</span>)<span class="kw-2">*</span>);
                <span class="kw">return </span><span class="prelude-val">Err</span>(Error::library_reset(stream_id, Reason::PROTOCOL_ERROR));
            }}
        }

        b = b.version(Version::HTTP_2);

        <span class="kw">let </span>is_connect;
        <span class="kw">if let </span><span class="prelude-val">Some</span>(method) = pseudo.method {
            is_connect = method == Method::CONNECT;
            b = b.method(method);
        } <span class="kw">else </span>{
            <span class="macro">malformed!</span>(<span class="string">"malformed headers: missing method"</span>);
        }

        <span class="kw">let </span>has_protocol = pseudo.protocol.is_some();
        <span class="kw">if </span>has_protocol {
            <span class="kw">if </span>is_connect {
                <span class="comment">// Assert that we have the right type.
                </span>b = b.extension::&lt;<span class="kw">crate</span>::ext::Protocol&gt;(pseudo.protocol.unwrap());
            } <span class="kw">else </span>{
                <span class="macro">malformed!</span>(<span class="string">"malformed headers: :protocol on non-CONNECT request"</span>);
            }
        }

        <span class="kw">if </span>pseudo.status.is_some() {
            <span class="macro">malformed!</span>(<span class="string">"malformed headers: :status field on request"</span>);
        }

        <span class="comment">// Convert the URI
        </span><span class="kw">let </span><span class="kw-2">mut </span>parts = uri::Parts::default();

        <span class="comment">// A request translated from HTTP/1 must not include the :authority
        // header
        </span><span class="kw">if let </span><span class="prelude-val">Some</span>(authority) = pseudo.authority {
            <span class="kw">let </span>maybe_authority = uri::Authority::from_maybe_shared(authority.clone().into_inner());
            parts.authority = <span class="prelude-val">Some</span>(maybe_authority.or_else(|why| {
                <span class="macro">malformed!</span>(
                    <span class="string">"malformed headers: malformed authority ({:?}): {}"</span>,
                    authority,
                    why,
                )
            })<span class="question-mark">?</span>);
        }

        <span class="comment">// A :scheme is required, except CONNECT.
        </span><span class="kw">if let </span><span class="prelude-val">Some</span>(scheme) = pseudo.scheme {
            <span class="kw">if </span>is_connect &amp;&amp; !has_protocol {
                <span class="macro">malformed!</span>(<span class="string">"malformed headers: :scheme in CONNECT"</span>);
            }
            <span class="kw">let </span>maybe_scheme = scheme.parse();
            <span class="kw">let </span>scheme = maybe_scheme.or_else(|why| {
                <span class="macro">malformed!</span>(
                    <span class="string">"malformed headers: malformed scheme ({:?}): {}"</span>,
                    scheme,
                    why,
                )
            })<span class="question-mark">?</span>;

            <span class="comment">// It's not possible to build an `Uri` from a scheme and path. So,
            // after validating is was a valid scheme, we just have to drop it
            // if there isn't an :authority.
            </span><span class="kw">if </span>parts.authority.is_some() {
                parts.scheme = <span class="prelude-val">Some</span>(scheme);
            }
        } <span class="kw">else if </span>!is_connect || has_protocol {
            <span class="macro">malformed!</span>(<span class="string">"malformed headers: missing scheme"</span>);
        }

        <span class="kw">if let </span><span class="prelude-val">Some</span>(path) = pseudo.path {
            <span class="kw">if </span>is_connect &amp;&amp; !has_protocol {
                <span class="macro">malformed!</span>(<span class="string">"malformed headers: :path in CONNECT"</span>);
            }

            <span class="comment">// This cannot be empty
            </span><span class="kw">if </span>path.is_empty() {
                <span class="macro">malformed!</span>(<span class="string">"malformed headers: missing path"</span>);
            }

            <span class="kw">let </span>maybe_path = uri::PathAndQuery::from_maybe_shared(path.clone().into_inner());
            parts.path_and_query = <span class="prelude-val">Some</span>(maybe_path.or_else(|why| {
                <span class="macro">malformed!</span>(<span class="string">"malformed headers: malformed path ({:?}): {}"</span>, path, why,)
            })<span class="question-mark">?</span>);
        } <span class="kw">else if </span>is_connect &amp;&amp; has_protocol {
            <span class="macro">malformed!</span>(<span class="string">"malformed headers: missing path in extended CONNECT"</span>);
        }

        b = b.uri(parts);

        <span class="kw">let </span><span class="kw-2">mut </span>request = <span class="kw">match </span>b.body(()) {
            <span class="prelude-val">Ok</span>(request) =&gt; request,
            <span class="prelude-val">Err</span>(e) =&gt; {
                <span class="comment">// TODO: Should there be more specialized handling for different
                // kinds of errors
                </span><span class="macro">proto_err!</span>(stream: <span class="string">"error building request: {}; stream={:?}"</span>, e, stream_id);
                <span class="kw">return </span><span class="prelude-val">Err</span>(Error::library_reset(stream_id, Reason::PROTOCOL_ERROR));
            }
        };

        <span class="kw-2">*</span>request.headers_mut() = fields;

        <span class="prelude-val">Ok</span>(request)
    }
}

<span class="comment">// ===== impl Handshaking =====

</span><span class="kw">impl</span>&lt;T, B&gt; fmt::Debug <span class="kw">for </span>Handshaking&lt;T, B&gt;
<span class="kw">where
    </span>B: Buf,
{
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; <span class="prelude-ty">Result</span>&lt;(), fmt::Error&gt; {
        <span class="kw">match </span><span class="kw-2">*</span><span class="self">self </span>{
            Handshaking::Flushing(<span class="kw">_</span>) =&gt; f.write_str(<span class="string">"Flushing(_)"</span>),
            Handshaking::ReadingPreface(<span class="kw">_</span>) =&gt; f.write_str(<span class="string">"ReadingPreface(_)"</span>),
            Handshaking::Done =&gt; f.write_str(<span class="string">"Done"</span>),
        }
    }
}
</code></pre></div></section></main></body></html>