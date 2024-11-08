<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rayon-1.10.0/src/iter/mod.rs`."><title>mod.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="rayon" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../rayon/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#1227" id="1227">1227</a>
<a href="#1228" id="1228">1228</a>
<a href="#1229" id="1229">1229</a>
<a href="#1230" id="1230">1230</a>
<a href="#1231" id="1231">1231</a>
<a href="#1232" id="1232">1232</a>
<a href="#1233" id="1233">1233</a>
<a href="#1234" id="1234">1234</a>
<a href="#1235" id="1235">1235</a>
<a href="#1236" id="1236">1236</a>
<a href="#1237" id="1237">1237</a>
<a href="#1238" id="1238">1238</a>
<a href="#1239" id="1239">1239</a>
<a href="#1240" id="1240">1240</a>
<a href="#1241" id="1241">1241</a>
<a href="#1242" id="1242">1242</a>
<a href="#1243" id="1243">1243</a>
<a href="#1244" id="1244">1244</a>
<a href="#1245" id="1245">1245</a>
<a href="#1246" id="1246">1246</a>
<a href="#1247" id="1247">1247</a>
<a href="#1248" id="1248">1248</a>
<a href="#1249" id="1249">1249</a>
<a href="#1250" id="1250">1250</a>
<a href="#1251" id="1251">1251</a>
<a href="#1252" id="1252">1252</a>
<a href="#1253" id="1253">1253</a>
<a href="#1254" id="1254">1254</a>
<a href="#1255" id="1255">1255</a>
<a href="#1256" id="1256">1256</a>
<a href="#1257" id="1257">1257</a>
<a href="#1258" id="1258">1258</a>
<a href="#1259" id="1259">1259</a>
<a href="#1260" id="1260">1260</a>
<a href="#1261" id="1261">1261</a>
<a href="#1262" id="1262">1262</a>
<a href="#1263" id="1263">1263</a>
<a href="#1264" id="1264">1264</a>
<a href="#1265" id="1265">1265</a>
<a href="#1266" id="1266">1266</a>
<a href="#1267" id="1267">1267</a>
<a href="#1268" id="1268">1268</a>
<a href="#1269" id="1269">1269</a>
<a href="#1270" id="1270">1270</a>
<a href="#1271" id="1271">1271</a>
<a href="#1272" id="1272">1272</a>
<a href="#1273" id="1273">1273</a>
<a href="#1274" id="1274">1274</a>
<a href="#1275" id="1275">1275</a>
<a href="#1276" id="1276">1276</a>
<a href="#1277" id="1277">1277</a>
<a href="#1278" id="1278">1278</a>
<a href="#1279" id="1279">1279</a>
<a href="#1280" id="1280">1280</a>
<a href="#1281" id="1281">1281</a>
<a href="#1282" id="1282">1282</a>
<a href="#1283" id="1283">1283</a>
<a href="#1284" id="1284">1284</a>
<a href="#1285" id="1285">1285</a>
<a href="#1286" id="1286">1286</a>
<a href="#1287" id="1287">1287</a>
<a href="#1288" id="1288">1288</a>
<a href="#1289" id="1289">1289</a>
<a href="#1290" id="1290">1290</a>
<a href="#1291" id="1291">1291</a>
<a href="#1292" id="1292">1292</a>
<a href="#1293" id="1293">1293</a>
<a href="#1294" id="1294">1294</a>
<a href="#1295" id="1295">1295</a>
<a href="#1296" id="1296">1296</a>
<a href="#1297" id="1297">1297</a>
<a href="#1298" id="1298">1298</a>
<a href="#1299" id="1299">1299</a>
<a href="#1300" id="1300">1300</a>
<a href="#1301" id="1301">1301</a>
<a href="#1302" id="1302">1302</a>
<a href="#1303" id="1303">1303</a>
<a href="#1304" id="1304">1304</a>
<a href="#1305" id="1305">1305</a>
<a href="#1306" id="1306">1306</a>
<a href="#1307" id="1307">1307</a>
<a href="#1308" id="1308">1308</a>
<a href="#1309" id="1309">1309</a>
<a href="#1310" id="1310">1310</a>
<a href="#1311" id="1311">1311</a>
<a href="#1312" id="1312">1312</a>
<a href="#1313" id="1313">1313</a>
<a href="#1314" id="1314">1314</a>
<a href="#1315" id="1315">1315</a>
<a href="#1316" id="1316">1316</a>
<a href="#1317" id="1317">1317</a>
<a href="#1318" id="1318">1318</a>
<a href="#1319" id="1319">1319</a>
<a href="#1320" id="1320">1320</a>
<a href="#1321" id="1321">1321</a>
<a href="#1322" id="1322">1322</a>
<a href="#1323" id="1323">1323</a>
<a href="#1324" id="1324">1324</a>
<a href="#1325" id="1325">1325</a>
<a href="#1326" id="1326">1326</a>
<a href="#1327" id="1327">1327</a>
<a href="#1328" id="1328">1328</a>
<a href="#1329" id="1329">1329</a>
<a href="#1330" id="1330">1330</a>
<a href="#1331" id="1331">1331</a>
<a href="#1332" id="1332">1332</a>
<a href="#1333" id="1333">1333</a>
<a href="#1334" id="1334">1334</a>
<a href="#1335" id="1335">1335</a>
<a href="#1336" id="1336">1336</a>
<a href="#1337" id="1337">1337</a>
<a href="#1338" id="1338">1338</a>
<a href="#1339" id="1339">1339</a>
<a href="#1340" id="1340">1340</a>
<a href="#1341" id="1341">1341</a>
<a href="#1342" id="1342">1342</a>
<a href="#1343" id="1343">1343</a>
<a href="#1344" id="1344">1344</a>
<a href="#1345" id="1345">1345</a>
<a href="#1346" id="1346">1346</a>
<a href="#1347" id="1347">1347</a>
<a href="#1348" id="1348">1348</a>
<a href="#1349" id="1349">1349</a>
<a href="#1350" id="1350">1350</a>
<a href="#1351" id="1351">1351</a>
<a href="#1352" id="1352">1352</a>
<a href="#1353" id="1353">1353</a>
<a href="#1354" id="1354">1354</a>
<a href="#1355" id="1355">1355</a>
<a href="#1356" id="1356">1356</a>
<a href="#1357" id="1357">1357</a>
<a href="#1358" id="1358">1358</a>
<a href="#1359" id="1359">1359</a>
<a href="#1360" id="1360">1360</a>
<a href="#1361" id="1361">1361</a>
<a href="#1362" id="1362">1362</a>
<a href="#1363" id="1363">1363</a>
<a href="#1364" id="1364">1364</a>
<a href="#1365" id="1365">1365</a>
<a href="#1366" id="1366">1366</a>
<a href="#1367" id="1367">1367</a>
<a href="#1368" id="1368">1368</a>
<a href="#1369" id="1369">1369</a>
<a href="#1370" id="1370">1370</a>
<a href="#1371" id="1371">1371</a>
<a href="#1372" id="1372">1372</a>
<a href="#1373" id="1373">1373</a>
<a href="#1374" id="1374">1374</a>
<a href="#1375" id="1375">1375</a>
<a href="#1376" id="1376">1376</a>
<a href="#1377" id="1377">1377</a>
<a href="#1378" id="1378">1378</a>
<a href="#1379" id="1379">1379</a>
<a href="#1380" id="1380">1380</a>
<a href="#1381" id="1381">1381</a>
<a href="#1382" id="1382">1382</a>
<a href="#1383" id="1383">1383</a>
<a href="#1384" id="1384">1384</a>
<a href="#1385" id="1385">1385</a>
<a href="#1386" id="1386">1386</a>
<a href="#1387" id="1387">1387</a>
<a href="#1388" id="1388">1388</a>
<a href="#1389" id="1389">1389</a>
<a href="#1390" id="1390">1390</a>
<a href="#1391" id="1391">1391</a>
<a href="#1392" id="1392">1392</a>
<a href="#1393" id="1393">1393</a>
<a href="#1394" id="1394">1394</a>
<a href="#1395" id="1395">1395</a>
<a href="#1396" id="1396">1396</a>
<a href="#1397" id="1397">1397</a>
<a href="#1398" id="1398">1398</a>
<a href="#1399" id="1399">1399</a>
<a href="#1400" id="1400">1400</a>
<a href="#1401" id="1401">1401</a>
<a href="#1402" id="1402">1402</a>
<a href="#1403" id="1403">1403</a>
<a href="#1404" id="1404">1404</a>
<a href="#1405" id="1405">1405</a>
<a href="#1406" id="1406">1406</a>
<a href="#1407" id="1407">1407</a>
<a href="#1408" id="1408">1408</a>
<a href="#1409" id="1409">1409</a>
<a href="#1410" id="1410">1410</a>
<a href="#1411" id="1411">1411</a>
<a href="#1412" id="1412">1412</a>
<a href="#1413" id="1413">1413</a>
<a href="#1414" id="1414">1414</a>
<a href="#1415" id="1415">1415</a>
<a href="#1416" id="1416">1416</a>
<a href="#1417" id="1417">1417</a>
<a href="#1418" id="1418">1418</a>
<a href="#1419" id="1419">1419</a>
<a href="#1420" id="1420">1420</a>
<a href="#1421" id="1421">1421</a>
<a href="#1422" id="1422">1422</a>
<a href="#1423" id="1423">1423</a>
<a href="#1424" id="1424">1424</a>
<a href="#1425" id="1425">1425</a>
<a href="#1426" id="1426">1426</a>
<a href="#1427" id="1427">1427</a>
<a href="#1428" id="1428">1428</a>
<a href="#1429" id="1429">1429</a>
<a href="#1430" id="1430">1430</a>
<a href="#1431" id="1431">1431</a>
<a href="#1432" id="1432">1432</a>
<a href="#1433" id="1433">1433</a>
<a href="#1434" id="1434">1434</a>
<a href="#1435" id="1435">1435</a>
<a href="#1436" id="1436">1436</a>
<a href="#1437" id="1437">1437</a>
<a href="#1438" id="1438">1438</a>
<a href="#1439" id="1439">1439</a>
<a href="#1440" id="1440">1440</a>
<a href="#1441" id="1441">1441</a>
<a href="#1442" id="1442">1442</a>
<a href="#1443" id="1443">1443</a>
<a href="#1444" id="1444">1444</a>
<a href="#1445" id="1445">1445</a>
<a href="#1446" id="1446">1446</a>
<a href="#1447" id="1447">1447</a>
<a href="#1448" id="1448">1448</a>
<a href="#1449" id="1449">1449</a>
<a href="#1450" id="1450">1450</a>
<a href="#1451" id="1451">1451</a>
<a href="#1452" id="1452">1452</a>
<a href="#1453" id="1453">1453</a>
<a href="#1454" id="1454">1454</a>
<a href="#1455" id="1455">1455</a>
<a href="#1456" id="1456">1456</a>
<a href="#1457" id="1457">1457</a>
<a href="#1458" id="1458">1458</a>
<a href="#1459" id="1459">1459</a>
<a href="#1460" id="1460">1460</a>
<a href="#1461" id="1461">1461</a>
<a href="#1462" id="1462">1462</a>
<a href="#1463" id="1463">1463</a>
<a href="#1464" id="1464">1464</a>
<a href="#1465" id="1465">1465</a>
<a href="#1466" id="1466">1466</a>
<a href="#1467" id="1467">1467</a>
<a href="#1468" id="1468">1468</a>
<a href="#1469" id="1469">1469</a>
<a href="#1470" id="1470">1470</a>
<a href="#1471" id="1471">1471</a>
<a href="#1472" id="1472">1472</a>
<a href="#1473" id="1473">1473</a>
<a href="#1474" id="1474">1474</a>
<a href="#1475" id="1475">1475</a>
<a href="#1476" id="1476">1476</a>
<a href="#1477" id="1477">1477</a>
<a href="#1478" id="1478">1478</a>
<a href="#1479" id="1479">1479</a>
<a href="#1480" id="1480">1480</a>
<a href="#1481" id="1481">1481</a>
<a href="#1482" id="1482">1482</a>
<a href="#1483" id="1483">1483</a>
<a href="#1484" id="1484">1484</a>
<a href="#1485" id="1485">1485</a>
<a href="#1486" id="1486">1486</a>
<a href="#1487" id="1487">1487</a>
<a href="#1488" id="1488">1488</a>
<a href="#1489" id="1489">1489</a>
<a href="#1490" id="1490">1490</a>
<a href="#1491" id="1491">1491</a>
<a href="#1492" id="1492">1492</a>
<a href="#1493" id="1493">1493</a>
<a href="#1494" id="1494">1494</a>
<a href="#1495" id="1495">1495</a>
<a href="#1496" id="1496">1496</a>
<a href="#1497" id="1497">1497</a>
<a href="#1498" id="1498">1498</a>
<a href="#1499" id="1499">1499</a>
<a href="#1500" id="1500">1500</a>
<a href="#1501" id="1501">1501</a>
<a href="#1502" id="1502">1502</a>
<a href="#1503" id="1503">1503</a>
<a href="#1504" id="1504">1504</a>
<a href="#1505" id="1505">1505</a>
<a href="#1506" id="1506">1506</a>
<a href="#1507" id="1507">1507</a>
<a href="#1508" id="1508">1508</a>
<a href="#1509" id="1509">1509</a>
<a href="#1510" id="1510">1510</a>
<a href="#1511" id="1511">1511</a>
<a href="#1512" id="1512">1512</a>
<a href="#1513" id="1513">1513</a>
<a href="#1514" id="1514">1514</a>
<a href="#1515" id="1515">1515</a>
<a href="#1516" id="1516">1516</a>
<a href="#1517" id="1517">1517</a>
<a href="#1518" id="1518">1518</a>
<a href="#1519" id="1519">1519</a>
<a href="#1520" id="1520">1520</a>
<a href="#1521" id="1521">1521</a>
<a href="#1522" id="1522">1522</a>
<a href="#1523" id="1523">1523</a>
<a href="#1524" id="1524">1524</a>
<a href="#1525" id="1525">1525</a>
<a href="#1526" id="1526">1526</a>
<a href="#1527" id="1527">1527</a>
<a href="#1528" id="1528">1528</a>
<a href="#1529" id="1529">1529</a>
<a href="#1530" id="1530">1530</a>
<a href="#1531" id="1531">1531</a>
<a href="#1532" id="1532">1532</a>
<a href="#1533" id="1533">1533</a>
<a href="#1534" id="1534">1534</a>
<a href="#1535" id="1535">1535</a>
<a href="#1536" id="1536">1536</a>
<a href="#1537" id="1537">1537</a>
<a href="#1538" id="1538">1538</a>
<a href="#1539" id="1539">1539</a>
<a href="#1540" id="1540">1540</a>
<a href="#1541" id="1541">1541</a>
<a href="#1542" id="1542">1542</a>
<a href="#1543" id="1543">1543</a>
<a href="#1544" id="1544">1544</a>
<a href="#1545" id="1545">1545</a>
<a href="#1546" id="1546">1546</a>
<a href="#1547" id="1547">1547</a>
<a href="#1548" id="1548">1548</a>
<a href="#1549" id="1549">1549</a>
<a href="#1550" id="1550">1550</a>
<a href="#1551" id="1551">1551</a>
<a href="#1552" id="1552">1552</a>
<a href="#1553" id="1553">1553</a>
<a href="#1554" id="1554">1554</a>
<a href="#1555" id="1555">1555</a>
<a href="#1556" id="1556">1556</a>
<a href="#1557" id="1557">1557</a>
<a href="#1558" id="1558">1558</a>
<a href="#1559" id="1559">1559</a>
<a href="#1560" id="1560">1560</a>
<a href="#1561" id="1561">1561</a>
<a href="#1562" id="1562">1562</a>
<a href="#1563" id="1563">1563</a>
<a href="#1564" id="1564">1564</a>
<a href="#1565" id="1565">1565</a>
<a href="#1566" id="1566">1566</a>
<a href="#1567" id="1567">1567</a>
<a href="#1568" id="1568">1568</a>
<a href="#1569" id="1569">1569</a>
<a href="#1570" id="1570">1570</a>
<a href="#1571" id="1571">1571</a>
<a href="#1572" id="1572">1572</a>
<a href="#1573" id="1573">1573</a>
<a href="#1574" id="1574">1574</a>
<a href="#1575" id="1575">1575</a>
<a href="#1576" id="1576">1576</a>
<a href="#1577" id="1577">1577</a>
<a href="#1578" id="1578">1578</a>
<a href="#1579" id="1579">1579</a>
<a href="#1580" id="1580">1580</a>
<a href="#1581" id="1581">1581</a>
<a href="#1582" id="1582">1582</a>
<a href="#1583" id="1583">1583</a>
<a href="#1584" id="1584">1584</a>
<a href="#1585" id="1585">1585</a>
<a href="#1586" id="1586">1586</a>
<a href="#1587" id="1587">1587</a>
<a href="#1588" id="1588">1588</a>
<a href="#1589" id="1589">1589</a>
<a href="#1590" id="1590">1590</a>
<a href="#1591" id="1591">1591</a>
<a href="#1592" id="1592">1592</a>
<a href="#1593" id="1593">1593</a>
<a href="#1594" id="1594">1594</a>
<a href="#1595" id="1595">1595</a>
<a href="#1596" id="1596">1596</a>
<a href="#1597" id="1597">1597</a>
<a href="#1598" id="1598">1598</a>
<a href="#1599" id="1599">1599</a>
<a href="#1600" id="1600">1600</a>
<a href="#1601" id="1601">1601</a>
<a href="#1602" id="1602">1602</a>
<a href="#1603" id="1603">1603</a>
<a href="#1604" id="1604">1604</a>
<a href="#1605" id="1605">1605</a>
<a href="#1606" id="1606">1606</a>
<a href="#1607" id="1607">1607</a>
<a href="#1608" id="1608">1608</a>
<a href="#1609" id="1609">1609</a>
<a href="#1610" id="1610">1610</a>
<a href="#1611" id="1611">1611</a>
<a href="#1612" id="1612">1612</a>
<a href="#1613" id="1613">1613</a>
<a href="#1614" id="1614">1614</a>
<a href="#1615" id="1615">1615</a>
<a href="#1616" id="1616">1616</a>
<a href="#1617" id="1617">1617</a>
<a href="#1618" id="1618">1618</a>
<a href="#1619" id="1619">1619</a>
<a href="#1620" id="1620">1620</a>
<a href="#1621" id="1621">1621</a>
<a href="#1622" id="1622">1622</a>
<a href="#1623" id="1623">1623</a>
<a href="#1624" id="1624">1624</a>
<a href="#1625" id="1625">1625</a>
<a href="#1626" id="1626">1626</a>
<a href="#1627" id="1627">1627</a>
<a href="#1628" id="1628">1628</a>
<a href="#1629" id="1629">1629</a>
<a href="#1630" id="1630">1630</a>
<a href="#1631" id="1631">1631</a>
<a href="#1632" id="1632">1632</a>
<a href="#1633" id="1633">1633</a>
<a href="#1634" id="1634">1634</a>
<a href="#1635" id="1635">1635</a>
<a href="#1636" id="1636">1636</a>
<a href="#1637" id="1637">1637</a>
<a href="#1638" id="1638">1638</a>
<a href="#1639" id="1639">1639</a>
<a href="#1640" id="1640">1640</a>
<a href="#1641" id="1641">1641</a>
<a href="#1642" id="1642">1642</a>
<a href="#1643" id="1643">1643</a>
<a href="#1644" id="1644">1644</a>
<a href="#1645" id="1645">1645</a>
<a href="#1646" id="1646">1646</a>
<a href="#1647" id="1647">1647</a>
<a href="#1648" id="1648">1648</a>
<a href="#1649" id="1649">1649</a>
<a href="#1650" id="1650">1650</a>
<a href="#1651" id="1651">1651</a>
<a href="#1652" id="1652">1652</a>
<a href="#1653" id="1653">1653</a>
<a href="#1654" id="1654">1654</a>
<a href="#1655" id="1655">1655</a>
<a href="#1656" id="1656">1656</a>
<a href="#1657" id="1657">1657</a>
<a href="#1658" id="1658">1658</a>
<a href="#1659" id="1659">1659</a>
<a href="#1660" id="1660">1660</a>
<a href="#1661" id="1661">1661</a>
<a href="#1662" id="1662">1662</a>
<a href="#1663" id="1663">1663</a>
<a href="#1664" id="1664">1664</a>
<a href="#1665" id="1665">1665</a>
<a href="#1666" id="1666">1666</a>
<a href="#1667" id="1667">1667</a>
<a href="#1668" id="1668">1668</a>
<a href="#1669" id="1669">1669</a>
<a href="#1670" id="1670">1670</a>
<a href="#1671" id="1671">1671</a>
<a href="#1672" id="1672">1672</a>
<a href="#1673" id="1673">1673</a>
<a href="#1674" id="1674">1674</a>
<a href="#1675" id="1675">1675</a>
<a href="#1676" id="1676">1676</a>
<a href="#1677" id="1677">1677</a>
<a href="#1678" id="1678">1678</a>
<a href="#1679" id="1679">1679</a>
<a href="#1680" id="1680">1680</a>
<a href="#1681" id="1681">1681</a>
<a href="#1682" id="1682">1682</a>
<a href="#1683" id="1683">1683</a>
<a href="#1684" id="1684">1684</a>
<a href="#1685" id="1685">1685</a>
<a href="#1686" id="1686">1686</a>
<a href="#1687" id="1687">1687</a>
<a href="#1688" id="1688">1688</a>
<a href="#1689" id="1689">1689</a>
<a href="#1690" id="1690">1690</a>
<a href="#1691" id="1691">1691</a>
<a href="#1692" id="1692">1692</a>
<a href="#1693" id="1693">1693</a>
<a href="#1694" id="1694">1694</a>
<a href="#1695" id="1695">1695</a>
<a href="#1696" id="1696">1696</a>
<a href="#1697" id="1697">1697</a>
<a href="#1698" id="1698">1698</a>
<a href="#1699" id="1699">1699</a>
<a href="#1700" id="1700">1700</a>
<a href="#1701" id="1701">1701</a>
<a href="#1702" id="1702">1702</a>
<a href="#1703" id="1703">1703</a>
<a href="#1704" id="1704">1704</a>
<a href="#1705" id="1705">1705</a>
<a href="#1706" id="1706">1706</a>
<a href="#1707" id="1707">1707</a>
<a href="#1708" id="1708">1708</a>
<a href="#1709" id="1709">1709</a>
<a href="#1710" id="1710">1710</a>
<a href="#1711" id="1711">1711</a>
<a href="#1712" id="1712">1712</a>
<a href="#1713" id="1713">1713</a>
<a href="#1714" id="1714">1714</a>
<a href="#1715" id="1715">1715</a>
<a href="#1716" id="1716">1716</a>
<a href="#1717" id="1717">1717</a>
<a href="#1718" id="1718">1718</a>
<a href="#1719" id="1719">1719</a>
<a href="#1720" id="1720">1720</a>
<a href="#1721" id="1721">1721</a>
<a href="#1722" id="1722">1722</a>
<a href="#1723" id="1723">1723</a>
<a href="#1724" id="1724">1724</a>
<a href="#1725" id="1725">1725</a>
<a href="#1726" id="1726">1726</a>
<a href="#1727" id="1727">1727</a>
<a href="#1728" id="1728">1728</a>
<a href="#1729" id="1729">1729</a>
<a href="#1730" id="1730">1730</a>
<a href="#1731" id="1731">1731</a>
<a href="#1732" id="1732">1732</a>
<a href="#1733" id="1733">1733</a>
<a href="#1734" id="1734">1734</a>
<a href="#1735" id="1735">1735</a>
<a href="#1736" id="1736">1736</a>
<a href="#1737" id="1737">1737</a>
<a href="#1738" id="1738">1738</a>
<a href="#1739" id="1739">1739</a>
<a href="#1740" id="1740">1740</a>
<a href="#1741" id="1741">1741</a>
<a href="#1742" id="1742">1742</a>
<a href="#1743" id="1743">1743</a>
<a href="#1744" id="1744">1744</a>
<a href="#1745" id="1745">1745</a>
<a href="#1746" id="1746">1746</a>
<a href="#1747" id="1747">1747</a>
<a href="#1748" id="1748">1748</a>
<a href="#1749" id="1749">1749</a>
<a href="#1750" id="1750">1750</a>
<a href="#1751" id="1751">1751</a>
<a href="#1752" id="1752">1752</a>
<a href="#1753" id="1753">1753</a>
<a href="#1754" id="1754">1754</a>
<a href="#1755" id="1755">1755</a>
<a href="#1756" id="1756">1756</a>
<a href="#1757" id="1757">1757</a>
<a href="#1758" id="1758">1758</a>
<a href="#1759" id="1759">1759</a>
<a href="#1760" id="1760">1760</a>
<a href="#1761" id="1761">1761</a>
<a href="#1762" id="1762">1762</a>
<a href="#1763" id="1763">1763</a>
<a href="#1764" id="1764">1764</a>
<a href="#1765" id="1765">1765</a>
<a href="#1766" id="1766">1766</a>
<a href="#1767" id="1767">1767</a>
<a href="#1768" id="1768">1768</a>
<a href="#1769" id="1769">1769</a>
<a href="#1770" id="1770">1770</a>
<a href="#1771" id="1771">1771</a>
<a href="#1772" id="1772">1772</a>
<a href="#1773" id="1773">1773</a>
<a href="#1774" id="1774">1774</a>
<a href="#1775" id="1775">1775</a>
<a href="#1776" id="1776">1776</a>
<a href="#1777" id="1777">1777</a>
<a href="#1778" id="1778">1778</a>
<a href="#1779" id="1779">1779</a>
<a href="#1780" id="1780">1780</a>
<a href="#1781" id="1781">1781</a>
<a href="#1782" id="1782">1782</a>
<a href="#1783" id="1783">1783</a>
<a href="#1784" id="1784">1784</a>
<a href="#1785" id="1785">1785</a>
<a href="#1786" id="1786">1786</a>
<a href="#1787" id="1787">1787</a>
<a href="#1788" id="1788">1788</a>
<a href="#1789" id="1789">1789</a>
<a href="#1790" id="1790">1790</a>
<a href="#1791" id="1791">1791</a>
<a href="#1792" id="1792">1792</a>
<a href="#1793" id="1793">1793</a>
<a href="#1794" id="1794">1794</a>
<a href="#1795" id="1795">1795</a>
<a href="#1796" id="1796">1796</a>
<a href="#1797" id="1797">1797</a>
<a href="#1798" id="1798">1798</a>
<a href="#1799" id="1799">1799</a>
<a href="#1800" id="1800">1800</a>
<a href="#1801" id="1801">1801</a>
<a href="#1802" id="1802">1802</a>
<a href="#1803" id="1803">1803</a>
<a href="#1804" id="1804">1804</a>
<a href="#1805" id="1805">1805</a>
<a href="#1806" id="1806">1806</a>
<a href="#1807" id="1807">1807</a>
<a href="#1808" id="1808">1808</a>
<a href="#1809" id="1809">1809</a>
<a href="#1810" id="1810">1810</a>
<a href="#1811" id="1811">1811</a>
<a href="#1812" id="1812">1812</a>
<a href="#1813" id="1813">1813</a>
<a href="#1814" id="1814">1814</a>
<a href="#1815" id="1815">1815</a>
<a href="#1816" id="1816">1816</a>
<a href="#1817" id="1817">1817</a>
<a href="#1818" id="1818">1818</a>
<a href="#1819" id="1819">1819</a>
<a href="#1820" id="1820">1820</a>
<a href="#1821" id="1821">1821</a>
<a href="#1822" id="1822">1822</a>
<a href="#1823" id="1823">1823</a>
<a href="#1824" id="1824">1824</a>
<a href="#1825" id="1825">1825</a>
<a href="#1826" id="1826">1826</a>
<a href="#1827" id="1827">1827</a>
<a href="#1828" id="1828">1828</a>
<a href="#1829" id="1829">1829</a>
<a href="#1830" id="1830">1830</a>
<a href="#1831" id="1831">1831</a>
<a href="#1832" id="1832">1832</a>
<a href="#1833" id="1833">1833</a>
<a href="#1834" id="1834">1834</a>
<a href="#1835" id="1835">1835</a>
<a href="#1836" id="1836">1836</a>
<a href="#1837" id="1837">1837</a>
<a href="#1838" id="1838">1838</a>
<a href="#1839" id="1839">1839</a>
<a href="#1840" id="1840">1840</a>
<a href="#1841" id="1841">1841</a>
<a href="#1842" id="1842">1842</a>
<a href="#1843" id="1843">1843</a>
<a href="#1844" id="1844">1844</a>
<a href="#1845" id="1845">1845</a>
<a href="#1846" id="1846">1846</a>
<a href="#1847" id="1847">1847</a>
<a href="#1848" id="1848">1848</a>
<a href="#1849" id="1849">1849</a>
<a href="#1850" id="1850">1850</a>
<a href="#1851" id="1851">1851</a>
<a href="#1852" id="1852">1852</a>
<a href="#1853" id="1853">1853</a>
<a href="#1854" id="1854">1854</a>
<a href="#1855" id="1855">1855</a>
<a href="#1856" id="1856">1856</a>
<a href="#1857" id="1857">1857</a>
<a href="#1858" id="1858">1858</a>
<a href="#1859" id="1859">1859</a>
<a href="#1860" id="1860">1860</a>
<a href="#1861" id="1861">1861</a>
<a href="#1862" id="1862">1862</a>
<a href="#1863" id="1863">1863</a>
<a href="#1864" id="1864">1864</a>
<a href="#1865" id="1865">1865</a>
<a href="#1866" id="1866">1866</a>
<a href="#1867" id="1867">1867</a>
<a href="#1868" id="1868">1868</a>
<a href="#1869" id="1869">1869</a>
<a href="#1870" id="1870">1870</a>
<a href="#1871" id="1871">1871</a>
<a href="#1872" id="1872">1872</a>
<a href="#1873" id="1873">1873</a>
<a href="#1874" id="1874">1874</a>
<a href="#1875" id="1875">1875</a>
<a href="#1876" id="1876">1876</a>
<a href="#1877" id="1877">1877</a>
<a href="#1878" id="1878">1878</a>
<a href="#1879" id="1879">1879</a>
<a href="#1880" id="1880">1880</a>
<a href="#1881" id="1881">1881</a>
<a href="#1882" id="1882">1882</a>
<a href="#1883" id="1883">1883</a>
<a href="#1884" id="1884">1884</a>
<a href="#1885" id="1885">1885</a>
<a href="#1886" id="1886">1886</a>
<a href="#1887" id="1887">1887</a>
<a href="#1888" id="1888">1888</a>
<a href="#1889" id="1889">1889</a>
<a href="#1890" id="1890">1890</a>
<a href="#1891" id="1891">1891</a>
<a href="#1892" id="1892">1892</a>
<a href="#1893" id="1893">1893</a>
<a href="#1894" id="1894">1894</a>
<a href="#1895" id="1895">1895</a>
<a href="#1896" id="1896">1896</a>
<a href="#1897" id="1897">1897</a>
<a href="#1898" id="1898">1898</a>
<a href="#1899" id="1899">1899</a>
<a href="#1900" id="1900">1900</a>
<a href="#1901" id="1901">1901</a>
<a href="#1902" id="1902">1902</a>
<a href="#1903" id="1903">1903</a>
<a href="#1904" id="1904">1904</a>
<a href="#1905" id="1905">1905</a>
<a href="#1906" id="1906">1906</a>
<a href="#1907" id="1907">1907</a>
<a href="#1908" id="1908">1908</a>
<a href="#1909" id="1909">1909</a>
<a href="#1910" id="1910">1910</a>
<a href="#1911" id="1911">1911</a>
<a href="#1912" id="1912">1912</a>
<a href="#1913" id="1913">1913</a>
<a href="#1914" id="1914">1914</a>
<a href="#1915" id="1915">1915</a>
<a href="#1916" id="1916">1916</a>
<a href="#1917" id="1917">1917</a>
<a href="#1918" id="1918">1918</a>
<a href="#1919" id="1919">1919</a>
<a href="#1920" id="1920">1920</a>
<a href="#1921" id="1921">1921</a>
<a href="#1922" id="1922">1922</a>
<a href="#1923" id="1923">1923</a>
<a href="#1924" id="1924">1924</a>
<a href="#1925" id="1925">1925</a>
<a href="#1926" id="1926">1926</a>
<a href="#1927" id="1927">1927</a>
<a href="#1928" id="1928">1928</a>
<a href="#1929" id="1929">1929</a>
<a href="#1930" id="1930">1930</a>
<a href="#1931" id="1931">1931</a>
<a href="#1932" id="1932">1932</a>
<a href="#1933" id="1933">1933</a>
<a href="#1934" id="1934">1934</a>
<a href="#1935" id="1935">1935</a>
<a href="#1936" id="1936">1936</a>
<a href="#1937" id="1937">1937</a>
<a href="#1938" id="1938">1938</a>
<a href="#1939" id="1939">1939</a>
<a href="#1940" id="1940">1940</a>
<a href="#1941" id="1941">1941</a>
<a href="#1942" id="1942">1942</a>
<a href="#1943" id="1943">1943</a>
<a href="#1944" id="1944">1944</a>
<a href="#1945" id="1945">1945</a>
<a href="#1946" id="1946">1946</a>
<a href="#1947" id="1947">1947</a>
<a href="#1948" id="1948">1948</a>
<a href="#1949" id="1949">1949</a>
<a href="#1950" id="1950">1950</a>
<a href="#1951" id="1951">1951</a>
<a href="#1952" id="1952">1952</a>
<a href="#1953" id="1953">1953</a>
<a href="#1954" id="1954">1954</a>
<a href="#1955" id="1955">1955</a>
<a href="#1956" id="1956">1956</a>
<a href="#1957" id="1957">1957</a>
<a href="#1958" id="1958">1958</a>
<a href="#1959" id="1959">1959</a>
<a href="#1960" id="1960">1960</a>
<a href="#1961" id="1961">1961</a>
<a href="#1962" id="1962">1962</a>
<a href="#1963" id="1963">1963</a>
<a href="#1964" id="1964">1964</a>
<a href="#1965" id="1965">1965</a>
<a href="#1966" id="1966">1966</a>
<a href="#1967" id="1967">1967</a>
<a href="#1968" id="1968">1968</a>
<a href="#1969" id="1969">1969</a>
<a href="#1970" id="1970">1970</a>
<a href="#1971" id="1971">1971</a>
<a href="#1972" id="1972">1972</a>
<a href="#1973" id="1973">1973</a>
<a href="#1974" id="1974">1974</a>
<a href="#1975" id="1975">1975</a>
<a href="#1976" id="1976">1976</a>
<a href="#1977" id="1977">1977</a>
<a href="#1978" id="1978">1978</a>
<a href="#1979" id="1979">1979</a>
<a href="#1980" id="1980">1980</a>
<a href="#1981" id="1981">1981</a>
<a href="#1982" id="1982">1982</a>
<a href="#1983" id="1983">1983</a>
<a href="#1984" id="1984">1984</a>
<a href="#1985" id="1985">1985</a>
<a href="#1986" id="1986">1986</a>
<a href="#1987" id="1987">1987</a>
<a href="#1988" id="1988">1988</a>
<a href="#1989" id="1989">1989</a>
<a href="#1990" id="1990">1990</a>
<a href="#1991" id="1991">1991</a>
<a href="#1992" id="1992">1992</a>
<a href="#1993" id="1993">1993</a>
<a href="#1994" id="1994">1994</a>
<a href="#1995" id="1995">1995</a>
<a href="#1996" id="1996">1996</a>
<a href="#1997" id="1997">1997</a>
<a href="#1998" id="1998">1998</a>
<a href="#1999" id="1999">1999</a>
<a href="#2000" id="2000">2000</a>
<a href="#2001" id="2001">2001</a>
<a href="#2002" id="2002">2002</a>
<a href="#2003" id="2003">2003</a>
<a href="#2004" id="2004">2004</a>
<a href="#2005" id="2005">2005</a>
<a href="#2006" id="2006">2006</a>
<a href="#2007" id="2007">2007</a>
<a href="#2008" id="2008">2008</a>
<a href="#2009" id="2009">2009</a>
<a href="#2010" id="2010">2010</a>
<a href="#2011" id="2011">2011</a>
<a href="#2012" id="2012">2012</a>
<a href="#2013" id="2013">2013</a>
<a href="#2014" id="2014">2014</a>
<a href="#2015" id="2015">2015</a>
<a href="#2016" id="2016">2016</a>
<a href="#2017" id="2017">2017</a>
<a href="#2018" id="2018">2018</a>
<a href="#2019" id="2019">2019</a>
<a href="#2020" id="2020">2020</a>
<a href="#2021" id="2021">2021</a>
<a href="#2022" id="2022">2022</a>
<a href="#2023" id="2023">2023</a>
<a href="#2024" id="2024">2024</a>
<a href="#2025" id="2025">2025</a>
<a href="#2026" id="2026">2026</a>
<a href="#2027" id="2027">2027</a>
<a href="#2028" id="2028">2028</a>
<a href="#2029" id="2029">2029</a>
<a href="#2030" id="2030">2030</a>
<a href="#2031" id="2031">2031</a>
<a href="#2032" id="2032">2032</a>
<a href="#2033" id="2033">2033</a>
<a href="#2034" id="2034">2034</a>
<a href="#2035" id="2035">2035</a>
<a href="#2036" id="2036">2036</a>
<a href="#2037" id="2037">2037</a>
<a href="#2038" id="2038">2038</a>
<a href="#2039" id="2039">2039</a>
<a href="#2040" id="2040">2040</a>
<a href="#2041" id="2041">2041</a>
<a href="#2042" id="2042">2042</a>
<a href="#2043" id="2043">2043</a>
<a href="#2044" id="2044">2044</a>
<a href="#2045" id="2045">2045</a>
<a href="#2046" id="2046">2046</a>
<a href="#2047" id="2047">2047</a>
<a href="#2048" id="2048">2048</a>
<a href="#2049" id="2049">2049</a>
<a href="#2050" id="2050">2050</a>
<a href="#2051" id="2051">2051</a>
<a href="#2052" id="2052">2052</a>
<a href="#2053" id="2053">2053</a>
<a href="#2054" id="2054">2054</a>
<a href="#2055" id="2055">2055</a>
<a href="#2056" id="2056">2056</a>
<a href="#2057" id="2057">2057</a>
<a href="#2058" id="2058">2058</a>
<a href="#2059" id="2059">2059</a>
<a href="#2060" id="2060">2060</a>
<a href="#2061" id="2061">2061</a>
<a href="#2062" id="2062">2062</a>
<a href="#2063" id="2063">2063</a>
<a href="#2064" id="2064">2064</a>
<a href="#2065" id="2065">2065</a>
<a href="#2066" id="2066">2066</a>
<a href="#2067" id="2067">2067</a>
<a href="#2068" id="2068">2068</a>
<a href="#2069" id="2069">2069</a>
<a href="#2070" id="2070">2070</a>
<a href="#2071" id="2071">2071</a>
<a href="#2072" id="2072">2072</a>
<a href="#2073" id="2073">2073</a>
<a href="#2074" id="2074">2074</a>
<a href="#2075" id="2075">2075</a>
<a href="#2076" id="2076">2076</a>
<a href="#2077" id="2077">2077</a>
<a href="#2078" id="2078">2078</a>
<a href="#2079" id="2079">2079</a>
<a href="#2080" id="2080">2080</a>
<a href="#2081" id="2081">2081</a>
<a href="#2082" id="2082">2082</a>
<a href="#2083" id="2083">2083</a>
<a href="#2084" id="2084">2084</a>
<a href="#2085" id="2085">2085</a>
<a href="#2086" id="2086">2086</a>
<a href="#2087" id="2087">2087</a>
<a href="#2088" id="2088">2088</a>
<a href="#2089" id="2089">2089</a>
<a href="#2090" id="2090">2090</a>
<a href="#2091" id="2091">2091</a>
<a href="#2092" id="2092">2092</a>
<a href="#2093" id="2093">2093</a>
<a href="#2094" id="2094">2094</a>
<a href="#2095" id="2095">2095</a>
<a href="#2096" id="2096">2096</a>
<a href="#2097" id="2097">2097</a>
<a href="#2098" id="2098">2098</a>
<a href="#2099" id="2099">2099</a>
<a href="#2100" id="2100">2100</a>
<a href="#2101" id="2101">2101</a>
<a href="#2102" id="2102">2102</a>
<a href="#2103" id="2103">2103</a>
<a href="#2104" id="2104">2104</a>
<a href="#2105" id="2105">2105</a>
<a href="#2106" id="2106">2106</a>
<a href="#2107" id="2107">2107</a>
<a href="#2108" id="2108">2108</a>
<a href="#2109" id="2109">2109</a>
<a href="#2110" id="2110">2110</a>
<a href="#2111" id="2111">2111</a>
<a href="#2112" id="2112">2112</a>
<a href="#2113" id="2113">2113</a>
<a href="#2114" id="2114">2114</a>
<a href="#2115" id="2115">2115</a>
<a href="#2116" id="2116">2116</a>
<a href="#2117" id="2117">2117</a>
<a href="#2118" id="2118">2118</a>
<a href="#2119" id="2119">2119</a>
<a href="#2120" id="2120">2120</a>
<a href="#2121" id="2121">2121</a>
<a href="#2122" id="2122">2122</a>
<a href="#2123" id="2123">2123</a>
<a href="#2124" id="2124">2124</a>
<a href="#2125" id="2125">2125</a>
<a href="#2126" id="2126">2126</a>
<a href="#2127" id="2127">2127</a>
<a href="#2128" id="2128">2128</a>
<a href="#2129" id="2129">2129</a>
<a href="#2130" id="2130">2130</a>
<a href="#2131" id="2131">2131</a>
<a href="#2132" id="2132">2132</a>
<a href="#2133" id="2133">2133</a>
<a href="#2134" id="2134">2134</a>
<a href="#2135" id="2135">2135</a>
<a href="#2136" id="2136">2136</a>
<a href="#2137" id="2137">2137</a>
<a href="#2138" id="2138">2138</a>
<a href="#2139" id="2139">2139</a>
<a href="#2140" id="2140">2140</a>
<a href="#2141" id="2141">2141</a>
<a href="#2142" id="2142">2142</a>
<a href="#2143" id="2143">2143</a>
<a href="#2144" id="2144">2144</a>
<a href="#2145" id="2145">2145</a>
<a href="#2146" id="2146">2146</a>
<a href="#2147" id="2147">2147</a>
<a href="#2148" id="2148">2148</a>
<a href="#2149" id="2149">2149</a>
<a href="#2150" id="2150">2150</a>
<a href="#2151" id="2151">2151</a>
<a href="#2152" id="2152">2152</a>
<a href="#2153" id="2153">2153</a>
<a href="#2154" id="2154">2154</a>
<a href="#2155" id="2155">2155</a>
<a href="#2156" id="2156">2156</a>
<a href="#2157" id="2157">2157</a>
<a href="#2158" id="2158">2158</a>
<a href="#2159" id="2159">2159</a>
<a href="#2160" id="2160">2160</a>
<a href="#2161" id="2161">2161</a>
<a href="#2162" id="2162">2162</a>
<a href="#2163" id="2163">2163</a>
<a href="#2164" id="2164">2164</a>
<a href="#2165" id="2165">2165</a>
<a href="#2166" id="2166">2166</a>
<a href="#2167" id="2167">2167</a>
<a href="#2168" id="2168">2168</a>
<a href="#2169" id="2169">2169</a>
<a href="#2170" id="2170">2170</a>
<a href="#2171" id="2171">2171</a>
<a href="#2172" id="2172">2172</a>
<a href="#2173" id="2173">2173</a>
<a href="#2174" id="2174">2174</a>
<a href="#2175" id="2175">2175</a>
<a href="#2176" id="2176">2176</a>
<a href="#2177" id="2177">2177</a>
<a href="#2178" id="2178">2178</a>
<a href="#2179" id="2179">2179</a>
<a href="#2180" id="2180">2180</a>
<a href="#2181" id="2181">2181</a>
<a href="#2182" id="2182">2182</a>
<a href="#2183" id="2183">2183</a>
<a href="#2184" id="2184">2184</a>
<a href="#2185" id="2185">2185</a>
<a href="#2186" id="2186">2186</a>
<a href="#2187" id="2187">2187</a>
<a href="#2188" id="2188">2188</a>
<a href="#2189" id="2189">2189</a>
<a href="#2190" id="2190">2190</a>
<a href="#2191" id="2191">2191</a>
<a href="#2192" id="2192">2192</a>
<a href="#2193" id="2193">2193</a>
<a href="#2194" id="2194">2194</a>
<a href="#2195" id="2195">2195</a>
<a href="#2196" id="2196">2196</a>
<a href="#2197" id="2197">2197</a>
<a href="#2198" id="2198">2198</a>
<a href="#2199" id="2199">2199</a>
<a href="#2200" id="2200">2200</a>
<a href="#2201" id="2201">2201</a>
<a href="#2202" id="2202">2202</a>
<a href="#2203" id="2203">2203</a>
<a href="#2204" id="2204">2204</a>
<a href="#2205" id="2205">2205</a>
<a href="#2206" id="2206">2206</a>
<a href="#2207" id="2207">2207</a>
<a href="#2208" id="2208">2208</a>
<a href="#2209" id="2209">2209</a>
<a href="#2210" id="2210">2210</a>
<a href="#2211" id="2211">2211</a>
<a href="#2212" id="2212">2212</a>
<a href="#2213" id="2213">2213</a>
<a href="#2214" id="2214">2214</a>
<a href="#2215" id="2215">2215</a>
<a href="#2216" id="2216">2216</a>
<a href="#2217" id="2217">2217</a>
<a href="#2218" id="2218">2218</a>
<a href="#2219" id="2219">2219</a>
<a href="#2220" id="2220">2220</a>
<a href="#2221" id="2221">2221</a>
<a href="#2222" id="2222">2222</a>
<a href="#2223" id="2223">2223</a>
<a href="#2224" id="2224">2224</a>
<a href="#2225" id="2225">2225</a>
<a href="#2226" id="2226">2226</a>
<a href="#2227" id="2227">2227</a>
<a href="#2228" id="2228">2228</a>
<a href="#2229" id="2229">2229</a>
<a href="#2230" id="2230">2230</a>
<a href="#2231" id="2231">2231</a>
<a href="#2232" id="2232">2232</a>
<a href="#2233" id="2233">2233</a>
<a href="#2234" id="2234">2234</a>
<a href="#2235" id="2235">2235</a>
<a href="#2236" id="2236">2236</a>
<a href="#2237" id="2237">2237</a>
<a href="#2238" id="2238">2238</a>
<a href="#2239" id="2239">2239</a>
<a href="#2240" id="2240">2240</a>
<a href="#2241" id="2241">2241</a>
<a href="#2242" id="2242">2242</a>
<a href="#2243" id="2243">2243</a>
<a href="#2244" id="2244">2244</a>
<a href="#2245" id="2245">2245</a>
<a href="#2246" id="2246">2246</a>
<a href="#2247" id="2247">2247</a>
<a href="#2248" id="2248">2248</a>
<a href="#2249" id="2249">2249</a>
<a href="#2250" id="2250">2250</a>
<a href="#2251" id="2251">2251</a>
<a href="#2252" id="2252">2252</a>
<a href="#2253" id="2253">2253</a>
<a href="#2254" id="2254">2254</a>
<a href="#2255" id="2255">2255</a>
<a href="#2256" id="2256">2256</a>
<a href="#2257" id="2257">2257</a>
<a href="#2258" id="2258">2258</a>
<a href="#2259" id="2259">2259</a>
<a href="#2260" id="2260">2260</a>
<a href="#2261" id="2261">2261</a>
<a href="#2262" id="2262">2262</a>
<a href="#2263" id="2263">2263</a>
<a href="#2264" id="2264">2264</a>
<a href="#2265" id="2265">2265</a>
<a href="#2266" id="2266">2266</a>
<a href="#2267" id="2267">2267</a>
<a href="#2268" id="2268">2268</a>
<a href="#2269" id="2269">2269</a>
<a href="#2270" id="2270">2270</a>
<a href="#2271" id="2271">2271</a>
<a href="#2272" id="2272">2272</a>
<a href="#2273" id="2273">2273</a>
<a href="#2274" id="2274">2274</a>
<a href="#2275" id="2275">2275</a>
<a href="#2276" id="2276">2276</a>
<a href="#2277" id="2277">2277</a>
<a href="#2278" id="2278">2278</a>
<a href="#2279" id="2279">2279</a>
<a href="#2280" id="2280">2280</a>
<a href="#2281" id="2281">2281</a>
<a href="#2282" id="2282">2282</a>
<a href="#2283" id="2283">2283</a>
<a href="#2284" id="2284">2284</a>
<a href="#2285" id="2285">2285</a>
<a href="#2286" id="2286">2286</a>
<a href="#2287" id="2287">2287</a>
<a href="#2288" id="2288">2288</a>
<a href="#2289" id="2289">2289</a>
<a href="#2290" id="2290">2290</a>
<a href="#2291" id="2291">2291</a>
<a href="#2292" id="2292">2292</a>
<a href="#2293" id="2293">2293</a>
<a href="#2294" id="2294">2294</a>
<a href="#2295" id="2295">2295</a>
<a href="#2296" id="2296">2296</a>
<a href="#2297" id="2297">2297</a>
<a href="#2298" id="2298">2298</a>
<a href="#2299" id="2299">2299</a>
<a href="#2300" id="2300">2300</a>
<a href="#2301" id="2301">2301</a>
<a href="#2302" id="2302">2302</a>
<a href="#2303" id="2303">2303</a>
<a href="#2304" id="2304">2304</a>
<a href="#2305" id="2305">2305</a>
<a href="#2306" id="2306">2306</a>
<a href="#2307" id="2307">2307</a>
<a href="#2308" id="2308">2308</a>
<a href="#2309" id="2309">2309</a>
<a href="#2310" id="2310">2310</a>
<a href="#2311" id="2311">2311</a>
<a href="#2312" id="2312">2312</a>
<a href="#2313" id="2313">2313</a>
<a href="#2314" id="2314">2314</a>
<a href="#2315" id="2315">2315</a>
<a href="#2316" id="2316">2316</a>
<a href="#2317" id="2317">2317</a>
<a href="#2318" id="2318">2318</a>
<a href="#2319" id="2319">2319</a>
<a href="#2320" id="2320">2320</a>
<a href="#2321" id="2321">2321</a>
<a href="#2322" id="2322">2322</a>
<a href="#2323" id="2323">2323</a>
<a href="#2324" id="2324">2324</a>
<a href="#2325" id="2325">2325</a>
<a href="#2326" id="2326">2326</a>
<a href="#2327" id="2327">2327</a>
<a href="#2328" id="2328">2328</a>
<a href="#2329" id="2329">2329</a>
<a href="#2330" id="2330">2330</a>
<a href="#2331" id="2331">2331</a>
<a href="#2332" id="2332">2332</a>
<a href="#2333" id="2333">2333</a>
<a href="#2334" id="2334">2334</a>
<a href="#2335" id="2335">2335</a>
<a href="#2336" id="2336">2336</a>
<a href="#2337" id="2337">2337</a>
<a href="#2338" id="2338">2338</a>
<a href="#2339" id="2339">2339</a>
<a href="#2340" id="2340">2340</a>
<a href="#2341" id="2341">2341</a>
<a href="#2342" id="2342">2342</a>
<a href="#2343" id="2343">2343</a>
<a href="#2344" id="2344">2344</a>
<a href="#2345" id="2345">2345</a>
<a href="#2346" id="2346">2346</a>
<a href="#2347" id="2347">2347</a>
<a href="#2348" id="2348">2348</a>
<a href="#2349" id="2349">2349</a>
<a href="#2350" id="2350">2350</a>
<a href="#2351" id="2351">2351</a>
<a href="#2352" id="2352">2352</a>
<a href="#2353" id="2353">2353</a>
<a href="#2354" id="2354">2354</a>
<a href="#2355" id="2355">2355</a>
<a href="#2356" id="2356">2356</a>
<a href="#2357" id="2357">2357</a>
<a href="#2358" id="2358">2358</a>
<a href="#2359" id="2359">2359</a>
<a href="#2360" id="2360">2360</a>
<a href="#2361" id="2361">2361</a>
<a href="#2362" id="2362">2362</a>
<a href="#2363" id="2363">2363</a>
<a href="#2364" id="2364">2364</a>
<a href="#2365" id="2365">2365</a>
<a href="#2366" id="2366">2366</a>
<a href="#2367" id="2367">2367</a>
<a href="#2368" id="2368">2368</a>
<a href="#2369" id="2369">2369</a>
<a href="#2370" id="2370">2370</a>
<a href="#2371" id="2371">2371</a>
<a href="#2372" id="2372">2372</a>
<a href="#2373" id="2373">2373</a>
<a href="#2374" id="2374">2374</a>
<a href="#2375" id="2375">2375</a>
<a href="#2376" id="2376">2376</a>
<a href="#2377" id="2377">2377</a>
<a href="#2378" id="2378">2378</a>
<a href="#2379" id="2379">2379</a>
<a href="#2380" id="2380">2380</a>
<a href="#2381" id="2381">2381</a>
<a href="#2382" id="2382">2382</a>
<a href="#2383" id="2383">2383</a>
<a href="#2384" id="2384">2384</a>
<a href="#2385" id="2385">2385</a>
<a href="#2386" id="2386">2386</a>
<a href="#2387" id="2387">2387</a>
<a href="#2388" id="2388">2388</a>
<a href="#2389" id="2389">2389</a>
<a href="#2390" id="2390">2390</a>
<a href="#2391" id="2391">2391</a>
<a href="#2392" id="2392">2392</a>
<a href="#2393" id="2393">2393</a>
<a href="#2394" id="2394">2394</a>
<a href="#2395" id="2395">2395</a>
<a href="#2396" id="2396">2396</a>
<a href="#2397" id="2397">2397</a>
<a href="#2398" id="2398">2398</a>
<a href="#2399" id="2399">2399</a>
<a href="#2400" id="2400">2400</a>
<a href="#2401" id="2401">2401</a>
<a href="#2402" id="2402">2402</a>
<a href="#2403" id="2403">2403</a>
<a href="#2404" id="2404">2404</a>
<a href="#2405" id="2405">2405</a>
<a href="#2406" id="2406">2406</a>
<a href="#2407" id="2407">2407</a>
<a href="#2408" id="2408">2408</a>
<a href="#2409" id="2409">2409</a>
<a href="#2410" id="2410">2410</a>
<a href="#2411" id="2411">2411</a>
<a href="#2412" id="2412">2412</a>
<a href="#2413" id="2413">2413</a>
<a href="#2414" id="2414">2414</a>
<a href="#2415" id="2415">2415</a>
<a href="#2416" id="2416">2416</a>
<a href="#2417" id="2417">2417</a>
<a href="#2418" id="2418">2418</a>
<a href="#2419" id="2419">2419</a>
<a href="#2420" id="2420">2420</a>
<a href="#2421" id="2421">2421</a>
<a href="#2422" id="2422">2422</a>
<a href="#2423" id="2423">2423</a>
<a href="#2424" id="2424">2424</a>
<a href="#2425" id="2425">2425</a>
<a href="#2426" id="2426">2426</a>
<a href="#2427" id="2427">2427</a>
<a href="#2428" id="2428">2428</a>
<a href="#2429" id="2429">2429</a>
<a href="#2430" id="2430">2430</a>
<a href="#2431" id="2431">2431</a>
<a href="#2432" id="2432">2432</a>
<a href="#2433" id="2433">2433</a>
<a href="#2434" id="2434">2434</a>
<a href="#2435" id="2435">2435</a>
<a href="#2436" id="2436">2436</a>
<a href="#2437" id="2437">2437</a>
<a href="#2438" id="2438">2438</a>
<a href="#2439" id="2439">2439</a>
<a href="#2440" id="2440">2440</a>
<a href="#2441" id="2441">2441</a>
<a href="#2442" id="2442">2442</a>
<a href="#2443" id="2443">2443</a>
<a href="#2444" id="2444">2444</a>
<a href="#2445" id="2445">2445</a>
<a href="#2446" id="2446">2446</a>
<a href="#2447" id="2447">2447</a>
<a href="#2448" id="2448">2448</a>
<a href="#2449" id="2449">2449</a>
<a href="#2450" id="2450">2450</a>
<a href="#2451" id="2451">2451</a>
<a href="#2452" id="2452">2452</a>
<a href="#2453" id="2453">2453</a>
<a href="#2454" id="2454">2454</a>
<a href="#2455" id="2455">2455</a>
<a href="#2456" id="2456">2456</a>
<a href="#2457" id="2457">2457</a>
<a href="#2458" id="2458">2458</a>
<a href="#2459" id="2459">2459</a>
<a href="#2460" id="2460">2460</a>
<a href="#2461" id="2461">2461</a>
<a href="#2462" id="2462">2462</a>
<a href="#2463" id="2463">2463</a>
<a href="#2464" id="2464">2464</a>
<a href="#2465" id="2465">2465</a>
<a href="#2466" id="2466">2466</a>
<a href="#2467" id="2467">2467</a>
<a href="#2468" id="2468">2468</a>
<a href="#2469" id="2469">2469</a>
<a href="#2470" id="2470">2470</a>
<a href="#2471" id="2471">2471</a>
<a href="#2472" id="2472">2472</a>
<a href="#2473" id="2473">2473</a>
<a href="#2474" id="2474">2474</a>
<a href="#2475" id="2475">2475</a>
<a href="#2476" id="2476">2476</a>
<a href="#2477" id="2477">2477</a>
<a href="#2478" id="2478">2478</a>
<a href="#2479" id="2479">2479</a>
<a href="#2480" id="2480">2480</a>
<a href="#2481" id="2481">2481</a>
<a href="#2482" id="2482">2482</a>
<a href="#2483" id="2483">2483</a>
<a href="#2484" id="2484">2484</a>
<a href="#2485" id="2485">2485</a>
<a href="#2486" id="2486">2486</a>
<a href="#2487" id="2487">2487</a>
<a href="#2488" id="2488">2488</a>
<a href="#2489" id="2489">2489</a>
<a href="#2490" id="2490">2490</a>
<a href="#2491" id="2491">2491</a>
<a href="#2492" id="2492">2492</a>
<a href="#2493" id="2493">2493</a>
<a href="#2494" id="2494">2494</a>
<a href="#2495" id="2495">2495</a>
<a href="#2496" id="2496">2496</a>
<a href="#2497" id="2497">2497</a>
<a href="#2498" id="2498">2498</a>
<a href="#2499" id="2499">2499</a>
<a href="#2500" id="2500">2500</a>
<a href="#2501" id="2501">2501</a>
<a href="#2502" id="2502">2502</a>
<a href="#2503" id="2503">2503</a>
<a href="#2504" id="2504">2504</a>
<a href="#2505" id="2505">2505</a>
<a href="#2506" id="2506">2506</a>
<a href="#2507" id="2507">2507</a>
<a href="#2508" id="2508">2508</a>
<a href="#2509" id="2509">2509</a>
<a href="#2510" id="2510">2510</a>
<a href="#2511" id="2511">2511</a>
<a href="#2512" id="2512">2512</a>
<a href="#2513" id="2513">2513</a>
<a href="#2514" id="2514">2514</a>
<a href="#2515" id="2515">2515</a>
<a href="#2516" id="2516">2516</a>
<a href="#2517" id="2517">2517</a>
<a href="#2518" id="2518">2518</a>
<a href="#2519" id="2519">2519</a>
<a href="#2520" id="2520">2520</a>
<a href="#2521" id="2521">2521</a>
<a href="#2522" id="2522">2522</a>
<a href="#2523" id="2523">2523</a>
<a href="#2524" id="2524">2524</a>
<a href="#2525" id="2525">2525</a>
<a href="#2526" id="2526">2526</a>
<a href="#2527" id="2527">2527</a>
<a href="#2528" id="2528">2528</a>
<a href="#2529" id="2529">2529</a>
<a href="#2530" id="2530">2530</a>
<a href="#2531" id="2531">2531</a>
<a href="#2532" id="2532">2532</a>
<a href="#2533" id="2533">2533</a>
<a href="#2534" id="2534">2534</a>
<a href="#2535" id="2535">2535</a>
<a href="#2536" id="2536">2536</a>
<a href="#2537" id="2537">2537</a>
<a href="#2538" id="2538">2538</a>
<a href="#2539" id="2539">2539</a>
<a href="#2540" id="2540">2540</a>
<a href="#2541" id="2541">2541</a>
<a href="#2542" id="2542">2542</a>
<a href="#2543" id="2543">2543</a>
<a href="#2544" id="2544">2544</a>
<a href="#2545" id="2545">2545</a>
<a href="#2546" id="2546">2546</a>
<a href="#2547" id="2547">2547</a>
<a href="#2548" id="2548">2548</a>
<a href="#2549" id="2549">2549</a>
<a href="#2550" id="2550">2550</a>
<a href="#2551" id="2551">2551</a>
<a href="#2552" id="2552">2552</a>
<a href="#2553" id="2553">2553</a>
<a href="#2554" id="2554">2554</a>
<a href="#2555" id="2555">2555</a>
<a href="#2556" id="2556">2556</a>
<a href="#2557" id="2557">2557</a>
<a href="#2558" id="2558">2558</a>
<a href="#2559" id="2559">2559</a>
<a href="#2560" id="2560">2560</a>
<a href="#2561" id="2561">2561</a>
<a href="#2562" id="2562">2562</a>
<a href="#2563" id="2563">2563</a>
<a href="#2564" id="2564">2564</a>
<a href="#2565" id="2565">2565</a>
<a href="#2566" id="2566">2566</a>
<a href="#2567" id="2567">2567</a>
<a href="#2568" id="2568">2568</a>
<a href="#2569" id="2569">2569</a>
<a href="#2570" id="2570">2570</a>
<a href="#2571" id="2571">2571</a>
<a href="#2572" id="2572">2572</a>
<a href="#2573" id="2573">2573</a>
<a href="#2574" id="2574">2574</a>
<a href="#2575" id="2575">2575</a>
<a href="#2576" id="2576">2576</a>
<a href="#2577" id="2577">2577</a>
<a href="#2578" id="2578">2578</a>
<a href="#2579" id="2579">2579</a>
<a href="#2580" id="2580">2580</a>
<a href="#2581" id="2581">2581</a>
<a href="#2582" id="2582">2582</a>
<a href="#2583" id="2583">2583</a>
<a href="#2584" id="2584">2584</a>
<a href="#2585" id="2585">2585</a>
<a href="#2586" id="2586">2586</a>
<a href="#2587" id="2587">2587</a>
<a href="#2588" id="2588">2588</a>
<a href="#2589" id="2589">2589</a>
<a href="#2590" id="2590">2590</a>
<a href="#2591" id="2591">2591</a>
<a href="#2592" id="2592">2592</a>
<a href="#2593" id="2593">2593</a>
<a href="#2594" id="2594">2594</a>
<a href="#2595" id="2595">2595</a>
<a href="#2596" id="2596">2596</a>
<a href="#2597" id="2597">2597</a>
<a href="#2598" id="2598">2598</a>
<a href="#2599" id="2599">2599</a>
<a href="#2600" id="2600">2600</a>
<a href="#2601" id="2601">2601</a>
<a href="#2602" id="2602">2602</a>
<a href="#2603" id="2603">2603</a>
<a href="#2604" id="2604">2604</a>
<a href="#2605" id="2605">2605</a>
<a href="#2606" id="2606">2606</a>
<a href="#2607" id="2607">2607</a>
<a href="#2608" id="2608">2608</a>
<a href="#2609" id="2609">2609</a>
<a href="#2610" id="2610">2610</a>
<a href="#2611" id="2611">2611</a>
<a href="#2612" id="2612">2612</a>
<a href="#2613" id="2613">2613</a>
<a href="#2614" id="2614">2614</a>
<a href="#2615" id="2615">2615</a>
<a href="#2616" id="2616">2616</a>
<a href="#2617" id="2617">2617</a>
<a href="#2618" id="2618">2618</a>
<a href="#2619" id="2619">2619</a>
<a href="#2620" id="2620">2620</a>
<a href="#2621" id="2621">2621</a>
<a href="#2622" id="2622">2622</a>
<a href="#2623" id="2623">2623</a>
<a href="#2624" id="2624">2624</a>
<a href="#2625" id="2625">2625</a>
<a href="#2626" id="2626">2626</a>
<a href="#2627" id="2627">2627</a>
<a href="#2628" id="2628">2628</a>
<a href="#2629" id="2629">2629</a>
<a href="#2630" id="2630">2630</a>
<a href="#2631" id="2631">2631</a>
<a href="#2632" id="2632">2632</a>
<a href="#2633" id="2633">2633</a>
<a href="#2634" id="2634">2634</a>
<a href="#2635" id="2635">2635</a>
<a href="#2636" id="2636">2636</a>
<a href="#2637" id="2637">2637</a>
<a href="#2638" id="2638">2638</a>
<a href="#2639" id="2639">2639</a>
<a href="#2640" id="2640">2640</a>
<a href="#2641" id="2641">2641</a>
<a href="#2642" id="2642">2642</a>
<a href="#2643" id="2643">2643</a>
<a href="#2644" id="2644">2644</a>
<a href="#2645" id="2645">2645</a>
<a href="#2646" id="2646">2646</a>
<a href="#2647" id="2647">2647</a>
<a href="#2648" id="2648">2648</a>
<a href="#2649" id="2649">2649</a>
<a href="#2650" id="2650">2650</a>
<a href="#2651" id="2651">2651</a>
<a href="#2652" id="2652">2652</a>
<a href="#2653" id="2653">2653</a>
<a href="#2654" id="2654">2654</a>
<a href="#2655" id="2655">2655</a>
<a href="#2656" id="2656">2656</a>
<a href="#2657" id="2657">2657</a>
<a href="#2658" id="2658">2658</a>
<a href="#2659" id="2659">2659</a>
<a href="#2660" id="2660">2660</a>
<a href="#2661" id="2661">2661</a>
<a href="#2662" id="2662">2662</a>
<a href="#2663" id="2663">2663</a>
<a href="#2664" id="2664">2664</a>
<a href="#2665" id="2665">2665</a>
<a href="#2666" id="2666">2666</a>
<a href="#2667" id="2667">2667</a>
<a href="#2668" id="2668">2668</a>
<a href="#2669" id="2669">2669</a>
<a href="#2670" id="2670">2670</a>
<a href="#2671" id="2671">2671</a>
<a href="#2672" id="2672">2672</a>
<a href="#2673" id="2673">2673</a>
<a href="#2674" id="2674">2674</a>
<a href="#2675" id="2675">2675</a>
<a href="#2676" id="2676">2676</a>
<a href="#2677" id="2677">2677</a>
<a href="#2678" id="2678">2678</a>
<a href="#2679" id="2679">2679</a>
<a href="#2680" id="2680">2680</a>
<a href="#2681" id="2681">2681</a>
<a href="#2682" id="2682">2682</a>
<a href="#2683" id="2683">2683</a>
<a href="#2684" id="2684">2684</a>
<a href="#2685" id="2685">2685</a>
<a href="#2686" id="2686">2686</a>
<a href="#2687" id="2687">2687</a>
<a href="#2688" id="2688">2688</a>
<a href="#2689" id="2689">2689</a>
<a href="#2690" id="2690">2690</a>
<a href="#2691" id="2691">2691</a>
<a href="#2692" id="2692">2692</a>
<a href="#2693" id="2693">2693</a>
<a href="#2694" id="2694">2694</a>
<a href="#2695" id="2695">2695</a>
<a href="#2696" id="2696">2696</a>
<a href="#2697" id="2697">2697</a>
<a href="#2698" id="2698">2698</a>
<a href="#2699" id="2699">2699</a>
<a href="#2700" id="2700">2700</a>
<a href="#2701" id="2701">2701</a>
<a href="#2702" id="2702">2702</a>
<a href="#2703" id="2703">2703</a>
<a href="#2704" id="2704">2704</a>
<a href="#2705" id="2705">2705</a>
<a href="#2706" id="2706">2706</a>
<a href="#2707" id="2707">2707</a>
<a href="#2708" id="2708">2708</a>
<a href="#2709" id="2709">2709</a>
<a href="#2710" id="2710">2710</a>
<a href="#2711" id="2711">2711</a>
<a href="#2712" id="2712">2712</a>
<a href="#2713" id="2713">2713</a>
<a href="#2714" id="2714">2714</a>
<a href="#2715" id="2715">2715</a>
<a href="#2716" id="2716">2716</a>
<a href="#2717" id="2717">2717</a>
<a href="#2718" id="2718">2718</a>
<a href="#2719" id="2719">2719</a>
<a href="#2720" id="2720">2720</a>
<a href="#2721" id="2721">2721</a>
<a href="#2722" id="2722">2722</a>
<a href="#2723" id="2723">2723</a>
<a href="#2724" id="2724">2724</a>
<a href="#2725" id="2725">2725</a>
<a href="#2726" id="2726">2726</a>
<a href="#2727" id="2727">2727</a>
<a href="#2728" id="2728">2728</a>
<a href="#2729" id="2729">2729</a>
<a href="#2730" id="2730">2730</a>
<a href="#2731" id="2731">2731</a>
<a href="#2732" id="2732">2732</a>
<a href="#2733" id="2733">2733</a>
<a href="#2734" id="2734">2734</a>
<a href="#2735" id="2735">2735</a>
<a href="#2736" id="2736">2736</a>
<a href="#2737" id="2737">2737</a>
<a href="#2738" id="2738">2738</a>
<a href="#2739" id="2739">2739</a>
<a href="#2740" id="2740">2740</a>
<a href="#2741" id="2741">2741</a>
<a href="#2742" id="2742">2742</a>
<a href="#2743" id="2743">2743</a>
<a href="#2744" id="2744">2744</a>
<a href="#2745" id="2745">2745</a>
<a href="#2746" id="2746">2746</a>
<a href="#2747" id="2747">2747</a>
<a href="#2748" id="2748">2748</a>
<a href="#2749" id="2749">2749</a>
<a href="#2750" id="2750">2750</a>
<a href="#2751" id="2751">2751</a>
<a href="#2752" id="2752">2752</a>
<a href="#2753" id="2753">2753</a>
<a href="#2754" id="2754">2754</a>
<a href="#2755" id="2755">2755</a>
<a href="#2756" id="2756">2756</a>
<a href="#2757" id="2757">2757</a>
<a href="#2758" id="2758">2758</a>
<a href="#2759" id="2759">2759</a>
<a href="#2760" id="2760">2760</a>
<a href="#2761" id="2761">2761</a>
<a href="#2762" id="2762">2762</a>
<a href="#2763" id="2763">2763</a>
<a href="#2764" id="2764">2764</a>
<a href="#2765" id="2765">2765</a>
<a href="#2766" id="2766">2766</a>
<a href="#2767" id="2767">2767</a>
<a href="#2768" id="2768">2768</a>
<a href="#2769" id="2769">2769</a>
<a href="#2770" id="2770">2770</a>
<a href="#2771" id="2771">2771</a>
<a href="#2772" id="2772">2772</a>
<a href="#2773" id="2773">2773</a>
<a href="#2774" id="2774">2774</a>
<a href="#2775" id="2775">2775</a>
<a href="#2776" id="2776">2776</a>
<a href="#2777" id="2777">2777</a>
<a href="#2778" id="2778">2778</a>
<a href="#2779" id="2779">2779</a>
<a href="#2780" id="2780">2780</a>
<a href="#2781" id="2781">2781</a>
<a href="#2782" id="2782">2782</a>
<a href="#2783" id="2783">2783</a>
<a href="#2784" id="2784">2784</a>
<a href="#2785" id="2785">2785</a>
<a href="#2786" id="2786">2786</a>
<a href="#2787" id="2787">2787</a>
<a href="#2788" id="2788">2788</a>
<a href="#2789" id="2789">2789</a>
<a href="#2790" id="2790">2790</a>
<a href="#2791" id="2791">2791</a>
<a href="#2792" id="2792">2792</a>
<a href="#2793" id="2793">2793</a>
<a href="#2794" id="2794">2794</a>
<a href="#2795" id="2795">2795</a>
<a href="#2796" id="2796">2796</a>
<a href="#2797" id="2797">2797</a>
<a href="#2798" id="2798">2798</a>
<a href="#2799" id="2799">2799</a>
<a href="#2800" id="2800">2800</a>
<a href="#2801" id="2801">2801</a>
<a href="#2802" id="2802">2802</a>
<a href="#2803" id="2803">2803</a>
<a href="#2804" id="2804">2804</a>
<a href="#2805" id="2805">2805</a>
<a href="#2806" id="2806">2806</a>
<a href="#2807" id="2807">2807</a>
<a href="#2808" id="2808">2808</a>
<a href="#2809" id="2809">2809</a>
<a href="#2810" id="2810">2810</a>
<a href="#2811" id="2811">2811</a>
<a href="#2812" id="2812">2812</a>
<a href="#2813" id="2813">2813</a>
<a href="#2814" id="2814">2814</a>
<a href="#2815" id="2815">2815</a>
<a href="#2816" id="2816">2816</a>
<a href="#2817" id="2817">2817</a>
<a href="#2818" id="2818">2818</a>
<a href="#2819" id="2819">2819</a>
<a href="#2820" id="2820">2820</a>
<a href="#2821" id="2821">2821</a>
<a href="#2822" id="2822">2822</a>
<a href="#2823" id="2823">2823</a>
<a href="#2824" id="2824">2824</a>
<a href="#2825" id="2825">2825</a>
<a href="#2826" id="2826">2826</a>
<a href="#2827" id="2827">2827</a>
<a href="#2828" id="2828">2828</a>
<a href="#2829" id="2829">2829</a>
<a href="#2830" id="2830">2830</a>
<a href="#2831" id="2831">2831</a>
<a href="#2832" id="2832">2832</a>
<a href="#2833" id="2833">2833</a>
<a href="#2834" id="2834">2834</a>
<a href="#2835" id="2835">2835</a>
<a href="#2836" id="2836">2836</a>
<a href="#2837" id="2837">2837</a>
<a href="#2838" id="2838">2838</a>
<a href="#2839" id="2839">2839</a>
<a href="#2840" id="2840">2840</a>
<a href="#2841" id="2841">2841</a>
<a href="#2842" id="2842">2842</a>
<a href="#2843" id="2843">2843</a>
<a href="#2844" id="2844">2844</a>
<a href="#2845" id="2845">2845</a>
<a href="#2846" id="2846">2846</a>
<a href="#2847" id="2847">2847</a>
<a href="#2848" id="2848">2848</a>
<a href="#2849" id="2849">2849</a>
<a href="#2850" id="2850">2850</a>
<a href="#2851" id="2851">2851</a>
<a href="#2852" id="2852">2852</a>
<a href="#2853" id="2853">2853</a>
<a href="#2854" id="2854">2854</a>
<a href="#2855" id="2855">2855</a>
<a href="#2856" id="2856">2856</a>
<a href="#2857" id="2857">2857</a>
<a href="#2858" id="2858">2858</a>
<a href="#2859" id="2859">2859</a>
<a href="#2860" id="2860">2860</a>
<a href="#2861" id="2861">2861</a>
<a href="#2862" id="2862">2862</a>
<a href="#2863" id="2863">2863</a>
<a href="#2864" id="2864">2864</a>
<a href="#2865" id="2865">2865</a>
<a href="#2866" id="2866">2866</a>
<a href="#2867" id="2867">2867</a>
<a href="#2868" id="2868">2868</a>
<a href="#2869" id="2869">2869</a>
<a href="#2870" id="2870">2870</a>
<a href="#2871" id="2871">2871</a>
<a href="#2872" id="2872">2872</a>
<a href="#2873" id="2873">2873</a>
<a href="#2874" id="2874">2874</a>
<a href="#2875" id="2875">2875</a>
<a href="#2876" id="2876">2876</a>
<a href="#2877" id="2877">2877</a>
<a href="#2878" id="2878">2878</a>
<a href="#2879" id="2879">2879</a>
<a href="#2880" id="2880">2880</a>
<a href="#2881" id="2881">2881</a>
<a href="#2882" id="2882">2882</a>
<a href="#2883" id="2883">2883</a>
<a href="#2884" id="2884">2884</a>
<a href="#2885" id="2885">2885</a>
<a href="#2886" id="2886">2886</a>
<a href="#2887" id="2887">2887</a>
<a href="#2888" id="2888">2888</a>
<a href="#2889" id="2889">2889</a>
<a href="#2890" id="2890">2890</a>
<a href="#2891" id="2891">2891</a>
<a href="#2892" id="2892">2892</a>
<a href="#2893" id="2893">2893</a>
<a href="#2894" id="2894">2894</a>
<a href="#2895" id="2895">2895</a>
<a href="#2896" id="2896">2896</a>
<a href="#2897" id="2897">2897</a>
<a href="#2898" id="2898">2898</a>
<a href="#2899" id="2899">2899</a>
<a href="#2900" id="2900">2900</a>
<a href="#2901" id="2901">2901</a>
<a href="#2902" id="2902">2902</a>
<a href="#2903" id="2903">2903</a>
<a href="#2904" id="2904">2904</a>
<a href="#2905" id="2905">2905</a>
<a href="#2906" id="2906">2906</a>
<a href="#2907" id="2907">2907</a>
<a href="#2908" id="2908">2908</a>
<a href="#2909" id="2909">2909</a>
<a href="#2910" id="2910">2910</a>
<a href="#2911" id="2911">2911</a>
<a href="#2912" id="2912">2912</a>
<a href="#2913" id="2913">2913</a>
<a href="#2914" id="2914">2914</a>
<a href="#2915" id="2915">2915</a>
<a href="#2916" id="2916">2916</a>
<a href="#2917" id="2917">2917</a>
<a href="#2918" id="2918">2918</a>
<a href="#2919" id="2919">2919</a>
<a href="#2920" id="2920">2920</a>
<a href="#2921" id="2921">2921</a>
<a href="#2922" id="2922">2922</a>
<a href="#2923" id="2923">2923</a>
<a href="#2924" id="2924">2924</a>
<a href="#2925" id="2925">2925</a>
<a href="#2926" id="2926">2926</a>
<a href="#2927" id="2927">2927</a>
<a href="#2928" id="2928">2928</a>
<a href="#2929" id="2929">2929</a>
<a href="#2930" id="2930">2930</a>
<a href="#2931" id="2931">2931</a>
<a href="#2932" id="2932">2932</a>
<a href="#2933" id="2933">2933</a>
<a href="#2934" id="2934">2934</a>
<a href="#2935" id="2935">2935</a>
<a href="#2936" id="2936">2936</a>
<a href="#2937" id="2937">2937</a>
<a href="#2938" id="2938">2938</a>
<a href="#2939" id="2939">2939</a>
<a href="#2940" id="2940">2940</a>
<a href="#2941" id="2941">2941</a>
<a href="#2942" id="2942">2942</a>
<a href="#2943" id="2943">2943</a>
<a href="#2944" id="2944">2944</a>
<a href="#2945" id="2945">2945</a>
<a href="#2946" id="2946">2946</a>
<a href="#2947" id="2947">2947</a>
<a href="#2948" id="2948">2948</a>
<a href="#2949" id="2949">2949</a>
<a href="#2950" id="2950">2950</a>
<a href="#2951" id="2951">2951</a>
<a href="#2952" id="2952">2952</a>
<a href="#2953" id="2953">2953</a>
<a href="#2954" id="2954">2954</a>
<a href="#2955" id="2955">2955</a>
<a href="#2956" id="2956">2956</a>
<a href="#2957" id="2957">2957</a>
<a href="#2958" id="2958">2958</a>
<a href="#2959" id="2959">2959</a>
<a href="#2960" id="2960">2960</a>
<a href="#2961" id="2961">2961</a>
<a href="#2962" id="2962">2962</a>
<a href="#2963" id="2963">2963</a>
<a href="#2964" id="2964">2964</a>
<a href="#2965" id="2965">2965</a>
<a href="#2966" id="2966">2966</a>
<a href="#2967" id="2967">2967</a>
<a href="#2968" id="2968">2968</a>
<a href="#2969" id="2969">2969</a>
<a href="#2970" id="2970">2970</a>
<a href="#2971" id="2971">2971</a>
<a href="#2972" id="2972">2972</a>
<a href="#2973" id="2973">2973</a>
<a href="#2974" id="2974">2974</a>
<a href="#2975" id="2975">2975</a>
<a href="#2976" id="2976">2976</a>
<a href="#2977" id="2977">2977</a>
<a href="#2978" id="2978">2978</a>
<a href="#2979" id="2979">2979</a>
<a href="#2980" id="2980">2980</a>
<a href="#2981" id="2981">2981</a>
<a href="#2982" id="2982">2982</a>
<a href="#2983" id="2983">2983</a>
<a href="#2984" id="2984">2984</a>
<a href="#2985" id="2985">2985</a>
<a href="#2986" id="2986">2986</a>
<a href="#2987" id="2987">2987</a>
<a href="#2988" id="2988">2988</a>
<a href="#2989" id="2989">2989</a>
<a href="#2990" id="2990">2990</a>
<a href="#2991" id="2991">2991</a>
<a href="#2992" id="2992">2992</a>
<a href="#2993" id="2993">2993</a>
<a href="#2994" id="2994">2994</a>
<a href="#2995" id="2995">2995</a>
<a href="#2996" id="2996">2996</a>
<a href="#2997" id="2997">2997</a>
<a href="#2998" id="2998">2998</a>
<a href="#2999" id="2999">2999</a>
<a href="#3000" id="3000">3000</a>
<a href="#3001" id="3001">3001</a>
<a href="#3002" id="3002">3002</a>
<a href="#3003" id="3003">3003</a>
<a href="#3004" id="3004">3004</a>
<a href="#3005" id="3005">3005</a>
<a href="#3006" id="3006">3006</a>
<a href="#3007" id="3007">3007</a>
<a href="#3008" id="3008">3008</a>
<a href="#3009" id="3009">3009</a>
<a href="#3010" id="3010">3010</a>
<a href="#3011" id="3011">3011</a>
<a href="#3012" id="3012">3012</a>
<a href="#3013" id="3013">3013</a>
<a href="#3014" id="3014">3014</a>
<a href="#3015" id="3015">3015</a>
<a href="#3016" id="3016">3016</a>
<a href="#3017" id="3017">3017</a>
<a href="#3018" id="3018">3018</a>
<a href="#3019" id="3019">3019</a>
<a href="#3020" id="3020">3020</a>
<a href="#3021" id="3021">3021</a>
<a href="#3022" id="3022">3022</a>
<a href="#3023" id="3023">3023</a>
<a href="#3024" id="3024">3024</a>
<a href="#3025" id="3025">3025</a>
<a href="#3026" id="3026">3026</a>
<a href="#3027" id="3027">3027</a>
<a href="#3028" id="3028">3028</a>
<a href="#3029" id="3029">3029</a>
<a href="#3030" id="3030">3030</a>
<a href="#3031" id="3031">3031</a>
<a href="#3032" id="3032">3032</a>
<a href="#3033" id="3033">3033</a>
<a href="#3034" id="3034">3034</a>
<a href="#3035" id="3035">3035</a>
<a href="#3036" id="3036">3036</a>
<a href="#3037" id="3037">3037</a>
<a href="#3038" id="3038">3038</a>
<a href="#3039" id="3039">3039</a>
<a href="#3040" id="3040">3040</a>
<a href="#3041" id="3041">3041</a>
<a href="#3042" id="3042">3042</a>
<a href="#3043" id="3043">3043</a>
<a href="#3044" id="3044">3044</a>
<a href="#3045" id="3045">3045</a>
<a href="#3046" id="3046">3046</a>
<a href="#3047" id="3047">3047</a>
<a href="#3048" id="3048">3048</a>
<a href="#3049" id="3049">3049</a>
<a href="#3050" id="3050">3050</a>
<a href="#3051" id="3051">3051</a>
<a href="#3052" id="3052">3052</a>
<a href="#3053" id="3053">3053</a>
<a href="#3054" id="3054">3054</a>
<a href="#3055" id="3055">3055</a>
<a href="#3056" id="3056">3056</a>
<a href="#3057" id="3057">3057</a>
<a href="#3058" id="3058">3058</a>
<a href="#3059" id="3059">3059</a>
<a href="#3060" id="3060">3060</a>
<a href="#3061" id="3061">3061</a>
<a href="#3062" id="3062">3062</a>
<a href="#3063" id="3063">3063</a>
<a href="#3064" id="3064">3064</a>
<a href="#3065" id="3065">3065</a>
<a href="#3066" id="3066">3066</a>
<a href="#3067" id="3067">3067</a>
<a href="#3068" id="3068">3068</a>
<a href="#3069" id="3069">3069</a>
<a href="#3070" id="3070">3070</a>
<a href="#3071" id="3071">3071</a>
<a href="#3072" id="3072">3072</a>
<a href="#3073" id="3073">3073</a>
<a href="#3074" id="3074">3074</a>
<a href="#3075" id="3075">3075</a>
<a href="#3076" id="3076">3076</a>
<a href="#3077" id="3077">3077</a>
<a href="#3078" id="3078">3078</a>
<a href="#3079" id="3079">3079</a>
<a href="#3080" id="3080">3080</a>
<a href="#3081" id="3081">3081</a>
<a href="#3082" id="3082">3082</a>
<a href="#3083" id="3083">3083</a>
<a href="#3084" id="3084">3084</a>
<a href="#3085" id="3085">3085</a>
<a href="#3086" id="3086">3086</a>
<a href="#3087" id="3087">3087</a>
<a href="#3088" id="3088">3088</a>
<a href="#3089" id="3089">3089</a>
<a href="#3090" id="3090">3090</a>
<a href="#3091" id="3091">3091</a>
<a href="#3092" id="3092">3092</a>
<a href="#3093" id="3093">3093</a>
<a href="#3094" id="3094">3094</a>
<a href="#3095" id="3095">3095</a>
<a href="#3096" id="3096">3096</a>
<a href="#3097" id="3097">3097</a>
<a href="#3098" id="3098">3098</a>
<a href="#3099" id="3099">3099</a>
<a href="#3100" id="3100">3100</a>
<a href="#3101" id="3101">3101</a>
<a href="#3102" id="3102">3102</a>
<a href="#3103" id="3103">3103</a>
<a href="#3104" id="3104">3104</a>
<a href="#3105" id="3105">3105</a>
<a href="#3106" id="3106">3106</a>
<a href="#3107" id="3107">3107</a>
<a href="#3108" id="3108">3108</a>
<a href="#3109" id="3109">3109</a>
<a href="#3110" id="3110">3110</a>
<a href="#3111" id="3111">3111</a>
<a href="#3112" id="3112">3112</a>
<a href="#3113" id="3113">3113</a>
<a href="#3114" id="3114">3114</a>
<a href="#3115" id="3115">3115</a>
<a href="#3116" id="3116">3116</a>
<a href="#3117" id="3117">3117</a>
<a href="#3118" id="3118">3118</a>
<a href="#3119" id="3119">3119</a>
<a href="#3120" id="3120">3120</a>
<a href="#3121" id="3121">3121</a>
<a href="#3122" id="3122">3122</a>
<a href="#3123" id="3123">3123</a>
<a href="#3124" id="3124">3124</a>
<a href="#3125" id="3125">3125</a>
<a href="#3126" id="3126">3126</a>
<a href="#3127" id="3127">3127</a>
<a href="#3128" id="3128">3128</a>
<a href="#3129" id="3129">3129</a>
<a href="#3130" id="3130">3130</a>
<a href="#3131" id="3131">3131</a>
<a href="#3132" id="3132">3132</a>
<a href="#3133" id="3133">3133</a>
<a href="#3134" id="3134">3134</a>
<a href="#3135" id="3135">3135</a>
<a href="#3136" id="3136">3136</a>
<a href="#3137" id="3137">3137</a>
<a href="#3138" id="3138">3138</a>
<a href="#3139" id="3139">3139</a>
<a href="#3140" id="3140">3140</a>
<a href="#3141" id="3141">3141</a>
<a href="#3142" id="3142">3142</a>
<a href="#3143" id="3143">3143</a>
<a href="#3144" id="3144">3144</a>
<a href="#3145" id="3145">3145</a>
<a href="#3146" id="3146">3146</a>
<a href="#3147" id="3147">3147</a>
<a href="#3148" id="3148">3148</a>
<a href="#3149" id="3149">3149</a>
<a href="#3150" id="3150">3150</a>
<a href="#3151" id="3151">3151</a>
<a href="#3152" id="3152">3152</a>
<a href="#3153" id="3153">3153</a>
<a href="#3154" id="3154">3154</a>
<a href="#3155" id="3155">3155</a>
<a href="#3156" id="3156">3156</a>
<a href="#3157" id="3157">3157</a>
<a href="#3158" id="3158">3158</a>
<a href="#3159" id="3159">3159</a>
<a href="#3160" id="3160">3160</a>
<a href="#3161" id="3161">3161</a>
<a href="#3162" id="3162">3162</a>
<a href="#3163" id="3163">3163</a>
<a href="#3164" id="3164">3164</a>
<a href="#3165" id="3165">3165</a>
<a href="#3166" id="3166">3166</a>
<a href="#3167" id="3167">3167</a>
<a href="#3168" id="3168">3168</a>
<a href="#3169" id="3169">3169</a>
<a href="#3170" id="3170">3170</a>
<a href="#3171" id="3171">3171</a>
<a href="#3172" id="3172">3172</a>
<a href="#3173" id="3173">3173</a>
<a href="#3174" id="3174">3174</a>
<a href="#3175" id="3175">3175</a>
<a href="#3176" id="3176">3176</a>
<a href="#3177" id="3177">3177</a>
<a href="#3178" id="3178">3178</a>
<a href="#3179" id="3179">3179</a>
<a href="#3180" id="3180">3180</a>
<a href="#3181" id="3181">3181</a>
<a href="#3182" id="3182">3182</a>
<a href="#3183" id="3183">3183</a>
<a href="#3184" id="3184">3184</a>
<a href="#3185" id="3185">3185</a>
<a href="#3186" id="3186">3186</a>
<a href="#3187" id="3187">3187</a>
<a href="#3188" id="3188">3188</a>
<a href="#3189" id="3189">3189</a>
<a href="#3190" id="3190">3190</a>
<a href="#3191" id="3191">3191</a>
<a href="#3192" id="3192">3192</a>
<a href="#3193" id="3193">3193</a>
<a href="#3194" id="3194">3194</a>
<a href="#3195" id="3195">3195</a>
<a href="#3196" id="3196">3196</a>
<a href="#3197" id="3197">3197</a>
<a href="#3198" id="3198">3198</a>
<a href="#3199" id="3199">3199</a>
<a href="#3200" id="3200">3200</a>
<a href="#3201" id="3201">3201</a>
<a href="#3202" id="3202">3202</a>
<a href="#3203" id="3203">3203</a>
<a href="#3204" id="3204">3204</a>
<a href="#3205" id="3205">3205</a>
<a href="#3206" id="3206">3206</a>
<a href="#3207" id="3207">3207</a>
<a href="#3208" id="3208">3208</a>
<a href="#3209" id="3209">3209</a>
<a href="#3210" id="3210">3210</a>
<a href="#3211" id="3211">3211</a>
<a href="#3212" id="3212">3212</a>
<a href="#3213" id="3213">3213</a>
<a href="#3214" id="3214">3214</a>
<a href="#3215" id="3215">3215</a>
<a href="#3216" id="3216">3216</a>
<a href="#3217" id="3217">3217</a>
<a href="#3218" id="3218">3218</a>
<a href="#3219" id="3219">3219</a>
<a href="#3220" id="3220">3220</a>
<a href="#3221" id="3221">3221</a>
<a href="#3222" id="3222">3222</a>
<a href="#3223" id="3223">3223</a>
<a href="#3224" id="3224">3224</a>
<a href="#3225" id="3225">3225</a>
<a href="#3226" id="3226">3226</a>
<a href="#3227" id="3227">3227</a>
<a href="#3228" id="3228">3228</a>
<a href="#3229" id="3229">3229</a>
<a href="#3230" id="3230">3230</a>
<a href="#3231" id="3231">3231</a>
<a href="#3232" id="3232">3232</a>
<a href="#3233" id="3233">3233</a>
<a href="#3234" id="3234">3234</a>
<a href="#3235" id="3235">3235</a>
<a href="#3236" id="3236">3236</a>
<a href="#3237" id="3237">3237</a>
<a href="#3238" id="3238">3238</a>
<a href="#3239" id="3239">3239</a>
<a href="#3240" id="3240">3240</a>
<a href="#3241" id="3241">3241</a>
<a href="#3242" id="3242">3242</a>
<a href="#3243" id="3243">3243</a>
<a href="#3244" id="3244">3244</a>
<a href="#3245" id="3245">3245</a>
<a href="#3246" id="3246">3246</a>
<a href="#3247" id="3247">3247</a>
<a href="#3248" id="3248">3248</a>
<a href="#3249" id="3249">3249</a>
<a href="#3250" id="3250">3250</a>
<a href="#3251" id="3251">3251</a>
<a href="#3252" id="3252">3252</a>
<a href="#3253" id="3253">3253</a>
<a href="#3254" id="3254">3254</a>
<a href="#3255" id="3255">3255</a>
<a href="#3256" id="3256">3256</a>
<a href="#3257" id="3257">3257</a>
<a href="#3258" id="3258">3258</a>
<a href="#3259" id="3259">3259</a>
<a href="#3260" id="3260">3260</a>
<a href="#3261" id="3261">3261</a>
<a href="#3262" id="3262">3262</a>
<a href="#3263" id="3263">3263</a>
<a href="#3264" id="3264">3264</a>
<a href="#3265" id="3265">3265</a>
<a href="#3266" id="3266">3266</a>
<a href="#3267" id="3267">3267</a>
<a href="#3268" id="3268">3268</a>
<a href="#3269" id="3269">3269</a>
<a href="#3270" id="3270">3270</a>
<a href="#3271" id="3271">3271</a>
<a href="#3272" id="3272">3272</a>
<a href="#3273" id="3273">3273</a>
<a href="#3274" id="3274">3274</a>
<a href="#3275" id="3275">3275</a>
<a href="#3276" id="3276">3276</a>
<a href="#3277" id="3277">3277</a>
<a href="#3278" id="3278">3278</a>
<a href="#3279" id="3279">3279</a>
<a href="#3280" id="3280">3280</a>
<a href="#3281" id="3281">3281</a>
<a href="#3282" id="3282">3282</a>
<a href="#3283" id="3283">3283</a>
<a href="#3284" id="3284">3284</a>
<a href="#3285" id="3285">3285</a>
<a href="#3286" id="3286">3286</a>
<a href="#3287" id="3287">3287</a>
<a href="#3288" id="3288">3288</a>
<a href="#3289" id="3289">3289</a>
<a href="#3290" id="3290">3290</a>
<a href="#3291" id="3291">3291</a>
<a href="#3292" id="3292">3292</a>
<a href="#3293" id="3293">3293</a>
<a href="#3294" id="3294">3294</a>
<a href="#3295" id="3295">3295</a>
<a href="#3296" id="3296">3296</a>
<a href="#3297" id="3297">3297</a>
<a href="#3298" id="3298">3298</a>
<a href="#3299" id="3299">3299</a>
<a href="#3300" id="3300">3300</a>
<a href="#3301" id="3301">3301</a>
<a href="#3302" id="3302">3302</a>
<a href="#3303" id="3303">3303</a>
<a href="#3304" id="3304">3304</a>
<a href="#3305" id="3305">3305</a>
<a href="#3306" id="3306">3306</a>
<a href="#3307" id="3307">3307</a>
<a href="#3308" id="3308">3308</a>
<a href="#3309" id="3309">3309</a>
<a href="#3310" id="3310">3310</a>
<a href="#3311" id="3311">3311</a>
<a href="#3312" id="3312">3312</a>
<a href="#3313" id="3313">3313</a>
<a href="#3314" id="3314">3314</a>
<a href="#3315" id="3315">3315</a>
<a href="#3316" id="3316">3316</a>
<a href="#3317" id="3317">3317</a>
<a href="#3318" id="3318">3318</a>
<a href="#3319" id="3319">3319</a>
<a href="#3320" id="3320">3320</a>
<a href="#3321" id="3321">3321</a>
<a href="#3322" id="3322">3322</a>
<a href="#3323" id="3323">3323</a>
<a href="#3324" id="3324">3324</a>
<a href="#3325" id="3325">3325</a>
<a href="#3326" id="3326">3326</a>
<a href="#3327" id="3327">3327</a>
<a href="#3328" id="3328">3328</a>
<a href="#3329" id="3329">3329</a>
<a href="#3330" id="3330">3330</a>
<a href="#3331" id="3331">3331</a>
<a href="#3332" id="3332">3332</a>
<a href="#3333" id="3333">3333</a>
<a href="#3334" id="3334">3334</a>
<a href="#3335" id="3335">3335</a>
<a href="#3336" id="3336">3336</a>
<a href="#3337" id="3337">3337</a>
<a href="#3338" id="3338">3338</a>
<a href="#3339" id="3339">3339</a>
<a href="#3340" id="3340">3340</a>
<a href="#3341" id="3341">3341</a>
<a href="#3342" id="3342">3342</a>
<a href="#3343" id="3343">3343</a>
<a href="#3344" id="3344">3344</a>
<a href="#3345" id="3345">3345</a>
<a href="#3346" id="3346">3346</a>
<a href="#3347" id="3347">3347</a>
<a href="#3348" id="3348">3348</a>
<a href="#3349" id="3349">3349</a>
<a href="#3350" id="3350">3350</a>
<a href="#3351" id="3351">3351</a>
<a href="#3352" id="3352">3352</a>
<a href="#3353" id="3353">3353</a>
<a href="#3354" id="3354">3354</a>
<a href="#3355" id="3355">3355</a>
<a href="#3356" id="3356">3356</a>
<a href="#3357" id="3357">3357</a>
<a href="#3358" id="3358">3358</a>
<a href="#3359" id="3359">3359</a>
<a href="#3360" id="3360">3360</a>
<a href="#3361" id="3361">3361</a>
<a href="#3362" id="3362">3362</a>
<a href="#3363" id="3363">3363</a>
<a href="#3364" id="3364">3364</a>
<a href="#3365" id="3365">3365</a>
<a href="#3366" id="3366">3366</a>
<a href="#3367" id="3367">3367</a>
<a href="#3368" id="3368">3368</a>
<a href="#3369" id="3369">3369</a>
<a href="#3370" id="3370">3370</a>
<a href="#3371" id="3371">3371</a>
<a href="#3372" id="3372">3372</a>
<a href="#3373" id="3373">3373</a>
<a href="#3374" id="3374">3374</a>
<a href="#3375" id="3375">3375</a>
<a href="#3376" id="3376">3376</a>
<a href="#3377" id="3377">3377</a>
<a href="#3378" id="3378">3378</a>
<a href="#3379" id="3379">3379</a>
<a href="#3380" id="3380">3380</a>
<a href="#3381" id="3381">3381</a>
<a href="#3382" id="3382">3382</a>
<a href="#3383" id="3383">3383</a>
<a href="#3384" id="3384">3384</a>
<a href="#3385" id="3385">3385</a>
<a href="#3386" id="3386">3386</a>
<a href="#3387" id="3387">3387</a>
<a href="#3388" id="3388">3388</a>
<a href="#3389" id="3389">3389</a>
<a href="#3390" id="3390">3390</a>
<a href="#3391" id="3391">3391</a>
<a href="#3392" id="3392">3392</a>
<a href="#3393" id="3393">3393</a>
<a href="#3394" id="3394">3394</a>
<a href="#3395" id="3395">3395</a>
<a href="#3396" id="3396">3396</a>
<a href="#3397" id="3397">3397</a>
<a href="#3398" id="3398">3398</a>
<a href="#3399" id="3399">3399</a>
<a href="#3400" id="3400">3400</a>
<a href="#3401" id="3401">3401</a>
<a href="#3402" id="3402">3402</a>
<a href="#3403" id="3403">3403</a>
<a href="#3404" id="3404">3404</a>
<a href="#3405" id="3405">3405</a>
<a href="#3406" id="3406">3406</a>
<a href="#3407" id="3407">3407</a>
<a href="#3408" id="3408">3408</a>
<a href="#3409" id="3409">3409</a>
<a href="#3410" id="3410">3410</a>
<a href="#3411" id="3411">3411</a>
<a href="#3412" id="3412">3412</a>
<a href="#3413" id="3413">3413</a>
<a href="#3414" id="3414">3414</a>
<a href="#3415" id="3415">3415</a>
<a href="#3416" id="3416">3416</a>
<a href="#3417" id="3417">3417</a>
<a href="#3418" id="3418">3418</a>
<a href="#3419" id="3419">3419</a>
<a href="#3420" id="3420">3420</a>
<a href="#3421" id="3421">3421</a>
<a href="#3422" id="3422">3422</a>
<a href="#3423" id="3423">3423</a>
<a href="#3424" id="3424">3424</a>
<a href="#3425" id="3425">3425</a>
<a href="#3426" id="3426">3426</a>
<a href="#3427" id="3427">3427</a>
<a href="#3428" id="3428">3428</a>
<a href="#3429" id="3429">3429</a>
<a href="#3430" id="3430">3430</a>
<a href="#3431" id="3431">3431</a>
<a href="#3432" id="3432">3432</a>
<a href="#3433" id="3433">3433</a>
<a href="#3434" id="3434">3434</a>
<a href="#3435" id="3435">3435</a>
<a href="#3436" id="3436">3436</a>
<a href="#3437" id="3437">3437</a>
<a href="#3438" id="3438">3438</a>
<a href="#3439" id="3439">3439</a>
<a href="#3440" id="3440">3440</a>
<a href="#3441" id="3441">3441</a>
<a href="#3442" id="3442">3442</a>
<a href="#3443" id="3443">3443</a>
<a href="#3444" id="3444">3444</a>
<a href="#3445" id="3445">3445</a>
<a href="#3446" id="3446">3446</a>
<a href="#3447" id="3447">3447</a>
<a href="#3448" id="3448">3448</a>
<a href="#3449" id="3449">3449</a>
<a href="#3450" id="3450">3450</a>
<a href="#3451" id="3451">3451</a>
<a href="#3452" id="3452">3452</a>
<a href="#3453" id="3453">3453</a>
<a href="#3454" id="3454">3454</a>
<a href="#3455" id="3455">3455</a>
<a href="#3456" id="3456">3456</a>
<a href="#3457" id="3457">3457</a>
<a href="#3458" id="3458">3458</a>
<a href="#3459" id="3459">3459</a>
<a href="#3460" id="3460">3460</a>
<a href="#3461" id="3461">3461</a>
<a href="#3462" id="3462">3462</a>
<a href="#3463" id="3463">3463</a>
<a href="#3464" id="3464">3464</a>
<a href="#3465" id="3465">3465</a>
<a href="#3466" id="3466">3466</a>
<a href="#3467" id="3467">3467</a>
<a href="#3468" id="3468">3468</a>
<a href="#3469" id="3469">3469</a>
<a href="#3470" id="3470">3470</a>
<a href="#3471" id="3471">3471</a>
<a href="#3472" id="3472">3472</a>
<a href="#3473" id="3473">3473</a>
<a href="#3474" id="3474">3474</a>
<a href="#3475" id="3475">3475</a>
<a href="#3476" id="3476">3476</a>
<a href="#3477" id="3477">3477</a>
<a href="#3478" id="3478">3478</a>
<a href="#3479" id="3479">3479</a>
<a href="#3480" id="3480">3480</a>
<a href="#3481" id="3481">3481</a>
<a href="#3482" id="3482">3482</a>
<a href="#3483" id="3483">3483</a>
<a href="#3484" id="3484">3484</a>
<a href="#3485" id="3485">3485</a>
<a href="#3486" id="3486">3486</a>
<a href="#3487" id="3487">3487</a>
<a href="#3488" id="3488">3488</a>
<a href="#3489" id="3489">3489</a>
<a href="#3490" id="3490">3490</a>
<a href="#3491" id="3491">3491</a>
<a href="#3492" id="3492">3492</a>
<a href="#3493" id="3493">3493</a>
<a href="#3494" id="3494">3494</a>
<a href="#3495" id="3495">3495</a>
<a href="#3496" id="3496">3496</a>
<a href="#3497" id="3497">3497</a>
<a href="#3498" id="3498">3498</a>
<a href="#3499" id="3499">3499</a>
<a href="#3500" id="3500">3500</a>
<a href="#3501" id="3501">3501</a>
<a href="#3502" id="3502">3502</a>
<a href="#3503" id="3503">3503</a>
<a href="#3504" id="3504">3504</a>
<a href="#3505" id="3505">3505</a>
<a href="#3506" id="3506">3506</a>
<a href="#3507" id="3507">3507</a>
<a href="#3508" id="3508">3508</a>
<a href="#3509" id="3509">3509</a>
<a href="#3510" id="3510">3510</a>
<a href="#3511" id="3511">3511</a>
<a href="#3512" id="3512">3512</a>
<a href="#3513" id="3513">3513</a>
<a href="#3514" id="3514">3514</a>
<a href="#3515" id="3515">3515</a>
<a href="#3516" id="3516">3516</a>
<a href="#3517" id="3517">3517</a>
<a href="#3518" id="3518">3518</a>
<a href="#3519" id="3519">3519</a>
<a href="#3520" id="3520">3520</a>
<a href="#3521" id="3521">3521</a>
<a href="#3522" id="3522">3522</a>
<a href="#3523" id="3523">3523</a>
<a href="#3524" id="3524">3524</a>
<a href="#3525" id="3525">3525</a>
<a href="#3526" id="3526">3526</a>
<a href="#3527" id="3527">3527</a>
<a href="#3528" id="3528">3528</a>
<a href="#3529" id="3529">3529</a>
<a href="#3530" id="3530">3530</a>
<a href="#3531" id="3531">3531</a>
<a href="#3532" id="3532">3532</a>
<a href="#3533" id="3533">3533</a>
<a href="#3534" id="3534">3534</a>
<a href="#3535" id="3535">3535</a>
<a href="#3536" id="3536">3536</a>
<a href="#3537" id="3537">3537</a>
<a href="#3538" id="3538">3538</a>
<a href="#3539" id="3539">3539</a>
<a href="#3540" id="3540">3540</a>
<a href="#3541" id="3541">3541</a>
<a href="#3542" id="3542">3542</a>
<a href="#3543" id="3543">3543</a>
<a href="#3544" id="3544">3544</a>
<a href="#3545" id="3545">3545</a>
<a href="#3546" id="3546">3546</a>
<a href="#3547" id="3547">3547</a>
<a href="#3548" id="3548">3548</a>
<a href="#3549" id="3549">3549</a>
<a href="#3550" id="3550">3550</a>
<a href="#3551" id="3551">3551</a>
<a href="#3552" id="3552">3552</a>
<a href="#3553" id="3553">3553</a>
<a href="#3554" id="3554">3554</a>
<a href="#3555" id="3555">3555</a>
<a href="#3556" id="3556">3556</a>
<a href="#3557" id="3557">3557</a>
<a href="#3558" id="3558">3558</a>
<a href="#3559" id="3559">3559</a>
<a href="#3560" id="3560">3560</a>
<a href="#3561" id="3561">3561</a>
<a href="#3562" id="3562">3562</a>
<a href="#3563" id="3563">3563</a>
<a href="#3564" id="3564">3564</a>
<a href="#3565" id="3565">3565</a>
<a href="#3566" id="3566">3566</a>
<a href="#3567" id="3567">3567</a>
<a href="#3568" id="3568">3568</a>
<a href="#3569" id="3569">3569</a>
<a href="#3570" id="3570">3570</a>
<a href="#3571" id="3571">3571</a>
<a href="#3572" id="3572">3572</a>
<a href="#3573" id="3573">3573</a>
<a href="#3574" id="3574">3574</a>
<a href="#3575" id="3575">3575</a>
<a href="#3576" id="3576">3576</a>
<a href="#3577" id="3577">3577</a>
<a href="#3578" id="3578">3578</a>
<a href="#3579" id="3579">3579</a>
<a href="#3580" id="3580">3580</a>
<a href="#3581" id="3581">3581</a>
<a href="#3582" id="3582">3582</a>
<a href="#3583" id="3583">3583</a>
<a href="#3584" id="3584">3584</a>
<a href="#3585" id="3585">3585</a>
<a href="#3586" id="3586">3586</a>
<a href="#3587" id="3587">3587</a>
<a href="#3588" id="3588">3588</a>
<a href="#3589" id="3589">3589</a>
<a href="#3590" id="3590">3590</a>
<a href="#3591" id="3591">3591</a>
<a href="#3592" id="3592">3592</a>
<a href="#3593" id="3593">3593</a>
<a href="#3594" id="3594">3594</a>
<a href="#3595" id="3595">3595</a>
<a href="#3596" id="3596">3596</a>
<a href="#3597" id="3597">3597</a>
<a href="#3598" id="3598">3598</a>
<a href="#3599" id="3599">3599</a>
<a href="#3600" id="3600">3600</a>
<a href="#3601" id="3601">3601</a>
<a href="#3602" id="3602">3602</a>
<a href="#3603" id="3603">3603</a>
<a href="#3604" id="3604">3604</a>
<a href="#3605" id="3605">3605</a>
<a href="#3606" id="3606">3606</a>
<a href="#3607" id="3607">3607</a>
<a href="#3608" id="3608">3608</a>
<a href="#3609" id="3609">3609</a>
<a href="#3610" id="3610">3610</a>
<a href="#3611" id="3611">3611</a>
<a href="#3612" id="3612">3612</a>
<a href="#3613" id="3613">3613</a>
<a href="#3614" id="3614">3614</a>
<a href="#3615" id="3615">3615</a>
<a href="#3616" id="3616">3616</a>
<a href="#3617" id="3617">3617</a>
<a href="#3618" id="3618">3618</a>
<a href="#3619" id="3619">3619</a>
<a href="#3620" id="3620">3620</a>
<a href="#3621" id="3621">3621</a>
<a href="#3622" id="3622">3622</a>
<a href="#3623" id="3623">3623</a>
<a href="#3624" id="3624">3624</a>
<a href="#3625" id="3625">3625</a>
<a href="#3626" id="3626">3626</a>
<a href="#3627" id="3627">3627</a>
<a href="#3628" id="3628">3628</a>
<a href="#3629" id="3629">3629</a>
<a href="#3630" id="3630">3630</a>
<a href="#3631" id="3631">3631</a>
<a href="#3632" id="3632">3632</a>
<a href="#3633" id="3633">3633</a>
<a href="#3634" id="3634">3634</a>
<a href="#3635" id="3635">3635</a>
<a href="#3636" id="3636">3636</a>
<a href="#3637" id="3637">3637</a>
<a href="#3638" id="3638">3638</a>
<a href="#3639" id="3639">3639</a>
<a href="#3640" id="3640">3640</a>
<a href="#3641" id="3641">3641</a>
<a href="#3642" id="3642">3642</a>
<a href="#3643" id="3643">3643</a>
<a href="#3644" id="3644">3644</a>
<a href="#3645" id="3645">3645</a>
<a href="#3646" id="3646">3646</a>
<a href="#3647" id="3647">3647</a>
<a href="#3648" id="3648">3648</a>
<a href="#3649" id="3649">3649</a>
<a href="#3650" id="3650">3650</a>
<a href="#3651" id="3651">3651</a>
<a href="#3652" id="3652">3652</a>
<a href="#3653" id="3653">3653</a>
<a href="#3654" id="3654">3654</a>
<a href="#3655" id="3655">3655</a>
<a href="#3656" id="3656">3656</a>
</pre></div><pre class="rust"><code><span class="doccomment">//! Traits for writing parallel programs using an iterator-style interface
//!
//! You will rarely need to interact with this module directly unless you have
//! need to name one of the iterator types.
//!
//! Parallel iterators make it easy to write iterator-like chains that
//! execute in parallel: typically all you have to do is convert the
//! first `.iter()` (or `iter_mut()`, `into_iter()`, etc) method into
//! `par_iter()` (or `par_iter_mut()`, `into_par_iter()`, etc). For
//! example, to compute the sum of the squares of a sequence of
//! integers, one might write:
//!
//! ```rust
//! use rayon::prelude::*;
//! fn sum_of_squares(input: &amp;[i32]) -&gt; i32 {
//!     input.par_iter()
//!          .map(|i| i * i)
//!          .sum()
//! }
//! ```
//!
//! Or, to increment all the integers in a slice, you could write:
//!
//! ```rust
//! use rayon::prelude::*;
//! fn increment_all(input: &amp;mut [i32]) {
//!     input.par_iter_mut()
//!          .for_each(|p| *p += 1);
//! }
//! ```
//!
//! To use parallel iterators, first import the traits by adding
//! something like `use rayon::prelude::*` to your module. You can
//! then call `par_iter`, `par_iter_mut`, or `into_par_iter` to get a
//! parallel iterator. Like a [regular iterator][], parallel
//! iterators work by first constructing a computation and then
//! executing it.
//!
//! In addition to `par_iter()` and friends, some types offer other
//! ways to create (or consume) parallel iterators:
//!
//! - Slices (`&amp;[T]`, `&amp;mut [T]`) offer methods like `par_split` and
//!   `par_windows`, as well as various parallel sorting
//!   operations. See [the `ParallelSlice` trait] for the full list.
//! - Strings (`&amp;str`) offer methods like `par_split` and `par_lines`.
//!   See [the `ParallelString` trait] for the full list.
//! - Various collections offer [`par_extend`], which grows a
//!   collection given a parallel iterator. (If you don't have a
//!   collection to extend, you can use [`collect()`] to create a new
//!   one from scratch.)
//!
//! [the `ParallelSlice` trait]: ../slice/trait.ParallelSlice.html
//! [the `ParallelString` trait]: ../str/trait.ParallelString.html
//! [`par_extend`]: trait.ParallelExtend.html
//! [`collect()`]: trait.ParallelIterator.html#method.collect
//!
//! To see the full range of methods available on parallel iterators,
//! check out the [`ParallelIterator`] and [`IndexedParallelIterator`]
//! traits.
//!
//! If you'd like to build a custom parallel iterator, or to write your own
//! combinator, then check out the [split] function and the [plumbing] module.
//!
//! [regular iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
//! [`ParallelIterator`]: trait.ParallelIterator.html
//! [`IndexedParallelIterator`]: trait.IndexedParallelIterator.html
//! [split]: fn.split.html
//! [plumbing]: plumbing/index.html
//!
//! Note: Several of the `ParallelIterator` methods rely on a `Try` trait which
//! has been deliberately obscured from the public API.  This trait is intended
//! to mirror the unstable `std::ops::Try` with implementations for `Option` and
//! `Result`, where `Some`/`Ok` values will let those iterators continue, but
//! `None`/`Err` values will exit early.
//!
//! A note about object safety: It is currently _not_ possible to wrap
//! a `ParallelIterator` (or any trait that depends on it) using a
//! `Box&lt;dyn ParallelIterator&gt;` or other kind of dynamic allocation,
//! because `ParallelIterator` is **not object-safe**.
//! (This keeps the implementation simpler and allows extra optimizations.)

