<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.37.0/src/runtime/runtime.rs`."><title>runtime.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="tokio" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../tokio/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">crate</span>::runtime::blocking::BlockingPool;
<span class="kw">use </span><span class="kw">crate</span>::runtime::scheduler::CurrentThread;
<span class="kw">use </span><span class="kw">crate</span>::runtime::{context, EnterGuard, Handle};
<span class="kw">use </span><span class="kw">crate</span>::task::JoinHandle;

<span class="kw">use </span>std::future::Future;
<span class="kw">use </span>std::time::Duration;

<span class="macro">cfg_rt_multi_thread!</span> {
    <span class="kw">use </span><span class="kw">crate</span>::runtime::Builder;
    <span class="kw">use </span><span class="kw">crate</span>::runtime::scheduler::MultiThread;

    <span class="macro">cfg_unstable!</span> {
        <span class="kw">use </span><span class="kw">crate</span>::runtime::scheduler::MultiThreadAlt;
    }
}

<span class="doccomment">/// The Tokio runtime.
///
/// The runtime provides an I/O driver, task scheduler, [timer], and
/// blocking pool, necessary for running asynchronous tasks.
///
/// Instances of `Runtime` can be created using [`new`], or [`Builder`].
/// However, most users will use the [`#[tokio::main]`][main] annotation on
/// their entry point instead.
///
/// See [module level][mod] documentation for more details.
///
/// # Shutdown
///
/// Shutting down the runtime is done by dropping the value, or calling
/// [`shutdown_background`] or [`shutdown_timeout`].
///
/// Tasks spawned through [`Runtime::spawn`] keep running until they yield.
/// Then they are dropped. They are not *guaranteed* to run to completion, but
/// *might* do so if they do not yield until completion.
///
/// Blocking functions spawned through [`Runtime::spawn_blocking`] keep running
/// until they return.
///
/// The thread initiating the shutdown blocks until all spawned work has been
/// stopped. This can take an indefinite amount of time. The `Drop`
/// implementation waits forever for this.
///
/// The [`shutdown_background`] and [`shutdown_timeout`] methods can be used if
/// waiting forever is undesired. When the timeout is reached, spawned work that
/// did not stop in time and threads running it are leaked. The work continues
/// to run until one of the stopping conditions is fulfilled, but the thread
/// initiating the shutdown is unblocked.
///
/// Once the runtime has been dropped, any outstanding I/O resources bound to
/// it will no longer function. Calling any method on them will result in an
/// error.
///
/// # Sharing
///
/// There are several ways to establish shared access to a Tokio runtime:
///
///  * Using an &lt;code&gt;[Arc]\&lt;Runtime&gt;&lt;/code&gt;.
///  * Using a [`Handle`].
///  * Entering the runtime context.
///
/// Using an &lt;code&gt;[Arc]\&lt;Runtime&gt;&lt;/code&gt; or [`Handle`] allows you to do various
/// things with the runtime such as spawning new tasks or entering the runtime
/// context. Both types can be cloned to create a new handle that allows access
/// to the same runtime. By passing clones into different tasks or threads, you
/// will be able to access the runtime from those tasks or threads.
///
/// The difference between &lt;code&gt;[Arc]\&lt;Runtime&gt;&lt;/code&gt; and [`Handle`] is that
/// an &lt;code&gt;[Arc]\&lt;Runtime&gt;&lt;/code&gt; will prevent the runtime from shutting down,
/// whereas a [`Handle`] does not prevent that. This is because shutdown of the
/// runtime happens when the destructor of the `Runtime` object runs.
///
/// Calls to [`shutdown_background`] and [`shutdown_timeout`] require exclusive
/// ownership of the `Runtime` type. When using an &lt;code&gt;[Arc]\&lt;Runtime&gt;&lt;/code&gt;,
/// this can be achieved via [`Arc::try_unwrap`] when only one strong count
/// reference is left over.
///
/// The runtime context is entered using the [`Runtime::enter`] or
/// [`Handle::enter`] methods, which use a thread-local variable to store the
/// current runtime. Whenever you are inside the runtime context, methods such
/// as [`tokio::spawn`] will use the runtime whose context you are inside.
///
/// [timer]: crate::time
/// [mod]: index.html
/// [`new`]: method@Self::new
/// [`Builder`]: struct@Builder
/// [`Handle`]: struct@Handle
/// [main]: macro@crate::main
/// [`tokio::spawn`]: crate::spawn
/// [`Arc::try_unwrap`]: std::sync::Arc::try_unwrap
/// [Arc]: std::sync::Arc
/// [`shutdown_background`]: method@Runtime::shutdown_background
/// [`shutdown_timeout`]: method@Runtime::shutdown_timeout
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>Runtime {
    <span class="doccomment">/// Task scheduler
    </span>scheduler: Scheduler,

    <span class="doccomment">/// Handle to runtime, also contains driver handles
    </span>handle: Handle,

    <span class="doccomment">/// Blocking pool handle, used to signal shutdown
    </span>blocking_pool: BlockingPool,
}

