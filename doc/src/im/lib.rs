<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/im-15.1.0/./src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="im" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../im/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

</span><span class="doccomment">//! # Immutable Data Structures for Rust
//!
//! This library implements several of the more commonly useful immutable data
//! structures for Rust.
//!
//! ## What are immutable data structures?
//!
//! Immutable data structures are data structures which can be copied and
//! modified efficiently without altering the original. The most uncomplicated
//! example of this is the venerable [cons list][cons-list]. This crate offers a
//! selection of more modern and flexible data structures with similar
//! properties, tuned for the needs of Rust developers.
//!
//! Briefly, the following data structures are provided:
//!
//! * [Vectors][vector::Vector] based on [RRB trees][rrb-tree]
//! * [Hash maps][hashmap::HashMap]/[sets][hashset::HashSet] based on [hash
//!   array mapped tries][hamt]
//! * [Ordered maps][ordmap::OrdMap]/[sets][ordset::OrdSet] based on
//!   [B-trees][b-tree]
//!
//! ## Why Would I Want This?
//!
//! While immutable data structures can be a game changer for other
//! programming languages, the most obvious benefit - avoiding the
//! accidental mutation of data - is already handled so well by Rust's
//! type system that it's just not something a Rust programmer needs
//! to worry about even when using data structures that would send a
//! conscientious Clojure programmer into a panic.
//!
//! Immutable data structures offer other benefits, though, some of
//! which are useful even in a language like Rust. The most prominent
//! is *structural sharing*, which means that if two data structures
//! are mostly copies of each other, most of the memory they take up
//! will be shared between them. This implies that making copies of an
//! immutable data structure is cheap: it's really only a matter of
//! copying a pointer and increasing a reference counter, where in the
//! case of [`Vec`][std::vec::Vec] you have to allocate the same
//! amount of memory all over again and make a copy of every element
//! it contains. For immutable data structures, extra memory isn't
//! allocated until you modify either the copy or the original, and
//! then only the memory needed to record the difference.
//!
//! Another goal of this library has been the idea that you shouldn't
//! even have to think about what data structure to use in any given
//! situation, until the point where you need to start worrying about
//! optimisation - which, in practice, often never comes. Beyond the
//! shape of your data (ie. whether to use a list or a map), it should
//! be fine not to think too carefully about data structures - you can
//! just pick the one that has the right shape and it should have
//! acceptable performance characteristics for every operation you
//! might need. Specialised data structures will always be faster at
//! what they've been specialised for, but `im` aims to provide the
//! data structures which deliver the least chance of accidentally
//! using them for the wrong thing.
//!
//! For instance, [`Vec`][std::vec::Vec] beats everything at memory
//! usage, indexing and operations that happen at the back of the
//! list, but is terrible at insertion and removal, and gets worse the
//! closer to the front of the list you get.
//! [`VecDeque`][std::collections::VecDeque] adds a little bit of
//! complexity in order to make operations at the front as efficient
//! as operations at the back, but is still bad at insertion and
//! especially concatenation. [`Vector`][vector::Vector] adds another
//! bit of complexity, and could never match [`Vec`][std::vec::Vec] at
//! what it's best at, but in return every operation you can throw at
//! it can be completed in a reasonable amount of time - even normally
//! expensive operations like copying and especially concatenation are
//! reasonably cheap when using a [`Vector`][vector::Vector].
//!
//! It should be noted, however, that because of its simplicity,
//! [`Vec`][std::vec::Vec] actually beats [`Vector`][vector::Vector] even at its
//! strongest operations at small sizes, just because modern CPUs are
//! hyperoptimised for things like copying small chunks of contiguous memory -
//! you actually need to go past a certain size (usually in the vicinity of
//! several hundred elements) before you get to the point where
//! [`Vec`][std::vec::Vec] isn't always going to be the fastest choice.
//! [`Vector`][vector::Vector] attempts to overcome this by actually just being
//! an array at very small sizes, and being able to switch efficiently to the
//! full data structure when it grows large enough. Thus,
//! [`Vector`][vector::Vector] will actually be equivalent to
//! [Vec][std::vec::Vec] until it grows past the size of a single chunk.
//!
//! The maps - [`HashMap`][hashmap::HashMap] and
//! [`OrdMap`][ordmap::OrdMap] - generally perform similarly to their
//! equivalents in the standard library, but tend to run a bit slower
//! on the basic operations ([`HashMap`][hashmap::HashMap] is almost
//! neck and neck with its counterpart, while
//! [`OrdMap`][ordmap::OrdMap] currently tends to run 2-3x slower). On
//! the other hand, they offer the cheap copy and structural sharing
//! between copies that you'd expect from immutable data structures.
//!
//! In conclusion, the aim of this library is to provide a safe
//! default choice for the most common kinds of data structures,
//! allowing you to defer careful thinking about the right data
//! structure for the job until you need to start looking for
//! optimisations - and you may find, especially for larger data sets,
//! that immutable data structures are still the right choice.
//!
//! ## Values
//!
//! Because we need to make copies of shared nodes in these data structures
//! before updating them, the values you store in them must implement
//! [`Clone`][std::clone::Clone].  For primitive values that implement
//! [`Copy`][std::marker::Copy], such as numbers, everything is fine: this is
//! the case for which the data structures are optimised, and performance is
//! going to be great.
//!
//! On the other hand, if you want to store values for which cloning is
//! expensive, or values that don't implement [`Clone`][std::clone::Clone], you
//! need to wrap them in [`Rc`][std::rc::Rc] or [`Arc`][std::sync::Arc]. Thus,
//! if you have a complex structure `BigBlobOfData` and you want to store a list
//! of them as a `Vector&lt;BigBlobOfData&gt;`, you should instead use a
//! `Vector&lt;Rc&lt;BigBlobOfData&gt;&gt;`, which is going to save you not only the time
//! spent cloning the big blobs of data, but also the memory spent keeping
//! multiple copies of it around, as [`Rc`][std::rc::Rc] keeps a single
//! reference counted copy around instead.
//!
//! If you're storing smaller values that aren't
//! [`Copy`][std::marker::Copy]able, you'll need to exercise judgement: if your
//! values are going to be very cheap to clone, as would be the case for short
//! [`String`][std::string::String]s or small [`Vec`][std::vec::Vec]s, you're
//! probably better off storing them directly without wrapping them in an
//! [`Rc`][std::rc::Rc], because, like the [`Rc`][std::rc::Rc], they're just
//! pointers to some data on the heap, and that data isn't expensive to clone -
//! you might actually lose more performance from the extra redirection of
//! wrapping them in an [`Rc`][std::rc::Rc] than you would from occasionally
//! cloning them.
//!
//! ### When does cloning happen?
//!
//! So when will your values actually be cloned? The easy answer is only if you
//! [`clone`][std::clone::Clone::clone] the data structure itself, and then only
//! lazily as you change it. Values are stored in tree nodes inside the data
//! structure, each node of which contains up to 64 values. When you
//! [`clone`][std::clone::Clone::clone] a data structure, nothing is actually
//! copied - it's just the reference count on the root node that's incremented,
//! to indicate that it's shared between two data structures. It's only when you
//! actually modify one of the shared data structures that nodes are cloned:
//! when you make a change somewhere in the tree, the node containing the change
//! needs to be cloned, and then its parent nodes need to be updated to contain
//! the new child node instead of the old version, and so they're cloned as
//! well.
//!
//! We can call this "lazy" cloning - if you make two copies of a data structure
//! and you never change either of them, there's never any need to clone the
//! data they contain. It's only when you start making changes that cloning
//! starts to happen, and then only on the specific tree nodes that are part of
//! the change. Note that the implications of lazily cloning the data structure
//! extend to memory usage as well as the CPU workload of copying the data
//! around - cloning an immutable data structure means both copies share the
//! same allocated memory, until you start making changes.
//!
//! Most crucially, if you never clone the data structure, the data inside it is
//! also never cloned, and in this case it acts just like a mutable data
//! structure, with minimal performance differences (but still non-zero, as we
//! still have to check for shared nodes).
//!
//! ## Data Structures
//!
//! We'll attempt to provide a comprehensive guide to the available
//! data structures below.
//!
//! ### Performance Notes
//!
//! "Big O notation" is the standard way of talking about the time
//! complexity of data structure operations. If you're not familiar
//! with big O notation, here's a quick cheat sheet:
//!
//! *O(1)* means an operation runs in constant time: it will take the
//! same time to complete regardless of the size of the data
//! structure.
//!
//! *O(n)* means an operation runs in linear time: if you double the
//! size of your data structure, the operation will take twice as long
//! to complete; if you quadruple the size, it will take four times as
//! long, etc.
//!
//! *O(log n)* means an operation runs in logarithmic time: for
//! *log&lt;sub&gt;2&lt;/sub&gt;*, if you double the size of your data structure,
//! the operation will take one step longer to complete; if you
//! quadruple the size, it will need two steps more; and so on.
//! However, the data structures in this library generally run in
//! *log&lt;sub&gt;64&lt;/sub&gt;* time, meaning you have to make your data
//! structure 64 times bigger to need one extra step, and 4096 times
//! bigger to need two steps. This means that, while they still count
//! as O(log n), operations on all but really large data sets will run
//! at near enough to O(1) that you won't usually notice.
//!
//! *O(n log n)* is the most expensive operation you'll see in this
//! library: it means that for every one of the *n* elements in your
//! data structure, you have to perform *log n* operations. In our
//! case, as noted above, this is often close enough to O(n) that it's
//! not usually as bad as it sounds, but even O(n) isn't cheap and the
//! cost still increases logarithmically, if slowly, as the size of
//! your data increases. O(n log n) basically means "are you sure you
//! need to do this?"
//!
//! *O(1)** means 'amortised O(1),' which means that an operation
//! usually runs in constant time but will occasionally be more
//! expensive: for instance,
//! [`Vector::push_back`][vector::Vector::push_back], if called in
//! sequence, will be O(1) most of the time but every 64th time it
//! will be O(log n), as it fills up its tail chunk and needs to
//! insert it into the tree. Please note that the O(1) with the
//! asterisk attached is not a common notation; it's just a convention
//! I've used in these docs to save myself from having to type
//! 'amortised' everywhere.
//!
//! ### Lists
//!
//! Lists are sequences of single elements which maintain the order in
//! which you inserted them. The only list in this library is
//! [`Vector`][vector::Vector], which offers the best all round
//! performance characteristics: it's pretty good at everything, even
//! if there's always another kind of list that's better at something.
//!
//! | Type | Algorithm | Constraints | Order | Push | Pop | Split | Append | Lookup |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | [`Vector&lt;A&gt;`][vector::Vector] | [RRB tree][rrb-tree] | [`Clone`][std::clone::Clone] | insertion | O(1)\* | O(1)\* | O(log n) | O(log n) | O(log n) |
//!
//! ### Maps
//!
//! Maps are mappings of keys to values, where the most common read
//! operation is to find the value associated with a given key. Maps
//! may or may not have a defined order. Any given key can only occur
//! once inside a map, and setting a key to a different value will
//! overwrite the previous value.
//!
//! | Type | Algorithm | Key Constraints | Order | Insert | Remove | Lookup |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | [`HashMap&lt;K, V&gt;`][hashmap::HashMap] | [HAMT][hamt] | [`Clone`][std::clone::Clone] + [`Hash`][std::hash::Hash] + [`Eq`][std::cmp::Eq] | undefined | O(log n) | O(log n) | O(log n) |
//! | [`OrdMap&lt;K, V&gt;`][ordmap::OrdMap] | [B-tree][b-tree] | [`Clone`][std::clone::Clone] + [`Ord`][std::cmp::Ord] | sorted | O(log n) | O(log n) | O(log n) |
//!
//! ### Sets
//!
//! Sets are collections of unique values, and may or may not have a
//! defined order. Their crucial property is that any given value can
//! only exist once in a given set.
//!
//! | Type | Algorithm | Constraints | Order | Insert | Remove | Lookup |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | [`HashSet&lt;A&gt;`][hashset::HashSet] | [HAMT][hamt] | [`Clone`][std::clone::Clone] + [`Hash`][std::hash::Hash] + [`Eq`][std::cmp::Eq] | undefined | O(log n) | O(log n) | O(log n) |
//! | [`OrdSet&lt;A&gt;`][ordset::OrdSet] | [B-tree][b-tree] | [`Clone`][std::clone::Clone] + [`Ord`][std::cmp::Ord] | sorted | O(log n) | O(log n) | O(log n) |
//!
//! ## In-place Mutation
//!
//! All of these data structures support in-place copy-on-write
//! mutation, which means that if you're the sole user of a data
//! structure, you can update it in place without taking the
//! performance hit of making a copy of the data structure before
//! modifying it (this is about an order of magnitude faster than
//! immutable operations, almost as fast as
//! [`std::collections`][std::collections]'s mutable data structures).
//!
//! Thanks to [`Rc`][std::rc::Rc]'s reference counting, we are able to
//! determine whether a node in a data structure is being shared with
//! other data structures, or whether it's safe to mutate it in place.
//! When it's shared, we'll automatically make a copy of the node
//! before modifying it. The consequence of this is that cloning a
//! data structure becomes a lazy operation: the initial clone is
//! instant, and as you modify the cloned data structure it will clone
//! chunks only where you change them, so that if you change the
//! entire thing you will eventually have performed a full clone.
//!
//! This also gives us a couple of other optimisations for free:
//! implementations of immutable data structures in other languages
//! often have the idea of local mutation, like Clojure's transients
//! or Haskell's `ST` monad - a managed scope where you can treat an
//! immutable data structure like a mutable one, gaining a
//! considerable amount of performance because you no longer need to
//! copy your changed nodes for every operation, just the first time
//! you hit a node that's sharing structure. In Rust, we don't need to
//! think about this kind of managed scope, it's all taken care of
//! behind the scenes because of our low level access to the garbage
//! collector (which, in our case, is just a simple
//! [`Rc`][std::rc::Rc]).
//!
//! ## Thread Safety
//!
//! The data structures in the `im` crate are thread safe, through
//! [`Arc`][std::sync::Arc]. This comes with a slight performance impact, so
//! that if you prioritise speed over thread safety, you may want to use the
//! `im-rc` crate instead, which is identical to `im` except that it uses
//! [`Rc`][std::rc::Rc] instead of [`Arc`][std::sync::Arc], implying that the
//! data structures in `im-rc` do not implement [`Send`][std::marker::Send] and
//! [`Sync`][std::marker::Sync]. This yields approximately a 20-25% increase in
//! general performance.
//!
//! ## Feature Flags
//!
//! `im` comes with optional support for the following crates through Cargo
//! feature flags. You can enable them in your `Cargo.toml` file like this:
//!
//! ```no_compile
//! [dependencies]
//! im = { version = "*", features = ["proptest", "serde"] }
//! ```
//!
//! | Feature | Description |
//! | ------- | ----------- |
//! | [`pool`](https://crates.io/crates/refpool) | Constructors and pool types for [`refpool`](https://crates.io/crates/refpool) memory pools (only available in `im-rc`) |
//! | [`proptest`](https://crates.io/crates/proptest) | Strategies for all `im` datatypes under a `proptest` namespace, eg. `im::vector::proptest::vector()` |
//! | [`quickcheck`](https://crates.io/crates/quickcheck) | [`quickcheck::Arbitrary`](https://docs.rs/quickcheck/latest/quickcheck/trait.Arbitrary.html) implementations for all `im` datatypes (not available in `im-rc`) |
//! | [`rayon`](https://crates.io/crates/rayon) | parallel iterator implementations for [`Vector`][vector::Vector] (not available in `im-rc`) |
//! | [`serde`](https://crates.io/crates/serde) | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) implementations for all `im` datatypes |
//! | [`arbitrary`](https://crates.io/crates/arbitrary/) | [`arbitrary::Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) implementations for all `im` datatypes |
//!
//! [std::collections]: https://doc.rust-lang.org/std/collections/index.html
//! [std::collections::VecDeque]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
//! [std::vec::Vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
//! [std::string::String]: https://doc.rust-lang.org/std/string/struct.String.html
//! [std::rc::Rc]: https://doc.rust-lang.org/std/rc/struct.Rc.html
//! [std::sync::Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [std::cmp::Eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
//! [std::cmp::Ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
//! [std::clone::Clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
//! [std::clone::Clone::clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone
//! [std::marker::Copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
//! [std::hash::Hash]: https://doc.rust-lang.org/std/hash/trait.Hash.html
//! [std::marker::Send]: https://doc.rust-lang.org/std/marker/trait.Send.html
//! [std::marker::Sync]: https://doc.rust-lang.org/std/marker/trait.Sync.html
//! [hashmap::HashMap]: ./struct.HashMap.html
//! [hashset::HashSet]: ./struct.HashSet.html
//! [ordmap::OrdMap]: ./struct.OrdMap.html
//! [ordset::OrdSet]: ./struct.OrdSet.html
//! [vector::Vector]: ./struct.Vector.html
//! [vector::Vector::push_back]: ./vector/enum.Vector.html#method.push_back
//! [rrb-tree]: https://infoscience.epfl.ch/record/213452/files/rrbvector.pdf
//! [hamt]: https://en.wikipedia.org/wiki/Hash_array_mapped_trie
//! [b-tree]: https://en.wikipedia.org/wiki/B-tree
//! [cons-list]: https://en.wikipedia.org/wiki/Cons#Lists

