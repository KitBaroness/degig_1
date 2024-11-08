<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/unicode-segmentation-1.11.0/src/grapheme.rs`."><title>grapheme.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="unicode_segmentation" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://unicode-rs.github.io/unicode-rs_sm.png"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../unicode_segmentation/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 &lt;LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0&gt; or the MIT license
// &lt;LICENSE-MIT or http://opensource.org/licenses/MIT&gt;, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

</span><span class="kw">use </span>core::cmp;

<span class="kw">use </span><span class="kw">crate</span>::tables::grapheme::GraphemeCat;

<span class="doccomment">/// External iterator for grapheme clusters and byte offsets.
///
/// This struct is created by the [`grapheme_indices`] method on the [`UnicodeSegmentation`]
/// trait. See its documentation for more.
///
/// [`grapheme_indices`]: trait.UnicodeSegmentation.html#tymethod.grapheme_indices
/// [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html
</span><span class="attr">#[derive(Clone)]
</span><span class="kw">pub struct </span>GraphemeIndices&lt;<span class="lifetime">'a</span>&gt; {
    start_offset: usize,
    iter: Graphemes&lt;<span class="lifetime">'a</span>&gt;,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; GraphemeIndices&lt;<span class="lifetime">'a</span>&gt; {
    <span class="attr">#[inline]
    </span><span class="doccomment">/// View the underlying data (the part yet to be iterated) as a slice of the original string.
    ///
    /// ```rust
    /// # use unicode_segmentation::UnicodeSegmentation;
    /// let mut iter = "abc".grapheme_indices(true);
    /// assert_eq!(iter.as_str(), "abc");
    /// iter.next();
    /// assert_eq!(iter.as_str(), "bc");
    /// iter.next();
    /// iter.next();
    /// assert_eq!(iter.as_str(), "");
    /// ```
    </span><span class="kw">pub fn </span>as_str(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str {
        <span class="self">self</span>.iter.as_str()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>GraphemeIndices&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = (usize, <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str);

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;(usize, <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str)&gt; {
        <span class="self">self</span>.iter
            .next()
            .map(|s| (s.as_ptr() <span class="kw">as </span>usize - <span class="self">self</span>.start_offset, s))
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        <span class="self">self</span>.iter.size_hint()
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; DoubleEndedIterator <span class="kw">for </span>GraphemeIndices&lt;<span class="lifetime">'a</span>&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>next_back(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;(usize, <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str)&gt; {
        <span class="self">self</span>.iter
            .next_back()
            .map(|s| (s.as_ptr() <span class="kw">as </span>usize - <span class="self">self</span>.start_offset, s))
    }
}

<span class="doccomment">/// External iterator for a string's
/// [grapheme clusters](http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries).
///
/// This struct is created by the [`graphemes`] method on the [`UnicodeSegmentation`] trait. See its
/// documentation for more.
///
/// [`graphemes`]: trait.UnicodeSegmentation.html#tymethod.graphemes
/// [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>Graphemes&lt;<span class="lifetime">'a</span>&gt; {
    string: <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str,
    cursor: GraphemeCursor,
    cursor_back: GraphemeCursor,
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Graphemes&lt;<span class="lifetime">'a</span>&gt; {
    <span class="attr">#[inline]
    </span><span class="doccomment">/// View the underlying data (the part yet to be iterated) as a slice of the original string.
    ///
    /// ```rust
    /// # use unicode_segmentation::UnicodeSegmentation;
    /// let mut iter = "abc".graphemes(true);
    /// assert_eq!(iter.as_str(), "abc");
    /// iter.next();
    /// assert_eq!(iter.as_str(), "bc");
    /// iter.next();
    /// iter.next();
    /// assert_eq!(iter.as_str(), "");
    /// ```
    </span><span class="kw">pub fn </span>as_str(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str {
        <span class="kw-2">&amp;</span><span class="self">self</span>.string[<span class="self">self</span>.cursor.cur_cursor()..<span class="self">self</span>.cursor_back.cur_cursor()]
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; Iterator <span class="kw">for </span>Graphemes&lt;<span class="lifetime">'a</span>&gt; {
    <span class="kw">type </span>Item = <span class="kw-2">&amp;</span><span class="lifetime">'a </span>str;

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>size_hint(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; (usize, <span class="prelude-ty">Option</span>&lt;usize&gt;) {
        <span class="kw">let </span>slen = <span class="self">self</span>.cursor_back.cur_cursor() - <span class="self">self</span>.cursor.cur_cursor();
        (cmp::min(slen, <span class="number">1</span>), <span class="prelude-val">Some</span>(slen))
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>next(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str&gt; {
        <span class="kw">let </span>start = <span class="self">self</span>.cursor.cur_cursor();
        <span class="kw">if </span>start == <span class="self">self</span>.cursor_back.cur_cursor() {
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }
        <span class="kw">let </span>next = <span class="self">self</span>.cursor.next_boundary(<span class="self">self</span>.string, <span class="number">0</span>).unwrap().unwrap();
        <span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span><span class="self">self</span>.string[start..next])
    }
}

<span class="kw">impl</span>&lt;<span class="lifetime">'a</span>&gt; DoubleEndedIterator <span class="kw">for </span>Graphemes&lt;<span class="lifetime">'a</span>&gt; {
    <span class="attr">#[inline]
    </span><span class="kw">fn </span>next_back(<span class="kw-2">&amp;mut </span><span class="self">self</span>) -&gt; <span class="prelude-ty">Option</span>&lt;<span class="kw-2">&amp;</span><span class="lifetime">'a </span>str&gt; {
        <span class="kw">let </span>end = <span class="self">self</span>.cursor_back.cur_cursor();
        <span class="kw">if </span>end == <span class="self">self</span>.cursor.cur_cursor() {
            <span class="kw">return </span><span class="prelude-val">None</span>;
        }
        <span class="kw">let </span>prev = <span class="self">self
            </span>.cursor_back
            .prev_boundary(<span class="self">self</span>.string, <span class="number">0</span>)
            .unwrap()
            .unwrap();
        <span class="prelude-val">Some</span>(<span class="kw-2">&amp;</span><span class="self">self</span>.string[prev..end])
    }
}

<span class="attr">#[inline]
</span><span class="kw">pub fn </span>new_graphemes&lt;<span class="lifetime">'b</span>&gt;(s: <span class="kw-2">&amp;</span><span class="lifetime">'b </span>str, is_extended: bool) -&gt; Graphemes&lt;<span class="lifetime">'b</span>&gt; {
    <span class="kw">let </span>len = s.len();
    Graphemes {
        string: s,
        cursor: GraphemeCursor::new(<span class="number">0</span>, len, is_extended),
        cursor_back: GraphemeCursor::new(len, len, is_extended),
    }
}

<span class="attr">#[inline]
</span><span class="kw">pub fn </span>new_grapheme_indices&lt;<span class="lifetime">'b</span>&gt;(s: <span class="kw-2">&amp;</span><span class="lifetime">'b </span>str, is_extended: bool) -&gt; GraphemeIndices&lt;<span class="lifetime">'b</span>&gt; {
    GraphemeIndices {
        start_offset: s.as_ptr() <span class="kw">as </span>usize,
        iter: new_graphemes(s, is_extended),
    }
}

