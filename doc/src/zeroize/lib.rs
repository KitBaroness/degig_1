<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/zeroize-1.3.0/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="zeroize" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../zeroize/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Securely zero memory with a simple trait ([Zeroize]) built on stable Rust
//! primitives which guarantee the operation will not be "optimized away".
//!
//! ## About
//!
//! [Zeroing memory securely is hard] - compilers optimize for performance, and
//! in doing so they love to "optimize away" unnecessary zeroing calls. There are
//! many documented "tricks" to attempt to avoid these optimizations and ensure
//! that a zeroing routine is performed reliably.
//!
//! This crate isn't about tricks: it uses [core::ptr::write_volatile]
//! and [core::sync::atomic] memory fences to provide easy-to-use, portable
//! zeroing behavior which works on all of Rust's core number types and slices
//! thereof, implemented in pure Rust with no usage of FFI or assembly.
//!
//! - No insecure fallbacks!
//! - No dependencies!
//! - No FFI or inline assembly! **WASM friendly** (and tested)!
//! - `#![no_std]` i.e. **embedded-friendly**!
//! - No functionality besides securely zeroing memory!
//! - (Optional) Custom derive support for zeroing complex structures
//!
//! ## Minimum Supported Rust Version
//!
//! Requires Rust **1.47** or newer.
//!
//! In the future, we reserve the right to change MSRV (i.e. MSRV is out-of-scope
//! for this crate's SemVer guarantees), however when we do it will be accompanied
//! by a minor version bump.
//!
//! ## Usage
//!
//! ```
//! use zeroize::Zeroize;
//!
//! fn main() {
//!     // Protip: don't embed secrets in your source code.
//!     // This is just an example.
//!     let mut secret = b"Air shield password: 1,2,3,4,5".to_vec();
//!     // [ ... ] open the air shield here
//!
//!     // Now that we're done using the secret, zero it out.
//!     secret.zeroize();
//! }
//! ```
//!
//! The [Zeroize] trait is impl'd on all of Rust's core scalar types including
//! integers, floats, `bool`, and `char`.
//!
//! Additionally, it's implemented on slices and `IterMut`s of the above types.
//!
//! When the `alloc` feature is enabled (which it is by default), it's also
//! impl'd for `Vec&lt;T&gt;` for the above types as well as `String`, where it provides
//! [Vec::clear()] / [String::clear()]-like behavior (truncating to zero-length)
//! but ensures the backing memory is securely zeroed with some caveats.
//! (NOTE: see "Stack/Heap Zeroing Notes" for important `Vec`/`String` details)
//!
//! The [DefaultIsZeroes] marker trait can be impl'd on types which also
//! impl [Default], which implements [Zeroize] by overwriting a value with
//! the default value.
//!
//! ## Custom Derive Support
//!
//! This crate has custom derive support for the `Zeroize` trait,
//! gated under the `zeroize` crate's `zeroize_derive` Cargo feature,
//! which automatically calls `zeroize()` on all members of a struct
//! or tuple struct.
//!
//! Additionally it supports the following attribute:
//!
//! - `#[zeroize(drop)]`: call `zeroize()` when this item is dropped
//!
//! Example which derives `Drop`:
//!
//! ```
//! # #[cfg(feature = "derive")]
//! # {
//! use zeroize::Zeroize;
//!
//! // This struct will be zeroized on drop
//! #[derive(Zeroize)]
//! #[zeroize(drop)]
//! struct MyStruct([u8; 32]);
//! # }
//! ```
//!
//! Example which does not derive `Drop` (useful for e.g. `Copy` types)
//!
//! ```
//! #[cfg(feature = "derive")]
//! # {
//! use zeroize::Zeroize;
//!
//! // This struct will *NOT* be zeroized on drop
//! #[derive(Copy, Clone, Zeroize)]
//! struct MyStruct([u8; 32]);
//! # }
//! ```
//!
//! ## `Zeroizing&lt;Z&gt;`: wrapper for zeroizing arbitrary values on drop
//!
//! `Zeroizing&lt;Z: Zeroize&gt;` is a generic wrapper type that impls `Deref`
//! and `DerefMut`, allowing access to an inner value of type `Z`, and also
//! impls a `Drop` handler which calls `zeroize()` on its contents:
//!
//! ```
//! use zeroize::Zeroizing;
//!
//! fn main() {
//!     let mut secret = Zeroizing::new([0u8; 5]);
//!
//!     // Set the air shield password
//!     // Protip (again): don't embed secrets in your source code.
//!     secret.copy_from_slice(&amp;[1, 2, 3, 4, 5]);
//!     assert_eq!(secret.as_ref(), &amp;[1, 2, 3, 4, 5]);
//!
//!     // The contents of `secret` will be automatically zeroized on drop
//! }
//! ```
//!
//! ## What guarantees does this crate provide?
//!
//! This crate guarantees the following:
//!
//! 1. The zeroing operation can't be "optimized away" by the compiler.
//! 2. All subsequent reads to memory will see "zeroized" values.
//!
//! LLVM's volatile semantics ensure #1 is true.
//!
//! Additionally, thanks to work by the [Unsafe Code Guidelines Working Group],
//! we can now fairly confidently say #2 is true as well. Previously there were
//! worries that the approach used by this crate (mixing volatile and
//! non-volatile accesses) was undefined behavior due to language contained
//! in the documentation for `write_volatile`, however after some discussion
//! [these remarks have been removed] and the specific usage pattern in this
//! crate is considered to be well-defined.
//!
//! Additionally this crate leverages [compiler_fence] from
//! [core::sync::atomic] with the strictest ordering ([Ordering::SeqCst])
//! as a precaution to help ensure reads are not reordered before memory has
//! been zeroed.
//!
//! All of that said, there is still potential for microarchitectural attacks
//! (ala Spectre/Meltdown) to leak "zeroized" secrets through covert channels.
//! This crate makes no guarantees that zeroized values cannot be leaked
//! through such channels, as they represent flaws in the underlying hardware.
//!
//! ## Stack/Heap Zeroing Notes
//!
//! This crate can be used to zero values from either the stack or the heap.
//!
//! However, be aware several operations in Rust can unintentionally leave
//! copies of data in memory. This includes but is not limited to:
//!
//! - Moves and `Copy`
//! - Heap reallocation when using `Vec` and `String`
//! - Borrowers of a reference making copies of the data
//!
//! [`Pin`][pin] can be leveraged in conjunction with this crate to ensure
//! data kept on the stack isn't moved.
//!
//! The `Zeroize` impls for `Vec` and `String` zeroize the entire capacity of
//! their backing buffer, but cannot guarantee copies of the data were not
//! previously made by buffer reallocation. It's therefore important when
//! attempting to zeroize such buffers to initialize them to the correct
//! capacity, and take care to prevent subsequent reallocation.
//!
//! The `secrecy` crate provides higher-level abstractions for eliminating
//! usage patterns which can cause reallocations:
//!
//! &lt;https://crates.io/crates/secrecy&gt;
//!
//! ## What about: clearing registers, mlock, mprotect, etc?
//!
//! This crate is focused on providing simple, unobtrusive support for reliably
//! zeroing memory using the best approach possible on stable Rust.
//!
//! Clearing registers is a difficult problem that can't easily be solved by
//! something like a crate, and requires either inline ASM or rustc support.
//! See &lt;https://github.com/rust-lang/rust/issues/17046&gt; for background on
//! this particular problem.
//!
//! Other memory protection mechanisms are interesting and useful, but often
//! overkill (e.g. defending against RAM scraping or attackers with swap access).
//! In as much as there may be merit to these approaches, there are also many
//! other crates that already implement more sophisticated memory protections.
//! Such protections are explicitly out-of-scope for this crate.
//!
//! Zeroing memory is [good cryptographic hygiene] and this crate seeks to promote
//! it in the most unobtrusive manner possible. This includes omitting complex
//! `unsafe` memory protection systems and just trying to make the best memory
//! zeroing crate available.
//!
//! [Zeroize]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html
//! [Zeroing memory securely is hard]: http://www.daemonology.net/blog/2014-09-04-how-to-zero-a-buffer.html
//! [Vec::clear()]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear
//! [String::clear()]: https://doc.rust-lang.org/std/string/struct.String.html#method.clear
//! [DefaultIsZeroes]: https://docs.rs/zeroize/latest/zeroize/trait.DefaultIsZeroes.html
//! [Default]: https://doc.rust-lang.org/std/default/trait.Default.html
//! [core::ptr::write_volatile]: https://doc.rust-lang.org/core/ptr/fn.write_volatile.html
//! [Unsafe Code Guidelines Working Group]: https://github.com/rust-lang/unsafe-code-guidelines
//! [these remarks have been removed]: https://github.com/rust-lang/rust/pull/60972
//! [core::sync::atomic]: https://doc.rust-lang.org/stable/core/sync/atomic/index.html
//! [Ordering::SeqCst]: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.SeqCst
//! [compiler_fence]: https://doc.rust-lang.org/stable/core/sync/atomic/fn.compiler_fence.html
//! [pin]: https://doc.rust-lang.org/std/pin/struct.Pin.html
//! [good cryptographic hygiene]: https://github.com/veorq/cryptocoding#clean-memory-of-secret-data

