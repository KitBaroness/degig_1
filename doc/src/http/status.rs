<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/http-0.2.12/src/status.rs`."><title>status.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="http" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../http/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! HTTP status codes
//!
//! This module contains HTTP-status code related structs an errors. The main
//! type in this module is `StatusCode` which is not intended to be used through
//! this module but rather the `http::StatusCode` type.
//!
//! # Examples
//!
//! ```
//! use http::StatusCode;
//!
//! assert_eq!(StatusCode::from_u16(200).unwrap(), StatusCode::OK);
//! assert_eq!(StatusCode::NOT_FOUND, 404);
//! assert!(StatusCode::OK.is_success());
//! ```

</span><span class="kw">use </span>std::convert::TryFrom;
<span class="kw">use </span>std::num::NonZeroU16;
<span class="kw">use </span>std::error::Error;
<span class="kw">use </span>std::fmt;
<span class="kw">use </span>std::str::FromStr;

<span class="doccomment">/// An HTTP status code (`status-code` in RFC 7230 et al.).
///
/// Constants are provided for known status codes, including those in the IANA
/// [HTTP Status Code Registry](
/// https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml).
///
/// Status code values in the range 100-999 (inclusive) are supported by this
/// type. Values in the range 100-599 are semantically classified by the most
/// significant digit. See [`StatusCode::is_success`], etc. Values above 599
/// are unclassified but allowed for legacy compatibility, though their use is
/// discouraged. Applications may interpret such values as protocol errors.
///
/// # Examples
///
/// ```
/// use http::StatusCode;
///
/// assert_eq!(StatusCode::from_u16(200).unwrap(), StatusCode::OK);
/// assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
/// assert!(StatusCode::OK.is_success());
/// ```
</span><span class="attr">#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
</span><span class="kw">pub struct </span>StatusCode(NonZeroU16);

<span class="doccomment">/// A possible error value when converting a `StatusCode` from a `u16` or `&amp;str`
///
/// This error indicates that the supplied input was not a valid number, was less
/// than 100, or was greater than 999.
</span><span class="kw">pub struct </span>InvalidStatusCode {
    _priv: (),
}