</span><span class="attr">#![forbid(rust_2018_idioms)]
#![deny(unsafe_code, nonstandard_style)]
#![warn(unreachable_pub, missing_docs)]
#![cfg_attr(has_specialisation, feature(specialization))]

#[cfg(test)]
#[macro_use]
</span><span class="kw">extern crate </span>pretty_assertions;

<span class="kw">mod </span>config;
<span class="kw">mod </span>nodes;
<span class="kw">mod </span>sort;
<span class="kw">mod </span>sync;

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>util;

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>ord;
<span class="kw">pub use </span><span class="kw">crate</span>::ord::map <span class="kw">as </span>ordmap;
<span class="kw">pub use </span><span class="kw">crate</span>::ord::set <span class="kw">as </span>ordset;

<span class="attr">#[macro_use]
</span><span class="kw">mod </span>hash;
<span class="kw">pub use </span><span class="kw">crate</span>::hash::map <span class="kw">as </span>hashmap;
<span class="kw">pub use </span><span class="kw">crate</span>::hash::set <span class="kw">as </span>hashset;

<span class="attr">#[macro_use]
</span><span class="kw">pub mod </span>vector;

<span class="kw">pub mod </span>iter;

<span class="attr">#[cfg(any(test, feature = <span class="string">"proptest"</span>))]
</span><span class="kw">pub mod </span>proptest;

