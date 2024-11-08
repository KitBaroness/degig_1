<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `programs/mmoshforge/src/curve/precise_number.rs`."><title>precise_number.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="mmoshforge" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../mmoshforge/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Defines PreciseNumber, a U192 wrapper with float-like operations
</span><span class="comment">// Stolen from SPL math, but changing inner unit

</span><span class="kw">use </span>std::convert::<span class="kw-2">*</span>;

<span class="kw">use </span>anchor_lang::prelude::msg;

<span class="kw">use </span><span class="kw">crate</span>::curve::signed_precise_number::SignedPreciseNumber;
<span class="kw">use </span><span class="kw">crate</span>::curve::curveuint::U192;

<span class="comment">// Allows for easy swapping between different internal representations
</span><span class="kw">pub type </span>InnerUint = U192; 

<span class="kw">pub static </span>ONE_PREC: PreciseNumber = PreciseNumber { value: one() };
<span class="kw">pub static </span>ZERO_PREC: PreciseNumber = PreciseNumber { value: zero() };
<span class="kw">pub static </span>TWO_PREC: PreciseNumber = PreciseNumber { value: two() };

<span class="doccomment">/// The representation of the number one as a precise number as 10^18
</span><span class="kw">pub const </span>ONE: u128 = <span class="number">1_000_000_000_000_000_000</span>;

<span class="doccomment">/// Struct encapsulating a fixed-point number that allows for decimal calculations
</span><span class="attr">#[derive(Clone, Debug, PartialEq)]
</span><span class="kw">pub struct </span>PreciseNumber {
  <span class="doccomment">/// Wrapper over the inner value, which is multiplied by ONE
  </span><span class="kw">pub </span>value: InnerUint,
}