</span><span class="attr">#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = <span class="string">"https://docs.rs/zeroize/1.3.0"</span>)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = <span class="string">"alloc"</span>)]
#[cfg_attr(test, macro_use)]
</span><span class="kw">extern crate </span>alloc;

<span class="attr">#[cfg(feature = <span class="string">"zeroize_derive"</span>)]
#[cfg_attr(docsrs, doc(cfg(feature = <span class="string">"zeroize_derive"</span>)))]
</span><span class="kw">pub use </span>zeroize_derive::Zeroize;

<span class="attr">#[cfg(any(target_arch = <span class="string">"x86"</span>, target_arch = <span class="string">"x86_64"</span>))]
</span><span class="kw">mod </span>x86;

<span class="kw">use </span>core::{ops, ptr, slice::IterMut, sync::atomic};

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
</span><span class="kw">use </span>alloc::{boxed::Box, string::String, vec::Vec};

<span class="doccomment">/// Trait for securely erasing types from memory
</span><span class="kw">pub trait </span>Zeroize {
    <span class="doccomment">/// Zero out this object from memory using Rust intrinsics which ensure the
    /// zeroization operation is not "optimized away" by the compiler.
    </span><span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>);
}

<span class="doccomment">/// Marker trait for types whose `Default` is the desired zeroization result
</span><span class="kw">pub trait </span>DefaultIsZeroes: Copy + Default + Sized {}

