<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="Source of the Rust file `/Users/jeyakumar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/aes-0.7.5/src/soft/fixslice64.rs`."><title>fixslice64.rs - source</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-46f98efaafac5295.ttf.woff2,FiraSans-Regular-018c141bf0843ffd.woff2,FiraSans-Medium-8f9a781e4970d388.woff2,SourceCodePro-Regular-562dcc5011b6de7d.ttf.woff2,SourceCodePro-Semibold-d899c5a5c4aeb14a.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2" crossorigin href="../../../static.files/${f}">`).join(""))</script><link rel="stylesheet" href="../../../static.files/normalize-76eba96aa4d2e634.css"><link rel="stylesheet" href="../../../static.files/rustdoc-081576b923113409.css"><meta name="rustdoc-vars" data-root-path="../../../" data-static-root-path="../../../static.files/" data-current-crate="aes" data-themes="" data-resource-suffix="" data-rustdoc-version="1.79.0 (129f3b996 2024-06-10)" data-channel="1.79.0" data-search-js="search-bf21c90c8c1d92b1.js" data-settings-js="settings-4313503d2e1961c2.js" ><script src="../../../static.files/storage-e32f0c247825364d.js"></script><script defer src="../../../static.files/src-script-e66d777a5a92e9b2.js"></script><script defer src="../../../src-files.js"></script><script defer src="../../../static.files/main-20a3ad099b048cf2.js"></script><noscript><link rel="stylesheet" href="../../../static.files/noscript-09095024cf37855e.css"></noscript><link rel="icon" href="https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg"></head><body class="rustdoc src"><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><nav class="sidebar"><div class="src-sidebar-title"><h2>Files</h2></div></nav><div class="sidebar-resizer"></div><main><nav class="sub"><form class="search-form"><span></span><div id="sidebar-button" tabindex="-1"><a href="../../../aes/all.html" title="show sidebar"></a></div><input class="search-input" name="search" aria-label="Run search in the documentation" autocomplete="off" spellcheck="false" placeholder="Type ‘S’ or ‘/’ to search, ‘?’ for more options…" type="search"><div id="help-button" tabindex="-1"><a href="../../../help.html" title="help">?</a></div><div id="settings-menu" tabindex="-1"><a href="../../../settings.html" title="settings">Settings</a></div></form></nav><section id="main-content" class="content"><div class="example-wrap"><div data-nosnippet><pre class="src-line-numbers"><a href="#1" id="1">1</a>
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
</pre></div><pre class="rust"><code><span class="doccomment">//! Fixsliced implementations of AES-128, AES-192 and AES-256 (64-bit)
//! adapted from the C implementation.
//!
//! All implementations are fully bitsliced and do not rely on any
//! Look-Up Table (LUT).
//!
//! See the paper at &lt;https://eprint.iacr.org/2020/1123.pdf&gt; for more details.
//!
//! # Author (original C code)
//!
//! Alexandre Adomnicai, Nanyang Technological University, Singapore
//! &lt;alexandre.adomnicai@ntu.edu.sg&gt;
//!
//! Originally licensed MIT. Relicensed as Apache 2.0+MIT with permission.

</span><span class="attr">#![allow(clippy::unreadable_literal)]

</span><span class="kw">use </span><span class="kw">crate</span>::Block;
<span class="kw">use </span>cipher::{
    consts::{U16, U24, U32},
    generic_array::GenericArray,
};

<span class="doccomment">/// AES block batch size for this implementation
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">const </span>FIXSLICE_BLOCKS: usize = <span class="number">4</span>;

<span class="doccomment">/// AES-128 round keys
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">type </span>FixsliceKeys128 = [u64; <span class="number">88</span>];

<span class="doccomment">/// AES-192 round keys
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">type </span>FixsliceKeys192 = [u64; <span class="number">104</span>];

<span class="doccomment">/// AES-256 round keys
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">type </span>FixsliceKeys256 = [u64; <span class="number">120</span>];

<span class="doccomment">/// 512-bit internal state
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">type </span>State = [u64; <span class="number">8</span>];

<span class="doccomment">/// Fully bitsliced AES-128 key schedule to match the fully-fixsliced representation.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>aes128_key_schedule(key: <span class="kw-2">&amp;</span>GenericArray&lt;u8, U16&gt;) -&gt; FixsliceKeys128 {
    <span class="kw">let </span><span class="kw-2">mut </span>rkeys = [<span class="number">0u64</span>; <span class="number">88</span>];

    bitslice(<span class="kw-2">&amp;mut </span>rkeys[..<span class="number">8</span>], key, key, key, key);

    <span class="kw">let </span><span class="kw-2">mut </span>rk_off = <span class="number">0</span>;
    <span class="kw">for </span>rcon <span class="kw">in </span><span class="number">0</span>..<span class="number">10 </span>{
        memshift32(<span class="kw-2">&amp;mut </span>rkeys, rk_off);
        rk_off += <span class="number">8</span>;

        sub_bytes(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        sub_bytes_nots(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);

        <span class="kw">if </span>rcon &lt; <span class="number">8 </span>{
            add_round_constant_bit(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)], rcon);
        } <span class="kw">else </span>{
            add_round_constant_bit(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)], rcon - <span class="number">8</span>);
            add_round_constant_bit(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)], rcon - <span class="number">7</span>);
            add_round_constant_bit(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)], rcon - <span class="number">5</span>);
            add_round_constant_bit(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)], rcon - <span class="number">4</span>);
        }

        xor_columns(<span class="kw-2">&amp;mut </span>rkeys, rk_off, <span class="number">8</span>, ror_distance(<span class="number">1</span>, <span class="number">3</span>));
    }

    <span class="comment">// Adjust to match fixslicing format
    </span><span class="attr">#[cfg(feature = <span class="string">"compact"</span>)]
    </span>{
        <span class="kw">for </span>i <span class="kw">in </span>(<span class="number">8</span>..<span class="number">88</span>).step_by(<span class="number">16</span>) {
            inv_shift_rows_1(<span class="kw-2">&amp;mut </span>rkeys[i..(i + <span class="number">8</span>)]);
        }
    }
    <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
    </span>{
        <span class="kw">for </span>i <span class="kw">in </span>(<span class="number">8</span>..<span class="number">72</span>).step_by(<span class="number">32</span>) {
            inv_shift_rows_1(<span class="kw-2">&amp;mut </span>rkeys[i..(i + <span class="number">8</span>)]);
            inv_shift_rows_2(<span class="kw-2">&amp;mut </span>rkeys[(i + <span class="number">8</span>)..(i + <span class="number">16</span>)]);
            inv_shift_rows_3(<span class="kw-2">&amp;mut </span>rkeys[(i + <span class="number">16</span>)..(i + <span class="number">24</span>)]);
        }
        inv_shift_rows_1(<span class="kw-2">&amp;mut </span>rkeys[<span class="number">72</span>..<span class="number">80</span>]);
    }

    <span class="comment">// Account for NOTs removed from sub_bytes
    </span><span class="kw">for </span>i <span class="kw">in </span><span class="number">1</span>..<span class="number">11 </span>{
        sub_bytes_nots(<span class="kw-2">&amp;mut </span>rkeys[(i * <span class="number">8</span>)..(i * <span class="number">8 </span>+ <span class="number">8</span>)]);
    }

    rkeys
}

<span class="doccomment">/// Fully bitsliced AES-192 key schedule to match the fully-fixsliced representation.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>aes192_key_schedule(key: <span class="kw-2">&amp;</span>GenericArray&lt;u8, U24&gt;) -&gt; FixsliceKeys192 {
    <span class="kw">let </span><span class="kw-2">mut </span>rkeys = [<span class="number">0u64</span>; <span class="number">104</span>];
    <span class="kw">let </span><span class="kw-2">mut </span>tmp = [<span class="number">0u64</span>; <span class="number">8</span>];

    bitslice(
        <span class="kw-2">&amp;mut </span>rkeys[..<span class="number">8</span>],
        <span class="kw-2">&amp;</span>key[..<span class="number">16</span>],
        <span class="kw-2">&amp;</span>key[..<span class="number">16</span>],
        <span class="kw-2">&amp;</span>key[..<span class="number">16</span>],
        <span class="kw-2">&amp;</span>key[..<span class="number">16</span>],
    );
    bitslice(<span class="kw-2">&amp;mut </span>tmp, <span class="kw-2">&amp;</span>key[<span class="number">8</span>..], <span class="kw-2">&amp;</span>key[<span class="number">8</span>..], <span class="kw-2">&amp;</span>key[<span class="number">8</span>..], <span class="kw-2">&amp;</span>key[<span class="number">8</span>..]);

    <span class="kw">let </span><span class="kw-2">mut </span>rcon = <span class="number">0</span>;
    <span class="kw">let </span><span class="kw-2">mut </span>rk_off = <span class="number">8</span>;

    <span class="kw">loop </span>{
        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">8 </span>{
            rkeys[rk_off + i] = (<span class="number">0x00ff00ff00ff00ff </span>&amp; (tmp[i] &gt;&gt; <span class="number">8</span>))
                | (<span class="number">0xff00ff00ff00ff00 </span>&amp; (rkeys[(rk_off - <span class="number">8</span>) + i] &lt;&lt; <span class="number">8</span>));
        }

        sub_bytes(<span class="kw-2">&amp;mut </span>tmp);
        sub_bytes_nots(<span class="kw-2">&amp;mut </span>tmp);

        add_round_constant_bit(<span class="kw-2">&amp;mut </span>tmp, rcon);
        rcon += <span class="number">1</span>;

        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">8 </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>ti = rkeys[rk_off + i];
            ti ^= <span class="number">0x0f000f000f000f00 </span>&amp; ror(tmp[i], ror_distance(<span class="number">1</span>, <span class="number">1</span>));
            ti ^= <span class="number">0xf000f000f000f000 </span>&amp; (ti &lt;&lt; <span class="number">4</span>);
            tmp[i] = ti;
        }
        rkeys[rk_off..(rk_off + <span class="number">8</span>)].copy_from_slice(<span class="kw-2">&amp;</span>tmp);
        rk_off += <span class="number">8</span>;

        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">8 </span>{
            <span class="kw">let </span>ui = tmp[i];
            <span class="kw">let </span><span class="kw-2">mut </span>ti = (<span class="number">0x00ff00ff00ff00ff </span>&amp; (rkeys[(rk_off - <span class="number">16</span>) + i] &gt;&gt; <span class="number">8</span>))
                | (<span class="number">0xff00ff00ff00ff00 </span>&amp; (ui &lt;&lt; <span class="number">8</span>));
            ti ^= <span class="number">0x000f000f000f000f </span>&amp; (ui &gt;&gt; <span class="number">12</span>);
            tmp[i] = ti
                ^ (<span class="number">0xfff0fff0fff0fff0 </span>&amp; (ti &lt;&lt; <span class="number">4</span>))
                ^ (<span class="number">0xff00ff00ff00ff00 </span>&amp; (ti &lt;&lt; <span class="number">8</span>))
                ^ (<span class="number">0xf000f000f000f000 </span>&amp; (ti &lt;&lt; <span class="number">12</span>));
        }
        rkeys[rk_off..(rk_off + <span class="number">8</span>)].copy_from_slice(<span class="kw-2">&amp;</span>tmp);
        rk_off += <span class="number">8</span>;

        sub_bytes(<span class="kw-2">&amp;mut </span>tmp);
        sub_bytes_nots(<span class="kw-2">&amp;mut </span>tmp);

        add_round_constant_bit(<span class="kw-2">&amp;mut </span>tmp, rcon);
        rcon += <span class="number">1</span>;

        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">8 </span>{
            <span class="kw">let </span><span class="kw-2">mut </span>ti = (<span class="number">0x00ff00ff00ff00ff </span>&amp; (rkeys[(rk_off - <span class="number">16</span>) + i] &gt;&gt; <span class="number">8</span>))
                | (<span class="number">0xff00ff00ff00ff00 </span>&amp; (rkeys[(rk_off - <span class="number">8</span>) + i] &lt;&lt; <span class="number">8</span>));
            ti ^= <span class="number">0x000f000f000f000f </span>&amp; ror(tmp[i], ror_distance(<span class="number">1</span>, <span class="number">3</span>));
            rkeys[rk_off + i] = ti
                ^ (<span class="number">0xfff0fff0fff0fff0 </span>&amp; (ti &lt;&lt; <span class="number">4</span>))
                ^ (<span class="number">0xff00ff00ff00ff00 </span>&amp; (ti &lt;&lt; <span class="number">8</span>))
                ^ (<span class="number">0xf000f000f000f000 </span>&amp; (ti &lt;&lt; <span class="number">12</span>));
        }
        rk_off += <span class="number">8</span>;

        <span class="kw">if </span>rcon &gt;= <span class="number">8 </span>{
            <span class="kw">break</span>;
        }

        <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">8 </span>{
            <span class="kw">let </span>ui = rkeys[(rk_off - <span class="number">8</span>) + i];
            <span class="kw">let </span><span class="kw-2">mut </span>ti = rkeys[(rk_off - <span class="number">16</span>) + i];
            ti ^= <span class="number">0x0f000f000f000f00 </span>&amp; (ui &gt;&gt; <span class="number">4</span>);
            ti ^= <span class="number">0xf000f000f000f000 </span>&amp; (ti &lt;&lt; <span class="number">4</span>);
            tmp[i] = ti;
        }
    }

    <span class="comment">// Adjust to match fixslicing format
    </span><span class="attr">#[cfg(feature = <span class="string">"compact"</span>)]
    </span>{
        <span class="kw">for </span>i <span class="kw">in </span>(<span class="number">8</span>..<span class="number">104</span>).step_by(<span class="number">16</span>) {
            inv_shift_rows_1(<span class="kw-2">&amp;mut </span>rkeys[i..(i + <span class="number">8</span>)]);
        }
    }
    <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
    </span>{
        <span class="kw">for </span>i <span class="kw">in </span>(<span class="number">0</span>..<span class="number">96</span>).step_by(<span class="number">32</span>) {
            inv_shift_rows_1(<span class="kw-2">&amp;mut </span>rkeys[(i + <span class="number">8</span>)..(i + <span class="number">16</span>)]);
            inv_shift_rows_2(<span class="kw-2">&amp;mut </span>rkeys[(i + <span class="number">16</span>)..(i + <span class="number">24</span>)]);
            inv_shift_rows_3(<span class="kw-2">&amp;mut </span>rkeys[(i + <span class="number">24</span>)..(i + <span class="number">32</span>)]);
        }
    }

    <span class="comment">// Account for NOTs removed from sub_bytes
    </span><span class="kw">for </span>i <span class="kw">in </span><span class="number">1</span>..<span class="number">13 </span>{
        sub_bytes_nots(<span class="kw-2">&amp;mut </span>rkeys[(i * <span class="number">8</span>)..(i * <span class="number">8 </span>+ <span class="number">8</span>)]);
    }

    rkeys
}

