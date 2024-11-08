<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/keccak-0.1.5/src/lib.rs`."><title>lib.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../" data-static-root-path="../../static.files/" data-current-crate="keccak" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../src-files.js"></script><script defer src="../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../keccak/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Keccak [sponge function](https://en.wikipedia.org/wiki/Sponge_function).
//!
//! If you are looking for SHA-3 hash functions take a look at [`sha3`][1] and
//! [`tiny-keccak`][2] crates.
//!
//! To disable loop unrolling (e.g. for constraint targets) use `no_unroll`
//! feature.
//!
//! ```
//! // Test vectors are from KeccakCodePackage
//! let mut data = [0u64; 25];
//!
//! keccak::f1600(&amp;mut data);
//! assert_eq!(data, [
//!     0xF1258F7940E1DDE7, 0x84D5CCF933C0478A, 0xD598261EA65AA9EE, 0xBD1547306F80494D,
//!     0x8B284E056253D057, 0xFF97A42D7F8E6FD4, 0x90FEE5A0A44647C4, 0x8C5BDA0CD6192E76,
//!     0xAD30A6F71B19059C, 0x30935AB7D08FFC64, 0xEB5AA93F2317D635, 0xA9A6E6260D712103,
//!     0x81A57C16DBCF555F, 0x43B831CD0347C826, 0x01F22F1A11A5569F, 0x05E5635A21D9AE61,
//!     0x64BEFEF28CC970F2, 0x613670957BC46611, 0xB87C5A554FD00ECB, 0x8C3EE88A1CCF32C8,
//!     0x940C7922AE3A2614, 0x1841F924A2C509E4, 0x16F53526E70465C2, 0x75F644E97F30A13B,
//!     0xEAF1FF7B5CECA249,
//! ]);
//!
//! keccak::f1600(&amp;mut data);
//! assert_eq!(data, [
//!     0x2D5C954DF96ECB3C, 0x6A332CD07057B56D, 0x093D8D1270D76B6C, 0x8A20D9B25569D094,
//!     0x4F9C4F99E5E7F156, 0xF957B9A2DA65FB38, 0x85773DAE1275AF0D, 0xFAF4F247C3D810F7,
//!     0x1F1B9EE6F79A8759, 0xE4FECC0FEE98B425, 0x68CE61B6B9CE68A1, 0xDEEA66C4BA8F974F,
//!     0x33C43D836EAFB1F5, 0xE00654042719DBD9, 0x7CF8A9F009831265, 0xFD5449A6BF174743,
//!     0x97DDAD33D8994B40, 0x48EAD5FC5D0BE774, 0xE3B8C8EE55B7B03C, 0x91A0226E649E42E9,
//!     0x900E3129E7BADD7B, 0x202A9EC5FAA3CCE8, 0x5B3402464E1C3DB6, 0x609F4E62A44C1059,
//!     0x20D06CD26A8FBF5C,
//! ]);
//! ```
//!
//! [1]: https://docs.rs/sha3
//! [2]: https://docs.rs/tiny-keccak

</span><span class="attr">#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = <span class="string">"simd"</span>, feature(portable_simd))]
#![doc(
    html_logo_url = <span class="string">"https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"</span>,
    html_favicon_url = <span class="string">"https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"
</span>)]
#![allow(non_upper_case_globals)]
#![warn(
    clippy::mod_module_files,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

</span><span class="kw">use </span>core::{
    convert::TryInto,
    fmt::Debug,
    mem::size_of,
    ops::{BitAnd, BitAndAssign, BitXor, BitXorAssign, Not},
};

<span class="attr">#[rustfmt::skip]
</span><span class="kw">mod </span>unroll;

<span class="attr">#[cfg(all(target_arch = <span class="string">"aarch64"</span>, feature = <span class="string">"asm"</span>))]
</span><span class="kw">mod </span>armv8;

<span class="attr">#[cfg(all(target_arch = <span class="string">"aarch64"</span>, feature = <span class="string">"asm"</span>))]
</span><span class="macro">cpufeatures::new!</span>(armv8_sha3_intrinsics, <span class="string">"sha3"</span>);

<span class="kw">const </span>PLEN: usize = <span class="number">25</span>;

