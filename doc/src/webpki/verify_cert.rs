<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rustls-webpki-0.101.7/src/verify_cert.rs`."><title>verify_cert.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="webpki" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../webpki/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2015 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

</span><span class="kw">use </span>core::default::Default;
<span class="kw">use </span>core::ops::ControlFlow;

<span class="kw">use crate</span>::{
    cert::{Cert, EndEntityOrCa},
    der, public_values_eq, signed_data, subject_name, time, CertRevocationList, Error,
    SignatureAlgorithm, TrustAnchor,
};

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>ChainOptions&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">pub</span>(<span class="kw">crate</span>) eku: KeyUsage,
    <span class="kw">pub</span>(<span class="kw">crate</span>) supported_sig_algs: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[<span class="kw-2">&amp;</span><span class="lifetime">'a </span>SignatureAlgorithm],
    <span class="kw">pub</span>(<span class="kw">crate</span>) trust_anchors: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[TrustAnchor&lt;<span class="lifetime">'a</span>&gt;],
    <span class="kw">pub</span>(<span class="kw">crate</span>) intermediate_certs: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[<span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8]],
    <span class="kw">pub</span>(<span class="kw">crate</span>) crls: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw">dyn </span>CertRevocationList],
}

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>build_chain(opts: <span class="kw-2">&amp;</span>ChainOptions, cert: <span class="kw-2">&amp;</span>Cert, time: time::Time) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
    build_chain_inner(opts, cert, time, <span class="number">0</span>, <span class="kw-2">&amp;mut </span>Budget::default()).map_err(|e| <span class="kw">match </span>e {
        ControlFlow::Break(err) =&gt; err,
        ControlFlow::Continue(err) =&gt; err,
    })
}

<span class="kw">fn </span>build_chain_inner(
    opts: <span class="kw-2">&amp;</span>ChainOptions,
    cert: <span class="kw-2">&amp;</span>Cert,
    time: time::Time,
    sub_ca_count: usize,
    budget: <span class="kw-2">&amp;mut </span>Budget,
) -&gt; <span class="prelude-ty">Result</span>&lt;(), ControlFlow&lt;Error, Error&gt;&gt; {
    <span class="kw">let </span>used_as_ca = used_as_ca(<span class="kw-2">&amp;</span>cert.ee_or_ca);

    check_issuer_independent_properties(cert, time, used_as_ca, sub_ca_count, opts.eku.inner)<span class="question-mark">?</span>;

    <span class="comment">// TODO: HPKP checks.

    </span><span class="kw">match </span>used_as_ca {
        UsedAsCa::Yes =&gt; {
            <span class="kw">const </span>MAX_SUB_CA_COUNT: usize = <span class="number">6</span>;

            <span class="kw">if </span>sub_ca_count &gt;= MAX_SUB_CA_COUNT {
                <span class="kw">return </span><span class="prelude-val">Err</span>(Error::MaximumPathDepthExceeded.into());
            }
        }
        UsedAsCa::No =&gt; {
            <span class="macro">assert_eq!</span>(<span class="number">0</span>, sub_ca_count);
        }
    }

    <span class="kw">let </span>result = loop_while_non_fatal_error(
        Error::UnknownIssuer,
        opts.trust_anchors,
        |trust_anchor: <span class="kw-2">&amp;</span>TrustAnchor| {
            <span class="kw">let </span>trust_anchor_subject = untrusted::Input::from(trust_anchor.subject);
            <span class="kw">if </span>!public_values_eq(cert.issuer, trust_anchor_subject) {
                <span class="kw">return </span><span class="prelude-val">Err</span>(Error::UnknownIssuer.into());
            }

            <span class="comment">// TODO: check_distrust(trust_anchor_subject, trust_anchor_spki)?;

            </span>check_signed_chain(
                opts.supported_sig_algs,
                cert,
                trust_anchor,
                opts.crls,
                budget,
            )<span class="question-mark">?</span>;

            check_signed_chain_name_constraints(cert, trust_anchor, budget)<span class="question-mark">?</span>;

            <span class="prelude-val">Ok</span>(())
        },
    );

    <span class="kw">let </span>err = <span class="kw">match </span>result {
        <span class="prelude-val">Ok</span>(()) =&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(()),
        <span class="comment">// Fatal errors should halt further path building.
        </span>res @ <span class="prelude-val">Err</span>(ControlFlow::Break(<span class="kw">_</span>)) =&gt; <span class="kw">return </span>res,
        <span class="comment">// Non-fatal errors should be carried forward as the default_error for subsequent
        // loop_while_non_fatal_error processing and only returned once all other path-building
        // options have been exhausted.
        </span><span class="prelude-val">Err</span>(ControlFlow::Continue(err)) =&gt; err,
    };

    loop_while_non_fatal_error(err, opts.intermediate_certs, |cert_der| {
        <span class="kw">let </span>potential_issuer =
            Cert::from_der(untrusted::Input::from(cert_der), EndEntityOrCa::Ca(cert))<span class="question-mark">?</span>;

        <span class="kw">if </span>!public_values_eq(potential_issuer.subject, cert.issuer) {
            <span class="kw">return </span><span class="prelude-val">Err</span>(Error::UnknownIssuer.into());
        }

        <span class="comment">// Prevent loops; see RFC 4158 section 5.2.
        </span><span class="kw">let </span><span class="kw-2">mut </span>prev = cert;
        <span class="kw">loop </span>{
            <span class="kw">if </span>public_values_eq(potential_issuer.spki.value(), prev.spki.value())
                &amp;&amp; public_values_eq(potential_issuer.subject, prev.subject)
            {
                <span class="kw">return </span><span class="prelude-val">Err</span>(Error::UnknownIssuer.into());
            }
            <span class="kw">match </span><span class="kw-2">&amp;</span>prev.ee_or_ca {
                EndEntityOrCa::EndEntity =&gt; {
                    <span class="kw">break</span>;
                }
                EndEntityOrCa::Ca(child_cert) =&gt; {
                    prev = child_cert;
                }
            }
        }

        <span class="kw">let </span>next_sub_ca_count = <span class="kw">match </span>used_as_ca {
            UsedAsCa::No =&gt; sub_ca_count,
            UsedAsCa::Yes =&gt; sub_ca_count + <span class="number">1</span>,
        };

        budget.consume_build_chain_call()<span class="question-mark">?</span>;
        build_chain_inner(opts, <span class="kw-2">&amp;</span>potential_issuer, time, next_sub_ca_count, budget)
    })
}

<span class="kw">fn </span>check_signed_chain(
    supported_sig_algs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>SignatureAlgorithm],
    cert_chain: <span class="kw-2">&amp;</span>Cert,
    trust_anchor: <span class="kw-2">&amp;</span>TrustAnchor,
    crls: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span><span class="kw">dyn </span>CertRevocationList],
    budget: <span class="kw-2">&amp;mut </span>Budget,
) -&gt; <span class="prelude-ty">Result</span>&lt;(), ControlFlow&lt;Error, Error&gt;&gt; {
    <span class="kw">let </span><span class="kw-2">mut </span>spki_value = untrusted::Input::from(trust_anchor.spki);
    <span class="kw">let </span><span class="kw-2">mut </span>issuer_subject = untrusted::Input::from(trust_anchor.subject);
    <span class="kw">let </span><span class="kw-2">mut </span>issuer_key_usage = <span class="prelude-val">None</span>; <span class="comment">// TODO(XXX): Consider whether to track TrustAnchor KU.
    </span><span class="kw">let </span><span class="kw-2">mut </span>cert = cert_chain;
    <span class="kw">loop </span>{
        signed_data::verify_signed_data(supported_sig_algs, spki_value, <span class="kw-2">&amp;</span>cert.signed_data, budget)<span class="question-mark">?</span>;

        <span class="kw">if </span>!crls.is_empty() {
            check_crls(
                supported_sig_algs,
                cert,
                issuer_subject,
                spki_value,
                issuer_key_usage,
                crls,
                budget,
            )<span class="question-mark">?</span>;
        }

        <span class="kw">match </span><span class="kw-2">&amp;</span>cert.ee_or_ca {
            EndEntityOrCa::Ca(child_cert) =&gt; {
                spki_value = cert.spki.value();
                issuer_subject = cert.subject;
                issuer_key_usage = cert.key_usage;
                cert = child_cert;
            }
            EndEntityOrCa::EndEntity =&gt; {
                <span class="kw">break</span>;
            }
        }
    }

    <span class="prelude-val">Ok</span>(())
}

<span class="kw">fn </span>check_signed_chain_name_constraints(
    cert_chain: <span class="kw-2">&amp;</span>Cert,
    trust_anchor: <span class="kw-2">&amp;</span>TrustAnchor,
    budget: <span class="kw-2">&amp;mut </span>Budget,
) -&gt; <span class="prelude-ty">Result</span>&lt;(), ControlFlow&lt;Error, Error&gt;&gt; {
    <span class="kw">let </span><span class="kw-2">mut </span>cert = cert_chain;
    <span class="kw">let </span><span class="kw-2">mut </span>name_constraints = trust_anchor
        .name_constraints
        .as_ref()
        .map(|der| untrusted::Input::from(der));

    <span class="kw">loop </span>{
        untrusted::read_all_optional(name_constraints, Error::BadDer, |value| {
            subject_name::check_name_constraints(value, cert, budget)
        })<span class="question-mark">?</span>;

        <span class="kw">match </span><span class="kw-2">&amp;</span>cert.ee_or_ca {
            EndEntityOrCa::Ca(child_cert) =&gt; {
                name_constraints = cert.name_constraints;
                cert = child_cert;
            }
            EndEntityOrCa::EndEntity =&gt; {
                <span class="kw">break</span>;
            }
        }
    }

    <span class="prelude-val">Ok</span>(())
}

<span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Budget {
    signatures: usize,
    build_chain_calls: usize,
    name_constraint_comparisons: usize,
}

<span class="kw">impl </span>Budget {
    <span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>consume_signature(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="self">self</span>.signatures = <span class="self">self
            </span>.signatures
            .checked_sub(<span class="number">1</span>)
            .ok_or(Error::MaximumSignatureChecksExceeded)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>consume_build_chain_call(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="self">self</span>.build_chain_calls = <span class="self">self
            </span>.build_chain_calls
            .checked_sub(<span class="number">1</span>)
            .ok_or(Error::MaximumPathBuildCallsExceeded)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }

    <span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>consume_name_constraint_comparison(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="self">self</span>.name_constraint_comparisons = <span class="self">self
            </span>.name_constraint_comparisons
            .checked_sub(<span class="number">1</span>)
            .ok_or(Error::MaximumNameConstraintComparisonsExceeded)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="kw">impl </span>Default <span class="kw">for </span>Budget {
    <span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            <span class="comment">// This limit is taken from the remediation for golang CVE-2018-16875.  However,
            // note that golang subsequently implemented AKID matching due to this limit
            // being hit in real applications (see &lt;https://github.com/spiffe/spire/issues/1004&gt;).
            // So this may actually be too aggressive.
            </span>signatures: <span class="number">100</span>,

            <span class="comment">// This limit is taken from NSS libmozpkix, see:
            // &lt;https://github.com/nss-dev/nss/blob/bb4a1d38dd9e92923525ac6b5ed0288479f3f3fc/lib/mozpkix/lib/pkixbuild.cpp#L381-L393&gt;
            </span>build_chain_calls: <span class="number">200_000</span>,

            <span class="comment">// This limit is taken from golang crypto/x509's default, see:
            // &lt;https://github.com/golang/go/blob/ac17bb6f13979f2ab9fcd45f0758b43ed72d0973/src/crypto/x509/verify.go#L588-L592&gt;
            </span>name_constraint_comparisons: <span class="number">250_000</span>,
        }
    }
}

<span class="comment">// Zero-sized marker type representing positive assertion that revocation status was checked
// for a certificate and the result was that the certificate is not revoked.
</span><span class="kw">struct </span>CertNotRevoked(());

<span class="kw">impl </span>CertNotRevoked {
    <span class="comment">// Construct a CertNotRevoked marker.
    </span><span class="kw">fn </span>assertion() -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>(())
    }
}

<span class="kw">fn </span>check_crls(
    supported_sig_algs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>SignatureAlgorithm],
    cert: <span class="kw-2">&amp;</span>Cert,
    issuer_subject: untrusted::Input,
    issuer_spki: untrusted::Input,
    issuer_ku: <span class="prelude-ty">Option</span>&lt;untrusted::Input&gt;,
    crls: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span><span class="kw">dyn </span>CertRevocationList],
    budget: <span class="kw-2">&amp;mut </span>Budget,
) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="prelude-ty">Option</span>&lt;CertNotRevoked&gt;, Error&gt; {
    <span class="macro">assert!</span>(public_values_eq(cert.issuer, issuer_subject));

    <span class="kw">let </span>crl = <span class="kw">match </span>crls
        .iter()
        .find(|candidate_crl| candidate_crl.issuer() == cert.issuer())
    {
        <span class="prelude-val">Some</span>(crl) =&gt; crl,
        <span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="prelude-val">None</span>),
    };

    <span class="comment">// Verify the CRL signature with the issuer SPKI.
    // TODO(XXX): consider whether we can refactor so this happens once up-front, instead
    //            of per-lookup.
    //            https://github.com/rustls/webpki/issues/81
    // Note: The `verify_signature` method is part of a public trait in the exported API.
    //       We can't add a budget argument to that fn in a semver compatible way and so must
    //       consume signature budget here before calling verify_signature.
    </span>budget.consume_signature()<span class="question-mark">?</span>;
    crl.verify_signature(supported_sig_algs, issuer_spki.as_slice_less_safe())
        .map_err(crl_signature_err)<span class="question-mark">?</span>;

    <span class="comment">// Verify that if the issuer has a KeyUsage bitstring it asserts cRLSign.
    </span>KeyUsageMode::CrlSign.check(issuer_ku)<span class="question-mark">?</span>;

    <span class="comment">// Try to find the cert serial in the verified CRL contents.
    </span><span class="kw">let </span>cert_serial = cert.serial.as_slice_less_safe();
    <span class="kw">match </span>crl.find_serial(cert_serial)<span class="question-mark">? </span>{
        <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Ok</span>(<span class="prelude-val">Some</span>(CertNotRevoked::assertion())),
        <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="prelude-val">Err</span>(Error::CertRevoked),
    }
}

<span class="comment">// When verifying CRL signed data we want to disambiguate the context of possible errors by mapping
// them to CRL specific variants that a consumer can use to tell the issue was with the CRL's
// signature, not a certificate.
</span><span class="kw">fn </span>crl_signature_err(err: Error) -&gt; Error {
    <span class="kw">match </span>err {
        Error::UnsupportedSignatureAlgorithm =&gt; Error::UnsupportedCrlSignatureAlgorithm,
        Error::UnsupportedSignatureAlgorithmForPublicKey =&gt; {
            Error::UnsupportedCrlSignatureAlgorithmForPublicKey
        }
        Error::InvalidSignatureForPublicKey =&gt; Error::InvalidCrlSignatureForPublicKey,
        <span class="kw">_ </span>=&gt; err,
    }
}

<span class="kw">fn </span>check_issuer_independent_properties(
    cert: <span class="kw-2">&amp;</span>Cert,
    time: time::Time,
    used_as_ca: UsedAsCa,
    sub_ca_count: usize,
    eku: ExtendedKeyUsage,
) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
    <span class="comment">// TODO: check_distrust(trust_anchor_subject, trust_anchor_spki)?;
    // TODO: Check signature algorithm like mozilla::pkix.
    // TODO: Check SPKI like mozilla::pkix.
    // TODO: check for active distrust like mozilla::pkix.

    // For cert validation, we ignore the KeyUsage extension. For CA
    // certificates, BasicConstraints.cA makes KeyUsage redundant. Firefox
    // and other common browsers do not check KeyUsage for end-entities,
    // though it would be kind of nice to ensure that a KeyUsage without
    // the keyEncipherment bit could not be used for RSA key exchange.

    </span>cert.validity
        .read_all(Error::BadDer, |value| check_validity(value, time))<span class="question-mark">?</span>;
    untrusted::read_all_optional(cert.basic_constraints, Error::BadDer, |value| {
        check_basic_constraints(value, used_as_ca, sub_ca_count)
    })<span class="question-mark">?</span>;
    untrusted::read_all_optional(cert.eku, Error::BadDer, |value| eku.check(value))<span class="question-mark">?</span>;

    <span class="prelude-val">Ok</span>(())
}

<span class="comment">// https://tools.ietf.org/html/rfc5280#section-4.1.2.5
</span><span class="kw">fn </span>check_validity(input: <span class="kw-2">&amp;mut </span>untrusted::Reader, time: time::Time) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
    <span class="kw">let </span>not_before = der::time_choice(input)<span class="question-mark">?</span>;
    <span class="kw">let </span>not_after = der::time_choice(input)<span class="question-mark">?</span>;

    <span class="kw">if </span>not_before &gt; not_after {
        <span class="kw">return </span><span class="prelude-val">Err</span>(Error::InvalidCertValidity);
    }
    <span class="kw">if </span>time &lt; not_before {
        <span class="kw">return </span><span class="prelude-val">Err</span>(Error::CertNotValidYet);
    }
    <span class="kw">if </span>time &gt; not_after {
        <span class="kw">return </span><span class="prelude-val">Err</span>(Error::CertExpired);
    }

    <span class="comment">// TODO: mozilla::pkix allows the TrustDomain to check not_before and
    // not_after, to enforce things like a maximum validity period. We should
    // do something similar.

    </span><span class="prelude-val">Ok</span>(())
}

<span class="attr">#[derive(Clone, Copy, PartialEq)]
</span><span class="kw">enum </span>UsedAsCa {
    Yes,
    No,
}

<span class="kw">fn </span>used_as_ca(ee_or_ca: <span class="kw-2">&amp;</span>EndEntityOrCa) -&gt; UsedAsCa {
    <span class="kw">match </span>ee_or_ca {
        EndEntityOrCa::EndEntity =&gt; UsedAsCa::No,
        EndEntityOrCa::Ca(..) =&gt; UsedAsCa::Yes,
    }
}

<span class="comment">// https://tools.ietf.org/html/rfc5280#section-4.2.1.9
</span><span class="kw">fn </span>check_basic_constraints(
    input: <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;mut </span>untrusted::Reader&gt;,
    used_as_ca: UsedAsCa,
    sub_ca_count: usize,
) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
    <span class="kw">let </span>(is_ca, path_len_constraint) = <span class="kw">match </span>input {
        <span class="prelude-val">Some</span>(input) =&gt; {
            <span class="kw">let </span>is_ca = der::optional_boolean(input)<span class="question-mark">?</span>;

            <span class="comment">// https://bugzilla.mozilla.org/show_bug.cgi?id=985025: RFC 5280
            // says that a certificate must not have pathLenConstraint unless
            // it is a CA certificate, but some real-world end-entity
            // certificates have pathLenConstraint.
            </span><span class="kw">let </span>path_len_constraint = <span class="kw">if </span>!input.at_end() {
                <span class="kw">let </span>value = der::small_nonnegative_integer(input)<span class="question-mark">?</span>;
                <span class="prelude-val">Some</span>(usize::from(value))
            } <span class="kw">else </span>{
                <span class="prelude-val">None
            </span>};

            (is_ca, path_len_constraint)
        }
        <span class="prelude-val">None </span>=&gt; (<span class="bool-val">false</span>, <span class="prelude-val">None</span>),
    };

    <span class="kw">match </span>(used_as_ca, is_ca, path_len_constraint) {
        (UsedAsCa::No, <span class="bool-val">true</span>, <span class="kw">_</span>) =&gt; <span class="prelude-val">Err</span>(Error::CaUsedAsEndEntity),
        (UsedAsCa::Yes, <span class="bool-val">false</span>, <span class="kw">_</span>) =&gt; <span class="prelude-val">Err</span>(Error::EndEntityUsedAsCa),
        (UsedAsCa::Yes, <span class="bool-val">true</span>, <span class="prelude-val">Some</span>(len)) <span class="kw">if </span>sub_ca_count &gt; len =&gt; {
            <span class="prelude-val">Err</span>(Error::PathLenConstraintViolated)
        }
        <span class="kw">_ </span>=&gt; <span class="prelude-val">Ok</span>(()),
    }
}

<span class="doccomment">/// The expected key usage of a certificate.
///
/// This type represents the expected key usage of an end entity certificate. Although for most
/// kinds of certificates the extended key usage extension is optional (and so certificates
/// not carrying a particular value in the EKU extension are acceptable). If the extension
/// is present, the certificate MUST only be used for one of the purposes indicated.
///
/// &lt;https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.12&gt;
</span><span class="attr">#[derive(Clone, Copy)]
</span><span class="kw">pub struct </span>KeyUsage {
    inner: ExtendedKeyUsage,
}

<span class="kw">impl </span>KeyUsage {
    <span class="doccomment">/// Construct a new [`KeyUsage`] as appropriate for server certificate authentication.
    ///
    /// As specified in &lt;https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.12&gt;, this does not require the certificate to specify the eKU extension.
    </span><span class="kw">pub const fn </span>server_auth() -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            inner: ExtendedKeyUsage::RequiredIfPresent(EKU_SERVER_AUTH),
        }
    }

    <span class="doccomment">/// Construct a new [`KeyUsage`] as appropriate for client certificate authentication.
    ///
    /// As specified in &lt;&gt;, this does not require the certificate to specify the eKU extension.
    </span><span class="kw">pub const fn </span>client_auth() -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            inner: ExtendedKeyUsage::RequiredIfPresent(EKU_CLIENT_AUTH),
        }
    }

    <span class="doccomment">/// Construct a new [`KeyUsage`] requiring a certificate to support the specified OID.
    </span><span class="kw">pub const fn </span>required(oid: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[u8]) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            inner: ExtendedKeyUsage::Required(KeyPurposeId::new(oid)),
        }
    }
}

<span class="doccomment">/// Extended Key Usage (EKU) of a certificate.
</span><span class="attr">#[derive(Clone, Copy)]
</span><span class="kw">enum </span>ExtendedKeyUsage {
    <span class="doccomment">/// The certificate must contain the specified [`KeyPurposeId`] as EKU.
    </span>Required(KeyPurposeId),

    <span class="doccomment">/// If the certificate has EKUs, then the specified [`KeyPurposeId`] must be included.
    </span>RequiredIfPresent(KeyPurposeId),
}

<span class="kw">impl </span>ExtendedKeyUsage {
    <span class="comment">// https://tools.ietf.org/html/rfc5280#section-4.2.1.12
    </span><span class="kw">fn </span>check(<span class="kw-2">&amp;</span><span class="self">self</span>, input: <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;mut </span>untrusted::Reader&gt;) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="kw">let </span>input = <span class="kw">match </span>(input, <span class="self">self</span>) {
            (<span class="prelude-val">Some</span>(input), <span class="kw">_</span>) =&gt; input,
            (<span class="prelude-val">None</span>, <span class="self">Self</span>::RequiredIfPresent(<span class="kw">_</span>)) =&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(()),
            (<span class="prelude-val">None</span>, <span class="self">Self</span>::Required(<span class="kw">_</span>)) =&gt; <span class="kw">return </span><span class="prelude-val">Err</span>(Error::RequiredEkuNotFound),
        };

        <span class="kw">loop </span>{
            <span class="kw">let </span>value = der::expect_tag_and_get_value(input, der::Tag::OID)<span class="question-mark">?</span>;
            <span class="kw">if </span><span class="self">self</span>.key_purpose_id_equals(value) {
                input.skip_to_end();
                <span class="kw">break</span>;
            }

            <span class="kw">if </span>input.at_end() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(Error::RequiredEkuNotFound);
            }
        }

        <span class="prelude-val">Ok</span>(())
    }

    <span class="kw">fn </span>key_purpose_id_equals(<span class="kw-2">&amp;</span><span class="self">self</span>, value: untrusted::Input&lt;<span class="lifetime">'_</span>&gt;) -&gt; bool {
        public_values_eq(
            <span class="kw">match </span><span class="self">self </span>{
                ExtendedKeyUsage::Required(eku) =&gt; <span class="kw-2">*</span>eku,
                ExtendedKeyUsage::RequiredIfPresent(eku) =&gt; <span class="kw-2">*</span>eku,
            }
            .oid_value,
            value,
        )
    }
}

<span class="doccomment">/// An OID value indicating an Extended Key Usage (EKU) key purpose.
</span><span class="attr">#[derive(Clone, Copy)]
</span><span class="kw">struct </span>KeyPurposeId {
    oid_value: untrusted::Input&lt;<span class="lifetime">'static</span>&gt;,
}

<span class="kw">impl </span>KeyPurposeId {
    <span class="doccomment">/// Construct a new [`KeyPurposeId`].
    ///
    /// `oid` is the OBJECT IDENTIFIER in bytes.
    </span><span class="kw">const fn </span>new(oid: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[u8]) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            oid_value: untrusted::Input::from(oid),
        }
    }
}

<span class="kw">impl </span>PartialEq&lt;<span class="self">Self</span>&gt; <span class="kw">for </span>KeyPurposeId {
    <span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; bool {
        public_values_eq(<span class="self">self</span>.oid_value, other.oid_value)
    }
}

<span class="kw">impl </span>Eq <span class="kw">for </span>KeyPurposeId {}

<span class="comment">// id-pkix            OBJECT IDENTIFIER ::= { 1 3 6 1 5 5 7 }
// id-kp              OBJECT IDENTIFIER ::= { id-pkix 3 }

// id-kp-serverAuth   OBJECT IDENTIFIER ::= { id-kp 1 }
</span><span class="attr">#[allow(clippy::identity_op)] </span><span class="comment">// TODO: Make this clearer
</span><span class="kw">const </span>EKU_SERVER_AUTH: KeyPurposeId = KeyPurposeId::new(<span class="kw-2">&amp;</span>[(<span class="number">40 </span>* <span class="number">1</span>) + <span class="number">3</span>, <span class="number">6</span>, <span class="number">1</span>, <span class="number">5</span>, <span class="number">5</span>, <span class="number">7</span>, <span class="number">3</span>, <span class="number">1</span>]);

<span class="comment">// id-kp-clientAuth   OBJECT IDENTIFIER ::= { id-kp 2 }
</span><span class="attr">#[allow(clippy::identity_op)] </span><span class="comment">// TODO: Make this clearer
</span><span class="kw">const </span>EKU_CLIENT_AUTH: KeyPurposeId = KeyPurposeId::new(<span class="kw-2">&amp;</span>[(<span class="number">40 </span>* <span class="number">1</span>) + <span class="number">3</span>, <span class="number">6</span>, <span class="number">1</span>, <span class="number">5</span>, <span class="number">5</span>, <span class="number">7</span>, <span class="number">3</span>, <span class="number">2</span>]);

<span class="comment">// https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.3
</span><span class="attr">#[repr(u8)]
#[derive(Clone, Copy)]
</span><span class="kw">enum </span>KeyUsageMode {
    <span class="comment">// DigitalSignature = 0,
    // ContentCommitment = 1,
    // KeyEncipherment = 2,
    // DataEncipherment = 3,
    // KeyAgreement = 4,
    // CertSign = 5,
    </span>CrlSign = <span class="number">6</span>,
    <span class="comment">// EncipherOnly = 7,
    // DecipherOnly = 8,
</span>}

<span class="kw">impl </span>KeyUsageMode {
    <span class="comment">// https://www.rfc-editor.org/rfc/rfc5280#section-4.2.1.3
    </span><span class="kw">fn </span>check(<span class="self">self</span>, input: <span class="prelude-ty">Option</span>&lt;untrusted::Input&gt;) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="kw">let </span>bit_string = <span class="kw">match </span>input {
            <span class="prelude-val">Some</span>(input) =&gt; input,
            <span class="comment">// While RFC 5280 requires KeyUsage be present, historically the absence of a KeyUsage
            // has been treated as "Any Usage". We follow that convention here and assume the absence
            // of KeyUsage implies the required_ku_bit_if_present we're checking for.
            </span><span class="prelude-val">None </span>=&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(()),
        };

        <span class="kw">let </span>flags = der::bit_string_flags(<span class="kw-2">&amp;mut </span>untrusted::Reader::new(bit_string))<span class="question-mark">?</span>;
        <span class="attr">#[allow(clippy::as_conversions)] </span><span class="comment">// u8 always fits in usize.
        </span><span class="kw">match </span>flags.bit_set(<span class="self">self </span><span class="kw">as </span>usize) {
            <span class="bool-val">true </span>=&gt; <span class="prelude-val">Ok</span>(()),
            <span class="bool-val">false </span>=&gt; <span class="prelude-val">Err</span>(Error::IssuerNotCrlSigner),
        }
    }
}

<span class="kw">fn </span>loop_while_non_fatal_error&lt;V&gt;(
    default_error: Error,
    values: V,
    <span class="kw-2">mut </span>f: <span class="kw">impl </span>FnMut(V::Item) -&gt; <span class="prelude-ty">Result</span>&lt;(), ControlFlow&lt;Error, Error&gt;&gt;,
) -&gt; <span class="prelude-ty">Result</span>&lt;(), ControlFlow&lt;Error, Error&gt;&gt;
<span class="kw">where
    </span>V: IntoIterator,
{
    <span class="kw">let </span><span class="kw-2">mut </span>error = default_error;
    <span class="kw">for </span>v <span class="kw">in </span>values {
        <span class="kw">match </span>f(v) {
            <span class="prelude-val">Ok</span>(()) =&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(()),
            <span class="comment">// Fatal errors should halt further looping.
            </span>res @ <span class="prelude-val">Err</span>(ControlFlow::Break(<span class="kw">_</span>)) =&gt; <span class="kw">return </span>res,
            <span class="comment">// Non-fatal errors should be ranked by specificity and only returned
            // once all other path-building options have been exhausted.
            </span><span class="prelude-val">Err</span>(ControlFlow::Continue(new_error)) =&gt; error = error.most_specific(new_error),
        }
    }
    <span class="prelude-val">Err</span>(error.into())
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">use </span><span class="kw">crate</span>::test_utils::{make_end_entity, make_issuer};

    <span class="attr">#[test]
    </span><span class="kw">fn </span>eku_key_purpose_id() {
        <span class="macro">assert!</span>(ExtendedKeyUsage::RequiredIfPresent(EKU_SERVER_AUTH)
            .key_purpose_id_equals(EKU_SERVER_AUTH.oid_value))
    }

    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">enum </span>TrustAnchorIsActualIssuer {
        Yes,
        No,
    }

    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>build_degenerate_chain(
        intermediate_count: usize,
        trust_anchor_is_actual_issuer: TrustAnchorIsActualIssuer,
        budget: <span class="prelude-ty">Option</span>&lt;Budget&gt;,
    ) -&gt; ControlFlow&lt;Error, Error&gt; {
        <span class="kw">let </span>ca_cert = make_issuer(<span class="string">"Bogus Subject"</span>, <span class="prelude-val">None</span>);
        <span class="kw">let </span>ca_cert_der = ca_cert.serialize_der().unwrap();

        <span class="kw">let </span><span class="kw-2">mut </span>intermediates = Vec::with_capacity(intermediate_count);
        <span class="kw">let </span><span class="kw-2">mut </span>issuer = ca_cert;
        <span class="kw">for _ in </span><span class="number">0</span>..intermediate_count {
            <span class="kw">let </span>intermediate = make_issuer(<span class="string">"Bogus Subject"</span>, <span class="prelude-val">None</span>);
            <span class="kw">let </span>intermediate_der = intermediate.serialize_der_with_signer(<span class="kw-2">&amp;</span>issuer).unwrap();
            intermediates.push(intermediate_der);
            issuer = intermediate;
        }

        <span class="kw">if let </span>TrustAnchorIsActualIssuer::No = trust_anchor_is_actual_issuer {
            intermediates.pop();
        }

        verify_chain(
            <span class="kw-2">&amp;</span>ca_cert_der,
            <span class="kw-2">&amp;</span>intermediates,
            <span class="kw-2">&amp;</span>make_end_entity(<span class="kw-2">&amp;</span>issuer),
            budget,
        )
        .unwrap_err()
    }

    <span class="attr">#[test]
    #[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>test_too_many_signatures() {
        <span class="macro">assert!</span>(<span class="macro">matches!</span>(
            build_degenerate_chain(<span class="number">5</span>, TrustAnchorIsActualIssuer::Yes, <span class="prelude-val">None</span>),
            ControlFlow::Break(Error::MaximumSignatureChecksExceeded)
        ));
    }

    <span class="attr">#[test]
    #[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>test_too_many_path_calls() {
        <span class="macro">assert!</span>(<span class="macro">matches!</span>(
            build_degenerate_chain(
                <span class="number">10</span>,
                TrustAnchorIsActualIssuer::No,
                <span class="prelude-val">Some</span>(Budget {
                    <span class="comment">// Crafting a chain that will expend the build chain calls budget without
                    // first expending the signature checks budget is tricky, so we artificially
                    // inflate the signature limit to make this test easier to write.
                    </span>signatures: usize::MAX,
                    ..Budget::default()
                })
            ),
            ControlFlow::Break(Error::MaximumPathBuildCallsExceeded)
        ));
    }

    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>build_linear_chain(chain_length: usize) -&gt; <span class="prelude-ty">Result</span>&lt;(), ControlFlow&lt;Error, Error&gt;&gt; {
        <span class="kw">let </span>ca_cert = make_issuer(<span class="macro">format!</span>(<span class="string">"Bogus Subject {chain_length}"</span>), <span class="prelude-val">None</span>);
        <span class="kw">let </span>ca_cert_der = ca_cert.serialize_der().unwrap();

        <span class="kw">let </span><span class="kw-2">mut </span>intermediates = Vec::with_capacity(chain_length);
        <span class="kw">let </span><span class="kw-2">mut </span>issuer = ca_cert;
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..chain_length {
            <span class="kw">let </span>intermediate = make_issuer(<span class="macro">format!</span>(<span class="string">"Bogus Subject {i}"</span>), <span class="prelude-val">None</span>);
            <span class="kw">let </span>intermediate_der = intermediate.serialize_der_with_signer(<span class="kw-2">&amp;</span>issuer).unwrap();
            intermediates.push(intermediate_der);
            issuer = intermediate;
        }

        verify_chain(
            <span class="kw-2">&amp;</span>ca_cert_der,
            <span class="kw-2">&amp;</span>intermediates,
            <span class="kw-2">&amp;</span>make_end_entity(<span class="kw-2">&amp;</span>issuer),
            <span class="prelude-val">None</span>,
        )
    }

    <span class="attr">#[test]
    #[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>longest_allowed_path() {
        <span class="macro">assert!</span>(build_linear_chain(<span class="number">1</span>).is_ok());
        <span class="macro">assert!</span>(build_linear_chain(<span class="number">2</span>).is_ok());
        <span class="macro">assert!</span>(build_linear_chain(<span class="number">3</span>).is_ok());
        <span class="macro">assert!</span>(build_linear_chain(<span class="number">4</span>).is_ok());
        <span class="macro">assert!</span>(build_linear_chain(<span class="number">5</span>).is_ok());
        <span class="macro">assert!</span>(build_linear_chain(<span class="number">6</span>).is_ok());
    }

    <span class="attr">#[test]
    #[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>path_too_long() {
        <span class="macro">assert!</span>(<span class="macro">matches!</span>(
            build_linear_chain(<span class="number">7</span>),
            <span class="prelude-val">Err</span>(ControlFlow::Continue(Error::MaximumPathDepthExceeded))
        ));
    }

    <span class="attr">#[test]
    #[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>name_constraint_budget() {
        <span class="comment">// Issue a trust anchor that imposes name constraints. The constraint should match
        // the end entity certificate SAN.
        </span><span class="kw">let </span>ca_cert = make_issuer(
            <span class="string">"Constrained Root"</span>,
            <span class="prelude-val">Some</span>(rcgen::NameConstraints {
                permitted_subtrees: <span class="macro">vec!</span>[rcgen::GeneralSubtree::DnsName(<span class="string">".com"</span>.into())],
                excluded_subtrees: <span class="macro">vec!</span>[],
            }),
        );
        <span class="kw">let </span>ca_cert_der = ca_cert.serialize_der().unwrap();

        <span class="comment">// Create a series of intermediate issuers. We'll only use one in the actual built path,
        // helping demonstrate that the name constraint budget is not expended checking certificates
        // that are not part of the path we compute.
        </span><span class="kw">const </span>NUM_INTERMEDIATES: usize = <span class="number">5</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>intermediates = Vec::with_capacity(NUM_INTERMEDIATES);
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..NUM_INTERMEDIATES {
            intermediates.push(make_issuer(<span class="macro">format!</span>(<span class="string">"Intermediate {i}"</span>), <span class="prelude-val">None</span>));
        }

        <span class="comment">// Each intermediate should be issued by the trust anchor.
        </span><span class="kw">let </span><span class="kw-2">mut </span>intermediates_der = Vec::with_capacity(NUM_INTERMEDIATES);
        <span class="kw">for </span>intermediate <span class="kw">in </span><span class="kw-2">&amp;</span>intermediates {
            intermediates_der.push(intermediate.serialize_der_with_signer(<span class="kw-2">&amp;</span>ca_cert).unwrap());
        }

        <span class="comment">// Create an end-entity cert that is issued by the last of the intermediates.
        </span><span class="kw">let </span>ee_cert = make_end_entity(intermediates.last().unwrap());

        <span class="comment">// We use a custom budget to make it easier to write a test, otherwise it is tricky to
        // stuff enough names/constraints into the potential chains while staying within the path
        // depth limit and the build chain call limit.
        </span><span class="kw">let </span>passing_budget = Budget {
            <span class="comment">// One comparison against the intermediate's distinguished name.
            // One comparison against the EE's distinguished name.
            // One comparison against the EE's SAN.
            //  = 3 total comparisons.
            </span>name_constraint_comparisons: <span class="number">3</span>,
            ..Budget::default()
        };

        <span class="comment">// Validation should succeed with the name constraint comparison budget allocated above.
        // This shows that we're not consuming budget on unused intermediates: we didn't budget
        // enough comparisons for that to pass the overall chain building.
        </span><span class="macro">assert!</span>(verify_chain(
            <span class="kw-2">&amp;</span>ca_cert_der,
            <span class="kw-2">&amp;</span>intermediates_der,
            <span class="kw-2">&amp;</span>ee_cert,
            <span class="prelude-val">Some</span>(passing_budget),
        )
        .is_ok());

        <span class="kw">let </span>failing_budget = Budget {
            <span class="comment">// See passing_budget: 2 comparisons is not sufficient.
            </span>name_constraint_comparisons: <span class="number">2</span>,
            ..Budget::default()
        };
        <span class="comment">// Validation should fail when the budget is smaller than the number of comparisons performed
        // on the validated path. This demonstrates we properly fail path building when too many
        // name constraint comparisons occur.
        </span><span class="kw">let </span>result = verify_chain(
            <span class="kw-2">&amp;</span>ca_cert_der,
            <span class="kw-2">&amp;</span>intermediates_der,
            <span class="kw-2">&amp;</span>ee_cert,
            <span class="prelude-val">Some</span>(failing_budget),
        );

        <span class="macro">assert!</span>(<span class="macro">matches!</span>(
            result,
            <span class="prelude-val">Err</span>(ControlFlow::Break(
                Error::MaximumNameConstraintComparisonsExceeded
            ))
        ));
    }

    <span class="attr">#[cfg(feature = <span class="string">"alloc"</span>)]
    </span><span class="kw">fn </span>verify_chain(
        trust_anchor_der: <span class="kw-2">&amp;</span>[u8],
        intermediates_der: <span class="kw-2">&amp;</span>[Vec&lt;u8&gt;],
        ee_cert_der: <span class="kw-2">&amp;</span>[u8],
        budget: <span class="prelude-ty">Option</span>&lt;Budget&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), ControlFlow&lt;Error, Error&gt;&gt; {
        <span class="kw">use </span><span class="kw">crate</span>::ECDSA_P256_SHA256;
        <span class="kw">use crate</span>::{EndEntityCert, Time};

        <span class="kw">let </span>anchors = <span class="kw-2">&amp;</span>[TrustAnchor::try_from_cert_der(trust_anchor_der).unwrap()];
        <span class="kw">let </span>time = Time::from_seconds_since_unix_epoch(<span class="number">0x1fed_f00d</span>);
        <span class="kw">let </span>cert = EndEntityCert::try_from(ee_cert_der).unwrap();
        <span class="kw">let </span>intermediates_der = intermediates_der
            .iter()
            .map(|x| x.as_ref())
            .collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();

        build_chain_inner(
            <span class="kw-2">&amp;</span>ChainOptions {
                eku: KeyUsage::server_auth(),
                supported_sig_algs: <span class="kw-2">&amp;</span>[<span class="kw-2">&amp;</span>ECDSA_P256_SHA256],
                trust_anchors: anchors,
                intermediate_certs: <span class="kw-2">&amp;</span>intermediates_der,
                crls: <span class="kw-2">&amp;</span>[],
            },
            cert.inner(),
            time,
            <span class="number">0</span>,
            <span class="kw-2">&amp;mut </span>budget.unwrap_or_default(),
        )
    }
}
</code></pre></div></section></main></body></html>