</span><span class="kw">use </span><span class="self">self</span>::plumbing::<span class="kw-2">*</span>;
<span class="kw">use </span><span class="self">self</span>::private::Try;
<span class="kw">pub use </span>either::Either;
<span class="kw">use </span>std::cmp::Ordering;
<span class="kw">use </span>std::collections::LinkedList;
<span class="kw">use </span>std::iter::{Product, Sum};
<span class="kw">use </span>std::ops::{Fn, RangeBounds};

<span class="kw">pub mod </span>plumbing;

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>test;

<span class="comment">// There is a method to the madness here:
//
// - These modules are private but expose certain types to the end-user
//   (e.g., `enumerate::Enumerate`) -- specifically, the types that appear in the
//   public API surface of the `ParallelIterator` traits.
// - In **this** module, those public types are always used unprefixed, which forces
//   us to add a `pub use` and helps identify if we missed anything.
// - In contrast, items that appear **only** in the body of a method,
//   e.g. `find::find()`, are always used **prefixed**, so that they
//   can be readily distinguished.

</span><span class="kw">mod </span>blocks;
<span class="kw">mod </span>chain;
<span class="kw">mod </span>chunks;
<span class="kw">mod </span>cloned;
<span class="kw">mod </span>collect;
<span class="kw">mod </span>copied;
<span class="kw">mod </span>empty;
<span class="kw">mod </span>enumerate;
<span class="kw">mod </span>extend;
<span class="kw">mod </span>filter;
<span class="kw">mod </span>filter_map;
<span class="kw">mod </span>find;
<span class="kw">mod </span>find_first_last;
<span class="kw">mod </span>flat_map;
<span class="kw">mod </span>flat_map_iter;
<span class="kw">mod </span>flatten;
<span class="kw">mod </span>flatten_iter;
<span class="kw">mod </span>fold;
<span class="kw">mod </span>fold_chunks;
<span class="kw">mod </span>fold_chunks_with;
<span class="kw">mod </span>for_each;
<span class="kw">mod </span>from_par_iter;
<span class="kw">mod </span>inspect;
<span class="kw">mod </span>interleave;
<span class="kw">mod </span>interleave_shortest;
<span class="kw">mod </span>intersperse;
<span class="kw">mod </span>len;
<span class="kw">mod </span>map;
<span class="kw">mod </span>map_with;
<span class="kw">mod </span>multizip;
<span class="kw">mod </span>noop;
<span class="kw">mod </span>once;
<span class="kw">mod </span>panic_fuse;
<span class="kw">mod </span>par_bridge;
<span class="kw">mod </span>positions;
<span class="kw">mod </span>product;
<span class="kw">mod </span>reduce;
<span class="kw">mod </span>repeat;
<span class="kw">mod </span>rev;
<span class="kw">mod </span>skip;
<span class="kw">mod </span>skip_any;
<span class="kw">mod </span>skip_any_while;
<span class="kw">mod </span>splitter;
<span class="kw">mod </span>step_by;
<span class="kw">mod </span>sum;
<span class="kw">mod </span>take;
<span class="kw">mod </span>take_any;
<span class="kw">mod </span>take_any_while;
<span class="kw">mod </span>try_fold;
<span class="kw">mod </span>try_reduce;
<span class="kw">mod </span>try_reduce_with;
<span class="kw">mod </span>unzip;
<span class="kw">mod </span>update;
<span class="kw">mod </span>walk_tree;
<span class="kw">mod </span>while_some;
<span class="kw">mod </span>zip;
<span class="kw">mod </span>zip_eq;

