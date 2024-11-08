<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.37.0/src/time/interval.rs`."><title>interval.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="tokio" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../tokio/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">crate</span>::future::poll_fn;
<span class="kw">use </span><span class="kw">crate</span>::time::{sleep_until, Duration, Instant, Sleep};
<span class="kw">use </span><span class="kw">crate</span>::util::trace;

<span class="kw">use </span>std::future::Future;
<span class="kw">use </span>std::panic::Location;
<span class="kw">use </span>std::pin::Pin;
<span class="kw">use </span>std::task::{Context, Poll};

<span class="doccomment">/// Creates new [`Interval`] that yields with interval of `period`. The first
/// tick completes immediately. The default [`MissedTickBehavior`] is
/// [`Burst`](MissedTickBehavior::Burst), but this can be configured
/// by calling [`set_missed_tick_behavior`](Interval::set_missed_tick_behavior).
///
/// An interval will tick indefinitely. At any time, the [`Interval`] value can
/// be dropped. This cancels the interval.
///
/// This function is equivalent to
/// [`interval_at(Instant::now(), period)`](interval_at).
///
/// # Panics
///
/// This function panics if `period` is zero.
///
/// # Examples
///
/// ```
/// use tokio::time::{self, Duration};
///
/// #[tokio::main]
/// async fn main() {
///     let mut interval = time::interval(Duration::from_millis(10));
///
///     interval.tick().await; // ticks immediately
///     interval.tick().await; // ticks after 10ms
///     interval.tick().await; // ticks after 10ms
///
///     // approximately 20ms have elapsed.
/// }
/// ```
///
/// A simple example using `interval` to execute a task every two seconds.
///
/// The difference between `interval` and [`sleep`] is that an [`Interval`]
/// measures the time since the last tick, which means that [`.tick().await`]
/// may wait for a shorter time than the duration specified for the interval
/// if some time has passed between calls to [`.tick().await`].
///
/// If the tick in the example below was replaced with [`sleep`], the task
/// would only be executed once every three seconds, and not every two
/// seconds.
///
/// ```
/// use tokio::time;
///
/// async fn task_that_takes_a_second() {
///     println!("hello");
///     time::sleep(time::Duration::from_secs(1)).await
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let mut interval = time::interval(time::Duration::from_secs(2));
///     for _i in 0..5 {
///         interval.tick().await;
///         task_that_takes_a_second().await;
///     }
/// }
/// ```
///
/// [`sleep`]: crate::time::sleep()
/// [`.tick().await`]: Interval::tick
</span><span class="attr">#[track_caller]
</span><span class="kw">pub fn </span>interval(period: Duration) -&gt; Interval {
    <span class="macro">assert!</span>(period &gt; Duration::new(<span class="number">0</span>, <span class="number">0</span>), <span class="string">"`period` must be non-zero."</span>);
    internal_interval_at(Instant::now(), period, trace::caller_location())
}

<span class="doccomment">/// Creates new [`Interval`] that yields with interval of `period` with the
/// first tick completing at `start`. The default [`MissedTickBehavior`] is
/// [`Burst`](MissedTickBehavior::Burst), but this can be configured
/// by calling [`set_missed_tick_behavior`](Interval::set_missed_tick_behavior).
///
/// An interval will tick indefinitely. At any time, the [`Interval`] value can
/// be dropped. This cancels the interval.
///
/// # Panics
///
/// This function panics if `period` is zero.
///
/// # Examples
///
/// ```
/// use tokio::time::{interval_at, Duration, Instant};
///
/// #[tokio::main]
/// async fn main() {
///     let start = Instant::now() + Duration::from_millis(50);
///     let mut interval = interval_at(start, Duration::from_millis(10));
///
///     interval.tick().await; // ticks after 50ms
///     interval.tick().await; // ticks after 10ms
///     interval.tick().await; // ticks after 10ms
///
///     // approximately 70ms have elapsed.
/// }
/// ```
</span><span class="attr">#[track_caller]
</span><span class="kw">pub fn </span>interval_at(start: Instant, period: Duration) -&gt; Interval {
    <span class="macro">assert!</span>(period &gt; Duration::new(<span class="number">0</span>, <span class="number">0</span>), <span class="string">"`period` must be non-zero."</span>);
    internal_interval_at(start, period, trace::caller_location())
}