<span class="kw">const </span>RHO: [u32; <span class="number">24</span>] = [
    <span class="number">1</span>, <span class="number">3</span>, <span class="number">6</span>, <span class="number">10</span>, <span class="number">15</span>, <span class="number">21</span>, <span class="number">28</span>, <span class="number">36</span>, <span class="number">45</span>, <span class="number">55</span>, <span class="number">2</span>, <span class="number">14</span>, <span class="number">27</span>, <span class="number">41</span>, <span class="number">56</span>, <span class="number">8</span>, <span class="number">25</span>, <span class="number">43</span>, <span class="number">62</span>, <span class="number">18</span>, <span class="number">39</span>, <span class="number">61</span>, <span class="number">20</span>, <span class="number">44</span>,
];

<span class="kw">const </span>PI: [usize; <span class="number">24</span>] = [
    <span class="number">10</span>, <span class="number">7</span>, <span class="number">11</span>, <span class="number">17</span>, <span class="number">18</span>, <span class="number">3</span>, <span class="number">5</span>, <span class="number">16</span>, <span class="number">8</span>, <span class="number">21</span>, <span class="number">24</span>, <span class="number">4</span>, <span class="number">15</span>, <span class="number">23</span>, <span class="number">19</span>, <span class="number">13</span>, <span class="number">12</span>, <span class="number">2</span>, <span class="number">20</span>, <span class="number">14</span>, <span class="number">22</span>, <span class="number">9</span>, <span class="number">6</span>, <span class="number">1</span>,
];

<span class="kw">const </span>RC: [u64; <span class="number">24</span>] = [
    <span class="number">0x0000000000000001</span>,
    <span class="number">0x0000000000008082</span>,
    <span class="number">0x800000000000808a</span>,
    <span class="number">0x8000000080008000</span>,
    <span class="number">0x000000000000808b</span>,
    <span class="number">0x0000000080000001</span>,
    <span class="number">0x8000000080008081</span>,
    <span class="number">0x8000000000008009</span>,
    <span class="number">0x000000000000008a</span>,
    <span class="number">0x0000000000000088</span>,
    <span class="number">0x0000000080008009</span>,
    <span class="number">0x000000008000000a</span>,
    <span class="number">0x000000008000808b</span>,
    <span class="number">0x800000000000008b</span>,
    <span class="number">0x8000000000008089</span>,
    <span class="number">0x8000000000008003</span>,
    <span class="number">0x8000000000008002</span>,
    <span class="number">0x8000000000000080</span>,
    <span class="number">0x000000000000800a</span>,
    <span class="number">0x800000008000000a</span>,
    <span class="number">0x8000000080008081</span>,
    <span class="number">0x8000000000008080</span>,
    <span class="number">0x0000000080000001</span>,
    <span class="number">0x8000000080008008</span>,
];

<span class="doccomment">/// Keccak is a permutation over an array of lanes which comprise the sponge
/// construction.
</span><span class="kw">pub trait </span>LaneSize:
    Copy
    + Clone
    + Debug
    + Default
    + PartialEq
    + BitAndAssign
    + BitAnd&lt;Output = <span class="self">Self</span>&gt;
    + BitXorAssign
    + BitXor&lt;Output = <span class="self">Self</span>&gt;
    + Not&lt;Output = <span class="self">Self</span>&gt;
{
    <span class="doccomment">/// Number of rounds of the Keccak-f permutation.
    </span><span class="kw">const </span>KECCAK_F_ROUND_COUNT: usize;

    <span class="doccomment">/// Truncate function.
    </span><span class="kw">fn </span>truncate_rc(rc: u64) -&gt; <span class="self">Self</span>;

    <span class="doccomment">/// Rotate left function.
    </span><span class="kw">fn </span>rotate_left(<span class="self">self</span>, n: u32) -&gt; <span class="self">Self</span>;
}