<span class="kw">impl </span>StatusCode {
    <span class="doccomment">/// Converts a u16 to a status code.
    ///
    /// The function validates the correctness of the supplied u16. It must be
    /// greater or equal to 100 and less than 1000.
    ///
    /// # Example
    ///
    /// ```
    /// use http::StatusCode;
    ///
    /// let ok = StatusCode::from_u16(200).unwrap();
    /// assert_eq!(ok, StatusCode::OK);
    ///
    /// let err = StatusCode::from_u16(99);
    /// assert!(err.is_err());
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>from_u16(src: u16) -&gt; <span class="prelude-ty">Result</span>&lt;StatusCode, InvalidStatusCode&gt; {
        <span class="kw">if </span>src &lt; <span class="number">100 </span>|| src &gt;= <span class="number">1000 </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(InvalidStatusCode::new());
        }

        NonZeroU16::new(src)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }

    <span class="doccomment">/// Converts a &amp;[u8] to a status code
    </span><span class="kw">pub fn </span>from_bytes(src: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;StatusCode, InvalidStatusCode&gt; {
        <span class="kw">if </span>src.len() != <span class="number">3 </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(InvalidStatusCode::new());
        }

        <span class="kw">let </span>a = src[<span class="number">0</span>].wrapping_sub(<span class="string">b'0'</span>) <span class="kw">as </span>u16;
        <span class="kw">let </span>b = src[<span class="number">1</span>].wrapping_sub(<span class="string">b'0'</span>) <span class="kw">as </span>u16;
        <span class="kw">let </span>c = src[<span class="number">2</span>].wrapping_sub(<span class="string">b'0'</span>) <span class="kw">as </span>u16;

        <span class="kw">if </span>a == <span class="number">0 </span>|| a &gt; <span class="number">9 </span>|| b &gt; <span class="number">9 </span>|| c &gt; <span class="number">9 </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(InvalidStatusCode::new());
        }

        <span class="kw">let </span>status = (a * <span class="number">100</span>) + (b * <span class="number">10</span>) + c;
        NonZeroU16::new(status)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }

    <span class="doccomment">/// Returns the `u16` corresponding to this `StatusCode`.
    ///
    /// # Note
    ///
    /// This is the same as the `From&lt;StatusCode&gt;` implementation, but
    /// included as an inherent method because that implementation doesn't
    /// appear in rustdocs, as well as a way to force the type instead of
    /// relying on inference.
    ///
    /// # Example
    ///
    /// ```
    /// let status = http::StatusCode::OK;
    /// assert_eq!(status.as_u16(), 200);
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>as_u16(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; u16 {
        (<span class="kw-2">*</span><span class="self">self</span>).into()
    }

    <span class="doccomment">/// Returns a &amp;str representation of the `StatusCode`
    ///
    /// The return value only includes a numerical representation of the
    /// status code. The canonical reason is not included.
    ///
    /// # Example
    ///
    /// ```
    /// let status = http::StatusCode::OK;
    /// assert_eq!(status.as_str(), "200");
    /// ```
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>as_str(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>str {
        <span class="kw">let </span>offset = (<span class="self">self</span>.<span class="number">0</span>.get() - <span class="number">100</span>) <span class="kw">as </span>usize;
        <span class="kw">let </span>offset = offset * <span class="number">3</span>;

        <span class="comment">// Invariant: self has checked range [100, 999] and CODE_DIGITS is
        // ASCII-only, of length 900 * 3 = 2700 bytes

        </span><span class="attr">#[cfg(debug_assertions)]
        </span>{ <span class="kw-2">&amp;</span>CODE_DIGITS[offset..offset+<span class="number">3</span>] }

        <span class="attr">#[cfg(not(debug_assertions))]
        </span><span class="kw">unsafe </span>{ CODE_DIGITS.get_unchecked(offset..offset+<span class="number">3</span>) }
    }

    <span class="doccomment">/// Get the standardised `reason-phrase` for this status code.
    ///
    /// This is mostly here for servers writing responses, but could potentially have application
    /// at other times.
    ///
    /// The reason phrase is defined as being exclusively for human readers. You should avoid
    /// deriving any meaning from it at all costs.
    ///
    /// Bear in mind also that in HTTP/2.0 and HTTP/3.0 the reason phrase is abolished from
    /// transmission, and so this canonical reason phrase really is the only reason phrase you’ll
    /// find.
    ///
    /// # Example
    ///
    /// ```
    /// let status = http::StatusCode::OK;
    /// assert_eq!(status.canonical_reason(), Some("OK"));
    /// ```
    </span><span class="kw">pub fn </span>canonical_reason(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'static </span>str&gt; {
        canonical_reason(<span class="self">self</span>.<span class="number">0</span>.get())
    }

    <span class="doccomment">/// Check if status is within 100-199.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_informational(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="number">200 </span>&gt; <span class="self">self</span>.<span class="number">0</span>.get() &amp;&amp; <span class="self">self</span>.<span class="number">0</span>.get() &gt;= <span class="number">100
    </span>}

    <span class="doccomment">/// Check if status is within 200-299.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_success(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="number">300 </span>&gt; <span class="self">self</span>.<span class="number">0</span>.get() &amp;&amp; <span class="self">self</span>.<span class="number">0</span>.get() &gt;= <span class="number">200
    </span>}

    <span class="doccomment">/// Check if status is within 300-399.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_redirection(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="number">400 </span>&gt; <span class="self">self</span>.<span class="number">0</span>.get() &amp;&amp; <span class="self">self</span>.<span class="number">0</span>.get() &gt;= <span class="number">300
    </span>}

    <span class="doccomment">/// Check if status is within 400-499.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_client_error(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="number">500 </span>&gt; <span class="self">self</span>.<span class="number">0</span>.get() &amp;&amp; <span class="self">self</span>.<span class="number">0</span>.get() &gt;= <span class="number">400
    </span>}

    <span class="doccomment">/// Check if status is within 500-599.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_server_error(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="number">600 </span>&gt; <span class="self">self</span>.<span class="number">0</span>.get() &amp;&amp; <span class="self">self</span>.<span class="number">0</span>.get() &gt;= <span class="number">500
    </span>}
}