<span class="attr">#[cfg(any(test, feature = <span class="string">"serde"</span>))]
#[doc(hidden)]
</span><span class="kw">pub mod </span>ser;

<span class="attr">#[cfg(feature = <span class="string">"arbitrary"</span>)]
#[doc(hidden)]
</span><span class="kw">pub mod </span>arbitrary;

<span class="attr">#[cfg(all(threadsafe, feature = <span class="string">"quickcheck"</span>))]
#[doc(hidden)]
</span><span class="kw">pub mod </span>quickcheck;

<span class="attr">#[cfg(any(threadsafe, not(feature = <span class="string">"pool"</span>)))]
</span><span class="kw">mod </span>fakepool;

<span class="attr">#[cfg(all(threadsafe, feature = <span class="string">"pool"</span>))]
</span><span class="macro">compile_error!</span>(
    <span class="string">"The `pool` feature is not threadsafe but you've enabled it on a threadsafe version of `im`."
</span>);

<span class="kw">pub use </span><span class="kw">crate</span>::hashmap::HashMap;
<span class="kw">pub use </span><span class="kw">crate</span>::hashset::HashSet;
<span class="kw">pub use </span><span class="kw">crate</span>::ordmap::OrdMap;
<span class="kw">pub use </span><span class="kw">crate</span>::ordset::OrdSet;
<span class="attr">#[doc(inline)]
</span><span class="kw">pub use </span><span class="kw">crate</span>::vector::Vector;

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>test;

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests;