<span class="doccomment">/// The precise-number 1 as a InnerUint. 24 decimals of precision
</span><span class="attr">#[inline]
</span><span class="kw">pub const fn </span>one() -&gt; InnerUint {
  <span class="comment">// InnerUint::from(1_000_000_000_000_000_000_000_000_u128)
  </span>U192([<span class="number">1000000000000000000_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
  <span class="comment">// InnerUint::from(ONE)
</span>}

<span class="attr">#[inline]
</span><span class="kw">pub const fn </span>two() -&gt; InnerUint {
  <span class="comment">// InnerUint::from(1_000_000_000_000_000_000_000_000_u128)
  </span>U192([<span class="number">2000000000000000000_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
  <span class="comment">// InnerUint::from(ONE)
</span>}

<span class="comment">// 0.693147180369123816490000
</span><span class="attr">#[inline]
</span><span class="kw">pub const fn </span>ln2hi() -&gt; InnerUint {
  U192([<span class="number">13974485815783726801_u64</span>, <span class="number">3_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>LN2HI: PreciseNumber = PreciseNumber { value: ln2hi() };
<span class="attr">#[inline]

</span><span class="kw">pub const fn </span>ln2hi_scale() -&gt; InnerUint {
  U192([<span class="number">7766279631452241920_u64</span>, <span class="number">5_u64</span>, <span class="number">0_u64</span>])
}

<span class="kw">pub const </span>LN2HI_SCALE: PreciseNumber = PreciseNumber {
  value: ln2hi_scale(),
};

<span class="comment">// 1.90821492927058770002e-10 /* 3dea39ef 35793c76 */
// Note that ln2lo is lower than our max precision, so we store both it and the thirty zeroes to scale by
</span><span class="attr">#[inline]
</span><span class="kw">pub const fn </span>ln2lo() -&gt; InnerUint {
  U192([<span class="number">3405790746697269248_u64</span>, <span class="number">1034445385942222_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>LN2LO: PreciseNumber = PreciseNumber { value: ln2lo() };

<span class="attr">#[inline]
</span><span class="kw">pub const fn </span>ln2lo_scale() -&gt; InnerUint {
  U192([<span class="number">80237960548581376_u64</span>, <span class="number">10841254275107988496_u64</span>, <span class="number">293873_u64</span>])
}

<span class="kw">pub const </span>LN2LO_SCALE: PreciseNumber = PreciseNumber {
  value: ln2lo_scale(),
};

<span class="comment">// 6.666666666666735130e-01
</span><span class="attr">#[inline]
</span><span class="kw">pub const fn </span>l1() -&gt; InnerUint {
  U192([<span class="number">666666666666673513_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>L1: PreciseNumber = PreciseNumber { value: l1() };

<span class="attr">#[inline]
</span><span class="kw">pub const fn </span>l2() -&gt; InnerUint {
  U192([<span class="number">399999999994094190_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>L2: PreciseNumber = PreciseNumber { value: l2() };

<span class="attr">#[inline]
</span><span class="kw">pub const fn </span>l3() -&gt; InnerUint {
  U192([<span class="number">285714287436623914_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>L3: PreciseNumber = PreciseNumber { value: l3() };

<span class="attr">#[inline]
</span><span class="kw">pub const fn </span>l4() -&gt; InnerUint {
  U192([<span class="number">222221984321497839_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>L4: PreciseNumber = PreciseNumber { value: l4() };

<span class="attr">#[inline]
</span><span class="kw">pub const fn </span>l5() -&gt; InnerUint {
  U192([<span class="number">181835721616180501_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>L5: PreciseNumber = PreciseNumber { value: l5() };

<span class="kw">pub const fn </span>l6() -&gt; InnerUint {
  U192([<span class="number">153138376992093733_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>L6: PreciseNumber = PreciseNumber { value: l6() };

<span class="attr">#[inline]
</span><span class="kw">pub const fn </span>l7() -&gt; InnerUint {
  U192([<span class="number">147981986051165859_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>L7: PreciseNumber = PreciseNumber { value: l7() };

<span class="attr">#[inline]
</span><span class="kw">pub const fn </span>sqrt2overtwo() -&gt; InnerUint {
  U192([<span class="number">707106781186547600_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>SQRT2OVERTWO: PreciseNumber = PreciseNumber {
  value: sqrt2overtwo(),
};

<span class="attr">#[inline]
</span><span class="kw">pub const fn </span>half() -&gt; InnerUint {
  U192([<span class="number">500000000000000000_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
}
<span class="kw">pub const </span>HALF: PreciseNumber = PreciseNumber { value: half() };

<span class="doccomment">/// The number 0 as a PreciseNumber, used for easier calculations.
</span><span class="attr">#[inline]
</span><span class="kw">pub const fn </span>zero() -&gt; InnerUint {
  U192([<span class="number">0_u64</span>, <span class="number">0_u64</span>, <span class="number">0_u64</span>])
}

<span class="kw">impl </span>PreciseNumber {
  <span class="kw">pub fn </span>signed(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; SignedPreciseNumber {
    SignedPreciseNumber {
      value: <span class="self">self</span>.clone(),
      is_negative: <span class="bool-val">false</span>,
    }
  }

  <span class="doccomment">/// Correction to apply to avoid truncation errors on division.  Since
  /// integer operations will always floor the result, we artifically bump it
  /// up by one half to get the expect result.
  </span><span class="kw">fn </span>rounding_correction() -&gt; InnerUint {
    InnerUint::from(ONE / <span class="number">2</span>)
  }

  <span class="kw">pub fn </span>zero() -&gt; <span class="self">Self </span>{
    <span class="self">Self </span>{ value: zero() }
  }

  <span class="kw">pub fn </span>one() -&gt; <span class="self">Self </span>{
    <span class="self">Self </span>{ value: one() }
  }

  <span class="doccomment">/// Create a precise number from an imprecise u128, should always succeed
  </span><span class="kw">pub fn </span>new(value: u128) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
    <span class="kw">let </span>value = InnerUint::from(value).checked_mul(one())<span class="question-mark">?</span>;
    <span class="prelude-val">Some</span>(<span class="self">Self </span>{ value })
  }

  <span class="doccomment">/// Convert a precise number back to u128
  </span><span class="kw">pub fn </span>to_imprecise(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;u128&gt; {
    <span class="self">self
      </span>.value
      .checked_add(<span class="self">Self</span>::rounding_correction())<span class="question-mark">?
      </span>.checked_div(one())
      .map(|v| v.as_u128())
  }

  <span class="doccomment">/// Checks that two PreciseNumbers are equal within some tolerance
  </span><span class="kw">pub fn </span>almost_eq(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: <span class="kw-2">&amp;</span><span class="self">Self</span>, precision: InnerUint) -&gt; bool {
    <span class="kw">let </span>(difference, <span class="kw">_</span>) = <span class="self">self</span>.unsigned_sub(rhs);
    difference.value &lt; precision
  }

  <span class="doccomment">/// Checks that a number is less than another
  </span><span class="kw">pub fn </span>less_than(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; bool {
    <span class="self">self</span>.value &lt; rhs.value
  }

  <span class="doccomment">/// Checks that a number is greater than another
  </span><span class="kw">pub fn </span>greater_than(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; bool {
    <span class="self">self</span>.value &gt; rhs.value
  }

  <span class="doccomment">/// Checks that a number is less than another
  </span><span class="kw">pub fn </span>less_than_or_equal(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; bool {
    <span class="self">self</span>.value &lt;= rhs.value
  }

  <span class="doccomment">/// Checks that a number is greater than another
  </span><span class="kw">pub fn </span>greater_than_or_equal(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; bool {
    <span class="self">self</span>.value &gt;= rhs.value
  }

  <span class="doccomment">/// Floors a precise value to a precision of ONE
  </span><span class="kw">pub fn </span>floor(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
    <span class="kw">let </span>value = <span class="self">self</span>.value.checked_div(one())<span class="question-mark">?</span>.checked_mul(one())<span class="question-mark">?</span>;
    <span class="prelude-val">Some</span>(<span class="self">Self </span>{ value })
  }

  <span class="doccomment">/// Ceiling a precise value to a precision of ONE
  </span><span class="kw">pub fn </span>ceiling(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
    <span class="kw">let </span>value = <span class="self">self
      </span>.value
      .checked_add(one().checked_sub(InnerUint::from(<span class="number">1</span>))<span class="question-mark">?</span>)<span class="question-mark">?
      </span>.checked_div(one())<span class="question-mark">?
      </span>.checked_mul(one())<span class="question-mark">?</span>;
    <span class="prelude-val">Some</span>(<span class="self">Self </span>{ value })
  }

  <span class="doccomment">/// Performs a checked division on two precise numbers
  </span><span class="kw">pub fn </span>checked_div(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
    <span class="kw">if </span><span class="kw-2">*</span>rhs == <span class="self">Self</span>::zero() {
      <span class="kw">return </span><span class="prelude-val">None</span>;
    }
    <span class="kw">match </span><span class="self">self</span>.value.checked_mul(one()) {
      <span class="prelude-val">Some</span>(v) =&gt; {
        <span class="kw">let </span>value = v
          .checked_add(<span class="self">Self</span>::rounding_correction())<span class="question-mark">?
          </span>.checked_div(rhs.value)<span class="question-mark">?</span>;
        <span class="prelude-val">Some</span>(<span class="self">Self </span>{ value })
      }
      <span class="prelude-val">None </span>=&gt; {
        <span class="kw">let </span>value = <span class="self">self
          </span>.value
          .checked_add(<span class="self">Self</span>::rounding_correction())<span class="question-mark">?
          </span>.checked_div(rhs.value)<span class="question-mark">?
          </span>.checked_mul(one())<span class="question-mark">?</span>;
        <span class="prelude-val">Some</span>(<span class="self">Self </span>{ value })
      }
    }
  }

  <span class="doccomment">/// Performs a multiplication on two precise numbers
  </span><span class="kw">pub fn </span>checked_mul(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
    <span class="kw">match </span><span class="self">self</span>.value.checked_mul(rhs.value) {
      <span class="prelude-val">Some</span>(v) =&gt; {
        <span class="kw">let </span>value = v
          .checked_add(<span class="self">Self</span>::rounding_correction())<span class="question-mark">?
          </span>.checked_div(one())<span class="question-mark">?</span>;
        <span class="prelude-val">Some</span>(<span class="self">Self </span>{ value })
      }
      <span class="prelude-val">None </span>=&gt; {
        <span class="kw">let </span>value = <span class="kw">if </span><span class="self">self</span>.value &gt;= rhs.value {
          <span class="self">self</span>.value.checked_div(one())<span class="question-mark">?</span>.checked_mul(rhs.value)<span class="question-mark">?
        </span>} <span class="kw">else </span>{
          rhs.value.checked_div(one())<span class="question-mark">?</span>.checked_mul(<span class="self">self</span>.value)<span class="question-mark">?
        </span>};
        <span class="prelude-val">Some</span>(<span class="self">Self </span>{ value })
      }
    }
  }

  <span class="doccomment">/// Performs addition of two precise numbers
  </span><span class="kw">pub fn </span>checked_add(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
    <span class="kw">let </span>value = <span class="self">self</span>.value.checked_add(rhs.value)<span class="question-mark">?</span>;
    <span class="prelude-val">Some</span>(<span class="self">Self </span>{ value })
  }

  <span class="doccomment">/// Subtracts the argument from self
  </span><span class="kw">pub fn </span>checked_sub(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
    <span class="kw">let </span>value = <span class="self">self</span>.value.checked_sub(rhs.value)<span class="question-mark">?</span>;
    <span class="prelude-val">Some</span>(<span class="self">Self </span>{ value })
  }

  <span class="kw">pub fn </span>unsigned_sub(<span class="kw-2">&amp;</span><span class="self">self</span>, rhs: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; (<span class="self">Self</span>, bool) {
    <span class="kw">match </span><span class="self">self</span>.value.checked_sub(rhs.value) {
      <span class="prelude-val">None </span>=&gt; {
        <span class="kw">let </span>value = rhs.value.checked_sub(<span class="self">self</span>.value).unwrap();
        (<span class="self">Self </span>{ value }, <span class="bool-val">true</span>)
      }
      <span class="prelude-val">Some</span>(value) =&gt; (<span class="self">Self </span>{ value }, <span class="bool-val">false</span>),
    }
  }

  <span class="comment">// Frexp breaks f into a normalized fraction
  // and an integral power of two.
  // It returns frac and exp satisfying f == frac × 2**exp,
  // with the absolute value of frac in the interval [½, 1).
  //
  // Special cases are:
  //	Frexp(±0) = ±0, 0
  //	Frexp(±Inf) = ±Inf, 0
  //	Frexp(NaN) = NaN, 0
  </span><span class="kw">fn </span>frexp(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;(<span class="self">Self</span>, i64)&gt; {
    <span class="kw">if </span><span class="self">self</span>.eq(<span class="kw-2">&amp;</span>ZERO_PREC) {
      <span class="prelude-val">Some</span>((ZERO_PREC.clone(), <span class="number">0</span>))
    } <span class="kw">else if </span><span class="self">self</span>.less_than(<span class="kw-2">&amp;</span>ONE_PREC) {
      <span class="kw">let </span>first_leading = <span class="self">self</span>.value.<span class="number">0</span>[<span class="number">0</span>].leading_zeros();
      <span class="kw">let </span>one_leading = ONE_PREC.value.<span class="number">0</span>[<span class="number">0</span>].leading_zeros();
      <span class="kw">let </span>bits = i64::from(first_leading.checked_sub(one_leading).unwrap());
      <span class="kw">let </span>frac = PreciseNumber {
        value: <span class="self">self</span>.value &lt;&lt; bits,
      };
      <span class="kw">if </span>frac.less_than(<span class="kw-2">&amp;</span>HALF) {
        <span class="prelude-val">Some</span>((frac.checked_mul(<span class="kw-2">&amp;</span>TWO_PREC).unwrap(), -bits - <span class="number">1</span>))
      } <span class="kw">else </span>{
        <span class="prelude-val">Some</span>((frac, -bits))
      }
    } <span class="kw">else </span>{
      <span class="kw">let </span>bits = <span class="number">128_i64</span>.checked_sub(i64::from(<span class="self">self</span>.to_imprecise()<span class="question-mark">?</span>.leading_zeros()))<span class="question-mark">?</span>;
      <span class="kw">let </span>frac = PreciseNumber {
        value: <span class="self">self</span>.value &gt;&gt; bits,
      };
      <span class="kw">if </span>frac.less_than(<span class="kw-2">&amp;</span>HALF) {
        <span class="prelude-val">Some</span>((frac.checked_mul(<span class="kw-2">&amp;</span>TWO_PREC).unwrap(), bits - <span class="number">1</span>))
      } <span class="kw">else </span>{
        <span class="prelude-val">Some</span>((frac, bits))
      }
    }
  }

  <span class="comment">// Modified from the original to support precise numbers instead of floats
  /*
    Floating-point logarithm.
    Borrowed from https://arm-software.github.io/golang-utils/src/math/log.go.html
  */
  // The original C code, the long comment, and the constants
  // below are from FreeBSD's /usr/src/lib/msun/src/e_log.c
  // and came with this notice. The go code is a simpler
  // version of the original C.
  //
  // ====================================================
  // Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
  //
  // Developed at SunPro, a Sun Microsystems, Inc. business.
  // Permission to use, copy, modify, and distribute this
  // software is freely granted, provided that this notice
  // is preserved.
  // ====================================================
  //
  // __ieee754_log(x)
  // Return the logarithm of x
  //
  // Method :
  //   1. Argument Reduction: find k and f such that
  //			x = 2**k * (1+f),
  //	   where  sqrt(2)/2 &lt; 1+f &lt; sqrt(2) .
  //
  //   2. Approximation of log(1+f).
  //	Let s = f/(2+f) ; based on log(1+f) = log(1+s) - log(1-s)
  //		 = 2s + 2/3 s**3 + 2/5 s**5 + .....,
  //	     	 = 2s + s*R
  //      We use a special Reme algorithm on [0,0.1716] to generate
  //	a polynomial of degree 14 to approximate R.  The maximum error
  //	of this polynomial approximation is bounded by 2**-58.45. In
  //	other words,
  //		        2      4      6      8      10      12      14
  //	    R(z) ~ L1*s +L2*s +L3*s +L4*s +L5*s  +L6*s  +L7*s
  //	(the values of L1 to L7 are listed in the program) and
  //	    |      2          14          |     -58.45
  //	    | L1*s +...+L7*s    -  R(z) | &lt;= 2
  //	    |                             |
  //	Note that 2s = f - s*f = f - hfsq + s*hfsq, where hfsq = f*f/2.
  //	In order to guarantee error in log below 1ulp, we compute log by
  //		log(1+f) = f - s*(f - R)		(if f is not too large)
  //		log(1+f) = f - (hfsq - s*(hfsq+R)).	(better accuracy)
  //
  //	3. Finally,  log(x) = k*Ln2 + log(1+f).
  //			    = k*Ln2_hi+(f-(hfsq-(s*(hfsq+R)+k*Ln2_lo)))
  //	   Here Ln2 is split into two floating point number:
  //			Ln2_hi + Ln2_lo,
  //	   where n*Ln2_hi is always exact for |n| &lt; 2000.
  //
  // Special cases:
  //	log(x) is NaN with signal if x &lt; 0 (including -INF) ;
  //	log(+INF) is +INF; log(0) is -INF with signal;
  //	log(NaN) is that NaN with no signal.
  //
  // Accuracy:
  //	according to an error analysis, the error is always less than
  //	1 ulp (unit in the last place).
  //
  // Constants:
  // The hexadecimal values are the intended ones for the following
  // constants. The decimal values may be used, provided that the
  // compiler will convert from decimal to binary accurately enough
  // to produce the hexadecimal values shown.
  // Frexp breaks f into a normalized fraction
  // and an integral power of two.
  // It returns frac and exp satisfying f == frac × 2**exp,
  // with the absolute value of frac in the interval [½, 1).
  //
  // Log returns the natural logarithm of x.
  //
  // Special cases are:
  //	Log(+Inf) = +Inf
  //	Log(0) = -Inf
  //	Log(x &lt; 0) = NaN
  </span><span class="kw">pub fn </span>log(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;SignedPreciseNumber&gt; {
    <span class="kw">if </span><span class="self">self</span>.eq(<span class="kw-2">&amp;</span>ZERO_PREC) {
      <span class="kw">return </span><span class="prelude-val">None</span>;
    }

    <span class="kw">if </span><span class="self">self</span>.eq(<span class="kw-2">&amp;</span>ONE_PREC) {
      <span class="kw">return </span><span class="prelude-val">Some</span>(SignedPreciseNumber {
        value: ZERO_PREC.clone(),
        is_negative: <span class="bool-val">false</span>,
      });
    }

    <span class="kw">let </span>(f1_init, ki_init) = <span class="self">self</span>.frexp()<span class="question-mark">?</span>;

    <span class="kw">let </span>(f1, ki) = <span class="kw">if </span>f1_init.less_than(<span class="kw-2">&amp;</span>SQRT2OVERTWO) {
      <span class="kw">let </span>new_f1 = f1_init.checked_mul(<span class="kw-2">&amp;</span>TWO_PREC)<span class="question-mark">?</span>;
      <span class="kw">let </span>new_k1 = ki_init.checked_sub(<span class="number">1</span>)<span class="question-mark">?</span>;
      (new_f1, new_k1)
    } <span class="kw">else </span>{
      (f1_init, ki_init)
    };

    <span class="kw">let </span>f = f1.signed().checked_sub(<span class="kw-2">&amp;</span>PreciseNumber::one().signed())<span class="question-mark">?</span>;

    <span class="kw">let </span>s_divisor = PreciseNumber { value: two() }.signed().checked_add(<span class="kw-2">&amp;</span>f)<span class="question-mark">?</span>;
    <span class="kw">let </span>s = <span class="kw-2">&amp;</span>f.checked_div(<span class="kw-2">&amp;</span>s_divisor)<span class="question-mark">?</span>;
    <span class="kw">let </span>s2 = s.checked_mul(s)<span class="question-mark">?</span>.value;
    <span class="kw">let </span>s4 = s2.checked_mul(<span class="kw-2">&amp;</span>s2)<span class="question-mark">?</span>;
    <span class="comment">// s2 * (L1 + s4*(L3+s4*(L5+s4*L7)))
    </span><span class="kw">let </span>t1 = s2.checked_mul(<span class="kw-2">&amp;</span>L1.checked_add(<span class="kw-2">&amp;</span>s4.checked_mul(
      <span class="kw-2">&amp;</span>L3.checked_add(<span class="kw-2">&amp;</span>s4.checked_mul(<span class="kw-2">&amp;</span>L5.checked_add(<span class="kw-2">&amp;</span>s4.checked_mul(<span class="kw-2">&amp;</span>L7)<span class="question-mark">?</span>)<span class="question-mark">?</span>)<span class="question-mark">?</span>)<span class="question-mark">?</span>,
    )<span class="question-mark">?</span>)<span class="question-mark">?</span>)<span class="question-mark">?</span>;

    <span class="comment">// s4 * (L2 + s4*(L4+s4*L6))
    </span><span class="kw">let </span>t2 =
      s4.checked_mul(<span class="kw-2">&amp;</span>L2.checked_add(<span class="kw-2">&amp;</span>s4.checked_mul(<span class="kw-2">&amp;</span>L4.checked_add(<span class="kw-2">&amp;</span>s4.checked_mul(<span class="kw-2">&amp;</span>L6)<span class="question-mark">?</span>)<span class="question-mark">?</span>)<span class="question-mark">?</span>)<span class="question-mark">?</span>)<span class="question-mark">?</span>;

    <span class="kw">let </span>r = t1.checked_add(<span class="kw-2">&amp;</span>t2)<span class="question-mark">?</span>;
    <span class="kw">let </span>hfsq = f
      .checked_mul(<span class="kw-2">&amp;</span>f)<span class="question-mark">?
      </span>.checked_div(<span class="kw-2">&amp;</span>PreciseNumber { value: two() }.signed())<span class="question-mark">?</span>;
    <span class="kw">let </span>k = SignedPreciseNumber {
      value: PreciseNumber::new(u128::try_from(ki.abs()).ok()<span class="question-mark">?</span>)<span class="question-mark">?</span>,
      is_negative: ki &lt; <span class="number">0</span>,
    };

    <span class="comment">// k*Ln2Hi - ((hfsq - (s*(hfsq+R) + k*Ln2Lo)) - f)
    </span><span class="kw">let </span>kl2hi = k
      .checked_mul(<span class="kw-2">&amp;</span>LN2HI.signed())<span class="question-mark">?
      </span>.checked_div(<span class="kw-2">&amp;</span>LN2HI_SCALE.signed())<span class="question-mark">?</span>;
    <span class="kw">let </span>shfsqr = s.checked_mul(<span class="kw-2">&amp;</span>hfsq.checked_add(<span class="kw-2">&amp;</span>r.signed())<span class="question-mark">?</span>)<span class="question-mark">?</span>;
    <span class="kw">let </span>kl2lo = k
      .checked_mul(<span class="kw-2">&amp;</span>LN2LO.signed())<span class="question-mark">?
      </span>.checked_div(<span class="kw-2">&amp;</span>LN2LO_SCALE.signed())<span class="question-mark">?</span>;

    <span class="kw">let </span>shfsqr_kl2lo = shfsqr.checked_add(<span class="kw-2">&amp;</span>kl2lo)<span class="question-mark">?</span>;
    <span class="kw">let </span>hfsq_shfsqr_kl2lo = hfsq.checked_sub(<span class="kw-2">&amp;</span>shfsqr_kl2lo)<span class="question-mark">?</span>;
    <span class="kw">let </span>f_hfsq_shfsqr_kl2lo = hfsq_shfsqr_kl2lo.checked_sub(<span class="kw-2">&amp;</span>f)<span class="question-mark">?</span>;

    kl2hi.checked_sub(<span class="kw-2">&amp;</span>f_hfsq_shfsqr_kl2lo)
  }

  <span class="comment">/*
  b = pow/frac
  y = a^b
  ln (y) = bln (a)
  y = e^(b ln (a))
  */
  </span><span class="kw">pub fn </span>pow(<span class="kw-2">&amp;</span><span class="self">self</span>, exp: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
    <span class="kw">if </span><span class="self">self</span>.eq(<span class="kw-2">&amp;</span>ZERO_PREC) {
      <span class="kw">return </span><span class="prelude-val">Some</span>(ZERO_PREC.clone());
    }

    <span class="kw">let </span>lg = <span class="self">self</span>.log()<span class="question-mark">?</span>;
    <span class="kw">let </span>x = exp.signed().checked_mul(<span class="kw-2">&amp;</span>lg)<span class="question-mark">?</span>;
    x.exp()
  }

  <span class="kw">pub fn </span>print(<span class="kw-2">&amp;</span><span class="self">self</span>) {
    <span class="kw">let </span>whole = <span class="self">self</span>.floor().unwrap().to_imprecise().unwrap();
    <span class="kw">let </span>decimals = <span class="self">self
      </span>.checked_sub(<span class="kw-2">&amp;</span>PreciseNumber::new(whole).unwrap())
      .unwrap()
      .checked_mul(<span class="kw-2">&amp;</span>PreciseNumber::new(ONE).unwrap())
      .unwrap()
      .to_imprecise()
      .unwrap();
    <span class="macro">msg!</span>(<span class="string">"{}.{:0&gt;width$}"</span>, whole, decimals, width = <span class="number">18</span>);
  }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
  <span class="kw">use super</span>::<span class="kw-2">*</span>;

  <span class="attr">#[test]
  </span><span class="kw">fn </span>test_pow() {
    <span class="kw">let </span>precision = InnerUint::from(<span class="number">5_000_000_000_000_u128</span>); <span class="comment">// correct to at least 12 decimal places
    </span><span class="kw">let </span>test = PreciseNumber::new(<span class="number">8</span>).unwrap();
    <span class="kw">let </span>sqrt = test.pow(<span class="kw-2">&amp;</span>HALF).unwrap();
    <span class="kw">let </span>expected = PreciseNumber::new(<span class="number">28284271247461903</span>)
      .unwrap()
      .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">10000000000000000</span>).unwrap())
      .unwrap();
    <span class="macro">assert!</span>(sqrt.almost_eq(<span class="kw-2">&amp;</span>expected, precision));

    <span class="kw">let </span>test2 = PreciseNumber::new(<span class="number">55</span>)
      .unwrap()
      .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">100</span>).unwrap())
      .unwrap();
    <span class="kw">let </span>squared = test2.pow(<span class="kw-2">&amp;</span>TWO_PREC).unwrap();
    <span class="kw">let </span>expected = PreciseNumber::new(<span class="number">3025</span>)
      .unwrap()
      .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">10000</span>).unwrap())
      .unwrap();
    <span class="macro">assert!</span>(squared.almost_eq(<span class="kw-2">&amp;</span>expected, precision));
  }

  <span class="attr">#[test]
  </span><span class="kw">fn </span>test_log() {
    <span class="kw">let </span>precision = InnerUint::from(<span class="number">5_000_000_000_u128</span>); <span class="comment">// correct to at least 9 decimal places

    </span><span class="kw">let </span>test = PreciseNumber::new(<span class="number">9</span>).unwrap();
    <span class="kw">let </span>log = test.log().unwrap().value;
    <span class="kw">let </span>expected = PreciseNumber::new(<span class="number">21972245773362196</span>)
      .unwrap()
      .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">10000000000000000</span>).unwrap())
      .unwrap();
    <span class="macro">assert!</span>(log.almost_eq(<span class="kw-2">&amp;</span>expected, precision));

    <span class="kw">let </span>test2 = PreciseNumber::new(<span class="number">2</span>).unwrap();
    <span class="macro">assert!</span>(test2.log().unwrap().value.almost_eq(
      <span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">6931471805599453</span>)
        .unwrap()
        .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">10000000000000000</span>).unwrap())
        .unwrap(),
      precision
    ));

    <span class="kw">let </span>test3 = <span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">12</span>)
      .unwrap()
      .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">10</span>).unwrap())
      .unwrap();
    <span class="macro">assert!</span>(test3.log().unwrap().value.almost_eq(
      <span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">1823215567939546</span>)
        .unwrap()
        .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">10000000000000000</span>).unwrap())
        .unwrap(),
      precision
    ));

    <span class="kw">let </span>test5 = <span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">15</span>)
      .unwrap()
      .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">10</span>).unwrap())
      .unwrap();
    <span class="macro">assert!</span>(test5.log().unwrap().value.almost_eq(
      <span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">4054651081081644</span>)
        .unwrap()
        .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">10000000000000000</span>).unwrap())
        .unwrap(),
      precision
    ));

    <span class="kw">let </span>test6 = PreciseNumber::new(<span class="number">4</span>)
      .unwrap()
      .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">1000000</span>).unwrap())
      .unwrap();
    <span class="macro">assert!</span>(test6.log().unwrap().value.almost_eq(
      <span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">12429216196844383</span>)
        .unwrap()
        .checked_div(<span class="kw-2">&amp;</span>PreciseNumber::new(<span class="number">1000000000000000</span>).unwrap())
        .unwrap(),
      precision
    ));
  }

  <span class="attr">#[test]
  </span><span class="kw">fn </span>test_floor() {
    <span class="kw">let </span>whole_number = PreciseNumber::new(<span class="number">2</span>).unwrap();
    <span class="kw">let </span><span class="kw-2">mut </span>decimal_number = PreciseNumber::new(<span class="number">2</span>).unwrap();
    decimal_number.value += InnerUint::from(<span class="number">1</span>);
    <span class="kw">let </span>floor = decimal_number.floor().unwrap();
    <span class="kw">let </span>floor_again = floor.floor().unwrap();
    <span class="macro">assert_eq!</span>(whole_number.value, floor.value);
    <span class="macro">assert_eq!</span>(whole_number.value, floor_again.value);
  }

  <span class="attr">#[test]
  </span><span class="kw">fn </span>test_ceiling() {
    <span class="kw">let </span>whole_number = PreciseNumber::new(<span class="number">2</span>).unwrap();
    <span class="kw">let </span><span class="kw-2">mut </span>decimal_number = PreciseNumber::new(<span class="number">2</span>).unwrap();
    decimal_number.value -= InnerUint::from(<span class="number">1</span>);
    <span class="kw">let </span>ceiling = decimal_number.ceiling().unwrap();
    <span class="kw">let </span>ceiling_again = ceiling.ceiling().unwrap();
    <span class="macro">assert_eq!</span>(whole_number.value, ceiling.value);
    <span class="macro">assert_eq!</span>(whole_number.value, ceiling_again.value);
  }

  <span class="comment">// // Keep around for testing. Can drop a debugger and find out the binary for the inner unit
  // #[test]
  // fn get_constants() {
  //   let one = PreciseNumber { value: InnerUint::from(ONE) };
  //   let two = PreciseNumber::new(2).unwrap();
  //   let sqrt_two_over_two = PreciseNumber::new(7071067811865476).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000).unwrap()
  //   ).unwrap();
  //   // Purposefully take 2 decimals off of ln2hi to keep precision
  //   let ln2hi = PreciseNumber::new(693147180369123816490_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(1_000_000_000_000_000_000_0).unwrap()
  //   ).unwrap();
  //   let ln2scale = &amp;PreciseNumber::new(100).unwrap();
  //   let extraprecisescale = &amp;PreciseNumber::new(1000000000).unwrap();

  //   let ln2lo_scale = &amp;PreciseNumber::new(100000000000000000000000000).unwrap();
  //   let ln2lo = PreciseNumber::new(19082149292705877_u128).unwrap();
  //   let l1 = PreciseNumber::new(6666666666666735130_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000000).unwrap()
  //   ).unwrap();
  //   let l2 = PreciseNumber::new(3999999999940941908_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000000).unwrap()
  //   ).unwrap();
  //   let l3 = PreciseNumber::new(2857142874366239149_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000000).unwrap()
  //   ).unwrap();
  //   let l4 = PreciseNumber::new(2222219843214978396_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000000).unwrap()
  //   ).unwrap();
  //   let l5 = PreciseNumber::new(1818357216161805012_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000000).unwrap()
  //   ).unwrap();
  //   let l6 = PreciseNumber::new(1531383769920937332_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000000).unwrap()
  //   ).unwrap();
  //   let l7 = PreciseNumber::new(1479819860511658591_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000000).unwrap()
  //   ).unwrap();

  //   let invln2 = PreciseNumber::new(144269504088896338700_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(100000000000000000000).unwrap()
  //   ).unwrap();

  //   let halfln2 = PreciseNumber::new(34657359027997264_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(100000000000000000).unwrap()
  //   ).unwrap();

  //   let threehalfln2 = PreciseNumber::new(10397207708399179_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000).unwrap()
  //   ).unwrap();

  //   let half = PreciseNumber::new(5_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10).unwrap()
  //   ).unwrap();

  //   let p1 = PreciseNumber::new(166666666666666019037_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(1000000000000000000000).unwrap()
  //   ).unwrap();

  //   let p2 = PreciseNumber::new(277777777770155933842_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(100000000000000000000000).unwrap()
  //   ).unwrap();

  //   let p3 = PreciseNumber::new(661375632143793436117_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000000000000).unwrap()
  //   ).unwrap();

  //   let p4 = PreciseNumber::new(165339022054652515390_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(100000000000000000000000000).unwrap()
  //   ).unwrap();

  //   let p5 = PreciseNumber::new(413813679705723846039_u128).unwrap().checked_div(
  //     &amp;PreciseNumber::new(10000000000000000000000000000).unwrap()
  //   ).unwrap();

  //   l1.print();
  //   l2.print();
  //   l3.print();
  //   l4.print();
  //   l5.print();
  //   l6.print();
  //   l7.print();
  //   ln2hi.print();
  //   ln2lo.print();
  //   ln2lo_scale.print();
  //   sqrt_two_over_two.print();

  //   let s = 1;
  // }
</span>}
</code></pre></div></section></main></body></html>