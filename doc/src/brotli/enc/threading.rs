<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/brotli-4.0.0/src/enc/threading.rs`."><title>threading.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="brotli" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../brotli/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">super</span>::backward_references::{AnyHasher, BrotliEncoderParams, CloneWithAlloc, UnionHasher};
<span class="kw">use </span><span class="kw">super</span>::encode::{
    BrotliEncoderCompressStream, BrotliEncoderCreateInstance, BrotliEncoderDestroyInstance,
    BrotliEncoderMaxCompressedSize, BrotliEncoderOperation,
    BrotliEncoderSetCustomDictionaryWithOptionalPrecomputedHasher, HasherSetup, SanitizeParams,
};
<span class="kw">use </span><span class="kw">super</span>::BrotliAlloc;
<span class="kw">use </span>alloc::{Allocator, SliceWrapper, SliceWrapperMut};
<span class="kw">use </span>concat::{BroCatli, BroCatliResult};
<span class="kw">use </span>core::any;
<span class="kw">use </span>core::marker::PhantomData;
<span class="kw">use </span>core::mem;
<span class="kw">use </span>core::ops::Range;
<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">use </span>std;
<span class="kw">pub type </span>PoisonedThreadError = ();

<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">pub type </span>LowLevelThreadError = std::boxed::Box&lt;<span class="kw">dyn </span>any::Any + Send + <span class="lifetime">'static</span>&gt;;
<span class="attr">#[cfg(not(feature = <span class="string">"std"</span>))]
</span><span class="kw">pub type </span>LowLevelThreadError = ();

<span class="kw">pub trait </span>AnyBoxConstructor {
    <span class="kw">fn </span>new(data: LowLevelThreadError) -&gt; <span class="self">Self</span>;
}

<span class="kw">pub trait </span>Joinable&lt;T: Send + <span class="lifetime">'static</span>, U: Send + <span class="lifetime">'static</span>&gt;: Sized {
    <span class="kw">fn </span>join(<span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;T, U&gt;;
}
<span class="attr">#[derive(Debug)]
</span><span class="kw">pub enum </span>BrotliEncoderThreadError {
    InsufficientOutputSpace,
    ConcatenationDidNotProcessFullFile,
    ConcatenationError(BroCatliResult),
    ConcatenationFinalizationError(BroCatliResult),
    OtherThreadPanic,
    ThreadExecError(LowLevelThreadError),
}

<span class="kw">impl </span>AnyBoxConstructor <span class="kw">for </span>BrotliEncoderThreadError {
    <span class="kw">fn </span>new(data: LowLevelThreadError) -&gt; <span class="self">Self </span>{
        BrotliEncoderThreadError::ThreadExecError(data)
    }
}

<span class="kw">pub struct </span>CompressedFileChunk&lt;Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>&gt;
<span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send,
{
    data_backing: &lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory,
    data_size: usize,
}
<span class="kw">pub struct </span>CompressionThreadResult&lt;Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>&gt;
<span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send,
{
    compressed: <span class="prelude-ty">Result</span>&lt;CompressedFileChunk&lt;Alloc&gt;, BrotliEncoderThreadError&gt;,
    alloc: Alloc,
}
<span class="kw">pub enum </span>InternalSendAlloc&lt;
    ReturnVal: Send + <span class="lifetime">'static</span>,
    ExtraInput: Send + <span class="lifetime">'static</span>,
    Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>,
    Join: Joinable&lt;ReturnVal, BrotliEncoderThreadError&gt;,
&gt; <span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send,
{
    A(Alloc, ExtraInput),
    Join(Join),
    SpawningOrJoining(PhantomData&lt;ReturnVal&gt;),
}
<span class="kw">impl</span>&lt;
        ReturnVal: Send + <span class="lifetime">'static</span>,
        ExtraInput: Send + <span class="lifetime">'static</span>,
        Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>,
        Join: Joinable&lt;ReturnVal, BrotliEncoderThreadError&gt;,
    &gt; InternalSendAlloc&lt;ReturnVal, ExtraInput, Alloc, Join&gt;
<span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send,
{
    <span class="kw">fn </span>unwrap_input(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; (<span class="kw-2">&amp;mut </span>Alloc, <span class="kw-2">&amp;mut </span>ExtraInput) {
        <span class="kw">match </span><span class="kw-2">*</span><span class="self">self </span>{
            InternalSendAlloc::A(<span class="kw-2">ref mut </span>alloc, <span class="kw-2">ref mut </span>extra) =&gt; (alloc, extra),
            <span class="kw">_ </span>=&gt; <span class="macro">panic!</span>(<span class="string">"Bad state for allocator"</span>),
        }
    }
}

<span class="kw">pub struct </span>SendAlloc&lt;
    ReturnValue: Send + <span class="lifetime">'static</span>,
    ExtraInput: Send + <span class="lifetime">'static</span>,
    Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>,
    Join: Joinable&lt;ReturnValue, BrotliEncoderThreadError&gt;,
&gt;(<span class="kw">pub </span>InternalSendAlloc&lt;ReturnValue, ExtraInput, Alloc, Join&gt;)
<span class="comment">//FIXME pub
</span><span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send;

<span class="kw">impl</span>&lt;
        ReturnValue: Send + <span class="lifetime">'static</span>,
        ExtraInput: Send + <span class="lifetime">'static</span>,
        Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>,
        Join: Joinable&lt;ReturnValue, BrotliEncoderThreadError&gt;,
    &gt; SendAlloc&lt;ReturnValue, ExtraInput, Alloc, Join&gt;
<span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send,
{
    <span class="kw">pub fn </span>new(alloc: Alloc, extra_input: ExtraInput) -&gt; <span class="self">Self </span>{
        SendAlloc::&lt;ReturnValue, ExtraInput, Alloc, Join&gt;(InternalSendAlloc::A(alloc, extra_input))
    }
    <span class="kw">pub fn </span>unwrap_or(<span class="self">self</span>, other: Alloc, other_extra: ExtraInput) -&gt; (Alloc, ExtraInput) {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0 </span>{
            InternalSendAlloc::A(alloc, extra_input) =&gt; (alloc, extra_input),
            InternalSendAlloc::SpawningOrJoining(<span class="kw">_</span>) | InternalSendAlloc::Join(<span class="kw">_</span>) =&gt; {
                (other, other_extra)
            }
        }
    }
    <span class="kw">fn </span>unwrap_view_mut(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; (<span class="kw-2">&amp;mut </span>Alloc, <span class="kw-2">&amp;mut </span>ExtraInput) {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0 </span>{
            InternalSendAlloc::A(<span class="kw-2">ref mut </span>alloc, <span class="kw-2">ref mut </span>extra_input) =&gt; (alloc, extra_input),
            InternalSendAlloc::SpawningOrJoining(<span class="kw">_</span>) | InternalSendAlloc::Join(<span class="kw">_</span>) =&gt; {
                <span class="macro">panic!</span>(<span class="string">"Item permanently borrowed/leaked"</span>)
            }
        }
    }
    <span class="kw">pub fn </span>unwrap(<span class="self">self</span>) -&gt; (Alloc, ExtraInput) {
        <span class="kw">match </span><span class="self">self</span>.<span class="number">0 </span>{
            InternalSendAlloc::A(alloc, extra_input) =&gt; (alloc, extra_input),
            InternalSendAlloc::SpawningOrJoining(<span class="kw">_</span>) | InternalSendAlloc::Join(<span class="kw">_</span>) =&gt; {
                <span class="macro">panic!</span>(<span class="string">"Item permanently borrowed/leaked"</span>)
            }
        }
    }
    <span class="kw">pub fn </span>replace_with_default(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; (Alloc, ExtraInput) {
        <span class="kw">match </span>mem::replace(
            <span class="kw-2">&amp;mut </span><span class="self">self</span>.<span class="number">0</span>,
            InternalSendAlloc::SpawningOrJoining(PhantomData),
        ) {
            InternalSendAlloc::A(alloc, extra_input) =&gt; (alloc, extra_input),
            InternalSendAlloc::SpawningOrJoining(<span class="kw">_</span>) | InternalSendAlloc::Join(<span class="kw">_</span>) =&gt; {
                <span class="macro">panic!</span>(<span class="string">"Item permanently borrowed/leaked"</span>)
            }
        }
    }
}

<span class="kw">pub enum </span>InternalOwned&lt;T&gt; {
    <span class="comment">// FIXME pub
    </span>Item(T),
    Borrowed,
}

<span class="kw">pub struct </span>Owned&lt;T&gt;(<span class="kw">pub </span>InternalOwned&lt;T&gt;); <span class="comment">// FIXME pub
</span><span class="kw">impl</span>&lt;T&gt; Owned&lt;T&gt; {
    <span class="kw">pub fn </span>new(data: T) -&gt; <span class="self">Self </span>{
        Owned::&lt;T&gt;(InternalOwned::Item(data))
    }
    <span class="kw">pub fn </span>unwrap_or(<span class="self">self</span>, other: T) -&gt; T {
        <span class="kw">if let </span>InternalOwned::Item(x) = <span class="self">self</span>.<span class="number">0 </span>{
            x
        } <span class="kw">else </span>{
            other
        }
    }
    <span class="kw">pub fn </span>unwrap(<span class="self">self</span>) -&gt; T {
        <span class="kw">if let </span>InternalOwned::Item(x) = <span class="self">self</span>.<span class="number">0 </span>{
            x
        } <span class="kw">else </span>{
            <span class="macro">panic!</span>(<span class="string">"Item permanently borrowed"</span>)
        }
    }
    <span class="kw">pub fn </span>view(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>T {
        <span class="kw">if let </span>InternalOwned::Item(<span class="kw-2">ref </span>x) = <span class="self">self</span>.<span class="number">0 </span>{
            x
        } <span class="kw">else </span>{
            <span class="macro">panic!</span>(<span class="string">"Item permanently borrowed"</span>)
        }
    }
}

<span class="kw">pub trait </span>OwnedRetriever&lt;U: Send + <span class="lifetime">'static</span>&gt; {
    <span class="kw">fn </span>view&lt;T, F: FnOnce(<span class="kw-2">&amp;</span>U) -&gt; T&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, f: F) -&gt; <span class="prelude-ty">Result</span>&lt;T, PoisonedThreadError&gt;;
    <span class="kw">fn </span>unwrap(<span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;U, PoisonedThreadError&gt;;
}

<span class="attr">#[cfg(feature = <span class="string">"std"</span>)]
</span><span class="kw">impl</span>&lt;U: Send + <span class="lifetime">'static</span>&gt; OwnedRetriever&lt;U&gt; <span class="kw">for </span>std::sync::Arc&lt;std::sync::RwLock&lt;U&gt;&gt; {
    <span class="kw">fn </span>view&lt;T, F: FnOnce(<span class="kw-2">&amp;</span>U) -&gt; T&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, f: F) -&gt; <span class="prelude-ty">Result</span>&lt;T, PoisonedThreadError&gt; {
        <span class="kw">match </span><span class="self">self</span>.read() {
            <span class="prelude-val">Ok</span>(<span class="kw-2">ref </span>u) =&gt; <span class="prelude-val">Ok</span>(f(u)),
            <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; <span class="prelude-val">Err</span>(PoisonedThreadError::default()),
        }
    }
    <span class="kw">fn </span>unwrap(<span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;U, PoisonedThreadError&gt; {
        <span class="kw">match </span>std::sync::Arc::try_unwrap(<span class="self">self</span>) {
            <span class="prelude-val">Ok</span>(rwlock) =&gt; <span class="kw">match </span>rwlock.into_inner() {
                <span class="prelude-val">Ok</span>(u) =&gt; <span class="prelude-val">Ok</span>(u),
                <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; <span class="prelude-val">Err</span>(PoisonedThreadError::default()),
            },
            <span class="prelude-val">Err</span>(<span class="kw">_</span>) =&gt; <span class="prelude-val">Err</span>(PoisonedThreadError::default()),
        }
    }
}

<span class="kw">pub trait </span>BatchSpawnable&lt;
    ReturnValue: Send + <span class="lifetime">'static</span>,
    ExtraInput: Send + <span class="lifetime">'static</span>,
    Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>,
    U: Send + <span class="lifetime">'static </span>+ Sync,
&gt; <span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send + <span class="lifetime">'static</span>,
{
    <span class="kw">type </span>JoinHandle: Joinable&lt;ReturnValue, BrotliEncoderThreadError&gt;;
    <span class="kw">type </span>FinalJoinHandle: OwnedRetriever&lt;U&gt;;
    <span class="comment">// this function takes in an input slice
    // a SendAlloc per thread and converts them all into JoinHandle
    // the input is borrowed until the joins complete
    // owned is set to borrowed
    // the final join handle is a r/w lock which will return the SliceW to the owner
    // the FinalJoinHandle is only to be called when each individual JoinHandle has been examined
    // the function is called with the thread_index, the num_threads, a reference to the slice under a read lock,
    // and an allocator from the alloc_per_thread
    </span><span class="kw">fn </span>make_spawner(<span class="kw-2">&amp;mut </span><span class="self">self</span>, input: <span class="kw-2">&amp;mut </span>Owned&lt;U&gt;) -&gt; <span class="self">Self</span>::FinalJoinHandle;
    <span class="kw">fn </span>spawn&lt;F: Fn(ExtraInput, usize, usize, <span class="kw-2">&amp;</span>U, Alloc) -&gt; ReturnValue + Send + <span class="lifetime">'static </span>+ Copy&gt;(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        handle: <span class="kw-2">&amp;mut </span><span class="self">Self</span>::FinalJoinHandle,
        alloc: <span class="kw-2">&amp;mut </span>SendAlloc&lt;ReturnValue, ExtraInput, Alloc, <span class="self">Self</span>::JoinHandle&gt;,
        index: usize,
        num_threads: usize,
        f: F,
    );
}

<span class="kw">pub trait </span>BatchSpawnableLite&lt;
    ReturnValue: Send + <span class="lifetime">'static</span>,
    ExtraInput: Send + <span class="lifetime">'static</span>,
    Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>,
    U: Send + <span class="lifetime">'static </span>+ Sync,
&gt; <span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send + <span class="lifetime">'static</span>,
{
    <span class="kw">type </span>JoinHandle: Joinable&lt;ReturnValue, BrotliEncoderThreadError&gt;;
    <span class="kw">type </span>FinalJoinHandle: OwnedRetriever&lt;U&gt;;
    <span class="kw">fn </span>make_spawner(<span class="kw-2">&amp;mut </span><span class="self">self</span>, input: <span class="kw-2">&amp;mut </span>Owned&lt;U&gt;) -&gt; <span class="self">Self</span>::FinalJoinHandle;
    <span class="kw">fn </span>spawn(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        handle: <span class="kw-2">&amp;mut </span><span class="self">Self</span>::FinalJoinHandle,
        alloc_per_thread: <span class="kw-2">&amp;mut </span>SendAlloc&lt;ReturnValue, ExtraInput, Alloc, <span class="self">Self</span>::JoinHandle&gt;,
        index: usize,
        num_threads: usize,
        f: <span class="kw">fn</span>(ExtraInput, usize, usize, <span class="kw-2">&amp;</span>U, Alloc) -&gt; ReturnValue,
    );
}
<span class="comment">/*
impl&lt;ReturnValue:Send+'static,
     ExtraInput:Send+'static,
     Alloc:BrotliAlloc+Send+'static,
     U:Send+'static+Sync&gt;
     BatchSpawnableLite&lt;T, Alloc, U&gt; for BatchSpawnable&lt;T, Alloc, U&gt; {
  type JoinHandle = &lt;Self as BatchSpawnable&lt;T, Alloc, U&gt;&gt;::JoinHandle;
  type FinalJoinHandle = &lt;Self as BatchSpawnable&lt;T, Alloc, U&gt;&gt;::FinalJoinHandle;
  fn batch_spawn(
    &amp;mut self,
    input: &amp;mut Owned&lt;U&gt;,
    alloc_per_thread:&amp;mut [SendAlloc&lt;ReturnValue, ExtraInput, Alloc, Self::JoinHandle&gt;],
    f: fn(usize, usize, &amp;U, Alloc) -&gt; T,
  ) -&gt; Self::FinalJoinHandle {
   &lt;Self as BatchSpawnable&lt;ReturnValue, ExtraInput,  Alloc, U&gt;&gt;::batch_spawn(self, input, alloc_per_thread, f)
  }
}*/

</span><span class="kw">pub fn </span>CompressMultiSlice&lt;
    Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>,
    Spawner: BatchSpawnableLite&lt;
        CompressionThreadResult&lt;Alloc&gt;,
        UnionHasher&lt;Alloc&gt;,
        Alloc,
        (
            &lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory,
            BrotliEncoderParams,
        ),
    &gt;,
&gt;(
    params: <span class="kw-2">&amp;</span>BrotliEncoderParams,
    input_slice: <span class="kw-2">&amp;</span>[u8],
    output: <span class="kw-2">&amp;mut </span>[u8],
    alloc_per_thread: <span class="kw-2">&amp;mut </span>[SendAlloc&lt;
        CompressionThreadResult&lt;Alloc&gt;,
        UnionHasher&lt;Alloc&gt;,
        Alloc,
        Spawner::JoinHandle,
    &gt;],
    thread_spawner: <span class="kw-2">&amp;mut </span>Spawner,
) -&gt; <span class="prelude-ty">Result</span>&lt;usize, BrotliEncoderThreadError&gt;
<span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send + Sync,
    &lt;Alloc <span class="kw">as </span>Allocator&lt;u16&gt;&gt;::AllocatedMemory: Send + Sync,
    &lt;Alloc <span class="kw">as </span>Allocator&lt;u32&gt;&gt;::AllocatedMemory: Send + Sync,
{
    <span class="kw">let </span>input = <span class="kw">if let </span>InternalSendAlloc::A(<span class="kw-2">ref mut </span>alloc, <span class="kw-2">ref </span>_extra) = alloc_per_thread[<span class="number">0</span>].<span class="number">0 </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>input = &lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::alloc_cell(alloc, input_slice.len());
        input.slice_mut().clone_from_slice(input_slice);
        input
    } <span class="kw">else </span>{
        &lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory::default()
    };
    <span class="kw">let </span><span class="kw-2">mut </span>owned_input = Owned::new(input);
    <span class="kw">let </span>ret = CompressMulti(
        params,
        <span class="kw-2">&amp;mut </span>owned_input,
        output,
        alloc_per_thread,
        thread_spawner,
    );
    <span class="kw">if let </span>InternalSendAlloc::A(<span class="kw-2">ref mut </span>alloc, <span class="kw-2">ref </span>_extra) = alloc_per_thread[<span class="number">0</span>].<span class="number">0 </span>{
        &lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::free_cell(alloc, owned_input.unwrap());
    }
    ret
}

<span class="kw">fn </span>get_range(thread_index: usize, num_threads: usize, file_size: usize) -&gt; Range&lt;usize&gt; {
    ((thread_index * file_size) / num_threads)..(((thread_index + <span class="number">1</span>) * file_size) / num_threads)
}

<span class="kw">fn </span>compress_part&lt;Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>, SliceW: SliceWrapper&lt;u8&gt;&gt;(
    hasher: UnionHasher&lt;Alloc&gt;,
    thread_index: usize,
    num_threads: usize,
    input_and_params: <span class="kw-2">&amp;</span>(SliceW, BrotliEncoderParams),
    <span class="kw-2">mut </span>alloc: Alloc,
) -&gt; CompressionThreadResult&lt;Alloc&gt;
<span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send + <span class="lifetime">'static</span>,
{
    <span class="kw">let </span><span class="kw-2">mut </span>range = get_range(thread_index, num_threads, input_and_params.<span class="number">0</span>.len());
    <span class="kw">let </span><span class="kw-2">mut </span>mem = &lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::alloc_cell(
        <span class="kw-2">&amp;mut </span>alloc,
        BrotliEncoderMaxCompressedSize(range.end - range.start),
    );
    <span class="kw">let </span><span class="kw-2">mut </span>state = BrotliEncoderCreateInstance(alloc);
    state.params = input_and_params.<span class="number">1</span>.clone();
    <span class="kw">if </span>thread_index != <span class="number">0 </span>{
        state.params.catable = <span class="bool-val">true</span>; <span class="comment">// make sure we can concatenate this to the other work results
        </span>state.params.magic_number = <span class="bool-val">false</span>; <span class="comment">// no reason to pepper this around
    </span>}
    state.params.appendable = <span class="bool-val">true</span>; <span class="comment">// make sure we are at least appendable, so that future items can be catted in
    </span><span class="kw">if </span>thread_index != <span class="number">0 </span>{
        BrotliEncoderSetCustomDictionaryWithOptionalPrecomputedHasher(
            <span class="kw-2">&amp;mut </span>state,
            range.start,
            <span class="kw-2">&amp;</span>input_and_params.<span class="number">0</span>.slice()[..range.start],
            hasher,
        );
    }
    <span class="kw">let </span><span class="kw-2">mut </span>out_offset = <span class="number">0usize</span>;
    <span class="kw">let </span>compression_result;
    <span class="kw">let </span><span class="kw-2">mut </span>available_out = mem.len();
    <span class="kw">loop </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>next_in_offset = <span class="number">0usize</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>available_in = range.end - range.start;
        <span class="kw">let </span>result = BrotliEncoderCompressStream(
            <span class="kw-2">&amp;mut </span>state,
            BrotliEncoderOperation::BROTLI_OPERATION_FINISH,
            <span class="kw-2">&amp;mut </span>available_in,
            <span class="kw-2">&amp;</span>input_and_params.<span class="number">0</span>.slice()[range.clone()],
            <span class="kw-2">&amp;mut </span>next_in_offset,
            <span class="kw-2">&amp;mut </span>available_out,
            mem.slice_mut(),
            <span class="kw-2">&amp;mut </span>out_offset,
            <span class="kw-2">&amp;mut </span><span class="prelude-val">None</span>,
            <span class="kw-2">&amp;mut </span>|_a, _b, _c, _d| (),
        );
        <span class="kw">let </span>new_range = range.start + next_in_offset..range.end;
        range = new_range;
        <span class="kw">if </span>result != <span class="number">0 </span>{
            compression_result = <span class="prelude-val">Ok</span>(out_offset);
            <span class="kw">break</span>;
        } <span class="kw">else if </span>available_out == <span class="number">0 </span>{
            compression_result = <span class="prelude-val">Err</span>(BrotliEncoderThreadError::InsufficientOutputSpace); <span class="comment">// mark no space??
            </span><span class="kw">break</span>;
        }
    }
    BrotliEncoderDestroyInstance(<span class="kw-2">&amp;mut </span>state);
    <span class="kw">match </span>compression_result {
        <span class="prelude-val">Ok</span>(size) =&gt; CompressionThreadResult::&lt;Alloc&gt; {
            compressed: <span class="prelude-val">Ok</span>(CompressedFileChunk {
                data_backing: mem,
                data_size: size,
            }),
            alloc: state.m8,
        },
        <span class="prelude-val">Err</span>(e) =&gt; {
            &lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::free_cell(<span class="kw-2">&amp;mut </span>state.m8, mem);
            CompressionThreadResult::&lt;Alloc&gt; {
                compressed: <span class="prelude-val">Err</span>(e),
                alloc: state.m8,
            }
        }
    }
}

<span class="kw">pub fn </span>CompressMulti&lt;
    Alloc: BrotliAlloc + Send + <span class="lifetime">'static</span>,
    SliceW: SliceWrapper&lt;u8&gt; + Send + <span class="lifetime">'static </span>+ Sync,
    Spawner: BatchSpawnableLite&lt;
        CompressionThreadResult&lt;Alloc&gt;,
        UnionHasher&lt;Alloc&gt;,
        Alloc,
        (SliceW, BrotliEncoderParams),
    &gt;,
&gt;(
    params: <span class="kw-2">&amp;</span>BrotliEncoderParams,
    owned_input: <span class="kw-2">&amp;mut </span>Owned&lt;SliceW&gt;,
    output: <span class="kw-2">&amp;mut </span>[u8],
    alloc_per_thread: <span class="kw-2">&amp;mut </span>[SendAlloc&lt;
        CompressionThreadResult&lt;Alloc&gt;,
        UnionHasher&lt;Alloc&gt;,
        Alloc,
        Spawner::JoinHandle,
    &gt;],
    thread_spawner: <span class="kw-2">&amp;mut </span>Spawner,
) -&gt; <span class="prelude-ty">Result</span>&lt;usize, BrotliEncoderThreadError&gt;
<span class="kw">where
    </span>&lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::AllocatedMemory: Send,
    &lt;Alloc <span class="kw">as </span>Allocator&lt;u16&gt;&gt;::AllocatedMemory: Send,
    &lt;Alloc <span class="kw">as </span>Allocator&lt;u32&gt;&gt;::AllocatedMemory: Send,
{
    <span class="kw">let </span>num_threads = alloc_per_thread.len();
    <span class="kw">let </span>actually_owned_mem = mem::replace(owned_input, Owned(InternalOwned::Borrowed));
    <span class="kw">let </span><span class="kw-2">mut </span>owned_input_pair = Owned::new((actually_owned_mem.unwrap(), params.clone()));
    <span class="comment">// start thread spawner
    </span><span class="kw">let </span><span class="kw-2">mut </span>spawner_and_input = thread_spawner.make_spawner(<span class="kw-2">&amp;mut </span>owned_input_pair);
    <span class="kw">if </span>num_threads &gt; <span class="number">1 </span>{
        <span class="comment">// spawn first thread without "custom dictionary" while we compute the custom dictionary for other work items
        </span>thread_spawner.spawn(
            <span class="kw-2">&amp;mut </span>spawner_and_input,
            <span class="kw-2">&amp;mut </span>alloc_per_thread[<span class="number">0</span>],
            <span class="number">0</span>,
            num_threads,
            compress_part,
        );
    }
    <span class="comment">// populate all hashers at once, cloning them one by one
    </span><span class="kw">let </span><span class="kw-2">mut </span>compression_last_thread_result;
    <span class="kw">if </span>num_threads &gt; <span class="number">1 </span>&amp;&amp; params.favor_cpu_efficiency {
        <span class="kw">let </span><span class="kw-2">mut </span>local_params = params.clone();
        SanitizeParams(<span class="kw-2">&amp;mut </span>local_params);
        <span class="kw">let </span><span class="kw-2">mut </span>hasher = UnionHasher::Uninit;
        HasherSetup(
            alloc_per_thread[num_threads - <span class="number">1</span>].<span class="number">0</span>.unwrap_input().<span class="number">0</span>,
            <span class="kw-2">&amp;mut </span>hasher,
            <span class="kw-2">&amp;mut </span>local_params,
            <span class="kw-2">&amp;</span>[],
            <span class="number">0</span>,
            <span class="number">0</span>,
            <span class="number">0</span>,
        );
        <span class="kw">for </span>thread_index <span class="kw">in </span><span class="number">1</span>..num_threads {
            <span class="kw">let </span>res = spawner_and_input.view(|input_and_params: <span class="kw-2">&amp;</span>(SliceW, BrotliEncoderParams)| {
                <span class="kw">let </span>range = get_range(thread_index - <span class="number">1</span>, num_threads, input_and_params.<span class="number">0</span>.len());
                <span class="kw">let </span>overlap = hasher.StoreLookahead().wrapping_sub(<span class="number">1</span>);
                <span class="kw">if </span>range.end - range.start &gt; overlap {
                    hasher.BulkStoreRange(
                        input_and_params.<span class="number">0</span>.slice(),
                        !(<span class="number">0</span>),
                        <span class="kw">if </span>range.start &gt; overlap {
                            range.start - overlap
                        } <span class="kw">else </span>{
                            <span class="number">0
                        </span>},
                        range.end - overlap,
                    );
                }
            });
            <span class="kw">if let </span><span class="prelude-val">Err</span>(_e) = res {
                <span class="kw">return </span><span class="prelude-val">Err</span>(BrotliEncoderThreadError::OtherThreadPanic);
            }
            <span class="kw">if </span>thread_index + <span class="number">1 </span>!= num_threads {
                {
                    <span class="kw">let </span>(alloc, out_hasher) = alloc_per_thread[thread_index].unwrap_view_mut();
                    <span class="kw-2">*</span>out_hasher = hasher.clone_with_alloc(alloc);
                }
                thread_spawner.spawn(
                    <span class="kw-2">&amp;mut </span>spawner_and_input,
                    <span class="kw-2">&amp;mut </span>alloc_per_thread[thread_index],
                    thread_index,
                    num_threads,
                    compress_part,
                );
            }
        }
        <span class="kw">let </span>(alloc, _extra) = alloc_per_thread[num_threads - <span class="number">1</span>].replace_with_default();
        compression_last_thread_result = spawner_and_input.view(<span class="kw">move </span>|input_and_params:<span class="kw-2">&amp;</span>(SliceW, BrotliEncoderParams)| -&gt; CompressionThreadResult&lt;Alloc&gt; {
        compress_part(hasher,
                      num_threads - <span class="number">1</span>,
                      num_threads,
                      input_and_params,
                      alloc,
        )
      });
    } <span class="kw">else </span>{
        <span class="kw">if </span>num_threads &gt; <span class="number">1 </span>{
            <span class="kw">for </span>thread_index <span class="kw">in </span><span class="number">1</span>..num_threads - <span class="number">1 </span>{
                thread_spawner.spawn(
                    <span class="kw-2">&amp;mut </span>spawner_and_input,
                    <span class="kw-2">&amp;mut </span>alloc_per_thread[thread_index],
                    thread_index,
                    num_threads,
                    compress_part,
                );
            }
        }
        <span class="kw">let </span>(alloc, _extra) = alloc_per_thread[num_threads - <span class="number">1</span>].replace_with_default();
        compression_last_thread_result = spawner_and_input.view(<span class="kw">move </span>|input_and_params:<span class="kw-2">&amp;</span>(SliceW, BrotliEncoderParams)| -&gt; CompressionThreadResult&lt;Alloc&gt; {
        compress_part(UnionHasher::Uninit,
                      num_threads - <span class="number">1</span>,
                      num_threads,
                      input_and_params,
                      alloc,
        )
      });
    }
    <span class="kw">let </span><span class="kw-2">mut </span>compression_result = <span class="prelude-val">Err</span>(BrotliEncoderThreadError::InsufficientOutputSpace);
    <span class="kw">let </span><span class="kw-2">mut </span>out_file_size = <span class="number">0usize</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>bro_cat_li = BroCatli::new();
    <span class="kw">for </span>(index, thread) <span class="kw">in </span>alloc_per_thread.iter_mut().enumerate() {
        <span class="kw">let </span><span class="kw-2">mut </span>cur_result = <span class="kw">if </span>index + <span class="number">1 </span>== num_threads {
            <span class="kw">match </span>mem::replace(<span class="kw-2">&amp;mut </span>compression_last_thread_result, <span class="prelude-val">Err</span>(())) {
                <span class="prelude-val">Ok</span>(result) =&gt; result,
                <span class="prelude-val">Err</span>(_err) =&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(BrotliEncoderThreadError::OtherThreadPanic),
            }
        } <span class="kw">else </span>{
            <span class="kw">match </span>mem::replace(
                <span class="kw-2">&amp;mut </span>thread.<span class="number">0</span>,
                InternalSendAlloc::SpawningOrJoining(PhantomData),
            ) {
                InternalSendAlloc::A(<span class="kw">_</span>, <span class="kw">_</span>) | InternalSendAlloc::SpawningOrJoining(<span class="kw">_</span>) =&gt; {
                    <span class="macro">panic!</span>(<span class="string">"Thread not properly spawned"</span>)
                }
                InternalSendAlloc::Join(join) =&gt; <span class="kw">match </span>join.join() {
                    <span class="prelude-val">Ok</span>(result) =&gt; result,
                    <span class="prelude-val">Err</span>(err) =&gt; {
                        <span class="kw">return </span><span class="prelude-val">Err</span>(err);
                    }
                },
            }
        };
        <span class="kw">match </span>cur_result.compressed {
            <span class="prelude-val">Ok</span>(compressed_out) =&gt; {
                bro_cat_li.new_brotli_file();
                <span class="kw">let </span><span class="kw-2">mut </span>in_offset = <span class="number">0usize</span>;
                <span class="kw">let </span>cat_result = bro_cat_li.stream(
                    <span class="kw-2">&amp;</span>compressed_out.data_backing.slice()[..compressed_out.data_size],
                    <span class="kw-2">&amp;mut </span>in_offset,
                    output,
                    <span class="kw-2">&amp;mut </span>out_file_size,
                );
                <span class="kw">match </span>cat_result {
                    BroCatliResult::Success | BroCatliResult::NeedsMoreInput =&gt; {
                        compression_result = <span class="prelude-val">Ok</span>(out_file_size);
                    }
                    BroCatliResult::NeedsMoreOutput =&gt; {
                        compression_result = <span class="prelude-val">Err</span>(BrotliEncoderThreadError::InsufficientOutputSpace);
                        <span class="comment">// not enough space
                    </span>}
                    err =&gt; {
                        compression_result = <span class="prelude-val">Err</span>(BrotliEncoderThreadError::ConcatenationError(err));
                        <span class="comment">// misc error
                    </span>}
                }
                &lt;Alloc <span class="kw">as </span>Allocator&lt;u8&gt;&gt;::free_cell(
                    <span class="kw-2">&amp;mut </span>cur_result.alloc,
                    compressed_out.data_backing,
                );
            }
            <span class="prelude-val">Err</span>(e) =&gt; {
                compression_result = <span class="prelude-val">Err</span>(e);
            }
        }
        thread.<span class="number">0 </span>= InternalSendAlloc::A(cur_result.alloc, UnionHasher::Uninit);
    }
    compression_result<span class="question-mark">?</span>;
    <span class="kw">match </span>bro_cat_li.finish(output, <span class="kw-2">&amp;mut </span>out_file_size) {
        BroCatliResult::Success =&gt; compression_result = <span class="prelude-val">Ok</span>(out_file_size),
        err =&gt; {
            compression_result = <span class="prelude-val">Err</span>(BrotliEncoderThreadError::ConcatenationFinalizationError(
                err,
            ))
        }
    }
    <span class="kw">if let </span><span class="prelude-val">Ok</span>(retrieved_owned_input) = spawner_and_input.unwrap() {
        <span class="kw-2">*</span>owned_input = Owned::new(retrieved_owned_input.<span class="number">0</span>); <span class="comment">// return the input to its rightful owner before returning
    </span>} <span class="kw">else if </span>compression_result.is_ok() {
        compression_result = <span class="prelude-val">Err</span>(BrotliEncoderThreadError::OtherThreadPanic);
    }
    compression_result
}
</code></pre></div></section></main></body></html>