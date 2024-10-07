<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.18.9/src/alt_bn128/compression.rs`."><title>compression.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="solana_program" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../solana_program/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">pub mod </span>prelude {
    <span class="kw">pub use </span><span class="kw">crate</span>::alt_bn128::compression::{
        alt_bn128_compression_size::<span class="kw-2">*</span>, consts::<span class="kw-2">*</span>, target_arch::<span class="kw-2">*</span>, AltBn128CompressionError,
    };
}

<span class="kw">use </span>thiserror::Error;

<span class="kw">mod </span>consts {
    <span class="kw">pub const </span>ALT_BN128_G1_COMPRESS: u64 = <span class="number">0</span>;
    <span class="kw">pub const </span>ALT_BN128_G1_DECOMPRESS: u64 = <span class="number">1</span>;
    <span class="kw">pub const </span>ALT_BN128_G2_COMPRESS: u64 = <span class="number">2</span>;
    <span class="kw">pub const </span>ALT_BN128_G2_DECOMPRESS: u64 = <span class="number">3</span>;
}

<span class="kw">mod </span>alt_bn128_compression_size {
    <span class="kw">pub const </span>G1: usize = <span class="number">64</span>;
    <span class="kw">pub const </span>G2: usize = <span class="number">128</span>;
    <span class="kw">pub const </span>G1_COMPRESSED: usize = <span class="number">32</span>;
    <span class="kw">pub const </span>G2_COMPRESSED: usize = <span class="number">64</span>;
}

<span class="attr">#[derive(Debug, Error, Clone, PartialEq, Eq)]
</span><span class="kw">pub enum </span>AltBn128CompressionError {
    <span class="attr">#[error(<span class="string">"Unexpected error"</span>)]
    </span>UnexpectedError,
    <span class="attr">#[error(<span class="string">"Failed to decompress g1"</span>)]
    </span>G1DecompressionFailed,
    <span class="attr">#[error(<span class="string">"Failed to decompress g2"</span>)]
    </span>G2DecompressionFailed,
    <span class="attr">#[error(<span class="string">"Failed to compress affine g1"</span>)]
    </span>G1CompressionFailed,
    <span class="attr">#[error(<span class="string">"Failed to compress affine g2"</span>)]
    </span>G2CompressionFailed,
    <span class="attr">#[error(<span class="string">"Invalid input size"</span>)]
    </span>InvalidInputSize,
}

<span class="kw">impl </span>From&lt;u64&gt; <span class="kw">for </span>AltBn128CompressionError {
    <span class="kw">fn </span>from(v: u64) -&gt; AltBn128CompressionError {
        <span class="kw">match </span>v {
            <span class="number">1 </span>=&gt; AltBn128CompressionError::G1DecompressionFailed,
            <span class="number">2 </span>=&gt; AltBn128CompressionError::G2DecompressionFailed,
            <span class="number">3 </span>=&gt; AltBn128CompressionError::G1CompressionFailed,
            <span class="number">4 </span>=&gt; AltBn128CompressionError::G2CompressionFailed,
            <span class="number">5 </span>=&gt; AltBn128CompressionError::InvalidInputSize,
            <span class="kw">_ </span>=&gt; AltBn128CompressionError::UnexpectedError,
        }
    }
}

<span class="kw">impl </span>From&lt;AltBn128CompressionError&gt; <span class="kw">for </span>u64 {
    <span class="kw">fn </span>from(v: AltBn128CompressionError) -&gt; u64 {
        <span class="kw">match </span>v {
            AltBn128CompressionError::G1DecompressionFailed =&gt; <span class="number">1</span>,
            AltBn128CompressionError::G2DecompressionFailed =&gt; <span class="number">2</span>,
            AltBn128CompressionError::G1CompressionFailed =&gt; <span class="number">3</span>,
            AltBn128CompressionError::G2CompressionFailed =&gt; <span class="number">4</span>,
            AltBn128CompressionError::InvalidInputSize =&gt; <span class="number">5</span>,
            AltBn128CompressionError::UnexpectedError =&gt; <span class="number">0</span>,
        }
    }
}

<span class="attr">#[cfg(not(target_os = <span class="string">"solana"</span>))]
</span><span class="kw">mod </span>target_arch {

    <span class="kw">use </span>{
        <span class="kw">super</span>::<span class="kw-2">*</span>,
        <span class="kw">crate</span>::alt_bn128::compression::alt_bn128_compression_size,
        ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate},
    };

    <span class="kw">type </span>G1 = ark_bn254::g1::G1Affine;
    <span class="kw">type </span>G2 = ark_bn254::g2::G2Affine;

    <span class="kw">pub fn </span>alt_bn128_g1_decompress(
        g1_bytes: <span class="kw-2">&amp;</span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;[u8; alt_bn128_compression_size::G1], AltBn128CompressionError&gt; {
        <span class="kw">let </span>g1_bytes: [u8; alt_bn128_compression_size::G1_COMPRESSED] = g1_bytes
            .try_into()
            .map_err(|<span class="kw">_</span>| AltBn128CompressionError::InvalidInputSize)<span class="question-mark">?</span>;
        <span class="kw">if </span>g1_bytes == [<span class="number">0u8</span>; alt_bn128_compression_size::G1_COMPRESSED] {
            <span class="kw">return </span><span class="prelude-val">Ok</span>([<span class="number">0u8</span>; alt_bn128_compression_size::G1]);
        }
        <span class="kw">let </span>decompressed_g1 = G1::deserialize_with_mode(
            convert_endianness::&lt;<span class="number">32</span>, <span class="number">32</span>&gt;(<span class="kw-2">&amp;</span>g1_bytes).as_slice(),
            Compress::Yes,
            Validate::No,
        )
        .map_err(|<span class="kw">_</span>| AltBn128CompressionError::G1DecompressionFailed)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>decompressed_g1_bytes = [<span class="number">0u8</span>; alt_bn128_compression_size::G1];
        decompressed_g1
            .x
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>decompressed_g1_bytes[..<span class="number">32</span>], Compress::No)
            .map_err(|<span class="kw">_</span>| AltBn128CompressionError::G1DecompressionFailed)<span class="question-mark">?</span>;
        decompressed_g1
            .y
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>decompressed_g1_bytes[<span class="number">32</span>..], Compress::No)
            .map_err(|<span class="kw">_</span>| AltBn128CompressionError::G1DecompressionFailed)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(convert_endianness::&lt;<span class="number">32</span>, <span class="number">64</span>&gt;(<span class="kw-2">&amp;</span>decompressed_g1_bytes))
    }

    <span class="kw">pub fn </span>alt_bn128_g1_compress(
        g1_bytes: <span class="kw-2">&amp;</span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;[u8; alt_bn128_compression_size::G1_COMPRESSED], AltBn128CompressionError&gt; {
        <span class="kw">let </span>g1_bytes: [u8; alt_bn128_compression_size::G1] = g1_bytes
            .try_into()
            .map_err(|<span class="kw">_</span>| AltBn128CompressionError::InvalidInputSize)<span class="question-mark">?</span>;
        <span class="kw">if </span>g1_bytes == [<span class="number">0u8</span>; alt_bn128_compression_size::G1] {
            <span class="kw">return </span><span class="prelude-val">Ok</span>([<span class="number">0u8</span>; alt_bn128_compression_size::G1_COMPRESSED]);
        }
        <span class="kw">let </span>g1 = G1::deserialize_with_mode(
            convert_endianness::&lt;<span class="number">32</span>, <span class="number">64</span>&gt;(<span class="kw-2">&amp;</span>g1_bytes).as_slice(),
            Compress::No,
            Validate::No,
        )
        .map_err(|<span class="kw">_</span>| AltBn128CompressionError::G1CompressionFailed)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>g1_bytes = [<span class="number">0u8</span>; alt_bn128_compression_size::G1_COMPRESSED];
        G1::serialize_compressed(<span class="kw-2">&amp;</span>g1, g1_bytes.as_mut_slice())
            .map_err(|<span class="kw">_</span>| AltBn128CompressionError::G2CompressionFailed)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(convert_endianness::&lt;<span class="number">32</span>, <span class="number">32</span>&gt;(<span class="kw-2">&amp;</span>g1_bytes))
    }

    <span class="kw">pub fn </span>alt_bn128_g2_decompress(
        g2_bytes: <span class="kw-2">&amp;</span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;[u8; alt_bn128_compression_size::G2], AltBn128CompressionError&gt; {
        <span class="kw">let </span>g2_bytes: [u8; alt_bn128_compression_size::G2_COMPRESSED] = g2_bytes
            .try_into()
            .map_err(|<span class="kw">_</span>| AltBn128CompressionError::InvalidInputSize)<span class="question-mark">?</span>;
        <span class="kw">if </span>g2_bytes == [<span class="number">0u8</span>; alt_bn128_compression_size::G2_COMPRESSED] {
            <span class="kw">return </span><span class="prelude-val">Ok</span>([<span class="number">0u8</span>; alt_bn128_compression_size::G2]);
        }
        <span class="kw">let </span>decompressed_g2 =
            G2::deserialize_compressed(convert_endianness::&lt;<span class="number">64</span>, <span class="number">64</span>&gt;(<span class="kw-2">&amp;</span>g2_bytes).as_slice())
                .map_err(|<span class="kw">_</span>| AltBn128CompressionError::G2DecompressionFailed)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>decompressed_g2_bytes = [<span class="number">0u8</span>; alt_bn128_compression_size::G2];
        decompressed_g2
            .x
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>decompressed_g2_bytes[..<span class="number">64</span>], Compress::No)
            .map_err(|<span class="kw">_</span>| AltBn128CompressionError::G2DecompressionFailed)<span class="question-mark">?</span>;
        decompressed_g2
            .y
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>decompressed_g2_bytes[<span class="number">64</span>..<span class="number">128</span>], Compress::No)
            .map_err(|<span class="kw">_</span>| AltBn128CompressionError::G2DecompressionFailed)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(convert_endianness::&lt;<span class="number">64</span>, <span class="number">128</span>&gt;(<span class="kw-2">&amp;</span>decompressed_g2_bytes))
    }

    <span class="kw">pub fn </span>alt_bn128_g2_compress(
        g2_bytes: <span class="kw-2">&amp;</span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;[u8; alt_bn128_compression_size::G2_COMPRESSED], AltBn128CompressionError&gt; {
        <span class="kw">let </span>g2_bytes: [u8; alt_bn128_compression_size::G2] = g2_bytes
            .try_into()
            .map_err(|<span class="kw">_</span>| AltBn128CompressionError::InvalidInputSize)<span class="question-mark">?</span>;
        <span class="kw">if </span>g2_bytes == [<span class="number">0u8</span>; alt_bn128_compression_size::G2] {
            <span class="kw">return </span><span class="prelude-val">Ok</span>([<span class="number">0u8</span>; alt_bn128_compression_size::G2_COMPRESSED]);
        }
        <span class="kw">let </span>g2 = G2::deserialize_with_mode(
            convert_endianness::&lt;<span class="number">64</span>, <span class="number">128</span>&gt;(<span class="kw-2">&amp;</span>g2_bytes).as_slice(),
            Compress::No,
            Validate::No,
        )
        .map_err(|<span class="kw">_</span>| AltBn128CompressionError::G2DecompressionFailed)<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>g2_bytes = [<span class="number">0u8</span>; alt_bn128_compression_size::G2_COMPRESSED];
        G2::serialize_compressed(<span class="kw-2">&amp;</span>g2, g2_bytes.as_mut_slice())
            .map_err(|<span class="kw">_</span>| AltBn128CompressionError::G2CompressionFailed)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(convert_endianness::&lt;<span class="number">64</span>, <span class="number">64</span>&gt;(<span class="kw-2">&amp;</span>g2_bytes))
    }

    <span class="kw">pub fn </span>convert_endianness&lt;<span class="kw">const </span>CHUNK_SIZE: usize, <span class="kw">const </span>ARRAY_SIZE: usize&gt;(
        bytes: <span class="kw-2">&amp;</span>[u8; ARRAY_SIZE],
    ) -&gt; [u8; ARRAY_SIZE] {
        <span class="kw">let </span>reversed: [<span class="kw">_</span>; ARRAY_SIZE] = bytes
            .chunks_exact(CHUNK_SIZE)
            .flat_map(|chunk| chunk.iter().rev().copied())
            .enumerate()
            .fold([<span class="number">0u8</span>; ARRAY_SIZE], |<span class="kw-2">mut </span>acc, (i, v)| {
                acc[i] = v;
                acc
            });
        reversed
    }
}

<span class="attr">#[cfg(target_os = <span class="string">"solana"</span>)]
</span><span class="kw">mod </span>target_arch {
    <span class="kw">use </span>{
        <span class="kw">super</span>::<span class="kw-2">*</span>,
        alt_bn128_compression_size::{G1, G1_COMPRESSED, G2, G2_COMPRESSED},
        prelude::<span class="kw-2">*</span>,
    };

    <span class="kw">pub fn </span>alt_bn128_g1_compress(
        input: <span class="kw-2">&amp;</span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;[u8; G1_COMPRESSED], AltBn128CompressionError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>result_buffer = [<span class="number">0</span>; G1_COMPRESSED];
        <span class="kw">let </span>result = <span class="kw">unsafe </span>{
            <span class="kw">crate</span>::syscalls::sol_alt_bn128_compression(
                ALT_BN128_G1_COMPRESS,
                input <span class="kw">as </span><span class="kw-2">*const </span><span class="kw">_ as </span><span class="kw-2">*const </span>u8,
                input.len() <span class="kw">as </span>u64,
                <span class="kw-2">&amp;mut </span>result_buffer <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_ as </span><span class="kw-2">*mut </span>u8,
            )
        };

        <span class="kw">match </span>result {
            <span class="number">0 </span>=&gt; <span class="prelude-val">Ok</span>(result_buffer),
            error =&gt; <span class="prelude-val">Err</span>(AltBn128CompressionError::from(error)),
        }
    }

    <span class="kw">pub fn </span>alt_bn128_g1_decompress(input: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;[u8; G1], AltBn128CompressionError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>result_buffer = [<span class="number">0</span>; G1];
        <span class="kw">let </span>result = <span class="kw">unsafe </span>{
            <span class="kw">crate</span>::syscalls::sol_alt_bn128_compression(
                ALT_BN128_G1_DECOMPRESS,
                input <span class="kw">as </span><span class="kw-2">*const </span><span class="kw">_ as </span><span class="kw-2">*const </span>u8,
                input.len() <span class="kw">as </span>u64,
                <span class="kw-2">&amp;mut </span>result_buffer <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_ as </span><span class="kw-2">*mut </span>u8,
            )
        };

        <span class="kw">match </span>result {
            <span class="number">0 </span>=&gt; <span class="prelude-val">Ok</span>(result_buffer),
            error =&gt; <span class="prelude-val">Err</span>(AltBn128CompressionError::from(error)),
        }
    }

    <span class="kw">pub fn </span>alt_bn128_g2_compress(
        input: <span class="kw-2">&amp;</span>[u8],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;[u8; G2_COMPRESSED], AltBn128CompressionError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>result_buffer = [<span class="number">0</span>; G2_COMPRESSED];
        <span class="kw">let </span>result = <span class="kw">unsafe </span>{
            <span class="kw">crate</span>::syscalls::sol_alt_bn128_compression(
                ALT_BN128_G2_COMPRESS,
                input <span class="kw">as </span><span class="kw-2">*const </span><span class="kw">_ as </span><span class="kw-2">*const </span>u8,
                input.len() <span class="kw">as </span>u64,
                <span class="kw-2">&amp;mut </span>result_buffer <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_ as </span><span class="kw-2">*mut </span>u8,
            )
        };

        <span class="kw">match </span>result {
            <span class="number">0 </span>=&gt; <span class="prelude-val">Ok</span>(result_buffer),
            error =&gt; <span class="prelude-val">Err</span>(AltBn128CompressionError::from(error)),
        }
    }

    <span class="kw">pub fn </span>alt_bn128_g2_decompress(
        input: <span class="kw-2">&amp;</span>[u8; G2_COMPRESSED],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;[u8; G2], AltBn128CompressionError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>result_buffer = [<span class="number">0</span>; G2];
        <span class="kw">let </span>result = <span class="kw">unsafe </span>{
            <span class="kw">crate</span>::syscalls::sol_alt_bn128_compression(
                ALT_BN128_G2_DECOMPRESS,
                input <span class="kw">as </span><span class="kw-2">*const </span><span class="kw">_ as </span><span class="kw-2">*const </span>u8,
                input.len() <span class="kw">as </span>u64,
                <span class="kw-2">&amp;mut </span>result_buffer <span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_ as </span><span class="kw-2">*mut </span>u8,
            )
        };

        <span class="kw">match </span>result {
            <span class="number">0 </span>=&gt; <span class="prelude-val">Ok</span>(result_buffer),
            error =&gt; <span class="prelude-val">Err</span>(AltBn128CompressionError::from(error)),
        }
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span>{
        <span class="kw">super</span>::<span class="kw-2">*</span>,
        <span class="kw">crate</span>::alt_bn128::compression::target_arch::convert_endianness,
        ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate},
        std::ops::Neg,
        target_arch::{
            alt_bn128_g1_compress, alt_bn128_g1_decompress, alt_bn128_g2_compress,
            alt_bn128_g2_decompress,
        },
    };
    <span class="kw">type </span>G1 = ark_bn254::g1::G1Affine;
    <span class="kw">type </span>G2 = ark_bn254::g2::G2Affine;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>alt_bn128_g1_compression() {
        <span class="kw">let </span>g1_be = [
            <span class="number">45</span>, <span class="number">206</span>, <span class="number">255</span>, <span class="number">166</span>, <span class="number">152</span>, <span class="number">55</span>, <span class="number">128</span>, <span class="number">138</span>, <span class="number">79</span>, <span class="number">217</span>, <span class="number">145</span>, <span class="number">164</span>, <span class="number">25</span>, <span class="number">74</span>, <span class="number">120</span>, <span class="number">234</span>, <span class="number">234</span>, <span class="number">217</span>,
            <span class="number">68</span>, <span class="number">149</span>, <span class="number">162</span>, <span class="number">44</span>, <span class="number">133</span>, <span class="number">120</span>, <span class="number">184</span>, <span class="number">205</span>, <span class="number">12</span>, <span class="number">44</span>, <span class="number">175</span>, <span class="number">98</span>, <span class="number">168</span>, <span class="number">172</span>, <span class="number">20</span>, <span class="number">24</span>, <span class="number">216</span>, <span class="number">15</span>, <span class="number">209</span>,
            <span class="number">175</span>, <span class="number">106</span>, <span class="number">75</span>, <span class="number">147</span>, <span class="number">236</span>, <span class="number">90</span>, <span class="number">101</span>, <span class="number">123</span>, <span class="number">219</span>, <span class="number">245</span>, <span class="number">151</span>, <span class="number">209</span>, <span class="number">202</span>, <span class="number">218</span>, <span class="number">104</span>, <span class="number">148</span>, <span class="number">8</span>, <span class="number">32</span>,
            <span class="number">254</span>, <span class="number">243</span>, <span class="number">191</span>, <span class="number">218</span>, <span class="number">122</span>, <span class="number">42</span>, <span class="number">81</span>, <span class="number">193</span>, <span class="number">84</span>,
        ];
        <span class="kw">let </span>g1_le = convert_endianness::&lt;<span class="number">32</span>, <span class="number">64</span>&gt;(<span class="kw-2">&amp;</span>g1_be);
        <span class="kw">let </span>g1: G1 =
            G1::deserialize_with_mode(g1_le.as_slice(), Compress::No, Validate::No).unwrap();

        <span class="kw">let </span>g1_neg = g1.neg();
        <span class="kw">let </span><span class="kw-2">mut </span>g1_neg_be = [<span class="number">0u8</span>; <span class="number">64</span>];
        g1_neg
            .x
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>g1_neg_be[..<span class="number">32</span>], Compress::No)
            .unwrap();
        g1_neg
            .y
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>g1_neg_be[<span class="number">32</span>..<span class="number">64</span>], Compress::No)
            .unwrap();
        <span class="kw">let </span>g1_neg_be: [u8; <span class="number">64</span>] = convert_endianness::&lt;<span class="number">32</span>, <span class="number">64</span>&gt;(<span class="kw-2">&amp;</span>g1_neg_be);

        <span class="kw">let </span>points = [(g1, g1_be), (g1_neg, g1_neg_be)];

        <span class="kw">for </span>(point, g1_be) <span class="kw">in </span><span class="kw-2">&amp;</span>points {
            <span class="kw">let </span><span class="kw-2">mut </span>compressed_ref = [<span class="number">0u8</span>; <span class="number">32</span>];
            G1::serialize_with_mode(point, compressed_ref.as_mut_slice(), Compress::Yes).unwrap();
            <span class="kw">let </span>compressed_ref: [u8; <span class="number">32</span>] = convert_endianness::&lt;<span class="number">32</span>, <span class="number">32</span>&gt;(<span class="kw-2">&amp;</span>compressed_ref);

            <span class="kw">let </span>decompressed = alt_bn128_g1_decompress(compressed_ref.as_slice()).unwrap();

            <span class="macro">assert_eq!</span>(
                alt_bn128_g1_compress(<span class="kw-2">&amp;</span>decompressed).unwrap(),
                compressed_ref
            );
            <span class="macro">assert_eq!</span>(decompressed, <span class="kw-2">*</span>g1_be);
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>alt_bn128_g2_compression() {
        <span class="kw">let </span>g2_be = [
            <span class="number">40</span>, <span class="number">57</span>, <span class="number">233</span>, <span class="number">205</span>, <span class="number">180</span>, <span class="number">46</span>, <span class="number">35</span>, <span class="number">111</span>, <span class="number">215</span>, <span class="number">5</span>, <span class="number">23</span>, <span class="number">93</span>, <span class="number">12</span>, <span class="number">71</span>, <span class="number">118</span>, <span class="number">225</span>, <span class="number">7</span>, <span class="number">46</span>, <span class="number">247</span>, <span class="number">147</span>,
            <span class="number">47</span>, <span class="number">130</span>, <span class="number">106</span>, <span class="number">189</span>, <span class="number">184</span>, <span class="number">80</span>, <span class="number">146</span>, <span class="number">103</span>, <span class="number">141</span>, <span class="number">52</span>, <span class="number">242</span>, <span class="number">25</span>, <span class="number">0</span>, <span class="number">203</span>, <span class="number">124</span>, <span class="number">176</span>, <span class="number">110</span>, <span class="number">34</span>, <span class="number">151</span>,
            <span class="number">212</span>, <span class="number">66</span>, <span class="number">180</span>, <span class="number">238</span>, <span class="number">151</span>, <span class="number">236</span>, <span class="number">189</span>, <span class="number">133</span>, <span class="number">209</span>, <span class="number">17</span>, <span class="number">137</span>, <span class="number">205</span>, <span class="number">183</span>, <span class="number">168</span>, <span class="number">196</span>, <span class="number">92</span>, <span class="number">159</span>, <span class="number">75</span>,
            <span class="number">174</span>, <span class="number">81</span>, <span class="number">168</span>, <span class="number">18</span>, <span class="number">86</span>, <span class="number">176</span>, <span class="number">56</span>, <span class="number">16</span>, <span class="number">26</span>, <span class="number">210</span>, <span class="number">20</span>, <span class="number">18</span>, <span class="number">81</span>, <span class="number">122</span>, <span class="number">142</span>, <span class="number">104</span>, <span class="number">62</span>, <span class="number">251</span>, <span class="number">169</span>,
            <span class="number">98</span>, <span class="number">141</span>, <span class="number">21</span>, <span class="number">253</span>, <span class="number">50</span>, <span class="number">130</span>, <span class="number">182</span>, <span class="number">15</span>, <span class="number">33</span>, <span class="number">109</span>, <span class="number">228</span>, <span class="number">31</span>, <span class="number">79</span>, <span class="number">183</span>, <span class="number">88</span>, <span class="number">147</span>, <span class="number">174</span>, <span class="number">108</span>, <span class="number">4</span>,
            <span class="number">22</span>, <span class="number">14</span>, <span class="number">129</span>, <span class="number">168</span>, <span class="number">6</span>, <span class="number">80</span>, <span class="number">246</span>, <span class="number">254</span>, <span class="number">100</span>, <span class="number">218</span>, <span class="number">131</span>, <span class="number">94</span>, <span class="number">49</span>, <span class="number">247</span>, <span class="number">211</span>, <span class="number">3</span>, <span class="number">245</span>, <span class="number">22</span>, <span class="number">200</span>,
            <span class="number">177</span>, <span class="number">91</span>, <span class="number">60</span>, <span class="number">144</span>, <span class="number">147</span>, <span class="number">174</span>, <span class="number">90</span>, <span class="number">17</span>, <span class="number">19</span>, <span class="number">189</span>, <span class="number">62</span>, <span class="number">147</span>, <span class="number">152</span>, <span class="number">18</span>,
        ];
        <span class="kw">let </span>g2_le = convert_endianness::&lt;<span class="number">64</span>, <span class="number">128</span>&gt;(<span class="kw-2">&amp;</span>g2_be);
        <span class="kw">let </span>g2: G2 =
            G2::deserialize_with_mode(g2_le.as_slice(), Compress::No, Validate::No).unwrap();

        <span class="kw">let </span>g2_neg = g2.neg();
        <span class="kw">let </span><span class="kw-2">mut </span>g2_neg_be = [<span class="number">0u8</span>; <span class="number">128</span>];
        g2_neg
            .x
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>g2_neg_be[..<span class="number">64</span>], Compress::No)
            .unwrap();
        g2_neg
            .y
            .serialize_with_mode(<span class="kw-2">&amp;mut </span>g2_neg_be[<span class="number">64</span>..<span class="number">128</span>], Compress::No)
            .unwrap();
        <span class="kw">let </span>g2_neg_be: [u8; <span class="number">128</span>] = convert_endianness::&lt;<span class="number">64</span>, <span class="number">128</span>&gt;(<span class="kw-2">&amp;</span>g2_neg_be);

        <span class="kw">let </span>points = [(g2, g2_be), (g2_neg, g2_neg_be)];

        <span class="kw">for </span>(point, g2_be) <span class="kw">in </span><span class="kw-2">&amp;</span>points {
            <span class="kw">let </span><span class="kw-2">mut </span>compressed_ref = [<span class="number">0u8</span>; <span class="number">64</span>];
            G2::serialize_with_mode(point, compressed_ref.as_mut_slice(), Compress::Yes).unwrap();
            <span class="kw">let </span>compressed_ref: [u8; <span class="number">64</span>] = convert_endianness::&lt;<span class="number">64</span>, <span class="number">64</span>&gt;(<span class="kw-2">&amp;</span>compressed_ref);

            <span class="kw">let </span>decompressed = alt_bn128_g2_decompress(compressed_ref.as_slice()).unwrap();

            <span class="macro">assert_eq!</span>(
                alt_bn128_g2_compress(<span class="kw-2">&amp;</span>decompressed).unwrap(),
                compressed_ref
            );
            <span class="macro">assert_eq!</span>(decompressed, <span class="kw-2">*</span>g2_be);
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>alt_bn128_compression_g1_point_of_infitity() {
        <span class="kw">let </span>g1_bytes = <span class="macro">vec!</span>[<span class="number">0u8</span>; <span class="number">64</span>];
        <span class="kw">let </span>g1_compressed = alt_bn128_g1_compress(<span class="kw-2">&amp;</span>g1_bytes).unwrap();
        <span class="kw">let </span>g1_decompressed = alt_bn128_g1_decompress(<span class="kw-2">&amp;</span>g1_compressed).unwrap();
        <span class="macro">assert_eq!</span>(g1_bytes, g1_decompressed);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>alt_bn128_compression_g2_point_of_infitity() {
        <span class="kw">let </span>g1_bytes = <span class="macro">vec!</span>[<span class="number">0u8</span>; <span class="number">128</span>];
        <span class="kw">let </span>g1_compressed = alt_bn128_g2_compress(<span class="kw-2">&amp;</span>g1_bytes).unwrap();
        <span class="kw">let </span>g1_decompressed = alt_bn128_g2_decompress(<span class="kw-2">&amp;</span>g1_compressed).unwrap();
        <span class="macro">assert_eq!</span>(g1_bytes, g1_decompressed);
    }
    <span class="attr">#[test]
    </span><span class="kw">fn </span>alt_bn128_compression_pairing_test_input() {
        <span class="kw">use </span>serde::Deserialize;

        <span class="kw">let </span>test_data = <span class="string">r#"[
        {
            "Input": "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "jeff1",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "2eca0c7238bf16e83e7a1e6c5d49540685ff51380f309842a98561558019fc0203d3260361bb8451de5ff5ecd17f010ff22f5c31cdf184e9020b06fa5997db841213d2149b006137fcfb23036606f848d638d576a120ca981b5b1a5f9300b3ee2276cf730cf493cd95d64677bbb75fc42db72513a4c1e387b476d056f80aa75f21ee6226d31426322afcda621464d0611d226783262e21bb3bc86b537e986237096df1f82dff337dd5972e32a8ad43e28a78a96a823ef1cd4debe12b6552ea5f06967a1237ebfeca9aaae0d6d0bab8e28c198c5a339ef8a2407e31cdac516db922160fa257a5fd5b280642ff47b65eca77e626cb685c84fa6d3b6882a283ddd1198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "jeff2",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "0f25929bcb43d5a57391564615c9e70a992b10eafa4db109709649cf48c50dd216da2f5cb6be7a0aa72c440c53c9bbdfec6c36c7d515536431b3a865468acbba2e89718ad33c8bed92e210e81d1853435399a271913a6520736a4729cf0d51eb01a9e2ffa2e92599b68e44de5bcf354fa2642bd4f26b259daa6f7ce3ed57aeb314a9a87b789a58af499b314e13c3d65bede56c07ea2d418d6874857b70763713178fb49a2d6cd347dc58973ff49613a20757d0fcc22079f9abd10c3baee245901b9e027bd5cfc2cb5db82d4dc9677ac795ec500ecd47deee3b5da006d6d049b811d7511c78158de484232fc68daf8a45cf217d1c2fae693ff5871e8752d73b21198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "jeff3",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "2f2ea0b3da1e8ef11914acf8b2e1b32d99df51f5f4f206fc6b947eae860eddb6068134ddb33dc888ef446b648d72338684d678d2eb2371c61a50734d78da4b7225f83c8b6ab9de74e7da488ef02645c5a16a6652c3c71a15dc37fe3a5dcb7cb122acdedd6308e3bb230d226d16a105295f523a8a02bfc5e8bd2da135ac4c245d065bbad92e7c4e31bf3757f1fe7362a63fbfee50e7dc68da116e67d600d9bf6806d302580dc0661002994e7cd3a7f224e7ddc27802777486bf80f40e4ca3cfdb186bac5188a98c45e6016873d107f5cd131f3a3e339d0375e58bd6219347b008122ae2b09e539e152ec5364e7e2204b03d11d3caa038bfc7cd499f8176aacbee1f39e4e4afc4bc74790a4a028aff2c3d2538731fb755edefd8cb48d6ea589b5e283f150794b6736f670d6a1033f9b46c6f5204f50813eb85c8dc4b59db1c5d39140d97ee4d2b36d99bc49974d18ecca3e7ad51011956051b464d9e27d46cc25e0764bb98575bd466d32db7b15f582b2d5c452b36aa394b789366e5e3ca5aabd415794ab061441e51d01e94640b7e3084a07e02c78cf3103c542bc5b298669f211b88da1679b0b64a63b7e0e7bfe52aae524f73a55be7fe70c7e9bfc94b4cf0da1213d2149b006137fcfb23036606f848d638d576a120ca981b5b1a5f9300b3ee2276cf730cf493cd95d64677bbb75fc42db72513a4c1e387b476d056f80aa75f21ee6226d31426322afcda621464d0611d226783262e21bb3bc86b537e986237096df1f82dff337dd5972e32a8ad43e28a78a96a823ef1cd4debe12b6552ea5f",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "jeff4",
            "Gas": 147000,
            "NoBenchmark": false
        },{
            "Input": "20a754d2071d4d53903e3b31a7e98ad6882d58aec240ef981fdf0a9d22c5926a29c853fcea789887315916bbeb89ca37edb355b4f980c9a12a94f30deeed30211213d2149b006137fcfb23036606f848d638d576a120ca981b5b1a5f9300b3ee2276cf730cf493cd95d64677bbb75fc42db72513a4c1e387b476d056f80aa75f21ee6226d31426322afcda621464d0611d226783262e21bb3bc86b537e986237096df1f82dff337dd5972e32a8ad43e28a78a96a823ef1cd4debe12b6552ea5f1abb4a25eb9379ae96c84fff9f0540abcfc0a0d11aeda02d4f37e4baf74cb0c11073b3ff2cdbb38755f8691ea59e9606696b3ff278acfc098fa8226470d03869217cee0a9ad79a4493b5253e2e4e3a39fc2df38419f230d341f60cb064a0ac290a3d76f140db8418ba512272381446eb73958670f00cf46f1d9e64cba057b53c26f64a8ec70387a13e41430ed3ee4a7db2059cc5fc13c067194bcc0cb49a98552fd72bd9edb657346127da132e5b82ab908f5816c826acb499e22f2412d1a2d70f25929bcb43d5a57391564615c9e70a992b10eafa4db109709649cf48c50dd2198a1f162a73261f112401aa2db79c7dab1533c9935c77290a6ce3b191f2318d198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "jeff5",
            "Gas": 147000,
            "NoBenchmark": false
        },{
            "Input": "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c103188585e2364128fe25c70558f1560f4f9350baf3959e603cc91486e110936198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000000",
            "Name": "jeff6",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000000",
            "Name": "one_point",
            "Gas": 79000,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "two_point_match_2",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "two_point_match_3",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "105456a333e6d636854f987ea7bb713dfd0ae8371a72aea313ae0c32c0bf10160cf031d41b41557f3e7e3ba0c51bebe5da8e6ecd855ec50fc87efcdeac168bcc0476be093a6d2b4bbf907172049874af11e1b6267606e00804d3ff0037ec57fd3010c68cb50161b7d1d96bb71edfec9880171954e56871abf3d93cc94d745fa114c059d74e5b6c4ec14ae5864ebe23a71781d86c29fb8fb6cce94f70d3de7a2101b33461f39d9e887dbb100f170a2345dde3c07e256d1dfa2b657ba5cd030427000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000021a2c3013d2ea92e13c800cde68ef56a294b883f6ac35d25f587c09b1b3c635f7290158a80cd3d66530f74dc94c94adb88f5cdb481acca997b6e60071f08a115f2f997f3dbd66a7afe07fe7862ce239edba9e05c5afff7f8a1259c9733b2dfbb929d1691530ca701b4a106054688728c9972c8512e9789e9567aae23e302ccd75",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "two_point_match_4",
            "Gas": 113000,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "ten_point_match_1",
            "Gas": 385000,
            "NoBenchmark": false
        },{
            "Input": "00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002203e205db4f19b37b60121b83a7333706db86431c6d835849957ed8c3928ad7927dc7234fd11d3e8c36c59277c3e6f149d5cd3cfa9a62aee49f8130962b4b3b9195e8aa5b7827463722b8c153931579d3505566b4edf48d498e185f0509de15204bb53b8977e5f92a0bc372742c4830944a59b4fe6b1c0466e2a6dad122b5d2e030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd31a76dae6d3272396d0cbe61fced2bc532edac647851e3ac53ce1cc9c7e645a83198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "ten_point_match_2",
            "Gas": 385000,
            "NoBenchmark": false
        },{
            "Input": "105456a333e6d636854f987ea7bb713dfd0ae8371a72aea313ae0c32c0bf10160cf031d41b41557f3e7e3ba0c51bebe5da8e6ecd855ec50fc87efcdeac168bcc0476be093a6d2b4bbf907172049874af11e1b6267606e00804d3ff0037ec57fd3010c68cb50161b7d1d96bb71edfec9880171954e56871abf3d93cc94d745fa114c059d74e5b6c4ec14ae5864ebe23a71781d86c29fb8fb6cce94f70d3de7a2101b33461f39d9e887dbb100f170a2345dde3c07e256d1dfa2b657ba5cd030427000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000021a2c3013d2ea92e13c800cde68ef56a294b883f6ac35d25f587c09b1b3c635f7290158a80cd3d66530f74dc94c94adb88f5cdb481acca997b6e60071f08a115f2f997f3dbd66a7afe07fe7862ce239edba9e05c5afff7f8a1259c9733b2dfbb929d1691530ca701b4a106054688728c9972c8512e9789e9567aae23e302ccd75",
            "Expected": "0000000000000000000000000000000000000000000000000000000000000001",
            "Name": "ten_point_match_3",
            "Gas": 113000,
            "NoBenchmark": false
        }
        ]"#</span>;

        <span class="attr">#[derive(Deserialize)]
        #[serde(rename_all = <span class="string">"PascalCase"</span>)]
        </span><span class="kw">struct </span>TestCase {
            input: String,
        }

        <span class="kw">let </span>test_cases: Vec&lt;TestCase&gt; = serde_json::from_str(test_data).unwrap();

        test_cases.iter().for_each(|test| {
            <span class="kw">let </span>input = array_bytes::hex2bytes_unchecked(<span class="kw-2">&amp;</span>test.input);
            <span class="kw">let </span>g1 = input[<span class="number">0</span>..<span class="number">64</span>].to_vec();
            <span class="kw">let </span>g1_compressed = alt_bn128_g1_compress(<span class="kw-2">&amp;</span>g1).unwrap();
            <span class="macro">assert_eq!</span>(g1, alt_bn128_g1_decompress(<span class="kw-2">&amp;</span>g1_compressed).unwrap());
            <span class="kw">let </span>g2 = input[<span class="number">64</span>..<span class="number">192</span>].to_vec();
            <span class="kw">let </span>g2_compressed = alt_bn128_g2_compress(<span class="kw-2">&amp;</span>g2).unwrap();
            <span class="macro">assert_eq!</span>(g2, alt_bn128_g2_decompress(<span class="kw-2">&amp;</span>g2_compressed).unwrap());
        });
    }
}
</code></pre></div></section></main></body></html>