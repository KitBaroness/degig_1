<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mio-0.8.11/src/poll.rs`."><title>poll.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="mio" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../mio/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="attr">#[cfg(all(
    unix,
    not(mio_unsupported_force_poll_poll),
    not(any(target_os = <span class="string">"solaris"</span>, target_os = <span class="string">"vita"</span>))
))]
</span><span class="kw">use </span>std::os::unix::io::{AsRawFd, RawFd};
<span class="attr">#[cfg(all(debug_assertions, not(target_os = <span class="string">"wasi"</span>)))]
</span><span class="kw">use </span>std::sync::atomic::{AtomicBool, Ordering};
<span class="attr">#[cfg(all(debug_assertions, not(target_os = <span class="string">"wasi"</span>)))]
</span><span class="kw">use </span>std::sync::Arc;
<span class="kw">use </span>std::time::Duration;
<span class="kw">use </span>std::{fmt, io};

<span class="kw">use crate</span>::{event, sys, Events, Interest, Token};

<span class="doccomment">/// Polls for readiness events on all registered values.
///
/// `Poll` allows a program to monitor a large number of [`event::Source`]s,
/// waiting until one or more become "ready" for some class of operations; e.g.
/// reading and writing. An event source is considered ready if it is possible
/// to immediately perform a corresponding operation; e.g. [`read`] or
/// [`write`].
///
/// To use `Poll`, an `event::Source` must first be registered with the `Poll`
/// instance using the [`register`] method on its associated `Register`,
/// supplying readiness interest. The readiness interest tells `Poll` which
/// specific operations on the handle to monitor for readiness. A `Token` is
/// also passed to the [`register`] function. When `Poll` returns a readiness
/// event, it will include this token.  This associates the event with the
/// event source that generated the event.
///
/// [`event::Source`]: ./event/trait.Source.html
/// [`read`]: ./net/struct.TcpStream.html#method.read
/// [`write`]: ./net/struct.TcpStream.html#method.write
/// [`register`]: struct.Registry.html#method.register
///
/// # Examples
///
/// A basic example -- establishing a `TcpStream` connection.
///
</span><span class="attr">#[cfg_attr(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>), doc = <span class="string">"```"</span>)]
#[cfg_attr(not(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>)), doc = <span class="string">"```ignore"</span>)]
</span><span class="doccomment">/// # use std::error::Error;
/// # fn main() -&gt; Result&lt;(), Box&lt;dyn Error&gt;&gt; {
/// use mio::{Events, Poll, Interest, Token};
/// use mio::net::TcpStream;
///
/// use std::net::{self, SocketAddr};
///
/// // Bind a server socket to connect to.
/// let addr: SocketAddr = "127.0.0.1:0".parse()?;
/// let server = net::TcpListener::bind(addr)?;
///
/// // Construct a new `Poll` handle as well as the `Events` we'll store into
/// let mut poll = Poll::new()?;
/// let mut events = Events::with_capacity(1024);
///
/// // Connect the stream
/// let mut stream = TcpStream::connect(server.local_addr()?)?;
///
/// // Register the stream with `Poll`
/// poll.registry().register(&amp;mut stream, Token(0), Interest::READABLE | Interest::WRITABLE)?;
///
/// // Wait for the socket to become ready. This has to happens in a loop to
/// // handle spurious wakeups.
/// loop {
///     poll.poll(&amp;mut events, None)?;
///
///     for event in &amp;events {
///         if event.token() == Token(0) &amp;&amp; event.is_writable() {
///             // The socket connected (probably, it could still be a spurious
///             // wakeup)
///             return Ok(());
///         }
///     }
/// }
/// # }
/// ```
///
/// # Portability
///
/// Using `Poll` provides a portable interface across supported platforms as
/// long as the caller takes the following into consideration:
///
/// ### Spurious events
///
/// [`Poll::poll`] may return readiness events even if the associated
/// event source is not actually ready. Given the same code, this may
/// happen more on some platforms than others. It is important to never assume
/// that, just because a readiness event was received, that the associated
/// operation will succeed as well.
///
/// If operation fails with [`WouldBlock`], then the caller should not treat
/// this as an error, but instead should wait until another readiness event is
/// received.
///
/// ### Draining readiness
///
/// Once a readiness event is received, the corresponding operation must be
/// performed repeatedly until it returns [`WouldBlock`]. Unless this is done,
/// there is no guarantee that another readiness event will be delivered, even
/// if further data is received for the event source.
///
/// [`WouldBlock`]: std::io::ErrorKind::WouldBlock
///
/// ### Readiness operations
///
/// The only readiness operations that are guaranteed to be present on all
/// supported platforms are [`readable`] and [`writable`]. All other readiness
/// operations may have false negatives and as such should be considered
/// **hints**. This means that if a socket is registered with [`readable`]
/// interest and either an error or close is received, a readiness event will
/// be generated for the socket, but it **may** only include `readable`
/// readiness. Also note that, given the potential for spurious events,
/// receiving a readiness event with `read_closed`, `write_closed`, or `error`
/// doesn't actually mean that a `read` on the socket will return a result
/// matching the readiness event.
///
/// In other words, portable programs that explicitly check for [`read_closed`],
/// [`write_closed`], or [`error`] readiness should be doing so as an
/// **optimization** and always be able to handle an error or close situation
/// when performing the actual read operation.
///
/// [`readable`]: ./event/struct.Event.html#method.is_readable
/// [`writable`]: ./event/struct.Event.html#method.is_writable
/// [`error`]: ./event/struct.Event.html#method.is_error
/// [`read_closed`]: ./event/struct.Event.html#method.is_read_closed
/// [`write_closed`]: ./event/struct.Event.html#method.is_write_closed
///
/// ### Registering handles
///
/// Unless otherwise noted, it should be assumed that types implementing
/// [`event::Source`] will never become ready unless they are registered with
/// `Poll`.
///
/// For example:
///
</span><span class="attr">#[cfg_attr(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>), doc = <span class="string">"```"</span>)]
#[cfg_attr(not(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>)), doc = <span class="string">"```ignore"</span>)]
</span><span class="doccomment">/// # use std::error::Error;
/// # use std::net;
/// # fn main() -&gt; Result&lt;(), Box&lt;dyn Error&gt;&gt; {
/// use mio::{Poll, Interest, Token};
/// use mio::net::TcpStream;
/// use std::net::SocketAddr;
/// use std::time::Duration;
/// use std::thread;
///
/// let address: SocketAddr = "127.0.0.1:0".parse()?;
/// let listener = net::TcpListener::bind(address)?;
/// let mut sock = TcpStream::connect(listener.local_addr()?)?;
///
/// thread::sleep(Duration::from_secs(1));
///
/// let poll = Poll::new()?;
///
/// // The connect is not guaranteed to have started until it is registered at
/// // this point
/// poll.registry().register(&amp;mut sock, Token(0), Interest::READABLE | Interest::WRITABLE)?;
/// #     Ok(())
/// # }
/// ```
///
/// ### Dropping `Poll`
///
/// When the `Poll` instance is dropped it may cancel in-flight operations for
/// the registered [event sources], meaning that no further events for them may
/// be received. It also means operations on the registered event sources may no
/// longer work. It is up to the user to keep the `Poll` instance alive while
/// registered event sources are being used.
///
/// [event sources]: ./event/trait.Source.html
///
/// ### Accessing raw fd/socket/handle
///
/// Mio makes it possible for many types to be converted into a raw file
/// descriptor (fd, Unix), socket (Windows) or handle (Windows). This makes it
/// possible to support more operations on the type than Mio supports, for
/// example it makes [mio-aio] possible. However accessing the raw fd is not
/// without it's pitfalls.
///
/// Specifically performing I/O operations outside of Mio on these types (via
/// the raw fd) has unspecified behaviour. It could cause no more events to be
/// generated for the type even though it returned `WouldBlock` (in an operation
/// directly accessing the fd). The behaviour is OS specific and Mio can only
/// guarantee cross-platform behaviour if it can control the I/O.
///
/// [mio-aio]: https://github.com/asomers/mio-aio
///
/// *The following is **not** guaranteed, just a description of the current
/// situation!* Mio is allowed to change the following without it being considered
/// a breaking change, don't depend on this, it's just here to inform the user.
/// Currently the kqueue and epoll implementation support direct I/O operations
/// on the fd without Mio's knowledge. Windows however needs **all** I/O
/// operations to go through Mio otherwise it is not able to update it's
/// internal state properly and won't generate events.
///
/// ### Polling without registering event sources
///
///
/// *The following is **not** guaranteed, just a description of the current
/// situation!* Mio is allowed to change the following without it being
/// considered a breaking change, don't depend on this, it's just here to inform
/// the user. On platforms that use epoll, kqueue or IOCP (see implementation
/// notes below) polling without previously registering [event sources] will
/// result in sleeping forever, only a process signal will be able to wake up
/// the thread.
///
/// On WASM/WASI this is different as it doesn't support process signals,
/// furthermore the WASI specification doesn't specify a behaviour in this
/// situation, thus it's up to the implementation what to do here. As an
/// example, the wasmtime runtime will return `EINVAL` in this situation, but
/// different runtimes may return different results. If you have further
/// insights or thoughts about this situation (and/or how Mio should handle it)
/// please add you comment to [pull request#1580].
///
/// [event sources]: crate::event::Source
/// [pull request#1580]: https://github.com/tokio-rs/mio/pull/1580
///
/// # Implementation notes
///
/// `Poll` is backed by the selector provided by the operating system.
///
/// |      OS       |  Selector |
/// |---------------|-----------|
/// | Android       | [epoll]   |
/// | DragonFly BSD | [kqueue]  |
/// | FreeBSD       | [kqueue]  |
/// | iOS           | [kqueue]  |
/// | illumos       | [epoll]   |
/// | Linux         | [epoll]   |
/// | NetBSD        | [kqueue]  |
/// | OpenBSD       | [kqueue]  |
/// | Windows       | [IOCP]    |
/// | macOS         | [kqueue]  |
///
/// On all supported platforms, socket operations are handled by using the
/// system selector. Platform specific extensions (e.g. [`SourceFd`]) allow
/// accessing other features provided by individual system selectors. For
/// example, Linux's [`signalfd`] feature can be used by registering the FD with
/// `Poll` via [`SourceFd`].
///
/// On all platforms except windows, a call to [`Poll::poll`] is mostly just a
/// direct call to the system selector. However, [IOCP] uses a completion model
/// instead of a readiness model. In this case, `Poll` must adapt the completion
/// model Mio's API. While non-trivial, the bridge layer is still quite
/// efficient. The most expensive part being calls to `read` and `write` require
/// data to be copied into an intermediate buffer before it is passed to the
/// kernel.
///
/// [epoll]: https://man7.org/linux/man-pages/man7/epoll.7.html
/// [kqueue]: https://www.freebsd.org/cgi/man.cgi?query=kqueue&amp;sektion=2
/// [IOCP]: https://docs.microsoft.com/en-us/windows/win32/fileio/i-o-completion-ports
/// [`signalfd`]: https://man7.org/linux/man-pages/man2/signalfd.2.html
/// [`SourceFd`]: unix/struct.SourceFd.html
/// [`Poll::poll`]: struct.Poll.html#method.poll
</span><span class="kw">pub struct </span>Poll {
    registry: Registry,
}