<span class="comment">// maybe unify with PairResult?
// An enum describing information about a potential boundary.
</span><span class="attr">#[derive(PartialEq, Eq, Clone, Debug)]
</span><span class="kw">enum </span>GraphemeState {
    <span class="comment">// No information is known.
    </span>Unknown,
    <span class="comment">// It is known to not be a boundary.
    </span>NotBreak,
    <span class="comment">// It is known to be a boundary.
    </span>Break,
    <span class="comment">// The codepoint after is a Regional Indicator Symbol, so a boundary iff
    // it is preceded by an even number of RIS codepoints. (GB12, GB13)
    </span>Regional,
    <span class="comment">// The codepoint after is Extended_Pictographic,
    // so whether it's a boundary depends on pre-context according to GB11.
    </span>Emoji,
}

<span class="doccomment">/// Cursor-based segmenter for grapheme clusters.
///
/// This allows working with ropes and other datastructures where the string is not contiguous or
/// fully known at initialization time.
</span><span class="attr">#[derive(Clone, Debug)]
</span><span class="kw">pub struct </span>GraphemeCursor {
    <span class="comment">// Current cursor position.
    </span>offset: usize,
    <span class="comment">// Total length of the string.
    </span>len: usize,
    <span class="comment">// A config flag indicating whether this cursor computes legacy or extended
    // grapheme cluster boundaries (enables GB9a and GB9b if set).
    </span>is_extended: bool,
    <span class="comment">// Information about the potential boundary at `offset`
    </span>state: GraphemeState,
    <span class="comment">// Category of codepoint immediately preceding cursor, if known.
    </span>cat_before: <span class="prelude-ty">Option</span>&lt;GraphemeCat&gt;,
    <span class="comment">// Category of codepoint immediately after cursor, if known.
    </span>cat_after: <span class="prelude-ty">Option</span>&lt;GraphemeCat&gt;,
    <span class="comment">// If set, at least one more codepoint immediately preceding this offset
    // is needed to resolve whether there's a boundary at `offset`.
    </span>pre_context_offset: <span class="prelude-ty">Option</span>&lt;usize&gt;,
    <span class="comment">// The number of RIS codepoints preceding `offset`. If `pre_context_offset`
    // is set, then counts the number of RIS between that and `offset`, otherwise
    // is an accurate count relative to the string.
    </span>ris_count: <span class="prelude-ty">Option</span>&lt;usize&gt;,
    <span class="comment">// Set if a call to `prev_boundary` or `next_boundary` was suspended due
    // to needing more input.
    </span>resuming: bool,
    <span class="comment">// Cached grapheme category and associated scalar value range.
    </span>grapheme_cat_cache: (u32, u32, GraphemeCat),
}

<span class="doccomment">/// An error return indicating that not enough content was available in the
/// provided chunk to satisfy the query, and that more content must be provided.
</span><span class="attr">#[derive(PartialEq, Eq, Debug)]
</span><span class="kw">pub enum </span>GraphemeIncomplete {
    <span class="doccomment">/// More pre-context is needed. The caller should call `provide_context`
    /// with a chunk ending at the offset given, then retry the query. This
    /// will only be returned if the `chunk_start` parameter is nonzero.
    </span>PreContext(usize),

    <span class="doccomment">/// When requesting `prev_boundary`, the cursor is moving past the beginning
    /// of the current chunk, so the chunk before that is requested. This will
    /// only be returned if the `chunk_start` parameter is nonzero.
    </span>PrevChunk,

    <span class="doccomment">/// When requesting `next_boundary`, the cursor is moving past the end of the
    /// current chunk, so the chunk after that is requested. This will only be
    /// returned if the chunk ends before the `len` parameter provided on
    /// creation of the cursor.
    </span>NextChunk, <span class="comment">// requesting chunk following the one given

    </span><span class="doccomment">/// An error returned when the chunk given does not contain the cursor position.
    </span>InvalidOffset,
}

<span class="comment">// An enum describing the result from lookup of a pair of categories.
</span><span class="attr">#[derive(PartialEq, Eq)]
</span><span class="kw">enum </span>PairResult {
    NotBreak, <span class="comment">// definitely not a break
    </span>Break,    <span class="comment">// definitely a break
    </span>Extended, <span class="comment">// a break iff not in extended mode
    </span>Regional, <span class="comment">// a break if preceded by an even number of RIS
    </span>Emoji,    <span class="comment">// a break if preceded by emoji base and (Extend)*
</span>}