<span class="kw">pub use </span><span class="self">self</span>::{
    blocks::{ExponentialBlocks, UniformBlocks},
    chain::Chain,
    chunks::Chunks,
    cloned::Cloned,
    copied::Copied,
    empty::{empty, Empty},
    enumerate::Enumerate,
    filter::Filter,
    filter_map::FilterMap,
    flat_map::FlatMap,
    flat_map_iter::FlatMapIter,
    flatten::Flatten,
    flatten_iter::FlattenIter,
    fold::{Fold, FoldWith},
    fold_chunks::FoldChunks,
    fold_chunks_with::FoldChunksWith,
    inspect::Inspect,
    interleave::Interleave,
    interleave_shortest::InterleaveShortest,
    intersperse::Intersperse,
    len::{MaxLen, MinLen},
    map::Map,
    map_with::{MapInit, MapWith},
    multizip::MultiZip,
    once::{once, Once},
    panic_fuse::PanicFuse,
    par_bridge::{IterBridge, ParallelBridge},
    positions::Positions,
    repeat::{repeat, repeatn, Repeat, RepeatN},
    rev::Rev,
    skip::Skip,
    skip_any::SkipAny,
    skip_any_while::SkipAnyWhile,
    splitter::{split, Split},
    step_by::StepBy,
    take::Take,
    take_any::TakeAny,
    take_any_while::TakeAnyWhile,
    try_fold::{TryFold, TryFoldWith},
    update::Update,
    walk_tree::{
        walk_tree, walk_tree_postfix, walk_tree_prefix, WalkTree, WalkTreePostfix, WalkTreePrefix,
    },
    while_some::WhileSome,
    zip::Zip,
    zip_eq::ZipEq,
};

