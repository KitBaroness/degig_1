<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.37.0/src/sync/watch.rs`."><title>watch.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="tokio" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../tokio/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="attr">#![cfg_attr(not(feature = <span class="string">"sync"</span>), allow(dead_code, unreachable_pub))]

</span><span class="doccomment">//! A multi-producer, multi-consumer channel that only retains the *last* sent
//! value.
//!
//! This channel is useful for watching for changes to a value from multiple
//! points in the code base, for example, changes to configuration values.
//!
//! # Usage
//!
//! [`channel`] returns a [`Sender`] / [`Receiver`] pair. These are the producer
//! and consumer halves of the channel. The channel is created with an initial
//! value.
//!
//! Each [`Receiver`] independently tracks the last value *seen* by its caller.
//!
//! To access the **current** value stored in the channel and mark it as *seen*
//! by a given [`Receiver`], use [`Receiver::borrow_and_update()`].
//!
//! To access the current value **without** marking it as *seen*, use
//! [`Receiver::borrow()`]. (If the value has already been marked *seen*,
//! [`Receiver::borrow()`] is equivalent to [`Receiver::borrow_and_update()`].)
//!
//! For more information on when to use these methods, see
//! [here](#borrow_and_update-versus-borrow).
//!
//! ## Change notifications
//!
//! The [`Receiver`] half provides an asynchronous [`changed`] method. This
//! method is ready when a new, *unseen* value is sent via the [`Sender`] half.
//!
//! * [`Receiver::changed()`] returns `Ok(())` on receiving a new value, or
//!   `Err(`[`error::RecvError`]`)` if the [`Sender`] has been dropped.
//! * If the current value is *unseen* when calling [`changed`], then
//!   [`changed`] will return immediately. If the current value is *seen*, then
//!   it will sleep until either a new message is sent via the [`Sender`] half,
//!   or the [`Sender`] is dropped.
//! * On completion, the [`changed`] method marks the new value as *seen*.
//! * At creation, the initial value is considered *seen*. In other words,
//!   [`Receiver::changed()`] will not return until a subsequent value is sent.
//! * New [`Receiver`] instances can be created with [`Sender::subscribe()`].
//!   The current value at the time the [`Receiver`] is created is considered
//!   *seen*.
//!
//! ## `borrow_and_update` versus `borrow`
//!
//! If the receiver intends to await notifications from [`changed`] in a loop,
//! [`Receiver::borrow_and_update()`] should be preferred over
//! [`Receiver::borrow()`].  This avoids a potential race where a new value is
//! sent between [`changed`] being ready and the value being read. (If
//! [`Receiver::borrow()`] is used, the loop may run twice with the same value.)
//!
//! If the receiver is only interested in the current value, and does not intend
//! to wait for changes, then [`Receiver::borrow()`] can be used. It may be more
//! convenient to use [`borrow`](Receiver::borrow) since it's an `&amp;self`
//! method---[`borrow_and_update`](Receiver::borrow_and_update) requires `&amp;mut
//! self`.
//!
//! # Examples
//!
//! The following example prints `hello! world! `.
//!
//! ```
//! use tokio::sync::watch;
//! use tokio::time::{Duration, sleep};
//!
//! # async fn dox() -&gt; Result&lt;(), Box&lt;dyn std::error::Error&gt;&gt; {
//! let (tx, mut rx) = watch::channel("hello");
//!
//! tokio::spawn(async move {
//!     // Use the equivalent of a "do-while" loop so the initial value is
//!     // processed before awaiting the `changed()` future.
//!     loop {
//!         println!("{}! ", *rx.borrow_and_update());
//!         if rx.changed().await.is_err() {
//!             break;
//!         }
//!     }
//! });
//!
//! sleep(Duration::from_millis(100)).await;
//! tx.send("world")?;
//! # Ok(())
//! # }
//! ```
//!
//! # Closing
//!
//! [`Sender::is_closed`] and [`Sender::closed`] allow the producer to detect
//! when all [`Receiver`] handles have been dropped. This indicates that there
//! is no further interest in the values being produced and work can be stopped.
//!
//! The value in the channel will not be dropped until the sender and all
//! receivers have been dropped.
//!
//! # Thread safety
//!
//! Both [`Sender`] and [`Receiver`] are thread safe. They can be moved to other
//! threads and can be used in a concurrent environment. Clones of [`Receiver`]
//! handles may be moved to separate threads and also used concurrently.
//!
//! [`Sender`]: crate::sync::watch::Sender
//! [`Receiver`]: crate::sync::watch::Receiver
//! [`changed`]: crate::sync::watch::Receiver::changed
//! [`Receiver::changed()`]: crate::sync::watch::Receiver::changed
//! [`Receiver::borrow()`]: crate::sync::watch::Receiver::borrow
//! [`Receiver::borrow_and_update()`]:
//!     crate::sync::watch::Receiver::borrow_and_update
//! [`channel`]: crate::sync::watch::channel
//! [`Sender::is_closed`]: crate::sync::watch::Sender::is_closed
//! [`Sender::closed`]: crate::sync::watch::Sender::closed
//! [`Sender::subscribe()`]: crate::sync::watch::Sender::subscribe

</span><span class="kw">use </span><span class="kw">crate</span>::sync::notify::Notify;

<span class="kw">use </span><span class="kw">crate</span>::loom::sync::atomic::AtomicUsize;
<span class="kw">use </span><span class="kw">crate</span>::loom::sync::atomic::Ordering::{AcqRel, Relaxed};
<span class="kw">use </span><span class="kw">crate</span>::loom::sync::{Arc, RwLock, RwLockReadGuard};
<span class="kw">use </span>std::fmt;
<span class="kw">use </span>std::mem;
<span class="kw">use </span>std::ops;
<span class="kw">use </span>std::panic;

<span class="doccomment">/// Receives values from the associated [`Sender`](struct@Sender).
///
/// Instances are created by the [`channel`](fn@channel) function.
///
/// To turn this receiver into a `Stream`, you can use the [`WatchStream`]
/// wrapper.
///
/// [`WatchStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.WatchStream.html
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>Receiver&lt;T&gt; {
    <span class="doccomment">/// Pointer to the shared state
    </span>shared: Arc&lt;Shared&lt;T&gt;&gt;,

    <span class="doccomment">/// Last observed version
    </span>version: Version,
}

<span class="doccomment">/// Sends values to the associated [`Receiver`](struct@Receiver).
///
/// Instances are created by the [`channel`](fn@channel) function.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>Sender&lt;T&gt; {
    shared: Arc&lt;Shared&lt;T&gt;&gt;,
}

<span class="kw">impl</span>&lt;T&gt; Clone <span class="kw">for </span>Sender&lt;T&gt; {
    <span class="kw">fn </span>clone(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self</span>.shared.ref_count_tx.fetch_add(<span class="number">1</span>, Relaxed);

        <span class="self">Self </span>{
            shared: <span class="self">self</span>.shared.clone(),
        }
    }
}

