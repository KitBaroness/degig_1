<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rayon-1.10.0/src/slice/quicksort.rs`."><title>quicksort.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="rayon" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../rayon/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Parallel quicksort.
//!
//! This implementation is copied verbatim from `std::slice::sort_unstable` and then parallelized.
//! The only difference from the original is that calls to `recurse` are executed in parallel using
//! `rayon_core::join`.

</span><span class="kw">use </span>std::marker::PhantomData;
<span class="kw">use </span>std::mem::{<span class="self">self</span>, MaybeUninit};
<span class="kw">use </span>std::ptr;

<span class="doccomment">/// When dropped, copies from `src` into `dest`.
</span><span class="attr">#[must_use]
</span><span class="kw">struct </span>CopyOnDrop&lt;<span class="lifetime">'a</span>, T&gt; {
    src: <span class="kw-2">*const </span>T,
    dest: <span class="kw-2">*mut </span>T,
    <span class="doccomment">/// `src` is often a local pointer here, make sure we have appropriate
    /// PhantomData so that dropck can protect us.
    </span>marker: PhantomData&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>T&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, T&gt; CopyOnDrop&lt;<span class="lifetime">'a</span>, T&gt; {
    <span class="doccomment">/// Construct from a source pointer and a destination
    /// Assumes dest lives longer than src, since there is no easy way to
    /// copy down lifetime information from another pointer
    </span><span class="kw">unsafe fn </span>new(src: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>T, dest: <span class="kw-2">*mut </span>T) -&gt; <span class="self">Self </span>{
        CopyOnDrop {
            src,
            dest,
            marker: PhantomData,
        }
    }
}

<span class="kw">impl</span>&lt;T&gt; Drop <span class="kw">for </span>CopyOnDrop&lt;<span class="lifetime">'_</span>, T&gt; {
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="comment">// SAFETY:  This is a helper class.
        //          Please refer to its usage for correctness.
        //          Namely, one must be sure that `src` and `dst` does not overlap as required by `ptr::copy_nonoverlapping`.
        </span><span class="kw">unsafe </span>{
            ptr::copy_nonoverlapping(<span class="self">self</span>.src, <span class="self">self</span>.dest, <span class="number">1</span>);
        }
    }
}

<span class="doccomment">/// Shifts the first element to the right until it encounters a greater or equal element.
</span><span class="kw">fn </span>shift_head&lt;T, F&gt;(v: <span class="kw-2">&amp;mut </span>[T], is_less: <span class="kw-2">&amp;</span>F)
<span class="kw">where
    </span>F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool,
{
    <span class="kw">let </span>len = v.len();
    <span class="comment">// SAFETY: The unsafe operations below involves indexing without a bounds check (by offsetting a
    // pointer) and copying memory (`ptr::copy_nonoverlapping`).
    //
    // a. Indexing:
    //  1. We checked the size of the array to &gt;=2.
    //  2. All the indexing that we will do is always between {0 &lt;= index &lt; len} at most.
    //
    // b. Memory copying
    //  1. We are obtaining pointers to references which are guaranteed to be valid.
    //  2. They cannot overlap because we obtain pointers to difference indices of the slice.
    //     Namely, `i` and `i-1`.
    //  3. If the slice is properly aligned, the elements are properly aligned.
    //     It is the caller's responsibility to make sure the slice is properly aligned.
    //
    // See comments below for further detail.
    </span><span class="kw">unsafe </span>{
        <span class="comment">// If the first two elements are out-of-order...
        </span><span class="kw">if </span>len &gt;= <span class="number">2 </span>&amp;&amp; is_less(v.get_unchecked(<span class="number">1</span>), v.get_unchecked(<span class="number">0</span>)) {
            <span class="comment">// Read the first element into a stack-allocated variable. If a following comparison
            // operation panics, `hole` will get dropped and automatically write the element back
            // into the slice.
            </span><span class="kw">let </span>tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(<span class="number">0</span>)));
            <span class="kw">let </span>v = v.as_mut_ptr();
            <span class="kw">let </span><span class="kw-2">mut </span>hole = CopyOnDrop::new(<span class="kw-2">&amp;*</span>tmp, v.add(<span class="number">1</span>));
            ptr::copy_nonoverlapping(v.add(<span class="number">1</span>), v.add(<span class="number">0</span>), <span class="number">1</span>);

            <span class="kw">for </span>i <span class="kw">in </span><span class="number">2</span>..len {
                <span class="kw">if </span>!is_less(<span class="kw-2">&amp;*</span>v.add(i), <span class="kw-2">&amp;*</span>tmp) {
                    <span class="kw">break</span>;
                }

                <span class="comment">// Move `i`-th element one place to the left, thus shifting the hole to the right.
                </span>ptr::copy_nonoverlapping(v.add(i), v.add(i - <span class="number">1</span>), <span class="number">1</span>);
                hole.dest = v.add(i);
            }
            <span class="comment">// `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        </span>}
    }
}

<span class="doccomment">/// Shifts the last element to the left until it encounters a smaller or equal element.
</span><span class="kw">fn </span>shift_tail&lt;T, F&gt;(v: <span class="kw-2">&amp;mut </span>[T], is_less: <span class="kw-2">&amp;</span>F)
<span class="kw">where
    </span>F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool,
{
    <span class="kw">let </span>len = v.len();
    <span class="comment">// SAFETY: The unsafe operations below involves indexing without a bound check (by offsetting a
    // pointer) and copying memory (`ptr::copy_nonoverlapping`).
    //
    // a. Indexing:
    //  1. We checked the size of the array to &gt;= 2.
    //  2. All the indexing that we will do is always between `0 &lt;= index &lt; len-1` at most.
    //
    // b. Memory copying
    //  1. We are obtaining pointers to references which are guaranteed to be valid.
    //  2. They cannot overlap because we obtain pointers to difference indices of the slice.
    //     Namely, `i` and `i+1`.
    //  3. If the slice is properly aligned, the elements are properly aligned.
    //     It is the caller's responsibility to make sure the slice is properly aligned.
    //
    // See comments below for further detail.
    </span><span class="kw">unsafe </span>{
        <span class="comment">// If the last two elements are out-of-order...
        </span><span class="kw">if </span>len &gt;= <span class="number">2 </span>&amp;&amp; is_less(v.get_unchecked(len - <span class="number">1</span>), v.get_unchecked(len - <span class="number">2</span>)) {
            <span class="comment">// Read the last element into a stack-allocated variable. If a following comparison
            // operation panics, `hole` will get dropped and automatically write the element back
            // into the slice.
            </span><span class="kw">let </span>tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(len - <span class="number">1</span>)));
            <span class="kw">let </span>v = v.as_mut_ptr();
            <span class="kw">let </span><span class="kw-2">mut </span>hole = CopyOnDrop::new(<span class="kw-2">&amp;*</span>tmp, v.add(len - <span class="number">2</span>));
            ptr::copy_nonoverlapping(v.add(len - <span class="number">2</span>), v.add(len - <span class="number">1</span>), <span class="number">1</span>);

            <span class="kw">for </span>i <span class="kw">in </span>(<span class="number">0</span>..len - <span class="number">2</span>).rev() {
                <span class="kw">if </span>!is_less(<span class="kw-2">&amp;*</span>tmp, <span class="kw-2">&amp;*</span>v.add(i)) {
                    <span class="kw">break</span>;
                }

                <span class="comment">// Move `i`-th element one place to the right, thus shifting the hole to the left.
                </span>ptr::copy_nonoverlapping(v.add(i), v.add(i + <span class="number">1</span>), <span class="number">1</span>);
                hole.dest = v.add(i);
            }
            <span class="comment">// `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        </span>}
    }
}

