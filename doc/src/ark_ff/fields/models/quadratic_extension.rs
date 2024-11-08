<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ark-ff-0.4.2/src/fields/models/quadratic_extension.rs`."><title>quadratic_extension.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../../" data-static-root-path="../../../../static.files/" data-current-crate="ark_ff" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../../src-files.js"></script><script defer src="../../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="alternate icon" type="image/png" href="../../../../static.files/favicon-32x32-422f7d1d52889060.png"><link rel="icon" type="image/svg+xml" href="../../../../static.files/favicon-2c020d218678b618.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../../ark_ff/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="kw">use </span>ark_serialize::{
    CanonicalDeserialize, CanonicalDeserializeWithFlags, CanonicalSerialize,
    CanonicalSerializeWithFlags, Compress, EmptyFlags, Flags, SerializationError, Valid, Validate,
};
<span class="kw">use </span>ark_std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt,
    io::{Read, Write},
    iter::Chain,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    vec::Vec,
};

<span class="kw">use </span>num_traits::{One, Zero};
<span class="kw">use </span>zeroize::Zeroize;

<span class="kw">use </span>ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

<span class="kw">use crate</span>::{
    biginteger::BigInteger,
    fields::{Field, LegendreSymbol, PrimeField},
    SqrtPrecomputation, ToConstraintField, UniformRand,
};

<span class="doccomment">/// Defines a Quadratic extension field from a quadratic non-residue.
</span><span class="kw">pub trait </span>QuadExtConfig: <span class="lifetime">'static </span>+ Send + Sync + Sized {
    <span class="doccomment">/// The prime field that this quadratic extension is eventually an extension of.
    </span><span class="kw">type </span>BasePrimeField: PrimeField;
    <span class="doccomment">/// The base field that this field is a quadratic extension of.
    ///
    /// Note: while for simple instances of quadratic extensions such as `Fp2`
    /// we might see `BaseField == BasePrimeField`, it won't always hold true.
    /// E.g. for an extension tower: `BasePrimeField == Fp`, but `BaseField == Fp3`.
    </span><span class="kw">type </span>BaseField: Field&lt;BasePrimeField = <span class="self">Self</span>::BasePrimeField&gt;;
    <span class="doccomment">/// The type of the coefficients for an efficient implemntation of the
    /// Frobenius endomorphism.
    </span><span class="kw">type </span>FrobCoeff: Field;

    <span class="doccomment">/// The degree of the extension over the base prime field.
    </span><span class="kw">const </span>DEGREE_OVER_BASE_PRIME_FIELD: usize;

    <span class="doccomment">/// The quadratic non-residue used to construct the extension.
    </span><span class="kw">const </span>NONRESIDUE: <span class="self">Self</span>::BaseField;

    <span class="doccomment">/// Coefficients for the Frobenius automorphism.
    </span><span class="kw">const </span>FROBENIUS_COEFF_C1: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[<span class="self">Self</span>::FrobCoeff];

    <span class="doccomment">/// A specializable method for multiplying an element of the base field by
    /// the quadratic non-residue. This is used in Karatsuba multiplication
    /// and in complex squaring.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>mul_base_field_by_nonresidue_in_place(fe: <span class="kw-2">&amp;mut </span><span class="self">Self</span>::BaseField) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self</span>::BaseField {
        <span class="kw-2">*</span>fe <span class="kw-2">*</span>= <span class="kw-2">&amp;</span><span class="self">Self</span>::NONRESIDUE;
        fe
    }

    <span class="doccomment">/// A specializable method for setting `y = x + NONRESIDUE * y`.
    /// This allows for optimizations when the non-residue is
    /// canonically negative in the field.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>mul_base_field_by_nonresidue_and_add(y: <span class="kw-2">&amp;mut </span><span class="self">Self</span>::BaseField, x: <span class="kw-2">&amp;</span><span class="self">Self</span>::BaseField) {
        <span class="self">Self</span>::mul_base_field_by_nonresidue_in_place(y);
        <span class="kw-2">*</span>y += x;
    }

    <span class="doccomment">/// A specializable method for computing x + mul_base_field_by_nonresidue(y) + y
    /// This allows for optimizations when the non-residue is not -1.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>mul_base_field_by_nonresidue_plus_one_and_add(y: <span class="kw-2">&amp;mut </span><span class="self">Self</span>::BaseField, x: <span class="kw-2">&amp;</span><span class="self">Self</span>::BaseField) {
        <span class="kw">let </span>old_y = <span class="kw-2">*</span>y;
        <span class="self">Self</span>::mul_base_field_by_nonresidue_and_add(y, x);
        <span class="kw-2">*</span>y += old_y;
    }

    <span class="doccomment">/// A specializable method for computing x - mul_base_field_by_nonresidue(y)
    /// This allows for optimizations when the non-residue is
    /// canonically negative in the field.
    </span><span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>sub_and_mul_base_field_by_nonresidue(y: <span class="kw-2">&amp;mut </span><span class="self">Self</span>::BaseField, x: <span class="kw-2">&amp;</span><span class="self">Self</span>::BaseField) {
        <span class="self">Self</span>::mul_base_field_by_nonresidue_in_place(y);
        <span class="kw">let </span><span class="kw-2">mut </span>result = <span class="kw-2">*</span>x;
        result -= <span class="kw-2">&amp;*</span>y;
        <span class="kw-2">*</span>y = result;
    }

    <span class="doccomment">/// A specializable method for multiplying an element of the base field by
    /// the appropriate Frobenius coefficient.
    </span><span class="kw">fn </span>mul_base_field_by_frob_coeff(fe: <span class="kw-2">&amp;mut </span><span class="self">Self</span>::BaseField, power: usize);
}

<span class="doccomment">/// An element of a quadratic extension field F_p\[X\]/(X^2 - P::NONRESIDUE) is
/// represented as c0 + c1 * X, for c0, c1 in `P::BaseField`.
</span><span class="attr">#[derive(Derivative)]
#[derivative(
    Default(bound = <span class="string">"P: QuadExtConfig"</span>),
    Hash(bound = <span class="string">"P: QuadExtConfig"</span>),
    Clone(bound = <span class="string">"P: QuadExtConfig"</span>),
    Copy(bound = <span class="string">"P: QuadExtConfig"</span>),
    Debug(bound = <span class="string">"P: QuadExtConfig"</span>),
    PartialEq(bound = <span class="string">"P: QuadExtConfig"</span>),
    Eq(bound = <span class="string">"P: QuadExtConfig"</span>)
)]
</span><span class="kw">pub struct </span>QuadExtField&lt;P: QuadExtConfig&gt; {
    <span class="doccomment">/// Coefficient `c0` in the representation of the field element `c = c0 + c1 * X`
    </span><span class="kw">pub </span>c0: P::BaseField,
    <span class="doccomment">/// Coefficient `c1` in the representation of the field element `c = c0 + c1 * X`
    </span><span class="kw">pub </span>c1: P::BaseField,
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; QuadExtField&lt;P&gt; {
    <span class="doccomment">/// Create a new field element from coefficients `c0` and `c1`,
    /// so that the result is of the form `c0 + c1 * X`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ark_std::test_rng;
    /// # use ark_test_curves::bls12_381::{Fq as Fp, Fq2 as Fp2};
    /// # use ark_std::UniformRand;
    /// let c0: Fp = Fp::rand(&amp;mut test_rng());
    /// let c1: Fp = Fp::rand(&amp;mut test_rng());
    /// // `Fp2` a degree-2 extension over `Fp`.
    /// let c: Fp2 = Fp2::new(c0, c1);
    /// ```
    </span><span class="kw">pub const fn </span>new(c0: P::BaseField, c1: P::BaseField) -&gt; <span class="self">Self </span>{
        <span class="self">Self </span>{ c0, c1 }
    }

    <span class="doccomment">/// This is only to be used when the element is *known* to be in the
    /// cyclotomic subgroup.
    </span><span class="kw">pub fn </span>conjugate_in_place(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.c1 = -<span class="self">self</span>.c1;
        <span class="self">self
    </span>}

    <span class="doccomment">/// Norm of QuadExtField over `P::BaseField`:`Norm(a) = a * a.conjugate()`.
    /// This simplifies to: `Norm(a) = a.x^2 - P::NON_RESIDUE * a.y^2`.
    /// This is alternatively expressed as `Norm(a) = a^(1 + p)`.
    ///
    /// # Examples
    /// ```
    /// # use ark_std::test_rng;
    /// # use ark_std::{UniformRand, Zero};
    /// # use ark_test_curves::{Field, bls12_381::Fq2 as Fp2};
    /// let c: Fp2 = Fp2::rand(&amp;mut test_rng());
    /// let norm = c.norm();
    /// // We now compute the norm using the `a * a.conjugate()` approach.
    /// // A Frobenius map sends an element of `Fp2` to one of its p_th powers:
    /// // `a.frobenius_map_in_place(1) -&gt; a^p` and `a^p` is also `a`'s Galois conjugate.
    /// let mut c_conjugate = c;
    /// c_conjugate.frobenius_map_in_place(1);
    /// let norm2 = c * c_conjugate;
    /// // Computing the norm of an `Fp2` element should result in an element
    /// // in BaseField `Fp`, i.e. `c1 == 0`
    /// assert!(norm2.c1.is_zero());
    /// assert_eq!(norm, norm2.c0);
    /// ```
    </span><span class="kw">pub fn </span>norm(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; P::BaseField {
        <span class="comment">// t1 = c0.square() - P::NON_RESIDUE * c1^2
        </span><span class="kw">let </span><span class="kw-2">mut </span>result = <span class="self">self</span>.c1.square();
        P::sub_and_mul_base_field_by_nonresidue(<span class="kw-2">&amp;mut </span>result, <span class="kw-2">&amp;</span><span class="self">self</span>.c0.square());
        result
    }

    <span class="doccomment">/// In-place multiply both coefficients `c0` &amp; `c1` of the quadratic
    /// extension field by an element from the base field.
    </span><span class="kw">pub fn </span>mul_assign_by_basefield(<span class="kw-2">&amp;mut </span><span class="self">self</span>, element: <span class="kw-2">&amp;</span>P::BaseField) {
        <span class="self">self</span>.c0 <span class="kw-2">*</span>= element;
        <span class="self">self</span>.c1 <span class="kw-2">*</span>= element;
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; Zero <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">fn </span>zero() -&gt; <span class="self">Self </span>{
        QuadExtField::new(P::BaseField::zero(), P::BaseField::zero())
    }

    <span class="kw">fn </span>is_zero(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.c0.is_zero() &amp;&amp; <span class="self">self</span>.c1.is_zero()
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; One <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">fn </span>one() -&gt; <span class="self">Self </span>{
        QuadExtField::new(P::BaseField::one(), P::BaseField::zero())
    }

    <span class="kw">fn </span>is_one(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; bool {
        <span class="self">self</span>.c0.is_one() &amp;&amp; <span class="self">self</span>.c1.is_zero()
    }
}

<span class="kw">type </span>BaseFieldIter&lt;P&gt; = &lt;&lt;P <span class="kw">as </span>QuadExtConfig&gt;::BaseField <span class="kw">as </span>Field&gt;::BasePrimeFieldIter;
<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; Field <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">type </span>BasePrimeField = P::BasePrimeField;

    <span class="kw">type </span>BasePrimeFieldIter = Chain&lt;BaseFieldIter&lt;P&gt;, BaseFieldIter&lt;P&gt;&gt;;

    <span class="kw">const </span>SQRT_PRECOMP: <span class="prelude-ty">Option</span>&lt;SqrtPrecomputation&lt;<span class="self">Self</span>&gt;&gt; = <span class="prelude-val">None</span>;

    <span class="kw">const </span>ZERO: <span class="self">Self </span>= <span class="self">Self</span>::new(P::BaseField::ZERO, P::BaseField::ZERO);
    <span class="kw">const </span>ONE: <span class="self">Self </span>= <span class="self">Self</span>::new(P::BaseField::ONE, P::BaseField::ZERO);

    <span class="kw">fn </span>extension_degree() -&gt; u64 {
        <span class="number">2 </span>* P::BaseField::extension_degree()
    }

    <span class="kw">fn </span>from_base_prime_field(elem: <span class="self">Self</span>::BasePrimeField) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>fe = P::BaseField::from_base_prime_field(elem);
        <span class="self">Self</span>::new(fe, P::BaseField::ZERO)
    }

    <span class="kw">fn </span>to_base_prime_field_elements(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self</span>::BasePrimeFieldIter {
        <span class="self">self</span>.c0
            .to_base_prime_field_elements()
            .chain(<span class="self">self</span>.c1.to_base_prime_field_elements())
    }

    <span class="kw">fn </span>from_base_prime_field_elems(elems: <span class="kw-2">&amp;</span>[<span class="self">Self</span>::BasePrimeField]) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
        <span class="kw">if </span>elems.len() != (<span class="self">Self</span>::extension_degree() <span class="kw">as </span>usize) {
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }
        <span class="kw">let </span>base_ext_deg = P::BaseField::extension_degree() <span class="kw">as </span>usize;
        <span class="prelude-val">Some</span>(<span class="self">Self</span>::new(
            P::BaseField::from_base_prime_field_elems(<span class="kw-2">&amp;</span>elems[<span class="number">0</span>..base_ext_deg]).unwrap(),
            P::BaseField::from_base_prime_field_elems(<span class="kw-2">&amp;</span>elems[base_ext_deg..]).unwrap(),
        ))
    }

    <span class="kw">fn </span>double(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>result = <span class="kw-2">*</span><span class="self">self</span>;
        result.double_in_place();
        result
    }

    <span class="kw">fn </span>double_in_place(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.c0.double_in_place();
        <span class="self">self</span>.c1.double_in_place();
        <span class="self">self
    </span>}

    <span class="kw">fn </span>neg_in_place(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="self">self</span>.c0.neg_in_place();
        <span class="self">self</span>.c1.neg_in_place();
        <span class="self">self
    </span>}

    <span class="kw">fn </span>square(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span><span class="kw-2">mut </span>result = <span class="kw-2">*</span><span class="self">self</span>;
        result.square_in_place();
        result
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from_random_bytes_with_flags&lt;F: Flags&gt;(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Option</span>&lt;(<span class="self">Self</span>, F)&gt; {
        <span class="kw">let </span>split_at = bytes.len() / <span class="number">2</span>;
        <span class="kw">if let </span><span class="prelude-val">Some</span>(c0) = P::BaseField::from_random_bytes(<span class="kw-2">&amp;</span>bytes[..split_at]) {
            <span class="kw">if let </span><span class="prelude-val">Some</span>((c1, flags)) =
                P::BaseField::from_random_bytes_with_flags(<span class="kw-2">&amp;</span>bytes[split_at..])
            {
                <span class="kw">return </span><span class="prelude-val">Some</span>((QuadExtField::new(c0, c1), flags));
            }
        }
        <span class="prelude-val">None
    </span>}

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from_random_bytes(bytes: <span class="kw-2">&amp;</span>[u8]) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
        <span class="self">Self</span>::from_random_bytes_with_flags::&lt;EmptyFlags&gt;(bytes).map(|f| f.<span class="number">0</span>)
    }

    <span class="kw">fn </span>square_in_place(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;mut </span><span class="self">Self </span>{
        <span class="comment">// (c0, c1)^2 = (c0 + x*c1)^2
        //            = c0^2 + 2 c0 c1 x + c1^2 x^2
        //            = c0^2 + beta * c1^2 + 2 c0 * c1 * x
        //            = (c0^2 + beta * c1^2, 2 c0 * c1)
        // Where beta is P::NONRESIDUE.
        // When beta = -1, we can re-use intermediate additions to improve performance.
        </span><span class="kw">if </span>P::NONRESIDUE == -P::BaseField::ONE {
            <span class="comment">// When the non-residue is -1, we save 2 intermediate additions,
            // and use one fewer intermediate variable

            </span><span class="kw">let </span>c0_copy = <span class="self">self</span>.c0;
            <span class="comment">// v0 = c0 - c1
            </span><span class="kw">let </span><span class="kw-2">mut </span>v0 = <span class="self">self</span>.c0;
            v0 -= <span class="kw-2">&amp;</span><span class="self">self</span>.c1;
            <span class="self">self</span>.c0 += <span class="self">self</span>.c1;
            <span class="comment">// result.c0 *= (c0 - c1)
            // result.c0 = (c0 - c1) * (c0 + c1) = c0^2 - c1^2
            </span><span class="self">self</span>.c0 <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>v0;

            <span class="comment">// result.c1 = 2 c1
            </span><span class="self">self</span>.c1.double_in_place();
            <span class="comment">// result.c1 *= c0
            // result.c1 = (2 * c1) * c0
            </span><span class="self">self</span>.c1 <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>c0_copy;

            <span class="self">self
        </span>} <span class="kw">else </span>{
            <span class="comment">// v0 = c0 - c1
            </span><span class="kw">let </span><span class="kw-2">mut </span>v0 = <span class="self">self</span>.c0 - <span class="kw-2">&amp;</span><span class="self">self</span>.c1;
            <span class="comment">// v3 = c0 - beta * c1
            </span><span class="kw">let </span><span class="kw-2">mut </span>v3 = <span class="self">self</span>.c1;
            P::sub_and_mul_base_field_by_nonresidue(<span class="kw-2">&amp;mut </span>v3, <span class="kw-2">&amp;</span><span class="self">self</span>.c0);
            <span class="comment">// v2 = c0 * c1
            </span><span class="kw">let </span>v2 = <span class="self">self</span>.c0 * <span class="kw-2">&amp;</span><span class="self">self</span>.c1;

            <span class="comment">// v0 = (v0 * v3)
            // v0 = (c0 - c1) * (c0 - beta*c1)
            // v0 = c0^2 - beta * c0 * c1 - c0 * c1 + beta * c1^2
            </span>v0 <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>v3;

            <span class="comment">// result.c1 = 2 * c0 * c1
            </span><span class="self">self</span>.c1 = v2;
            <span class="self">self</span>.c1.double_in_place();
            <span class="comment">// result.c0 = (c0^2 - beta * c0 * c1 - c0 * c1 + beta * c1^2) + ((beta + 1) c0 * c1)
            // result.c0 = (c0^2 - beta * c0 * c1 + beta * c1^2) + (beta * c0 * c1)
            // result.c0 = c0^2 + beta * c1^2
            </span><span class="self">self</span>.c0 = v2;
            P::mul_base_field_by_nonresidue_plus_one_and_add(<span class="kw-2">&amp;mut </span><span class="self">self</span>.c0, <span class="kw-2">&amp;</span>v0);

            <span class="self">self
        </span>}
    }

    <span class="kw">fn </span>inverse(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
        <span class="kw">if </span><span class="self">self</span>.is_zero() {
            <span class="prelude-val">None
        </span>} <span class="kw">else </span>{
            <span class="comment">// Guide to Pairing-based Cryptography, Algorithm 5.19.
            // v1 = c1.square()
            </span><span class="kw">let </span>v1 = <span class="self">self</span>.c1.square();
            <span class="comment">// v0 = c0.square() - beta * v1
            </span><span class="kw">let </span><span class="kw-2">mut </span>v0 = v1;
            P::sub_and_mul_base_field_by_nonresidue(<span class="kw-2">&amp;mut </span>v0, <span class="kw-2">&amp;</span><span class="self">self</span>.c0.square());

            v0.inverse().map(|v1| {
                <span class="kw">let </span>c0 = <span class="self">self</span>.c0 * <span class="kw-2">&amp;</span>v1;
                <span class="kw">let </span>c1 = -(<span class="self">self</span>.c1 * <span class="kw-2">&amp;</span>v1);
                <span class="self">Self</span>::new(c0, c1)
            })
        }
    }

    <span class="kw">fn </span>inverse_in_place(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;mut </span><span class="self">Self</span>&gt; {
        <span class="kw">if let </span><span class="prelude-val">Some</span>(inverse) = <span class="self">self</span>.inverse() {
            <span class="kw-2">*</span><span class="self">self </span>= inverse;
            <span class="prelude-val">Some</span>(<span class="self">self</span>)
        } <span class="kw">else </span>{
            <span class="prelude-val">None
        </span>}
    }

    <span class="kw">fn </span>frobenius_map_in_place(<span class="kw-2">&amp;mut </span><span class="self">self</span>, power: usize) {
        <span class="self">self</span>.c0.frobenius_map_in_place(power);
        <span class="self">self</span>.c1.frobenius_map_in_place(power);
        P::mul_base_field_by_frob_coeff(<span class="kw-2">&amp;mut </span><span class="self">self</span>.c1, power);
    }

    <span class="kw">fn </span>legendre(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; LegendreSymbol {
        <span class="comment">// The LegendreSymbol in a field of order q for an element x can be
        // computed as x^((q-1)/2).
        // Since we are in a quadratic extension of a field F_p,
        // we have that q = p^2.
        // Notice then that (q-1)/2 = ((p-1)/2) * (1 + p).
        // This implies that we can compute the symbol as (x^(1+p))^((p-1)/2).
        // Recall that computing x^(1 + p) is equivalent to taking the norm of x,
        // and it will output an element in the base field F_p.
        // Then exponentiating by (p-1)/2 in the base field is equivalent to computing
        // the legendre symbol in the base field.
        </span><span class="self">self</span>.norm().legendre()
    }

    <span class="kw">fn </span>sqrt(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="self">Self</span>&gt; {
        <span class="comment">// Square root based on the complex method. See
        // https://eprint.iacr.org/2012/685.pdf (page 15, algorithm 8)
        </span><span class="kw">if </span><span class="self">self</span>.c1.is_zero() {
            <span class="comment">// for c = c0 + c1 * x, we have c1 = 0
            // sqrt(c) == sqrt(c0) is an element of Fp2, i.e. sqrt(c0) = a = a0 + a1 * x for some a0, a1 in Fp
            // squaring both sides: c0 = a0^2 + a1^2 * x^2 + (2 * a0 * a1 * x) = a0^2 + (a1^2 * P::NONRESIDUE)
            // since there are no `x` terms on LHS, a0 * a1 = 0
            // so either a0 = sqrt(c0) or a1 = sqrt(c0/P::NONRESIDUE)
            </span><span class="kw">if </span><span class="self">self</span>.c0.legendre().is_qr() {
                <span class="comment">// either c0 is a valid sqrt in the base field
                </span><span class="kw">return </span><span class="self">self</span>.c0.sqrt().map(|c0| <span class="self">Self</span>::new(c0, P::BaseField::ZERO));
            } <span class="kw">else </span>{
                <span class="comment">// or we need to compute sqrt(c0/P::NONRESIDUE)
                </span><span class="kw">return </span>(<span class="self">self</span>.c0.div(P::NONRESIDUE))
                    .sqrt()
                    .map(|res| <span class="self">Self</span>::new(P::BaseField::ZERO, res));
            }
        }
        <span class="comment">// Try computing the square root
        // Check at the end of the algorithm if it was a square root
        </span><span class="kw">let </span>alpha = <span class="self">self</span>.norm();

        <span class="comment">// Compute `(p+1)/2` as `1/2`.
        // This is cheaper than `P::BaseField::one().double().inverse()`
        </span><span class="kw">let </span><span class="kw-2">mut </span>two_inv = P::BasePrimeField::MODULUS;

        two_inv.add_with_carry(<span class="kw-2">&amp;</span><span class="number">1u64</span>.into());
        two_inv.div2();

        <span class="kw">let </span>two_inv = P::BasePrimeField::from(two_inv);
        <span class="kw">let </span>two_inv = P::BaseField::from_base_prime_field(two_inv);

        alpha.sqrt().and_then(|alpha| {
            <span class="kw">let </span><span class="kw-2">mut </span>delta = (alpha + <span class="kw-2">&amp;</span><span class="self">self</span>.c0) * <span class="kw-2">&amp;</span>two_inv;
            <span class="kw">if </span>delta.legendre().is_qnr() {
                delta -= <span class="kw-2">&amp;</span>alpha;
            }
            <span class="kw">let </span>c0 = delta.sqrt().expect(<span class="string">"Delta must have a square root"</span>);
            <span class="kw">let </span>c0_inv = c0.inverse().expect(<span class="string">"c0 must have an inverse"</span>);
            <span class="kw">let </span>sqrt_cand = <span class="self">Self</span>::new(c0, <span class="self">self</span>.c1 * <span class="kw-2">&amp;</span>two_inv * <span class="kw-2">&amp;</span>c0_inv);
            <span class="comment">// Check if sqrt_cand is actually the square root
            // if not, there exists no square root.
            </span><span class="kw">if </span>sqrt_cand.square() == <span class="kw-2">*</span><span class="self">self </span>{
                <span class="prelude-val">Some</span>(sqrt_cand)
            } <span class="kw">else </span>{
                <span class="attr">#[cfg(debug_assertions)]
                </span>{
                    <span class="kw">use </span><span class="kw">crate</span>::fields::LegendreSymbol::<span class="kw-2">*</span>;
                    <span class="kw">if </span><span class="self">self</span>.legendre() != QuadraticNonResidue {
                        <span class="macro">panic!</span>(
                            <span class="string">"Input has a square root per its legendre symbol, but it was not found"
                        </span>)
                    }
                }
                <span class="prelude-val">None
            </span>}
        })
    }

    <span class="kw">fn </span>sqrt_in_place(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;mut </span><span class="self">Self</span>&gt; {
        (<span class="kw-2">*</span><span class="self">self</span>).sqrt().map(|sqrt| {
            <span class="kw-2">*</span><span class="self">self </span>= sqrt;
            <span class="self">self
        </span>})
    }
}

<span class="doccomment">/// `QuadExtField` elements are ordered lexicographically.
</span><span class="kw">impl</span>&lt;P: QuadExtConfig&gt; Ord <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>cmp(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; Ordering {
        <span class="kw">match </span><span class="self">self</span>.c1.cmp(<span class="kw-2">&amp;</span>other.c1) {
            Ordering::Greater =&gt; Ordering::Greater,
            Ordering::Less =&gt; Ordering::Less,
            Ordering::Equal =&gt; <span class="self">self</span>.c0.cmp(<span class="kw-2">&amp;</span>other.c0),
        }
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; PartialOrd <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>partial_cmp(<span class="kw-2">&amp;</span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Ordering&gt; {
        <span class="prelude-val">Some</span>(<span class="self">self</span>.cmp(other))
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; Zeroize <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="comment">// The phantom data does not contain element-specific data
    // and thus does not need to be zeroized.
    </span><span class="kw">fn </span>zeroize(<span class="kw-2">&amp;mut </span><span class="self">self</span>) {
        <span class="self">self</span>.c0.zeroize();
        <span class="self">self</span>.c1.zeroize();
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;u128&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">fn </span>from(other: u128) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::new(other.into(), P::BaseField::ZERO)
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;i128&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from(val: i128) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>abs = <span class="self">Self</span>::from(val.unsigned_abs());
        <span class="kw">if </span>val.is_positive() {
            abs
        } <span class="kw">else </span>{
            -abs
        }
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;u64&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">fn </span>from(other: u64) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::new(other.into(), P::BaseField::ZERO)
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;i64&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from(val: i64) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>abs = <span class="self">Self</span>::from(val.unsigned_abs());
        <span class="kw">if </span>val.is_positive() {
            abs
        } <span class="kw">else </span>{
            -abs
        }
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;u32&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">fn </span>from(other: u32) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::new(other.into(), P::BaseField::ZERO)
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;i32&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from(val: i32) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>abs = <span class="self">Self</span>::from(val.unsigned_abs());
        <span class="kw">if </span>val.is_positive() {
            abs
        } <span class="kw">else </span>{
            -abs
        }
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;u16&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">fn </span>from(other: u16) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::new(other.into(), P::BaseField::ZERO)
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;i16&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from(val: i16) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>abs = <span class="self">Self</span>::from(val.unsigned_abs());
        <span class="kw">if </span>val.is_positive() {
            abs
        } <span class="kw">else </span>{
            -abs
        }
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;u8&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">fn </span>from(other: u8) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::new(other.into(), P::BaseField::ZERO)
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;i8&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>from(val: i8) -&gt; <span class="self">Self </span>{
        <span class="kw">let </span>abs = <span class="self">Self</span>::from(val.unsigned_abs());
        <span class="kw">if </span>val.is_positive() {
            abs
        } <span class="kw">else </span>{
            -abs
        }
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; From&lt;bool&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">fn </span>from(other: bool) -&gt; <span class="self">Self </span>{
        <span class="self">Self</span>::new(u8::from(other).into(), P::BaseField::ZERO)
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; Neg <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">type </span>Output = <span class="self">Self</span>;
    <span class="attr">#[inline]
    #[must_use]
    </span><span class="kw">fn </span>neg(<span class="kw-2">mut </span><span class="self">self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self</span>.c0.neg_in_place();
        <span class="self">self</span>.c1.neg_in_place();
        <span class="self">self
    </span>}
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; Distribution&lt;QuadExtField&lt;P&gt;&gt; <span class="kw">for </span>Standard {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>sample&lt;R: Rng + <span class="question-mark">?</span>Sized&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>, rng: <span class="kw-2">&amp;mut </span>R) -&gt; QuadExtField&lt;P&gt; {
        QuadExtField::new(UniformRand::rand(rng), UniformRand::rand(rng))
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, P: QuadExtConfig&gt; Add&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>QuadExtField&lt;P&gt;&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">type </span>Output = <span class="self">Self</span>;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>add(<span class="kw-2">mut </span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self </span>+= other;
        <span class="self">self
    </span>}
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, P: QuadExtConfig&gt; Sub&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>QuadExtField&lt;P&gt;&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">type </span>Output = <span class="self">Self</span>;

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>sub(<span class="kw-2">mut </span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self </span>-= other;
        <span class="self">self
    </span>}
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, P: QuadExtConfig&gt; Mul&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>QuadExtField&lt;P&gt;&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">type </span>Output = <span class="self">Self</span>;

    <span class="attr">#[inline(always)]
    </span><span class="kw">fn </span>mul(<span class="kw-2">mut </span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self </span><span class="kw-2">*</span>= other;
        <span class="self">self
    </span>}
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, P: QuadExtConfig&gt; Div&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>QuadExtField&lt;P&gt;&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">type </span>Output = <span class="self">Self</span>;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>div(<span class="kw-2">mut </span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) -&gt; <span class="self">Self </span>{
        <span class="self">self</span>.mul_assign(<span class="kw-2">&amp;</span>other.inverse().unwrap());
        <span class="self">self
    </span>}
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, P: QuadExtConfig&gt; AddAssign&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="self">Self</span>&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>add_assign(<span class="kw-2">&amp;mut </span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) {
        <span class="self">self</span>.c0 += <span class="kw-2">&amp;</span>other.c0;
        <span class="self">self</span>.c1 += <span class="kw-2">&amp;</span>other.c1;
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, P: QuadExtConfig&gt; SubAssign&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="self">Self</span>&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>sub_assign(<span class="kw-2">&amp;mut </span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) {
        <span class="self">self</span>.c0 -= <span class="kw-2">&amp;</span>other.c0;
        <span class="self">self</span>.c1 -= <span class="kw-2">&amp;</span>other.c1;
    }
}

<span class="macro">impl_additive_ops_from_ref!</span>(QuadExtField, QuadExtConfig);
<span class="macro">impl_multiplicative_ops_from_ref!</span>(QuadExtField, QuadExtConfig);

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, P: QuadExtConfig&gt; MulAssign&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="self">Self</span>&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>mul_assign(<span class="kw-2">&amp;mut </span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) {
        <span class="kw">if </span><span class="self">Self</span>::extension_degree() == <span class="number">2 </span>{
            <span class="kw">let </span>c1_input = [<span class="self">self</span>.c0, <span class="self">self</span>.c1];
            P::mul_base_field_by_nonresidue_in_place(<span class="kw-2">&amp;mut </span><span class="self">self</span>.c1);
            <span class="kw-2">*</span><span class="self">self </span>= <span class="self">Self</span>::new(
                P::BaseField::sum_of_products(<span class="kw-2">&amp;</span>[<span class="self">self</span>.c0, <span class="self">self</span>.c1], <span class="kw-2">&amp;</span>[other.c0, other.c1]),
                P::BaseField::sum_of_products(<span class="kw-2">&amp;</span>c1_input, <span class="kw-2">&amp;</span>[other.c1, other.c0]),
            )
        } <span class="kw">else </span>{
            <span class="comment">// Karatsuba multiplication;
            // Guide to Pairing-based cryprography, Algorithm 5.16.
            </span><span class="kw">let </span><span class="kw-2">mut </span>v0 = <span class="self">self</span>.c0;
            v0 <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>other.c0;
            <span class="kw">let </span><span class="kw-2">mut </span>v1 = <span class="self">self</span>.c1;
            v1 <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>other.c1;

            <span class="self">self</span>.c1 += <span class="kw-2">&amp;</span><span class="self">self</span>.c0;
            <span class="self">self</span>.c1 <span class="kw-2">*</span>= <span class="kw-2">&amp;</span>(other.c0 + <span class="kw-2">&amp;</span>other.c1);
            <span class="self">self</span>.c1 -= <span class="kw-2">&amp;</span>v0;
            <span class="self">self</span>.c1 -= <span class="kw-2">&amp;</span>v1;
            <span class="self">self</span>.c0 = v1;
            P::mul_base_field_by_nonresidue_and_add(<span class="kw-2">&amp;mut </span><span class="self">self</span>.c0, <span class="kw-2">&amp;</span>v0);
        }
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>, P: QuadExtConfig&gt; DivAssign&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span><span class="self">Self</span>&gt; <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>div_assign(<span class="kw-2">&amp;mut </span><span class="self">self</span>, other: <span class="kw-2">&amp;</span><span class="self">Self</span>) {
        <span class="self">self</span>.mul_assign(<span class="kw-2">&amp;</span>other.inverse().unwrap());
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; fmt::Display <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">fn </span>fmt(<span class="kw-2">&amp;</span><span class="self">self</span>, f: <span class="kw-2">&amp;mut </span>fmt::Formatter&lt;<span class="lifetime">'_</span>&gt;) -&gt; fmt::Result {
        <span class="macro">write!</span>(f, <span class="string">"QuadExtField({} + {} * u)"</span>, <span class="self">self</span>.c0, <span class="self">self</span>.c1)
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; CanonicalSerializeWithFlags <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>serialize_with_flags&lt;W: Write, F: Flags&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        <span class="kw-2">mut </span>writer: W,
        flags: F,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), SerializationError&gt; {
        <span class="self">self</span>.c0.serialize_compressed(<span class="kw-2">&amp;mut </span>writer)<span class="question-mark">?</span>;
        <span class="self">self</span>.c1.serialize_with_flags(<span class="kw-2">&amp;mut </span>writer, flags)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(())
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>serialized_size_with_flags&lt;F: Flags&gt;(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.c0.compressed_size() + <span class="self">self</span>.c1.serialized_size_with_flags::&lt;F&gt;()
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; CanonicalSerialize <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>serialize_with_mode&lt;W: Write&gt;(
        <span class="kw-2">&amp;</span><span class="self">self</span>,
        writer: W,
        _compress: Compress,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(), SerializationError&gt; {
        <span class="self">self</span>.serialize_with_flags(writer, EmptyFlags)
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>serialized_size(<span class="kw-2">&amp;</span><span class="self">self</span>, _compress: Compress) -&gt; usize {
        <span class="self">self</span>.serialized_size_with_flags::&lt;EmptyFlags&gt;()
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; CanonicalDeserializeWithFlags <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>deserialize_with_flags&lt;R: Read, F: Flags&gt;(
        <span class="kw-2">mut </span>reader: R,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;(<span class="self">Self</span>, F), SerializationError&gt; {
        <span class="kw">let </span>c0 = CanonicalDeserialize::deserialize_compressed(<span class="kw-2">&amp;mut </span>reader)<span class="question-mark">?</span>;
        <span class="kw">let </span>(c1, flags) = CanonicalDeserializeWithFlags::deserialize_with_flags(<span class="kw-2">&amp;mut </span>reader)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>((QuadExtField::new(c0, c1), flags))
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; Valid <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="kw">fn </span>check(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;(), SerializationError&gt; {
        <span class="self">self</span>.c0.check()<span class="question-mark">?</span>;
        <span class="self">self</span>.c1.check()
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; CanonicalDeserialize <span class="kw">for </span>QuadExtField&lt;P&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>deserialize_with_mode&lt;R: Read&gt;(
        <span class="kw-2">mut </span>reader: R,
        compress: Compress,
        validate: Validate,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="self">Self</span>, SerializationError&gt; {
        <span class="kw">let </span>c0: P::BaseField =
            CanonicalDeserialize::deserialize_with_mode(<span class="kw-2">&amp;mut </span>reader, compress, validate)<span class="question-mark">?</span>;
        <span class="kw">let </span>c1: P::BaseField =
            CanonicalDeserialize::deserialize_with_mode(<span class="kw-2">&amp;mut </span>reader, compress, validate)<span class="question-mark">?</span>;
        <span class="prelude-val">Ok</span>(QuadExtField::new(c0, c1))
    }
}

<span class="kw">impl</span>&lt;P: QuadExtConfig&gt; ToConstraintField&lt;P::BasePrimeField&gt; <span class="kw">for </span>QuadExtField&lt;P&gt;
<span class="kw">where
    </span>P::BaseField: ToConstraintField&lt;P::BasePrimeField&gt;,
{
    <span class="kw">fn </span>to_field_elements(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;Vec&lt;P::BasePrimeField&gt;&gt; {
        <span class="kw">let </span><span class="kw-2">mut </span>res = Vec::new();
        <span class="kw">let </span><span class="kw-2">mut </span>c0_elems = <span class="self">self</span>.c0.to_field_elements()<span class="question-mark">?</span>;
        <span class="kw">let </span><span class="kw-2">mut </span>c1_elems = <span class="self">self</span>.c1.to_field_elements()<span class="question-mark">?</span>;

        res.append(<span class="kw-2">&amp;mut </span>c0_elems);
        res.append(<span class="kw-2">&amp;mut </span>c1_elems);

        <span class="prelude-val">Some</span>(res)
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>quad_ext_tests {
    <span class="kw">use super</span>::<span class="kw-2">*</span>;
    <span class="kw">use </span>ark_std::test_rng;
    <span class="kw">use </span>ark_test_curves::{
        bls12_381::{Fq, Fq2},
        Field,
    };

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_from_base_prime_field_elements() {
        <span class="kw">let </span>ext_degree = Fq2::extension_degree() <span class="kw">as </span>usize;
        <span class="comment">// Test on slice lengths that aren't equal to the extension degree
        </span><span class="kw">let </span>max_num_elems_to_test = <span class="number">4</span>;
        <span class="kw">for </span>d <span class="kw">in </span><span class="number">0</span>..max_num_elems_to_test {
            <span class="kw">if </span>d == ext_degree {
                <span class="kw">continue</span>;
            }
            <span class="kw">let </span><span class="kw-2">mut </span>random_coeffs = Vec::&lt;Fq&gt;::new();
            <span class="kw">for _ in </span><span class="number">0</span>..d {
                random_coeffs.push(Fq::rand(<span class="kw-2">&amp;mut </span>test_rng()));
            }
            <span class="kw">let </span>res = Fq2::from_base_prime_field_elems(<span class="kw-2">&amp;</span>random_coeffs);
            <span class="macro">assert_eq!</span>(res, <span class="prelude-val">None</span>);
        }
        <span class="comment">// Test on slice lengths that are equal to the extension degree
        // We test consistency against Fq2::new
        </span><span class="kw">let </span>number_of_tests = <span class="number">10</span>;
        <span class="kw">for _ in </span><span class="number">0</span>..number_of_tests {
            <span class="kw">let </span><span class="kw-2">mut </span>random_coeffs = Vec::&lt;Fq&gt;::new();
            <span class="kw">for _ in </span><span class="number">0</span>..ext_degree {
                random_coeffs.push(Fq::rand(<span class="kw-2">&amp;mut </span>test_rng()));
            }
            <span class="kw">let </span>actual = Fq2::from_base_prime_field_elems(<span class="kw-2">&amp;</span>random_coeffs).unwrap();
            <span class="kw">let </span>expected = Fq2::new(random_coeffs[<span class="number">0</span>], random_coeffs[<span class="number">1</span>]);
            <span class="macro">assert_eq!</span>(actual, expected);
        }
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>test_from_base_prime_field_element() {
        <span class="kw">let </span>ext_degree = Fq2::extension_degree() <span class="kw">as </span>usize;
        <span class="kw">let </span>max_num_elems_to_test = <span class="number">10</span>;
        <span class="kw">for _ in </span><span class="number">0</span>..max_num_elems_to_test {
            <span class="kw">let </span><span class="kw-2">mut </span>random_coeffs = <span class="macro">vec!</span>[Fq::zero(); ext_degree];
            <span class="kw">let </span>random_coeff = Fq::rand(<span class="kw-2">&amp;mut </span>test_rng());
            <span class="kw">let </span>res = Fq2::from_base_prime_field(random_coeff);
            random_coeffs[<span class="number">0</span>] = random_coeff;
            <span class="macro">assert_eq!</span>(
                res,
                Fq2::from_base_prime_field_elems(<span class="kw-2">&amp;</span>random_coeffs).unwrap()
            );
        }
    }
}
</code></pre></div></section></main></body></html>