<span class="kw">impl</span>&lt;Z&gt; Zeroize <span class="kw">for </span>Z
<span class="kw">where
    </span>Z: DefaultIsZeroes,
{
    <span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        volatile_write(<span class="self">self</span>, Z::default());
        atomic_fence();
    }
}

<span class="macro">macro_rules!</span> impl_zeroize_with_default {
    ($(<span class="macro-nonterminal">$</span><span class="kw">type</span>:<span class="macro-nonterminal">ty</span>),+) =&gt; {
        $(<span class="kw">impl </span>DefaultIsZeroes <span class="kw">for </span><span class="macro-nonterminal">$</span><span class="kw">type </span>{})+
    };
}

<span class="macro">impl_zeroize_with_default!</span>(<span class="macro-nonterminal">i8</span>, i16, i32, i64, i128, isize);
<span class="macro">impl_zeroize_with_default!</span>(u8, u16, u32, u64, u128, usize);
<span class="macro">impl_zeroize_with_default!</span>(f32, f64, char, bool);

<span class="doccomment">/// Implement `Zeroize` on arrays of types that impl `Zeroize`
</span><span class="macro">macro_rules!</span> impl_zeroize_for_array {
    ($(<span class="macro-nonterminal">$size</span>:expr),+) =&gt; {
        $(
            <span class="kw">impl</span>&lt;Z&gt; Zeroize <span class="kw">for </span>[Z; <span class="macro-nonterminal">$size</span>]
            <span class="kw">where
                </span>Z: Zeroize
            {
                <span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
                    <span class="self">self</span>.iter_mut().zeroize();
                }
            }
        )+
     };
}

