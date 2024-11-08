<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rayon-core-1.12.1/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="rayon_core" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../rayon_core/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Rayon-core houses the core stable APIs of Rayon.
//!
//! These APIs have been mirrored in the Rayon crate and it is recommended to use these from there.
//!
//! [`join`] is used to take two closures and potentially run them in parallel.
//!   - It will run in parallel if task B gets stolen before task A can finish.
//!   - It will run sequentially if task A finishes before task B is stolen and can continue on task B.
//!
//! [`scope`] creates a scope in which you can run any number of parallel tasks.
//! These tasks can spawn nested tasks and scopes, but given the nature of work stealing, the order of execution can not be guaranteed.
//! The scope will exist until all tasks spawned within the scope have been completed.
//!
//! [`spawn`] add a task into the 'static' or 'global' scope, or a local scope created by the [`scope()`] function.
//!
//! [`ThreadPool`] can be used to create your own thread pools (using [`ThreadPoolBuilder`]) or to customize the global one.
//! Tasks spawned within the pool (using [`install()`], [`join()`], etc.) will be added to a deque,
//! where it becomes available for work stealing from other threads in the local threadpool.
//!
//! [`join`]: fn.join.html
//! [`scope`]: fn.scope.html
//! [`scope()`]: fn.scope.html
//! [`spawn`]: fn.spawn.html
//! [`ThreadPool`]: struct.threadpool.html
//! [`install()`]: struct.ThreadPool.html#method.install
//! [`spawn()`]: struct.ThreadPool.html#method.spawn
//! [`join()`]: struct.ThreadPool.html#method.join
//! [`ThreadPoolBuilder`]: struct.ThreadPoolBuilder.html
//!
//! # Global fallback when threading is unsupported
//!
//! Rayon uses `std` APIs for threading, but some targets have incomplete implementations that
//! always return `Unsupported` errors. The WebAssembly `wasm32-unknown-unknown` and `wasm32-wasi`
//! targets are notable examples of this. Rather than panicking on the unsupported error when
//! creating the implicit global threadpool, Rayon configures a fallback mode instead.
//!
//! This fallback mode mostly functions as if it were using a single-threaded "pool", like setting
//! `RAYON_NUM_THREADS=1`. For example, `join` will execute its two closures sequentially, since
//! there is no other thread to share the work. However, since the pool is not running independent
//! of the main thread, non-blocking calls like `spawn` may not execute at all, unless a lower-
//! priority call like `broadcast` gives them an opening. The fallback mode does not try to emulate
//! anything like thread preemption or `async` task switching, but `yield_now` or `yield_local`
//! can also volunteer execution time.
//!
//! Explicit `ThreadPoolBuilder` methods always report their error without any fallback.
//!
//! # Restricting multiple versions
//!
//! In order to ensure proper coordination between threadpools, and especially
//! to make sure there's only one global threadpool, `rayon-core` is actively
//! restricted from building multiple versions of itself into a single target.
//! You may see a build error like this in violation:
//!
//! ```text
//! error: native library `rayon-core` is being linked to by more
//! than one package, and can only be linked to by one package
//! ```
//!
//! While we strive to keep `rayon-core` semver-compatible, it's still
//! possible to arrive at this situation if different crates have overly
//! restrictive tilde or inequality requirements for `rayon-core`.  The
//! conflicting requirements will need to be resolved before the build will
//! succeed.

</span><span class="attr">#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(unreachable_pub)]
#![warn(rust_2018_idioms)]

</span><span class="kw">use </span>std::any::Any;
<span class="kw">use </span>std::env;
<span class="kw">use </span>std::error::Error;
<span class="kw">use </span>std::fmt;
<span class="kw">use </span>std::io;
<span class="kw">use </span>std::marker::PhantomData;
<span class="kw">use </span>std::str::FromStr;
<span class="kw">use </span>std::thread;

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>private;

<span class="kw">mod </span>broadcast;
<span class="kw">mod </span>job;
<span class="kw">mod </span>join;
<span class="kw">mod </span>latch;
<span class="kw">mod </span>registry;
<span class="kw">mod </span>scope;
<span class="kw">mod </span>sleep;
<span class="kw">mod </span>spawn;
<span class="kw">mod </span>thread_pool;
<span class="kw">mod </span>unwind;

<span class="kw">mod </span>compile_fail;
<span class="kw">mod </span>test;

<span class="kw">pub use </span><span class="self">self</span>::broadcast::{broadcast, spawn_broadcast, BroadcastContext};
<span class="kw">pub use </span><span class="self">self</span>::join::{join, join_context};
<span class="kw">pub use </span><span class="self">self</span>::registry::ThreadBuilder;
<span class="kw">pub use </span><span class="self">self</span>::scope::{in_place_scope, scope, Scope};
<span class="kw">pub use </span><span class="self">self</span>::scope::{in_place_scope_fifo, scope_fifo, ScopeFifo};
<span class="kw">pub use </span><span class="self">self</span>::spawn::{spawn, spawn_fifo};
<span class="kw">pub use </span><span class="self">self</span>::thread_pool::current_thread_has_pending_tasks;
<span class="kw">pub use </span><span class="self">self</span>::thread_pool::current_thread_index;
<span class="kw">pub use </span><span class="self">self</span>::thread_pool::ThreadPool;
<span class="kw">pub use </span><span class="self">self</span>::thread_pool::{yield_local, yield_now, Yield};