<span class="doccomment">/// Registers I/O resources.
</span><span class="kw">pub struct </span>Registry {
    selector: sys::Selector,
    <span class="doccomment">/// Whether this selector currently has an associated waker.
    </span><span class="attr">#[cfg(all(debug_assertions, not(target_os = <span class="string">"wasi"</span>)))]
    </span>has_waker: Arc&lt;AtomicBool&gt;,
}

<span class="kw">impl </span>Poll {
    <span class="macro">cfg_os_poll!</span> {
        <span class="doccomment">/// Return a new `Poll` handle.
        ///
        /// This function will make a syscall to the operating system to create
        /// the system selector. If this syscall fails, `Poll::new` will return
        /// with the error.
        ///
        /// close-on-exec flag is set on the file descriptors used by the selector to prevent
        /// leaking it to executed processes. However, on some systems such as
        /// old Linux systems that don't support `epoll_create1` syscall it is done
        /// non-atomically, so a separate thread executing in parallel to this
        /// function may accidentally leak the file descriptor if it executes a
        /// new process before this function returns.
        ///
        /// See [struct] level docs for more details.
        ///
        /// [struct]: struct.Poll.html
        ///
        /// # Examples
        ///
        /// ```
        /// # use std::error::Error;
        /// # fn main() -&gt; Result&lt;(), Box&lt;dyn Error&gt;&gt; {
        /// use mio::{Poll, Events};
        /// use std::time::Duration;
        ///
        /// let mut poll = match Poll::new() {
        ///     Ok(poll) =&gt; poll,
        ///     Err(e) =&gt; panic!("failed to create Poll instance; err={:?}", e),
        /// };
        ///
        /// // Create a structure to receive polled events
        /// let mut events = Events::with_capacity(1024);
        ///
        /// // Wait for events, but none will be received because no
        /// // `event::Source`s have been registered with this `Poll` instance.
        /// poll.poll(&amp;mut events, Some(Duration::from_millis(500)))?;
        /// assert!(events.is_empty());
        /// #     Ok(())
        /// # }
        /// ```
        </span><span class="kw">pub fn </span>new() -&gt; io::Result&lt;Poll&gt; {
            sys::Selector::new().map(|selector| Poll {
                registry: Registry {
                    selector,
                    <span class="attr">#[cfg(all(debug_assertions, not(target_os = <span class="string">"wasi"</span>)))]
                    </span>has_waker: Arc::new(AtomicBool::new(<span class="bool-val">false</span>)),
                },
            })
        }
    }

    <span class="doccomment">/// Create a separate `Registry` which can be used to register
    /// `event::Source`s.
    </span><span class="kw">pub fn </span>registry(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>Registry {
        <span class="kw-2">&amp;</span><span class="self">self</span>.registry
    }

    <span class="doccomment">/// Wait for readiness events
    ///
    /// Blocks the current thread and waits for readiness events for any of the
    /// [`event::Source`]s that have been registered with this `Poll` instance.
    /// The function will block until either at least one readiness event has
    /// been received or `timeout` has elapsed. A `timeout` of `None` means that
    /// `poll` will block until a readiness event has been received.
    ///
    /// The supplied `events` will be cleared and newly received readiness events
    /// will be pushed onto the end. At most `events.capacity()` events will be
    /// returned. If there are further pending readiness events, they will be
    /// returned on the next call to `poll`.
    ///
    /// A single call to `poll` may result in multiple readiness events being
    /// returned for a single event source. For example, if a TCP socket becomes
    /// both readable and writable, it may be possible for a single readiness
    /// event to be returned with both [`readable`] and [`writable`] readiness
    /// **OR** two separate events may be returned, one with [`readable`] set
    /// and one with [`writable`] set.
    ///
    /// Note that the `timeout` will be rounded up to the system clock
    /// granularity (usually 1ms), and kernel scheduling delays mean that
    /// the blocking interval may be overrun by a small amount.
    ///
    /// See the [struct] level documentation for a higher level discussion of
    /// polling.
    ///
    /// [`event::Source`]: ./event/trait.Source.html
    /// [`readable`]: struct.Interest.html#associatedconstant.READABLE
    /// [`writable`]: struct.Interest.html#associatedconstant.WRITABLE
    /// [struct]: struct.Poll.html
    /// [`iter`]: ./event/struct.Events.html#method.iter
    ///
    /// # Notes
    ///
    /// This returns any errors without attempting to retry, previous versions
    /// of Mio would automatically retry the poll call if it was interrupted
    /// (if `EINTR` was returned).
    ///
    /// Currently if the `timeout` elapses without any readiness events
    /// triggering this will return `Ok(())`. However we're not guaranteeing
    /// this behaviour as this depends on the OS.
    ///
    /// # Examples
    ///
    /// A basic example -- establishing a `TcpStream` connection.
    ///
    </span><span class="attr">#[cfg_attr(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>), doc = <span class="string">"```"</span>)]
    #[cfg_attr(not(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>)), doc = <span class="string">"```ignore"</span>)]
    </span><span class="doccomment">/// # use std::error::Error;
    /// # fn main() -&gt; Result&lt;(), Box&lt;dyn Error&gt;&gt; {
    /// use mio::{Events, Poll, Interest, Token};
    /// use mio::net::TcpStream;
    ///
    /// use std::net::{TcpListener, SocketAddr};
    /// use std::thread;
    ///
    /// // Bind a server socket to connect to.
    /// let addr: SocketAddr = "127.0.0.1:0".parse()?;
    /// let server = TcpListener::bind(addr)?;
    /// let addr = server.local_addr()?.clone();
    ///
    /// // Spawn a thread to accept the socket
    /// thread::spawn(move || {
    ///     let _ = server.accept();
    /// });
    ///
    /// // Construct a new `Poll` handle as well as the `Events` we'll store into
    /// let mut poll = Poll::new()?;
    /// let mut events = Events::with_capacity(1024);
    ///
    /// // Connect the stream
    /// let mut stream = TcpStream::connect(addr)?;
    ///
    /// // Register the stream with `Poll`
    /// poll.registry().register(
    ///     &amp;mut stream,
    ///     Token(0),
    ///     Interest::READABLE | Interest::WRITABLE)?;
    ///
    /// // Wait for the socket to become ready. This has to happens in a loop to
    /// // handle spurious wakeups.
    /// loop {
    ///     poll.poll(&amp;mut events, None)?;
    ///
    ///     for event in &amp;events {
    ///         if event.token() == Token(0) &amp;&amp; event.is_writable() {
    ///             // The socket connected (probably, it could still be a spurious
    ///             // wakeup)
    ///             return Ok(());
    ///         }
    ///     }
    /// }
    /// # }
    /// ```
    ///
    /// [struct]: #
    </span><span class="kw">pub fn </span>poll(<span class="kw-2">&amp;mut </span><span class="self">self</span>, events: <span class="kw-2">&amp;mut </span>Events, timeout: <span class="prelude-ty">Option</span>&lt;Duration&gt;) -&gt; io::Result&lt;()&gt; {
        <span class="self">self</span>.registry.selector.select(events.sys(), timeout)
    }
}

<span class="attr">#[cfg(all(
    unix,
    not(mio_unsupported_force_poll_poll),
    not(any(target_os = <span class="string">"solaris"</span>, target_os = <span class="string">"vita"</span>))
))]
</span><span class="kw">impl </span>AsRawFd <span class="kw">for </span>Poll {
    <span class="kw">fn </span>as_raw_fd(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; RawFd {
        <span class="self">self</span>.registry.as_raw_fd()
    }
}

<span class="kw">impl </span>fmt::Debug <span class="kw">for </span>Poll {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, fmt: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        fmt.debug_struct(<span class="string">"Poll"</span>).finish()
    }
}