<span class="comment">// TODO(tarcieri): const generics
</span><span class="macro">impl_zeroize_for_array!</span>(
    <span class="number">1</span>, <span class="number">2</span>, <span class="number">3</span>, <span class="number">4</span>, <span class="number">5</span>, <span class="number">6</span>, <span class="number">7</span>, <span class="number">8</span>, <span class="number">9</span>, <span class="number">10</span>, <span class="number">11</span>, <span class="number">12</span>, <span class="number">13</span>, <span class="number">14</span>, <span class="number">15</span>, <span class="number">16</span>, <span class="number">17</span>, <span class="number">18</span>, <span class="number">19</span>, <span class="number">20</span>, <span class="number">21</span>, <span class="number">22</span>, <span class="number">23</span>, <span class="number">24</span>, <span class="number">25</span>, <span class="number">26</span>,
    <span class="number">27</span>, <span class="number">28</span>, <span class="number">29</span>, <span class="number">30</span>, <span class="number">31</span>, <span class="number">32</span>, <span class="number">33</span>, <span class="number">34</span>, <span class="number">35</span>, <span class="number">36</span>, <span class="number">37</span>, <span class="number">38</span>, <span class="number">39</span>, <span class="number">40</span>, <span class="number">41</span>, <span class="number">42</span>, <span class="number">43</span>, <span class="number">44</span>, <span class="number">45</span>, <span class="number">46</span>, <span class="number">47</span>, <span class="number">48</span>, <span class="number">49</span>, <span class="number">50</span>,
    <span class="number">51</span>, <span class="number">52</span>, <span class="number">53</span>, <span class="number">54</span>, <span class="number">55</span>, <span class="number">56</span>, <span class="number">57</span>, <span class="number">58</span>, <span class="number">59</span>, <span class="number">60</span>, <span class="number">61</span>, <span class="number">62</span>, <span class="number">63</span>, <span class="number">64
</span>);

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, Z&gt; Zeroize <span class="kw">for </span>IterMut&lt;<span class="lifetime">'a</span>, Z&gt;
<span class="kw">where
    </span>Z: Zeroize,
{
    <span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">for </span>elem <span class="kw">in </span><span class="self">self </span>{
            elem.zeroize();
        }
    }
}

<span class="kw">impl</span>&lt;Z&gt; Zeroize <span class="kw">for </span><span class="prelude-ty">Option</span>&lt;Z&gt;
<span class="kw">where
    </span>Z: Zeroize,
{
    <span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">if let </span><span class="prelude-val">Some</span>(value) = <span class="self">self </span>{
            value.zeroize();

            <span class="comment">// Ensures self is None and that the value was dropped. Without the take, the drop
            // of the (zeroized) value isn't called, which might lead to a leak or other
            // unexpected behavior. For example, if this were Option&lt;Vec&lt;T&gt;&gt;, the above call to
            // zeroize would not free the allocated memory, but the the `take` call will.
            </span><span class="self">self</span>.take();
        }

        <span class="comment">// Ensure that if the `Option` were previously `Some` but a value was copied/moved out
        // that the remaining space in the `Option` is zeroized.
        //
        // Safety:
        //
        // The memory pointed to by `self` is valid for `mem::size_of::&lt;Self&gt;()` bytes.
        // It is also properly aligned, because `u8` has an alignment of `1`.
        </span><span class="kw">unsafe </span>{
            volatile_set(<span class="self">self </span><span class="kw">as </span><span class="kw-2">*mut </span><span class="kw">_ as </span><span class="kw-2">*mut </span>u8, <span class="number">0</span>, core::mem::size_of::&lt;<span class="self">Self</span>&gt;());
        }

        <span class="comment">// Ensures self is overwritten with the default bit pattern. volatile_write can't be
        // used because Option&lt;Z&gt; is not copy.
        //
        // Safety:
        //
        // self is safe to replace with the default, which the take() call above should have
        // already done semantically. Any value which needed to be dropped will have been
        // done so by take().
        </span><span class="kw">unsafe </span>{ ptr::write_volatile(<span class="self">self</span>, Option::default()) }

        atomic_fence();
    }
}