<span class="doccomment">/// Partially sorts a slice by shifting several out-of-order elements around.
///
/// Returns `true` if the slice is sorted at the end. This function is *O*(*n*) worst-case.
</span><span class="attr">#[cold]
</span><span class="kw">fn </span>partial_insertion_sort&lt;T, F&gt;(v: <span class="kw-2">&amp;mut </span>[T], is_less: <span class="kw-2">&amp;</span>F) -&gt; bool
<span class="kw">where
    </span>F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool,
{
    <span class="comment">// Maximum number of adjacent out-of-order pairs that will get shifted.
    </span><span class="kw">const </span>MAX_STEPS: usize = <span class="number">5</span>;
    <span class="comment">// If the slice is shorter than this, don't shift any elements.
    </span><span class="kw">const </span>SHORTEST_SHIFTING: usize = <span class="number">50</span>;

    <span class="kw">let </span>len = v.len();
    <span class="kw">let </span><span class="kw-2">mut </span>i = <span class="number">1</span>;

    <span class="kw">for _ in </span><span class="number">0</span>..MAX_STEPS {
        <span class="comment">// SAFETY: We already explicitly did the bound checking with `i &lt; len`.
        // All our subsequent indexing is only in the range `0 &lt;= index &lt; len`
        </span><span class="kw">unsafe </span>{
            <span class="comment">// Find the next pair of adjacent out-of-order elements.
            </span><span class="kw">while </span>i &lt; len &amp;&amp; !is_less(v.get_unchecked(i), v.get_unchecked(i - <span class="number">1</span>)) {
                i += <span class="number">1</span>;
            }
        }

        <span class="comment">// Are we done?
        </span><span class="kw">if </span>i == len {
            <span class="kw">return </span><span class="bool-val">true</span>;
        }

        <span class="comment">// Don't shift elements on short arrays, that has a performance cost.
        </span><span class="kw">if </span>len &lt; SHORTEST_SHIFTING {
            <span class="kw">return </span><span class="bool-val">false</span>;
        }

        <span class="comment">// Swap the found pair of elements. This puts them in correct order.
        </span>v.swap(i - <span class="number">1</span>, i);

        <span class="comment">// Shift the smaller element to the left.
        </span>shift_tail(<span class="kw-2">&amp;mut </span>v[..i], is_less);
        <span class="comment">// Shift the greater element to the right.
        </span>shift_head(<span class="kw-2">&amp;mut </span>v[i..], is_less);
    }

    <span class="comment">// Didn't manage to sort the slice in the limited number of steps.
    </span><span class="bool-val">false
</span>}

<span class="doccomment">/// Sorts a slice using insertion sort, which is *O*(*n*^2) worst-case.
</span><span class="kw">fn </span>insertion_sort&lt;T, F&gt;(v: <span class="kw-2">&amp;mut </span>[T], is_less: <span class="kw-2">&amp;</span>F)
<span class="kw">where
    </span>F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool,
{
    <span class="kw">for </span>i <span class="kw">in </span><span class="number">1</span>..v.len() {
        shift_tail(<span class="kw-2">&amp;mut </span>v[..i + <span class="number">1</span>], is_less);
    }
}

<span class="doccomment">/// Sorts `v` using heapsort, which guarantees *O*(*n* \* log(*n*)) worst-case.
</span><span class="attr">#[cold]
</span><span class="kw">fn </span>heapsort&lt;T, F&gt;(v: <span class="kw-2">&amp;mut </span>[T], is_less: <span class="kw-2">&amp;</span>F)
<span class="kw">where
    </span>F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool,
{
    <span class="comment">// This binary heap respects the invariant `parent &gt;= child`.
    </span><span class="kw">let </span>sift_down = |v: <span class="kw-2">&amp;mut </span>[T], <span class="kw-2">mut </span>node| {
        <span class="kw">loop </span>{
            <span class="comment">// Children of `node`.
            </span><span class="kw">let </span><span class="kw-2">mut </span>child = <span class="number">2 </span>* node + <span class="number">1</span>;
            <span class="kw">if </span>child &gt;= v.len() {
                <span class="kw">break</span>;
            }

            <span class="comment">// Choose the greater child.
            </span><span class="kw">if </span>child + <span class="number">1 </span>&lt; v.len() &amp;&amp; is_less(<span class="kw-2">&amp;</span>v[child], <span class="kw-2">&amp;</span>v[child + <span class="number">1</span>]) {
                child += <span class="number">1</span>;
            }

            <span class="comment">// Stop if the invariant holds at `node`.
            </span><span class="kw">if </span>!is_less(<span class="kw-2">&amp;</span>v[node], <span class="kw-2">&amp;</span>v[child]) {
                <span class="kw">break</span>;
            }

            <span class="comment">// Swap `node` with the greater child, move one step down, and continue sifting.
            </span>v.swap(node, child);
            node = child;
        }
    };

    <span class="comment">// Build the heap in linear time.
    </span><span class="kw">for </span>i <span class="kw">in </span>(<span class="number">0</span>..v.len() / <span class="number">2</span>).rev() {
        sift_down(v, i);
    }

    <span class="comment">// Pop maximal elements from the heap.
    </span><span class="kw">for </span>i <span class="kw">in </span>(<span class="number">1</span>..v.len()).rev() {
        v.swap(<span class="number">0</span>, i);
        sift_down(<span class="kw-2">&amp;mut </span>v[..i], <span class="number">0</span>);
    }
}

