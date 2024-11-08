<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/proc-macro2-1.0.81/src/fallback.rs`."><title>fallback.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="proc_macro2" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../proc_macro2/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="attr">#[cfg(span_locations)]
</span><span class="kw">use </span><span class="kw">crate</span>::location::LineColumn;
<span class="kw">use </span><span class="kw">crate</span>::parse::{<span class="self">self</span>, Cursor};
<span class="kw">use </span><span class="kw">crate</span>::rcvec::{RcVec, RcVecBuilder, RcVecIntoIter, RcVecMut};
<span class="kw">use crate</span>::{Delimiter, Spacing, TokenTree};
<span class="attr">#[cfg(all(span_locations, not(fuzzing)))]
</span><span class="kw">use </span>alloc::collections::BTreeMap;
<span class="attr">#[cfg(all(span_locations, not(fuzzing)))]
</span><span class="kw">use </span>core::cell::RefCell;
<span class="attr">#[cfg(span_locations)]
</span><span class="kw">use </span>core::cmp;
<span class="kw">use </span>core::fmt::{<span class="self">self</span>, Debug, Display, Write};
<span class="kw">use </span>core::mem::ManuallyDrop;
<span class="attr">#[cfg(span_locations)]
</span><span class="kw">use </span>core::ops::Range;
<span class="kw">use </span>core::ops::RangeBounds;
<span class="kw">use </span>core::ptr;
<span class="kw">use </span>core::str::{<span class="self">self</span>, FromStr};
<span class="kw">use </span>std::ffi::CStr;
<span class="attr">#[cfg(procmacro2_semver_exempt)]
</span><span class="kw">use </span>std::path::PathBuf;

<span class="doccomment">/// Force use of proc-macro2's fallback implementation of the API for now, even
/// if the compiler's implementation is available.
</span><span class="kw">pub fn </span>force() {
    <span class="attr">#[cfg(wrap_proc_macro)]
    </span><span class="kw">crate</span>::detection::force_fallback();
}

<span class="doccomment">/// Resume using the compiler's implementation of the proc macro API if it is
/// available.
</span><span class="kw">pub fn </span>unforce() {
    <span class="attr">#[cfg(wrap_proc_macro)]
    </span><span class="kw">crate</span>::detection::unforce_fallback();
}

<span class="attr">#[derive(Clone)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>TokenStream {
    inner: RcVec&lt;TokenTree&gt;,
}

<span class="attr">#[derive(Debug)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>LexError {
    <span class="kw">pub</span>(<span class="kw">crate</span>) span: Span,
}

<span class="kw">impl </span>LexError {
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>span(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Span {
        <span class="self">self</span>.span
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>call_site() -&gt; <span class="self">Self </span>{
        LexError {
            span: Span::call_site(),
        }
    }
}

<span class="kw">impl </span>TokenStream {
    <span class="kw">pub fn </span>new() -&gt; <span class="self">Self </span>{
        TokenStream {
            inner: RcVecBuilder::new().build(),
        }
    }

    <span class="kw">pub fn </span>is_empty(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.inner.len() == <span class="number">0
    </span>}

    <span class="kw">fn </span>take_inner(<span class="self">self</span>) -&gt; RcVecBuilder&lt;TokenTree&gt; {
        <span class="kw">let </span>nodrop = ManuallyDrop::new(<span class="self">self</span>);
        <span class="kw">unsafe </span>{ ptr::read(<span class="kw-2">&amp;</span>nodrop.inner) }.make_owned()
    }
}

<span class="kw">fn </span>push_token_from_proc_macro(<span class="kw-2">mut </span>vec: RcVecMut&lt;TokenTree&gt;, token: TokenTree) {
    <span class="comment">// https://github.com/dtolnay/proc-macro2/issues/235
    </span><span class="kw">match </span>token {
        TokenTree::Literal(<span class="kw">crate</span>::Literal {
            <span class="attr">#[cfg(wrap_proc_macro)]
                </span>inner: <span class="kw">crate</span>::imp::Literal::Fallback(literal),
            <span class="attr">#[cfg(not(wrap_proc_macro))]
                </span>inner: literal,
            ..
        }) <span class="kw">if </span>literal.repr.starts_with(<span class="string">'-'</span>) =&gt; {
            push_negative_literal(vec, literal);
        }
        <span class="kw">_ </span>=&gt; vec.push(token),
    }

    <span class="attr">#[cold]
    </span><span class="kw">fn </span>push_negative_literal(<span class="kw-2">mut </span>vec: RcVecMut&lt;TokenTree&gt;, <span class="kw-2">mut </span>literal: Literal) {
        literal.repr.remove(<span class="number">0</span>);
        <span class="kw">let </span><span class="kw-2">mut </span>punct = <span class="kw">crate</span>::Punct::new(<span class="string">'-'</span>, Spacing::Alone);
        punct.set_span(<span class="kw">crate</span>::Span::_new_fallback(literal.span));
        vec.push(TokenTree::Punct(punct));
        vec.push(TokenTree::Literal(<span class="kw">crate</span>::Literal::_new_fallback(literal)));
    }
}

<span class="comment">// Nonrecursive to prevent stack overflow.
</span><span class="kw">impl </span>Drop <span class="kw">for </span>TokenStream {
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">let </span><span class="kw-2">mut </span>inner = <span class="kw">match </span><span class="self">self</span>.inner.get_mut() {
            <span class="prelude-val">Some</span>(inner) =&gt; inner,
            <span class="prelude-val">None </span>=&gt; <span class="kw">return</span>,
        };
        <span class="kw">while let </span><span class="prelude-val">Some</span>(token) = inner.pop() {
            <span class="kw">let </span>group = <span class="kw">match </span>token {
                TokenTree::Group(group) =&gt; group.inner,
                <span class="kw">_ </span>=&gt; <span class="kw">continue</span>,
            };
            <span class="attr">#[cfg(wrap_proc_macro)]
            </span><span class="kw">let </span>group = <span class="kw">match </span>group {
                <span class="kw">crate</span>::imp::Group::Fallback(group) =&gt; group,
                <span class="kw">crate</span>::imp::Group::Compiler(<span class="kw">_</span>) =&gt; <span class="kw">continue</span>,
            };
            inner.extend(group.stream.take_inner());
        }
    }
}

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>TokenStreamBuilder {
    inner: RcVecBuilder&lt;TokenTree&gt;,
}

<span class="kw">impl </span>TokenStreamBuilder {
    <span class="kw">pub fn </span>new() -&gt; <span class="self">Self </span>{
        TokenStreamBuilder {
            inner: RcVecBuilder::new(),
        }
    }

    <span class="kw">pub fn </span>with_capacity(cap: usize) -&gt; <span class="self">Self </span>{
        TokenStreamBuilder {
            inner: RcVecBuilder::with_capacity(cap),
        }
    }

    <span class="kw">pub fn </span>push_token_from_parser(<span class="kw-2">&amp;mut </span><span class="self">self</span>, tt: TokenTree) {
        <span class="self">self</span>.inner.push(tt);
    }

    <span class="kw">pub fn </span>build(<span class="self">self</span>) -&gt; TokenStream {
        TokenStream {
            inner: <span class="self">self</span>.inner.build(),
        }
    }
}

<span class="attr">#[cfg(span_locations)]
</span><span class="kw">fn </span>get_cursor(src: <span class="kw-2">&amp;</span>str) -&gt; Cursor {
    <span class="attr">#[cfg(fuzzing)]
    </span><span class="kw">return </span>Cursor { rest: src, off: <span class="number">1 </span>};

    <span class="comment">// Create a dummy file &amp; add it to the source map
    </span><span class="attr">#[cfg(not(fuzzing))]
    </span>SOURCE_MAP.with(|sm| {
        <span class="kw">let </span><span class="kw-2">mut </span>sm = sm.borrow_mut();
        <span class="kw">let </span>span = sm.add_file(src);
        Cursor {
            rest: src,
            off: span.lo,
        }
    })
}

<span class="attr">#[cfg(not(span_locations))]
</span><span class="kw">fn </span>get_cursor(src: <span class="kw-2">&amp;</span>str) -&gt; Cursor {
    Cursor { rest: src }
}

<span class="kw">impl </span>FromStr <span class="kw">for </span>TokenStream {
    <span class="kw">type </span><span class="prelude-val">Err </span>= LexError;

    <span class="kw">fn </span>from_str(src: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;TokenStream, LexError&gt; {
        <span class="comment">// Create a dummy file &amp; add it to the source map
        </span><span class="kw">let </span><span class="kw-2">mut </span>cursor = get_cursor(src);

        <span class="comment">// Strip a byte order mark if present
        </span><span class="kw">const </span>BYTE_ORDER_MARK: <span class="kw-2">&amp;</span>str = <span class="string">"\u{feff}"</span>;
        <span class="kw">if </span>cursor.starts_with(BYTE_ORDER_MARK) {
            cursor = cursor.advance(BYTE_ORDER_MARK.len());
        }

        parse::token_stream(cursor)
    }
}

<span class="kw">impl </span>Display <span class="kw">for </span>LexError {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        f.write_str(<span class="string">"cannot parse string into token stream"</span>)
    }
}

<span class="kw">impl </span>Display <span class="kw">for </span>TokenStream {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">let </span><span class="kw-2">mut </span>joint = <span class="bool-val">false</span>;
        <span class="kw">for </span>(i, tt) <span class="kw">in </span><span class="self">self</span>.inner.iter().enumerate() {
            <span class="kw">if </span>i != <span class="number">0 </span>&amp;&amp; !joint {
                <span class="macro">write!</span>(f, <span class="string">" "</span>)<span class="question-mark">?</span>;
            }
            joint = <span class="bool-val">false</span>;
            <span class="kw">match </span>tt {
                TokenTree::Group(tt) =&gt; Display::fmt(tt, f),
                TokenTree::Ident(tt) =&gt; Display::fmt(tt, f),
                TokenTree::Punct(tt) =&gt; {
                    joint = tt.spacing() == Spacing::Joint;
                    Display::fmt(tt, f)
                }
                TokenTree::Literal(tt) =&gt; Display::fmt(tt, f),
            }<span class="question-mark">?</span>;
        }

        <span class="prelude-val">Ok</span>(())
    }
}

<span class="kw">impl </span>Debug <span class="kw">for </span>TokenStream {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        f.write_str(<span class="string">"TokenStream "</span>)<span class="question-mark">?</span>;
        f.debug_list().entries(<span class="self">self</span>.clone()).finish()
    }
}

<span class="attr">#[cfg(feature = <span class="string">"proc-macro"</span>)]
</span><span class="kw">impl </span>From&lt;proc_macro::TokenStream&gt; <span class="kw">for </span>TokenStream {
    <span class="kw">fn </span>from(inner: proc_macro::TokenStream) -&gt; <span class="self">Self </span>{
        inner
            .to_string()
            .parse()
            .expect(<span class="string">"compiler token stream parse failed"</span>)
    }
}

<span class="attr">#[cfg(feature = <span class="string">"proc-macro"</span>)]
</span><span class="kw">impl </span>From&lt;TokenStream&gt; <span class="kw">for </span>proc_macro::TokenStream {
    <span class="kw">fn </span>from(inner: TokenStream) -&gt; <span class="self">Self </span>{
        inner
            .to_string()
            .parse()
            .expect(<span class="string">"failed to parse to compiler tokens"</span>)
    }
}

<span class="kw">impl </span>From&lt;TokenTree&gt; <span class="kw">for </span>TokenStream {
    <span class="kw">fn </span>from(tree: TokenTree) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>stream = RcVecBuilder::new();
        push_token_from_proc_macro(stream.as_mut(), tree);
        TokenStream {
            inner: stream.build(),
        }
    }
}

<span class="kw">impl </span>FromIterator&lt;TokenTree&gt; <span class="kw">for </span>TokenStream {
    <span class="kw">fn </span>from_iter&lt;I: IntoIterator&lt;Item = TokenTree&gt;&gt;(tokens: I) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>stream = TokenStream::new();
        stream.extend(tokens);
        stream
    }
}

<span class="kw">impl </span>FromIterator&lt;TokenStream&gt; <span class="kw">for </span>TokenStream {
    <span class="kw">fn </span>from_iter&lt;I: IntoIterator&lt;Item = TokenStream&gt;&gt;(streams: I) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>v = RcVecBuilder::new();

        <span class="kw">for </span>stream <span class="kw">in </span>streams {
            v.extend(stream.take_inner());
        }

        TokenStream { inner: v.build() }
    }
}

<span class="kw">impl </span>Extend&lt;TokenTree&gt; <span class="kw">for </span>TokenStream {
    <span class="kw">fn </span>extend&lt;I: IntoIterator&lt;Item = TokenTree&gt;&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, tokens: I) {
        <span class="kw">let </span><span class="kw-2">mut </span>vec = <span class="self">self</span>.inner.make_mut();
        tokens
            .into_iter()
            .for_each(|token| push_token_from_proc_macro(vec.as_mut(), token));
    }
}

<span class="kw">impl </span>Extend&lt;TokenStream&gt; <span class="kw">for </span>TokenStream {
    <span class="kw">fn </span>extend&lt;I: IntoIterator&lt;Item = TokenStream&gt;&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, streams: I) {
        <span class="self">self</span>.inner.make_mut().extend(streams.into_iter().flatten());
    }
}

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">type </span>TokenTreeIter = RcVecIntoIter&lt;TokenTree&gt;;

<span class="kw">impl </span>IntoIterator <span class="kw">for </span>TokenStream {
    <span class="kw">type </span>Item = TokenTree;
    <span class="kw">type </span>IntoIter = TokenTreeIter;

    <span class="kw">fn </span>into_iter(<span class="self">self</span>) -&gt; TokenTreeIter {
        <span class="self">self</span>.take_inner().into_iter()
    }
}

<span class="attr">#[cfg(procmacro2_semver_exempt)]
#[derive(Clone, PartialEq, Eq)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>SourceFile {
    path: PathBuf,
}

<span class="attr">#[cfg(procmacro2_semver_exempt)]
</span><span class="kw">impl </span>SourceFile {
    <span class="doccomment">/// Get the path to this source file as a string.
    </span><span class="kw">pub fn </span>path(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; PathBuf {
        <span class="self">self</span>.path.clone()
    }

    <span class="kw">pub fn </span>is_real(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="bool-val">false
    </span>}
}

<span class="attr">#[cfg(procmacro2_semver_exempt)]
</span><span class="kw">impl </span>Debug <span class="kw">for </span>SourceFile {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        f.debug_struct(<span class="string">"SourceFile"</span>)
            .field(<span class="string">"path"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.path())
            .field(<span class="string">"is_real"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.is_real())
            .finish()
    }
}

<span class="attr">#[cfg(all(span_locations, not(fuzzing)))]
</span><span class="macro">thread_local!</span> {
    <span class="kw">static </span>SOURCE_MAP: RefCell&lt;SourceMap&gt; = RefCell::new(SourceMap {
        <span class="comment">// Start with a single dummy file which all call_site() and def_site()
        // spans reference.
        </span>files: <span class="macro">vec!</span>[FileInfo {
            source_text: String::new(),
            span: Span { lo: <span class="number">0</span>, hi: <span class="number">0 </span>},
            lines: <span class="macro">vec!</span>[<span class="number">0</span>],
            char_index_to_byte_offset: BTreeMap::new(),
        }],
    });
}

<span class="attr">#[cfg(span_locations)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>invalidate_current_thread_spans() {
    <span class="attr">#[cfg(not(fuzzing))]
    </span>SOURCE_MAP.with(|sm| sm.borrow_mut().files.truncate(<span class="number">1</span>));
}

<span class="attr">#[cfg(all(span_locations, not(fuzzing)))]
</span><span class="kw">struct </span>FileInfo {
    source_text: String,
    span: Span,
    lines: Vec&lt;usize&gt;,
    char_index_to_byte_offset: BTreeMap&lt;usize, usize&gt;,
}

<span class="attr">#[cfg(all(span_locations, not(fuzzing)))]
</span><span class="kw">impl </span>FileInfo {
    <span class="kw">fn </span>offset_line_column(<span class="kw-2">&amp;</span><span class="self">self</span>, offset: usize) -&gt; LineColumn {
        <span class="macro">assert!</span>(<span class="self">self</span>.span_within(Span {
            lo: offset <span class="kw">as </span>u32,
            hi: offset <span class="kw">as </span>u32,
        }));
        <span class="kw">let </span>offset = offset - <span class="self">self</span>.span.lo <span class="kw">as </span>usize;
        <span class="kw">match </span><span class="self">self</span>.lines.binary_search(<span class="kw-2">&amp;</span>offset) {
            <span class="prelude-val">Ok</span>(found) =&gt; LineColumn {
                line: found + <span class="number">1</span>,
                column: <span class="number">0</span>,
            },
            <span class="prelude-val">Err</span>(idx) =&gt; LineColumn {
                line: idx,
                column: offset - <span class="self">self</span>.lines[idx - <span class="number">1</span>],
            },
        }
    }

    <span class="kw">fn </span>span_within(<span class="kw-2">&amp;</span><span class="self">self</span>, span: Span) -&gt; bool {
        span.lo &gt;= <span class="self">self</span>.span.lo &amp;&amp; span.hi &lt;= <span class="self">self</span>.span.hi
    }

    <span class="kw">fn </span>byte_range(<span class="kw-2">&amp;mut </span><span class="self">self</span>, span: Span) -&gt; Range&lt;usize&gt; {
        <span class="kw">let </span>lo_char = (span.lo - <span class="self">self</span>.span.lo) <span class="kw">as </span>usize;

        <span class="comment">// Look up offset of the largest already-computed char index that is
        // less than or equal to the current requested one. We resume counting
        // chars from that point.
        </span><span class="kw">let </span>(<span class="kw-2">&amp;</span>last_char_index, <span class="kw-2">&amp;</span>last_byte_offset) = <span class="self">self
            </span>.char_index_to_byte_offset
            .range(..=lo_char)
            .next_back()
            .unwrap_or((<span class="kw-2">&amp;</span><span class="number">0</span>, <span class="kw-2">&amp;</span><span class="number">0</span>));

        <span class="kw">let </span>lo_byte = <span class="kw">if </span>last_char_index == lo_char {
            last_byte_offset
        } <span class="kw">else </span>{
            <span class="kw">let </span>total_byte_offset = <span class="kw">match </span><span class="self">self</span>.source_text[last_byte_offset..]
                .char_indices()
                .nth(lo_char - last_char_index)
            {
                <span class="prelude-val">Some</span>((additional_offset, _ch)) =&gt; last_byte_offset + additional_offset,
                <span class="prelude-val">None </span>=&gt; <span class="self">self</span>.source_text.len(),
            };
            <span class="self">self</span>.char_index_to_byte_offset
                .insert(lo_char, total_byte_offset);
            total_byte_offset
        };

        <span class="kw">let </span>trunc_lo = <span class="kw-2">&amp;</span><span class="self">self</span>.source_text[lo_byte..];
        <span class="kw">let </span>char_len = (span.hi - span.lo) <span class="kw">as </span>usize;
        lo_byte..<span class="kw">match </span>trunc_lo.char_indices().nth(char_len) {
            <span class="prelude-val">Some</span>((offset, _ch)) =&gt; lo_byte + offset,
            <span class="prelude-val">None </span>=&gt; <span class="self">self</span>.source_text.len(),
        }
    }

    <span class="kw">fn </span>source_text(<span class="kw-2">&amp;mut </span><span class="self">self</span>, span: Span) -&gt; String {
        <span class="kw">let </span>byte_range = <span class="self">self</span>.byte_range(span);
        <span class="self">self</span>.source_text[byte_range].to_owned()
    }
}

<span class="doccomment">/// Computes the offsets of each line in the given source string
/// and the total number of characters
</span><span class="attr">#[cfg(all(span_locations, not(fuzzing)))]
</span><span class="kw">fn </span>lines_offsets(s: <span class="kw-2">&amp;</span>str) -&gt; (usize, Vec&lt;usize&gt;) {
    <span class="kw">let </span><span class="kw-2">mut </span>lines = <span class="macro">vec!</span>[<span class="number">0</span>];
    <span class="kw">let </span><span class="kw-2">mut </span>total = <span class="number">0</span>;

    <span class="kw">for </span>ch <span class="kw">in </span>s.chars() {
        total += <span class="number">1</span>;
        <span class="kw">if </span>ch == <span class="string">'\n' </span>{
            lines.push(total);
        }
    }

    (total, lines)
}

<span class="attr">#[cfg(all(span_locations, not(fuzzing)))]
</span><span class="kw">struct </span>SourceMap {
    files: Vec&lt;FileInfo&gt;,
}

<span class="attr">#[cfg(all(span_locations, not(fuzzing)))]
</span><span class="kw">impl </span>SourceMap {
    <span class="kw">fn </span>next_start_pos(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u32 {
        <span class="comment">// Add 1 so there's always space between files.
        //
        // We'll always have at least 1 file, as we initialize our files list
        // with a dummy file.
        </span><span class="self">self</span>.files.last().unwrap().span.hi + <span class="number">1
    </span>}

    <span class="kw">fn </span>add_file(<span class="kw-2">&amp;mut </span><span class="self">self</span>, src: <span class="kw-2">&amp;</span>str) -&gt; Span {
        <span class="kw">let </span>(len, lines) = lines_offsets(src);
        <span class="kw">let </span>lo = <span class="self">self</span>.next_start_pos();
        <span class="kw">let </span>span = Span {
            lo,
            hi: lo + (len <span class="kw">as </span>u32),
        };

        <span class="self">self</span>.files.push(FileInfo {
            source_text: src.to_owned(),
            span,
            lines,
            <span class="comment">// Populated lazily by source_text().
            </span>char_index_to_byte_offset: BTreeMap::new(),
        });

        span
    }

    <span class="attr">#[cfg(procmacro2_semver_exempt)]
    </span><span class="kw">fn </span>filepath(<span class="kw-2">&amp;</span><span class="self">self</span>, span: Span) -&gt; PathBuf {
        <span class="kw">for </span>(i, file) <span class="kw">in </span><span class="self">self</span>.files.iter().enumerate() {
            <span class="kw">if </span>file.span_within(span) {
                <span class="kw">return </span>PathBuf::from(<span class="kw">if </span>i == <span class="number">0 </span>{
                    <span class="string">"&lt;unspecified&gt;"</span>.to_owned()
                } <span class="kw">else </span>{
                    <span class="macro">format!</span>(<span class="string">"&lt;parsed string {}&gt;"</span>, i)
                });
            }
        }
        <span class="macro">unreachable!</span>(<span class="string">"Invalid span with no related FileInfo!"</span>);
    }

    <span class="kw">fn </span>fileinfo(<span class="kw-2">&amp;</span><span class="self">self</span>, span: Span) -&gt; <span class="kw-2">&amp;</span>FileInfo {
        <span class="kw">for </span>file <span class="kw">in </span><span class="kw-2">&amp;</span><span class="self">self</span>.files {
            <span class="kw">if </span>file.span_within(span) {
                <span class="kw">return </span>file;
            }
        }
        <span class="macro">unreachable!</span>(<span class="string">"Invalid span with no related FileInfo!"</span>);
    }

    <span class="kw">fn </span>fileinfo_mut(<span class="kw-2">&amp;mut </span><span class="self">self</span>, span: Span) -&gt; <span class="kw-2">&amp;mut </span>FileInfo {
        <span class="kw">for </span>file <span class="kw">in </span><span class="kw-2">&amp;mut </span><span class="self">self</span>.files {
            <span class="kw">if </span>file.span_within(span) {
                <span class="kw">return </span>file;
            }
        }
        <span class="macro">unreachable!</span>(<span class="string">"Invalid span with no related FileInfo!"</span>);
    }
}

<span class="attr">#[derive(Clone, Copy, PartialEq, Eq)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Span {
    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) lo: u32,
    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) hi: u32,
}

<span class="kw">impl </span>Span {
    <span class="attr">#[cfg(not(span_locations))]
    </span><span class="kw">pub fn </span>call_site() -&gt; <span class="self">Self </span>{
        Span {}
    }

    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">pub fn </span>call_site() -&gt; <span class="self">Self </span>{
        Span { lo: <span class="number">0</span>, hi: <span class="number">0 </span>}
    }

    <span class="kw">pub fn </span>mixed_site() -&gt; <span class="self">Self </span>{
        Span::call_site()
    }

    <span class="attr">#[cfg(procmacro2_semver_exempt)]
    </span><span class="kw">pub fn </span>def_site() -&gt; <span class="self">Self </span>{
        Span::call_site()
    }

    <span class="kw">pub fn </span>resolved_at(<span class="kw-2">&amp;</span><span class="self">self</span>, _other: Span) -&gt; Span {
        <span class="comment">// Stable spans consist only of line/column information, so
        // `resolved_at` and `located_at` only select which span the
        // caller wants line/column information from.
        </span><span class="kw-2">*</span><span class="self">self
    </span>}

    <span class="kw">pub fn </span>located_at(<span class="kw-2">&amp;</span><span class="self">self</span>, other: Span) -&gt; Span {
        other
    }

    <span class="attr">#[cfg(procmacro2_semver_exempt)]
    </span><span class="kw">pub fn </span>source_file(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; SourceFile {
        <span class="attr">#[cfg(fuzzing)]
        </span><span class="kw">return </span>SourceFile {
            path: PathBuf::from(<span class="string">"&lt;unspecified&gt;"</span>),
        };

        <span class="attr">#[cfg(not(fuzzing))]
        </span>SOURCE_MAP.with(|sm| {
            <span class="kw">let </span>sm = sm.borrow();
            <span class="kw">let </span>path = sm.filepath(<span class="kw-2">*</span><span class="self">self</span>);
            SourceFile { path }
        })
    }

    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">pub fn </span>byte_range(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Range&lt;usize&gt; {
        <span class="attr">#[cfg(fuzzing)]
        </span><span class="kw">return </span><span class="number">0</span>..<span class="number">0</span>;

        <span class="attr">#[cfg(not(fuzzing))]
        </span>{
            <span class="kw">if </span><span class="self">self</span>.is_call_site() {
                <span class="number">0</span>..<span class="number">0
            </span>} <span class="kw">else </span>{
                SOURCE_MAP.with(|sm| sm.borrow_mut().fileinfo_mut(<span class="kw-2">*</span><span class="self">self</span>).byte_range(<span class="kw-2">*</span><span class="self">self</span>))
            }
        }
    }

    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">pub fn </span>start(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LineColumn {
        <span class="attr">#[cfg(fuzzing)]
        </span><span class="kw">return </span>LineColumn { line: <span class="number">0</span>, column: <span class="number">0 </span>};

        <span class="attr">#[cfg(not(fuzzing))]
        </span>SOURCE_MAP.with(|sm| {
            <span class="kw">let </span>sm = sm.borrow();
            <span class="kw">let </span>fi = sm.fileinfo(<span class="kw-2">*</span><span class="self">self</span>);
            fi.offset_line_column(<span class="self">self</span>.lo <span class="kw">as </span>usize)
        })
    }

    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">pub fn </span>end(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LineColumn {
        <span class="attr">#[cfg(fuzzing)]
        </span><span class="kw">return </span>LineColumn { line: <span class="number">0</span>, column: <span class="number">0 </span>};

        <span class="attr">#[cfg(not(fuzzing))]
        </span>SOURCE_MAP.with(|sm| {
            <span class="kw">let </span>sm = sm.borrow();
            <span class="kw">let </span>fi = sm.fileinfo(<span class="kw-2">*</span><span class="self">self</span>);
            fi.offset_line_column(<span class="self">self</span>.hi <span class="kw">as </span>usize)
        })
    }

    <span class="attr">#[cfg(not(span_locations))]
    </span><span class="kw">pub fn </span>join(<span class="kw-2">&amp;</span><span class="self">self</span>, _other: Span) -&gt; <span class="prelude-ty">Option</span>&lt;Span&gt; {
        <span class="prelude-val">Some</span>(Span {})
    }

    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">pub fn </span>join(<span class="kw-2">&amp;</span><span class="self">self</span>, other: Span) -&gt; <span class="prelude-ty">Option</span>&lt;Span&gt; {
        <span class="attr">#[cfg(fuzzing)]
        </span><span class="kw">return </span>{
            <span class="kw">let _ </span>= other;
            <span class="prelude-val">None
        </span>};

        <span class="attr">#[cfg(not(fuzzing))]
        </span>SOURCE_MAP.with(|sm| {
            <span class="kw">let </span>sm = sm.borrow();
            <span class="comment">// If `other` is not within the same FileInfo as us, return None.
            </span><span class="kw">if </span>!sm.fileinfo(<span class="kw-2">*</span><span class="self">self</span>).span_within(other) {
                <span class="kw">return </span><span class="prelude-val">None</span>;
            }
            <span class="prelude-val">Some</span>(Span {
                lo: cmp::min(<span class="self">self</span>.lo, other.lo),
                hi: cmp::max(<span class="self">self</span>.hi, other.hi),
            })
        })
    }

    <span class="attr">#[cfg(not(span_locations))]
    </span><span class="kw">pub fn </span>source_text(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;String&gt; {
        <span class="prelude-val">None
    </span>}

    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">pub fn </span>source_text(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;String&gt; {
        <span class="attr">#[cfg(fuzzing)]
        </span><span class="kw">return </span><span class="prelude-val">None</span>;

        <span class="attr">#[cfg(not(fuzzing))]
        </span>{
            <span class="kw">if </span><span class="self">self</span>.is_call_site() {
                <span class="prelude-val">None
            </span>} <span class="kw">else </span>{
                <span class="prelude-val">Some</span>(SOURCE_MAP.with(|sm| sm.borrow_mut().fileinfo_mut(<span class="kw-2">*</span><span class="self">self</span>).source_text(<span class="kw-2">*</span><span class="self">self</span>)))
            }
        }
    }

    <span class="attr">#[cfg(not(span_locations))]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>first_byte(<span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self
    </span>}

    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>first_byte(<span class="self">self</span>) -&gt; <span class="self">Self </span>{
        Span {
            lo: <span class="self">self</span>.lo,
            hi: cmp::min(<span class="self">self</span>.lo.saturating_add(<span class="number">1</span>), <span class="self">self</span>.hi),
        }
    }

    <span class="attr">#[cfg(not(span_locations))]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>last_byte(<span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self
    </span>}

    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>last_byte(<span class="self">self</span>) -&gt; <span class="self">Self </span>{
        Span {
            lo: cmp::max(<span class="self">self</span>.hi.saturating_sub(<span class="number">1</span>), <span class="self">self</span>.lo),
            hi: <span class="self">self</span>.hi,
        }
    }

    <span class="attr">#[cfg(span_locations)]
    </span><span class="kw">fn </span>is_call_site(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.lo == <span class="number">0 </span>&amp;&amp; <span class="self">self</span>.hi == <span class="number">0
    </span>}
}

<span class="kw">impl </span>Debug <span class="kw">for </span>Span {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="attr">#[cfg(span_locations)]
        </span><span class="kw">return </span><span class="macro">write!</span>(f, <span class="string">"bytes({}..{})"</span>, <span class="self">self</span>.lo, <span class="self">self</span>.hi);

        <span class="attr">#[cfg(not(span_locations))]
        </span><span class="macro">write!</span>(f, <span class="string">"Span"</span>)
    }
}

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>debug_span_field_if_nontrivial(debug: <span class="kw-2">&amp;mut </span>fmt::DebugStruct, span: Span) {
    <span class="attr">#[cfg(span_locations)]
    </span>{
        <span class="kw">if </span>span.is_call_site() {
            <span class="kw">return</span>;
        }
    }

    <span class="kw">if </span><span class="macro">cfg!</span>(span_locations) {
        debug.field(<span class="string">"span"</span>, <span class="kw-2">&amp;</span>span);
    }
}

<span class="attr">#[derive(Clone)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Group {
    delimiter: Delimiter,
    stream: TokenStream,
    span: Span,
}

<span class="kw">impl </span>Group {
    <span class="kw">pub fn </span>new(delimiter: Delimiter, stream: TokenStream) -&gt; <span class="self">Self </span>{
        Group {
            delimiter,
            stream,
            span: Span::call_site(),
        }
    }

    <span class="kw">pub fn </span>delimiter(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Delimiter {
        <span class="self">self</span>.delimiter
    }

    <span class="kw">pub fn </span>stream(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; TokenStream {
        <span class="self">self</span>.stream.clone()
    }

    <span class="kw">pub fn </span>span(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Span {
        <span class="self">self</span>.span
    }

    <span class="kw">pub fn </span>span_open(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Span {
        <span class="self">self</span>.span.first_byte()
    }

    <span class="kw">pub fn </span>span_close(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Span {
        <span class="self">self</span>.span.last_byte()
    }

    <span class="kw">pub fn </span>set_span(<span class="kw-2">&amp;mut </span><span class="self">self</span>, span: Span) {
        <span class="self">self</span>.span = span;
    }
}

<span class="kw">impl </span>Display <span class="kw">for </span>Group {
    <span class="comment">// We attempt to match libproc_macro's formatting.
    // Empty parens: ()
    // Nonempty parens: (...)
    // Empty brackets: []
    // Nonempty brackets: [...]
    // Empty braces: { }
    // Nonempty braces: { ... }
    </span><span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">let </span>(open, close) = <span class="kw">match </span><span class="self">self</span>.delimiter {
            Delimiter::Parenthesis =&gt; (<span class="string">"("</span>, <span class="string">")"</span>),
            Delimiter::Brace =&gt; (<span class="string">"{ "</span>, <span class="string">"}"</span>),
            Delimiter::Bracket =&gt; (<span class="string">"["</span>, <span class="string">"]"</span>),
            Delimiter::None =&gt; (<span class="string">""</span>, <span class="string">""</span>),
        };

        f.write_str(open)<span class="question-mark">?</span>;
        Display::fmt(<span class="kw-2">&amp;</span><span class="self">self</span>.stream, f)<span class="question-mark">?</span>;
        <span class="kw">if </span><span class="self">self</span>.delimiter == Delimiter::Brace &amp;&amp; !<span class="self">self</span>.stream.inner.is_empty() {
            f.write_str(<span class="string">" "</span>)<span class="question-mark">?</span>;
        }
        f.write_str(close)<span class="question-mark">?</span>;

        <span class="prelude-val">Ok</span>(())
    }
}

<span class="kw">impl </span>Debug <span class="kw">for </span>Group {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, fmt: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">let </span><span class="kw-2">mut </span>debug = fmt.debug_struct(<span class="string">"Group"</span>);
        debug.field(<span class="string">"delimiter"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.delimiter);
        debug.field(<span class="string">"stream"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.stream);
        debug_span_field_if_nontrivial(<span class="kw-2">&amp;mut </span>debug, <span class="self">self</span>.span);
        debug.finish()
    }
}

<span class="attr">#[derive(Clone)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Ident {
    sym: String,
    span: Span,
    raw: bool,
}

<span class="kw">impl </span>Ident {
    <span class="attr">#[track_caller]
    </span><span class="kw">pub fn </span>new_checked(string: <span class="kw-2">&amp;</span>str, span: Span) -&gt; <span class="self">Self </span>{
        validate_ident(string);
        Ident::new_unchecked(string, span)
    }

    <span class="kw">pub fn </span>new_unchecked(string: <span class="kw-2">&amp;</span>str, span: Span) -&gt; <span class="self">Self </span>{
        Ident {
            sym: string.to_owned(),
            span,
            raw: <span class="bool-val">false</span>,
        }
    }

    <span class="attr">#[track_caller]
    </span><span class="kw">pub fn </span>new_raw_checked(string: <span class="kw-2">&amp;</span>str, span: Span) -&gt; <span class="self">Self </span>{
        validate_ident_raw(string);
        Ident::new_raw_unchecked(string, span)
    }

    <span class="kw">pub fn </span>new_raw_unchecked(string: <span class="kw-2">&amp;</span>str, span: Span) -&gt; <span class="self">Self </span>{
        Ident {
            sym: string.to_owned(),
            span,
            raw: <span class="bool-val">true</span>,
        }
    }

    <span class="kw">pub fn </span>span(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Span {
        <span class="self">self</span>.span
    }

    <span class="kw">pub fn </span>set_span(<span class="kw-2">&amp;mut </span><span class="self">self</span>, span: Span) {
        <span class="self">self</span>.span = span;
    }
}

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_ident_start(c: char) -&gt; bool {
    c == <span class="string">'_' </span>|| unicode_ident::is_xid_start(c)
}

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_ident_continue(c: char) -&gt; bool {
    unicode_ident::is_xid_continue(c)
}

<span class="attr">#[track_caller]
</span><span class="kw">fn </span>validate_ident(string: <span class="kw-2">&amp;</span>str) {
    <span class="kw">if </span>string.is_empty() {
        <span class="macro">panic!</span>(<span class="string">"Ident is not allowed to be empty; use Option&lt;Ident&gt;"</span>);
    }

    <span class="kw">if </span>string.bytes().all(|digit| <span class="string">b'0' </span>&lt;= digit &amp;&amp; digit &lt;= <span class="string">b'9'</span>) {
        <span class="macro">panic!</span>(<span class="string">"Ident cannot be a number; use Literal instead"</span>);
    }

    <span class="kw">fn </span>ident_ok(string: <span class="kw-2">&amp;</span>str) -&gt; bool {
        <span class="kw">let </span><span class="kw-2">mut </span>chars = string.chars();
        <span class="kw">let </span>first = chars.next().unwrap();
        <span class="kw">if </span>!is_ident_start(first) {
            <span class="kw">return </span><span class="bool-val">false</span>;
        }
        <span class="kw">for </span>ch <span class="kw">in </span>chars {
            <span class="kw">if </span>!is_ident_continue(ch) {
                <span class="kw">return </span><span class="bool-val">false</span>;
            }
        }
        <span class="bool-val">true
    </span>}

    <span class="kw">if </span>!ident_ok(string) {
        <span class="macro">panic!</span>(<span class="string">"{:?} is not a valid Ident"</span>, string);
    }
}

<span class="attr">#[track_caller]
</span><span class="kw">fn </span>validate_ident_raw(string: <span class="kw-2">&amp;</span>str) {
    validate_ident(string);

    <span class="kw">match </span>string {
        <span class="string">"_" </span>| <span class="string">"super" </span>| <span class="string">"self" </span>| <span class="string">"Self" </span>| <span class="string">"crate" </span>=&gt; {
            <span class="macro">panic!</span>(<span class="string">"`r#{}` cannot be a raw identifier"</span>, string);
        }
        <span class="kw">_ </span>=&gt; {}
    }
}