<span class="doccomment">/// Impl `Zeroize` on slices of types that can be zeroized with `Default`.
///
/// This impl can eventually be optimized using an memset intrinsic,
/// such as `core::intrinsics::volatile_set_memory`. For that reason the blanket
/// impl on slices is bounded by `DefaultIsZeroes`.
///
/// To zeroize a mut slice of `Z: Zeroize` which does not impl
/// `DefaultIsZeroes`, call `iter_mut().zeroize()`.
</span><span class="kw">impl</span>&lt;Z&gt; Zeroize <span class="kw">for </span>[Z]
<span class="kw">where
    </span>Z: DefaultIsZeroes,
{
    <span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="macro">assert!</span>(<span class="self">self</span>.len() &lt;= core::isize::MAX <span class="kw">as </span>usize);
        <span class="comment">// Safety:
        //
        // This is safe, because the slice is well aligned and is backed by a single allocated
        // object for at least `self.len()` elements of type `Z`.
        // `self.len()` is also not larger than an `isize`, because of the assertion above.
        // The memory of the slice should not wrap around the address space.
        </span><span class="kw">unsafe </span>{ volatile_set(<span class="self">self</span>.as_mut_ptr(), Z::default(), <span class="self">self</span>.len()) };
        atomic_fence();
    }
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
#[cfg_attr(docsrs, doc(cfg(feature = <span class="string">"alloc"</span>)))]
</span><span class="kw">impl</span>&lt;Z&gt; Zeroize <span class="kw">for </span>Vec&lt;Z&gt;
<span class="kw">where
    </span>Z: Zeroize,
{
    <span class="doccomment">/// "Best effort" zeroization for `Vec`.
    ///
    /// Ensures the entire capacity of the `Vec` is zeroed. Cannot ensure that
    /// previous reallocations did not leave values on the heap.
    </span><span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.iter_mut().zeroize();

        <span class="comment">// Zero the capacity of the `Vec` that is not initialized.
        </span>{
            <span class="comment">// Safety:
            //
            // This is safe, because `Vec` never allocates more than `isize::MAX` bytes.
            // This exact use case is even mentioned in the documentation of `pointer::add`.
            </span><span class="kw">let </span>extra_capacity_start = <span class="kw">unsafe </span>{ <span class="self">self</span>.as_mut_ptr().add(<span class="self">self</span>.len()) <span class="kw">as </span><span class="kw-2">*mut </span>u8 };
            <span class="kw">let </span>extra_capacity_len = <span class="self">self</span>.capacity().saturating_sub(<span class="self">self</span>.len());

            <span class="comment">// Safety:
            // The memory pointed to by `extra_capacity_start` is valid for `extra_capacity_len *
            // mem::size_of::&lt;Z&gt;()` bytes, because the allocation of the `Vec` has enough reported
            // capacity for elements of type `Z`.
            // It is also properly aligned, because the `T` here is `u8`, which has an alignment of
            // `1`.
            // `extra_capacity_len` is not larger than an `isize`, because `Vec` never allocates
            // more than `isize::MAX` bytes.
            // The `Vec` allocation also guarantees to never wrap around the address space.
            </span><span class="kw">unsafe </span>{
                volatile_set(
                    extra_capacity_start,
                    <span class="number">0</span>,
                    extra_capacity_len * core::mem::size_of::&lt;Z&gt;(),
                )
            };
            atomic_fence();
        }

        <span class="self">self</span>.clear();
    }
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
#[cfg_attr(docsrs, doc(cfg(feature = <span class="string">"alloc"</span>)))]
</span><span class="kw">impl</span>&lt;Z&gt; Zeroize <span class="kw">for </span>Box&lt;[Z]&gt;
<span class="kw">where
    </span>Z: Zeroize,
{
    <span class="doccomment">/// Unlike `Vec`, `Box&lt;[Z]&gt;` cannot reallocate, so we can be sure that we are not leaving
    /// values on the heap.
    </span><span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.iter_mut().zeroize();
    }
}

<span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
#[cfg_attr(docsrs, doc(cfg(feature = <span class="string">"alloc"</span>)))]
</span><span class="kw">impl </span>Zeroize <span class="kw">for </span>String {
    <span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">unsafe </span>{ <span class="self">self</span>.as_mut_vec() }.zeroize();
    }
}

<span class="doccomment">/// Fallible trait for representing cases where zeroization may or may not be
/// possible.
///
/// This is primarily useful for scenarios like reference counted data, where
/// zeroization is only possible when the last reference is dropped.
</span><span class="kw">pub trait </span>TryZeroize {
    <span class="doccomment">/// Try to zero out this object from memory using Rust intrinsics which
    /// ensure the zeroization operation is not "optimized away" by the
    /// compiler.
    </span><span class="attr">#[must_use]
    </span><span class="kw">fn </span>try_zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; bool;
}