<span class="attr">#[cfg_attr(not(all(tokio_unstable, feature = <span class="string">"tracing"</span>)), allow(unused_variables))]
</span><span class="kw">fn </span>internal_interval_at(
    start: Instant,
    period: Duration,
    location: <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'static </span>Location&lt;<span class="lifetime">'static</span>&gt;&gt;,
) -&gt; Interval {
    <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"tracing"</span>))]
    </span><span class="kw">let </span>resource_span = {
        <span class="kw">let </span>location = location.expect(<span class="string">"should have location if tracing"</span>);

        <span class="macro">tracing::trace_span!</span>(
            parent: <span class="prelude-val">None</span>,
            <span class="string">"runtime.resource"</span>,
            concrete_type = <span class="string">"Interval"</span>,
            kind = <span class="string">"timer"</span>,
            loc.file = location.file(),
            loc.line = location.line(),
            loc.col = location.column(),
        )
    };

    <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"tracing"</span>))]
    </span><span class="kw">let </span>delay = resource_span.in_scope(|| Box::pin(sleep_until(start)));

    <span class="attr">#[cfg(not(all(tokio_unstable, feature = <span class="string">"tracing"</span>)))]
    </span><span class="kw">let </span>delay = Box::pin(sleep_until(start));

    Interval {
        delay,
        period,
        missed_tick_behavior: MissedTickBehavior::default(),
        <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"tracing"</span>))]
        </span>resource_span,
    }
}

