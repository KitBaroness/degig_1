<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana_rbpf-0.8.0/src/ebpf.rs`."><title>ebpf.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="solana_rbpf" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/qmonnet/rbpf/master/misc/rbpf.ico"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../solana_rbpf/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="attr">#![allow(clippy::arithmetic_side_effects)]
</span><span class="comment">// Copyright 2016 6WIND S.A. &lt;quentin.monnet@6wind.com&gt;
//
// Licensed under the Apache License, Version 2.0 &lt;http://www.apache.org/licenses/LICENSE-2.0&gt; or
// the MIT license &lt;http://opensource.org/licenses/MIT&gt;, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

</span><span class="doccomment">//! This module contains all the definitions related to eBPF, and some functions permitting to
//! manipulate eBPF instructions.
//!
//! The number of bytes in an instruction, the maximum number of instructions in a program, and
//! also all operation codes are defined here as constants.
//!
//! The structure for an instruction used by this crate, as well as the function to extract it from
//! a program, is also defined in the module.
//!
//! To learn more about these instructions, see the Linux kernel documentation:
//! &lt;https://www.kernel.org/doc/Documentation/networking/filter.txt&gt;, or for a shorter version of
//! the list of the operation codes: &lt;https://github.com/iovisor/bpf-docs/blob/master/eBPF.md&gt;

</span><span class="kw">use </span>byteorder::{ByteOrder, LittleEndian};
<span class="kw">use </span>hash32::{Hash, Hasher, Murmur3Hasher};
<span class="kw">use </span>std::fmt;

<span class="doccomment">/// Solana BPF version flag
</span><span class="kw">pub const </span>EF_SBPF_V2: u32 = <span class="number">0x20</span>;
<span class="doccomment">/// Maximum number of instructions in an eBPF program.
</span><span class="kw">pub const </span>PROG_MAX_INSNS: usize = <span class="number">65_536</span>;
<span class="doccomment">/// Size of an eBPF instructions, in bytes.
</span><span class="kw">pub const </span>INSN_SIZE: usize = <span class="number">8</span>;
<span class="doccomment">/// Frame pointer register
</span><span class="kw">pub const </span>FRAME_PTR_REG: usize = <span class="number">10</span>;
<span class="doccomment">/// Stack pointer register
</span><span class="kw">pub const </span>STACK_PTR_REG: usize = <span class="number">11</span>;
<span class="doccomment">/// First scratch register
</span><span class="kw">pub const </span>FIRST_SCRATCH_REG: usize = <span class="number">6</span>;
<span class="doccomment">/// Number of scratch registers
</span><span class="kw">pub const </span>SCRATCH_REGS: usize = <span class="number">4</span>;
<span class="doccomment">/// Alignment of the memory regions in host address space in bytes
</span><span class="kw">pub const </span>HOST_ALIGN: usize = <span class="number">16</span>;
<span class="doccomment">/// Upper half of a pointer is the region index, lower half the virtual address inside that region.
</span><span class="kw">pub const </span>VIRTUAL_ADDRESS_BITS: usize = <span class="number">32</span>;

<span class="comment">// Memory map regions virtual addresses need to be (1 &lt;&lt; VIRTUAL_ADDRESS_BITS) bytes apart.
// Also the region at index 0 should be skipped to catch NULL ptr accesses.

</span><span class="doccomment">/// Start of the program bits (text and ro segments) in the memory map
</span><span class="kw">pub const </span>MM_PROGRAM_START: u64 = <span class="number">0x100000000</span>;
<span class="doccomment">/// Start of the stack in the memory map
</span><span class="kw">pub const </span>MM_STACK_START: u64 = <span class="number">0x200000000</span>;
<span class="doccomment">/// Start of the heap in the memory map
</span><span class="kw">pub const </span>MM_HEAP_START: u64 = <span class="number">0x300000000</span>;
<span class="doccomment">/// Start of the input buffers in the memory map
</span><span class="kw">pub const </span>MM_INPUT_START: u64 = <span class="number">0x400000000</span>;

<span class="comment">// eBPF op codes.
// See also https://www.kernel.org/doc/Documentation/networking/filter.txt

// Three least significant bits are operation class:
</span><span class="doccomment">/// BPF operation class: load from immediate. [DEPRECATED]
</span><span class="kw">pub const </span>BPF_LD: u8 = <span class="number">0x00</span>;
<span class="doccomment">/// BPF operation class: load from register.
</span><span class="kw">pub const </span>BPF_LDX: u8 = <span class="number">0x01</span>;
<span class="doccomment">/// BPF operation class: store immediate.
</span><span class="kw">pub const </span>BPF_ST: u8 = <span class="number">0x02</span>;
<span class="doccomment">/// BPF operation class: store value from register.
</span><span class="kw">pub const </span>BPF_STX: u8 = <span class="number">0x03</span>;
<span class="doccomment">/// BPF operation class: 32 bits arithmetic operation.
</span><span class="kw">pub const </span>BPF_ALU: u8 = <span class="number">0x04</span>;
<span class="doccomment">/// BPF operation class: jump.
</span><span class="kw">pub const </span>BPF_JMP: u8 = <span class="number">0x05</span>;
<span class="doccomment">/// BPF operation class: product / quotient / remainder.
</span><span class="kw">pub const </span>BPF_PQR: u8 = <span class="number">0x06</span>;
<span class="doccomment">/// BPF operation class: 64 bits arithmetic operation.
</span><span class="kw">pub const </span>BPF_ALU64: u8 = <span class="number">0x07</span>;

<span class="comment">// For load and store instructions:
// +------------+--------+------------+
// |   3 bits   | 2 bits |   3 bits   |
// |    mode    |  size  | insn class |
// +------------+--------+------------+
// (MSB)                          (LSB)

// Size modifiers:
</span><span class="doccomment">/// BPF size modifier: word (4 bytes).
</span><span class="kw">pub const </span>BPF_W: u8 = <span class="number">0x00</span>;
<span class="doccomment">/// BPF size modifier: half-word (2 bytes).
</span><span class="kw">pub const </span>BPF_H: u8 = <span class="number">0x08</span>;
<span class="doccomment">/// BPF size modifier: byte (1 byte).
</span><span class="kw">pub const </span>BPF_B: u8 = <span class="number">0x10</span>;
<span class="doccomment">/// BPF size modifier: double word (8 bytes).
</span><span class="kw">pub const </span>BPF_DW: u8 = <span class="number">0x18</span>;

<span class="comment">// Mode modifiers:
</span><span class="doccomment">/// BPF mode modifier: immediate value.
</span><span class="kw">pub const </span>BPF_IMM: u8 = <span class="number">0x00</span>;
<span class="doccomment">/// BPF mode modifier: absolute load.
</span><span class="kw">pub const </span>BPF_ABS: u8 = <span class="number">0x20</span>;
<span class="doccomment">/// BPF mode modifier: indirect load.
</span><span class="kw">pub const </span>BPF_IND: u8 = <span class="number">0x40</span>;
<span class="doccomment">/// BPF mode modifier: load from / store to memory.
</span><span class="kw">pub const </span>BPF_MEM: u8 = <span class="number">0x60</span>;
<span class="comment">// [ 0x80 reserved ]
// [ 0xa0 reserved ]
// [ 0xc0 reserved ]

// For arithmetic (BPF_ALU/BPF_ALU64) and jump (BPF_JMP) instructions:
// +----------------+--------+--------+
// |     4 bits     |1 b.|   3 bits   |
// | operation code | src| insn class |
// +----------------+----+------------+
// (MSB)                          (LSB)

// Source modifiers:
</span><span class="doccomment">/// BPF source operand modifier: 32-bit immediate value.
</span><span class="kw">pub const </span>BPF_K: u8 = <span class="number">0x00</span>;
<span class="doccomment">/// BPF source operand modifier: `src` register.
</span><span class="kw">pub const </span>BPF_X: u8 = <span class="number">0x08</span>;

<span class="comment">// Operation codes -- BPF_ALU or BPF_ALU64 classes:
</span><span class="doccomment">/// BPF ALU/ALU64 operation code: addition.
</span><span class="kw">pub const </span>BPF_ADD: u8 = <span class="number">0x00</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: subtraction.
</span><span class="kw">pub const </span>BPF_SUB: u8 = <span class="number">0x10</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: multiplication. [DEPRECATED]
</span><span class="kw">pub const </span>BPF_MUL: u8 = <span class="number">0x20</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: division. [DEPRECATED]
</span><span class="kw">pub const </span>BPF_DIV: u8 = <span class="number">0x30</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: or.
</span><span class="kw">pub const </span>BPF_OR: u8 = <span class="number">0x40</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: and.
</span><span class="kw">pub const </span>BPF_AND: u8 = <span class="number">0x50</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: left shift.
</span><span class="kw">pub const </span>BPF_LSH: u8 = <span class="number">0x60</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: right shift.
</span><span class="kw">pub const </span>BPF_RSH: u8 = <span class="number">0x70</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: negation. [DEPRECATED]
</span><span class="kw">pub const </span>BPF_NEG: u8 = <span class="number">0x80</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: modulus. [DEPRECATED]
</span><span class="kw">pub const </span>BPF_MOD: u8 = <span class="number">0x90</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: exclusive or.
</span><span class="kw">pub const </span>BPF_XOR: u8 = <span class="number">0xa0</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: move.
</span><span class="kw">pub const </span>BPF_MOV: u8 = <span class="number">0xb0</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: sign extending right shift.
</span><span class="kw">pub const </span>BPF_ARSH: u8 = <span class="number">0xc0</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: endianness conversion.
</span><span class="kw">pub const </span>BPF_END: u8 = <span class="number">0xd0</span>;
<span class="doccomment">/// BPF ALU/ALU64 operation code: high or.
</span><span class="kw">pub const </span>BPF_HOR: u8 = <span class="number">0xf0</span>;

<span class="comment">// Operation codes -- BPF_PQR class:
//    7         6               5                               4       3          2-0
// 0  Unsigned  Multiplication  Product Lower Half / Quotient   32 Bit  Immediate  PQR
// 1  Signed    Division        Product Upper Half / Remainder  64 Bit  Register   PQR
</span><span class="doccomment">/// BPF PQR operation code: unsigned high multiplication.
</span><span class="kw">pub const </span>BPF_UHMUL: u8 = <span class="number">0x20</span>;
<span class="doccomment">/// BPF PQR operation code: unsigned division quotient.
</span><span class="kw">pub const </span>BPF_UDIV: u8 = <span class="number">0x40</span>;
<span class="doccomment">/// BPF PQR operation code: unsigned division remainder.
</span><span class="kw">pub const </span>BPF_UREM: u8 = <span class="number">0x60</span>;
<span class="doccomment">/// BPF PQR operation code: low multiplication.
</span><span class="kw">pub const </span>BPF_LMUL: u8 = <span class="number">0x80</span>;
<span class="doccomment">/// BPF PQR operation code: signed high multiplication.
</span><span class="kw">pub const </span>BPF_SHMUL: u8 = <span class="number">0xA0</span>;
<span class="doccomment">/// BPF PQR operation code: signed division quotient.
</span><span class="kw">pub const </span>BPF_SDIV: u8 = <span class="number">0xC0</span>;
<span class="doccomment">/// BPF PQR operation code: signed division remainder.
</span><span class="kw">pub const </span>BPF_SREM: u8 = <span class="number">0xE0</span>;

<span class="comment">// Operation codes -- BPF_JMP class:
</span><span class="doccomment">/// BPF JMP operation code: jump.
</span><span class="kw">pub const </span>BPF_JA: u8 = <span class="number">0x00</span>;
<span class="doccomment">/// BPF JMP operation code: jump if equal.
</span><span class="kw">pub const </span>BPF_JEQ: u8 = <span class="number">0x10</span>;
<span class="doccomment">/// BPF JMP operation code: jump if greater than.
</span><span class="kw">pub const </span>BPF_JGT: u8 = <span class="number">0x20</span>;
<span class="doccomment">/// BPF JMP operation code: jump if greater or equal.
</span><span class="kw">pub const </span>BPF_JGE: u8 = <span class="number">0x30</span>;
<span class="doccomment">/// BPF JMP operation code: jump if `src` &amp; `reg`.
</span><span class="kw">pub const </span>BPF_JSET: u8 = <span class="number">0x40</span>;
<span class="doccomment">/// BPF JMP operation code: jump if not equal.
</span><span class="kw">pub const </span>BPF_JNE: u8 = <span class="number">0x50</span>;
<span class="doccomment">/// BPF JMP operation code: jump if greater than (signed).
</span><span class="kw">pub const </span>BPF_JSGT: u8 = <span class="number">0x60</span>;
<span class="doccomment">/// BPF JMP operation code: jump if greater or equal (signed).
</span><span class="kw">pub const </span>BPF_JSGE: u8 = <span class="number">0x70</span>;
<span class="doccomment">/// BPF JMP operation code: syscall function call.
</span><span class="kw">pub const </span>BPF_CALL: u8 = <span class="number">0x80</span>;
<span class="doccomment">/// BPF JMP operation code: return from program.
</span><span class="kw">pub const </span>BPF_EXIT: u8 = <span class="number">0x90</span>;
<span class="doccomment">/// BPF JMP operation code: jump if lower than.
</span><span class="kw">pub const </span>BPF_JLT: u8 = <span class="number">0xa0</span>;
<span class="doccomment">/// BPF JMP operation code: jump if lower or equal.
</span><span class="kw">pub const </span>BPF_JLE: u8 = <span class="number">0xb0</span>;
<span class="doccomment">/// BPF JMP operation code: jump if lower than (signed).
</span><span class="kw">pub const </span>BPF_JSLT: u8 = <span class="number">0xc0</span>;
<span class="doccomment">/// BPF JMP operation code: jump if lower or equal (signed).
</span><span class="kw">pub const </span>BPF_JSLE: u8 = <span class="number">0xd0</span>;

<span class="comment">// Op codes
// (Following operation names are not “official”, but may be proper to rbpf; Linux kernel only
// combines above flags and does not attribute a name per operation.)

</span><span class="doccomment">/// BPF opcode: `lddw dst, imm` /// `dst = imm`. [DEPRECATED]
</span><span class="kw">pub const </span>LD_DW_IMM: u8 = BPF_LD | BPF_IMM | BPF_DW;
<span class="doccomment">/// BPF opcode: `ldxb dst, [src + off]` /// `dst = (src + off) as u8`.
</span><span class="kw">pub const </span>LD_B_REG: u8 = BPF_LDX | BPF_MEM | BPF_B;
<span class="doccomment">/// BPF opcode: `ldxh dst, [src + off]` /// `dst = (src + off) as u16`.
</span><span class="kw">pub const </span>LD_H_REG: u8 = BPF_LDX | BPF_MEM | BPF_H;
<span class="doccomment">/// BPF opcode: `ldxw dst, [src + off]` /// `dst = (src + off) as u32`.
</span><span class="kw">pub const </span>LD_W_REG: u8 = BPF_LDX | BPF_MEM | BPF_W;
<span class="doccomment">/// BPF opcode: `ldxdw dst, [src + off]` /// `dst = (src + off) as u64`.
</span><span class="kw">pub const </span>LD_DW_REG: u8 = BPF_LDX | BPF_MEM | BPF_DW;
<span class="doccomment">/// BPF opcode: `stb [dst + off], imm` /// `(dst + offset) as u8 = imm`.
</span><span class="kw">pub const </span>ST_B_IMM: u8 = BPF_ST | BPF_MEM | BPF_B;
<span class="doccomment">/// BPF opcode: `sth [dst + off], imm` /// `(dst + offset) as u16 = imm`.
</span><span class="kw">pub const </span>ST_H_IMM: u8 = BPF_ST | BPF_MEM | BPF_H;
<span class="doccomment">/// BPF opcode: `stw [dst + off], imm` /// `(dst + offset) as u32 = imm`.
</span><span class="kw">pub const </span>ST_W_IMM: u8 = BPF_ST | BPF_MEM | BPF_W;
<span class="doccomment">/// BPF opcode: `stdw [dst + off], imm` /// `(dst + offset) as u64 = imm`.
</span><span class="kw">pub const </span>ST_DW_IMM: u8 = BPF_ST | BPF_MEM | BPF_DW;
<span class="doccomment">/// BPF opcode: `stxb [dst + off], src` /// `(dst + offset) as u8 = src`.
</span><span class="kw">pub const </span>ST_B_REG: u8 = BPF_STX | BPF_MEM | BPF_B;
<span class="doccomment">/// BPF opcode: `stxh [dst + off], src` /// `(dst + offset) as u16 = src`.
</span><span class="kw">pub const </span>ST_H_REG: u8 = BPF_STX | BPF_MEM | BPF_H;
<span class="doccomment">/// BPF opcode: `stxw [dst + off], src` /// `(dst + offset) as u32 = src`.
</span><span class="kw">pub const </span>ST_W_REG: u8 = BPF_STX | BPF_MEM | BPF_W;
<span class="doccomment">/// BPF opcode: `stxdw [dst + off], src` /// `(dst + offset) as u64 = src`.
</span><span class="kw">pub const </span>ST_DW_REG: u8 = BPF_STX | BPF_MEM | BPF_DW;

<span class="doccomment">/// BPF opcode: `add32 dst, imm` /// `dst += imm`.
</span><span class="kw">pub const </span>ADD32_IMM: u8 = BPF_ALU | BPF_K | BPF_ADD;
<span class="doccomment">/// BPF opcode: `add32 dst, src` /// `dst += src`.
</span><span class="kw">pub const </span>ADD32_REG: u8 = BPF_ALU | BPF_X | BPF_ADD;
<span class="doccomment">/// BPF opcode: `sub32 dst, imm` /// `dst = imm - dst`.
</span><span class="kw">pub const </span>SUB32_IMM: u8 = BPF_ALU | BPF_K | BPF_SUB;
<span class="doccomment">/// BPF opcode: `sub32 dst, src` /// `dst -= src`.
</span><span class="kw">pub const </span>SUB32_REG: u8 = BPF_ALU | BPF_X | BPF_SUB;
<span class="doccomment">/// BPF opcode: `mul32 dst, imm` /// `dst *= imm`.
</span><span class="kw">pub const </span>MUL32_IMM: u8 = BPF_ALU | BPF_K | BPF_MUL;
<span class="doccomment">/// BPF opcode: `mul32 dst, src` /// `dst *= src`.
</span><span class="kw">pub const </span>MUL32_REG: u8 = BPF_ALU | BPF_X | BPF_MUL;
<span class="doccomment">/// BPF opcode: `div32 dst, imm` /// `dst /= imm`.
</span><span class="kw">pub const </span>DIV32_IMM: u8 = BPF_ALU | BPF_K | BPF_DIV;
<span class="doccomment">/// BPF opcode: `div32 dst, src` /// `dst /= src`.
</span><span class="kw">pub const </span>DIV32_REG: u8 = BPF_ALU | BPF_X | BPF_DIV;
<span class="doccomment">/// BPF opcode: `or32 dst, imm` /// `dst |= imm`.
</span><span class="kw">pub const </span>OR32_IMM: u8 = BPF_ALU | BPF_K | BPF_OR;
<span class="doccomment">/// BPF opcode: `or32 dst, src` /// `dst |= src`.
</span><span class="kw">pub const </span>OR32_REG: u8 = BPF_ALU | BPF_X | BPF_OR;
<span class="doccomment">/// BPF opcode: `and32 dst, imm` /// `dst &amp;= imm`.
</span><span class="kw">pub const </span>AND32_IMM: u8 = BPF_ALU | BPF_K | BPF_AND;
<span class="doccomment">/// BPF opcode: `and32 dst, src` /// `dst &amp;= src`.
</span><span class="kw">pub const </span>AND32_REG: u8 = BPF_ALU | BPF_X | BPF_AND;
<span class="doccomment">/// BPF opcode: `lsh32 dst, imm` /// `dst &lt;&lt;= imm`.
</span><span class="kw">pub const </span>LSH32_IMM: u8 = BPF_ALU | BPF_K | BPF_LSH;
<span class="doccomment">/// BPF opcode: `lsh32 dst, src` /// `dst &lt;&lt;= src`.
</span><span class="kw">pub const </span>LSH32_REG: u8 = BPF_ALU | BPF_X | BPF_LSH;
<span class="doccomment">/// BPF opcode: `rsh32 dst, imm` /// `dst &gt;&gt;= imm`.
</span><span class="kw">pub const </span>RSH32_IMM: u8 = BPF_ALU | BPF_K | BPF_RSH;
<span class="doccomment">/// BPF opcode: `rsh32 dst, src` /// `dst &gt;&gt;= src`.
</span><span class="kw">pub const </span>RSH32_REG: u8 = BPF_ALU | BPF_X | BPF_RSH;
<span class="doccomment">/// BPF opcode: `neg32 dst` /// `dst = -dst`.
</span><span class="kw">pub const </span>NEG32: u8 = BPF_ALU | BPF_NEG;
<span class="doccomment">/// BPF opcode: `mod32 dst, imm` /// `dst %= imm`.
</span><span class="kw">pub const </span>MOD32_IMM: u8 = BPF_ALU | BPF_K | BPF_MOD;
<span class="doccomment">/// BPF opcode: `mod32 dst, src` /// `dst %= src`.
</span><span class="kw">pub const </span>MOD32_REG: u8 = BPF_ALU | BPF_X | BPF_MOD;
<span class="doccomment">/// BPF opcode: `xor32 dst, imm` /// `dst ^= imm`.
</span><span class="kw">pub const </span>XOR32_IMM: u8 = BPF_ALU | BPF_K | BPF_XOR;
<span class="doccomment">/// BPF opcode: `xor32 dst, src` /// `dst ^= src`.
</span><span class="kw">pub const </span>XOR32_REG: u8 = BPF_ALU | BPF_X | BPF_XOR;
<span class="doccomment">/// BPF opcode: `mov32 dst, imm` /// `dst = imm`.
</span><span class="kw">pub const </span>MOV32_IMM: u8 = BPF_ALU | BPF_K | BPF_MOV;
<span class="doccomment">/// BPF opcode: `mov32 dst, src` /// `dst = src`.
</span><span class="kw">pub const </span>MOV32_REG: u8 = BPF_ALU | BPF_X | BPF_MOV;
<span class="doccomment">/// BPF opcode: `arsh32 dst, imm` /// `dst &gt;&gt;= imm (arithmetic)`.
</span><span class="kw">pub const </span>ARSH32_IMM: u8 = BPF_ALU | BPF_K | BPF_ARSH;
<span class="doccomment">/// BPF opcode: `arsh32 dst, src` /// `dst &gt;&gt;= src (arithmetic)`.
</span><span class="kw">pub const </span>ARSH32_REG: u8 = BPF_ALU | BPF_X | BPF_ARSH;

<span class="doccomment">/// BPF opcode: `lmul32 dst, imm` /// `dst *= (dst * imm) as u32`.
</span><span class="kw">pub const </span>LMUL32_IMM: u8 = BPF_PQR | BPF_K | BPF_LMUL;
<span class="doccomment">/// BPF opcode: `lmul32 dst, src` /// `dst *= (dst * src) as u32`.
</span><span class="kw">pub const </span>LMUL32_REG: u8 = BPF_PQR | BPF_X | BPF_LMUL;
<span class="doccomment">/// BPF opcode: `uhmul32 dst, imm` /// `dst = (dst * imm) as u64`.
</span><span class="comment">// pub const UHMUL32_IMM: u8 = BPF_PQR | BPF_K | BPF_UHMUL;
</span><span class="doccomment">/// BPF opcode: `uhmul32 dst, src` /// `dst = (dst * src) as u64`.
</span><span class="comment">// pub const UHMUL32_REG: u8 = BPF_PQR | BPF_X | BPF_UHMUL;
</span><span class="doccomment">/// BPF opcode: `udiv32 dst, imm` /// `dst /= imm`.
</span><span class="kw">pub const </span>UDIV32_IMM: u8 = BPF_PQR | BPF_K | BPF_UDIV;
<span class="doccomment">/// BPF opcode: `udiv32 dst, src` /// `dst /= src`.
</span><span class="kw">pub const </span>UDIV32_REG: u8 = BPF_PQR | BPF_X | BPF_UDIV;
<span class="doccomment">/// BPF opcode: `urem32 dst, imm` /// `dst %= imm`.
</span><span class="kw">pub const </span>UREM32_IMM: u8 = BPF_PQR | BPF_K | BPF_UREM;
<span class="doccomment">/// BPF opcode: `urem32 dst, src` /// `dst %= src`.
</span><span class="kw">pub const </span>UREM32_REG: u8 = BPF_PQR | BPF_X | BPF_UREM;
<span class="doccomment">/// BPF opcode: `shmul32 dst, imm` /// `dst = (dst * imm) as i64`.
</span><span class="comment">// pub const SHMUL32_IMM: u8 = BPF_PQR | BPF_K | BPF_SHMUL;
</span><span class="doccomment">/// BPF opcode: `shmul32 dst, src` /// `dst = (dst * src) as i64`.
</span><span class="comment">// pub const SHMUL32_REG: u8 = BPF_PQR | BPF_X | BPF_SHMUL;
</span><span class="doccomment">/// BPF opcode: `sdiv32 dst, imm` /// `dst /= imm`.
</span><span class="kw">pub const </span>SDIV32_IMM: u8 = BPF_PQR | BPF_K | BPF_SDIV;
<span class="doccomment">/// BPF opcode: `sdiv32 dst, src` /// `dst /= src`.
</span><span class="kw">pub const </span>SDIV32_REG: u8 = BPF_PQR | BPF_X | BPF_SDIV;
<span class="doccomment">/// BPF opcode: `srem32 dst, imm` /// `dst %= imm`.
</span><span class="kw">pub const </span>SREM32_IMM: u8 = BPF_PQR | BPF_K | BPF_SREM;
<span class="doccomment">/// BPF opcode: `srem32 dst, src` /// `dst %= src`.
</span><span class="kw">pub const </span>SREM32_REG: u8 = BPF_PQR | BPF_X | BPF_SREM;

<span class="doccomment">/// BPF opcode: `le dst` /// `dst = htole&lt;imm&gt;(dst), with imm in {16, 32, 64}`.
</span><span class="kw">pub const </span>LE: u8 = BPF_ALU | BPF_K | BPF_END;
<span class="doccomment">/// BPF opcode: `be dst` /// `dst = htobe&lt;imm&gt;(dst), with imm in {16, 32, 64}`.
</span><span class="kw">pub const </span>BE: u8 = BPF_ALU | BPF_X | BPF_END;

<span class="doccomment">/// BPF opcode: `add64 dst, imm` /// `dst += imm`.
</span><span class="kw">pub const </span>ADD64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_ADD;
<span class="doccomment">/// BPF opcode: `add64 dst, src` /// `dst += src`.
</span><span class="kw">pub const </span>ADD64_REG: u8 = BPF_ALU64 | BPF_X | BPF_ADD;
<span class="doccomment">/// BPF opcode: `sub64 dst, imm` /// `dst -= imm`.
</span><span class="kw">pub const </span>SUB64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_SUB;
<span class="doccomment">/// BPF opcode: `sub64 dst, src` /// `dst -= src`.
</span><span class="kw">pub const </span>SUB64_REG: u8 = BPF_ALU64 | BPF_X | BPF_SUB;
<span class="doccomment">/// BPF opcode: `div64 dst, imm` /// `dst /= imm`.
</span><span class="kw">pub const </span>MUL64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_MUL;
<span class="doccomment">/// BPF opcode: `div64 dst, src` /// `dst /= src`.
</span><span class="kw">pub const </span>MUL64_REG: u8 = BPF_ALU64 | BPF_X | BPF_MUL;
<span class="doccomment">/// BPF opcode: `div64 dst, imm` /// `dst /= imm`.
</span><span class="kw">pub const </span>DIV64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_DIV;
<span class="doccomment">/// BPF opcode: `div64 dst, src` /// `dst /= src`.
</span><span class="kw">pub const </span>DIV64_REG: u8 = BPF_ALU64 | BPF_X | BPF_DIV;
<span class="doccomment">/// BPF opcode: `or64 dst, imm` /// `dst |= imm`.
</span><span class="kw">pub const </span>OR64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_OR;
<span class="doccomment">/// BPF opcode: `or64 dst, src` /// `dst |= src`.
</span><span class="kw">pub const </span>OR64_REG: u8 = BPF_ALU64 | BPF_X | BPF_OR;
<span class="doccomment">/// BPF opcode: `and64 dst, imm` /// `dst &amp;= imm`.
</span><span class="kw">pub const </span>AND64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_AND;
<span class="doccomment">/// BPF opcode: `and64 dst, src` /// `dst &amp;= src`.
</span><span class="kw">pub const </span>AND64_REG: u8 = BPF_ALU64 | BPF_X | BPF_AND;
<span class="doccomment">/// BPF opcode: `lsh64 dst, imm` /// `dst &lt;&lt;= imm`.
</span><span class="kw">pub const </span>LSH64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_LSH;
<span class="doccomment">/// BPF opcode: `lsh64 dst, src` /// `dst &lt;&lt;= src`.
</span><span class="kw">pub const </span>LSH64_REG: u8 = BPF_ALU64 | BPF_X | BPF_LSH;
<span class="doccomment">/// BPF opcode: `rsh64 dst, imm` /// `dst &gt;&gt;= imm`.
</span><span class="kw">pub const </span>RSH64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_RSH;
<span class="doccomment">/// BPF opcode: `rsh64 dst, src` /// `dst &gt;&gt;= src`.
</span><span class="kw">pub const </span>RSH64_REG: u8 = BPF_ALU64 | BPF_X | BPF_RSH;
<span class="doccomment">/// BPF opcode: `neg64 dst, imm` /// `dst = -dst`.
</span><span class="kw">pub const </span>NEG64: u8 = BPF_ALU64 | BPF_NEG;
<span class="doccomment">/// BPF opcode: `mod64 dst, imm` /// `dst %= imm`.
</span><span class="kw">pub const </span>MOD64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_MOD;
<span class="doccomment">/// BPF opcode: `mod64 dst, src` /// `dst %= src`.
</span><span class="kw">pub const </span>MOD64_REG: u8 = BPF_ALU64 | BPF_X | BPF_MOD;
<span class="doccomment">/// BPF opcode: `xor64 dst, imm` /// `dst ^= imm`.
</span><span class="kw">pub const </span>XOR64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_XOR;
<span class="doccomment">/// BPF opcode: `xor64 dst, src` /// `dst ^= src`.
</span><span class="kw">pub const </span>XOR64_REG: u8 = BPF_ALU64 | BPF_X | BPF_XOR;
<span class="doccomment">/// BPF opcode: `mov64 dst, imm` /// `dst = imm`.
</span><span class="kw">pub const </span>MOV64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_MOV;
<span class="doccomment">/// BPF opcode: `mov64 dst, src` /// `dst = src`.
</span><span class="kw">pub const </span>MOV64_REG: u8 = BPF_ALU64 | BPF_X | BPF_MOV;
<span class="doccomment">/// BPF opcode: `arsh64 dst, imm` /// `dst &gt;&gt;= imm (arithmetic)`.
</span><span class="kw">pub const </span>ARSH64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_ARSH;
<span class="doccomment">/// BPF opcode: `arsh64 dst, src` /// `dst &gt;&gt;= src (arithmetic)`.
</span><span class="kw">pub const </span>ARSH64_REG: u8 = BPF_ALU64 | BPF_X | BPF_ARSH;
<span class="doccomment">/// BPF opcode: `hor64 dst, imm` /// `dst |= imm &lt;&lt; 32`.
</span><span class="kw">pub const </span>HOR64_IMM: u8 = BPF_ALU64 | BPF_K | BPF_HOR;

<span class="doccomment">/// BPF opcode: `lmul64 dst, imm` /// `dst = (dst * imm) as u64`.
</span><span class="kw">pub const </span>LMUL64_IMM: u8 = BPF_PQR | BPF_B | BPF_K | BPF_LMUL;
<span class="doccomment">/// BPF opcode: `lmul64 dst, src` /// `dst = (dst * src) as u64`.
</span><span class="kw">pub const </span>LMUL64_REG: u8 = BPF_PQR | BPF_B | BPF_X | BPF_LMUL;
<span class="doccomment">/// BPF opcode: `uhmul64 dst, imm` /// `dst = (dst * imm) &gt;&gt; 64`.
</span><span class="kw">pub const </span>UHMUL64_IMM: u8 = BPF_PQR | BPF_B | BPF_K | BPF_UHMUL;
<span class="doccomment">/// BPF opcode: `uhmul64 dst, src` /// `dst = (dst * src) &gt;&gt; 64`.
</span><span class="kw">pub const </span>UHMUL64_REG: u8 = BPF_PQR | BPF_B | BPF_X | BPF_UHMUL;
<span class="doccomment">/// BPF opcode: `udiv64 dst, imm` /// `dst /= imm`.
</span><span class="kw">pub const </span>UDIV64_IMM: u8 = BPF_PQR | BPF_B | BPF_K | BPF_UDIV;
<span class="doccomment">/// BPF opcode: `udiv64 dst, src` /// `dst /= src`.
</span><span class="kw">pub const </span>UDIV64_REG: u8 = BPF_PQR | BPF_B | BPF_X | BPF_UDIV;
<span class="doccomment">/// BPF opcode: `urem64 dst, imm` /// `dst %= imm`.
</span><span class="kw">pub const </span>UREM64_IMM: u8 = BPF_PQR | BPF_B | BPF_K | BPF_UREM;
<span class="doccomment">/// BPF opcode: `urem64 dst, src` /// `dst %= src`.
</span><span class="kw">pub const </span>UREM64_REG: u8 = BPF_PQR | BPF_B | BPF_X | BPF_UREM;
<span class="doccomment">/// BPF opcode: `shmul64 dst, imm` /// `dst = (dst * imm) &gt;&gt; 64`.
</span><span class="kw">pub const </span>SHMUL64_IMM: u8 = BPF_PQR | BPF_B | BPF_K | BPF_SHMUL;
<span class="doccomment">/// BPF opcode: `shmul64 dst, src` /// `dst = (dst * src) &gt;&gt; 64`.
</span><span class="kw">pub const </span>SHMUL64_REG: u8 = BPF_PQR | BPF_B | BPF_X | BPF_SHMUL;
<span class="doccomment">/// BPF opcode: `sdiv64 dst, imm` /// `dst /= imm`.
</span><span class="kw">pub const </span>SDIV64_IMM: u8 = BPF_PQR | BPF_B | BPF_K | BPF_SDIV;
<span class="doccomment">/// BPF opcode: `sdiv64 dst, src` /// `dst /= src`.
</span><span class="kw">pub const </span>SDIV64_REG: u8 = BPF_PQR | BPF_B | BPF_X | BPF_SDIV;
<span class="doccomment">/// BPF opcode: `srem64 dst, imm` /// `dst %= imm`.
</span><span class="kw">pub const </span>SREM64_IMM: u8 = BPF_PQR | BPF_B | BPF_K | BPF_SREM;
<span class="doccomment">/// BPF opcode: `srem64 dst, src` /// `dst %= src`.
</span><span class="kw">pub const </span>SREM64_REG: u8 = BPF_PQR | BPF_B | BPF_X | BPF_SREM;

<span class="doccomment">/// BPF opcode: `ja +off` /// `PC += off`.
</span><span class="kw">pub const </span>JA: u8 = BPF_JMP | BPF_JA;
<span class="doccomment">/// BPF opcode: `jeq dst, imm, +off` /// `PC += off if dst == imm`.
</span><span class="kw">pub const </span>JEQ_IMM: u8 = BPF_JMP | BPF_K | BPF_JEQ;
<span class="doccomment">/// BPF opcode: `jeq dst, src, +off` /// `PC += off if dst == src`.
</span><span class="kw">pub const </span>JEQ_REG: u8 = BPF_JMP | BPF_X | BPF_JEQ;
<span class="doccomment">/// BPF opcode: `jgt dst, imm, +off` /// `PC += off if dst &gt; imm`.
</span><span class="kw">pub const </span>JGT_IMM: u8 = BPF_JMP | BPF_K | BPF_JGT;
<span class="doccomment">/// BPF opcode: `jgt dst, src, +off` /// `PC += off if dst &gt; src`.
</span><span class="kw">pub const </span>JGT_REG: u8 = BPF_JMP | BPF_X | BPF_JGT;
<span class="doccomment">/// BPF opcode: `jge dst, imm, +off` /// `PC += off if dst &gt;= imm`.
</span><span class="kw">pub const </span>JGE_IMM: u8 = BPF_JMP | BPF_K | BPF_JGE;
<span class="doccomment">/// BPF opcode: `jge dst, src, +off` /// `PC += off if dst &gt;= src`.
</span><span class="kw">pub const </span>JGE_REG: u8 = BPF_JMP | BPF_X | BPF_JGE;
<span class="doccomment">/// BPF opcode: `jlt dst, imm, +off` /// `PC += off if dst &lt; imm`.
</span><span class="kw">pub const </span>JLT_IMM: u8 = BPF_JMP | BPF_K | BPF_JLT;
<span class="doccomment">/// BPF opcode: `jlt dst, src, +off` /// `PC += off if dst &lt; src`.
</span><span class="kw">pub const </span>JLT_REG: u8 = BPF_JMP | BPF_X | BPF_JLT;
<span class="doccomment">/// BPF opcode: `jle dst, imm, +off` /// `PC += off if dst &lt;= imm`.
</span><span class="kw">pub const </span>JLE_IMM: u8 = BPF_JMP | BPF_K | BPF_JLE;
<span class="doccomment">/// BPF opcode: `jle dst, src, +off` /// `PC += off if dst &lt;= src`.
</span><span class="kw">pub const </span>JLE_REG: u8 = BPF_JMP | BPF_X | BPF_JLE;
<span class="doccomment">/// BPF opcode: `jset dst, imm, +off` /// `PC += off if dst &amp; imm`.
</span><span class="kw">pub const </span>JSET_IMM: u8 = BPF_JMP | BPF_K | BPF_JSET;
<span class="doccomment">/// BPF opcode: `jset dst, src, +off` /// `PC += off if dst &amp; src`.
</span><span class="kw">pub const </span>JSET_REG: u8 = BPF_JMP | BPF_X | BPF_JSET;
<span class="doccomment">/// BPF opcode: `jne dst, imm, +off` /// `PC += off if dst != imm`.
</span><span class="kw">pub const </span>JNE_IMM: u8 = BPF_JMP | BPF_K | BPF_JNE;
<span class="doccomment">/// BPF opcode: `jne dst, src, +off` /// `PC += off if dst != src`.
</span><span class="kw">pub const </span>JNE_REG: u8 = BPF_JMP | BPF_X | BPF_JNE;
<span class="doccomment">/// BPF opcode: `jsgt dst, imm, +off` /// `PC += off if dst &gt; imm (signed)`.
</span><span class="kw">pub const </span>JSGT_IMM: u8 = BPF_JMP | BPF_K | BPF_JSGT;
<span class="doccomment">/// BPF opcode: `jsgt dst, src, +off` /// `PC += off if dst &gt; src (signed)`.
</span><span class="kw">pub const </span>JSGT_REG: u8 = BPF_JMP | BPF_X | BPF_JSGT;
<span class="doccomment">/// BPF opcode: `jsge dst, imm, +off` /// `PC += off if dst &gt;= imm (signed)`.
</span><span class="kw">pub const </span>JSGE_IMM: u8 = BPF_JMP | BPF_K | BPF_JSGE;
<span class="doccomment">/// BPF opcode: `jsge dst, src, +off` /// `PC += off if dst &gt;= src (signed)`.
</span><span class="kw">pub const </span>JSGE_REG: u8 = BPF_JMP | BPF_X | BPF_JSGE;
<span class="doccomment">/// BPF opcode: `jslt dst, imm, +off` /// `PC += off if dst &lt; imm (signed)`.
</span><span class="kw">pub const </span>JSLT_IMM: u8 = BPF_JMP | BPF_K | BPF_JSLT;
<span class="doccomment">/// BPF opcode: `jslt dst, src, +off` /// `PC += off if dst &lt; src (signed)`.
</span><span class="kw">pub const </span>JSLT_REG: u8 = BPF_JMP | BPF_X | BPF_JSLT;
<span class="doccomment">/// BPF opcode: `jsle dst, imm, +off` /// `PC += off if dst &lt;= imm (signed)`.
</span><span class="kw">pub const </span>JSLE_IMM: u8 = BPF_JMP | BPF_K | BPF_JSLE;
<span class="doccomment">/// BPF opcode: `jsle dst, src, +off` /// `PC += off if dst &lt;= src (signed)`.
</span><span class="kw">pub const </span>JSLE_REG: u8 = BPF_JMP | BPF_X | BPF_JSLE;

<span class="doccomment">/// BPF opcode: `call imm` /// syscall function call to syscall with key `imm`.
</span><span class="kw">pub const </span>CALL_IMM: u8 = BPF_JMP | BPF_CALL;
<span class="doccomment">/// BPF opcode: tail call.
</span><span class="kw">pub const </span>CALL_REG: u8 = BPF_JMP | BPF_X | BPF_CALL;
<span class="doccomment">/// BPF opcode: `exit` /// `return r0`.
</span><span class="kw">pub const </span>EXIT: u8 = BPF_JMP | BPF_EXIT;

<span class="comment">// Used in JIT
</span><span class="doccomment">/// Mask to extract the operation class from an operation code.
</span><span class="kw">pub const </span>BPF_CLS_MASK: u8 = <span class="number">0x07</span>;
<span class="doccomment">/// Mask to extract the arithmetic operation code from an instruction operation code.
</span><span class="kw">pub const </span>BPF_ALU_OP_MASK: u8 = <span class="number">0xf0</span>;

<span class="doccomment">/// An eBPF instruction.
///
/// See &lt;https://www.kernel.org/doc/Documentation/networking/filter.txt&gt; for the Linux kernel
/// documentation about eBPF, or &lt;https://github.com/iovisor/bpf-docs/blob/master/eBPF.md&gt; for a
/// more concise version.
</span><span class="attr">#[derive(PartialEq, Eq, Clone, Default)]
</span><span class="kw">pub struct </span>Insn {
    <span class="doccomment">/// Instruction pointer.
    </span><span class="kw">pub </span>ptr: usize,
    <span class="doccomment">/// Operation code.
    </span><span class="kw">pub </span>opc: u8,
    <span class="doccomment">/// Destination register operand.
    </span><span class="kw">pub </span>dst: u8,
    <span class="doccomment">/// Source register operand.
    </span><span class="kw">pub </span>src: u8,
    <span class="doccomment">/// Offset operand.
    </span><span class="kw">pub </span>off: i16,
    <span class="doccomment">/// Immediate value operand.
    </span><span class="kw">pub </span>imm: i64,
}

<span class="kw">impl </span>fmt::Debug <span class="kw">for </span>Insn {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter) -&gt; fmt::Result {
        <span class="macro">write!</span>(
            f,
            <span class="string">"Insn {{ ptr: 0x{:08x?}, opc: 0x{:02x?}, dst: {}, src: {}, off: 0x{:04x?}, imm: 0x{:08x?} }}"</span>,
            <span class="self">self</span>.ptr, <span class="self">self</span>.opc, <span class="self">self</span>.dst, <span class="self">self</span>.src, <span class="self">self</span>.off, <span class="self">self</span>.imm
        )
    }
}

<span class="kw">impl </span>Insn {
    <span class="doccomment">/// Turn an `Insn` back into an array of bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use solana_rbpf::ebpf;
    ///
    /// let prog: &amp;[u8] = &amp;[
    ///     0xb7, 0x12, 0x56, 0x34, 0xde, 0xbc, 0x9a, 0x78,
    ///     ];
    /// let insn = ebpf::Insn {
    ///     ptr: 0x00,
    ///     opc: 0xb7,
    ///     dst: 2,
    ///     src: 1,
    ///     off: 0x3456,
    ///     imm: 0x789abcde
    /// };
    /// assert_eq!(insn.to_array(), prog);
    /// ```
    </span><span class="kw">pub fn </span>to_array(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; [u8; INSN_SIZE] {
        [
            <span class="self">self</span>.opc,
            <span class="self">self</span>.src.wrapping_shl(<span class="number">4</span>) | <span class="self">self</span>.dst,
            (<span class="self">self</span>.off &amp; <span class="number">0xff</span>) <span class="kw">as </span>u8,
            <span class="self">self</span>.off.wrapping_shr(<span class="number">8</span>) <span class="kw">as </span>u8,
            (<span class="self">self</span>.imm &amp; <span class="number">0xff</span>) <span class="kw">as </span>u8,
            (<span class="self">self</span>.imm &amp; <span class="number">0xff_00</span>).wrapping_shr(<span class="number">8</span>) <span class="kw">as </span>u8,
            (<span class="self">self</span>.imm <span class="kw">as </span>u32 &amp; <span class="number">0xff_00_00</span>).wrapping_shr(<span class="number">16</span>) <span class="kw">as </span>u8,
            (<span class="self">self</span>.imm <span class="kw">as </span>u32 &amp; <span class="number">0xff_00_00_00</span>).wrapping_shr(<span class="number">24</span>) <span class="kw">as </span>u8,
        ]
    }

    <span class="doccomment">/// Turn an `Insn` into an vector of bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use solana_rbpf::ebpf;
    ///
    /// let prog: Vec&lt;u8&gt; = vec![
    ///     0xb7, 0x12, 0x56, 0x34, 0xde, 0xbc, 0x9a, 0x78,
    ///     ];
    /// let insn = ebpf::Insn {
    ///     ptr: 0x00,
    ///     opc: 0xb7,
    ///     dst: 2,
    ///     src: 1,
    ///     off: 0x3456,
    ///     imm: 0x789abcde
    /// };
    /// assert_eq!(insn.to_vec(), prog);
    /// ```
    </span><span class="kw">pub fn </span>to_vec(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; Vec&lt;u8&gt; {
        <span class="self">self</span>.to_array().to_vec()
    }
}

<span class="doccomment">/// Get the instruction at `idx` of an eBPF program. `idx` is the index (number) of the
/// instruction (not a byte offset). The first instruction has index 0.
///
/// # Panics
///
/// Panics if it is not possible to get the instruction (if idx is too high, or last instruction is
/// incomplete).
///
/// # Examples
///
/// ```
/// use solana_rbpf::ebpf;
///
/// let prog = &amp;[
///     0xb7, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
///     ];
/// let insn = ebpf::get_insn(prog, 1);
/// assert_eq!(insn.opc, 0x95);
/// ```
///
/// The example below will panic, since the last instruction is not complete and cannot be loaded.
///
/// ```rust,should_panic
/// use solana_rbpf::ebpf;
///
/// let prog = &amp;[
///     0xb7, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
///     0x95, 0x00, 0x00, 0x00, 0x00, 0x00              // two bytes missing
///     ];
/// let insn = ebpf::get_insn(prog, 1);
/// ```
</span><span class="kw">pub fn </span>get_insn(prog: <span class="kw-2">&amp;</span>[u8], pc: usize) -&gt; Insn {
    <span class="comment">// This guard should not be needed in most cases, since the verifier already checks the program
    // size, and indexes should be fine in the interpreter/JIT. But this function is publicly
    // available and user can call it with any `pc`, so we have to check anyway.
    </span><span class="macro">debug_assert!</span>(
        (pc + <span class="number">1</span>) * INSN_SIZE &lt;= prog.len(),
        <span class="string">"cannot reach instruction at index {:?} in program containing {:?} bytes"</span>,
        pc,
        prog.len()
    );
    get_insn_unchecked(prog, pc)
}
<span class="doccomment">/// Same as `get_insn` except not checked
</span><span class="kw">pub fn </span>get_insn_unchecked(prog: <span class="kw-2">&amp;</span>[u8], pc: usize) -&gt; Insn {
    Insn {
        ptr: pc,
        opc: prog[INSN_SIZE * pc],
        dst: prog[INSN_SIZE * pc + <span class="number">1</span>] &amp; <span class="number">0x0f</span>,
        src: (prog[INSN_SIZE * pc + <span class="number">1</span>] &amp; <span class="number">0xf0</span>) &gt;&gt; <span class="number">4</span>,
        off: LittleEndian::read_i16(<span class="kw-2">&amp;</span>prog[(INSN_SIZE * pc + <span class="number">2</span>)..]),
        imm: LittleEndian::read_i32(<span class="kw-2">&amp;</span>prog[(INSN_SIZE * pc + <span class="number">4</span>)..]) <span class="kw">as </span>i64,
    }
}

<span class="doccomment">/// Merge the two halves of a LD_DW_IMM instruction
</span><span class="kw">pub fn </span>augment_lddw_unchecked(prog: <span class="kw-2">&amp;</span>[u8], insn: <span class="kw-2">&amp;mut </span>Insn) {
    <span class="kw">let </span>more_significant_half = LittleEndian::read_i32(<span class="kw-2">&amp;</span>prog[((insn.ptr + <span class="number">1</span>) * INSN_SIZE + <span class="number">4</span>)..]);
    insn.imm = ((insn.imm <span class="kw">as </span>u64 &amp; <span class="number">0xffffffff</span>) | ((more_significant_half <span class="kw">as </span>u64) &lt;&lt; <span class="number">32</span>)) <span class="kw">as </span>i64;
}

<span class="doccomment">/// Hash a symbol name
///
/// This function is used by both the relocator and the VM to translate symbol names
/// into a 32 bit id used to identify a syscall function.  The 32 bit id is used in the
/// eBPF `call` instruction's imm field.
</span><span class="kw">pub fn </span>hash_symbol_name(name: <span class="kw-2">&amp;</span>[u8]) -&gt; u32 {
    <span class="kw">let </span><span class="kw-2">mut </span>hasher = Murmur3Hasher::default();
    Hash::hash_slice(name, <span class="kw-2">&amp;mut </span>hasher);
    hasher.finish()
}
</code></pre></div></section></main></body></html>