<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/unicode-segmentation-1.11.0/src/tables.rs`."><title>tables.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="unicode_segmentation" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://unicode-rs.github.io/unicode-rs_sm.png"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../unicode_segmentation/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
<a href="#1174" id="1174">1174</a>
<a href="#1175" id="1175">1175</a>
<a href="#1176" id="1176">1176</a>
<a href="#1177" id="1177">1177</a>
<a href="#1178" id="1178">1178</a>
<a href="#1179" id="1179">1179</a>
<a href="#1180" id="1180">1180</a>
<a href="#1181" id="1181">1181</a>
<a href="#1182" id="1182">1182</a>
<a href="#1183" id="1183">1183</a>
<a href="#1184" id="1184">1184</a>
<a href="#1185" id="1185">1185</a>
<a href="#1186" id="1186">1186</a>
<a href="#1187" id="1187">1187</a>
<a href="#1188" id="1188">1188</a>
<a href="#1189" id="1189">1189</a>
<a href="#1190" id="1190">1190</a>
<a href="#1191" id="1191">1191</a>
<a href="#1192" id="1192">1192</a>
<a href="#1193" id="1193">1193</a>
<a href="#1194" id="1194">1194</a>
<a href="#1195" id="1195">1195</a>
<a href="#1196" id="1196">1196</a>
<a href="#1197" id="1197">1197</a>
<a href="#1198" id="1198">1198</a>
<a href="#1199" id="1199">1199</a>
<a href="#1200" id="1200">1200</a>
<a href="#1201" id="1201">1201</a>
<a href="#1202" id="1202">1202</a>
<a href="#1203" id="1203">1203</a>
<a href="#1204" id="1204">1204</a>
<a href="#1205" id="1205">1205</a>
<a href="#1206" id="1206">1206</a>
<a href="#1207" id="1207">1207</a>
<a href="#1208" id="1208">1208</a>
<a href="#1209" id="1209">1209</a>
<a href="#1210" id="1210">1210</a>
<a href="#1211" id="1211">1211</a>
<a href="#1212" id="1212">1212</a>
<a href="#1213" id="1213">1213</a>
<a href="#1214" id="1214">1214</a>
<a href="#1215" id="1215">1215</a>
<a href="#1216" id="1216">1216</a>
<a href="#1217" id="1217">1217</a>
<a href="#1218" id="1218">1218</a>
<a href="#1219" id="1219">1219</a>
<a href="#1220" id="1220">1220</a>
<a href="#1221" id="1221">1221</a>
<a href="#1222" id="1222">1222</a>
<a href="#1223" id="1223">1223</a>
<a href="#1224" id="1224">1224</a>
<a href="#1225" id="1225">1225</a>
<a href="#1226" id="1226">1226</a>
<a href="#1227" id="1227">1227</a>
<a href="#1228" id="1228">1228</a>
<a href="#1229" id="1229">1229</a>
<a href="#1230" id="1230">1230</a>
<a href="#1231" id="1231">1231</a>
<a href="#1232" id="1232">1232</a>
<a href="#1233" id="1233">1233</a>
<a href="#1234" id="1234">1234</a>
<a href="#1235" id="1235">1235</a>
<a href="#1236" id="1236">1236</a>
<a href="#1237" id="1237">1237</a>
<a href="#1238" id="1238">1238</a>
<a href="#1239" id="1239">1239</a>
<a href="#1240" id="1240">1240</a>
<a href="#1241" id="1241">1241</a>
<a href="#1242" id="1242">1242</a>
<a href="#1243" id="1243">1243</a>
<a href="#1244" id="1244">1244</a>
<a href="#1245" id="1245">1245</a>
<a href="#1246" id="1246">1246</a>
<a href="#1247" id="1247">1247</a>
<a href="#1248" id="1248">1248</a>
<a href="#1249" id="1249">1249</a>
<a href="#1250" id="1250">1250</a>
<a href="#1251" id="1251">1251</a>
<a href="#1252" id="1252">1252</a>
<a href="#1253" id="1253">1253</a>
<a href="#1254" id="1254">1254</a>
<a href="#1255" id="1255">1255</a>
<a href="#1256" id="1256">1256</a>
<a href="#1257" id="1257">1257</a>
<a href="#1258" id="1258">1258</a>
<a href="#1259" id="1259">1259</a>
<a href="#1260" id="1260">1260</a>
<a href="#1261" id="1261">1261</a>
<a href="#1262" id="1262">1262</a>
<a href="#1263" id="1263">1263</a>
<a href="#1264" id="1264">1264</a>
<a href="#1265" id="1265">1265</a>
<a href="#1266" id="1266">1266</a>
<a href="#1267" id="1267">1267</a>
<a href="#1268" id="1268">1268</a>
<a href="#1269" id="1269">1269</a>
<a href="#1270" id="1270">1270</a>
<a href="#1271" id="1271">1271</a>
<a href="#1272" id="1272">1272</a>
<a href="#1273" id="1273">1273</a>
<a href="#1274" id="1274">1274</a>
<a href="#1275" id="1275">1275</a>
<a href="#1276" id="1276">1276</a>
<a href="#1277" id="1277">1277</a>
<a href="#1278" id="1278">1278</a>
<a href="#1279" id="1279">1279</a>
<a href="#1280" id="1280">1280</a>
<a href="#1281" id="1281">1281</a>
<a href="#1282" id="1282">1282</a>
<a href="#1283" id="1283">1283</a>
<a href="#1284" id="1284">1284</a>
<a href="#1285" id="1285">1285</a>
<a href="#1286" id="1286">1286</a>
<a href="#1287" id="1287">1287</a>
<a href="#1288" id="1288">1288</a>
<a href="#1289" id="1289">1289</a>
<a href="#1290" id="1290">1290</a>
<a href="#1291" id="1291">1291</a>
<a href="#1292" id="1292">1292</a>
<a href="#1293" id="1293">1293</a>
<a href="#1294" id="1294">1294</a>
<a href="#1295" id="1295">1295</a>
<a href="#1296" id="1296">1296</a>
<a href="#1297" id="1297">1297</a>
<a href="#1298" id="1298">1298</a>
<a href="#1299" id="1299">1299</a>
<a href="#1300" id="1300">1300</a>
<a href="#1301" id="1301">1301</a>
<a href="#1302" id="1302">1302</a>
<a href="#1303" id="1303">1303</a>
<a href="#1304" id="1304">1304</a>
<a href="#1305" id="1305">1305</a>
<a href="#1306" id="1306">1306</a>
<a href="#1307" id="1307">1307</a>
<a href="#1308" id="1308">1308</a>
<a href="#1309" id="1309">1309</a>
<a href="#1310" id="1310">1310</a>
<a href="#1311" id="1311">1311</a>
<a href="#1312" id="1312">1312</a>
<a href="#1313" id="1313">1313</a>
<a href="#1314" id="1314">1314</a>
<a href="#1315" id="1315">1315</a>
<a href="#1316" id="1316">1316</a>
<a href="#1317" id="1317">1317</a>
<a href="#1318" id="1318">1318</a>
<a href="#1319" id="1319">1319</a>
<a href="#1320" id="1320">1320</a>
<a href="#1321" id="1321">1321</a>
<a href="#1322" id="1322">1322</a>
<a href="#1323" id="1323">1323</a>
<a href="#1324" id="1324">1324</a>
<a href="#1325" id="1325">1325</a>
<a href="#1326" id="1326">1326</a>
<a href="#1327" id="1327">1327</a>
<a href="#1328" id="1328">1328</a>
<a href="#1329" id="1329">1329</a>
<a href="#1330" id="1330">1330</a>
<a href="#1331" id="1331">1331</a>
<a href="#1332" id="1332">1332</a>
<a href="#1333" id="1333">1333</a>
<a href="#1334" id="1334">1334</a>
<a href="#1335" id="1335">1335</a>
<a href="#1336" id="1336">1336</a>
<a href="#1337" id="1337">1337</a>
<a href="#1338" id="1338">1338</a>
<a href="#1339" id="1339">1339</a>
<a href="#1340" id="1340">1340</a>
<a href="#1341" id="1341">1341</a>
<a href="#1342" id="1342">1342</a>
<a href="#1343" id="1343">1343</a>
<a href="#1344" id="1344">1344</a>
<a href="#1345" id="1345">1345</a>
<a href="#1346" id="1346">1346</a>
<a href="#1347" id="1347">1347</a>
<a href="#1348" id="1348">1348</a>
<a href="#1349" id="1349">1349</a>
<a href="#1350" id="1350">1350</a>
<a href="#1351" id="1351">1351</a>
<a href="#1352" id="1352">1352</a>
<a href="#1353" id="1353">1353</a>
<a href="#1354" id="1354">1354</a>
<a href="#1355" id="1355">1355</a>
<a href="#1356" id="1356">1356</a>
<a href="#1357" id="1357">1357</a>
<a href="#1358" id="1358">1358</a>
<a href="#1359" id="1359">1359</a>
<a href="#1360" id="1360">1360</a>
<a href="#1361" id="1361">1361</a>
<a href="#1362" id="1362">1362</a>
<a href="#1363" id="1363">1363</a>
<a href="#1364" id="1364">1364</a>
<a href="#1365" id="1365">1365</a>
<a href="#1366" id="1366">1366</a>
<a href="#1367" id="1367">1367</a>
<a href="#1368" id="1368">1368</a>
<a href="#1369" id="1369">1369</a>
<a href="#1370" id="1370">1370</a>
<a href="#1371" id="1371">1371</a>
<a href="#1372" id="1372">1372</a>
<a href="#1373" id="1373">1373</a>
<a href="#1374" id="1374">1374</a>
<a href="#1375" id="1375">1375</a>
<a href="#1376" id="1376">1376</a>
<a href="#1377" id="1377">1377</a>
<a href="#1378" id="1378">1378</a>
<a href="#1379" id="1379">1379</a>
<a href="#1380" id="1380">1380</a>
<a href="#1381" id="1381">1381</a>
<a href="#1382" id="1382">1382</a>
<a href="#1383" id="1383">1383</a>
<a href="#1384" id="1384">1384</a>
<a href="#1385" id="1385">1385</a>
<a href="#1386" id="1386">1386</a>
<a href="#1387" id="1387">1387</a>
<a href="#1388" id="1388">1388</a>
<a href="#1389" id="1389">1389</a>
<a href="#1390" id="1390">1390</a>
<a href="#1391" id="1391">1391</a>
<a href="#1392" id="1392">1392</a>
<a href="#1393" id="1393">1393</a>
<a href="#1394" id="1394">1394</a>
<a href="#1395" id="1395">1395</a>
<a href="#1396" id="1396">1396</a>
<a href="#1397" id="1397">1397</a>
<a href="#1398" id="1398">1398</a>
<a href="#1399" id="1399">1399</a>
<a href="#1400" id="1400">1400</a>
<a href="#1401" id="1401">1401</a>
<a href="#1402" id="1402">1402</a>
<a href="#1403" id="1403">1403</a>
<a href="#1404" id="1404">1404</a>
<a href="#1405" id="1405">1405</a>
<a href="#1406" id="1406">1406</a>
<a href="#1407" id="1407">1407</a>
<a href="#1408" id="1408">1408</a>
<a href="#1409" id="1409">1409</a>
<a href="#1410" id="1410">1410</a>
<a href="#1411" id="1411">1411</a>
<a href="#1412" id="1412">1412</a>
<a href="#1413" id="1413">1413</a>
<a href="#1414" id="1414">1414</a>
<a href="#1415" id="1415">1415</a>
<a href="#1416" id="1416">1416</a>
<a href="#1417" id="1417">1417</a>
<a href="#1418" id="1418">1418</a>
<a href="#1419" id="1419">1419</a>
<a href="#1420" id="1420">1420</a>
<a href="#1421" id="1421">1421</a>
<a href="#1422" id="1422">1422</a>
<a href="#1423" id="1423">1423</a>
<a href="#1424" id="1424">1424</a>
<a href="#1425" id="1425">1425</a>
<a href="#1426" id="1426">1426</a>
<a href="#1427" id="1427">1427</a>
<a href="#1428" id="1428">1428</a>
<a href="#1429" id="1429">1429</a>
<a href="#1430" id="1430">1430</a>
<a href="#1431" id="1431">1431</a>
<a href="#1432" id="1432">1432</a>
<a href="#1433" id="1433">1433</a>
<a href="#1434" id="1434">1434</a>
<a href="#1435" id="1435">1435</a>
<a href="#1436" id="1436">1436</a>
<a href="#1437" id="1437">1437</a>
<a href="#1438" id="1438">1438</a>
<a href="#1439" id="1439">1439</a>
<a href="#1440" id="1440">1440</a>
<a href="#1441" id="1441">1441</a>
<a href="#1442" id="1442">1442</a>
<a href="#1443" id="1443">1443</a>
<a href="#1444" id="1444">1444</a>
<a href="#1445" id="1445">1445</a>
<a href="#1446" id="1446">1446</a>
<a href="#1447" id="1447">1447</a>
<a href="#1448" id="1448">1448</a>
<a href="#1449" id="1449">1449</a>
<a href="#1450" id="1450">1450</a>
<a href="#1451" id="1451">1451</a>
<a href="#1452" id="1452">1452</a>
<a href="#1453" id="1453">1453</a>
<a href="#1454" id="1454">1454</a>
<a href="#1455" id="1455">1455</a>
<a href="#1456" id="1456">1456</a>
<a href="#1457" id="1457">1457</a>
<a href="#1458" id="1458">1458</a>
<a href="#1459" id="1459">1459</a>
<a href="#1460" id="1460">1460</a>
<a href="#1461" id="1461">1461</a>
<a href="#1462" id="1462">1462</a>
<a href="#1463" id="1463">1463</a>
<a href="#1464" id="1464">1464</a>
<a href="#1465" id="1465">1465</a>
<a href="#1466" id="1466">1466</a>
<a href="#1467" id="1467">1467</a>
<a href="#1468" id="1468">1468</a>
<a href="#1469" id="1469">1469</a>
<a href="#1470" id="1470">1470</a>
<a href="#1471" id="1471">1471</a>
<a href="#1472" id="1472">1472</a>
<a href="#1473" id="1473">1473</a>
<a href="#1474" id="1474">1474</a>
<a href="#1475" id="1475">1475</a>
<a href="#1476" id="1476">1476</a>
<a href="#1477" id="1477">1477</a>
<a href="#1478" id="1478">1478</a>
<a href="#1479" id="1479">1479</a>
<a href="#1480" id="1480">1480</a>
<a href="#1481" id="1481">1481</a>
<a href="#1482" id="1482">1482</a>
<a href="#1483" id="1483">1483</a>
<a href="#1484" id="1484">1484</a>
<a href="#1485" id="1485">1485</a>
<a href="#1486" id="1486">1486</a>
<a href="#1487" id="1487">1487</a>
<a href="#1488" id="1488">1488</a>
<a href="#1489" id="1489">1489</a>
<a href="#1490" id="1490">1490</a>
<a href="#1491" id="1491">1491</a>
<a href="#1492" id="1492">1492</a>
<a href="#1493" id="1493">1493</a>
<a href="#1494" id="1494">1494</a>
<a href="#1495" id="1495">1495</a>
<a href="#1496" id="1496">1496</a>
<a href="#1497" id="1497">1497</a>
<a href="#1498" id="1498">1498</a>
<a href="#1499" id="1499">1499</a>
<a href="#1500" id="1500">1500</a>
<a href="#1501" id="1501">1501</a>
<a href="#1502" id="1502">1502</a>
<a href="#1503" id="1503">1503</a>
<a href="#1504" id="1504">1504</a>
<a href="#1505" id="1505">1505</a>
<a href="#1506" id="1506">1506</a>
<a href="#1507" id="1507">1507</a>
<a href="#1508" id="1508">1508</a>
<a href="#1509" id="1509">1509</a>
<a href="#1510" id="1510">1510</a>
<a href="#1511" id="1511">1511</a>
<a href="#1512" id="1512">1512</a>
<a href="#1513" id="1513">1513</a>
<a href="#1514" id="1514">1514</a>
<a href="#1515" id="1515">1515</a>
<a href="#1516" id="1516">1516</a>
<a href="#1517" id="1517">1517</a>
<a href="#1518" id="1518">1518</a>
<a href="#1519" id="1519">1519</a>
<a href="#1520" id="1520">1520</a>
<a href="#1521" id="1521">1521</a>
<a href="#1522" id="1522">1522</a>
<a href="#1523" id="1523">1523</a>
<a href="#1524" id="1524">1524</a>
<a href="#1525" id="1525">1525</a>
<a href="#1526" id="1526">1526</a>
<a href="#1527" id="1527">1527</a>
<a href="#1528" id="1528">1528</a>
<a href="#1529" id="1529">1529</a>
<a href="#1530" id="1530">1530</a>
<a href="#1531" id="1531">1531</a>
<a href="#1532" id="1532">1532</a>
<a href="#1533" id="1533">1533</a>
<a href="#1534" id="1534">1534</a>
<a href="#1535" id="1535">1535</a>
<a href="#1536" id="1536">1536</a>
<a href="#1537" id="1537">1537</a>
<a href="#1538" id="1538">1538</a>
<a href="#1539" id="1539">1539</a>
<a href="#1540" id="1540">1540</a>
<a href="#1541" id="1541">1541</a>
<a href="#1542" id="1542">1542</a>
<a href="#1543" id="1543">1543</a>
<a href="#1544" id="1544">1544</a>
<a href="#1545" id="1545">1545</a>
<a href="#1546" id="1546">1546</a>
<a href="#1547" id="1547">1547</a>
<a href="#1548" id="1548">1548</a>
<a href="#1549" id="1549">1549</a>
<a href="#1550" id="1550">1550</a>
<a href="#1551" id="1551">1551</a>
<a href="#1552" id="1552">1552</a>
<a href="#1553" id="1553">1553</a>
<a href="#1554" id="1554">1554</a>
<a href="#1555" id="1555">1555</a>
<a href="#1556" id="1556">1556</a>
<a href="#1557" id="1557">1557</a>
<a href="#1558" id="1558">1558</a>
<a href="#1559" id="1559">1559</a>
<a href="#1560" id="1560">1560</a>
<a href="#1561" id="1561">1561</a>
<a href="#1562" id="1562">1562</a>
<a href="#1563" id="1563">1563</a>
<a href="#1564" id="1564">1564</a>
<a href="#1565" id="1565">1565</a>
<a href="#1566" id="1566">1566</a>
<a href="#1567" id="1567">1567</a>
<a href="#1568" id="1568">1568</a>
<a href="#1569" id="1569">1569</a>
<a href="#1570" id="1570">1570</a>
<a href="#1571" id="1571">1571</a>
<a href="#1572" id="1572">1572</a>
<a href="#1573" id="1573">1573</a>
<a href="#1574" id="1574">1574</a>
<a href="#1575" id="1575">1575</a>
<a href="#1576" id="1576">1576</a>
<a href="#1577" id="1577">1577</a>
<a href="#1578" id="1578">1578</a>
<a href="#1579" id="1579">1579</a>
<a href="#1580" id="1580">1580</a>
<a href="#1581" id="1581">1581</a>
<a href="#1582" id="1582">1582</a>
<a href="#1583" id="1583">1583</a>
<a href="#1584" id="1584">1584</a>
<a href="#1585" id="1585">1585</a>
<a href="#1586" id="1586">1586</a>
<a href="#1587" id="1587">1587</a>
<a href="#1588" id="1588">1588</a>
<a href="#1589" id="1589">1589</a>
<a href="#1590" id="1590">1590</a>
<a href="#1591" id="1591">1591</a>
<a href="#1592" id="1592">1592</a>
<a href="#1593" id="1593">1593</a>
<a href="#1594" id="1594">1594</a>
<a href="#1595" id="1595">1595</a>
<a href="#1596" id="1596">1596</a>
<a href="#1597" id="1597">1597</a>
<a href="#1598" id="1598">1598</a>
<a href="#1599" id="1599">1599</a>
<a href="#1600" id="1600">1600</a>
<a href="#1601" id="1601">1601</a>
<a href="#1602" id="1602">1602</a>
<a href="#1603" id="1603">1603</a>
<a href="#1604" id="1604">1604</a>
<a href="#1605" id="1605">1605</a>
<a href="#1606" id="1606">1606</a>
<a href="#1607" id="1607">1607</a>
<a href="#1608" id="1608">1608</a>
<a href="#1609" id="1609">1609</a>
<a href="#1610" id="1610">1610</a>
<a href="#1611" id="1611">1611</a>
<a href="#1612" id="1612">1612</a>
<a href="#1613" id="1613">1613</a>
<a href="#1614" id="1614">1614</a>
<a href="#1615" id="1615">1615</a>
<a href="#1616" id="1616">1616</a>
<a href="#1617" id="1617">1617</a>
<a href="#1618" id="1618">1618</a>
<a href="#1619" id="1619">1619</a>
<a href="#1620" id="1620">1620</a>
<a href="#1621" id="1621">1621</a>
<a href="#1622" id="1622">1622</a>
<a href="#1623" id="1623">1623</a>
<a href="#1624" id="1624">1624</a>
<a href="#1625" id="1625">1625</a>
<a href="#1626" id="1626">1626</a>
<a href="#1627" id="1627">1627</a>
<a href="#1628" id="1628">1628</a>
<a href="#1629" id="1629">1629</a>
<a href="#1630" id="1630">1630</a>
<a href="#1631" id="1631">1631</a>
<a href="#1632" id="1632">1632</a>
<a href="#1633" id="1633">1633</a>
<a href="#1634" id="1634">1634</a>
<a href="#1635" id="1635">1635</a>
<a href="#1636" id="1636">1636</a>
<a href="#1637" id="1637">1637</a>
<a href="#1638" id="1638">1638</a>
<a href="#1639" id="1639">1639</a>
<a href="#1640" id="1640">1640</a>
<a href="#1641" id="1641">1641</a>
<a href="#1642" id="1642">1642</a>
<a href="#1643" id="1643">1643</a>
<a href="#1644" id="1644">1644</a>
<a href="#1645" id="1645">1645</a>
<a href="#1646" id="1646">1646</a>
<a href="#1647" id="1647">1647</a>
<a href="#1648" id="1648">1648</a>
<a href="#1649" id="1649">1649</a>
<a href="#1650" id="1650">1650</a>
<a href="#1651" id="1651">1651</a>
<a href="#1652" id="1652">1652</a>
<a href="#1653" id="1653">1653</a>
<a href="#1654" id="1654">1654</a>
<a href="#1655" id="1655">1655</a>
<a href="#1656" id="1656">1656</a>
<a href="#1657" id="1657">1657</a>
<a href="#1658" id="1658">1658</a>
<a href="#1659" id="1659">1659</a>
<a href="#1660" id="1660">1660</a>
<a href="#1661" id="1661">1661</a>
<a href="#1662" id="1662">1662</a>
<a href="#1663" id="1663">1663</a>
<a href="#1664" id="1664">1664</a>
<a href="#1665" id="1665">1665</a>
<a href="#1666" id="1666">1666</a>
<a href="#1667" id="1667">1667</a>
<a href="#1668" id="1668">1668</a>
<a href="#1669" id="1669">1669</a>
<a href="#1670" id="1670">1670</a>
<a href="#1671" id="1671">1671</a>
<a href="#1672" id="1672">1672</a>
<a href="#1673" id="1673">1673</a>
<a href="#1674" id="1674">1674</a>
<a href="#1675" id="1675">1675</a>
<a href="#1676" id="1676">1676</a>
<a href="#1677" id="1677">1677</a>
<a href="#1678" id="1678">1678</a>
<a href="#1679" id="1679">1679</a>
<a href="#1680" id="1680">1680</a>
<a href="#1681" id="1681">1681</a>
<a href="#1682" id="1682">1682</a>
<a href="#1683" id="1683">1683</a>
<a href="#1684" id="1684">1684</a>
<a href="#1685" id="1685">1685</a>
<a href="#1686" id="1686">1686</a>
<a href="#1687" id="1687">1687</a>
<a href="#1688" id="1688">1688</a>
<a href="#1689" id="1689">1689</a>
<a href="#1690" id="1690">1690</a>
<a href="#1691" id="1691">1691</a>
<a href="#1692" id="1692">1692</a>
<a href="#1693" id="1693">1693</a>
<a href="#1694" id="1694">1694</a>
<a href="#1695" id="1695">1695</a>
<a href="#1696" id="1696">1696</a>
<a href="#1697" id="1697">1697</a>
<a href="#1698" id="1698">1698</a>
<a href="#1699" id="1699">1699</a>
<a href="#1700" id="1700">1700</a>
<a href="#1701" id="1701">1701</a>
<a href="#1702" id="1702">1702</a>
<a href="#1703" id="1703">1703</a>
<a href="#1704" id="1704">1704</a>
<a href="#1705" id="1705">1705</a>
<a href="#1706" id="1706">1706</a>
<a href="#1707" id="1707">1707</a>
<a href="#1708" id="1708">1708</a>
<a href="#1709" id="1709">1709</a>
<a href="#1710" id="1710">1710</a>
<a href="#1711" id="1711">1711</a>
<a href="#1712" id="1712">1712</a>
<a href="#1713" id="1713">1713</a>
<a href="#1714" id="1714">1714</a>
<a href="#1715" id="1715">1715</a>
<a href="#1716" id="1716">1716</a>
<a href="#1717" id="1717">1717</a>
<a href="#1718" id="1718">1718</a>
<a href="#1719" id="1719">1719</a>
<a href="#1720" id="1720">1720</a>
<a href="#1721" id="1721">1721</a>
<a href="#1722" id="1722">1722</a>
<a href="#1723" id="1723">1723</a>
<a href="#1724" id="1724">1724</a>
<a href="#1725" id="1725">1725</a>
<a href="#1726" id="1726">1726</a>
<a href="#1727" id="1727">1727</a>
<a href="#1728" id="1728">1728</a>
<a href="#1729" id="1729">1729</a>
<a href="#1730" id="1730">1730</a>
<a href="#1731" id="1731">1731</a>
<a href="#1732" id="1732">1732</a>
<a href="#1733" id="1733">1733</a>
<a href="#1734" id="1734">1734</a>
<a href="#1735" id="1735">1735</a>
<a href="#1736" id="1736">1736</a>
<a href="#1737" id="1737">1737</a>
<a href="#1738" id="1738">1738</a>
<a href="#1739" id="1739">1739</a>
<a href="#1740" id="1740">1740</a>
<a href="#1741" id="1741">1741</a>
<a href="#1742" id="1742">1742</a>
<a href="#1743" id="1743">1743</a>
<a href="#1744" id="1744">1744</a>
<a href="#1745" id="1745">1745</a>
<a href="#1746" id="1746">1746</a>
<a href="#1747" id="1747">1747</a>
<a href="#1748" id="1748">1748</a>
<a href="#1749" id="1749">1749</a>
<a href="#1750" id="1750">1750</a>
<a href="#1751" id="1751">1751</a>
<a href="#1752" id="1752">1752</a>
<a href="#1753" id="1753">1753</a>
<a href="#1754" id="1754">1754</a>
<a href="#1755" id="1755">1755</a>
<a href="#1756" id="1756">1756</a>
<a href="#1757" id="1757">1757</a>
<a href="#1758" id="1758">1758</a>
<a href="#1759" id="1759">1759</a>
<a href="#1760" id="1760">1760</a>
<a href="#1761" id="1761">1761</a>
<a href="#1762" id="1762">1762</a>
<a href="#1763" id="1763">1763</a>
<a href="#1764" id="1764">1764</a>
<a href="#1765" id="1765">1765</a>
<a href="#1766" id="1766">1766</a>
<a href="#1767" id="1767">1767</a>
<a href="#1768" id="1768">1768</a>
<a href="#1769" id="1769">1769</a>
<a href="#1770" id="1770">1770</a>
<a href="#1771" id="1771">1771</a>
<a href="#1772" id="1772">1772</a>
<a href="#1773" id="1773">1773</a>
<a href="#1774" id="1774">1774</a>
<a href="#1775" id="1775">1775</a>
<a href="#1776" id="1776">1776</a>
<a href="#1777" id="1777">1777</a>
<a href="#1778" id="1778">1778</a>
<a href="#1779" id="1779">1779</a>
<a href="#1780" id="1780">1780</a>
<a href="#1781" id="1781">1781</a>
<a href="#1782" id="1782">1782</a>
<a href="#1783" id="1783">1783</a>
<a href="#1784" id="1784">1784</a>
<a href="#1785" id="1785">1785</a>
<a href="#1786" id="1786">1786</a>
<a href="#1787" id="1787">1787</a>
<a href="#1788" id="1788">1788</a>
<a href="#1789" id="1789">1789</a>
<a href="#1790" id="1790">1790</a>
<a href="#1791" id="1791">1791</a>
<a href="#1792" id="1792">1792</a>
<a href="#1793" id="1793">1793</a>
<a href="#1794" id="1794">1794</a>
<a href="#1795" id="1795">1795</a>
<a href="#1796" id="1796">1796</a>
<a href="#1797" id="1797">1797</a>
<a href="#1798" id="1798">1798</a>
<a href="#1799" id="1799">1799</a>
<a href="#1800" id="1800">1800</a>
<a href="#1801" id="1801">1801</a>
<a href="#1802" id="1802">1802</a>
<a href="#1803" id="1803">1803</a>
<a href="#1804" id="1804">1804</a>
<a href="#1805" id="1805">1805</a>
<a href="#1806" id="1806">1806</a>
<a href="#1807" id="1807">1807</a>
<a href="#1808" id="1808">1808</a>
<a href="#1809" id="1809">1809</a>
<a href="#1810" id="1810">1810</a>
<a href="#1811" id="1811">1811</a>
<a href="#1812" id="1812">1812</a>
<a href="#1813" id="1813">1813</a>
<a href="#1814" id="1814">1814</a>
<a href="#1815" id="1815">1815</a>
<a href="#1816" id="1816">1816</a>
<a href="#1817" id="1817">1817</a>
<a href="#1818" id="1818">1818</a>
<a href="#1819" id="1819">1819</a>
<a href="#1820" id="1820">1820</a>
<a href="#1821" id="1821">1821</a>
<a href="#1822" id="1822">1822</a>
<a href="#1823" id="1823">1823</a>
<a href="#1824" id="1824">1824</a>
<a href="#1825" id="1825">1825</a>
<a href="#1826" id="1826">1826</a>
<a href="#1827" id="1827">1827</a>
<a href="#1828" id="1828">1828</a>
<a href="#1829" id="1829">1829</a>
<a href="#1830" id="1830">1830</a>
<a href="#1831" id="1831">1831</a>
<a href="#1832" id="1832">1832</a>
<a href="#1833" id="1833">1833</a>
<a href="#1834" id="1834">1834</a>
<a href="#1835" id="1835">1835</a>
<a href="#1836" id="1836">1836</a>
<a href="#1837" id="1837">1837</a>
<a href="#1838" id="1838">1838</a>
<a href="#1839" id="1839">1839</a>
<a href="#1840" id="1840">1840</a>
<a href="#1841" id="1841">1841</a>
<a href="#1842" id="1842">1842</a>
<a href="#1843" id="1843">1843</a>
<a href="#1844" id="1844">1844</a>
<a href="#1845" id="1845">1845</a>
<a href="#1846" id="1846">1846</a>
<a href="#1847" id="1847">1847</a>
<a href="#1848" id="1848">1848</a>
<a href="#1849" id="1849">1849</a>
<a href="#1850" id="1850">1850</a>
<a href="#1851" id="1851">1851</a>
<a href="#1852" id="1852">1852</a>
<a href="#1853" id="1853">1853</a>
<a href="#1854" id="1854">1854</a>
<a href="#1855" id="1855">1855</a>
<a href="#1856" id="1856">1856</a>
<a href="#1857" id="1857">1857</a>
<a href="#1858" id="1858">1858</a>
<a href="#1859" id="1859">1859</a>
<a href="#1860" id="1860">1860</a>
<a href="#1861" id="1861">1861</a>
<a href="#1862" id="1862">1862</a>
<a href="#1863" id="1863">1863</a>
<a href="#1864" id="1864">1864</a>
<a href="#1865" id="1865">1865</a>
<a href="#1866" id="1866">1866</a>
<a href="#1867" id="1867">1867</a>
<a href="#1868" id="1868">1868</a>
<a href="#1869" id="1869">1869</a>
<a href="#1870" id="1870">1870</a>
<a href="#1871" id="1871">1871</a>
<a href="#1872" id="1872">1872</a>
<a href="#1873" id="1873">1873</a>
<a href="#1874" id="1874">1874</a>
<a href="#1875" id="1875">1875</a>
<a href="#1876" id="1876">1876</a>
<a href="#1877" id="1877">1877</a>
<a href="#1878" id="1878">1878</a>
<a href="#1879" id="1879">1879</a>
<a href="#1880" id="1880">1880</a>
<a href="#1881" id="1881">1881</a>
<a href="#1882" id="1882">1882</a>
<a href="#1883" id="1883">1883</a>
<a href="#1884" id="1884">1884</a>
<a href="#1885" id="1885">1885</a>
<a href="#1886" id="1886">1886</a>
<a href="#1887" id="1887">1887</a>
<a href="#1888" id="1888">1888</a>
<a href="#1889" id="1889">1889</a>
<a href="#1890" id="1890">1890</a>
<a href="#1891" id="1891">1891</a>
<a href="#1892" id="1892">1892</a>
<a href="#1893" id="1893">1893</a>
<a href="#1894" id="1894">1894</a>
<a href="#1895" id="1895">1895</a>
<a href="#1896" id="1896">1896</a>
<a href="#1897" id="1897">1897</a>
<a href="#1898" id="1898">1898</a>
<a href="#1899" id="1899">1899</a>
<a href="#1900" id="1900">1900</a>
<a href="#1901" id="1901">1901</a>
<a href="#1902" id="1902">1902</a>
<a href="#1903" id="1903">1903</a>
<a href="#1904" id="1904">1904</a>
<a href="#1905" id="1905">1905</a>
<a href="#1906" id="1906">1906</a>
<a href="#1907" id="1907">1907</a>
<a href="#1908" id="1908">1908</a>
<a href="#1909" id="1909">1909</a>
<a href="#1910" id="1910">1910</a>
<a href="#1911" id="1911">1911</a>
<a href="#1912" id="1912">1912</a>
<a href="#1913" id="1913">1913</a>
<a href="#1914" id="1914">1914</a>
<a href="#1915" id="1915">1915</a>
<a href="#1916" id="1916">1916</a>
<a href="#1917" id="1917">1917</a>
<a href="#1918" id="1918">1918</a>
<a href="#1919" id="1919">1919</a>
<a href="#1920" id="1920">1920</a>
<a href="#1921" id="1921">1921</a>
<a href="#1922" id="1922">1922</a>
<a href="#1923" id="1923">1923</a>
<a href="#1924" id="1924">1924</a>
<a href="#1925" id="1925">1925</a>
<a href="#1926" id="1926">1926</a>
<a href="#1927" id="1927">1927</a>
<a href="#1928" id="1928">1928</a>
<a href="#1929" id="1929">1929</a>
<a href="#1930" id="1930">1930</a>
<a href="#1931" id="1931">1931</a>
<a href="#1932" id="1932">1932</a>
<a href="#1933" id="1933">1933</a>
<a href="#1934" id="1934">1934</a>
<a href="#1935" id="1935">1935</a>
<a href="#1936" id="1936">1936</a>
<a href="#1937" id="1937">1937</a>
<a href="#1938" id="1938">1938</a>
<a href="#1939" id="1939">1939</a>
<a href="#1940" id="1940">1940</a>
<a href="#1941" id="1941">1941</a>
<a href="#1942" id="1942">1942</a>
<a href="#1943" id="1943">1943</a>
<a href="#1944" id="1944">1944</a>
<a href="#1945" id="1945">1945</a>
<a href="#1946" id="1946">1946</a>
<a href="#1947" id="1947">1947</a>
<a href="#1948" id="1948">1948</a>
<a href="#1949" id="1949">1949</a>
<a href="#1950" id="1950">1950</a>
<a href="#1951" id="1951">1951</a>
<a href="#1952" id="1952">1952</a>
<a href="#1953" id="1953">1953</a>
<a href="#1954" id="1954">1954</a>
<a href="#1955" id="1955">1955</a>
<a href="#1956" id="1956">1956</a>
<a href="#1957" id="1957">1957</a>
<a href="#1958" id="1958">1958</a>
<a href="#1959" id="1959">1959</a>
<a href="#1960" id="1960">1960</a>
<a href="#1961" id="1961">1961</a>
<a href="#1962" id="1962">1962</a>
<a href="#1963" id="1963">1963</a>
<a href="#1964" id="1964">1964</a>
<a href="#1965" id="1965">1965</a>
<a href="#1966" id="1966">1966</a>
<a href="#1967" id="1967">1967</a>
<a href="#1968" id="1968">1968</a>
<a href="#1969" id="1969">1969</a>
<a href="#1970" id="1970">1970</a>
<a href="#1971" id="1971">1971</a>
<a href="#1972" id="1972">1972</a>
<a href="#1973" id="1973">1973</a>
<a href="#1974" id="1974">1974</a>
<a href="#1975" id="1975">1975</a>
<a href="#1976" id="1976">1976</a>
<a href="#1977" id="1977">1977</a>
<a href="#1978" id="1978">1978</a>
<a href="#1979" id="1979">1979</a>
<a href="#1980" id="1980">1980</a>
<a href="#1981" id="1981">1981</a>
<a href="#1982" id="1982">1982</a>
<a href="#1983" id="1983">1983</a>
<a href="#1984" id="1984">1984</a>
<a href="#1985" id="1985">1985</a>
<a href="#1986" id="1986">1986</a>
<a href="#1987" id="1987">1987</a>
<a href="#1988" id="1988">1988</a>
<a href="#1989" id="1989">1989</a>
<a href="#1990" id="1990">1990</a>
<a href="#1991" id="1991">1991</a>
<a href="#1992" id="1992">1992</a>
<a href="#1993" id="1993">1993</a>
<a href="#1994" id="1994">1994</a>
<a href="#1995" id="1995">1995</a>
<a href="#1996" id="1996">1996</a>
<a href="#1997" id="1997">1997</a>
<a href="#1998" id="1998">1998</a>
<a href="#1999" id="1999">1999</a>
<a href="#2000" id="2000">2000</a>
<a href="#2001" id="2001">2001</a>
<a href="#2002" id="2002">2002</a>
<a href="#2003" id="2003">2003</a>
<a href="#2004" id="2004">2004</a>
<a href="#2005" id="2005">2005</a>
<a href="#2006" id="2006">2006</a>
<a href="#2007" id="2007">2007</a>
<a href="#2008" id="2008">2008</a>
<a href="#2009" id="2009">2009</a>
<a href="#2010" id="2010">2010</a>
<a href="#2011" id="2011">2011</a>
<a href="#2012" id="2012">2012</a>
<a href="#2013" id="2013">2013</a>
<a href="#2014" id="2014">2014</a>
<a href="#2015" id="2015">2015</a>
<a href="#2016" id="2016">2016</a>
<a href="#2017" id="2017">2017</a>
<a href="#2018" id="2018">2018</a>
<a href="#2019" id="2019">2019</a>
<a href="#2020" id="2020">2020</a>
<a href="#2021" id="2021">2021</a>
<a href="#2022" id="2022">2022</a>
<a href="#2023" id="2023">2023</a>
<a href="#2024" id="2024">2024</a>
<a href="#2025" id="2025">2025</a>
<a href="#2026" id="2026">2026</a>
<a href="#2027" id="2027">2027</a>
<a href="#2028" id="2028">2028</a>
<a href="#2029" id="2029">2029</a>
<a href="#2030" id="2030">2030</a>
<a href="#2031" id="2031">2031</a>
<a href="#2032" id="2032">2032</a>
<a href="#2033" id="2033">2033</a>
<a href="#2034" id="2034">2034</a>
<a href="#2035" id="2035">2035</a>
<a href="#2036" id="2036">2036</a>
<a href="#2037" id="2037">2037</a>
<a href="#2038" id="2038">2038</a>
<a href="#2039" id="2039">2039</a>
<a href="#2040" id="2040">2040</a>
<a href="#2041" id="2041">2041</a>
<a href="#2042" id="2042">2042</a>
<a href="#2043" id="2043">2043</a>
<a href="#2044" id="2044">2044</a>
<a href="#2045" id="2045">2045</a>
<a href="#2046" id="2046">2046</a>
<a href="#2047" id="2047">2047</a>
<a href="#2048" id="2048">2048</a>
<a href="#2049" id="2049">2049</a>
<a href="#2050" id="2050">2050</a>
<a href="#2051" id="2051">2051</a>
<a href="#2052" id="2052">2052</a>
<a href="#2053" id="2053">2053</a>
<a href="#2054" id="2054">2054</a>
<a href="#2055" id="2055">2055</a>
<a href="#2056" id="2056">2056</a>
<a href="#2057" id="2057">2057</a>
<a href="#2058" id="2058">2058</a>
<a href="#2059" id="2059">2059</a>
<a href="#2060" id="2060">2060</a>
<a href="#2061" id="2061">2061</a>
<a href="#2062" id="2062">2062</a>
<a href="#2063" id="2063">2063</a>
<a href="#2064" id="2064">2064</a>
<a href="#2065" id="2065">2065</a>
<a href="#2066" id="2066">2066</a>
<a href="#2067" id="2067">2067</a>
<a href="#2068" id="2068">2068</a>
<a href="#2069" id="2069">2069</a>
<a href="#2070" id="2070">2070</a>
<a href="#2071" id="2071">2071</a>
<a href="#2072" id="2072">2072</a>
<a href="#2073" id="2073">2073</a>
<a href="#2074" id="2074">2074</a>
<a href="#2075" id="2075">2075</a>
<a href="#2076" id="2076">2076</a>
<a href="#2077" id="2077">2077</a>
<a href="#2078" id="2078">2078</a>
<a href="#2079" id="2079">2079</a>
<a href="#2080" id="2080">2080</a>
<a href="#2081" id="2081">2081</a>
<a href="#2082" id="2082">2082</a>
<a href="#2083" id="2083">2083</a>
<a href="#2084" id="2084">2084</a>
<a href="#2085" id="2085">2085</a>
<a href="#2086" id="2086">2086</a>
<a href="#2087" id="2087">2087</a>
<a href="#2088" id="2088">2088</a>
<a href="#2089" id="2089">2089</a>
<a href="#2090" id="2090">2090</a>
<a href="#2091" id="2091">2091</a>
<a href="#2092" id="2092">2092</a>
<a href="#2093" id="2093">2093</a>
<a href="#2094" id="2094">2094</a>
<a href="#2095" id="2095">2095</a>
<a href="#2096" id="2096">2096</a>
<a href="#2097" id="2097">2097</a>
<a href="#2098" id="2098">2098</a>
<a href="#2099" id="2099">2099</a>
<a href="#2100" id="2100">2100</a>
<a href="#2101" id="2101">2101</a>
<a href="#2102" id="2102">2102</a>
<a href="#2103" id="2103">2103</a>
<a href="#2104" id="2104">2104</a>
<a href="#2105" id="2105">2105</a>
<a href="#2106" id="2106">2106</a>
<a href="#2107" id="2107">2107</a>
<a href="#2108" id="2108">2108</a>
<a href="#2109" id="2109">2109</a>
<a href="#2110" id="2110">2110</a>
<a href="#2111" id="2111">2111</a>
<a href="#2112" id="2112">2112</a>
<a href="#2113" id="2113">2113</a>
<a href="#2114" id="2114">2114</a>
<a href="#2115" id="2115">2115</a>
<a href="#2116" id="2116">2116</a>
<a href="#2117" id="2117">2117</a>
<a href="#2118" id="2118">2118</a>
<a href="#2119" id="2119">2119</a>
<a href="#2120" id="2120">2120</a>
<a href="#2121" id="2121">2121</a>
<a href="#2122" id="2122">2122</a>
<a href="#2123" id="2123">2123</a>
<a href="#2124" id="2124">2124</a>
<a href="#2125" id="2125">2125</a>
<a href="#2126" id="2126">2126</a>
<a href="#2127" id="2127">2127</a>
<a href="#2128" id="2128">2128</a>
<a href="#2129" id="2129">2129</a>
<a href="#2130" id="2130">2130</a>
<a href="#2131" id="2131">2131</a>
<a href="#2132" id="2132">2132</a>
<a href="#2133" id="2133">2133</a>
<a href="#2134" id="2134">2134</a>
<a href="#2135" id="2135">2135</a>
<a href="#2136" id="2136">2136</a>
<a href="#2137" id="2137">2137</a>
<a href="#2138" id="2138">2138</a>
<a href="#2139" id="2139">2139</a>
<a href="#2140" id="2140">2140</a>
<a href="#2141" id="2141">2141</a>
<a href="#2142" id="2142">2142</a>
<a href="#2143" id="2143">2143</a>
<a href="#2144" id="2144">2144</a>
<a href="#2145" id="2145">2145</a>
<a href="#2146" id="2146">2146</a>
<a href="#2147" id="2147">2147</a>
<a href="#2148" id="2148">2148</a>
<a href="#2149" id="2149">2149</a>
<a href="#2150" id="2150">2150</a>
<a href="#2151" id="2151">2151</a>
<a href="#2152" id="2152">2152</a>
<a href="#2153" id="2153">2153</a>
<a href="#2154" id="2154">2154</a>
<a href="#2155" id="2155">2155</a>
<a href="#2156" id="2156">2156</a>
<a href="#2157" id="2157">2157</a>
<a href="#2158" id="2158">2158</a>
<a href="#2159" id="2159">2159</a>
<a href="#2160" id="2160">2160</a>
<a href="#2161" id="2161">2161</a>
<a href="#2162" id="2162">2162</a>
<a href="#2163" id="2163">2163</a>
<a href="#2164" id="2164">2164</a>
<a href="#2165" id="2165">2165</a>
<a href="#2166" id="2166">2166</a>
<a href="#2167" id="2167">2167</a>
<a href="#2168" id="2168">2168</a>
<a href="#2169" id="2169">2169</a>
<a href="#2170" id="2170">2170</a>
<a href="#2171" id="2171">2171</a>
<a href="#2172" id="2172">2172</a>
<a href="#2173" id="2173">2173</a>
<a href="#2174" id="2174">2174</a>
<a href="#2175" id="2175">2175</a>
<a href="#2176" id="2176">2176</a>
<a href="#2177" id="2177">2177</a>
<a href="#2178" id="2178">2178</a>
<a href="#2179" id="2179">2179</a>
<a href="#2180" id="2180">2180</a>
<a href="#2181" id="2181">2181</a>
<a href="#2182" id="2182">2182</a>
<a href="#2183" id="2183">2183</a>
<a href="#2184" id="2184">2184</a>
<a href="#2185" id="2185">2185</a>
<a href="#2186" id="2186">2186</a>
<a href="#2187" id="2187">2187</a>
<a href="#2188" id="2188">2188</a>
<a href="#2189" id="2189">2189</a>
<a href="#2190" id="2190">2190</a>
<a href="#2191" id="2191">2191</a>
<a href="#2192" id="2192">2192</a>
<a href="#2193" id="2193">2193</a>
<a href="#2194" id="2194">2194</a>
<a href="#2195" id="2195">2195</a>
<a href="#2196" id="2196">2196</a>
<a href="#2197" id="2197">2197</a>
<a href="#2198" id="2198">2198</a>
<a href="#2199" id="2199">2199</a>
<a href="#2200" id="2200">2200</a>
<a href="#2201" id="2201">2201</a>
<a href="#2202" id="2202">2202</a>
<a href="#2203" id="2203">2203</a>
<a href="#2204" id="2204">2204</a>
<a href="#2205" id="2205">2205</a>
<a href="#2206" id="2206">2206</a>
<a href="#2207" id="2207">2207</a>
<a href="#2208" id="2208">2208</a>
<a href="#2209" id="2209">2209</a>
<a href="#2210" id="2210">2210</a>
<a href="#2211" id="2211">2211</a>
<a href="#2212" id="2212">2212</a>
<a href="#2213" id="2213">2213</a>
<a href="#2214" id="2214">2214</a>
<a href="#2215" id="2215">2215</a>
<a href="#2216" id="2216">2216</a>
<a href="#2217" id="2217">2217</a>
<a href="#2218" id="2218">2218</a>
<a href="#2219" id="2219">2219</a>
<a href="#2220" id="2220">2220</a>
<a href="#2221" id="2221">2221</a>
<a href="#2222" id="2222">2222</a>
<a href="#2223" id="2223">2223</a>
<a href="#2224" id="2224">2224</a>
<a href="#2225" id="2225">2225</a>
<a href="#2226" id="2226">2226</a>
<a href="#2227" id="2227">2227</a>
<a href="#2228" id="2228">2228</a>
<a href="#2229" id="2229">2229</a>
<a href="#2230" id="2230">2230</a>
<a href="#2231" id="2231">2231</a>
<a href="#2232" id="2232">2232</a>
<a href="#2233" id="2233">2233</a>
<a href="#2234" id="2234">2234</a>
<a href="#2235" id="2235">2235</a>
<a href="#2236" id="2236">2236</a>
<a href="#2237" id="2237">2237</a>
<a href="#2238" id="2238">2238</a>
<a href="#2239" id="2239">2239</a>
<a href="#2240" id="2240">2240</a>
<a href="#2241" id="2241">2241</a>
<a href="#2242" id="2242">2242</a>
<a href="#2243" id="2243">2243</a>
<a href="#2244" id="2244">2244</a>
<a href="#2245" id="2245">2245</a>
<a href="#2246" id="2246">2246</a>
<a href="#2247" id="2247">2247</a>
<a href="#2248" id="2248">2248</a>
<a href="#2249" id="2249">2249</a>
<a href="#2250" id="2250">2250</a>
<a href="#2251" id="2251">2251</a>
<a href="#2252" id="2252">2252</a>
<a href="#2253" id="2253">2253</a>
<a href="#2254" id="2254">2254</a>
<a href="#2255" id="2255">2255</a>
<a href="#2256" id="2256">2256</a>
<a href="#2257" id="2257">2257</a>
<a href="#2258" id="2258">2258</a>
<a href="#2259" id="2259">2259</a>
<a href="#2260" id="2260">2260</a>
<a href="#2261" id="2261">2261</a>
<a href="#2262" id="2262">2262</a>
<a href="#2263" id="2263">2263</a>
<a href="#2264" id="2264">2264</a>
<a href="#2265" id="2265">2265</a>
<a href="#2266" id="2266">2266</a>
<a href="#2267" id="2267">2267</a>
<a href="#2268" id="2268">2268</a>
<a href="#2269" id="2269">2269</a>
<a href="#2270" id="2270">2270</a>
<a href="#2271" id="2271">2271</a>
<a href="#2272" id="2272">2272</a>
<a href="#2273" id="2273">2273</a>
<a href="#2274" id="2274">2274</a>
<a href="#2275" id="2275">2275</a>
<a href="#2276" id="2276">2276</a>
<a href="#2277" id="2277">2277</a>
<a href="#2278" id="2278">2278</a>
<a href="#2279" id="2279">2279</a>
<a href="#2280" id="2280">2280</a>
<a href="#2281" id="2281">2281</a>
<a href="#2282" id="2282">2282</a>
<a href="#2283" id="2283">2283</a>
<a href="#2284" id="2284">2284</a>
<a href="#2285" id="2285">2285</a>
<a href="#2286" id="2286">2286</a>
<a href="#2287" id="2287">2287</a>
<a href="#2288" id="2288">2288</a>
<a href="#2289" id="2289">2289</a>
<a href="#2290" id="2290">2290</a>
<a href="#2291" id="2291">2291</a>
<a href="#2292" id="2292">2292</a>
<a href="#2293" id="2293">2293</a>
<a href="#2294" id="2294">2294</a>
<a href="#2295" id="2295">2295</a>
<a href="#2296" id="2296">2296</a>
<a href="#2297" id="2297">2297</a>
<a href="#2298" id="2298">2298</a>
<a href="#2299" id="2299">2299</a>
<a href="#2300" id="2300">2300</a>
<a href="#2301" id="2301">2301</a>
<a href="#2302" id="2302">2302</a>
<a href="#2303" id="2303">2303</a>
<a href="#2304" id="2304">2304</a>
<a href="#2305" id="2305">2305</a>
<a href="#2306" id="2306">2306</a>
<a href="#2307" id="2307">2307</a>
<a href="#2308" id="2308">2308</a>
<a href="#2309" id="2309">2309</a>
<a href="#2310" id="2310">2310</a>
<a href="#2311" id="2311">2311</a>
<a href="#2312" id="2312">2312</a>
<a href="#2313" id="2313">2313</a>
<a href="#2314" id="2314">2314</a>
<a href="#2315" id="2315">2315</a>
<a href="#2316" id="2316">2316</a>
<a href="#2317" id="2317">2317</a>
<a href="#2318" id="2318">2318</a>
<a href="#2319" id="2319">2319</a>
<a href="#2320" id="2320">2320</a>
<a href="#2321" id="2321">2321</a>
<a href="#2322" id="2322">2322</a>
<a href="#2323" id="2323">2323</a>
<a href="#2324" id="2324">2324</a>
<a href="#2325" id="2325">2325</a>
<a href="#2326" id="2326">2326</a>
<a href="#2327" id="2327">2327</a>
<a href="#2328" id="2328">2328</a>
<a href="#2329" id="2329">2329</a>
<a href="#2330" id="2330">2330</a>
<a href="#2331" id="2331">2331</a>
<a href="#2332" id="2332">2332</a>
<a href="#2333" id="2333">2333</a>
<a href="#2334" id="2334">2334</a>
<a href="#2335" id="2335">2335</a>
<a href="#2336" id="2336">2336</a>
<a href="#2337" id="2337">2337</a>
<a href="#2338" id="2338">2338</a>
<a href="#2339" id="2339">2339</a>
<a href="#2340" id="2340">2340</a>
<a href="#2341" id="2341">2341</a>
<a href="#2342" id="2342">2342</a>
<a href="#2343" id="2343">2343</a>
<a href="#2344" id="2344">2344</a>
<a href="#2345" id="2345">2345</a>
<a href="#2346" id="2346">2346</a>
<a href="#2347" id="2347">2347</a>
<a href="#2348" id="2348">2348</a>
<a href="#2349" id="2349">2349</a>
<a href="#2350" id="2350">2350</a>
<a href="#2351" id="2351">2351</a>
<a href="#2352" id="2352">2352</a>
<a href="#2353" id="2353">2353</a>
<a href="#2354" id="2354">2354</a>
<a href="#2355" id="2355">2355</a>
<a href="#2356" id="2356">2356</a>
<a href="#2357" id="2357">2357</a>
<a href="#2358" id="2358">2358</a>
<a href="#2359" id="2359">2359</a>
<a href="#2360" id="2360">2360</a>
<a href="#2361" id="2361">2361</a>
<a href="#2362" id="2362">2362</a>
<a href="#2363" id="2363">2363</a>
<a href="#2364" id="2364">2364</a>
<a href="#2365" id="2365">2365</a>
<a href="#2366" id="2366">2366</a>
<a href="#2367" id="2367">2367</a>
<a href="#2368" id="2368">2368</a>
<a href="#2369" id="2369">2369</a>
<a href="#2370" id="2370">2370</a>
<a href="#2371" id="2371">2371</a>
<a href="#2372" id="2372">2372</a>
<a href="#2373" id="2373">2373</a>
<a href="#2374" id="2374">2374</a>
<a href="#2375" id="2375">2375</a>
<a href="#2376" id="2376">2376</a>
<a href="#2377" id="2377">2377</a>
<a href="#2378" id="2378">2378</a>
<a href="#2379" id="2379">2379</a>
<a href="#2380" id="2380">2380</a>
<a href="#2381" id="2381">2381</a>
<a href="#2382" id="2382">2382</a>
<a href="#2383" id="2383">2383</a>
<a href="#2384" id="2384">2384</a>
<a href="#2385" id="2385">2385</a>
<a href="#2386" id="2386">2386</a>
<a href="#2387" id="2387">2387</a>
<a href="#2388" id="2388">2388</a>
<a href="#2389" id="2389">2389</a>
<a href="#2390" id="2390">2390</a>
<a href="#2391" id="2391">2391</a>
<a href="#2392" id="2392">2392</a>
<a href="#2393" id="2393">2393</a>
<a href="#2394" id="2394">2394</a>
<a href="#2395" id="2395">2395</a>
<a href="#2396" id="2396">2396</a>
<a href="#2397" id="2397">2397</a>
<a href="#2398" id="2398">2398</a>
<a href="#2399" id="2399">2399</a>
<a href="#2400" id="2400">2400</a>
<a href="#2401" id="2401">2401</a>
<a href="#2402" id="2402">2402</a>
<a href="#2403" id="2403">2403</a>
<a href="#2404" id="2404">2404</a>
<a href="#2405" id="2405">2405</a>
<a href="#2406" id="2406">2406</a>
<a href="#2407" id="2407">2407</a>
<a href="#2408" id="2408">2408</a>
<a href="#2409" id="2409">2409</a>
<a href="#2410" id="2410">2410</a>
<a href="#2411" id="2411">2411</a>
<a href="#2412" id="2412">2412</a>
<a href="#2413" id="2413">2413</a>
<a href="#2414" id="2414">2414</a>
<a href="#2415" id="2415">2415</a>
<a href="#2416" id="2416">2416</a>
<a href="#2417" id="2417">2417</a>
<a href="#2418" id="2418">2418</a>
<a href="#2419" id="2419">2419</a>
<a href="#2420" id="2420">2420</a>
<a href="#2421" id="2421">2421</a>
<a href="#2422" id="2422">2422</a>
<a href="#2423" id="2423">2423</a>
<a href="#2424" id="2424">2424</a>
<a href="#2425" id="2425">2425</a>
<a href="#2426" id="2426">2426</a>
<a href="#2427" id="2427">2427</a>
<a href="#2428" id="2428">2428</a>
<a href="#2429" id="2429">2429</a>
<a href="#2430" id="2430">2430</a>
<a href="#2431" id="2431">2431</a>
<a href="#2432" id="2432">2432</a>
<a href="#2433" id="2433">2433</a>
<a href="#2434" id="2434">2434</a>
<a href="#2435" id="2435">2435</a>
<a href="#2436" id="2436">2436</a>
<a href="#2437" id="2437">2437</a>
<a href="#2438" id="2438">2438</a>
<a href="#2439" id="2439">2439</a>
<a href="#2440" id="2440">2440</a>
<a href="#2441" id="2441">2441</a>
<a href="#2442" id="2442">2442</a>
<a href="#2443" id="2443">2443</a>
<a href="#2444" id="2444">2444</a>
<a href="#2445" id="2445">2445</a>
<a href="#2446" id="2446">2446</a>
<a href="#2447" id="2447">2447</a>
<a href="#2448" id="2448">2448</a>
<a href="#2449" id="2449">2449</a>
<a href="#2450" id="2450">2450</a>
<a href="#2451" id="2451">2451</a>
<a href="#2452" id="2452">2452</a>
<a href="#2453" id="2453">2453</a>
<a href="#2454" id="2454">2454</a>
<a href="#2455" id="2455">2455</a>
<a href="#2456" id="2456">2456</a>
<a href="#2457" id="2457">2457</a>
<a href="#2458" id="2458">2458</a>
<a href="#2459" id="2459">2459</a>
<a href="#2460" id="2460">2460</a>
<a href="#2461" id="2461">2461</a>
<a href="#2462" id="2462">2462</a>
<a href="#2463" id="2463">2463</a>
<a href="#2464" id="2464">2464</a>
<a href="#2465" id="2465">2465</a>
<a href="#2466" id="2466">2466</a>
<a href="#2467" id="2467">2467</a>
<a href="#2468" id="2468">2468</a>
<a href="#2469" id="2469">2469</a>
<a href="#2470" id="2470">2470</a>
<a href="#2471" id="2471">2471</a>
<a href="#2472" id="2472">2472</a>
<a href="#2473" id="2473">2473</a>
<a href="#2474" id="2474">2474</a>
<a href="#2475" id="2475">2475</a>
<a href="#2476" id="2476">2476</a>
<a href="#2477" id="2477">2477</a>
<a href="#2478" id="2478">2478</a>
<a href="#2479" id="2479">2479</a>
<a href="#2480" id="2480">2480</a>
<a href="#2481" id="2481">2481</a>
<a href="#2482" id="2482">2482</a>
<a href="#2483" id="2483">2483</a>
<a href="#2484" id="2484">2484</a>
<a href="#2485" id="2485">2485</a>
<a href="#2486" id="2486">2486</a>
<a href="#2487" id="2487">2487</a>
<a href="#2488" id="2488">2488</a>
<a href="#2489" id="2489">2489</a>
<a href="#2490" id="2490">2490</a>
<a href="#2491" id="2491">2491</a>
<a href="#2492" id="2492">2492</a>
<a href="#2493" id="2493">2493</a>
<a href="#2494" id="2494">2494</a>
<a href="#2495" id="2495">2495</a>
<a href="#2496" id="2496">2496</a>
<a href="#2497" id="2497">2497</a>
<a href="#2498" id="2498">2498</a>
<a href="#2499" id="2499">2499</a>
<a href="#2500" id="2500">2500</a>
<a href="#2501" id="2501">2501</a>
<a href="#2502" id="2502">2502</a>
<a href="#2503" id="2503">2503</a>
<a href="#2504" id="2504">2504</a>
<a href="#2505" id="2505">2505</a>
<a href="#2506" id="2506">2506</a>
<a href="#2507" id="2507">2507</a>
<a href="#2508" id="2508">2508</a>
<a href="#2509" id="2509">2509</a>
<a href="#2510" id="2510">2510</a>
<a href="#2511" id="2511">2511</a>
<a href="#2512" id="2512">2512</a>
<a href="#2513" id="2513">2513</a>
<a href="#2514" id="2514">2514</a>
<a href="#2515" id="2515">2515</a>
<a href="#2516" id="2516">2516</a>
<a href="#2517" id="2517">2517</a>
<a href="#2518" id="2518">2518</a>
<a href="#2519" id="2519">2519</a>
<a href="#2520" id="2520">2520</a>
<a href="#2521" id="2521">2521</a>
<a href="#2522" id="2522">2522</a>
<a href="#2523" id="2523">2523</a>
<a href="#2524" id="2524">2524</a>
<a href="#2525" id="2525">2525</a>
<a href="#2526" id="2526">2526</a>
<a href="#2527" id="2527">2527</a>
<a href="#2528" id="2528">2528</a>
<a href="#2529" id="2529">2529</a>
<a href="#2530" id="2530">2530</a>
<a href="#2531" id="2531">2531</a>
<a href="#2532" id="2532">2532</a>
<a href="#2533" id="2533">2533</a>
<a href="#2534" id="2534">2534</a>
<a href="#2535" id="2535">2535</a>
<a href="#2536" id="2536">2536</a>
<a href="#2537" id="2537">2537</a>
<a href="#2538" id="2538">2538</a>
<a href="#2539" id="2539">2539</a>
<a href="#2540" id="2540">2540</a>
<a href="#2541" id="2541">2541</a>
<a href="#2542" id="2542">2542</a>
<a href="#2543" id="2543">2543</a>
<a href="#2544" id="2544">2544</a>
<a href="#2545" id="2545">2545</a>
<a href="#2546" id="2546">2546</a>
<a href="#2547" id="2547">2547</a>
<a href="#2548" id="2548">2548</a>
<a href="#2549" id="2549">2549</a>
<a href="#2550" id="2550">2550</a>
<a href="#2551" id="2551">2551</a>
<a href="#2552" id="2552">2552</a>
<a href="#2553" id="2553">2553</a>
<a href="#2554" id="2554">2554</a>
<a href="#2555" id="2555">2555</a>
<a href="#2556" id="2556">2556</a>
<a href="#2557" id="2557">2557</a>
<a href="#2558" id="2558">2558</a>
<a href="#2559" id="2559">2559</a>
<a href="#2560" id="2560">2560</a>
<a href="#2561" id="2561">2561</a>
<a href="#2562" id="2562">2562</a>
<a href="#2563" id="2563">2563</a>
<a href="#2564" id="2564">2564</a>
<a href="#2565" id="2565">2565</a>
<a href="#2566" id="2566">2566</a>
<a href="#2567" id="2567">2567</a>
<a href="#2568" id="2568">2568</a>
<a href="#2569" id="2569">2569</a>
<a href="#2570" id="2570">2570</a>
<a href="#2571" id="2571">2571</a>
<a href="#2572" id="2572">2572</a>
<a href="#2573" id="2573">2573</a>
<a href="#2574" id="2574">2574</a>
<a href="#2575" id="2575">2575</a>
<a href="#2576" id="2576">2576</a>
<a href="#2577" id="2577">2577</a>
<a href="#2578" id="2578">2578</a>
<a href="#2579" id="2579">2579</a>
<a href="#2580" id="2580">2580</a>
<a href="#2581" id="2581">2581</a>
<a href="#2582" id="2582">2582</a>
<a href="#2583" id="2583">2583</a>
<a href="#2584" id="2584">2584</a>
<a href="#2585" id="2585">2585</a>
<a href="#2586" id="2586">2586</a>
<a href="#2587" id="2587">2587</a>
<a href="#2588" id="2588">2588</a>
<a href="#2589" id="2589">2589</a>
<a href="#2590" id="2590">2590</a>
<a href="#2591" id="2591">2591</a>
<a href="#2592" id="2592">2592</a>
<a href="#2593" id="2593">2593</a>
<a href="#2594" id="2594">2594</a>
<a href="#2595" id="2595">2595</a>
<a href="#2596" id="2596">2596</a>
<a href="#2597" id="2597">2597</a>
<a href="#2598" id="2598">2598</a>
<a href="#2599" id="2599">2599</a>
<a href="#2600" id="2600">2600</a>
<a href="#2601" id="2601">2601</a>
<a href="#2602" id="2602">2602</a>
<a href="#2603" id="2603">2603</a>
<a href="#2604" id="2604">2604</a>
<a href="#2605" id="2605">2605</a>
<a href="#2606" id="2606">2606</a>
<a href="#2607" id="2607">2607</a>
<a href="#2608" id="2608">2608</a>
<a href="#2609" id="2609">2609</a>
<a href="#2610" id="2610">2610</a>
<a href="#2611" id="2611">2611</a>
<a href="#2612" id="2612">2612</a>
<a href="#2613" id="2613">2613</a>
<a href="#2614" id="2614">2614</a>
<a href="#2615" id="2615">2615</a>
<a href="#2616" id="2616">2616</a>
<a href="#2617" id="2617">2617</a>
<a href="#2618" id="2618">2618</a>
<a href="#2619" id="2619">2619</a>
<a href="#2620" id="2620">2620</a>
<a href="#2621" id="2621">2621</a>
<a href="#2622" id="2622">2622</a>
<a href="#2623" id="2623">2623</a>
<a href="#2624" id="2624">2624</a>
<a href="#2625" id="2625">2625</a>
<a href="#2626" id="2626">2626</a>
<a href="#2627" id="2627">2627</a>
<a href="#2628" id="2628">2628</a>
<a href="#2629" id="2629">2629</a>
<a href="#2630" id="2630">2630</a>
<a href="#2631" id="2631">2631</a>
<a href="#2632" id="2632">2632</a>
<a href="#2633" id="2633">2633</a>
<a href="#2634" id="2634">2634</a>
<a href="#2635" id="2635">2635</a>
<a href="#2636" id="2636">2636</a>
<a href="#2637" id="2637">2637</a>
<a href="#2638" id="2638">2638</a>
<a href="#2639" id="2639">2639</a>
<a href="#2640" id="2640">2640</a>
<a href="#2641" id="2641">2641</a>
<a href="#2642" id="2642">2642</a>
<a href="#2643" id="2643">2643</a>
<a href="#2644" id="2644">2644</a>
<a href="#2645" id="2645">2645</a>
<a href="#2646" id="2646">2646</a>
<a href="#2647" id="2647">2647</a>
<a href="#2648" id="2648">2648</a>
<a href="#2649" id="2649">2649</a>
<a href="#2650" id="2650">2650</a>
<a href="#2651" id="2651">2651</a>
<a href="#2652" id="2652">2652</a>
<a href="#2653" id="2653">2653</a>
<a href="#2654" id="2654">2654</a>
<a href="#2655" id="2655">2655</a>
<a href="#2656" id="2656">2656</a>
<a href="#2657" id="2657">2657</a>
<a href="#2658" id="2658">2658</a>
<a href="#2659" id="2659">2659</a>
<a href="#2660" id="2660">2660</a>
<a href="#2661" id="2661">2661</a>
<a href="#2662" id="2662">2662</a>
<a href="#2663" id="2663">2663</a>
<a href="#2664" id="2664">2664</a>
<a href="#2665" id="2665">2665</a>
<a href="#2666" id="2666">2666</a>
<a href="#2667" id="2667">2667</a>
<a href="#2668" id="2668">2668</a>
<a href="#2669" id="2669">2669</a>
<a href="#2670" id="2670">2670</a>
<a href="#2671" id="2671">2671</a>
<a href="#2672" id="2672">2672</a>
<a href="#2673" id="2673">2673</a>
<a href="#2674" id="2674">2674</a>
<a href="#2675" id="2675">2675</a>
<a href="#2676" id="2676">2676</a>
<a href="#2677" id="2677">2677</a>
<a href="#2678" id="2678">2678</a>
<a href="#2679" id="2679">2679</a>
<a href="#2680" id="2680">2680</a>
<a href="#2681" id="2681">2681</a>
<a href="#2682" id="2682">2682</a>
<a href="#2683" id="2683">2683</a>
<a href="#2684" id="2684">2684</a>
<a href="#2685" id="2685">2685</a>
<a href="#2686" id="2686">2686</a>
<a href="#2687" id="2687">2687</a>
<a href="#2688" id="2688">2688</a>
<a href="#2689" id="2689">2689</a>
<a href="#2690" id="2690">2690</a>
<a href="#2691" id="2691">2691</a>
<a href="#2692" id="2692">2692</a>
<a href="#2693" id="2693">2693</a>
<a href="#2694" id="2694">2694</a>
<a href="#2695" id="2695">2695</a>
<a href="#2696" id="2696">2696</a>
<a href="#2697" id="2697">2697</a>
<a href="#2698" id="2698">2698</a>
<a href="#2699" id="2699">2699</a>
<a href="#2700" id="2700">2700</a>
<a href="#2701" id="2701">2701</a>
<a href="#2702" id="2702">2702</a>
<a href="#2703" id="2703">2703</a>
<a href="#2704" id="2704">2704</a>
<a href="#2705" id="2705">2705</a>
<a href="#2706" id="2706">2706</a>
<a href="#2707" id="2707">2707</a>
<a href="#2708" id="2708">2708</a>
<a href="#2709" id="2709">2709</a>
<a href="#2710" id="2710">2710</a>
<a href="#2711" id="2711">2711</a>
<a href="#2712" id="2712">2712</a>
<a href="#2713" id="2713">2713</a>
<a href="#2714" id="2714">2714</a>
<a href="#2715" id="2715">2715</a>
<a href="#2716" id="2716">2716</a>
<a href="#2717" id="2717">2717</a>
<a href="#2718" id="2718">2718</a>
<a href="#2719" id="2719">2719</a>
<a href="#2720" id="2720">2720</a>
<a href="#2721" id="2721">2721</a>
<a href="#2722" id="2722">2722</a>
<a href="#2723" id="2723">2723</a>
<a href="#2724" id="2724">2724</a>
<a href="#2725" id="2725">2725</a>
<a href="#2726" id="2726">2726</a>
<a href="#2727" id="2727">2727</a>
<a href="#2728" id="2728">2728</a>
<a href="#2729" id="2729">2729</a>
<a href="#2730" id="2730">2730</a>
<a href="#2731" id="2731">2731</a>
<a href="#2732" id="2732">2732</a>
<a href="#2733" id="2733">2733</a>
<a href="#2734" id="2734">2734</a>
<a href="#2735" id="2735">2735</a>
<a href="#2736" id="2736">2736</a>
<a href="#2737" id="2737">2737</a>
<a href="#2738" id="2738">2738</a>
<a href="#2739" id="2739">2739</a>
<a href="#2740" id="2740">2740</a>
<a href="#2741" id="2741">2741</a>
<a href="#2742" id="2742">2742</a>
<a href="#2743" id="2743">2743</a>
<a href="#2744" id="2744">2744</a>
<a href="#2745" id="2745">2745</a>
<a href="#2746" id="2746">2746</a>
<a href="#2747" id="2747">2747</a>
<a href="#2748" id="2748">2748</a>
<a href="#2749" id="2749">2749</a>
<a href="#2750" id="2750">2750</a>
<a href="#2751" id="2751">2751</a>
<a href="#2752" id="2752">2752</a>
<a href="#2753" id="2753">2753</a>
<a href="#2754" id="2754">2754</a>
<a href="#2755" id="2755">2755</a>
<a href="#2756" id="2756">2756</a>
<a href="#2757" id="2757">2757</a>
<a href="#2758" id="2758">2758</a>
<a href="#2759" id="2759">2759</a>
<a href="#2760" id="2760">2760</a>
<a href="#2761" id="2761">2761</a>
<a href="#2762" id="2762">2762</a>
<a href="#2763" id="2763">2763</a>
<a href="#2764" id="2764">2764</a>
<a href="#2765" id="2765">2765</a>
<a href="#2766" id="2766">2766</a>
<a href="#2767" id="2767">2767</a>
<a href="#2768" id="2768">2768</a>
<a href="#2769" id="2769">2769</a>
<a href="#2770" id="2770">2770</a>
<a href="#2771" id="2771">2771</a>
<a href="#2772" id="2772">2772</a>
<a href="#2773" id="2773">2773</a>
<a href="#2774" id="2774">2774</a>
<a href="#2775" id="2775">2775</a>
<a href="#2776" id="2776">2776</a>
<a href="#2777" id="2777">2777</a>
<a href="#2778" id="2778">2778</a>
<a href="#2779" id="2779">2779</a>
<a href="#2780" id="2780">2780</a>
<a href="#2781" id="2781">2781</a>
<a href="#2782" id="2782">2782</a>
<a href="#2783" id="2783">2783</a>
<a href="#2784" id="2784">2784</a>
<a href="#2785" id="2785">2785</a>
<a href="#2786" id="2786">2786</a>
<a href="#2787" id="2787">2787</a>
<a href="#2788" id="2788">2788</a>
<a href="#2789" id="2789">2789</a>
<a href="#2790" id="2790">2790</a>
<a href="#2791" id="2791">2791</a>
<a href="#2792" id="2792">2792</a>
<a href="#2793" id="2793">2793</a>
<a href="#2794" id="2794">2794</a>
<a href="#2795" id="2795">2795</a>
<a href="#2796" id="2796">2796</a>
<a href="#2797" id="2797">2797</a>
<a href="#2798" id="2798">2798</a>
<a href="#2799" id="2799">2799</a>
<a href="#2800" id="2800">2800</a>
<a href="#2801" id="2801">2801</a>
<a href="#2802" id="2802">2802</a>
<a href="#2803" id="2803">2803</a>
<a href="#2804" id="2804">2804</a>
<a href="#2805" id="2805">2805</a>
<a href="#2806" id="2806">2806</a>
<a href="#2807" id="2807">2807</a>
<a href="#2808" id="2808">2808</a>
<a href="#2809" id="2809">2809</a>
<a href="#2810" id="2810">2810</a>
<a href="#2811" id="2811">2811</a>
<a href="#2812" id="2812">2812</a>
<a href="#2813" id="2813">2813</a>
<a href="#2814" id="2814">2814</a>
<a href="#2815" id="2815">2815</a>
<a href="#2816" id="2816">2816</a>
<a href="#2817" id="2817">2817</a>
<a href="#2818" id="2818">2818</a>
<a href="#2819" id="2819">2819</a>
<a href="#2820" id="2820">2820</a>
<a href="#2821" id="2821">2821</a>
<a href="#2822" id="2822">2822</a>
<a href="#2823" id="2823">2823</a>
<a href="#2824" id="2824">2824</a>
<a href="#2825" id="2825">2825</a>
<a href="#2826" id="2826">2826</a>
<a href="#2827" id="2827">2827</a>
<a href="#2828" id="2828">2828</a>
<a href="#2829" id="2829">2829</a>
<a href="#2830" id="2830">2830</a>
<a href="#2831" id="2831">2831</a>
<a href="#2832" id="2832">2832</a>
<a href="#2833" id="2833">2833</a>
<a href="#2834" id="2834">2834</a>
<a href="#2835" id="2835">2835</a>
<a href="#2836" id="2836">2836</a>
<a href="#2837" id="2837">2837</a>
<a href="#2838" id="2838">2838</a>
<a href="#2839" id="2839">2839</a>
<a href="#2840" id="2840">2840</a>
<a href="#2841" id="2841">2841</a>
<a href="#2842" id="2842">2842</a>
<a href="#2843" id="2843">2843</a>
<a href="#2844" id="2844">2844</a>
<a href="#2845" id="2845">2845</a>
<a href="#2846" id="2846">2846</a>
<a href="#2847" id="2847">2847</a>
<a href="#2848" id="2848">2848</a>
<a href="#2849" id="2849">2849</a>
<a href="#2850" id="2850">2850</a>
<a href="#2851" id="2851">2851</a>
<a href="#2852" id="2852">2852</a>
<a href="#2853" id="2853">2853</a>
<a href="#2854" id="2854">2854</a>
<a href="#2855" id="2855">2855</a>
<a href="#2856" id="2856">2856</a>
<a href="#2857" id="2857">2857</a>
<a href="#2858" id="2858">2858</a>
<a href="#2859" id="2859">2859</a>
<a href="#2860" id="2860">2860</a>
<a href="#2861" id="2861">2861</a>
<a href="#2862" id="2862">2862</a>
<a href="#2863" id="2863">2863</a>
<a href="#2864" id="2864">2864</a>
<a href="#2865" id="2865">2865</a>
<a href="#2866" id="2866">2866</a>
<a href="#2867" id="2867">2867</a>
<a href="#2868" id="2868">2868</a>
<a href="#2869" id="2869">2869</a>
<a href="#2870" id="2870">2870</a>
<a href="#2871" id="2871">2871</a>
<a href="#2872" id="2872">2872</a>
<a href="#2873" id="2873">2873</a>
<a href="#2874" id="2874">2874</a>
<a href="#2875" id="2875">2875</a>
<a href="#2876" id="2876">2876</a>
<a href="#2877" id="2877">2877</a>
<a href="#2878" id="2878">2878</a>
<a href="#2879" id="2879">2879</a>
<a href="#2880" id="2880">2880</a>
<a href="#2881" id="2881">2881</a>
<a href="#2882" id="2882">2882</a>
<a href="#2883" id="2883">2883</a>
<a href="#2884" id="2884">2884</a>
<a href="#2885" id="2885">2885</a>
<a href="#2886" id="2886">2886</a>
<a href="#2887" id="2887">2887</a>
<a href="#2888" id="2888">2888</a>
<a href="#2889" id="2889">2889</a>
<a href="#2890" id="2890">2890</a>
<a href="#2891" id="2891">2891</a>
<a href="#2892" id="2892">2892</a>
<a href="#2893" id="2893">2893</a>
<a href="#2894" id="2894">2894</a>
<a href="#2895" id="2895">2895</a>
<a href="#2896" id="2896">2896</a>
<a href="#2897" id="2897">2897</a>
<a href="#2898" id="2898">2898</a>
<a href="#2899" id="2899">2899</a>
<a href="#2900" id="2900">2900</a>
<a href="#2901" id="2901">2901</a>
<a href="#2902" id="2902">2902</a>
<a href="#2903" id="2903">2903</a>
<a href="#2904" id="2904">2904</a>
<a href="#2905" id="2905">2905</a>
<a href="#2906" id="2906">2906</a>
<a href="#2907" id="2907">2907</a>
<a href="#2908" id="2908">2908</a>
<a href="#2909" id="2909">2909</a>
<a href="#2910" id="2910">2910</a>
<a href="#2911" id="2911">2911</a>
<a href="#2912" id="2912">2912</a>
<a href="#2913" id="2913">2913</a>
<a href="#2914" id="2914">2914</a>
<a href="#2915" id="2915">2915</a>
<a href="#2916" id="2916">2916</a>
<a href="#2917" id="2917">2917</a>
<a href="#2918" id="2918">2918</a>
<a href="#2919" id="2919">2919</a>
<a href="#2920" id="2920">2920</a>
<a href="#2921" id="2921">2921</a>
<a href="#2922" id="2922">2922</a>
<a href="#2923" id="2923">2923</a>
<a href="#2924" id="2924">2924</a>
<a href="#2925" id="2925">2925</a>
<a href="#2926" id="2926">2926</a>
<a href="#2927" id="2927">2927</a>
<a href="#2928" id="2928">2928</a>
<a href="#2929" id="2929">2929</a>
<a href="#2930" id="2930">2930</a>
<a href="#2931" id="2931">2931</a>
<a href="#2932" id="2932">2932</a>
<a href="#2933" id="2933">2933</a>
<a href="#2934" id="2934">2934</a>
<a href="#2935" id="2935">2935</a>
<a href="#2936" id="2936">2936</a>
<a href="#2937" id="2937">2937</a>
<a href="#2938" id="2938">2938</a>
<a href="#2939" id="2939">2939</a>
<a href="#2940" id="2940">2940</a>
<a href="#2941" id="2941">2941</a>
<a href="#2942" id="2942">2942</a>
<a href="#2943" id="2943">2943</a>
<a href="#2944" id="2944">2944</a>
<a href="#2945" id="2945">2945</a>
<a href="#2946" id="2946">2946</a>
<a href="#2947" id="2947">2947</a>
<a href="#2948" id="2948">2948</a>
<a href="#2949" id="2949">2949</a>
<a href="#2950" id="2950">2950</a>
<a href="#2951" id="2951">2951</a>
<a href="#2952" id="2952">2952</a>
<a href="#2953" id="2953">2953</a>
<a href="#2954" id="2954">2954</a>
<a href="#2955" id="2955">2955</a>
<a href="#2956" id="2956">2956</a>
<a href="#2957" id="2957">2957</a>
<a href="#2958" id="2958">2958</a>
<a href="#2959" id="2959">2959</a>
<a href="#2960" id="2960">2960</a>
<a href="#2961" id="2961">2961</a>
<a href="#2962" id="2962">2962</a>
<a href="#2963" id="2963">2963</a>
<a href="#2964" id="2964">2964</a>
<a href="#2965" id="2965">2965</a>
<a href="#2966" id="2966">2966</a>
<a href="#2967" id="2967">2967</a>
<a href="#2968" id="2968">2968</a>
<a href="#2969" id="2969">2969</a>
<a href="#2970" id="2970">2970</a>
<a href="#2971" id="2971">2971</a>
<a href="#2972" id="2972">2972</a>
<a href="#2973" id="2973">2973</a>
<a href="#2974" id="2974">2974</a>
<a href="#2975" id="2975">2975</a>
<a href="#2976" id="2976">2976</a>
<a href="#2977" id="2977">2977</a>
<a href="#2978" id="2978">2978</a>
<a href="#2979" id="2979">2979</a>
<a href="#2980" id="2980">2980</a>
<a href="#2981" id="2981">2981</a>
<a href="#2982" id="2982">2982</a>
<a href="#2983" id="2983">2983</a>
<a href="#2984" id="2984">2984</a>
<a href="#2985" id="2985">2985</a>
<a href="#2986" id="2986">2986</a>
<a href="#2987" id="2987">2987</a>
<a href="#2988" id="2988">2988</a>
<a href="#2989" id="2989">2989</a>
<a href="#2990" id="2990">2990</a>
<a href="#2991" id="2991">2991</a>
<a href="#2992" id="2992">2992</a>
</pre></div><pre class="rust"><code><span class="comment">// Copyright 2012-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 &lt;LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0&gt; or the MIT license
// &lt;LICENSE-MIT or http://opensource.org/licenses/MIT&gt;, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// NOTE: The following code was generated by "scripts/unicode.py", do not edit directly

</span><span class="attr">#![allow(missing_docs, non_upper_case_globals, non_snake_case)]

</span><span class="doccomment">/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-segmentation is based on.
</span><span class="kw">pub const </span>UNICODE_VERSION: (u64, u64, u64) = (<span class="number">15</span>, <span class="number">1</span>, <span class="number">0</span>);

<span class="kw">pub mod </span>util {
    <span class="attr">#[inline]
    </span><span class="kw">pub fn </span>bsearch_range_table(c: char, r: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char,char)]) -&gt; bool {
        <span class="kw">use </span>core::cmp::Ordering::{Equal, Less, Greater};
        r.binary_search_by(|<span class="kw-2">&amp;</span>(lo,hi)| {
            <span class="kw">if </span>lo &lt;= c &amp;&amp; c &lt;= hi { Equal }
            <span class="kw">else if </span>hi &lt; c { Less }
            <span class="kw">else </span>{ Greater }
        }).is_ok()
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>is_alphabetic(c: char) -&gt; bool {
        <span class="kw">match </span>c {
            <span class="string">'a' </span>..= <span class="string">'z' </span>| <span class="string">'A' </span>..= <span class="string">'Z' </span>=&gt; <span class="bool-val">true</span>,
            c <span class="kw">if </span>c &gt; <span class="string">'' </span>=&gt; <span class="kw">super</span>::derived_property::Alphabetic(c),
            <span class="kw">_ </span>=&gt; <span class="bool-val">false</span>,
        }
    }

    <span class="attr">#[inline]
    </span><span class="kw">fn </span>is_numeric(c: char) -&gt; bool {
        <span class="kw">match </span>c {
            <span class="string">'0' </span>..= <span class="string">'9' </span>=&gt; <span class="bool-val">true</span>,
            c <span class="kw">if </span>c &gt; <span class="string">'' </span>=&gt; <span class="kw">super</span>::general_category::N(c),
            <span class="kw">_ </span>=&gt; <span class="bool-val">false</span>,
        }
    }

    <span class="attr">#[inline]
    </span><span class="kw">pub fn </span>is_alphanumeric(c: char) -&gt; bool {
        is_alphabetic(c) || is_numeric(c)
    }
}

<span class="kw">mod </span>general_category {
    <span class="kw">const </span>N_table: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char)] = <span class="kw-2">&amp;</span>[
        (<span class="string">'\u{30}'</span>, <span class="string">'\u{39}'</span>), (<span class="string">'\u{b2}'</span>, <span class="string">'\u{b3}'</span>), (<span class="string">'\u{b9}'</span>, <span class="string">'\u{b9}'</span>), (<span class="string">'\u{bc}'</span>, <span class="string">'\u{be}'</span>),
        (<span class="string">'\u{660}'</span>, <span class="string">'\u{669}'</span>), (<span class="string">'\u{6f0}'</span>, <span class="string">'\u{6f9}'</span>), (<span class="string">'\u{7c0}'</span>, <span class="string">'\u{7c9}'</span>), (<span class="string">'\u{966}'</span>,
        <span class="string">'\u{96f}'</span>), (<span class="string">'\u{9e6}'</span>, <span class="string">'\u{9ef}'</span>), (<span class="string">'\u{9f4}'</span>, <span class="string">'\u{9f9}'</span>), (<span class="string">'\u{a66}'</span>, <span class="string">'\u{a6f}'</span>),
        (<span class="string">'\u{ae6}'</span>, <span class="string">'\u{aef}'</span>), (<span class="string">'\u{b66}'</span>, <span class="string">'\u{b6f}'</span>), (<span class="string">'\u{b72}'</span>, <span class="string">'\u{b77}'</span>), (<span class="string">'\u{be6}'</span>,
        <span class="string">'\u{bf2}'</span>), (<span class="string">'\u{c66}'</span>, <span class="string">'\u{c6f}'</span>), (<span class="string">'\u{c78}'</span>, <span class="string">'\u{c7e}'</span>), (<span class="string">'\u{ce6}'</span>, <span class="string">'\u{cef}'</span>),
        (<span class="string">'\u{d58}'</span>, <span class="string">'\u{d5e}'</span>), (<span class="string">'\u{d66}'</span>, <span class="string">'\u{d78}'</span>), (<span class="string">'\u{de6}'</span>, <span class="string">'\u{def}'</span>), (<span class="string">'\u{e50}'</span>,
        <span class="string">'\u{e59}'</span>), (<span class="string">'\u{ed0}'</span>, <span class="string">'\u{ed9}'</span>), (<span class="string">'\u{f20}'</span>, <span class="string">'\u{f33}'</span>), (<span class="string">'\u{1040}'</span>, <span class="string">'\u{1049}'</span>),
        (<span class="string">'\u{1090}'</span>, <span class="string">'\u{1099}'</span>), (<span class="string">'\u{1369}'</span>, <span class="string">'\u{137c}'</span>), (<span class="string">'\u{16ee}'</span>, <span class="string">'\u{16f0}'</span>), (<span class="string">'\u{17e0}'</span>,
        <span class="string">'\u{17e9}'</span>), (<span class="string">'\u{17f0}'</span>, <span class="string">'\u{17f9}'</span>), (<span class="string">'\u{1810}'</span>, <span class="string">'\u{1819}'</span>), (<span class="string">'\u{1946}'</span>, <span class="string">'\u{194f}'</span>),
        (<span class="string">'\u{19d0}'</span>, <span class="string">'\u{19da}'</span>), (<span class="string">'\u{1a80}'</span>, <span class="string">'\u{1a89}'</span>), (<span class="string">'\u{1a90}'</span>, <span class="string">'\u{1a99}'</span>), (<span class="string">'\u{1b50}'</span>,
        <span class="string">'\u{1b59}'</span>), (<span class="string">'\u{1bb0}'</span>, <span class="string">'\u{1bb9}'</span>), (<span class="string">'\u{1c40}'</span>, <span class="string">'\u{1c49}'</span>), (<span class="string">'\u{1c50}'</span>, <span class="string">'\u{1c59}'</span>),
        (<span class="string">'\u{2070}'</span>, <span class="string">'\u{2070}'</span>), (<span class="string">'\u{2074}'</span>, <span class="string">'\u{2079}'</span>), (<span class="string">'\u{2080}'</span>, <span class="string">'\u{2089}'</span>), (<span class="string">'\u{2150}'</span>,
        <span class="string">'\u{2182}'</span>), (<span class="string">'\u{2185}'</span>, <span class="string">'\u{2189}'</span>), (<span class="string">'\u{2460}'</span>, <span class="string">'\u{249b}'</span>), (<span class="string">'\u{24ea}'</span>, <span class="string">'\u{24ff}'</span>),
        (<span class="string">'\u{2776}'</span>, <span class="string">'\u{2793}'</span>), (<span class="string">'\u{2cfd}'</span>, <span class="string">'\u{2cfd}'</span>), (<span class="string">'\u{3007}'</span>, <span class="string">'\u{3007}'</span>), (<span class="string">'\u{3021}'</span>,
        <span class="string">'\u{3029}'</span>), (<span class="string">'\u{3038}'</span>, <span class="string">'\u{303a}'</span>), (<span class="string">'\u{3192}'</span>, <span class="string">'\u{3195}'</span>), (<span class="string">'\u{3220}'</span>, <span class="string">'\u{3229}'</span>),
        (<span class="string">'\u{3248}'</span>, <span class="string">'\u{324f}'</span>), (<span class="string">'\u{3251}'</span>, <span class="string">'\u{325f}'</span>), (<span class="string">'\u{3280}'</span>, <span class="string">'\u{3289}'</span>), (<span class="string">'\u{32b1}'</span>,
        <span class="string">'\u{32bf}'</span>), (<span class="string">'\u{a620}'</span>, <span class="string">'\u{a629}'</span>), (<span class="string">'\u{a6e6}'</span>, <span class="string">'\u{a6ef}'</span>), (<span class="string">'\u{a830}'</span>, <span class="string">'\u{a835}'</span>),
        (<span class="string">'\u{a8d0}'</span>, <span class="string">'\u{a8d9}'</span>), (<span class="string">'\u{a900}'</span>, <span class="string">'\u{a909}'</span>), (<span class="string">'\u{a9d0}'</span>, <span class="string">'\u{a9d9}'</span>), (<span class="string">'\u{a9f0}'</span>,
        <span class="string">'\u{a9f9}'</span>), (<span class="string">'\u{aa50}'</span>, <span class="string">'\u{aa59}'</span>), (<span class="string">'\u{abf0}'</span>, <span class="string">'\u{abf9}'</span>), (<span class="string">'\u{ff10}'</span>, <span class="string">'\u{ff19}'</span>),
        (<span class="string">'\u{10107}'</span>, <span class="string">'\u{10133}'</span>), (<span class="string">'\u{10140}'</span>, <span class="string">'\u{10178}'</span>), (<span class="string">'\u{1018a}'</span>, <span class="string">'\u{1018b}'</span>),
        (<span class="string">'\u{102e1}'</span>, <span class="string">'\u{102fb}'</span>), (<span class="string">'\u{10320}'</span>, <span class="string">'\u{10323}'</span>), (<span class="string">'\u{10341}'</span>, <span class="string">'\u{10341}'</span>),
        (<span class="string">'\u{1034a}'</span>, <span class="string">'\u{1034a}'</span>), (<span class="string">'\u{103d1}'</span>, <span class="string">'\u{103d5}'</span>), (<span class="string">'\u{104a0}'</span>, <span class="string">'\u{104a9}'</span>),
        (<span class="string">'\u{10858}'</span>, <span class="string">'\u{1085f}'</span>), (<span class="string">'\u{10879}'</span>, <span class="string">'\u{1087f}'</span>), (<span class="string">'\u{108a7}'</span>, <span class="string">'\u{108af}'</span>),
        (<span class="string">'\u{108fb}'</span>, <span class="string">'\u{108ff}'</span>), (<span class="string">'\u{10916}'</span>, <span class="string">'\u{1091b}'</span>), (<span class="string">'\u{109bc}'</span>, <span class="string">'\u{109bd}'</span>),
        (<span class="string">'\u{109c0}'</span>, <span class="string">'\u{109cf}'</span>), (<span class="string">'\u{109d2}'</span>, <span class="string">'\u{109ff}'</span>), (<span class="string">'\u{10a40}'</span>, <span class="string">'\u{10a48}'</span>),
        (<span class="string">'\u{10a7d}'</span>, <span class="string">'\u{10a7e}'</span>), (<span class="string">'\u{10a9d}'</span>, <span class="string">'\u{10a9f}'</span>), (<span class="string">'\u{10aeb}'</span>, <span class="string">'\u{10aef}'</span>),
        (<span class="string">'\u{10b58}'</span>, <span class="string">'\u{10b5f}'</span>), (<span class="string">'\u{10b78}'</span>, <span class="string">'\u{10b7f}'</span>), (<span class="string">'\u{10ba9}'</span>, <span class="string">'\u{10baf}'</span>),
        (<span class="string">'\u{10cfa}'</span>, <span class="string">'\u{10cff}'</span>), (<span class="string">'\u{10d30}'</span>, <span class="string">'\u{10d39}'</span>), (<span class="string">'\u{10e60}'</span>, <span class="string">'\u{10e7e}'</span>),
        (<span class="string">'\u{10f1d}'</span>, <span class="string">'\u{10f26}'</span>), (<span class="string">'\u{10f51}'</span>, <span class="string">'\u{10f54}'</span>), (<span class="string">'\u{10fc5}'</span>, <span class="string">'\u{10fcb}'</span>),
        (<span class="string">'\u{11052}'</span>, <span class="string">'\u{1106f}'</span>), (<span class="string">'\u{110f0}'</span>, <span class="string">'\u{110f9}'</span>), (<span class="string">'\u{11136}'</span>, <span class="string">'\u{1113f}'</span>),
        (<span class="string">'\u{111d0}'</span>, <span class="string">'\u{111d9}'</span>), (<span class="string">'\u{111e1}'</span>, <span class="string">'\u{111f4}'</span>), (<span class="string">'\u{112f0}'</span>, <span class="string">'\u{112f9}'</span>),
        (<span class="string">'\u{11450}'</span>, <span class="string">'\u{11459}'</span>), (<span class="string">'\u{114d0}'</span>, <span class="string">'\u{114d9}'</span>), (<span class="string">'\u{11650}'</span>, <span class="string">'\u{11659}'</span>),
        (<span class="string">'\u{116c0}'</span>, <span class="string">'\u{116c9}'</span>), (<span class="string">'\u{11730}'</span>, <span class="string">'\u{1173b}'</span>), (<span class="string">'\u{118e0}'</span>, <span class="string">'\u{118f2}'</span>),
        (<span class="string">'\u{11950}'</span>, <span class="string">'\u{11959}'</span>), (<span class="string">'\u{11c50}'</span>, <span class="string">'\u{11c6c}'</span>), (<span class="string">'\u{11d50}'</span>, <span class="string">'\u{11d59}'</span>),
        (<span class="string">'\u{11da0}'</span>, <span class="string">'\u{11da9}'</span>), (<span class="string">'\u{11f50}'</span>, <span class="string">'\u{11f59}'</span>), (<span class="string">'\u{11fc0}'</span>, <span class="string">'\u{11fd4}'</span>),
        (<span class="string">'\u{12400}'</span>, <span class="string">'\u{1246e}'</span>), (<span class="string">'\u{16a60}'</span>, <span class="string">'\u{16a69}'</span>), (<span class="string">'\u{16ac0}'</span>, <span class="string">'\u{16ac9}'</span>),
        (<span class="string">'\u{16b50}'</span>, <span class="string">'\u{16b59}'</span>), (<span class="string">'\u{16b5b}'</span>, <span class="string">'\u{16b61}'</span>), (<span class="string">'\u{16e80}'</span>, <span class="string">'\u{16e96}'</span>),
        (<span class="string">'\u{1d2c0}'</span>, <span class="string">'\u{1d2d3}'</span>), (<span class="string">'\u{1d2e0}'</span>, <span class="string">'\u{1d2f3}'</span>), (<span class="string">'\u{1d360}'</span>, <span class="string">'\u{1d378}'</span>),
        (<span class="string">'\u{1d7ce}'</span>, <span class="string">'\u{1d7ff}'</span>), (<span class="string">'\u{1e140}'</span>, <span class="string">'\u{1e149}'</span>), (<span class="string">'\u{1e2f0}'</span>, <span class="string">'\u{1e2f9}'</span>),
        (<span class="string">'\u{1e4f0}'</span>, <span class="string">'\u{1e4f9}'</span>), (<span class="string">'\u{1e8c7}'</span>, <span class="string">'\u{1e8cf}'</span>), (<span class="string">'\u{1e950}'</span>, <span class="string">'\u{1e959}'</span>),
        (<span class="string">'\u{1ec71}'</span>, <span class="string">'\u{1ecab}'</span>), (<span class="string">'\u{1ecad}'</span>, <span class="string">'\u{1ecaf}'</span>), (<span class="string">'\u{1ecb1}'</span>, <span class="string">'\u{1ecb4}'</span>),
        (<span class="string">'\u{1ed01}'</span>, <span class="string">'\u{1ed2d}'</span>), (<span class="string">'\u{1ed2f}'</span>, <span class="string">'\u{1ed3d}'</span>), (<span class="string">'\u{1f100}'</span>, <span class="string">'\u{1f10c}'</span>),
        (<span class="string">'\u{1fbf0}'</span>, <span class="string">'\u{1fbf9}'</span>)
    ];

    <span class="attr">#[inline]
    </span><span class="kw">pub fn </span>N(c: char) -&gt; bool {
        <span class="kw">super</span>::util::bsearch_range_table(c, N_table)
    }

}

<span class="kw">mod </span>derived_property {
    <span class="kw">const </span>Alphabetic_table: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char)] = <span class="kw-2">&amp;</span>[
        (<span class="string">'\u{41}'</span>, <span class="string">'\u{5a}'</span>), (<span class="string">'\u{61}'</span>, <span class="string">'\u{7a}'</span>), (<span class="string">'\u{aa}'</span>, <span class="string">'\u{aa}'</span>), (<span class="string">'\u{b5}'</span>, <span class="string">'\u{b5}'</span>),
        (<span class="string">'\u{ba}'</span>, <span class="string">'\u{ba}'</span>), (<span class="string">'\u{c0}'</span>, <span class="string">'\u{d6}'</span>), (<span class="string">'\u{d8}'</span>, <span class="string">'\u{f6}'</span>), (<span class="string">'\u{f8}'</span>, <span class="string">'\u{2c1}'</span>),
        (<span class="string">'\u{2c6}'</span>, <span class="string">'\u{2d1}'</span>), (<span class="string">'\u{2e0}'</span>, <span class="string">'\u{2e4}'</span>), (<span class="string">'\u{2ec}'</span>, <span class="string">'\u{2ec}'</span>), (<span class="string">'\u{2ee}'</span>,
        <span class="string">'\u{2ee}'</span>), (<span class="string">'\u{345}'</span>, <span class="string">'\u{345}'</span>), (<span class="string">'\u{370}'</span>, <span class="string">'\u{374}'</span>), (<span class="string">'\u{376}'</span>, <span class="string">'\u{377}'</span>),
        (<span class="string">'\u{37a}'</span>, <span class="string">'\u{37d}'</span>), (<span class="string">'\u{37f}'</span>, <span class="string">'\u{37f}'</span>), (<span class="string">'\u{386}'</span>, <span class="string">'\u{386}'</span>), (<span class="string">'\u{388}'</span>,
        <span class="string">'\u{38a}'</span>), (<span class="string">'\u{38c}'</span>, <span class="string">'\u{38c}'</span>), (<span class="string">'\u{38e}'</span>, <span class="string">'\u{3a1}'</span>), (<span class="string">'\u{3a3}'</span>, <span class="string">'\u{3f5}'</span>),
        (<span class="string">'\u{3f7}'</span>, <span class="string">'\u{481}'</span>), (<span class="string">'\u{48a}'</span>, <span class="string">'\u{52f}'</span>), (<span class="string">'\u{531}'</span>, <span class="string">'\u{556}'</span>), (<span class="string">'\u{559}'</span>,
        <span class="string">'\u{559}'</span>), (<span class="string">'\u{560}'</span>, <span class="string">'\u{588}'</span>), (<span class="string">'\u{5b0}'</span>, <span class="string">'\u{5bd}'</span>), (<span class="string">'\u{5bf}'</span>, <span class="string">'\u{5bf}'</span>),
        (<span class="string">'\u{5c1}'</span>, <span class="string">'\u{5c2}'</span>), (<span class="string">'\u{5c4}'</span>, <span class="string">'\u{5c5}'</span>), (<span class="string">'\u{5c7}'</span>, <span class="string">'\u{5c7}'</span>), (<span class="string">'\u{5d0}'</span>,
        <span class="string">'\u{5ea}'</span>), (<span class="string">'\u{5ef}'</span>, <span class="string">'\u{5f2}'</span>), (<span class="string">'\u{610}'</span>, <span class="string">'\u{61a}'</span>), (<span class="string">'\u{620}'</span>, <span class="string">'\u{657}'</span>),
        (<span class="string">'\u{659}'</span>, <span class="string">'\u{65f}'</span>), (<span class="string">'\u{66e}'</span>, <span class="string">'\u{6d3}'</span>), (<span class="string">'\u{6d5}'</span>, <span class="string">'\u{6dc}'</span>), (<span class="string">'\u{6e1}'</span>,
        <span class="string">'\u{6e8}'</span>), (<span class="string">'\u{6ed}'</span>, <span class="string">'\u{6ef}'</span>), (<span class="string">'\u{6fa}'</span>, <span class="string">'\u{6fc}'</span>), (<span class="string">'\u{6ff}'</span>, <span class="string">'\u{6ff}'</span>),
        (<span class="string">'\u{710}'</span>, <span class="string">'\u{73f}'</span>), (<span class="string">'\u{74d}'</span>, <span class="string">'\u{7b1}'</span>), (<span class="string">'\u{7ca}'</span>, <span class="string">'\u{7ea}'</span>), (<span class="string">'\u{7f4}'</span>,
        <span class="string">'\u{7f5}'</span>), (<span class="string">'\u{7fa}'</span>, <span class="string">'\u{7fa}'</span>), (<span class="string">'\u{800}'</span>, <span class="string">'\u{817}'</span>), (<span class="string">'\u{81a}'</span>, <span class="string">'\u{82c}'</span>),
        (<span class="string">'\u{840}'</span>, <span class="string">'\u{858}'</span>), (<span class="string">'\u{860}'</span>, <span class="string">'\u{86a}'</span>), (<span class="string">'\u{870}'</span>, <span class="string">'\u{887}'</span>), (<span class="string">'\u{889}'</span>,
        <span class="string">'\u{88e}'</span>), (<span class="string">'\u{8a0}'</span>, <span class="string">'\u{8c9}'</span>), (<span class="string">'\u{8d4}'</span>, <span class="string">'\u{8df}'</span>), (<span class="string">'\u{8e3}'</span>, <span class="string">'\u{8e9}'</span>),
        (<span class="string">'\u{8f0}'</span>, <span class="string">'\u{93b}'</span>), (<span class="string">'\u{93d}'</span>, <span class="string">'\u{94c}'</span>), (<span class="string">'\u{94e}'</span>, <span class="string">'\u{950}'</span>), (<span class="string">'\u{955}'</span>,
        <span class="string">'\u{963}'</span>), (<span class="string">'\u{971}'</span>, <span class="string">'\u{983}'</span>), (<span class="string">'\u{985}'</span>, <span class="string">'\u{98c}'</span>), (<span class="string">'\u{98f}'</span>, <span class="string">'\u{990}'</span>),
        (<span class="string">'\u{993}'</span>, <span class="string">'\u{9a8}'</span>), (<span class="string">'\u{9aa}'</span>, <span class="string">'\u{9b0}'</span>), (<span class="string">'\u{9b2}'</span>, <span class="string">'\u{9b2}'</span>), (<span class="string">'\u{9b6}'</span>,
        <span class="string">'\u{9b9}'</span>), (<span class="string">'\u{9bd}'</span>, <span class="string">'\u{9c4}'</span>), (<span class="string">'\u{9c7}'</span>, <span class="string">'\u{9c8}'</span>), (<span class="string">'\u{9cb}'</span>, <span class="string">'\u{9cc}'</span>),
        (<span class="string">'\u{9ce}'</span>, <span class="string">'\u{9ce}'</span>), (<span class="string">'\u{9d7}'</span>, <span class="string">'\u{9d7}'</span>), (<span class="string">'\u{9dc}'</span>, <span class="string">'\u{9dd}'</span>), (<span class="string">'\u{9df}'</span>,
        <span class="string">'\u{9e3}'</span>), (<span class="string">'\u{9f0}'</span>, <span class="string">'\u{9f1}'</span>), (<span class="string">'\u{9fc}'</span>, <span class="string">'\u{9fc}'</span>), (<span class="string">'\u{a01}'</span>, <span class="string">'\u{a03}'</span>),
        (<span class="string">'\u{a05}'</span>, <span class="string">'\u{a0a}'</span>), (<span class="string">'\u{a0f}'</span>, <span class="string">'\u{a10}'</span>), (<span class="string">'\u{a13}'</span>, <span class="string">'\u{a28}'</span>), (<span class="string">'\u{a2a}'</span>,
        <span class="string">'\u{a30}'</span>), (<span class="string">'\u{a32}'</span>, <span class="string">'\u{a33}'</span>), (<span class="string">'\u{a35}'</span>, <span class="string">'\u{a36}'</span>), (<span class="string">'\u{a38}'</span>, <span class="string">'\u{a39}'</span>),
        (<span class="string">'\u{a3e}'</span>, <span class="string">'\u{a42}'</span>), (<span class="string">'\u{a47}'</span>, <span class="string">'\u{a48}'</span>), (<span class="string">'\u{a4b}'</span>, <span class="string">'\u{a4c}'</span>), (<span class="string">'\u{a51}'</span>,
        <span class="string">'\u{a51}'</span>), (<span class="string">'\u{a59}'</span>, <span class="string">'\u{a5c}'</span>), (<span class="string">'\u{a5e}'</span>, <span class="string">'\u{a5e}'</span>), (<span class="string">'\u{a70}'</span>, <span class="string">'\u{a75}'</span>),
        (<span class="string">'\u{a81}'</span>, <span class="string">'\u{a83}'</span>), (<span class="string">'\u{a85}'</span>, <span class="string">'\u{a8d}'</span>), (<span class="string">'\u{a8f}'</span>, <span class="string">'\u{a91}'</span>), (<span class="string">'\u{a93}'</span>,
        <span class="string">'\u{aa8}'</span>), (<span class="string">'\u{aaa}'</span>, <span class="string">'\u{ab0}'</span>), (<span class="string">'\u{ab2}'</span>, <span class="string">'\u{ab3}'</span>), (<span class="string">'\u{ab5}'</span>, <span class="string">'\u{ab9}'</span>),
        (<span class="string">'\u{abd}'</span>, <span class="string">'\u{ac5}'</span>), (<span class="string">'\u{ac7}'</span>, <span class="string">'\u{ac9}'</span>), (<span class="string">'\u{acb}'</span>, <span class="string">'\u{acc}'</span>), (<span class="string">'\u{ad0}'</span>,
        <span class="string">'\u{ad0}'</span>), (<span class="string">'\u{ae0}'</span>, <span class="string">'\u{ae3}'</span>), (<span class="string">'\u{af9}'</span>, <span class="string">'\u{afc}'</span>), (<span class="string">'\u{b01}'</span>, <span class="string">'\u{b03}'</span>),
        (<span class="string">'\u{b05}'</span>, <span class="string">'\u{b0c}'</span>), (<span class="string">'\u{b0f}'</span>, <span class="string">'\u{b10}'</span>), (<span class="string">'\u{b13}'</span>, <span class="string">'\u{b28}'</span>), (<span class="string">'\u{b2a}'</span>,
        <span class="string">'\u{b30}'</span>), (<span class="string">'\u{b32}'</span>, <span class="string">'\u{b33}'</span>), (<span class="string">'\u{b35}'</span>, <span class="string">'\u{b39}'</span>), (<span class="string">'\u{b3d}'</span>, <span class="string">'\u{b44}'</span>),
        (<span class="string">'\u{b47}'</span>, <span class="string">'\u{b48}'</span>), (<span class="string">'\u{b4b}'</span>, <span class="string">'\u{b4c}'</span>), (<span class="string">'\u{b56}'</span>, <span class="string">'\u{b57}'</span>), (<span class="string">'\u{b5c}'</span>,
        <span class="string">'\u{b5d}'</span>), (<span class="string">'\u{b5f}'</span>, <span class="string">'\u{b63}'</span>), (<span class="string">'\u{b71}'</span>, <span class="string">'\u{b71}'</span>), (<span class="string">'\u{b82}'</span>, <span class="string">'\u{b83}'</span>),
        (<span class="string">'\u{b85}'</span>, <span class="string">'\u{b8a}'</span>), (<span class="string">'\u{b8e}'</span>, <span class="string">'\u{b90}'</span>), (<span class="string">'\u{b92}'</span>, <span class="string">'\u{b95}'</span>), (<span class="string">'\u{b99}'</span>,
        <span class="string">'\u{b9a}'</span>), (<span class="string">'\u{b9c}'</span>, <span class="string">'\u{b9c}'</span>), (<span class="string">'\u{b9e}'</span>, <span class="string">'\u{b9f}'</span>), (<span class="string">'\u{ba3}'</span>, <span class="string">'\u{ba4}'</span>),
        (<span class="string">'\u{ba8}'</span>, <span class="string">'\u{baa}'</span>), (<span class="string">'\u{bae}'</span>, <span class="string">'\u{bb9}'</span>), (<span class="string">'\u{bbe}'</span>, <span class="string">'\u{bc2}'</span>), (<span class="string">'\u{bc6}'</span>,
        <span class="string">'\u{bc8}'</span>), (<span class="string">'\u{bca}'</span>, <span class="string">'\u{bcc}'</span>), (<span class="string">'\u{bd0}'</span>, <span class="string">'\u{bd0}'</span>), (<span class="string">'\u{bd7}'</span>, <span class="string">'\u{bd7}'</span>),
        (<span class="string">'\u{c00}'</span>, <span class="string">'\u{c0c}'</span>), (<span class="string">'\u{c0e}'</span>, <span class="string">'\u{c10}'</span>), (<span class="string">'\u{c12}'</span>, <span class="string">'\u{c28}'</span>), (<span class="string">'\u{c2a}'</span>,
        <span class="string">'\u{c39}'</span>), (<span class="string">'\u{c3d}'</span>, <span class="string">'\u{c44}'</span>), (<span class="string">'\u{c46}'</span>, <span class="string">'\u{c48}'</span>), (<span class="string">'\u{c4a}'</span>, <span class="string">'\u{c4c}'</span>),
        (<span class="string">'\u{c55}'</span>, <span class="string">'\u{c56}'</span>), (<span class="string">'\u{c58}'</span>, <span class="string">'\u{c5a}'</span>), (<span class="string">'\u{c5d}'</span>, <span class="string">'\u{c5d}'</span>), (<span class="string">'\u{c60}'</span>,
        <span class="string">'\u{c63}'</span>), (<span class="string">'\u{c80}'</span>, <span class="string">'\u{c83}'</span>), (<span class="string">'\u{c85}'</span>, <span class="string">'\u{c8c}'</span>), (<span class="string">'\u{c8e}'</span>, <span class="string">'\u{c90}'</span>),
        (<span class="string">'\u{c92}'</span>, <span class="string">'\u{ca8}'</span>), (<span class="string">'\u{caa}'</span>, <span class="string">'\u{cb3}'</span>), (<span class="string">'\u{cb5}'</span>, <span class="string">'\u{cb9}'</span>), (<span class="string">'\u{cbd}'</span>,
        <span class="string">'\u{cc4}'</span>), (<span class="string">'\u{cc6}'</span>, <span class="string">'\u{cc8}'</span>), (<span class="string">'\u{cca}'</span>, <span class="string">'\u{ccc}'</span>), (<span class="string">'\u{cd5}'</span>, <span class="string">'\u{cd6}'</span>),
        (<span class="string">'\u{cdd}'</span>, <span class="string">'\u{cde}'</span>), (<span class="string">'\u{ce0}'</span>, <span class="string">'\u{ce3}'</span>), (<span class="string">'\u{cf1}'</span>, <span class="string">'\u{cf3}'</span>), (<span class="string">'\u{d00}'</span>,
        <span class="string">'\u{d0c}'</span>), (<span class="string">'\u{d0e}'</span>, <span class="string">'\u{d10}'</span>), (<span class="string">'\u{d12}'</span>, <span class="string">'\u{d3a}'</span>), (<span class="string">'\u{d3d}'</span>, <span class="string">'\u{d44}'</span>),
        (<span class="string">'\u{d46}'</span>, <span class="string">'\u{d48}'</span>), (<span class="string">'\u{d4a}'</span>, <span class="string">'\u{d4c}'</span>), (<span class="string">'\u{d4e}'</span>, <span class="string">'\u{d4e}'</span>), (<span class="string">'\u{d54}'</span>,
        <span class="string">'\u{d57}'</span>), (<span class="string">'\u{d5f}'</span>, <span class="string">'\u{d63}'</span>), (<span class="string">'\u{d7a}'</span>, <span class="string">'\u{d7f}'</span>), (<span class="string">'\u{d81}'</span>, <span class="string">'\u{d83}'</span>),
        (<span class="string">'\u{d85}'</span>, <span class="string">'\u{d96}'</span>), (<span class="string">'\u{d9a}'</span>, <span class="string">'\u{db1}'</span>), (<span class="string">'\u{db3}'</span>, <span class="string">'\u{dbb}'</span>), (<span class="string">'\u{dbd}'</span>,
        <span class="string">'\u{dbd}'</span>), (<span class="string">'\u{dc0}'</span>, <span class="string">'\u{dc6}'</span>), (<span class="string">'\u{dcf}'</span>, <span class="string">'\u{dd4}'</span>), (<span class="string">'\u{dd6}'</span>, <span class="string">'\u{dd6}'</span>),
        (<span class="string">'\u{dd8}'</span>, <span class="string">'\u{ddf}'</span>), (<span class="string">'\u{df2}'</span>, <span class="string">'\u{df3}'</span>), (<span class="string">'\u{e01}'</span>, <span class="string">'\u{e3a}'</span>), (<span class="string">'\u{e40}'</span>,
        <span class="string">'\u{e46}'</span>), (<span class="string">'\u{e4d}'</span>, <span class="string">'\u{e4d}'</span>), (<span class="string">'\u{e81}'</span>, <span class="string">'\u{e82}'</span>), (<span class="string">'\u{e84}'</span>, <span class="string">'\u{e84}'</span>),
        (<span class="string">'\u{e86}'</span>, <span class="string">'\u{e8a}'</span>), (<span class="string">'\u{e8c}'</span>, <span class="string">'\u{ea3}'</span>), (<span class="string">'\u{ea5}'</span>, <span class="string">'\u{ea5}'</span>), (<span class="string">'\u{ea7}'</span>,
        <span class="string">'\u{eb9}'</span>), (<span class="string">'\u{ebb}'</span>, <span class="string">'\u{ebd}'</span>), (<span class="string">'\u{ec0}'</span>, <span class="string">'\u{ec4}'</span>), (<span class="string">'\u{ec6}'</span>, <span class="string">'\u{ec6}'</span>),
        (<span class="string">'\u{ecd}'</span>, <span class="string">'\u{ecd}'</span>), (<span class="string">'\u{edc}'</span>, <span class="string">'\u{edf}'</span>), (<span class="string">'\u{f00}'</span>, <span class="string">'\u{f00}'</span>), (<span class="string">'\u{f40}'</span>,
        <span class="string">'\u{f47}'</span>), (<span class="string">'\u{f49}'</span>, <span class="string">'\u{f6c}'</span>), (<span class="string">'\u{f71}'</span>, <span class="string">'\u{f83}'</span>), (<span class="string">'\u{f88}'</span>, <span class="string">'\u{f97}'</span>),
        (<span class="string">'\u{f99}'</span>, <span class="string">'\u{fbc}'</span>), (<span class="string">'\u{1000}'</span>, <span class="string">'\u{1036}'</span>), (<span class="string">'\u{1038}'</span>, <span class="string">'\u{1038}'</span>), (<span class="string">'\u{103b}'</span>,
        <span class="string">'\u{103f}'</span>), (<span class="string">'\u{1050}'</span>, <span class="string">'\u{108f}'</span>), (<span class="string">'\u{109a}'</span>, <span class="string">'\u{109d}'</span>), (<span class="string">'\u{10a0}'</span>, <span class="string">'\u{10c5}'</span>),
        (<span class="string">'\u{10c7}'</span>, <span class="string">'\u{10c7}'</span>), (<span class="string">'\u{10cd}'</span>, <span class="string">'\u{10cd}'</span>), (<span class="string">'\u{10d0}'</span>, <span class="string">'\u{10fa}'</span>), (<span class="string">'\u{10fc}'</span>,
        <span class="string">'\u{1248}'</span>), (<span class="string">'\u{124a}'</span>, <span class="string">'\u{124d}'</span>), (<span class="string">'\u{1250}'</span>, <span class="string">'\u{1256}'</span>), (<span class="string">'\u{1258}'</span>, <span class="string">'\u{1258}'</span>),
        (<span class="string">'\u{125a}'</span>, <span class="string">'\u{125d}'</span>), (<span class="string">'\u{1260}'</span>, <span class="string">'\u{1288}'</span>), (<span class="string">'\u{128a}'</span>, <span class="string">'\u{128d}'</span>), (<span class="string">'\u{1290}'</span>,
        <span class="string">'\u{12b0}'</span>), (<span class="string">'\u{12b2}'</span>, <span class="string">'\u{12b5}'</span>), (<span class="string">'\u{12b8}'</span>, <span class="string">'\u{12be}'</span>), (<span class="string">'\u{12c0}'</span>, <span class="string">'\u{12c0}'</span>),
        (<span class="string">'\u{12c2}'</span>, <span class="string">'\u{12c5}'</span>), (<span class="string">'\u{12c8}'</span>, <span class="string">'\u{12d6}'</span>), (<span class="string">'\u{12d8}'</span>, <span class="string">'\u{1310}'</span>), (<span class="string">'\u{1312}'</span>,
        <span class="string">'\u{1315}'</span>), (<span class="string">'\u{1318}'</span>, <span class="string">'\u{135a}'</span>), (<span class="string">'\u{1380}'</span>, <span class="string">'\u{138f}'</span>), (<span class="string">'\u{13a0}'</span>, <span class="string">'\u{13f5}'</span>),
        (<span class="string">'\u{13f8}'</span>, <span class="string">'\u{13fd}'</span>), (<span class="string">'\u{1401}'</span>, <span class="string">'\u{166c}'</span>), (<span class="string">'\u{166f}'</span>, <span class="string">'\u{167f}'</span>), (<span class="string">'\u{1681}'</span>,
        <span class="string">'\u{169a}'</span>), (<span class="string">'\u{16a0}'</span>, <span class="string">'\u{16ea}'</span>), (<span class="string">'\u{16ee}'</span>, <span class="string">'\u{16f8}'</span>), (<span class="string">'\u{1700}'</span>, <span class="string">'\u{1713}'</span>),
        (<span class="string">'\u{171f}'</span>, <span class="string">'\u{1733}'</span>), (<span class="string">'\u{1740}'</span>, <span class="string">'\u{1753}'</span>), (<span class="string">'\u{1760}'</span>, <span class="string">'\u{176c}'</span>), (<span class="string">'\u{176e}'</span>,
        <span class="string">'\u{1770}'</span>), (<span class="string">'\u{1772}'</span>, <span class="string">'\u{1773}'</span>), (<span class="string">'\u{1780}'</span>, <span class="string">'\u{17b3}'</span>), (<span class="string">'\u{17b6}'</span>, <span class="string">'\u{17c8}'</span>),
        (<span class="string">'\u{17d7}'</span>, <span class="string">'\u{17d7}'</span>), (<span class="string">'\u{17dc}'</span>, <span class="string">'\u{17dc}'</span>), (<span class="string">'\u{1820}'</span>, <span class="string">'\u{1878}'</span>), (<span class="string">'\u{1880}'</span>,
        <span class="string">'\u{18aa}'</span>), (<span class="string">'\u{18b0}'</span>, <span class="string">'\u{18f5}'</span>), (<span class="string">'\u{1900}'</span>, <span class="string">'\u{191e}'</span>), (<span class="string">'\u{1920}'</span>, <span class="string">'\u{192b}'</span>),
        (<span class="string">'\u{1930}'</span>, <span class="string">'\u{1938}'</span>), (<span class="string">'\u{1950}'</span>, <span class="string">'\u{196d}'</span>), (<span class="string">'\u{1970}'</span>, <span class="string">'\u{1974}'</span>), (<span class="string">'\u{1980}'</span>,
        <span class="string">'\u{19ab}'</span>), (<span class="string">'\u{19b0}'</span>, <span class="string">'\u{19c9}'</span>), (<span class="string">'\u{1a00}'</span>, <span class="string">'\u{1a1b}'</span>), (<span class="string">'\u{1a20}'</span>, <span class="string">'\u{1a5e}'</span>),
        (<span class="string">'\u{1a61}'</span>, <span class="string">'\u{1a74}'</span>), (<span class="string">'\u{1aa7}'</span>, <span class="string">'\u{1aa7}'</span>), (<span class="string">'\u{1abf}'</span>, <span class="string">'\u{1ac0}'</span>), (<span class="string">'\u{1acc}'</span>,
        <span class="string">'\u{1ace}'</span>), (<span class="string">'\u{1b00}'</span>, <span class="string">'\u{1b33}'</span>), (<span class="string">'\u{1b35}'</span>, <span class="string">'\u{1b43}'</span>), (<span class="string">'\u{1b45}'</span>, <span class="string">'\u{1b4c}'</span>),
        (<span class="string">'\u{1b80}'</span>, <span class="string">'\u{1ba9}'</span>), (<span class="string">'\u{1bac}'</span>, <span class="string">'\u{1baf}'</span>), (<span class="string">'\u{1bba}'</span>, <span class="string">'\u{1be5}'</span>), (<span class="string">'\u{1be7}'</span>,
        <span class="string">'\u{1bf1}'</span>), (<span class="string">'\u{1c00}'</span>, <span class="string">'\u{1c36}'</span>), (<span class="string">'\u{1c4d}'</span>, <span class="string">'\u{1c4f}'</span>), (<span class="string">'\u{1c5a}'</span>, <span class="string">'\u{1c7d}'</span>),
        (<span class="string">'\u{1c80}'</span>, <span class="string">'\u{1c88}'</span>), (<span class="string">'\u{1c90}'</span>, <span class="string">'\u{1cba}'</span>), (<span class="string">'\u{1cbd}'</span>, <span class="string">'\u{1cbf}'</span>), (<span class="string">'\u{1ce9}'</span>,
        <span class="string">'\u{1cec}'</span>), (<span class="string">'\u{1cee}'</span>, <span class="string">'\u{1cf3}'</span>), (<span class="string">'\u{1cf5}'</span>, <span class="string">'\u{1cf6}'</span>), (<span class="string">'\u{1cfa}'</span>, <span class="string">'\u{1cfa}'</span>),
        (<span class="string">'\u{1d00}'</span>, <span class="string">'\u{1dbf}'</span>), (<span class="string">'\u{1de7}'</span>, <span class="string">'\u{1df4}'</span>), (<span class="string">'\u{1e00}'</span>, <span class="string">'\u{1f15}'</span>), (<span class="string">'\u{1f18}'</span>,
        <span class="string">'\u{1f1d}'</span>), (<span class="string">'\u{1f20}'</span>, <span class="string">'\u{1f45}'</span>), (<span class="string">'\u{1f48}'</span>, <span class="string">'\u{1f4d}'</span>), (<span class="string">'\u{1f50}'</span>, <span class="string">'\u{1f57}'</span>),
        (<span class="string">'\u{1f59}'</span>, <span class="string">'\u{1f59}'</span>), (<span class="string">'\u{1f5b}'</span>, <span class="string">'\u{1f5b}'</span>), (<span class="string">'\u{1f5d}'</span>, <span class="string">'\u{1f5d}'</span>), (<span class="string">'\u{1f5f}'</span>,
        <span class="string">'\u{1f7d}'</span>), (<span class="string">'\u{1f80}'</span>, <span class="string">'\u{1fb4}'</span>), (<span class="string">'\u{1fb6}'</span>, <span class="string">'\u{1fbc}'</span>), (<span class="string">'\u{1fbe}'</span>, <span class="string">'\u{1fbe}'</span>),
        (<span class="string">'\u{1fc2}'</span>, <span class="string">'\u{1fc4}'</span>), (<span class="string">'\u{1fc6}'</span>, <span class="string">'\u{1fcc}'</span>), (<span class="string">'\u{1fd0}'</span>, <span class="string">'\u{1fd3}'</span>), (<span class="string">'\u{1fd6}'</span>,
        <span class="string">'\u{1fdb}'</span>), (<span class="string">'\u{1fe0}'</span>, <span class="string">'\u{1fec}'</span>), (<span class="string">'\u{1ff2}'</span>, <span class="string">'\u{1ff4}'</span>), (<span class="string">'\u{1ff6}'</span>, <span class="string">'\u{1ffc}'</span>),
        (<span class="string">'\u{2071}'</span>, <span class="string">'\u{2071}'</span>), (<span class="string">'\u{207f}'</span>, <span class="string">'\u{207f}'</span>), (<span class="string">'\u{2090}'</span>, <span class="string">'\u{209c}'</span>), (<span class="string">'\u{2102}'</span>,
        <span class="string">'\u{2102}'</span>), (<span class="string">'\u{2107}'</span>, <span class="string">'\u{2107}'</span>), (<span class="string">'\u{210a}'</span>, <span class="string">'\u{2113}'</span>), (<span class="string">'\u{2115}'</span>, <span class="string">'\u{2115}'</span>),
        (<span class="string">'\u{2119}'</span>, <span class="string">'\u{211d}'</span>), (<span class="string">'\u{2124}'</span>, <span class="string">'\u{2124}'</span>), (<span class="string">'\u{2126}'</span>, <span class="string">'\u{2126}'</span>), (<span class="string">'\u{2128}'</span>,
        <span class="string">'\u{2128}'</span>), (<span class="string">'\u{212a}'</span>, <span class="string">'\u{212d}'</span>), (<span class="string">'\u{212f}'</span>, <span class="string">'\u{2139}'</span>), (<span class="string">'\u{213c}'</span>, <span class="string">'\u{213f}'</span>),
        (<span class="string">'\u{2145}'</span>, <span class="string">'\u{2149}'</span>), (<span class="string">'\u{214e}'</span>, <span class="string">'\u{214e}'</span>), (<span class="string">'\u{2160}'</span>, <span class="string">'\u{2188}'</span>), (<span class="string">'\u{24b6}'</span>,
        <span class="string">'\u{24e9}'</span>), (<span class="string">'\u{2c00}'</span>, <span class="string">'\u{2ce4}'</span>), (<span class="string">'\u{2ceb}'</span>, <span class="string">'\u{2cee}'</span>), (<span class="string">'\u{2cf2}'</span>, <span class="string">'\u{2cf3}'</span>),
        (<span class="string">'\u{2d00}'</span>, <span class="string">'\u{2d25}'</span>), (<span class="string">'\u{2d27}'</span>, <span class="string">'\u{2d27}'</span>), (<span class="string">'\u{2d2d}'</span>, <span class="string">'\u{2d2d}'</span>), (<span class="string">'\u{2d30}'</span>,
        <span class="string">'\u{2d67}'</span>), (<span class="string">'\u{2d6f}'</span>, <span class="string">'\u{2d6f}'</span>), (<span class="string">'\u{2d80}'</span>, <span class="string">'\u{2d96}'</span>), (<span class="string">'\u{2da0}'</span>, <span class="string">'\u{2da6}'</span>),
        (<span class="string">'\u{2da8}'</span>, <span class="string">'\u{2dae}'</span>), (<span class="string">'\u{2db0}'</span>, <span class="string">'\u{2db6}'</span>), (<span class="string">'\u{2db8}'</span>, <span class="string">'\u{2dbe}'</span>), (<span class="string">'\u{2dc0}'</span>,
        <span class="string">'\u{2dc6}'</span>), (<span class="string">'\u{2dc8}'</span>, <span class="string">'\u{2dce}'</span>), (<span class="string">'\u{2dd0}'</span>, <span class="string">'\u{2dd6}'</span>), (<span class="string">'\u{2dd8}'</span>, <span class="string">'\u{2dde}'</span>),
        (<span class="string">'\u{2de0}'</span>, <span class="string">'\u{2dff}'</span>), (<span class="string">'\u{2e2f}'</span>, <span class="string">'\u{2e2f}'</span>), (<span class="string">'\u{3005}'</span>, <span class="string">'\u{3007}'</span>), (<span class="string">'\u{3021}'</span>,
        <span class="string">'\u{3029}'</span>), (<span class="string">'\u{3031}'</span>, <span class="string">'\u{3035}'</span>), (<span class="string">'\u{3038}'</span>, <span class="string">'\u{303c}'</span>), (<span class="string">'\u{3041}'</span>, <span class="string">'\u{3096}'</span>),
        (<span class="string">'\u{309d}'</span>, <span class="string">'\u{309f}'</span>), (<span class="string">'\u{30a1}'</span>, <span class="string">'\u{30fa}'</span>), (<span class="string">'\u{30fc}'</span>, <span class="string">'\u{30ff}'</span>), (<span class="string">'\u{3105}'</span>,
        <span class="string">'\u{312f}'</span>), (<span class="string">'\u{3131}'</span>, <span class="string">'\u{318e}'</span>), (<span class="string">'\u{31a0}'</span>, <span class="string">'\u{31bf}'</span>), (<span class="string">'\u{31f0}'</span>, <span class="string">'\u{31ff}'</span>),
        (<span class="string">'\u{3400}'</span>, <span class="string">'\u{4dbf}'</span>), (<span class="string">'\u{4e00}'</span>, <span class="string">'\u{a48c}'</span>), (<span class="string">'\u{a4d0}'</span>, <span class="string">'\u{a4fd}'</span>), (<span class="string">'\u{a500}'</span>,
        <span class="string">'\u{a60c}'</span>), (<span class="string">'\u{a610}'</span>, <span class="string">'\u{a61f}'</span>), (<span class="string">'\u{a62a}'</span>, <span class="string">'\u{a62b}'</span>), (<span class="string">'\u{a640}'</span>, <span class="string">'\u{a66e}'</span>),
        (<span class="string">'\u{a674}'</span>, <span class="string">'\u{a67b}'</span>), (<span class="string">'\u{a67f}'</span>, <span class="string">'\u{a6ef}'</span>), (<span class="string">'\u{a717}'</span>, <span class="string">'\u{a71f}'</span>), (<span class="string">'\u{a722}'</span>,
        <span class="string">'\u{a788}'</span>), (<span class="string">'\u{a78b}'</span>, <span class="string">'\u{a7ca}'</span>), (<span class="string">'\u{a7d0}'</span>, <span class="string">'\u{a7d1}'</span>), (<span class="string">'\u{a7d3}'</span>, <span class="string">'\u{a7d3}'</span>),
        (<span class="string">'\u{a7d5}'</span>, <span class="string">'\u{a7d9}'</span>), (<span class="string">'\u{a7f2}'</span>, <span class="string">'\u{a805}'</span>), (<span class="string">'\u{a807}'</span>, <span class="string">'\u{a827}'</span>), (<span class="string">'\u{a840}'</span>,
        <span class="string">'\u{a873}'</span>), (<span class="string">'\u{a880}'</span>, <span class="string">'\u{a8c3}'</span>), (<span class="string">'\u{a8c5}'</span>, <span class="string">'\u{a8c5}'</span>), (<span class="string">'\u{a8f2}'</span>, <span class="string">'\u{a8f7}'</span>),
        (<span class="string">'\u{a8fb}'</span>, <span class="string">'\u{a8fb}'</span>), (<span class="string">'\u{a8fd}'</span>, <span class="string">'\u{a8ff}'</span>), (<span class="string">'\u{a90a}'</span>, <span class="string">'\u{a92a}'</span>), (<span class="string">'\u{a930}'</span>,
        <span class="string">'\u{a952}'</span>), (<span class="string">'\u{a960}'</span>, <span class="string">'\u{a97c}'</span>), (<span class="string">'\u{a980}'</span>, <span class="string">'\u{a9b2}'</span>), (<span class="string">'\u{a9b4}'</span>, <span class="string">'\u{a9bf}'</span>),
        (<span class="string">'\u{a9cf}'</span>, <span class="string">'\u{a9cf}'</span>), (<span class="string">'\u{a9e0}'</span>, <span class="string">'\u{a9ef}'</span>), (<span class="string">'\u{a9fa}'</span>, <span class="string">'\u{a9fe}'</span>), (<span class="string">'\u{aa00}'</span>,
        <span class="string">'\u{aa36}'</span>), (<span class="string">'\u{aa40}'</span>, <span class="string">'\u{aa4d}'</span>), (<span class="string">'\u{aa60}'</span>, <span class="string">'\u{aa76}'</span>), (<span class="string">'\u{aa7a}'</span>, <span class="string">'\u{aabe}'</span>),
        (<span class="string">'\u{aac0}'</span>, <span class="string">'\u{aac0}'</span>), (<span class="string">'\u{aac2}'</span>, <span class="string">'\u{aac2}'</span>), (<span class="string">'\u{aadb}'</span>, <span class="string">'\u{aadd}'</span>), (<span class="string">'\u{aae0}'</span>,
        <span class="string">'\u{aaef}'</span>), (<span class="string">'\u{aaf2}'</span>, <span class="string">'\u{aaf5}'</span>), (<span class="string">'\u{ab01}'</span>, <span class="string">'\u{ab06}'</span>), (<span class="string">'\u{ab09}'</span>, <span class="string">'\u{ab0e}'</span>),
        (<span class="string">'\u{ab11}'</span>, <span class="string">'\u{ab16}'</span>), (<span class="string">'\u{ab20}'</span>, <span class="string">'\u{ab26}'</span>), (<span class="string">'\u{ab28}'</span>, <span class="string">'\u{ab2e}'</span>), (<span class="string">'\u{ab30}'</span>,
        <span class="string">'\u{ab5a}'</span>), (<span class="string">'\u{ab5c}'</span>, <span class="string">'\u{ab69}'</span>), (<span class="string">'\u{ab70}'</span>, <span class="string">'\u{abea}'</span>), (<span class="string">'\u{ac00}'</span>, <span class="string">'\u{d7a3}'</span>),
        (<span class="string">'\u{d7b0}'</span>, <span class="string">'\u{d7c6}'</span>), (<span class="string">'\u{d7cb}'</span>, <span class="string">'\u{d7fb}'</span>), (<span class="string">'\u{f900}'</span>, <span class="string">'\u{fa6d}'</span>), (<span class="string">'\u{fa70}'</span>,
        <span class="string">'\u{fad9}'</span>), (<span class="string">'\u{fb00}'</span>, <span class="string">'\u{fb06}'</span>), (<span class="string">'\u{fb13}'</span>, <span class="string">'\u{fb17}'</span>), (<span class="string">'\u{fb1d}'</span>, <span class="string">'\u{fb28}'</span>),
        (<span class="string">'\u{fb2a}'</span>, <span class="string">'\u{fb36}'</span>), (<span class="string">'\u{fb38}'</span>, <span class="string">'\u{fb3c}'</span>), (<span class="string">'\u{fb3e}'</span>, <span class="string">'\u{fb3e}'</span>), (<span class="string">'\u{fb40}'</span>,
        <span class="string">'\u{fb41}'</span>), (<span class="string">'\u{fb43}'</span>, <span class="string">'\u{fb44}'</span>), (<span class="string">'\u{fb46}'</span>, <span class="string">'\u{fbb1}'</span>), (<span class="string">'\u{fbd3}'</span>, <span class="string">'\u{fd3d}'</span>),
        (<span class="string">'\u{fd50}'</span>, <span class="string">'\u{fd8f}'</span>), (<span class="string">'\u{fd92}'</span>, <span class="string">'\u{fdc7}'</span>), (<span class="string">'\u{fdf0}'</span>, <span class="string">'\u{fdfb}'</span>), (<span class="string">'\u{fe70}'</span>,
        <span class="string">'\u{fe74}'</span>), (<span class="string">'\u{fe76}'</span>, <span class="string">'\u{fefc}'</span>), (<span class="string">'\u{ff21}'</span>, <span class="string">'\u{ff3a}'</span>), (<span class="string">'\u{ff41}'</span>, <span class="string">'\u{ff5a}'</span>),
        (<span class="string">'\u{ff66}'</span>, <span class="string">'\u{ffbe}'</span>), (<span class="string">'\u{ffc2}'</span>, <span class="string">'\u{ffc7}'</span>), (<span class="string">'\u{ffca}'</span>, <span class="string">'\u{ffcf}'</span>), (<span class="string">'\u{ffd2}'</span>,
        <span class="string">'\u{ffd7}'</span>), (<span class="string">'\u{ffda}'</span>, <span class="string">'\u{ffdc}'</span>), (<span class="string">'\u{10000}'</span>, <span class="string">'\u{1000b}'</span>), (<span class="string">'\u{1000d}'</span>,
        <span class="string">'\u{10026}'</span>), (<span class="string">'\u{10028}'</span>, <span class="string">'\u{1003a}'</span>), (<span class="string">'\u{1003c}'</span>, <span class="string">'\u{1003d}'</span>), (<span class="string">'\u{1003f}'</span>,
        <span class="string">'\u{1004d}'</span>), (<span class="string">'\u{10050}'</span>, <span class="string">'\u{1005d}'</span>), (<span class="string">'\u{10080}'</span>, <span class="string">'\u{100fa}'</span>), (<span class="string">'\u{10140}'</span>,
        <span class="string">'\u{10174}'</span>), (<span class="string">'\u{10280}'</span>, <span class="string">'\u{1029c}'</span>), (<span class="string">'\u{102a0}'</span>, <span class="string">'\u{102d0}'</span>), (<span class="string">'\u{10300}'</span>,
        <span class="string">'\u{1031f}'</span>), (<span class="string">'\u{1032d}'</span>, <span class="string">'\u{1034a}'</span>), (<span class="string">'\u{10350}'</span>, <span class="string">'\u{1037a}'</span>), (<span class="string">'\u{10380}'</span>,
        <span class="string">'\u{1039d}'</span>), (<span class="string">'\u{103a0}'</span>, <span class="string">'\u{103c3}'</span>), (<span class="string">'\u{103c8}'</span>, <span class="string">'\u{103cf}'</span>), (<span class="string">'\u{103d1}'</span>,
        <span class="string">'\u{103d5}'</span>), (<span class="string">'\u{10400}'</span>, <span class="string">'\u{1049d}'</span>), (<span class="string">'\u{104b0}'</span>, <span class="string">'\u{104d3}'</span>), (<span class="string">'\u{104d8}'</span>,
        <span class="string">'\u{104fb}'</span>), (<span class="string">'\u{10500}'</span>, <span class="string">'\u{10527}'</span>), (<span class="string">'\u{10530}'</span>, <span class="string">'\u{10563}'</span>), (<span class="string">'\u{10570}'</span>,
        <span class="string">'\u{1057a}'</span>), (<span class="string">'\u{1057c}'</span>, <span class="string">'\u{1058a}'</span>), (<span class="string">'\u{1058c}'</span>, <span class="string">'\u{10592}'</span>), (<span class="string">'\u{10594}'</span>,
        <span class="string">'\u{10595}'</span>), (<span class="string">'\u{10597}'</span>, <span class="string">'\u{105a1}'</span>), (<span class="string">'\u{105a3}'</span>, <span class="string">'\u{105b1}'</span>), (<span class="string">'\u{105b3}'</span>,
        <span class="string">'\u{105b9}'</span>), (<span class="string">'\u{105bb}'</span>, <span class="string">'\u{105bc}'</span>), (<span class="string">'\u{10600}'</span>, <span class="string">'\u{10736}'</span>), (<span class="string">'\u{10740}'</span>,
        <span class="string">'\u{10755}'</span>), (<span class="string">'\u{10760}'</span>, <span class="string">'\u{10767}'</span>), (<span class="string">'\u{10780}'</span>, <span class="string">'\u{10785}'</span>), (<span class="string">'\u{10787}'</span>,
        <span class="string">'\u{107b0}'</span>), (<span class="string">'\u{107b2}'</span>, <span class="string">'\u{107ba}'</span>), (<span class="string">'\u{10800}'</span>, <span class="string">'\u{10805}'</span>), (<span class="string">'\u{10808}'</span>,
        <span class="string">'\u{10808}'</span>), (<span class="string">'\u{1080a}'</span>, <span class="string">'\u{10835}'</span>), (<span class="string">'\u{10837}'</span>, <span class="string">'\u{10838}'</span>), (<span class="string">'\u{1083c}'</span>,
        <span class="string">'\u{1083c}'</span>), (<span class="string">'\u{1083f}'</span>, <span class="string">'\u{10855}'</span>), (<span class="string">'\u{10860}'</span>, <span class="string">'\u{10876}'</span>), (<span class="string">'\u{10880}'</span>,
        <span class="string">'\u{1089e}'</span>), (<span class="string">'\u{108e0}'</span>, <span class="string">'\u{108f2}'</span>), (<span class="string">'\u{108f4}'</span>, <span class="string">'\u{108f5}'</span>), (<span class="string">'\u{10900}'</span>,
        <span class="string">'\u{10915}'</span>), (<span class="string">'\u{10920}'</span>, <span class="string">'\u{10939}'</span>), (<span class="string">'\u{10980}'</span>, <span class="string">'\u{109b7}'</span>), (<span class="string">'\u{109be}'</span>,
        <span class="string">'\u{109bf}'</span>), (<span class="string">'\u{10a00}'</span>, <span class="string">'\u{10a03}'</span>), (<span class="string">'\u{10a05}'</span>, <span class="string">'\u{10a06}'</span>), (<span class="string">'\u{10a0c}'</span>,
        <span class="string">'\u{10a13}'</span>), (<span class="string">'\u{10a15}'</span>, <span class="string">'\u{10a17}'</span>), (<span class="string">'\u{10a19}'</span>, <span class="string">'\u{10a35}'</span>), (<span class="string">'\u{10a60}'</span>,
        <span class="string">'\u{10a7c}'</span>), (<span class="string">'\u{10a80}'</span>, <span class="string">'\u{10a9c}'</span>), (<span class="string">'\u{10ac0}'</span>, <span class="string">'\u{10ac7}'</span>), (<span class="string">'\u{10ac9}'</span>,
        <span class="string">'\u{10ae4}'</span>), (<span class="string">'\u{10b00}'</span>, <span class="string">'\u{10b35}'</span>), (<span class="string">'\u{10b40}'</span>, <span class="string">'\u{10b55}'</span>), (<span class="string">'\u{10b60}'</span>,
        <span class="string">'\u{10b72}'</span>), (<span class="string">'\u{10b80}'</span>, <span class="string">'\u{10b91}'</span>), (<span class="string">'\u{10c00}'</span>, <span class="string">'\u{10c48}'</span>), (<span class="string">'\u{10c80}'</span>,
        <span class="string">'\u{10cb2}'</span>), (<span class="string">'\u{10cc0}'</span>, <span class="string">'\u{10cf2}'</span>), (<span class="string">'\u{10d00}'</span>, <span class="string">'\u{10d27}'</span>), (<span class="string">'\u{10e80}'</span>,
        <span class="string">'\u{10ea9}'</span>), (<span class="string">'\u{10eab}'</span>, <span class="string">'\u{10eac}'</span>), (<span class="string">'\u{10eb0}'</span>, <span class="string">'\u{10eb1}'</span>), (<span class="string">'\u{10f00}'</span>,
        <span class="string">'\u{10f1c}'</span>), (<span class="string">'\u{10f27}'</span>, <span class="string">'\u{10f27}'</span>), (<span class="string">'\u{10f30}'</span>, <span class="string">'\u{10f45}'</span>), (<span class="string">'\u{10f70}'</span>,
        <span class="string">'\u{10f81}'</span>), (<span class="string">'\u{10fb0}'</span>, <span class="string">'\u{10fc4}'</span>), (<span class="string">'\u{10fe0}'</span>, <span class="string">'\u{10ff6}'</span>), (<span class="string">'\u{11000}'</span>,
        <span class="string">'\u{11045}'</span>), (<span class="string">'\u{11071}'</span>, <span class="string">'\u{11075}'</span>), (<span class="string">'\u{11080}'</span>, <span class="string">'\u{110b8}'</span>), (<span class="string">'\u{110c2}'</span>,
        <span class="string">'\u{110c2}'</span>), (<span class="string">'\u{110d0}'</span>, <span class="string">'\u{110e8}'</span>), (<span class="string">'\u{11100}'</span>, <span class="string">'\u{11132}'</span>), (<span class="string">'\u{11144}'</span>,
        <span class="string">'\u{11147}'</span>), (<span class="string">'\u{11150}'</span>, <span class="string">'\u{11172}'</span>), (<span class="string">'\u{11176}'</span>, <span class="string">'\u{11176}'</span>), (<span class="string">'\u{11180}'</span>,
        <span class="string">'\u{111bf}'</span>), (<span class="string">'\u{111c1}'</span>, <span class="string">'\u{111c4}'</span>), (<span class="string">'\u{111ce}'</span>, <span class="string">'\u{111cf}'</span>), (<span class="string">'\u{111da}'</span>,
        <span class="string">'\u{111da}'</span>), (<span class="string">'\u{111dc}'</span>, <span class="string">'\u{111dc}'</span>), (<span class="string">'\u{11200}'</span>, <span class="string">'\u{11211}'</span>), (<span class="string">'\u{11213}'</span>,
        <span class="string">'\u{11234}'</span>), (<span class="string">'\u{11237}'</span>, <span class="string">'\u{11237}'</span>), (<span class="string">'\u{1123e}'</span>, <span class="string">'\u{11241}'</span>), (<span class="string">'\u{11280}'</span>,
        <span class="string">'\u{11286}'</span>), (<span class="string">'\u{11288}'</span>, <span class="string">'\u{11288}'</span>), (<span class="string">'\u{1128a}'</span>, <span class="string">'\u{1128d}'</span>), (<span class="string">'\u{1128f}'</span>,
        <span class="string">'\u{1129d}'</span>), (<span class="string">'\u{1129f}'</span>, <span class="string">'\u{112a8}'</span>), (<span class="string">'\u{112b0}'</span>, <span class="string">'\u{112e8}'</span>), (<span class="string">'\u{11300}'</span>,
        <span class="string">'\u{11303}'</span>), (<span class="string">'\u{11305}'</span>, <span class="string">'\u{1130c}'</span>), (<span class="string">'\u{1130f}'</span>, <span class="string">'\u{11310}'</span>), (<span class="string">'\u{11313}'</span>,
        <span class="string">'\u{11328}'</span>), (<span class="string">'\u{1132a}'</span>, <span class="string">'\u{11330}'</span>), (<span class="string">'\u{11332}'</span>, <span class="string">'\u{11333}'</span>), (<span class="string">'\u{11335}'</span>,
        <span class="string">'\u{11339}'</span>), (<span class="string">'\u{1133d}'</span>, <span class="string">'\u{11344}'</span>), (<span class="string">'\u{11347}'</span>, <span class="string">'\u{11348}'</span>), (<span class="string">'\u{1134b}'</span>,
        <span class="string">'\u{1134c}'</span>), (<span class="string">'\u{11350}'</span>, <span class="string">'\u{11350}'</span>), (<span class="string">'\u{11357}'</span>, <span class="string">'\u{11357}'</span>), (<span class="string">'\u{1135d}'</span>,
        <span class="string">'\u{11363}'</span>), (<span class="string">'\u{11400}'</span>, <span class="string">'\u{11441}'</span>), (<span class="string">'\u{11443}'</span>, <span class="string">'\u{11445}'</span>), (<span class="string">'\u{11447}'</span>,
        <span class="string">'\u{1144a}'</span>), (<span class="string">'\u{1145f}'</span>, <span class="string">'\u{11461}'</span>), (<span class="string">'\u{11480}'</span>, <span class="string">'\u{114c1}'</span>), (<span class="string">'\u{114c4}'</span>,
        <span class="string">'\u{114c5}'</span>), (<span class="string">'\u{114c7}'</span>, <span class="string">'\u{114c7}'</span>), (<span class="string">'\u{11580}'</span>, <span class="string">'\u{115b5}'</span>), (<span class="string">'\u{115b8}'</span>,
        <span class="string">'\u{115be}'</span>), (<span class="string">'\u{115d8}'</span>, <span class="string">'\u{115dd}'</span>), (<span class="string">'\u{11600}'</span>, <span class="string">'\u{1163e}'</span>), (<span class="string">'\u{11640}'</span>,
        <span class="string">'\u{11640}'</span>), (<span class="string">'\u{11644}'</span>, <span class="string">'\u{11644}'</span>), (<span class="string">'\u{11680}'</span>, <span class="string">'\u{116b5}'</span>), (<span class="string">'\u{116b8}'</span>,
        <span class="string">'\u{116b8}'</span>), (<span class="string">'\u{11700}'</span>, <span class="string">'\u{1171a}'</span>), (<span class="string">'\u{1171d}'</span>, <span class="string">'\u{1172a}'</span>), (<span class="string">'\u{11740}'</span>,
        <span class="string">'\u{11746}'</span>), (<span class="string">'\u{11800}'</span>, <span class="string">'\u{11838}'</span>), (<span class="string">'\u{118a0}'</span>, <span class="string">'\u{118df}'</span>), (<span class="string">'\u{118ff}'</span>,
        <span class="string">'\u{11906}'</span>), (<span class="string">'\u{11909}'</span>, <span class="string">'\u{11909}'</span>), (<span class="string">'\u{1190c}'</span>, <span class="string">'\u{11913}'</span>), (<span class="string">'\u{11915}'</span>,
        <span class="string">'\u{11916}'</span>), (<span class="string">'\u{11918}'</span>, <span class="string">'\u{11935}'</span>), (<span class="string">'\u{11937}'</span>, <span class="string">'\u{11938}'</span>), (<span class="string">'\u{1193b}'</span>,
        <span class="string">'\u{1193c}'</span>), (<span class="string">'\u{1193f}'</span>, <span class="string">'\u{11942}'</span>), (<span class="string">'\u{119a0}'</span>, <span class="string">'\u{119a7}'</span>), (<span class="string">'\u{119aa}'</span>,
        <span class="string">'\u{119d7}'</span>), (<span class="string">'\u{119da}'</span>, <span class="string">'\u{119df}'</span>), (<span class="string">'\u{119e1}'</span>, <span class="string">'\u{119e1}'</span>), (<span class="string">'\u{119e3}'</span>,
        <span class="string">'\u{119e4}'</span>), (<span class="string">'\u{11a00}'</span>, <span class="string">'\u{11a32}'</span>), (<span class="string">'\u{11a35}'</span>, <span class="string">'\u{11a3e}'</span>), (<span class="string">'\u{11a50}'</span>,
        <span class="string">'\u{11a97}'</span>), (<span class="string">'\u{11a9d}'</span>, <span class="string">'\u{11a9d}'</span>), (<span class="string">'\u{11ab0}'</span>, <span class="string">'\u{11af8}'</span>), (<span class="string">'\u{11c00}'</span>,
        <span class="string">'\u{11c08}'</span>), (<span class="string">'\u{11c0a}'</span>, <span class="string">'\u{11c36}'</span>), (<span class="string">'\u{11c38}'</span>, <span class="string">'\u{11c3e}'</span>), (<span class="string">'\u{11c40}'</span>,
        <span class="string">'\u{11c40}'</span>), (<span class="string">'\u{11c72}'</span>, <span class="string">'\u{11c8f}'</span>), (<span class="string">'\u{11c92}'</span>, <span class="string">'\u{11ca7}'</span>), (<span class="string">'\u{11ca9}'</span>,
        <span class="string">'\u{11cb6}'</span>), (<span class="string">'\u{11d00}'</span>, <span class="string">'\u{11d06}'</span>), (<span class="string">'\u{11d08}'</span>, <span class="string">'\u{11d09}'</span>), (<span class="string">'\u{11d0b}'</span>,
        <span class="string">'\u{11d36}'</span>), (<span class="string">'\u{11d3a}'</span>, <span class="string">'\u{11d3a}'</span>), (<span class="string">'\u{11d3c}'</span>, <span class="string">'\u{11d3d}'</span>), (<span class="string">'\u{11d3f}'</span>,
        <span class="string">'\u{11d41}'</span>), (<span class="string">'\u{11d43}'</span>, <span class="string">'\u{11d43}'</span>), (<span class="string">'\u{11d46}'</span>, <span class="string">'\u{11d47}'</span>), (<span class="string">'\u{11d60}'</span>,
        <span class="string">'\u{11d65}'</span>), (<span class="string">'\u{11d67}'</span>, <span class="string">'\u{11d68}'</span>), (<span class="string">'\u{11d6a}'</span>, <span class="string">'\u{11d8e}'</span>), (<span class="string">'\u{11d90}'</span>,
        <span class="string">'\u{11d91}'</span>), (<span class="string">'\u{11d93}'</span>, <span class="string">'\u{11d96}'</span>), (<span class="string">'\u{11d98}'</span>, <span class="string">'\u{11d98}'</span>), (<span class="string">'\u{11ee0}'</span>,
        <span class="string">'\u{11ef6}'</span>), (<span class="string">'\u{11f00}'</span>, <span class="string">'\u{11f10}'</span>), (<span class="string">'\u{11f12}'</span>, <span class="string">'\u{11f3a}'</span>), (<span class="string">'\u{11f3e}'</span>,
        <span class="string">'\u{11f40}'</span>), (<span class="string">'\u{11fb0}'</span>, <span class="string">'\u{11fb0}'</span>), (<span class="string">'\u{12000}'</span>, <span class="string">'\u{12399}'</span>), (<span class="string">'\u{12400}'</span>,
        <span class="string">'\u{1246e}'</span>), (<span class="string">'\u{12480}'</span>, <span class="string">'\u{12543}'</span>), (<span class="string">'\u{12f90}'</span>, <span class="string">'\u{12ff0}'</span>), (<span class="string">'\u{13000}'</span>,
        <span class="string">'\u{1342f}'</span>), (<span class="string">'\u{13441}'</span>, <span class="string">'\u{13446}'</span>), (<span class="string">'\u{14400}'</span>, <span class="string">'\u{14646}'</span>), (<span class="string">'\u{16800}'</span>,
        <span class="string">'\u{16a38}'</span>), (<span class="string">'\u{16a40}'</span>, <span class="string">'\u{16a5e}'</span>), (<span class="string">'\u{16a70}'</span>, <span class="string">'\u{16abe}'</span>), (<span class="string">'\u{16ad0}'</span>,
        <span class="string">'\u{16aed}'</span>), (<span class="string">'\u{16b00}'</span>, <span class="string">'\u{16b2f}'</span>), (<span class="string">'\u{16b40}'</span>, <span class="string">'\u{16b43}'</span>), (<span class="string">'\u{16b63}'</span>,
        <span class="string">'\u{16b77}'</span>), (<span class="string">'\u{16b7d}'</span>, <span class="string">'\u{16b8f}'</span>), (<span class="string">'\u{16e40}'</span>, <span class="string">'\u{16e7f}'</span>), (<span class="string">'\u{16f00}'</span>,
        <span class="string">'\u{16f4a}'</span>), (<span class="string">'\u{16f4f}'</span>, <span class="string">'\u{16f87}'</span>), (<span class="string">'\u{16f8f}'</span>, <span class="string">'\u{16f9f}'</span>), (<span class="string">'\u{16fe0}'</span>,
        <span class="string">'\u{16fe1}'</span>), (<span class="string">'\u{16fe3}'</span>, <span class="string">'\u{16fe3}'</span>), (<span class="string">'\u{16ff0}'</span>, <span class="string">'\u{16ff1}'</span>), (<span class="string">'\u{17000}'</span>,
        <span class="string">'\u{187f7}'</span>), (<span class="string">'\u{18800}'</span>, <span class="string">'\u{18cd5}'</span>), (<span class="string">'\u{18d00}'</span>, <span class="string">'\u{18d08}'</span>), (<span class="string">'\u{1aff0}'</span>,
        <span class="string">'\u{1aff3}'</span>), (<span class="string">'\u{1aff5}'</span>, <span class="string">'\u{1affb}'</span>), (<span class="string">'\u{1affd}'</span>, <span class="string">'\u{1affe}'</span>), (<span class="string">'\u{1b000}'</span>,
        <span class="string">'\u{1b122}'</span>), (<span class="string">'\u{1b132}'</span>, <span class="string">'\u{1b132}'</span>), (<span class="string">'\u{1b150}'</span>, <span class="string">'\u{1b152}'</span>), (<span class="string">'\u{1b155}'</span>,
        <span class="string">'\u{1b155}'</span>), (<span class="string">'\u{1b164}'</span>, <span class="string">'\u{1b167}'</span>), (<span class="string">'\u{1b170}'</span>, <span class="string">'\u{1b2fb}'</span>), (<span class="string">'\u{1bc00}'</span>,
        <span class="string">'\u{1bc6a}'</span>), (<span class="string">'\u{1bc70}'</span>, <span class="string">'\u{1bc7c}'</span>), (<span class="string">'\u{1bc80}'</span>, <span class="string">'\u{1bc88}'</span>), (<span class="string">'\u{1bc90}'</span>,
        <span class="string">'\u{1bc99}'</span>), (<span class="string">'\u{1bc9e}'</span>, <span class="string">'\u{1bc9e}'</span>), (<span class="string">'\u{1d400}'</span>, <span class="string">'\u{1d454}'</span>), (<span class="string">'\u{1d456}'</span>,
        <span class="string">'\u{1d49c}'</span>), (<span class="string">'\u{1d49e}'</span>, <span class="string">'\u{1d49f}'</span>), (<span class="string">'\u{1d4a2}'</span>, <span class="string">'\u{1d4a2}'</span>), (<span class="string">'\u{1d4a5}'</span>,
        <span class="string">'\u{1d4a6}'</span>), (<span class="string">'\u{1d4a9}'</span>, <span class="string">'\u{1d4ac}'</span>), (<span class="string">'\u{1d4ae}'</span>, <span class="string">'\u{1d4b9}'</span>), (<span class="string">'\u{1d4bb}'</span>,
        <span class="string">'\u{1d4bb}'</span>), (<span class="string">'\u{1d4bd}'</span>, <span class="string">'\u{1d4c3}'</span>), (<span class="string">'\u{1d4c5}'</span>, <span class="string">'\u{1d505}'</span>), (<span class="string">'\u{1d507}'</span>,
        <span class="string">'\u{1d50a}'</span>), (<span class="string">'\u{1d50d}'</span>, <span class="string">'\u{1d514}'</span>), (<span class="string">'\u{1d516}'</span>, <span class="string">'\u{1d51c}'</span>), (<span class="string">'\u{1d51e}'</span>,
        <span class="string">'\u{1d539}'</span>), (<span class="string">'\u{1d53b}'</span>, <span class="string">'\u{1d53e}'</span>), (<span class="string">'\u{1d540}'</span>, <span class="string">'\u{1d544}'</span>), (<span class="string">'\u{1d546}'</span>,
        <span class="string">'\u{1d546}'</span>), (<span class="string">'\u{1d54a}'</span>, <span class="string">'\u{1d550}'</span>), (<span class="string">'\u{1d552}'</span>, <span class="string">'\u{1d6a5}'</span>), (<span class="string">'\u{1d6a8}'</span>,
        <span class="string">'\u{1d6c0}'</span>), (<span class="string">'\u{1d6c2}'</span>, <span class="string">'\u{1d6da}'</span>), (<span class="string">'\u{1d6dc}'</span>, <span class="string">'\u{1d6fa}'</span>), (<span class="string">'\u{1d6fc}'</span>,
        <span class="string">'\u{1d714}'</span>), (<span class="string">'\u{1d716}'</span>, <span class="string">'\u{1d734}'</span>), (<span class="string">'\u{1d736}'</span>, <span class="string">'\u{1d74e}'</span>), (<span class="string">'\u{1d750}'</span>,
        <span class="string">'\u{1d76e}'</span>), (<span class="string">'\u{1d770}'</span>, <span class="string">'\u{1d788}'</span>), (<span class="string">'\u{1d78a}'</span>, <span class="string">'\u{1d7a8}'</span>), (<span class="string">'\u{1d7aa}'</span>,
        <span class="string">'\u{1d7c2}'</span>), (<span class="string">'\u{1d7c4}'</span>, <span class="string">'\u{1d7cb}'</span>), (<span class="string">'\u{1df00}'</span>, <span class="string">'\u{1df1e}'</span>), (<span class="string">'\u{1df25}'</span>,
        <span class="string">'\u{1df2a}'</span>), (<span class="string">'\u{1e000}'</span>, <span class="string">'\u{1e006}'</span>), (<span class="string">'\u{1e008}'</span>, <span class="string">'\u{1e018}'</span>), (<span class="string">'\u{1e01b}'</span>,
        <span class="string">'\u{1e021}'</span>), (<span class="string">'\u{1e023}'</span>, <span class="string">'\u{1e024}'</span>), (<span class="string">'\u{1e026}'</span>, <span class="string">'\u{1e02a}'</span>), (<span class="string">'\u{1e030}'</span>,
        <span class="string">'\u{1e06d}'</span>), (<span class="string">'\u{1e08f}'</span>, <span class="string">'\u{1e08f}'</span>), (<span class="string">'\u{1e100}'</span>, <span class="string">'\u{1e12c}'</span>), (<span class="string">'\u{1e137}'</span>,
        <span class="string">'\u{1e13d}'</span>), (<span class="string">'\u{1e14e}'</span>, <span class="string">'\u{1e14e}'</span>), (<span class="string">'\u{1e290}'</span>, <span class="string">'\u{1e2ad}'</span>), (<span class="string">'\u{1e2c0}'</span>,
        <span class="string">'\u{1e2eb}'</span>), (<span class="string">'\u{1e4d0}'</span>, <span class="string">'\u{1e4eb}'</span>), (<span class="string">'\u{1e7e0}'</span>, <span class="string">'\u{1e7e6}'</span>), (<span class="string">'\u{1e7e8}'</span>,
        <span class="string">'\u{1e7eb}'</span>), (<span class="string">'\u{1e7ed}'</span>, <span class="string">'\u{1e7ee}'</span>), (<span class="string">'\u{1e7f0}'</span>, <span class="string">'\u{1e7fe}'</span>), (<span class="string">'\u{1e800}'</span>,
        <span class="string">'\u{1e8c4}'</span>), (<span class="string">'\u{1e900}'</span>, <span class="string">'\u{1e943}'</span>), (<span class="string">'\u{1e947}'</span>, <span class="string">'\u{1e947}'</span>), (<span class="string">'\u{1e94b}'</span>,
        <span class="string">'\u{1e94b}'</span>), (<span class="string">'\u{1ee00}'</span>, <span class="string">'\u{1ee03}'</span>), (<span class="string">'\u{1ee05}'</span>, <span class="string">'\u{1ee1f}'</span>), (<span class="string">'\u{1ee21}'</span>,
        <span class="string">'\u{1ee22}'</span>), (<span class="string">'\u{1ee24}'</span>, <span class="string">'\u{1ee24}'</span>), (<span class="string">'\u{1ee27}'</span>, <span class="string">'\u{1ee27}'</span>), (<span class="string">'\u{1ee29}'</span>,
        <span class="string">'\u{1ee32}'</span>), (<span class="string">'\u{1ee34}'</span>, <span class="string">'\u{1ee37}'</span>), (<span class="string">'\u{1ee39}'</span>, <span class="string">'\u{1ee39}'</span>), (<span class="string">'\u{1ee3b}'</span>,
        <span class="string">'\u{1ee3b}'</span>), (<span class="string">'\u{1ee42}'</span>, <span class="string">'\u{1ee42}'</span>), (<span class="string">'\u{1ee47}'</span>, <span class="string">'\u{1ee47}'</span>), (<span class="string">'\u{1ee49}'</span>,
        <span class="string">'\u{1ee49}'</span>), (<span class="string">'\u{1ee4b}'</span>, <span class="string">'\u{1ee4b}'</span>), (<span class="string">'\u{1ee4d}'</span>, <span class="string">'\u{1ee4f}'</span>), (<span class="string">'\u{1ee51}'</span>,
        <span class="string">'\u{1ee52}'</span>), (<span class="string">'\u{1ee54}'</span>, <span class="string">'\u{1ee54}'</span>), (<span class="string">'\u{1ee57}'</span>, <span class="string">'\u{1ee57}'</span>), (<span class="string">'\u{1ee59}'</span>,
        <span class="string">'\u{1ee59}'</span>), (<span class="string">'\u{1ee5b}'</span>, <span class="string">'\u{1ee5b}'</span>), (<span class="string">'\u{1ee5d}'</span>, <span class="string">'\u{1ee5d}'</span>), (<span class="string">'\u{1ee5f}'</span>,
        <span class="string">'\u{1ee5f}'</span>), (<span class="string">'\u{1ee61}'</span>, <span class="string">'\u{1ee62}'</span>), (<span class="string">'\u{1ee64}'</span>, <span class="string">'\u{1ee64}'</span>), (<span class="string">'\u{1ee67}'</span>,
        <span class="string">'\u{1ee6a}'</span>), (<span class="string">'\u{1ee6c}'</span>, <span class="string">'\u{1ee72}'</span>), (<span class="string">'\u{1ee74}'</span>, <span class="string">'\u{1ee77}'</span>), (<span class="string">'\u{1ee79}'</span>,
        <span class="string">'\u{1ee7c}'</span>), (<span class="string">'\u{1ee7e}'</span>, <span class="string">'\u{1ee7e}'</span>), (<span class="string">'\u{1ee80}'</span>, <span class="string">'\u{1ee89}'</span>), (<span class="string">'\u{1ee8b}'</span>,
        <span class="string">'\u{1ee9b}'</span>), (<span class="string">'\u{1eea1}'</span>, <span class="string">'\u{1eea3}'</span>), (<span class="string">'\u{1eea5}'</span>, <span class="string">'\u{1eea9}'</span>), (<span class="string">'\u{1eeab}'</span>,
        <span class="string">'\u{1eebb}'</span>), (<span class="string">'\u{1f130}'</span>, <span class="string">'\u{1f149}'</span>), (<span class="string">'\u{1f150}'</span>, <span class="string">'\u{1f169}'</span>), (<span class="string">'\u{1f170}'</span>,
        <span class="string">'\u{1f189}'</span>), (<span class="string">'\u{20000}'</span>, <span class="string">'\u{2a6df}'</span>), (<span class="string">'\u{2a700}'</span>, <span class="string">'\u{2b739}'</span>), (<span class="string">'\u{2b740}'</span>,
        <span class="string">'\u{2b81d}'</span>), (<span class="string">'\u{2b820}'</span>, <span class="string">'\u{2cea1}'</span>), (<span class="string">'\u{2ceb0}'</span>, <span class="string">'\u{2ebe0}'</span>), (<span class="string">'\u{2ebf0}'</span>,
        <span class="string">'\u{2ee5d}'</span>), (<span class="string">'\u{2f800}'</span>, <span class="string">'\u{2fa1d}'</span>), (<span class="string">'\u{30000}'</span>, <span class="string">'\u{3134a}'</span>), (<span class="string">'\u{31350}'</span>,
        <span class="string">'\u{323af}'</span>)
    ];

    <span class="attr">#[inline]
    </span><span class="kw">pub fn </span>Alphabetic(c: char) -&gt; bool {
        <span class="kw">super</span>::util::bsearch_range_table(c, Alphabetic_table)
    }

}

<span class="kw">pub mod </span>grapheme {
    <span class="kw">use </span>core::result::Result::{<span class="prelude-val">Ok</span>, <span class="prelude-val">Err</span>};

    <span class="kw">pub use </span><span class="self">self</span>::GraphemeCat::<span class="kw-2">*</span>;

    <span class="attr">#[allow(non_camel_case_types)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    </span><span class="kw">pub enum </span>GraphemeCat {
        GC_Any,
        GC_CR,
        GC_Control,
        GC_Extend,
        GC_Extended_Pictographic,
        GC_L,
        GC_LF,
        GC_LV,
        GC_LVT,
        GC_Prepend,
        GC_Regional_Indicator,
        GC_SpacingMark,
        GC_T,
        GC_V,
        GC_ZWJ,
    }

    <span class="kw">fn </span>bsearch_range_value_table(c: char, r: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char, GraphemeCat)], default_lower: u32, default_upper: u32) -&gt; (u32, u32, GraphemeCat) {
        <span class="kw">use </span>core::cmp::Ordering::{Equal, Less, Greater};
        <span class="kw">match </span>r.binary_search_by(|<span class="kw-2">&amp;</span>(lo, hi, <span class="kw">_</span>)| {
            <span class="kw">if </span>lo &lt;= c &amp;&amp; c &lt;= hi { Equal }
            <span class="kw">else if </span>hi &lt; c { Less }
            <span class="kw">else </span>{ Greater }
        }) {
            <span class="prelude-val">Ok</span>(idx) =&gt; {
                <span class="kw">let </span>(lower, upper, cat) = r[idx];
                (lower <span class="kw">as </span>u32, upper <span class="kw">as </span>u32, cat)
            }
            <span class="prelude-val">Err</span>(idx) =&gt; {
                (
                    <span class="kw">if </span>idx &gt; <span class="number">0 </span>{ r[idx-<span class="number">1</span>].<span class="number">1 </span><span class="kw">as </span>u32 + <span class="number">1 </span>} <span class="kw">else </span>{ default_lower },
                    r.get(idx).map(|c|c.<span class="number">0 </span><span class="kw">as </span>u32 - <span class="number">1</span>).unwrap_or(default_upper),
                    GC_Any,
                )
            }
        }
    }

    <span class="kw">pub fn </span>grapheme_category(c: char) -&gt; (u32, u32, GraphemeCat) {
        <span class="comment">// Perform a quick O(1) lookup in a precomputed table to determine
        // the slice of the range table to search in.
        </span><span class="kw">let </span>lookup_interval = <span class="number">0x80</span>;
        <span class="kw">let </span>idx = (c <span class="kw">as </span>u32 / lookup_interval) <span class="kw">as </span>usize;
        <span class="kw">let </span>range = grapheme_cat_lookup.get(idx..(idx + <span class="number">2</span>)).map_or(
          <span class="comment">// If the `idx` is outside of the precomputed table - use the slice
          // starting from the last covered index in the precomputed table and
          // ending with the length of the range table.
          </span><span class="number">1443</span>..<span class="number">1449</span>,
          |r| (r[<span class="number">0</span>] <span class="kw">as </span>usize)..((r[<span class="number">1</span>] + <span class="number">1</span>) <span class="kw">as </span>usize)
        );

        <span class="comment">// Compute pessimistic default lower and upper bounds on the category.
        // If character doesn't map to any range and there is no adjacent range
        // in the table slice - these bounds has to apply.
        </span><span class="kw">let </span>lower = idx <span class="kw">as </span>u32 * lookup_interval;
        <span class="kw">let </span>upper = lower + lookup_interval - <span class="number">1</span>;
        bsearch_range_value_table(c, <span class="kw-2">&amp;</span>grapheme_cat_table[range], lower, upper)
    }

    <span class="kw">const </span>grapheme_cat_lookup: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[u16] = <span class="kw-2">&amp;</span>[
        <span class="number">0</span>, <span class="number">5</span>, <span class="number">9</span>, <span class="number">9</span>, <span class="number">9</span>, <span class="number">9</span>, <span class="number">9</span>, <span class="number">10</span>, <span class="number">10</span>, <span class="number">10</span>, <span class="number">11</span>, <span class="number">11</span>, <span class="number">16</span>, <span class="number">21</span>, <span class="number">26</span>, <span class="number">29</span>, <span class="number">32</span>, <span class="number">37</span>, <span class="number">41</span>, <span class="number">53</span>, <span class="number">65</span>, <span class="number">75</span>, <span class="number">86</span>, <span class="number">97</span>,
        <span class="number">106</span>, <span class="number">116</span>, <span class="number">131</span>, <span class="number">143</span>, <span class="number">153</span>, <span class="number">157</span>, <span class="number">161</span>, <span class="number">168</span>, <span class="number">173</span>, <span class="number">183</span>, <span class="number">188</span>, <span class="number">189</span>, <span class="number">191</span>, <span class="number">191</span>, <span class="number">191</span>, <span class="number">192</span>, <span class="number">192</span>, <span class="number">192</span>,
        <span class="number">192</span>, <span class="number">192</span>, <span class="number">192</span>, <span class="number">192</span>, <span class="number">192</span>, <span class="number">198</span>, <span class="number">206</span>, <span class="number">209</span>, <span class="number">211</span>, <span class="number">219</span>, <span class="number">219</span>, <span class="number">232</span>, <span class="number">233</span>, <span class="number">242</span>, <span class="number">258</span>, <span class="number">262</span>, <span class="number">270</span>, <span class="number">270</span>,
        <span class="number">271</span>, <span class="number">271</span>, <span class="number">271</span>, <span class="number">271</span>, <span class="number">271</span>, <span class="number">279</span>, <span class="number">280</span>, <span class="number">282</span>, <span class="number">284</span>, <span class="number">284</span>, <span class="number">284</span>, <span class="number">286</span>, <span class="number">290</span>, <span class="number">290</span>, <span class="number">291</span>, <span class="number">291</span>, <span class="number">295</span>, <span class="number">297</span>,
        <span class="number">298</span>, <span class="number">313</span>, <span class="number">317</span>, <span class="number">317</span>, <span class="number">317</span>, <span class="number">318</span>, <span class="number">318</span>, <span class="number">318</span>, <span class="number">318</span>, <span class="number">322</span>, <span class="number">322</span>, <span class="number">322</span>, <span class="number">323</span>, <span class="number">324</span>, <span class="number">325</span>, <span class="number">325</span>, <span class="number">325</span>, <span class="number">325</span>,
        <span class="number">325</span>, <span class="number">328</span>, <span class="number">329</span>, <span class="number">329</span>, <span class="number">329</span>, <span class="number">329</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>,
        <span class="number">331</span>, <span class="number">331</span>, <span class="number">331</span>, <span class="number">333</span>, <span class="number">335</span>, <span class="number">335</span>, <span class="number">335</span>, <span class="number">342</span>, <span class="number">347</span>, <span class="number">351</span>, <span class="number">360</span>, <span class="number">369</span>, <span class="number">379</span>, <span class="number">379</span>, <span class="number">386</span>, <span class="number">395</span>, <span class="number">405</span>, <span class="number">413</span>,
        <span class="number">423</span>, <span class="number">431</span>, <span class="number">441</span>, <span class="number">450</span>, <span class="number">459</span>, <span class="number">469</span>, <span class="number">477</span>, <span class="number">487</span>, <span class="number">495</span>, <span class="number">505</span>, <span class="number">514</span>, <span class="number">523</span>, <span class="number">533</span>, <span class="number">541</span>, <span class="number">551</span>, <span class="number">559</span>, <span class="number">569</span>, <span class="number">578</span>,
        <span class="number">587</span>, <span class="number">597</span>, <span class="number">605</span>, <span class="number">615</span>, <span class="number">623</span>, <span class="number">633</span>, <span class="number">642</span>, <span class="number">651</span>, <span class="number">661</span>, <span class="number">669</span>, <span class="number">679</span>, <span class="number">687</span>, <span class="number">697</span>, <span class="number">706</span>, <span class="number">715</span>, <span class="number">725</span>, <span class="number">733</span>, <span class="number">743</span>,
        <span class="number">751</span>, <span class="number">761</span>, <span class="number">770</span>, <span class="number">779</span>, <span class="number">789</span>, <span class="number">797</span>, <span class="number">807</span>, <span class="number">815</span>, <span class="number">825</span>, <span class="number">834</span>, <span class="number">843</span>, <span class="number">853</span>, <span class="number">861</span>, <span class="number">871</span>, <span class="number">879</span>, <span class="number">889</span>, <span class="number">898</span>, <span class="number">907</span>,
        <span class="number">917</span>, <span class="number">925</span>, <span class="number">935</span>, <span class="number">943</span>, <span class="number">953</span>, <span class="number">962</span>, <span class="number">971</span>, <span class="number">981</span>, <span class="number">989</span>, <span class="number">999</span>, <span class="number">1007</span>, <span class="number">1017</span>, <span class="number">1026</span>, <span class="number">1035</span>, <span class="number">1045</span>, <span class="number">1053</span>, <span class="number">1063</span>,
        <span class="number">1071</span>, <span class="number">1081</span>, <span class="number">1090</span>, <span class="number">1099</span>, <span class="number">1109</span>, <span class="number">1117</span>, <span class="number">1127</span>, <span class="number">1135</span>, <span class="number">1145</span>, <span class="number">1154</span>, <span class="number">1163</span>, <span class="number">1173</span>, <span class="number">1181</span>, <span class="number">1186</span>, <span class="number">1186</span>,
        <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>,
        <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>,
        <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>,
        <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>,
        <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1186</span>, <span class="number">1187</span>, <span class="number">1187</span>, <span class="number">1187</span>, <span class="number">1187</span>, <span class="number">1187</span>, <span class="number">1187</span>,
        <span class="number">1189</span>, <span class="number">1190</span>, <span class="number">1190</span>, <span class="number">1192</span>, <span class="number">1192</span>, <span class="number">1192</span>, <span class="number">1192</span>, <span class="number">1193</span>, <span class="number">1193</span>, <span class="number">1194</span>, <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1195</span>,
        <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1195</span>, <span class="number">1200</span>, <span class="number">1201</span>, <span class="number">1201</span>, <span class="number">1201</span>, <span class="number">1201</span>, <span class="number">1201</span>,
        <span class="number">1202</span>, <span class="number">1202</span>, <span class="number">1202</span>, <span class="number">1204</span>, <span class="number">1205</span>, <span class="number">1206</span>, <span class="number">1212</span>, <span class="number">1221</span>, <span class="number">1227</span>, <span class="number">1236</span>, <span class="number">1244</span>, <span class="number">1247</span>, <span class="number">1260</span>, <span class="number">1260</span>, <span class="number">1267</span>,
        <span class="number">1278</span>, <span class="number">1278</span>, <span class="number">1286</span>, <span class="number">1292</span>, <span class="number">1299</span>, <span class="number">1303</span>, <span class="number">1303</span>, <span class="number">1307</span>, <span class="number">1307</span>, <span class="number">1318</span>, <span class="number">1324</span>, <span class="number">1333</span>, <span class="number">1337</span>, <span class="number">1337</span>, <span class="number">1337</span>,
        <span class="number">1342</span>, <span class="number">1349</span>, <span class="number">1355</span>, <span class="number">1361</span>, <span class="number">1361</span>, <span class="number">1363</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>,
        <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>,
        <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>,
        <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1372</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>,
        <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>,
        <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>,
        <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>,
        <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>,
        <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>,
        <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>,
        <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1375</span>, <span class="number">1376</span>, <span class="number">1377</span>, <span class="number">1377</span>, <span class="number">1377</span>, <span class="number">1377</span>, <span class="number">1377</span>, <span class="number">1377</span>, <span class="number">1377</span>,
        <span class="number">1377</span>, <span class="number">1378</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>,
        <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>,
        <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>,
        <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>,
        <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>,
        <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>,
        <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>,
        <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>,
        <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>,
        <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>,
        <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1382</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>,
        <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>,
        <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1384</span>, <span class="number">1386</span>, <span class="number">1386</span>,
        <span class="number">1386</span>, <span class="number">1386</span>, <span class="number">1392</span>, <span class="number">1395</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>,
        <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1396</span>, <span class="number">1399</span>, <span class="number">1402</span>, <span class="number">1402</span>, <span class="number">1402</span>, <span class="number">1402</span>, <span class="number">1402</span>, <span class="number">1402</span>, <span class="number">1402</span>, <span class="number">1402</span>, <span class="number">1402</span>,
        <span class="number">1402</span>, <span class="number">1402</span>, <span class="number">1407</span>, <span class="number">1408</span>, <span class="number">1409</span>, <span class="number">1409</span>, <span class="number">1409</span>, <span class="number">1411</span>, <span class="number">1411</span>, <span class="number">1411</span>, <span class="number">1411</span>, <span class="number">1412</span>, <span class="number">1412</span>, <span class="number">1412</span>, <span class="number">1412</span>,
        <span class="number">1412</span>, <span class="number">1412</span>, <span class="number">1412</span>, <span class="number">1412</span>, <span class="number">1413</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>,
        <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1414</span>, <span class="number">1415</span>, <span class="number">1419</span>, <span class="number">1423</span>, <span class="number">1428</span>, <span class="number">1428</span>, <span class="number">1428</span>, <span class="number">1430</span>, <span class="number">1430</span>, <span class="number">1430</span>, <span class="number">1431</span>,
        <span class="number">1431</span>, <span class="number">1432</span>, <span class="number">1433</span>, <span class="number">1434</span>, <span class="number">1435</span>, <span class="number">1438</span>, <span class="number">1440</span>, <span class="number">1442</span>, <span class="number">1442</span>, <span class="number">1442</span>, <span class="number">1443</span>, <span class="number">1443</span>, <span class="number">1443</span>, <span class="number">1443</span>, <span class="number">1443</span>,
        <span class="number">1443</span>, <span class="number">1443</span>, <span class="number">1443</span>, <span class="number">1443</span>, <span class="number">1443
    </span>];

    <span class="kw">const </span>grapheme_cat_table: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char, GraphemeCat)] = <span class="kw-2">&amp;</span>[
        (<span class="string">'\u{0}'</span>, <span class="string">'\u{9}'</span>, GC_Control), (<span class="string">'\u{a}'</span>, <span class="string">'\u{a}'</span>, GC_LF), (<span class="string">'\u{b}'</span>, <span class="string">'\u{c}'</span>, GC_Control),
        (<span class="string">'\u{d}'</span>, <span class="string">'\u{d}'</span>, GC_CR), (<span class="string">'\u{e}'</span>, <span class="string">'\u{1f}'</span>, GC_Control), (<span class="string">'\u{7f}'</span>, <span class="string">'\u{9f}'</span>,
        GC_Control), (<span class="string">'\u{a9}'</span>, <span class="string">'\u{a9}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{ad}'</span>, <span class="string">'\u{ad}'</span>,
        GC_Control), (<span class="string">'\u{ae}'</span>, <span class="string">'\u{ae}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{300}'</span>, <span class="string">'\u{36f}'</span>,
        GC_Extend), (<span class="string">'\u{483}'</span>, <span class="string">'\u{489}'</span>, GC_Extend), (<span class="string">'\u{591}'</span>, <span class="string">'\u{5bd}'</span>, GC_Extend),
        (<span class="string">'\u{5bf}'</span>, <span class="string">'\u{5bf}'</span>, GC_Extend), (<span class="string">'\u{5c1}'</span>, <span class="string">'\u{5c2}'</span>, GC_Extend), (<span class="string">'\u{5c4}'</span>, <span class="string">'\u{5c5}'</span>,
        GC_Extend), (<span class="string">'\u{5c7}'</span>, <span class="string">'\u{5c7}'</span>, GC_Extend), (<span class="string">'\u{600}'</span>, <span class="string">'\u{605}'</span>, GC_Prepend),
        (<span class="string">'\u{610}'</span>, <span class="string">'\u{61a}'</span>, GC_Extend), (<span class="string">'\u{61c}'</span>, <span class="string">'\u{61c}'</span>, GC_Control), (<span class="string">'\u{64b}'</span>,
        <span class="string">'\u{65f}'</span>, GC_Extend), (<span class="string">'\u{670}'</span>, <span class="string">'\u{670}'</span>, GC_Extend), (<span class="string">'\u{6d6}'</span>, <span class="string">'\u{6dc}'</span>, GC_Extend),
        (<span class="string">'\u{6dd}'</span>, <span class="string">'\u{6dd}'</span>, GC_Prepend), (<span class="string">'\u{6df}'</span>, <span class="string">'\u{6e4}'</span>, GC_Extend), (<span class="string">'\u{6e7}'</span>,
        <span class="string">'\u{6e8}'</span>, GC_Extend), (<span class="string">'\u{6ea}'</span>, <span class="string">'\u{6ed}'</span>, GC_Extend), (<span class="string">'\u{70f}'</span>, <span class="string">'\u{70f}'</span>,
        GC_Prepend), (<span class="string">'\u{711}'</span>, <span class="string">'\u{711}'</span>, GC_Extend), (<span class="string">'\u{730}'</span>, <span class="string">'\u{74a}'</span>, GC_Extend),
        (<span class="string">'\u{7a6}'</span>, <span class="string">'\u{7b0}'</span>, GC_Extend), (<span class="string">'\u{7eb}'</span>, <span class="string">'\u{7f3}'</span>, GC_Extend), (<span class="string">'\u{7fd}'</span>, <span class="string">'\u{7fd}'</span>,
        GC_Extend), (<span class="string">'\u{816}'</span>, <span class="string">'\u{819}'</span>, GC_Extend), (<span class="string">'\u{81b}'</span>, <span class="string">'\u{823}'</span>, GC_Extend),
        (<span class="string">'\u{825}'</span>, <span class="string">'\u{827}'</span>, GC_Extend), (<span class="string">'\u{829}'</span>, <span class="string">'\u{82d}'</span>, GC_Extend), (<span class="string">'\u{859}'</span>, <span class="string">'\u{85b}'</span>,
        GC_Extend), (<span class="string">'\u{890}'</span>, <span class="string">'\u{891}'</span>, GC_Prepend), (<span class="string">'\u{898}'</span>, <span class="string">'\u{89f}'</span>, GC_Extend),
        (<span class="string">'\u{8ca}'</span>, <span class="string">'\u{8e1}'</span>, GC_Extend), (<span class="string">'\u{8e2}'</span>, <span class="string">'\u{8e2}'</span>, GC_Prepend), (<span class="string">'\u{8e3}'</span>,
        <span class="string">'\u{902}'</span>, GC_Extend), (<span class="string">'\u{903}'</span>, <span class="string">'\u{903}'</span>, GC_SpacingMark), (<span class="string">'\u{93a}'</span>, <span class="string">'\u{93a}'</span>,
        GC_Extend), (<span class="string">'\u{93b}'</span>, <span class="string">'\u{93b}'</span>, GC_SpacingMark), (<span class="string">'\u{93c}'</span>, <span class="string">'\u{93c}'</span>, GC_Extend),
        (<span class="string">'\u{93e}'</span>, <span class="string">'\u{940}'</span>, GC_SpacingMark), (<span class="string">'\u{941}'</span>, <span class="string">'\u{948}'</span>, GC_Extend), (<span class="string">'\u{949}'</span>,
        <span class="string">'\u{94c}'</span>, GC_SpacingMark), (<span class="string">'\u{94d}'</span>, <span class="string">'\u{94d}'</span>, GC_Extend), (<span class="string">'\u{94e}'</span>, <span class="string">'\u{94f}'</span>,
        GC_SpacingMark), (<span class="string">'\u{951}'</span>, <span class="string">'\u{957}'</span>, GC_Extend), (<span class="string">'\u{962}'</span>, <span class="string">'\u{963}'</span>, GC_Extend),
        (<span class="string">'\u{981}'</span>, <span class="string">'\u{981}'</span>, GC_Extend), (<span class="string">'\u{982}'</span>, <span class="string">'\u{983}'</span>, GC_SpacingMark), (<span class="string">'\u{9bc}'</span>,
        <span class="string">'\u{9bc}'</span>, GC_Extend), (<span class="string">'\u{9be}'</span>, <span class="string">'\u{9be}'</span>, GC_Extend), (<span class="string">'\u{9bf}'</span>, <span class="string">'\u{9c0}'</span>,
        GC_SpacingMark), (<span class="string">'\u{9c1}'</span>, <span class="string">'\u{9c4}'</span>, GC_Extend), (<span class="string">'\u{9c7}'</span>, <span class="string">'\u{9c8}'</span>, GC_SpacingMark),
        (<span class="string">'\u{9cb}'</span>, <span class="string">'\u{9cc}'</span>, GC_SpacingMark), (<span class="string">'\u{9cd}'</span>, <span class="string">'\u{9cd}'</span>, GC_Extend), (<span class="string">'\u{9d7}'</span>,
        <span class="string">'\u{9d7}'</span>, GC_Extend), (<span class="string">'\u{9e2}'</span>, <span class="string">'\u{9e3}'</span>, GC_Extend), (<span class="string">'\u{9fe}'</span>, <span class="string">'\u{9fe}'</span>, GC_Extend),
        (<span class="string">'\u{a01}'</span>, <span class="string">'\u{a02}'</span>, GC_Extend), (<span class="string">'\u{a03}'</span>, <span class="string">'\u{a03}'</span>, GC_SpacingMark), (<span class="string">'\u{a3c}'</span>,
        <span class="string">'\u{a3c}'</span>, GC_Extend), (<span class="string">'\u{a3e}'</span>, <span class="string">'\u{a40}'</span>, GC_SpacingMark), (<span class="string">'\u{a41}'</span>, <span class="string">'\u{a42}'</span>,
        GC_Extend), (<span class="string">'\u{a47}'</span>, <span class="string">'\u{a48}'</span>, GC_Extend), (<span class="string">'\u{a4b}'</span>, <span class="string">'\u{a4d}'</span>, GC_Extend),
        (<span class="string">'\u{a51}'</span>, <span class="string">'\u{a51}'</span>, GC_Extend), (<span class="string">'\u{a70}'</span>, <span class="string">'\u{a71}'</span>, GC_Extend), (<span class="string">'\u{a75}'</span>, <span class="string">'\u{a75}'</span>,
        GC_Extend), (<span class="string">'\u{a81}'</span>, <span class="string">'\u{a82}'</span>, GC_Extend), (<span class="string">'\u{a83}'</span>, <span class="string">'\u{a83}'</span>, GC_SpacingMark),
        (<span class="string">'\u{abc}'</span>, <span class="string">'\u{abc}'</span>, GC_Extend), (<span class="string">'\u{abe}'</span>, <span class="string">'\u{ac0}'</span>, GC_SpacingMark), (<span class="string">'\u{ac1}'</span>,
        <span class="string">'\u{ac5}'</span>, GC_Extend), (<span class="string">'\u{ac7}'</span>, <span class="string">'\u{ac8}'</span>, GC_Extend), (<span class="string">'\u{ac9}'</span>, <span class="string">'\u{ac9}'</span>,
        GC_SpacingMark), (<span class="string">'\u{acb}'</span>, <span class="string">'\u{acc}'</span>, GC_SpacingMark), (<span class="string">'\u{acd}'</span>, <span class="string">'\u{acd}'</span>, GC_Extend),
        (<span class="string">'\u{ae2}'</span>, <span class="string">'\u{ae3}'</span>, GC_Extend), (<span class="string">'\u{afa}'</span>, <span class="string">'\u{aff}'</span>, GC_Extend), (<span class="string">'\u{b01}'</span>, <span class="string">'\u{b01}'</span>,
        GC_Extend), (<span class="string">'\u{b02}'</span>, <span class="string">'\u{b03}'</span>, GC_SpacingMark), (<span class="string">'\u{b3c}'</span>, <span class="string">'\u{b3c}'</span>, GC_Extend),
        (<span class="string">'\u{b3e}'</span>, <span class="string">'\u{b3f}'</span>, GC_Extend), (<span class="string">'\u{b40}'</span>, <span class="string">'\u{b40}'</span>, GC_SpacingMark), (<span class="string">'\u{b41}'</span>,
        <span class="string">'\u{b44}'</span>, GC_Extend), (<span class="string">'\u{b47}'</span>, <span class="string">'\u{b48}'</span>, GC_SpacingMark), (<span class="string">'\u{b4b}'</span>, <span class="string">'\u{b4c}'</span>,
        GC_SpacingMark), (<span class="string">'\u{b4d}'</span>, <span class="string">'\u{b4d}'</span>, GC_Extend), (<span class="string">'\u{b55}'</span>, <span class="string">'\u{b57}'</span>, GC_Extend),
        (<span class="string">'\u{b62}'</span>, <span class="string">'\u{b63}'</span>, GC_Extend), (<span class="string">'\u{b82}'</span>, <span class="string">'\u{b82}'</span>, GC_Extend), (<span class="string">'\u{bbe}'</span>, <span class="string">'\u{bbe}'</span>,
        GC_Extend), (<span class="string">'\u{bbf}'</span>, <span class="string">'\u{bbf}'</span>, GC_SpacingMark), (<span class="string">'\u{bc0}'</span>, <span class="string">'\u{bc0}'</span>, GC_Extend),
        (<span class="string">'\u{bc1}'</span>, <span class="string">'\u{bc2}'</span>, GC_SpacingMark), (<span class="string">'\u{bc6}'</span>, <span class="string">'\u{bc8}'</span>, GC_SpacingMark), (<span class="string">'\u{bca}'</span>,
        <span class="string">'\u{bcc}'</span>, GC_SpacingMark), (<span class="string">'\u{bcd}'</span>, <span class="string">'\u{bcd}'</span>, GC_Extend), (<span class="string">'\u{bd7}'</span>, <span class="string">'\u{bd7}'</span>,
        GC_Extend), (<span class="string">'\u{c00}'</span>, <span class="string">'\u{c00}'</span>, GC_Extend), (<span class="string">'\u{c01}'</span>, <span class="string">'\u{c03}'</span>, GC_SpacingMark),
        (<span class="string">'\u{c04}'</span>, <span class="string">'\u{c04}'</span>, GC_Extend), (<span class="string">'\u{c3c}'</span>, <span class="string">'\u{c3c}'</span>, GC_Extend), (<span class="string">'\u{c3e}'</span>, <span class="string">'\u{c40}'</span>,
        GC_Extend), (<span class="string">'\u{c41}'</span>, <span class="string">'\u{c44}'</span>, GC_SpacingMark), (<span class="string">'\u{c46}'</span>, <span class="string">'\u{c48}'</span>, GC_Extend),
        (<span class="string">'\u{c4a}'</span>, <span class="string">'\u{c4d}'</span>, GC_Extend), (<span class="string">'\u{c55}'</span>, <span class="string">'\u{c56}'</span>, GC_Extend), (<span class="string">'\u{c62}'</span>, <span class="string">'\u{c63}'</span>,
        GC_Extend), (<span class="string">'\u{c81}'</span>, <span class="string">'\u{c81}'</span>, GC_Extend), (<span class="string">'\u{c82}'</span>, <span class="string">'\u{c83}'</span>, GC_SpacingMark),
        (<span class="string">'\u{cbc}'</span>, <span class="string">'\u{cbc}'</span>, GC_Extend), (<span class="string">'\u{cbe}'</span>, <span class="string">'\u{cbe}'</span>, GC_SpacingMark), (<span class="string">'\u{cbf}'</span>,
        <span class="string">'\u{cbf}'</span>, GC_Extend), (<span class="string">'\u{cc0}'</span>, <span class="string">'\u{cc1}'</span>, GC_SpacingMark), (<span class="string">'\u{cc2}'</span>, <span class="string">'\u{cc2}'</span>,
        GC_Extend), (<span class="string">'\u{cc3}'</span>, <span class="string">'\u{cc4}'</span>, GC_SpacingMark), (<span class="string">'\u{cc6}'</span>, <span class="string">'\u{cc6}'</span>, GC_Extend),
        (<span class="string">'\u{cc7}'</span>, <span class="string">'\u{cc8}'</span>, GC_SpacingMark), (<span class="string">'\u{cca}'</span>, <span class="string">'\u{ccb}'</span>, GC_SpacingMark), (<span class="string">'\u{ccc}'</span>,
        <span class="string">'\u{ccd}'</span>, GC_Extend), (<span class="string">'\u{cd5}'</span>, <span class="string">'\u{cd6}'</span>, GC_Extend), (<span class="string">'\u{ce2}'</span>, <span class="string">'\u{ce3}'</span>, GC_Extend),
        (<span class="string">'\u{cf3}'</span>, <span class="string">'\u{cf3}'</span>, GC_SpacingMark), (<span class="string">'\u{d00}'</span>, <span class="string">'\u{d01}'</span>, GC_Extend), (<span class="string">'\u{d02}'</span>,
        <span class="string">'\u{d03}'</span>, GC_SpacingMark), (<span class="string">'\u{d3b}'</span>, <span class="string">'\u{d3c}'</span>, GC_Extend), (<span class="string">'\u{d3e}'</span>, <span class="string">'\u{d3e}'</span>,
        GC_Extend), (<span class="string">'\u{d3f}'</span>, <span class="string">'\u{d40}'</span>, GC_SpacingMark), (<span class="string">'\u{d41}'</span>, <span class="string">'\u{d44}'</span>, GC_Extend),
        (<span class="string">'\u{d46}'</span>, <span class="string">'\u{d48}'</span>, GC_SpacingMark), (<span class="string">'\u{d4a}'</span>, <span class="string">'\u{d4c}'</span>, GC_SpacingMark), (<span class="string">'\u{d4d}'</span>,
        <span class="string">'\u{d4d}'</span>, GC_Extend), (<span class="string">'\u{d4e}'</span>, <span class="string">'\u{d4e}'</span>, GC_Prepend), (<span class="string">'\u{d57}'</span>, <span class="string">'\u{d57}'</span>,
        GC_Extend), (<span class="string">'\u{d62}'</span>, <span class="string">'\u{d63}'</span>, GC_Extend), (<span class="string">'\u{d81}'</span>, <span class="string">'\u{d81}'</span>, GC_Extend),
        (<span class="string">'\u{d82}'</span>, <span class="string">'\u{d83}'</span>, GC_SpacingMark), (<span class="string">'\u{dca}'</span>, <span class="string">'\u{dca}'</span>, GC_Extend), (<span class="string">'\u{dcf}'</span>,
        <span class="string">'\u{dcf}'</span>, GC_Extend), (<span class="string">'\u{dd0}'</span>, <span class="string">'\u{dd1}'</span>, GC_SpacingMark), (<span class="string">'\u{dd2}'</span>, <span class="string">'\u{dd4}'</span>,
        GC_Extend), (<span class="string">'\u{dd6}'</span>, <span class="string">'\u{dd6}'</span>, GC_Extend), (<span class="string">'\u{dd8}'</span>, <span class="string">'\u{dde}'</span>, GC_SpacingMark),
        (<span class="string">'\u{ddf}'</span>, <span class="string">'\u{ddf}'</span>, GC_Extend), (<span class="string">'\u{df2}'</span>, <span class="string">'\u{df3}'</span>, GC_SpacingMark), (<span class="string">'\u{e31}'</span>,
        <span class="string">'\u{e31}'</span>, GC_Extend), (<span class="string">'\u{e33}'</span>, <span class="string">'\u{e33}'</span>, GC_SpacingMark), (<span class="string">'\u{e34}'</span>, <span class="string">'\u{e3a}'</span>,
        GC_Extend), (<span class="string">'\u{e47}'</span>, <span class="string">'\u{e4e}'</span>, GC_Extend), (<span class="string">'\u{eb1}'</span>, <span class="string">'\u{eb1}'</span>, GC_Extend),
        (<span class="string">'\u{eb3}'</span>, <span class="string">'\u{eb3}'</span>, GC_SpacingMark), (<span class="string">'\u{eb4}'</span>, <span class="string">'\u{ebc}'</span>, GC_Extend), (<span class="string">'\u{ec8}'</span>,
        <span class="string">'\u{ece}'</span>, GC_Extend), (<span class="string">'\u{f18}'</span>, <span class="string">'\u{f19}'</span>, GC_Extend), (<span class="string">'\u{f35}'</span>, <span class="string">'\u{f35}'</span>, GC_Extend),
        (<span class="string">'\u{f37}'</span>, <span class="string">'\u{f37}'</span>, GC_Extend), (<span class="string">'\u{f39}'</span>, <span class="string">'\u{f39}'</span>, GC_Extend), (<span class="string">'\u{f3e}'</span>, <span class="string">'\u{f3f}'</span>,
        GC_SpacingMark), (<span class="string">'\u{f71}'</span>, <span class="string">'\u{f7e}'</span>, GC_Extend), (<span class="string">'\u{f7f}'</span>, <span class="string">'\u{f7f}'</span>, GC_SpacingMark),
        (<span class="string">'\u{f80}'</span>, <span class="string">'\u{f84}'</span>, GC_Extend), (<span class="string">'\u{f86}'</span>, <span class="string">'\u{f87}'</span>, GC_Extend), (<span class="string">'\u{f8d}'</span>, <span class="string">'\u{f97}'</span>,
        GC_Extend), (<span class="string">'\u{f99}'</span>, <span class="string">'\u{fbc}'</span>, GC_Extend), (<span class="string">'\u{fc6}'</span>, <span class="string">'\u{fc6}'</span>, GC_Extend),
        (<span class="string">'\u{102d}'</span>, <span class="string">'\u{1030}'</span>, GC_Extend), (<span class="string">'\u{1031}'</span>, <span class="string">'\u{1031}'</span>, GC_SpacingMark), (<span class="string">'\u{1032}'</span>,
        <span class="string">'\u{1037}'</span>, GC_Extend), (<span class="string">'\u{1039}'</span>, <span class="string">'\u{103a}'</span>, GC_Extend), (<span class="string">'\u{103b}'</span>, <span class="string">'\u{103c}'</span>,
        GC_SpacingMark), (<span class="string">'\u{103d}'</span>, <span class="string">'\u{103e}'</span>, GC_Extend), (<span class="string">'\u{1056}'</span>, <span class="string">'\u{1057}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1058}'</span>, <span class="string">'\u{1059}'</span>, GC_Extend), (<span class="string">'\u{105e}'</span>, <span class="string">'\u{1060}'</span>, GC_Extend),
        (<span class="string">'\u{1071}'</span>, <span class="string">'\u{1074}'</span>, GC_Extend), (<span class="string">'\u{1082}'</span>, <span class="string">'\u{1082}'</span>, GC_Extend), (<span class="string">'\u{1084}'</span>,
        <span class="string">'\u{1084}'</span>, GC_SpacingMark), (<span class="string">'\u{1085}'</span>, <span class="string">'\u{1086}'</span>, GC_Extend), (<span class="string">'\u{108d}'</span>, <span class="string">'\u{108d}'</span>,
        GC_Extend), (<span class="string">'\u{109d}'</span>, <span class="string">'\u{109d}'</span>, GC_Extend), (<span class="string">'\u{1100}'</span>, <span class="string">'\u{115f}'</span>, GC_L),
        (<span class="string">'\u{1160}'</span>, <span class="string">'\u{11a7}'</span>, GC_V), (<span class="string">'\u{11a8}'</span>, <span class="string">'\u{11ff}'</span>, GC_T), (<span class="string">'\u{135d}'</span>, <span class="string">'\u{135f}'</span>,
        GC_Extend), (<span class="string">'\u{1712}'</span>, <span class="string">'\u{1714}'</span>, GC_Extend), (<span class="string">'\u{1715}'</span>, <span class="string">'\u{1715}'</span>, GC_SpacingMark),
        (<span class="string">'\u{1732}'</span>, <span class="string">'\u{1733}'</span>, GC_Extend), (<span class="string">'\u{1734}'</span>, <span class="string">'\u{1734}'</span>, GC_SpacingMark), (<span class="string">'\u{1752}'</span>,
        <span class="string">'\u{1753}'</span>, GC_Extend), (<span class="string">'\u{1772}'</span>, <span class="string">'\u{1773}'</span>, GC_Extend), (<span class="string">'\u{17b4}'</span>, <span class="string">'\u{17b5}'</span>,
        GC_Extend), (<span class="string">'\u{17b6}'</span>, <span class="string">'\u{17b6}'</span>, GC_SpacingMark), (<span class="string">'\u{17b7}'</span>, <span class="string">'\u{17bd}'</span>, GC_Extend),
        (<span class="string">'\u{17be}'</span>, <span class="string">'\u{17c5}'</span>, GC_SpacingMark), (<span class="string">'\u{17c6}'</span>, <span class="string">'\u{17c6}'</span>, GC_Extend), (<span class="string">'\u{17c7}'</span>,
        <span class="string">'\u{17c8}'</span>, GC_SpacingMark), (<span class="string">'\u{17c9}'</span>, <span class="string">'\u{17d3}'</span>, GC_Extend), (<span class="string">'\u{17dd}'</span>, <span class="string">'\u{17dd}'</span>,
        GC_Extend), (<span class="string">'\u{180b}'</span>, <span class="string">'\u{180d}'</span>, GC_Extend), (<span class="string">'\u{180e}'</span>, <span class="string">'\u{180e}'</span>, GC_Control),
        (<span class="string">'\u{180f}'</span>, <span class="string">'\u{180f}'</span>, GC_Extend), (<span class="string">'\u{1885}'</span>, <span class="string">'\u{1886}'</span>, GC_Extend), (<span class="string">'\u{18a9}'</span>,
        <span class="string">'\u{18a9}'</span>, GC_Extend), (<span class="string">'\u{1920}'</span>, <span class="string">'\u{1922}'</span>, GC_Extend), (<span class="string">'\u{1923}'</span>, <span class="string">'\u{1926}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1927}'</span>, <span class="string">'\u{1928}'</span>, GC_Extend), (<span class="string">'\u{1929}'</span>, <span class="string">'\u{192b}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1930}'</span>, <span class="string">'\u{1931}'</span>, GC_SpacingMark), (<span class="string">'\u{1932}'</span>, <span class="string">'\u{1932}'</span>,
        GC_Extend), (<span class="string">'\u{1933}'</span>, <span class="string">'\u{1938}'</span>, GC_SpacingMark), (<span class="string">'\u{1939}'</span>, <span class="string">'\u{193b}'</span>, GC_Extend),
        (<span class="string">'\u{1a17}'</span>, <span class="string">'\u{1a18}'</span>, GC_Extend), (<span class="string">'\u{1a19}'</span>, <span class="string">'\u{1a1a}'</span>, GC_SpacingMark), (<span class="string">'\u{1a1b}'</span>,
        <span class="string">'\u{1a1b}'</span>, GC_Extend), (<span class="string">'\u{1a55}'</span>, <span class="string">'\u{1a55}'</span>, GC_SpacingMark), (<span class="string">'\u{1a56}'</span>, <span class="string">'\u{1a56}'</span>,
        GC_Extend), (<span class="string">'\u{1a57}'</span>, <span class="string">'\u{1a57}'</span>, GC_SpacingMark), (<span class="string">'\u{1a58}'</span>, <span class="string">'\u{1a5e}'</span>, GC_Extend),
        (<span class="string">'\u{1a60}'</span>, <span class="string">'\u{1a60}'</span>, GC_Extend), (<span class="string">'\u{1a62}'</span>, <span class="string">'\u{1a62}'</span>, GC_Extend), (<span class="string">'\u{1a65}'</span>,
        <span class="string">'\u{1a6c}'</span>, GC_Extend), (<span class="string">'\u{1a6d}'</span>, <span class="string">'\u{1a72}'</span>, GC_SpacingMark), (<span class="string">'\u{1a73}'</span>, <span class="string">'\u{1a7c}'</span>,
        GC_Extend), (<span class="string">'\u{1a7f}'</span>, <span class="string">'\u{1a7f}'</span>, GC_Extend), (<span class="string">'\u{1ab0}'</span>, <span class="string">'\u{1ace}'</span>, GC_Extend),
        (<span class="string">'\u{1b00}'</span>, <span class="string">'\u{1b03}'</span>, GC_Extend), (<span class="string">'\u{1b04}'</span>, <span class="string">'\u{1b04}'</span>, GC_SpacingMark), (<span class="string">'\u{1b34}'</span>,
        <span class="string">'\u{1b3a}'</span>, GC_Extend), (<span class="string">'\u{1b3b}'</span>, <span class="string">'\u{1b3b}'</span>, GC_SpacingMark), (<span class="string">'\u{1b3c}'</span>, <span class="string">'\u{1b3c}'</span>,
        GC_Extend), (<span class="string">'\u{1b3d}'</span>, <span class="string">'\u{1b41}'</span>, GC_SpacingMark), (<span class="string">'\u{1b42}'</span>, <span class="string">'\u{1b42}'</span>, GC_Extend),
        (<span class="string">'\u{1b43}'</span>, <span class="string">'\u{1b44}'</span>, GC_SpacingMark), (<span class="string">'\u{1b6b}'</span>, <span class="string">'\u{1b73}'</span>, GC_Extend), (<span class="string">'\u{1b80}'</span>,
        <span class="string">'\u{1b81}'</span>, GC_Extend), (<span class="string">'\u{1b82}'</span>, <span class="string">'\u{1b82}'</span>, GC_SpacingMark), (<span class="string">'\u{1ba1}'</span>, <span class="string">'\u{1ba1}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1ba2}'</span>, <span class="string">'\u{1ba5}'</span>, GC_Extend), (<span class="string">'\u{1ba6}'</span>, <span class="string">'\u{1ba7}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1ba8}'</span>, <span class="string">'\u{1ba9}'</span>, GC_Extend), (<span class="string">'\u{1baa}'</span>, <span class="string">'\u{1baa}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1bab}'</span>, <span class="string">'\u{1bad}'</span>, GC_Extend), (<span class="string">'\u{1be6}'</span>, <span class="string">'\u{1be6}'</span>, GC_Extend),
        (<span class="string">'\u{1be7}'</span>, <span class="string">'\u{1be7}'</span>, GC_SpacingMark), (<span class="string">'\u{1be8}'</span>, <span class="string">'\u{1be9}'</span>, GC_Extend), (<span class="string">'\u{1bea}'</span>,
        <span class="string">'\u{1bec}'</span>, GC_SpacingMark), (<span class="string">'\u{1bed}'</span>, <span class="string">'\u{1bed}'</span>, GC_Extend), (<span class="string">'\u{1bee}'</span>, <span class="string">'\u{1bee}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1bef}'</span>, <span class="string">'\u{1bf1}'</span>, GC_Extend), (<span class="string">'\u{1bf2}'</span>, <span class="string">'\u{1bf3}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1c24}'</span>, <span class="string">'\u{1c2b}'</span>, GC_SpacingMark), (<span class="string">'\u{1c2c}'</span>, <span class="string">'\u{1c33}'</span>,
        GC_Extend), (<span class="string">'\u{1c34}'</span>, <span class="string">'\u{1c35}'</span>, GC_SpacingMark), (<span class="string">'\u{1c36}'</span>, <span class="string">'\u{1c37}'</span>, GC_Extend),
        (<span class="string">'\u{1cd0}'</span>, <span class="string">'\u{1cd2}'</span>, GC_Extend), (<span class="string">'\u{1cd4}'</span>, <span class="string">'\u{1ce0}'</span>, GC_Extend), (<span class="string">'\u{1ce1}'</span>,
        <span class="string">'\u{1ce1}'</span>, GC_SpacingMark), (<span class="string">'\u{1ce2}'</span>, <span class="string">'\u{1ce8}'</span>, GC_Extend), (<span class="string">'\u{1ced}'</span>, <span class="string">'\u{1ced}'</span>,
        GC_Extend), (<span class="string">'\u{1cf4}'</span>, <span class="string">'\u{1cf4}'</span>, GC_Extend), (<span class="string">'\u{1cf7}'</span>, <span class="string">'\u{1cf7}'</span>, GC_SpacingMark),
        (<span class="string">'\u{1cf8}'</span>, <span class="string">'\u{1cf9}'</span>, GC_Extend), (<span class="string">'\u{1dc0}'</span>, <span class="string">'\u{1dff}'</span>, GC_Extend), (<span class="string">'\u{200b}'</span>,
        <span class="string">'\u{200b}'</span>, GC_Control), (<span class="string">'\u{200c}'</span>, <span class="string">'\u{200c}'</span>, GC_Extend), (<span class="string">'\u{200d}'</span>, <span class="string">'\u{200d}'</span>,
        GC_ZWJ), (<span class="string">'\u{200e}'</span>, <span class="string">'\u{200f}'</span>, GC_Control), (<span class="string">'\u{2028}'</span>, <span class="string">'\u{202e}'</span>, GC_Control),
        (<span class="string">'\u{203c}'</span>, <span class="string">'\u{203c}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2049}'</span>, <span class="string">'\u{2049}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{2060}'</span>, <span class="string">'\u{206f}'</span>, GC_Control), (<span class="string">'\u{20d0}'</span>, <span class="string">'\u{20f0}'</span>,
        GC_Extend), (<span class="string">'\u{2122}'</span>, <span class="string">'\u{2122}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2139}'</span>, <span class="string">'\u{2139}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{2194}'</span>, <span class="string">'\u{2199}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{21a9}'</span>,
        <span class="string">'\u{21aa}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{231a}'</span>, <span class="string">'\u{231b}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{2328}'</span>, <span class="string">'\u{2328}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2388}'</span>, <span class="string">'\u{2388}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{23cf}'</span>, <span class="string">'\u{23cf}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{23e9}'</span>,
        <span class="string">'\u{23f3}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{23f8}'</span>, <span class="string">'\u{23fa}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{24c2}'</span>, <span class="string">'\u{24c2}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{25aa}'</span>, <span class="string">'\u{25ab}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{25b6}'</span>, <span class="string">'\u{25b6}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{25c0}'</span>,
        <span class="string">'\u{25c0}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{25fb}'</span>, <span class="string">'\u{25fe}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{2600}'</span>, <span class="string">'\u{2605}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2607}'</span>, <span class="string">'\u{2612}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{2614}'</span>, <span class="string">'\u{2685}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2690}'</span>,
        <span class="string">'\u{2705}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2708}'</span>, <span class="string">'\u{2712}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{2714}'</span>, <span class="string">'\u{2714}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2716}'</span>, <span class="string">'\u{2716}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{271d}'</span>, <span class="string">'\u{271d}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2721}'</span>,
        <span class="string">'\u{2721}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2728}'</span>, <span class="string">'\u{2728}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{2733}'</span>, <span class="string">'\u{2734}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2744}'</span>, <span class="string">'\u{2744}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{2747}'</span>, <span class="string">'\u{2747}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{274c}'</span>,
        <span class="string">'\u{274c}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{274e}'</span>, <span class="string">'\u{274e}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{2753}'</span>, <span class="string">'\u{2755}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2757}'</span>, <span class="string">'\u{2757}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{2763}'</span>, <span class="string">'\u{2767}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2795}'</span>,
        <span class="string">'\u{2797}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{27a1}'</span>, <span class="string">'\u{27a1}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{27b0}'</span>, <span class="string">'\u{27b0}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{27bf}'</span>, <span class="string">'\u{27bf}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{2934}'</span>, <span class="string">'\u{2935}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2b05}'</span>,
        <span class="string">'\u{2b07}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2b1b}'</span>, <span class="string">'\u{2b1c}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{2b50}'</span>, <span class="string">'\u{2b50}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{2b55}'</span>, <span class="string">'\u{2b55}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{2cef}'</span>, <span class="string">'\u{2cf1}'</span>, GC_Extend), (<span class="string">'\u{2d7f}'</span>, <span class="string">'\u{2d7f}'</span>,
        GC_Extend), (<span class="string">'\u{2de0}'</span>, <span class="string">'\u{2dff}'</span>, GC_Extend), (<span class="string">'\u{302a}'</span>, <span class="string">'\u{302f}'</span>, GC_Extend),
        (<span class="string">'\u{3030}'</span>, <span class="string">'\u{3030}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{303d}'</span>, <span class="string">'\u{303d}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{3099}'</span>, <span class="string">'\u{309a}'</span>, GC_Extend), (<span class="string">'\u{3297}'</span>, <span class="string">'\u{3297}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{3299}'</span>, <span class="string">'\u{3299}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{a66f}'</span>,
        <span class="string">'\u{a672}'</span>, GC_Extend), (<span class="string">'\u{a674}'</span>, <span class="string">'\u{a67d}'</span>, GC_Extend), (<span class="string">'\u{a69e}'</span>, <span class="string">'\u{a69f}'</span>,
        GC_Extend), (<span class="string">'\u{a6f0}'</span>, <span class="string">'\u{a6f1}'</span>, GC_Extend), (<span class="string">'\u{a802}'</span>, <span class="string">'\u{a802}'</span>, GC_Extend),
        (<span class="string">'\u{a806}'</span>, <span class="string">'\u{a806}'</span>, GC_Extend), (<span class="string">'\u{a80b}'</span>, <span class="string">'\u{a80b}'</span>, GC_Extend), (<span class="string">'\u{a823}'</span>,
        <span class="string">'\u{a824}'</span>, GC_SpacingMark), (<span class="string">'\u{a825}'</span>, <span class="string">'\u{a826}'</span>, GC_Extend), (<span class="string">'\u{a827}'</span>, <span class="string">'\u{a827}'</span>,
        GC_SpacingMark), (<span class="string">'\u{a82c}'</span>, <span class="string">'\u{a82c}'</span>, GC_Extend), (<span class="string">'\u{a880}'</span>, <span class="string">'\u{a881}'</span>,
        GC_SpacingMark), (<span class="string">'\u{a8b4}'</span>, <span class="string">'\u{a8c3}'</span>, GC_SpacingMark), (<span class="string">'\u{a8c4}'</span>, <span class="string">'\u{a8c5}'</span>,
        GC_Extend), (<span class="string">'\u{a8e0}'</span>, <span class="string">'\u{a8f1}'</span>, GC_Extend), (<span class="string">'\u{a8ff}'</span>, <span class="string">'\u{a8ff}'</span>, GC_Extend),
        (<span class="string">'\u{a926}'</span>, <span class="string">'\u{a92d}'</span>, GC_Extend), (<span class="string">'\u{a947}'</span>, <span class="string">'\u{a951}'</span>, GC_Extend), (<span class="string">'\u{a952}'</span>,
        <span class="string">'\u{a953}'</span>, GC_SpacingMark), (<span class="string">'\u{a960}'</span>, <span class="string">'\u{a97c}'</span>, GC_L), (<span class="string">'\u{a980}'</span>, <span class="string">'\u{a982}'</span>,
        GC_Extend), (<span class="string">'\u{a983}'</span>, <span class="string">'\u{a983}'</span>, GC_SpacingMark), (<span class="string">'\u{a9b3}'</span>, <span class="string">'\u{a9b3}'</span>, GC_Extend),
        (<span class="string">'\u{a9b4}'</span>, <span class="string">'\u{a9b5}'</span>, GC_SpacingMark), (<span class="string">'\u{a9b6}'</span>, <span class="string">'\u{a9b9}'</span>, GC_Extend), (<span class="string">'\u{a9ba}'</span>,
        <span class="string">'\u{a9bb}'</span>, GC_SpacingMark), (<span class="string">'\u{a9bc}'</span>, <span class="string">'\u{a9bd}'</span>, GC_Extend), (<span class="string">'\u{a9be}'</span>, <span class="string">'\u{a9c0}'</span>,
        GC_SpacingMark), (<span class="string">'\u{a9e5}'</span>, <span class="string">'\u{a9e5}'</span>, GC_Extend), (<span class="string">'\u{aa29}'</span>, <span class="string">'\u{aa2e}'</span>, GC_Extend),
        (<span class="string">'\u{aa2f}'</span>, <span class="string">'\u{aa30}'</span>, GC_SpacingMark), (<span class="string">'\u{aa31}'</span>, <span class="string">'\u{aa32}'</span>, GC_Extend), (<span class="string">'\u{aa33}'</span>,
        <span class="string">'\u{aa34}'</span>, GC_SpacingMark), (<span class="string">'\u{aa35}'</span>, <span class="string">'\u{aa36}'</span>, GC_Extend), (<span class="string">'\u{aa43}'</span>, <span class="string">'\u{aa43}'</span>,
        GC_Extend), (<span class="string">'\u{aa4c}'</span>, <span class="string">'\u{aa4c}'</span>, GC_Extend), (<span class="string">'\u{aa4d}'</span>, <span class="string">'\u{aa4d}'</span>, GC_SpacingMark),
        (<span class="string">'\u{aa7c}'</span>, <span class="string">'\u{aa7c}'</span>, GC_Extend), (<span class="string">'\u{aab0}'</span>, <span class="string">'\u{aab0}'</span>, GC_Extend), (<span class="string">'\u{aab2}'</span>,
        <span class="string">'\u{aab4}'</span>, GC_Extend), (<span class="string">'\u{aab7}'</span>, <span class="string">'\u{aab8}'</span>, GC_Extend), (<span class="string">'\u{aabe}'</span>, <span class="string">'\u{aabf}'</span>,
        GC_Extend), (<span class="string">'\u{aac1}'</span>, <span class="string">'\u{aac1}'</span>, GC_Extend), (<span class="string">'\u{aaeb}'</span>, <span class="string">'\u{aaeb}'</span>, GC_SpacingMark),
        (<span class="string">'\u{aaec}'</span>, <span class="string">'\u{aaed}'</span>, GC_Extend), (<span class="string">'\u{aaee}'</span>, <span class="string">'\u{aaef}'</span>, GC_SpacingMark), (<span class="string">'\u{aaf5}'</span>,
        <span class="string">'\u{aaf5}'</span>, GC_SpacingMark), (<span class="string">'\u{aaf6}'</span>, <span class="string">'\u{aaf6}'</span>, GC_Extend), (<span class="string">'\u{abe3}'</span>, <span class="string">'\u{abe4}'</span>,
        GC_SpacingMark), (<span class="string">'\u{abe5}'</span>, <span class="string">'\u{abe5}'</span>, GC_Extend), (<span class="string">'\u{abe6}'</span>, <span class="string">'\u{abe7}'</span>,
        GC_SpacingMark), (<span class="string">'\u{abe8}'</span>, <span class="string">'\u{abe8}'</span>, GC_Extend), (<span class="string">'\u{abe9}'</span>, <span class="string">'\u{abea}'</span>,
        GC_SpacingMark), (<span class="string">'\u{abec}'</span>, <span class="string">'\u{abec}'</span>, GC_SpacingMark), (<span class="string">'\u{abed}'</span>, <span class="string">'\u{abed}'</span>,
        GC_Extend), (<span class="string">'\u{ac00}'</span>, <span class="string">'\u{ac00}'</span>, GC_LV), (<span class="string">'\u{ac01}'</span>, <span class="string">'\u{ac1b}'</span>, GC_LVT), (<span class="string">'\u{ac1c}'</span>,
        <span class="string">'\u{ac1c}'</span>, GC_LV), (<span class="string">'\u{ac1d}'</span>, <span class="string">'\u{ac37}'</span>, GC_LVT), (<span class="string">'\u{ac38}'</span>, <span class="string">'\u{ac38}'</span>, GC_LV),
        (<span class="string">'\u{ac39}'</span>, <span class="string">'\u{ac53}'</span>, GC_LVT), (<span class="string">'\u{ac54}'</span>, <span class="string">'\u{ac54}'</span>, GC_LV), (<span class="string">'\u{ac55}'</span>, <span class="string">'\u{ac6f}'</span>,
        GC_LVT), (<span class="string">'\u{ac70}'</span>, <span class="string">'\u{ac70}'</span>, GC_LV), (<span class="string">'\u{ac71}'</span>, <span class="string">'\u{ac8b}'</span>, GC_LVT), (<span class="string">'\u{ac8c}'</span>,
        <span class="string">'\u{ac8c}'</span>, GC_LV), (<span class="string">'\u{ac8d}'</span>, <span class="string">'\u{aca7}'</span>, GC_LVT), (<span class="string">'\u{aca8}'</span>, <span class="string">'\u{aca8}'</span>, GC_LV),
        (<span class="string">'\u{aca9}'</span>, <span class="string">'\u{acc3}'</span>, GC_LVT), (<span class="string">'\u{acc4}'</span>, <span class="string">'\u{acc4}'</span>, GC_LV), (<span class="string">'\u{acc5}'</span>, <span class="string">'\u{acdf}'</span>,
        GC_LVT), (<span class="string">'\u{ace0}'</span>, <span class="string">'\u{ace0}'</span>, GC_LV), (<span class="string">'\u{ace1}'</span>, <span class="string">'\u{acfb}'</span>, GC_LVT), (<span class="string">'\u{acfc}'</span>,
        <span class="string">'\u{acfc}'</span>, GC_LV), (<span class="string">'\u{acfd}'</span>, <span class="string">'\u{ad17}'</span>, GC_LVT), (<span class="string">'\u{ad18}'</span>, <span class="string">'\u{ad18}'</span>, GC_LV),
        (<span class="string">'\u{ad19}'</span>, <span class="string">'\u{ad33}'</span>, GC_LVT), (<span class="string">'\u{ad34}'</span>, <span class="string">'\u{ad34}'</span>, GC_LV), (<span class="string">'\u{ad35}'</span>, <span class="string">'\u{ad4f}'</span>,
        GC_LVT), (<span class="string">'\u{ad50}'</span>, <span class="string">'\u{ad50}'</span>, GC_LV), (<span class="string">'\u{ad51}'</span>, <span class="string">'\u{ad6b}'</span>, GC_LVT), (<span class="string">'\u{ad6c}'</span>,
        <span class="string">'\u{ad6c}'</span>, GC_LV), (<span class="string">'\u{ad6d}'</span>, <span class="string">'\u{ad87}'</span>, GC_LVT), (<span class="string">'\u{ad88}'</span>, <span class="string">'\u{ad88}'</span>, GC_LV),
        (<span class="string">'\u{ad89}'</span>, <span class="string">'\u{ada3}'</span>, GC_LVT), (<span class="string">'\u{ada4}'</span>, <span class="string">'\u{ada4}'</span>, GC_LV), (<span class="string">'\u{ada5}'</span>, <span class="string">'\u{adbf}'</span>,
        GC_LVT), (<span class="string">'\u{adc0}'</span>, <span class="string">'\u{adc0}'</span>, GC_LV), (<span class="string">'\u{adc1}'</span>, <span class="string">'\u{addb}'</span>, GC_LVT), (<span class="string">'\u{addc}'</span>,
        <span class="string">'\u{addc}'</span>, GC_LV), (<span class="string">'\u{addd}'</span>, <span class="string">'\u{adf7}'</span>, GC_LVT), (<span class="string">'\u{adf8}'</span>, <span class="string">'\u{adf8}'</span>, GC_LV),
        (<span class="string">'\u{adf9}'</span>, <span class="string">'\u{ae13}'</span>, GC_LVT), (<span class="string">'\u{ae14}'</span>, <span class="string">'\u{ae14}'</span>, GC_LV), (<span class="string">'\u{ae15}'</span>, <span class="string">'\u{ae2f}'</span>,
        GC_LVT), (<span class="string">'\u{ae30}'</span>, <span class="string">'\u{ae30}'</span>, GC_LV), (<span class="string">'\u{ae31}'</span>, <span class="string">'\u{ae4b}'</span>, GC_LVT), (<span class="string">'\u{ae4c}'</span>,
        <span class="string">'\u{ae4c}'</span>, GC_LV), (<span class="string">'\u{ae4d}'</span>, <span class="string">'\u{ae67}'</span>, GC_LVT), (<span class="string">'\u{ae68}'</span>, <span class="string">'\u{ae68}'</span>, GC_LV),
        (<span class="string">'\u{ae69}'</span>, <span class="string">'\u{ae83}'</span>, GC_LVT), (<span class="string">'\u{ae84}'</span>, <span class="string">'\u{ae84}'</span>, GC_LV), (<span class="string">'\u{ae85}'</span>, <span class="string">'\u{ae9f}'</span>,
        GC_LVT), (<span class="string">'\u{aea0}'</span>, <span class="string">'\u{aea0}'</span>, GC_LV), (<span class="string">'\u{aea1}'</span>, <span class="string">'\u{aebb}'</span>, GC_LVT), (<span class="string">'\u{aebc}'</span>,
        <span class="string">'\u{aebc}'</span>, GC_LV), (<span class="string">'\u{aebd}'</span>, <span class="string">'\u{aed7}'</span>, GC_LVT), (<span class="string">'\u{aed8}'</span>, <span class="string">'\u{aed8}'</span>, GC_LV),
        (<span class="string">'\u{aed9}'</span>, <span class="string">'\u{aef3}'</span>, GC_LVT), (<span class="string">'\u{aef4}'</span>, <span class="string">'\u{aef4}'</span>, GC_LV), (<span class="string">'\u{aef5}'</span>, <span class="string">'\u{af0f}'</span>,
        GC_LVT), (<span class="string">'\u{af10}'</span>, <span class="string">'\u{af10}'</span>, GC_LV), (<span class="string">'\u{af11}'</span>, <span class="string">'\u{af2b}'</span>, GC_LVT), (<span class="string">'\u{af2c}'</span>,
        <span class="string">'\u{af2c}'</span>, GC_LV), (<span class="string">'\u{af2d}'</span>, <span class="string">'\u{af47}'</span>, GC_LVT), (<span class="string">'\u{af48}'</span>, <span class="string">'\u{af48}'</span>, GC_LV),
        (<span class="string">'\u{af49}'</span>, <span class="string">'\u{af63}'</span>, GC_LVT), (<span class="string">'\u{af64}'</span>, <span class="string">'\u{af64}'</span>, GC_LV), (<span class="string">'\u{af65}'</span>, <span class="string">'\u{af7f}'</span>,
        GC_LVT), (<span class="string">'\u{af80}'</span>, <span class="string">'\u{af80}'</span>, GC_LV), (<span class="string">'\u{af81}'</span>, <span class="string">'\u{af9b}'</span>, GC_LVT), (<span class="string">'\u{af9c}'</span>,
        <span class="string">'\u{af9c}'</span>, GC_LV), (<span class="string">'\u{af9d}'</span>, <span class="string">'\u{afb7}'</span>, GC_LVT), (<span class="string">'\u{afb8}'</span>, <span class="string">'\u{afb8}'</span>, GC_LV),
        (<span class="string">'\u{afb9}'</span>, <span class="string">'\u{afd3}'</span>, GC_LVT), (<span class="string">'\u{afd4}'</span>, <span class="string">'\u{afd4}'</span>, GC_LV), (<span class="string">'\u{afd5}'</span>, <span class="string">'\u{afef}'</span>,
        GC_LVT), (<span class="string">'\u{aff0}'</span>, <span class="string">'\u{aff0}'</span>, GC_LV), (<span class="string">'\u{aff1}'</span>, <span class="string">'\u{b00b}'</span>, GC_LVT), (<span class="string">'\u{b00c}'</span>,
        <span class="string">'\u{b00c}'</span>, GC_LV), (<span class="string">'\u{b00d}'</span>, <span class="string">'\u{b027}'</span>, GC_LVT), (<span class="string">'\u{b028}'</span>, <span class="string">'\u{b028}'</span>, GC_LV),
        (<span class="string">'\u{b029}'</span>, <span class="string">'\u{b043}'</span>, GC_LVT), (<span class="string">'\u{b044}'</span>, <span class="string">'\u{b044}'</span>, GC_LV), (<span class="string">'\u{b045}'</span>, <span class="string">'\u{b05f}'</span>,
        GC_LVT), (<span class="string">'\u{b060}'</span>, <span class="string">'\u{b060}'</span>, GC_LV), (<span class="string">'\u{b061}'</span>, <span class="string">'\u{b07b}'</span>, GC_LVT), (<span class="string">'\u{b07c}'</span>,
        <span class="string">'\u{b07c}'</span>, GC_LV), (<span class="string">'\u{b07d}'</span>, <span class="string">'\u{b097}'</span>, GC_LVT), (<span class="string">'\u{b098}'</span>, <span class="string">'\u{b098}'</span>, GC_LV),
        (<span class="string">'\u{b099}'</span>, <span class="string">'\u{b0b3}'</span>, GC_LVT), (<span class="string">'\u{b0b4}'</span>, <span class="string">'\u{b0b4}'</span>, GC_LV), (<span class="string">'\u{b0b5}'</span>, <span class="string">'\u{b0cf}'</span>,
        GC_LVT), (<span class="string">'\u{b0d0}'</span>, <span class="string">'\u{b0d0}'</span>, GC_LV), (<span class="string">'\u{b0d1}'</span>, <span class="string">'\u{b0eb}'</span>, GC_LVT), (<span class="string">'\u{b0ec}'</span>,
        <span class="string">'\u{b0ec}'</span>, GC_LV), (<span class="string">'\u{b0ed}'</span>, <span class="string">'\u{b107}'</span>, GC_LVT), (<span class="string">'\u{b108}'</span>, <span class="string">'\u{b108}'</span>, GC_LV),
        (<span class="string">'\u{b109}'</span>, <span class="string">'\u{b123}'</span>, GC_LVT), (<span class="string">'\u{b124}'</span>, <span class="string">'\u{b124}'</span>, GC_LV), (<span class="string">'\u{b125}'</span>, <span class="string">'\u{b13f}'</span>,
        GC_LVT), (<span class="string">'\u{b140}'</span>, <span class="string">'\u{b140}'</span>, GC_LV), (<span class="string">'\u{b141}'</span>, <span class="string">'\u{b15b}'</span>, GC_LVT), (<span class="string">'\u{b15c}'</span>,
        <span class="string">'\u{b15c}'</span>, GC_LV), (<span class="string">'\u{b15d}'</span>, <span class="string">'\u{b177}'</span>, GC_LVT), (<span class="string">'\u{b178}'</span>, <span class="string">'\u{b178}'</span>, GC_LV),
        (<span class="string">'\u{b179}'</span>, <span class="string">'\u{b193}'</span>, GC_LVT), (<span class="string">'\u{b194}'</span>, <span class="string">'\u{b194}'</span>, GC_LV), (<span class="string">'\u{b195}'</span>, <span class="string">'\u{b1af}'</span>,
        GC_LVT), (<span class="string">'\u{b1b0}'</span>, <span class="string">'\u{b1b0}'</span>, GC_LV), (<span class="string">'\u{b1b1}'</span>, <span class="string">'\u{b1cb}'</span>, GC_LVT), (<span class="string">'\u{b1cc}'</span>,
        <span class="string">'\u{b1cc}'</span>, GC_LV), (<span class="string">'\u{b1cd}'</span>, <span class="string">'\u{b1e7}'</span>, GC_LVT), (<span class="string">'\u{b1e8}'</span>, <span class="string">'\u{b1e8}'</span>, GC_LV),
        (<span class="string">'\u{b1e9}'</span>, <span class="string">'\u{b203}'</span>, GC_LVT), (<span class="string">'\u{b204}'</span>, <span class="string">'\u{b204}'</span>, GC_LV), (<span class="string">'\u{b205}'</span>, <span class="string">'\u{b21f}'</span>,
        GC_LVT), (<span class="string">'\u{b220}'</span>, <span class="string">'\u{b220}'</span>, GC_LV), (<span class="string">'\u{b221}'</span>, <span class="string">'\u{b23b}'</span>, GC_LVT), (<span class="string">'\u{b23c}'</span>,
        <span class="string">'\u{b23c}'</span>, GC_LV), (<span class="string">'\u{b23d}'</span>, <span class="string">'\u{b257}'</span>, GC_LVT), (<span class="string">'\u{b258}'</span>, <span class="string">'\u{b258}'</span>, GC_LV),
        (<span class="string">'\u{b259}'</span>, <span class="string">'\u{b273}'</span>, GC_LVT), (<span class="string">'\u{b274}'</span>, <span class="string">'\u{b274}'</span>, GC_LV), (<span class="string">'\u{b275}'</span>, <span class="string">'\u{b28f}'</span>,
        GC_LVT), (<span class="string">'\u{b290}'</span>, <span class="string">'\u{b290}'</span>, GC_LV), (<span class="string">'\u{b291}'</span>, <span class="string">'\u{b2ab}'</span>, GC_LVT), (<span class="string">'\u{b2ac}'</span>,
        <span class="string">'\u{b2ac}'</span>, GC_LV), (<span class="string">'\u{b2ad}'</span>, <span class="string">'\u{b2c7}'</span>, GC_LVT), (<span class="string">'\u{b2c8}'</span>, <span class="string">'\u{b2c8}'</span>, GC_LV),
        (<span class="string">'\u{b2c9}'</span>, <span class="string">'\u{b2e3}'</span>, GC_LVT), (<span class="string">'\u{b2e4}'</span>, <span class="string">'\u{b2e4}'</span>, GC_LV), (<span class="string">'\u{b2e5}'</span>, <span class="string">'\u{b2ff}'</span>,
        GC_LVT), (<span class="string">'\u{b300}'</span>, <span class="string">'\u{b300}'</span>, GC_LV), (<span class="string">'\u{b301}'</span>, <span class="string">'\u{b31b}'</span>, GC_LVT), (<span class="string">'\u{b31c}'</span>,
        <span class="string">'\u{b31c}'</span>, GC_LV), (<span class="string">'\u{b31d}'</span>, <span class="string">'\u{b337}'</span>, GC_LVT), (<span class="string">'\u{b338}'</span>, <span class="string">'\u{b338}'</span>, GC_LV),
        (<span class="string">'\u{b339}'</span>, <span class="string">'\u{b353}'</span>, GC_LVT), (<span class="string">'\u{b354}'</span>, <span class="string">'\u{b354}'</span>, GC_LV), (<span class="string">'\u{b355}'</span>, <span class="string">'\u{b36f}'</span>,
        GC_LVT), (<span class="string">'\u{b370}'</span>, <span class="string">'\u{b370}'</span>, GC_LV), (<span class="string">'\u{b371}'</span>, <span class="string">'\u{b38b}'</span>, GC_LVT), (<span class="string">'\u{b38c}'</span>,
        <span class="string">'\u{b38c}'</span>, GC_LV), (<span class="string">'\u{b38d}'</span>, <span class="string">'\u{b3a7}'</span>, GC_LVT), (<span class="string">'\u{b3a8}'</span>, <span class="string">'\u{b3a8}'</span>, GC_LV),
        (<span class="string">'\u{b3a9}'</span>, <span class="string">'\u{b3c3}'</span>, GC_LVT), (<span class="string">'\u{b3c4}'</span>, <span class="string">'\u{b3c4}'</span>, GC_LV), (<span class="string">'\u{b3c5}'</span>, <span class="string">'\u{b3df}'</span>,
        GC_LVT), (<span class="string">'\u{b3e0}'</span>, <span class="string">'\u{b3e0}'</span>, GC_LV), (<span class="string">'\u{b3e1}'</span>, <span class="string">'\u{b3fb}'</span>, GC_LVT), (<span class="string">'\u{b3fc}'</span>,
        <span class="string">'\u{b3fc}'</span>, GC_LV), (<span class="string">'\u{b3fd}'</span>, <span class="string">'\u{b417}'</span>, GC_LVT), (<span class="string">'\u{b418}'</span>, <span class="string">'\u{b418}'</span>, GC_LV),
        (<span class="string">'\u{b419}'</span>, <span class="string">'\u{b433}'</span>, GC_LVT), (<span class="string">'\u{b434}'</span>, <span class="string">'\u{b434}'</span>, GC_LV), (<span class="string">'\u{b435}'</span>, <span class="string">'\u{b44f}'</span>,
        GC_LVT), (<span class="string">'\u{b450}'</span>, <span class="string">'\u{b450}'</span>, GC_LV), (<span class="string">'\u{b451}'</span>, <span class="string">'\u{b46b}'</span>, GC_LVT), (<span class="string">'\u{b46c}'</span>,
        <span class="string">'\u{b46c}'</span>, GC_LV), (<span class="string">'\u{b46d}'</span>, <span class="string">'\u{b487}'</span>, GC_LVT), (<span class="string">'\u{b488}'</span>, <span class="string">'\u{b488}'</span>, GC_LV),
        (<span class="string">'\u{b489}'</span>, <span class="string">'\u{b4a3}'</span>, GC_LVT), (<span class="string">'\u{b4a4}'</span>, <span class="string">'\u{b4a4}'</span>, GC_LV), (<span class="string">'\u{b4a5}'</span>, <span class="string">'\u{b4bf}'</span>,
        GC_LVT), (<span class="string">'\u{b4c0}'</span>, <span class="string">'\u{b4c0}'</span>, GC_LV), (<span class="string">'\u{b4c1}'</span>, <span class="string">'\u{b4db}'</span>, GC_LVT), (<span class="string">'\u{b4dc}'</span>,
        <span class="string">'\u{b4dc}'</span>, GC_LV), (<span class="string">'\u{b4dd}'</span>, <span class="string">'\u{b4f7}'</span>, GC_LVT), (<span class="string">'\u{b4f8}'</span>, <span class="string">'\u{b4f8}'</span>, GC_LV),
        (<span class="string">'\u{b4f9}'</span>, <span class="string">'\u{b513}'</span>, GC_LVT), (<span class="string">'\u{b514}'</span>, <span class="string">'\u{b514}'</span>, GC_LV), (<span class="string">'\u{b515}'</span>, <span class="string">'\u{b52f}'</span>,
        GC_LVT), (<span class="string">'\u{b530}'</span>, <span class="string">'\u{b530}'</span>, GC_LV), (<span class="string">'\u{b531}'</span>, <span class="string">'\u{b54b}'</span>, GC_LVT), (<span class="string">'\u{b54c}'</span>,
        <span class="string">'\u{b54c}'</span>, GC_LV), (<span class="string">'\u{b54d}'</span>, <span class="string">'\u{b567}'</span>, GC_LVT), (<span class="string">'\u{b568}'</span>, <span class="string">'\u{b568}'</span>, GC_LV),
        (<span class="string">'\u{b569}'</span>, <span class="string">'\u{b583}'</span>, GC_LVT), (<span class="string">'\u{b584}'</span>, <span class="string">'\u{b584}'</span>, GC_LV), (<span class="string">'\u{b585}'</span>, <span class="string">'\u{b59f}'</span>,
        GC_LVT), (<span class="string">'\u{b5a0}'</span>, <span class="string">'\u{b5a0}'</span>, GC_LV), (<span class="string">'\u{b5a1}'</span>, <span class="string">'\u{b5bb}'</span>, GC_LVT), (<span class="string">'\u{b5bc}'</span>,
        <span class="string">'\u{b5bc}'</span>, GC_LV), (<span class="string">'\u{b5bd}'</span>, <span class="string">'\u{b5d7}'</span>, GC_LVT), (<span class="string">'\u{b5d8}'</span>, <span class="string">'\u{b5d8}'</span>, GC_LV),
        (<span class="string">'\u{b5d9}'</span>, <span class="string">'\u{b5f3}'</span>, GC_LVT), (<span class="string">'\u{b5f4}'</span>, <span class="string">'\u{b5f4}'</span>, GC_LV), (<span class="string">'\u{b5f5}'</span>, <span class="string">'\u{b60f}'</span>,
        GC_LVT), (<span class="string">'\u{b610}'</span>, <span class="string">'\u{b610}'</span>, GC_LV), (<span class="string">'\u{b611}'</span>, <span class="string">'\u{b62b}'</span>, GC_LVT), (<span class="string">'\u{b62c}'</span>,
        <span class="string">'\u{b62c}'</span>, GC_LV), (<span class="string">'\u{b62d}'</span>, <span class="string">'\u{b647}'</span>, GC_LVT), (<span class="string">'\u{b648}'</span>, <span class="string">'\u{b648}'</span>, GC_LV),
        (<span class="string">'\u{b649}'</span>, <span class="string">'\u{b663}'</span>, GC_LVT), (<span class="string">'\u{b664}'</span>, <span class="string">'\u{b664}'</span>, GC_LV), (<span class="string">'\u{b665}'</span>, <span class="string">'\u{b67f}'</span>,
        GC_LVT), (<span class="string">'\u{b680}'</span>, <span class="string">'\u{b680}'</span>, GC_LV), (<span class="string">'\u{b681}'</span>, <span class="string">'\u{b69b}'</span>, GC_LVT), (<span class="string">'\u{b69c}'</span>,
        <span class="string">'\u{b69c}'</span>, GC_LV), (<span class="string">'\u{b69d}'</span>, <span class="string">'\u{b6b7}'</span>, GC_LVT), (<span class="string">'\u{b6b8}'</span>, <span class="string">'\u{b6b8}'</span>, GC_LV),
        (<span class="string">'\u{b6b9}'</span>, <span class="string">'\u{b6d3}'</span>, GC_LVT), (<span class="string">'\u{b6d4}'</span>, <span class="string">'\u{b6d4}'</span>, GC_LV), (<span class="string">'\u{b6d5}'</span>, <span class="string">'\u{b6ef}'</span>,
        GC_LVT), (<span class="string">'\u{b6f0}'</span>, <span class="string">'\u{b6f0}'</span>, GC_LV), (<span class="string">'\u{b6f1}'</span>, <span class="string">'\u{b70b}'</span>, GC_LVT), (<span class="string">'\u{b70c}'</span>,
        <span class="string">'\u{b70c}'</span>, GC_LV), (<span class="string">'\u{b70d}'</span>, <span class="string">'\u{b727}'</span>, GC_LVT), (<span class="string">'\u{b728}'</span>, <span class="string">'\u{b728}'</span>, GC_LV),
        (<span class="string">'\u{b729}'</span>, <span class="string">'\u{b743}'</span>, GC_LVT), (<span class="string">'\u{b744}'</span>, <span class="string">'\u{b744}'</span>, GC_LV), (<span class="string">'\u{b745}'</span>, <span class="string">'\u{b75f}'</span>,
        GC_LVT), (<span class="string">'\u{b760}'</span>, <span class="string">'\u{b760}'</span>, GC_LV), (<span class="string">'\u{b761}'</span>, <span class="string">'\u{b77b}'</span>, GC_LVT), (<span class="string">'\u{b77c}'</span>,
        <span class="string">'\u{b77c}'</span>, GC_LV), (<span class="string">'\u{b77d}'</span>, <span class="string">'\u{b797}'</span>, GC_LVT), (<span class="string">'\u{b798}'</span>, <span class="string">'\u{b798}'</span>, GC_LV),
        (<span class="string">'\u{b799}'</span>, <span class="string">'\u{b7b3}'</span>, GC_LVT), (<span class="string">'\u{b7b4}'</span>, <span class="string">'\u{b7b4}'</span>, GC_LV), (<span class="string">'\u{b7b5}'</span>, <span class="string">'\u{b7cf}'</span>,
        GC_LVT), (<span class="string">'\u{b7d0}'</span>, <span class="string">'\u{b7d0}'</span>, GC_LV), (<span class="string">'\u{b7d1}'</span>, <span class="string">'\u{b7eb}'</span>, GC_LVT), (<span class="string">'\u{b7ec}'</span>,
        <span class="string">'\u{b7ec}'</span>, GC_LV), (<span class="string">'\u{b7ed}'</span>, <span class="string">'\u{b807}'</span>, GC_LVT), (<span class="string">'\u{b808}'</span>, <span class="string">'\u{b808}'</span>, GC_LV),
        (<span class="string">'\u{b809}'</span>, <span class="string">'\u{b823}'</span>, GC_LVT), (<span class="string">'\u{b824}'</span>, <span class="string">'\u{b824}'</span>, GC_LV), (<span class="string">'\u{b825}'</span>, <span class="string">'\u{b83f}'</span>,
        GC_LVT), (<span class="string">'\u{b840}'</span>, <span class="string">'\u{b840}'</span>, GC_LV), (<span class="string">'\u{b841}'</span>, <span class="string">'\u{b85b}'</span>, GC_LVT), (<span class="string">'\u{b85c}'</span>,
        <span class="string">'\u{b85c}'</span>, GC_LV), (<span class="string">'\u{b85d}'</span>, <span class="string">'\u{b877}'</span>, GC_LVT), (<span class="string">'\u{b878}'</span>, <span class="string">'\u{b878}'</span>, GC_LV),
        (<span class="string">'\u{b879}'</span>, <span class="string">'\u{b893}'</span>, GC_LVT), (<span class="string">'\u{b894}'</span>, <span class="string">'\u{b894}'</span>, GC_LV), (<span class="string">'\u{b895}'</span>, <span class="string">'\u{b8af}'</span>,
        GC_LVT), (<span class="string">'\u{b8b0}'</span>, <span class="string">'\u{b8b0}'</span>, GC_LV), (<span class="string">'\u{b8b1}'</span>, <span class="string">'\u{b8cb}'</span>, GC_LVT), (<span class="string">'\u{b8cc}'</span>,
        <span class="string">'\u{b8cc}'</span>, GC_LV), (<span class="string">'\u{b8cd}'</span>, <span class="string">'\u{b8e7}'</span>, GC_LVT), (<span class="string">'\u{b8e8}'</span>, <span class="string">'\u{b8e8}'</span>, GC_LV),
        (<span class="string">'\u{b8e9}'</span>, <span class="string">'\u{b903}'</span>, GC_LVT), (<span class="string">'\u{b904}'</span>, <span class="string">'\u{b904}'</span>, GC_LV), (<span class="string">'\u{b905}'</span>, <span class="string">'\u{b91f}'</span>,
        GC_LVT), (<span class="string">'\u{b920}'</span>, <span class="string">'\u{b920}'</span>, GC_LV), (<span class="string">'\u{b921}'</span>, <span class="string">'\u{b93b}'</span>, GC_LVT), (<span class="string">'\u{b93c}'</span>,
        <span class="string">'\u{b93c}'</span>, GC_LV), (<span class="string">'\u{b93d}'</span>, <span class="string">'\u{b957}'</span>, GC_LVT), (<span class="string">'\u{b958}'</span>, <span class="string">'\u{b958}'</span>, GC_LV),
        (<span class="string">'\u{b959}'</span>, <span class="string">'\u{b973}'</span>, GC_LVT), (<span class="string">'\u{b974}'</span>, <span class="string">'\u{b974}'</span>, GC_LV), (<span class="string">'\u{b975}'</span>, <span class="string">'\u{b98f}'</span>,
        GC_LVT), (<span class="string">'\u{b990}'</span>, <span class="string">'\u{b990}'</span>, GC_LV), (<span class="string">'\u{b991}'</span>, <span class="string">'\u{b9ab}'</span>, GC_LVT), (<span class="string">'\u{b9ac}'</span>,
        <span class="string">'\u{b9ac}'</span>, GC_LV), (<span class="string">'\u{b9ad}'</span>, <span class="string">'\u{b9c7}'</span>, GC_LVT), (<span class="string">'\u{b9c8}'</span>, <span class="string">'\u{b9c8}'</span>, GC_LV),
        (<span class="string">'\u{b9c9}'</span>, <span class="string">'\u{b9e3}'</span>, GC_LVT), (<span class="string">'\u{b9e4}'</span>, <span class="string">'\u{b9e4}'</span>, GC_LV), (<span class="string">'\u{b9e5}'</span>, <span class="string">'\u{b9ff}'</span>,
        GC_LVT), (<span class="string">'\u{ba00}'</span>, <span class="string">'\u{ba00}'</span>, GC_LV), (<span class="string">'\u{ba01}'</span>, <span class="string">'\u{ba1b}'</span>, GC_LVT), (<span class="string">'\u{ba1c}'</span>,
        <span class="string">'\u{ba1c}'</span>, GC_LV), (<span class="string">'\u{ba1d}'</span>, <span class="string">'\u{ba37}'</span>, GC_LVT), (<span class="string">'\u{ba38}'</span>, <span class="string">'\u{ba38}'</span>, GC_LV),
        (<span class="string">'\u{ba39}'</span>, <span class="string">'\u{ba53}'</span>, GC_LVT), (<span class="string">'\u{ba54}'</span>, <span class="string">'\u{ba54}'</span>, GC_LV), (<span class="string">'\u{ba55}'</span>, <span class="string">'\u{ba6f}'</span>,
        GC_LVT), (<span class="string">'\u{ba70}'</span>, <span class="string">'\u{ba70}'</span>, GC_LV), (<span class="string">'\u{ba71}'</span>, <span class="string">'\u{ba8b}'</span>, GC_LVT), (<span class="string">'\u{ba8c}'</span>,
        <span class="string">'\u{ba8c}'</span>, GC_LV), (<span class="string">'\u{ba8d}'</span>, <span class="string">'\u{baa7}'</span>, GC_LVT), (<span class="string">'\u{baa8}'</span>, <span class="string">'\u{baa8}'</span>, GC_LV),
        (<span class="string">'\u{baa9}'</span>, <span class="string">'\u{bac3}'</span>, GC_LVT), (<span class="string">'\u{bac4}'</span>, <span class="string">'\u{bac4}'</span>, GC_LV), (<span class="string">'\u{bac5}'</span>, <span class="string">'\u{badf}'</span>,
        GC_LVT), (<span class="string">'\u{bae0}'</span>, <span class="string">'\u{bae0}'</span>, GC_LV), (<span class="string">'\u{bae1}'</span>, <span class="string">'\u{bafb}'</span>, GC_LVT), (<span class="string">'\u{bafc}'</span>,
        <span class="string">'\u{bafc}'</span>, GC_LV), (<span class="string">'\u{bafd}'</span>, <span class="string">'\u{bb17}'</span>, GC_LVT), (<span class="string">'\u{bb18}'</span>, <span class="string">'\u{bb18}'</span>, GC_LV),
        (<span class="string">'\u{bb19}'</span>, <span class="string">'\u{bb33}'</span>, GC_LVT), (<span class="string">'\u{bb34}'</span>, <span class="string">'\u{bb34}'</span>, GC_LV), (<span class="string">'\u{bb35}'</span>, <span class="string">'\u{bb4f}'</span>,
        GC_LVT), (<span class="string">'\u{bb50}'</span>, <span class="string">'\u{bb50}'</span>, GC_LV), (<span class="string">'\u{bb51}'</span>, <span class="string">'\u{bb6b}'</span>, GC_LVT), (<span class="string">'\u{bb6c}'</span>,
        <span class="string">'\u{bb6c}'</span>, GC_LV), (<span class="string">'\u{bb6d}'</span>, <span class="string">'\u{bb87}'</span>, GC_LVT), (<span class="string">'\u{bb88}'</span>, <span class="string">'\u{bb88}'</span>, GC_LV),
        (<span class="string">'\u{bb89}'</span>, <span class="string">'\u{bba3}'</span>, GC_LVT), (<span class="string">'\u{bba4}'</span>, <span class="string">'\u{bba4}'</span>, GC_LV), (<span class="string">'\u{bba5}'</span>, <span class="string">'\u{bbbf}'</span>,
        GC_LVT), (<span class="string">'\u{bbc0}'</span>, <span class="string">'\u{bbc0}'</span>, GC_LV), (<span class="string">'\u{bbc1}'</span>, <span class="string">'\u{bbdb}'</span>, GC_LVT), (<span class="string">'\u{bbdc}'</span>,
        <span class="string">'\u{bbdc}'</span>, GC_LV), (<span class="string">'\u{bbdd}'</span>, <span class="string">'\u{bbf7}'</span>, GC_LVT), (<span class="string">'\u{bbf8}'</span>, <span class="string">'\u{bbf8}'</span>, GC_LV),
        (<span class="string">'\u{bbf9}'</span>, <span class="string">'\u{bc13}'</span>, GC_LVT), (<span class="string">'\u{bc14}'</span>, <span class="string">'\u{bc14}'</span>, GC_LV), (<span class="string">'\u{bc15}'</span>, <span class="string">'\u{bc2f}'</span>,
        GC_LVT), (<span class="string">'\u{bc30}'</span>, <span class="string">'\u{bc30}'</span>, GC_LV), (<span class="string">'\u{bc31}'</span>, <span class="string">'\u{bc4b}'</span>, GC_LVT), (<span class="string">'\u{bc4c}'</span>,
        <span class="string">'\u{bc4c}'</span>, GC_LV), (<span class="string">'\u{bc4d}'</span>, <span class="string">'\u{bc67}'</span>, GC_LVT), (<span class="string">'\u{bc68}'</span>, <span class="string">'\u{bc68}'</span>, GC_LV),
        (<span class="string">'\u{bc69}'</span>, <span class="string">'\u{bc83}'</span>, GC_LVT), (<span class="string">'\u{bc84}'</span>, <span class="string">'\u{bc84}'</span>, GC_LV), (<span class="string">'\u{bc85}'</span>, <span class="string">'\u{bc9f}'</span>,
        GC_LVT), (<span class="string">'\u{bca0}'</span>, <span class="string">'\u{bca0}'</span>, GC_LV), (<span class="string">'\u{bca1}'</span>, <span class="string">'\u{bcbb}'</span>, GC_LVT), (<span class="string">'\u{bcbc}'</span>,
        <span class="string">'\u{bcbc}'</span>, GC_LV), (<span class="string">'\u{bcbd}'</span>, <span class="string">'\u{bcd7}'</span>, GC_LVT), (<span class="string">'\u{bcd8}'</span>, <span class="string">'\u{bcd8}'</span>, GC_LV),
        (<span class="string">'\u{bcd9}'</span>, <span class="string">'\u{bcf3}'</span>, GC_LVT), (<span class="string">'\u{bcf4}'</span>, <span class="string">'\u{bcf4}'</span>, GC_LV), (<span class="string">'\u{bcf5}'</span>, <span class="string">'\u{bd0f}'</span>,
        GC_LVT), (<span class="string">'\u{bd10}'</span>, <span class="string">'\u{bd10}'</span>, GC_LV), (<span class="string">'\u{bd11}'</span>, <span class="string">'\u{bd2b}'</span>, GC_LVT), (<span class="string">'\u{bd2c}'</span>,
        <span class="string">'\u{bd2c}'</span>, GC_LV), (<span class="string">'\u{bd2d}'</span>, <span class="string">'\u{bd47}'</span>, GC_LVT), (<span class="string">'\u{bd48}'</span>, <span class="string">'\u{bd48}'</span>, GC_LV),
        (<span class="string">'\u{bd49}'</span>, <span class="string">'\u{bd63}'</span>, GC_LVT), (<span class="string">'\u{bd64}'</span>, <span class="string">'\u{bd64}'</span>, GC_LV), (<span class="string">'\u{bd65}'</span>, <span class="string">'\u{bd7f}'</span>,
        GC_LVT), (<span class="string">'\u{bd80}'</span>, <span class="string">'\u{bd80}'</span>, GC_LV), (<span class="string">'\u{bd81}'</span>, <span class="string">'\u{bd9b}'</span>, GC_LVT), (<span class="string">'\u{bd9c}'</span>,
        <span class="string">'\u{bd9c}'</span>, GC_LV), (<span class="string">'\u{bd9d}'</span>, <span class="string">'\u{bdb7}'</span>, GC_LVT), (<span class="string">'\u{bdb8}'</span>, <span class="string">'\u{bdb8}'</span>, GC_LV),
        (<span class="string">'\u{bdb9}'</span>, <span class="string">'\u{bdd3}'</span>, GC_LVT), (<span class="string">'\u{bdd4}'</span>, <span class="string">'\u{bdd4}'</span>, GC_LV), (<span class="string">'\u{bdd5}'</span>, <span class="string">'\u{bdef}'</span>,
        GC_LVT), (<span class="string">'\u{bdf0}'</span>, <span class="string">'\u{bdf0}'</span>, GC_LV), (<span class="string">'\u{bdf1}'</span>, <span class="string">'\u{be0b}'</span>, GC_LVT), (<span class="string">'\u{be0c}'</span>,
        <span class="string">'\u{be0c}'</span>, GC_LV), (<span class="string">'\u{be0d}'</span>, <span class="string">'\u{be27}'</span>, GC_LVT), (<span class="string">'\u{be28}'</span>, <span class="string">'\u{be28}'</span>, GC_LV),
        (<span class="string">'\u{be29}'</span>, <span class="string">'\u{be43}'</span>, GC_LVT), (<span class="string">'\u{be44}'</span>, <span class="string">'\u{be44}'</span>, GC_LV), (<span class="string">'\u{be45}'</span>, <span class="string">'\u{be5f}'</span>,
        GC_LVT), (<span class="string">'\u{be60}'</span>, <span class="string">'\u{be60}'</span>, GC_LV), (<span class="string">'\u{be61}'</span>, <span class="string">'\u{be7b}'</span>, GC_LVT), (<span class="string">'\u{be7c}'</span>,
        <span class="string">'\u{be7c}'</span>, GC_LV), (<span class="string">'\u{be7d}'</span>, <span class="string">'\u{be97}'</span>, GC_LVT), (<span class="string">'\u{be98}'</span>, <span class="string">'\u{be98}'</span>, GC_LV),
        (<span class="string">'\u{be99}'</span>, <span class="string">'\u{beb3}'</span>, GC_LVT), (<span class="string">'\u{beb4}'</span>, <span class="string">'\u{beb4}'</span>, GC_LV), (<span class="string">'\u{beb5}'</span>, <span class="string">'\u{becf}'</span>,
        GC_LVT), (<span class="string">'\u{bed0}'</span>, <span class="string">'\u{bed0}'</span>, GC_LV), (<span class="string">'\u{bed1}'</span>, <span class="string">'\u{beeb}'</span>, GC_LVT), (<span class="string">'\u{beec}'</span>,
        <span class="string">'\u{beec}'</span>, GC_LV), (<span class="string">'\u{beed}'</span>, <span class="string">'\u{bf07}'</span>, GC_LVT), (<span class="string">'\u{bf08}'</span>, <span class="string">'\u{bf08}'</span>, GC_LV),
        (<span class="string">'\u{bf09}'</span>, <span class="string">'\u{bf23}'</span>, GC_LVT), (<span class="string">'\u{bf24}'</span>, <span class="string">'\u{bf24}'</span>, GC_LV), (<span class="string">'\u{bf25}'</span>, <span class="string">'\u{bf3f}'</span>,
        GC_LVT), (<span class="string">'\u{bf40}'</span>, <span class="string">'\u{bf40}'</span>, GC_LV), (<span class="string">'\u{bf41}'</span>, <span class="string">'\u{bf5b}'</span>, GC_LVT), (<span class="string">'\u{bf5c}'</span>,
        <span class="string">'\u{bf5c}'</span>, GC_LV), (<span class="string">'\u{bf5d}'</span>, <span class="string">'\u{bf77}'</span>, GC_LVT), (<span class="string">'\u{bf78}'</span>, <span class="string">'\u{bf78}'</span>, GC_LV),
        (<span class="string">'\u{bf79}'</span>, <span class="string">'\u{bf93}'</span>, GC_LVT), (<span class="string">'\u{bf94}'</span>, <span class="string">'\u{bf94}'</span>, GC_LV), (<span class="string">'\u{bf95}'</span>, <span class="string">'\u{bfaf}'</span>,
        GC_LVT), (<span class="string">'\u{bfb0}'</span>, <span class="string">'\u{bfb0}'</span>, GC_LV), (<span class="string">'\u{bfb1}'</span>, <span class="string">'\u{bfcb}'</span>, GC_LVT), (<span class="string">'\u{bfcc}'</span>,
        <span class="string">'\u{bfcc}'</span>, GC_LV), (<span class="string">'\u{bfcd}'</span>, <span class="string">'\u{bfe7}'</span>, GC_LVT), (<span class="string">'\u{bfe8}'</span>, <span class="string">'\u{bfe8}'</span>, GC_LV),
        (<span class="string">'\u{bfe9}'</span>, <span class="string">'\u{c003}'</span>, GC_LVT), (<span class="string">'\u{c004}'</span>, <span class="string">'\u{c004}'</span>, GC_LV), (<span class="string">'\u{c005}'</span>, <span class="string">'\u{c01f}'</span>,
        GC_LVT), (<span class="string">'\u{c020}'</span>, <span class="string">'\u{c020}'</span>, GC_LV), (<span class="string">'\u{c021}'</span>, <span class="string">'\u{c03b}'</span>, GC_LVT), (<span class="string">'\u{c03c}'</span>,
        <span class="string">'\u{c03c}'</span>, GC_LV), (<span class="string">'\u{c03d}'</span>, <span class="string">'\u{c057}'</span>, GC_LVT), (<span class="string">'\u{c058}'</span>, <span class="string">'\u{c058}'</span>, GC_LV),
        (<span class="string">'\u{c059}'</span>, <span class="string">'\u{c073}'</span>, GC_LVT), (<span class="string">'\u{c074}'</span>, <span class="string">'\u{c074}'</span>, GC_LV), (<span class="string">'\u{c075}'</span>, <span class="string">'\u{c08f}'</span>,
        GC_LVT), (<span class="string">'\u{c090}'</span>, <span class="string">'\u{c090}'</span>, GC_LV), (<span class="string">'\u{c091}'</span>, <span class="string">'\u{c0ab}'</span>, GC_LVT), (<span class="string">'\u{c0ac}'</span>,
        <span class="string">'\u{c0ac}'</span>, GC_LV), (<span class="string">'\u{c0ad}'</span>, <span class="string">'\u{c0c7}'</span>, GC_LVT), (<span class="string">'\u{c0c8}'</span>, <span class="string">'\u{c0c8}'</span>, GC_LV),
        (<span class="string">'\u{c0c9}'</span>, <span class="string">'\u{c0e3}'</span>, GC_LVT), (<span class="string">'\u{c0e4}'</span>, <span class="string">'\u{c0e4}'</span>, GC_LV), (<span class="string">'\u{c0e5}'</span>, <span class="string">'\u{c0ff}'</span>,
        GC_LVT), (<span class="string">'\u{c100}'</span>, <span class="string">'\u{c100}'</span>, GC_LV), (<span class="string">'\u{c101}'</span>, <span class="string">'\u{c11b}'</span>, GC_LVT), (<span class="string">'\u{c11c}'</span>,
        <span class="string">'\u{c11c}'</span>, GC_LV), (<span class="string">'\u{c11d}'</span>, <span class="string">'\u{c137}'</span>, GC_LVT), (<span class="string">'\u{c138}'</span>, <span class="string">'\u{c138}'</span>, GC_LV),
        (<span class="string">'\u{c139}'</span>, <span class="string">'\u{c153}'</span>, GC_LVT), (<span class="string">'\u{c154}'</span>, <span class="string">'\u{c154}'</span>, GC_LV), (<span class="string">'\u{c155}'</span>, <span class="string">'\u{c16f}'</span>,
        GC_LVT), (<span class="string">'\u{c170}'</span>, <span class="string">'\u{c170}'</span>, GC_LV), (<span class="string">'\u{c171}'</span>, <span class="string">'\u{c18b}'</span>, GC_LVT), (<span class="string">'\u{c18c}'</span>,
        <span class="string">'\u{c18c}'</span>, GC_LV), (<span class="string">'\u{c18d}'</span>, <span class="string">'\u{c1a7}'</span>, GC_LVT), (<span class="string">'\u{c1a8}'</span>, <span class="string">'\u{c1a8}'</span>, GC_LV),
        (<span class="string">'\u{c1a9}'</span>, <span class="string">'\u{c1c3}'</span>, GC_LVT), (<span class="string">'\u{c1c4}'</span>, <span class="string">'\u{c1c4}'</span>, GC_LV), (<span class="string">'\u{c1c5}'</span>, <span class="string">'\u{c1df}'</span>,
        GC_LVT), (<span class="string">'\u{c1e0}'</span>, <span class="string">'\u{c1e0}'</span>, GC_LV), (<span class="string">'\u{c1e1}'</span>, <span class="string">'\u{c1fb}'</span>, GC_LVT), (<span class="string">'\u{c1fc}'</span>,
        <span class="string">'\u{c1fc}'</span>, GC_LV), (<span class="string">'\u{c1fd}'</span>, <span class="string">'\u{c217}'</span>, GC_LVT), (<span class="string">'\u{c218}'</span>, <span class="string">'\u{c218}'</span>, GC_LV),
        (<span class="string">'\u{c219}'</span>, <span class="string">'\u{c233}'</span>, GC_LVT), (<span class="string">'\u{c234}'</span>, <span class="string">'\u{c234}'</span>, GC_LV), (<span class="string">'\u{c235}'</span>, <span class="string">'\u{c24f}'</span>,
        GC_LVT), (<span class="string">'\u{c250}'</span>, <span class="string">'\u{c250}'</span>, GC_LV), (<span class="string">'\u{c251}'</span>, <span class="string">'\u{c26b}'</span>, GC_LVT), (<span class="string">'\u{c26c}'</span>,
        <span class="string">'\u{c26c}'</span>, GC_LV), (<span class="string">'\u{c26d}'</span>, <span class="string">'\u{c287}'</span>, GC_LVT), (<span class="string">'\u{c288}'</span>, <span class="string">'\u{c288}'</span>, GC_LV),
        (<span class="string">'\u{c289}'</span>, <span class="string">'\u{c2a3}'</span>, GC_LVT), (<span class="string">'\u{c2a4}'</span>, <span class="string">'\u{c2a4}'</span>, GC_LV), (<span class="string">'\u{c2a5}'</span>, <span class="string">'\u{c2bf}'</span>,
        GC_LVT), (<span class="string">'\u{c2c0}'</span>, <span class="string">'\u{c2c0}'</span>, GC_LV), (<span class="string">'\u{c2c1}'</span>, <span class="string">'\u{c2db}'</span>, GC_LVT), (<span class="string">'\u{c2dc}'</span>,
        <span class="string">'\u{c2dc}'</span>, GC_LV), (<span class="string">'\u{c2dd}'</span>, <span class="string">'\u{c2f7}'</span>, GC_LVT), (<span class="string">'\u{c2f8}'</span>, <span class="string">'\u{c2f8}'</span>, GC_LV),
        (<span class="string">'\u{c2f9}'</span>, <span class="string">'\u{c313}'</span>, GC_LVT), (<span class="string">'\u{c314}'</span>, <span class="string">'\u{c314}'</span>, GC_LV), (<span class="string">'\u{c315}'</span>, <span class="string">'\u{c32f}'</span>,
        GC_LVT), (<span class="string">'\u{c330}'</span>, <span class="string">'\u{c330}'</span>, GC_LV), (<span class="string">'\u{c331}'</span>, <span class="string">'\u{c34b}'</span>, GC_LVT), (<span class="string">'\u{c34c}'</span>,
        <span class="string">'\u{c34c}'</span>, GC_LV), (<span class="string">'\u{c34d}'</span>, <span class="string">'\u{c367}'</span>, GC_LVT), (<span class="string">'\u{c368}'</span>, <span class="string">'\u{c368}'</span>, GC_LV),
        (<span class="string">'\u{c369}'</span>, <span class="string">'\u{c383}'</span>, GC_LVT), (<span class="string">'\u{c384}'</span>, <span class="string">'\u{c384}'</span>, GC_LV), (<span class="string">'\u{c385}'</span>, <span class="string">'\u{c39f}'</span>,
        GC_LVT), (<span class="string">'\u{c3a0}'</span>, <span class="string">'\u{c3a0}'</span>, GC_LV), (<span class="string">'\u{c3a1}'</span>, <span class="string">'\u{c3bb}'</span>, GC_LVT), (<span class="string">'\u{c3bc}'</span>,
        <span class="string">'\u{c3bc}'</span>, GC_LV), (<span class="string">'\u{c3bd}'</span>, <span class="string">'\u{c3d7}'</span>, GC_LVT), (<span class="string">'\u{c3d8}'</span>, <span class="string">'\u{c3d8}'</span>, GC_LV),
        (<span class="string">'\u{c3d9}'</span>, <span class="string">'\u{c3f3}'</span>, GC_LVT), (<span class="string">'\u{c3f4}'</span>, <span class="string">'\u{c3f4}'</span>, GC_LV), (<span class="string">'\u{c3f5}'</span>, <span class="string">'\u{c40f}'</span>,
        GC_LVT), (<span class="string">'\u{c410}'</span>, <span class="string">'\u{c410}'</span>, GC_LV), (<span class="string">'\u{c411}'</span>, <span class="string">'\u{c42b}'</span>, GC_LVT), (<span class="string">'\u{c42c}'</span>,
        <span class="string">'\u{c42c}'</span>, GC_LV), (<span class="string">'\u{c42d}'</span>, <span class="string">'\u{c447}'</span>, GC_LVT), (<span class="string">'\u{c448}'</span>, <span class="string">'\u{c448}'</span>, GC_LV),
        (<span class="string">'\u{c449}'</span>, <span class="string">'\u{c463}'</span>, GC_LVT), (<span class="string">'\u{c464}'</span>, <span class="string">'\u{c464}'</span>, GC_LV), (<span class="string">'\u{c465}'</span>, <span class="string">'\u{c47f}'</span>,
        GC_LVT), (<span class="string">'\u{c480}'</span>, <span class="string">'\u{c480}'</span>, GC_LV), (<span class="string">'\u{c481}'</span>, <span class="string">'\u{c49b}'</span>, GC_LVT), (<span class="string">'\u{c49c}'</span>,
        <span class="string">'\u{c49c}'</span>, GC_LV), (<span class="string">'\u{c49d}'</span>, <span class="string">'\u{c4b7}'</span>, GC_LVT), (<span class="string">'\u{c4b8}'</span>, <span class="string">'\u{c4b8}'</span>, GC_LV),
        (<span class="string">'\u{c4b9}'</span>, <span class="string">'\u{c4d3}'</span>, GC_LVT), (<span class="string">'\u{c4d4}'</span>, <span class="string">'\u{c4d4}'</span>, GC_LV), (<span class="string">'\u{c4d5}'</span>, <span class="string">'\u{c4ef}'</span>,
        GC_LVT), (<span class="string">'\u{c4f0}'</span>, <span class="string">'\u{c4f0}'</span>, GC_LV), (<span class="string">'\u{c4f1}'</span>, <span class="string">'\u{c50b}'</span>, GC_LVT), (<span class="string">'\u{c50c}'</span>,
        <span class="string">'\u{c50c}'</span>, GC_LV), (<span class="string">'\u{c50d}'</span>, <span class="string">'\u{c527}'</span>, GC_LVT), (<span class="string">'\u{c528}'</span>, <span class="string">'\u{c528}'</span>, GC_LV),
        (<span class="string">'\u{c529}'</span>, <span class="string">'\u{c543}'</span>, GC_LVT), (<span class="string">'\u{c544}'</span>, <span class="string">'\u{c544}'</span>, GC_LV), (<span class="string">'\u{c545}'</span>, <span class="string">'\u{c55f}'</span>,
        GC_LVT), (<span class="string">'\u{c560}'</span>, <span class="string">'\u{c560}'</span>, GC_LV), (<span class="string">'\u{c561}'</span>, <span class="string">'\u{c57b}'</span>, GC_LVT), (<span class="string">'\u{c57c}'</span>,
        <span class="string">'\u{c57c}'</span>, GC_LV), (<span class="string">'\u{c57d}'</span>, <span class="string">'\u{c597}'</span>, GC_LVT), (<span class="string">'\u{c598}'</span>, <span class="string">'\u{c598}'</span>, GC_LV),
        (<span class="string">'\u{c599}'</span>, <span class="string">'\u{c5b3}'</span>, GC_LVT), (<span class="string">'\u{c5b4}'</span>, <span class="string">'\u{c5b4}'</span>, GC_LV), (<span class="string">'\u{c5b5}'</span>, <span class="string">'\u{c5cf}'</span>,
        GC_LVT), (<span class="string">'\u{c5d0}'</span>, <span class="string">'\u{c5d0}'</span>, GC_LV), (<span class="string">'\u{c5d1}'</span>, <span class="string">'\u{c5eb}'</span>, GC_LVT), (<span class="string">'\u{c5ec}'</span>,
        <span class="string">'\u{c5ec}'</span>, GC_LV), (<span class="string">'\u{c5ed}'</span>, <span class="string">'\u{c607}'</span>, GC_LVT), (<span class="string">'\u{c608}'</span>, <span class="string">'\u{c608}'</span>, GC_LV),
        (<span class="string">'\u{c609}'</span>, <span class="string">'\u{c623}'</span>, GC_LVT), (<span class="string">'\u{c624}'</span>, <span class="string">'\u{c624}'</span>, GC_LV), (<span class="string">'\u{c625}'</span>, <span class="string">'\u{c63f}'</span>,
        GC_LVT), (<span class="string">'\u{c640}'</span>, <span class="string">'\u{c640}'</span>, GC_LV), (<span class="string">'\u{c641}'</span>, <span class="string">'\u{c65b}'</span>, GC_LVT), (<span class="string">'\u{c65c}'</span>,
        <span class="string">'\u{c65c}'</span>, GC_LV), (<span class="string">'\u{c65d}'</span>, <span class="string">'\u{c677}'</span>, GC_LVT), (<span class="string">'\u{c678}'</span>, <span class="string">'\u{c678}'</span>, GC_LV),
        (<span class="string">'\u{c679}'</span>, <span class="string">'\u{c693}'</span>, GC_LVT), (<span class="string">'\u{c694}'</span>, <span class="string">'\u{c694}'</span>, GC_LV), (<span class="string">'\u{c695}'</span>, <span class="string">'\u{c6af}'</span>,
        GC_LVT), (<span class="string">'\u{c6b0}'</span>, <span class="string">'\u{c6b0}'</span>, GC_LV), (<span class="string">'\u{c6b1}'</span>, <span class="string">'\u{c6cb}'</span>, GC_LVT), (<span class="string">'\u{c6cc}'</span>,
        <span class="string">'\u{c6cc}'</span>, GC_LV), (<span class="string">'\u{c6cd}'</span>, <span class="string">'\u{c6e7}'</span>, GC_LVT), (<span class="string">'\u{c6e8}'</span>, <span class="string">'\u{c6e8}'</span>, GC_LV),
        (<span class="string">'\u{c6e9}'</span>, <span class="string">'\u{c703}'</span>, GC_LVT), (<span class="string">'\u{c704}'</span>, <span class="string">'\u{c704}'</span>, GC_LV), (<span class="string">'\u{c705}'</span>, <span class="string">'\u{c71f}'</span>,
        GC_LVT), (<span class="string">'\u{c720}'</span>, <span class="string">'\u{c720}'</span>, GC_LV), (<span class="string">'\u{c721}'</span>, <span class="string">'\u{c73b}'</span>, GC_LVT), (<span class="string">'\u{c73c}'</span>,
        <span class="string">'\u{c73c}'</span>, GC_LV), (<span class="string">'\u{c73d}'</span>, <span class="string">'\u{c757}'</span>, GC_LVT), (<span class="string">'\u{c758}'</span>, <span class="string">'\u{c758}'</span>, GC_LV),
        (<span class="string">'\u{c759}'</span>, <span class="string">'\u{c773}'</span>, GC_LVT), (<span class="string">'\u{c774}'</span>, <span class="string">'\u{c774}'</span>, GC_LV), (<span class="string">'\u{c775}'</span>, <span class="string">'\u{c78f}'</span>,
        GC_LVT), (<span class="string">'\u{c790}'</span>, <span class="string">'\u{c790}'</span>, GC_LV), (<span class="string">'\u{c791}'</span>, <span class="string">'\u{c7ab}'</span>, GC_LVT), (<span class="string">'\u{c7ac}'</span>,
        <span class="string">'\u{c7ac}'</span>, GC_LV), (<span class="string">'\u{c7ad}'</span>, <span class="string">'\u{c7c7}'</span>, GC_LVT), (<span class="string">'\u{c7c8}'</span>, <span class="string">'\u{c7c8}'</span>, GC_LV),
        (<span class="string">'\u{c7c9}'</span>, <span class="string">'\u{c7e3}'</span>, GC_LVT), (<span class="string">'\u{c7e4}'</span>, <span class="string">'\u{c7e4}'</span>, GC_LV), (<span class="string">'\u{c7e5}'</span>, <span class="string">'\u{c7ff}'</span>,
        GC_LVT), (<span class="string">'\u{c800}'</span>, <span class="string">'\u{c800}'</span>, GC_LV), (<span class="string">'\u{c801}'</span>, <span class="string">'\u{c81b}'</span>, GC_LVT), (<span class="string">'\u{c81c}'</span>,
        <span class="string">'\u{c81c}'</span>, GC_LV), (<span class="string">'\u{c81d}'</span>, <span class="string">'\u{c837}'</span>, GC_LVT), (<span class="string">'\u{c838}'</span>, <span class="string">'\u{c838}'</span>, GC_LV),
        (<span class="string">'\u{c839}'</span>, <span class="string">'\u{c853}'</span>, GC_LVT), (<span class="string">'\u{c854}'</span>, <span class="string">'\u{c854}'</span>, GC_LV), (<span class="string">'\u{c855}'</span>, <span class="string">'\u{c86f}'</span>,
        GC_LVT), (<span class="string">'\u{c870}'</span>, <span class="string">'\u{c870}'</span>, GC_LV), (<span class="string">'\u{c871}'</span>, <span class="string">'\u{c88b}'</span>, GC_LVT), (<span class="string">'\u{c88c}'</span>,
        <span class="string">'\u{c88c}'</span>, GC_LV), (<span class="string">'\u{c88d}'</span>, <span class="string">'\u{c8a7}'</span>, GC_LVT), (<span class="string">'\u{c8a8}'</span>, <span class="string">'\u{c8a8}'</span>, GC_LV),
        (<span class="string">'\u{c8a9}'</span>, <span class="string">'\u{c8c3}'</span>, GC_LVT), (<span class="string">'\u{c8c4}'</span>, <span class="string">'\u{c8c4}'</span>, GC_LV), (<span class="string">'\u{c8c5}'</span>, <span class="string">'\u{c8df}'</span>,
        GC_LVT), (<span class="string">'\u{c8e0}'</span>, <span class="string">'\u{c8e0}'</span>, GC_LV), (<span class="string">'\u{c8e1}'</span>, <span class="string">'\u{c8fb}'</span>, GC_LVT), (<span class="string">'\u{c8fc}'</span>,
        <span class="string">'\u{c8fc}'</span>, GC_LV), (<span class="string">'\u{c8fd}'</span>, <span class="string">'\u{c917}'</span>, GC_LVT), (<span class="string">'\u{c918}'</span>, <span class="string">'\u{c918}'</span>, GC_LV),
        (<span class="string">'\u{c919}'</span>, <span class="string">'\u{c933}'</span>, GC_LVT), (<span class="string">'\u{c934}'</span>, <span class="string">'\u{c934}'</span>, GC_LV), (<span class="string">'\u{c935}'</span>, <span class="string">'\u{c94f}'</span>,
        GC_LVT), (<span class="string">'\u{c950}'</span>, <span class="string">'\u{c950}'</span>, GC_LV), (<span class="string">'\u{c951}'</span>, <span class="string">'\u{c96b}'</span>, GC_LVT), (<span class="string">'\u{c96c}'</span>,
        <span class="string">'\u{c96c}'</span>, GC_LV), (<span class="string">'\u{c96d}'</span>, <span class="string">'\u{c987}'</span>, GC_LVT), (<span class="string">'\u{c988}'</span>, <span class="string">'\u{c988}'</span>, GC_LV),
        (<span class="string">'\u{c989}'</span>, <span class="string">'\u{c9a3}'</span>, GC_LVT), (<span class="string">'\u{c9a4}'</span>, <span class="string">'\u{c9a4}'</span>, GC_LV), (<span class="string">'\u{c9a5}'</span>, <span class="string">'\u{c9bf}'</span>,
        GC_LVT), (<span class="string">'\u{c9c0}'</span>, <span class="string">'\u{c9c0}'</span>, GC_LV), (<span class="string">'\u{c9c1}'</span>, <span class="string">'\u{c9db}'</span>, GC_LVT), (<span class="string">'\u{c9dc}'</span>,
        <span class="string">'\u{c9dc}'</span>, GC_LV), (<span class="string">'\u{c9dd}'</span>, <span class="string">'\u{c9f7}'</span>, GC_LVT), (<span class="string">'\u{c9f8}'</span>, <span class="string">'\u{c9f8}'</span>, GC_LV),
        (<span class="string">'\u{c9f9}'</span>, <span class="string">'\u{ca13}'</span>, GC_LVT), (<span class="string">'\u{ca14}'</span>, <span class="string">'\u{ca14}'</span>, GC_LV), (<span class="string">'\u{ca15}'</span>, <span class="string">'\u{ca2f}'</span>,
        GC_LVT), (<span class="string">'\u{ca30}'</span>, <span class="string">'\u{ca30}'</span>, GC_LV), (<span class="string">'\u{ca31}'</span>, <span class="string">'\u{ca4b}'</span>, GC_LVT), (<span class="string">'\u{ca4c}'</span>,
        <span class="string">'\u{ca4c}'</span>, GC_LV), (<span class="string">'\u{ca4d}'</span>, <span class="string">'\u{ca67}'</span>, GC_LVT), (<span class="string">'\u{ca68}'</span>, <span class="string">'\u{ca68}'</span>, GC_LV),
        (<span class="string">'\u{ca69}'</span>, <span class="string">'\u{ca83}'</span>, GC_LVT), (<span class="string">'\u{ca84}'</span>, <span class="string">'\u{ca84}'</span>, GC_LV), (<span class="string">'\u{ca85}'</span>, <span class="string">'\u{ca9f}'</span>,
        GC_LVT), (<span class="string">'\u{caa0}'</span>, <span class="string">'\u{caa0}'</span>, GC_LV), (<span class="string">'\u{caa1}'</span>, <span class="string">'\u{cabb}'</span>, GC_LVT), (<span class="string">'\u{cabc}'</span>,
        <span class="string">'\u{cabc}'</span>, GC_LV), (<span class="string">'\u{cabd}'</span>, <span class="string">'\u{cad7}'</span>, GC_LVT), (<span class="string">'\u{cad8}'</span>, <span class="string">'\u{cad8}'</span>, GC_LV),
        (<span class="string">'\u{cad9}'</span>, <span class="string">'\u{caf3}'</span>, GC_LVT), (<span class="string">'\u{caf4}'</span>, <span class="string">'\u{caf4}'</span>, GC_LV), (<span class="string">'\u{caf5}'</span>, <span class="string">'\u{cb0f}'</span>,
        GC_LVT), (<span class="string">'\u{cb10}'</span>, <span class="string">'\u{cb10}'</span>, GC_LV), (<span class="string">'\u{cb11}'</span>, <span class="string">'\u{cb2b}'</span>, GC_LVT), (<span class="string">'\u{cb2c}'</span>,
        <span class="string">'\u{cb2c}'</span>, GC_LV), (<span class="string">'\u{cb2d}'</span>, <span class="string">'\u{cb47}'</span>, GC_LVT), (<span class="string">'\u{cb48}'</span>, <span class="string">'\u{cb48}'</span>, GC_LV),
        (<span class="string">'\u{cb49}'</span>, <span class="string">'\u{cb63}'</span>, GC_LVT), (<span class="string">'\u{cb64}'</span>, <span class="string">'\u{cb64}'</span>, GC_LV), (<span class="string">'\u{cb65}'</span>, <span class="string">'\u{cb7f}'</span>,
        GC_LVT), (<span class="string">'\u{cb80}'</span>, <span class="string">'\u{cb80}'</span>, GC_LV), (<span class="string">'\u{cb81}'</span>, <span class="string">'\u{cb9b}'</span>, GC_LVT), (<span class="string">'\u{cb9c}'</span>,
        <span class="string">'\u{cb9c}'</span>, GC_LV), (<span class="string">'\u{cb9d}'</span>, <span class="string">'\u{cbb7}'</span>, GC_LVT), (<span class="string">'\u{cbb8}'</span>, <span class="string">'\u{cbb8}'</span>, GC_LV),
        (<span class="string">'\u{cbb9}'</span>, <span class="string">'\u{cbd3}'</span>, GC_LVT), (<span class="string">'\u{cbd4}'</span>, <span class="string">'\u{cbd4}'</span>, GC_LV), (<span class="string">'\u{cbd5}'</span>, <span class="string">'\u{cbef}'</span>,
        GC_LVT), (<span class="string">'\u{cbf0}'</span>, <span class="string">'\u{cbf0}'</span>, GC_LV), (<span class="string">'\u{cbf1}'</span>, <span class="string">'\u{cc0b}'</span>, GC_LVT), (<span class="string">'\u{cc0c}'</span>,
        <span class="string">'\u{cc0c}'</span>, GC_LV), (<span class="string">'\u{cc0d}'</span>, <span class="string">'\u{cc27}'</span>, GC_LVT), (<span class="string">'\u{cc28}'</span>, <span class="string">'\u{cc28}'</span>, GC_LV),
        (<span class="string">'\u{cc29}'</span>, <span class="string">'\u{cc43}'</span>, GC_LVT), (<span class="string">'\u{cc44}'</span>, <span class="string">'\u{cc44}'</span>, GC_LV), (<span class="string">'\u{cc45}'</span>, <span class="string">'\u{cc5f}'</span>,
        GC_LVT), (<span class="string">'\u{cc60}'</span>, <span class="string">'\u{cc60}'</span>, GC_LV), (<span class="string">'\u{cc61}'</span>, <span class="string">'\u{cc7b}'</span>, GC_LVT), (<span class="string">'\u{cc7c}'</span>,
        <span class="string">'\u{cc7c}'</span>, GC_LV), (<span class="string">'\u{cc7d}'</span>, <span class="string">'\u{cc97}'</span>, GC_LVT), (<span class="string">'\u{cc98}'</span>, <span class="string">'\u{cc98}'</span>, GC_LV),
        (<span class="string">'\u{cc99}'</span>, <span class="string">'\u{ccb3}'</span>, GC_LVT), (<span class="string">'\u{ccb4}'</span>, <span class="string">'\u{ccb4}'</span>, GC_LV), (<span class="string">'\u{ccb5}'</span>, <span class="string">'\u{cccf}'</span>,
        GC_LVT), (<span class="string">'\u{ccd0}'</span>, <span class="string">'\u{ccd0}'</span>, GC_LV), (<span class="string">'\u{ccd1}'</span>, <span class="string">'\u{cceb}'</span>, GC_LVT), (<span class="string">'\u{ccec}'</span>,
        <span class="string">'\u{ccec}'</span>, GC_LV), (<span class="string">'\u{cced}'</span>, <span class="string">'\u{cd07}'</span>, GC_LVT), (<span class="string">'\u{cd08}'</span>, <span class="string">'\u{cd08}'</span>, GC_LV),
        (<span class="string">'\u{cd09}'</span>, <span class="string">'\u{cd23}'</span>, GC_LVT), (<span class="string">'\u{cd24}'</span>, <span class="string">'\u{cd24}'</span>, GC_LV), (<span class="string">'\u{cd25}'</span>, <span class="string">'\u{cd3f}'</span>,
        GC_LVT), (<span class="string">'\u{cd40}'</span>, <span class="string">'\u{cd40}'</span>, GC_LV), (<span class="string">'\u{cd41}'</span>, <span class="string">'\u{cd5b}'</span>, GC_LVT), (<span class="string">'\u{cd5c}'</span>,
        <span class="string">'\u{cd5c}'</span>, GC_LV), (<span class="string">'\u{cd5d}'</span>, <span class="string">'\u{cd77}'</span>, GC_LVT), (<span class="string">'\u{cd78}'</span>, <span class="string">'\u{cd78}'</span>, GC_LV),
        (<span class="string">'\u{cd79}'</span>, <span class="string">'\u{cd93}'</span>, GC_LVT), (<span class="string">'\u{cd94}'</span>, <span class="string">'\u{cd94}'</span>, GC_LV), (<span class="string">'\u{cd95}'</span>, <span class="string">'\u{cdaf}'</span>,
        GC_LVT), (<span class="string">'\u{cdb0}'</span>, <span class="string">'\u{cdb0}'</span>, GC_LV), (<span class="string">'\u{cdb1}'</span>, <span class="string">'\u{cdcb}'</span>, GC_LVT), (<span class="string">'\u{cdcc}'</span>,
        <span class="string">'\u{cdcc}'</span>, GC_LV), (<span class="string">'\u{cdcd}'</span>, <span class="string">'\u{cde7}'</span>, GC_LVT), (<span class="string">'\u{cde8}'</span>, <span class="string">'\u{cde8}'</span>, GC_LV),
        (<span class="string">'\u{cde9}'</span>, <span class="string">'\u{ce03}'</span>, GC_LVT), (<span class="string">'\u{ce04}'</span>, <span class="string">'\u{ce04}'</span>, GC_LV), (<span class="string">'\u{ce05}'</span>, <span class="string">'\u{ce1f}'</span>,
        GC_LVT), (<span class="string">'\u{ce20}'</span>, <span class="string">'\u{ce20}'</span>, GC_LV), (<span class="string">'\u{ce21}'</span>, <span class="string">'\u{ce3b}'</span>, GC_LVT), (<span class="string">'\u{ce3c}'</span>,
        <span class="string">'\u{ce3c}'</span>, GC_LV), (<span class="string">'\u{ce3d}'</span>, <span class="string">'\u{ce57}'</span>, GC_LVT), (<span class="string">'\u{ce58}'</span>, <span class="string">'\u{ce58}'</span>, GC_LV),
        (<span class="string">'\u{ce59}'</span>, <span class="string">'\u{ce73}'</span>, GC_LVT), (<span class="string">'\u{ce74}'</span>, <span class="string">'\u{ce74}'</span>, GC_LV), (<span class="string">'\u{ce75}'</span>, <span class="string">'\u{ce8f}'</span>,
        GC_LVT), (<span class="string">'\u{ce90}'</span>, <span class="string">'\u{ce90}'</span>, GC_LV), (<span class="string">'\u{ce91}'</span>, <span class="string">'\u{ceab}'</span>, GC_LVT), (<span class="string">'\u{ceac}'</span>,
        <span class="string">'\u{ceac}'</span>, GC_LV), (<span class="string">'\u{cead}'</span>, <span class="string">'\u{cec7}'</span>, GC_LVT), (<span class="string">'\u{cec8}'</span>, <span class="string">'\u{cec8}'</span>, GC_LV),
        (<span class="string">'\u{cec9}'</span>, <span class="string">'\u{cee3}'</span>, GC_LVT), (<span class="string">'\u{cee4}'</span>, <span class="string">'\u{cee4}'</span>, GC_LV), (<span class="string">'\u{cee5}'</span>, <span class="string">'\u{ceff}'</span>,
        GC_LVT), (<span class="string">'\u{cf00}'</span>, <span class="string">'\u{cf00}'</span>, GC_LV), (<span class="string">'\u{cf01}'</span>, <span class="string">'\u{cf1b}'</span>, GC_LVT), (<span class="string">'\u{cf1c}'</span>,
        <span class="string">'\u{cf1c}'</span>, GC_LV), (<span class="string">'\u{cf1d}'</span>, <span class="string">'\u{cf37}'</span>, GC_LVT), (<span class="string">'\u{cf38}'</span>, <span class="string">'\u{cf38}'</span>, GC_LV),
        (<span class="string">'\u{cf39}'</span>, <span class="string">'\u{cf53}'</span>, GC_LVT), (<span class="string">'\u{cf54}'</span>, <span class="string">'\u{cf54}'</span>, GC_LV), (<span class="string">'\u{cf55}'</span>, <span class="string">'\u{cf6f}'</span>,
        GC_LVT), (<span class="string">'\u{cf70}'</span>, <span class="string">'\u{cf70}'</span>, GC_LV), (<span class="string">'\u{cf71}'</span>, <span class="string">'\u{cf8b}'</span>, GC_LVT), (<span class="string">'\u{cf8c}'</span>,
        <span class="string">'\u{cf8c}'</span>, GC_LV), (<span class="string">'\u{cf8d}'</span>, <span class="string">'\u{cfa7}'</span>, GC_LVT), (<span class="string">'\u{cfa8}'</span>, <span class="string">'\u{cfa8}'</span>, GC_LV),
        (<span class="string">'\u{cfa9}'</span>, <span class="string">'\u{cfc3}'</span>, GC_LVT), (<span class="string">'\u{cfc4}'</span>, <span class="string">'\u{cfc4}'</span>, GC_LV), (<span class="string">'\u{cfc5}'</span>, <span class="string">'\u{cfdf}'</span>,
        GC_LVT), (<span class="string">'\u{cfe0}'</span>, <span class="string">'\u{cfe0}'</span>, GC_LV), (<span class="string">'\u{cfe1}'</span>, <span class="string">'\u{cffb}'</span>, GC_LVT), (<span class="string">'\u{cffc}'</span>,
        <span class="string">'\u{cffc}'</span>, GC_LV), (<span class="string">'\u{cffd}'</span>, <span class="string">'\u{d017}'</span>, GC_LVT), (<span class="string">'\u{d018}'</span>, <span class="string">'\u{d018}'</span>, GC_LV),
        (<span class="string">'\u{d019}'</span>, <span class="string">'\u{d033}'</span>, GC_LVT), (<span class="string">'\u{d034}'</span>, <span class="string">'\u{d034}'</span>, GC_LV), (<span class="string">'\u{d035}'</span>, <span class="string">'\u{d04f}'</span>,
        GC_LVT), (<span class="string">'\u{d050}'</span>, <span class="string">'\u{d050}'</span>, GC_LV), (<span class="string">'\u{d051}'</span>, <span class="string">'\u{d06b}'</span>, GC_LVT), (<span class="string">'\u{d06c}'</span>,
        <span class="string">'\u{d06c}'</span>, GC_LV), (<span class="string">'\u{d06d}'</span>, <span class="string">'\u{d087}'</span>, GC_LVT), (<span class="string">'\u{d088}'</span>, <span class="string">'\u{d088}'</span>, GC_LV),
        (<span class="string">'\u{d089}'</span>, <span class="string">'\u{d0a3}'</span>, GC_LVT), (<span class="string">'\u{d0a4}'</span>, <span class="string">'\u{d0a4}'</span>, GC_LV), (<span class="string">'\u{d0a5}'</span>, <span class="string">'\u{d0bf}'</span>,
        GC_LVT), (<span class="string">'\u{d0c0}'</span>, <span class="string">'\u{d0c0}'</span>, GC_LV), (<span class="string">'\u{d0c1}'</span>, <span class="string">'\u{d0db}'</span>, GC_LVT), (<span class="string">'\u{d0dc}'</span>,
        <span class="string">'\u{d0dc}'</span>, GC_LV), (<span class="string">'\u{d0dd}'</span>, <span class="string">'\u{d0f7}'</span>, GC_LVT), (<span class="string">'\u{d0f8}'</span>, <span class="string">'\u{d0f8}'</span>, GC_LV),
        (<span class="string">'\u{d0f9}'</span>, <span class="string">'\u{d113}'</span>, GC_LVT), (<span class="string">'\u{d114}'</span>, <span class="string">'\u{d114}'</span>, GC_LV), (<span class="string">'\u{d115}'</span>, <span class="string">'\u{d12f}'</span>,
        GC_LVT), (<span class="string">'\u{d130}'</span>, <span class="string">'\u{d130}'</span>, GC_LV), (<span class="string">'\u{d131}'</span>, <span class="string">'\u{d14b}'</span>, GC_LVT), (<span class="string">'\u{d14c}'</span>,
        <span class="string">'\u{d14c}'</span>, GC_LV), (<span class="string">'\u{d14d}'</span>, <span class="string">'\u{d167}'</span>, GC_LVT), (<span class="string">'\u{d168}'</span>, <span class="string">'\u{d168}'</span>, GC_LV),
        (<span class="string">'\u{d169}'</span>, <span class="string">'\u{d183}'</span>, GC_LVT), (<span class="string">'\u{d184}'</span>, <span class="string">'\u{d184}'</span>, GC_LV), (<span class="string">'\u{d185}'</span>, <span class="string">'\u{d19f}'</span>,
        GC_LVT), (<span class="string">'\u{d1a0}'</span>, <span class="string">'\u{d1a0}'</span>, GC_LV), (<span class="string">'\u{d1a1}'</span>, <span class="string">'\u{d1bb}'</span>, GC_LVT), (<span class="string">'\u{d1bc}'</span>,
        <span class="string">'\u{d1bc}'</span>, GC_LV), (<span class="string">'\u{d1bd}'</span>, <span class="string">'\u{d1d7}'</span>, GC_LVT), (<span class="string">'\u{d1d8}'</span>, <span class="string">'\u{d1d8}'</span>, GC_LV),
        (<span class="string">'\u{d1d9}'</span>, <span class="string">'\u{d1f3}'</span>, GC_LVT), (<span class="string">'\u{d1f4}'</span>, <span class="string">'\u{d1f4}'</span>, GC_LV), (<span class="string">'\u{d1f5}'</span>, <span class="string">'\u{d20f}'</span>,
        GC_LVT), (<span class="string">'\u{d210}'</span>, <span class="string">'\u{d210}'</span>, GC_LV), (<span class="string">'\u{d211}'</span>, <span class="string">'\u{d22b}'</span>, GC_LVT), (<span class="string">'\u{d22c}'</span>,
        <span class="string">'\u{d22c}'</span>, GC_LV), (<span class="string">'\u{d22d}'</span>, <span class="string">'\u{d247}'</span>, GC_LVT), (<span class="string">'\u{d248}'</span>, <span class="string">'\u{d248}'</span>, GC_LV),
        (<span class="string">'\u{d249}'</span>, <span class="string">'\u{d263}'</span>, GC_LVT), (<span class="string">'\u{d264}'</span>, <span class="string">'\u{d264}'</span>, GC_LV), (<span class="string">'\u{d265}'</span>, <span class="string">'\u{d27f}'</span>,
        GC_LVT), (<span class="string">'\u{d280}'</span>, <span class="string">'\u{d280}'</span>, GC_LV), (<span class="string">'\u{d281}'</span>, <span class="string">'\u{d29b}'</span>, GC_LVT), (<span class="string">'\u{d29c}'</span>,
        <span class="string">'\u{d29c}'</span>, GC_LV), (<span class="string">'\u{d29d}'</span>, <span class="string">'\u{d2b7}'</span>, GC_LVT), (<span class="string">'\u{d2b8}'</span>, <span class="string">'\u{d2b8}'</span>, GC_LV),
        (<span class="string">'\u{d2b9}'</span>, <span class="string">'\u{d2d3}'</span>, GC_LVT), (<span class="string">'\u{d2d4}'</span>, <span class="string">'\u{d2d4}'</span>, GC_LV), (<span class="string">'\u{d2d5}'</span>, <span class="string">'\u{d2ef}'</span>,
        GC_LVT), (<span class="string">'\u{d2f0}'</span>, <span class="string">'\u{d2f0}'</span>, GC_LV), (<span class="string">'\u{d2f1}'</span>, <span class="string">'\u{d30b}'</span>, GC_LVT), (<span class="string">'\u{d30c}'</span>,
        <span class="string">'\u{d30c}'</span>, GC_LV), (<span class="string">'\u{d30d}'</span>, <span class="string">'\u{d327}'</span>, GC_LVT), (<span class="string">'\u{d328}'</span>, <span class="string">'\u{d328}'</span>, GC_LV),
        (<span class="string">'\u{d329}'</span>, <span class="string">'\u{d343}'</span>, GC_LVT), (<span class="string">'\u{d344}'</span>, <span class="string">'\u{d344}'</span>, GC_LV), (<span class="string">'\u{d345}'</span>, <span class="string">'\u{d35f}'</span>,
        GC_LVT), (<span class="string">'\u{d360}'</span>, <span class="string">'\u{d360}'</span>, GC_LV), (<span class="string">'\u{d361}'</span>, <span class="string">'\u{d37b}'</span>, GC_LVT), (<span class="string">'\u{d37c}'</span>,
        <span class="string">'\u{d37c}'</span>, GC_LV), (<span class="string">'\u{d37d}'</span>, <span class="string">'\u{d397}'</span>, GC_LVT), (<span class="string">'\u{d398}'</span>, <span class="string">'\u{d398}'</span>, GC_LV),
        (<span class="string">'\u{d399}'</span>, <span class="string">'\u{d3b3}'</span>, GC_LVT), (<span class="string">'\u{d3b4}'</span>, <span class="string">'\u{d3b4}'</span>, GC_LV), (<span class="string">'\u{d3b5}'</span>, <span class="string">'\u{d3cf}'</span>,
        GC_LVT), (<span class="string">'\u{d3d0}'</span>, <span class="string">'\u{d3d0}'</span>, GC_LV), (<span class="string">'\u{d3d1}'</span>, <span class="string">'\u{d3eb}'</span>, GC_LVT), (<span class="string">'\u{d3ec}'</span>,
        <span class="string">'\u{d3ec}'</span>, GC_LV), (<span class="string">'\u{d3ed}'</span>, <span class="string">'\u{d407}'</span>, GC_LVT), (<span class="string">'\u{d408}'</span>, <span class="string">'\u{d408}'</span>, GC_LV),
        (<span class="string">'\u{d409}'</span>, <span class="string">'\u{d423}'</span>, GC_LVT), (<span class="string">'\u{d424}'</span>, <span class="string">'\u{d424}'</span>, GC_LV), (<span class="string">'\u{d425}'</span>, <span class="string">'\u{d43f}'</span>,
        GC_LVT), (<span class="string">'\u{d440}'</span>, <span class="string">'\u{d440}'</span>, GC_LV), (<span class="string">'\u{d441}'</span>, <span class="string">'\u{d45b}'</span>, GC_LVT), (<span class="string">'\u{d45c}'</span>,
        <span class="string">'\u{d45c}'</span>, GC_LV), (<span class="string">'\u{d45d}'</span>, <span class="string">'\u{d477}'</span>, GC_LVT), (<span class="string">'\u{d478}'</span>, <span class="string">'\u{d478}'</span>, GC_LV),
        (<span class="string">'\u{d479}'</span>, <span class="string">'\u{d493}'</span>, GC_LVT), (<span class="string">'\u{d494}'</span>, <span class="string">'\u{d494}'</span>, GC_LV), (<span class="string">'\u{d495}'</span>, <span class="string">'\u{d4af}'</span>,
        GC_LVT), (<span class="string">'\u{d4b0}'</span>, <span class="string">'\u{d4b0}'</span>, GC_LV), (<span class="string">'\u{d4b1}'</span>, <span class="string">'\u{d4cb}'</span>, GC_LVT), (<span class="string">'\u{d4cc}'</span>,
        <span class="string">'\u{d4cc}'</span>, GC_LV), (<span class="string">'\u{d4cd}'</span>, <span class="string">'\u{d4e7}'</span>, GC_LVT), (<span class="string">'\u{d4e8}'</span>, <span class="string">'\u{d4e8}'</span>, GC_LV),
        (<span class="string">'\u{d4e9}'</span>, <span class="string">'\u{d503}'</span>, GC_LVT), (<span class="string">'\u{d504}'</span>, <span class="string">'\u{d504}'</span>, GC_LV), (<span class="string">'\u{d505}'</span>, <span class="string">'\u{d51f}'</span>,
        GC_LVT), (<span class="string">'\u{d520}'</span>, <span class="string">'\u{d520}'</span>, GC_LV), (<span class="string">'\u{d521}'</span>, <span class="string">'\u{d53b}'</span>, GC_LVT), (<span class="string">'\u{d53c}'</span>,
        <span class="string">'\u{d53c}'</span>, GC_LV), (<span class="string">'\u{d53d}'</span>, <span class="string">'\u{d557}'</span>, GC_LVT), (<span class="string">'\u{d558}'</span>, <span class="string">'\u{d558}'</span>, GC_LV),
        (<span class="string">'\u{d559}'</span>, <span class="string">'\u{d573}'</span>, GC_LVT), (<span class="string">'\u{d574}'</span>, <span class="string">'\u{d574}'</span>, GC_LV), (<span class="string">'\u{d575}'</span>, <span class="string">'\u{d58f}'</span>,
        GC_LVT), (<span class="string">'\u{d590}'</span>, <span class="string">'\u{d590}'</span>, GC_LV), (<span class="string">'\u{d591}'</span>, <span class="string">'\u{d5ab}'</span>, GC_LVT), (<span class="string">'\u{d5ac}'</span>,
        <span class="string">'\u{d5ac}'</span>, GC_LV), (<span class="string">'\u{d5ad}'</span>, <span class="string">'\u{d5c7}'</span>, GC_LVT), (<span class="string">'\u{d5c8}'</span>, <span class="string">'\u{d5c8}'</span>, GC_LV),
        (<span class="string">'\u{d5c9}'</span>, <span class="string">'\u{d5e3}'</span>, GC_LVT), (<span class="string">'\u{d5e4}'</span>, <span class="string">'\u{d5e4}'</span>, GC_LV), (<span class="string">'\u{d5e5}'</span>, <span class="string">'\u{d5ff}'</span>,
        GC_LVT), (<span class="string">'\u{d600}'</span>, <span class="string">'\u{d600}'</span>, GC_LV), (<span class="string">'\u{d601}'</span>, <span class="string">'\u{d61b}'</span>, GC_LVT), (<span class="string">'\u{d61c}'</span>,
        <span class="string">'\u{d61c}'</span>, GC_LV), (<span class="string">'\u{d61d}'</span>, <span class="string">'\u{d637}'</span>, GC_LVT), (<span class="string">'\u{d638}'</span>, <span class="string">'\u{d638}'</span>, GC_LV),
        (<span class="string">'\u{d639}'</span>, <span class="string">'\u{d653}'</span>, GC_LVT), (<span class="string">'\u{d654}'</span>, <span class="string">'\u{d654}'</span>, GC_LV), (<span class="string">'\u{d655}'</span>, <span class="string">'\u{d66f}'</span>,
        GC_LVT), (<span class="string">'\u{d670}'</span>, <span class="string">'\u{d670}'</span>, GC_LV), (<span class="string">'\u{d671}'</span>, <span class="string">'\u{d68b}'</span>, GC_LVT), (<span class="string">'\u{d68c}'</span>,
        <span class="string">'\u{d68c}'</span>, GC_LV), (<span class="string">'\u{d68d}'</span>, <span class="string">'\u{d6a7}'</span>, GC_LVT), (<span class="string">'\u{d6a8}'</span>, <span class="string">'\u{d6a8}'</span>, GC_LV),
        (<span class="string">'\u{d6a9}'</span>, <span class="string">'\u{d6c3}'</span>, GC_LVT), (<span class="string">'\u{d6c4}'</span>, <span class="string">'\u{d6c4}'</span>, GC_LV), (<span class="string">'\u{d6c5}'</span>, <span class="string">'\u{d6df}'</span>,
        GC_LVT), (<span class="string">'\u{d6e0}'</span>, <span class="string">'\u{d6e0}'</span>, GC_LV), (<span class="string">'\u{d6e1}'</span>, <span class="string">'\u{d6fb}'</span>, GC_LVT), (<span class="string">'\u{d6fc}'</span>,
        <span class="string">'\u{d6fc}'</span>, GC_LV), (<span class="string">'\u{d6fd}'</span>, <span class="string">'\u{d717}'</span>, GC_LVT), (<span class="string">'\u{d718}'</span>, <span class="string">'\u{d718}'</span>, GC_LV),
        (<span class="string">'\u{d719}'</span>, <span class="string">'\u{d733}'</span>, GC_LVT), (<span class="string">'\u{d734}'</span>, <span class="string">'\u{d734}'</span>, GC_LV), (<span class="string">'\u{d735}'</span>, <span class="string">'\u{d74f}'</span>,
        GC_LVT), (<span class="string">'\u{d750}'</span>, <span class="string">'\u{d750}'</span>, GC_LV), (<span class="string">'\u{d751}'</span>, <span class="string">'\u{d76b}'</span>, GC_LVT), (<span class="string">'\u{d76c}'</span>,
        <span class="string">'\u{d76c}'</span>, GC_LV), (<span class="string">'\u{d76d}'</span>, <span class="string">'\u{d787}'</span>, GC_LVT), (<span class="string">'\u{d788}'</span>, <span class="string">'\u{d788}'</span>, GC_LV),
        (<span class="string">'\u{d789}'</span>, <span class="string">'\u{d7a3}'</span>, GC_LVT), (<span class="string">'\u{d7b0}'</span>, <span class="string">'\u{d7c6}'</span>, GC_V), (<span class="string">'\u{d7cb}'</span>, <span class="string">'\u{d7fb}'</span>,
        GC_T), (<span class="string">'\u{fb1e}'</span>, <span class="string">'\u{fb1e}'</span>, GC_Extend), (<span class="string">'\u{fe00}'</span>, <span class="string">'\u{fe0f}'</span>, GC_Extend),
        (<span class="string">'\u{fe20}'</span>, <span class="string">'\u{fe2f}'</span>, GC_Extend), (<span class="string">'\u{feff}'</span>, <span class="string">'\u{feff}'</span>, GC_Control), (<span class="string">'\u{ff9e}'</span>,
        <span class="string">'\u{ff9f}'</span>, GC_Extend), (<span class="string">'\u{fff0}'</span>, <span class="string">'\u{fffb}'</span>, GC_Control), (<span class="string">'\u{101fd}'</span>, <span class="string">'\u{101fd}'</span>,
        GC_Extend), (<span class="string">'\u{102e0}'</span>, <span class="string">'\u{102e0}'</span>, GC_Extend), (<span class="string">'\u{10376}'</span>, <span class="string">'\u{1037a}'</span>, GC_Extend),
        (<span class="string">'\u{10a01}'</span>, <span class="string">'\u{10a03}'</span>, GC_Extend), (<span class="string">'\u{10a05}'</span>, <span class="string">'\u{10a06}'</span>, GC_Extend), (<span class="string">'\u{10a0c}'</span>,
        <span class="string">'\u{10a0f}'</span>, GC_Extend), (<span class="string">'\u{10a38}'</span>, <span class="string">'\u{10a3a}'</span>, GC_Extend), (<span class="string">'\u{10a3f}'</span>, <span class="string">'\u{10a3f}'</span>,
        GC_Extend), (<span class="string">'\u{10ae5}'</span>, <span class="string">'\u{10ae6}'</span>, GC_Extend), (<span class="string">'\u{10d24}'</span>, <span class="string">'\u{10d27}'</span>, GC_Extend),
        (<span class="string">'\u{10eab}'</span>, <span class="string">'\u{10eac}'</span>, GC_Extend), (<span class="string">'\u{10efd}'</span>, <span class="string">'\u{10eff}'</span>, GC_Extend), (<span class="string">'\u{10f46}'</span>,
        <span class="string">'\u{10f50}'</span>, GC_Extend), (<span class="string">'\u{10f82}'</span>, <span class="string">'\u{10f85}'</span>, GC_Extend), (<span class="string">'\u{11000}'</span>, <span class="string">'\u{11000}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11001}'</span>, <span class="string">'\u{11001}'</span>, GC_Extend), (<span class="string">'\u{11002}'</span>, <span class="string">'\u{11002}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11038}'</span>, <span class="string">'\u{11046}'</span>, GC_Extend), (<span class="string">'\u{11070}'</span>, <span class="string">'\u{11070}'</span>,
        GC_Extend), (<span class="string">'\u{11073}'</span>, <span class="string">'\u{11074}'</span>, GC_Extend), (<span class="string">'\u{1107f}'</span>, <span class="string">'\u{11081}'</span>, GC_Extend),
        (<span class="string">'\u{11082}'</span>, <span class="string">'\u{11082}'</span>, GC_SpacingMark), (<span class="string">'\u{110b0}'</span>, <span class="string">'\u{110b2}'</span>, GC_SpacingMark),
        (<span class="string">'\u{110b3}'</span>, <span class="string">'\u{110b6}'</span>, GC_Extend), (<span class="string">'\u{110b7}'</span>, <span class="string">'\u{110b8}'</span>, GC_SpacingMark),
        (<span class="string">'\u{110b9}'</span>, <span class="string">'\u{110ba}'</span>, GC_Extend), (<span class="string">'\u{110bd}'</span>, <span class="string">'\u{110bd}'</span>, GC_Prepend), (<span class="string">'\u{110c2}'</span>,
        <span class="string">'\u{110c2}'</span>, GC_Extend), (<span class="string">'\u{110cd}'</span>, <span class="string">'\u{110cd}'</span>, GC_Prepend), (<span class="string">'\u{11100}'</span>, <span class="string">'\u{11102}'</span>,
        GC_Extend), (<span class="string">'\u{11127}'</span>, <span class="string">'\u{1112b}'</span>, GC_Extend), (<span class="string">'\u{1112c}'</span>, <span class="string">'\u{1112c}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1112d}'</span>, <span class="string">'\u{11134}'</span>, GC_Extend), (<span class="string">'\u{11145}'</span>, <span class="string">'\u{11146}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11173}'</span>, <span class="string">'\u{11173}'</span>, GC_Extend), (<span class="string">'\u{11180}'</span>, <span class="string">'\u{11181}'</span>,
        GC_Extend), (<span class="string">'\u{11182}'</span>, <span class="string">'\u{11182}'</span>, GC_SpacingMark), (<span class="string">'\u{111b3}'</span>, <span class="string">'\u{111b5}'</span>,
        GC_SpacingMark), (<span class="string">'\u{111b6}'</span>, <span class="string">'\u{111be}'</span>, GC_Extend), (<span class="string">'\u{111bf}'</span>, <span class="string">'\u{111c0}'</span>,
        GC_SpacingMark), (<span class="string">'\u{111c2}'</span>, <span class="string">'\u{111c3}'</span>, GC_Prepend), (<span class="string">'\u{111c9}'</span>, <span class="string">'\u{111cc}'</span>,
        GC_Extend), (<span class="string">'\u{111ce}'</span>, <span class="string">'\u{111ce}'</span>, GC_SpacingMark), (<span class="string">'\u{111cf}'</span>, <span class="string">'\u{111cf}'</span>,
        GC_Extend), (<span class="string">'\u{1122c}'</span>, <span class="string">'\u{1122e}'</span>, GC_SpacingMark), (<span class="string">'\u{1122f}'</span>, <span class="string">'\u{11231}'</span>,
        GC_Extend), (<span class="string">'\u{11232}'</span>, <span class="string">'\u{11233}'</span>, GC_SpacingMark), (<span class="string">'\u{11234}'</span>, <span class="string">'\u{11234}'</span>,
        GC_Extend), (<span class="string">'\u{11235}'</span>, <span class="string">'\u{11235}'</span>, GC_SpacingMark), (<span class="string">'\u{11236}'</span>, <span class="string">'\u{11237}'</span>,
        GC_Extend), (<span class="string">'\u{1123e}'</span>, <span class="string">'\u{1123e}'</span>, GC_Extend), (<span class="string">'\u{11241}'</span>, <span class="string">'\u{11241}'</span>, GC_Extend),
        (<span class="string">'\u{112df}'</span>, <span class="string">'\u{112df}'</span>, GC_Extend), (<span class="string">'\u{112e0}'</span>, <span class="string">'\u{112e2}'</span>, GC_SpacingMark),
        (<span class="string">'\u{112e3}'</span>, <span class="string">'\u{112ea}'</span>, GC_Extend), (<span class="string">'\u{11300}'</span>, <span class="string">'\u{11301}'</span>, GC_Extend), (<span class="string">'\u{11302}'</span>,
        <span class="string">'\u{11303}'</span>, GC_SpacingMark), (<span class="string">'\u{1133b}'</span>, <span class="string">'\u{1133c}'</span>, GC_Extend), (<span class="string">'\u{1133e}'</span>,
        <span class="string">'\u{1133e}'</span>, GC_Extend), (<span class="string">'\u{1133f}'</span>, <span class="string">'\u{1133f}'</span>, GC_SpacingMark), (<span class="string">'\u{11340}'</span>,
        <span class="string">'\u{11340}'</span>, GC_Extend), (<span class="string">'\u{11341}'</span>, <span class="string">'\u{11344}'</span>, GC_SpacingMark), (<span class="string">'\u{11347}'</span>,
        <span class="string">'\u{11348}'</span>, GC_SpacingMark), (<span class="string">'\u{1134b}'</span>, <span class="string">'\u{1134d}'</span>, GC_SpacingMark), (<span class="string">'\u{11357}'</span>,
        <span class="string">'\u{11357}'</span>, GC_Extend), (<span class="string">'\u{11362}'</span>, <span class="string">'\u{11363}'</span>, GC_SpacingMark), (<span class="string">'\u{11366}'</span>,
        <span class="string">'\u{1136c}'</span>, GC_Extend), (<span class="string">'\u{11370}'</span>, <span class="string">'\u{11374}'</span>, GC_Extend), (<span class="string">'\u{11435}'</span>, <span class="string">'\u{11437}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11438}'</span>, <span class="string">'\u{1143f}'</span>, GC_Extend), (<span class="string">'\u{11440}'</span>, <span class="string">'\u{11441}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11442}'</span>, <span class="string">'\u{11444}'</span>, GC_Extend), (<span class="string">'\u{11445}'</span>, <span class="string">'\u{11445}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11446}'</span>, <span class="string">'\u{11446}'</span>, GC_Extend), (<span class="string">'\u{1145e}'</span>, <span class="string">'\u{1145e}'</span>,
        GC_Extend), (<span class="string">'\u{114b0}'</span>, <span class="string">'\u{114b0}'</span>, GC_Extend), (<span class="string">'\u{114b1}'</span>, <span class="string">'\u{114b2}'</span>,
        GC_SpacingMark), (<span class="string">'\u{114b3}'</span>, <span class="string">'\u{114b8}'</span>, GC_Extend), (<span class="string">'\u{114b9}'</span>, <span class="string">'\u{114b9}'</span>,
        GC_SpacingMark), (<span class="string">'\u{114ba}'</span>, <span class="string">'\u{114ba}'</span>, GC_Extend), (<span class="string">'\u{114bb}'</span>, <span class="string">'\u{114bc}'</span>,
        GC_SpacingMark), (<span class="string">'\u{114bd}'</span>, <span class="string">'\u{114bd}'</span>, GC_Extend), (<span class="string">'\u{114be}'</span>, <span class="string">'\u{114be}'</span>,
        GC_SpacingMark), (<span class="string">'\u{114bf}'</span>, <span class="string">'\u{114c0}'</span>, GC_Extend), (<span class="string">'\u{114c1}'</span>, <span class="string">'\u{114c1}'</span>,
        GC_SpacingMark), (<span class="string">'\u{114c2}'</span>, <span class="string">'\u{114c3}'</span>, GC_Extend), (<span class="string">'\u{115af}'</span>, <span class="string">'\u{115af}'</span>,
        GC_Extend), (<span class="string">'\u{115b0}'</span>, <span class="string">'\u{115b1}'</span>, GC_SpacingMark), (<span class="string">'\u{115b2}'</span>, <span class="string">'\u{115b5}'</span>,
        GC_Extend), (<span class="string">'\u{115b8}'</span>, <span class="string">'\u{115bb}'</span>, GC_SpacingMark), (<span class="string">'\u{115bc}'</span>, <span class="string">'\u{115bd}'</span>,
        GC_Extend), (<span class="string">'\u{115be}'</span>, <span class="string">'\u{115be}'</span>, GC_SpacingMark), (<span class="string">'\u{115bf}'</span>, <span class="string">'\u{115c0}'</span>,
        GC_Extend), (<span class="string">'\u{115dc}'</span>, <span class="string">'\u{115dd}'</span>, GC_Extend), (<span class="string">'\u{11630}'</span>, <span class="string">'\u{11632}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11633}'</span>, <span class="string">'\u{1163a}'</span>, GC_Extend), (<span class="string">'\u{1163b}'</span>, <span class="string">'\u{1163c}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1163d}'</span>, <span class="string">'\u{1163d}'</span>, GC_Extend), (<span class="string">'\u{1163e}'</span>, <span class="string">'\u{1163e}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1163f}'</span>, <span class="string">'\u{11640}'</span>, GC_Extend), (<span class="string">'\u{116ab}'</span>, <span class="string">'\u{116ab}'</span>,
        GC_Extend), (<span class="string">'\u{116ac}'</span>, <span class="string">'\u{116ac}'</span>, GC_SpacingMark), (<span class="string">'\u{116ad}'</span>, <span class="string">'\u{116ad}'</span>,
        GC_Extend), (<span class="string">'\u{116ae}'</span>, <span class="string">'\u{116af}'</span>, GC_SpacingMark), (<span class="string">'\u{116b0}'</span>, <span class="string">'\u{116b5}'</span>,
        GC_Extend), (<span class="string">'\u{116b6}'</span>, <span class="string">'\u{116b6}'</span>, GC_SpacingMark), (<span class="string">'\u{116b7}'</span>, <span class="string">'\u{116b7}'</span>,
        GC_Extend), (<span class="string">'\u{1171d}'</span>, <span class="string">'\u{1171f}'</span>, GC_Extend), (<span class="string">'\u{11722}'</span>, <span class="string">'\u{11725}'</span>, GC_Extend),
        (<span class="string">'\u{11726}'</span>, <span class="string">'\u{11726}'</span>, GC_SpacingMark), (<span class="string">'\u{11727}'</span>, <span class="string">'\u{1172b}'</span>, GC_Extend),
        (<span class="string">'\u{1182c}'</span>, <span class="string">'\u{1182e}'</span>, GC_SpacingMark), (<span class="string">'\u{1182f}'</span>, <span class="string">'\u{11837}'</span>, GC_Extend),
        (<span class="string">'\u{11838}'</span>, <span class="string">'\u{11838}'</span>, GC_SpacingMark), (<span class="string">'\u{11839}'</span>, <span class="string">'\u{1183a}'</span>, GC_Extend),
        (<span class="string">'\u{11930}'</span>, <span class="string">'\u{11930}'</span>, GC_Extend), (<span class="string">'\u{11931}'</span>, <span class="string">'\u{11935}'</span>, GC_SpacingMark),
        (<span class="string">'\u{11937}'</span>, <span class="string">'\u{11938}'</span>, GC_SpacingMark), (<span class="string">'\u{1193b}'</span>, <span class="string">'\u{1193c}'</span>, GC_Extend),
        (<span class="string">'\u{1193d}'</span>, <span class="string">'\u{1193d}'</span>, GC_SpacingMark), (<span class="string">'\u{1193e}'</span>, <span class="string">'\u{1193e}'</span>, GC_Extend),
        (<span class="string">'\u{1193f}'</span>, <span class="string">'\u{1193f}'</span>, GC_Prepend), (<span class="string">'\u{11940}'</span>, <span class="string">'\u{11940}'</span>, GC_SpacingMark),
        (<span class="string">'\u{11941}'</span>, <span class="string">'\u{11941}'</span>, GC_Prepend), (<span class="string">'\u{11942}'</span>, <span class="string">'\u{11942}'</span>, GC_SpacingMark),
        (<span class="string">'\u{11943}'</span>, <span class="string">'\u{11943}'</span>, GC_Extend), (<span class="string">'\u{119d1}'</span>, <span class="string">'\u{119d3}'</span>, GC_SpacingMark),
        (<span class="string">'\u{119d4}'</span>, <span class="string">'\u{119d7}'</span>, GC_Extend), (<span class="string">'\u{119da}'</span>, <span class="string">'\u{119db}'</span>, GC_Extend), (<span class="string">'\u{119dc}'</span>,
        <span class="string">'\u{119df}'</span>, GC_SpacingMark), (<span class="string">'\u{119e0}'</span>, <span class="string">'\u{119e0}'</span>, GC_Extend), (<span class="string">'\u{119e4}'</span>,
        <span class="string">'\u{119e4}'</span>, GC_SpacingMark), (<span class="string">'\u{11a01}'</span>, <span class="string">'\u{11a0a}'</span>, GC_Extend), (<span class="string">'\u{11a33}'</span>,
        <span class="string">'\u{11a38}'</span>, GC_Extend), (<span class="string">'\u{11a39}'</span>, <span class="string">'\u{11a39}'</span>, GC_SpacingMark), (<span class="string">'\u{11a3a}'</span>,
        <span class="string">'\u{11a3a}'</span>, GC_Prepend), (<span class="string">'\u{11a3b}'</span>, <span class="string">'\u{11a3e}'</span>, GC_Extend), (<span class="string">'\u{11a47}'</span>, <span class="string">'\u{11a47}'</span>,
        GC_Extend), (<span class="string">'\u{11a51}'</span>, <span class="string">'\u{11a56}'</span>, GC_Extend), (<span class="string">'\u{11a57}'</span>, <span class="string">'\u{11a58}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11a59}'</span>, <span class="string">'\u{11a5b}'</span>, GC_Extend), (<span class="string">'\u{11a84}'</span>, <span class="string">'\u{11a89}'</span>,
        GC_Prepend), (<span class="string">'\u{11a8a}'</span>, <span class="string">'\u{11a96}'</span>, GC_Extend), (<span class="string">'\u{11a97}'</span>, <span class="string">'\u{11a97}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11a98}'</span>, <span class="string">'\u{11a99}'</span>, GC_Extend), (<span class="string">'\u{11c2f}'</span>, <span class="string">'\u{11c2f}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11c30}'</span>, <span class="string">'\u{11c36}'</span>, GC_Extend), (<span class="string">'\u{11c38}'</span>, <span class="string">'\u{11c3d}'</span>,
        GC_Extend), (<span class="string">'\u{11c3e}'</span>, <span class="string">'\u{11c3e}'</span>, GC_SpacingMark), (<span class="string">'\u{11c3f}'</span>, <span class="string">'\u{11c3f}'</span>,
        GC_Extend), (<span class="string">'\u{11c92}'</span>, <span class="string">'\u{11ca7}'</span>, GC_Extend), (<span class="string">'\u{11ca9}'</span>, <span class="string">'\u{11ca9}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11caa}'</span>, <span class="string">'\u{11cb0}'</span>, GC_Extend), (<span class="string">'\u{11cb1}'</span>, <span class="string">'\u{11cb1}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11cb2}'</span>, <span class="string">'\u{11cb3}'</span>, GC_Extend), (<span class="string">'\u{11cb4}'</span>, <span class="string">'\u{11cb4}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11cb5}'</span>, <span class="string">'\u{11cb6}'</span>, GC_Extend), (<span class="string">'\u{11d31}'</span>, <span class="string">'\u{11d36}'</span>,
        GC_Extend), (<span class="string">'\u{11d3a}'</span>, <span class="string">'\u{11d3a}'</span>, GC_Extend), (<span class="string">'\u{11d3c}'</span>, <span class="string">'\u{11d3d}'</span>, GC_Extend),
        (<span class="string">'\u{11d3f}'</span>, <span class="string">'\u{11d45}'</span>, GC_Extend), (<span class="string">'\u{11d46}'</span>, <span class="string">'\u{11d46}'</span>, GC_Prepend), (<span class="string">'\u{11d47}'</span>,
        <span class="string">'\u{11d47}'</span>, GC_Extend), (<span class="string">'\u{11d8a}'</span>, <span class="string">'\u{11d8e}'</span>, GC_SpacingMark), (<span class="string">'\u{11d90}'</span>,
        <span class="string">'\u{11d91}'</span>, GC_Extend), (<span class="string">'\u{11d93}'</span>, <span class="string">'\u{11d94}'</span>, GC_SpacingMark), (<span class="string">'\u{11d95}'</span>,
        <span class="string">'\u{11d95}'</span>, GC_Extend), (<span class="string">'\u{11d96}'</span>, <span class="string">'\u{11d96}'</span>, GC_SpacingMark), (<span class="string">'\u{11d97}'</span>,
        <span class="string">'\u{11d97}'</span>, GC_Extend), (<span class="string">'\u{11ef3}'</span>, <span class="string">'\u{11ef4}'</span>, GC_Extend), (<span class="string">'\u{11ef5}'</span>, <span class="string">'\u{11ef6}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11f00}'</span>, <span class="string">'\u{11f01}'</span>, GC_Extend), (<span class="string">'\u{11f02}'</span>, <span class="string">'\u{11f02}'</span>,
        GC_Prepend), (<span class="string">'\u{11f03}'</span>, <span class="string">'\u{11f03}'</span>, GC_SpacingMark), (<span class="string">'\u{11f34}'</span>, <span class="string">'\u{11f35}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11f36}'</span>, <span class="string">'\u{11f3a}'</span>, GC_Extend), (<span class="string">'\u{11f3e}'</span>, <span class="string">'\u{11f3f}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11f40}'</span>, <span class="string">'\u{11f40}'</span>, GC_Extend), (<span class="string">'\u{11f41}'</span>, <span class="string">'\u{11f41}'</span>,
        GC_SpacingMark), (<span class="string">'\u{11f42}'</span>, <span class="string">'\u{11f42}'</span>, GC_Extend), (<span class="string">'\u{13430}'</span>, <span class="string">'\u{1343f}'</span>,
        GC_Control), (<span class="string">'\u{13440}'</span>, <span class="string">'\u{13440}'</span>, GC_Extend), (<span class="string">'\u{13447}'</span>, <span class="string">'\u{13455}'</span>, GC_Extend),
        (<span class="string">'\u{16af0}'</span>, <span class="string">'\u{16af4}'</span>, GC_Extend), (<span class="string">'\u{16b30}'</span>, <span class="string">'\u{16b36}'</span>, GC_Extend), (<span class="string">'\u{16f4f}'</span>,
        <span class="string">'\u{16f4f}'</span>, GC_Extend), (<span class="string">'\u{16f51}'</span>, <span class="string">'\u{16f87}'</span>, GC_SpacingMark), (<span class="string">'\u{16f8f}'</span>,
        <span class="string">'\u{16f92}'</span>, GC_Extend), (<span class="string">'\u{16fe4}'</span>, <span class="string">'\u{16fe4}'</span>, GC_Extend), (<span class="string">'\u{16ff0}'</span>, <span class="string">'\u{16ff1}'</span>,
        GC_SpacingMark), (<span class="string">'\u{1bc9d}'</span>, <span class="string">'\u{1bc9e}'</span>, GC_Extend), (<span class="string">'\u{1bca0}'</span>, <span class="string">'\u{1bca3}'</span>,
        GC_Control), (<span class="string">'\u{1cf00}'</span>, <span class="string">'\u{1cf2d}'</span>, GC_Extend), (<span class="string">'\u{1cf30}'</span>, <span class="string">'\u{1cf46}'</span>, GC_Extend),
        (<span class="string">'\u{1d165}'</span>, <span class="string">'\u{1d165}'</span>, GC_Extend), (<span class="string">'\u{1d166}'</span>, <span class="string">'\u{1d166}'</span>, GC_SpacingMark),
        (<span class="string">'\u{1d167}'</span>, <span class="string">'\u{1d169}'</span>, GC_Extend), (<span class="string">'\u{1d16d}'</span>, <span class="string">'\u{1d16d}'</span>, GC_SpacingMark),
        (<span class="string">'\u{1d16e}'</span>, <span class="string">'\u{1d172}'</span>, GC_Extend), (<span class="string">'\u{1d173}'</span>, <span class="string">'\u{1d17a}'</span>, GC_Control), (<span class="string">'\u{1d17b}'</span>,
        <span class="string">'\u{1d182}'</span>, GC_Extend), (<span class="string">'\u{1d185}'</span>, <span class="string">'\u{1d18b}'</span>, GC_Extend), (<span class="string">'\u{1d1aa}'</span>, <span class="string">'\u{1d1ad}'</span>,
        GC_Extend), (<span class="string">'\u{1d242}'</span>, <span class="string">'\u{1d244}'</span>, GC_Extend), (<span class="string">'\u{1da00}'</span>, <span class="string">'\u{1da36}'</span>, GC_Extend),
        (<span class="string">'\u{1da3b}'</span>, <span class="string">'\u{1da6c}'</span>, GC_Extend), (<span class="string">'\u{1da75}'</span>, <span class="string">'\u{1da75}'</span>, GC_Extend), (<span class="string">'\u{1da84}'</span>,
        <span class="string">'\u{1da84}'</span>, GC_Extend), (<span class="string">'\u{1da9b}'</span>, <span class="string">'\u{1da9f}'</span>, GC_Extend), (<span class="string">'\u{1daa1}'</span>, <span class="string">'\u{1daaf}'</span>,
        GC_Extend), (<span class="string">'\u{1e000}'</span>, <span class="string">'\u{1e006}'</span>, GC_Extend), (<span class="string">'\u{1e008}'</span>, <span class="string">'\u{1e018}'</span>, GC_Extend),
        (<span class="string">'\u{1e01b}'</span>, <span class="string">'\u{1e021}'</span>, GC_Extend), (<span class="string">'\u{1e023}'</span>, <span class="string">'\u{1e024}'</span>, GC_Extend), (<span class="string">'\u{1e026}'</span>,
        <span class="string">'\u{1e02a}'</span>, GC_Extend), (<span class="string">'\u{1e08f}'</span>, <span class="string">'\u{1e08f}'</span>, GC_Extend), (<span class="string">'\u{1e130}'</span>, <span class="string">'\u{1e136}'</span>,
        GC_Extend), (<span class="string">'\u{1e2ae}'</span>, <span class="string">'\u{1e2ae}'</span>, GC_Extend), (<span class="string">'\u{1e2ec}'</span>, <span class="string">'\u{1e2ef}'</span>, GC_Extend),
        (<span class="string">'\u{1e4ec}'</span>, <span class="string">'\u{1e4ef}'</span>, GC_Extend), (<span class="string">'\u{1e8d0}'</span>, <span class="string">'\u{1e8d6}'</span>, GC_Extend), (<span class="string">'\u{1e944}'</span>,
        <span class="string">'\u{1e94a}'</span>, GC_Extend), (<span class="string">'\u{1f000}'</span>, <span class="string">'\u{1f0ff}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f10d}'</span>,
        <span class="string">'\u{1f10f}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f12f}'</span>, <span class="string">'\u{1f12f}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{1f16c}'</span>, <span class="string">'\u{1f171}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{1f17e}'</span>, <span class="string">'\u{1f17f}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f18e}'</span>, <span class="string">'\u{1f18e}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{1f191}'</span>, <span class="string">'\u{1f19a}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{1f1ad}'</span>, <span class="string">'\u{1f1e5}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f1e6}'</span>, <span class="string">'\u{1f1ff}'</span>,
        GC_Regional_Indicator), (<span class="string">'\u{1f201}'</span>, <span class="string">'\u{1f20f}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f21a}'</span>,
        <span class="string">'\u{1f21a}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f22f}'</span>, <span class="string">'\u{1f22f}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{1f232}'</span>, <span class="string">'\u{1f23a}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{1f23c}'</span>, <span class="string">'\u{1f23f}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f249}'</span>, <span class="string">'\u{1f3fa}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{1f3fb}'</span>, <span class="string">'\u{1f3ff}'</span>, GC_Extend), (<span class="string">'\u{1f400}'</span>, <span class="string">'\u{1f53d}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{1f546}'</span>, <span class="string">'\u{1f64f}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{1f680}'</span>, <span class="string">'\u{1f6ff}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f774}'</span>, <span class="string">'\u{1f77f}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{1f7d5}'</span>, <span class="string">'\u{1f7ff}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{1f80c}'</span>, <span class="string">'\u{1f80f}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f848}'</span>, <span class="string">'\u{1f84f}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{1f85a}'</span>, <span class="string">'\u{1f85f}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{1f888}'</span>, <span class="string">'\u{1f88f}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f8ae}'</span>, <span class="string">'\u{1f8ff}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{1f90c}'</span>, <span class="string">'\u{1f93a}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{1f93c}'</span>, <span class="string">'\u{1f945}'</span>, GC_Extended_Pictographic), (<span class="string">'\u{1f947}'</span>, <span class="string">'\u{1faff}'</span>,
        GC_Extended_Pictographic), (<span class="string">'\u{1fc00}'</span>, <span class="string">'\u{1fffd}'</span>, GC_Extended_Pictographic),
        (<span class="string">'\u{e0000}'</span>, <span class="string">'\u{e001f}'</span>, GC_Control), (<span class="string">'\u{e0020}'</span>, <span class="string">'\u{e007f}'</span>, GC_Extend), (<span class="string">'\u{e0080}'</span>,
        <span class="string">'\u{e00ff}'</span>, GC_Control), (<span class="string">'\u{e0100}'</span>, <span class="string">'\u{e01ef}'</span>, GC_Extend), (<span class="string">'\u{e01f0}'</span>, <span class="string">'\u{e0fff}'</span>,
        GC_Control)
    ];

}

<span class="kw">pub mod </span>word {
    <span class="kw">use </span>core::result::Result::{<span class="prelude-val">Ok</span>, <span class="prelude-val">Err</span>};

    <span class="kw">pub use </span><span class="self">self</span>::WordCat::<span class="kw-2">*</span>;

    <span class="attr">#[allow(non_camel_case_types)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    </span><span class="kw">pub enum </span>WordCat {
        WC_ALetter,
        WC_Any,
        WC_CR,
        WC_Double_Quote,
        WC_Extend,
        WC_ExtendNumLet,
        WC_Format,
        WC_Hebrew_Letter,
        WC_Katakana,
        WC_LF,
        WC_MidLetter,
        WC_MidNum,
        WC_MidNumLet,
        WC_Newline,
        WC_Numeric,
        WC_Regional_Indicator,
        WC_Single_Quote,
        WC_WSegSpace,
        WC_ZWJ,
    }

    <span class="kw">fn </span>bsearch_range_value_table(c: char, r: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char, WordCat)], default_lower: u32, default_upper: u32) -&gt; (u32, u32, WordCat) {
        <span class="kw">use </span>core::cmp::Ordering::{Equal, Less, Greater};
        <span class="kw">match </span>r.binary_search_by(|<span class="kw-2">&amp;</span>(lo, hi, <span class="kw">_</span>)| {
            <span class="kw">if </span>lo &lt;= c &amp;&amp; c &lt;= hi { Equal }
            <span class="kw">else if </span>hi &lt; c { Less }
            <span class="kw">else </span>{ Greater }
        }) {
            <span class="prelude-val">Ok</span>(idx) =&gt; {
                <span class="kw">let </span>(lower, upper, cat) = r[idx];
                (lower <span class="kw">as </span>u32, upper <span class="kw">as </span>u32, cat)
            }
            <span class="prelude-val">Err</span>(idx) =&gt; {
                (
                    <span class="kw">if </span>idx &gt; <span class="number">0 </span>{ r[idx-<span class="number">1</span>].<span class="number">1 </span><span class="kw">as </span>u32 + <span class="number">1 </span>} <span class="kw">else </span>{ default_lower },
                    r.get(idx).map(|c|c.<span class="number">0 </span><span class="kw">as </span>u32 - <span class="number">1</span>).unwrap_or(default_upper),
                    WC_Any,
                )
            }
        }
    }

    <span class="kw">pub fn </span>word_category(c: char) -&gt; (u32, u32, WordCat) {
        <span class="comment">// Perform a quick O(1) lookup in a precomputed table to determine
        // the slice of the range table to search in.
        </span><span class="kw">let </span>lookup_interval = <span class="number">0x80</span>;
        <span class="kw">let </span>idx = (c <span class="kw">as </span>u32 / lookup_interval) <span class="kw">as </span>usize;
        <span class="kw">let </span>range = word_cat_lookup.get(idx..(idx + <span class="number">2</span>)).map_or(
          <span class="comment">// If the `idx` is outside of the precomputed table - use the slice
          // starting from the last covered index in the precomputed table and
          // ending with the length of the range table.
          </span><span class="number">1049</span>..<span class="number">1052</span>,
          |r| (r[<span class="number">0</span>] <span class="kw">as </span>usize)..((r[<span class="number">1</span>] + <span class="number">1</span>) <span class="kw">as </span>usize)
        );

        <span class="comment">// Compute pessimistic default lower and upper bounds on the category.
        // If character doesn't map to any range and there is no adjacent range
        // in the table slice - these bounds has to apply.
        </span><span class="kw">let </span>lower = idx <span class="kw">as </span>u32 * lookup_interval;
        <span class="kw">let </span>upper = lower + lookup_interval - <span class="number">1</span>;
        bsearch_range_value_table(c, <span class="kw-2">&amp;</span>word_cat_table[range], lower, upper)
    }

    <span class="kw">const </span>word_cat_lookup: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[u16] = <span class="kw-2">&amp;</span>[
        <span class="number">0</span>, <span class="number">14</span>, <span class="number">22</span>, <span class="number">22</span>, <span class="number">22</span>, <span class="number">22</span>, <span class="number">24</span>, <span class="number">30</span>, <span class="number">36</span>, <span class="number">36</span>, <span class="number">38</span>, <span class="number">43</span>, <span class="number">55</span>, <span class="number">66</span>, <span class="number">78</span>, <span class="number">82</span>, <span class="number">92</span>, <span class="number">103</span>, <span class="number">110</span>, <span class="number">120</span>, <span class="number">142</span>, <span class="number">161</span>,
        <span class="number">179</span>, <span class="number">197</span>, <span class="number">214</span>, <span class="number">230</span>, <span class="number">249</span>, <span class="number">265</span>, <span class="number">277</span>, <span class="number">281</span>, <span class="number">285</span>, <span class="number">294</span>, <span class="number">300</span>, <span class="number">307</span>, <span class="number">315</span>, <span class="number">315</span>, <span class="number">315</span>, <span class="number">320</span>, <span class="number">328</span>, <span class="number">332</span>,
        <span class="number">335</span>, <span class="number">335</span>, <span class="number">335</span>, <span class="number">335</span>, <span class="number">335</span>, <span class="number">337</span>, <span class="number">341</span>, <span class="number">350</span>, <span class="number">353</span>, <span class="number">358</span>, <span class="number">364</span>, <span class="number">368</span>, <span class="number">369</span>, <span class="number">374</span>, <span class="number">377</span>, <span class="number">383</span>, <span class="number">390</span>, <span class="number">396</span>,
        <span class="number">408</span>, <span class="number">408</span>, <span class="number">410</span>, <span class="number">410</span>, <span class="number">410</span>, <span class="number">419</span>, <span class="number">429</span>, <span class="number">448</span>, <span class="number">450</span>, <span class="number">463</span>, <span class="number">464</span>, <span class="number">464</span>, <span class="number">464</span>, <span class="number">464</span>, <span class="number">464</span>, <span class="number">464</span>, <span class="number">465</span>, <span class="number">465</span>,
        <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">465</span>, <span class="number">469</span>, <span class="number">475</span>, <span class="number">485</span>, <span class="number">486</span>,
        <span class="number">486</span>, <span class="number">486</span>, <span class="number">486</span>, <span class="number">491</span>, <span class="number">495</span>, <span class="number">496</span>, <span class="number">499</span>, <span class="number">499</span>, <span class="number">500</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>, <span class="number">501</span>,
        <span class="number">501</span>, <span class="number">501</span>, <span class="number">503</span>, <span class="number">503</span>, <span class="number">503</span>, <span class="number">510</span>, <span class="number">514</span>, <span class="number">514</span>, <span class="number">518</span>, <span class="number">528</span>, <span class="number">537</span>, <span class="number">543</span>, <span class="number">550</span>, <span class="number">558</span>, <span class="number">567</span>, <span class="number">573</span>, <span class="number">577</span>, <span class="number">577</span>,
        <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>,
        <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>,
        <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>,
        <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>,
        <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">577</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>,
        <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>,
        <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>,
        <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>,
        <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">580</span>, <span class="number">591</span>, <span class="number">592</span>, <span class="number">592</span>, <span class="number">592</span>, <span class="number">593</span>,
        <span class="number">596</span>, <span class="number">608</span>, <span class="number">610</span>, <span class="number">619</span>, <span class="number">627</span>, <span class="number">633</span>, <span class="number">634</span>, <span class="number">635</span>, <span class="number">636</span>, <span class="number">636</span>, <span class="number">639</span>, <span class="number">643</span>, <span class="number">647</span>, <span class="number">647</span>, <span class="number">651</span>, <span class="number">654</span>, <span class="number">661</span>, <span class="number">661</span>,
        <span class="number">661</span>, <span class="number">664</span>, <span class="number">667</span>, <span class="number">674</span>, <span class="number">677</span>, <span class="number">679</span>, <span class="number">681</span>, <span class="number">691</span>, <span class="number">695</span>, <span class="number">698</span>, <span class="number">699</span>, <span class="number">700</span>, <span class="number">702</span>, <span class="number">705</span>, <span class="number">705</span>, <span class="number">705</span>, <span class="number">709</span>, <span class="number">713</span>,
        <span class="number">717</span>, <span class="number">725</span>, <span class="number">733</span>, <span class="number">743</span>, <span class="number">752</span>, <span class="number">758</span>, <span class="number">766</span>, <span class="number">784</span>, <span class="number">784</span>, <span class="number">790</span>, <span class="number">795</span>, <span class="number">795</span>, <span class="number">800</span>, <span class="number">804</span>, <span class="number">808</span>, <span class="number">810</span>, <span class="number">810</span>, <span class="number">812</span>,
        <span class="number">814</span>, <span class="number">827</span>, <span class="number">834</span>, <span class="number">843</span>, <span class="number">847</span>, <span class="number">847</span>, <span class="number">847</span>, <span class="number">853</span>, <span class="number">856</span>, <span class="number">868</span>, <span class="number">874</span>, <span class="number">874</span>, <span class="number">876</span>, <span class="number">884</span>, <span class="number">885</span>, <span class="number">885</span>, <span class="number">885</span>, <span class="number">885</span>,
        <span class="number">885</span>, <span class="number">885</span>, <span class="number">885</span>, <span class="number">885</span>, <span class="number">886</span>, <span class="number">887</span>, <span class="number">887</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>,
        <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">888</span>, <span class="number">889</span>, <span class="number">889</span>, <span class="number">889</span>, <span class="number">889</span>, <span class="number">889</span>, <span class="number">889</span>, <span class="number">889</span>, <span class="number">889</span>,
        <span class="number">889</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>,
        <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>, <span class="number">894</span>,
        <span class="number">894</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>,
        <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>,
        <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>,
        <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>, <span class="number">895</span>,
        <span class="number">895</span>, <span class="number">898</span>, <span class="number">902</span>, <span class="number">907</span>, <span class="number">908</span>, <span class="number">908</span>, <span class="number">908</span>, <span class="number">908</span>, <span class="number">908</span>, <span class="number">909</span>, <span class="number">909</span>, <span class="number">912</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>,
        <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>,
        <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>,
        <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>,
        <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>,
        <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>,
        <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>,
        <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">919</span>, <span class="number">922</span>, <span class="number">923</span>, <span class="number">923</span>, <span class="number">926</span>,
        <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>,
        <span class="number">926</span>, <span class="number">926</span>, <span class="number">926</span>, <span class="number">928</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>,
        <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>,
        <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">932</span>, <span class="number">934</span>, <span class="number">934</span>, <span class="number">934</span>, <span class="number">934</span>, <span class="number">937</span>, <span class="number">940</span>, <span class="number">941</span>, <span class="number">941</span>, <span class="number">941</span>, <span class="number">941</span>, <span class="number">942</span>, <span class="number">950</span>, <span class="number">959</span>,
        <span class="number">959</span>, <span class="number">959</span>, <span class="number">963</span>, <span class="number">967</span>, <span class="number">972</span>, <span class="number">972</span>, <span class="number">972</span>, <span class="number">972</span>, <span class="number">972</span>, <span class="number">975</span>, <span class="number">978</span>, <span class="number">978</span>, <span class="number">978</span>, <span class="number">978</span>, <span class="number">978</span>, <span class="number">978</span>, <span class="number">978</span>, <span class="number">978</span>,
        <span class="number">978</span>, <span class="number">980</span>, <span class="number">980</span>, <span class="number">986</span>, <span class="number">987</span>, <span class="number">992</span>, <span class="number">992</span>, <span class="number">992</span>, <span class="number">997</span>, <span class="number">997</span>, <span class="number">997</span>, <span class="number">997</span>, <span class="number">1000</span>, <span class="number">1000</span>, <span class="number">1000</span>, <span class="number">1000</span>, <span class="number">1000</span>,
        <span class="number">1000</span>, <span class="number">1004</span>, <span class="number">1004</span>, <span class="number">1006</span>, <span class="number">1010</span>, <span class="number">1010</span>, <span class="number">1010</span>, <span class="number">1010</span>, <span class="number">1010</span>, <span class="number">1010</span>, <span class="number">1010</span>, <span class="number">1010</span>, <span class="number">1010</span>, <span class="number">1010</span>, <span class="number">1038</span>,
        <span class="number">1043</span>, <span class="number">1043</span>, <span class="number">1043</span>, <span class="number">1043</span>, <span class="number">1043</span>, <span class="number">1045</span>, <span class="number">1047</span>, <span class="number">1047</span>, <span class="number">1047</span>, <span class="number">1047</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>,
        <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1048</span>, <span class="number">1049</span>, <span class="number">1049</span>, <span class="number">1049</span>, <span class="number">1049</span>,
        <span class="number">1049</span>, <span class="number">1049</span>, <span class="number">1049</span>, <span class="number">1049
    </span>];

    <span class="kw">const </span>word_cat_table: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char, WordCat)] = <span class="kw-2">&amp;</span>[
        (<span class="string">'\u{a}'</span>, <span class="string">'\u{a}'</span>, WC_LF), (<span class="string">'\u{b}'</span>, <span class="string">'\u{c}'</span>, WC_Newline), (<span class="string">'\u{d}'</span>, <span class="string">'\u{d}'</span>, WC_CR),
        (<span class="string">'\u{20}'</span>, <span class="string">'\u{20}'</span>, WC_WSegSpace), (<span class="string">'\u{22}'</span>, <span class="string">'\u{22}'</span>, WC_Double_Quote), (<span class="string">'\u{27}'</span>,
        <span class="string">'\u{27}'</span>, WC_Single_Quote), (<span class="string">'\u{2c}'</span>, <span class="string">'\u{2c}'</span>, WC_MidNum), (<span class="string">'\u{2e}'</span>, <span class="string">'\u{2e}'</span>,
        WC_MidNumLet), (<span class="string">'\u{30}'</span>, <span class="string">'\u{39}'</span>, WC_Numeric), (<span class="string">'\u{3a}'</span>, <span class="string">'\u{3a}'</span>, WC_MidLetter),
        (<span class="string">'\u{3b}'</span>, <span class="string">'\u{3b}'</span>, WC_MidNum), (<span class="string">'\u{41}'</span>, <span class="string">'\u{5a}'</span>, WC_ALetter), (<span class="string">'\u{5f}'</span>, <span class="string">'\u{5f}'</span>,
        WC_ExtendNumLet), (<span class="string">'\u{61}'</span>, <span class="string">'\u{7a}'</span>, WC_ALetter), (<span class="string">'\u{85}'</span>, <span class="string">'\u{85}'</span>, WC_Newline),
        (<span class="string">'\u{aa}'</span>, <span class="string">'\u{aa}'</span>, WC_ALetter), (<span class="string">'\u{ad}'</span>, <span class="string">'\u{ad}'</span>, WC_Format), (<span class="string">'\u{b5}'</span>, <span class="string">'\u{b5}'</span>,
        WC_ALetter), (<span class="string">'\u{b7}'</span>, <span class="string">'\u{b7}'</span>, WC_MidLetter), (<span class="string">'\u{ba}'</span>, <span class="string">'\u{ba}'</span>, WC_ALetter),
        (<span class="string">'\u{c0}'</span>, <span class="string">'\u{d6}'</span>, WC_ALetter), (<span class="string">'\u{d8}'</span>, <span class="string">'\u{f6}'</span>, WC_ALetter), (<span class="string">'\u{f8}'</span>, <span class="string">'\u{2d7}'</span>,
        WC_ALetter), (<span class="string">'\u{2de}'</span>, <span class="string">'\u{2ff}'</span>, WC_ALetter), (<span class="string">'\u{300}'</span>, <span class="string">'\u{36f}'</span>, WC_Extend),
        (<span class="string">'\u{370}'</span>, <span class="string">'\u{374}'</span>, WC_ALetter), (<span class="string">'\u{376}'</span>, <span class="string">'\u{377}'</span>, WC_ALetter), (<span class="string">'\u{37a}'</span>,
        <span class="string">'\u{37d}'</span>, WC_ALetter), (<span class="string">'\u{37e}'</span>, <span class="string">'\u{37e}'</span>, WC_MidNum), (<span class="string">'\u{37f}'</span>, <span class="string">'\u{37f}'</span>,
        WC_ALetter), (<span class="string">'\u{386}'</span>, <span class="string">'\u{386}'</span>, WC_ALetter), (<span class="string">'\u{387}'</span>, <span class="string">'\u{387}'</span>, WC_MidLetter),
        (<span class="string">'\u{388}'</span>, <span class="string">'\u{38a}'</span>, WC_ALetter), (<span class="string">'\u{38c}'</span>, <span class="string">'\u{38c}'</span>, WC_ALetter), (<span class="string">'\u{38e}'</span>,
        <span class="string">'\u{3a1}'</span>, WC_ALetter), (<span class="string">'\u{3a3}'</span>, <span class="string">'\u{3f5}'</span>, WC_ALetter), (<span class="string">'\u{3f7}'</span>, <span class="string">'\u{481}'</span>,
        WC_ALetter), (<span class="string">'\u{483}'</span>, <span class="string">'\u{489}'</span>, WC_Extend), (<span class="string">'\u{48a}'</span>, <span class="string">'\u{52f}'</span>, WC_ALetter),
        (<span class="string">'\u{531}'</span>, <span class="string">'\u{556}'</span>, WC_ALetter), (<span class="string">'\u{559}'</span>, <span class="string">'\u{55c}'</span>, WC_ALetter), (<span class="string">'\u{55e}'</span>,
        <span class="string">'\u{55e}'</span>, WC_ALetter), (<span class="string">'\u{55f}'</span>, <span class="string">'\u{55f}'</span>, WC_MidLetter), (<span class="string">'\u{560}'</span>, <span class="string">'\u{588}'</span>,
        WC_ALetter), (<span class="string">'\u{589}'</span>, <span class="string">'\u{589}'</span>, WC_MidNum), (<span class="string">'\u{58a}'</span>, <span class="string">'\u{58a}'</span>, WC_ALetter),
        (<span class="string">'\u{591}'</span>, <span class="string">'\u{5bd}'</span>, WC_Extend), (<span class="string">'\u{5bf}'</span>, <span class="string">'\u{5bf}'</span>, WC_Extend), (<span class="string">'\u{5c1}'</span>, <span class="string">'\u{5c2}'</span>,
        WC_Extend), (<span class="string">'\u{5c4}'</span>, <span class="string">'\u{5c5}'</span>, WC_Extend), (<span class="string">'\u{5c7}'</span>, <span class="string">'\u{5c7}'</span>, WC_Extend),
        (<span class="string">'\u{5d0}'</span>, <span class="string">'\u{5ea}'</span>, WC_Hebrew_Letter), (<span class="string">'\u{5ef}'</span>, <span class="string">'\u{5f2}'</span>, WC_Hebrew_Letter),
        (<span class="string">'\u{5f3}'</span>, <span class="string">'\u{5f3}'</span>, WC_ALetter), (<span class="string">'\u{5f4}'</span>, <span class="string">'\u{5f4}'</span>, WC_MidLetter), (<span class="string">'\u{600}'</span>,
        <span class="string">'\u{605}'</span>, WC_Numeric), (<span class="string">'\u{60c}'</span>, <span class="string">'\u{60d}'</span>, WC_MidNum), (<span class="string">'\u{610}'</span>, <span class="string">'\u{61a}'</span>,
        WC_Extend), (<span class="string">'\u{61c}'</span>, <span class="string">'\u{61c}'</span>, WC_Format), (<span class="string">'\u{620}'</span>, <span class="string">'\u{64a}'</span>, WC_ALetter),
        (<span class="string">'\u{64b}'</span>, <span class="string">'\u{65f}'</span>, WC_Extend), (<span class="string">'\u{660}'</span>, <span class="string">'\u{669}'</span>, WC_Numeric), (<span class="string">'\u{66b}'</span>,
        <span class="string">'\u{66b}'</span>, WC_Numeric), (<span class="string">'\u{66c}'</span>, <span class="string">'\u{66c}'</span>, WC_MidNum), (<span class="string">'\u{66e}'</span>, <span class="string">'\u{66f}'</span>,
        WC_ALetter), (<span class="string">'\u{670}'</span>, <span class="string">'\u{670}'</span>, WC_Extend), (<span class="string">'\u{671}'</span>, <span class="string">'\u{6d3}'</span>, WC_ALetter),
        (<span class="string">'\u{6d5}'</span>, <span class="string">'\u{6d5}'</span>, WC_ALetter), (<span class="string">'\u{6d6}'</span>, <span class="string">'\u{6dc}'</span>, WC_Extend), (<span class="string">'\u{6dd}'</span>,
        <span class="string">'\u{6dd}'</span>, WC_Numeric), (<span class="string">'\u{6df}'</span>, <span class="string">'\u{6e4}'</span>, WC_Extend), (<span class="string">'\u{6e5}'</span>, <span class="string">'\u{6e6}'</span>,
        WC_ALetter), (<span class="string">'\u{6e7}'</span>, <span class="string">'\u{6e8}'</span>, WC_Extend), (<span class="string">'\u{6ea}'</span>, <span class="string">'\u{6ed}'</span>, WC_Extend),
        (<span class="string">'\u{6ee}'</span>, <span class="string">'\u{6ef}'</span>, WC_ALetter), (<span class="string">'\u{6f0}'</span>, <span class="string">'\u{6f9}'</span>, WC_Numeric), (<span class="string">'\u{6fa}'</span>,
        <span class="string">'\u{6fc}'</span>, WC_ALetter), (<span class="string">'\u{6ff}'</span>, <span class="string">'\u{6ff}'</span>, WC_ALetter), (<span class="string">'\u{70f}'</span>, <span class="string">'\u{710}'</span>,
        WC_ALetter), (<span class="string">'\u{711}'</span>, <span class="string">'\u{711}'</span>, WC_Extend), (<span class="string">'\u{712}'</span>, <span class="string">'\u{72f}'</span>, WC_ALetter),
        (<span class="string">'\u{730}'</span>, <span class="string">'\u{74a}'</span>, WC_Extend), (<span class="string">'\u{74d}'</span>, <span class="string">'\u{7a5}'</span>, WC_ALetter), (<span class="string">'\u{7a6}'</span>,
        <span class="string">'\u{7b0}'</span>, WC_Extend), (<span class="string">'\u{7b1}'</span>, <span class="string">'\u{7b1}'</span>, WC_ALetter), (<span class="string">'\u{7c0}'</span>, <span class="string">'\u{7c9}'</span>,
        WC_Numeric), (<span class="string">'\u{7ca}'</span>, <span class="string">'\u{7ea}'</span>, WC_ALetter), (<span class="string">'\u{7eb}'</span>, <span class="string">'\u{7f3}'</span>, WC_Extend),
        (<span class="string">'\u{7f4}'</span>, <span class="string">'\u{7f5}'</span>, WC_ALetter), (<span class="string">'\u{7f8}'</span>, <span class="string">'\u{7f8}'</span>, WC_MidNum), (<span class="string">'\u{7fa}'</span>,
        <span class="string">'\u{7fa}'</span>, WC_ALetter), (<span class="string">'\u{7fd}'</span>, <span class="string">'\u{7fd}'</span>, WC_Extend), (<span class="string">'\u{800}'</span>, <span class="string">'\u{815}'</span>,
        WC_ALetter), (<span class="string">'\u{816}'</span>, <span class="string">'\u{819}'</span>, WC_Extend), (<span class="string">'\u{81a}'</span>, <span class="string">'\u{81a}'</span>, WC_ALetter),
        (<span class="string">'\u{81b}'</span>, <span class="string">'\u{823}'</span>, WC_Extend), (<span class="string">'\u{824}'</span>, <span class="string">'\u{824}'</span>, WC_ALetter), (<span class="string">'\u{825}'</span>,
        <span class="string">'\u{827}'</span>, WC_Extend), (<span class="string">'\u{828}'</span>, <span class="string">'\u{828}'</span>, WC_ALetter), (<span class="string">'\u{829}'</span>, <span class="string">'\u{82d}'</span>,
        WC_Extend), (<span class="string">'\u{840}'</span>, <span class="string">'\u{858}'</span>, WC_ALetter), (<span class="string">'\u{859}'</span>, <span class="string">'\u{85b}'</span>, WC_Extend),
        (<span class="string">'\u{860}'</span>, <span class="string">'\u{86a}'</span>, WC_ALetter), (<span class="string">'\u{870}'</span>, <span class="string">'\u{887}'</span>, WC_ALetter), (<span class="string">'\u{889}'</span>,
        <span class="string">'\u{88e}'</span>, WC_ALetter), (<span class="string">'\u{890}'</span>, <span class="string">'\u{891}'</span>, WC_Numeric), (<span class="string">'\u{898}'</span>, <span class="string">'\u{89f}'</span>,
        WC_Extend), (<span class="string">'\u{8a0}'</span>, <span class="string">'\u{8c9}'</span>, WC_ALetter), (<span class="string">'\u{8ca}'</span>, <span class="string">'\u{8e1}'</span>, WC_Extend),
        (<span class="string">'\u{8e2}'</span>, <span class="string">'\u{8e2}'</span>, WC_Numeric), (<span class="string">'\u{8e3}'</span>, <span class="string">'\u{903}'</span>, WC_Extend), (<span class="string">'\u{904}'</span>,
        <span class="string">'\u{939}'</span>, WC_ALetter), (<span class="string">'\u{93a}'</span>, <span class="string">'\u{93c}'</span>, WC_Extend), (<span class="string">'\u{93d}'</span>, <span class="string">'\u{93d}'</span>,
        WC_ALetter), (<span class="string">'\u{93e}'</span>, <span class="string">'\u{94f}'</span>, WC_Extend), (<span class="string">'\u{950}'</span>, <span class="string">'\u{950}'</span>, WC_ALetter),
        (<span class="string">'\u{951}'</span>, <span class="string">'\u{957}'</span>, WC_Extend), (<span class="string">'\u{958}'</span>, <span class="string">'\u{961}'</span>, WC_ALetter), (<span class="string">'\u{962}'</span>,
        <span class="string">'\u{963}'</span>, WC_Extend), (<span class="string">'\u{966}'</span>, <span class="string">'\u{96f}'</span>, WC_Numeric), (<span class="string">'\u{971}'</span>, <span class="string">'\u{980}'</span>,
        WC_ALetter), (<span class="string">'\u{981}'</span>, <span class="string">'\u{983}'</span>, WC_Extend), (<span class="string">'\u{985}'</span>, <span class="string">'\u{98c}'</span>, WC_ALetter),
        (<span class="string">'\u{98f}'</span>, <span class="string">'\u{990}'</span>, WC_ALetter), (<span class="string">'\u{993}'</span>, <span class="string">'\u{9a8}'</span>, WC_ALetter), (<span class="string">'\u{9aa}'</span>,
        <span class="string">'\u{9b0}'</span>, WC_ALetter), (<span class="string">'\u{9b2}'</span>, <span class="string">'\u{9b2}'</span>, WC_ALetter), (<span class="string">'\u{9b6}'</span>, <span class="string">'\u{9b9}'</span>,
        WC_ALetter), (<span class="string">'\u{9bc}'</span>, <span class="string">'\u{9bc}'</span>, WC_Extend), (<span class="string">'\u{9bd}'</span>, <span class="string">'\u{9bd}'</span>, WC_ALetter),
        (<span class="string">'\u{9be}'</span>, <span class="string">'\u{9c4}'</span>, WC_Extend), (<span class="string">'\u{9c7}'</span>, <span class="string">'\u{9c8}'</span>, WC_Extend), (<span class="string">'\u{9cb}'</span>, <span class="string">'\u{9cd}'</span>,
        WC_Extend), (<span class="string">'\u{9ce}'</span>, <span class="string">'\u{9ce}'</span>, WC_ALetter), (<span class="string">'\u{9d7}'</span>, <span class="string">'\u{9d7}'</span>, WC_Extend),
        (<span class="string">'\u{9dc}'</span>, <span class="string">'\u{9dd}'</span>, WC_ALetter), (<span class="string">'\u{9df}'</span>, <span class="string">'\u{9e1}'</span>, WC_ALetter), (<span class="string">'\u{9e2}'</span>,
        <span class="string">'\u{9e3}'</span>, WC_Extend), (<span class="string">'\u{9e6}'</span>, <span class="string">'\u{9ef}'</span>, WC_Numeric), (<span class="string">'\u{9f0}'</span>, <span class="string">'\u{9f1}'</span>,
        WC_ALetter), (<span class="string">'\u{9fc}'</span>, <span class="string">'\u{9fc}'</span>, WC_ALetter), (<span class="string">'\u{9fe}'</span>, <span class="string">'\u{9fe}'</span>, WC_Extend),
        (<span class="string">'\u{a01}'</span>, <span class="string">'\u{a03}'</span>, WC_Extend), (<span class="string">'\u{a05}'</span>, <span class="string">'\u{a0a}'</span>, WC_ALetter), (<span class="string">'\u{a0f}'</span>,
        <span class="string">'\u{a10}'</span>, WC_ALetter), (<span class="string">'\u{a13}'</span>, <span class="string">'\u{a28}'</span>, WC_ALetter), (<span class="string">'\u{a2a}'</span>, <span class="string">'\u{a30}'</span>,
        WC_ALetter), (<span class="string">'\u{a32}'</span>, <span class="string">'\u{a33}'</span>, WC_ALetter), (<span class="string">'\u{a35}'</span>, <span class="string">'\u{a36}'</span>, WC_ALetter),
        (<span class="string">'\u{a38}'</span>, <span class="string">'\u{a39}'</span>, WC_ALetter), (<span class="string">'\u{a3c}'</span>, <span class="string">'\u{a3c}'</span>, WC_Extend), (<span class="string">'\u{a3e}'</span>,
        <span class="string">'\u{a42}'</span>, WC_Extend), (<span class="string">'\u{a47}'</span>, <span class="string">'\u{a48}'</span>, WC_Extend), (<span class="string">'\u{a4b}'</span>, <span class="string">'\u{a4d}'</span>, WC_Extend),
        (<span class="string">'\u{a51}'</span>, <span class="string">'\u{a51}'</span>, WC_Extend), (<span class="string">'\u{a59}'</span>, <span class="string">'\u{a5c}'</span>, WC_ALetter), (<span class="string">'\u{a5e}'</span>,
        <span class="string">'\u{a5e}'</span>, WC_ALetter), (<span class="string">'\u{a66}'</span>, <span class="string">'\u{a6f}'</span>, WC_Numeric), (<span class="string">'\u{a70}'</span>, <span class="string">'\u{a71}'</span>,
        WC_Extend), (<span class="string">'\u{a72}'</span>, <span class="string">'\u{a74}'</span>, WC_ALetter), (<span class="string">'\u{a75}'</span>, <span class="string">'\u{a75}'</span>, WC_Extend),
        (<span class="string">'\u{a81}'</span>, <span class="string">'\u{a83}'</span>, WC_Extend), (<span class="string">'\u{a85}'</span>, <span class="string">'\u{a8d}'</span>, WC_ALetter), (<span class="string">'\u{a8f}'</span>,
        <span class="string">'\u{a91}'</span>, WC_ALetter), (<span class="string">'\u{a93}'</span>, <span class="string">'\u{aa8}'</span>, WC_ALetter), (<span class="string">'\u{aaa}'</span>, <span class="string">'\u{ab0}'</span>,
        WC_ALetter), (<span class="string">'\u{ab2}'</span>, <span class="string">'\u{ab3}'</span>, WC_ALetter), (<span class="string">'\u{ab5}'</span>, <span class="string">'\u{ab9}'</span>, WC_ALetter),
        (<span class="string">'\u{abc}'</span>, <span class="string">'\u{abc}'</span>, WC_Extend), (<span class="string">'\u{abd}'</span>, <span class="string">'\u{abd}'</span>, WC_ALetter), (<span class="string">'\u{abe}'</span>,
        <span class="string">'\u{ac5}'</span>, WC_Extend), (<span class="string">'\u{ac7}'</span>, <span class="string">'\u{ac9}'</span>, WC_Extend), (<span class="string">'\u{acb}'</span>, <span class="string">'\u{acd}'</span>, WC_Extend),
        (<span class="string">'\u{ad0}'</span>, <span class="string">'\u{ad0}'</span>, WC_ALetter), (<span class="string">'\u{ae0}'</span>, <span class="string">'\u{ae1}'</span>, WC_ALetter), (<span class="string">'\u{ae2}'</span>,
        <span class="string">'\u{ae3}'</span>, WC_Extend), (<span class="string">'\u{ae6}'</span>, <span class="string">'\u{aef}'</span>, WC_Numeric), (<span class="string">'\u{af9}'</span>, <span class="string">'\u{af9}'</span>,
        WC_ALetter), (<span class="string">'\u{afa}'</span>, <span class="string">'\u{aff}'</span>, WC_Extend), (<span class="string">'\u{b01}'</span>, <span class="string">'\u{b03}'</span>, WC_Extend),
        (<span class="string">'\u{b05}'</span>, <span class="string">'\u{b0c}'</span>, WC_ALetter), (<span class="string">'\u{b0f}'</span>, <span class="string">'\u{b10}'</span>, WC_ALetter), (<span class="string">'\u{b13}'</span>,
        <span class="string">'\u{b28}'</span>, WC_ALetter), (<span class="string">'\u{b2a}'</span>, <span class="string">'\u{b30}'</span>, WC_ALetter), (<span class="string">'\u{b32}'</span>, <span class="string">'\u{b33}'</span>,
        WC_ALetter), (<span class="string">'\u{b35}'</span>, <span class="string">'\u{b39}'</span>, WC_ALetter), (<span class="string">'\u{b3c}'</span>, <span class="string">'\u{b3c}'</span>, WC_Extend),
        (<span class="string">'\u{b3d}'</span>, <span class="string">'\u{b3d}'</span>, WC_ALetter), (<span class="string">'\u{b3e}'</span>, <span class="string">'\u{b44}'</span>, WC_Extend), (<span class="string">'\u{b47}'</span>,
        <span class="string">'\u{b48}'</span>, WC_Extend), (<span class="string">'\u{b4b}'</span>, <span class="string">'\u{b4d}'</span>, WC_Extend), (<span class="string">'\u{b55}'</span>, <span class="string">'\u{b57}'</span>, WC_Extend),
        (<span class="string">'\u{b5c}'</span>, <span class="string">'\u{b5d}'</span>, WC_ALetter), (<span class="string">'\u{b5f}'</span>, <span class="string">'\u{b61}'</span>, WC_ALetter), (<span class="string">'\u{b62}'</span>,
        <span class="string">'\u{b63}'</span>, WC_Extend), (<span class="string">'\u{b66}'</span>, <span class="string">'\u{b6f}'</span>, WC_Numeric), (<span class="string">'\u{b71}'</span>, <span class="string">'\u{b71}'</span>,
        WC_ALetter), (<span class="string">'\u{b82}'</span>, <span class="string">'\u{b82}'</span>, WC_Extend), (<span class="string">'\u{b83}'</span>, <span class="string">'\u{b83}'</span>, WC_ALetter),
        (<span class="string">'\u{b85}'</span>, <span class="string">'\u{b8a}'</span>, WC_ALetter), (<span class="string">'\u{b8e}'</span>, <span class="string">'\u{b90}'</span>, WC_ALetter), (<span class="string">'\u{b92}'</span>,
        <span class="string">'\u{b95}'</span>, WC_ALetter), (<span class="string">'\u{b99}'</span>, <span class="string">'\u{b9a}'</span>, WC_ALetter), (<span class="string">'\u{b9c}'</span>, <span class="string">'\u{b9c}'</span>,
        WC_ALetter), (<span class="string">'\u{b9e}'</span>, <span class="string">'\u{b9f}'</span>, WC_ALetter), (<span class="string">'\u{ba3}'</span>, <span class="string">'\u{ba4}'</span>, WC_ALetter),
        (<span class="string">'\u{ba8}'</span>, <span class="string">'\u{baa}'</span>, WC_ALetter), (<span class="string">'\u{bae}'</span>, <span class="string">'\u{bb9}'</span>, WC_ALetter), (<span class="string">'\u{bbe}'</span>,
        <span class="string">'\u{bc2}'</span>, WC_Extend), (<span class="string">'\u{bc6}'</span>, <span class="string">'\u{bc8}'</span>, WC_Extend), (<span class="string">'\u{bca}'</span>, <span class="string">'\u{bcd}'</span>, WC_Extend),
        (<span class="string">'\u{bd0}'</span>, <span class="string">'\u{bd0}'</span>, WC_ALetter), (<span class="string">'\u{bd7}'</span>, <span class="string">'\u{bd7}'</span>, WC_Extend), (<span class="string">'\u{be6}'</span>,
        <span class="string">'\u{bef}'</span>, WC_Numeric), (<span class="string">'\u{c00}'</span>, <span class="string">'\u{c04}'</span>, WC_Extend), (<span class="string">'\u{c05}'</span>, <span class="string">'\u{c0c}'</span>,
        WC_ALetter), (<span class="string">'\u{c0e}'</span>, <span class="string">'\u{c10}'</span>, WC_ALetter), (<span class="string">'\u{c12}'</span>, <span class="string">'\u{c28}'</span>, WC_ALetter),
        (<span class="string">'\u{c2a}'</span>, <span class="string">'\u{c39}'</span>, WC_ALetter), (<span class="string">'\u{c3c}'</span>, <span class="string">'\u{c3c}'</span>, WC_Extend), (<span class="string">'\u{c3d}'</span>,
        <span class="string">'\u{c3d}'</span>, WC_ALetter), (<span class="string">'\u{c3e}'</span>, <span class="string">'\u{c44}'</span>, WC_Extend), (<span class="string">'\u{c46}'</span>, <span class="string">'\u{c48}'</span>,
        WC_Extend), (<span class="string">'\u{c4a}'</span>, <span class="string">'\u{c4d}'</span>, WC_Extend), (<span class="string">'\u{c55}'</span>, <span class="string">'\u{c56}'</span>, WC_Extend),
        (<span class="string">'\u{c58}'</span>, <span class="string">'\u{c5a}'</span>, WC_ALetter), (<span class="string">'\u{c5d}'</span>, <span class="string">'\u{c5d}'</span>, WC_ALetter), (<span class="string">'\u{c60}'</span>,
        <span class="string">'\u{c61}'</span>, WC_ALetter), (<span class="string">'\u{c62}'</span>, <span class="string">'\u{c63}'</span>, WC_Extend), (<span class="string">'\u{c66}'</span>, <span class="string">'\u{c6f}'</span>,
        WC_Numeric), (<span class="string">'\u{c80}'</span>, <span class="string">'\u{c80}'</span>, WC_ALetter), (<span class="string">'\u{c81}'</span>, <span class="string">'\u{c83}'</span>, WC_Extend),
        (<span class="string">'\u{c85}'</span>, <span class="string">'\u{c8c}'</span>, WC_ALetter), (<span class="string">'\u{c8e}'</span>, <span class="string">'\u{c90}'</span>, WC_ALetter), (<span class="string">'\u{c92}'</span>,
        <span class="string">'\u{ca8}'</span>, WC_ALetter), (<span class="string">'\u{caa}'</span>, <span class="string">'\u{cb3}'</span>, WC_ALetter), (<span class="string">'\u{cb5}'</span>, <span class="string">'\u{cb9}'</span>,
        WC_ALetter), (<span class="string">'\u{cbc}'</span>, <span class="string">'\u{cbc}'</span>, WC_Extend), (<span class="string">'\u{cbd}'</span>, <span class="string">'\u{cbd}'</span>, WC_ALetter),
        (<span class="string">'\u{cbe}'</span>, <span class="string">'\u{cc4}'</span>, WC_Extend), (<span class="string">'\u{cc6}'</span>, <span class="string">'\u{cc8}'</span>, WC_Extend), (<span class="string">'\u{cca}'</span>, <span class="string">'\u{ccd}'</span>,
        WC_Extend), (<span class="string">'\u{cd5}'</span>, <span class="string">'\u{cd6}'</span>, WC_Extend), (<span class="string">'\u{cdd}'</span>, <span class="string">'\u{cde}'</span>, WC_ALetter),
        (<span class="string">'\u{ce0}'</span>, <span class="string">'\u{ce1}'</span>, WC_ALetter), (<span class="string">'\u{ce2}'</span>, <span class="string">'\u{ce3}'</span>, WC_Extend), (<span class="string">'\u{ce6}'</span>,
        <span class="string">'\u{cef}'</span>, WC_Numeric), (<span class="string">'\u{cf1}'</span>, <span class="string">'\u{cf2}'</span>, WC_ALetter), (<span class="string">'\u{cf3}'</span>, <span class="string">'\u{cf3}'</span>,
        WC_Extend), (<span class="string">'\u{d00}'</span>, <span class="string">'\u{d03}'</span>, WC_Extend), (<span class="string">'\u{d04}'</span>, <span class="string">'\u{d0c}'</span>, WC_ALetter),
        (<span class="string">'\u{d0e}'</span>, <span class="string">'\u{d10}'</span>, WC_ALetter), (<span class="string">'\u{d12}'</span>, <span class="string">'\u{d3a}'</span>, WC_ALetter), (<span class="string">'\u{d3b}'</span>,
        <span class="string">'\u{d3c}'</span>, WC_Extend), (<span class="string">'\u{d3d}'</span>, <span class="string">'\u{d3d}'</span>, WC_ALetter), (<span class="string">'\u{d3e}'</span>, <span class="string">'\u{d44}'</span>,
        WC_Extend), (<span class="string">'\u{d46}'</span>, <span class="string">'\u{d48}'</span>, WC_Extend), (<span class="string">'\u{d4a}'</span>, <span class="string">'\u{d4d}'</span>, WC_Extend),
        (<span class="string">'\u{d4e}'</span>, <span class="string">'\u{d4e}'</span>, WC_ALetter), (<span class="string">'\u{d54}'</span>, <span class="string">'\u{d56}'</span>, WC_ALetter), (<span class="string">'\u{d57}'</span>,
        <span class="string">'\u{d57}'</span>, WC_Extend), (<span class="string">'\u{d5f}'</span>, <span class="string">'\u{d61}'</span>, WC_ALetter), (<span class="string">'\u{d62}'</span>, <span class="string">'\u{d63}'</span>,
        WC_Extend), (<span class="string">'\u{d66}'</span>, <span class="string">'\u{d6f}'</span>, WC_Numeric), (<span class="string">'\u{d7a}'</span>, <span class="string">'\u{d7f}'</span>, WC_ALetter),
        (<span class="string">'\u{d81}'</span>, <span class="string">'\u{d83}'</span>, WC_Extend), (<span class="string">'\u{d85}'</span>, <span class="string">'\u{d96}'</span>, WC_ALetter), (<span class="string">'\u{d9a}'</span>,
        <span class="string">'\u{db1}'</span>, WC_ALetter), (<span class="string">'\u{db3}'</span>, <span class="string">'\u{dbb}'</span>, WC_ALetter), (<span class="string">'\u{dbd}'</span>, <span class="string">'\u{dbd}'</span>,
        WC_ALetter), (<span class="string">'\u{dc0}'</span>, <span class="string">'\u{dc6}'</span>, WC_ALetter), (<span class="string">'\u{dca}'</span>, <span class="string">'\u{dca}'</span>, WC_Extend),
        (<span class="string">'\u{dcf}'</span>, <span class="string">'\u{dd4}'</span>, WC_Extend), (<span class="string">'\u{dd6}'</span>, <span class="string">'\u{dd6}'</span>, WC_Extend), (<span class="string">'\u{dd8}'</span>, <span class="string">'\u{ddf}'</span>,
        WC_Extend), (<span class="string">'\u{de6}'</span>, <span class="string">'\u{def}'</span>, WC_Numeric), (<span class="string">'\u{df2}'</span>, <span class="string">'\u{df3}'</span>, WC_Extend),
        (<span class="string">'\u{e31}'</span>, <span class="string">'\u{e31}'</span>, WC_Extend), (<span class="string">'\u{e34}'</span>, <span class="string">'\u{e3a}'</span>, WC_Extend), (<span class="string">'\u{e47}'</span>, <span class="string">'\u{e4e}'</span>,
        WC_Extend), (<span class="string">'\u{e50}'</span>, <span class="string">'\u{e59}'</span>, WC_Numeric), (<span class="string">'\u{eb1}'</span>, <span class="string">'\u{eb1}'</span>, WC_Extend),
        (<span class="string">'\u{eb4}'</span>, <span class="string">'\u{ebc}'</span>, WC_Extend), (<span class="string">'\u{ec8}'</span>, <span class="string">'\u{ece}'</span>, WC_Extend), (<span class="string">'\u{ed0}'</span>, <span class="string">'\u{ed9}'</span>,
        WC_Numeric), (<span class="string">'\u{f00}'</span>, <span class="string">'\u{f00}'</span>, WC_ALetter), (<span class="string">'\u{f18}'</span>, <span class="string">'\u{f19}'</span>, WC_Extend),
        (<span class="string">'\u{f20}'</span>, <span class="string">'\u{f29}'</span>, WC_Numeric), (<span class="string">'\u{f35}'</span>, <span class="string">'\u{f35}'</span>, WC_Extend), (<span class="string">'\u{f37}'</span>,
        <span class="string">'\u{f37}'</span>, WC_Extend), (<span class="string">'\u{f39}'</span>, <span class="string">'\u{f39}'</span>, WC_Extend), (<span class="string">'\u{f3e}'</span>, <span class="string">'\u{f3f}'</span>, WC_Extend),
        (<span class="string">'\u{f40}'</span>, <span class="string">'\u{f47}'</span>, WC_ALetter), (<span class="string">'\u{f49}'</span>, <span class="string">'\u{f6c}'</span>, WC_ALetter), (<span class="string">'\u{f71}'</span>,
        <span class="string">'\u{f84}'</span>, WC_Extend), (<span class="string">'\u{f86}'</span>, <span class="string">'\u{f87}'</span>, WC_Extend), (<span class="string">'\u{f88}'</span>, <span class="string">'\u{f8c}'</span>,
        WC_ALetter), (<span class="string">'\u{f8d}'</span>, <span class="string">'\u{f97}'</span>, WC_Extend), (<span class="string">'\u{f99}'</span>, <span class="string">'\u{fbc}'</span>, WC_Extend),
        (<span class="string">'\u{fc6}'</span>, <span class="string">'\u{fc6}'</span>, WC_Extend), (<span class="string">'\u{102b}'</span>, <span class="string">'\u{103e}'</span>, WC_Extend), (<span class="string">'\u{1040}'</span>,
        <span class="string">'\u{1049}'</span>, WC_Numeric), (<span class="string">'\u{1056}'</span>, <span class="string">'\u{1059}'</span>, WC_Extend), (<span class="string">'\u{105e}'</span>, <span class="string">'\u{1060}'</span>,
        WC_Extend), (<span class="string">'\u{1062}'</span>, <span class="string">'\u{1064}'</span>, WC_Extend), (<span class="string">'\u{1067}'</span>, <span class="string">'\u{106d}'</span>, WC_Extend),
        (<span class="string">'\u{1071}'</span>, <span class="string">'\u{1074}'</span>, WC_Extend), (<span class="string">'\u{1082}'</span>, <span class="string">'\u{108d}'</span>, WC_Extend), (<span class="string">'\u{108f}'</span>,
        <span class="string">'\u{108f}'</span>, WC_Extend), (<span class="string">'\u{1090}'</span>, <span class="string">'\u{1099}'</span>, WC_Numeric), (<span class="string">'\u{109a}'</span>, <span class="string">'\u{109d}'</span>,
        WC_Extend), (<span class="string">'\u{10a0}'</span>, <span class="string">'\u{10c5}'</span>, WC_ALetter), (<span class="string">'\u{10c7}'</span>, <span class="string">'\u{10c7}'</span>, WC_ALetter),
        (<span class="string">'\u{10cd}'</span>, <span class="string">'\u{10cd}'</span>, WC_ALetter), (<span class="string">'\u{10d0}'</span>, <span class="string">'\u{10fa}'</span>, WC_ALetter), (<span class="string">'\u{10fc}'</span>,
        <span class="string">'\u{1248}'</span>, WC_ALetter), (<span class="string">'\u{124a}'</span>, <span class="string">'\u{124d}'</span>, WC_ALetter), (<span class="string">'\u{1250}'</span>, <span class="string">'\u{1256}'</span>,
        WC_ALetter), (<span class="string">'\u{1258}'</span>, <span class="string">'\u{1258}'</span>, WC_ALetter), (<span class="string">'\u{125a}'</span>, <span class="string">'\u{125d}'</span>, WC_ALetter),
        (<span class="string">'\u{1260}'</span>, <span class="string">'\u{1288}'</span>, WC_ALetter), (<span class="string">'\u{128a}'</span>, <span class="string">'\u{128d}'</span>, WC_ALetter), (<span class="string">'\u{1290}'</span>,
        <span class="string">'\u{12b0}'</span>, WC_ALetter), (<span class="string">'\u{12b2}'</span>, <span class="string">'\u{12b5}'</span>, WC_ALetter), (<span class="string">'\u{12b8}'</span>, <span class="string">'\u{12be}'</span>,
        WC_ALetter), (<span class="string">'\u{12c0}'</span>, <span class="string">'\u{12c0}'</span>, WC_ALetter), (<span class="string">'\u{12c2}'</span>, <span class="string">'\u{12c5}'</span>, WC_ALetter),
        (<span class="string">'\u{12c8}'</span>, <span class="string">'\u{12d6}'</span>, WC_ALetter), (<span class="string">'\u{12d8}'</span>, <span class="string">'\u{1310}'</span>, WC_ALetter), (<span class="string">'\u{1312}'</span>,
        <span class="string">'\u{1315}'</span>, WC_ALetter), (<span class="string">'\u{1318}'</span>, <span class="string">'\u{135a}'</span>, WC_ALetter), (<span class="string">'\u{135d}'</span>, <span class="string">'\u{135f}'</span>,
        WC_Extend), (<span class="string">'\u{1380}'</span>, <span class="string">'\u{138f}'</span>, WC_ALetter), (<span class="string">'\u{13a0}'</span>, <span class="string">'\u{13f5}'</span>, WC_ALetter),
        (<span class="string">'\u{13f8}'</span>, <span class="string">'\u{13fd}'</span>, WC_ALetter), (<span class="string">'\u{1401}'</span>, <span class="string">'\u{166c}'</span>, WC_ALetter), (<span class="string">'\u{166f}'</span>,
        <span class="string">'\u{167f}'</span>, WC_ALetter), (<span class="string">'\u{1680}'</span>, <span class="string">'\u{1680}'</span>, WC_WSegSpace), (<span class="string">'\u{1681}'</span>, <span class="string">'\u{169a}'</span>,
        WC_ALetter), (<span class="string">'\u{16a0}'</span>, <span class="string">'\u{16ea}'</span>, WC_ALetter), (<span class="string">'\u{16ee}'</span>, <span class="string">'\u{16f8}'</span>, WC_ALetter),
        (<span class="string">'\u{1700}'</span>, <span class="string">'\u{1711}'</span>, WC_ALetter), (<span class="string">'\u{1712}'</span>, <span class="string">'\u{1715}'</span>, WC_Extend), (<span class="string">'\u{171f}'</span>,
        <span class="string">'\u{1731}'</span>, WC_ALetter), (<span class="string">'\u{1732}'</span>, <span class="string">'\u{1734}'</span>, WC_Extend), (<span class="string">'\u{1740}'</span>, <span class="string">'\u{1751}'</span>,
        WC_ALetter), (<span class="string">'\u{1752}'</span>, <span class="string">'\u{1753}'</span>, WC_Extend), (<span class="string">'\u{1760}'</span>, <span class="string">'\u{176c}'</span>, WC_ALetter),
        (<span class="string">'\u{176e}'</span>, <span class="string">'\u{1770}'</span>, WC_ALetter), (<span class="string">'\u{1772}'</span>, <span class="string">'\u{1773}'</span>, WC_Extend), (<span class="string">'\u{17b4}'</span>,
        <span class="string">'\u{17d3}'</span>, WC_Extend), (<span class="string">'\u{17dd}'</span>, <span class="string">'\u{17dd}'</span>, WC_Extend), (<span class="string">'\u{17e0}'</span>, <span class="string">'\u{17e9}'</span>,
        WC_Numeric), (<span class="string">'\u{180b}'</span>, <span class="string">'\u{180d}'</span>, WC_Extend), (<span class="string">'\u{180e}'</span>, <span class="string">'\u{180e}'</span>, WC_Format),
        (<span class="string">'\u{180f}'</span>, <span class="string">'\u{180f}'</span>, WC_Extend), (<span class="string">'\u{1810}'</span>, <span class="string">'\u{1819}'</span>, WC_Numeric), (<span class="string">'\u{1820}'</span>,
        <span class="string">'\u{1878}'</span>, WC_ALetter), (<span class="string">'\u{1880}'</span>, <span class="string">'\u{1884}'</span>, WC_ALetter), (<span class="string">'\u{1885}'</span>, <span class="string">'\u{1886}'</span>,
        WC_Extend), (<span class="string">'\u{1887}'</span>, <span class="string">'\u{18a8}'</span>, WC_ALetter), (<span class="string">'\u{18a9}'</span>, <span class="string">'\u{18a9}'</span>, WC_Extend),
        (<span class="string">'\u{18aa}'</span>, <span class="string">'\u{18aa}'</span>, WC_ALetter), (<span class="string">'\u{18b0}'</span>, <span class="string">'\u{18f5}'</span>, WC_ALetter), (<span class="string">'\u{1900}'</span>,
        <span class="string">'\u{191e}'</span>, WC_ALetter), (<span class="string">'\u{1920}'</span>, <span class="string">'\u{192b}'</span>, WC_Extend), (<span class="string">'\u{1930}'</span>, <span class="string">'\u{193b}'</span>,
        WC_Extend), (<span class="string">'\u{1946}'</span>, <span class="string">'\u{194f}'</span>, WC_Numeric), (<span class="string">'\u{19d0}'</span>, <span class="string">'\u{19d9}'</span>, WC_Numeric),
        (<span class="string">'\u{1a00}'</span>, <span class="string">'\u{1a16}'</span>, WC_ALetter), (<span class="string">'\u{1a17}'</span>, <span class="string">'\u{1a1b}'</span>, WC_Extend), (<span class="string">'\u{1a55}'</span>,
        <span class="string">'\u{1a5e}'</span>, WC_Extend), (<span class="string">'\u{1a60}'</span>, <span class="string">'\u{1a7c}'</span>, WC_Extend), (<span class="string">'\u{1a7f}'</span>, <span class="string">'\u{1a7f}'</span>,
        WC_Extend), (<span class="string">'\u{1a80}'</span>, <span class="string">'\u{1a89}'</span>, WC_Numeric), (<span class="string">'\u{1a90}'</span>, <span class="string">'\u{1a99}'</span>, WC_Numeric),
        (<span class="string">'\u{1ab0}'</span>, <span class="string">'\u{1ace}'</span>, WC_Extend), (<span class="string">'\u{1b00}'</span>, <span class="string">'\u{1b04}'</span>, WC_Extend), (<span class="string">'\u{1b05}'</span>,
        <span class="string">'\u{1b33}'</span>, WC_ALetter), (<span class="string">'\u{1b34}'</span>, <span class="string">'\u{1b44}'</span>, WC_Extend), (<span class="string">'\u{1b45}'</span>, <span class="string">'\u{1b4c}'</span>,
        WC_ALetter), (<span class="string">'\u{1b50}'</span>, <span class="string">'\u{1b59}'</span>, WC_Numeric), (<span class="string">'\u{1b6b}'</span>, <span class="string">'\u{1b73}'</span>, WC_Extend),
        (<span class="string">'\u{1b80}'</span>, <span class="string">'\u{1b82}'</span>, WC_Extend), (<span class="string">'\u{1b83}'</span>, <span class="string">'\u{1ba0}'</span>, WC_ALetter), (<span class="string">'\u{1ba1}'</span>,
        <span class="string">'\u{1bad}'</span>, WC_Extend), (<span class="string">'\u{1bae}'</span>, <span class="string">'\u{1baf}'</span>, WC_ALetter), (<span class="string">'\u{1bb0}'</span>, <span class="string">'\u{1bb9}'</span>,
        WC_Numeric), (<span class="string">'\u{1bba}'</span>, <span class="string">'\u{1be5}'</span>, WC_ALetter), (<span class="string">'\u{1be6}'</span>, <span class="string">'\u{1bf3}'</span>, WC_Extend),
        (<span class="string">'\u{1c00}'</span>, <span class="string">'\u{1c23}'</span>, WC_ALetter), (<span class="string">'\u{1c24}'</span>, <span class="string">'\u{1c37}'</span>, WC_Extend), (<span class="string">'\u{1c40}'</span>,
        <span class="string">'\u{1c49}'</span>, WC_Numeric), (<span class="string">'\u{1c4d}'</span>, <span class="string">'\u{1c4f}'</span>, WC_ALetter), (<span class="string">'\u{1c50}'</span>, <span class="string">'\u{1c59}'</span>,
        WC_Numeric), (<span class="string">'\u{1c5a}'</span>, <span class="string">'\u{1c7d}'</span>, WC_ALetter), (<span class="string">'\u{1c80}'</span>, <span class="string">'\u{1c88}'</span>, WC_ALetter),
        (<span class="string">'\u{1c90}'</span>, <span class="string">'\u{1cba}'</span>, WC_ALetter), (<span class="string">'\u{1cbd}'</span>, <span class="string">'\u{1cbf}'</span>, WC_ALetter), (<span class="string">'\u{1cd0}'</span>,
        <span class="string">'\u{1cd2}'</span>, WC_Extend), (<span class="string">'\u{1cd4}'</span>, <span class="string">'\u{1ce8}'</span>, WC_Extend), (<span class="string">'\u{1ce9}'</span>, <span class="string">'\u{1cec}'</span>,
        WC_ALetter), (<span class="string">'\u{1ced}'</span>, <span class="string">'\u{1ced}'</span>, WC_Extend), (<span class="string">'\u{1cee}'</span>, <span class="string">'\u{1cf3}'</span>, WC_ALetter),
        (<span class="string">'\u{1cf4}'</span>, <span class="string">'\u{1cf4}'</span>, WC_Extend), (<span class="string">'\u{1cf5}'</span>, <span class="string">'\u{1cf6}'</span>, WC_ALetter), (<span class="string">'\u{1cf7}'</span>,
        <span class="string">'\u{1cf9}'</span>, WC_Extend), (<span class="string">'\u{1cfa}'</span>, <span class="string">'\u{1cfa}'</span>, WC_ALetter), (<span class="string">'\u{1d00}'</span>, <span class="string">'\u{1dbf}'</span>,
        WC_ALetter), (<span class="string">'\u{1dc0}'</span>, <span class="string">'\u{1dff}'</span>, WC_Extend), (<span class="string">'\u{1e00}'</span>, <span class="string">'\u{1f15}'</span>, WC_ALetter),
        (<span class="string">'\u{1f18}'</span>, <span class="string">'\u{1f1d}'</span>, WC_ALetter), (<span class="string">'\u{1f20}'</span>, <span class="string">'\u{1f45}'</span>, WC_ALetter), (<span class="string">'\u{1f48}'</span>,
        <span class="string">'\u{1f4d}'</span>, WC_ALetter), (<span class="string">'\u{1f50}'</span>, <span class="string">'\u{1f57}'</span>, WC_ALetter), (<span class="string">'\u{1f59}'</span>, <span class="string">'\u{1f59}'</span>,
        WC_ALetter), (<span class="string">'\u{1f5b}'</span>, <span class="string">'\u{1f5b}'</span>, WC_ALetter), (<span class="string">'\u{1f5d}'</span>, <span class="string">'\u{1f5d}'</span>, WC_ALetter),
        (<span class="string">'\u{1f5f}'</span>, <span class="string">'\u{1f7d}'</span>, WC_ALetter), (<span class="string">'\u{1f80}'</span>, <span class="string">'\u{1fb4}'</span>, WC_ALetter), (<span class="string">'\u{1fb6}'</span>,
        <span class="string">'\u{1fbc}'</span>, WC_ALetter), (<span class="string">'\u{1fbe}'</span>, <span class="string">'\u{1fbe}'</span>, WC_ALetter), (<span class="string">'\u{1fc2}'</span>, <span class="string">'\u{1fc4}'</span>,
        WC_ALetter), (<span class="string">'\u{1fc6}'</span>, <span class="string">'\u{1fcc}'</span>, WC_ALetter), (<span class="string">'\u{1fd0}'</span>, <span class="string">'\u{1fd3}'</span>, WC_ALetter),
        (<span class="string">'\u{1fd6}'</span>, <span class="string">'\u{1fdb}'</span>, WC_ALetter), (<span class="string">'\u{1fe0}'</span>, <span class="string">'\u{1fec}'</span>, WC_ALetter), (<span class="string">'\u{1ff2}'</span>,
        <span class="string">'\u{1ff4}'</span>, WC_ALetter), (<span class="string">'\u{1ff6}'</span>, <span class="string">'\u{1ffc}'</span>, WC_ALetter), (<span class="string">'\u{2000}'</span>, <span class="string">'\u{2006}'</span>,
        WC_WSegSpace), (<span class="string">'\u{2008}'</span>, <span class="string">'\u{200a}'</span>, WC_WSegSpace), (<span class="string">'\u{200c}'</span>, <span class="string">'\u{200c}'</span>, WC_Extend),
        (<span class="string">'\u{200d}'</span>, <span class="string">'\u{200d}'</span>, WC_ZWJ), (<span class="string">'\u{200e}'</span>, <span class="string">'\u{200f}'</span>, WC_Format), (<span class="string">'\u{2018}'</span>,
        <span class="string">'\u{2019}'</span>, WC_MidNumLet), (<span class="string">'\u{2024}'</span>, <span class="string">'\u{2024}'</span>, WC_MidNumLet), (<span class="string">'\u{2027}'</span>, <span class="string">'\u{2027}'</span>,
        WC_MidLetter), (<span class="string">'\u{2028}'</span>, <span class="string">'\u{2029}'</span>, WC_Newline), (<span class="string">'\u{202a}'</span>, <span class="string">'\u{202e}'</span>, WC_Format),
        (<span class="string">'\u{202f}'</span>, <span class="string">'\u{202f}'</span>, WC_ExtendNumLet), (<span class="string">'\u{203f}'</span>, <span class="string">'\u{2040}'</span>, WC_ExtendNumLet),
        (<span class="string">'\u{2044}'</span>, <span class="string">'\u{2044}'</span>, WC_MidNum), (<span class="string">'\u{2054}'</span>, <span class="string">'\u{2054}'</span>, WC_ExtendNumLet), (<span class="string">'\u{205f}'</span>,
        <span class="string">'\u{205f}'</span>, WC_WSegSpace), (<span class="string">'\u{2060}'</span>, <span class="string">'\u{2064}'</span>, WC_Format), (<span class="string">'\u{2066}'</span>, <span class="string">'\u{206f}'</span>,
        WC_Format), (<span class="string">'\u{2071}'</span>, <span class="string">'\u{2071}'</span>, WC_ALetter), (<span class="string">'\u{207f}'</span>, <span class="string">'\u{207f}'</span>, WC_ALetter),
        (<span class="string">'\u{2090}'</span>, <span class="string">'\u{209c}'</span>, WC_ALetter), (<span class="string">'\u{20d0}'</span>, <span class="string">'\u{20f0}'</span>, WC_Extend), (<span class="string">'\u{2102}'</span>,
        <span class="string">'\u{2102}'</span>, WC_ALetter), (<span class="string">'\u{2107}'</span>, <span class="string">'\u{2107}'</span>, WC_ALetter), (<span class="string">'\u{210a}'</span>, <span class="string">'\u{2113}'</span>,
        WC_ALetter), (<span class="string">'\u{2115}'</span>, <span class="string">'\u{2115}'</span>, WC_ALetter), (<span class="string">'\u{2119}'</span>, <span class="string">'\u{211d}'</span>, WC_ALetter),
        (<span class="string">'\u{2124}'</span>, <span class="string">'\u{2124}'</span>, WC_ALetter), (<span class="string">'\u{2126}'</span>, <span class="string">'\u{2126}'</span>, WC_ALetter), (<span class="string">'\u{2128}'</span>,
        <span class="string">'\u{2128}'</span>, WC_ALetter), (<span class="string">'\u{212a}'</span>, <span class="string">'\u{212d}'</span>, WC_ALetter), (<span class="string">'\u{212f}'</span>, <span class="string">'\u{2139}'</span>,
        WC_ALetter), (<span class="string">'\u{213c}'</span>, <span class="string">'\u{213f}'</span>, WC_ALetter), (<span class="string">'\u{2145}'</span>, <span class="string">'\u{2149}'</span>, WC_ALetter),
        (<span class="string">'\u{214e}'</span>, <span class="string">'\u{214e}'</span>, WC_ALetter), (<span class="string">'\u{2160}'</span>, <span class="string">'\u{2188}'</span>, WC_ALetter), (<span class="string">'\u{24b6}'</span>,
        <span class="string">'\u{24e9}'</span>, WC_ALetter), (<span class="string">'\u{2c00}'</span>, <span class="string">'\u{2ce4}'</span>, WC_ALetter), (<span class="string">'\u{2ceb}'</span>, <span class="string">'\u{2cee}'</span>,
        WC_ALetter), (<span class="string">'\u{2cef}'</span>, <span class="string">'\u{2cf1}'</span>, WC_Extend), (<span class="string">'\u{2cf2}'</span>, <span class="string">'\u{2cf3}'</span>, WC_ALetter),
        (<span class="string">'\u{2d00}'</span>, <span class="string">'\u{2d25}'</span>, WC_ALetter), (<span class="string">'\u{2d27}'</span>, <span class="string">'\u{2d27}'</span>, WC_ALetter), (<span class="string">'\u{2d2d}'</span>,
        <span class="string">'\u{2d2d}'</span>, WC_ALetter), (<span class="string">'\u{2d30}'</span>, <span class="string">'\u{2d67}'</span>, WC_ALetter), (<span class="string">'\u{2d6f}'</span>, <span class="string">'\u{2d6f}'</span>,
        WC_ALetter), (<span class="string">'\u{2d7f}'</span>, <span class="string">'\u{2d7f}'</span>, WC_Extend), (<span class="string">'\u{2d80}'</span>, <span class="string">'\u{2d96}'</span>, WC_ALetter),
        (<span class="string">'\u{2da0}'</span>, <span class="string">'\u{2da6}'</span>, WC_ALetter), (<span class="string">'\u{2da8}'</span>, <span class="string">'\u{2dae}'</span>, WC_ALetter), (<span class="string">'\u{2db0}'</span>,
        <span class="string">'\u{2db6}'</span>, WC_ALetter), (<span class="string">'\u{2db8}'</span>, <span class="string">'\u{2dbe}'</span>, WC_ALetter), (<span class="string">'\u{2dc0}'</span>, <span class="string">'\u{2dc6}'</span>,
        WC_ALetter), (<span class="string">'\u{2dc8}'</span>, <span class="string">'\u{2dce}'</span>, WC_ALetter), (<span class="string">'\u{2dd0}'</span>, <span class="string">'\u{2dd6}'</span>, WC_ALetter),
        (<span class="string">'\u{2dd8}'</span>, <span class="string">'\u{2dde}'</span>, WC_ALetter), (<span class="string">'\u{2de0}'</span>, <span class="string">'\u{2dff}'</span>, WC_Extend), (<span class="string">'\u{2e2f}'</span>,
        <span class="string">'\u{2e2f}'</span>, WC_ALetter), (<span class="string">'\u{3000}'</span>, <span class="string">'\u{3000}'</span>, WC_WSegSpace), (<span class="string">'\u{3005}'</span>, <span class="string">'\u{3005}'</span>,
        WC_ALetter), (<span class="string">'\u{302a}'</span>, <span class="string">'\u{302f}'</span>, WC_Extend), (<span class="string">'\u{3031}'</span>, <span class="string">'\u{3035}'</span>, WC_Katakana),
        (<span class="string">'\u{303b}'</span>, <span class="string">'\u{303c}'</span>, WC_ALetter), (<span class="string">'\u{3099}'</span>, <span class="string">'\u{309a}'</span>, WC_Extend), (<span class="string">'\u{309b}'</span>,
        <span class="string">'\u{309c}'</span>, WC_Katakana), (<span class="string">'\u{30a0}'</span>, <span class="string">'\u{30fa}'</span>, WC_Katakana), (<span class="string">'\u{30fc}'</span>, <span class="string">'\u{30ff}'</span>,
        WC_Katakana), (<span class="string">'\u{3105}'</span>, <span class="string">'\u{312f}'</span>, WC_ALetter), (<span class="string">'\u{3131}'</span>, <span class="string">'\u{318e}'</span>, WC_ALetter),
        (<span class="string">'\u{31a0}'</span>, <span class="string">'\u{31bf}'</span>, WC_ALetter), (<span class="string">'\u{31f0}'</span>, <span class="string">'\u{31ff}'</span>, WC_Katakana), (<span class="string">'\u{32d0}'</span>,
        <span class="string">'\u{32fe}'</span>, WC_Katakana), (<span class="string">'\u{3300}'</span>, <span class="string">'\u{3357}'</span>, WC_Katakana), (<span class="string">'\u{a000}'</span>, <span class="string">'\u{a48c}'</span>,
        WC_ALetter), (<span class="string">'\u{a4d0}'</span>, <span class="string">'\u{a4fd}'</span>, WC_ALetter), (<span class="string">'\u{a500}'</span>, <span class="string">'\u{a60c}'</span>, WC_ALetter),
        (<span class="string">'\u{a610}'</span>, <span class="string">'\u{a61f}'</span>, WC_ALetter), (<span class="string">'\u{a620}'</span>, <span class="string">'\u{a629}'</span>, WC_Numeric), (<span class="string">'\u{a62a}'</span>,
        <span class="string">'\u{a62b}'</span>, WC_ALetter), (<span class="string">'\u{a640}'</span>, <span class="string">'\u{a66e}'</span>, WC_ALetter), (<span class="string">'\u{a66f}'</span>, <span class="string">'\u{a672}'</span>,
        WC_Extend), (<span class="string">'\u{a674}'</span>, <span class="string">'\u{a67d}'</span>, WC_Extend), (<span class="string">'\u{a67f}'</span>, <span class="string">'\u{a69d}'</span>, WC_ALetter),
        (<span class="string">'\u{a69e}'</span>, <span class="string">'\u{a69f}'</span>, WC_Extend), (<span class="string">'\u{a6a0}'</span>, <span class="string">'\u{a6ef}'</span>, WC_ALetter), (<span class="string">'\u{a6f0}'</span>,
        <span class="string">'\u{a6f1}'</span>, WC_Extend), (<span class="string">'\u{a708}'</span>, <span class="string">'\u{a7ca}'</span>, WC_ALetter), (<span class="string">'\u{a7d0}'</span>, <span class="string">'\u{a7d1}'</span>,
        WC_ALetter), (<span class="string">'\u{a7d3}'</span>, <span class="string">'\u{a7d3}'</span>, WC_ALetter), (<span class="string">'\u{a7d5}'</span>, <span class="string">'\u{a7d9}'</span>, WC_ALetter),
        (<span class="string">'\u{a7f2}'</span>, <span class="string">'\u{a801}'</span>, WC_ALetter), (<span class="string">'\u{a802}'</span>, <span class="string">'\u{a802}'</span>, WC_Extend), (<span class="string">'\u{a803}'</span>,
        <span class="string">'\u{a805}'</span>, WC_ALetter), (<span class="string">'\u{a806}'</span>, <span class="string">'\u{a806}'</span>, WC_Extend), (<span class="string">'\u{a807}'</span>, <span class="string">'\u{a80a}'</span>,
        WC_ALetter), (<span class="string">'\u{a80b}'</span>, <span class="string">'\u{a80b}'</span>, WC_Extend), (<span class="string">'\u{a80c}'</span>, <span class="string">'\u{a822}'</span>, WC_ALetter),
        (<span class="string">'\u{a823}'</span>, <span class="string">'\u{a827}'</span>, WC_Extend), (<span class="string">'\u{a82c}'</span>, <span class="string">'\u{a82c}'</span>, WC_Extend), (<span class="string">'\u{a840}'</span>,
        <span class="string">'\u{a873}'</span>, WC_ALetter), (<span class="string">'\u{a880}'</span>, <span class="string">'\u{a881}'</span>, WC_Extend), (<span class="string">'\u{a882}'</span>, <span class="string">'\u{a8b3}'</span>,
        WC_ALetter), (<span class="string">'\u{a8b4}'</span>, <span class="string">'\u{a8c5}'</span>, WC_Extend), (<span class="string">'\u{a8d0}'</span>, <span class="string">'\u{a8d9}'</span>, WC_Numeric),
        (<span class="string">'\u{a8e0}'</span>, <span class="string">'\u{a8f1}'</span>, WC_Extend), (<span class="string">'\u{a8f2}'</span>, <span class="string">'\u{a8f7}'</span>, WC_ALetter), (<span class="string">'\u{a8fb}'</span>,
        <span class="string">'\u{a8fb}'</span>, WC_ALetter), (<span class="string">'\u{a8fd}'</span>, <span class="string">'\u{a8fe}'</span>, WC_ALetter), (<span class="string">'\u{a8ff}'</span>, <span class="string">'\u{a8ff}'</span>,
        WC_Extend), (<span class="string">'\u{a900}'</span>, <span class="string">'\u{a909}'</span>, WC_Numeric), (<span class="string">'\u{a90a}'</span>, <span class="string">'\u{a925}'</span>, WC_ALetter),
        (<span class="string">'\u{a926}'</span>, <span class="string">'\u{a92d}'</span>, WC_Extend), (<span class="string">'\u{a930}'</span>, <span class="string">'\u{a946}'</span>, WC_ALetter), (<span class="string">'\u{a947}'</span>,
        <span class="string">'\u{a953}'</span>, WC_Extend), (<span class="string">'\u{a960}'</span>, <span class="string">'\u{a97c}'</span>, WC_ALetter), (<span class="string">'\u{a980}'</span>, <span class="string">'\u{a983}'</span>,
        WC_Extend), (<span class="string">'\u{a984}'</span>, <span class="string">'\u{a9b2}'</span>, WC_ALetter), (<span class="string">'\u{a9b3}'</span>, <span class="string">'\u{a9c0}'</span>, WC_Extend),
        (<span class="string">'\u{a9cf}'</span>, <span class="string">'\u{a9cf}'</span>, WC_ALetter), (<span class="string">'\u{a9d0}'</span>, <span class="string">'\u{a9d9}'</span>, WC_Numeric), (<span class="string">'\u{a9e5}'</span>,
        <span class="string">'\u{a9e5}'</span>, WC_Extend), (<span class="string">'\u{a9f0}'</span>, <span class="string">'\u{a9f9}'</span>, WC_Numeric), (<span class="string">'\u{aa00}'</span>, <span class="string">'\u{aa28}'</span>,
        WC_ALetter), (<span class="string">'\u{aa29}'</span>, <span class="string">'\u{aa36}'</span>, WC_Extend), (<span class="string">'\u{aa40}'</span>, <span class="string">'\u{aa42}'</span>, WC_ALetter),
        (<span class="string">'\u{aa43}'</span>, <span class="string">'\u{aa43}'</span>, WC_Extend), (<span class="string">'\u{aa44}'</span>, <span class="string">'\u{aa4b}'</span>, WC_ALetter), (<span class="string">'\u{aa4c}'</span>,
        <span class="string">'\u{aa4d}'</span>, WC_Extend), (<span class="string">'\u{aa50}'</span>, <span class="string">'\u{aa59}'</span>, WC_Numeric), (<span class="string">'\u{aa7b}'</span>, <span class="string">'\u{aa7d}'</span>,
        WC_Extend), (<span class="string">'\u{aab0}'</span>, <span class="string">'\u{aab0}'</span>, WC_Extend), (<span class="string">'\u{aab2}'</span>, <span class="string">'\u{aab4}'</span>, WC_Extend),
        (<span class="string">'\u{aab7}'</span>, <span class="string">'\u{aab8}'</span>, WC_Extend), (<span class="string">'\u{aabe}'</span>, <span class="string">'\u{aabf}'</span>, WC_Extend), (<span class="string">'\u{aac1}'</span>,
        <span class="string">'\u{aac1}'</span>, WC_Extend), (<span class="string">'\u{aae0}'</span>, <span class="string">'\u{aaea}'</span>, WC_ALetter), (<span class="string">'\u{aaeb}'</span>, <span class="string">'\u{aaef}'</span>,
        WC_Extend), (<span class="string">'\u{aaf2}'</span>, <span class="string">'\u{aaf4}'</span>, WC_ALetter), (<span class="string">'\u{aaf5}'</span>, <span class="string">'\u{aaf6}'</span>, WC_Extend),
        (<span class="string">'\u{ab01}'</span>, <span class="string">'\u{ab06}'</span>, WC_ALetter), (<span class="string">'\u{ab09}'</span>, <span class="string">'\u{ab0e}'</span>, WC_ALetter), (<span class="string">'\u{ab11}'</span>,
        <span class="string">'\u{ab16}'</span>, WC_ALetter), (<span class="string">'\u{ab20}'</span>, <span class="string">'\u{ab26}'</span>, WC_ALetter), (<span class="string">'\u{ab28}'</span>, <span class="string">'\u{ab2e}'</span>,
        WC_ALetter), (<span class="string">'\u{ab30}'</span>, <span class="string">'\u{ab69}'</span>, WC_ALetter), (<span class="string">'\u{ab70}'</span>, <span class="string">'\u{abe2}'</span>, WC_ALetter),
        (<span class="string">'\u{abe3}'</span>, <span class="string">'\u{abea}'</span>, WC_Extend), (<span class="string">'\u{abec}'</span>, <span class="string">'\u{abed}'</span>, WC_Extend), (<span class="string">'\u{abf0}'</span>,
        <span class="string">'\u{abf9}'</span>, WC_Numeric), (<span class="string">'\u{ac00}'</span>, <span class="string">'\u{d7a3}'</span>, WC_ALetter), (<span class="string">'\u{d7b0}'</span>, <span class="string">'\u{d7c6}'</span>,
        WC_ALetter), (<span class="string">'\u{d7cb}'</span>, <span class="string">'\u{d7fb}'</span>, WC_ALetter), (<span class="string">'\u{fb00}'</span>, <span class="string">'\u{fb06}'</span>, WC_ALetter),
        (<span class="string">'\u{fb13}'</span>, <span class="string">'\u{fb17}'</span>, WC_ALetter), (<span class="string">'\u{fb1d}'</span>, <span class="string">'\u{fb1d}'</span>, WC_Hebrew_Letter),
        (<span class="string">'\u{fb1e}'</span>, <span class="string">'\u{fb1e}'</span>, WC_Extend), (<span class="string">'\u{fb1f}'</span>, <span class="string">'\u{fb28}'</span>, WC_Hebrew_Letter),
        (<span class="string">'\u{fb2a}'</span>, <span class="string">'\u{fb36}'</span>, WC_Hebrew_Letter), (<span class="string">'\u{fb38}'</span>, <span class="string">'\u{fb3c}'</span>, WC_Hebrew_Letter),
        (<span class="string">'\u{fb3e}'</span>, <span class="string">'\u{fb3e}'</span>, WC_Hebrew_Letter), (<span class="string">'\u{fb40}'</span>, <span class="string">'\u{fb41}'</span>, WC_Hebrew_Letter),
        (<span class="string">'\u{fb43}'</span>, <span class="string">'\u{fb44}'</span>, WC_Hebrew_Letter), (<span class="string">'\u{fb46}'</span>, <span class="string">'\u{fb4f}'</span>, WC_Hebrew_Letter),
        (<span class="string">'\u{fb50}'</span>, <span class="string">'\u{fbb1}'</span>, WC_ALetter), (<span class="string">'\u{fbd3}'</span>, <span class="string">'\u{fd3d}'</span>, WC_ALetter), (<span class="string">'\u{fd50}'</span>,
        <span class="string">'\u{fd8f}'</span>, WC_ALetter), (<span class="string">'\u{fd92}'</span>, <span class="string">'\u{fdc7}'</span>, WC_ALetter), (<span class="string">'\u{fdf0}'</span>, <span class="string">'\u{fdfb}'</span>,
        WC_ALetter), (<span class="string">'\u{fe00}'</span>, <span class="string">'\u{fe0f}'</span>, WC_Extend), (<span class="string">'\u{fe10}'</span>, <span class="string">'\u{fe10}'</span>, WC_MidNum),
        (<span class="string">'\u{fe13}'</span>, <span class="string">'\u{fe13}'</span>, WC_MidLetter), (<span class="string">'\u{fe14}'</span>, <span class="string">'\u{fe14}'</span>, WC_MidNum), (<span class="string">'\u{fe20}'</span>,
        <span class="string">'\u{fe2f}'</span>, WC_Extend), (<span class="string">'\u{fe33}'</span>, <span class="string">'\u{fe34}'</span>, WC_ExtendNumLet), (<span class="string">'\u{fe4d}'</span>, <span class="string">'\u{fe4f}'</span>,
        WC_ExtendNumLet), (<span class="string">'\u{fe50}'</span>, <span class="string">'\u{fe50}'</span>, WC_MidNum), (<span class="string">'\u{fe52}'</span>, <span class="string">'\u{fe52}'</span>,
        WC_MidNumLet), (<span class="string">'\u{fe54}'</span>, <span class="string">'\u{fe54}'</span>, WC_MidNum), (<span class="string">'\u{fe55}'</span>, <span class="string">'\u{fe55}'</span>, WC_MidLetter),
        (<span class="string">'\u{fe70}'</span>, <span class="string">'\u{fe74}'</span>, WC_ALetter), (<span class="string">'\u{fe76}'</span>, <span class="string">'\u{fefc}'</span>, WC_ALetter), (<span class="string">'\u{feff}'</span>,
        <span class="string">'\u{feff}'</span>, WC_Format), (<span class="string">'\u{ff07}'</span>, <span class="string">'\u{ff07}'</span>, WC_MidNumLet), (<span class="string">'\u{ff0c}'</span>, <span class="string">'\u{ff0c}'</span>,
        WC_MidNum), (<span class="string">'\u{ff0e}'</span>, <span class="string">'\u{ff0e}'</span>, WC_MidNumLet), (<span class="string">'\u{ff10}'</span>, <span class="string">'\u{ff19}'</span>, WC_Numeric),
        (<span class="string">'\u{ff1a}'</span>, <span class="string">'\u{ff1a}'</span>, WC_MidLetter), (<span class="string">'\u{ff1b}'</span>, <span class="string">'\u{ff1b}'</span>, WC_MidNum), (<span class="string">'\u{ff21}'</span>,
        <span class="string">'\u{ff3a}'</span>, WC_ALetter), (<span class="string">'\u{ff3f}'</span>, <span class="string">'\u{ff3f}'</span>, WC_ExtendNumLet), (<span class="string">'\u{ff41}'</span>, <span class="string">'\u{ff5a}'</span>,
        WC_ALetter), (<span class="string">'\u{ff66}'</span>, <span class="string">'\u{ff9d}'</span>, WC_Katakana), (<span class="string">'\u{ff9e}'</span>, <span class="string">'\u{ff9f}'</span>, WC_Extend),
        (<span class="string">'\u{ffa0}'</span>, <span class="string">'\u{ffbe}'</span>, WC_ALetter), (<span class="string">'\u{ffc2}'</span>, <span class="string">'\u{ffc7}'</span>, WC_ALetter), (<span class="string">'\u{ffca}'</span>,
        <span class="string">'\u{ffcf}'</span>, WC_ALetter), (<span class="string">'\u{ffd2}'</span>, <span class="string">'\u{ffd7}'</span>, WC_ALetter), (<span class="string">'\u{ffda}'</span>, <span class="string">'\u{ffdc}'</span>,
        WC_ALetter), (<span class="string">'\u{fff9}'</span>, <span class="string">'\u{fffb}'</span>, WC_Format), (<span class="string">'\u{10000}'</span>, <span class="string">'\u{1000b}'</span>, WC_ALetter),
        (<span class="string">'\u{1000d}'</span>, <span class="string">'\u{10026}'</span>, WC_ALetter), (<span class="string">'\u{10028}'</span>, <span class="string">'\u{1003a}'</span>, WC_ALetter),
        (<span class="string">'\u{1003c}'</span>, <span class="string">'\u{1003d}'</span>, WC_ALetter), (<span class="string">'\u{1003f}'</span>, <span class="string">'\u{1004d}'</span>, WC_ALetter),
        (<span class="string">'\u{10050}'</span>, <span class="string">'\u{1005d}'</span>, WC_ALetter), (<span class="string">'\u{10080}'</span>, <span class="string">'\u{100fa}'</span>, WC_ALetter),
        (<span class="string">'\u{10140}'</span>, <span class="string">'\u{10174}'</span>, WC_ALetter), (<span class="string">'\u{101fd}'</span>, <span class="string">'\u{101fd}'</span>, WC_Extend), (<span class="string">'\u{10280}'</span>,
        <span class="string">'\u{1029c}'</span>, WC_ALetter), (<span class="string">'\u{102a0}'</span>, <span class="string">'\u{102d0}'</span>, WC_ALetter), (<span class="string">'\u{102e0}'</span>, <span class="string">'\u{102e0}'</span>,
        WC_Extend), (<span class="string">'\u{10300}'</span>, <span class="string">'\u{1031f}'</span>, WC_ALetter), (<span class="string">'\u{1032d}'</span>, <span class="string">'\u{1034a}'</span>, WC_ALetter),
        (<span class="string">'\u{10350}'</span>, <span class="string">'\u{10375}'</span>, WC_ALetter), (<span class="string">'\u{10376}'</span>, <span class="string">'\u{1037a}'</span>, WC_Extend), (<span class="string">'\u{10380}'</span>,
        <span class="string">'\u{1039d}'</span>, WC_ALetter), (<span class="string">'\u{103a0}'</span>, <span class="string">'\u{103c3}'</span>, WC_ALetter), (<span class="string">'\u{103c8}'</span>, <span class="string">'\u{103cf}'</span>,
        WC_ALetter), (<span class="string">'\u{103d1}'</span>, <span class="string">'\u{103d5}'</span>, WC_ALetter), (<span class="string">'\u{10400}'</span>, <span class="string">'\u{1049d}'</span>, WC_ALetter),
        (<span class="string">'\u{104a0}'</span>, <span class="string">'\u{104a9}'</span>, WC_Numeric), (<span class="string">'\u{104b0}'</span>, <span class="string">'\u{104d3}'</span>, WC_ALetter),
        (<span class="string">'\u{104d8}'</span>, <span class="string">'\u{104fb}'</span>, WC_ALetter), (<span class="string">'\u{10500}'</span>, <span class="string">'\u{10527}'</span>, WC_ALetter),
        (<span class="string">'\u{10530}'</span>, <span class="string">'\u{10563}'</span>, WC_ALetter), (<span class="string">'\u{10570}'</span>, <span class="string">'\u{1057a}'</span>, WC_ALetter),
        (<span class="string">'\u{1057c}'</span>, <span class="string">'\u{1058a}'</span>, WC_ALetter), (<span class="string">'\u{1058c}'</span>, <span class="string">'\u{10592}'</span>, WC_ALetter),
        (<span class="string">'\u{10594}'</span>, <span class="string">'\u{10595}'</span>, WC_ALetter), (<span class="string">'\u{10597}'</span>, <span class="string">'\u{105a1}'</span>, WC_ALetter),
        (<span class="string">'\u{105a3}'</span>, <span class="string">'\u{105b1}'</span>, WC_ALetter), (<span class="string">'\u{105b3}'</span>, <span class="string">'\u{105b9}'</span>, WC_ALetter),
        (<span class="string">'\u{105bb}'</span>, <span class="string">'\u{105bc}'</span>, WC_ALetter), (<span class="string">'\u{10600}'</span>, <span class="string">'\u{10736}'</span>, WC_ALetter),
        (<span class="string">'\u{10740}'</span>, <span class="string">'\u{10755}'</span>, WC_ALetter), (<span class="string">'\u{10760}'</span>, <span class="string">'\u{10767}'</span>, WC_ALetter),
        (<span class="string">'\u{10780}'</span>, <span class="string">'\u{10785}'</span>, WC_ALetter), (<span class="string">'\u{10787}'</span>, <span class="string">'\u{107b0}'</span>, WC_ALetter),
        (<span class="string">'\u{107b2}'</span>, <span class="string">'\u{107ba}'</span>, WC_ALetter), (<span class="string">'\u{10800}'</span>, <span class="string">'\u{10805}'</span>, WC_ALetter),
        (<span class="string">'\u{10808}'</span>, <span class="string">'\u{10808}'</span>, WC_ALetter), (<span class="string">'\u{1080a}'</span>, <span class="string">'\u{10835}'</span>, WC_ALetter),
        (<span class="string">'\u{10837}'</span>, <span class="string">'\u{10838}'</span>, WC_ALetter), (<span class="string">'\u{1083c}'</span>, <span class="string">'\u{1083c}'</span>, WC_ALetter),
        (<span class="string">'\u{1083f}'</span>, <span class="string">'\u{10855}'</span>, WC_ALetter), (<span class="string">'\u{10860}'</span>, <span class="string">'\u{10876}'</span>, WC_ALetter),
        (<span class="string">'\u{10880}'</span>, <span class="string">'\u{1089e}'</span>, WC_ALetter), (<span class="string">'\u{108e0}'</span>, <span class="string">'\u{108f2}'</span>, WC_ALetter),
        (<span class="string">'\u{108f4}'</span>, <span class="string">'\u{108f5}'</span>, WC_ALetter), (<span class="string">'\u{10900}'</span>, <span class="string">'\u{10915}'</span>, WC_ALetter),
        (<span class="string">'\u{10920}'</span>, <span class="string">'\u{10939}'</span>, WC_ALetter), (<span class="string">'\u{10980}'</span>, <span class="string">'\u{109b7}'</span>, WC_ALetter),
        (<span class="string">'\u{109be}'</span>, <span class="string">'\u{109bf}'</span>, WC_ALetter), (<span class="string">'\u{10a00}'</span>, <span class="string">'\u{10a00}'</span>, WC_ALetter),
        (<span class="string">'\u{10a01}'</span>, <span class="string">'\u{10a03}'</span>, WC_Extend), (<span class="string">'\u{10a05}'</span>, <span class="string">'\u{10a06}'</span>, WC_Extend), (<span class="string">'\u{10a0c}'</span>,
        <span class="string">'\u{10a0f}'</span>, WC_Extend), (<span class="string">'\u{10a10}'</span>, <span class="string">'\u{10a13}'</span>, WC_ALetter), (<span class="string">'\u{10a15}'</span>, <span class="string">'\u{10a17}'</span>,
        WC_ALetter), (<span class="string">'\u{10a19}'</span>, <span class="string">'\u{10a35}'</span>, WC_ALetter), (<span class="string">'\u{10a38}'</span>, <span class="string">'\u{10a3a}'</span>, WC_Extend),
        (<span class="string">'\u{10a3f}'</span>, <span class="string">'\u{10a3f}'</span>, WC_Extend), (<span class="string">'\u{10a60}'</span>, <span class="string">'\u{10a7c}'</span>, WC_ALetter), (<span class="string">'\u{10a80}'</span>,
        <span class="string">'\u{10a9c}'</span>, WC_ALetter), (<span class="string">'\u{10ac0}'</span>, <span class="string">'\u{10ac7}'</span>, WC_ALetter), (<span class="string">'\u{10ac9}'</span>, <span class="string">'\u{10ae4}'</span>,
        WC_ALetter), (<span class="string">'\u{10ae5}'</span>, <span class="string">'\u{10ae6}'</span>, WC_Extend), (<span class="string">'\u{10b00}'</span>, <span class="string">'\u{10b35}'</span>, WC_ALetter),
        (<span class="string">'\u{10b40}'</span>, <span class="string">'\u{10b55}'</span>, WC_ALetter), (<span class="string">'\u{10b60}'</span>, <span class="string">'\u{10b72}'</span>, WC_ALetter),
        (<span class="string">'\u{10b80}'</span>, <span class="string">'\u{10b91}'</span>, WC_ALetter), (<span class="string">'\u{10c00}'</span>, <span class="string">'\u{10c48}'</span>, WC_ALetter),
        (<span class="string">'\u{10c80}'</span>, <span class="string">'\u{10cb2}'</span>, WC_ALetter), (<span class="string">'\u{10cc0}'</span>, <span class="string">'\u{10cf2}'</span>, WC_ALetter),
        (<span class="string">'\u{10d00}'</span>, <span class="string">'\u{10d23}'</span>, WC_ALetter), (<span class="string">'\u{10d24}'</span>, <span class="string">'\u{10d27}'</span>, WC_Extend), (<span class="string">'\u{10d30}'</span>,
        <span class="string">'\u{10d39}'</span>, WC_Numeric), (<span class="string">'\u{10e80}'</span>, <span class="string">'\u{10ea9}'</span>, WC_ALetter), (<span class="string">'\u{10eab}'</span>, <span class="string">'\u{10eac}'</span>,
        WC_Extend), (<span class="string">'\u{10eb0}'</span>, <span class="string">'\u{10eb1}'</span>, WC_ALetter), (<span class="string">'\u{10efd}'</span>, <span class="string">'\u{10eff}'</span>, WC_Extend),
        (<span class="string">'\u{10f00}'</span>, <span class="string">'\u{10f1c}'</span>, WC_ALetter), (<span class="string">'\u{10f27}'</span>, <span class="string">'\u{10f27}'</span>, WC_ALetter),
        (<span class="string">'\u{10f30}'</span>, <span class="string">'\u{10f45}'</span>, WC_ALetter), (<span class="string">'\u{10f46}'</span>, <span class="string">'\u{10f50}'</span>, WC_Extend), (<span class="string">'\u{10f70}'</span>,
        <span class="string">'\u{10f81}'</span>, WC_ALetter), (<span class="string">'\u{10f82}'</span>, <span class="string">'\u{10f85}'</span>, WC_Extend), (<span class="string">'\u{10fb0}'</span>, <span class="string">'\u{10fc4}'</span>,
        WC_ALetter), (<span class="string">'\u{10fe0}'</span>, <span class="string">'\u{10ff6}'</span>, WC_ALetter), (<span class="string">'\u{11000}'</span>, <span class="string">'\u{11002}'</span>, WC_Extend),
        (<span class="string">'\u{11003}'</span>, <span class="string">'\u{11037}'</span>, WC_ALetter), (<span class="string">'\u{11038}'</span>, <span class="string">'\u{11046}'</span>, WC_Extend), (<span class="string">'\u{11066}'</span>,
        <span class="string">'\u{1106f}'</span>, WC_Numeric), (<span class="string">'\u{11070}'</span>, <span class="string">'\u{11070}'</span>, WC_Extend), (<span class="string">'\u{11071}'</span>, <span class="string">'\u{11072}'</span>,
        WC_ALetter), (<span class="string">'\u{11073}'</span>, <span class="string">'\u{11074}'</span>, WC_Extend), (<span class="string">'\u{11075}'</span>, <span class="string">'\u{11075}'</span>, WC_ALetter),
        (<span class="string">'\u{1107f}'</span>, <span class="string">'\u{11082}'</span>, WC_Extend), (<span class="string">'\u{11083}'</span>, <span class="string">'\u{110af}'</span>, WC_ALetter), (<span class="string">'\u{110b0}'</span>,
        <span class="string">'\u{110ba}'</span>, WC_Extend), (<span class="string">'\u{110bd}'</span>, <span class="string">'\u{110bd}'</span>, WC_Numeric), (<span class="string">'\u{110c2}'</span>, <span class="string">'\u{110c2}'</span>,
        WC_Extend), (<span class="string">'\u{110cd}'</span>, <span class="string">'\u{110cd}'</span>, WC_Numeric), (<span class="string">'\u{110d0}'</span>, <span class="string">'\u{110e8}'</span>, WC_ALetter),
        (<span class="string">'\u{110f0}'</span>, <span class="string">'\u{110f9}'</span>, WC_Numeric), (<span class="string">'\u{11100}'</span>, <span class="string">'\u{11102}'</span>, WC_Extend), (<span class="string">'\u{11103}'</span>,
        <span class="string">'\u{11126}'</span>, WC_ALetter), (<span class="string">'\u{11127}'</span>, <span class="string">'\u{11134}'</span>, WC_Extend), (<span class="string">'\u{11136}'</span>, <span class="string">'\u{1113f}'</span>,
        WC_Numeric), (<span class="string">'\u{11144}'</span>, <span class="string">'\u{11144}'</span>, WC_ALetter), (<span class="string">'\u{11145}'</span>, <span class="string">'\u{11146}'</span>, WC_Extend),
        (<span class="string">'\u{11147}'</span>, <span class="string">'\u{11147}'</span>, WC_ALetter), (<span class="string">'\u{11150}'</span>, <span class="string">'\u{11172}'</span>, WC_ALetter),
        (<span class="string">'\u{11173}'</span>, <span class="string">'\u{11173}'</span>, WC_Extend), (<span class="string">'\u{11176}'</span>, <span class="string">'\u{11176}'</span>, WC_ALetter), (<span class="string">'\u{11180}'</span>,
        <span class="string">'\u{11182}'</span>, WC_Extend), (<span class="string">'\u{11183}'</span>, <span class="string">'\u{111b2}'</span>, WC_ALetter), (<span class="string">'\u{111b3}'</span>, <span class="string">'\u{111c0}'</span>,
        WC_Extend), (<span class="string">'\u{111c1}'</span>, <span class="string">'\u{111c4}'</span>, WC_ALetter), (<span class="string">'\u{111c9}'</span>, <span class="string">'\u{111cc}'</span>, WC_Extend),
        (<span class="string">'\u{111ce}'</span>, <span class="string">'\u{111cf}'</span>, WC_Extend), (<span class="string">'\u{111d0}'</span>, <span class="string">'\u{111d9}'</span>, WC_Numeric), (<span class="string">'\u{111da}'</span>,
        <span class="string">'\u{111da}'</span>, WC_ALetter), (<span class="string">'\u{111dc}'</span>, <span class="string">'\u{111dc}'</span>, WC_ALetter), (<span class="string">'\u{11200}'</span>, <span class="string">'\u{11211}'</span>,
        WC_ALetter), (<span class="string">'\u{11213}'</span>, <span class="string">'\u{1122b}'</span>, WC_ALetter), (<span class="string">'\u{1122c}'</span>, <span class="string">'\u{11237}'</span>, WC_Extend),
        (<span class="string">'\u{1123e}'</span>, <span class="string">'\u{1123e}'</span>, WC_Extend), (<span class="string">'\u{1123f}'</span>, <span class="string">'\u{11240}'</span>, WC_ALetter), (<span class="string">'\u{11241}'</span>,
        <span class="string">'\u{11241}'</span>, WC_Extend), (<span class="string">'\u{11280}'</span>, <span class="string">'\u{11286}'</span>, WC_ALetter), (<span class="string">'\u{11288}'</span>, <span class="string">'\u{11288}'</span>,
        WC_ALetter), (<span class="string">'\u{1128a}'</span>, <span class="string">'\u{1128d}'</span>, WC_ALetter), (<span class="string">'\u{1128f}'</span>, <span class="string">'\u{1129d}'</span>, WC_ALetter),
        (<span class="string">'\u{1129f}'</span>, <span class="string">'\u{112a8}'</span>, WC_ALetter), (<span class="string">'\u{112b0}'</span>, <span class="string">'\u{112de}'</span>, WC_ALetter),
        (<span class="string">'\u{112df}'</span>, <span class="string">'\u{112ea}'</span>, WC_Extend), (<span class="string">'\u{112f0}'</span>, <span class="string">'\u{112f9}'</span>, WC_Numeric), (<span class="string">'\u{11300}'</span>,
        <span class="string">'\u{11303}'</span>, WC_Extend), (<span class="string">'\u{11305}'</span>, <span class="string">'\u{1130c}'</span>, WC_ALetter), (<span class="string">'\u{1130f}'</span>, <span class="string">'\u{11310}'</span>,
        WC_ALetter), (<span class="string">'\u{11313}'</span>, <span class="string">'\u{11328}'</span>, WC_ALetter), (<span class="string">'\u{1132a}'</span>, <span class="string">'\u{11330}'</span>, WC_ALetter),
        (<span class="string">'\u{11332}'</span>, <span class="string">'\u{11333}'</span>, WC_ALetter), (<span class="string">'\u{11335}'</span>, <span class="string">'\u{11339}'</span>, WC_ALetter),
        (<span class="string">'\u{1133b}'</span>, <span class="string">'\u{1133c}'</span>, WC_Extend), (<span class="string">'\u{1133d}'</span>, <span class="string">'\u{1133d}'</span>, WC_ALetter), (<span class="string">'\u{1133e}'</span>,
        <span class="string">'\u{11344}'</span>, WC_Extend), (<span class="string">'\u{11347}'</span>, <span class="string">'\u{11348}'</span>, WC_Extend), (<span class="string">'\u{1134b}'</span>, <span class="string">'\u{1134d}'</span>,
        WC_Extend), (<span class="string">'\u{11350}'</span>, <span class="string">'\u{11350}'</span>, WC_ALetter), (<span class="string">'\u{11357}'</span>, <span class="string">'\u{11357}'</span>, WC_Extend),
        (<span class="string">'\u{1135d}'</span>, <span class="string">'\u{11361}'</span>, WC_ALetter), (<span class="string">'\u{11362}'</span>, <span class="string">'\u{11363}'</span>, WC_Extend), (<span class="string">'\u{11366}'</span>,
        <span class="string">'\u{1136c}'</span>, WC_Extend), (<span class="string">'\u{11370}'</span>, <span class="string">'\u{11374}'</span>, WC_Extend), (<span class="string">'\u{11400}'</span>, <span class="string">'\u{11434}'</span>,
        WC_ALetter), (<span class="string">'\u{11435}'</span>, <span class="string">'\u{11446}'</span>, WC_Extend), (<span class="string">'\u{11447}'</span>, <span class="string">'\u{1144a}'</span>, WC_ALetter),
        (<span class="string">'\u{11450}'</span>, <span class="string">'\u{11459}'</span>, WC_Numeric), (<span class="string">'\u{1145e}'</span>, <span class="string">'\u{1145e}'</span>, WC_Extend), (<span class="string">'\u{1145f}'</span>,
        <span class="string">'\u{11461}'</span>, WC_ALetter), (<span class="string">'\u{11480}'</span>, <span class="string">'\u{114af}'</span>, WC_ALetter), (<span class="string">'\u{114b0}'</span>, <span class="string">'\u{114c3}'</span>,
        WC_Extend), (<span class="string">'\u{114c4}'</span>, <span class="string">'\u{114c5}'</span>, WC_ALetter), (<span class="string">'\u{114c7}'</span>, <span class="string">'\u{114c7}'</span>, WC_ALetter),
        (<span class="string">'\u{114d0}'</span>, <span class="string">'\u{114d9}'</span>, WC_Numeric), (<span class="string">'\u{11580}'</span>, <span class="string">'\u{115ae}'</span>, WC_ALetter),
        (<span class="string">'\u{115af}'</span>, <span class="string">'\u{115b5}'</span>, WC_Extend), (<span class="string">'\u{115b8}'</span>, <span class="string">'\u{115c0}'</span>, WC_Extend), (<span class="string">'\u{115d8}'</span>,
        <span class="string">'\u{115db}'</span>, WC_ALetter), (<span class="string">'\u{115dc}'</span>, <span class="string">'\u{115dd}'</span>, WC_Extend), (<span class="string">'\u{11600}'</span>, <span class="string">'\u{1162f}'</span>,
        WC_ALetter), (<span class="string">'\u{11630}'</span>, <span class="string">'\u{11640}'</span>, WC_Extend), (<span class="string">'\u{11644}'</span>, <span class="string">'\u{11644}'</span>, WC_ALetter),
        (<span class="string">'\u{11650}'</span>, <span class="string">'\u{11659}'</span>, WC_Numeric), (<span class="string">'\u{11680}'</span>, <span class="string">'\u{116aa}'</span>, WC_ALetter),
        (<span class="string">'\u{116ab}'</span>, <span class="string">'\u{116b7}'</span>, WC_Extend), (<span class="string">'\u{116b8}'</span>, <span class="string">'\u{116b8}'</span>, WC_ALetter), (<span class="string">'\u{116c0}'</span>,
        <span class="string">'\u{116c9}'</span>, WC_Numeric), (<span class="string">'\u{1171d}'</span>, <span class="string">'\u{1172b}'</span>, WC_Extend), (<span class="string">'\u{11730}'</span>, <span class="string">'\u{11739}'</span>,
        WC_Numeric), (<span class="string">'\u{11800}'</span>, <span class="string">'\u{1182b}'</span>, WC_ALetter), (<span class="string">'\u{1182c}'</span>, <span class="string">'\u{1183a}'</span>, WC_Extend),
        (<span class="string">'\u{118a0}'</span>, <span class="string">'\u{118df}'</span>, WC_ALetter), (<span class="string">'\u{118e0}'</span>, <span class="string">'\u{118e9}'</span>, WC_Numeric),
        (<span class="string">'\u{118ff}'</span>, <span class="string">'\u{11906}'</span>, WC_ALetter), (<span class="string">'\u{11909}'</span>, <span class="string">'\u{11909}'</span>, WC_ALetter),
        (<span class="string">'\u{1190c}'</span>, <span class="string">'\u{11913}'</span>, WC_ALetter), (<span class="string">'\u{11915}'</span>, <span class="string">'\u{11916}'</span>, WC_ALetter),
        (<span class="string">'\u{11918}'</span>, <span class="string">'\u{1192f}'</span>, WC_ALetter), (<span class="string">'\u{11930}'</span>, <span class="string">'\u{11935}'</span>, WC_Extend), (<span class="string">'\u{11937}'</span>,
        <span class="string">'\u{11938}'</span>, WC_Extend), (<span class="string">'\u{1193b}'</span>, <span class="string">'\u{1193e}'</span>, WC_Extend), (<span class="string">'\u{1193f}'</span>, <span class="string">'\u{1193f}'</span>,
        WC_ALetter), (<span class="string">'\u{11940}'</span>, <span class="string">'\u{11940}'</span>, WC_Extend), (<span class="string">'\u{11941}'</span>, <span class="string">'\u{11941}'</span>, WC_ALetter),
        (<span class="string">'\u{11942}'</span>, <span class="string">'\u{11943}'</span>, WC_Extend), (<span class="string">'\u{11950}'</span>, <span class="string">'\u{11959}'</span>, WC_Numeric), (<span class="string">'\u{119a0}'</span>,
        <span class="string">'\u{119a7}'</span>, WC_ALetter), (<span class="string">'\u{119aa}'</span>, <span class="string">'\u{119d0}'</span>, WC_ALetter), (<span class="string">'\u{119d1}'</span>, <span class="string">'\u{119d7}'</span>,
        WC_Extend), (<span class="string">'\u{119da}'</span>, <span class="string">'\u{119e0}'</span>, WC_Extend), (<span class="string">'\u{119e1}'</span>, <span class="string">'\u{119e1}'</span>, WC_ALetter),
        (<span class="string">'\u{119e3}'</span>, <span class="string">'\u{119e3}'</span>, WC_ALetter), (<span class="string">'\u{119e4}'</span>, <span class="string">'\u{119e4}'</span>, WC_Extend), (<span class="string">'\u{11a00}'</span>,
        <span class="string">'\u{11a00}'</span>, WC_ALetter), (<span class="string">'\u{11a01}'</span>, <span class="string">'\u{11a0a}'</span>, WC_Extend), (<span class="string">'\u{11a0b}'</span>, <span class="string">'\u{11a32}'</span>,
        WC_ALetter), (<span class="string">'\u{11a33}'</span>, <span class="string">'\u{11a39}'</span>, WC_Extend), (<span class="string">'\u{11a3a}'</span>, <span class="string">'\u{11a3a}'</span>, WC_ALetter),
        (<span class="string">'\u{11a3b}'</span>, <span class="string">'\u{11a3e}'</span>, WC_Extend), (<span class="string">'\u{11a47}'</span>, <span class="string">'\u{11a47}'</span>, WC_Extend), (<span class="string">'\u{11a50}'</span>,
        <span class="string">'\u{11a50}'</span>, WC_ALetter), (<span class="string">'\u{11a51}'</span>, <span class="string">'\u{11a5b}'</span>, WC_Extend), (<span class="string">'\u{11a5c}'</span>, <span class="string">'\u{11a89}'</span>,
        WC_ALetter), (<span class="string">'\u{11a8a}'</span>, <span class="string">'\u{11a99}'</span>, WC_Extend), (<span class="string">'\u{11a9d}'</span>, <span class="string">'\u{11a9d}'</span>, WC_ALetter),
        (<span class="string">'\u{11ab0}'</span>, <span class="string">'\u{11af8}'</span>, WC_ALetter), (<span class="string">'\u{11c00}'</span>, <span class="string">'\u{11c08}'</span>, WC_ALetter),
        (<span class="string">'\u{11c0a}'</span>, <span class="string">'\u{11c2e}'</span>, WC_ALetter), (<span class="string">'\u{11c2f}'</span>, <span class="string">'\u{11c36}'</span>, WC_Extend), (<span class="string">'\u{11c38}'</span>,
        <span class="string">'\u{11c3f}'</span>, WC_Extend), (<span class="string">'\u{11c40}'</span>, <span class="string">'\u{11c40}'</span>, WC_ALetter), (<span class="string">'\u{11c50}'</span>, <span class="string">'\u{11c59}'</span>,
        WC_Numeric), (<span class="string">'\u{11c72}'</span>, <span class="string">'\u{11c8f}'</span>, WC_ALetter), (<span class="string">'\u{11c92}'</span>, <span class="string">'\u{11ca7}'</span>, WC_Extend),
        (<span class="string">'\u{11ca9}'</span>, <span class="string">'\u{11cb6}'</span>, WC_Extend), (<span class="string">'\u{11d00}'</span>, <span class="string">'\u{11d06}'</span>, WC_ALetter), (<span class="string">'\u{11d08}'</span>,
        <span class="string">'\u{11d09}'</span>, WC_ALetter), (<span class="string">'\u{11d0b}'</span>, <span class="string">'\u{11d30}'</span>, WC_ALetter), (<span class="string">'\u{11d31}'</span>, <span class="string">'\u{11d36}'</span>,
        WC_Extend), (<span class="string">'\u{11d3a}'</span>, <span class="string">'\u{11d3a}'</span>, WC_Extend), (<span class="string">'\u{11d3c}'</span>, <span class="string">'\u{11d3d}'</span>, WC_Extend),
        (<span class="string">'\u{11d3f}'</span>, <span class="string">'\u{11d45}'</span>, WC_Extend), (<span class="string">'\u{11d46}'</span>, <span class="string">'\u{11d46}'</span>, WC_ALetter), (<span class="string">'\u{11d47}'</span>,
        <span class="string">'\u{11d47}'</span>, WC_Extend), (<span class="string">'\u{11d50}'</span>, <span class="string">'\u{11d59}'</span>, WC_Numeric), (<span class="string">'\u{11d60}'</span>, <span class="string">'\u{11d65}'</span>,
        WC_ALetter), (<span class="string">'\u{11d67}'</span>, <span class="string">'\u{11d68}'</span>, WC_ALetter), (<span class="string">'\u{11d6a}'</span>, <span class="string">'\u{11d89}'</span>, WC_ALetter),
        (<span class="string">'\u{11d8a}'</span>, <span class="string">'\u{11d8e}'</span>, WC_Extend), (<span class="string">'\u{11d90}'</span>, <span class="string">'\u{11d91}'</span>, WC_Extend), (<span class="string">'\u{11d93}'</span>,
        <span class="string">'\u{11d97}'</span>, WC_Extend), (<span class="string">'\u{11d98}'</span>, <span class="string">'\u{11d98}'</span>, WC_ALetter), (<span class="string">'\u{11da0}'</span>, <span class="string">'\u{11da9}'</span>,
        WC_Numeric), (<span class="string">'\u{11ee0}'</span>, <span class="string">'\u{11ef2}'</span>, WC_ALetter), (<span class="string">'\u{11ef3}'</span>, <span class="string">'\u{11ef6}'</span>, WC_Extend),
        (<span class="string">'\u{11f00}'</span>, <span class="string">'\u{11f01}'</span>, WC_Extend), (<span class="string">'\u{11f02}'</span>, <span class="string">'\u{11f02}'</span>, WC_ALetter), (<span class="string">'\u{11f03}'</span>,
        <span class="string">'\u{11f03}'</span>, WC_Extend), (<span class="string">'\u{11f04}'</span>, <span class="string">'\u{11f10}'</span>, WC_ALetter), (<span class="string">'\u{11f12}'</span>, <span class="string">'\u{11f33}'</span>,
        WC_ALetter), (<span class="string">'\u{11f34}'</span>, <span class="string">'\u{11f3a}'</span>, WC_Extend), (<span class="string">'\u{11f3e}'</span>, <span class="string">'\u{11f42}'</span>, WC_Extend),
        (<span class="string">'\u{11f50}'</span>, <span class="string">'\u{11f59}'</span>, WC_Numeric), (<span class="string">'\u{11fb0}'</span>, <span class="string">'\u{11fb0}'</span>, WC_ALetter),
        (<span class="string">'\u{12000}'</span>, <span class="string">'\u{12399}'</span>, WC_ALetter), (<span class="string">'\u{12400}'</span>, <span class="string">'\u{1246e}'</span>, WC_ALetter),
        (<span class="string">'\u{12480}'</span>, <span class="string">'\u{12543}'</span>, WC_ALetter), (<span class="string">'\u{12f90}'</span>, <span class="string">'\u{12ff0}'</span>, WC_ALetter),
        (<span class="string">'\u{13000}'</span>, <span class="string">'\u{1342f}'</span>, WC_ALetter), (<span class="string">'\u{13430}'</span>, <span class="string">'\u{1343f}'</span>, WC_Format), (<span class="string">'\u{13440}'</span>,
        <span class="string">'\u{13440}'</span>, WC_Extend), (<span class="string">'\u{13441}'</span>, <span class="string">'\u{13446}'</span>, WC_ALetter), (<span class="string">'\u{13447}'</span>, <span class="string">'\u{13455}'</span>,
        WC_Extend), (<span class="string">'\u{14400}'</span>, <span class="string">'\u{14646}'</span>, WC_ALetter), (<span class="string">'\u{16800}'</span>, <span class="string">'\u{16a38}'</span>, WC_ALetter),
        (<span class="string">'\u{16a40}'</span>, <span class="string">'\u{16a5e}'</span>, WC_ALetter), (<span class="string">'\u{16a60}'</span>, <span class="string">'\u{16a69}'</span>, WC_Numeric),
        (<span class="string">'\u{16a70}'</span>, <span class="string">'\u{16abe}'</span>, WC_ALetter), (<span class="string">'\u{16ac0}'</span>, <span class="string">'\u{16ac9}'</span>, WC_Numeric),
        (<span class="string">'\u{16ad0}'</span>, <span class="string">'\u{16aed}'</span>, WC_ALetter), (<span class="string">'\u{16af0}'</span>, <span class="string">'\u{16af4}'</span>, WC_Extend), (<span class="string">'\u{16b00}'</span>,
        <span class="string">'\u{16b2f}'</span>, WC_ALetter), (<span class="string">'\u{16b30}'</span>, <span class="string">'\u{16b36}'</span>, WC_Extend), (<span class="string">'\u{16b40}'</span>, <span class="string">'\u{16b43}'</span>,
        WC_ALetter), (<span class="string">'\u{16b50}'</span>, <span class="string">'\u{16b59}'</span>, WC_Numeric), (<span class="string">'\u{16b63}'</span>, <span class="string">'\u{16b77}'</span>, WC_ALetter),
        (<span class="string">'\u{16b7d}'</span>, <span class="string">'\u{16b8f}'</span>, WC_ALetter), (<span class="string">'\u{16e40}'</span>, <span class="string">'\u{16e7f}'</span>, WC_ALetter),
        (<span class="string">'\u{16f00}'</span>, <span class="string">'\u{16f4a}'</span>, WC_ALetter), (<span class="string">'\u{16f4f}'</span>, <span class="string">'\u{16f4f}'</span>, WC_Extend), (<span class="string">'\u{16f50}'</span>,
        <span class="string">'\u{16f50}'</span>, WC_ALetter), (<span class="string">'\u{16f51}'</span>, <span class="string">'\u{16f87}'</span>, WC_Extend), (<span class="string">'\u{16f8f}'</span>, <span class="string">'\u{16f92}'</span>,
        WC_Extend), (<span class="string">'\u{16f93}'</span>, <span class="string">'\u{16f9f}'</span>, WC_ALetter), (<span class="string">'\u{16fe0}'</span>, <span class="string">'\u{16fe1}'</span>, WC_ALetter),
        (<span class="string">'\u{16fe3}'</span>, <span class="string">'\u{16fe3}'</span>, WC_ALetter), (<span class="string">'\u{16fe4}'</span>, <span class="string">'\u{16fe4}'</span>, WC_Extend), (<span class="string">'\u{16ff0}'</span>,
        <span class="string">'\u{16ff1}'</span>, WC_Extend), (<span class="string">'\u{1aff0}'</span>, <span class="string">'\u{1aff3}'</span>, WC_Katakana), (<span class="string">'\u{1aff5}'</span>, <span class="string">'\u{1affb}'</span>,
        WC_Katakana), (<span class="string">'\u{1affd}'</span>, <span class="string">'\u{1affe}'</span>, WC_Katakana), (<span class="string">'\u{1b000}'</span>, <span class="string">'\u{1b000}'</span>,
        WC_Katakana), (<span class="string">'\u{1b120}'</span>, <span class="string">'\u{1b122}'</span>, WC_Katakana), (<span class="string">'\u{1b155}'</span>, <span class="string">'\u{1b155}'</span>,
        WC_Katakana), (<span class="string">'\u{1b164}'</span>, <span class="string">'\u{1b167}'</span>, WC_Katakana), (<span class="string">'\u{1bc00}'</span>, <span class="string">'\u{1bc6a}'</span>,
        WC_ALetter), (<span class="string">'\u{1bc70}'</span>, <span class="string">'\u{1bc7c}'</span>, WC_ALetter), (<span class="string">'\u{1bc80}'</span>, <span class="string">'\u{1bc88}'</span>, WC_ALetter),
        (<span class="string">'\u{1bc90}'</span>, <span class="string">'\u{1bc99}'</span>, WC_ALetter), (<span class="string">'\u{1bc9d}'</span>, <span class="string">'\u{1bc9e}'</span>, WC_Extend), (<span class="string">'\u{1bca0}'</span>,
        <span class="string">'\u{1bca3}'</span>, WC_Format), (<span class="string">'\u{1cf00}'</span>, <span class="string">'\u{1cf2d}'</span>, WC_Extend), (<span class="string">'\u{1cf30}'</span>, <span class="string">'\u{1cf46}'</span>,
        WC_Extend), (<span class="string">'\u{1d165}'</span>, <span class="string">'\u{1d169}'</span>, WC_Extend), (<span class="string">'\u{1d16d}'</span>, <span class="string">'\u{1d172}'</span>, WC_Extend),
        (<span class="string">'\u{1d173}'</span>, <span class="string">'\u{1d17a}'</span>, WC_Format), (<span class="string">'\u{1d17b}'</span>, <span class="string">'\u{1d182}'</span>, WC_Extend), (<span class="string">'\u{1d185}'</span>,
        <span class="string">'\u{1d18b}'</span>, WC_Extend), (<span class="string">'\u{1d1aa}'</span>, <span class="string">'\u{1d1ad}'</span>, WC_Extend), (<span class="string">'\u{1d242}'</span>, <span class="string">'\u{1d244}'</span>,
        WC_Extend), (<span class="string">'\u{1d400}'</span>, <span class="string">'\u{1d454}'</span>, WC_ALetter), (<span class="string">'\u{1d456}'</span>, <span class="string">'\u{1d49c}'</span>, WC_ALetter),
        (<span class="string">'\u{1d49e}'</span>, <span class="string">'\u{1d49f}'</span>, WC_ALetter), (<span class="string">'\u{1d4a2}'</span>, <span class="string">'\u{1d4a2}'</span>, WC_ALetter),
        (<span class="string">'\u{1d4a5}'</span>, <span class="string">'\u{1d4a6}'</span>, WC_ALetter), (<span class="string">'\u{1d4a9}'</span>, <span class="string">'\u{1d4ac}'</span>, WC_ALetter),
        (<span class="string">'\u{1d4ae}'</span>, <span class="string">'\u{1d4b9}'</span>, WC_ALetter), (<span class="string">'\u{1d4bb}'</span>, <span class="string">'\u{1d4bb}'</span>, WC_ALetter),
        (<span class="string">'\u{1d4bd}'</span>, <span class="string">'\u{1d4c3}'</span>, WC_ALetter), (<span class="string">'\u{1d4c5}'</span>, <span class="string">'\u{1d505}'</span>, WC_ALetter),
        (<span class="string">'\u{1d507}'</span>, <span class="string">'\u{1d50a}'</span>, WC_ALetter), (<span class="string">'\u{1d50d}'</span>, <span class="string">'\u{1d514}'</span>, WC_ALetter),
        (<span class="string">'\u{1d516}'</span>, <span class="string">'\u{1d51c}'</span>, WC_ALetter), (<span class="string">'\u{1d51e}'</span>, <span class="string">'\u{1d539}'</span>, WC_ALetter),
        (<span class="string">'\u{1d53b}'</span>, <span class="string">'\u{1d53e}'</span>, WC_ALetter), (<span class="string">'\u{1d540}'</span>, <span class="string">'\u{1d544}'</span>, WC_ALetter),
        (<span class="string">'\u{1d546}'</span>, <span class="string">'\u{1d546}'</span>, WC_ALetter), (<span class="string">'\u{1d54a}'</span>, <span class="string">'\u{1d550}'</span>, WC_ALetter),
        (<span class="string">'\u{1d552}'</span>, <span class="string">'\u{1d6a5}'</span>, WC_ALetter), (<span class="string">'\u{1d6a8}'</span>, <span class="string">'\u{1d6c0}'</span>, WC_ALetter),
        (<span class="string">'\u{1d6c2}'</span>, <span class="string">'\u{1d6da}'</span>, WC_ALetter), (<span class="string">'\u{1d6dc}'</span>, <span class="string">'\u{1d6fa}'</span>, WC_ALetter),
        (<span class="string">'\u{1d6fc}'</span>, <span class="string">'\u{1d714}'</span>, WC_ALetter), (<span class="string">'\u{1d716}'</span>, <span class="string">'\u{1d734}'</span>, WC_ALetter),
        (<span class="string">'\u{1d736}'</span>, <span class="string">'\u{1d74e}'</span>, WC_ALetter), (<span class="string">'\u{1d750}'</span>, <span class="string">'\u{1d76e}'</span>, WC_ALetter),
        (<span class="string">'\u{1d770}'</span>, <span class="string">'\u{1d788}'</span>, WC_ALetter), (<span class="string">'\u{1d78a}'</span>, <span class="string">'\u{1d7a8}'</span>, WC_ALetter),
        (<span class="string">'\u{1d7aa}'</span>, <span class="string">'\u{1d7c2}'</span>, WC_ALetter), (<span class="string">'\u{1d7c4}'</span>, <span class="string">'\u{1d7cb}'</span>, WC_ALetter),
        (<span class="string">'\u{1d7ce}'</span>, <span class="string">'\u{1d7ff}'</span>, WC_Numeric), (<span class="string">'\u{1da00}'</span>, <span class="string">'\u{1da36}'</span>, WC_Extend), (<span class="string">'\u{1da3b}'</span>,
        <span class="string">'\u{1da6c}'</span>, WC_Extend), (<span class="string">'\u{1da75}'</span>, <span class="string">'\u{1da75}'</span>, WC_Extend), (<span class="string">'\u{1da84}'</span>, <span class="string">'\u{1da84}'</span>,
        WC_Extend), (<span class="string">'\u{1da9b}'</span>, <span class="string">'\u{1da9f}'</span>, WC_Extend), (<span class="string">'\u{1daa1}'</span>, <span class="string">'\u{1daaf}'</span>, WC_Extend),
        (<span class="string">'\u{1df00}'</span>, <span class="string">'\u{1df1e}'</span>, WC_ALetter), (<span class="string">'\u{1df25}'</span>, <span class="string">'\u{1df2a}'</span>, WC_ALetter),
        (<span class="string">'\u{1e000}'</span>, <span class="string">'\u{1e006}'</span>, WC_Extend), (<span class="string">'\u{1e008}'</span>, <span class="string">'\u{1e018}'</span>, WC_Extend), (<span class="string">'\u{1e01b}'</span>,
        <span class="string">'\u{1e021}'</span>, WC_Extend), (<span class="string">'\u{1e023}'</span>, <span class="string">'\u{1e024}'</span>, WC_Extend), (<span class="string">'\u{1e026}'</span>, <span class="string">'\u{1e02a}'</span>,
        WC_Extend), (<span class="string">'\u{1e030}'</span>, <span class="string">'\u{1e06d}'</span>, WC_ALetter), (<span class="string">'\u{1e08f}'</span>, <span class="string">'\u{1e08f}'</span>, WC_Extend),
        (<span class="string">'\u{1e100}'</span>, <span class="string">'\u{1e12c}'</span>, WC_ALetter), (<span class="string">'\u{1e130}'</span>, <span class="string">'\u{1e136}'</span>, WC_Extend), (<span class="string">'\u{1e137}'</span>,
        <span class="string">'\u{1e13d}'</span>, WC_ALetter), (<span class="string">'\u{1e140}'</span>, <span class="string">'\u{1e149}'</span>, WC_Numeric), (<span class="string">'\u{1e14e}'</span>, <span class="string">'\u{1e14e}'</span>,
        WC_ALetter), (<span class="string">'\u{1e290}'</span>, <span class="string">'\u{1e2ad}'</span>, WC_ALetter), (<span class="string">'\u{1e2ae}'</span>, <span class="string">'\u{1e2ae}'</span>, WC_Extend),
        (<span class="string">'\u{1e2c0}'</span>, <span class="string">'\u{1e2eb}'</span>, WC_ALetter), (<span class="string">'\u{1e2ec}'</span>, <span class="string">'\u{1e2ef}'</span>, WC_Extend), (<span class="string">'\u{1e2f0}'</span>,
        <span class="string">'\u{1e2f9}'</span>, WC_Numeric), (<span class="string">'\u{1e4d0}'</span>, <span class="string">'\u{1e4eb}'</span>, WC_ALetter), (<span class="string">'\u{1e4ec}'</span>, <span class="string">'\u{1e4ef}'</span>,
        WC_Extend), (<span class="string">'\u{1e4f0}'</span>, <span class="string">'\u{1e4f9}'</span>, WC_Numeric), (<span class="string">'\u{1e7e0}'</span>, <span class="string">'\u{1e7e6}'</span>, WC_ALetter),
        (<span class="string">'\u{1e7e8}'</span>, <span class="string">'\u{1e7eb}'</span>, WC_ALetter), (<span class="string">'\u{1e7ed}'</span>, <span class="string">'\u{1e7ee}'</span>, WC_ALetter),
        (<span class="string">'\u{1e7f0}'</span>, <span class="string">'\u{1e7fe}'</span>, WC_ALetter), (<span class="string">'\u{1e800}'</span>, <span class="string">'\u{1e8c4}'</span>, WC_ALetter),
        (<span class="string">'\u{1e8d0}'</span>, <span class="string">'\u{1e8d6}'</span>, WC_Extend), (<span class="string">'\u{1e900}'</span>, <span class="string">'\u{1e943}'</span>, WC_ALetter), (<span class="string">'\u{1e944}'</span>,
        <span class="string">'\u{1e94a}'</span>, WC_Extend), (<span class="string">'\u{1e94b}'</span>, <span class="string">'\u{1e94b}'</span>, WC_ALetter), (<span class="string">'\u{1e950}'</span>, <span class="string">'\u{1e959}'</span>,
        WC_Numeric), (<span class="string">'\u{1ee00}'</span>, <span class="string">'\u{1ee03}'</span>, WC_ALetter), (<span class="string">'\u{1ee05}'</span>, <span class="string">'\u{1ee1f}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee21}'</span>, <span class="string">'\u{1ee22}'</span>, WC_ALetter), (<span class="string">'\u{1ee24}'</span>, <span class="string">'\u{1ee24}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee27}'</span>, <span class="string">'\u{1ee27}'</span>, WC_ALetter), (<span class="string">'\u{1ee29}'</span>, <span class="string">'\u{1ee32}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee34}'</span>, <span class="string">'\u{1ee37}'</span>, WC_ALetter), (<span class="string">'\u{1ee39}'</span>, <span class="string">'\u{1ee39}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee3b}'</span>, <span class="string">'\u{1ee3b}'</span>, WC_ALetter), (<span class="string">'\u{1ee42}'</span>, <span class="string">'\u{1ee42}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee47}'</span>, <span class="string">'\u{1ee47}'</span>, WC_ALetter), (<span class="string">'\u{1ee49}'</span>, <span class="string">'\u{1ee49}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee4b}'</span>, <span class="string">'\u{1ee4b}'</span>, WC_ALetter), (<span class="string">'\u{1ee4d}'</span>, <span class="string">'\u{1ee4f}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee51}'</span>, <span class="string">'\u{1ee52}'</span>, WC_ALetter), (<span class="string">'\u{1ee54}'</span>, <span class="string">'\u{1ee54}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee57}'</span>, <span class="string">'\u{1ee57}'</span>, WC_ALetter), (<span class="string">'\u{1ee59}'</span>, <span class="string">'\u{1ee59}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee5b}'</span>, <span class="string">'\u{1ee5b}'</span>, WC_ALetter), (<span class="string">'\u{1ee5d}'</span>, <span class="string">'\u{1ee5d}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee5f}'</span>, <span class="string">'\u{1ee5f}'</span>, WC_ALetter), (<span class="string">'\u{1ee61}'</span>, <span class="string">'\u{1ee62}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee64}'</span>, <span class="string">'\u{1ee64}'</span>, WC_ALetter), (<span class="string">'\u{1ee67}'</span>, <span class="string">'\u{1ee6a}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee6c}'</span>, <span class="string">'\u{1ee72}'</span>, WC_ALetter), (<span class="string">'\u{1ee74}'</span>, <span class="string">'\u{1ee77}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee79}'</span>, <span class="string">'\u{1ee7c}'</span>, WC_ALetter), (<span class="string">'\u{1ee7e}'</span>, <span class="string">'\u{1ee7e}'</span>, WC_ALetter),
        (<span class="string">'\u{1ee80}'</span>, <span class="string">'\u{1ee89}'</span>, WC_ALetter), (<span class="string">'\u{1ee8b}'</span>, <span class="string">'\u{1ee9b}'</span>, WC_ALetter),
        (<span class="string">'\u{1eea1}'</span>, <span class="string">'\u{1eea3}'</span>, WC_ALetter), (<span class="string">'\u{1eea5}'</span>, <span class="string">'\u{1eea9}'</span>, WC_ALetter),
        (<span class="string">'\u{1eeab}'</span>, <span class="string">'\u{1eebb}'</span>, WC_ALetter), (<span class="string">'\u{1f130}'</span>, <span class="string">'\u{1f149}'</span>, WC_ALetter),
        (<span class="string">'\u{1f150}'</span>, <span class="string">'\u{1f169}'</span>, WC_ALetter), (<span class="string">'\u{1f170}'</span>, <span class="string">'\u{1f189}'</span>, WC_ALetter),
        (<span class="string">'\u{1f1e6}'</span>, <span class="string">'\u{1f1ff}'</span>, WC_Regional_Indicator), (<span class="string">'\u{1f3fb}'</span>, <span class="string">'\u{1f3ff}'</span>, WC_Extend),
        (<span class="string">'\u{1fbf0}'</span>, <span class="string">'\u{1fbf9}'</span>, WC_Numeric), (<span class="string">'\u{e0001}'</span>, <span class="string">'\u{e0001}'</span>, WC_Format), (<span class="string">'\u{e0020}'</span>,
        <span class="string">'\u{e007f}'</span>, WC_Extend), (<span class="string">'\u{e0100}'</span>, <span class="string">'\u{e01ef}'</span>, WC_Extend)
    ];

}
<span class="kw">pub mod </span>emoji {
    <span class="kw">use </span>core::result::Result::{<span class="prelude-val">Ok</span>, <span class="prelude-val">Err</span>};

    <span class="kw">pub use </span><span class="self">self</span>::EmojiCat::<span class="kw-2">*</span>;

    <span class="attr">#[allow(non_camel_case_types)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    </span><span class="kw">pub enum </span>EmojiCat {
        EC_Any,
        EC_Extended_Pictographic,
    }

    <span class="kw">fn </span>bsearch_range_value_table(c: char, r: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char, EmojiCat)], default_lower: u32, default_upper: u32) -&gt; (u32, u32, EmojiCat) {
        <span class="kw">use </span>core::cmp::Ordering::{Equal, Less, Greater};
        <span class="kw">match </span>r.binary_search_by(|<span class="kw-2">&amp;</span>(lo, hi, <span class="kw">_</span>)| {
            <span class="kw">if </span>lo &lt;= c &amp;&amp; c &lt;= hi { Equal }
            <span class="kw">else if </span>hi &lt; c { Less }
            <span class="kw">else </span>{ Greater }
        }) {
            <span class="prelude-val">Ok</span>(idx) =&gt; {
                <span class="kw">let </span>(lower, upper, cat) = r[idx];
                (lower <span class="kw">as </span>u32, upper <span class="kw">as </span>u32, cat)
            }
            <span class="prelude-val">Err</span>(idx) =&gt; {
                (
                    <span class="kw">if </span>idx &gt; <span class="number">0 </span>{ r[idx-<span class="number">1</span>].<span class="number">1 </span><span class="kw">as </span>u32 + <span class="number">1 </span>} <span class="kw">else </span>{ default_lower },
                    r.get(idx).map(|c|c.<span class="number">0 </span><span class="kw">as </span>u32 - <span class="number">1</span>).unwrap_or(default_upper),
                    EC_Any,
                )
            }
        }
    }

    <span class="kw">pub fn </span>emoji_category(c: char) -&gt; (u32, u32, EmojiCat) {
        <span class="comment">// Perform a quick O(1) lookup in a precomputed table to determine
        // the slice of the range table to search in.
        </span><span class="kw">let </span>lookup_interval = <span class="number">0x80</span>;
        <span class="kw">let </span>idx = (c <span class="kw">as </span>u32 / lookup_interval) <span class="kw">as </span>usize;
        <span class="kw">let </span>range = emoji_cat_lookup.get(idx..(idx + <span class="number">2</span>)).map_or(
          <span class="comment">// If the `idx` is outside of the precomputed table - use the slice
          // starting from the last covered index in the precomputed table and
          // ending with the length of the range table.
          </span><span class="number">77</span>..<span class="number">78</span>,
          |r| (r[<span class="number">0</span>] <span class="kw">as </span>usize)..((r[<span class="number">1</span>] + <span class="number">1</span>) <span class="kw">as </span>usize)
        );

        <span class="comment">// Compute pessimistic default lower and upper bounds on the category.
        // If character doesn't map to any range and there is no adjacent range
        // in the table slice - these bounds has to apply.
        </span><span class="kw">let </span>lower = idx <span class="kw">as </span>u32 * lookup_interval;
        <span class="kw">let </span>upper = lower + lookup_interval - <span class="number">1</span>;
        bsearch_range_value_table(c, <span class="kw-2">&amp;</span>emoji_cat_table[range], lower, upper)
    }

    <span class="kw">const </span>emoji_cat_lookup: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[u8] = <span class="kw-2">&amp;</span>[
        <span class="number">0</span>, <span class="number">0</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>,
        <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>,
        <span class="number">2</span>, <span class="number">2</span>, <span class="number">2</span>, <span class="number">4</span>, <span class="number">4</span>, <span class="number">6</span>, <span class="number">8</span>, <span class="number">8</span>, <span class="number">8</span>, <span class="number">10</span>, <span class="number">14</span>, <span class="number">14</span>, <span class="number">15</span>, <span class="number">15</span>, <span class="number">19</span>, <span class="number">21</span>, <span class="number">22</span>, <span class="number">37</span>, <span class="number">41</span>, <span class="number">41</span>, <span class="number">41</span>, <span class="number">42</span>, <span class="number">42</span>, <span class="number">42</span>, <span class="number">42</span>,
        <span class="number">46</span>, <span class="number">46</span>, <span class="number">46</span>, <span class="number">46</span>, <span class="number">46</span>, <span class="number">46</span>, <span class="number">46</span>, <span class="number">46</span>, <span class="number">46</span>, <span class="number">46</span>, <span class="number">48</span>, <span class="number">48</span>, <span class="number">48</span>, <span class="number">48</span>, <span class="number">48</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>,
        <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">50</span>, <span class="number">51</span>, <span class="number">55</span>, <span class="number">58</span>, <span class="number">63</span>, <span class="number">63</span>, <span class="number">63</span>, <span class="number">64</span>, <span class="number">64</span>, <span class="number">64</span>, <span class="number">65</span>, <span class="number">65</span>, <span class="number">66</span>, <span class="number">67</span>,
        <span class="number">68</span>, <span class="number">69</span>, <span class="number">72</span>, <span class="number">74</span>, <span class="number">76</span>, <span class="number">76</span>, <span class="number">76</span>, <span class="number">77</span>, <span class="number">77</span>, <span class="number">77</span>, <span class="number">77</span>, <span class="number">77</span>, <span class="number">77</span>, <span class="number">77</span>, <span class="number">77</span>, <span class="number">77</span>, <span class="number">77
    </span>];

    <span class="kw">const </span>emoji_cat_table: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char, EmojiCat)] = <span class="kw-2">&amp;</span>[
        (<span class="string">'\u{a9}'</span>, <span class="string">'\u{a9}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{ae}'</span>, <span class="string">'\u{ae}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{203c}'</span>, <span class="string">'\u{203c}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2049}'</span>,
        <span class="string">'\u{2049}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2122}'</span>, <span class="string">'\u{2122}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{2139}'</span>, <span class="string">'\u{2139}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2194}'</span>, <span class="string">'\u{2199}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{21a9}'</span>, <span class="string">'\u{21aa}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{231a}'</span>,
        <span class="string">'\u{231b}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2328}'</span>, <span class="string">'\u{2328}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{2388}'</span>, <span class="string">'\u{2388}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{23cf}'</span>, <span class="string">'\u{23cf}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{23e9}'</span>, <span class="string">'\u{23f3}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{23f8}'</span>,
        <span class="string">'\u{23fa}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{24c2}'</span>, <span class="string">'\u{24c2}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{25aa}'</span>, <span class="string">'\u{25ab}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{25b6}'</span>, <span class="string">'\u{25b6}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{25c0}'</span>, <span class="string">'\u{25c0}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{25fb}'</span>,
        <span class="string">'\u{25fe}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2600}'</span>, <span class="string">'\u{2605}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{2607}'</span>, <span class="string">'\u{2612}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2614}'</span>, <span class="string">'\u{2685}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{2690}'</span>, <span class="string">'\u{2705}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2708}'</span>,
        <span class="string">'\u{2712}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2714}'</span>, <span class="string">'\u{2714}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{2716}'</span>, <span class="string">'\u{2716}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{271d}'</span>, <span class="string">'\u{271d}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{2721}'</span>, <span class="string">'\u{2721}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2728}'</span>,
        <span class="string">'\u{2728}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2733}'</span>, <span class="string">'\u{2734}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{2744}'</span>, <span class="string">'\u{2744}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2747}'</span>, <span class="string">'\u{2747}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{274c}'</span>, <span class="string">'\u{274c}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{274e}'</span>,
        <span class="string">'\u{274e}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2753}'</span>, <span class="string">'\u{2755}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{2757}'</span>, <span class="string">'\u{2757}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2763}'</span>, <span class="string">'\u{2767}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{2795}'</span>, <span class="string">'\u{2797}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{27a1}'</span>,
        <span class="string">'\u{27a1}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{27b0}'</span>, <span class="string">'\u{27b0}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{27bf}'</span>, <span class="string">'\u{27bf}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2934}'</span>, <span class="string">'\u{2935}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{2b05}'</span>, <span class="string">'\u{2b07}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2b1b}'</span>,
        <span class="string">'\u{2b1c}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{2b50}'</span>, <span class="string">'\u{2b50}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{2b55}'</span>, <span class="string">'\u{2b55}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{3030}'</span>, <span class="string">'\u{3030}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{303d}'</span>, <span class="string">'\u{303d}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{3297}'</span>,
        <span class="string">'\u{3297}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{3299}'</span>, <span class="string">'\u{3299}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{1f000}'</span>, <span class="string">'\u{1f0ff}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{1f10d}'</span>, <span class="string">'\u{1f10f}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{1f12f}'</span>, <span class="string">'\u{1f12f}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{1f16c}'</span>, <span class="string">'\u{1f171}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{1f17e}'</span>, <span class="string">'\u{1f17f}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{1f18e}'</span>, <span class="string">'\u{1f18e}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{1f191}'</span>, <span class="string">'\u{1f19a}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{1f1ad}'</span>, <span class="string">'\u{1f1e5}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{1f201}'</span>, <span class="string">'\u{1f20f}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{1f21a}'</span>, <span class="string">'\u{1f21a}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{1f22f}'</span>, <span class="string">'\u{1f22f}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{1f232}'</span>, <span class="string">'\u{1f23a}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{1f23c}'</span>, <span class="string">'\u{1f23f}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{1f249}'</span>, <span class="string">'\u{1f3fa}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{1f400}'</span>, <span class="string">'\u{1f53d}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{1f546}'</span>, <span class="string">'\u{1f64f}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{1f680}'</span>, <span class="string">'\u{1f6ff}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{1f774}'</span>, <span class="string">'\u{1f77f}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{1f7d5}'</span>, <span class="string">'\u{1f7ff}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{1f80c}'</span>, <span class="string">'\u{1f80f}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{1f848}'</span>, <span class="string">'\u{1f84f}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{1f85a}'</span>, <span class="string">'\u{1f85f}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{1f888}'</span>, <span class="string">'\u{1f88f}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{1f8ae}'</span>, <span class="string">'\u{1f8ff}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{1f90c}'</span>, <span class="string">'\u{1f93a}'</span>, EC_Extended_Pictographic), (<span class="string">'\u{1f93c}'</span>, <span class="string">'\u{1f945}'</span>,
        EC_Extended_Pictographic), (<span class="string">'\u{1f947}'</span>, <span class="string">'\u{1faff}'</span>, EC_Extended_Pictographic),
        (<span class="string">'\u{1fc00}'</span>, <span class="string">'\u{1fffd}'</span>, EC_Extended_Pictographic)
    ];

}
<span class="kw">pub mod </span>sentence {
    <span class="kw">use </span>core::result::Result::{<span class="prelude-val">Ok</span>, <span class="prelude-val">Err</span>};

    <span class="kw">pub use </span><span class="self">self</span>::SentenceCat::<span class="kw-2">*</span>;

    <span class="attr">#[allow(non_camel_case_types)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    </span><span class="kw">pub enum </span>SentenceCat {
        SC_ATerm,
        SC_Any,
        SC_CR,
        SC_Close,
        SC_Extend,
        SC_Format,
        SC_LF,
        SC_Lower,
        SC_Numeric,
        SC_OLetter,
        SC_SContinue,
        SC_STerm,
        SC_Sep,
        SC_Sp,
        SC_Upper,
    }

    <span class="kw">fn </span>bsearch_range_value_table(c: char, r: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char, SentenceCat)], default_lower: u32, default_upper: u32) -&gt; (u32, u32, SentenceCat) {
        <span class="kw">use </span>core::cmp::Ordering::{Equal, Less, Greater};
        <span class="kw">match </span>r.binary_search_by(|<span class="kw-2">&amp;</span>(lo, hi, <span class="kw">_</span>)| {
            <span class="kw">if </span>lo &lt;= c &amp;&amp; c &lt;= hi { Equal }
            <span class="kw">else if </span>hi &lt; c { Less }
            <span class="kw">else </span>{ Greater }
        }) {
            <span class="prelude-val">Ok</span>(idx) =&gt; {
                <span class="kw">let </span>(lower, upper, cat) = r[idx];
                (lower <span class="kw">as </span>u32, upper <span class="kw">as </span>u32, cat)
            }
            <span class="prelude-val">Err</span>(idx) =&gt; {
                (
                    <span class="kw">if </span>idx &gt; <span class="number">0 </span>{ r[idx-<span class="number">1</span>].<span class="number">1 </span><span class="kw">as </span>u32 + <span class="number">1 </span>} <span class="kw">else </span>{ default_lower },
                    r.get(idx).map(|c|c.<span class="number">0 </span><span class="kw">as </span>u32 - <span class="number">1</span>).unwrap_or(default_upper),
                    SC_Any,
                )
            }
        }
    }

    <span class="kw">pub fn </span>sentence_category(c: char) -&gt; (u32, u32, SentenceCat) {
        <span class="comment">// Perform a quick O(1) lookup in a precomputed table to determine
        // the slice of the range table to search in.
        </span><span class="kw">let </span>lookup_interval = <span class="number">0x80</span>;
        <span class="kw">let </span>idx = (c <span class="kw">as </span>u32 / lookup_interval) <span class="kw">as </span>usize;
        <span class="kw">let </span>range = sentence_cat_lookup.get(idx..(idx + <span class="number">2</span>)).map_or(
          <span class="comment">// If the `idx` is outside of the precomputed table - use the slice
          // starting from the last covered index in the precomputed table and
          // ending with the length of the range table.
          </span><span class="number">2411</span>..<span class="number">2423</span>,
          |r| (r[<span class="number">0</span>] <span class="kw">as </span>usize)..((r[<span class="number">1</span>] + <span class="number">1</span>) <span class="kw">as </span>usize)
        );

        <span class="comment">// Compute pessimistic default lower and upper bounds on the category.
        // If character doesn't map to any range and there is no adjacent range
        // in the table slice - these bounds has to apply.
        </span><span class="kw">let </span>lower = idx <span class="kw">as </span>u32 * lookup_interval;
        <span class="kw">let </span>upper = lower + lookup_interval - <span class="number">1</span>;
        bsearch_range_value_table(c, <span class="kw-2">&amp;</span>sentence_cat_table[range], lower, upper)
    }

    <span class="kw">const </span>sentence_cat_lookup: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[u16] = <span class="kw-2">&amp;</span>[
        <span class="number">0</span>, <span class="number">19</span>, <span class="number">31</span>, <span class="number">154</span>, <span class="number">247</span>, <span class="number">314</span>, <span class="number">323</span>, <span class="number">333</span>, <span class="number">375</span>, <span class="number">409</span>, <span class="number">528</span>, <span class="number">579</span>, <span class="number">588</span>, <span class="number">599</span>, <span class="number">612</span>, <span class="number">618</span>, <span class="number">629</span>, <span class="number">643</span>, <span class="number">650</span>,
        <span class="number">661</span>, <span class="number">683</span>, <span class="number">702</span>, <span class="number">720</span>, <span class="number">738</span>, <span class="number">755</span>, <span class="number">771</span>, <span class="number">790</span>, <span class="number">806</span>, <span class="number">818</span>, <span class="number">825</span>, <span class="number">840</span>, <span class="number">850</span>, <span class="number">856</span>, <span class="number">871</span>, <span class="number">882</span>, <span class="number">882</span>, <span class="number">882</span>,
        <span class="number">887</span>, <span class="number">895</span>, <span class="number">901</span>, <span class="number">904</span>, <span class="number">904</span>, <span class="number">904</span>, <span class="number">904</span>, <span class="number">904</span>, <span class="number">907</span>, <span class="number">912</span>, <span class="number">922</span>, <span class="number">929</span>, <span class="number">938</span>, <span class="number">944</span>, <span class="number">951</span>, <span class="number">954</span>, <span class="number">960</span>, <span class="number">965</span>,
        <span class="number">974</span>, <span class="number">981</span>, <span class="number">989</span>, <span class="number">1001</span>, <span class="number">1001</span>, <span class="number">1003</span>, <span class="number">1131</span>, <span class="number">1250</span>, <span class="number">1268</span>, <span class="number">1289</span>, <span class="number">1309</span>, <span class="number">1312</span>, <span class="number">1337</span>, <span class="number">1341</span>, <span class="number">1341</span>, <span class="number">1341</span>,
        <span class="number">1343</span>, <span class="number">1343</span>, <span class="number">1343</span>, <span class="number">1345</span>, <span class="number">1345</span>, <span class="number">1345</span>, <span class="number">1345</span>, <span class="number">1345</span>, <span class="number">1347</span>, <span class="number">1349</span>, <span class="number">1349</span>, <span class="number">1349</span>, <span class="number">1349</span>, <span class="number">1352</span>, <span class="number">1352</span>,
        <span class="number">1352</span>, <span class="number">1352</span>, <span class="number">1352</span>, <span class="number">1370</span>, <span class="number">1477</span>, <span class="number">1483</span>, <span class="number">1493</span>, <span class="number">1502</span>, <span class="number">1502</span>, <span class="number">1502</span>, <span class="number">1502</span>, <span class="number">1513</span>, <span class="number">1518</span>, <span class="number">1519</span>, <span class="number">1522</span>,
        <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>,
        <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>,
        <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>,
        <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1522</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>,
        <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1523</span>, <span class="number">1526</span>, <span class="number">1526</span>, <span class="number">1526</span>, <span class="number">1581</span>, <span class="number">1614</span>, <span class="number">1697</span>, <span class="number">1770</span>, <span class="number">1781</span>, <span class="number">1791</span>, <span class="number">1798</span>, <span class="number">1809</span>,
        <span class="number">1820</span>, <span class="number">1837</span>, <span class="number">1844</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>,
        <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>,
        <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>,
        <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>,
        <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>,
        <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>, <span class="number">1850</span>,
        <span class="number">1850</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>,
        <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>,
        <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>,
        <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>,
        <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1853</span>, <span class="number">1854</span>, <span class="number">1855</span>, <span class="number">1865</span>, <span class="number">1866</span>, <span class="number">1866</span>,
        <span class="number">1866</span>, <span class="number">1868</span>, <span class="number">1871</span>, <span class="number">1887</span>, <span class="number">1889</span>, <span class="number">1906</span>, <span class="number">1914</span>, <span class="number">1920</span>, <span class="number">1921</span>, <span class="number">1922</span>, <span class="number">1923</span>, <span class="number">1923</span>, <span class="number">1926</span>, <span class="number">1930</span>, <span class="number">1934</span>,
        <span class="number">1936</span>, <span class="number">1940</span>, <span class="number">1943</span>, <span class="number">1950</span>, <span class="number">1950</span>, <span class="number">1950</span>, <span class="number">1953</span>, <span class="number">1958</span>, <span class="number">1965</span>, <span class="number">1968</span>, <span class="number">1970</span>, <span class="number">1972</span>, <span class="number">1983</span>, <span class="number">1987</span>, <span class="number">1990</span>,
        <span class="number">1991</span>, <span class="number">1992</span>, <span class="number">1994</span>, <span class="number">1997</span>, <span class="number">1997</span>, <span class="number">1997</span>, <span class="number">2001</span>, <span class="number">2006</span>, <span class="number">2011</span>, <span class="number">2020</span>, <span class="number">2029</span>, <span class="number">2040</span>, <span class="number">2052</span>, <span class="number">2060</span>, <span class="number">2069</span>,
        <span class="number">2087</span>, <span class="number">2087</span>, <span class="number">2094</span>, <span class="number">2099</span>, <span class="number">2099</span>, <span class="number">2106</span>, <span class="number">2111</span>, <span class="number">2115</span>, <span class="number">2120</span>, <span class="number">2120</span>, <span class="number">2122</span>, <span class="number">2125</span>, <span class="number">2140</span>, <span class="number">2147</span>, <span class="number">2157</span>,
        <span class="number">2162</span>, <span class="number">2162</span>, <span class="number">2162</span>, <span class="number">2169</span>, <span class="number">2172</span>, <span class="number">2184</span>, <span class="number">2190</span>, <span class="number">2190</span>, <span class="number">2193</span>, <span class="number">2202</span>, <span class="number">2203</span>, <span class="number">2203</span>, <span class="number">2203</span>, <span class="number">2203</span>, <span class="number">2203</span>,
        <span class="number">2203</span>, <span class="number">2203</span>, <span class="number">2203</span>, <span class="number">2204</span>, <span class="number">2205</span>, <span class="number">2205</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>,
        <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2206</span>, <span class="number">2207</span>, <span class="number">2207</span>, <span class="number">2207</span>,
        <span class="number">2207</span>, <span class="number">2207</span>, <span class="number">2207</span>, <span class="number">2207</span>, <span class="number">2207</span>, <span class="number">2207</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>,
        <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>,
        <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2212</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>,
        <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>,
        <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>,
        <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>,
        <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>,
        <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2213</span>, <span class="number">2217</span>, <span class="number">2222</span>, <span class="number">2229</span>, <span class="number">2230</span>, <span class="number">2230</span>, <span class="number">2230</span>,
        <span class="number">2230</span>, <span class="number">2230</span>, <span class="number">2232</span>, <span class="number">2233</span>, <span class="number">2236</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>,
        <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>,
        <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>,
        <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2243</span>, <span class="number">2244</span>, <span class="number">2244</span>, <span class="number">2244</span>, <span class="number">2244</span>, <span class="number">2244</span>, <span class="number">2244</span>, <span class="number">2244</span>,
        <span class="number">2244</span>, <span class="number">2244</span>, <span class="number">2244</span>, <span class="number">2245</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>,
        <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>,
        <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>,
        <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>,
        <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2246</span>, <span class="number">2249</span>, <span class="number">2249</span>,
        <span class="number">2249</span>, <span class="number">2254</span>, <span class="number">2254</span>, <span class="number">2254</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>,
        <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2255</span>, <span class="number">2257</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>,
        <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>,
        <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>, <span class="number">2262</span>,
        <span class="number">2262</span>, <span class="number">2264</span>, <span class="number">2264</span>, <span class="number">2264</span>, <span class="number">2264</span>, <span class="number">2267</span>, <span class="number">2270</span>, <span class="number">2271</span>, <span class="number">2271</span>, <span class="number">2271</span>, <span class="number">2271</span>, <span class="number">2276</span>, <span class="number">2289</span>, <span class="number">2301</span>, <span class="number">2306</span>,
        <span class="number">2311</span>, <span class="number">2317</span>, <span class="number">2323</span>, <span class="number">2331</span>, <span class="number">2331</span>, <span class="number">2331</span>, <span class="number">2331</span>, <span class="number">2331</span>, <span class="number">2334</span>, <span class="number">2338</span>, <span class="number">2338</span>, <span class="number">2338</span>, <span class="number">2338</span>, <span class="number">2338</span>, <span class="number">2338</span>,
        <span class="number">2338</span>, <span class="number">2338</span>, <span class="number">2338</span>, <span class="number">2342</span>, <span class="number">2342</span>, <span class="number">2348</span>, <span class="number">2349</span>, <span class="number">2354</span>, <span class="number">2354</span>, <span class="number">2354</span>, <span class="number">2359</span>, <span class="number">2359</span>, <span class="number">2359</span>, <span class="number">2359</span>, <span class="number">2362</span>,
        <span class="number">2362</span>, <span class="number">2362</span>, <span class="number">2362</span>, <span class="number">2362</span>, <span class="number">2362</span>, <span class="number">2366</span>, <span class="number">2366</span>, <span class="number">2368</span>, <span class="number">2373</span>, <span class="number">2373</span>, <span class="number">2373</span>, <span class="number">2373</span>, <span class="number">2373</span>, <span class="number">2373</span>, <span class="number">2373</span>,
        <span class="number">2373</span>, <span class="number">2373</span>, <span class="number">2373</span>, <span class="number">2401</span>, <span class="number">2406</span>, <span class="number">2406</span>, <span class="number">2406</span>, <span class="number">2406</span>, <span class="number">2406</span>, <span class="number">2408</span>, <span class="number">2409</span>, <span class="number">2409</span>, <span class="number">2409</span>, <span class="number">2409</span>, <span class="number">2409</span>,
        <span class="number">2409</span>, <span class="number">2409</span>, <span class="number">2409</span>, <span class="number">2409</span>, <span class="number">2410</span>, <span class="number">2410</span>, <span class="number">2410</span>, <span class="number">2410</span>, <span class="number">2410</span>, <span class="number">2410</span>, <span class="number">2410</span>, <span class="number">2410</span>, <span class="number">2410</span>, <span class="number">2410</span>, <span class="number">2410</span>,
        <span class="number">2411</span>, <span class="number">2411</span>, <span class="number">2411</span>, <span class="number">2411</span>, <span class="number">2411</span>, <span class="number">2411</span>, <span class="number">2411</span>, <span class="number">2411
    </span>];

    <span class="kw">const </span>sentence_cat_table: <span class="kw-2">&amp;</span><span class="lifetime">'static </span>[(char, char, SentenceCat)] = <span class="kw-2">&amp;</span>[
        (<span class="string">'\u{9}'</span>, <span class="string">'\u{9}'</span>, SC_Sp), (<span class="string">'\u{a}'</span>, <span class="string">'\u{a}'</span>, SC_LF), (<span class="string">'\u{b}'</span>, <span class="string">'\u{c}'</span>, SC_Sp), (<span class="string">'\u{d}'</span>,
        <span class="string">'\u{d}'</span>, SC_CR), (<span class="string">'\u{20}'</span>, <span class="string">'\u{20}'</span>, SC_Sp), (<span class="string">'\u{21}'</span>, <span class="string">'\u{21}'</span>, SC_STerm), (<span class="string">'\u{22}'</span>,
        <span class="string">'\u{22}'</span>, SC_Close), (<span class="string">'\u{27}'</span>, <span class="string">'\u{29}'</span>, SC_Close), (<span class="string">'\u{2c}'</span>, <span class="string">'\u{2d}'</span>, SC_SContinue),
        (<span class="string">'\u{2e}'</span>, <span class="string">'\u{2e}'</span>, SC_ATerm), (<span class="string">'\u{30}'</span>, <span class="string">'\u{39}'</span>, SC_Numeric), (<span class="string">'\u{3a}'</span>, <span class="string">'\u{3a}'</span>,
        SC_SContinue), (<span class="string">'\u{3f}'</span>, <span class="string">'\u{3f}'</span>, SC_STerm), (<span class="string">'\u{41}'</span>, <span class="string">'\u{5a}'</span>, SC_Upper), (<span class="string">'\u{5b}'</span>,
        <span class="string">'\u{5b}'</span>, SC_Close), (<span class="string">'\u{5d}'</span>, <span class="string">'\u{5d}'</span>, SC_Close), (<span class="string">'\u{61}'</span>, <span class="string">'\u{7a}'</span>, SC_Lower),
        (<span class="string">'\u{7b}'</span>, <span class="string">'\u{7b}'</span>, SC_Close), (<span class="string">'\u{7d}'</span>, <span class="string">'\u{7d}'</span>, SC_Close), (<span class="string">'\u{85}'</span>, <span class="string">'\u{85}'</span>,
        SC_Sep), (<span class="string">'\u{a0}'</span>, <span class="string">'\u{a0}'</span>, SC_Sp), (<span class="string">'\u{aa}'</span>, <span class="string">'\u{aa}'</span>, SC_Lower), (<span class="string">'\u{ab}'</span>, <span class="string">'\u{ab}'</span>,
        SC_Close), (<span class="string">'\u{ad}'</span>, <span class="string">'\u{ad}'</span>, SC_Format), (<span class="string">'\u{b5}'</span>, <span class="string">'\u{b5}'</span>, SC_Lower), (<span class="string">'\u{ba}'</span>,
        <span class="string">'\u{ba}'</span>, SC_Lower), (<span class="string">'\u{bb}'</span>, <span class="string">'\u{bb}'</span>, SC_Close), (<span class="string">'\u{c0}'</span>, <span class="string">'\u{d6}'</span>, SC_Upper),
        (<span class="string">'\u{d8}'</span>, <span class="string">'\u{de}'</span>, SC_Upper), (<span class="string">'\u{df}'</span>, <span class="string">'\u{f6}'</span>, SC_Lower), (<span class="string">'\u{f8}'</span>, <span class="string">'\u{ff}'</span>,
        SC_Lower), (<span class="string">'\u{100}'</span>, <span class="string">'\u{100}'</span>, SC_Upper), (<span class="string">'\u{101}'</span>, <span class="string">'\u{101}'</span>, SC_Lower), (<span class="string">'\u{102}'</span>,
        <span class="string">'\u{102}'</span>, SC_Upper), (<span class="string">'\u{103}'</span>, <span class="string">'\u{103}'</span>, SC_Lower), (<span class="string">'\u{104}'</span>, <span class="string">'\u{104}'</span>, SC_Upper),
        (<span class="string">'\u{105}'</span>, <span class="string">'\u{105}'</span>, SC_Lower), (<span class="string">'\u{106}'</span>, <span class="string">'\u{106}'</span>, SC_Upper), (<span class="string">'\u{107}'</span>, <span class="string">'\u{107}'</span>,
        SC_Lower), (<span class="string">'\u{108}'</span>, <span class="string">'\u{108}'</span>, SC_Upper), (<span class="string">'\u{109}'</span>, <span class="string">'\u{109}'</span>, SC_Lower), (<span class="string">'\u{10a}'</span>,
        <span class="string">'\u{10a}'</span>, SC_Upper), (<span class="string">'\u{10b}'</span>, <span class="string">'\u{10b}'</span>, SC_Lower), (<span class="string">'\u{10c}'</span>, <span class="string">'\u{10c}'</span>, SC_Upper),
        (<span class="string">'\u{10d}'</span>, <span class="string">'\u{10d}'</span>, SC_Lower), (<span class="string">'\u{10e}'</span>, <span class="string">'\u{10e}'</span>, SC_Upper), (<span class="string">'\u{10f}'</span>, <span class="string">'\u{10f}'</span>,
        SC_Lower), (<span class="string">'\u{110}'</span>, <span class="string">'\u{110}'</span>, SC_Upper), (<span class="string">'\u{111}'</span>, <span class="string">'\u{111}'</span>, SC_Lower), (<span class="string">'\u{112}'</span>,
        <span class="string">'\u{112}'</span>, SC_Upper), (<span class="string">'\u{113}'</span>, <span class="string">'\u{113}'</span>, SC_Lower), (<span class="string">'\u{114}'</span>, <span class="string">'\u{114}'</span>, SC_Upper),
        (<span class="string">'\u{115}'</span>, <span class="string">'\u{115}'</span>, SC_Lower), (<span class="string">'\u{116}'</span>, <span class="string">'\u{116}'</span>, SC_Upper), (<span class="string">'\u{117}'</span>, <span class="string">'\u{117}'</span>,
        SC_Lower), (<span class="string">'\u{118}'</span>, <span class="string">'\u{118}'</span>, SC_Upper), (<span class="string">'\u{119}'</span>, <span class="string">'\u{119}'</span>, SC_Lower), (<span class="string">'\u{11a}'</span>,
        <span class="string">'\u{11a}'</span>, SC_Upper), (<span class="string">'\u{11b}'</span>, <span class="string">'\u{11b}'</span>, SC_Lower), (<span class="string">'\u{11c}'</span>, <span class="string">'\u{11c}'</span>, SC_Upper),
        (<span class="string">'\u{11d}'</span>, <span class="string">'\u{11d}'</span>, SC_Lower), (<span class="string">'\u{11e}'</span>, <span class="string">'\u{11e}'</span>, SC_Upper), (<span class="string">'\u{11f}'</span>, <span class="string">'\u{11f}'</span>,
        SC_Lower), (<span class="string">'\u{120}'</span>, <span class="string">'\u{120}'</span>, SC_Upper), (<span class="string">'\u{121}'</span>, <span class="string">'\u{121}'</span>, SC_Lower), (<span class="string">'\u{122}'</span>,
        <span class="string">'\u{122}'</span>, SC_Upper), (<span class="string">'\u{123}'</span>, <span class="string">'\u{123}'</span>, SC_Lower), (<span class="string">'\u{124}'</span>, <span class="string">'\u{124}'</span>, SC_Upper),
        (<span class="string">'\u{125}'</span>, <span class="string">'\u{125}'</span>, SC_Lower), (<span class="string">'\u{126}'</span>, <span class="string">'\u{126}'</span>, SC_Upper), (<span class="string">'\u{127}'</span>, <span class="string">'\u{127}'</span>,
        SC_Lower), (<span class="string">'\u{128}'</span>, <span class="string">'\u{128}'</span>, SC_Upper), (<span class="string">'\u{129}'</span>, <span class="string">'\u{129}'</span>, SC_Lower), (<span class="string">'\u{12a}'</span>,
        <span class="string">'\u{12a}'</span>, SC_Upper), (<span class="string">'\u{12b}'</span>, <span class="string">'\u{12b}'</span>, SC_Lower), (<span class="string">'\u{12c}'</span>, <span class="string">'\u{12c}'</span>, SC_Upper),
        (<span class="string">'\u{12d}'</span>, <span class="string">'\u{12d}'</span>, SC_Lower), (<span class="string">'\u{12e}'</span>, <span class="string">'\u{12e}'</span>, SC_Upper), (<span class="string">'\u{12f}'</span>, <span class="string">'\u{12f}'</span>,
        SC_Lower), (<span class="string">'\u{130}'</span>, <span class="string">'\u{130}'</span>, SC_Upper), (<span class="string">'\u{131}'</span>, <span class="string">'\u{131}'</span>, SC_Lower), (<span class="string">'\u{132}'</span>,
        <span class="string">'\u{132}'</span>, SC_Upper), (<span class="string">'\u{133}'</span>, <span class="string">'\u{133}'</span>, SC_Lower), (<span class="string">'\u{134}'</span>, <span class="string">'\u{134}'</span>, SC_Upper),
        (<span class="string">'\u{135}'</span>, <span class="string">'\u{135}'</span>, SC_Lower), (<span class="string">'\u{136}'</span>, <span class="string">'\u{136}'</span>, SC_Upper), (<span class="string">'\u{137}'</span>, <span class="string">'\u{138}'</span>,
        SC_Lower), (<span class="string">'\u{139}'</span>, <span class="string">'\u{139}'</span>, SC_Upper), (<span class="string">'\u{13a}'</span>, <span class="string">'\u{13a}'</span>, SC_Lower), (<span class="string">'\u{13b}'</span>,
        <span class="string">'\u{13b}'</span>, SC_Upper), (<span class="string">'\u{13c}'</span>, <span class="string">'\u{13c}'</span>, SC_Lower), (<span class="string">'\u{13d}'</span>, <span class="string">'\u{13d}'</span>, SC_Upper),
        (<span class="string">'\u{13e}'</span>, <span class="string">'\u{13e}'</span>, SC_Lower), (<span class="string">'\u{13f}'</span>, <span class="string">'\u{13f}'</span>, SC_Upper), (<span class="string">'\u{140}'</span>, <span class="string">'\u{140}'</span>,
        SC_Lower), (<span class="string">'\u{141}'</span>, <span class="string">'\u{141}'</span>, SC_Upper), (<span class="string">'\u{142}'</span>, <span class="string">'\u{142}'</span>, SC_Lower), (<span class="string">'\u{143}'</span>,
        <span class="string">'\u{143}'</span>, SC_Upper), (<span class="string">'\u{144}'</span>, <span class="string">'\u{144}'</span>, SC_Lower), (<span class="string">'\u{145}'</span>, <span class="string">'\u{145}'</span>, SC_Upper),
        (<span class="string">'\u{146}'</span>, <span class="string">'\u{146}'</span>, SC_Lower), (<span class="string">'\u{147}'</span>, <span class="string">'\u{147}'</span>, SC_Upper), (<span class="string">'\u{148}'</span>, <span class="string">'\u{149}'</span>,
        SC_Lower), (<span class="string">'\u{14a}'</span>, <span class="string">'\u{14a}'</span>, SC_Upper), (<span class="string">'\u{14b}'</span>, <span class="string">'\u{14b}'</span>, SC_Lower), (<span class="string">'\u{14c}'</span>,
        <span class="string">'\u{14c}'</span>, SC_Upper), (<span class="string">'\u{14d}'</span>, <span class="string">'\u{14d}'</span>, SC_Lower), (<span class="string">'\u{14e}'</span>, <span class="string">'\u{14e}'</span>, SC_Upper),
        (<span class="string">'\u{14f}'</span>, <span class="string">'\u{14f}'</span>, SC_Lower), (<span class="string">'\u{150}'</span>, <span class="string">'\u{150}'</span>, SC_Upper), (<span class="string">'\u{151}'</span>, <span class="string">'\u{151}'</span>,
        SC_Lower), (<span class="string">'\u{152}'</span>, <span class="string">'\u{152}'</span>, SC_Upper), (<span class="string">'\u{153}'</span>, <span class="string">'\u{153}'</span>, SC_Lower), (<span class="string">'\u{154}'</span>,
        <span class="string">'\u{154}'</span>, SC_Upper), (<span class="string">'\u{155}'</span>, <span class="string">'\u{155}'</span>, SC_Lower), (<span class="string">'\u{156}'</span>, <span class="string">'\u{156}'</span>, SC_Upper),
        (<span class="string">'\u{157}'</span>, <span class="string">'\u{157}'</span>, SC_Lower), (<span class="string">'\u{158}'</span>, <span class="string">'\u{158}'</span>, SC_Upper), (<span class="string">'\u{159}'</span>, <span class="string">'\u{159}'</span>,
        SC_Lower), (<span class="string">'\u{15a}'</span>, <span class="string">'\u{15a}'</span>, SC_Upper), (<span class="string">'\u{15b}'</span>, <span class="string">'\u{15b}'</span>, SC_Lower), (<span class="string">'\u{15c}'</span>,
        <span class="string">'\u{15c}'</span>, SC_Upper), (<span class="string">'\u{15d}'</span>, <span class="string">'\u{15d}'</span>, SC_Lower), (<span class="string">'\u{15e}'</span>, <span class="string">'\u{15e}'</span>, SC_Upper),
        (<span class="string">'\u{15f}'</span>, <span class="string">'\u{15f}'</span>, SC_Lower), (<span class="string">'\u{160}'</span>, <span class="string">'\u{160}'</span>, SC_Upper), (<span class="string">'\u{161}'</span>, <span class="string">'\u{161}'</span>,
        SC_Lower), (<span class="string">'\u{162}'</span>, <span class="string">'\u{162}'</span>, SC_Upper), (<span class="string">'\u{163}'</span>, <span class="string">'\u{163}'</span>, SC_Lower), (<span class="string">'\u{164}'</span>,
        <span class="string">'\u{164}'</span>, SC_Upper), (<span class="string">'\u{165}'</span>, <span class="string">'\u{165}'</span>, SC_Lower), (<span class="string">'\u{166}'</span>, <span class="string">'\u{166}'</span>, SC_Upper),
        (<span class="string">'\u{167}'</span>, <span class="string">'\u{167}'</span>, SC_Lower), (<span class="string">'\u{168}'</span>, <span class="string">'\u{168}'</span>, SC_Upper), (<span class="string">'\u{169}'</span>, <span class="string">'\u{169}'</span>,
        SC_Lower), (<span class="string">'\u{16a}'</span>, <span class="string">'\u{16a}'</span>, SC_Upper), (<span class="string">'\u{16b}'</span>, <span class="string">'\u{16b}'</span>, SC_Lower), (<span class="string">'\u{16c}'</span>,
        <span class="string">'\u{16c}'</span>, SC_Upper), (<span class="string">'\u{16d}'</span>, <span class="string">'\u{16d}'</span>, SC_Lower), (<span class="string">'\u{16e}'</span>, <span class="string">'\u{16e}'</span>, SC_Upper),
        (<span class="string">'\u{16f}'</span>, <span class="string">'\u{16f}'</span>, SC_Lower), (<span class="string">'\u{170}'</span>, <span class="string">'\u{170}'</span>, SC_Upper), (<span class="string">'\u{171}'</span>, <span class="string">'\u{171}'</span>,
        SC_Lower), (<span class="string">'\u{172}'</span>, <span class="string">'\u{172}'</span>, SC_Upper), (<span class="string">'\u{173}'</span>, <span class="string">'\u{173}'</span>, SC_Lower), (<span class="string">'\u{174}'</span>,
        <span class="string">'\u{174}'</span>, SC_Upper), (<span class="string">'\u{175}'</span>, <span class="string">'\u{175}'</span>, SC_Lower), (<span class="string">'\u{176}'</span>, <span class="string">'\u{176}'</span>, SC_Upper),
        (<span class="string">'\u{177}'</span>, <span class="string">'\u{177}'</span>, SC_Lower), (<span class="string">'\u{178}'</span>, <span class="string">'\u{179}'</span>, SC_Upper), (<span class="string">'\u{17a}'</span>, <span class="string">'\u{17a}'</span>,
        SC_Lower), (<span class="string">'\u{17b}'</span>, <span class="string">'\u{17b}'</span>, SC_Upper), (<span class="string">'\u{17c}'</span>, <span class="string">'\u{17c}'</span>, SC_Lower), (<span class="string">'\u{17d}'</span>,
        <span class="string">'\u{17d}'</span>, SC_Upper), (<span class="string">'\u{17e}'</span>, <span class="string">'\u{180}'</span>, SC_Lower), (<span class="string">'\u{181}'</span>, <span class="string">'\u{182}'</span>, SC_Upper),
        (<span class="string">'\u{183}'</span>, <span class="string">'\u{183}'</span>, SC_Lower), (<span class="string">'\u{184}'</span>, <span class="string">'\u{184}'</span>, SC_Upper), (<span class="string">'\u{185}'</span>, <span class="string">'\u{185}'</span>,
        SC_Lower), (<span class="string">'\u{186}'</span>, <span class="string">'\u{187}'</span>, SC_Upper), (<span class="string">'\u{188}'</span>, <span class="string">'\u{188}'</span>, SC_Lower), (<span class="string">'\u{189}'</span>,
        <span class="string">'\u{18b}'</span>, SC_Upper), (<span class="string">'\u{18c}'</span>, <span class="string">'\u{18d}'</span>, SC_Lower), (<span class="string">'\u{18e}'</span>, <span class="string">'\u{191}'</span>, SC_Upper),
        (<span class="string">'\u{192}'</span>, <span class="string">'\u{192}'</span>, SC_Lower), (<span class="string">'\u{193}'</span>, <span class="string">'\u{194}'</span>, SC_Upper), (<span class="string">'\u{195}'</span>, <span class="string">'\u{195}'</span>,
        SC_Lower), (<span class="string">'\u{196}'</span>, <span class="string">'\u{198}'</span>, SC_Upper), (<span class="string">'\u{199}'</span>, <span class="string">'\u{19b}'</span>, SC_Lower), (<span class="string">'\u{19c}'</span>,
        <span class="string">'\u{19d}'</span>, SC_Upper), (<span class="string">'\u{19e}'</span>, <span class="string">'\u{19e}'</span>, SC_Lower), (<span class="string">'\u{19f}'</span>, <span class="string">'\u{1a0}'</span>, SC_Upper),
        (<span class="string">'\u{1a1}'</span>, <span class="string">'\u{1a1}'</span>, SC_Lower), (<span class="string">'\u{1a2}'</span>, <span class="string">'\u{1a2}'</span>, SC_Upper), (<span class="string">'\u{1a3}'</span>, <span class="string">'\u{1a3}'</span>,
        SC_Lower), (<span class="string">'\u{1a4}'</span>, <span class="string">'\u{1a4}'</span>, SC_Upper), (<span class="string">'\u{1a5}'</span>, <span class="string">'\u{1a5}'</span>, SC_Lower), (<span class="string">'\u{1a6}'</span>,
        <span class="string">'\u{1a7}'</span>, SC_Upper), (<span class="string">'\u{1a8}'</span>, <span class="string">'\u{1a8}'</span>, SC_Lower), (<span class="string">'\u{1a9}'</span>, <span class="string">'\u{1a9}'</span>, SC_Upper),
        (<span class="string">'\u{1aa}'</span>, <span class="string">'\u{1ab}'</span>, SC_Lower), (<span class="string">'\u{1ac}'</span>, <span class="string">'\u{1ac}'</span>, SC_Upper), (<span class="string">'\u{1ad}'</span>, <span class="string">'\u{1ad}'</span>,
        SC_Lower), (<span class="string">'\u{1ae}'</span>, <span class="string">'\u{1af}'</span>, SC_Upper), (<span class="string">'\u{1b0}'</span>, <span class="string">'\u{1b0}'</span>, SC_Lower), (<span class="string">'\u{1b1}'</span>,
        <span class="string">'\u{1b3}'</span>, SC_Upper), (<span class="string">'\u{1b4}'</span>, <span class="string">'\u{1b4}'</span>, SC_Lower), (<span class="string">'\u{1b5}'</span>, <span class="string">'\u{1b5}'</span>, SC_Upper),
        (<span class="string">'\u{1b6}'</span>, <span class="string">'\u{1b6}'</span>, SC_Lower), (<span class="string">'\u{1b7}'</span>, <span class="string">'\u{1b8}'</span>, SC_Upper), (<span class="string">'\u{1b9}'</span>, <span class="string">'\u{1ba}'</span>,
        SC_Lower), (<span class="string">'\u{1bb}'</span>, <span class="string">'\u{1bb}'</span>, SC_OLetter), (<span class="string">'\u{1bc}'</span>, <span class="string">'\u{1bc}'</span>, SC_Upper), (<span class="string">'\u{1bd}'</span>,
        <span class="string">'\u{1bf}'</span>, SC_Lower), (<span class="string">'\u{1c0}'</span>, <span class="string">'\u{1c3}'</span>, SC_OLetter), (<span class="string">'\u{1c4}'</span>, <span class="string">'\u{1c5}'</span>, SC_Upper),
        (<span class="string">'\u{1c6}'</span>, <span class="string">'\u{1c6}'</span>, SC_Lower), (<span class="string">'\u{1c7}'</span>, <span class="string">'\u{1c8}'</span>, SC_Upper), (<span class="string">'\u{1c9}'</span>, <span class="string">'\u{1c9}'</span>,
        SC_Lower), (<span class="string">'\u{1ca}'</span>, <span class="string">'\u{1cb}'</span>, SC_Upper), (<span class="string">'\u{1cc}'</span>, <span class="string">'\u{1cc}'</span>, SC_Lower), (<span class="string">'\u{1cd}'</span>,
        <span class="string">'\u{1cd}'</span>, SC_Upper), (<span class="string">'\u{1ce}'</span>, <span class="string">'\u{1ce}'</span>, SC_Lower), (<span class="string">'\u{1cf}'</span>, <span class="string">'\u{1cf}'</span>, SC_Upper),
        (<span class="string">'\u{1d0}'</span>, <span class="string">'\u{1d0}'</span>, SC_Lower), (<span class="string">'\u{1d1}'</span>, <span class="string">'\u{1d1}'</span>, SC_Upper), (<span class="string">'\u{1d2}'</span>, <span class="string">'\u{1d2}'</span>,
        SC_Lower), (<span class="string">'\u{1d3}'</span>, <span class="string">'\u{1d3}'</span>, SC_Upper), (<span class="string">'\u{1d4}'</span>, <span class="string">'\u{1d4}'</span>, SC_Lower), (<span class="string">'\u{1d5}'</span>,
        <span class="string">'\u{1d5}'</span>, SC_Upper), (<span class="string">'\u{1d6}'</span>, <span class="string">'\u{1d6}'</span>, SC_Lower), (<span class="string">'\u{1d7}'</span>, <span class="string">'\u{1d7}'</span>, SC_Upper),
        (<span class="string">'\u{1d8}'</span>, <span class="string">'\u{1d8}'</span>, SC_Lower), (<span class="string">'\u{1d9}'</span>, <span class="string">'\u{1d9}'</span>, SC_Upper), (<span class="string">'\u{1da}'</span>, <span class="string">'\u{1da}'</span>,
        SC_Lower), (<span class="string">'\u{1db}'</span>, <span class="string">'\u{1db}'</span>, SC_Upper), (<span class="string">'\u{1dc}'</span>, <span class="string">'\u{1dd}'</span>, SC_Lower), (<span class="string">'\u{1de}'</span>,
        <span class="string">'\u{1de}'</span>, SC_Upper), (<span class="string">'\u{1df}'</span>, <span class="string">'\u{1df}'</span>, SC_Lower), (<span class="string">'\u{1e0}'</span>, <span class="string">'\u{1e0}'</span>, SC_Upper),
        (<span class="string">'\u{1e1}'</span>, <span class="string">'\u{1e1}'</span>, SC_Lower), (<span class="string">'\u{1e2}'</span>, <span class="string">'\u{1e2}'</span>, SC_Upper), (<span class="string">'\u{1e3}'</span>, <span class="string">'\u{1e3}'</span>,
        SC_Lower), (<span class="string">'\u{1e4}'</span>, <span class="string">'\u{1e4}'</span>, SC_Upper), (<span class="string">'\u{1e5}'</span>, <span class="string">'\u{1e5}'</span>, SC_Lower), (<span class="string">'\u{1e6}'</span>,
        <span class="string">'\u{1e6}'</span>, SC_Upper), (<span class="string">'\u{1e7}'</span>, <span class="string">'\u{1e7}'</span>, SC_Lower), (<span class="string">'\u{1e8}'</span>, <span class="string">'\u{1e8}'</span>, SC_Upper),
        (<span class="string">'\u{1e9}'</span>, <span class="string">'\u{1e9}'</span>, SC_Lower), (<span class="string">'\u{1ea}'</span>, <span class="string">'\u{1ea}'</span>, SC_Upper), (<span class="string">'\u{1eb}'</span>, <span class="string">'\u{1eb}'</span>,
        SC_Lower), (<span class="string">'\u{1ec}'</span>, <span class="string">'\u{1ec}'</span>, SC_Upper), (<span class="string">'\u{1ed}'</span>, <span class="string">'\u{1ed}'</span>, SC_Lower), (<span class="string">'\u{1ee}'</span>,
        <span class="string">'\u{1ee}'</span>, SC_Upper), (<span class="string">'\u{1ef}'</span>, <span class="string">'\u{1f0}'</span>, SC_Lower), (<span class="string">'\u{1f1}'</span>, <span class="string">'\u{1f2}'</span>, SC_Upper),
        (<span class="string">'\u{1f3}'</span>, <span class="string">'\u{1f3}'</span>, SC_Lower), (<span class="string">'\u{1f4}'</span>, <span class="string">'\u{1f4}'</span>, SC_Upper), (<span class="string">'\u{1f5}'</span>, <span class="string">'\u{1f5}'</span>,
        SC_Lower), (<span class="string">'\u{1f6}'</span>, <span class="string">'\u{1f8}'</span>, SC_Upper), (<span class="string">'\u{1f9}'</span>, <span class="string">'\u{1f9}'</span>, SC_Lower), (<span class="string">'\u{1fa}'</span>,
        <span class="string">'\u{1fa}'</span>, SC_Upper), (<span class="string">'\u{1fb}'</span>, <span class="string">'\u{1fb}'</span>, SC_Lower), (<span class="string">'\u{1fc}'</span>, <span class="string">'\u{1fc}'</span>, SC_Upper),
        (<span class="string">'\u{1fd}'</span>, <span class="string">'\u{1fd}'</span>, SC_Lower), (<span class="string">'\u{1fe}'</span>, <span class="string">'\u{1fe}'</span>, SC_Upper), (<span class="string">'\u{1ff}'</span>, <span class="string">'\u{1ff}'</span>,
        SC_Lower), (<span class="string">'\u{200}'</span>, <span class="string">'\u{200}'</span>, SC_Upper), (<span class="string">'\u{201}'</span>, <span class="string">'\u{201}'</span>, SC_Lower), (<span class="string">'\u{202}'</span>,
        <span class="string">'\u{202}'</span>, SC_Upper), (<span class="string">'\u{203}'</span>, <span class="string">'\u{203}'</span>, SC_Lower), (<span class="string">'\u{204}'</span>, <span class="string">'\u{204}'</span>, SC_Upper),
        (<span class="string">'\u{205}'</span>, <span class="string">'\u{205}'</span>, SC_Lower), (<span class="string">'\u{206}'</span>, <span class="string">'\u{206}'</span>, SC_Upper), (<span class="string">'\u{207}'</span>, <span class="string">'\u{207}'</span>,
        SC_Lower), (<span class="string">'\u{208}'</span>, <span class="string">'\u{208}'</span>, SC_Upper), (<span class="string">'\u{209}'</span>, <span class="string">'\u{209}'</span>, SC_Lower), (<span class="string">'\u{20a}'</span>,
        <span class="string">'\u{20a}'</span>, SC_Upper), (<span class="string">'\u{20b}'</span>, <span class="string">'\u{20b}'</span>, SC_Lower), (<span class="string">'\u{20c}'</span>, <span class="string">'\u{20c}'</span>, SC_Upper),
        (<span class="string">'\u{20d}'</span>, <span class="string">'\u{20d}'</span>, SC_Lower), (<span class="string">'\u{20e}'</span>, <span class="string">'\u{20e}'</span>, SC_Upper), (<span class="string">'\u{20f}'</span>, <span class="string">'\u{20f}'</span>,
        SC_Lower), (<span class="string">'\u{210}'</span>, <span class="string">'\u{210}'</span>, SC_Upper), (<span class="string">'\u{211}'</span>, <span class="string">'\u{211}'</span>, SC_Lower), (<span class="string">'\u{212}'</span>,
        <span class="string">'\u{212}'</span>, SC_Upper), (<span class="string">'\u{213}'</span>, <span class="string">'\u{213}'</span>, SC_Lower), (<span class="string">'\u{214}'</span>, <span class="string">'\u{214}'</span>, SC_Upper),
        (<span class="string">'\u{215}'</span>, <span class="string">'\u{215}'</span>, SC_Lower), (<span class="string">'\u{216}'</span>, <span class="string">'\u{216}'</span>, SC_Upper), (<span class="string">'\u{217}'</span>, <span class="string">'\u{217}'</span>,
        SC_Lower), (<span class="string">'\u{218}'</span>, <span class="string">'\u{218}'</span>, SC_Upper), (<span class="string">'\u{219}'</span>, <span class="string">'\u{219}'</span>, SC_Lower), (<span class="string">'\u{21a}'</span>,
        <span class="string">'\u{21a}'</span>, SC_Upper), (<span class="string">'\u{21b}'</span>, <span class="string">'\u{21b}'</span>, SC_Lower), (<span class="string">'\u{21c}'</span>, <span class="string">'\u{21c}'</span>, SC_Upper),
        (<span class="string">'\u{21d}'</span>, <span class="string">'\u{21d}'</span>, SC_Lower), (<span class="string">'\u{21e}'</span>, <span class="string">'\u{21e}'</span>, SC_Upper), (<span class="string">'\u{21f}'</span>, <span class="string">'\u{21f}'</span>,
        SC_Lower), (<span class="string">'\u{220}'</span>, <span class="string">'\u{220}'</span>, SC_Upper), (<span class="string">'\u{221}'</span>, <span class="string">'\u{221}'</span>, SC_Lower), (<span class="string">'\u{222}'</span>,
        <span class="string">'\u{222}'</span>, SC_Upper), (<span class="string">'\u{223}'</span>, <span class="string">'\u{223}'</span>, SC_Lower), (<span class="string">'\u{224}'</span>, <span class="string">'\u{224}'</span>, SC_Upper),
        (<span class="string">'\u{225}'</span>, <span class="string">'\u{225}'</span>, SC_Lower), (<span class="string">'\u{226}'</span>, <span class="string">'\u{226}'</span>, SC_Upper), (<span class="string">'\u{227}'</span>, <span class="string">'\u{227}'</span>,
        SC_Lower), (<span class="string">'\u{228}'</span>, <span class="string">'\u{228}'</span>, SC_Upper), (<span class="string">'\u{229}'</span>, <span class="string">'\u{229}'</span>, SC_Lower), (<span class="string">'\u{22a}'</span>,
        <span class="string">'\u{22a}'</span>, SC_Upper), (<span class="string">'\u{22b}'</span>, <span class="string">'\u{22b}'</span>, SC_Lower), (<span class="string">'\u{22c}'</span>, <span class="string">'\u{22c}'</span>, SC_Upper),
        (<span class="string">'\u{22d}'</span>, <span class="string">'\u{22d}'</span>, SC_Lower), (<span class="string">'\u{22e}'</span>, <span class="string">'\u{22e}'</span>, SC_Upper), (<span class="string">'\u{22f}'</span>, <span class="string">'\u{22f}'</span>,
        SC_Lower), (<span class="string">'\u{230}'</span>, <span class="string">'\u{230}'</span>, SC_Upper), (<span class="string">'\u{231}'</span>, <span class="string">'\u{231}'</span>, SC_Lower), (<span class="string">'\u{232}'</span>,
        <span class="string">'\u{232}'</span>, SC_Upper), (<span class="string">'\u{233}'</span>, <span class="string">'\u{239}'</span>, SC_Lower), (<span class="string">'\u{23a}'</span>, <span class="string">'\u{23b}'</span>, SC_Upper),
        (<span class="string">'\u{23c}'</span>, <span class="string">'\u{23c}'</span>, SC_Lower), (<span class="string">'\u{23d}'</span>, <span class="string">'\u{23e}'</span>, SC_Upper), (<span class="string">'\u{23f}'</span>, <span class="string">'\u{240}'</span>,
        SC_Lower), (<span class="string">'\u{241}'</span>, <span class="string">'\u{241}'</span>, SC_Upper), (<span class="string">'\u{242}'</span>, <span class="string">'\u{242}'</span>, SC_Lower), (<span class="string">'\u{243}'</span>,
        <span class="string">'\u{246}'</span>, SC_Upper), (<span class="string">'\u{247}'</span>, <span class="string">'\u{247}'</span>, SC_Lower), (<span class="string">'\u{248}'</span>, <span class="string">'\u{248}'</span>, SC_Upper),
        (<span class="string">'\u{249}'</span>, <span class="string">'\u{249}'</span>, SC_Lower), (<span class="string">'\u{24a}'</span>, <span class="string">'\u{24a}'</span>, SC_Upper), (<span class="string">'\u{24b}'</span>, <span class="string">'\u{24b}'</span>,
        SC_Lower), (<span class="string">'\u{24c}'</span>, <span class="string">'\u{24c}'</span>, SC_Upper), (<span class="string">'\u{24d}'</span>, <span class="string">'\u{24d}'</span>, SC_Lower), (<span class="string">'\u{24e}'</span>,
        <span class="string">'\u{24e}'</span>, SC_Upper), (<span class="string">'\u{24f}'</span>, <span class="string">'\u{293}'</span>, SC_Lower), (<span class="string">'\u{294}'</span>, <span class="string">'\u{294}'</span>, SC_OLetter),
        (<span class="string">'\u{295}'</span>, <span class="string">'\u{2b8}'</span>, SC_Lower), (<span class="string">'\u{2b9}'</span>, <span class="string">'\u{2bf}'</span>, SC_OLetter), (<span class="string">'\u{2c0}'</span>, <span class="string">'\u{2c1}'</span>,
        SC_Lower), (<span class="string">'\u{2c6}'</span>, <span class="string">'\u{2d1}'</span>, SC_OLetter), (<span class="string">'\u{2e0}'</span>, <span class="string">'\u{2e4}'</span>, SC_Lower), (<span class="string">'\u{2ec}'</span>,
        <span class="string">'\u{2ec}'</span>, SC_OLetter), (<span class="string">'\u{2ee}'</span>, <span class="string">'\u{2ee}'</span>, SC_OLetter), (<span class="string">'\u{300}'</span>, <span class="string">'\u{36f}'</span>,
        SC_Extend), (<span class="string">'\u{370}'</span>, <span class="string">'\u{370}'</span>, SC_Upper), (<span class="string">'\u{371}'</span>, <span class="string">'\u{371}'</span>, SC_Lower), (<span class="string">'\u{372}'</span>,
        <span class="string">'\u{372}'</span>, SC_Upper), (<span class="string">'\u{373}'</span>, <span class="string">'\u{373}'</span>, SC_Lower), (<span class="string">'\u{374}'</span>, <span class="string">'\u{374}'</span>, SC_OLetter),
        (<span class="string">'\u{376}'</span>, <span class="string">'\u{376}'</span>, SC_Upper), (<span class="string">'\u{377}'</span>, <span class="string">'\u{377}'</span>, SC_Lower), (<span class="string">'\u{37a}'</span>, <span class="string">'\u{37d}'</span>,
        SC_Lower), (<span class="string">'\u{37f}'</span>, <span class="string">'\u{37f}'</span>, SC_Upper), (<span class="string">'\u{386}'</span>, <span class="string">'\u{386}'</span>, SC_Upper), (<span class="string">'\u{388}'</span>,
        <span class="string">'\u{38a}'</span>, SC_Upper), (<span class="string">'\u{38c}'</span>, <span class="string">'\u{38c}'</span>, SC_Upper), (<span class="string">'\u{38e}'</span>, <span class="string">'\u{38f}'</span>, SC_Upper),
        (<span class="string">'\u{390}'</span>, <span class="string">'\u{390}'</span>, SC_Lower), (<span class="string">'\u{391}'</span>, <span class="string">'\u{3a1}'</span>, SC_Upper), (<span class="string">'\u{3a3}'</span>, <span class="string">'\u{3ab}'</span>,
        SC_Upper), (<span class="string">'\u{3ac}'</span>, <span class="string">'\u{3ce}'</span>, SC_Lower), (<span class="string">'\u{3cf}'</span>, <span class="string">'\u{3cf}'</span>, SC_Upper), (<span class="string">'\u{3d0}'</span>,
        <span class="string">'\u{3d1}'</span>, SC_Lower), (<span class="string">'\u{3d2}'</span>, <span class="string">'\u{3d4}'</span>, SC_Upper), (<span class="string">'\u{3d5}'</span>, <span class="string">'\u{3d7}'</span>, SC_Lower),
        (<span class="string">'\u{3d8}'</span>, <span class="string">'\u{3d8}'</span>, SC_Upper), (<span class="string">'\u{3d9}'</span>, <span class="string">'\u{3d9}'</span>, SC_Lower), (<span class="string">'\u{3da}'</span>, <span class="string">'\u{3da}'</span>,
        SC_Upper), (<span class="string">'\u{3db}'</span>, <span class="string">'\u{3db}'</span>, SC_Lower), (<span class="string">'\u{3dc}'</span>, <span class="string">'\u{3dc}'</span>, SC_Upper), (<span class="string">'\u{3dd}'</span>,
        <span class="string">'\u{3dd}'</span>, SC_Lower), (<span class="string">'\u{3de}'</span>, <span class="string">'\u{3de}'</span>, SC_Upper), (<span class="string">'\u{3df}'</span>, <span class="string">'\u{3df}'</span>, SC_Lower),
        (<span class="string">'\u{3e0}'</span>, <span class="string">'\u{3e0}'</span>, SC_Upper), (<span class="string">'\u{3e1}'</span>, <span class="string">'\u{3e1}'</span>, SC_Lower), (<span class="string">'\u{3e2}'</span>, <span class="string">'\u{3e2}'</span>,
        SC_Upper), (<span class="string">'\u{3e3}'</span>, <span class="string">'\u{3e3}'</span>, SC_Lower), (<span class="string">'\u{3e4}'</span>, <span class="string">'\u{3e4}'</span>, SC_Upper), (<span class="string">'\u{3e5}'</span>,
        <span class="string">'\u{3e5}'</span>, SC_Lower), (<span class="string">'\u{3e6}'</span>, <span class="string">'\u{3e6}'</span>, SC_Upper), (<span class="string">'\u{3e7}'</span>, <span class="string">'\u{3e7}'</span>, SC_Lower),
        (<span class="string">'\u{3e8}'</span>, <span class="string">'\u{3e8}'</span>, SC_Upper), (<span class="string">'\u{3e9}'</span>, <span class="string">'\u{3e9}'</span>, SC_Lower), (<span class="string">'\u{3ea}'</span>, <span class="string">'\u{3ea}'</span>,
        SC_Upper), (<span class="string">'\u{3eb}'</span>, <span class="string">'\u{3eb}'</span>, SC_Lower), (<span class="string">'\u{3ec}'</span>, <span class="string">'\u{3ec}'</span>, SC_Upper), (<span class="string">'\u{3ed}'</span>,
        <span class="string">'\u{3ed}'</span>, SC_Lower), (<span class="string">'\u{3ee}'</span>, <span class="string">'\u{3ee}'</span>, SC_Upper), (<span class="string">'\u{3ef}'</span>, <span class="string">'\u{3f3}'</span>, SC_Lower),
        (<span class="string">'\u{3f4}'</span>, <span class="string">'\u{3f4}'</span>, SC_Upper), (<span class="string">'\u{3f5}'</span>, <span class="string">'\u{3f5}'</span>, SC_Lower), (<span class="string">'\u{3f7}'</span>, <span class="string">'\u{3f7}'</span>,
        SC_Upper), (<span class="string">'\u{3f8}'</span>, <span class="string">'\u{3f8}'</span>, SC_Lower), (<span class="string">'\u{3f9}'</span>, <span class="string">'\u{3fa}'</span>, SC_Upper), (<span class="string">'\u{3fb}'</span>,
        <span class="string">'\u{3fc}'</span>, SC_Lower), (<span class="string">'\u{3fd}'</span>, <span class="string">'\u{42f}'</span>, SC_Upper), (<span class="string">'\u{430}'</span>, <span class="string">'\u{45f}'</span>, SC_Lower),
        (<span class="string">'\u{460}'</span>, <span class="string">'\u{460}'</span>, SC_Upper), (<span class="string">'\u{461}'</span>, <span class="string">'\u{461}'</span>, SC_Lower), (<span class="string">'\u{462}'</span>, <span class="string">'\u{462}'</span>,
        SC_Upper), (<span class="string">'\u{463}'</span>, <span class="string">'\u{463}'</span>, SC_Lower), (<span class="string">'\u{464}'</span>, <span class="string">'\u{464}'</span>, SC_Upper), (<span class="string">'\u{465}'</span>,
        <span class="string">'\u{465}'</span>, SC_Lower), (<span class="string">'\u{466}'</span>, <span class="string">'\u{466}'</span>, SC_Upper), (<span class="string">'\u{467}'</span>, <span class="string">'\u{467}'</span>, SC_Lower),
        (<span class="string">'\u{468}'</span>, <span class="string">'\u{468}'</span>, SC_Upper), (<span class="string">'\u{469}'</span>, <span class="string">'\u{469}'</span>, SC_Lower), (<span class="string">'\u{46a}'</span>, <span class="string">'\u{46a}'</span>,
        SC_Upper), (<span class="string">'\u{46b}'</span>, <span class="string">'\u{46b}'</span>, SC_Lower), (<span class="string">'\u{46c}'</span>, <span class="string">'\u{46c}'</span>, SC_Upper), (<span class="string">'\u{46d}'</span>,
        <span class="string">'\u{46d}'</span>, SC_Lower), (<span class="string">'\u{46e}'</span>, <span class="string">'\u{46e}'</span>, SC_Upper), (<span class="string">'\u{46f}'</span>, <span class="string">'\u{46f}'</span>, SC_Lower),
        (<span class="string">'\u{470}'</span>, <span class="string">'\u{470}'</span>, SC_Upper), (<span class="string">'\u{471}'</span>, <span class="string">'\u{471}'</span>, SC_Lower), (<span class="string">'\u{472}'</span>, <span class="string">'\u{472}'</span>,
        SC_Upper), (<span class="string">'\u{473}'</span>, <span class="string">'\u{473}'</span>, SC_Lower), (<span class="string">'\u{474}'</span>, <span class="string">'\u{474}'</span>, SC_Upper), (<span class="string">'\u{475}'</span>,
        <span class="string">'\u{475}'</span>, SC_Lower), (<span class="string">'\u{476}'</span>, <span class="string">'\u{476}'</span>, SC_Upper), (<span class="string">'\u{477}'</span>, <span class="string">'\u{477}'</span>, SC_Lower),
        (<span class="string">'\u{478}'</span>, <span class="string">'\u{478}'</span>, SC_Upper), (<span class="string">'\u{479}'</span>, <span class="string">'\u{479}'</span>, SC_Lower), (<span class="string">'\u{47a}'</span>, <span class="string">'\u{47a}'</span>,
        SC_Upper), (<span class="string">'\u{47b}'</span>, <span class="string">'\u{47b}'</span>, SC_Lower), (<span class="string">'\u{47c}'</span>, <span class="string">'\u{47c}'</span>, SC_Upper), (<span class="string">'\u{47d}'</span>,
        <span class="string">'\u{47d}'</span>, SC_Lower), (<span class="string">'\u{47e}'</span>, <span class="string">'\u{47e}'</span>, SC_Upper), (<span class="string">'\u{47f}'</span>, <span class="string">'\u{47f}'</span>, SC_Lower),
        (<span class="string">'\u{480}'</span>, <span class="string">'\u{480}'</span>, SC_Upper), (<span class="string">'\u{481}'</span>, <span class="string">'\u{481}'</span>, SC_Lower), (<span class="string">'\u{483}'</span>, <span class="string">'\u{489}'</span>,
        SC_Extend), (<span class="string">'\u{48a}'</span>, <span class="string">'\u{48a}'</span>, SC_Upper), (<span class="string">'\u{48b}'</span>, <span class="string">'\u{48b}'</span>, SC_Lower), (<span class="string">'\u{48c}'</span>,
        <span class="string">'\u{48c}'</span>, SC_Upper), (<span class="string">'\u{48d}'</span>, <span class="string">'\u{48d}'</span>, SC_Lower), (<span class="string">'\u{48e}'</span>, <span class="string">'\u{48e}'</span>, SC_Upper),
        (<span class="string">'\u{48f}'</span>, <span class="string">'\u{48f}'</span>, SC_Lower), (<span class="string">'\u{490}'</span>, <span class="string">'\u{490}'</span>, SC_Upper), (<span class="string">'\u{491}'</span>, <span class="string">'\u{491}'</span>,
        SC_Lower), (<span class="string">'\u{492}'</span>, <span class="string">'\u{492}'</span>, SC_Upper), (<span class="string">'\u{493}'</span>, <span class="string">'\u{493}'</span>, SC_Lower), (<span class="string">'\u{494}'</span>,
        <span class="string">'\u{494}'</span>, SC_Upper), (<span class="string">'\u{495}'</span>, <span class="string">'\u{495}'</span>, SC_Lower), (<span class="string">'\u{496}'</span>, <span class="string">'\u{496}'</span>, SC_Upper),
        (<span class="string">'\u{497}'</span>, <span class="string">'\u{497}'</span>, SC_Lower), (<span class="string">'\u{498}'</span>, <span class="string">'\u{498}'</span>, SC_Upper), (<span class="string">'\u{499}'</span>, <span class="string">'\u{499}'</span>,
        SC_Lower), (<span class="string">'\u{49a}'</span>, <span class="string">'\u{49a}'</span>, SC_Upper), (<span class="string">'\u{49b}'</span>, <span class="string">'\u{49b}'</span>, SC_Lower), (<span class="string">'\u{49c}'</span>,
        <span class="string">'\u{49c}'</span>, SC_Upper), (<span class="string">'\u{49d}'</span>, <span class="string">'\u{49d}'</span>, SC_Lower), (<span class="string">'\u{49e}'</span>, <span class="string">'\u{49e}'</span>, SC_Upper),
        (<span class="string">'\u{49f}'</span>, <span class="string">'\u{49f}'</span>, SC_Lower), (<span class="string">'\u{4a0}'</span>, <span class="string">'\u{4a0}'</span>, SC_Upper), (<span class="string">'\u{4a1}'</span>, <span class="string">'\u{4a1}'</span>,
        SC_Lower), (<span class="string">'\u{4a2}'</span>, <span class="string">'\u{4a2}'</span>, SC_Upper), (<span class="string">'\u{4a3}'</span>, <span class="string">'\u{4a3}'</span>, SC_Lower), (<span class="string">'\u{4a4}'</span>,
        <span class="string">'\u{4a4}'</span>, SC_Upper), (<span class="string">'\u{4a5}'</span>, <span class="string">'\u{4a5}'</span>, SC_Lower), (<span class="string">'\u{4a6}'</span>, <span class="string">'\u{4a6}'</span>, SC_Upper),
        (<span class="string">'\u{4a7}'</span>, <span class="string">'\u{4a7}'</span>, SC_Lower), (<span class="string">'\u{4a8}'</span>, <span class="string">'\u{4a8}'</span>, SC_Upper), (<span class="string">'\u{4a9}'</span>, <span class="string">'\u{4a9}'</span>,
        SC_Lower), (<span class="string">'\u{4aa}'</span>, <span class="string">'\u{4aa}'</span>, SC_Upper), (<span class="string">'\u{4ab}'</span>, <span class="string">'\u{4ab}'</span>, SC_Lower), (<span class="string">'\u{4ac}'</span>,
        <span class="string">'\u{4ac}'</span>, SC_Upper), (<span class="string">'\u{4ad}'</span>, <span class="string">'\u{4ad}'</span>, SC_Lower), (<span class="string">'\u{4ae}'</span>, <span class="string">'\u{4ae}'</span>, SC_Upper),
        (<span class="string">'\u{4af}'</span>, <span class="string">'\u{4af}'</span>, SC_Lower), (<span class="string">'\u{4b0}'</span>, <span class="string">'\u{4b0}'</span>, SC_Upper), (<span class="string">'\u{4b1}'</span>, <span class="string">'\u{4b1}'</span>,
        SC_Lower), (<span class="string">'\u{4b2}'</span>, <span class="string">'\u{4b2}'</span>, SC_Upper), (<span class="string">'\u{4b3}'</span>, <span class="string">'\u{4b3}'</span>, SC_Lower), (<span class="string">'\u{4b4}'</span>,
        <span class="string">'\u{4b4}'</span>, SC_Upper), (<span class="string">'\u{4b5}'</span>, <span class="string">'\u{4b5}'</span>, SC_Lower), (<span class="string">'\u{4b6}'</span>, <span class="string">'\u{4b6}'</span>, SC_Upper),
        (<span class="string">'\u{4b7}'</span>, <span class="string">'\u{4b7}'</span>, SC_Lower), (<span class="string">'\u{4b8}'</span>, <span class="string">'\u{4b8}'</span>, SC_Upper), (<span class="string">'\u{4b9}'</span>, <span class="string">'\u{4b9}'</span>,
        SC_Lower), (<span class="string">'\u{4ba}'</span>, <span class="string">'\u{4ba}'</span>, SC_Upper), (<span class="string">'\u{4bb}'</span>, <span class="string">'\u{4bb}'</span>, SC_Lower), (<span class="string">'\u{4bc}'</span>,
        <span class="string">'\u{4bc}'</span>, SC_Upper), (<span class="string">'\u{4bd}'</span>, <span class="string">'\u{4bd}'</span>, SC_Lower), (<span class="string">'\u{4be}'</span>, <span class="string">'\u{4be}'</span>, SC_Upper),
        (<span class="string">'\u{4bf}'</span>, <span class="string">'\u{4bf}'</span>, SC_Lower), (<span class="string">'\u{4c0}'</span>, <span class="string">'\u{4c1}'</span>, SC_Upper), (<span class="string">'\u{4c2}'</span>, <span class="string">'\u{4c2}'</span>,
        SC_Lower), (<span class="string">'\u{4c3}'</span>, <span class="string">'\u{4c3}'</span>, SC_Upper), (<span class="string">'\u{4c4}'</span>, <span class="string">'\u{4c4}'</span>, SC_Lower), (<span class="string">'\u{4c5}'</span>,
        <span class="string">'\u{4c5}'</span>, SC_Upper), (<span class="string">'\u{4c6}'</span>, <span class="string">'\u{4c6}'</span>, SC_Lower), (<span class="string">'\u{4c7}'</span>, <span class="string">'\u{4c7}'</span>, SC_Upper),
        (<span class="string">'\u{4c8}'</span>, <span class="string">'\u{4c8}'</span>, SC_Lower), (<span class="string">'\u{4c9}'</span>, <span class="string">'\u{4c9}'</span>, SC_Upper), (<span class="string">'\u{4ca}'</span>, <span class="string">'\u{4ca}'</span>,
        SC_Lower), (<span class="string">'\u{4cb}'</span>, <span class="string">'\u{4cb}'</span>, SC_Upper), (<span class="string">'\u{4cc}'</span>, <span class="string">'\u{4cc}'</span>, SC_Lower), (<span class="string">'\u{4cd}'</span>,
        <span class="string">'\u{4cd}'</span>, SC_Upper), (<span class="string">'\u{4ce}'</span>, <span class="string">'\u{4cf}'</span>, SC_Lower), (<span class="string">'\u{4d0}'</span>, <span class="string">'\u{4d0}'</span>, SC_Upper),
        (<span class="string">'\u{4d1}'</span>, <span class="string">'\u{4d1}'</span>, SC_Lower), (<span class="string">'\u{4d2}'</span>, <span class="string">'\u{4d2}'</span>, SC_Upper), (<span class="string">'\u{4d3}'</span>, <span class="string">'\u{4d3}'</span>,
        SC_Lower), (<span class="string">'\u{4d4}'</span>, <span class="string">'\u{4d4}'</span>, SC_Upper), (<span class="string">'\u{4d5}'</span>, <span class="string">'\u{4d5}'</span>, SC_Lower), (<span class="string">'\u{4d6}'</span>,
        <span class="string">'\u{4d6}'</span>, SC_Upper), (<span class="string">'\u{4d7}'</span>, <span class="string">'\u{4d7}'</span>, SC_Lower), (<span class="string">'\u{4d8}'</span>, <span class="string">'\u{4d8}'</span>, SC_Upper),
        (<span class="string">'\u{4d9}'</span>, <span class="string">'\u{4d9}'</span>, SC_Lower), (<span class="string">'\u{4da}'</span>, <span class="string">'\u{4da}'</span>, SC_Upper), (<span class="string">'\u{4db}'</span>, <span class="string">'\u{4db}'</span>,
        SC_Lower), (<span class="string">'\u{4dc}'</span>, <span class="string">'\u{4dc}'</span>, SC_Upper), (<span class="string">'\u{4dd}'</span>, <span class="string">'\u{4dd}'</span>, SC_Lower), (<span class="string">'\u{4de}'</span>,
        <span class="string">'\u{4de}'</span>, SC_Upper), (<span class="string">'\u{4df}'</span>, <span class="string">'\u{4df}'</span>, SC_Lower), (<span class="string">'\u{4e0}'</span>, <span class="string">'\u{4e0}'</span>, SC_Upper),
        (<span class="string">'\u{4e1}'</span>, <span class="string">'\u{4e1}'</span>, SC_Lower), (<span class="string">'\u{4e2}'</span>, <span class="string">'\u{4e2}'</span>, SC_Upper), (<span class="string">'\u{4e3}'</span>, <span class="string">'\u{4e3}'</span>,
        SC_Lower), (<span class="string">'\u{4e4}'</span>, <span class="string">'\u{4e4}'</span>, SC_Upper), (<span class="string">'\u{4e5}'</span>, <span class="string">'\u{4e5}'</span>, SC_Lower), (<span class="string">'\u{4e6}'</span>,
        <span class="string">'\u{4e6}'</span>, SC_Upper), (<span class="string">'\u{4e7}'</span>, <span class="string">'\u{4e7}'</span>, SC_Lower), (<span class="string">'\u{4e8}'</span>, <span class="string">'\u{4e8}'</span>, SC_Upper),
        (<span class="string">'\u{4e9}'</span>, <span class="string">'\u{4e9}'</span>, SC_Lower), (<span class="string">'\u{4ea}'</span>, <span class="string">'\u{4ea}'</span>, SC_Upper), (<span class="string">'\u{4eb}'</span>, <span class="string">'\u{4eb}'</span>,
        SC_Lower), (<span class="string">'\u{4ec}'</span>, <span class="string">'\u{4ec}'</span>, SC_Upper), (<span class="string">'\u{4ed}'</span>, <span class="string">'\u{4ed}'</span>, SC_Lower), (<span class="string">'\u{4ee}'</span>,
        <span class="string">'\u{4ee}'</span>, SC_Upper), (<span class="string">'\u{4ef}'</span>, <span class="string">'\u{4ef}'</span>, SC_Lower), (<span class="string">'\u{4f0}'</span>, <span class="string">'\u{4f0}'</span>, SC_Upper),
        (<span class="string">'\u{4f1}'</span>, <span class="string">'\u{4f1}'</span>, SC_Lower), (<span class="string">'\u{4f2}'</span>, <span class="string">'\u{4f2}'</span>, SC_Upper), (<span class="string">'\u{4f3}'</span>, <span class="string">'\u{4f3}'</span>,
        SC_Lower), (<span class="string">'\u{4f4}'</span>, <span class="string">'\u{4f4}'</span>, SC_Upper), (<span class="string">'\u{4f5}'</span>, <span class="string">'\u{4f5}'</span>, SC_Lower), (<span class="string">'\u{4f6}'</span>,
        <span class="string">'\u{4f6}'</span>, SC_Upper), (<span class="string">'\u{4f7}'</span>, <span class="string">'\u{4f7}'</span>, SC_Lower), (<span class="string">'\u{4f8}'</span>, <span class="string">'\u{4f8}'</span>, SC_Upper),
        (<span class="string">'\u{4f9}'</span>, <span class="string">'\u{4f9}'</span>, SC_Lower), (<span class="string">'\u{4fa}'</span>, <span class="string">'\u{4fa}'</span>, SC_Upper), (<span class="string">'\u{4fb}'</span>, <span class="string">'\u{4fb}'</span>,
        SC_Lower), (<span class="string">'\u{4fc}'</span>, <span class="string">'\u{4fc}'</span>, SC_Upper), (<span class="string">'\u{4fd}'</span>, <span class="string">'\u{4fd}'</span>, SC_Lower), (<span class="string">'\u{4fe}'</span>,
        <span class="string">'\u{4fe}'</span>, SC_Upper), (<span class="string">'\u{4ff}'</span>, <span class="string">'\u{4ff}'</span>, SC_Lower), (<span class="string">'\u{500}'</span>, <span class="string">'\u{500}'</span>, SC_Upper),
        (<span class="string">'\u{501}'</span>, <span class="string">'\u{501}'</span>, SC_Lower), (<span class="string">'\u{502}'</span>, <span class="string">'\u{502}'</span>, SC_Upper), (<span class="string">'\u{503}'</span>, <span class="string">'\u{503}'</span>,
        SC_Lower), (<span class="string">'\u{504}'</span>, <span class="string">'\u{504}'</span>, SC_Upper), (<span class="string">'\u{505}'</span>, <span class="string">'\u{505}'</span>, SC_Lower), (<span class="string">'\u{506}'</span>,
        <span class="string">'\u{506}'</span>, SC_Upper), (<span class="string">'\u{507}'</span>, <span class="string">'\u{507}'</span>, SC_Lower), (<span class="string">'\u{508}'</span>, <span class="string">'\u{508}'</span>, SC_Upper),
        (<span class="string">'\u{509}'</span>, <span class="string">'\u{509}'</span>, SC_Lower), (<span class="string">'\u{50a}'</span>, <span class="string">'\u{50a}'</span>, SC_Upper), (<span class="string">'\u{50b}'</span>, <span class="string">'\u{50b}'</span>,
        SC_Lower), (<span class="string">'\u{50c}'</span>, <span class="string">'\u{50c}'</span>, SC_Upper), (<span class="string">'\u{50d}'</span>, <span class="string">'\u{50d}'</span>, SC_Lower), (<span class="string">'\u{50e}'</span>,
        <span class="string">'\u{50e}'</span>, SC_Upper), (<span class="string">'\u{50f}'</span>, <span class="string">'\u{50f}'</span>, SC_Lower), (<span class="string">'\u{510}'</span>, <span class="string">'\u{510}'</span>, SC_Upper),
        (<span class="string">'\u{511}'</span>, <span class="string">'\u{511}'</span>, SC_Lower), (<span class="string">'\u{512}'</span>, <span class="string">'\u{512}'</span>, SC_Upper), (<span class="string">'\u{513}'</span>, <span class="string">'\u{513}'</span>,
        SC_Lower), (<span class="string">'\u{514}'</span>, <span class="string">'\u{514}'</span>, SC_Upper), (<span class="string">'\u{515}'</span>, <span class="string">'\u{515}'</span>, SC_Lower), (<span class="string">'\u{516}'</span>,
        <span class="string">'\u{516}'</span>, SC_Upper), (<span class="string">'\u{517}'</span>, <span class="string">'\u{517}'</span>, SC_Lower), (<span class="string">'\u{518}'</span>, <span class="string">'\u{518}'</span>, SC_Upper),
        (<span class="string">'\u{519}'</span>, <span class="string">'\u{519}'</span>, SC_Lower), (<span class="string">'\u{51a}'</span>, <span class="string">'\u{51a}'</span>, SC_Upper), (<span class="string">'\u{51b}'</span>, <span class="string">'\u{51b}'</span>,
        SC_Lower), (<span class="string">'\u{51c}'</span>, <span class="string">'\u{51c}'</span>, SC_Upper), (<span class="string">'\u{51d}'</span>, <span class="string">'\u{51d}'</span>, SC_Lower), (<span class="string">'\u{51e}'</span>,
        <span class="string">'\u{51e}'</span>, SC_Upper), (<span class="string">'\u{51f}'</span>, <span class="string">'\u{51f}'</span>, SC_Lower), (<span class="string">'\u{520}'</span>, <span class="string">'\u{520}'</span>, SC_Upper),
        (<span class="string">'\u{521}'</span>, <span class="string">'\u{521}'</span>, SC_Lower), (<span class="string">'\u{522}'</span>, <span class="string">'\u{522}'</span>, SC_Upper), (<span class="string">'\u{523}'</span>, <span class="string">'\u{523}'</span>,
        SC_Lower), (<span class="string">'\u{524}'</span>, <span class="string">'\u{524}'</span>, SC_Upper), (<span class="string">'\u{525}'</span>, <span class="string">'\u{525}'</span>, SC_Lower), (<span class="string">'\u{526}'</span>,
        <span class="string">'\u{526}'</span>, SC_Upper), (<span class="string">'\u{527}'</span>, <span class="string">'\u{527}'</span>, SC_Lower), (<span class="string">'\u{528}'</span>, <span class="string">'\u{528}'</span>, SC_Upper),
        (<span class="string">'\u{529}'</span>, <span class="string">'\u{529}'</span>, SC_Lower), (<span class="string">'\u{52a}'</span>, <span class="string">'\u{52a}'</span>, SC_Upper), (<span class="string">'\u{52b}'</span>, <span class="string">'\u{52b}'</span>,
        SC_Lower), (<span class="string">'\u{52c}'</span>, <span class="string">'\u{52c}'</span>, SC_Upper), (<span class="string">'\u{52d}'</span>, <span class="string">'\u{52d}'</span>, SC_Lower), (<span class="string">'\u{52e}'</span>,
        <span class="string">'\u{52e}'</span>, SC_Upper), (<span class="string">'\u{52f}'</span>, <span class="string">'\u{52f}'</span>, SC_Lower), (<span class="string">'\u{531}'</span>, <span class="string">'\u{556}'</span>, SC_Upper),
        (<span class="string">'\u{559}'</span>, <span class="string">'\u{559}'</span>, SC_OLetter), (<span class="string">'\u{55d}'</span>, <span class="string">'\u{55d}'</span>, SC_SContinue), (<span class="string">'\u{560}'</span>,
        <span class="string">'\u{588}'</span>, SC_Lower), (<span class="string">'\u{589}'</span>, <span class="string">'\u{589}'</span>, SC_STerm), (<span class="string">'\u{591}'</span>, <span class="string">'\u{5bd}'</span>, SC_Extend),
        (<span class="string">'\u{5bf}'</span>, <span class="string">'\u{5bf}'</span>, SC_Extend), (<span class="string">'\u{5c1}'</span>, <span class="string">'\u{5c2}'</span>, SC_Extend), (<span class="string">'\u{5c4}'</span>, <span class="string">'\u{5c5}'</span>,
        SC_Extend), (<span class="string">'\u{5c7}'</span>, <span class="string">'\u{5c7}'</span>, SC_Extend), (<span class="string">'\u{5d0}'</span>, <span class="string">'\u{5ea}'</span>, SC_OLetter),
        (<span class="string">'\u{5ef}'</span>, <span class="string">'\u{5f3}'</span>, SC_OLetter), (<span class="string">'\u{600}'</span>, <span class="string">'\u{605}'</span>, SC_Numeric), (<span class="string">'\u{60c}'</span>,
        <span class="string">'\u{60d}'</span>, SC_SContinue), (<span class="string">'\u{610}'</span>, <span class="string">'\u{61a}'</span>, SC_Extend), (<span class="string">'\u{61c}'</span>, <span class="string">'\u{61c}'</span>,
        SC_Format), (<span class="string">'\u{61d}'</span>, <span class="string">'\u{61f}'</span>, SC_STerm), (<span class="string">'\u{620}'</span>, <span class="string">'\u{64a}'</span>, SC_OLetter),
        (<span class="string">'\u{64b}'</span>, <span class="string">'\u{65f}'</span>, SC_Extend), (<span class="string">'\u{660}'</span>, <span class="string">'\u{669}'</span>, SC_Numeric), (<span class="string">'\u{66b}'</span>,
        <span class="string">'\u{66c}'</span>, SC_Numeric), (<span class="string">'\u{66e}'</span>, <span class="string">'\u{66f}'</span>, SC_OLetter), (<span class="string">'\u{670}'</span>, <span class="string">'\u{670}'</span>,
        SC_Extend), (<span class="string">'\u{671}'</span>, <span class="string">'\u{6d3}'</span>, SC_OLetter), (<span class="string">'\u{6d4}'</span>, <span class="string">'\u{6d4}'</span>, SC_STerm),
        (<span class="string">'\u{6d5}'</span>, <span class="string">'\u{6d5}'</span>, SC_OLetter), (<span class="string">'\u{6d6}'</span>, <span class="string">'\u{6dc}'</span>, SC_Extend), (<span class="string">'\u{6dd}'</span>,
        <span class="string">'\u{6dd}'</span>, SC_Numeric), (<span class="string">'\u{6df}'</span>, <span class="string">'\u{6e4}'</span>, SC_Extend), (<span class="string">'\u{6e5}'</span>, <span class="string">'\u{6e6}'</span>,
        SC_OLetter), (<span class="string">'\u{6e7}'</span>, <span class="string">'\u{6e8}'</span>, SC_Extend), (<span class="string">'\u{6ea}'</span>, <span class="string">'\u{6ed}'</span>, SC_Extend),
        (<span class="string">'\u{6ee}'</span>, <span class="string">'\u{6ef}'</span>, SC_OLetter), (<span class="string">'\u{6f0}'</span>, <span class="string">'\u{6f9}'</span>, SC_Numeric), (<span class="string">'\u{6fa}'</span>,
        <span class="string">'\u{6fc}'</span>, SC_OLetter), (<span class="string">'\u{6ff}'</span>, <span class="string">'\u{6ff}'</span>, SC_OLetter), (<span class="string">'\u{700}'</span>, <span class="string">'\u{702}'</span>,
        SC_STerm), (<span class="string">'\u{70f}'</span>, <span class="string">'\u{70f}'</span>, SC_Format), (<span class="string">'\u{710}'</span>, <span class="string">'\u{710}'</span>, SC_OLetter),
        (<span class="string">'\u{711}'</span>, <span class="string">'\u{711}'</span>, SC_Extend), (<span class="string">'\u{712}'</span>, <span class="string">'\u{72f}'</span>, SC_OLetter), (<span class="string">'\u{730}'</span>,
        <span class="string">'\u{74a}'</span>, SC_Extend), (<span class="string">'\u{74d}'</span>, <span class="string">'\u{7a5}'</span>, SC_OLetter), (<span class="string">'\u{7a6}'</span>, <span class="string">'\u{7b0}'</span>,
        SC_Extend), (<span class="string">'\u{7b1}'</span>, <span class="string">'\u{7b1}'</span>, SC_OLetter), (<span class="string">'\u{7c0}'</span>, <span class="string">'\u{7c9}'</span>, SC_Numeric),
        (<span class="string">'\u{7ca}'</span>, <span class="string">'\u{7ea}'</span>, SC_OLetter), (<span class="string">'\u{7eb}'</span>, <span class="string">'\u{7f3}'</span>, SC_Extend), (<span class="string">'\u{7f4}'</span>,
        <span class="string">'\u{7f5}'</span>, SC_OLetter), (<span class="string">'\u{7f8}'</span>, <span class="string">'\u{7f8}'</span>, SC_SContinue), (<span class="string">'\u{7f9}'</span>, <span class="string">'\u{7f9}'</span>,
        SC_STerm), (<span class="string">'\u{7fa}'</span>, <span class="string">'\u{7fa}'</span>, SC_OLetter), (<span class="string">'\u{7fd}'</span>, <span class="string">'\u{7fd}'</span>, SC_Extend),
        (<span class="string">'\u{800}'</span>, <span class="string">'\u{815}'</span>, SC_OLetter), (<span class="string">'\u{816}'</span>, <span class="string">'\u{819}'</span>, SC_Extend), (<span class="string">'\u{81a}'</span>,
        <span class="string">'\u{81a}'</span>, SC_OLetter), (<span class="string">'\u{81b}'</span>, <span class="string">'\u{823}'</span>, SC_Extend), (<span class="string">'\u{824}'</span>, <span class="string">'\u{824}'</span>,
        SC_OLetter), (<span class="string">'\u{825}'</span>, <span class="string">'\u{827}'</span>, SC_Extend), (<span class="string">'\u{828}'</span>, <span class="string">'\u{828}'</span>, SC_OLetter),
        (<span class="string">'\u{829}'</span>, <span class="string">'\u{82d}'</span>, SC_Extend), (<span class="string">'\u{837}'</span>, <span class="string">'\u{837}'</span>, SC_STerm), (<span class="string">'\u{839}'</span>, <span class="string">'\u{839}'</span>,
        SC_STerm), (<span class="string">'\u{83d}'</span>, <span class="string">'\u{83e}'</span>, SC_STerm), (<span class="string">'\u{840}'</span>, <span class="string">'\u{858}'</span>, SC_OLetter), (<span class="string">'\u{859}'</span>,
        <span class="string">'\u{85b}'</span>, SC_Extend), (<span class="string">'\u{860}'</span>, <span class="string">'\u{86a}'</span>, SC_OLetter), (<span class="string">'\u{870}'</span>, <span class="string">'\u{887}'</span>,
        SC_OLetter), (<span class="string">'\u{889}'</span>, <span class="string">'\u{88e}'</span>, SC_OLetter), (<span class="string">'\u{890}'</span>, <span class="string">'\u{891}'</span>, SC_Numeric),
        (<span class="string">'\u{898}'</span>, <span class="string">'\u{89f}'</span>, SC_Extend), (<span class="string">'\u{8a0}'</span>, <span class="string">'\u{8c9}'</span>, SC_OLetter), (<span class="string">'\u{8ca}'</span>,
        <span class="string">'\u{8e1}'</span>, SC_Extend), (<span class="string">'\u{8e2}'</span>, <span class="string">'\u{8e2}'</span>, SC_Numeric), (<span class="string">'\u{8e3}'</span>, <span class="string">'\u{903}'</span>,
        SC_Extend), (<span class="string">'\u{904}'</span>, <span class="string">'\u{939}'</span>, SC_OLetter), (<span class="string">'\u{93a}'</span>, <span class="string">'\u{93c}'</span>, SC_Extend),
        (<span class="string">'\u{93d}'</span>, <span class="string">'\u{93d}'</span>, SC_OLetter), (<span class="string">'\u{93e}'</span>, <span class="string">'\u{94f}'</span>, SC_Extend), (<span class="string">'\u{950}'</span>,
        <span class="string">'\u{950}'</span>, SC_OLetter), (<span class="string">'\u{951}'</span>, <span class="string">'\u{957}'</span>, SC_Extend), (<span class="string">'\u{958}'</span>, <span class="string">'\u{961}'</span>,
        SC_OLetter), (<span class="string">'\u{962}'</span>, <span class="string">'\u{963}'</span>, SC_Extend), (<span class="string">'\u{964}'</span>, <span class="string">'\u{965}'</span>, SC_STerm),
        (<span class="string">'\u{966}'</span>, <span class="string">'\u{96f}'</span>, SC_Numeric), (<span class="string">'\u{971}'</span>, <span class="string">'\u{980}'</span>, SC_OLetter), (<span class="string">'\u{981}'</span>,
        <span class="string">'\u{983}'</span>, SC_Extend), (<span class="string">'\u{985}'</span>, <span class="string">'\u{98c}'</span>, SC_OLetter), (<span class="string">'\u{98f}'</span>, <span class="string">'\u{990}'</span>,
        SC_OLetter), (<span class="string">'\u{993}'</span>, <span class="string">'\u{9a8}'</span>, SC_OLetter), (<span class="string">'\u{9aa}'</span>, <span class="string">'\u{9b0}'</span>, SC_OLetter),
        (<span class="string">'\u{9b2}'</span>, <span class="string">'\u{9b2}'</span>, SC_OLetter), (<span class="string">'\u{9b6}'</span>, <span class="string">'\u{9b9}'</span>, SC_OLetter), (<span class="string">'\u{9bc}'</span>,
        <span class="string">'\u{9bc}'</span>, SC_Extend), (<span class="string">'\u{9bd}'</span>, <span class="string">'\u{9bd}'</span>, SC_OLetter), (<span class="string">'\u{9be}'</span>, <span class="string">'\u{9c4}'</span>,
        SC_Extend), (<span class="string">'\u{9c7}'</span>, <span class="string">'\u{9c8}'</span>, SC_Extend), (<span class="string">'\u{9cb}'</span>, <span class="string">'\u{9cd}'</span>, SC_Extend),
        (<span class="string">'\u{9ce}'</span>, <span class="string">'\u{9ce}'</span>, SC_OLetter), (<span class="string">'\u{9d7}'</span>, <span class="string">'\u{9d7}'</span>, SC_Extend), (<span class="string">'\u{9dc}'</span>,
        <span class="string">'\u{9dd}'</span>, SC_OLetter), (<span class="string">'\u{9df}'</span>, <span class="string">'\u{9e1}'</span>, SC_OLetter), (<span class="string">'\u{9e2}'</span>, <span class="string">'\u{9e3}'</span>,
        SC_Extend), (<span class="string">'\u{9e6}'</span>, <span class="string">'\u{9ef}'</span>, SC_Numeric), (<span class="string">'\u{9f0}'</span>, <span class="string">'\u{9f1}'</span>, SC_OLetter),
        (<span class="string">'\u{9fc}'</span>, <span class="string">'\u{9fc}'</span>, SC_OLetter), (<span class="string">'\u{9fe}'</span>, <span class="string">'\u{9fe}'</span>, SC_Extend), (<span class="string">'\u{a01}'</span>,
        <span class="string">'\u{a03}'</span>, SC_Extend), (<span class="string">'\u{a05}'</span>, <span class="string">'\u{a0a}'</span>, SC_OLetter), (<span class="string">'\u{a0f}'</span>, <span class="string">'\u{a10}'</span>,
        SC_OLetter), (<span class="string">'\u{a13}'</span>, <span class="string">'\u{a28}'</span>, SC_OLetter), (<span class="string">'\u{a2a}'</span>, <span class="string">'\u{a30}'</span>, SC_OLetter),
        (<span class="string">'\u{a32}'</span>, <span class="string">'\u{a33}'</span>, SC_OLetter), (<span class="string">'\u{a35}'</span>, <span class="string">'\u{a36}'</span>, SC_OLetter), (<span class="string">'\u{a38}'</span>,
        <span class="string">'\u{a39}'</span>, SC_OLetter), (<span class="string">'\u{a3c}'</span>, <span class="string">'\u{a3c}'</span>, SC_Extend), (<span class="string">'\u{a3e}'</span>, <span class="string">'\u{a42}'</span>,
        SC_Extend), (<span class="string">'\u{a47}'</span>, <span class="string">'\u{a48}'</span>, SC_Extend), (<span class="string">'\u{a4b}'</span>, <span class="string">'\u{a4d}'</span>, SC_Extend),
        (<span class="string">'\u{a51}'</span>, <span class="string">'\u{a51}'</span>, SC_Extend), (<span class="string">'\u{a59}'</span>, <span class="string">'\u{a5c}'</span>, SC_OLetter), (<span class="string">'\u{a5e}'</span>,
        <span class="string">'\u{a5e}'</span>, SC_OLetter), (<span class="string">'\u{a66}'</span>, <span class="string">'\u{a6f}'</span>, SC_Numeric), (<span class="string">'\u{a70}'</span>, <span class="string">'\u{a71}'</span>,
        SC_Extend), (<span class="string">'\u{a72}'</span>, <span class="string">'\u{a74}'</span>, SC_OLetter), (<span class="string">'\u{a75}'</span>, <span class="string">'\u{a75}'</span>, SC_Extend),
        (<span class="string">'\u{a81}'</span>, <span class="string">'\u{a83}'</span>, SC_Extend), (<span class="string">'\u{a85}'</span>, <span class="string">'\u{a8d}'</span>, SC_OLetter), (<span class="string">'\u{a8f}'</span>,
        <span class="string">'\u{a91}'</span>, SC_OLetter), (<span class="string">'\u{a93}'</span>, <span class="string">'\u{aa8}'</span>, SC_OLetter), (<span class="string">'\u{aaa}'</span>, <span class="string">'\u{ab0}'</span>,
        SC_OLetter), (<span class="string">'\u{ab2}'</span>, <span class="string">'\u{ab3}'</span>, SC_OLetter), (<span class="string">'\u{ab5}'</span>, <span class="string">'\u{ab9}'</span>, SC_OLetter),
        (<span class="string">'\u{abc}'</span>, <span class="string">'\u{abc}'</span>, SC_Extend), (<span class="string">'\u{abd}'</span>, <span class="string">'\u{abd}'</span>, SC_OLetter), (<span class="string">'\u{abe}'</span>,
        <span class="string">'\u{ac5}'</span>, SC_Extend), (<span class="string">'\u{ac7}'</span>, <span class="string">'\u{ac9}'</span>, SC_Extend), (<span class="string">'\u{acb}'</span>, <span class="string">'\u{acd}'</span>, SC_Extend),
        (<span class="string">'\u{ad0}'</span>, <span class="string">'\u{ad0}'</span>, SC_OLetter), (<span class="string">'\u{ae0}'</span>, <span class="string">'\u{ae1}'</span>, SC_OLetter), (<span class="string">'\u{ae2}'</span>,
        <span class="string">'\u{ae3}'</span>, SC_Extend), (<span class="string">'\u{ae6}'</span>, <span class="string">'\u{aef}'</span>, SC_Numeric), (<span class="string">'\u{af9}'</span>, <span class="string">'\u{af9}'</span>,
        SC_OLetter), (<span class="string">'\u{afa}'</span>, <span class="string">'\u{aff}'</span>, SC_Extend), (<span class="string">'\u{b01}'</span>, <span class="string">'\u{b03}'</span>, SC_Extend),
        (<span class="string">'\u{b05}'</span>, <span class="string">'\u{b0c}'</span>, SC_OLetter), (<span class="string">'\u{b0f}'</span>, <span class="string">'\u{b10}'</span>, SC_OLetter), (<span class="string">'\u{b13}'</span>,
        <span class="string">'\u{b28}'</span>, SC_OLetter), (<span class="string">'\u{b2a}'</span>, <span class="string">'\u{b30}'</span>, SC_OLetter), (<span class="string">'\u{b32}'</span>, <span class="string">'\u{b33}'</span>,
        SC_OLetter), (<span class="string">'\u{b35}'</span>, <span class="string">'\u{b39}'</span>, SC_OLetter), (<span class="string">'\u{b3c}'</span>, <span class="string">'\u{b3c}'</span>, SC_Extend),
        (<span class="string">'\u{b3d}'</span>, <span class="string">'\u{b3d}'</span>, SC_OLetter), (<span class="string">'\u{b3e}'</span>, <span class="string">'\u{b44}'</span>, SC_Extend), (<span class="string">'\u{b47}'</span>,
        <span class="string">'\u{b48}'</span>, SC_Extend), (<span class="string">'\u{b4b}'</span>, <span class="string">'\u{b4d}'</span>, SC_Extend), (<span class="string">'\u{b55}'</span>, <span class="string">'\u{b57}'</span>, SC_Extend),
        (<span class="string">'\u{b5c}'</span>, <span class="string">'\u{b5d}'</span>, SC_OLetter), (<span class="string">'\u{b5f}'</span>, <span class="string">'\u{b61}'</span>, SC_OLetter), (<span class="string">'\u{b62}'</span>,
        <span class="string">'\u{b63}'</span>, SC_Extend), (<span class="string">'\u{b66}'</span>, <span class="string">'\u{b6f}'</span>, SC_Numeric), (<span class="string">'\u{b71}'</span>, <span class="string">'\u{b71}'</span>,
        SC_OLetter), (<span class="string">'\u{b82}'</span>, <span class="string">'\u{b82}'</span>, SC_Extend), (<span class="string">'\u{b83}'</span>, <span class="string">'\u{b83}'</span>, SC_OLetter),
        (<span class="string">'\u{b85}'</span>, <span class="string">'\u{b8a}'</span>, SC_OLetter), (<span class="string">'\u{b8e}'</span>, <span class="string">'\u{b90}'</span>, SC_OLetter), (<span class="string">'\u{b92}'</span>,
        <span class="string">'\u{b95}'</span>, SC_OLetter), (<span class="string">'\u{b99}'</span>, <span class="string">'\u{b9a}'</span>, SC_OLetter), (<span class="string">'\u{b9c}'</span>, <span class="string">'\u{b9c}'</span>,
        SC_OLetter), (<span class="string">'\u{b9e}'</span>, <span class="string">'\u{b9f}'</span>, SC_OLetter), (<span class="string">'\u{ba3}'</span>, <span class="string">'\u{ba4}'</span>, SC_OLetter),
        (<span class="string">'\u{ba8}'</span>, <span class="string">'\u{baa}'</span>, SC_OLetter), (<span class="string">'\u{bae}'</span>, <span class="string">'\u{bb9}'</span>, SC_OLetter), (<span class="string">'\u{bbe}'</span>,
        <span class="string">'\u{bc2}'</span>, SC_Extend), (<span class="string">'\u{bc6}'</span>, <span class="string">'\u{bc8}'</span>, SC_Extend), (<span class="string">'\u{bca}'</span>, <span class="string">'\u{bcd}'</span>, SC_Extend),
        (<span class="string">'\u{bd0}'</span>, <span class="string">'\u{bd0}'</span>, SC_OLetter), (<span class="string">'\u{bd7}'</span>, <span class="string">'\u{bd7}'</span>, SC_Extend), (<span class="string">'\u{be6}'</span>,
        <span class="string">'\u{bef}'</span>, SC_Numeric), (<span class="string">'\u{c00}'</span>, <span class="string">'\u{c04}'</span>, SC_Extend), (<span class="string">'\u{c05}'</span>, <span class="string">'\u{c0c}'</span>,
        SC_OLetter), (<span class="string">'\u{c0e}'</span>, <span class="string">'\u{c10}'</span>, SC_OLetter), (<span class="string">'\u{c12}'</span>, <span class="string">'\u{c28}'</span>, SC_OLetter),
        (<span class="string">'\u{c2a}'</span>, <span class="string">'\u{c39}'</span>, SC_OLetter), (<span class="string">'\u{c3c}'</span>, <span class="string">'\u{c3c}'</span>, SC_Extend), (<span class="string">'\u{c3d}'</span>,
        <span class="string">'\u{c3d}'</span>, SC_OLetter), (<span class="string">'\u{c3e}'</span>, <span class="string">'\u{c44}'</span>, SC_Extend), (<span class="string">'\u{c46}'</span>, <span class="string">'\u{c48}'</span>,
        SC_Extend), (<span class="string">'\u{c4a}'</span>, <span class="string">'\u{c4d}'</span>, SC_Extend), (<span class="string">'\u{c55}'</span>, <span class="string">'\u{c56}'</span>, SC_Extend),
        (<span class="string">'\u{c58}'</span>, <span class="string">'\u{c5a}'</span>, SC_OLetter), (<span class="string">'\u{c5d}'</span>, <span class="string">'\u{c5d}'</span>, SC_OLetter), (<span class="string">'\u{c60}'</span>,
        <span class="string">'\u{c61}'</span>, SC_OLetter), (<span class="string">'\u{c62}'</span>, <span class="string">'\u{c63}'</span>, SC_Extend), (<span class="string">'\u{c66}'</span>, <span class="string">'\u{c6f}'</span>,
        SC_Numeric), (<span class="string">'\u{c80}'</span>, <span class="string">'\u{c80}'</span>, SC_OLetter), (<span class="string">'\u{c81}'</span>, <span class="string">'\u{c83}'</span>, SC_Extend),
        (<span class="string">'\u{c85}'</span>, <span class="string">'\u{c8c}'</span>, SC_OLetter), (<span class="string">'\u{c8e}'</span>, <span class="string">'\u{c90}'</span>, SC_OLetter), (<span class="string">'\u{c92}'</span>,
        <span class="string">'\u{ca8}'</span>, SC_OLetter), (<span class="string">'\u{caa}'</span>, <span class="string">'\u{cb3}'</span>, SC_OLetter), (<span class="string">'\u{cb5}'</span>, <span class="string">'\u{cb9}'</span>,
        SC_OLetter), (<span class="string">'\u{cbc}'</span>, <span class="string">'\u{cbc}'</span>, SC_Extend), (<span class="string">'\u{cbd}'</span>, <span class="string">'\u{cbd}'</span>, SC_OLetter),
        (<span class="string">'\u{cbe}'</span>, <span class="string">'\u{cc4}'</span>, SC_Extend), (<span class="string">'\u{cc6}'</span>, <span class="string">'\u{cc8}'</span>, SC_Extend), (<span class="string">'\u{cca}'</span>, <span class="string">'\u{ccd}'</span>,
        SC_Extend), (<span class="string">'\u{cd5}'</span>, <span class="string">'\u{cd6}'</span>, SC_Extend), (<span class="string">'\u{cdd}'</span>, <span class="string">'\u{cde}'</span>, SC_OLetter),
        (<span class="string">'\u{ce0}'</span>, <span class="string">'\u{ce1}'</span>, SC_OLetter), (<span class="string">'\u{ce2}'</span>, <span class="string">'\u{ce3}'</span>, SC_Extend), (<span class="string">'\u{ce6}'</span>,
        <span class="string">'\u{cef}'</span>, SC_Numeric), (<span class="string">'\u{cf1}'</span>, <span class="string">'\u{cf2}'</span>, SC_OLetter), (<span class="string">'\u{cf3}'</span>, <span class="string">'\u{cf3}'</span>,
        SC_Extend), (<span class="string">'\u{d00}'</span>, <span class="string">'\u{d03}'</span>, SC_Extend), (<span class="string">'\u{d04}'</span>, <span class="string">'\u{d0c}'</span>, SC_OLetter),
        (<span class="string">'\u{d0e}'</span>, <span class="string">'\u{d10}'</span>, SC_OLetter), (<span class="string">'\u{d12}'</span>, <span class="string">'\u{d3a}'</span>, SC_OLetter), (<span class="string">'\u{d3b}'</span>,
        <span class="string">'\u{d3c}'</span>, SC_Extend), (<span class="string">'\u{d3d}'</span>, <span class="string">'\u{d3d}'</span>, SC_OLetter), (<span class="string">'\u{d3e}'</span>, <span class="string">'\u{d44}'</span>,
        SC_Extend), (<span class="string">'\u{d46}'</span>, <span class="string">'\u{d48}'</span>, SC_Extend), (<span class="string">'\u{d4a}'</span>, <span class="string">'\u{d4d}'</span>, SC_Extend),
        (<span class="string">'\u{d4e}'</span>, <span class="string">'\u{d4e}'</span>, SC_OLetter), (<span class="string">'\u{d54}'</span>, <span class="string">'\u{d56}'</span>, SC_OLetter), (<span class="string">'\u{d57}'</span>,
        <span class="string">'\u{d57}'</span>, SC_Extend), (<span class="string">'\u{d5f}'</span>, <span class="string">'\u{d61}'</span>, SC_OLetter), (<span class="string">'\u{d62}'</span>, <span class="string">'\u{d63}'</span>,
        SC_Extend), (<span class="string">'\u{d66}'</span>, <span class="string">'\u{d6f}'</span>, SC_Numeric), (<span class="string">'\u{d7a}'</span>, <span class="string">'\u{d7f}'</span>, SC_OLetter),
        (<span class="string">'\u{d81}'</span>, <span class="string">'\u{d83}'</span>, SC_Extend), (<span class="string">'\u{d85}'</span>, <span class="string">'\u{d96}'</span>, SC_OLetter), (<span class="string">'\u{d9a}'</span>,
        <span class="string">'\u{db1}'</span>, SC_OLetter), (<span class="string">'\u{db3}'</span>, <span class="string">'\u{dbb}'</span>, SC_OLetter), (<span class="string">'\u{dbd}'</span>, <span class="string">'\u{dbd}'</span>,
        SC_OLetter), (<span class="string">'\u{dc0}'</span>, <span class="string">'\u{dc6}'</span>, SC_OLetter), (<span class="string">'\u{dca}'</span>, <span class="string">'\u{dca}'</span>, SC_Extend),
        (<span class="string">'\u{dcf}'</span>, <span class="string">'\u{dd4}'</span>, SC_Extend), (<span class="string">'\u{dd6}'</span>, <span class="string">'\u{dd6}'</span>, SC_Extend), (<span class="string">'\u{dd8}'</span>, <span class="string">'\u{ddf}'</span>,
        SC_Extend), (<span class="string">'\u{de6}'</span>, <span class="string">'\u{def}'</span>, SC_Numeric), (<span class="string">'\u{df2}'</span>, <span class="string">'\u{df3}'</span>, SC_Extend),
        (<span class="string">'\u{e01}'</span>, <span class="string">'\u{e30}'</span>, SC_OLetter), (<span class="string">'\u{e31}'</span>, <span class="string">'\u{e31}'</span>, SC_Extend), (<span class="string">'\u{e32}'</span>,
        <span class="string">'\u{e33}'</span>, SC_OLetter), (<span class="string">'\u{e34}'</span>, <span class="string">'\u{e3a}'</span>, SC_Extend), (<span class="string">'\u{e40}'</span>, <span class="string">'\u{e46}'</span>,
        SC_OLetter), (<span class="string">'\u{e47}'</span>, <span class="string">'\u{e4e}'</span>, SC_Extend), (<span class="string">'\u{e50}'</span>, <span class="string">'\u{e59}'</span>, SC_Numeric),
        (<span class="string">'\u{e81}'</span>, <span class="string">'\u{e82}'</span>, SC_OLetter), (<span class="string">'\u{e84}'</span>, <span class="string">'\u{e84}'</span>, SC_OLetter), (<span class="string">'\u{e86}'</span>,
        <span class="string">'\u{e8a}'</span>, SC_OLetter), (<span class="string">'\u{e8c}'</span>, <span class="string">'\u{ea3}'</span>, SC_OLetter), (<span class="string">'\u{ea5}'</span>, <span class="string">'\u{ea5}'</span>,
        SC_OLetter), (<span class="string">'\u{ea7}'</span>, <span class="string">'\u{eb0}'</span>, SC_OLetter), (<span class="string">'\u{eb1}'</span>, <span class="string">'\u{eb1}'</span>, SC_Extend),
        (<span class="string">'\u{eb2}'</span>, <span class="string">'\u{eb3}'</span>, SC_OLetter), (<span class="string">'\u{eb4}'</span>, <span class="string">'\u{ebc}'</span>, SC_Extend), (<span class="string">'\u{ebd}'</span>,
        <span class="string">'\u{ebd}'</span>, SC_OLetter), (<span class="string">'\u{ec0}'</span>, <span class="string">'\u{ec4}'</span>, SC_OLetter), (<span class="string">'\u{ec6}'</span>, <span class="string">'\u{ec6}'</span>,
        SC_OLetter), (<span class="string">'\u{ec8}'</span>, <span class="string">'\u{ece}'</span>, SC_Extend), (<span class="string">'\u{ed0}'</span>, <span class="string">'\u{ed9}'</span>, SC_Numeric),
        (<span class="string">'\u{edc}'</span>, <span class="string">'\u{edf}'</span>, SC_OLetter), (<span class="string">'\u{f00}'</span>, <span class="string">'\u{f00}'</span>, SC_OLetter), (<span class="string">'\u{f18}'</span>,
        <span class="string">'\u{f19}'</span>, SC_Extend), (<span class="string">'\u{f20}'</span>, <span class="string">'\u{f29}'</span>, SC_Numeric), (<span class="string">'\u{f35}'</span>, <span class="string">'\u{f35}'</span>,
        SC_Extend), (<span class="string">'\u{f37}'</span>, <span class="string">'\u{f37}'</span>, SC_Extend), (<span class="string">'\u{f39}'</span>, <span class="string">'\u{f39}'</span>, SC_Extend),
        (<span class="string">'\u{f3a}'</span>, <span class="string">'\u{f3d}'</span>, SC_Close), (<span class="string">'\u{f3e}'</span>, <span class="string">'\u{f3f}'</span>, SC_Extend), (<span class="string">'\u{f40}'</span>, <span class="string">'\u{f47}'</span>,
        SC_OLetter), (<span class="string">'\u{f49}'</span>, <span class="string">'\u{f6c}'</span>, SC_OLetter), (<span class="string">'\u{f71}'</span>, <span class="string">'\u{f84}'</span>, SC_Extend),
        (<span class="string">'\u{f86}'</span>, <span class="string">'\u{f87}'</span>, SC_Extend), (<span class="string">'\u{f88}'</span>, <span class="string">'\u{f8c}'</span>, SC_OLetter), (<span class="string">'\u{f8d}'</span>,
        <span class="string">'\u{f97}'</span>, SC_Extend), (<span class="string">'\u{f99}'</span>, <span class="string">'\u{fbc}'</span>, SC_Extend), (<span class="string">'\u{fc6}'</span>, <span class="string">'\u{fc6}'</span>, SC_Extend),
        (<span class="string">'\u{1000}'</span>, <span class="string">'\u{102a}'</span>, SC_OLetter), (<span class="string">'\u{102b}'</span>, <span class="string">'\u{103e}'</span>, SC_Extend), (<span class="string">'\u{103f}'</span>,
        <span class="string">'\u{103f}'</span>, SC_OLetter), (<span class="string">'\u{1040}'</span>, <span class="string">'\u{1049}'</span>, SC_Numeric), (<span class="string">'\u{104a}'</span>, <span class="string">'\u{104b}'</span>,
        SC_STerm), (<span class="string">'\u{1050}'</span>, <span class="string">'\u{1055}'</span>, SC_OLetter), (<span class="string">'\u{1056}'</span>, <span class="string">'\u{1059}'</span>, SC_Extend),
        (<span class="string">'\u{105a}'</span>, <span class="string">'\u{105d}'</span>, SC_OLetter), (<span class="string">'\u{105e}'</span>, <span class="string">'\u{1060}'</span>, SC_Extend), (<span class="string">'\u{1061}'</span>,
        <span class="string">'\u{1061}'</span>, SC_OLetter), (<span class="string">'\u{1062}'</span>, <span class="string">'\u{1064}'</span>, SC_Extend), (<span class="string">'\u{1065}'</span>, <span class="string">'\u{1066}'</span>,
        SC_OLetter), (<span class="string">'\u{1067}'</span>, <span class="string">'\u{106d}'</span>, SC_Extend), (<span class="string">'\u{106e}'</span>, <span class="string">'\u{1070}'</span>, SC_OLetter),
        (<span class="string">'\u{1071}'</span>, <span class="string">'\u{1074}'</span>, SC_Extend), (<span class="string">'\u{1075}'</span>, <span class="string">'\u{1081}'</span>, SC_OLetter), (<span class="string">'\u{1082}'</span>,
        <span class="string">'\u{108d}'</span>, SC_Extend), (<span class="string">'\u{108e}'</span>, <span class="string">'\u{108e}'</span>, SC_OLetter), (<span class="string">'\u{108f}'</span>, <span class="string">'\u{108f}'</span>,
        SC_Extend), (<span class="string">'\u{1090}'</span>, <span class="string">'\u{1099}'</span>, SC_Numeric), (<span class="string">'\u{109a}'</span>, <span class="string">'\u{109d}'</span>, SC_Extend),
        (<span class="string">'\u{10a0}'</span>, <span class="string">'\u{10c5}'</span>, SC_Upper), (<span class="string">'\u{10c7}'</span>, <span class="string">'\u{10c7}'</span>, SC_Upper), (<span class="string">'\u{10cd}'</span>,
        <span class="string">'\u{10cd}'</span>, SC_Upper), (<span class="string">'\u{10d0}'</span>, <span class="string">'\u{10fa}'</span>, SC_OLetter), (<span class="string">'\u{10fc}'</span>, <span class="string">'\u{10fc}'</span>,
        SC_Lower), (<span class="string">'\u{10fd}'</span>, <span class="string">'\u{1248}'</span>, SC_OLetter), (<span class="string">'\u{124a}'</span>, <span class="string">'\u{124d}'</span>, SC_OLetter),
        (<span class="string">'\u{1250}'</span>, <span class="string">'\u{1256}'</span>, SC_OLetter), (<span class="string">'\u{1258}'</span>, <span class="string">'\u{1258}'</span>, SC_OLetter), (<span class="string">'\u{125a}'</span>,
        <span class="string">'\u{125d}'</span>, SC_OLetter), (<span class="string">'\u{1260}'</span>, <span class="string">'\u{1288}'</span>, SC_OLetter), (<span class="string">'\u{128a}'</span>, <span class="string">'\u{128d}'</span>,
        SC_OLetter), (<span class="string">'\u{1290}'</span>, <span class="string">'\u{12b0}'</span>, SC_OLetter), (<span class="string">'\u{12b2}'</span>, <span class="string">'\u{12b5}'</span>, SC_OLetter),
        (<span class="string">'\u{12b8}'</span>, <span class="string">'\u{12be}'</span>, SC_OLetter), (<span class="string">'\u{12c0}'</span>, <span class="string">'\u{12c0}'</span>, SC_OLetter), (<span class="string">'\u{12c2}'</span>,
        <span class="string">'\u{12c5}'</span>, SC_OLetter), (<span class="string">'\u{12c8}'</span>, <span class="string">'\u{12d6}'</span>, SC_OLetter), (<span class="string">'\u{12d8}'</span>, <span class="string">'\u{1310}'</span>,
        SC_OLetter), (<span class="string">'\u{1312}'</span>, <span class="string">'\u{1315}'</span>, SC_OLetter), (<span class="string">'\u{1318}'</span>, <span class="string">'\u{135a}'</span>, SC_OLetter),
        (<span class="string">'\u{135d}'</span>, <span class="string">'\u{135f}'</span>, SC_Extend), (<span class="string">'\u{1362}'</span>, <span class="string">'\u{1362}'</span>, SC_STerm), (<span class="string">'\u{1367}'</span>,
        <span class="string">'\u{1368}'</span>, SC_STerm), (<span class="string">'\u{1380}'</span>, <span class="string">'\u{138f}'</span>, SC_OLetter), (<span class="string">'\u{13a0}'</span>, <span class="string">'\u{13f5}'</span>,
        SC_Upper), (<span class="string">'\u{13f8}'</span>, <span class="string">'\u{13fd}'</span>, SC_Lower), (<span class="string">'\u{1401}'</span>, <span class="string">'\u{166c}'</span>, SC_OLetter),
        (<span class="string">'\u{166e}'</span>, <span class="string">'\u{166e}'</span>, SC_STerm), (<span class="string">'\u{166f}'</span>, <span class="string">'\u{167f}'</span>, SC_OLetter), (<span class="string">'\u{1680}'</span>,
        <span class="string">'\u{1680}'</span>, SC_Sp), (<span class="string">'\u{1681}'</span>, <span class="string">'\u{169a}'</span>, SC_OLetter), (<span class="string">'\u{169b}'</span>, <span class="string">'\u{169c}'</span>,
        SC_Close), (<span class="string">'\u{16a0}'</span>, <span class="string">'\u{16ea}'</span>, SC_OLetter), (<span class="string">'\u{16ee}'</span>, <span class="string">'\u{16f8}'</span>, SC_OLetter),
        (<span class="string">'\u{1700}'</span>, <span class="string">'\u{1711}'</span>, SC_OLetter), (<span class="string">'\u{1712}'</span>, <span class="string">'\u{1715}'</span>, SC_Extend), (<span class="string">'\u{171f}'</span>,
        <span class="string">'\u{1731}'</span>, SC_OLetter), (<span class="string">'\u{1732}'</span>, <span class="string">'\u{1734}'</span>, SC_Extend), (<span class="string">'\u{1735}'</span>, <span class="string">'\u{1736}'</span>,
        SC_STerm), (<span class="string">'\u{1740}'</span>, <span class="string">'\u{1751}'</span>, SC_OLetter), (<span class="string">'\u{1752}'</span>, <span class="string">'\u{1753}'</span>, SC_Extend),
        (<span class="string">'\u{1760}'</span>, <span class="string">'\u{176c}'</span>, SC_OLetter), (<span class="string">'\u{176e}'</span>, <span class="string">'\u{1770}'</span>, SC_OLetter), (<span class="string">'\u{1772}'</span>,
        <span class="string">'\u{1773}'</span>, SC_Extend), (<span class="string">'\u{1780}'</span>, <span class="string">'\u{17b3}'</span>, SC_OLetter), (<span class="string">'\u{17b4}'</span>, <span class="string">'\u{17d3}'</span>,
        SC_Extend), (<span class="string">'\u{17d4}'</span>, <span class="string">'\u{17d5}'</span>, SC_STerm), (<span class="string">'\u{17d7}'</span>, <span class="string">'\u{17d7}'</span>, SC_OLetter),
        (<span class="string">'\u{17dc}'</span>, <span class="string">'\u{17dc}'</span>, SC_OLetter), (<span class="string">'\u{17dd}'</span>, <span class="string">'\u{17dd}'</span>, SC_Extend), (<span class="string">'\u{17e0}'</span>,
        <span class="string">'\u{17e9}'</span>, SC_Numeric), (<span class="string">'\u{1802}'</span>, <span class="string">'\u{1802}'</span>, SC_SContinue), (<span class="string">'\u{1803}'</span>, <span class="string">'\u{1803}'</span>,
        SC_STerm), (<span class="string">'\u{1808}'</span>, <span class="string">'\u{1808}'</span>, SC_SContinue), (<span class="string">'\u{1809}'</span>, <span class="string">'\u{1809}'</span>, SC_STerm),
        (<span class="string">'\u{180b}'</span>, <span class="string">'\u{180d}'</span>, SC_Extend), (<span class="string">'\u{180e}'</span>, <span class="string">'\u{180e}'</span>, SC_Format), (<span class="string">'\u{180f}'</span>,
        <span class="string">'\u{180f}'</span>, SC_Extend), (<span class="string">'\u{1810}'</span>, <span class="string">'\u{1819}'</span>, SC_Numeric), (<span class="string">'\u{1820}'</span>, <span class="string">'\u{1878}'</span>,
        SC_OLetter), (<span class="string">'\u{1880}'</span>, <span class="string">'\u{1884}'</span>, SC_OLetter), (<span class="string">'\u{1885}'</span>, <span class="string">'\u{1886}'</span>, SC_Extend),
        (<span class="string">'\u{1887}'</span>, <span class="string">'\u{18a8}'</span>, SC_OLetter), (<span class="string">'\u{18a9}'</span>, <span class="string">'\u{18a9}'</span>, SC_Extend), (<span class="string">'\u{18aa}'</span>,
        <span class="string">'\u{18aa}'</span>, SC_OLetter), (<span class="string">'\u{18b0}'</span>, <span class="string">'\u{18f5}'</span>, SC_OLetter), (<span class="string">'\u{1900}'</span>, <span class="string">'\u{191e}'</span>,
        SC_OLetter), (<span class="string">'\u{1920}'</span>, <span class="string">'\u{192b}'</span>, SC_Extend), (<span class="string">'\u{1930}'</span>, <span class="string">'\u{193b}'</span>, SC_Extend),
        (<span class="string">'\u{1944}'</span>, <span class="string">'\u{1945}'</span>, SC_STerm), (<span class="string">'\u{1946}'</span>, <span class="string">'\u{194f}'</span>, SC_Numeric), (<span class="string">'\u{1950}'</span>,
        <span class="string">'\u{196d}'</span>, SC_OLetter), (<span class="string">'\u{1970}'</span>, <span class="string">'\u{1974}'</span>, SC_OLetter), (<span class="string">'\u{1980}'</span>, <span class="string">'\u{19ab}'</span>,
        SC_OLetter), (<span class="string">'\u{19b0}'</span>, <span class="string">'\u{19c9}'</span>, SC_OLetter), (<span class="string">'\u{19d0}'</span>, <span class="string">'\u{19d9}'</span>, SC_Numeric),
        (<span class="string">'\u{1a00}'</span>, <span class="string">'\u{1a16}'</span>, SC_OLetter), (<span class="string">'\u{1a17}'</span>, <span class="string">'\u{1a1b}'</span>, SC_Extend), (<span class="string">'\u{1a20}'</span>,
        <span class="string">'\u{1a54}'</span>, SC_OLetter), (<span class="string">'\u{1a55}'</span>, <span class="string">'\u{1a5e}'</span>, SC_Extend), (<span class="string">'\u{1a60}'</span>, <span class="string">'\u{1a7c}'</span>,
        SC_Extend), (<span class="string">'\u{1a7f}'</span>, <span class="string">'\u{1a7f}'</span>, SC_Extend), (<span class="string">'\u{1a80}'</span>, <span class="string">'\u{1a89}'</span>, SC_Numeric),
        (<span class="string">'\u{1a90}'</span>, <span class="string">'\u{1a99}'</span>, SC_Numeric), (<span class="string">'\u{1aa7}'</span>, <span class="string">'\u{1aa7}'</span>, SC_OLetter), (<span class="string">'\u{1aa8}'</span>,
        <span class="string">'\u{1aab}'</span>, SC_STerm), (<span class="string">'\u{1ab0}'</span>, <span class="string">'\u{1ace}'</span>, SC_Extend), (<span class="string">'\u{1b00}'</span>, <span class="string">'\u{1b04}'</span>,
        SC_Extend), (<span class="string">'\u{1b05}'</span>, <span class="string">'\u{1b33}'</span>, SC_OLetter), (<span class="string">'\u{1b34}'</span>, <span class="string">'\u{1b44}'</span>, SC_Extend),
        (<span class="string">'\u{1b45}'</span>, <span class="string">'\u{1b4c}'</span>, SC_OLetter), (<span class="string">'\u{1b50}'</span>, <span class="string">'\u{1b59}'</span>, SC_Numeric), (<span class="string">'\u{1b5a}'</span>,
        <span class="string">'\u{1b5b}'</span>, SC_STerm), (<span class="string">'\u{1b5e}'</span>, <span class="string">'\u{1b5f}'</span>, SC_STerm), (<span class="string">'\u{1b6b}'</span>, <span class="string">'\u{1b73}'</span>,
        SC_Extend), (<span class="string">'\u{1b7d}'</span>, <span class="string">'\u{1b7e}'</span>, SC_STerm), (<span class="string">'\u{1b80}'</span>, <span class="string">'\u{1b82}'</span>, SC_Extend),
        (<span class="string">'\u{1b83}'</span>, <span class="string">'\u{1ba0}'</span>, SC_OLetter), (<span class="string">'\u{1ba1}'</span>, <span class="string">'\u{1bad}'</span>, SC_Extend), (<span class="string">'\u{1bae}'</span>,
        <span class="string">'\u{1baf}'</span>, SC_OLetter), (<span class="string">'\u{1bb0}'</span>, <span class="string">'\u{1bb9}'</span>, SC_Numeric), (<span class="string">'\u{1bba}'</span>, <span class="string">'\u{1be5}'</span>,
        SC_OLetter), (<span class="string">'\u{1be6}'</span>, <span class="string">'\u{1bf3}'</span>, SC_Extend), (<span class="string">'\u{1c00}'</span>, <span class="string">'\u{1c23}'</span>, SC_OLetter),
        (<span class="string">'\u{1c24}'</span>, <span class="string">'\u{1c37}'</span>, SC_Extend), (<span class="string">'\u{1c3b}'</span>, <span class="string">'\u{1c3c}'</span>, SC_STerm), (<span class="string">'\u{1c40}'</span>,
        <span class="string">'\u{1c49}'</span>, SC_Numeric), (<span class="string">'\u{1c4d}'</span>, <span class="string">'\u{1c4f}'</span>, SC_OLetter), (<span class="string">'\u{1c50}'</span>, <span class="string">'\u{1c59}'</span>,
        SC_Numeric), (<span class="string">'\u{1c5a}'</span>, <span class="string">'\u{1c7d}'</span>, SC_OLetter), (<span class="string">'\u{1c7e}'</span>, <span class="string">'\u{1c7f}'</span>, SC_STerm),
        (<span class="string">'\u{1c80}'</span>, <span class="string">'\u{1c88}'</span>, SC_Lower), (<span class="string">'\u{1c90}'</span>, <span class="string">'\u{1cba}'</span>, SC_OLetter), (<span class="string">'\u{1cbd}'</span>,
        <span class="string">'\u{1cbf}'</span>, SC_OLetter), (<span class="string">'\u{1cd0}'</span>, <span class="string">'\u{1cd2}'</span>, SC_Extend), (<span class="string">'\u{1cd4}'</span>, <span class="string">'\u{1ce8}'</span>,
        SC_Extend), (<span class="string">'\u{1ce9}'</span>, <span class="string">'\u{1cec}'</span>, SC_OLetter), (<span class="string">'\u{1ced}'</span>, <span class="string">'\u{1ced}'</span>, SC_Extend),
        (<span class="string">'\u{1cee}'</span>, <span class="string">'\u{1cf3}'</span>, SC_OLetter), (<span class="string">'\u{1cf4}'</span>, <span class="string">'\u{1cf4}'</span>, SC_Extend), (<span class="string">'\u{1cf5}'</span>,
        <span class="string">'\u{1cf6}'</span>, SC_OLetter), (<span class="string">'\u{1cf7}'</span>, <span class="string">'\u{1cf9}'</span>, SC_Extend), (<span class="string">'\u{1cfa}'</span>, <span class="string">'\u{1cfa}'</span>,
        SC_OLetter), (<span class="string">'\u{1d00}'</span>, <span class="string">'\u{1dbf}'</span>, SC_Lower), (<span class="string">'\u{1dc0}'</span>, <span class="string">'\u{1dff}'</span>, SC_Extend),
        (<span class="string">'\u{1e00}'</span>, <span class="string">'\u{1e00}'</span>, SC_Upper), (<span class="string">'\u{1e01}'</span>, <span class="string">'\u{1e01}'</span>, SC_Lower), (<span class="string">'\u{1e02}'</span>,
        <span class="string">'\u{1e02}'</span>, SC_Upper), (<span class="string">'\u{1e03}'</span>, <span class="string">'\u{1e03}'</span>, SC_Lower), (<span class="string">'\u{1e04}'</span>, <span class="string">'\u{1e04}'</span>,
        SC_Upper), (<span class="string">'\u{1e05}'</span>, <span class="string">'\u{1e05}'</span>, SC_Lower), (<span class="string">'\u{1e06}'</span>, <span class="string">'\u{1e06}'</span>, SC_Upper),
        (<span class="string">'\u{1e07}'</span>, <span class="string">'\u{1e07}'</span>, SC_Lower), (<span class="string">'\u{1e08}'</span>, <span class="string">'\u{1e08}'</span>, SC_Upper), (<span class="string">'\u{1e09}'</span>,
        <span class="string">'\u{1e09}'</span>, SC_Lower), (<span class="string">'\u{1e0a}'</span>, <span class="string">'\u{1e0a}'</span>, SC_Upper), (<span class="string">'\u{1e0b}'</span>, <span class="string">'\u{1e0b}'</span>,
        SC_Lower), (<span class="string">'\u{1e0c}'</span>, <span class="string">'\u{1e0c}'</span>, SC_Upper), (<span class="string">'\u{1e0d}'</span>, <span class="string">'\u{1e0d}'</span>, SC_Lower),
        (<span class="string">'\u{1e0e}'</span>, <span class="string">'\u{1e0e}'</span>, SC_Upper), (<span class="string">'\u{1e0f}'</span>, <span class="string">'\u{1e0f}'</span>, SC_Lower), (<span class="string">'\u{1e10}'</span>,
        <span class="string">'\u{1e10}'</span>, SC_Upper), (<span class="string">'\u{1e11}'</span>, <span class="string">'\u{1e11}'</span>, SC_Lower), (<span class="string">'\u{1e12}'</span>, <span class="string">'\u{1e12}'</span>,
        SC_Upper), (<span class="string">'\u{1e13}'</span>, <span class="string">'\u{1e13}'</span>, SC_Lower), (<span class="string">'\u{1e14}'</span>, <span class="string">'\u{1e14}'</span>, SC_Upper),
        (<span class="string">'\u{1e15}'</span>, <span class="string">'\u{1e15}'</span>, SC_Lower), (<span class="string">'\u{1e16}'</span>, <span class="string">'\u{1e16}'</span>, SC_Upper), (<span class="string">'\u{1e17}'</span>,
        <span class="string">'\u{1e17}'</span>, SC_Lower), (<span class="string">'\u{1e18}'</span>, <span class="string">'\u{1e18}'</span>, SC_Upper), (<span class="string">'\u{1e19}'</span>, <span class="string">'\u{1e19}'</span>,
        SC_Lower), (<span class="string">'\u{1e1a}'</span>, <span class="string">'\u{1e1a}'</span>, SC_Upper), (<span class="string">'\u{1e1b}'</span>, <span class="string">'\u{1e1b}'</span>, SC_Lower),
        (<span class="string">'\u{1e1c}'</span>, <span class="string">'\u{1e1c}'</span>, SC_Upper), (<span class="string">'\u{1e1d}'</span>, <span class="string">'\u{1e1d}'</span>, SC_Lower), (<span class="string">'\u{1e1e}'</span>,
        <span class="string">'\u{1e1e}'</span>, SC_Upper), (<span class="string">'\u{1e1f}'</span>, <span class="string">'\u{1e1f}'</span>, SC_Lower), (<span class="string">'\u{1e20}'</span>, <span class="string">'\u{1e20}'</span>,
        SC_Upper), (<span class="string">'\u{1e21}'</span>, <span class="string">'\u{1e21}'</span>, SC_Lower), (<span class="string">'\u{1e22}'</span>, <span class="string">'\u{1e22}'</span>, SC_Upper),
        (<span class="string">'\u{1e23}'</span>, <span class="string">'\u{1e23}'</span>, SC_Lower), (<span class="string">'\u{1e24}'</span>, <span class="string">'\u{1e24}'</span>, SC_Upper), (<span class="string">'\u{1e25}'</span>,
        <span class="string">'\u{1e25}'</span>, SC_Lower), (<span class="string">'\u{1e26}'</span>, <span class="string">'\u{1e26}'</span>, SC_Upper), (<span class="string">'\u{1e27}'</span>, <span class="string">'\u{1e27}'</span>,
        SC_Lower), (<span class="string">'\u{1e28}'</span>, <span class="string">'\u{1e28}'</span>, SC_Upper), (<span class="string">'\u{1e29}'</span>, <span class="string">'\u{1e29}'</span>, SC_Lower),
        (<span class="string">'\u{1e2a}'</span>, <span class="string">'\u{1e2a}'</span>, SC_Upper), (<span class="string">'\u{1e2b}'</span>, <span class="string">'\u{1e2b}'</span>, SC_Lower), (<span class="string">'\u{1e2c}'</span>,
        <span class="string">'\u{1e2c}'</span>, SC_Upper), (<span class="string">'\u{1e2d}'</span>, <span class="string">'\u{1e2d}'</span>, SC_Lower), (<span class="string">'\u{1e2e}'</span>, <span class="string">'\u{1e2e}'</span>,
        SC_Upper), (<span class="string">'\u{1e2f}'</span>, <span class="string">'\u{1e2f}'</span>, SC_Lower), (<span class="string">'\u{1e30}'</span>, <span class="string">'\u{1e30}'</span>, SC_Upper),
        (<span class="string">'\u{1e31}'</span>, <span class="string">'\u{1e31}'</span>, SC_Lower), (<span class="string">'\u{1e32}'</span>, <span class="string">'\u{1e32}'</span>, SC_Upper), (<span class="string">'\u{1e33}'</span>,
        <span class="string">'\u{1e33}'</span>, SC_Lower), (<span class="string">'\u{1e34}'</span>, <span class="string">'\u{1e34}'</span>, SC_Upper), (<span class="string">'\u{1e35}'</span>, <span class="string">'\u{1e35}'</span>,
        SC_Lower), (<span class="string">'\u{1e36}'</span>, <span class="string">'\u{1e36}'</span>, SC_Upper), (<span class="string">'\u{1e37}'</span>, <span class="string">'\u{1e37}'</span>, SC_Lower),
        (<span class="string">'\u{1e38}'</span>, <span class="string">'\u{1e38}'</span>, SC_Upper), (<span class="string">'\u{1e39}'</span>, <span class="string">'\u{1e39}'</span>, SC_Lower), (<span class="string">'\u{1e3a}'</span>,
        <span class="string">'\u{1e3a}'</span>, SC_Upper), (<span class="string">'\u{1e3b}'</span>, <span class="string">'\u{1e3b}'</span>, SC_Lower), (<span class="string">'\u{1e3c}'</span>, <span class="string">'\u{1e3c}'</span>,
        SC_Upper), (<span class="string">'\u{1e3d}'</span>, <span class="string">'\u{1e3d}'</span>, SC_Lower), (<span class="string">'\u{1e3e}'</span>, <span class="string">'\u{1e3e}'</span>, SC_Upper),
        (<span class="string">'\u{1e3f}'</span>, <span class="string">'\u{1e3f}'</span>, SC_Lower), (<span class="string">'\u{1e40}'</span>, <span class="string">'\u{1e40}'</span>, SC_Upper), (<span class="string">'\u{1e41}'</span>,
        <span class="string">'\u{1e41}'</span>, SC_Lower), (<span class="string">'\u{1e42}'</span>, <span class="string">'\u{1e42}'</span>, SC_Upper), (<span class="string">'\u{1e43}'</span>, <span class="string">'\u{1e43}'</span>,
        SC_Lower), (<span class="string">'\u{1e44}'</span>, <span class="string">'\u{1e44}'</span>, SC_Upper), (<span class="string">'\u{1e45}'</span>, <span class="string">'\u{1e45}'</span>, SC_Lower),
        (<span class="string">'\u{1e46}'</span>, <span class="string">'\u{1e46}'</span>, SC_Upper), (<span class="string">'\u{1e47}'</span>, <span class="string">'\u{1e47}'</span>, SC_Lower), (<span class="string">'\u{1e48}'</span>,
        <span class="string">'\u{1e48}'</span>, SC_Upper), (<span class="string">'\u{1e49}'</span>, <span class="string">'\u{1e49}'</span>, SC_Lower), (<span class="string">'\u{1e4a}'</span>, <span class="string">'\u{1e4a}'</span>,
        SC_Upper), (<span class="string">'\u{1e4b}'</span>, <span class="string">'\u{1e4b}'</span>, SC_Lower), (<span class="string">'\u{1e4c}'</span>, <span class="string">'\u{1e4c}'</span>, SC_Upper),
        (<span class="string">'\u{1e4d}'</span>, <span class="string">'\u{1e4d}'</span>, SC_Lower), (<span class="string">'\u{1e4e}'</span>, <span class="string">'\u{1e4e}'</span>, SC_Upper), (<span class="string">'\u{1e4f}'</span>,
        <span class="string">'\u{1e4f}'</span>, SC_Lower), (<span class="string">'\u{1e50}'</span>, <span class="string">'\u{1e50}'</span>, SC_Upper), (<span class="string">'\u{1e51}'</span>, <span class="string">'\u{1e51}'</span>,
        SC_Lower), (<span class="string">'\u{1e52}'</span>, <span class="string">'\u{1e52}'</span>, SC_Upper), (<span class="string">'\u{1e53}'</span>, <span class="string">'\u{1e53}'</span>, SC_Lower),
        (<span class="string">'\u{1e54}'</span>, <span class="string">'\u{1e54}'</span>, SC_Upper), (<span class="string">'\u{1e55}'</span>, <span class="string">'\u{1e55}'</span>, SC_Lower), (<span class="string">'\u{1e56}'</span>,
        <span class="string">'\u{1e56}'</span>, SC_Upper), (<span class="string">'\u{1e57}'</span>, <span class="string">'\u{1e57}'</span>, SC_Lower), (<span class="string">'\u{1e58}'</span>, <span class="string">'\u{1e58}'</span>,
        SC_Upper), (<span class="string">'\u{1e59}'</span>, <span class="string">'\u{1e59}'</span>, SC_Lower), (<span class="string">'\u{1e5a}'</span>, <span class="string">'\u{1e5a}'</span>, SC_Upper),
        (<span class="string">'\u{1e5b}'</span>, <span class="string">'\u{1e5b}'</span>, SC_Lower), (<span class="string">'\u{1e5c}'</span>, <span class="string">'\u{1e5c}'</span>, SC_Upper), (<span class="string">'\u{1e5d}'</span>,
        <span class="string">'\u{1e5d}'</span>, SC_Lower), (<span class="string">'\u{1e5e}'</span>, <span class="string">'\u{1e5e}'</span>, SC_Upper), (<span class="string">'\u{1e5f}'</span>, <span class="string">'\u{1e5f}'</span>,
        SC_Lower), (<span class="string">'\u{1e60}'</span>, <span class="string">'\u{1e60}'</span>, SC_Upper), (<span class="string">'\u{1e61}'</span>, <span class="string">'\u{1e61}'</span>, SC_Lower),
        (<span class="string">'\u{1e62}'</span>, <span class="string">'\u{1e62}'</span>, SC_Upper), (<span class="string">'\u{1e63}'</span>, <span class="string">'\u{1e63}'</span>, SC_Lower), (<span class="string">'\u{1e64}'</span>,
        <span class="string">'\u{1e64}'</span>, SC_Upper), (<span class="string">'\u{1e65}'</span>, <span class="string">'\u{1e65}'</span>, SC_Lower), (<span class="string">'\u{1e66}'</span>, <span class="string">'\u{1e66}'</span>,
        SC_Upper), (<span class="string">'\u{1e67}'</span>, <span class="string">'\u{1e67}'</span>, SC_Lower), (<span class="string">'\u{1e68}'</span>, <span class="string">'\u{1e68}'</span>, SC_Upper),
        (<span class="string">'\u{1e69}'</span>, <span class="string">'\u{1e69}'</span>, SC_Lower), (<span class="string">'\u{1e6a}'</span>, <span class="string">'\u{1e6a}'</span>, SC_Upper), (<span class="string">'\u{1e6b}'</span>,
        <span class="string">'\u{1e6b}'</span>, SC_Lower), (<span class="string">'\u{1e6c}'</span>, <span class="string">'\u{1e6c}'</span>, SC_Upper), (<span class="string">'\u{1e6d}'</span>, <span class="string">'\u{1e6d}'</span>,
        SC_Lower), (<span class="string">'\u{1e6e}'</span>, <span class="string">'\u{1e6e}'</span>, SC_Upper), (<span class="string">'\u{1e6f}'</span>, <span class="string">'\u{1e6f}'</span>, SC_Lower),
        (<span class="string">'\u{1e70}'</span>, <span class="string">'\u{1e70}'</span>, SC_Upper), (<span class="string">'\u{1e71}'</span>, <span class="string">'\u{1e71}'</span>, SC_Lower), (<span class="string">'\u{1e72}'</span>,
        <span class="string">'\u{1e72}'</span>, SC_Upper), (<span class="string">'\u{1e73}'</span>, <span class="string">'\u{1e73}'</span>, SC_Lower), (<span class="string">'\u{1e74}'</span>, <span class="string">'\u{1e74}'</span>,
        SC_Upper), (<span class="string">'\u{1e75}'</span>, <span class="string">'\u{1e75}'</span>, SC_Lower), (<span class="string">'\u{1e76}'</span>, <span class="string">'\u{1e76}'</span>, SC_Upper),
        (<span class="string">'\u{1e77}'</span>, <span class="string">'\u{1e77}'</span>, SC_Lower), (<span class="string">'\u{1e78}'</span>, <span class="string">'\u{1e78}'</span>, SC_Upper), (<span class="string">'\u{1e79}'</span>,
        <span class="string">'\u{1e79}'</span>, SC_Lower), (<span class="string">'\u{1e7a}'</span>, <span class="string">'\u{1e7a}'</span>, SC_Upper), (<span class="string">'\u{1e7b}'</span>, <span class="string">'\u{1e7b}'</span>,
        SC_Lower), (<span class="string">'\u{1e7c}'</span>, <span class="string">'\u{1e7c}'</span>, SC_Upper), (<span class="string">'\u{1e7d}'</span>, <span class="string">'\u{1e7d}'</span>, SC_Lower),
        (<span class="string">'\u{1e7e}'</span>, <span class="string">'\u{1e7e}'</span>, SC_Upper), (<span class="string">'\u{1e7f}'</span>, <span class="string">'\u{1e7f}'</span>, SC_Lower), (<span class="string">'\u{1e80}'</span>,
        <span class="string">'\u{1e80}'</span>, SC_Upper), (<span class="string">'\u{1e81}'</span>, <span class="string">'\u{1e81}'</span>, SC_Lower), (<span class="string">'\u{1e82}'</span>, <span class="string">'\u{1e82}'</span>,
        SC_Upper), (<span class="string">'\u{1e83}'</span>, <span class="string">'\u{1e83}'</span>, SC_Lower), (<span class="string">'\u{1e84}'</span>, <span class="string">'\u{1e84}'</span>, SC_Upper),
        (<span class="string">'\u{1e85}'</span>, <span class="string">'\u{1e85}'</span>, SC_Lower), (<span class="string">'\u{1e86}'</span>, <span class="string">'\u{1e86}'</span>, SC_Upper), (<span class="string">'\u{1e87}'</span>,
        <span class="string">'\u{1e87}'</span>, SC_Lower), (<span class="string">'\u{1e88}'</span>, <span class="string">'\u{1e88}'</span>, SC_Upper), (<span class="string">'\u{1e89}'</span>, <span class="string">'\u{1e89}'</span>,
        SC_Lower), (<span class="string">'\u{1e8a}'</span>, <span class="string">'\u{1e8a}'</span>, SC_Upper), (<span class="string">'\u{1e8b}'</span>, <span class="string">'\u{1e8b}'</span>, SC_Lower),
        (<span class="string">'\u{1e8c}'</span>, <span class="string">'\u{1e8c}'</span>, SC_Upper), (<span class="string">'\u{1e8d}'</span>, <span class="string">'\u{1e8d}'</span>, SC_Lower), (<span class="string">'\u{1e8e}'</span>,
        <span class="string">'\u{1e8e}'</span>, SC_Upper), (<span class="string">'\u{1e8f}'</span>, <span class="string">'\u{1e8f}'</span>, SC_Lower), (<span class="string">'\u{1e90}'</span>, <span class="string">'\u{1e90}'</span>,
        SC_Upper), (<span class="string">'\u{1e91}'</span>, <span class="string">'\u{1e91}'</span>, SC_Lower), (<span class="string">'\u{1e92}'</span>, <span class="string">'\u{1e92}'</span>, SC_Upper),
        (<span class="string">'\u{1e93}'</span>, <span class="string">'\u{1e93}'</span>, SC_Lower), (<span class="string">'\u{1e94}'</span>, <span class="string">'\u{1e94}'</span>, SC_Upper), (<span class="string">'\u{1e95}'</span>,
        <span class="string">'\u{1e9d}'</span>, SC_Lower), (<span class="string">'\u{1e9e}'</span>, <span class="string">'\u{1e9e}'</span>, SC_Upper), (<span class="string">'\u{1e9f}'</span>, <span class="string">'\u{1e9f}'</span>,
        SC_Lower), (<span class="string">'\u{1ea0}'</span>, <span class="string">'\u{1ea0}'</span>, SC_Upper), (<span class="string">'\u{1ea1}'</span>, <span class="string">'\u{1ea1}'</span>, SC_Lower),
        (<span class="string">'\u{1ea2}'</span>, <span class="string">'\u{1ea2}'</span>, SC_Upper), (<span class="string">'\u{1ea3}'</span>, <span class="string">'\u{1ea3}'</span>, SC_Lower), (<span class="string">'\u{1ea4}'</span>,
        <span class="string">'\u{1ea4}'</span>, SC_Upper), (<span class="string">'\u{1ea5}'</span>, <span class="string">'\u{1ea5}'</span>, SC_Lower), (<span class="string">'\u{1ea6}'</span>, <span class="string">'\u{1ea6}'</span>,
        SC_Upper), (<span class="string">'\u{1ea7}'</span>, <span class="string">'\u{1ea7}'</span>, SC_Lower), (<span class="string">'\u{1ea8}'</span>, <span class="string">'\u{1ea8}'</span>, SC_Upper),
        (<span class="string">'\u{1ea9}'</span>, <span class="string">'\u{1ea9}'</span>, SC_Lower), (<span class="string">'\u{1eaa}'</span>, <span class="string">'\u{1eaa}'</span>, SC_Upper), (<span class="string">'\u{1eab}'</span>,
        <span class="string">'\u{1eab}'</span>, SC_Lower), (<span class="string">'\u{1eac}'</span>, <span class="string">'\u{1eac}'</span>, SC_Upper), (<span class="string">'\u{1ead}'</span>, <span class="string">'\u{1ead}'</span>,
        SC_Lower), (<span class="string">'\u{1eae}'</span>, <span class="string">'\u{1eae}'</span>, SC_Upper), (<span class="string">'\u{1eaf}'</span>, <span class="string">'\u{1eaf}'</span>, SC_Lower),
        (<span class="string">'\u{1eb0}'</span>, <span class="string">'\u{1eb0}'</span>, SC_Upper), (<span class="string">'\u{1eb1}'</span>, <span class="string">'\u{1eb1}'</span>, SC_Lower), (<span class="string">'\u{1eb2}'</span>,
        <span class="string">'\u{1eb2}'</span>, SC_Upper), (<span class="string">'\u{1eb3}'</span>, <span class="string">'\u{1eb3}'</span>, SC_Lower), (<span class="string">'\u{1eb4}'</span>, <span class="string">'\u{1eb4}'</span>,
        SC_Upper), (<span class="string">'\u{1eb5}'</span>, <span class="string">'\u{1eb5}'</span>, SC_Lower), (<span class="string">'\u{1eb6}'</span>, <span class="string">'\u{1eb6}'</span>, SC_Upper),
        (<span class="string">'\u{1eb7}'</span>, <span class="string">'\u{1eb7}'</span>, SC_Lower), (<span class="string">'\u{1eb8}'</span>, <span class="string">'\u{1eb8}'</span>, SC_Upper), (<span class="string">'\u{1eb9}'</span>,
        <span class="string">'\u{1eb9}'</span>, SC_Lower), (<span class="string">'\u{1eba}'</span>, <span class="string">'\u{1eba}'</span>, SC_Upper), (<span class="string">'\u{1ebb}'</span>, <span class="string">'\u{1ebb}'</span>,
        SC_Lower), (<span class="string">'\u{1ebc}'</span>, <span class="string">'\u{1ebc}'</span>, SC_Upper), (<span class="string">'\u{1ebd}'</span>, <span class="string">'\u{1ebd}'</span>, SC_Lower),
        (<span class="string">'\u{1ebe}'</span>, <span class="string">'\u{1ebe}'</span>, SC_Upper), (<span class="string">'\u{1ebf}'</span>, <span class="string">'\u{1ebf}'</span>, SC_Lower), (<span class="string">'\u{1ec0}'</span>,
        <span class="string">'\u{1ec0}'</span>, SC_Upper), (<span class="string">'\u{1ec1}'</span>, <span class="string">'\u{1ec1}'</span>, SC_Lower), (<span class="string">'\u{1ec2}'</span>, <span class="string">'\u{1ec2}'</span>,
        SC_Upper), (<span class="string">'\u{1ec3}'</span>, <span class="string">'\u{1ec3}'</span>, SC_Lower), (<span class="string">'\u{1ec4}'</span>, <span class="string">'\u{1ec4}'</span>, SC_Upper),
        (<span class="string">'\u{1ec5}'</span>, <span class="string">'\u{1ec5}'</span>, SC_Lower), (<span class="string">'\u{1ec6}'</span>, <span class="string">'\u{1ec6}'</span>, SC_Upper), (<span class="string">'\u{1ec7}'</span>,
        <span class="string">'\u{1ec7}'</span>, SC_Lower), (<span class="string">'\u{1ec8}'</span>, <span class="string">'\u{1ec8}'</span>, SC_Upper), (<span class="string">'\u{1ec9}'</span>, <span class="string">'\u{1ec9}'</span>,
        SC_Lower), (<span class="string">'\u{1eca}'</span>, <span class="string">'\u{1eca}'</span>, SC_Upper), (<span class="string">'\u{1ecb}'</span>, <span class="string">'\u{1ecb}'</span>, SC_Lower),
        (<span class="string">'\u{1ecc}'</span>, <span class="string">'\u{1ecc}'</span>, SC_Upper), (<span class="string">'\u{1ecd}'</span>, <span class="string">'\u{1ecd}'</span>, SC_Lower), (<span class="string">'\u{1ece}'</span>,
        <span class="string">'\u{1ece}'</span>, SC_Upper), (<span class="string">'\u{1ecf}'</span>, <span class="string">'\u{1ecf}'</span>, SC_Lower), (<span class="string">'\u{1ed0}'</span>, <span class="string">'\u{1ed0}'</span>,
        SC_Upper), (<span class="string">'\u{1ed1}'</span>, <span class="string">'\u{1ed1}'</span>, SC_Lower), (<span class="string">'\u{1ed2}'</span>, <span class="string">'\u{1ed2}'</span>, SC_Upper),
        (<span class="string">'\u{1ed3}'</span>, <span class="string">'\u{1ed3}'</span>, SC_Lower), (<span class="string">'\u{1ed4}'</span>, <span class="string">'\u{1ed4}'</span>, SC_Upper), (<span class="string">'\u{1ed5}'</span>,
        <span class="string">'\u{1ed5}'</span>, SC_Lower), (<span class="string">'\u{1ed6}'</span>, <span class="string">'\u{1ed6}'</span>, SC_Upper), (<span class="string">'\u{1ed7}'</span>, <span class="string">'\u{1ed7}'</span>,
        SC_Lower), (<span class="string">'\u{1ed8}'</span>, <span class="string">'\u{1ed8}'</span>, SC_Upper), (<span class="string">'\u{1ed9}'</span>, <span class="string">'\u{1ed9}'</span>, SC_Lower),
        (<span class="string">'\u{1eda}'</span>, <span class="string">'\u{1eda}'</span>, SC_Upper), (<span class="string">'\u{1edb}'</span>, <span class="string">'\u{1edb}'</span>, SC_Lower), (<span class="string">'\u{1edc}'</span>,
        <span class="string">'\u{1edc}'</span>, SC_Upper), (<span class="string">'\u{1edd}'</span>, <span class="string">'\u{1edd}'</span>, SC_Lower), (<span class="string">'\u{1ede}'</span>, <span class="string">'\u{1ede}'</span>,
        SC_Upper), (<span class="string">'\u{1edf}'</span>, <span class="string">'\u{1edf}'</span>, SC_Lower), (<span class="string">'\u{1ee0}'</span>, <span class="string">'\u{1ee0}'</span>, SC_Upper),
        (<span class="string">'\u{1ee1}'</span>, <span class="string">'\u{1ee1}'</span>, SC_Lower), (<span class="string">'\u{1ee2}'</span>, <span class="string">'\u{1ee2}'</span>, SC_Upper), (<span class="string">'\u{1ee3}'</span>,
        <span class="string">'\u{1ee3}'</span>, SC_Lower), (<span class="string">'\u{1ee4}'</span>, <span class="string">'\u{1ee4}'</span>, SC_Upper), (<span class="string">'\u{1ee5}'</span>, <span class="string">'\u{1ee5}'</span>,
        SC_Lower), (<span class="string">'\u{1ee6}'</span>, <span class="string">'\u{1ee6}'</span>, SC_Upper), (<span class="string">'\u{1ee7}'</span>, <span class="string">'\u{1ee7}'</span>, SC_Lower),
        (<span class="string">'\u{1ee8}'</span>, <span class="string">'\u{1ee8}'</span>, SC_Upper), (<span class="string">'\u{1ee9}'</span>, <span class="string">'\u{1ee9}'</span>, SC_Lower), (<span class="string">'\u{1eea}'</span>,
        <span class="string">'\u{1eea}'</span>, SC_Upper), (<span class="string">'\u{1eeb}'</span>, <span class="string">'\u{1eeb}'</span>, SC_Lower), (<span class="string">'\u{1eec}'</span>, <span class="string">'\u{1eec}'</span>,
        SC_Upper), (<span class="string">'\u{1eed}'</span>, <span class="string">'\u{1eed}'</span>, SC_Lower), (<span class="string">'\u{1eee}'</span>, <span class="string">'\u{1eee}'</span>, SC_Upper),
        (<span class="string">'\u{1eef}'</span>, <span class="string">'\u{1eef}'</span>, SC_Lower), (<span class="string">'\u{1ef0}'</span>, <span class="string">'\u{1ef0}'</span>, SC_Upper), (<span class="string">'\u{1ef1}'</span>,
        <span class="string">'\u{1ef1}'</span>, SC_Lower), (<span class="string">'\u{1ef2}'</span>, <span class="string">'\u{1ef2}'</span>, SC_Upper), (<span class="string">'\u{1ef3}'</span>, <span class="string">'\u{1ef3}'</span>,
        SC_Lower), (<span class="string">'\u{1ef4}'</span>, <span class="string">'\u{1ef4}'</span>, SC_Upper), (<span class="string">'\u{1ef5}'</span>, <span class="string">'\u{1ef5}'</span>, SC_Lower),
        (<span class="string">'\u{1ef6}'</span>, <span class="string">'\u{1ef6}'</span>, SC_Upper), (<span class="string">'\u{1ef7}'</span>, <span class="string">'\u{1ef7}'</span>, SC_Lower), (<span class="string">'\u{1ef8}'</span>,
        <span class="string">'\u{1ef8}'</span>, SC_Upper), (<span class="string">'\u{1ef9}'</span>, <span class="string">'\u{1ef9}'</span>, SC_Lower), (<span class="string">'\u{1efa}'</span>, <span class="string">'\u{1efa}'</span>,
        SC_Upper), (<span class="string">'\u{1efb}'</span>, <span class="string">'\u{1efb}'</span>, SC_Lower), (<span class="string">'\u{1efc}'</span>, <span class="string">'\u{1efc}'</span>, SC_Upper),
        (<span class="string">'\u{1efd}'</span>, <span class="string">'\u{1efd}'</span>, SC_Lower), (<span class="string">'\u{1efe}'</span>, <span class="string">'\u{1efe}'</span>, SC_Upper), (<span class="string">'\u{1eff}'</span>,
        <span class="string">'\u{1f07}'</span>, SC_Lower), (<span class="string">'\u{1f08}'</span>, <span class="string">'\u{1f0f}'</span>, SC_Upper), (<span class="string">'\u{1f10}'</span>, <span class="string">'\u{1f15}'</span>,
        SC_Lower), (<span class="string">'\u{1f18}'</span>, <span class="string">'\u{1f1d}'</span>, SC_Upper), (<span class="string">'\u{1f20}'</span>, <span class="string">'\u{1f27}'</span>, SC_Lower),
        (<span class="string">'\u{1f28}'</span>, <span class="string">'\u{1f2f}'</span>, SC_Upper), (<span class="string">'\u{1f30}'</span>, <span class="string">'\u{1f37}'</span>, SC_Lower), (<span class="string">'\u{1f38}'</span>,
        <span class="string">'\u{1f3f}'</span>, SC_Upper), (<span class="string">'\u{1f40}'</span>, <span class="string">'\u{1f45}'</span>, SC_Lower), (<span class="string">'\u{1f48}'</span>, <span class="string">'\u{1f4d}'</span>,
        SC_Upper), (<span class="string">'\u{1f50}'</span>, <span class="string">'\u{1f57}'</span>, SC_Lower), (<span class="string">'\u{1f59}'</span>, <span class="string">'\u{1f59}'</span>, SC_Upper),
        (<span class="string">'\u{1f5b}'</span>, <span class="string">'\u{1f5b}'</span>, SC_Upper), (<span class="string">'\u{1f5d}'</span>, <span class="string">'\u{1f5d}'</span>, SC_Upper), (<span class="string">'\u{1f5f}'</span>,
        <span class="string">'\u{1f5f}'</span>, SC_Upper), (<span class="string">'\u{1f60}'</span>, <span class="string">'\u{1f67}'</span>, SC_Lower), (<span class="string">'\u{1f68}'</span>, <span class="string">'\u{1f6f}'</span>,
        SC_Upper), (<span class="string">'\u{1f70}'</span>, <span class="string">'\u{1f7d}'</span>, SC_Lower), (<span class="string">'\u{1f80}'</span>, <span class="string">'\u{1f87}'</span>, SC_Lower),
        (<span class="string">'\u{1f88}'</span>, <span class="string">'\u{1f8f}'</span>, SC_Upper), (<span class="string">'\u{1f90}'</span>, <span class="string">'\u{1f97}'</span>, SC_Lower), (<span class="string">'\u{1f98}'</span>,
        <span class="string">'\u{1f9f}'</span>, SC_Upper), (<span class="string">'\u{1fa0}'</span>, <span class="string">'\u{1fa7}'</span>, SC_Lower), (<span class="string">'\u{1fa8}'</span>, <span class="string">'\u{1faf}'</span>,
        SC_Upper), (<span class="string">'\u{1fb0}'</span>, <span class="string">'\u{1fb4}'</span>, SC_Lower), (<span class="string">'\u{1fb6}'</span>, <span class="string">'\u{1fb7}'</span>, SC_Lower),
        (<span class="string">'\u{1fb8}'</span>, <span class="string">'\u{1fbc}'</span>, SC_Upper), (<span class="string">'\u{1fbe}'</span>, <span class="string">'\u{1fbe}'</span>, SC_Lower), (<span class="string">'\u{1fc2}'</span>,
        <span class="string">'\u{1fc4}'</span>, SC_Lower), (<span class="string">'\u{1fc6}'</span>, <span class="string">'\u{1fc7}'</span>, SC_Lower), (<span class="string">'\u{1fc8}'</span>, <span class="string">'\u{1fcc}'</span>,
        SC_Upper), (<span class="string">'\u{1fd0}'</span>, <span class="string">'\u{1fd3}'</span>, SC_Lower), (<span class="string">'\u{1fd6}'</span>, <span class="string">'\u{1fd7}'</span>, SC_Lower),
        (<span class="string">'\u{1fd8}'</span>, <span class="string">'\u{1fdb}'</span>, SC_Upper), (<span class="string">'\u{1fe0}'</span>, <span class="string">'\u{1fe7}'</span>, SC_Lower), (<span class="string">'\u{1fe8}'</span>,
        <span class="string">'\u{1fec}'</span>, SC_Upper), (<span class="string">'\u{1ff2}'</span>, <span class="string">'\u{1ff4}'</span>, SC_Lower), (<span class="string">'\u{1ff6}'</span>, <span class="string">'\u{1ff7}'</span>,
        SC_Lower), (<span class="string">'\u{1ff8}'</span>, <span class="string">'\u{1ffc}'</span>, SC_Upper), (<span class="string">'\u{2000}'</span>, <span class="string">'\u{200a}'</span>, SC_Sp), (<span class="string">'\u{200b}'</span>,
        <span class="string">'\u{200b}'</span>, SC_Format), (<span class="string">'\u{200c}'</span>, <span class="string">'\u{200d}'</span>, SC_Extend), (<span class="string">'\u{200e}'</span>, <span class="string">'\u{200f}'</span>,
        SC_Format), (<span class="string">'\u{2013}'</span>, <span class="string">'\u{2014}'</span>, SC_SContinue), (<span class="string">'\u{2018}'</span>, <span class="string">'\u{201f}'</span>, SC_Close),
        (<span class="string">'\u{2024}'</span>, <span class="string">'\u{2024}'</span>, SC_ATerm), (<span class="string">'\u{2028}'</span>, <span class="string">'\u{2029}'</span>, SC_Sep), (<span class="string">'\u{202a}'</span>,
        <span class="string">'\u{202e}'</span>, SC_Format), (<span class="string">'\u{202f}'</span>, <span class="string">'\u{202f}'</span>, SC_Sp), (<span class="string">'\u{2039}'</span>, <span class="string">'\u{203a}'</span>, SC_Close),
        (<span class="string">'\u{203c}'</span>, <span class="string">'\u{203d}'</span>, SC_STerm), (<span class="string">'\u{2045}'</span>, <span class="string">'\u{2046}'</span>, SC_Close), (<span class="string">'\u{2047}'</span>,
        <span class="string">'\u{2049}'</span>, SC_STerm), (<span class="string">'\u{205f}'</span>, <span class="string">'\u{205f}'</span>, SC_Sp), (<span class="string">'\u{2060}'</span>, <span class="string">'\u{2064}'</span>, SC_Format),
        (<span class="string">'\u{2066}'</span>, <span class="string">'\u{206f}'</span>, SC_Format), (<span class="string">'\u{2071}'</span>, <span class="string">'\u{2071}'</span>, SC_Lower), (<span class="string">'\u{207d}'</span>,
        <span class="string">'\u{207e}'</span>, SC_Close), (<span class="string">'\u{207f}'</span>, <span class="string">'\u{207f}'</span>, SC_Lower), (<span class="string">'\u{208d}'</span>, <span class="string">'\u{208e}'</span>,
        SC_Close), (<span class="string">'\u{2090}'</span>, <span class="string">'\u{209c}'</span>, SC_Lower), (<span class="string">'\u{20d0}'</span>, <span class="string">'\u{20f0}'</span>, SC_Extend),
        (<span class="string">'\u{2102}'</span>, <span class="string">'\u{2102}'</span>, SC_Upper), (<span class="string">'\u{2107}'</span>, <span class="string">'\u{2107}'</span>, SC_Upper), (<span class="string">'\u{210a}'</span>,
        <span class="string">'\u{210a}'</span>, SC_Lower), (<span class="string">'\u{210b}'</span>, <span class="string">'\u{210d}'</span>, SC_Upper), (<span class="string">'\u{210e}'</span>, <span class="string">'\u{210f}'</span>,
        SC_Lower), (<span class="string">'\u{2110}'</span>, <span class="string">'\u{2112}'</span>, SC_Upper), (<span class="string">'\u{2113}'</span>, <span class="string">'\u{2113}'</span>, SC_Lower),
        (<span class="string">'\u{2115}'</span>, <span class="string">'\u{2115}'</span>, SC_Upper), (<span class="string">'\u{2119}'</span>, <span class="string">'\u{211d}'</span>, SC_Upper), (<span class="string">'\u{2124}'</span>,
        <span class="string">'\u{2124}'</span>, SC_Upper), (<span class="string">'\u{2126}'</span>, <span class="string">'\u{2126}'</span>, SC_Upper), (<span class="string">'\u{2128}'</span>, <span class="string">'\u{2128}'</span>,
        SC_Upper), (<span class="string">'\u{212a}'</span>, <span class="string">'\u{212d}'</span>, SC_Upper), (<span class="string">'\u{212f}'</span>, <span class="string">'\u{212f}'</span>, SC_Lower),
        (<span class="string">'\u{2130}'</span>, <span class="string">'\u{2133}'</span>, SC_Upper), (<span class="string">'\u{2134}'</span>, <span class="string">'\u{2134}'</span>, SC_Lower), (<span class="string">'\u{2135}'</span>,
        <span class="string">'\u{2138}'</span>, SC_OLetter), (<span class="string">'\u{2139}'</span>, <span class="string">'\u{2139}'</span>, SC_Lower), (<span class="string">'\u{213c}'</span>, <span class="string">'\u{213d}'</span>,
        SC_Lower), (<span class="string">'\u{213e}'</span>, <span class="string">'\u{213f}'</span>, SC_Upper), (<span class="string">'\u{2145}'</span>, <span class="string">'\u{2145}'</span>, SC_Upper),
        (<span class="string">'\u{2146}'</span>, <span class="string">'\u{2149}'</span>, SC_Lower), (<span class="string">'\u{214e}'</span>, <span class="string">'\u{214e}'</span>, SC_Lower), (<span class="string">'\u{2160}'</span>,
        <span class="string">'\u{216f}'</span>, SC_Upper), (<span class="string">'\u{2170}'</span>, <span class="string">'\u{217f}'</span>, SC_Lower), (<span class="string">'\u{2180}'</span>, <span class="string">'\u{2182}'</span>,
        SC_OLetter), (<span class="string">'\u{2183}'</span>, <span class="string">'\u{2183}'</span>, SC_Upper), (<span class="string">'\u{2184}'</span>, <span class="string">'\u{2184}'</span>, SC_Lower),
        (<span class="string">'\u{2185}'</span>, <span class="string">'\u{2188}'</span>, SC_OLetter), (<span class="string">'\u{2308}'</span>, <span class="string">'\u{230b}'</span>, SC_Close), (<span class="string">'\u{2329}'</span>,
        <span class="string">'\u{232a}'</span>, SC_Close), (<span class="string">'\u{24b6}'</span>, <span class="string">'\u{24cf}'</span>, SC_Upper), (<span class="string">'\u{24d0}'</span>, <span class="string">'\u{24e9}'</span>,
        SC_Lower), (<span class="string">'\u{275b}'</span>, <span class="string">'\u{2760}'</span>, SC_Close), (<span class="string">'\u{2768}'</span>, <span class="string">'\u{2775}'</span>, SC_Close),
        (<span class="string">'\u{27c5}'</span>, <span class="string">'\u{27c6}'</span>, SC_Close), (<span class="string">'\u{27e6}'</span>, <span class="string">'\u{27ef}'</span>, SC_Close), (<span class="string">'\u{2983}'</span>,
        <span class="string">'\u{2998}'</span>, SC_Close), (<span class="string">'\u{29d8}'</span>, <span class="string">'\u{29db}'</span>, SC_Close), (<span class="string">'\u{29fc}'</span>, <span class="string">'\u{29fd}'</span>,
        SC_Close), (<span class="string">'\u{2c00}'</span>, <span class="string">'\u{2c2f}'</span>, SC_Upper), (<span class="string">'\u{2c30}'</span>, <span class="string">'\u{2c5f}'</span>, SC_Lower),
        (<span class="string">'\u{2c60}'</span>, <span class="string">'\u{2c60}'</span>, SC_Upper), (<span class="string">'\u{2c61}'</span>, <span class="string">'\u{2c61}'</span>, SC_Lower), (<span class="string">'\u{2c62}'</span>,
        <span class="string">'\u{2c64}'</span>, SC_Upper), (<span class="string">'\u{2c65}'</span>, <span class="string">'\u{2c66}'</span>, SC_Lower), (<span class="string">'\u{2c67}'</span>, <span class="string">'\u{2c67}'</span>,
        SC_Upper), (<span class="string">'\u{2c68}'</span>, <span class="string">'\u{2c68}'</span>, SC_Lower), (<span class="string">'\u{2c69}'</span>, <span class="string">'\u{2c69}'</span>, SC_Upper),
        (<span class="string">'\u{2c6a}'</span>, <span class="string">'\u{2c6a}'</span>, SC_Lower), (<span class="string">'\u{2c6b}'</span>, <span class="string">'\u{2c6b}'</span>, SC_Upper), (<span class="string">'\u{2c6c}'</span>,
        <span class="string">'\u{2c6c}'</span>, SC_Lower), (<span class="string">'\u{2c6d}'</span>, <span class="string">'\u{2c70}'</span>, SC_Upper), (<span class="string">'\u{2c71}'</span>, <span class="string">'\u{2c71}'</span>,
        SC_Lower), (<span class="string">'\u{2c72}'</span>, <span class="string">'\u{2c72}'</span>, SC_Upper), (<span class="string">'\u{2c73}'</span>, <span class="string">'\u{2c74}'</span>, SC_Lower),
        (<span class="string">'\u{2c75}'</span>, <span class="string">'\u{2c75}'</span>, SC_Upper), (<span class="string">'\u{2c76}'</span>, <span class="string">'\u{2c7d}'</span>, SC_Lower), (<span class="string">'\u{2c7e}'</span>,
        <span class="string">'\u{2c80}'</span>, SC_Upper), (<span class="string">'\u{2c81}'</span>, <span class="string">'\u{2c81}'</span>, SC_Lower), (<span class="string">'\u{2c82}'</span>, <span class="string">'\u{2c82}'</span>,
        SC_Upper), (<span class="string">'\u{2c83}'</span>, <span class="string">'\u{2c83}'</span>, SC_Lower), (<span class="string">'\u{2c84}'</span>, <span class="string">'\u{2c84}'</span>, SC_Upper),
        (<span class="string">'\u{2c85}'</span>, <span class="string">'\u{2c85}'</span>, SC_Lower), (<span class="string">'\u{2c86}'</span>, <span class="string">'\u{2c86}'</span>, SC_Upper), (<span class="string">'\u{2c87}'</span>,
        <span class="string">'\u{2c87}'</span>, SC_Lower), (<span class="string">'\u{2c88}'</span>, <span class="string">'\u{2c88}'</span>, SC_Upper), (<span class="string">'\u{2c89}'</span>, <span class="string">'\u{2c89}'</span>,
        SC_Lower), (<span class="string">'\u{2c8a}'</span>, <span class="string">'\u{2c8a}'</span>, SC_Upper), (<span class="string">'\u{2c8b}'</span>, <span class="string">'\u{2c8b}'</span>, SC_Lower),
        (<span class="string">'\u{2c8c}'</span>, <span class="string">'\u{2c8c}'</span>, SC_Upper), (<span class="string">'\u{2c8d}'</span>, <span class="string">'\u{2c8d}'</span>, SC_Lower), (<span class="string">'\u{2c8e}'</span>,
        <span class="string">'\u{2c8e}'</span>, SC_Upper), (<span class="string">'\u{2c8f}'</span>, <span class="string">'\u{2c8f}'</span>, SC_Lower), (<span class="string">'\u{2c90}'</span>, <span class="string">'\u{2c90}'</span>,
        SC_Upper), (<span class="string">'\u{2c91}'</span>, <span class="string">'\u{2c91}'</span>, SC_Lower), (<span class="string">'\u{2c92}'</span>, <span class="string">'\u{2c92}'</span>, SC_Upper),
        (<span class="string">'\u{2c93}'</span>, <span class="string">'\u{2c93}'</span>, SC_Lower), (<span class="string">'\u{2c94}'</span>, <span class="string">'\u{2c94}'</span>, SC_Upper), (<span class="string">'\u{2c95}'</span>,
        <span class="string">'\u{2c95}'</span>, SC_Lower), (<span class="string">'\u{2c96}'</span>, <span class="string">'\u{2c96}'</span>, SC_Upper), (<span class="string">'\u{2c97}'</span>, <span class="string">'\u{2c97}'</span>,
        SC_Lower), (<span class="string">'\u{2c98}'</span>, <span class="string">'\u{2c98}'</span>, SC_Upper), (<span class="string">'\u{2c99}'</span>, <span class="string">'\u{2c99}'</span>, SC_Lower),
        (<span class="string">'\u{2c9a}'</span>, <span class="string">'\u{2c9a}'</span>, SC_Upper), (<span class="string">'\u{2c9b}'</span>, <span class="string">'\u{2c9b}'</span>, SC_Lower), (<span class="string">'\u{2c9c}'</span>,
        <span class="string">'\u{2c9c}'</span>, SC_Upper), (<span class="string">'\u{2c9d}'</span>, <span class="string">'\u{2c9d}'</span>, SC_Lower), (<span class="string">'\u{2c9e}'</span>, <span class="string">'\u{2c9e}'</span>,
        SC_Upper), (<span class="string">'\u{2c9f}'</span>, <span class="string">'\u{2c9f}'</span>, SC_Lower), (<span class="string">'\u{2ca0}'</span>, <span class="string">'\u{2ca0}'</span>, SC_Upper),
        (<span class="string">'\u{2ca1}'</span>, <span class="string">'\u{2ca1}'</span>, SC_Lower), (<span class="string">'\u{2ca2}'</span>, <span class="string">'\u{2ca2}'</span>, SC_Upper), (<span class="string">'\u{2ca3}'</span>,
        <span class="string">'\u{2ca3}'</span>, SC_Lower), (<span class="string">'\u{2ca4}'</span>, <span class="string">'\u{2ca4}'</span>, SC_Upper), (<span class="string">'\u{2ca5}'</span>, <span class="string">'\u{2ca5}'</span>,
        SC_Lower), (<span class="string">'\u{2ca6}'</span>, <span class="string">'\u{2ca6}'</span>, SC_Upper), (<span class="string">'\u{2ca7}'</span>, <span class="string">'\u{2ca7}'</span>, SC_Lower),
        (<span class="string">'\u{2ca8}'</span>, <span class="string">'\u{2ca8}'</span>, SC_Upper), (<span class="string">'\u{2ca9}'</span>, <span class="string">'\u{2ca9}'</span>, SC_Lower), (<span class="string">'\u{2caa}'</span>,
        <span class="string">'\u{2caa}'</span>, SC_Upper), (<span class="string">'\u{2cab}'</span>, <span class="string">'\u{2cab}'</span>, SC_Lower), (<span class="string">'\u{2cac}'</span>, <span class="string">'\u{2cac}'</span>,
        SC_Upper), (<span class="string">'\u{2cad}'</span>, <span class="string">'\u{2cad}'</span>, SC_Lower), (<span class="string">'\u{2cae}'</span>, <span class="string">'\u{2cae}'</span>, SC_Upper),
        (<span class="string">'\u{2caf}'</span>, <span class="string">'\u{2caf}'</span>, SC_Lower), (<span class="string">'\u{2cb0}'</span>, <span class="string">'\u{2cb0}'</span>, SC_Upper), (<span class="string">'\u{2cb1}'</span>,
        <span class="string">'\u{2cb1}'</span>, SC_Lower), (<span class="string">'\u{2cb2}'</span>, <span class="string">'\u{2cb2}'</span>, SC_Upper), (<span class="string">'\u{2cb3}'</span>, <span class="string">'\u{2cb3}'</span>,
        SC_Lower), (<span class="string">'\u{2cb4}'</span>, <span class="string">'\u{2cb4}'</span>, SC_Upper), (<span class="string">'\u{2cb5}'</span>, <span class="string">'\u{2cb5}'</span>, SC_Lower),
        (<span class="string">'\u{2cb6}'</span>, <span class="string">'\u{2cb6}'</span>, SC_Upper), (<span class="string">'\u{2cb7}'</span>, <span class="string">'\u{2cb7}'</span>, SC_Lower), (<span class="string">'\u{2cb8}'</span>,
        <span class="string">'\u{2cb8}'</span>, SC_Upper), (<span class="string">'\u{2cb9}'</span>, <span class="string">'\u{2cb9}'</span>, SC_Lower), (<span class="string">'\u{2cba}'</span>, <span class="string">'\u{2cba}'</span>,
        SC_Upper), (<span class="string">'\u{2cbb}'</span>, <span class="string">'\u{2cbb}'</span>, SC_Lower), (<span class="string">'\u{2cbc}'</span>, <span class="string">'\u{2cbc}'</span>, SC_Upper),
        (<span class="string">'\u{2cbd}'</span>, <span class="string">'\u{2cbd}'</span>, SC_Lower), (<span class="string">'\u{2cbe}'</span>, <span class="string">'\u{2cbe}'</span>, SC_Upper), (<span class="string">'\u{2cbf}'</span>,
        <span class="string">'\u{2cbf}'</span>, SC_Lower), (<span class="string">'\u{2cc0}'</span>, <span class="string">'\u{2cc0}'</span>, SC_Upper), (<span class="string">'\u{2cc1}'</span>, <span class="string">'\u{2cc1}'</span>,
        SC_Lower), (<span class="string">'\u{2cc2}'</span>, <span class="string">'\u{2cc2}'</span>, SC_Upper), (<span class="string">'\u{2cc3}'</span>, <span class="string">'\u{2cc3}'</span>, SC_Lower),
        (<span class="string">'\u{2cc4}'</span>, <span class="string">'\u{2cc4}'</span>, SC_Upper), (<span class="string">'\u{2cc5}'</span>, <span class="string">'\u{2cc5}'</span>, SC_Lower), (<span class="string">'\u{2cc6}'</span>,
        <span class="string">'\u{2cc6}'</span>, SC_Upper), (<span class="string">'\u{2cc7}'</span>, <span class="string">'\u{2cc7}'</span>, SC_Lower), (<span class="string">'\u{2cc8}'</span>, <span class="string">'\u{2cc8}'</span>,
        SC_Upper), (<span class="string">'\u{2cc9}'</span>, <span class="string">'\u{2cc9}'</span>, SC_Lower), (<span class="string">'\u{2cca}'</span>, <span class="string">'\u{2cca}'</span>, SC_Upper),
        (<span class="string">'\u{2ccb}'</span>, <span class="string">'\u{2ccb}'</span>, SC_Lower), (<span class="string">'\u{2ccc}'</span>, <span class="string">'\u{2ccc}'</span>, SC_Upper), (<span class="string">'\u{2ccd}'</span>,
        <span class="string">'\u{2ccd}'</span>, SC_Lower), (<span class="string">'\u{2cce}'</span>, <span class="string">'\u{2cce}'</span>, SC_Upper), (<span class="string">'\u{2ccf}'</span>, <span class="string">'\u{2ccf}'</span>,
        SC_Lower), (<span class="string">'\u{2cd0}'</span>, <span class="string">'\u{2cd0}'</span>, SC_Upper), (<span class="string">'\u{2cd1}'</span>, <span class="string">'\u{2cd1}'</span>, SC_Lower),
        (<span class="string">'\u{2cd2}'</span>, <span class="string">'\u{2cd2}'</span>, SC_Upper), (<span class="string">'\u{2cd3}'</span>, <span class="string">'\u{2cd3}'</span>, SC_Lower), (<span class="string">'\u{2cd4}'</span>,
        <span class="string">'\u{2cd4}'</span>, SC_Upper), (<span class="string">'\u{2cd5}'</span>, <span class="string">'\u{2cd5}'</span>, SC_Lower), (<span class="string">'\u{2cd6}'</span>, <span class="string">'\u{2cd6}'</span>,
        SC_Upper), (<span class="string">'\u{2cd7}'</span>, <span class="string">'\u{2cd7}'</span>, SC_Lower), (<span class="string">'\u{2cd8}'</span>, <span class="string">'\u{2cd8}'</span>, SC_Upper),
        (<span class="string">'\u{2cd9}'</span>, <span class="string">'\u{2cd9}'</span>, SC_Lower), (<span class="string">'\u{2cda}'</span>, <span class="string">'\u{2cda}'</span>, SC_Upper), (<span class="string">'\u{2cdb}'</span>,
        <span class="string">'\u{2cdb}'</span>, SC_Lower), (<span class="string">'\u{2cdc}'</span>, <span class="string">'\u{2cdc}'</span>, SC_Upper), (<span class="string">'\u{2cdd}'</span>, <span class="string">'\u{2cdd}'</span>,
        SC_Lower), (<span class="string">'\u{2cde}'</span>, <span class="string">'\u{2cde}'</span>, SC_Upper), (<span class="string">'\u{2cdf}'</span>, <span class="string">'\u{2cdf}'</span>, SC_Lower),
        (<span class="string">'\u{2ce0}'</span>, <span class="string">'\u{2ce0}'</span>, SC_Upper), (<span class="string">'\u{2ce1}'</span>, <span class="string">'\u{2ce1}'</span>, SC_Lower), (<span class="string">'\u{2ce2}'</span>,
        <span class="string">'\u{2ce2}'</span>, SC_Upper), (<span class="string">'\u{2ce3}'</span>, <span class="string">'\u{2ce4}'</span>, SC_Lower), (<span class="string">'\u{2ceb}'</span>, <span class="string">'\u{2ceb}'</span>,
        SC_Upper), (<span class="string">'\u{2cec}'</span>, <span class="string">'\u{2cec}'</span>, SC_Lower), (<span class="string">'\u{2ced}'</span>, <span class="string">'\u{2ced}'</span>, SC_Upper),
        (<span class="string">'\u{2cee}'</span>, <span class="string">'\u{2cee}'</span>, SC_Lower), (<span class="string">'\u{2cef}'</span>, <span class="string">'\u{2cf1}'</span>, SC_Extend), (<span class="string">'\u{2cf2}'</span>,
        <span class="string">'\u{2cf2}'</span>, SC_Upper), (<span class="string">'\u{2cf3}'</span>, <span class="string">'\u{2cf3}'</span>, SC_Lower), (<span class="string">'\u{2d00}'</span>, <span class="string">'\u{2d25}'</span>,
        SC_Lower), (<span class="string">'\u{2d27}'</span>, <span class="string">'\u{2d27}'</span>, SC_Lower), (<span class="string">'\u{2d2d}'</span>, <span class="string">'\u{2d2d}'</span>, SC_Lower),
        (<span class="string">'\u{2d30}'</span>, <span class="string">'\u{2d67}'</span>, SC_OLetter), (<span class="string">'\u{2d6f}'</span>, <span class="string">'\u{2d6f}'</span>, SC_OLetter), (<span class="string">'\u{2d7f}'</span>,
        <span class="string">'\u{2d7f}'</span>, SC_Extend), (<span class="string">'\u{2d80}'</span>, <span class="string">'\u{2d96}'</span>, SC_OLetter), (<span class="string">'\u{2da0}'</span>, <span class="string">'\u{2da6}'</span>,
        SC_OLetter), (<span class="string">'\u{2da8}'</span>, <span class="string">'\u{2dae}'</span>, SC_OLetter), (<span class="string">'\u{2db0}'</span>, <span class="string">'\u{2db6}'</span>, SC_OLetter),
        (<span class="string">'\u{2db8}'</span>, <span class="string">'\u{2dbe}'</span>, SC_OLetter), (<span class="string">'\u{2dc0}'</span>, <span class="string">'\u{2dc6}'</span>, SC_OLetter), (<span class="string">'\u{2dc8}'</span>,
        <span class="string">'\u{2dce}'</span>, SC_OLetter), (<span class="string">'\u{2dd0}'</span>, <span class="string">'\u{2dd6}'</span>, SC_OLetter), (<span class="string">'\u{2dd8}'</span>, <span class="string">'\u{2dde}'</span>,
        SC_OLetter), (<span class="string">'\u{2de0}'</span>, <span class="string">'\u{2dff}'</span>, SC_Extend), (<span class="string">'\u{2e00}'</span>, <span class="string">'\u{2e0d}'</span>, SC_Close),
        (<span class="string">'\u{2e1c}'</span>, <span class="string">'\u{2e1d}'</span>, SC_Close), (<span class="string">'\u{2e20}'</span>, <span class="string">'\u{2e29}'</span>, SC_Close), (<span class="string">'\u{2e2e}'</span>,
        <span class="string">'\u{2e2e}'</span>, SC_STerm), (<span class="string">'\u{2e2f}'</span>, <span class="string">'\u{2e2f}'</span>, SC_OLetter), (<span class="string">'\u{2e3c}'</span>, <span class="string">'\u{2e3c}'</span>,
        SC_STerm), (<span class="string">'\u{2e42}'</span>, <span class="string">'\u{2e42}'</span>, SC_Close), (<span class="string">'\u{2e53}'</span>, <span class="string">'\u{2e54}'</span>, SC_STerm),
        (<span class="string">'\u{2e55}'</span>, <span class="string">'\u{2e5c}'</span>, SC_Close), (<span class="string">'\u{3000}'</span>, <span class="string">'\u{3000}'</span>, SC_Sp), (<span class="string">'\u{3001}'</span>,
        <span class="string">'\u{3001}'</span>, SC_SContinue), (<span class="string">'\u{3002}'</span>, <span class="string">'\u{3002}'</span>, SC_STerm), (<span class="string">'\u{3005}'</span>, <span class="string">'\u{3007}'</span>,
        SC_OLetter), (<span class="string">'\u{3008}'</span>, <span class="string">'\u{3011}'</span>, SC_Close), (<span class="string">'\u{3014}'</span>, <span class="string">'\u{301b}'</span>, SC_Close),
        (<span class="string">'\u{301d}'</span>, <span class="string">'\u{301f}'</span>, SC_Close), (<span class="string">'\u{3021}'</span>, <span class="string">'\u{3029}'</span>, SC_OLetter), (<span class="string">'\u{302a}'</span>,
        <span class="string">'\u{302f}'</span>, SC_Extend), (<span class="string">'\u{3031}'</span>, <span class="string">'\u{3035}'</span>, SC_OLetter), (<span class="string">'\u{3038}'</span>, <span class="string">'\u{303c}'</span>,
        SC_OLetter), (<span class="string">'\u{3041}'</span>, <span class="string">'\u{3096}'</span>, SC_OLetter), (<span class="string">'\u{3099}'</span>, <span class="string">'\u{309a}'</span>, SC_Extend),
        (<span class="string">'\u{309d}'</span>, <span class="string">'\u{309f}'</span>, SC_OLetter), (<span class="string">'\u{30a1}'</span>, <span class="string">'\u{30fa}'</span>, SC_OLetter), (<span class="string">'\u{30fc}'</span>,
        <span class="string">'\u{30ff}'</span>, SC_OLetter), (<span class="string">'\u{3105}'</span>, <span class="string">'\u{312f}'</span>, SC_OLetter), (<span class="string">'\u{3131}'</span>, <span class="string">'\u{318e}'</span>,
        SC_OLetter), (<span class="string">'\u{31a0}'</span>, <span class="string">'\u{31bf}'</span>, SC_OLetter), (<span class="string">'\u{31f0}'</span>, <span class="string">'\u{31ff}'</span>, SC_OLetter),
        (<span class="string">'\u{3400}'</span>, <span class="string">'\u{4dbf}'</span>, SC_OLetter), (<span class="string">'\u{4e00}'</span>, <span class="string">'\u{a48c}'</span>, SC_OLetter), (<span class="string">'\u{a4d0}'</span>,
        <span class="string">'\u{a4fd}'</span>, SC_OLetter), (<span class="string">'\u{a4ff}'</span>, <span class="string">'\u{a4ff}'</span>, SC_STerm), (<span class="string">'\u{a500}'</span>, <span class="string">'\u{a60c}'</span>,
        SC_OLetter), (<span class="string">'\u{a60e}'</span>, <span class="string">'\u{a60f}'</span>, SC_STerm), (<span class="string">'\u{a610}'</span>, <span class="string">'\u{a61f}'</span>, SC_OLetter),
        (<span class="string">'\u{a620}'</span>, <span class="string">'\u{a629}'</span>, SC_Numeric), (<span class="string">'\u{a62a}'</span>, <span class="string">'\u{a62b}'</span>, SC_OLetter), (<span class="string">'\u{a640}'</span>,
        <span class="string">'\u{a640}'</span>, SC_Upper), (<span class="string">'\u{a641}'</span>, <span class="string">'\u{a641}'</span>, SC_Lower), (<span class="string">'\u{a642}'</span>, <span class="string">'\u{a642}'</span>,
        SC_Upper), (<span class="string">'\u{a643}'</span>, <span class="string">'\u{a643}'</span>, SC_Lower), (<span class="string">'\u{a644}'</span>, <span class="string">'\u{a644}'</span>, SC_Upper),
        (<span class="string">'\u{a645}'</span>, <span class="string">'\u{a645}'</span>, SC_Lower), (<span class="string">'\u{a646}'</span>, <span class="string">'\u{a646}'</span>, SC_Upper), (<span class="string">'\u{a647}'</span>,
        <span class="string">'\u{a647}'</span>, SC_Lower), (<span class="string">'\u{a648}'</span>, <span class="string">'\u{a648}'</span>, SC_Upper), (<span class="string">'\u{a649}'</span>, <span class="string">'\u{a649}'</span>,
        SC_Lower), (<span class="string">'\u{a64a}'</span>, <span class="string">'\u{a64a}'</span>, SC_Upper), (<span class="string">'\u{a64b}'</span>, <span class="string">'\u{a64b}'</span>, SC_Lower),
        (<span class="string">'\u{a64c}'</span>, <span class="string">'\u{a64c}'</span>, SC_Upper), (<span class="string">'\u{a64d}'</span>, <span class="string">'\u{a64d}'</span>, SC_Lower), (<span class="string">'\u{a64e}'</span>,
        <span class="string">'\u{a64e}'</span>, SC_Upper), (<span class="string">'\u{a64f}'</span>, <span class="string">'\u{a64f}'</span>, SC_Lower), (<span class="string">'\u{a650}'</span>, <span class="string">'\u{a650}'</span>,
        SC_Upper), (<span class="string">'\u{a651}'</span>, <span class="string">'\u{a651}'</span>, SC_Lower), (<span class="string">'\u{a652}'</span>, <span class="string">'\u{a652}'</span>, SC_Upper),
        (<span class="string">'\u{a653}'</span>, <span class="string">'\u{a653}'</span>, SC_Lower), (<span class="string">'\u{a654}'</span>, <span class="string">'\u{a654}'</span>, SC_Upper), (<span class="string">'\u{a655}'</span>,
        <span class="string">'\u{a655}'</span>, SC_Lower), (<span class="string">'\u{a656}'</span>, <span class="string">'\u{a656}'</span>, SC_Upper), (<span class="string">'\u{a657}'</span>, <span class="string">'\u{a657}'</span>,
        SC_Lower), (<span class="string">'\u{a658}'</span>, <span class="string">'\u{a658}'</span>, SC_Upper), (<span class="string">'\u{a659}'</span>, <span class="string">'\u{a659}'</span>, SC_Lower),
        (<span class="string">'\u{a65a}'</span>, <span class="string">'\u{a65a}'</span>, SC_Upper), (<span class="string">'\u{a65b}'</span>, <span class="string">'\u{a65b}'</span>, SC_Lower), (<span class="string">'\u{a65c}'</span>,
        <span class="string">'\u{a65c}'</span>, SC_Upper), (<span class="string">'\u{a65d}'</span>, <span class="string">'\u{a65d}'</span>, SC_Lower), (<span class="string">'\u{a65e}'</span>, <span class="string">'\u{a65e}'</span>,
        SC_Upper), (<span class="string">'\u{a65f}'</span>, <span class="string">'\u{a65f}'</span>, SC_Lower), (<span class="string">'\u{a660}'</span>, <span class="string">'\u{a660}'</span>, SC_Upper),
        (<span class="string">'\u{a661}'</span>, <span class="string">'\u{a661}'</span>, SC_Lower), (<span class="string">'\u{a662}'</span>, <span class="string">'\u{a662}'</span>, SC_Upper), (<span class="string">'\u{a663}'</span>,
        <span class="string">'\u{a663}'</span>, SC_Lower), (<span class="string">'\u{a664}'</span>, <span class="string">'\u{a664}'</span>, SC_Upper), (<span class="string">'\u{a665}'</span>, <span class="string">'\u{a665}'</span>,
        SC_Lower), (<span class="string">'\u{a666}'</span>, <span class="string">'\u{a666}'</span>, SC_Upper), (<span class="string">'\u{a667}'</span>, <span class="string">'\u{a667}'</span>, SC_Lower),
        (<span class="string">'\u{a668}'</span>, <span class="string">'\u{a668}'</span>, SC_Upper), (<span class="string">'\u{a669}'</span>, <span class="string">'\u{a669}'</span>, SC_Lower), (<span class="string">'\u{a66a}'</span>,
        <span class="string">'\u{a66a}'</span>, SC_Upper), (<span class="string">'\u{a66b}'</span>, <span class="string">'\u{a66b}'</span>, SC_Lower), (<span class="string">'\u{a66c}'</span>, <span class="string">'\u{a66c}'</span>,
        SC_Upper), (<span class="string">'\u{a66d}'</span>, <span class="string">'\u{a66d}'</span>, SC_Lower), (<span class="string">'\u{a66e}'</span>, <span class="string">'\u{a66e}'</span>, SC_OLetter),
        (<span class="string">'\u{a66f}'</span>, <span class="string">'\u{a672}'</span>, SC_Extend), (<span class="string">'\u{a674}'</span>, <span class="string">'\u{a67d}'</span>, SC_Extend), (<span class="string">'\u{a67f}'</span>,
        <span class="string">'\u{a67f}'</span>, SC_OLetter), (<span class="string">'\u{a680}'</span>, <span class="string">'\u{a680}'</span>, SC_Upper), (<span class="string">'\u{a681}'</span>, <span class="string">'\u{a681}'</span>,
        SC_Lower), (<span class="string">'\u{a682}'</span>, <span class="string">'\u{a682}'</span>, SC_Upper), (<span class="string">'\u{a683}'</span>, <span class="string">'\u{a683}'</span>, SC_Lower),
        (<span class="string">'\u{a684}'</span>, <span class="string">'\u{a684}'</span>, SC_Upper), (<span class="string">'\u{a685}'</span>, <span class="string">'\u{a685}'</span>, SC_Lower), (<span class="string">'\u{a686}'</span>,
        <span class="string">'\u{a686}'</span>, SC_Upper), (<span class="string">'\u{a687}'</span>, <span class="string">'\u{a687}'</span>, SC_Lower), (<span class="string">'\u{a688}'</span>, <span class="string">'\u{a688}'</span>,
        SC_Upper), (<span class="string">'\u{a689}'</span>, <span class="string">'\u{a689}'</span>, SC_Lower), (<span class="string">'\u{a68a}'</span>, <span class="string">'\u{a68a}'</span>, SC_Upper),
        (<span class="string">'\u{a68b}'</span>, <span class="string">'\u{a68b}'</span>, SC_Lower), (<span class="string">'\u{a68c}'</span>, <span class="string">'\u{a68c}'</span>, SC_Upper), (<span class="string">'\u{a68d}'</span>,
        <span class="string">'\u{a68d}'</span>, SC_Lower), (<span class="string">'\u{a68e}'</span>, <span class="string">'\u{a68e}'</span>, SC_Upper), (<span class="string">'\u{a68f}'</span>, <span class="string">'\u{a68f}'</span>,
        SC_Lower), (<span class="string">'\u{a690}'</span>, <span class="string">'\u{a690}'</span>, SC_Upper), (<span class="string">'\u{a691}'</span>, <span class="string">'\u{a691}'</span>, SC_Lower),
        (<span class="string">'\u{a692}'</span>, <span class="string">'\u{a692}'</span>, SC_Upper), (<span class="string">'\u{a693}'</span>, <span class="string">'\u{a693}'</span>, SC_Lower), (<span class="string">'\u{a694}'</span>,
        <span class="string">'\u{a694}'</span>, SC_Upper), (<span class="string">'\u{a695}'</span>, <span class="string">'\u{a695}'</span>, SC_Lower), (<span class="string">'\u{a696}'</span>, <span class="string">'\u{a696}'</span>,
        SC_Upper), (<span class="string">'\u{a697}'</span>, <span class="string">'\u{a697}'</span>, SC_Lower), (<span class="string">'\u{a698}'</span>, <span class="string">'\u{a698}'</span>, SC_Upper),
        (<span class="string">'\u{a699}'</span>, <span class="string">'\u{a699}'</span>, SC_Lower), (<span class="string">'\u{a69a}'</span>, <span class="string">'\u{a69a}'</span>, SC_Upper), (<span class="string">'\u{a69b}'</span>,
        <span class="string">'\u{a69d}'</span>, SC_Lower), (<span class="string">'\u{a69e}'</span>, <span class="string">'\u{a69f}'</span>, SC_Extend), (<span class="string">'\u{a6a0}'</span>, <span class="string">'\u{a6ef}'</span>,
        SC_OLetter), (<span class="string">'\u{a6f0}'</span>, <span class="string">'\u{a6f1}'</span>, SC_Extend), (<span class="string">'\u{a6f3}'</span>, <span class="string">'\u{a6f3}'</span>, SC_STerm),
        (<span class="string">'\u{a6f7}'</span>, <span class="string">'\u{a6f7}'</span>, SC_STerm), (<span class="string">'\u{a717}'</span>, <span class="string">'\u{a71f}'</span>, SC_OLetter), (<span class="string">'\u{a722}'</span>,
        <span class="string">'\u{a722}'</span>, SC_Upper), (<span class="string">'\u{a723}'</span>, <span class="string">'\u{a723}'</span>, SC_Lower), (<span class="string">'\u{a724}'</span>, <span class="string">'\u{a724}'</span>,
        SC_Upper), (<span class="string">'\u{a725}'</span>, <span class="string">'\u{a725}'</span>, SC_Lower), (<span class="string">'\u{a726}'</span>, <span class="string">'\u{a726}'</span>, SC_Upper),
        (<span class="string">'\u{a727}'</span>, <span class="string">'\u{a727}'</span>, SC_Lower), (<span class="string">'\u{a728}'</span>, <span class="string">'\u{a728}'</span>, SC_Upper), (<span class="string">'\u{a729}'</span>,
        <span class="string">'\u{a729}'</span>, SC_Lower), (<span class="string">'\u{a72a}'</span>, <span class="string">'\u{a72a}'</span>, SC_Upper), (<span class="string">'\u{a72b}'</span>, <span class="string">'\u{a72b}'</span>,
        SC_Lower), (<span class="string">'\u{a72c}'</span>, <span class="string">'\u{a72c}'</span>, SC_Upper), (<span class="string">'\u{a72d}'</span>, <span class="string">'\u{a72d}'</span>, SC_Lower),
        (<span class="string">'\u{a72e}'</span>, <span class="string">'\u{a72e}'</span>, SC_Upper), (<span class="string">'\u{a72f}'</span>, <span class="string">'\u{a731}'</span>, SC_Lower), (<span class="string">'\u{a732}'</span>,
        <span class="string">'\u{a732}'</span>, SC_Upper), (<span class="string">'\u{a733}'</span>, <span class="string">'\u{a733}'</span>, SC_Lower), (<span class="string">'\u{a734}'</span>, <span class="string">'\u{a734}'</span>,
        SC_Upper), (<span class="string">'\u{a735}'</span>, <span class="string">'\u{a735}'</span>, SC_Lower), (<span class="string">'\u{a736}'</span>, <span class="string">'\u{a736}'</span>, SC_Upper),
        (<span class="string">'\u{a737}'</span>, <span class="string">'\u{a737}'</span>, SC_Lower), (<span class="string">'\u{a738}'</span>, <span class="string">'\u{a738}'</span>, SC_Upper), (<span class="string">'\u{a739}'</span>,
        <span class="string">'\u{a739}'</span>, SC_Lower), (<span class="string">'\u{a73a}'</span>, <span class="string">'\u{a73a}'</span>, SC_Upper), (<span class="string">'\u{a73b}'</span>, <span class="string">'\u{a73b}'</span>,
        SC_Lower), (<span class="string">'\u{a73c}'</span>, <span class="string">'\u{a73c}'</span>, SC_Upper), (<span class="string">'\u{a73d}'</span>, <span class="string">'\u{a73d}'</span>, SC_Lower),
        (<span class="string">'\u{a73e}'</span>, <span class="string">'\u{a73e}'</span>, SC_Upper), (<span class="string">'\u{a73f}'</span>, <span class="string">'\u{a73f}'</span>, SC_Lower), (<span class="string">'\u{a740}'</span>,
        <span class="string">'\u{a740}'</span>, SC_Upper), (<span class="string">'\u{a741}'</span>, <span class="string">'\u{a741}'</span>, SC_Lower), (<span class="string">'\u{a742}'</span>, <span class="string">'\u{a742}'</span>,
        SC_Upper), (<span class="string">'\u{a743}'</span>, <span class="string">'\u{a743}'</span>, SC_Lower), (<span class="string">'\u{a744}'</span>, <span class="string">'\u{a744}'</span>, SC_Upper),
        (<span class="string">'\u{a745}'</span>, <span class="string">'\u{a745}'</span>, SC_Lower), (<span class="string">'\u{a746}'</span>, <span class="string">'\u{a746}'</span>, SC_Upper), (<span class="string">'\u{a747}'</span>,
        <span class="string">'\u{a747}'</span>, SC_Lower), (<span class="string">'\u{a748}'</span>, <span class="string">'\u{a748}'</span>, SC_Upper), (<span class="string">'\u{a749}'</span>, <span class="string">'\u{a749}'</span>,
        SC_Lower), (<span class="string">'\u{a74a}'</span>, <span class="string">'\u{a74a}'</span>, SC_Upper), (<span class="string">'\u{a74b}'</span>, <span class="string">'\u{a74b}'</span>, SC_Lower),
        (<span class="string">'\u{a74c}'</span>, <span class="string">'\u{a74c}'</span>, SC_Upper), (<span class="string">'\u{a74d}'</span>, <span class="string">'\u{a74d}'</span>, SC_Lower), (<span class="string">'\u{a74e}'</span>,
        <span class="string">'\u{a74e}'</span>, SC_Upper), (<span class="string">'\u{a74f}'</span>, <span class="string">'\u{a74f}'</span>, SC_Lower), (<span class="string">'\u{a750}'</span>, <span class="string">'\u{a750}'</span>,
        SC_Upper), (<span class="string">'\u{a751}'</span>, <span class="string">'\u{a751}'</span>, SC_Lower), (<span class="string">'\u{a752}'</span>, <span class="string">'\u{a752}'</span>, SC_Upper),
        (<span class="string">'\u{a753}'</span>, <span class="string">'\u{a753}'</span>, SC_Lower), (<span class="string">'\u{a754}'</span>, <span class="string">'\u{a754}'</span>, SC_Upper), (<span class="string">'\u{a755}'</span>,
        <span class="string">'\u{a755}'</span>, SC_Lower), (<span class="string">'\u{a756}'</span>, <span class="string">'\u{a756}'</span>, SC_Upper), (<span class="string">'\u{a757}'</span>, <span class="string">'\u{a757}'</span>,
        SC_Lower), (<span class="string">'\u{a758}'</span>, <span class="string">'\u{a758}'</span>, SC_Upper), (<span class="string">'\u{a759}'</span>, <span class="string">'\u{a759}'</span>, SC_Lower),
        (<span class="string">'\u{a75a}'</span>, <span class="string">'\u{a75a}'</span>, SC_Upper), (<span class="string">'\u{a75b}'</span>, <span class="string">'\u{a75b}'</span>, SC_Lower), (<span class="string">'\u{a75c}'</span>,
        <span class="string">'\u{a75c}'</span>, SC_Upper), (<span class="string">'\u{a75d}'</span>, <span class="string">'\u{a75d}'</span>, SC_Lower), (<span class="string">'\u{a75e}'</span>, <span class="string">'\u{a75e}'</span>,
        SC_Upper), (<span class="string">'\u{a75f}'</span>, <span class="string">'\u{a75f}'</span>, SC_Lower), (<span class="string">'\u{a760}'</span>, <span class="string">'\u{a760}'</span>, SC_Upper),
        (<span class="string">'\u{a761}'</span>, <span class="string">'\u{a761}'</span>, SC_Lower), (<span class="string">'\u{a762}'</span>, <span class="string">'\u{a762}'</span>, SC_Upper), (<span class="string">'\u{a763}'</span>,
        <span class="string">'\u{a763}'</span>, SC_Lower), (<span class="string">'\u{a764}'</span>, <span class="string">'\u{a764}'</span>, SC_Upper), (<span class="string">'\u{a765}'</span>, <span class="string">'\u{a765}'</span>,
        SC_Lower), (<span class="string">'\u{a766}'</span>, <span class="string">'\u{a766}'</span>, SC_Upper), (<span class="string">'\u{a767}'</span>, <span class="string">'\u{a767}'</span>, SC_Lower),
        (<span class="string">'\u{a768}'</span>, <span class="string">'\u{a768}'</span>, SC_Upper), (<span class="string">'\u{a769}'</span>, <span class="string">'\u{a769}'</span>, SC_Lower), (<span class="string">'\u{a76a}'</span>,
        <span class="string">'\u{a76a}'</span>, SC_Upper), (<span class="string">'\u{a76b}'</span>, <span class="string">'\u{a76b}'</span>, SC_Lower), (<span class="string">'\u{a76c}'</span>, <span class="string">'\u{a76c}'</span>,
        SC_Upper), (<span class="string">'\u{a76d}'</span>, <span class="string">'\u{a76d}'</span>, SC_Lower), (<span class="string">'\u{a76e}'</span>, <span class="string">'\u{a76e}'</span>, SC_Upper),
        (<span class="string">'\u{a76f}'</span>, <span class="string">'\u{a778}'</span>, SC_Lower), (<span class="string">'\u{a779}'</span>, <span class="string">'\u{a779}'</span>, SC_Upper), (<span class="string">'\u{a77a}'</span>,
        <span class="string">'\u{a77a}'</span>, SC_Lower), (<span class="string">'\u{a77b}'</span>, <span class="string">'\u{a77b}'</span>, SC_Upper), (<span class="string">'\u{a77c}'</span>, <span class="string">'\u{a77c}'</span>,
        SC_Lower), (<span class="string">'\u{a77d}'</span>, <span class="string">'\u{a77e}'</span>, SC_Upper), (<span class="string">'\u{a77f}'</span>, <span class="string">'\u{a77f}'</span>, SC_Lower),
        (<span class="string">'\u{a780}'</span>, <span class="string">'\u{a780}'</span>, SC_Upper), (<span class="string">'\u{a781}'</span>, <span class="string">'\u{a781}'</span>, SC_Lower), (<span class="string">'\u{a782}'</span>,
        <span class="string">'\u{a782}'</span>, SC_Upper), (<span class="string">'\u{a783}'</span>, <span class="string">'\u{a783}'</span>, SC_Lower), (<span class="string">'\u{a784}'</span>, <span class="string">'\u{a784}'</span>,
        SC_Upper), (<span class="string">'\u{a785}'</span>, <span class="string">'\u{a785}'</span>, SC_Lower), (<span class="string">'\u{a786}'</span>, <span class="string">'\u{a786}'</span>, SC_Upper),
        (<span class="string">'\u{a787}'</span>, <span class="string">'\u{a787}'</span>, SC_Lower), (<span class="string">'\u{a788}'</span>, <span class="string">'\u{a788}'</span>, SC_OLetter), (<span class="string">'\u{a78b}'</span>,
        <span class="string">'\u{a78b}'</span>, SC_Upper), (<span class="string">'\u{a78c}'</span>, <span class="string">'\u{a78c}'</span>, SC_Lower), (<span class="string">'\u{a78d}'</span>, <span class="string">'\u{a78d}'</span>,
        SC_Upper), (<span class="string">'\u{a78e}'</span>, <span class="string">'\u{a78e}'</span>, SC_Lower), (<span class="string">'\u{a78f}'</span>, <span class="string">'\u{a78f}'</span>, SC_OLetter),
        (<span class="string">'\u{a790}'</span>, <span class="string">'\u{a790}'</span>, SC_Upper), (<span class="string">'\u{a791}'</span>, <span class="string">'\u{a791}'</span>, SC_Lower), (<span class="string">'\u{a792}'</span>,
        <span class="string">'\u{a792}'</span>, SC_Upper), (<span class="string">'\u{a793}'</span>, <span class="string">'\u{a795}'</span>, SC_Lower), (<span class="string">'\u{a796}'</span>, <span class="string">'\u{a796}'</span>,
        SC_Upper), (<span class="string">'\u{a797}'</span>, <span class="string">'\u{a797}'</span>, SC_Lower), (<span class="string">'\u{a798}'</span>, <span class="string">'\u{a798}'</span>, SC_Upper),
        (<span class="string">'\u{a799}'</span>, <span class="string">'\u{a799}'</span>, SC_Lower), (<span class="string">'\u{a79a}'</span>, <span class="string">'\u{a79a}'</span>, SC_Upper), (<span class="string">'\u{a79b}'</span>,
        <span class="string">'\u{a79b}'</span>, SC_Lower), (<span class="string">'\u{a79c}'</span>, <span class="string">'\u{a79c}'</span>, SC_Upper), (<span class="string">'\u{a79d}'</span>, <span class="string">'\u{a79d}'</span>,
        SC_Lower), (<span class="string">'\u{a79e}'</span>, <span class="string">'\u{a79e}'</span>, SC_Upper), (<span class="string">'\u{a79f}'</span>, <span class="string">'\u{a79f}'</span>, SC_Lower),
        (<span class="string">'\u{a7a0}'</span>, <span class="string">'\u{a7a0}'</span>, SC_Upper), (<span class="string">'\u{a7a1}'</span>, <span class="string">'\u{a7a1}'</span>, SC_Lower), (<span class="string">'\u{a7a2}'</span>,
        <span class="string">'\u{a7a2}'</span>, SC_Upper), (<span class="string">'\u{a7a3}'</span>, <span class="string">'\u{a7a3}'</span>, SC_Lower), (<span class="string">'\u{a7a4}'</span>, <span class="string">'\u{a7a4}'</span>,
        SC_Upper), (<span class="string">'\u{a7a5}'</span>, <span class="string">'\u{a7a5}'</span>, SC_Lower), (<span class="string">'\u{a7a6}'</span>, <span class="string">'\u{a7a6}'</span>, SC_Upper),
        (<span class="string">'\u{a7a7}'</span>, <span class="string">'\u{a7a7}'</span>, SC_Lower), (<span class="string">'\u{a7a8}'</span>, <span class="string">'\u{a7a8}'</span>, SC_Upper), (<span class="string">'\u{a7a9}'</span>,
        <span class="string">'\u{a7a9}'</span>, SC_Lower), (<span class="string">'\u{a7aa}'</span>, <span class="string">'\u{a7ae}'</span>, SC_Upper), (<span class="string">'\u{a7af}'</span>, <span class="string">'\u{a7af}'</span>,
        SC_Lower), (<span class="string">'\u{a7b0}'</span>, <span class="string">'\u{a7b4}'</span>, SC_Upper), (<span class="string">'\u{a7b5}'</span>, <span class="string">'\u{a7b5}'</span>, SC_Lower),
        (<span class="string">'\u{a7b6}'</span>, <span class="string">'\u{a7b6}'</span>, SC_Upper), (<span class="string">'\u{a7b7}'</span>, <span class="string">'\u{a7b7}'</span>, SC_Lower), (<span class="string">'\u{a7b8}'</span>,
        <span class="string">'\u{a7b8}'</span>, SC_Upper), (<span class="string">'\u{a7b9}'</span>, <span class="string">'\u{a7b9}'</span>, SC_Lower), (<span class="string">'\u{a7ba}'</span>, <span class="string">'\u{a7ba}'</span>,
        SC_Upper), (<span class="string">'\u{a7bb}'</span>, <span class="string">'\u{a7bb}'</span>, SC_Lower), (<span class="string">'\u{a7bc}'</span>, <span class="string">'\u{a7bc}'</span>, SC_Upper),
        (<span class="string">'\u{a7bd}'</span>, <span class="string">'\u{a7bd}'</span>, SC_Lower), (<span class="string">'\u{a7be}'</span>, <span class="string">'\u{a7be}'</span>, SC_Upper), (<span class="string">'\u{a7bf}'</span>,
        <span class="string">'\u{a7bf}'</span>, SC_Lower), (<span class="string">'\u{a7c0}'</span>, <span class="string">'\u{a7c0}'</span>, SC_Upper), (<span class="string">'\u{a7c1}'</span>, <span class="string">'\u{a7c1}'</span>,
        SC_Lower), (<span class="string">'\u{a7c2}'</span>, <span class="string">'\u{a7c2}'</span>, SC_Upper), (<span class="string">'\u{a7c3}'</span>, <span class="string">'\u{a7c3}'</span>, SC_Lower),
        (<span class="string">'\u{a7c4}'</span>, <span class="string">'\u{a7c7}'</span>, SC_Upper), (<span class="string">'\u{a7c8}'</span>, <span class="string">'\u{a7c8}'</span>, SC_Lower), (<span class="string">'\u{a7c9}'</span>,
        <span class="string">'\u{a7c9}'</span>, SC_Upper), (<span class="string">'\u{a7ca}'</span>, <span class="string">'\u{a7ca}'</span>, SC_Lower), (<span class="string">'\u{a7d0}'</span>, <span class="string">'\u{a7d0}'</span>,
        SC_Upper), (<span class="string">'\u{a7d1}'</span>, <span class="string">'\u{a7d1}'</span>, SC_Lower), (<span class="string">'\u{a7d3}'</span>, <span class="string">'\u{a7d3}'</span>, SC_Lower),
        (<span class="string">'\u{a7d5}'</span>, <span class="string">'\u{a7d5}'</span>, SC_Lower), (<span class="string">'\u{a7d6}'</span>, <span class="string">'\u{a7d6}'</span>, SC_Upper), (<span class="string">'\u{a7d7}'</span>,
        <span class="string">'\u{a7d7}'</span>, SC_Lower), (<span class="string">'\u{a7d8}'</span>, <span class="string">'\u{a7d8}'</span>, SC_Upper), (<span class="string">'\u{a7d9}'</span>, <span class="string">'\u{a7d9}'</span>,
        SC_Lower), (<span class="string">'\u{a7f2}'</span>, <span class="string">'\u{a7f4}'</span>, SC_Lower), (<span class="string">'\u{a7f5}'</span>, <span class="string">'\u{a7f5}'</span>, SC_Upper),
        (<span class="string">'\u{a7f6}'</span>, <span class="string">'\u{a7f6}'</span>, SC_Lower), (<span class="string">'\u{a7f7}'</span>, <span class="string">'\u{a7f7}'</span>, SC_OLetter), (<span class="string">'\u{a7f8}'</span>,
        <span class="string">'\u{a7fa}'</span>, SC_Lower), (<span class="string">'\u{a7fb}'</span>, <span class="string">'\u{a801}'</span>, SC_OLetter), (<span class="string">'\u{a802}'</span>, <span class="string">'\u{a802}'</span>,
        SC_Extend), (<span class="string">'\u{a803}'</span>, <span class="string">'\u{a805}'</span>, SC_OLetter), (<span class="string">'\u{a806}'</span>, <span class="string">'\u{a806}'</span>, SC_Extend),
        (<span class="string">'\u{a807}'</span>, <span class="string">'\u{a80a}'</span>, SC_OLetter), (<span class="string">'\u{a80b}'</span>, <span class="string">'\u{a80b}'</span>, SC_Extend), (<span class="string">'\u{a80c}'</span>,
        <span class="string">'\u{a822}'</span>, SC_OLetter), (<span class="string">'\u{a823}'</span>, <span class="string">'\u{a827}'</span>, SC_Extend), (<span class="string">'\u{a82c}'</span>, <span class="string">'\u{a82c}'</span>,
        SC_Extend), (<span class="string">'\u{a840}'</span>, <span class="string">'\u{a873}'</span>, SC_OLetter), (<span class="string">'\u{a876}'</span>, <span class="string">'\u{a877}'</span>, SC_STerm),
        (<span class="string">'\u{a880}'</span>, <span class="string">'\u{a881}'</span>, SC_Extend), (<span class="string">'\u{a882}'</span>, <span class="string">'\u{a8b3}'</span>, SC_OLetter), (<span class="string">'\u{a8b4}'</span>,
        <span class="string">'\u{a8c5}'</span>, SC_Extend), (<span class="string">'\u{a8ce}'</span>, <span class="string">'\u{a8cf}'</span>, SC_STerm), (<span class="string">'\u{a8d0}'</span>, <span class="string">'\u{a8d9}'</span>,
        SC_Numeric), (<span class="string">'\u{a8e0}'</span>, <span class="string">'\u{a8f1}'</span>, SC_Extend), (<span class="string">'\u{a8f2}'</span>, <span class="string">'\u{a8f7}'</span>, SC_OLetter),
        (<span class="string">'\u{a8fb}'</span>, <span class="string">'\u{a8fb}'</span>, SC_OLetter), (<span class="string">'\u{a8fd}'</span>, <span class="string">'\u{a8fe}'</span>, SC_OLetter), (<span class="string">'\u{a8ff}'</span>,
        <span class="string">'\u{a8ff}'</span>, SC_Extend), (<span class="string">'\u{a900}'</span>, <span class="string">'\u{a909}'</span>, SC_Numeric), (<span class="string">'\u{a90a}'</span>, <span class="string">'\u{a925}'</span>,
        SC_OLetter), (<span class="string">'\u{a926}'</span>, <span class="string">'\u{a92d}'</span>, SC_Extend), (<span class="string">'\u{a92f}'</span>, <span class="string">'\u{a92f}'</span>, SC_STerm),
        (<span class="string">'\u{a930}'</span>, <span class="string">'\u{a946}'</span>, SC_OLetter), (<span class="string">'\u{a947}'</span>, <span class="string">'\u{a953}'</span>, SC_Extend), (<span class="string">'\u{a960}'</span>,
        <span class="string">'\u{a97c}'</span>, SC_OLetter), (<span class="string">'\u{a980}'</span>, <span class="string">'\u{a983}'</span>, SC_Extend), (<span class="string">'\u{a984}'</span>, <span class="string">'\u{a9b2}'</span>,
        SC_OLetter), (<span class="string">'\u{a9b3}'</span>, <span class="string">'\u{a9c0}'</span>, SC_Extend), (<span class="string">'\u{a9c8}'</span>, <span class="string">'\u{a9c9}'</span>, SC_STerm),
        (<span class="string">'\u{a9cf}'</span>, <span class="string">'\u{a9cf}'</span>, SC_OLetter), (<span class="string">'\u{a9d0}'</span>, <span class="string">'\u{a9d9}'</span>, SC_Numeric), (<span class="string">'\u{a9e0}'</span>,
        <span class="string">'\u{a9e4}'</span>, SC_OLetter), (<span class="string">'\u{a9e5}'</span>, <span class="string">'\u{a9e5}'</span>, SC_Extend), (<span class="string">'\u{a9e6}'</span>, <span class="string">'\u{a9ef}'</span>,
        SC_OLetter), (<span class="string">'\u{a9f0}'</span>, <span class="string">'\u{a9f9}'</span>, SC_Numeric), (<span class="string">'\u{a9fa}'</span>, <span class="string">'\u{a9fe}'</span>, SC_OLetter),
        (<span class="string">'\u{aa00}'</span>, <span class="string">'\u{aa28}'</span>, SC_OLetter), (<span class="string">'\u{aa29}'</span>, <span class="string">'\u{aa36}'</span>, SC_Extend), (<span class="string">'\u{aa40}'</span>,
        <span class="string">'\u{aa42}'</span>, SC_OLetter), (<span class="string">'\u{aa43}'</span>, <span class="string">'\u{aa43}'</span>, SC_Extend), (<span class="string">'\u{aa44}'</span>, <span class="string">'\u{aa4b}'</span>,
        SC_OLetter), (<span class="string">'\u{aa4c}'</span>, <span class="string">'\u{aa4d}'</span>, SC_Extend), (<span class="string">'\u{aa50}'</span>, <span class="string">'\u{aa59}'</span>, SC_Numeric),
        (<span class="string">'\u{aa5d}'</span>, <span class="string">'\u{aa5f}'</span>, SC_STerm), (<span class="string">'\u{aa60}'</span>, <span class="string">'\u{aa76}'</span>, SC_OLetter), (<span class="string">'\u{aa7a}'</span>,
        <span class="string">'\u{aa7a}'</span>, SC_OLetter), (<span class="string">'\u{aa7b}'</span>, <span class="string">'\u{aa7d}'</span>, SC_Extend), (<span class="string">'\u{aa7e}'</span>, <span class="string">'\u{aaaf}'</span>,
        SC_OLetter), (<span class="string">'\u{aab0}'</span>, <span class="string">'\u{aab0}'</span>, SC_Extend), (<span class="string">'\u{aab1}'</span>, <span class="string">'\u{aab1}'</span>, SC_OLetter),
        (<span class="string">'\u{aab2}'</span>, <span class="string">'\u{aab4}'</span>, SC_Extend), (<span class="string">'\u{aab5}'</span>, <span class="string">'\u{aab6}'</span>, SC_OLetter), (<span class="string">'\u{aab7}'</span>,
        <span class="string">'\u{aab8}'</span>, SC_Extend), (<span class="string">'\u{aab9}'</span>, <span class="string">'\u{aabd}'</span>, SC_OLetter), (<span class="string">'\u{aabe}'</span>, <span class="string">'\u{aabf}'</span>,
        SC_Extend), (<span class="string">'\u{aac0}'</span>, <span class="string">'\u{aac0}'</span>, SC_OLetter), (<span class="string">'\u{aac1}'</span>, <span class="string">'\u{aac1}'</span>, SC_Extend),
        (<span class="string">'\u{aac2}'</span>, <span class="string">'\u{aac2}'</span>, SC_OLetter), (<span class="string">'\u{aadb}'</span>, <span class="string">'\u{aadd}'</span>, SC_OLetter), (<span class="string">'\u{aae0}'</span>,
        <span class="string">'\u{aaea}'</span>, SC_OLetter), (<span class="string">'\u{aaeb}'</span>, <span class="string">'\u{aaef}'</span>, SC_Extend), (<span class="string">'\u{aaf0}'</span>, <span class="string">'\u{aaf1}'</span>,
        SC_STerm), (<span class="string">'\u{aaf2}'</span>, <span class="string">'\u{aaf4}'</span>, SC_OLetter), (<span class="string">'\u{aaf5}'</span>, <span class="string">'\u{aaf6}'</span>, SC_Extend),
        (<span class="string">'\u{ab01}'</span>, <span class="string">'\u{ab06}'</span>, SC_OLetter), (<span class="string">'\u{ab09}'</span>, <span class="string">'\u{ab0e}'</span>, SC_OLetter), (<span class="string">'\u{ab11}'</span>,
        <span class="string">'\u{ab16}'</span>, SC_OLetter), (<span class="string">'\u{ab20}'</span>, <span class="string">'\u{ab26}'</span>, SC_OLetter), (<span class="string">'\u{ab28}'</span>, <span class="string">'\u{ab2e}'</span>,
        SC_OLetter), (<span class="string">'\u{ab30}'</span>, <span class="string">'\u{ab5a}'</span>, SC_Lower), (<span class="string">'\u{ab5c}'</span>, <span class="string">'\u{ab69}'</span>, SC_Lower),
        (<span class="string">'\u{ab70}'</span>, <span class="string">'\u{abbf}'</span>, SC_Lower), (<span class="string">'\u{abc0}'</span>, <span class="string">'\u{abe2}'</span>, SC_OLetter), (<span class="string">'\u{abe3}'</span>,
        <span class="string">'\u{abea}'</span>, SC_Extend), (<span class="string">'\u{abeb}'</span>, <span class="string">'\u{abeb}'</span>, SC_STerm), (<span class="string">'\u{abec}'</span>, <span class="string">'\u{abed}'</span>,
        SC_Extend), (<span class="string">'\u{abf0}'</span>, <span class="string">'\u{abf9}'</span>, SC_Numeric), (<span class="string">'\u{ac00}'</span>, <span class="string">'\u{d7a3}'</span>, SC_OLetter),
        (<span class="string">'\u{d7b0}'</span>, <span class="string">'\u{d7c6}'</span>, SC_OLetter), (<span class="string">'\u{d7cb}'</span>, <span class="string">'\u{d7fb}'</span>, SC_OLetter), (<span class="string">'\u{f900}'</span>,
        <span class="string">'\u{fa6d}'</span>, SC_OLetter), (<span class="string">'\u{fa70}'</span>, <span class="string">'\u{fad9}'</span>, SC_OLetter), (<span class="string">'\u{fb00}'</span>, <span class="string">'\u{fb06}'</span>,
        SC_Lower), (<span class="string">'\u{fb13}'</span>, <span class="string">'\u{fb17}'</span>, SC_Lower), (<span class="string">'\u{fb1d}'</span>, <span class="string">'\u{fb1d}'</span>, SC_OLetter),
        (<span class="string">'\u{fb1e}'</span>, <span class="string">'\u{fb1e}'</span>, SC_Extend), (<span class="string">'\u{fb1f}'</span>, <span class="string">'\u{fb28}'</span>, SC_OLetter), (<span class="string">'\u{fb2a}'</span>,
        <span class="string">'\u{fb36}'</span>, SC_OLetter), (<span class="string">'\u{fb38}'</span>, <span class="string">'\u{fb3c}'</span>, SC_OLetter), (<span class="string">'\u{fb3e}'</span>, <span class="string">'\u{fb3e}'</span>,
        SC_OLetter), (<span class="string">'\u{fb40}'</span>, <span class="string">'\u{fb41}'</span>, SC_OLetter), (<span class="string">'\u{fb43}'</span>, <span class="string">'\u{fb44}'</span>, SC_OLetter),
        (<span class="string">'\u{fb46}'</span>, <span class="string">'\u{fbb1}'</span>, SC_OLetter), (<span class="string">'\u{fbd3}'</span>, <span class="string">'\u{fd3d}'</span>, SC_OLetter), (<span class="string">'\u{fd3e}'</span>,
        <span class="string">'\u{fd3f}'</span>, SC_Close), (<span class="string">'\u{fd50}'</span>, <span class="string">'\u{fd8f}'</span>, SC_OLetter), (<span class="string">'\u{fd92}'</span>, <span class="string">'\u{fdc7}'</span>,
        SC_OLetter), (<span class="string">'\u{fdf0}'</span>, <span class="string">'\u{fdfb}'</span>, SC_OLetter), (<span class="string">'\u{fe00}'</span>, <span class="string">'\u{fe0f}'</span>, SC_Extend),
        (<span class="string">'\u{fe10}'</span>, <span class="string">'\u{fe11}'</span>, SC_SContinue), (<span class="string">'\u{fe13}'</span>, <span class="string">'\u{fe13}'</span>, SC_SContinue), (<span class="string">'\u{fe17}'</span>,
        <span class="string">'\u{fe18}'</span>, SC_Close), (<span class="string">'\u{fe20}'</span>, <span class="string">'\u{fe2f}'</span>, SC_Extend), (<span class="string">'\u{fe31}'</span>, <span class="string">'\u{fe32}'</span>,
        SC_SContinue), (<span class="string">'\u{fe35}'</span>, <span class="string">'\u{fe44}'</span>, SC_Close), (<span class="string">'\u{fe47}'</span>, <span class="string">'\u{fe48}'</span>, SC_Close),
        (<span class="string">'\u{fe50}'</span>, <span class="string">'\u{fe51}'</span>, SC_SContinue), (<span class="string">'\u{fe52}'</span>, <span class="string">'\u{fe52}'</span>, SC_ATerm), (<span class="string">'\u{fe55}'</span>,
        <span class="string">'\u{fe55}'</span>, SC_SContinue), (<span class="string">'\u{fe56}'</span>, <span class="string">'\u{fe57}'</span>, SC_STerm), (<span class="string">'\u{fe58}'</span>, <span class="string">'\u{fe58}'</span>,
        SC_SContinue), (<span class="string">'\u{fe59}'</span>, <span class="string">'\u{fe5e}'</span>, SC_Close), (<span class="string">'\u{fe63}'</span>, <span class="string">'\u{fe63}'</span>, SC_SContinue),
        (<span class="string">'\u{fe70}'</span>, <span class="string">'\u{fe74}'</span>, SC_OLetter), (<span class="string">'\u{fe76}'</span>, <span class="string">'\u{fefc}'</span>, SC_OLetter), (<span class="string">'\u{feff}'</span>,
        <span class="string">'\u{feff}'</span>, SC_Format), (<span class="string">'\u{ff01}'</span>, <span class="string">'\u{ff01}'</span>, SC_STerm), (<span class="string">'\u{ff08}'</span>, <span class="string">'\u{ff09}'</span>,
        SC_Close), (<span class="string">'\u{ff0c}'</span>, <span class="string">'\u{ff0d}'</span>, SC_SContinue), (<span class="string">'\u{ff0e}'</span>, <span class="string">'\u{ff0e}'</span>, SC_ATerm),
        (<span class="string">'\u{ff10}'</span>, <span class="string">'\u{ff19}'</span>, SC_Numeric), (<span class="string">'\u{ff1a}'</span>, <span class="string">'\u{ff1a}'</span>, SC_SContinue), (<span class="string">'\u{ff1f}'</span>,
        <span class="string">'\u{ff1f}'</span>, SC_STerm), (<span class="string">'\u{ff21}'</span>, <span class="string">'\u{ff3a}'</span>, SC_Upper), (<span class="string">'\u{ff3b}'</span>, <span class="string">'\u{ff3b}'</span>,
        SC_Close), (<span class="string">'\u{ff3d}'</span>, <span class="string">'\u{ff3d}'</span>, SC_Close), (<span class="string">'\u{ff41}'</span>, <span class="string">'\u{ff5a}'</span>, SC_Lower),
        (<span class="string">'\u{ff5b}'</span>, <span class="string">'\u{ff5b}'</span>, SC_Close), (<span class="string">'\u{ff5d}'</span>, <span class="string">'\u{ff5d}'</span>, SC_Close), (<span class="string">'\u{ff5f}'</span>,
        <span class="string">'\u{ff60}'</span>, SC_Close), (<span class="string">'\u{ff61}'</span>, <span class="string">'\u{ff61}'</span>, SC_STerm), (<span class="string">'\u{ff62}'</span>, <span class="string">'\u{ff63}'</span>,
        SC_Close), (<span class="string">'\u{ff64}'</span>, <span class="string">'\u{ff64}'</span>, SC_SContinue), (<span class="string">'\u{ff66}'</span>, <span class="string">'\u{ff9d}'</span>, SC_OLetter),
        (<span class="string">'\u{ff9e}'</span>, <span class="string">'\u{ff9f}'</span>, SC_Extend), (<span class="string">'\u{ffa0}'</span>, <span class="string">'\u{ffbe}'</span>, SC_OLetter), (<span class="string">'\u{ffc2}'</span>,
        <span class="string">'\u{ffc7}'</span>, SC_OLetter), (<span class="string">'\u{ffca}'</span>, <span class="string">'\u{ffcf}'</span>, SC_OLetter), (<span class="string">'\u{ffd2}'</span>, <span class="string">'\u{ffd7}'</span>,
        SC_OLetter), (<span class="string">'\u{ffda}'</span>, <span class="string">'\u{ffdc}'</span>, SC_OLetter), (<span class="string">'\u{fff9}'</span>, <span class="string">'\u{fffb}'</span>, SC_Format),
        (<span class="string">'\u{10000}'</span>, <span class="string">'\u{1000b}'</span>, SC_OLetter), (<span class="string">'\u{1000d}'</span>, <span class="string">'\u{10026}'</span>, SC_OLetter),
        (<span class="string">'\u{10028}'</span>, <span class="string">'\u{1003a}'</span>, SC_OLetter), (<span class="string">'\u{1003c}'</span>, <span class="string">'\u{1003d}'</span>, SC_OLetter),
        (<span class="string">'\u{1003f}'</span>, <span class="string">'\u{1004d}'</span>, SC_OLetter), (<span class="string">'\u{10050}'</span>, <span class="string">'\u{1005d}'</span>, SC_OLetter),
        (<span class="string">'\u{10080}'</span>, <span class="string">'\u{100fa}'</span>, SC_OLetter), (<span class="string">'\u{10140}'</span>, <span class="string">'\u{10174}'</span>, SC_OLetter),
        (<span class="string">'\u{101fd}'</span>, <span class="string">'\u{101fd}'</span>, SC_Extend), (<span class="string">'\u{10280}'</span>, <span class="string">'\u{1029c}'</span>, SC_OLetter), (<span class="string">'\u{102a0}'</span>,
        <span class="string">'\u{102d0}'</span>, SC_OLetter), (<span class="string">'\u{102e0}'</span>, <span class="string">'\u{102e0}'</span>, SC_Extend), (<span class="string">'\u{10300}'</span>, <span class="string">'\u{1031f}'</span>,
        SC_OLetter), (<span class="string">'\u{1032d}'</span>, <span class="string">'\u{1034a}'</span>, SC_OLetter), (<span class="string">'\u{10350}'</span>, <span class="string">'\u{10375}'</span>, SC_OLetter),
        (<span class="string">'\u{10376}'</span>, <span class="string">'\u{1037a}'</span>, SC_Extend), (<span class="string">'\u{10380}'</span>, <span class="string">'\u{1039d}'</span>, SC_OLetter), (<span class="string">'\u{103a0}'</span>,
        <span class="string">'\u{103c3}'</span>, SC_OLetter), (<span class="string">'\u{103c8}'</span>, <span class="string">'\u{103cf}'</span>, SC_OLetter), (<span class="string">'\u{103d1}'</span>, <span class="string">'\u{103d5}'</span>,
        SC_OLetter), (<span class="string">'\u{10400}'</span>, <span class="string">'\u{10427}'</span>, SC_Upper), (<span class="string">'\u{10428}'</span>, <span class="string">'\u{1044f}'</span>, SC_Lower),
        (<span class="string">'\u{10450}'</span>, <span class="string">'\u{1049d}'</span>, SC_OLetter), (<span class="string">'\u{104a0}'</span>, <span class="string">'\u{104a9}'</span>, SC_Numeric),
        (<span class="string">'\u{104b0}'</span>, <span class="string">'\u{104d3}'</span>, SC_Upper), (<span class="string">'\u{104d8}'</span>, <span class="string">'\u{104fb}'</span>, SC_Lower), (<span class="string">'\u{10500}'</span>,
        <span class="string">'\u{10527}'</span>, SC_OLetter), (<span class="string">'\u{10530}'</span>, <span class="string">'\u{10563}'</span>, SC_OLetter), (<span class="string">'\u{10570}'</span>, <span class="string">'\u{1057a}'</span>,
        SC_Upper), (<span class="string">'\u{1057c}'</span>, <span class="string">'\u{1058a}'</span>, SC_Upper), (<span class="string">'\u{1058c}'</span>, <span class="string">'\u{10592}'</span>, SC_Upper),
        (<span class="string">'\u{10594}'</span>, <span class="string">'\u{10595}'</span>, SC_Upper), (<span class="string">'\u{10597}'</span>, <span class="string">'\u{105a1}'</span>, SC_Lower), (<span class="string">'\u{105a3}'</span>,
        <span class="string">'\u{105b1}'</span>, SC_Lower), (<span class="string">'\u{105b3}'</span>, <span class="string">'\u{105b9}'</span>, SC_Lower), (<span class="string">'\u{105bb}'</span>, <span class="string">'\u{105bc}'</span>,
        SC_Lower), (<span class="string">'\u{10600}'</span>, <span class="string">'\u{10736}'</span>, SC_OLetter), (<span class="string">'\u{10740}'</span>, <span class="string">'\u{10755}'</span>, SC_OLetter),
        (<span class="string">'\u{10760}'</span>, <span class="string">'\u{10767}'</span>, SC_OLetter), (<span class="string">'\u{10780}'</span>, <span class="string">'\u{10780}'</span>, SC_Lower), (<span class="string">'\u{10781}'</span>,
        <span class="string">'\u{10782}'</span>, SC_OLetter), (<span class="string">'\u{10783}'</span>, <span class="string">'\u{10785}'</span>, SC_Lower), (<span class="string">'\u{10787}'</span>, <span class="string">'\u{107b0}'</span>,
        SC_Lower), (<span class="string">'\u{107b2}'</span>, <span class="string">'\u{107ba}'</span>, SC_Lower), (<span class="string">'\u{10800}'</span>, <span class="string">'\u{10805}'</span>, SC_OLetter),
        (<span class="string">'\u{10808}'</span>, <span class="string">'\u{10808}'</span>, SC_OLetter), (<span class="string">'\u{1080a}'</span>, <span class="string">'\u{10835}'</span>, SC_OLetter),
        (<span class="string">'\u{10837}'</span>, <span class="string">'\u{10838}'</span>, SC_OLetter), (<span class="string">'\u{1083c}'</span>, <span class="string">'\u{1083c}'</span>, SC_OLetter),
        (<span class="string">'\u{1083f}'</span>, <span class="string">'\u{10855}'</span>, SC_OLetter), (<span class="string">'\u{10860}'</span>, <span class="string">'\u{10876}'</span>, SC_OLetter),
        (<span class="string">'\u{10880}'</span>, <span class="string">'\u{1089e}'</span>, SC_OLetter), (<span class="string">'\u{108e0}'</span>, <span class="string">'\u{108f2}'</span>, SC_OLetter),
        (<span class="string">'\u{108f4}'</span>, <span class="string">'\u{108f5}'</span>, SC_OLetter), (<span class="string">'\u{10900}'</span>, <span class="string">'\u{10915}'</span>, SC_OLetter),
        (<span class="string">'\u{10920}'</span>, <span class="string">'\u{10939}'</span>, SC_OLetter), (<span class="string">'\u{10980}'</span>, <span class="string">'\u{109b7}'</span>, SC_OLetter),
        (<span class="string">'\u{109be}'</span>, <span class="string">'\u{109bf}'</span>, SC_OLetter), (<span class="string">'\u{10a00}'</span>, <span class="string">'\u{10a00}'</span>, SC_OLetter),
        (<span class="string">'\u{10a01}'</span>, <span class="string">'\u{10a03}'</span>, SC_Extend), (<span class="string">'\u{10a05}'</span>, <span class="string">'\u{10a06}'</span>, SC_Extend), (<span class="string">'\u{10a0c}'</span>,
        <span class="string">'\u{10a0f}'</span>, SC_Extend), (<span class="string">'\u{10a10}'</span>, <span class="string">'\u{10a13}'</span>, SC_OLetter), (<span class="string">'\u{10a15}'</span>, <span class="string">'\u{10a17}'</span>,
        SC_OLetter), (<span class="string">'\u{10a19}'</span>, <span class="string">'\u{10a35}'</span>, SC_OLetter), (<span class="string">'\u{10a38}'</span>, <span class="string">'\u{10a3a}'</span>, SC_Extend),
        (<span class="string">'\u{10a3f}'</span>, <span class="string">'\u{10a3f}'</span>, SC_Extend), (<span class="string">'\u{10a56}'</span>, <span class="string">'\u{10a57}'</span>, SC_STerm), (<span class="string">'\u{10a60}'</span>,
        <span class="string">'\u{10a7c}'</span>, SC_OLetter), (<span class="string">'\u{10a80}'</span>, <span class="string">'\u{10a9c}'</span>, SC_OLetter), (<span class="string">'\u{10ac0}'</span>, <span class="string">'\u{10ac7}'</span>,
        SC_OLetter), (<span class="string">'\u{10ac9}'</span>, <span class="string">'\u{10ae4}'</span>, SC_OLetter), (<span class="string">'\u{10ae5}'</span>, <span class="string">'\u{10ae6}'</span>, SC_Extend),
        (<span class="string">'\u{10b00}'</span>, <span class="string">'\u{10b35}'</span>, SC_OLetter), (<span class="string">'\u{10b40}'</span>, <span class="string">'\u{10b55}'</span>, SC_OLetter),
        (<span class="string">'\u{10b60}'</span>, <span class="string">'\u{10b72}'</span>, SC_OLetter), (<span class="string">'\u{10b80}'</span>, <span class="string">'\u{10b91}'</span>, SC_OLetter),
        (<span class="string">'\u{10c00}'</span>, <span class="string">'\u{10c48}'</span>, SC_OLetter), (<span class="string">'\u{10c80}'</span>, <span class="string">'\u{10cb2}'</span>, SC_Upper), (<span class="string">'\u{10cc0}'</span>,
        <span class="string">'\u{10cf2}'</span>, SC_Lower), (<span class="string">'\u{10d00}'</span>, <span class="string">'\u{10d23}'</span>, SC_OLetter), (<span class="string">'\u{10d24}'</span>, <span class="string">'\u{10d27}'</span>,
        SC_Extend), (<span class="string">'\u{10d30}'</span>, <span class="string">'\u{10d39}'</span>, SC_Numeric), (<span class="string">'\u{10e80}'</span>, <span class="string">'\u{10ea9}'</span>, SC_OLetter),
        (<span class="string">'\u{10eab}'</span>, <span class="string">'\u{10eac}'</span>, SC_Extend), (<span class="string">'\u{10eb0}'</span>, <span class="string">'\u{10eb1}'</span>, SC_OLetter), (<span class="string">'\u{10efd}'</span>,
        <span class="string">'\u{10eff}'</span>, SC_Extend), (<span class="string">'\u{10f00}'</span>, <span class="string">'\u{10f1c}'</span>, SC_OLetter), (<span class="string">'\u{10f27}'</span>, <span class="string">'\u{10f27}'</span>,
        SC_OLetter), (<span class="string">'\u{10f30}'</span>, <span class="string">'\u{10f45}'</span>, SC_OLetter), (<span class="string">'\u{10f46}'</span>, <span class="string">'\u{10f50}'</span>, SC_Extend),
        (<span class="string">'\u{10f55}'</span>, <span class="string">'\u{10f59}'</span>, SC_STerm), (<span class="string">'\u{10f70}'</span>, <span class="string">'\u{10f81}'</span>, SC_OLetter), (<span class="string">'\u{10f82}'</span>,
        <span class="string">'\u{10f85}'</span>, SC_Extend), (<span class="string">'\u{10f86}'</span>, <span class="string">'\u{10f89}'</span>, SC_STerm), (<span class="string">'\u{10fb0}'</span>, <span class="string">'\u{10fc4}'</span>,
        SC_OLetter), (<span class="string">'\u{10fe0}'</span>, <span class="string">'\u{10ff6}'</span>, SC_OLetter), (<span class="string">'\u{11000}'</span>, <span class="string">'\u{11002}'</span>, SC_Extend),
        (<span class="string">'\u{11003}'</span>, <span class="string">'\u{11037}'</span>, SC_OLetter), (<span class="string">'\u{11038}'</span>, <span class="string">'\u{11046}'</span>, SC_Extend), (<span class="string">'\u{11047}'</span>,
        <span class="string">'\u{11048}'</span>, SC_STerm), (<span class="string">'\u{11066}'</span>, <span class="string">'\u{1106f}'</span>, SC_Numeric), (<span class="string">'\u{11070}'</span>, <span class="string">'\u{11070}'</span>,
        SC_Extend), (<span class="string">'\u{11071}'</span>, <span class="string">'\u{11072}'</span>, SC_OLetter), (<span class="string">'\u{11073}'</span>, <span class="string">'\u{11074}'</span>, SC_Extend),
        (<span class="string">'\u{11075}'</span>, <span class="string">'\u{11075}'</span>, SC_OLetter), (<span class="string">'\u{1107f}'</span>, <span class="string">'\u{11082}'</span>, SC_Extend), (<span class="string">'\u{11083}'</span>,
        <span class="string">'\u{110af}'</span>, SC_OLetter), (<span class="string">'\u{110b0}'</span>, <span class="string">'\u{110ba}'</span>, SC_Extend), (<span class="string">'\u{110bd}'</span>, <span class="string">'\u{110bd}'</span>,
        SC_Numeric), (<span class="string">'\u{110be}'</span>, <span class="string">'\u{110c1}'</span>, SC_STerm), (<span class="string">'\u{110c2}'</span>, <span class="string">'\u{110c2}'</span>, SC_Extend),
        (<span class="string">'\u{110cd}'</span>, <span class="string">'\u{110cd}'</span>, SC_Numeric), (<span class="string">'\u{110d0}'</span>, <span class="string">'\u{110e8}'</span>, SC_OLetter),
        (<span class="string">'\u{110f0}'</span>, <span class="string">'\u{110f9}'</span>, SC_Numeric), (<span class="string">'\u{11100}'</span>, <span class="string">'\u{11102}'</span>, SC_Extend), (<span class="string">'\u{11103}'</span>,
        <span class="string">'\u{11126}'</span>, SC_OLetter), (<span class="string">'\u{11127}'</span>, <span class="string">'\u{11134}'</span>, SC_Extend), (<span class="string">'\u{11136}'</span>, <span class="string">'\u{1113f}'</span>,
        SC_Numeric), (<span class="string">'\u{11141}'</span>, <span class="string">'\u{11143}'</span>, SC_STerm), (<span class="string">'\u{11144}'</span>, <span class="string">'\u{11144}'</span>, SC_OLetter),
        (<span class="string">'\u{11145}'</span>, <span class="string">'\u{11146}'</span>, SC_Extend), (<span class="string">'\u{11147}'</span>, <span class="string">'\u{11147}'</span>, SC_OLetter), (<span class="string">'\u{11150}'</span>,
        <span class="string">'\u{11172}'</span>, SC_OLetter), (<span class="string">'\u{11173}'</span>, <span class="string">'\u{11173}'</span>, SC_Extend), (<span class="string">'\u{11176}'</span>, <span class="string">'\u{11176}'</span>,
        SC_OLetter), (<span class="string">'\u{11180}'</span>, <span class="string">'\u{11182}'</span>, SC_Extend), (<span class="string">'\u{11183}'</span>, <span class="string">'\u{111b2}'</span>, SC_OLetter),
        (<span class="string">'\u{111b3}'</span>, <span class="string">'\u{111c0}'</span>, SC_Extend), (<span class="string">'\u{111c1}'</span>, <span class="string">'\u{111c4}'</span>, SC_OLetter), (<span class="string">'\u{111c5}'</span>,
        <span class="string">'\u{111c6}'</span>, SC_STerm), (<span class="string">'\u{111c9}'</span>, <span class="string">'\u{111cc}'</span>, SC_Extend), (<span class="string">'\u{111cd}'</span>, <span class="string">'\u{111cd}'</span>,
        SC_STerm), (<span class="string">'\u{111ce}'</span>, <span class="string">'\u{111cf}'</span>, SC_Extend), (<span class="string">'\u{111d0}'</span>, <span class="string">'\u{111d9}'</span>, SC_Numeric),
        (<span class="string">'\u{111da}'</span>, <span class="string">'\u{111da}'</span>, SC_OLetter), (<span class="string">'\u{111dc}'</span>, <span class="string">'\u{111dc}'</span>, SC_OLetter),
        (<span class="string">'\u{111de}'</span>, <span class="string">'\u{111df}'</span>, SC_STerm), (<span class="string">'\u{11200}'</span>, <span class="string">'\u{11211}'</span>, SC_OLetter), (<span class="string">'\u{11213}'</span>,
        <span class="string">'\u{1122b}'</span>, SC_OLetter), (<span class="string">'\u{1122c}'</span>, <span class="string">'\u{11237}'</span>, SC_Extend), (<span class="string">'\u{11238}'</span>, <span class="string">'\u{11239}'</span>,
        SC_STerm), (<span class="string">'\u{1123b}'</span>, <span class="string">'\u{1123c}'</span>, SC_STerm), (<span class="string">'\u{1123e}'</span>, <span class="string">'\u{1123e}'</span>, SC_Extend),
        (<span class="string">'\u{1123f}'</span>, <span class="string">'\u{11240}'</span>, SC_OLetter), (<span class="string">'\u{11241}'</span>, <span class="string">'\u{11241}'</span>, SC_Extend), (<span class="string">'\u{11280}'</span>,
        <span class="string">'\u{11286}'</span>, SC_OLetter), (<span class="string">'\u{11288}'</span>, <span class="string">'\u{11288}'</span>, SC_OLetter), (<span class="string">'\u{1128a}'</span>, <span class="string">'\u{1128d}'</span>,
        SC_OLetter), (<span class="string">'\u{1128f}'</span>, <span class="string">'\u{1129d}'</span>, SC_OLetter), (<span class="string">'\u{1129f}'</span>, <span class="string">'\u{112a8}'</span>, SC_OLetter),
        (<span class="string">'\u{112a9}'</span>, <span class="string">'\u{112a9}'</span>, SC_STerm), (<span class="string">'\u{112b0}'</span>, <span class="string">'\u{112de}'</span>, SC_OLetter), (<span class="string">'\u{112df}'</span>,
        <span class="string">'\u{112ea}'</span>, SC_Extend), (<span class="string">'\u{112f0}'</span>, <span class="string">'\u{112f9}'</span>, SC_Numeric), (<span class="string">'\u{11300}'</span>, <span class="string">'\u{11303}'</span>,
        SC_Extend), (<span class="string">'\u{11305}'</span>, <span class="string">'\u{1130c}'</span>, SC_OLetter), (<span class="string">'\u{1130f}'</span>, <span class="string">'\u{11310}'</span>, SC_OLetter),
        (<span class="string">'\u{11313}'</span>, <span class="string">'\u{11328}'</span>, SC_OLetter), (<span class="string">'\u{1132a}'</span>, <span class="string">'\u{11330}'</span>, SC_OLetter),
        (<span class="string">'\u{11332}'</span>, <span class="string">'\u{11333}'</span>, SC_OLetter), (<span class="string">'\u{11335}'</span>, <span class="string">'\u{11339}'</span>, SC_OLetter),
        (<span class="string">'\u{1133b}'</span>, <span class="string">'\u{1133c}'</span>, SC_Extend), (<span class="string">'\u{1133d}'</span>, <span class="string">'\u{1133d}'</span>, SC_OLetter), (<span class="string">'\u{1133e}'</span>,
        <span class="string">'\u{11344}'</span>, SC_Extend), (<span class="string">'\u{11347}'</span>, <span class="string">'\u{11348}'</span>, SC_Extend), (<span class="string">'\u{1134b}'</span>, <span class="string">'\u{1134d}'</span>,
        SC_Extend), (<span class="string">'\u{11350}'</span>, <span class="string">'\u{11350}'</span>, SC_OLetter), (<span class="string">'\u{11357}'</span>, <span class="string">'\u{11357}'</span>, SC_Extend),
        (<span class="string">'\u{1135d}'</span>, <span class="string">'\u{11361}'</span>, SC_OLetter), (<span class="string">'\u{11362}'</span>, <span class="string">'\u{11363}'</span>, SC_Extend), (<span class="string">'\u{11366}'</span>,
        <span class="string">'\u{1136c}'</span>, SC_Extend), (<span class="string">'\u{11370}'</span>, <span class="string">'\u{11374}'</span>, SC_Extend), (<span class="string">'\u{11400}'</span>, <span class="string">'\u{11434}'</span>,
        SC_OLetter), (<span class="string">'\u{11435}'</span>, <span class="string">'\u{11446}'</span>, SC_Extend), (<span class="string">'\u{11447}'</span>, <span class="string">'\u{1144a}'</span>, SC_OLetter),
        (<span class="string">'\u{1144b}'</span>, <span class="string">'\u{1144c}'</span>, SC_STerm), (<span class="string">'\u{11450}'</span>, <span class="string">'\u{11459}'</span>, SC_Numeric), (<span class="string">'\u{1145e}'</span>,
        <span class="string">'\u{1145e}'</span>, SC_Extend), (<span class="string">'\u{1145f}'</span>, <span class="string">'\u{11461}'</span>, SC_OLetter), (<span class="string">'\u{11480}'</span>, <span class="string">'\u{114af}'</span>,
        SC_OLetter), (<span class="string">'\u{114b0}'</span>, <span class="string">'\u{114c3}'</span>, SC_Extend), (<span class="string">'\u{114c4}'</span>, <span class="string">'\u{114c5}'</span>, SC_OLetter),
        (<span class="string">'\u{114c7}'</span>, <span class="string">'\u{114c7}'</span>, SC_OLetter), (<span class="string">'\u{114d0}'</span>, <span class="string">'\u{114d9}'</span>, SC_Numeric),
        (<span class="string">'\u{11580}'</span>, <span class="string">'\u{115ae}'</span>, SC_OLetter), (<span class="string">'\u{115af}'</span>, <span class="string">'\u{115b5}'</span>, SC_Extend), (<span class="string">'\u{115b8}'</span>,
        <span class="string">'\u{115c0}'</span>, SC_Extend), (<span class="string">'\u{115c2}'</span>, <span class="string">'\u{115c3}'</span>, SC_STerm), (<span class="string">'\u{115c9}'</span>, <span class="string">'\u{115d7}'</span>,
        SC_STerm), (<span class="string">'\u{115d8}'</span>, <span class="string">'\u{115db}'</span>, SC_OLetter), (<span class="string">'\u{115dc}'</span>, <span class="string">'\u{115dd}'</span>, SC_Extend),
        (<span class="string">'\u{11600}'</span>, <span class="string">'\u{1162f}'</span>, SC_OLetter), (<span class="string">'\u{11630}'</span>, <span class="string">'\u{11640}'</span>, SC_Extend), (<span class="string">'\u{11641}'</span>,
        <span class="string">'\u{11642}'</span>, SC_STerm), (<span class="string">'\u{11644}'</span>, <span class="string">'\u{11644}'</span>, SC_OLetter), (<span class="string">'\u{11650}'</span>, <span class="string">'\u{11659}'</span>,
        SC_Numeric), (<span class="string">'\u{11680}'</span>, <span class="string">'\u{116aa}'</span>, SC_OLetter), (<span class="string">'\u{116ab}'</span>, <span class="string">'\u{116b7}'</span>, SC_Extend),
        (<span class="string">'\u{116b8}'</span>, <span class="string">'\u{116b8}'</span>, SC_OLetter), (<span class="string">'\u{116c0}'</span>, <span class="string">'\u{116c9}'</span>, SC_Numeric),
        (<span class="string">'\u{11700}'</span>, <span class="string">'\u{1171a}'</span>, SC_OLetter), (<span class="string">'\u{1171d}'</span>, <span class="string">'\u{1172b}'</span>, SC_Extend), (<span class="string">'\u{11730}'</span>,
        <span class="string">'\u{11739}'</span>, SC_Numeric), (<span class="string">'\u{1173c}'</span>, <span class="string">'\u{1173e}'</span>, SC_STerm), (<span class="string">'\u{11740}'</span>, <span class="string">'\u{11746}'</span>,
        SC_OLetter), (<span class="string">'\u{11800}'</span>, <span class="string">'\u{1182b}'</span>, SC_OLetter), (<span class="string">'\u{1182c}'</span>, <span class="string">'\u{1183a}'</span>, SC_Extend),
        (<span class="string">'\u{118a0}'</span>, <span class="string">'\u{118bf}'</span>, SC_Upper), (<span class="string">'\u{118c0}'</span>, <span class="string">'\u{118df}'</span>, SC_Lower), (<span class="string">'\u{118e0}'</span>,
        <span class="string">'\u{118e9}'</span>, SC_Numeric), (<span class="string">'\u{118ff}'</span>, <span class="string">'\u{11906}'</span>, SC_OLetter), (<span class="string">'\u{11909}'</span>, <span class="string">'\u{11909}'</span>,
        SC_OLetter), (<span class="string">'\u{1190c}'</span>, <span class="string">'\u{11913}'</span>, SC_OLetter), (<span class="string">'\u{11915}'</span>, <span class="string">'\u{11916}'</span>, SC_OLetter),
        (<span class="string">'\u{11918}'</span>, <span class="string">'\u{1192f}'</span>, SC_OLetter), (<span class="string">'\u{11930}'</span>, <span class="string">'\u{11935}'</span>, SC_Extend), (<span class="string">'\u{11937}'</span>,
        <span class="string">'\u{11938}'</span>, SC_Extend), (<span class="string">'\u{1193b}'</span>, <span class="string">'\u{1193e}'</span>, SC_Extend), (<span class="string">'\u{1193f}'</span>, <span class="string">'\u{1193f}'</span>,
        SC_OLetter), (<span class="string">'\u{11940}'</span>, <span class="string">'\u{11940}'</span>, SC_Extend), (<span class="string">'\u{11941}'</span>, <span class="string">'\u{11941}'</span>, SC_OLetter),
        (<span class="string">'\u{11942}'</span>, <span class="string">'\u{11943}'</span>, SC_Extend), (<span class="string">'\u{11944}'</span>, <span class="string">'\u{11944}'</span>, SC_STerm), (<span class="string">'\u{11946}'</span>,
        <span class="string">'\u{11946}'</span>, SC_STerm), (<span class="string">'\u{11950}'</span>, <span class="string">'\u{11959}'</span>, SC_Numeric), (<span class="string">'\u{119a0}'</span>, <span class="string">'\u{119a7}'</span>,
        SC_OLetter), (<span class="string">'\u{119aa}'</span>, <span class="string">'\u{119d0}'</span>, SC_OLetter), (<span class="string">'\u{119d1}'</span>, <span class="string">'\u{119d7}'</span>, SC_Extend),
        (<span class="string">'\u{119da}'</span>, <span class="string">'\u{119e0}'</span>, SC_Extend), (<span class="string">'\u{119e1}'</span>, <span class="string">'\u{119e1}'</span>, SC_OLetter), (<span class="string">'\u{119e3}'</span>,
        <span class="string">'\u{119e3}'</span>, SC_OLetter), (<span class="string">'\u{119e4}'</span>, <span class="string">'\u{119e4}'</span>, SC_Extend), (<span class="string">'\u{11a00}'</span>, <span class="string">'\u{11a00}'</span>,
        SC_OLetter), (<span class="string">'\u{11a01}'</span>, <span class="string">'\u{11a0a}'</span>, SC_Extend), (<span class="string">'\u{11a0b}'</span>, <span class="string">'\u{11a32}'</span>, SC_OLetter),
        (<span class="string">'\u{11a33}'</span>, <span class="string">'\u{11a39}'</span>, SC_Extend), (<span class="string">'\u{11a3a}'</span>, <span class="string">'\u{11a3a}'</span>, SC_OLetter), (<span class="string">'\u{11a3b}'</span>,
        <span class="string">'\u{11a3e}'</span>, SC_Extend), (<span class="string">'\u{11a42}'</span>, <span class="string">'\u{11a43}'</span>, SC_STerm), (<span class="string">'\u{11a47}'</span>, <span class="string">'\u{11a47}'</span>,
        SC_Extend), (<span class="string">'\u{11a50}'</span>, <span class="string">'\u{11a50}'</span>, SC_OLetter), (<span class="string">'\u{11a51}'</span>, <span class="string">'\u{11a5b}'</span>, SC_Extend),
        (<span class="string">'\u{11a5c}'</span>, <span class="string">'\u{11a89}'</span>, SC_OLetter), (<span class="string">'\u{11a8a}'</span>, <span class="string">'\u{11a99}'</span>, SC_Extend), (<span class="string">'\u{11a9b}'</span>,
        <span class="string">'\u{11a9c}'</span>, SC_STerm), (<span class="string">'\u{11a9d}'</span>, <span class="string">'\u{11a9d}'</span>, SC_OLetter), (<span class="string">'\u{11ab0}'</span>, <span class="string">'\u{11af8}'</span>,
        SC_OLetter), (<span class="string">'\u{11c00}'</span>, <span class="string">'\u{11c08}'</span>, SC_OLetter), (<span class="string">'\u{11c0a}'</span>, <span class="string">'\u{11c2e}'</span>, SC_OLetter),
        (<span class="string">'\u{11c2f}'</span>, <span class="string">'\u{11c36}'</span>, SC_Extend), (<span class="string">'\u{11c38}'</span>, <span class="string">'\u{11c3f}'</span>, SC_Extend), (<span class="string">'\u{11c40}'</span>,
        <span class="string">'\u{11c40}'</span>, SC_OLetter), (<span class="string">'\u{11c41}'</span>, <span class="string">'\u{11c42}'</span>, SC_STerm), (<span class="string">'\u{11c50}'</span>, <span class="string">'\u{11c59}'</span>,
        SC_Numeric), (<span class="string">'\u{11c72}'</span>, <span class="string">'\u{11c8f}'</span>, SC_OLetter), (<span class="string">'\u{11c92}'</span>, <span class="string">'\u{11ca7}'</span>, SC_Extend),
        (<span class="string">'\u{11ca9}'</span>, <span class="string">'\u{11cb6}'</span>, SC_Extend), (<span class="string">'\u{11d00}'</span>, <span class="string">'\u{11d06}'</span>, SC_OLetter), (<span class="string">'\u{11d08}'</span>,
        <span class="string">'\u{11d09}'</span>, SC_OLetter), (<span class="string">'\u{11d0b}'</span>, <span class="string">'\u{11d30}'</span>, SC_OLetter), (<span class="string">'\u{11d31}'</span>, <span class="string">'\u{11d36}'</span>,
        SC_Extend), (<span class="string">'\u{11d3a}'</span>, <span class="string">'\u{11d3a}'</span>, SC_Extend), (<span class="string">'\u{11d3c}'</span>, <span class="string">'\u{11d3d}'</span>, SC_Extend),
        (<span class="string">'\u{11d3f}'</span>, <span class="string">'\u{11d45}'</span>, SC_Extend), (<span class="string">'\u{11d46}'</span>, <span class="string">'\u{11d46}'</span>, SC_OLetter), (<span class="string">'\u{11d47}'</span>,
        <span class="string">'\u{11d47}'</span>, SC_Extend), (<span class="string">'\u{11d50}'</span>, <span class="string">'\u{11d59}'</span>, SC_Numeric), (<span class="string">'\u{11d60}'</span>, <span class="string">'\u{11d65}'</span>,
        SC_OLetter), (<span class="string">'\u{11d67}'</span>, <span class="string">'\u{11d68}'</span>, SC_OLetter), (<span class="string">'\u{11d6a}'</span>, <span class="string">'\u{11d89}'</span>, SC_OLetter),
        (<span class="string">'\u{11d8a}'</span>, <span class="string">'\u{11d8e}'</span>, SC_Extend), (<span class="string">'\u{11d90}'</span>, <span class="string">'\u{11d91}'</span>, SC_Extend), (<span class="string">'\u{11d93}'</span>,
        <span class="string">'\u{11d97}'</span>, SC_Extend), (<span class="string">'\u{11d98}'</span>, <span class="string">'\u{11d98}'</span>, SC_OLetter), (<span class="string">'\u{11da0}'</span>, <span class="string">'\u{11da9}'</span>,
        SC_Numeric), (<span class="string">'\u{11ee0}'</span>, <span class="string">'\u{11ef2}'</span>, SC_OLetter), (<span class="string">'\u{11ef3}'</span>, <span class="string">'\u{11ef6}'</span>, SC_Extend),
        (<span class="string">'\u{11ef7}'</span>, <span class="string">'\u{11ef8}'</span>, SC_STerm), (<span class="string">'\u{11f00}'</span>, <span class="string">'\u{11f01}'</span>, SC_Extend), (<span class="string">'\u{11f02}'</span>,
        <span class="string">'\u{11f02}'</span>, SC_OLetter), (<span class="string">'\u{11f03}'</span>, <span class="string">'\u{11f03}'</span>, SC_Extend), (<span class="string">'\u{11f04}'</span>, <span class="string">'\u{11f10}'</span>,
        SC_OLetter), (<span class="string">'\u{11f12}'</span>, <span class="string">'\u{11f33}'</span>, SC_OLetter), (<span class="string">'\u{11f34}'</span>, <span class="string">'\u{11f3a}'</span>, SC_Extend),
        (<span class="string">'\u{11f3e}'</span>, <span class="string">'\u{11f42}'</span>, SC_Extend), (<span class="string">'\u{11f43}'</span>, <span class="string">'\u{11f44}'</span>, SC_STerm), (<span class="string">'\u{11f50}'</span>,
        <span class="string">'\u{11f59}'</span>, SC_Numeric), (<span class="string">'\u{11fb0}'</span>, <span class="string">'\u{11fb0}'</span>, SC_OLetter), (<span class="string">'\u{12000}'</span>, <span class="string">'\u{12399}'</span>,
        SC_OLetter), (<span class="string">'\u{12400}'</span>, <span class="string">'\u{1246e}'</span>, SC_OLetter), (<span class="string">'\u{12480}'</span>, <span class="string">'\u{12543}'</span>, SC_OLetter),
        (<span class="string">'\u{12f90}'</span>, <span class="string">'\u{12ff0}'</span>, SC_OLetter), (<span class="string">'\u{13000}'</span>, <span class="string">'\u{1342f}'</span>, SC_OLetter),
        (<span class="string">'\u{13430}'</span>, <span class="string">'\u{1343f}'</span>, SC_Format), (<span class="string">'\u{13440}'</span>, <span class="string">'\u{13440}'</span>, SC_Extend), (<span class="string">'\u{13441}'</span>,
        <span class="string">'\u{13446}'</span>, SC_OLetter), (<span class="string">'\u{13447}'</span>, <span class="string">'\u{13455}'</span>, SC_Extend), (<span class="string">'\u{14400}'</span>, <span class="string">'\u{14646}'</span>,
        SC_OLetter), (<span class="string">'\u{16800}'</span>, <span class="string">'\u{16a38}'</span>, SC_OLetter), (<span class="string">'\u{16a40}'</span>, <span class="string">'\u{16a5e}'</span>, SC_OLetter),
        (<span class="string">'\u{16a60}'</span>, <span class="string">'\u{16a69}'</span>, SC_Numeric), (<span class="string">'\u{16a6e}'</span>, <span class="string">'\u{16a6f}'</span>, SC_STerm), (<span class="string">'\u{16a70}'</span>,
        <span class="string">'\u{16abe}'</span>, SC_OLetter), (<span class="string">'\u{16ac0}'</span>, <span class="string">'\u{16ac9}'</span>, SC_Numeric), (<span class="string">'\u{16ad0}'</span>, <span class="string">'\u{16aed}'</span>,
        SC_OLetter), (<span class="string">'\u{16af0}'</span>, <span class="string">'\u{16af4}'</span>, SC_Extend), (<span class="string">'\u{16af5}'</span>, <span class="string">'\u{16af5}'</span>, SC_STerm),
        (<span class="string">'\u{16b00}'</span>, <span class="string">'\u{16b2f}'</span>, SC_OLetter), (<span class="string">'\u{16b30}'</span>, <span class="string">'\u{16b36}'</span>, SC_Extend), (<span class="string">'\u{16b37}'</span>,
        <span class="string">'\u{16b38}'</span>, SC_STerm), (<span class="string">'\u{16b40}'</span>, <span class="string">'\u{16b43}'</span>, SC_OLetter), (<span class="string">'\u{16b44}'</span>, <span class="string">'\u{16b44}'</span>,
        SC_STerm), (<span class="string">'\u{16b50}'</span>, <span class="string">'\u{16b59}'</span>, SC_Numeric), (<span class="string">'\u{16b63}'</span>, <span class="string">'\u{16b77}'</span>, SC_OLetter),
        (<span class="string">'\u{16b7d}'</span>, <span class="string">'\u{16b8f}'</span>, SC_OLetter), (<span class="string">'\u{16e40}'</span>, <span class="string">'\u{16e5f}'</span>, SC_Upper), (<span class="string">'\u{16e60}'</span>,
        <span class="string">'\u{16e7f}'</span>, SC_Lower), (<span class="string">'\u{16e98}'</span>, <span class="string">'\u{16e98}'</span>, SC_STerm), (<span class="string">'\u{16f00}'</span>, <span class="string">'\u{16f4a}'</span>,
        SC_OLetter), (<span class="string">'\u{16f4f}'</span>, <span class="string">'\u{16f4f}'</span>, SC_Extend), (<span class="string">'\u{16f50}'</span>, <span class="string">'\u{16f50}'</span>, SC_OLetter),
        (<span class="string">'\u{16f51}'</span>, <span class="string">'\u{16f87}'</span>, SC_Extend), (<span class="string">'\u{16f8f}'</span>, <span class="string">'\u{16f92}'</span>, SC_Extend), (<span class="string">'\u{16f93}'</span>,
        <span class="string">'\u{16f9f}'</span>, SC_OLetter), (<span class="string">'\u{16fe0}'</span>, <span class="string">'\u{16fe1}'</span>, SC_OLetter), (<span class="string">'\u{16fe3}'</span>, <span class="string">'\u{16fe3}'</span>,
        SC_OLetter), (<span class="string">'\u{16fe4}'</span>, <span class="string">'\u{16fe4}'</span>, SC_Extend), (<span class="string">'\u{16ff0}'</span>, <span class="string">'\u{16ff1}'</span>, SC_Extend),
        (<span class="string">'\u{17000}'</span>, <span class="string">'\u{187f7}'</span>, SC_OLetter), (<span class="string">'\u{18800}'</span>, <span class="string">'\u{18cd5}'</span>, SC_OLetter),
        (<span class="string">'\u{18d00}'</span>, <span class="string">'\u{18d08}'</span>, SC_OLetter), (<span class="string">'\u{1aff0}'</span>, <span class="string">'\u{1aff3}'</span>, SC_OLetter),
        (<span class="string">'\u{1aff5}'</span>, <span class="string">'\u{1affb}'</span>, SC_OLetter), (<span class="string">'\u{1affd}'</span>, <span class="string">'\u{1affe}'</span>, SC_OLetter),
        (<span class="string">'\u{1b000}'</span>, <span class="string">'\u{1b122}'</span>, SC_OLetter), (<span class="string">'\u{1b132}'</span>, <span class="string">'\u{1b132}'</span>, SC_OLetter),
        (<span class="string">'\u{1b150}'</span>, <span class="string">'\u{1b152}'</span>, SC_OLetter), (<span class="string">'\u{1b155}'</span>, <span class="string">'\u{1b155}'</span>, SC_OLetter),
        (<span class="string">'\u{1b164}'</span>, <span class="string">'\u{1b167}'</span>, SC_OLetter), (<span class="string">'\u{1b170}'</span>, <span class="string">'\u{1b2fb}'</span>, SC_OLetter),
        (<span class="string">'\u{1bc00}'</span>, <span class="string">'\u{1bc6a}'</span>, SC_OLetter), (<span class="string">'\u{1bc70}'</span>, <span class="string">'\u{1bc7c}'</span>, SC_OLetter),
        (<span class="string">'\u{1bc80}'</span>, <span class="string">'\u{1bc88}'</span>, SC_OLetter), (<span class="string">'\u{1bc90}'</span>, <span class="string">'\u{1bc99}'</span>, SC_OLetter),
        (<span class="string">'\u{1bc9d}'</span>, <span class="string">'\u{1bc9e}'</span>, SC_Extend), (<span class="string">'\u{1bc9f}'</span>, <span class="string">'\u{1bc9f}'</span>, SC_STerm), (<span class="string">'\u{1bca0}'</span>,
        <span class="string">'\u{1bca3}'</span>, SC_Format), (<span class="string">'\u{1cf00}'</span>, <span class="string">'\u{1cf2d}'</span>, SC_Extend), (<span class="string">'\u{1cf30}'</span>, <span class="string">'\u{1cf46}'</span>,
        SC_Extend), (<span class="string">'\u{1d165}'</span>, <span class="string">'\u{1d169}'</span>, SC_Extend), (<span class="string">'\u{1d16d}'</span>, <span class="string">'\u{1d172}'</span>, SC_Extend),
        (<span class="string">'\u{1d173}'</span>, <span class="string">'\u{1d17a}'</span>, SC_Format), (<span class="string">'\u{1d17b}'</span>, <span class="string">'\u{1d182}'</span>, SC_Extend), (<span class="string">'\u{1d185}'</span>,
        <span class="string">'\u{1d18b}'</span>, SC_Extend), (<span class="string">'\u{1d1aa}'</span>, <span class="string">'\u{1d1ad}'</span>, SC_Extend), (<span class="string">'\u{1d242}'</span>, <span class="string">'\u{1d244}'</span>,
        SC_Extend), (<span class="string">'\u{1d400}'</span>, <span class="string">'\u{1d419}'</span>, SC_Upper), (<span class="string">'\u{1d41a}'</span>, <span class="string">'\u{1d433}'</span>, SC_Lower),
        (<span class="string">'\u{1d434}'</span>, <span class="string">'\u{1d44d}'</span>, SC_Upper), (<span class="string">'\u{1d44e}'</span>, <span class="string">'\u{1d454}'</span>, SC_Lower), (<span class="string">'\u{1d456}'</span>,
        <span class="string">'\u{1d467}'</span>, SC_Lower), (<span class="string">'\u{1d468}'</span>, <span class="string">'\u{1d481}'</span>, SC_Upper), (<span class="string">'\u{1d482}'</span>, <span class="string">'\u{1d49b}'</span>,
        SC_Lower), (<span class="string">'\u{1d49c}'</span>, <span class="string">'\u{1d49c}'</span>, SC_Upper), (<span class="string">'\u{1d49e}'</span>, <span class="string">'\u{1d49f}'</span>, SC_Upper),
        (<span class="string">'\u{1d4a2}'</span>, <span class="string">'\u{1d4a2}'</span>, SC_Upper), (<span class="string">'\u{1d4a5}'</span>, <span class="string">'\u{1d4a6}'</span>, SC_Upper), (<span class="string">'\u{1d4a9}'</span>,
        <span class="string">'\u{1d4ac}'</span>, SC_Upper), (<span class="string">'\u{1d4ae}'</span>, <span class="string">'\u{1d4b5}'</span>, SC_Upper), (<span class="string">'\u{1d4b6}'</span>, <span class="string">'\u{1d4b9}'</span>,
        SC_Lower), (<span class="string">'\u{1d4bb}'</span>, <span class="string">'\u{1d4bb}'</span>, SC_Lower), (<span class="string">'\u{1d4bd}'</span>, <span class="string">'\u{1d4c3}'</span>, SC_Lower),
        (<span class="string">'\u{1d4c5}'</span>, <span class="string">'\u{1d4cf}'</span>, SC_Lower), (<span class="string">'\u{1d4d0}'</span>, <span class="string">'\u{1d4e9}'</span>, SC_Upper), (<span class="string">'\u{1d4ea}'</span>,
        <span class="string">'\u{1d503}'</span>, SC_Lower), (<span class="string">'\u{1d504}'</span>, <span class="string">'\u{1d505}'</span>, SC_Upper), (<span class="string">'\u{1d507}'</span>, <span class="string">'\u{1d50a}'</span>,
        SC_Upper), (<span class="string">'\u{1d50d}'</span>, <span class="string">'\u{1d514}'</span>, SC_Upper), (<span class="string">'\u{1d516}'</span>, <span class="string">'\u{1d51c}'</span>, SC_Upper),
        (<span class="string">'\u{1d51e}'</span>, <span class="string">'\u{1d537}'</span>, SC_Lower), (<span class="string">'\u{1d538}'</span>, <span class="string">'\u{1d539}'</span>, SC_Upper), (<span class="string">'\u{1d53b}'</span>,
        <span class="string">'\u{1d53e}'</span>, SC_Upper), (<span class="string">'\u{1d540}'</span>, <span class="string">'\u{1d544}'</span>, SC_Upper), (<span class="string">'\u{1d546}'</span>, <span class="string">'\u{1d546}'</span>,
        SC_Upper), (<span class="string">'\u{1d54a}'</span>, <span class="string">'\u{1d550}'</span>, SC_Upper), (<span class="string">'\u{1d552}'</span>, <span class="string">'\u{1d56b}'</span>, SC_Lower),
        (<span class="string">'\u{1d56c}'</span>, <span class="string">'\u{1d585}'</span>, SC_Upper), (<span class="string">'\u{1d586}'</span>, <span class="string">'\u{1d59f}'</span>, SC_Lower), (<span class="string">'\u{1d5a0}'</span>,
        <span class="string">'\u{1d5b9}'</span>, SC_Upper), (<span class="string">'\u{1d5ba}'</span>, <span class="string">'\u{1d5d3}'</span>, SC_Lower), (<span class="string">'\u{1d5d4}'</span>, <span class="string">'\u{1d5ed}'</span>,
        SC_Upper), (<span class="string">'\u{1d5ee}'</span>, <span class="string">'\u{1d607}'</span>, SC_Lower), (<span class="string">'\u{1d608}'</span>, <span class="string">'\u{1d621}'</span>, SC_Upper),
        (<span class="string">'\u{1d622}'</span>, <span class="string">'\u{1d63b}'</span>, SC_Lower), (<span class="string">'\u{1d63c}'</span>, <span class="string">'\u{1d655}'</span>, SC_Upper), (<span class="string">'\u{1d656}'</span>,
        <span class="string">'\u{1d66f}'</span>, SC_Lower), (<span class="string">'\u{1d670}'</span>, <span class="string">'\u{1d689}'</span>, SC_Upper), (<span class="string">'\u{1d68a}'</span>, <span class="string">'\u{1d6a5}'</span>,
        SC_Lower), (<span class="string">'\u{1d6a8}'</span>, <span class="string">'\u{1d6c0}'</span>, SC_Upper), (<span class="string">'\u{1d6c2}'</span>, <span class="string">'\u{1d6da}'</span>, SC_Lower),
        (<span class="string">'\u{1d6dc}'</span>, <span class="string">'\u{1d6e1}'</span>, SC_Lower), (<span class="string">'\u{1d6e2}'</span>, <span class="string">'\u{1d6fa}'</span>, SC_Upper), (<span class="string">'\u{1d6fc}'</span>,
        <span class="string">'\u{1d714}'</span>, SC_Lower), (<span class="string">'\u{1d716}'</span>, <span class="string">'\u{1d71b}'</span>, SC_Lower), (<span class="string">'\u{1d71c}'</span>, <span class="string">'\u{1d734}'</span>,
        SC_Upper), (<span class="string">'\u{1d736}'</span>, <span class="string">'\u{1d74e}'</span>, SC_Lower), (<span class="string">'\u{1d750}'</span>, <span class="string">'\u{1d755}'</span>, SC_Lower),
        (<span class="string">'\u{1d756}'</span>, <span class="string">'\u{1d76e}'</span>, SC_Upper), (<span class="string">'\u{1d770}'</span>, <span class="string">'\u{1d788}'</span>, SC_Lower), (<span class="string">'\u{1d78a}'</span>,
        <span class="string">'\u{1d78f}'</span>, SC_Lower), (<span class="string">'\u{1d790}'</span>, <span class="string">'\u{1d7a8}'</span>, SC_Upper), (<span class="string">'\u{1d7aa}'</span>, <span class="string">'\u{1d7c2}'</span>,
        SC_Lower), (<span class="string">'\u{1d7c4}'</span>, <span class="string">'\u{1d7c9}'</span>, SC_Lower), (<span class="string">'\u{1d7ca}'</span>, <span class="string">'\u{1d7ca}'</span>, SC_Upper),
        (<span class="string">'\u{1d7cb}'</span>, <span class="string">'\u{1d7cb}'</span>, SC_Lower), (<span class="string">'\u{1d7ce}'</span>, <span class="string">'\u{1d7ff}'</span>, SC_Numeric), (<span class="string">'\u{1da00}'</span>,
        <span class="string">'\u{1da36}'</span>, SC_Extend), (<span class="string">'\u{1da3b}'</span>, <span class="string">'\u{1da6c}'</span>, SC_Extend), (<span class="string">'\u{1da75}'</span>, <span class="string">'\u{1da75}'</span>,
        SC_Extend), (<span class="string">'\u{1da84}'</span>, <span class="string">'\u{1da84}'</span>, SC_Extend), (<span class="string">'\u{1da88}'</span>, <span class="string">'\u{1da88}'</span>, SC_STerm),
        (<span class="string">'\u{1da9b}'</span>, <span class="string">'\u{1da9f}'</span>, SC_Extend), (<span class="string">'\u{1daa1}'</span>, <span class="string">'\u{1daaf}'</span>, SC_Extend), (<span class="string">'\u{1df00}'</span>,
        <span class="string">'\u{1df09}'</span>, SC_Lower), (<span class="string">'\u{1df0a}'</span>, <span class="string">'\u{1df0a}'</span>, SC_OLetter), (<span class="string">'\u{1df0b}'</span>, <span class="string">'\u{1df1e}'</span>,
        SC_Lower), (<span class="string">'\u{1df25}'</span>, <span class="string">'\u{1df2a}'</span>, SC_Lower), (<span class="string">'\u{1e000}'</span>, <span class="string">'\u{1e006}'</span>, SC_Extend),
        (<span class="string">'\u{1e008}'</span>, <span class="string">'\u{1e018}'</span>, SC_Extend), (<span class="string">'\u{1e01b}'</span>, <span class="string">'\u{1e021}'</span>, SC_Extend), (<span class="string">'\u{1e023}'</span>,
        <span class="string">'\u{1e024}'</span>, SC_Extend), (<span class="string">'\u{1e026}'</span>, <span class="string">'\u{1e02a}'</span>, SC_Extend), (<span class="string">'\u{1e030}'</span>, <span class="string">'\u{1e06d}'</span>,
        SC_Lower), (<span class="string">'\u{1e08f}'</span>, <span class="string">'\u{1e08f}'</span>, SC_Extend), (<span class="string">'\u{1e100}'</span>, <span class="string">'\u{1e12c}'</span>, SC_OLetter),
        (<span class="string">'\u{1e130}'</span>, <span class="string">'\u{1e136}'</span>, SC_Extend), (<span class="string">'\u{1e137}'</span>, <span class="string">'\u{1e13d}'</span>, SC_OLetter), (<span class="string">'\u{1e140}'</span>,
        <span class="string">'\u{1e149}'</span>, SC_Numeric), (<span class="string">'\u{1e14e}'</span>, <span class="string">'\u{1e14e}'</span>, SC_OLetter), (<span class="string">'\u{1e290}'</span>, <span class="string">'\u{1e2ad}'</span>,
        SC_OLetter), (<span class="string">'\u{1e2ae}'</span>, <span class="string">'\u{1e2ae}'</span>, SC_Extend), (<span class="string">'\u{1e2c0}'</span>, <span class="string">'\u{1e2eb}'</span>, SC_OLetter),
        (<span class="string">'\u{1e2ec}'</span>, <span class="string">'\u{1e2ef}'</span>, SC_Extend), (<span class="string">'\u{1e2f0}'</span>, <span class="string">'\u{1e2f9}'</span>, SC_Numeric), (<span class="string">'\u{1e4d0}'</span>,
        <span class="string">'\u{1e4eb}'</span>, SC_OLetter), (<span class="string">'\u{1e4ec}'</span>, <span class="string">'\u{1e4ef}'</span>, SC_Extend), (<span class="string">'\u{1e4f0}'</span>, <span class="string">'\u{1e4f9}'</span>,
        SC_Numeric), (<span class="string">'\u{1e7e0}'</span>, <span class="string">'\u{1e7e6}'</span>, SC_OLetter), (<span class="string">'\u{1e7e8}'</span>, <span class="string">'\u{1e7eb}'</span>, SC_OLetter),
        (<span class="string">'\u{1e7ed}'</span>, <span class="string">'\u{1e7ee}'</span>, SC_OLetter), (<span class="string">'\u{1e7f0}'</span>, <span class="string">'\u{1e7fe}'</span>, SC_OLetter),
        (<span class="string">'\u{1e800}'</span>, <span class="string">'\u{1e8c4}'</span>, SC_OLetter), (<span class="string">'\u{1e8d0}'</span>, <span class="string">'\u{1e8d6}'</span>, SC_Extend), (<span class="string">'\u{1e900}'</span>,
        <span class="string">'\u{1e921}'</span>, SC_Upper), (<span class="string">'\u{1e922}'</span>, <span class="string">'\u{1e943}'</span>, SC_Lower), (<span class="string">'\u{1e944}'</span>, <span class="string">'\u{1e94a}'</span>,
        SC_Extend), (<span class="string">'\u{1e94b}'</span>, <span class="string">'\u{1e94b}'</span>, SC_OLetter), (<span class="string">'\u{1e950}'</span>, <span class="string">'\u{1e959}'</span>, SC_Numeric),
        (<span class="string">'\u{1ee00}'</span>, <span class="string">'\u{1ee03}'</span>, SC_OLetter), (<span class="string">'\u{1ee05}'</span>, <span class="string">'\u{1ee1f}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee21}'</span>, <span class="string">'\u{1ee22}'</span>, SC_OLetter), (<span class="string">'\u{1ee24}'</span>, <span class="string">'\u{1ee24}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee27}'</span>, <span class="string">'\u{1ee27}'</span>, SC_OLetter), (<span class="string">'\u{1ee29}'</span>, <span class="string">'\u{1ee32}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee34}'</span>, <span class="string">'\u{1ee37}'</span>, SC_OLetter), (<span class="string">'\u{1ee39}'</span>, <span class="string">'\u{1ee39}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee3b}'</span>, <span class="string">'\u{1ee3b}'</span>, SC_OLetter), (<span class="string">'\u{1ee42}'</span>, <span class="string">'\u{1ee42}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee47}'</span>, <span class="string">'\u{1ee47}'</span>, SC_OLetter), (<span class="string">'\u{1ee49}'</span>, <span class="string">'\u{1ee49}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee4b}'</span>, <span class="string">'\u{1ee4b}'</span>, SC_OLetter), (<span class="string">'\u{1ee4d}'</span>, <span class="string">'\u{1ee4f}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee51}'</span>, <span class="string">'\u{1ee52}'</span>, SC_OLetter), (<span class="string">'\u{1ee54}'</span>, <span class="string">'\u{1ee54}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee57}'</span>, <span class="string">'\u{1ee57}'</span>, SC_OLetter), (<span class="string">'\u{1ee59}'</span>, <span class="string">'\u{1ee59}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee5b}'</span>, <span class="string">'\u{1ee5b}'</span>, SC_OLetter), (<span class="string">'\u{1ee5d}'</span>, <span class="string">'\u{1ee5d}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee5f}'</span>, <span class="string">'\u{1ee5f}'</span>, SC_OLetter), (<span class="string">'\u{1ee61}'</span>, <span class="string">'\u{1ee62}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee64}'</span>, <span class="string">'\u{1ee64}'</span>, SC_OLetter), (<span class="string">'\u{1ee67}'</span>, <span class="string">'\u{1ee6a}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee6c}'</span>, <span class="string">'\u{1ee72}'</span>, SC_OLetter), (<span class="string">'\u{1ee74}'</span>, <span class="string">'\u{1ee77}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee79}'</span>, <span class="string">'\u{1ee7c}'</span>, SC_OLetter), (<span class="string">'\u{1ee7e}'</span>, <span class="string">'\u{1ee7e}'</span>, SC_OLetter),
        (<span class="string">'\u{1ee80}'</span>, <span class="string">'\u{1ee89}'</span>, SC_OLetter), (<span class="string">'\u{1ee8b}'</span>, <span class="string">'\u{1ee9b}'</span>, SC_OLetter),
        (<span class="string">'\u{1eea1}'</span>, <span class="string">'\u{1eea3}'</span>, SC_OLetter), (<span class="string">'\u{1eea5}'</span>, <span class="string">'\u{1eea9}'</span>, SC_OLetter),
        (<span class="string">'\u{1eeab}'</span>, <span class="string">'\u{1eebb}'</span>, SC_OLetter), (<span class="string">'\u{1f130}'</span>, <span class="string">'\u{1f149}'</span>, SC_Upper), (<span class="string">'\u{1f150}'</span>,
        <span class="string">'\u{1f169}'</span>, SC_Upper), (<span class="string">'\u{1f170}'</span>, <span class="string">'\u{1f189}'</span>, SC_Upper), (<span class="string">'\u{1f676}'</span>, <span class="string">'\u{1f678}'</span>,
        SC_Close), (<span class="string">'\u{1fbf0}'</span>, <span class="string">'\u{1fbf9}'</span>, SC_Numeric), (<span class="string">'\u{20000}'</span>, <span class="string">'\u{2a6df}'</span>, SC_OLetter),
        (<span class="string">'\u{2a700}'</span>, <span class="string">'\u{2b739}'</span>, SC_OLetter), (<span class="string">'\u{2b740}'</span>, <span class="string">'\u{2b81d}'</span>, SC_OLetter),
        (<span class="string">'\u{2b820}'</span>, <span class="string">'\u{2cea1}'</span>, SC_OLetter), (<span class="string">'\u{2ceb0}'</span>, <span class="string">'\u{2ebe0}'</span>, SC_OLetter),
        (<span class="string">'\u{2ebf0}'</span>, <span class="string">'\u{2ee5d}'</span>, SC_OLetter), (<span class="string">'\u{2f800}'</span>, <span class="string">'\u{2fa1d}'</span>, SC_OLetter),
        (<span class="string">'\u{30000}'</span>, <span class="string">'\u{3134a}'</span>, SC_OLetter), (<span class="string">'\u{31350}'</span>, <span class="string">'\u{323af}'</span>, SC_OLetter),
        (<span class="string">'\u{e0001}'</span>, <span class="string">'\u{e0001}'</span>, SC_Format), (<span class="string">'\u{e0020}'</span>, <span class="string">'\u{e007f}'</span>, SC_Extend), (<span class="string">'\u{e0100}'</span>,
        <span class="string">'\u{e01ef}'</span>, SC_Extend)
    ];

}
</code></pre></div></section></main></body></html>