<span class="doccomment">/// Update a value inside multiple levels of data structures.
///
/// This macro takes a [`Vector`][Vector], [`OrdMap`][OrdMap] or [`HashMap`][HashMap],
/// a key or a series of keys, and a value, and returns the data structure with the
/// new value at the location described by the keys.
///
/// If one of the keys in the path doesn't exist, the macro will panic.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate im;
/// # use std::sync::Arc;
/// # fn main() {
/// let vec_inside_vec = vector![vector![1, 2, 3], vector![4, 5, 6]];
///
/// let expected = vector![vector![1, 2, 3], vector![4, 5, 1337]];
///
/// assert_eq!(expected, update_in![vec_inside_vec, 1 =&gt; 2, 1337]);
/// # }
/// ```
///
/// [Vector]: ../vector/enum.Vector.html
/// [HashMap]: ../hashmap/struct.HashMap.html
/// [OrdMap]: ../ordmap/struct.OrdMap.html
</span><span class="attr">#[macro_export]
</span><span class="macro">macro_rules!</span> update_in {
    (<span class="macro-nonterminal">$target</span>:expr, <span class="macro-nonterminal">$path</span>:expr =&gt; $(<span class="macro-nonterminal">$tail</span>:tt) =&gt; <span class="kw-2">*</span>, <span class="macro-nonterminal">$value</span>:expr ) =&gt; {{
        <span class="kw">let </span>inner = <span class="macro-nonterminal">$target</span>.get(<span class="macro-nonterminal">$path</span>).expect(<span class="string">"update_in! macro: key not found in target"</span>);
        <span class="macro-nonterminal">$target</span>.update(<span class="macro-nonterminal">$path</span>, <span class="macro">update_in!</span>(inner, $(<span class="macro-nonterminal">$tail</span>) =&gt; <span class="kw-2">*</span>, <span class="macro-nonterminal">$value</span>))
    }};

    (<span class="macro-nonterminal">$target</span>:expr, <span class="macro-nonterminal">$path</span>:expr, <span class="macro-nonterminal">$value</span>:expr) =&gt; {
        <span class="macro-nonterminal">$target</span>.update(<span class="macro-nonterminal">$path</span>, <span class="macro-nonterminal">$value</span>)
    };
}