<span class="doccomment">/// `IntoParallelIterator` implements the conversion to a [`ParallelIterator`].
///
/// By implementing `IntoParallelIterator` for a type, you define how it will
/// transformed into an iterator. This is a parallel version of the standard
/// library's [`std::iter::IntoIterator`] trait.
///
/// [`ParallelIterator`]: trait.ParallelIterator.html
/// [`std::iter::IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
</span><span class="kw">pub trait </span>IntoParallelIterator {
    <span class="doccomment">/// The parallel iterator type that will be created.
    </span><span class="kw">type </span>Iter: ParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;;

    <span class="doccomment">/// The type of item that the parallel iterator will produce.
    </span><span class="kw">type </span>Item: Send;

    <span class="doccomment">/// Converts `self` into a parallel iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// println!("counting in parallel:");
    /// (0..100).into_par_iter()
    ///     .for_each(|i| println!("{}", i));
    /// ```
    ///
    /// This conversion is often implicit for arguments to methods like [`zip`].
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let v: Vec&lt;_&gt; = (0..5).into_par_iter().zip(5..10).collect();
    /// assert_eq!(v, [(0, 5), (1, 6), (2, 7), (3, 8), (4, 9)]);
    /// ```
    ///
    /// [`zip`]: trait.IndexedParallelIterator.html#method.zip
    </span><span class="kw">fn </span>into_par_iter(<span class="self">self</span>) -&gt; <span class="self">Self</span>::Iter;
}

<span class="doccomment">/// `IntoParallelRefIterator` implements the conversion to a
/// [`ParallelIterator`], providing shared references to the data.
///
/// This is a parallel version of the `iter()` method
/// defined by various collections.
///
/// This trait is automatically implemented
/// `for I where &amp;I: IntoParallelIterator`. In most cases, users
/// will want to implement [`IntoParallelIterator`] rather than implement
/// this trait directly.
///
/// [`ParallelIterator`]: trait.ParallelIterator.html
/// [`IntoParallelIterator`]: trait.IntoParallelIterator.html
</span><span class="kw">pub trait </span>IntoParallelRefIterator&lt;<span class="lifetime">'data</span>&gt; {
    <span class="doccomment">/// The type of the parallel iterator that will be returned.
    </span><span class="kw">type </span>Iter: ParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;;

    <span class="doccomment">/// The type of item that the parallel iterator will produce.
    /// This will typically be an `&amp;'data T` reference type.
    </span><span class="kw">type </span>Item: Send + <span class="lifetime">'data</span>;

    <span class="doccomment">/// Converts `self` into a parallel iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let v: Vec&lt;_&gt; = (0..100).collect();
    /// assert_eq!(v.par_iter().sum::&lt;i32&gt;(), 100 * 99 / 2);
    ///
    /// // `v.par_iter()` is shorthand for `(&amp;v).into_par_iter()`,
    /// // producing the exact same references.
    /// assert!(v.par_iter().zip(&amp;v)
    ///          .all(|(a, b)| std::ptr::eq(a, b)));
    /// ```
    </span><span class="kw">fn </span>par_iter(<span class="kw-2">&amp;</span><span class="lifetime">'data </span><span class="self">self</span>) -&gt; <span class="self">Self</span>::Iter;
}

<span class="kw">impl</span>&lt;<span class="lifetime">'data</span>, I: <span class="lifetime">'data </span>+ <span class="question-mark">?</span>Sized&gt; IntoParallelRefIterator&lt;<span class="lifetime">'data</span>&gt; <span class="kw">for </span>I
<span class="kw">where
    </span><span class="kw-2">&amp;</span><span class="lifetime">'data </span>I: IntoParallelIterator,
{
    <span class="kw">type </span>Iter = &lt;<span class="kw-2">&amp;</span><span class="lifetime">'data </span>I <span class="kw">as </span>IntoParallelIterator&gt;::Iter;
    <span class="kw">type </span>Item = &lt;<span class="kw-2">&amp;</span><span class="lifetime">'data </span>I <span class="kw">as </span>IntoParallelIterator&gt;::Item;

    <span class="kw">fn </span>par_iter(<span class="kw-2">&amp;</span><span class="lifetime">'data </span><span class="self">self</span>) -&gt; <span class="self">Self</span>::Iter {
        <span class="self">self</span>.into_par_iter()
    }
}

<span class="doccomment">/// `IntoParallelRefMutIterator` implements the conversion to a
/// [`ParallelIterator`], providing mutable references to the data.
///
/// This is a parallel version of the `iter_mut()` method
/// defined by various collections.
///
/// This trait is automatically implemented
/// `for I where &amp;mut I: IntoParallelIterator`. In most cases, users
/// will want to implement [`IntoParallelIterator`] rather than implement
/// this trait directly.
///
/// [`ParallelIterator`]: trait.ParallelIterator.html
/// [`IntoParallelIterator`]: trait.IntoParallelIterator.html
</span><span class="kw">pub trait </span>IntoParallelRefMutIterator&lt;<span class="lifetime">'data</span>&gt; {
    <span class="doccomment">/// The type of iterator that will be created.
    </span><span class="kw">type </span>Iter: ParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;;

    <span class="doccomment">/// The type of item that will be produced; this is typically an
    /// `&amp;'data mut T` reference.
    </span><span class="kw">type </span>Item: Send + <span class="lifetime">'data</span>;

    <span class="doccomment">/// Creates the parallel iterator from `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let mut v = vec![0usize; 5];
    /// v.par_iter_mut().enumerate().for_each(|(i, x)| *x = i);
    /// assert_eq!(v, [0, 1, 2, 3, 4]);
    /// ```
    </span><span class="kw">fn </span>par_iter_mut(<span class="kw-2">&amp;</span><span class="lifetime">'data </span><span class="kw-2">mut </span><span class="self">self</span>) -&gt; <span class="self">Self</span>::Iter;
}

<span class="kw">impl</span>&lt;<span class="lifetime">'data</span>, I: <span class="lifetime">'data </span>+ <span class="question-mark">?</span>Sized&gt; IntoParallelRefMutIterator&lt;<span class="lifetime">'data</span>&gt; <span class="kw">for </span>I
<span class="kw">where
    </span><span class="kw-2">&amp;</span><span class="lifetime">'data </span><span class="kw-2">mut </span>I: IntoParallelIterator,
{
    <span class="kw">type </span>Iter = &lt;<span class="kw-2">&amp;</span><span class="lifetime">'data </span><span class="kw-2">mut </span>I <span class="kw">as </span>IntoParallelIterator&gt;::Iter;
    <span class="kw">type </span>Item = &lt;<span class="kw-2">&amp;</span><span class="lifetime">'data </span><span class="kw-2">mut </span>I <span class="kw">as </span>IntoParallelIterator&gt;::Item;

    <span class="kw">fn </span>par_iter_mut(<span class="kw-2">&amp;</span><span class="lifetime">'data </span><span class="kw-2">mut </span><span class="self">self</span>) -&gt; <span class="self">Self</span>::Iter {
        <span class="self">self</span>.into_par_iter()
    }
}

