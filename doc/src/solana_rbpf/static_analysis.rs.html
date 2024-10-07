<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana_rbpf-0.8.0/src/static_analysis.rs`."><title>static_analysis.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="solana_rbpf" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/qmonnet/rbpf/master/misc/rbpf.ico"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../solana_rbpf/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="attr">#![allow(clippy::arithmetic_side_effects)]
</span><span class="doccomment">//! Static Byte Code Analysis

</span><span class="kw">use </span><span class="kw">crate</span>::disassembler::disassemble_instruction;
<span class="kw">use crate</span>::{
    ebpf,
    elf::Executable,
    error::EbpfError,
    vm::{ContextObject, DynamicAnalysis, TestContextObject},
};
<span class="kw">use </span>rustc_demangle::demangle;
<span class="kw">use </span>std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

<span class="doccomment">/// Register state recorded after executing one instruction
///
/// The last register is the program counter (aka pc).
</span><span class="kw">pub type </span>TraceLogEntry = [u64; <span class="number">12</span>];

<span class="doccomment">/// Used for topological sort
</span><span class="attr">#[derive(PartialEq, Eq, Debug)]
</span><span class="kw">pub struct </span>TopologicalIndex {
    <span class="doccomment">/// Strongly connected component ID
    </span><span class="kw">pub </span>scc_id: usize,
    <span class="doccomment">/// Discovery order inside a strongly connected component
    </span><span class="kw">pub </span>discovery: usize,
}

<span class="kw">impl </span>Default <span class="kw">for </span>TopologicalIndex {
    <span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            scc_id: usize::MAX,
            discovery: usize::MAX,
        }
    }
}

<span class="kw">impl </span>Ord <span class="kw">for </span>TopologicalIndex {
    <span class="kw">fn </span>cmp(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; std::cmp::Ordering {
        (<span class="self">self</span>.scc_id.cmp(<span class="kw-2">&amp;</span>other.scc_id)).then(<span class="self">self</span>.discovery.cmp(<span class="kw-2">&amp;</span>other.discovery))
    }
}

<span class="kw">impl </span>PartialOrd <span class="kw">for </span>TopologicalIndex {
    <span class="kw">fn </span>partial_cmp(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;std::cmp::Ordering&gt; {
        <span class="prelude-val">Some</span>(<span class="self">self</span>.cmp(other))
    }
}

<span class="doccomment">/// A node of the control-flow graph
</span><span class="attr">#[derive(Debug)]
</span><span class="kw">pub struct </span>CfgNode {
    <span class="doccomment">/// Human readable name
    </span><span class="kw">pub </span>label: String,
    <span class="doccomment">/// Predecessors which can jump to the start of this basic block
    </span><span class="kw">pub </span>sources: Vec&lt;usize&gt;,
    <span class="doccomment">/// Successors which the end of this basic block can jump to
    </span><span class="kw">pub </span>destinations: Vec&lt;usize&gt;,
    <span class="doccomment">/// Range of the instructions belonging to this basic block
    </span><span class="kw">pub </span>instructions: std::ops::Range&lt;usize&gt;,
    <span class="doccomment">/// Topological index
    </span><span class="kw">pub </span>topo_index: TopologicalIndex,
    <span class="doccomment">/// Immediate dominator (the last control flow junction)
    </span><span class="kw">pub </span>dominator_parent: usize,
    <span class="doccomment">/// All basic blocks which can only be reached through this one
    </span><span class="kw">pub </span>dominated_children: Vec&lt;usize&gt;,
}

<span class="doccomment">/// An instruction or Φ node of the data-flow graph
</span><span class="attr">#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
</span><span class="kw">pub enum </span>DfgNode {
    <span class="doccomment">/// Points to a single instruction
    </span>InstructionNode(usize),
    <span class="doccomment">/// Points to a basic block which starts with a Φ node (because it has multiple CFG sources)
    </span>PhiNode(usize),
}

<span class="doccomment">/// The register or memory location a data-flow edge guards
</span><span class="attr">#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
</span><span class="kw">pub enum </span>DataResource {
    <span class="doccomment">/// A BPF register
    </span>Register(u8),
    <span class="doccomment">/// A (potentially writeable) memory location
    </span>Memory,
}

<span class="doccomment">/// The kind of a data-flow edge
</span><span class="attr">#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
</span><span class="kw">pub enum </span>DfgEdgeKind {
    <span class="doccomment">/// This kind represents data-flow edges which actually carry data
    ///
    /// E.g. the destination reads a resource, written by the source.
    </span>Filled,
    <span class="doccomment">/// This kind incurrs no actual data-flow
    ///
    /// E.g. the destination overwrites a resource, written by the source.
    </span>Empty,
}

<span class="doccomment">/// An edge of the data-flow graph
</span><span class="attr">#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
</span><span class="kw">pub struct </span>DfgEdge {
    <span class="doccomment">/// The DfgNode that the destination depends on
    </span><span class="kw">pub </span>source: DfgNode,
    <span class="doccomment">/// The DfgNode that depends on the source
    </span><span class="kw">pub </span>destination: DfgNode,
    <span class="doccomment">/// Write-read or write-write
    </span><span class="kw">pub </span>kind: DfgEdgeKind,
    <span class="doccomment">/// A register or memory location
    </span><span class="kw">pub </span>resource: DataResource,
}

<span class="kw">impl </span>Default <span class="kw">for </span>CfgNode {
    <span class="kw">fn </span>default() -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{
            label: String::new(),
            sources: Vec::new(),
            destinations: Vec::new(),
            instructions: <span class="number">0</span>..<span class="number">0</span>,
            topo_index: TopologicalIndex::default(),
            dominator_parent: usize::MAX,
            dominated_children: Vec::new(),
        }
    }
}

<span class="doccomment">/// Result of the executable analysis
</span><span class="kw">pub struct </span>Analysis&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// The program which is analyzed
    </span>executable: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>Executable&lt;TestContextObject&gt;,
    <span class="doccomment">/// Plain list of instructions as they occur in the executable
    </span><span class="kw">pub </span>instructions: Vec&lt;ebpf::Insn&gt;,
    <span class="doccomment">/// Functions in the executable
    </span><span class="kw">pub </span>functions: BTreeMap&lt;usize, (u32, String)&gt;,
    <span class="doccomment">/// Nodes of the control-flow graph
    </span><span class="kw">pub </span>cfg_nodes: BTreeMap&lt;usize, CfgNode&gt;,
    <span class="doccomment">/// Topological order of cfg_nodes
    </span><span class="kw">pub </span>topological_order: Vec&lt;usize&gt;,
    <span class="doccomment">/// CfgNode where the execution starts
    </span><span class="kw">pub </span>entrypoint: usize,
    <span class="doccomment">/// Virtual CfgNode that reaches all functions
    </span><span class="kw">pub </span>super_root: usize,
    <span class="doccomment">/// Data flow edges (the keys are DfgEdge sources)
    </span><span class="kw">pub </span>dfg_forward_edges: BTreeMap&lt;DfgNode, BTreeSet&lt;DfgEdge&gt;&gt;,
    <span class="doccomment">/// Data flow edges (the keys are DfgEdge destinations)
    </span><span class="kw">pub </span>dfg_reverse_edges: BTreeMap&lt;DfgNode, BTreeSet&lt;DfgEdge&gt;&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Analysis&lt;<span class="lifetime">'a</span>&gt; {
    <span class="doccomment">/// Analyze an executable statically
    </span><span class="kw">pub fn </span>from_executable&lt;C: ContextObject&gt;(
        executable: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>Executable&lt;C&gt;,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, EbpfError&gt; {
        <span class="kw">let </span>(_program_vm_addr, program) = executable.get_text_bytes();
        <span class="kw">let </span><span class="kw-2">mut </span>functions = BTreeMap::new();
        <span class="kw">for </span>(key, (function_name, pc)) <span class="kw">in </span>executable.get_function_registry().iter() {
            functions.insert(
                pc,
                (key, String::from_utf8_lossy(function_name).to_string()),
            );
        }
        <span class="macro">debug_assert!</span>(
            program.len() % ebpf::INSN_SIZE == <span class="number">0</span>,
            <span class="string">"eBPF program length must be a multiple of {:?} octets is {:?}"</span>,
            ebpf::INSN_SIZE,
            program.len()
        );
        <span class="kw">let </span><span class="kw-2">mut </span>instructions = Vec::with_capacity(program.len() / ebpf::INSN_SIZE);
        <span class="kw">let </span><span class="kw-2">mut </span>insn_ptr: usize = <span class="number">0</span>;
        <span class="kw">while </span>insn_ptr * ebpf::INSN_SIZE &lt; program.len() {
            <span class="kw">let </span><span class="kw-2">mut </span>insn = ebpf::get_insn_unchecked(program, insn_ptr);
            <span class="kw">if </span>insn.opc == ebpf::LD_DW_IMM {
                insn_ptr += <span class="number">1</span>;
                <span class="kw">if </span>insn_ptr * ebpf::INSN_SIZE &gt;= program.len() {
                    <span class="kw">break</span>;
                }
                ebpf::augment_lddw_unchecked(program, <span class="kw-2">&amp;mut </span>insn);
            }
            instructions.push(insn);
            insn_ptr += <span class="number">1</span>;
        }
        <span class="kw">let </span><span class="kw-2">mut </span>result = <span class="self">Self </span>{
            <span class="comment">// Removes the generic ContextObject which is safe because we are not going to execute the program
            </span>executable: <span class="kw">unsafe </span>{ std::mem::transmute(executable) },
            instructions,
            functions,
            cfg_nodes: BTreeMap::new(),
            topological_order: Vec::new(),
            entrypoint: executable.get_entrypoint_instruction_offset(),
            super_root: insn_ptr,
            dfg_forward_edges: BTreeMap::new(),
            dfg_reverse_edges: BTreeMap::new(),
        };
        result.split_into_basic_blocks(<span class="bool-val">false</span>);
        result.control_flow_graph_tarjan();
        result.control_flow_graph_dominance_hierarchy();
        result.label_basic_blocks();
        <span class="kw">let </span>basic_block_outputs = result.intra_basic_block_data_flow();
        result.inter_basic_block_data_flow(basic_block_outputs);
        <span class="prelude-val">Ok</span>(result)
    }

    <span class="kw">fn </span>link_cfg_edges(<span class="kw-2">&amp;mut </span><span class="self">self</span>, cfg_edges: Vec&lt;(usize, Vec&lt;usize&gt;)&gt;, both_directions: bool) {
        <span class="kw">for </span>(source, destinations) <span class="kw">in </span>cfg_edges {
            <span class="kw">if </span>both_directions {
                <span class="self">self</span>.cfg_nodes.get_mut(<span class="kw-2">&amp;</span>source).unwrap().destinations = destinations.clone();
            }
            <span class="kw">for </span>destination <span class="kw">in </span><span class="kw-2">&amp;</span>destinations {
                <span class="self">self</span>.cfg_nodes
                    .get_mut(destination)
                    .unwrap()
                    .sources
                    .push(source);
            }
        }
    }

    <span class="doccomment">/// Splits the sequence of instructions into basic blocks
    ///
    /// Also links the control-flow graph edges between the basic blocks.
    </span><span class="kw">pub fn </span>split_into_basic_blocks(<span class="kw-2">&amp;mut </span><span class="self">self</span>, flatten_call_graph: bool) {
        <span class="self">self</span>.cfg_nodes.insert(<span class="number">0</span>, CfgNode::default());
        <span class="kw">for </span>pc <span class="kw">in </span><span class="self">self</span>.functions.keys() {
            <span class="self">self</span>.cfg_nodes.entry(<span class="kw-2">*</span>pc).or_default();
        }
        <span class="kw">let </span><span class="kw-2">mut </span>cfg_edges = BTreeMap::new();
        <span class="kw">for </span>insn <span class="kw">in </span><span class="self">self</span>.instructions.iter() {
            <span class="kw">let </span>target_pc = (insn.ptr <span class="kw">as </span>isize + insn.off <span class="kw">as </span>isize + <span class="number">1</span>) <span class="kw">as </span>usize;
            <span class="kw">match </span>insn.opc {
                ebpf::CALL_IMM =&gt; {
                    <span class="kw">if let </span><span class="prelude-val">Some</span>((function_name, _function)) = <span class="self">self
                        </span>.executable
                        .get_loader()
                        .get_function_registry()
                        .lookup_by_key(insn.imm <span class="kw">as </span>u32)
                    {
                        <span class="kw">if </span>function_name == <span class="string">b"abort" </span>{
                            <span class="self">self</span>.cfg_nodes.entry(insn.ptr + <span class="number">1</span>).or_default();
                            cfg_edges.insert(insn.ptr, (insn.opc, Vec::new()));
                        }
                    } <span class="kw">else if let </span><span class="prelude-val">Some</span>((_function_name, target_pc)) = <span class="self">self
                        </span>.executable
                        .get_function_registry()
                        .lookup_by_key(insn.imm <span class="kw">as </span>u32)
                    {
                        <span class="self">self</span>.cfg_nodes.entry(insn.ptr + <span class="number">1</span>).or_default();
                        <span class="self">self</span>.cfg_nodes.entry(target_pc).or_default();
                        <span class="kw">let </span>destinations = <span class="kw">if </span>flatten_call_graph {
                            <span class="macro">vec!</span>[insn.ptr + <span class="number">1</span>, target_pc]
                        } <span class="kw">else </span>{
                            <span class="macro">vec!</span>[insn.ptr + <span class="number">1</span>]
                        };
                        cfg_edges.insert(insn.ptr, (insn.opc, destinations));
                    }
                }
                ebpf::CALL_REG =&gt; {
                    <span class="comment">// Abnormal CFG edge
                    </span><span class="self">self</span>.cfg_nodes.entry(insn.ptr + <span class="number">1</span>).or_default();
                    <span class="kw">let </span>destinations = <span class="kw">if </span>flatten_call_graph {
                        <span class="macro">vec!</span>[insn.ptr + <span class="number">1</span>, <span class="self">self</span>.super_root]
                    } <span class="kw">else </span>{
                        <span class="macro">vec!</span>[insn.ptr + <span class="number">1</span>]
                    };
                    cfg_edges.insert(insn.ptr, (insn.opc, destinations));
                }
                ebpf::EXIT =&gt; {
                    <span class="self">self</span>.cfg_nodes.entry(insn.ptr + <span class="number">1</span>).or_default();
                    cfg_edges.insert(insn.ptr, (insn.opc, Vec::new()));
                }
                ebpf::JA =&gt; {
                    <span class="self">self</span>.cfg_nodes.entry(insn.ptr + <span class="number">1</span>).or_default();
                    <span class="self">self</span>.cfg_nodes.entry(target_pc).or_default();
                    cfg_edges.insert(insn.ptr, (insn.opc, <span class="macro">vec!</span>[target_pc]));
                }
                ebpf::JEQ_IMM
                | ebpf::JGT_IMM
                | ebpf::JGE_IMM
                | ebpf::JLT_IMM
                | ebpf::JLE_IMM
                | ebpf::JSET_IMM
                | ebpf::JNE_IMM
                | ebpf::JSGT_IMM
                | ebpf::JSGE_IMM
                | ebpf::JSLT_IMM
                | ebpf::JSLE_IMM
                | ebpf::JEQ_REG
                | ebpf::JGT_REG
                | ebpf::JGE_REG
                | ebpf::JLT_REG
                | ebpf::JLE_REG
                | ebpf::JSET_REG
                | ebpf::JNE_REG
                | ebpf::JSGT_REG
                | ebpf::JSGE_REG
                | ebpf::JSLT_REG
                | ebpf::JSLE_REG =&gt; {
                    <span class="self">self</span>.cfg_nodes.entry(insn.ptr + <span class="number">1</span>).or_default();
                    <span class="self">self</span>.cfg_nodes.entry(target_pc).or_default();
                    cfg_edges.insert(insn.ptr, (insn.opc, <span class="macro">vec!</span>[insn.ptr + <span class="number">1</span>, target_pc]));
                }
                <span class="kw">_ </span>=&gt; {}
            }
        }
        {
            <span class="kw">let </span><span class="kw-2">mut </span>cfg_nodes = BTreeMap::new();
            std::mem::swap(<span class="kw-2">&amp;mut </span><span class="self">self</span>.cfg_nodes, <span class="kw-2">&amp;mut </span>cfg_nodes);
            <span class="kw">let </span><span class="kw-2">mut </span>cfg_nodes = cfg_nodes
                .into_iter()
                .filter(|(cfg_node_start, _cfg_node)| {
                    <span class="kw">match </span><span class="self">self
                        </span>.instructions
                        .binary_search_by(|insn| insn.ptr.cmp(cfg_node_start))
                    {
                        <span class="prelude-val">Ok</span>(<span class="kw">_</span>) =&gt; <span class="bool-val">true</span>,
                        <span class="prelude-val">Err</span>(_index) =&gt; <span class="bool-val">false</span>,
                    }
                })
                .collect();
            std::mem::swap(<span class="kw-2">&amp;mut </span><span class="self">self</span>.cfg_nodes, <span class="kw-2">&amp;mut </span>cfg_nodes);
            <span class="kw">for </span>cfg_edge <span class="kw">in </span>cfg_edges.values_mut() {
                cfg_edge
                    .<span class="number">1
                    </span>.retain(|destination| <span class="self">self</span>.cfg_nodes.contains_key(destination));
            }
            <span class="kw">let </span><span class="kw-2">mut </span>functions = BTreeMap::new();
            std::mem::swap(<span class="kw-2">&amp;mut </span><span class="self">self</span>.functions, <span class="kw-2">&amp;mut </span>functions);
            <span class="kw">let </span><span class="kw-2">mut </span>functions = functions
                .into_iter()
                .filter(|(function_start, <span class="kw">_</span>)| <span class="self">self</span>.cfg_nodes.contains_key(function_start))
                .collect();
            std::mem::swap(<span class="kw-2">&amp;mut </span><span class="self">self</span>.functions, <span class="kw-2">&amp;mut </span>functions);
        }
        {
            <span class="kw">let </span><span class="kw-2">mut </span>instruction_index = <span class="number">0</span>;
            <span class="kw">let </span><span class="kw-2">mut </span>cfg_node_iter = <span class="self">self</span>.cfg_nodes.iter_mut().peekable();
            <span class="kw">let </span><span class="kw-2">mut </span>cfg_edge_iter = cfg_edges.iter_mut().peekable();
            <span class="kw">while let </span><span class="prelude-val">Some</span>((cfg_node_start, cfg_node)) = cfg_node_iter.next() {
                <span class="kw">let </span>cfg_node_end = <span class="kw">if let </span><span class="prelude-val">Some</span>(next_cfg_node) = cfg_node_iter.peek() {
                    <span class="kw-2">*</span>next_cfg_node.<span class="number">0 </span>- <span class="number">1
                </span>} <span class="kw">else </span>{
                    <span class="self">self</span>.instructions.last().unwrap().ptr
                };
                cfg_node.instructions.start = instruction_index;
                <span class="kw">while </span>instruction_index &lt; <span class="self">self</span>.instructions.len() {
                    <span class="kw">if </span><span class="self">self</span>.instructions[instruction_index].ptr &lt;= cfg_node_end {
                        instruction_index += <span class="number">1</span>;
                        cfg_node.instructions.end = instruction_index;
                    } <span class="kw">else </span>{
                        <span class="kw">break</span>;
                    }
                }
                <span class="kw">if let </span><span class="prelude-val">Some</span>(next_cfg_edge) = cfg_edge_iter.peek() {
                    <span class="kw">if </span><span class="kw-2">*</span>next_cfg_edge.<span class="number">0 </span>&lt;= cfg_node_end {
                        cfg_node.destinations = next_cfg_edge.<span class="number">1 </span>.<span class="number">1</span>.clone();
                        cfg_edge_iter.next();
                        <span class="kw">continue</span>;
                    }
                }
                <span class="kw">if let </span><span class="prelude-val">Some</span>(next_cfg_node) = cfg_node_iter.peek() {
                    <span class="kw">if </span>!<span class="self">self</span>.functions.contains_key(cfg_node_start) {
                        cfg_node.destinations.push(<span class="kw-2">*</span>next_cfg_node.<span class="number">0</span>);
                    }
                }
            }
        }
        <span class="self">self</span>.link_cfg_edges(
            <span class="self">self</span>.cfg_nodes
                .iter()
                .map(|(source, cfg_node)| (<span class="kw-2">*</span>source, cfg_node.destinations.clone()))
                .collect::&lt;Vec&lt;(usize, Vec&lt;usize&gt;)&gt;&gt;(),
            <span class="bool-val">false</span>,
        );
        <span class="kw">if </span>flatten_call_graph {
            <span class="kw">let </span><span class="kw-2">mut </span>destinations = Vec::new();
            <span class="kw">let </span><span class="kw-2">mut </span>cfg_edges = Vec::new();
            <span class="kw">for </span>(source, cfg_node) <span class="kw">in </span><span class="self">self</span>.cfg_nodes.iter() {
                <span class="kw">if </span><span class="self">self</span>.functions.contains_key(source) {
                    destinations = cfg_node
                        .sources
                        .iter()
                        .map(|destination| {
                            <span class="self">self</span>.instructions
                                [<span class="self">self</span>.cfg_nodes.get(destination).unwrap().instructions.end]
                                .ptr
                        })
                        .collect();
                }
                <span class="kw">if </span>cfg_node.destinations.is_empty()
                    &amp;&amp; <span class="self">self</span>.instructions[cfg_node.instructions.end - <span class="number">1</span>].opc == ebpf::EXIT
                {
                    cfg_edges.push((<span class="kw-2">*</span>source, destinations.clone()));
                }
            }
            <span class="self">self</span>.link_cfg_edges(cfg_edges, <span class="bool-val">true</span>);
        }
    }

    <span class="doccomment">/// Gives the basic blocks names
    </span><span class="kw">pub fn </span>label_basic_blocks(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">for </span>(pc, cfg_node) <span class="kw">in </span><span class="self">self</span>.cfg_nodes.iter_mut() {
            cfg_node.label = <span class="kw">if let </span><span class="prelude-val">Some</span>(function) = <span class="self">self</span>.functions.get(pc) {
                demangle(<span class="kw-2">&amp;</span>function.<span class="number">1</span>).to_string()
            } <span class="kw">else </span>{
                <span class="macro">format!</span>(<span class="string">"lbb_{pc}"</span>)
            };
        }
        <span class="kw">if let </span><span class="prelude-val">Some</span>(super_root) = <span class="self">self</span>.cfg_nodes.get_mut(<span class="kw-2">&amp;</span><span class="self">self</span>.super_root) {
            super_root.label = <span class="string">"super_root"</span>.to_string();
        }
    }

    <span class="doccomment">/// Generates labels for assembler code
    </span><span class="kw">pub fn </span>disassemble_label&lt;W: std::io::Write&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        output: <span class="kw-2">&amp;mut </span>W,
        suppress_extra_newlines: bool,
        pc: usize,
        last_basic_block: <span class="kw-2">&amp;mut </span>usize,
    ) -&gt; std::io::Result&lt;()&gt; {
        <span class="kw">if let </span><span class="prelude-val">Some</span>(cfg_node) = <span class="self">self</span>.cfg_nodes.get(<span class="kw-2">&amp;</span>pc) {
            <span class="kw">let </span>is_function = <span class="self">self</span>.functions.contains_key(<span class="kw-2">&amp;</span>pc);
            <span class="kw">if </span>is_function || cfg_node.sources != <span class="macro">vec!</span>[<span class="kw-2">*</span>last_basic_block] {
                <span class="kw">if </span>is_function &amp;&amp; !suppress_extra_newlines {
                    <span class="macro">writeln!</span>(output)<span class="question-mark">?</span>;
                }
                <span class="macro">writeln!</span>(output, <span class="string">"{}:"</span>, cfg_node.label)<span class="question-mark">?</span>;
            }
            <span class="kw">let </span>last_insn = <span class="kw-2">&amp;</span><span class="self">self</span>.instructions[cfg_node.instructions.end - <span class="number">1</span>];
            <span class="kw-2">*</span>last_basic_block = <span class="kw">if </span>last_insn.opc == ebpf::JA {
                usize::MAX
            } <span class="kw">else </span>{
                pc
            };
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Generates assembler code for a single instruction
    </span><span class="kw">pub fn </span>disassemble_instruction(<span class="kw-2">&amp;</span><span class="self">self</span>, insn: <span class="kw-2">&amp;</span>ebpf::Insn) -&gt; String {
        disassemble_instruction(
            insn,
            <span class="kw-2">&amp;</span><span class="self">self</span>.cfg_nodes,
            <span class="self">self</span>.executable.get_function_registry(),
            <span class="self">self</span>.executable.get_loader(),
            <span class="self">self</span>.executable.get_sbpf_version(),
        )
    }

    <span class="doccomment">/// Generates assembler code for the analyzed executable
    </span><span class="kw">pub fn </span>disassemble&lt;W: std::io::Write&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, output: <span class="kw-2">&amp;mut </span>W) -&gt; std::io::Result&lt;()&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>last_basic_block = usize::MAX;
        <span class="kw">for </span>insn <span class="kw">in </span><span class="self">self</span>.instructions.iter() {
            <span class="self">self</span>.disassemble_label(
                output,
                <span class="prelude-val">Some</span>(insn) == <span class="self">self</span>.instructions.first(),
                insn.ptr,
                <span class="kw-2">&amp;mut </span>last_basic_block,
            )<span class="question-mark">?</span>;
            <span class="macro">writeln!</span>(output, <span class="string">"    {}"</span>, <span class="self">self</span>.disassemble_instruction(insn))<span class="question-mark">?</span>;
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Use this method to print the trace log
    </span><span class="kw">pub fn </span>disassemble_trace_log&lt;W: std::io::Write&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        output: <span class="kw-2">&amp;mut </span>W,
        trace_log: <span class="kw-2">&amp;</span>[TraceLogEntry],
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), std::io::Error&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>pc_to_insn_index = <span class="macro">vec!</span>[
            <span class="number">0usize</span>;
            <span class="self">self</span>.instructions
                .last()
                .map(|insn| insn.ptr + <span class="number">2</span>)
                .unwrap_or(<span class="number">0</span>)
        ];
        <span class="kw">for </span>(index, insn) <span class="kw">in </span><span class="self">self</span>.instructions.iter().enumerate() {
            pc_to_insn_index[insn.ptr] = index;
            pc_to_insn_index[insn.ptr + <span class="number">1</span>] = index;
        }
        <span class="kw">for </span>(index, entry) <span class="kw">in </span>trace_log.iter().enumerate() {
            <span class="kw">let </span>pc = entry[<span class="number">11</span>] <span class="kw">as </span>usize;
            <span class="kw">let </span>insn = <span class="kw-2">&amp;</span><span class="self">self</span>.instructions[pc_to_insn_index[pc]];
            <span class="macro">writeln!</span>(
                output,
                <span class="string">"{:5?} {:016X?} {:5?}: {}"</span>,
                index,
                <span class="kw-2">&amp;</span>entry[<span class="number">0</span>..<span class="number">11</span>],
                pc,
                <span class="self">self</span>.disassemble_instruction(insn),
            )<span class="question-mark">?</span>;
        }
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Iterates over the cfg_nodes while providing the PC range of the function they belong to.
    </span><span class="kw">pub fn </span>iter_cfg_by_function(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
    ) -&gt; <span class="kw">impl </span>Iterator&lt;Item = (std::ops::Range&lt;usize&gt;, usize, <span class="kw-2">&amp;</span>CfgNode)&gt; + <span class="lifetime">'_ </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>function_iter = <span class="self">self</span>.functions.keys().peekable();
        <span class="kw">let </span><span class="kw-2">mut </span>function_start = <span class="kw-2">*</span>function_iter.next().unwrap();
        <span class="self">self</span>.cfg_nodes
            .iter()
            .map(<span class="kw">move </span>|(cfg_node_start, cfg_node)| {
                <span class="kw">if </span><span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span>cfg_node_start) == function_iter.peek() {
                    function_start = <span class="kw-2">*</span>function_iter.next().unwrap();
                }
                <span class="kw">let </span>function_end = <span class="kw">if let </span><span class="prelude-val">Some</span>(next_function) = function_iter.peek() {
                    <span class="kw-2">**</span>next_function
                } <span class="kw">else </span>{
                    <span class="self">self</span>.instructions.last().unwrap().ptr + <span class="number">1
                </span>};
                (function_start..function_end, <span class="kw-2">*</span>cfg_node_start, cfg_node)
            })
    }

    <span class="doccomment">/// Generates a graphviz DOT of the analyzed executable
    </span><span class="kw">pub fn </span>visualize_graphically&lt;W: std::io::Write&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        output: <span class="kw-2">&amp;mut </span>W,
        dynamic_analysis: <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>DynamicAnalysis&gt;,
    ) -&gt; std::io::Result&lt;()&gt; {
        <span class="kw">fn </span>html_escape(string: <span class="kw-2">&amp;</span>str) -&gt; String {
            string
                .replace(<span class="string">'&amp;'</span>, <span class="string">"&amp;amp;"</span>)
                .replace(<span class="string">'&lt;'</span>, <span class="string">"&amp;lt;"</span>)
                .replace(<span class="string">'&gt;'</span>, <span class="string">"&amp;gt;"</span>)
                .replace(<span class="string">'\"'</span>, <span class="string">"&amp;quot;"</span>)
        }
        <span class="kw">fn </span>emit_cfg_node&lt;W: std::io::Write&gt;(
            output: <span class="kw-2">&amp;mut </span>W,
            dynamic_analysis: <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span>DynamicAnalysis&gt;,
            analysis: <span class="kw-2">&amp;</span>Analysis,
            function_range: std::ops::Range&lt;usize&gt;,
            alias_nodes: <span class="kw-2">&amp;mut </span>HashSet&lt;usize&gt;,
            cfg_node_start: usize,
        ) -&gt; std::io::Result&lt;()&gt; {
            <span class="kw">let </span>cfg_node = <span class="kw-2">&amp;</span>analysis.cfg_nodes[<span class="kw-2">&amp;</span>cfg_node_start];
            <span class="macro">writeln!</span>(output, <span class="string">"    lbb_{} [label=&lt;&lt;table border=\"0\" cellborder=\"0\" cellpadding=\"3\"&gt;{}&lt;/table&gt;&gt;];"</span>,
                cfg_node_start,
                analysis.instructions[cfg_node.instructions.clone()].iter()
                .map(|insn| {
                    <span class="kw">let </span>desc = analysis.disassemble_instruction(
                        insn
                    );
                    <span class="kw">if let </span><span class="prelude-val">Some</span>(split_index) = desc.find(<span class="string">' '</span>) {
                        <span class="kw">let </span><span class="kw-2">mut </span>rest = desc[split_index+<span class="number">1</span>..].to_string();
                        <span class="kw">if </span>rest.len() &gt; MAX_CELL_CONTENT_LENGTH + <span class="number">1 </span>{
                            rest.truncate(MAX_CELL_CONTENT_LENGTH);
                            rest = <span class="macro">format!</span>(<span class="string">"{rest}…"</span>);
                        }
                        <span class="macro">format!</span>(<span class="string">"&lt;tr&gt;&lt;td align=\"left\"&gt;{}&lt;/td&gt;&lt;td align=\"left\"&gt;{}&lt;/td&gt;&lt;/tr&gt;"</span>, html_escape(<span class="kw-2">&amp;</span>desc[..split_index]), html_escape(<span class="kw-2">&amp;</span>rest))
                    } <span class="kw">else </span>{
                        <span class="macro">format!</span>(<span class="string">"&lt;tr&gt;&lt;td align=\"left\"&gt;{}&lt;/td&gt;&lt;/tr&gt;"</span>, html_escape(<span class="kw-2">&amp;</span>desc))
                    }
                })
                .collect::&lt;Vec&lt;String&gt;&gt;()
                .join(<span class="string">""</span>)
            )<span class="question-mark">?</span>;
            <span class="kw">if let </span><span class="prelude-val">Some</span>(dynamic_analysis) = dynamic_analysis {
                <span class="kw">if let </span><span class="prelude-val">Some</span>(recorded_edges) = dynamic_analysis.edges.get(<span class="kw-2">&amp;</span>cfg_node_start) {
                    <span class="kw">for </span>destination <span class="kw">in </span>recorded_edges.keys() {
                        <span class="kw">if </span>!function_range.contains(destination) {
                            alias_nodes.insert(<span class="kw-2">*</span>destination);
                        }
                    }
                }
            }
            <span class="kw">for </span>child <span class="kw">in </span><span class="kw-2">&amp;</span>cfg_node.dominated_children {
                emit_cfg_node(
                    output,
                    dynamic_analysis,
                    analysis,
                    function_range.clone(),
                    alias_nodes,
                    <span class="kw-2">*</span>child,
                )<span class="question-mark">?</span>;
            }
            <span class="prelude-val">Ok</span>(())
        }
        <span class="macro">writeln!</span>(
            output,
            <span class="string">"digraph {{
  graph [
    rankdir=LR;
    concentrate=True;
    style=filled;
    color=lightgrey;
  ];
  node [
    shape=rect;
    style=filled;
    fillcolor=white;
    fontname=\"Courier New\";
  ];
  edge [
    fontname=\"Courier New\";
  ];"
        </span>)<span class="question-mark">?</span>;
        <span class="kw">const </span>MAX_CELL_CONTENT_LENGTH: usize = <span class="number">15</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>function_iter = <span class="self">self</span>.functions.keys().peekable();
        <span class="kw">while let </span><span class="prelude-val">Some</span>(function_start) = function_iter.next() {
            <span class="kw">let </span>function_end = <span class="kw">if let </span><span class="prelude-val">Some</span>(next_function) = function_iter.peek() {
                <span class="kw-2">**</span>next_function
            } <span class="kw">else </span>{
                <span class="self">self</span>.instructions.last().unwrap().ptr + <span class="number">1
            </span>};
            <span class="kw">let </span><span class="kw-2">mut </span>alias_nodes = HashSet::new();
            <span class="macro">writeln!</span>(output, <span class="string">"  subgraph cluster_{} {{"</span>, <span class="kw-2">*</span>function_start)<span class="question-mark">?</span>;
            <span class="macro">writeln!</span>(
                output,
                <span class="string">"    label={:?};"</span>,
                html_escape(<span class="kw-2">&amp;</span><span class="self">self</span>.cfg_nodes[function_start].label)
            )<span class="question-mark">?</span>;
            <span class="macro">writeln!</span>(output, <span class="string">"    tooltip=lbb_{};"</span>, <span class="kw-2">*</span>function_start)<span class="question-mark">?</span>;
            emit_cfg_node(
                output,
                dynamic_analysis,
                <span class="self">self</span>,
                <span class="kw-2">*</span>function_start..function_end,
                <span class="kw-2">&amp;mut </span>alias_nodes,
                <span class="kw-2">*</span>function_start,
            )<span class="question-mark">?</span>;
            <span class="kw">for </span>alias_node <span class="kw">in </span>alias_nodes.iter() {
                <span class="macro">writeln!</span>(
                    output,
                    <span class="string">"    alias_{}_lbb_{} ["</span>,
                    <span class="kw-2">*</span>function_start, <span class="kw-2">*</span>alias_node
                )<span class="question-mark">?</span>;
                <span class="macro">writeln!</span>(output, <span class="string">"        label=lbb_{:?};"</span>, <span class="kw-2">*</span>alias_node)<span class="question-mark">?</span>;
                <span class="macro">writeln!</span>(output, <span class="string">"        tooltip=lbb_{:?};"</span>, <span class="kw-2">*</span>alias_node)<span class="question-mark">?</span>;
                <span class="macro">writeln!</span>(output, <span class="string">"        URL=\"#lbb_{:?}\";"</span>, <span class="kw-2">*</span>alias_node)<span class="question-mark">?</span>;
                <span class="macro">writeln!</span>(output, <span class="string">"    ];"</span>)<span class="question-mark">?</span>;
            }
            <span class="macro">writeln!</span>(output, <span class="string">"  }}"</span>)<span class="question-mark">?</span>;
        }
        <span class="kw">for </span>(function_range, cfg_node_start, cfg_node) <span class="kw">in </span><span class="self">self</span>.iter_cfg_by_function() {
            <span class="kw">if </span>cfg_node_start != cfg_node.dominator_parent {
                <span class="macro">writeln!</span>(
                    output,
                    <span class="string">"  lbb_{} -&gt; lbb_{} [style=dotted; arrowhead=none];"</span>,
                    cfg_node_start, cfg_node.dominator_parent,
                )<span class="question-mark">?</span>;
            }
            <span class="kw">let </span><span class="kw-2">mut </span>edges: BTreeMap&lt;usize, usize&gt; = cfg_node
                .destinations
                .iter()
                .map(|destination| (<span class="kw-2">*</span>destination, <span class="number">0</span>))
                .collect();
            <span class="kw">if let </span><span class="prelude-val">Some</span>(dynamic_analysis) = dynamic_analysis {
                <span class="kw">if let </span><span class="prelude-val">Some</span>(recorded_edges) = dynamic_analysis.edges.get(<span class="kw-2">&amp;</span>cfg_node_start) {
                    <span class="kw">for </span>(destination, recorded_counter) <span class="kw">in </span>recorded_edges.iter() {
                        edges
                            .entry(<span class="kw-2">*</span>destination)
                            .and_modify(|counter| {
                                <span class="kw-2">*</span>counter = <span class="kw-2">*</span>recorded_counter;
                            })
                            .or_insert(<span class="kw-2">*</span>recorded_counter);
                    }
                }
            }
            <span class="kw">let </span>counter_sum: usize = edges.values().sum();
            <span class="kw">if </span>counter_sum == <span class="number">0 </span>&amp;&amp; !edges.is_empty() {
                <span class="macro">writeln!</span>(
                    output,
                    <span class="string">"  lbb_{} -&gt; {{{}}};"</span>,
                    cfg_node_start,
                    edges
                        .keys()
                        .map(|destination| <span class="macro">format!</span>(<span class="string">"lbb_{}"</span>, <span class="kw-2">*</span>destination))
                        .collect::&lt;Vec&lt;String&gt;&gt;()
                        .join(<span class="string">" "</span>)
                )<span class="question-mark">?</span>;
            } <span class="kw">else if let </span><span class="prelude-val">Some</span>(dynamic_analysis) = dynamic_analysis {
                <span class="kw">for </span>(destination, counter) <span class="kw">in </span>edges {
                    <span class="macro">write!</span>(output, <span class="string">"  lbb_{cfg_node_start} -&gt; "</span>)<span class="question-mark">?</span>;
                    <span class="kw">if </span>function_range.contains(<span class="kw-2">&amp;</span>destination) {
                        <span class="macro">write!</span>(output, <span class="string">"lbb_{destination}"</span>)<span class="question-mark">?</span>;
                    } <span class="kw">else </span>{
                        <span class="macro">write!</span>(
                            output,
                            <span class="string">"alias_{0}_lbb_{1}"</span>,
                            function_range.start, destination
                        )<span class="question-mark">?</span>;
                    }
                    <span class="macro">writeln!</span>(
                        output,
                        <span class="string">" [label=\"{}\";color=\"{} 1.0 {}.0\"];"</span>,
                        counter,
                        counter <span class="kw">as </span>f32 / (dynamic_analysis.edge_counter_max <span class="kw">as </span>f32 * <span class="number">3.0</span>)
                            + <span class="number">2.0 </span>/ <span class="number">3.0</span>,
                        (counter != <span class="number">0</span>) <span class="kw">as </span>i32,
                    )<span class="question-mark">?</span>;
                }
            }
        }
        <span class="macro">writeln!</span>(output, <span class="string">"}}"</span>)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }

    <span class="doccomment">/// Finds the strongly connected components
    ///
    /// Generates a topological order as by-product.
    </span><span class="kw">pub fn </span>control_flow_graph_tarjan(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">if </span><span class="self">self</span>.cfg_nodes.is_empty() {
            <span class="kw">return</span>;
        }
        <span class="kw">struct </span>NodeState {
            cfg_node: usize,
            discovery: usize,
            lowlink: usize,
            scc_id: usize,
            is_on_scc_stack: bool,
        }
        <span class="kw">let </span><span class="kw-2">mut </span>nodes = <span class="self">self
            </span>.cfg_nodes
            .iter_mut()
            .enumerate()
            .map(|(v, (key, cfg_node))| {
                cfg_node.topo_index.scc_id = v;
                NodeState {
                    cfg_node: <span class="kw-2">*</span>key,
                    discovery: usize::MAX,
                    lowlink: usize::MAX,
                    scc_id: usize::MAX,
                    is_on_scc_stack: <span class="bool-val">false</span>,
                }
            })
            .collect::&lt;Vec&lt;NodeState&gt;&gt;();
        <span class="kw">let </span><span class="kw-2">mut </span>scc_id = <span class="number">0</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>scc_stack = Vec::new();
        <span class="kw">let </span><span class="kw-2">mut </span>discovered = <span class="number">0</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>next_v = <span class="number">1</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>recursion_stack = <span class="macro">vec!</span>[(<span class="number">0</span>, <span class="number">0</span>)];
        <span class="lifetime">'dfs</span>: <span class="kw">while let </span><span class="prelude-val">Some</span>((v, edge_index)) = recursion_stack.pop() {
            <span class="kw">let </span>node = <span class="kw-2">&amp;mut </span>nodes[v];
            <span class="kw">if </span>edge_index == <span class="number">0 </span>{
                node.discovery = discovered;
                node.lowlink = discovered;
                node.is_on_scc_stack = <span class="bool-val">true</span>;
                scc_stack.push(v);
                discovered += <span class="number">1</span>;
            }
            <span class="kw">let </span>cfg_node = <span class="self">self</span>.cfg_nodes.get(<span class="kw-2">&amp;</span>node.cfg_node).unwrap();
            <span class="kw">for </span>j <span class="kw">in </span>edge_index..cfg_node.destinations.len() {
                <span class="kw">let </span>w = <span class="self">self
                    </span>.cfg_nodes
                    .get(<span class="kw-2">&amp;</span>cfg_node.destinations[j])
                    .unwrap()
                    .topo_index
                    .scc_id;
                <span class="kw">if </span>nodes[w].discovery == usize::MAX {
                    recursion_stack.push((v, j + <span class="number">1</span>));
                    recursion_stack.push((w, <span class="number">0</span>));
                    <span class="kw">continue </span><span class="lifetime">'dfs</span>;
                } <span class="kw">else if </span>nodes[w].is_on_scc_stack {
                    nodes[v].lowlink = nodes[v].lowlink.min(nodes[w].discovery);
                }
            }
            <span class="kw">if </span>nodes[v].discovery == nodes[v].lowlink {
                <span class="kw">let </span><span class="kw-2">mut </span>index_in_scc = <span class="number">0</span>;
                <span class="kw">while let </span><span class="prelude-val">Some</span>(w) = scc_stack.pop() {
                    <span class="kw">let </span>node = <span class="kw-2">&amp;mut </span>nodes[w];
                    node.is_on_scc_stack = <span class="bool-val">false</span>;
                    node.scc_id = scc_id;
                    node.discovery = index_in_scc;
                    index_in_scc += <span class="number">1</span>;
                    <span class="kw">if </span>w == v {
                        <span class="kw">break</span>;
                    }
                }
                scc_id += <span class="number">1</span>;
            }
            <span class="kw">if let </span><span class="prelude-val">Some</span>((w, <span class="kw">_</span>)) = recursion_stack.last() {
                nodes[<span class="kw-2">*</span>w].lowlink = nodes[<span class="kw-2">*</span>w].lowlink.min(nodes[v].lowlink);
            } <span class="kw">else </span>{
                <span class="kw">loop </span>{
                    <span class="kw">if </span>next_v == nodes.len() {
                        <span class="kw">break </span><span class="lifetime">'dfs</span>;
                    }
                    <span class="kw">if </span>nodes[next_v].discovery == usize::MAX {
                        <span class="kw">break</span>;
                    }
                    next_v += <span class="number">1</span>;
                }
                recursion_stack.push((next_v, <span class="number">0</span>));
                next_v += <span class="number">1</span>;
            }
        }
        <span class="kw">for </span>node <span class="kw">in </span><span class="kw-2">&amp;</span>nodes {
            <span class="kw">let </span>cfg_node = <span class="self">self</span>.cfg_nodes.get_mut(<span class="kw-2">&amp;</span>node.cfg_node).unwrap();
            cfg_node.topo_index = TopologicalIndex {
                scc_id: node.scc_id,
                discovery: node.discovery,
            };
        }
        <span class="kw">let </span><span class="kw-2">mut </span>topological_order = <span class="self">self</span>.cfg_nodes.keys().cloned().collect::&lt;Vec&lt;<span class="kw">_</span>&gt;&gt;();
        topological_order.sort_by(|a, b| {
            <span class="self">self</span>.cfg_nodes[b]
                .topo_index
                .cmp(<span class="kw-2">&amp;</span><span class="self">self</span>.cfg_nodes[a].topo_index)
        });
        <span class="self">self</span>.topological_order = topological_order;
        <span class="kw">let </span><span class="kw-2">mut </span>super_root = CfgNode {
            instructions: <span class="self">self</span>.instructions.len()..<span class="self">self</span>.instructions.len(),
            ..CfgNode::default()
        };
        <span class="kw">let </span><span class="kw-2">mut </span>first_node = <span class="self">self</span>.topological_order.first().cloned();
        <span class="kw">let </span><span class="kw-2">mut </span>has_external_source = <span class="bool-val">false</span>;
        <span class="kw">for </span>(index, v) <span class="kw">in </span><span class="self">self</span>.topological_order.iter().enumerate() {
            <span class="kw">let </span>cfg_node = <span class="kw-2">&amp;</span><span class="self">self</span>.cfg_nodes[v];
            has_external_source |= cfg_node.sources.iter().any(|source| {
                <span class="self">self</span>.cfg_nodes[source].topo_index.scc_id != cfg_node.topo_index.scc_id
            });
            <span class="kw">if </span><span class="self">self
                </span>.topological_order
                .get(index + <span class="number">1</span>)
                .map(|next_v| {
                    <span class="self">self</span>.cfg_nodes[next_v].topo_index.scc_id != cfg_node.topo_index.scc_id
                })
                .unwrap_or(<span class="bool-val">true</span>)
            {
                <span class="kw">if </span>!has_external_source &amp;&amp; first_node != <span class="prelude-val">Some</span>(<span class="self">self</span>.super_root) {
                    super_root.destinations.push(first_node.unwrap());
                }
                first_node = <span class="self">self</span>.topological_order.get(index + <span class="number">1</span>).cloned();
                has_external_source = <span class="bool-val">false</span>;
            }
        }
        <span class="kw">for </span>v <span class="kw">in </span>super_root.destinations.iter() {
            <span class="kw">let </span>cfg_node = <span class="self">self</span>.cfg_nodes.get_mut(v).unwrap();
            cfg_node.sources.push(<span class="self">self</span>.super_root);
            <span class="self">self</span>.functions.entry(<span class="kw-2">*</span>v).or_insert_with(|| {
                <span class="kw">let </span>name = <span class="macro">format!</span>(<span class="string">"function_{}"</span>, <span class="kw-2">*</span>v);
                <span class="kw">let </span>hash = ebpf::hash_symbol_name(name.as_bytes());
                (hash, name)
            });
        }
        <span class="self">self</span>.cfg_nodes.insert(<span class="self">self</span>.super_root, super_root);
    }

    <span class="kw">fn </span>control_flow_graph_dominance_intersect(<span class="kw-2">&amp;</span><span class="self">self</span>, <span class="kw-2">mut </span>a: usize, <span class="kw-2">mut </span>b: usize) -&gt; usize {
        <span class="kw">while </span>a != b {
            <span class="kw">match </span><span class="self">self</span>.cfg_nodes[<span class="kw-2">&amp;</span>a]
                .topo_index
                .cmp(<span class="kw-2">&amp;</span><span class="self">self</span>.cfg_nodes[<span class="kw-2">&amp;</span>b].topo_index)
            {
                std::cmp::Ordering::Greater =&gt; {
                    b = <span class="self">self</span>.cfg_nodes[<span class="kw-2">&amp;</span>b].dominator_parent;
                }
                std::cmp::Ordering::Less =&gt; {
                    a = <span class="self">self</span>.cfg_nodes[<span class="kw-2">&amp;</span>a].dominator_parent;
                }
                std::cmp::Ordering::Equal =&gt; <span class="macro">unreachable!</span>(),
            }
        }
        b
    }

    <span class="doccomment">/// Finds the dominance hierarchy of the control-flow graph
    ///
    /// Uses the Cooper-Harvey-Kennedy algorithm.
    </span><span class="kw">pub fn </span>control_flow_graph_dominance_hierarchy(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="kw">if </span><span class="self">self</span>.cfg_nodes.is_empty() {
            <span class="kw">return</span>;
        }
        <span class="self">self</span>.cfg_nodes
            .get_mut(<span class="kw-2">&amp;</span><span class="self">self</span>.super_root)
            .unwrap()
            .dominator_parent = <span class="self">self</span>.super_root;
        <span class="kw">loop </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>terminate = <span class="bool-val">true</span>;
            <span class="kw">for </span>b <span class="kw">in </span><span class="self">self</span>.topological_order.iter() {
                <span class="kw">let </span>cfg_node = <span class="kw-2">&amp;</span><span class="self">self</span>.cfg_nodes[b];
                <span class="kw">let </span><span class="kw-2">mut </span>dominator_parent = usize::MAX;
                <span class="kw">for </span>p <span class="kw">in </span>cfg_node.sources.iter() {
                    <span class="kw">if </span><span class="self">self</span>.cfg_nodes[p].dominator_parent == usize::MAX {
                        <span class="kw">continue</span>;
                    }
                    dominator_parent = <span class="kw">if </span>dominator_parent == usize::MAX {
                        <span class="kw-2">*</span>p
                    } <span class="kw">else </span>{
                        <span class="self">self</span>.control_flow_graph_dominance_intersect(<span class="kw-2">*</span>p, dominator_parent)
                    };
                }
                <span class="kw">if </span>cfg_node.dominator_parent != dominator_parent {
                    <span class="kw">let </span>cfg_node = <span class="self">self</span>.cfg_nodes.get_mut(b).unwrap();
                    cfg_node.dominator_parent = dominator_parent;
                    terminate = <span class="bool-val">false</span>;
                }
            }
            <span class="kw">if </span>terminate {
                <span class="kw">break</span>;
            }
        }
        <span class="kw">for </span>b <span class="kw">in </span><span class="self">self</span>.topological_order.iter() {
            <span class="kw">let </span>cfg_node = <span class="kw-2">&amp;</span><span class="self">self</span>.cfg_nodes[b];
            <span class="macro">assert_ne!</span>(cfg_node.dominator_parent, usize::MAX);
            <span class="kw">if </span><span class="kw-2">*</span>b == cfg_node.dominator_parent {
                <span class="kw">continue</span>;
            }
            <span class="kw">let </span>p = cfg_node.dominator_parent;
            <span class="kw">let </span>dominator_cfg_node = <span class="self">self</span>.cfg_nodes.get_mut(<span class="kw-2">&amp;</span>p).unwrap();
            dominator_cfg_node.dominated_children.push(<span class="kw-2">*</span>b);
        }
    }

    <span class="doccomment">/// Connect the dependencies between the instructions inside of the basic blocks
    </span><span class="kw">pub fn </span>intra_basic_block_data_flow(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; BTreeMap&lt;usize, HashMap&lt;DataResource, usize&gt;&gt; {
        <span class="kw">fn </span>bind(
            state: <span class="kw-2">&amp;mut </span>(
                usize,
                BTreeMap&lt;DfgNode, BTreeSet&lt;DfgEdge&gt;&gt;,
                HashMap&lt;DataResource, usize&gt;,
            ),
            insn: <span class="kw-2">&amp;</span>ebpf::Insn,
            is_output: bool,
            resource: DataResource,
        ) {
            <span class="kw">let </span>kind = <span class="kw">if </span>is_output {
                DfgEdgeKind::Empty
            } <span class="kw">else </span>{
                DfgEdgeKind::Filled
            };
            <span class="kw">let </span>source = <span class="kw">if let </span><span class="prelude-val">Some</span>(source) = state.<span class="number">2</span>.get(<span class="kw-2">&amp;</span>resource) {
                DfgNode::InstructionNode(<span class="kw-2">*</span>source)
            } <span class="kw">else </span>{
                DfgNode::PhiNode(state.<span class="number">0</span>)
            };
            <span class="kw">let </span>destination = DfgNode::InstructionNode(insn.ptr);
            state.<span class="number">1</span>.entry(source.clone()).or_default().insert(DfgEdge {
                source,
                destination,
                kind,
                resource: resource.clone(),
            });
            <span class="kw">if </span>is_output {
                state.<span class="number">2</span>.insert(resource, insn.ptr);
            }
        }
        <span class="kw">let </span><span class="kw-2">mut </span>state = (<span class="number">0</span>, BTreeMap::new(), HashMap::new());
        <span class="kw">let </span>data_dependencies = <span class="self">self
            </span>.cfg_nodes
            .iter()
            .map(|(basic_block_start, basic_block)| {
                state.<span class="number">0 </span>= <span class="kw-2">*</span>basic_block_start;
                <span class="kw">for </span>insn <span class="kw">in </span><span class="self">self</span>.instructions[basic_block.instructions.clone()].iter() {
                    <span class="kw">match </span>insn.opc {
                        ebpf::LD_DW_IMM =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">true</span>, DataResource::Register(insn.dst));
                        }
                        ebpf::LD_B_REG | ebpf::LD_H_REG | ebpf::LD_W_REG | ebpf::LD_DW_REG =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Memory);
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.src));
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">true</span>, DataResource::Register(insn.dst));
                        }
                        ebpf::ST_B_IMM | ebpf::ST_H_IMM | ebpf::ST_W_IMM | ebpf::ST_DW_IMM =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.dst));
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">true</span>, DataResource::Memory);
                        }
                        ebpf::ST_B_REG | ebpf::ST_H_REG | ebpf::ST_W_REG | ebpf::ST_DW_REG =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.src));
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.dst));
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">true</span>, DataResource::Memory);
                        }
                        ebpf::ADD32_IMM
                        | ebpf::SUB32_IMM
                        | ebpf::MUL32_IMM
                        | ebpf::DIV32_IMM
                        | ebpf::SDIV32_IMM
                        | ebpf::OR32_IMM
                        | ebpf::AND32_IMM
                        | ebpf::LSH32_IMM
                        | ebpf::RSH32_IMM
                        | ebpf::MOD32_IMM
                        | ebpf::XOR32_IMM
                        | ebpf::ARSH32_IMM
                        | ebpf::ADD64_IMM
                        | ebpf::SUB64_IMM
                        | ebpf::MUL64_IMM
                        | ebpf::DIV64_IMM
                        | ebpf::SDIV64_IMM
                        | ebpf::OR64_IMM
                        | ebpf::AND64_IMM
                        | ebpf::LSH64_IMM
                        | ebpf::RSH64_IMM
                        | ebpf::MOD64_IMM
                        | ebpf::XOR64_IMM
                        | ebpf::ARSH64_IMM
                        | ebpf::HOR64_IMM
                        | ebpf::NEG32
                        | ebpf::NEG64
                        | ebpf::LE
                        | ebpf::BE =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.dst));
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">true</span>, DataResource::Register(insn.dst));
                        }
                        ebpf::MOV32_IMM | ebpf::MOV64_IMM =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">true</span>, DataResource::Register(insn.dst));
                        }
                        ebpf::ADD32_REG
                        | ebpf::SUB32_REG
                        | ebpf::MUL32_REG
                        | ebpf::DIV32_REG
                        | ebpf::SDIV32_REG
                        | ebpf::OR32_REG
                        | ebpf::AND32_REG
                        | ebpf::LSH32_REG
                        | ebpf::RSH32_REG
                        | ebpf::MOD32_REG
                        | ebpf::XOR32_REG
                        | ebpf::ARSH32_REG
                        | ebpf::ADD64_REG
                        | ebpf::SUB64_REG
                        | ebpf::MUL64_REG
                        | ebpf::DIV64_REG
                        | ebpf::SDIV64_REG
                        | ebpf::OR64_REG
                        | ebpf::AND64_REG
                        | ebpf::LSH64_REG
                        | ebpf::RSH64_REG
                        | ebpf::MOD64_REG
                        | ebpf::XOR64_REG
                        | ebpf::ARSH64_REG =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.src));
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.dst));
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">true</span>, DataResource::Register(insn.dst));
                        }
                        ebpf::MOV32_REG | ebpf::MOV64_REG =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.src));
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">true</span>, DataResource::Register(insn.dst));
                        }
                        ebpf::JEQ_IMM
                        | ebpf::JGT_IMM
                        | ebpf::JGE_IMM
                        | ebpf::JLT_IMM
                        | ebpf::JLE_IMM
                        | ebpf::JSET_IMM
                        | ebpf::JNE_IMM
                        | ebpf::JSGT_IMM
                        | ebpf::JSGE_IMM
                        | ebpf::JSLT_IMM
                        | ebpf::JSLE_IMM =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.dst));
                        }
                        ebpf::JEQ_REG
                        | ebpf::JGT_REG
                        | ebpf::JGE_REG
                        | ebpf::JLT_REG
                        | ebpf::JLE_REG
                        | ebpf::JSET_REG
                        | ebpf::JNE_REG
                        | ebpf::JSGT_REG
                        | ebpf::JSGE_REG
                        | ebpf::JSLT_REG
                        | ebpf::JSLE_REG =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.src));
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(insn.dst));
                        }
                        ebpf::CALL_REG | ebpf::CALL_IMM =&gt; {
                            <span class="kw">if </span>insn.opc == ebpf::CALL_REG
                                &amp;&amp; !(ebpf::FIRST_SCRATCH_REG
                                    ..ebpf::FIRST_SCRATCH_REG + ebpf::SCRATCH_REGS)
                                    .contains(<span class="kw-2">&amp;</span>(insn.imm <span class="kw">as </span>usize))
                            {
                                bind(
                                    <span class="kw-2">&amp;mut </span>state,
                                    insn,
                                    <span class="bool-val">false</span>,
                                    DataResource::Register(insn.imm <span class="kw">as </span>u8),
                                );
                            }
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Memory);
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">true</span>, DataResource::Memory);
                            <span class="kw">for </span>reg <span class="kw">in </span>(<span class="number">0</span>..ebpf::FIRST_SCRATCH_REG).chain([<span class="number">10</span>].iter().cloned()) {
                                bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(reg <span class="kw">as </span>u8));
                                bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">true</span>, DataResource::Register(reg <span class="kw">as </span>u8));
                            }
                        }
                        ebpf::EXIT =&gt; {
                            bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Memory);
                            <span class="kw">for </span>reg <span class="kw">in </span>(<span class="number">0</span>..ebpf::FIRST_SCRATCH_REG).chain([<span class="number">10</span>].iter().cloned()) {
                                bind(<span class="kw-2">&amp;mut </span>state, insn, <span class="bool-val">false</span>, DataResource::Register(reg <span class="kw">as </span>u8));
                            }
                        }
                        <span class="kw">_ </span>=&gt; {}
                    }
                }
                <span class="kw">let </span><span class="kw-2">mut </span>deps = HashMap::new();
                std::mem::swap(<span class="kw-2">&amp;mut </span>deps, <span class="kw-2">&amp;mut </span>state.<span class="number">2</span>);
                (<span class="kw-2">*</span>basic_block_start, deps)
            })
            .collect();
        <span class="self">self</span>.dfg_forward_edges = state.<span class="number">1</span>;
        data_dependencies
    }

    <span class="doccomment">/// Connect the dependencies inbetween the basic blocks
    </span><span class="kw">pub fn </span>inter_basic_block_data_flow(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        basic_block_outputs: BTreeMap&lt;usize, HashMap&lt;DataResource, usize&gt;&gt;,
    ) {
        <span class="kw">let </span><span class="kw-2">mut </span>continue_propagation = <span class="bool-val">true</span>;
        <span class="kw">while </span>continue_propagation {
            continue_propagation = <span class="bool-val">false</span>;
            <span class="kw">for </span>basic_block_start <span class="kw">in </span><span class="self">self</span>.topological_order.iter().rev() {
                <span class="kw">if </span>!<span class="self">self
                    </span>.dfg_forward_edges
                    .contains_key(<span class="kw-2">&amp;</span>DfgNode::PhiNode(<span class="kw-2">*</span>basic_block_start))
                {
                    <span class="kw">continue</span>;
                }
                <span class="kw">let </span>basic_block = <span class="kw-2">&amp;</span><span class="self">self</span>.cfg_nodes[basic_block_start];
                <span class="kw">let </span><span class="kw-2">mut </span>edges = BTreeSet::new();
                std::mem::swap(
                    <span class="self">self</span>.dfg_forward_edges
                        .get_mut(<span class="kw-2">&amp;</span>DfgNode::PhiNode(<span class="kw-2">*</span>basic_block_start))
                        .unwrap(),
                    <span class="kw-2">&amp;mut </span>edges,
                );
                <span class="kw">for </span>predecessor <span class="kw">in </span>basic_block.sources.iter() {
                    <span class="kw">let </span>provided_outputs = <span class="kw-2">&amp;</span>basic_block_outputs[predecessor];
                    <span class="kw">for </span>edge <span class="kw">in </span>edges.iter() {
                        <span class="kw">let </span><span class="kw-2">mut </span>source_is_a_phi_node = <span class="bool-val">false</span>;
                        <span class="kw">let </span>source = <span class="kw">if let </span><span class="prelude-val">Some</span>(source) = provided_outputs.get(<span class="kw-2">&amp;</span>edge.resource) {
                            DfgNode::InstructionNode(<span class="kw-2">*</span>source)
                        } <span class="kw">else </span>{
                            source_is_a_phi_node = <span class="bool-val">true</span>;
                            DfgNode::PhiNode(<span class="kw-2">*</span>predecessor)
                        };
                        <span class="kw">let </span><span class="kw-2">mut </span>edge = edge.clone();
                        <span class="kw">if </span>basic_block.sources.len() != <span class="number">1 </span>{
                            edge.destination = DfgNode::PhiNode(<span class="kw-2">*</span>basic_block_start);
                        }
                        <span class="kw">if </span><span class="self">self
                            </span>.dfg_forward_edges
                            .entry(source.clone())
                            .or_default()
                            .insert(edge.clone())
                            &amp;&amp; source_is_a_phi_node
                            &amp;&amp; source != DfgNode::PhiNode(<span class="kw-2">*</span>basic_block_start)
                        {
                            continue_propagation = <span class="bool-val">true</span>;
                        }
                    }
                }
                <span class="kw">let </span>reflective_edges = <span class="self">self
                    </span>.dfg_forward_edges
                    .get_mut(<span class="kw-2">&amp;</span>DfgNode::PhiNode(<span class="kw-2">*</span>basic_block_start))
                    .unwrap();
                <span class="kw">for </span>edge <span class="kw">in </span>reflective_edges.iter() {
                    <span class="kw">if </span>edges.insert(edge.clone()) {
                        continue_propagation = <span class="bool-val">true</span>;
                    }
                }
                std::mem::swap(reflective_edges, <span class="kw-2">&amp;mut </span>edges);
            }
        }
        <span class="kw">for </span>(basic_block_start, basic_block) <span class="kw">in </span><span class="self">self</span>.cfg_nodes.iter() {
            <span class="kw">if </span>basic_block.sources.len() == <span class="number">1 </span>{
                <span class="self">self</span>.dfg_forward_edges
                    .remove(<span class="kw-2">&amp;</span>DfgNode::PhiNode(<span class="kw-2">*</span>basic_block_start));
            }
        }
        <span class="kw">for </span>dfg_edges <span class="kw">in </span><span class="self">self</span>.dfg_forward_edges.values() {
            <span class="kw">for </span>dfg_edge <span class="kw">in </span>dfg_edges.iter() {
                <span class="self">self</span>.dfg_reverse_edges
                    .entry(dfg_edge.destination.clone())
                    .or_default()
                    .insert(dfg_edge.clone());
            }
        }
    }
}
</code></pre></div></section></main></body></html>