<span class="doccomment">/// Fully bitsliced AES-256 key schedule to match the fully-fixsliced representation.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>aes256_key_schedule(key: <span class="kw-2">&amp;</span>GenericArray&lt;u8, U32&gt;) -&gt; FixsliceKeys256 {
    <span class="kw">let </span><span class="kw-2">mut </span>rkeys = [<span class="number">0u64</span>; <span class="number">120</span>];

    bitslice(
        <span class="kw-2">&amp;mut </span>rkeys[..<span class="number">8</span>],
        <span class="kw-2">&amp;</span>key[..<span class="number">16</span>],
        <span class="kw-2">&amp;</span>key[..<span class="number">16</span>],
        <span class="kw-2">&amp;</span>key[..<span class="number">16</span>],
        <span class="kw-2">&amp;</span>key[..<span class="number">16</span>],
    );
    bitslice(
        <span class="kw-2">&amp;mut </span>rkeys[<span class="number">8</span>..<span class="number">16</span>],
        <span class="kw-2">&amp;</span>key[<span class="number">16</span>..],
        <span class="kw-2">&amp;</span>key[<span class="number">16</span>..],
        <span class="kw-2">&amp;</span>key[<span class="number">16</span>..],
        <span class="kw-2">&amp;</span>key[<span class="number">16</span>..],
    );

    <span class="kw">let </span><span class="kw-2">mut </span>rk_off = <span class="number">8</span>;

    <span class="kw">let </span><span class="kw-2">mut </span>rcon = <span class="number">0</span>;
    <span class="kw">loop </span>{
        memshift32(<span class="kw-2">&amp;mut </span>rkeys, rk_off);
        rk_off += <span class="number">8</span>;

        sub_bytes(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        sub_bytes_nots(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);

        add_round_constant_bit(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)], rcon);
        xor_columns(<span class="kw-2">&amp;mut </span>rkeys, rk_off, <span class="number">16</span>, ror_distance(<span class="number">1</span>, <span class="number">3</span>));
        rcon += <span class="number">1</span>;

        <span class="kw">if </span>rcon == <span class="number">7 </span>{
            <span class="kw">break</span>;
        }

        memshift32(<span class="kw-2">&amp;mut </span>rkeys, rk_off);
        rk_off += <span class="number">8</span>;

        sub_bytes(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        sub_bytes_nots(<span class="kw-2">&amp;mut </span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);

        xor_columns(<span class="kw-2">&amp;mut </span>rkeys, rk_off, <span class="number">16</span>, ror_distance(<span class="number">0</span>, <span class="number">3</span>));
    }

    <span class="comment">// Adjust to match fixslicing format
    </span><span class="attr">#[cfg(feature = <span class="string">"compact"</span>)]
    </span>{
        <span class="kw">for </span>i <span class="kw">in </span>(<span class="number">8</span>..<span class="number">120</span>).step_by(<span class="number">16</span>) {
            inv_shift_rows_1(<span class="kw-2">&amp;mut </span>rkeys[i..(i + <span class="number">8</span>)]);
        }
    }
    <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
    </span>{
        <span class="kw">for </span>i <span class="kw">in </span>(<span class="number">8</span>..<span class="number">104</span>).step_by(<span class="number">32</span>) {
            inv_shift_rows_1(<span class="kw-2">&amp;mut </span>rkeys[i..(i + <span class="number">8</span>)]);
            inv_shift_rows_2(<span class="kw-2">&amp;mut </span>rkeys[(i + <span class="number">8</span>)..(i + <span class="number">16</span>)]);
            inv_shift_rows_3(<span class="kw-2">&amp;mut </span>rkeys[(i + <span class="number">16</span>)..(i + <span class="number">24</span>)]);
        }
        inv_shift_rows_1(<span class="kw-2">&amp;mut </span>rkeys[<span class="number">104</span>..<span class="number">112</span>]);
    }

    <span class="comment">// Account for NOTs removed from sub_bytes
    </span><span class="kw">for </span>i <span class="kw">in </span><span class="number">1</span>..<span class="number">15 </span>{
        sub_bytes_nots(<span class="kw-2">&amp;mut </span>rkeys[(i * <span class="number">8</span>)..(i * <span class="number">8 </span>+ <span class="number">8</span>)]);
    }

    rkeys
}

<span class="doccomment">/// Fully-fixsliced AES-128 decryption (the InvShiftRows is completely omitted).
///
/// Decrypts four blocks in-place and in parallel.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>aes128_decrypt(rkeys: <span class="kw-2">&amp;</span>FixsliceKeys128, blocks: <span class="kw-2">&amp;mut </span>[Block]) {
    <span class="macro">debug_assert_eq!</span>(blocks.len(), FIXSLICE_BLOCKS);
    <span class="kw">let </span><span class="kw-2">mut </span>state = State::default();

    bitslice(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>blocks[<span class="number">0</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">1</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">2</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">3</span>]);

    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[<span class="number">80</span>..]);
    inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);

    <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
    </span>{
        inv_shift_rows_2(<span class="kw-2">&amp;mut </span>state);
    }

    <span class="kw">let </span><span class="kw-2">mut </span>rk_off = <span class="number">72</span>;
    <span class="kw">loop </span>{
        <span class="attr">#[cfg(feature = <span class="string">"compact"</span>)]
        </span>{
            inv_shift_rows_2(<span class="kw-2">&amp;mut </span>state);
        }

        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        inv_mix_columns_1(<span class="kw-2">&amp;mut </span>state);
        inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
        rk_off -= <span class="number">8</span>;

        <span class="kw">if </span>rk_off == <span class="number">0 </span>{
            <span class="kw">break</span>;
        }

        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        inv_mix_columns_0(<span class="kw-2">&amp;mut </span>state);
        inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
        rk_off -= <span class="number">8</span>;

        <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
        </span>{
            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            inv_mix_columns_3(<span class="kw-2">&amp;mut </span>state);
            inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
            rk_off -= <span class="number">8</span>;

            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            inv_mix_columns_2(<span class="kw-2">&amp;mut </span>state);
            inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
            rk_off -= <span class="number">8</span>;
        }
    }

    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[..<span class="number">8</span>]);

    inv_bitslice(<span class="kw-2">&amp;</span>state, blocks);
}

<span class="doccomment">/// Fully-fixsliced AES-128 encryption (the ShiftRows is completely omitted).
///
/// Encrypts four blocks in-place and in parallel.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>aes128_encrypt(rkeys: <span class="kw-2">&amp;</span>FixsliceKeys128, blocks: <span class="kw-2">&amp;mut </span>[Block]) {
    <span class="macro">debug_assert_eq!</span>(blocks.len(), FIXSLICE_BLOCKS);
    <span class="kw">let </span><span class="kw-2">mut </span>state = State::default();

    bitslice(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>blocks[<span class="number">0</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">1</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">2</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">3</span>]);

    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[..<span class="number">8</span>]);

    <span class="kw">let </span><span class="kw-2">mut </span>rk_off = <span class="number">8</span>;
    <span class="kw">loop </span>{
        sub_bytes(<span class="kw-2">&amp;mut </span>state);
        mix_columns_1(<span class="kw-2">&amp;mut </span>state);
        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        rk_off += <span class="number">8</span>;

        <span class="attr">#[cfg(feature = <span class="string">"compact"</span>)]
        </span>{
            shift_rows_2(<span class="kw-2">&amp;mut </span>state);
        }

        <span class="kw">if </span>rk_off == <span class="number">80 </span>{
            <span class="kw">break</span>;
        }

        <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
        </span>{
            sub_bytes(<span class="kw-2">&amp;mut </span>state);
            mix_columns_2(<span class="kw-2">&amp;mut </span>state);
            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            rk_off += <span class="number">8</span>;

            sub_bytes(<span class="kw-2">&amp;mut </span>state);
            mix_columns_3(<span class="kw-2">&amp;mut </span>state);
            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            rk_off += <span class="number">8</span>;
        }

        sub_bytes(<span class="kw-2">&amp;mut </span>state);
        mix_columns_0(<span class="kw-2">&amp;mut </span>state);
        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        rk_off += <span class="number">8</span>;
    }

    <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
    </span>{
        shift_rows_2(<span class="kw-2">&amp;mut </span>state);
    }

    sub_bytes(<span class="kw-2">&amp;mut </span>state);
    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[<span class="number">80</span>..]);

    inv_bitslice(<span class="kw-2">&amp;</span>state, blocks);
}