<span class="kw">impl </span>PartialEq <span class="kw">for </span>Ident {
    <span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>Ident) -&gt; bool {
        <span class="self">self</span>.sym == other.sym &amp;&amp; <span class="self">self</span>.raw == other.raw
    }
}

<span class="kw">impl</span>&lt;T&gt; PartialEq&lt;T&gt; <span class="kw">for </span>Ident
<span class="kw">where
    </span>T: <span class="question-mark">?</span>Sized + AsRef&lt;str&gt;,
{
    <span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>T) -&gt; bool {
        <span class="kw">let </span>other = other.as_ref();
        <span class="kw">if </span><span class="self">self</span>.raw {
            other.starts_with(<span class="string">"r#"</span>) &amp;&amp; <span class="self">self</span>.sym == other[<span class="number">2</span>..]
        } <span class="kw">else </span>{
            <span class="self">self</span>.sym == other
        }
    }
}

<span class="kw">impl </span>Display <span class="kw">for </span>Ident {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">if </span><span class="self">self</span>.raw {
            f.write_str(<span class="string">"r#"</span>)<span class="question-mark">?</span>;
        }
        Display::fmt(<span class="kw-2">&amp;</span><span class="self">self</span>.sym, f)
    }
}

<span class="attr">#[allow(clippy::missing_fields_in_debug)]
</span><span class="kw">impl </span>Debug <span class="kw">for </span>Ident {
    <span class="comment">// Ident(proc_macro), Ident(r#union)
    </span><span class="attr">#[cfg(not(span_locations))]
    </span><span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">let </span><span class="kw-2">mut </span>debug = f.debug_tuple(<span class="string">"Ident"</span>);
        debug.field(<span class="kw-2">&amp;</span><span class="macro">format_args!</span>(<span class="string">"{}"</span>, <span class="self">self</span>));
        debug.finish()
    }

    <span class="comment">// Ident {
    //     sym: proc_macro,
    //     span: bytes(128..138)
    // }
    </span><span class="attr">#[cfg(span_locations)]
    </span><span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">let </span><span class="kw-2">mut </span>debug = f.debug_struct(<span class="string">"Ident"</span>);
        debug.field(<span class="string">"sym"</span>, <span class="kw-2">&amp;</span><span class="macro">format_args!</span>(<span class="string">"{}"</span>, <span class="self">self</span>));
        debug_span_field_if_nontrivial(<span class="kw-2">&amp;mut </span>debug, <span class="self">self</span>.span);
        debug.finish()
    }
}