<span class="attr">#[cfg(not(feature = <span class="string">"web_spin_lock"</span>))]
</span><span class="kw">use </span>std::sync;

<span class="attr">#[cfg(feature = <span class="string">"web_spin_lock"</span>)]
</span><span class="kw">use </span>wasm_sync <span class="kw">as </span>sync;

<span class="kw">use </span><span class="self">self</span>::registry::{CustomSpawn, DefaultSpawn, ThreadSpawn};

<span class="doccomment">/// Returns the maximum number of threads that Rayon supports in a single thread-pool.
///
/// If a higher thread count is requested by calling `ThreadPoolBuilder::num_threads` or by setting
/// the `RAYON_NUM_THREADS` environment variable, then it will be reduced to this maximum.
///
/// The value may vary between different targets, and is subject to change in new Rayon versions.
</span><span class="kw">pub fn </span>max_num_threads() -&gt; usize {
    <span class="comment">// We are limited by the bits available in the sleep counter's `AtomicUsize`.
    </span><span class="kw">crate</span>::sleep::THREADS_MAX
}

<span class="doccomment">/// Returns the number of threads in the current registry. If this
/// code is executing within a Rayon thread-pool, then this will be
/// the number of threads for the thread-pool of the current
/// thread. Otherwise, it will be the number of threads for the global
/// thread-pool.
///
/// This can be useful when trying to judge how many times to split
/// parallel work (the parallel iterator traits use this value
/// internally for this purpose).
///
/// # Future compatibility note
///
/// Note that unless this thread-pool was created with a
/// builder that specifies the number of threads, then this
/// number may vary over time in future versions (see [the
/// `num_threads()` method for details][snt]).
///
/// [snt]: struct.ThreadPoolBuilder.html#method.num_threads
</span><span class="kw">pub fn </span>current_num_threads() -&gt; usize {
    <span class="kw">crate</span>::registry::Registry::current_num_threads()
}

<span class="doccomment">/// Error when initializing a thread pool.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>ThreadPoolBuildError {
    kind: ErrorKind,
}

<span class="attr">#[derive(Debug)]
</span><span class="kw">enum </span>ErrorKind {
    GlobalPoolAlreadyInitialized,
    CurrentThreadAlreadyInPool,
    IOError(io::Error),
}

<span class="doccomment">/// Used to create a new [`ThreadPool`] or to configure the global rayon thread pool.
/// ## Creating a ThreadPool
/// The following creates a thread pool with 22 threads.
///
/// ```rust
/// # use rayon_core as rayon;
/// let pool = rayon::ThreadPoolBuilder::new().num_threads(22).build().unwrap();
/// ```
///
/// To instead configure the global thread pool, use [`build_global()`]:
///
/// ```rust
/// # use rayon_core as rayon;
/// rayon::ThreadPoolBuilder::new().num_threads(22).build_global().unwrap();
/// ```
///
/// [`ThreadPool`]: struct.ThreadPool.html
/// [`build_global()`]: struct.ThreadPoolBuilder.html#method.build_global
</span><span class="kw">pub struct </span>ThreadPoolBuilder&lt;S = DefaultSpawn&gt; {
    <span class="doccomment">/// The number of threads in the rayon thread pool.
    /// If zero will use the RAYON_NUM_THREADS environment variable.
    /// If RAYON_NUM_THREADS is invalid or zero will use the default.
    </span>num_threads: usize,

    <span class="doccomment">/// The thread we're building *from* will also be part of the pool.
    </span>use_current_thread: bool,

    <span class="doccomment">/// Custom closure, if any, to handle a panic that we cannot propagate
    /// anywhere else.
    </span>panic_handler: <span class="prelude-ty">Option</span>&lt;Box&lt;PanicHandler&gt;&gt;,

    <span class="doccomment">/// Closure to compute the name of a thread.
    </span>get_thread_name: <span class="prelude-ty">Option</span>&lt;Box&lt;<span class="kw">dyn </span>FnMut(usize) -&gt; String&gt;&gt;,

    <span class="doccomment">/// The stack size for the created worker threads
    </span>stack_size: <span class="prelude-ty">Option</span>&lt;usize&gt;,

    <span class="doccomment">/// Closure invoked on worker thread start.
    </span>start_handler: <span class="prelude-ty">Option</span>&lt;Box&lt;StartHandler&gt;&gt;,

    <span class="doccomment">/// Closure invoked on worker thread exit.
    </span>exit_handler: <span class="prelude-ty">Option</span>&lt;Box&lt;ExitHandler&gt;&gt;,

    <span class="doccomment">/// Closure invoked to spawn threads.
    </span>spawn_handler: S,

    <span class="doccomment">/// If false, worker threads will execute spawned jobs in a
    /// "depth-first" fashion. If true, they will do a "breadth-first"
    /// fashion. Depth-first is the default.
    </span>breadth_first: bool,
}

<span class="doccomment">/// Contains the rayon thread pool configuration. Use [`ThreadPoolBuilder`] instead.
///
/// [`ThreadPoolBuilder`]: struct.ThreadPoolBuilder.html
</span><span class="attr">#[deprecated(note = <span class="string">"Use `ThreadPoolBuilder`"</span>)]
#[derive(Default)]
</span><span class="kw">pub struct </span>Configuration {
    builder: ThreadPoolBuilder,
}

<span class="doccomment">/// The type for a panic handling closure. Note that this same closure
/// may be invoked multiple times in parallel.
</span><span class="kw">type </span>PanicHandler = <span class="kw">dyn </span>Fn(Box&lt;<span class="kw">dyn </span>Any + Send&gt;) + Send + Sync;