<span class="attr">#[inline]
</span><span class="kw">fn </span>check_pair(before: GraphemeCat, after: GraphemeCat) -&gt; PairResult {
    <span class="kw">use </span><span class="self">self</span>::PairResult::<span class="kw-2">*</span>;
    <span class="kw">use </span><span class="kw">crate</span>::tables::grapheme::GraphemeCat::<span class="kw-2">*</span>;
    <span class="kw">match </span>(before, after) {
        (GC_CR, GC_LF) =&gt; NotBreak,                                 <span class="comment">// GB3
        </span>(GC_Control, <span class="kw">_</span>) =&gt; Break,                                   <span class="comment">// GB4
        </span>(GC_CR, <span class="kw">_</span>) =&gt; Break,                                        <span class="comment">// GB4
        </span>(GC_LF, <span class="kw">_</span>) =&gt; Break,                                        <span class="comment">// GB4
        </span>(<span class="kw">_</span>, GC_Control) =&gt; Break,                                   <span class="comment">// GB5
        </span>(<span class="kw">_</span>, GC_CR) =&gt; Break,                                        <span class="comment">// GB5
        </span>(<span class="kw">_</span>, GC_LF) =&gt; Break,                                        <span class="comment">// GB5
        </span>(GC_L, GC_L) =&gt; NotBreak,                                   <span class="comment">// GB6
        </span>(GC_L, GC_V) =&gt; NotBreak,                                   <span class="comment">// GB6
        </span>(GC_L, GC_LV) =&gt; NotBreak,                                  <span class="comment">// GB6
        </span>(GC_L, GC_LVT) =&gt; NotBreak,                                 <span class="comment">// GB6
        </span>(GC_LV, GC_V) =&gt; NotBreak,                                  <span class="comment">// GB7
        </span>(GC_LV, GC_T) =&gt; NotBreak,                                  <span class="comment">// GB7
        </span>(GC_V, GC_V) =&gt; NotBreak,                                   <span class="comment">// GB7
        </span>(GC_V, GC_T) =&gt; NotBreak,                                   <span class="comment">// GB7
        </span>(GC_LVT, GC_T) =&gt; NotBreak,                                 <span class="comment">// GB8
        </span>(GC_T, GC_T) =&gt; NotBreak,                                   <span class="comment">// GB8
        </span>(<span class="kw">_</span>, GC_Extend) =&gt; NotBreak,                                 <span class="comment">// GB9
        </span>(<span class="kw">_</span>, GC_ZWJ) =&gt; NotBreak,                                    <span class="comment">// GB9
        </span>(<span class="kw">_</span>, GC_SpacingMark) =&gt; Extended,                            <span class="comment">// GB9a
        </span>(GC_Prepend, <span class="kw">_</span>) =&gt; Extended,                                <span class="comment">// GB9b
        </span>(GC_ZWJ, GC_Extended_Pictographic) =&gt; Emoji,                <span class="comment">// GB11
        </span>(GC_Regional_Indicator, GC_Regional_Indicator) =&gt; Regional, <span class="comment">// GB12, GB13
        </span>(<span class="kw">_</span>, <span class="kw">_</span>) =&gt; Break,                                            <span class="comment">// GB999
    </span>}
}