<span class="macro">macro_rules!</span> impl_lanesize {
    (<span class="macro-nonterminal">$</span><span class="kw">type</span>:<span class="macro-nonterminal">ty</span>, <span class="macro-nonterminal">$round</span>:expr, <span class="macro-nonterminal">$truncate</span>:expr) =&gt; {
        <span class="kw">impl </span>LaneSize <span class="kw">for </span><span class="macro-nonterminal">$</span><span class="kw">type </span>{
            <span class="kw">const </span><span class="macro-nonterminal">KECCAK_F_ROUND_COUNT</span>: usize = <span class="macro-nonterminal">$round</span>;

            <span class="kw">fn </span>truncate_rc(rc: u64) -&gt; <span class="self">Self </span>{
                <span class="macro-nonterminal">$truncate</span>(rc)
            }

            <span class="kw">fn </span>rotate_left(<span class="self">self</span>, n: u32) -&gt; <span class="self">Self </span>{
                <span class="self">self</span>.rotate_left(n)
            }
        }
    };
}

<span class="macro">impl_lanesize!</span>(u8, <span class="number">18</span>, |rc: u64| { rc.to_le_bytes()[<span class="number">0</span>] });
<span class="macro">impl_lanesize!</span>(u16, <span class="number">20</span>, |rc: u64| {
    <span class="kw">let </span>tmp = rc.to_le_bytes();
    <span class="attr">#[allow(clippy::unwrap_used)]
    </span><span class="self">Self</span>::from_le_bytes(tmp[..size_of::&lt;<span class="self">Self</span>&gt;()].try_into().unwrap())
});
<span class="macro">impl_lanesize!</span>(u32, <span class="number">22</span>, |rc: u64| {
    <span class="kw">let </span>tmp = rc.to_le_bytes();
    <span class="attr">#[allow(clippy::unwrap_used)]
    </span><span class="self">Self</span>::from_le_bytes(tmp[..size_of::&lt;<span class="self">Self</span>&gt;()].try_into().unwrap())
});
<span class="macro">impl_lanesize!</span>(u64, <span class="number">24</span>, |rc: u64| { rc });

<span class="macro">macro_rules!</span> impl_keccak {
    (<span class="macro-nonterminal">$pname</span>:ident, <span class="macro-nonterminal">$fname</span>:ident, <span class="macro-nonterminal">$</span><span class="kw">type</span>:<span class="macro-nonterminal">ty</span>) =&gt; {
        <span class="doccomment">/// Keccak-p sponge function
        </span><span class="kw">pub fn </span><span class="macro-nonterminal">$pname</span>(state: <span class="kw-2">&amp;mut </span>[<span class="macro-nonterminal">$</span><span class="kw">type</span>; <span class="macro-nonterminal">PLEN</span>], round_count: usize) {
            keccak_p(state, round_count);
        }

        <span class="doccomment">/// Keccak-f sponge function
        </span><span class="kw">pub fn </span><span class="macro-nonterminal">$fname</span>(state: <span class="kw-2">&amp;mut </span>[<span class="macro-nonterminal">$</span><span class="kw">type</span>; <span class="macro-nonterminal">PLEN</span>]) {
            keccak_p(state, &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::KECCAK_F_ROUND_COUNT</span>);
        }
    };
}

<span class="macro">impl_keccak!</span>(p200, f200, u8);
<span class="macro">impl_keccak!</span>(p400, f400, u16);
<span class="macro">impl_keccak!</span>(p800, f800, u32);

<span class="attr">#[cfg(not(all(target_arch = <span class="string">"aarch64"</span>, feature = <span class="string">"asm"</span>)))]
</span><span class="macro">impl_keccak!</span>(p1600, f1600, u64);

<span class="doccomment">/// Keccak-p[1600, rc] permutation.
</span><span class="attr">#[cfg(all(target_arch = <span class="string">"aarch64"</span>, feature = <span class="string">"asm"</span>))]
</span><span class="kw">pub fn </span>p1600(state: <span class="kw-2">&amp;mut </span>[u64; PLEN], round_count: usize) {
    <span class="kw">if </span>armv8_sha3_intrinsics::get() {
        <span class="kw">unsafe </span>{ armv8::p1600_armv8_sha3_asm(state, round_count) }
    } <span class="kw">else </span>{
        keccak_p(state, round_count);
    }
}