<span class="attr">#[derive(Clone)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Literal {
    <span class="kw">pub</span>(<span class="kw">crate</span>) repr: String,
    span: Span,
}

<span class="macro">macro_rules!</span> suffixed_numbers {
    ($(<span class="macro-nonterminal">$name</span>:ident =&gt; <span class="macro-nonterminal">$kind</span>:ident,)<span class="kw-2">*</span>) =&gt; ($(
        <span class="kw">pub fn </span><span class="macro-nonterminal">$name</span>(n: <span class="macro-nonterminal">$kind</span>) -&gt; Literal {
            Literal::_new(<span class="macro">format!</span>(<span class="macro">concat!</span>(<span class="string">"{}"</span>, <span class="macro">stringify!</span>(<span class="macro-nonterminal">$kind</span>)), n))
        }
    )<span class="kw-2">*</span>)
}

<span class="macro">macro_rules!</span> unsuffixed_numbers {
    ($(<span class="macro-nonterminal">$name</span>:ident =&gt; <span class="macro-nonterminal">$kind</span>:ident,)<span class="kw-2">*</span>) =&gt; ($(
        <span class="kw">pub fn </span><span class="macro-nonterminal">$name</span>(n: <span class="macro-nonterminal">$kind</span>) -&gt; Literal {
            Literal::_new(n.to_string())
        }
    )<span class="kw-2">*</span>)
}

