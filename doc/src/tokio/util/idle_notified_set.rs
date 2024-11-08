<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.37.0/src/util/idle_notified_set.rs`."><title>idle_notified_set.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="tokio" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../tokio/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! This module defines an `IdleNotifiedSet`, which is a collection of elements.
//! Each element is intended to correspond to a task, and the collection will
//! keep track of which tasks have had their waker notified, and which have not.
//!
//! Each entry in the set holds some user-specified value. The value's type is
//! specified using the `T` parameter. It will usually be a `JoinHandle` or
//! similar.

</span><span class="kw">use </span>std::marker::PhantomPinned;
<span class="kw">use </span>std::mem::ManuallyDrop;
<span class="kw">use </span>std::ptr::NonNull;
<span class="kw">use </span>std::task::{Context, Waker};

<span class="kw">use </span><span class="kw">crate</span>::loom::cell::UnsafeCell;
<span class="kw">use </span><span class="kw">crate</span>::loom::sync::{Arc, Mutex};
<span class="kw">use </span><span class="kw">crate</span>::util::linked_list::{<span class="self">self</span>, Link};
<span class="kw">use </span><span class="kw">crate</span>::util::{waker_ref, Wake};

<span class="kw">type </span>LinkedList&lt;T&gt; =
    linked_list::LinkedList&lt;ListEntry&lt;T&gt;, &lt;ListEntry&lt;T&gt; <span class="kw">as </span>linked_list::Link&gt;::Target&gt;;

<span class="doccomment">/// This is the main handle to the collection.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>IdleNotifiedSet&lt;T&gt; {
    lists: Arc&lt;Lists&lt;T&gt;&gt;,
    length: usize,
}

<span class="doccomment">/// A handle to an entry that is guaranteed to be stored in the idle or notified
/// list of its `IdleNotifiedSet`. This value borrows the `IdleNotifiedSet`
/// mutably to prevent the entry from being moved to the `Neither` list, which
/// only the `IdleNotifiedSet` may do.
///
/// The main consequence of being stored in one of the lists is that the `value`
/// field has not yet been consumed.
///
/// Note: This entry can be moved from the idle to the notified list while this
/// object exists by waking its waker.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>EntryInOneOfTheLists&lt;<span class="lifetime">'a</span>, T&gt; {
    entry: Arc&lt;ListEntry&lt;T&gt;&gt;,
    set: <span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>IdleNotifiedSet&lt;T&gt;,
}

<span class="kw">type </span>Lists&lt;T&gt; = Mutex&lt;ListsInner&lt;T&gt;&gt;;

<span class="doccomment">/// The linked lists hold strong references to the `ListEntry` items, and the
/// `ListEntry` items also hold a strong reference back to the Lists object, but
/// the destructor of the `IdleNotifiedSet` will clear the two lists, so once
/// that object is destroyed, no ref-cycles will remain.
</span><span class="kw">struct </span>ListsInner&lt;T&gt; {
    notified: LinkedList&lt;T&gt;,
    idle: LinkedList&lt;T&gt;,
    <span class="doccomment">/// Whenever an element in the `notified` list is woken, this waker will be
    /// notified and consumed, if it exists.
    </span>waker: <span class="prelude-ty">Option</span>&lt;Waker&gt;,
}

<span class="doccomment">/// Which of the two lists in the shared Lists object is this entry stored in?
///
/// If the value is `Idle`, then an entry's waker may move it to the notified
/// list. Otherwise, only the `IdleNotifiedSet` may move it.
///
/// If the value is `Neither`, then it is still possible that the entry is in
/// some third external list (this happens in `drain`).
</span><span class="attr">#[derive(Copy, Clone, Eq, PartialEq)]
</span><span class="kw">enum </span>List {
    Notified,
    Idle,
    Neither,
}

<span class="doccomment">/// An entry in the list.
///
/// # Safety
///
/// The `my_list` field must only be accessed while holding the mutex in
/// `parent`. It is an invariant that the value of `my_list` corresponds to
/// which linked list in the `parent` holds this entry. Once this field takes
/// the value `Neither`, then it may never be modified again.
///
/// If the value of `my_list` is `Notified` or `Idle`, then the `pointers` field
/// must only be accessed while holding the mutex. If the value of `my_list` is
/// `Neither`, then the `pointers` field may be accessed by the
/// `IdleNotifiedSet` (this happens inside `drain`).
///
/// The `value` field is owned by the `IdleNotifiedSet` and may only be accessed
/// by the `IdleNotifiedSet`. The operation that sets the value of `my_list` to
/// `Neither` assumes ownership of the `value`, and it must either drop it or
/// move it out from this entry to prevent it from getting leaked. (Since the
/// two linked lists are emptied in the destructor of `IdleNotifiedSet`, the
/// value should not be leaked.)
</span><span class="kw">struct </span>ListEntry&lt;T&gt; {
    <span class="doccomment">/// The linked list pointers of the list this entry is in.
    </span>pointers: linked_list::Pointers&lt;ListEntry&lt;T&gt;&gt;,
    <span class="doccomment">/// Pointer to the shared `Lists` struct.
    </span>parent: Arc&lt;Lists&lt;T&gt;&gt;,
    <span class="doccomment">/// The value stored in this entry.
    </span>value: UnsafeCell&lt;ManuallyDrop&lt;T&gt;&gt;,
    <span class="doccomment">/// Used to remember which list this entry is in.
    </span>my_list: UnsafeCell&lt;List&gt;,
    <span class="doccomment">/// Required by the `linked_list::Pointers` field.
    </span>_pin: PhantomPinned,
}

<span class="macro">generate_addr_of_methods!</span> {
    <span class="kw">impl</span>&lt;T&gt; ListEntry&lt;T&gt; {
        <span class="kw">unsafe fn </span>addr_of_pointers(<span class="self">self</span>: NonNull&lt;<span class="self">Self</span>&gt;) -&gt; NonNull&lt;linked_list::Pointers&lt;ListEntry&lt;T&gt;&gt;&gt; {
            <span class="kw-2">&amp;</span><span class="self">self</span>.pointers
        }
    }
}

<span class="comment">// With mutable access to the `IdleNotifiedSet`, you can get mutable access to
// the values.
</span><span class="kw">unsafe impl</span>&lt;T: Send&gt; Send <span class="kw">for </span>IdleNotifiedSet&lt;T&gt; {}
<span class="comment">// With the current API we strictly speaking don't even need `T: Sync`, but we
// require it anyway to support adding &amp;self APIs that access the values in the
// future.
</span><span class="kw">unsafe impl</span>&lt;T: Sync&gt; Sync <span class="kw">for </span>IdleNotifiedSet&lt;T&gt; {}

<span class="comment">// These impls control when it is safe to create a Waker. Since the waker does
// not allow access to the value in any way (including its destructor), it is
// not necessary for `T` to be Send or Sync.
</span><span class="kw">unsafe impl</span>&lt;T&gt; Send <span class="kw">for </span>ListEntry&lt;T&gt; {}
<span class="kw">unsafe impl</span>&lt;T&gt; Sync <span class="kw">for </span>ListEntry&lt;T&gt; {}

<span class="kw">impl</span>&lt;T&gt; IdleNotifiedSet&lt;T&gt; {
    <span class="doccomment">/// Create a new `IdleNotifiedSet`.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>new() -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>lists = Mutex::new(ListsInner {
            notified: LinkedList::new(),
            idle: LinkedList::new(),
            waker: <span class="prelude-val">None</span>,
        });

        IdleNotifiedSet {
            lists: Arc::new(lists),
            length: <span class="number">0</span>,
        }
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.length
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>is_empty(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.length == <span class="number">0
    </span>}

    <span class="doccomment">/// Insert the given value into the `idle` list.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>insert_idle(<span class="kw-2">&amp;mut </span><span class="self">self</span>, value: T) -&gt; EntryInOneOfTheLists&lt;<span class="lifetime">'_</span>, T&gt; {
        <span class="self">self</span>.length += <span class="number">1</span>;

        <span class="kw">let </span>entry = Arc::new(ListEntry {
            parent: <span class="self">self</span>.lists.clone(),
            value: UnsafeCell::new(ManuallyDrop::new(value)),
            my_list: UnsafeCell::new(List::Idle),
            pointers: linked_list::Pointers::new(),
            _pin: PhantomPinned,
        });

        {
            <span class="kw">let </span><span class="kw-2">mut </span>lock = <span class="self">self</span>.lists.lock();
            lock.idle.push_front(entry.clone());
        }

        <span class="comment">// Safety: We just put the entry in the idle list, so it is in one of the lists.
        </span>EntryInOneOfTheLists { entry, set: <span class="self">self </span>}
    }

    <span class="doccomment">/// Pop an entry from the notified list to poll it. The entry is moved to
    /// the idle list atomically.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>pop_notified(<span class="kw-2">&amp;mut </span><span class="self">self</span>, waker: <span class="kw-2">&amp;</span>Waker) -&gt; <span class="prelude-ty">Option</span>&lt;EntryInOneOfTheLists&lt;<span class="lifetime">'_</span>, T&gt;&gt; {
        <span class="comment">// We don't decrement the length because this call moves the entry to
        // the idle list rather than removing it.
        </span><span class="kw">if </span><span class="self">self</span>.length == <span class="number">0 </span>{
            <span class="comment">// Fast path.
            </span><span class="kw">return </span><span class="prelude-val">None</span>;
        }

        <span class="kw">let </span><span class="kw-2">mut </span>lock = <span class="self">self</span>.lists.lock();

        <span class="kw">let </span>should_update_waker = <span class="kw">match </span>lock.waker.as_mut() {
            <span class="prelude-val">Some</span>(cur_waker) =&gt; !waker.will_wake(cur_waker),
            <span class="prelude-val">None </span>=&gt; <span class="bool-val">true</span>,
        };
        <span class="kw">if </span>should_update_waker {
            lock.waker = <span class="prelude-val">Some</span>(waker.clone());
        }

        <span class="comment">// Pop the entry, returning None if empty.
        </span><span class="kw">let </span>entry = lock.notified.pop_back()<span class="question-mark">?</span>;

        lock.idle.push_front(entry.clone());

        <span class="comment">// Safety: We are holding the lock.
        </span>entry.my_list.with_mut(|ptr| <span class="kw">unsafe </span>{
            <span class="kw-2">*</span>ptr = List::Idle;
        });

        drop(lock);

        <span class="comment">// Safety: We just put the entry in the idle list, so it is in one of the lists.
        </span><span class="prelude-val">Some</span>(EntryInOneOfTheLists { entry, set: <span class="self">self </span>})
    }

    <span class="doccomment">/// Tries to pop an entry from the notified list to poll it. The entry is moved to
    /// the idle list atomically.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>try_pop_notified(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;EntryInOneOfTheLists&lt;<span class="lifetime">'_</span>, T&gt;&gt; {
        <span class="comment">// We don't decrement the length because this call moves the entry to
        // the idle list rather than removing it.
        </span><span class="kw">if </span><span class="self">self</span>.length == <span class="number">0 </span>{
            <span class="comment">// Fast path.
            </span><span class="kw">return </span><span class="prelude-val">None</span>;
        }

        <span class="kw">let </span><span class="kw-2">mut </span>lock = <span class="self">self</span>.lists.lock();

        <span class="comment">// Pop the entry, returning None if empty.
        </span><span class="kw">let </span>entry = lock.notified.pop_back()<span class="question-mark">?</span>;

        lock.idle.push_front(entry.clone());

        <span class="comment">// Safety: We are holding the lock.
        </span>entry.my_list.with_mut(|ptr| <span class="kw">unsafe </span>{
            <span class="kw-2">*</span>ptr = List::Idle;
        });

        drop(lock);

        <span class="comment">// Safety: We just put the entry in the idle list, so it is in one of the lists.
        </span><span class="prelude-val">Some</span>(EntryInOneOfTheLists { entry, set: <span class="self">self </span>})
    }

    <span class="doccomment">/// Call a function on every element in this list.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>for_each&lt;F: FnMut(<span class="kw-2">&amp;mut </span>T)&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, <span class="kw-2">mut </span>func: F) {
        <span class="kw">fn </span>get_ptrs&lt;T&gt;(list: <span class="kw-2">&amp;mut </span>LinkedList&lt;T&gt;, ptrs: <span class="kw-2">&amp;mut </span>Vec&lt;<span class="kw-2">*mut </span>T&gt;) {
            <span class="kw">let </span><span class="kw-2">mut </span>node = list.last();

            <span class="kw">while let </span><span class="prelude-val">Some</span>(entry) = node {
                ptrs.push(entry.value.with_mut(|ptr| {
                    <span class="kw">let </span>ptr: <span class="kw-2">*mut </span>ManuallyDrop&lt;T&gt; = ptr;
                    <span class="kw">let </span>ptr: <span class="kw-2">*mut </span>T = ptr.cast();
                    ptr
                }));

                <span class="kw">let </span>prev = entry.pointers.get_prev();
                node = prev.map(|prev| <span class="kw">unsafe </span>{ <span class="kw-2">&amp;*</span>prev.as_ptr() });
            }
        }

        <span class="comment">// Atomically get a raw pointer to the value of every entry.
        //
        // Since this only locks the mutex once, it is not possible for a value
        // to get moved from the idle list to the notified list during the
        // operation, which would otherwise result in some value being listed
        // twice.
        </span><span class="kw">let </span><span class="kw-2">mut </span>ptrs = Vec::with_capacity(<span class="self">self</span>.len());
        {
            <span class="kw">let </span><span class="kw-2">mut </span>lock = <span class="self">self</span>.lists.lock();

            get_ptrs(<span class="kw-2">&amp;mut </span>lock.idle, <span class="kw-2">&amp;mut </span>ptrs);
            get_ptrs(<span class="kw-2">&amp;mut </span>lock.notified, <span class="kw-2">&amp;mut </span>ptrs);
        }
        <span class="macro">debug_assert_eq!</span>(ptrs.len(), ptrs.capacity());

        <span class="kw">for </span>ptr <span class="kw">in </span>ptrs {
            <span class="comment">// Safety: When we grabbed the pointers, the entries were in one of
            // the two lists. This means that their value was valid at the time,
            // and it must still be valid because we are the IdleNotifiedSet,
            // and only we can remove an entry from the two lists. (It's
            // possible that an entry is moved from one list to the other during
            // this loop, but that is ok.)
            </span>func(<span class="kw">unsafe </span>{ <span class="kw-2">&amp;mut *</span>ptr });
        }
    }

    <span class="doccomment">/// Remove all entries in both lists, applying some function to each element.
    ///
    /// The closure is called on all elements even if it panics. Having it panic
    /// twice is a double-panic, and will abort the application.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>drain&lt;F: FnMut(T)&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, func: F) {
        <span class="kw">if </span><span class="self">self</span>.length == <span class="number">0 </span>{
            <span class="comment">// Fast path.
            </span><span class="kw">return</span>;
        }
        <span class="self">self</span>.length = <span class="number">0</span>;

        <span class="comment">// The LinkedList is not cleared on panic, so we use a bomb to clear it.
        //
        // This value has the invariant that any entry in its `all_entries` list
        // has `my_list` set to `Neither` and that the value has not yet been
        // dropped.
        </span><span class="kw">struct </span>AllEntries&lt;T, F: FnMut(T)&gt; {
            all_entries: LinkedList&lt;T&gt;,
            func: F,
        }

        <span class="kw">impl</span>&lt;T, F: FnMut(T)&gt; AllEntries&lt;T, F&gt; {
            <span class="kw">fn </span>pop_next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; bool {
                <span class="kw">if let </span><span class="prelude-val">Some</span>(entry) = <span class="self">self</span>.all_entries.pop_back() {
                    <span class="comment">// Safety: We just took this value from the list, so we can
                    // destroy the value in the entry.
                    </span>entry
                        .value
                        .with_mut(|ptr| <span class="kw">unsafe </span>{ (<span class="self">self</span>.func)(ManuallyDrop::take(<span class="kw-2">&amp;mut *</span>ptr)) });
                    <span class="bool-val">true
                </span>} <span class="kw">else </span>{
                    <span class="bool-val">false
                </span>}
            }
        }

        <span class="kw">impl</span>&lt;T, F: FnMut(T)&gt; Drop <span class="kw">for </span>AllEntries&lt;T, F&gt; {
            <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
                <span class="kw">while </span><span class="self">self</span>.pop_next() {}
            }
        }

        <span class="kw">let </span><span class="kw-2">mut </span>all_entries = AllEntries {
            all_entries: LinkedList::new(),
            func,
        };

        <span class="comment">// Atomically move all entries to the new linked list in the AllEntries
        // object.
        </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>lock = <span class="self">self</span>.lists.lock();
            <span class="kw">unsafe </span>{
                <span class="comment">// Safety: We are holding the lock and `all_entries` is a new
                // LinkedList.
                </span>move_to_new_list(<span class="kw-2">&amp;mut </span>lock.idle, <span class="kw-2">&amp;mut </span>all_entries.all_entries);
                move_to_new_list(<span class="kw-2">&amp;mut </span>lock.notified, <span class="kw-2">&amp;mut </span>all_entries.all_entries);
            }
        }

        <span class="comment">// Keep destroying entries in the list until it is empty.
        //
        // If the closure panics, then the destructor of the `AllEntries` bomb
        // ensures that we keep running the destructor on the remaining values.
        // A second panic will abort the program.
        </span><span class="kw">while </span>all_entries.pop_next() {}
    }
}

<span class="doccomment">/// # Safety
///
/// The mutex for the entries must be held, and the target list must be such
/// that setting `my_list` to `Neither` is ok.
</span><span class="kw">unsafe fn </span>move_to_new_list&lt;T&gt;(from: <span class="kw-2">&amp;mut </span>LinkedList&lt;T&gt;, to: <span class="kw-2">&amp;mut </span>LinkedList&lt;T&gt;) {
    <span class="kw">while let </span><span class="prelude-val">Some</span>(entry) = from.pop_back() {
        entry.my_list.with_mut(|ptr| {
            <span class="kw-2">*</span>ptr = List::Neither;
        });
        to.push_front(entry);
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, T&gt; EntryInOneOfTheLists&lt;<span class="lifetime">'a</span>, T&gt; {
    <span class="doccomment">/// Remove this entry from the list it is in, returning the value associated
    /// with the entry.
    ///
    /// This consumes the value, since it is no longer guaranteed to be in a
    /// list.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>remove(<span class="self">self</span>) -&gt; T {
        <span class="self">self</span>.set.length -= <span class="number">1</span>;

        {
            <span class="kw">let </span><span class="kw-2">mut </span>lock = <span class="self">self</span>.set.lists.lock();

            <span class="comment">// Safety: We are holding the lock so there is no race, and we will
            // remove the entry afterwards to uphold invariants.
            </span><span class="kw">let </span>old_my_list = <span class="self">self</span>.entry.my_list.with_mut(|ptr| <span class="kw">unsafe </span>{
                <span class="kw">let </span>old_my_list = <span class="kw-2">*</span>ptr;
                <span class="kw-2">*</span>ptr = List::Neither;
                old_my_list
            });

            <span class="kw">let </span>list = <span class="kw">match </span>old_my_list {
                List::Idle =&gt; <span class="kw-2">&amp;mut </span>lock.idle,
                List::Notified =&gt; <span class="kw-2">&amp;mut </span>lock.notified,
                <span class="comment">// An entry in one of the lists is in one of the lists.
                </span>List::Neither =&gt; <span class="macro">unreachable!</span>(),
            };

            <span class="kw">unsafe </span>{
                <span class="comment">// Safety: We just checked that the entry is in this particular
                // list.
                </span>list.remove(ListEntry::as_raw(<span class="kw-2">&amp;</span><span class="self">self</span>.entry)).unwrap();
            }
        }

        <span class="comment">// By setting `my_list` to `Neither`, we have taken ownership of the
        // value. We return it to the caller.
        //
        // Safety: We have a mutable reference to the `IdleNotifiedSet` that
        // owns this entry, so we can use its permission to access the value.
        </span><span class="self">self</span>.entry
            .value
            .with_mut(|ptr| <span class="kw">unsafe </span>{ ManuallyDrop::take(<span class="kw-2">&amp;mut *</span>ptr) })
    }

    <span class="doccomment">/// Access the value in this entry together with a context for its waker.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>with_value_and_context&lt;F, U&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, func: F) -&gt; U
    <span class="kw">where
        </span>F: FnOnce(<span class="kw-2">&amp;mut </span>T, <span class="kw-2">&amp;mut </span>Context&lt;<span class="lifetime">'_</span>&gt;) -&gt; U,
        T: <span class="lifetime">'static</span>,
    {
        <span class="kw">let </span>waker = waker_ref(<span class="kw-2">&amp;</span><span class="self">self</span>.entry);

        <span class="kw">let </span><span class="kw-2">mut </span>context = Context::from_waker(<span class="kw-2">&amp;</span>waker);

        <span class="comment">// Safety: We have a mutable reference to the `IdleNotifiedSet` that
        // owns this entry, so we can use its permission to access the value.
        </span><span class="self">self</span>.entry
            .value
            .with_mut(|ptr| <span class="kw">unsafe </span>{ func(<span class="kw-2">&amp;mut *</span>ptr, <span class="kw-2">&amp;mut </span>context) })
    }
}

<span class="kw">impl</span>&lt;T&gt; Drop <span class="kw">for </span>IdleNotifiedSet&lt;T&gt; {
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="comment">// Clear both lists.
        </span><span class="self">self</span>.drain(drop);

        <span class="attr">#[cfg(debug_assertions)]
        </span><span class="kw">if </span>!std::thread::panicking() {
            <span class="kw">let </span>lock = <span class="self">self</span>.lists.lock();
            <span class="macro">assert!</span>(lock.idle.is_empty());
            <span class="macro">assert!</span>(lock.notified.is_empty());
        }
    }
}

<span class="kw">impl</span>&lt;T: <span class="lifetime">'static</span>&gt; Wake <span class="kw">for </span>ListEntry&lt;T&gt; {
    <span class="kw">fn </span>wake_by_ref(me: <span class="kw-2">&amp;</span>Arc&lt;<span class="self">Self</span>&gt;) {
        <span class="kw">let </span><span class="kw-2">mut </span>lock = me.parent.lock();

        <span class="comment">// Safety: We are holding the lock and we will update the lists to
        // maintain invariants.
        </span><span class="kw">let </span>old_my_list = me.my_list.with_mut(|ptr| <span class="kw">unsafe </span>{
            <span class="kw">let </span>old_my_list = <span class="kw-2">*</span>ptr;
            <span class="kw">if </span>old_my_list == List::Idle {
                <span class="kw-2">*</span>ptr = List::Notified;
            }
            old_my_list
        });

        <span class="kw">if </span>old_my_list == List::Idle {
            <span class="comment">// We move ourself to the notified list.
            </span><span class="kw">let </span>me = <span class="kw">unsafe </span>{
                <span class="comment">// Safety: We just checked that we are in this particular list.
                </span>lock.idle.remove(ListEntry::as_raw(me)).unwrap()
            };
            lock.notified.push_front(me);

            <span class="kw">if let </span><span class="prelude-val">Some</span>(waker) = lock.waker.take() {
                drop(lock);
                waker.wake();
            }
        }
    }

    <span class="kw">fn </span>wake(me: Arc&lt;<span class="self">Self</span>&gt;) {
        <span class="self">Self</span>::wake_by_ref(<span class="kw-2">&amp;</span>me);
    }
}

<span class="doccomment">/// # Safety
///
/// `ListEntry` is forced to be !Unpin.
</span><span class="kw">unsafe impl</span>&lt;T&gt; linked_list::Link <span class="kw">for </span>ListEntry&lt;T&gt; {
    <span class="kw">type </span>Handle = Arc&lt;ListEntry&lt;T&gt;&gt;;
    <span class="kw">type </span>Target = ListEntry&lt;T&gt;;

    <span class="kw">fn </span>as_raw(handle: <span class="kw-2">&amp;</span><span class="self">Self</span>::Handle) -&gt; NonNull&lt;ListEntry&lt;T&gt;&gt; {
        <span class="kw">let </span>ptr: <span class="kw-2">*const </span>ListEntry&lt;T&gt; = Arc::as_ptr(handle);
        <span class="comment">// Safety: We can't get a null pointer from `Arc::as_ptr`.
        </span><span class="kw">unsafe </span>{ NonNull::new_unchecked(ptr <span class="kw">as </span><span class="kw-2">*mut </span>ListEntry&lt;T&gt;) }
    }

    <span class="kw">unsafe fn </span>from_raw(ptr: NonNull&lt;ListEntry&lt;T&gt;&gt;) -&gt; Arc&lt;ListEntry&lt;T&gt;&gt; {
        Arc::from_raw(ptr.as_ptr())
    }

    <span class="kw">unsafe fn </span>pointers(
        target: NonNull&lt;ListEntry&lt;T&gt;&gt;,
    ) -&gt; NonNull&lt;linked_list::Pointers&lt;ListEntry&lt;T&gt;&gt;&gt; {
        ListEntry::addr_of_pointers(target)
    }
}

<span class="attr">#[cfg(all(test, not(loom)))]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span><span class="kw">crate</span>::runtime::Builder;
    <span class="kw">use </span><span class="kw">crate</span>::task::JoinSet;

    <span class="comment">// A test that runs under miri.
    //
    // https://github.com/tokio-rs/tokio/pull/5693
    </span><span class="attr">#[test]
    </span><span class="kw">fn </span>join_set_test() {
        <span class="kw">let </span>rt = Builder::new_current_thread().build().unwrap();

        <span class="kw">let </span><span class="kw-2">mut </span>set = JoinSet::new();
        set.spawn_on(futures::future::ready(()), rt.handle());

        rt.block_on(set.join_next()).unwrap().unwrap();
    }
}
</code></pre></div></section></main></body></html>