<span class="doccomment">/// Get a value inside multiple levels of data structures.
///
/// This macro takes a [`Vector`][Vector], [`OrdMap`][OrdMap] or [`HashMap`][HashMap],
/// along with a key or a series of keys, and returns the value at the location inside
/// the data structure described by the key sequence, or `None` if any of the keys didn't
/// exist.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate im;
/// # use std::sync::Arc;
/// # fn main() {
/// let vec_inside_vec = vector![vector![1, 2, 3], vector![4, 5, 6]];
///
/// assert_eq!(Some(&amp;6), get_in![vec_inside_vec, 1 =&gt; 2]);
/// # }
/// ```
///
/// [Vector]: ../vector/enum.Vector.html
/// [HashMap]: ../hashmap/struct.HashMap.html
/// [OrdMap]: ../ordmap/struct.OrdMap.html
</span><span class="attr">#[macro_export]
</span><span class="macro">macro_rules!</span> get_in {
    (<span class="macro-nonterminal">$target</span>:expr, <span class="macro-nonterminal">$path</span>:expr =&gt; $(<span class="macro-nonterminal">$tail</span>:tt) =&gt; * ) =&gt; {{
        <span class="macro-nonterminal">$target</span>.get(<span class="macro-nonterminal">$path</span>).and_then(|v| <span class="macro">get_in!</span>(v, $(<span class="macro-nonterminal">$tail</span>) =&gt; <span class="kw-2">*</span>))
    }};

    (<span class="macro-nonterminal">$target</span>:expr, <span class="macro-nonterminal">$path</span>:expr) =&gt; {
        <span class="macro-nonterminal">$target</span>.get(<span class="macro-nonterminal">$path</span>)
    };
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>lib_test {
    <span class="attr">#[test]
    </span><span class="kw">fn </span>update_in() {
        <span class="kw">let </span>vector = <span class="macro">vector!</span>[<span class="number">1</span>, <span class="number">2</span>, <span class="number">3</span>, <span class="number">4</span>, <span class="number">5</span>];
        <span class="macro">assert_eq!</span>(<span class="macro">vector!</span>[<span class="number">1</span>, <span class="number">2</span>, <span class="number">23</span>, <span class="number">4</span>, <span class="number">5</span>], <span class="macro">update_in!</span>(vector, <span class="number">2</span>, <span class="number">23</span>));
        <span class="kw">let </span>hashmap = <span class="macro">hashmap!</span>[<span class="number">1 </span>=&gt; <span class="number">1</span>, <span class="number">2 </span>=&gt; <span class="number">2</span>, <span class="number">3 </span>=&gt; <span class="number">3</span>];
        <span class="macro">assert_eq!</span>(
            <span class="macro">hashmap!</span>[<span class="number">1 </span>=&gt; <span class="number">1</span>, <span class="number">2 </span>=&gt; <span class="number">23</span>, <span class="number">3 </span>=&gt; <span class="number">3</span>],
            <span class="macro">update_in!</span>(hashmap, <span class="number">2</span>, <span class="number">23</span>)
        );
        <span class="kw">let </span>ordmap = <span class="macro">ordmap!</span>[<span class="number">1 </span>=&gt; <span class="number">1</span>, <span class="number">2 </span>=&gt; <span class="number">2</span>, <span class="number">3 </span>=&gt; <span class="number">3</span>];
        <span class="macro">assert_eq!</span>(<span class="macro">ordmap!</span>[<span class="number">1 </span>=&gt; <span class="number">1</span>, <span class="number">2 </span>=&gt; <span class="number">23</span>, <span class="number">3 </span>=&gt; <span class="number">3</span>], <span class="macro">update_in!</span>(ordmap, <span class="number">2</span>, <span class="number">23</span>));

        <span class="kw">let </span>vecs = <span class="macro">vector!</span>[<span class="macro">vector!</span>[<span class="number">1</span>, <span class="number">2</span>, <span class="number">3</span>], <span class="macro">vector!</span>[<span class="number">4</span>, <span class="number">5</span>, <span class="number">6</span>], <span class="macro">vector!</span>[<span class="number">7</span>, <span class="number">8</span>, <span class="number">9</span>]];
        <span class="kw">let </span>vecs_target = <span class="macro">vector!</span>[<span class="macro">vector!</span>[<span class="number">1</span>, <span class="number">2</span>, <span class="number">3</span>], <span class="macro">vector!</span>[<span class="number">4</span>, <span class="number">5</span>, <span class="number">23</span>], <span class="macro">vector!</span>[<span class="number">7</span>, <span class="number">8</span>, <span class="number">9</span>]];
        <span class="macro">assert_eq!</span>(vecs_target, <span class="macro">update_in!</span>(vecs, <span class="number">1 </span>=&gt; <span class="number">2</span>, <span class="number">23</span>));
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>get_in() {
        <span class="kw">let </span>vector = <span class="macro">vector!</span>[<span class="number">1</span>, <span class="number">2</span>, <span class="number">3</span>, <span class="number">4</span>, <span class="number">5</span>];
        <span class="macro">assert_eq!</span>(<span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span><span class="number">3</span>), <span class="macro">get_in!</span>(vector, <span class="number">2</span>));
        <span class="kw">let </span>hashmap = <span class="macro">hashmap!</span>[<span class="number">1 </span>=&gt; <span class="number">1</span>, <span class="number">2 </span>=&gt; <span class="number">2</span>, <span class="number">3 </span>=&gt; <span class="number">3</span>];
        <span class="macro">assert_eq!</span>(<span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span><span class="number">2</span>), <span class="macro">get_in!</span>(hashmap, <span class="kw-2">&amp;</span><span class="number">2</span>));
        <span class="kw">let </span>ordmap = <span class="macro">ordmap!</span>[<span class="number">1 </span>=&gt; <span class="number">1</span>, <span class="number">2 </span>=&gt; <span class="number">2</span>, <span class="number">3 </span>=&gt; <span class="number">3</span>];
        <span class="macro">assert_eq!</span>(<span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span><span class="number">2</span>), <span class="macro">get_in!</span>(ordmap, <span class="kw-2">&amp;</span><span class="number">2</span>));

        <span class="kw">let </span>vecs = <span class="macro">vector!</span>[<span class="macro">vector!</span>[<span class="number">1</span>, <span class="number">2</span>, <span class="number">3</span>], <span class="macro">vector!</span>[<span class="number">4</span>, <span class="number">5</span>, <span class="number">6</span>], <span class="macro">vector!</span>[<span class="number">7</span>, <span class="number">8</span>, <span class="number">9</span>]];
        <span class="macro">assert_eq!</span>(<span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span><span class="number">6</span>), <span class="macro">get_in!</span>(vecs, <span class="number">1 </span>=&gt; <span class="number">2</span>));
    }
}
</code></pre></div></section></main></body></html>