<span class="kw">impl </span>Literal {
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>_new(repr: String) -&gt; <span class="self">Self </span>{
        Literal {
            repr,
            span: Span::call_site(),
        }
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">unsafe fn </span>from_str_unchecked(repr: <span class="kw-2">&amp;</span>str) -&gt; <span class="self">Self </span>{
        Literal::_new(repr.to_owned())
    }

    <span class="macro">suffixed_numbers!</span> {
        u8_suffixed =&gt; u8,
        u16_suffixed =&gt; u16,
        u32_suffixed =&gt; u32,
        u64_suffixed =&gt; u64,
        u128_suffixed =&gt; u128,
        usize_suffixed =&gt; usize,
        i8_suffixed =&gt; i8,
        i16_suffixed =&gt; i16,
        i32_suffixed =&gt; i32,
        i64_suffixed =&gt; i64,
        i128_suffixed =&gt; i128,
        isize_suffixed =&gt; isize,

        f32_suffixed =&gt; f32,
        f64_suffixed =&gt; f64,
    }

    <span class="macro">unsuffixed_numbers!</span> {
        u8_unsuffixed =&gt; u8,
        u16_unsuffixed =&gt; u16,
        u32_unsuffixed =&gt; u32,
        u64_unsuffixed =&gt; u64,
        u128_unsuffixed =&gt; u128,
        usize_unsuffixed =&gt; usize,
        i8_unsuffixed =&gt; i8,
        i16_unsuffixed =&gt; i16,
        i32_unsuffixed =&gt; i32,
        i64_unsuffixed =&gt; i64,
        i128_unsuffixed =&gt; i128,
        isize_unsuffixed =&gt; isize,
    }

    <span class="kw">pub fn </span>f32_unsuffixed(f: f32) -&gt; Literal {
        <span class="kw">let </span><span class="kw-2">mut </span>s = f.to_string();
        <span class="kw">if </span>!s.contains(<span class="string">'.'</span>) {
            s.push_str(<span class="string">".0"</span>);
        }
        Literal::_new(s)
    }

    <span class="kw">pub fn </span>f64_unsuffixed(f: f64) -&gt; Literal {
        <span class="kw">let </span><span class="kw-2">mut </span>s = f.to_string();
        <span class="kw">if </span>!s.contains(<span class="string">'.'</span>) {
            s.push_str(<span class="string">".0"</span>);
        }
        Literal::_new(s)
    }

    <span class="kw">pub fn </span>string(string: <span class="kw-2">&amp;</span>str) -&gt; Literal {
        <span class="kw">let </span><span class="kw-2">mut </span>repr = String::with_capacity(string.len() + <span class="number">2</span>);
        repr.push(<span class="string">'"'</span>);
        escape_utf8(string, <span class="kw-2">&amp;mut </span>repr);
        repr.push(<span class="string">'"'</span>);
        Literal::_new(repr)
    }

    <span class="kw">pub fn </span>character(ch: char) -&gt; Literal {
        <span class="kw">let </span><span class="kw-2">mut </span>repr = String::new();
        repr.push(<span class="string">'\''</span>);
        <span class="kw">if </span>ch == <span class="string">'"' </span>{
            <span class="comment">// escape_debug turns this into '\"' which is unnecessary.
            </span>repr.push(ch);
        } <span class="kw">else </span>{
            repr.extend(ch.escape_debug());
        }
        repr.push(<span class="string">'\''</span>);
        Literal::_new(repr)
    }

    <span class="kw">pub fn </span>byte_character(byte: u8) -&gt; Literal {
        <span class="kw">let </span><span class="kw-2">mut </span>repr = <span class="string">"b'"</span>.to_string();
        <span class="attr">#[allow(clippy::match_overlapping_arm)]
        </span><span class="kw">match </span>byte {
            <span class="string">b'\0' </span>=&gt; repr.push_str(<span class="string">r"\0"</span>),
            <span class="string">b'\t' </span>=&gt; repr.push_str(<span class="string">r"\t"</span>),
            <span class="string">b'\n' </span>=&gt; repr.push_str(<span class="string">r"\n"</span>),
            <span class="string">b'\r' </span>=&gt; repr.push_str(<span class="string">r"\r"</span>),
            <span class="string">b'\'' </span>=&gt; repr.push_str(<span class="string">r"\'"</span>),
            <span class="string">b'\\' </span>=&gt; repr.push_str(<span class="string">r"\\"</span>),
            <span class="string">b'\x20'</span>..=<span class="string">b'\x7E' </span>=&gt; repr.push(byte <span class="kw">as </span>char),
            <span class="kw">_ </span>=&gt; {
                <span class="kw">let _ </span>= <span class="macro">write!</span>(repr, <span class="string">r"\x{:02X}"</span>, byte);
            }
        }
        repr.push(<span class="string">'\''</span>);
        Literal::_new(repr)
    }

    <span class="kw">pub fn </span>byte_string(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; Literal {
        <span class="kw">let </span><span class="kw-2">mut </span>repr = <span class="string">"b\""</span>.to_string();
        <span class="kw">let </span><span class="kw-2">mut </span>bytes = bytes.iter();
        <span class="kw">while let </span><span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span>b) = bytes.next() {
            <span class="attr">#[allow(clippy::match_overlapping_arm)]
            </span><span class="kw">match </span>b {
                <span class="string">b'\0' </span>=&gt; repr.push_str(<span class="kw">match </span>bytes.as_slice().first() {
                    <span class="comment">// circumvent clippy::octal_escapes lint
                    </span><span class="prelude-val">Some</span>(<span class="string">b'0'</span>..=<span class="string">b'7'</span>) =&gt; <span class="string">r"\x00"</span>,
                    <span class="kw">_ </span>=&gt; <span class="string">r"\0"</span>,
                }),
                <span class="string">b'\t' </span>=&gt; repr.push_str(<span class="string">r"\t"</span>),
                <span class="string">b'\n' </span>=&gt; repr.push_str(<span class="string">r"\n"</span>),
                <span class="string">b'\r' </span>=&gt; repr.push_str(<span class="string">r"\r"</span>),
                <span class="string">b'"' </span>=&gt; repr.push_str(<span class="string">"\\\""</span>),
                <span class="string">b'\\' </span>=&gt; repr.push_str(<span class="string">r"\\"</span>),
                <span class="string">b'\x20'</span>..=<span class="string">b'\x7E' </span>=&gt; repr.push(b <span class="kw">as </span>char),
                <span class="kw">_ </span>=&gt; {
                    <span class="kw">let _ </span>= <span class="macro">write!</span>(repr, <span class="string">r"\x{:02X}"</span>, b);
                }
            }
        }
        repr.push(<span class="string">'"'</span>);
        Literal::_new(repr)
    }

    <span class="kw">pub fn </span>c_string(string: <span class="kw-2">&amp;</span>CStr) -&gt; Literal {
        <span class="kw">let </span><span class="kw-2">mut </span>repr = <span class="string">"c\""</span>.to_string();
        <span class="kw">let </span><span class="kw-2">mut </span>bytes = string.to_bytes();
        <span class="kw">while </span>!bytes.is_empty() {
            <span class="kw">let </span>(valid, invalid) = <span class="kw">match </span>str::from_utf8(bytes) {
                <span class="prelude-val">Ok</span>(all_valid) =&gt; {
                    bytes = <span class="string">b""</span>;
                    (all_valid, bytes)
                }
                <span class="prelude-val">Err</span>(utf8_error) =&gt; {
                    <span class="kw">let </span>(valid, rest) = bytes.split_at(utf8_error.valid_up_to());
                    <span class="kw">let </span>valid = str::from_utf8(valid).unwrap();
                    <span class="kw">let </span>invalid = utf8_error
                        .error_len()
                        .map_or(rest, |error_len| <span class="kw-2">&amp;</span>rest[..error_len]);
                    bytes = <span class="kw-2">&amp;</span>bytes[valid.len() + invalid.len()..];
                    (valid, invalid)
                }
            };
            escape_utf8(valid, <span class="kw-2">&amp;mut </span>repr);
            <span class="kw">for </span><span class="kw-2">&amp;</span>byte <span class="kw">in </span>invalid {
                <span class="kw">let _ </span>= <span class="macro">write!</span>(repr, <span class="string">r"\x{:02X}"</span>, byte);
            }
        }
        repr.push(<span class="string">'"'</span>);
        Literal::_new(repr)
    }

    <span class="kw">pub fn </span>span(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Span {
        <span class="self">self</span>.span
    }

    <span class="kw">pub fn </span>set_span(<span class="kw-2">&amp;mut </span><span class="self">self</span>, span: Span) {
        <span class="self">self</span>.span = span;
    }

    <span class="kw">pub fn </span>subspan&lt;R: RangeBounds&lt;usize&gt;&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, range: R) -&gt; <span class="prelude-ty">Option</span>&lt;Span&gt; {
        <span class="attr">#[cfg(not(span_locations))]
        </span>{
            <span class="kw">let _ </span>= range;
            <span class="prelude-val">None
        </span>}

        <span class="attr">#[cfg(span_locations)]
        </span>{
            <span class="kw">use </span>core::ops::Bound;

            <span class="kw">let </span>lo = <span class="kw">match </span>range.start_bound() {
                Bound::Included(start) =&gt; {
                    <span class="kw">let </span>start = u32::try_from(<span class="kw-2">*</span>start).ok()<span class="question-mark">?</span>;
                    <span class="self">self</span>.span.lo.checked_add(start)<span class="question-mark">?
                </span>}
                Bound::Excluded(start) =&gt; {
                    <span class="kw">let </span>start = u32::try_from(<span class="kw-2">*</span>start).ok()<span class="question-mark">?</span>;
                    <span class="self">self</span>.span.lo.checked_add(start)<span class="question-mark">?</span>.checked_add(<span class="number">1</span>)<span class="question-mark">?
                </span>}
                Bound::Unbounded =&gt; <span class="self">self</span>.span.lo,
            };
            <span class="kw">let </span>hi = <span class="kw">match </span>range.end_bound() {
                Bound::Included(end) =&gt; {
                    <span class="kw">let </span>end = u32::try_from(<span class="kw-2">*</span>end).ok()<span class="question-mark">?</span>;
                    <span class="self">self</span>.span.lo.checked_add(end)<span class="question-mark">?</span>.checked_add(<span class="number">1</span>)<span class="question-mark">?
                </span>}
                Bound::Excluded(end) =&gt; {
                    <span class="kw">let </span>end = u32::try_from(<span class="kw-2">*</span>end).ok()<span class="question-mark">?</span>;
                    <span class="self">self</span>.span.lo.checked_add(end)<span class="question-mark">?
                </span>}
                Bound::Unbounded =&gt; <span class="self">self</span>.span.hi,
            };
            <span class="kw">if </span>lo &lt;= hi &amp;&amp; hi &lt;= <span class="self">self</span>.span.hi {
                <span class="prelude-val">Some</span>(Span { lo, hi })
            } <span class="kw">else </span>{
                <span class="prelude-val">None
            </span>}
        }
    }
}