<span class="kw">impl </span>Registry {
    <span class="doccomment">/// Register an [`event::Source`] with the `Poll` instance.
    ///
    /// Once registered, the `Poll` instance will monitor the event source for
    /// readiness state changes. When it notices a state change, it will return
    /// a readiness event for the handle the next time [`poll`] is called.
    ///
    /// See [`Poll`] docs for a high level overview.
    ///
    /// # Arguments
    ///
    /// `source: &amp;mut S: event::Source`: This is the source of events that the
    /// `Poll` instance should monitor for readiness state changes.
    ///
    /// `token: Token`: The caller picks a token to associate with the socket.
    /// When [`poll`] returns an event for the handle, this token is included.
    /// This allows the caller to map the event to its source. The token
    /// associated with the `event::Source` can be changed at any time by
    /// calling [`reregister`].
    ///
    /// See documentation on [`Token`] for an example showing how to pick
    /// [`Token`] values.
    ///
    /// `interest: Interest`: Specifies which operations `Poll` should monitor
    /// for readiness. `Poll` will only return readiness events for operations
    /// specified by this argument.
    ///
    /// If a socket is registered with readable interest and the socket becomes
    /// writable, no event will be returned from [`poll`].
    ///
    /// The readiness interest for an `event::Source` can be changed at any time
    /// by calling [`reregister`].
    ///
    /// # Notes
    ///
    /// Callers must ensure that if a source being registered with a `Poll`
    /// instance was previously registered with that `Poll` instance, then a
    /// call to [`deregister`] has already occurred. Consecutive calls to
    /// `register` is unspecified behavior.
    ///
    /// Unless otherwise specified, the caller should assume that once an event
    /// source is registered with a `Poll` instance, it is bound to that `Poll`
    /// instance for the lifetime of the event source. This remains true even
    /// if the event source is deregistered from the poll instance using
    /// [`deregister`].
    ///
    /// [`event::Source`]: ./event/trait.Source.html
    /// [`poll`]: struct.Poll.html#method.poll
    /// [`reregister`]: struct.Registry.html#method.reregister
    /// [`deregister`]: struct.Registry.html#method.deregister
    /// [`Token`]: struct.Token.html
    ///
    /// # Examples
    ///
    </span><span class="attr">#[cfg_attr(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>), doc = <span class="string">"```"</span>)]
    #[cfg_attr(not(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>)), doc = <span class="string">"```ignore"</span>)]
    </span><span class="doccomment">/// # use std::error::Error;
    /// # use std::net;
    /// # fn main() -&gt; Result&lt;(), Box&lt;dyn Error&gt;&gt; {
    /// use mio::{Events, Poll, Interest, Token};
    /// use mio::net::TcpStream;
    /// use std::net::SocketAddr;
    /// use std::time::{Duration, Instant};
    ///
    /// let mut poll = Poll::new()?;
    ///
    /// let address: SocketAddr = "127.0.0.1:0".parse()?;
    /// let listener = net::TcpListener::bind(address)?;
    /// let mut socket = TcpStream::connect(listener.local_addr()?)?;
    ///
    /// // Register the socket with `poll`
    /// poll.registry().register(
    ///     &amp;mut socket,
    ///     Token(0),
    ///     Interest::READABLE | Interest::WRITABLE)?;
    ///
    /// let mut events = Events::with_capacity(1024);
    /// let start = Instant::now();
    /// let timeout = Duration::from_millis(500);
    ///
    /// loop {
    ///     let elapsed = start.elapsed();
    ///
    ///     if elapsed &gt;= timeout {
    ///         // Connection timed out
    ///         return Ok(());
    ///     }
    ///
    ///     let remaining = timeout - elapsed;
    ///     poll.poll(&amp;mut events, Some(remaining))?;
    ///
    ///     for event in &amp;events {
    ///         if event.token() == Token(0) {
    ///             // Something (probably) happened on the socket.
    ///             return Ok(());
    ///         }
    ///     }
    /// }
    /// # }
    /// ```
    </span><span class="kw">pub fn </span>register&lt;S&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, source: <span class="kw-2">&amp;mut </span>S, token: Token, interests: Interest) -&gt; io::Result&lt;()&gt;
    <span class="kw">where
        </span>S: event::Source + <span class="question-mark">?</span>Sized,
    {
        <span class="macro">trace!</span>(
            <span class="string">"registering event source with poller: token={:?}, interests={:?}"</span>,
            token,
            interests
        );
        source.register(<span class="self">self</span>, token, interests)
    }

    <span class="doccomment">/// Re-register an [`event::Source`] with the `Poll` instance.
    ///
    /// Re-registering an event source allows changing the details of the
    /// registration. Specifically, it allows updating the associated `token`
    /// and `interests` specified in previous `register` and `reregister` calls.
    ///
    /// The `reregister` arguments fully override the previous values. In other
    /// words, if a socket is registered with [`readable`] interest and the call
    /// to `reregister` specifies [`writable`], then read interest is no longer
    /// requested for the handle.
    ///
    /// The event source must have previously been registered with this instance
    /// of `Poll`, otherwise the behavior is unspecified.
    ///
    /// See the [`register`] documentation for details about the function
    /// arguments and see the [`struct`] docs for a high level overview of
    /// polling.
    ///
    /// # Examples
    ///
    </span><span class="attr">#[cfg_attr(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>), doc = <span class="string">"```"</span>)]
    #[cfg_attr(not(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>)), doc = <span class="string">"```ignore"</span>)]
    </span><span class="doccomment">/// # use std::error::Error;
    /// # use std::net;
    /// # fn main() -&gt; Result&lt;(), Box&lt;dyn Error&gt;&gt; {
    /// use mio::{Poll, Interest, Token};
    /// use mio::net::TcpStream;
    /// use std::net::SocketAddr;
    ///
    /// let poll = Poll::new()?;
    ///
    /// let address: SocketAddr = "127.0.0.1:0".parse()?;
    /// let listener = net::TcpListener::bind(address)?;
    /// let mut socket = TcpStream::connect(listener.local_addr()?)?;
    ///
    /// // Register the socket with `poll`, requesting readable
    /// poll.registry().register(
    ///     &amp;mut socket,
    ///     Token(0),
    ///     Interest::READABLE)?;
    ///
    /// // Reregister the socket specifying write interest instead. Even though
    /// // the token is the same it must be specified.
    /// poll.registry().reregister(
    ///     &amp;mut socket,
    ///     Token(0),
    ///     Interest::WRITABLE)?;
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`event::Source`]: ./event/trait.Source.html
    /// [`struct`]: struct.Poll.html
    /// [`register`]: struct.Registry.html#method.register
    /// [`readable`]: ./event/struct.Event.html#is_readable
    /// [`writable`]: ./event/struct.Event.html#is_writable
    </span><span class="kw">pub fn </span>reregister&lt;S&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, source: <span class="kw-2">&amp;mut </span>S, token: Token, interests: Interest) -&gt; io::Result&lt;()&gt;
    <span class="kw">where
        </span>S: event::Source + <span class="question-mark">?</span>Sized,
    {
        <span class="macro">trace!</span>(
            <span class="string">"reregistering event source with poller: token={:?}, interests={:?}"</span>,
            token,
            interests
        );
        source.reregister(<span class="self">self</span>, token, interests)
    }

    <span class="doccomment">/// Deregister an [`event::Source`] with the `Poll` instance.
    ///
    /// When an event source is deregistered, the `Poll` instance will no longer
    /// monitor it for readiness state changes. Deregistering clears up any
    /// internal resources needed to track the handle.  After an explicit call
    /// to this method completes, it is guaranteed that the token previously
    /// registered to this handle will not be returned by a future poll, so long
    /// as a happens-before relationship is established between this call and
    /// the poll.
    ///
    /// The event source must have previously been registered with this instance
    /// of `Poll`, otherwise the behavior is unspecified.
    ///
    /// A handle can be passed back to `register` after it has been
    /// deregistered; however, it must be passed back to the **same** `Poll`
    /// instance, otherwise the behavior is unspecified.
    ///
    /// # Examples
    ///
    </span><span class="attr">#[cfg_attr(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>), doc = <span class="string">"```"</span>)]
    #[cfg_attr(not(all(feature = <span class="string">"os-poll"</span>, feature = <span class="string">"net"</span>)), doc = <span class="string">"```ignore"</span>)]
    </span><span class="doccomment">/// # use std::error::Error;
    /// # use std::net;
    /// # fn main() -&gt; Result&lt;(), Box&lt;dyn Error&gt;&gt; {
    /// use mio::{Events, Poll, Interest, Token};
    /// use mio::net::TcpStream;
    /// use std::net::SocketAddr;
    /// use std::time::Duration;
    ///
    /// let mut poll = Poll::new()?;
    ///
    /// let address: SocketAddr = "127.0.0.1:0".parse()?;
    /// let listener = net::TcpListener::bind(address)?;
    /// let mut socket = TcpStream::connect(listener.local_addr()?)?;
    ///
    /// // Register the socket with `poll`
    /// poll.registry().register(
    ///     &amp;mut socket,
    ///     Token(0),
    ///     Interest::READABLE)?;
    ///
    /// poll.registry().deregister(&amp;mut socket)?;
    ///
    /// let mut events = Events::with_capacity(1024);
    ///
    /// // Set a timeout because this poll should never receive any events.
    /// poll.poll(&amp;mut events, Some(Duration::from_secs(1)))?;
    /// assert!(events.is_empty());
    /// #     Ok(())
    /// # }
    /// ```
    </span><span class="kw">pub fn </span>deregister&lt;S&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, source: <span class="kw-2">&amp;mut </span>S) -&gt; io::Result&lt;()&gt;
    <span class="kw">where
        </span>S: event::Source + <span class="question-mark">?</span>Sized,
    {
        <span class="macro">trace!</span>(<span class="string">"deregistering event source from poller"</span>);
        source.deregister(<span class="self">self</span>)
    }

    <span class="doccomment">/// Creates a new independently owned `Registry`.
    ///
    /// Event sources registered with this `Registry` will be registered with
    /// the original `Registry` and `Poll` instance.
    </span><span class="kw">pub fn </span>try_clone(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; io::Result&lt;Registry&gt; {
        <span class="self">self</span>.selector.try_clone().map(|selector| Registry {
            selector,
            <span class="attr">#[cfg(all(debug_assertions, not(target_os = <span class="string">"wasi"</span>)))]
            </span>has_waker: Arc::clone(<span class="kw-2">&amp;</span><span class="self">self</span>.has_waker),
        })
    }

    <span class="doccomment">/// Internal check to ensure only a single `Waker` is active per [`Poll`]
    /// instance.
    </span><span class="attr">#[cfg(all(debug_assertions, not(target_os = <span class="string">"wasi"</span>)))]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>register_waker(<span class="kw-2">&amp;</span><span class="self">self</span>) {
        <span class="macro">assert!</span>(
            !<span class="self">self</span>.has_waker.swap(<span class="bool-val">true</span>, Ordering::AcqRel),
            <span class="string">"Only a single `Waker` can be active per `Poll` instance"
        </span>);
    }

    <span class="doccomment">/// Get access to the `sys::Selector`.
    </span><span class="attr">#[cfg(any(not(target_os = <span class="string">"wasi"</span>), feature = <span class="string">"net"</span>))]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>selector(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>sys::Selector {
        <span class="kw-2">&amp;</span><span class="self">self</span>.selector
    }
}

<span class="kw">impl </span>fmt::Debug <span class="kw">for </span>Registry {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, fmt: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        fmt.debug_struct(<span class="string">"Registry"</span>).finish()
    }
}

<span class="attr">#[cfg(all(
    unix,
    not(mio_unsupported_force_poll_poll),
    not(any(target_os = <span class="string">"solaris"</span>, target_os = <span class="string">"vita"</span>))
))]
</span><span class="kw">impl </span>AsRawFd <span class="kw">for </span>Registry {
    <span class="kw">fn </span>as_raw_fd(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; RawFd {
        <span class="self">self</span>.selector.as_raw_fd()
    }
}

<span class="macro">cfg_os_poll!</span> {
    <span class="attr">#[cfg(all(
        unix,
        not(mio_unsupported_force_poll_poll),
        not(any(target_os = <span class="string">"solaris"</span>, target_os = <span class="string">"vita"</span>)),
    ))]
    #[test]
    </span><span class="kw">pub fn </span>as_raw_fd() {
        <span class="kw">let </span>poll = Poll::new().unwrap();
        <span class="macro">assert!</span>(poll.as_raw_fd() &gt; <span class="number">0</span>);
    }
}
</code></pre></div></section></main></body></html>