<span class="doccomment">/// Partitions `v` into elements smaller than `pivot`, followed by elements greater than or equal
/// to `pivot`.
///
/// Returns the number of elements smaller than `pivot`.
///
/// Partitioning is performed block-by-block in order to minimize the cost of branching operations.
/// This idea is presented in the [BlockQuicksort][pdf] paper.
///
/// [pdf]: https://drops.dagstuhl.de/opus/volltexte/2016/6389/pdf/LIPIcs-ESA-2016-38.pdf
</span><span class="kw">fn </span>partition_in_blocks&lt;T, F&gt;(v: <span class="kw-2">&amp;mut </span>[T], pivot: <span class="kw-2">&amp;</span>T, is_less: <span class="kw-2">&amp;</span>F) -&gt; usize
<span class="kw">where
    </span>F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool,
{
    <span class="comment">// Number of elements in a typical block.
    </span><span class="kw">const </span>BLOCK: usize = <span class="number">128</span>;

    <span class="comment">// The partitioning algorithm repeats the following steps until completion:
    //
    // 1. Trace a block from the left side to identify elements greater than or equal to the pivot.
    // 2. Trace a block from the right side to identify elements smaller than the pivot.
    // 3. Exchange the identified elements between the left and right side.
    //
    // We keep the following variables for a block of elements:
    //
    // 1. `block` - Number of elements in the block.
    // 2. `start` - Start pointer into the `offsets` array.
    // 3. `end` - End pointer into the `offsets` array.
    // 4. `offsets - Indices of out-of-order elements within the block.

    // The current block on the left side (from `l` to `l.add(block_l)`).
    </span><span class="kw">let </span><span class="kw-2">mut </span>l = v.as_mut_ptr();
    <span class="kw">let </span><span class="kw-2">mut </span>block_l = BLOCK;
    <span class="kw">let </span><span class="kw-2">mut </span>start_l = ptr::null_mut();
    <span class="kw">let </span><span class="kw-2">mut </span>end_l = ptr::null_mut();
    <span class="kw">let </span><span class="kw-2">mut </span>offsets_l = [MaybeUninit::&lt;u8&gt;::uninit(); BLOCK];

    <span class="comment">// The current block on the right side (from `r.sub(block_r)` to `r`).
    // SAFETY: The documentation for .add() specifically mention that `vec.as_ptr().add(vec.len())` is always safe`
    </span><span class="kw">let </span><span class="kw-2">mut </span>r = <span class="kw">unsafe </span>{ l.add(v.len()) };
    <span class="kw">let </span><span class="kw-2">mut </span>block_r = BLOCK;
    <span class="kw">let </span><span class="kw-2">mut </span>start_r = ptr::null_mut();
    <span class="kw">let </span><span class="kw-2">mut </span>end_r = ptr::null_mut();
    <span class="kw">let </span><span class="kw-2">mut </span>offsets_r = [MaybeUninit::&lt;u8&gt;::uninit(); BLOCK];

    <span class="comment">// FIXME: When we get VLAs, try creating one array of length `min(v.len(), 2 * BLOCK)` rather
    // than two fixed-size arrays of length `BLOCK`. VLAs might be more cache-efficient.

    // Returns the number of elements between pointers `l` (inclusive) and `r` (exclusive).
    </span><span class="kw">fn </span>width&lt;T&gt;(l: <span class="kw-2">*mut </span>T, r: <span class="kw-2">*mut </span>T) -&gt; usize {
        <span class="macro">assert!</span>(mem::size_of::&lt;T&gt;() &gt; <span class="number">0</span>);
        <span class="comment">// FIXME: this should *likely* use `offset_from`, but more
        // investigation is needed (including running tests in miri).
        // TODO unstable: (r.addr() - l.addr()) / mem::size_of::&lt;T&gt;()
        </span>(r <span class="kw">as </span>usize - l <span class="kw">as </span>usize) / mem::size_of::&lt;T&gt;()
    }

    <span class="kw">loop </span>{
        <span class="comment">// We are done with partitioning block-by-block when `l` and `r` get very close. Then we do
        // some patch-up work in order to partition the remaining elements in between.
        </span><span class="kw">let </span>is_done = width(l, r) &lt;= <span class="number">2 </span>* BLOCK;

        <span class="kw">if </span>is_done {
            <span class="comment">// Number of remaining elements (still not compared to the pivot).
            </span><span class="kw">let </span><span class="kw-2">mut </span>rem = width(l, r);
            <span class="kw">if </span>start_l &lt; end_l || start_r &lt; end_r {
                rem -= BLOCK;
            }

            <span class="comment">// Adjust block sizes so that the left and right block don't overlap, but get perfectly
            // aligned to cover the whole remaining gap.
            </span><span class="kw">if </span>start_l &lt; end_l {
                block_r = rem;
            } <span class="kw">else if </span>start_r &lt; end_r {
                block_l = rem;
            } <span class="kw">else </span>{
                <span class="comment">// There were the same number of elements to switch on both blocks during the last
                // iteration, so there are no remaining elements on either block. Cover the remaining
                // items with roughly equally-sized blocks.
                </span>block_l = rem / <span class="number">2</span>;
                block_r = rem - block_l;
            }
            <span class="macro">debug_assert!</span>(block_l &lt;= BLOCK &amp;&amp; block_r &lt;= BLOCK);
            <span class="macro">debug_assert!</span>(width(l, r) == block_l + block_r);
        }

        <span class="kw">if </span>start_l == end_l {
            <span class="comment">// Trace `block_l` elements from the left side.
            // TODO unstable: start_l = MaybeUninit::slice_as_mut_ptr(&amp;mut offsets_l);
            </span>start_l = offsets_l.as_mut_ptr() <span class="kw">as </span><span class="kw-2">*mut </span>u8;
            end_l = start_l;
            <span class="kw">let </span><span class="kw-2">mut </span>elem = l;

            <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..block_l {
                <span class="comment">// SAFETY: The unsafety operations below involve the usage of the `offset`.
                //         According to the conditions required by the function, we satisfy them because:
                //         1. `offsets_l` is stack-allocated, and thus considered separate allocated object.
                //         2. The function `is_less` returns a `bool`.
                //            Casting a `bool` will never overflow `isize`.
                //         3. We have guaranteed that `block_l` will be `&lt;= BLOCK`.
                //            Plus, `end_l` was initially set to the begin pointer of `offsets_` which was declared on the stack.
                //            Thus, we know that even in the worst case (all invocations of `is_less` returns false) we will only be at most 1 byte pass the end.
                //        Another unsafety operation here is dereferencing `elem`.
                //        However, `elem` was initially the begin pointer to the slice which is always valid.
                </span><span class="kw">unsafe </span>{
                    <span class="comment">// Branchless comparison.
                    </span><span class="kw-2">*</span>end_l = i <span class="kw">as </span>u8;
                    end_l = end_l.offset(!is_less(<span class="kw-2">&amp;*</span>elem, pivot) <span class="kw">as </span>isize);
                    elem = elem.offset(<span class="number">1</span>);
                }
            }
        }

        <span class="kw">if </span>start_r == end_r {
            <span class="comment">// Trace `block_r` elements from the right side.
            // TODO unstable: start_r = MaybeUninit::slice_as_mut_ptr(&amp;mut offsets_r);
            </span>start_r = offsets_r.as_mut_ptr() <span class="kw">as </span><span class="kw-2">*mut </span>u8;
            end_r = start_r;
            <span class="kw">let </span><span class="kw-2">mut </span>elem = r;

            <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..block_r {
                <span class="comment">// SAFETY: The unsafety operations below involve the usage of the `offset`.
                //         According to the conditions required by the function, we satisfy them because:
                //         1. `offsets_r` is stack-allocated, and thus considered separate allocated object.
                //         2. The function `is_less` returns a `bool`.
                //            Casting a `bool` will never overflow `isize`.
                //         3. We have guaranteed that `block_r` will be `&lt;= BLOCK`.
                //            Plus, `end_r` was initially set to the begin pointer of `offsets_` which was declared on the stack.
                //            Thus, we know that even in the worst case (all invocations of `is_less` returns true) we will only be at most 1 byte pass the end.
                //        Another unsafety operation here is dereferencing `elem`.
                //        However, `elem` was initially `1 * sizeof(T)` past the end and we decrement it by `1 * sizeof(T)` before accessing it.
                //        Plus, `block_r` was asserted to be less than `BLOCK` and `elem` will therefore at most be pointing to the beginning of the slice.
                </span><span class="kw">unsafe </span>{
                    <span class="comment">// Branchless comparison.
                    </span>elem = elem.offset(-<span class="number">1</span>);
                    <span class="kw-2">*</span>end_r = i <span class="kw">as </span>u8;
                    end_r = end_r.offset(is_less(<span class="kw-2">&amp;*</span>elem, pivot) <span class="kw">as </span>isize);
                }
            }
        }

        <span class="comment">// Number of out-of-order elements to swap between the left and right side.
        </span><span class="kw">let </span>count = Ord::min(width(start_l, end_l), width(start_r, end_r));

        <span class="kw">if </span>count &gt; <span class="number">0 </span>{
            <span class="macro">macro_rules!</span> left {
                () =&gt; {
                    l.offset(<span class="kw-2">*</span>start_l <span class="kw">as </span>isize)
                };
            }
            <span class="macro">macro_rules!</span> right {
                () =&gt; {
                    r.offset(-(<span class="kw-2">*</span>start_r <span class="kw">as </span>isize) - <span class="number">1</span>)
                };
            }

            <span class="comment">// Instead of swapping one pair at the time, it is more efficient to perform a cyclic
            // permutation. This is not strictly equivalent to swapping, but produces a similar
            // result using fewer memory operations.

            // SAFETY: The use of `ptr::read` is valid because there is at least one element in
            // both `offsets_l` and `offsets_r`, so `left!` is a valid pointer to read from.
            //
            // The uses of `left!` involve calls to `offset` on `l`, which points to the
            // beginning of `v`. All the offsets pointed-to by `start_l` are at most `block_l`, so
            // these `offset` calls are safe as all reads are within the block. The same argument
            // applies for the uses of `right!`.
            //
            // The calls to `start_l.offset` are valid because there are at most `count-1` of them,
            // plus the final one at the end of the unsafe block, where `count` is the minimum number
            // of collected offsets in `offsets_l` and `offsets_r`, so there is no risk of there not
            // being enough elements. The same reasoning applies to the calls to `start_r.offset`.
            //
            // The calls to `copy_nonoverlapping` are safe because `left!` and `right!` are guaranteed
            // not to overlap, and are valid because of the reasoning above.
            </span><span class="kw">unsafe </span>{
                <span class="kw">let </span>tmp = ptr::read(<span class="macro">left!</span>());
                ptr::copy_nonoverlapping(<span class="macro">right!</span>(), <span class="macro">left!</span>(), <span class="number">1</span>);

                <span class="kw">for _ in </span><span class="number">1</span>..count {
                    start_l = start_l.offset(<span class="number">1</span>);
                    ptr::copy_nonoverlapping(<span class="macro">left!</span>(), <span class="macro">right!</span>(), <span class="number">1</span>);
                    start_r = start_r.offset(<span class="number">1</span>);
                    ptr::copy_nonoverlapping(<span class="macro">right!</span>(), <span class="macro">left!</span>(), <span class="number">1</span>);
                }

                ptr::copy_nonoverlapping(<span class="kw-2">&amp;</span>tmp, <span class="macro">right!</span>(), <span class="number">1</span>);
                mem::forget(tmp);
                start_l = start_l.offset(<span class="number">1</span>);
                start_r = start_r.offset(<span class="number">1</span>);
            }
        }

        <span class="kw">if </span>start_l == end_l {
            <span class="comment">// All out-of-order elements in the left block were moved. Move to the next block.

            // block-width-guarantee
            // SAFETY: if `!is_done` then the slice width is guaranteed to be at least `2*BLOCK` wide. There
            // are at most `BLOCK` elements in `offsets_l` because of its size, so the `offset` operation is
            // safe. Otherwise, the debug assertions in the `is_done` case guarantee that
            // `width(l, r) == block_l + block_r`, namely, that the block sizes have been adjusted to account
            // for the smaller number of remaining elements.
            </span>l = <span class="kw">unsafe </span>{ l.add(block_l) };
        }

        <span class="kw">if </span>start_r == end_r {
            <span class="comment">// All out-of-order elements in the right block were moved. Move to the previous block.

            // SAFETY: Same argument as [block-width-guarantee]. Either this is a full block `2*BLOCK`-wide,
            // or `block_r` has been adjusted for the last handful of elements.
            </span>r = <span class="kw">unsafe </span>{ r.offset(-(block_r <span class="kw">as </span>isize)) };
        }

        <span class="kw">if </span>is_done {
            <span class="kw">break</span>;
        }
    }

    <span class="comment">// All that remains now is at most one block (either the left or the right) with out-of-order
    // elements that need to be moved. Such remaining elements can be simply shifted to the end
    // within their block.

    </span><span class="kw">if </span>start_l &lt; end_l {
        <span class="comment">// The left block remains.
        // Move its remaining out-of-order elements to the far right.
        </span><span class="macro">debug_assert_eq!</span>(width(l, r), block_l);
        <span class="kw">while </span>start_l &lt; end_l {
            <span class="comment">// remaining-elements-safety
            // SAFETY: while the loop condition holds there are still elements in `offsets_l`, so it
            // is safe to point `end_l` to the previous element.
            //
            // The `ptr::swap` is safe if both its arguments are valid for reads and writes:
            //  - Per the debug assert above, the distance between `l` and `r` is `block_l`
            //    elements, so there can be at most `block_l` remaining offsets between `start_l`
            //    and `end_l`. This means `r` will be moved at most `block_l` steps back, which
            //    makes the `r.offset` calls valid (at that point `l == r`).
            //  - `offsets_l` contains valid offsets into `v` collected during the partitioning of
            //    the last block, so the `l.offset` calls are valid.
            </span><span class="kw">unsafe </span>{
                end_l = end_l.offset(-<span class="number">1</span>);
                ptr::swap(l.offset(<span class="kw-2">*</span>end_l <span class="kw">as </span>isize), r.offset(-<span class="number">1</span>));
                r = r.offset(-<span class="number">1</span>);
            }
        }
        width(v.as_mut_ptr(), r)
    } <span class="kw">else if </span>start_r &lt; end_r {
        <span class="comment">// The right block remains.
        // Move its remaining out-of-order elements to the far left.
        </span><span class="macro">debug_assert_eq!</span>(width(l, r), block_r);
        <span class="kw">while </span>start_r &lt; end_r {
            <span class="comment">// SAFETY: See the reasoning in [remaining-elements-safety].
            </span><span class="kw">unsafe </span>{
                end_r = end_r.offset(-<span class="number">1</span>);
                ptr::swap(l, r.offset(-(<span class="kw-2">*</span>end_r <span class="kw">as </span>isize) - <span class="number">1</span>));
                l = l.offset(<span class="number">1</span>);
            }
        }
        width(v.as_mut_ptr(), l)
    } <span class="kw">else </span>{
        <span class="comment">// Nothing else to do, we're done.
        </span>width(v.as_mut_ptr(), l)
    }
}

