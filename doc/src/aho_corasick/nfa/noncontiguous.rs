<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/aho-corasick-1.1.3/src/nfa/noncontiguous.rs`."><title>noncontiguous.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="aho_corasick" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../aho_corasick/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">/*!
Provides a noncontiguous NFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a noncontiguous NFA directly. Using an `NFA` directly is typically
only necessary when one needs access to the [`Automaton`] trait implementation.
*/

</span><span class="kw">use </span>alloc::{
    collections::{BTreeSet, VecDeque},
    vec,
    vec::Vec,
};

<span class="kw">use crate</span>::{
    automaton::Automaton,
    util::{
        alphabet::{ByteClassSet, ByteClasses},
        error::{BuildError, MatchError},
        prefilter::{<span class="self">self</span>, opposite_ascii_case, Prefilter},
        primitives::{IteratorIndexExt, PatternID, SmallIndex, StateID},
        remapper::Remapper,
        search::{Anchored, MatchKind},
        special::Special,
    },
};

<span class="doccomment">/// A noncontiguous NFA implementation of Aho-Corasick.
///
/// When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
/// this type directly. Using an `NFA` directly is typically only necessary
/// when one needs access to the [`Automaton`] trait implementation.
///
/// This NFA represents the "core" implementation of Aho-Corasick in this
/// crate. Namely, constructing this NFA involving building a trie and then
/// filling in the failure transitions between states, similar to what is
/// described in any standard textbook description of Aho-Corasick.
///
/// In order to minimize heap usage and to avoid additional construction costs,
/// this implementation represents the transitions of all states as distinct
/// sparse memory allocations. This is where it gets its name from. That is,
/// this NFA has no contiguous memory allocation for its transition table. Each
/// state gets its own allocation.
///
/// While the sparse representation keeps memory usage to somewhat reasonable
/// levels, it is still quite large and also results in somewhat mediocre
/// search performance. For this reason, it is almost always a good idea to
/// use a [`contiguous::NFA`](crate::nfa::contiguous::NFA) instead. It is
/// marginally slower to build, but has higher throughput and can sometimes use
/// an order of magnitude less memory. The main reason to use a noncontiguous
/// NFA is when you need the fastest possible construction time, or when a
/// contiguous NFA does not have the desired capacity. (The total number of NFA
/// states it can have is fewer than a noncontiguous NFA.)
///
/// # Example
///
/// This example shows how to build an `NFA` directly and use it to execute
/// [`Automaton::try_find`]:
///
/// ```
/// use aho_corasick::{
///     automaton::Automaton,
///     nfa::noncontiguous::NFA,
///     Input, Match,
/// };
///
/// let patterns = &amp;["b", "abc", "abcd"];
/// let haystack = "abcd";
///
/// let nfa = NFA::new(patterns).unwrap();
/// assert_eq!(
///     Some(Match::must(0, 1..2)),
///     nfa.try_find(&amp;Input::new(haystack))?,
/// );
/// # Ok::&lt;(), Box&lt;dyn std::error::Error&gt;&gt;(())
/// ```
///
/// It is also possible to implement your own version of `try_find`. See the
/// [`Automaton`] documentation for an example.
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub struct </span>NFA {
    <span class="doccomment">/// The match semantics built into this NFA.
    </span>match_kind: MatchKind,
    <span class="doccomment">/// A set of states. Each state defines its own transitions, a fail
    /// transition and a set of indices corresponding to matches.
    ///
    /// The first state is always the fail state, which is used only as a
    /// sentinel. Namely, in the final NFA, no transition into the fail state
    /// exists. (Well, they do, but they aren't followed. Instead, the state's
    /// failure transition is followed.)
    ///
    /// The second state (index 1) is always the dead state. Dead states are
    /// in every automaton, but only used when leftmost-{first,longest} match
    /// semantics are enabled. Specifically, they instruct search to stop
    /// at specific points in order to report the correct match location. In
    /// the standard Aho-Corasick construction, there are no transitions to
    /// the dead state.
    ///
    /// The third state (index 2) is generally intended to be the starting or
    /// "root" state.
    </span>states: Vec&lt;State&gt;,
    <span class="doccomment">/// Transitions stored in a sparse representation via a linked list.
    ///
    /// Each transition contains three pieces of information: the byte it
    /// is defined for, the state it transitions to and a link to the next
    /// transition in the same state (or `StateID::ZERO` if it is the last
    /// transition).
    ///
    /// The first transition for each state is determined by `State::sparse`.
    ///
    /// Note that this contains a complete set of all transitions in this NFA,
    /// including states that have a dense representation for transitions.
    /// (Adding dense transitions for a state doesn't remove its sparse
    /// transitions, since deleting transitions from this particular sparse
    /// representation would be fairly expensive.)
    </span>sparse: Vec&lt;Transition&gt;,
    <span class="doccomment">/// Transitions stored in a dense representation.
    ///
    /// A state has a row in this table if and only if `State::dense` is
    /// not equal to `StateID::ZERO`. When not zero, there are precisely
    /// `NFA::byte_classes::alphabet_len()` entries beginning at `State::dense`
    /// in this table.
    ///
    /// Generally a very small minority of states have a dense representation
    /// since it uses so much memory.
    </span>dense: Vec&lt;StateID&gt;,
    <span class="doccomment">/// Matches stored in linked list for each state.
    ///
    /// Like sparse transitions, each match has a link to the next match in the
    /// state.
    ///
    /// The first match for each state is determined by `State::matches`.
    </span>matches: Vec&lt;Match&gt;,
    <span class="doccomment">/// The length, in bytes, of each pattern in this NFA. This slice is
    /// indexed by `PatternID`.
    ///
    /// The number of entries in this vector corresponds to the total number of
    /// patterns in this automaton.
    </span>pattern_lens: Vec&lt;SmallIndex&gt;,
    <span class="doccomment">/// A prefilter for quickly skipping to candidate matches, if pertinent.
    </span>prefilter: <span class="prelude-ty">Option</span>&lt;Prefilter&gt;,
    <span class="doccomment">/// A set of equivalence classes in terms of bytes. We compute this while
    /// building the NFA, but don't use it in the NFA's states. Instead, we
    /// use this for building the DFA. We store it on the NFA since it's easy
    /// to compute while visiting the patterns.
    </span>byte_classes: ByteClasses,
    <span class="doccomment">/// The length, in bytes, of the shortest pattern in this automaton. This
    /// information is useful for detecting whether an automaton matches the
    /// empty string or not.
    </span>min_pattern_len: usize,
    <span class="doccomment">/// The length, in bytes, of the longest pattern in this automaton. This
    /// information is useful for keeping correct buffer sizes when searching
    /// on streams.
    </span>max_pattern_len: usize,
    <span class="doccomment">/// The information required to deduce which states are "special" in this
    /// NFA.
    ///
    /// Since the DEAD and FAIL states are always the first two states and
    /// there are only ever two start states (which follow all of the match
    /// states), it follows that we can determine whether a state is a fail,
    /// dead, match or start with just a few comparisons on the ID itself:
    ///
    ///    is_dead(sid): sid == NFA::DEAD
    ///    is_fail(sid): sid == NFA::FAIL
    ///   is_match(sid): NFA::FAIL &lt; sid &amp;&amp; sid &lt;= max_match_id
    ///   is_start(sid): sid == start_unanchored_id || sid == start_anchored_id
    ///
    /// Note that this only applies to the NFA after it has been constructed.
    /// During construction, the start states are the first ones added and the
    /// match states are inter-leaved with non-match states. Once all of the
    /// states have been added, the states are shuffled such that the above
    /// predicates hold.
    </span>special: Special,
}

<span class="kw">impl </span>NFA {
    <span class="doccomment">/// Create a new Aho-Corasick noncontiguous NFA using the default
    /// configuration.
    ///
    /// Use a [`Builder`] if you want to change the configuration.
    </span><span class="kw">pub fn </span>new&lt;I, P&gt;(patterns: I) -&gt; <span class="prelude-ty">Result</span>&lt;NFA, BuildError&gt;
    <span class="kw">where
        </span>I: IntoIterator&lt;Item = P&gt;,
        P: AsRef&lt;[u8]&gt;,
    {
        NFA::builder().build(patterns)
    }

    <span class="doccomment">/// A convenience method for returning a new Aho-Corasick noncontiguous NFA
    /// builder.
    ///
    /// This usually permits one to just import the `NFA` type.
    </span><span class="kw">pub fn </span>builder() -&gt; Builder {
        Builder::new()
    }
}