<span class="doccomment">/// `Zeroizing` is a a wrapper for any `Z: Zeroize` type which implements a
/// `Drop` handler which zeroizes dropped values.
</span><span class="attr">#[derive(Clone, Debug, Eq, PartialEq)]
</span><span class="kw">pub struct </span>Zeroizing&lt;Z: Zeroize&gt;(Z);

<span class="kw">impl</span>&lt;Z&gt; Zeroizing&lt;Z&gt;
<span class="kw">where
    </span>Z: Zeroize,
{
    <span class="doccomment">/// Move value inside a `Zeroizing` wrapper which ensures it will be
    /// zeroized when it's dropped.
    </span><span class="kw">pub fn </span>new(value: Z) -&gt; <span class="self">Self </span>{
        value.into()
    }
}

<span class="kw">impl</span>&lt;Z&gt; From&lt;Z&gt; <span class="kw">for </span>Zeroizing&lt;Z&gt;
<span class="kw">where
    </span>Z: Zeroize,
{
    <span class="kw">fn </span>from(value: Z) -&gt; Zeroizing&lt;Z&gt; {
        Zeroizing(value)
    }
}

<span class="kw">impl</span>&lt;Z&gt; ops::Deref <span class="kw">for </span>Zeroizing&lt;Z&gt;
<span class="kw">where
    </span>Z: Zeroize,
{
    <span class="kw">type </span>Target = Z;

    <span class="kw">fn </span>deref(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>Z {
        <span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0
    </span>}
}

<span class="kw">impl</span>&lt;Z&gt; ops::DerefMut <span class="kw">for </span>Zeroizing&lt;Z&gt;
<span class="kw">where
    </span>Z: Zeroize,
{
    <span class="kw">fn </span>deref_mut(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span>Z {
        <span class="kw-2">&amp;mut </span><span class="self">self</span>.<span class="number">0
    </span>}
}

<span class="kw">impl</span>&lt;Z&gt; Zeroize <span class="kw">for </span>Zeroizing&lt;Z&gt;
<span class="kw">where
    </span>Z: Zeroize,
{
    <span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.<span class="number">0</span>.zeroize();
    }
}

<span class="kw">impl</span>&lt;Z&gt; Drop <span class="kw">for </span>Zeroizing&lt;Z&gt;
<span class="kw">where
    </span>Z: Zeroize,
{
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.<span class="number">0</span>.zeroize()
    }
}

<span class="doccomment">/// Use fences to prevent accesses from being reordered before this
/// point, which should hopefully help ensure that all accessors
/// see zeroes after this point.
</span><span class="attr">#[inline]
</span><span class="kw">fn </span>atomic_fence() {
    atomic::compiler_fence(atomic::Ordering::SeqCst);
}

<span class="doccomment">/// Perform a volatile write to the destination
</span><span class="attr">#[inline]
</span><span class="kw">fn </span>volatile_write&lt;T: Copy + Sized&gt;(dst: <span class="kw-2">&amp;mut </span>T, src: T) {
    <span class="kw">unsafe </span>{ ptr::write_volatile(dst, src) }
}