<span class="doccomment">/// Partitions `v` into elements smaller than `v[pivot]`, followed by elements greater than or
/// equal to `v[pivot]`.
///
/// Returns a tuple of:
///
/// 1. Number of elements smaller than `v[pivot]`.
/// 2. True if `v` was already partitioned.
</span><span class="kw">fn </span>partition&lt;T, F&gt;(v: <span class="kw-2">&amp;mut </span>[T], pivot: usize, is_less: <span class="kw-2">&amp;</span>F) -&gt; (usize, bool)
<span class="kw">where
    </span>F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool,
{
    <span class="kw">let </span>(mid, was_partitioned) = {
        <span class="comment">// Place the pivot at the beginning of slice.
        </span>v.swap(<span class="number">0</span>, pivot);
        <span class="kw">let </span>(pivot, v) = v.split_at_mut(<span class="number">1</span>);
        <span class="kw">let </span>pivot = <span class="kw-2">&amp;mut </span>pivot[<span class="number">0</span>];

        <span class="comment">// Read the pivot into a stack-allocated variable for efficiency. If a following comparison
        // operation panics, the pivot will be automatically written back into the slice.

        // SAFETY: `pivot` is a reference to the first element of `v`, so `ptr::read` is safe.
        </span><span class="kw">let </span>tmp = mem::ManuallyDrop::new(<span class="kw">unsafe </span>{ ptr::read(pivot) });
        <span class="kw">let </span>_pivot_guard = <span class="kw">unsafe </span>{ CopyOnDrop::new(<span class="kw-2">&amp;*</span>tmp, pivot) };
        <span class="kw">let </span>pivot = <span class="kw-2">&amp;*</span>tmp;

        <span class="comment">// Find the first pair of out-of-order elements.
        </span><span class="kw">let </span><span class="kw-2">mut </span>l = <span class="number">0</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>r = v.len();

        <span class="comment">// SAFETY: The unsafety below involves indexing an array.
        // For the first one: We already do the bounds checking here with `l &lt; r`.
        // For the second one: We initially have `l == 0` and `r == v.len()` and we checked that `l &lt; r` at every indexing operation.
        //                     From here we know that `r` must be at least `r == l` which was shown to be valid from the first one.
        </span><span class="kw">unsafe </span>{
            <span class="comment">// Find the first element greater than or equal to the pivot.
            </span><span class="kw">while </span>l &lt; r &amp;&amp; is_less(v.get_unchecked(l), pivot) {
                l += <span class="number">1</span>;
            }

            <span class="comment">// Find the last element smaller that the pivot.
            </span><span class="kw">while </span>l &lt; r &amp;&amp; !is_less(v.get_unchecked(r - <span class="number">1</span>), pivot) {
                r -= <span class="number">1</span>;
            }
        }

        (
            l + partition_in_blocks(<span class="kw-2">&amp;mut </span>v[l..r], pivot, is_less),
            l &gt;= r,
        )

        <span class="comment">// `_pivot_guard` goes out of scope and writes the pivot (which is a stack-allocated
        // variable) back into the slice where it originally was. This step is critical in ensuring
        // safety!
    </span>};

    <span class="comment">// Place the pivot between the two partitions.
    </span>v.swap(<span class="number">0</span>, mid);

    (mid, was_partitioned)
}