<span class="kw">impl </span>NFA {
    <span class="doccomment">/// The DEAD state is a sentinel state like the FAIL state. The DEAD state
    /// instructs any search to stop and return any currently recorded match,
    /// or no match otherwise. Generally speaking, it is impossible for an
    /// unanchored standard search to enter a DEAD state. But an anchored
    /// search can, and so to can a leftmost search.
    ///
    /// We put DEAD before FAIL so that DEAD is always 0. We repeat this
    /// decision across the other Aho-Corasicm automata, so that DEAD
    /// states there are always 0 too. It's not that we need all of the
    /// implementations to agree, but rather, the contiguous NFA and the DFA
    /// use a sort of "premultiplied" state identifier where the only state
    /// whose ID is always known and constant is the first state. Subsequent
    /// state IDs depend on how much space has already been used in the
    /// transition table.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">const </span>DEAD: StateID = StateID::new_unchecked(<span class="number">0</span>);
    <span class="doccomment">/// The FAIL state mostly just corresponds to the ID of any transition on a
    /// state that isn't explicitly defined. When one transitions into the FAIL
    /// state, one must follow the previous state's failure transition before
    /// doing the next state lookup. In this way, FAIL is more of a sentinel
    /// than a state that one actually transitions into. In particular, it is
    /// never exposed in the `Automaton` interface.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">const </span>FAIL: StateID = StateID::new_unchecked(<span class="number">1</span>);

    <span class="doccomment">/// Returns the equivalence classes of bytes found while constructing
    /// this NFA.
    ///
    /// Note that the NFA doesn't actually make use of these equivalence
    /// classes. Instead, these are useful for building the DFA when desired.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>byte_classes(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>ByteClasses {
        <span class="kw-2">&amp;</span><span class="self">self</span>.byte_classes
    }

    <span class="doccomment">/// Returns a slice containing the length of each pattern in this searcher.
    /// It is indexed by `PatternID` and has length `NFA::patterns_len`.
    ///
    /// This is exposed for convenience when building a contiguous NFA. But it
    /// can be reconstructed from the `Automaton` API if necessary.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>pattern_lens_raw(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>[SmallIndex] {
        <span class="kw-2">&amp;</span><span class="self">self</span>.pattern_lens
    }

    <span class="doccomment">/// Returns a slice of all states in this non-contiguous NFA.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>states(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>[State] {
        <span class="kw-2">&amp;</span><span class="self">self</span>.states
    }

    <span class="doccomment">/// Returns the underlying "special" state information for this NFA.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>special(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>Special {
        <span class="kw-2">&amp;</span><span class="self">self</span>.special
    }

    <span class="doccomment">/// Swaps the states at `id1` and `id2`.
    ///
    /// This does not update the transitions of any state to account for the
    /// state swap.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>swap_states(<span class="kw-2">&amp;mut </span><span class="self">self</span>, id1: StateID, id2: StateID) {
        <span class="self">self</span>.states.swap(id1.as_usize(), id2.as_usize());
    }

    <span class="doccomment">/// Re-maps all state IDs in this NFA according to the `map` function
    /// given.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>remap(<span class="kw-2">&amp;mut </span><span class="self">self</span>, map: <span class="kw">impl </span>Fn(StateID) -&gt; StateID) {
        <span class="kw">let </span>alphabet_len = <span class="self">self</span>.byte_classes.alphabet_len();
        <span class="kw">for </span>state <span class="kw">in </span><span class="self">self</span>.states.iter_mut() {
            state.fail = map(state.fail);
            <span class="kw">let </span><span class="kw-2">mut </span>link = state.sparse;
            <span class="kw">while </span>link != StateID::ZERO {
                <span class="kw">let </span>t = <span class="kw-2">&amp;mut </span><span class="self">self</span>.sparse[link];
                t.next = map(t.next);
                link = t.link;
            }
            <span class="kw">if </span>state.dense != StateID::ZERO {
                <span class="kw">let </span>start = state.dense.as_usize();
                <span class="kw">for </span>next <span class="kw">in </span><span class="self">self</span>.dense[start..][..alphabet_len].iter_mut() {
                    <span class="kw-2">*</span>next = map(<span class="kw-2">*</span>next);
                }
            }
        }
    }

    <span class="doccomment">/// Iterate over all of the transitions for the given state ID.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>iter_trans(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        sid: StateID,
    ) -&gt; <span class="kw">impl </span>Iterator&lt;Item = Transition&gt; + <span class="lifetime">'_ </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>link = <span class="self">self</span>.states[sid].sparse;
        core::iter::from_fn(<span class="kw">move </span>|| {
            <span class="kw">if </span>link == StateID::ZERO {
                <span class="kw">return </span><span class="prelude-val">None</span>;
            }
            <span class="kw">let </span>t = <span class="self">self</span>.sparse[link];
            link = t.link;
            <span class="prelude-val">Some</span>(t)
        })
    }

    <span class="doccomment">/// Iterate over all of the matches for the given state ID.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>iter_matches(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        sid: StateID,
    ) -&gt; <span class="kw">impl </span>Iterator&lt;Item = PatternID&gt; + <span class="lifetime">'_ </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>link = <span class="self">self</span>.states[sid].matches;
        core::iter::from_fn(<span class="kw">move </span>|| {
            <span class="kw">if </span>link == StateID::ZERO {
                <span class="kw">return </span><span class="prelude-val">None</span>;
            }
            <span class="kw">let </span>m = <span class="self">self</span>.matches[link];
            link = m.link;
            <span class="prelude-val">Some</span>(m.pid)
        })
    }

    <span class="doccomment">/// Return the link following the one given. If the one given is the last
    /// link for the given state, then return `None`.
    ///
    /// If no previous link is given, then this returns the first link in the
    /// state, if one exists.
    ///
    /// This is useful for manually iterating over the transitions in a single
    /// state without borrowing the NFA. This permits mutating other parts of
    /// the NFA during iteration. Namely, one can access the transition pointed
    /// to by the link via `self.sparse[link]`.
    </span><span class="kw">fn </span>next_link(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        sid: StateID,
        prev: <span class="prelude-ty">Option</span>&lt;StateID&gt;,
    ) -&gt; <span class="prelude-ty">Option</span>&lt;StateID&gt; {
        <span class="kw">let </span>link =
            prev.map_or(<span class="self">self</span>.states[sid].sparse, |p| <span class="self">self</span>.sparse[p].link);
        <span class="kw">if </span>link == StateID::ZERO {
            <span class="prelude-val">None
        </span>} <span class="kw">else </span>{
            <span class="prelude-val">Some</span>(link)
        }
    }

    <span class="doccomment">/// Follow the transition for the given byte in the given state. If no such
    /// transition exists, then the FAIL state ID is returned.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>follow_transition(<span class="kw-2">&amp;</span><span class="self">self</span>, sid: StateID, byte: u8) -&gt; StateID {
        <span class="kw">let </span>s = <span class="kw-2">&amp;</span><span class="self">self</span>.states[sid];
        <span class="comment">// This is a special case that targets starting states and states
        // near a start state. Namely, after the initial trie is constructed,
        // we look for states close to the start state to convert to a dense
        // representation for their transitions. This winds up using a lot more
        // memory per state in exchange for faster transition lookups. But
        // since we only do this for a small number of states (by default), the
        // memory usage is usually minimal.
        //
        // This has *massive* benefit when executing searches because the
        // unanchored starting state is by far the hottest state and is
        // frequently visited. Moreover, the 'for' loop below that works
        // decently on an actually sparse state is disastrous on a state that
        // is nearly or completely dense.
        </span><span class="kw">if </span>s.dense == StateID::ZERO {
            <span class="self">self</span>.follow_transition_sparse(sid, byte)
        } <span class="kw">else </span>{
            <span class="kw">let </span>class = usize::from(<span class="self">self</span>.byte_classes.get(byte));
            <span class="self">self</span>.dense[s.dense.as_usize() + class]
        }
    }

    <span class="doccomment">/// Like `follow_transition`, but always uses the sparse representation.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>follow_transition_sparse(<span class="kw-2">&amp;</span><span class="self">self</span>, sid: StateID, byte: u8) -&gt; StateID {
        <span class="kw">for </span>t <span class="kw">in </span><span class="self">self</span>.iter_trans(sid) {
            <span class="kw">if </span>byte &lt;= t.byte {
                <span class="kw">if </span>byte == t.byte {
                    <span class="kw">return </span>t.next;
                }
                <span class="kw">break</span>;
            }
        }
        NFA::FAIL
    }

    <span class="doccomment">/// Set the transition for the given byte to the state ID given.
    ///
    /// Note that one should not set transitions to the FAIL state. It is not
    /// technically incorrect, but it wastes space. If a transition is not
    /// defined, then it is automatically assumed to lead to the FAIL state.
    </span><span class="kw">fn </span>add_transition(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        prev: StateID,
        byte: u8,
        next: StateID,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="kw">if </span><span class="self">self</span>.states[prev].dense != StateID::ZERO {
            <span class="kw">let </span>dense = <span class="self">self</span>.states[prev].dense;
            <span class="kw">let </span>class = usize::from(<span class="self">self</span>.byte_classes.get(byte));
            <span class="self">self</span>.dense[dense.as_usize() + class] = next;
        }

        <span class="kw">let </span>head = <span class="self">self</span>.states[prev].sparse;
        <span class="kw">if </span>head == StateID::ZERO || byte &lt; <span class="self">self</span>.sparse[head].byte {
            <span class="kw">let </span>new_link = <span class="self">self</span>.alloc_transition()<span class="question-mark">?</span>;
            <span class="self">self</span>.sparse[new_link] = Transition { byte, next, link: head };
            <span class="self">self</span>.states[prev].sparse = new_link;
            <span class="kw">return </span><span class="prelude-val">Ok</span>(());
        } <span class="kw">else if </span>byte == <span class="self">self</span>.sparse[head].byte {
            <span class="self">self</span>.sparse[head].next = next;
            <span class="kw">return </span><span class="prelude-val">Ok</span>(());
        }

        <span class="comment">// We handled the only cases where the beginning of the transition
        // chain needs to change. At this point, we now know that there is
        // at least one entry in the transition chain and the byte for that
        // transition is less than the byte for the transition we're adding.
        </span><span class="kw">let </span>(<span class="kw-2">mut </span>link_prev, <span class="kw-2">mut </span>link_next) = (head, <span class="self">self</span>.sparse[head].link);
        <span class="kw">while </span>link_next != StateID::ZERO &amp;&amp; byte &gt; <span class="self">self</span>.sparse[link_next].byte
        {
            link_prev = link_next;
            link_next = <span class="self">self</span>.sparse[link_next].link;
        }
        <span class="kw">if </span>link_next == StateID::ZERO || byte &lt; <span class="self">self</span>.sparse[link_next].byte {
            <span class="kw">let </span>link = <span class="self">self</span>.alloc_transition()<span class="question-mark">?</span>;
            <span class="self">self</span>.sparse[link] = Transition { byte, next, link: link_next };
            <span class="self">self</span>.sparse[link_prev].link = link;
        } <span class="kw">else </span>{
            <span class="macro">assert_eq!</span>(byte, <span class="self">self</span>.sparse[link_next].byte);
            <span class="self">self</span>.sparse[link_next].next = next;
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// This sets every possible transition (all 255 of them) for the given
    /// state to the name `next` value.
    ///
    /// This is useful for efficiently initializing start/dead states.
    ///
    /// # Panics
    ///
    /// This requires that the state has no transitions added to it already.
    /// If it has any transitions, then this panics. It will also panic if
    /// the state has been densified prior to calling this.
    </span><span class="kw">fn </span>init_full_state(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        prev: StateID,
        next: StateID,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="macro">assert_eq!</span>(
            StateID::ZERO,
            <span class="self">self</span>.states[prev].dense,
            <span class="string">"state must not be dense yet"
        </span>);
        <span class="macro">assert_eq!</span>(
            StateID::ZERO,
            <span class="self">self</span>.states[prev].sparse,
            <span class="string">"state must have zero transitions"
        </span>);
        <span class="kw">let </span><span class="kw-2">mut </span>prev_link = StateID::ZERO;
        <span class="kw">for </span>byte <span class="kw">in </span><span class="number">0</span>..=<span class="number">255 </span>{
            <span class="kw">let </span>new_link = <span class="self">self</span>.alloc_transition()<span class="question-mark">?</span>;
            <span class="self">self</span>.sparse[new_link] =
                Transition { byte, next, link: StateID::ZERO };
            <span class="kw">if </span>prev_link == StateID::ZERO {
                <span class="self">self</span>.states[prev].sparse = new_link;
            } <span class="kw">else </span>{
                <span class="self">self</span>.sparse[prev_link].link = new_link;
            }
            prev_link = new_link;
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Add a match for the given pattern ID to the state for the given ID.
    </span><span class="kw">fn </span>add_match(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        sid: StateID,
        pid: PatternID,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="kw">let </span>head = <span class="self">self</span>.states[sid].matches;
        <span class="kw">let </span><span class="kw-2">mut </span>link = head;
        <span class="kw">while </span><span class="self">self</span>.matches[link].link != StateID::ZERO {
            link = <span class="self">self</span>.matches[link].link;
        }
        <span class="kw">let </span>new_match_link = <span class="self">self</span>.alloc_match()<span class="question-mark">?</span>;
        <span class="self">self</span>.matches[new_match_link].pid = pid;
        <span class="kw">if </span>link == StateID::ZERO {
            <span class="self">self</span>.states[sid].matches = new_match_link;
        } <span class="kw">else </span>{
            <span class="self">self</span>.matches[link].link = new_match_link;
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Copy matches from the `src` state to the `dst` state. This is useful
    /// when a match state can be reached via a failure transition. In which
    /// case, you'll want to copy the matches (if any) from the state reached
    /// by the failure transition to the original state you were at.
    </span><span class="kw">fn </span>copy_matches(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        src: StateID,
        dst: StateID,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="kw">let </span>head_dst = <span class="self">self</span>.states[dst].matches;
        <span class="kw">let </span><span class="kw-2">mut </span>link_dst = head_dst;
        <span class="kw">while </span><span class="self">self</span>.matches[link_dst].link != StateID::ZERO {
            link_dst = <span class="self">self</span>.matches[link_dst].link;
        }
        <span class="kw">let </span><span class="kw-2">mut </span>link_src = <span class="self">self</span>.states[src].matches;
        <span class="kw">while </span>link_src != StateID::ZERO {
            <span class="kw">let </span>new_match_link =
                StateID::new(<span class="self">self</span>.matches.len()).map_err(|e| {
                    BuildError::state_id_overflow(
                        StateID::MAX.as_u64(),
                        e.attempted(),
                    )
                })<span class="question-mark">?</span>;
            <span class="self">self</span>.matches.push(Match {
                pid: <span class="self">self</span>.matches[link_src].pid,
                link: StateID::ZERO,
            });
            <span class="kw">if </span>link_dst == StateID::ZERO {
                <span class="self">self</span>.states[dst].matches = new_match_link;
            } <span class="kw">else </span>{
                <span class="self">self</span>.matches[link_dst].link = new_match_link;
            }

            link_dst = new_match_link;
            link_src = <span class="self">self</span>.matches[link_src].link;
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Create a new entry in `NFA::trans`, if there's room, and return that
    /// entry's ID. If there's no room, then an error is returned.
    </span><span class="kw">fn </span>alloc_transition(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;StateID, BuildError&gt; {
        <span class="kw">let </span>id = StateID::new(<span class="self">self</span>.sparse.len()).map_err(|e| {
            BuildError::state_id_overflow(StateID::MAX.as_u64(), e.attempted())
        })<span class="question-mark">?</span>;
        <span class="self">self</span>.sparse.push(Transition::default());
        <span class="prelude-val">Ok</span>(id)
    }

    <span class="doccomment">/// Create a new entry in `NFA::matches`, if there's room, and return that
    /// entry's ID. If there's no room, then an error is returned.
    </span><span class="kw">fn </span>alloc_match(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;StateID, BuildError&gt; {
        <span class="kw">let </span>id = StateID::new(<span class="self">self</span>.matches.len()).map_err(|e| {
            BuildError::state_id_overflow(StateID::MAX.as_u64(), e.attempted())
        })<span class="question-mark">?</span>;
        <span class="self">self</span>.matches.push(Match::default());
        <span class="prelude-val">Ok</span>(id)
    }

    <span class="doccomment">/// Create a new set of `N` transitions in this NFA's dense transition
    /// table. The ID return corresponds to the index at which the `N`
    /// transitions begin. So `id+0` is the first transition and `id+(N-1)` is
    /// the last.
    ///
    /// `N` is determined via `NFA::byte_classes::alphabet_len`.
    </span><span class="kw">fn </span>alloc_dense_state(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;StateID, BuildError&gt; {
        <span class="kw">let </span>id = StateID::new(<span class="self">self</span>.dense.len()).map_err(|e| {
            BuildError::state_id_overflow(StateID::MAX.as_u64(), e.attempted())
        })<span class="question-mark">?</span>;
        <span class="comment">// We use FAIL because it's the correct default. If a state doesn't
        // have a transition defined for every possible byte value, then the
        // transition function should return NFA::FAIL.
        </span><span class="self">self</span>.dense.extend(
            core::iter::repeat(NFA::FAIL)
                .take(<span class="self">self</span>.byte_classes.alphabet_len()),
        );
        <span class="prelude-val">Ok</span>(id)
    }

    <span class="doccomment">/// Allocate and add a fresh state to the underlying NFA and return its
    /// ID (guaranteed to be one more than the ID of the previously allocated
    /// state). If the ID would overflow `StateID`, then this returns an error.
    </span><span class="kw">fn </span>alloc_state(<span class="kw-2">&amp;mut </span><span class="self">self</span>, depth: usize) -&gt; <span class="prelude-ty">Result</span>&lt;StateID, BuildError&gt; {
        <span class="comment">// This is OK because we error when building the trie if we see a
        // pattern whose length cannot fit into a 'SmallIndex', and the longest
        // possible depth corresponds to the length of the longest pattern.
        </span><span class="kw">let </span>depth = SmallIndex::new(depth)
            .expect(<span class="string">"patterns longer than SmallIndex::MAX are not allowed"</span>);
        <span class="kw">let </span>id = StateID::new(<span class="self">self</span>.states.len()).map_err(|e| {
            BuildError::state_id_overflow(StateID::MAX.as_u64(), e.attempted())
        })<span class="question-mark">?</span>;
        <span class="self">self</span>.states.push(State {
            sparse: StateID::ZERO,
            dense: StateID::ZERO,
            matches: StateID::ZERO,
            fail: <span class="self">self</span>.special.start_unanchored_id,
            depth,
        });
        <span class="prelude-val">Ok</span>(id)
    }
}

<span class="comment">// SAFETY: 'start_state' always returns a valid state ID, 'next_state' always
// returns a valid state ID given a valid state ID. We otherwise claim that
// all other methods are correct as well.
</span><span class="kw">unsafe impl </span>Automaton <span class="kw">for </span>NFA {
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>start_state(<span class="kw-2">&amp;</span><span class="self">self</span>, anchored: Anchored) -&gt; <span class="prelude-ty">Result</span>&lt;StateID, MatchError&gt; {
        <span class="kw">match </span>anchored {
            Anchored::No =&gt; <span class="prelude-val">Ok</span>(<span class="self">self</span>.special.start_unanchored_id),
            Anchored::Yes =&gt; <span class="prelude-val">Ok</span>(<span class="self">self</span>.special.start_anchored_id),
        }
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>next_state(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        anchored: Anchored,
        <span class="kw-2">mut </span>sid: StateID,
        byte: u8,
    ) -&gt; StateID {
        <span class="comment">// This terminates since:
        //
        // 1. state.fail never points to the FAIL state.
        // 2. All state.fail values point to a state closer to the start state.
        // 3. The start state has no transitions to the FAIL state.
        </span><span class="kw">loop </span>{
            <span class="kw">let </span>next = <span class="self">self</span>.follow_transition(sid, byte);
            <span class="kw">if </span>next != NFA::FAIL {
                <span class="kw">return </span>next;
            }
            <span class="comment">// For an anchored search, we never follow failure transitions
            // because failure transitions lead us down a path to matching
            // a *proper* suffix of the path we were on. Thus, it can only
            // produce matches that appear after the beginning of the search.
            </span><span class="kw">if </span>anchored.is_anchored() {
                <span class="kw">return </span>NFA::DEAD;
            }
            sid = <span class="self">self</span>.states[sid].fail();
        }
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>is_special(<span class="kw-2">&amp;</span><span class="self">self</span>, sid: StateID) -&gt; bool {
        sid &lt;= <span class="self">self</span>.special.max_special_id
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>is_dead(<span class="kw-2">&amp;</span><span class="self">self</span>, sid: StateID) -&gt; bool {
        sid == NFA::DEAD
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>is_match(<span class="kw-2">&amp;</span><span class="self">self</span>, sid: StateID) -&gt; bool {
        <span class="comment">// N.B. This returns true when sid==NFA::FAIL but that's okay because
        // NFA::FAIL is not actually a valid state ID from the perspective of
        // the Automaton trait. Namely, it is never returned by 'start_state'
        // or by 'next_state'. So we don't need to care about it here.
        </span>!<span class="self">self</span>.is_dead(sid) &amp;&amp; sid &lt;= <span class="self">self</span>.special.max_match_id
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>is_start(<span class="kw-2">&amp;</span><span class="self">self</span>, sid: StateID) -&gt; bool {
        sid == <span class="self">self</span>.special.start_unanchored_id
            || sid == <span class="self">self</span>.special.start_anchored_id
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>match_kind(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; MatchKind {
        <span class="self">self</span>.match_kind
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>patterns_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.pattern_lens.len()
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>pattern_len(<span class="kw-2">&amp;</span><span class="self">self</span>, pid: PatternID) -&gt; usize {
        <span class="self">self</span>.pattern_lens[pid].as_usize()
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>min_pattern_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.min_pattern_len
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>max_pattern_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.max_pattern_len
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>match_len(<span class="kw-2">&amp;</span><span class="self">self</span>, sid: StateID) -&gt; usize {
        <span class="self">self</span>.iter_matches(sid).count()
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>match_pattern(<span class="kw-2">&amp;</span><span class="self">self</span>, sid: StateID, index: usize) -&gt; PatternID {
        <span class="self">self</span>.iter_matches(sid).nth(index).unwrap()
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>memory_usage(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.states.len() * core::mem::size_of::&lt;State&gt;()
            + <span class="self">self</span>.sparse.len() * core::mem::size_of::&lt;Transition&gt;()
            + <span class="self">self</span>.matches.len() * core::mem::size_of::&lt;Match&gt;()
            + <span class="self">self</span>.dense.len() * StateID::SIZE
            + <span class="self">self</span>.pattern_lens.len() * SmallIndex::SIZE
            + <span class="self">self</span>.prefilter.as_ref().map_or(<span class="number">0</span>, |p| p.memory_usage())
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>prefilter(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>Prefilter&gt; {
        <span class="self">self</span>.prefilter.as_ref()
    }
}

<span class="doccomment">/// A representation of a sparse NFA state for an Aho-Corasick automaton.
///
/// It contains the transitions to the next state, a failure transition for
/// cases where there exists no other transition for the current input byte
/// and the matches implied by visiting this state (if any).
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>State {
    <span class="doccomment">/// A pointer to `NFA::trans` corresponding to the head of a linked list
    /// containing all of the transitions for this state.
    ///
    /// This is `StateID::ZERO` if and only if this state has zero transitions.
    </span>sparse: StateID,
    <span class="doccomment">/// A pointer to a row of `N` transitions in `NFA::dense`. These
    /// transitions correspond precisely to what is obtained by traversing
    /// `sparse`, but permits constant time lookup.
    ///
    /// When this is zero (which is true for most states in the default
    /// configuration), then this state has no dense representation.
    ///
    /// Note that `N` is equal to `NFA::byte_classes::alphabet_len()`. This is
    /// typically much less than 256 (the maximum value).
    </span>dense: StateID,
    <span class="doccomment">/// A pointer to `NFA::matches` corresponding to the head of a linked list
    /// containing all of the matches for this state.
    ///
    /// This is `StateID::ZERO` if and only if this state is not a match state.
    </span>matches: StateID,
    <span class="doccomment">/// The state that should be transitioned to if the current byte in the
    /// haystack does not have a corresponding transition defined in this
    /// state.
    </span>fail: StateID,
    <span class="doccomment">/// The depth of this state. Specifically, this is the distance from this
    /// state to the starting state. (For the special sentinel states DEAD and
    /// FAIL, their depth is always 0.) The depth of a starting state is 0.
    ///
    /// Note that depth is currently not used in this non-contiguous NFA. It
    /// may in the future, but it is used in the contiguous NFA. Namely, it
    /// permits an optimization where states near the starting state have their
    /// transitions stored in a dense fashion, but all other states have their
    /// transitions stored in a sparse fashion. (This non-contiguous NFA uses
    /// a sparse representation for all states unconditionally.) In any case,
    /// this is really the only convenient place to compute and store this
    /// information, which we need when building the contiguous NFA.
    </span>depth: SmallIndex,
}

<span class="kw">impl </span>State {
    <span class="doccomment">/// Return true if and only if this state is a match state.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_match(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.matches != StateID::ZERO
    }

    <span class="doccomment">/// Returns the failure transition for this state.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>fail(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; StateID {
        <span class="self">self</span>.fail
    }

    <span class="doccomment">/// Returns the depth of this state. That is, the number of transitions
    /// this state is from the start state of the NFA.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>depth(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; SmallIndex {
        <span class="self">self</span>.depth
    }
}

<span class="doccomment">/// A single transition in a non-contiguous NFA.
</span><span class="attr">#[derive(Clone, Copy, Default)]
#[repr(packed)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Transition {
    byte: u8,
    next: StateID,
    link: StateID,
}

<span class="kw">impl </span>Transition {
    <span class="doccomment">/// Return the byte for which this transition is defined.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>byte(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u8 {
        <span class="self">self</span>.byte
    }

    <span class="doccomment">/// Return the ID of the state that this transition points to.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>next(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; StateID {
        <span class="self">self</span>.next
    }

    <span class="doccomment">/// Return the ID of the next transition.
    </span><span class="kw">fn </span>link(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; StateID {
        <span class="self">self</span>.link
    }
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>Transition {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        <span class="macro">write!</span>(
            f,
            <span class="string">"Transition(byte: {:X?}, next: {:?}, link: {:?})"</span>,
            <span class="self">self</span>.byte,
            <span class="self">self</span>.next().as_usize(),
            <span class="self">self</span>.link().as_usize()
        )
    }
}

<span class="doccomment">/// A single match in a non-contiguous NFA.
</span><span class="attr">#[derive(Clone, Copy, Default)]
</span><span class="kw">struct </span>Match {
    pid: PatternID,
    link: StateID,
}

<span class="kw">impl </span>Match {
    <span class="doccomment">/// Return the pattern ID for this match.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>pattern(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; PatternID {
        <span class="self">self</span>.pid
    }

    <span class="doccomment">/// Return the ID of the next match.
    </span><span class="kw">fn </span>link(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; StateID {
        <span class="self">self</span>.link
    }
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>Match {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        <span class="macro">write!</span>(
            f,
            <span class="string">"Match(pid: {:?}, link: {:?})"</span>,
            <span class="self">self</span>.pattern().as_usize(),
            <span class="self">self</span>.link().as_usize()
        )
    }
}

<span class="doccomment">/// A builder for configuring an Aho-Corasick noncontiguous NFA.
///
/// This builder has a subset of the options available to a
/// [`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
/// their behavior is identical.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>Builder {
    match_kind: MatchKind,
    prefilter: bool,
    ascii_case_insensitive: bool,
    dense_depth: usize,
}

<span class="kw">impl </span>Default <span class="kw">for </span>Builder {
    <span class="kw">fn </span>default() -&gt; Builder {
        Builder {
            match_kind: MatchKind::default(),
            prefilter: <span class="bool-val">true</span>,
            ascii_case_insensitive: <span class="bool-val">false</span>,
            dense_depth: <span class="number">3</span>,
        }
    }
}

<span class="kw">impl </span>Builder {
    <span class="doccomment">/// Create a new builder for configuring an Aho-Corasick noncontiguous NFA.
    </span><span class="kw">pub fn </span>new() -&gt; Builder {
        Builder::default()
    }

    <span class="doccomment">/// Build an Aho-Corasick noncontiguous NFA from the given iterator of
    /// patterns.
    ///
    /// A builder may be reused to create more NFAs.
    </span><span class="kw">pub fn </span>build&lt;I, P&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, patterns: I) -&gt; <span class="prelude-ty">Result</span>&lt;NFA, BuildError&gt;
    <span class="kw">where
        </span>I: IntoIterator&lt;Item = P&gt;,
        P: AsRef&lt;[u8]&gt;,
    {
        <span class="macro">debug!</span>(<span class="string">"building non-contiguous NFA"</span>);
        <span class="kw">let </span>nfa = Compiler::new(<span class="self">self</span>)<span class="question-mark">?</span>.compile(patterns)<span class="question-mark">?</span>;
        <span class="macro">debug!</span>(
            <span class="string">"non-contiguous NFA built, &lt;states: {:?}, size: {:?}&gt;"</span>,
            nfa.states.len(),
            nfa.memory_usage()
        );
        <span class="prelude-val">Ok</span>(nfa)
    }

    <span class="doccomment">/// Set the desired match semantics.
    ///
    /// See
    /// [`AhoCorasickBuilder::match_kind`](crate::AhoCorasickBuilder::match_kind)
    /// for more documentation and examples.
    </span><span class="kw">pub fn </span>match_kind(<span class="kw-2">&amp;mut </span><span class="self">self</span>, kind: MatchKind) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.match_kind = kind;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Enable ASCII-aware case insensitive matching.
    ///
    /// See
    /// [`AhoCorasickBuilder::ascii_case_insensitive`](crate::AhoCorasickBuilder::ascii_case_insensitive)
    /// for more documentation and examples.
    </span><span class="kw">pub fn </span>ascii_case_insensitive(<span class="kw-2">&amp;mut </span><span class="self">self</span>, yes: bool) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.ascii_case_insensitive = yes;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Set the limit on how many states use a dense representation for their
    /// transitions. Other states will generally use a sparse representation.
    ///
    /// See
    /// [`AhoCorasickBuilder::dense_depth`](crate::AhoCorasickBuilder::dense_depth)
    /// for more documentation and examples.
    </span><span class="kw">pub fn </span>dense_depth(<span class="kw-2">&amp;mut </span><span class="self">self</span>, depth: usize) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.dense_depth = depth;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Enable heuristic prefilter optimizations.
    ///
    /// See
    /// [`AhoCorasickBuilder::prefilter`](crate::AhoCorasickBuilder::prefilter)
    /// for more documentation and examples.
    </span><span class="kw">pub fn </span>prefilter(<span class="kw-2">&amp;mut </span><span class="self">self</span>, yes: bool) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.prefilter = yes;
        <span class="self">self
    </span>}
}

<span class="doccomment">/// A compiler uses a builder configuration and builds up the NFA formulation
/// of an Aho-Corasick automaton. This roughly corresponds to the standard
/// formulation described in textbooks, with some tweaks to support leftmost
/// searching.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">struct </span>Compiler&lt;<span class="lifetime">'a</span>&gt; {
    builder: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>Builder,
    prefilter: prefilter::Builder,
    nfa: NFA,
    byteset: ByteClassSet,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Compiler&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">fn </span>new(builder: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>Builder) -&gt; <span class="prelude-ty">Result</span>&lt;Compiler&lt;<span class="lifetime">'a</span>&gt;, BuildError&gt; {
        <span class="kw">let </span>prefilter = prefilter::Builder::new(builder.match_kind)
            .ascii_case_insensitive(builder.ascii_case_insensitive);
        <span class="prelude-val">Ok</span>(Compiler {
            builder,
            prefilter,
            nfa: NFA {
                match_kind: builder.match_kind,
                states: <span class="macro">vec!</span>[],
                sparse: <span class="macro">vec!</span>[],
                dense: <span class="macro">vec!</span>[],
                matches: <span class="macro">vec!</span>[],
                pattern_lens: <span class="macro">vec!</span>[],
                prefilter: <span class="prelude-val">None</span>,
                byte_classes: ByteClasses::singletons(),
                min_pattern_len: usize::MAX,
                max_pattern_len: <span class="number">0</span>,
                special: Special::zero(),
            },
            byteset: ByteClassSet::empty(),
        })
    }

    <span class="kw">fn </span>compile&lt;I, P&gt;(<span class="kw-2">mut </span><span class="self">self</span>, patterns: I) -&gt; <span class="prelude-ty">Result</span>&lt;NFA, BuildError&gt;
    <span class="kw">where
        </span>I: IntoIterator&lt;Item = P&gt;,
        P: AsRef&lt;[u8]&gt;,
    {
        <span class="comment">// Add dummy transition/match links, so that no valid link will point
        // to another link at index 0.
        </span><span class="self">self</span>.nfa.sparse.push(Transition::default());
        <span class="self">self</span>.nfa.matches.push(Match::default());
        <span class="comment">// Add a dummy dense transition so that no states can have dense==0
        // represent a valid pointer to dense transitions. This permits
        // dense==0 to be a sentinel indicating "no dense transitions."
        </span><span class="self">self</span>.nfa.dense.push(NFA::DEAD);
        <span class="comment">// the dead state, only used for leftmost and fixed to id==0
        </span><span class="self">self</span>.nfa.alloc_state(<span class="number">0</span>)<span class="question-mark">?</span>;
        <span class="comment">// the fail state, which is never entered and fixed to id==1
        </span><span class="self">self</span>.nfa.alloc_state(<span class="number">0</span>)<span class="question-mark">?</span>;
        <span class="comment">// unanchored start state, initially fixed to id==2 but later shuffled
        // to appear after all non-start match states.
        </span><span class="self">self</span>.nfa.special.start_unanchored_id = <span class="self">self</span>.nfa.alloc_state(<span class="number">0</span>)<span class="question-mark">?</span>;
        <span class="comment">// anchored start state, initially fixed to id==3 but later shuffled
        // to appear after unanchored start state.
        </span><span class="self">self</span>.nfa.special.start_anchored_id = <span class="self">self</span>.nfa.alloc_state(<span class="number">0</span>)<span class="question-mark">?</span>;
        <span class="comment">// Initialize the unanchored starting state in order to make it dense,
        // and thus make transition lookups on this state faster.
        </span><span class="self">self</span>.init_unanchored_start_state()<span class="question-mark">?</span>;
        <span class="comment">// Set all transitions on the DEAD state to point to itself. This way,
        // the DEAD state can never be escaped. It MUST be used as a sentinel
        // in any correct search.
        </span><span class="self">self</span>.add_dead_state_loop()<span class="question-mark">?</span>;
        <span class="comment">// Build the base trie from the given patterns.
        </span><span class="self">self</span>.build_trie(patterns)<span class="question-mark">?</span>;
        <span class="self">self</span>.nfa.states.shrink_to_fit();
        <span class="comment">// Turn our set of bytes into equivalent classes. This NFA
        // implementation uses byte classes only for states that use a dense
        // representation of transitions. (And that's why this comes before
        // `self.densify()`, as the byte classes need to be set first.)
        </span><span class="self">self</span>.nfa.byte_classes = <span class="self">self</span>.byteset.byte_classes();
        <span class="comment">// Add transitions (and maybe matches) to the anchored starting state.
        // The anchored starting state is used for anchored searches. The only
        // mechanical difference between it and the unanchored start state is
        // that missing transitions map to the DEAD state instead of the FAIL
        // state.
        </span><span class="self">self</span>.set_anchored_start_state()<span class="question-mark">?</span>;
        <span class="comment">// Rewrite transitions to the FAIL state on the unanchored start state
        // as self-transitions. This keeps the start state active at all times.
        </span><span class="self">self</span>.add_unanchored_start_state_loop();
        <span class="comment">// Make some (possibly zero) states use a dense representation for
        // transitions. It's important to do this right after the states
        // and non-failure transitions are solidified. That way, subsequent
        // accesses (particularly `fill_failure_transitions`) will benefit from
        // the faster transition lookup in densified states.
        </span><span class="self">self</span>.densify()<span class="question-mark">?</span>;
        <span class="comment">// The meat of the Aho-Corasick algorithm: compute and write failure
        // transitions. i.e., the state to move to when a transition isn't
        // defined in the current state. These are epsilon transitions and thus
        // make this formulation an NFA.
        </span><span class="self">self</span>.fill_failure_transitions()<span class="question-mark">?</span>;
        <span class="comment">// Handle a special case under leftmost semantics when at least one
        // of the patterns is the empty string.
        </span><span class="self">self</span>.close_start_state_loop_for_leftmost();
        <span class="comment">// Shuffle states so that we have DEAD, FAIL, MATCH, ..., START, START,
        // NON-MATCH, ... This permits us to very quickly query the type of
        // the state we're currently in during a search.
        </span><span class="self">self</span>.shuffle();
        <span class="self">self</span>.nfa.prefilter = <span class="self">self</span>.prefilter.build();
        <span class="comment">// Store the maximum ID of all *relevant* special states. Start states
        // are only relevant when we have a prefilter, otherwise, there is zero
        // reason to care about whether a state is a start state or not during
        // a search. Indeed, without a prefilter, we are careful to explicitly
        // NOT care about start states, otherwise the search can ping pong
        // between the unrolled loop and the handling of special-status states
        // and destroy perf.
        </span><span class="self">self</span>.nfa.special.max_special_id = <span class="kw">if </span><span class="self">self</span>.nfa.prefilter.is_some() {
            <span class="comment">// Why the anchored starting state? Because we always put it
            // after the unanchored starting state and it is therefore the
            // maximum. Why put unanchored followed by anchored? No particular
            // reason, but that's how the states are logically organized in the
            // Thompson NFA implementation found in regex-automata. ¯\_(ツ)_/¯
            </span><span class="self">self</span>.nfa.special.start_anchored_id
        } <span class="kw">else </span>{
            <span class="self">self</span>.nfa.special.max_match_id
        };
        <span class="self">self</span>.nfa.sparse.shrink_to_fit();
        <span class="self">self</span>.nfa.dense.shrink_to_fit();
        <span class="self">self</span>.nfa.matches.shrink_to_fit();
        <span class="self">self</span>.nfa.pattern_lens.shrink_to_fit();
        <span class="prelude-val">Ok</span>(<span class="self">self</span>.nfa)
    }

    <span class="doccomment">/// This sets up the initial prefix trie that makes up the Aho-Corasick
    /// automaton. Effectively, it creates the basic structure of the
    /// automaton, where every pattern given has a path from the start state to
    /// the end of the pattern.
    </span><span class="kw">fn </span>build_trie&lt;I, P&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, patterns: I) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt;
    <span class="kw">where
        </span>I: IntoIterator&lt;Item = P&gt;,
        P: AsRef&lt;[u8]&gt;,
    {
        <span class="lifetime">'PATTERNS</span>: <span class="kw">for </span>(i, pat) <span class="kw">in </span>patterns.into_iter().enumerate() {
            <span class="kw">let </span>pid = PatternID::new(i).map_err(|e| {
                BuildError::pattern_id_overflow(
                    PatternID::MAX.as_u64(),
                    e.attempted(),
                )
            })<span class="question-mark">?</span>;
            <span class="kw">let </span>pat = pat.as_ref();
            <span class="kw">let </span>patlen = SmallIndex::new(pat.len())
                .map_err(|<span class="kw">_</span>| BuildError::pattern_too_long(pid, pat.len()))<span class="question-mark">?</span>;
            <span class="self">self</span>.nfa.min_pattern_len =
                core::cmp::min(<span class="self">self</span>.nfa.min_pattern_len, pat.len());
            <span class="self">self</span>.nfa.max_pattern_len =
                core::cmp::max(<span class="self">self</span>.nfa.max_pattern_len, pat.len());
            <span class="macro">assert_eq!</span>(
                i,
                <span class="self">self</span>.nfa.pattern_lens.len(),
                <span class="string">"expected number of patterns to match pattern ID"
            </span>);
            <span class="self">self</span>.nfa.pattern_lens.push(patlen);
            <span class="comment">// We add the pattern to the prefilter here because the pattern
            // ID in the prefilter is determined with respect to the patterns
            // added to the prefilter. That is, it isn't the ID we have here,
            // but the one determined by its own accounting of patterns.
            // To ensure they line up, we add every pattern we see to the
            // prefilter, even if some patterns ultimately are impossible to
            // match (in leftmost-first semantics specifically).
            //
            // Another way of doing this would be to expose an API in the
            // prefilter to permit setting your own pattern IDs. Or to just use
            // our own map and go between them. But this case is sufficiently
            // rare that we don't bother and just make sure they're in sync.
            </span><span class="kw">if </span><span class="self">self</span>.builder.prefilter {
                <span class="self">self</span>.prefilter.add(pat);
            }

            <span class="kw">let </span><span class="kw-2">mut </span>prev = <span class="self">self</span>.nfa.special.start_unanchored_id;
            <span class="kw">let </span><span class="kw-2">mut </span>saw_match = <span class="bool-val">false</span>;
            <span class="kw">for </span>(depth, <span class="kw-2">&amp;</span>b) <span class="kw">in </span>pat.iter().enumerate() {
                <span class="comment">// When leftmost-first match semantics are requested, we
                // specifically stop adding patterns when a previously added
                // pattern is a prefix of it. We avoid adding it because
                // leftmost-first semantics imply that the pattern can never
                // match. This is not just an optimization to save space! It
                // is necessary for correctness. In fact, this is the only
                // difference in the automaton between the implementations for
                // leftmost-first and leftmost-longest.
                </span>saw_match = saw_match || <span class="self">self</span>.nfa.states[prev].is_match();
                <span class="kw">if </span><span class="self">self</span>.builder.match_kind.is_leftmost_first() &amp;&amp; saw_match {
                    <span class="comment">// Skip to the next pattern immediately. This avoids
                    // incorrectly adding a match after this loop terminates.
                    </span><span class="kw">continue </span><span class="lifetime">'PATTERNS</span>;
                }

                <span class="comment">// Add this byte to our equivalence classes. These don't
                // get used while building the trie, but other Aho-Corasick
                // implementations may use them.
                </span><span class="self">self</span>.byteset.set_range(b, b);
                <span class="kw">if </span><span class="self">self</span>.builder.ascii_case_insensitive {
                    <span class="kw">let </span>b = opposite_ascii_case(b);
                    <span class="self">self</span>.byteset.set_range(b, b);
                }

                <span class="comment">// If the transition from prev using the current byte already
                // exists, then just move through it. Otherwise, add a new
                // state. We track the depth here so that we can determine
                // how to represent transitions. States near the start state
                // use a dense representation that uses more memory but is
                // faster. Other states use a sparse representation that uses
                // less memory but is slower.
                </span><span class="kw">let </span>next = <span class="self">self</span>.nfa.follow_transition(prev, b);
                <span class="kw">if </span>next != NFA::FAIL {
                    prev = next;
                } <span class="kw">else </span>{
                    <span class="kw">let </span>next = <span class="self">self</span>.nfa.alloc_state(depth)<span class="question-mark">?</span>;
                    <span class="self">self</span>.nfa.add_transition(prev, b, next)<span class="question-mark">?</span>;
                    <span class="kw">if </span><span class="self">self</span>.builder.ascii_case_insensitive {
                        <span class="kw">let </span>b = opposite_ascii_case(b);
                        <span class="self">self</span>.nfa.add_transition(prev, b, next)<span class="question-mark">?</span>;
                    }
                    prev = next;
                }
            }
            <span class="comment">// Once the pattern has been added, log the match in the final
            // state that it reached.
            </span><span class="self">self</span>.nfa.add_match(prev, pid)<span class="question-mark">?</span>;
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// This routine creates failure transitions according to the standard
    /// textbook formulation of the Aho-Corasick algorithm, with a couple small
    /// tweaks to support "leftmost" semantics.
    ///
    /// Building failure transitions is the most interesting part of building
    /// the Aho-Corasick automaton, because they are what allow searches to
    /// be performed in linear time. Specifically, a failure transition is
    /// a single transition associated with each state that points back to
    /// the longest proper suffix of the pattern being searched. The failure
    /// transition is followed whenever there exists no transition on the
    /// current state for the current input byte. If there is no other proper
    /// suffix, then the failure transition points back to the starting state.
    ///
    /// For example, let's say we built an Aho-Corasick automaton with the
    /// following patterns: 'abcd' and 'cef'. The trie looks like this:
    ///
    /// ```ignore
    ///          a - S1 - b - S2 - c - S3 - d - S4*
    ///         /
    ///     S0 - c - S5 - e - S6 - f - S7*
    /// ```
    ///
    /// At this point, it should be fairly straight-forward to see how this
    /// trie can be used in a simplistic way. At any given position in the
    /// text we're searching (called the "subject" string), all we need to do
    /// is follow the transitions in the trie by consuming one transition for
    /// each byte in the subject string. If we reach a match state, then we can
    /// report that location as a match.
    ///
    /// The trick comes when searching a subject string like 'abcef'. We'll
    /// initially follow the transition from S0 to S1 and wind up in S3 after
    /// observng the 'c' byte. At this point, the next byte is 'e' but state
    /// S3 has no transition for 'e', so the search fails. We then would need
    /// to restart the search at the next position in 'abcef', which
    /// corresponds to 'b'. The match would fail, but the next search starting
    /// at 'c' would finally succeed. The problem with this approach is that
    /// we wind up searching the subject string potentially many times. In
    /// effect, this makes the algorithm have worst case `O(n * m)` complexity,
    /// where `n ~ len(subject)` and `m ~ len(all patterns)`. We would instead
    /// like to achieve a `O(n + m)` worst case complexity.
    ///
    /// This is where failure transitions come in. Instead of dying at S3 in
    /// the first search, the automaton can instruct the search to move to
    /// another part of the automaton that corresponds to a suffix of what
    /// we've seen so far. Recall that we've seen 'abc' in the subject string,
    /// and the automaton does indeed have a non-empty suffix, 'c', that could
    /// potentially lead to another match. Thus, the actual Aho-Corasick
    /// automaton for our patterns in this case looks like this:
    ///
    /// ```ignore
    ///          a - S1 - b - S2 - c - S3 - d - S4*
    ///         /                      /
    ///        /       ----------------
    ///       /       /
    ///     S0 - c - S5 - e - S6 - f - S7*
    /// ```
    ///
    /// That is, we have a failure transition from S3 to S5, which is followed
    /// exactly in cases when we are in state S3 but see any byte other than
    /// 'd' (that is, we've "failed" to find a match in this portion of our
    /// trie). We know we can transition back to S5 because we've already seen
    /// a 'c' byte, so we don't need to re-scan it. We can then pick back up
    /// with the search starting at S5 and complete our match.
    ///
    /// Adding failure transitions to a trie is fairly simple, but subtle. The
    /// key issue is that you might have multiple failure transition that you
    /// need to follow. For example, look at the trie for the patterns
    /// 'abcd', 'b', 'bcd' and 'cd':
    ///
    /// ```ignore
    ///          - a - S1 - b - S2* - c - S3 - d - S4*
    ///         /               /         /
    ///        /         -------   -------
    ///       /         /         /
    ///     S0 --- b - S5* - c - S6 - d - S7*
    ///       \                  /
    ///        \         --------
    ///         \       /
    ///          - c - S8 - d - S9*
    /// ```
    ///
    /// The failure transitions for this trie are defined from S2 to S5,
    /// S3 to S6 and S6 to S8. Moreover, state S2 needs to track that it
    /// corresponds to a match, since its failure transition to S5 is itself
    /// a match state.
    ///
    /// Perhaps simplest way to think about adding these failure transitions
    /// is recursively. That is, if you know the failure transitions for every
    /// possible previous state that could be visited (e.g., when computing the
    /// failure transition for S3, you already know the failure transitions
    /// for S0, S1 and S2), then you can simply follow the failure transition
    /// of the previous state and check whether the incoming transition is
    /// defined after following the failure transition.
    ///
    /// For example, when determining the failure state for S3, by our
    /// assumptions, we already know that there is a failure transition from
    /// S2 (the previous state) to S5. So we follow that transition and check
    /// whether the transition connecting S2 to S3 is defined. Indeed, it is,
    /// as there is a transition from S5 to S6 for the byte 'c'. If no such
    /// transition existed, we could keep following the failure transitions
    /// until we reach the start state, which is the failure transition for
    /// every state that has no corresponding proper suffix.
    ///
    /// We don't actually use recursion to implement this, but instead, use a
    /// breadth first search of the automaton. Our base case is the start
    /// state, whose failure transition is just a transition to itself.
    ///
    /// When building a leftmost automaton, we proceed as above, but only
    /// include a subset of failure transitions. Namely, we omit any failure
    /// transitions that appear after a match state in the trie. This is
    /// because failure transitions always point back to a proper suffix of
    /// what has been seen so far. Thus, following a failure transition after
    /// a match implies looking for a match that starts after the one that has
    /// already been seen, which is of course therefore not the leftmost match.
    ///
    /// N.B. I came up with this algorithm on my own, and after scouring all of
    /// the other AC implementations I know of (Perl, Snort, many on GitHub).
    /// I couldn't find any that implement leftmost semantics like this.
    /// Perl of course needs leftmost-first semantics, but they implement it
    /// with a seeming hack at *search* time instead of encoding it into the
    /// automaton. There are also a couple Java libraries that support leftmost
    /// longest semantics, but they do it by building a queue of matches at
    /// search time, which is even worse than what Perl is doing. ---AG
    </span><span class="kw">fn </span>fill_failure_transitions(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="kw">let </span>is_leftmost = <span class="self">self</span>.builder.match_kind.is_leftmost();
        <span class="kw">let </span>start_uid = <span class="self">self</span>.nfa.special.start_unanchored_id;
        <span class="comment">// Initialize the queue for breadth first search with all transitions
        // out of the start state. We handle the start state specially because
        // we only want to follow non-self transitions. If we followed self
        // transitions, then this would never terminate.
        </span><span class="kw">let </span><span class="kw-2">mut </span>queue = VecDeque::new();
        <span class="kw">let </span><span class="kw-2">mut </span>seen = <span class="self">self</span>.queued_set();
        <span class="kw">let </span><span class="kw-2">mut </span>prev_link = <span class="prelude-val">None</span>;
        <span class="kw">while let </span><span class="prelude-val">Some</span>(link) = <span class="self">self</span>.nfa.next_link(start_uid, prev_link) {
            prev_link = <span class="prelude-val">Some</span>(link);
            <span class="kw">let </span>t = <span class="self">self</span>.nfa.sparse[link];

            <span class="comment">// Skip anything we've seen before and any self-transitions on the
            // start state.
            </span><span class="kw">if </span>start_uid == t.next() || seen.contains(t.next) {
                <span class="kw">continue</span>;
            }
            queue.push_back(t.next);
            seen.insert(t.next);
            <span class="comment">// Under leftmost semantics, if a state immediately following
            // the start state is a match state, then we never want to
            // follow its failure transition since the failure transition
            // necessarily leads back to the start state, which we never
            // want to do for leftmost matching after a match has been
            // found.
            //
            // We apply the same logic to non-start states below as well.
            </span><span class="kw">if </span>is_leftmost &amp;&amp; <span class="self">self</span>.nfa.states[t.next].is_match() {
                <span class="self">self</span>.nfa.states[t.next].fail = NFA::DEAD;
            }
        }
        <span class="kw">while let </span><span class="prelude-val">Some</span>(id) = queue.pop_front() {
            <span class="kw">let </span><span class="kw-2">mut </span>prev_link = <span class="prelude-val">None</span>;
            <span class="kw">while let </span><span class="prelude-val">Some</span>(link) = <span class="self">self</span>.nfa.next_link(id, prev_link) {
                prev_link = <span class="prelude-val">Some</span>(link);
                <span class="kw">let </span>t = <span class="self">self</span>.nfa.sparse[link];

                <span class="kw">if </span>seen.contains(t.next) {
                    <span class="comment">// The only way to visit a duplicate state in a transition
                    // list is when ASCII case insensitivity is enabled. In
                    // this case, we want to skip it since it's redundant work.
                    // But it would also end up duplicating matches, which
                    // results in reporting duplicate matches in some cases.
                    // See the 'acasei010' regression test.
                    </span><span class="kw">continue</span>;
                }
                queue.push_back(t.next);
                seen.insert(t.next);

                <span class="comment">// As above for start states, under leftmost semantics, once
                // we see a match all subsequent states should have no failure
                // transitions because failure transitions always imply looking
                // for a match that is a suffix of what has been seen so far
                // (where "seen so far" corresponds to the string formed by
                // following the transitions from the start state to the
                // current state). Under leftmost semantics, we specifically do
                // not want to allow this to happen because we always want to
                // report the match found at the leftmost position.
                //
                // The difference between leftmost-first and leftmost-longest
                // occurs previously while we build the trie. For
                // leftmost-first, we simply omit any entries that would
                // otherwise require passing through a match state.
                //
                // Note that for correctness, the failure transition has to be
                // set to the dead state for ALL states following a match, not
                // just the match state itself. However, by setting the failure
                // transition to the dead state on all match states, the dead
                // state will automatically propagate to all subsequent states
                // via the failure state computation below.
                </span><span class="kw">if </span>is_leftmost &amp;&amp; <span class="self">self</span>.nfa.states[t.next].is_match() {
                    <span class="self">self</span>.nfa.states[t.next].fail = NFA::DEAD;
                    <span class="kw">continue</span>;
                }
                <span class="kw">let </span><span class="kw-2">mut </span>fail = <span class="self">self</span>.nfa.states[id].fail;
                <span class="kw">while </span><span class="self">self</span>.nfa.follow_transition(fail, t.byte) == NFA::FAIL {
                    fail = <span class="self">self</span>.nfa.states[fail].fail;
                }
                fail = <span class="self">self</span>.nfa.follow_transition(fail, t.byte);
                <span class="self">self</span>.nfa.states[t.next].fail = fail;
                <span class="self">self</span>.nfa.copy_matches(fail, t.next)<span class="question-mark">?</span>;
            }
            <span class="comment">// If the start state is a match state, then this automaton can
            // match the empty string. This implies all states are match states
            // since every position matches the empty string, so copy the
            // matches from the start state to every state. Strictly speaking,
            // this is only necessary for overlapping matches since each
            // non-empty non-start match state needs to report empty matches
            // in addition to its own. For the non-overlapping case, such
            // states only report the first match, which is never empty since
            // it isn't a start state.
            </span><span class="kw">if </span>!is_leftmost {
                <span class="self">self</span>.nfa
                    .copy_matches(<span class="self">self</span>.nfa.special.start_unanchored_id, id)<span class="question-mark">?</span>;
            }
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Shuffle the states so that they appear in this sequence:
    ///
    ///   DEAD, FAIL, MATCH..., START, START, NON-MATCH...
    ///
    /// The idea here is that if we know how special states are laid out in our
    /// transition table, then we can determine what "kind" of state we're in
    /// just by comparing our current state ID with a particular value. In this
    /// way, we avoid doing extra memory lookups.
    ///
    /// Before shuffling begins, our states look something like this:
    ///
    ///   DEAD, FAIL, START, START, (MATCH | NON-MATCH)...
    ///
    /// So all we need to do is move all of the MATCH states so that they
    /// all appear before any NON-MATCH state, like so:
    ///
    ///   DEAD, FAIL, START, START, MATCH... NON-MATCH...
    ///
    /// Then it's just a simple matter of swapping the two START states with
    /// the last two MATCH states.
    ///
    /// (This is the same technique used for fully compiled DFAs in
    /// regex-automata.)
    </span><span class="kw">fn </span>shuffle(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">let </span>old_start_uid = <span class="self">self</span>.nfa.special.start_unanchored_id;
        <span class="kw">let </span>old_start_aid = <span class="self">self</span>.nfa.special.start_anchored_id;
        <span class="macro">assert!</span>(old_start_uid &lt; old_start_aid);
        <span class="macro">assert_eq!</span>(
            <span class="number">3</span>,
            old_start_aid.as_usize(),
            <span class="string">"anchored start state should be at index 3"
        </span>);
        <span class="comment">// We implement shuffling by a sequence of pairwise swaps of states.
        // Since we have a number of things referencing states via their
        // IDs and swapping them changes their IDs, we need to record every
        // swap we make so that we can remap IDs. The remapper handles this
        // book-keeping for us.
        </span><span class="kw">let </span><span class="kw-2">mut </span>remapper = Remapper::new(<span class="kw-2">&amp;</span><span class="self">self</span>.nfa, <span class="number">0</span>);
        <span class="comment">// The way we proceed here is by moving all match states so that
        // they directly follow the start states. So it will go: DEAD, FAIL,
        // START-UNANCHORED, START-ANCHORED, MATCH, ..., NON-MATCH, ...
        //
        // To do that, we proceed forward through all states after
        // START-ANCHORED and swap match states so that they appear before all
        // non-match states.
        </span><span class="kw">let </span><span class="kw-2">mut </span>next_avail = StateID::from(<span class="number">4u8</span>);
        <span class="kw">for </span>i <span class="kw">in </span>next_avail.as_usize()..<span class="self">self</span>.nfa.states.len() {
            <span class="kw">let </span>sid = StateID::new(i).unwrap();
            <span class="kw">if </span>!<span class="self">self</span>.nfa.states[sid].is_match() {
                <span class="kw">continue</span>;
            }
            remapper.swap(<span class="kw-2">&amp;mut </span><span class="self">self</span>.nfa, sid, next_avail);
            <span class="comment">// The key invariant here is that only non-match states exist
            // between 'next_avail' and 'sid' (with them being potentially
            // equivalent). Thus, incrementing 'next_avail' by 1 is guaranteed
            // to land on the leftmost non-match state. (Unless 'next_avail'
            // and 'sid' are equivalent, in which case, a swap will occur but
            // it is a no-op.)
            </span>next_avail = StateID::new(next_avail.one_more()).unwrap();
        }
        <span class="comment">// Now we'd like to move the start states to immediately following the
        // match states. (The start states may themselves be match states, but
        // we'll handle that later.) We arrange the states this way so that we
        // don't necessarily need to check whether a state is a start state or
        // not before checking whether a state is a match state. For example,
        // we'd like to be able to write this as our state machine loop:
        //
        //   sid = start()
        //   for byte in haystack:
        //     sid = next(sid, byte)
        //     if sid &lt;= nfa.max_start_id:
        //       if sid &lt;= nfa.max_dead_id:
        //         # search complete
        //       elif sid &lt;= nfa.max_match_id:
        //         # found match
        //
        // The important context here is that we might not want to look for
        // start states at all. Namely, if a searcher doesn't have a prefilter,
        // then there is no reason to care about whether we're in a start state
        // or not. And indeed, if we did check for it, this very hot loop would
        // ping pong between the special state handling and the main state
        // transition logic. This in turn stalls the CPU by killing branch
        // prediction.
        //
        // So essentially, we really want to be able to "forget" that start
        // states even exist and this is why we put them at the end.
        </span><span class="kw">let </span>new_start_aid =
            StateID::new(next_avail.as_usize().checked_sub(<span class="number">1</span>).unwrap())
                .unwrap();
        remapper.swap(<span class="kw-2">&amp;mut </span><span class="self">self</span>.nfa, old_start_aid, new_start_aid);
        <span class="kw">let </span>new_start_uid =
            StateID::new(next_avail.as_usize().checked_sub(<span class="number">2</span>).unwrap())
                .unwrap();
        remapper.swap(<span class="kw-2">&amp;mut </span><span class="self">self</span>.nfa, old_start_uid, new_start_uid);
        <span class="kw">let </span>new_max_match_id =
            StateID::new(next_avail.as_usize().checked_sub(<span class="number">3</span>).unwrap())
                .unwrap();
        <span class="self">self</span>.nfa.special.max_match_id = new_max_match_id;
        <span class="self">self</span>.nfa.special.start_unanchored_id = new_start_uid;
        <span class="self">self</span>.nfa.special.start_anchored_id = new_start_aid;
        <span class="comment">// If one start state is a match state, then they both are.
        </span><span class="kw">if </span><span class="self">self</span>.nfa.states[<span class="self">self</span>.nfa.special.start_anchored_id].is_match() {
            <span class="self">self</span>.nfa.special.max_match_id = <span class="self">self</span>.nfa.special.start_anchored_id;
        }
        remapper.remap(<span class="kw-2">&amp;mut </span><span class="self">self</span>.nfa);
    }

    <span class="doccomment">/// Attempts to convert the transition representation of a subset of states
    /// in this NFA from sparse to dense. This can greatly improve search
    /// performance since states with a higher number of transitions tend to
    /// correlate with very active states.
    ///
    /// We generally only densify states that are close to the start state.
    /// These tend to be the most active states and thus benefit from a dense
    /// representation more than other states.
    ///
    /// This tends to best balance between memory usage and performance. In
    /// particular, the *vast majority* of all states in a typical Aho-Corasick
    /// automaton have only 1 transition and are usually farther from the start
    /// state and thus don't get densified.
    ///
    /// Note that this doesn't remove the sparse representation of transitions
    /// for states that are densified. It could be done, but actually removing
    /// entries from `NFA::sparse` is likely more expensive than it's worth.
    </span><span class="kw">fn </span>densify(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="self">self</span>.nfa.states.len() {
            <span class="kw">let </span>sid = StateID::new(i).unwrap();
            <span class="comment">// Don't bother densifying states that are only used as sentinels.
            </span><span class="kw">if </span>sid == NFA::DEAD || sid == NFA::FAIL {
                <span class="kw">continue</span>;
            }
            <span class="comment">// Only densify states that are "close enough" to the start state.
            </span><span class="kw">if </span><span class="self">self</span>.nfa.states[sid].depth.as_usize()
                &gt;= <span class="self">self</span>.builder.dense_depth
            {
                <span class="kw">continue</span>;
            }
            <span class="kw">let </span>dense = <span class="self">self</span>.nfa.alloc_dense_state()<span class="question-mark">?</span>;
            <span class="kw">let </span><span class="kw-2">mut </span>prev_link = <span class="prelude-val">None</span>;
            <span class="kw">while let </span><span class="prelude-val">Some</span>(link) = <span class="self">self</span>.nfa.next_link(sid, prev_link) {
                prev_link = <span class="prelude-val">Some</span>(link);
                <span class="kw">let </span>t = <span class="self">self</span>.nfa.sparse[link];

                <span class="kw">let </span>class = usize::from(<span class="self">self</span>.nfa.byte_classes.get(t.byte));
                <span class="kw">let </span>index = dense.as_usize() + class;
                <span class="self">self</span>.nfa.dense[index] = t.next;
            }
            <span class="self">self</span>.nfa.states[sid].dense = dense;
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Returns a set that tracked queued states.
    ///
    /// This is only necessary when ASCII case insensitivity is enabled, since
    /// it is the only way to visit the same state twice. Otherwise, this
    /// returns an inert set that nevers adds anything and always reports
    /// `false` for every member test.
    </span><span class="kw">fn </span>queued_set(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; QueuedSet {
        <span class="kw">if </span><span class="self">self</span>.builder.ascii_case_insensitive {
            QueuedSet::active()
        } <span class="kw">else </span>{
            QueuedSet::inert()
        }
    }

    <span class="doccomment">/// Initializes the unanchored start state by making it dense. This is
    /// achieved by explicitly setting every transition to the FAIL state.
    /// This isn't necessary for correctness, since any missing transition is
    /// automatically assumed to be mapped to the FAIL state. We do this to
    /// make the unanchored starting state dense, and thus in turn make
    /// transition lookups on it faster. (Which is worth doing because it's
    /// the most active state.)
    </span><span class="kw">fn </span>init_unanchored_start_state(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="kw">let </span>start_uid = <span class="self">self</span>.nfa.special.start_unanchored_id;
        <span class="kw">let </span>start_aid = <span class="self">self</span>.nfa.special.start_anchored_id;
        <span class="self">self</span>.nfa.init_full_state(start_uid, NFA::FAIL)<span class="question-mark">?</span>;
        <span class="self">self</span>.nfa.init_full_state(start_aid, NFA::FAIL)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Setup the anchored start state by copying all of the transitions and
    /// matches from the unanchored starting state with one change: the failure
    /// transition is changed to the DEAD state, so that for any undefined
    /// transitions, the search will stop.
    </span><span class="kw">fn </span>set_anchored_start_state(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="kw">let </span>start_uid = <span class="self">self</span>.nfa.special.start_unanchored_id;
        <span class="kw">let </span>start_aid = <span class="self">self</span>.nfa.special.start_anchored_id;
        <span class="kw">let </span>(<span class="kw-2">mut </span>uprev_link, <span class="kw-2">mut </span>aprev_link) = (<span class="prelude-val">None</span>, <span class="prelude-val">None</span>);
        <span class="kw">loop </span>{
            <span class="kw">let </span>unext = <span class="self">self</span>.nfa.next_link(start_uid, uprev_link);
            <span class="kw">let </span>anext = <span class="self">self</span>.nfa.next_link(start_aid, aprev_link);
            <span class="kw">let </span>(ulink, alink) = <span class="kw">match </span>(unext, anext) {
                (<span class="prelude-val">Some</span>(ulink), <span class="prelude-val">Some</span>(alink)) =&gt; (ulink, alink),
                (<span class="prelude-val">None</span>, <span class="prelude-val">None</span>) =&gt; <span class="kw">break</span>,
                <span class="kw">_ </span>=&gt; <span class="macro">unreachable!</span>(),
            };
            uprev_link = <span class="prelude-val">Some</span>(ulink);
            aprev_link = <span class="prelude-val">Some</span>(alink);
            <span class="self">self</span>.nfa.sparse[alink].next = <span class="self">self</span>.nfa.sparse[ulink].next;
        }
        <span class="self">self</span>.nfa.copy_matches(start_uid, start_aid)<span class="question-mark">?</span>;
        <span class="comment">// This is the main difference between the unanchored and anchored
        // starting states. If a lookup on an anchored starting state fails,
        // then the search should stop.
        //
        // N.B. This assumes that the loop on the unanchored starting state
        // hasn't been created yet.
        </span><span class="self">self</span>.nfa.states[start_aid].fail = NFA::DEAD;
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Set the failure transitions on the start state to loop back to the
    /// start state. This effectively permits the Aho-Corasick automaton to
    /// match at any position. This is also required for finding the next
    /// state to terminate, namely, finding the next state should never return
    /// a fail_id.
    ///
    /// This must be done after building the initial trie, since trie
    /// construction depends on transitions to `fail_id` to determine whether a
    /// state already exists or not.
    </span><span class="kw">fn </span>add_unanchored_start_state_loop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">let </span>start_uid = <span class="self">self</span>.nfa.special.start_unanchored_id;
        <span class="kw">let </span><span class="kw-2">mut </span>prev_link = <span class="prelude-val">None</span>;
        <span class="kw">while let </span><span class="prelude-val">Some</span>(link) = <span class="self">self</span>.nfa.next_link(start_uid, prev_link) {
            prev_link = <span class="prelude-val">Some</span>(link);
            <span class="kw">if </span><span class="self">self</span>.nfa.sparse[link].next() == NFA::FAIL {
                <span class="self">self</span>.nfa.sparse[link].next = start_uid;
            }
        }
    }

    <span class="doccomment">/// Remove the start state loop by rewriting any transitions on the start
    /// state back to the start state with transitions to the dead state.
    ///
    /// The loop is only closed when two conditions are met: the start state
    /// is a match state and the match kind is leftmost-first or
    /// leftmost-longest.
    ///
    /// The reason for this is that under leftmost semantics, a start state
    /// that is also a match implies that we should never restart the search
    /// process. We allow normal transitions out of the start state, but if
    /// none exist, we transition to the dead state, which signals that
    /// searching should stop.
    </span><span class="kw">fn </span>close_start_state_loop_for_leftmost(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">let </span>start_uid = <span class="self">self</span>.nfa.special.start_unanchored_id;
        <span class="kw">let </span>start = <span class="kw-2">&amp;mut </span><span class="self">self</span>.nfa.states[start_uid];
        <span class="kw">let </span>dense = start.dense;
        <span class="kw">if </span><span class="self">self</span>.builder.match_kind.is_leftmost() &amp;&amp; start.is_match() {
            <span class="kw">let </span><span class="kw-2">mut </span>prev_link = <span class="prelude-val">None</span>;
            <span class="kw">while let </span><span class="prelude-val">Some</span>(link) = <span class="self">self</span>.nfa.next_link(start_uid, prev_link) {
                prev_link = <span class="prelude-val">Some</span>(link);
                <span class="kw">if </span><span class="self">self</span>.nfa.sparse[link].next() == start_uid {
                    <span class="self">self</span>.nfa.sparse[link].next = NFA::DEAD;
                    <span class="kw">if </span>dense != StateID::ZERO {
                        <span class="kw">let </span>b = <span class="self">self</span>.nfa.sparse[link].byte;
                        <span class="kw">let </span>class = usize::from(<span class="self">self</span>.nfa.byte_classes.get(b));
                        <span class="self">self</span>.nfa.dense[dense.as_usize() + class] = NFA::DEAD;
                    }
                }
            }
        }
    }

    <span class="doccomment">/// Sets all transitions on the dead state to point back to the dead state.
    /// Normally, missing transitions map back to the failure state, but the
    /// point of the dead state is to act as a sink that can never be escaped.
    </span><span class="kw">fn </span>add_dead_state_loop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="self">self</span>.nfa.init_full_state(NFA::DEAD, NFA::DEAD)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="doccomment">/// A set of state identifiers used to avoid revisiting the same state multiple
/// times when filling in failure transitions.
///
/// This set has an "inert" and an "active" mode. When inert, the set never
/// stores anything and always returns `false` for every member test. This is
/// useful to avoid the performance and memory overhead of maintaining this
/// set when it is not needed.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">struct </span>QueuedSet {
    set: <span class="prelude-ty">Option</span>&lt;BTreeSet&lt;StateID&gt;&gt;,
}

<span class="kw">impl </span>QueuedSet {
    <span class="doccomment">/// Return an inert set that returns `false` for every state ID membership
    /// test.
    </span><span class="kw">fn </span>inert() -&gt; QueuedSet {
        QueuedSet { set: <span class="prelude-val">None </span>}
    }

    <span class="doccomment">/// Return an active set that tracks state ID membership.
    </span><span class="kw">fn </span>active() -&gt; QueuedSet {
        QueuedSet { set: <span class="prelude-val">Some</span>(BTreeSet::new()) }
    }

    <span class="doccomment">/// Inserts the given state ID into this set. (If the set is inert, then
    /// this is a no-op.)
    </span><span class="kw">fn </span>insert(<span class="kw-2">&amp;mut </span><span class="self">self</span>, state_id: StateID) {
        <span class="kw">if let </span><span class="prelude-val">Some</span>(<span class="kw-2">ref mut </span>set) = <span class="self">self</span>.set {
            set.insert(state_id);
        }
    }

    <span class="doccomment">/// Returns true if and only if the given state ID is in this set. If the
    /// set is inert, this always returns false.
    </span><span class="kw">fn </span>contains(<span class="kw-2">&amp;</span><span class="self">self</span>, state_id: StateID) -&gt; bool {
        <span class="kw">match </span><span class="self">self</span>.set {
            <span class="prelude-val">None </span>=&gt; <span class="bool-val">false</span>,
            <span class="prelude-val">Some</span>(<span class="kw-2">ref </span>set) =&gt; set.contains(<span class="kw-2">&amp;</span>state_id),
        }
    }
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>NFA {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; core::fmt::Result {
        <span class="kw">use crate</span>::{
            automaton::{fmt_state_indicator, sparse_transitions},
            util::debug::DebugByte,
        };

        <span class="macro">writeln!</span>(f, <span class="string">"noncontiguous::NFA("</span>)<span class="question-mark">?</span>;
        <span class="kw">for </span>(sid, state) <span class="kw">in </span><span class="self">self</span>.states.iter().with_state_ids() {
            <span class="comment">// The FAIL state doesn't actually have space for a state allocated
            // for it, so we have to treat it as a special case.
            </span><span class="kw">if </span>sid == NFA::FAIL {
                <span class="macro">writeln!</span>(f, <span class="string">"F {:06}:"</span>, sid.as_usize())<span class="question-mark">?</span>;
                <span class="kw">continue</span>;
            }
            fmt_state_indicator(f, <span class="self">self</span>, sid)<span class="question-mark">?</span>;
            <span class="macro">write!</span>(
                f,
                <span class="string">"{:06}({:06}): "</span>,
                sid.as_usize(),
                state.fail.as_usize()
            )<span class="question-mark">?</span>;

            <span class="kw">let </span>it = sparse_transitions(
                <span class="self">self</span>.iter_trans(sid).map(|t| (t.byte, t.next)),
            )
            .enumerate();
            <span class="kw">for </span>(i, (start, end, sid)) <span class="kw">in </span>it {
                <span class="kw">if </span>i &gt; <span class="number">0 </span>{
                    <span class="macro">write!</span>(f, <span class="string">", "</span>)<span class="question-mark">?</span>;
                }
                <span class="kw">if </span>start == end {
                    <span class="macro">write!</span>(
                        f,
                        <span class="string">"{:?} =&gt; {:?}"</span>,
                        DebugByte(start),
                        sid.as_usize()
                    )<span class="question-mark">?</span>;
                } <span class="kw">else </span>{
                    <span class="macro">write!</span>(
                        f,
                        <span class="string">"{:?}-{:?} =&gt; {:?}"</span>,
                        DebugByte(start),
                        DebugByte(end),
                        sid.as_usize()
                    )<span class="question-mark">?</span>;
                }
            }

            <span class="macro">write!</span>(f, <span class="string">"\n"</span>)<span class="question-mark">?</span>;
            <span class="kw">if </span><span class="self">self</span>.is_match(sid) {
                <span class="macro">write!</span>(f, <span class="string">"         matches: "</span>)<span class="question-mark">?</span>;
                <span class="kw">for </span>(i, pid) <span class="kw">in </span><span class="self">self</span>.iter_matches(sid).enumerate() {
                    <span class="kw">if </span>i &gt; <span class="number">0 </span>{
                        <span class="macro">write!</span>(f, <span class="string">", "</span>)<span class="question-mark">?</span>;
                    }
                    <span class="macro">write!</span>(f, <span class="string">"{}"</span>, pid.as_usize())<span class="question-mark">?</span>;
                }
                <span class="macro">write!</span>(f, <span class="string">"\n"</span>)<span class="question-mark">?</span>;
            }
        }
        <span class="macro">writeln!</span>(f, <span class="string">"match kind: {:?}"</span>, <span class="self">self</span>.match_kind)<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"prefilter: {:?}"</span>, <span class="self">self</span>.prefilter.is_some())<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"state length: {:?}"</span>, <span class="self">self</span>.states.len())<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"pattern length: {:?}"</span>, <span class="self">self</span>.patterns_len())<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"shortest pattern length: {:?}"</span>, <span class="self">self</span>.min_pattern_len)<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"longest pattern length: {:?}"</span>, <span class="self">self</span>.max_pattern_len)<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"memory usage: {:?}"</span>, <span class="self">self</span>.memory_usage())<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">")"</span>)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }
}
</code></pre></div></section></main></body></html>