<span class="doccomment">/// Perform a volatile `memset` operation which fills a slice with a value
///
/// Safety:
/// The memory pointed to by `dst` must be a single allocated object that is valid for `count`
/// contiguous elements of `T`.
/// `count` must not be larger than an `isize`.
/// `dst` being offset by `mem::size_of::&lt;T&gt; * count` bytes must not wrap around the address space.
/// Also `dst` must be properly aligned.
</span><span class="attr">#[inline]
</span><span class="kw">unsafe fn </span>volatile_set&lt;T: Copy + Sized&gt;(dst: <span class="kw-2">*mut </span>T, src: T, count: usize) {
    <span class="comment">// TODO(tarcieri): use `volatile_set_memory` when stabilized
    </span><span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..count {
        <span class="comment">// Safety:
        //
        // This is safe because there is room for at least `count` objects of type `T` in the
        // allocation pointed to by `dst`, because `count &lt;= isize::MAX` and because
        // `dst.add(count)` must not wrap around the address space.
        </span><span class="kw">let </span>ptr = dst.add(i);
        <span class="comment">// Safety:
        //
        // This is safe, because the pointer is valid and because `dst` is well aligned for `T` and
        // `ptr` is an offset of `dst` by a multiple of `mem::size_of::&lt;T&gt;()` bytes.
        </span>ptr::write_volatile(ptr, src);
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;
    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">use </span>alloc::boxed::Box;

    <span class="attr">#[test]
    </span><span class="kw">fn </span>zeroize_byte_arrays() {
        <span class="kw">let </span><span class="kw-2">mut </span>arr = [<span class="number">42u8</span>; <span class="number">64</span>];
        arr.zeroize();
        <span class="macro">assert_eq!</span>(arr.as_ref(), [<span class="number">0u8</span>; <span class="number">64</span>].as_ref());
    }

    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    #[test]
    </span><span class="kw">fn </span>zeroize_vec() {
        <span class="kw">let </span><span class="kw-2">mut </span>vec = <span class="macro">vec!</span>[<span class="number">42</span>; <span class="number">3</span>];
        vec.zeroize();
        <span class="macro">assert!</span>(vec.is_empty());
    }

    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    #[test]
    </span><span class="kw">fn </span>zeroize_vec_entire_capacity() {
        <span class="attr">#[derive(Clone)]
        </span><span class="kw">struct </span>PanicOnNonZeroDrop(u64);

        <span class="kw">impl </span>Zeroize <span class="kw">for </span>PanicOnNonZeroDrop {
            <span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
                <span class="self">self</span>.<span class="number">0 </span>= <span class="number">0</span>;
            }
        }

        <span class="kw">impl </span>Drop <span class="kw">for </span>PanicOnNonZeroDrop {
            <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
                <span class="kw">if </span><span class="self">self</span>.<span class="number">0 </span>!= <span class="number">0 </span>{
                    <span class="macro">panic!</span>(<span class="string">"dropped non-zeroized data"</span>);
                }
            }
        }

        <span class="comment">// Ensure that the entire capacity of the vec is zeroized and that no unitinialized data
        // is ever interpreted as initialized
        </span><span class="kw">let </span><span class="kw-2">mut </span>vec = <span class="macro">vec!</span>[PanicOnNonZeroDrop(<span class="number">42</span>); <span class="number">2</span>];

        <span class="kw">unsafe </span>{
            vec.set_len(<span class="number">1</span>);
        }

        vec.zeroize();

        <span class="kw">unsafe </span>{
            vec.set_len(<span class="number">2</span>);
        }

        drop(vec);
    }

    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    #[test]
    </span><span class="kw">fn </span>zeroize_string() {
        <span class="kw">let </span><span class="kw-2">mut </span>string = String::from(<span class="string">"Hello, world!"</span>);
        string.zeroize();
        <span class="macro">assert!</span>(string.is_empty());
    }

    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    #[test]
    </span><span class="kw">fn </span>zeroize_string_entire_capacity() {
        <span class="kw">let </span><span class="kw-2">mut </span>string = String::from(<span class="string">"Hello, world!"</span>);
        string.truncate(<span class="number">5</span>);

        string.zeroize();

        <span class="comment">// convert the string to a vec to easily access the unused capacity
        </span><span class="kw">let </span><span class="kw-2">mut </span>as_vec = string.into_bytes();
        <span class="kw">unsafe </span>{ as_vec.set_len(as_vec.capacity()) };

        <span class="macro">assert!</span>(as_vec.iter().all(|byte| <span class="kw-2">*</span>byte == <span class="number">0</span>));
    }

    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    #[test]
    </span><span class="kw">fn </span>zeroize_box() {
        <span class="kw">let </span><span class="kw-2">mut </span>boxed_arr = Box::new([<span class="number">42u8</span>; <span class="number">3</span>]);
        boxed_arr.zeroize();
        <span class="macro">assert_eq!</span>(boxed_arr.as_ref(), <span class="kw-2">&amp;</span>[<span class="number">0u8</span>; <span class="number">3</span>]);
    }
}
</code></pre></div></section></main></body></html>