<span class="doccomment">/// Returns a reference to the inner value.
///
/// Outstanding borrows hold a read lock on the inner value. This means that
/// long-lived borrows could cause the producer half to block. It is recommended
/// to keep the borrow as short-lived as possible. Additionally, if you are
/// running in an environment that allows `!Send` futures, you must ensure that
/// the returned `Ref` type is never held alive across an `.await` point,
/// otherwise, it can lead to a deadlock.
///
/// The priority policy of the lock is dependent on the underlying lock
/// implementation, and this type does not guarantee that any particular policy
/// will be used. In particular, a producer which is waiting to acquire the lock
/// in `send` might or might not block concurrent calls to `borrow`, e.g.:
///
/// &lt;details&gt;&lt;summary&gt;Potential deadlock example&lt;/summary&gt;
///
/// ```text
/// // Task 1 (on thread A)    |  // Task 2 (on thread B)
/// let _ref1 = rx.borrow();   |
///                            |  // will block
///                            |  let _ = tx.send(());
/// // may deadlock            |
/// let _ref2 = rx.borrow();   |
/// ```
/// &lt;/details&gt;
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>Ref&lt;<span class="lifetime">'a</span>, T&gt; {
    inner: RwLockReadGuard&lt;<span class="lifetime">'a</span>, T&gt;,
    has_changed: bool,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, T&gt; Ref&lt;<span class="lifetime">'a</span>, T&gt; {
    <span class="doccomment">/// Indicates if the borrowed value is considered as _changed_ since the last
    /// time it has been marked as seen.
    ///
    /// Unlike [`Receiver::has_changed()`], this method does not fail if the channel is closed.
    ///
    /// When borrowed from the [`Sender`] this function will always return `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, mut rx) = watch::channel("hello");
    ///
    ///     tx.send("goodbye").unwrap();
    ///     // The sender does never consider the value as changed.
    ///     assert!(!tx.borrow().has_changed());
    ///
    ///     // Drop the sender immediately, just for testing purposes.
    ///     drop(tx);
    ///
    ///     // Even if the sender has already been dropped...
    ///     assert!(rx.has_changed().is_err());
    ///     // ...the modified value is still readable and detected as changed.
    ///     assert_eq!(*rx.borrow(), "goodbye");
    ///     assert!(rx.borrow().has_changed());
    ///
    ///     // Read the changed value and mark it as seen.
    ///     {
    ///         let received = rx.borrow_and_update();
    ///         assert_eq!(*received, "goodbye");
    ///         assert!(received.has_changed());
    ///         // Release the read lock when leaving this scope.
    ///     }
    ///
    ///     // Now the value has already been marked as seen and could
    ///     // never be modified again (after the sender has been dropped).
    ///     assert!(!rx.borrow().has_changed());
    /// }
    /// ```
    </span><span class="kw">pub fn </span>has_changed(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.has_changed
    }
}

<span class="kw">struct </span>Shared&lt;T&gt; {
    <span class="doccomment">/// The most recent value.
    </span>value: RwLock&lt;T&gt;,

    <span class="doccomment">/// The current version.
    ///
    /// The lowest bit represents a "closed" state. The rest of the bits
    /// represent the current version.
    </span>state: AtomicState,

    <span class="doccomment">/// Tracks the number of `Receiver` instances.
    </span>ref_count_rx: AtomicUsize,

    <span class="doccomment">/// Tracks the number of `Sender` instances.
    </span>ref_count_tx: AtomicUsize,

    <span class="doccomment">/// Notifies waiting receivers that the value changed.
    </span>notify_rx: big_notify::BigNotify,

    <span class="doccomment">/// Notifies any task listening for `Receiver` dropped events.
    </span>notify_tx: Notify,
}

<span class="kw">impl</span>&lt;T: fmt::Debug&gt; fmt::Debug <span class="kw">for </span>Shared&lt;T&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        <span class="kw">let </span>state = <span class="self">self</span>.state.load();
        f.debug_struct(<span class="string">"Shared"</span>)
            .field(<span class="string">"value"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.value)
            .field(<span class="string">"version"</span>, <span class="kw-2">&amp;</span>state.version())
            .field(<span class="string">"is_closed"</span>, <span class="kw-2">&amp;</span>state.is_closed())
            .field(<span class="string">"ref_count_rx"</span>, <span class="kw-2">&amp;</span><span class="self">self</span>.ref_count_rx)
            .finish()
    }
}

<span class="kw">pub mod </span>error {
    <span class="doccomment">//! Watch error types.

    </span><span class="kw">use </span>std::error::Error;
    <span class="kw">use </span>std::fmt;

    <span class="doccomment">/// Error produced when sending a value fails.
    </span><span class="attr">#[derive(PartialEq, Eq, Clone, Copy)]
    </span><span class="kw">pub struct </span>SendError&lt;T&gt;(<span class="kw">pub </span>T);

    <span class="comment">// ===== impl SendError =====

    </span><span class="kw">impl</span>&lt;T&gt; fmt::Debug <span class="kw">for </span>SendError&lt;T&gt; {
        <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
            f.debug_struct(<span class="string">"SendError"</span>).finish_non_exhaustive()
        }
    }

    <span class="kw">impl</span>&lt;T&gt; fmt::Display <span class="kw">for </span>SendError&lt;T&gt; {
        <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, fmt: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
            <span class="macro">write!</span>(fmt, <span class="string">"channel closed"</span>)
        }
    }

    <span class="kw">impl</span>&lt;T&gt; Error <span class="kw">for </span>SendError&lt;T&gt; {}

    <span class="doccomment">/// Error produced when receiving a change notification.
    </span><span class="attr">#[derive(Debug, Clone)]
    </span><span class="kw">pub struct </span>RecvError(<span class="kw">pub</span>(<span class="kw">super</span>) ());

    <span class="comment">// ===== impl RecvError =====

    </span><span class="kw">impl </span>fmt::Display <span class="kw">for </span>RecvError {
        <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, fmt: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
            <span class="macro">write!</span>(fmt, <span class="string">"channel closed"</span>)
        }
    }

    <span class="kw">impl </span>Error <span class="kw">for </span>RecvError {}
}