<span class="doccomment">/// The flavor of a `Runtime`.
///
/// This is the return type for [`Handle::runtime_flavor`](crate::runtime::Handle::runtime_flavor()).
</span><span class="attr">#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
</span><span class="kw">pub enum </span>RuntimeFlavor {
    <span class="doccomment">/// The flavor that executes all tasks on the current thread.
    </span>CurrentThread,
    <span class="doccomment">/// The flavor that executes tasks across multiple threads.
    </span>MultiThread,
    <span class="doccomment">/// The flavor that executes tasks across multiple threads.
    </span><span class="attr">#[cfg(tokio_unstable)]
    </span>MultiThreadAlt,
}

<span class="doccomment">/// The runtime scheduler is either a multi-thread or a current-thread executor.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">enum </span>Scheduler {
    <span class="doccomment">/// Execute all tasks on the current-thread.
    </span>CurrentThread(CurrentThread),

    <span class="doccomment">/// Execute tasks across multiple threads.
    </span><span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
    </span>MultiThread(MultiThread),

    <span class="doccomment">/// Execute tasks across multiple threads.
    </span><span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
    </span>MultiThreadAlt(MultiThreadAlt),
}

<span class="kw">impl </span>Runtime {
    <span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>from_parts(
        scheduler: Scheduler,
        handle: Handle,
        blocking_pool: BlockingPool,
    ) -&gt; Runtime {
        Runtime {
            scheduler,
            handle,
            blocking_pool,
        }
    }

    <span class="macro">cfg_not_wasi!</span> {
        <span class="doccomment">/// Creates a new runtime instance with default configuration values.
        ///
        /// This results in the multi threaded scheduler, I/O driver, and time driver being
        /// initialized.
        ///
        /// Most applications will not need to call this function directly. Instead,
        /// they will use the  [`#[tokio::main]` attribute][main]. When a more complex
        /// configuration is necessary, the [runtime builder] may be used.
        ///
        /// See [module level][mod] documentation for more details.
        ///
        /// # Examples
        ///
        /// Creating a new `Runtime` with default configuration values.
        ///
        /// ```
        /// use tokio::runtime::Runtime;
        ///
        /// let rt = Runtime::new()
        ///     .unwrap();
        ///
        /// // Use the runtime...
        /// ```
        ///
        /// [mod]: index.html
        /// [main]: ../attr.main.html
        /// [threaded scheduler]: index.html#threaded-scheduler
        /// [runtime builder]: crate::runtime::Builder
        </span><span class="attr">#[cfg(feature = <span class="string">"rt-multi-thread"</span>)]
        #[cfg_attr(docsrs, doc(cfg(feature = <span class="string">"rt-multi-thread"</span>)))]
        </span><span class="kw">pub fn </span>new() -&gt; std::io::Result&lt;Runtime&gt; {
            Builder::new_multi_thread().enable_all().build()
        }
    }

    <span class="doccomment">/// Returns a handle to the runtime's spawner.
    ///
    /// The returned handle can be used to spawn tasks that run on this runtime, and can
    /// be cloned to allow moving the `Handle` to other threads.
    ///
    /// Calling [`Handle::block_on`] on a handle to a `current_thread` runtime is error-prone.
    /// Refer to the documentation of [`Handle::block_on`] for more.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    ///
    /// let rt = Runtime::new()
    ///     .unwrap();
    ///
    /// let handle = rt.handle();
    ///
    /// // Use the handle...
    /// ```
    </span><span class="kw">pub fn </span>handle(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>Handle {
        <span class="kw-2">&amp;</span><span class="self">self</span>.handle
    }

    <span class="doccomment">/// Spawns a future onto the Tokio runtime.
    ///
    /// This spawns the given future onto the runtime's executor, usually a
    /// thread pool. The thread pool is then responsible for polling the future
    /// until it completes.
    ///
    /// The provided future will start running in the background immediately
    /// when `spawn` is called, even if you don't await the returned
    /// `JoinHandle`.
    ///
    /// See [module level][mod] documentation for more details.
    ///
    /// [mod]: index.html
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    ///
    /// # fn dox() {
    /// // Create the runtime
    /// let rt = Runtime::new().unwrap();
    ///
    /// // Spawn a future onto the runtime
    /// rt.spawn(async {
    ///     println!("now running on a worker thread");
    /// });
    /// # }
    /// ```
    </span><span class="attr">#[track_caller]
    </span><span class="kw">pub fn </span>spawn&lt;F&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, future: F) -&gt; JoinHandle&lt;F::Output&gt;
    <span class="kw">where
        </span>F: Future + Send + <span class="lifetime">'static</span>,
        F::Output: Send + <span class="lifetime">'static</span>,
    {
        <span class="self">self</span>.handle.spawn(future)
    }

    <span class="doccomment">/// Runs the provided function on an executor dedicated to blocking operations.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    ///
    /// # fn dox() {
    /// // Create the runtime
    /// let rt = Runtime::new().unwrap();
    ///
    /// // Spawn a blocking function onto the runtime
    /// rt.spawn_blocking(|| {
    ///     println!("now running on a worker thread");
    /// });
    /// # }
    /// ```
    </span><span class="attr">#[track_caller]
    </span><span class="kw">pub fn </span>spawn_blocking&lt;F, R&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, func: F) -&gt; JoinHandle&lt;R&gt;
    <span class="kw">where
        </span>F: FnOnce() -&gt; R + Send + <span class="lifetime">'static</span>,
        R: Send + <span class="lifetime">'static</span>,
    {
        <span class="self">self</span>.handle.spawn_blocking(func)
    }

    <span class="doccomment">/// Runs a future to completion on the Tokio runtime. This is the
    /// runtime's entry point.
    ///
    /// This runs the given future on the current thread, blocking until it is
    /// complete, and yielding its resolved result. Any tasks or timers
    /// which the future spawns internally will be executed on the runtime.
    ///
    /// # Non-worker future
    ///
    /// Note that the future required by this function does not run as a
    /// worker. The expectation is that other tasks are spawned by the future here.
    /// Awaiting on other futures from the future provided here will not
    /// perform as fast as those spawned as workers.
    ///
    /// # Multi thread scheduler
    ///
    /// When the multi thread scheduler is used this will allow futures
    /// to run within the io driver and timer context of the overall runtime.
    ///
    /// Any spawned tasks will continue running after `block_on` returns.
    ///
    /// # Current thread scheduler
    ///
    /// When the current thread scheduler is enabled `block_on`
    /// can be called concurrently from multiple threads. The first call
    /// will take ownership of the io and timer drivers. This means
    /// other threads which do not own the drivers will hook into that one.
    /// When the first `block_on` completes, other threads will be able to
    /// "steal" the driver to allow continued execution of their futures.
    ///
    /// Any spawned tasks will be suspended after `block_on` returns. Calling
    /// `block_on` again will resume previously spawned tasks.
    ///
    /// # Panics
    ///
    /// This function panics if the provided future panics, or if called within an
    /// asynchronous execution context.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::runtime::Runtime;
    ///
    /// // Create the runtime
    /// let rt  = Runtime::new().unwrap();
    ///
    /// // Execute the future, blocking the current thread until completion
    /// rt.block_on(async {
    ///     println!("hello");
    /// });
    /// ```
    ///
    /// [handle]: fn@Handle::block_on
    </span><span class="attr">#[track_caller]
    </span><span class="kw">pub fn </span>block_on&lt;F: Future&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, future: F) -&gt; F::Output {
        <span class="attr">#[cfg(all(
            tokio_unstable,
            tokio_taskdump,
            feature = <span class="string">"rt"</span>,
            target_os = <span class="string">"linux"</span>,
            any(target_arch = <span class="string">"aarch64"</span>, target_arch = <span class="string">"x86"</span>, target_arch = <span class="string">"x86_64"</span>)
        ))]
        </span><span class="kw">let </span>future = <span class="kw">super</span>::task::trace::Trace::root(future);

        <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"tracing"</span>))]
        </span><span class="kw">let </span>future = <span class="kw">crate</span>::util::trace::task(
            future,
            <span class="string">"block_on"</span>,
            <span class="prelude-val">None</span>,
            <span class="kw">crate</span>::runtime::task::Id::next().as_u64(),
        );

        <span class="kw">let </span>_enter = <span class="self">self</span>.enter();

        <span class="kw">match </span><span class="kw-2">&amp;</span><span class="self">self</span>.scheduler {
            Scheduler::CurrentThread(exec) =&gt; exec.block_on(<span class="kw-2">&amp;</span><span class="self">self</span>.handle.inner, future),
            <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
            </span>Scheduler::MultiThread(exec) =&gt; exec.block_on(<span class="kw-2">&amp;</span><span class="self">self</span>.handle.inner, future),
            <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
            </span>Scheduler::MultiThreadAlt(exec) =&gt; exec.block_on(<span class="kw-2">&amp;</span><span class="self">self</span>.handle.inner, future),
        }
    }

    <span class="doccomment">/// Enters the runtime context.
    ///
    /// This allows you to construct types that must have an executor
    /// available on creation such as [`Sleep`] or [`TcpStream`]. It will
    /// also allow you to call methods such as [`tokio::spawn`].
    ///
    /// [`Sleep`]: struct@crate::time::Sleep
    /// [`TcpStream`]: struct@crate::net::TcpStream
    /// [`tokio::spawn`]: fn@crate::spawn
    ///
    /// # Example
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    /// use tokio::task::JoinHandle;
    ///
    /// fn function_that_spawns(msg: String) -&gt; JoinHandle&lt;()&gt; {
    ///     // Had we not used `rt.enter` below, this would panic.
    ///     tokio::spawn(async move {
    ///         println!("{}", msg);
    ///     })
    /// }
    ///
    /// fn main() {
    ///     let rt = Runtime::new().unwrap();
    ///
    ///     let s = "Hello World!".to_string();
    ///
    ///     // By entering the context, we tie `tokio::spawn` to this executor.
    ///     let _guard = rt.enter();
    ///     let handle = function_that_spawns(s);
    ///
    ///     // Wait for the task before we end the test.
    ///     rt.block_on(handle).unwrap();
    /// }
    /// ```
    </span><span class="kw">pub fn </span>enter(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; EnterGuard&lt;<span class="lifetime">'_</span>&gt; {
        <span class="self">self</span>.handle.enter()
    }

    <span class="doccomment">/// Shuts down the runtime, waiting for at most `duration` for all spawned
    /// work to stop.
    ///
    /// See the [struct level documentation](Runtime#shutdown) for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    /// use tokio::task;
    ///
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// fn main() {
    ///    let runtime = Runtime::new().unwrap();
    ///
    ///    runtime.block_on(async move {
    ///        task::spawn_blocking(move || {
    ///            thread::sleep(Duration::from_secs(10_000));
    ///        });
    ///    });
    ///
    ///    runtime.shutdown_timeout(Duration::from_millis(100));
    /// }
    /// ```
    </span><span class="kw">pub fn </span>shutdown_timeout(<span class="kw-2">mut </span><span class="self">self</span>, duration: Duration) {
        <span class="comment">// Wakeup and shutdown all the worker threads
        </span><span class="self">self</span>.handle.inner.shutdown();
        <span class="self">self</span>.blocking_pool.shutdown(<span class="prelude-val">Some</span>(duration));
    }

    <span class="doccomment">/// Shuts down the runtime, without waiting for any spawned work to stop.
    ///
    /// This can be useful if you want to drop a runtime from within another runtime.
    /// Normally, dropping a runtime will block indefinitely for spawned blocking tasks
    /// to complete, which would normally not be permitted within an asynchronous context.
    /// By calling `shutdown_background()`, you can drop the runtime from such a context.
    ///
    /// Note however, that because we do not wait for any blocking tasks to complete, this
    /// may result in a resource leak (in that any blocking tasks are still running until they
    /// return.
    ///
    /// See the [struct level documentation](Runtime#shutdown) for more details.
    ///
    /// This function is equivalent to calling `shutdown_timeout(Duration::from_nanos(0))`.
    ///
    /// ```
    /// use tokio::runtime::Runtime;
    ///
    /// fn main() {
    ///    let runtime = Runtime::new().unwrap();
    ///
    ///    runtime.block_on(async move {
    ///        let inner_runtime = Runtime::new().unwrap();
    ///        // ...
    ///        inner_runtime.shutdown_background();
    ///    });
    /// }
    /// ```
    </span><span class="kw">pub fn </span>shutdown_background(<span class="self">self</span>) {
        <span class="self">self</span>.shutdown_timeout(Duration::from_nanos(<span class="number">0</span>));
    }
}