<span class="doccomment">/// Defines the behavior of an [`Interval`] when it misses a tick.
///
/// Sometimes, an [`Interval`]'s tick is missed. For example, consider the
/// following:
///
/// ```
/// use tokio::time::{self, Duration};
/// # async fn task_that_takes_one_to_three_millis() {}
///
/// #[tokio::main]
/// async fn main() {
///     // ticks every 2 milliseconds
///     let mut interval = time::interval(Duration::from_millis(2));
///     for _ in 0..5 {
///         interval.tick().await;
///         // if this takes more than 2 milliseconds, a tick will be delayed
///         task_that_takes_one_to_three_millis().await;
///     }
/// }
/// ```
///
/// Generally, a tick is missed if too much time is spent without calling
/// [`Interval::tick()`].
///
/// By default, when a tick is missed, [`Interval`] fires ticks as quickly as it
/// can until it is "caught up" in time to where it should be.
/// `MissedTickBehavior` can be used to specify a different behavior for
/// [`Interval`] to exhibit. Each variant represents a different strategy.
///
/// Note that because the executor cannot guarantee exact precision with timers,
/// these strategies will only apply when the delay is greater than 5
/// milliseconds.
</span><span class="attr">#[derive(Debug, Clone, Copy, PartialEq, Eq)]
</span><span class="kw">pub enum </span>MissedTickBehavior {
    <span class="doccomment">/// Ticks as fast as possible until caught up.
    ///
    /// When this strategy is used, [`Interval`] schedules ticks "normally" (the
    /// same as it would have if the ticks hadn't been delayed), which results
    /// in it firing ticks as fast as possible until it is caught up in time to
    /// where it should be. Unlike [`Delay`] and [`Skip`], the ticks yielded
    /// when `Burst` is used (the [`Instant`]s that [`tick`](Interval::tick)
    /// yields) aren't different than they would have been if a tick had not
    /// been missed. Like [`Skip`], and unlike [`Delay`], the ticks may be
    /// shortened.
    ///
    /// This looks something like this:
    /// ```text
    /// Expected ticks: |     1     |     2     |     3     |     4     |     5     |     6     |
    /// Actual ticks:   | work -----|          delay          | work | work | work -| work -----|
    /// ```
    ///
    /// In code:
    ///
    /// ```
    /// use tokio::time::{interval, Duration};
    /// # async fn task_that_takes_200_millis() {}
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let mut interval = interval(Duration::from_millis(50));
    ///
    /// // First tick resolves immediately after creation
    /// interval.tick().await;
    ///
    /// task_that_takes_200_millis().await;
    /// // The `Interval` has missed a tick
    ///
    /// // Since we have exceeded our timeout, this will resolve immediately
    /// interval.tick().await;
    ///
    /// // Since we are more than 100ms after the start of `interval`, this will
    /// // also resolve immediately.
    /// interval.tick().await;
    ///
    /// // Also resolves immediately, because it was supposed to resolve at
    /// // 150ms after the start of `interval`
    /// interval.tick().await;
    ///
    /// // Resolves immediately
    /// interval.tick().await;
    ///
    /// // Since we have gotten to 200ms after the start of `interval`, this
    /// // will resolve after 50ms
    /// interval.tick().await;
    /// # }
    /// ```
    ///
    /// This is the default behavior when [`Interval`] is created with
    /// [`interval`] and [`interval_at`].
    ///
    /// [`Delay`]: MissedTickBehavior::Delay
    /// [`Skip`]: MissedTickBehavior::Skip
    </span>Burst,

    <span class="doccomment">/// Tick at multiples of `period` from when [`tick`] was called, rather than
    /// from `start`.
    ///
    /// When this strategy is used and [`Interval`] has missed a tick, instead
    /// of scheduling ticks to fire at multiples of `period` from `start` (the
    /// time when the first tick was fired), it schedules all future ticks to
    /// happen at a regular `period` from the point when [`tick`] was called.
    /// Unlike [`Burst`] and [`Skip`], ticks are not shortened, and they aren't
    /// guaranteed to happen at a multiple of `period` from `start` any longer.
    ///
    /// This looks something like this:
    /// ```text
    /// Expected ticks: |     1     |     2     |     3     |     4     |     5     |     6     |
    /// Actual ticks:   | work -----|          delay          | work -----| work -----| work -----|
    /// ```
    ///
    /// In code:
    ///
    /// ```
    /// use tokio::time::{interval, Duration, MissedTickBehavior};
    /// # async fn task_that_takes_more_than_50_millis() {}
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let mut interval = interval(Duration::from_millis(50));
    /// interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    ///
    /// task_that_takes_more_than_50_millis().await;
    /// // The `Interval` has missed a tick
    ///
    /// // Since we have exceeded our timeout, this will resolve immediately
    /// interval.tick().await;
    ///
    /// // But this one, rather than also resolving immediately, as might happen
    /// // with the `Burst` or `Skip` behaviors, will not resolve until
    /// // 50ms after the call to `tick` up above. That is, in `tick`, when we
    /// // recognize that we missed a tick, we schedule the next tick to happen
    /// // 50ms (or whatever the `period` is) from right then, not from when
    /// // were *supposed* to tick
    /// interval.tick().await;
    /// # }
    /// ```
    ///
    /// [`Burst`]: MissedTickBehavior::Burst
    /// [`Skip`]: MissedTickBehavior::Skip
    /// [`tick`]: Interval::tick
    </span>Delay,

    <span class="doccomment">/// Skips missed ticks and tick on the next multiple of `period` from
    /// `start`.
    ///
    /// When this strategy is used, [`Interval`] schedules the next tick to fire
    /// at the next-closest tick that is a multiple of `period` away from
    /// `start` (the point where [`Interval`] first ticked). Like [`Burst`], all
    /// ticks remain multiples of `period` away from `start`, but unlike
    /// [`Burst`], the ticks may not be *one* multiple of `period` away from the
    /// last tick. Like [`Delay`], the ticks are no longer the same as they
    /// would have been if ticks had not been missed, but unlike [`Delay`], and
    /// like [`Burst`], the ticks may be shortened to be less than one `period`
    /// away from each other.
    ///
    /// This looks something like this:
    /// ```text
    /// Expected ticks: |     1     |     2     |     3     |     4     |     5     |     6     |
    /// Actual ticks:   | work -----|          delay          | work ---| work -----| work -----|
    /// ```
    ///
    /// In code:
    ///
    /// ```
    /// use tokio::time::{interval, Duration, MissedTickBehavior};
    /// # async fn task_that_takes_75_millis() {}
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let mut interval = interval(Duration::from_millis(50));
    /// interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    ///
    /// task_that_takes_75_millis().await;
    /// // The `Interval` has missed a tick
    ///
    /// // Since we have exceeded our timeout, this will resolve immediately
    /// interval.tick().await;
    ///
    /// // This one will resolve after 25ms, 100ms after the start of
    /// // `interval`, which is the closest multiple of `period` from the start
    /// // of `interval` after the call to `tick` up above.
    /// interval.tick().await;
    /// # }
    /// ```
    ///
    /// [`Burst`]: MissedTickBehavior::Burst
    /// [`Delay`]: MissedTickBehavior::Delay
    </span>Skip,
}