<span class="kw">mod </span>big_notify {
    <span class="kw">use </span><span class="kw">super</span>::Notify;
    <span class="kw">use </span><span class="kw">crate</span>::sync::notify::Notified;

    <span class="comment">// To avoid contention on the lock inside the `Notify`, we store multiple
    // copies of it. Then, we use either circular access or randomness to spread
    // out threads over different `Notify` objects.
    //
    // Some simple benchmarks show that randomness performs slightly better than
    // circular access (probably due to contention on `next`), so we prefer to
    // use randomness when Tokio is compiled with a random number generator.
    //
    // When the random number generator is not available, we fall back to
    // circular access.

    </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">struct </span>BigNotify {
        <span class="attr">#[cfg(not(all(not(loom), feature = <span class="string">"sync"</span>, any(feature = <span class="string">"rt"</span>, feature = <span class="string">"macros"</span>))))]
        </span>next: std::sync::atomic::AtomicUsize,
        inner: [Notify; <span class="number">8</span>],
    }

    <span class="kw">impl </span>BigNotify {
        <span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>new() -&gt; <span class="self">Self </span>{
            <span class="self">Self </span>{
                <span class="attr">#[cfg(not(all(
                    not(loom),
                    feature = <span class="string">"sync"</span>,
                    any(feature = <span class="string">"rt"</span>, feature = <span class="string">"macros"</span>)
                )))]
                </span>next: std::sync::atomic::AtomicUsize::new(<span class="number">0</span>),
                inner: Default::default(),
            }
        }

        <span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>notify_waiters(<span class="kw-2">&amp;</span><span class="self">self</span>) {
            <span class="kw">for </span>notify <span class="kw">in </span><span class="kw-2">&amp;</span><span class="self">self</span>.inner {
                notify.notify_waiters();
            }
        }

        <span class="doccomment">/// This function implements the case where randomness is not available.
        </span><span class="attr">#[cfg(not(all(not(loom), feature = <span class="string">"sync"</span>, any(feature = <span class="string">"rt"</span>, feature = <span class="string">"macros"</span>))))]
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>notified(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Notified&lt;<span class="lifetime">'_</span>&gt; {
            <span class="kw">let </span>i = <span class="self">self</span>.next.fetch_add(<span class="number">1</span>, std::sync::atomic::Ordering::Relaxed) % <span class="number">8</span>;
            <span class="self">self</span>.inner[i].notified()
        }

        <span class="doccomment">/// This function implements the case where randomness is available.
        </span><span class="attr">#[cfg(all(not(loom), feature = <span class="string">"sync"</span>, any(feature = <span class="string">"rt"</span>, feature = <span class="string">"macros"</span>)))]
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>notified(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Notified&lt;<span class="lifetime">'_</span>&gt; {
            <span class="kw">let </span>i = <span class="kw">crate</span>::runtime::context::thread_rng_n(<span class="number">8</span>) <span class="kw">as </span>usize;
            <span class="self">self</span>.inner[i].notified()
        }
    }
}