<span class="kw">impl </span>fmt::Debug <span class="kw">for </span>StatusCode {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        fmt::Debug::fmt(<span class="kw-2">&amp;</span><span class="self">self</span>.<span class="number">0</span>, f)
    }
}

<span class="doccomment">/// Formats the status code, *including* the canonical reason.
///
/// # Example
///
/// ```
/// # use http::StatusCode;
/// assert_eq!(format!("{}", StatusCode::OK), "200 OK");
/// ```
</span><span class="kw">impl </span>fmt::Display <span class="kw">for </span>StatusCode {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        <span class="macro">write!</span>(
            f,
            <span class="string">"{} {}"</span>,
            u16::from(<span class="kw-2">*</span><span class="self">self</span>),
            <span class="self">self</span>.canonical_reason().unwrap_or(<span class="string">"&lt;unknown status code&gt;"</span>)
        )
    }
}

<span class="kw">impl </span>Default <span class="kw">for </span>StatusCode {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>default() -&gt; StatusCode {
        StatusCode::OK
    }
}

<span class="kw">impl </span>PartialEq&lt;u16&gt; <span class="kw">for </span>StatusCode {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>u16) -&gt; bool {
        <span class="self">self</span>.as_u16() == <span class="kw-2">*</span>other
    }
}

<span class="kw">impl </span>PartialEq&lt;StatusCode&gt; <span class="kw">for </span>u16 {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>StatusCode) -&gt; bool {
        <span class="kw-2">*</span><span class="self">self </span>== other.as_u16()
    }
}

<span class="kw">impl </span>From&lt;StatusCode&gt; <span class="kw">for </span>u16 {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from(status: StatusCode) -&gt; u16 {
        status.<span class="number">0</span>.get()
    }
}

