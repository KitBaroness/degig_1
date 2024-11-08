<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/aho-corasick-1.1.3/src/nfa/contiguous.rs`."><title>contiguous.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="aho_corasick" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../aho_corasick/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">/*!
Provides a contiguous NFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a contiguous NFA directly. Using an `NFA` directly is typically only
necessary when one needs access to the [`Automaton`] trait implementation.
*/

</span><span class="kw">use </span>alloc::{vec, vec::Vec};

<span class="kw">use crate</span>::{
    automaton::Automaton,
    nfa::noncontiguous,
    util::{
        alphabet::ByteClasses,
        error::{BuildError, MatchError},
        int::{Usize, U16, U32},
        prefilter::Prefilter,
        primitives::{IteratorIndexExt, PatternID, SmallIndex, StateID},
        search::{Anchored, MatchKind},
        special::Special,
    },
};

<span class="doccomment">/// A contiguous NFA implementation of Aho-Corasick.
///
/// When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
/// this type directly. Using an `NFA` directly is typically only necessary
/// when one needs access to the [`Automaton`] trait implementation.
///
/// This NFA can only be built by first constructing a [`noncontiguous::NFA`].
/// Both [`NFA::new`] and [`Builder::build`] do this for you automatically, but
/// [`Builder::build_from_noncontiguous`] permits doing it explicitly.
///
/// The main difference between a noncontiguous NFA and a contiguous NFA is
/// that the latter represents all of its states and transitions in a single
/// allocation, where as the former uses a separate allocation for each state.
/// Doing this at construction time while keeping a low memory footprint isn't
/// feasible, which is primarily why there are two different NFA types: one
/// that does the least amount of work possible to build itself, and another
/// that does a little extra work to compact itself and make state transitions
/// faster by making some states use a dense representation.
///
/// Because a contiguous NFA uses a single allocation, there is a lot more
/// opportunity for compression tricks to reduce the heap memory used. Indeed,
/// it is not uncommon for a contiguous NFA to use an order of magnitude less
/// heap memory than a noncontiguous NFA. Since building a contiguous NFA
/// usually only takes a fraction of the time it takes to build a noncontiguous
/// NFA, the overall build time is not much slower. Thus, in most cases, a
/// contiguous NFA is the best choice.
///
/// Since a contiguous NFA uses various tricks for compression and to achieve
/// faster state transitions, currently, its limit on the number of states
/// is somewhat smaller than what a noncontiguous NFA can achieve. Generally
/// speaking, you shouldn't expect to run into this limit if the number of
/// patterns is under 1 million. It is plausible that this limit will be
/// increased in the future. If the limit is reached, building a contiguous NFA
/// will return an error. Often, since building a contiguous NFA is relatively
/// cheap, it can make sense to always try it even if you aren't sure if it
/// will fail or not. If it does, you can always fall back to a noncontiguous
/// NFA. (Indeed, the main [`AhoCorasick`](crate::AhoCorasick) type employs a
/// strategy similar to this at construction time.)
///
/// # Example
///
/// This example shows how to build an `NFA` directly and use it to execute
/// [`Automaton::try_find`]:
///
/// ```
/// use aho_corasick::{
///     automaton::Automaton,
///     nfa::contiguous::NFA,
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
    <span class="doccomment">/// The raw NFA representation. Each state is packed with a header
    /// (containing the format of the state, the failure transition and, for
    /// a sparse state, the number of transitions), its transitions and any
    /// matching pattern IDs for match states.
    </span>repr: Vec&lt;u32&gt;,
    <span class="doccomment">/// The length of each pattern. This is used to compute the start offset
    /// of a match.
    </span>pattern_lens: Vec&lt;SmallIndex&gt;,
    <span class="doccomment">/// The total number of states in this NFA.
    </span>state_len: usize,
    <span class="doccomment">/// A prefilter for accelerating searches, if one exists.
    </span>prefilter: <span class="prelude-ty">Option</span>&lt;Prefilter&gt;,
    <span class="doccomment">/// The match semantics built into this NFA.
    </span>match_kind: MatchKind,
    <span class="doccomment">/// The alphabet size, or total number of equivalence classes, for this
    /// NFA. Dense states always have this many transitions.
    </span>alphabet_len: usize,
    <span class="doccomment">/// The equivalence classes for this NFA. All transitions, dense and
    /// sparse, are defined on equivalence classes and not on the 256 distinct
    /// byte values.
    </span>byte_classes: ByteClasses,
    <span class="doccomment">/// The length of the shortest pattern in this automaton.
    </span>min_pattern_len: usize,
    <span class="doccomment">/// The length of the longest pattern in this automaton.
    </span>max_pattern_len: usize,
    <span class="doccomment">/// The information required to deduce which states are "special" in this
    /// NFA.
    </span>special: Special,
}

<span class="kw">impl </span>NFA {
    <span class="doccomment">/// Create a new Aho-Corasick contiguous NFA using the default
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

    <span class="doccomment">/// A convenience method for returning a new Aho-Corasick contiguous NFA
    /// builder.
    ///
    /// This usually permits one to just import the `NFA` type.
    </span><span class="kw">pub fn </span>builder() -&gt; Builder {
        Builder::new()
    }
}