<span class="kw">use </span><span class="self">self</span>::state::{AtomicState, Version};
<span class="kw">mod </span>state {
    <span class="kw">use </span><span class="kw">crate</span>::loom::sync::atomic::AtomicUsize;
    <span class="kw">use </span><span class="kw">crate</span>::loom::sync::atomic::Ordering;

    <span class="kw">const </span>CLOSED_BIT: usize = <span class="number">1</span>;

    <span class="comment">// Using 2 as the step size preserves the `CLOSED_BIT`.
    </span><span class="kw">const </span>STEP_SIZE: usize = <span class="number">2</span>;

    <span class="doccomment">/// The version part of the state. The lowest bit is always zero.
    </span><span class="attr">#[derive(Copy, Clone, Debug, Eq, PartialEq)]
    </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">struct </span>Version(usize);

    <span class="doccomment">/// Snapshot of the state. The first bit is used as the CLOSED bit.
    /// The remaining bits are used as the version.
    ///
    /// The CLOSED bit tracks whether the Sender has been dropped. Dropping all
    /// receivers does not set it.
    </span><span class="attr">#[derive(Copy, Clone, Debug)]
    </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">struct </span>StateSnapshot(usize);

    <span class="doccomment">/// The state stored in an atomic integer.
    ///
    /// The `Sender` uses `Release` ordering for storing a new state
    /// and the `Receiver`s use `Acquire` ordering for loading the
    /// current state. This ensures that written values are seen by
    /// the `Receiver`s for a proper handover.
    </span><span class="attr">#[derive(Debug)]
    </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">struct </span>AtomicState(AtomicUsize);

    <span class="kw">impl </span>Version {
        <span class="doccomment">/// Decrements the version.
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>decrement(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
            <span class="comment">// Using a wrapping decrement here is required to ensure that the
            // operation is consistent with `std::sync::atomic::AtomicUsize::fetch_add()`
            // which wraps on overflow.
            </span><span class="self">self</span>.<span class="number">0 </span>= <span class="self">self</span>.<span class="number">0</span>.wrapping_sub(STEP_SIZE);
        }

        <span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">const </span>INITIAL: <span class="self">Self </span>= Version(<span class="number">0</span>);
    }

    <span class="kw">impl </span>StateSnapshot {
        <span class="doccomment">/// Extract the version from the state.
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>version(<span class="self">self</span>) -&gt; Version {
            Version(<span class="self">self</span>.<span class="number">0 </span>&amp; !CLOSED_BIT)
        }

        <span class="doccomment">/// Is the closed bit set?
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>is_closed(<span class="self">self</span>) -&gt; bool {
            (<span class="self">self</span>.<span class="number">0 </span>&amp; CLOSED_BIT) == CLOSED_BIT
        }
    }

    <span class="kw">impl </span>AtomicState {
        <span class="doccomment">/// Create a new `AtomicState` that is not closed and which has the
        /// version set to `Version::INITIAL`.
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>new() -&gt; <span class="self">Self </span>{
            AtomicState(AtomicUsize::new(Version::INITIAL.<span class="number">0</span>))
        }

        <span class="doccomment">/// Load the current value of the state.
        ///
        /// Only used by the receiver and for debugging purposes.
        ///
        /// The receiver side (read-only) uses `Acquire` ordering for a proper handover
        /// of the shared value with the sender side (single writer). The state is always
        /// updated after modifying and before releasing the (exclusive) lock on the
        /// shared value.
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>load(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; StateSnapshot {
            StateSnapshot(<span class="self">self</span>.<span class="number">0</span>.load(Ordering::Acquire))
        }

        <span class="doccomment">/// Increment the version counter.
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>increment_version_while_locked(<span class="kw-2">&amp;</span><span class="self">self</span>) {
            <span class="comment">// Use `Release` ordering to ensure that the shared value
            // has been written before updating the version. The shared
            // value is still protected by an exclusive lock during this
            // method.
            </span><span class="self">self</span>.<span class="number">0</span>.fetch_add(STEP_SIZE, Ordering::Release);
        }

        <span class="doccomment">/// Set the closed bit in the state.
        </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>set_closed(<span class="kw-2">&amp;</span><span class="self">self</span>) {
            <span class="self">self</span>.<span class="number">0</span>.fetch_or(CLOSED_BIT, Ordering::Release);
        }
    }
}

<span class="doccomment">/// Creates a new watch channel, returning the "send" and "receive" handles.
///
/// All values sent by [`Sender`] will become visible to the [`Receiver`] handles.
/// Only the last value sent is made available to the [`Receiver`] half. All
/// intermediate values are dropped.
///
/// # Examples
///
/// The following example prints `hello! world! `.
///
/// ```
/// use tokio::sync::watch;
/// use tokio::time::{Duration, sleep};
///
/// # async fn dox() -&gt; Result&lt;(), Box&lt;dyn std::error::Error&gt;&gt; {
/// let (tx, mut rx) = watch::channel("hello");
///
/// tokio::spawn(async move {
///     // Use the equivalent of a "do-while" loop so the initial value is
///     // processed before awaiting the `changed()` future.
///     loop {
///         println!("{}! ", *rx.borrow_and_update());
///         if rx.changed().await.is_err() {
///             break;
///         }
///     }
/// });
///
/// sleep(Duration::from_millis(100)).await;
/// tx.send("world")?;
/// # Ok(())
/// # }
/// ```
///
/// [`Sender`]: struct@Sender
/// [`Receiver`]: struct@Receiver
</span><span class="kw">pub fn </span>channel&lt;T&gt;(init: T) -&gt; (Sender&lt;T&gt;, Receiver&lt;T&gt;) {
    <span class="kw">let </span>shared = Arc::new(Shared {
        value: RwLock::new(init),
        state: AtomicState::new(),
        ref_count_rx: AtomicUsize::new(<span class="number">1</span>),
        ref_count_tx: AtomicUsize::new(<span class="number">1</span>),
        notify_rx: big_notify::BigNotify::new(),
        notify_tx: Notify::new(),
    });

    <span class="kw">let </span>tx = Sender {
        shared: shared.clone(),
    };

    <span class="kw">let </span>rx = Receiver {
        shared,
        version: Version::INITIAL,
    };

    (tx, rx)
}

<span class="kw">impl</span>&lt;T&gt; Receiver&lt;T&gt; {
    <span class="kw">fn </span>from_shared(version: Version, shared: Arc&lt;Shared&lt;T&gt;&gt;) -&gt; <span class="self">Self </span>{
        <span class="comment">// No synchronization necessary as this is only used as a counter and
        // not memory access.
        </span>shared.ref_count_rx.fetch_add(<span class="number">1</span>, Relaxed);

        <span class="self">Self </span>{ shared, version }
    }

    <span class="doccomment">/// Returns a reference to the most recently sent value.
    ///
    /// This method does not mark the returned value as seen, so future calls to
    /// [`changed`] may return immediately even if you have already seen the
    /// value with a call to `borrow`.
    ///
    /// Outstanding borrows hold a read lock on the inner value. This means that
    /// long-lived borrows could cause the producer half to block. It is recommended
    /// to keep the borrow as short-lived as possible. Additionally, if you are
    /// running in an environment that allows `!Send` futures, you must ensure that
    /// the returned `Ref` type is never held alive across an `.await` point,
    /// otherwise, it can lead to a deadlock.
    ///
    /// The priority policy of the lock is dependent on the underlying lock
    /// implementation, and this type does not guarantee that any particular policy
    /// will be used. In particular, a producer which is waiting to acquire the lock
    /// in `send` might or might not block concurrent calls to `borrow`, e.g.:
    ///
    /// &lt;details&gt;&lt;summary&gt;Potential deadlock example&lt;/summary&gt;
    ///
    /// ```text
    /// // Task 1 (on thread A)    |  // Task 2 (on thread B)
    /// let _ref1 = rx.borrow();   |
    ///                            |  // will block
    ///                            |  let _ = tx.send(());
    /// // may deadlock            |
    /// let _ref2 = rx.borrow();   |
    /// ```
    /// &lt;/details&gt;
    ///
    /// For more information on when to use this method versus
    /// [`borrow_and_update`], see [here](self#borrow_and_update-versus-borrow).
    ///
    /// [`changed`]: Receiver::changed
    /// [`borrow_and_update`]: Receiver::borrow_and_update
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// let (_, rx) = watch::channel("hello");
    /// assert_eq!(*rx.borrow(), "hello");
    /// ```
    </span><span class="kw">pub fn </span>borrow(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Ref&lt;<span class="lifetime">'_</span>, T&gt; {
        <span class="kw">let </span>inner = <span class="self">self</span>.shared.value.read().unwrap();

        <span class="comment">// After obtaining a read-lock no concurrent writes could occur
        // and the loaded version matches that of the borrowed reference.
        </span><span class="kw">let </span>new_version = <span class="self">self</span>.shared.state.load().version();
        <span class="kw">let </span>has_changed = <span class="self">self</span>.version != new_version;

        Ref { inner, has_changed }
    }

    <span class="doccomment">/// Returns a reference to the most recently sent value and marks that value
    /// as seen.
    ///
    /// This method marks the current value as seen. Subsequent calls to [`changed`]
    /// will not return immediately until the [`Sender`] has modified the shared
    /// value again.
    ///
    /// Outstanding borrows hold a read lock on the inner value. This means that
    /// long-lived borrows could cause the producer half to block. It is recommended
    /// to keep the borrow as short-lived as possible. Additionally, if you are
    /// running in an environment that allows `!Send` futures, you must ensure that
    /// the returned `Ref` type is never held alive across an `.await` point,
    /// otherwise, it can lead to a deadlock.
    ///
    /// The priority policy of the lock is dependent on the underlying lock
    /// implementation, and this type does not guarantee that any particular policy
    /// will be used. In particular, a producer which is waiting to acquire the lock
    /// in `send` might or might not block concurrent calls to `borrow`, e.g.:
    ///
    /// &lt;details&gt;&lt;summary&gt;Potential deadlock example&lt;/summary&gt;
    ///
    /// ```text
    /// // Task 1 (on thread A)                |  // Task 2 (on thread B)
    /// let _ref1 = rx1.borrow_and_update();   |
    ///                                        |  // will block
    ///                                        |  let _ = tx.send(());
    /// // may deadlock                        |
    /// let _ref2 = rx2.borrow_and_update();   |
    /// ```
    /// &lt;/details&gt;
    ///
    /// For more information on when to use this method versus [`borrow`], see
    /// [here](self#borrow_and_update-versus-borrow).
    ///
    /// [`changed`]: Receiver::changed
    /// [`borrow`]: Receiver::borrow
    </span><span class="kw">pub fn </span>borrow_and_update(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; Ref&lt;<span class="lifetime">'_</span>, T&gt; {
        <span class="kw">let </span>inner = <span class="self">self</span>.shared.value.read().unwrap();

        <span class="comment">// After obtaining a read-lock no concurrent writes could occur
        // and the loaded version matches that of the borrowed reference.
        </span><span class="kw">let </span>new_version = <span class="self">self</span>.shared.state.load().version();
        <span class="kw">let </span>has_changed = <span class="self">self</span>.version != new_version;

        <span class="comment">// Mark the shared value as seen by updating the version
        </span><span class="self">self</span>.version = new_version;

        Ref { inner, has_changed }
    }

    <span class="doccomment">/// Checks if this channel contains a message that this receiver has not yet
    /// seen. The new value is not marked as seen.
    ///
    /// Although this method is called `has_changed`, it does not check new
    /// messages for equality, so this call will return true even if the new
    /// message is equal to the old message.
    ///
    /// Returns an error if the channel has been closed.
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, mut rx) = watch::channel("hello");
    ///
    ///     tx.send("goodbye").unwrap();
    ///
    ///     assert!(rx.has_changed().unwrap());
    ///     assert_eq!(*rx.borrow_and_update(), "goodbye");
    ///
    ///     // The value has been marked as seen
    ///     assert!(!rx.has_changed().unwrap());
    ///
    ///     drop(tx);
    ///     // The `tx` handle has been dropped
    ///     assert!(rx.has_changed().is_err());
    /// }
    /// ```
    </span><span class="kw">pub fn </span>has_changed(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;bool, error::RecvError&gt; {
        <span class="comment">// Load the version from the state
        </span><span class="kw">let </span>state = <span class="self">self</span>.shared.state.load();
        <span class="kw">if </span>state.is_closed() {
            <span class="comment">// The sender has dropped.
            </span><span class="kw">return </span><span class="prelude-val">Err</span>(error::RecvError(()));
        }
        <span class="kw">let </span>new_version = state.version();

        <span class="prelude-val">Ok</span>(<span class="self">self</span>.version != new_version)
    }

    <span class="doccomment">/// Marks the state as changed.
    ///
    /// After invoking this method [`has_changed()`](Self::has_changed)
    /// returns `true` and [`changed()`](Self::changed) returns
    /// immediately, regardless of whether a new value has been sent.
    ///
    /// This is useful for triggering an initial change notification after
    /// subscribing to synchronize new receivers.
    </span><span class="kw">pub fn </span>mark_changed(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.version.decrement();
    }

    <span class="doccomment">/// Marks the state as unchanged.
    ///
    /// The current value will be considered seen by the receiver.
    ///
    /// This is useful if you are not interested in the current value
    /// visible in the receiver.
    </span><span class="kw">pub fn </span>mark_unchanged(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">let </span>current_version = <span class="self">self</span>.shared.state.load().version();
        <span class="self">self</span>.version = current_version;
    }

    <span class="doccomment">/// Waits for a change notification, then marks the newest value as seen.
    ///
    /// If the newest value in the channel has not yet been marked seen when
    /// this method is called, the method marks that value seen and returns
    /// immediately. If the newest value has already been marked seen, then the
    /// method sleeps until a new message is sent by the [`Sender`] connected to
    /// this `Receiver`, or until the [`Sender`] is dropped.
    ///
    /// This method returns an error if and only if the [`Sender`] is dropped.
    ///
    /// For more information, see
    /// [*Change notifications*](self#change-notifications) in the module-level documentation.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. If you use it as the event in a
    /// [`tokio::select!`](crate::select) statement and some other branch
    /// completes first, then it is guaranteed that no values have been marked
    /// seen by this call to `changed`.
    ///
    /// [`Sender`]: struct@Sender
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, mut rx) = watch::channel("hello");
    ///
    ///     tokio::spawn(async move {
    ///         tx.send("goodbye").unwrap();
    ///     });
    ///
    ///     assert!(rx.changed().await.is_ok());
    ///     assert_eq!(*rx.borrow_and_update(), "goodbye");
    ///
    ///     // The `tx` handle has been dropped
    ///     assert!(rx.changed().await.is_err());
    /// }
    /// ```
    </span><span class="kw">pub async fn </span>changed(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), error::RecvError&gt; {
        changed_impl(<span class="kw-2">&amp;</span><span class="self">self</span>.shared, <span class="kw-2">&amp;mut </span><span class="self">self</span>.version).<span class="kw">await
    </span>}

    <span class="doccomment">/// Waits for a value that satisfies the provided condition.
    ///
    /// This method will call the provided closure whenever something is sent on
    /// the channel. Once the closure returns `true`, this method will return a
    /// reference to the value that was passed to the closure.
    ///
    /// Before `wait_for` starts waiting for changes, it will call the closure
    /// on the current value. If the closure returns `true` when given the
    /// current value, then `wait_for` will immediately return a reference to
    /// the current value. This is the case even if the current value is already
    /// considered seen.
    ///
    /// The watch channel only keeps track of the most recent value, so if
    /// several messages are sent faster than `wait_for` is able to call the
    /// closure, then it may skip some updates. Whenever the closure is called,
    /// it will be called with the most recent value.
    ///
    /// When this function returns, the value that was passed to the closure
    /// when it returned `true` will be considered seen.
    ///
    /// If the channel is closed, then `wait_for` will return a `RecvError`.
    /// Once this happens, no more messages can ever be sent on the channel.
    /// When an error is returned, it is guaranteed that the closure has been
    /// called on the last value, and that it returned `false` for that value.
    /// (If the closure returned `true`, then the last value would have been
    /// returned instead of the error.)
    ///
    /// Like the `borrow` method, the returned borrow holds a read lock on the
    /// inner value. This means that long-lived borrows could cause the producer
    /// half to block. It is recommended to keep the borrow as short-lived as
    /// possible. See the documentation of `borrow` for more information on
    /// this.
    ///
    /// [`Receiver::changed()`]: crate::sync::watch::Receiver::changed
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// #[tokio::main]
    ///
    /// async fn main() {
    ///     let (tx, _rx) = watch::channel("hello");
    ///
    ///     tx.send("goodbye").unwrap();
    ///
    ///     // here we subscribe to a second receiver
    ///     // now in case of using `changed` we would have
    ///     // to first check the current value and then wait
    ///     // for changes or else `changed` would hang.
    ///     let mut rx2 = tx.subscribe();
    ///
    ///     // in place of changed we have use `wait_for`
    ///     // which would automatically check the current value
    ///     // and wait for changes until the closure returns true.
    ///     assert!(rx2.wait_for(|val| *val == "goodbye").await.is_ok());
    ///     assert_eq!(*rx2.borrow(), "goodbye");
    /// }
    /// ```
    </span><span class="kw">pub async fn </span>wait_for(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        <span class="kw-2">mut </span>f: <span class="kw">impl </span>FnMut(<span class="kw-2">&amp;</span>T) -&gt; bool,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;Ref&lt;<span class="lifetime">'_</span>, T&gt;, error::RecvError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>closed = <span class="bool-val">false</span>;
        <span class="kw">loop </span>{
            {
                <span class="kw">let </span>inner = <span class="self">self</span>.shared.value.read().unwrap();

                <span class="kw">let </span>new_version = <span class="self">self</span>.shared.state.load().version();
                <span class="kw">let </span>has_changed = <span class="self">self</span>.version != new_version;
                <span class="self">self</span>.version = new_version;

                <span class="kw">if </span>!closed || has_changed {
                    <span class="kw">let </span>result = panic::catch_unwind(panic::AssertUnwindSafe(|| f(<span class="kw-2">&amp;</span>inner)));
                    <span class="kw">match </span>result {
                        <span class="prelude-val">Ok</span>(<span class="bool-val">true</span>) =&gt; {
                            <span class="kw">return </span><span class="prelude-val">Ok</span>(Ref { inner, has_changed });
                        }
                        <span class="prelude-val">Ok</span>(<span class="bool-val">false</span>) =&gt; {
                            <span class="comment">// Skip the value.
                        </span>}
                        <span class="prelude-val">Err</span>(panicked) =&gt; {
                            <span class="comment">// Drop the read-lock to avoid poisoning it.
                            </span>drop(inner);
                            <span class="comment">// Forward the panic to the caller.
                            </span>panic::resume_unwind(panicked);
                            <span class="comment">// Unreachable
                        </span>}
                    };
                }
            }

            <span class="kw">if </span>closed {
                <span class="kw">return </span><span class="prelude-val">Err</span>(error::RecvError(()));
            }

            <span class="comment">// Wait for the value to change.
            </span>closed = changed_impl(<span class="kw-2">&amp;</span><span class="self">self</span>.shared, <span class="kw-2">&amp;mut </span><span class="self">self</span>.version).<span class="kw">await</span>.is_err();
        }
    }

    <span class="doccomment">/// Returns `true` if receivers belong to the same channel.
    ///
    /// # Examples
    ///
    /// ```
    /// let (tx, rx) = tokio::sync::watch::channel(true);
    /// let rx2 = rx.clone();
    /// assert!(rx.same_channel(&amp;rx2));
    ///
    /// let (tx3, rx3) = tokio::sync::watch::channel(true);
    /// assert!(!rx3.same_channel(&amp;rx2));
    /// ```
    </span><span class="kw">pub fn </span>same_channel(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; bool {
        Arc::ptr_eq(<span class="kw-2">&amp;</span><span class="self">self</span>.shared, <span class="kw-2">&amp;</span>other.shared)
    }

    <span class="macro">cfg_process_driver!</span> {
        <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>try_has_changed(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="prelude-ty">Result</span>&lt;(), error::RecvError&gt;&gt; {
            maybe_changed(<span class="kw-2">&amp;</span><span class="self">self</span>.shared, <span class="kw-2">&amp;mut </span><span class="self">self</span>.version)
        }
    }
}

<span class="kw">fn </span>maybe_changed&lt;T&gt;(
    shared: <span class="kw-2">&amp;</span>Shared&lt;T&gt;,
    version: <span class="kw-2">&amp;mut </span>Version,
) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="prelude-ty">Result</span>&lt;(), error::RecvError&gt;&gt; {
    <span class="comment">// Load the version from the state
    </span><span class="kw">let </span>state = shared.state.load();
    <span class="kw">let </span>new_version = state.version();

    <span class="kw">if </span><span class="kw-2">*</span>version != new_version {
        <span class="comment">// Observe the new version and return
        </span><span class="kw-2">*</span>version = new_version;
        <span class="kw">return </span><span class="prelude-val">Some</span>(<span class="prelude-val">Ok</span>(()));
    }

    <span class="kw">if </span>state.is_closed() {
        <span class="comment">// The sender has been dropped.
        </span><span class="kw">return </span><span class="prelude-val">Some</span>(<span class="prelude-val">Err</span>(error::RecvError(())));
    }

    <span class="prelude-val">None
</span>}

<span class="kw">async fn </span>changed_impl&lt;T&gt;(
    shared: <span class="kw-2">&amp;</span>Shared&lt;T&gt;,
    version: <span class="kw-2">&amp;mut </span>Version,
) -&gt; <span class="prelude-ty">Result</span>&lt;(), error::RecvError&gt; {
    <span class="kw">crate</span>::trace::async_trace_leaf().<span class="kw">await</span>;

    <span class="kw">loop </span>{
        <span class="comment">// In order to avoid a race condition, we first request a notification,
        // **then** check the current value's version. If a new version exists,
        // the notification request is dropped.
        </span><span class="kw">let </span>notified = shared.notify_rx.notified();

        <span class="kw">if let </span><span class="prelude-val">Some</span>(ret) = maybe_changed(shared, version) {
            <span class="kw">return </span>ret;
        }

        notified.<span class="kw">await</span>;
        <span class="comment">// loop around again in case the wake-up was spurious
    </span>}
}

<span class="kw">impl</span>&lt;T&gt; Clone <span class="kw">for </span>Receiver&lt;T&gt; {
    <span class="kw">fn </span>clone(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>version = <span class="self">self</span>.version;
        <span class="kw">let </span>shared = <span class="self">self</span>.shared.clone();

        <span class="self">Self</span>::from_shared(version, shared)
    }
}

<span class="kw">impl</span>&lt;T&gt; Drop <span class="kw">for </span>Receiver&lt;T&gt; {
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="comment">// No synchronization necessary as this is only used as a counter and
        // not memory access.
        </span><span class="kw">if </span><span class="number">1 </span>== <span class="self">self</span>.shared.ref_count_rx.fetch_sub(<span class="number">1</span>, Relaxed) {
            <span class="comment">// This is the last `Receiver` handle, tasks waiting on `Sender::closed()`
            </span><span class="self">self</span>.shared.notify_tx.notify_waiters();
        }
    }
}

<span class="kw">impl</span>&lt;T&gt; Sender&lt;T&gt; {
    <span class="doccomment">/// Creates the sending-half of the [`watch`] channel.
    ///
    /// See documentation of [`watch::channel`] for errors when calling this function.
    /// Beware that attempting to send a value when there are no receivers will
    /// return an error.
    ///
    /// [`watch`]: crate::sync::watch
    /// [`watch::channel`]: crate::sync::watch
    ///
    /// # Examples
    /// ```
    /// let sender = tokio::sync::watch::Sender::new(0u8);
    /// assert!(sender.send(3).is_err());
    /// let _rec = sender.subscribe();
    /// assert!(sender.send(4).is_ok());
    /// ```
    </span><span class="kw">pub fn </span>new(init: T) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>(tx, <span class="kw">_</span>) = channel(init);
        tx
    }

    <span class="doccomment">/// Sends a new value via the channel, notifying all receivers.
    ///
    /// This method fails if the channel is closed, which is the case when
    /// every receiver has been dropped. It is possible to reopen the channel
    /// using the [`subscribe`] method. However, when `send` fails, the value
    /// isn't made available for future receivers (but returned with the
    /// [`SendError`]).
    ///
    /// To always make a new value available for future receivers, even if no
    /// receiver currently exists, one of the other send methods
    /// ([`send_if_modified`], [`send_modify`], or [`send_replace`]) can be
    /// used instead.
    ///
    /// [`subscribe`]: Sender::subscribe
    /// [`SendError`]: error::SendError
    /// [`send_if_modified`]: Sender::send_if_modified
    /// [`send_modify`]: Sender::send_modify
    /// [`send_replace`]: Sender::send_replace
    </span><span class="kw">pub fn </span>send(<span class="kw-2">&amp;</span><span class="self">self</span>, value: T) -&gt; <span class="prelude-ty">Result</span>&lt;(), error::SendError&lt;T&gt;&gt; {
        <span class="comment">// This is pretty much only useful as a hint anyway, so synchronization isn't critical.
        </span><span class="kw">if </span><span class="number">0 </span>== <span class="self">self</span>.receiver_count() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(error::SendError(value));
        }

        <span class="self">self</span>.send_replace(value);
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Modifies the watched value **unconditionally** in-place,
    /// notifying all receivers.
    ///
    /// This can be useful for modifying the watched value, without
    /// having to allocate a new instance. Additionally, this
    /// method permits sending values even when there are no receivers.
    ///
    /// Prefer to use the more versatile function [`Self::send_if_modified()`]
    /// if the value is only modified conditionally during the mutable borrow
    /// to prevent unneeded change notifications for unmodified values.
    ///
    /// # Panics
    ///
    /// This function panics when the invocation of the `modify` closure panics.
    /// No receivers are notified when panicking. All changes of the watched
    /// value applied by the closure before panicking will be visible in
    /// subsequent calls to `borrow`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// struct State {
    ///     counter: usize,
    /// }
    /// let (state_tx, state_rx) = watch::channel(State { counter: 0 });
    /// state_tx.send_modify(|state| state.counter += 1);
    /// assert_eq!(state_rx.borrow().counter, 1);
    /// ```
    </span><span class="kw">pub fn </span>send_modify&lt;F&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, modify: F)
    <span class="kw">where
        </span>F: FnOnce(<span class="kw-2">&amp;mut </span>T),
    {
        <span class="self">self</span>.send_if_modified(|value| {
            modify(value);
            <span class="bool-val">true
        </span>});
    }

    <span class="doccomment">/// Modifies the watched value **conditionally** in-place,
    /// notifying all receivers only if modified.
    ///
    /// This can be useful for modifying the watched value, without
    /// having to allocate a new instance. Additionally, this
    /// method permits sending values even when there are no receivers.
    ///
    /// The `modify` closure must return `true` if the value has actually
    /// been modified during the mutable borrow. It should only return `false`
    /// if the value is guaranteed to be unmodified despite the mutable
    /// borrow.
    ///
    /// Receivers are only notified if the closure returned `true`. If the
    /// closure has modified the value but returned `false` this results
    /// in a *silent modification*, i.e. the modified value will be visible
    /// in subsequent calls to `borrow`, but receivers will not receive
    /// a change notification.
    ///
    /// Returns the result of the closure, i.e. `true` if the value has
    /// been modified and `false` otherwise.
    ///
    /// # Panics
    ///
    /// This function panics when the invocation of the `modify` closure panics.
    /// No receivers are notified when panicking. All changes of the watched
    /// value applied by the closure before panicking will be visible in
    /// subsequent calls to `borrow`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// struct State {
    ///     counter: usize,
    /// }
    /// let (state_tx, mut state_rx) = watch::channel(State { counter: 1 });
    /// let inc_counter_if_odd = |state: &amp;mut State| {
    ///     if state.counter % 2 == 1 {
    ///         state.counter += 1;
    ///         return true;
    ///     }
    ///     false
    /// };
    ///
    /// assert_eq!(state_rx.borrow().counter, 1);
    ///
    /// assert!(!state_rx.has_changed().unwrap());
    /// assert!(state_tx.send_if_modified(inc_counter_if_odd));
    /// assert!(state_rx.has_changed().unwrap());
    /// assert_eq!(state_rx.borrow_and_update().counter, 2);
    ///
    /// assert!(!state_rx.has_changed().unwrap());
    /// assert!(!state_tx.send_if_modified(inc_counter_if_odd));
    /// assert!(!state_rx.has_changed().unwrap());
    /// assert_eq!(state_rx.borrow_and_update().counter, 2);
    /// ```
    </span><span class="kw">pub fn </span>send_if_modified&lt;F&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, modify: F) -&gt; bool
    <span class="kw">where
        </span>F: FnOnce(<span class="kw-2">&amp;mut </span>T) -&gt; bool,
    {
        {
            <span class="comment">// Acquire the write lock and update the value.
            </span><span class="kw">let </span><span class="kw-2">mut </span>lock = <span class="self">self</span>.shared.value.write().unwrap();

            <span class="comment">// Update the value and catch possible panic inside func.
            </span><span class="kw">let </span>result = panic::catch_unwind(panic::AssertUnwindSafe(|| modify(<span class="kw-2">&amp;mut </span>lock)));
            <span class="kw">match </span>result {
                <span class="prelude-val">Ok</span>(modified) =&gt; {
                    <span class="kw">if </span>!modified {
                        <span class="comment">// Abort, i.e. don't notify receivers if unmodified
                        </span><span class="kw">return </span><span class="bool-val">false</span>;
                    }
                    <span class="comment">// Continue if modified
                </span>}
                <span class="prelude-val">Err</span>(panicked) =&gt; {
                    <span class="comment">// Drop the lock to avoid poisoning it.
                    </span>drop(lock);
                    <span class="comment">// Forward the panic to the caller.
                    </span>panic::resume_unwind(panicked);
                    <span class="comment">// Unreachable
                </span>}
            };

            <span class="self">self</span>.shared.state.increment_version_while_locked();

            <span class="comment">// Release the write lock.
            //
            // Incrementing the version counter while holding the lock ensures
            // that receivers are able to figure out the version number of the
            // value they are currently looking at.
            </span>drop(lock);
        }

        <span class="self">self</span>.shared.notify_rx.notify_waiters();

        <span class="bool-val">true
    </span>}

    <span class="doccomment">/// Sends a new value via the channel, notifying all receivers and returning
    /// the previous value in the channel.
    ///
    /// This can be useful for reusing the buffers inside a watched value.
    /// Additionally, this method permits sending values even when there are no
    /// receivers.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// let (tx, _rx) = watch::channel(1);
    /// assert_eq!(tx.send_replace(2), 1);
    /// assert_eq!(tx.send_replace(3), 2);
    /// ```
    </span><span class="kw">pub fn </span>send_replace(<span class="kw-2">&amp;</span><span class="self">self</span>, <span class="kw-2">mut </span>value: T) -&gt; T {
        <span class="comment">// swap old watched value with the new one
        </span><span class="self">self</span>.send_modify(|old| mem::swap(old, <span class="kw-2">&amp;mut </span>value));

        value
    }

    <span class="doccomment">/// Returns a reference to the most recently sent value
    ///
    /// Outstanding borrows hold a read lock on the inner value. This means that
    /// long-lived borrows could cause the producer half to block. It is recommended
    /// to keep the borrow as short-lived as possible. Additionally, if you are
    /// running in an environment that allows `!Send` futures, you must ensure that
    /// the returned `Ref` type is never held alive across an `.await` point,
    /// otherwise, it can lead to a deadlock.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// let (tx, _) = watch::channel("hello");
    /// assert_eq!(*tx.borrow(), "hello");
    /// ```
    </span><span class="kw">pub fn </span>borrow(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Ref&lt;<span class="lifetime">'_</span>, T&gt; {
        <span class="kw">let </span>inner = <span class="self">self</span>.shared.value.read().unwrap();

        <span class="comment">// The sender/producer always sees the current version
        </span><span class="kw">let </span>has_changed = <span class="bool-val">false</span>;

        Ref { inner, has_changed }
    }

    <span class="doccomment">/// Checks if the channel has been closed. This happens when all receivers
    /// have dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// let (tx, rx) = tokio::sync::watch::channel(());
    /// assert!(!tx.is_closed());
    ///
    /// drop(rx);
    /// assert!(tx.is_closed());
    /// ```
    </span><span class="kw">pub fn </span>is_closed(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.receiver_count() == <span class="number">0
    </span>}

    <span class="doccomment">/// Completes when all receivers have dropped.
    ///
    /// This allows the producer to get notified when interest in the produced
    /// values is canceled and immediately stop doing work.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. Once the channel is closed, it stays closed
    /// forever and all future calls to `closed` will return immediately.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, rx) = watch::channel("hello");
    ///
    ///     tokio::spawn(async move {
    ///         // use `rx`
    ///         drop(rx);
    ///     });
    ///
    ///     // Waits for `rx` to drop
    ///     tx.closed().await;
    ///     println!("the `rx` handles dropped")
    /// }
    /// ```
    </span><span class="kw">pub async fn </span>closed(<span class="kw-2">&amp;</span><span class="self">self</span>) {
        <span class="kw">crate</span>::trace::async_trace_leaf().<span class="kw">await</span>;

        <span class="kw">while </span><span class="self">self</span>.receiver_count() &gt; <span class="number">0 </span>{
            <span class="kw">let </span>notified = <span class="self">self</span>.shared.notify_tx.notified();

            <span class="kw">if </span><span class="self">self</span>.receiver_count() == <span class="number">0 </span>{
                <span class="kw">return</span>;
            }

            notified.<span class="kw">await</span>;
            <span class="comment">// The channel could have been reopened in the meantime by calling
            // `subscribe`, so we loop again.
        </span>}
    }

    <span class="doccomment">/// Creates a new [`Receiver`] connected to this `Sender`.
    ///
    /// All messages sent before this call to `subscribe` are initially marked
    /// as seen by the new `Receiver`.
    ///
    /// This method can be called even if there are no other receivers. In this
    /// case, the channel is reopened.
    ///
    /// # Examples
    ///
    /// The new channel will receive messages sent on this `Sender`.
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, _rx) = watch::channel(0u64);
    ///
    ///     tx.send(5).unwrap();
    ///
    ///     let rx = tx.subscribe();
    ///     assert_eq!(5, *rx.borrow());
    ///
    ///     tx.send(10).unwrap();
    ///     assert_eq!(10, *rx.borrow());
    /// }
    /// ```
    ///
    /// The most recent message is considered seen by the channel, so this test
    /// is guaranteed to pass.
    ///
    /// ```
    /// use tokio::sync::watch;
    /// use tokio::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, _rx) = watch::channel(0u64);
    ///     tx.send(5).unwrap();
    ///     let mut rx = tx.subscribe();
    ///
    ///     tokio::spawn(async move {
    ///         // by spawning and sleeping, the message is sent after `main`
    ///         // hits the call to `changed`.
    ///         # if false {
    ///         tokio::time::sleep(Duration::from_millis(10)).await;
    ///         # }
    ///         tx.send(100).unwrap();
    ///     });
    ///
    ///     rx.changed().await.unwrap();
    ///     assert_eq!(100, *rx.borrow());
    /// }
    /// ```
    </span><span class="kw">pub fn </span>subscribe(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Receiver&lt;T&gt; {
        <span class="kw">let </span>shared = <span class="self">self</span>.shared.clone();
        <span class="kw">let </span>version = shared.state.load().version();

        <span class="comment">// The CLOSED bit in the state tracks only whether the sender is
        // dropped, so we do not need to unset it if this reopens the channel.
        </span>Receiver::from_shared(version, shared)
    }

    <span class="doccomment">/// Returns the number of receivers that currently exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::watch;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, rx1) = watch::channel("hello");
    ///
    ///     assert_eq!(1, tx.receiver_count());
    ///
    ///     let mut _rx2 = rx1.clone();
    ///
    ///     assert_eq!(2, tx.receiver_count());
    /// }
    /// ```
    </span><span class="kw">pub fn </span>receiver_count(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.shared.ref_count_rx.load(Relaxed)
    }
}