<span class="doccomment">/// Fully-fixsliced AES-192 decryption (the InvShiftRows is completely omitted).
///
/// Decrypts four blocks in-place and in parallel.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>aes192_decrypt(rkeys: <span class="kw-2">&amp;</span>FixsliceKeys192, blocks: <span class="kw-2">&amp;mut </span>[Block]) {
    <span class="macro">debug_assert_eq!</span>(blocks.len(), FIXSLICE_BLOCKS);
    <span class="kw">let </span><span class="kw-2">mut </span>state = State::default();

    bitslice(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>blocks[<span class="number">0</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">1</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">2</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">3</span>]);

    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[<span class="number">96</span>..]);
    inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);

    <span class="kw">let </span><span class="kw-2">mut </span>rk_off = <span class="number">88</span>;
    <span class="kw">loop </span>{
        <span class="attr">#[cfg(feature = <span class="string">"compact"</span>)]
        </span>{
            inv_shift_rows_2(<span class="kw-2">&amp;mut </span>state);
        }
        <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
        </span>{
            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            inv_mix_columns_3(<span class="kw-2">&amp;mut </span>state);
            inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
            rk_off -= <span class="number">8</span>;

            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            inv_mix_columns_2(<span class="kw-2">&amp;mut </span>state);
            inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
            rk_off -= <span class="number">8</span>;
        }

        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        inv_mix_columns_1(<span class="kw-2">&amp;mut </span>state);
        inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
        rk_off -= <span class="number">8</span>;

        <span class="kw">if </span>rk_off == <span class="number">0 </span>{
            <span class="kw">break</span>;
        }

        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        inv_mix_columns_0(<span class="kw-2">&amp;mut </span>state);
        inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
        rk_off -= <span class="number">8</span>;
    }

    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[..<span class="number">8</span>]);

    inv_bitslice(<span class="kw-2">&amp;</span>state, blocks);
}

<span class="doccomment">/// Fully-fixsliced AES-192 encryption (the ShiftRows is completely omitted).
///
/// Encrypts four blocks in-place and in parallel.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>aes192_encrypt(rkeys: <span class="kw-2">&amp;</span>FixsliceKeys192, blocks: <span class="kw-2">&amp;mut </span>[Block]) {
    <span class="macro">debug_assert_eq!</span>(blocks.len(), FIXSLICE_BLOCKS);
    <span class="kw">let </span><span class="kw-2">mut </span>state = State::default();

    bitslice(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>blocks[<span class="number">0</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">1</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">2</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">3</span>]);

    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[..<span class="number">8</span>]);

    <span class="kw">let </span><span class="kw-2">mut </span>rk_off = <span class="number">8</span>;
    <span class="kw">loop </span>{
        sub_bytes(<span class="kw-2">&amp;mut </span>state);
        mix_columns_1(<span class="kw-2">&amp;mut </span>state);
        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        rk_off += <span class="number">8</span>;

        <span class="attr">#[cfg(feature = <span class="string">"compact"</span>)]
        </span>{
            shift_rows_2(<span class="kw-2">&amp;mut </span>state);
        }
        <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
        </span>{
            sub_bytes(<span class="kw-2">&amp;mut </span>state);
            mix_columns_2(<span class="kw-2">&amp;mut </span>state);
            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            rk_off += <span class="number">8</span>;

            sub_bytes(<span class="kw-2">&amp;mut </span>state);
            mix_columns_3(<span class="kw-2">&amp;mut </span>state);
            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            rk_off += <span class="number">8</span>;
        }

        <span class="kw">if </span>rk_off == <span class="number">96 </span>{
            <span class="kw">break</span>;
        }

        sub_bytes(<span class="kw-2">&amp;mut </span>state);
        mix_columns_0(<span class="kw-2">&amp;mut </span>state);
        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        rk_off += <span class="number">8</span>;
    }

    sub_bytes(<span class="kw-2">&amp;mut </span>state);
    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[<span class="number">96</span>..]);

    inv_bitslice(<span class="kw-2">&amp;</span>state, blocks);
}

<span class="doccomment">/// Fully-fixsliced AES-256 decryption (the InvShiftRows is completely omitted).
///
/// Decrypts four blocks in-place and in parallel.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>aes256_decrypt(rkeys: <span class="kw-2">&amp;</span>FixsliceKeys256, blocks: <span class="kw-2">&amp;mut </span>[Block]) {
    <span class="macro">debug_assert_eq!</span>(blocks.len(), FIXSLICE_BLOCKS);
    <span class="kw">let </span><span class="kw-2">mut </span>state = State::default();

    bitslice(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>blocks[<span class="number">0</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">1</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">2</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">3</span>]);

    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[<span class="number">112</span>..]);
    inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);

    <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
    </span>{
        inv_shift_rows_2(<span class="kw-2">&amp;mut </span>state);
    }

    <span class="kw">let </span><span class="kw-2">mut </span>rk_off = <span class="number">104</span>;
    <span class="kw">loop </span>{
        <span class="attr">#[cfg(feature = <span class="string">"compact"</span>)]
        </span>{
            inv_shift_rows_2(<span class="kw-2">&amp;mut </span>state);
        }

        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        inv_mix_columns_1(<span class="kw-2">&amp;mut </span>state);
        inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
        rk_off -= <span class="number">8</span>;

        <span class="kw">if </span>rk_off == <span class="number">0 </span>{
            <span class="kw">break</span>;
        }

        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        inv_mix_columns_0(<span class="kw-2">&amp;mut </span>state);
        inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
        rk_off -= <span class="number">8</span>;

        <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
        </span>{
            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            inv_mix_columns_3(<span class="kw-2">&amp;mut </span>state);
            inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
            rk_off -= <span class="number">8</span>;

            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            inv_mix_columns_2(<span class="kw-2">&amp;mut </span>state);
            inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
            rk_off -= <span class="number">8</span>;
        }
    }

    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[..<span class="number">8</span>]);

    inv_bitslice(<span class="kw-2">&amp;</span>state, blocks);
}

<span class="doccomment">/// Fully-fixsliced AES-256 encryption (the ShiftRows is completely omitted).
///
/// Encrypts four blocks in-place and in parallel.
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>aes256_encrypt(rkeys: <span class="kw-2">&amp;</span>FixsliceKeys256, blocks: <span class="kw-2">&amp;mut </span>[Block]) {
    <span class="macro">debug_assert_eq!</span>(blocks.len(), FIXSLICE_BLOCKS);
    <span class="kw">let </span><span class="kw-2">mut </span>state = State::default();

    bitslice(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>blocks[<span class="number">0</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">1</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">2</span>], <span class="kw-2">&amp;</span>blocks[<span class="number">3</span>]);

    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[..<span class="number">8</span>]);

    <span class="kw">let </span><span class="kw-2">mut </span>rk_off = <span class="number">8</span>;
    <span class="kw">loop </span>{
        sub_bytes(<span class="kw-2">&amp;mut </span>state);
        mix_columns_1(<span class="kw-2">&amp;mut </span>state);
        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        rk_off += <span class="number">8</span>;

        <span class="attr">#[cfg(feature = <span class="string">"compact"</span>)]
        </span>{
            shift_rows_2(<span class="kw-2">&amp;mut </span>state);
        }

        <span class="kw">if </span>rk_off == <span class="number">112 </span>{
            <span class="kw">break</span>;
        }

        <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
        </span>{
            sub_bytes(<span class="kw-2">&amp;mut </span>state);
            mix_columns_2(<span class="kw-2">&amp;mut </span>state);
            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            rk_off += <span class="number">8</span>;

            sub_bytes(<span class="kw-2">&amp;mut </span>state);
            mix_columns_3(<span class="kw-2">&amp;mut </span>state);
            add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
            rk_off += <span class="number">8</span>;
        }

        sub_bytes(<span class="kw-2">&amp;mut </span>state);
        mix_columns_0(<span class="kw-2">&amp;mut </span>state);
        add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[rk_off..(rk_off + <span class="number">8</span>)]);
        rk_off += <span class="number">8</span>;
    }

    <span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
    </span>{
        shift_rows_2(<span class="kw-2">&amp;mut </span>state);
    }

    sub_bytes(<span class="kw-2">&amp;mut </span>state);
    add_round_key(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>rkeys[<span class="number">112</span>..]);

    inv_bitslice(<span class="kw-2">&amp;</span>state, blocks);
}