<span class="kw">impl </span>MissedTickBehavior {
    <span class="doccomment">/// If a tick is missed, this method is called to determine when the next tick should happen.
    </span><span class="kw">fn </span>next_timeout(<span class="kw-2">&amp;</span><span class="self">self</span>, timeout: Instant, now: Instant, period: Duration) -&gt; Instant {
        <span class="kw">match </span><span class="self">self </span>{
            <span class="self">Self</span>::Burst =&gt; timeout + period,
            <span class="self">Self</span>::Delay =&gt; now + period,
            <span class="self">Self</span>::Skip =&gt; {
                now + period
                    - Duration::from_nanos(
                        ((now - timeout).as_nanos() % period.as_nanos())
                            .try_into()
                            <span class="comment">// This operation is practically guaranteed not to
                            // fail, as in order for it to fail, `period` would
                            // have to be longer than `now - timeout`, and both
                            // would have to be longer than 584 years.
                            //
                            // If it did fail, there's not a good way to pass
                            // the error along to the user, so we just panic.
                            </span>.expect(
                                <span class="string">"too much time has elapsed since the interval was supposed to tick"</span>,
                            ),
                    )
            }
        }
    }
}

<span class="kw">impl </span>Default <span class="kw">for </span>MissedTickBehavior {
    <span class="doccomment">/// Returns [`MissedTickBehavior::Burst`].
    ///
    /// For most usecases, the [`Burst`] strategy is what is desired.
    /// Additionally, to preserve backwards compatibility, the [`Burst`]
    /// strategy must be the default. For these reasons,
    /// [`MissedTickBehavior::Burst`] is the default for [`MissedTickBehavior`].
    /// See [`Burst`] for more details.
    ///
    /// [`Burst`]: MissedTickBehavior::Burst
    </span><span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::Burst
    }
}

<span class="doccomment">/// Interval returned by [`interval`] and [`interval_at`].
///
/// This type allows you to wait on a sequence of instants with a certain
/// duration between each instant. Unlike calling [`sleep`] in a loop, this lets
/// you count the time spent between the calls to [`sleep`] as well.
///
/// An `Interval` can be turned into a `Stream` with [`IntervalStream`].
///
/// [`IntervalStream`]: https://docs.rs/tokio-stream/latest/tokio_stream/wrappers/struct.IntervalStream.html
/// [`sleep`]: crate::time::sleep()
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>Interval {
    <span class="doccomment">/// Future that completes the next time the `Interval` yields a value.
    </span>delay: Pin&lt;Box&lt;Sleep&gt;&gt;,

    <span class="doccomment">/// The duration between values yielded by `Interval`.
    </span>period: Duration,

    <span class="doccomment">/// The strategy `Interval` should use when a tick is missed.
    </span>missed_tick_behavior: MissedTickBehavior,

    <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"tracing"</span>))]
    </span>resource_span: tracing::Span,
}