<span class="kw">impl</span>&lt;T&gt; Drop <span class="kw">for </span>Sender&lt;T&gt; {
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">if </span><span class="self">self</span>.shared.ref_count_tx.fetch_sub(<span class="number">1</span>, AcqRel) == <span class="number">1 </span>{
            <span class="self">self</span>.shared.state.set_closed();
            <span class="self">self</span>.shared.notify_rx.notify_waiters();
        }
    }
}

<span class="comment">// ===== impl Ref =====

</span><span class="kw">impl</span>&lt;T&gt; ops::Deref <span class="kw">for </span>Ref&lt;<span class="lifetime">'_</span>, T&gt; {
    <span class="kw">type </span>Target = T;

    <span class="kw">fn </span>deref(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>T {
        <span class="self">self</span>.inner.deref()
    }
}

<span class="attr">#[cfg(all(test, loom))]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span>futures::future::FutureExt;
    <span class="kw">use </span>loom::thread;

    <span class="comment">// test for https://github.com/tokio-rs/tokio/issues/3168
    </span><span class="attr">#[test]
    </span><span class="kw">fn </span>watch_spurious_wakeup() {
        loom::model(|| {
            <span class="kw">let </span>(send, <span class="kw-2">mut </span>recv) = <span class="kw">crate</span>::sync::watch::channel(<span class="number">0i32</span>);

            send.send(<span class="number">1</span>).unwrap();

            <span class="kw">let </span>send_thread = thread::spawn(<span class="kw">move </span>|| {
                send.send(<span class="number">2</span>).unwrap();
                send
            });

            recv.changed().now_or_never();

            <span class="kw">let </span>send = send_thread.join().unwrap();
            <span class="kw">let </span>recv_thread = thread::spawn(<span class="kw">move </span>|| {
                recv.changed().now_or_never();
                recv.changed().now_or_never();
                recv
            });

            send.send(<span class="number">3</span>).unwrap();

            <span class="kw">let </span><span class="kw-2">mut </span>recv = recv_thread.join().unwrap();
            <span class="kw">let </span>send_thread = thread::spawn(<span class="kw">move </span>|| {
                send.send(<span class="number">2</span>).unwrap();
            });

            recv.changed().now_or_never();

            send_thread.join().unwrap();
        });
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>watch_borrow() {
        loom::model(|| {
            <span class="kw">let </span>(send, <span class="kw-2">mut </span>recv) = <span class="kw">crate</span>::sync::watch::channel(<span class="number">0i32</span>);

            <span class="macro">assert!</span>(send.borrow().eq(<span class="kw-2">&amp;</span><span class="number">0</span>));
            <span class="macro">assert!</span>(recv.borrow().eq(<span class="kw-2">&amp;</span><span class="number">0</span>));

            send.send(<span class="number">1</span>).unwrap();
            <span class="macro">assert!</span>(send.borrow().eq(<span class="kw-2">&amp;</span><span class="number">1</span>));

            <span class="kw">let </span>send_thread = thread::spawn(<span class="kw">move </span>|| {
                send.send(<span class="number">2</span>).unwrap();
                send
            });

            recv.changed().now_or_never();

            <span class="kw">let </span>send = send_thread.join().unwrap();
            <span class="kw">let </span>recv_thread = thread::spawn(<span class="kw">move </span>|| {
                recv.changed().now_or_never();
                recv.changed().now_or_never();
                recv
            });

            send.send(<span class="number">3</span>).unwrap();

            <span class="kw">let </span>recv = recv_thread.join().unwrap();
            <span class="macro">assert!</span>(recv.borrow().eq(<span class="kw-2">&amp;</span><span class="number">3</span>));
            <span class="macro">assert!</span>(send.borrow().eq(<span class="kw-2">&amp;</span><span class="number">3</span>));

            send.send(<span class="number">2</span>).unwrap();

            thread::spawn(<span class="kw">move </span>|| {
                <span class="macro">assert!</span>(recv.borrow().eq(<span class="kw-2">&amp;</span><span class="number">2</span>));
            });
            <span class="macro">assert!</span>(send.borrow().eq(<span class="kw-2">&amp;</span><span class="number">2</span>));
        });
    }
}
</code></pre></div></section></main></body></html>