<span class="doccomment">/// Parallel version of the standard iterator trait.
///
/// The combinators on this trait are available on **all** parallel
/// iterators.  Additional methods can be found on the
/// [`IndexedParallelIterator`] trait: those methods are only
/// available for parallel iterators where the number of items is
/// known in advance (so, e.g., after invoking `filter`, those methods
/// become unavailable).
///
/// For examples of using parallel iterators, see [the docs on the
/// `iter` module][iter].
///
/// [iter]: index.html
/// [`IndexedParallelIterator`]: trait.IndexedParallelIterator.html
</span><span class="kw">pub trait </span>ParallelIterator: Sized + Send {
    <span class="doccomment">/// The type of item that this parallel iterator produces.
    /// For example, if you use the [`for_each`] method, this is the type of
    /// item that your closure will be invoked with.
    ///
    /// [`for_each`]: #method.for_each
    </span><span class="kw">type </span>Item: Send;

    <span class="doccomment">/// Executes `OP` on each item produced by the iterator, in parallel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// (0..100).into_par_iter().for_each(|x| println!("{:?}", x));
    /// ```
    </span><span class="kw">fn </span>for_each&lt;OP&gt;(<span class="self">self</span>, op: OP)
    <span class="kw">where
        </span>OP: Fn(<span class="self">Self</span>::Item) + Sync + Send,
    {
        for_each::for_each(<span class="self">self</span>, <span class="kw-2">&amp;</span>op)
    }

    <span class="doccomment">/// Executes `OP` on the given `init` value with each item produced by
    /// the iterator, in parallel.
    ///
    /// The `init` value will be cloned only as needed to be paired with
    /// the group of items in each rayon job.  It does not require the type
    /// to be `Sync`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::mpsc::channel;
    /// use rayon::prelude::*;
    ///
    /// let (sender, receiver) = channel();
    ///
    /// (0..5).into_par_iter().for_each_with(sender, |s, x| s.send(x).unwrap());
    ///
    /// let mut res: Vec&lt;_&gt; = receiver.iter().collect();
    ///
    /// res.sort();
    ///
    /// assert_eq!(&amp;res[..], &amp;[0, 1, 2, 3, 4])
    /// ```
    </span><span class="kw">fn </span>for_each_with&lt;OP, T&gt;(<span class="self">self</span>, init: T, op: OP)
    <span class="kw">where
        </span>OP: Fn(<span class="kw-2">&amp;mut </span>T, <span class="self">Self</span>::Item) + Sync + Send,
        T: Send + Clone,
    {
        <span class="self">self</span>.map_with(init, op).collect()
    }

    <span class="doccomment">/// Executes `OP` on a value returned by `init` with each item produced by
    /// the iterator, in parallel.
    ///
    /// The `init` function will be called only as needed for a value to be
    /// paired with the group of items in each rayon job.  There is no
    /// constraint on that returned type at all!
    ///
    /// # Examples
    ///
    /// ```
    /// use rand::Rng;
    /// use rayon::prelude::*;
    ///
    /// let mut v = vec![0u8; 1_000_000];
    ///
    /// v.par_chunks_mut(1000)
    ///     .for_each_init(
    ///         || rand::thread_rng(),
    ///         |rng, chunk| rng.fill(chunk),
    ///     );
    ///
    /// // There's a remote chance that this will fail...
    /// for i in 0u8..=255 {
    ///     assert!(v.contains(&amp;i));
    /// }
    /// ```
    </span><span class="kw">fn </span>for_each_init&lt;OP, INIT, T&gt;(<span class="self">self</span>, init: INIT, op: OP)
    <span class="kw">where
        </span>OP: Fn(<span class="kw-2">&amp;mut </span>T, <span class="self">Self</span>::Item) + Sync + Send,
        INIT: Fn() -&gt; T + Sync + Send,
    {
        <span class="self">self</span>.map_init(init, op).collect()
    }

    <span class="doccomment">/// Executes a fallible `OP` on each item produced by the iterator, in parallel.
    ///
    /// If the `OP` returns `Result::Err` or `Option::None`, we will attempt to
    /// stop processing the rest of the items in the iterator as soon as
    /// possible, and we will return that terminating value.  Otherwise, we will
    /// return an empty `Result::Ok(())` or `Option::Some(())`.  If there are
    /// multiple errors in parallel, it is not specified which will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use std::io::{self, Write};
    ///
    /// // This will stop iteration early if there's any write error, like
    /// // having piped output get closed on the other end.
    /// (0..100).into_par_iter()
    ///     .try_for_each(|x| writeln!(io::stdout(), "{:?}", x))
    ///     .expect("expected no write errors");
    /// ```
    </span><span class="kw">fn </span>try_for_each&lt;OP, R&gt;(<span class="self">self</span>, op: OP) -&gt; R
    <span class="kw">where
        </span>OP: Fn(<span class="self">Self</span>::Item) -&gt; R + Sync + Send,
        R: Try&lt;Output = ()&gt; + Send,
    {
        <span class="kw">fn </span>ok&lt;R: Try&lt;Output = ()&gt;&gt;(<span class="kw">_</span>: (), <span class="kw">_</span>: ()) -&gt; R {
            R::from_output(())
        }

        <span class="self">self</span>.map(op).try_reduce(&lt;()&gt;::default, ok)
    }

    <span class="doccomment">/// Executes a fallible `OP` on the given `init` value with each item
    /// produced by the iterator, in parallel.
    ///
    /// This combines the `init` semantics of [`for_each_with()`] and the
    /// failure semantics of [`try_for_each()`].
    ///
    /// [`for_each_with()`]: #method.for_each_with
    /// [`try_for_each()`]: #method.try_for_each
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::mpsc::channel;
    /// use rayon::prelude::*;
    ///
    /// let (sender, receiver) = channel();
    ///
    /// (0..5).into_par_iter()
    ///     .try_for_each_with(sender, |s, x| s.send(x))
    ///     .expect("expected no send errors");
    ///
    /// let mut res: Vec&lt;_&gt; = receiver.iter().collect();
    ///
    /// res.sort();
    ///
    /// assert_eq!(&amp;res[..], &amp;[0, 1, 2, 3, 4])
    /// ```
    </span><span class="kw">fn </span>try_for_each_with&lt;OP, T, R&gt;(<span class="self">self</span>, init: T, op: OP) -&gt; R
    <span class="kw">where
        </span>OP: Fn(<span class="kw-2">&amp;mut </span>T, <span class="self">Self</span>::Item) -&gt; R + Sync + Send,
        T: Send + Clone,
        R: Try&lt;Output = ()&gt; + Send,
    {
        <span class="kw">fn </span>ok&lt;R: Try&lt;Output = ()&gt;&gt;(<span class="kw">_</span>: (), <span class="kw">_</span>: ()) -&gt; R {
            R::from_output(())
        }

        <span class="self">self</span>.map_with(init, op).try_reduce(&lt;()&gt;::default, ok)
    }

    <span class="doccomment">/// Executes a fallible `OP` on a value returned by `init` with each item
    /// produced by the iterator, in parallel.
    ///
    /// This combines the `init` semantics of [`for_each_init()`] and the
    /// failure semantics of [`try_for_each()`].
    ///
    /// [`for_each_init()`]: #method.for_each_init
    /// [`try_for_each()`]: #method.try_for_each
    ///
    /// # Examples
    ///
    /// ```
    /// use rand::Rng;
    /// use rayon::prelude::*;
    ///
    /// let mut v = vec![0u8; 1_000_000];
    ///
    /// v.par_chunks_mut(1000)
    ///     .try_for_each_init(
    ///         || rand::thread_rng(),
    ///         |rng, chunk| rng.try_fill(chunk),
    ///     )
    ///     .expect("expected no rand errors");
    ///
    /// // There's a remote chance that this will fail...
    /// for i in 0u8..=255 {
    ///     assert!(v.contains(&amp;i));
    /// }
    /// ```
    </span><span class="kw">fn </span>try_for_each_init&lt;OP, INIT, T, R&gt;(<span class="self">self</span>, init: INIT, op: OP) -&gt; R
    <span class="kw">where
        </span>OP: Fn(<span class="kw-2">&amp;mut </span>T, <span class="self">Self</span>::Item) -&gt; R + Sync + Send,
        INIT: Fn() -&gt; T + Sync + Send,
        R: Try&lt;Output = ()&gt; + Send,
    {
        <span class="kw">fn </span>ok&lt;R: Try&lt;Output = ()&gt;&gt;(<span class="kw">_</span>: (), <span class="kw">_</span>: ()) -&gt; R {
            R::from_output(())
        }

        <span class="self">self</span>.map_init(init, op).try_reduce(&lt;()&gt;::default, ok)
    }

    <span class="doccomment">/// Counts the number of items in this parallel iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let count = (0..100).into_par_iter().count();
    ///
    /// assert_eq!(count, 100);
    /// ```
    </span><span class="kw">fn </span>count(<span class="self">self</span>) -&gt; usize {
        <span class="kw">fn </span>one&lt;T&gt;(<span class="kw">_</span>: T) -&gt; usize {
            <span class="number">1
        </span>}

        <span class="self">self</span>.map(one).sum()
    }

    <span class="doccomment">/// Applies `map_op` to each item of this iterator, producing a new
    /// iterator with the results.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let mut par_iter = (0..5).into_par_iter().map(|x| x * 2);
    ///
    /// let doubles: Vec&lt;_&gt; = par_iter.collect();
    ///
    /// assert_eq!(&amp;doubles[..], &amp;[0, 2, 4, 6, 8]);
    /// ```
    </span><span class="kw">fn </span>map&lt;F, R&gt;(<span class="self">self</span>, map_op: F) -&gt; Map&lt;<span class="self">Self</span>, F&gt;
    <span class="kw">where
        </span>F: Fn(<span class="self">Self</span>::Item) -&gt; R + Sync + Send,
        R: Send,
    {
        Map::new(<span class="self">self</span>, map_op)
    }

    <span class="doccomment">/// Applies `map_op` to the given `init` value with each item of this
    /// iterator, producing a new iterator with the results.
    ///
    /// The `init` value will be cloned only as needed to be paired with
    /// the group of items in each rayon job.  It does not require the type
    /// to be `Sync`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::mpsc::channel;
    /// use rayon::prelude::*;
    ///
    /// let (sender, receiver) = channel();
    ///
    /// let a: Vec&lt;_&gt; = (0..5)
    ///                 .into_par_iter()            // iterating over i32
    ///                 .map_with(sender, |s, x| {
    ///                     s.send(x).unwrap();     // sending i32 values through the channel
    ///                     x                       // returning i32
    ///                 })
    ///                 .collect();                 // collecting the returned values into a vector
    ///
    /// let mut b: Vec&lt;_&gt; = receiver.iter()         // iterating over the values in the channel
    ///                             .collect();     // and collecting them
    /// b.sort();
    ///
    /// assert_eq!(a, b);
    /// ```
    </span><span class="kw">fn </span>map_with&lt;F, T, R&gt;(<span class="self">self</span>, init: T, map_op: F) -&gt; MapWith&lt;<span class="self">Self</span>, T, F&gt;
    <span class="kw">where
        </span>F: Fn(<span class="kw-2">&amp;mut </span>T, <span class="self">Self</span>::Item) -&gt; R + Sync + Send,
        T: Send + Clone,
        R: Send,
    {
        MapWith::new(<span class="self">self</span>, init, map_op)
    }

    <span class="doccomment">/// Applies `map_op` to a value returned by `init` with each item of this
    /// iterator, producing a new iterator with the results.
    ///
    /// The `init` function will be called only as needed for a value to be
    /// paired with the group of items in each rayon job.  There is no
    /// constraint on that returned type at all!
    ///
    /// # Examples
    ///
    /// ```
    /// use rand::Rng;
    /// use rayon::prelude::*;
    ///
    /// let a: Vec&lt;_&gt; = (1i32..1_000_000)
    ///     .into_par_iter()
    ///     .map_init(
    ///         || rand::thread_rng(),  // get the thread-local RNG
    ///         |rng, x| if rng.gen() { // randomly negate items
    ///             -x
    ///         } else {
    ///             x
    ///         },
    ///     ).collect();
    ///
    /// // There's a remote chance that this will fail...
    /// assert!(a.iter().any(|&amp;x| x &lt; 0));
    /// assert!(a.iter().any(|&amp;x| x &gt; 0));
    /// ```
    </span><span class="kw">fn </span>map_init&lt;F, INIT, T, R&gt;(<span class="self">self</span>, init: INIT, map_op: F) -&gt; MapInit&lt;<span class="self">Self</span>, INIT, F&gt;
    <span class="kw">where
        </span>F: Fn(<span class="kw-2">&amp;mut </span>T, <span class="self">Self</span>::Item) -&gt; R + Sync + Send,
        INIT: Fn() -&gt; T + Sync + Send,
        R: Send,
    {
        MapInit::new(<span class="self">self</span>, init, map_op)
    }

    <span class="doccomment">/// Creates an iterator which clones all of its elements.  This may be
    /// useful when you have an iterator over `&amp;T`, but you need `T`, and
    /// that type implements `Clone`. See also [`copied()`].
    ///
    /// [`copied()`]: #method.copied
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [1, 2, 3];
    ///
    /// let v_cloned: Vec&lt;_&gt; = a.par_iter().cloned().collect();
    ///
    /// // cloned is the same as .map(|&amp;x| x), for integers
    /// let v_map: Vec&lt;_&gt; = a.par_iter().map(|&amp;x| x).collect();
    ///
    /// assert_eq!(v_cloned, vec![1, 2, 3]);
    /// assert_eq!(v_map, vec![1, 2, 3]);
    /// ```
    </span><span class="kw">fn </span>cloned&lt;<span class="lifetime">'a</span>, T&gt;(<span class="self">self</span>) -&gt; Cloned&lt;<span class="self">Self</span>&gt;
    <span class="kw">where
        </span>T: <span class="lifetime">'a </span>+ Clone + Send,
        <span class="self">Self</span>: ParallelIterator&lt;Item = <span class="kw-2">&amp;</span><span class="lifetime">'a </span>T&gt;,
    {
        Cloned::new(<span class="self">self</span>)
    }

    <span class="doccomment">/// Creates an iterator which copies all of its elements.  This may be
    /// useful when you have an iterator over `&amp;T`, but you need `T`, and
    /// that type implements `Copy`. See also [`cloned()`].
    ///
    /// [`cloned()`]: #method.cloned
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [1, 2, 3];
    ///
    /// let v_copied: Vec&lt;_&gt; = a.par_iter().copied().collect();
    ///
    /// // copied is the same as .map(|&amp;x| x), for integers
    /// let v_map: Vec&lt;_&gt; = a.par_iter().map(|&amp;x| x).collect();
    ///
    /// assert_eq!(v_copied, vec![1, 2, 3]);
    /// assert_eq!(v_map, vec![1, 2, 3]);
    /// ```
    </span><span class="kw">fn </span>copied&lt;<span class="lifetime">'a</span>, T&gt;(<span class="self">self</span>) -&gt; Copied&lt;<span class="self">Self</span>&gt;
    <span class="kw">where
        </span>T: <span class="lifetime">'a </span>+ Copy + Send,
        <span class="self">Self</span>: ParallelIterator&lt;Item = <span class="kw-2">&amp;</span><span class="lifetime">'a </span>T&gt;,
    {
        Copied::new(<span class="self">self</span>)
    }

    <span class="doccomment">/// Applies `inspect_op` to a reference to each item of this iterator,
    /// producing a new iterator passing through the original items.  This is
    /// often useful for debugging to see what's happening in iterator stages.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [1, 4, 2, 3];
    ///
    /// // this iterator sequence is complex.
    /// let sum = a.par_iter()
    ///             .cloned()
    ///             .filter(|&amp;x| x % 2 == 0)
    ///             .reduce(|| 0, |sum, i| sum + i);
    ///
    /// println!("{}", sum);
    ///
    /// // let's add some inspect() calls to investigate what's happening
    /// let sum = a.par_iter()
    ///             .cloned()
    ///             .inspect(|x| println!("about to filter: {}", x))
    ///             .filter(|&amp;x| x % 2 == 0)
    ///             .inspect(|x| println!("made it through filter: {}", x))
    ///             .reduce(|| 0, |sum, i| sum + i);
    ///
    /// println!("{}", sum);
    /// ```
    </span><span class="kw">fn </span>inspect&lt;OP&gt;(<span class="self">self</span>, inspect_op: OP) -&gt; Inspect&lt;<span class="self">Self</span>, OP&gt;
    <span class="kw">where
        </span>OP: Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) + Sync + Send,
    {
        Inspect::new(<span class="self">self</span>, inspect_op)
    }

    <span class="doccomment">/// Mutates each item of this iterator before yielding it.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let par_iter = (0..5).into_par_iter().update(|x| {*x *= 2;});
    ///
    /// let doubles: Vec&lt;_&gt; = par_iter.collect();
    ///
    /// assert_eq!(&amp;doubles[..], &amp;[0, 2, 4, 6, 8]);
    /// ```
    </span><span class="kw">fn </span>update&lt;F&gt;(<span class="self">self</span>, update_op: F) -&gt; Update&lt;<span class="self">Self</span>, F&gt;
    <span class="kw">where
        </span>F: Fn(<span class="kw-2">&amp;mut </span><span class="self">Self</span>::Item) + Sync + Send,
    {
        Update::new(<span class="self">self</span>, update_op)
    }

    <span class="doccomment">/// Applies `filter_op` to each item of this iterator, producing a new
    /// iterator with only the items that gave `true` results.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let mut par_iter = (0..10).into_par_iter().filter(|x| x % 2 == 0);
    ///
    /// let even_numbers: Vec&lt;_&gt; = par_iter.collect();
    ///
    /// assert_eq!(&amp;even_numbers[..], &amp;[0, 2, 4, 6, 8]);
    /// ```
    </span><span class="kw">fn </span>filter&lt;P&gt;(<span class="self">self</span>, filter_op: P) -&gt; Filter&lt;<span class="self">Self</span>, P&gt;
    <span class="kw">where
        </span>P: Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        Filter::new(<span class="self">self</span>, filter_op)
    }

    <span class="doccomment">/// Applies `filter_op` to each item of this iterator to get an `Option`,
    /// producing a new iterator with only the items from `Some` results.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let mut par_iter = (0..10).into_par_iter()
    ///                         .filter_map(|x| {
    ///                             if x % 2 == 0 { Some(x * 3) }
    ///                             else { None }
    ///                         });
    ///
    /// let even_numbers: Vec&lt;_&gt; = par_iter.collect();
    ///
    /// assert_eq!(&amp;even_numbers[..], &amp;[0, 6, 12, 18, 24]);
    /// ```
    </span><span class="kw">fn </span>filter_map&lt;P, R&gt;(<span class="self">self</span>, filter_op: P) -&gt; FilterMap&lt;<span class="self">Self</span>, P&gt;
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; <span class="prelude-ty">Option</span>&lt;R&gt; + Sync + Send,
        R: Send,
    {
        FilterMap::new(<span class="self">self</span>, filter_op)
    }

    <span class="doccomment">/// Applies `map_op` to each item of this iterator to get nested parallel iterators,
    /// producing a new parallel iterator that flattens these back into one.
    ///
    /// See also [`flat_map_iter`](#method.flat_map_iter).
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [[1, 2], [3, 4], [5, 6], [7, 8]];
    ///
    /// let par_iter = a.par_iter().cloned().flat_map(|a| a.to_vec());
    ///
    /// let vec: Vec&lt;_&gt; = par_iter.collect();
    ///
    /// assert_eq!(&amp;vec[..], &amp;[1, 2, 3, 4, 5, 6, 7, 8]);
    /// ```
    </span><span class="kw">fn </span>flat_map&lt;F, PI&gt;(<span class="self">self</span>, map_op: F) -&gt; FlatMap&lt;<span class="self">Self</span>, F&gt;
    <span class="kw">where
        </span>F: Fn(<span class="self">Self</span>::Item) -&gt; PI + Sync + Send,
        PI: IntoParallelIterator,
    {
        FlatMap::new(<span class="self">self</span>, map_op)
    }

    <span class="doccomment">/// Applies `map_op` to each item of this iterator to get nested serial iterators,
    /// producing a new parallel iterator that flattens these back into one.
    ///
    /// # `flat_map_iter` versus `flat_map`
    ///
    /// These two methods are similar but behave slightly differently. With [`flat_map`],
    /// each of the nested iterators must be a parallel iterator, and they will be further
    /// split up with nested parallelism. With `flat_map_iter`, each nested iterator is a
    /// sequential `Iterator`, and we only parallelize _between_ them, while the items
    /// produced by each nested iterator are processed sequentially.
    ///
    /// When choosing between these methods, consider whether nested parallelism suits the
    /// potential iterators at hand. If there's little computation involved, or its length
    /// is much less than the outer parallel iterator, then it may perform better to avoid
    /// the overhead of parallelism, just flattening sequentially with `flat_map_iter`.
    /// If there is a lot of computation, potentially outweighing the outer parallel
    /// iterator, then the nested parallelism of `flat_map` may be worthwhile.
    ///
    /// [`flat_map`]: #method.flat_map
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use std::cell::RefCell;
    ///
    /// let a = [[1, 2], [3, 4], [5, 6], [7, 8]];
    ///
    /// let par_iter = a.par_iter().flat_map_iter(|a| {
    ///     // The serial iterator doesn't have to be thread-safe, just its items.
    ///     let cell_iter = RefCell::new(a.iter().cloned());
    ///     std::iter::from_fn(move || cell_iter.borrow_mut().next())
    /// });
    ///
    /// let vec: Vec&lt;_&gt; = par_iter.collect();
    ///
    /// assert_eq!(&amp;vec[..], &amp;[1, 2, 3, 4, 5, 6, 7, 8]);
    /// ```
    </span><span class="kw">fn </span>flat_map_iter&lt;F, SI&gt;(<span class="self">self</span>, map_op: F) -&gt; FlatMapIter&lt;<span class="self">Self</span>, F&gt;
    <span class="kw">where
        </span>F: Fn(<span class="self">Self</span>::Item) -&gt; SI + Sync + Send,
        SI: IntoIterator,
        SI::Item: Send,
    {
        FlatMapIter::new(<span class="self">self</span>, map_op)
    }

    <span class="doccomment">/// An adaptor that flattens parallel-iterable `Item`s into one large iterator.
    ///
    /// See also [`flatten_iter`](#method.flatten_iter).
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let x: Vec&lt;Vec&lt;_&gt;&gt; = vec![vec![1, 2], vec![3, 4]];
    /// let y: Vec&lt;_&gt; = x.into_par_iter().flatten().collect();
    ///
    /// assert_eq!(y, vec![1, 2, 3, 4]);
    /// ```
    </span><span class="kw">fn </span>flatten(<span class="self">self</span>) -&gt; Flatten&lt;<span class="self">Self</span>&gt;
    <span class="kw">where
        </span><span class="self">Self</span>::Item: IntoParallelIterator,
    {
        Flatten::new(<span class="self">self</span>)
    }

    <span class="doccomment">/// An adaptor that flattens serial-iterable `Item`s into one large iterator.
    ///
    /// See also [`flatten`](#method.flatten) and the analogous comparison of
    /// [`flat_map_iter` versus `flat_map`](#flat_map_iter-versus-flat_map).
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let x: Vec&lt;Vec&lt;_&gt;&gt; = vec![vec![1, 2], vec![3, 4]];
    /// let iters: Vec&lt;_&gt; = x.into_iter().map(Vec::into_iter).collect();
    /// let y: Vec&lt;_&gt; = iters.into_par_iter().flatten_iter().collect();
    ///
    /// assert_eq!(y, vec![1, 2, 3, 4]);
    /// ```
    </span><span class="kw">fn </span>flatten_iter(<span class="self">self</span>) -&gt; FlattenIter&lt;<span class="self">Self</span>&gt;
    <span class="kw">where
        </span><span class="self">Self</span>::Item: IntoIterator,
        &lt;<span class="self">Self</span>::Item <span class="kw">as </span>IntoIterator&gt;::Item: Send,
    {
        FlattenIter::new(<span class="self">self</span>)
    }

    <span class="doccomment">/// Reduces the items in the iterator into one item using `op`.
    /// The argument `identity` should be a closure that can produce
    /// "identity" value which may be inserted into the sequence as
    /// needed to create opportunities for parallel execution. So, for
    /// example, if you are doing a summation, then `identity()` ought
    /// to produce something that represents the zero for your type
    /// (but consider just calling `sum()` in that case).
    ///
    /// # Examples
    ///
    /// ```
    /// // Iterate over a sequence of pairs `(x0, y0), ..., (xN, yN)`
    /// // and use reduce to compute one pair `(x0 + ... + xN, y0 + ... + yN)`
    /// // where the first/second elements are summed separately.
    /// use rayon::prelude::*;
    /// let sums = [(0, 1), (5, 6), (16, 2), (8, 9)]
    ///            .par_iter()        // iterating over &amp;(i32, i32)
    ///            .cloned()          // iterating over (i32, i32)
    ///            .reduce(|| (0, 0), // the "identity" is 0 in both columns
    ///                    |a, b| (a.0 + b.0, a.1 + b.1));
    /// assert_eq!(sums, (0 + 5 + 16 + 8, 1 + 6 + 2 + 9));
    /// ```
    ///
    /// **Note:** unlike a sequential `fold` operation, the order in
    /// which `op` will be applied to reduce the result is not fully
    /// specified. So `op` should be [associative] or else the results
    /// will be non-deterministic. And of course `identity()` should
    /// produce a true identity.
    ///
    /// [associative]: https://en.wikipedia.org/wiki/Associative_property
    </span><span class="kw">fn </span>reduce&lt;OP, ID&gt;(<span class="self">self</span>, identity: ID, op: OP) -&gt; <span class="self">Self</span>::Item
    <span class="kw">where
        </span>OP: Fn(<span class="self">Self</span>::Item, <span class="self">Self</span>::Item) -&gt; <span class="self">Self</span>::Item + Sync + Send,
        ID: Fn() -&gt; <span class="self">Self</span>::Item + Sync + Send,
    {
        reduce::reduce(<span class="self">self</span>, identity, op)
    }

    <span class="doccomment">/// Reduces the items in the iterator into one item using `op`.
    /// If the iterator is empty, `None` is returned; otherwise,
    /// `Some` is returned.
    ///
    /// This version of `reduce` is simple but somewhat less
    /// efficient. If possible, it is better to call `reduce()`, which
    /// requires an identity element.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let sums = [(0, 1), (5, 6), (16, 2), (8, 9)]
    ///            .par_iter()        // iterating over &amp;(i32, i32)
    ///            .cloned()          // iterating over (i32, i32)
    ///            .reduce_with(|a, b| (a.0 + b.0, a.1 + b.1))
    ///            .unwrap();
    /// assert_eq!(sums, (0 + 5 + 16 + 8, 1 + 6 + 2 + 9));
    /// ```
    ///
    /// **Note:** unlike a sequential `fold` operation, the order in
    /// which `op` will be applied to reduce the result is not fully
    /// specified. So `op` should be [associative] or else the results
    /// will be non-deterministic.
    ///
    /// [associative]: https://en.wikipedia.org/wiki/Associative_property
    </span><span class="kw">fn </span>reduce_with&lt;OP&gt;(<span class="self">self</span>, op: OP) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span>OP: Fn(<span class="self">Self</span>::Item, <span class="self">Self</span>::Item) -&gt; <span class="self">Self</span>::Item + Sync + Send,
    {
        <span class="kw">fn </span>opt_fold&lt;T&gt;(op: <span class="kw">impl </span>Fn(T, T) -&gt; T) -&gt; <span class="kw">impl </span>Fn(<span class="prelude-ty">Option</span>&lt;T&gt;, T) -&gt; <span class="prelude-ty">Option</span>&lt;T&gt; {
            <span class="kw">move </span>|opt_a, b| <span class="kw">match </span>opt_a {
                <span class="prelude-val">Some</span>(a) =&gt; <span class="prelude-val">Some</span>(op(a, b)),
                <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Some</span>(b),
            }
        }

        <span class="kw">fn </span>opt_reduce&lt;T&gt;(op: <span class="kw">impl </span>Fn(T, T) -&gt; T) -&gt; <span class="kw">impl </span>Fn(<span class="prelude-ty">Option</span>&lt;T&gt;, <span class="prelude-ty">Option</span>&lt;T&gt;) -&gt; <span class="prelude-ty">Option</span>&lt;T&gt; {
            <span class="kw">move </span>|opt_a, opt_b| <span class="kw">match </span>(opt_a, opt_b) {
                (<span class="prelude-val">Some</span>(a), <span class="prelude-val">Some</span>(b)) =&gt; <span class="prelude-val">Some</span>(op(a, b)),
                (<span class="prelude-val">Some</span>(v), <span class="prelude-val">None</span>) | (<span class="prelude-val">None</span>, <span class="prelude-val">Some</span>(v)) =&gt; <span class="prelude-val">Some</span>(v),
                (<span class="prelude-val">None</span>, <span class="prelude-val">None</span>) =&gt; <span class="prelude-val">None</span>,
            }
        }

        <span class="self">self</span>.fold(&lt;<span class="kw">_</span>&gt;::default, opt_fold(<span class="kw-2">&amp;</span>op))
            .reduce(&lt;<span class="kw">_</span>&gt;::default, opt_reduce(<span class="kw-2">&amp;</span>op))
    }

    <span class="doccomment">/// Reduces the items in the iterator into one item using a fallible `op`.
    /// The `identity` argument is used the same way as in [`reduce()`].
    ///
    /// [`reduce()`]: #method.reduce
    ///
    /// If a `Result::Err` or `Option::None` item is found, or if `op` reduces
    /// to one, we will attempt to stop processing the rest of the items in the
    /// iterator as soon as possible, and we will return that terminating value.
    /// Otherwise, we will return the final reduced `Result::Ok(T)` or
    /// `Option::Some(T)`.  If there are multiple errors in parallel, it is not
    /// specified which will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// // Compute the sum of squares, being careful about overflow.
    /// fn sum_squares&lt;I: IntoParallelIterator&lt;Item = i32&gt;&gt;(iter: I) -&gt; Option&lt;i32&gt; {
    ///     iter.into_par_iter()
    ///         .map(|i| i.checked_mul(i))            // square each item,
    ///         .try_reduce(|| 0, i32::checked_add)   // and add them up!
    /// }
    /// assert_eq!(sum_squares(0..5), Some(0 + 1 + 4 + 9 + 16));
    ///
    /// // The sum might overflow
    /// assert_eq!(sum_squares(0..10_000), None);
    ///
    /// // Or the squares might overflow before it even reaches `try_reduce`
    /// assert_eq!(sum_squares(1_000_000..1_000_001), None);
    /// ```
    </span><span class="kw">fn </span>try_reduce&lt;T, OP, ID&gt;(<span class="self">self</span>, identity: ID, op: OP) -&gt; <span class="self">Self</span>::Item
    <span class="kw">where
        </span>OP: Fn(T, T) -&gt; <span class="self">Self</span>::Item + Sync + Send,
        ID: Fn() -&gt; T + Sync + Send,
        <span class="self">Self</span>::Item: Try&lt;Output = T&gt;,
    {
        try_reduce::try_reduce(<span class="self">self</span>, identity, op)
    }

    <span class="doccomment">/// Reduces the items in the iterator into one item using a fallible `op`.
    ///
    /// Like [`reduce_with()`], if the iterator is empty, `None` is returned;
    /// otherwise, `Some` is returned.  Beyond that, it behaves like
    /// [`try_reduce()`] for handling `Err`/`None`.
    ///
    /// [`reduce_with()`]: #method.reduce_with
    /// [`try_reduce()`]: #method.try_reduce
    ///
    /// For instance, with `Option` items, the return value may be:
    /// - `None`, the iterator was empty
    /// - `Some(None)`, we stopped after encountering `None`.
    /// - `Some(Some(x))`, the entire iterator reduced to `x`.
    ///
    /// With `Result` items, the nesting is more obvious:
    /// - `None`, the iterator was empty
    /// - `Some(Err(e))`, we stopped after encountering an error `e`.
    /// - `Some(Ok(x))`, the entire iterator reduced to `x`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let files = ["/dev/null", "/does/not/exist"];
    ///
    /// // Find the biggest file
    /// files.into_par_iter()
    ///     .map(|path| std::fs::metadata(path).map(|m| (path, m.len())))
    ///     .try_reduce_with(|a, b| {
    ///         Ok(if a.1 &gt;= b.1 { a } else { b })
    ///     })
    ///     .expect("Some value, since the iterator is not empty")
    ///     .expect_err("not found");
    /// ```
    </span><span class="kw">fn </span>try_reduce_with&lt;T, OP&gt;(<span class="self">self</span>, op: OP) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span>OP: Fn(T, T) -&gt; <span class="self">Self</span>::Item + Sync + Send,
        <span class="self">Self</span>::Item: Try&lt;Output = T&gt;,
    {
        try_reduce_with::try_reduce_with(<span class="self">self</span>, op)
    }

    <span class="doccomment">/// Parallel fold is similar to sequential fold except that the
    /// sequence of items may be subdivided before it is
    /// folded. Consider a list of numbers like `22 3 77 89 46`. If
    /// you used sequential fold to add them (`fold(0, |a,b| a+b)`,
    /// you would wind up first adding 0 + 22, then 22 + 3, then 25 +
    /// 77, and so forth. The **parallel fold** works similarly except
    /// that it first breaks up your list into sublists, and hence
    /// instead of yielding up a single sum at the end, it yields up
    /// multiple sums. The number of results is nondeterministic, as
    /// is the point where the breaks occur.
    ///
    /// So if we did the same parallel fold (`fold(0, |a,b| a+b)`) on
    /// our example list, we might wind up with a sequence of two numbers,
    /// like so:
    ///
    /// ```notrust
    /// 22 3 77 89 46
    ///       |     |
    ///     102   135
    /// ```
    ///
    /// Or perhaps these three numbers:
    ///
    /// ```notrust
    /// 22 3 77 89 46
    ///       |  |  |
    ///     102 89 46
    /// ```
    ///
    /// In general, Rayon will attempt to find good breaking points
    /// that keep all of your cores busy.
    ///
    /// ### Fold versus reduce
    ///
    /// The `fold()` and `reduce()` methods each take an identity element
    /// and a combining function, but they operate rather differently.
    ///
    /// `reduce()` requires that the identity function has the same
    /// type as the things you are iterating over, and it fully
    /// reduces the list of items into a single item. So, for example,
    /// imagine we are iterating over a list of bytes `bytes: [128_u8,
    /// 64_u8, 64_u8]`. If we used `bytes.reduce(|| 0_u8, |a: u8, b:
    /// u8| a + b)`, we would get an overflow. This is because `0`,
    /// `a`, and `b` here are all bytes, just like the numbers in the
    /// list (I wrote the types explicitly above, but those are the
    /// only types you can use). To avoid the overflow, we would need
    /// to do something like `bytes.map(|b| b as u32).reduce(|| 0, |a,
    /// b| a + b)`, in which case our result would be `256`.
    ///
    /// In contrast, with `fold()`, the identity function does not
    /// have to have the same type as the things you are iterating
    /// over, and you potentially get back many results. So, if we
    /// continue with the `bytes` example from the previous paragraph,
    /// we could do `bytes.fold(|| 0_u32, |a, b| a + (b as u32))` to
    /// convert our bytes into `u32`. And of course we might not get
    /// back a single sum.
    ///
    /// There is a more subtle distinction as well, though it's
    /// actually implied by the above points. When you use `reduce()`,
    /// your reduction function is sometimes called with values that
    /// were never part of your original parallel iterator (for
    /// example, both the left and right might be a partial sum). With
    /// `fold()`, in contrast, the left value in the fold function is
    /// always the accumulator, and the right value is always from
    /// your original sequence.
    ///
    /// ### Fold vs Map/Reduce
    ///
    /// Fold makes sense if you have some operation where it is
    /// cheaper to create groups of elements at a time. For example,
    /// imagine collecting characters into a string. If you were going
    /// to use map/reduce, you might try this:
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let s =
    ///     ['a', 'b', 'c', 'd', 'e']
    ///     .par_iter()
    ///     .map(|c: &amp;char| format!("{}", c))
    ///     .reduce(|| String::new(),
    ///             |mut a: String, b: String| { a.push_str(&amp;b); a });
    ///
    /// assert_eq!(s, "abcde");
    /// ```
    ///
    /// Because reduce produces the same type of element as its input,
    /// you have to first map each character into a string, and then
    /// you can reduce them. This means we create one string per
    /// element in our iterator -- not so great. Using `fold`, we can
    /// do this instead:
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let s =
    ///     ['a', 'b', 'c', 'd', 'e']
    ///     .par_iter()
    ///     .fold(|| String::new(),
    ///             |mut s: String, c: &amp;char| { s.push(*c); s })
    ///     .reduce(|| String::new(),
    ///             |mut a: String, b: String| { a.push_str(&amp;b); a });
    ///
    /// assert_eq!(s, "abcde");
    /// ```
    ///
    /// Now `fold` will process groups of our characters at a time,
    /// and we only make one string per group. We should wind up with
    /// some small-ish number of strings roughly proportional to the
    /// number of CPUs you have (it will ultimately depend on how busy
    /// your processors are). Note that we still need to do a reduce
    /// afterwards to combine those groups of strings into a single
    /// string.
    ///
    /// You could use a similar trick to save partial results (e.g., a
    /// cache) or something similar.
    ///
    /// ### Combining fold with other operations
    ///
    /// You can combine `fold` with `reduce` if you want to produce a
    /// single value. This is then roughly equivalent to a map/reduce
    /// combination in effect:
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let bytes = 0..22_u8;
    /// let sum = bytes.into_par_iter()
    ///                .fold(|| 0_u32, |a: u32, b: u8| a + (b as u32))
    ///                .sum::&lt;u32&gt;();
    ///
    /// assert_eq!(sum, (0..22).sum()); // compare to sequential
    /// ```
    </span><span class="kw">fn </span>fold&lt;T, ID, F&gt;(<span class="self">self</span>, identity: ID, fold_op: F) -&gt; Fold&lt;<span class="self">Self</span>, ID, F&gt;
    <span class="kw">where
        </span>F: Fn(T, <span class="self">Self</span>::Item) -&gt; T + Sync + Send,
        ID: Fn() -&gt; T + Sync + Send,
        T: Send,
    {
        Fold::new(<span class="self">self</span>, identity, fold_op)
    }

    <span class="doccomment">/// Applies `fold_op` to the given `init` value with each item of this
    /// iterator, finally producing the value for further use.
    ///
    /// This works essentially like `fold(|| init.clone(), fold_op)`, except
    /// it doesn't require the `init` type to be `Sync`, nor any other form
    /// of added synchronization.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let bytes = 0..22_u8;
    /// let sum = bytes.into_par_iter()
    ///                .fold_with(0_u32, |a: u32, b: u8| a + (b as u32))
    ///                .sum::&lt;u32&gt;();
    ///
    /// assert_eq!(sum, (0..22).sum()); // compare to sequential
    /// ```
    </span><span class="kw">fn </span>fold_with&lt;F, T&gt;(<span class="self">self</span>, init: T, fold_op: F) -&gt; FoldWith&lt;<span class="self">Self</span>, T, F&gt;
    <span class="kw">where
        </span>F: Fn(T, <span class="self">Self</span>::Item) -&gt; T + Sync + Send,
        T: Send + Clone,
    {
        FoldWith::new(<span class="self">self</span>, init, fold_op)
    }

    <span class="doccomment">/// Performs a fallible parallel fold.
    ///
    /// This is a variation of [`fold()`] for operations which can fail with
    /// `Option::None` or `Result::Err`.  The first such failure stops
    /// processing the local set of items, without affecting other folds in the
    /// iterator's subdivisions.
    ///
    /// Often, `try_fold()` will be followed by [`try_reduce()`]
    /// for a final reduction and global short-circuiting effect.
    ///
    /// [`fold()`]: #method.fold
    /// [`try_reduce()`]: #method.try_reduce
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let bytes = 0..22_u8;
    /// let sum = bytes.into_par_iter()
    ///                .try_fold(|| 0_u32, |a: u32, b: u8| a.checked_add(b as u32))
    ///                .try_reduce(|| 0, u32::checked_add);
    ///
    /// assert_eq!(sum, Some((0..22).sum())); // compare to sequential
    /// ```
    </span><span class="kw">fn </span>try_fold&lt;T, R, ID, F&gt;(<span class="self">self</span>, identity: ID, fold_op: F) -&gt; TryFold&lt;<span class="self">Self</span>, R, ID, F&gt;
    <span class="kw">where
        </span>F: Fn(T, <span class="self">Self</span>::Item) -&gt; R + Sync + Send,
        ID: Fn() -&gt; T + Sync + Send,
        R: Try&lt;Output = T&gt; + Send,
    {
        TryFold::new(<span class="self">self</span>, identity, fold_op)
    }

    <span class="doccomment">/// Performs a fallible parallel fold with a cloneable `init` value.
    ///
    /// This combines the `init` semantics of [`fold_with()`] and the failure
    /// semantics of [`try_fold()`].
    ///
    /// [`fold_with()`]: #method.fold_with
    /// [`try_fold()`]: #method.try_fold
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let bytes = 0..22_u8;
    /// let sum = bytes.into_par_iter()
    ///                .try_fold_with(0_u32, |a: u32, b: u8| a.checked_add(b as u32))
    ///                .try_reduce(|| 0, u32::checked_add);
    ///
    /// assert_eq!(sum, Some((0..22).sum())); // compare to sequential
    /// ```
    </span><span class="kw">fn </span>try_fold_with&lt;F, T, R&gt;(<span class="self">self</span>, init: T, fold_op: F) -&gt; TryFoldWith&lt;<span class="self">Self</span>, R, F&gt;
    <span class="kw">where
        </span>F: Fn(T, <span class="self">Self</span>::Item) -&gt; R + Sync + Send,
        R: Try&lt;Output = T&gt; + Send,
        T: Clone + Send,
    {
        TryFoldWith::new(<span class="self">self</span>, init, fold_op)
    }

    <span class="doccomment">/// Sums up the items in the iterator.
    ///
    /// Note that the order in items will be reduced is not specified,
    /// so if the `+` operator is not truly [associative] \(as is the
    /// case for floating point numbers), then the results are not
    /// fully deterministic.
    ///
    /// [associative]: https://en.wikipedia.org/wiki/Associative_property
    ///
    /// Basically equivalent to `self.reduce(|| 0, |a, b| a + b)`,
    /// except that the type of `0` and the `+` operation may vary
    /// depending on the type of value being produced.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [1, 5, 7];
    ///
    /// let sum: i32 = a.par_iter().sum();
    ///
    /// assert_eq!(sum, 13);
    /// ```
    </span><span class="kw">fn </span>sum&lt;S&gt;(<span class="self">self</span>) -&gt; S
    <span class="kw">where
        </span>S: Send + Sum&lt;<span class="self">Self</span>::Item&gt; + Sum&lt;S&gt;,
    {
        sum::sum(<span class="self">self</span>)
    }

    <span class="doccomment">/// Multiplies all the items in the iterator.
    ///
    /// Note that the order in items will be reduced is not specified,
    /// so if the `*` operator is not truly [associative] \(as is the
    /// case for floating point numbers), then the results are not
    /// fully deterministic.
    ///
    /// [associative]: https://en.wikipedia.org/wiki/Associative_property
    ///
    /// Basically equivalent to `self.reduce(|| 1, |a, b| a * b)`,
    /// except that the type of `1` and the `*` operation may vary
    /// depending on the type of value being produced.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// fn factorial(n: u32) -&gt; u32 {
    ///    (1..n+1).into_par_iter().product()
    /// }
    ///
    /// assert_eq!(factorial(0), 1);
    /// assert_eq!(factorial(1), 1);
    /// assert_eq!(factorial(5), 120);
    /// ```
    </span><span class="kw">fn </span>product&lt;P&gt;(<span class="self">self</span>) -&gt; P
    <span class="kw">where
        </span>P: Send + Product&lt;<span class="self">Self</span>::Item&gt; + Product&lt;P&gt;,
    {
        product::product(<span class="self">self</span>)
    }

    <span class="doccomment">/// Computes the minimum of all the items in the iterator. If the
    /// iterator is empty, `None` is returned; otherwise, `Some(min)`
    /// is returned.
    ///
    /// Note that the order in which the items will be reduced is not
    /// specified, so if the `Ord` impl is not truly associative, then
    /// the results are not deterministic.
    ///
    /// Basically equivalent to `self.reduce_with(|a, b| Ord::min(a, b))`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [45, 74, 32];
    ///
    /// assert_eq!(a.par_iter().min(), Some(&amp;32));
    ///
    /// let b: [i32; 0] = [];
    ///
    /// assert_eq!(b.par_iter().min(), None);
    /// ```
    </span><span class="kw">fn </span>min(<span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span><span class="self">Self</span>::Item: Ord,
    {
        <span class="self">self</span>.reduce_with(Ord::min)
    }

    <span class="doccomment">/// Computes the minimum of all the items in the iterator with respect to
    /// the given comparison function. If the iterator is empty, `None` is
    /// returned; otherwise, `Some(min)` is returned.
    ///
    /// Note that the order in which the items will be reduced is not
    /// specified, so if the comparison function is not associative, then
    /// the results are not deterministic.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [-3_i32, 77, 53, 240, -1];
    ///
    /// assert_eq!(a.par_iter().min_by(|x, y| x.cmp(y)), Some(&amp;-3));
    /// ```
    </span><span class="kw">fn </span>min_by&lt;F&gt;(<span class="self">self</span>, f: F) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span>F: Sync + Send + Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item, <span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; Ordering,
    {
        <span class="kw">fn </span>min&lt;T&gt;(f: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; Ordering) -&gt; <span class="kw">impl </span>Fn(T, T) -&gt; T {
            <span class="kw">move </span>|a, b| <span class="kw">match </span>f(<span class="kw-2">&amp;</span>a, <span class="kw-2">&amp;</span>b) {
                Ordering::Greater =&gt; b,
                <span class="kw">_ </span>=&gt; a,
            }
        }

        <span class="self">self</span>.reduce_with(min(f))
    }

    <span class="doccomment">/// Computes the item that yields the minimum value for the given
    /// function. If the iterator is empty, `None` is returned;
    /// otherwise, `Some(item)` is returned.
    ///
    /// Note that the order in which the items will be reduced is not
    /// specified, so if the `Ord` impl is not truly associative, then
    /// the results are not deterministic.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [-3_i32, 34, 2, 5, -10, -3, -23];
    ///
    /// assert_eq!(a.par_iter().min_by_key(|x| x.abs()), Some(&amp;2));
    /// ```
    </span><span class="kw">fn </span>min_by_key&lt;K, F&gt;(<span class="self">self</span>, f: F) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span>K: Ord + Send,
        F: Sync + Send + Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; K,
    {
        <span class="kw">fn </span>key&lt;T, K&gt;(f: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; K) -&gt; <span class="kw">impl </span>Fn(T) -&gt; (K, T) {
            <span class="kw">move </span>|x| (f(<span class="kw-2">&amp;</span>x), x)
        }

        <span class="kw">fn </span>min_key&lt;T, K: Ord&gt;(a: (K, T), b: (K, T)) -&gt; (K, T) {
            <span class="kw">match </span>(a.<span class="number">0</span>).cmp(<span class="kw-2">&amp;</span>b.<span class="number">0</span>) {
                Ordering::Greater =&gt; b,
                <span class="kw">_ </span>=&gt; a,
            }
        }

        <span class="kw">let </span>(<span class="kw">_</span>, x) = <span class="self">self</span>.map(key(f)).reduce_with(min_key)<span class="question-mark">?</span>;
        <span class="prelude-val">Some</span>(x)
    }

    <span class="doccomment">/// Computes the maximum of all the items in the iterator. If the
    /// iterator is empty, `None` is returned; otherwise, `Some(max)`
    /// is returned.
    ///
    /// Note that the order in which the items will be reduced is not
    /// specified, so if the `Ord` impl is not truly associative, then
    /// the results are not deterministic.
    ///
    /// Basically equivalent to `self.reduce_with(|a, b| Ord::max(a, b))`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [45, 74, 32];
    ///
    /// assert_eq!(a.par_iter().max(), Some(&amp;74));
    ///
    /// let b: [i32; 0] = [];
    ///
    /// assert_eq!(b.par_iter().max(), None);
    /// ```
    </span><span class="kw">fn </span>max(<span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span><span class="self">Self</span>::Item: Ord,
    {
        <span class="self">self</span>.reduce_with(Ord::max)
    }

    <span class="doccomment">/// Computes the maximum of all the items in the iterator with respect to
    /// the given comparison function. If the iterator is empty, `None` is
    /// returned; otherwise, `Some(max)` is returned.
    ///
    /// Note that the order in which the items will be reduced is not
    /// specified, so if the comparison function is not associative, then
    /// the results are not deterministic.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [-3_i32, 77, 53, 240, -1];
    ///
    /// assert_eq!(a.par_iter().max_by(|x, y| x.abs().cmp(&amp;y.abs())), Some(&amp;240));
    /// ```
    </span><span class="kw">fn </span>max_by&lt;F&gt;(<span class="self">self</span>, f: F) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span>F: Sync + Send + Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item, <span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; Ordering,
    {
        <span class="kw">fn </span>max&lt;T&gt;(f: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; Ordering) -&gt; <span class="kw">impl </span>Fn(T, T) -&gt; T {
            <span class="kw">move </span>|a, b| <span class="kw">match </span>f(<span class="kw-2">&amp;</span>a, <span class="kw-2">&amp;</span>b) {
                Ordering::Greater =&gt; a,
                <span class="kw">_ </span>=&gt; b,
            }
        }

        <span class="self">self</span>.reduce_with(max(f))
    }

    <span class="doccomment">/// Computes the item that yields the maximum value for the given
    /// function. If the iterator is empty, `None` is returned;
    /// otherwise, `Some(item)` is returned.
    ///
    /// Note that the order in which the items will be reduced is not
    /// specified, so if the `Ord` impl is not truly associative, then
    /// the results are not deterministic.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [-3_i32, 34, 2, 5, -10, -3, -23];
    ///
    /// assert_eq!(a.par_iter().max_by_key(|x| x.abs()), Some(&amp;34));
    /// ```
    </span><span class="kw">fn </span>max_by_key&lt;K, F&gt;(<span class="self">self</span>, f: F) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span>K: Ord + Send,
        F: Sync + Send + Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; K,
    {
        <span class="kw">fn </span>key&lt;T, K&gt;(f: <span class="kw">impl </span>Fn(<span class="kw-2">&amp;</span>T) -&gt; K) -&gt; <span class="kw">impl </span>Fn(T) -&gt; (K, T) {
            <span class="kw">move </span>|x| (f(<span class="kw-2">&amp;</span>x), x)
        }

        <span class="kw">fn </span>max_key&lt;T, K: Ord&gt;(a: (K, T), b: (K, T)) -&gt; (K, T) {
            <span class="kw">match </span>(a.<span class="number">0</span>).cmp(<span class="kw-2">&amp;</span>b.<span class="number">0</span>) {
                Ordering::Greater =&gt; a,
                <span class="kw">_ </span>=&gt; b,
            }
        }

        <span class="kw">let </span>(<span class="kw">_</span>, x) = <span class="self">self</span>.map(key(f)).reduce_with(max_key)<span class="question-mark">?</span>;
        <span class="prelude-val">Some</span>(x)
    }

    <span class="doccomment">/// Takes two iterators and creates a new iterator over both.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [0, 1, 2];
    /// let b = [9, 8, 7];
    ///
    /// let par_iter = a.par_iter().chain(b.par_iter());
    ///
    /// let chained: Vec&lt;_&gt; = par_iter.cloned().collect();
    ///
    /// assert_eq!(&amp;chained[..], &amp;[0, 1, 2, 9, 8, 7]);
    /// ```
    </span><span class="kw">fn </span>chain&lt;C&gt;(<span class="self">self</span>, chain: C) -&gt; Chain&lt;<span class="self">Self</span>, C::Iter&gt;
    <span class="kw">where
        </span>C: IntoParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;,
    {
        Chain::new(<span class="self">self</span>, chain.into_par_iter())
    }

    <span class="doccomment">/// Searches for **some** item in the parallel iterator that
    /// matches the given predicate and returns it. This operation
    /// is similar to [`find` on sequential iterators][find] but
    /// the item returned may not be the **first** one in the parallel
    /// sequence which matches, since we search the entire sequence in parallel.
    ///
    /// Once a match is found, we will attempt to stop processing
    /// the rest of the items in the iterator as soon as possible
    /// (just as `find` stops iterating once a match is found).
    ///
    /// [find]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [1, 2, 3, 3];
    ///
    /// assert_eq!(a.par_iter().find_any(|&amp;&amp;x| x == 3), Some(&amp;3));
    ///
    /// assert_eq!(a.par_iter().find_any(|&amp;&amp;x| x == 100), None);
    /// ```
    </span><span class="kw">fn </span>find_any&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span>P: Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        find::find(<span class="self">self</span>, predicate)
    }

    <span class="doccomment">/// Searches for the sequentially **first** item in the parallel iterator
    /// that matches the given predicate and returns it.
    ///
    /// Once a match is found, all attempts to the right of the match
    /// will be stopped, while attempts to the left must continue in case
    /// an earlier match is found.
    ///
    /// For added performance, you might consider using `find_first` in conjunction with
    /// [`by_exponential_blocks()`][IndexedParallelIterator::by_exponential_blocks].
    ///
    /// Note that not all parallel iterators have a useful order, much like
    /// sequential `HashMap` iteration, so "first" may be nebulous.  If you
    /// just want the first match that discovered anywhere in the iterator,
    /// `find_any` is a better choice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [1, 2, 3, 3];
    ///
    /// assert_eq!(a.par_iter().find_first(|&amp;&amp;x| x == 3), Some(&amp;3));
    ///
    /// assert_eq!(a.par_iter().find_first(|&amp;&amp;x| x == 100), None);
    /// ```
    </span><span class="kw">fn </span>find_first&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span>P: Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        find_first_last::find_first(<span class="self">self</span>, predicate)
    }

    <span class="doccomment">/// Searches for the sequentially **last** item in the parallel iterator
    /// that matches the given predicate and returns it.
    ///
    /// Once a match is found, all attempts to the left of the match
    /// will be stopped, while attempts to the right must continue in case
    /// a later match is found.
    ///
    /// Note that not all parallel iterators have a useful order, much like
    /// sequential `HashMap` iteration, so "last" may be nebulous.  When the
    /// order doesn't actually matter to you, `find_any` is a better choice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [1, 2, 3, 3];
    ///
    /// assert_eq!(a.par_iter().find_last(|&amp;&amp;x| x == 3), Some(&amp;3));
    ///
    /// assert_eq!(a.par_iter().find_last(|&amp;&amp;x| x == 100), None);
    /// ```
    </span><span class="kw">fn </span>find_last&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span>P: Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        find_first_last::find_last(<span class="self">self</span>, predicate)
    }

    <span class="doccomment">/// Applies the given predicate to the items in the parallel iterator
    /// and returns **any** non-None result of the map operation.
    ///
    /// Once a non-None value is produced from the map operation, we will
    /// attempt to stop processing the rest of the items in the iterator
    /// as soon as possible.
    ///
    /// Note that this method only returns **some** item in the parallel
    /// iterator that is not None from the map predicate. The item returned
    /// may not be the **first** non-None value produced in the parallel
    /// sequence, since the entire sequence is mapped over in parallel.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let c = ["lol", "NaN", "5", "5"];
    ///
    /// let found_number = c.par_iter().find_map_any(|s| s.parse().ok());
    ///
    /// assert_eq!(found_number, Some(5));
    /// ```
    </span><span class="kw">fn </span>find_map_any&lt;P, R&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;R&gt;
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; <span class="prelude-ty">Option</span>&lt;R&gt; + Sync + Send,
        R: Send,
    {
        <span class="kw">fn </span>yes&lt;T&gt;(<span class="kw">_</span>: <span class="kw-2">&amp;</span>T) -&gt; bool {
            <span class="bool-val">true
        </span>}
        <span class="self">self</span>.filter_map(predicate).find_any(yes)
    }

    <span class="doccomment">/// Applies the given predicate to the items in the parallel iterator and
    /// returns the sequentially **first** non-None result of the map operation.
    ///
    /// Once a non-None value is produced from the map operation, all attempts
    /// to the right of the match will be stopped, while attempts to the left
    /// must continue in case an earlier match is found.
    ///
    /// Note that not all parallel iterators have a useful order, much like
    /// sequential `HashMap` iteration, so "first" may be nebulous. If you
    /// just want the first non-None value discovered anywhere in the iterator,
    /// `find_map_any` is a better choice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let c = ["lol", "NaN", "2", "5"];
    ///
    /// let first_number = c.par_iter().find_map_first(|s| s.parse().ok());
    ///
    /// assert_eq!(first_number, Some(2));
    /// ```
    </span><span class="kw">fn </span>find_map_first&lt;P, R&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;R&gt;
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; <span class="prelude-ty">Option</span>&lt;R&gt; + Sync + Send,
        R: Send,
    {
        <span class="kw">fn </span>yes&lt;T&gt;(<span class="kw">_</span>: <span class="kw-2">&amp;</span>T) -&gt; bool {
            <span class="bool-val">true
        </span>}
        <span class="self">self</span>.filter_map(predicate).find_first(yes)
    }

    <span class="doccomment">/// Applies the given predicate to the items in the parallel iterator and
    /// returns the sequentially **last** non-None result of the map operation.
    ///
    /// Once a non-None value is produced from the map operation, all attempts
    /// to the left of the match will be stopped, while attempts to the right
    /// must continue in case a later match is found.
    ///
    /// Note that not all parallel iterators have a useful order, much like
    /// sequential `HashMap` iteration, so "first" may be nebulous. If you
    /// just want the first non-None value discovered anywhere in the iterator,
    /// `find_map_any` is a better choice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let c = ["lol", "NaN", "2", "5"];
    ///
    /// let last_number = c.par_iter().find_map_last(|s| s.parse().ok());
    ///
    /// assert_eq!(last_number, Some(5));
    /// ```
    </span><span class="kw">fn </span>find_map_last&lt;P, R&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;R&gt;
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; <span class="prelude-ty">Option</span>&lt;R&gt; + Sync + Send,
        R: Send,
    {
        <span class="kw">fn </span>yes&lt;T&gt;(<span class="kw">_</span>: <span class="kw-2">&amp;</span>T) -&gt; bool {
            <span class="bool-val">true
        </span>}
        <span class="self">self</span>.filter_map(predicate).find_last(yes)
    }

    <span class="attr">#[doc(hidden)]
    #[deprecated(note = <span class="string">"parallel `find` does not search in order -- use `find_any`, \\
                         `find_first`, or `find_last`"</span>)]
    </span><span class="kw">fn </span>find&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>::Item&gt;
    <span class="kw">where
        </span>P: Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        <span class="self">self</span>.find_any(predicate)
    }

    <span class="doccomment">/// Searches for **some** item in the parallel iterator that
    /// matches the given predicate, and if so returns true.  Once
    /// a match is found, we'll attempt to stop process the rest
    /// of the items.  Proving that there's no match, returning false,
    /// does require visiting every item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [0, 12, 3, 4, 0, 23, 0];
    ///
    /// let is_valid = a.par_iter().any(|&amp;x| x &gt; 10);
    ///
    /// assert!(is_valid);
    /// ```
    </span><span class="kw">fn </span>any&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; bool
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        <span class="self">self</span>.map(predicate).find_any(bool::clone).is_some()
    }

    <span class="doccomment">/// Tests that every item in the parallel iterator matches the given
    /// predicate, and if so returns true.  If a counter-example is found,
    /// we'll attempt to stop processing more items, then return false.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [0, 12, 3, 4, 0, 23, 0];
    ///
    /// let is_valid = a.par_iter().all(|&amp;x| x &gt; 10);
    ///
    /// assert!(!is_valid);
    /// ```
    </span><span class="kw">fn </span>all&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; bool
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        <span class="attr">#[inline]
        </span><span class="kw">fn </span>is_false(x: <span class="kw-2">&amp;</span>bool) -&gt; bool {
            !x
        }

        <span class="self">self</span>.map(predicate).find_any(is_false).is_none()
    }

    <span class="doccomment">/// Creates an iterator over the `Some` items of this iterator, halting
    /// as soon as any `None` is found.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use std::sync::atomic::{AtomicUsize, Ordering};
    ///
    /// let counter = AtomicUsize::new(0);
    /// let value = (0_i32..2048)
    ///     .into_par_iter()
    ///     .map(|x| {
    ///              counter.fetch_add(1, Ordering::SeqCst);
    ///              if x &lt; 1024 { Some(x) } else { None }
    ///          })
    ///     .while_some()
    ///     .max();
    ///
    /// assert!(value &lt; Some(1024));
    /// assert!(counter.load(Ordering::SeqCst) &lt; 2048); // should not have visited every single one
    /// ```
    </span><span class="kw">fn </span>while_some&lt;T&gt;(<span class="self">self</span>) -&gt; WhileSome&lt;<span class="self">Self</span>&gt;
    <span class="kw">where
        </span><span class="self">Self</span>: ParallelIterator&lt;Item = <span class="prelude-ty">Option</span>&lt;T&gt;&gt;,
        T: Send,
    {
        WhileSome::new(<span class="self">self</span>)
    }

    <span class="doccomment">/// Wraps an iterator with a fuse in case of panics, to halt all threads
    /// as soon as possible.
    ///
    /// Panics within parallel iterators are always propagated to the caller,
    /// but they don't always halt the rest of the iterator right away, due to
    /// the internal semantics of [`join`]. This adaptor makes a greater effort
    /// to stop processing other items sooner, with the cost of additional
    /// synchronization overhead, which may also inhibit some optimizations.
    ///
    /// [`join`]: ../fn.join.html#panics
    ///
    /// # Examples
    ///
    /// If this code didn't use `panic_fuse()`, it would continue processing
    /// many more items in other threads (with long sleep delays) before the
    /// panic is finally propagated.
    ///
    /// ```should_panic
    /// use rayon::prelude::*;
    /// use std::{thread, time};
    ///
    /// (0..1_000_000)
    ///     .into_par_iter()
    ///     .panic_fuse()
    ///     .for_each(|i| {
    ///         // simulate some work
    ///         thread::sleep(time::Duration::from_secs(1));
    ///         assert!(i &gt; 0); // oops!
    ///     });
    /// ```
    </span><span class="kw">fn </span>panic_fuse(<span class="self">self</span>) -&gt; PanicFuse&lt;<span class="self">Self</span>&gt; {
        PanicFuse::new(<span class="self">self</span>)
    }

    <span class="doccomment">/// Creates a fresh collection containing all the elements produced
    /// by this parallel iterator.
    ///
    /// You may prefer [`collect_into_vec()`] implemented on
    /// [`IndexedParallelIterator`], if your underlying iterator also implements
    /// it. [`collect_into_vec()`] allocates efficiently with precise knowledge
    /// of how many elements the iterator contains, and even allows you to reuse
    /// an existing vector's backing store rather than allocating a fresh vector.
    ///
    /// See also [`collect_vec_list()`][Self::collect_vec_list] for collecting
    /// into a `LinkedList&lt;Vec&lt;T&gt;&gt;`.
    ///
    /// [`IndexedParallelIterator`]: trait.IndexedParallelIterator.html
    /// [`collect_into_vec()`]:
    ///     trait.IndexedParallelIterator.html#method.collect_into_vec
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let sync_vec: Vec&lt;_&gt; = (0..100).into_iter().collect();
    ///
    /// let async_vec: Vec&lt;_&gt; = (0..100).into_par_iter().collect();
    ///
    /// assert_eq!(sync_vec, async_vec);
    /// ```
    ///
    /// You can collect a pair of collections like [`unzip`](#method.unzip)
    /// for paired items:
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [(0, 1), (1, 2), (2, 3), (3, 4)];
    /// let (first, second): (Vec&lt;_&gt;, Vec&lt;_&gt;) = a.into_par_iter().collect();
    ///
    /// assert_eq!(first, [0, 1, 2, 3]);
    /// assert_eq!(second, [1, 2, 3, 4]);
    /// ```
    ///
    /// Or like [`partition_map`](#method.partition_map) for `Either` items:
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use rayon::iter::Either;
    ///
    /// let (left, right): (Vec&lt;_&gt;, Vec&lt;_&gt;) = (0..8).into_par_iter().map(|x| {
    ///     if x % 2 == 0 {
    ///         Either::Left(x * 4)
    ///     } else {
    ///         Either::Right(x * 3)
    ///     }
    /// }).collect();
    ///
    /// assert_eq!(left, [0, 8, 16, 24]);
    /// assert_eq!(right, [3, 9, 15, 21]);
    /// ```
    ///
    /// You can even collect an arbitrarily-nested combination of pairs and `Either`:
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use rayon::iter::Either;
    ///
    /// let (first, (left, right)): (Vec&lt;_&gt;, (Vec&lt;_&gt;, Vec&lt;_&gt;))
    ///     = (0..8).into_par_iter().map(|x| {
    ///         if x % 2 == 0 {
    ///             (x, Either::Left(x * 4))
    ///         } else {
    ///             (-x, Either::Right(x * 3))
    ///         }
    ///     }).collect();
    ///
    /// assert_eq!(first, [0, -1, 2, -3, 4, -5, 6, -7]);
    /// assert_eq!(left, [0, 8, 16, 24]);
    /// assert_eq!(right, [3, 9, 15, 21]);
    /// ```
    ///
    /// All of that can _also_ be combined with short-circuiting collection of
    /// `Result` or `Option` types:
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use rayon::iter::Either;
    ///
    /// let result: Result&lt;(Vec&lt;_&gt;, (Vec&lt;_&gt;, Vec&lt;_&gt;)), _&gt;
    ///     = (0..8).into_par_iter().map(|x| {
    ///         if x &gt; 5 {
    ///             Err(x)
    ///         } else if x % 2 == 0 {
    ///             Ok((x, Either::Left(x * 4)))
    ///         } else {
    ///             Ok((-x, Either::Right(x * 3)))
    ///         }
    ///     }).collect();
    ///
    /// let error = result.unwrap_err();
    /// assert!(error == 6 || error == 7);
    /// ```
    </span><span class="kw">fn </span>collect&lt;C&gt;(<span class="self">self</span>) -&gt; C
    <span class="kw">where
        </span>C: FromParallelIterator&lt;<span class="self">Self</span>::Item&gt;,
    {
        C::from_par_iter(<span class="self">self</span>)
    }

    <span class="doccomment">/// Unzips the items of a parallel iterator into a pair of arbitrary
    /// `ParallelExtend` containers.
    ///
    /// You may prefer to use `unzip_into_vecs()`, which allocates more
    /// efficiently with precise knowledge of how many elements the
    /// iterator contains, and even allows you to reuse existing
    /// vectors' backing stores rather than allocating fresh vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [(0, 1), (1, 2), (2, 3), (3, 4)];
    ///
    /// let (left, right): (Vec&lt;_&gt;, Vec&lt;_&gt;) = a.par_iter().cloned().unzip();
    ///
    /// assert_eq!(left, [0, 1, 2, 3]);
    /// assert_eq!(right, [1, 2, 3, 4]);
    /// ```
    ///
    /// Nested pairs can be unzipped too.
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let (values, (squares, cubes)): (Vec&lt;_&gt;, (Vec&lt;_&gt;, Vec&lt;_&gt;)) = (0..4).into_par_iter()
    ///     .map(|i| (i, (i * i, i * i * i)))
    ///     .unzip();
    ///
    /// assert_eq!(values, [0, 1, 2, 3]);
    /// assert_eq!(squares, [0, 1, 4, 9]);
    /// assert_eq!(cubes, [0, 1, 8, 27]);
    /// ```
    </span><span class="kw">fn </span>unzip&lt;A, B, FromA, FromB&gt;(<span class="self">self</span>) -&gt; (FromA, FromB)
    <span class="kw">where
        </span><span class="self">Self</span>: ParallelIterator&lt;Item = (A, B)&gt;,
        FromA: Default + Send + ParallelExtend&lt;A&gt;,
        FromB: Default + Send + ParallelExtend&lt;B&gt;,
        A: Send,
        B: Send,
    {
        unzip::unzip(<span class="self">self</span>)
    }

    <span class="doccomment">/// Partitions the items of a parallel iterator into a pair of arbitrary
    /// `ParallelExtend` containers.  Items for which the `predicate` returns
    /// true go into the first container, and the rest go into the second.
    ///
    /// Note: unlike the standard `Iterator::partition`, this allows distinct
    /// collection types for the left and right items.  This is more flexible,
    /// but may require new type annotations when converting sequential code
    /// that used type inference assuming the two were the same.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let (left, right): (Vec&lt;_&gt;, Vec&lt;_&gt;) = (0..8).into_par_iter().partition(|x| x % 2 == 0);
    ///
    /// assert_eq!(left, [0, 2, 4, 6]);
    /// assert_eq!(right, [1, 3, 5, 7]);
    /// ```
    </span><span class="kw">fn </span>partition&lt;A, B, P&gt;(<span class="self">self</span>, predicate: P) -&gt; (A, B)
    <span class="kw">where
        </span>A: Default + Send + ParallelExtend&lt;<span class="self">Self</span>::Item&gt;,
        B: Default + Send + ParallelExtend&lt;<span class="self">Self</span>::Item&gt;,
        P: Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        unzip::partition(<span class="self">self</span>, predicate)
    }

    <span class="doccomment">/// Partitions and maps the items of a parallel iterator into a pair of
    /// arbitrary `ParallelExtend` containers.  `Either::Left` items go into
    /// the first container, and `Either::Right` items go into the second.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use rayon::iter::Either;
    ///
    /// let (left, right): (Vec&lt;_&gt;, Vec&lt;_&gt;) = (0..8).into_par_iter()
    ///     .partition_map(|x| {
    ///         if x % 2 == 0 {
    ///             Either::Left(x * 4)
    ///         } else {
    ///             Either::Right(x * 3)
    ///         }
    ///     });
    ///
    /// assert_eq!(left, [0, 8, 16, 24]);
    /// assert_eq!(right, [3, 9, 15, 21]);
    /// ```
    ///
    /// Nested `Either` enums can be split as well.
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use rayon::iter::Either::*;
    ///
    /// let ((fizzbuzz, fizz), (buzz, other)): ((Vec&lt;_&gt;, Vec&lt;_&gt;), (Vec&lt;_&gt;, Vec&lt;_&gt;)) = (1..20)
    ///     .into_par_iter()
    ///     .partition_map(|x| match (x % 3, x % 5) {
    ///         (0, 0) =&gt; Left(Left(x)),
    ///         (0, _) =&gt; Left(Right(x)),
    ///         (_, 0) =&gt; Right(Left(x)),
    ///         (_, _) =&gt; Right(Right(x)),
    ///     });
    ///
    /// assert_eq!(fizzbuzz, [15]);
    /// assert_eq!(fizz, [3, 6, 9, 12, 18]);
    /// assert_eq!(buzz, [5, 10]);
    /// assert_eq!(other, [1, 2, 4, 7, 8, 11, 13, 14, 16, 17, 19]);
    /// ```
    </span><span class="kw">fn </span>partition_map&lt;A, B, P, L, R&gt;(<span class="self">self</span>, predicate: P) -&gt; (A, B)
    <span class="kw">where
        </span>A: Default + Send + ParallelExtend&lt;L&gt;,
        B: Default + Send + ParallelExtend&lt;R&gt;,
        P: Fn(<span class="self">Self</span>::Item) -&gt; Either&lt;L, R&gt; + Sync + Send,
        L: Send,
        R: Send,
    {
        unzip::partition_map(<span class="self">self</span>, predicate)
    }

    <span class="doccomment">/// Intersperses clones of an element between items of this iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let x = vec![1, 2, 3];
    /// let r: Vec&lt;_&gt; = x.into_par_iter().intersperse(-1).collect();
    ///
    /// assert_eq!(r, vec![1, -1, 2, -1, 3]);
    /// ```
    </span><span class="kw">fn </span>intersperse(<span class="self">self</span>, element: <span class="self">Self</span>::Item) -&gt; Intersperse&lt;<span class="self">Self</span>&gt;
    <span class="kw">where
        </span><span class="self">Self</span>::Item: Clone,
    {
        Intersperse::new(<span class="self">self</span>, element)
    }

    <span class="doccomment">/// Creates an iterator that yields `n` elements from *anywhere* in the original iterator.
    ///
    /// This is similar to [`IndexedParallelIterator::take`] without being
    /// constrained to the "first" `n` of the original iterator order. The
    /// taken items will still maintain their relative order where that is
    /// visible in `collect`, `reduce`, and similar outputs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let result: Vec&lt;_&gt; = (0..100)
    ///     .into_par_iter()
    ///     .filter(|&amp;x| x % 2 == 0)
    ///     .take_any(5)
    ///     .collect();
    ///
    /// assert_eq!(result.len(), 5);
    /// assert!(result.windows(2).all(|w| w[0] &lt; w[1]));
    /// ```
    </span><span class="kw">fn </span>take_any(<span class="self">self</span>, n: usize) -&gt; TakeAny&lt;<span class="self">Self</span>&gt; {
        TakeAny::new(<span class="self">self</span>, n)
    }

    <span class="doccomment">/// Creates an iterator that skips `n` elements from *anywhere* in the original iterator.
    ///
    /// This is similar to [`IndexedParallelIterator::skip`] without being
    /// constrained to the "first" `n` of the original iterator order. The
    /// remaining items will still maintain their relative order where that is
    /// visible in `collect`, `reduce`, and similar outputs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let result: Vec&lt;_&gt; = (0..100)
    ///     .into_par_iter()
    ///     .filter(|&amp;x| x % 2 == 0)
    ///     .skip_any(5)
    ///     .collect();
    ///
    /// assert_eq!(result.len(), 45);
    /// assert!(result.windows(2).all(|w| w[0] &lt; w[1]));
    /// ```
    </span><span class="kw">fn </span>skip_any(<span class="self">self</span>, n: usize) -&gt; SkipAny&lt;<span class="self">Self</span>&gt; {
        SkipAny::new(<span class="self">self</span>, n)
    }

    <span class="doccomment">/// Creates an iterator that takes elements from *anywhere* in the original iterator
    /// until the given `predicate` returns `false`.
    ///
    /// The `predicate` may be anything -- e.g. it could be checking a fact about the item, a
    /// global condition unrelated to the item itself, or some combination thereof.
    ///
    /// If parallel calls to the `predicate` race and give different results, then the
    /// `true` results will still take those particular items, while respecting the `false`
    /// result from elsewhere to skip any further items.
    ///
    /// This is similar to [`Iterator::take_while`] without being constrained to the original
    /// iterator order. The taken items will still maintain their relative order where that is
    /// visible in `collect`, `reduce`, and similar outputs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let result: Vec&lt;_&gt; = (0..100)
    ///     .into_par_iter()
    ///     .take_any_while(|x| *x &lt; 50)
    ///     .collect();
    ///
    /// assert!(result.len() &lt;= 50);
    /// assert!(result.windows(2).all(|w| w[0] &lt; w[1]));
    /// ```
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use std::sync::atomic::AtomicUsize;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// // Collect any group of items that sum &lt;= 1000
    /// let quota = AtomicUsize::new(1000);
    /// let result: Vec&lt;_&gt; = (0_usize..100)
    ///     .into_par_iter()
    ///     .take_any_while(|&amp;x| {
    ///         quota.fetch_update(Relaxed, Relaxed, |q| q.checked_sub(x))
    ///             .is_ok()
    ///     })
    ///     .collect();
    ///
    /// let sum = result.iter().sum::&lt;usize&gt;();
    /// assert!(matches!(sum, 902..=1000));
    /// ```
    </span><span class="kw">fn </span>take_any_while&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; TakeAnyWhile&lt;<span class="self">Self</span>, P&gt;
    <span class="kw">where
        </span>P: Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        TakeAnyWhile::new(<span class="self">self</span>, predicate)
    }

    <span class="doccomment">/// Creates an iterator that skips elements from *anywhere* in the original iterator
    /// until the given `predicate` returns `false`.
    ///
    /// The `predicate` may be anything -- e.g. it could be checking a fact about the item, a
    /// global condition unrelated to the item itself, or some combination thereof.
    ///
    /// If parallel calls to the `predicate` race and give different results, then the
    /// `true` results will still skip those particular items, while respecting the `false`
    /// result from elsewhere to skip any further items.
    ///
    /// This is similar to [`Iterator::skip_while`] without being constrained to the original
    /// iterator order. The remaining items will still maintain their relative order where that is
    /// visible in `collect`, `reduce`, and similar outputs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let result: Vec&lt;_&gt; = (0..100)
    ///     .into_par_iter()
    ///     .skip_any_while(|x| *x &lt; 50)
    ///     .collect();
    ///
    /// assert!(result.len() &gt;= 50);
    /// assert!(result.windows(2).all(|w| w[0] &lt; w[1]));
    /// ```
    </span><span class="kw">fn </span>skip_any_while&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; SkipAnyWhile&lt;<span class="self">Self</span>, P&gt;
    <span class="kw">where
        </span>P: Fn(<span class="kw-2">&amp;</span><span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        SkipAnyWhile::new(<span class="self">self</span>, predicate)
    }

    <span class="doccomment">/// Collects this iterator into a linked list of vectors.
    ///
    /// This is useful when you need to condense a parallel iterator into a collection,
    /// but have no specific requirements for what that collection should be. If you
    /// plan to store the collection longer-term, `Vec&lt;T&gt;` is, as always, likely the
    /// best default choice, despite the overhead that comes from concatenating each
    /// vector. Or, if this is an `IndexedParallelIterator`, you should also prefer to
    /// just collect to a `Vec&lt;T&gt;`.
    ///
    /// Internally, most [`FromParallelIterator`]/[`ParallelExtend`] implementations
    /// use this strategy; each job collecting their chunk of the iterator to a `Vec&lt;T&gt;`
    /// and those chunks getting merged into a `LinkedList`, before then extending the
    /// collection with each vector. This is a very efficient way to collect an
    /// unindexed parallel iterator, without much intermediate data movement.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::LinkedList;
    /// use rayon::prelude::*;
    ///
    /// let result: LinkedList&lt;Vec&lt;_&gt;&gt; = (0..=100)
    ///     .into_par_iter()
    ///     .filter(|x| x % 2 == 0)
    ///     .flat_map(|x| 0..x)
    ///     .collect_vec_list();
    ///
    /// // `par_iter.collect_vec_list().into_iter().flatten()` turns
    /// // a parallel iterator into a serial one
    /// let total_len = result.into_iter().flatten().count();
    /// assert_eq!(total_len, 2550);
    /// ```
    </span><span class="kw">fn </span>collect_vec_list(<span class="self">self</span>) -&gt; LinkedList&lt;Vec&lt;<span class="self">Self</span>::Item&gt;&gt; {
        <span class="kw">match </span>extend::fast_collect(<span class="self">self</span>) {
            Either::Left(vec) =&gt; {
                <span class="kw">let </span><span class="kw-2">mut </span>list = LinkedList::new();
                <span class="kw">if </span>!vec.is_empty() {
                    list.push_back(vec);
                }
                list
            }
            Either::Right(list) =&gt; list,
        }
    }

    <span class="doccomment">/// Internal method used to define the behavior of this parallel
    /// iterator. You should not need to call this directly.
    ///
    /// This method causes the iterator `self` to start producing
    /// items and to feed them to the consumer `consumer` one by one.
    /// It may split the consumer before doing so to create the
    /// opportunity to produce in parallel.
    ///
    /// See the [README] for more details on the internals of parallel
    /// iterators.
    ///
    /// [README]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md
    </span><span class="kw">fn </span>drive_unindexed&lt;C&gt;(<span class="self">self</span>, consumer: C) -&gt; C::Result
    <span class="kw">where
        </span>C: UnindexedConsumer&lt;<span class="self">Self</span>::Item&gt;;

    <span class="doccomment">/// Internal method used to define the behavior of this parallel
    /// iterator. You should not need to call this directly.
    ///
    /// Returns the number of items produced by this iterator, if known
    /// statically. This can be used by consumers to trigger special fast
    /// paths. Therefore, if `Some(_)` is returned, this iterator must only
    /// use the (indexed) `Consumer` methods when driving a consumer, such
    /// as `split_at()`. Calling `UnindexedConsumer::split_off_left()` or
    /// other `UnindexedConsumer` methods -- or returning an inaccurate
    /// value -- may result in panics.
    ///
    /// This method is currently used to optimize `collect` for want
    /// of true Rust specialization; it may be removed when
    /// specialization is stable.
    </span><span class="kw">fn </span>opt_len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt; {
        <span class="prelude-val">None
    </span>}
}