<span class="doccomment">/// Partitions `v` into elements equal to `v[pivot]` followed by elements greater than `v[pivot]`.
///
/// Returns the number of elements equal to the pivot. It is assumed that `v` does not contain
/// elements smaller than the pivot.
</span><span class="kw">fn </span>partition_equal&lt;T, F&gt;(v: <span class="kw-2">&amp;mut </span>[T], pivot: usize, is_less: <span class="kw-2">&amp;</span>F) -&gt; usize
<span class="kw">where
    </span>F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool,
{
    <span class="comment">// Place the pivot at the beginning of slice.
    </span>v.swap(<span class="number">0</span>, pivot);
    <span class="kw">let </span>(pivot, v) = v.split_at_mut(<span class="number">1</span>);
    <span class="kw">let </span>pivot = <span class="kw-2">&amp;mut </span>pivot[<span class="number">0</span>];

    <span class="comment">// Read the pivot into a stack-allocated variable for efficiency. If a following comparison
    // operation panics, the pivot will be automatically written back into the slice.
    // SAFETY: The pointer here is valid because it is obtained from a reference to a slice.
    </span><span class="kw">let </span>tmp = mem::ManuallyDrop::new(<span class="kw">unsafe </span>{ ptr::read(pivot) });
    <span class="kw">let </span>_pivot_guard = <span class="kw">unsafe </span>{ CopyOnDrop::new(<span class="kw-2">&amp;*</span>tmp, pivot) };
    <span class="kw">let </span>pivot = <span class="kw-2">&amp;*</span>tmp;

    <span class="comment">// Now partition the slice.
    </span><span class="kw">let </span><span class="kw-2">mut </span>l = <span class="number">0</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>r = v.len();
    <span class="kw">loop </span>{
        <span class="comment">// SAFETY: The unsafety below involves indexing an array.
        // For the first one: We already do the bounds checking here with `l &lt; r`.
        // For the second one: We initially have `l == 0` and `r == v.len()` and we checked that `l &lt; r` at every indexing operation.
        //                     From here we know that `r` must be at least `r == l` which was shown to be valid from the first one.
        </span><span class="kw">unsafe </span>{
            <span class="comment">// Find the first element greater than the pivot.
            </span><span class="kw">while </span>l &lt; r &amp;&amp; !is_less(pivot, v.get_unchecked(l)) {
                l += <span class="number">1</span>;
            }

            <span class="comment">// Find the last element equal to the pivot.
            </span><span class="kw">while </span>l &lt; r &amp;&amp; is_less(pivot, v.get_unchecked(r - <span class="number">1</span>)) {
                r -= <span class="number">1</span>;
            }

            <span class="comment">// Are we done?
            </span><span class="kw">if </span>l &gt;= r {
                <span class="kw">break</span>;
            }

            <span class="comment">// Swap the found pair of out-of-order elements.
            </span>r -= <span class="number">1</span>;
            <span class="kw">let </span>ptr = v.as_mut_ptr();
            ptr::swap(ptr.add(l), ptr.add(r));
            l += <span class="number">1</span>;
        }
    }

    <span class="comment">// We found `l` elements equal to the pivot. Add 1 to account for the pivot itself.
    </span>l + <span class="number">1

    </span><span class="comment">// `_pivot_guard` goes out of scope and writes the pivot (which is a stack-allocated variable)
    // back into the slice where it originally was. This step is critical in ensuring safety!
</span>}