<span class="attr">#[allow(clippy::single_match)] </span><span class="comment">// there are comments in the error branch, so we don't want if-let
</span><span class="kw">impl </span>Drop <span class="kw">for </span>Runtime {
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">match </span><span class="kw-2">&amp;mut </span><span class="self">self</span>.scheduler {
            Scheduler::CurrentThread(current_thread) =&gt; {
                <span class="comment">// This ensures that tasks spawned on the current-thread
                // runtime are dropped inside the runtime's context.
                </span><span class="kw">let </span>_guard = context::try_set_current(<span class="kw-2">&amp;</span><span class="self">self</span>.handle.inner);
                current_thread.shutdown(<span class="kw-2">&amp;</span><span class="self">self</span>.handle.inner);
            }
            <span class="attr">#[cfg(all(feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
            </span>Scheduler::MultiThread(multi_thread) =&gt; {
                <span class="comment">// The threaded scheduler drops its tasks on its worker threads, which is
                // already in the runtime's context.
                </span>multi_thread.shutdown(<span class="kw-2">&amp;</span><span class="self">self</span>.handle.inner);
            }
            <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"rt-multi-thread"</span>, not(target_os = <span class="string">"wasi"</span>)))]
            </span>Scheduler::MultiThreadAlt(multi_thread) =&gt; {
                <span class="comment">// The threaded scheduler drops its tasks on its worker threads, which is
                // already in the runtime's context.
                </span>multi_thread.shutdown(<span class="kw-2">&amp;</span><span class="self">self</span>.handle.inner);
            }
        }
    }
}

<span class="kw">impl </span>std::panic::UnwindSafe <span class="kw">for </span>Runtime {}

<span class="kw">impl </span>std::panic::RefUnwindSafe <span class="kw">for </span>Runtime {}

<span class="macro">cfg_metrics!</span> {
    <span class="kw">impl </span>Runtime {
        <span class="doccomment">/// TODO
        </span><span class="kw">pub fn </span>metrics(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw">crate</span>::runtime::RuntimeMetrics {
            <span class="self">self</span>.handle.metrics()
        }
    }
}
</code></pre></div></section></main></body></html>