<span class="kw">impl</span>&lt;T: ParallelIterator&gt; IntoParallelIterator <span class="kw">for </span>T {
    <span class="kw">type </span>Iter = T;
    <span class="kw">type </span>Item = T::Item;

    <span class="kw">fn </span>into_par_iter(<span class="self">self</span>) -&gt; T {
        <span class="self">self
    </span>}
}

<span class="doccomment">/// An iterator that supports "random access" to its data, meaning
/// that you can split it at arbitrary indices and draw data from
/// those points.
///
/// **Note:** Not implemented for `u64`, `i64`, `u128`, or `i128` ranges
</span><span class="comment">// Waiting for `ExactSizeIterator::is_empty` to be stabilized. See rust-lang/rust#35428
</span><span class="attr">#[allow(clippy::len_without_is_empty)]
</span><span class="kw">pub trait </span>IndexedParallelIterator: ParallelIterator {
    <span class="doccomment">/// Divides an iterator into sequential blocks of exponentially-increasing size.
    ///
    /// Normally, parallel iterators are recursively divided into tasks in parallel.
    /// This adaptor changes the default behavior by splitting the iterator into a **sequence**
    /// of parallel iterators of increasing sizes.
    /// Sizes grow exponentially in order to avoid creating
    /// too many blocks. This also allows to balance the current block with all previous ones.
    ///
    /// This can have many applications but the most notable ones are:
    /// - better performance with [`find_first()`][ParallelIterator::find_first]
    /// - more predictable performance with [`find_any()`][ParallelIterator::find_any]
    ///   or any interruptible computation
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// assert_eq!((0..10_000).into_par_iter()
    ///                       .by_exponential_blocks()
    ///                       .find_first(|&amp;e| e==4_999), Some(4_999))
    /// ```
    ///
    /// In this example, without blocks, rayon will split the initial range into two but all work
    /// on the right hand side (from 5,000 onwards) is **useless** since the sequential algorithm
    /// never goes there. This means that if two threads are used there will be **no** speedup **at
    /// all**.
    ///
    /// `by_exponential_blocks` on the other hand will start with the leftmost range from 0
    /// to `p` (threads number), continue with p to 3p, the 3p to 7p...
    ///
    /// Each subrange is treated in parallel, while all subranges are treated sequentially.
    /// We therefore ensure a logarithmic number of blocks (and overhead) while guaranteeing
    /// we stop at the first block containing the searched data.
    </span><span class="kw">fn </span>by_exponential_blocks(<span class="self">self</span>) -&gt; ExponentialBlocks&lt;<span class="self">Self</span>&gt; {
        ExponentialBlocks::new(<span class="self">self</span>)
    }

    <span class="doccomment">/// Divides an iterator into sequential blocks of the given size.
    ///
    /// Normally, parallel iterators are recursively divided into tasks in parallel.
    /// This adaptor changes the default behavior by splitting the iterator into a **sequence**
    /// of parallel iterators of given `block_size`.
    /// The main application is to obtain better
    /// memory locality (especially if the reduce operation re-use folded data).
    ///
    /// **Panics** if `block_size` is 0.
    ///
    /// # Example
    /// ```
    /// use rayon::prelude::*;
    /// // during most reductions v1 and v2 fit the cache
    /// let v = (0u32..10_000_000)
    ///     .into_par_iter()
    ///     .by_uniform_blocks(1_000_000)
    ///     .fold(Vec::new, |mut v, e| { v.push(e); v})
    ///     .reduce(Vec::new, |mut v1, mut v2| { v1.append(&amp;mut v2); v1});
    /// assert_eq!(v, (0u32..10_000_000).collect::&lt;Vec&lt;u32&gt;&gt;());
    /// ```
    </span><span class="attr">#[track_caller]
    </span><span class="kw">fn </span>by_uniform_blocks(<span class="self">self</span>, block_size: usize) -&gt; UniformBlocks&lt;<span class="self">Self</span>&gt; {
        <span class="macro">assert!</span>(block_size != <span class="number">0</span>, <span class="string">"block_size must not be zero"</span>);
        UniformBlocks::new(<span class="self">self</span>, block_size)
    }

    <span class="doccomment">/// Collects the results of the iterator into the specified
    /// vector. The vector is always cleared before execution
    /// begins. If possible, reusing the vector across calls can lead
    /// to better performance since it reuses the same backing buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// // any prior data will be cleared
    /// let mut vec = vec![-1, -2, -3];
    ///
    /// (0..5).into_par_iter()
    ///     .collect_into_vec(&amp;mut vec);
    ///
    /// assert_eq!(vec, [0, 1, 2, 3, 4]);
    /// ```
    </span><span class="kw">fn </span>collect_into_vec(<span class="self">self</span>, target: <span class="kw-2">&amp;mut </span>Vec&lt;<span class="self">Self</span>::Item&gt;) {
        collect::collect_into_vec(<span class="self">self</span>, target);
    }

    <span class="doccomment">/// Unzips the results of the iterator into the specified
    /// vectors. The vectors are always cleared before execution
    /// begins. If possible, reusing the vectors across calls can lead
    /// to better performance since they reuse the same backing buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// // any prior data will be cleared
    /// let mut left = vec![42; 10];
    /// let mut right = vec![-1; 10];
    ///
    /// (10..15).into_par_iter()
    ///     .enumerate()
    ///     .unzip_into_vecs(&amp;mut left, &amp;mut right);
    ///
    /// assert_eq!(left, [0, 1, 2, 3, 4]);
    /// assert_eq!(right, [10, 11, 12, 13, 14]);
    /// ```
    </span><span class="kw">fn </span>unzip_into_vecs&lt;A, B&gt;(<span class="self">self</span>, left: <span class="kw-2">&amp;mut </span>Vec&lt;A&gt;, right: <span class="kw-2">&amp;mut </span>Vec&lt;B&gt;)
    <span class="kw">where
        </span><span class="self">Self</span>: IndexedParallelIterator&lt;Item = (A, B)&gt;,
        A: Send,
        B: Send,
    {
        collect::unzip_into_vecs(<span class="self">self</span>, left, right);
    }

    <span class="doccomment">/// Iterates over tuples `(A, B)`, where the items `A` are from
    /// this iterator and `B` are from the iterator given as argument.
    /// Like the `zip` method on ordinary iterators, if the two
    /// iterators are of unequal length, you only get the items they
    /// have in common.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let result: Vec&lt;_&gt; = (1..4)
    ///     .into_par_iter()
    ///     .zip(vec!['a', 'b', 'c'])
    ///     .collect();
    ///
    /// assert_eq!(result, [(1, 'a'), (2, 'b'), (3, 'c')]);
    /// ```
    </span><span class="kw">fn </span>zip&lt;Z&gt;(<span class="self">self</span>, zip_op: Z) -&gt; Zip&lt;<span class="self">Self</span>, Z::Iter&gt;
    <span class="kw">where
        </span>Z: IntoParallelIterator,
        Z::Iter: IndexedParallelIterator,
    {
        Zip::new(<span class="self">self</span>, zip_op.into_par_iter())
    }

    <span class="doccomment">/// The same as `Zip`, but requires that both iterators have the same length.
    ///
    /// # Panics
    /// Will panic if `self` and `zip_op` are not the same length.
    ///
    /// ```should_panic
    /// use rayon::prelude::*;
    ///
    /// let one = [1u8];
    /// let two = [2u8, 2];
    /// let one_iter = one.par_iter();
    /// let two_iter = two.par_iter();
    ///
    /// // this will panic
    /// let zipped: Vec&lt;(&amp;u8, &amp;u8)&gt; = one_iter.zip_eq(two_iter).collect();
    ///
    /// // we should never get here
    /// assert_eq!(1, zipped.len());
    /// ```
    </span><span class="attr">#[track_caller]
    </span><span class="kw">fn </span>zip_eq&lt;Z&gt;(<span class="self">self</span>, zip_op: Z) -&gt; ZipEq&lt;<span class="self">Self</span>, Z::Iter&gt;
    <span class="kw">where
        </span>Z: IntoParallelIterator,
        Z::Iter: IndexedParallelIterator,
    {
        <span class="kw">let </span>zip_op_iter = zip_op.into_par_iter();
        <span class="macro">assert_eq!</span>(
            <span class="self">self</span>.len(),
            zip_op_iter.len(),
            <span class="string">"iterators must have the same length"
        </span>);
        ZipEq::new(<span class="self">self</span>, zip_op_iter)
    }

    <span class="doccomment">/// Interleaves elements of this iterator and the other given
    /// iterator. Alternately yields elements from this iterator and
    /// the given iterator, until both are exhausted. If one iterator
    /// is exhausted before the other, the last elements are provided
    /// from the other.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let (x, y) = (vec![1, 2], vec![3, 4, 5, 6]);
    /// let r: Vec&lt;i32&gt; = x.into_par_iter().interleave(y).collect();
    /// assert_eq!(r, vec![1, 3, 2, 4, 5, 6]);
    /// ```
    </span><span class="kw">fn </span>interleave&lt;I&gt;(<span class="self">self</span>, other: I) -&gt; Interleave&lt;<span class="self">Self</span>, I::Iter&gt;
    <span class="kw">where
        </span>I: IntoParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;,
        I::Iter: IndexedParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;,
    {
        Interleave::new(<span class="self">self</span>, other.into_par_iter())
    }

    <span class="doccomment">/// Interleaves elements of this iterator and the other given
    /// iterator, until one is exhausted.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let (x, y) = (vec![1, 2, 3, 4], vec![5, 6]);
    /// let r: Vec&lt;i32&gt; = x.into_par_iter().interleave_shortest(y).collect();
    /// assert_eq!(r, vec![1, 5, 2, 6, 3]);
    /// ```
    </span><span class="kw">fn </span>interleave_shortest&lt;I&gt;(<span class="self">self</span>, other: I) -&gt; InterleaveShortest&lt;<span class="self">Self</span>, I::Iter&gt;
    <span class="kw">where
        </span>I: IntoParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;,
        I::Iter: IndexedParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;,
    {
        InterleaveShortest::new(<span class="self">self</span>, other.into_par_iter())
    }

    <span class="doccomment">/// Splits an iterator up into fixed-size chunks.
    ///
    /// Returns an iterator that returns `Vec`s of the given number of elements.
    /// If the number of elements in the iterator is not divisible by `chunk_size`,
    /// the last chunk may be shorter than `chunk_size`.
    ///
    /// See also [`par_chunks()`] and [`par_chunks_mut()`] for similar behavior on
    /// slices, without having to allocate intermediate `Vec`s for the chunks.
    ///
    /// [`par_chunks()`]: ../slice/trait.ParallelSlice.html#method.par_chunks
    /// [`par_chunks_mut()`]: ../slice/trait.ParallelSliceMut.html#method.par_chunks_mut
    ///
    /// **Panics** if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let r: Vec&lt;Vec&lt;i32&gt;&gt; = a.into_par_iter().chunks(3).collect();
    /// assert_eq!(r, vec![vec![1,2,3], vec![4,5,6], vec![7,8,9], vec![10]]);
    /// ```
    </span><span class="attr">#[track_caller]
    </span><span class="kw">fn </span>chunks(<span class="self">self</span>, chunk_size: usize) -&gt; Chunks&lt;<span class="self">Self</span>&gt; {
        <span class="macro">assert!</span>(chunk_size != <span class="number">0</span>, <span class="string">"chunk_size must not be zero"</span>);
        Chunks::new(<span class="self">self</span>, chunk_size)
    }

    <span class="doccomment">/// Splits an iterator into fixed-size chunks, performing a sequential [`fold()`] on
    /// each chunk.
    ///
    /// Returns an iterator that produces a folded result for each chunk of items
    /// produced by this iterator.
    ///
    /// This works essentially like:
    ///
    /// ```text
    /// iter.chunks(chunk_size)
    ///     .map(|chunk|
    ///         chunk.into_iter()
    ///             .fold(identity, fold_op)
    ///     )
    /// ```
    ///
    /// except there is no per-chunk allocation overhead.
    ///
    /// [`fold()`]: std::iter::Iterator#method.fold
    ///
    /// **Panics** if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let chunk_sums = nums.into_par_iter().fold_chunks(2, || 0, |a, n| a + n).collect::&lt;Vec&lt;_&gt;&gt;();
    /// assert_eq!(chunk_sums, vec![3, 7, 11, 15, 19]);
    /// ```
    </span><span class="attr">#[track_caller]
    </span><span class="kw">fn </span>fold_chunks&lt;T, ID, F&gt;(
        <span class="self">self</span>,
        chunk_size: usize,
        identity: ID,
        fold_op: F,
    ) -&gt; FoldChunks&lt;<span class="self">Self</span>, ID, F&gt;
    <span class="kw">where
        </span>ID: Fn() -&gt; T + Send + Sync,
        F: Fn(T, <span class="self">Self</span>::Item) -&gt; T + Send + Sync,
        T: Send,
    {
        <span class="macro">assert!</span>(chunk_size != <span class="number">0</span>, <span class="string">"chunk_size must not be zero"</span>);
        FoldChunks::new(<span class="self">self</span>, chunk_size, identity, fold_op)
    }

    <span class="doccomment">/// Splits an iterator into fixed-size chunks, performing a sequential [`fold()`] on
    /// each chunk.
    ///
    /// Returns an iterator that produces a folded result for each chunk of items
    /// produced by this iterator.
    ///
    /// This works essentially like `fold_chunks(chunk_size, || init.clone(), fold_op)`,
    /// except it doesn't require the `init` type to be `Sync`, nor any other form of
    /// added synchronization.
    ///
    /// [`fold()`]: std::iter::Iterator#method.fold
    ///
    /// **Panics** if `chunk_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let chunk_sums = nums.into_par_iter().fold_chunks_with(2, 0, |a, n| a + n).collect::&lt;Vec&lt;_&gt;&gt;();
    /// assert_eq!(chunk_sums, vec![3, 7, 11, 15, 19]);
    /// ```
    </span><span class="attr">#[track_caller]
    </span><span class="kw">fn </span>fold_chunks_with&lt;T, F&gt;(
        <span class="self">self</span>,
        chunk_size: usize,
        init: T,
        fold_op: F,
    ) -&gt; FoldChunksWith&lt;<span class="self">Self</span>, T, F&gt;
    <span class="kw">where
        </span>T: Send + Clone,
        F: Fn(T, <span class="self">Self</span>::Item) -&gt; T + Send + Sync,
    {
        <span class="macro">assert!</span>(chunk_size != <span class="number">0</span>, <span class="string">"chunk_size must not be zero"</span>);
        FoldChunksWith::new(<span class="self">self</span>, chunk_size, init, fold_op)
    }

    <span class="doccomment">/// Lexicographically compares the elements of this `ParallelIterator` with those of
    /// another.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = vec![1, 2, 3];
    /// assert_eq!(x.par_iter().cmp(&amp;vec![1, 3, 0]), Less);
    /// assert_eq!(x.par_iter().cmp(&amp;vec![1, 2, 3]), Equal);
    /// assert_eq!(x.par_iter().cmp(&amp;vec![1, 2]), Greater);
    /// ```
    </span><span class="kw">fn </span>cmp&lt;I&gt;(<span class="self">self</span>, other: I) -&gt; Ordering
    <span class="kw">where
        </span>I: IntoParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;,
        I::Iter: IndexedParallelIterator,
        <span class="self">Self</span>::Item: Ord,
    {
        <span class="attr">#[inline]
        </span><span class="kw">fn </span>ordering&lt;T: Ord&gt;((x, y): (T, T)) -&gt; Ordering {
            Ord::cmp(<span class="kw-2">&amp;</span>x, <span class="kw-2">&amp;</span>y)
        }

        <span class="attr">#[inline]
        </span><span class="kw">fn </span>inequal(<span class="kw-2">&amp;</span>ord: <span class="kw-2">&amp;</span>Ordering) -&gt; bool {
            ord != Ordering::Equal
        }

        <span class="kw">let </span>other = other.into_par_iter();
        <span class="kw">let </span>ord_len = <span class="self">self</span>.len().cmp(<span class="kw-2">&amp;</span>other.len());
        <span class="self">self</span>.zip(other)
            .map(ordering)
            .find_first(inequal)
            .unwrap_or(ord_len)
    }

    <span class="doccomment">/// Lexicographically compares the elements of this `ParallelIterator` with those of
    /// another.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use std::cmp::Ordering::*;
    /// use std::f64::NAN;
    ///
    /// let x = vec![1.0, 2.0, 3.0];
    /// assert_eq!(x.par_iter().partial_cmp(&amp;vec![1.0, 3.0, 0.0]), Some(Less));
    /// assert_eq!(x.par_iter().partial_cmp(&amp;vec![1.0, 2.0, 3.0]), Some(Equal));
    /// assert_eq!(x.par_iter().partial_cmp(&amp;vec![1.0, 2.0]), Some(Greater));
    /// assert_eq!(x.par_iter().partial_cmp(&amp;vec![1.0, NAN]), None);
    /// ```
    </span><span class="kw">fn </span>partial_cmp&lt;I&gt;(<span class="self">self</span>, other: I) -&gt; <span class="prelude-ty">Option</span>&lt;Ordering&gt;
    <span class="kw">where
        </span>I: IntoParallelIterator,
        I::Iter: IndexedParallelIterator,
        <span class="self">Self</span>::Item: PartialOrd&lt;I::Item&gt;,
    {
        <span class="attr">#[inline]
        </span><span class="kw">fn </span>ordering&lt;T: PartialOrd&lt;U&gt;, U&gt;((x, y): (T, U)) -&gt; <span class="prelude-ty">Option</span>&lt;Ordering&gt; {
            PartialOrd::partial_cmp(<span class="kw-2">&amp;</span>x, <span class="kw-2">&amp;</span>y)
        }

        <span class="attr">#[inline]
        </span><span class="kw">fn </span>inequal(<span class="kw-2">&amp;</span>ord: <span class="kw-2">&amp;</span><span class="prelude-ty">Option</span>&lt;Ordering&gt;) -&gt; bool {
            ord != <span class="prelude-val">Some</span>(Ordering::Equal)
        }

        <span class="kw">let </span>other = other.into_par_iter();
        <span class="kw">let </span>ord_len = <span class="self">self</span>.len().cmp(<span class="kw-2">&amp;</span>other.len());
        <span class="self">self</span>.zip(other)
            .map(ordering)
            .find_first(inequal)
            .unwrap_or(<span class="prelude-val">Some</span>(ord_len))
    }

    <span class="doccomment">/// Determines if the elements of this `ParallelIterator`
    /// are equal to those of another
    </span><span class="kw">fn </span>eq&lt;I&gt;(<span class="self">self</span>, other: I) -&gt; bool
    <span class="kw">where
        </span>I: IntoParallelIterator,
        I::Iter: IndexedParallelIterator,
        <span class="self">Self</span>::Item: PartialEq&lt;I::Item&gt;,
    {
        <span class="attr">#[inline]
        </span><span class="kw">fn </span>eq&lt;T: PartialEq&lt;U&gt;, U&gt;((x, y): (T, U)) -&gt; bool {
            PartialEq::eq(<span class="kw-2">&amp;</span>x, <span class="kw-2">&amp;</span>y)
        }

        <span class="kw">let </span>other = other.into_par_iter();
        <span class="self">self</span>.len() == other.len() &amp;&amp; <span class="self">self</span>.zip(other).all(eq)
    }

    <span class="doccomment">/// Determines if the elements of this `ParallelIterator`
    /// are unequal to those of another
    </span><span class="kw">fn </span>ne&lt;I&gt;(<span class="self">self</span>, other: I) -&gt; bool
    <span class="kw">where
        </span>I: IntoParallelIterator,
        I::Iter: IndexedParallelIterator,
        <span class="self">Self</span>::Item: PartialEq&lt;I::Item&gt;,
    {
        !<span class="self">self</span>.eq(other)
    }

    <span class="doccomment">/// Determines if the elements of this `ParallelIterator`
    /// are lexicographically less than those of another.
    </span><span class="kw">fn </span>lt&lt;I&gt;(<span class="self">self</span>, other: I) -&gt; bool
    <span class="kw">where
        </span>I: IntoParallelIterator,
        I::Iter: IndexedParallelIterator,
        <span class="self">Self</span>::Item: PartialOrd&lt;I::Item&gt;,
    {
        <span class="self">self</span>.partial_cmp(other) == <span class="prelude-val">Some</span>(Ordering::Less)
    }

    <span class="doccomment">/// Determines if the elements of this `ParallelIterator`
    /// are less or equal to those of another.
    </span><span class="kw">fn </span>le&lt;I&gt;(<span class="self">self</span>, other: I) -&gt; bool
    <span class="kw">where
        </span>I: IntoParallelIterator,
        I::Iter: IndexedParallelIterator,
        <span class="self">Self</span>::Item: PartialOrd&lt;I::Item&gt;,
    {
        <span class="kw">let </span>ord = <span class="self">self</span>.partial_cmp(other);
        ord == <span class="prelude-val">Some</span>(Ordering::Equal) || ord == <span class="prelude-val">Some</span>(Ordering::Less)
    }

    <span class="doccomment">/// Determines if the elements of this `ParallelIterator`
    /// are lexicographically greater than those of another.
    </span><span class="kw">fn </span>gt&lt;I&gt;(<span class="self">self</span>, other: I) -&gt; bool
    <span class="kw">where
        </span>I: IntoParallelIterator,
        I::Iter: IndexedParallelIterator,
        <span class="self">Self</span>::Item: PartialOrd&lt;I::Item&gt;,
    {
        <span class="self">self</span>.partial_cmp(other) == <span class="prelude-val">Some</span>(Ordering::Greater)
    }

    <span class="doccomment">/// Determines if the elements of this `ParallelIterator`
    /// are less or equal to those of another.
    </span><span class="kw">fn </span>ge&lt;I&gt;(<span class="self">self</span>, other: I) -&gt; bool
    <span class="kw">where
        </span>I: IntoParallelIterator,
        I::Iter: IndexedParallelIterator,
        <span class="self">Self</span>::Item: PartialOrd&lt;I::Item&gt;,
    {
        <span class="kw">let </span>ord = <span class="self">self</span>.partial_cmp(other);
        ord == <span class="prelude-val">Some</span>(Ordering::Equal) || ord == <span class="prelude-val">Some</span>(Ordering::Greater)
    }

    <span class="doccomment">/// Yields an index along with each item.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let chars = vec!['a', 'b', 'c'];
    /// let result: Vec&lt;_&gt; = chars
    ///     .into_par_iter()
    ///     .enumerate()
    ///     .collect();
    ///
    /// assert_eq!(result, [(0, 'a'), (1, 'b'), (2, 'c')]);
    /// ```
    </span><span class="kw">fn </span>enumerate(<span class="self">self</span>) -&gt; Enumerate&lt;<span class="self">Self</span>&gt; {
        Enumerate::new(<span class="self">self</span>)
    }

    <span class="doccomment">/// Creates an iterator that steps by the given amount
    ///
    /// # Examples
    ///
    /// ```
    ///use rayon::prelude::*;
    ///
    /// let range = (3..10);
    /// let result: Vec&lt;i32&gt; = range
    ///    .into_par_iter()
    ///    .step_by(3)
    ///    .collect();
    ///
    /// assert_eq!(result, [3, 6, 9])
    /// ```
    </span><span class="kw">fn </span>step_by(<span class="self">self</span>, step: usize) -&gt; StepBy&lt;<span class="self">Self</span>&gt; {
        StepBy::new(<span class="self">self</span>, step)
    }

    <span class="doccomment">/// Creates an iterator that skips the first `n` elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let result: Vec&lt;_&gt; = (0..100)
    ///     .into_par_iter()
    ///     .skip(95)
    ///     .collect();
    ///
    /// assert_eq!(result, [95, 96, 97, 98, 99]);
    /// ```
    </span><span class="kw">fn </span>skip(<span class="self">self</span>, n: usize) -&gt; Skip&lt;<span class="self">Self</span>&gt; {
        Skip::new(<span class="self">self</span>, n)
    }

    <span class="doccomment">/// Creates an iterator that yields the first `n` elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let result: Vec&lt;_&gt; = (0..100)
    ///     .into_par_iter()
    ///     .take(5)
    ///     .collect();
    ///
    /// assert_eq!(result, [0, 1, 2, 3, 4]);
    /// ```
    </span><span class="kw">fn </span>take(<span class="self">self</span>, n: usize) -&gt; Take&lt;<span class="self">Self</span>&gt; {
        Take::new(<span class="self">self</span>, n)
    }

    <span class="doccomment">/// Searches for **some** item in the parallel iterator that
    /// matches the given predicate, and returns its index.  Like
    /// `ParallelIterator::find_any`, the parallel search will not
    /// necessarily find the **first** match, and once a match is
    /// found we'll attempt to stop processing any more.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [1, 2, 3, 3];
    ///
    /// let i = a.par_iter().position_any(|&amp;x| x == 3).expect("found");
    /// assert!(i == 2 || i == 3);
    ///
    /// assert_eq!(a.par_iter().position_any(|&amp;x| x == 100), None);
    /// ```
    </span><span class="kw">fn </span>position_any&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt;
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        <span class="attr">#[inline]
        </span><span class="kw">fn </span>check(<span class="kw-2">&amp;</span>(<span class="kw">_</span>, p): <span class="kw-2">&amp;</span>(usize, bool)) -&gt; bool {
            p
        }

        <span class="kw">let </span>(i, <span class="kw">_</span>) = <span class="self">self</span>.map(predicate).enumerate().find_any(check)<span class="question-mark">?</span>;
        <span class="prelude-val">Some</span>(i)
    }

    <span class="doccomment">/// Searches for the sequentially **first** item in the parallel iterator
    /// that matches the given predicate, and returns its index.
    ///
    /// Like `ParallelIterator::find_first`, once a match is found,
    /// all attempts to the right of the match will be stopped, while
    /// attempts to the left must continue in case an earlier match
    /// is found.
    ///
    /// Note that not all parallel iterators have a useful order, much like
    /// sequential `HashMap` iteration, so "first" may be nebulous.  If you
    /// just want the first match that discovered anywhere in the iterator,
    /// `position_any` is a better choice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [1, 2, 3, 3];
    ///
    /// assert_eq!(a.par_iter().position_first(|&amp;x| x == 3), Some(2));
    ///
    /// assert_eq!(a.par_iter().position_first(|&amp;x| x == 100), None);
    /// ```
    </span><span class="kw">fn </span>position_first&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt;
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        <span class="attr">#[inline]
        </span><span class="kw">fn </span>check(<span class="kw-2">&amp;</span>(<span class="kw">_</span>, p): <span class="kw-2">&amp;</span>(usize, bool)) -&gt; bool {
            p
        }

        <span class="kw">let </span>(i, <span class="kw">_</span>) = <span class="self">self</span>.map(predicate).enumerate().find_first(check)<span class="question-mark">?</span>;
        <span class="prelude-val">Some</span>(i)
    }

    <span class="doccomment">/// Searches for the sequentially **last** item in the parallel iterator
    /// that matches the given predicate, and returns its index.
    ///
    /// Like `ParallelIterator::find_last`, once a match is found,
    /// all attempts to the left of the match will be stopped, while
    /// attempts to the right must continue in case a later match
    /// is found.
    ///
    /// Note that not all parallel iterators have a useful order, much like
    /// sequential `HashMap` iteration, so "last" may be nebulous.  When the
    /// order doesn't actually matter to you, `position_any` is a better
    /// choice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let a = [1, 2, 3, 3];
    ///
    /// assert_eq!(a.par_iter().position_last(|&amp;x| x == 3), Some(3));
    ///
    /// assert_eq!(a.par_iter().position_last(|&amp;x| x == 100), None);
    /// ```
    </span><span class="kw">fn </span>position_last&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt;
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        <span class="attr">#[inline]
        </span><span class="kw">fn </span>check(<span class="kw-2">&amp;</span>(<span class="kw">_</span>, p): <span class="kw-2">&amp;</span>(usize, bool)) -&gt; bool {
            p
        }

        <span class="kw">let </span>(i, <span class="kw">_</span>) = <span class="self">self</span>.map(predicate).enumerate().find_last(check)<span class="question-mark">?</span>;
        <span class="prelude-val">Some</span>(i)
    }

    <span class="attr">#[doc(hidden)]
    #[deprecated(
        note = <span class="string">"parallel `position` does not search in order -- use `position_any`, \\
                `position_first`, or `position_last`"
    </span>)]
    </span><span class="kw">fn </span>position&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; <span class="prelude-ty">Option</span>&lt;usize&gt;
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        <span class="self">self</span>.position_any(predicate)
    }

    <span class="doccomment">/// Searches for items in the parallel iterator that match the given
    /// predicate, and returns their indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    ///
    /// // Find the positions of primes congruent to 1 modulo 6
    /// let p1mod6: Vec&lt;_&gt; = primes.par_iter().positions(|&amp;p| p % 6 == 1).collect();
    /// assert_eq!(p1mod6, [3, 5, 7]); // primes 7, 13, and 19
    ///
    /// // Find the positions of primes congruent to 5 modulo 6
    /// let p5mod6: Vec&lt;_&gt; = primes.par_iter().positions(|&amp;p| p % 6 == 5).collect();
    /// assert_eq!(p5mod6, [2, 4, 6, 8, 9]); // primes 5, 11, 17, 23, and 29
    /// ```
    </span><span class="kw">fn </span>positions&lt;P&gt;(<span class="self">self</span>, predicate: P) -&gt; Positions&lt;<span class="self">Self</span>, P&gt;
    <span class="kw">where
        </span>P: Fn(<span class="self">Self</span>::Item) -&gt; bool + Sync + Send,
    {
        Positions::new(<span class="self">self</span>, predicate)
    }

    <span class="doccomment">/// Produces a new iterator with the elements of this iterator in
    /// reverse order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let result: Vec&lt;_&gt; = (0..5)
    ///     .into_par_iter()
    ///     .rev()
    ///     .collect();
    ///
    /// assert_eq!(result, [4, 3, 2, 1, 0]);
    /// ```
    </span><span class="kw">fn </span>rev(<span class="self">self</span>) -&gt; Rev&lt;<span class="self">Self</span>&gt; {
        Rev::new(<span class="self">self</span>)
    }

    <span class="doccomment">/// Sets the minimum length of iterators desired to process in each
    /// rayon job.  Rayon will not split any smaller than this length, but
    /// of course an iterator could already be smaller to begin with.
    ///
    /// Producers like `zip` and `interleave` will use greater of the two
    /// minimums.
    /// Chained iterators and iterators inside `flat_map` may each use
    /// their own minimum length.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let min = (0..1_000_000)
    ///     .into_par_iter()
    ///     .with_min_len(1234)
    ///     .fold(|| 0, |acc, _| acc + 1) // count how many are in this segment
    ///     .min().unwrap();
    ///
    /// assert!(min &gt;= 1234);
    /// ```
    </span><span class="kw">fn </span>with_min_len(<span class="self">self</span>, min: usize) -&gt; MinLen&lt;<span class="self">Self</span>&gt; {
        MinLen::new(<span class="self">self</span>, min)
    }

    <span class="doccomment">/// Sets the maximum length of iterators desired to process in each
    /// rayon job.  Rayon will try to split at least below this length,
    /// unless that would put it below the length from `with_min_len()`.
    /// For example, given min=10 and max=15, a length of 16 will not be
    /// split any further.
    ///
    /// Producers like `zip` and `interleave` will use lesser of the two
    /// maximums.
    /// Chained iterators and iterators inside `flat_map` may each use
    /// their own maximum length.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let max = (0..1_000_000)
    ///     .into_par_iter()
    ///     .with_max_len(1234)
    ///     .fold(|| 0, |acc, _| acc + 1) // count how many are in this segment
    ///     .max().unwrap();
    ///
    /// assert!(max &lt;= 1234);
    /// ```
    </span><span class="kw">fn </span>with_max_len(<span class="self">self</span>, max: usize) -&gt; MaxLen&lt;<span class="self">Self</span>&gt; {
        MaxLen::new(<span class="self">self</span>, max)
    }

    <span class="doccomment">/// Produces an exact count of how many items this iterator will
    /// produce, presuming no panic occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let par_iter = (0..100).into_par_iter().zip(vec![0; 10]);
    /// assert_eq!(par_iter.len(), 10);
    ///
    /// let vec: Vec&lt;_&gt; = par_iter.collect();
    /// assert_eq!(vec.len(), 10);
    /// ```
    </span><span class="kw">fn </span>len(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize;

    <span class="doccomment">/// Internal method used to define the behavior of this parallel
    /// iterator. You should not need to call this directly.
    ///
    /// This method causes the iterator `self` to start producing
    /// items and to feed them to the consumer `consumer` one by one.
    /// It may split the consumer before doing so to create the
    /// opportunity to produce in parallel. If a split does happen, it
    /// will inform the consumer of the index where the split should
    /// occur (unlike `ParallelIterator::drive_unindexed()`).
    ///
    /// See the [README] for more details on the internals of parallel
    /// iterators.
    ///
    /// [README]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md
    </span><span class="kw">fn </span>drive&lt;C: Consumer&lt;<span class="self">Self</span>::Item&gt;&gt;(<span class="self">self</span>, consumer: C) -&gt; C::Result;

    <span class="doccomment">/// Internal method used to define the behavior of this parallel
    /// iterator. You should not need to call this directly.
    ///
    /// This method converts the iterator into a producer P and then
    /// invokes `callback.callback()` with P. Note that the type of
    /// this producer is not defined as part of the API, since
    /// `callback` must be defined generically for all producers. This
    /// allows the producer type to contain references; it also means
    /// that parallel iterators can adjust that type without causing a
    /// breaking change.
    ///
    /// See the [README] for more details on the internals of parallel
    /// iterators.
    ///
    /// [README]: https://github.com/rayon-rs/rayon/blob/main/src/iter/plumbing/README.md
    </span><span class="kw">fn </span>with_producer&lt;CB: ProducerCallback&lt;<span class="self">Self</span>::Item&gt;&gt;(<span class="self">self</span>, callback: CB) -&gt; CB::Output;
}

