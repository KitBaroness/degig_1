<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-syntax-0.8.3/src/ast/parse.rs`."><title>parse.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="regex_syntax" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../regex_syntax/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#3657" id="3657">3657</a>
<a href="#3658" id="3658">3658</a>
<a href="#3659" id="3659">3659</a>
<a href="#3660" id="3660">3660</a>
<a href="#3661" id="3661">3661</a>
<a href="#3662" id="3662">3662</a>
<a href="#3663" id="3663">3663</a>
<a href="#3664" id="3664">3664</a>
<a href="#3665" id="3665">3665</a>
<a href="#3666" id="3666">3666</a>
<a href="#3667" id="3667">3667</a>
<a href="#3668" id="3668">3668</a>
<a href="#3669" id="3669">3669</a>
<a href="#3670" id="3670">3670</a>
<a href="#3671" id="3671">3671</a>
<a href="#3672" id="3672">3672</a>
<a href="#3673" id="3673">3673</a>
<a href="#3674" id="3674">3674</a>
<a href="#3675" id="3675">3675</a>
<a href="#3676" id="3676">3676</a>
<a href="#3677" id="3677">3677</a>
<a href="#3678" id="3678">3678</a>
<a href="#3679" id="3679">3679</a>
<a href="#3680" id="3680">3680</a>
<a href="#3681" id="3681">3681</a>
<a href="#3682" id="3682">3682</a>
<a href="#3683" id="3683">3683</a>
<a href="#3684" id="3684">3684</a>
<a href="#3685" id="3685">3685</a>
<a href="#3686" id="3686">3686</a>
<a href="#3687" id="3687">3687</a>
<a href="#3688" id="3688">3688</a>
<a href="#3689" id="3689">3689</a>
<a href="#3690" id="3690">3690</a>
<a href="#3691" id="3691">3691</a>
<a href="#3692" id="3692">3692</a>
<a href="#3693" id="3693">3693</a>
<a href="#3694" id="3694">3694</a>
<a href="#3695" id="3695">3695</a>
<a href="#3696" id="3696">3696</a>
<a href="#3697" id="3697">3697</a>
<a href="#3698" id="3698">3698</a>
<a href="#3699" id="3699">3699</a>
<a href="#3700" id="3700">3700</a>
<a href="#3701" id="3701">3701</a>
<a href="#3702" id="3702">3702</a>
<a href="#3703" id="3703">3703</a>
<a href="#3704" id="3704">3704</a>
<a href="#3705" id="3705">3705</a>
<a href="#3706" id="3706">3706</a>
<a href="#3707" id="3707">3707</a>
<a href="#3708" id="3708">3708</a>
<a href="#3709" id="3709">3709</a>
<a href="#3710" id="3710">3710</a>
<a href="#3711" id="3711">3711</a>
<a href="#3712" id="3712">3712</a>
<a href="#3713" id="3713">3713</a>
<a href="#3714" id="3714">3714</a>
<a href="#3715" id="3715">3715</a>
<a href="#3716" id="3716">3716</a>
<a href="#3717" id="3717">3717</a>
<a href="#3718" id="3718">3718</a>
<a href="#3719" id="3719">3719</a>
<a href="#3720" id="3720">3720</a>
<a href="#3721" id="3721">3721</a>
<a href="#3722" id="3722">3722</a>
<a href="#3723" id="3723">3723</a>
<a href="#3724" id="3724">3724</a>
<a href="#3725" id="3725">3725</a>
<a href="#3726" id="3726">3726</a>
<a href="#3727" id="3727">3727</a>
<a href="#3728" id="3728">3728</a>
<a href="#3729" id="3729">3729</a>
<a href="#3730" id="3730">3730</a>
<a href="#3731" id="3731">3731</a>
<a href="#3732" id="3732">3732</a>
<a href="#3733" id="3733">3733</a>
<a href="#3734" id="3734">3734</a>
<a href="#3735" id="3735">3735</a>
<a href="#3736" id="3736">3736</a>
<a href="#3737" id="3737">3737</a>
<a href="#3738" id="3738">3738</a>
<a href="#3739" id="3739">3739</a>
<a href="#3740" id="3740">3740</a>
<a href="#3741" id="3741">3741</a>
<a href="#3742" id="3742">3742</a>
<a href="#3743" id="3743">3743</a>
<a href="#3744" id="3744">3744</a>
<a href="#3745" id="3745">3745</a>
<a href="#3746" id="3746">3746</a>
<a href="#3747" id="3747">3747</a>
<a href="#3748" id="3748">3748</a>
<a href="#3749" id="3749">3749</a>
<a href="#3750" id="3750">3750</a>
<a href="#3751" id="3751">3751</a>
<a href="#3752" id="3752">3752</a>
<a href="#3753" id="3753">3753</a>
<a href="#3754" id="3754">3754</a>
<a href="#3755" id="3755">3755</a>
<a href="#3756" id="3756">3756</a>
<a href="#3757" id="3757">3757</a>
<a href="#3758" id="3758">3758</a>
<a href="#3759" id="3759">3759</a>
<a href="#3760" id="3760">3760</a>
<a href="#3761" id="3761">3761</a>
<a href="#3762" id="3762">3762</a>
<a href="#3763" id="3763">3763</a>
<a href="#3764" id="3764">3764</a>
<a href="#3765" id="3765">3765</a>
<a href="#3766" id="3766">3766</a>
<a href="#3767" id="3767">3767</a>
<a href="#3768" id="3768">3768</a>
<a href="#3769" id="3769">3769</a>
<a href="#3770" id="3770">3770</a>
<a href="#3771" id="3771">3771</a>
<a href="#3772" id="3772">3772</a>
<a href="#3773" id="3773">3773</a>
<a href="#3774" id="3774">3774</a>
<a href="#3775" id="3775">3775</a>
<a href="#3776" id="3776">3776</a>
<a href="#3777" id="3777">3777</a>
<a href="#3778" id="3778">3778</a>
<a href="#3779" id="3779">3779</a>
<a href="#3780" id="3780">3780</a>
<a href="#3781" id="3781">3781</a>
<a href="#3782" id="3782">3782</a>
<a href="#3783" id="3783">3783</a>
<a href="#3784" id="3784">3784</a>
<a href="#3785" id="3785">3785</a>
<a href="#3786" id="3786">3786</a>
<a href="#3787" id="3787">3787</a>
<a href="#3788" id="3788">3788</a>
<a href="#3789" id="3789">3789</a>
<a href="#3790" id="3790">3790</a>
<a href="#3791" id="3791">3791</a>
<a href="#3792" id="3792">3792</a>
<a href="#3793" id="3793">3793</a>
<a href="#3794" id="3794">3794</a>
<a href="#3795" id="3795">3795</a>
<a href="#3796" id="3796">3796</a>
<a href="#3797" id="3797">3797</a>
<a href="#3798" id="3798">3798</a>
<a href="#3799" id="3799">3799</a>
<a href="#3800" id="3800">3800</a>
<a href="#3801" id="3801">3801</a>
<a href="#3802" id="3802">3802</a>
<a href="#3803" id="3803">3803</a>
<a href="#3804" id="3804">3804</a>
<a href="#3805" id="3805">3805</a>
<a href="#3806" id="3806">3806</a>
<a href="#3807" id="3807">3807</a>
<a href="#3808" id="3808">3808</a>
<a href="#3809" id="3809">3809</a>
<a href="#3810" id="3810">3810</a>
<a href="#3811" id="3811">3811</a>
<a href="#3812" id="3812">3812</a>
<a href="#3813" id="3813">3813</a>
<a href="#3814" id="3814">3814</a>
<a href="#3815" id="3815">3815</a>
<a href="#3816" id="3816">3816</a>
<a href="#3817" id="3817">3817</a>
<a href="#3818" id="3818">3818</a>
<a href="#3819" id="3819">3819</a>
<a href="#3820" id="3820">3820</a>
<a href="#3821" id="3821">3821</a>
<a href="#3822" id="3822">3822</a>
<a href="#3823" id="3823">3823</a>
<a href="#3824" id="3824">3824</a>
<a href="#3825" id="3825">3825</a>
<a href="#3826" id="3826">3826</a>
<a href="#3827" id="3827">3827</a>
<a href="#3828" id="3828">3828</a>
<a href="#3829" id="3829">3829</a>
<a href="#3830" id="3830">3830</a>
<a href="#3831" id="3831">3831</a>
<a href="#3832" id="3832">3832</a>
<a href="#3833" id="3833">3833</a>
<a href="#3834" id="3834">3834</a>
<a href="#3835" id="3835">3835</a>
<a href="#3836" id="3836">3836</a>
<a href="#3837" id="3837">3837</a>
<a href="#3838" id="3838">3838</a>
<a href="#3839" id="3839">3839</a>
<a href="#3840" id="3840">3840</a>
<a href="#3841" id="3841">3841</a>
<a href="#3842" id="3842">3842</a>
<a href="#3843" id="3843">3843</a>
<a href="#3844" id="3844">3844</a>
<a href="#3845" id="3845">3845</a>
<a href="#3846" id="3846">3846</a>
<a href="#3847" id="3847">3847</a>
<a href="#3848" id="3848">3848</a>
<a href="#3849" id="3849">3849</a>
<a href="#3850" id="3850">3850</a>
<a href="#3851" id="3851">3851</a>
<a href="#3852" id="3852">3852</a>
<a href="#3853" id="3853">3853</a>
<a href="#3854" id="3854">3854</a>
<a href="#3855" id="3855">3855</a>
<a href="#3856" id="3856">3856</a>
<a href="#3857" id="3857">3857</a>
<a href="#3858" id="3858">3858</a>
<a href="#3859" id="3859">3859</a>
<a href="#3860" id="3860">3860</a>
<a href="#3861" id="3861">3861</a>
<a href="#3862" id="3862">3862</a>
<a href="#3863" id="3863">3863</a>
<a href="#3864" id="3864">3864</a>
<a href="#3865" id="3865">3865</a>
<a href="#3866" id="3866">3866</a>
<a href="#3867" id="3867">3867</a>
<a href="#3868" id="3868">3868</a>
<a href="#3869" id="3869">3869</a>
<a href="#3870" id="3870">3870</a>
<a href="#3871" id="3871">3871</a>
<a href="#3872" id="3872">3872</a>
<a href="#3873" id="3873">3873</a>
<a href="#3874" id="3874">3874</a>
<a href="#3875" id="3875">3875</a>
<a href="#3876" id="3876">3876</a>
<a href="#3877" id="3877">3877</a>
<a href="#3878" id="3878">3878</a>
<a href="#3879" id="3879">3879</a>
<a href="#3880" id="3880">3880</a>
<a href="#3881" id="3881">3881</a>
<a href="#3882" id="3882">3882</a>
<a href="#3883" id="3883">3883</a>
<a href="#3884" id="3884">3884</a>
<a href="#3885" id="3885">3885</a>
<a href="#3886" id="3886">3886</a>
<a href="#3887" id="3887">3887</a>
<a href="#3888" id="3888">3888</a>
<a href="#3889" id="3889">3889</a>
<a href="#3890" id="3890">3890</a>
<a href="#3891" id="3891">3891</a>
<a href="#3892" id="3892">3892</a>
<a href="#3893" id="3893">3893</a>
<a href="#3894" id="3894">3894</a>
<a href="#3895" id="3895">3895</a>
<a href="#3896" id="3896">3896</a>
<a href="#3897" id="3897">3897</a>
<a href="#3898" id="3898">3898</a>
<a href="#3899" id="3899">3899</a>
<a href="#3900" id="3900">3900</a>
<a href="#3901" id="3901">3901</a>
<a href="#3902" id="3902">3902</a>
<a href="#3903" id="3903">3903</a>
<a href="#3904" id="3904">3904</a>
<a href="#3905" id="3905">3905</a>
<a href="#3906" id="3906">3906</a>
<a href="#3907" id="3907">3907</a>
<a href="#3908" id="3908">3908</a>
<a href="#3909" id="3909">3909</a>
<a href="#3910" id="3910">3910</a>
<a href="#3911" id="3911">3911</a>
<a href="#3912" id="3912">3912</a>
<a href="#3913" id="3913">3913</a>
<a href="#3914" id="3914">3914</a>
<a href="#3915" id="3915">3915</a>
<a href="#3916" id="3916">3916</a>
<a href="#3917" id="3917">3917</a>
<a href="#3918" id="3918">3918</a>
<a href="#3919" id="3919">3919</a>
<a href="#3920" id="3920">3920</a>
<a href="#3921" id="3921">3921</a>
<a href="#3922" id="3922">3922</a>
<a href="#3923" id="3923">3923</a>
<a href="#3924" id="3924">3924</a>
<a href="#3925" id="3925">3925</a>
<a href="#3926" id="3926">3926</a>
<a href="#3927" id="3927">3927</a>
<a href="#3928" id="3928">3928</a>
<a href="#3929" id="3929">3929</a>
<a href="#3930" id="3930">3930</a>
<a href="#3931" id="3931">3931</a>
<a href="#3932" id="3932">3932</a>
<a href="#3933" id="3933">3933</a>
<a href="#3934" id="3934">3934</a>
<a href="#3935" id="3935">3935</a>
<a href="#3936" id="3936">3936</a>
<a href="#3937" id="3937">3937</a>
<a href="#3938" id="3938">3938</a>
<a href="#3939" id="3939">3939</a>
<a href="#3940" id="3940">3940</a>
<a href="#3941" id="3941">3941</a>
<a href="#3942" id="3942">3942</a>
<a href="#3943" id="3943">3943</a>
<a href="#3944" id="3944">3944</a>
<a href="#3945" id="3945">3945</a>
<a href="#3946" id="3946">3946</a>
<a href="#3947" id="3947">3947</a>
<a href="#3948" id="3948">3948</a>
<a href="#3949" id="3949">3949</a>
<a href="#3950" id="3950">3950</a>
<a href="#3951" id="3951">3951</a>
<a href="#3952" id="3952">3952</a>
<a href="#3953" id="3953">3953</a>
<a href="#3954" id="3954">3954</a>
<a href="#3955" id="3955">3955</a>
<a href="#3956" id="3956">3956</a>
<a href="#3957" id="3957">3957</a>
<a href="#3958" id="3958">3958</a>
<a href="#3959" id="3959">3959</a>
<a href="#3960" id="3960">3960</a>
<a href="#3961" id="3961">3961</a>
<a href="#3962" id="3962">3962</a>
<a href="#3963" id="3963">3963</a>
<a href="#3964" id="3964">3964</a>
<a href="#3965" id="3965">3965</a>
<a href="#3966" id="3966">3966</a>
<a href="#3967" id="3967">3967</a>
<a href="#3968" id="3968">3968</a>
<a href="#3969" id="3969">3969</a>
<a href="#3970" id="3970">3970</a>
<a href="#3971" id="3971">3971</a>
<a href="#3972" id="3972">3972</a>
<a href="#3973" id="3973">3973</a>
<a href="#3974" id="3974">3974</a>
<a href="#3975" id="3975">3975</a>
<a href="#3976" id="3976">3976</a>
<a href="#3977" id="3977">3977</a>
<a href="#3978" id="3978">3978</a>
<a href="#3979" id="3979">3979</a>
<a href="#3980" id="3980">3980</a>
<a href="#3981" id="3981">3981</a>
<a href="#3982" id="3982">3982</a>
<a href="#3983" id="3983">3983</a>
<a href="#3984" id="3984">3984</a>
<a href="#3985" id="3985">3985</a>
<a href="#3986" id="3986">3986</a>
<a href="#3987" id="3987">3987</a>
<a href="#3988" id="3988">3988</a>
<a href="#3989" id="3989">3989</a>
<a href="#3990" id="3990">3990</a>
<a href="#3991" id="3991">3991</a>
<a href="#3992" id="3992">3992</a>
<a href="#3993" id="3993">3993</a>
<a href="#3994" id="3994">3994</a>
<a href="#3995" id="3995">3995</a>
<a href="#3996" id="3996">3996</a>
<a href="#3997" id="3997">3997</a>
<a href="#3998" id="3998">3998</a>
<a href="#3999" id="3999">3999</a>
<a href="#4000" id="4000">4000</a>
<a href="#4001" id="4001">4001</a>
<a href="#4002" id="4002">4002</a>
<a href="#4003" id="4003">4003</a>
<a href="#4004" id="4004">4004</a>
<a href="#4005" id="4005">4005</a>
<a href="#4006" id="4006">4006</a>
<a href="#4007" id="4007">4007</a>
<a href="#4008" id="4008">4008</a>
<a href="#4009" id="4009">4009</a>
<a href="#4010" id="4010">4010</a>
<a href="#4011" id="4011">4011</a>
<a href="#4012" id="4012">4012</a>
<a href="#4013" id="4013">4013</a>
<a href="#4014" id="4014">4014</a>
<a href="#4015" id="4015">4015</a>
<a href="#4016" id="4016">4016</a>
<a href="#4017" id="4017">4017</a>
<a href="#4018" id="4018">4018</a>
<a href="#4019" id="4019">4019</a>
<a href="#4020" id="4020">4020</a>
<a href="#4021" id="4021">4021</a>
<a href="#4022" id="4022">4022</a>
<a href="#4023" id="4023">4023</a>
<a href="#4024" id="4024">4024</a>
<a href="#4025" id="4025">4025</a>
<a href="#4026" id="4026">4026</a>
<a href="#4027" id="4027">4027</a>
<a href="#4028" id="4028">4028</a>
<a href="#4029" id="4029">4029</a>
<a href="#4030" id="4030">4030</a>
<a href="#4031" id="4031">4031</a>
<a href="#4032" id="4032">4032</a>
<a href="#4033" id="4033">4033</a>
<a href="#4034" id="4034">4034</a>
<a href="#4035" id="4035">4035</a>
<a href="#4036" id="4036">4036</a>
<a href="#4037" id="4037">4037</a>
<a href="#4038" id="4038">4038</a>
<a href="#4039" id="4039">4039</a>
<a href="#4040" id="4040">4040</a>
<a href="#4041" id="4041">4041</a>
<a href="#4042" id="4042">4042</a>
<a href="#4043" id="4043">4043</a>
<a href="#4044" id="4044">4044</a>
<a href="#4045" id="4045">4045</a>
<a href="#4046" id="4046">4046</a>
<a href="#4047" id="4047">4047</a>
<a href="#4048" id="4048">4048</a>
<a href="#4049" id="4049">4049</a>
<a href="#4050" id="4050">4050</a>
<a href="#4051" id="4051">4051</a>
<a href="#4052" id="4052">4052</a>
<a href="#4053" id="4053">4053</a>
<a href="#4054" id="4054">4054</a>
<a href="#4055" id="4055">4055</a>
<a href="#4056" id="4056">4056</a>
<a href="#4057" id="4057">4057</a>
<a href="#4058" id="4058">4058</a>
<a href="#4059" id="4059">4059</a>
<a href="#4060" id="4060">4060</a>
<a href="#4061" id="4061">4061</a>
<a href="#4062" id="4062">4062</a>
<a href="#4063" id="4063">4063</a>
<a href="#4064" id="4064">4064</a>
<a href="#4065" id="4065">4065</a>
<a href="#4066" id="4066">4066</a>
<a href="#4067" id="4067">4067</a>
<a href="#4068" id="4068">4068</a>
<a href="#4069" id="4069">4069</a>
<a href="#4070" id="4070">4070</a>
<a href="#4071" id="4071">4071</a>
<a href="#4072" id="4072">4072</a>
<a href="#4073" id="4073">4073</a>
<a href="#4074" id="4074">4074</a>
<a href="#4075" id="4075">4075</a>
<a href="#4076" id="4076">4076</a>
<a href="#4077" id="4077">4077</a>
<a href="#4078" id="4078">4078</a>
<a href="#4079" id="4079">4079</a>
<a href="#4080" id="4080">4080</a>
<a href="#4081" id="4081">4081</a>
<a href="#4082" id="4082">4082</a>
<a href="#4083" id="4083">4083</a>
<a href="#4084" id="4084">4084</a>
<a href="#4085" id="4085">4085</a>
<a href="#4086" id="4086">4086</a>
<a href="#4087" id="4087">4087</a>
<a href="#4088" id="4088">4088</a>
<a href="#4089" id="4089">4089</a>
<a href="#4090" id="4090">4090</a>
<a href="#4091" id="4091">4091</a>
<a href="#4092" id="4092">4092</a>
<a href="#4093" id="4093">4093</a>
<a href="#4094" id="4094">4094</a>
<a href="#4095" id="4095">4095</a>
<a href="#4096" id="4096">4096</a>
<a href="#4097" id="4097">4097</a>
<a href="#4098" id="4098">4098</a>
<a href="#4099" id="4099">4099</a>
<a href="#4100" id="4100">4100</a>
<a href="#4101" id="4101">4101</a>
<a href="#4102" id="4102">4102</a>
<a href="#4103" id="4103">4103</a>
<a href="#4104" id="4104">4104</a>
<a href="#4105" id="4105">4105</a>
<a href="#4106" id="4106">4106</a>
<a href="#4107" id="4107">4107</a>
<a href="#4108" id="4108">4108</a>
<a href="#4109" id="4109">4109</a>
<a href="#4110" id="4110">4110</a>
<a href="#4111" id="4111">4111</a>
<a href="#4112" id="4112">4112</a>
<a href="#4113" id="4113">4113</a>
<a href="#4114" id="4114">4114</a>
<a href="#4115" id="4115">4115</a>
<a href="#4116" id="4116">4116</a>
<a href="#4117" id="4117">4117</a>
<a href="#4118" id="4118">4118</a>
<a href="#4119" id="4119">4119</a>
<a href="#4120" id="4120">4120</a>
<a href="#4121" id="4121">4121</a>
<a href="#4122" id="4122">4122</a>
<a href="#4123" id="4123">4123</a>
<a href="#4124" id="4124">4124</a>
<a href="#4125" id="4125">4125</a>
<a href="#4126" id="4126">4126</a>
<a href="#4127" id="4127">4127</a>
<a href="#4128" id="4128">4128</a>
<a href="#4129" id="4129">4129</a>
<a href="#4130" id="4130">4130</a>
<a href="#4131" id="4131">4131</a>
<a href="#4132" id="4132">4132</a>
<a href="#4133" id="4133">4133</a>
<a href="#4134" id="4134">4134</a>
<a href="#4135" id="4135">4135</a>
<a href="#4136" id="4136">4136</a>
<a href="#4137" id="4137">4137</a>
<a href="#4138" id="4138">4138</a>
<a href="#4139" id="4139">4139</a>
<a href="#4140" id="4140">4140</a>
<a href="#4141" id="4141">4141</a>
<a href="#4142" id="4142">4142</a>
<a href="#4143" id="4143">4143</a>
<a href="#4144" id="4144">4144</a>
<a href="#4145" id="4145">4145</a>
<a href="#4146" id="4146">4146</a>
<a href="#4147" id="4147">4147</a>
<a href="#4148" id="4148">4148</a>
<a href="#4149" id="4149">4149</a>
<a href="#4150" id="4150">4150</a>
<a href="#4151" id="4151">4151</a>
<a href="#4152" id="4152">4152</a>
<a href="#4153" id="4153">4153</a>
<a href="#4154" id="4154">4154</a>
<a href="#4155" id="4155">4155</a>
<a href="#4156" id="4156">4156</a>
<a href="#4157" id="4157">4157</a>
<a href="#4158" id="4158">4158</a>
<a href="#4159" id="4159">4159</a>
<a href="#4160" id="4160">4160</a>
<a href="#4161" id="4161">4161</a>
<a href="#4162" id="4162">4162</a>
<a href="#4163" id="4163">4163</a>
<a href="#4164" id="4164">4164</a>
<a href="#4165" id="4165">4165</a>
<a href="#4166" id="4166">4166</a>
<a href="#4167" id="4167">4167</a>
<a href="#4168" id="4168">4168</a>
<a href="#4169" id="4169">4169</a>
<a href="#4170" id="4170">4170</a>
<a href="#4171" id="4171">4171</a>
<a href="#4172" id="4172">4172</a>
<a href="#4173" id="4173">4173</a>
<a href="#4174" id="4174">4174</a>
<a href="#4175" id="4175">4175</a>
<a href="#4176" id="4176">4176</a>
<a href="#4177" id="4177">4177</a>
<a href="#4178" id="4178">4178</a>
<a href="#4179" id="4179">4179</a>
<a href="#4180" id="4180">4180</a>
<a href="#4181" id="4181">4181</a>
<a href="#4182" id="4182">4182</a>
<a href="#4183" id="4183">4183</a>
<a href="#4184" id="4184">4184</a>
<a href="#4185" id="4185">4185</a>
<a href="#4186" id="4186">4186</a>
<a href="#4187" id="4187">4187</a>
<a href="#4188" id="4188">4188</a>
<a href="#4189" id="4189">4189</a>
<a href="#4190" id="4190">4190</a>
<a href="#4191" id="4191">4191</a>
<a href="#4192" id="4192">4192</a>
<a href="#4193" id="4193">4193</a>
<a href="#4194" id="4194">4194</a>
<a href="#4195" id="4195">4195</a>
<a href="#4196" id="4196">4196</a>
<a href="#4197" id="4197">4197</a>
<a href="#4198" id="4198">4198</a>
<a href="#4199" id="4199">4199</a>
<a href="#4200" id="4200">4200</a>
<a href="#4201" id="4201">4201</a>
<a href="#4202" id="4202">4202</a>
<a href="#4203" id="4203">4203</a>
<a href="#4204" id="4204">4204</a>
<a href="#4205" id="4205">4205</a>
<a href="#4206" id="4206">4206</a>
<a href="#4207" id="4207">4207</a>
<a href="#4208" id="4208">4208</a>
<a href="#4209" id="4209">4209</a>
<a href="#4210" id="4210">4210</a>
<a href="#4211" id="4211">4211</a>
<a href="#4212" id="4212">4212</a>
<a href="#4213" id="4213">4213</a>
<a href="#4214" id="4214">4214</a>
<a href="#4215" id="4215">4215</a>
<a href="#4216" id="4216">4216</a>
<a href="#4217" id="4217">4217</a>
<a href="#4218" id="4218">4218</a>
<a href="#4219" id="4219">4219</a>
<a href="#4220" id="4220">4220</a>
<a href="#4221" id="4221">4221</a>
<a href="#4222" id="4222">4222</a>
<a href="#4223" id="4223">4223</a>
<a href="#4224" id="4224">4224</a>
<a href="#4225" id="4225">4225</a>
<a href="#4226" id="4226">4226</a>
<a href="#4227" id="4227">4227</a>
<a href="#4228" id="4228">4228</a>
<a href="#4229" id="4229">4229</a>
<a href="#4230" id="4230">4230</a>
<a href="#4231" id="4231">4231</a>
<a href="#4232" id="4232">4232</a>
<a href="#4233" id="4233">4233</a>
<a href="#4234" id="4234">4234</a>
<a href="#4235" id="4235">4235</a>
<a href="#4236" id="4236">4236</a>
<a href="#4237" id="4237">4237</a>
<a href="#4238" id="4238">4238</a>
<a href="#4239" id="4239">4239</a>
<a href="#4240" id="4240">4240</a>
<a href="#4241" id="4241">4241</a>
<a href="#4242" id="4242">4242</a>
<a href="#4243" id="4243">4243</a>
<a href="#4244" id="4244">4244</a>
<a href="#4245" id="4245">4245</a>
<a href="#4246" id="4246">4246</a>
<a href="#4247" id="4247">4247</a>
<a href="#4248" id="4248">4248</a>
<a href="#4249" id="4249">4249</a>
<a href="#4250" id="4250">4250</a>
<a href="#4251" id="4251">4251</a>
<a href="#4252" id="4252">4252</a>
<a href="#4253" id="4253">4253</a>
<a href="#4254" id="4254">4254</a>
<a href="#4255" id="4255">4255</a>
<a href="#4256" id="4256">4256</a>
<a href="#4257" id="4257">4257</a>
<a href="#4258" id="4258">4258</a>
<a href="#4259" id="4259">4259</a>
<a href="#4260" id="4260">4260</a>
<a href="#4261" id="4261">4261</a>
<a href="#4262" id="4262">4262</a>
<a href="#4263" id="4263">4263</a>
<a href="#4264" id="4264">4264</a>
<a href="#4265" id="4265">4265</a>
<a href="#4266" id="4266">4266</a>
<a href="#4267" id="4267">4267</a>
<a href="#4268" id="4268">4268</a>
<a href="#4269" id="4269">4269</a>
<a href="#4270" id="4270">4270</a>
<a href="#4271" id="4271">4271</a>
<a href="#4272" id="4272">4272</a>
<a href="#4273" id="4273">4273</a>
<a href="#4274" id="4274">4274</a>
<a href="#4275" id="4275">4275</a>
<a href="#4276" id="4276">4276</a>
<a href="#4277" id="4277">4277</a>
<a href="#4278" id="4278">4278</a>
<a href="#4279" id="4279">4279</a>
<a href="#4280" id="4280">4280</a>
<a href="#4281" id="4281">4281</a>
<a href="#4282" id="4282">4282</a>
<a href="#4283" id="4283">4283</a>
<a href="#4284" id="4284">4284</a>
<a href="#4285" id="4285">4285</a>
<a href="#4286" id="4286">4286</a>
<a href="#4287" id="4287">4287</a>
<a href="#4288" id="4288">4288</a>
<a href="#4289" id="4289">4289</a>
<a href="#4290" id="4290">4290</a>
<a href="#4291" id="4291">4291</a>
<a href="#4292" id="4292">4292</a>
<a href="#4293" id="4293">4293</a>
<a href="#4294" id="4294">4294</a>
<a href="#4295" id="4295">4295</a>
<a href="#4296" id="4296">4296</a>
<a href="#4297" id="4297">4297</a>
<a href="#4298" id="4298">4298</a>
<a href="#4299" id="4299">4299</a>
<a href="#4300" id="4300">4300</a>
<a href="#4301" id="4301">4301</a>
<a href="#4302" id="4302">4302</a>
<a href="#4303" id="4303">4303</a>
<a href="#4304" id="4304">4304</a>
<a href="#4305" id="4305">4305</a>
<a href="#4306" id="4306">4306</a>
<a href="#4307" id="4307">4307</a>
<a href="#4308" id="4308">4308</a>
<a href="#4309" id="4309">4309</a>
<a href="#4310" id="4310">4310</a>
<a href="#4311" id="4311">4311</a>
<a href="#4312" id="4312">4312</a>
<a href="#4313" id="4313">4313</a>
<a href="#4314" id="4314">4314</a>
<a href="#4315" id="4315">4315</a>
<a href="#4316" id="4316">4316</a>
<a href="#4317" id="4317">4317</a>
<a href="#4318" id="4318">4318</a>
<a href="#4319" id="4319">4319</a>
<a href="#4320" id="4320">4320</a>
<a href="#4321" id="4321">4321</a>
<a href="#4322" id="4322">4322</a>
<a href="#4323" id="4323">4323</a>
<a href="#4324" id="4324">4324</a>
<a href="#4325" id="4325">4325</a>
<a href="#4326" id="4326">4326</a>
<a href="#4327" id="4327">4327</a>
<a href="#4328" id="4328">4328</a>
<a href="#4329" id="4329">4329</a>
<a href="#4330" id="4330">4330</a>
<a href="#4331" id="4331">4331</a>
<a href="#4332" id="4332">4332</a>
<a href="#4333" id="4333">4333</a>
<a href="#4334" id="4334">4334</a>
<a href="#4335" id="4335">4335</a>
<a href="#4336" id="4336">4336</a>
<a href="#4337" id="4337">4337</a>
<a href="#4338" id="4338">4338</a>
<a href="#4339" id="4339">4339</a>
<a href="#4340" id="4340">4340</a>
<a href="#4341" id="4341">4341</a>
<a href="#4342" id="4342">4342</a>
<a href="#4343" id="4343">4343</a>
<a href="#4344" id="4344">4344</a>
<a href="#4345" id="4345">4345</a>
<a href="#4346" id="4346">4346</a>
<a href="#4347" id="4347">4347</a>
<a href="#4348" id="4348">4348</a>
<a href="#4349" id="4349">4349</a>
<a href="#4350" id="4350">4350</a>
<a href="#4351" id="4351">4351</a>
<a href="#4352" id="4352">4352</a>
<a href="#4353" id="4353">4353</a>
<a href="#4354" id="4354">4354</a>
<a href="#4355" id="4355">4355</a>
<a href="#4356" id="4356">4356</a>
<a href="#4357" id="4357">4357</a>
<a href="#4358" id="4358">4358</a>
<a href="#4359" id="4359">4359</a>
<a href="#4360" id="4360">4360</a>
<a href="#4361" id="4361">4361</a>
<a href="#4362" id="4362">4362</a>
<a href="#4363" id="4363">4363</a>
<a href="#4364" id="4364">4364</a>
<a href="#4365" id="4365">4365</a>
<a href="#4366" id="4366">4366</a>
<a href="#4367" id="4367">4367</a>
<a href="#4368" id="4368">4368</a>
<a href="#4369" id="4369">4369</a>
<a href="#4370" id="4370">4370</a>
<a href="#4371" id="4371">4371</a>
<a href="#4372" id="4372">4372</a>
<a href="#4373" id="4373">4373</a>
<a href="#4374" id="4374">4374</a>
<a href="#4375" id="4375">4375</a>
<a href="#4376" id="4376">4376</a>
<a href="#4377" id="4377">4377</a>
<a href="#4378" id="4378">4378</a>
<a href="#4379" id="4379">4379</a>
<a href="#4380" id="4380">4380</a>
<a href="#4381" id="4381">4381</a>
<a href="#4382" id="4382">4382</a>
<a href="#4383" id="4383">4383</a>
<a href="#4384" id="4384">4384</a>
<a href="#4385" id="4385">4385</a>
<a href="#4386" id="4386">4386</a>
<a href="#4387" id="4387">4387</a>
<a href="#4388" id="4388">4388</a>
<a href="#4389" id="4389">4389</a>
<a href="#4390" id="4390">4390</a>
<a href="#4391" id="4391">4391</a>
<a href="#4392" id="4392">4392</a>
<a href="#4393" id="4393">4393</a>
<a href="#4394" id="4394">4394</a>
<a href="#4395" id="4395">4395</a>
<a href="#4396" id="4396">4396</a>
<a href="#4397" id="4397">4397</a>
<a href="#4398" id="4398">4398</a>
<a href="#4399" id="4399">4399</a>
<a href="#4400" id="4400">4400</a>
<a href="#4401" id="4401">4401</a>
<a href="#4402" id="4402">4402</a>
<a href="#4403" id="4403">4403</a>
<a href="#4404" id="4404">4404</a>
<a href="#4405" id="4405">4405</a>
<a href="#4406" id="4406">4406</a>
<a href="#4407" id="4407">4407</a>
<a href="#4408" id="4408">4408</a>
<a href="#4409" id="4409">4409</a>
<a href="#4410" id="4410">4410</a>
<a href="#4411" id="4411">4411</a>
<a href="#4412" id="4412">4412</a>
<a href="#4413" id="4413">4413</a>
<a href="#4414" id="4414">4414</a>
<a href="#4415" id="4415">4415</a>
<a href="#4416" id="4416">4416</a>
<a href="#4417" id="4417">4417</a>
<a href="#4418" id="4418">4418</a>
<a href="#4419" id="4419">4419</a>
<a href="#4420" id="4420">4420</a>
<a href="#4421" id="4421">4421</a>
<a href="#4422" id="4422">4422</a>
<a href="#4423" id="4423">4423</a>
<a href="#4424" id="4424">4424</a>
<a href="#4425" id="4425">4425</a>
<a href="#4426" id="4426">4426</a>
<a href="#4427" id="4427">4427</a>
<a href="#4428" id="4428">4428</a>
<a href="#4429" id="4429">4429</a>
<a href="#4430" id="4430">4430</a>
<a href="#4431" id="4431">4431</a>
<a href="#4432" id="4432">4432</a>
<a href="#4433" id="4433">4433</a>
<a href="#4434" id="4434">4434</a>
<a href="#4435" id="4435">4435</a>
<a href="#4436" id="4436">4436</a>
<a href="#4437" id="4437">4437</a>
<a href="#4438" id="4438">4438</a>
<a href="#4439" id="4439">4439</a>
<a href="#4440" id="4440">4440</a>
<a href="#4441" id="4441">4441</a>
<a href="#4442" id="4442">4442</a>
<a href="#4443" id="4443">4443</a>
<a href="#4444" id="4444">4444</a>
<a href="#4445" id="4445">4445</a>
<a href="#4446" id="4446">4446</a>
<a href="#4447" id="4447">4447</a>
<a href="#4448" id="4448">4448</a>
<a href="#4449" id="4449">4449</a>
<a href="#4450" id="4450">4450</a>
<a href="#4451" id="4451">4451</a>
<a href="#4452" id="4452">4452</a>
<a href="#4453" id="4453">4453</a>
<a href="#4454" id="4454">4454</a>
<a href="#4455" id="4455">4455</a>
<a href="#4456" id="4456">4456</a>
<a href="#4457" id="4457">4457</a>
<a href="#4458" id="4458">4458</a>
<a href="#4459" id="4459">4459</a>
<a href="#4460" id="4460">4460</a>
<a href="#4461" id="4461">4461</a>
<a href="#4462" id="4462">4462</a>
<a href="#4463" id="4463">4463</a>
<a href="#4464" id="4464">4464</a>
<a href="#4465" id="4465">4465</a>
<a href="#4466" id="4466">4466</a>
<a href="#4467" id="4467">4467</a>
<a href="#4468" id="4468">4468</a>
<a href="#4469" id="4469">4469</a>
<a href="#4470" id="4470">4470</a>
<a href="#4471" id="4471">4471</a>
<a href="#4472" id="4472">4472</a>
<a href="#4473" id="4473">4473</a>
<a href="#4474" id="4474">4474</a>
<a href="#4475" id="4475">4475</a>
<a href="#4476" id="4476">4476</a>
<a href="#4477" id="4477">4477</a>
<a href="#4478" id="4478">4478</a>
<a href="#4479" id="4479">4479</a>
<a href="#4480" id="4480">4480</a>
<a href="#4481" id="4481">4481</a>
<a href="#4482" id="4482">4482</a>
<a href="#4483" id="4483">4483</a>
<a href="#4484" id="4484">4484</a>
<a href="#4485" id="4485">4485</a>
<a href="#4486" id="4486">4486</a>
<a href="#4487" id="4487">4487</a>
<a href="#4488" id="4488">4488</a>
<a href="#4489" id="4489">4489</a>
<a href="#4490" id="4490">4490</a>
<a href="#4491" id="4491">4491</a>
<a href="#4492" id="4492">4492</a>
<a href="#4493" id="4493">4493</a>
<a href="#4494" id="4494">4494</a>
<a href="#4495" id="4495">4495</a>
<a href="#4496" id="4496">4496</a>
<a href="#4497" id="4497">4497</a>
<a href="#4498" id="4498">4498</a>
<a href="#4499" id="4499">4499</a>
<a href="#4500" id="4500">4500</a>
<a href="#4501" id="4501">4501</a>
<a href="#4502" id="4502">4502</a>
<a href="#4503" id="4503">4503</a>
<a href="#4504" id="4504">4504</a>
<a href="#4505" id="4505">4505</a>
<a href="#4506" id="4506">4506</a>
<a href="#4507" id="4507">4507</a>
<a href="#4508" id="4508">4508</a>
<a href="#4509" id="4509">4509</a>
<a href="#4510" id="4510">4510</a>
<a href="#4511" id="4511">4511</a>
<a href="#4512" id="4512">4512</a>
<a href="#4513" id="4513">4513</a>
<a href="#4514" id="4514">4514</a>
<a href="#4515" id="4515">4515</a>
<a href="#4516" id="4516">4516</a>
<a href="#4517" id="4517">4517</a>
<a href="#4518" id="4518">4518</a>
<a href="#4519" id="4519">4519</a>
<a href="#4520" id="4520">4520</a>
<a href="#4521" id="4521">4521</a>
<a href="#4522" id="4522">4522</a>
<a href="#4523" id="4523">4523</a>
<a href="#4524" id="4524">4524</a>
<a href="#4525" id="4525">4525</a>
<a href="#4526" id="4526">4526</a>
<a href="#4527" id="4527">4527</a>
<a href="#4528" id="4528">4528</a>
<a href="#4529" id="4529">4529</a>
<a href="#4530" id="4530">4530</a>
<a href="#4531" id="4531">4531</a>
<a href="#4532" id="4532">4532</a>
<a href="#4533" id="4533">4533</a>
<a href="#4534" id="4534">4534</a>
<a href="#4535" id="4535">4535</a>
<a href="#4536" id="4536">4536</a>
<a href="#4537" id="4537">4537</a>
<a href="#4538" id="4538">4538</a>
<a href="#4539" id="4539">4539</a>
<a href="#4540" id="4540">4540</a>
<a href="#4541" id="4541">4541</a>
<a href="#4542" id="4542">4542</a>
<a href="#4543" id="4543">4543</a>
<a href="#4544" id="4544">4544</a>
<a href="#4545" id="4545">4545</a>
<a href="#4546" id="4546">4546</a>
<a href="#4547" id="4547">4547</a>
<a href="#4548" id="4548">4548</a>
<a href="#4549" id="4549">4549</a>
<a href="#4550" id="4550">4550</a>
<a href="#4551" id="4551">4551</a>
<a href="#4552" id="4552">4552</a>
<a href="#4553" id="4553">4553</a>
<a href="#4554" id="4554">4554</a>
<a href="#4555" id="4555">4555</a>
<a href="#4556" id="4556">4556</a>
<a href="#4557" id="4557">4557</a>
<a href="#4558" id="4558">4558</a>
<a href="#4559" id="4559">4559</a>
<a href="#4560" id="4560">4560</a>
<a href="#4561" id="4561">4561</a>
<a href="#4562" id="4562">4562</a>
<a href="#4563" id="4563">4563</a>
<a href="#4564" id="4564">4564</a>
<a href="#4565" id="4565">4565</a>
<a href="#4566" id="4566">4566</a>
<a href="#4567" id="4567">4567</a>
<a href="#4568" id="4568">4568</a>
<a href="#4569" id="4569">4569</a>
<a href="#4570" id="4570">4570</a>
<a href="#4571" id="4571">4571</a>
<a href="#4572" id="4572">4572</a>
<a href="#4573" id="4573">4573</a>
<a href="#4574" id="4574">4574</a>
<a href="#4575" id="4575">4575</a>
<a href="#4576" id="4576">4576</a>
<a href="#4577" id="4577">4577</a>
<a href="#4578" id="4578">4578</a>
<a href="#4579" id="4579">4579</a>
<a href="#4580" id="4580">4580</a>
<a href="#4581" id="4581">4581</a>
<a href="#4582" id="4582">4582</a>
<a href="#4583" id="4583">4583</a>
<a href="#4584" id="4584">4584</a>
<a href="#4585" id="4585">4585</a>
<a href="#4586" id="4586">4586</a>
<a href="#4587" id="4587">4587</a>
<a href="#4588" id="4588">4588</a>
<a href="#4589" id="4589">4589</a>
<a href="#4590" id="4590">4590</a>
<a href="#4591" id="4591">4591</a>
<a href="#4592" id="4592">4592</a>
<a href="#4593" id="4593">4593</a>
<a href="#4594" id="4594">4594</a>
<a href="#4595" id="4595">4595</a>
<a href="#4596" id="4596">4596</a>
<a href="#4597" id="4597">4597</a>
<a href="#4598" id="4598">4598</a>
<a href="#4599" id="4599">4599</a>
<a href="#4600" id="4600">4600</a>
<a href="#4601" id="4601">4601</a>
<a href="#4602" id="4602">4602</a>
<a href="#4603" id="4603">4603</a>
<a href="#4604" id="4604">4604</a>
<a href="#4605" id="4605">4605</a>
<a href="#4606" id="4606">4606</a>
<a href="#4607" id="4607">4607</a>
<a href="#4608" id="4608">4608</a>
<a href="#4609" id="4609">4609</a>
<a href="#4610" id="4610">4610</a>
<a href="#4611" id="4611">4611</a>
<a href="#4612" id="4612">4612</a>
<a href="#4613" id="4613">4613</a>
<a href="#4614" id="4614">4614</a>
<a href="#4615" id="4615">4615</a>
<a href="#4616" id="4616">4616</a>
<a href="#4617" id="4617">4617</a>
<a href="#4618" id="4618">4618</a>
<a href="#4619" id="4619">4619</a>
<a href="#4620" id="4620">4620</a>
<a href="#4621" id="4621">4621</a>
<a href="#4622" id="4622">4622</a>
<a href="#4623" id="4623">4623</a>
<a href="#4624" id="4624">4624</a>
<a href="#4625" id="4625">4625</a>
<a href="#4626" id="4626">4626</a>
<a href="#4627" id="4627">4627</a>
<a href="#4628" id="4628">4628</a>
<a href="#4629" id="4629">4629</a>
<a href="#4630" id="4630">4630</a>
<a href="#4631" id="4631">4631</a>
<a href="#4632" id="4632">4632</a>
<a href="#4633" id="4633">4633</a>
<a href="#4634" id="4634">4634</a>
<a href="#4635" id="4635">4635</a>
<a href="#4636" id="4636">4636</a>
<a href="#4637" id="4637">4637</a>
<a href="#4638" id="4638">4638</a>
<a href="#4639" id="4639">4639</a>
<a href="#4640" id="4640">4640</a>
<a href="#4641" id="4641">4641</a>
<a href="#4642" id="4642">4642</a>
<a href="#4643" id="4643">4643</a>
<a href="#4644" id="4644">4644</a>
<a href="#4645" id="4645">4645</a>
<a href="#4646" id="4646">4646</a>
<a href="#4647" id="4647">4647</a>
<a href="#4648" id="4648">4648</a>
<a href="#4649" id="4649">4649</a>
<a href="#4650" id="4650">4650</a>
<a href="#4651" id="4651">4651</a>
<a href="#4652" id="4652">4652</a>
<a href="#4653" id="4653">4653</a>
<a href="#4654" id="4654">4654</a>
<a href="#4655" id="4655">4655</a>
<a href="#4656" id="4656">4656</a>
<a href="#4657" id="4657">4657</a>
<a href="#4658" id="4658">4658</a>
<a href="#4659" id="4659">4659</a>
<a href="#4660" id="4660">4660</a>
<a href="#4661" id="4661">4661</a>
<a href="#4662" id="4662">4662</a>
<a href="#4663" id="4663">4663</a>
<a href="#4664" id="4664">4664</a>
<a href="#4665" id="4665">4665</a>
<a href="#4666" id="4666">4666</a>
<a href="#4667" id="4667">4667</a>
<a href="#4668" id="4668">4668</a>
<a href="#4669" id="4669">4669</a>
<a href="#4670" id="4670">4670</a>
<a href="#4671" id="4671">4671</a>
<a href="#4672" id="4672">4672</a>
<a href="#4673" id="4673">4673</a>
<a href="#4674" id="4674">4674</a>
<a href="#4675" id="4675">4675</a>
<a href="#4676" id="4676">4676</a>
<a href="#4677" id="4677">4677</a>
<a href="#4678" id="4678">4678</a>
<a href="#4679" id="4679">4679</a>
<a href="#4680" id="4680">4680</a>
<a href="#4681" id="4681">4681</a>
<a href="#4682" id="4682">4682</a>
<a href="#4683" id="4683">4683</a>
<a href="#4684" id="4684">4684</a>
<a href="#4685" id="4685">4685</a>
<a href="#4686" id="4686">4686</a>
<a href="#4687" id="4687">4687</a>
<a href="#4688" id="4688">4688</a>
<a href="#4689" id="4689">4689</a>
<a href="#4690" id="4690">4690</a>
<a href="#4691" id="4691">4691</a>
<a href="#4692" id="4692">4692</a>
<a href="#4693" id="4693">4693</a>
<a href="#4694" id="4694">4694</a>
<a href="#4695" id="4695">4695</a>
<a href="#4696" id="4696">4696</a>
<a href="#4697" id="4697">4697</a>
<a href="#4698" id="4698">4698</a>
<a href="#4699" id="4699">4699</a>
<a href="#4700" id="4700">4700</a>
<a href="#4701" id="4701">4701</a>
<a href="#4702" id="4702">4702</a>
<a href="#4703" id="4703">4703</a>
<a href="#4704" id="4704">4704</a>
<a href="#4705" id="4705">4705</a>
<a href="#4706" id="4706">4706</a>
<a href="#4707" id="4707">4707</a>
<a href="#4708" id="4708">4708</a>
<a href="#4709" id="4709">4709</a>
<a href="#4710" id="4710">4710</a>
<a href="#4711" id="4711">4711</a>
<a href="#4712" id="4712">4712</a>
<a href="#4713" id="4713">4713</a>
<a href="#4714" id="4714">4714</a>
<a href="#4715" id="4715">4715</a>
<a href="#4716" id="4716">4716</a>
<a href="#4717" id="4717">4717</a>
<a href="#4718" id="4718">4718</a>
<a href="#4719" id="4719">4719</a>
<a href="#4720" id="4720">4720</a>
<a href="#4721" id="4721">4721</a>
<a href="#4722" id="4722">4722</a>
<a href="#4723" id="4723">4723</a>
<a href="#4724" id="4724">4724</a>
<a href="#4725" id="4725">4725</a>
<a href="#4726" id="4726">4726</a>
<a href="#4727" id="4727">4727</a>
<a href="#4728" id="4728">4728</a>
<a href="#4729" id="4729">4729</a>
<a href="#4730" id="4730">4730</a>
<a href="#4731" id="4731">4731</a>
<a href="#4732" id="4732">4732</a>
<a href="#4733" id="4733">4733</a>
<a href="#4734" id="4734">4734</a>
<a href="#4735" id="4735">4735</a>
<a href="#4736" id="4736">4736</a>
<a href="#4737" id="4737">4737</a>
<a href="#4738" id="4738">4738</a>
<a href="#4739" id="4739">4739</a>
<a href="#4740" id="4740">4740</a>
<a href="#4741" id="4741">4741</a>
<a href="#4742" id="4742">4742</a>
<a href="#4743" id="4743">4743</a>
<a href="#4744" id="4744">4744</a>
<a href="#4745" id="4745">4745</a>
<a href="#4746" id="4746">4746</a>
<a href="#4747" id="4747">4747</a>
<a href="#4748" id="4748">4748</a>
<a href="#4749" id="4749">4749</a>
<a href="#4750" id="4750">4750</a>
<a href="#4751" id="4751">4751</a>
<a href="#4752" id="4752">4752</a>
<a href="#4753" id="4753">4753</a>
<a href="#4754" id="4754">4754</a>
<a href="#4755" id="4755">4755</a>
<a href="#4756" id="4756">4756</a>
<a href="#4757" id="4757">4757</a>
<a href="#4758" id="4758">4758</a>
<a href="#4759" id="4759">4759</a>
<a href="#4760" id="4760">4760</a>
<a href="#4761" id="4761">4761</a>
<a href="#4762" id="4762">4762</a>
<a href="#4763" id="4763">4763</a>
<a href="#4764" id="4764">4764</a>
<a href="#4765" id="4765">4765</a>
<a href="#4766" id="4766">4766</a>
<a href="#4767" id="4767">4767</a>
<a href="#4768" id="4768">4768</a>
<a href="#4769" id="4769">4769</a>
<a href="#4770" id="4770">4770</a>
<a href="#4771" id="4771">4771</a>
<a href="#4772" id="4772">4772</a>
<a href="#4773" id="4773">4773</a>
<a href="#4774" id="4774">4774</a>
<a href="#4775" id="4775">4775</a>
<a href="#4776" id="4776">4776</a>
<a href="#4777" id="4777">4777</a>
<a href="#4778" id="4778">4778</a>
<a href="#4779" id="4779">4779</a>
<a href="#4780" id="4780">4780</a>
<a href="#4781" id="4781">4781</a>
<a href="#4782" id="4782">4782</a>
<a href="#4783" id="4783">4783</a>
<a href="#4784" id="4784">4784</a>
<a href="#4785" id="4785">4785</a>
<a href="#4786" id="4786">4786</a>
<a href="#4787" id="4787">4787</a>
<a href="#4788" id="4788">4788</a>
<a href="#4789" id="4789">4789</a>
<a href="#4790" id="4790">4790</a>
<a href="#4791" id="4791">4791</a>
<a href="#4792" id="4792">4792</a>
<a href="#4793" id="4793">4793</a>
<a href="#4794" id="4794">4794</a>
<a href="#4795" id="4795">4795</a>
<a href="#4796" id="4796">4796</a>
<a href="#4797" id="4797">4797</a>
<a href="#4798" id="4798">4798</a>
<a href="#4799" id="4799">4799</a>
<a href="#4800" id="4800">4800</a>
<a href="#4801" id="4801">4801</a>
<a href="#4802" id="4802">4802</a>
<a href="#4803" id="4803">4803</a>
<a href="#4804" id="4804">4804</a>
<a href="#4805" id="4805">4805</a>
<a href="#4806" id="4806">4806</a>
<a href="#4807" id="4807">4807</a>
<a href="#4808" id="4808">4808</a>
<a href="#4809" id="4809">4809</a>
<a href="#4810" id="4810">4810</a>
<a href="#4811" id="4811">4811</a>
<a href="#4812" id="4812">4812</a>
<a href="#4813" id="4813">4813</a>
<a href="#4814" id="4814">4814</a>
<a href="#4815" id="4815">4815</a>
<a href="#4816" id="4816">4816</a>
<a href="#4817" id="4817">4817</a>
<a href="#4818" id="4818">4818</a>
<a href="#4819" id="4819">4819</a>
<a href="#4820" id="4820">4820</a>
<a href="#4821" id="4821">4821</a>
<a href="#4822" id="4822">4822</a>
<a href="#4823" id="4823">4823</a>
<a href="#4824" id="4824">4824</a>
<a href="#4825" id="4825">4825</a>
<a href="#4826" id="4826">4826</a>
<a href="#4827" id="4827">4827</a>
<a href="#4828" id="4828">4828</a>
<a href="#4829" id="4829">4829</a>
<a href="#4830" id="4830">4830</a>
<a href="#4831" id="4831">4831</a>
<a href="#4832" id="4832">4832</a>
<a href="#4833" id="4833">4833</a>
<a href="#4834" id="4834">4834</a>
<a href="#4835" id="4835">4835</a>
<a href="#4836" id="4836">4836</a>
<a href="#4837" id="4837">4837</a>
<a href="#4838" id="4838">4838</a>
<a href="#4839" id="4839">4839</a>
<a href="#4840" id="4840">4840</a>
<a href="#4841" id="4841">4841</a>
<a href="#4842" id="4842">4842</a>
<a href="#4843" id="4843">4843</a>
<a href="#4844" id="4844">4844</a>
<a href="#4845" id="4845">4845</a>
<a href="#4846" id="4846">4846</a>
<a href="#4847" id="4847">4847</a>
<a href="#4848" id="4848">4848</a>
<a href="#4849" id="4849">4849</a>
<a href="#4850" id="4850">4850</a>
<a href="#4851" id="4851">4851</a>
<a href="#4852" id="4852">4852</a>
<a href="#4853" id="4853">4853</a>
<a href="#4854" id="4854">4854</a>
<a href="#4855" id="4855">4855</a>
<a href="#4856" id="4856">4856</a>
<a href="#4857" id="4857">4857</a>
<a href="#4858" id="4858">4858</a>
<a href="#4859" id="4859">4859</a>
<a href="#4860" id="4860">4860</a>
<a href="#4861" id="4861">4861</a>
<a href="#4862" id="4862">4862</a>
<a href="#4863" id="4863">4863</a>
<a href="#4864" id="4864">4864</a>
<a href="#4865" id="4865">4865</a>
<a href="#4866" id="4866">4866</a>
<a href="#4867" id="4867">4867</a>
<a href="#4868" id="4868">4868</a>
<a href="#4869" id="4869">4869</a>
<a href="#4870" id="4870">4870</a>
<a href="#4871" id="4871">4871</a>
<a href="#4872" id="4872">4872</a>
<a href="#4873" id="4873">4873</a>
<a href="#4874" id="4874">4874</a>
<a href="#4875" id="4875">4875</a>
<a href="#4876" id="4876">4876</a>
<a href="#4877" id="4877">4877</a>
<a href="#4878" id="4878">4878</a>
<a href="#4879" id="4879">4879</a>
<a href="#4880" id="4880">4880</a>
<a href="#4881" id="4881">4881</a>
<a href="#4882" id="4882">4882</a>
<a href="#4883" id="4883">4883</a>
<a href="#4884" id="4884">4884</a>
<a href="#4885" id="4885">4885</a>
<a href="#4886" id="4886">4886</a>
<a href="#4887" id="4887">4887</a>
<a href="#4888" id="4888">4888</a>
<a href="#4889" id="4889">4889</a>
<a href="#4890" id="4890">4890</a>
<a href="#4891" id="4891">4891</a>
<a href="#4892" id="4892">4892</a>
<a href="#4893" id="4893">4893</a>
<a href="#4894" id="4894">4894</a>
<a href="#4895" id="4895">4895</a>
<a href="#4896" id="4896">4896</a>
<a href="#4897" id="4897">4897</a>
<a href="#4898" id="4898">4898</a>
<a href="#4899" id="4899">4899</a>
<a href="#4900" id="4900">4900</a>
<a href="#4901" id="4901">4901</a>
<a href="#4902" id="4902">4902</a>
<a href="#4903" id="4903">4903</a>
<a href="#4904" id="4904">4904</a>
<a href="#4905" id="4905">4905</a>
<a href="#4906" id="4906">4906</a>
<a href="#4907" id="4907">4907</a>
<a href="#4908" id="4908">4908</a>
<a href="#4909" id="4909">4909</a>
<a href="#4910" id="4910">4910</a>
<a href="#4911" id="4911">4911</a>
<a href="#4912" id="4912">4912</a>
<a href="#4913" id="4913">4913</a>
<a href="#4914" id="4914">4914</a>
<a href="#4915" id="4915">4915</a>
<a href="#4916" id="4916">4916</a>
<a href="#4917" id="4917">4917</a>
<a href="#4918" id="4918">4918</a>
<a href="#4919" id="4919">4919</a>
<a href="#4920" id="4920">4920</a>
<a href="#4921" id="4921">4921</a>
<a href="#4922" id="4922">4922</a>
<a href="#4923" id="4923">4923</a>
<a href="#4924" id="4924">4924</a>
<a href="#4925" id="4925">4925</a>
<a href="#4926" id="4926">4926</a>
<a href="#4927" id="4927">4927</a>
<a href="#4928" id="4928">4928</a>
<a href="#4929" id="4929">4929</a>
<a href="#4930" id="4930">4930</a>
<a href="#4931" id="4931">4931</a>
<a href="#4932" id="4932">4932</a>
<a href="#4933" id="4933">4933</a>
<a href="#4934" id="4934">4934</a>
<a href="#4935" id="4935">4935</a>
<a href="#4936" id="4936">4936</a>
<a href="#4937" id="4937">4937</a>
<a href="#4938" id="4938">4938</a>
<a href="#4939" id="4939">4939</a>
<a href="#4940" id="4940">4940</a>
<a href="#4941" id="4941">4941</a>
<a href="#4942" id="4942">4942</a>
<a href="#4943" id="4943">4943</a>
<a href="#4944" id="4944">4944</a>
<a href="#4945" id="4945">4945</a>
<a href="#4946" id="4946">4946</a>
<a href="#4947" id="4947">4947</a>
<a href="#4948" id="4948">4948</a>
<a href="#4949" id="4949">4949</a>
<a href="#4950" id="4950">4950</a>
<a href="#4951" id="4951">4951</a>
<a href="#4952" id="4952">4952</a>
<a href="#4953" id="4953">4953</a>
<a href="#4954" id="4954">4954</a>
<a href="#4955" id="4955">4955</a>
<a href="#4956" id="4956">4956</a>
<a href="#4957" id="4957">4957</a>
<a href="#4958" id="4958">4958</a>
<a href="#4959" id="4959">4959</a>
<a href="#4960" id="4960">4960</a>
<a href="#4961" id="4961">4961</a>
<a href="#4962" id="4962">4962</a>
<a href="#4963" id="4963">4963</a>
<a href="#4964" id="4964">4964</a>
<a href="#4965" id="4965">4965</a>
<a href="#4966" id="4966">4966</a>
<a href="#4967" id="4967">4967</a>
<a href="#4968" id="4968">4968</a>
<a href="#4969" id="4969">4969</a>
<a href="#4970" id="4970">4970</a>
<a href="#4971" id="4971">4971</a>
<a href="#4972" id="4972">4972</a>
<a href="#4973" id="4973">4973</a>
<a href="#4974" id="4974">4974</a>
<a href="#4975" id="4975">4975</a>
<a href="#4976" id="4976">4976</a>
<a href="#4977" id="4977">4977</a>
<a href="#4978" id="4978">4978</a>
<a href="#4979" id="4979">4979</a>
<a href="#4980" id="4980">4980</a>
<a href="#4981" id="4981">4981</a>
<a href="#4982" id="4982">4982</a>
<a href="#4983" id="4983">4983</a>
<a href="#4984" id="4984">4984</a>
<a href="#4985" id="4985">4985</a>
<a href="#4986" id="4986">4986</a>
<a href="#4987" id="4987">4987</a>
<a href="#4988" id="4988">4988</a>
<a href="#4989" id="4989">4989</a>
<a href="#4990" id="4990">4990</a>
<a href="#4991" id="4991">4991</a>
<a href="#4992" id="4992">4992</a>
<a href="#4993" id="4993">4993</a>
<a href="#4994" id="4994">4994</a>
<a href="#4995" id="4995">4995</a>
<a href="#4996" id="4996">4996</a>
<a href="#4997" id="4997">4997</a>
<a href="#4998" id="4998">4998</a>
<a href="#4999" id="4999">4999</a>
<a href="#5000" id="5000">5000</a>
<a href="#5001" id="5001">5001</a>
<a href="#5002" id="5002">5002</a>
<a href="#5003" id="5003">5003</a>
<a href="#5004" id="5004">5004</a>
<a href="#5005" id="5005">5005</a>
<a href="#5006" id="5006">5006</a>
<a href="#5007" id="5007">5007</a>
<a href="#5008" id="5008">5008</a>
<a href="#5009" id="5009">5009</a>
<a href="#5010" id="5010">5010</a>
<a href="#5011" id="5011">5011</a>
<a href="#5012" id="5012">5012</a>
<a href="#5013" id="5013">5013</a>
<a href="#5014" id="5014">5014</a>
<a href="#5015" id="5015">5015</a>
<a href="#5016" id="5016">5016</a>
<a href="#5017" id="5017">5017</a>
<a href="#5018" id="5018">5018</a>
<a href="#5019" id="5019">5019</a>
<a href="#5020" id="5020">5020</a>
<a href="#5021" id="5021">5021</a>
<a href="#5022" id="5022">5022</a>
<a href="#5023" id="5023">5023</a>
<a href="#5024" id="5024">5024</a>
<a href="#5025" id="5025">5025</a>
<a href="#5026" id="5026">5026</a>
<a href="#5027" id="5027">5027</a>
<a href="#5028" id="5028">5028</a>
<a href="#5029" id="5029">5029</a>
<a href="#5030" id="5030">5030</a>
<a href="#5031" id="5031">5031</a>
<a href="#5032" id="5032">5032</a>
<a href="#5033" id="5033">5033</a>
<a href="#5034" id="5034">5034</a>
<a href="#5035" id="5035">5035</a>
<a href="#5036" id="5036">5036</a>
<a href="#5037" id="5037">5037</a>
<a href="#5038" id="5038">5038</a>
<a href="#5039" id="5039">5039</a>
<a href="#5040" id="5040">5040</a>
<a href="#5041" id="5041">5041</a>
<a href="#5042" id="5042">5042</a>
<a href="#5043" id="5043">5043</a>
<a href="#5044" id="5044">5044</a>
<a href="#5045" id="5045">5045</a>
<a href="#5046" id="5046">5046</a>
<a href="#5047" id="5047">5047</a>
<a href="#5048" id="5048">5048</a>
<a href="#5049" id="5049">5049</a>
<a href="#5050" id="5050">5050</a>
<a href="#5051" id="5051">5051</a>
<a href="#5052" id="5052">5052</a>
<a href="#5053" id="5053">5053</a>
<a href="#5054" id="5054">5054</a>
<a href="#5055" id="5055">5055</a>
<a href="#5056" id="5056">5056</a>
<a href="#5057" id="5057">5057</a>
<a href="#5058" id="5058">5058</a>
<a href="#5059" id="5059">5059</a>
<a href="#5060" id="5060">5060</a>
<a href="#5061" id="5061">5061</a>
<a href="#5062" id="5062">5062</a>
<a href="#5063" id="5063">5063</a>
<a href="#5064" id="5064">5064</a>
<a href="#5065" id="5065">5065</a>
<a href="#5066" id="5066">5066</a>
<a href="#5067" id="5067">5067</a>
<a href="#5068" id="5068">5068</a>
<a href="#5069" id="5069">5069</a>
<a href="#5070" id="5070">5070</a>
<a href="#5071" id="5071">5071</a>
<a href="#5072" id="5072">5072</a>
<a href="#5073" id="5073">5073</a>
<a href="#5074" id="5074">5074</a>
<a href="#5075" id="5075">5075</a>
<a href="#5076" id="5076">5076</a>
<a href="#5077" id="5077">5077</a>
<a href="#5078" id="5078">5078</a>
<a href="#5079" id="5079">5079</a>
<a href="#5080" id="5080">5080</a>
<a href="#5081" id="5081">5081</a>
<a href="#5082" id="5082">5082</a>
<a href="#5083" id="5083">5083</a>
<a href="#5084" id="5084">5084</a>
<a href="#5085" id="5085">5085</a>
<a href="#5086" id="5086">5086</a>
<a href="#5087" id="5087">5087</a>
<a href="#5088" id="5088">5088</a>
<a href="#5089" id="5089">5089</a>
<a href="#5090" id="5090">5090</a>
<a href="#5091" id="5091">5091</a>
<a href="#5092" id="5092">5092</a>
<a href="#5093" id="5093">5093</a>
<a href="#5094" id="5094">5094</a>
<a href="#5095" id="5095">5095</a>
<a href="#5096" id="5096">5096</a>
<a href="#5097" id="5097">5097</a>
<a href="#5098" id="5098">5098</a>
<a href="#5099" id="5099">5099</a>
<a href="#5100" id="5100">5100</a>
<a href="#5101" id="5101">5101</a>
<a href="#5102" id="5102">5102</a>
<a href="#5103" id="5103">5103</a>
<a href="#5104" id="5104">5104</a>
<a href="#5105" id="5105">5105</a>
<a href="#5106" id="5106">5106</a>
<a href="#5107" id="5107">5107</a>
<a href="#5108" id="5108">5108</a>
<a href="#5109" id="5109">5109</a>
<a href="#5110" id="5110">5110</a>
<a href="#5111" id="5111">5111</a>
<a href="#5112" id="5112">5112</a>
<a href="#5113" id="5113">5113</a>
<a href="#5114" id="5114">5114</a>
<a href="#5115" id="5115">5115</a>
<a href="#5116" id="5116">5116</a>
<a href="#5117" id="5117">5117</a>
<a href="#5118" id="5118">5118</a>
<a href="#5119" id="5119">5119</a>
<a href="#5120" id="5120">5120</a>
<a href="#5121" id="5121">5121</a>
<a href="#5122" id="5122">5122</a>
<a href="#5123" id="5123">5123</a>
<a href="#5124" id="5124">5124</a>
<a href="#5125" id="5125">5125</a>
<a href="#5126" id="5126">5126</a>
<a href="#5127" id="5127">5127</a>
<a href="#5128" id="5128">5128</a>
<a href="#5129" id="5129">5129</a>
<a href="#5130" id="5130">5130</a>
<a href="#5131" id="5131">5131</a>
<a href="#5132" id="5132">5132</a>
<a href="#5133" id="5133">5133</a>
<a href="#5134" id="5134">5134</a>
<a href="#5135" id="5135">5135</a>
<a href="#5136" id="5136">5136</a>
<a href="#5137" id="5137">5137</a>
<a href="#5138" id="5138">5138</a>
<a href="#5139" id="5139">5139</a>
<a href="#5140" id="5140">5140</a>
<a href="#5141" id="5141">5141</a>
<a href="#5142" id="5142">5142</a>
<a href="#5143" id="5143">5143</a>
<a href="#5144" id="5144">5144</a>
<a href="#5145" id="5145">5145</a>
<a href="#5146" id="5146">5146</a>
<a href="#5147" id="5147">5147</a>
<a href="#5148" id="5148">5148</a>
<a href="#5149" id="5149">5149</a>
<a href="#5150" id="5150">5150</a>
<a href="#5151" id="5151">5151</a>
<a href="#5152" id="5152">5152</a>
<a href="#5153" id="5153">5153</a>
<a href="#5154" id="5154">5154</a>
<a href="#5155" id="5155">5155</a>
<a href="#5156" id="5156">5156</a>
<a href="#5157" id="5157">5157</a>
<a href="#5158" id="5158">5158</a>
<a href="#5159" id="5159">5159</a>
<a href="#5160" id="5160">5160</a>
<a href="#5161" id="5161">5161</a>
<a href="#5162" id="5162">5162</a>
<a href="#5163" id="5163">5163</a>
<a href="#5164" id="5164">5164</a>
<a href="#5165" id="5165">5165</a>
<a href="#5166" id="5166">5166</a>
<a href="#5167" id="5167">5167</a>
<a href="#5168" id="5168">5168</a>
<a href="#5169" id="5169">5169</a>
<a href="#5170" id="5170">5170</a>
<a href="#5171" id="5171">5171</a>
<a href="#5172" id="5172">5172</a>
<a href="#5173" id="5173">5173</a>
<a href="#5174" id="5174">5174</a>
<a href="#5175" id="5175">5175</a>
<a href="#5176" id="5176">5176</a>
<a href="#5177" id="5177">5177</a>
<a href="#5178" id="5178">5178</a>
<a href="#5179" id="5179">5179</a>
<a href="#5180" id="5180">5180</a>
<a href="#5181" id="5181">5181</a>
<a href="#5182" id="5182">5182</a>
<a href="#5183" id="5183">5183</a>
<a href="#5184" id="5184">5184</a>
<a href="#5185" id="5185">5185</a>
<a href="#5186" id="5186">5186</a>
<a href="#5187" id="5187">5187</a>
<a href="#5188" id="5188">5188</a>
<a href="#5189" id="5189">5189</a>
<a href="#5190" id="5190">5190</a>
<a href="#5191" id="5191">5191</a>
<a href="#5192" id="5192">5192</a>
<a href="#5193" id="5193">5193</a>
<a href="#5194" id="5194">5194</a>
<a href="#5195" id="5195">5195</a>
<a href="#5196" id="5196">5196</a>
<a href="#5197" id="5197">5197</a>
<a href="#5198" id="5198">5198</a>
<a href="#5199" id="5199">5199</a>
<a href="#5200" id="5200">5200</a>
<a href="#5201" id="5201">5201</a>
<a href="#5202" id="5202">5202</a>
<a href="#5203" id="5203">5203</a>
<a href="#5204" id="5204">5204</a>
<a href="#5205" id="5205">5205</a>
<a href="#5206" id="5206">5206</a>
<a href="#5207" id="5207">5207</a>
<a href="#5208" id="5208">5208</a>
<a href="#5209" id="5209">5209</a>
<a href="#5210" id="5210">5210</a>
<a href="#5211" id="5211">5211</a>
<a href="#5212" id="5212">5212</a>
<a href="#5213" id="5213">5213</a>
<a href="#5214" id="5214">5214</a>
<a href="#5215" id="5215">5215</a>
<a href="#5216" id="5216">5216</a>
<a href="#5217" id="5217">5217</a>
<a href="#5218" id="5218">5218</a>
<a href="#5219" id="5219">5219</a>
<a href="#5220" id="5220">5220</a>
<a href="#5221" id="5221">5221</a>
<a href="#5222" id="5222">5222</a>
<a href="#5223" id="5223">5223</a>
<a href="#5224" id="5224">5224</a>
<a href="#5225" id="5225">5225</a>
<a href="#5226" id="5226">5226</a>
<a href="#5227" id="5227">5227</a>
<a href="#5228" id="5228">5228</a>
<a href="#5229" id="5229">5229</a>
<a href="#5230" id="5230">5230</a>
<a href="#5231" id="5231">5231</a>
<a href="#5232" id="5232">5232</a>
<a href="#5233" id="5233">5233</a>
<a href="#5234" id="5234">5234</a>
<a href="#5235" id="5235">5235</a>
<a href="#5236" id="5236">5236</a>
<a href="#5237" id="5237">5237</a>
<a href="#5238" id="5238">5238</a>
<a href="#5239" id="5239">5239</a>
<a href="#5240" id="5240">5240</a>
<a href="#5241" id="5241">5241</a>
<a href="#5242" id="5242">5242</a>
<a href="#5243" id="5243">5243</a>
<a href="#5244" id="5244">5244</a>
<a href="#5245" id="5245">5245</a>
<a href="#5246" id="5246">5246</a>
<a href="#5247" id="5247">5247</a>
<a href="#5248" id="5248">5248</a>
<a href="#5249" id="5249">5249</a>
<a href="#5250" id="5250">5250</a>
<a href="#5251" id="5251">5251</a>
<a href="#5252" id="5252">5252</a>
<a href="#5253" id="5253">5253</a>
<a href="#5254" id="5254">5254</a>
<a href="#5255" id="5255">5255</a>
<a href="#5256" id="5256">5256</a>
<a href="#5257" id="5257">5257</a>
<a href="#5258" id="5258">5258</a>
<a href="#5259" id="5259">5259</a>
<a href="#5260" id="5260">5260</a>
<a href="#5261" id="5261">5261</a>
<a href="#5262" id="5262">5262</a>
<a href="#5263" id="5263">5263</a>
<a href="#5264" id="5264">5264</a>
<a href="#5265" id="5265">5265</a>
<a href="#5266" id="5266">5266</a>
<a href="#5267" id="5267">5267</a>
<a href="#5268" id="5268">5268</a>
<a href="#5269" id="5269">5269</a>
<a href="#5270" id="5270">5270</a>
<a href="#5271" id="5271">5271</a>
<a href="#5272" id="5272">5272</a>
<a href="#5273" id="5273">5273</a>
<a href="#5274" id="5274">5274</a>
<a href="#5275" id="5275">5275</a>
<a href="#5276" id="5276">5276</a>
<a href="#5277" id="5277">5277</a>
<a href="#5278" id="5278">5278</a>
<a href="#5279" id="5279">5279</a>
<a href="#5280" id="5280">5280</a>
<a href="#5281" id="5281">5281</a>
<a href="#5282" id="5282">5282</a>
<a href="#5283" id="5283">5283</a>
<a href="#5284" id="5284">5284</a>
<a href="#5285" id="5285">5285</a>
<a href="#5286" id="5286">5286</a>
<a href="#5287" id="5287">5287</a>
<a href="#5288" id="5288">5288</a>
<a href="#5289" id="5289">5289</a>
<a href="#5290" id="5290">5290</a>
<a href="#5291" id="5291">5291</a>
<a href="#5292" id="5292">5292</a>
<a href="#5293" id="5293">5293</a>
<a href="#5294" id="5294">5294</a>
<a href="#5295" id="5295">5295</a>
<a href="#5296" id="5296">5296</a>
<a href="#5297" id="5297">5297</a>
<a href="#5298" id="5298">5298</a>
<a href="#5299" id="5299">5299</a>
<a href="#5300" id="5300">5300</a>
<a href="#5301" id="5301">5301</a>
<a href="#5302" id="5302">5302</a>
<a href="#5303" id="5303">5303</a>
<a href="#5304" id="5304">5304</a>
<a href="#5305" id="5305">5305</a>
<a href="#5306" id="5306">5306</a>
<a href="#5307" id="5307">5307</a>
<a href="#5308" id="5308">5308</a>
<a href="#5309" id="5309">5309</a>
<a href="#5310" id="5310">5310</a>
<a href="#5311" id="5311">5311</a>
<a href="#5312" id="5312">5312</a>
<a href="#5313" id="5313">5313</a>
<a href="#5314" id="5314">5314</a>
<a href="#5315" id="5315">5315</a>
<a href="#5316" id="5316">5316</a>
<a href="#5317" id="5317">5317</a>
<a href="#5318" id="5318">5318</a>
<a href="#5319" id="5319">5319</a>
<a href="#5320" id="5320">5320</a>
<a href="#5321" id="5321">5321</a>
<a href="#5322" id="5322">5322</a>
<a href="#5323" id="5323">5323</a>
<a href="#5324" id="5324">5324</a>
<a href="#5325" id="5325">5325</a>
<a href="#5326" id="5326">5326</a>
<a href="#5327" id="5327">5327</a>
<a href="#5328" id="5328">5328</a>
<a href="#5329" id="5329">5329</a>
<a href="#5330" id="5330">5330</a>
<a href="#5331" id="5331">5331</a>
<a href="#5332" id="5332">5332</a>
<a href="#5333" id="5333">5333</a>
<a href="#5334" id="5334">5334</a>
<a href="#5335" id="5335">5335</a>
<a href="#5336" id="5336">5336</a>
<a href="#5337" id="5337">5337</a>
<a href="#5338" id="5338">5338</a>
<a href="#5339" id="5339">5339</a>
<a href="#5340" id="5340">5340</a>
<a href="#5341" id="5341">5341</a>
<a href="#5342" id="5342">5342</a>
<a href="#5343" id="5343">5343</a>
<a href="#5344" id="5344">5344</a>
<a href="#5345" id="5345">5345</a>
<a href="#5346" id="5346">5346</a>
<a href="#5347" id="5347">5347</a>
<a href="#5348" id="5348">5348</a>
<a href="#5349" id="5349">5349</a>
<a href="#5350" id="5350">5350</a>
<a href="#5351" id="5351">5351</a>
<a href="#5352" id="5352">5352</a>
<a href="#5353" id="5353">5353</a>
<a href="#5354" id="5354">5354</a>
<a href="#5355" id="5355">5355</a>
<a href="#5356" id="5356">5356</a>
<a href="#5357" id="5357">5357</a>
<a href="#5358" id="5358">5358</a>
<a href="#5359" id="5359">5359</a>
<a href="#5360" id="5360">5360</a>
<a href="#5361" id="5361">5361</a>
<a href="#5362" id="5362">5362</a>
<a href="#5363" id="5363">5363</a>
<a href="#5364" id="5364">5364</a>
<a href="#5365" id="5365">5365</a>
<a href="#5366" id="5366">5366</a>
<a href="#5367" id="5367">5367</a>
<a href="#5368" id="5368">5368</a>
<a href="#5369" id="5369">5369</a>
<a href="#5370" id="5370">5370</a>
<a href="#5371" id="5371">5371</a>
<a href="#5372" id="5372">5372</a>
<a href="#5373" id="5373">5373</a>
<a href="#5374" id="5374">5374</a>
<a href="#5375" id="5375">5375</a>
<a href="#5376" id="5376">5376</a>
<a href="#5377" id="5377">5377</a>
<a href="#5378" id="5378">5378</a>
<a href="#5379" id="5379">5379</a>
<a href="#5380" id="5380">5380</a>
<a href="#5381" id="5381">5381</a>
<a href="#5382" id="5382">5382</a>
<a href="#5383" id="5383">5383</a>
<a href="#5384" id="5384">5384</a>
<a href="#5385" id="5385">5385</a>
<a href="#5386" id="5386">5386</a>
<a href="#5387" id="5387">5387</a>
<a href="#5388" id="5388">5388</a>
<a href="#5389" id="5389">5389</a>
<a href="#5390" id="5390">5390</a>
<a href="#5391" id="5391">5391</a>
<a href="#5392" id="5392">5392</a>
<a href="#5393" id="5393">5393</a>
<a href="#5394" id="5394">5394</a>
<a href="#5395" id="5395">5395</a>
<a href="#5396" id="5396">5396</a>
<a href="#5397" id="5397">5397</a>
<a href="#5398" id="5398">5398</a>
<a href="#5399" id="5399">5399</a>
<a href="#5400" id="5400">5400</a>
<a href="#5401" id="5401">5401</a>
<a href="#5402" id="5402">5402</a>
<a href="#5403" id="5403">5403</a>
<a href="#5404" id="5404">5404</a>
<a href="#5405" id="5405">5405</a>
<a href="#5406" id="5406">5406</a>
<a href="#5407" id="5407">5407</a>
<a href="#5408" id="5408">5408</a>
<a href="#5409" id="5409">5409</a>
<a href="#5410" id="5410">5410</a>
<a href="#5411" id="5411">5411</a>
<a href="#5412" id="5412">5412</a>
<a href="#5413" id="5413">5413</a>
<a href="#5414" id="5414">5414</a>
<a href="#5415" id="5415">5415</a>
<a href="#5416" id="5416">5416</a>
<a href="#5417" id="5417">5417</a>
<a href="#5418" id="5418">5418</a>
<a href="#5419" id="5419">5419</a>
<a href="#5420" id="5420">5420</a>
<a href="#5421" id="5421">5421</a>
<a href="#5422" id="5422">5422</a>
<a href="#5423" id="5423">5423</a>
<a href="#5424" id="5424">5424</a>
<a href="#5425" id="5425">5425</a>
<a href="#5426" id="5426">5426</a>
<a href="#5427" id="5427">5427</a>
<a href="#5428" id="5428">5428</a>
<a href="#5429" id="5429">5429</a>
<a href="#5430" id="5430">5430</a>
<a href="#5431" id="5431">5431</a>
<a href="#5432" id="5432">5432</a>
<a href="#5433" id="5433">5433</a>
<a href="#5434" id="5434">5434</a>
<a href="#5435" id="5435">5435</a>
<a href="#5436" id="5436">5436</a>
<a href="#5437" id="5437">5437</a>
<a href="#5438" id="5438">5438</a>
<a href="#5439" id="5439">5439</a>
<a href="#5440" id="5440">5440</a>
<a href="#5441" id="5441">5441</a>
<a href="#5442" id="5442">5442</a>
<a href="#5443" id="5443">5443</a>
<a href="#5444" id="5444">5444</a>
<a href="#5445" id="5445">5445</a>
<a href="#5446" id="5446">5446</a>
<a href="#5447" id="5447">5447</a>
<a href="#5448" id="5448">5448</a>
<a href="#5449" id="5449">5449</a>
<a href="#5450" id="5450">5450</a>
<a href="#5451" id="5451">5451</a>
<a href="#5452" id="5452">5452</a>
<a href="#5453" id="5453">5453</a>
<a href="#5454" id="5454">5454</a>
<a href="#5455" id="5455">5455</a>
<a href="#5456" id="5456">5456</a>
<a href="#5457" id="5457">5457</a>
<a href="#5458" id="5458">5458</a>
<a href="#5459" id="5459">5459</a>
<a href="#5460" id="5460">5460</a>
<a href="#5461" id="5461">5461</a>
<a href="#5462" id="5462">5462</a>
<a href="#5463" id="5463">5463</a>
<a href="#5464" id="5464">5464</a>
<a href="#5465" id="5465">5465</a>
<a href="#5466" id="5466">5466</a>
<a href="#5467" id="5467">5467</a>
<a href="#5468" id="5468">5468</a>
<a href="#5469" id="5469">5469</a>
<a href="#5470" id="5470">5470</a>
<a href="#5471" id="5471">5471</a>
<a href="#5472" id="5472">5472</a>
<a href="#5473" id="5473">5473</a>
<a href="#5474" id="5474">5474</a>
<a href="#5475" id="5475">5475</a>
<a href="#5476" id="5476">5476</a>
<a href="#5477" id="5477">5477</a>
<a href="#5478" id="5478">5478</a>
<a href="#5479" id="5479">5479</a>
<a href="#5480" id="5480">5480</a>
<a href="#5481" id="5481">5481</a>
<a href="#5482" id="5482">5482</a>
<a href="#5483" id="5483">5483</a>
<a href="#5484" id="5484">5484</a>
<a href="#5485" id="5485">5485</a>
<a href="#5486" id="5486">5486</a>
<a href="#5487" id="5487">5487</a>
<a href="#5488" id="5488">5488</a>
<a href="#5489" id="5489">5489</a>
<a href="#5490" id="5490">5490</a>
<a href="#5491" id="5491">5491</a>
<a href="#5492" id="5492">5492</a>
<a href="#5493" id="5493">5493</a>
<a href="#5494" id="5494">5494</a>
<a href="#5495" id="5495">5495</a>
<a href="#5496" id="5496">5496</a>
<a href="#5497" id="5497">5497</a>
<a href="#5498" id="5498">5498</a>
<a href="#5499" id="5499">5499</a>
<a href="#5500" id="5500">5500</a>
<a href="#5501" id="5501">5501</a>
<a href="#5502" id="5502">5502</a>
<a href="#5503" id="5503">5503</a>
<a href="#5504" id="5504">5504</a>
<a href="#5505" id="5505">5505</a>
<a href="#5506" id="5506">5506</a>
<a href="#5507" id="5507">5507</a>
<a href="#5508" id="5508">5508</a>
<a href="#5509" id="5509">5509</a>
<a href="#5510" id="5510">5510</a>
<a href="#5511" id="5511">5511</a>
<a href="#5512" id="5512">5512</a>
<a href="#5513" id="5513">5513</a>
<a href="#5514" id="5514">5514</a>
<a href="#5515" id="5515">5515</a>
<a href="#5516" id="5516">5516</a>
<a href="#5517" id="5517">5517</a>
<a href="#5518" id="5518">5518</a>
<a href="#5519" id="5519">5519</a>
<a href="#5520" id="5520">5520</a>
<a href="#5521" id="5521">5521</a>
<a href="#5522" id="5522">5522</a>
<a href="#5523" id="5523">5523</a>
<a href="#5524" id="5524">5524</a>
<a href="#5525" id="5525">5525</a>
<a href="#5526" id="5526">5526</a>
<a href="#5527" id="5527">5527</a>
<a href="#5528" id="5528">5528</a>
<a href="#5529" id="5529">5529</a>
<a href="#5530" id="5530">5530</a>
<a href="#5531" id="5531">5531</a>
<a href="#5532" id="5532">5532</a>
<a href="#5533" id="5533">5533</a>
<a href="#5534" id="5534">5534</a>
<a href="#5535" id="5535">5535</a>
<a href="#5536" id="5536">5536</a>
<a href="#5537" id="5537">5537</a>
<a href="#5538" id="5538">5538</a>
<a href="#5539" id="5539">5539</a>
<a href="#5540" id="5540">5540</a>
<a href="#5541" id="5541">5541</a>
<a href="#5542" id="5542">5542</a>
<a href="#5543" id="5543">5543</a>
<a href="#5544" id="5544">5544</a>
<a href="#5545" id="5545">5545</a>
<a href="#5546" id="5546">5546</a>
<a href="#5547" id="5547">5547</a>
<a href="#5548" id="5548">5548</a>
<a href="#5549" id="5549">5549</a>
<a href="#5550" id="5550">5550</a>
<a href="#5551" id="5551">5551</a>
<a href="#5552" id="5552">5552</a>
<a href="#5553" id="5553">5553</a>
<a href="#5554" id="5554">5554</a>
<a href="#5555" id="5555">5555</a>
<a href="#5556" id="5556">5556</a>
<a href="#5557" id="5557">5557</a>
<a href="#5558" id="5558">5558</a>
<a href="#5559" id="5559">5559</a>
<a href="#5560" id="5560">5560</a>
<a href="#5561" id="5561">5561</a>
<a href="#5562" id="5562">5562</a>
<a href="#5563" id="5563">5563</a>
<a href="#5564" id="5564">5564</a>
<a href="#5565" id="5565">5565</a>
<a href="#5566" id="5566">5566</a>
<a href="#5567" id="5567">5567</a>
<a href="#5568" id="5568">5568</a>
<a href="#5569" id="5569">5569</a>
<a href="#5570" id="5570">5570</a>
<a href="#5571" id="5571">5571</a>
<a href="#5572" id="5572">5572</a>
<a href="#5573" id="5573">5573</a>
<a href="#5574" id="5574">5574</a>
<a href="#5575" id="5575">5575</a>
<a href="#5576" id="5576">5576</a>
<a href="#5577" id="5577">5577</a>
<a href="#5578" id="5578">5578</a>
<a href="#5579" id="5579">5579</a>
<a href="#5580" id="5580">5580</a>
<a href="#5581" id="5581">5581</a>
<a href="#5582" id="5582">5582</a>
<a href="#5583" id="5583">5583</a>
<a href="#5584" id="5584">5584</a>
<a href="#5585" id="5585">5585</a>
<a href="#5586" id="5586">5586</a>
<a href="#5587" id="5587">5587</a>
<a href="#5588" id="5588">5588</a>
<a href="#5589" id="5589">5589</a>
<a href="#5590" id="5590">5590</a>
<a href="#5591" id="5591">5591</a>
<a href="#5592" id="5592">5592</a>
<a href="#5593" id="5593">5593</a>
<a href="#5594" id="5594">5594</a>
<a href="#5595" id="5595">5595</a>
<a href="#5596" id="5596">5596</a>
<a href="#5597" id="5597">5597</a>
<a href="#5598" id="5598">5598</a>
<a href="#5599" id="5599">5599</a>
<a href="#5600" id="5600">5600</a>
<a href="#5601" id="5601">5601</a>
<a href="#5602" id="5602">5602</a>
<a href="#5603" id="5603">5603</a>
<a href="#5604" id="5604">5604</a>
<a href="#5605" id="5605">5605</a>
<a href="#5606" id="5606">5606</a>
<a href="#5607" id="5607">5607</a>
<a href="#5608" id="5608">5608</a>
<a href="#5609" id="5609">5609</a>
<a href="#5610" id="5610">5610</a>
<a href="#5611" id="5611">5611</a>
<a href="#5612" id="5612">5612</a>
<a href="#5613" id="5613">5613</a>
<a href="#5614" id="5614">5614</a>
<a href="#5615" id="5615">5615</a>
<a href="#5616" id="5616">5616</a>
<a href="#5617" id="5617">5617</a>
<a href="#5618" id="5618">5618</a>
<a href="#5619" id="5619">5619</a>
<a href="#5620" id="5620">5620</a>
<a href="#5621" id="5621">5621</a>
<a href="#5622" id="5622">5622</a>
<a href="#5623" id="5623">5623</a>
<a href="#5624" id="5624">5624</a>
<a href="#5625" id="5625">5625</a>
<a href="#5626" id="5626">5626</a>
<a href="#5627" id="5627">5627</a>
<a href="#5628" id="5628">5628</a>
<a href="#5629" id="5629">5629</a>
<a href="#5630" id="5630">5630</a>
<a href="#5631" id="5631">5631</a>
<a href="#5632" id="5632">5632</a>
<a href="#5633" id="5633">5633</a>
<a href="#5634" id="5634">5634</a>
<a href="#5635" id="5635">5635</a>
<a href="#5636" id="5636">5636</a>
<a href="#5637" id="5637">5637</a>
<a href="#5638" id="5638">5638</a>
<a href="#5639" id="5639">5639</a>
<a href="#5640" id="5640">5640</a>
<a href="#5641" id="5641">5641</a>
<a href="#5642" id="5642">5642</a>
<a href="#5643" id="5643">5643</a>
<a href="#5644" id="5644">5644</a>
<a href="#5645" id="5645">5645</a>
<a href="#5646" id="5646">5646</a>
<a href="#5647" id="5647">5647</a>
<a href="#5648" id="5648">5648</a>
<a href="#5649" id="5649">5649</a>
<a href="#5650" id="5650">5650</a>
<a href="#5651" id="5651">5651</a>
<a href="#5652" id="5652">5652</a>
<a href="#5653" id="5653">5653</a>
<a href="#5654" id="5654">5654</a>
<a href="#5655" id="5655">5655</a>
<a href="#5656" id="5656">5656</a>
<a href="#5657" id="5657">5657</a>
<a href="#5658" id="5658">5658</a>
<a href="#5659" id="5659">5659</a>
<a href="#5660" id="5660">5660</a>
<a href="#5661" id="5661">5661</a>
<a href="#5662" id="5662">5662</a>
<a href="#5663" id="5663">5663</a>
<a href="#5664" id="5664">5664</a>
<a href="#5665" id="5665">5665</a>
<a href="#5666" id="5666">5666</a>
<a href="#5667" id="5667">5667</a>
<a href="#5668" id="5668">5668</a>
<a href="#5669" id="5669">5669</a>
<a href="#5670" id="5670">5670</a>
<a href="#5671" id="5671">5671</a>
<a href="#5672" id="5672">5672</a>
<a href="#5673" id="5673">5673</a>
<a href="#5674" id="5674">5674</a>
<a href="#5675" id="5675">5675</a>
<a href="#5676" id="5676">5676</a>
<a href="#5677" id="5677">5677</a>
<a href="#5678" id="5678">5678</a>
<a href="#5679" id="5679">5679</a>
<a href="#5680" id="5680">5680</a>
<a href="#5681" id="5681">5681</a>
<a href="#5682" id="5682">5682</a>
<a href="#5683" id="5683">5683</a>
<a href="#5684" id="5684">5684</a>
<a href="#5685" id="5685">5685</a>
<a href="#5686" id="5686">5686</a>
<a href="#5687" id="5687">5687</a>
<a href="#5688" id="5688">5688</a>
<a href="#5689" id="5689">5689</a>
<a href="#5690" id="5690">5690</a>
<a href="#5691" id="5691">5691</a>
<a href="#5692" id="5692">5692</a>
<a href="#5693" id="5693">5693</a>
<a href="#5694" id="5694">5694</a>
<a href="#5695" id="5695">5695</a>
<a href="#5696" id="5696">5696</a>
<a href="#5697" id="5697">5697</a>
<a href="#5698" id="5698">5698</a>
<a href="#5699" id="5699">5699</a>
<a href="#5700" id="5700">5700</a>
<a href="#5701" id="5701">5701</a>
<a href="#5702" id="5702">5702</a>
<a href="#5703" id="5703">5703</a>
<a href="#5704" id="5704">5704</a>
<a href="#5705" id="5705">5705</a>
<a href="#5706" id="5706">5706</a>
<a href="#5707" id="5707">5707</a>
<a href="#5708" id="5708">5708</a>
<a href="#5709" id="5709">5709</a>
<a href="#5710" id="5710">5710</a>
<a href="#5711" id="5711">5711</a>
<a href="#5712" id="5712">5712</a>
<a href="#5713" id="5713">5713</a>
<a href="#5714" id="5714">5714</a>
<a href="#5715" id="5715">5715</a>
<a href="#5716" id="5716">5716</a>
<a href="#5717" id="5717">5717</a>
<a href="#5718" id="5718">5718</a>
<a href="#5719" id="5719">5719</a>
<a href="#5720" id="5720">5720</a>
<a href="#5721" id="5721">5721</a>
<a href="#5722" id="5722">5722</a>
<a href="#5723" id="5723">5723</a>
<a href="#5724" id="5724">5724</a>
<a href="#5725" id="5725">5725</a>
<a href="#5726" id="5726">5726</a>
<a href="#5727" id="5727">5727</a>
<a href="#5728" id="5728">5728</a>
<a href="#5729" id="5729">5729</a>
<a href="#5730" id="5730">5730</a>
<a href="#5731" id="5731">5731</a>
<a href="#5732" id="5732">5732</a>
<a href="#5733" id="5733">5733</a>
<a href="#5734" id="5734">5734</a>
<a href="#5735" id="5735">5735</a>
<a href="#5736" id="5736">5736</a>
<a href="#5737" id="5737">5737</a>
<a href="#5738" id="5738">5738</a>
<a href="#5739" id="5739">5739</a>
<a href="#5740" id="5740">5740</a>
<a href="#5741" id="5741">5741</a>
<a href="#5742" id="5742">5742</a>
<a href="#5743" id="5743">5743</a>
<a href="#5744" id="5744">5744</a>
<a href="#5745" id="5745">5745</a>
<a href="#5746" id="5746">5746</a>
<a href="#5747" id="5747">5747</a>
<a href="#5748" id="5748">5748</a>
<a href="#5749" id="5749">5749</a>
<a href="#5750" id="5750">5750</a>
<a href="#5751" id="5751">5751</a>
<a href="#5752" id="5752">5752</a>
<a href="#5753" id="5753">5753</a>
<a href="#5754" id="5754">5754</a>
<a href="#5755" id="5755">5755</a>
<a href="#5756" id="5756">5756</a>
<a href="#5757" id="5757">5757</a>
<a href="#5758" id="5758">5758</a>
<a href="#5759" id="5759">5759</a>
<a href="#5760" id="5760">5760</a>
<a href="#5761" id="5761">5761</a>
<a href="#5762" id="5762">5762</a>
<a href="#5763" id="5763">5763</a>
<a href="#5764" id="5764">5764</a>
<a href="#5765" id="5765">5765</a>
<a href="#5766" id="5766">5766</a>
<a href="#5767" id="5767">5767</a>
<a href="#5768" id="5768">5768</a>
<a href="#5769" id="5769">5769</a>
<a href="#5770" id="5770">5770</a>
<a href="#5771" id="5771">5771</a>
<a href="#5772" id="5772">5772</a>
<a href="#5773" id="5773">5773</a>
<a href="#5774" id="5774">5774</a>
<a href="#5775" id="5775">5775</a>
<a href="#5776" id="5776">5776</a>
<a href="#5777" id="5777">5777</a>
<a href="#5778" id="5778">5778</a>
<a href="#5779" id="5779">5779</a>
<a href="#5780" id="5780">5780</a>
<a href="#5781" id="5781">5781</a>
<a href="#5782" id="5782">5782</a>
<a href="#5783" id="5783">5783</a>
<a href="#5784" id="5784">5784</a>
<a href="#5785" id="5785">5785</a>
<a href="#5786" id="5786">5786</a>
<a href="#5787" id="5787">5787</a>
<a href="#5788" id="5788">5788</a>
<a href="#5789" id="5789">5789</a>
<a href="#5790" id="5790">5790</a>
<a href="#5791" id="5791">5791</a>
<a href="#5792" id="5792">5792</a>
<a href="#5793" id="5793">5793</a>
<a href="#5794" id="5794">5794</a>
<a href="#5795" id="5795">5795</a>
<a href="#5796" id="5796">5796</a>
<a href="#5797" id="5797">5797</a>
<a href="#5798" id="5798">5798</a>
<a href="#5799" id="5799">5799</a>
<a href="#5800" id="5800">5800</a>
<a href="#5801" id="5801">5801</a>
<a href="#5802" id="5802">5802</a>
<a href="#5803" id="5803">5803</a>
<a href="#5804" id="5804">5804</a>
<a href="#5805" id="5805">5805</a>
<a href="#5806" id="5806">5806</a>
<a href="#5807" id="5807">5807</a>
<a href="#5808" id="5808">5808</a>
<a href="#5809" id="5809">5809</a>
<a href="#5810" id="5810">5810</a>
<a href="#5811" id="5811">5811</a>
<a href="#5812" id="5812">5812</a>
<a href="#5813" id="5813">5813</a>
<a href="#5814" id="5814">5814</a>
<a href="#5815" id="5815">5815</a>
<a href="#5816" id="5816">5816</a>
<a href="#5817" id="5817">5817</a>
<a href="#5818" id="5818">5818</a>
<a href="#5819" id="5819">5819</a>
<a href="#5820" id="5820">5820</a>
<a href="#5821" id="5821">5821</a>
<a href="#5822" id="5822">5822</a>
<a href="#5823" id="5823">5823</a>
<a href="#5824" id="5824">5824</a>
<a href="#5825" id="5825">5825</a>
<a href="#5826" id="5826">5826</a>
<a href="#5827" id="5827">5827</a>
<a href="#5828" id="5828">5828</a>
<a href="#5829" id="5829">5829</a>
<a href="#5830" id="5830">5830</a>
<a href="#5831" id="5831">5831</a>
<a href="#5832" id="5832">5832</a>
<a href="#5833" id="5833">5833</a>
<a href="#5834" id="5834">5834</a>
<a href="#5835" id="5835">5835</a>
<a href="#5836" id="5836">5836</a>
<a href="#5837" id="5837">5837</a>
<a href="#5838" id="5838">5838</a>
<a href="#5839" id="5839">5839</a>
<a href="#5840" id="5840">5840</a>
<a href="#5841" id="5841">5841</a>
<a href="#5842" id="5842">5842</a>
<a href="#5843" id="5843">5843</a>
<a href="#5844" id="5844">5844</a>
<a href="#5845" id="5845">5845</a>
<a href="#5846" id="5846">5846</a>
<a href="#5847" id="5847">5847</a>
<a href="#5848" id="5848">5848</a>
<a href="#5849" id="5849">5849</a>
<a href="#5850" id="5850">5850</a>
<a href="#5851" id="5851">5851</a>
<a href="#5852" id="5852">5852</a>
<a href="#5853" id="5853">5853</a>
<a href="#5854" id="5854">5854</a>
<a href="#5855" id="5855">5855</a>
<a href="#5856" id="5856">5856</a>
<a href="#5857" id="5857">5857</a>
<a href="#5858" id="5858">5858</a>
<a href="#5859" id="5859">5859</a>
<a href="#5860" id="5860">5860</a>
<a href="#5861" id="5861">5861</a>
<a href="#5862" id="5862">5862</a>
<a href="#5863" id="5863">5863</a>
<a href="#5864" id="5864">5864</a>
<a href="#5865" id="5865">5865</a>
<a href="#5866" id="5866">5866</a>
<a href="#5867" id="5867">5867</a>
<a href="#5868" id="5868">5868</a>
<a href="#5869" id="5869">5869</a>
<a href="#5870" id="5870">5870</a>
<a href="#5871" id="5871">5871</a>
<a href="#5872" id="5872">5872</a>
<a href="#5873" id="5873">5873</a>
<a href="#5874" id="5874">5874</a>
<a href="#5875" id="5875">5875</a>
<a href="#5876" id="5876">5876</a>
<a href="#5877" id="5877">5877</a>
<a href="#5878" id="5878">5878</a>
<a href="#5879" id="5879">5879</a>
<a href="#5880" id="5880">5880</a>
<a href="#5881" id="5881">5881</a>
<a href="#5882" id="5882">5882</a>
<a href="#5883" id="5883">5883</a>
<a href="#5884" id="5884">5884</a>
<a href="#5885" id="5885">5885</a>
<a href="#5886" id="5886">5886</a>
<a href="#5887" id="5887">5887</a>
<a href="#5888" id="5888">5888</a>
<a href="#5889" id="5889">5889</a>
<a href="#5890" id="5890">5890</a>
<a href="#5891" id="5891">5891</a>
<a href="#5892" id="5892">5892</a>
<a href="#5893" id="5893">5893</a>
<a href="#5894" id="5894">5894</a>
<a href="#5895" id="5895">5895</a>
<a href="#5896" id="5896">5896</a>
<a href="#5897" id="5897">5897</a>
<a href="#5898" id="5898">5898</a>
<a href="#5899" id="5899">5899</a>
<a href="#5900" id="5900">5900</a>
<a href="#5901" id="5901">5901</a>
<a href="#5902" id="5902">5902</a>
<a href="#5903" id="5903">5903</a>
<a href="#5904" id="5904">5904</a>
<a href="#5905" id="5905">5905</a>
<a href="#5906" id="5906">5906</a>
<a href="#5907" id="5907">5907</a>
<a href="#5908" id="5908">5908</a>
<a href="#5909" id="5909">5909</a>
<a href="#5910" id="5910">5910</a>
<a href="#5911" id="5911">5911</a>
<a href="#5912" id="5912">5912</a>
<a href="#5913" id="5913">5913</a>
<a href="#5914" id="5914">5914</a>
<a href="#5915" id="5915">5915</a>
<a href="#5916" id="5916">5916</a>
<a href="#5917" id="5917">5917</a>
<a href="#5918" id="5918">5918</a>
<a href="#5919" id="5919">5919</a>
<a href="#5920" id="5920">5920</a>
<a href="#5921" id="5921">5921</a>
<a href="#5922" id="5922">5922</a>
<a href="#5923" id="5923">5923</a>
<a href="#5924" id="5924">5924</a>
<a href="#5925" id="5925">5925</a>
<a href="#5926" id="5926">5926</a>
<a href="#5927" id="5927">5927</a>
<a href="#5928" id="5928">5928</a>
<a href="#5929" id="5929">5929</a>
<a href="#5930" id="5930">5930</a>
<a href="#5931" id="5931">5931</a>
<a href="#5932" id="5932">5932</a>
<a href="#5933" id="5933">5933</a>
<a href="#5934" id="5934">5934</a>
<a href="#5935" id="5935">5935</a>
<a href="#5936" id="5936">5936</a>
<a href="#5937" id="5937">5937</a>
<a href="#5938" id="5938">5938</a>
<a href="#5939" id="5939">5939</a>
<a href="#5940" id="5940">5940</a>
<a href="#5941" id="5941">5941</a>
<a href="#5942" id="5942">5942</a>
<a href="#5943" id="5943">5943</a>
<a href="#5944" id="5944">5944</a>
<a href="#5945" id="5945">5945</a>
<a href="#5946" id="5946">5946</a>
<a href="#5947" id="5947">5947</a>
<a href="#5948" id="5948">5948</a>
<a href="#5949" id="5949">5949</a>
<a href="#5950" id="5950">5950</a>
<a href="#5951" id="5951">5951</a>
<a href="#5952" id="5952">5952</a>
<a href="#5953" id="5953">5953</a>
<a href="#5954" id="5954">5954</a>
<a href="#5955" id="5955">5955</a>
<a href="#5956" id="5956">5956</a>
<a href="#5957" id="5957">5957</a>
<a href="#5958" id="5958">5958</a>
<a href="#5959" id="5959">5959</a>
<a href="#5960" id="5960">5960</a>
<a href="#5961" id="5961">5961</a>
<a href="#5962" id="5962">5962</a>
<a href="#5963" id="5963">5963</a>
<a href="#5964" id="5964">5964</a>
<a href="#5965" id="5965">5965</a>
<a href="#5966" id="5966">5966</a>
<a href="#5967" id="5967">5967</a>
<a href="#5968" id="5968">5968</a>
<a href="#5969" id="5969">5969</a>
<a href="#5970" id="5970">5970</a>
<a href="#5971" id="5971">5971</a>
<a href="#5972" id="5972">5972</a>
<a href="#5973" id="5973">5973</a>
<a href="#5974" id="5974">5974</a>
<a href="#5975" id="5975">5975</a>
<a href="#5976" id="5976">5976</a>
<a href="#5977" id="5977">5977</a>
<a href="#5978" id="5978">5978</a>
<a href="#5979" id="5979">5979</a>
<a href="#5980" id="5980">5980</a>
<a href="#5981" id="5981">5981</a>
<a href="#5982" id="5982">5982</a>
<a href="#5983" id="5983">5983</a>
<a href="#5984" id="5984">5984</a>
<a href="#5985" id="5985">5985</a>
<a href="#5986" id="5986">5986</a>
<a href="#5987" id="5987">5987</a>
<a href="#5988" id="5988">5988</a>
<a href="#5989" id="5989">5989</a>
<a href="#5990" id="5990">5990</a>
<a href="#5991" id="5991">5991</a>
<a href="#5992" id="5992">5992</a>
<a href="#5993" id="5993">5993</a>
<a href="#5994" id="5994">5994</a>
<a href="#5995" id="5995">5995</a>
<a href="#5996" id="5996">5996</a>
<a href="#5997" id="5997">5997</a>
<a href="#5998" id="5998">5998</a>
<a href="#5999" id="5999">5999</a>
<a href="#6000" id="6000">6000</a>
<a href="#6001" id="6001">6001</a>
<a href="#6002" id="6002">6002</a>
<a href="#6003" id="6003">6003</a>
<a href="#6004" id="6004">6004</a>
<a href="#6005" id="6005">6005</a>
<a href="#6006" id="6006">6006</a>
<a href="#6007" id="6007">6007</a>
<a href="#6008" id="6008">6008</a>
<a href="#6009" id="6009">6009</a>
<a href="#6010" id="6010">6010</a>
<a href="#6011" id="6011">6011</a>
<a href="#6012" id="6012">6012</a>
<a href="#6013" id="6013">6013</a>
<a href="#6014" id="6014">6014</a>
<a href="#6015" id="6015">6015</a>
<a href="#6016" id="6016">6016</a>
<a href="#6017" id="6017">6017</a>
<a href="#6018" id="6018">6018</a>
<a href="#6019" id="6019">6019</a>
<a href="#6020" id="6020">6020</a>
<a href="#6021" id="6021">6021</a>
<a href="#6022" id="6022">6022</a>
<a href="#6023" id="6023">6023</a>
<a href="#6024" id="6024">6024</a>
<a href="#6025" id="6025">6025</a>
<a href="#6026" id="6026">6026</a>
<a href="#6027" id="6027">6027</a>
<a href="#6028" id="6028">6028</a>
<a href="#6029" id="6029">6029</a>
<a href="#6030" id="6030">6030</a>
<a href="#6031" id="6031">6031</a>
<a href="#6032" id="6032">6032</a>
<a href="#6033" id="6033">6033</a>
<a href="#6034" id="6034">6034</a>
<a href="#6035" id="6035">6035</a>
<a href="#6036" id="6036">6036</a>
<a href="#6037" id="6037">6037</a>
<a href="#6038" id="6038">6038</a>
<a href="#6039" id="6039">6039</a>
<a href="#6040" id="6040">6040</a>
<a href="#6041" id="6041">6041</a>
<a href="#6042" id="6042">6042</a>
<a href="#6043" id="6043">6043</a>
<a href="#6044" id="6044">6044</a>
<a href="#6045" id="6045">6045</a>
<a href="#6046" id="6046">6046</a>
<a href="#6047" id="6047">6047</a>
<a href="#6048" id="6048">6048</a>
<a href="#6049" id="6049">6049</a>
<a href="#6050" id="6050">6050</a>
<a href="#6051" id="6051">6051</a>
<a href="#6052" id="6052">6052</a>
<a href="#6053" id="6053">6053</a>
<a href="#6054" id="6054">6054</a>
<a href="#6055" id="6055">6055</a>
<a href="#6056" id="6056">6056</a>
<a href="#6057" id="6057">6057</a>
<a href="#6058" id="6058">6058</a>
<a href="#6059" id="6059">6059</a>
<a href="#6060" id="6060">6060</a>
<a href="#6061" id="6061">6061</a>
<a href="#6062" id="6062">6062</a>
<a href="#6063" id="6063">6063</a>
<a href="#6064" id="6064">6064</a>
<a href="#6065" id="6065">6065</a>
<a href="#6066" id="6066">6066</a>
<a href="#6067" id="6067">6067</a>
<a href="#6068" id="6068">6068</a>
<a href="#6069" id="6069">6069</a>
<a href="#6070" id="6070">6070</a>
<a href="#6071" id="6071">6071</a>
<a href="#6072" id="6072">6072</a>
<a href="#6073" id="6073">6073</a>
<a href="#6074" id="6074">6074</a>
<a href="#6075" id="6075">6075</a>
<a href="#6076" id="6076">6076</a>
<a href="#6077" id="6077">6077</a>
<a href="#6078" id="6078">6078</a>
<a href="#6079" id="6079">6079</a>
<a href="#6080" id="6080">6080</a>
<a href="#6081" id="6081">6081</a>
<a href="#6082" id="6082">6082</a>
<a href="#6083" id="6083">6083</a>
<a href="#6084" id="6084">6084</a>
<a href="#6085" id="6085">6085</a>
<a href="#6086" id="6086">6086</a>
<a href="#6087" id="6087">6087</a>
<a href="#6088" id="6088">6088</a>
<a href="#6089" id="6089">6089</a>
<a href="#6090" id="6090">6090</a>
<a href="#6091" id="6091">6091</a>
<a href="#6092" id="6092">6092</a>
<a href="#6093" id="6093">6093</a>
<a href="#6094" id="6094">6094</a>
<a href="#6095" id="6095">6095</a>
<a href="#6096" id="6096">6096</a>
<a href="#6097" id="6097">6097</a>
<a href="#6098" id="6098">6098</a>
<a href="#6099" id="6099">6099</a>
<a href="#6100" id="6100">6100</a>
<a href="#6101" id="6101">6101</a>
<a href="#6102" id="6102">6102</a>
<a href="#6103" id="6103">6103</a>
<a href="#6104" id="6104">6104</a>
<a href="#6105" id="6105">6105</a>
<a href="#6106" id="6106">6106</a>
<a href="#6107" id="6107">6107</a>
<a href="#6108" id="6108">6108</a>
<a href="#6109" id="6109">6109</a>
<a href="#6110" id="6110">6110</a>
<a href="#6111" id="6111">6111</a>
<a href="#6112" id="6112">6112</a>
<a href="#6113" id="6113">6113</a>
<a href="#6114" id="6114">6114</a>
<a href="#6115" id="6115">6115</a>
<a href="#6116" id="6116">6116</a>
<a href="#6117" id="6117">6117</a>
<a href="#6118" id="6118">6118</a>
<a href="#6119" id="6119">6119</a>
<a href="#6120" id="6120">6120</a>
<a href="#6121" id="6121">6121</a>
<a href="#6122" id="6122">6122</a>
<a href="#6123" id="6123">6123</a>
<a href="#6124" id="6124">6124</a>
<a href="#6125" id="6125">6125</a>
<a href="#6126" id="6126">6126</a>
<a href="#6127" id="6127">6127</a>
<a href="#6128" id="6128">6128</a>
<a href="#6129" id="6129">6129</a>
<a href="#6130" id="6130">6130</a>
<a href="#6131" id="6131">6131</a>
<a href="#6132" id="6132">6132</a>
<a href="#6133" id="6133">6133</a>
<a href="#6134" id="6134">6134</a>
<a href="#6135" id="6135">6135</a>
<a href="#6136" id="6136">6136</a>
<a href="#6137" id="6137">6137</a>
<a href="#6138" id="6138">6138</a>
<a href="#6139" id="6139">6139</a>
<a href="#6140" id="6140">6140</a>
<a href="#6141" id="6141">6141</a>
<a href="#6142" id="6142">6142</a>
<a href="#6143" id="6143">6143</a>
<a href="#6144" id="6144">6144</a>
<a href="#6145" id="6145">6145</a>
<a href="#6146" id="6146">6146</a>
<a href="#6147" id="6147">6147</a>
<a href="#6148" id="6148">6148</a>
<a href="#6149" id="6149">6149</a>
<a href="#6150" id="6150">6150</a>
<a href="#6151" id="6151">6151</a>
<a href="#6152" id="6152">6152</a>
<a href="#6153" id="6153">6153</a>
<a href="#6154" id="6154">6154</a>
<a href="#6155" id="6155">6155</a>
<a href="#6156" id="6156">6156</a>
<a href="#6157" id="6157">6157</a>
<a href="#6158" id="6158">6158</a>
<a href="#6159" id="6159">6159</a>
<a href="#6160" id="6160">6160</a>
<a href="#6161" id="6161">6161</a>
<a href="#6162" id="6162">6162</a>
<a href="#6163" id="6163">6163</a>
<a href="#6164" id="6164">6164</a>
<a href="#6165" id="6165">6165</a>
<a href="#6166" id="6166">6166</a>
<a href="#6167" id="6167">6167</a>
<a href="#6168" id="6168">6168</a>
<a href="#6169" id="6169">6169</a>
<a href="#6170" id="6170">6170</a>
<a href="#6171" id="6171">6171</a>
<a href="#6172" id="6172">6172</a>
<a href="#6173" id="6173">6173</a>
<a href="#6174" id="6174">6174</a>
<a href="#6175" id="6175">6175</a>
<a href="#6176" id="6176">6176</a>
<a href="#6177" id="6177">6177</a>
<a href="#6178" id="6178">6178</a>
<a href="#6179" id="6179">6179</a>
<a href="#6180" id="6180">6180</a>
<a href="#6181" id="6181">6181</a>
<a href="#6182" id="6182">6182</a>
<a href="#6183" id="6183">6183</a>
<a href="#6184" id="6184">6184</a>
<a href="#6185" id="6185">6185</a>
<a href="#6186" id="6186">6186</a>
<a href="#6187" id="6187">6187</a>
<a href="#6188" id="6188">6188</a>
<a href="#6189" id="6189">6189</a>
<a href="#6190" id="6190">6190</a>
<a href="#6191" id="6191">6191</a>
<a href="#6192" id="6192">6192</a>
<a href="#6193" id="6193">6193</a>
<a href="#6194" id="6194">6194</a>
<a href="#6195" id="6195">6195</a>
<a href="#6196" id="6196">6196</a>
<a href="#6197" id="6197">6197</a>
<a href="#6198" id="6198">6198</a>
<a href="#6199" id="6199">6199</a>
<a href="#6200" id="6200">6200</a>
<a href="#6201" id="6201">6201</a>
<a href="#6202" id="6202">6202</a>
<a href="#6203" id="6203">6203</a>
<a href="#6204" id="6204">6204</a>
<a href="#6205" id="6205">6205</a>
<a href="#6206" id="6206">6206</a>
<a href="#6207" id="6207">6207</a>
<a href="#6208" id="6208">6208</a>
<a href="#6209" id="6209">6209</a>
<a href="#6210" id="6210">6210</a>
<a href="#6211" id="6211">6211</a>
<a href="#6212" id="6212">6212</a>
<a href="#6213" id="6213">6213</a>
<a href="#6214" id="6214">6214</a>
<a href="#6215" id="6215">6215</a>
<a href="#6216" id="6216">6216</a>
<a href="#6217" id="6217">6217</a>
<a href="#6218" id="6218">6218</a>
<a href="#6219" id="6219">6219</a>
<a href="#6220" id="6220">6220</a>
<a href="#6221" id="6221">6221</a>
<a href="#6222" id="6222">6222</a>
<a href="#6223" id="6223">6223</a>
<a href="#6224" id="6224">6224</a>
<a href="#6225" id="6225">6225</a>
<a href="#6226" id="6226">6226</a>
<a href="#6227" id="6227">6227</a>
<a href="#6228" id="6228">6228</a>
<a href="#6229" id="6229">6229</a>
<a href="#6230" id="6230">6230</a>
<a href="#6231" id="6231">6231</a>
<a href="#6232" id="6232">6232</a>
<a href="#6233" id="6233">6233</a>
<a href="#6234" id="6234">6234</a>
<a href="#6235" id="6235">6235</a>
<a href="#6236" id="6236">6236</a>
<a href="#6237" id="6237">6237</a>
<a href="#6238" id="6238">6238</a>
<a href="#6239" id="6239">6239</a>
<a href="#6240" id="6240">6240</a>
<a href="#6241" id="6241">6241</a>
<a href="#6242" id="6242">6242</a>
<a href="#6243" id="6243">6243</a>
<a href="#6244" id="6244">6244</a>
<a href="#6245" id="6245">6245</a>
<a href="#6246" id="6246">6246</a>
<a href="#6247" id="6247">6247</a>
<a href="#6248" id="6248">6248</a>
<a href="#6249" id="6249">6249</a>
<a href="#6250" id="6250">6250</a>
<a href="#6251" id="6251">6251</a>
<a href="#6252" id="6252">6252</a>
<a href="#6253" id="6253">6253</a>
<a href="#6254" id="6254">6254</a>
<a href="#6255" id="6255">6255</a>
<a href="#6256" id="6256">6256</a>
<a href="#6257" id="6257">6257</a>
<a href="#6258" id="6258">6258</a>
<a href="#6259" id="6259">6259</a>
<a href="#6260" id="6260">6260</a>
<a href="#6261" id="6261">6261</a>
<a href="#6262" id="6262">6262</a>
<a href="#6263" id="6263">6263</a>
<a href="#6264" id="6264">6264</a>
<a href="#6265" id="6265">6265</a>
<a href="#6266" id="6266">6266</a>
<a href="#6267" id="6267">6267</a>
<a href="#6268" id="6268">6268</a>
<a href="#6269" id="6269">6269</a>
<a href="#6270" id="6270">6270</a>
<a href="#6271" id="6271">6271</a>
<a href="#6272" id="6272">6272</a>
<a href="#6273" id="6273">6273</a>
<a href="#6274" id="6274">6274</a>
<a href="#6275" id="6275">6275</a>
<a href="#6276" id="6276">6276</a>
<a href="#6277" id="6277">6277</a>
<a href="#6278" id="6278">6278</a>
<a href="#6279" id="6279">6279</a>
<a href="#6280" id="6280">6280</a>
<a href="#6281" id="6281">6281</a>
<a href="#6282" id="6282">6282</a>
<a href="#6283" id="6283">6283</a>
<a href="#6284" id="6284">6284</a>
<a href="#6285" id="6285">6285</a>
<a href="#6286" id="6286">6286</a>
<a href="#6287" id="6287">6287</a>
<a href="#6288" id="6288">6288</a>
<a href="#6289" id="6289">6289</a>
<a href="#6290" id="6290">6290</a>
<a href="#6291" id="6291">6291</a>
<a href="#6292" id="6292">6292</a>
<a href="#6293" id="6293">6293</a>
<a href="#6294" id="6294">6294</a>
<a href="#6295" id="6295">6295</a>
<a href="#6296" id="6296">6296</a>
<a href="#6297" id="6297">6297</a>
<a href="#6298" id="6298">6298</a>
<a href="#6299" id="6299">6299</a>
<a href="#6300" id="6300">6300</a>
<a href="#6301" id="6301">6301</a>
<a href="#6302" id="6302">6302</a>
<a href="#6303" id="6303">6303</a>
<a href="#6304" id="6304">6304</a>
<a href="#6305" id="6305">6305</a>
<a href="#6306" id="6306">6306</a>
<a href="#6307" id="6307">6307</a>
<a href="#6308" id="6308">6308</a>
<a href="#6309" id="6309">6309</a>
<a href="#6310" id="6310">6310</a>
<a href="#6311" id="6311">6311</a>
<a href="#6312" id="6312">6312</a>
<a href="#6313" id="6313">6313</a>
<a href="#6314" id="6314">6314</a>
<a href="#6315" id="6315">6315</a>
<a href="#6316" id="6316">6316</a>
<a href="#6317" id="6317">6317</a>
<a href="#6318" id="6318">6318</a>
<a href="#6319" id="6319">6319</a>
<a href="#6320" id="6320">6320</a>
<a href="#6321" id="6321">6321</a>
<a href="#6322" id="6322">6322</a>
<a href="#6323" id="6323">6323</a>
<a href="#6324" id="6324">6324</a>
<a href="#6325" id="6325">6325</a>
<a href="#6326" id="6326">6326</a>
<a href="#6327" id="6327">6327</a>
<a href="#6328" id="6328">6328</a>
<a href="#6329" id="6329">6329</a>
<a href="#6330" id="6330">6330</a>
<a href="#6331" id="6331">6331</a>
<a href="#6332" id="6332">6332</a>
<a href="#6333" id="6333">6333</a>
<a href="#6334" id="6334">6334</a>
<a href="#6335" id="6335">6335</a>
<a href="#6336" id="6336">6336</a>
<a href="#6337" id="6337">6337</a>
<a href="#6338" id="6338">6338</a>
<a href="#6339" id="6339">6339</a>
<a href="#6340" id="6340">6340</a>
<a href="#6341" id="6341">6341</a>
<a href="#6342" id="6342">6342</a>
<a href="#6343" id="6343">6343</a>
<a href="#6344" id="6344">6344</a>
<a href="#6345" id="6345">6345</a>
<a href="#6346" id="6346">6346</a>
<a href="#6347" id="6347">6347</a>
<a href="#6348" id="6348">6348</a>
<a href="#6349" id="6349">6349</a>
<a href="#6350" id="6350">6350</a>
<a href="#6351" id="6351">6351</a>
<a href="#6352" id="6352">6352</a>
<a href="#6353" id="6353">6353</a>
<a href="#6354" id="6354">6354</a>
<a href="#6355" id="6355">6355</a>
<a href="#6356" id="6356">6356</a>
<a href="#6357" id="6357">6357</a>
<a href="#6358" id="6358">6358</a>
<a href="#6359" id="6359">6359</a>
<a href="#6360" id="6360">6360</a>
<a href="#6361" id="6361">6361</a>
<a href="#6362" id="6362">6362</a>
<a href="#6363" id="6363">6363</a>
<a href="#6364" id="6364">6364</a>
<a href="#6365" id="6365">6365</a>
<a href="#6366" id="6366">6366</a>
<a href="#6367" id="6367">6367</a>
<a href="#6368" id="6368">6368</a>
<a href="#6369" id="6369">6369</a>
<a href="#6370" id="6370">6370</a>
<a href="#6371" id="6371">6371</a>
<a href="#6372" id="6372">6372</a>
<a href="#6373" id="6373">6373</a>
<a href="#6374" id="6374">6374</a>
<a href="#6375" id="6375">6375</a>
<a href="#6376" id="6376">6376</a>
<a href="#6377" id="6377">6377</a>
</pre></div><pre class="rust"><code><span class="doccomment">/*!
This module provides a regular expression parser.
*/

</span><span class="kw">use </span>core::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    mem,
};

<span class="kw">use </span>alloc::{
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};

<span class="kw">use crate</span>::{
    ast::{<span class="self">self</span>, Ast, Position, Span},
    either::Either,
    is_escapeable_character, is_meta_character,
};

<span class="kw">type </span><span class="prelude-ty">Result</span>&lt;T&gt; = core::result::Result&lt;T, ast::Error&gt;;

<span class="doccomment">/// A primitive is an expression with no sub-expressions. This includes
/// literals, assertions and non-set character classes. This representation
/// is used as intermediate state in the parser.
///
/// This does not include ASCII character classes, since they can only appear
/// within a set character class.
</span><span class="attr">#[derive(Clone, Debug, Eq, PartialEq)]
</span><span class="kw">enum </span>Primitive {
    Literal(ast::Literal),
    Assertion(ast::Assertion),
    Dot(Span),
    Perl(ast::ClassPerl),
    Unicode(ast::ClassUnicode),
}

<span class="kw">impl </span>Primitive {
    <span class="doccomment">/// Return the span of this primitive.
    </span><span class="kw">fn </span>span(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>Span {
        <span class="kw">match </span><span class="kw-2">*</span><span class="self">self </span>{
            Primitive::Literal(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
            Primitive::Assertion(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
            Primitive::Dot(<span class="kw-2">ref </span>span) =&gt; span,
            Primitive::Perl(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
            Primitive::Unicode(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
        }
    }

    <span class="doccomment">/// Convert this primitive into a proper AST.
    </span><span class="kw">fn </span>into_ast(<span class="self">self</span>) -&gt; Ast {
        <span class="kw">match </span><span class="self">self </span>{
            Primitive::Literal(lit) =&gt; Ast::literal(lit),
            Primitive::Assertion(assert) =&gt; Ast::assertion(assert),
            Primitive::Dot(span) =&gt; Ast::dot(span),
            Primitive::Perl(cls) =&gt; Ast::class_perl(cls),
            Primitive::Unicode(cls) =&gt; Ast::class_unicode(cls),
        }
    }

    <span class="doccomment">/// Convert this primitive into an item in a character class.
    ///
    /// If this primitive is not a legal item (i.e., an assertion or a dot),
    /// then return an error.
    </span><span class="kw">fn </span>into_class_set_item&lt;P: Borrow&lt;Parser&gt;&gt;(
        <span class="self">self</span>,
        p: <span class="kw-2">&amp;</span>ParserI&lt;<span class="lifetime">'_</span>, P&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;ast::ClassSetItem&gt; {
        <span class="kw">use </span><span class="self">self</span>::Primitive::<span class="kw-2">*</span>;
        <span class="kw">use </span><span class="kw">crate</span>::ast::ClassSetItem;

        <span class="kw">match </span><span class="self">self </span>{
            Literal(lit) =&gt; <span class="prelude-val">Ok</span>(ClassSetItem::Literal(lit)),
            Perl(cls) =&gt; <span class="prelude-val">Ok</span>(ClassSetItem::Perl(cls)),
            Unicode(cls) =&gt; <span class="prelude-val">Ok</span>(ClassSetItem::Unicode(cls)),
            x =&gt; <span class="prelude-val">Err</span>(p.error(<span class="kw-2">*</span>x.span(), ast::ErrorKind::ClassEscapeInvalid)),
        }
    }

    <span class="doccomment">/// Convert this primitive into a literal in a character class. In
    /// particular, literals are the only valid items that can appear in
    /// ranges.
    ///
    /// If this primitive is not a legal item (i.e., a class, assertion or a
    /// dot), then return an error.
    </span><span class="kw">fn </span>into_class_literal&lt;P: Borrow&lt;Parser&gt;&gt;(
        <span class="self">self</span>,
        p: <span class="kw-2">&amp;</span>ParserI&lt;<span class="lifetime">'_</span>, P&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Literal&gt; {
        <span class="kw">use </span><span class="self">self</span>::Primitive::<span class="kw-2">*</span>;

        <span class="kw">match </span><span class="self">self </span>{
            Literal(lit) =&gt; <span class="prelude-val">Ok</span>(lit),
            x =&gt; <span class="prelude-val">Err</span>(p.error(<span class="kw-2">*</span>x.span(), ast::ErrorKind::ClassRangeLiteral)),
        }
    }
}

<span class="doccomment">/// Returns true if the given character is a hexadecimal digit.
</span><span class="kw">fn </span>is_hex(c: char) -&gt; bool {
    (<span class="string">'0' </span>&lt;= c &amp;&amp; c &lt;= <span class="string">'9'</span>) || (<span class="string">'a' </span>&lt;= c &amp;&amp; c &lt;= <span class="string">'f'</span>) || (<span class="string">'A' </span>&lt;= c &amp;&amp; c &lt;= <span class="string">'F'</span>)
}

<span class="doccomment">/// Returns true if the given character is a valid in a capture group name.
///
/// If `first` is true, then `c` is treated as the first character in the
/// group name (which must be alphabetic or underscore).
</span><span class="kw">fn </span>is_capture_char(c: char, first: bool) -&gt; bool {
    <span class="kw">if </span>first {
        c == <span class="string">'_' </span>|| c.is_alphabetic()
    } <span class="kw">else </span>{
        c == <span class="string">'_' </span>|| c == <span class="string">'.' </span>|| c == <span class="string">'[' </span>|| c == <span class="string">']' </span>|| c.is_alphanumeric()
    }
}

<span class="doccomment">/// A builder for a regular expression parser.
///
/// This builder permits modifying configuration options for the parser.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>ParserBuilder {
    ignore_whitespace: bool,
    nest_limit: u32,
    octal: bool,
    empty_min_range: bool,
}

<span class="kw">impl </span>Default <span class="kw">for </span>ParserBuilder {
    <span class="kw">fn </span>default() -&gt; ParserBuilder {
        ParserBuilder::new()
    }
}

<span class="kw">impl </span>ParserBuilder {
    <span class="doccomment">/// Create a new parser builder with a default configuration.
    </span><span class="kw">pub fn </span>new() -&gt; ParserBuilder {
        ParserBuilder {
            ignore_whitespace: <span class="bool-val">false</span>,
            nest_limit: <span class="number">250</span>,
            octal: <span class="bool-val">false</span>,
            empty_min_range: <span class="bool-val">false</span>,
        }
    }

    <span class="doccomment">/// Build a parser from this configuration with the given pattern.
    </span><span class="kw">pub fn </span>build(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Parser {
        Parser {
            pos: Cell::new(Position { offset: <span class="number">0</span>, line: <span class="number">1</span>, column: <span class="number">1 </span>}),
            capture_index: Cell::new(<span class="number">0</span>),
            nest_limit: <span class="self">self</span>.nest_limit,
            octal: <span class="self">self</span>.octal,
            empty_min_range: <span class="self">self</span>.empty_min_range,
            initial_ignore_whitespace: <span class="self">self</span>.ignore_whitespace,
            ignore_whitespace: Cell::new(<span class="self">self</span>.ignore_whitespace),
            comments: RefCell::new(<span class="macro">vec!</span>[]),
            stack_group: RefCell::new(<span class="macro">vec!</span>[]),
            stack_class: RefCell::new(<span class="macro">vec!</span>[]),
            capture_names: RefCell::new(<span class="macro">vec!</span>[]),
            scratch: RefCell::new(String::new()),
        }
    }

    <span class="doccomment">/// Set the nesting limit for this parser.
    ///
    /// The nesting limit controls how deep the abstract syntax tree is allowed
    /// to be. If the AST exceeds the given limit (e.g., with too many nested
    /// groups), then an error is returned by the parser.
    ///
    /// The purpose of this limit is to act as a heuristic to prevent stack
    /// overflow for consumers that do structural induction on an `Ast` using
    /// explicit recursion. While this crate never does this (instead using
    /// constant stack space and moving the call stack to the heap), other
    /// crates may.
    ///
    /// This limit is not checked until the entire AST is parsed. Therefore,
    /// if callers want to put a limit on the amount of heap space used, then
    /// they should impose a limit on the length, in bytes, of the concrete
    /// pattern string. In particular, this is viable since this parser
    /// implementation will limit itself to heap space proportional to the
    /// length of the pattern string.
    ///
    /// Note that a nest limit of `0` will return a nest limit error for most
    /// patterns but not all. For example, a nest limit of `0` permits `a` but
    /// not `ab`, since `ab` requires a concatenation, which results in a nest
    /// depth of `1`. In general, a nest limit is not something that manifests
    /// in an obvious way in the concrete syntax, therefore, it should not be
    /// used in a granular way.
    </span><span class="kw">pub fn </span>nest_limit(<span class="kw-2">&amp;mut </span><span class="self">self</span>, limit: u32) -&gt; <span class="kw-2">&amp;mut </span>ParserBuilder {
        <span class="self">self</span>.nest_limit = limit;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Whether to support octal syntax or not.
    ///
    /// Octal syntax is a little-known way of uttering Unicode codepoints in
    /// a regular expression. For example, `a`, `\x61`, `\u0061` and
    /// `\141` are all equivalent regular expressions, where the last example
    /// shows octal syntax.
    ///
    /// While supporting octal syntax isn't in and of itself a problem, it does
    /// make good error messages harder. That is, in PCRE based regex engines,
    /// syntax like `\0` invokes a backreference, which is explicitly
    /// unsupported in Rust's regex engine. However, many users expect it to
    /// be supported. Therefore, when octal support is disabled, the error
    /// message will explicitly mention that backreferences aren't supported.
    ///
    /// Octal syntax is disabled by default.
    </span><span class="kw">pub fn </span>octal(<span class="kw-2">&amp;mut </span><span class="self">self</span>, yes: bool) -&gt; <span class="kw-2">&amp;mut </span>ParserBuilder {
        <span class="self">self</span>.octal = yes;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Enable verbose mode in the regular expression.
    ///
    /// When enabled, verbose mode permits insignificant whitespace in many
    /// places in the regular expression, as well as comments. Comments are
    /// started using `#` and continue until the end of the line.
    ///
    /// By default, this is disabled. It may be selectively enabled in the
    /// regular expression by using the `x` flag regardless of this setting.
    </span><span class="kw">pub fn </span>ignore_whitespace(<span class="kw-2">&amp;mut </span><span class="self">self</span>, yes: bool) -&gt; <span class="kw-2">&amp;mut </span>ParserBuilder {
        <span class="self">self</span>.ignore_whitespace = yes;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Allow using `{,n}` as an equivalent to `{0,n}`.
    ///
    /// When enabled, the parser accepts `{,n}` as valid syntax for `{0,n}`.
    /// Most regular expression engines don't support the `{,n}` syntax, but
    /// some others do it, namely Python's `re` library.
    ///
    /// This is disabled by default.
    </span><span class="kw">pub fn </span>empty_min_range(<span class="kw-2">&amp;mut </span><span class="self">self</span>, yes: bool) -&gt; <span class="kw-2">&amp;mut </span>ParserBuilder {
        <span class="self">self</span>.empty_min_range = yes;
        <span class="self">self
    </span>}
}

<span class="doccomment">/// A regular expression parser.
///
/// This parses a string representation of a regular expression into an
/// abstract syntax tree. The size of the tree is proportional to the length
/// of the regular expression pattern.
///
/// A `Parser` can be configured in more detail via a [`ParserBuilder`].
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>Parser {
    <span class="doccomment">/// The current position of the parser.
    </span>pos: Cell&lt;Position&gt;,
    <span class="doccomment">/// The current capture index.
    </span>capture_index: Cell&lt;u32&gt;,
    <span class="doccomment">/// The maximum number of open parens/brackets allowed. If the parser
    /// exceeds this number, then an error is returned.
    </span>nest_limit: u32,
    <span class="doccomment">/// Whether to support octal syntax or not. When `false`, the parser will
    /// return an error helpfully pointing out that backreferences are not
    /// supported.
    </span>octal: bool,
    <span class="doccomment">/// The initial setting for `ignore_whitespace` as provided by
    /// `ParserBuilder`. It is used when resetting the parser's state.
    </span>initial_ignore_whitespace: bool,
    <span class="doccomment">/// Whether the parser supports `{,n}` repetitions as an equivalent to
    /// `{0,n}.`
    </span>empty_min_range: bool,
    <span class="doccomment">/// Whether whitespace should be ignored. When enabled, comments are
    /// also permitted.
    </span>ignore_whitespace: Cell&lt;bool&gt;,
    <span class="doccomment">/// A list of comments, in order of appearance.
    </span>comments: RefCell&lt;Vec&lt;ast::Comment&gt;&gt;,
    <span class="doccomment">/// A stack of grouped sub-expressions, including alternations.
    </span>stack_group: RefCell&lt;Vec&lt;GroupState&gt;&gt;,
    <span class="doccomment">/// A stack of nested character classes. This is only non-empty when
    /// parsing a class.
    </span>stack_class: RefCell&lt;Vec&lt;ClassState&gt;&gt;,
    <span class="doccomment">/// A sorted sequence of capture names. This is used to detect duplicate
    /// capture names and report an error if one is detected.
    </span>capture_names: RefCell&lt;Vec&lt;ast::CaptureName&gt;&gt;,
    <span class="doccomment">/// A scratch buffer used in various places. Mostly this is used to
    /// accumulate relevant characters from parts of a pattern.
    </span>scratch: RefCell&lt;String&gt;,
}

<span class="doccomment">/// ParserI is the internal parser implementation.
///
/// We use this separate type so that we can carry the provided pattern string
/// along with us. In particular, a `Parser` internal state is not tied to any
/// one pattern, but `ParserI` is.
///
/// This type also lets us use `ParserI&lt;&amp;Parser&gt;` in production code while
/// retaining the convenience of `ParserI&lt;Parser&gt;` for tests, which sometimes
/// work against the internal interface of the parser.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">struct </span>ParserI&lt;<span class="lifetime">'s</span>, P&gt; {
    <span class="doccomment">/// The parser state/configuration.
    </span>parser: P,
    <span class="doccomment">/// The full regular expression provided by the user.
    </span>pattern: <span class="kw-2">&amp;</span><span class="lifetime">'s </span>str,
}

<span class="doccomment">/// GroupState represents a single stack frame while parsing nested groups
/// and alternations. Each frame records the state up to an opening parenthesis
/// or a alternating bracket `|`.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">enum </span>GroupState {
    <span class="doccomment">/// This state is pushed whenever an opening group is found.
    </span>Group {
        <span class="doccomment">/// The concatenation immediately preceding the opening group.
        </span>concat: ast::Concat,
        <span class="doccomment">/// The group that has been opened. Its sub-AST is always empty.
        </span>group: ast::Group,
        <span class="doccomment">/// Whether this group has the `x` flag enabled or not.
        </span>ignore_whitespace: bool,
    },
    <span class="doccomment">/// This state is pushed whenever a new alternation branch is found. If
    /// an alternation branch is found and this state is at the top of the
    /// stack, then this state should be modified to include the new
    /// alternation.
    </span>Alternation(ast::Alternation),
}

<span class="doccomment">/// ClassState represents a single stack frame while parsing character classes.
/// Each frame records the state up to an intersection, difference, symmetric
/// difference or nested class.
///
/// Note that a parser's character class stack is only non-empty when parsing
/// a character class. In all other cases, it is empty.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">enum </span>ClassState {
    <span class="doccomment">/// This state is pushed whenever an opening bracket is found.
    </span>Open {
        <span class="doccomment">/// The union of class items immediately preceding this class.
        </span><span class="kw">union</span>: ast::ClassSetUnion,
        <span class="doccomment">/// The class that has been opened. Typically this just corresponds
        /// to the `[`, but it can also include `[^` since `^` indicates
        /// negation of the class.
        </span>set: ast::ClassBracketed,
    },
    <span class="doccomment">/// This state is pushed when a operator is seen. When popped, the stored
    /// set becomes the left hand side of the operator.
    </span>Op {
        <span class="doccomment">/// The type of the operation, i.e., &amp;&amp;, -- or ~~.
        </span>kind: ast::ClassSetBinaryOpKind,
        <span class="doccomment">/// The left-hand side of the operator.
        </span>lhs: ast::ClassSet,
    },
}

<span class="kw">impl </span>Parser {
    <span class="doccomment">/// Create a new parser with a default configuration.
    ///
    /// The parser can be run with either the `parse` or `parse_with_comments`
    /// methods. The parse methods return an abstract syntax tree.
    ///
    /// To set configuration options on the parser, use [`ParserBuilder`].
    </span><span class="kw">pub fn </span>new() -&gt; Parser {
        ParserBuilder::new().build()
    }

    <span class="doccomment">/// Parse the regular expression into an abstract syntax tree.
    </span><span class="kw">pub fn </span>parse(<span class="kw-2">&amp;mut </span><span class="self">self</span>, pattern: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Result</span>&lt;Ast&gt; {
        ParserI::new(<span class="self">self</span>, pattern).parse()
    }

    <span class="doccomment">/// Parse the regular expression and return an abstract syntax tree with
    /// all of the comments found in the pattern.
    </span><span class="kw">pub fn </span>parse_with_comments(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        pattern: <span class="kw-2">&amp;</span>str,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;ast::WithComments&gt; {
        ParserI::new(<span class="self">self</span>, pattern).parse_with_comments()
    }

    <span class="doccomment">/// Reset the internal state of a parser.
    ///
    /// This is called at the beginning of every parse. This prevents the
    /// parser from running with inconsistent state (say, if a previous
    /// invocation returned an error and the parser is reused).
    </span><span class="kw">fn </span>reset(<span class="kw-2">&amp;</span><span class="self">self</span>) {
        <span class="comment">// These settings should be in line with the construction
        // in `ParserBuilder::build`.
        </span><span class="self">self</span>.pos.set(Position { offset: <span class="number">0</span>, line: <span class="number">1</span>, column: <span class="number">1 </span>});
        <span class="self">self</span>.ignore_whitespace.set(<span class="self">self</span>.initial_ignore_whitespace);
        <span class="self">self</span>.comments.borrow_mut().clear();
        <span class="self">self</span>.stack_group.borrow_mut().clear();
        <span class="self">self</span>.stack_class.borrow_mut().clear();
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'s</span>, P: Borrow&lt;Parser&gt;&gt; ParserI&lt;<span class="lifetime">'s</span>, P&gt; {
    <span class="doccomment">/// Build an internal parser from a parser configuration and a pattern.
    </span><span class="kw">fn </span>new(parser: P, pattern: <span class="kw-2">&amp;</span><span class="lifetime">'s </span>str) -&gt; ParserI&lt;<span class="lifetime">'s</span>, P&gt; {
        ParserI { parser, pattern }
    }

    <span class="doccomment">/// Return a reference to the parser state.
    </span><span class="kw">fn </span>parser(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>Parser {
        <span class="self">self</span>.parser.borrow()
    }

    <span class="doccomment">/// Return a reference to the pattern being parsed.
    </span><span class="kw">fn </span>pattern(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span>str {
        <span class="self">self</span>.pattern
    }

    <span class="doccomment">/// Create a new error with the given span and error type.
    </span><span class="kw">fn </span>error(<span class="kw-2">&amp;</span><span class="self">self</span>, span: Span, kind: ast::ErrorKind) -&gt; ast::Error {
        ast::Error { kind, pattern: <span class="self">self</span>.pattern().to_string(), span }
    }

    <span class="doccomment">/// Return the current offset of the parser.
    ///
    /// The offset starts at `0` from the beginning of the regular expression
    /// pattern string.
    </span><span class="kw">fn </span>offset(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.parser().pos.get().offset
    }

    <span class="doccomment">/// Return the current line number of the parser.
    ///
    /// The line number starts at `1`.
    </span><span class="kw">fn </span>line(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.parser().pos.get().line
    }

    <span class="doccomment">/// Return the current column of the parser.
    ///
    /// The column number starts at `1` and is reset whenever a `\n` is seen.
    </span><span class="kw">fn </span>column(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.parser().pos.get().column
    }

    <span class="doccomment">/// Return the next capturing index. Each subsequent call increments the
    /// internal index.
    ///
    /// The span given should correspond to the location of the opening
    /// parenthesis.
    ///
    /// If the capture limit is exceeded, then an error is returned.
    </span><span class="kw">fn </span>next_capture_index(<span class="kw-2">&amp;</span><span class="self">self</span>, span: Span) -&gt; <span class="prelude-ty">Result</span>&lt;u32&gt; {
        <span class="kw">let </span>current = <span class="self">self</span>.parser().capture_index.get();
        <span class="kw">let </span>i = current.checked_add(<span class="number">1</span>).ok_or_else(|| {
            <span class="self">self</span>.error(span, ast::ErrorKind::CaptureLimitExceeded)
        })<span class="question-mark">?</span>;
        <span class="self">self</span>.parser().capture_index.set(i);
        <span class="prelude-val">Ok</span>(i)
    }

    <span class="doccomment">/// Adds the given capture name to this parser. If this capture name has
    /// already been used, then an error is returned.
    </span><span class="kw">fn </span>add_capture_name(<span class="kw-2">&amp;</span><span class="self">self</span>, cap: <span class="kw-2">&amp;</span>ast::CaptureName) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>names = <span class="self">self</span>.parser().capture_names.borrow_mut();
        <span class="kw">match </span>names
            .binary_search_by_key(<span class="kw-2">&amp;</span>cap.name.as_str(), |c| c.name.as_str())
        {
            <span class="prelude-val">Err</span>(i) =&gt; {
                names.insert(i, cap.clone());
                <span class="prelude-val">Ok</span>(())
            }
            <span class="prelude-val">Ok</span>(i) =&gt; <span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                cap.span,
                ast::ErrorKind::GroupNameDuplicate { original: names[i].span },
            )),
        }
    }

    <span class="doccomment">/// Return whether the parser should ignore whitespace or not.
    </span><span class="kw">fn </span>ignore_whitespace(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.parser().ignore_whitespace.get()
    }

    <span class="doccomment">/// Return the character at the current position of the parser.
    ///
    /// This panics if the current position does not point to a valid char.
    </span><span class="kw">fn </span>char(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; char {
        <span class="self">self</span>.char_at(<span class="self">self</span>.offset())
    }

    <span class="doccomment">/// Return the character at the given position.
    ///
    /// This panics if the given position does not point to a valid char.
    </span><span class="kw">fn </span>char_at(<span class="kw-2">&amp;</span><span class="self">self</span>, i: usize) -&gt; char {
        <span class="self">self</span>.pattern()[i..]
            .chars()
            .next()
            .unwrap_or_else(|| <span class="macro">panic!</span>(<span class="string">"expected char at offset {}"</span>, i))
    }

    <span class="doccomment">/// Bump the parser to the next Unicode scalar value.
    ///
    /// If the end of the input has been reached, then `false` is returned.
    </span><span class="kw">fn </span>bump(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="kw">if </span><span class="self">self</span>.is_eof() {
            <span class="kw">return </span><span class="bool-val">false</span>;
        }
        <span class="kw">let </span>Position { <span class="kw-2">mut </span>offset, <span class="kw-2">mut </span>line, <span class="kw-2">mut </span>column } = <span class="self">self</span>.pos();
        <span class="kw">if </span><span class="self">self</span>.char() == <span class="string">'\n' </span>{
            line = line.checked_add(<span class="number">1</span>).unwrap();
            column = <span class="number">1</span>;
        } <span class="kw">else </span>{
            column = column.checked_add(<span class="number">1</span>).unwrap();
        }
        offset += <span class="self">self</span>.char().len_utf8();
        <span class="self">self</span>.parser().pos.set(Position { offset, line, column });
        <span class="self">self</span>.pattern()[<span class="self">self</span>.offset()..].chars().next().is_some()
    }

    <span class="doccomment">/// If the substring starting at the current position of the parser has
    /// the given prefix, then bump the parser to the character immediately
    /// following the prefix and return true. Otherwise, don't bump the parser
    /// and return false.
    </span><span class="kw">fn </span>bump_if(<span class="kw-2">&amp;</span><span class="self">self</span>, prefix: <span class="kw-2">&amp;</span>str) -&gt; bool {
        <span class="kw">if </span><span class="self">self</span>.pattern()[<span class="self">self</span>.offset()..].starts_with(prefix) {
            <span class="kw">for _ in </span><span class="number">0</span>..prefix.chars().count() {
                <span class="self">self</span>.bump();
            }
            <span class="bool-val">true
        </span>} <span class="kw">else </span>{
            <span class="bool-val">false
        </span>}
    }

    <span class="doccomment">/// Returns true if and only if the parser is positioned at a look-around
    /// prefix. The conditions under which this returns true must always
    /// correspond to a regular expression that would otherwise be consider
    /// invalid.
    ///
    /// This should only be called immediately after parsing the opening of
    /// a group or a set of flags.
    </span><span class="kw">fn </span>is_lookaround_prefix(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.bump_if(<span class="string">"?="</span>)
            || <span class="self">self</span>.bump_if(<span class="string">"?!"</span>)
            || <span class="self">self</span>.bump_if(<span class="string">"?&lt;="</span>)
            || <span class="self">self</span>.bump_if(<span class="string">"?&lt;!"</span>)
    }

    <span class="doccomment">/// Bump the parser, and if the `x` flag is enabled, bump through any
    /// subsequent spaces. Return true if and only if the parser is not at
    /// EOF.
    </span><span class="kw">fn </span>bump_and_bump_space(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="kw">if </span>!<span class="self">self</span>.bump() {
            <span class="kw">return </span><span class="bool-val">false</span>;
        }
        <span class="self">self</span>.bump_space();
        !<span class="self">self</span>.is_eof()
    }

    <span class="doccomment">/// If the `x` flag is enabled (i.e., whitespace insensitivity with
    /// comments), then this will advance the parser through all whitespace
    /// and comments to the next non-whitespace non-comment byte.
    ///
    /// If the `x` flag is disabled, then this is a no-op.
    ///
    /// This should be used selectively throughout the parser where
    /// arbitrary whitespace is permitted when the `x` flag is enabled. For
    /// example, `{   5  , 6}` is equivalent to `{5,6}`.
    </span><span class="kw">fn </span>bump_space(<span class="kw-2">&amp;</span><span class="self">self</span>) {
        <span class="kw">if </span>!<span class="self">self</span>.ignore_whitespace() {
            <span class="kw">return</span>;
        }
        <span class="kw">while </span>!<span class="self">self</span>.is_eof() {
            <span class="kw">if </span><span class="self">self</span>.char().is_whitespace() {
                <span class="self">self</span>.bump();
            } <span class="kw">else if </span><span class="self">self</span>.char() == <span class="string">'#' </span>{
                <span class="kw">let </span>start = <span class="self">self</span>.pos();
                <span class="kw">let </span><span class="kw-2">mut </span>comment_text = String::new();
                <span class="self">self</span>.bump();
                <span class="kw">while </span>!<span class="self">self</span>.is_eof() {
                    <span class="kw">let </span>c = <span class="self">self</span>.char();
                    <span class="self">self</span>.bump();
                    <span class="kw">if </span>c == <span class="string">'\n' </span>{
                        <span class="kw">break</span>;
                    }
                    comment_text.push(c);
                }
                <span class="kw">let </span>comment = ast::Comment {
                    span: Span::new(start, <span class="self">self</span>.pos()),
                    comment: comment_text,
                };
                <span class="self">self</span>.parser().comments.borrow_mut().push(comment);
            } <span class="kw">else </span>{
                <span class="kw">break</span>;
            }
        }
    }

    <span class="doccomment">/// Peek at the next character in the input without advancing the parser.
    ///
    /// If the input has been exhausted, then this returns `None`.
    </span><span class="kw">fn </span>peek(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;char&gt; {
        <span class="kw">if </span><span class="self">self</span>.is_eof() {
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }
        <span class="self">self</span>.pattern()[<span class="self">self</span>.offset() + <span class="self">self</span>.char().len_utf8()..].chars().next()
    }

    <span class="doccomment">/// Like peek, but will ignore spaces when the parser is in whitespace
    /// insensitive mode.
    </span><span class="kw">fn </span>peek_space(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;char&gt; {
        <span class="kw">if </span>!<span class="self">self</span>.ignore_whitespace() {
            <span class="kw">return </span><span class="self">self</span>.peek();
        }
        <span class="kw">if </span><span class="self">self</span>.is_eof() {
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }
        <span class="kw">let </span><span class="kw-2">mut </span>start = <span class="self">self</span>.offset() + <span class="self">self</span>.char().len_utf8();
        <span class="kw">let </span><span class="kw-2">mut </span>in_comment = <span class="bool-val">false</span>;
        <span class="kw">for </span>(i, c) <span class="kw">in </span><span class="self">self</span>.pattern()[start..].char_indices() {
            <span class="kw">if </span>c.is_whitespace() {
                <span class="kw">continue</span>;
            } <span class="kw">else if </span>!in_comment &amp;&amp; c == <span class="string">'#' </span>{
                in_comment = <span class="bool-val">true</span>;
            } <span class="kw">else if </span>in_comment &amp;&amp; c == <span class="string">'\n' </span>{
                in_comment = <span class="bool-val">false</span>;
            } <span class="kw">else </span>{
                start += i;
                <span class="kw">break</span>;
            }
        }
        <span class="self">self</span>.pattern()[start..].chars().next()
    }

    <span class="doccomment">/// Returns true if the next call to `bump` would return false.
    </span><span class="kw">fn </span>is_eof(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.offset() == <span class="self">self</span>.pattern().len()
    }

    <span class="doccomment">/// Return the current position of the parser, which includes the offset,
    /// line and column.
    </span><span class="kw">fn </span>pos(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Position {
        <span class="self">self</span>.parser().pos.get()
    }

    <span class="doccomment">/// Create a span at the current position of the parser. Both the start
    /// and end of the span are set.
    </span><span class="kw">fn </span>span(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Span {
        Span::splat(<span class="self">self</span>.pos())
    }

    <span class="doccomment">/// Create a span that covers the current character.
    </span><span class="kw">fn </span>span_char(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Span {
        <span class="kw">let </span><span class="kw-2">mut </span>next = Position {
            offset: <span class="self">self</span>.offset().checked_add(<span class="self">self</span>.char().len_utf8()).unwrap(),
            line: <span class="self">self</span>.line(),
            column: <span class="self">self</span>.column().checked_add(<span class="number">1</span>).unwrap(),
        };
        <span class="kw">if </span><span class="self">self</span>.char() == <span class="string">'\n' </span>{
            next.line += <span class="number">1</span>;
            next.column = <span class="number">1</span>;
        }
        Span::new(<span class="self">self</span>.pos(), next)
    }

    <span class="doccomment">/// Parse and push a single alternation on to the parser's internal stack.
    /// If the top of the stack already has an alternation, then add to that
    /// instead of pushing a new one.
    ///
    /// The concatenation given corresponds to a single alternation branch.
    /// The concatenation returned starts the next branch and is empty.
    ///
    /// This assumes the parser is currently positioned at `|` and will advance
    /// the parser to the character following `|`.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>push_alternate(<span class="kw-2">&amp;</span><span class="self">self</span>, <span class="kw-2">mut </span>concat: ast::Concat) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Concat&gt; {
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'|'</span>);
        concat.span.end = <span class="self">self</span>.pos();
        <span class="self">self</span>.push_or_add_alternation(concat);
        <span class="self">self</span>.bump();
        <span class="prelude-val">Ok</span>(ast::Concat { span: <span class="self">self</span>.span(), asts: <span class="macro">vec!</span>[] })
    }

    <span class="doccomment">/// Pushes or adds the given branch of an alternation to the parser's
    /// internal stack of state.
    </span><span class="kw">fn </span>push_or_add_alternation(<span class="kw-2">&amp;</span><span class="self">self</span>, concat: ast::Concat) {
        <span class="kw">use </span><span class="self">self</span>::GroupState::<span class="kw-2">*</span>;

        <span class="kw">let </span><span class="kw-2">mut </span>stack = <span class="self">self</span>.parser().stack_group.borrow_mut();
        <span class="kw">if let </span><span class="prelude-val">Some</span>(<span class="kw-2">&amp;mut </span>Alternation(<span class="kw-2">ref mut </span>alts)) = stack.last_mut() {
            alts.asts.push(concat.into_ast());
            <span class="kw">return</span>;
        }
        stack.push(Alternation(ast::Alternation {
            span: Span::new(concat.span.start, <span class="self">self</span>.pos()),
            asts: <span class="macro">vec!</span>[concat.into_ast()],
        }));
    }

    <span class="doccomment">/// Parse and push a group AST (and its parent concatenation) on to the
    /// parser's internal stack. Return a fresh concatenation corresponding
    /// to the group's sub-AST.
    ///
    /// If a set of flags was found (with no group), then the concatenation
    /// is returned with that set of flags added.
    ///
    /// This assumes that the parser is currently positioned on the opening
    /// parenthesis. It advances the parser to the character at the start
    /// of the sub-expression (or adjoining expression).
    ///
    /// If there was a problem parsing the start of the group, then an error
    /// is returned.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>push_group(<span class="kw-2">&amp;</span><span class="self">self</span>, <span class="kw-2">mut </span>concat: ast::Concat) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Concat&gt; {
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'('</span>);
        <span class="kw">match </span><span class="self">self</span>.parse_group()<span class="question-mark">? </span>{
            Either::Left(set) =&gt; {
                <span class="kw">let </span>ignore = set.flags.flag_state(ast::Flag::IgnoreWhitespace);
                <span class="kw">if let </span><span class="prelude-val">Some</span>(v) = ignore {
                    <span class="self">self</span>.parser().ignore_whitespace.set(v);
                }

                concat.asts.push(Ast::flags(set));
                <span class="prelude-val">Ok</span>(concat)
            }
            Either::Right(group) =&gt; {
                <span class="kw">let </span>old_ignore_whitespace = <span class="self">self</span>.ignore_whitespace();
                <span class="kw">let </span>new_ignore_whitespace = group
                    .flags()
                    .and_then(|f| f.flag_state(ast::Flag::IgnoreWhitespace))
                    .unwrap_or(old_ignore_whitespace);
                <span class="self">self</span>.parser().stack_group.borrow_mut().push(
                    GroupState::Group {
                        concat,
                        group,
                        ignore_whitespace: old_ignore_whitespace,
                    },
                );
                <span class="self">self</span>.parser().ignore_whitespace.set(new_ignore_whitespace);
                <span class="prelude-val">Ok</span>(ast::Concat { span: <span class="self">self</span>.span(), asts: <span class="macro">vec!</span>[] })
            }
        }
    }

    <span class="doccomment">/// Pop a group AST from the parser's internal stack and set the group's
    /// AST to the given concatenation. Return the concatenation containing
    /// the group.
    ///
    /// This assumes that the parser is currently positioned on the closing
    /// parenthesis and advances the parser to the character following the `)`.
    ///
    /// If no such group could be popped, then an unopened group error is
    /// returned.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>pop_group(<span class="kw-2">&amp;</span><span class="self">self</span>, <span class="kw-2">mut </span>group_concat: ast::Concat) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Concat&gt; {
        <span class="kw">use </span><span class="self">self</span>::GroupState::<span class="kw-2">*</span>;

        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">')'</span>);
        <span class="kw">let </span><span class="kw-2">mut </span>stack = <span class="self">self</span>.parser().stack_group.borrow_mut();
        <span class="kw">let </span>(<span class="kw-2">mut </span>prior_concat, <span class="kw-2">mut </span>group, ignore_whitespace, alt) = <span class="kw">match </span>stack
            .pop()
        {
            <span class="prelude-val">Some</span>(Group { concat, group, ignore_whitespace }) =&gt; {
                (concat, group, ignore_whitespace, <span class="prelude-val">None</span>)
            }
            <span class="prelude-val">Some</span>(Alternation(alt)) =&gt; <span class="kw">match </span>stack.pop() {
                <span class="prelude-val">Some</span>(Group { concat, group, ignore_whitespace }) =&gt; {
                    (concat, group, ignore_whitespace, <span class="prelude-val">Some</span>(alt))
                }
                <span class="prelude-val">None </span>| <span class="prelude-val">Some</span>(Alternation(<span class="kw">_</span>)) =&gt; {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                        <span class="self">self</span>.span_char(),
                        ast::ErrorKind::GroupUnopened,
                    ));
                }
            },
            <span class="prelude-val">None </span>=&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self
                    </span>.error(<span class="self">self</span>.span_char(), ast::ErrorKind::GroupUnopened));
            }
        };
        <span class="self">self</span>.parser().ignore_whitespace.set(ignore_whitespace);
        group_concat.span.end = <span class="self">self</span>.pos();
        <span class="self">self</span>.bump();
        group.span.end = <span class="self">self</span>.pos();
        <span class="kw">match </span>alt {
            <span class="prelude-val">Some</span>(<span class="kw-2">mut </span>alt) =&gt; {
                alt.span.end = group_concat.span.end;
                alt.asts.push(group_concat.into_ast());
                group.ast = Box::new(alt.into_ast());
            }
            <span class="prelude-val">None </span>=&gt; {
                group.ast = Box::new(group_concat.into_ast());
            }
        }
        prior_concat.asts.push(Ast::group(group));
        <span class="prelude-val">Ok</span>(prior_concat)
    }

    <span class="doccomment">/// Pop the last state from the parser's internal stack, if it exists, and
    /// add the given concatenation to it. There either must be no state or a
    /// single alternation item on the stack. Any other scenario produces an
    /// error.
    ///
    /// This assumes that the parser has advanced to the end.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>pop_group_end(<span class="kw-2">&amp;</span><span class="self">self</span>, <span class="kw-2">mut </span>concat: ast::Concat) -&gt; <span class="prelude-ty">Result</span>&lt;Ast&gt; {
        concat.span.end = <span class="self">self</span>.pos();
        <span class="kw">let </span><span class="kw-2">mut </span>stack = <span class="self">self</span>.parser().stack_group.borrow_mut();
        <span class="kw">let </span>ast = <span class="kw">match </span>stack.pop() {
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Ok</span>(concat.into_ast()),
            <span class="prelude-val">Some</span>(GroupState::Alternation(<span class="kw-2">mut </span>alt)) =&gt; {
                alt.span.end = <span class="self">self</span>.pos();
                alt.asts.push(concat.into_ast());
                <span class="prelude-val">Ok</span>(Ast::alternation(alt))
            }
            <span class="prelude-val">Some</span>(GroupState::Group { group, .. }) =&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(
                    <span class="self">self</span>.error(group.span, ast::ErrorKind::GroupUnclosed)
                );
            }
        };
        <span class="comment">// If we try to pop again, there should be nothing.
        </span><span class="kw">match </span>stack.pop() {
            <span class="prelude-val">None </span>=&gt; ast,
            <span class="prelude-val">Some</span>(GroupState::Alternation(<span class="kw">_</span>)) =&gt; {
                <span class="comment">// This unreachable is unfortunate. This case can't happen
                // because the only way we can be here is if there were two
                // `GroupState::Alternation`s adjacent in the parser's stack,
                // which we guarantee to never happen because we never push a
                // `GroupState::Alternation` if one is already at the top of
                // the stack.
                </span><span class="macro">unreachable!</span>()
            }
            <span class="prelude-val">Some</span>(GroupState::Group { group, .. }) =&gt; {
                <span class="prelude-val">Err</span>(<span class="self">self</span>.error(group.span, ast::ErrorKind::GroupUnclosed))
            }
        }
    }

    <span class="doccomment">/// Parse the opening of a character class and push the current class
    /// parsing context onto the parser's stack. This assumes that the parser
    /// is positioned at an opening `[`. The given union should correspond to
    /// the union of set items built up before seeing the `[`.
    ///
    /// If there was a problem parsing the opening of the class, then an error
    /// is returned. Otherwise, a new union of set items for the class is
    /// returned (which may be populated with either a `]` or a `-`).
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>push_class_open(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        parent_union: ast::ClassSetUnion,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;ast::ClassSetUnion&gt; {
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'['</span>);

        <span class="kw">let </span>(nested_set, nested_union) = <span class="self">self</span>.parse_set_class_open()<span class="question-mark">?</span>;
        <span class="self">self</span>.parser()
            .stack_class
            .borrow_mut()
            .push(ClassState::Open { <span class="kw">union</span>: parent_union, set: nested_set });
        <span class="prelude-val">Ok</span>(nested_union)
    }

    <span class="doccomment">/// Parse the end of a character class set and pop the character class
    /// parser stack. The union given corresponds to the last union built
    /// before seeing the closing `]`. The union returned corresponds to the
    /// parent character class set with the nested class added to it.
    ///
    /// This assumes that the parser is positioned at a `]` and will advance
    /// the parser to the byte immediately following the `]`.
    ///
    /// If the stack is empty after popping, then this returns the final
    /// "top-level" character class AST (where a "top-level" character class
    /// is one that is not nested inside any other character class).
    ///
    /// If there is no corresponding opening bracket on the parser's stack,
    /// then an error is returned.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>pop_class(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        nested_union: ast::ClassSetUnion,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;Either&lt;ast::ClassSetUnion, ast::ClassBracketed&gt;&gt; {
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">']'</span>);

        <span class="kw">let </span>item = ast::ClassSet::Item(nested_union.into_item());
        <span class="kw">let </span>prevset = <span class="self">self</span>.pop_class_op(item);
        <span class="kw">let </span><span class="kw-2">mut </span>stack = <span class="self">self</span>.parser().stack_class.borrow_mut();
        <span class="kw">match </span>stack.pop() {
            <span class="prelude-val">None </span>=&gt; {
                <span class="comment">// We can never observe an empty stack:
                //
                // 1) We are guaranteed to start with a non-empty stack since
                //    the character class parser is only initiated when it sees
                //    a `[`.
                // 2) If we ever observe an empty stack while popping after
                //    seeing a `]`, then we signal the character class parser
                //    to terminate.
                </span><span class="macro">panic!</span>(<span class="string">"unexpected empty character class stack"</span>)
            }
            <span class="prelude-val">Some</span>(ClassState::Op { .. }) =&gt; {
                <span class="comment">// This panic is unfortunate, but this case is impossible
                // since we already popped the Op state if one exists above.
                // Namely, every push to the class parser stack is guarded by
                // whether an existing Op is already on the top of the stack.
                // If it is, the existing Op is modified. That is, the stack
                // can never have consecutive Op states.
                </span><span class="macro">panic!</span>(<span class="string">"unexpected ClassState::Op"</span>)
            }
            <span class="prelude-val">Some</span>(ClassState::Open { <span class="kw-2">mut </span>union, <span class="kw-2">mut </span>set }) =&gt; {
                <span class="self">self</span>.bump();
                set.span.end = <span class="self">self</span>.pos();
                set.kind = prevset;
                <span class="kw">if </span>stack.is_empty() {
                    <span class="prelude-val">Ok</span>(Either::Right(set))
                } <span class="kw">else </span>{
                    union.push(ast::ClassSetItem::Bracketed(Box::new(set)));
                    <span class="prelude-val">Ok</span>(Either::Left(union))
                }
            }
        }
    }

    <span class="doccomment">/// Return an "unclosed class" error whose span points to the most
    /// recently opened class.
    ///
    /// This should only be called while parsing a character class.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>unclosed_class_error(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; ast::Error {
        <span class="kw">for </span>state <span class="kw">in </span><span class="self">self</span>.parser().stack_class.borrow().iter().rev() {
            <span class="kw">if let </span>ClassState::Open { <span class="kw-2">ref </span>set, .. } = <span class="kw-2">*</span>state {
                <span class="kw">return </span><span class="self">self</span>.error(set.span, ast::ErrorKind::ClassUnclosed);
            }
        }
        <span class="comment">// We are guaranteed to have a non-empty stack with at least
        // one open bracket, so we should never get here.
        </span><span class="macro">panic!</span>(<span class="string">"no open character class found"</span>)
    }

    <span class="doccomment">/// Push the current set of class items on to the class parser's stack as
    /// the left hand side of the given operator.
    ///
    /// A fresh set union is returned, which should be used to build the right
    /// hand side of this operator.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>push_class_op(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        next_kind: ast::ClassSetBinaryOpKind,
        next_union: ast::ClassSetUnion,
    ) -&gt; ast::ClassSetUnion {
        <span class="kw">let </span>item = ast::ClassSet::Item(next_union.into_item());
        <span class="kw">let </span>new_lhs = <span class="self">self</span>.pop_class_op(item);
        <span class="self">self</span>.parser()
            .stack_class
            .borrow_mut()
            .push(ClassState::Op { kind: next_kind, lhs: new_lhs });
        ast::ClassSetUnion { span: <span class="self">self</span>.span(), items: <span class="macro">vec!</span>[] }
    }

    <span class="doccomment">/// Pop a character class set from the character class parser stack. If the
    /// top of the stack is just an item (not an operation), then return the
    /// given set unchanged. If the top of the stack is an operation, then the
    /// given set will be used as the rhs of the operation on the top of the
    /// stack. In that case, the binary operation is returned as a set.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>pop_class_op(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: ast::ClassSet) -&gt; ast::ClassSet {
        <span class="kw">let </span><span class="kw-2">mut </span>stack = <span class="self">self</span>.parser().stack_class.borrow_mut();
        <span class="kw">let </span>(kind, lhs) = <span class="kw">match </span>stack.pop() {
            <span class="prelude-val">Some</span>(ClassState::Op { kind, lhs }) =&gt; (kind, lhs),
            <span class="prelude-val">Some</span>(state @ ClassState::Open { .. }) =&gt; {
                stack.push(state);
                <span class="kw">return </span>rhs;
            }
            <span class="prelude-val">None </span>=&gt; <span class="macro">unreachable!</span>(),
        };
        <span class="kw">let </span>span = Span::new(lhs.span().start, rhs.span().end);
        ast::ClassSet::BinaryOp(ast::ClassSetBinaryOp {
            span,
            kind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'s</span>, P: Borrow&lt;Parser&gt;&gt; ParserI&lt;<span class="lifetime">'s</span>, P&gt; {
    <span class="doccomment">/// Parse the regular expression into an abstract syntax tree.
    </span><span class="kw">fn </span>parse(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Ast&gt; {
        <span class="self">self</span>.parse_with_comments().map(|astc| astc.ast)
    }

    <span class="doccomment">/// Parse the regular expression and return an abstract syntax tree with
    /// all of the comments found in the pattern.
    </span><span class="kw">fn </span>parse_with_comments(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;ast::WithComments&gt; {
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.offset(), <span class="number">0</span>, <span class="string">"parser can only be used once"</span>);
        <span class="self">self</span>.parser().reset();
        <span class="kw">let </span><span class="kw-2">mut </span>concat = ast::Concat { span: <span class="self">self</span>.span(), asts: <span class="macro">vec!</span>[] };
        <span class="kw">loop </span>{
            <span class="self">self</span>.bump_space();
            <span class="kw">if </span><span class="self">self</span>.is_eof() {
                <span class="kw">break</span>;
            }
            <span class="kw">match </span><span class="self">self</span>.char() {
                <span class="string">'(' </span>=&gt; concat = <span class="self">self</span>.push_group(concat)<span class="question-mark">?</span>,
                <span class="string">')' </span>=&gt; concat = <span class="self">self</span>.pop_group(concat)<span class="question-mark">?</span>,
                <span class="string">'|' </span>=&gt; concat = <span class="self">self</span>.push_alternate(concat)<span class="question-mark">?</span>,
                <span class="string">'[' </span>=&gt; {
                    <span class="kw">let </span>class = <span class="self">self</span>.parse_set_class()<span class="question-mark">?</span>;
                    concat.asts.push(Ast::class_bracketed(class));
                }
                <span class="string">'?' </span>=&gt; {
                    concat = <span class="self">self</span>.parse_uncounted_repetition(
                        concat,
                        ast::RepetitionKind::ZeroOrOne,
                    )<span class="question-mark">?</span>;
                }
                <span class="string">'*' </span>=&gt; {
                    concat = <span class="self">self</span>.parse_uncounted_repetition(
                        concat,
                        ast::RepetitionKind::ZeroOrMore,
                    )<span class="question-mark">?</span>;
                }
                <span class="string">'+' </span>=&gt; {
                    concat = <span class="self">self</span>.parse_uncounted_repetition(
                        concat,
                        ast::RepetitionKind::OneOrMore,
                    )<span class="question-mark">?</span>;
                }
                <span class="string">'{' </span>=&gt; {
                    concat = <span class="self">self</span>.parse_counted_repetition(concat)<span class="question-mark">?</span>;
                }
                <span class="kw">_ </span>=&gt; concat.asts.push(<span class="self">self</span>.parse_primitive()<span class="question-mark">?</span>.into_ast()),
            }
        }
        <span class="kw">let </span>ast = <span class="self">self</span>.pop_group_end(concat)<span class="question-mark">?</span>;
        NestLimiter::new(<span class="self">self</span>).check(<span class="kw-2">&amp;</span>ast)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(ast::WithComments {
            ast,
            comments: mem::replace(
                <span class="kw-2">&amp;mut *</span><span class="self">self</span>.parser().comments.borrow_mut(),
                <span class="macro">vec!</span>[],
            ),
        })
    }

    <span class="doccomment">/// Parses an uncounted repetition operation. An uncounted repetition
    /// operator includes ?, * and +, but does not include the {m,n} syntax.
    /// The given `kind` should correspond to the operator observed by the
    /// caller.
    ///
    /// This assumes that the parser is currently positioned at the repetition
    /// operator and advances the parser to the first character after the
    /// operator. (Note that the operator may include a single additional `?`,
    /// which makes the operator ungreedy.)
    ///
    /// The caller should include the concatenation that is being built. The
    /// concatenation returned includes the repetition operator applied to the
    /// last expression in the given concatenation.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_uncounted_repetition(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        <span class="kw-2">mut </span>concat: ast::Concat,
        kind: ast::RepetitionKind,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Concat&gt; {
        <span class="macro">assert!</span>(
            <span class="self">self</span>.char() == <span class="string">'?' </span>|| <span class="self">self</span>.char() == <span class="string">'*' </span>|| <span class="self">self</span>.char() == <span class="string">'+'
        </span>);
        <span class="kw">let </span>op_start = <span class="self">self</span>.pos();
        <span class="kw">let </span>ast = <span class="kw">match </span>concat.asts.pop() {
            <span class="prelude-val">Some</span>(ast) =&gt; ast,
            <span class="prelude-val">None </span>=&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(
                    <span class="self">self</span>.error(<span class="self">self</span>.span(), ast::ErrorKind::RepetitionMissing)
                )
            }
        };
        <span class="kw">match </span>ast {
            Ast::Empty(<span class="kw">_</span>) | Ast::Flags(<span class="kw">_</span>) =&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(
                    <span class="self">self</span>.error(<span class="self">self</span>.span(), ast::ErrorKind::RepetitionMissing)
                )
            }
            <span class="kw">_ </span>=&gt; {}
        }
        <span class="kw">let </span><span class="kw-2">mut </span>greedy = <span class="bool-val">true</span>;
        <span class="kw">if </span><span class="self">self</span>.bump() &amp;&amp; <span class="self">self</span>.char() == <span class="string">'?' </span>{
            greedy = <span class="bool-val">false</span>;
            <span class="self">self</span>.bump();
        }
        concat.asts.push(Ast::repetition(ast::Repetition {
            span: ast.span().with_end(<span class="self">self</span>.pos()),
            op: ast::RepetitionOp {
                span: Span::new(op_start, <span class="self">self</span>.pos()),
                kind,
            },
            greedy,
            ast: Box::new(ast),
        }));
        <span class="prelude-val">Ok</span>(concat)
    }

    <span class="doccomment">/// Parses a counted repetition operation. A counted repetition operator
    /// corresponds to the {m,n} syntax, and does not include the ?, * or +
    /// operators.
    ///
    /// This assumes that the parser is currently positioned at the opening `{`
    /// and advances the parser to the first character after the operator.
    /// (Note that the operator may include a single additional `?`, which
    /// makes the operator ungreedy.)
    ///
    /// The caller should include the concatenation that is being built. The
    /// concatenation returned includes the repetition operator applied to the
    /// last expression in the given concatenation.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_counted_repetition(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        <span class="kw-2">mut </span>concat: ast::Concat,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Concat&gt; {
        <span class="macro">assert!</span>(<span class="self">self</span>.char() == <span class="string">'{'</span>);
        <span class="kw">let </span>start = <span class="self">self</span>.pos();
        <span class="kw">let </span>ast = <span class="kw">match </span>concat.asts.pop() {
            <span class="prelude-val">Some</span>(ast) =&gt; ast,
            <span class="prelude-val">None </span>=&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(
                    <span class="self">self</span>.error(<span class="self">self</span>.span(), ast::ErrorKind::RepetitionMissing)
                )
            }
        };
        <span class="kw">match </span>ast {
            Ast::Empty(<span class="kw">_</span>) | Ast::Flags(<span class="kw">_</span>) =&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(
                    <span class="self">self</span>.error(<span class="self">self</span>.span(), ast::ErrorKind::RepetitionMissing)
                )
            }
            <span class="kw">_ </span>=&gt; {}
        }
        <span class="kw">if </span>!<span class="self">self</span>.bump_and_bump_space() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(start, <span class="self">self</span>.pos()),
                ast::ErrorKind::RepetitionCountUnclosed,
            ));
        }
        <span class="kw">let </span>count_start = specialize_err(
            <span class="self">self</span>.parse_decimal(),
            ast::ErrorKind::DecimalEmpty,
            ast::ErrorKind::RepetitionCountDecimalEmpty,
        );
        <span class="kw">if </span><span class="self">self</span>.is_eof() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(start, <span class="self">self</span>.pos()),
                ast::ErrorKind::RepetitionCountUnclosed,
            ));
        }
        <span class="kw">let </span>range = <span class="kw">if </span><span class="self">self</span>.char() == <span class="string">',' </span>{
            <span class="kw">if </span>!<span class="self">self</span>.bump_and_bump_space() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                    Span::new(start, <span class="self">self</span>.pos()),
                    ast::ErrorKind::RepetitionCountUnclosed,
                ));
            }
            <span class="kw">if </span><span class="self">self</span>.char() != <span class="string">'}' </span>{
                <span class="kw">let </span>count_start = <span class="kw">match </span>count_start {
                    <span class="prelude-val">Ok</span>(c) =&gt; c,
                    <span class="prelude-val">Err</span>(err)
                        <span class="kw">if </span>err.kind
                            == ast::ErrorKind::RepetitionCountDecimalEmpty =&gt;
                    {
                        <span class="kw">if </span><span class="self">self</span>.parser().empty_min_range {
                            <span class="number">0
                        </span>} <span class="kw">else </span>{
                            <span class="kw">return </span><span class="prelude-val">Err</span>(err);
                        }
                    }
                    err =&gt; err<span class="question-mark">?</span>,
                };
                <span class="kw">let </span>count_end = specialize_err(
                    <span class="self">self</span>.parse_decimal(),
                    ast::ErrorKind::DecimalEmpty,
                    ast::ErrorKind::RepetitionCountDecimalEmpty,
                )<span class="question-mark">?</span>;
                ast::RepetitionRange::Bounded(count_start, count_end)
            } <span class="kw">else </span>{
                ast::RepetitionRange::AtLeast(count_start<span class="question-mark">?</span>)
            }
        } <span class="kw">else </span>{
            ast::RepetitionRange::Exactly(count_start<span class="question-mark">?</span>)
        };

        <span class="kw">if </span><span class="self">self</span>.is_eof() || <span class="self">self</span>.char() != <span class="string">'}' </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(start, <span class="self">self</span>.pos()),
                ast::ErrorKind::RepetitionCountUnclosed,
            ));
        }

        <span class="kw">let </span><span class="kw-2">mut </span>greedy = <span class="bool-val">true</span>;
        <span class="kw">if </span><span class="self">self</span>.bump_and_bump_space() &amp;&amp; <span class="self">self</span>.char() == <span class="string">'?' </span>{
            greedy = <span class="bool-val">false</span>;
            <span class="self">self</span>.bump();
        }

        <span class="kw">let </span>op_span = Span::new(start, <span class="self">self</span>.pos());
        <span class="kw">if </span>!range.is_valid() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(
                <span class="self">self</span>.error(op_span, ast::ErrorKind::RepetitionCountInvalid)
            );
        }
        concat.asts.push(Ast::repetition(ast::Repetition {
            span: ast.span().with_end(<span class="self">self</span>.pos()),
            op: ast::RepetitionOp {
                span: op_span,
                kind: ast::RepetitionKind::Range(range),
            },
            greedy,
            ast: Box::new(ast),
        }));
        <span class="prelude-val">Ok</span>(concat)
    }

    <span class="doccomment">/// Parse a group (which contains a sub-expression) or a set of flags.
    ///
    /// If a group was found, then it is returned with an empty AST. If a set
    /// of flags is found, then that set is returned.
    ///
    /// The parser should be positioned at the opening parenthesis.
    ///
    /// This advances the parser to the character before the start of the
    /// sub-expression (in the case of a group) or to the closing parenthesis
    /// immediately following the set of flags.
    ///
    /// # Errors
    ///
    /// If flags are given and incorrectly specified, then a corresponding
    /// error is returned.
    ///
    /// If a capture name is given and it is incorrectly specified, then a
    /// corresponding error is returned.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_group(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Either&lt;ast::SetFlags, ast::Group&gt;&gt; {
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'('</span>);
        <span class="kw">let </span>open_span = <span class="self">self</span>.span_char();
        <span class="self">self</span>.bump();
        <span class="self">self</span>.bump_space();
        <span class="kw">if </span><span class="self">self</span>.is_lookaround_prefix() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(open_span.start, <span class="self">self</span>.span().end),
                ast::ErrorKind::UnsupportedLookAround,
            ));
        }
        <span class="kw">let </span>inner_span = <span class="self">self</span>.span();
        <span class="kw">let </span><span class="kw-2">mut </span>starts_with_p = <span class="bool-val">true</span>;
        <span class="kw">if </span><span class="self">self</span>.bump_if(<span class="string">"?P&lt;"</span>) || {
            starts_with_p = <span class="bool-val">false</span>;
            <span class="self">self</span>.bump_if(<span class="string">"?&lt;"</span>)
        } {
            <span class="kw">let </span>capture_index = <span class="self">self</span>.next_capture_index(open_span)<span class="question-mark">?</span>;
            <span class="kw">let </span>name = <span class="self">self</span>.parse_capture_name(capture_index)<span class="question-mark">?</span>;
            <span class="prelude-val">Ok</span>(Either::Right(ast::Group {
                span: open_span,
                kind: ast::GroupKind::CaptureName { starts_with_p, name },
                ast: Box::new(Ast::empty(<span class="self">self</span>.span())),
            }))
        } <span class="kw">else if </span><span class="self">self</span>.bump_if(<span class="string">"?"</span>) {
            <span class="kw">if </span><span class="self">self</span>.is_eof() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(
                    <span class="self">self</span>.error(open_span, ast::ErrorKind::GroupUnclosed)
                );
            }
            <span class="kw">let </span>flags = <span class="self">self</span>.parse_flags()<span class="question-mark">?</span>;
            <span class="kw">let </span>char_end = <span class="self">self</span>.char();
            <span class="self">self</span>.bump();
            <span class="kw">if </span>char_end == <span class="string">')' </span>{
                <span class="comment">// We don't allow empty flags, e.g., `(?)`. We instead
                // interpret it as a repetition operator missing its argument.
                </span><span class="kw">if </span>flags.items.is_empty() {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                        inner_span,
                        ast::ErrorKind::RepetitionMissing,
                    ));
                }
                <span class="prelude-val">Ok</span>(Either::Left(ast::SetFlags {
                    span: Span { end: <span class="self">self</span>.pos(), ..open_span },
                    flags,
                }))
            } <span class="kw">else </span>{
                <span class="macro">assert_eq!</span>(char_end, <span class="string">':'</span>);
                <span class="prelude-val">Ok</span>(Either::Right(ast::Group {
                    span: open_span,
                    kind: ast::GroupKind::NonCapturing(flags),
                    ast: Box::new(Ast::empty(<span class="self">self</span>.span())),
                }))
            }
        } <span class="kw">else </span>{
            <span class="kw">let </span>capture_index = <span class="self">self</span>.next_capture_index(open_span)<span class="question-mark">?</span>;
            <span class="prelude-val">Ok</span>(Either::Right(ast::Group {
                span: open_span,
                kind: ast::GroupKind::CaptureIndex(capture_index),
                ast: Box::new(Ast::empty(<span class="self">self</span>.span())),
            }))
        }
    }

    <span class="doccomment">/// Parses a capture group name. Assumes that the parser is positioned at
    /// the first character in the name following the opening `&lt;` (and may
    /// possibly be EOF). This advances the parser to the first character
    /// following the closing `&gt;`.
    ///
    /// The caller must provide the capture index of the group for this name.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_capture_name(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        capture_index: u32,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;ast::CaptureName&gt; {
        <span class="kw">if </span><span class="self">self</span>.is_eof() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self
                </span>.error(<span class="self">self</span>.span(), ast::ErrorKind::GroupNameUnexpectedEof));
        }
        <span class="kw">let </span>start = <span class="self">self</span>.pos();
        <span class="kw">loop </span>{
            <span class="kw">if </span><span class="self">self</span>.char() == <span class="string">'&gt;' </span>{
                <span class="kw">break</span>;
            }
            <span class="kw">if </span>!is_capture_char(<span class="self">self</span>.char(), <span class="self">self</span>.pos() == start) {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                    <span class="self">self</span>.span_char(),
                    ast::ErrorKind::GroupNameInvalid,
                ));
            }
            <span class="kw">if </span>!<span class="self">self</span>.bump() {
                <span class="kw">break</span>;
            }
        }
        <span class="kw">let </span>end = <span class="self">self</span>.pos();
        <span class="kw">if </span><span class="self">self</span>.is_eof() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self
                </span>.error(<span class="self">self</span>.span(), ast::ErrorKind::GroupNameUnexpectedEof));
        }
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'&gt;'</span>);
        <span class="self">self</span>.bump();
        <span class="kw">let </span>name = <span class="kw-2">&amp;</span><span class="self">self</span>.pattern()[start.offset..end.offset];
        <span class="kw">if </span>name.is_empty() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(start, start),
                ast::ErrorKind::GroupNameEmpty,
            ));
        }
        <span class="kw">let </span>capname = ast::CaptureName {
            span: Span::new(start, end),
            name: name.to_string(),
            index: capture_index,
        };
        <span class="self">self</span>.add_capture_name(<span class="kw-2">&amp;</span>capname)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(capname)
    }

    <span class="doccomment">/// Parse a sequence of flags starting at the current character.
    ///
    /// This advances the parser to the character immediately following the
    /// flags, which is guaranteed to be either `:` or `)`.
    ///
    /// # Errors
    ///
    /// If any flags are duplicated, then an error is returned.
    ///
    /// If the negation operator is used more than once, then an error is
    /// returned.
    ///
    /// If no flags could be found or if the negation operation is not followed
    /// by any flags, then an error is returned.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_flags(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Flags&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>flags = ast::Flags { span: <span class="self">self</span>.span(), items: <span class="macro">vec!</span>[] };
        <span class="kw">let </span><span class="kw-2">mut </span>last_was_negation = <span class="prelude-val">None</span>;
        <span class="kw">while </span><span class="self">self</span>.char() != <span class="string">':' </span>&amp;&amp; <span class="self">self</span>.char() != <span class="string">')' </span>{
            <span class="kw">if </span><span class="self">self</span>.char() == <span class="string">'-' </span>{
                last_was_negation = <span class="prelude-val">Some</span>(<span class="self">self</span>.span_char());
                <span class="kw">let </span>item = ast::FlagsItem {
                    span: <span class="self">self</span>.span_char(),
                    kind: ast::FlagsItemKind::Negation,
                };
                <span class="kw">if let </span><span class="prelude-val">Some</span>(i) = flags.add_item(item) {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                        <span class="self">self</span>.span_char(),
                        ast::ErrorKind::FlagRepeatedNegation {
                            original: flags.items[i].span,
                        },
                    ));
                }
            } <span class="kw">else </span>{
                last_was_negation = <span class="prelude-val">None</span>;
                <span class="kw">let </span>item = ast::FlagsItem {
                    span: <span class="self">self</span>.span_char(),
                    kind: ast::FlagsItemKind::Flag(<span class="self">self</span>.parse_flag()<span class="question-mark">?</span>),
                };
                <span class="kw">if let </span><span class="prelude-val">Some</span>(i) = flags.add_item(item) {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                        <span class="self">self</span>.span_char(),
                        ast::ErrorKind::FlagDuplicate {
                            original: flags.items[i].span,
                        },
                    ));
                }
            }
            <span class="kw">if </span>!<span class="self">self</span>.bump() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(
                    <span class="self">self</span>.error(<span class="self">self</span>.span(), ast::ErrorKind::FlagUnexpectedEof)
                );
            }
        }
        <span class="kw">if let </span><span class="prelude-val">Some</span>(span) = last_was_negation {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(span, ast::ErrorKind::FlagDanglingNegation));
        }
        flags.span.end = <span class="self">self</span>.pos();
        <span class="prelude-val">Ok</span>(flags)
    }

    <span class="doccomment">/// Parse the current character as a flag. Do not advance the parser.
    ///
    /// # Errors
    ///
    /// If the flag is not recognized, then an error is returned.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_flag(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Flag&gt; {
        <span class="kw">match </span><span class="self">self</span>.char() {
            <span class="string">'i' </span>=&gt; <span class="prelude-val">Ok</span>(ast::Flag::CaseInsensitive),
            <span class="string">'m' </span>=&gt; <span class="prelude-val">Ok</span>(ast::Flag::MultiLine),
            <span class="string">'s' </span>=&gt; <span class="prelude-val">Ok</span>(ast::Flag::DotMatchesNewLine),
            <span class="string">'U' </span>=&gt; <span class="prelude-val">Ok</span>(ast::Flag::SwapGreed),
            <span class="string">'u' </span>=&gt; <span class="prelude-val">Ok</span>(ast::Flag::Unicode),
            <span class="string">'R' </span>=&gt; <span class="prelude-val">Ok</span>(ast::Flag::CRLF),
            <span class="string">'x' </span>=&gt; <span class="prelude-val">Ok</span>(ast::Flag::IgnoreWhitespace),
            <span class="kw">_ </span>=&gt; {
                <span class="prelude-val">Err</span>(<span class="self">self
                    </span>.error(<span class="self">self</span>.span_char(), ast::ErrorKind::FlagUnrecognized))
            }
        }
    }

    <span class="doccomment">/// Parse a primitive AST. e.g., A literal, non-set character class or
    /// assertion.
    ///
    /// This assumes that the parser expects a primitive at the current
    /// location. i.e., All other non-primitive cases have been handled.
    /// For example, if the parser's position is at `|`, then `|` will be
    /// treated as a literal (e.g., inside a character class).
    ///
    /// This advances the parser to the first character immediately following
    /// the primitive.
    </span><span class="kw">fn </span>parse_primitive(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Primitive&gt; {
        <span class="kw">match </span><span class="self">self</span>.char() {
            <span class="string">'\\' </span>=&gt; <span class="self">self</span>.parse_escape(),
            <span class="string">'.' </span>=&gt; {
                <span class="kw">let </span>ast = Primitive::Dot(<span class="self">self</span>.span_char());
                <span class="self">self</span>.bump();
                <span class="prelude-val">Ok</span>(ast)
            }
            <span class="string">'^' </span>=&gt; {
                <span class="kw">let </span>ast = Primitive::Assertion(ast::Assertion {
                    span: <span class="self">self</span>.span_char(),
                    kind: ast::AssertionKind::StartLine,
                });
                <span class="self">self</span>.bump();
                <span class="prelude-val">Ok</span>(ast)
            }
            <span class="string">'$' </span>=&gt; {
                <span class="kw">let </span>ast = Primitive::Assertion(ast::Assertion {
                    span: <span class="self">self</span>.span_char(),
                    kind: ast::AssertionKind::EndLine,
                });
                <span class="self">self</span>.bump();
                <span class="prelude-val">Ok</span>(ast)
            }
            c =&gt; {
                <span class="kw">let </span>ast = Primitive::Literal(ast::Literal {
                    span: <span class="self">self</span>.span_char(),
                    kind: ast::LiteralKind::Verbatim,
                    c,
                });
                <span class="self">self</span>.bump();
                <span class="prelude-val">Ok</span>(ast)
            }
        }
    }

    <span class="doccomment">/// Parse an escape sequence as a primitive AST.
    ///
    /// This assumes the parser is positioned at the start of the escape
    /// sequence, i.e., `\`. It advances the parser to the first position
    /// immediately following the escape sequence.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_escape(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Primitive&gt; {
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'\\'</span>);
        <span class="kw">let </span>start = <span class="self">self</span>.pos();
        <span class="kw">if </span>!<span class="self">self</span>.bump() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(start, <span class="self">self</span>.pos()),
                ast::ErrorKind::EscapeUnexpectedEof,
            ));
        }
        <span class="kw">let </span>c = <span class="self">self</span>.char();
        <span class="comment">// Put some of the more complicated routines into helpers.
        </span><span class="kw">match </span>c {
            <span class="string">'0'</span>..=<span class="string">'7' </span>=&gt; {
                <span class="kw">if </span>!<span class="self">self</span>.parser().octal {
                    <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                        Span::new(start, <span class="self">self</span>.span_char().end),
                        ast::ErrorKind::UnsupportedBackreference,
                    ));
                }
                <span class="kw">let </span><span class="kw-2">mut </span>lit = <span class="self">self</span>.parse_octal();
                lit.span.start = start;
                <span class="kw">return </span><span class="prelude-val">Ok</span>(Primitive::Literal(lit));
            }
            <span class="string">'8'</span>..=<span class="string">'9' </span><span class="kw">if </span>!<span class="self">self</span>.parser().octal =&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                    Span::new(start, <span class="self">self</span>.span_char().end),
                    ast::ErrorKind::UnsupportedBackreference,
                ));
            }
            <span class="string">'x' </span>| <span class="string">'u' </span>| <span class="string">'U' </span>=&gt; {
                <span class="kw">let </span><span class="kw-2">mut </span>lit = <span class="self">self</span>.parse_hex()<span class="question-mark">?</span>;
                lit.span.start = start;
                <span class="kw">return </span><span class="prelude-val">Ok</span>(Primitive::Literal(lit));
            }
            <span class="string">'p' </span>| <span class="string">'P' </span>=&gt; {
                <span class="kw">let </span><span class="kw-2">mut </span>cls = <span class="self">self</span>.parse_unicode_class()<span class="question-mark">?</span>;
                cls.span.start = start;
                <span class="kw">return </span><span class="prelude-val">Ok</span>(Primitive::Unicode(cls));
            }
            <span class="string">'d' </span>| <span class="string">'s' </span>| <span class="string">'w' </span>| <span class="string">'D' </span>| <span class="string">'S' </span>| <span class="string">'W' </span>=&gt; {
                <span class="kw">let </span><span class="kw-2">mut </span>cls = <span class="self">self</span>.parse_perl_class();
                cls.span.start = start;
                <span class="kw">return </span><span class="prelude-val">Ok</span>(Primitive::Perl(cls));
            }
            <span class="kw">_ </span>=&gt; {}
        }

        <span class="comment">// Handle all of the one letter sequences inline.
        </span><span class="self">self</span>.bump();
        <span class="kw">let </span>span = Span::new(start, <span class="self">self</span>.pos());
        <span class="kw">if </span>is_meta_character(c) {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span,
                kind: ast::LiteralKind::Meta,
                c,
            }));
        }
        <span class="kw">if </span>is_escapeable_character(c) {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span,
                kind: ast::LiteralKind::Superfluous,
                c,
            }));
        }
        <span class="kw">let </span>special = |kind, c| {
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span,
                kind: ast::LiteralKind::Special(kind),
                c,
            }))
        };
        <span class="kw">match </span>c {
            <span class="string">'a' </span>=&gt; special(ast::SpecialLiteralKind::Bell, <span class="string">'\x07'</span>),
            <span class="string">'f' </span>=&gt; special(ast::SpecialLiteralKind::FormFeed, <span class="string">'\x0C'</span>),
            <span class="string">'t' </span>=&gt; special(ast::SpecialLiteralKind::Tab, <span class="string">'\t'</span>),
            <span class="string">'n' </span>=&gt; special(ast::SpecialLiteralKind::LineFeed, <span class="string">'\n'</span>),
            <span class="string">'r' </span>=&gt; special(ast::SpecialLiteralKind::CarriageReturn, <span class="string">'\r'</span>),
            <span class="string">'v' </span>=&gt; special(ast::SpecialLiteralKind::VerticalTab, <span class="string">'\x0B'</span>),
            <span class="string">'A' </span>=&gt; <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span,
                kind: ast::AssertionKind::StartText,
            })),
            <span class="string">'z' </span>=&gt; <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span,
                kind: ast::AssertionKind::EndText,
            })),
            <span class="string">'b' </span>=&gt; {
                <span class="kw">let </span><span class="kw-2">mut </span>wb = ast::Assertion {
                    span,
                    kind: ast::AssertionKind::WordBoundary,
                };
                <span class="comment">// After a \b, we "try" to parse things like \b{start} for
                // special word boundary assertions.
                </span><span class="kw">if </span>!<span class="self">self</span>.is_eof() &amp;&amp; <span class="self">self</span>.char() == <span class="string">'{' </span>{
                    <span class="kw">if let </span><span class="prelude-val">Some</span>(kind) =
                        <span class="self">self</span>.maybe_parse_special_word_boundary(start)<span class="question-mark">?
                    </span>{
                        wb.kind = kind;
                        wb.span.end = <span class="self">self</span>.pos();
                    }
                }
                <span class="prelude-val">Ok</span>(Primitive::Assertion(wb))
            }
            <span class="string">'B' </span>=&gt; <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span,
                kind: ast::AssertionKind::NotWordBoundary,
            })),
            <span class="string">'&lt;' </span>=&gt; <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span,
                kind: ast::AssertionKind::WordBoundaryStartAngle,
            })),
            <span class="string">'&gt;' </span>=&gt; <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span,
                kind: ast::AssertionKind::WordBoundaryEndAngle,
            })),
            <span class="kw">_ </span>=&gt; <span class="prelude-val">Err</span>(<span class="self">self</span>.error(span, ast::ErrorKind::EscapeUnrecognized)),
        }
    }

    <span class="doccomment">/// Attempt to parse a specialty word boundary. That is, `\b{start}`,
    /// `\b{end}`, `\b{start-half}` or `\b{end-half}`.
    ///
    /// This is similar to `maybe_parse_ascii_class` in that, in most cases,
    /// if it fails it will just return `None` with no error. This is done
    /// because `\b{5}` is a valid expression and we want to let that be parsed
    /// by the existing counted repetition parsing code. (I thought about just
    /// invoking the counted repetition code from here, but it seemed a little
    /// ham-fisted.)
    ///
    /// Unlike `maybe_parse_ascii_class` though, this can return an error.
    /// Namely, if we definitely know it isn't a counted repetition, then we
    /// return an error specific to the specialty word boundaries.
    ///
    /// This assumes the parser is positioned at a `{` immediately following
    /// a `\b`. When `None` is returned, the parser is returned to the position
    /// at which it started: pointing at a `{`.
    ///
    /// The position given should correspond to the start of the `\b`.
    </span><span class="kw">fn </span>maybe_parse_special_word_boundary(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        wb_start: Position,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="prelude-ty">Option</span>&lt;ast::AssertionKind&gt;&gt; {
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'{'</span>);

        <span class="kw">let </span>is_valid_char = |c| <span class="kw">match </span>c {
            <span class="string">'A'</span>..=<span class="string">'Z' </span>| <span class="string">'a'</span>..=<span class="string">'z' </span>| <span class="string">'-' </span>=&gt; <span class="bool-val">true</span>,
            <span class="kw">_ </span>=&gt; <span class="bool-val">false</span>,
        };
        <span class="kw">let </span>start = <span class="self">self</span>.pos();
        <span class="kw">if </span>!<span class="self">self</span>.bump_and_bump_space() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(wb_start, <span class="self">self</span>.pos()),
                ast::ErrorKind::SpecialWordOrRepetitionUnexpectedEof,
            ));
        }
        <span class="kw">let </span>start_contents = <span class="self">self</span>.pos();
        <span class="comment">// This is one of the critical bits: if the first non-whitespace
        // character isn't in [-A-Za-z] (i.e., this can't be a special word
        // boundary), then we bail and let the counted repetition parser deal
        // with this.
        </span><span class="kw">if </span>!is_valid_char(<span class="self">self</span>.char()) {
            <span class="self">self</span>.parser().pos.set(start);
            <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="prelude-val">None</span>);
        }

        <span class="comment">// Now collect up our chars until we see a '}'.
        </span><span class="kw">let </span><span class="kw-2">mut </span>scratch = <span class="self">self</span>.parser().scratch.borrow_mut();
        scratch.clear();
        <span class="kw">while </span>!<span class="self">self</span>.is_eof() &amp;&amp; is_valid_char(<span class="self">self</span>.char()) {
            scratch.push(<span class="self">self</span>.char());
            <span class="self">self</span>.bump_and_bump_space();
        }
        <span class="kw">if </span><span class="self">self</span>.is_eof() || <span class="self">self</span>.char() != <span class="string">'}' </span>{
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(start, <span class="self">self</span>.pos()),
                ast::ErrorKind::SpecialWordBoundaryUnclosed,
            ));
        }
        <span class="kw">let </span>end = <span class="self">self</span>.pos();
        <span class="self">self</span>.bump();
        <span class="kw">let </span>kind = <span class="kw">match </span>scratch.as_str() {
            <span class="string">"start" </span>=&gt; ast::AssertionKind::WordBoundaryStart,
            <span class="string">"end" </span>=&gt; ast::AssertionKind::WordBoundaryEnd,
            <span class="string">"start-half" </span>=&gt; ast::AssertionKind::WordBoundaryStartHalf,
            <span class="string">"end-half" </span>=&gt; ast::AssertionKind::WordBoundaryEndHalf,
            <span class="kw">_ </span>=&gt; {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                    Span::new(start_contents, end),
                    ast::ErrorKind::SpecialWordBoundaryUnrecognized,
                ))
            }
        };
        <span class="prelude-val">Ok</span>(<span class="prelude-val">Some</span>(kind))
    }

    <span class="doccomment">/// Parse an octal representation of a Unicode codepoint up to 3 digits
    /// long. This expects the parser to be positioned at the first octal
    /// digit and advances the parser to the first character immediately
    /// following the octal number. This also assumes that parsing octal
    /// escapes is enabled.
    ///
    /// Assuming the preconditions are met, this routine can never fail.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_octal(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; ast::Literal {
        <span class="macro">assert!</span>(<span class="self">self</span>.parser().octal);
        <span class="macro">assert!</span>(<span class="string">'0' </span>&lt;= <span class="self">self</span>.char() &amp;&amp; <span class="self">self</span>.char() &lt;= <span class="string">'7'</span>);
        <span class="kw">let </span>start = <span class="self">self</span>.pos();
        <span class="comment">// Parse up to two more digits.
        </span><span class="kw">while </span><span class="self">self</span>.bump()
            &amp;&amp; <span class="string">'0' </span>&lt;= <span class="self">self</span>.char()
            &amp;&amp; <span class="self">self</span>.char() &lt;= <span class="string">'7'
            </span>&amp;&amp; <span class="self">self</span>.pos().offset - start.offset &lt;= <span class="number">2
        </span>{}
        <span class="kw">let </span>end = <span class="self">self</span>.pos();
        <span class="kw">let </span>octal = <span class="kw-2">&amp;</span><span class="self">self</span>.pattern()[start.offset..end.offset];
        <span class="comment">// Parsing the octal should never fail since the above guarantees a
        // valid number.
        </span><span class="kw">let </span>codepoint =
            u32::from_str_radix(octal, <span class="number">8</span>).expect(<span class="string">"valid octal number"</span>);
        <span class="comment">// The max value for 3 digit octal is 0777 = 511 and [0, 511] has no
        // invalid Unicode scalar values.
        </span><span class="kw">let </span>c = char::from_u32(codepoint).expect(<span class="string">"Unicode scalar value"</span>);
        ast::Literal {
            span: Span::new(start, end),
            kind: ast::LiteralKind::Octal,
            c,
        }
    }

    <span class="doccomment">/// Parse a hex representation of a Unicode codepoint. This handles both
    /// hex notations, i.e., `\xFF` and `\x{FFFF}`. This expects the parser to
    /// be positioned at the `x`, `u` or `U` prefix. The parser is advanced to
    /// the first character immediately following the hexadecimal literal.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_hex(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Literal&gt; {
        <span class="macro">assert!</span>(
            <span class="self">self</span>.char() == <span class="string">'x' </span>|| <span class="self">self</span>.char() == <span class="string">'u' </span>|| <span class="self">self</span>.char() == <span class="string">'U'
        </span>);

        <span class="kw">let </span>hex_kind = <span class="kw">match </span><span class="self">self</span>.char() {
            <span class="string">'x' </span>=&gt; ast::HexLiteralKind::X,
            <span class="string">'u' </span>=&gt; ast::HexLiteralKind::UnicodeShort,
            <span class="kw">_ </span>=&gt; ast::HexLiteralKind::UnicodeLong,
        };
        <span class="kw">if </span>!<span class="self">self</span>.bump_and_bump_space() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(
                <span class="self">self</span>.error(<span class="self">self</span>.span(), ast::ErrorKind::EscapeUnexpectedEof)
            );
        }
        <span class="kw">if </span><span class="self">self</span>.char() == <span class="string">'{' </span>{
            <span class="self">self</span>.parse_hex_brace(hex_kind)
        } <span class="kw">else </span>{
            <span class="self">self</span>.parse_hex_digits(hex_kind)
        }
    }

    <span class="doccomment">/// Parse an N-digit hex representation of a Unicode codepoint. This
    /// expects the parser to be positioned at the first digit and will advance
    /// the parser to the first character immediately following the escape
    /// sequence.
    ///
    /// The number of digits given must be 2 (for `\xNN`), 4 (for `\uNNNN`)
    /// or 8 (for `\UNNNNNNNN`).
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_hex_digits(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        kind: ast::HexLiteralKind,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Literal&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>scratch = <span class="self">self</span>.parser().scratch.borrow_mut();
        scratch.clear();

        <span class="kw">let </span>start = <span class="self">self</span>.pos();
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..kind.digits() {
            <span class="kw">if </span>i &gt; <span class="number">0 </span>&amp;&amp; !<span class="self">self</span>.bump_and_bump_space() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self
                    </span>.error(<span class="self">self</span>.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }
            <span class="kw">if </span>!is_hex(<span class="self">self</span>.char()) {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                    <span class="self">self</span>.span_char(),
                    ast::ErrorKind::EscapeHexInvalidDigit,
                ));
            }
            scratch.push(<span class="self">self</span>.char());
        }
        <span class="comment">// The final bump just moves the parser past the literal, which may
        // be EOF.
        </span><span class="self">self</span>.bump_and_bump_space();
        <span class="kw">let </span>end = <span class="self">self</span>.pos();
        <span class="kw">let </span>hex = scratch.as_str();
        <span class="kw">match </span>u32::from_str_radix(hex, <span class="number">16</span>).ok().and_then(char::from_u32) {
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(start, end),
                ast::ErrorKind::EscapeHexInvalid,
            )),
            <span class="prelude-val">Some</span>(c) =&gt; <span class="prelude-val">Ok</span>(ast::Literal {
                span: Span::new(start, end),
                kind: ast::LiteralKind::HexFixed(kind),
                c,
            }),
        }
    }

    <span class="doccomment">/// Parse a hex representation of any Unicode scalar value. This expects
    /// the parser to be positioned at the opening brace `{` and will advance
    /// the parser to the first character following the closing brace `}`.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_hex_brace(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        kind: ast::HexLiteralKind,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;ast::Literal&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>scratch = <span class="self">self</span>.parser().scratch.borrow_mut();
        scratch.clear();

        <span class="kw">let </span>brace_pos = <span class="self">self</span>.pos();
        <span class="kw">let </span>start = <span class="self">self</span>.span_char().end;
        <span class="kw">while </span><span class="self">self</span>.bump_and_bump_space() &amp;&amp; <span class="self">self</span>.char() != <span class="string">'}' </span>{
            <span class="kw">if </span>!is_hex(<span class="self">self</span>.char()) {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                    <span class="self">self</span>.span_char(),
                    ast::ErrorKind::EscapeHexInvalidDigit,
                ));
            }
            scratch.push(<span class="self">self</span>.char());
        }
        <span class="kw">if </span><span class="self">self</span>.is_eof() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(brace_pos, <span class="self">self</span>.pos()),
                ast::ErrorKind::EscapeUnexpectedEof,
            ));
        }
        <span class="kw">let </span>end = <span class="self">self</span>.pos();
        <span class="kw">let </span>hex = scratch.as_str();
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'}'</span>);
        <span class="self">self</span>.bump_and_bump_space();

        <span class="kw">if </span>hex.is_empty() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(brace_pos, <span class="self">self</span>.pos()),
                ast::ErrorKind::EscapeHexEmpty,
            ));
        }
        <span class="kw">match </span>u32::from_str_radix(hex, <span class="number">16</span>).ok().and_then(char::from_u32) {
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(start, end),
                ast::ErrorKind::EscapeHexInvalid,
            )),
            <span class="prelude-val">Some</span>(c) =&gt; <span class="prelude-val">Ok</span>(ast::Literal {
                span: Span::new(start, <span class="self">self</span>.pos()),
                kind: ast::LiteralKind::HexBrace(kind),
                c,
            }),
        }
    }

    <span class="doccomment">/// Parse a decimal number into a u32 while trimming leading and trailing
    /// whitespace.
    ///
    /// This expects the parser to be positioned at the first position where
    /// a decimal digit could occur. This will advance the parser to the byte
    /// immediately following the last contiguous decimal digit.
    ///
    /// If no decimal digit could be found or if there was a problem parsing
    /// the complete set of digits into a u32, then an error is returned.
    </span><span class="kw">fn </span>parse_decimal(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;u32&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>scratch = <span class="self">self</span>.parser().scratch.borrow_mut();
        scratch.clear();

        <span class="kw">while </span>!<span class="self">self</span>.is_eof() &amp;&amp; <span class="self">self</span>.char().is_whitespace() {
            <span class="self">self</span>.bump();
        }
        <span class="kw">let </span>start = <span class="self">self</span>.pos();
        <span class="kw">while </span>!<span class="self">self</span>.is_eof() &amp;&amp; <span class="string">'0' </span>&lt;= <span class="self">self</span>.char() &amp;&amp; <span class="self">self</span>.char() &lt;= <span class="string">'9' </span>{
            scratch.push(<span class="self">self</span>.char());
            <span class="self">self</span>.bump_and_bump_space();
        }
        <span class="kw">let </span>span = Span::new(start, <span class="self">self</span>.pos());
        <span class="kw">while </span>!<span class="self">self</span>.is_eof() &amp;&amp; <span class="self">self</span>.char().is_whitespace() {
            <span class="self">self</span>.bump_and_bump_space();
        }
        <span class="kw">let </span>digits = scratch.as_str();
        <span class="kw">if </span>digits.is_empty() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(span, ast::ErrorKind::DecimalEmpty));
        }
        <span class="kw">match </span>u32::from_str_radix(digits, <span class="number">10</span>).ok() {
            <span class="prelude-val">Some</span>(n) =&gt; <span class="prelude-val">Ok</span>(n),
            <span class="prelude-val">None </span>=&gt; <span class="prelude-val">Err</span>(<span class="self">self</span>.error(span, ast::ErrorKind::DecimalInvalid)),
        }
    }

    <span class="doccomment">/// Parse a standard character class consisting primarily of characters or
    /// character ranges, but can also contain nested character classes of
    /// any type (sans `.`).
    ///
    /// This assumes the parser is positioned at the opening `[`. If parsing
    /// is successful, then the parser is advanced to the position immediately
    /// following the closing `]`.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_set_class(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;ast::ClassBracketed&gt; {
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'['</span>);

        <span class="kw">let </span><span class="kw-2">mut </span>union =
            ast::ClassSetUnion { span: <span class="self">self</span>.span(), items: <span class="macro">vec!</span>[] };
        <span class="kw">loop </span>{
            <span class="self">self</span>.bump_space();
            <span class="kw">if </span><span class="self">self</span>.is_eof() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.unclosed_class_error());
            }
            <span class="kw">match </span><span class="self">self</span>.char() {
                <span class="string">'[' </span>=&gt; {
                    <span class="comment">// If we've already parsed the opening bracket, then
                    // attempt to treat this as the beginning of an ASCII
                    // class. If ASCII class parsing fails, then the parser
                    // backs up to `[`.
                    </span><span class="kw">if </span>!<span class="self">self</span>.parser().stack_class.borrow().is_empty() {
                        <span class="kw">if let </span><span class="prelude-val">Some</span>(cls) = <span class="self">self</span>.maybe_parse_ascii_class() {
                            union.push(ast::ClassSetItem::Ascii(cls));
                            <span class="kw">continue</span>;
                        }
                    }
                    union = <span class="self">self</span>.push_class_open(union)<span class="question-mark">?</span>;
                }
                <span class="string">']' </span>=&gt; <span class="kw">match </span><span class="self">self</span>.pop_class(union)<span class="question-mark">? </span>{
                    Either::Left(nested_union) =&gt; {
                        union = nested_union;
                    }
                    Either::Right(class) =&gt; <span class="kw">return </span><span class="prelude-val">Ok</span>(class),
                },
                <span class="string">'&amp;' </span><span class="kw">if </span><span class="self">self</span>.peek() == <span class="prelude-val">Some</span>(<span class="string">'&amp;'</span>) =&gt; {
                    <span class="macro">assert!</span>(<span class="self">self</span>.bump_if(<span class="string">"&amp;&amp;"</span>));
                    union = <span class="self">self</span>.push_class_op(
                        ast::ClassSetBinaryOpKind::Intersection,
                        union,
                    );
                }
                <span class="string">'-' </span><span class="kw">if </span><span class="self">self</span>.peek() == <span class="prelude-val">Some</span>(<span class="string">'-'</span>) =&gt; {
                    <span class="macro">assert!</span>(<span class="self">self</span>.bump_if(<span class="string">"--"</span>));
                    union = <span class="self">self</span>.push_class_op(
                        ast::ClassSetBinaryOpKind::Difference,
                        union,
                    );
                }
                <span class="string">'~' </span><span class="kw">if </span><span class="self">self</span>.peek() == <span class="prelude-val">Some</span>(<span class="string">'~'</span>) =&gt; {
                    <span class="macro">assert!</span>(<span class="self">self</span>.bump_if(<span class="string">"~~"</span>));
                    union = <span class="self">self</span>.push_class_op(
                        ast::ClassSetBinaryOpKind::SymmetricDifference,
                        union,
                    );
                }
                <span class="kw">_ </span>=&gt; {
                    union.push(<span class="self">self</span>.parse_set_class_range()<span class="question-mark">?</span>);
                }
            }
        }
    }

    <span class="doccomment">/// Parse a single primitive item in a character class set. The item to
    /// be parsed can either be one of a simple literal character, a range
    /// between two simple literal characters or a "primitive" character
    /// class like \w or \p{Greek}.
    ///
    /// If an invalid escape is found, or if a character class is found where
    /// a simple literal is expected (e.g., in a range), then an error is
    /// returned.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_set_class_range(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;ast::ClassSetItem&gt; {
        <span class="kw">let </span>prim1 = <span class="self">self</span>.parse_set_class_item()<span class="question-mark">?</span>;
        <span class="self">self</span>.bump_space();
        <span class="kw">if </span><span class="self">self</span>.is_eof() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.unclosed_class_error());
        }
        <span class="comment">// If the next char isn't a `-`, then we don't have a range.
        // There are two exceptions. If the char after a `-` is a `]`, then
        // `-` is interpreted as a literal `-`. Alternatively, if the char
        // after a `-` is a `-`, then `--` corresponds to a "difference"
        // operation.
        </span><span class="kw">if </span><span class="self">self</span>.char() != <span class="string">'-'
            </span>|| <span class="self">self</span>.peek_space() == <span class="prelude-val">Some</span>(<span class="string">']'</span>)
            || <span class="self">self</span>.peek_space() == <span class="prelude-val">Some</span>(<span class="string">'-'</span>)
        {
            <span class="kw">return </span>prim1.into_class_set_item(<span class="self">self</span>);
        }
        <span class="comment">// OK, now we're parsing a range, so bump past the `-` and parse the
        // second half of the range.
        </span><span class="kw">if </span>!<span class="self">self</span>.bump_and_bump_space() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.unclosed_class_error());
        }
        <span class="kw">let </span>prim2 = <span class="self">self</span>.parse_set_class_item()<span class="question-mark">?</span>;
        <span class="kw">let </span>range = ast::ClassSetRange {
            span: Span::new(prim1.span().start, prim2.span().end),
            start: prim1.into_class_literal(<span class="self">self</span>)<span class="question-mark">?</span>,
            end: prim2.into_class_literal(<span class="self">self</span>)<span class="question-mark">?</span>,
        };
        <span class="kw">if </span>!range.is_valid() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(
                <span class="self">self</span>.error(range.span, ast::ErrorKind::ClassRangeInvalid)
            );
        }
        <span class="prelude-val">Ok</span>(ast::ClassSetItem::Range(range))
    }

    <span class="doccomment">/// Parse a single item in a character class as a primitive, where the
    /// primitive either consists of a verbatim literal or a single escape
    /// sequence.
    ///
    /// This assumes the parser is positioned at the beginning of a primitive,
    /// and advances the parser to the first position after the primitive if
    /// successful.
    ///
    /// Note that it is the caller's responsibility to report an error if an
    /// illegal primitive was parsed.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_set_class_item(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;Primitive&gt; {
        <span class="kw">if </span><span class="self">self</span>.char() == <span class="string">'\\' </span>{
            <span class="self">self</span>.parse_escape()
        } <span class="kw">else </span>{
            <span class="kw">let </span>x = Primitive::Literal(ast::Literal {
                span: <span class="self">self</span>.span_char(),
                kind: ast::LiteralKind::Verbatim,
                c: <span class="self">self</span>.char(),
            });
            <span class="self">self</span>.bump();
            <span class="prelude-val">Ok</span>(x)
        }
    }

    <span class="doccomment">/// Parses the opening of a character class set. This includes the opening
    /// bracket along with `^` if present to indicate negation. This also
    /// starts parsing the opening set of unioned items if applicable, since
    /// there are special rules applied to certain characters in the opening
    /// of a character class. For example, `[^]]` is the class of all
    /// characters not equal to `]`. (`]` would need to be escaped in any other
    /// position.) Similarly for `-`.
    ///
    /// In all cases, the op inside the returned `ast::ClassBracketed` is an
    /// empty union. This empty union should be replaced with the actual item
    /// when it is popped from the parser's stack.
    ///
    /// This assumes the parser is positioned at the opening `[` and advances
    /// the parser to the first non-special byte of the character class.
    ///
    /// An error is returned if EOF is found.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_set_class_open(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(ast::ClassBracketed, ast::ClassSetUnion)&gt; {
        <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'['</span>);
        <span class="kw">let </span>start = <span class="self">self</span>.pos();
        <span class="kw">if </span>!<span class="self">self</span>.bump_and_bump_space() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                Span::new(start, <span class="self">self</span>.pos()),
                ast::ErrorKind::ClassUnclosed,
            ));
        }

        <span class="kw">let </span>negated = <span class="kw">if </span><span class="self">self</span>.char() != <span class="string">'^' </span>{
            <span class="bool-val">false
        </span>} <span class="kw">else </span>{
            <span class="kw">if </span>!<span class="self">self</span>.bump_and_bump_space() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                    Span::new(start, <span class="self">self</span>.pos()),
                    ast::ErrorKind::ClassUnclosed,
                ));
            }
            <span class="bool-val">true
        </span>};
        <span class="comment">// Accept any number of `-` as literal `-`.
        </span><span class="kw">let </span><span class="kw-2">mut </span>union =
            ast::ClassSetUnion { span: <span class="self">self</span>.span(), items: <span class="macro">vec!</span>[] };
        <span class="kw">while </span><span class="self">self</span>.char() == <span class="string">'-' </span>{
            union.push(ast::ClassSetItem::Literal(ast::Literal {
                span: <span class="self">self</span>.span_char(),
                kind: ast::LiteralKind::Verbatim,
                c: <span class="string">'-'</span>,
            }));
            <span class="kw">if </span>!<span class="self">self</span>.bump_and_bump_space() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                    Span::new(start, start),
                    ast::ErrorKind::ClassUnclosed,
                ));
            }
        }
        <span class="comment">// If `]` is the *first* char in a set, then interpret it as a literal
        // `]`. That is, an empty class is impossible to write.
        </span><span class="kw">if </span>union.items.is_empty() &amp;&amp; <span class="self">self</span>.char() == <span class="string">']' </span>{
            union.push(ast::ClassSetItem::Literal(ast::Literal {
                span: <span class="self">self</span>.span_char(),
                kind: ast::LiteralKind::Verbatim,
                c: <span class="string">']'</span>,
            }));
            <span class="kw">if </span>!<span class="self">self</span>.bump_and_bump_space() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                    Span::new(start, <span class="self">self</span>.pos()),
                    ast::ErrorKind::ClassUnclosed,
                ));
            }
        }
        <span class="kw">let </span>set = ast::ClassBracketed {
            span: Span::new(start, <span class="self">self</span>.pos()),
            negated,
            kind: ast::ClassSet::union(ast::ClassSetUnion {
                span: Span::new(union.span.start, union.span.start),
                items: <span class="macro">vec!</span>[],
            }),
        };
        <span class="prelude-val">Ok</span>((set, union))
    }

    <span class="doccomment">/// Attempt to parse an ASCII character class, e.g., `[:alnum:]`.
    ///
    /// This assumes the parser is positioned at the opening `[`.
    ///
    /// If no valid ASCII character class could be found, then this does not
    /// advance the parser and `None` is returned. Otherwise, the parser is
    /// advanced to the first byte following the closing `]` and the
    /// corresponding ASCII class is returned.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>maybe_parse_ascii_class(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;ast::ClassAscii&gt; {
        <span class="comment">// ASCII character classes are interesting from a parsing perspective
        // because parsing cannot fail with any interesting error. For example,
        // in order to use an ASCII character class, it must be enclosed in
        // double brackets, e.g., `[[:alnum:]]`. Alternatively, you might think
        // of it as "ASCII character classes have the syntax `[:NAME:]` which
        // can only appear within character brackets." This means that things
        // like `[[:lower:]A]` are legal constructs.
        //
        // However, if one types an incorrect ASCII character class, e.g.,
        // `[[:loower:]]`, then we treat that as a normal nested character
        // class containing the characters `:elorw`. One might argue that we
        // should return an error instead since the repeated colons give away
        // the intent to write an ASCII class. But what if the user typed
        // `[[:lower]]` instead? How can we tell that was intended to be an
        // ASCII class and not just a normal nested class?
        //
        // Reasonable people can probably disagree over this, but for better
        // or worse, we implement semantics that never fails at the expense
        // of better failure modes.
        </span><span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'['</span>);
        <span class="comment">// If parsing fails, then we back up the parser to this starting point.
        </span><span class="kw">let </span>start = <span class="self">self</span>.pos();
        <span class="kw">let </span><span class="kw-2">mut </span>negated = <span class="bool-val">false</span>;
        <span class="kw">if </span>!<span class="self">self</span>.bump() || <span class="self">self</span>.char() != <span class="string">':' </span>{
            <span class="self">self</span>.parser().pos.set(start);
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }
        <span class="kw">if </span>!<span class="self">self</span>.bump() {
            <span class="self">self</span>.parser().pos.set(start);
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }
        <span class="kw">if </span><span class="self">self</span>.char() == <span class="string">'^' </span>{
            negated = <span class="bool-val">true</span>;
            <span class="kw">if </span>!<span class="self">self</span>.bump() {
                <span class="self">self</span>.parser().pos.set(start);
                <span class="kw">return </span><span class="prelude-val">None</span>;
            }
        }
        <span class="kw">let </span>name_start = <span class="self">self</span>.offset();
        <span class="kw">while </span><span class="self">self</span>.char() != <span class="string">':' </span>&amp;&amp; <span class="self">self</span>.bump() {}
        <span class="kw">if </span><span class="self">self</span>.is_eof() {
            <span class="self">self</span>.parser().pos.set(start);
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }
        <span class="kw">let </span>name = <span class="kw-2">&amp;</span><span class="self">self</span>.pattern()[name_start..<span class="self">self</span>.offset()];
        <span class="kw">if </span>!<span class="self">self</span>.bump_if(<span class="string">":]"</span>) {
            <span class="self">self</span>.parser().pos.set(start);
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }
        <span class="kw">let </span>kind = <span class="kw">match </span>ast::ClassAsciiKind::from_name(name) {
            <span class="prelude-val">Some</span>(kind) =&gt; kind,
            <span class="prelude-val">None </span>=&gt; {
                <span class="self">self</span>.parser().pos.set(start);
                <span class="kw">return </span><span class="prelude-val">None</span>;
            }
        };
        <span class="prelude-val">Some</span>(ast::ClassAscii {
            span: Span::new(start, <span class="self">self</span>.pos()),
            kind,
            negated,
        })
    }

    <span class="doccomment">/// Parse a Unicode class in either the single character notation, `\pN`
    /// or the multi-character bracketed notation, `\p{Greek}`. This assumes
    /// the parser is positioned at the `p` (or `P` for negation) and will
    /// advance the parser to the character immediately following the class.
    ///
    /// Note that this does not check whether the class name is valid or not.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_unicode_class(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;ast::ClassUnicode&gt; {
        <span class="macro">assert!</span>(<span class="self">self</span>.char() == <span class="string">'p' </span>|| <span class="self">self</span>.char() == <span class="string">'P'</span>);

        <span class="kw">let </span><span class="kw-2">mut </span>scratch = <span class="self">self</span>.parser().scratch.borrow_mut();
        scratch.clear();

        <span class="kw">let </span>negated = <span class="self">self</span>.char() == <span class="string">'P'</span>;
        <span class="kw">if </span>!<span class="self">self</span>.bump_and_bump_space() {
            <span class="kw">return </span><span class="prelude-val">Err</span>(
                <span class="self">self</span>.error(<span class="self">self</span>.span(), ast::ErrorKind::EscapeUnexpectedEof)
            );
        }
        <span class="kw">let </span>(start, kind) = <span class="kw">if </span><span class="self">self</span>.char() == <span class="string">'{' </span>{
            <span class="kw">let </span>start = <span class="self">self</span>.span_char().end;
            <span class="kw">while </span><span class="self">self</span>.bump_and_bump_space() &amp;&amp; <span class="self">self</span>.char() != <span class="string">'}' </span>{
                scratch.push(<span class="self">self</span>.char());
            }
            <span class="kw">if </span><span class="self">self</span>.is_eof() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self
                    </span>.error(<span class="self">self</span>.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }
            <span class="macro">assert_eq!</span>(<span class="self">self</span>.char(), <span class="string">'}'</span>);
            <span class="self">self</span>.bump();

            <span class="kw">let </span>name = scratch.as_str();
            <span class="kw">if let </span><span class="prelude-val">Some</span>(i) = name.find(<span class="string">"!="</span>) {
                (
                    start,
                    ast::ClassUnicodeKind::NamedValue {
                        op: ast::ClassUnicodeOpKind::NotEqual,
                        name: name[..i].to_string(),
                        value: name[i + <span class="number">2</span>..].to_string(),
                    },
                )
            } <span class="kw">else if let </span><span class="prelude-val">Some</span>(i) = name.find(<span class="string">':'</span>) {
                (
                    start,
                    ast::ClassUnicodeKind::NamedValue {
                        op: ast::ClassUnicodeOpKind::Colon,
                        name: name[..i].to_string(),
                        value: name[i + <span class="number">1</span>..].to_string(),
                    },
                )
            } <span class="kw">else if let </span><span class="prelude-val">Some</span>(i) = name.find(<span class="string">'='</span>) {
                (
                    start,
                    ast::ClassUnicodeKind::NamedValue {
                        op: ast::ClassUnicodeOpKind::Equal,
                        name: name[..i].to_string(),
                        value: name[i + <span class="number">1</span>..].to_string(),
                    },
                )
            } <span class="kw">else </span>{
                (start, ast::ClassUnicodeKind::Named(name.to_string()))
            }
        } <span class="kw">else </span>{
            <span class="kw">let </span>start = <span class="self">self</span>.pos();
            <span class="kw">let </span>c = <span class="self">self</span>.char();
            <span class="kw">if </span>c == <span class="string">'\\' </span>{
                <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.error(
                    <span class="self">self</span>.span_char(),
                    ast::ErrorKind::UnicodeClassInvalid,
                ));
            }
            <span class="self">self</span>.bump_and_bump_space();
            <span class="kw">let </span>kind = ast::ClassUnicodeKind::OneLetter(c);
            (start, kind)
        };
        <span class="prelude-val">Ok</span>(ast::ClassUnicode {
            span: Span::new(start, <span class="self">self</span>.pos()),
            negated,
            kind,
        })
    }

    <span class="doccomment">/// Parse a Perl character class, e.g., `\d` or `\W`. This assumes the
    /// parser is currently at a valid character class name and will be
    /// advanced to the character immediately following the class.
    </span><span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>parse_perl_class(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; ast::ClassPerl {
        <span class="kw">let </span>c = <span class="self">self</span>.char();
        <span class="kw">let </span>span = <span class="self">self</span>.span_char();
        <span class="self">self</span>.bump();
        <span class="kw">let </span>(negated, kind) = <span class="kw">match </span>c {
            <span class="string">'d' </span>=&gt; (<span class="bool-val">false</span>, ast::ClassPerlKind::Digit),
            <span class="string">'D' </span>=&gt; (<span class="bool-val">true</span>, ast::ClassPerlKind::Digit),
            <span class="string">'s' </span>=&gt; (<span class="bool-val">false</span>, ast::ClassPerlKind::Space),
            <span class="string">'S' </span>=&gt; (<span class="bool-val">true</span>, ast::ClassPerlKind::Space),
            <span class="string">'w' </span>=&gt; (<span class="bool-val">false</span>, ast::ClassPerlKind::Word),
            <span class="string">'W' </span>=&gt; (<span class="bool-val">true</span>, ast::ClassPerlKind::Word),
            c =&gt; <span class="macro">panic!</span>(<span class="string">"expected valid Perl class but got '{}'"</span>, c),
        };
        ast::ClassPerl { span, kind, negated }
    }
}

<span class="doccomment">/// A type that traverses a fully parsed Ast and checks whether its depth
/// exceeds the specified nesting limit. If it does, then an error is returned.
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">struct </span>NestLimiter&lt;<span class="lifetime">'p</span>, <span class="lifetime">'s</span>, P&gt; {
    <span class="doccomment">/// The parser that is checking the nest limit.
    </span>p: <span class="kw-2">&amp;</span><span class="lifetime">'p </span>ParserI&lt;<span class="lifetime">'s</span>, P&gt;,
    <span class="doccomment">/// The current depth while walking an Ast.
    </span>depth: u32,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'p</span>, <span class="lifetime">'s</span>, P: Borrow&lt;Parser&gt;&gt; NestLimiter&lt;<span class="lifetime">'p</span>, <span class="lifetime">'s</span>, P&gt; {
    <span class="kw">fn </span>new(p: <span class="kw-2">&amp;</span><span class="lifetime">'p </span>ParserI&lt;<span class="lifetime">'s</span>, P&gt;) -&gt; NestLimiter&lt;<span class="lifetime">'p</span>, <span class="lifetime">'s</span>, P&gt; {
        NestLimiter { p, depth: <span class="number">0 </span>}
    }

    <span class="attr">#[inline(never)]
    </span><span class="kw">fn </span>check(<span class="self">self</span>, ast: <span class="kw-2">&amp;</span>Ast) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        ast::visit(ast, <span class="self">self</span>)
    }

    <span class="kw">fn </span>increment_depth(<span class="kw-2">&amp;mut </span><span class="self">self</span>, span: <span class="kw-2">&amp;</span>Span) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="kw">let </span>new = <span class="self">self</span>.depth.checked_add(<span class="number">1</span>).ok_or_else(|| {
            <span class="self">self</span>.p.error(
                span.clone(),
                ast::ErrorKind::NestLimitExceeded(u32::MAX),
            )
        })<span class="question-mark">?</span>;
        <span class="kw">let </span>limit = <span class="self">self</span>.p.parser().nest_limit;
        <span class="kw">if </span>new &gt; limit {
            <span class="kw">return </span><span class="prelude-val">Err</span>(<span class="self">self</span>.p.error(
                span.clone(),
                ast::ErrorKind::NestLimitExceeded(limit),
            ));
        }
        <span class="self">self</span>.depth = new;
        <span class="prelude-val">Ok</span>(())
    }

    <span class="kw">fn </span>decrement_depth(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="comment">// Assuming the correctness of the visitor, this should never drop
        // below 0.
        </span><span class="self">self</span>.depth = <span class="self">self</span>.depth.checked_sub(<span class="number">1</span>).unwrap();
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'p</span>, <span class="lifetime">'s</span>, P: Borrow&lt;Parser&gt;&gt; ast::Visitor <span class="kw">for </span>NestLimiter&lt;<span class="lifetime">'p</span>, <span class="lifetime">'s</span>, P&gt; {
    <span class="kw">type </span>Output = ();
    <span class="kw">type </span><span class="prelude-val">Err </span>= ast::Error;

    <span class="kw">fn </span>finish(<span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="prelude-val">Ok</span>(())
    }

    <span class="kw">fn </span>visit_pre(<span class="kw-2">&amp;mut </span><span class="self">self</span>, ast: <span class="kw-2">&amp;</span>Ast) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="kw">let </span>span = <span class="kw">match </span><span class="kw-2">*</span>ast {
            Ast::Empty(<span class="kw">_</span>)
            | Ast::Flags(<span class="kw">_</span>)
            | Ast::Literal(<span class="kw">_</span>)
            | Ast::Dot(<span class="kw">_</span>)
            | Ast::Assertion(<span class="kw">_</span>)
            | Ast::ClassUnicode(<span class="kw">_</span>)
            | Ast::ClassPerl(<span class="kw">_</span>) =&gt; {
                <span class="comment">// These are all base cases, so we don't increment depth.
                </span><span class="kw">return </span><span class="prelude-val">Ok</span>(());
            }
            Ast::ClassBracketed(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
            Ast::Repetition(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
            Ast::Group(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
            Ast::Alternation(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
            Ast::Concat(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
        };
        <span class="self">self</span>.increment_depth(span)
    }

    <span class="kw">fn </span>visit_post(<span class="kw-2">&amp;mut </span><span class="self">self</span>, ast: <span class="kw-2">&amp;</span>Ast) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="kw">match </span><span class="kw-2">*</span>ast {
            Ast::Empty(<span class="kw">_</span>)
            | Ast::Flags(<span class="kw">_</span>)
            | Ast::Literal(<span class="kw">_</span>)
            | Ast::Dot(<span class="kw">_</span>)
            | Ast::Assertion(<span class="kw">_</span>)
            | Ast::ClassUnicode(<span class="kw">_</span>)
            | Ast::ClassPerl(<span class="kw">_</span>) =&gt; {
                <span class="comment">// These are all base cases, so we don't decrement depth.
                </span><span class="prelude-val">Ok</span>(())
            }
            Ast::ClassBracketed(<span class="kw">_</span>)
            | Ast::Repetition(<span class="kw">_</span>)
            | Ast::Group(<span class="kw">_</span>)
            | Ast::Alternation(<span class="kw">_</span>)
            | Ast::Concat(<span class="kw">_</span>) =&gt; {
                <span class="self">self</span>.decrement_depth();
                <span class="prelude-val">Ok</span>(())
            }
        }
    }

    <span class="kw">fn </span>visit_class_set_item_pre(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        ast: <span class="kw-2">&amp;</span>ast::ClassSetItem,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="kw">let </span>span = <span class="kw">match </span><span class="kw-2">*</span>ast {
            ast::ClassSetItem::Empty(<span class="kw">_</span>)
            | ast::ClassSetItem::Literal(<span class="kw">_</span>)
            | ast::ClassSetItem::Range(<span class="kw">_</span>)
            | ast::ClassSetItem::Ascii(<span class="kw">_</span>)
            | ast::ClassSetItem::Unicode(<span class="kw">_</span>)
            | ast::ClassSetItem::Perl(<span class="kw">_</span>) =&gt; {
                <span class="comment">// These are all base cases, so we don't increment depth.
                </span><span class="kw">return </span><span class="prelude-val">Ok</span>(());
            }
            ast::ClassSetItem::Bracketed(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
            ast::ClassSetItem::Union(<span class="kw-2">ref </span>x) =&gt; <span class="kw-2">&amp;</span>x.span,
        };
        <span class="self">self</span>.increment_depth(span)
    }

    <span class="kw">fn </span>visit_class_set_item_post(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        ast: <span class="kw-2">&amp;</span>ast::ClassSetItem,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="kw">match </span><span class="kw-2">*</span>ast {
            ast::ClassSetItem::Empty(<span class="kw">_</span>)
            | ast::ClassSetItem::Literal(<span class="kw">_</span>)
            | ast::ClassSetItem::Range(<span class="kw">_</span>)
            | ast::ClassSetItem::Ascii(<span class="kw">_</span>)
            | ast::ClassSetItem::Unicode(<span class="kw">_</span>)
            | ast::ClassSetItem::Perl(<span class="kw">_</span>) =&gt; {
                <span class="comment">// These are all base cases, so we don't decrement depth.
                </span><span class="prelude-val">Ok</span>(())
            }
            ast::ClassSetItem::Bracketed(<span class="kw">_</span>) | ast::ClassSetItem::Union(<span class="kw">_</span>) =&gt; {
                <span class="self">self</span>.decrement_depth();
                <span class="prelude-val">Ok</span>(())
            }
        }
    }

    <span class="kw">fn </span>visit_class_set_binary_op_pre(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        ast: <span class="kw-2">&amp;</span>ast::ClassSetBinaryOp,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="self">self</span>.increment_depth(<span class="kw-2">&amp;</span>ast.span)
    }

    <span class="kw">fn </span>visit_class_set_binary_op_post(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        _ast: <span class="kw-2">&amp;</span>ast::ClassSetBinaryOp,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;()&gt; {
        <span class="self">self</span>.decrement_depth();
        <span class="prelude-val">Ok</span>(())
    }
}

<span class="doccomment">/// When the result is an error, transforms the ast::ErrorKind from the source
/// Result into another one. This function is used to return clearer error
/// messages when possible.
</span><span class="kw">fn </span>specialize_err&lt;T&gt;(
    result: <span class="prelude-ty">Result</span>&lt;T&gt;,
    from: ast::ErrorKind,
    to: ast::ErrorKind,
) -&gt; <span class="prelude-ty">Result</span>&lt;T&gt; {
    <span class="kw">if let </span><span class="prelude-val">Err</span>(e) = result {
        <span class="kw">if </span>e.kind == from {
            <span class="prelude-val">Err</span>(ast::Error { kind: to, pattern: e.pattern, span: e.span })
        } <span class="kw">else </span>{
            <span class="prelude-val">Err</span>(e)
        }
    } <span class="kw">else </span>{
        result
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span>core::ops::Range;

    <span class="kw">use </span>alloc::format;

    <span class="kw">use super</span>::<span class="kw-2">*</span>;

    <span class="comment">// Our own assert_eq, which has slightly better formatting (but honestly
    // still kind of crappy).
    </span><span class="macro">macro_rules!</span> assert_eq {
        (<span class="macro-nonterminal">$left</span>:expr, <span class="macro-nonterminal">$right</span>:expr) =&gt; {{
            <span class="kw">match </span>(<span class="kw-2">&amp;</span><span class="macro-nonterminal">$left</span>, <span class="kw-2">&amp;</span><span class="macro-nonterminal">$right</span>) {
                (left_val, right_val) =&gt; {
                    <span class="kw">if </span>!(<span class="kw-2">*</span>left_val == <span class="kw-2">*</span>right_val) {
                        <span class="macro">panic!</span>(
                            <span class="string">"assertion failed: `(left == right)`\n\n\
                             left:  `{:?}`\nright: `{:?}`\n\n"</span>,
                            left_val, right_val
                        )
                    }
                }
            }
        }};
    }

    <span class="comment">// We create these errors to compare with real ast::Errors in the tests.
    // We define equality between TestError and ast::Error to disregard the
    // pattern string in ast::Error, which is annoying to provide in tests.
    </span><span class="attr">#[derive(Clone, Debug)]
    </span><span class="kw">struct </span>TestError {
        span: Span,
        kind: ast::ErrorKind,
    }

    <span class="kw">impl </span>PartialEq&lt;ast::Error&gt; <span class="kw">for </span>TestError {
        <span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>ast::Error) -&gt; bool {
            <span class="self">self</span>.span == other.span &amp;&amp; <span class="self">self</span>.kind == other.kind
        }
    }

    <span class="kw">impl </span>PartialEq&lt;TestError&gt; <span class="kw">for </span>ast::Error {
        <span class="kw">fn </span>eq(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span>TestError) -&gt; bool {
            <span class="self">self</span>.span == other.span &amp;&amp; <span class="self">self</span>.kind == other.kind
        }
    }

    <span class="kw">fn </span>s(str: <span class="kw-2">&amp;</span>str) -&gt; String {
        str.to_string()
    }

    <span class="kw">fn </span>parser(pattern: <span class="kw-2">&amp;</span>str) -&gt; ParserI&lt;<span class="lifetime">'_</span>, Parser&gt; {
        ParserI::new(Parser::new(), pattern)
    }

    <span class="kw">fn </span>parser_octal(pattern: <span class="kw-2">&amp;</span>str) -&gt; ParserI&lt;<span class="lifetime">'_</span>, Parser&gt; {
        <span class="kw">let </span>parser = ParserBuilder::new().octal(<span class="bool-val">true</span>).build();
        ParserI::new(parser, pattern)
    }

    <span class="kw">fn </span>parser_empty_min_range(pattern: <span class="kw-2">&amp;</span>str) -&gt; ParserI&lt;<span class="lifetime">'_</span>, Parser&gt; {
        <span class="kw">let </span>parser = ParserBuilder::new().empty_min_range(<span class="bool-val">true</span>).build();
        ParserI::new(parser, pattern)
    }

    <span class="kw">fn </span>parser_nest_limit(
        pattern: <span class="kw-2">&amp;</span>str,
        nest_limit: u32,
    ) -&gt; ParserI&lt;<span class="lifetime">'_</span>, Parser&gt; {
        <span class="kw">let </span>p = ParserBuilder::new().nest_limit(nest_limit).build();
        ParserI::new(p, pattern)
    }

    <span class="kw">fn </span>parser_ignore_whitespace(pattern: <span class="kw-2">&amp;</span>str) -&gt; ParserI&lt;<span class="lifetime">'_</span>, Parser&gt; {
        <span class="kw">let </span>p = ParserBuilder::new().ignore_whitespace(<span class="bool-val">true</span>).build();
        ParserI::new(p, pattern)
    }

    <span class="doccomment">/// Short alias for creating a new span.
    </span><span class="kw">fn </span>nspan(start: Position, end: Position) -&gt; Span {
        Span::new(start, end)
    }

    <span class="doccomment">/// Short alias for creating a new position.
    </span><span class="kw">fn </span>npos(offset: usize, line: usize, column: usize) -&gt; Position {
        Position::new(offset, line, column)
    }

    <span class="doccomment">/// Create a new span from the given offset range. This assumes a single
    /// line and sets the columns based on the offsets. i.e., This only works
    /// out of the box for ASCII, which is fine for most tests.
    </span><span class="kw">fn </span>span(range: Range&lt;usize&gt;) -&gt; Span {
        <span class="kw">let </span>start = Position::new(range.start, <span class="number">1</span>, range.start + <span class="number">1</span>);
        <span class="kw">let </span>end = Position::new(range.end, <span class="number">1</span>, range.end + <span class="number">1</span>);
        Span::new(start, end)
    }

    <span class="doccomment">/// Create a new span for the corresponding byte range in the given string.
    </span><span class="kw">fn </span>span_range(subject: <span class="kw-2">&amp;</span>str, range: Range&lt;usize&gt;) -&gt; Span {
        <span class="kw">let </span>start = Position {
            offset: range.start,
            line: <span class="number">1 </span>+ subject[..range.start].matches(<span class="string">'\n'</span>).count(),
            column: <span class="number">1 </span>+ subject[..range.start]
                .chars()
                .rev()
                .position(|c| c == <span class="string">'\n'</span>)
                .unwrap_or(subject[..range.start].chars().count()),
        };
        <span class="kw">let </span>end = Position {
            offset: range.end,
            line: <span class="number">1 </span>+ subject[..range.end].matches(<span class="string">'\n'</span>).count(),
            column: <span class="number">1 </span>+ subject[..range.end]
                .chars()
                .rev()
                .position(|c| c == <span class="string">'\n'</span>)
                .unwrap_or(subject[..range.end].chars().count()),
        };
        Span::new(start, end)
    }

    <span class="doccomment">/// Create a verbatim literal starting at the given position.
    </span><span class="kw">fn </span>lit(c: char, start: usize) -&gt; Ast {
        lit_with(c, span(start..start + c.len_utf8()))
    }

    <span class="doccomment">/// Create a meta literal starting at the given position.
    </span><span class="kw">fn </span>meta_lit(c: char, span: Span) -&gt; Ast {
        Ast::literal(ast::Literal { span, kind: ast::LiteralKind::Meta, c })
    }

    <span class="doccomment">/// Create a verbatim literal with the given span.
    </span><span class="kw">fn </span>lit_with(c: char, span: Span) -&gt; Ast {
        Ast::literal(ast::Literal {
            span,
            kind: ast::LiteralKind::Verbatim,
            c,
        })
    }

    <span class="doccomment">/// Create a concatenation with the given range.
    </span><span class="kw">fn </span>concat(range: Range&lt;usize&gt;, asts: Vec&lt;Ast&gt;) -&gt; Ast {
        concat_with(span(range), asts)
    }

    <span class="doccomment">/// Create a concatenation with the given span.
    </span><span class="kw">fn </span>concat_with(span: Span, asts: Vec&lt;Ast&gt;) -&gt; Ast {
        Ast::concat(ast::Concat { span, asts })
    }

    <span class="doccomment">/// Create an alternation with the given span.
    </span><span class="kw">fn </span>alt(range: Range&lt;usize&gt;, asts: Vec&lt;Ast&gt;) -&gt; Ast {
        Ast::alternation(ast::Alternation { span: span(range), asts })
    }

    <span class="doccomment">/// Create a capturing group with the given span.
    </span><span class="kw">fn </span>group(range: Range&lt;usize&gt;, index: u32, ast: Ast) -&gt; Ast {
        Ast::group(ast::Group {
            span: span(range),
            kind: ast::GroupKind::CaptureIndex(index),
            ast: Box::new(ast),
        })
    }

    <span class="doccomment">/// Create an ast::SetFlags.
    ///
    /// The given pattern should be the full pattern string. The range given
    /// should correspond to the byte offsets where the flag set occurs.
    ///
    /// If negated is true, then the set is interpreted as beginning with a
    /// negation.
    </span><span class="kw">fn </span>flag_set(
        pat: <span class="kw-2">&amp;</span>str,
        range: Range&lt;usize&gt;,
        flag: ast::Flag,
        negated: bool,
    ) -&gt; Ast {
        <span class="kw">let </span><span class="kw-2">mut </span>items = <span class="macro">vec!</span>[ast::FlagsItem {
            span: span_range(pat, (range.end - <span class="number">2</span>)..(range.end - <span class="number">1</span>)),
            kind: ast::FlagsItemKind::Flag(flag),
        }];
        <span class="kw">if </span>negated {
            items.insert(
                <span class="number">0</span>,
                ast::FlagsItem {
                    span: span_range(pat, (range.start + <span class="number">2</span>)..(range.end - <span class="number">2</span>)),
                    kind: ast::FlagsItemKind::Negation,
                },
            );
        }
        Ast::flags(ast::SetFlags {
            span: span_range(pat, range.clone()),
            flags: ast::Flags {
                span: span_range(pat, (range.start + <span class="number">2</span>)..(range.end - <span class="number">1</span>)),
                items,
            },
        })
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_nest_limit() {
        <span class="comment">// A nest limit of 0 still allows some types of regexes.
        </span><span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">""</span>, <span class="number">0</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::empty(span(<span class="number">0</span>..<span class="number">0</span>)))
        );
        <span class="macro">assert_eq!</span>(parser_nest_limit(<span class="string">"a"</span>, <span class="number">0</span>).parse(), <span class="prelude-val">Ok</span>(lit(<span class="string">'a'</span>, <span class="number">0</span>)));

        <span class="comment">// Test repetition operations, which require one level of nesting.
        </span><span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"a+"</span>, <span class="number">0</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">0</span>),
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"a+"</span>, <span class="number">1</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">2</span>),
                    kind: ast::RepetitionKind::OneOrMore,
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"(a)+"</span>, <span class="number">1</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">1</span>),
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"a+*"</span>, <span class="number">1</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">1</span>),
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"a+*"</span>, <span class="number">2</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">2</span>..<span class="number">3</span>),
                    kind: ast::RepetitionKind::ZeroOrMore,
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(Ast::repetition(ast::Repetition {
                    span: span(<span class="number">0</span>..<span class="number">2</span>),
                    op: ast::RepetitionOp {
                        span: span(<span class="number">1</span>..<span class="number">2</span>),
                        kind: ast::RepetitionKind::OneOrMore,
                    },
                    greedy: <span class="bool-val">true</span>,
                    ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
                })),
            }))
        );

        <span class="comment">// Test concatenations. A concatenation requires one level of nesting.
        </span><span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"ab"</span>, <span class="number">0</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">0</span>),
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"ab"</span>, <span class="number">1</span>).parse(),
            <span class="prelude-val">Ok</span>(concat(<span class="number">0</span>..<span class="number">2</span>, <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">0</span>), lit(<span class="string">'b'</span>, <span class="number">1</span>)]))
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"abc"</span>, <span class="number">1</span>).parse(),
            <span class="prelude-val">Ok</span>(concat(<span class="number">0</span>..<span class="number">3</span>, <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">0</span>), lit(<span class="string">'b'</span>, <span class="number">1</span>), lit(<span class="string">'c'</span>, <span class="number">2</span>)]))
        );

        <span class="comment">// Test alternations. An alternation requires one level of nesting.
        </span><span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"a|b"</span>, <span class="number">0</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">0</span>),
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"a|b"</span>, <span class="number">1</span>).parse(),
            <span class="prelude-val">Ok</span>(alt(<span class="number">0</span>..<span class="number">3</span>, <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">0</span>), lit(<span class="string">'b'</span>, <span class="number">2</span>)]))
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"a|b|c"</span>, <span class="number">1</span>).parse(),
            <span class="prelude-val">Ok</span>(alt(<span class="number">0</span>..<span class="number">5</span>, <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">0</span>), lit(<span class="string">'b'</span>, <span class="number">2</span>), lit(<span class="string">'c'</span>, <span class="number">4</span>)]))
        );

        <span class="comment">// Test character classes. Classes form their own mini-recursive
        // syntax!
        </span><span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"[a]"</span>, <span class="number">0</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">0</span>),
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"[a]"</span>, <span class="number">1</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassSet::Item(ast::ClassSetItem::Literal(
                    ast::Literal {
                        span: span(<span class="number">1</span>..<span class="number">2</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'a'</span>,
                    }
                )),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"[ab]"</span>, <span class="number">1</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">1</span>),
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"[ab[cd]]"</span>, <span class="number">2</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">7</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">2</span>),
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"[ab[cd]]"</span>, <span class="number">3</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">3</span>),
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"[a--b]"</span>, <span class="number">1</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">1</span>),
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_nest_limit(<span class="string">"[a--bc]"</span>, <span class="number">2</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::NestLimitExceeded(<span class="number">2</span>),
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_comments() {
        <span class="kw">let </span>pat = <span class="string">"(?x)
# This is comment 1.
foo # This is comment 2.
  # This is comment 3.
bar
# This is comment 4."</span>;
        <span class="kw">let </span>astc = parser(pat).parse_with_comments().unwrap();
        <span class="macro">assert_eq!</span>(
            astc.ast,
            concat_with(
                span_range(pat, <span class="number">0</span>..pat.len()),
                <span class="macro">vec!</span>[
                    flag_set(pat, <span class="number">0</span>..<span class="number">4</span>, ast::Flag::IgnoreWhitespace, <span class="bool-val">false</span>),
                    lit_with(<span class="string">'f'</span>, span_range(pat, <span class="number">26</span>..<span class="number">27</span>)),
                    lit_with(<span class="string">'o'</span>, span_range(pat, <span class="number">27</span>..<span class="number">28</span>)),
                    lit_with(<span class="string">'o'</span>, span_range(pat, <span class="number">28</span>..<span class="number">29</span>)),
                    lit_with(<span class="string">'b'</span>, span_range(pat, <span class="number">74</span>..<span class="number">75</span>)),
                    lit_with(<span class="string">'a'</span>, span_range(pat, <span class="number">75</span>..<span class="number">76</span>)),
                    lit_with(<span class="string">'r'</span>, span_range(pat, <span class="number">76</span>..<span class="number">77</span>)),
                ]
            )
        );
        <span class="macro">assert_eq!</span>(
            astc.comments,
            <span class="macro">vec!</span>[
                ast::Comment {
                    span: span_range(pat, <span class="number">5</span>..<span class="number">26</span>),
                    comment: s(<span class="string">" This is comment 1."</span>),
                },
                ast::Comment {
                    span: span_range(pat, <span class="number">30</span>..<span class="number">51</span>),
                    comment: s(<span class="string">" This is comment 2."</span>),
                },
                ast::Comment {
                    span: span_range(pat, <span class="number">53</span>..<span class="number">74</span>),
                    comment: s(<span class="string">" This is comment 3."</span>),
                },
                ast::Comment {
                    span: span_range(pat, <span class="number">78</span>..<span class="number">98</span>),
                    comment: s(<span class="string">" This is comment 4."</span>),
                },
            ]
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_holistic() {
        <span class="macro">assert_eq!</span>(parser(<span class="string">"]"</span>).parse(), <span class="prelude-val">Ok</span>(lit(<span class="string">']'</span>, <span class="number">0</span>)));
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\\\.\+\*\?\(\)\|\[\]\{\}\^\$\#\&amp;\-\~"</span>).parse(),
            <span class="prelude-val">Ok</span>(concat(
                <span class="number">0</span>..<span class="number">36</span>,
                <span class="macro">vec!</span>[
                    meta_lit(<span class="string">'\\'</span>, span(<span class="number">0</span>..<span class="number">2</span>)),
                    meta_lit(<span class="string">'.'</span>, span(<span class="number">2</span>..<span class="number">4</span>)),
                    meta_lit(<span class="string">'+'</span>, span(<span class="number">4</span>..<span class="number">6</span>)),
                    meta_lit(<span class="string">'*'</span>, span(<span class="number">6</span>..<span class="number">8</span>)),
                    meta_lit(<span class="string">'?'</span>, span(<span class="number">8</span>..<span class="number">10</span>)),
                    meta_lit(<span class="string">'('</span>, span(<span class="number">10</span>..<span class="number">12</span>)),
                    meta_lit(<span class="string">')'</span>, span(<span class="number">12</span>..<span class="number">14</span>)),
                    meta_lit(<span class="string">'|'</span>, span(<span class="number">14</span>..<span class="number">16</span>)),
                    meta_lit(<span class="string">'['</span>, span(<span class="number">16</span>..<span class="number">18</span>)),
                    meta_lit(<span class="string">']'</span>, span(<span class="number">18</span>..<span class="number">20</span>)),
                    meta_lit(<span class="string">'{'</span>, span(<span class="number">20</span>..<span class="number">22</span>)),
                    meta_lit(<span class="string">'}'</span>, span(<span class="number">22</span>..<span class="number">24</span>)),
                    meta_lit(<span class="string">'^'</span>, span(<span class="number">24</span>..<span class="number">26</span>)),
                    meta_lit(<span class="string">'$'</span>, span(<span class="number">26</span>..<span class="number">28</span>)),
                    meta_lit(<span class="string">'#'</span>, span(<span class="number">28</span>..<span class="number">30</span>)),
                    meta_lit(<span class="string">'&amp;'</span>, span(<span class="number">30</span>..<span class="number">32</span>)),
                    meta_lit(<span class="string">'-'</span>, span(<span class="number">32</span>..<span class="number">34</span>)),
                    meta_lit(<span class="string">'~'</span>, span(<span class="number">34</span>..<span class="number">36</span>)),
                ]
            ))
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_ignore_whitespace() {
        <span class="comment">// Test that basic whitespace insensitivity works.
        </span><span class="kw">let </span>pat = <span class="string">"(?x)a b"</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(concat_with(
                nspan(npos(<span class="number">0</span>, <span class="number">1</span>, <span class="number">1</span>), npos(<span class="number">7</span>, <span class="number">1</span>, <span class="number">8</span>)),
                <span class="macro">vec!</span>[
                    flag_set(pat, <span class="number">0</span>..<span class="number">4</span>, ast::Flag::IgnoreWhitespace, <span class="bool-val">false</span>),
                    lit_with(<span class="string">'a'</span>, nspan(npos(<span class="number">4</span>, <span class="number">1</span>, <span class="number">5</span>), npos(<span class="number">5</span>, <span class="number">1</span>, <span class="number">6</span>))),
                    lit_with(<span class="string">'b'</span>, nspan(npos(<span class="number">6</span>, <span class="number">1</span>, <span class="number">7</span>), npos(<span class="number">7</span>, <span class="number">1</span>, <span class="number">8</span>))),
                ]
            ))
        );

        <span class="comment">// Test that we can toggle whitespace insensitivity.
        </span><span class="kw">let </span>pat = <span class="string">"(?x)a b(?-x)a b"</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(concat_with(
                nspan(npos(<span class="number">0</span>, <span class="number">1</span>, <span class="number">1</span>), npos(<span class="number">15</span>, <span class="number">1</span>, <span class="number">16</span>)),
                <span class="macro">vec!</span>[
                    flag_set(pat, <span class="number">0</span>..<span class="number">4</span>, ast::Flag::IgnoreWhitespace, <span class="bool-val">false</span>),
                    lit_with(<span class="string">'a'</span>, nspan(npos(<span class="number">4</span>, <span class="number">1</span>, <span class="number">5</span>), npos(<span class="number">5</span>, <span class="number">1</span>, <span class="number">6</span>))),
                    lit_with(<span class="string">'b'</span>, nspan(npos(<span class="number">6</span>, <span class="number">1</span>, <span class="number">7</span>), npos(<span class="number">7</span>, <span class="number">1</span>, <span class="number">8</span>))),
                    flag_set(pat, <span class="number">7</span>..<span class="number">12</span>, ast::Flag::IgnoreWhitespace, <span class="bool-val">true</span>),
                    lit_with(<span class="string">'a'</span>, nspan(npos(<span class="number">12</span>, <span class="number">1</span>, <span class="number">13</span>), npos(<span class="number">13</span>, <span class="number">1</span>, <span class="number">14</span>))),
                    lit_with(<span class="string">' '</span>, nspan(npos(<span class="number">13</span>, <span class="number">1</span>, <span class="number">14</span>), npos(<span class="number">14</span>, <span class="number">1</span>, <span class="number">15</span>))),
                    lit_with(<span class="string">'b'</span>, nspan(npos(<span class="number">14</span>, <span class="number">1</span>, <span class="number">15</span>), npos(<span class="number">15</span>, <span class="number">1</span>, <span class="number">16</span>))),
                ]
            ))
        );

        <span class="comment">// Test that nesting whitespace insensitive flags works.
        </span><span class="kw">let </span>pat = <span class="string">"a (?x:a )a "</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(concat_with(
                span_range(pat, <span class="number">0</span>..<span class="number">11</span>),
                <span class="macro">vec!</span>[
                    lit_with(<span class="string">'a'</span>, span_range(pat, <span class="number">0</span>..<span class="number">1</span>)),
                    lit_with(<span class="string">' '</span>, span_range(pat, <span class="number">1</span>..<span class="number">2</span>)),
                    Ast::group(ast::Group {
                        span: span_range(pat, <span class="number">2</span>..<span class="number">9</span>),
                        kind: ast::GroupKind::NonCapturing(ast::Flags {
                            span: span_range(pat, <span class="number">4</span>..<span class="number">5</span>),
                            items: <span class="macro">vec!</span>[ast::FlagsItem {
                                span: span_range(pat, <span class="number">4</span>..<span class="number">5</span>),
                                kind: ast::FlagsItemKind::Flag(
                                    ast::Flag::IgnoreWhitespace
                                ),
                            },],
                        }),
                        ast: Box::new(lit_with(<span class="string">'a'</span>, span_range(pat, <span class="number">6</span>..<span class="number">7</span>))),
                    }),
                    lit_with(<span class="string">'a'</span>, span_range(pat, <span class="number">9</span>..<span class="number">10</span>)),
                    lit_with(<span class="string">' '</span>, span_range(pat, <span class="number">10</span>..<span class="number">11</span>)),
                ]
            ))
        );

        <span class="comment">// Test that whitespace after an opening paren is insignificant.
        </span><span class="kw">let </span>pat = <span class="string">"(?x)( ?P&lt;foo&gt; a )"</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(concat_with(
                span_range(pat, <span class="number">0</span>..pat.len()),
                <span class="macro">vec!</span>[
                    flag_set(pat, <span class="number">0</span>..<span class="number">4</span>, ast::Flag::IgnoreWhitespace, <span class="bool-val">false</span>),
                    Ast::group(ast::Group {
                        span: span_range(pat, <span class="number">4</span>..pat.len()),
                        kind: ast::GroupKind::CaptureName {
                            starts_with_p: <span class="bool-val">true</span>,
                            name: ast::CaptureName {
                                span: span_range(pat, <span class="number">9</span>..<span class="number">12</span>),
                                name: s(<span class="string">"foo"</span>),
                                index: <span class="number">1</span>,
                            }
                        },
                        ast: Box::new(lit_with(<span class="string">'a'</span>, span_range(pat, <span class="number">14</span>..<span class="number">15</span>))),
                    }),
                ]
            ))
        );
        <span class="kw">let </span>pat = <span class="string">"(?x)(  a )"</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(concat_with(
                span_range(pat, <span class="number">0</span>..pat.len()),
                <span class="macro">vec!</span>[
                    flag_set(pat, <span class="number">0</span>..<span class="number">4</span>, ast::Flag::IgnoreWhitespace, <span class="bool-val">false</span>),
                    Ast::group(ast::Group {
                        span: span_range(pat, <span class="number">4</span>..pat.len()),
                        kind: ast::GroupKind::CaptureIndex(<span class="number">1</span>),
                        ast: Box::new(lit_with(<span class="string">'a'</span>, span_range(pat, <span class="number">7</span>..<span class="number">8</span>))),
                    }),
                ]
            ))
        );
        <span class="kw">let </span>pat = <span class="string">"(?x)(  ?:  a )"</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(concat_with(
                span_range(pat, <span class="number">0</span>..pat.len()),
                <span class="macro">vec!</span>[
                    flag_set(pat, <span class="number">0</span>..<span class="number">4</span>, ast::Flag::IgnoreWhitespace, <span class="bool-val">false</span>),
                    Ast::group(ast::Group {
                        span: span_range(pat, <span class="number">4</span>..pat.len()),
                        kind: ast::GroupKind::NonCapturing(ast::Flags {
                            span: span_range(pat, <span class="number">8</span>..<span class="number">8</span>),
                            items: <span class="macro">vec!</span>[],
                        }),
                        ast: Box::new(lit_with(<span class="string">'a'</span>, span_range(pat, <span class="number">11</span>..<span class="number">12</span>))),
                    }),
                ]
            ))
        );
        <span class="kw">let </span>pat = <span class="string">r"(?x)\x { 53 }"</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(concat_with(
                span_range(pat, <span class="number">0</span>..pat.len()),
                <span class="macro">vec!</span>[
                    flag_set(pat, <span class="number">0</span>..<span class="number">4</span>, ast::Flag::IgnoreWhitespace, <span class="bool-val">false</span>),
                    Ast::literal(ast::Literal {
                        span: span(<span class="number">4</span>..<span class="number">13</span>),
                        kind: ast::LiteralKind::HexBrace(
                            ast::HexLiteralKind::X
                        ),
                        c: <span class="string">'S'</span>,
                    }),
                ]
            ))
        );

        <span class="comment">// Test that whitespace after an escape is OK.
        </span><span class="kw">let </span>pat = <span class="string">r"(?x)\ "</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(concat_with(
                span_range(pat, <span class="number">0</span>..pat.len()),
                <span class="macro">vec!</span>[
                    flag_set(pat, <span class="number">0</span>..<span class="number">4</span>, ast::Flag::IgnoreWhitespace, <span class="bool-val">false</span>),
                    Ast::literal(ast::Literal {
                        span: span_range(pat, <span class="number">4</span>..<span class="number">6</span>),
                        kind: ast::LiteralKind::Superfluous,
                        c: <span class="string">' '</span>,
                    }),
                ]
            ))
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_newlines() {
        <span class="kw">let </span>pat = <span class="string">".\n."</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(concat_with(
                span_range(pat, <span class="number">0</span>..<span class="number">3</span>),
                <span class="macro">vec!</span>[
                    Ast::dot(span_range(pat, <span class="number">0</span>..<span class="number">1</span>)),
                    lit_with(<span class="string">'\n'</span>, span_range(pat, <span class="number">1</span>..<span class="number">2</span>)),
                    Ast::dot(span_range(pat, <span class="number">2</span>..<span class="number">3</span>)),
                ]
            ))
        );

        <span class="kw">let </span>pat = <span class="string">"foobar\nbaz\nquux\n"</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(concat_with(
                span_range(pat, <span class="number">0</span>..pat.len()),
                <span class="macro">vec!</span>[
                    lit_with(<span class="string">'f'</span>, nspan(npos(<span class="number">0</span>, <span class="number">1</span>, <span class="number">1</span>), npos(<span class="number">1</span>, <span class="number">1</span>, <span class="number">2</span>))),
                    lit_with(<span class="string">'o'</span>, nspan(npos(<span class="number">1</span>, <span class="number">1</span>, <span class="number">2</span>), npos(<span class="number">2</span>, <span class="number">1</span>, <span class="number">3</span>))),
                    lit_with(<span class="string">'o'</span>, nspan(npos(<span class="number">2</span>, <span class="number">1</span>, <span class="number">3</span>), npos(<span class="number">3</span>, <span class="number">1</span>, <span class="number">4</span>))),
                    lit_with(<span class="string">'b'</span>, nspan(npos(<span class="number">3</span>, <span class="number">1</span>, <span class="number">4</span>), npos(<span class="number">4</span>, <span class="number">1</span>, <span class="number">5</span>))),
                    lit_with(<span class="string">'a'</span>, nspan(npos(<span class="number">4</span>, <span class="number">1</span>, <span class="number">5</span>), npos(<span class="number">5</span>, <span class="number">1</span>, <span class="number">6</span>))),
                    lit_with(<span class="string">'r'</span>, nspan(npos(<span class="number">5</span>, <span class="number">1</span>, <span class="number">6</span>), npos(<span class="number">6</span>, <span class="number">1</span>, <span class="number">7</span>))),
                    lit_with(<span class="string">'\n'</span>, nspan(npos(<span class="number">6</span>, <span class="number">1</span>, <span class="number">7</span>), npos(<span class="number">7</span>, <span class="number">2</span>, <span class="number">1</span>))),
                    lit_with(<span class="string">'b'</span>, nspan(npos(<span class="number">7</span>, <span class="number">2</span>, <span class="number">1</span>), npos(<span class="number">8</span>, <span class="number">2</span>, <span class="number">2</span>))),
                    lit_with(<span class="string">'a'</span>, nspan(npos(<span class="number">8</span>, <span class="number">2</span>, <span class="number">2</span>), npos(<span class="number">9</span>, <span class="number">2</span>, <span class="number">3</span>))),
                    lit_with(<span class="string">'z'</span>, nspan(npos(<span class="number">9</span>, <span class="number">2</span>, <span class="number">3</span>), npos(<span class="number">10</span>, <span class="number">2</span>, <span class="number">4</span>))),
                    lit_with(<span class="string">'\n'</span>, nspan(npos(<span class="number">10</span>, <span class="number">2</span>, <span class="number">4</span>), npos(<span class="number">11</span>, <span class="number">3</span>, <span class="number">1</span>))),
                    lit_with(<span class="string">'q'</span>, nspan(npos(<span class="number">11</span>, <span class="number">3</span>, <span class="number">1</span>), npos(<span class="number">12</span>, <span class="number">3</span>, <span class="number">2</span>))),
                    lit_with(<span class="string">'u'</span>, nspan(npos(<span class="number">12</span>, <span class="number">3</span>, <span class="number">2</span>), npos(<span class="number">13</span>, <span class="number">3</span>, <span class="number">3</span>))),
                    lit_with(<span class="string">'u'</span>, nspan(npos(<span class="number">13</span>, <span class="number">3</span>, <span class="number">3</span>), npos(<span class="number">14</span>, <span class="number">3</span>, <span class="number">4</span>))),
                    lit_with(<span class="string">'x'</span>, nspan(npos(<span class="number">14</span>, <span class="number">3</span>, <span class="number">4</span>), npos(<span class="number">15</span>, <span class="number">3</span>, <span class="number">5</span>))),
                    lit_with(<span class="string">'\n'</span>, nspan(npos(<span class="number">15</span>, <span class="number">3</span>, <span class="number">5</span>), npos(<span class="number">16</span>, <span class="number">4</span>, <span class="number">1</span>))),
                ]
            ))
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_uncounted_repetition() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a*"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">2</span>),
                    kind: ast::RepetitionKind::ZeroOrMore,
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a+"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">2</span>),
                    kind: ast::RepetitionKind::OneOrMore,
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a?"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">2</span>),
                    kind: ast::RepetitionKind::ZeroOrOne,
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a??"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">3</span>),
                    kind: ast::RepetitionKind::ZeroOrOne,
                },
                greedy: <span class="bool-val">false</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a?"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">2</span>),
                    kind: ast::RepetitionKind::ZeroOrOne,
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a?b"</span>).parse(),
            <span class="prelude-val">Ok</span>(concat(
                <span class="number">0</span>..<span class="number">3</span>,
                <span class="macro">vec!</span>[
                    Ast::repetition(ast::Repetition {
                        span: span(<span class="number">0</span>..<span class="number">2</span>),
                        op: ast::RepetitionOp {
                            span: span(<span class="number">1</span>..<span class="number">2</span>),
                            kind: ast::RepetitionKind::ZeroOrOne,
                        },
                        greedy: <span class="bool-val">true</span>,
                        ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
                    }),
                    lit(<span class="string">'b'</span>, <span class="number">2</span>),
                ]
            ))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a??b"</span>).parse(),
            <span class="prelude-val">Ok</span>(concat(
                <span class="number">0</span>..<span class="number">4</span>,
                <span class="macro">vec!</span>[
                    Ast::repetition(ast::Repetition {
                        span: span(<span class="number">0</span>..<span class="number">3</span>),
                        op: ast::RepetitionOp {
                            span: span(<span class="number">1</span>..<span class="number">3</span>),
                            kind: ast::RepetitionKind::ZeroOrOne,
                        },
                        greedy: <span class="bool-val">false</span>,
                        ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
                    }),
                    lit(<span class="string">'b'</span>, <span class="number">3</span>),
                ]
            ))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"ab?"</span>).parse(),
            <span class="prelude-val">Ok</span>(concat(
                <span class="number">0</span>..<span class="number">3</span>,
                <span class="macro">vec!</span>[
                    lit(<span class="string">'a'</span>, <span class="number">0</span>),
                    Ast::repetition(ast::Repetition {
                        span: span(<span class="number">1</span>..<span class="number">3</span>),
                        op: ast::RepetitionOp {
                            span: span(<span class="number">2</span>..<span class="number">3</span>),
                            kind: ast::RepetitionKind::ZeroOrOne,
                        },
                        greedy: <span class="bool-val">true</span>,
                        ast: Box::new(lit(<span class="string">'b'</span>, <span class="number">1</span>)),
                    }),
                ]
            ))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(ab)?"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">4</span>..<span class="number">5</span>),
                    kind: ast::RepetitionKind::ZeroOrOne,
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(group(
                    <span class="number">0</span>..<span class="number">4</span>,
                    <span class="number">1</span>,
                    concat(<span class="number">1</span>..<span class="number">3</span>, <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">1</span>), lit(<span class="string">'b'</span>, <span class="number">2</span>),])
                )),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"|a?"</span>).parse(),
            <span class="prelude-val">Ok</span>(alt(
                <span class="number">0</span>..<span class="number">3</span>,
                <span class="macro">vec!</span>[
                    Ast::empty(span(<span class="number">0</span>..<span class="number">0</span>)),
                    Ast::repetition(ast::Repetition {
                        span: span(<span class="number">1</span>..<span class="number">3</span>),
                        op: ast::RepetitionOp {
                            span: span(<span class="number">2</span>..<span class="number">3</span>),
                            kind: ast::RepetitionKind::ZeroOrOne,
                        },
                        greedy: <span class="bool-val">true</span>,
                        ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">1</span>)),
                    }),
                ]
            ))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"*"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">0</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(?i)*"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(*)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(?:?)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"+"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">0</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"?"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">0</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(?)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"|*"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"|+"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"|?"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_counted_repetition() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{5}"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">4</span>),
                    kind: ast::RepetitionKind::Range(
                        ast::RepetitionRange::Exactly(<span class="number">5</span>)
                    ),
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{5,}"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">5</span>),
                    kind: ast::RepetitionKind::Range(
                        ast::RepetitionRange::AtLeast(<span class="number">5</span>)
                    ),
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{5,9}"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">6</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">6</span>),
                    kind: ast::RepetitionKind::Range(
                        ast::RepetitionRange::Bounded(<span class="number">5</span>, <span class="number">9</span>)
                    ),
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{5}?"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">5</span>),
                    kind: ast::RepetitionKind::Range(
                        ast::RepetitionRange::Exactly(<span class="number">5</span>)
                    ),
                },
                greedy: <span class="bool-val">false</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"ab{5}"</span>).parse(),
            <span class="prelude-val">Ok</span>(concat(
                <span class="number">0</span>..<span class="number">5</span>,
                <span class="macro">vec!</span>[
                    lit(<span class="string">'a'</span>, <span class="number">0</span>),
                    Ast::repetition(ast::Repetition {
                        span: span(<span class="number">1</span>..<span class="number">5</span>),
                        op: ast::RepetitionOp {
                            span: span(<span class="number">2</span>..<span class="number">5</span>),
                            kind: ast::RepetitionKind::Range(
                                ast::RepetitionRange::Exactly(<span class="number">5</span>)
                            ),
                        },
                        greedy: <span class="bool-val">true</span>,
                        ast: Box::new(lit(<span class="string">'b'</span>, <span class="number">1</span>)),
                    }),
                ]
            ))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"ab{5}c"</span>).parse(),
            <span class="prelude-val">Ok</span>(concat(
                <span class="number">0</span>..<span class="number">6</span>,
                <span class="macro">vec!</span>[
                    lit(<span class="string">'a'</span>, <span class="number">0</span>),
                    Ast::repetition(ast::Repetition {
                        span: span(<span class="number">1</span>..<span class="number">5</span>),
                        op: ast::RepetitionOp {
                            span: span(<span class="number">2</span>..<span class="number">5</span>),
                            kind: ast::RepetitionKind::Range(
                                ast::RepetitionRange::Exactly(<span class="number">5</span>)
                            ),
                        },
                        greedy: <span class="bool-val">true</span>,
                        ast: Box::new(lit(<span class="string">'b'</span>, <span class="number">1</span>)),
                    }),
                    lit(<span class="string">'c'</span>, <span class="number">5</span>),
                ]
            ))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{ 5 }"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">6</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">6</span>),
                    kind: ast::RepetitionKind::Range(
                        ast::RepetitionRange::Exactly(<span class="number">5</span>)
                    ),
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{ 5 , 9 }"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">10</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">10</span>),
                    kind: ast::RepetitionKind::Range(
                        ast::RepetitionRange::Bounded(<span class="number">5</span>, <span class="number">9</span>)
                    ),
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser_empty_min_range(<span class="string">r"a{,9}"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">5</span>),
                    kind: ast::RepetitionKind::Range(
                        ast::RepetitionRange::Bounded(<span class="number">0</span>, <span class="number">9</span>)
                    ),
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser_ignore_whitespace(<span class="string">r"a{5,9} ?"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">8</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">1</span>..<span class="number">8</span>),
                    kind: ast::RepetitionKind::Range(
                        ast::RepetitionRange::Bounded(<span class="number">5</span>, <span class="number">9</span>)
                    ),
                },
                greedy: <span class="bool-val">false</span>,
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">0</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b{5,9}"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::repetition(ast::Repetition {
                span: span(<span class="number">0</span>..<span class="number">7</span>),
                op: ast::RepetitionOp {
                    span: span(<span class="number">2</span>..<span class="number">7</span>),
                    kind: ast::RepetitionKind::Range(
                        ast::RepetitionRange::Bounded(<span class="number">5</span>, <span class="number">9</span>)
                    ),
                },
                greedy: <span class="bool-val">true</span>,
                ast: Box::new(Ast::assertion(ast::Assertion {
                    span: span(<span class="number">0</span>..<span class="number">2</span>),
                    kind: ast::AssertionKind::WordBoundary,
                })),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(?i){0}"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(?m){1,1}"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{]}"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::RepetitionCountDecimalEmpty,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{1,]}"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::RepetitionCountDecimalEmpty,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::RepetitionCountUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{}"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::RepetitionCountDecimalEmpty,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{a"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::RepetitionCountDecimalEmpty,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{9999999999}"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">12</span>),
                kind: ast::ErrorKind::DecimalInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{9"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::RepetitionCountUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{9,a"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::RepetitionCountDecimalEmpty,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{9,9999999999}"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">14</span>),
                kind: ast::ErrorKind::DecimalInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{9,"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::RepetitionCountUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{9,11"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::RepetitionCountUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a{2,1}"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::RepetitionCountInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"{5}"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">0</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"|{5}"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::RepetitionMissing,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_alternate() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a|b"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::alternation(ast::Alternation {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                asts: <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">0</span>), lit(<span class="string">'b'</span>, <span class="number">2</span>)],
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(a|b)"</span>).parse(),
            <span class="prelude-val">Ok</span>(group(
                <span class="number">0</span>..<span class="number">5</span>,
                <span class="number">1</span>,
                Ast::alternation(ast::Alternation {
                    span: span(<span class="number">1</span>..<span class="number">4</span>),
                    asts: <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">1</span>), lit(<span class="string">'b'</span>, <span class="number">3</span>)],
                })
            ))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a|b|c"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::alternation(ast::Alternation {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                asts: <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">0</span>), lit(<span class="string">'b'</span>, <span class="number">2</span>), lit(<span class="string">'c'</span>, <span class="number">4</span>)],
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"ax|by|cz"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::alternation(ast::Alternation {
                span: span(<span class="number">0</span>..<span class="number">8</span>),
                asts: <span class="macro">vec!</span>[
                    concat(<span class="number">0</span>..<span class="number">2</span>, <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">0</span>), lit(<span class="string">'x'</span>, <span class="number">1</span>)]),
                    concat(<span class="number">3</span>..<span class="number">5</span>, <span class="macro">vec!</span>[lit(<span class="string">'b'</span>, <span class="number">3</span>), lit(<span class="string">'y'</span>, <span class="number">4</span>)]),
                    concat(<span class="number">6</span>..<span class="number">8</span>, <span class="macro">vec!</span>[lit(<span class="string">'c'</span>, <span class="number">6</span>), lit(<span class="string">'z'</span>, <span class="number">7</span>)]),
                ],
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(ax|by|cz)"</span>).parse(),
            <span class="prelude-val">Ok</span>(group(
                <span class="number">0</span>..<span class="number">10</span>,
                <span class="number">1</span>,
                Ast::alternation(ast::Alternation {
                    span: span(<span class="number">1</span>..<span class="number">9</span>),
                    asts: <span class="macro">vec!</span>[
                        concat(<span class="number">1</span>..<span class="number">3</span>, <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">1</span>), lit(<span class="string">'x'</span>, <span class="number">2</span>)]),
                        concat(<span class="number">4</span>..<span class="number">6</span>, <span class="macro">vec!</span>[lit(<span class="string">'b'</span>, <span class="number">4</span>), lit(<span class="string">'y'</span>, <span class="number">5</span>)]),
                        concat(<span class="number">7</span>..<span class="number">9</span>, <span class="macro">vec!</span>[lit(<span class="string">'c'</span>, <span class="number">7</span>), lit(<span class="string">'z'</span>, <span class="number">8</span>)]),
                    ],
                })
            ))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(ax|(by|(cz)))"</span>).parse(),
            <span class="prelude-val">Ok</span>(group(
                <span class="number">0</span>..<span class="number">14</span>,
                <span class="number">1</span>,
                alt(
                    <span class="number">1</span>..<span class="number">13</span>,
                    <span class="macro">vec!</span>[
                        concat(<span class="number">1</span>..<span class="number">3</span>, <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">1</span>), lit(<span class="string">'x'</span>, <span class="number">2</span>)]),
                        group(
                            <span class="number">4</span>..<span class="number">13</span>,
                            <span class="number">2</span>,
                            alt(
                                <span class="number">5</span>..<span class="number">12</span>,
                                <span class="macro">vec!</span>[
                                    concat(
                                        <span class="number">5</span>..<span class="number">7</span>,
                                        <span class="macro">vec!</span>[lit(<span class="string">'b'</span>, <span class="number">5</span>), lit(<span class="string">'y'</span>, <span class="number">6</span>)]
                                    ),
                                    group(
                                        <span class="number">8</span>..<span class="number">12</span>,
                                        <span class="number">3</span>,
                                        concat(
                                            <span class="number">9</span>..<span class="number">11</span>,
                                            <span class="macro">vec!</span>[lit(<span class="string">'c'</span>, <span class="number">9</span>), lit(<span class="string">'z'</span>, <span class="number">10</span>),]
                                        )
                                    ),
                                ]
                            )
                        ),
                    ]
                )
            ))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"|"</span>).parse(),
            <span class="prelude-val">Ok</span>(alt(
                <span class="number">0</span>..<span class="number">1</span>,
                <span class="macro">vec!</span>[Ast::empty(span(<span class="number">0</span>..<span class="number">0</span>)), Ast::empty(span(<span class="number">1</span>..<span class="number">1</span>)),]
            ))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"||"</span>).parse(),
            <span class="prelude-val">Ok</span>(alt(
                <span class="number">0</span>..<span class="number">2</span>,
                <span class="macro">vec!</span>[
                    Ast::empty(span(<span class="number">0</span>..<span class="number">0</span>)),
                    Ast::empty(span(<span class="number">1</span>..<span class="number">1</span>)),
                    Ast::empty(span(<span class="number">2</span>..<span class="number">2</span>)),
                ]
            ))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a|"</span>).parse(),
            <span class="prelude-val">Ok</span>(alt(<span class="number">0</span>..<span class="number">2</span>, <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">0</span>), Ast::empty(span(<span class="number">2</span>..<span class="number">2</span>)),]))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"|a"</span>).parse(),
            <span class="prelude-val">Ok</span>(alt(<span class="number">0</span>..<span class="number">2</span>, <span class="macro">vec!</span>[Ast::empty(span(<span class="number">0</span>..<span class="number">0</span>)), lit(<span class="string">'a'</span>, <span class="number">1</span>),]))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(|)"</span>).parse(),
            <span class="prelude-val">Ok</span>(group(
                <span class="number">0</span>..<span class="number">3</span>,
                <span class="number">1</span>,
                alt(
                    <span class="number">1</span>..<span class="number">2</span>,
                    <span class="macro">vec!</span>[Ast::empty(span(<span class="number">1</span>..<span class="number">1</span>)), Ast::empty(span(<span class="number">2</span>..<span class="number">2</span>)),]
                )
            ))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(a|)"</span>).parse(),
            <span class="prelude-val">Ok</span>(group(
                <span class="number">0</span>..<span class="number">4</span>,
                <span class="number">1</span>,
                alt(<span class="number">1</span>..<span class="number">3</span>, <span class="macro">vec!</span>[lit(<span class="string">'a'</span>, <span class="number">1</span>), Ast::empty(span(<span class="number">3</span>..<span class="number">3</span>)),])
            ))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(|a)"</span>).parse(),
            <span class="prelude-val">Ok</span>(group(
                <span class="number">0</span>..<span class="number">4</span>,
                <span class="number">1</span>,
                alt(<span class="number">1</span>..<span class="number">3</span>, <span class="macro">vec!</span>[Ast::empty(span(<span class="number">1</span>..<span class="number">1</span>)), lit(<span class="string">'a'</span>, <span class="number">2</span>),])
            ))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a|b)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::GroupUnopened,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(a|b"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::GroupUnclosed,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_unsupported_lookaround() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(?=a)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::UnsupportedLookAround,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(?!a)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::UnsupportedLookAround,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(?&lt;=a)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::UnsupportedLookAround,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"(?&lt;!a)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::UnsupportedLookAround,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_group() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?i)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::flags(ast::SetFlags {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                flags: ast::Flags {
                    span: span(<span class="number">2</span>..<span class="number">3</span>),
                    items: <span class="macro">vec!</span>[ast::FlagsItem {
                        span: span(<span class="number">2</span>..<span class="number">3</span>),
                        kind: ast::FlagsItemKind::Flag(
                            ast::Flag::CaseInsensitive
                        ),
                    }],
                },
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?iU)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::flags(ast::SetFlags {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                flags: ast::Flags {
                    span: span(<span class="number">2</span>..<span class="number">4</span>),
                    items: <span class="macro">vec!</span>[
                        ast::FlagsItem {
                            span: span(<span class="number">2</span>..<span class="number">3</span>),
                            kind: ast::FlagsItemKind::Flag(
                                ast::Flag::CaseInsensitive
                            ),
                        },
                        ast::FlagsItem {
                            span: span(<span class="number">3</span>..<span class="number">4</span>),
                            kind: ast::FlagsItemKind::Flag(
                                ast::Flag::SwapGreed
                            ),
                        },
                    ],
                },
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?i-U)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::flags(ast::SetFlags {
                span: span(<span class="number">0</span>..<span class="number">6</span>),
                flags: ast::Flags {
                    span: span(<span class="number">2</span>..<span class="number">5</span>),
                    items: <span class="macro">vec!</span>[
                        ast::FlagsItem {
                            span: span(<span class="number">2</span>..<span class="number">3</span>),
                            kind: ast::FlagsItemKind::Flag(
                                ast::Flag::CaseInsensitive
                            ),
                        },
                        ast::FlagsItem {
                            span: span(<span class="number">3</span>..<span class="number">4</span>),
                            kind: ast::FlagsItemKind::Negation,
                        },
                        ast::FlagsItem {
                            span: span(<span class="number">4</span>..<span class="number">5</span>),
                            kind: ast::FlagsItemKind::Flag(
                                ast::Flag::SwapGreed
                            ),
                        },
                    ],
                },
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"()"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::GroupKind::CaptureIndex(<span class="number">1</span>),
                ast: Box::new(Ast::empty(span(<span class="number">1</span>..<span class="number">1</span>))),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(a)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                kind: ast::GroupKind::CaptureIndex(<span class="number">1</span>),
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">1</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(())"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                kind: ast::GroupKind::CaptureIndex(<span class="number">1</span>),
                ast: Box::new(Ast::group(ast::Group {
                    span: span(<span class="number">1</span>..<span class="number">3</span>),
                    kind: ast::GroupKind::CaptureIndex(<span class="number">2</span>),
                    ast: Box::new(Ast::empty(span(<span class="number">2</span>..<span class="number">2</span>))),
                })),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?:a)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                kind: ast::GroupKind::NonCapturing(ast::Flags {
                    span: span(<span class="number">2</span>..<span class="number">2</span>),
                    items: <span class="macro">vec!</span>[],
                }),
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">3</span>)),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?i:a)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">6</span>),
                kind: ast::GroupKind::NonCapturing(ast::Flags {
                    span: span(<span class="number">2</span>..<span class="number">3</span>),
                    items: <span class="macro">vec!</span>[ast::FlagsItem {
                        span: span(<span class="number">2</span>..<span class="number">3</span>),
                        kind: ast::FlagsItemKind::Flag(
                            ast::Flag::CaseInsensitive
                        ),
                    },],
                }),
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">4</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?i-U:a)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">8</span>),
                kind: ast::GroupKind::NonCapturing(ast::Flags {
                    span: span(<span class="number">2</span>..<span class="number">5</span>),
                    items: <span class="macro">vec!</span>[
                        ast::FlagsItem {
                            span: span(<span class="number">2</span>..<span class="number">3</span>),
                            kind: ast::FlagsItemKind::Flag(
                                ast::Flag::CaseInsensitive
                            ),
                        },
                        ast::FlagsItem {
                            span: span(<span class="number">3</span>..<span class="number">4</span>),
                            kind: ast::FlagsItemKind::Negation,
                        },
                        ast::FlagsItem {
                            span: span(<span class="number">4</span>..<span class="number">5</span>),
                            kind: ast::FlagsItemKind::Flag(
                                ast::Flag::SwapGreed
                            ),
                        },
                    ],
                }),
                ast: Box::new(lit(<span class="string">'a'</span>, <span class="number">6</span>)),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"("</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::GroupUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::GroupUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::FlagUnrecognized,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::GroupNameUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(a"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::GroupUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(()"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::GroupUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">")"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::GroupUnopened,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"a)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::GroupUnopened,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_capture_name() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?&lt;a&gt;z)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">7</span>),
                kind: ast::GroupKind::CaptureName {
                    starts_with_p: <span class="bool-val">false</span>,
                    name: ast::CaptureName {
                        span: span(<span class="number">3</span>..<span class="number">4</span>),
                        name: s(<span class="string">"a"</span>),
                        index: <span class="number">1</span>,
                    }
                },
                ast: Box::new(lit(<span class="string">'z'</span>, <span class="number">5</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;a&gt;z)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">8</span>),
                kind: ast::GroupKind::CaptureName {
                    starts_with_p: <span class="bool-val">true</span>,
                    name: ast::CaptureName {
                        span: span(<span class="number">4</span>..<span class="number">5</span>),
                        name: s(<span class="string">"a"</span>),
                        index: <span class="number">1</span>,
                    }
                },
                ast: Box::new(lit(<span class="string">'z'</span>, <span class="number">6</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;abc&gt;z)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">10</span>),
                kind: ast::GroupKind::CaptureName {
                    starts_with_p: <span class="bool-val">true</span>,
                    name: ast::CaptureName {
                        span: span(<span class="number">4</span>..<span class="number">7</span>),
                        name: s(<span class="string">"abc"</span>),
                        index: <span class="number">1</span>,
                    }
                },
                ast: Box::new(lit(<span class="string">'z'</span>, <span class="number">8</span>)),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;a_1&gt;z)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">10</span>),
                kind: ast::GroupKind::CaptureName {
                    starts_with_p: <span class="bool-val">true</span>,
                    name: ast::CaptureName {
                        span: span(<span class="number">4</span>..<span class="number">7</span>),
                        name: s(<span class="string">"a_1"</span>),
                        index: <span class="number">1</span>,
                    }
                },
                ast: Box::new(lit(<span class="string">'z'</span>, <span class="number">8</span>)),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;a.1&gt;z)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">10</span>),
                kind: ast::GroupKind::CaptureName {
                    starts_with_p: <span class="bool-val">true</span>,
                    name: ast::CaptureName {
                        span: span(<span class="number">4</span>..<span class="number">7</span>),
                        name: s(<span class="string">"a.1"</span>),
                        index: <span class="number">1</span>,
                    }
                },
                ast: Box::new(lit(<span class="string">'z'</span>, <span class="number">8</span>)),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;a[1]&gt;z)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: span(<span class="number">0</span>..<span class="number">11</span>),
                kind: ast::GroupKind::CaptureName {
                    starts_with_p: <span class="bool-val">true</span>,
                    name: ast::CaptureName {
                        span: span(<span class="number">4</span>..<span class="number">8</span>),
                        name: s(<span class="string">"a[1]"</span>),
                        index: <span class="number">1</span>,
                    }
                },
                ast: Box::new(lit(<span class="string">'z'</span>, <span class="number">9</span>)),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;a¾&gt;)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: Span::new(
                    Position::new(<span class="number">0</span>, <span class="number">1</span>, <span class="number">1</span>),
                    Position::new(<span class="number">9</span>, <span class="number">1</span>, <span class="number">9</span>),
                ),
                kind: ast::GroupKind::CaptureName {
                    starts_with_p: <span class="bool-val">true</span>,
                    name: ast::CaptureName {
                        span: Span::new(
                            Position::new(<span class="number">4</span>, <span class="number">1</span>, <span class="number">5</span>),
                            Position::new(<span class="number">7</span>, <span class="number">1</span>, <span class="number">7</span>),
                        ),
                        name: s(<span class="string">"a¾"</span>),
                        index: <span class="number">1</span>,
                    }
                },
                ast: Box::new(Ast::empty(Span::new(
                    Position::new(<span class="number">8</span>, <span class="number">1</span>, <span class="number">8</span>),
                    Position::new(<span class="number">8</span>, <span class="number">1</span>, <span class="number">8</span>),
                ))),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;名字&gt;)"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::group(ast::Group {
                span: Span::new(
                    Position::new(<span class="number">0</span>, <span class="number">1</span>, <span class="number">1</span>),
                    Position::new(<span class="number">12</span>, <span class="number">1</span>, <span class="number">9</span>),
                ),
                kind: ast::GroupKind::CaptureName {
                    starts_with_p: <span class="bool-val">true</span>,
                    name: ast::CaptureName {
                        span: Span::new(
                            Position::new(<span class="number">4</span>, <span class="number">1</span>, <span class="number">5</span>),
                            Position::new(<span class="number">10</span>, <span class="number">1</span>, <span class="number">7</span>),
                        ),
                        name: s(<span class="string">"名字"</span>),
                        index: <span class="number">1</span>,
                    }
                },
                ast: Box::new(Ast::empty(Span::new(
                    Position::new(<span class="number">11</span>, <span class="number">1</span>, <span class="number">8</span>),
                    Position::new(<span class="number">11</span>, <span class="number">1</span>, <span class="number">8</span>),
                ))),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::GroupNameUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;&gt;z)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::GroupNameEmpty,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;a"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">5</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::GroupNameUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;ab"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">6</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::GroupNameUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;0a"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::GroupNameInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;~"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::GroupNameInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;abc~"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">7</span>..<span class="number">8</span>),
                kind: ast::ErrorKind::GroupNameInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;a&gt;y)(?P&lt;a&gt;z)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">12</span>..<span class="number">13</span>),
                kind: ast::ErrorKind::GroupNameDuplicate {
                    original: span(<span class="number">4</span>..<span class="number">5</span>),
                },
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;5&gt;)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::GroupNameInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;5a&gt;)"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::GroupNameInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;¾&gt;)"</span>).parse().unwrap_err(),
            TestError {
                span: Span::new(
                    Position::new(<span class="number">4</span>, <span class="number">1</span>, <span class="number">5</span>),
                    Position::new(<span class="number">6</span>, <span class="number">1</span>, <span class="number">6</span>),
                ),
                kind: ast::ErrorKind::GroupNameInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;¾a&gt;)"</span>).parse().unwrap_err(),
            TestError {
                span: Span::new(
                    Position::new(<span class="number">4</span>, <span class="number">1</span>, <span class="number">5</span>),
                    Position::new(<span class="number">6</span>, <span class="number">1</span>, <span class="number">6</span>),
                ),
                kind: ast::ErrorKind::GroupNameInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;☃&gt;)"</span>).parse().unwrap_err(),
            TestError {
                span: Span::new(
                    Position::new(<span class="number">4</span>, <span class="number">1</span>, <span class="number">5</span>),
                    Position::new(<span class="number">7</span>, <span class="number">1</span>, <span class="number">6</span>),
                ),
                kind: ast::ErrorKind::GroupNameInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?P&lt;a☃&gt;)"</span>).parse().unwrap_err(),
            TestError {
                span: Span::new(
                    Position::new(<span class="number">5</span>, <span class="number">1</span>, <span class="number">6</span>),
                    Position::new(<span class="number">8</span>, <span class="number">1</span>, <span class="number">7</span>),
                ),
                kind: ast::ErrorKind::GroupNameInvalid,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_flags() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"i:"</span>).parse_flags(),
            <span class="prelude-val">Ok</span>(ast::Flags {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                items: <span class="macro">vec!</span>[ast::FlagsItem {
                    span: span(<span class="number">0</span>..<span class="number">1</span>),
                    kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                }],
            })
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"i)"</span>).parse_flags(),
            <span class="prelude-val">Ok</span>(ast::Flags {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                items: <span class="macro">vec!</span>[ast::FlagsItem {
                    span: span(<span class="number">0</span>..<span class="number">1</span>),
                    kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
                }],
            })
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"isU:"</span>).parse_flags(),
            <span class="prelude-val">Ok</span>(ast::Flags {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                items: <span class="macro">vec!</span>[
                    ast::FlagsItem {
                        span: span(<span class="number">0</span>..<span class="number">1</span>),
                        kind: ast::FlagsItemKind::Flag(
                            ast::Flag::CaseInsensitive
                        ),
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">1</span>..<span class="number">2</span>),
                        kind: ast::FlagsItemKind::Flag(
                            ast::Flag::DotMatchesNewLine
                        ),
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">2</span>..<span class="number">3</span>),
                        kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
                    },
                ],
            })
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"-isU:"</span>).parse_flags(),
            <span class="prelude-val">Ok</span>(ast::Flags {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                items: <span class="macro">vec!</span>[
                    ast::FlagsItem {
                        span: span(<span class="number">0</span>..<span class="number">1</span>),
                        kind: ast::FlagsItemKind::Negation,
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">1</span>..<span class="number">2</span>),
                        kind: ast::FlagsItemKind::Flag(
                            ast::Flag::CaseInsensitive
                        ),
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">2</span>..<span class="number">3</span>),
                        kind: ast::FlagsItemKind::Flag(
                            ast::Flag::DotMatchesNewLine
                        ),
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">3</span>..<span class="number">4</span>),
                        kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
                    },
                ],
            })
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"i-sU:"</span>).parse_flags(),
            <span class="prelude-val">Ok</span>(ast::Flags {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                items: <span class="macro">vec!</span>[
                    ast::FlagsItem {
                        span: span(<span class="number">0</span>..<span class="number">1</span>),
                        kind: ast::FlagsItemKind::Flag(
                            ast::Flag::CaseInsensitive
                        ),
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">1</span>..<span class="number">2</span>),
                        kind: ast::FlagsItemKind::Negation,
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">2</span>..<span class="number">3</span>),
                        kind: ast::FlagsItemKind::Flag(
                            ast::Flag::DotMatchesNewLine
                        ),
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">3</span>..<span class="number">4</span>),
                        kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
                    },
                ],
            })
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"i-sR:"</span>).parse_flags(),
            <span class="prelude-val">Ok</span>(ast::Flags {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                items: <span class="macro">vec!</span>[
                    ast::FlagsItem {
                        span: span(<span class="number">0</span>..<span class="number">1</span>),
                        kind: ast::FlagsItemKind::Flag(
                            ast::Flag::CaseInsensitive
                        ),
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">1</span>..<span class="number">2</span>),
                        kind: ast::FlagsItemKind::Negation,
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">2</span>..<span class="number">3</span>),
                        kind: ast::FlagsItemKind::Flag(
                            ast::Flag::DotMatchesNewLine
                        ),
                    },
                    ast::FlagsItem {
                        span: span(<span class="number">3</span>..<span class="number">4</span>),
                        kind: ast::FlagsItemKind::Flag(ast::Flag::CRLF),
                    },
                ],
            })
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"isU"</span>).parse_flags().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::FlagUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"isUa:"</span>).parse_flags().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::FlagUnrecognized,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"isUi:"</span>).parse_flags().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::FlagDuplicate { original: span(<span class="number">0</span>..<span class="number">1</span>) },
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"i-sU-i:"</span>).parse_flags().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::FlagRepeatedNegation {
                    original: span(<span class="number">1</span>..<span class="number">2</span>),
                },
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"-)"</span>).parse_flags().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::FlagDanglingNegation,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"i-)"</span>).parse_flags().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::FlagDanglingNegation,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"iU-)"</span>).parse_flags().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::FlagDanglingNegation,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_flag() {
        <span class="macro">assert_eq!</span>(parser(<span class="string">"i"</span>).parse_flag(), <span class="prelude-val">Ok</span>(ast::Flag::CaseInsensitive));
        <span class="macro">assert_eq!</span>(parser(<span class="string">"m"</span>).parse_flag(), <span class="prelude-val">Ok</span>(ast::Flag::MultiLine));
        <span class="macro">assert_eq!</span>(parser(<span class="string">"s"</span>).parse_flag(), <span class="prelude-val">Ok</span>(ast::Flag::DotMatchesNewLine));
        <span class="macro">assert_eq!</span>(parser(<span class="string">"U"</span>).parse_flag(), <span class="prelude-val">Ok</span>(ast::Flag::SwapGreed));
        <span class="macro">assert_eq!</span>(parser(<span class="string">"u"</span>).parse_flag(), <span class="prelude-val">Ok</span>(ast::Flag::Unicode));
        <span class="macro">assert_eq!</span>(parser(<span class="string">"R"</span>).parse_flag(), <span class="prelude-val">Ok</span>(ast::Flag::CRLF));
        <span class="macro">assert_eq!</span>(parser(<span class="string">"x"</span>).parse_flag(), <span class="prelude-val">Ok</span>(ast::Flag::IgnoreWhitespace));

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"a"</span>).parse_flag().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::FlagUnrecognized,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"☃"</span>).parse_flag().unwrap_err(),
            TestError {
                span: span_range(<span class="string">"☃"</span>, <span class="number">0</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::FlagUnrecognized,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_primitive_non_escape() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"."</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Dot(span(<span class="number">0</span>..<span class="number">1</span>)))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"^"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::AssertionKind::StartLine,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"$"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::AssertionKind::EndLine,
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"a"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::LiteralKind::Verbatim,
                c: <span class="string">'a'</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"|"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::LiteralKind::Verbatim,
                c: <span class="string">'|'</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"☃"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span_range(<span class="string">"☃"</span>, <span class="number">0</span>..<span class="number">3</span>),
                kind: ast::LiteralKind::Verbatim,
                c: <span class="string">'☃'</span>,
            }))
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_escape() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\|"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::LiteralKind::Meta,
                c: <span class="string">'|'</span>,
            }))
        );
        <span class="kw">let </span>specials = <span class="kw-2">&amp;</span>[
            (<span class="string">r"\a"</span>, <span class="string">'\x07'</span>, ast::SpecialLiteralKind::Bell),
            (<span class="string">r"\f"</span>, <span class="string">'\x0C'</span>, ast::SpecialLiteralKind::FormFeed),
            (<span class="string">r"\t"</span>, <span class="string">'\t'</span>, ast::SpecialLiteralKind::Tab),
            (<span class="string">r"\n"</span>, <span class="string">'\n'</span>, ast::SpecialLiteralKind::LineFeed),
            (<span class="string">r"\r"</span>, <span class="string">'\r'</span>, ast::SpecialLiteralKind::CarriageReturn),
            (<span class="string">r"\v"</span>, <span class="string">'\x0B'</span>, ast::SpecialLiteralKind::VerticalTab),
        ];
        <span class="kw">for </span><span class="kw-2">&amp;</span>(pat, c, <span class="kw-2">ref </span>kind) <span class="kw">in </span>specials {
            <span class="macro">assert_eq!</span>(
                parser(pat).parse_primitive(),
                <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                    span: span(<span class="number">0</span>..<span class="number">2</span>),
                    kind: ast::LiteralKind::Special(kind.clone()),
                    c,
                }))
            );
        }
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\A"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::AssertionKind::StartText,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\z"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::AssertionKind::EndText,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::AssertionKind::WordBoundary,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b{start}"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">9</span>),
                kind: ast::AssertionKind::WordBoundaryStart,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b{end}"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">7</span>),
                kind: ast::AssertionKind::WordBoundaryEnd,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b{start-half}"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">14</span>),
                kind: ast::AssertionKind::WordBoundaryStartHalf,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b{end-half}"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">12</span>),
                kind: ast::AssertionKind::WordBoundaryEndHalf,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\&lt;"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::AssertionKind::WordBoundaryStartAngle,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\&gt;"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::AssertionKind::WordBoundaryEndAngle,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\B"</span>).parse_primitive(),
            <span class="prelude-val">Ok</span>(Primitive::Assertion(ast::Assertion {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::AssertionKind::NotWordBoundary,
            }))
        );

        <span class="comment">// We also support superfluous escapes in most cases now too.
        </span><span class="kw">for </span>c <span class="kw">in </span>[<span class="string">'!'</span>, <span class="string">'@'</span>, <span class="string">'%'</span>, <span class="string">'"'</span>, <span class="string">'\''</span>, <span class="string">'/'</span>, <span class="string">' '</span>] {
            <span class="kw">let </span>pat = <span class="macro">format!</span>(<span class="string">r"\{}"</span>, c);
            <span class="macro">assert_eq!</span>(
                parser(<span class="kw-2">&amp;</span>pat).parse_primitive(),
                <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                    span: span(<span class="number">0</span>..<span class="number">2</span>),
                    kind: ast::LiteralKind::Superfluous,
                    c,
                }))
            );
        }

        <span class="comment">// Some superfluous escapes, namely [0-9A-Za-z], are still banned. This
        // gives flexibility for future evolution.
        </span><span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\e"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::EscapeUnrecognized,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\y"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::EscapeUnrecognized,
            }
        );

        <span class="comment">// Starting a special word boundary without any non-whitespace chars
        // after the brace makes it ambiguous whether the user meant to write
        // a counted repetition (probably not?) or an actual special word
        // boundary assertion.
        </span><span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b{"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::SpecialWordOrRepetitionUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_ignore_whitespace(<span class="string">r"\b{ "</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::SpecialWordOrRepetitionUnexpectedEof,
            }
        );
        <span class="comment">// When 'x' is not enabled, the space is seen as a non-[-A-Za-z] char,
        // and thus causes the parser to treat it as a counted repetition.
        </span><span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b{ "</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::RepetitionCountUnclosed,
            }
        );
        <span class="comment">// In this case, we got some valid chars that makes it look like the
        // user is writing one of the special word boundary assertions, but
        // we forget to close the brace.
        </span><span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b{foo"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::SpecialWordBoundaryUnclosed,
            }
        );
        <span class="comment">// We get the same error as above, except it is provoked by seeing a
        // char that we know is invalid before seeing a closing brace.
        </span><span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b{foo!}"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::SpecialWordBoundaryUnclosed,
            }
        );
        <span class="comment">// And this one occurs when, syntactically, everything looks okay, but
        // we don't use a valid spelling of a word boundary assertion.
        </span><span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\b{foo}"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::SpecialWordBoundaryUnrecognized,
            }
        );

        <span class="comment">// An unfinished escape is illegal.
        </span><span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_unsupported_backreference() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\0"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::UnsupportedBackreference,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\9"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::UnsupportedBackreference,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_octal() {
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">511 </span>{
            <span class="kw">let </span>pat = <span class="macro">format!</span>(<span class="string">r"\{:o}"</span>, i);
            <span class="macro">assert_eq!</span>(
                parser_octal(<span class="kw-2">&amp;</span>pat).parse_escape(),
                <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                    span: span(<span class="number">0</span>..pat.len()),
                    kind: ast::LiteralKind::Octal,
                    c: char::from_u32(i).unwrap(),
                }))
            );
        }
        <span class="macro">assert_eq!</span>(
            parser_octal(<span class="string">r"\778"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                kind: ast::LiteralKind::Octal,
                c: <span class="string">'?'</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser_octal(<span class="string">r"\7777"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                kind: ast::LiteralKind::Octal,
                c: <span class="string">'\u{01FF}'</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser_octal(<span class="string">r"\778"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::concat(ast::Concat {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                asts: <span class="macro">vec!</span>[
                    Ast::literal(ast::Literal {
                        span: span(<span class="number">0</span>..<span class="number">3</span>),
                        kind: ast::LiteralKind::Octal,
                        c: <span class="string">'?'</span>,
                    }),
                    Ast::literal(ast::Literal {
                        span: span(<span class="number">3</span>..<span class="number">4</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'8'</span>,
                    }),
                ],
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser_octal(<span class="string">r"\7777"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::concat(ast::Concat {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                asts: <span class="macro">vec!</span>[
                    Ast::literal(ast::Literal {
                        span: span(<span class="number">0</span>..<span class="number">4</span>),
                        kind: ast::LiteralKind::Octal,
                        c: <span class="string">'\u{01FF}'</span>,
                    }),
                    Ast::literal(ast::Literal {
                        span: span(<span class="number">4</span>..<span class="number">5</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'7'</span>,
                    }),
                ],
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser_octal(<span class="string">r"\8"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::EscapeUnrecognized,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_hex_two() {
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">256 </span>{
            <span class="kw">let </span>pat = <span class="macro">format!</span>(<span class="string">r"\x{:02x}"</span>, i);
            <span class="macro">assert_eq!</span>(
                parser(<span class="kw-2">&amp;</span>pat).parse_escape(),
                <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                    span: span(<span class="number">0</span>..pat.len()),
                    kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::X),
                    c: char::from_u32(i).unwrap(),
                }))
            );
        }

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\xF"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\xG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\xFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_hex_four() {
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">65536 </span>{
            <span class="kw">let </span>c = <span class="kw">match </span>char::from_u32(i) {
                <span class="prelude-val">None </span>=&gt; <span class="kw">continue</span>,
                <span class="prelude-val">Some</span>(c) =&gt; c,
            };
            <span class="kw">let </span>pat = <span class="macro">format!</span>(<span class="string">r"\u{:04x}"</span>, i);
            <span class="macro">assert_eq!</span>(
                parser(<span class="kw-2">&amp;</span>pat).parse_escape(),
                <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                    span: span(<span class="number">0</span>..pat.len()),
                    kind: ast::LiteralKind::HexFixed(
                        ast::HexLiteralKind::UnicodeShort
                    ),
                    c,
                }))
            );
        }

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\uF"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\uG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\uFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\uFFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\uFFFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">5</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\uD800"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::EscapeHexInvalid,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_hex_eight() {
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">65536 </span>{
            <span class="kw">let </span>c = <span class="kw">match </span>char::from_u32(i) {
                <span class="prelude-val">None </span>=&gt; <span class="kw">continue</span>,
                <span class="prelude-val">Some</span>(c) =&gt; c,
            };
            <span class="kw">let </span>pat = <span class="macro">format!</span>(<span class="string">r"\U{:08x}"</span>, i);
            <span class="macro">assert_eq!</span>(
                parser(<span class="kw-2">&amp;</span>pat).parse_escape(),
                <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                    span: span(<span class="number">0</span>..pat.len()),
                    kind: ast::LiteralKind::HexFixed(
                        ast::HexLiteralKind::UnicodeLong
                    ),
                    c,
                }))
            );
        }

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\UF"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\UG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\UFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\UFFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\UFFFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">5</span>..<span class="number">6</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\UFFFFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">6</span>..<span class="number">7</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\UFFFFFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">7</span>..<span class="number">8</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\UFFFFFFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">8</span>..<span class="number">9</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\UFFFFFFFG"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">9</span>..<span class="number">10</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_hex_brace() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\u{26c4}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span(<span class="number">0</span>..<span class="number">8</span>),
                kind: ast::LiteralKind::HexBrace(
                    ast::HexLiteralKind::UnicodeShort
                ),
                c: <span class="string">'⛄'</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\U{26c4}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span(<span class="number">0</span>..<span class="number">8</span>),
                kind: ast::LiteralKind::HexBrace(
                    ast::HexLiteralKind::UnicodeLong
                ),
                c: <span class="string">'⛄'</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x{26c4}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span(<span class="number">0</span>..<span class="number">8</span>),
                kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
                c: <span class="string">'⛄'</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x{26C4}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span(<span class="number">0</span>..<span class="number">8</span>),
                kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
                c: <span class="string">'⛄'</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x{10fFfF}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Literal(ast::Literal {
                span: span(<span class="number">0</span>..<span class="number">10</span>),
                kind: ast::LiteralKind::HexBrace(ast::HexLiteralKind::X),
                c: <span class="string">'\u{10FFFF}'</span>,
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x{"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x{FF"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x{}"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::EscapeHexEmpty,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x{FGF}"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::EscapeHexInvalidDigit,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x{FFFFFF}"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">9</span>),
                kind: ast::ErrorKind::EscapeHexInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x{D800}"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">7</span>),
                kind: ast::ErrorKind::EscapeHexInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\x{FFFFFFFFF}"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">12</span>),
                kind: ast::ErrorKind::EscapeHexInvalid,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_decimal() {
        <span class="macro">assert_eq!</span>(parser(<span class="string">"123"</span>).parse_decimal(), <span class="prelude-val">Ok</span>(<span class="number">123</span>));
        <span class="macro">assert_eq!</span>(parser(<span class="string">"0"</span>).parse_decimal(), <span class="prelude-val">Ok</span>(<span class="number">0</span>));
        <span class="macro">assert_eq!</span>(parser(<span class="string">"01"</span>).parse_decimal(), <span class="prelude-val">Ok</span>(<span class="number">1</span>));

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"-1"</span>).parse_decimal().unwrap_err(),
            TestError { span: span(<span class="number">0</span>..<span class="number">0</span>), kind: ast::ErrorKind::DecimalEmpty }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">""</span>).parse_decimal().unwrap_err(),
            TestError { span: span(<span class="number">0</span>..<span class="number">0</span>), kind: ast::ErrorKind::DecimalEmpty }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"9999999999"</span>).parse_decimal().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">10</span>),
                kind: ast::ErrorKind::DecimalInvalid,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_set_class() {
        <span class="kw">fn </span>union(span: Span, items: Vec&lt;ast::ClassSetItem&gt;) -&gt; ast::ClassSet {
            ast::ClassSet::union(ast::ClassSetUnion { span, items })
        }

        <span class="kw">fn </span>intersection(
            span: Span,
            lhs: ast::ClassSet,
            rhs: ast::ClassSet,
        ) -&gt; ast::ClassSet {
            ast::ClassSet::BinaryOp(ast::ClassSetBinaryOp {
                span,
                kind: ast::ClassSetBinaryOpKind::Intersection,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
        }

        <span class="kw">fn </span>difference(
            span: Span,
            lhs: ast::ClassSet,
            rhs: ast::ClassSet,
        ) -&gt; ast::ClassSet {
            ast::ClassSet::BinaryOp(ast::ClassSetBinaryOp {
                span,
                kind: ast::ClassSetBinaryOpKind::Difference,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
        }

        <span class="kw">fn </span>symdifference(
            span: Span,
            lhs: ast::ClassSet,
            rhs: ast::ClassSet,
        ) -&gt; ast::ClassSet {
            ast::ClassSet::BinaryOp(ast::ClassSetBinaryOp {
                span,
                kind: ast::ClassSetBinaryOpKind::SymmetricDifference,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
        }

        <span class="kw">fn </span>itemset(item: ast::ClassSetItem) -&gt; ast::ClassSet {
            ast::ClassSet::Item(item)
        }

        <span class="kw">fn </span>item_ascii(cls: ast::ClassAscii) -&gt; ast::ClassSetItem {
            ast::ClassSetItem::Ascii(cls)
        }

        <span class="kw">fn </span>item_unicode(cls: ast::ClassUnicode) -&gt; ast::ClassSetItem {
            ast::ClassSetItem::Unicode(cls)
        }

        <span class="kw">fn </span>item_perl(cls: ast::ClassPerl) -&gt; ast::ClassSetItem {
            ast::ClassSetItem::Perl(cls)
        }

        <span class="kw">fn </span>item_bracket(cls: ast::ClassBracketed) -&gt; ast::ClassSetItem {
            ast::ClassSetItem::Bracketed(Box::new(cls))
        }

        <span class="kw">fn </span>lit(span: Span, c: char) -&gt; ast::ClassSetItem {
            ast::ClassSetItem::Literal(ast::Literal {
                span,
                kind: ast::LiteralKind::Verbatim,
                c,
            })
        }

        <span class="kw">fn </span>empty(span: Span) -&gt; ast::ClassSetItem {
            ast::ClassSetItem::Empty(span)
        }

        <span class="kw">fn </span>range(span: Span, start: char, end: char) -&gt; ast::ClassSetItem {
            <span class="kw">let </span>pos1 = Position {
                offset: span.start.offset + start.len_utf8(),
                column: span.start.column + <span class="number">1</span>,
                ..span.start
            };
            <span class="kw">let </span>pos2 = Position {
                offset: span.end.offset - end.len_utf8(),
                column: span.end.column - <span class="number">1</span>,
                ..span.end
            };
            ast::ClassSetItem::Range(ast::ClassSetRange {
                span,
                start: ast::Literal {
                    span: Span { end: pos1, ..span },
                    kind: ast::LiteralKind::Verbatim,
                    c: start,
                },
                end: ast::Literal {
                    span: Span { start: pos2, ..span },
                    kind: ast::LiteralKind::Verbatim,
                    c: end,
                },
            })
        }

        <span class="kw">fn </span>alnum(span: Span, negated: bool) -&gt; ast::ClassAscii {
            ast::ClassAscii { span, kind: ast::ClassAsciiKind::Alnum, negated }
        }

        <span class="kw">fn </span>lower(span: Span, negated: bool) -&gt; ast::ClassAscii {
            ast::ClassAscii { span, kind: ast::ClassAsciiKind::Lower, negated }
        }

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[[:alnum:]]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">11</span>),
                negated: <span class="bool-val">false</span>,
                kind: itemset(item_ascii(alnum(span(<span class="number">1</span>..<span class="number">10</span>), <span class="bool-val">false</span>))),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[[[:alnum:]]]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">13</span>),
                negated: <span class="bool-val">false</span>,
                kind: itemset(item_bracket(ast::ClassBracketed {
                    span: span(<span class="number">1</span>..<span class="number">12</span>),
                    negated: <span class="bool-val">false</span>,
                    kind: itemset(item_ascii(alnum(span(<span class="number">2</span>..<span class="number">11</span>), <span class="bool-val">false</span>))),
                })),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[[:alnum:]&amp;&amp;[:lower:]]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">22</span>),
                negated: <span class="bool-val">false</span>,
                kind: intersection(
                    span(<span class="number">1</span>..<span class="number">21</span>),
                    itemset(item_ascii(alnum(span(<span class="number">1</span>..<span class="number">10</span>), <span class="bool-val">false</span>))),
                    itemset(item_ascii(lower(span(<span class="number">12</span>..<span class="number">21</span>), <span class="bool-val">false</span>))),
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[[:alnum:]--[:lower:]]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">22</span>),
                negated: <span class="bool-val">false</span>,
                kind: difference(
                    span(<span class="number">1</span>..<span class="number">21</span>),
                    itemset(item_ascii(alnum(span(<span class="number">1</span>..<span class="number">10</span>), <span class="bool-val">false</span>))),
                    itemset(item_ascii(lower(span(<span class="number">12</span>..<span class="number">21</span>), <span class="bool-val">false</span>))),
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[[:alnum:]~~[:lower:]]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">22</span>),
                negated: <span class="bool-val">false</span>,
                kind: symdifference(
                    span(<span class="number">1</span>..<span class="number">21</span>),
                    itemset(item_ascii(alnum(span(<span class="number">1</span>..<span class="number">10</span>), <span class="bool-val">false</span>))),
                    itemset(item_ascii(lower(span(<span class="number">12</span>..<span class="number">21</span>), <span class="bool-val">false</span>))),
                ),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[a]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                negated: <span class="bool-val">false</span>,
                kind: itemset(lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">'a'</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[a\]]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                negated: <span class="bool-val">false</span>,
                kind: union(
                    span(<span class="number">1</span>..<span class="number">4</span>),
                    <span class="macro">vec!</span>[
                        lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">'a'</span>),
                        ast::ClassSetItem::Literal(ast::Literal {
                            span: span(<span class="number">2</span>..<span class="number">4</span>),
                            kind: ast::LiteralKind::Meta,
                            c: <span class="string">']'</span>,
                        }),
                    ]
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[a\-z]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">6</span>),
                negated: <span class="bool-val">false</span>,
                kind: union(
                    span(<span class="number">1</span>..<span class="number">5</span>),
                    <span class="macro">vec!</span>[
                        lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">'a'</span>),
                        ast::ClassSetItem::Literal(ast::Literal {
                            span: span(<span class="number">2</span>..<span class="number">4</span>),
                            kind: ast::LiteralKind::Meta,
                            c: <span class="string">'-'</span>,
                        }),
                        lit(span(<span class="number">4</span>..<span class="number">5</span>), <span class="string">'z'</span>),
                    ]
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[ab]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                negated: <span class="bool-val">false</span>,
                kind: union(
                    span(<span class="number">1</span>..<span class="number">3</span>),
                    <span class="macro">vec!</span>[lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">'a'</span>), lit(span(<span class="number">2</span>..<span class="number">3</span>), <span class="string">'b'</span>),]
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[a-]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                negated: <span class="bool-val">false</span>,
                kind: union(
                    span(<span class="number">1</span>..<span class="number">3</span>),
                    <span class="macro">vec!</span>[lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">'a'</span>), lit(span(<span class="number">2</span>..<span class="number">3</span>), <span class="string">'-'</span>),]
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[-a]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                negated: <span class="bool-val">false</span>,
                kind: union(
                    span(<span class="number">1</span>..<span class="number">3</span>),
                    <span class="macro">vec!</span>[lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">'-'</span>), lit(span(<span class="number">2</span>..<span class="number">3</span>), <span class="string">'a'</span>),]
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[\pL]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                negated: <span class="bool-val">false</span>,
                kind: itemset(item_unicode(ast::ClassUnicode {
                    span: span(<span class="number">1</span>..<span class="number">4</span>),
                    negated: <span class="bool-val">false</span>,
                    kind: ast::ClassUnicodeKind::OneLetter(<span class="string">'L'</span>),
                })),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[\w]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                negated: <span class="bool-val">false</span>,
                kind: itemset(item_perl(ast::ClassPerl {
                    span: span(<span class="number">1</span>..<span class="number">3</span>),
                    kind: ast::ClassPerlKind::Word,
                    negated: <span class="bool-val">false</span>,
                })),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[a\wz]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">6</span>),
                negated: <span class="bool-val">false</span>,
                kind: union(
                    span(<span class="number">1</span>..<span class="number">5</span>),
                    <span class="macro">vec!</span>[
                        lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">'a'</span>),
                        item_perl(ast::ClassPerl {
                            span: span(<span class="number">2</span>..<span class="number">4</span>),
                            kind: ast::ClassPerlKind::Word,
                            negated: <span class="bool-val">false</span>,
                        }),
                        lit(span(<span class="number">4</span>..<span class="number">5</span>), <span class="string">'z'</span>),
                    ]
                ),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[a-z]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                negated: <span class="bool-val">false</span>,
                kind: itemset(range(span(<span class="number">1</span>..<span class="number">4</span>), <span class="string">'a'</span>, <span class="string">'z'</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[a-cx-z]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">8</span>),
                negated: <span class="bool-val">false</span>,
                kind: union(
                    span(<span class="number">1</span>..<span class="number">7</span>),
                    <span class="macro">vec!</span>[
                        range(span(<span class="number">1</span>..<span class="number">4</span>), <span class="string">'a'</span>, <span class="string">'c'</span>),
                        range(span(<span class="number">4</span>..<span class="number">7</span>), <span class="string">'x'</span>, <span class="string">'z'</span>),
                    ]
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[\w&amp;&amp;a-cx-z]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">12</span>),
                negated: <span class="bool-val">false</span>,
                kind: intersection(
                    span(<span class="number">1</span>..<span class="number">11</span>),
                    itemset(item_perl(ast::ClassPerl {
                        span: span(<span class="number">1</span>..<span class="number">3</span>),
                        kind: ast::ClassPerlKind::Word,
                        negated: <span class="bool-val">false</span>,
                    })),
                    union(
                        span(<span class="number">5</span>..<span class="number">11</span>),
                        <span class="macro">vec!</span>[
                            range(span(<span class="number">5</span>..<span class="number">8</span>), <span class="string">'a'</span>, <span class="string">'c'</span>),
                            range(span(<span class="number">8</span>..<span class="number">11</span>), <span class="string">'x'</span>, <span class="string">'z'</span>),
                        ]
                    ),
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[a-cx-z&amp;&amp;\w]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">12</span>),
                negated: <span class="bool-val">false</span>,
                kind: intersection(
                    span(<span class="number">1</span>..<span class="number">11</span>),
                    union(
                        span(<span class="number">1</span>..<span class="number">7</span>),
                        <span class="macro">vec!</span>[
                            range(span(<span class="number">1</span>..<span class="number">4</span>), <span class="string">'a'</span>, <span class="string">'c'</span>),
                            range(span(<span class="number">4</span>..<span class="number">7</span>), <span class="string">'x'</span>, <span class="string">'z'</span>),
                        ]
                    ),
                    itemset(item_perl(ast::ClassPerl {
                        span: span(<span class="number">9</span>..<span class="number">11</span>),
                        kind: ast::ClassPerlKind::Word,
                        negated: <span class="bool-val">false</span>,
                    })),
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[a--b--c]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">9</span>),
                negated: <span class="bool-val">false</span>,
                kind: difference(
                    span(<span class="number">1</span>..<span class="number">8</span>),
                    difference(
                        span(<span class="number">1</span>..<span class="number">5</span>),
                        itemset(lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">'a'</span>)),
                        itemset(lit(span(<span class="number">4</span>..<span class="number">5</span>), <span class="string">'b'</span>)),
                    ),
                    itemset(lit(span(<span class="number">7</span>..<span class="number">8</span>), <span class="string">'c'</span>)),
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[a~~b~~c]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">9</span>),
                negated: <span class="bool-val">false</span>,
                kind: symdifference(
                    span(<span class="number">1</span>..<span class="number">8</span>),
                    symdifference(
                        span(<span class="number">1</span>..<span class="number">5</span>),
                        itemset(lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">'a'</span>)),
                        itemset(lit(span(<span class="number">4</span>..<span class="number">5</span>), <span class="string">'b'</span>)),
                    ),
                    itemset(lit(span(<span class="number">7</span>..<span class="number">8</span>), <span class="string">'c'</span>)),
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[\^&amp;&amp;^]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">7</span>),
                negated: <span class="bool-val">false</span>,
                kind: intersection(
                    span(<span class="number">1</span>..<span class="number">6</span>),
                    itemset(ast::ClassSetItem::Literal(ast::Literal {
                        span: span(<span class="number">1</span>..<span class="number">3</span>),
                        kind: ast::LiteralKind::Meta,
                        c: <span class="string">'^'</span>,
                    })),
                    itemset(lit(span(<span class="number">5</span>..<span class="number">6</span>), <span class="string">'^'</span>)),
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[\&amp;&amp;&amp;&amp;]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">7</span>),
                negated: <span class="bool-val">false</span>,
                kind: intersection(
                    span(<span class="number">1</span>..<span class="number">6</span>),
                    itemset(ast::ClassSetItem::Literal(ast::Literal {
                        span: span(<span class="number">1</span>..<span class="number">3</span>),
                        kind: ast::LiteralKind::Meta,
                        c: <span class="string">'&amp;'</span>,
                    })),
                    itemset(lit(span(<span class="number">5</span>..<span class="number">6</span>), <span class="string">'&amp;'</span>)),
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[&amp;&amp;&amp;&amp;]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">6</span>),
                negated: <span class="bool-val">false</span>,
                kind: intersection(
                    span(<span class="number">1</span>..<span class="number">5</span>),
                    intersection(
                        span(<span class="number">1</span>..<span class="number">3</span>),
                        itemset(empty(span(<span class="number">1</span>..<span class="number">1</span>))),
                        itemset(empty(span(<span class="number">3</span>..<span class="number">3</span>))),
                    ),
                    itemset(empty(span(<span class="number">5</span>..<span class="number">5</span>))),
                ),
            }))
        );

        <span class="kw">let </span>pat = <span class="string">"[☃-⛄]"</span>;
        <span class="macro">assert_eq!</span>(
            parser(pat).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span_range(pat, <span class="number">0</span>..<span class="number">9</span>),
                negated: <span class="bool-val">false</span>,
                kind: itemset(ast::ClassSetItem::Range(ast::ClassSetRange {
                    span: span_range(pat, <span class="number">1</span>..<span class="number">8</span>),
                    start: ast::Literal {
                        span: span_range(pat, <span class="number">1</span>..<span class="number">4</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'☃'</span>,
                    },
                    end: ast::Literal {
                        span: span_range(pat, <span class="number">5</span>..<span class="number">8</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'⛄'</span>,
                    },
                })),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[]]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                negated: <span class="bool-val">false</span>,
                kind: itemset(lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">']'</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[]\[]"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_bracketed(ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                negated: <span class="bool-val">false</span>,
                kind: union(
                    span(<span class="number">1</span>..<span class="number">4</span>),
                    <span class="macro">vec!</span>[
                        lit(span(<span class="number">1</span>..<span class="number">2</span>), <span class="string">']'</span>),
                        ast::ClassSetItem::Literal(ast::Literal {
                            span: span(<span class="number">2</span>..<span class="number">4</span>),
                            kind: ast::LiteralKind::Meta,
                            c: <span class="string">'['</span>,
                        }),
                    ]
                ),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[\[]]"</span>).parse(),
            <span class="prelude-val">Ok</span>(concat(
                <span class="number">0</span>..<span class="number">5</span>,
                <span class="macro">vec!</span>[
                    Ast::class_bracketed(ast::ClassBracketed {
                        span: span(<span class="number">0</span>..<span class="number">4</span>),
                        negated: <span class="bool-val">false</span>,
                        kind: itemset(ast::ClassSetItem::Literal(
                            ast::Literal {
                                span: span(<span class="number">1</span>..<span class="number">3</span>),
                                kind: ast::LiteralKind::Meta,
                                c: <span class="string">'['</span>,
                            }
                        )),
                    }),
                    Ast::literal(ast::Literal {
                        span: span(<span class="number">4</span>..<span class="number">5</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">']'</span>,
                    }),
                ]
            ))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"["</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[["</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[[-]"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[[[:alnum:]"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[\b]"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::ClassEscapeInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[\w-a]"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::ClassRangeLiteral,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[a-\w]"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::ClassRangeLiteral,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[z-a]"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">1</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::ClassRangeInvalid,
            }
        );

        <span class="macro">assert_eq!</span>(
            parser_ignore_whitespace(<span class="string">"[a "</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_ignore_whitespace(<span class="string">"[a- "</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_set_class_open() {
        <span class="macro">assert_eq!</span>(parser(<span class="string">"[a]"</span>).parse_set_class_open(), {
            <span class="kw">let </span>set = ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassSet::union(ast::ClassSetUnion {
                    span: span(<span class="number">1</span>..<span class="number">1</span>),
                    items: <span class="macro">vec!</span>[],
                }),
            };
            <span class="kw">let </span>union = ast::ClassSetUnion { span: span(<span class="number">1</span>..<span class="number">1</span>), items: <span class="macro">vec!</span>[] };
            <span class="prelude-val">Ok</span>((set, union))
        });
        <span class="macro">assert_eq!</span>(
            parser_ignore_whitespace(<span class="string">"[   a]"</span>).parse_set_class_open(),
            {
                <span class="kw">let </span>set = ast::ClassBracketed {
                    span: span(<span class="number">0</span>..<span class="number">4</span>),
                    negated: <span class="bool-val">false</span>,
                    kind: ast::ClassSet::union(ast::ClassSetUnion {
                        span: span(<span class="number">4</span>..<span class="number">4</span>),
                        items: <span class="macro">vec!</span>[],
                    }),
                };
                <span class="kw">let </span>union =
                    ast::ClassSetUnion { span: span(<span class="number">4</span>..<span class="number">4</span>), items: <span class="macro">vec!</span>[] };
                <span class="prelude-val">Ok</span>((set, union))
            }
        );
        <span class="macro">assert_eq!</span>(parser(<span class="string">"[^a]"</span>).parse_set_class_open(), {
            <span class="kw">let </span>set = ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                negated: <span class="bool-val">true</span>,
                kind: ast::ClassSet::union(ast::ClassSetUnion {
                    span: span(<span class="number">2</span>..<span class="number">2</span>),
                    items: <span class="macro">vec!</span>[],
                }),
            };
            <span class="kw">let </span>union = ast::ClassSetUnion { span: span(<span class="number">2</span>..<span class="number">2</span>), items: <span class="macro">vec!</span>[] };
            <span class="prelude-val">Ok</span>((set, union))
        });
        <span class="macro">assert_eq!</span>(
            parser_ignore_whitespace(<span class="string">"[ ^ a]"</span>).parse_set_class_open(),
            {
                <span class="kw">let </span>set = ast::ClassBracketed {
                    span: span(<span class="number">0</span>..<span class="number">4</span>),
                    negated: <span class="bool-val">true</span>,
                    kind: ast::ClassSet::union(ast::ClassSetUnion {
                        span: span(<span class="number">4</span>..<span class="number">4</span>),
                        items: <span class="macro">vec!</span>[],
                    }),
                };
                <span class="kw">let </span>union =
                    ast::ClassSetUnion { span: span(<span class="number">4</span>..<span class="number">4</span>), items: <span class="macro">vec!</span>[] };
                <span class="prelude-val">Ok</span>((set, union))
            }
        );
        <span class="macro">assert_eq!</span>(parser(<span class="string">"[-a]"</span>).parse_set_class_open(), {
            <span class="kw">let </span>set = ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassSet::union(ast::ClassSetUnion {
                    span: span(<span class="number">1</span>..<span class="number">1</span>),
                    items: <span class="macro">vec!</span>[],
                }),
            };
            <span class="kw">let </span>union = ast::ClassSetUnion {
                span: span(<span class="number">1</span>..<span class="number">2</span>),
                items: <span class="macro">vec!</span>[ast::ClassSetItem::Literal(ast::Literal {
                    span: span(<span class="number">1</span>..<span class="number">2</span>),
                    kind: ast::LiteralKind::Verbatim,
                    c: <span class="string">'-'</span>,
                })],
            };
            <span class="prelude-val">Ok</span>((set, union))
        });
        <span class="macro">assert_eq!</span>(
            parser_ignore_whitespace(<span class="string">"[ - a]"</span>).parse_set_class_open(),
            {
                <span class="kw">let </span>set = ast::ClassBracketed {
                    span: span(<span class="number">0</span>..<span class="number">4</span>),
                    negated: <span class="bool-val">false</span>,
                    kind: ast::ClassSet::union(ast::ClassSetUnion {
                        span: span(<span class="number">2</span>..<span class="number">2</span>),
                        items: <span class="macro">vec!</span>[],
                    }),
                };
                <span class="kw">let </span>union = ast::ClassSetUnion {
                    span: span(<span class="number">2</span>..<span class="number">3</span>),
                    items: <span class="macro">vec!</span>[ast::ClassSetItem::Literal(ast::Literal {
                        span: span(<span class="number">2</span>..<span class="number">3</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'-'</span>,
                    })],
                };
                <span class="prelude-val">Ok</span>((set, union))
            }
        );
        <span class="macro">assert_eq!</span>(parser(<span class="string">"[^-a]"</span>).parse_set_class_open(), {
            <span class="kw">let </span>set = ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                negated: <span class="bool-val">true</span>,
                kind: ast::ClassSet::union(ast::ClassSetUnion {
                    span: span(<span class="number">2</span>..<span class="number">2</span>),
                    items: <span class="macro">vec!</span>[],
                }),
            };
            <span class="kw">let </span>union = ast::ClassSetUnion {
                span: span(<span class="number">2</span>..<span class="number">3</span>),
                items: <span class="macro">vec!</span>[ast::ClassSetItem::Literal(ast::Literal {
                    span: span(<span class="number">2</span>..<span class="number">3</span>),
                    kind: ast::LiteralKind::Verbatim,
                    c: <span class="string">'-'</span>,
                })],
            };
            <span class="prelude-val">Ok</span>((set, union))
        });
        <span class="macro">assert_eq!</span>(parser(<span class="string">"[--a]"</span>).parse_set_class_open(), {
            <span class="kw">let </span>set = ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassSet::union(ast::ClassSetUnion {
                    span: span(<span class="number">1</span>..<span class="number">1</span>),
                    items: <span class="macro">vec!</span>[],
                }),
            };
            <span class="kw">let </span>union = ast::ClassSetUnion {
                span: span(<span class="number">1</span>..<span class="number">3</span>),
                items: <span class="macro">vec!</span>[
                    ast::ClassSetItem::Literal(ast::Literal {
                        span: span(<span class="number">1</span>..<span class="number">2</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'-'</span>,
                    }),
                    ast::ClassSetItem::Literal(ast::Literal {
                        span: span(<span class="number">2</span>..<span class="number">3</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'-'</span>,
                    }),
                ],
            };
            <span class="prelude-val">Ok</span>((set, union))
        });
        <span class="macro">assert_eq!</span>(parser(<span class="string">"[]a]"</span>).parse_set_class_open(), {
            <span class="kw">let </span>set = ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassSet::union(ast::ClassSetUnion {
                    span: span(<span class="number">1</span>..<span class="number">1</span>),
                    items: <span class="macro">vec!</span>[],
                }),
            };
            <span class="kw">let </span>union = ast::ClassSetUnion {
                span: span(<span class="number">1</span>..<span class="number">2</span>),
                items: <span class="macro">vec!</span>[ast::ClassSetItem::Literal(ast::Literal {
                    span: span(<span class="number">1</span>..<span class="number">2</span>),
                    kind: ast::LiteralKind::Verbatim,
                    c: <span class="string">']'</span>,
                })],
            };
            <span class="prelude-val">Ok</span>((set, union))
        });
        <span class="macro">assert_eq!</span>(
            parser_ignore_whitespace(<span class="string">"[ ] a]"</span>).parse_set_class_open(),
            {
                <span class="kw">let </span>set = ast::ClassBracketed {
                    span: span(<span class="number">0</span>..<span class="number">4</span>),
                    negated: <span class="bool-val">false</span>,
                    kind: ast::ClassSet::union(ast::ClassSetUnion {
                        span: span(<span class="number">2</span>..<span class="number">2</span>),
                        items: <span class="macro">vec!</span>[],
                    }),
                };
                <span class="kw">let </span>union = ast::ClassSetUnion {
                    span: span(<span class="number">2</span>..<span class="number">3</span>),
                    items: <span class="macro">vec!</span>[ast::ClassSetItem::Literal(ast::Literal {
                        span: span(<span class="number">2</span>..<span class="number">3</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">']'</span>,
                    })],
                };
                <span class="prelude-val">Ok</span>((set, union))
            }
        );
        <span class="macro">assert_eq!</span>(parser(<span class="string">"[^]a]"</span>).parse_set_class_open(), {
            <span class="kw">let </span>set = ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                negated: <span class="bool-val">true</span>,
                kind: ast::ClassSet::union(ast::ClassSetUnion {
                    span: span(<span class="number">2</span>..<span class="number">2</span>),
                    items: <span class="macro">vec!</span>[],
                }),
            };
            <span class="kw">let </span>union = ast::ClassSetUnion {
                span: span(<span class="number">2</span>..<span class="number">3</span>),
                items: <span class="macro">vec!</span>[ast::ClassSetItem::Literal(ast::Literal {
                    span: span(<span class="number">2</span>..<span class="number">3</span>),
                    kind: ast::LiteralKind::Verbatim,
                    c: <span class="string">']'</span>,
                })],
            };
            <span class="prelude-val">Ok</span>((set, union))
        });
        <span class="macro">assert_eq!</span>(parser(<span class="string">"[-]a]"</span>).parse_set_class_open(), {
            <span class="kw">let </span>set = ast::ClassBracketed {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassSet::union(ast::ClassSetUnion {
                    span: span(<span class="number">1</span>..<span class="number">1</span>),
                    items: <span class="macro">vec!</span>[],
                }),
            };
            <span class="kw">let </span>union = ast::ClassSetUnion {
                span: span(<span class="number">1</span>..<span class="number">2</span>),
                items: <span class="macro">vec!</span>[ast::ClassSetItem::Literal(ast::Literal {
                    span: span(<span class="number">1</span>..<span class="number">2</span>),
                    kind: ast::LiteralKind::Verbatim,
                    c: <span class="string">'-'</span>,
                })],
            };
            <span class="prelude-val">Ok</span>((set, union))
        });

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"["</span>).parse_set_class_open().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">1</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser_ignore_whitespace(<span class="string">"[    "</span>)
                .parse_set_class_open()
                .unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[^"</span>).parse_set_class_open().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[]"</span>).parse_set_class_open().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[-"</span>).parse_set_class_open().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">0</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">"[--"</span>).parse_set_class_open().unwrap_err(),
            TestError {
                span: span(<span class="number">0</span>..<span class="number">0</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );

        <span class="comment">// See: https://github.com/rust-lang/regex/issues/792
        </span><span class="macro">assert_eq!</span>(
            parser(<span class="string">"(?x)[-#]"</span>).parse_with_comments().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::ClassUnclosed,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>maybe_parse_ascii_class() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[:alnum:]"</span>).maybe_parse_ascii_class(),
            <span class="prelude-val">Some</span>(ast::ClassAscii {
                span: span(<span class="number">0</span>..<span class="number">9</span>),
                kind: ast::ClassAsciiKind::Alnum,
                negated: <span class="bool-val">false</span>,
            })
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[:alnum:]A"</span>).maybe_parse_ascii_class(),
            <span class="prelude-val">Some</span>(ast::ClassAscii {
                span: span(<span class="number">0</span>..<span class="number">9</span>),
                kind: ast::ClassAsciiKind::Alnum,
                negated: <span class="bool-val">false</span>,
            })
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"[:^alnum:]"</span>).maybe_parse_ascii_class(),
            <span class="prelude-val">Some</span>(ast::ClassAscii {
                span: span(<span class="number">0</span>..<span class="number">10</span>),
                kind: ast::ClassAsciiKind::Alnum,
                negated: <span class="bool-val">true</span>,
            })
        );

        <span class="kw">let </span>p = parser(<span class="string">r"[:"</span>);
        <span class="macro">assert_eq!</span>(p.maybe_parse_ascii_class(), <span class="prelude-val">None</span>);
        <span class="macro">assert_eq!</span>(p.offset(), <span class="number">0</span>);

        <span class="kw">let </span>p = parser(<span class="string">r"[:^"</span>);
        <span class="macro">assert_eq!</span>(p.maybe_parse_ascii_class(), <span class="prelude-val">None</span>);
        <span class="macro">assert_eq!</span>(p.offset(), <span class="number">0</span>);

        <span class="kw">let </span>p = parser(<span class="string">r"[^:alnum:]"</span>);
        <span class="macro">assert_eq!</span>(p.maybe_parse_ascii_class(), <span class="prelude-val">None</span>);
        <span class="macro">assert_eq!</span>(p.offset(), <span class="number">0</span>);

        <span class="kw">let </span>p = parser(<span class="string">r"[:alnnum:]"</span>);
        <span class="macro">assert_eq!</span>(p.maybe_parse_ascii_class(), <span class="prelude-val">None</span>);
        <span class="macro">assert_eq!</span>(p.offset(), <span class="number">0</span>);

        <span class="kw">let </span>p = parser(<span class="string">r"[:alnum]"</span>);
        <span class="macro">assert_eq!</span>(p.maybe_parse_ascii_class(), <span class="prelude-val">None</span>);
        <span class="macro">assert_eq!</span>(p.offset(), <span class="number">0</span>);

        <span class="kw">let </span>p = parser(<span class="string">r"[:alnum:"</span>);
        <span class="macro">assert_eq!</span>(p.maybe_parse_ascii_class(), <span class="prelude-val">None</span>);
        <span class="macro">assert_eq!</span>(p.offset(), <span class="number">0</span>);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_unicode_class() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\pN"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassUnicodeKind::OneLetter(<span class="string">'N'</span>),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\PN"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                negated: <span class="bool-val">true</span>,
                kind: ast::ClassUnicodeKind::OneLetter(<span class="string">'N'</span>),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{N}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassUnicodeKind::Named(s(<span class="string">"N"</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\P{N}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                negated: <span class="bool-val">true</span>,
                kind: ast::ClassUnicodeKind::Named(s(<span class="string">"N"</span>)),
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{Greek}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">9</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassUnicodeKind::Named(s(<span class="string">"Greek"</span>)),
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{scx:Katakana}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">16</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassUnicodeKind::NamedValue {
                    op: ast::ClassUnicodeOpKind::Colon,
                    name: s(<span class="string">"scx"</span>),
                    value: s(<span class="string">"Katakana"</span>),
                },
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{scx=Katakana}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">16</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassUnicodeKind::NamedValue {
                    op: ast::ClassUnicodeOpKind::Equal,
                    name: s(<span class="string">"scx"</span>),
                    value: s(<span class="string">"Katakana"</span>),
                },
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{scx!=Katakana}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">17</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassUnicodeKind::NamedValue {
                    op: ast::ClassUnicodeOpKind::NotEqual,
                    name: s(<span class="string">"scx"</span>),
                    value: s(<span class="string">"Katakana"</span>),
                },
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{:}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassUnicodeKind::NamedValue {
                    op: ast::ClassUnicodeOpKind::Colon,
                    name: s(<span class="string">""</span>),
                    value: s(<span class="string">""</span>),
                },
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{=}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">5</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassUnicodeKind::NamedValue {
                    op: ast::ClassUnicodeOpKind::Equal,
                    name: s(<span class="string">""</span>),
                    value: s(<span class="string">""</span>),
                },
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{!=}"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Unicode(ast::ClassUnicode {
                span: span(<span class="number">0</span>..<span class="number">6</span>),
                negated: <span class="bool-val">false</span>,
                kind: ast::ClassUnicodeKind::NamedValue {
                    op: ast::ClassUnicodeOpKind::NotEqual,
                    name: s(<span class="string">""</span>),
                    value: s(<span class="string">""</span>),
                },
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">2</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">3</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{N"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">4</span>..<span class="number">4</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{Greek"</span>).parse_escape().unwrap_err(),
            TestError {
                span: span(<span class="number">8</span>..<span class="number">8</span>),
                kind: ast::ErrorKind::EscapeUnexpectedEof,
            }
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\pNz"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::concat(ast::Concat {
                span: span(<span class="number">0</span>..<span class="number">4</span>),
                asts: <span class="macro">vec!</span>[
                    Ast::class_unicode(ast::ClassUnicode {
                        span: span(<span class="number">0</span>..<span class="number">3</span>),
                        negated: <span class="bool-val">false</span>,
                        kind: ast::ClassUnicodeKind::OneLetter(<span class="string">'N'</span>),
                    }),
                    Ast::literal(ast::Literal {
                        span: span(<span class="number">3</span>..<span class="number">4</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'z'</span>,
                    }),
                ],
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p{Greek}z"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::concat(ast::Concat {
                span: span(<span class="number">0</span>..<span class="number">10</span>),
                asts: <span class="macro">vec!</span>[
                    Ast::class_unicode(ast::ClassUnicode {
                        span: span(<span class="number">0</span>..<span class="number">9</span>),
                        negated: <span class="bool-val">false</span>,
                        kind: ast::ClassUnicodeKind::Named(s(<span class="string">"Greek"</span>)),
                    }),
                    Ast::literal(ast::Literal {
                        span: span(<span class="number">9</span>..<span class="number">10</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'z'</span>,
                    }),
                ],
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\p\{"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::UnicodeClassInvalid,
            }
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\P\{"</span>).parse().unwrap_err(),
            TestError {
                span: span(<span class="number">2</span>..<span class="number">3</span>),
                kind: ast::ErrorKind::UnicodeClassInvalid,
            }
        );
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>parse_perl_class() {
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\d"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Perl(ast::ClassPerl {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ClassPerlKind::Digit,
                negated: <span class="bool-val">false</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\D"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Perl(ast::ClassPerl {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ClassPerlKind::Digit,
                negated: <span class="bool-val">true</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\s"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Perl(ast::ClassPerl {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ClassPerlKind::Space,
                negated: <span class="bool-val">false</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\S"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Perl(ast::ClassPerl {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ClassPerlKind::Space,
                negated: <span class="bool-val">true</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\w"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Perl(ast::ClassPerl {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ClassPerlKind::Word,
                negated: <span class="bool-val">false</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\W"</span>).parse_escape(),
            <span class="prelude-val">Ok</span>(Primitive::Perl(ast::ClassPerl {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ClassPerlKind::Word,
                negated: <span class="bool-val">true</span>,
            }))
        );

        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\d"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::class_perl(ast::ClassPerl {
                span: span(<span class="number">0</span>..<span class="number">2</span>),
                kind: ast::ClassPerlKind::Digit,
                negated: <span class="bool-val">false</span>,
            }))
        );
        <span class="macro">assert_eq!</span>(
            parser(<span class="string">r"\dz"</span>).parse(),
            <span class="prelude-val">Ok</span>(Ast::concat(ast::Concat {
                span: span(<span class="number">0</span>..<span class="number">3</span>),
                asts: <span class="macro">vec!</span>[
                    Ast::class_perl(ast::ClassPerl {
                        span: span(<span class="number">0</span>..<span class="number">2</span>),
                        kind: ast::ClassPerlKind::Digit,
                        negated: <span class="bool-val">false</span>,
                    }),
                    Ast::literal(ast::Literal {
                        span: span(<span class="number">2</span>..<span class="number">3</span>),
                        kind: ast::LiteralKind::Verbatim,
                        c: <span class="string">'z'</span>,
                    }),
                ],
            }))
        );
    }

    <span class="comment">// This tests a bug fix where the nest limit checker wasn't decrementing
    // its depth during post-traversal, which causes long regexes to trip
    // the default limit too aggressively.
    </span><span class="attr">#[test]
    </span><span class="kw">fn </span>regression_454_nest_too_big() {
        <span class="kw">let </span>pattern = <span class="string">r#"
        2(?:
          [45]\d{3}|
          7(?:
            1[0-267]|
            2[0-289]|
            3[0-29]|
            4[01]|
            5[1-3]|
            6[013]|
            7[0178]|
            91
          )|
          8(?:
            0[125]|
            [139][1-6]|
            2[0157-9]|
            41|
            6[1-35]|
            7[1-5]|
            8[1-8]|
            90
          )|
          9(?:
            0[0-2]|
            1[0-4]|
            2[568]|
            3[3-6]|
            5[5-7]|
            6[0167]|
            7[15]|
            8[0146-9]
          )
        )\d{4}
        "#</span>;
        <span class="macro">assert!</span>(parser_nest_limit(pattern, <span class="number">50</span>).parse().is_ok());
    }

    <span class="comment">// This tests that we treat a trailing `-` in a character class as a
    // literal `-` even when whitespace mode is enabled and there is whitespace
    // after the trailing `-`.
    </span><span class="attr">#[test]
    </span><span class="kw">fn </span>regression_455_trailing_dash_ignore_whitespace() {
        <span class="macro">assert!</span>(parser(<span class="string">"(?x)[ / - ]"</span>).parse().is_ok());
        <span class="macro">assert!</span>(parser(<span class="string">"(?x)[ a - ]"</span>).parse().is_ok());
        <span class="macro">assert!</span>(parser(
            <span class="string">"(?x)[
            a
            - ]
        "
        </span>)
        .parse()
        .is_ok());
        <span class="macro">assert!</span>(parser(
            <span class="string">"(?x)[
            a # wat
            - ]
        "
        </span>)
        .parse()
        .is_ok());

        <span class="macro">assert!</span>(parser(<span class="string">"(?x)[ / -"</span>).parse().is_err());
        <span class="macro">assert!</span>(parser(<span class="string">"(?x)[ / - "</span>).parse().is_err());
        <span class="macro">assert!</span>(parser(
            <span class="string">"(?x)[
            / -
        "
        </span>)
        .parse()
        .is_err());
        <span class="macro">assert!</span>(parser(
            <span class="string">"(?x)[
            / - # wat
        "
        </span>)
        .parse()
        .is_err());
    }
}
</code></pre></div></section></main></body></html>