<span class="doccomment">/// Note that the 4 bitwise NOT (^= 0xffffffffffffffff) are accounted for here so that it is a true
/// inverse of 'sub_bytes'.
</span><span class="kw">fn </span>inv_sub_bytes(state: <span class="kw-2">&amp;mut </span>[u64]) {
    <span class="macro">debug_assert_eq!</span>(state.len(), <span class="number">8</span>);

    <span class="comment">// Scheduled using https://github.com/Ko-/aes-armcortexm/tree/public/scheduler
    // Inline "stack" comments reflect suggested stores and loads (ARM Cortex-M3 and M4)

    </span><span class="kw">let </span>u7 = state[<span class="number">0</span>];
    <span class="kw">let </span>u6 = state[<span class="number">1</span>];
    <span class="kw">let </span>u5 = state[<span class="number">2</span>];
    <span class="kw">let </span>u4 = state[<span class="number">3</span>];
    <span class="kw">let </span>u3 = state[<span class="number">4</span>];
    <span class="kw">let </span>u2 = state[<span class="number">5</span>];
    <span class="kw">let </span>u1 = state[<span class="number">6</span>];
    <span class="kw">let </span>u0 = state[<span class="number">7</span>];

    <span class="kw">let </span>t23 = u0 ^ u3;
    <span class="kw">let </span>t8 = u1 ^ t23;
    <span class="kw">let </span>m2 = t23 &amp; t8;
    <span class="kw">let </span>t4 = u4 ^ t8;
    <span class="kw">let </span>t22 = u1 ^ u3;
    <span class="kw">let </span>t2 = u0 ^ u1;
    <span class="kw">let </span>t1 = u3 ^ u4;
    <span class="comment">// t23 -&gt; stack
    </span><span class="kw">let </span>t9 = u7 ^ t1;
    <span class="comment">// t8 -&gt; stack
    </span><span class="kw">let </span>m7 = t22 &amp; t9;
    <span class="comment">// t9 -&gt; stack
    </span><span class="kw">let </span>t24 = u4 ^ u7;
    <span class="comment">// m7 -&gt; stack
    </span><span class="kw">let </span>t10 = t2 ^ t24;
    <span class="comment">// u4 -&gt; stack
    </span><span class="kw">let </span>m14 = t2 &amp; t10;
    <span class="kw">let </span>r5 = u6 ^ u7;
    <span class="comment">// m2 -&gt; stack
    </span><span class="kw">let </span>t3 = t1 ^ r5;
    <span class="comment">// t2 -&gt; stack
    </span><span class="kw">let </span>t13 = t2 ^ r5;
    <span class="kw">let </span>t19 = t22 ^ r5;
    <span class="comment">// t3 -&gt; stack
    </span><span class="kw">let </span>t17 = u2 ^ t19;
    <span class="comment">// t4 -&gt; stack
    </span><span class="kw">let </span>t25 = u2 ^ t1;
    <span class="kw">let </span>r13 = u1 ^ u6;
    <span class="comment">// t25 -&gt; stack
    </span><span class="kw">let </span>t20 = t24 ^ r13;
    <span class="comment">// t17 -&gt; stack
    </span><span class="kw">let </span>m9 = t20 &amp; t17;
    <span class="comment">// t20 -&gt; stack
    </span><span class="kw">let </span>r17 = u2 ^ u5;
    <span class="comment">// t22 -&gt; stack
    </span><span class="kw">let </span>t6 = t22 ^ r17;
    <span class="comment">// t13 -&gt; stack
    </span><span class="kw">let </span>m1 = t13 &amp; t6;
    <span class="kw">let </span>y5 = u0 ^ r17;
    <span class="kw">let </span>m4 = t19 &amp; y5;
    <span class="kw">let </span>m5 = m4 ^ m1;
    <span class="kw">let </span>m17 = m5 ^ t24;
    <span class="kw">let </span>r18 = u5 ^ u6;
    <span class="kw">let </span>t27 = t1 ^ r18;
    <span class="kw">let </span>t15 = t10 ^ t27;
    <span class="comment">// t6 -&gt; stack
    </span><span class="kw">let </span>m11 = t1 &amp; t15;
    <span class="kw">let </span>m15 = m14 ^ m11;
    <span class="kw">let </span>m21 = m17 ^ m15;
    <span class="comment">// t1 -&gt; stack
    // t4 &lt;- stack
    </span><span class="kw">let </span>m12 = t4 &amp; t27;
    <span class="kw">let </span>m13 = m12 ^ m11;
    <span class="kw">let </span>t14 = t10 ^ r18;
    <span class="kw">let </span>m3 = t14 ^ m1;
    <span class="comment">// m2 &lt;- stack
    </span><span class="kw">let </span>m16 = m3 ^ m2;
    <span class="kw">let </span>m20 = m16 ^ m13;
    <span class="comment">// u4 &lt;- stack
    </span><span class="kw">let </span>r19 = u2 ^ u4;
    <span class="kw">let </span>t16 = r13 ^ r19;
    <span class="comment">// t3 &lt;- stack
    </span><span class="kw">let </span>t26 = t3 ^ t16;
    <span class="kw">let </span>m6 = t3 &amp; t16;
    <span class="kw">let </span>m8 = t26 ^ m6;
    <span class="comment">// t10 -&gt; stack
    // m7 &lt;- stack
    </span><span class="kw">let </span>m18 = m8 ^ m7;
    <span class="kw">let </span>m22 = m18 ^ m13;
    <span class="kw">let </span>m25 = m22 &amp; m20;
    <span class="kw">let </span>m26 = m21 ^ m25;
    <span class="kw">let </span>m10 = m9 ^ m6;
    <span class="kw">let </span>m19 = m10 ^ m15;
    <span class="comment">// t25 &lt;- stack
    </span><span class="kw">let </span>m23 = m19 ^ t25;
    <span class="kw">let </span>m28 = m23 ^ m25;
    <span class="kw">let </span>m24 = m22 ^ m23;
    <span class="kw">let </span>m30 = m26 &amp; m24;
    <span class="kw">let </span>m39 = m23 ^ m30;
    <span class="kw">let </span>m48 = m39 &amp; y5;
    <span class="kw">let </span>m57 = m39 &amp; t19;
    <span class="comment">// m48 -&gt; stack
    </span><span class="kw">let </span>m36 = m24 ^ m25;
    <span class="kw">let </span>m31 = m20 &amp; m23;
    <span class="kw">let </span>m27 = m20 ^ m21;
    <span class="kw">let </span>m32 = m27 &amp; m31;
    <span class="kw">let </span>m29 = m28 &amp; m27;
    <span class="kw">let </span>m37 = m21 ^ m29;
    <span class="comment">// m39 -&gt; stack
    </span><span class="kw">let </span>m42 = m37 ^ m39;
    <span class="kw">let </span>m52 = m42 &amp; t15;
    <span class="comment">// t27 -&gt; stack
    // t1 &lt;- stack
    </span><span class="kw">let </span>m61 = m42 &amp; t1;
    <span class="kw">let </span>p0 = m52 ^ m61;
    <span class="kw">let </span>p16 = m57 ^ m61;
    <span class="comment">// m57 -&gt; stack
    // t20 &lt;- stack
    </span><span class="kw">let </span>m60 = m37 &amp; t20;
    <span class="comment">// p16 -&gt; stack
    // t17 &lt;- stack
    </span><span class="kw">let </span>m51 = m37 &amp; t17;
    <span class="kw">let </span>m33 = m27 ^ m25;
    <span class="kw">let </span>m38 = m32 ^ m33;
    <span class="kw">let </span>m43 = m37 ^ m38;
    <span class="kw">let </span>m49 = m43 &amp; t16;
    <span class="kw">let </span>p6 = m49 ^ m60;
    <span class="kw">let </span>p13 = m49 ^ m51;
    <span class="kw">let </span>m58 = m43 &amp; t3;
    <span class="comment">// t9 &lt;- stack
    </span><span class="kw">let </span>m50 = m38 &amp; t9;
    <span class="comment">// t22 &lt;- stack
    </span><span class="kw">let </span>m59 = m38 &amp; t22;
    <span class="comment">// p6 -&gt; stack
    </span><span class="kw">let </span>p1 = m58 ^ m59;
    <span class="kw">let </span>p7 = p0 ^ p1;
    <span class="kw">let </span>m34 = m21 &amp; m22;
    <span class="kw">let </span>m35 = m24 &amp; m34;
    <span class="kw">let </span>m40 = m35 ^ m36;
    <span class="kw">let </span>m41 = m38 ^ m40;
    <span class="kw">let </span>m45 = m42 ^ m41;
    <span class="comment">// t27 &lt;- stack
    </span><span class="kw">let </span>m53 = m45 &amp; t27;
    <span class="kw">let </span>p8 = m50 ^ m53;
    <span class="kw">let </span>p23 = p7 ^ p8;
    <span class="comment">// t4 &lt;- stack
    </span><span class="kw">let </span>m62 = m45 &amp; t4;
    <span class="kw">let </span>p14 = m49 ^ m62;
    <span class="kw">let </span>s6 = p14 ^ p23;
    <span class="comment">// t10 &lt;- stack
    </span><span class="kw">let </span>m54 = m41 &amp; t10;
    <span class="kw">let </span>p2 = m54 ^ m62;
    <span class="kw">let </span>p22 = p2 ^ p7;
    <span class="kw">let </span>s0 = p13 ^ p22;
    <span class="kw">let </span>p17 = m58 ^ p2;
    <span class="kw">let </span>p15 = m54 ^ m59;
    <span class="comment">// t2 &lt;- stack
    </span><span class="kw">let </span>m63 = m41 &amp; t2;
    <span class="comment">// m39 &lt;- stack
    </span><span class="kw">let </span>m44 = m39 ^ m40;
    <span class="comment">// p17 -&gt; stack
    // t6 &lt;- stack
    </span><span class="kw">let </span>m46 = m44 &amp; t6;
    <span class="kw">let </span>p5 = m46 ^ m51;
    <span class="comment">// p23 -&gt; stack
    </span><span class="kw">let </span>p18 = m63 ^ p5;
    <span class="kw">let </span>p24 = p5 ^ p7;
    <span class="comment">// m48 &lt;- stack
    </span><span class="kw">let </span>p12 = m46 ^ m48;
    <span class="kw">let </span>s3 = p12 ^ p22;
    <span class="comment">// t13 &lt;- stack
    </span><span class="kw">let </span>m55 = m44 &amp; t13;
    <span class="kw">let </span>p9 = m55 ^ m63;
    <span class="comment">// p16 &lt;- stack
    </span><span class="kw">let </span>s7 = p9 ^ p16;
    <span class="comment">// t8 &lt;- stack
    </span><span class="kw">let </span>m47 = m40 &amp; t8;
    <span class="kw">let </span>p3 = m47 ^ m50;
    <span class="kw">let </span>p19 = p2 ^ p3;
    <span class="kw">let </span>s5 = p19 ^ p24;
    <span class="kw">let </span>p11 = p0 ^ p3;
    <span class="kw">let </span>p26 = p9 ^ p11;
    <span class="comment">// t23 &lt;- stack
    </span><span class="kw">let </span>m56 = m40 &amp; t23;
    <span class="kw">let </span>p4 = m48 ^ m56;
    <span class="comment">// p6 &lt;- stack
    </span><span class="kw">let </span>p20 = p4 ^ p6;
    <span class="kw">let </span>p29 = p15 ^ p20;
    <span class="kw">let </span>s1 = p26 ^ p29;
    <span class="comment">// m57 &lt;- stack
    </span><span class="kw">let </span>p10 = m57 ^ p4;
    <span class="kw">let </span>p27 = p10 ^ p18;
    <span class="comment">// p23 &lt;- stack
    </span><span class="kw">let </span>s4 = p23 ^ p27;
    <span class="kw">let </span>p25 = p6 ^ p10;
    <span class="kw">let </span>p28 = p11 ^ p25;
    <span class="comment">// p17 &lt;- stack
    </span><span class="kw">let </span>s2 = p17 ^ p28;

    state[<span class="number">0</span>] = s7;
    state[<span class="number">1</span>] = s6;
    state[<span class="number">2</span>] = s5;
    state[<span class="number">3</span>] = s4;
    state[<span class="number">4</span>] = s3;
    state[<span class="number">5</span>] = s2;
    state[<span class="number">6</span>] = s1;
    state[<span class="number">7</span>] = s0;
}