<span class="doccomment">/// Scatters some elements around in an attempt to break patterns that might cause imbalanced
/// partitions in quicksort.
</span><span class="attr">#[cold]
</span><span class="kw">fn </span>break_patterns&lt;T&gt;(v: <span class="kw-2">&amp;mut </span>[T]) {
    <span class="kw">let </span>len = v.len();
    <span class="kw">if </span>len &gt;= <span class="number">8 </span>{
        <span class="comment">// Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
        </span><span class="kw">let </span><span class="kw-2">mut </span>random = len <span class="kw">as </span>u32;
        <span class="kw">let </span><span class="kw-2">mut </span>gen_u32 = || {
            random ^= random &lt;&lt; <span class="number">13</span>;
            random ^= random &gt;&gt; <span class="number">17</span>;
            random ^= random &lt;&lt; <span class="number">5</span>;
            random
        };
        <span class="kw">let </span><span class="kw-2">mut </span>gen_usize = || {
            <span class="kw">if </span>usize::BITS &lt;= <span class="number">32 </span>{
                gen_u32() <span class="kw">as </span>usize
            } <span class="kw">else </span>{
                (((gen_u32() <span class="kw">as </span>u64) &lt;&lt; <span class="number">32</span>) | (gen_u32() <span class="kw">as </span>u64)) <span class="kw">as </span>usize
            }
        };

        <span class="comment">// Take random numbers modulo this number.
        // The number fits into `usize` because `len` is not greater than `isize::MAX`.
        </span><span class="kw">let </span>modulus = len.next_power_of_two();

        <span class="comment">// Some pivot candidates will be in the nearby of this index. Let's randomize them.
        </span><span class="kw">let </span>pos = len / <span class="number">4 </span>* <span class="number">2</span>;

        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">3 </span>{
            <span class="comment">// Generate a random number modulo `len`. However, in order to avoid costly operations
            // we first take it modulo a power of two, and then decrease by `len` until it fits
            // into the range `[0, len - 1]`.
            </span><span class="kw">let </span><span class="kw-2">mut </span>other = gen_usize() &amp; (modulus - <span class="number">1</span>);

            <span class="comment">// `other` is guaranteed to be less than `2 * len`.
            </span><span class="kw">if </span>other &gt;= len {
                other -= len;
            }

            v.swap(pos - <span class="number">1 </span>+ i, other);
        }
    }
}