<span class="doccomment">/// The type for a closure that gets invoked when a thread starts. The
/// closure is passed the index of the thread on which it is invoked.
/// Note that this same closure may be invoked multiple times in parallel.
</span><span class="kw">type </span>StartHandler = <span class="kw">dyn </span>Fn(usize) + Send + Sync;

<span class="doccomment">/// The type for a closure that gets invoked when a thread exits. The
/// closure is passed the index of the thread on which is is invoked.
/// Note that this same closure may be invoked multiple times in parallel.
</span><span class="kw">type </span>ExitHandler = <span class="kw">dyn </span>Fn(usize) + Send + Sync;

<span class="comment">// NB: We can't `#[derive(Default)]` because `S` is left ambiguous.
</span><span class="kw">impl </span>Default <span class="kw">for </span>ThreadPoolBuilder {
    <span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        ThreadPoolBuilder {
            num_threads: <span class="number">0</span>,
            use_current_thread: <span class="bool-val">false</span>,
            panic_handler: <span class="prelude-val">None</span>,
            get_thread_name: <span class="prelude-val">None</span>,
            stack_size: <span class="prelude-val">None</span>,
            start_handler: <span class="prelude-val">None</span>,
            exit_handler: <span class="prelude-val">None</span>,
            spawn_handler: DefaultSpawn,
            breadth_first: <span class="bool-val">false</span>,
        }
    }
}

<span class="kw">impl </span>ThreadPoolBuilder {
    <span class="doccomment">/// Creates and returns a valid rayon thread pool builder, but does not initialize it.
    </span><span class="kw">pub fn </span>new() -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::default()
    }
}

