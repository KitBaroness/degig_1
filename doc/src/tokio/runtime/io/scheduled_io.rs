<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.37.0/src/runtime/io/scheduled_io.rs`."><title>scheduled_io.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../" data-static-root-path="../../../../static.files/" data-current-crate="tokio" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../src-files.js"></script><script defer src="../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../tokio/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span><span class="kw">crate</span>::io::interest::Interest;
<span class="kw">use </span><span class="kw">crate</span>::io::ready::Ready;
<span class="kw">use </span><span class="kw">crate</span>::loom::sync::atomic::AtomicUsize;
<span class="kw">use </span><span class="kw">crate</span>::loom::sync::Mutex;
<span class="kw">use </span><span class="kw">crate</span>::runtime::io::{Direction, ReadyEvent, Tick};
<span class="kw">use </span><span class="kw">crate</span>::util::bit;
<span class="kw">use </span><span class="kw">crate</span>::util::linked_list::{<span class="self">self</span>, LinkedList};
<span class="kw">use </span><span class="kw">crate</span>::util::WakeList;

<span class="kw">use </span>std::cell::UnsafeCell;
<span class="kw">use </span>std::future::Future;
<span class="kw">use </span>std::marker::PhantomPinned;
<span class="kw">use </span>std::pin::Pin;
<span class="kw">use </span>std::ptr::NonNull;
<span class="kw">use </span>std::sync::atomic::Ordering::{AcqRel, Acquire};
<span class="kw">use </span>std::task::{Context, Poll, Waker};

<span class="doccomment">/// Stored in the I/O driver resource slab.
</span><span class="attr">#[derive(Debug)]
</span><span class="comment">// # This struct should be cache padded to avoid false sharing. The cache padding rules are copied
// from crossbeam-utils/src/cache_padded.rs
//
// Starting from Intel's Sandy Bridge, spatial prefetcher is now pulling pairs of 64-byte cache
// lines at a time, so we have to align to 128 bytes rather than 64.
//
// Sources:
// - https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-optimization-manual.pdf
// - https://github.com/facebook/folly/blob/1b5288e6eea6df074758f877c849b6e73bbb9fbb/folly/lang/Align.h#L107
//
// ARM's big.LITTLE architecture has asymmetric cores and "big" cores have 128-byte cache line size.
//
// Sources:
// - https://www.mono-project.com/news/2016/09/12/arm64-icache/
//
// powerpc64 has 128-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_ppc64x.go#L9
</span><span class="attr">#[cfg_attr(
    any(
        target_arch = <span class="string">"x86_64"</span>,
        target_arch = <span class="string">"aarch64"</span>,
        target_arch = <span class="string">"powerpc64"</span>,
    ),
    repr(align(<span class="number">128</span>))
)]
</span><span class="comment">// arm, mips, mips64, sparc, and hexagon have 32-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_arm.go#L7
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips.go#L7
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mipsle.go#L7
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips64x.go#L9
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L17
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/hexagon/include/asm/cache.h#L12
</span><span class="attr">#[cfg_attr(
    any(
        target_arch = <span class="string">"arm"</span>,
        target_arch = <span class="string">"mips"</span>,
        target_arch = <span class="string">"mips64"</span>,
        target_arch = <span class="string">"sparc"</span>,
        target_arch = <span class="string">"hexagon"</span>,
    ),
    repr(align(<span class="number">32</span>))
)]
</span><span class="comment">// m68k has 16-byte cache line size.
//
// Sources:
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/m68k/include/asm/cache.h#L9
</span><span class="attr">#[cfg_attr(target_arch = <span class="string">"m68k"</span>, repr(align(<span class="number">16</span>)))]
</span><span class="comment">// s390x has 256-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_s390x.go#L7
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/s390/include/asm/cache.h#L13
</span><span class="attr">#[cfg_attr(target_arch = <span class="string">"s390x"</span>, repr(align(<span class="number">256</span>)))]
</span><span class="comment">// x86, riscv, wasm, and sparc64 have 64-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/dda2991c2ea0c5914714469c4defc2562a907230/src/internal/cpu/cpu_x86.go#L9
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_wasm.go#L7
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L19
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/riscv/include/asm/cache.h#L10
//
// All others are assumed to have 64-byte cache line size.
</span><span class="attr">#[cfg_attr(
    not(any(
        target_arch = <span class="string">"x86_64"</span>,
        target_arch = <span class="string">"aarch64"</span>,
        target_arch = <span class="string">"powerpc64"</span>,
        target_arch = <span class="string">"arm"</span>,
        target_arch = <span class="string">"mips"</span>,
        target_arch = <span class="string">"mips64"</span>,
        target_arch = <span class="string">"sparc"</span>,
        target_arch = <span class="string">"hexagon"</span>,
        target_arch = <span class="string">"m68k"</span>,
        target_arch = <span class="string">"s390x"</span>,
    )),
    repr(align(<span class="number">64</span>))
)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">struct </span>ScheduledIo {
    <span class="kw">pub</span>(<span class="kw">super</span>) linked_list_pointers: UnsafeCell&lt;linked_list::Pointers&lt;<span class="self">Self</span>&gt;&gt;,

    <span class="doccomment">/// Packs the resource's readiness and I/O driver latest tick.
    </span>readiness: AtomicUsize,

    waiters: Mutex&lt;Waiters&gt;,
}

<span class="kw">type </span>WaitList = LinkedList&lt;Waiter, &lt;Waiter <span class="kw">as </span>linked_list::Link&gt;::Target&gt;;

<span class="attr">#[derive(Debug, Default)]
</span><span class="kw">struct </span>Waiters {
    <span class="doccomment">/// List of all current waiters.
    </span>list: WaitList,

    <span class="doccomment">/// Waker used for `AsyncRead`.
    </span>reader: <span class="prelude-ty">Option</span>&lt;Waker&gt;,

    <span class="doccomment">/// Waker used for `AsyncWrite`.
    </span>writer: <span class="prelude-ty">Option</span>&lt;Waker&gt;,
}

<span class="attr">#[derive(Debug)]
</span><span class="kw">struct </span>Waiter {
    pointers: linked_list::Pointers&lt;Waiter&gt;,

    <span class="doccomment">/// The waker for this task.
    </span>waker: <span class="prelude-ty">Option</span>&lt;Waker&gt;,

    <span class="doccomment">/// The interest this waiter is waiting on.
    </span>interest: Interest,

    is_ready: bool,

    <span class="doccomment">/// Should never be `!Unpin`.
    </span>_p: PhantomPinned,
}

<span class="macro">generate_addr_of_methods!</span> {
    <span class="kw">impl</span>&lt;&gt; Waiter {
        <span class="kw">unsafe fn </span>addr_of_pointers(<span class="self">self</span>: NonNull&lt;<span class="self">Self</span>&gt;) -&gt; NonNull&lt;linked_list::Pointers&lt;Waiter&gt;&gt; {
            <span class="kw-2">&amp;</span><span class="self">self</span>.pointers
        }
    }
}

<span class="doccomment">/// Future returned by `readiness()`.
</span><span class="kw">struct </span>Readiness&lt;<span class="lifetime">'a</span>&gt; {
    scheduled_io: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>ScheduledIo,

    state: State,

    <span class="doccomment">/// Entry in the waiter `LinkedList`.
    </span>waiter: UnsafeCell&lt;Waiter&gt;,
}

<span class="kw">enum </span>State {
    Init,
    Waiting,
    Done,
}

<span class="comment">// The `ScheduledIo::readiness` (`AtomicUsize`) is packed full of goodness.
//
// | shutdown | driver tick | readiness |
// |----------+-------------+-----------|
// |   1 bit  |  15 bits    +   16 bits |

</span><span class="kw">const </span>READINESS: bit::Pack = bit::Pack::least_significant(<span class="number">16</span>);

<span class="kw">const </span>TICK: bit::Pack = READINESS.then(<span class="number">15</span>);

<span class="kw">const </span>SHUTDOWN: bit::Pack = TICK.then(<span class="number">1</span>);

<span class="comment">// ===== impl ScheduledIo =====

</span><span class="kw">impl </span>Default <span class="kw">for </span>ScheduledIo {
    <span class="kw">fn </span>default() -&gt; ScheduledIo {
        ScheduledIo {
            linked_list_pointers: UnsafeCell::new(linked_list::Pointers::new()),
            readiness: AtomicUsize::new(<span class="number">0</span>),
            waiters: Mutex::new(Waiters::default()),
        }
    }
}

<span class="kw">impl </span>ScheduledIo {
    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>token(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; mio::Token {
        <span class="comment">// use `expose_addr` when stable
        </span>mio::Token(<span class="self">self </span><span class="kw">as </span><span class="kw-2">*const </span><span class="kw">_ as </span>usize)
    }

    <span class="doccomment">/// Invoked when the IO driver is shut down; forces this `ScheduledIo` into a
    /// permanently shutdown state.
    </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>shutdown(<span class="kw-2">&amp;</span><span class="self">self</span>) {
        <span class="kw">let </span>mask = SHUTDOWN.pack(<span class="number">1</span>, <span class="number">0</span>);
        <span class="self">self</span>.readiness.fetch_or(mask, AcqRel);
        <span class="self">self</span>.wake(Ready::ALL);
    }

    <span class="doccomment">/// Sets the readiness on this `ScheduledIo` by invoking the given closure on
    /// the current value, returning the previous readiness value.
    ///
    /// # Arguments
    /// - `tick`: whether setting the tick or trying to clear readiness for a
    ///    specific tick.
    /// - `f`: a closure returning a new readiness value given the previous
    ///   readiness.
    </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>set_readiness(<span class="kw-2">&amp;</span><span class="self">self</span>, tick: Tick, f: <span class="kw">impl </span>Fn(Ready) -&gt; Ready) {
        <span class="kw">let </span><span class="kw-2">mut </span>current = <span class="self">self</span>.readiness.load(Acquire);

        <span class="comment">// If the io driver is shut down, then you are only allowed to clear readiness.
        </span><span class="macro">debug_assert!</span>(SHUTDOWN.unpack(current) == <span class="number">0 </span>|| <span class="macro">matches!</span>(tick, Tick::Clear(<span class="kw">_</span>)));

        <span class="kw">loop </span>{
            <span class="comment">// Mask out the tick bits so that the modifying function doesn't see
            // them.
            </span><span class="kw">let </span>current_readiness = Ready::from_usize(current);
            <span class="kw">let </span>new = f(current_readiness);

            <span class="kw">let </span>new_tick = <span class="kw">match </span>tick {
                Tick::Set =&gt; {
                    <span class="kw">let </span>current = TICK.unpack(current);
                    current.wrapping_add(<span class="number">1</span>) % (TICK.max_value() + <span class="number">1</span>)
                }
                Tick::Clear(t) =&gt; {
                    <span class="kw">if </span>TICK.unpack(current) <span class="kw">as </span>u8 != t {
                        <span class="comment">// Trying to clear readiness with an old event!
                        </span><span class="kw">return</span>;
                    }

                    t <span class="kw">as </span>usize
                }
            };
            <span class="kw">let </span>next = TICK.pack(new_tick, new.as_usize());

            <span class="kw">match </span><span class="self">self
                </span>.readiness
                .compare_exchange(current, next, AcqRel, Acquire)
            {
                <span class="prelude-val">Ok</span>(<span class="kw">_</span>) =&gt; <span class="kw">return</span>,
                <span class="comment">// we lost the race, retry!
                </span><span class="prelude-val">Err</span>(actual) =&gt; current = actual,
            }
        }
    }

    <span class="doccomment">/// Notifies all pending waiters that have registered interest in `ready`.
    ///
    /// There may be many waiters to notify. Waking the pending task **must** be
    /// done from outside of the lock otherwise there is a potential for a
    /// deadlock.
    ///
    /// A stack array of wakers is created and filled with wakers to notify, the
    /// lock is released, and the wakers are notified. Because there may be more
    /// than 32 wakers to notify, if the stack array fills up, the lock is
    /// released, the array is cleared, and the iteration continues.
    </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>wake(<span class="kw-2">&amp;</span><span class="self">self</span>, ready: Ready) {
        <span class="kw">let </span><span class="kw-2">mut </span>wakers = WakeList::new();

        <span class="kw">let </span><span class="kw-2">mut </span>waiters = <span class="self">self</span>.waiters.lock();

        <span class="comment">// check for AsyncRead slot
        </span><span class="kw">if </span>ready.is_readable() {
            <span class="kw">if let </span><span class="prelude-val">Some</span>(waker) = waiters.reader.take() {
                wakers.push(waker);
            }
        }

        <span class="comment">// check for AsyncWrite slot
        </span><span class="kw">if </span>ready.is_writable() {
            <span class="kw">if let </span><span class="prelude-val">Some</span>(waker) = waiters.writer.take() {
                wakers.push(waker);
            }
        }

        <span class="lifetime">'outer</span>: <span class="kw">loop </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>iter = waiters.list.drain_filter(|w| ready.satisfies(w.interest));

            <span class="kw">while </span>wakers.can_push() {
                <span class="kw">match </span>iter.next() {
                    <span class="prelude-val">Some</span>(waiter) =&gt; {
                        <span class="kw">let </span>waiter = <span class="kw">unsafe </span>{ <span class="kw-2">&amp;mut *</span>waiter.as_ptr() };

                        <span class="kw">if let </span><span class="prelude-val">Some</span>(waker) = waiter.waker.take() {
                            waiter.is_ready = <span class="bool-val">true</span>;
                            wakers.push(waker);
                        }
                    }
                    <span class="prelude-val">None </span>=&gt; {
                        <span class="kw">break </span><span class="lifetime">'outer</span>;
                    }
                }
            }

            drop(waiters);

            wakers.wake_all();

            <span class="comment">// Acquire the lock again.
            </span>waiters = <span class="self">self</span>.waiters.lock();
        }

        <span class="comment">// Release the lock before notifying
        </span>drop(waiters);

        wakers.wake_all();
    }

    <span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>ready_event(<span class="kw-2">&amp;</span><span class="self">self</span>, interest: Interest) -&gt; ReadyEvent {
        <span class="kw">let </span>curr = <span class="self">self</span>.readiness.load(Acquire);

        ReadyEvent {
            tick: TICK.unpack(curr) <span class="kw">as </span>u8,
            ready: interest.mask() &amp; Ready::from_usize(READINESS.unpack(curr)),
            is_shutdown: SHUTDOWN.unpack(curr) != <span class="number">0</span>,
        }
    }

    <span class="doccomment">/// Polls for readiness events in a given direction.
    ///
    /// These are to support `AsyncRead` and `AsyncWrite` polling methods,
    /// which cannot use the `async fn` version. This uses reserved reader
    /// and writer slots.
    </span><span class="kw">pub</span>(<span class="kw">super</span>) <span class="kw">fn </span>poll_readiness(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        cx: <span class="kw-2">&amp;mut </span>Context&lt;<span class="lifetime">'_</span>&gt;,
        direction: Direction,
    ) -&gt; Poll&lt;ReadyEvent&gt; {
        <span class="kw">let </span>curr = <span class="self">self</span>.readiness.load(Acquire);

        <span class="kw">let </span>ready = direction.mask() &amp; Ready::from_usize(READINESS.unpack(curr));
        <span class="kw">let </span>is_shutdown = SHUTDOWN.unpack(curr) != <span class="number">0</span>;

        <span class="kw">if </span>ready.is_empty() &amp;&amp; !is_shutdown {
            <span class="comment">// Update the task info
            </span><span class="kw">let </span><span class="kw-2">mut </span>waiters = <span class="self">self</span>.waiters.lock();
            <span class="kw">let </span>slot = <span class="kw">match </span>direction {
                Direction::Read =&gt; <span class="kw-2">&amp;mut </span>waiters.reader,
                Direction::Write =&gt; <span class="kw-2">&amp;mut </span>waiters.writer,
            };

            <span class="comment">// Avoid cloning the waker if one is already stored that matches the
            // current task.
            </span><span class="kw">match </span>slot {
                <span class="prelude-val">Some</span>(existing) =&gt; {
                    <span class="kw">if </span>!existing.will_wake(cx.waker()) {
                        <span class="kw-2">*</span>existing = cx.waker().clone();
                    }
                }
                <span class="prelude-val">None </span>=&gt; {
                    <span class="kw-2">*</span>slot = <span class="prelude-val">Some</span>(cx.waker().clone());
                }
            }

            <span class="comment">// Try again, in case the readiness was changed while we were
            // taking the waiters lock
            </span><span class="kw">let </span>curr = <span class="self">self</span>.readiness.load(Acquire);
            <span class="kw">let </span>ready = direction.mask() &amp; Ready::from_usize(READINESS.unpack(curr));
            <span class="kw">let </span>is_shutdown = SHUTDOWN.unpack(curr) != <span class="number">0</span>;
            <span class="kw">if </span>is_shutdown {
                Poll::Ready(ReadyEvent {
                    tick: TICK.unpack(curr) <span class="kw">as </span>u8,
                    ready: direction.mask(),
                    is_shutdown,
                })
            } <span class="kw">else if </span>ready.is_empty() {
                Poll::Pending
            } <span class="kw">else </span>{
                Poll::Ready(ReadyEvent {
                    tick: TICK.unpack(curr) <span class="kw">as </span>u8,
                    ready,
                    is_shutdown,
                })
            }
        } <span class="kw">else </span>{
            Poll::Ready(ReadyEvent {
                tick: TICK.unpack(curr) <span class="kw">as </span>u8,
                ready,
                is_shutdown,
            })
        }
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>clear_readiness(<span class="kw-2">&amp;</span><span class="self">self</span>, event: ReadyEvent) {
        <span class="comment">// This consumes the current readiness state **except** for closed
        // states. Closed states are excluded because they are final states.
        </span><span class="kw">let </span>mask_no_closed = event.ready - Ready::READ_CLOSED - Ready::WRITE_CLOSED;
        <span class="self">self</span>.set_readiness(Tick::Clear(event.tick), |curr| curr - mask_no_closed);
    }

    <span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>clear_wakers(<span class="kw-2">&amp;</span><span class="self">self</span>) {
        <span class="kw">let </span><span class="kw-2">mut </span>waiters = <span class="self">self</span>.waiters.lock();
        waiters.reader.take();
        waiters.writer.take();
    }
}

<span class="kw">impl </span>Drop <span class="kw">for </span>ScheduledIo {
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.wake(Ready::ALL);
    }
}

<span class="kw">unsafe impl </span>Send <span class="kw">for </span>ScheduledIo {}
<span class="kw">unsafe impl </span>Sync <span class="kw">for </span>ScheduledIo {}

<span class="kw">impl </span>ScheduledIo {
    <span class="doccomment">/// An async version of `poll_readiness` which uses a linked list of wakers.
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">async fn </span>readiness(<span class="kw-2">&amp;</span><span class="self">self</span>, interest: Interest) -&gt; ReadyEvent {
        <span class="self">self</span>.readiness_fut(interest).<span class="kw">await
    </span>}

    <span class="comment">// This is in a separate function so that the borrow checker doesn't think
    // we are borrowing the `UnsafeCell` possibly over await boundaries.
    //
    // Go figure.
    </span><span class="kw">fn </span>readiness_fut(<span class="kw-2">&amp;</span><span class="self">self</span>, interest: Interest) -&gt; Readiness&lt;<span class="lifetime">'_</span>&gt; {
        Readiness {
            scheduled_io: <span class="self">self</span>,
            state: State::Init,
            waiter: UnsafeCell::new(Waiter {
                pointers: linked_list::Pointers::new(),
                waker: <span class="prelude-val">None</span>,
                is_ready: <span class="bool-val">false</span>,
                interest,
                _p: PhantomPinned,
            }),
        }
    }
}

<span class="kw">unsafe impl </span>linked_list::Link <span class="kw">for </span>Waiter {
    <span class="kw">type </span>Handle = NonNull&lt;Waiter&gt;;
    <span class="kw">type </span>Target = Waiter;

    <span class="kw">fn </span>as_raw(handle: <span class="kw-2">&amp;</span>NonNull&lt;Waiter&gt;) -&gt; NonNull&lt;Waiter&gt; {
        <span class="kw-2">*</span>handle
    }

    <span class="kw">unsafe fn </span>from_raw(ptr: NonNull&lt;Waiter&gt;) -&gt; NonNull&lt;Waiter&gt; {
        ptr
    }

    <span class="kw">unsafe fn </span>pointers(target: NonNull&lt;Waiter&gt;) -&gt; NonNull&lt;linked_list::Pointers&lt;Waiter&gt;&gt; {
        Waiter::addr_of_pointers(target)
    }
}

<span class="comment">// ===== impl Readiness =====

</span><span class="kw">impl </span>Future <span class="kw">for </span>Readiness&lt;<span class="lifetime">'_</span>&gt; {
    <span class="kw">type </span>Output = ReadyEvent;

    <span class="kw">fn </span>poll(<span class="self">self</span>: Pin&lt;<span class="kw-2">&amp;mut </span><span class="self">Self</span>&gt;, cx: <span class="kw-2">&amp;mut </span>Context&lt;<span class="lifetime">'_</span>&gt;) -&gt; Poll&lt;<span class="self">Self</span>::Output&gt; {
        <span class="kw">use </span>std::sync::atomic::Ordering::SeqCst;

        <span class="kw">let </span>(scheduled_io, state, waiter) = <span class="kw">unsafe </span>{
            <span class="kw">let </span>me = <span class="self">self</span>.get_unchecked_mut();
            (<span class="kw-2">&amp;</span>me.scheduled_io, <span class="kw-2">&amp;mut </span>me.state, <span class="kw-2">&amp;</span>me.waiter)
        };

        <span class="kw">loop </span>{
            <span class="kw">match </span><span class="kw-2">*</span>state {
                State::Init =&gt; {
                    <span class="comment">// Optimistically check existing readiness
                    </span><span class="kw">let </span>curr = scheduled_io.readiness.load(SeqCst);
                    <span class="kw">let </span>ready = Ready::from_usize(READINESS.unpack(curr));
                    <span class="kw">let </span>is_shutdown = SHUTDOWN.unpack(curr) != <span class="number">0</span>;

                    <span class="comment">// Safety: `waiter.interest` never changes
                    </span><span class="kw">let </span>interest = <span class="kw">unsafe </span>{ (<span class="kw-2">*</span>waiter.get()).interest };
                    <span class="kw">let </span>ready = ready.intersection(interest);

                    <span class="kw">if </span>!ready.is_empty() || is_shutdown {
                        <span class="comment">// Currently ready!
                        </span><span class="kw">let </span>tick = TICK.unpack(curr) <span class="kw">as </span>u8;
                        <span class="kw-2">*</span>state = State::Done;
                        <span class="kw">return </span>Poll::Ready(ReadyEvent {
                            tick,
                            ready,
                            is_shutdown,
                        });
                    }

                    <span class="comment">// Wasn't ready, take the lock (and check again while locked).
                    </span><span class="kw">let </span><span class="kw-2">mut </span>waiters = scheduled_io.waiters.lock();

                    <span class="kw">let </span>curr = scheduled_io.readiness.load(SeqCst);
                    <span class="kw">let </span><span class="kw-2">mut </span>ready = Ready::from_usize(READINESS.unpack(curr));
                    <span class="kw">let </span>is_shutdown = SHUTDOWN.unpack(curr) != <span class="number">0</span>;

                    <span class="kw">if </span>is_shutdown {
                        ready = Ready::ALL;
                    }

                    <span class="kw">let </span>ready = ready.intersection(interest);

                    <span class="kw">if </span>!ready.is_empty() || is_shutdown {
                        <span class="comment">// Currently ready!
                        </span><span class="kw">let </span>tick = TICK.unpack(curr) <span class="kw">as </span>u8;
                        <span class="kw-2">*</span>state = State::Done;
                        <span class="kw">return </span>Poll::Ready(ReadyEvent {
                            tick,
                            ready,
                            is_shutdown,
                        });
                    }

                    <span class="comment">// Not ready even after locked, insert into list...

                    // Safety: called while locked
                    </span><span class="kw">unsafe </span>{
                        (<span class="kw-2">*</span>waiter.get()).waker = <span class="prelude-val">Some</span>(cx.waker().clone());
                    }

                    <span class="comment">// Insert the waiter into the linked list
                    //
                    // safety: pointers from `UnsafeCell` are never null.
                    </span>waiters
                        .list
                        .push_front(<span class="kw">unsafe </span>{ NonNull::new_unchecked(waiter.get()) });
                    <span class="kw-2">*</span>state = State::Waiting;
                }
                State::Waiting =&gt; {
                    <span class="comment">// Currently in the "Waiting" state, implying the caller has
                    // a waiter stored in the waiter list (guarded by
                    // `notify.waiters`). In order to access the waker fields,
                    // we must hold the lock.

                    </span><span class="kw">let </span>waiters = scheduled_io.waiters.lock();

                    <span class="comment">// Safety: called while locked
                    </span><span class="kw">let </span>w = <span class="kw">unsafe </span>{ <span class="kw-2">&amp;mut *</span>waiter.get() };

                    <span class="kw">if </span>w.is_ready {
                        <span class="comment">// Our waker has been notified.
                        </span><span class="kw-2">*</span>state = State::Done;
                    } <span class="kw">else </span>{
                        <span class="comment">// Update the waker, if necessary.
                        </span><span class="kw">if </span>!w.waker.as_ref().unwrap().will_wake(cx.waker()) {
                            w.waker = <span class="prelude-val">Some</span>(cx.waker().clone());
                        }

                        <span class="kw">return </span>Poll::Pending;
                    }

                    <span class="comment">// Explicit drop of the lock to indicate the scope that the
                    // lock is held. Because holding the lock is required to
                    // ensure safe access to fields not held within the lock, it
                    // is helpful to visualize the scope of the critical
                    // section.
                    </span>drop(waiters);
                }
                State::Done =&gt; {
                    <span class="comment">// Safety: State::Done means it is no longer shared
                    </span><span class="kw">let </span>w = <span class="kw">unsafe </span>{ <span class="kw-2">&amp;mut *</span>waiter.get() };

                    <span class="kw">let </span>curr = scheduled_io.readiness.load(Acquire);
                    <span class="kw">let </span>is_shutdown = SHUTDOWN.unpack(curr) != <span class="number">0</span>;

                    <span class="comment">// The returned tick might be newer than the event
                    // which notified our waker. This is ok because the future
                    // still didn't return `Poll::Ready`.
                    </span><span class="kw">let </span>tick = TICK.unpack(curr) <span class="kw">as </span>u8;

                    <span class="comment">// The readiness state could have been cleared in the meantime,
                    // but we allow the returned ready set to be empty.
                    </span><span class="kw">let </span>curr_ready = Ready::from_usize(READINESS.unpack(curr));
                    <span class="kw">let </span>ready = curr_ready.intersection(w.interest);

                    <span class="kw">return </span>Poll::Ready(ReadyEvent {
                        tick,
                        ready,
                        is_shutdown,
                    });
                }
            }
        }
    }
}

<span class="kw">impl </span>Drop <span class="kw">for </span>Readiness&lt;<span class="lifetime">'_</span>&gt; {
    <span class="kw">fn </span>drop(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">let </span><span class="kw-2">mut </span>waiters = <span class="self">self</span>.scheduled_io.waiters.lock();

        <span class="comment">// Safety: `waiter` is only ever stored in `waiters`
        </span><span class="kw">unsafe </span>{
            waiters
                .list
                .remove(NonNull::new_unchecked(<span class="self">self</span>.waiter.get()))
        };
    }
}

<span class="kw">unsafe impl </span>Send <span class="kw">for </span>Readiness&lt;<span class="lifetime">'_</span>&gt; {}
<span class="kw">unsafe impl </span>Sync <span class="kw">for </span>Readiness&lt;<span class="lifetime">'_</span>&gt; {}
</code></pre></div></section></main></body></html>