<span class="doccomment">/// Bitsliced implementation of the AES Sbox based on Boyar, Peralta and Calik.
///
/// See: &lt;http://www.cs.yale.edu/homes/peralta/CircuitStuff/SLP_AES_113.txt&gt;
///
/// Note that the 4 bitwise NOT (^= 0xffffffffffffffff) are moved to the key schedule.
</span><span class="kw">fn </span>sub_bytes(state: <span class="kw-2">&amp;mut </span>[u64]) {
    <span class="macro">debug_assert_eq!</span>(state.len(), <span class="number">8</span>);

    <span class="comment">// Scheduled using https://github.com/Ko-/aes-armcortexm/tree/public/scheduler
    // Inline "stack" comments reflect suggested stores and loads (ARM Cortex-M3 and M4)

    </span><span class="kw">let </span>u7 = state[<span class="number">0</span>];
    <span class="kw">let </span>u6 = state[<span class="number">1</span>];
    <span class="kw">let </span>u5 = state[<span class="number">2</span>];
    <span class="kw">let </span>u4 = state[<span class="number">3</span>];
    <span class="kw">let </span>u3 = state[<span class="number">4</span>];
    <span class="kw">let </span>u2 = state[<span class="number">5</span>];
    <span class="kw">let </span>u1 = state[<span class="number">6</span>];
    <span class="kw">let </span>u0 = state[<span class="number">7</span>];

    <span class="kw">let </span>y14 = u3 ^ u5;
    <span class="kw">let </span>y13 = u0 ^ u6;
    <span class="kw">let </span>y12 = y13 ^ y14;
    <span class="kw">let </span>t1 = u4 ^ y12;
    <span class="kw">let </span>y15 = t1 ^ u5;
    <span class="kw">let </span>t2 = y12 &amp; y15;
    <span class="kw">let </span>y6 = y15 ^ u7;
    <span class="kw">let </span>y20 = t1 ^ u1;
    <span class="comment">// y12 -&gt; stack
    </span><span class="kw">let </span>y9 = u0 ^ u3;
    <span class="comment">// y20 -&gt; stack
    </span><span class="kw">let </span>y11 = y20 ^ y9;
    <span class="comment">// y9 -&gt; stack
    </span><span class="kw">let </span>t12 = y9 &amp; y11;
    <span class="comment">// y6 -&gt; stack
    </span><span class="kw">let </span>y7 = u7 ^ y11;
    <span class="kw">let </span>y8 = u0 ^ u5;
    <span class="kw">let </span>t0 = u1 ^ u2;
    <span class="kw">let </span>y10 = y15 ^ t0;
    <span class="comment">// y15 -&gt; stack
    </span><span class="kw">let </span>y17 = y10 ^ y11;
    <span class="comment">// y14 -&gt; stack
    </span><span class="kw">let </span>t13 = y14 &amp; y17;
    <span class="kw">let </span>t14 = t13 ^ t12;
    <span class="comment">// y17 -&gt; stack
    </span><span class="kw">let </span>y19 = y10 ^ y8;
    <span class="comment">// y10 -&gt; stack
    </span><span class="kw">let </span>t15 = y8 &amp; y10;
    <span class="kw">let </span>t16 = t15 ^ t12;
    <span class="kw">let </span>y16 = t0 ^ y11;
    <span class="comment">// y11 -&gt; stack
    </span><span class="kw">let </span>y21 = y13 ^ y16;
    <span class="comment">// y13 -&gt; stack
    </span><span class="kw">let </span>t7 = y13 &amp; y16;
    <span class="comment">// y16 -&gt; stack
    </span><span class="kw">let </span>y18 = u0 ^ y16;
    <span class="kw">let </span>y1 = t0 ^ u7;
    <span class="kw">let </span>y4 = y1 ^ u3;
    <span class="comment">// u7 -&gt; stack
    </span><span class="kw">let </span>t5 = y4 &amp; u7;
    <span class="kw">let </span>t6 = t5 ^ t2;
    <span class="kw">let </span>t18 = t6 ^ t16;
    <span class="kw">let </span>t22 = t18 ^ y19;
    <span class="kw">let </span>y2 = y1 ^ u0;
    <span class="kw">let </span>t10 = y2 &amp; y7;
    <span class="kw">let </span>t11 = t10 ^ t7;
    <span class="kw">let </span>t20 = t11 ^ t16;
    <span class="kw">let </span>t24 = t20 ^ y18;
    <span class="kw">let </span>y5 = y1 ^ u6;
    <span class="kw">let </span>t8 = y5 &amp; y1;
    <span class="kw">let </span>t9 = t8 ^ t7;
    <span class="kw">let </span>t19 = t9 ^ t14;
    <span class="kw">let </span>t23 = t19 ^ y21;
    <span class="kw">let </span>y3 = y5 ^ y8;
    <span class="comment">// y6 &lt;- stack
    </span><span class="kw">let </span>t3 = y3 &amp; y6;
    <span class="kw">let </span>t4 = t3 ^ t2;
    <span class="comment">// y20 &lt;- stack
    </span><span class="kw">let </span>t17 = t4 ^ y20;
    <span class="kw">let </span>t21 = t17 ^ t14;
    <span class="kw">let </span>t26 = t21 &amp; t23;
    <span class="kw">let </span>t27 = t24 ^ t26;
    <span class="kw">let </span>t31 = t22 ^ t26;
    <span class="kw">let </span>t25 = t21 ^ t22;
    <span class="comment">// y4 -&gt; stack
    </span><span class="kw">let </span>t28 = t25 &amp; t27;
    <span class="kw">let </span>t29 = t28 ^ t22;
    <span class="kw">let </span>z14 = t29 &amp; y2;
    <span class="kw">let </span>z5 = t29 &amp; y7;
    <span class="kw">let </span>t30 = t23 ^ t24;
    <span class="kw">let </span>t32 = t31 &amp; t30;
    <span class="kw">let </span>t33 = t32 ^ t24;
    <span class="kw">let </span>t35 = t27 ^ t33;
    <span class="kw">let </span>t36 = t24 &amp; t35;
    <span class="kw">let </span>t38 = t27 ^ t36;
    <span class="kw">let </span>t39 = t29 &amp; t38;
    <span class="kw">let </span>t40 = t25 ^ t39;
    <span class="kw">let </span>t43 = t29 ^ t40;
    <span class="comment">// y16 &lt;- stack
    </span><span class="kw">let </span>z3 = t43 &amp; y16;
    <span class="kw">let </span>tc12 = z3 ^ z5;
    <span class="comment">// tc12 -&gt; stack
    // y13 &lt;- stack
    </span><span class="kw">let </span>z12 = t43 &amp; y13;
    <span class="kw">let </span>z13 = t40 &amp; y5;
    <span class="kw">let </span>z4 = t40 &amp; y1;
    <span class="kw">let </span>tc6 = z3 ^ z4;
    <span class="kw">let </span>t34 = t23 ^ t33;
    <span class="kw">let </span>t37 = t36 ^ t34;
    <span class="kw">let </span>t41 = t40 ^ t37;
    <span class="comment">// y10 &lt;- stack
    </span><span class="kw">let </span>z8 = t41 &amp; y10;
    <span class="kw">let </span>z17 = t41 &amp; y8;
    <span class="kw">let </span>t44 = t33 ^ t37;
    <span class="comment">// y15 &lt;- stack
    </span><span class="kw">let </span>z0 = t44 &amp; y15;
    <span class="comment">// z17 -&gt; stack
    // y12 &lt;- stack
    </span><span class="kw">let </span>z9 = t44 &amp; y12;
    <span class="kw">let </span>z10 = t37 &amp; y3;
    <span class="kw">let </span>z1 = t37 &amp; y6;
    <span class="kw">let </span>tc5 = z1 ^ z0;
    <span class="kw">let </span>tc11 = tc6 ^ tc5;
    <span class="comment">// y4 &lt;- stack
    </span><span class="kw">let </span>z11 = t33 &amp; y4;
    <span class="kw">let </span>t42 = t29 ^ t33;
    <span class="kw">let </span>t45 = t42 ^ t41;
    <span class="comment">// y17 &lt;- stack
    </span><span class="kw">let </span>z7 = t45 &amp; y17;
    <span class="kw">let </span>tc8 = z7 ^ tc6;
    <span class="comment">// y14 &lt;- stack
    </span><span class="kw">let </span>z16 = t45 &amp; y14;
    <span class="comment">// y11 &lt;- stack
    </span><span class="kw">let </span>z6 = t42 &amp; y11;
    <span class="kw">let </span>tc16 = z6 ^ tc8;
    <span class="comment">// z14 -&gt; stack
    // y9 &lt;- stack
    </span><span class="kw">let </span>z15 = t42 &amp; y9;
    <span class="kw">let </span>tc20 = z15 ^ tc16;
    <span class="kw">let </span>tc1 = z15 ^ z16;
    <span class="kw">let </span>tc2 = z10 ^ tc1;
    <span class="kw">let </span>tc21 = tc2 ^ z11;
    <span class="kw">let </span>tc3 = z9 ^ tc2;
    <span class="kw">let </span>s0 = tc3 ^ tc16;
    <span class="kw">let </span>s3 = tc3 ^ tc11;
    <span class="kw">let </span>s1 = s3 ^ tc16;
    <span class="kw">let </span>tc13 = z13 ^ tc1;
    <span class="comment">// u7 &lt;- stack
    </span><span class="kw">let </span>z2 = t33 &amp; u7;
    <span class="kw">let </span>tc4 = z0 ^ z2;
    <span class="kw">let </span>tc7 = z12 ^ tc4;
    <span class="kw">let </span>tc9 = z8 ^ tc7;
    <span class="kw">let </span>tc10 = tc8 ^ tc9;
    <span class="comment">// z14 &lt;- stack
    </span><span class="kw">let </span>tc17 = z14 ^ tc10;
    <span class="kw">let </span>s5 = tc21 ^ tc17;
    <span class="kw">let </span>tc26 = tc17 ^ tc20;
    <span class="comment">// z17 &lt;- stack
    </span><span class="kw">let </span>s2 = tc26 ^ z17;
    <span class="comment">// tc12 &lt;- stack
    </span><span class="kw">let </span>tc14 = tc4 ^ tc12;
    <span class="kw">let </span>tc18 = tc13 ^ tc14;
    <span class="kw">let </span>s6 = tc10 ^ tc18;
    <span class="kw">let </span>s7 = z12 ^ tc18;
    <span class="kw">let </span>s4 = tc14 ^ s3;

    state[<span class="number">0</span>] = s7;
    state[<span class="number">1</span>] = s6;
    state[<span class="number">2</span>] = s5;
    state[<span class="number">3</span>] = s4;
    state[<span class="number">4</span>] = s3;
    state[<span class="number">5</span>] = s2;
    state[<span class="number">6</span>] = s1;
    state[<span class="number">7</span>] = s0;
}

<span class="doccomment">/// NOT operations that are omitted in S-box
</span><span class="attr">#[inline]
</span><span class="kw">fn </span>sub_bytes_nots(state: <span class="kw-2">&amp;mut </span>[u64]) {
    <span class="macro">debug_assert_eq!</span>(state.len(), <span class="number">8</span>);
    state[<span class="number">0</span>] ^= <span class="number">0xffffffffffffffff</span>;
    state[<span class="number">1</span>] ^= <span class="number">0xffffffffffffffff</span>;
    state[<span class="number">5</span>] ^= <span class="number">0xffffffffffffffff</span>;
    state[<span class="number">6</span>] ^= <span class="number">0xffffffffffffffff</span>;
}

<span class="doccomment">/// Computation of the MixColumns transformation in the fixsliced representation, with different
/// rotations used according to the round number mod 4.
///
/// Based on Käsper-Schwabe, similar to https://github.com/Ko-/aes-armcortexm.
</span><span class="macro">macro_rules!</span> define_mix_columns {
    (
        <span class="macro-nonterminal">$name</span>:ident,
        <span class="macro-nonterminal">$name_inv</span>:ident,
        <span class="macro-nonterminal">$first_rotate</span>:path,
        <span class="macro-nonterminal">$second_rotate</span>:path
    ) =&gt; {
        <span class="attr">#[rustfmt::skip]
        </span><span class="kw">fn </span><span class="macro-nonterminal">$name</span>(state: <span class="kw-2">&amp;mut </span>State) {
            <span class="kw">let </span>(a0, a1, a2, a3, a4, a5, a6, a7) = (
                state[<span class="number">0</span>], state[<span class="number">1</span>], state[<span class="number">2</span>], state[<span class="number">3</span>], state[<span class="number">4</span>], state[<span class="number">5</span>], state[<span class="number">6</span>], state[<span class="number">7</span>]
            );
            <span class="kw">let </span>(b0, b1, b2, b3, b4, b5, b6, b7) = (
                <span class="macro-nonterminal">$first_rotate</span>(a0),
                <span class="macro-nonterminal">$first_rotate</span>(a1),
                <span class="macro-nonterminal">$first_rotate</span>(a2),
                <span class="macro-nonterminal">$first_rotate</span>(a3),
                <span class="macro-nonterminal">$first_rotate</span>(a4),
                <span class="macro-nonterminal">$first_rotate</span>(a5),
                <span class="macro-nonterminal">$first_rotate</span>(a6),
                <span class="macro-nonterminal">$first_rotate</span>(a7),
            );
            <span class="kw">let </span>(c0, c1, c2, c3, c4, c5, c6, c7) = (
                a0 ^ b0,
                a1 ^ b1,
                a2 ^ b2,
                a3 ^ b3,
                a4 ^ b4,
                a5 ^ b5,
                a6 ^ b6,
                a7 ^ b7,
            );
            state[<span class="number">0</span>] = b0      ^ c7 ^ <span class="macro-nonterminal">$second_rotate</span>(c0);
            state[<span class="number">1</span>] = b1 ^ c0 ^ c7 ^ <span class="macro-nonterminal">$second_rotate</span>(c1);
            state[<span class="number">2</span>] = b2 ^ c1      ^ <span class="macro-nonterminal">$second_rotate</span>(c2);
            state[<span class="number">3</span>] = b3 ^ c2 ^ c7 ^ <span class="macro-nonterminal">$second_rotate</span>(c3);
            state[<span class="number">4</span>] = b4 ^ c3 ^ c7 ^ <span class="macro-nonterminal">$second_rotate</span>(c4);
            state[<span class="number">5</span>] = b5 ^ c4      ^ <span class="macro-nonterminal">$second_rotate</span>(c5);
            state[<span class="number">6</span>] = b6 ^ c5      ^ <span class="macro-nonterminal">$second_rotate</span>(c6);
            state[<span class="number">7</span>] = b7 ^ c6      ^ <span class="macro-nonterminal">$second_rotate</span>(c7);
        }

        <span class="attr">#[rustfmt::skip]
        </span><span class="kw">fn </span><span class="macro-nonterminal">$name_inv</span>(state: <span class="kw-2">&amp;mut </span>State) {
            <span class="kw">let </span>(a0, a1, a2, a3, a4, a5, a6, a7) = (
                state[<span class="number">0</span>], state[<span class="number">1</span>], state[<span class="number">2</span>], state[<span class="number">3</span>], state[<span class="number">4</span>], state[<span class="number">5</span>], state[<span class="number">6</span>], state[<span class="number">7</span>]
            );
            <span class="kw">let </span>(b0, b1, b2, b3, b4, b5, b6, b7) = (
                <span class="macro-nonterminal">$first_rotate</span>(a0),
                <span class="macro-nonterminal">$first_rotate</span>(a1),
                <span class="macro-nonterminal">$first_rotate</span>(a2),
                <span class="macro-nonterminal">$first_rotate</span>(a3),
                <span class="macro-nonterminal">$first_rotate</span>(a4),
                <span class="macro-nonterminal">$first_rotate</span>(a5),
                <span class="macro-nonterminal">$first_rotate</span>(a6),
                <span class="macro-nonterminal">$first_rotate</span>(a7),
            );
            <span class="kw">let </span>(c0, c1, c2, c3, c4, c5, c6, c7) = (
                a0 ^ b0,
                a1 ^ b1,
                a2 ^ b2,
                a3 ^ b3,
                a4 ^ b4,
                a5 ^ b5,
                a6 ^ b6,
                a7 ^ b7,
            );
            <span class="kw">let </span>(d0, d1, d2, d3, d4, d5, d6, d7) = (
                a0      ^ c7,
                a1 ^ c0 ^ c7,
                a2 ^ c1,
                a3 ^ c2 ^ c7,
                a4 ^ c3 ^ c7,
                a5 ^ c4,
                a6 ^ c5,
                a7 ^ c6,
            );
            <span class="kw">let </span>(e0, e1, e2, e3, e4, e5, e6, e7) = (
                c0      ^ d6,
                c1      ^ d6 ^ d7,
                c2 ^ d0      ^ d7,
                c3 ^ d1 ^ d6,
                c4 ^ d2 ^ d6 ^ d7,
                c5 ^ d3      ^ d7,
                c6 ^ d4,
                c7 ^ d5,
            );
            state[<span class="number">0</span>] = d0 ^ e0 ^ <span class="macro-nonterminal">$second_rotate</span>(e0);
            state[<span class="number">1</span>] = d1 ^ e1 ^ <span class="macro-nonterminal">$second_rotate</span>(e1);
            state[<span class="number">2</span>] = d2 ^ e2 ^ <span class="macro-nonterminal">$second_rotate</span>(e2);
            state[<span class="number">3</span>] = d3 ^ e3 ^ <span class="macro-nonterminal">$second_rotate</span>(e3);
            state[<span class="number">4</span>] = d4 ^ e4 ^ <span class="macro-nonterminal">$second_rotate</span>(e4);
            state[<span class="number">5</span>] = d5 ^ e5 ^ <span class="macro-nonterminal">$second_rotate</span>(e5);
            state[<span class="number">6</span>] = d6 ^ e6 ^ <span class="macro-nonterminal">$second_rotate</span>(e6);
            state[<span class="number">7</span>] = d7 ^ e7 ^ <span class="macro-nonterminal">$second_rotate</span>(e7);
        }
    }
}

