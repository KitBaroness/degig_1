<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rustls-0.21.11/src/server/tls12.rs`."><title>tls12.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="rustls" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../rustls/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">crate</span>::check::inappropriate_message;
<span class="kw">use </span><span class="kw">crate</span>::common_state::{CommonState, Side, State};
<span class="kw">use </span><span class="kw">crate</span>::conn::ConnectionRandoms;
<span class="kw">use </span><span class="kw">crate</span>::enums::ProtocolVersion;
<span class="kw">use </span><span class="kw">crate</span>::enums::{AlertDescription, ContentType, HandshakeType};
<span class="kw">use </span><span class="kw">crate</span>::error::{Error, PeerIncompatible, PeerMisbehaved};
<span class="kw">use </span><span class="kw">crate</span>::hash_hs::HandshakeHash;
<span class="kw">use </span><span class="kw">crate</span>::key::Certificate;
<span class="attr">#[cfg(feature = <span class="string">"logging"</span>)]
</span><span class="kw">use </span><span class="kw">crate</span>::log::{debug, trace};
<span class="kw">use </span><span class="kw">crate</span>::msgs::base::Payload;
<span class="kw">use </span><span class="kw">crate</span>::msgs::ccs::ChangeCipherSpecPayload;
<span class="kw">use </span><span class="kw">crate</span>::msgs::codec::Codec;
<span class="kw">use </span><span class="kw">crate</span>::msgs::handshake::{ClientECDHParams, HandshakeMessagePayload, HandshakePayload};
<span class="kw">use </span><span class="kw">crate</span>::msgs::handshake::{NewSessionTicketPayload, SessionId};
<span class="kw">use </span><span class="kw">crate</span>::msgs::message::{Message, MessagePayload};
<span class="kw">use </span><span class="kw">crate</span>::msgs::persist;
<span class="attr">#[cfg(feature = <span class="string">"secret_extraction"</span>)]
</span><span class="kw">use </span><span class="kw">crate</span>::suites::PartiallyExtractedSecrets;
<span class="kw">use </span><span class="kw">crate</span>::tls12::{<span class="self">self</span>, ConnectionSecrets, Tls12CipherSuite};
<span class="kw">use crate</span>::{kx, ticketer, verify};

<span class="kw">use </span><span class="kw">super</span>::common::ActiveCertifiedKey;
<span class="kw">use </span><span class="kw">super</span>::hs::{<span class="self">self</span>, ServerContext};
<span class="kw">use </span><span class="kw">super</span>::server_conn::{ProducesTickets, ServerConfig, ServerConnectionData};

<span class="kw">use </span>ring::constant_time;

<span class="kw">use </span>std::sync::Arc;

<span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">use </span>client_hello::CompleteClientHelloHandling;

<span class="kw">mod </span>client_hello {
    <span class="kw">use </span><span class="kw">crate</span>::enums::SignatureScheme;
    <span class="kw">use </span><span class="kw">crate</span>::msgs::enums::ECPointFormat;
    <span class="kw">use </span><span class="kw">crate</span>::msgs::enums::{ClientCertificateType, Compression};
    <span class="kw">use </span><span class="kw">crate</span>::msgs::handshake::ClientExtension;
    <span class="kw">use </span><span class="kw">crate</span>::msgs::handshake::ServerECDHParams;
    <span class="kw">use </span><span class="kw">crate</span>::msgs::handshake::{CertificateRequestPayload, ClientSessionTicket, Random};
    <span class="kw">use </span><span class="kw">crate</span>::msgs::handshake::{CertificateStatus, ECDHEServerKeyExchange};
    <span class="kw">use </span><span class="kw">crate</span>::msgs::handshake::{ClientHelloPayload, ServerHelloPayload};
    <span class="kw">use </span><span class="kw">crate</span>::msgs::handshake::{ServerExtension, ServerKeyExchangePayload};
    <span class="kw">use </span><span class="kw">crate</span>::sign;
    <span class="kw">use </span><span class="kw">crate</span>::verify::DigitallySignedStruct;

    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::server) <span class="kw">struct </span>CompleteClientHelloHandling {
        <span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::server) config: Arc&lt;ServerConfig&gt;,
        <span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::server) transcript: HandshakeHash,
        <span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::server) session_id: SessionId,
        <span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::server) suite: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>Tls12CipherSuite,
        <span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::server) using_ems: bool,
        <span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::server) randoms: ConnectionRandoms,
        <span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::server) send_ticket: bool,
        <span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::server) extra_exts: Vec&lt;ServerExtension&gt;,
    }

    <span class="kw">impl </span>CompleteClientHelloHandling {
        <span class="kw">pub</span>(<span class="kw">in </span><span class="kw">crate</span>::server) <span class="kw">fn </span>handle_client_hello(
            <span class="kw-2">mut </span><span class="self">self</span>,
            cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;,
            server_key: ActiveCertifiedKey,
            chm: <span class="kw-2">&amp;</span>Message,
            client_hello: <span class="kw-2">&amp;</span>ClientHelloPayload,
            sigschemes_ext: Vec&lt;SignatureScheme&gt;,
            tls13_enabled: bool,
        ) -&gt; hs::NextStateOrError {
            <span class="comment">// -- TLS1.2 only from hereon in --
            </span><span class="self">self</span>.transcript.add_message(chm);

            <span class="kw">if </span>client_hello.ems_support_offered() {
                <span class="self">self</span>.using_ems = <span class="bool-val">true</span>;
            }

            <span class="kw">let </span>groups_ext = client_hello
                .get_namedgroups_extension()
                .ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::NamedGroupsExtensionRequired,
                    )
                })<span class="question-mark">?</span>;
            <span class="kw">let </span>ecpoints_ext = client_hello
                .get_ecpoints_extension()
                .ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::EcPointsExtensionRequired,
                    )
                })<span class="question-mark">?</span>;

            <span class="macro">trace!</span>(<span class="string">"namedgroups {:?}"</span>, groups_ext);
            <span class="macro">trace!</span>(<span class="string">"ecpoints {:?}"</span>, ecpoints_ext);

            <span class="kw">if </span>!ecpoints_ext.contains(<span class="kw-2">&amp;</span>ECPointFormat::Uncompressed) {
                <span class="kw">return </span><span class="prelude-val">Err</span>(cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerIncompatible::UncompressedEcPointsRequired,
                ));
            }

            <span class="comment">// -- If TLS1.3 is enabled, signal the downgrade in the server random
            </span><span class="kw">if </span>tls13_enabled {
                <span class="self">self</span>.randoms.server[<span class="number">24</span>..].copy_from_slice(<span class="kw-2">&amp;</span>tls12::DOWNGRADE_SENTINEL);
            }

            <span class="comment">// -- Check for resumption --
            // We can do this either by (in order of preference):
            // 1. receiving a ticket that decrypts
            // 2. receiving a sessionid that is in our cache
            //
            // If we receive a ticket, the sessionid won't be in our
            // cache, so don't check.
            //
            // If either works, we end up with a ServerConnectionValue
            // which is passed to start_resumption and concludes
            // our handling of the ClientHello.
            //
            </span><span class="kw">let </span><span class="kw-2">mut </span>ticket_received = <span class="bool-val">false</span>;
            <span class="kw">let </span>resume_data = client_hello
                .get_ticket_extension()
                .and_then(|ticket_ext| <span class="kw">match </span>ticket_ext {
                    ClientExtension::SessionTicket(ClientSessionTicket::Offer(ticket)) =&gt; {
                        <span class="prelude-val">Some</span>(ticket)
                    }
                    <span class="kw">_ </span>=&gt; <span class="prelude-val">None</span>,
                })
                .and_then(|ticket| {
                    ticket_received = <span class="bool-val">true</span>;
                    <span class="macro">debug!</span>(<span class="string">"Ticket received"</span>);
                    <span class="kw">let </span>data = <span class="self">self</span>.config.ticketer.decrypt(<span class="kw-2">&amp;</span>ticket.<span class="number">0</span>);
                    <span class="kw">if </span>data.is_none() {
                        <span class="macro">debug!</span>(<span class="string">"Ticket didn't decrypt"</span>);
                    }
                    data
                })
                .or_else(|| {
                    <span class="comment">// Perhaps resume?  If we received a ticket, the sessionid
                    // does not correspond to a real session.
                    </span><span class="kw">if </span>client_hello.session_id.is_empty() || ticket_received {
                        <span class="kw">return </span><span class="prelude-val">None</span>;
                    }

                    <span class="self">self</span>.config
                        .session_storage
                        .get(<span class="kw-2">&amp;</span>client_hello.session_id.get_encoding())
                })
                .and_then(|x| persist::ServerSessionValue::read_bytes(<span class="kw-2">&amp;</span>x).ok())
                .filter(|resumedata| {
                    hs::can_resume(<span class="self">self</span>.suite.into(), <span class="kw-2">&amp;</span>cx.data.sni, <span class="self">self</span>.using_ems, resumedata)
                });

            <span class="kw">if let </span><span class="prelude-val">Some</span>(data) = resume_data {
                <span class="kw">return </span><span class="self">self</span>.start_resumption(cx, client_hello, <span class="kw-2">&amp;</span>client_hello.session_id, data);
            }

            <span class="comment">// Now we have chosen a ciphersuite, we can make kx decisions.
            </span><span class="kw">let </span>sigschemes = <span class="self">self
                </span>.suite
                .resolve_sig_schemes(<span class="kw-2">&amp;</span>sigschemes_ext);

            <span class="kw">if </span>sigschemes.is_empty() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(cx.common.send_fatal_alert(
                    AlertDescription::HandshakeFailure,
                    PeerIncompatible::NoSignatureSchemesInCommon,
                ));
            }

            <span class="kw">let </span>group = <span class="self">self
                </span>.config
                .kx_groups
                .iter()
                .find(|skxg| groups_ext.contains(<span class="kw-2">&amp;</span>skxg.name))
                .cloned()
                .ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::NoKxGroupsInCommon,
                    )
                })<span class="question-mark">?</span>;

            <span class="kw">let </span>ecpoint = ECPointFormat::SUPPORTED
                .iter()
                .find(|format| ecpoints_ext.contains(format))
                .cloned()
                .ok_or_else(|| {
                    cx.common.send_fatal_alert(
                        AlertDescription::HandshakeFailure,
                        PeerIncompatible::NoEcPointFormatsInCommon,
                    )
                })<span class="question-mark">?</span>;

            <span class="macro">debug_assert_eq!</span>(ecpoint, ECPointFormat::Uncompressed);

            <span class="kw">let </span>(<span class="kw-2">mut </span>ocsp_response, <span class="kw-2">mut </span>sct_list) =
                (server_key.get_ocsp(), server_key.get_sct_list());

            <span class="comment">// If we're not offered a ticket or a potential session ID, allocate a session ID.
            </span><span class="kw">if </span>!<span class="self">self</span>.config.session_storage.can_cache() {
                <span class="self">self</span>.session_id = SessionId::empty();
            } <span class="kw">else if </span><span class="self">self</span>.session_id.is_empty() &amp;&amp; !ticket_received {
                <span class="self">self</span>.session_id = SessionId::random()<span class="question-mark">?</span>;
            }

            <span class="self">self</span>.send_ticket = emit_server_hello(
                <span class="kw-2">&amp;</span><span class="self">self</span>.config,
                <span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript,
                cx,
                <span class="self">self</span>.session_id,
                <span class="self">self</span>.suite,
                <span class="self">self</span>.using_ems,
                <span class="kw-2">&amp;mut </span>ocsp_response,
                <span class="kw-2">&amp;mut </span>sct_list,
                client_hello,
                <span class="prelude-val">None</span>,
                <span class="kw-2">&amp;</span><span class="self">self</span>.randoms,
                <span class="self">self</span>.extra_exts,
            )<span class="question-mark">?</span>;
            emit_certificate(<span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript, cx.common, server_key.get_cert());
            <span class="kw">if let </span><span class="prelude-val">Some</span>(ocsp_response) = ocsp_response {
                emit_cert_status(<span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript, cx.common, ocsp_response);
            }
            <span class="kw">let </span>server_kx = emit_server_kx(
                <span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript,
                cx.common,
                sigschemes,
                group,
                server_key.get_key(),
                <span class="kw-2">&amp;</span><span class="self">self</span>.randoms,
            )<span class="question-mark">?</span>;
            <span class="kw">let </span>doing_client_auth = emit_certificate_req(<span class="kw-2">&amp;</span><span class="self">self</span>.config, <span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript, cx)<span class="question-mark">?</span>;
            emit_server_hello_done(<span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript, cx.common);

            <span class="kw">if </span>doing_client_auth {
                <span class="prelude-val">Ok</span>(Box::new(ExpectCertificate {
                    config: <span class="self">self</span>.config,
                    transcript: <span class="self">self</span>.transcript,
                    randoms: <span class="self">self</span>.randoms,
                    session_id: <span class="self">self</span>.session_id,
                    suite: <span class="self">self</span>.suite,
                    using_ems: <span class="self">self</span>.using_ems,
                    server_kx,
                    send_ticket: <span class="self">self</span>.send_ticket,
                }))
            } <span class="kw">else </span>{
                <span class="prelude-val">Ok</span>(Box::new(ExpectClientKx {
                    config: <span class="self">self</span>.config,
                    transcript: <span class="self">self</span>.transcript,
                    randoms: <span class="self">self</span>.randoms,
                    session_id: <span class="self">self</span>.session_id,
                    suite: <span class="self">self</span>.suite,
                    using_ems: <span class="self">self</span>.using_ems,
                    server_kx,
                    client_cert: <span class="prelude-val">None</span>,
                    send_ticket: <span class="self">self</span>.send_ticket,
                }))
            }
        }

        <span class="kw">fn </span>start_resumption(
            <span class="kw-2">mut </span><span class="self">self</span>,
            cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;,
            client_hello: <span class="kw-2">&amp;</span>ClientHelloPayload,
            id: <span class="kw-2">&amp;</span>SessionId,
            resumedata: persist::ServerSessionValue,
        ) -&gt; hs::NextStateOrError {
            <span class="macro">debug!</span>(<span class="string">"Resuming connection"</span>);

            <span class="kw">if </span>resumedata.extended_ms &amp;&amp; !<span class="self">self</span>.using_ems {
                <span class="kw">return </span><span class="prelude-val">Err</span>(cx.common.send_fatal_alert(
                    AlertDescription::IllegalParameter,
                    PeerMisbehaved::ResumptionAttemptedWithVariedEms,
                ));
            }

            <span class="self">self</span>.session_id = <span class="kw-2">*</span>id;
            <span class="self">self</span>.send_ticket = emit_server_hello(
                <span class="kw-2">&amp;</span><span class="self">self</span>.config,
                <span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript,
                cx,
                <span class="self">self</span>.session_id,
                <span class="self">self</span>.suite,
                <span class="self">self</span>.using_ems,
                <span class="kw-2">&amp;mut </span><span class="prelude-val">None</span>,
                <span class="kw-2">&amp;mut </span><span class="prelude-val">None</span>,
                client_hello,
                <span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span>resumedata),
                <span class="kw-2">&amp;</span><span class="self">self</span>.randoms,
                <span class="self">self</span>.extra_exts,
            )<span class="question-mark">?</span>;

            <span class="kw">let </span>secrets = ConnectionSecrets::new_resume(
                <span class="self">self</span>.randoms,
                <span class="self">self</span>.suite,
                <span class="kw-2">&amp;</span>resumedata.master_secret.<span class="number">0</span>,
            );
            <span class="self">self</span>.config.key_log.log(
                <span class="string">"CLIENT_RANDOM"</span>,
                <span class="kw-2">&amp;</span>secrets.randoms.client,
                <span class="kw-2">&amp;</span>secrets.master_secret,
            );
            cx.common
                .start_encryption_tls12(<span class="kw-2">&amp;</span>secrets, Side::Server);
            cx.common.peer_certificates = resumedata.client_cert_chain;

            <span class="kw">if </span><span class="self">self</span>.send_ticket {
                emit_ticket(
                    <span class="kw-2">&amp;</span>secrets,
                    <span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript,
                    <span class="self">self</span>.using_ems,
                    cx,
                    <span class="kw-2">&amp;*</span><span class="self">self</span>.config.ticketer,
                )<span class="question-mark">?</span>;
            }
            emit_ccs(cx.common);
            cx.common
                .record_layer
                .start_encrypting();
            emit_finished(<span class="kw-2">&amp;</span>secrets, <span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript, cx.common);

            <span class="prelude-val">Ok</span>(Box::new(ExpectCcs {
                config: <span class="self">self</span>.config,
                secrets,
                transcript: <span class="self">self</span>.transcript,
                session_id: <span class="self">self</span>.session_id,
                using_ems: <span class="self">self</span>.using_ems,
                resuming: <span class="bool-val">true</span>,
                send_ticket: <span class="self">self</span>.send_ticket,
            }))
        }
    }

    <span class="kw">fn </span>emit_server_hello(
        config: <span class="kw-2">&amp;</span>ServerConfig,
        transcript: <span class="kw-2">&amp;mut </span>HandshakeHash,
        cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;,
        session_id: SessionId,
        suite: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>Tls12CipherSuite,
        using_ems: bool,
        ocsp_response: <span class="kw-2">&amp;mut </span><span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>[u8]&gt;,
        sct_list: <span class="kw-2">&amp;mut </span><span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>[u8]&gt;,
        hello: <span class="kw-2">&amp;</span>ClientHelloPayload,
        resumedata: <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>persist::ServerSessionValue&gt;,
        randoms: <span class="kw-2">&amp;</span>ConnectionRandoms,
        extra_exts: Vec&lt;ServerExtension&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;bool, Error&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>ep = hs::ExtensionProcessing::new();
        ep.process_common(
            config,
            cx,
            ocsp_response,
            sct_list,
            hello,
            resumedata,
            extra_exts,
        )<span class="question-mark">?</span>;
        ep.process_tls12(config, hello, using_ems);

        <span class="kw">let </span>sh = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::ServerHello,
                payload: HandshakePayload::ServerHello(ServerHelloPayload {
                    legacy_version: ProtocolVersion::TLSv1_2,
                    random: Random::from(randoms.server),
                    session_id,
                    cipher_suite: suite.common.suite,
                    compression_method: Compression::Null,
                    extensions: ep.exts,
                }),
            }),
        };

        <span class="macro">trace!</span>(<span class="string">"sending server hello {:?}"</span>, sh);
        transcript.add_message(<span class="kw-2">&amp;</span>sh);
        cx.common.send_msg(sh, <span class="bool-val">false</span>);
        <span class="prelude-val">Ok</span>(ep.send_ticket)
    }

    <span class="kw">fn </span>emit_certificate(
        transcript: <span class="kw-2">&amp;mut </span>HandshakeHash,
        common: <span class="kw-2">&amp;mut </span>CommonState,
        cert_chain: <span class="kw-2">&amp;</span>[Certificate],
    ) {
        <span class="kw">let </span>c = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::Certificate,
                payload: HandshakePayload::Certificate(cert_chain.to_owned()),
            }),
        };

        transcript.add_message(<span class="kw-2">&amp;</span>c);
        common.send_msg(c, <span class="bool-val">false</span>);
    }

    <span class="kw">fn </span>emit_cert_status(transcript: <span class="kw-2">&amp;mut </span>HandshakeHash, common: <span class="kw-2">&amp;mut </span>CommonState, ocsp: <span class="kw-2">&amp;</span>[u8]) {
        <span class="kw">let </span>st = CertificateStatus::new(ocsp.to_owned());

        <span class="kw">let </span>c = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::CertificateStatus,
                payload: HandshakePayload::CertificateStatus(st),
            }),
        };

        transcript.add_message(<span class="kw-2">&amp;</span>c);
        common.send_msg(c, <span class="bool-val">false</span>);
    }

    <span class="kw">fn </span>emit_server_kx(
        transcript: <span class="kw-2">&amp;mut </span>HandshakeHash,
        common: <span class="kw-2">&amp;mut </span>CommonState,
        sigschemes: Vec&lt;SignatureScheme&gt;,
        skxg: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>kx::SupportedKxGroup,
        signing_key: <span class="kw-2">&amp;</span><span class="kw">dyn </span>sign::SigningKey,
        randoms: <span class="kw-2">&amp;</span>ConnectionRandoms,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;kx::KeyExchange, Error&gt; {
        <span class="kw">let </span>kx = kx::KeyExchange::start(skxg).ok_or(Error::FailedToGetRandomBytes)<span class="question-mark">?</span>;
        <span class="kw">let </span>secdh = ServerECDHParams::new(skxg.name, kx.pubkey.as_ref());

        <span class="kw">let </span><span class="kw-2">mut </span>msg = Vec::new();
        msg.extend(randoms.client);
        msg.extend(randoms.server);
        secdh.encode(<span class="kw-2">&amp;mut </span>msg);

        <span class="kw">let </span>signer = signing_key
            .choose_scheme(<span class="kw-2">&amp;</span>sigschemes)
            .ok_or_else(|| Error::General(<span class="string">"incompatible signing key"</span>.to_string()))<span class="question-mark">?</span>;
        <span class="kw">let </span>sigscheme = signer.scheme();
        <span class="kw">let </span>sig = signer.sign(<span class="kw-2">&amp;</span>msg)<span class="question-mark">?</span>;

        <span class="kw">let </span>skx = ServerKeyExchangePayload::ECDHE(ECDHEServerKeyExchange {
            params: secdh,
            dss: DigitallySignedStruct::new(sigscheme, sig),
        });

        <span class="kw">let </span>m = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::ServerKeyExchange,
                payload: HandshakePayload::ServerKeyExchange(skx),
            }),
        };

        transcript.add_message(<span class="kw-2">&amp;</span>m);
        common.send_msg(m, <span class="bool-val">false</span>);
        <span class="prelude-val">Ok</span>(kx)
    }

    <span class="kw">fn </span>emit_certificate_req(
        config: <span class="kw-2">&amp;</span>ServerConfig,
        transcript: <span class="kw-2">&amp;mut </span>HandshakeHash,
        cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;bool, Error&gt; {
        <span class="kw">let </span>client_auth = <span class="kw-2">&amp;</span>config.verifier;

        <span class="kw">if </span>!client_auth.offer_client_auth() {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="bool-val">false</span>);
        }

        <span class="kw">let </span>verify_schemes = client_auth.supported_verify_schemes();

        <span class="kw">let </span>names = config
            .verifier
            .client_auth_root_subjects()
            .to_vec();

        <span class="kw">let </span>cr = CertificateRequestPayload {
            certtypes: <span class="macro">vec!</span>[
                ClientCertificateType::RSASign,
                ClientCertificateType::ECDSASign,
            ],
            sigschemes: verify_schemes,
            canames: names,
        };

        <span class="kw">let </span>m = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::CertificateRequest,
                payload: HandshakePayload::CertificateRequest(cr),
            }),
        };

        <span class="macro">trace!</span>(<span class="string">"Sending CertificateRequest {:?}"</span>, m);
        transcript.add_message(<span class="kw-2">&amp;</span>m);
        cx.common.send_msg(m, <span class="bool-val">false</span>);
        <span class="prelude-val">Ok</span>(<span class="bool-val">true</span>)
    }

    <span class="kw">fn </span>emit_server_hello_done(transcript: <span class="kw-2">&amp;mut </span>HandshakeHash, common: <span class="kw-2">&amp;mut </span>CommonState) {
        <span class="kw">let </span>m = Message {
            version: ProtocolVersion::TLSv1_2,
            payload: MessagePayload::handshake(HandshakeMessagePayload {
                typ: HandshakeType::ServerHelloDone,
                payload: HandshakePayload::ServerHelloDone,
            }),
        };

        transcript.add_message(<span class="kw-2">&amp;</span>m);
        common.send_msg(m, <span class="bool-val">false</span>);
    }
}

<span class="comment">// --- Process client's Certificate for client auth ---
</span><span class="kw">struct </span>ExpectCertificate {
    config: Arc&lt;ServerConfig&gt;,
    transcript: HandshakeHash,
    randoms: ConnectionRandoms,
    session_id: SessionId,
    suite: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>Tls12CipherSuite,
    using_ems: bool,
    server_kx: kx::KeyExchange,
    send_ticket: bool,
}

<span class="kw">impl </span>State&lt;ServerConnectionData&gt; <span class="kw">for </span>ExpectCertificate {
    <span class="kw">fn </span>handle(<span class="kw-2">mut </span><span class="self">self</span>: Box&lt;<span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;, m: Message) -&gt; hs::NextStateOrError {
        <span class="self">self</span>.transcript.add_message(<span class="kw-2">&amp;</span>m);
        <span class="kw">let </span>cert_chain = <span class="macro">require_handshake_msg_move!</span>(
            m,
            HandshakeType::Certificate,
            HandshakePayload::Certificate
        )<span class="question-mark">?</span>;

        <span class="comment">// If we can't determine if the auth is mandatory, abort
        </span><span class="kw">let </span>mandatory = <span class="self">self
            </span>.config
            .verifier
            .client_auth_mandatory();

        <span class="macro">trace!</span>(<span class="string">"certs {:?}"</span>, cert_chain);

        <span class="kw">let </span>client_cert = <span class="kw">match </span>cert_chain.split_first() {
            <span class="prelude-val">None </span><span class="kw">if </span>mandatory =&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(cx.common.send_fatal_alert(
                    AlertDescription::CertificateRequired,
                    Error::NoCertificatesPresented,
                ));
            }
            <span class="prelude-val">None </span>=&gt; {
                <span class="macro">debug!</span>(<span class="string">"client auth requested but no certificate supplied"</span>);
                <span class="self">self</span>.transcript.abandon_client_auth();
                <span class="prelude-val">None
            </span>}
            <span class="prelude-val">Some</span>((end_entity, intermediates)) =&gt; {
                <span class="kw">let </span>now = std::time::SystemTime::now();
                <span class="self">self</span>.config
                    .verifier
                    .verify_client_cert(end_entity, intermediates, now)
                    .map_err(|err| {
                        cx.common
                            .send_cert_verify_error_alert(err)
                    })<span class="question-mark">?</span>;

                <span class="prelude-val">Some</span>(cert_chain)
            }
        };

        <span class="prelude-val">Ok</span>(Box::new(ExpectClientKx {
            config: <span class="self">self</span>.config,
            transcript: <span class="self">self</span>.transcript,
            randoms: <span class="self">self</span>.randoms,
            session_id: <span class="self">self</span>.session_id,
            suite: <span class="self">self</span>.suite,
            using_ems: <span class="self">self</span>.using_ems,
            server_kx: <span class="self">self</span>.server_kx,
            client_cert,
            send_ticket: <span class="self">self</span>.send_ticket,
        }))
    }
}

<span class="comment">// --- Process client's KeyExchange ---
</span><span class="kw">struct </span>ExpectClientKx {
    config: Arc&lt;ServerConfig&gt;,
    transcript: HandshakeHash,
    randoms: ConnectionRandoms,
    session_id: SessionId,
    suite: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>Tls12CipherSuite,
    using_ems: bool,
    server_kx: kx::KeyExchange,
    client_cert: <span class="prelude-ty">Option</span>&lt;Vec&lt;Certificate&gt;&gt;,
    send_ticket: bool,
}

<span class="kw">impl </span>State&lt;ServerConnectionData&gt; <span class="kw">for </span>ExpectClientKx {
    <span class="kw">fn </span>handle(<span class="kw-2">mut </span><span class="self">self</span>: Box&lt;<span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;, m: Message) -&gt; hs::NextStateOrError {
        <span class="kw">let </span>client_kx = <span class="macro">require_handshake_msg!</span>(
            m,
            HandshakeType::ClientKeyExchange,
            HandshakePayload::ClientKeyExchange
        )<span class="question-mark">?</span>;
        <span class="self">self</span>.transcript.add_message(<span class="kw-2">&amp;</span>m);
        <span class="kw">let </span>ems_seed = <span class="self">self
            </span>.using_ems
            .then(|| <span class="self">self</span>.transcript.get_current_hash());

        <span class="comment">// Complete key agreement, and set up encryption with the
        // resulting premaster secret.
        </span><span class="kw">let </span>peer_kx_params =
            tls12::decode_ecdh_params::&lt;ClientECDHParams&gt;(cx.common, <span class="kw-2">&amp;</span>client_kx.<span class="number">0</span>)<span class="question-mark">?</span>;
        <span class="kw">let </span>secrets = ConnectionSecrets::from_key_exchange(
            <span class="self">self</span>.server_kx,
            <span class="kw-2">&amp;</span>peer_kx_params.public.<span class="number">0</span>,
            ems_seed,
            <span class="self">self</span>.randoms,
            <span class="self">self</span>.suite,
        )<span class="question-mark">?</span>;

        <span class="self">self</span>.config.key_log.log(
            <span class="string">"CLIENT_RANDOM"</span>,
            <span class="kw-2">&amp;</span>secrets.randoms.client,
            <span class="kw-2">&amp;</span>secrets.master_secret,
        );
        cx.common
            .start_encryption_tls12(<span class="kw-2">&amp;</span>secrets, Side::Server);

        <span class="kw">if let </span><span class="prelude-val">Some</span>(client_cert) = <span class="self">self</span>.client_cert {
            <span class="prelude-val">Ok</span>(Box::new(ExpectCertificateVerify {
                config: <span class="self">self</span>.config,
                secrets,
                transcript: <span class="self">self</span>.transcript,
                session_id: <span class="self">self</span>.session_id,
                using_ems: <span class="self">self</span>.using_ems,
                client_cert,
                send_ticket: <span class="self">self</span>.send_ticket,
            }))
        } <span class="kw">else </span>{
            <span class="prelude-val">Ok</span>(Box::new(ExpectCcs {
                config: <span class="self">self</span>.config,
                secrets,
                transcript: <span class="self">self</span>.transcript,
                session_id: <span class="self">self</span>.session_id,
                using_ems: <span class="self">self</span>.using_ems,
                resuming: <span class="bool-val">false</span>,
                send_ticket: <span class="self">self</span>.send_ticket,
            }))
        }
    }
}

<span class="comment">// --- Process client's certificate proof ---
</span><span class="kw">struct </span>ExpectCertificateVerify {
    config: Arc&lt;ServerConfig&gt;,
    secrets: ConnectionSecrets,
    transcript: HandshakeHash,
    session_id: SessionId,
    using_ems: bool,
    client_cert: Vec&lt;Certificate&gt;,
    send_ticket: bool,
}

<span class="kw">impl </span>State&lt;ServerConnectionData&gt; <span class="kw">for </span>ExpectCertificateVerify {
    <span class="kw">fn </span>handle(<span class="kw-2">mut </span><span class="self">self</span>: Box&lt;<span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;, m: Message) -&gt; hs::NextStateOrError {
        <span class="kw">let </span>rc = {
            <span class="kw">let </span>sig = <span class="macro">require_handshake_msg!</span>(
                m,
                HandshakeType::CertificateVerify,
                HandshakePayload::CertificateVerify
            )<span class="question-mark">?</span>;

            <span class="kw">match </span><span class="self">self</span>.transcript.take_handshake_buf() {
                <span class="prelude-val">Some</span>(msgs) =&gt; {
                    <span class="kw">let </span>certs = <span class="kw-2">&amp;</span><span class="self">self</span>.client_cert;
                    <span class="self">self</span>.config
                        .verifier
                        .verify_tls12_signature(<span class="kw-2">&amp;</span>msgs, <span class="kw-2">&amp;</span>certs[<span class="number">0</span>], sig)
                }
                <span class="prelude-val">None </span>=&gt; {
                    <span class="comment">// This should be unreachable; the handshake buffer was initialized with
                    // client authentication if the verifier wants to offer it.
                    // `transcript.abandon_client_auth()` can extract it, but its only caller in
                    // this flow will also set `ExpectClientKx::client_cert` to `None`, making it
                    // impossible to reach this state.
                    </span><span class="kw">return </span><span class="prelude-val">Err</span>(cx.common.send_fatal_alert(
                        AlertDescription::AccessDenied,
                        Error::General(<span class="string">"client authentication not set up"</span>.into()),
                    ));
                }
            }
        };

        <span class="kw">if let </span><span class="prelude-val">Err</span>(e) = rc {
            <span class="kw">return </span><span class="prelude-val">Err</span>(cx
                .common
                .send_cert_verify_error_alert(e));
        }

        <span class="macro">trace!</span>(<span class="string">"client CertificateVerify OK"</span>);
        cx.common.peer_certificates = <span class="prelude-val">Some</span>(<span class="self">self</span>.client_cert);

        <span class="self">self</span>.transcript.add_message(<span class="kw-2">&amp;</span>m);
        <span class="prelude-val">Ok</span>(Box::new(ExpectCcs {
            config: <span class="self">self</span>.config,
            secrets: <span class="self">self</span>.secrets,
            transcript: <span class="self">self</span>.transcript,
            session_id: <span class="self">self</span>.session_id,
            using_ems: <span class="self">self</span>.using_ems,
            resuming: <span class="bool-val">false</span>,
            send_ticket: <span class="self">self</span>.send_ticket,
        }))
    }
}

<span class="comment">// --- Process client's ChangeCipherSpec ---
</span><span class="kw">struct </span>ExpectCcs {
    config: Arc&lt;ServerConfig&gt;,
    secrets: ConnectionSecrets,
    transcript: HandshakeHash,
    session_id: SessionId,
    using_ems: bool,
    resuming: bool,
    send_ticket: bool,
}

<span class="kw">impl </span>State&lt;ServerConnectionData&gt; <span class="kw">for </span>ExpectCcs {
    <span class="kw">fn </span>handle(<span class="self">self</span>: Box&lt;<span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;, m: Message) -&gt; hs::NextStateOrError {
        <span class="kw">match </span>m.payload {
            MessagePayload::ChangeCipherSpec(..) =&gt; {}
            payload =&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(inappropriate_message(
                    <span class="kw-2">&amp;</span>payload,
                    <span class="kw-2">&amp;</span>[ContentType::ChangeCipherSpec],
                ))
            }
        }

        <span class="comment">// CCS should not be received interleaved with fragmented handshake-level
        // message.
        </span>cx.common.check_aligned_handshake()<span class="question-mark">?</span>;

        cx.common
            .record_layer
            .start_decrypting();
        <span class="prelude-val">Ok</span>(Box::new(ExpectFinished {
            config: <span class="self">self</span>.config,
            secrets: <span class="self">self</span>.secrets,
            transcript: <span class="self">self</span>.transcript,
            session_id: <span class="self">self</span>.session_id,
            using_ems: <span class="self">self</span>.using_ems,
            resuming: <span class="self">self</span>.resuming,
            send_ticket: <span class="self">self</span>.send_ticket,
        }))
    }
}

<span class="comment">// --- Process client's Finished ---
</span><span class="kw">fn </span>get_server_connection_value_tls12(
    secrets: <span class="kw-2">&amp;</span>ConnectionSecrets,
    using_ems: bool,
    cx: <span class="kw-2">&amp;</span>ServerContext&lt;<span class="lifetime">'_</span>&gt;,
    time_now: ticketer::TimeBase,
) -&gt; persist::ServerSessionValue {
    <span class="kw">let </span>version = ProtocolVersion::TLSv1_2;
    <span class="kw">let </span>secret = secrets.get_master_secret();

    <span class="kw">let </span><span class="kw-2">mut </span>v = persist::ServerSessionValue::new(
        cx.data.sni.as_ref(),
        version,
        secrets.suite().common.suite,
        secret,
        cx.common.peer_certificates.clone(),
        cx.common.alpn_protocol.clone(),
        cx.data.resumption_data.clone(),
        time_now,
        <span class="number">0</span>,
    );

    <span class="kw">if </span>using_ems {
        v.set_extended_ms_used();
    }

    v
}

<span class="kw">fn </span>emit_ticket(
    secrets: <span class="kw-2">&amp;</span>ConnectionSecrets,
    transcript: <span class="kw-2">&amp;mut </span>HandshakeHash,
    using_ems: bool,
    cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;,
    ticketer: <span class="kw-2">&amp;</span><span class="kw">dyn </span>ProducesTickets,
) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
    <span class="kw">let </span>time_now = ticketer::TimeBase::now()<span class="question-mark">?</span>;
    <span class="kw">let </span>plain = get_server_connection_value_tls12(secrets, using_ems, cx, time_now).get_encoding();

    <span class="comment">// If we can't produce a ticket for some reason, we can't
    // report an error. Send an empty one.
    </span><span class="kw">let </span>ticket = ticketer
        .encrypt(<span class="kw-2">&amp;</span>plain)
        .unwrap_or_default();
    <span class="kw">let </span>ticket_lifetime = ticketer.lifetime();

    <span class="kw">let </span>m = Message {
        version: ProtocolVersion::TLSv1_2,
        payload: MessagePayload::handshake(HandshakeMessagePayload {
            typ: HandshakeType::NewSessionTicket,
            payload: HandshakePayload::NewSessionTicket(NewSessionTicketPayload::new(
                ticket_lifetime,
                ticket,
            )),
        }),
    };

    transcript.add_message(<span class="kw-2">&amp;</span>m);
    cx.common.send_msg(m, <span class="bool-val">false</span>);
    <span class="prelude-val">Ok</span>(())
}

<span class="kw">fn </span>emit_ccs(common: <span class="kw-2">&amp;mut </span>CommonState) {
    <span class="kw">let </span>m = Message {
        version: ProtocolVersion::TLSv1_2,
        payload: MessagePayload::ChangeCipherSpec(ChangeCipherSpecPayload {}),
    };

    common.send_msg(m, <span class="bool-val">false</span>);
}

<span class="kw">fn </span>emit_finished(
    secrets: <span class="kw-2">&amp;</span>ConnectionSecrets,
    transcript: <span class="kw-2">&amp;mut </span>HandshakeHash,
    common: <span class="kw-2">&amp;mut </span>CommonState,
) {
    <span class="kw">let </span>vh = transcript.get_current_hash();
    <span class="kw">let </span>verify_data = secrets.server_verify_data(<span class="kw-2">&amp;</span>vh);
    <span class="kw">let </span>verify_data_payload = Payload::new(verify_data);

    <span class="kw">let </span>f = Message {
        version: ProtocolVersion::TLSv1_2,
        payload: MessagePayload::handshake(HandshakeMessagePayload {
            typ: HandshakeType::Finished,
            payload: HandshakePayload::Finished(verify_data_payload),
        }),
    };

    transcript.add_message(<span class="kw-2">&amp;</span>f);
    common.send_msg(f, <span class="bool-val">true</span>);
}

<span class="kw">struct </span>ExpectFinished {
    config: Arc&lt;ServerConfig&gt;,
    secrets: ConnectionSecrets,
    transcript: HandshakeHash,
    session_id: SessionId,
    using_ems: bool,
    resuming: bool,
    send_ticket: bool,
}

<span class="kw">impl </span>State&lt;ServerConnectionData&gt; <span class="kw">for </span>ExpectFinished {
    <span class="kw">fn </span>handle(<span class="kw-2">mut </span><span class="self">self</span>: Box&lt;<span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;, m: Message) -&gt; hs::NextStateOrError {
        <span class="kw">let </span>finished =
            <span class="macro">require_handshake_msg!</span>(m, HandshakeType::Finished, HandshakePayload::Finished)<span class="question-mark">?</span>;

        cx.common.check_aligned_handshake()<span class="question-mark">?</span>;

        <span class="kw">let </span>vh = <span class="self">self</span>.transcript.get_current_hash();
        <span class="kw">let </span>expect_verify_data = <span class="self">self</span>.secrets.client_verify_data(<span class="kw-2">&amp;</span>vh);

        <span class="kw">let </span>_fin_verified =
            constant_time::verify_slices_are_equal(<span class="kw-2">&amp;</span>expect_verify_data, <span class="kw-2">&amp;</span>finished.<span class="number">0</span>)
                .map_err(|<span class="kw">_</span>| {
                    cx.common
                        .send_fatal_alert(AlertDescription::DecryptError, Error::DecryptError)
                })
                .map(|<span class="kw">_</span>| verify::FinishedMessageVerified::assertion())<span class="question-mark">?</span>;

        <span class="comment">// Save connection, perhaps
        </span><span class="kw">if </span>!<span class="self">self</span>.resuming &amp;&amp; !<span class="self">self</span>.session_id.is_empty() {
            <span class="kw">let </span>time_now = ticketer::TimeBase::now()<span class="question-mark">?</span>;
            <span class="kw">let </span>value =
                get_server_connection_value_tls12(<span class="kw-2">&amp;</span><span class="self">self</span>.secrets, <span class="self">self</span>.using_ems, cx, time_now);

            <span class="kw">let </span>worked = <span class="self">self
                </span>.config
                .session_storage
                .put(<span class="self">self</span>.session_id.get_encoding(), value.get_encoding());
            <span class="kw">if </span>worked {
                <span class="macro">debug!</span>(<span class="string">"Session saved"</span>);
            } <span class="kw">else </span>{
                <span class="macro">debug!</span>(<span class="string">"Session not saved"</span>);
            }
        }

        <span class="comment">// Send our CCS and Finished.
        </span><span class="self">self</span>.transcript.add_message(<span class="kw-2">&amp;</span>m);
        <span class="kw">if </span>!<span class="self">self</span>.resuming {
            <span class="kw">if </span><span class="self">self</span>.send_ticket {
                emit_ticket(
                    <span class="kw-2">&amp;</span><span class="self">self</span>.secrets,
                    <span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript,
                    <span class="self">self</span>.using_ems,
                    cx,
                    <span class="kw-2">&amp;*</span><span class="self">self</span>.config.ticketer,
                )<span class="question-mark">?</span>;
            }
            emit_ccs(cx.common);
            cx.common
                .record_layer
                .start_encrypting();
            emit_finished(<span class="kw-2">&amp;</span><span class="self">self</span>.secrets, <span class="kw-2">&amp;mut </span><span class="self">self</span>.transcript, cx.common);
        }

        cx.common.start_traffic();
        <span class="prelude-val">Ok</span>(Box::new(ExpectTraffic {
            secrets: <span class="self">self</span>.secrets,
            _fin_verified,
        }))
    }
}

<span class="comment">// --- Process traffic ---
</span><span class="kw">struct </span>ExpectTraffic {
    secrets: ConnectionSecrets,
    _fin_verified: verify::FinishedMessageVerified,
}

<span class="kw">impl </span>ExpectTraffic {}

<span class="kw">impl </span>State&lt;ServerConnectionData&gt; <span class="kw">for </span>ExpectTraffic {
    <span class="kw">fn </span>handle(<span class="self">self</span>: Box&lt;<span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>ServerContext&lt;<span class="lifetime">'_</span>&gt;, m: Message) -&gt; hs::NextStateOrError {
        <span class="kw">match </span>m.payload {
            MessagePayload::ApplicationData(payload) =&gt; cx
                .common
                .take_received_plaintext(payload),
            payload =&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(inappropriate_message(
                    <span class="kw-2">&amp;</span>payload,
                    <span class="kw-2">&amp;</span>[ContentType::ApplicationData],
                ));
            }
        }
        <span class="prelude-val">Ok</span>(<span class="self">self</span>)
    }

    <span class="kw">fn </span>export_keying_material(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        output: <span class="kw-2">&amp;mut </span>[u8],
        label: <span class="kw-2">&amp;</span>[u8],
        context: <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>[u8]&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="self">self</span>.secrets
            .export_keying_material(output, label, context);
        <span class="prelude-val">Ok</span>(())
    }

    <span class="attr">#[cfg(feature = <span class="string">"secret_extraction"</span>)]
    </span><span class="kw">fn </span>extract_secrets(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;PartiallyExtractedSecrets, Error&gt; {
        <span class="self">self</span>.secrets
            .extract_secrets(Side::Server)
    }
}
</code></pre></div></section></main></body></html>