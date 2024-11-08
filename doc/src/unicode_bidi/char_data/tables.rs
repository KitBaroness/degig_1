<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/unicode-bidi-0.3.15/src/char_data/tables.rs`."><title>tables.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="unicode_bidi" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../unicode_bidi/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// NOTE:
// The following code was generated by "tools/generate.py". do not edit directly

</span><span class="attr">#![allow(missing_docs, non_upper_case_globals, non_snake_case)]
#![cfg_attr(rustfmt, rustfmt_skip)]

</span><span class="doccomment">/// The [Unicode version](http://www.unicode.org/versions/) of data
</span><span class="kw">pub const </span>UNICODE_VERSION: (u64, u64, u64) = (<span class="number">15</span>, <span class="number">0</span>, <span class="number">0</span>);

<span class="attr">#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
</span><span class="doccomment">/// Represents values of the Unicode character property
/// [`Bidi_Class`](http://www.unicode.org/reports/tr44/#Bidi_Class), also
/// known as the *bidirectional character type*.
///
/// * &lt;http://www.unicode.org/reports/tr9/#Bidirectional_Character_Types&gt;
/// * &lt;http://www.unicode.org/reports/tr44/#Bidi_Class_Values&gt;
</span><span class="kw">pub enum </span>BidiClass {
    AL,
    AN,
    B,
    BN,
    CS,
    EN,
    ES,
    ET,
    FSI,
    L,
    LRE,
    LRI,
    LRO,
    NSM,
    ON,
    PDF,
    PDI,
    R,
    RLE,
    RLI,
    RLO,
    S,
    WS,
}

<span class="attr">#[cfg(feature = <span class="string">"hardcoded-data"</span>)]
</span><span class="kw">use </span><span class="self">self</span>::BidiClass::<span class="kw-2">*</span>;

<span class="attr">#[cfg(feature = <span class="string">"hardcoded-data"</span>)]
</span><span class="kw">pub const </span>bidi_class_table: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char, BidiClass)] = <span class="kw-2">&amp;</span>[
    (<span class="string">'\u{0}'</span>, <span class="string">'\u{8}'</span>, BN), (<span class="string">'\u{9}'</span>, <span class="string">'\u{9}'</span>, S), (<span class="string">'\u{a}'</span>, <span class="string">'\u{a}'</span>, B), (<span class="string">'\u{b}'</span>, <span class="string">'\u{b}'</span>, S),
    (<span class="string">'\u{c}'</span>, <span class="string">'\u{c}'</span>, WS), (<span class="string">'\u{d}'</span>, <span class="string">'\u{d}'</span>, B), (<span class="string">'\u{e}'</span>, <span class="string">'\u{1b}'</span>, BN), (<span class="string">'\u{1c}'</span>, <span class="string">'\u{1e}'</span>, B),
    (<span class="string">'\u{1f}'</span>, <span class="string">'\u{1f}'</span>, S), (<span class="string">'\u{20}'</span>, <span class="string">'\u{20}'</span>, WS), (<span class="string">'\u{21}'</span>, <span class="string">'\u{22}'</span>, ON), (<span class="string">'\u{23}'</span>,
    <span class="string">'\u{25}'</span>, ET), (<span class="string">'\u{26}'</span>, <span class="string">'\u{2a}'</span>, ON), (<span class="string">'\u{2b}'</span>, <span class="string">'\u{2b}'</span>, ES), (<span class="string">'\u{2c}'</span>, <span class="string">'\u{2c}'</span>, CS),
    (<span class="string">'\u{2d}'</span>, <span class="string">'\u{2d}'</span>, ES), (<span class="string">'\u{2e}'</span>, <span class="string">'\u{2f}'</span>, CS), (<span class="string">'\u{30}'</span>, <span class="string">'\u{39}'</span>, EN), (<span class="string">'\u{3a}'</span>,
    <span class="string">'\u{3a}'</span>, CS), (<span class="string">'\u{3b}'</span>, <span class="string">'\u{40}'</span>, ON), (<span class="string">'\u{41}'</span>, <span class="string">'\u{5a}'</span>, L), (<span class="string">'\u{5b}'</span>, <span class="string">'\u{60}'</span>, ON),
    (<span class="string">'\u{61}'</span>, <span class="string">'\u{7a}'</span>, L), (<span class="string">'\u{7b}'</span>, <span class="string">'\u{7e}'</span>, ON), (<span class="string">'\u{7f}'</span>, <span class="string">'\u{84}'</span>, BN), (<span class="string">'\u{85}'</span>,
    <span class="string">'\u{85}'</span>, B), (<span class="string">'\u{86}'</span>, <span class="string">'\u{9f}'</span>, BN), (<span class="string">'\u{a0}'</span>, <span class="string">'\u{a0}'</span>, CS), (<span class="string">'\u{a1}'</span>, <span class="string">'\u{a1}'</span>, ON),
    (<span class="string">'\u{a2}'</span>, <span class="string">'\u{a5}'</span>, ET), (<span class="string">'\u{a6}'</span>, <span class="string">'\u{a9}'</span>, ON), (<span class="string">'\u{aa}'</span>, <span class="string">'\u{aa}'</span>, L), (<span class="string">'\u{ab}'</span>,
    <span class="string">'\u{ac}'</span>, ON), (<span class="string">'\u{ad}'</span>, <span class="string">'\u{ad}'</span>, BN), (<span class="string">'\u{ae}'</span>, <span class="string">'\u{af}'</span>, ON), (<span class="string">'\u{b0}'</span>, <span class="string">'\u{b1}'</span>, ET),
    (<span class="string">'\u{b2}'</span>, <span class="string">'\u{b3}'</span>, EN), (<span class="string">'\u{b4}'</span>, <span class="string">'\u{b4}'</span>, ON), (<span class="string">'\u{b5}'</span>, <span class="string">'\u{b5}'</span>, L), (<span class="string">'\u{b6}'</span>,
    <span class="string">'\u{b8}'</span>, ON), (<span class="string">'\u{b9}'</span>, <span class="string">'\u{b9}'</span>, EN), (<span class="string">'\u{ba}'</span>, <span class="string">'\u{ba}'</span>, L), (<span class="string">'\u{bb}'</span>, <span class="string">'\u{bf}'</span>, ON),
    (<span class="string">'\u{c0}'</span>, <span class="string">'\u{d6}'</span>, L), (<span class="string">'\u{d7}'</span>, <span class="string">'\u{d7}'</span>, ON), (<span class="string">'\u{d8}'</span>, <span class="string">'\u{f6}'</span>, L), (<span class="string">'\u{f7}'</span>, <span class="string">'\u{f7}'</span>,
    ON), (<span class="string">'\u{f8}'</span>, <span class="string">'\u{2b8}'</span>, L), (<span class="string">'\u{2b9}'</span>, <span class="string">'\u{2ba}'</span>, ON), (<span class="string">'\u{2bb}'</span>, <span class="string">'\u{2c1}'</span>, L),
    (<span class="string">'\u{2c2}'</span>, <span class="string">'\u{2cf}'</span>, ON), (<span class="string">'\u{2d0}'</span>, <span class="string">'\u{2d1}'</span>, L), (<span class="string">'\u{2d2}'</span>, <span class="string">'\u{2df}'</span>, ON), (<span class="string">'\u{2e0}'</span>,
    <span class="string">'\u{2e4}'</span>, L), (<span class="string">'\u{2e5}'</span>, <span class="string">'\u{2ed}'</span>, ON), (<span class="string">'\u{2ee}'</span>, <span class="string">'\u{2ee}'</span>, L), (<span class="string">'\u{2ef}'</span>, <span class="string">'\u{2ff}'</span>,
    ON), (<span class="string">'\u{300}'</span>, <span class="string">'\u{36f}'</span>, NSM), (<span class="string">'\u{370}'</span>, <span class="string">'\u{373}'</span>, L), (<span class="string">'\u{374}'</span>, <span class="string">'\u{375}'</span>, ON),
    (<span class="string">'\u{376}'</span>, <span class="string">'\u{377}'</span>, L), (<span class="string">'\u{37a}'</span>, <span class="string">'\u{37d}'</span>, L), (<span class="string">'\u{37e}'</span>, <span class="string">'\u{37e}'</span>, ON), (<span class="string">'\u{37f}'</span>,
    <span class="string">'\u{37f}'</span>, L), (<span class="string">'\u{384}'</span>, <span class="string">'\u{385}'</span>, ON), (<span class="string">'\u{386}'</span>, <span class="string">'\u{386}'</span>, L), (<span class="string">'\u{387}'</span>, <span class="string">'\u{387}'</span>,
    ON), (<span class="string">'\u{388}'</span>, <span class="string">'\u{38a}'</span>, L), (<span class="string">'\u{38c}'</span>, <span class="string">'\u{38c}'</span>, L), (<span class="string">'\u{38e}'</span>, <span class="string">'\u{3a1}'</span>, L),
    (<span class="string">'\u{3a3}'</span>, <span class="string">'\u{3f5}'</span>, L), (<span class="string">'\u{3f6}'</span>, <span class="string">'\u{3f6}'</span>, ON), (<span class="string">'\u{3f7}'</span>, <span class="string">'\u{482}'</span>, L), (<span class="string">'\u{483}'</span>,
    <span class="string">'\u{489}'</span>, NSM), (<span class="string">'\u{48a}'</span>, <span class="string">'\u{52f}'</span>, L), (<span class="string">'\u{531}'</span>, <span class="string">'\u{556}'</span>, L), (<span class="string">'\u{559}'</span>, <span class="string">'\u{589}'</span>,
    L), (<span class="string">'\u{58a}'</span>, <span class="string">'\u{58a}'</span>, ON), (<span class="string">'\u{58d}'</span>, <span class="string">'\u{58e}'</span>, ON), (<span class="string">'\u{58f}'</span>, <span class="string">'\u{58f}'</span>, ET),
    (<span class="string">'\u{590}'</span>, <span class="string">'\u{590}'</span>, R), (<span class="string">'\u{591}'</span>, <span class="string">'\u{5bd}'</span>, NSM), (<span class="string">'\u{5be}'</span>, <span class="string">'\u{5be}'</span>, R), (<span class="string">'\u{5bf}'</span>,
    <span class="string">'\u{5bf}'</span>, NSM), (<span class="string">'\u{5c0}'</span>, <span class="string">'\u{5c0}'</span>, R), (<span class="string">'\u{5c1}'</span>, <span class="string">'\u{5c2}'</span>, NSM), (<span class="string">'\u{5c3}'</span>, <span class="string">'\u{5c3}'</span>,
    R), (<span class="string">'\u{5c4}'</span>, <span class="string">'\u{5c5}'</span>, NSM), (<span class="string">'\u{5c6}'</span>, <span class="string">'\u{5c6}'</span>, R), (<span class="string">'\u{5c7}'</span>, <span class="string">'\u{5c7}'</span>, NSM),
    (<span class="string">'\u{5c8}'</span>, <span class="string">'\u{5ff}'</span>, R), (<span class="string">'\u{600}'</span>, <span class="string">'\u{605}'</span>, AN), (<span class="string">'\u{606}'</span>, <span class="string">'\u{607}'</span>, ON), (<span class="string">'\u{608}'</span>,
    <span class="string">'\u{608}'</span>, AL), (<span class="string">'\u{609}'</span>, <span class="string">'\u{60a}'</span>, ET), (<span class="string">'\u{60b}'</span>, <span class="string">'\u{60b}'</span>, AL), (<span class="string">'\u{60c}'</span>, <span class="string">'\u{60c}'</span>,
    CS), (<span class="string">'\u{60d}'</span>, <span class="string">'\u{60d}'</span>, AL), (<span class="string">'\u{60e}'</span>, <span class="string">'\u{60f}'</span>, ON), (<span class="string">'\u{610}'</span>, <span class="string">'\u{61a}'</span>, NSM),
    (<span class="string">'\u{61b}'</span>, <span class="string">'\u{64a}'</span>, AL), (<span class="string">'\u{64b}'</span>, <span class="string">'\u{65f}'</span>, NSM), (<span class="string">'\u{660}'</span>, <span class="string">'\u{669}'</span>, AN), (<span class="string">'\u{66a}'</span>,
    <span class="string">'\u{66a}'</span>, ET), (<span class="string">'\u{66b}'</span>, <span class="string">'\u{66c}'</span>, AN), (<span class="string">'\u{66d}'</span>, <span class="string">'\u{66f}'</span>, AL), (<span class="string">'\u{670}'</span>, <span class="string">'\u{670}'</span>,
    NSM), (<span class="string">'\u{671}'</span>, <span class="string">'\u{6d5}'</span>, AL), (<span class="string">'\u{6d6}'</span>, <span class="string">'\u{6dc}'</span>, NSM), (<span class="string">'\u{6dd}'</span>, <span class="string">'\u{6dd}'</span>, AN),
    (<span class="string">'\u{6de}'</span>, <span class="string">'\u{6de}'</span>, ON), (<span class="string">'\u{6df}'</span>, <span class="string">'\u{6e4}'</span>, NSM), (<span class="string">'\u{6e5}'</span>, <span class="string">'\u{6e6}'</span>, AL), (<span class="string">'\u{6e7}'</span>,
    <span class="string">'\u{6e8}'</span>, NSM), (<span class="string">'\u{6e9}'</span>, <span class="string">'\u{6e9}'</span>, ON), (<span class="string">'\u{6ea}'</span>, <span class="string">'\u{6ed}'</span>, NSM), (<span class="string">'\u{6ee}'</span>, <span class="string">'\u{6ef}'</span>,
    AL), (<span class="string">'\u{6f0}'</span>, <span class="string">'\u{6f9}'</span>, EN), (<span class="string">'\u{6fa}'</span>, <span class="string">'\u{710}'</span>, AL), (<span class="string">'\u{711}'</span>, <span class="string">'\u{711}'</span>, NSM),
    (<span class="string">'\u{712}'</span>, <span class="string">'\u{72f}'</span>, AL), (<span class="string">'\u{730}'</span>, <span class="string">'\u{74a}'</span>, NSM), (<span class="string">'\u{74b}'</span>, <span class="string">'\u{7a5}'</span>, AL), (<span class="string">'\u{7a6}'</span>,
    <span class="string">'\u{7b0}'</span>, NSM), (<span class="string">'\u{7b1}'</span>, <span class="string">'\u{7bf}'</span>, AL), (<span class="string">'\u{7c0}'</span>, <span class="string">'\u{7ea}'</span>, R), (<span class="string">'\u{7eb}'</span>, <span class="string">'\u{7f3}'</span>,
    NSM), (<span class="string">'\u{7f4}'</span>, <span class="string">'\u{7f5}'</span>, R), (<span class="string">'\u{7f6}'</span>, <span class="string">'\u{7f9}'</span>, ON), (<span class="string">'\u{7fa}'</span>, <span class="string">'\u{7fc}'</span>, R),
    (<span class="string">'\u{7fd}'</span>, <span class="string">'\u{7fd}'</span>, NSM), (<span class="string">'\u{7fe}'</span>, <span class="string">'\u{815}'</span>, R), (<span class="string">'\u{816}'</span>, <span class="string">'\u{819}'</span>, NSM), (<span class="string">'\u{81a}'</span>,
    <span class="string">'\u{81a}'</span>, R), (<span class="string">'\u{81b}'</span>, <span class="string">'\u{823}'</span>, NSM), (<span class="string">'\u{824}'</span>, <span class="string">'\u{824}'</span>, R), (<span class="string">'\u{825}'</span>, <span class="string">'\u{827}'</span>,
    NSM), (<span class="string">'\u{828}'</span>, <span class="string">'\u{828}'</span>, R), (<span class="string">'\u{829}'</span>, <span class="string">'\u{82d}'</span>, NSM), (<span class="string">'\u{82e}'</span>, <span class="string">'\u{858}'</span>, R),
    (<span class="string">'\u{859}'</span>, <span class="string">'\u{85b}'</span>, NSM), (<span class="string">'\u{85c}'</span>, <span class="string">'\u{85f}'</span>, R), (<span class="string">'\u{860}'</span>, <span class="string">'\u{86a}'</span>, AL), (<span class="string">'\u{86b}'</span>,
    <span class="string">'\u{86f}'</span>, R), (<span class="string">'\u{870}'</span>, <span class="string">'\u{88e}'</span>, AL), (<span class="string">'\u{88f}'</span>, <span class="string">'\u{88f}'</span>, R), (<span class="string">'\u{890}'</span>, <span class="string">'\u{891}'</span>,
    AN), (<span class="string">'\u{892}'</span>, <span class="string">'\u{897}'</span>, R), (<span class="string">'\u{898}'</span>, <span class="string">'\u{89f}'</span>, NSM), (<span class="string">'\u{8a0}'</span>, <span class="string">'\u{8c9}'</span>, AL),
    (<span class="string">'\u{8ca}'</span>, <span class="string">'\u{8e1}'</span>, NSM), (<span class="string">'\u{8e2}'</span>, <span class="string">'\u{8e2}'</span>, AN), (<span class="string">'\u{8e3}'</span>, <span class="string">'\u{902}'</span>, NSM),
    (<span class="string">'\u{903}'</span>, <span class="string">'\u{939}'</span>, L), (<span class="string">'\u{93a}'</span>, <span class="string">'\u{93a}'</span>, NSM), (<span class="string">'\u{93b}'</span>, <span class="string">'\u{93b}'</span>, L), (<span class="string">'\u{93c}'</span>,
    <span class="string">'\u{93c}'</span>, NSM), (<span class="string">'\u{93d}'</span>, <span class="string">'\u{940}'</span>, L), (<span class="string">'\u{941}'</span>, <span class="string">'\u{948}'</span>, NSM), (<span class="string">'\u{949}'</span>, <span class="string">'\u{94c}'</span>,
    L), (<span class="string">'\u{94d}'</span>, <span class="string">'\u{94d}'</span>, NSM), (<span class="string">'\u{94e}'</span>, <span class="string">'\u{950}'</span>, L), (<span class="string">'\u{951}'</span>, <span class="string">'\u{957}'</span>, NSM),
    (<span class="string">'\u{958}'</span>, <span class="string">'\u{961}'</span>, L), (<span class="string">'\u{962}'</span>, <span class="string">'\u{963}'</span>, NSM), (<span class="string">'\u{964}'</span>, <span class="string">'\u{980}'</span>, L), (<span class="string">'\u{981}'</span>,
    <span class="string">'\u{981}'</span>, NSM), (<span class="string">'\u{982}'</span>, <span class="string">'\u{983}'</span>, L), (<span class="string">'\u{985}'</span>, <span class="string">'\u{98c}'</span>, L), (<span class="string">'\u{98f}'</span>, <span class="string">'\u{990}'</span>,
    L), (<span class="string">'\u{993}'</span>, <span class="string">'\u{9a8}'</span>, L), (<span class="string">'\u{9aa}'</span>, <span class="string">'\u{9b0}'</span>, L), (<span class="string">'\u{9b2}'</span>, <span class="string">'\u{9b2}'</span>, L), (<span class="string">'\u{9b6}'</span>,
    <span class="string">'\u{9b9}'</span>, L), (<span class="string">'\u{9bc}'</span>, <span class="string">'\u{9bc}'</span>, NSM), (<span class="string">'\u{9bd}'</span>, <span class="string">'\u{9c0}'</span>, L), (<span class="string">'\u{9c1}'</span>, <span class="string">'\u{9c4}'</span>,
    NSM), (<span class="string">'\u{9c7}'</span>, <span class="string">'\u{9c8}'</span>, L), (<span class="string">'\u{9cb}'</span>, <span class="string">'\u{9cc}'</span>, L), (<span class="string">'\u{9cd}'</span>, <span class="string">'\u{9cd}'</span>, NSM),
    (<span class="string">'\u{9ce}'</span>, <span class="string">'\u{9ce}'</span>, L), (<span class="string">'\u{9d7}'</span>, <span class="string">'\u{9d7}'</span>, L), (<span class="string">'\u{9dc}'</span>, <span class="string">'\u{9dd}'</span>, L), (<span class="string">'\u{9df}'</span>,
    <span class="string">'\u{9e1}'</span>, L), (<span class="string">'\u{9e2}'</span>, <span class="string">'\u{9e3}'</span>, NSM), (<span class="string">'\u{9e6}'</span>, <span class="string">'\u{9f1}'</span>, L), (<span class="string">'\u{9f2}'</span>, <span class="string">'\u{9f3}'</span>,
    ET), (<span class="string">'\u{9f4}'</span>, <span class="string">'\u{9fa}'</span>, L), (<span class="string">'\u{9fb}'</span>, <span class="string">'\u{9fb}'</span>, ET), (<span class="string">'\u{9fc}'</span>, <span class="string">'\u{9fd}'</span>, L),
    (<span class="string">'\u{9fe}'</span>, <span class="string">'\u{9fe}'</span>, NSM), (<span class="string">'\u{a01}'</span>, <span class="string">'\u{a02}'</span>, NSM), (<span class="string">'\u{a03}'</span>, <span class="string">'\u{a03}'</span>, L), (<span class="string">'\u{a05}'</span>,
    <span class="string">'\u{a0a}'</span>, L), (<span class="string">'\u{a0f}'</span>, <span class="string">'\u{a10}'</span>, L), (<span class="string">'\u{a13}'</span>, <span class="string">'\u{a28}'</span>, L), (<span class="string">'\u{a2a}'</span>, <span class="string">'\u{a30}'</span>, L),
    (<span class="string">'\u{a32}'</span>, <span class="string">'\u{a33}'</span>, L), (<span class="string">'\u{a35}'</span>, <span class="string">'\u{a36}'</span>, L), (<span class="string">'\u{a38}'</span>, <span class="string">'\u{a39}'</span>, L), (<span class="string">'\u{a3c}'</span>,
    <span class="string">'\u{a3c}'</span>, NSM), (<span class="string">'\u{a3e}'</span>, <span class="string">'\u{a40}'</span>, L), (<span class="string">'\u{a41}'</span>, <span class="string">'\u{a42}'</span>, NSM), (<span class="string">'\u{a47}'</span>, <span class="string">'\u{a48}'</span>,
    NSM), (<span class="string">'\u{a4b}'</span>, <span class="string">'\u{a4d}'</span>, NSM), (<span class="string">'\u{a51}'</span>, <span class="string">'\u{a51}'</span>, NSM), (<span class="string">'\u{a59}'</span>, <span class="string">'\u{a5c}'</span>, L),
    (<span class="string">'\u{a5e}'</span>, <span class="string">'\u{a5e}'</span>, L), (<span class="string">'\u{a66}'</span>, <span class="string">'\u{a6f}'</span>, L), (<span class="string">'\u{a70}'</span>, <span class="string">'\u{a71}'</span>, NSM), (<span class="string">'\u{a72}'</span>,
    <span class="string">'\u{a74}'</span>, L), (<span class="string">'\u{a75}'</span>, <span class="string">'\u{a75}'</span>, NSM), (<span class="string">'\u{a76}'</span>, <span class="string">'\u{a76}'</span>, L), (<span class="string">'\u{a81}'</span>, <span class="string">'\u{a82}'</span>,
    NSM), (<span class="string">'\u{a83}'</span>, <span class="string">'\u{a83}'</span>, L), (<span class="string">'\u{a85}'</span>, <span class="string">'\u{a8d}'</span>, L), (<span class="string">'\u{a8f}'</span>, <span class="string">'\u{a91}'</span>, L),
    (<span class="string">'\u{a93}'</span>, <span class="string">'\u{aa8}'</span>, L), (<span class="string">'\u{aaa}'</span>, <span class="string">'\u{ab0}'</span>, L), (<span class="string">'\u{ab2}'</span>, <span class="string">'\u{ab3}'</span>, L), (<span class="string">'\u{ab5}'</span>,
    <span class="string">'\u{ab9}'</span>, L), (<span class="string">'\u{abc}'</span>, <span class="string">'\u{abc}'</span>, NSM), (<span class="string">'\u{abd}'</span>, <span class="string">'\u{ac0}'</span>, L), (<span class="string">'\u{ac1}'</span>, <span class="string">'\u{ac5}'</span>,
    NSM), (<span class="string">'\u{ac7}'</span>, <span class="string">'\u{ac8}'</span>, NSM), (<span class="string">'\u{ac9}'</span>, <span class="string">'\u{ac9}'</span>, L), (<span class="string">'\u{acb}'</span>, <span class="string">'\u{acc}'</span>, L),
    (<span class="string">'\u{acd}'</span>, <span class="string">'\u{acd}'</span>, NSM), (<span class="string">'\u{ad0}'</span>, <span class="string">'\u{ad0}'</span>, L), (<span class="string">'\u{ae0}'</span>, <span class="string">'\u{ae1}'</span>, L), (<span class="string">'\u{ae2}'</span>,
    <span class="string">'\u{ae3}'</span>, NSM), (<span class="string">'\u{ae6}'</span>, <span class="string">'\u{af0}'</span>, L), (<span class="string">'\u{af1}'</span>, <span class="string">'\u{af1}'</span>, ET), (<span class="string">'\u{af9}'</span>, <span class="string">'\u{af9}'</span>,
    L), (<span class="string">'\u{afa}'</span>, <span class="string">'\u{aff}'</span>, NSM), (<span class="string">'\u{b01}'</span>, <span class="string">'\u{b01}'</span>, NSM), (<span class="string">'\u{b02}'</span>, <span class="string">'\u{b03}'</span>, L),
    (<span class="string">'\u{b05}'</span>, <span class="string">'\u{b0c}'</span>, L), (<span class="string">'\u{b0f}'</span>, <span class="string">'\u{b10}'</span>, L), (<span class="string">'\u{b13}'</span>, <span class="string">'\u{b28}'</span>, L), (<span class="string">'\u{b2a}'</span>,
    <span class="string">'\u{b30}'</span>, L), (<span class="string">'\u{b32}'</span>, <span class="string">'\u{b33}'</span>, L), (<span class="string">'\u{b35}'</span>, <span class="string">'\u{b39}'</span>, L), (<span class="string">'\u{b3c}'</span>, <span class="string">'\u{b3c}'</span>,
    NSM), (<span class="string">'\u{b3d}'</span>, <span class="string">'\u{b3e}'</span>, L), (<span class="string">'\u{b3f}'</span>, <span class="string">'\u{b3f}'</span>, NSM), (<span class="string">'\u{b40}'</span>, <span class="string">'\u{b40}'</span>, L),
    (<span class="string">'\u{b41}'</span>, <span class="string">'\u{b44}'</span>, NSM), (<span class="string">'\u{b47}'</span>, <span class="string">'\u{b48}'</span>, L), (<span class="string">'\u{b4b}'</span>, <span class="string">'\u{b4c}'</span>, L), (<span class="string">'\u{b4d}'</span>,
    <span class="string">'\u{b4d}'</span>, NSM), (<span class="string">'\u{b55}'</span>, <span class="string">'\u{b56}'</span>, NSM), (<span class="string">'\u{b57}'</span>, <span class="string">'\u{b57}'</span>, L), (<span class="string">'\u{b5c}'</span>, <span class="string">'\u{b5d}'</span>,
    L), (<span class="string">'\u{b5f}'</span>, <span class="string">'\u{b61}'</span>, L), (<span class="string">'\u{b62}'</span>, <span class="string">'\u{b63}'</span>, NSM), (<span class="string">'\u{b66}'</span>, <span class="string">'\u{b77}'</span>, L),
    (<span class="string">'\u{b82}'</span>, <span class="string">'\u{b82}'</span>, NSM), (<span class="string">'\u{b83}'</span>, <span class="string">'\u{b83}'</span>, L), (<span class="string">'\u{b85}'</span>, <span class="string">'\u{b8a}'</span>, L), (<span class="string">'\u{b8e}'</span>,
    <span class="string">'\u{b90}'</span>, L), (<span class="string">'\u{b92}'</span>, <span class="string">'\u{b95}'</span>, L), (<span class="string">'\u{b99}'</span>, <span class="string">'\u{b9a}'</span>, L), (<span class="string">'\u{b9c}'</span>, <span class="string">'\u{b9c}'</span>, L),
    (<span class="string">'\u{b9e}'</span>, <span class="string">'\u{b9f}'</span>, L), (<span class="string">'\u{ba3}'</span>, <span class="string">'\u{ba4}'</span>, L), (<span class="string">'\u{ba8}'</span>, <span class="string">'\u{baa}'</span>, L), (<span class="string">'\u{bae}'</span>,
    <span class="string">'\u{bb9}'</span>, L), (<span class="string">'\u{bbe}'</span>, <span class="string">'\u{bbf}'</span>, L), (<span class="string">'\u{bc0}'</span>, <span class="string">'\u{bc0}'</span>, NSM), (<span class="string">'\u{bc1}'</span>, <span class="string">'\u{bc2}'</span>,
    L), (<span class="string">'\u{bc6}'</span>, <span class="string">'\u{bc8}'</span>, L), (<span class="string">'\u{bca}'</span>, <span class="string">'\u{bcc}'</span>, L), (<span class="string">'\u{bcd}'</span>, <span class="string">'\u{bcd}'</span>, NSM),
    (<span class="string">'\u{bd0}'</span>, <span class="string">'\u{bd0}'</span>, L), (<span class="string">'\u{bd7}'</span>, <span class="string">'\u{bd7}'</span>, L), (<span class="string">'\u{be6}'</span>, <span class="string">'\u{bf2}'</span>, L), (<span class="string">'\u{bf3}'</span>,
    <span class="string">'\u{bf8}'</span>, ON), (<span class="string">'\u{bf9}'</span>, <span class="string">'\u{bf9}'</span>, ET), (<span class="string">'\u{bfa}'</span>, <span class="string">'\u{bfa}'</span>, ON), (<span class="string">'\u{c00}'</span>, <span class="string">'\u{c00}'</span>,
    NSM), (<span class="string">'\u{c01}'</span>, <span class="string">'\u{c03}'</span>, L), (<span class="string">'\u{c04}'</span>, <span class="string">'\u{c04}'</span>, NSM), (<span class="string">'\u{c05}'</span>, <span class="string">'\u{c0c}'</span>, L),
    (<span class="string">'\u{c0e}'</span>, <span class="string">'\u{c10}'</span>, L), (<span class="string">'\u{c12}'</span>, <span class="string">'\u{c28}'</span>, L), (<span class="string">'\u{c2a}'</span>, <span class="string">'\u{c39}'</span>, L), (<span class="string">'\u{c3c}'</span>,
    <span class="string">'\u{c3c}'</span>, NSM), (<span class="string">'\u{c3d}'</span>, <span class="string">'\u{c3d}'</span>, L), (<span class="string">'\u{c3e}'</span>, <span class="string">'\u{c40}'</span>, NSM), (<span class="string">'\u{c41}'</span>, <span class="string">'\u{c44}'</span>,
    L), (<span class="string">'\u{c46}'</span>, <span class="string">'\u{c48}'</span>, NSM), (<span class="string">'\u{c4a}'</span>, <span class="string">'\u{c4d}'</span>, NSM), (<span class="string">'\u{c55}'</span>, <span class="string">'\u{c56}'</span>, NSM),
    (<span class="string">'\u{c58}'</span>, <span class="string">'\u{c5a}'</span>, L), (<span class="string">'\u{c5d}'</span>, <span class="string">'\u{c5d}'</span>, L), (<span class="string">'\u{c60}'</span>, <span class="string">'\u{c61}'</span>, L), (<span class="string">'\u{c62}'</span>,
    <span class="string">'\u{c63}'</span>, NSM), (<span class="string">'\u{c66}'</span>, <span class="string">'\u{c6f}'</span>, L), (<span class="string">'\u{c77}'</span>, <span class="string">'\u{c77}'</span>, L), (<span class="string">'\u{c78}'</span>, <span class="string">'\u{c7e}'</span>,
    ON), (<span class="string">'\u{c7f}'</span>, <span class="string">'\u{c80}'</span>, L), (<span class="string">'\u{c81}'</span>, <span class="string">'\u{c81}'</span>, NSM), (<span class="string">'\u{c82}'</span>, <span class="string">'\u{c8c}'</span>, L),
    (<span class="string">'\u{c8e}'</span>, <span class="string">'\u{c90}'</span>, L), (<span class="string">'\u{c92}'</span>, <span class="string">'\u{ca8}'</span>, L), (<span class="string">'\u{caa}'</span>, <span class="string">'\u{cb3}'</span>, L), (<span class="string">'\u{cb5}'</span>,
    <span class="string">'\u{cb9}'</span>, L), (<span class="string">'\u{cbc}'</span>, <span class="string">'\u{cbc}'</span>, NSM), (<span class="string">'\u{cbd}'</span>, <span class="string">'\u{cc4}'</span>, L), (<span class="string">'\u{cc6}'</span>, <span class="string">'\u{cc8}'</span>,
    L), (<span class="string">'\u{cca}'</span>, <span class="string">'\u{ccb}'</span>, L), (<span class="string">'\u{ccc}'</span>, <span class="string">'\u{ccd}'</span>, NSM), (<span class="string">'\u{cd5}'</span>, <span class="string">'\u{cd6}'</span>, L),
    (<span class="string">'\u{cdd}'</span>, <span class="string">'\u{cde}'</span>, L), (<span class="string">'\u{ce0}'</span>, <span class="string">'\u{ce1}'</span>, L), (<span class="string">'\u{ce2}'</span>, <span class="string">'\u{ce3}'</span>, NSM), (<span class="string">'\u{ce6}'</span>,
    <span class="string">'\u{cef}'</span>, L), (<span class="string">'\u{cf1}'</span>, <span class="string">'\u{cf3}'</span>, L), (<span class="string">'\u{d00}'</span>, <span class="string">'\u{d01}'</span>, NSM), (<span class="string">'\u{d02}'</span>, <span class="string">'\u{d0c}'</span>,
    L), (<span class="string">'\u{d0e}'</span>, <span class="string">'\u{d10}'</span>, L), (<span class="string">'\u{d12}'</span>, <span class="string">'\u{d3a}'</span>, L), (<span class="string">'\u{d3b}'</span>, <span class="string">'\u{d3c}'</span>, NSM),
    (<span class="string">'\u{d3d}'</span>, <span class="string">'\u{d40}'</span>, L), (<span class="string">'\u{d41}'</span>, <span class="string">'\u{d44}'</span>, NSM), (<span class="string">'\u{d46}'</span>, <span class="string">'\u{d48}'</span>, L), (<span class="string">'\u{d4a}'</span>,
    <span class="string">'\u{d4c}'</span>, L), (<span class="string">'\u{d4d}'</span>, <span class="string">'\u{d4d}'</span>, NSM), (<span class="string">'\u{d4e}'</span>, <span class="string">'\u{d4f}'</span>, L), (<span class="string">'\u{d54}'</span>, <span class="string">'\u{d61}'</span>,
    L), (<span class="string">'\u{d62}'</span>, <span class="string">'\u{d63}'</span>, NSM), (<span class="string">'\u{d66}'</span>, <span class="string">'\u{d7f}'</span>, L), (<span class="string">'\u{d81}'</span>, <span class="string">'\u{d81}'</span>, NSM),
    (<span class="string">'\u{d82}'</span>, <span class="string">'\u{d83}'</span>, L), (<span class="string">'\u{d85}'</span>, <span class="string">'\u{d96}'</span>, L), (<span class="string">'\u{d9a}'</span>, <span class="string">'\u{db1}'</span>, L), (<span class="string">'\u{db3}'</span>,
    <span class="string">'\u{dbb}'</span>, L), (<span class="string">'\u{dbd}'</span>, <span class="string">'\u{dbd}'</span>, L), (<span class="string">'\u{dc0}'</span>, <span class="string">'\u{dc6}'</span>, L), (<span class="string">'\u{dca}'</span>, <span class="string">'\u{dca}'</span>,
    NSM), (<span class="string">'\u{dcf}'</span>, <span class="string">'\u{dd1}'</span>, L), (<span class="string">'\u{dd2}'</span>, <span class="string">'\u{dd4}'</span>, NSM), (<span class="string">'\u{dd6}'</span>, <span class="string">'\u{dd6}'</span>, NSM),
    (<span class="string">'\u{dd8}'</span>, <span class="string">'\u{ddf}'</span>, L), (<span class="string">'\u{de6}'</span>, <span class="string">'\u{def}'</span>, L), (<span class="string">'\u{df2}'</span>, <span class="string">'\u{df4}'</span>, L), (<span class="string">'\u{e01}'</span>,
    <span class="string">'\u{e30}'</span>, L), (<span class="string">'\u{e31}'</span>, <span class="string">'\u{e31}'</span>, NSM), (<span class="string">'\u{e32}'</span>, <span class="string">'\u{e33}'</span>, L), (<span class="string">'\u{e34}'</span>, <span class="string">'\u{e3a}'</span>,
    NSM), (<span class="string">'\u{e3f}'</span>, <span class="string">'\u{e3f}'</span>, ET), (<span class="string">'\u{e40}'</span>, <span class="string">'\u{e46}'</span>, L), (<span class="string">'\u{e47}'</span>, <span class="string">'\u{e4e}'</span>, NSM),
    (<span class="string">'\u{e4f}'</span>, <span class="string">'\u{e5b}'</span>, L), (<span class="string">'\u{e81}'</span>, <span class="string">'\u{e82}'</span>, L), (<span class="string">'\u{e84}'</span>, <span class="string">'\u{e84}'</span>, L), (<span class="string">'\u{e86}'</span>,
    <span class="string">'\u{e8a}'</span>, L), (<span class="string">'\u{e8c}'</span>, <span class="string">'\u{ea3}'</span>, L), (<span class="string">'\u{ea5}'</span>, <span class="string">'\u{ea5}'</span>, L), (<span class="string">'\u{ea7}'</span>, <span class="string">'\u{eb0}'</span>, L),
    (<span class="string">'\u{eb1}'</span>, <span class="string">'\u{eb1}'</span>, NSM), (<span class="string">'\u{eb2}'</span>, <span class="string">'\u{eb3}'</span>, L), (<span class="string">'\u{eb4}'</span>, <span class="string">'\u{ebc}'</span>, NSM), (<span class="string">'\u{ebd}'</span>,
    <span class="string">'\u{ebd}'</span>, L), (<span class="string">'\u{ec0}'</span>, <span class="string">'\u{ec4}'</span>, L), (<span class="string">'\u{ec6}'</span>, <span class="string">'\u{ec6}'</span>, L), (<span class="string">'\u{ec8}'</span>, <span class="string">'\u{ece}'</span>,
    NSM), (<span class="string">'\u{ed0}'</span>, <span class="string">'\u{ed9}'</span>, L), (<span class="string">'\u{edc}'</span>, <span class="string">'\u{edf}'</span>, L), (<span class="string">'\u{f00}'</span>, <span class="string">'\u{f17}'</span>, L),
    (<span class="string">'\u{f18}'</span>, <span class="string">'\u{f19}'</span>, NSM), (<span class="string">'\u{f1a}'</span>, <span class="string">'\u{f34}'</span>, L), (<span class="string">'\u{f35}'</span>, <span class="string">'\u{f35}'</span>, NSM), (<span class="string">'\u{f36}'</span>,
    <span class="string">'\u{f36}'</span>, L), (<span class="string">'\u{f37}'</span>, <span class="string">'\u{f37}'</span>, NSM), (<span class="string">'\u{f38}'</span>, <span class="string">'\u{f38}'</span>, L), (<span class="string">'\u{f39}'</span>, <span class="string">'\u{f39}'</span>,
    NSM), (<span class="string">'\u{f3a}'</span>, <span class="string">'\u{f3d}'</span>, ON), (<span class="string">'\u{f3e}'</span>, <span class="string">'\u{f47}'</span>, L), (<span class="string">'\u{f49}'</span>, <span class="string">'\u{f6c}'</span>, L),
    (<span class="string">'\u{f71}'</span>, <span class="string">'\u{f7e}'</span>, NSM), (<span class="string">'\u{f7f}'</span>, <span class="string">'\u{f7f}'</span>, L), (<span class="string">'\u{f80}'</span>, <span class="string">'\u{f84}'</span>, NSM), (<span class="string">'\u{f85}'</span>,
    <span class="string">'\u{f85}'</span>, L), (<span class="string">'\u{f86}'</span>, <span class="string">'\u{f87}'</span>, NSM), (<span class="string">'\u{f88}'</span>, <span class="string">'\u{f8c}'</span>, L), (<span class="string">'\u{f8d}'</span>, <span class="string">'\u{f97}'</span>,
    NSM), (<span class="string">'\u{f99}'</span>, <span class="string">'\u{fbc}'</span>, NSM), (<span class="string">'\u{fbe}'</span>, <span class="string">'\u{fc5}'</span>, L), (<span class="string">'\u{fc6}'</span>, <span class="string">'\u{fc6}'</span>, NSM),
    (<span class="string">'\u{fc7}'</span>, <span class="string">'\u{fcc}'</span>, L), (<span class="string">'\u{fce}'</span>, <span class="string">'\u{fda}'</span>, L), (<span class="string">'\u{1000}'</span>, <span class="string">'\u{102c}'</span>, L), (<span class="string">'\u{102d}'</span>,
    <span class="string">'\u{1030}'</span>, NSM), (<span class="string">'\u{1031}'</span>, <span class="string">'\u{1031}'</span>, L), (<span class="string">'\u{1032}'</span>, <span class="string">'\u{1037}'</span>, NSM), (<span class="string">'\u{1038}'</span>,
    <span class="string">'\u{1038}'</span>, L), (<span class="string">'\u{1039}'</span>, <span class="string">'\u{103a}'</span>, NSM), (<span class="string">'\u{103b}'</span>, <span class="string">'\u{103c}'</span>, L), (<span class="string">'\u{103d}'</span>,
    <span class="string">'\u{103e}'</span>, NSM), (<span class="string">'\u{103f}'</span>, <span class="string">'\u{1057}'</span>, L), (<span class="string">'\u{1058}'</span>, <span class="string">'\u{1059}'</span>, NSM), (<span class="string">'\u{105a}'</span>,
    <span class="string">'\u{105d}'</span>, L), (<span class="string">'\u{105e}'</span>, <span class="string">'\u{1060}'</span>, NSM), (<span class="string">'\u{1061}'</span>, <span class="string">'\u{1070}'</span>, L), (<span class="string">'\u{1071}'</span>,
    <span class="string">'\u{1074}'</span>, NSM), (<span class="string">'\u{1075}'</span>, <span class="string">'\u{1081}'</span>, L), (<span class="string">'\u{1082}'</span>, <span class="string">'\u{1082}'</span>, NSM), (<span class="string">'\u{1083}'</span>,
    <span class="string">'\u{1084}'</span>, L), (<span class="string">'\u{1085}'</span>, <span class="string">'\u{1086}'</span>, NSM), (<span class="string">'\u{1087}'</span>, <span class="string">'\u{108c}'</span>, L), (<span class="string">'\u{108d}'</span>,
    <span class="string">'\u{108d}'</span>, NSM), (<span class="string">'\u{108e}'</span>, <span class="string">'\u{109c}'</span>, L), (<span class="string">'\u{109d}'</span>, <span class="string">'\u{109d}'</span>, NSM), (<span class="string">'\u{109e}'</span>,
    <span class="string">'\u{10c5}'</span>, L), (<span class="string">'\u{10c7}'</span>, <span class="string">'\u{10c7}'</span>, L), (<span class="string">'\u{10cd}'</span>, <span class="string">'\u{10cd}'</span>, L), (<span class="string">'\u{10d0}'</span>,
    <span class="string">'\u{1248}'</span>, L), (<span class="string">'\u{124a}'</span>, <span class="string">'\u{124d}'</span>, L), (<span class="string">'\u{1250}'</span>, <span class="string">'\u{1256}'</span>, L), (<span class="string">'\u{1258}'</span>,
    <span class="string">'\u{1258}'</span>, L), (<span class="string">'\u{125a}'</span>, <span class="string">'\u{125d}'</span>, L), (<span class="string">'\u{1260}'</span>, <span class="string">'\u{1288}'</span>, L), (<span class="string">'\u{128a}'</span>,
    <span class="string">'\u{128d}'</span>, L), (<span class="string">'\u{1290}'</span>, <span class="string">'\u{12b0}'</span>, L), (<span class="string">'\u{12b2}'</span>, <span class="string">'\u{12b5}'</span>, L), (<span class="string">'\u{12b8}'</span>,
    <span class="string">'\u{12be}'</span>, L), (<span class="string">'\u{12c0}'</span>, <span class="string">'\u{12c0}'</span>, L), (<span class="string">'\u{12c2}'</span>, <span class="string">'\u{12c5}'</span>, L), (<span class="string">'\u{12c8}'</span>,
    <span class="string">'\u{12d6}'</span>, L), (<span class="string">'\u{12d8}'</span>, <span class="string">'\u{1310}'</span>, L), (<span class="string">'\u{1312}'</span>, <span class="string">'\u{1315}'</span>, L), (<span class="string">'\u{1318}'</span>,
    <span class="string">'\u{135a}'</span>, L), (<span class="string">'\u{135d}'</span>, <span class="string">'\u{135f}'</span>, NSM), (<span class="string">'\u{1360}'</span>, <span class="string">'\u{137c}'</span>, L), (<span class="string">'\u{1380}'</span>,
    <span class="string">'\u{138f}'</span>, L), (<span class="string">'\u{1390}'</span>, <span class="string">'\u{1399}'</span>, ON), (<span class="string">'\u{13a0}'</span>, <span class="string">'\u{13f5}'</span>, L), (<span class="string">'\u{13f8}'</span>,
    <span class="string">'\u{13fd}'</span>, L), (<span class="string">'\u{1400}'</span>, <span class="string">'\u{1400}'</span>, ON), (<span class="string">'\u{1401}'</span>, <span class="string">'\u{167f}'</span>, L), (<span class="string">'\u{1680}'</span>,
    <span class="string">'\u{1680}'</span>, WS), (<span class="string">'\u{1681}'</span>, <span class="string">'\u{169a}'</span>, L), (<span class="string">'\u{169b}'</span>, <span class="string">'\u{169c}'</span>, ON), (<span class="string">'\u{16a0}'</span>,
    <span class="string">'\u{16f8}'</span>, L), (<span class="string">'\u{1700}'</span>, <span class="string">'\u{1711}'</span>, L), (<span class="string">'\u{1712}'</span>, <span class="string">'\u{1714}'</span>, NSM), (<span class="string">'\u{1715}'</span>,
    <span class="string">'\u{1715}'</span>, L), (<span class="string">'\u{171f}'</span>, <span class="string">'\u{1731}'</span>, L), (<span class="string">'\u{1732}'</span>, <span class="string">'\u{1733}'</span>, NSM), (<span class="string">'\u{1734}'</span>,
    <span class="string">'\u{1736}'</span>, L), (<span class="string">'\u{1740}'</span>, <span class="string">'\u{1751}'</span>, L), (<span class="string">'\u{1752}'</span>, <span class="string">'\u{1753}'</span>, NSM), (<span class="string">'\u{1760}'</span>,
    <span class="string">'\u{176c}'</span>, L), (<span class="string">'\u{176e}'</span>, <span class="string">'\u{1770}'</span>, L), (<span class="string">'\u{1772}'</span>, <span class="string">'\u{1773}'</span>, NSM), (<span class="string">'\u{1780}'</span>,
    <span class="string">'\u{17b3}'</span>, L), (<span class="string">'\u{17b4}'</span>, <span class="string">'\u{17b5}'</span>, NSM), (<span class="string">'\u{17b6}'</span>, <span class="string">'\u{17b6}'</span>, L), (<span class="string">'\u{17b7}'</span>,
    <span class="string">'\u{17bd}'</span>, NSM), (<span class="string">'\u{17be}'</span>, <span class="string">'\u{17c5}'</span>, L), (<span class="string">'\u{17c6}'</span>, <span class="string">'\u{17c6}'</span>, NSM), (<span class="string">'\u{17c7}'</span>,
    <span class="string">'\u{17c8}'</span>, L), (<span class="string">'\u{17c9}'</span>, <span class="string">'\u{17d3}'</span>, NSM), (<span class="string">'\u{17d4}'</span>, <span class="string">'\u{17da}'</span>, L), (<span class="string">'\u{17db}'</span>,
    <span class="string">'\u{17db}'</span>, ET), (<span class="string">'\u{17dc}'</span>, <span class="string">'\u{17dc}'</span>, L), (<span class="string">'\u{17dd}'</span>, <span class="string">'\u{17dd}'</span>, NSM), (<span class="string">'\u{17e0}'</span>,
    <span class="string">'\u{17e9}'</span>, L), (<span class="string">'\u{17f0}'</span>, <span class="string">'\u{17f9}'</span>, ON), (<span class="string">'\u{1800}'</span>, <span class="string">'\u{180a}'</span>, ON), (<span class="string">'\u{180b}'</span>,
    <span class="string">'\u{180d}'</span>, NSM), (<span class="string">'\u{180e}'</span>, <span class="string">'\u{180e}'</span>, BN), (<span class="string">'\u{180f}'</span>, <span class="string">'\u{180f}'</span>, NSM), (<span class="string">'\u{1810}'</span>,
    <span class="string">'\u{1819}'</span>, L), (<span class="string">'\u{1820}'</span>, <span class="string">'\u{1878}'</span>, L), (<span class="string">'\u{1880}'</span>, <span class="string">'\u{1884}'</span>, L), (<span class="string">'\u{1885}'</span>,
    <span class="string">'\u{1886}'</span>, NSM), (<span class="string">'\u{1887}'</span>, <span class="string">'\u{18a8}'</span>, L), (<span class="string">'\u{18a9}'</span>, <span class="string">'\u{18a9}'</span>, NSM), (<span class="string">'\u{18aa}'</span>,
    <span class="string">'\u{18aa}'</span>, L), (<span class="string">'\u{18b0}'</span>, <span class="string">'\u{18f5}'</span>, L), (<span class="string">'\u{1900}'</span>, <span class="string">'\u{191e}'</span>, L), (<span class="string">'\u{1920}'</span>,
    <span class="string">'\u{1922}'</span>, NSM), (<span class="string">'\u{1923}'</span>, <span class="string">'\u{1926}'</span>, L), (<span class="string">'\u{1927}'</span>, <span class="string">'\u{1928}'</span>, NSM), (<span class="string">'\u{1929}'</span>,
    <span class="string">'\u{192b}'</span>, L), (<span class="string">'\u{1930}'</span>, <span class="string">'\u{1931}'</span>, L), (<span class="string">'\u{1932}'</span>, <span class="string">'\u{1932}'</span>, NSM), (<span class="string">'\u{1933}'</span>,
    <span class="string">'\u{1938}'</span>, L), (<span class="string">'\u{1939}'</span>, <span class="string">'\u{193b}'</span>, NSM), (<span class="string">'\u{1940}'</span>, <span class="string">'\u{1940}'</span>, ON), (<span class="string">'\u{1944}'</span>,
    <span class="string">'\u{1945}'</span>, ON), (<span class="string">'\u{1946}'</span>, <span class="string">'\u{196d}'</span>, L), (<span class="string">'\u{1970}'</span>, <span class="string">'\u{1974}'</span>, L), (<span class="string">'\u{1980}'</span>,
    <span class="string">'\u{19ab}'</span>, L), (<span class="string">'\u{19b0}'</span>, <span class="string">'\u{19c9}'</span>, L), (<span class="string">'\u{19d0}'</span>, <span class="string">'\u{19da}'</span>, L), (<span class="string">'\u{19de}'</span>,
    <span class="string">'\u{19ff}'</span>, ON), (<span class="string">'\u{1a00}'</span>, <span class="string">'\u{1a16}'</span>, L), (<span class="string">'\u{1a17}'</span>, <span class="string">'\u{1a18}'</span>, NSM), (<span class="string">'\u{1a19}'</span>,
    <span class="string">'\u{1a1a}'</span>, L), (<span class="string">'\u{1a1b}'</span>, <span class="string">'\u{1a1b}'</span>, NSM), (<span class="string">'\u{1a1e}'</span>, <span class="string">'\u{1a55}'</span>, L), (<span class="string">'\u{1a56}'</span>,
    <span class="string">'\u{1a56}'</span>, NSM), (<span class="string">'\u{1a57}'</span>, <span class="string">'\u{1a57}'</span>, L), (<span class="string">'\u{1a58}'</span>, <span class="string">'\u{1a5e}'</span>, NSM), (<span class="string">'\u{1a60}'</span>,
    <span class="string">'\u{1a60}'</span>, NSM), (<span class="string">'\u{1a61}'</span>, <span class="string">'\u{1a61}'</span>, L), (<span class="string">'\u{1a62}'</span>, <span class="string">'\u{1a62}'</span>, NSM), (<span class="string">'\u{1a63}'</span>,
    <span class="string">'\u{1a64}'</span>, L), (<span class="string">'\u{1a65}'</span>, <span class="string">'\u{1a6c}'</span>, NSM), (<span class="string">'\u{1a6d}'</span>, <span class="string">'\u{1a72}'</span>, L), (<span class="string">'\u{1a73}'</span>,
    <span class="string">'\u{1a7c}'</span>, NSM), (<span class="string">'\u{1a7f}'</span>, <span class="string">'\u{1a7f}'</span>, NSM), (<span class="string">'\u{1a80}'</span>, <span class="string">'\u{1a89}'</span>, L), (<span class="string">'\u{1a90}'</span>,
    <span class="string">'\u{1a99}'</span>, L), (<span class="string">'\u{1aa0}'</span>, <span class="string">'\u{1aad}'</span>, L), (<span class="string">'\u{1ab0}'</span>, <span class="string">'\u{1ace}'</span>, NSM), (<span class="string">'\u{1b00}'</span>,
    <span class="string">'\u{1b03}'</span>, NSM), (<span class="string">'\u{1b04}'</span>, <span class="string">'\u{1b33}'</span>, L), (<span class="string">'\u{1b34}'</span>, <span class="string">'\u{1b34}'</span>, NSM), (<span class="string">'\u{1b35}'</span>,
    <span class="string">'\u{1b35}'</span>, L), (<span class="string">'\u{1b36}'</span>, <span class="string">'\u{1b3a}'</span>, NSM), (<span class="string">'\u{1b3b}'</span>, <span class="string">'\u{1b3b}'</span>, L), (<span class="string">'\u{1b3c}'</span>,
    <span class="string">'\u{1b3c}'</span>, NSM), (<span class="string">'\u{1b3d}'</span>, <span class="string">'\u{1b41}'</span>, L), (<span class="string">'\u{1b42}'</span>, <span class="string">'\u{1b42}'</span>, NSM), (<span class="string">'\u{1b43}'</span>,
    <span class="string">'\u{1b4c}'</span>, L), (<span class="string">'\u{1b50}'</span>, <span class="string">'\u{1b6a}'</span>, L), (<span class="string">'\u{1b6b}'</span>, <span class="string">'\u{1b73}'</span>, NSM), (<span class="string">'\u{1b74}'</span>,
    <span class="string">'\u{1b7e}'</span>, L), (<span class="string">'\u{1b80}'</span>, <span class="string">'\u{1b81}'</span>, NSM), (<span class="string">'\u{1b82}'</span>, <span class="string">'\u{1ba1}'</span>, L), (<span class="string">'\u{1ba2}'</span>,
    <span class="string">'\u{1ba5}'</span>, NSM), (<span class="string">'\u{1ba6}'</span>, <span class="string">'\u{1ba7}'</span>, L), (<span class="string">'\u{1ba8}'</span>, <span class="string">'\u{1ba9}'</span>, NSM), (<span class="string">'\u{1baa}'</span>,
    <span class="string">'\u{1baa}'</span>, L), (<span class="string">'\u{1bab}'</span>, <span class="string">'\u{1bad}'</span>, NSM), (<span class="string">'\u{1bae}'</span>, <span class="string">'\u{1be5}'</span>, L), (<span class="string">'\u{1be6}'</span>,
    <span class="string">'\u{1be6}'</span>, NSM), (<span class="string">'\u{1be7}'</span>, <span class="string">'\u{1be7}'</span>, L), (<span class="string">'\u{1be8}'</span>, <span class="string">'\u{1be9}'</span>, NSM), (<span class="string">'\u{1bea}'</span>,
    <span class="string">'\u{1bec}'</span>, L), (<span class="string">'\u{1bed}'</span>, <span class="string">'\u{1bed}'</span>, NSM), (<span class="string">'\u{1bee}'</span>, <span class="string">'\u{1bee}'</span>, L), (<span class="string">'\u{1bef}'</span>,
    <span class="string">'\u{1bf1}'</span>, NSM), (<span class="string">'\u{1bf2}'</span>, <span class="string">'\u{1bf3}'</span>, L), (<span class="string">'\u{1bfc}'</span>, <span class="string">'\u{1c2b}'</span>, L), (<span class="string">'\u{1c2c}'</span>,
    <span class="string">'\u{1c33}'</span>, NSM), (<span class="string">'\u{1c34}'</span>, <span class="string">'\u{1c35}'</span>, L), (<span class="string">'\u{1c36}'</span>, <span class="string">'\u{1c37}'</span>, NSM), (<span class="string">'\u{1c3b}'</span>,
    <span class="string">'\u{1c49}'</span>, L), (<span class="string">'\u{1c4d}'</span>, <span class="string">'\u{1c88}'</span>, L), (<span class="string">'\u{1c90}'</span>, <span class="string">'\u{1cba}'</span>, L), (<span class="string">'\u{1cbd}'</span>,
    <span class="string">'\u{1cc7}'</span>, L), (<span class="string">'\u{1cd0}'</span>, <span class="string">'\u{1cd2}'</span>, NSM), (<span class="string">'\u{1cd3}'</span>, <span class="string">'\u{1cd3}'</span>, L), (<span class="string">'\u{1cd4}'</span>,
    <span class="string">'\u{1ce0}'</span>, NSM), (<span class="string">'\u{1ce1}'</span>, <span class="string">'\u{1ce1}'</span>, L), (<span class="string">'\u{1ce2}'</span>, <span class="string">'\u{1ce8}'</span>, NSM), (<span class="string">'\u{1ce9}'</span>,
    <span class="string">'\u{1cec}'</span>, L), (<span class="string">'\u{1ced}'</span>, <span class="string">'\u{1ced}'</span>, NSM), (<span class="string">'\u{1cee}'</span>, <span class="string">'\u{1cf3}'</span>, L), (<span class="string">'\u{1cf4}'</span>,
    <span class="string">'\u{1cf4}'</span>, NSM), (<span class="string">'\u{1cf5}'</span>, <span class="string">'\u{1cf7}'</span>, L), (<span class="string">'\u{1cf8}'</span>, <span class="string">'\u{1cf9}'</span>, NSM), (<span class="string">'\u{1cfa}'</span>,
    <span class="string">'\u{1cfa}'</span>, L), (<span class="string">'\u{1d00}'</span>, <span class="string">'\u{1dbf}'</span>, L), (<span class="string">'\u{1dc0}'</span>, <span class="string">'\u{1dff}'</span>, NSM), (<span class="string">'\u{1e00}'</span>,
    <span class="string">'\u{1f15}'</span>, L), (<span class="string">'\u{1f18}'</span>, <span class="string">'\u{1f1d}'</span>, L), (<span class="string">'\u{1f20}'</span>, <span class="string">'\u{1f45}'</span>, L), (<span class="string">'\u{1f48}'</span>,
    <span class="string">'\u{1f4d}'</span>, L), (<span class="string">'\u{1f50}'</span>, <span class="string">'\u{1f57}'</span>, L), (<span class="string">'\u{1f59}'</span>, <span class="string">'\u{1f59}'</span>, L), (<span class="string">'\u{1f5b}'</span>,
    <span class="string">'\u{1f5b}'</span>, L), (<span class="string">'\u{1f5d}'</span>, <span class="string">'\u{1f5d}'</span>, L), (<span class="string">'\u{1f5f}'</span>, <span class="string">'\u{1f7d}'</span>, L), (<span class="string">'\u{1f80}'</span>,
    <span class="string">'\u{1fb4}'</span>, L), (<span class="string">'\u{1fb6}'</span>, <span class="string">'\u{1fbc}'</span>, L), (<span class="string">'\u{1fbd}'</span>, <span class="string">'\u{1fbd}'</span>, ON), (<span class="string">'\u{1fbe}'</span>,
    <span class="string">'\u{1fbe}'</span>, L), (<span class="string">'\u{1fbf}'</span>, <span class="string">'\u{1fc1}'</span>, ON), (<span class="string">'\u{1fc2}'</span>, <span class="string">'\u{1fc4}'</span>, L), (<span class="string">'\u{1fc6}'</span>,
    <span class="string">'\u{1fcc}'</span>, L), (<span class="string">'\u{1fcd}'</span>, <span class="string">'\u{1fcf}'</span>, ON), (<span class="string">'\u{1fd0}'</span>, <span class="string">'\u{1fd3}'</span>, L), (<span class="string">'\u{1fd6}'</span>,
    <span class="string">'\u{1fdb}'</span>, L), (<span class="string">'\u{1fdd}'</span>, <span class="string">'\u{1fdf}'</span>, ON), (<span class="string">'\u{1fe0}'</span>, <span class="string">'\u{1fec}'</span>, L), (<span class="string">'\u{1fed}'</span>,
    <span class="string">'\u{1fef}'</span>, ON), (<span class="string">'\u{1ff2}'</span>, <span class="string">'\u{1ff4}'</span>, L), (<span class="string">'\u{1ff6}'</span>, <span class="string">'\u{1ffc}'</span>, L), (<span class="string">'\u{1ffd}'</span>,
    <span class="string">'\u{1ffe}'</span>, ON), (<span class="string">'\u{2000}'</span>, <span class="string">'\u{200a}'</span>, WS), (<span class="string">'\u{200b}'</span>, <span class="string">'\u{200d}'</span>, BN), (<span class="string">'\u{200e}'</span>,
    <span class="string">'\u{200e}'</span>, L), (<span class="string">'\u{200f}'</span>, <span class="string">'\u{200f}'</span>, R), (<span class="string">'\u{2010}'</span>, <span class="string">'\u{2027}'</span>, ON), (<span class="string">'\u{2028}'</span>,
    <span class="string">'\u{2028}'</span>, WS), (<span class="string">'\u{2029}'</span>, <span class="string">'\u{2029}'</span>, B), (<span class="string">'\u{202a}'</span>, <span class="string">'\u{202a}'</span>, LRE), (<span class="string">'\u{202b}'</span>,
    <span class="string">'\u{202b}'</span>, RLE), (<span class="string">'\u{202c}'</span>, <span class="string">'\u{202c}'</span>, PDF), (<span class="string">'\u{202d}'</span>, <span class="string">'\u{202d}'</span>, LRO), (<span class="string">'\u{202e}'</span>,
    <span class="string">'\u{202e}'</span>, RLO), (<span class="string">'\u{202f}'</span>, <span class="string">'\u{202f}'</span>, CS), (<span class="string">'\u{2030}'</span>, <span class="string">'\u{2034}'</span>, ET), (<span class="string">'\u{2035}'</span>,
    <span class="string">'\u{2043}'</span>, ON), (<span class="string">'\u{2044}'</span>, <span class="string">'\u{2044}'</span>, CS), (<span class="string">'\u{2045}'</span>, <span class="string">'\u{205e}'</span>, ON), (<span class="string">'\u{205f}'</span>,
    <span class="string">'\u{205f}'</span>, WS), (<span class="string">'\u{2060}'</span>, <span class="string">'\u{2064}'</span>, BN), (<span class="string">'\u{2066}'</span>, <span class="string">'\u{2066}'</span>, LRI), (<span class="string">'\u{2067}'</span>,
    <span class="string">'\u{2067}'</span>, RLI), (<span class="string">'\u{2068}'</span>, <span class="string">'\u{2068}'</span>, FSI), (<span class="string">'\u{2069}'</span>, <span class="string">'\u{2069}'</span>, PDI), (<span class="string">'\u{206a}'</span>,
    <span class="string">'\u{206f}'</span>, BN), (<span class="string">'\u{2070}'</span>, <span class="string">'\u{2070}'</span>, EN), (<span class="string">'\u{2071}'</span>, <span class="string">'\u{2071}'</span>, L), (<span class="string">'\u{2074}'</span>,
    <span class="string">'\u{2079}'</span>, EN), (<span class="string">'\u{207a}'</span>, <span class="string">'\u{207b}'</span>, ES), (<span class="string">'\u{207c}'</span>, <span class="string">'\u{207e}'</span>, ON), (<span class="string">'\u{207f}'</span>,
    <span class="string">'\u{207f}'</span>, L), (<span class="string">'\u{2080}'</span>, <span class="string">'\u{2089}'</span>, EN), (<span class="string">'\u{208a}'</span>, <span class="string">'\u{208b}'</span>, ES), (<span class="string">'\u{208c}'</span>,
    <span class="string">'\u{208e}'</span>, ON), (<span class="string">'\u{2090}'</span>, <span class="string">'\u{209c}'</span>, L), (<span class="string">'\u{20a0}'</span>, <span class="string">'\u{20cf}'</span>, ET), (<span class="string">'\u{20d0}'</span>,
    <span class="string">'\u{20f0}'</span>, NSM), (<span class="string">'\u{2100}'</span>, <span class="string">'\u{2101}'</span>, ON), (<span class="string">'\u{2102}'</span>, <span class="string">'\u{2102}'</span>, L), (<span class="string">'\u{2103}'</span>,
    <span class="string">'\u{2106}'</span>, ON), (<span class="string">'\u{2107}'</span>, <span class="string">'\u{2107}'</span>, L), (<span class="string">'\u{2108}'</span>, <span class="string">'\u{2109}'</span>, ON), (<span class="string">'\u{210a}'</span>,
    <span class="string">'\u{2113}'</span>, L), (<span class="string">'\u{2114}'</span>, <span class="string">'\u{2114}'</span>, ON), (<span class="string">'\u{2115}'</span>, <span class="string">'\u{2115}'</span>, L), (<span class="string">'\u{2116}'</span>,
    <span class="string">'\u{2118}'</span>, ON), (<span class="string">'\u{2119}'</span>, <span class="string">'\u{211d}'</span>, L), (<span class="string">'\u{211e}'</span>, <span class="string">'\u{2123}'</span>, ON), (<span class="string">'\u{2124}'</span>,
    <span class="string">'\u{2124}'</span>, L), (<span class="string">'\u{2125}'</span>, <span class="string">'\u{2125}'</span>, ON), (<span class="string">'\u{2126}'</span>, <span class="string">'\u{2126}'</span>, L), (<span class="string">'\u{2127}'</span>,
    <span class="string">'\u{2127}'</span>, ON), (<span class="string">'\u{2128}'</span>, <span class="string">'\u{2128}'</span>, L), (<span class="string">'\u{2129}'</span>, <span class="string">'\u{2129}'</span>, ON), (<span class="string">'\u{212a}'</span>,
    <span class="string">'\u{212d}'</span>, L), (<span class="string">'\u{212e}'</span>, <span class="string">'\u{212e}'</span>, ET), (<span class="string">'\u{212f}'</span>, <span class="string">'\u{2139}'</span>, L), (<span class="string">'\u{213a}'</span>,
    <span class="string">'\u{213b}'</span>, ON), (<span class="string">'\u{213c}'</span>, <span class="string">'\u{213f}'</span>, L), (<span class="string">'\u{2140}'</span>, <span class="string">'\u{2144}'</span>, ON), (<span class="string">'\u{2145}'</span>,
    <span class="string">'\u{2149}'</span>, L), (<span class="string">'\u{214a}'</span>, <span class="string">'\u{214d}'</span>, ON), (<span class="string">'\u{214e}'</span>, <span class="string">'\u{214f}'</span>, L), (<span class="string">'\u{2150}'</span>,
    <span class="string">'\u{215f}'</span>, ON), (<span class="string">'\u{2160}'</span>, <span class="string">'\u{2188}'</span>, L), (<span class="string">'\u{2189}'</span>, <span class="string">'\u{218b}'</span>, ON), (<span class="string">'\u{2190}'</span>,
    <span class="string">'\u{2211}'</span>, ON), (<span class="string">'\u{2212}'</span>, <span class="string">'\u{2212}'</span>, ES), (<span class="string">'\u{2213}'</span>, <span class="string">'\u{2213}'</span>, ET), (<span class="string">'\u{2214}'</span>,
    <span class="string">'\u{2335}'</span>, ON), (<span class="string">'\u{2336}'</span>, <span class="string">'\u{237a}'</span>, L), (<span class="string">'\u{237b}'</span>, <span class="string">'\u{2394}'</span>, ON), (<span class="string">'\u{2395}'</span>,
    <span class="string">'\u{2395}'</span>, L), (<span class="string">'\u{2396}'</span>, <span class="string">'\u{2426}'</span>, ON), (<span class="string">'\u{2440}'</span>, <span class="string">'\u{244a}'</span>, ON), (<span class="string">'\u{2460}'</span>,
    <span class="string">'\u{2487}'</span>, ON), (<span class="string">'\u{2488}'</span>, <span class="string">'\u{249b}'</span>, EN), (<span class="string">'\u{249c}'</span>, <span class="string">'\u{24e9}'</span>, L), (<span class="string">'\u{24ea}'</span>,
    <span class="string">'\u{26ab}'</span>, ON), (<span class="string">'\u{26ac}'</span>, <span class="string">'\u{26ac}'</span>, L), (<span class="string">'\u{26ad}'</span>, <span class="string">'\u{27ff}'</span>, ON), (<span class="string">'\u{2800}'</span>,
    <span class="string">'\u{28ff}'</span>, L), (<span class="string">'\u{2900}'</span>, <span class="string">'\u{2b73}'</span>, ON), (<span class="string">'\u{2b76}'</span>, <span class="string">'\u{2b95}'</span>, ON), (<span class="string">'\u{2b97}'</span>,
    <span class="string">'\u{2bff}'</span>, ON), (<span class="string">'\u{2c00}'</span>, <span class="string">'\u{2ce4}'</span>, L), (<span class="string">'\u{2ce5}'</span>, <span class="string">'\u{2cea}'</span>, ON), (<span class="string">'\u{2ceb}'</span>,
    <span class="string">'\u{2cee}'</span>, L), (<span class="string">'\u{2cef}'</span>, <span class="string">'\u{2cf1}'</span>, NSM), (<span class="string">'\u{2cf2}'</span>, <span class="string">'\u{2cf3}'</span>, L), (<span class="string">'\u{2cf9}'</span>,
    <span class="string">'\u{2cff}'</span>, ON), (<span class="string">'\u{2d00}'</span>, <span class="string">'\u{2d25}'</span>, L), (<span class="string">'\u{2d27}'</span>, <span class="string">'\u{2d27}'</span>, L), (<span class="string">'\u{2d2d}'</span>,
    <span class="string">'\u{2d2d}'</span>, L), (<span class="string">'\u{2d30}'</span>, <span class="string">'\u{2d67}'</span>, L), (<span class="string">'\u{2d6f}'</span>, <span class="string">'\u{2d70}'</span>, L), (<span class="string">'\u{2d7f}'</span>,
    <span class="string">'\u{2d7f}'</span>, NSM), (<span class="string">'\u{2d80}'</span>, <span class="string">'\u{2d96}'</span>, L), (<span class="string">'\u{2da0}'</span>, <span class="string">'\u{2da6}'</span>, L), (<span class="string">'\u{2da8}'</span>,
    <span class="string">'\u{2dae}'</span>, L), (<span class="string">'\u{2db0}'</span>, <span class="string">'\u{2db6}'</span>, L), (<span class="string">'\u{2db8}'</span>, <span class="string">'\u{2dbe}'</span>, L), (<span class="string">'\u{2dc0}'</span>,
    <span class="string">'\u{2dc6}'</span>, L), (<span class="string">'\u{2dc8}'</span>, <span class="string">'\u{2dce}'</span>, L), (<span class="string">'\u{2dd0}'</span>, <span class="string">'\u{2dd6}'</span>, L), (<span class="string">'\u{2dd8}'</span>,
    <span class="string">'\u{2dde}'</span>, L), (<span class="string">'\u{2de0}'</span>, <span class="string">'\u{2dff}'</span>, NSM), (<span class="string">'\u{2e00}'</span>, <span class="string">'\u{2e5d}'</span>, ON), (<span class="string">'\u{2e80}'</span>,
    <span class="string">'\u{2e99}'</span>, ON), (<span class="string">'\u{2e9b}'</span>, <span class="string">'\u{2ef3}'</span>, ON), (<span class="string">'\u{2f00}'</span>, <span class="string">'\u{2fd5}'</span>, ON), (<span class="string">'\u{2ff0}'</span>,
    <span class="string">'\u{2ffb}'</span>, ON), (<span class="string">'\u{3000}'</span>, <span class="string">'\u{3000}'</span>, WS), (<span class="string">'\u{3001}'</span>, <span class="string">'\u{3004}'</span>, ON), (<span class="string">'\u{3005}'</span>,
    <span class="string">'\u{3007}'</span>, L), (<span class="string">'\u{3008}'</span>, <span class="string">'\u{3020}'</span>, ON), (<span class="string">'\u{3021}'</span>, <span class="string">'\u{3029}'</span>, L), (<span class="string">'\u{302a}'</span>,
    <span class="string">'\u{302d}'</span>, NSM), (<span class="string">'\u{302e}'</span>, <span class="string">'\u{302f}'</span>, L), (<span class="string">'\u{3030}'</span>, <span class="string">'\u{3030}'</span>, ON), (<span class="string">'\u{3031}'</span>,
    <span class="string">'\u{3035}'</span>, L), (<span class="string">'\u{3036}'</span>, <span class="string">'\u{3037}'</span>, ON), (<span class="string">'\u{3038}'</span>, <span class="string">'\u{303c}'</span>, L), (<span class="string">'\u{303d}'</span>,
    <span class="string">'\u{303f}'</span>, ON), (<span class="string">'\u{3041}'</span>, <span class="string">'\u{3096}'</span>, L), (<span class="string">'\u{3099}'</span>, <span class="string">'\u{309a}'</span>, NSM), (<span class="string">'\u{309b}'</span>,
    <span class="string">'\u{309c}'</span>, ON), (<span class="string">'\u{309d}'</span>, <span class="string">'\u{309f}'</span>, L), (<span class="string">'\u{30a0}'</span>, <span class="string">'\u{30a0}'</span>, ON), (<span class="string">'\u{30a1}'</span>,
    <span class="string">'\u{30fa}'</span>, L), (<span class="string">'\u{30fb}'</span>, <span class="string">'\u{30fb}'</span>, ON), (<span class="string">'\u{30fc}'</span>, <span class="string">'\u{30ff}'</span>, L), (<span class="string">'\u{3105}'</span>,
    <span class="string">'\u{312f}'</span>, L), (<span class="string">'\u{3131}'</span>, <span class="string">'\u{318e}'</span>, L), (<span class="string">'\u{3190}'</span>, <span class="string">'\u{31bf}'</span>, L), (<span class="string">'\u{31c0}'</span>,
    <span class="string">'\u{31e3}'</span>, ON), (<span class="string">'\u{31f0}'</span>, <span class="string">'\u{321c}'</span>, L), (<span class="string">'\u{321d}'</span>, <span class="string">'\u{321e}'</span>, ON), (<span class="string">'\u{3220}'</span>,
    <span class="string">'\u{324f}'</span>, L), (<span class="string">'\u{3250}'</span>, <span class="string">'\u{325f}'</span>, ON), (<span class="string">'\u{3260}'</span>, <span class="string">'\u{327b}'</span>, L), (<span class="string">'\u{327c}'</span>,
    <span class="string">'\u{327e}'</span>, ON), (<span class="string">'\u{327f}'</span>, <span class="string">'\u{32b0}'</span>, L), (<span class="string">'\u{32b1}'</span>, <span class="string">'\u{32bf}'</span>, ON), (<span class="string">'\u{32c0}'</span>,
    <span class="string">'\u{32cb}'</span>, L), (<span class="string">'\u{32cc}'</span>, <span class="string">'\u{32cf}'</span>, ON), (<span class="string">'\u{32d0}'</span>, <span class="string">'\u{3376}'</span>, L), (<span class="string">'\u{3377}'</span>,
    <span class="string">'\u{337a}'</span>, ON), (<span class="string">'\u{337b}'</span>, <span class="string">'\u{33dd}'</span>, L), (<span class="string">'\u{33de}'</span>, <span class="string">'\u{33df}'</span>, ON), (<span class="string">'\u{33e0}'</span>,
    <span class="string">'\u{33fe}'</span>, L), (<span class="string">'\u{33ff}'</span>, <span class="string">'\u{33ff}'</span>, ON), (<span class="string">'\u{3400}'</span>, <span class="string">'\u{4dbf}'</span>, L), (<span class="string">'\u{4dc0}'</span>,
    <span class="string">'\u{4dff}'</span>, ON), (<span class="string">'\u{4e00}'</span>, <span class="string">'\u{a48c}'</span>, L), (<span class="string">'\u{a490}'</span>, <span class="string">'\u{a4c6}'</span>, ON), (<span class="string">'\u{a4d0}'</span>,
    <span class="string">'\u{a60c}'</span>, L), (<span class="string">'\u{a60d}'</span>, <span class="string">'\u{a60f}'</span>, ON), (<span class="string">'\u{a610}'</span>, <span class="string">'\u{a62b}'</span>, L), (<span class="string">'\u{a640}'</span>,
    <span class="string">'\u{a66e}'</span>, L), (<span class="string">'\u{a66f}'</span>, <span class="string">'\u{a672}'</span>, NSM), (<span class="string">'\u{a673}'</span>, <span class="string">'\u{a673}'</span>, ON), (<span class="string">'\u{a674}'</span>,
    <span class="string">'\u{a67d}'</span>, NSM), (<span class="string">'\u{a67e}'</span>, <span class="string">'\u{a67f}'</span>, ON), (<span class="string">'\u{a680}'</span>, <span class="string">'\u{a69d}'</span>, L), (<span class="string">'\u{a69e}'</span>,
    <span class="string">'\u{a69f}'</span>, NSM), (<span class="string">'\u{a6a0}'</span>, <span class="string">'\u{a6ef}'</span>, L), (<span class="string">'\u{a6f0}'</span>, <span class="string">'\u{a6f1}'</span>, NSM), (<span class="string">'\u{a6f2}'</span>,
    <span class="string">'\u{a6f7}'</span>, L), (<span class="string">'\u{a700}'</span>, <span class="string">'\u{a721}'</span>, ON), (<span class="string">'\u{a722}'</span>, <span class="string">'\u{a787}'</span>, L), (<span class="string">'\u{a788}'</span>,
    <span class="string">'\u{a788}'</span>, ON), (<span class="string">'\u{a789}'</span>, <span class="string">'\u{a7ca}'</span>, L), (<span class="string">'\u{a7d0}'</span>, <span class="string">'\u{a7d1}'</span>, L), (<span class="string">'\u{a7d3}'</span>,
    <span class="string">'\u{a7d3}'</span>, L), (<span class="string">'\u{a7d5}'</span>, <span class="string">'\u{a7d9}'</span>, L), (<span class="string">'\u{a7f2}'</span>, <span class="string">'\u{a801}'</span>, L), (<span class="string">'\u{a802}'</span>,
    <span class="string">'\u{a802}'</span>, NSM), (<span class="string">'\u{a803}'</span>, <span class="string">'\u{a805}'</span>, L), (<span class="string">'\u{a806}'</span>, <span class="string">'\u{a806}'</span>, NSM), (<span class="string">'\u{a807}'</span>,
    <span class="string">'\u{a80a}'</span>, L), (<span class="string">'\u{a80b}'</span>, <span class="string">'\u{a80b}'</span>, NSM), (<span class="string">'\u{a80c}'</span>, <span class="string">'\u{a824}'</span>, L), (<span class="string">'\u{a825}'</span>,
    <span class="string">'\u{a826}'</span>, NSM), (<span class="string">'\u{a827}'</span>, <span class="string">'\u{a827}'</span>, L), (<span class="string">'\u{a828}'</span>, <span class="string">'\u{a82b}'</span>, ON), (<span class="string">'\u{a82c}'</span>,
    <span class="string">'\u{a82c}'</span>, NSM), (<span class="string">'\u{a830}'</span>, <span class="string">'\u{a837}'</span>, L), (<span class="string">'\u{a838}'</span>, <span class="string">'\u{a839}'</span>, ET), (<span class="string">'\u{a840}'</span>,
    <span class="string">'\u{a873}'</span>, L), (<span class="string">'\u{a874}'</span>, <span class="string">'\u{a877}'</span>, ON), (<span class="string">'\u{a880}'</span>, <span class="string">'\u{a8c3}'</span>, L), (<span class="string">'\u{a8c4}'</span>,
    <span class="string">'\u{a8c5}'</span>, NSM), (<span class="string">'\u{a8ce}'</span>, <span class="string">'\u{a8d9}'</span>, L), (<span class="string">'\u{a8e0}'</span>, <span class="string">'\u{a8f1}'</span>, NSM), (<span class="string">'\u{a8f2}'</span>,
    <span class="string">'\u{a8fe}'</span>, L), (<span class="string">'\u{a8ff}'</span>, <span class="string">'\u{a8ff}'</span>, NSM), (<span class="string">'\u{a900}'</span>, <span class="string">'\u{a925}'</span>, L), (<span class="string">'\u{a926}'</span>,
    <span class="string">'\u{a92d}'</span>, NSM), (<span class="string">'\u{a92e}'</span>, <span class="string">'\u{a946}'</span>, L), (<span class="string">'\u{a947}'</span>, <span class="string">'\u{a951}'</span>, NSM), (<span class="string">'\u{a952}'</span>,
    <span class="string">'\u{a953}'</span>, L), (<span class="string">'\u{a95f}'</span>, <span class="string">'\u{a97c}'</span>, L), (<span class="string">'\u{a980}'</span>, <span class="string">'\u{a982}'</span>, NSM), (<span class="string">'\u{a983}'</span>,
    <span class="string">'\u{a9b2}'</span>, L), (<span class="string">'\u{a9b3}'</span>, <span class="string">'\u{a9b3}'</span>, NSM), (<span class="string">'\u{a9b4}'</span>, <span class="string">'\u{a9b5}'</span>, L), (<span class="string">'\u{a9b6}'</span>,
    <span class="string">'\u{a9b9}'</span>, NSM), (<span class="string">'\u{a9ba}'</span>, <span class="string">'\u{a9bb}'</span>, L), (<span class="string">'\u{a9bc}'</span>, <span class="string">'\u{a9bd}'</span>, NSM), (<span class="string">'\u{a9be}'</span>,
    <span class="string">'\u{a9cd}'</span>, L), (<span class="string">'\u{a9cf}'</span>, <span class="string">'\u{a9d9}'</span>, L), (<span class="string">'\u{a9de}'</span>, <span class="string">'\u{a9e4}'</span>, L), (<span class="string">'\u{a9e5}'</span>,
    <span class="string">'\u{a9e5}'</span>, NSM), (<span class="string">'\u{a9e6}'</span>, <span class="string">'\u{a9fe}'</span>, L), (<span class="string">'\u{aa00}'</span>, <span class="string">'\u{aa28}'</span>, L), (<span class="string">'\u{aa29}'</span>,
    <span class="string">'\u{aa2e}'</span>, NSM), (<span class="string">'\u{aa2f}'</span>, <span class="string">'\u{aa30}'</span>, L), (<span class="string">'\u{aa31}'</span>, <span class="string">'\u{aa32}'</span>, NSM), (<span class="string">'\u{aa33}'</span>,
    <span class="string">'\u{aa34}'</span>, L), (<span class="string">'\u{aa35}'</span>, <span class="string">'\u{aa36}'</span>, NSM), (<span class="string">'\u{aa40}'</span>, <span class="string">'\u{aa42}'</span>, L), (<span class="string">'\u{aa43}'</span>,
    <span class="string">'\u{aa43}'</span>, NSM), (<span class="string">'\u{aa44}'</span>, <span class="string">'\u{aa4b}'</span>, L), (<span class="string">'\u{aa4c}'</span>, <span class="string">'\u{aa4c}'</span>, NSM), (<span class="string">'\u{aa4d}'</span>,
    <span class="string">'\u{aa4d}'</span>, L), (<span class="string">'\u{aa50}'</span>, <span class="string">'\u{aa59}'</span>, L), (<span class="string">'\u{aa5c}'</span>, <span class="string">'\u{aa7b}'</span>, L), (<span class="string">'\u{aa7c}'</span>,
    <span class="string">'\u{aa7c}'</span>, NSM), (<span class="string">'\u{aa7d}'</span>, <span class="string">'\u{aaaf}'</span>, L), (<span class="string">'\u{aab0}'</span>, <span class="string">'\u{aab0}'</span>, NSM), (<span class="string">'\u{aab1}'</span>,
    <span class="string">'\u{aab1}'</span>, L), (<span class="string">'\u{aab2}'</span>, <span class="string">'\u{aab4}'</span>, NSM), (<span class="string">'\u{aab5}'</span>, <span class="string">'\u{aab6}'</span>, L), (<span class="string">'\u{aab7}'</span>,
    <span class="string">'\u{aab8}'</span>, NSM), (<span class="string">'\u{aab9}'</span>, <span class="string">'\u{aabd}'</span>, L), (<span class="string">'\u{aabe}'</span>, <span class="string">'\u{aabf}'</span>, NSM), (<span class="string">'\u{aac0}'</span>,
    <span class="string">'\u{aac0}'</span>, L), (<span class="string">'\u{aac1}'</span>, <span class="string">'\u{aac1}'</span>, NSM), (<span class="string">'\u{aac2}'</span>, <span class="string">'\u{aac2}'</span>, L), (<span class="string">'\u{aadb}'</span>,
    <span class="string">'\u{aaeb}'</span>, L), (<span class="string">'\u{aaec}'</span>, <span class="string">'\u{aaed}'</span>, NSM), (<span class="string">'\u{aaee}'</span>, <span class="string">'\u{aaf5}'</span>, L), (<span class="string">'\u{aaf6}'</span>,
    <span class="string">'\u{aaf6}'</span>, NSM), (<span class="string">'\u{ab01}'</span>, <span class="string">'\u{ab06}'</span>, L), (<span class="string">'\u{ab09}'</span>, <span class="string">'\u{ab0e}'</span>, L), (<span class="string">'\u{ab11}'</span>,
    <span class="string">'\u{ab16}'</span>, L), (<span class="string">'\u{ab20}'</span>, <span class="string">'\u{ab26}'</span>, L), (<span class="string">'\u{ab28}'</span>, <span class="string">'\u{ab2e}'</span>, L), (<span class="string">'\u{ab30}'</span>,
    <span class="string">'\u{ab69}'</span>, L), (<span class="string">'\u{ab6a}'</span>, <span class="string">'\u{ab6b}'</span>, ON), (<span class="string">'\u{ab70}'</span>, <span class="string">'\u{abe4}'</span>, L), (<span class="string">'\u{abe5}'</span>,
    <span class="string">'\u{abe5}'</span>, NSM), (<span class="string">'\u{abe6}'</span>, <span class="string">'\u{abe7}'</span>, L), (<span class="string">'\u{abe8}'</span>, <span class="string">'\u{abe8}'</span>, NSM), (<span class="string">'\u{abe9}'</span>,
    <span class="string">'\u{abec}'</span>, L), (<span class="string">'\u{abed}'</span>, <span class="string">'\u{abed}'</span>, NSM), (<span class="string">'\u{abf0}'</span>, <span class="string">'\u{abf9}'</span>, L), (<span class="string">'\u{ac00}'</span>,
    <span class="string">'\u{d7a3}'</span>, L), (<span class="string">'\u{d7b0}'</span>, <span class="string">'\u{d7c6}'</span>, L), (<span class="string">'\u{d7cb}'</span>, <span class="string">'\u{d7fb}'</span>, L), (<span class="string">'\u{e000}'</span>,
    <span class="string">'\u{fa6d}'</span>, L), (<span class="string">'\u{fa70}'</span>, <span class="string">'\u{fad9}'</span>, L), (<span class="string">'\u{fb00}'</span>, <span class="string">'\u{fb06}'</span>, L), (<span class="string">'\u{fb13}'</span>,
    <span class="string">'\u{fb17}'</span>, L), (<span class="string">'\u{fb1d}'</span>, <span class="string">'\u{fb1d}'</span>, R), (<span class="string">'\u{fb1e}'</span>, <span class="string">'\u{fb1e}'</span>, NSM), (<span class="string">'\u{fb1f}'</span>,
    <span class="string">'\u{fb28}'</span>, R), (<span class="string">'\u{fb29}'</span>, <span class="string">'\u{fb29}'</span>, ES), (<span class="string">'\u{fb2a}'</span>, <span class="string">'\u{fb4f}'</span>, R), (<span class="string">'\u{fb50}'</span>,
    <span class="string">'\u{fd3d}'</span>, AL), (<span class="string">'\u{fd3e}'</span>, <span class="string">'\u{fd4f}'</span>, ON), (<span class="string">'\u{fd50}'</span>, <span class="string">'\u{fdce}'</span>, AL), (<span class="string">'\u{fdcf}'</span>,
    <span class="string">'\u{fdcf}'</span>, ON), (<span class="string">'\u{fdf0}'</span>, <span class="string">'\u{fdfc}'</span>, AL), (<span class="string">'\u{fdfd}'</span>, <span class="string">'\u{fdff}'</span>, ON), (<span class="string">'\u{fe00}'</span>,
    <span class="string">'\u{fe0f}'</span>, NSM), (<span class="string">'\u{fe10}'</span>, <span class="string">'\u{fe19}'</span>, ON), (<span class="string">'\u{fe20}'</span>, <span class="string">'\u{fe2f}'</span>, NSM), (<span class="string">'\u{fe30}'</span>,
    <span class="string">'\u{fe4f}'</span>, ON), (<span class="string">'\u{fe50}'</span>, <span class="string">'\u{fe50}'</span>, CS), (<span class="string">'\u{fe51}'</span>, <span class="string">'\u{fe51}'</span>, ON), (<span class="string">'\u{fe52}'</span>,
    <span class="string">'\u{fe52}'</span>, CS), (<span class="string">'\u{fe54}'</span>, <span class="string">'\u{fe54}'</span>, ON), (<span class="string">'\u{fe55}'</span>, <span class="string">'\u{fe55}'</span>, CS), (<span class="string">'\u{fe56}'</span>,
    <span class="string">'\u{fe5e}'</span>, ON), (<span class="string">'\u{fe5f}'</span>, <span class="string">'\u{fe5f}'</span>, ET), (<span class="string">'\u{fe60}'</span>, <span class="string">'\u{fe61}'</span>, ON), (<span class="string">'\u{fe62}'</span>,
    <span class="string">'\u{fe63}'</span>, ES), (<span class="string">'\u{fe64}'</span>, <span class="string">'\u{fe66}'</span>, ON), (<span class="string">'\u{fe68}'</span>, <span class="string">'\u{fe68}'</span>, ON), (<span class="string">'\u{fe69}'</span>,
    <span class="string">'\u{fe6a}'</span>, ET), (<span class="string">'\u{fe6b}'</span>, <span class="string">'\u{fe6b}'</span>, ON), (<span class="string">'\u{fe70}'</span>, <span class="string">'\u{fefe}'</span>, AL), (<span class="string">'\u{feff}'</span>,
    <span class="string">'\u{feff}'</span>, BN), (<span class="string">'\u{ff01}'</span>, <span class="string">'\u{ff02}'</span>, ON), (<span class="string">'\u{ff03}'</span>, <span class="string">'\u{ff05}'</span>, ET), (<span class="string">'\u{ff06}'</span>,
    <span class="string">'\u{ff0a}'</span>, ON), (<span class="string">'\u{ff0b}'</span>, <span class="string">'\u{ff0b}'</span>, ES), (<span class="string">'\u{ff0c}'</span>, <span class="string">'\u{ff0c}'</span>, CS), (<span class="string">'\u{ff0d}'</span>,
    <span class="string">'\u{ff0d}'</span>, ES), (<span class="string">'\u{ff0e}'</span>, <span class="string">'\u{ff0f}'</span>, CS), (<span class="string">'\u{ff10}'</span>, <span class="string">'\u{ff19}'</span>, EN), (<span class="string">'\u{ff1a}'</span>,
    <span class="string">'\u{ff1a}'</span>, CS), (<span class="string">'\u{ff1b}'</span>, <span class="string">'\u{ff20}'</span>, ON), (<span class="string">'\u{ff21}'</span>, <span class="string">'\u{ff3a}'</span>, L), (<span class="string">'\u{ff3b}'</span>,
    <span class="string">'\u{ff40}'</span>, ON), (<span class="string">'\u{ff41}'</span>, <span class="string">'\u{ff5a}'</span>, L), (<span class="string">'\u{ff5b}'</span>, <span class="string">'\u{ff65}'</span>, ON), (<span class="string">'\u{ff66}'</span>,
    <span class="string">'\u{ffbe}'</span>, L), (<span class="string">'\u{ffc2}'</span>, <span class="string">'\u{ffc7}'</span>, L), (<span class="string">'\u{ffca}'</span>, <span class="string">'\u{ffcf}'</span>, L), (<span class="string">'\u{ffd2}'</span>,
    <span class="string">'\u{ffd7}'</span>, L), (<span class="string">'\u{ffda}'</span>, <span class="string">'\u{ffdc}'</span>, L), (<span class="string">'\u{ffe0}'</span>, <span class="string">'\u{ffe1}'</span>, ET), (<span class="string">'\u{ffe2}'</span>,
    <span class="string">'\u{ffe4}'</span>, ON), (<span class="string">'\u{ffe5}'</span>, <span class="string">'\u{ffe6}'</span>, ET), (<span class="string">'\u{ffe8}'</span>, <span class="string">'\u{ffee}'</span>, ON), (<span class="string">'\u{fff9}'</span>,
    <span class="string">'\u{fffd}'</span>, ON), (<span class="string">'\u{10000}'</span>, <span class="string">'\u{1000b}'</span>, L), (<span class="string">'\u{1000d}'</span>, <span class="string">'\u{10026}'</span>, L), (<span class="string">'\u{10028}'</span>,
    <span class="string">'\u{1003a}'</span>, L), (<span class="string">'\u{1003c}'</span>, <span class="string">'\u{1003d}'</span>, L), (<span class="string">'\u{1003f}'</span>, <span class="string">'\u{1004d}'</span>, L), (<span class="string">'\u{10050}'</span>,
    <span class="string">'\u{1005d}'</span>, L), (<span class="string">'\u{10080}'</span>, <span class="string">'\u{100fa}'</span>, L), (<span class="string">'\u{10100}'</span>, <span class="string">'\u{10100}'</span>, L), (<span class="string">'\u{10101}'</span>,
    <span class="string">'\u{10101}'</span>, ON), (<span class="string">'\u{10102}'</span>, <span class="string">'\u{10102}'</span>, L), (<span class="string">'\u{10107}'</span>, <span class="string">'\u{10133}'</span>, L), (<span class="string">'\u{10137}'</span>,
    <span class="string">'\u{1013f}'</span>, L), (<span class="string">'\u{10140}'</span>, <span class="string">'\u{1018c}'</span>, ON), (<span class="string">'\u{1018d}'</span>, <span class="string">'\u{1018e}'</span>, L), (<span class="string">'\u{10190}'</span>,
    <span class="string">'\u{1019c}'</span>, ON), (<span class="string">'\u{101a0}'</span>, <span class="string">'\u{101a0}'</span>, ON), (<span class="string">'\u{101d0}'</span>, <span class="string">'\u{101fc}'</span>, L), (<span class="string">'\u{101fd}'</span>,
    <span class="string">'\u{101fd}'</span>, NSM), (<span class="string">'\u{10280}'</span>, <span class="string">'\u{1029c}'</span>, L), (<span class="string">'\u{102a0}'</span>, <span class="string">'\u{102d0}'</span>, L), (<span class="string">'\u{102e0}'</span>,
    <span class="string">'\u{102e0}'</span>, NSM), (<span class="string">'\u{102e1}'</span>, <span class="string">'\u{102fb}'</span>, EN), (<span class="string">'\u{10300}'</span>, <span class="string">'\u{10323}'</span>, L), (<span class="string">'\u{1032d}'</span>,
    <span class="string">'\u{1034a}'</span>, L), (<span class="string">'\u{10350}'</span>, <span class="string">'\u{10375}'</span>, L), (<span class="string">'\u{10376}'</span>, <span class="string">'\u{1037a}'</span>, NSM), (<span class="string">'\u{10380}'</span>,
    <span class="string">'\u{1039d}'</span>, L), (<span class="string">'\u{1039f}'</span>, <span class="string">'\u{103c3}'</span>, L), (<span class="string">'\u{103c8}'</span>, <span class="string">'\u{103d5}'</span>, L), (<span class="string">'\u{10400}'</span>,
    <span class="string">'\u{1049d}'</span>, L), (<span class="string">'\u{104a0}'</span>, <span class="string">'\u{104a9}'</span>, L), (<span class="string">'\u{104b0}'</span>, <span class="string">'\u{104d3}'</span>, L), (<span class="string">'\u{104d8}'</span>,
    <span class="string">'\u{104fb}'</span>, L), (<span class="string">'\u{10500}'</span>, <span class="string">'\u{10527}'</span>, L), (<span class="string">'\u{10530}'</span>, <span class="string">'\u{10563}'</span>, L), (<span class="string">'\u{1056f}'</span>,
    <span class="string">'\u{1057a}'</span>, L), (<span class="string">'\u{1057c}'</span>, <span class="string">'\u{1058a}'</span>, L), (<span class="string">'\u{1058c}'</span>, <span class="string">'\u{10592}'</span>, L), (<span class="string">'\u{10594}'</span>,
    <span class="string">'\u{10595}'</span>, L), (<span class="string">'\u{10597}'</span>, <span class="string">'\u{105a1}'</span>, L), (<span class="string">'\u{105a3}'</span>, <span class="string">'\u{105b1}'</span>, L), (<span class="string">'\u{105b3}'</span>,
    <span class="string">'\u{105b9}'</span>, L), (<span class="string">'\u{105bb}'</span>, <span class="string">'\u{105bc}'</span>, L), (<span class="string">'\u{10600}'</span>, <span class="string">'\u{10736}'</span>, L), (<span class="string">'\u{10740}'</span>,
    <span class="string">'\u{10755}'</span>, L), (<span class="string">'\u{10760}'</span>, <span class="string">'\u{10767}'</span>, L), (<span class="string">'\u{10780}'</span>, <span class="string">'\u{10785}'</span>, L), (<span class="string">'\u{10787}'</span>,
    <span class="string">'\u{107b0}'</span>, L), (<span class="string">'\u{107b2}'</span>, <span class="string">'\u{107ba}'</span>, L), (<span class="string">'\u{10800}'</span>, <span class="string">'\u{1091e}'</span>, R), (<span class="string">'\u{1091f}'</span>,
    <span class="string">'\u{1091f}'</span>, ON), (<span class="string">'\u{10920}'</span>, <span class="string">'\u{10a00}'</span>, R), (<span class="string">'\u{10a01}'</span>, <span class="string">'\u{10a03}'</span>, NSM), (<span class="string">'\u{10a04}'</span>,
    <span class="string">'\u{10a04}'</span>, R), (<span class="string">'\u{10a05}'</span>, <span class="string">'\u{10a06}'</span>, NSM), (<span class="string">'\u{10a07}'</span>, <span class="string">'\u{10a0b}'</span>, R), (<span class="string">'\u{10a0c}'</span>,
    <span class="string">'\u{10a0f}'</span>, NSM), (<span class="string">'\u{10a10}'</span>, <span class="string">'\u{10a37}'</span>, R), (<span class="string">'\u{10a38}'</span>, <span class="string">'\u{10a3a}'</span>, NSM), (<span class="string">'\u{10a3b}'</span>,
    <span class="string">'\u{10a3e}'</span>, R), (<span class="string">'\u{10a3f}'</span>, <span class="string">'\u{10a3f}'</span>, NSM), (<span class="string">'\u{10a40}'</span>, <span class="string">'\u{10ae4}'</span>, R), (<span class="string">'\u{10ae5}'</span>,
    <span class="string">'\u{10ae6}'</span>, NSM), (<span class="string">'\u{10ae7}'</span>, <span class="string">'\u{10b38}'</span>, R), (<span class="string">'\u{10b39}'</span>, <span class="string">'\u{10b3f}'</span>, ON), (<span class="string">'\u{10b40}'</span>,
    <span class="string">'\u{10cff}'</span>, R), (<span class="string">'\u{10d00}'</span>, <span class="string">'\u{10d23}'</span>, AL), (<span class="string">'\u{10d24}'</span>, <span class="string">'\u{10d27}'</span>, NSM), (<span class="string">'\u{10d28}'</span>,
    <span class="string">'\u{10d2f}'</span>, R), (<span class="string">'\u{10d30}'</span>, <span class="string">'\u{10d39}'</span>, AN), (<span class="string">'\u{10d3a}'</span>, <span class="string">'\u{10e5f}'</span>, R), (<span class="string">'\u{10e60}'</span>,
    <span class="string">'\u{10e7e}'</span>, AN), (<span class="string">'\u{10e7f}'</span>, <span class="string">'\u{10eaa}'</span>, R), (<span class="string">'\u{10eab}'</span>, <span class="string">'\u{10eac}'</span>, NSM), (<span class="string">'\u{10ead}'</span>,
    <span class="string">'\u{10efc}'</span>, R), (<span class="string">'\u{10efd}'</span>, <span class="string">'\u{10eff}'</span>, NSM), (<span class="string">'\u{10f00}'</span>, <span class="string">'\u{10f2f}'</span>, R), (<span class="string">'\u{10f30}'</span>,
    <span class="string">'\u{10f45}'</span>, AL), (<span class="string">'\u{10f46}'</span>, <span class="string">'\u{10f50}'</span>, NSM), (<span class="string">'\u{10f51}'</span>, <span class="string">'\u{10f59}'</span>, AL), (<span class="string">'\u{10f5a}'</span>,
    <span class="string">'\u{10f81}'</span>, R), (<span class="string">'\u{10f82}'</span>, <span class="string">'\u{10f85}'</span>, NSM), (<span class="string">'\u{10f86}'</span>, <span class="string">'\u{10fff}'</span>, R), (<span class="string">'\u{11000}'</span>,
    <span class="string">'\u{11000}'</span>, L), (<span class="string">'\u{11001}'</span>, <span class="string">'\u{11001}'</span>, NSM), (<span class="string">'\u{11002}'</span>, <span class="string">'\u{11037}'</span>, L), (<span class="string">'\u{11038}'</span>,
    <span class="string">'\u{11046}'</span>, NSM), (<span class="string">'\u{11047}'</span>, <span class="string">'\u{1104d}'</span>, L), (<span class="string">'\u{11052}'</span>, <span class="string">'\u{11065}'</span>, ON), (<span class="string">'\u{11066}'</span>,
    <span class="string">'\u{1106f}'</span>, L), (<span class="string">'\u{11070}'</span>, <span class="string">'\u{11070}'</span>, NSM), (<span class="string">'\u{11071}'</span>, <span class="string">'\u{11072}'</span>, L), (<span class="string">'\u{11073}'</span>,
    <span class="string">'\u{11074}'</span>, NSM), (<span class="string">'\u{11075}'</span>, <span class="string">'\u{11075}'</span>, L), (<span class="string">'\u{1107f}'</span>, <span class="string">'\u{11081}'</span>, NSM), (<span class="string">'\u{11082}'</span>,
    <span class="string">'\u{110b2}'</span>, L), (<span class="string">'\u{110b3}'</span>, <span class="string">'\u{110b6}'</span>, NSM), (<span class="string">'\u{110b7}'</span>, <span class="string">'\u{110b8}'</span>, L), (<span class="string">'\u{110b9}'</span>,
    <span class="string">'\u{110ba}'</span>, NSM), (<span class="string">'\u{110bb}'</span>, <span class="string">'\u{110c1}'</span>, L), (<span class="string">'\u{110c2}'</span>, <span class="string">'\u{110c2}'</span>, NSM), (<span class="string">'\u{110cd}'</span>,
    <span class="string">'\u{110cd}'</span>, L), (<span class="string">'\u{110d0}'</span>, <span class="string">'\u{110e8}'</span>, L), (<span class="string">'\u{110f0}'</span>, <span class="string">'\u{110f9}'</span>, L), (<span class="string">'\u{11100}'</span>,
    <span class="string">'\u{11102}'</span>, NSM), (<span class="string">'\u{11103}'</span>, <span class="string">'\u{11126}'</span>, L), (<span class="string">'\u{11127}'</span>, <span class="string">'\u{1112b}'</span>, NSM), (<span class="string">'\u{1112c}'</span>,
    <span class="string">'\u{1112c}'</span>, L), (<span class="string">'\u{1112d}'</span>, <span class="string">'\u{11134}'</span>, NSM), (<span class="string">'\u{11136}'</span>, <span class="string">'\u{11147}'</span>, L), (<span class="string">'\u{11150}'</span>,
    <span class="string">'\u{11172}'</span>, L), (<span class="string">'\u{11173}'</span>, <span class="string">'\u{11173}'</span>, NSM), (<span class="string">'\u{11174}'</span>, <span class="string">'\u{11176}'</span>, L), (<span class="string">'\u{11180}'</span>,
    <span class="string">'\u{11181}'</span>, NSM), (<span class="string">'\u{11182}'</span>, <span class="string">'\u{111b5}'</span>, L), (<span class="string">'\u{111b6}'</span>, <span class="string">'\u{111be}'</span>, NSM), (<span class="string">'\u{111bf}'</span>,
    <span class="string">'\u{111c8}'</span>, L), (<span class="string">'\u{111c9}'</span>, <span class="string">'\u{111cc}'</span>, NSM), (<span class="string">'\u{111cd}'</span>, <span class="string">'\u{111ce}'</span>, L), (<span class="string">'\u{111cf}'</span>,
    <span class="string">'\u{111cf}'</span>, NSM), (<span class="string">'\u{111d0}'</span>, <span class="string">'\u{111df}'</span>, L), (<span class="string">'\u{111e1}'</span>, <span class="string">'\u{111f4}'</span>, L), (<span class="string">'\u{11200}'</span>,
    <span class="string">'\u{11211}'</span>, L), (<span class="string">'\u{11213}'</span>, <span class="string">'\u{1122e}'</span>, L), (<span class="string">'\u{1122f}'</span>, <span class="string">'\u{11231}'</span>, NSM), (<span class="string">'\u{11232}'</span>,
    <span class="string">'\u{11233}'</span>, L), (<span class="string">'\u{11234}'</span>, <span class="string">'\u{11234}'</span>, NSM), (<span class="string">'\u{11235}'</span>, <span class="string">'\u{11235}'</span>, L), (<span class="string">'\u{11236}'</span>,
    <span class="string">'\u{11237}'</span>, NSM), (<span class="string">'\u{11238}'</span>, <span class="string">'\u{1123d}'</span>, L), (<span class="string">'\u{1123e}'</span>, <span class="string">'\u{1123e}'</span>, NSM), (<span class="string">'\u{1123f}'</span>,
    <span class="string">'\u{11240}'</span>, L), (<span class="string">'\u{11241}'</span>, <span class="string">'\u{11241}'</span>, NSM), (<span class="string">'\u{11280}'</span>, <span class="string">'\u{11286}'</span>, L), (<span class="string">'\u{11288}'</span>,
    <span class="string">'\u{11288}'</span>, L), (<span class="string">'\u{1128a}'</span>, <span class="string">'\u{1128d}'</span>, L), (<span class="string">'\u{1128f}'</span>, <span class="string">'\u{1129d}'</span>, L), (<span class="string">'\u{1129f}'</span>,
    <span class="string">'\u{112a9}'</span>, L), (<span class="string">'\u{112b0}'</span>, <span class="string">'\u{112de}'</span>, L), (<span class="string">'\u{112df}'</span>, <span class="string">'\u{112df}'</span>, NSM), (<span class="string">'\u{112e0}'</span>,
    <span class="string">'\u{112e2}'</span>, L), (<span class="string">'\u{112e3}'</span>, <span class="string">'\u{112ea}'</span>, NSM), (<span class="string">'\u{112f0}'</span>, <span class="string">'\u{112f9}'</span>, L), (<span class="string">'\u{11300}'</span>,
    <span class="string">'\u{11301}'</span>, NSM), (<span class="string">'\u{11302}'</span>, <span class="string">'\u{11303}'</span>, L), (<span class="string">'\u{11305}'</span>, <span class="string">'\u{1130c}'</span>, L), (<span class="string">'\u{1130f}'</span>,
    <span class="string">'\u{11310}'</span>, L), (<span class="string">'\u{11313}'</span>, <span class="string">'\u{11328}'</span>, L), (<span class="string">'\u{1132a}'</span>, <span class="string">'\u{11330}'</span>, L), (<span class="string">'\u{11332}'</span>,
    <span class="string">'\u{11333}'</span>, L), (<span class="string">'\u{11335}'</span>, <span class="string">'\u{11339}'</span>, L), (<span class="string">'\u{1133b}'</span>, <span class="string">'\u{1133c}'</span>, NSM), (<span class="string">'\u{1133d}'</span>,
    <span class="string">'\u{1133f}'</span>, L), (<span class="string">'\u{11340}'</span>, <span class="string">'\u{11340}'</span>, NSM), (<span class="string">'\u{11341}'</span>, <span class="string">'\u{11344}'</span>, L), (<span class="string">'\u{11347}'</span>,
    <span class="string">'\u{11348}'</span>, L), (<span class="string">'\u{1134b}'</span>, <span class="string">'\u{1134d}'</span>, L), (<span class="string">'\u{11350}'</span>, <span class="string">'\u{11350}'</span>, L), (<span class="string">'\u{11357}'</span>,
    <span class="string">'\u{11357}'</span>, L), (<span class="string">'\u{1135d}'</span>, <span class="string">'\u{11363}'</span>, L), (<span class="string">'\u{11366}'</span>, <span class="string">'\u{1136c}'</span>, NSM), (<span class="string">'\u{11370}'</span>,
    <span class="string">'\u{11374}'</span>, NSM), (<span class="string">'\u{11400}'</span>, <span class="string">'\u{11437}'</span>, L), (<span class="string">'\u{11438}'</span>, <span class="string">'\u{1143f}'</span>, NSM), (<span class="string">'\u{11440}'</span>,
    <span class="string">'\u{11441}'</span>, L), (<span class="string">'\u{11442}'</span>, <span class="string">'\u{11444}'</span>, NSM), (<span class="string">'\u{11445}'</span>, <span class="string">'\u{11445}'</span>, L), (<span class="string">'\u{11446}'</span>,
    <span class="string">'\u{11446}'</span>, NSM), (<span class="string">'\u{11447}'</span>, <span class="string">'\u{1145b}'</span>, L), (<span class="string">'\u{1145d}'</span>, <span class="string">'\u{1145d}'</span>, L), (<span class="string">'\u{1145e}'</span>,
    <span class="string">'\u{1145e}'</span>, NSM), (<span class="string">'\u{1145f}'</span>, <span class="string">'\u{11461}'</span>, L), (<span class="string">'\u{11480}'</span>, <span class="string">'\u{114b2}'</span>, L), (<span class="string">'\u{114b3}'</span>,
    <span class="string">'\u{114b8}'</span>, NSM), (<span class="string">'\u{114b9}'</span>, <span class="string">'\u{114b9}'</span>, L), (<span class="string">'\u{114ba}'</span>, <span class="string">'\u{114ba}'</span>, NSM), (<span class="string">'\u{114bb}'</span>,
    <span class="string">'\u{114be}'</span>, L), (<span class="string">'\u{114bf}'</span>, <span class="string">'\u{114c0}'</span>, NSM), (<span class="string">'\u{114c1}'</span>, <span class="string">'\u{114c1}'</span>, L), (<span class="string">'\u{114c2}'</span>,
    <span class="string">'\u{114c3}'</span>, NSM), (<span class="string">'\u{114c4}'</span>, <span class="string">'\u{114c7}'</span>, L), (<span class="string">'\u{114d0}'</span>, <span class="string">'\u{114d9}'</span>, L), (<span class="string">'\u{11580}'</span>,
    <span class="string">'\u{115b1}'</span>, L), (<span class="string">'\u{115b2}'</span>, <span class="string">'\u{115b5}'</span>, NSM), (<span class="string">'\u{115b8}'</span>, <span class="string">'\u{115bb}'</span>, L), (<span class="string">'\u{115bc}'</span>,
    <span class="string">'\u{115bd}'</span>, NSM), (<span class="string">'\u{115be}'</span>, <span class="string">'\u{115be}'</span>, L), (<span class="string">'\u{115bf}'</span>, <span class="string">'\u{115c0}'</span>, NSM), (<span class="string">'\u{115c1}'</span>,
    <span class="string">'\u{115db}'</span>, L), (<span class="string">'\u{115dc}'</span>, <span class="string">'\u{115dd}'</span>, NSM), (<span class="string">'\u{11600}'</span>, <span class="string">'\u{11632}'</span>, L), (<span class="string">'\u{11633}'</span>,
    <span class="string">'\u{1163a}'</span>, NSM), (<span class="string">'\u{1163b}'</span>, <span class="string">'\u{1163c}'</span>, L), (<span class="string">'\u{1163d}'</span>, <span class="string">'\u{1163d}'</span>, NSM), (<span class="string">'\u{1163e}'</span>,
    <span class="string">'\u{1163e}'</span>, L), (<span class="string">'\u{1163f}'</span>, <span class="string">'\u{11640}'</span>, NSM), (<span class="string">'\u{11641}'</span>, <span class="string">'\u{11644}'</span>, L), (<span class="string">'\u{11650}'</span>,
    <span class="string">'\u{11659}'</span>, L), (<span class="string">'\u{11660}'</span>, <span class="string">'\u{1166c}'</span>, ON), (<span class="string">'\u{11680}'</span>, <span class="string">'\u{116aa}'</span>, L), (<span class="string">'\u{116ab}'</span>,
    <span class="string">'\u{116ab}'</span>, NSM), (<span class="string">'\u{116ac}'</span>, <span class="string">'\u{116ac}'</span>, L), (<span class="string">'\u{116ad}'</span>, <span class="string">'\u{116ad}'</span>, NSM), (<span class="string">'\u{116ae}'</span>,
    <span class="string">'\u{116af}'</span>, L), (<span class="string">'\u{116b0}'</span>, <span class="string">'\u{116b5}'</span>, NSM), (<span class="string">'\u{116b6}'</span>, <span class="string">'\u{116b6}'</span>, L), (<span class="string">'\u{116b7}'</span>,
    <span class="string">'\u{116b7}'</span>, NSM), (<span class="string">'\u{116b8}'</span>, <span class="string">'\u{116b9}'</span>, L), (<span class="string">'\u{116c0}'</span>, <span class="string">'\u{116c9}'</span>, L), (<span class="string">'\u{11700}'</span>,
    <span class="string">'\u{1171a}'</span>, L), (<span class="string">'\u{1171d}'</span>, <span class="string">'\u{1171f}'</span>, NSM), (<span class="string">'\u{11720}'</span>, <span class="string">'\u{11721}'</span>, L), (<span class="string">'\u{11722}'</span>,
    <span class="string">'\u{11725}'</span>, NSM), (<span class="string">'\u{11726}'</span>, <span class="string">'\u{11726}'</span>, L), (<span class="string">'\u{11727}'</span>, <span class="string">'\u{1172b}'</span>, NSM), (<span class="string">'\u{11730}'</span>,
    <span class="string">'\u{11746}'</span>, L), (<span class="string">'\u{11800}'</span>, <span class="string">'\u{1182e}'</span>, L), (<span class="string">'\u{1182f}'</span>, <span class="string">'\u{11837}'</span>, NSM), (<span class="string">'\u{11838}'</span>,
    <span class="string">'\u{11838}'</span>, L), (<span class="string">'\u{11839}'</span>, <span class="string">'\u{1183a}'</span>, NSM), (<span class="string">'\u{1183b}'</span>, <span class="string">'\u{1183b}'</span>, L), (<span class="string">'\u{118a0}'</span>,
    <span class="string">'\u{118f2}'</span>, L), (<span class="string">'\u{118ff}'</span>, <span class="string">'\u{11906}'</span>, L), (<span class="string">'\u{11909}'</span>, <span class="string">'\u{11909}'</span>, L), (<span class="string">'\u{1190c}'</span>,
    <span class="string">'\u{11913}'</span>, L), (<span class="string">'\u{11915}'</span>, <span class="string">'\u{11916}'</span>, L), (<span class="string">'\u{11918}'</span>, <span class="string">'\u{11935}'</span>, L), (<span class="string">'\u{11937}'</span>,
    <span class="string">'\u{11938}'</span>, L), (<span class="string">'\u{1193b}'</span>, <span class="string">'\u{1193c}'</span>, NSM), (<span class="string">'\u{1193d}'</span>, <span class="string">'\u{1193d}'</span>, L), (<span class="string">'\u{1193e}'</span>,
    <span class="string">'\u{1193e}'</span>, NSM), (<span class="string">'\u{1193f}'</span>, <span class="string">'\u{11942}'</span>, L), (<span class="string">'\u{11943}'</span>, <span class="string">'\u{11943}'</span>, NSM), (<span class="string">'\u{11944}'</span>,
    <span class="string">'\u{11946}'</span>, L), (<span class="string">'\u{11950}'</span>, <span class="string">'\u{11959}'</span>, L), (<span class="string">'\u{119a0}'</span>, <span class="string">'\u{119a7}'</span>, L), (<span class="string">'\u{119aa}'</span>,
    <span class="string">'\u{119d3}'</span>, L), (<span class="string">'\u{119d4}'</span>, <span class="string">'\u{119d7}'</span>, NSM), (<span class="string">'\u{119da}'</span>, <span class="string">'\u{119db}'</span>, NSM), (<span class="string">'\u{119dc}'</span>,
    <span class="string">'\u{119df}'</span>, L), (<span class="string">'\u{119e0}'</span>, <span class="string">'\u{119e0}'</span>, NSM), (<span class="string">'\u{119e1}'</span>, <span class="string">'\u{119e4}'</span>, L), (<span class="string">'\u{11a00}'</span>,
    <span class="string">'\u{11a00}'</span>, L), (<span class="string">'\u{11a01}'</span>, <span class="string">'\u{11a06}'</span>, NSM), (<span class="string">'\u{11a07}'</span>, <span class="string">'\u{11a08}'</span>, L), (<span class="string">'\u{11a09}'</span>,
    <span class="string">'\u{11a0a}'</span>, NSM), (<span class="string">'\u{11a0b}'</span>, <span class="string">'\u{11a32}'</span>, L), (<span class="string">'\u{11a33}'</span>, <span class="string">'\u{11a38}'</span>, NSM), (<span class="string">'\u{11a39}'</span>,
    <span class="string">'\u{11a3a}'</span>, L), (<span class="string">'\u{11a3b}'</span>, <span class="string">'\u{11a3e}'</span>, NSM), (<span class="string">'\u{11a3f}'</span>, <span class="string">'\u{11a46}'</span>, L), (<span class="string">'\u{11a47}'</span>,
    <span class="string">'\u{11a47}'</span>, NSM), (<span class="string">'\u{11a50}'</span>, <span class="string">'\u{11a50}'</span>, L), (<span class="string">'\u{11a51}'</span>, <span class="string">'\u{11a56}'</span>, NSM), (<span class="string">'\u{11a57}'</span>,
    <span class="string">'\u{11a58}'</span>, L), (<span class="string">'\u{11a59}'</span>, <span class="string">'\u{11a5b}'</span>, NSM), (<span class="string">'\u{11a5c}'</span>, <span class="string">'\u{11a89}'</span>, L), (<span class="string">'\u{11a8a}'</span>,
    <span class="string">'\u{11a96}'</span>, NSM), (<span class="string">'\u{11a97}'</span>, <span class="string">'\u{11a97}'</span>, L), (<span class="string">'\u{11a98}'</span>, <span class="string">'\u{11a99}'</span>, NSM), (<span class="string">'\u{11a9a}'</span>,
    <span class="string">'\u{11aa2}'</span>, L), (<span class="string">'\u{11ab0}'</span>, <span class="string">'\u{11af8}'</span>, L), (<span class="string">'\u{11b00}'</span>, <span class="string">'\u{11b09}'</span>, L), (<span class="string">'\u{11c00}'</span>,
    <span class="string">'\u{11c08}'</span>, L), (<span class="string">'\u{11c0a}'</span>, <span class="string">'\u{11c2f}'</span>, L), (<span class="string">'\u{11c30}'</span>, <span class="string">'\u{11c36}'</span>, NSM), (<span class="string">'\u{11c38}'</span>,
    <span class="string">'\u{11c3d}'</span>, NSM), (<span class="string">'\u{11c3e}'</span>, <span class="string">'\u{11c45}'</span>, L), (<span class="string">'\u{11c50}'</span>, <span class="string">'\u{11c6c}'</span>, L), (<span class="string">'\u{11c70}'</span>,
    <span class="string">'\u{11c8f}'</span>, L), (<span class="string">'\u{11c92}'</span>, <span class="string">'\u{11ca7}'</span>, NSM), (<span class="string">'\u{11ca9}'</span>, <span class="string">'\u{11ca9}'</span>, L), (<span class="string">'\u{11caa}'</span>,
    <span class="string">'\u{11cb0}'</span>, NSM), (<span class="string">'\u{11cb1}'</span>, <span class="string">'\u{11cb1}'</span>, L), (<span class="string">'\u{11cb2}'</span>, <span class="string">'\u{11cb3}'</span>, NSM), (<span class="string">'\u{11cb4}'</span>,
    <span class="string">'\u{11cb4}'</span>, L), (<span class="string">'\u{11cb5}'</span>, <span class="string">'\u{11cb6}'</span>, NSM), (<span class="string">'\u{11d00}'</span>, <span class="string">'\u{11d06}'</span>, L), (<span class="string">'\u{11d08}'</span>,
    <span class="string">'\u{11d09}'</span>, L), (<span class="string">'\u{11d0b}'</span>, <span class="string">'\u{11d30}'</span>, L), (<span class="string">'\u{11d31}'</span>, <span class="string">'\u{11d36}'</span>, NSM), (<span class="string">'\u{11d3a}'</span>,
    <span class="string">'\u{11d3a}'</span>, NSM), (<span class="string">'\u{11d3c}'</span>, <span class="string">'\u{11d3d}'</span>, NSM), (<span class="string">'\u{11d3f}'</span>, <span class="string">'\u{11d45}'</span>, NSM),
    (<span class="string">'\u{11d46}'</span>, <span class="string">'\u{11d46}'</span>, L), (<span class="string">'\u{11d47}'</span>, <span class="string">'\u{11d47}'</span>, NSM), (<span class="string">'\u{11d50}'</span>, <span class="string">'\u{11d59}'</span>, L),
    (<span class="string">'\u{11d60}'</span>, <span class="string">'\u{11d65}'</span>, L), (<span class="string">'\u{11d67}'</span>, <span class="string">'\u{11d68}'</span>, L), (<span class="string">'\u{11d6a}'</span>, <span class="string">'\u{11d8e}'</span>, L),
    (<span class="string">'\u{11d90}'</span>, <span class="string">'\u{11d91}'</span>, NSM), (<span class="string">'\u{11d93}'</span>, <span class="string">'\u{11d94}'</span>, L), (<span class="string">'\u{11d95}'</span>, <span class="string">'\u{11d95}'</span>, NSM),
    (<span class="string">'\u{11d96}'</span>, <span class="string">'\u{11d96}'</span>, L), (<span class="string">'\u{11d97}'</span>, <span class="string">'\u{11d97}'</span>, NSM), (<span class="string">'\u{11d98}'</span>, <span class="string">'\u{11d98}'</span>, L),
    (<span class="string">'\u{11da0}'</span>, <span class="string">'\u{11da9}'</span>, L), (<span class="string">'\u{11ee0}'</span>, <span class="string">'\u{11ef2}'</span>, L), (<span class="string">'\u{11ef3}'</span>, <span class="string">'\u{11ef4}'</span>, NSM),
    (<span class="string">'\u{11ef5}'</span>, <span class="string">'\u{11ef8}'</span>, L), (<span class="string">'\u{11f00}'</span>, <span class="string">'\u{11f01}'</span>, NSM), (<span class="string">'\u{11f02}'</span>, <span class="string">'\u{11f10}'</span>, L),
    (<span class="string">'\u{11f12}'</span>, <span class="string">'\u{11f35}'</span>, L), (<span class="string">'\u{11f36}'</span>, <span class="string">'\u{11f3a}'</span>, NSM), (<span class="string">'\u{11f3e}'</span>, <span class="string">'\u{11f3f}'</span>, L),
    (<span class="string">'\u{11f40}'</span>, <span class="string">'\u{11f40}'</span>, NSM), (<span class="string">'\u{11f41}'</span>, <span class="string">'\u{11f41}'</span>, L), (<span class="string">'\u{11f42}'</span>, <span class="string">'\u{11f42}'</span>, NSM),
    (<span class="string">'\u{11f43}'</span>, <span class="string">'\u{11f59}'</span>, L), (<span class="string">'\u{11fb0}'</span>, <span class="string">'\u{11fb0}'</span>, L), (<span class="string">'\u{11fc0}'</span>, <span class="string">'\u{11fd4}'</span>, L),
    (<span class="string">'\u{11fd5}'</span>, <span class="string">'\u{11fdc}'</span>, ON), (<span class="string">'\u{11fdd}'</span>, <span class="string">'\u{11fe0}'</span>, ET), (<span class="string">'\u{11fe1}'</span>, <span class="string">'\u{11ff1}'</span>, ON),
    (<span class="string">'\u{11fff}'</span>, <span class="string">'\u{12399}'</span>, L), (<span class="string">'\u{12400}'</span>, <span class="string">'\u{1246e}'</span>, L), (<span class="string">'\u{12470}'</span>, <span class="string">'\u{12474}'</span>, L),
    (<span class="string">'\u{12480}'</span>, <span class="string">'\u{12543}'</span>, L), (<span class="string">'\u{12f90}'</span>, <span class="string">'\u{12ff2}'</span>, L), (<span class="string">'\u{13000}'</span>, <span class="string">'\u{1343f}'</span>, L),
    (<span class="string">'\u{13440}'</span>, <span class="string">'\u{13440}'</span>, NSM), (<span class="string">'\u{13441}'</span>, <span class="string">'\u{13446}'</span>, L), (<span class="string">'\u{13447}'</span>, <span class="string">'\u{13455}'</span>, NSM),
    (<span class="string">'\u{14400}'</span>, <span class="string">'\u{14646}'</span>, L), (<span class="string">'\u{16800}'</span>, <span class="string">'\u{16a38}'</span>, L), (<span class="string">'\u{16a40}'</span>, <span class="string">'\u{16a5e}'</span>, L),
    (<span class="string">'\u{16a60}'</span>, <span class="string">'\u{16a69}'</span>, L), (<span class="string">'\u{16a6e}'</span>, <span class="string">'\u{16abe}'</span>, L), (<span class="string">'\u{16ac0}'</span>, <span class="string">'\u{16ac9}'</span>, L),
    (<span class="string">'\u{16ad0}'</span>, <span class="string">'\u{16aed}'</span>, L), (<span class="string">'\u{16af0}'</span>, <span class="string">'\u{16af4}'</span>, NSM), (<span class="string">'\u{16af5}'</span>, <span class="string">'\u{16af5}'</span>, L),
    (<span class="string">'\u{16b00}'</span>, <span class="string">'\u{16b2f}'</span>, L), (<span class="string">'\u{16b30}'</span>, <span class="string">'\u{16b36}'</span>, NSM), (<span class="string">'\u{16b37}'</span>, <span class="string">'\u{16b45}'</span>, L),
    (<span class="string">'\u{16b50}'</span>, <span class="string">'\u{16b59}'</span>, L), (<span class="string">'\u{16b5b}'</span>, <span class="string">'\u{16b61}'</span>, L), (<span class="string">'\u{16b63}'</span>, <span class="string">'\u{16b77}'</span>, L),
    (<span class="string">'\u{16b7d}'</span>, <span class="string">'\u{16b8f}'</span>, L), (<span class="string">'\u{16e40}'</span>, <span class="string">'\u{16e9a}'</span>, L), (<span class="string">'\u{16f00}'</span>, <span class="string">'\u{16f4a}'</span>, L),
    (<span class="string">'\u{16f4f}'</span>, <span class="string">'\u{16f4f}'</span>, NSM), (<span class="string">'\u{16f50}'</span>, <span class="string">'\u{16f87}'</span>, L), (<span class="string">'\u{16f8f}'</span>, <span class="string">'\u{16f92}'</span>, NSM),
    (<span class="string">'\u{16f93}'</span>, <span class="string">'\u{16f9f}'</span>, L), (<span class="string">'\u{16fe0}'</span>, <span class="string">'\u{16fe1}'</span>, L), (<span class="string">'\u{16fe2}'</span>, <span class="string">'\u{16fe2}'</span>, ON),
    (<span class="string">'\u{16fe3}'</span>, <span class="string">'\u{16fe3}'</span>, L), (<span class="string">'\u{16fe4}'</span>, <span class="string">'\u{16fe4}'</span>, NSM), (<span class="string">'\u{16ff0}'</span>, <span class="string">'\u{16ff1}'</span>, L),
    (<span class="string">'\u{17000}'</span>, <span class="string">'\u{187f7}'</span>, L), (<span class="string">'\u{18800}'</span>, <span class="string">'\u{18cd5}'</span>, L), (<span class="string">'\u{18d00}'</span>, <span class="string">'\u{18d08}'</span>, L),
    (<span class="string">'\u{1aff0}'</span>, <span class="string">'\u{1aff3}'</span>, L), (<span class="string">'\u{1aff5}'</span>, <span class="string">'\u{1affb}'</span>, L), (<span class="string">'\u{1affd}'</span>, <span class="string">'\u{1affe}'</span>, L),
    (<span class="string">'\u{1b000}'</span>, <span class="string">'\u{1b122}'</span>, L), (<span class="string">'\u{1b132}'</span>, <span class="string">'\u{1b132}'</span>, L), (<span class="string">'\u{1b150}'</span>, <span class="string">'\u{1b152}'</span>, L),
    (<span class="string">'\u{1b155}'</span>, <span class="string">'\u{1b155}'</span>, L), (<span class="string">'\u{1b164}'</span>, <span class="string">'\u{1b167}'</span>, L), (<span class="string">'\u{1b170}'</span>, <span class="string">'\u{1b2fb}'</span>, L),
    (<span class="string">'\u{1bc00}'</span>, <span class="string">'\u{1bc6a}'</span>, L), (<span class="string">'\u{1bc70}'</span>, <span class="string">'\u{1bc7c}'</span>, L), (<span class="string">'\u{1bc80}'</span>, <span class="string">'\u{1bc88}'</span>, L),
    (<span class="string">'\u{1bc90}'</span>, <span class="string">'\u{1bc99}'</span>, L), (<span class="string">'\u{1bc9c}'</span>, <span class="string">'\u{1bc9c}'</span>, L), (<span class="string">'\u{1bc9d}'</span>, <span class="string">'\u{1bc9e}'</span>, NSM),
    (<span class="string">'\u{1bc9f}'</span>, <span class="string">'\u{1bc9f}'</span>, L), (<span class="string">'\u{1bca0}'</span>, <span class="string">'\u{1bca3}'</span>, BN), (<span class="string">'\u{1cf00}'</span>, <span class="string">'\u{1cf2d}'</span>, NSM),
    (<span class="string">'\u{1cf30}'</span>, <span class="string">'\u{1cf46}'</span>, NSM), (<span class="string">'\u{1cf50}'</span>, <span class="string">'\u{1cfc3}'</span>, L), (<span class="string">'\u{1d000}'</span>, <span class="string">'\u{1d0f5}'</span>, L),
    (<span class="string">'\u{1d100}'</span>, <span class="string">'\u{1d126}'</span>, L), (<span class="string">'\u{1d129}'</span>, <span class="string">'\u{1d166}'</span>, L), (<span class="string">'\u{1d167}'</span>, <span class="string">'\u{1d169}'</span>, NSM),
    (<span class="string">'\u{1d16a}'</span>, <span class="string">'\u{1d172}'</span>, L), (<span class="string">'\u{1d173}'</span>, <span class="string">'\u{1d17a}'</span>, BN), (<span class="string">'\u{1d17b}'</span>, <span class="string">'\u{1d182}'</span>, NSM),
    (<span class="string">'\u{1d183}'</span>, <span class="string">'\u{1d184}'</span>, L), (<span class="string">'\u{1d185}'</span>, <span class="string">'\u{1d18b}'</span>, NSM), (<span class="string">'\u{1d18c}'</span>, <span class="string">'\u{1d1a9}'</span>, L),
    (<span class="string">'\u{1d1aa}'</span>, <span class="string">'\u{1d1ad}'</span>, NSM), (<span class="string">'\u{1d1ae}'</span>, <span class="string">'\u{1d1e8}'</span>, L), (<span class="string">'\u{1d1e9}'</span>, <span class="string">'\u{1d1ea}'</span>, ON),
    (<span class="string">'\u{1d200}'</span>, <span class="string">'\u{1d241}'</span>, ON), (<span class="string">'\u{1d242}'</span>, <span class="string">'\u{1d244}'</span>, NSM), (<span class="string">'\u{1d245}'</span>, <span class="string">'\u{1d245}'</span>, ON),
    (<span class="string">'\u{1d2c0}'</span>, <span class="string">'\u{1d2d3}'</span>, L), (<span class="string">'\u{1d2e0}'</span>, <span class="string">'\u{1d2f3}'</span>, L), (<span class="string">'\u{1d300}'</span>, <span class="string">'\u{1d356}'</span>, ON),
    (<span class="string">'\u{1d360}'</span>, <span class="string">'\u{1d378}'</span>, L), (<span class="string">'\u{1d400}'</span>, <span class="string">'\u{1d454}'</span>, L), (<span class="string">'\u{1d456}'</span>, <span class="string">'\u{1d49c}'</span>, L),
    (<span class="string">'\u{1d49e}'</span>, <span class="string">'\u{1d49f}'</span>, L), (<span class="string">'\u{1d4a2}'</span>, <span class="string">'\u{1d4a2}'</span>, L), (<span class="string">'\u{1d4a5}'</span>, <span class="string">'\u{1d4a6}'</span>, L),
    (<span class="string">'\u{1d4a9}'</span>, <span class="string">'\u{1d4ac}'</span>, L), (<span class="string">'\u{1d4ae}'</span>, <span class="string">'\u{1d4b9}'</span>, L), (<span class="string">'\u{1d4bb}'</span>, <span class="string">'\u{1d4bb}'</span>, L),
    (<span class="string">'\u{1d4bd}'</span>, <span class="string">'\u{1d4c3}'</span>, L), (<span class="string">'\u{1d4c5}'</span>, <span class="string">'\u{1d505}'</span>, L), (<span class="string">'\u{1d507}'</span>, <span class="string">'\u{1d50a}'</span>, L),
    (<span class="string">'\u{1d50d}'</span>, <span class="string">'\u{1d514}'</span>, L), (<span class="string">'\u{1d516}'</span>, <span class="string">'\u{1d51c}'</span>, L), (<span class="string">'\u{1d51e}'</span>, <span class="string">'\u{1d539}'</span>, L),
    (<span class="string">'\u{1d53b}'</span>, <span class="string">'\u{1d53e}'</span>, L), (<span class="string">'\u{1d540}'</span>, <span class="string">'\u{1d544}'</span>, L), (<span class="string">'\u{1d546}'</span>, <span class="string">'\u{1d546}'</span>, L),
    (<span class="string">'\u{1d54a}'</span>, <span class="string">'\u{1d550}'</span>, L), (<span class="string">'\u{1d552}'</span>, <span class="string">'\u{1d6a5}'</span>, L), (<span class="string">'\u{1d6a8}'</span>, <span class="string">'\u{1d6da}'</span>, L),
    (<span class="string">'\u{1d6db}'</span>, <span class="string">'\u{1d6db}'</span>, ON), (<span class="string">'\u{1d6dc}'</span>, <span class="string">'\u{1d714}'</span>, L), (<span class="string">'\u{1d715}'</span>, <span class="string">'\u{1d715}'</span>, ON),
    (<span class="string">'\u{1d716}'</span>, <span class="string">'\u{1d74e}'</span>, L), (<span class="string">'\u{1d74f}'</span>, <span class="string">'\u{1d74f}'</span>, ON), (<span class="string">'\u{1d750}'</span>, <span class="string">'\u{1d788}'</span>, L),
    (<span class="string">'\u{1d789}'</span>, <span class="string">'\u{1d789}'</span>, ON), (<span class="string">'\u{1d78a}'</span>, <span class="string">'\u{1d7c2}'</span>, L), (<span class="string">'\u{1d7c3}'</span>, <span class="string">'\u{1d7c3}'</span>, ON),
    (<span class="string">'\u{1d7c4}'</span>, <span class="string">'\u{1d7cb}'</span>, L), (<span class="string">'\u{1d7ce}'</span>, <span class="string">'\u{1d7ff}'</span>, EN), (<span class="string">'\u{1d800}'</span>, <span class="string">'\u{1d9ff}'</span>, L),
    (<span class="string">'\u{1da00}'</span>, <span class="string">'\u{1da36}'</span>, NSM), (<span class="string">'\u{1da37}'</span>, <span class="string">'\u{1da3a}'</span>, L), (<span class="string">'\u{1da3b}'</span>, <span class="string">'\u{1da6c}'</span>, NSM),
    (<span class="string">'\u{1da6d}'</span>, <span class="string">'\u{1da74}'</span>, L), (<span class="string">'\u{1da75}'</span>, <span class="string">'\u{1da75}'</span>, NSM), (<span class="string">'\u{1da76}'</span>, <span class="string">'\u{1da83}'</span>, L),
    (<span class="string">'\u{1da84}'</span>, <span class="string">'\u{1da84}'</span>, NSM), (<span class="string">'\u{1da85}'</span>, <span class="string">'\u{1da8b}'</span>, L), (<span class="string">'\u{1da9b}'</span>, <span class="string">'\u{1da9f}'</span>, NSM),
    (<span class="string">'\u{1daa1}'</span>, <span class="string">'\u{1daaf}'</span>, NSM), (<span class="string">'\u{1df00}'</span>, <span class="string">'\u{1df1e}'</span>, L), (<span class="string">'\u{1df25}'</span>, <span class="string">'\u{1df2a}'</span>, L),
    (<span class="string">'\u{1e000}'</span>, <span class="string">'\u{1e006}'</span>, NSM), (<span class="string">'\u{1e008}'</span>, <span class="string">'\u{1e018}'</span>, NSM), (<span class="string">'\u{1e01b}'</span>, <span class="string">'\u{1e021}'</span>,
    NSM), (<span class="string">'\u{1e023}'</span>, <span class="string">'\u{1e024}'</span>, NSM), (<span class="string">'\u{1e026}'</span>, <span class="string">'\u{1e02a}'</span>, NSM), (<span class="string">'\u{1e030}'</span>,
    <span class="string">'\u{1e06d}'</span>, L), (<span class="string">'\u{1e08f}'</span>, <span class="string">'\u{1e08f}'</span>, NSM), (<span class="string">'\u{1e100}'</span>, <span class="string">'\u{1e12c}'</span>, L), (<span class="string">'\u{1e130}'</span>,
    <span class="string">'\u{1e136}'</span>, NSM), (<span class="string">'\u{1e137}'</span>, <span class="string">'\u{1e13d}'</span>, L), (<span class="string">'\u{1e140}'</span>, <span class="string">'\u{1e149}'</span>, L), (<span class="string">'\u{1e14e}'</span>,
    <span class="string">'\u{1e14f}'</span>, L), (<span class="string">'\u{1e290}'</span>, <span class="string">'\u{1e2ad}'</span>, L), (<span class="string">'\u{1e2ae}'</span>, <span class="string">'\u{1e2ae}'</span>, NSM), (<span class="string">'\u{1e2c0}'</span>,
    <span class="string">'\u{1e2eb}'</span>, L), (<span class="string">'\u{1e2ec}'</span>, <span class="string">'\u{1e2ef}'</span>, NSM), (<span class="string">'\u{1e2f0}'</span>, <span class="string">'\u{1e2f9}'</span>, L), (<span class="string">'\u{1e2ff}'</span>,
    <span class="string">'\u{1e2ff}'</span>, ET), (<span class="string">'\u{1e4d0}'</span>, <span class="string">'\u{1e4eb}'</span>, L), (<span class="string">'\u{1e4ec}'</span>, <span class="string">'\u{1e4ef}'</span>, NSM), (<span class="string">'\u{1e4f0}'</span>,
    <span class="string">'\u{1e4f9}'</span>, L), (<span class="string">'\u{1e7e0}'</span>, <span class="string">'\u{1e7e6}'</span>, L), (<span class="string">'\u{1e7e8}'</span>, <span class="string">'\u{1e7eb}'</span>, L), (<span class="string">'\u{1e7ed}'</span>,
    <span class="string">'\u{1e7ee}'</span>, L), (<span class="string">'\u{1e7f0}'</span>, <span class="string">'\u{1e7fe}'</span>, L), (<span class="string">'\u{1e800}'</span>, <span class="string">'\u{1e8cf}'</span>, R), (<span class="string">'\u{1e8d0}'</span>,
    <span class="string">'\u{1e8d6}'</span>, NSM), (<span class="string">'\u{1e8d7}'</span>, <span class="string">'\u{1e943}'</span>, R), (<span class="string">'\u{1e944}'</span>, <span class="string">'\u{1e94a}'</span>, NSM), (<span class="string">'\u{1e94b}'</span>,
    <span class="string">'\u{1ec70}'</span>, R), (<span class="string">'\u{1ec71}'</span>, <span class="string">'\u{1ecb4}'</span>, AL), (<span class="string">'\u{1ecb5}'</span>, <span class="string">'\u{1ed00}'</span>, R), (<span class="string">'\u{1ed01}'</span>,
    <span class="string">'\u{1ed3d}'</span>, AL), (<span class="string">'\u{1ed3e}'</span>, <span class="string">'\u{1edff}'</span>, R), (<span class="string">'\u{1ee00}'</span>, <span class="string">'\u{1eeef}'</span>, AL), (<span class="string">'\u{1eef0}'</span>,
    <span class="string">'\u{1eef1}'</span>, ON), (<span class="string">'\u{1eef2}'</span>, <span class="string">'\u{1eeff}'</span>, AL), (<span class="string">'\u{1ef00}'</span>, <span class="string">'\u{1efff}'</span>, R), (<span class="string">'\u{1f000}'</span>,
    <span class="string">'\u{1f02b}'</span>, ON), (<span class="string">'\u{1f030}'</span>, <span class="string">'\u{1f093}'</span>, ON), (<span class="string">'\u{1f0a0}'</span>, <span class="string">'\u{1f0ae}'</span>, ON), (<span class="string">'\u{1f0b1}'</span>,
    <span class="string">'\u{1f0bf}'</span>, ON), (<span class="string">'\u{1f0c1}'</span>, <span class="string">'\u{1f0cf}'</span>, ON), (<span class="string">'\u{1f0d1}'</span>, <span class="string">'\u{1f0f5}'</span>, ON), (<span class="string">'\u{1f100}'</span>,
    <span class="string">'\u{1f10a}'</span>, EN), (<span class="string">'\u{1f10b}'</span>, <span class="string">'\u{1f10f}'</span>, ON), (<span class="string">'\u{1f110}'</span>, <span class="string">'\u{1f12e}'</span>, L), (<span class="string">'\u{1f12f}'</span>,
    <span class="string">'\u{1f12f}'</span>, ON), (<span class="string">'\u{1f130}'</span>, <span class="string">'\u{1f169}'</span>, L), (<span class="string">'\u{1f16a}'</span>, <span class="string">'\u{1f16f}'</span>, ON), (<span class="string">'\u{1f170}'</span>,
    <span class="string">'\u{1f1ac}'</span>, L), (<span class="string">'\u{1f1ad}'</span>, <span class="string">'\u{1f1ad}'</span>, ON), (<span class="string">'\u{1f1e6}'</span>, <span class="string">'\u{1f202}'</span>, L), (<span class="string">'\u{1f210}'</span>,
    <span class="string">'\u{1f23b}'</span>, L), (<span class="string">'\u{1f240}'</span>, <span class="string">'\u{1f248}'</span>, L), (<span class="string">'\u{1f250}'</span>, <span class="string">'\u{1f251}'</span>, L), (<span class="string">'\u{1f260}'</span>,
    <span class="string">'\u{1f265}'</span>, ON), (<span class="string">'\u{1f300}'</span>, <span class="string">'\u{1f6d7}'</span>, ON), (<span class="string">'\u{1f6dc}'</span>, <span class="string">'\u{1f6ec}'</span>, ON), (<span class="string">'\u{1f6f0}'</span>,
    <span class="string">'\u{1f6fc}'</span>, ON), (<span class="string">'\u{1f700}'</span>, <span class="string">'\u{1f776}'</span>, ON), (<span class="string">'\u{1f77b}'</span>, <span class="string">'\u{1f7d9}'</span>, ON), (<span class="string">'\u{1f7e0}'</span>,
    <span class="string">'\u{1f7eb}'</span>, ON), (<span class="string">'\u{1f7f0}'</span>, <span class="string">'\u{1f7f0}'</span>, ON), (<span class="string">'\u{1f800}'</span>, <span class="string">'\u{1f80b}'</span>, ON), (<span class="string">'\u{1f810}'</span>,
    <span class="string">'\u{1f847}'</span>, ON), (<span class="string">'\u{1f850}'</span>, <span class="string">'\u{1f859}'</span>, ON), (<span class="string">'\u{1f860}'</span>, <span class="string">'\u{1f887}'</span>, ON), (<span class="string">'\u{1f890}'</span>,
    <span class="string">'\u{1f8ad}'</span>, ON), (<span class="string">'\u{1f8b0}'</span>, <span class="string">'\u{1f8b1}'</span>, ON), (<span class="string">'\u{1f900}'</span>, <span class="string">'\u{1fa53}'</span>, ON), (<span class="string">'\u{1fa60}'</span>,
    <span class="string">'\u{1fa6d}'</span>, ON), (<span class="string">'\u{1fa70}'</span>, <span class="string">'\u{1fa7c}'</span>, ON), (<span class="string">'\u{1fa80}'</span>, <span class="string">'\u{1fa88}'</span>, ON), (<span class="string">'\u{1fa90}'</span>,
    <span class="string">'\u{1fabd}'</span>, ON), (<span class="string">'\u{1fabf}'</span>, <span class="string">'\u{1fac5}'</span>, ON), (<span class="string">'\u{1face}'</span>, <span class="string">'\u{1fadb}'</span>, ON), (<span class="string">'\u{1fae0}'</span>,
    <span class="string">'\u{1fae8}'</span>, ON), (<span class="string">'\u{1faf0}'</span>, <span class="string">'\u{1faf8}'</span>, ON), (<span class="string">'\u{1fb00}'</span>, <span class="string">'\u{1fb92}'</span>, ON), (<span class="string">'\u{1fb94}'</span>,
    <span class="string">'\u{1fbca}'</span>, ON), (<span class="string">'\u{1fbf0}'</span>, <span class="string">'\u{1fbf9}'</span>, EN), (<span class="string">'\u{20000}'</span>, <span class="string">'\u{2a6df}'</span>, L), (<span class="string">'\u{2a700}'</span>,
    <span class="string">'\u{2b739}'</span>, L), (<span class="string">'\u{2b740}'</span>, <span class="string">'\u{2b81d}'</span>, L), (<span class="string">'\u{2b820}'</span>, <span class="string">'\u{2cea1}'</span>, L), (<span class="string">'\u{2ceb0}'</span>,
    <span class="string">'\u{2ebe0}'</span>, L), (<span class="string">'\u{2f800}'</span>, <span class="string">'\u{2fa1d}'</span>, L), (<span class="string">'\u{30000}'</span>, <span class="string">'\u{3134a}'</span>, L), (<span class="string">'\u{31350}'</span>,
    <span class="string">'\u{323af}'</span>, L), (<span class="string">'\u{e0001}'</span>, <span class="string">'\u{e0001}'</span>, BN), (<span class="string">'\u{e0020}'</span>, <span class="string">'\u{e007f}'</span>, BN), (<span class="string">'\u{e0100}'</span>,
    <span class="string">'\u{e01ef}'</span>, NSM), (<span class="string">'\u{f0000}'</span>, <span class="string">'\u{ffffd}'</span>, L), (<span class="string">'\u{100000}'</span>, <span class="string">'\u{10fffd}'</span>, L)
];

<span class="kw">pub const </span>bidi_pairs_table: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char, <span class="prelude-ty">Option</span>&lt;char&gt;)] = <span class="kw-2">&amp;</span>[
    (<span class="string">'\u{28}'</span>, <span class="string">'\u{29}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{5b}'</span>, <span class="string">'\u{5d}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{7b}'</span>, <span class="string">'\u{7d}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{f3a}'</span>,
    <span class="string">'\u{f3b}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{f3c}'</span>, <span class="string">'\u{f3d}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{169b}'</span>, <span class="string">'\u{169c}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2045}'</span>,
    <span class="string">'\u{2046}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{207d}'</span>, <span class="string">'\u{207e}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{208d}'</span>, <span class="string">'\u{208e}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2308}'</span>,
    <span class="string">'\u{2309}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{230a}'</span>, <span class="string">'\u{230b}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2329}'</span>, <span class="string">'\u{232a}'</span>, <span class="prelude-val">Some</span>(<span class="string">'\u{3008}'</span>)),
    (<span class="string">'\u{2768}'</span>, <span class="string">'\u{2769}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{276a}'</span>, <span class="string">'\u{276b}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{276c}'</span>, <span class="string">'\u{276d}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{276e}'</span>, <span class="string">'\u{276f}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2770}'</span>, <span class="string">'\u{2771}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2772}'</span>, <span class="string">'\u{2773}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{2774}'</span>, <span class="string">'\u{2775}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{27c5}'</span>, <span class="string">'\u{27c6}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{27e6}'</span>, <span class="string">'\u{27e7}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{27e8}'</span>, <span class="string">'\u{27e9}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{27ea}'</span>, <span class="string">'\u{27eb}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{27ec}'</span>, <span class="string">'\u{27ed}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{27ee}'</span>, <span class="string">'\u{27ef}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2983}'</span>, <span class="string">'\u{2984}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2985}'</span>, <span class="string">'\u{2986}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{2987}'</span>, <span class="string">'\u{2988}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2989}'</span>, <span class="string">'\u{298a}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{298b}'</span>, <span class="string">'\u{298c}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{298d}'</span>, <span class="string">'\u{2990}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{298f}'</span>, <span class="string">'\u{298e}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2991}'</span>, <span class="string">'\u{2992}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{2993}'</span>, <span class="string">'\u{2994}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2995}'</span>, <span class="string">'\u{2996}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2997}'</span>, <span class="string">'\u{2998}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{29d8}'</span>, <span class="string">'\u{29d9}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{29da}'</span>, <span class="string">'\u{29db}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{29fc}'</span>, <span class="string">'\u{29fd}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{2e22}'</span>, <span class="string">'\u{2e23}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2e24}'</span>, <span class="string">'\u{2e25}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2e26}'</span>, <span class="string">'\u{2e27}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{2e28}'</span>, <span class="string">'\u{2e29}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2e55}'</span>, <span class="string">'\u{2e56}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2e57}'</span>, <span class="string">'\u{2e58}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{2e59}'</span>, <span class="string">'\u{2e5a}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{2e5b}'</span>, <span class="string">'\u{2e5c}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{3008}'</span>, <span class="string">'\u{3009}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{300a}'</span>, <span class="string">'\u{300b}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{300c}'</span>, <span class="string">'\u{300d}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{300e}'</span>, <span class="string">'\u{300f}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{3010}'</span>, <span class="string">'\u{3011}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{3014}'</span>, <span class="string">'\u{3015}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{3016}'</span>, <span class="string">'\u{3017}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{3018}'</span>, <span class="string">'\u{3019}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{301a}'</span>, <span class="string">'\u{301b}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{fe59}'</span>, <span class="string">'\u{fe5a}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{fe5b}'</span>, <span class="string">'\u{fe5c}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{fe5d}'</span>, <span class="string">'\u{fe5e}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{ff08}'</span>, <span class="string">'\u{ff09}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{ff3b}'</span>, <span class="string">'\u{ff3d}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{ff5b}'</span>, <span class="string">'\u{ff5d}'</span>, <span class="prelude-val">None</span>), (<span class="string">'\u{ff5f}'</span>, <span class="string">'\u{ff60}'</span>, <span class="prelude-val">None</span>),
    (<span class="string">'\u{ff62}'</span>, <span class="string">'\u{ff63}'</span>, <span class="prelude-val">None</span>)
];

</code></pre></div></section></main></body></html>