<span class="macro">define_mix_columns!</span>(
    mix_columns_0,
    inv_mix_columns_0,
    rotate_rows_1,
    rotate_rows_2
);

<span class="macro">define_mix_columns!</span>(
    mix_columns_1,
    inv_mix_columns_1,
    rotate_rows_and_columns_1_1,
    rotate_rows_and_columns_2_2
);

<span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
</span><span class="macro">define_mix_columns!</span>(
    mix_columns_2,
    inv_mix_columns_2,
    rotate_rows_and_columns_1_2,
    rotate_rows_2
);

<span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
</span><span class="macro">define_mix_columns!</span>(
    mix_columns_3,
    inv_mix_columns_3,
    rotate_rows_and_columns_1_3,
    rotate_rows_and_columns_2_2
);

<span class="attr">#[inline]
</span><span class="kw">fn </span>delta_swap_1(a: <span class="kw-2">&amp;mut </span>u64, shift: u32, mask: u64) {
    <span class="kw">let </span>t = (<span class="kw-2">*</span>a ^ ((<span class="kw-2">*</span>a) &gt;&gt; shift)) &amp; mask;
    <span class="kw-2">*</span>a ^= t ^ (t &lt;&lt; shift);
}

<span class="attr">#[inline]
</span><span class="kw">fn </span>delta_swap_2(a: <span class="kw-2">&amp;mut </span>u64, b: <span class="kw-2">&amp;mut </span>u64, shift: u32, mask: u64) {
    <span class="kw">let </span>t = (<span class="kw-2">*</span>a ^ ((<span class="kw-2">*</span>b) &gt;&gt; shift)) &amp; mask;
    <span class="kw-2">*</span>a ^= t;
    <span class="kw-2">*</span>b ^= t &lt;&lt; shift;
}

<span class="doccomment">/// Applies ShiftRows once on an AES state (or key).
</span><span class="attr">#[cfg(any(not(feature = <span class="string">"compact"</span>), feature = <span class="string">"hazmat"</span>))]
#[inline]
</span><span class="kw">fn </span>shift_rows_1(state: <span class="kw-2">&amp;mut </span>[u64]) {
    <span class="macro">debug_assert_eq!</span>(state.len(), <span class="number">8</span>);
    <span class="kw">for </span>x <span class="kw">in </span>state.iter_mut() {
        delta_swap_1(x, <span class="number">8</span>, <span class="number">0x00f000ff000f0000</span>);
        delta_swap_1(x, <span class="number">4</span>, <span class="number">0x0f0f00000f0f0000</span>);
    }
}

<span class="doccomment">/// Applies ShiftRows twice on an AES state (or key).
</span><span class="attr">#[inline]
</span><span class="kw">fn </span>shift_rows_2(state: <span class="kw-2">&amp;mut </span>[u64]) {
    <span class="macro">debug_assert_eq!</span>(state.len(), <span class="number">8</span>);
    <span class="kw">for </span>x <span class="kw">in </span>state.iter_mut() {
        delta_swap_1(x, <span class="number">8</span>, <span class="number">0x00ff000000ff0000</span>);
    }
}

<span class="doccomment">/// Applies ShiftRows three times on an AES state (or key).
</span><span class="attr">#[inline]
</span><span class="kw">fn </span>shift_rows_3(state: <span class="kw-2">&amp;mut </span>[u64]) {
    <span class="macro">debug_assert_eq!</span>(state.len(), <span class="number">8</span>);
    <span class="kw">for </span>x <span class="kw">in </span>state.iter_mut() {
        delta_swap_1(x, <span class="number">8</span>, <span class="number">0x000f00ff00f00000</span>);
        delta_swap_1(x, <span class="number">4</span>, <span class="number">0x0f0f00000f0f0000</span>);
    }
}

<span class="attr">#[inline(always)]
</span><span class="kw">fn </span>inv_shift_rows_1(state: <span class="kw-2">&amp;mut </span>[u64]) {
    shift_rows_3(state);
}

<span class="attr">#[inline(always)]
</span><span class="kw">fn </span>inv_shift_rows_2(state: <span class="kw-2">&amp;mut </span>[u64]) {
    shift_rows_2(state);
}

<span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
#[inline(always)]
</span><span class="kw">fn </span>inv_shift_rows_3(state: <span class="kw-2">&amp;mut </span>[u64]) {
    shift_rows_1(state);
}

<span class="doccomment">/// XOR the columns after the S-box during the key schedule round function.
///
/// The `idx_xor` parameter refers to the index of the previous round key that is
/// involved in the XOR computation (should be 8 and 16 for AES-128 and AES-256,
/// respectively).
///
/// The `idx_ror` parameter refers to the rotation value, which varies between the
/// different key schedules.
</span><span class="kw">fn </span>xor_columns(rkeys: <span class="kw-2">&amp;mut </span>[u64], offset: usize, idx_xor: usize, idx_ror: u32) {
    <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">8 </span>{
        <span class="kw">let </span>off_i = offset + i;
        <span class="kw">let </span>rk = rkeys[off_i - idx_xor] ^ (<span class="number">0x000f000f000f000f </span>&amp; ror(rkeys[off_i], idx_ror));
        rkeys[off_i] = rk
            ^ (<span class="number">0xfff0fff0fff0fff0 </span>&amp; (rk &lt;&lt; <span class="number">4</span>))
            ^ (<span class="number">0xff00ff00ff00ff00 </span>&amp; (rk &lt;&lt; <span class="number">8</span>))
            ^ (<span class="number">0xf000f000f000f000 </span>&amp; (rk &lt;&lt; <span class="number">12</span>));
    }
}