<span class="kw">impl </span>NFA {
    <span class="doccomment">/// A sentinel state ID indicating that a search should stop once it has
    /// entered this state. When a search stops, it returns a match if one
    /// has been found, otherwise no match. A contiguous NFA always has an
    /// actual dead state at this ID.
    </span><span class="kw">const </span>DEAD: StateID = StateID::new_unchecked(<span class="number">0</span>);
    <span class="doccomment">/// Another sentinel state ID indicating that a search should move through
    /// current state's failure transition.
    ///
    /// Note that unlike DEAD, this does not actually point to a valid state
    /// in a contiguous NFA. (noncontiguous::NFA::FAIL does point to a valid
    /// state.) Instead, this points to the position that is guaranteed to
    /// never be a valid state ID (by making sure it points to a place in the
    /// middle of the encoding of the DEAD state). Since we never need to
    /// actually look at the FAIL state itself, this works out.
    ///
    /// By why do it this way? So that FAIL is a constant. I don't have any
    /// concrete evidence that this materially helps matters, but it's easy to
    /// do. The alternative would be making the FAIL ID point to the second
    /// state, which could be made a constant but is a little trickier to do.
    /// The easiest path is to just make the FAIL state a runtime value, but
    /// since comparisons with FAIL occur in perf critical parts of the search,
    /// we want it to be as tight as possible and not waste any registers.
    ///
    /// Very hand wavy... But the code complexity that results from this is
    /// very mild.
    </span><span class="kw">const </span>FAIL: StateID = StateID::new_unchecked(<span class="number">1</span>);
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
        <span class="kw">let </span>repr = <span class="kw-2">&amp;</span><span class="self">self</span>.repr;
        <span class="kw">let </span>class = <span class="self">self</span>.byte_classes.get(byte);
        <span class="kw">let </span>u32tosid = StateID::from_u32_unchecked;
        <span class="kw">loop </span>{
            <span class="kw">let </span>o = sid.as_usize();
            <span class="kw">let </span>kind = repr[o] &amp; <span class="number">0xFF</span>;
            <span class="comment">// I tried to encapsulate the "next transition" logic into its own
            // function, but it seemed to always result in sub-optimal codegen
            // that led to real and significant slowdowns. So we just inline
            // the logic here.
            //
            // I've also tried a lot of different ways to speed up this
            // routine, and most of them have failed.
            </span><span class="kw">if </span>kind == State::KIND_DENSE {
                <span class="kw">let </span>next = u32tosid(repr[o + <span class="number">2 </span>+ usize::from(class)]);
                <span class="kw">if </span>next != NFA::FAIL {
                    <span class="kw">return </span>next;
                }
            } <span class="kw">else if </span>kind == State::KIND_ONE {
                <span class="kw">if </span>class == repr[o].low_u16().high_u8() {
                    <span class="kw">return </span>u32tosid(repr[o + <span class="number">2</span>]);
                }
            } <span class="kw">else </span>{
                <span class="comment">// NOTE: I tried a SWAR technique in the loop below, but found
                // it slower. See the 'swar' test in the tests for this module.
                </span><span class="kw">let </span>trans_len = kind.as_usize();
                <span class="kw">let </span>classes_len = u32_len(trans_len);
                <span class="kw">let </span>trans_offset = o + <span class="number">2 </span>+ classes_len;
                <span class="kw">for </span>(i, <span class="kw-2">&amp;</span>chunk) <span class="kw">in
                    </span>repr[o + <span class="number">2</span>..][..classes_len].iter().enumerate()
                {
                    <span class="kw">let </span>classes = chunk.to_ne_bytes();
                    <span class="kw">if </span>classes[<span class="number">0</span>] == class {
                        <span class="kw">return </span>u32tosid(repr[trans_offset + i * <span class="number">4</span>]);
                    }
                    <span class="kw">if </span>classes[<span class="number">1</span>] == class {
                        <span class="kw">return </span>u32tosid(repr[trans_offset + i * <span class="number">4 </span>+ <span class="number">1</span>]);
                    }
                    <span class="kw">if </span>classes[<span class="number">2</span>] == class {
                        <span class="kw">return </span>u32tosid(repr[trans_offset + i * <span class="number">4 </span>+ <span class="number">2</span>]);
                    }
                    <span class="kw">if </span>classes[<span class="number">3</span>] == class {
                        <span class="kw">return </span>u32tosid(repr[trans_offset + i * <span class="number">4 </span>+ <span class="number">3</span>]);
                    }
                }
            }
            <span class="comment">// For an anchored search, we never follow failure transitions
            // because failure transitions lead us down a path to matching
            // a *proper* suffix of the path we were on. Thus, it can only
            // produce matches that appear after the beginning of the search.
            </span><span class="kw">if </span>anchored.is_anchored() {
                <span class="kw">return </span>NFA::DEAD;
            }
            sid = u32tosid(repr[o + <span class="number">1</span>]);
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
        !<span class="self">self</span>.is_dead(sid) &amp;&amp; sid &lt;= <span class="self">self</span>.special.max_match_id
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
        State::match_len(<span class="self">self</span>.alphabet_len, <span class="kw-2">&amp;</span><span class="self">self</span>.repr[sid.as_usize()..])
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>match_pattern(<span class="kw-2">&amp;</span><span class="self">self</span>, sid: StateID, index: usize) -&gt; PatternID {
        State::match_pattern(
            <span class="self">self</span>.alphabet_len,
            <span class="kw-2">&amp;</span><span class="self">self</span>.repr[sid.as_usize()..],
            index,
        )
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>memory_usage(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="kw">use </span>core::mem::size_of;

        (<span class="self">self</span>.repr.len() * size_of::&lt;u32&gt;())
            + (<span class="self">self</span>.pattern_lens.len() * size_of::&lt;SmallIndex&gt;())
            + <span class="self">self</span>.prefilter.as_ref().map_or(<span class="number">0</span>, |p| p.memory_usage())
    }

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>prefilter(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>Prefilter&gt; {
        <span class="self">self</span>.prefilter.as_ref()
    }
}

<span class="kw">impl </span>core::fmt::Debug <span class="kw">for </span>NFA {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter) -&gt; core::fmt::Result {
        <span class="kw">use </span><span class="kw">crate</span>::automaton::fmt_state_indicator;

        <span class="macro">writeln!</span>(f, <span class="string">"contiguous::NFA("</span>)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>sid = NFA::DEAD; <span class="comment">// always the first state and always present
        </span><span class="kw">loop </span>{
            <span class="kw">let </span>raw = <span class="kw-2">&amp;</span><span class="self">self</span>.repr[sid.as_usize()..];
            <span class="kw">if </span>raw.is_empty() {
                <span class="kw">break</span>;
            }
            <span class="kw">let </span>is_match = <span class="self">self</span>.is_match(sid);
            <span class="kw">let </span>state = State::read(<span class="self">self</span>.alphabet_len, is_match, raw);
            fmt_state_indicator(f, <span class="self">self</span>, sid)<span class="question-mark">?</span>;
            <span class="macro">write!</span>(
                f,
                <span class="string">"{:06}({:06}): "</span>,
                sid.as_usize(),
                state.fail.as_usize()
            )<span class="question-mark">?</span>;
            state.fmt(f)<span class="question-mark">?</span>;
            <span class="macro">write!</span>(f, <span class="string">"\n"</span>)<span class="question-mark">?</span>;
            <span class="kw">if </span><span class="self">self</span>.is_match(sid) {
                <span class="macro">write!</span>(f, <span class="string">"         matches: "</span>)<span class="question-mark">?</span>;
                <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..state.match_len {
                    <span class="kw">let </span>pid = State::match_pattern(<span class="self">self</span>.alphabet_len, raw, i);
                    <span class="kw">if </span>i &gt; <span class="number">0 </span>{
                        <span class="macro">write!</span>(f, <span class="string">", "</span>)<span class="question-mark">?</span>;
                    }
                    <span class="macro">write!</span>(f, <span class="string">"{}"</span>, pid.as_usize())<span class="question-mark">?</span>;
                }
                <span class="macro">write!</span>(f, <span class="string">"\n"</span>)<span class="question-mark">?</span>;
            }
            <span class="comment">// The FAIL state doesn't actually have space for a state allocated
            // for it, so we have to treat it as a special case. write below
            // the DEAD state.
            </span><span class="kw">if </span>sid == NFA::DEAD {
                <span class="macro">writeln!</span>(f, <span class="string">"F {:06}:"</span>, NFA::FAIL.as_usize())<span class="question-mark">?</span>;
            }
            <span class="kw">let </span>len = State::len(<span class="self">self</span>.alphabet_len, is_match, raw);
            sid = StateID::new(sid.as_usize().checked_add(len).unwrap())
                .unwrap();
        }
        <span class="macro">writeln!</span>(f, <span class="string">"match kind: {:?}"</span>, <span class="self">self</span>.match_kind)<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"prefilter: {:?}"</span>, <span class="self">self</span>.prefilter.is_some())<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"state length: {:?}"</span>, <span class="self">self</span>.state_len)<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"pattern length: {:?}"</span>, <span class="self">self</span>.patterns_len())<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"shortest pattern length: {:?}"</span>, <span class="self">self</span>.min_pattern_len)<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"longest pattern length: {:?}"</span>, <span class="self">self</span>.max_pattern_len)<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"alphabet length: {:?}"</span>, <span class="self">self</span>.alphabet_len)<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"byte classes: {:?}"</span>, <span class="self">self</span>.byte_classes)<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">"memory usage: {:?}"</span>, <span class="self">self</span>.memory_usage())<span class="question-mark">?</span>;
        <span class="macro">writeln!</span>(f, <span class="string">")"</span>)<span class="question-mark">?</span>;

        <span class="prelude-val">Ok</span>(())
    }
}