<span class="doccomment">/// Keccak-f[1600] permutation.
</span><span class="attr">#[cfg(all(target_arch = <span class="string">"aarch64"</span>, feature = <span class="string">"asm"</span>))]
</span><span class="kw">pub fn </span>f1600(state: <span class="kw-2">&amp;mut </span>[u64; PLEN]) {
    <span class="kw">if </span>armv8_sha3_intrinsics::get() {
        <span class="kw">unsafe </span>{ armv8::p1600_armv8_sha3_asm(state, <span class="number">24</span>) }
    } <span class="kw">else </span>{
        keccak_p(state, u64::KECCAK_F_ROUND_COUNT);
    }
}

<span class="attr">#[cfg(feature = <span class="string">"simd"</span>)]
</span><span class="doccomment">/// SIMD implementations for Keccak-f1600 sponge function
</span><span class="kw">pub mod </span>simd {
    <span class="kw">use crate</span>::{keccak_p, LaneSize, PLEN};
    <span class="kw">pub use </span>core::simd::{u64x2, u64x4, u64x8};

    <span class="macro">macro_rules!</span> impl_lanesize_simd_u64xn {
        (<span class="macro-nonterminal">$</span><span class="kw">type</span>:<span class="macro-nonterminal">ty</span>) =&gt; {
            <span class="kw">impl </span>LaneSize <span class="kw">for </span><span class="macro-nonterminal">$</span><span class="kw">type </span>{
                <span class="kw">const </span><span class="macro-nonterminal">KECCAK_F_ROUND_COUNT</span>: usize = <span class="number">24</span>;

                <span class="kw">fn </span>truncate_rc(rc: u64) -&gt; <span class="self">Self </span>{
                    <span class="self">Self</span>::splat(rc)
                }

                <span class="kw">fn </span>rotate_left(<span class="self">self</span>, n: u32) -&gt; <span class="self">Self </span>{
                    <span class="self">self </span>&lt;&lt; <span class="self">Self</span>::splat(n.into()) | <span class="self">self </span>&gt;&gt; <span class="self">Self</span>::splat((<span class="number">64 </span>- n).into())
                }
            }
        };
    }

    <span class="macro">impl_lanesize_simd_u64xn!</span>(u64x2);
    <span class="macro">impl_lanesize_simd_u64xn!</span>(u64x4);
    <span class="macro">impl_lanesize_simd_u64xn!</span>(u64x8);

    <span class="macro">impl_keccak!</span>(p1600x2, f1600x2, u64x2);
    <span class="macro">impl_keccak!</span>(p1600x4, f1600x4, u64x4);
    <span class="macro">impl_keccak!</span>(p1600x8, f1600x8, u64x8);
}

<span class="attr">#[allow(unused_assignments)]
</span><span class="doccomment">/// Generic Keccak-p sponge function
</span><span class="kw">pub fn </span>keccak_p&lt;L: LaneSize&gt;(state: <span class="kw-2">&amp;mut </span>[L; PLEN], round_count: usize) {
    <span class="kw">if </span>round_count &gt; L::KECCAK_F_ROUND_COUNT {
        <span class="macro">panic!</span>(<span class="string">"A round_count greater than KECCAK_F_ROUND_COUNT is not supported!"</span>);
    }

    <span class="comment">// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf#page=25
    // "the rounds of KECCAK-p[b, nr] match the last rounds of KECCAK-f[b]"
    </span><span class="kw">let </span>round_consts = <span class="kw-2">&amp;</span>RC[(L::KECCAK_F_ROUND_COUNT - round_count)..L::KECCAK_F_ROUND_COUNT];

    <span class="comment">// not unrolling this loop results in a much smaller function, plus
    // it positively influences performance due to the smaller load on I-cache
    </span><span class="kw">for </span><span class="kw-2">&amp;</span>rc <span class="kw">in </span>round_consts {
        <span class="kw">let </span><span class="kw-2">mut </span>array = [L::default(); <span class="number">5</span>];

        <span class="comment">// Theta
        </span><span class="macro">unroll5!</span>(x, {
            <span class="macro">unroll5!</span>(y, {
                array[x] ^= state[<span class="number">5 </span>* y + x];
            });
        });

        <span class="macro">unroll5!</span>(x, {
            <span class="macro">unroll5!</span>(y, {
                <span class="kw">let </span>t1 = array[(x + <span class="number">4</span>) % <span class="number">5</span>];
                <span class="kw">let </span>t2 = array[(x + <span class="number">1</span>) % <span class="number">5</span>].rotate_left(<span class="number">1</span>);
                state[<span class="number">5 </span>* y + x] ^= t1 ^ t2;
            });
        });

        <span class="comment">// Rho and pi
        </span><span class="kw">let </span><span class="kw-2">mut </span>last = state[<span class="number">1</span>];
        <span class="macro">unroll24!</span>(x, {
            array[<span class="number">0</span>] = state[PI[x]];
            state[PI[x]] = last.rotate_left(RHO[x]);
            last = array[<span class="number">0</span>];
        });

        <span class="comment">// Chi
        </span><span class="macro">unroll5!</span>(y_step, {
            <span class="kw">let </span>y = <span class="number">5 </span>* y_step;

            <span class="macro">unroll5!</span>(x, {
                array[x] = state[y + x];
            });

            <span class="macro">unroll5!</span>(x, {
                <span class="kw">let </span>t1 = !array[(x + <span class="number">1</span>) % <span class="number">5</span>];
                <span class="kw">let </span>t2 = array[(x + <span class="number">2</span>) % <span class="number">5</span>];
                state[y + x] = array[x] ^ (t1 &amp; t2);
            });
        });

        <span class="comment">// Iota
        </span>state[<span class="number">0</span>] ^= L::truncate_rc(rc);
    }
}