<span class="doccomment">/// Chooses a pivot in `v` and returns the index and `true` if the slice is likely already sorted.
///
/// Elements in `v` might be reordered in the process.
</span><span class="kw">fn </span>choose_pivot&lt;T, F&gt;(v: <span class="kw-2">&amp;mut </span>[T], is_less: <span class="kw-2">&amp;</span>F) -&gt; (usize, bool)
<span class="kw">where
    </span>F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool,
{
    <span class="comment">// Minimum length to choose the median-of-medians method.
    // Shorter slices use the simple median-of-three method.
    </span><span class="kw">const </span>SHORTEST_MEDIAN_OF_MEDIANS: usize = <span class="number">50</span>;
    <span class="comment">// Maximum number of swaps that can be performed in this function.
    </span><span class="kw">const </span>MAX_SWAPS: usize = <span class="number">4 </span>* <span class="number">3</span>;

    <span class="kw">let </span>len = v.len();

    <span class="comment">// Three indices near which we are going to choose a pivot.
    </span><span class="attr">#[allow(clippy::identity_op)]
    </span><span class="kw">let </span><span class="kw-2">mut </span>a = len / <span class="number">4 </span>* <span class="number">1</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>b = len / <span class="number">4 </span>* <span class="number">2</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>c = len / <span class="number">4 </span>* <span class="number">3</span>;

    <span class="comment">// Counts the total number of swaps we are about to perform while sorting indices.
    </span><span class="kw">let </span><span class="kw-2">mut </span>swaps = <span class="number">0</span>;

    <span class="kw">if </span>len &gt;= <span class="number">8 </span>{
        <span class="comment">// Swaps indices so that `v[a] &lt;= v[b]`.
        // SAFETY: `len &gt;= 8` so there are at least two elements in the neighborhoods of
        // `a`, `b` and `c`. This means the three calls to `sort_adjacent` result in
        // corresponding calls to `sort3` with valid 3-item neighborhoods around each
        // pointer, which in turn means the calls to `sort2` are done with valid
        // references. Thus the `v.get_unchecked` calls are safe, as is the `ptr::swap`
        // call.
        </span><span class="kw">let </span><span class="kw-2">mut </span>sort2 = |a: <span class="kw-2">&amp;mut </span>usize, b: <span class="kw-2">&amp;mut </span>usize| <span class="kw">unsafe </span>{
            <span class="kw">if </span>is_less(v.get_unchecked(<span class="kw-2">*</span>b), v.get_unchecked(<span class="kw-2">*</span>a)) {
                ptr::swap(a, b);
                swaps += <span class="number">1</span>;
            }
        };

        <span class="comment">// Swaps indices so that `v[a] &lt;= v[b] &lt;= v[c]`.
        </span><span class="kw">let </span><span class="kw-2">mut </span>sort3 = |a: <span class="kw-2">&amp;mut </span>usize, b: <span class="kw-2">&amp;mut </span>usize, c: <span class="kw-2">&amp;mut </span>usize| {
            sort2(a, b);
            sort2(b, c);
            sort2(a, b);
        };

        <span class="kw">if </span>len &gt;= SHORTEST_MEDIAN_OF_MEDIANS {
            <span class="comment">// Finds the median of `v[a - 1], v[a], v[a + 1]` and stores the index into `a`.
            </span><span class="kw">let </span><span class="kw-2">mut </span>sort_adjacent = |a: <span class="kw-2">&amp;mut </span>usize| {
                <span class="kw">let </span>tmp = <span class="kw-2">*</span>a;
                sort3(<span class="kw-2">&amp;mut </span>(tmp - <span class="number">1</span>), a, <span class="kw-2">&amp;mut </span>(tmp + <span class="number">1</span>));
            };

            <span class="comment">// Find medians in the neighborhoods of `a`, `b`, and `c`.
            </span>sort_adjacent(<span class="kw-2">&amp;mut </span>a);
            sort_adjacent(<span class="kw-2">&amp;mut </span>b);
            sort_adjacent(<span class="kw-2">&amp;mut </span>c);
        }

        <span class="comment">// Find the median among `a`, `b`, and `c`.
        </span>sort3(<span class="kw-2">&amp;mut </span>a, <span class="kw-2">&amp;mut </span>b, <span class="kw-2">&amp;mut </span>c);
    }

    <span class="kw">if </span>swaps &lt; MAX_SWAPS {
        (b, swaps == <span class="number">0</span>)
    } <span class="kw">else </span>{
        <span class="comment">// The maximum number of swaps was performed. Chances are the slice is descending or mostly
        // descending, so reversing will probably help sort it faster.
        </span>v.reverse();
        (len - <span class="number">1 </span>- b, <span class="bool-val">true</span>)
    }
}

<span class="doccomment">/// Sorts `v` recursively.
///
/// If the slice had a predecessor in the original array, it is specified as `pred`.
///
/// `limit` is the number of allowed imbalanced partitions before switching to `heapsort`. If zero,
/// this function will immediately switch to heapsort.
</span><span class="kw">fn </span>recurse&lt;<span class="lifetime">'a</span>, T, F&gt;(<span class="kw-2">mut </span>v: <span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>[T], is_less: <span class="kw-2">&amp;</span>F, <span class="kw-2">mut </span>pred: <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="kw-2">mut </span>T&gt;, <span class="kw-2">mut </span>limit: u32)
<span class="kw">where
    </span>T: Send,
    F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool + Sync,
{
    <span class="comment">// Slices of up to this length get sorted using insertion sort.
    </span><span class="kw">const </span>MAX_INSERTION: usize = <span class="number">20</span>;
    <span class="comment">// If both partitions are up to this length, we continue sequentially. This number is as small
    // as possible but so that the overhead of Rayon's task scheduling is still negligible.
    </span><span class="kw">const </span>MAX_SEQUENTIAL: usize = <span class="number">2000</span>;

    <span class="comment">// True if the last partitioning was reasonably balanced.
    </span><span class="kw">let </span><span class="kw-2">mut </span>was_balanced = <span class="bool-val">true</span>;
    <span class="comment">// True if the last partitioning didn't shuffle elements (the slice was already partitioned).
    </span><span class="kw">let </span><span class="kw-2">mut </span>was_partitioned = <span class="bool-val">true</span>;

    <span class="kw">loop </span>{
        <span class="kw">let </span>len = v.len();

        <span class="comment">// Very short slices get sorted using insertion sort.
        </span><span class="kw">if </span>len &lt;= MAX_INSERTION {
            insertion_sort(v, is_less);
            <span class="kw">return</span>;
        }

        <span class="comment">// If too many bad pivot choices were made, simply fall back to heapsort in order to
        // guarantee `O(n * log(n))` worst-case.
        </span><span class="kw">if </span>limit == <span class="number">0 </span>{
            heapsort(v, is_less);
            <span class="kw">return</span>;
        }

        <span class="comment">// If the last partitioning was imbalanced, try breaking patterns in the slice by shuffling
        // some elements around. Hopefully we'll choose a better pivot this time.
        </span><span class="kw">if </span>!was_balanced {
            break_patterns(v);
            limit -= <span class="number">1</span>;
        }

        <span class="comment">// Choose a pivot and try guessing whether the slice is already sorted.
        </span><span class="kw">let </span>(pivot, likely_sorted) = choose_pivot(v, is_less);

        <span class="comment">// If the last partitioning was decently balanced and didn't shuffle elements, and if pivot
        // selection predicts the slice is likely already sorted...
        </span><span class="kw">if </span>was_balanced &amp;&amp; was_partitioned &amp;&amp; likely_sorted {
            <span class="comment">// Try identifying several out-of-order elements and shifting them to correct
            // positions. If the slice ends up being completely sorted, we're done.
            </span><span class="kw">if </span>partial_insertion_sort(v, is_less) {
                <span class="kw">return</span>;
            }
        }

        <span class="comment">// If the chosen pivot is equal to the predecessor, then it's the smallest element in the
        // slice. Partition the slice into elements equal to and elements greater than the pivot.
        // This case is usually hit when the slice contains many duplicate elements.
        </span><span class="kw">if let </span><span class="prelude-val">Some</span>(<span class="kw-2">ref </span>p) = pred {
            <span class="kw">if </span>!is_less(p, <span class="kw-2">&amp;</span>v[pivot]) {
                <span class="kw">let </span>mid = partition_equal(v, pivot, is_less);

                <span class="comment">// Continue sorting elements greater than the pivot.
                </span>v = <span class="kw-2">&amp;mut </span>v[mid..];
                <span class="kw">continue</span>;
            }
        }

        <span class="comment">// Partition the slice.
        </span><span class="kw">let </span>(mid, was_p) = partition(v, pivot, is_less);
        was_balanced = Ord::min(mid, len - mid) &gt;= len / <span class="number">8</span>;
        was_partitioned = was_p;

        <span class="comment">// Split the slice into `left`, `pivot`, and `right`.
        </span><span class="kw">let </span>(left, right) = v.split_at_mut(mid);
        <span class="kw">let </span>(pivot, right) = right.split_at_mut(<span class="number">1</span>);
        <span class="kw">let </span>pivot = <span class="kw-2">&amp;mut </span>pivot[<span class="number">0</span>];

        <span class="kw">if </span>Ord::max(left.len(), right.len()) &lt;= MAX_SEQUENTIAL {
            <span class="comment">// Recurse into the shorter side only in order to minimize the total number of recursive
            // calls and consume less stack space. Then just continue with the longer side (this is
            // akin to tail recursion).
            </span><span class="kw">if </span>left.len() &lt; right.len() {
                recurse(left, is_less, pred, limit);
                v = right;
                pred = <span class="prelude-val">Some</span>(pivot);
            } <span class="kw">else </span>{
                recurse(right, is_less, <span class="prelude-val">Some</span>(pivot), limit);
                v = left;
            }
        } <span class="kw">else </span>{
            <span class="comment">// Sort the left and right half in parallel.
            </span>rayon_core::join(
                || recurse(left, is_less, pred, limit),
                || recurse(right, is_less, <span class="prelude-val">Some</span>(pivot), limit),
            );
            <span class="kw">break</span>;
        }
    }
}