<span class="kw">impl </span>Interval {
    <span class="doccomment">/// Completes when the next instant in the interval has been reached.
    ///
    /// # Cancel safety
    ///
    /// This method is cancellation safe. If `tick` is used as the branch in a `tokio::select!` and
    /// another branch completes first, then no tick has been consumed.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::time;
    ///
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut interval = time::interval(Duration::from_millis(10));
    ///
    ///     interval.tick().await;
    ///     // approximately 0ms have elapsed. The first tick completes immediately.
    ///     interval.tick().await;
    ///     interval.tick().await;
    ///
    ///     // approximately 20ms have elapsed.
    /// }
    /// ```
    </span><span class="kw">pub async fn </span>tick(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; Instant {
        <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"tracing"</span>))]
        </span><span class="kw">let </span>resource_span = <span class="self">self</span>.resource_span.clone();
        <span class="attr">#[cfg(all(tokio_unstable, feature = <span class="string">"tracing"</span>))]
        </span><span class="kw">let </span>instant = trace::async_op(
            || poll_fn(|cx| <span class="self">self</span>.poll_tick(cx)),
            resource_span,
            <span class="string">"Interval::tick"</span>,
            <span class="string">"poll_tick"</span>,
            <span class="bool-val">false</span>,
        );
        <span class="attr">#[cfg(not(all(tokio_unstable, feature = <span class="string">"tracing"</span>)))]
        </span><span class="kw">let </span>instant = poll_fn(|cx| <span class="self">self</span>.poll_tick(cx));

        instant.<span class="kw">await
    </span>}

    <span class="doccomment">/// Polls for the next instant in the interval to be reached.
    ///
    /// This method can return the following values:
    ///
    ///  * `Poll::Pending` if the next instant has not yet been reached.
    ///  * `Poll::Ready(instant)` if the next instant has been reached.
    ///
    /// When this method returns `Poll::Pending`, the current task is scheduled
    /// to receive a wakeup when the instant has elapsed. Note that on multiple
    /// calls to `poll_tick`, only the [`Waker`](std::task::Waker) from the
    /// [`Context`] passed to the most recent call is scheduled to receive a
    /// wakeup.
    </span><span class="kw">pub fn </span>poll_tick(<span class="kw-2">&amp;mut </span><span class="self">self</span>, cx: <span class="kw-2">&amp;mut </span>Context&lt;<span class="lifetime">'_</span>&gt;) -&gt; Poll&lt;Instant&gt; {
        <span class="comment">// Wait for the delay to be done
        </span><span class="macro">ready!</span>(Pin::new(<span class="kw-2">&amp;mut </span><span class="self">self</span>.delay).poll(cx));

        <span class="comment">// Get the time when we were scheduled to tick
        </span><span class="kw">let </span>timeout = <span class="self">self</span>.delay.deadline();

        <span class="kw">let </span>now = Instant::now();

        <span class="comment">// If a tick was not missed, and thus we are being called before the
        // next tick is due, just schedule the next tick normally, one `period`
        // after `timeout`
        //
        // However, if a tick took excessively long and we are now behind,
        // schedule the next tick according to how the user specified with
        // `MissedTickBehavior`
        </span><span class="kw">let </span>next = <span class="kw">if </span>now &gt; timeout + Duration::from_millis(<span class="number">5</span>) {
            <span class="self">self</span>.missed_tick_behavior
                .next_timeout(timeout, now, <span class="self">self</span>.period)
        } <span class="kw">else </span>{
            timeout + <span class="self">self</span>.period
        };

        <span class="comment">// When we arrive here, the internal delay returned `Poll::Ready`.
        // Reset the delay but do not register it. It should be registered with
        // the next call to [`poll_tick`].
        </span><span class="self">self</span>.delay.as_mut().reset_without_reregister(next);

        <span class="comment">// Return the time when we were scheduled to tick
        </span>Poll::Ready(timeout)
    }

    <span class="doccomment">/// Resets the interval to complete one period after the current time.
    ///
    /// This method ignores [`MissedTickBehavior`] strategy.
    ///
    /// This is equivalent to calling `reset_at(Instant::now() + period)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::time;
    ///
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut interval = time::interval(Duration::from_millis(100));
    ///
    ///     interval.tick().await;
    ///
    ///     time::sleep(Duration::from_millis(50)).await;
    ///     interval.reset();
    ///
    ///     interval.tick().await;
    ///     interval.tick().await;
    ///
    ///     // approximately 250ms have elapsed.
    /// }
    /// ```
    </span><span class="kw">pub fn </span>reset(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.delay.as_mut().reset(Instant::now() + <span class="self">self</span>.period);
    }

    <span class="doccomment">/// Resets the interval immediately.
    ///
    /// This method ignores [`MissedTickBehavior`] strategy.
    ///
    /// This is equivalent to calling `reset_at(Instant::now())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::time;
    ///
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut interval = time::interval(Duration::from_millis(100));
    ///
    ///     interval.tick().await;
    ///
    ///     time::sleep(Duration::from_millis(50)).await;
    ///     interval.reset_immediately();
    ///
    ///     interval.tick().await;
    ///     interval.tick().await;
    ///
    ///     // approximately 150ms have elapsed.
    /// }
    /// ```
    </span><span class="kw">pub fn </span>reset_immediately(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.delay.as_mut().reset(Instant::now());
    }

    <span class="doccomment">/// Resets the interval after the specified [`std::time::Duration`].
    ///
    /// This method ignores [`MissedTickBehavior`] strategy.
    ///
    /// This is equivalent to calling `reset_at(Instant::now() + after)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::time;
    ///
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut interval = time::interval(Duration::from_millis(100));
    ///     interval.tick().await;
    ///
    ///     time::sleep(Duration::from_millis(50)).await;
    ///
    ///     let after = Duration::from_millis(20);
    ///     interval.reset_after(after);
    ///
    ///     interval.tick().await;
    ///     interval.tick().await;
    ///
    ///     // approximately 170ms have elapsed.
    /// }
    /// ```
    </span><span class="kw">pub fn </span>reset_after(<span class="kw-2">&amp;mut </span><span class="self">self</span>, after: Duration) {
        <span class="self">self</span>.delay.as_mut().reset(Instant::now() + after);
    }

    <span class="doccomment">/// Resets the interval to a [`crate::time::Instant`] deadline.
    ///
    /// Sets the next tick to expire at the given instant. If the instant is in
    /// the past, then the [`MissedTickBehavior`] strategy will be used to
    /// catch up. If the instant is in the future, then the next tick will
    /// complete at the given instant, even if that means that it will sleep for
    /// longer than the duration of this [`Interval`]. If the [`Interval`] had
    /// any missed ticks before calling this method, then those are discarded.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::time::{self, Instant};
    ///
    /// use std::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut interval = time::interval(Duration::from_millis(100));
    ///     interval.tick().await;
    ///
    ///     time::sleep(Duration::from_millis(50)).await;
    ///
    ///     let deadline = Instant::now() + Duration::from_millis(30);
    ///     interval.reset_at(deadline);
    ///
    ///     interval.tick().await;
    ///     interval.tick().await;
    ///
    ///     // approximately 180ms have elapsed.
    /// }
    /// ```
    </span><span class="kw">pub fn </span>reset_at(<span class="kw-2">&amp;mut </span><span class="self">self</span>, deadline: Instant) {
        <span class="self">self</span>.delay.as_mut().reset(deadline);
    }

    <span class="doccomment">/// Returns the [`MissedTickBehavior`] strategy currently being used.
    </span><span class="kw">pub fn </span>missed_tick_behavior(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; MissedTickBehavior {
        <span class="self">self</span>.missed_tick_behavior
    }

    <span class="doccomment">/// Sets the [`MissedTickBehavior`] strategy that should be used.
    </span><span class="kw">pub fn </span>set_missed_tick_behavior(<span class="kw-2">&amp;mut </span><span class="self">self</span>, behavior: MissedTickBehavior) {
        <span class="self">self</span>.missed_tick_behavior = behavior;
    }

    <span class="doccomment">/// Returns the period of the interval.
    </span><span class="kw">pub fn </span>period(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Duration {
        <span class="self">self</span>.period
    }
}
</code></pre></div></section></main></body></html>