<span class="doccomment">/// Bitslice four 128-bit input blocks input0, input1, input2, input3 into a 512-bit internal state.
</span><span class="kw">fn </span>bitslice(output: <span class="kw-2">&amp;mut </span>[u64], input0: <span class="kw-2">&amp;</span>[u8], input1: <span class="kw-2">&amp;</span>[u8], input2: <span class="kw-2">&amp;</span>[u8], input3: <span class="kw-2">&amp;</span>[u8]) {
    <span class="macro">debug_assert_eq!</span>(output.len(), <span class="number">8</span>);
    <span class="macro">debug_assert_eq!</span>(input0.len(), <span class="number">16</span>);
    <span class="macro">debug_assert_eq!</span>(input1.len(), <span class="number">16</span>);
    <span class="macro">debug_assert_eq!</span>(input2.len(), <span class="number">16</span>);
    <span class="macro">debug_assert_eq!</span>(input3.len(), <span class="number">16</span>);

    <span class="comment">// Bitslicing is a bit index manipulation. 512 bits of data means each bit is positioned at a
    // 9-bit index. AES data is 4 blocks, each one a 4x4 column-major matrix of bytes, so the
    // index is initially ([b]lock, [c]olumn, [r]ow, [p]osition):
    //     b1 b0 c1 c0 r1 r0 p2 p1 p0
    //
    // The desired bitsliced data groups first by bit position, then row, column, block:
    //     p2 p1 p0 r1 r0 c1 c0 b1 b0

    </span><span class="attr">#[rustfmt::skip]
    </span><span class="kw">fn </span>read_reordered(input: <span class="kw-2">&amp;</span>[u8]) -&gt; u64 {
        (u64::from(input[<span class="number">0x0</span>])        ) |
        (u64::from(input[<span class="number">0x1</span>]) &lt;&lt; <span class="number">0x10</span>) |
        (u64::from(input[<span class="number">0x2</span>]) &lt;&lt; <span class="number">0x20</span>) |
        (u64::from(input[<span class="number">0x3</span>]) &lt;&lt; <span class="number">0x30</span>) |
        (u64::from(input[<span class="number">0x8</span>]) &lt;&lt; <span class="number">0x08</span>) |
        (u64::from(input[<span class="number">0x9</span>]) &lt;&lt; <span class="number">0x18</span>) |
        (u64::from(input[<span class="number">0xa</span>]) &lt;&lt; <span class="number">0x28</span>) |
        (u64::from(input[<span class="number">0xb</span>]) &lt;&lt; <span class="number">0x38</span>)
    }

    <span class="comment">// Reorder each block's bytes on input
    //     __ __ c1 c0 r1 r0 __ __ __ =&gt; __ __ c0 r1 r0 c1 __ __ __
    // Reorder by relabeling (note the order of input)
    //     b1 b0 c0 __ __ __ __ __ __ =&gt; c0 b1 b0 __ __ __ __ __ __
    </span><span class="kw">let </span><span class="kw-2">mut </span>t0 = read_reordered(<span class="kw-2">&amp;</span>input0[<span class="number">0x00</span>..<span class="number">0x0c</span>]);
    <span class="kw">let </span><span class="kw-2">mut </span>t4 = read_reordered(<span class="kw-2">&amp;</span>input0[<span class="number">0x04</span>..<span class="number">0x10</span>]);
    <span class="kw">let </span><span class="kw-2">mut </span>t1 = read_reordered(<span class="kw-2">&amp;</span>input1[<span class="number">0x00</span>..<span class="number">0x0c</span>]);
    <span class="kw">let </span><span class="kw-2">mut </span>t5 = read_reordered(<span class="kw-2">&amp;</span>input1[<span class="number">0x04</span>..<span class="number">0x10</span>]);
    <span class="kw">let </span><span class="kw-2">mut </span>t2 = read_reordered(<span class="kw-2">&amp;</span>input2[<span class="number">0x00</span>..<span class="number">0x0c</span>]);
    <span class="kw">let </span><span class="kw-2">mut </span>t6 = read_reordered(<span class="kw-2">&amp;</span>input2[<span class="number">0x04</span>..<span class="number">0x10</span>]);
    <span class="kw">let </span><span class="kw-2">mut </span>t3 = read_reordered(<span class="kw-2">&amp;</span>input3[<span class="number">0x00</span>..<span class="number">0x0c</span>]);
    <span class="kw">let </span><span class="kw-2">mut </span>t7 = read_reordered(<span class="kw-2">&amp;</span>input3[<span class="number">0x04</span>..<span class="number">0x10</span>]);

    <span class="comment">// Bit Index Swap 6 &lt;-&gt; 0:
    //     __ __ b0 __ __ __ __ __ p0 =&gt; __ __ p0 __ __ __ __ __ b0
    </span><span class="kw">let </span>m0 = <span class="number">0x5555555555555555</span>;
    delta_swap_2(<span class="kw-2">&amp;mut </span>t1, <span class="kw-2">&amp;mut </span>t0, <span class="number">1</span>, m0);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t3, <span class="kw-2">&amp;mut </span>t2, <span class="number">1</span>, m0);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t5, <span class="kw-2">&amp;mut </span>t4, <span class="number">1</span>, m0);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t7, <span class="kw-2">&amp;mut </span>t6, <span class="number">1</span>, m0);

    <span class="comment">// Bit Index Swap 7 &lt;-&gt; 1:
    //     __ b1 __ __ __ __ __ p1 __ =&gt; __ p1 __ __ __ __ __ b1 __
    </span><span class="kw">let </span>m1 = <span class="number">0x3333333333333333</span>;
    delta_swap_2(<span class="kw-2">&amp;mut </span>t2, <span class="kw-2">&amp;mut </span>t0, <span class="number">2</span>, m1);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t3, <span class="kw-2">&amp;mut </span>t1, <span class="number">2</span>, m1);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t6, <span class="kw-2">&amp;mut </span>t4, <span class="number">2</span>, m1);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t7, <span class="kw-2">&amp;mut </span>t5, <span class="number">2</span>, m1);

    <span class="comment">// Bit Index Swap 8 &lt;-&gt; 2:
    //     c0 __ __ __ __ __ p2 __ __ =&gt; p2 __ __ __ __ __ c0 __ __
    </span><span class="kw">let </span>m2 = <span class="number">0x0f0f0f0f0f0f0f0f</span>;
    delta_swap_2(<span class="kw-2">&amp;mut </span>t4, <span class="kw-2">&amp;mut </span>t0, <span class="number">4</span>, m2);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t5, <span class="kw-2">&amp;mut </span>t1, <span class="number">4</span>, m2);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t6, <span class="kw-2">&amp;mut </span>t2, <span class="number">4</span>, m2);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t7, <span class="kw-2">&amp;mut </span>t3, <span class="number">4</span>, m2);

    <span class="comment">// Final bitsliced bit index, as desired:
    //     p2 p1 p0 r1 r0 c1 c0 b1 b0
    </span>output[<span class="number">0</span>] = t0;
    output[<span class="number">1</span>] = t1;
    output[<span class="number">2</span>] = t2;
    output[<span class="number">3</span>] = t3;
    output[<span class="number">4</span>] = t4;
    output[<span class="number">5</span>] = t5;
    output[<span class="number">6</span>] = t6;
    output[<span class="number">7</span>] = t7;
}

<span class="doccomment">/// Un-bitslice a 512-bit internal state into four 128-bit blocks of output.
</span><span class="kw">fn </span>inv_bitslice(input: <span class="kw-2">&amp;</span>[u64], output: <span class="kw-2">&amp;mut </span>[Block]) {
    <span class="macro">debug_assert_eq!</span>(input.len(), <span class="number">8</span>);
    <span class="macro">debug_assert_eq!</span>(output.len(), <span class="number">4</span>);

    <span class="comment">// Unbitslicing is a bit index manipulation. 512 bits of data means each bit is positioned at
    // a 9-bit index. AES data is 4 blocks, each one a 4x4 column-major matrix of bytes, so the
    // desired index for the output is ([b]lock, [c]olumn, [r]ow, [p]osition):
    //     b1 b0 c1 c0 r1 r0 p2 p1 p0
    //
    // The initially bitsliced data groups first by bit position, then row, column, block:
    //     p2 p1 p0 r1 r0 c1 c0 b1 b0

    </span><span class="kw">let </span><span class="kw-2">mut </span>t0 = input[<span class="number">0</span>];
    <span class="kw">let </span><span class="kw-2">mut </span>t1 = input[<span class="number">1</span>];
    <span class="kw">let </span><span class="kw-2">mut </span>t2 = input[<span class="number">2</span>];
    <span class="kw">let </span><span class="kw-2">mut </span>t3 = input[<span class="number">3</span>];
    <span class="kw">let </span><span class="kw-2">mut </span>t4 = input[<span class="number">4</span>];
    <span class="kw">let </span><span class="kw-2">mut </span>t5 = input[<span class="number">5</span>];
    <span class="kw">let </span><span class="kw-2">mut </span>t6 = input[<span class="number">6</span>];
    <span class="kw">let </span><span class="kw-2">mut </span>t7 = input[<span class="number">7</span>];

    <span class="comment">// TODO: these bit index swaps are identical to those in 'packing'

    // Bit Index Swap 6 &lt;-&gt; 0:
    //     __ __ p0 __ __ __ __ __ b0 =&gt; __ __ b0 __ __ __ __ __ p0
    </span><span class="kw">let </span>m0 = <span class="number">0x5555555555555555</span>;
    delta_swap_2(<span class="kw-2">&amp;mut </span>t1, <span class="kw-2">&amp;mut </span>t0, <span class="number">1</span>, m0);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t3, <span class="kw-2">&amp;mut </span>t2, <span class="number">1</span>, m0);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t5, <span class="kw-2">&amp;mut </span>t4, <span class="number">1</span>, m0);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t7, <span class="kw-2">&amp;mut </span>t6, <span class="number">1</span>, m0);

    <span class="comment">// Bit Index Swap 7 &lt;-&gt; 1:
    //     __ p1 __ __ __ __ __ b1 __ =&gt; __ b1 __ __ __ __ __ p1 __
    </span><span class="kw">let </span>m1 = <span class="number">0x3333333333333333</span>;
    delta_swap_2(<span class="kw-2">&amp;mut </span>t2, <span class="kw-2">&amp;mut </span>t0, <span class="number">2</span>, m1);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t3, <span class="kw-2">&amp;mut </span>t1, <span class="number">2</span>, m1);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t6, <span class="kw-2">&amp;mut </span>t4, <span class="number">2</span>, m1);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t7, <span class="kw-2">&amp;mut </span>t5, <span class="number">2</span>, m1);

    <span class="comment">// Bit Index Swap 8 &lt;-&gt; 2:
    //     p2 __ __ __ __ __ c0 __ __ =&gt; c0 __ __ __ __ __ p2 __ __
    </span><span class="kw">let </span>m2 = <span class="number">0x0f0f0f0f0f0f0f0f</span>;
    delta_swap_2(<span class="kw-2">&amp;mut </span>t4, <span class="kw-2">&amp;mut </span>t0, <span class="number">4</span>, m2);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t5, <span class="kw-2">&amp;mut </span>t1, <span class="number">4</span>, m2);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t6, <span class="kw-2">&amp;mut </span>t2, <span class="number">4</span>, m2);
    delta_swap_2(<span class="kw-2">&amp;mut </span>t7, <span class="kw-2">&amp;mut </span>t3, <span class="number">4</span>, m2);

    <span class="attr">#[rustfmt::skip]
    </span><span class="kw">fn </span>write_reordered(columns: u64, output: <span class="kw-2">&amp;mut </span>[u8]) {
        output[<span class="number">0x0</span>] = (columns        ) <span class="kw">as </span>u8;
        output[<span class="number">0x1</span>] = (columns &gt;&gt; <span class="number">0x10</span>) <span class="kw">as </span>u8;
        output[<span class="number">0x2</span>] = (columns &gt;&gt; <span class="number">0x20</span>) <span class="kw">as </span>u8;
        output[<span class="number">0x3</span>] = (columns &gt;&gt; <span class="number">0x30</span>) <span class="kw">as </span>u8;
        output[<span class="number">0x8</span>] = (columns &gt;&gt; <span class="number">0x08</span>) <span class="kw">as </span>u8;
        output[<span class="number">0x9</span>] = (columns &gt;&gt; <span class="number">0x18</span>) <span class="kw">as </span>u8;
        output[<span class="number">0xa</span>] = (columns &gt;&gt; <span class="number">0x28</span>) <span class="kw">as </span>u8;
        output[<span class="number">0xb</span>] = (columns &gt;&gt; <span class="number">0x38</span>) <span class="kw">as </span>u8;
    }

    <span class="comment">// Reorder by relabeling (note the order of output)
    //     c0 b1 b0 __ __ __ __ __ __ =&gt; b1 b0 c0 __ __ __ __ __ __
    // Reorder each block's bytes on output
    //     __ __ c0 r1 r0 c1 __ __ __ =&gt; __ __ c1 c0 r1 r0 __ __ __
    </span>write_reordered(t0, <span class="kw-2">&amp;mut </span>output[<span class="number">0</span>][<span class="number">0x00</span>..<span class="number">0x0c</span>]);
    write_reordered(t4, <span class="kw-2">&amp;mut </span>output[<span class="number">0</span>][<span class="number">0x04</span>..<span class="number">0x10</span>]);
    write_reordered(t1, <span class="kw-2">&amp;mut </span>output[<span class="number">1</span>][<span class="number">0x00</span>..<span class="number">0x0c</span>]);
    write_reordered(t5, <span class="kw-2">&amp;mut </span>output[<span class="number">1</span>][<span class="number">0x04</span>..<span class="number">0x10</span>]);
    write_reordered(t2, <span class="kw-2">&amp;mut </span>output[<span class="number">2</span>][<span class="number">0x00</span>..<span class="number">0x0c</span>]);
    write_reordered(t6, <span class="kw-2">&amp;mut </span>output[<span class="number">2</span>][<span class="number">0x04</span>..<span class="number">0x10</span>]);
    write_reordered(t3, <span class="kw-2">&amp;mut </span>output[<span class="number">3</span>][<span class="number">0x00</span>..<span class="number">0x0c</span>]);
    write_reordered(t7, <span class="kw-2">&amp;mut </span>output[<span class="number">3</span>][<span class="number">0x04</span>..<span class="number">0x10</span>]);

    <span class="comment">// Final AES bit index, as desired:
    //     b1 b0 c1 c0 r1 r0 p2 p1 p0
</span>}