<span class="doccomment">/// Note: the `S: ThreadSpawn` constraint is an internal implementation detail for the
/// default spawn and those set by [`spawn_handler`](#method.spawn_handler).
</span><span class="kw">impl</span>&lt;S&gt; ThreadPoolBuilder&lt;S&gt;
<span class="kw">where
    </span>S: ThreadSpawn,
{
    <span class="doccomment">/// Creates a new `ThreadPool` initialized using this configuration.
    </span><span class="kw">pub fn </span>build(<span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;ThreadPool, ThreadPoolBuildError&gt; {
        ThreadPool::build(<span class="self">self</span>)
    }

    <span class="doccomment">/// Initializes the global thread pool. This initialization is
    /// **optional**.  If you do not call this function, the thread pool
    /// will be automatically initialized with the default
    /// configuration. Calling `build_global` is not recommended, except
    /// in two scenarios:
    ///
    /// - You wish to change the default configuration.
    /// - You are running a benchmark, in which case initializing may
    ///   yield slightly more consistent results, since the worker threads
    ///   will already be ready to go even in the first iteration.  But
    ///   this cost is minimal.
    ///
    /// Initialization of the global thread pool happens exactly
    /// once. Once started, the configuration cannot be
    /// changed. Therefore, if you call `build_global` a second time, it
    /// will return an error. An `Ok` result indicates that this
    /// is the first initialization of the thread pool.
    </span><span class="kw">pub fn </span>build_global(<span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), ThreadPoolBuildError&gt; {
        <span class="kw">let </span>registry = registry::init_global_registry(<span class="self">self</span>)<span class="question-mark">?</span>;
        registry.wait_until_primed();
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="kw">impl </span>ThreadPoolBuilder {
    <span class="doccomment">/// Creates a scoped `ThreadPool` initialized using this configuration.
    ///
    /// This is a convenience function for building a pool using [`std::thread::scope`]
    /// to spawn threads in a [`spawn_handler`](#method.spawn_handler).
    /// The threads in this pool will start by calling `wrapper`, which should
    /// do initialization and continue by calling `ThreadBuilder::run()`.
    ///
    /// [`std::thread::scope`]: https://doc.rust-lang.org/std/thread/fn.scope.html
    ///
    /// # Examples
    ///
    /// A scoped pool may be useful in combination with scoped thread-local variables.
    ///
    /// ```
    /// # use rayon_core as rayon;
    ///
    /// scoped_tls::scoped_thread_local!(static POOL_DATA: Vec&lt;i32&gt;);
    ///
    /// fn main() -&gt; Result&lt;(), rayon::ThreadPoolBuildError&gt; {
    ///     let pool_data = vec![1, 2, 3];
    ///
    ///     // We haven't assigned any TLS data yet.
    ///     assert!(!POOL_DATA.is_set());
    ///
    ///     rayon::ThreadPoolBuilder::new()
    ///         .build_scoped(
    ///             // Borrow `pool_data` in TLS for each thread.
    ///             |thread| POOL_DATA.set(&amp;pool_data, || thread.run()),
    ///             // Do some work that needs the TLS data.
    ///             |pool| pool.install(|| assert!(POOL_DATA.is_set())),
    ///         )?;
    ///
    ///     // Once we've returned, `pool_data` is no longer borrowed.
    ///     drop(pool_data);
    ///     Ok(())
    /// }
    /// ```
    </span><span class="kw">pub fn </span>build_scoped&lt;W, F, R&gt;(<span class="self">self</span>, wrapper: W, with_pool: F) -&gt; <span class="prelude-ty">Result</span>&lt;R, ThreadPoolBuildError&gt;
    <span class="kw">where
        </span>W: Fn(ThreadBuilder) + Sync, <span class="comment">// expected to call `run()`
        </span>F: FnOnce(<span class="kw-2">&amp;</span>ThreadPool) -&gt; R,
    {
        std::thread::scope(|scope| {
            <span class="kw">let </span>pool = <span class="self">self
                </span>.spawn_handler(|thread| {
                    <span class="kw">let </span><span class="kw-2">mut </span>builder = std::thread::Builder::new();
                    <span class="kw">if let </span><span class="prelude-val">Some</span>(name) = thread.name() {
                        builder = builder.name(name.to_string());
                    }
                    <span class="kw">if let </span><span class="prelude-val">Some</span>(size) = thread.stack_size() {
                        builder = builder.stack_size(size);
                    }
                    builder.spawn_scoped(scope, || wrapper(thread))<span class="question-mark">?</span>;
                    <span class="prelude-val">Ok</span>(())
                })
                .build()<span class="question-mark">?</span>;
            <span class="prelude-val">Ok</span>(with_pool(<span class="kw-2">&amp;</span>pool))
        })
    }
}

<span class="kw">impl</span>&lt;S&gt; ThreadPoolBuilder&lt;S&gt; {
    <span class="doccomment">/// Sets a custom function for spawning threads.
    ///
    /// Note that the threads will not exit until after the pool is dropped. It
    /// is up to the caller to wait for thread termination if that is important
    /// for any invariants. For instance, threads created in [`std::thread::scope`]
    /// will be joined before that scope returns, and this will block indefinitely
    /// if the pool is leaked. Furthermore, the global thread pool doesn't terminate
    /// until the entire process exits!
    ///
    /// # Examples
    ///
    /// A minimal spawn handler just needs to call `run()` from an independent thread.
    ///
    /// ```
    /// # use rayon_core as rayon;
    /// fn main() -&gt; Result&lt;(), rayon::ThreadPoolBuildError&gt; {
    ///     let pool = rayon::ThreadPoolBuilder::new()
    ///         .spawn_handler(|thread| {
    ///             std::thread::spawn(|| thread.run());
    ///             Ok(())
    ///         })
    ///         .build()?;
    ///
    ///     pool.install(|| println!("Hello from my custom thread!"));
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The default spawn handler sets the name and stack size if given, and propagates
    /// any errors from the thread builder.
    ///
    /// ```
    /// # use rayon_core as rayon;
    /// fn main() -&gt; Result&lt;(), rayon::ThreadPoolBuildError&gt; {
    ///     let pool = rayon::ThreadPoolBuilder::new()
    ///         .spawn_handler(|thread| {
    ///             let mut b = std::thread::Builder::new();
    ///             if let Some(name) = thread.name() {
    ///                 b = b.name(name.to_owned());
    ///             }
    ///             if let Some(stack_size) = thread.stack_size() {
    ///                 b = b.stack_size(stack_size);
    ///             }
    ///             b.spawn(|| thread.run())?;
    ///             Ok(())
    ///         })
    ///         .build()?;
    ///
    ///     pool.install(|| println!("Hello from my fully custom thread!"));
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This can also be used for a pool of scoped threads like [`crossbeam::scope`],
    /// or [`std::thread::scope`] introduced in Rust 1.63, which is encapsulated in
    /// [`build_scoped`](#method.build_scoped).
    ///
    /// [`crossbeam::scope`]: https://docs.rs/crossbeam/0.8/crossbeam/fn.scope.html
    /// [`std::thread::scope`]: https://doc.rust-lang.org/std/thread/fn.scope.html
    ///
    /// ```
    /// # use rayon_core as rayon;
    /// fn main() -&gt; Result&lt;(), rayon::ThreadPoolBuildError&gt; {
    ///     std::thread::scope(|scope| {
    ///         let pool = rayon::ThreadPoolBuilder::new()
    ///             .spawn_handler(|thread| {
    ///                 let mut builder = std::thread::Builder::new();
    ///                 if let Some(name) = thread.name() {
    ///                     builder = builder.name(name.to_string());
    ///                 }
    ///                 if let Some(size) = thread.stack_size() {
    ///                     builder = builder.stack_size(size);
    ///                 }
    ///                 builder.spawn_scoped(scope, || {
    ///                     // Add any scoped initialization here, then run!
    ///                     thread.run()
    ///                 })?;
    ///                 Ok(())
    ///             })
    ///             .build()?;
    ///
    ///         pool.install(|| println!("Hello from my custom scoped thread!"));
    ///         Ok(())
    ///     })
    /// }
    /// ```
    </span><span class="kw">pub fn </span>spawn_handler&lt;F&gt;(<span class="self">self</span>, spawn: F) -&gt; ThreadPoolBuilder&lt;CustomSpawn&lt;F&gt;&gt;
    <span class="kw">where
        </span>F: FnMut(ThreadBuilder) -&gt; io::Result&lt;()&gt;,
    {
        ThreadPoolBuilder {
            spawn_handler: CustomSpawn::new(spawn),
            <span class="comment">// ..self
            </span>num_threads: <span class="self">self</span>.num_threads,
            use_current_thread: <span class="self">self</span>.use_current_thread,
            panic_handler: <span class="self">self</span>.panic_handler,
            get_thread_name: <span class="self">self</span>.get_thread_name,
            stack_size: <span class="self">self</span>.stack_size,
            start_handler: <span class="self">self</span>.start_handler,
            exit_handler: <span class="self">self</span>.exit_handler,
            breadth_first: <span class="self">self</span>.breadth_first,
        }
    }

    <span class="doccomment">/// Returns a reference to the current spawn handler.
    </span><span class="kw">fn </span>get_spawn_handler(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span>S {
        <span class="kw-2">&amp;mut </span><span class="self">self</span>.spawn_handler
    }

    <span class="doccomment">/// Get the number of threads that will be used for the thread
    /// pool. See `num_threads()` for more information.
    </span><span class="kw">fn </span>get_num_threads(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="kw">if </span><span class="self">self</span>.num_threads &gt; <span class="number">0 </span>{
            <span class="self">self</span>.num_threads
        } <span class="kw">else </span>{
            <span class="kw">let </span>default = || {
                thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(<span class="number">1</span>)
            };

            <span class="kw">match </span>env::var(<span class="string">"RAYON_NUM_THREADS"</span>)
                .ok()
                .and_then(|s| usize::from_str(<span class="kw-2">&amp;</span>s).ok())
            {
                <span class="prelude-val">Some</span>(x @ <span class="number">1</span>..) =&gt; <span class="kw">return </span>x,
                <span class="prelude-val">Some</span>(<span class="number">0</span>) =&gt; <span class="kw">return </span>default(),
                <span class="kw">_ </span>=&gt; {}
            }

            <span class="comment">// Support for deprecated `RAYON_RS_NUM_CPUS`.
            </span><span class="kw">match </span>env::var(<span class="string">"RAYON_RS_NUM_CPUS"</span>)
                .ok()
                .and_then(|s| usize::from_str(<span class="kw-2">&amp;</span>s).ok())
            {
                <span class="prelude-val">Some</span>(x @ <span class="number">1</span>..) =&gt; x,
                <span class="kw">_ </span>=&gt; default(),
            }
        }
    }

    <span class="doccomment">/// Get the thread name for the thread with the given index.
    </span><span class="kw">fn </span>get_thread_name(<span class="kw-2">&amp;mut </span><span class="self">self</span>, index: usize) -&gt; <span class="prelude-ty">Option</span>&lt;String&gt; {
        <span class="kw">let </span>f = <span class="self">self</span>.get_thread_name.as_mut()<span class="question-mark">?</span>;
        <span class="prelude-val">Some</span>(f(index))
    }

    <span class="doccomment">/// Sets a closure which takes a thread index and returns
    /// the thread's name.
    </span><span class="kw">pub fn </span>thread_name&lt;F&gt;(<span class="kw-2">mut </span><span class="self">self</span>, closure: F) -&gt; <span class="self">Self
    </span><span class="kw">where
        </span>F: FnMut(usize) -&gt; String + <span class="lifetime">'static</span>,
    {
        <span class="self">self</span>.get_thread_name = <span class="prelude-val">Some</span>(Box::new(closure));
        <span class="self">self
    </span>}

    <span class="doccomment">/// Sets the number of threads to be used in the rayon threadpool.
    ///
    /// If you specify a non-zero number of threads using this
    /// function, then the resulting thread-pools are guaranteed to
    /// start at most this number of threads.
    ///
    /// If `num_threads` is 0, or you do not call this function, then
    /// the Rayon runtime will select the number of threads
    /// automatically. At present, this is based on the
    /// `RAYON_NUM_THREADS` environment variable (if set),
    /// or the number of logical CPUs (otherwise).
    /// In the future, however, the default behavior may
    /// change to dynamically add or remove threads as needed.
    ///
    /// **Future compatibility warning:** Given the default behavior
    /// may change in the future, if you wish to rely on a fixed
    /// number of threads, you should use this function to specify
    /// that number. To reproduce the current default behavior, you
    /// may wish to use [`std::thread::available_parallelism`]
    /// to query the number of CPUs dynamically.
    ///
    /// **Old environment variable:** `RAYON_NUM_THREADS` is a one-to-one
    /// replacement of the now deprecated `RAYON_RS_NUM_CPUS` environment
    /// variable. If both variables are specified, `RAYON_NUM_THREADS` will
    /// be preferred.
    </span><span class="kw">pub fn </span>num_threads(<span class="kw-2">mut </span><span class="self">self</span>, num_threads: usize) -&gt; <span class="self">Self </span>{
        <span class="self">self</span>.num_threads = num_threads;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Use the current thread as one of the threads in the pool.
    ///
    /// The current thread is guaranteed to be at index 0, and since the thread is not managed by
    /// rayon, the spawn and exit handlers do not run for that thread.
    ///
    /// Note that the current thread won't run the main work-stealing loop, so jobs spawned into
    /// the thread-pool will generally not be picked up automatically by this thread unless you
    /// yield to rayon in some way, like via [`yield_now()`], [`yield_local()`], or [`scope()`].
    ///
    /// # Local thread-pools
    ///
    /// Using this in a local thread-pool means the registry will be leaked. In future versions
    /// there might be a way of cleaning up the current-thread state.
    </span><span class="kw">pub fn </span>use_current_thread(<span class="kw-2">mut </span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self</span>.use_current_thread = <span class="bool-val">true</span>;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Returns a copy of the current panic handler.
    </span><span class="kw">fn </span>take_panic_handler(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Box&lt;PanicHandler&gt;&gt; {
        <span class="self">self</span>.panic_handler.take()
    }

    <span class="doccomment">/// Normally, whenever Rayon catches a panic, it tries to
    /// propagate it to someplace sensible, to try and reflect the
    /// semantics of sequential execution. But in some cases,
    /// particularly with the `spawn()` APIs, there is no
    /// obvious place where we should propagate the panic to.
    /// In that case, this panic handler is invoked.
    ///
    /// If no panic handler is set, the default is to abort the
    /// process, under the principle that panics should not go
    /// unobserved.
    ///
    /// If the panic handler itself panics, this will abort the
    /// process. To prevent this, wrap the body of your panic handler
    /// in a call to `std::panic::catch_unwind()`.
    </span><span class="kw">pub fn </span>panic_handler&lt;H&gt;(<span class="kw-2">mut </span><span class="self">self</span>, panic_handler: H) -&gt; <span class="self">Self
    </span><span class="kw">where
        </span>H: Fn(Box&lt;<span class="kw">dyn </span>Any + Send&gt;) + Send + Sync + <span class="lifetime">'static</span>,
    {
        <span class="self">self</span>.panic_handler = <span class="prelude-val">Some</span>(Box::new(panic_handler));
        <span class="self">self
    </span>}

    <span class="doccomment">/// Get the stack size of the worker threads
    </span><span class="kw">fn </span>get_stack_size(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="self">self</span>.stack_size
    }

    <span class="doccomment">/// Sets the stack size of the worker threads
    </span><span class="kw">pub fn </span>stack_size(<span class="kw-2">mut </span><span class="self">self</span>, stack_size: usize) -&gt; <span class="self">Self </span>{
        <span class="self">self</span>.stack_size = <span class="prelude-val">Some</span>(stack_size);
        <span class="self">self
    </span>}

    <span class="doccomment">/// **(DEPRECATED)** Suggest to worker threads that they execute
    /// spawned jobs in a "breadth-first" fashion.
    ///
    /// Typically, when a worker thread is idle or blocked, it will
    /// attempt to execute the job from the *top* of its local deque of
    /// work (i.e., the job most recently spawned). If this flag is set
    /// to true, however, workers will prefer to execute in a
    /// *breadth-first* fashion -- that is, they will search for jobs at
    /// the *bottom* of their local deque. (At present, workers *always*
    /// steal from the bottom of other workers' deques, regardless of
    /// the setting of this flag.)
    ///
    /// If you think of the tasks as a tree, where a parent task
    /// spawns its children in the tree, then this flag loosely
    /// corresponds to doing a breadth-first traversal of the tree,
    /// whereas the default would be to do a depth-first traversal.
    ///
    /// **Note that this is an "execution hint".** Rayon's task
    /// execution is highly dynamic and the precise order in which
    /// independent tasks are executed is not intended to be
    /// guaranteed.
    ///
    /// This `breadth_first()` method is now deprecated per [RFC #1],
    /// and in the future its effect may be removed. Consider using
    /// [`scope_fifo()`] for a similar effect.
    ///
    /// [RFC #1]: https://github.com/rayon-rs/rfcs/blob/master/accepted/rfc0001-scope-scheduling.md
    /// [`scope_fifo()`]: fn.scope_fifo.html
    </span><span class="attr">#[deprecated(note = <span class="string">"use `scope_fifo` and `spawn_fifo` for similar effect"</span>)]
    </span><span class="kw">pub fn </span>breadth_first(<span class="kw-2">mut </span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self</span>.breadth_first = <span class="bool-val">true</span>;
        <span class="self">self
    </span>}

    <span class="kw">fn </span>get_breadth_first(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.breadth_first
    }

    <span class="doccomment">/// Takes the current thread start callback, leaving `None`.
    </span><span class="kw">fn </span>take_start_handler(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Box&lt;StartHandler&gt;&gt; {
        <span class="self">self</span>.start_handler.take()
    }

    <span class="doccomment">/// Sets a callback to be invoked on thread start.
    ///
    /// The closure is passed the index of the thread on which it is invoked.
    /// Note that this same closure may be invoked multiple times in parallel.
    /// If this closure panics, the panic will be passed to the panic handler.
    /// If that handler returns, then startup will continue normally.
    </span><span class="kw">pub fn </span>start_handler&lt;H&gt;(<span class="kw-2">mut </span><span class="self">self</span>, start_handler: H) -&gt; <span class="self">Self
    </span><span class="kw">where
        </span>H: Fn(usize) + Send + Sync + <span class="lifetime">'static</span>,
    {
        <span class="self">self</span>.start_handler = <span class="prelude-val">Some</span>(Box::new(start_handler));
        <span class="self">self
    </span>}

    <span class="doccomment">/// Returns a current thread exit callback, leaving `None`.
    </span><span class="kw">fn </span>take_exit_handler(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Box&lt;ExitHandler&gt;&gt; {
        <span class="self">self</span>.exit_handler.take()
    }

    <span class="doccomment">/// Sets a callback to be invoked on thread exit.
    ///
    /// The closure is passed the index of the thread on which it is invoked.
    /// Note that this same closure may be invoked multiple times in parallel.
    /// If this closure panics, the panic will be passed to the panic handler.
    /// If that handler returns, then the thread will exit normally.
    </span><span class="kw">pub fn </span>exit_handler&lt;H&gt;(<span class="kw-2">mut </span><span class="self">self</span>, exit_handler: H) -&gt; <span class="self">Self
    </span><span class="kw">where
        </span>H: Fn(usize) + Send + Sync + <span class="lifetime">'static</span>,
    {
        <span class="self">self</span>.exit_handler = <span class="prelude-val">Some</span>(Box::new(exit_handler));
        <span class="self">self
    </span>}
}

<span class="attr">#[allow(deprecated)]
</span><span class="kw">impl </span>Configuration {
    <span class="doccomment">/// Creates and return a valid rayon thread pool configuration, but does not initialize it.
    </span><span class="kw">pub fn </span>new() -&gt; Configuration {
        Configuration {
            builder: ThreadPoolBuilder::new(),
        }
    }

    <span class="doccomment">/// Deprecated in favor of `ThreadPoolBuilder::build`.
    </span><span class="kw">pub fn </span>build(<span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;ThreadPool, Box&lt;<span class="kw">dyn </span>Error + <span class="lifetime">'static</span>&gt;&gt; {
        <span class="self">self</span>.builder.build().map_err(Box::from)
    }

    <span class="doccomment">/// Deprecated in favor of `ThreadPoolBuilder::thread_name`.
    </span><span class="kw">pub fn </span>thread_name&lt;F&gt;(<span class="kw-2">mut </span><span class="self">self</span>, closure: F) -&gt; <span class="self">Self
    </span><span class="kw">where
        </span>F: FnMut(usize) -&gt; String + <span class="lifetime">'static</span>,
    {
        <span class="self">self</span>.builder = <span class="self">self</span>.builder.thread_name(closure);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Deprecated in favor of `ThreadPoolBuilder::num_threads`.
    </span><span class="kw">pub fn </span>num_threads(<span class="kw-2">mut </span><span class="self">self</span>, num_threads: usize) -&gt; Configuration {
        <span class="self">self</span>.builder = <span class="self">self</span>.builder.num_threads(num_threads);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Deprecated in favor of `ThreadPoolBuilder::panic_handler`.
    </span><span class="kw">pub fn </span>panic_handler&lt;H&gt;(<span class="kw-2">mut </span><span class="self">self</span>, panic_handler: H) -&gt; Configuration
    <span class="kw">where
        </span>H: Fn(Box&lt;<span class="kw">dyn </span>Any + Send&gt;) + Send + Sync + <span class="lifetime">'static</span>,
    {
        <span class="self">self</span>.builder = <span class="self">self</span>.builder.panic_handler(panic_handler);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Deprecated in favor of `ThreadPoolBuilder::stack_size`.
    </span><span class="kw">pub fn </span>stack_size(<span class="kw-2">mut </span><span class="self">self</span>, stack_size: usize) -&gt; <span class="self">Self </span>{
        <span class="self">self</span>.builder = <span class="self">self</span>.builder.stack_size(stack_size);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Deprecated in favor of `ThreadPoolBuilder::breadth_first`.
    </span><span class="kw">pub fn </span>breadth_first(<span class="kw-2">mut </span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self</span>.builder = <span class="self">self</span>.builder.breadth_first();
        <span class="self">self
    </span>}

    <span class="doccomment">/// Deprecated in favor of `ThreadPoolBuilder::start_handler`.
    </span><span class="kw">pub fn </span>start_handler&lt;H&gt;(<span class="kw-2">mut </span><span class="self">self</span>, start_handler: H) -&gt; Configuration
    <span class="kw">where
        </span>H: Fn(usize) + Send + Sync + <span class="lifetime">'static</span>,
    {
        <span class="self">self</span>.builder = <span class="self">self</span>.builder.start_handler(start_handler);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Deprecated in favor of `ThreadPoolBuilder::exit_handler`.
    </span><span class="kw">pub fn </span>exit_handler&lt;H&gt;(<span class="kw-2">mut </span><span class="self">self</span>, exit_handler: H) -&gt; Configuration
    <span class="kw">where
        </span>H: Fn(usize) + Send + Sync + <span class="lifetime">'static</span>,
    {
        <span class="self">self</span>.builder = <span class="self">self</span>.builder.exit_handler(exit_handler);
        <span class="self">self
    </span>}

    <span class="doccomment">/// Returns a ThreadPoolBuilder with identical parameters.
    </span><span class="kw">fn </span>into_builder(<span class="self">self</span>) -&gt; ThreadPoolBuilder {
        <span class="self">self</span>.builder
    }
}

<span class="kw">impl </span>ThreadPoolBuildError {
    <span class="kw">fn </span>new(kind: ErrorKind) -&gt; ThreadPoolBuildError {
        ThreadPoolBuildError { kind }
    }

    <span class="kw">fn </span>is_unsupported(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="macro">matches!</span>(<span class="kw-2">&amp;</span><span class="self">self</span>.kind, ErrorKind::IOError(e) <span class="kw">if </span>e.kind() == io::ErrorKind::Unsupported)
    }
}

<span class="kw">const </span>GLOBAL_POOL_ALREADY_INITIALIZED: <span class="kw-2">&amp;</span>str =
    <span class="string">"The global thread pool has already been initialized."</span>;

<span class="kw">const </span>CURRENT_THREAD_ALREADY_IN_POOL: <span class="kw-2">&amp;</span>str =
    <span class="string">"The current thread is already part of another thread pool."</span>;

<span class="kw">impl </span>Error <span class="kw">for </span>ThreadPoolBuildError {
    <span class="attr">#[allow(deprecated)]
    </span><span class="kw">fn </span>description(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>str {
        <span class="kw">match </span><span class="self">self</span>.kind {
            ErrorKind::GlobalPoolAlreadyInitialized =&gt; GLOBAL_POOL_ALREADY_INITIALIZED,
            ErrorKind::CurrentThreadAlreadyInPool =&gt; CURRENT_THREAD_ALREADY_IN_POOL,
            ErrorKind::IOError(<span class="kw-2">ref </span>e) =&gt; e.description(),
        }
    }

    <span class="kw">fn </span>source(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>(<span class="kw">dyn </span>Error + <span class="lifetime">'static</span>)&gt; {
        <span class="kw">match </span><span class="kw-2">&amp;</span><span class="self">self</span>.kind {
            ErrorKind::GlobalPoolAlreadyInitialized | ErrorKind::CurrentThreadAlreadyInPool =&gt; <span class="prelude-val">None</span>,
            ErrorKind::IOError(e) =&gt; <span class="prelude-val">Some</span>(e),
        }
    }
}

<span class="kw">impl </span>fmt::Display <span class="kw">for </span>ThreadPoolBuildError {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        <span class="kw">match </span><span class="kw-2">&amp;</span><span class="self">self</span>.kind {
            ErrorKind::CurrentThreadAlreadyInPool =&gt; CURRENT_THREAD_ALREADY_IN_POOL.fmt(f),
            ErrorKind::GlobalPoolAlreadyInitialized =&gt; GLOBAL_POOL_ALREADY_INITIALIZED.fmt(f),
            ErrorKind::IOError(e) =&gt; e.fmt(f),
        }
    }
}

<span class="doccomment">/// Deprecated in favor of `ThreadPoolBuilder::build_global`.
</span><span class="attr">#[deprecated(note = <span class="string">"use `ThreadPoolBuilder::build_global`"</span>)]
#[allow(deprecated)]
</span><span class="kw">pub fn </span>initialize(config: Configuration) -&gt; <span class="prelude-ty">Result</span>&lt;(), Box&lt;<span class="kw">dyn </span>Error&gt;&gt; {
    config.into_builder().build_global().map_err(Box::from)
}

<span class="kw">impl</span>&lt;S&gt; fmt::Debug <span class="kw">for </span>ThreadPoolBuilder&lt;S&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        <span class="kw">let </span>ThreadPoolBuilder {
            <span class="kw-2">ref </span>num_threads,
            <span class="kw-2">ref </span>use_current_thread,
            <span class="kw-2">ref </span>get_thread_name,
            <span class="kw-2">ref </span>panic_handler,
            <span class="kw-2">ref </span>stack_size,
            <span class="kw-2">ref </span>start_handler,
            <span class="kw-2">ref </span>exit_handler,
            spawn_handler: <span class="kw">_</span>,
            <span class="kw-2">ref </span>breadth_first,
        } = <span class="kw-2">*</span><span class="self">self</span>;

        <span class="comment">// Just print `Some(&lt;closure&gt;)` or `None` to the debug
        // output.
        </span><span class="kw">struct </span>ClosurePlaceholder;
        <span class="kw">impl </span>fmt::Debug <span class="kw">for </span>ClosurePlaceholder {
            <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
                f.write_str(<span class="string">"&lt;closure&gt;"</span>)
            }
        }
        <span class="kw">let </span>get_thread_name = get_thread_name.as_ref().map(|<span class="kw">_</span>| ClosurePlaceholder);
        <span class="kw">let </span>panic_handler = panic_handler.as_ref().map(|<span class="kw">_</span>| ClosurePlaceholder);
        <span class="kw">let </span>start_handler = start_handler.as_ref().map(|<span class="kw">_</span>| ClosurePlaceholder);
        <span class="kw">let </span>exit_handler = exit_handler.as_ref().map(|<span class="kw">_</span>| ClosurePlaceholder);

        f.debug_struct(<span class="string">"ThreadPoolBuilder"</span>)
            .field(<span class="string">"num_threads"</span>, num_threads)
            .field(<span class="string">"use_current_thread"</span>, use_current_thread)
            .field(<span class="string">"get_thread_name"</span>, <span class="kw-2">&amp;</span>get_thread_name)
            .field(<span class="string">"panic_handler"</span>, <span class="kw-2">&amp;</span>panic_handler)
            .field(<span class="string">"stack_size"</span>, <span class="kw-2">&amp;</span>stack_size)
            .field(<span class="string">"start_handler"</span>, <span class="kw-2">&amp;</span>start_handler)
            .field(<span class="string">"exit_handler"</span>, <span class="kw-2">&amp;</span>exit_handler)
            .field(<span class="string">"breadth_first"</span>, <span class="kw-2">&amp;</span>breadth_first)
            .finish()
    }
}

<span class="attr">#[allow(deprecated)]
</span><span class="kw">impl </span>fmt::Debug <span class="kw">for </span>Configuration {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        <span class="self">self</span>.builder.fmt(f)
    }
}

<span class="doccomment">/// Provides the calling context to a closure called by `join_context`.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>FnContext {
    migrated: bool,

    <span class="doccomment">/// disable `Send` and `Sync`, just for a little future-proofing.
    </span>_marker: PhantomData&lt;<span class="kw-2">*mut </span>()&gt;,
}

<span class="kw">impl </span>FnContext {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>new(migrated: bool) -&gt; <span class="self">Self </span>{
        FnContext {
            migrated,
            _marker: PhantomData,
        }
    }
}

<span class="kw">impl </span>FnContext {
    <span class="doccomment">/// Returns `true` if the closure was called from a different thread
    /// than it was provided from.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub fn </span>migrated(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.migrated
    }
}
</code></pre></div></section></main></body></html>