<span class="kw">impl </span>GraphemeCursor {
    <span class="doccomment">/// Create a new cursor. The string and initial offset are given at creation
    /// time, but the contents of the string are not. The `is_extended` parameter
    /// controls whether extended grapheme clusters are selected.
    ///
    /// The `offset` parameter must be on a codepoint boundary.
    ///
    /// ```rust
    /// # use unicode_segmentation::GraphemeCursor;
    /// let s = "हिन्दी";
    /// let mut legacy = GraphemeCursor::new(0, s.len(), false);
    /// assert_eq!(legacy.next_boundary(s, 0), Ok(Some("ह".len())));
    /// let mut extended = GraphemeCursor::new(0, s.len(), true);
    /// assert_eq!(extended.next_boundary(s, 0), Ok(Some("हि".len())));
    /// ```
    </span><span class="kw">pub fn </span>new(offset: usize, len: usize, is_extended: bool) -&gt; GraphemeCursor {
        <span class="kw">let </span>state = <span class="kw">if </span>offset == <span class="number">0 </span>|| offset == len {
            GraphemeState::Break
        } <span class="kw">else </span>{
            GraphemeState::Unknown
        };
        GraphemeCursor {
            offset: offset,
            len: len,
            state: state,
            is_extended: is_extended,
            cat_before: <span class="prelude-val">None</span>,
            cat_after: <span class="prelude-val">None</span>,
            pre_context_offset: <span class="prelude-val">None</span>,
            ris_count: <span class="prelude-val">None</span>,
            resuming: <span class="bool-val">false</span>,
            grapheme_cat_cache: (<span class="number">0</span>, <span class="number">0</span>, GraphemeCat::GC_Control),
        }
    }

    <span class="kw">fn </span>grapheme_category(<span class="kw-2">&amp;mut </span><span class="self">self</span>, ch: char) -&gt; GraphemeCat {
        <span class="kw">use </span><span class="kw">crate</span>::tables::grapheme <span class="kw">as </span>gr;
        <span class="kw">use </span><span class="kw">crate</span>::tables::grapheme::GraphemeCat::<span class="kw-2">*</span>;

        <span class="kw">if </span>ch &lt;= <span class="string">'\u{7e}' </span>{
            <span class="comment">// Special-case optimization for ascii, except U+007F.  This
            // improves performance even for many primarily non-ascii texts,
            // due to use of punctuation and white space characters from the
            // ascii range.
            </span><span class="kw">if </span>ch &gt;= <span class="string">'\u{20}' </span>{
                GC_Any
            } <span class="kw">else if </span>ch == <span class="string">'\n' </span>{
                GC_LF
            } <span class="kw">else if </span>ch == <span class="string">'\r' </span>{
                GC_CR
            } <span class="kw">else </span>{
                GC_Control
            }
        } <span class="kw">else </span>{
            <span class="comment">// If this char isn't within the cached range, update the cache to the
            // range that includes it.
            </span><span class="kw">if </span>(ch <span class="kw">as </span>u32) &lt; <span class="self">self</span>.grapheme_cat_cache.<span class="number">0 </span>|| (ch <span class="kw">as </span>u32) &gt; <span class="self">self</span>.grapheme_cat_cache.<span class="number">1 </span>{
                <span class="self">self</span>.grapheme_cat_cache = gr::grapheme_category(ch);
            }
            <span class="self">self</span>.grapheme_cat_cache.<span class="number">2
        </span>}
    }

    <span class="comment">// Not sure I'm gonna keep this, the advantage over new() seems thin.

    </span><span class="doccomment">/// Set the cursor to a new location in the same string.
    ///
    /// ```rust
    /// # use unicode_segmentation::GraphemeCursor;
    /// let s = "abcd";
    /// let mut cursor = GraphemeCursor::new(0, s.len(), false);
    /// assert_eq!(cursor.cur_cursor(), 0);
    /// cursor.set_cursor(2);
    /// assert_eq!(cursor.cur_cursor(), 2);
    /// ```
    </span><span class="kw">pub fn </span>set_cursor(<span class="kw-2">&amp;mut </span><span class="self">self</span>, offset: usize) {
        <span class="kw">if </span>offset != <span class="self">self</span>.offset {
            <span class="self">self</span>.offset = offset;
            <span class="self">self</span>.state = <span class="kw">if </span>offset == <span class="number">0 </span>|| offset == <span class="self">self</span>.len {
                GraphemeState::Break
            } <span class="kw">else </span>{
                GraphemeState::Unknown
            };
            <span class="comment">// reset state derived from text around cursor
            </span><span class="self">self</span>.cat_before = <span class="prelude-val">None</span>;
            <span class="self">self</span>.cat_after = <span class="prelude-val">None</span>;
            <span class="self">self</span>.ris_count = <span class="prelude-val">None</span>;
        }
    }

    <span class="attr">#[inline]
    </span><span class="doccomment">/// The current offset of the cursor. Equal to the last value provided to
    /// `new()` or `set_cursor()`, or returned from `next_boundary()` or
    /// `prev_boundary()`.
    ///
    /// ```rust
    /// # use unicode_segmentation::GraphemeCursor;
    /// // Two flags (🇷🇸🇮🇴), each flag is two RIS codepoints, each RIS is 4 bytes.
    /// let flags = "\u{1F1F7}\u{1F1F8}\u{1F1EE}\u{1F1F4}";
    /// let mut cursor = GraphemeCursor::new(4, flags.len(), false);
    /// assert_eq!(cursor.cur_cursor(), 4);
    /// assert_eq!(cursor.next_boundary(flags, 0), Ok(Some(8)));
    /// assert_eq!(cursor.cur_cursor(), 8);
    /// ```
    </span><span class="kw">pub fn </span>cur_cursor(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; usize {
        <span class="self">self</span>.offset
    }

    <span class="doccomment">/// Provide additional pre-context when it is needed to decide a boundary.
    /// The end of the chunk must coincide with the value given in the
    /// `GraphemeIncomplete::PreContext` request.
    ///
    /// ```rust
    /// # use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};
    /// let flags = "\u{1F1F7}\u{1F1F8}\u{1F1EE}\u{1F1F4}";
    /// let mut cursor = GraphemeCursor::new(8, flags.len(), false);
    /// // Not enough pre-context to decide if there's a boundary between the two flags.
    /// assert_eq!(cursor.is_boundary(&amp;flags[8..], 8), Err(GraphemeIncomplete::PreContext(8)));
    /// // Provide one more Regional Indicator Symbol of pre-context
    /// cursor.provide_context(&amp;flags[4..8], 4);
    /// // Still not enough context to decide.
    /// assert_eq!(cursor.is_boundary(&amp;flags[8..], 8), Err(GraphemeIncomplete::PreContext(4)));
    /// // Provide additional requested context.
    /// cursor.provide_context(&amp;flags[0..4], 0);
    /// // That's enough to decide (it always is when context goes to the start of the string)
    /// assert_eq!(cursor.is_boundary(&amp;flags[8..], 8), Ok(true));
    /// ```
    </span><span class="kw">pub fn </span>provide_context(<span class="kw-2">&amp;mut </span><span class="self">self</span>, chunk: <span class="kw-2">&amp;</span>str, chunk_start: usize) {
        <span class="kw">use </span><span class="kw">crate</span>::tables::grapheme <span class="kw">as </span>gr;
        <span class="macro">assert!</span>(chunk_start + chunk.len() == <span class="self">self</span>.pre_context_offset.unwrap());
        <span class="self">self</span>.pre_context_offset = <span class="prelude-val">None</span>;
        <span class="kw">if </span><span class="self">self</span>.is_extended &amp;&amp; chunk_start + chunk.len() == <span class="self">self</span>.offset {
            <span class="kw">let </span>ch = chunk.chars().rev().next().unwrap();
            <span class="kw">if </span><span class="self">self</span>.grapheme_category(ch) == gr::GC_Prepend {
                <span class="self">self</span>.decide(<span class="bool-val">false</span>); <span class="comment">// GB9b
                </span><span class="kw">return</span>;
            }
        }
        <span class="kw">match </span><span class="self">self</span>.state {
            GraphemeState::Regional =&gt; <span class="self">self</span>.handle_regional(chunk, chunk_start),
            GraphemeState::Emoji =&gt; <span class="self">self</span>.handle_emoji(chunk, chunk_start),
            <span class="kw">_ </span>=&gt; {
                <span class="kw">if </span><span class="self">self</span>.cat_before.is_none() &amp;&amp; <span class="self">self</span>.offset == chunk.len() + chunk_start {
                    <span class="kw">let </span>ch = chunk.chars().rev().next().unwrap();
                    <span class="self">self</span>.cat_before = <span class="prelude-val">Some</span>(<span class="self">self</span>.grapheme_category(ch));
                }
            }
        }
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>decide(<span class="kw-2">&amp;mut </span><span class="self">self</span>, is_break: bool) {
        <span class="self">self</span>.state = <span class="kw">if </span>is_break {
            GraphemeState::Break
        } <span class="kw">else </span>{
            GraphemeState::NotBreak
        };
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>decision(<span class="kw-2">&amp;mut </span><span class="self">self</span>, is_break: bool) -&gt; <span class="prelude-ty">Result</span>&lt;bool, GraphemeIncomplete&gt; {
        <span class="self">self</span>.decide(is_break);
        <span class="prelude-val">Ok</span>(is_break)
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>is_boundary_result(<span class="kw-2">&amp;</span><span class="self">self</span>) -&gt; <span class="prelude-ty">Result</span>&lt;bool, GraphemeIncomplete&gt; {
        <span class="kw">if </span><span class="self">self</span>.state == GraphemeState::Break {
            <span class="prelude-val">Ok</span>(<span class="bool-val">true</span>)
        } <span class="kw">else if </span><span class="self">self</span>.state == GraphemeState::NotBreak {
            <span class="prelude-val">Ok</span>(<span class="bool-val">false</span>)
        } <span class="kw">else if let </span><span class="prelude-val">Some</span>(pre_context_offset) = <span class="self">self</span>.pre_context_offset {
            <span class="prelude-val">Err</span>(GraphemeIncomplete::PreContext(pre_context_offset))
        } <span class="kw">else </span>{
            <span class="macro">unreachable!</span>(<span class="string">"inconsistent state"</span>);
        }
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>handle_regional(<span class="kw-2">&amp;mut </span><span class="self">self</span>, chunk: <span class="kw-2">&amp;</span>str, chunk_start: usize) {
        <span class="kw">use </span><span class="kw">crate</span>::tables::grapheme <span class="kw">as </span>gr;
        <span class="kw">let </span><span class="kw-2">mut </span>ris_count = <span class="self">self</span>.ris_count.unwrap_or(<span class="number">0</span>);
        <span class="kw">for </span>ch <span class="kw">in </span>chunk.chars().rev() {
            <span class="kw">if </span><span class="self">self</span>.grapheme_category(ch) != gr::GC_Regional_Indicator {
                <span class="self">self</span>.ris_count = <span class="prelude-val">Some</span>(ris_count);
                <span class="self">self</span>.decide((ris_count % <span class="number">2</span>) == <span class="number">0</span>);
                <span class="kw">return</span>;
            }
            ris_count += <span class="number">1</span>;
        }
        <span class="self">self</span>.ris_count = <span class="prelude-val">Some</span>(ris_count);
        <span class="kw">if </span>chunk_start == <span class="number">0 </span>{
            <span class="self">self</span>.decide((ris_count % <span class="number">2</span>) == <span class="number">0</span>);
            <span class="kw">return</span>;
        }
        <span class="self">self</span>.pre_context_offset = <span class="prelude-val">Some</span>(chunk_start);
        <span class="self">self</span>.state = GraphemeState::Regional;
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>handle_emoji(<span class="kw-2">&amp;mut </span><span class="self">self</span>, chunk: <span class="kw-2">&amp;</span>str, chunk_start: usize) {
        <span class="kw">use </span><span class="kw">crate</span>::tables::grapheme <span class="kw">as </span>gr;
        <span class="kw">let </span><span class="kw-2">mut </span>iter = chunk.chars().rev();
        <span class="kw">if let </span><span class="prelude-val">Some</span>(ch) = iter.next() {
            <span class="kw">if </span><span class="self">self</span>.grapheme_category(ch) != gr::GC_ZWJ {
                <span class="self">self</span>.decide(<span class="bool-val">true</span>);
                <span class="kw">return</span>;
            }
        }
        <span class="kw">for </span>ch <span class="kw">in </span>iter {
            <span class="kw">match </span><span class="self">self</span>.grapheme_category(ch) {
                gr::GC_Extend =&gt; (),
                gr::GC_Extended_Pictographic =&gt; {
                    <span class="self">self</span>.decide(<span class="bool-val">false</span>);
                    <span class="kw">return</span>;
                }
                <span class="kw">_ </span>=&gt; {
                    <span class="self">self</span>.decide(<span class="bool-val">true</span>);
                    <span class="kw">return</span>;
                }
            }
        }
        <span class="kw">if </span>chunk_start == <span class="number">0 </span>{
            <span class="self">self</span>.decide(<span class="bool-val">true</span>);
            <span class="kw">return</span>;
        }
        <span class="self">self</span>.pre_context_offset = <span class="prelude-val">Some</span>(chunk_start);
        <span class="self">self</span>.state = GraphemeState::Emoji;
    }

    <span class="attr">#[inline]
    </span><span class="doccomment">/// Determine whether the current cursor location is a grapheme cluster boundary.
    /// Only a part of the string need be supplied. If `chunk_start` is nonzero or
    /// the length of `chunk` is not equal to `len` on creation, then this method
    /// may return `GraphemeIncomplete::PreContext`. The caller should then
    /// call `provide_context` with the requested chunk, then retry calling this
    /// method.
    ///
    /// For partial chunks, if the cursor is not at the beginning or end of the
    /// string, the chunk should contain at least the codepoint following the cursor.
    /// If the string is nonempty, the chunk must be nonempty.
    ///
    /// All calls should have consistent chunk contents (ie, if a chunk provides
    /// content for a given slice, all further chunks covering that slice must have
    /// the same content for it).
    ///
    /// ```rust
    /// # use unicode_segmentation::GraphemeCursor;
    /// let flags = "\u{1F1F7}\u{1F1F8}\u{1F1EE}\u{1F1F4}";
    /// let mut cursor = GraphemeCursor::new(8, flags.len(), false);
    /// assert_eq!(cursor.is_boundary(flags, 0), Ok(true));
    /// cursor.set_cursor(12);
    /// assert_eq!(cursor.is_boundary(flags, 0), Ok(false));
    /// ```
    </span><span class="kw">pub fn </span>is_boundary(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        chunk: <span class="kw-2">&amp;</span>str,
        chunk_start: usize,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;bool, GraphemeIncomplete&gt; {
        <span class="kw">use </span><span class="kw">crate</span>::tables::grapheme <span class="kw">as </span>gr;
        <span class="kw">if </span><span class="self">self</span>.state == GraphemeState::Break {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="bool-val">true</span>);
        }
        <span class="kw">if </span><span class="self">self</span>.state == GraphemeState::NotBreak {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="bool-val">false</span>);
        }
        <span class="kw">if </span><span class="self">self</span>.offset &lt; chunk_start || <span class="self">self</span>.offset &gt;= chunk_start + chunk.len() {
            <span class="kw">if </span><span class="self">self</span>.offset &gt; chunk_start + chunk.len() || <span class="self">self</span>.cat_after.is_none() {
                <span class="kw">return </span><span class="prelude-val">Err</span>(GraphemeIncomplete::InvalidOffset);
            }
        }
        <span class="kw">if let </span><span class="prelude-val">Some</span>(pre_context_offset) = <span class="self">self</span>.pre_context_offset {
            <span class="kw">return </span><span class="prelude-val">Err</span>(GraphemeIncomplete::PreContext(pre_context_offset));
        }
        <span class="kw">let </span>offset_in_chunk = <span class="self">self</span>.offset - chunk_start;
        <span class="kw">if </span><span class="self">self</span>.cat_after.is_none() {
            <span class="kw">let </span>ch = chunk[offset_in_chunk..].chars().next().unwrap();
            <span class="self">self</span>.cat_after = <span class="prelude-val">Some</span>(<span class="self">self</span>.grapheme_category(ch));
        }
        <span class="kw">if </span><span class="self">self</span>.offset == chunk_start {
            <span class="kw">let </span><span class="kw-2">mut </span>need_pre_context = <span class="bool-val">true</span>;
            <span class="kw">match </span><span class="self">self</span>.cat_after.unwrap() {
                gr::GC_Regional_Indicator =&gt; <span class="self">self</span>.state = GraphemeState::Regional,
                gr::GC_Extended_Pictographic =&gt; <span class="self">self</span>.state = GraphemeState::Emoji,
                <span class="kw">_ </span>=&gt; need_pre_context = <span class="self">self</span>.cat_before.is_none(),
            }
            <span class="kw">if </span>need_pre_context {
                <span class="self">self</span>.pre_context_offset = <span class="prelude-val">Some</span>(chunk_start);
                <span class="kw">return </span><span class="prelude-val">Err</span>(GraphemeIncomplete::PreContext(chunk_start));
            }
        }
        <span class="kw">if </span><span class="self">self</span>.cat_before.is_none() {
            <span class="kw">let </span>ch = chunk[..offset_in_chunk].chars().rev().next().unwrap();
            <span class="self">self</span>.cat_before = <span class="prelude-val">Some</span>(<span class="self">self</span>.grapheme_category(ch));
        }
        <span class="kw">match </span>check_pair(<span class="self">self</span>.cat_before.unwrap(), <span class="self">self</span>.cat_after.unwrap()) {
            PairResult::NotBreak =&gt; <span class="kw">return </span><span class="self">self</span>.decision(<span class="bool-val">false</span>),
            PairResult::Break =&gt; <span class="kw">return </span><span class="self">self</span>.decision(<span class="bool-val">true</span>),
            PairResult::Extended =&gt; {
                <span class="kw">let </span>is_extended = <span class="self">self</span>.is_extended;
                <span class="kw">return </span><span class="self">self</span>.decision(!is_extended);
            }
            PairResult::Regional =&gt; {
                <span class="kw">if let </span><span class="prelude-val">Some</span>(ris_count) = <span class="self">self</span>.ris_count {
                    <span class="kw">return </span><span class="self">self</span>.decision((ris_count % <span class="number">2</span>) == <span class="number">0</span>);
                }
                <span class="self">self</span>.handle_regional(<span class="kw-2">&amp;</span>chunk[..offset_in_chunk], chunk_start);
                <span class="self">self</span>.is_boundary_result()
            }
            PairResult::Emoji =&gt; {
                <span class="self">self</span>.handle_emoji(<span class="kw-2">&amp;</span>chunk[..offset_in_chunk], chunk_start);
                <span class="self">self</span>.is_boundary_result()
            }
        }
    }

    <span class="attr">#[inline]
    </span><span class="doccomment">/// Find the next boundary after the current cursor position. Only a part of
    /// the string need be supplied. If the chunk is incomplete, then this
    /// method might return `GraphemeIncomplete::PreContext` or
    /// `GraphemeIncomplete::NextChunk`. In the former case, the caller should
    /// call `provide_context` with the requested chunk, then retry. In the
    /// latter case, the caller should provide the chunk following the one
    /// given, then retry.
    ///
    /// See `is_boundary` for expectations on the provided chunk.
    ///
    /// ```rust
    /// # use unicode_segmentation::GraphemeCursor;
    /// let flags = "\u{1F1F7}\u{1F1F8}\u{1F1EE}\u{1F1F4}";
    /// let mut cursor = GraphemeCursor::new(4, flags.len(), false);
    /// assert_eq!(cursor.next_boundary(flags, 0), Ok(Some(8)));
    /// assert_eq!(cursor.next_boundary(flags, 0), Ok(Some(16)));
    /// assert_eq!(cursor.next_boundary(flags, 0), Ok(None));
    /// ```
    ///
    /// And an example that uses partial strings:
    ///
    /// ```rust
    /// # use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};
    /// let s = "abcd";
    /// let mut cursor = GraphemeCursor::new(0, s.len(), false);
    /// assert_eq!(cursor.next_boundary(&amp;s[..2], 0), Ok(Some(1)));
    /// assert_eq!(cursor.next_boundary(&amp;s[..2], 0), Err(GraphemeIncomplete::NextChunk));
    /// assert_eq!(cursor.next_boundary(&amp;s[2..4], 2), Ok(Some(2)));
    /// assert_eq!(cursor.next_boundary(&amp;s[2..4], 2), Ok(Some(3)));
    /// assert_eq!(cursor.next_boundary(&amp;s[2..4], 2), Ok(Some(4)));
    /// assert_eq!(cursor.next_boundary(&amp;s[2..4], 2), Ok(None));
    /// ```
    </span><span class="kw">pub fn </span>next_boundary(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        chunk: <span class="kw-2">&amp;</span>str,
        chunk_start: usize,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="prelude-ty">Option</span>&lt;usize&gt;, GraphemeIncomplete&gt; {
        <span class="kw">if </span><span class="self">self</span>.offset == <span class="self">self</span>.len {
            <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="prelude-val">None</span>);
        }
        <span class="kw">let </span><span class="kw-2">mut </span>iter = chunk[<span class="self">self</span>.offset - chunk_start..].chars();
        <span class="kw">let </span><span class="kw-2">mut </span>ch = iter.next().unwrap();
        <span class="kw">loop </span>{
            <span class="kw">if </span><span class="self">self</span>.resuming {
                <span class="kw">if </span><span class="self">self</span>.cat_after.is_none() {
                    <span class="self">self</span>.cat_after = <span class="prelude-val">Some</span>(<span class="self">self</span>.grapheme_category(ch));
                }
            } <span class="kw">else </span>{
                <span class="self">self</span>.offset += ch.len_utf8();
                <span class="self">self</span>.state = GraphemeState::Unknown;
                <span class="self">self</span>.cat_before = <span class="self">self</span>.cat_after.take();
                <span class="kw">if </span><span class="self">self</span>.cat_before.is_none() {
                    <span class="self">self</span>.cat_before = <span class="prelude-val">Some</span>(<span class="self">self</span>.grapheme_category(ch));
                }
                <span class="kw">if </span><span class="self">self</span>.cat_before.unwrap() == GraphemeCat::GC_Regional_Indicator {
                    <span class="self">self</span>.ris_count = <span class="self">self</span>.ris_count.map(|c| c + <span class="number">1</span>);
                } <span class="kw">else </span>{
                    <span class="self">self</span>.ris_count = <span class="prelude-val">Some</span>(<span class="number">0</span>);
                }
                <span class="kw">if let </span><span class="prelude-val">Some</span>(next_ch) = iter.next() {
                    ch = next_ch;
                    <span class="self">self</span>.cat_after = <span class="prelude-val">Some</span>(<span class="self">self</span>.grapheme_category(ch));
                } <span class="kw">else if </span><span class="self">self</span>.offset == <span class="self">self</span>.len {
                    <span class="self">self</span>.decide(<span class="bool-val">true</span>);
                } <span class="kw">else </span>{
                    <span class="self">self</span>.resuming = <span class="bool-val">true</span>;
                    <span class="kw">return </span><span class="prelude-val">Err</span>(GraphemeIncomplete::NextChunk);
                }
            }
            <span class="self">self</span>.resuming = <span class="bool-val">true</span>;
            <span class="kw">if </span><span class="self">self</span>.is_boundary(chunk, chunk_start)<span class="question-mark">? </span>{
                <span class="self">self</span>.resuming = <span class="bool-val">false</span>;
                <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="prelude-val">Some</span>(<span class="self">self</span>.offset));
            }
            <span class="self">self</span>.resuming = <span class="bool-val">false</span>;
        }
    }

    <span class="doccomment">/// Find the previous boundary after the current cursor position. Only a part
    /// of the string need be supplied. If the chunk is incomplete, then this
    /// method might return `GraphemeIncomplete::PreContext` or
    /// `GraphemeIncomplete::PrevChunk`. In the former case, the caller should
    /// call `provide_context` with the requested chunk, then retry. In the
    /// latter case, the caller should provide the chunk preceding the one
    /// given, then retry.
    ///
    /// See `is_boundary` for expectations on the provided chunk.
    ///
    /// ```rust
    /// # use unicode_segmentation::GraphemeCursor;
    /// let flags = "\u{1F1F7}\u{1F1F8}\u{1F1EE}\u{1F1F4}";
    /// let mut cursor = GraphemeCursor::new(12, flags.len(), false);
    /// assert_eq!(cursor.prev_boundary(flags, 0), Ok(Some(8)));
    /// assert_eq!(cursor.prev_boundary(flags, 0), Ok(Some(0)));
    /// assert_eq!(cursor.prev_boundary(flags, 0), Ok(None));
    /// ```
    ///
    /// And an example that uses partial strings (note the exact return is not
    /// guaranteed, and may be `PrevChunk` or `PreContext` arbitrarily):
    ///
    /// ```rust
    /// # use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};
    /// let s = "abcd";
    /// let mut cursor = GraphemeCursor::new(4, s.len(), false);
    /// assert_eq!(cursor.prev_boundary(&amp;s[2..4], 2), Ok(Some(3)));
    /// assert_eq!(cursor.prev_boundary(&amp;s[2..4], 2), Err(GraphemeIncomplete::PrevChunk));
    /// assert_eq!(cursor.prev_boundary(&amp;s[0..2], 0), Ok(Some(2)));
    /// assert_eq!(cursor.prev_boundary(&amp;s[0..2], 0), Ok(Some(1)));
    /// assert_eq!(cursor.prev_boundary(&amp;s[0..2], 0), Ok(Some(0)));
    /// assert_eq!(cursor.prev_boundary(&amp;s[0..2], 0), Ok(None));
    /// ```
    </span><span class="kw">pub fn </span>prev_boundary(
        <span class="kw-2">&amp;mut </span><span class="self">self</span>,
        chunk: <span class="kw-2">&amp;</span>str,
        chunk_start: usize,
    ) -&gt; <span class="prelude-ty">Result</span>&lt;<span class="prelude-ty">Option</span>&lt;usize&gt;, GraphemeIncomplete&gt; {
        <span class="kw">if </span><span class="self">self</span>.offset == <span class="number">0 </span>{
            <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="prelude-val">None</span>);
        }
        <span class="kw">if </span><span class="self">self</span>.offset == chunk_start {
            <span class="kw">return </span><span class="prelude-val">Err</span>(GraphemeIncomplete::PrevChunk);
        }
        <span class="kw">let </span><span class="kw-2">mut </span>iter = chunk[..<span class="self">self</span>.offset - chunk_start].chars().rev();
        <span class="kw">let </span><span class="kw-2">mut </span>ch = iter.next().unwrap();
        <span class="kw">loop </span>{
            <span class="kw">if </span><span class="self">self</span>.offset == chunk_start {
                <span class="self">self</span>.resuming = <span class="bool-val">true</span>;
                <span class="kw">return </span><span class="prelude-val">Err</span>(GraphemeIncomplete::PrevChunk);
            }
            <span class="kw">if </span><span class="self">self</span>.resuming {
                <span class="self">self</span>.cat_before = <span class="prelude-val">Some</span>(<span class="self">self</span>.grapheme_category(ch));
            } <span class="kw">else </span>{
                <span class="self">self</span>.offset -= ch.len_utf8();
                <span class="self">self</span>.cat_after = <span class="self">self</span>.cat_before.take();
                <span class="self">self</span>.state = GraphemeState::Unknown;
                <span class="kw">if let </span><span class="prelude-val">Some</span>(ris_count) = <span class="self">self</span>.ris_count {
                    <span class="self">self</span>.ris_count = <span class="kw">if </span>ris_count &gt; <span class="number">0 </span>{
                        <span class="prelude-val">Some</span>(ris_count - <span class="number">1</span>)
                    } <span class="kw">else </span>{
                        <span class="prelude-val">None
                    </span>};
                }
                <span class="kw">if let </span><span class="prelude-val">Some</span>(prev_ch) = iter.next() {
                    ch = prev_ch;
                    <span class="self">self</span>.cat_before = <span class="prelude-val">Some</span>(<span class="self">self</span>.grapheme_category(ch));
                } <span class="kw">else if </span><span class="self">self</span>.offset == <span class="number">0 </span>{
                    <span class="self">self</span>.decide(<span class="bool-val">true</span>);
                } <span class="kw">else </span>{
                    <span class="self">self</span>.resuming = <span class="bool-val">true</span>;
                    <span class="self">self</span>.cat_after = <span class="prelude-val">Some</span>(<span class="self">self</span>.grapheme_category(ch));
                    <span class="kw">return </span><span class="prelude-val">Err</span>(GraphemeIncomplete::PrevChunk);
                }
            }
            <span class="self">self</span>.resuming = <span class="bool-val">true</span>;
            <span class="kw">if </span><span class="self">self</span>.is_boundary(chunk, chunk_start)<span class="question-mark">? </span>{
                <span class="self">self</span>.resuming = <span class="bool-val">false</span>;
                <span class="kw">return </span><span class="prelude-val">Ok</span>(<span class="prelude-val">Some</span>(<span class="self">self</span>.offset));
            }
            <span class="self">self</span>.resuming = <span class="bool-val">false</span>;
        }
    }
}

<span class="attr">#[test]
</span><span class="kw">fn </span>test_grapheme_cursor_ris_precontext() {
    <span class="kw">let </span>s = <span class="string">"\u{1f1fa}\u{1f1f8}\u{1f1fa}\u{1f1f8}\u{1f1fa}\u{1f1f8}"</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>c = GraphemeCursor::new(<span class="number">8</span>, s.len(), <span class="bool-val">true</span>);
    <span class="macro">assert_eq!</span>(
        c.is_boundary(<span class="kw-2">&amp;</span>s[<span class="number">4</span>..], <span class="number">4</span>),
        <span class="prelude-val">Err</span>(GraphemeIncomplete::PreContext(<span class="number">4</span>))
    );
    c.provide_context(<span class="kw-2">&amp;</span>s[..<span class="number">4</span>], <span class="number">0</span>);
    <span class="macro">assert_eq!</span>(c.is_boundary(<span class="kw-2">&amp;</span>s[<span class="number">4</span>..], <span class="number">4</span>), <span class="prelude-val">Ok</span>(<span class="bool-val">true</span>));
}

<span class="attr">#[test]
</span><span class="kw">fn </span>test_grapheme_cursor_chunk_start_require_precontext() {
    <span class="kw">let </span>s = <span class="string">"\r\n"</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>c = GraphemeCursor::new(<span class="number">1</span>, s.len(), <span class="bool-val">true</span>);
    <span class="macro">assert_eq!</span>(
        c.is_boundary(<span class="kw-2">&amp;</span>s[<span class="number">1</span>..], <span class="number">1</span>),
        <span class="prelude-val">Err</span>(GraphemeIncomplete::PreContext(<span class="number">1</span>))
    );
    c.provide_context(<span class="kw-2">&amp;</span>s[..<span class="number">1</span>], <span class="number">0</span>);
    <span class="macro">assert_eq!</span>(c.is_boundary(<span class="kw-2">&amp;</span>s[<span class="number">1</span>..], <span class="number">1</span>), <span class="prelude-val">Ok</span>(<span class="bool-val">false</span>));
}

<span class="attr">#[test]
</span><span class="kw">fn </span>test_grapheme_cursor_prev_boundary() {
    <span class="kw">let </span>s = <span class="string">"abcd"</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>c = GraphemeCursor::new(<span class="number">3</span>, s.len(), <span class="bool-val">true</span>);
    <span class="macro">assert_eq!</span>(
        c.prev_boundary(<span class="kw-2">&amp;</span>s[<span class="number">2</span>..], <span class="number">2</span>),
        <span class="prelude-val">Err</span>(GraphemeIncomplete::PrevChunk)
    );
    <span class="macro">assert_eq!</span>(c.prev_boundary(<span class="kw-2">&amp;</span>s[..<span class="number">2</span>], <span class="number">0</span>), <span class="prelude-val">Ok</span>(<span class="prelude-val">Some</span>(<span class="number">2</span>)));
}

<span class="attr">#[test]
</span><span class="kw">fn </span>test_grapheme_cursor_prev_boundary_chunk_start() {
    <span class="kw">let </span>s = <span class="string">"abcd"</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>c = GraphemeCursor::new(<span class="number">2</span>, s.len(), <span class="bool-val">true</span>);
    <span class="macro">assert_eq!</span>(
        c.prev_boundary(<span class="kw-2">&amp;</span>s[<span class="number">2</span>..], <span class="number">2</span>),
        <span class="prelude-val">Err</span>(GraphemeIncomplete::PrevChunk)
    );
    <span class="macro">assert_eq!</span>(c.prev_boundary(<span class="kw-2">&amp;</span>s[..<span class="number">2</span>], <span class="number">0</span>), <span class="prelude-val">Ok</span>(<span class="prelude-val">Some</span>(<span class="number">1</span>)));
}
</code></pre></div></section></main></body></html>