<span class="doccomment">/// `FromParallelIterator` implements the creation of a collection
/// from a [`ParallelIterator`]. By implementing
/// `FromParallelIterator` for a given type, you define how it will be
/// created from an iterator.
///
/// `FromParallelIterator` is used through [`ParallelIterator`]'s [`collect()`] method.
///
/// [`ParallelIterator`]: trait.ParallelIterator.html
/// [`collect()`]: trait.ParallelIterator.html#method.collect
///
/// # Examples
///
/// Implementing `FromParallelIterator` for your type:
///
/// ```
/// use rayon::prelude::*;
/// use std::mem;
///
/// struct BlackHole {
///     mass: usize,
/// }
///
/// impl&lt;T: Send&gt; FromParallelIterator&lt;T&gt; for BlackHole {
///     fn from_par_iter&lt;I&gt;(par_iter: I) -&gt; Self
///         where I: IntoParallelIterator&lt;Item = T&gt;
///     {
///         let par_iter = par_iter.into_par_iter();
///         BlackHole {
///             mass: par_iter.count() * mem::size_of::&lt;T&gt;(),
///         }
///     }
/// }
///
/// let bh: BlackHole = (0i32..1000).into_par_iter().collect();
/// assert_eq!(bh.mass, 4000);
/// ```
</span><span class="kw">pub trait </span>FromParallelIterator&lt;T&gt;
<span class="kw">where
    </span>T: Send,
{
    <span class="doccomment">/// Creates an instance of the collection from the parallel iterator `par_iter`.
    ///
    /// If your collection is not naturally parallel, the easiest (and
    /// fastest) way to do this is often to collect `par_iter` into a
    /// [`LinkedList`] (via [`collect_vec_list`]) or another intermediate
    /// data structure and then sequentially extend your collection. However,
    /// a more 'native' technique is to use the [`par_iter.fold`] or
    /// [`par_iter.fold_with`] methods to create the collection.
    /// Alternatively, if your collection is 'natively' parallel, you
    /// can use `par_iter.for_each` to process each element in turn.
    ///
    /// [`LinkedList`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
    /// [`collect_vec_list`]: ParallelIterator::collect_vec_list
    /// [`par_iter.fold`]: trait.ParallelIterator.html#method.fold
    /// [`par_iter.fold_with`]: trait.ParallelIterator.html#method.fold_with
    /// [`par_iter.for_each`]: trait.ParallelIterator.html#method.for_each
    </span><span class="kw">fn </span>from_par_iter&lt;I&gt;(par_iter: I) -&gt; <span class="self">Self
    </span><span class="kw">where
        </span>I: IntoParallelIterator&lt;Item = T&gt;;
}