<span class="kw">impl </span>FromStr <span class="kw">for </span>Literal {
    <span class="kw">type </span><span class="prelude-val">Err </span>= LexError;

    <span class="kw">fn </span>from_str(repr: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, <span class="self">Self</span>::Err&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>cursor = get_cursor(repr);
        <span class="attr">#[cfg(span_locations)]
        </span><span class="kw">let </span>lo = cursor.off;

        <span class="kw">let </span>negative = cursor.starts_with_char(<span class="string">'-'</span>);
        <span class="kw">if </span>negative {
            cursor = cursor.advance(<span class="number">1</span>);
            <span class="kw">if </span>!cursor.starts_with_fn(|ch| ch.is_ascii_digit()) {
                <span class="kw">return </span><span class="prelude-val">Err</span>(LexError::call_site());
            }
        }

        <span class="kw">if let </span><span class="prelude-val">Ok</span>((rest, <span class="kw-2">mut </span>literal)) = parse::literal(cursor) {
            <span class="kw">if </span>rest.is_empty() {
                <span class="kw">if </span>negative {
                    literal.repr.insert(<span class="number">0</span>, <span class="string">'-'</span>);
                }
                literal.span = Span {
                    <span class="attr">#[cfg(span_locations)]
                    </span>lo,
                    <span class="attr">#[cfg(span_locations)]
                    </span>hi: rest.off,
                };
                <span class="kw">return </span><span class="prelude-val">Ok</span>(literal);
            }
        }
        <span class="prelude-val">Err</span>(LexError::call_site())
    }
}