<span class="doccomment">/// The "in memory" representation a single dense or sparse state.
///
/// A `State`'s in memory representation is not ever actually materialized
/// during a search with a contiguous NFA. Doing so would be too slow. (Indeed,
/// the only time a `State` is actually constructed is in `Debug` impls.)
/// Instead, a `State` exposes a number of static methods for reading certain
/// things from the raw binary encoding of the state.
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">struct </span>State&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// The state to transition to when 'class_to_next' yields a transition
    /// to the FAIL state.
    </span>fail: StateID,
    <span class="doccomment">/// The number of pattern IDs in this state. For a non-match state, this is
    /// always zero. Otherwise it is always bigger than zero.
    </span>match_len: usize,
    <span class="doccomment">/// The sparse or dense representation of the transitions for this state.
    </span>trans: StateTrans&lt;<span class="lifetime">'a</span>&gt;,
}

<span class="doccomment">/// The underlying representation of sparse or dense transitions for a state.
///
/// Note that like `State`, we don't typically construct values of this type
/// during a search since we don't always need all values and thus would
/// represent a lot of wasteful work.
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">enum </span>StateTrans&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// A sparse representation of transitions for a state, where only non-FAIL
    /// transitions are explicitly represented.
    </span>Sparse {
        classes: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u32],
        <span class="doccomment">/// The transitions for this state, where each transition is packed
        /// into a u32. The low 8 bits correspond to the byte class for the
        /// transition, and the high 24 bits correspond to the next state ID.
        ///
        /// This packing is why the max state ID allowed for a contiguous
        /// NFA is 2^24-1.
        </span>nexts: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u32],
    },
    <span class="doccomment">/// A "one transition" state that is never a match state.
    ///
    /// These are by far the most common state, so we use a specialized and
    /// very compact representation for them.
    </span>One {
        <span class="doccomment">/// The element of this NFA's alphabet that this transition is
        /// defined for.
        </span>class: u8,
        <span class="doccomment">/// The state this should transition to if the current symbol is
        /// equal to 'class'.
        </span>next: u32,
    },
    <span class="doccomment">/// A dense representation of transitions for a state, where all
    /// transitions are explicitly represented, including transitions to the
    /// FAIL state.
    </span>Dense {
        <span class="doccomment">/// A dense set of transitions to other states. The transitions may
        /// point to a FAIL state, in which case, the search should try the
        /// same transition lookup at 'fail'.
        ///
        /// Note that this is indexed by byte equivalence classes and not
        /// byte values. That means 'class_to_next[byte]' is wrong and
        /// 'class_to_next[classes.get(byte)]' is correct. The number of
        /// transitions is always equivalent to 'classes.alphabet_len()'.
        </span>class_to_next: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u32],
    },
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; State&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// The offset of where the "kind" of a state is stored. If it isn't one
    /// of the sentinel values below, then it's a sparse state and the kind
    /// corresponds to the number of transitions in the state.
    </span><span class="kw">const </span>KIND: usize = <span class="number">0</span>;

    <span class="doccomment">/// A sentinel value indicating that the state uses a dense representation.
    </span><span class="kw">const </span>KIND_DENSE: u32 = <span class="number">0xFF</span>;
    <span class="doccomment">/// A sentinel value indicating that the state uses a special "one
    /// transition" encoding. In practice, non-match states with one transition
    /// make up the overwhelming majority of all states in any given
    /// Aho-Corasick automaton, so we can specialize them using a very compact
    /// representation.
    </span><span class="kw">const </span>KIND_ONE: u32 = <span class="number">0xFE</span>;

    <span class="doccomment">/// The maximum number of transitions to encode as a sparse state. Usually
    /// states with a lot of transitions are either very rare, or occur near
    /// the start state. In the latter case, they are probably dense already
    /// anyway. In the former case, making them dense is fine because they're
    /// rare.
    ///
    /// This needs to be small enough to permit each of the sentinel values for
    /// 'KIND' above. Namely, a sparse state embeds the number of transitions
    /// into the 'KIND'. Basically, "sparse" is a state kind too, but it's the
    /// "else" branch.
    ///
    /// N.B. There isn't anything particularly magical about 127 here. I
    /// just picked it because I figured any sparse state with this many
    /// transitions is going to be exceptionally rare, and if it did have this
    /// many transitions, then it would be quite slow to do a linear scan on
    /// the transitions during a search anyway.
    </span><span class="kw">const </span>MAX_SPARSE_TRANSITIONS: usize = <span class="number">127</span>;

    <span class="doccomment">/// Remap state IDs in-place.
    ///
    /// `state` should be the the raw binary encoding of a state. (The start
    /// of the slice must correspond to the start of the state, but the slice
    /// may extend past the end of the encoding of the state.)
    </span><span class="kw">fn </span>remap(
        alphabet_len: usize,
        old_to_new: <span class="kw-2">&amp;</span>[StateID],
        state: <span class="kw-2">&amp;mut </span>[u32],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="kw">let </span>kind = State::kind(state);
        <span class="kw">if </span>kind == State::KIND_DENSE {
            state[<span class="number">1</span>] = old_to_new[state[<span class="number">1</span>].as_usize()].as_u32();
            <span class="kw">for </span>next <span class="kw">in </span>state[<span class="number">2</span>..][..alphabet_len].iter_mut() {
                <span class="kw-2">*</span>next = old_to_new[next.as_usize()].as_u32();
            }
        } <span class="kw">else if </span>kind == State::KIND_ONE {
            state[<span class="number">1</span>] = old_to_new[state[<span class="number">1</span>].as_usize()].as_u32();
            state[<span class="number">2</span>] = old_to_new[state[<span class="number">2</span>].as_usize()].as_u32();
        } <span class="kw">else </span>{
            <span class="kw">let </span>trans_len = State::sparse_trans_len(state);
            <span class="kw">let </span>classes_len = u32_len(trans_len);
            state[<span class="number">1</span>] = old_to_new[state[<span class="number">1</span>].as_usize()].as_u32();
            <span class="kw">for </span>next <span class="kw">in </span>state[<span class="number">2 </span>+ classes_len..][..trans_len].iter_mut() {
                <span class="kw-2">*</span>next = old_to_new[next.as_usize()].as_u32();
            }
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Returns the length, in number of u32s, of this state.
    ///
    /// This is useful for reading states consecutively, e.g., in the Debug
    /// impl without needing to store a separate map from state index to state
    /// identifier.
    ///
    /// `state` should be the the raw binary encoding of a state. (The start
    /// of the slice must correspond to the start of the state, but the slice
    /// may extend past the end of the encoding of the state.)
    </span><span class="kw">fn </span>len(alphabet_len: usize, is_match: bool, state: <span class="kw-2">&amp;</span>[u32]) -&gt; usize {
        <span class="kw">let </span>kind_len = <span class="number">1</span>;
        <span class="kw">let </span>fail_len = <span class="number">1</span>;
        <span class="kw">let </span>kind = State::kind(state);
        <span class="kw">let </span>(classes_len, trans_len) = <span class="kw">if </span>kind == State::KIND_DENSE {
            (<span class="number">0</span>, alphabet_len)
        } <span class="kw">else if </span>kind == State::KIND_ONE {
            (<span class="number">0</span>, <span class="number">1</span>)
        } <span class="kw">else </span>{
            <span class="kw">let </span>trans_len = State::sparse_trans_len(state);
            <span class="kw">let </span>classes_len = u32_len(trans_len);
            (classes_len, trans_len)
        };
        <span class="kw">let </span>match_len = <span class="kw">if </span>!is_match {
            <span class="number">0
        </span>} <span class="kw">else if </span>State::match_len(alphabet_len, state) == <span class="number">1 </span>{
            <span class="comment">// This is a special case because when there is one pattern ID for
            // a match state, it is represented by a single u32 with its high
            // bit set (which is impossible for a valid pattern ID).
            </span><span class="number">1
        </span>} <span class="kw">else </span>{
            <span class="comment">// We add 1 to include the u32 that indicates the number of
            // pattern IDs that follow.
            </span><span class="number">1 </span>+ State::match_len(alphabet_len, state)
        };
        kind_len + fail_len + classes_len + trans_len + match_len
    }

    <span class="doccomment">/// Returns the kind of this state.
    ///
    /// This only includes the low byte.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>kind(state: <span class="kw-2">&amp;</span>[u32]) -&gt; u32 {
        state[State::KIND] &amp; <span class="number">0xFF
    </span>}

    <span class="doccomment">/// Get the number of sparse transitions in this state. This can never
    /// be more than State::MAX_SPARSE_TRANSITIONS, as all states with more
    /// transitions are encoded as dense states.
    ///
    /// `state` should be the the raw binary encoding of a sparse state. (The
    /// start of the slice must correspond to the start of the state, but the
    /// slice may extend past the end of the encoding of the state.) If this
    /// isn't a sparse state, then the return value is unspecified.
    ///
    /// Do note that this is only legal to call on a sparse state. So for
    /// example, "one transition" state is not a sparse state, so it would not
    /// be legal to call this method on such a state.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>sparse_trans_len(state: <span class="kw-2">&amp;</span>[u32]) -&gt; usize {
        (state[State::KIND] &amp; <span class="number">0xFF</span>).as_usize()
    }

    <span class="doccomment">/// Returns the total number of matching pattern IDs in this state. Calling
    /// this on a state that isn't a match results in unspecified behavior.
    /// Thus, the returned number is never 0 for all correct calls.
    ///
    /// `state` should be the the raw binary encoding of a state. (The start
    /// of the slice must correspond to the start of the state, but the slice
    /// may extend past the end of the encoding of the state.)
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>match_len(alphabet_len: usize, state: <span class="kw-2">&amp;</span>[u32]) -&gt; usize {
        <span class="comment">// We don't need to handle KIND_ONE here because it can never be a
        // match state.
        </span><span class="kw">let </span>packed = <span class="kw">if </span>State::kind(state) == State::KIND_DENSE {
            <span class="kw">let </span>start = <span class="number">2 </span>+ alphabet_len;
            state[start].as_usize()
        } <span class="kw">else </span>{
            <span class="kw">let </span>trans_len = State::sparse_trans_len(state);
            <span class="kw">let </span>classes_len = u32_len(trans_len);
            <span class="kw">let </span>start = <span class="number">2 </span>+ classes_len + trans_len;
            state[start].as_usize()
        };
        <span class="kw">if </span>packed &amp; (<span class="number">1 </span>&lt;&lt; <span class="number">31</span>) == <span class="number">0 </span>{
            packed
        } <span class="kw">else </span>{
            <span class="number">1
        </span>}
    }

    <span class="doccomment">/// Returns the pattern ID corresponding to the given index for the state
    /// given. The `index` provided must be less than the number of pattern IDs
    /// in this state.
    ///
    /// `state` should be the the raw binary encoding of a state. (The start of
    /// the slice must correspond to the start of the state, but the slice may
    /// extend past the end of the encoding of the state.)
    ///
    /// If the given state is not a match state or if the index is out of
    /// bounds, then this has unspecified behavior.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>match_pattern(
        alphabet_len: usize,
        state: <span class="kw-2">&amp;</span>[u32],
        index: usize,
    ) -&gt; PatternID {
        <span class="comment">// We don't need to handle KIND_ONE here because it can never be a
        // match state.
        </span><span class="kw">let </span>start = <span class="kw">if </span>State::kind(state) == State::KIND_DENSE {
            <span class="number">2 </span>+ alphabet_len
        } <span class="kw">else </span>{
            <span class="kw">let </span>trans_len = State::sparse_trans_len(state);
            <span class="kw">let </span>classes_len = u32_len(trans_len);
            <span class="number">2 </span>+ classes_len + trans_len
        };
        <span class="kw">let </span>packed = state[start];
        <span class="kw">let </span>pid = <span class="kw">if </span>packed &amp; (<span class="number">1 </span>&lt;&lt; <span class="number">31</span>) == <span class="number">0 </span>{
            state[start + <span class="number">1 </span>+ index]
        } <span class="kw">else </span>{
            <span class="macro">assert_eq!</span>(<span class="number">0</span>, index);
            packed &amp; !(<span class="number">1 </span>&lt;&lt; <span class="number">31</span>)
        };
        PatternID::from_u32_unchecked(pid)
    }

    <span class="doccomment">/// Read a state's binary encoding to its in-memory representation.
    ///
    /// `alphabet_len` should be the total number of transitions defined for
    /// dense states.
    ///
    /// `is_match` should be true if this state is a match state and false
    /// otherwise.
    ///
    /// `state` should be the the raw binary encoding of a state. (The start
    /// of the slice must correspond to the start of the state, but the slice
    /// may extend past the end of the encoding of the state.)
    </span><span class="kw">fn </span>read(
        alphabet_len: usize,
        is_match: bool,
        state: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u32],
    ) -&gt; State&lt;<span class="lifetime">'a</span>&gt; {
        <span class="kw">let </span>kind = State::kind(state);
        <span class="kw">let </span>match_len =
            <span class="kw">if </span>!is_match { <span class="number">0 </span>} <span class="kw">else </span>{ State::match_len(alphabet_len, state) };
        <span class="kw">let </span>(trans, fail) = <span class="kw">if </span>kind == State::KIND_DENSE {
            <span class="kw">let </span>fail = StateID::from_u32_unchecked(state[<span class="number">1</span>]);
            <span class="kw">let </span>class_to_next = <span class="kw-2">&amp;</span>state[<span class="number">2</span>..][..alphabet_len];
            (StateTrans::Dense { class_to_next }, fail)
        } <span class="kw">else if </span>kind == State::KIND_ONE {
            <span class="kw">let </span>fail = StateID::from_u32_unchecked(state[<span class="number">1</span>]);
            <span class="kw">let </span>class = state[State::KIND].low_u16().high_u8();
            <span class="kw">let </span>next = state[<span class="number">2</span>];
            (StateTrans::One { class, next }, fail)
        } <span class="kw">else </span>{
            <span class="kw">let </span>fail = StateID::from_u32_unchecked(state[<span class="number">1</span>]);
            <span class="kw">let </span>trans_len = State::sparse_trans_len(state);
            <span class="kw">let </span>classes_len = u32_len(trans_len);
            <span class="kw">let </span>classes = <span class="kw-2">&amp;</span>state[<span class="number">2</span>..][..classes_len];
            <span class="kw">let </span>nexts = <span class="kw-2">&amp;</span>state[<span class="number">2 </span>+ classes_len..][..trans_len];
            (StateTrans::Sparse { classes, nexts }, fail)
        };
        State { fail, match_len, trans }
    }

    <span class="doccomment">/// Encode the "old" state from a noncontiguous NFA to its binary
    /// representation to the given `dst` slice. `classes` should be the byte
    /// classes computed for the noncontiguous NFA that the given state came
    /// from.
    ///
    /// This returns an error if `dst` became so big that `StateID`s can no
    /// longer be created for new states. Otherwise, it returns the state ID of
    /// the new state created.
    ///
    /// When `force_dense` is true, then the encoded state will always use a
    /// dense format. Otherwise, the choice between dense and sparse will be
    /// automatically chosen based on the old state.
    </span><span class="kw">fn </span>write(
        nnfa: <span class="kw-2">&amp;</span>noncontiguous::NFA,
        oldsid: StateID,
        old: <span class="kw-2">&amp;</span>noncontiguous::State,
        classes: <span class="kw-2">&amp;</span>ByteClasses,
        dst: <span class="kw-2">&amp;mut </span>Vec&lt;u32&gt;,
        force_dense: bool,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;StateID, BuildError&gt; {
        <span class="kw">let </span>sid = StateID::new(dst.len()).map_err(|e| {
            BuildError::state_id_overflow(StateID::MAX.as_u64(), e.attempted())
        })<span class="question-mark">?</span>;
        <span class="kw">let </span>old_len = nnfa.iter_trans(oldsid).count();
        <span class="comment">// For states with a lot of transitions, we might as well just make
        // them dense. These kinds of hot states tend to be very rare, so we're
        // okay with it. This also gives us more sentinels in the state's
        // 'kind', which lets us create different state kinds to save on
        // space.
        </span><span class="kw">let </span>kind = <span class="kw">if </span>force_dense || old_len &gt; State::MAX_SPARSE_TRANSITIONS {
            State::KIND_DENSE
        } <span class="kw">else if </span>old_len == <span class="number">1 </span>&amp;&amp; !old.is_match() {
            State::KIND_ONE
        } <span class="kw">else </span>{
            <span class="comment">// For a sparse state, the kind is just the number of transitions.
            </span>u32::try_from(old_len).unwrap()
        };
        <span class="kw">if </span>kind == State::KIND_DENSE {
            dst.push(kind);
            dst.push(old.fail().as_u32());
            State::write_dense_trans(nnfa, oldsid, classes, dst)<span class="question-mark">?</span>;
        } <span class="kw">else if </span>kind == State::KIND_ONE {
            <span class="kw">let </span>t = nnfa.iter_trans(oldsid).next().unwrap();
            <span class="kw">let </span>class = u32::from(classes.get(t.byte()));
            dst.push(kind | (class &lt;&lt; <span class="number">8</span>));
            dst.push(old.fail().as_u32());
            dst.push(t.next().as_u32());
        } <span class="kw">else </span>{
            dst.push(kind);
            dst.push(old.fail().as_u32());
            State::write_sparse_trans(nnfa, oldsid, classes, dst)<span class="question-mark">?</span>;
        }
        <span class="comment">// Now finally write the number of matches and the matches themselves.
        </span><span class="kw">if </span>old.is_match() {
            <span class="kw">let </span>matches_len = nnfa.iter_matches(oldsid).count();
            <span class="kw">if </span>matches_len == <span class="number">1 </span>{
                <span class="kw">let </span>pid = nnfa.iter_matches(oldsid).next().unwrap().as_u32();
                <span class="macro">assert_eq!</span>(<span class="number">0</span>, pid &amp; (<span class="number">1 </span>&lt;&lt; <span class="number">31</span>));
                dst.push((<span class="number">1 </span>&lt;&lt; <span class="number">31</span>) | pid);
            } <span class="kw">else </span>{
                <span class="macro">assert_eq!</span>(<span class="number">0</span>, matches_len &amp; (<span class="number">1 </span>&lt;&lt; <span class="number">31</span>));
                dst.push(matches_len.as_u32());
                dst.extend(nnfa.iter_matches(oldsid).map(|pid| pid.as_u32()));
            }
        }
        <span class="prelude-val">Ok</span>(sid)
    }

    <span class="doccomment">/// Encode the "old" state transitions from a noncontiguous NFA to its
    /// binary sparse representation to the given `dst` slice. `classes` should
    /// be the byte classes computed for the noncontiguous NFA that the given
    /// state came from.
    ///
    /// This returns an error if `dst` became so big that `StateID`s can no
    /// longer be created for new states.
    </span><span class="kw">fn </span>write_sparse_trans(
        nnfa: <span class="kw-2">&amp;</span>noncontiguous::NFA,
        oldsid: StateID,
        classes: <span class="kw-2">&amp;</span>ByteClasses,
        dst: <span class="kw-2">&amp;mut </span>Vec&lt;u32&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="kw">let </span>(<span class="kw-2">mut </span>chunk, <span class="kw-2">mut </span>len) = ([<span class="number">0</span>; <span class="number">4</span>], <span class="number">0</span>);
        <span class="kw">for </span>t <span class="kw">in </span>nnfa.iter_trans(oldsid) {
            chunk[len] = classes.get(t.byte());
            len += <span class="number">1</span>;
            <span class="kw">if </span>len == <span class="number">4 </span>{
                dst.push(u32::from_ne_bytes(chunk));
                chunk = [<span class="number">0</span>; <span class="number">4</span>];
                len = <span class="number">0</span>;
            }
        }
        <span class="kw">if </span>len &gt; <span class="number">0 </span>{
            <span class="comment">// In the case where the number of transitions isn't divisible
            // by 4, the last u32 chunk will have some left over room. In
            // this case, we "just" repeat the last equivalence class. By
            // doing this, we know the leftover faux transitions will never
            // be followed because if they were, it would have been followed
            // prior to it in the last equivalence class. This saves us some
            // branching in the search time state transition code.
            </span><span class="kw">let </span>repeat = chunk[len - <span class="number">1</span>];
            <span class="kw">while </span>len &lt; <span class="number">4 </span>{
                chunk[len] = repeat;
                len += <span class="number">1</span>;
            }
            dst.push(u32::from_ne_bytes(chunk));
        }
        <span class="kw">for </span>t <span class="kw">in </span>nnfa.iter_trans(oldsid) {
            dst.push(t.next().as_u32());
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Encode the "old" state transitions from a noncontiguous NFA to its
    /// binary dense representation to the given `dst` slice. `classes` should
    /// be the byte classes computed for the noncontiguous NFA that the given
    /// state came from.
    ///
    /// This returns an error if `dst` became so big that `StateID`s can no
    /// longer be created for new states.
    </span><span class="kw">fn </span>write_dense_trans(
        nnfa: <span class="kw-2">&amp;</span>noncontiguous::NFA,
        oldsid: StateID,
        classes: <span class="kw-2">&amp;</span>ByteClasses,
        dst: <span class="kw-2">&amp;mut </span>Vec&lt;u32&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), BuildError&gt; {
        <span class="comment">// Our byte classes let us shrink the size of our dense states to the
        // number of equivalence classes instead of just fixing it to 256.
        // Any non-explicitly defined transition is just a transition to the
        // FAIL state, so we fill that in first and then overwrite them with
        // explicitly defined transitions. (Most states probably only have one
        // or two explicitly defined transitions.)
        //
        // N.B. Remember that while building the contiguous NFA, we use state
        // IDs from the noncontiguous NFA. It isn't until we've added all
        // states that we go back and map noncontiguous IDs to contiguous IDs.
        </span><span class="kw">let </span>start = dst.len();
        dst.extend(
            core::iter::repeat(noncontiguous::NFA::FAIL.as_u32())
                .take(classes.alphabet_len()),
        );
        <span class="macro">assert!</span>(start &lt; dst.len(), <span class="string">"equivalence classes are never empty"</span>);
        <span class="kw">for </span>t <span class="kw">in </span>nnfa.iter_trans(oldsid) {
            dst[start + usize::from(classes.get(t.byte()))] =
                t.next().as_u32();
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Return an iterator over every explicitly defined transition in this
    /// state.
    </span><span class="kw">fn </span>transitions&lt;<span class="lifetime">'b</span>&gt;(<span class="kw-2">&amp;</span><span class="lifetime">'b </span><span class="self">self</span>) -&gt; <span class="kw">impl </span>Iterator&lt;Item = (u8, StateID)&gt; + <span class="lifetime">'b </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>i = <span class="number">0</span>;
        core::iter::from_fn(<span class="kw">move </span>|| <span class="kw">match </span><span class="self">self</span>.trans {
            StateTrans::Sparse { classes, nexts } =&gt; {
                <span class="kw">if </span>i &gt;= nexts.len() {
                    <span class="kw">return </span><span class="prelude-val">None</span>;
                }
                <span class="kw">let </span>chunk = classes[i / <span class="number">4</span>];
                <span class="kw">let </span>class = chunk.to_ne_bytes()[i % <span class="number">4</span>];
                <span class="kw">let </span>next = StateID::from_u32_unchecked(nexts[i]);
                i += <span class="number">1</span>;
                <span class="prelude-val">Some</span>((class, next))
            }
            StateTrans::One { class, next } =&gt; {
                <span class="kw">if </span>i == <span class="number">0 </span>{
                    i += <span class="number">1</span>;
                    <span class="prelude-val">Some</span>((class, StateID::from_u32_unchecked(next)))
                } <span class="kw">else </span>{
                    <span class="prelude-val">None
                </span>}
            }
            StateTrans::Dense { class_to_next } =&gt; {
                <span class="kw">if </span>i &gt;= class_to_next.len() {
                    <span class="kw">return </span><span class="prelude-val">None</span>;
                }
                <span class="kw">let </span>class = i.as_u8();
                <span class="kw">let </span>next = StateID::from_u32_unchecked(class_to_next[i]);
                i += <span class="number">1</span>;
                <span class="prelude-val">Some</span>((class, next))
            }
        })
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; core::fmt::Debug <span class="kw">for </span>State&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>core::fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; core::fmt::Result {
        <span class="kw">use crate</span>::{automaton::sparse_transitions, util::debug::DebugByte};

        <span class="kw">let </span>it = sparse_transitions(<span class="self">self</span>.transitions())
            <span class="comment">// Writing out all FAIL transitions is quite noisy. Instead, we
            // just require readers of the output to assume anything absent
            // maps to the FAIL transition.
            </span>.filter(|<span class="kw-2">&amp;</span>(<span class="kw">_</span>, <span class="kw">_</span>, sid)| sid != NFA::FAIL)
            .enumerate();
        <span class="kw">for </span>(i, (start, end, sid)) <span class="kw">in </span>it {
            <span class="kw">if </span>i &gt; <span class="number">0 </span>{
                <span class="macro">write!</span>(f, <span class="string">", "</span>)<span class="question-mark">?</span>;
            }
            <span class="kw">if </span>start == end {
                <span class="macro">write!</span>(f, <span class="string">"{:?} =&gt; {:?}"</span>, DebugByte(start), sid.as_usize())<span class="question-mark">?</span>;
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
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="doccomment">/// A builder for configuring an Aho-Corasick contiguous NFA.
///
/// This builder has a subset of the options available to a
/// [`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
/// their behavior is identical.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>Builder {
    noncontiguous: noncontiguous::Builder,
    dense_depth: usize,
    byte_classes: bool,
}

<span class="kw">impl </span>Default <span class="kw">for </span>Builder {
    <span class="kw">fn </span>default() -&gt; Builder {
        Builder {
            noncontiguous: noncontiguous::Builder::new(),
            dense_depth: <span class="number">2</span>,
            byte_classes: <span class="bool-val">true</span>,
        }
    }
}

<span class="kw">impl </span>Builder {
    <span class="doccomment">/// Create a new builder for configuring an Aho-Corasick contiguous NFA.
    </span><span class="kw">pub fn </span>new() -&gt; Builder {
        Builder::default()
    }

    <span class="doccomment">/// Build an Aho-Corasick contiguous NFA from the given iterator of
    /// patterns.
    ///
    /// A builder may be reused to create more NFAs.
    </span><span class="kw">pub fn </span>build&lt;I, P&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, patterns: I) -&gt; <span class="prelude-ty">Result</span>&lt;NFA, BuildError&gt;
    <span class="kw">where
        </span>I: IntoIterator&lt;Item = P&gt;,
        P: AsRef&lt;[u8]&gt;,
    {
        <span class="kw">let </span>nnfa = <span class="self">self</span>.noncontiguous.build(patterns)<span class="question-mark">?</span>;
        <span class="self">self</span>.build_from_noncontiguous(<span class="kw-2">&amp;</span>nnfa)
    }

    <span class="doccomment">/// Build an Aho-Corasick contiguous NFA from the given noncontiguous NFA.
    ///
    /// Note that when this method is used, only the `dense_depth` and
    /// `byte_classes` settings on this builder are respected. The other
    /// settings only apply to the initial construction of the Aho-Corasick
    /// automaton. Since using this method requires that initial construction
    /// has already completed, all settings impacting only initial construction
    /// are no longer relevant.
    </span><span class="kw">pub fn </span>build_from_noncontiguous(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        nnfa: <span class="kw-2">&amp;</span>noncontiguous::NFA,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;NFA, BuildError&gt; {
        <span class="macro">debug!</span>(<span class="string">"building contiguous NFA"</span>);
        <span class="kw">let </span>byte_classes = <span class="kw">if </span><span class="self">self</span>.byte_classes {
            nnfa.byte_classes().clone()
        } <span class="kw">else </span>{
            ByteClasses::singletons()
        };
        <span class="kw">let </span><span class="kw-2">mut </span>index_to_state_id = <span class="macro">vec!</span>[NFA::DEAD; nnfa.states().len()];
        <span class="kw">let </span><span class="kw-2">mut </span>nfa = NFA {
            repr: <span class="macro">vec!</span>[],
            pattern_lens: nnfa.pattern_lens_raw().to_vec(),
            state_len: nnfa.states().len(),
            prefilter: nnfa.prefilter().map(|p| p.clone()),
            match_kind: nnfa.match_kind(),
            alphabet_len: byte_classes.alphabet_len(),
            byte_classes,
            min_pattern_len: nnfa.min_pattern_len(),
            max_pattern_len: nnfa.max_pattern_len(),
            <span class="comment">// The special state IDs are set later.
            </span>special: Special::zero(),
        };
        <span class="kw">for </span>(oldsid, state) <span class="kw">in </span>nnfa.states().iter().with_state_ids() {
            <span class="comment">// We don't actually encode a fail state since it isn't necessary.
            // But we still want to make sure any FAIL ids are mapped
            // correctly.
            </span><span class="kw">if </span>oldsid == noncontiguous::NFA::FAIL {
                index_to_state_id[oldsid] = NFA::FAIL;
                <span class="kw">continue</span>;
            }
            <span class="kw">let </span>force_dense = state.depth().as_usize() &lt; <span class="self">self</span>.dense_depth;
            <span class="kw">let </span>newsid = State::write(
                nnfa,
                oldsid,
                state,
                <span class="kw-2">&amp;</span>nfa.byte_classes,
                <span class="kw-2">&amp;mut </span>nfa.repr,
                force_dense,
            )<span class="question-mark">?</span>;
            index_to_state_id[oldsid] = newsid;
        }
        <span class="kw">for </span><span class="kw-2">&amp;</span>newsid <span class="kw">in </span>index_to_state_id.iter() {
            <span class="kw">if </span>newsid == NFA::FAIL {
                <span class="kw">continue</span>;
            }
            <span class="kw">let </span>state = <span class="kw-2">&amp;mut </span>nfa.repr[newsid.as_usize()..];
            State::remap(nfa.alphabet_len, <span class="kw-2">&amp;</span>index_to_state_id, state)<span class="question-mark">?</span>;
        }
        <span class="comment">// Now that we've remapped all the IDs in our states, all that's left
        // is remapping the special state IDs.
        </span><span class="kw">let </span>remap = <span class="kw-2">&amp;</span>index_to_state_id;
        <span class="kw">let </span>old = nnfa.special();
        <span class="kw">let </span>new = <span class="kw-2">&amp;mut </span>nfa.special;
        new.max_special_id = remap[old.max_special_id];
        new.max_match_id = remap[old.max_match_id];
        new.start_unanchored_id = remap[old.start_unanchored_id];
        new.start_anchored_id = remap[old.start_anchored_id];
        <span class="macro">debug!</span>(
            <span class="string">"contiguous NFA built, &lt;states: {:?}, size: {:?}, \
             alphabet len: {:?}&gt;"</span>,
            nfa.state_len,
            nfa.memory_usage(),
            nfa.byte_classes.alphabet_len(),
        );
        <span class="comment">// The vectors can grow ~twice as big during construction because a
        // Vec amortizes growth. But here, let's shrink things back down to
        // what we actually need since we're never going to add more to it.
        </span>nfa.repr.shrink_to_fit();
        nfa.pattern_lens.shrink_to_fit();
        <span class="prelude-val">Ok</span>(nfa)
    }

    <span class="doccomment">/// Set the desired match semantics.
    ///
    /// This only applies when using [`Builder::build`] and not
    /// [`Builder::build_from_noncontiguous`].
    ///
    /// See
    /// [`AhoCorasickBuilder::match_kind`](crate::AhoCorasickBuilder::match_kind)
    /// for more documentation and examples.
    </span><span class="kw">pub fn </span>match_kind(<span class="kw-2">&amp;mut </span><span class="self">self</span>, kind: MatchKind) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.noncontiguous.match_kind(kind);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Enable ASCII-aware case insensitive matching.
    ///
    /// This only applies when using [`Builder::build`] and not
    /// [`Builder::build_from_noncontiguous`].
    ///
    /// See
    /// [`AhoCorasickBuilder::ascii_case_insensitive`](crate::AhoCorasickBuilder::ascii_case_insensitive)
    /// for more documentation and examples.
    </span><span class="kw">pub fn </span>ascii_case_insensitive(<span class="kw-2">&amp;mut </span><span class="self">self</span>, yes: bool) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.noncontiguous.ascii_case_insensitive(yes);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Enable heuristic prefilter optimizations.
    ///
    /// This only applies when using [`Builder::build`] and not
    /// [`Builder::build_from_noncontiguous`].
    ///
    /// See
    /// [`AhoCorasickBuilder::prefilter`](crate::AhoCorasickBuilder::prefilter)
    /// for more documentation and examples.
    </span><span class="kw">pub fn </span>prefilter(<span class="kw-2">&amp;mut </span><span class="self">self</span>, yes: bool) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.noncontiguous.prefilter(yes);
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

    <span class="doccomment">/// A debug setting for whether to attempt to shrink the size of the
    /// automaton's alphabet or not.
    ///
    /// This should never be enabled unless you're debugging an automaton.
    /// Namely, disabling byte classes makes transitions easier to reason
    /// about, since they use the actual bytes instead of equivalence classes.
    /// Disabling this confers no performance benefit at search time.
    ///
    /// See
    /// [`AhoCorasickBuilder::byte_classes`](crate::AhoCorasickBuilder::byte_classes)
    /// for more documentation and examples.
    </span><span class="kw">pub fn </span>byte_classes(<span class="kw-2">&amp;mut </span><span class="self">self</span>, yes: bool) -&gt; <span class="kw-2">&amp;mut </span>Builder {
        <span class="self">self</span>.byte_classes = yes;
        <span class="self">self
    </span>}
}

<span class="doccomment">/// Computes the number of u32 values needed to represent one byte per the
/// number of transitions given.
</span><span class="kw">fn </span>u32_len(ntrans: usize) -&gt; usize {
    <span class="kw">if </span>ntrans % <span class="number">4 </span>== <span class="number">0 </span>{
        ntrans &gt;&gt; <span class="number">2
    </span>} <span class="kw">else </span>{
        (ntrans &gt;&gt; <span class="number">2</span>) + <span class="number">1
    </span>}
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="comment">// This test demonstrates a SWAR technique I tried in the sparse transition
    // code inside of 'next_state'. Namely, sparse transitions work by
    // iterating over u32 chunks, with each chunk containing up to 4 classes
    // corresponding to 4 transitions. This SWAR technique lets us find a
    // matching transition without converting the u32 to a [u8; 4].
    //
    // It turned out to be a little slower unfortunately, which isn't too
    // surprising, since this is likely a throughput oriented optimization.
    // Loop unrolling doesn't really help us because the vast majority of
    // states have very few transitions.
    //
    // Anyway, this code was a little tricky to write, so I converted it to a
    // test in case someone figures out how to use it more effectively than
    // I could.
    //
    // (This also only works on little endian. So big endian would need to be
    // accounted for if we ever decided to use this I think.)
    </span><span class="attr">#[cfg(target_endian = <span class="string">"little"</span>)]
    #[test]
    </span><span class="kw">fn </span>swar() {
        <span class="kw">use super</span>::<span class="kw-2">*</span>;

        <span class="kw">fn </span>has_zero_byte(x: u32) -&gt; u32 {
            <span class="kw">const </span>LO_U32: u32 = <span class="number">0x01010101</span>;
            <span class="kw">const </span>HI_U32: u32 = <span class="number">0x80808080</span>;

            x.wrapping_sub(LO_U32) &amp; !x &amp; HI_U32
        }

        <span class="kw">fn </span>broadcast(b: u8) -&gt; u32 {
            (u32::from(b)) * (u32::MAX / <span class="number">255</span>)
        }

        <span class="kw">fn </span>index_of(x: u32) -&gt; usize {
            <span class="kw">let </span>o =
                (((x - <span class="number">1</span>) &amp; <span class="number">0x01010101</span>).wrapping_mul(<span class="number">0x01010101</span>) &gt;&gt; <span class="number">24</span>) - <span class="number">1</span>;
            o.as_usize()
        }

        <span class="kw">let </span>bytes: [u8; <span class="number">4</span>] = [<span class="string">b'1'</span>, <span class="string">b'A'</span>, <span class="string">b'a'</span>, <span class="string">b'z'</span>];
        <span class="kw">let </span>chunk = u32::from_ne_bytes(bytes);

        <span class="kw">let </span>needle = broadcast(<span class="string">b'1'</span>);
        <span class="macro">assert_eq!</span>(<span class="number">0</span>, index_of(has_zero_byte(needle ^ chunk)));
        <span class="kw">let </span>needle = broadcast(<span class="string">b'A'</span>);
        <span class="macro">assert_eq!</span>(<span class="number">1</span>, index_of(has_zero_byte(needle ^ chunk)));
        <span class="kw">let </span>needle = broadcast(<span class="string">b'a'</span>);
        <span class="macro">assert_eq!</span>(<span class="number">2</span>, index_of(has_zero_byte(needle ^ chunk)));
        <span class="kw">let </span>needle = broadcast(<span class="string">b'z'</span>);
        <span class="macro">assert_eq!</span>(<span class="number">3</span>, index_of(has_zero_byte(needle ^ chunk)));
    }
}
</code></pre></div></section></main></body></html>