<span class="kw">impl </span>FromStr <span class="kw">for </span>StatusCode {
    <span class="kw">type </span><span class="prelude-val">Err </span>= InvalidStatusCode;

    <span class="kw">fn </span>from_str(s: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;StatusCode, InvalidStatusCode&gt; {
        StatusCode::from_bytes(s.as_ref())
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; From&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>StatusCode&gt; <span class="kw">for </span>StatusCode {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from(t: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>StatusCode) -&gt; <span class="self">Self </span>{
        t.clone()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; TryFrom&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8]&gt; <span class="kw">for </span>StatusCode {
    <span class="kw">type </span>Error = InvalidStatusCode;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>try_from(t: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>[u8]) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, <span class="self">Self</span>::Error&gt; {
        StatusCode::from_bytes(t)
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; TryFrom&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str&gt; <span class="kw">for </span>StatusCode {
    <span class="kw">type </span>Error = InvalidStatusCode;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>try_from(t: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, <span class="self">Self</span>::Error&gt; {
        t.parse()
    }
}

<span class="kw">impl </span>TryFrom&lt;u16&gt; <span class="kw">for </span>StatusCode {
    <span class="kw">type </span>Error = InvalidStatusCode;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>try_from(t: u16) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, <span class="self">Self</span>::Error&gt; {
        StatusCode::from_u16(t)
    }
}

<span class="macro">macro_rules!</span> status_codes {
    (
        $(
            $(<span class="attr">#[<span class="macro-nonterminal">$docs</span>:meta]</span>)*
            (<span class="macro-nonterminal">$num</span>:expr, <span class="macro-nonterminal">$konst</span>:ident, <span class="macro-nonterminal">$phrase</span>:expr);
        )+
    ) =&gt; {
        <span class="kw">impl </span>StatusCode {
        $(
            $(<span class="attr">#[<span class="macro-nonterminal">$docs</span>]</span>)*
            <span class="kw">pub const </span><span class="macro-nonterminal">$konst</span>: StatusCode = StatusCode(<span class="kw">unsafe </span>{ NonZeroU16::new_unchecked(<span class="macro-nonterminal">$num</span>) });
        )+

        }

        <span class="kw">fn </span>canonical_reason(num: u16) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'static </span>str&gt; {
            <span class="kw">match </span>num {
                $(
                <span class="macro-nonterminal">$num </span>=&gt; <span class="prelude-val">Some</span>(<span class="macro-nonterminal">$phrase</span>),
                )+
                <span class="kw">_ </span>=&gt; <span class="prelude-val">None
            </span>}
        }
    }
}

<span class="macro">status_codes!</span> {
    <span class="doccomment">/// 100 Continue
    /// [[RFC7231, Section 6.2.1](https://tools.ietf.org/html/rfc7231#section-6.2.1)]
    </span>(<span class="number">100</span>, CONTINUE, <span class="string">"Continue"</span>);
    <span class="doccomment">/// 101 Switching Protocols
    /// [[RFC7231, Section 6.2.2](https://tools.ietf.org/html/rfc7231#section-6.2.2)]
    </span>(<span class="number">101</span>, SWITCHING_PROTOCOLS, <span class="string">"Switching Protocols"</span>);
    <span class="doccomment">/// 102 Processing
    /// [[RFC2518](https://tools.ietf.org/html/rfc2518)]
    </span>(<span class="number">102</span>, PROCESSING, <span class="string">"Processing"</span>);

    <span class="doccomment">/// 200 OK
    /// [[RFC7231, Section 6.3.1](https://tools.ietf.org/html/rfc7231#section-6.3.1)]
    </span>(<span class="number">200</span>, OK, <span class="string">"OK"</span>);
    <span class="doccomment">/// 201 Created
    /// [[RFC7231, Section 6.3.2](https://tools.ietf.org/html/rfc7231#section-6.3.2)]
    </span>(<span class="number">201</span>, CREATED, <span class="string">"Created"</span>);
    <span class="doccomment">/// 202 Accepted
    /// [[RFC7231, Section 6.3.3](https://tools.ietf.org/html/rfc7231#section-6.3.3)]
    </span>(<span class="number">202</span>, ACCEPTED, <span class="string">"Accepted"</span>);
    <span class="doccomment">/// 203 Non-Authoritative Information
    /// [[RFC7231, Section 6.3.4](https://tools.ietf.org/html/rfc7231#section-6.3.4)]
    </span>(<span class="number">203</span>, NON_AUTHORITATIVE_INFORMATION, <span class="string">"Non Authoritative Information"</span>);
    <span class="doccomment">/// 204 No Content
    /// [[RFC7231, Section 6.3.5](https://tools.ietf.org/html/rfc7231#section-6.3.5)]
    </span>(<span class="number">204</span>, NO_CONTENT, <span class="string">"No Content"</span>);
    <span class="doccomment">/// 205 Reset Content
    /// [[RFC7231, Section 6.3.6](https://tools.ietf.org/html/rfc7231#section-6.3.6)]
    </span>(<span class="number">205</span>, RESET_CONTENT, <span class="string">"Reset Content"</span>);
    <span class="doccomment">/// 206 Partial Content
    /// [[RFC7233, Section 4.1](https://tools.ietf.org/html/rfc7233#section-4.1)]
    </span>(<span class="number">206</span>, PARTIAL_CONTENT, <span class="string">"Partial Content"</span>);
    <span class="doccomment">/// 207 Multi-Status
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    </span>(<span class="number">207</span>, MULTI_STATUS, <span class="string">"Multi-Status"</span>);
    <span class="doccomment">/// 208 Already Reported
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    </span>(<span class="number">208</span>, ALREADY_REPORTED, <span class="string">"Already Reported"</span>);

    <span class="doccomment">/// 226 IM Used
    /// [[RFC3229](https://tools.ietf.org/html/rfc3229)]
    </span>(<span class="number">226</span>, IM_USED, <span class="string">"IM Used"</span>);

    <span class="doccomment">/// 300 Multiple Choices
    /// [[RFC7231, Section 6.4.1](https://tools.ietf.org/html/rfc7231#section-6.4.1)]
    </span>(<span class="number">300</span>, MULTIPLE_CHOICES, <span class="string">"Multiple Choices"</span>);
    <span class="doccomment">/// 301 Moved Permanently
    /// [[RFC7231, Section 6.4.2](https://tools.ietf.org/html/rfc7231#section-6.4.2)]
    </span>(<span class="number">301</span>, MOVED_PERMANENTLY, <span class="string">"Moved Permanently"</span>);
    <span class="doccomment">/// 302 Found
    /// [[RFC7231, Section 6.4.3](https://tools.ietf.org/html/rfc7231#section-6.4.3)]
    </span>(<span class="number">302</span>, FOUND, <span class="string">"Found"</span>);
    <span class="doccomment">/// 303 See Other
    /// [[RFC7231, Section 6.4.4](https://tools.ietf.org/html/rfc7231#section-6.4.4)]
    </span>(<span class="number">303</span>, SEE_OTHER, <span class="string">"See Other"</span>);
    <span class="doccomment">/// 304 Not Modified
    /// [[RFC7232, Section 4.1](https://tools.ietf.org/html/rfc7232#section-4.1)]
    </span>(<span class="number">304</span>, NOT_MODIFIED, <span class="string">"Not Modified"</span>);
    <span class="doccomment">/// 305 Use Proxy
    /// [[RFC7231, Section 6.4.5](https://tools.ietf.org/html/rfc7231#section-6.4.5)]
    </span>(<span class="number">305</span>, USE_PROXY, <span class="string">"Use Proxy"</span>);
    <span class="doccomment">/// 307 Temporary Redirect
    /// [[RFC7231, Section 6.4.7](https://tools.ietf.org/html/rfc7231#section-6.4.7)]
    </span>(<span class="number">307</span>, TEMPORARY_REDIRECT, <span class="string">"Temporary Redirect"</span>);
    <span class="doccomment">/// 308 Permanent Redirect
    /// [[RFC7238](https://tools.ietf.org/html/rfc7238)]
    </span>(<span class="number">308</span>, PERMANENT_REDIRECT, <span class="string">"Permanent Redirect"</span>);

    <span class="doccomment">/// 400 Bad Request
    /// [[RFC7231, Section 6.5.1](https://tools.ietf.org/html/rfc7231#section-6.5.1)]
    </span>(<span class="number">400</span>, BAD_REQUEST, <span class="string">"Bad Request"</span>);
    <span class="doccomment">/// 401 Unauthorized
    /// [[RFC7235, Section 3.1](https://tools.ietf.org/html/rfc7235#section-3.1)]
    </span>(<span class="number">401</span>, UNAUTHORIZED, <span class="string">"Unauthorized"</span>);
    <span class="doccomment">/// 402 Payment Required
    /// [[RFC7231, Section 6.5.2](https://tools.ietf.org/html/rfc7231#section-6.5.2)]
    </span>(<span class="number">402</span>, PAYMENT_REQUIRED, <span class="string">"Payment Required"</span>);
    <span class="doccomment">/// 403 Forbidden
    /// [[RFC7231, Section 6.5.3](https://tools.ietf.org/html/rfc7231#section-6.5.3)]
    </span>(<span class="number">403</span>, FORBIDDEN, <span class="string">"Forbidden"</span>);
    <span class="doccomment">/// 404 Not Found
    /// [[RFC7231, Section 6.5.4](https://tools.ietf.org/html/rfc7231#section-6.5.4)]
    </span>(<span class="number">404</span>, NOT_FOUND, <span class="string">"Not Found"</span>);
    <span class="doccomment">/// 405 Method Not Allowed
    /// [[RFC7231, Section 6.5.5](https://tools.ietf.org/html/rfc7231#section-6.5.5)]
    </span>(<span class="number">405</span>, METHOD_NOT_ALLOWED, <span class="string">"Method Not Allowed"</span>);
    <span class="doccomment">/// 406 Not Acceptable
    /// [[RFC7231, Section 6.5.6](https://tools.ietf.org/html/rfc7231#section-6.5.6)]
    </span>(<span class="number">406</span>, NOT_ACCEPTABLE, <span class="string">"Not Acceptable"</span>);
    <span class="doccomment">/// 407 Proxy Authentication Required
    /// [[RFC7235, Section 3.2](https://tools.ietf.org/html/rfc7235#section-3.2)]
    </span>(<span class="number">407</span>, PROXY_AUTHENTICATION_REQUIRED, <span class="string">"Proxy Authentication Required"</span>);
    <span class="doccomment">/// 408 Request Timeout
    /// [[RFC7231, Section 6.5.7](https://tools.ietf.org/html/rfc7231#section-6.5.7)]
    </span>(<span class="number">408</span>, REQUEST_TIMEOUT, <span class="string">"Request Timeout"</span>);
    <span class="doccomment">/// 409 Conflict
    /// [[RFC7231, Section 6.5.8](https://tools.ietf.org/html/rfc7231#section-6.5.8)]
    </span>(<span class="number">409</span>, CONFLICT, <span class="string">"Conflict"</span>);
    <span class="doccomment">/// 410 Gone
    /// [[RFC7231, Section 6.5.9](https://tools.ietf.org/html/rfc7231#section-6.5.9)]
    </span>(<span class="number">410</span>, GONE, <span class="string">"Gone"</span>);
    <span class="doccomment">/// 411 Length Required
    /// [[RFC7231, Section 6.5.10](https://tools.ietf.org/html/rfc7231#section-6.5.10)]
    </span>(<span class="number">411</span>, LENGTH_REQUIRED, <span class="string">"Length Required"</span>);
    <span class="doccomment">/// 412 Precondition Failed
    /// [[RFC7232, Section 4.2](https://tools.ietf.org/html/rfc7232#section-4.2)]
    </span>(<span class="number">412</span>, PRECONDITION_FAILED, <span class="string">"Precondition Failed"</span>);
    <span class="doccomment">/// 413 Payload Too Large
    /// [[RFC7231, Section 6.5.11](https://tools.ietf.org/html/rfc7231#section-6.5.11)]
    </span>(<span class="number">413</span>, PAYLOAD_TOO_LARGE, <span class="string">"Payload Too Large"</span>);
    <span class="doccomment">/// 414 URI Too Long
    /// [[RFC7231, Section 6.5.12](https://tools.ietf.org/html/rfc7231#section-6.5.12)]
    </span>(<span class="number">414</span>, URI_TOO_LONG, <span class="string">"URI Too Long"</span>);
    <span class="doccomment">/// 415 Unsupported Media Type
    /// [[RFC7231, Section 6.5.13](https://tools.ietf.org/html/rfc7231#section-6.5.13)]
    </span>(<span class="number">415</span>, UNSUPPORTED_MEDIA_TYPE, <span class="string">"Unsupported Media Type"</span>);
    <span class="doccomment">/// 416 Range Not Satisfiable
    /// [[RFC7233, Section 4.4](https://tools.ietf.org/html/rfc7233#section-4.4)]
    </span>(<span class="number">416</span>, RANGE_NOT_SATISFIABLE, <span class="string">"Range Not Satisfiable"</span>);
    <span class="doccomment">/// 417 Expectation Failed
    /// [[RFC7231, Section 6.5.14](https://tools.ietf.org/html/rfc7231#section-6.5.14)]
    </span>(<span class="number">417</span>, EXPECTATION_FAILED, <span class="string">"Expectation Failed"</span>);
    <span class="doccomment">/// 418 I'm a teapot
    /// [curiously not registered by IANA but [RFC2324](https://tools.ietf.org/html/rfc2324)]
    </span>(<span class="number">418</span>, IM_A_TEAPOT, <span class="string">"I'm a teapot"</span>);

    <span class="doccomment">/// 421 Misdirected Request
    /// [RFC7540, Section 9.1.2](http://tools.ietf.org/html/rfc7540#section-9.1.2)
    </span>(<span class="number">421</span>, MISDIRECTED_REQUEST, <span class="string">"Misdirected Request"</span>);
    <span class="doccomment">/// 422 Unprocessable Entity
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    </span>(<span class="number">422</span>, UNPROCESSABLE_ENTITY, <span class="string">"Unprocessable Entity"</span>);
    <span class="doccomment">/// 423 Locked
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    </span>(<span class="number">423</span>, LOCKED, <span class="string">"Locked"</span>);
    <span class="doccomment">/// 424 Failed Dependency
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    </span>(<span class="number">424</span>, FAILED_DEPENDENCY, <span class="string">"Failed Dependency"</span>);

    <span class="doccomment">/// 426 Upgrade Required
    /// [[RFC7231, Section 6.5.15](https://tools.ietf.org/html/rfc7231#section-6.5.15)]
    </span>(<span class="number">426</span>, UPGRADE_REQUIRED, <span class="string">"Upgrade Required"</span>);

    <span class="doccomment">/// 428 Precondition Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    </span>(<span class="number">428</span>, PRECONDITION_REQUIRED, <span class="string">"Precondition Required"</span>);
    <span class="doccomment">/// 429 Too Many Requests
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    </span>(<span class="number">429</span>, TOO_MANY_REQUESTS, <span class="string">"Too Many Requests"</span>);

    <span class="doccomment">/// 431 Request Header Fields Too Large
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    </span>(<span class="number">431</span>, REQUEST_HEADER_FIELDS_TOO_LARGE, <span class="string">"Request Header Fields Too Large"</span>);

    <span class="doccomment">/// 451 Unavailable For Legal Reasons
    /// [[RFC7725](http://tools.ietf.org/html/rfc7725)]
    </span>(<span class="number">451</span>, UNAVAILABLE_FOR_LEGAL_REASONS, <span class="string">"Unavailable For Legal Reasons"</span>);

    <span class="doccomment">/// 500 Internal Server Error
    /// [[RFC7231, Section 6.6.1](https://tools.ietf.org/html/rfc7231#section-6.6.1)]
    </span>(<span class="number">500</span>, INTERNAL_SERVER_ERROR, <span class="string">"Internal Server Error"</span>);
    <span class="doccomment">/// 501 Not Implemented
    /// [[RFC7231, Section 6.6.2](https://tools.ietf.org/html/rfc7231#section-6.6.2)]
    </span>(<span class="number">501</span>, NOT_IMPLEMENTED, <span class="string">"Not Implemented"</span>);
    <span class="doccomment">/// 502 Bad Gateway
    /// [[RFC7231, Section 6.6.3](https://tools.ietf.org/html/rfc7231#section-6.6.3)]
    </span>(<span class="number">502</span>, BAD_GATEWAY, <span class="string">"Bad Gateway"</span>);
    <span class="doccomment">/// 503 Service Unavailable
    /// [[RFC7231, Section 6.6.4](https://tools.ietf.org/html/rfc7231#section-6.6.4)]
    </span>(<span class="number">503</span>, SERVICE_UNAVAILABLE, <span class="string">"Service Unavailable"</span>);
    <span class="doccomment">/// 504 Gateway Timeout
    /// [[RFC7231, Section 6.6.5](https://tools.ietf.org/html/rfc7231#section-6.6.5)]
    </span>(<span class="number">504</span>, GATEWAY_TIMEOUT, <span class="string">"Gateway Timeout"</span>);
    <span class="doccomment">/// 505 HTTP Version Not Supported
    /// [[RFC7231, Section 6.6.6](https://tools.ietf.org/html/rfc7231#section-6.6.6)]
    </span>(<span class="number">505</span>, HTTP_VERSION_NOT_SUPPORTED, <span class="string">"HTTP Version Not Supported"</span>);
    <span class="doccomment">/// 506 Variant Also Negotiates
    /// [[RFC2295](https://tools.ietf.org/html/rfc2295)]
    </span>(<span class="number">506</span>, VARIANT_ALSO_NEGOTIATES, <span class="string">"Variant Also Negotiates"</span>);
    <span class="doccomment">/// 507 Insufficient Storage
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    </span>(<span class="number">507</span>, INSUFFICIENT_STORAGE, <span class="string">"Insufficient Storage"</span>);
    <span class="doccomment">/// 508 Loop Detected
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    </span>(<span class="number">508</span>, LOOP_DETECTED, <span class="string">"Loop Detected"</span>);

    <span class="doccomment">/// 510 Not Extended
    /// [[RFC2774](https://tools.ietf.org/html/rfc2774)]
    </span>(<span class="number">510</span>, NOT_EXTENDED, <span class="string">"Not Extended"</span>);
    <span class="doccomment">/// 511 Network Authentication Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    </span>(<span class="number">511</span>, NETWORK_AUTHENTICATION_REQUIRED, <span class="string">"Network Authentication Required"</span>);
}

<span class="kw">impl </span>InvalidStatusCode {
    <span class="kw">fn </span>new() -&gt; InvalidStatusCode {
        InvalidStatusCode {
            _priv: (),
        }
    }
}

<span class="kw">impl </span>fmt::Debug <span class="kw">for </span>InvalidStatusCode {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        f.debug_struct(<span class="string">"InvalidStatusCode"</span>)
            <span class="comment">// skip _priv noise
            </span>.finish()
    }
}

<span class="kw">impl </span>fmt::Display <span class="kw">for </span>InvalidStatusCode {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        f.write_str(<span class="string">"invalid status code"</span>)
    }
}

<span class="kw">impl </span>Error <span class="kw">for </span>InvalidStatusCode {}

<span class="comment">// A string of packed 3-ASCII-digit status code values for the supported range
// of [100, 999] (900 codes, 2700 bytes).
</span><span class="kw">const </span>CODE_DIGITS: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str = <span class="string">"\
100101102103104105106107108109110111112113114115116117118119\
120121122123124125126127128129130131132133134135136137138139\
140141142143144145146147148149150151152153154155156157158159\
160161162163164165166167168169170171172173174175176177178179\
180181182183184185186187188189190191192193194195196197198199\
200201202203204205206207208209210211212213214215216217218219\
220221222223224225226227228229230231232233234235236237238239\
240241242243244245246247248249250251252253254255256257258259\
260261262263264265266267268269270271272273274275276277278279\
280281282283284285286287288289290291292293294295296297298299\
300301302303304305306307308309310311312313314315316317318319\
320321322323324325326327328329330331332333334335336337338339\
340341342343344345346347348349350351352353354355356357358359\
360361362363364365366367368369370371372373374375376377378379\
380381382383384385386387388389390391392393394395396397398399\
400401402403404405406407408409410411412413414415416417418419\
420421422423424425426427428429430431432433434435436437438439\
440441442443444445446447448449450451452453454455456457458459\
460461462463464465466467468469470471472473474475476477478479\
480481482483484485486487488489490491492493494495496497498499\
500501502503504505506507508509510511512513514515516517518519\
520521522523524525526527528529530531532533534535536537538539\
540541542543544545546547548549550551552553554555556557558559\
560561562563564565566567568569570571572573574575576577578579\
580581582583584585586587588589590591592593594595596597598599\
600601602603604605606607608609610611612613614615616617618619\
620621622623624625626627628629630631632633634635636637638639\
640641642643644645646647648649650651652653654655656657658659\
660661662663664665666667668669670671672673674675676677678679\
680681682683684685686687688689690691692693694695696697698699\
700701702703704705706707708709710711712713714715716717718719\
720721722723724725726727728729730731732733734735736737738739\
740741742743744745746747748749750751752753754755756757758759\
760761762763764765766767768769770771772773774775776777778779\
780781782783784785786787788789790791792793794795796797798799\
800801802803804805806807808809810811812813814815816817818819\
820821822823824825826827828829830831832833834835836837838839\
840841842843844845846847848849850851852853854855856857858859\
860861862863864865866867868869870871872873874875876877878879\
880881882883884885886887888889890891892893894895896897898899\
900901902903904905906907908909910911912913914915916917918919\
920921922923924925926927928929930931932933934935936937938939\
940941942943944945946947948949950951952953954955956957958959\
960961962963964965966967968969970971972973974975976977978979\
980981982983984985986987988989990991992993994995996997998999"</span>;
</code></pre></div></section></main></body></html>