<span class="doccomment">/// `ParallelExtend` extends an existing collection with items from a [`ParallelIterator`].
///
/// [`ParallelIterator`]: trait.ParallelIterator.html
///
/// # Examples
///
/// Implementing `ParallelExtend` for your type:
///
/// ```
/// use rayon::prelude::*;
/// use std::mem;
///
/// struct BlackHole {
///     mass: usize,
/// }
///
/// impl&lt;T: Send&gt; ParallelExtend&lt;T&gt; for BlackHole {
///     fn par_extend&lt;I&gt;(&amp;mut self, par_iter: I)
///         where I: IntoParallelIterator&lt;Item = T&gt;
///     {
///         let par_iter = par_iter.into_par_iter();
///         self.mass += par_iter.count() * mem::size_of::&lt;T&gt;();
///     }
/// }
///
/// let mut bh = BlackHole { mass: 0 };
/// bh.par_extend(0i32..1000);
/// assert_eq!(bh.mass, 4000);
/// bh.par_extend(0i64..10);
/// assert_eq!(bh.mass, 4080);
/// ```
</span><span class="kw">pub trait </span>ParallelExtend&lt;T&gt;
<span class="kw">where
    </span>T: Send,
{
    <span class="doccomment">/// Extends an instance of the collection with the elements drawn
    /// from the parallel iterator `par_iter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let mut vec = vec![];
    /// vec.par_extend(0..5);
    /// vec.par_extend((0..5).into_par_iter().map(|i| i * i));
    /// assert_eq!(vec, [0, 1, 2, 3, 4, 0, 1, 4, 9, 16]);
    /// ```
    </span><span class="kw">fn </span>par_extend&lt;I&gt;(<span class="kw-2">&amp;mut </span><span class="self">self</span>, par_iter: I)
    <span class="kw">where
        </span>I: IntoParallelIterator&lt;Item = T&gt;;
}

<span class="doccomment">/// `ParallelDrainFull` creates a parallel iterator that moves all items
/// from a collection while retaining the original capacity.
///
/// Types which are indexable typically implement [`ParallelDrainRange`]
/// instead, where you can drain fully with `par_drain(..)`.
///
/// [`ParallelDrainRange`]: trait.ParallelDrainRange.html
</span><span class="kw">pub trait </span>ParallelDrainFull {
    <span class="doccomment">/// The draining parallel iterator type that will be created.
    </span><span class="kw">type </span>Iter: ParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;;

    <span class="doccomment">/// The type of item that the parallel iterator will produce.
    /// This is usually the same as `IntoParallelIterator::Item`.
    </span><span class="kw">type </span>Item: Send;

    <span class="doccomment">/// Returns a draining parallel iterator over an entire collection.
    ///
    /// When the iterator is dropped, all items are removed, even if the
    /// iterator was not fully consumed. If the iterator is leaked, for example
    /// using `std::mem::forget`, it is unspecified how many items are removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// use std::collections::{BinaryHeap, HashSet};
    ///
    /// let squares: HashSet&lt;i32&gt; = (0..10).map(|x| x * x).collect();
    ///
    /// let mut heap: BinaryHeap&lt;_&gt; = squares.iter().copied().collect();
    /// assert_eq!(
    ///     // heaps are drained in arbitrary order
    ///     heap.par_drain()
    ///         .inspect(|x| assert!(squares.contains(x)))
    ///         .count(),
    ///     squares.len(),
    /// );
    /// assert!(heap.is_empty());
    /// assert!(heap.capacity() &gt;= squares.len());
    /// ```
    </span><span class="kw">fn </span>par_drain(<span class="self">self</span>) -&gt; <span class="self">Self</span>::Iter;
}

<span class="doccomment">/// `ParallelDrainRange` creates a parallel iterator that moves a range of items
/// from a collection while retaining the original capacity.
///
/// Types which are not indexable may implement [`ParallelDrainFull`] instead.
///
/// [`ParallelDrainFull`]: trait.ParallelDrainFull.html
</span><span class="kw">pub trait </span>ParallelDrainRange&lt;Idx = usize&gt; {
    <span class="doccomment">/// The draining parallel iterator type that will be created.
    </span><span class="kw">type </span>Iter: ParallelIterator&lt;Item = <span class="self">Self</span>::Item&gt;;

    <span class="doccomment">/// The type of item that the parallel iterator will produce.
    /// This is usually the same as `IntoParallelIterator::Item`.
    </span><span class="kw">type </span>Item: Send;

    <span class="doccomment">/// Returns a draining parallel iterator over a range of the collection.
    ///
    /// When the iterator is dropped, all items in the range are removed, even
    /// if the iterator was not fully consumed. If the iterator is leaked, for
    /// example using `std::mem::forget`, it is unspecified how many items are
    /// removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let squares: Vec&lt;i32&gt; = (0..10).map(|x| x * x).collect();
    ///
    /// println!("RangeFull");
    /// let mut vec = squares.clone();
    /// assert!(vec.par_drain(..)
    ///            .eq(squares.par_iter().copied()));
    /// assert!(vec.is_empty());
    /// assert!(vec.capacity() &gt;= squares.len());
    ///
    /// println!("RangeFrom");
    /// let mut vec = squares.clone();
    /// assert!(vec.par_drain(5..)
    ///            .eq(squares[5..].par_iter().copied()));
    /// assert_eq!(&amp;vec[..], &amp;squares[..5]);
    /// assert!(vec.capacity() &gt;= squares.len());
    ///
    /// println!("RangeTo");
    /// let mut vec = squares.clone();
    /// assert!(vec.par_drain(..5)
    ///            .eq(squares[..5].par_iter().copied()));
    /// assert_eq!(&amp;vec[..], &amp;squares[5..]);
    /// assert!(vec.capacity() &gt;= squares.len());
    ///
    /// println!("RangeToInclusive");
    /// let mut vec = squares.clone();
    /// assert!(vec.par_drain(..=5)
    ///            .eq(squares[..=5].par_iter().copied()));
    /// assert_eq!(&amp;vec[..], &amp;squares[6..]);
    /// assert!(vec.capacity() &gt;= squares.len());
    ///
    /// println!("Range");
    /// let mut vec = squares.clone();
    /// assert!(vec.par_drain(3..7)
    ///            .eq(squares[3..7].par_iter().copied()));
    /// assert_eq!(&amp;vec[..3], &amp;squares[..3]);
    /// assert_eq!(&amp;vec[3..], &amp;squares[7..]);
    /// assert!(vec.capacity() &gt;= squares.len());
    ///
    /// println!("RangeInclusive");
    /// let mut vec = squares.clone();
    /// assert!(vec.par_drain(3..=7)
    ///            .eq(squares[3..=7].par_iter().copied()));
    /// assert_eq!(&amp;vec[..3], &amp;squares[..3]);
    /// assert_eq!(&amp;vec[3..], &amp;squares[8..]);
    /// assert!(vec.capacity() &gt;= squares.len());
    /// ```
    </span><span class="kw">fn </span>par_drain&lt;R: RangeBounds&lt;Idx&gt;&gt;(<span class="self">self</span>, range: R) -&gt; <span class="self">Self</span>::Iter;
}

<span class="doccomment">/// We hide the `Try` trait in a private module, as it's only meant to be a
/// stable clone of the standard library's `Try` trait, as yet unstable.
</span><span class="kw">mod </span>private {
    <span class="kw">use </span>std::convert::Infallible;
    <span class="kw">use </span>std::ops::ControlFlow::{<span class="self">self</span>, Break, Continue};
    <span class="kw">use </span>std::task::Poll;

    <span class="doccomment">/// Clone of `std::ops::Try`.
    ///
    /// Implementing this trait is not permitted outside of `rayon`.
    </span><span class="kw">pub trait </span>Try {
        <span class="macro">private_decl!</span> {}

        <span class="kw">type </span>Output;
        <span class="kw">type </span>Residual;

        <span class="kw">fn </span>from_output(output: <span class="self">Self</span>::Output) -&gt; <span class="self">Self</span>;

        <span class="kw">fn </span>from_residual(residual: <span class="self">Self</span>::Residual) -&gt; <span class="self">Self</span>;

        <span class="kw">fn </span>branch(<span class="self">self</span>) -&gt; ControlFlow&lt;<span class="self">Self</span>::Residual, <span class="self">Self</span>::Output&gt;;
    }

    <span class="kw">impl</span>&lt;B, C&gt; Try <span class="kw">for </span>ControlFlow&lt;B, C&gt; {
        <span class="macro">private_impl!</span> {}

        <span class="kw">type </span>Output = C;
        <span class="kw">type </span>Residual = ControlFlow&lt;B, Infallible&gt;;

        <span class="kw">fn </span>from_output(output: <span class="self">Self</span>::Output) -&gt; <span class="self">Self </span>{
            Continue(output)
        }

        <span class="kw">fn </span>from_residual(residual: <span class="self">Self</span>::Residual) -&gt; <span class="self">Self </span>{
            <span class="kw">match </span>residual {
                Break(b) =&gt; Break(b),
                Continue(<span class="kw">_</span>) =&gt; <span class="macro">unreachable!</span>(),
            }
        }

        <span class="kw">fn </span>branch(<span class="self">self</span>) -&gt; ControlFlow&lt;<span class="self">Self</span>::Residual, <span class="self">Self</span>::Output&gt; {
            <span class="kw">match </span><span class="self">self </span>{
                Continue(c) =&gt; Continue(c),
                Break(b) =&gt; Break(Break(b)),
            }
        }
    }

    <span class="kw">impl</span>&lt;T&gt; Try <span class="kw">for </span><span class="prelude-ty">Option</span>&lt;T&gt; {
        <span class="macro">private_impl!</span> {}

        <span class="kw">type </span>Output = T;
        <span class="kw">type </span>Residual = <span class="prelude-ty">Option</span>&lt;Infallible&gt;;

        <span class="kw">fn </span>from_output(output: <span class="self">Self</span>::Output) -&gt; <span class="self">Self </span>{
            <span class="prelude-val">Some</span>(output)
        }

        <span class="kw">fn </span>from_residual(residual: <span class="self">Self</span>::Residual) -&gt; <span class="self">Self </span>{
            <span class="kw">match </span>residual {
                <span class="prelude-val">None </span>=&gt; <span class="prelude-val">None</span>,
                <span class="prelude-val">Some</span>(<span class="kw">_</span>) =&gt; <span class="macro">unreachable!</span>(),
            }
        }

        <span class="kw">fn </span>branch(<span class="self">self</span>) -&gt; ControlFlow&lt;<span class="self">Self</span>::Residual, <span class="self">Self</span>::Output&gt; {
            <span class="kw">match </span><span class="self">self </span>{
                <span class="prelude-val">Some</span>(c) =&gt; Continue(c),
                <span class="prelude-val">None </span>=&gt; Break(<span class="prelude-val">None</span>),
            }
        }
    }

    <span class="kw">impl</span>&lt;T, E&gt; Try <span class="kw">for </span><span class="prelude-ty">Result</span>&lt;T, E&gt; {
        <span class="macro">private_impl!</span> {}

        <span class="kw">type </span>Output = T;
        <span class="kw">type </span>Residual = <span class="prelude-ty">Result</span>&lt;Infallible, E&gt;;

        <span class="kw">fn </span>from_output(output: <span class="self">Self</span>::Output) -&gt; <span class="self">Self </span>{
            <span class="prelude-val">Ok</span>(output)
        }

        <span class="kw">fn </span>from_residual(residual: <span class="self">Self</span>::Residual) -&gt; <span class="self">Self </span>{
            <span class="kw">match </span>residual {
                <span class="prelude-val">Err</span>(e) =&gt; <span class="prelude-val">Err</span>(e),
                <span class="prelude-val">Ok</span>(<span class="kw">_</span>) =&gt; <span class="macro">unreachable!</span>(),
            }
        }

        <span class="kw">fn </span>branch(<span class="self">self</span>) -&gt; ControlFlow&lt;<span class="self">Self</span>::Residual, <span class="self">Self</span>::Output&gt; {
            <span class="kw">match </span><span class="self">self </span>{
                <span class="prelude-val">Ok</span>(c) =&gt; Continue(c),
                <span class="prelude-val">Err</span>(e) =&gt; Break(<span class="prelude-val">Err</span>(e)),
            }
        }
    }

    <span class="kw">impl</span>&lt;T, E&gt; Try <span class="kw">for </span>Poll&lt;<span class="prelude-ty">Result</span>&lt;T, E&gt;&gt; {
        <span class="macro">private_impl!</span> {}

        <span class="kw">type </span>Output = Poll&lt;T&gt;;
        <span class="kw">type </span>Residual = <span class="prelude-ty">Result</span>&lt;Infallible, E&gt;;

        <span class="kw">fn </span>from_output(output: <span class="self">Self</span>::Output) -&gt; <span class="self">Self </span>{
            output.map(<span class="prelude-val">Ok</span>)
        }

        <span class="kw">fn </span>from_residual(residual: <span class="self">Self</span>::Residual) -&gt; <span class="self">Self </span>{
            <span class="kw">match </span>residual {
                <span class="prelude-val">Err</span>(e) =&gt; Poll::Ready(<span class="prelude-val">Err</span>(e)),
                <span class="prelude-val">Ok</span>(<span class="kw">_</span>) =&gt; <span class="macro">unreachable!</span>(),
            }
        }

        <span class="kw">fn </span>branch(<span class="self">self</span>) -&gt; ControlFlow&lt;<span class="self">Self</span>::Residual, <span class="self">Self</span>::Output&gt; {
            <span class="kw">match </span><span class="self">self </span>{
                Poll::Pending =&gt; Continue(Poll::Pending),
                Poll::Ready(<span class="prelude-val">Ok</span>(c)) =&gt; Continue(Poll::Ready(c)),
                Poll::Ready(<span class="prelude-val">Err</span>(e)) =&gt; Break(<span class="prelude-val">Err</span>(e)),
            }
        }
    }

    <span class="kw">impl</span>&lt;T, E&gt; Try <span class="kw">for </span>Poll&lt;<span class="prelude-ty">Option</span>&lt;<span class="prelude-ty">Result</span>&lt;T, E&gt;&gt;&gt; {
        <span class="macro">private_impl!</span> {}

        <span class="kw">type </span>Output = Poll&lt;<span class="prelude-ty">Option</span>&lt;T&gt;&gt;;
        <span class="kw">type </span>Residual = <span class="prelude-ty">Result</span>&lt;Infallible, E&gt;;

        <span class="kw">fn </span>from_output(output: <span class="self">Self</span>::Output) -&gt; <span class="self">Self </span>{
            <span class="kw">match </span>output {
                Poll::Ready(o) =&gt; Poll::Ready(o.map(<span class="prelude-val">Ok</span>)),
                Poll::Pending =&gt; Poll::Pending,
            }
        }

        <span class="kw">fn </span>from_residual(residual: <span class="self">Self</span>::Residual) -&gt; <span class="self">Self </span>{
            <span class="kw">match </span>residual {
                <span class="prelude-val">Err</span>(e) =&gt; Poll::Ready(<span class="prelude-val">Some</span>(<span class="prelude-val">Err</span>(e))),
                <span class="prelude-val">Ok</span>(<span class="kw">_</span>) =&gt; <span class="macro">unreachable!</span>(),
            }
        }

        <span class="kw">fn </span>branch(<span class="self">self</span>) -&gt; ControlFlow&lt;<span class="self">Self</span>::Residual, <span class="self">Self</span>::Output&gt; {
            <span class="kw">match </span><span class="self">self </span>{
                Poll::Pending =&gt; Continue(Poll::Pending),
                Poll::Ready(<span class="prelude-val">None</span>) =&gt; Continue(Poll::Ready(<span class="prelude-val">None</span>)),
                Poll::Ready(<span class="prelude-val">Some</span>(<span class="prelude-val">Ok</span>(c))) =&gt; Continue(Poll::Ready(<span class="prelude-val">Some</span>(c))),
                Poll::Ready(<span class="prelude-val">Some</span>(<span class="prelude-val">Err</span>(e))) =&gt; Break(<span class="prelude-val">Err</span>(e)),
            }
        }
    }
}
</code></pre></div></section></main></body></html>