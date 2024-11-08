<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/mpl-token-metadata-4.1.2/src/generated/errors/mpl_token_metadata.rs`."><title>mpl_token_metadata.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../" data-static-root-path="../../../../static.files/" data-current-crate="mpl_token_metadata" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../src-files.js"></script><script defer src="../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../mpl_token_metadata/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

</span><span class="kw">use </span>num_derive::FromPrimitive;
<span class="kw">use </span>thiserror::Error;

<span class="attr">#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
</span><span class="kw">pub enum </span>MplTokenMetadataError {
    <span class="doccomment">/// 0 (0x0) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>InstructionUnpackError,
    <span class="doccomment">/// 1 (0x1) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>InstructionPackError,
    <span class="doccomment">/// 2 (0x2) - Lamport balance below rent-exempt threshold
    </span><span class="attr">#[error(<span class="string">"Lamport balance below rent-exempt threshold"</span>)]
    </span>NotRentExempt,
    <span class="doccomment">/// 3 (0x3) - Already initialized
    </span><span class="attr">#[error(<span class="string">"Already initialized"</span>)]
    </span>AlreadyInitialized,
    <span class="doccomment">/// 4 (0x4) - Uninitialized
    </span><span class="attr">#[error(<span class="string">"Uninitialized"</span>)]
    </span>Uninitialized,
    <span class="doccomment">/// 5 (0x5) -  Metadata's key must match seed of ['metadata', program id, mint] provided
    </span><span class="attr">#[error(<span class="string">" Metadata's key must match seed of ['metadata', program id, mint] provided"</span>)]
    </span>InvalidMetadataKey,
    <span class="doccomment">/// 6 (0x6) - Edition's key must match seed of ['metadata', program id, name, 'edition'] provided
    </span><span class="attr">#[error(<span class="string">"Edition's key must match seed of ['metadata', program id, name, 'edition'] provided"</span>)]
    </span>InvalidEditionKey,
    <span class="doccomment">/// 7 (0x7) - Update Authority given does not match
    </span><span class="attr">#[error(<span class="string">"Update Authority given does not match"</span>)]
    </span>UpdateAuthorityIncorrect,
    <span class="doccomment">/// 8 (0x8) - Update Authority needs to be signer to update metadata
    </span><span class="attr">#[error(<span class="string">"Update Authority needs to be signer to update metadata"</span>)]
    </span>UpdateAuthorityIsNotSigner,
    <span class="doccomment">/// 9 (0x9) - You must be the mint authority and signer on this transaction
    </span><span class="attr">#[error(<span class="string">"You must be the mint authority and signer on this transaction"</span>)]
    </span>NotMintAuthority,
    <span class="doccomment">/// 10 (0xA) - Mint authority provided does not match the authority on the mint
    </span><span class="attr">#[error(<span class="string">"Mint authority provided does not match the authority on the mint"</span>)]
    </span>InvalidMintAuthority,
    <span class="doccomment">/// 11 (0xB) - Name too long
    </span><span class="attr">#[error(<span class="string">"Name too long"</span>)]
    </span>NameTooLong,
    <span class="doccomment">/// 12 (0xC) - Symbol too long
    </span><span class="attr">#[error(<span class="string">"Symbol too long"</span>)]
    </span>SymbolTooLong,
    <span class="doccomment">/// 13 (0xD) - URI too long
    </span><span class="attr">#[error(<span class="string">"URI too long"</span>)]
    </span>UriTooLong,
    <span class="doccomment">/// 14 (0xE) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner,
    <span class="doccomment">/// 15 (0xF) - Mint given does not match mint on Metadata
    </span><span class="attr">#[error(<span class="string">"Mint given does not match mint on Metadata"</span>)]
    </span>MintMismatch,
    <span class="doccomment">/// 16 (0x10) - Editions must have exactly one token
    </span><span class="attr">#[error(<span class="string">"Editions must have exactly one token"</span>)]
    </span>EditionsMustHaveExactlyOneToken,
    <span class="doccomment">/// 17 (0x11) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>MaxEditionsMintedAlready,
    <span class="doccomment">/// 18 (0x12) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>TokenMintToFailed,
    <span class="doccomment">/// 19 (0x13) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>MasterRecordMismatch,
    <span class="doccomment">/// 20 (0x14) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>DestinationMintMismatch,
    <span class="doccomment">/// 21 (0x15) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>EditionAlreadyMinted,
    <span class="doccomment">/// 22 (0x16) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>PrintingMintDecimalsShouldBeZero,
    <span class="doccomment">/// 23 (0x17) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>OneTimePrintingAuthorizationMintDecimalsShouldBeZero,
    <span class="doccomment">/// 24 (0x18) - EditionMintDecimalsShouldBeZero
    </span><span class="attr">#[error(<span class="string">"EditionMintDecimalsShouldBeZero"</span>)]
    </span>EditionMintDecimalsShouldBeZero,
    <span class="doccomment">/// 25 (0x19) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>TokenBurnFailed,
    <span class="doccomment">/// 26 (0x1A) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>TokenAccountOneTimeAuthMintMismatch,
    <span class="doccomment">/// 27 (0x1B) - Derived key invalid
    </span><span class="attr">#[error(<span class="string">"Derived key invalid"</span>)]
    </span>DerivedKeyInvalid,
    <span class="doccomment">/// 28 (0x1C) - The Printing mint does not match that on the master edition!
    </span><span class="attr">#[error(<span class="string">"The Printing mint does not match that on the master edition!"</span>)]
    </span>PrintingMintMismatch,
    <span class="doccomment">/// 29 (0x1D) - The One Time Printing Auth mint does not match that on the master edition!
    </span><span class="attr">#[error(<span class="string">"The One Time Printing Auth mint does not match that on the master edition!"</span>)]
    </span>OneTimePrintingAuthMintMismatch,
    <span class="doccomment">/// 30 (0x1E) - The mint of the token account does not match the Printing mint!
    </span><span class="attr">#[error(<span class="string">"The mint of the token account does not match the Printing mint!"</span>)]
    </span>TokenAccountMintMismatch,
    <span class="doccomment">/// 31 (0x1F) - The mint of the token account does not match the master metadata mint!
    </span><span class="attr">#[error(<span class="string">"The mint of the token account does not match the master metadata mint!"</span>)]
    </span>TokenAccountMintMismatchV2,
    <span class="doccomment">/// 32 (0x20) - Not enough tokens to mint a limited edition
    </span><span class="attr">#[error(<span class="string">"Not enough tokens to mint a limited edition"</span>)]
    </span>NotEnoughTokens,
    <span class="doccomment">/// 33 (0x21) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>PrintingMintAuthorizationAccountMismatch,
    <span class="doccomment">/// 34 (0x22) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>AuthorizationTokenAccountOwnerMismatch,
    <span class="doccomment">/// 35 (0x23) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>Disabled,
    <span class="doccomment">/// 36 (0x24) - Creators list too long
    </span><span class="attr">#[error(<span class="string">"Creators list too long"</span>)]
    </span>CreatorsTooLong,
    <span class="doccomment">/// 37 (0x25) - Creators must be at least one if set
    </span><span class="attr">#[error(<span class="string">"Creators must be at least one if set"</span>)]
    </span>CreatorsMustBeAtleastOne,
    <span class="doccomment">/// 38 (0x26) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>MustBeOneOfCreators,
    <span class="doccomment">/// 39 (0x27) - This metadata does not have creators
    </span><span class="attr">#[error(<span class="string">"This metadata does not have creators"</span>)]
    </span>NoCreatorsPresentOnMetadata,
    <span class="doccomment">/// 40 (0x28) - This creator address was not found
    </span><span class="attr">#[error(<span class="string">"This creator address was not found"</span>)]
    </span>CreatorNotFound,
    <span class="doccomment">/// 41 (0x29) - Basis points cannot be more than 10000
    </span><span class="attr">#[error(<span class="string">"Basis points cannot be more than 10000"</span>)]
    </span>InvalidBasisPoints,
    <span class="doccomment">/// 42 (0x2A) - Primary sale can only be flipped to true and is immutable
    </span><span class="attr">#[error(<span class="string">"Primary sale can only be flipped to true and is immutable"</span>)]
    </span>PrimarySaleCanOnlyBeFlippedToTrue,
    <span class="doccomment">/// 43 (0x2B) - Owner does not match that on the account given
    </span><span class="attr">#[error(<span class="string">"Owner does not match that on the account given"</span>)]
    </span>OwnerMismatch,
    <span class="doccomment">/// 44 (0x2C) - This account has no tokens to be used for authorization
    </span><span class="attr">#[error(<span class="string">"This account has no tokens to be used for authorization"</span>)]
    </span>NoBalanceInAccountForAuthorization,
    <span class="doccomment">/// 45 (0x2D) - Share total must equal 100 for creator array
    </span><span class="attr">#[error(<span class="string">"Share total must equal 100 for creator array"</span>)]
    </span>ShareTotalMustBe100,
    <span class="doccomment">/// 46 (0x2E) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>ReservationExists,
    <span class="doccomment">/// 47 (0x2F) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>ReservationDoesNotExist,
    <span class="doccomment">/// 48 (0x30) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>ReservationNotSet,
    <span class="doccomment">/// 49 (0x31) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>ReservationAlreadyMade,
    <span class="doccomment">/// 50 (0x32) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>BeyondMaxAddressSize,
    <span class="doccomment">/// 51 (0x33) - NumericalOverflowError
    </span><span class="attr">#[error(<span class="string">"NumericalOverflowError"</span>)]
    </span>NumericalOverflowError,
    <span class="doccomment">/// 52 (0x34) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>ReservationBreachesMaximumSupply,
    <span class="doccomment">/// 53 (0x35) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>AddressNotInReservation,
    <span class="doccomment">/// 54 (0x36) - You cannot unilaterally verify another creator, they must sign
    </span><span class="attr">#[error(<span class="string">"You cannot unilaterally verify another creator, they must sign"</span>)]
    </span>CannotVerifyAnotherCreator,
    <span class="doccomment">/// 55 (0x37) - You cannot unilaterally unverify another creator
    </span><span class="attr">#[error(<span class="string">"You cannot unilaterally unverify another creator"</span>)]
    </span>CannotUnverifyAnotherCreator,
    <span class="doccomment">/// 56 (0x38) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>SpotMismatch,
    <span class="doccomment">/// 57 (0x39) - Incorrect account owner
    </span><span class="attr">#[error(<span class="string">"Incorrect account owner"</span>)]
    </span>IncorrectOwner,
    <span class="doccomment">/// 58 (0x3A) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>PrintingWouldBreachMaximumSupply,
    <span class="doccomment">/// 59 (0x3B) - Data is immutable
    </span><span class="attr">#[error(<span class="string">"Data is immutable"</span>)]
    </span>DataIsImmutable,
    <span class="doccomment">/// 60 (0x3C) - No duplicate creator addresses
    </span><span class="attr">#[error(<span class="string">"No duplicate creator addresses"</span>)]
    </span>DuplicateCreatorAddress,
    <span class="doccomment">/// 61 (0x3D) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>ReservationSpotsRemainingShouldMatchTotalSpotsAtStart,
    <span class="doccomment">/// 62 (0x3E) - Invalid token program
    </span><span class="attr">#[error(<span class="string">"Invalid token program"</span>)]
    </span>InvalidTokenProgram,
    <span class="doccomment">/// 63 (0x3F) - Data type mismatch
    </span><span class="attr">#[error(<span class="string">"Data type mismatch"</span>)]
    </span>DataTypeMismatch,
    <span class="doccomment">/// 64 (0x40) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>BeyondAlottedAddressSize,
    <span class="doccomment">/// 65 (0x41) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>ReservationNotComplete,
    <span class="doccomment">/// 66 (0x42) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>TriedToReplaceAnExistingReservation,
    <span class="doccomment">/// 67 (0x43) - Invalid operation
    </span><span class="attr">#[error(<span class="string">"Invalid operation"</span>)]
    </span>InvalidOperation,
    <span class="doccomment">/// 68 (0x44) - Invalid Owner
    </span><span class="attr">#[error(<span class="string">"Invalid Owner"</span>)]
    </span>InvalidOwner,
    <span class="doccomment">/// 69 (0x45) - Printing mint supply must be zero for conversion
    </span><span class="attr">#[error(<span class="string">"Printing mint supply must be zero for conversion"</span>)]
    </span>PrintingMintSupplyMustBeZeroForConversion,
    <span class="doccomment">/// 70 (0x46) - One Time Auth mint supply must be zero for conversion
    </span><span class="attr">#[error(<span class="string">"One Time Auth mint supply must be zero for conversion"</span>)]
    </span>OneTimeAuthMintSupplyMustBeZeroForConversion,
    <span class="doccomment">/// 71 (0x47) - You tried to insert one edition too many into an edition mark pda
    </span><span class="attr">#[error(<span class="string">"You tried to insert one edition too many into an edition mark pda"</span>)]
    </span>InvalidEditionIndex,
    <span class="doccomment">/// 72 (0x48) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>ReservationArrayShouldBeSizeOne,
    <span class="doccomment">/// 73 (0x49) - Is Mutable can only be flipped to false
    </span><span class="attr">#[error(<span class="string">"Is Mutable can only be flipped to false"</span>)]
    </span>IsMutableCanOnlyBeFlippedToFalse,
    <span class="doccomment">/// 74 (0x4A) - Collection cannot be verified in this instruction
    </span><span class="attr">#[error(<span class="string">"Collection cannot be verified in this instruction"</span>)]
    </span>CollectionCannotBeVerifiedInThisInstruction,
    <span class="doccomment">/// 75 (0x4B) - This instruction was deprecated in a previous release and is now removed
    </span><span class="attr">#[error(<span class="string">"This instruction was deprecated in a previous release and is now removed"</span>)]
    </span>Removed,
    <span class="doccomment">/// 76 (0x4C) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>MustBeBurned,
    <span class="doccomment">/// 77 (0x4D) - This use method is invalid
    </span><span class="attr">#[error(<span class="string">"This use method is invalid"</span>)]
    </span>InvalidUseMethod,
    <span class="doccomment">/// 78 (0x4E) - Cannot Change Use Method after the first use
    </span><span class="attr">#[error(<span class="string">"Cannot Change Use Method after the first use"</span>)]
    </span>CannotChangeUseMethodAfterFirstUse,
    <span class="doccomment">/// 79 (0x4F) - Cannot Change Remaining or Available uses after the first use
    </span><span class="attr">#[error(<span class="string">"Cannot Change Remaining or Available uses after the first use"</span>)]
    </span>CannotChangeUsesAfterFirstUse,
    <span class="doccomment">/// 80 (0x50) - Collection Not Found on Metadata
    </span><span class="attr">#[error(<span class="string">"Collection Not Found on Metadata"</span>)]
    </span>CollectionNotFound,
    <span class="doccomment">/// 81 (0x51) - Collection Update Authority is invalid
    </span><span class="attr">#[error(<span class="string">"Collection Update Authority is invalid"</span>)]
    </span>InvalidCollectionUpdateAuthority,
    <span class="doccomment">/// 82 (0x52) - Collection Must Be a Unique Master Edition v2
    </span><span class="attr">#[error(<span class="string">"Collection Must Be a Unique Master Edition v2"</span>)]
    </span>CollectionMustBeAUniqueMasterEdition,
    <span class="doccomment">/// 83 (0x53) - The Use Authority Record Already Exists, to modify it Revoke, then Approve
    </span><span class="attr">#[error(<span class="string">"The Use Authority Record Already Exists, to modify it Revoke, then Approve"</span>)]
    </span>UseAuthorityRecordAlreadyExists,
    <span class="doccomment">/// 84 (0x54) - The Use Authority Record is empty or already revoked
    </span><span class="attr">#[error(<span class="string">"The Use Authority Record is empty or already revoked"</span>)]
    </span>UseAuthorityRecordAlreadyRevoked,
    <span class="doccomment">/// 85 (0x55) - This token has no uses
    </span><span class="attr">#[error(<span class="string">"This token has no uses"</span>)]
    </span>Unusable,
    <span class="doccomment">/// 86 (0x56) - There are not enough Uses left on this token.
    </span><span class="attr">#[error(<span class="string">"There are not enough Uses left on this token."</span>)]
    </span>NotEnoughUses,
    <span class="doccomment">/// 87 (0x57) - This Collection Authority Record Already Exists.
    </span><span class="attr">#[error(<span class="string">"This Collection Authority Record Already Exists."</span>)]
    </span>CollectionAuthorityRecordAlreadyExists,
    <span class="doccomment">/// 88 (0x58) - This Collection Authority Record Does Not Exist.
    </span><span class="attr">#[error(<span class="string">"This Collection Authority Record Does Not Exist."</span>)]
    </span>CollectionAuthorityDoesNotExist,
    <span class="doccomment">/// 89 (0x59) - This Use Authority Record is invalid.
    </span><span class="attr">#[error(<span class="string">"This Use Authority Record is invalid."</span>)]
    </span>InvalidUseAuthorityRecord,
    <span class="doccomment">/// 90 (0x5A) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>InvalidCollectionAuthorityRecord,
    <span class="doccomment">/// 91 (0x5B) - Metadata does not match the freeze authority on the mint
    </span><span class="attr">#[error(<span class="string">"Metadata does not match the freeze authority on the mint"</span>)]
    </span>InvalidFreezeAuthority,
    <span class="doccomment">/// 92 (0x5C) - All tokens in this account have not been delegated to this user.
    </span><span class="attr">#[error(<span class="string">"All tokens in this account have not been delegated to this user."</span>)]
    </span>InvalidDelegate,
    <span class="doccomment">/// 93 (0x5D) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>CannotAdjustVerifiedCreator,
    <span class="doccomment">/// 94 (0x5E) - Verified creators cannot be removed.
    </span><span class="attr">#[error(<span class="string">"Verified creators cannot be removed."</span>)]
    </span>CannotRemoveVerifiedCreator,
    <span class="doccomment">/// 95 (0x5F) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>CannotWipeVerifiedCreators,
    <span class="doccomment">/// 96 (0x60) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>NotAllowedToChangeSellerFeeBasisPoints,
    <span class="doccomment">/// 97 (0x61) - Edition override cannot be zero
    </span><span class="attr">#[error(<span class="string">"Edition override cannot be zero"</span>)]
    </span>EditionOverrideCannotBeZero,
    <span class="doccomment">/// 98 (0x62) - Invalid User
    </span><span class="attr">#[error(<span class="string">"Invalid User"</span>)]
    </span>InvalidUser,
    <span class="doccomment">/// 99 (0x63) - Revoke Collection Authority signer is incorrect
    </span><span class="attr">#[error(<span class="string">"Revoke Collection Authority signer is incorrect"</span>)]
    </span>RevokeCollectionAuthoritySignerIncorrect,
    <span class="doccomment">/// 100 (0x64) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>TokenCloseFailed,
    <span class="doccomment">/// 101 (0x65) - Can't use this function on unsized collection
    </span><span class="attr">#[error(<span class="string">"Can't use this function on unsized collection"</span>)]
    </span>UnsizedCollection,
    <span class="doccomment">/// 102 (0x66) - Can't use this function on a sized collection
    </span><span class="attr">#[error(<span class="string">"Can't use this function on a sized collection"</span>)]
    </span>SizedCollection,
    <span class="doccomment">/// 103 (0x67) - Missing collection metadata account
    </span><span class="attr">#[error(<span class="string">"Missing collection metadata account"</span>)]
    </span>MissingCollectionMetadata,
    <span class="doccomment">/// 104 (0x68) - This NFT is not a member of the specified collection.
    </span><span class="attr">#[error(<span class="string">"This NFT is not a member of the specified collection."</span>)]
    </span>NotAMemberOfCollection,
    <span class="doccomment">/// 105 (0x69) - This NFT is not a verified member of the specified collection.
    </span><span class="attr">#[error(<span class="string">"This NFT is not a verified member of the specified collection."</span>)]
    </span>NotVerifiedMemberOfCollection,
    <span class="doccomment">/// 106 (0x6A) - This NFT is not a collection parent NFT.
    </span><span class="attr">#[error(<span class="string">"This NFT is not a collection parent NFT."</span>)]
    </span>NotACollectionParent,
    <span class="doccomment">/// 107 (0x6B) - Could not determine a TokenStandard type.
    </span><span class="attr">#[error(<span class="string">"Could not determine a TokenStandard type."</span>)]
    </span>CouldNotDetermineTokenStandard,
    <span class="doccomment">/// 108 (0x6C) - This mint account has an edition but none was provided.
    </span><span class="attr">#[error(<span class="string">"This mint account has an edition but none was provided."</span>)]
    </span>MissingEditionAccount,
    <span class="doccomment">/// 109 (0x6D) - This edition is not a Master Edition
    </span><span class="attr">#[error(<span class="string">"This edition is not a Master Edition"</span>)]
    </span>NotAMasterEdition,
    <span class="doccomment">/// 110 (0x6E) - This Master Edition has existing prints
    </span><span class="attr">#[error(<span class="string">"This Master Edition has existing prints"</span>)]
    </span>MasterEditionHasPrints,
    <span class="doccomment">/// 111 (0x6F) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>BorshDeserializationError,
    <span class="doccomment">/// 112 (0x70) - Cannot update a verified collection in this command
    </span><span class="attr">#[error(<span class="string">"Cannot update a verified collection in this command"</span>)]
    </span>CannotUpdateVerifiedCollection,
    <span class="doccomment">/// 113 (0x71) - Edition account doesnt match collection
    </span><span class="attr">#[error(<span class="string">"Edition account doesnt match collection "</span>)]
    </span>CollectionMasterEditionAccountInvalid,
    <span class="doccomment">/// 114 (0x72) - Item is already verified.
    </span><span class="attr">#[error(<span class="string">"Item is already verified."</span>)]
    </span>AlreadyVerified,
    <span class="doccomment">/// 115 (0x73) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>AlreadyUnverified,
    <span class="doccomment">/// 116 (0x74) - This edition is not a Print Edition
    </span><span class="attr">#[error(<span class="string">"This edition is not a Print Edition"</span>)]
    </span>NotAPrintEdition,
    <span class="doccomment">/// 117 (0x75) - Invalid Master Edition
    </span><span class="attr">#[error(<span class="string">"Invalid Master Edition"</span>)]
    </span>InvalidMasterEdition,
    <span class="doccomment">/// 118 (0x76) - Invalid Print Edition
    </span><span class="attr">#[error(<span class="string">"Invalid Print Edition"</span>)]
    </span>InvalidPrintEdition,
    <span class="doccomment">/// 119 (0x77) - Invalid Edition Marker
    </span><span class="attr">#[error(<span class="string">"Invalid Edition Marker"</span>)]
    </span>InvalidEditionMarker,
    <span class="doccomment">/// 120 (0x78) - Reservation List is Deprecated
    </span><span class="attr">#[error(<span class="string">"Reservation List is Deprecated"</span>)]
    </span>ReservationListDeprecated,
    <span class="doccomment">/// 121 (0x79) - Print Edition does not match Master Edition
    </span><span class="attr">#[error(<span class="string">"Print Edition does not match Master Edition"</span>)]
    </span>PrintEditionDoesNotMatchMasterEdition,
    <span class="doccomment">/// 122 (0x7A) - Edition Number greater than max supply
    </span><span class="attr">#[error(<span class="string">"Edition Number greater than max supply"</span>)]
    </span>EditionNumberGreaterThanMaxSupply,
    <span class="doccomment">/// 123 (0x7B) - Must unverify before migrating collections.
    </span><span class="attr">#[error(<span class="string">"Must unverify before migrating collections."</span>)]
    </span>MustUnverify,
    <span class="doccomment">/// 124 (0x7C) - Invalid Escrow Account Bump Seed
    </span><span class="attr">#[error(<span class="string">"Invalid Escrow Account Bump Seed"</span>)]
    </span>InvalidEscrowBumpSeed,
    <span class="doccomment">/// 125 (0x7D) - Must Escrow Authority
    </span><span class="attr">#[error(<span class="string">"Must Escrow Authority"</span>)]
    </span>MustBeEscrowAuthority,
    <span class="doccomment">/// 126 (0x7E) - Invalid System Program
    </span><span class="attr">#[error(<span class="string">"Invalid System Program"</span>)]
    </span>InvalidSystemProgram,
    <span class="doccomment">/// 127 (0x7F) - Must be a Non Fungible Token
    </span><span class="attr">#[error(<span class="string">"Must be a Non Fungible Token"</span>)]
    </span>MustBeNonFungible,
    <span class="doccomment">/// 128 (0x80) - Insufficient tokens for transfer
    </span><span class="attr">#[error(<span class="string">"Insufficient tokens for transfer"</span>)]
    </span>InsufficientTokens,
    <span class="doccomment">/// 129 (0x81) - Borsh Serialization Error
    </span><span class="attr">#[error(<span class="string">"Borsh Serialization Error"</span>)]
    </span>BorshSerializationError,
    <span class="doccomment">/// 130 (0x82) - Cannot create NFT with no Freeze Authority.
    </span><span class="attr">#[error(<span class="string">"Cannot create NFT with no Freeze Authority."</span>)]
    </span>NoFreezeAuthoritySet,
    <span class="doccomment">/// 131 (0x83) - Invalid collection size change
    </span><span class="attr">#[error(<span class="string">"Invalid collection size change"</span>)]
    </span>InvalidCollectionSizeChange,
    <span class="doccomment">/// 132 (0x84) - Invalid bubblegum signer
    </span><span class="attr">#[error(<span class="string">"Invalid bubblegum signer"</span>)]
    </span>InvalidBubblegumSigner,
    <span class="doccomment">/// 133 (0x85) - Escrow parent cannot have a delegate
    </span><span class="attr">#[error(<span class="string">"Escrow parent cannot have a delegate"</span>)]
    </span>EscrowParentHasDelegate,
    <span class="doccomment">/// 134 (0x86) - Mint needs to be signer to initialize the account
    </span><span class="attr">#[error(<span class="string">"Mint needs to be signer to initialize the account"</span>)]
    </span>MintIsNotSigner,
    <span class="doccomment">/// 135 (0x87) - Invalid token standard
    </span><span class="attr">#[error(<span class="string">"Invalid token standard"</span>)]
    </span>InvalidTokenStandard,
    <span class="doccomment">/// 136 (0x88) - Invalid mint account for specified token standard
    </span><span class="attr">#[error(<span class="string">"Invalid mint account for specified token standard"</span>)]
    </span>InvalidMintForTokenStandard,
    <span class="doccomment">/// 137 (0x89) - Invalid authorization rules account
    </span><span class="attr">#[error(<span class="string">"Invalid authorization rules account"</span>)]
    </span>InvalidAuthorizationRules,
    <span class="doccomment">/// 138 (0x8A) - Missing authorization rules account
    </span><span class="attr">#[error(<span class="string">"Missing authorization rules account"</span>)]
    </span>MissingAuthorizationRules,
    <span class="doccomment">/// 139 (0x8B) - Missing programmable configuration
    </span><span class="attr">#[error(<span class="string">"Missing programmable configuration"</span>)]
    </span>MissingProgrammableConfig,
    <span class="doccomment">/// 140 (0x8C) - Invalid programmable configuration
    </span><span class="attr">#[error(<span class="string">"Invalid programmable configuration"</span>)]
    </span>InvalidProgrammableConfig,
    <span class="doccomment">/// 141 (0x8D) - Delegate already exists
    </span><span class="attr">#[error(<span class="string">"Delegate already exists"</span>)]
    </span>DelegateAlreadyExists,
    <span class="doccomment">/// 142 (0x8E) - Delegate not found
    </span><span class="attr">#[error(<span class="string">"Delegate not found"</span>)]
    </span>DelegateNotFound,
    <span class="doccomment">/// 143 (0x8F) - Required account not set in instruction builder
    </span><span class="attr">#[error(<span class="string">"Required account not set in instruction builder"</span>)]
    </span>MissingAccountInBuilder,
    <span class="doccomment">/// 144 (0x90) - Required argument not set in instruction builder
    </span><span class="attr">#[error(<span class="string">"Required argument not set in instruction builder"</span>)]
    </span>MissingArgumentInBuilder,
    <span class="doccomment">/// 145 (0x91) - Feature not supported currently
    </span><span class="attr">#[error(<span class="string">"Feature not supported currently"</span>)]
    </span>FeatureNotSupported,
    <span class="doccomment">/// 146 (0x92) - Invalid system wallet
    </span><span class="attr">#[error(<span class="string">"Invalid system wallet"</span>)]
    </span>InvalidSystemWallet,
    <span class="doccomment">/// 147 (0x93) - Only the sale delegate can transfer while its set
    </span><span class="attr">#[error(<span class="string">"Only the sale delegate can transfer while its set"</span>)]
    </span>OnlySaleDelegateCanTransfer,
    <span class="doccomment">/// 148 (0x94) - Missing token account
    </span><span class="attr">#[error(<span class="string">"Missing token account"</span>)]
    </span>MissingTokenAccount,
    <span class="doccomment">/// 149 (0x95) - Missing SPL token program
    </span><span class="attr">#[error(<span class="string">"Missing SPL token program"</span>)]
    </span>MissingSplTokenProgram,
    <span class="doccomment">/// 150 (0x96) - Missing authorization rules program
    </span><span class="attr">#[error(<span class="string">"Missing authorization rules program"</span>)]
    </span>MissingAuthorizationRulesProgram,
    <span class="doccomment">/// 151 (0x97) - Invalid delegate role for transfer
    </span><span class="attr">#[error(<span class="string">"Invalid delegate role for transfer"</span>)]
    </span>InvalidDelegateRoleForTransfer,
    <span class="doccomment">/// 152 (0x98) - Invalid transfer authority
    </span><span class="attr">#[error(<span class="string">"Invalid transfer authority"</span>)]
    </span>InvalidTransferAuthority,
    <span class="doccomment">/// 153 (0x99) - Instruction not supported for ProgrammableNonFungible assets
    </span><span class="attr">#[error(<span class="string">"Instruction not supported for ProgrammableNonFungible assets"</span>)]
    </span>InstructionNotSupported,
    <span class="doccomment">/// 154 (0x9A) - Public key does not match expected value
    </span><span class="attr">#[error(<span class="string">"Public key does not match expected value"</span>)]
    </span>KeyMismatch,
    <span class="doccomment">/// 155 (0x9B) - Token is locked
    </span><span class="attr">#[error(<span class="string">"Token is locked"</span>)]
    </span>LockedToken,
    <span class="doccomment">/// 156 (0x9C) - Token is unlocked
    </span><span class="attr">#[error(<span class="string">"Token is unlocked"</span>)]
    </span>UnlockedToken,
    <span class="doccomment">/// 157 (0x9D) - Missing delegate role
    </span><span class="attr">#[error(<span class="string">"Missing delegate role"</span>)]
    </span>MissingDelegateRole,
    <span class="doccomment">/// 158 (0x9E) - Invalid authority type
    </span><span class="attr">#[error(<span class="string">"Invalid authority type"</span>)]
    </span>InvalidAuthorityType,
    <span class="doccomment">/// 159 (0x9F) - Missing token record account
    </span><span class="attr">#[error(<span class="string">"Missing token record account"</span>)]
    </span>MissingTokenRecord,
    <span class="doccomment">/// 160 (0xA0) - Mint supply must be zero for programmable assets
    </span><span class="attr">#[error(<span class="string">"Mint supply must be zero for programmable assets"</span>)]
    </span>MintSupplyMustBeZero,
    <span class="doccomment">/// 161 (0xA1) - Data is empty or zeroed
    </span><span class="attr">#[error(<span class="string">"Data is empty or zeroed"</span>)]
    </span>DataIsEmptyOrZeroed,
    <span class="doccomment">/// 162 (0xA2) - Missing token owner
    </span><span class="attr">#[error(<span class="string">"Missing token owner"</span>)]
    </span>MissingTokenOwnerAccount,
    <span class="doccomment">/// 163 (0xA3) - Master edition account has an invalid length
    </span><span class="attr">#[error(<span class="string">"Master edition account has an invalid length"</span>)]
    </span>InvalidMasterEditionAccountLength,
    <span class="doccomment">/// 164 (0xA4) - Incorrect token state
    </span><span class="attr">#[error(<span class="string">"Incorrect token state"</span>)]
    </span>IncorrectTokenState,
    <span class="doccomment">/// 165 (0xA5) - Invalid delegate role
    </span><span class="attr">#[error(<span class="string">"Invalid delegate role"</span>)]
    </span>InvalidDelegateRole,
    <span class="doccomment">/// 166 (0xA6) - Print supply is required for non-fungibles
    </span><span class="attr">#[error(<span class="string">"Print supply is required for non-fungibles"</span>)]
    </span>MissingPrintSupply,
    <span class="doccomment">/// 167 (0xA7) - Missing master edition account
    </span><span class="attr">#[error(<span class="string">"Missing master edition account"</span>)]
    </span>MissingMasterEditionAccount,
    <span class="doccomment">/// 168 (0xA8) - Amount must be greater than zero
    </span><span class="attr">#[error(<span class="string">"Amount must be greater than zero"</span>)]
    </span>AmountMustBeGreaterThanZero,
    <span class="doccomment">/// 169 (0xA9) - Invalid delegate args
    </span><span class="attr">#[error(<span class="string">"Invalid delegate args"</span>)]
    </span>InvalidDelegateArgs,
    <span class="doccomment">/// 170 (0xAA) - Missing address for locked transfer
    </span><span class="attr">#[error(<span class="string">"Missing address for locked transfer"</span>)]
    </span>MissingLockedTransferAddress,
    <span class="doccomment">/// 171 (0xAB) - Invalid destination address for locked transfer
    </span><span class="attr">#[error(<span class="string">"Invalid destination address for locked transfer"</span>)]
    </span>InvalidLockedTransferAddress,
    <span class="doccomment">/// 172 (0xAC) - Exceeded account realloc increase limit
    </span><span class="attr">#[error(<span class="string">"Exceeded account realloc increase limit"</span>)]
    </span>DataIncrementLimitExceeded,
    <span class="doccomment">/// 173 (0xAD) - Cannot update the rule set of a programmable asset that has a delegate
    </span><span class="attr">#[error(<span class="string">"Cannot update the rule set of a programmable asset that has a delegate"</span>)]
    </span>CannotUpdateAssetWithDelegate,
    <span class="doccomment">/// 174 (0xAE) - Invalid token amount for this operation or token standard
    </span><span class="attr">#[error(<span class="string">"Invalid token amount for this operation or token standard"</span>)]
    </span>InvalidAmount,
    <span class="doccomment">/// 175 (0xAF) - Missing master edition mint account
    </span><span class="attr">#[error(<span class="string">"Missing master edition mint account"</span>)]
    </span>MissingMasterEditionMintAccount,
    <span class="doccomment">/// 176 (0xB0) - Missing master edition token account
    </span><span class="attr">#[error(<span class="string">"Missing master edition token account"</span>)]
    </span>MissingMasterEditionTokenAccount,
    <span class="doccomment">/// 177 (0xB1) - Missing edition marker account
    </span><span class="attr">#[error(<span class="string">"Missing edition marker account"</span>)]
    </span>MissingEditionMarkerAccount,
    <span class="doccomment">/// 178 (0xB2) - Cannot burn while persistent delegate is set
    </span><span class="attr">#[error(<span class="string">"Cannot burn while persistent delegate is set"</span>)]
    </span>CannotBurnWithDelegate,
    <span class="doccomment">/// 179 (0xB3) - Missing edition account
    </span><span class="attr">#[error(<span class="string">"Missing edition account"</span>)]
    </span>MissingEdition,
    <span class="doccomment">/// 180 (0xB4) - Invalid Associated Token Account Program
    </span><span class="attr">#[error(<span class="string">"Invalid Associated Token Account Program"</span>)]
    </span>InvalidAssociatedTokenAccountProgram,
    <span class="doccomment">/// 181 (0xB5) - Invalid InstructionsSysvar
    </span><span class="attr">#[error(<span class="string">"Invalid InstructionsSysvar"</span>)]
    </span>InvalidInstructionsSysvar,
    <span class="doccomment">/// 182 (0xB6) - Invalid or Unneeded parent accounts
    </span><span class="attr">#[error(<span class="string">"Invalid or Unneeded parent accounts"</span>)]
    </span>InvalidParentAccounts,
    <span class="doccomment">/// 183 (0xB7) - Authority cannot apply all update args
    </span><span class="attr">#[error(<span class="string">"Authority cannot apply all update args"</span>)]
    </span>InvalidUpdateArgs,
    <span class="doccomment">/// 184 (0xB8) - Token account does not have enough tokens
    </span><span class="attr">#[error(<span class="string">"Token account does not have enough tokens"</span>)]
    </span>InsufficientTokenBalance,
    <span class="doccomment">/// 185 (0xB9) - Missing collection account
    </span><span class="attr">#[error(<span class="string">"Missing collection account"</span>)]
    </span>MissingCollectionMint,
    <span class="doccomment">/// 186 (0xBA) - Missing collection master edition account
    </span><span class="attr">#[error(<span class="string">"Missing collection master edition account"</span>)]
    </span>MissingCollectionMasterEdition,
    <span class="doccomment">/// 187 (0xBB) - Invalid token record account
    </span><span class="attr">#[error(<span class="string">"Invalid token record account"</span>)]
    </span>InvalidTokenRecord,
    <span class="doccomment">/// 188 (0xBC) - The close authority needs to be revoked by the Utility Delegate
    </span><span class="attr">#[error(<span class="string">"The close authority needs to be revoked by the Utility Delegate"</span>)]
    </span>InvalidCloseAuthority,
    <span class="doccomment">/// 189 (0xBD) - Invalid or removed instruction
    </span><span class="attr">#[error(<span class="string">"Invalid or removed instruction"</span>)]
    </span>InvalidInstruction,
    <span class="doccomment">/// 190 (0xBE) - Missing delegate record
    </span><span class="attr">#[error(<span class="string">"Missing delegate record"</span>)]
    </span>MissingDelegateRecord,
    <span class="doccomment">/// 191 (0xBF) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>InvalidFeeAccount,
    <span class="doccomment">/// 192 (0xC0) -
    </span><span class="attr">#[error(<span class="string">""</span>)]
    </span>InvalidMetadataFlags,
    <span class="doccomment">/// 193 (0xC1) - Cannot change the update authority with a delegate
    </span><span class="attr">#[error(<span class="string">"Cannot change the update authority with a delegate"</span>)]
    </span>CannotChangeUpdateAuthorityWithDelegate,
    <span class="doccomment">/// 194 (0xC2) - Invalid mint extension type
    </span><span class="attr">#[error(<span class="string">"Invalid mint extension type"</span>)]
    </span>InvalidMintExtensionType,
    <span class="doccomment">/// 195 (0xC3) - Invalid mint close authority
    </span><span class="attr">#[error(<span class="string">"Invalid mint close authority"</span>)]
    </span>InvalidMintCloseAuthority,
    <span class="doccomment">/// 196 (0xC4) - Invalid metadata pointer
    </span><span class="attr">#[error(<span class="string">"Invalid metadata pointer"</span>)]
    </span>InvalidMetadataPointer,
    <span class="doccomment">/// 197 (0xC5) - Invalid token extension type
    </span><span class="attr">#[error(<span class="string">"Invalid token extension type"</span>)]
    </span>InvalidTokenExtensionType,
    <span class="doccomment">/// 198 (0xC6) - Missing immutable owner extension
    </span><span class="attr">#[error(<span class="string">"Missing immutable owner extension"</span>)]
    </span>MissingImmutableOwnerExtension,
}

<span class="kw">impl </span>solana_program::program_error::PrintProgramError <span class="kw">for </span>MplTokenMetadataError {
    <span class="kw">fn </span>print&lt;E&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>) {
        <span class="macro">solana_program::msg!</span>(<span class="kw-2">&amp;</span><span class="self">self</span>.to_string());
    }
}
</code></pre></div></section></main></body></html>