<span class="doccomment">/// Sorts `v` using pattern-defeating quicksort in parallel.
///
/// The algorithm is unstable, in-place, and *O*(*n* \* log(*n*)) worst-case.
</span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>par_quicksort&lt;T, F&gt;(v: <span class="kw-2">&amp;mut </span>[T], is_less: F)
<span class="kw">where
    </span>T: Send,
    F: Fn(<span class="kw-2">&amp;</span>T, <span class="kw-2">&amp;</span>T) -&gt; bool + Sync,
{
    <span class="comment">// Sorting has no meaningful behavior on zero-sized types.
    </span><span class="kw">if </span>mem::size_of::&lt;T&gt;() == <span class="number">0 </span>{
        <span class="kw">return</span>;
    }

    <span class="comment">// Limit the number of imbalanced partitions to `floor(log2(len)) + 1`.
    </span><span class="kw">let </span>limit = usize::BITS - v.len().leading_zeros();

    recurse(v, <span class="kw-2">&amp;</span>is_less, <span class="prelude-val">None</span>, limit);
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use </span><span class="kw">super</span>::heapsort;
    <span class="kw">use </span>rand::distributions::Uniform;
    <span class="kw">use </span>rand::{thread_rng, Rng};

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_heapsort() {
        <span class="kw">let </span>rng = <span class="kw-2">&amp;mut </span>thread_rng();

        <span class="kw">for </span>len <span class="kw">in </span>(<span class="number">0</span>..<span class="number">25</span>).chain(<span class="number">500</span>..<span class="number">501</span>) {
            <span class="kw">for </span><span class="kw-2">&amp;</span>modulus <span class="kw">in </span><span class="kw-2">&amp;</span>[<span class="number">5</span>, <span class="number">10</span>, <span class="number">100</span>] {
                <span class="kw">let </span>dist = Uniform::new(<span class="number">0</span>, modulus);
                <span class="kw">for _ in </span><span class="number">0</span>..<span class="number">100 </span>{
                    <span class="kw">let </span>v: Vec&lt;i32&gt; = rng.sample_iter(<span class="kw-2">&amp;</span>dist).take(len).collect();

                    <span class="comment">// Test heapsort using `&lt;` operator.
                    </span><span class="kw">let </span><span class="kw-2">mut </span>tmp = v.clone();
                    heapsort(<span class="kw-2">&amp;mut </span>tmp, <span class="kw-2">&amp;</span>|a, b| a &lt; b);
                    <span class="macro">assert!</span>(tmp.windows(<span class="number">2</span>).all(|w| w[<span class="number">0</span>] &lt;= w[<span class="number">1</span>]));

                    <span class="comment">// Test heapsort using `&gt;` operator.
                    </span><span class="kw">let </span><span class="kw-2">mut </span>tmp = v.clone();
                    heapsort(<span class="kw-2">&amp;mut </span>tmp, <span class="kw-2">&amp;</span>|a, b| a &gt; b);
                    <span class="macro">assert!</span>(tmp.windows(<span class="number">2</span>).all(|w| w[<span class="number">0</span>] &gt;= w[<span class="number">1</span>]));
                }
            }
        }

        <span class="comment">// Sort using a completely random comparison function.
        // This will reorder the elements *somehow*, but won't panic.
        </span><span class="kw">let </span><span class="kw-2">mut </span>v: Vec&lt;<span class="kw">_</span>&gt; = (<span class="number">0</span>..<span class="number">100</span>).collect();
        heapsort(<span class="kw-2">&amp;mut </span>v, <span class="kw-2">&amp;</span>|<span class="kw">_</span>, <span class="kw">_</span>| thread_rng().gen());
        heapsort(<span class="kw-2">&amp;mut </span>v, <span class="kw-2">&amp;</span>|a, b| a &lt; b);

        <span class="kw">for </span>(i, <span class="kw-2">&amp;</span>entry) <span class="kw">in </span>v.iter().enumerate() {
            <span class="macro">assert_eq!</span>(entry, i);
        }
    }
}
</code></pre></div></section></main></body></html>