<span class="attr">#[cfg(test)]
</span><span class="kw">mod </span>tests {
    <span class="kw">use crate</span>::{keccak_p, LaneSize, PLEN};

    <span class="kw">fn </span>keccak_f&lt;L: LaneSize&gt;(state_first: [L; PLEN], state_second: [L; PLEN]) {
        <span class="kw">let </span><span class="kw-2">mut </span>state = [L::default(); PLEN];

        keccak_p(<span class="kw-2">&amp;mut </span>state, L::KECCAK_F_ROUND_COUNT);
        <span class="macro">assert_eq!</span>(state, state_first);

        keccak_p(<span class="kw-2">&amp;mut </span>state, L::KECCAK_F_ROUND_COUNT);
        <span class="macro">assert_eq!</span>(state, state_second);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>keccak_f200() {
        <span class="comment">// Test vectors are copied from XKCP (eXtended Keccak Code Package)
        // https://github.com/XKCP/XKCP/blob/master/tests/TestVectors/KeccakF-200-IntermediateValues.txt
        </span><span class="kw">let </span>state_first = [
            <span class="number">0x3C</span>, <span class="number">0x28</span>, <span class="number">0x26</span>, <span class="number">0x84</span>, <span class="number">0x1C</span>, <span class="number">0xB3</span>, <span class="number">0x5C</span>, <span class="number">0x17</span>, <span class="number">0x1E</span>, <span class="number">0xAA</span>, <span class="number">0xE9</span>, <span class="number">0xB8</span>, <span class="number">0x11</span>, <span class="number">0x13</span>,
            <span class="number">0x4C</span>, <span class="number">0xEA</span>, <span class="number">0xA3</span>, <span class="number">0x85</span>, <span class="number">0x2C</span>, <span class="number">0x69</span>, <span class="number">0xD2</span>, <span class="number">0xC5</span>, <span class="number">0xAB</span>, <span class="number">0xAF</span>, <span class="number">0xEA</span>,
        ];
        <span class="kw">let </span>state_second = [
            <span class="number">0x1B</span>, <span class="number">0xEF</span>, <span class="number">0x68</span>, <span class="number">0x94</span>, <span class="number">0x92</span>, <span class="number">0xA8</span>, <span class="number">0xA5</span>, <span class="number">0x43</span>, <span class="number">0xA5</span>, <span class="number">0x99</span>, <span class="number">0x9F</span>, <span class="number">0xDB</span>, <span class="number">0x83</span>, <span class="number">0x4E</span>,
            <span class="number">0x31</span>, <span class="number">0x66</span>, <span class="number">0xA1</span>, <span class="number">0x4B</span>, <span class="number">0xE8</span>, <span class="number">0x27</span>, <span class="number">0xD9</span>, <span class="number">0x50</span>, <span class="number">0x40</span>, <span class="number">0x47</span>, <span class="number">0x9E</span>,
        ];

        keccak_f::&lt;u8&gt;(state_first, state_second);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>keccak_f400() {
        <span class="comment">// Test vectors are copied from XKCP (eXtended Keccak Code Package)
        // https://github.com/XKCP/XKCP/blob/master/tests/TestVectors/KeccakF-400-IntermediateValues.txt
        </span><span class="kw">let </span>state_first = [
            <span class="number">0x09F5</span>, <span class="number">0x40AC</span>, <span class="number">0x0FA9</span>, <span class="number">0x14F5</span>, <span class="number">0xE89F</span>, <span class="number">0xECA0</span>, <span class="number">0x5BD1</span>, <span class="number">0x7870</span>, <span class="number">0xEFF0</span>, <span class="number">0xBF8F</span>, <span class="number">0x0337</span>,
            <span class="number">0x6052</span>, <span class="number">0xDC75</span>, <span class="number">0x0EC9</span>, <span class="number">0xE776</span>, <span class="number">0x5246</span>, <span class="number">0x59A1</span>, <span class="number">0x5D81</span>, <span class="number">0x6D95</span>, <span class="number">0x6E14</span>, <span class="number">0x633E</span>, <span class="number">0x58EE</span>,
            <span class="number">0x71FF</span>, <span class="number">0x714C</span>, <span class="number">0xB38E</span>,
        ];
        <span class="kw">let </span>state_second = [
            <span class="number">0xE537</span>, <span class="number">0xD5D6</span>, <span class="number">0xDBE7</span>, <span class="number">0xAAF3</span>, <span class="number">0x9BC7</span>, <span class="number">0xCA7D</span>, <span class="number">0x86B2</span>, <span class="number">0xFDEC</span>, <span class="number">0x692C</span>, <span class="number">0x4E5B</span>, <span class="number">0x67B1</span>,
            <span class="number">0x15AD</span>, <span class="number">0xA7F7</span>, <span class="number">0xA66F</span>, <span class="number">0x67FF</span>, <span class="number">0x3F8A</span>, <span class="number">0x2F99</span>, <span class="number">0xE2C2</span>, <span class="number">0x656B</span>, <span class="number">0x5F31</span>, <span class="number">0x5BA6</span>, <span class="number">0xCA29</span>,
            <span class="number">0xC224</span>, <span class="number">0xB85C</span>, <span class="number">0x097C</span>,
        ];

        keccak_f::&lt;u16&gt;(state_first, state_second);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>keccak_f800() {
        <span class="comment">// Test vectors are copied from XKCP (eXtended Keccak Code Package)
        // https://github.com/XKCP/XKCP/blob/master/tests/TestVectors/KeccakF-800-IntermediateValues.txt
        </span><span class="kw">let </span>state_first = [
            <span class="number">0xE531D45D</span>, <span class="number">0xF404C6FB</span>, <span class="number">0x23A0BF99</span>, <span class="number">0xF1F8452F</span>, <span class="number">0x51FFD042</span>, <span class="number">0xE539F578</span>, <span class="number">0xF00B80A7</span>,
            <span class="number">0xAF973664</span>, <span class="number">0xBF5AF34C</span>, <span class="number">0x227A2424</span>, <span class="number">0x88172715</span>, <span class="number">0x9F685884</span>, <span class="number">0xB15CD054</span>, <span class="number">0x1BF4FC0E</span>,
            <span class="number">0x6166FA91</span>, <span class="number">0x1A9E599A</span>, <span class="number">0xA3970A1F</span>, <span class="number">0xAB659687</span>, <span class="number">0xAFAB8D68</span>, <span class="number">0xE74B1015</span>, <span class="number">0x34001A98</span>,
            <span class="number">0x4119EFF3</span>, <span class="number">0x930A0E76</span>, <span class="number">0x87B28070</span>, <span class="number">0x11EFE996</span>,
        ];
        <span class="kw">let </span>state_second = [
            <span class="number">0x75BF2D0D</span>, <span class="number">0x9B610E89</span>, <span class="number">0xC826AF40</span>, <span class="number">0x64CD84AB</span>, <span class="number">0xF905BDD6</span>, <span class="number">0xBC832835</span>, <span class="number">0x5F8001B9</span>,
            <span class="number">0x15662CCE</span>, <span class="number">0x8E38C95E</span>, <span class="number">0x701FE543</span>, <span class="number">0x1B544380</span>, <span class="number">0x89ACDEFF</span>, <span class="number">0x51EDB5DE</span>, <span class="number">0x0E9702D9</span>,
            <span class="number">0x6C19AA16</span>, <span class="number">0xA2913EEE</span>, <span class="number">0x60754E9A</span>, <span class="number">0x9819063C</span>, <span class="number">0xF4709254</span>, <span class="number">0xD09F9084</span>, <span class="number">0x772DA259</span>,
            <span class="number">0x1DB35DF7</span>, <span class="number">0x5AA60162</span>, <span class="number">0x358825D5</span>, <span class="number">0xB3783BAB</span>,
        ];

        keccak_f::&lt;u32&gt;(state_first, state_second);
    }

    <span class="attr">#[test]
    </span><span class="kw">fn </span>keccak_f1600() {
        <span class="comment">// Test vectors are copied from XKCP (eXtended Keccak Code Package)
        // https://github.com/XKCP/XKCP/blob/master/tests/TestVectors/KeccakF-1600-IntermediateValues.txt
        </span><span class="kw">let </span>state_first = [
            <span class="number">0xF1258F7940E1DDE7</span>,
            <span class="number">0x84D5CCF933C0478A</span>,
            <span class="number">0xD598261EA65AA9EE</span>,
            <span class="number">0xBD1547306F80494D</span>,
            <span class="number">0x8B284E056253D057</span>,
            <span class="number">0xFF97A42D7F8E6FD4</span>,
            <span class="number">0x90FEE5A0A44647C4</span>,
            <span class="number">0x8C5BDA0CD6192E76</span>,
            <span class="number">0xAD30A6F71B19059C</span>,
            <span class="number">0x30935AB7D08FFC64</span>,
            <span class="number">0xEB5AA93F2317D635</span>,
            <span class="number">0xA9A6E6260D712103</span>,
            <span class="number">0x81A57C16DBCF555F</span>,
            <span class="number">0x43B831CD0347C826</span>,
            <span class="number">0x01F22F1A11A5569F</span>,
            <span class="number">0x05E5635A21D9AE61</span>,
            <span class="number">0x64BEFEF28CC970F2</span>,
            <span class="number">0x613670957BC46611</span>,
            <span class="number">0xB87C5A554FD00ECB</span>,
            <span class="number">0x8C3EE88A1CCF32C8</span>,
            <span class="number">0x940C7922AE3A2614</span>,
            <span class="number">0x1841F924A2C509E4</span>,
            <span class="number">0x16F53526E70465C2</span>,
            <span class="number">0x75F644E97F30A13B</span>,
            <span class="number">0xEAF1FF7B5CECA249</span>,
        ];
        <span class="kw">let </span>state_second = [
            <span class="number">0x2D5C954DF96ECB3C</span>,
            <span class="number">0x6A332CD07057B56D</span>,
            <span class="number">0x093D8D1270D76B6C</span>,
            <span class="number">0x8A20D9B25569D094</span>,
            <span class="number">0x4F9C4F99E5E7F156</span>,
            <span class="number">0xF957B9A2DA65FB38</span>,
            <span class="number">0x85773DAE1275AF0D</span>,
            <span class="number">0xFAF4F247C3D810F7</span>,
            <span class="number">0x1F1B9EE6F79A8759</span>,
            <span class="number">0xE4FECC0FEE98B425</span>,
            <span class="number">0x68CE61B6B9CE68A1</span>,
            <span class="number">0xDEEA66C4BA8F974F</span>,
            <span class="number">0x33C43D836EAFB1F5</span>,
            <span class="number">0xE00654042719DBD9</span>,
            <span class="number">0x7CF8A9F009831265</span>,
            <span class="number">0xFD5449A6BF174743</span>,
            <span class="number">0x97DDAD33D8994B40</span>,
            <span class="number">0x48EAD5FC5D0BE774</span>,
            <span class="number">0xE3B8C8EE55B7B03C</span>,
            <span class="number">0x91A0226E649E42E9</span>,
            <span class="number">0x900E3129E7BADD7B</span>,
            <span class="number">0x202A9EC5FAA3CCE8</span>,
            <span class="number">0x5B3402464E1C3DB6</span>,
            <span class="number">0x609F4E62A44C1059</span>,
            <span class="number">0x20D06CD26A8FBF5C</span>,
        ];

        keccak_f::&lt;u64&gt;(state_first, state_second);
    }

    <span class="attr">#[cfg(feature = <span class="string">"simd"</span>)]
    </span><span class="kw">mod </span>simd {
        <span class="kw">use </span><span class="kw">super</span>::keccak_f;
        <span class="kw">use </span>core::simd::{u64x2, u64x4, u64x8};

        <span class="macro">macro_rules!</span> impl_keccak_f1600xn {
            (<span class="macro-nonterminal">$name</span>:ident, <span class="macro-nonterminal">$</span><span class="kw">type</span>:<span class="macro-nonterminal">ty</span>) =&gt; {
                <span class="attr">#[test]
                </span><span class="kw">fn </span><span class="macro-nonterminal">$name</span>() {
                    <span class="comment">// Test vectors are copied from XKCP (eXtended Keccak Code Package)
                    // https://github.com/XKCP/XKCP/blob/master/tests/TestVectors/KeccakF-1600-IntermediateValues.txt
                    </span><span class="kw">let </span>state_first = [
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xF1258F7940E1DDE7</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x84D5CCF933C0478A</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xD598261EA65AA9EE</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xBD1547306F80494D</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x8B284E056253D057</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xFF97A42D7F8E6FD4</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x90FEE5A0A44647C4</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x8C5BDA0CD6192E76</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xAD30A6F71B19059C</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x30935AB7D08FFC64</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xEB5AA93F2317D635</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xA9A6E6260D712103</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x81A57C16DBCF555F</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x43B831CD0347C826</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x01F22F1A11A5569F</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x05E5635A21D9AE61</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x64BEFEF28CC970F2</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x613670957BC46611</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xB87C5A554FD00ECB</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x8C3EE88A1CCF32C8</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x940C7922AE3A2614</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x1841F924A2C509E4</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x16F53526E70465C2</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x75F644E97F30A13B</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xEAF1FF7B5CECA249</span>),
                    ];
                    <span class="kw">let </span>state_second = [
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x2D5C954DF96ECB3C</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x6A332CD07057B56D</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x093D8D1270D76B6C</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x8A20D9B25569D094</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x4F9C4F99E5E7F156</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xF957B9A2DA65FB38</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x85773DAE1275AF0D</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xFAF4F247C3D810F7</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x1F1B9EE6F79A8759</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xE4FECC0FEE98B425</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x68CE61B6B9CE68A1</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xDEEA66C4BA8F974F</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x33C43D836EAFB1F5</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xE00654042719DBD9</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x7CF8A9F009831265</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xFD5449A6BF174743</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x97DDAD33D8994B40</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x48EAD5FC5D0BE774</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0xE3B8C8EE55B7B03C</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x91A0226E649E42E9</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x900E3129E7BADD7B</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x202A9EC5FAA3CCE8</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x5B3402464E1C3DB6</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x609F4E62A44C1059</span>),
                        &lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;<span class="macro-nonterminal">::splat</span>(<span class="number">0x20D06CD26A8FBF5C</span>),
                    ];

                    keccak_f::&lt;<span class="macro-nonterminal">$</span><span class="kw">type</span>&gt;(<span class="macro-nonterminal">state_first</span>, state_second);
                }
            };
        }

        <span class="macro">impl_keccak_f1600xn!</span>(keccak_f1600x2, u64x2);
        <span class="macro">impl_keccak_f1600xn!</span>(keccak_f1600x4, u64x4);
        <span class="macro">impl_keccak_f1600xn!</span>(keccak_f1600x8, u64x8);
    }
}
</code></pre></div></section></main></body></html>