<span class="kw">impl </span>Display <span class="kw">for </span>Literal {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        Display::fmt(<span class="kw-2">&amp;</span><span class="self">self</span>.repr, f)
    }
}

<span class="kw">impl </span>Debug <span class="kw">for </span>Literal {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, fmt: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="kw">let </span><span class="kw-2">mut </span>debug = fmt.debug_struct(<span class="string">"Literal"</span>);
        debug.field(<span class="string">"lit"</span>, <span class="kw-2">&amp;</span><span class="macro">format_args!</span>(<span class="string">"{}"</span>, <span class="self">self</span>.repr));
        debug_span_field_if_nontrivial(<span class="kw-2">&amp;mut </span>debug, <span class="self">self</span>.span);
        debug.finish()
    }
}

<span class="kw">fn </span>escape_utf8(string: <span class="kw-2">&amp;</span>str, repr: <span class="kw-2">&amp;mut </span>String) {
    <span class="kw">let </span><span class="kw-2">mut </span>chars = string.chars();
    <span class="kw">while let </span><span class="prelude-val">Some</span>(ch) = chars.next() {
        <span class="kw">if </span>ch == <span class="string">'\0' </span>{
            repr.push_str(
                <span class="kw">if </span>chars
                    .as_str()
                    .starts_with(|next| <span class="string">'0' </span>&lt;= next &amp;&amp; next &lt;= <span class="string">'7'</span>)
                {
                    <span class="comment">// circumvent clippy::octal_escapes lint
                    </span><span class="string">r"\x00"
                </span>} <span class="kw">else </span>{
                    <span class="string">r"\0"
                </span>},
            );
        } <span class="kw">else if </span>ch == <span class="string">'\'' </span>{
            <span class="comment">// escape_debug turns this into "\'" which is unnecessary.
            </span>repr.push(ch);
        } <span class="kw">else </span>{
            repr.extend(ch.escape_debug());
        }
    }
}
</code></pre></div></section></main></body></html>