<span class="doccomment">/// Copy 32-bytes within the provided slice to an 8-byte offset
</span><span class="kw">fn </span>memshift32(buffer: <span class="kw-2">&amp;mut </span>[u64], src_offset: usize) {
    <span class="macro">debug_assert_eq!</span>(src_offset % <span class="number">8</span>, <span class="number">0</span>);

    <span class="kw">let </span>dst_offset = src_offset + <span class="number">8</span>;
    <span class="macro">debug_assert!</span>(dst_offset + <span class="number">8 </span>&lt;= buffer.len());

    <span class="kw">for </span>i <span class="kw">in </span>(<span class="number">0</span>..<span class="number">8</span>).rev() {
        buffer[dst_offset + i] = buffer[src_offset + i];
    }
}

<span class="doccomment">/// XOR the round key to the internal state. The round keys are expected to be
/// pre-computed and to be packed in the fixsliced representation.
</span><span class="attr">#[inline]
</span><span class="kw">fn </span>add_round_key(state: <span class="kw-2">&amp;mut </span>State, rkey: <span class="kw-2">&amp;</span>[u64]) {
    <span class="macro">debug_assert_eq!</span>(rkey.len(), <span class="number">8</span>);
    <span class="kw">for </span>(a, b) <span class="kw">in </span>state.iter_mut().zip(rkey) {
        <span class="kw-2">*</span>a ^= b;
    }
}

<span class="attr">#[inline(always)]
</span><span class="kw">fn </span>add_round_constant_bit(state: <span class="kw-2">&amp;mut </span>[u64], bit: usize) {
    state[bit] ^= <span class="number">0x00000000f0000000</span>;
}

<span class="attr">#[inline(always)]
</span><span class="kw">fn </span>ror(x: u64, y: u32) -&gt; u64 {
    x.rotate_right(y)
}

<span class="attr">#[inline(always)]
</span><span class="kw">fn </span>ror_distance(rows: u32, cols: u32) -&gt; u32 {
    (rows &lt;&lt; <span class="number">4</span>) + (cols &lt;&lt; <span class="number">2</span>)
}

<span class="attr">#[inline(always)]
</span><span class="kw">fn </span>rotate_rows_1(x: u64) -&gt; u64 {
    ror(x, ror_distance(<span class="number">1</span>, <span class="number">0</span>))
}

<span class="attr">#[inline(always)]
</span><span class="kw">fn </span>rotate_rows_2(x: u64) -&gt; u64 {
    ror(x, ror_distance(<span class="number">2</span>, <span class="number">0</span>))
}

<span class="attr">#[inline(always)]
#[rustfmt::skip]
</span><span class="kw">fn </span>rotate_rows_and_columns_1_1(x: u64) -&gt; u64 {
    (ror(x, ror_distance(<span class="number">1</span>, <span class="number">1</span>)) &amp; <span class="number">0x0fff0fff0fff0fff</span>) |
    (ror(x, ror_distance(<span class="number">0</span>, <span class="number">1</span>)) &amp; <span class="number">0xf000f000f000f000</span>)
}

<span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
#[inline(always)]
#[rustfmt::skip]
</span><span class="kw">fn </span>rotate_rows_and_columns_1_2(x: u64) -&gt; u64 {
    (ror(x, ror_distance(<span class="number">1</span>, <span class="number">2</span>)) &amp; <span class="number">0x00ff00ff00ff00ff</span>) |
    (ror(x, ror_distance(<span class="number">0</span>, <span class="number">2</span>)) &amp; <span class="number">0xff00ff00ff00ff00</span>)
}

<span class="attr">#[cfg(not(feature = <span class="string">"compact"</span>))]
#[inline(always)]
#[rustfmt::skip]
</span><span class="kw">fn </span>rotate_rows_and_columns_1_3(x: u64) -&gt; u64 {
    (ror(x, ror_distance(<span class="number">1</span>, <span class="number">3</span>)) &amp; <span class="number">0x000f000f000f000f</span>) |
    (ror(x, ror_distance(<span class="number">0</span>, <span class="number">3</span>)) &amp; <span class="number">0xfff0fff0fff0fff0</span>)
}

<span class="attr">#[inline(always)]
#[rustfmt::skip]
</span><span class="kw">fn </span>rotate_rows_and_columns_2_2(x: u64) -&gt; u64 {
    (ror(x, ror_distance(<span class="number">2</span>, <span class="number">2</span>)) &amp; <span class="number">0x00ff00ff00ff00ff</span>) |
    (ror(x, ror_distance(<span class="number">1</span>, <span class="number">2</span>)) &amp; <span class="number">0xff00ff00ff00ff00</span>)
}

<span class="doccomment">/// Low-level "hazmat" AES functions.
///
/// Note: this isn't actually used in the `Aes128`/`Aes192`/`Aes256`
/// implementations in this crate, but instead provides raw access to
/// the AES round function gated under the `hazmat` crate feature.
</span><span class="attr">#[cfg(feature = <span class="string">"hazmat"</span>)]
</span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">mod </span>hazmat {
    <span class="kw">use super</span>::{
        bitslice, inv_bitslice, inv_mix_columns_0, inv_shift_rows_1, inv_sub_bytes, mix_columns_0,
        shift_rows_1, sub_bytes, sub_bytes_nots, State,
    };
    <span class="kw">use crate</span>::{Block, ParBlocks};

    <span class="doccomment">/// XOR the `src` block into the `dst` block in-place.
    </span><span class="kw">fn </span>xor_in_place(dst: <span class="kw-2">&amp;mut </span>Block, src: <span class="kw-2">&amp;</span>Block) {
        <span class="kw">for </span>(a, b) <span class="kw">in </span>dst.iter_mut().zip(src.as_slice()) {
            <span class="kw-2">*</span>a ^= <span class="kw-2">*</span>b;
        }
    }

    <span class="doccomment">/// Perform a bitslice operation, loading a single block.
    </span><span class="kw">fn </span>bitslice_block(block: <span class="kw-2">&amp;</span>Block) -&gt; State {
        <span class="kw">let </span><span class="kw-2">mut </span>state = State::default();
        bitslice(<span class="kw-2">&amp;mut </span>state, block, block, block, block);
        state
    }

    <span class="doccomment">/// Perform an inverse bitslice operation, extracting a single block.
    </span><span class="kw">fn </span>inv_bitslice_block(block: <span class="kw-2">&amp;mut </span>Block, state: <span class="kw-2">&amp;</span>State) {
        <span class="kw">let </span><span class="kw-2">mut </span>out = [Block::default(); <span class="number">4</span>];
        inv_bitslice(state, <span class="kw-2">&amp;mut </span>out);
        block.copy_from_slice(<span class="kw-2">&amp;</span>out[<span class="number">0</span>]);
    }

    <span class="doccomment">/// AES cipher (encrypt) round function.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>cipher_round(block: <span class="kw-2">&amp;mut </span>Block, round_key: <span class="kw-2">&amp;</span>Block) {
        <span class="kw">let </span><span class="kw-2">mut </span>state = bitslice_block(block);
        sub_bytes(<span class="kw-2">&amp;mut </span>state);
        sub_bytes_nots(<span class="kw-2">&amp;mut </span>state);
        shift_rows_1(<span class="kw-2">&amp;mut </span>state);
        mix_columns_0(<span class="kw-2">&amp;mut </span>state);
        inv_bitslice_block(block, <span class="kw-2">&amp;</span>state);
        xor_in_place(block, round_key);
    }

    <span class="doccomment">/// AES cipher (encrypt) round function: parallel version.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>cipher_round_par(blocks: <span class="kw-2">&amp;mut </span>ParBlocks, round_keys: <span class="kw-2">&amp;</span>ParBlocks) {
        <span class="kw">for </span>(chunk, keys) <span class="kw">in </span>blocks.chunks_exact_mut(<span class="number">4</span>).zip(round_keys.chunks_exact(<span class="number">4</span>)) {
            <span class="kw">let </span><span class="kw-2">mut </span>state = State::default();
            bitslice(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>chunk[<span class="number">0</span>], <span class="kw-2">&amp;</span>chunk[<span class="number">1</span>], <span class="kw-2">&amp;</span>chunk[<span class="number">2</span>], <span class="kw-2">&amp;</span>chunk[<span class="number">3</span>]);
            sub_bytes(<span class="kw-2">&amp;mut </span>state);
            sub_bytes_nots(<span class="kw-2">&amp;mut </span>state);
            shift_rows_1(<span class="kw-2">&amp;mut </span>state);
            mix_columns_0(<span class="kw-2">&amp;mut </span>state);
            inv_bitslice(<span class="kw-2">&amp;</span>state, chunk);

            <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">4 </span>{
                xor_in_place(<span class="kw-2">&amp;mut </span>chunk[i], <span class="kw-2">&amp;</span>keys[i]);
            }
        }
    }

    <span class="doccomment">/// AES cipher (encrypt) round function.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>equiv_inv_cipher_round(block: <span class="kw-2">&amp;mut </span>Block, round_key: <span class="kw-2">&amp;</span>Block) {
        <span class="kw">let </span><span class="kw-2">mut </span>state = State::default();
        bitslice(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>block, <span class="kw-2">&amp;</span>block, <span class="kw-2">&amp;</span>block, <span class="kw-2">&amp;</span>block);
        sub_bytes_nots(<span class="kw-2">&amp;mut </span>state);
        inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
        inv_shift_rows_1(<span class="kw-2">&amp;mut </span>state);
        inv_mix_columns_0(<span class="kw-2">&amp;mut </span>state);
        inv_bitslice_block(block, <span class="kw-2">&amp;</span>state);
        xor_in_place(block, round_key);
    }

    <span class="doccomment">/// AES cipher (encrypt) round function: parallel version.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>equiv_inv_cipher_round_par(blocks: <span class="kw-2">&amp;mut </span>ParBlocks, round_keys: <span class="kw-2">&amp;</span>ParBlocks) {
        <span class="kw">for </span>(chunk, keys) <span class="kw">in </span>blocks.chunks_exact_mut(<span class="number">4</span>).zip(round_keys.chunks_exact(<span class="number">4</span>)) {
            <span class="kw">let </span><span class="kw-2">mut </span>state = State::default();
            bitslice(<span class="kw-2">&amp;mut </span>state, <span class="kw-2">&amp;</span>chunk[<span class="number">0</span>], <span class="kw-2">&amp;</span>chunk[<span class="number">1</span>], <span class="kw-2">&amp;</span>chunk[<span class="number">2</span>], <span class="kw-2">&amp;</span>chunk[<span class="number">3</span>]);
            sub_bytes_nots(<span class="kw-2">&amp;mut </span>state);
            inv_sub_bytes(<span class="kw-2">&amp;mut </span>state);
            inv_shift_rows_1(<span class="kw-2">&amp;mut </span>state);
            inv_mix_columns_0(<span class="kw-2">&amp;mut </span>state);
            inv_bitslice(<span class="kw-2">&amp;</span>state, chunk);

            <span class="kw">for </span>i <span class="kw">in </span><span class="number">0</span>..<span class="number">4 </span>{
                xor_in_place(<span class="kw-2">&amp;mut </span>chunk[i], <span class="kw-2">&amp;</span>keys[i]);
            }
        }
    }

    <span class="doccomment">/// AES mix columns function.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>mix_columns(block: <span class="kw-2">&amp;mut </span>Block) {
        <span class="kw">let </span><span class="kw-2">mut </span>state = bitslice_block(block);
        mix_columns_0(<span class="kw-2">&amp;mut </span>state);
        inv_bitslice_block(block, <span class="kw-2">&amp;</span>state);
    }

    <span class="doccomment">/// AES inverse mix columns function.
    </span><span class="attr">#[inline]
    </span><span class="kw">pub</span>(<span class="kw">crate</span>) <span class="kw">fn </span>inv_mix_columns(block: <span class="kw-2">&amp;mut </span>Block) {
        <span class="kw">let </span><span class="kw-2">mut </span>state = bitslice_block(block);
        inv_mix_columns_0(<span class="kw-2">&amp;mut </span>state);
        inv_bitslice_block(block, <span class="kw-2">&amp;</span>state);
    }
}
</code></pre></div></section></main></body></html>