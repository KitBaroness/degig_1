<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/brotli-4.0.0/src/enc/backward_references/hash_to_binary_tree.rs`."><title>hash_to_binary_tree.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../" data-static-root-path="../../../../static.files/" data-current-crate="brotli" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../src-files.js"></script><script defer src="../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../brotli/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="attr">#![allow(dead_code, unused_imports)]
</span><span class="kw">use super</span>::{
    kDistanceCacheIndex, kDistanceCacheOffset, kHashMul32, kHashMul64, kHashMul64Long,
    kInvalidMatch, AnyHasher, BrotliEncoderParams, BrotliHasherParams, CloneWithAlloc, H9Opts,
    HasherSearchResult, HowPrepared, Struct1,
};
<span class="kw">use </span>alloc;
<span class="kw">use </span>alloc::{Allocator, SliceWrapper, SliceWrapperMut};
<span class="kw">use </span>core;
<span class="kw">use </span>enc::command::{
    CombineLengthCodes, Command, CommandCopyLen, ComputeDistanceCode, GetCopyLengthCode,
    GetInsertLengthCode, InitCommand, PrefixEncodeCopyDistance,
};
<span class="kw">use </span>enc::constants::{kCopyExtra, kInsExtra};
<span class="kw">use </span>enc::dictionary_hash::kStaticDictionaryHash;
<span class="kw">use </span>enc::literal_cost::BrotliEstimateBitCostsForLiterals;
<span class="kw">use </span>enc::static_dict::{
    kBrotliEncDictionary, BrotliDictionary, BrotliFindAllStaticDictionaryMatches,
};
<span class="kw">use </span>enc::static_dict::{
    FindMatchLengthWithLimit, BROTLI_UNALIGNED_LOAD32, BROTLI_UNALIGNED_LOAD64,
};
<span class="kw">use </span>enc::util::{brotli_max_size_t, floatX, FastLog2, Log2FloorNonZero};

<span class="kw">pub const </span>kInfinity: floatX = <span class="number">1.7e38 </span><span class="kw">as </span>floatX;
<span class="attr">#[derive(Clone, Copy, Debug)]
</span><span class="kw">pub enum </span>Union1 {
    cost(floatX),
    next(u32),
    shortcut(u32),
}

<span class="attr">#[derive(Clone, Copy, Debug)]
</span><span class="kw">pub struct </span>ZopfliNode {
    <span class="comment">//highest 7 bit is used to reconstruct the length code
    </span><span class="kw">pub </span>length: u32,
    <span class="comment">// distance associated with the length
    </span><span class="kw">pub </span>distance: u32,
    <span class="comment">// number of literal inserts before the copy; highest 5 bits contain distance short code + 1 (or zero if no short code)
    </span><span class="kw">pub </span>dcode_insert_length: u32,
    <span class="kw">pub </span>u: Union1,
}
<span class="kw">impl </span>Default <span class="kw">for </span>ZopfliNode {
    <span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        ZopfliNode {
            length: <span class="number">1</span>,
            distance: <span class="number">0</span>,
            dcode_insert_length: <span class="number">0</span>,
            u: Union1::cost(kInfinity),
        }
    }
}

<span class="kw">pub trait </span>Allocable&lt;T: Copy, AllocT: Allocator&lt;T&gt;&gt; {
    <span class="kw">fn </span>new(m: <span class="kw-2">&amp;mut </span>AllocT, init: T) -&gt; <span class="self">Self</span>;
    <span class="kw">fn </span>new_uninit(m: <span class="kw-2">&amp;mut </span>AllocT) -&gt; <span class="self">Self</span>;
    <span class="kw">fn </span>free(<span class="kw-2">&amp;mut </span><span class="self">self</span>, m: <span class="kw-2">&amp;mut </span>AllocT);
}
<span class="kw">pub trait </span>H10Params {
    <span class="kw">fn </span>max_tree_search_depth() -&gt; u32;
    <span class="kw">fn </span>max_tree_comp_length() -&gt; u32;
}

<span class="kw">pub struct </span>H10DefaultParams {}
<span class="kw">impl </span>H10Params <span class="kw">for </span>H10DefaultParams {
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>max_tree_search_depth() -&gt; u32 {
        <span class="number">64
    </span>}
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>max_tree_comp_length() -&gt; u32 {
        <span class="number">128
    </span>}
}

<span class="kw">const </span>BUCKET_BITS: usize = <span class="number">17</span>;

<span class="kw">pub struct </span>H10Buckets&lt;AllocU32: Allocator&lt;u32&gt;&gt;(AllocU32::AllocatedMemory);

<span class="kw">impl</span>&lt;AllocU32: Allocator&lt;u32&gt;&gt; Allocable&lt;u32, AllocU32&gt; <span class="kw">for </span>H10Buckets&lt;AllocU32&gt; {
    <span class="kw">fn </span>new(m: <span class="kw-2">&amp;mut </span>AllocU32, initializer: u32) -&gt; H10Buckets&lt;AllocU32&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>ret = m.alloc_cell(<span class="number">1 </span>&lt;&lt; BUCKET_BITS);
        <span class="kw">for </span>item <span class="kw">in </span>ret.slice_mut().iter_mut() {
            <span class="kw-2">*</span>item = initializer;
        }
        H10Buckets::&lt;AllocU32&gt;(ret)
    }
    <span class="kw">fn </span>new_uninit(m: <span class="kw-2">&amp;mut </span>AllocU32) -&gt; H10Buckets&lt;AllocU32&gt; {
        H10Buckets::&lt;AllocU32&gt;(m.alloc_cell(<span class="number">1 </span>&lt;&lt; BUCKET_BITS))
    }
    <span class="kw">fn </span>free(<span class="kw-2">&amp;mut </span><span class="self">self</span>, m: <span class="kw-2">&amp;mut </span>AllocU32) {
        m.free_cell(core::mem::take(<span class="kw-2">&amp;mut </span><span class="self">self</span>.<span class="number">0</span>));
    }
}

<span class="kw">impl</span>&lt;AllocU32: Allocator&lt;u32&gt;&gt; PartialEq&lt;H10Buckets&lt;AllocU32&gt;&gt; <span class="kw">for </span>H10Buckets&lt;AllocU32&gt; {
    <span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>H10Buckets&lt;AllocU32&gt;) -&gt; bool {
        <span class="kw">return </span><span class="self">self</span>.<span class="number">0</span>.slice() == other.<span class="number">0</span>.slice();
    }
}

<span class="kw">impl</span>&lt;AllocU32: Allocator&lt;u32&gt;&gt; SliceWrapper&lt;u32&gt; <span class="kw">for </span>H10Buckets&lt;AllocU32&gt; {
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>slice(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>[u32] {
        <span class="self">self</span>.<span class="number">0</span>.slice()
    }
}
<span class="kw">impl</span>&lt;AllocU32: Allocator&lt;u32&gt;&gt; SliceWrapperMut&lt;u32&gt; <span class="kw">for </span>H10Buckets&lt;AllocU32&gt; {
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>slice_mut(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span>[u32] {
        <span class="self">self</span>.<span class="number">0</span>.slice_mut()
    }
}

<span class="kw">pub struct </span>H10&lt;
    AllocU32: Allocator&lt;u32&gt;,
    Buckets: Allocable&lt;u32, AllocU32&gt; + SliceWrapperMut&lt;u32&gt; + SliceWrapper&lt;u32&gt;,
    Params: H10Params,
&gt; <span class="kw">where
    </span>Buckets: PartialEq&lt;Buckets&gt;,
{
    <span class="kw">pub </span>window_mask_: usize,
    <span class="kw">pub </span>common: Struct1,
    <span class="kw">pub </span>buckets_: Buckets,
    <span class="kw">pub </span>invalid_pos_: u32,
    <span class="kw">pub </span>forest: AllocU32::AllocatedMemory,
    <span class="kw">pub </span>_params: core::marker::PhantomData&lt;Params&gt;,
}

<span class="kw">impl</span>&lt;
        AllocU32: Allocator&lt;u32&gt;,
        Buckets: Allocable&lt;u32, AllocU32&gt; + SliceWrapperMut&lt;u32&gt; + SliceWrapper&lt;u32&gt;,
        Params: H10Params,
    &gt; PartialEq&lt;H10&lt;AllocU32, Buckets, Params&gt;&gt; <span class="kw">for </span>H10&lt;AllocU32, Buckets, Params&gt;
<span class="kw">where
    </span>Buckets: PartialEq&lt;Buckets&gt;,
{
    <span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>H10&lt;AllocU32, Buckets, Params&gt;) -&gt; bool {
        <span class="self">self</span>.window_mask_ == other.window_mask_
            &amp;&amp; <span class="self">self</span>.common == other.common
            &amp;&amp; <span class="self">self</span>.buckets_ == other.buckets_
            &amp;&amp; <span class="self">self</span>.invalid_pos_ == other.invalid_pos_
            &amp;&amp; <span class="self">self</span>.forest.slice() == other.forest.slice()
            &amp;&amp; <span class="self">self</span>._params == other._params
    }
}

<span class="kw">pub fn </span>InitializeH10&lt;AllocU32: Allocator&lt;u32&gt;&gt;(
    m32: <span class="kw-2">&amp;mut </span>AllocU32,
    one_shot: bool,
    params: <span class="kw-2">&amp;</span>BrotliEncoderParams,
    input_size: usize,
) -&gt; H10&lt;AllocU32, H10Buckets&lt;AllocU32&gt;, H10DefaultParams&gt; {
    initialize_h10::&lt;AllocU32, H10Buckets&lt;AllocU32&gt;&gt;(m32, one_shot, params, input_size)
}
<span class="kw">fn </span>initialize_h10&lt;
    AllocU32: Allocator&lt;u32&gt;,
    Buckets: SliceWrapperMut&lt;u32&gt; + SliceWrapper&lt;u32&gt; + Allocable&lt;u32, AllocU32&gt;,
&gt;(
    m32: <span class="kw-2">&amp;mut </span>AllocU32,
    one_shot: bool,
    params: <span class="kw-2">&amp;</span>BrotliEncoderParams,
    input_size: usize,
) -&gt; H10&lt;AllocU32, Buckets, H10DefaultParams&gt;
<span class="kw">where
    </span>Buckets: PartialEq&lt;Buckets&gt;,
{
    <span class="kw">let </span><span class="kw-2">mut </span>num_nodes = <span class="number">1 </span>&lt;&lt; params.lgwin;
    <span class="kw">if </span>one_shot &amp;&amp; input_size &lt; num_nodes {
        num_nodes = input_size;
    }
    <span class="kw">let </span>window_mask = (<span class="number">1 </span>&lt;&lt; params.lgwin) - <span class="number">1</span>;
    <span class="kw">let </span>invalid_pos = <span class="number">0u32</span>.wrapping_sub(window_mask);
    <span class="kw">let </span>buckets = &lt;Buckets <span class="kw">as </span>Allocable&lt;u32, AllocU32&gt;&gt;::new(m32, invalid_pos);
    H10::&lt;AllocU32, Buckets, H10DefaultParams&gt; {
        common: Struct1 {
            params: params.hasher,
            is_prepared_: <span class="number">1</span>,
            dict_num_lookups: <span class="number">0</span>,
            dict_num_matches: <span class="number">0</span>,
        },
        _params: core::marker::PhantomData::&lt;H10DefaultParams&gt;,
        window_mask_: window_mask <span class="kw">as </span>usize,
        invalid_pos_: invalid_pos,
        buckets_: buckets,
        forest: m32.alloc_cell(num_nodes * <span class="number">2</span>),
    }
}

<span class="kw">impl</span>&lt;
        AllocU32: Allocator&lt;u32&gt;,
        Buckets: Allocable&lt;u32, AllocU32&gt; + SliceWrapperMut&lt;u32&gt; + SliceWrapper&lt;u32&gt;,
        Params: H10Params,
    &gt; H10&lt;AllocU32, Buckets, Params&gt;
<span class="kw">where
    </span>Buckets: PartialEq&lt;Buckets&gt;,
{
    <span class="kw">pub fn </span>free(<span class="kw-2">&amp;mut </span><span class="self">self</span>, m32: <span class="kw-2">&amp;mut </span>AllocU32) {
        m32.free_cell(core::mem::take(<span class="kw-2">&amp;mut </span><span class="self">self</span>.forest));
        <span class="self">self</span>.buckets_.free(m32);
    }
}
<span class="kw">impl</span>&lt;
        Alloc: alloc::Allocator&lt;u16&gt; + alloc::Allocator&lt;u32&gt;,
        Buckets: Allocable&lt;u32, Alloc&gt; + SliceWrapperMut&lt;u32&gt; + SliceWrapper&lt;u32&gt;,
        Params: H10Params,
    &gt; CloneWithAlloc&lt;Alloc&gt; <span class="kw">for </span>H10&lt;Alloc, Buckets, Params&gt;
<span class="kw">where
    </span>Buckets: PartialEq&lt;Buckets&gt;,
{
    <span class="kw">fn </span>clone_with_alloc(<span class="kw-2">&amp;</span><span class="self">self</span>, m: <span class="kw-2">&amp;mut </span>Alloc) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>ret = H10::&lt;Alloc, Buckets, Params&gt; {
            window_mask_: <span class="self">self</span>.window_mask_,
            common: <span class="self">self</span>.common.clone(),
            buckets_: Buckets::new_uninit(m),
            invalid_pos_: <span class="self">self</span>.invalid_pos_,
            forest: &lt;Alloc <span class="kw">as </span>Allocator&lt;u32&gt;&gt;::alloc_cell(m, <span class="self">self</span>.forest.len()),
            _params: core::marker::PhantomData::&lt;Params&gt;,
        };
        ret.buckets_
            .slice_mut()
            .clone_from_slice(<span class="self">self</span>.buckets_.slice());
        ret.forest.slice_mut().clone_from_slice(<span class="self">self</span>.forest.slice());
        ret
    }
}

<span class="kw">impl</span>&lt;
        AllocU32: Allocator&lt;u32&gt;,
        Buckets: Allocable&lt;u32, AllocU32&gt; + SliceWrapperMut&lt;u32&gt; + SliceWrapper&lt;u32&gt;,
        Params: H10Params,
    &gt; AnyHasher <span class="kw">for </span>H10&lt;AllocU32, Buckets, Params&gt;
<span class="kw">where
    </span>Buckets: PartialEq&lt;Buckets&gt;,
{
    <span class="comment">/*  fn GetH10Tree(&amp;mut self) -&gt; Option&lt;&amp;mut H10&lt;AllocU32, Buckets, H10Params&gt;&gt; {
      Some(self)
    }*/
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>Opts(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; H9Opts {
        H9Opts {
            literal_byte_score: <span class="number">340</span>,
        }
    }
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>PrepareDistanceCache(<span class="kw-2">&amp;</span><span class="self">self</span>, _distance_cache: <span class="kw-2">&amp;mut </span>[i32]) {}
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>HashTypeLength(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="number">4
    </span>}
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>StoreLookahead(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        Params::max_tree_comp_length() <span class="kw">as </span>usize
    }
    <span class="kw">fn </span>StitchToPreviousBlock(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        num_bytes: usize,
        position: usize,
        ringbuffer: <span class="kw-2">&amp;</span>[u8],
        ringbuffer_mask: usize,
    ) {
        <span class="kw">super</span>::hq::StitchToPreviousBlockH10(<span class="self">self</span>, num_bytes, position, ringbuffer, ringbuffer_mask)
    }
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>GetHasherCommon(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span>Struct1 {
        <span class="kw-2">&amp;mut </span><span class="self">self</span>.common
    }
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>HashBytes(<span class="kw-2">&amp;</span><span class="self">self</span>, data: <span class="kw-2">&amp;</span>[u8]) -&gt; usize {
        <span class="kw">let </span>h = BROTLI_UNALIGNED_LOAD32(data).wrapping_mul(kHashMul32);
        (h &gt;&gt; (<span class="number">32i32 </span>- BUCKET_BITS <span class="kw">as </span>i32)) <span class="kw">as </span>usize
    }
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>Store(<span class="kw-2">&amp;mut </span><span class="self">self</span>, data: <span class="kw-2">&amp;</span>[u8], mask: usize, ix: usize) {
        <span class="kw">let </span>max_backward: usize = <span class="self">self</span>.window_mask_.wrapping_sub(<span class="number">16</span>).wrapping_add(<span class="number">1</span>);
        StoreAndFindMatchesH10(
            <span class="self">self</span>,
            data,
            ix,
            mask,
            Params::max_tree_comp_length() <span class="kw">as </span>usize,
            max_backward,
            <span class="kw-2">&amp;mut </span><span class="number">0</span>,
            <span class="kw-2">&amp;mut </span>[],
        );
    }
    <span class="kw">fn </span>StoreRange(<span class="kw-2">&amp;mut </span><span class="self">self</span>, data: <span class="kw-2">&amp;</span>[u8], mask: usize, ix_start: usize, ix_end: usize) {
        <span class="kw">let </span><span class="kw-2">mut </span>i: usize = ix_start;
        <span class="kw">let </span><span class="kw-2">mut </span>j: usize = ix_start;
        <span class="kw">if </span>ix_start.wrapping_add(<span class="number">63</span>) &lt;= ix_end {
            i = ix_end.wrapping_sub(<span class="number">63</span>);
        }
        <span class="kw">if </span>ix_start.wrapping_add(<span class="number">512</span>) &lt;= i {
            <span class="kw">while </span>j &lt; i {
                {
                    <span class="self">self</span>.Store(data, mask, j);
                }
                j = j.wrapping_add(<span class="number">8</span>);
            }
        }
        <span class="kw">while </span>i &lt; ix_end {
            {
                <span class="self">self</span>.Store(data, mask, i);
            }
            i = i.wrapping_add(<span class="number">1</span>);
        }
    }
    <span class="kw">fn </span>BulkStoreRange(<span class="kw-2">&amp;mut </span><span class="self">self</span>, data: <span class="kw-2">&amp;</span>[u8], mask: usize, ix_start: usize, ix_end: usize) {
        <span class="kw">for </span>i <span class="kw">in </span>ix_start..ix_end {
            <span class="self">self</span>.Store(data, mask, i);
        }
    }
    <span class="kw">fn </span>Prepare(<span class="kw-2">&amp;mut </span><span class="self">self</span>, _one_shot: bool, _input_size: usize, _data: <span class="kw-2">&amp;</span>[u8]) -&gt; HowPrepared {
        <span class="kw">if </span><span class="self">self</span>.common.is_prepared_ != <span class="number">0 </span>{
            <span class="kw">return </span>HowPrepared::ALREADY_PREPARED;
        }
        <span class="kw">let </span>invalid_pos = <span class="self">self</span>.invalid_pos_;
        <span class="kw">for </span>bucket <span class="kw">in </span><span class="self">self</span>.buckets_.slice_mut().iter_mut() {
            <span class="kw-2">*</span>bucket = invalid_pos;
        }
        <span class="self">self</span>.common.is_prepared_ = <span class="number">1</span>;
        HowPrepared::NEWLY_PREPARED
    }

    <span class="kw">fn </span>FindLongestMatch(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        _dictionary: <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>BrotliDictionary&gt;,
        _dictionary_hash: <span class="kw-2">&amp;</span>[u16],
        _data: <span class="kw-2">&amp;</span>[u8],
        _ring_buffer_mask: usize,
        _distance_cache: <span class="kw-2">&amp;</span>[i32],
        _cur_ix: usize,
        _max_length: usize,
        _max_backward: usize,
        _gap: usize,
        _max_distance: usize,
        _out: <span class="kw-2">&amp;mut </span>HasherSearchResult,
    ) -&gt; bool {
        <span class="macro">unimplemented!</span>();
    }
}

<span class="kw">pub struct </span>BackwardMatch(<span class="kw">pub </span>u64);

<span class="comment">//    pub distance : u32,
//    pub length_and_code : u32,
</span><span class="kw">impl </span>BackwardMatch {
    <span class="attr">#[inline(always)]
    </span><span class="kw">pub fn </span>distance(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u32 {
        <span class="self">self</span>.<span class="number">0 </span><span class="kw">as </span>u32
    }
    <span class="attr">#[inline(always)]
    </span><span class="kw">pub fn </span>length_and_code(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u32 {
        (<span class="self">self</span>.<span class="number">0 </span>&gt;&gt; <span class="number">32</span>) <span class="kw">as </span>u32
    }
}
<span class="kw">pub struct </span>BackwardMatchMut&lt;<span class="lifetime">'a</span>&gt;(<span class="kw">pub </span><span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>u64);

<span class="comment">//    pub distance : u32,
//    pub length_and_code : u32,
</span><span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; BackwardMatchMut&lt;<span class="lifetime">'a</span>&gt; {
    <span class="attr">#[inline(always)]
    </span><span class="kw">pub fn </span>distance(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u32 {
        <span class="kw-2">*</span><span class="self">self</span>.<span class="number">0 </span><span class="kw">as </span>u32
    }
    <span class="attr">#[inline(always)]
    </span><span class="kw">pub fn </span>length_and_code(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u32 {
        (<span class="kw-2">*</span><span class="self">self</span>.<span class="number">0 </span>&gt;&gt; <span class="number">32</span>) <span class="kw">as </span>u32
    }
    <span class="attr">#[inline(always)]
    </span><span class="kw">pub fn </span>set_distance(<span class="kw-2">&amp;mut </span><span class="self">self</span>, data: u32) {
        <span class="kw-2">*</span><span class="self">self</span>.<span class="number">0 </span>&amp;= <span class="number">0xffffffff00000000</span>;
        <span class="kw-2">*</span><span class="self">self</span>.<span class="number">0 </span>|= u64::from(data)
    }
    <span class="attr">#[inline(always)]
    </span><span class="kw">pub fn </span>set_length_and_code(<span class="kw-2">&amp;mut </span><span class="self">self</span>, data: u32) {
        <span class="kw-2">*</span><span class="self">self</span>.<span class="number">0 </span>= u64::from((<span class="kw-2">*</span><span class="self">self</span>.<span class="number">0</span>) <span class="kw">as </span>u32) | (u64::from(data) &lt;&lt; <span class="number">32</span>);
    }
}

<span class="attr">#[inline(always)]
</span><span class="kw">pub fn </span>InitBackwardMatch(xself: <span class="kw-2">&amp;mut </span>BackwardMatchMut, dist: usize, len: usize) {
    xself.set_distance(dist <span class="kw">as </span>u32);
    xself.set_length_and_code((len &lt;&lt; <span class="number">5</span>) <span class="kw">as </span>u32);
}

<span class="macro">macro_rules!</span> LeftChildIndexH10 {
    (<span class="macro-nonterminal">$xself</span>: expr, <span class="macro-nonterminal">$pos</span>: expr) =&gt; {
        (<span class="number">2usize</span>).wrapping_mul(<span class="macro-nonterminal">$pos </span>&amp; (<span class="kw-2">*</span><span class="macro-nonterminal">$xself</span>).window_mask_)
    };
}
<span class="macro">macro_rules!</span> RightChildIndexH10 {
    (<span class="macro-nonterminal">$xself</span>: expr, <span class="macro-nonterminal">$pos</span>: expr) =&gt; {
        (<span class="number">2usize</span>)
            .wrapping_mul(<span class="macro-nonterminal">$pos </span>&amp; (<span class="kw-2">*</span><span class="macro-nonterminal">$xself</span>).window_mask_)
            .wrapping_add(<span class="number">1</span>)
    };
}
<span class="comment">/*
fn LeftChildIndexH10&lt;AllocU32: Allocator&lt;u32&gt;,
     Buckets: Allocable&lt;u32, AllocU32&gt;+SliceWrapperMut&lt;u32&gt;+SliceWrapper&lt;u32&gt;,
     Params:H10Params&gt;(
    mut xself : &amp;mut H10&lt;AllocU32, Buckets, Params&gt;, pos : usize
) -&gt; usize {
    (2usize).wrapping_mul(pos &amp; xself.window_mask_)
}

fn RightChildIndexH10&lt;AllocU32: Allocator&lt;u32&gt;,
     Buckets: Allocable&lt;u32, AllocU32&gt;+SliceWrapperMut&lt;u32&gt;+SliceWrapper&lt;u32&gt;,
     Params:H10Params&gt;(
    mut xself : &amp;mut H10&lt;AllocU32, Buckets, Params&gt;, pos : usize
) -&gt; usize {
    (2usize).wrapping_mul(
        pos &amp; xself.window_mask_
    ).wrapping_add(
        1
    )
}
*/

</span><span class="kw">pub fn </span>StoreAndFindMatchesH10&lt;
    AllocU32: Allocator&lt;u32&gt;,
    Buckets: Allocable&lt;u32, AllocU32&gt; + SliceWrapperMut&lt;u32&gt; + SliceWrapper&lt;u32&gt;,
    Params: H10Params,
&gt;(
    xself: <span class="kw-2">&amp;mut </span>H10&lt;AllocU32, Buckets, Params&gt;,
    data: <span class="kw-2">&amp;</span>[u8],
    cur_ix: usize,
    ring_buffer_mask: usize,
    max_length: usize,
    max_backward: usize,
    best_len: <span class="kw-2">&amp;mut </span>usize,
    matches: <span class="kw-2">&amp;mut </span>[u64],
) -&gt; usize
<span class="kw">where
    </span>Buckets: PartialEq&lt;Buckets&gt;,
{
    <span class="kw">let </span><span class="kw-2">mut </span>matches_offset = <span class="number">0usize</span>;
    <span class="kw">let </span>cur_ix_masked: usize = cur_ix &amp; ring_buffer_mask;
    <span class="kw">let </span>max_comp_len: usize = core::cmp::min(max_length, <span class="number">128usize</span>);
    <span class="kw">let </span>should_reroot_tree = max_length &gt;= <span class="number">128</span>;
    <span class="kw">let </span>key = xself.HashBytes(<span class="kw-2">&amp;</span>data[cur_ix_masked..]);
    <span class="kw">let </span>forest: <span class="kw-2">&amp;mut </span>[u32] = xself.forest.slice_mut();
    <span class="kw">let </span><span class="kw-2">mut </span>prev_ix: usize = xself.buckets_.slice()[key] <span class="kw">as </span>usize;
    <span class="kw">let </span><span class="kw-2">mut </span>node_left: usize = <span class="macro">LeftChildIndexH10!</span>(xself, cur_ix);
    <span class="kw">let </span><span class="kw-2">mut </span>node_right: usize = <span class="macro">RightChildIndexH10!</span>(xself, cur_ix);
    <span class="kw">let </span><span class="kw-2">mut </span>best_len_left: usize = <span class="number">0usize</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>best_len_right: usize = <span class="number">0usize</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>depth_remaining: usize;
    <span class="kw">if </span>should_reroot_tree {
        xself.buckets_.slice_mut()[key] = cur_ix <span class="kw">as </span>u32;
    }
    depth_remaining = <span class="number">64usize</span>;
    <span class="lifetime">'break16</span>: <span class="kw">loop </span>{
        {
            <span class="kw">let </span>backward: usize = cur_ix.wrapping_sub(prev_ix);
            <span class="kw">let </span>prev_ix_masked: usize = prev_ix &amp; ring_buffer_mask;
            <span class="kw">if </span>backward == <span class="number">0usize </span>|| backward &gt; max_backward || depth_remaining == <span class="number">0usize </span>{
                <span class="kw">if </span>should_reroot_tree {
                    forest[node_left] = xself.invalid_pos_;
                    forest[node_right] = xself.invalid_pos_;
                }
                <span class="kw">break </span><span class="lifetime">'break16</span>;
            }
            {
                <span class="kw">let </span>cur_len: usize = core::cmp::min(best_len_left, best_len_right);

                <span class="kw">let </span>len: usize = cur_len.wrapping_add(FindMatchLengthWithLimit(
                    <span class="kw-2">&amp;</span>data[cur_ix_masked.wrapping_add(cur_len)..],
                    <span class="kw-2">&amp;</span>data[prev_ix_masked.wrapping_add(cur_len)..],
                    max_length.wrapping_sub(cur_len),
                ));
                <span class="kw">if </span>matches_offset != matches.len() &amp;&amp; (len &gt; <span class="kw-2">*</span>best_len) {
                    <span class="kw-2">*</span>best_len = len;
                    InitBackwardMatch(
                        <span class="kw-2">&amp;mut </span>BackwardMatchMut(<span class="kw-2">&amp;mut </span>matches[matches_offset]),
                        backward,
                        len,
                    );
                    matches_offset += <span class="number">1</span>;
                }
                <span class="kw">if </span>len &gt;= max_comp_len {
                    <span class="kw">if </span>should_reroot_tree {
                        forest[node_left] = forest[<span class="macro">LeftChildIndexH10!</span>(xself, prev_ix)];
                        forest[node_right] = forest[<span class="macro">RightChildIndexH10!</span>(xself, prev_ix)];
                    }
                    <span class="kw">break </span><span class="lifetime">'break16</span>;
                }
                <span class="kw">if </span>data[cur_ix_masked.wrapping_add(len)] <span class="kw">as </span>i32
                    &gt; data[prev_ix_masked.wrapping_add(len)] <span class="kw">as </span>i32
                {
                    best_len_left = len;
                    <span class="kw">if </span>should_reroot_tree {
                        forest[node_left] = prev_ix <span class="kw">as </span>u32;
                    }
                    node_left = <span class="macro">RightChildIndexH10!</span>(xself, prev_ix);
                    prev_ix = forest[node_left] <span class="kw">as </span>usize;
                } <span class="kw">else </span>{
                    best_len_right = len;
                    <span class="kw">if </span>should_reroot_tree {
                        forest[node_right] = prev_ix <span class="kw">as </span>u32;
                    }
                    node_right = <span class="macro">LeftChildIndexH10!</span>(xself, prev_ix);
                    prev_ix = forest[node_right] <span class="kw">as </span>usize;
                }
            }
        }
        depth_remaining = depth_remaining.wrapping_sub(<span class="number">1</span>);
    }
    matches_offset
}
</code></pre></div></section></main></body></html>