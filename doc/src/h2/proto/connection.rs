<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/h2-0.3.26/src/proto/connection.rs`."><title>connection.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="h2" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../h2/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">crate</span>::codec::UserError;
<span class="kw">use </span><span class="kw">crate</span>::frame::{Reason, StreamId};
<span class="kw">use crate</span>::{client, server};

<span class="kw">use </span><span class="kw">crate</span>::frame::DEFAULT_INITIAL_WINDOW_SIZE;
<span class="kw">use </span><span class="kw">crate</span>::proto::<span class="kw-2">*</span>;

<span class="kw">use </span>bytes::Bytes;
<span class="kw">use </span>futures_core::Stream;
<span class="kw">use </span>std::io;
<span class="kw">use </span>std::marker::PhantomData;
<span class="kw">use </span>std::pin::Pin;
<span class="kw">use </span>std::task::{Context, Poll};
<span class="kw">use </span>std::time::Duration;
<span class="kw">use </span>tokio::io::AsyncRead;

<span class="doccomment">/// An H2 connection
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Connection&lt;T, P, B: Buf = Bytes&gt;
<span class="kw">where
    </span>P: Peer,
{
    <span class="doccomment">/// Read / write frame values
    </span>codec: Codec&lt;T, Prioritized&lt;B&gt;&gt;,

    inner: ConnectionInner&lt;P, B&gt;,
}

<span class="comment">// Extracted part of `Connection` which does not depend on `T`. Reduces the amount of duplicated
// method instantiations.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">struct </span>ConnectionInner&lt;P, B: Buf = Bytes&gt;
<span class="kw">where
    </span>P: Peer,
{
    <span class="doccomment">/// Tracks the connection level state transitions.
    </span>state: State,

    <span class="doccomment">/// An error to report back once complete.
    ///
    /// This exists separately from State in order to support
    /// graceful shutdown.
    </span>error: <span class="prelude-ty">Option</span>&lt;frame::GoAway&gt;,

    <span class="doccomment">/// Pending GOAWAY frames to write.
    </span>go_away: GoAway,

    <span class="doccomment">/// Ping/pong handler
    </span>ping_pong: PingPong,

    <span class="doccomment">/// Connection settings
    </span>settings: Settings,

    <span class="doccomment">/// Stream state handler
    </span>streams: Streams&lt;B, P&gt;,

    <span class="doccomment">/// A `tracing` span tracking the lifetime of the connection.
    </span>span: tracing::Span,

    <span class="doccomment">/// Client or server
    </span>_phantom: PhantomData&lt;P&gt;,
}

<span class="kw">struct </span>DynConnection&lt;<span class="lifetime">'a</span>, B: Buf = Bytes&gt; {
    state: <span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>State,

    go_away: <span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>GoAway,

    streams: DynStreams&lt;<span class="lifetime">'a</span>, B&gt;,

    error: <span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span><span class="prelude-ty">Option</span>&lt;frame::GoAway&gt;,

    ping_pong: <span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>PingPong,
}

<span class="attr">#[derive(Debug, Clone)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>Config {
    <span class="kw">pub </span>next_stream_id: StreamId,
    <span class="kw">pub </span>initial_max_send_streams: usize,
    <span class="kw">pub </span>max_send_buffer_size: usize,
    <span class="kw">pub </span>reset_stream_duration: Duration,
    <span class="kw">pub </span>reset_stream_max: usize,
    <span class="kw">pub </span>remote_reset_stream_max: usize,
    <span class="kw">pub </span>local_error_reset_streams_max: <span class="prelude-ty">Option</span>&lt;usize&gt;,
    <span class="kw">pub </span>settings: frame::Settings,
}

<span class="attr">#[derive(Debug)]
</span><span class="kw">enum </span>State {
    <span class="doccomment">/// Currently open in a sane state
    </span>Open,

    <span class="doccomment">/// The codec must be flushed
    </span>Closing(Reason, Initiator),

    <span class="doccomment">/// In a closed state
    </span>Closed(Reason, Initiator),
}

<span class="kw">impl</span>&lt;T, P, B&gt; Connection&lt;T, P, B&gt;
<span class="kw">where
    </span>T: AsyncRead + AsyncWrite + Unpin,
    P: Peer,
    B: Buf,
{
    <span class="kw">pub fn </span>new(codec: Codec&lt;T, Prioritized&lt;B&gt;&gt;, config: Config) -&gt; Connection&lt;T, P, B&gt; {
        <span class="kw">fn </span>streams_config(config: <span class="kw-2">&amp;</span>Config) -&gt; streams::Config {
            streams::Config {
                local_init_window_sz: config
                    .settings
                    .initial_window_size()
                    .unwrap_or(DEFAULT_INITIAL_WINDOW_SIZE),
                initial_max_send_streams: config.initial_max_send_streams,
                local_max_buffer_size: config.max_send_buffer_size,
                local_next_stream_id: config.next_stream_id,
                local_push_enabled: config.settings.is_push_enabled().unwrap_or(<span class="bool-val">true</span>),
                extended_connect_protocol_enabled: config
                    .settings
                    .is_extended_connect_protocol_enabled()
                    .unwrap_or(<span class="bool-val">false</span>),
                local_reset_duration: config.reset_stream_duration,
                local_reset_max: config.reset_stream_max,
                remote_reset_max: config.remote_reset_stream_max,
                remote_init_window_sz: DEFAULT_INITIAL_WINDOW_SIZE,
                remote_max_initiated: config
                    .settings
                    .max_concurrent_streams()
                    .map(|max| max <span class="kw">as </span>usize),
                local_max_error_reset_streams: config.local_error_reset_streams_max,
            }
        }
        <span class="kw">let </span>streams = Streams::new(streams_config(<span class="kw-2">&amp;</span>config));
        Connection {
            codec,
            inner: ConnectionInner {
                state: State::Open,
                error: <span class="prelude-val">None</span>,
                go_away: GoAway::new(),
                ping_pong: PingPong::new(),
                settings: Settings::new(config.settings),
                streams,
                span: <span class="macro">tracing::debug_span!</span>(<span class="string">"Connection"</span>, peer = %P::NAME),
                _phantom: PhantomData,
            },
        }
    }

    <span class="doccomment">/// connection flow control
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>set_target_window_size(<span class="kw-2">&amp;mut </span><span class="self">self</span>, size: WindowSize) {
        <span class="kw">let </span>_res = <span class="self">self</span>.inner.streams.set_target_connection_window_size(size);
        <span class="comment">// TODO: proper error handling
        </span><span class="macro">debug_assert!</span>(_res.is_ok());
    }

    <span class="doccomment">/// Send a new SETTINGS frame with an updated initial window size.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>set_initial_window_size(<span class="kw-2">&amp;mut </span><span class="self">self</span>, size: WindowSize) -&gt; <span class="prelude-ty">Result</span>&lt;(), UserError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>settings = frame::Settings::default();
        settings.set_initial_window_size(<span class="prelude-val">Some</span>(size));
        <span class="self">self</span>.inner.settings.send_settings(settings)
    }

    <span class="doccomment">/// Send a new SETTINGS frame with extended CONNECT protocol enabled.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>set_enable_connect_protocol(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), UserError&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>settings = frame::Settings::default();
        settings.set_enable_connect_protocol(<span class="prelude-val">Some</span>(<span class="number">1</span>));
        <span class="self">self</span>.inner.settings.send_settings(settings)
    }

    <span class="doccomment">/// Returns the maximum number of concurrent streams that may be initiated
    /// by this peer.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>max_send_streams(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.inner.streams.max_send_streams()
    }

    <span class="doccomment">/// Returns the maximum number of concurrent streams that may be initiated
    /// by the remote peer.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>max_recv_streams(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.inner.streams.max_recv_streams()
    }

    <span class="attr">#[cfg(feature = <span class="string">"unstable"</span>)]
    </span><span class="kw">pub fn </span>num_wired_streams(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.inner.streams.num_wired_streams()
    }

    <span class="doccomment">/// Returns `Ready` when the connection is ready to receive a frame.
    ///
    /// Returns `Error` as this may raise errors that are caused by delayed
    /// processing of received frames.
    </span><span class="kw">fn </span>poll_ready(<span class="kw-2">&amp;mut </span><span class="self">self</span>, cx: <span class="kw-2">&amp;mut </span>Context) -&gt; Poll&lt;<span class="prelude-ty">Result</span>&lt;(), Error&gt;&gt; {
        <span class="kw">let </span>_e = <span class="self">self</span>.inner.span.enter();
        <span class="kw">let </span>span = <span class="macro">tracing::trace_span!</span>(<span class="string">"poll_ready"</span>);
        <span class="kw">let </span>_e = span.enter();
        <span class="comment">// The order of these calls don't really matter too much
        </span><span class="macro">ready!</span>(<span class="self">self</span>.inner.ping_pong.send_pending_pong(cx, <span class="kw-2">&amp;mut </span><span class="self">self</span>.codec))<span class="question-mark">?</span>;
        <span class="macro">ready!</span>(<span class="self">self</span>.inner.ping_pong.send_pending_ping(cx, <span class="kw-2">&amp;mut </span><span class="self">self</span>.codec))<span class="question-mark">?</span>;
        <span class="macro">ready!</span>(<span class="self">self
            </span>.inner
            .settings
            .poll_send(cx, <span class="kw-2">&amp;mut </span><span class="self">self</span>.codec, <span class="kw-2">&amp;mut </span><span class="self">self</span>.inner.streams))<span class="question-mark">?</span>;
        <span class="macro">ready!</span>(<span class="self">self</span>.inner.streams.send_pending_refusal(cx, <span class="kw-2">&amp;mut </span><span class="self">self</span>.codec))<span class="question-mark">?</span>;

        Poll::Ready(<span class="prelude-val">Ok</span>(()))
    }

    <span class="doccomment">/// Send any pending GOAWAY frames.
    ///
    /// This will return `Some(reason)` if the connection should be closed
    /// afterwards. If this is a graceful shutdown, this returns `None`.
    </span><span class="kw">fn </span>poll_go_away(<span class="kw-2">&amp;mut </span><span class="self">self</span>, cx: <span class="kw-2">&amp;mut </span>Context) -&gt; Poll&lt;<span class="prelude-ty">Option</span>&lt;io::Result&lt;Reason&gt;&gt;&gt; {
        <span class="self">self</span>.inner.go_away.send_pending_go_away(cx, <span class="kw-2">&amp;mut </span><span class="self">self</span>.codec)
    }

    <span class="kw">pub fn </span>go_away_from_user(<span class="kw-2">&amp;mut </span><span class="self">self</span>, e: Reason) {
        <span class="self">self</span>.inner.as_dyn().go_away_from_user(e)
    }

    <span class="kw">fn </span>take_error(<span class="kw-2">&amp;mut </span><span class="self">self</span>, ours: Reason, initiator: Initiator) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="kw">let </span>(debug_data, theirs) = <span class="self">self
            </span>.inner
            .error
            .take()
            .as_ref()
            .map_or((Bytes::new(), Reason::NO_ERROR), |frame| {
                (frame.debug_data().clone(), frame.reason())
            });

        <span class="kw">match </span>(ours, theirs) {
            (Reason::NO_ERROR, Reason::NO_ERROR) =&gt; <span class="prelude-val">Ok</span>(()),
            (ours, Reason::NO_ERROR) =&gt; <span class="prelude-val">Err</span>(Error::GoAway(Bytes::new(), ours, initiator)),
            <span class="comment">// If both sides reported an error, give their
            // error back to th user. We assume our error
            // was a consequence of their error, and less
            // important.
            </span>(<span class="kw">_</span>, theirs) =&gt; <span class="prelude-val">Err</span>(Error::remote_go_away(debug_data, theirs)),
        }
    }

    <span class="doccomment">/// Closes the connection by transitioning to a GOAWAY state
    /// iff there are no streams or references
    </span><span class="kw">pub fn </span>maybe_close_connection_if_no_streams(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="comment">// If we poll() and realize that there are no streams or references
        // then we can close the connection by transitioning to GOAWAY
        </span><span class="kw">if </span>!<span class="self">self</span>.inner.streams.has_streams_or_other_references() {
            <span class="self">self</span>.inner.as_dyn().go_away_now(Reason::NO_ERROR);
        }
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>take_user_pings(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;UserPings&gt; {
        <span class="self">self</span>.inner.ping_pong.take_user_pings()
    }

    <span class="doccomment">/// Advances the internal state of the connection.
    </span><span class="kw">pub fn </span>poll(<span class="kw-2">&amp;mut </span><span class="self">self</span>, cx: <span class="kw-2">&amp;mut </span>Context) -&gt; Poll&lt;<span class="prelude-ty">Result</span>&lt;(), Error&gt;&gt; {
        <span class="comment">// XXX(eliza): cloning the span is unfortunately necessary here in
        // order to placate the borrow checker — `self` is mutably borrowed by
        // `poll2`, which means that we can't borrow `self.span` to enter it.
        // The clone is just an atomic ref bump.
        </span><span class="kw">let </span>span = <span class="self">self</span>.inner.span.clone();
        <span class="kw">let </span>_e = span.enter();
        <span class="kw">let </span>span = <span class="macro">tracing::trace_span!</span>(<span class="string">"poll"</span>);
        <span class="kw">let </span>_e = span.enter();

        <span class="kw">loop </span>{
            <span class="macro">tracing::trace!</span>(connection.state = <span class="question-mark">?</span><span class="self">self</span>.inner.state);
            <span class="comment">// TODO: probably clean up this glob of code
            </span><span class="kw">match </span><span class="self">self</span>.inner.state {
                <span class="comment">// When open, continue to poll a frame
                </span>State::Open =&gt; {
                    <span class="kw">let </span>result = <span class="kw">match </span><span class="self">self</span>.poll2(cx) {
                        Poll::Ready(result) =&gt; result,
                        <span class="comment">// The connection is not ready to make progress
                        </span>Poll::Pending =&gt; {
                            <span class="comment">// Ensure all window updates have been sent.
                            //
                            // This will also handle flushing `self.codec`
                            </span><span class="macro">ready!</span>(<span class="self">self</span>.inner.streams.poll_complete(cx, <span class="kw-2">&amp;mut </span><span class="self">self</span>.codec))<span class="question-mark">?</span>;

                            <span class="kw">if </span>(<span class="self">self</span>.inner.error.is_some()
                                || <span class="self">self</span>.inner.go_away.should_close_on_idle())
                                &amp;&amp; !<span class="self">self</span>.inner.streams.has_streams()
                            {
                                <span class="self">self</span>.inner.as_dyn().go_away_now(Reason::NO_ERROR);
                                <span class="kw">continue</span>;
                            }

                            <span class="kw">return </span>Poll::Pending;
                        }
                    };

                    <span class="self">self</span>.inner.as_dyn().handle_poll2_result(result)<span class="question-mark">?
                </span>}
                State::Closing(reason, initiator) =&gt; {
                    <span class="macro">tracing::trace!</span>(<span class="string">"connection closing after flush"</span>);
                    <span class="comment">// Flush/shutdown the codec
                    </span><span class="macro">ready!</span>(<span class="self">self</span>.codec.shutdown(cx))<span class="question-mark">?</span>;

                    <span class="comment">// Transition the state to error
                    </span><span class="self">self</span>.inner.state = State::Closed(reason, initiator);
                }
                State::Closed(reason, initiator) =&gt; {
                    <span class="kw">return </span>Poll::Ready(<span class="self">self</span>.take_error(reason, initiator));
                }
            }
        }
    }

    <span class="kw">fn </span>poll2(<span class="kw-2">&amp;mut </span><span class="self">self</span>, cx: <span class="kw-2">&amp;mut </span>Context) -&gt; Poll&lt;<span class="prelude-ty">Result</span>&lt;(), Error&gt;&gt; {
        <span class="comment">// This happens outside of the loop to prevent needing to do a clock
        // check and then comparison of the queue possibly multiple times a
        // second (and thus, the clock wouldn't have changed enough to matter).
        </span><span class="self">self</span>.clear_expired_reset_streams();

        <span class="kw">loop </span>{
            <span class="comment">// First, ensure that the `Connection` is able to receive a frame
            //
            // The order here matters:
            // - poll_go_away may buffer a graceful shutdown GOAWAY frame
            // - If it has, we've also added a PING to be sent in poll_ready
            </span><span class="kw">if let </span><span class="prelude-val">Some</span>(reason) = <span class="macro">ready!</span>(<span class="self">self</span>.poll_go_away(cx)<span class="question-mark">?</span>) {
                <span class="kw">if </span><span class="self">self</span>.inner.go_away.should_close_now() {
                    <span class="kw">if </span><span class="self">self</span>.inner.go_away.is_user_initiated() {
                        <span class="comment">// A user initiated abrupt shutdown shouldn't return
                        // the same error back to the user.
                        </span><span class="kw">return </span>Poll::Ready(<span class="prelude-val">Ok</span>(()));
                    } <span class="kw">else </span>{
                        <span class="kw">return </span>Poll::Ready(<span class="prelude-val">Err</span>(Error::library_go_away(reason)));
                    }
                }
                <span class="comment">// Only NO_ERROR should be waiting for idle
                </span><span class="macro">debug_assert_eq!</span>(
                    reason,
                    Reason::NO_ERROR,
                    <span class="string">"graceful GOAWAY should be NO_ERROR"
                </span>);
            }
            <span class="macro">ready!</span>(<span class="self">self</span>.poll_ready(cx))<span class="question-mark">?</span>;

            <span class="kw">match </span><span class="self">self
                </span>.inner
                .as_dyn()
                .recv_frame(<span class="macro">ready!</span>(Pin::new(<span class="kw-2">&amp;mut </span><span class="self">self</span>.codec).poll_next(cx)<span class="question-mark">?</span>))<span class="question-mark">?
            </span>{
                ReceivedFrame::Settings(frame) =&gt; {
                    <span class="self">self</span>.inner.settings.recv_settings(
                        frame,
                        <span class="kw-2">&amp;mut </span><span class="self">self</span>.codec,
                        <span class="kw-2">&amp;mut </span><span class="self">self</span>.inner.streams,
                    )<span class="question-mark">?</span>;
                }
                ReceivedFrame::Continue =&gt; (),
                ReceivedFrame::Done =&gt; {
                    <span class="kw">return </span>Poll::Ready(<span class="prelude-val">Ok</span>(()));
                }
            }
        }
    }

    <span class="kw">fn </span>clear_expired_reset_streams(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.inner.streams.clear_expired_reset_streams();
    }
}

<span class="kw">impl</span>&lt;P, B&gt; ConnectionInner&lt;P, B&gt;
<span class="kw">where
    </span>P: Peer,
    B: Buf,
{
    <span class="kw">fn </span>as_dyn(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; DynConnection&lt;<span class="lifetime">'_</span>, B&gt; {
        <span class="kw">let </span>ConnectionInner {
            state,
            go_away,
            streams,
            error,
            ping_pong,
            ..
        } = <span class="self">self</span>;
        <span class="kw">let </span>streams = streams.as_dyn();
        DynConnection {
            state,
            go_away,
            streams,
            error,
            ping_pong,
        }
    }
}

<span class="kw">impl</span>&lt;B&gt; DynConnection&lt;<span class="lifetime">'_</span>, B&gt;
<span class="kw">where
    </span>B: Buf,
{
    <span class="kw">fn </span>go_away(<span class="kw-2">&amp;mut </span><span class="self">self</span>, id: StreamId, e: Reason) {
        <span class="kw">let </span>frame = frame::GoAway::new(id, e);
        <span class="self">self</span>.streams.send_go_away(id);
        <span class="self">self</span>.go_away.go_away(frame);
    }

    <span class="kw">fn </span>go_away_now(<span class="kw-2">&amp;mut </span><span class="self">self</span>, e: Reason) {
        <span class="kw">let </span>last_processed_id = <span class="self">self</span>.streams.last_processed_id();
        <span class="kw">let </span>frame = frame::GoAway::new(last_processed_id, e);
        <span class="self">self</span>.go_away.go_away_now(frame);
    }

    <span class="kw">fn </span>go_away_now_data(<span class="kw-2">&amp;mut </span><span class="self">self</span>, e: Reason, data: Bytes) {
        <span class="kw">let </span>last_processed_id = <span class="self">self</span>.streams.last_processed_id();
        <span class="kw">let </span>frame = frame::GoAway::with_debug_data(last_processed_id, e, data);
        <span class="self">self</span>.go_away.go_away_now(frame);
    }

    <span class="kw">fn </span>go_away_from_user(<span class="kw-2">&amp;mut </span><span class="self">self</span>, e: Reason) {
        <span class="kw">let </span>last_processed_id = <span class="self">self</span>.streams.last_processed_id();
        <span class="kw">let </span>frame = frame::GoAway::new(last_processed_id, e);
        <span class="self">self</span>.go_away.go_away_from_user(frame);

        <span class="comment">// Notify all streams of reason we're abruptly closing.
        </span><span class="self">self</span>.streams.handle_error(Error::user_go_away(e));
    }

    <span class="kw">fn </span>handle_poll2_result(<span class="kw-2">&amp;mut </span><span class="self">self</span>, result: <span class="prelude-ty">Result</span>&lt;(), Error&gt;) -&gt; <span class="prelude-ty">Result</span>&lt;(), Error&gt; {
        <span class="kw">match </span>result {
            <span class="comment">// The connection has shutdown normally
            </span><span class="prelude-val">Ok</span>(()) =&gt; {
                <span class="kw-2">*</span><span class="self">self</span>.state = State::Closing(Reason::NO_ERROR, Initiator::Library);
                <span class="prelude-val">Ok</span>(())
            }
            <span class="comment">// Attempting to read a frame resulted in a connection level
            // error. This is handled by setting a GOAWAY frame followed by
            // terminating the connection.
            </span><span class="prelude-val">Err</span>(Error::GoAway(debug_data, reason, initiator)) =&gt; {
                <span class="kw">let </span>e = Error::GoAway(debug_data.clone(), reason, initiator);
                <span class="macro">tracing::debug!</span>(error = <span class="question-mark">?</span>e, <span class="string">"Connection::poll; connection error"</span>);

                <span class="comment">// We may have already sent a GOAWAY for this error,
                // if so, don't send another, just flush and close up.
                </span><span class="kw">if </span><span class="self">self
                    </span>.go_away
                    .going_away()
                    .map_or(<span class="bool-val">false</span>, |frame| frame.reason() == reason)
                {
                    <span class="macro">tracing::trace!</span>(<span class="string">"    -&gt; already going away"</span>);
                    <span class="kw-2">*</span><span class="self">self</span>.state = State::Closing(reason, initiator);
                    <span class="kw">return </span><span class="prelude-val">Ok</span>(());
                }

                <span class="comment">// Reset all active streams
                </span><span class="self">self</span>.streams.handle_error(e);
                <span class="self">self</span>.go_away_now_data(reason, debug_data);
                <span class="prelude-val">Ok</span>(())
            }
            <span class="comment">// Attempting to read a frame resulted in a stream level error.
            // This is handled by resetting the frame then trying to read
            // another frame.
            </span><span class="prelude-val">Err</span>(Error::Reset(id, reason, initiator)) =&gt; {
                <span class="macro">debug_assert_eq!</span>(initiator, Initiator::Library);
                <span class="macro">tracing::trace!</span>(<span class="question-mark">?</span>id, <span class="question-mark">?</span>reason, <span class="string">"stream error"</span>);
                <span class="self">self</span>.streams.send_reset(id, reason);
                <span class="prelude-val">Ok</span>(())
            }
            <span class="comment">// Attempting to read a frame resulted in an I/O error. All
            // active streams must be reset.
            //
            // TODO: Are I/O errors recoverable?
            </span><span class="prelude-val">Err</span>(Error::Io(e, inner)) =&gt; {
                <span class="macro">tracing::debug!</span>(error = <span class="question-mark">?</span>e, <span class="string">"Connection::poll; IO error"</span>);
                <span class="kw">let </span>e = Error::Io(e, inner);

                <span class="comment">// Reset all active streams
                </span><span class="self">self</span>.streams.handle_error(e.clone());

                <span class="comment">// Return the error
                </span><span class="prelude-val">Err</span>(e)
            }
        }
    }

    <span class="kw">fn </span>recv_frame(<span class="kw-2">&amp;mut </span><span class="self">self</span>, frame: <span class="prelude-ty">Option</span>&lt;Frame&gt;) -&gt; <span class="prelude-ty">Result</span>&lt;ReceivedFrame, Error&gt; {
        <span class="kw">use </span><span class="kw">crate</span>::frame::Frame::<span class="kw-2">*</span>;
        <span class="kw">match </span>frame {
            <span class="prelude-val">Some</span>(Headers(frame)) =&gt; {
                <span class="macro">tracing::trace!</span>(<span class="question-mark">?</span>frame, <span class="string">"recv HEADERS"</span>);
                <span class="self">self</span>.streams.recv_headers(frame)<span class="question-mark">?</span>;
            }
            <span class="prelude-val">Some</span>(Data(frame)) =&gt; {
                <span class="macro">tracing::trace!</span>(<span class="question-mark">?</span>frame, <span class="string">"recv DATA"</span>);
                <span class="self">self</span>.streams.recv_data(frame)<span class="question-mark">?</span>;
            }
            <span class="prelude-val">Some</span>(Reset(frame)) =&gt; {
                <span class="macro">tracing::trace!</span>(<span class="question-mark">?</span>frame, <span class="string">"recv RST_STREAM"</span>);
                <span class="self">self</span>.streams.recv_reset(frame)<span class="question-mark">?</span>;
            }
            <span class="prelude-val">Some</span>(PushPromise(frame)) =&gt; {
                <span class="macro">tracing::trace!</span>(<span class="question-mark">?</span>frame, <span class="string">"recv PUSH_PROMISE"</span>);
                <span class="self">self</span>.streams.recv_push_promise(frame)<span class="question-mark">?</span>;
            }
            <span class="prelude-val">Some</span>(Settings(frame)) =&gt; {
                <span class="macro">tracing::trace!</span>(<span class="question-mark">?</span>frame, <span class="string">"recv SETTINGS"</span>);
                <span class="kw">return </span><span class="prelude-val">Ok</span>(ReceivedFrame::Settings(frame));
            }
            <span class="prelude-val">Some</span>(GoAway(frame)) =&gt; {
                <span class="macro">tracing::trace!</span>(<span class="question-mark">?</span>frame, <span class="string">"recv GOAWAY"</span>);
                <span class="comment">// This should prevent starting new streams,
                // but should allow continuing to process current streams
                // until they are all EOS. Once they are, State should
                // transition to GoAway.
                </span><span class="self">self</span>.streams.recv_go_away(<span class="kw-2">&amp;</span>frame)<span class="question-mark">?</span>;
                <span class="kw-2">*</span><span class="self">self</span>.error = <span class="prelude-val">Some</span>(frame);
            }
            <span class="prelude-val">Some</span>(Ping(frame)) =&gt; {
                <span class="macro">tracing::trace!</span>(<span class="question-mark">?</span>frame, <span class="string">"recv PING"</span>);
                <span class="kw">let </span>status = <span class="self">self</span>.ping_pong.recv_ping(frame);
                <span class="kw">if </span>status.is_shutdown() {
                    <span class="macro">assert!</span>(
                        <span class="self">self</span>.go_away.is_going_away(),
                        <span class="string">"received unexpected shutdown ping"
                    </span>);

                    <span class="kw">let </span>last_processed_id = <span class="self">self</span>.streams.last_processed_id();
                    <span class="self">self</span>.go_away(last_processed_id, Reason::NO_ERROR);
                }
            }
            <span class="prelude-val">Some</span>(WindowUpdate(frame)) =&gt; {
                <span class="macro">tracing::trace!</span>(<span class="question-mark">?</span>frame, <span class="string">"recv WINDOW_UPDATE"</span>);
                <span class="self">self</span>.streams.recv_window_update(frame)<span class="question-mark">?</span>;
            }
            <span class="prelude-val">Some</span>(Priority(frame)) =&gt; {
                <span class="macro">tracing::trace!</span>(<span class="question-mark">?</span>frame, <span class="string">"recv PRIORITY"</span>);
                <span class="comment">// TODO: handle
            </span>}
            <span class="prelude-val">None </span>=&gt; {
                <span class="macro">tracing::trace!</span>(<span class="string">"codec closed"</span>);
                <span class="self">self</span>.streams.recv_eof(<span class="bool-val">false</span>).expect(<span class="string">"mutex poisoned"</span>);
                <span class="kw">return </span><span class="prelude-val">Ok</span>(ReceivedFrame::Done);
            }
        }
        <span class="prelude-val">Ok</span>(ReceivedFrame::Continue)
    }
}

<span class="kw">enum </span>ReceivedFrame {
    Settings(frame::Settings),
    Continue,
    Done,
}

<span class="kw">impl</span>&lt;T, B&gt; Connection&lt;T, client::Peer, B&gt;
<span class="kw">where
    </span>T: AsyncRead + AsyncWrite,
    B: Buf,
{
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>streams(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>Streams&lt;B, client::Peer&gt; {
        <span class="kw-2">&amp;</span><span class="self">self</span>.inner.streams
    }
}

<span class="kw">impl</span>&lt;T, B&gt; Connection&lt;T, server::Peer, B&gt;
<span class="kw">where
    </span>T: AsyncRead + AsyncWrite + Unpin,
    B: Buf,
{
    <span class="kw">pub fn </span>next_incoming(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;StreamRef&lt;B&gt;&gt; {
        <span class="self">self</span>.inner.streams.next_incoming()
    }

    <span class="comment">// Graceful shutdown only makes sense for server peers.
    </span><span class="kw">pub fn </span>go_away_gracefully(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">if </span><span class="self">self</span>.inner.go_away.is_going_away() {
            <span class="comment">// No reason to start a new one.
            </span><span class="kw">return</span>;
        }

        <span class="comment">// According to http://httpwg.org/specs/rfc7540.html#GOAWAY:
        //
        // &gt; A server that is attempting to gracefully shut down a connection
        // &gt; SHOULD send an initial GOAWAY frame with the last stream
        // &gt; identifier set to 2^31-1 and a NO_ERROR code. This signals to the
        // &gt; client that a shutdown is imminent and that initiating further
        // &gt; requests is prohibited. After allowing time for any in-flight
        // &gt; stream creation (at least one round-trip time), the server can
        // &gt; send another GOAWAY frame with an updated last stream identifier.
        // &gt; This ensures that a connection can be cleanly shut down without
        // &gt; losing requests.
        </span><span class="self">self</span>.inner.as_dyn().go_away(StreamId::MAX, Reason::NO_ERROR);

        <span class="comment">// We take the advice of waiting 1 RTT literally, and wait
        // for a pong before proceeding.
        </span><span class="self">self</span>.inner.ping_pong.ping_shutdown();
    }
}

<span class="kw">impl</span>&lt;T, P, B&gt; Drop <span class="kw">for </span>Connection&lt;T, P, B&gt;
<span class="kw">where
    </span>P: Peer,
    B: Buf,
{
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="comment">// Ignore errors as this indicates that the mutex is poisoned.
        </span><span class="kw">let _ </span>= <span class="self">self</span>.inner.streams.recv_eof(<span class="bool-val">true</span>);
    }
}
</code></pre></div></section></main></body></html>